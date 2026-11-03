//! Co-op in the viewer: a thin Bevy shell around `coop_session::Session`.
//!
//! `connect` (before the window opens) starts the session; a guest waits
//! there for the host's place and loads it. `CoopPlugin` then runs the
//! session once per frame: network receive and (guest) apply at
//! `ViewerSet::Input`, collect, send, messages, status line, state dump and
//! quit checks at `ViewerSet::AfterFrame`. See docs/COOP.md.
//!
//! Scripted lines (`--stage`, `--run`, `--run-at`) are held until the
//! session is ready (the test harness's clock starts there). The host runs
//! them; a guest sends them to the host as commands. Two prefixes change
//! that: `local LINE` runs on the machine that has it (a guest moving its own
//! player), and `coop VERB ...` is the live-layer test bot
//! (`coop_live::bot`). The live layer (avatars, shared actors, loot,
//! dialogue, rules) is `coop_live`, added by [`CoopPlugin`].

use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use bevy::prelude::*;
use coop_session::json::{self, V};
use coop_session::{
    ChaosProfile, Command, Event as CoopEvent, Opening, OwnCharacter, Role, Session, SessionConfig,
    StartConfig,
};
use world::coop::apply::{Ctx, Opts};

use crate::args::{Args, Stance};
use crate::coop_live::{self, LiveNow};
use crate::dialogue::DialogueState;
use crate::frame_order::ViewerSet;
use crate::hud::{HudMessage, HudMessages};
use crate::scripts::{LaterCommands, Notices, Scripts, StartCommands, StartStage};
use crate::GameFiles;

/// How long a guest waits for the host before giving up.
const JOIN_WAIT: Duration = Duration::from_secs(90);
/// Seconds between state dumps.
const DUMP_EVERY: f32 = 1.0;

/// The session, a non-send resource (its transport is not `Send`).
pub struct CoopLink(pub Session);

/// Scripted lines waiting for the session to be ready.
#[derive(Resource, Default)]
pub struct Held {
    stage: Option<(String, u16)>,
    run: Vec<String>,
    run_at: Vec<(f32, String)>,
    released: Option<f32>,
    /// `local` lines (prefix removed), run on this machine.
    local: Vec<String>,
    local_at: Vec<(f32, String)>,
}

/// Takes the bot lines and the `local` lines out of the held ones.
fn split_special(held: &mut Held, bot: &mut coop_live::Bot, now: f32) {
    for l in std::mem::take(&mut held.run) {
        if coop_live::is_bot_line(&l) {
            bot.push_at(now, l);
        } else if let Some(rest) = l.strip_prefix("local ") {
            held.local.push(rest.to_string());
        } else {
            held.run.push(l);
        }
    }
    for (at, l) in std::mem::take(&mut held.run_at) {
        if coop_live::is_bot_line(&l) {
            bot.push_at(now + at, l);
        } else if let Some(rest) = l.strip_prefix("local ") {
            held.local_at.push((at, rest.to_string()));
        } else {
            held.run_at.push((at, l));
        }
    }
}

/// Run bookkeeping and the harness options.
#[derive(Resource)]
pub struct Run {
    role: Role,
    cell: String,
    state_dump: Option<PathBuf>,
    quit_file: Option<PathBuf>,
    /// `--coop-go-file`: the script clock waits for this file.
    go_file: Option<PathBuf>,
    /// The session logged Ready (the script clock may still wait for `go_file`).
    ready_seen: bool,
    quit_after: Option<f32>,
    show_text: bool,
    seq: u64,
    last_dump: f32,
    ready_at: Option<f32>,
    loaded_logged: bool,
    done: bool,
}

/// What `connect` hands to the plugin.
pub struct Started {
    control: Option<u16>,
    link: CoopLink,
    held: Held,
    run: Run,
}

thread_local! {
    static STASH: RefCell<Option<Started>> = const { RefCell::new(None) };
}

/// Keeps the started session for `CoopPlugin::build` (the plugin must be
/// `Send`, the session is not).
pub fn stash(started: Option<Started>) {
    STASH.with(|s| *s.borrow_mut() = started);
}

fn fnv(bytes: impl IntoIterator<Item = u8>) -> u64 {
    bytes.into_iter().fold(0xcbf2_9ce4_8422_2325, |h, b| {
        (h ^ b as u64).wrapping_mul(0x0000_0100_0000_01b3)
    })
}

/// A line in the log for a session that never started, so a harness sees
/// why (`start`, `error`, `shutdown`).
fn failure_log(path: Option<&Path>, role: &str, msg: &str) {
    let Some(path) = path else { return };
    use std::io::Write;
    if let Some(dir) = path.parent().filter(|d| !d.as_os_str().is_empty()) {
        let _ = std::fs::create_dir_all(dir);
    }
    let Ok(mut f) = std::fs::File::create(path) else {
        return;
    };
    let pid = std::process::id() as u64;
    let _ = writeln!(
        f,
        "{}",
        json::object(&[
            ("t", V::F(0.0)),
            ("ev", V::S("start")),
            ("role", V::S(role)),
            ("pid", V::U(pid))
        ])
    );
    let _ = writeln!(
        f,
        "{}",
        json::object(&[("t", V::F(0.0)), ("ev", V::S("error")), ("msg", V::S(msg))])
    );
    let _ = writeln!(
        f,
        "{}",
        json::object(&[
            ("t", V::F(0.0)),
            ("ev", V::S("shutdown")),
            ("reason", V::S("error")),
            ("clean", V::B(false)),
            ("code", V::I(1))
        ])
    );
}

/// Starts the session asked for by `--coop-host` / `--coop-join`. A guest
/// waits here for the host's place and sets `args.cell` / `args.at` to it,
/// and moves its `--stage` / `--run` / `--run-at` lines out of `args`
/// (they are sent to the host once ready). `None` without co-op options.
pub fn connect(args: &mut Args, order: &esm::LoadOrder) -> Result<Option<Started>, String> {
    let c = args.coop.clone();
    if !c.active() {
        return Ok(None);
    }
    crate::coop_live::configure(&c);
    let role = if c.host.is_some() {
        Role::Host
    } else {
        Role::Guest
    };
    let role_name = if role == Role::Host { "host" } else { "guest" };
    let pid = std::process::id();
    let mut cfg = SessionConfig {
        name: c.name.clone().unwrap_or_else(|| {
            if role == Role::Host {
                "Host".into()
            } else {
                format!("Guest {}", pid % 1000)
            }
        }),
        addr: match (&c.host, &c.join) {
            (Some(port), _) => port.to_string(),
            (_, Some(a)) => a.clone(),
            _ => String::new(),
        },
        live: true,
        native_dialogue: true,
        build_hash: fnv(env!("CARGO_PKG_VERSION").bytes()),
        mods_hash: fnv(order.plugins().iter().flat_map(|p| {
            p.name
                .to_ascii_lowercase()
                .into_bytes()
                .into_iter()
                .chain([0])
        })),
        chaos: c
            .net
            .as_deref()
            .and_then(ChaosProfile::by_name)
            .map(|p| (p, 0x5EED ^ pid as u64)),
        compat: coop_session::CompatConfig {
            exe: crate::coop_live::EXE_VERSION.into(),
            volatile_hash: coop_session::compat::volatile_hash(coop_session::compat::VOLATILE_KEYS),
            plugins: order.plugins().iter().map(|p| p.name.to_string()).collect(),
            allow_mismatch: c.allow_mismatch,
        },
        log: c.log.clone(),
        rules: c.rules,
        ..SessionConfig::default()
    };
    // Transport: UDP or Steam, random per-session token (see coop_link, coop_session::link).
    let notices = crate::coop_link::open(&c, role, &mut cfg).map_err(|e| {
        failure_log(c.log.as_deref(), role_name, &e);
        e
    })?;
    let started = match role {
        Role::Host => Session::host(cfg),
        Role::Guest => Session::join(cfg),
    };
    let mut session = started.map_err(|e| {
        failure_log(c.log.as_deref(), role_name, &e.to_string());
        e.to_string()
    })?;
    crate::coop_link::log_notices(&mut session, &notices);
    // How this process starts (docs/COOP.md, "The start of a game"): a host's --new-game runs the
    // opening for the host player; a guest makes its own character unless it brings one.
    let own = if c.no_chargen {
        OwnCharacter::Skip
    } else if c.own_load.is_some() {
        OwnCharacter::Provided
    } else {
        OwnCharacter::Create
    };
    session.set_start_config(StartConfig {
        new_game: c.new_game,
        own,
    });
    if let Some(path) = c.own_load.as_deref().filter(|_| role == Role::Guest) {
        match own_sheet(order, path) {
            Ok(sheet) => session.set_own_sheet(sheet),
            Err(e) => {
                failure_log(c.log.as_deref(), role_name, &e);
                return Err(e);
            }
        }
    }
    let who = c.name.clone().unwrap_or_else(|| {
        if role == Role::Host {
            "Host".into()
        } else {
            "Guest".into()
        }
    });
    if let Err(e) = crate::coop_party::setup(&mut session, role, &c, &who) {
        failure_log(c.log.as_deref(), role_name, &e);
        return Err(e);
    }
    if role == Role::Guest {
        let begun = Instant::now();
        let place = loop {
            session.poll(0.016);
            // the host's place and how far its own opening is both arrive right after the join
            if let (Some(p), Some(_)) = (session.host_place(), session.host_opening()) {
                break p;
            }
            if let Some(why) = session.last_error().map(str::to_string) {
                session.log_event(
                    "shutdown",
                    &[
                        ("reason", V::S("error")),
                        ("clean", V::B(false)),
                        ("code", V::I(1)),
                    ],
                );
                return Err(why);
            }
            if c.quit_file.as_deref().is_some_and(Path::exists) {
                session.log_event(
                    "shutdown",
                    &[
                        ("reason", V::S("quit-file")),
                        ("clean", V::B(true)),
                        ("code", V::I(0)),
                    ],
                );
                std::process::exit(0);
            }
            if begun.elapsed() > JOIN_WAIT {
                session.log_event("error", &[("msg", V::S("no answer from the host"))]);
                session.log_event(
                    "shutdown",
                    &[
                        ("reason", V::S("error")),
                        ("clean", V::B(false)),
                        ("code", V::I(1)),
                    ],
                );
                return Err("the host did not answer".into());
            }
            std::thread::sleep(Duration::from_millis(16));
        };
        // The host's opening is over and this guest makes its character: it does so in a local
        // room and is let go to the host's place afterwards (`start_follow_ok`).
        let local_room =
            own == OwnCharacter::Create && session.host_opening() == Some(Opening::Over);
        if args.cell == crate::args::COOP_PLACE {
            args.cell = if local_room {
                coop_session::start::LOCAL_ROOM.to_string()
            } else {
                place.query()
            };
        }
        if args.at.is_none() && !local_room {
            args.at = place.pos.filter(|_| place.outdoors()).map(|p| Stance {
                feet: p,
                heading: 0.0,
                pitch: 0.0,
            });
        }
        if local_room {
            println!(
                "Co-op: joined; the host's opening is over, making your character in {} first.",
                args.cell
            );
        } else {
            println!("Co-op: joined; loading the host's place {}.", args.cell);
        }
    } else {
        println!("Co-op: hosting on UDP port {}.", c.host.unwrap_or_default());
    }
    let held = Held {
        // a new game's opening starts at once (not when the first guest arrives): its stage stays
        // in the arguments for the normal start
        stage: if c.new_game { None } else { args.stage.take() },
        run: std::mem::take(&mut args.run),
        run_at: std::mem::take(&mut args.run_at),
        released: None,
        local: Vec::new(),
        local_at: Vec::new(),
    };
    let run = Run {
        role,
        cell: args.cell.clone(),
        state_dump: c.state_dump.clone(),
        quit_file: c.quit_file.clone(),
        go_file: c.go_file.clone(),
        ready_seen: false,
        quit_after: c.quit_after,
        show_text: false,
        seq: 0,
        last_dump: -DUMP_EVERY,
        ready_at: None,
        loaded_logged: false,
        done: false,
    };
    Ok(Some(Started {
        control: c.control,
        link: CoopLink(session),
        held,
        run,
    }))
}

/// A guest's own character from a save (`--load-fos FILE` or `--coop-load FILE`): the name, SPECIAL,
/// tags and perks. Nothing else of the save is used.
fn own_sheet(order: &esm::LoadOrder, path: &Path) -> Result<coop_session::Sheet, String> {
    let state = if path
        .extension()
        .is_some_and(|e| e.eq_ignore_ascii_case("fos"))
    {
        let bytes =
            std::fs::read(path).map_err(|e| format!("can't read {}: {e}", path.display()))?;
        world::fos_import::import(order, &bytes)?.state
    } else {
        let text = std::fs::read_to_string(path)
            .map_err(|e| format!("can't read {}: {e}", path.display()))?;
        world::save::load(&text)?.0
    };
    Ok(coop_session::Sheet::read(&state))
}

/// A guest's own character making (`coop_session::start`): the session says which menu is next, the
/// game's own menu opens for this player only, and its closing is what moves the flow on.
fn start_menus(
    link: NonSendMut<CoopLink>,
    state: Res<DialogueState>,
    mut menus: ResMut<crate::menus::Menus>,
) {
    use coop_session::Step;
    use world::chargen::CharacterMenu as M;
    if link.0.role() != Role::Guest || state.0.player_cell.is_none() {
        return;
    }
    let busy = menus.is_open();
    // `start_poll` needs `&mut Session`; the link is the only holder
    let mut link = link;
    if let Some(step) = link.0.start_poll(&state.0, busy) {
        let m = match step {
            Step::Name => M::Name,
            Step::Special => M::Special {
                points: coop_session::start::SPECIAL_POINTS,
            },
            Step::TagSkills => M::TagSkills {
                count: coop_session::start::TAG_COUNT,
                preselect: true,
            },
            Step::Traits => M::Traits {
                max: coop_session::start::TRAIT_MAX,
            },
        };
        println!("Co-op: your character, {} menu.", step.name());
        menus.push(crate::menus::Menu::Character(m));
    }
}

/// Registers the co-op systems when `connect` started a session.
pub struct CoopPlugin;

impl Plugin for CoopPlugin {
    fn build(&self, app: &mut App) {
        let Some(started) = STASH.with(|s| s.borrow_mut().take()) else {
            return;
        };
        let role = started.run.role;
        app.insert_non_send_resource(started.link)
            .insert_resource(started.held)
            .insert_resource(started.run)
            .add_systems(Startup, setup_text)
            .add_systems(Update, receive.in_set(ViewerSet::Input))
            .add_systems(Update, send.in_set(ViewerSet::AfterFrame))
            .add_systems(Update, start_menus.in_set(ViewerSet::AfterFrame))
            .add_systems(Last, at_exit)
            .add_plugins(coop_live::LivePlugin)
            .add_plugins(crate::coop_party::PartyPlugin(role));
        if let Some(port) = started.control {
            app.add_plugins(crate::coop_control::ControlPlugin(port));
        }
    }
}

#[derive(Component)]
pub(crate) struct CoopText;

fn setup_text(mut commands: Commands) {
    commands.spawn((
        Text::new(String::new()),
        TextFont {
            font_size: 11.0,
            ..default()
        },
        TextColor(Color::srgba(0.7, 0.95, 0.7, 0.8)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(4.0),
            right: Val::Px(8.0),
            padding: UiRect::all(Val::Px(2.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.45)),
        Visibility::Hidden,
        CoopText,
    ));
}

/// Start of the frame: packets in; a guest applies the host's state.
fn receive(
    mut link: NonSendMut<CoopLink>,
    mut state: ResMut<DialogueState>,
    game: Res<GameFiles>,
    scripts: Res<Scripts>,
) {
    let ctx = Ctx {
        order: &game.0.order,
        scripts: &scripts.0,
    };
    link.0.pre_frame_with(
        &mut state.0,
        Opts {
            present: true,
            ctx: Some(ctx),
        },
    );
}

fn log_script(s: &mut Session, line: &str) {
    s.log_event("script", &[("line", V::S(line))]);
}

impl Run {
    /// This machine hosts.
    pub(crate) fn is_host(&self) -> bool {
        self.role == Role::Host
    }

    /// The session is ready (the host's clock started).
    pub(crate) fn is_ready(&self) -> bool {
        self.ready_at.is_some()
    }
}

/// The state dump text (the next sequence number), as `--coop-state-dump` writes it.
pub(crate) fn dump_text(
    s: &Session,
    run: &mut Run,
    state: &world::scripting::GameState,
    game: &GameFiles,
    now: f32,
    live: &LiveNow,
) -> String {
    run.seq += 1;
    let root = Session::state_root(state);
    let mut counters = live.counters.clone();
    counters.start = Some(s.start_info());
    counters.status = Some(s.status());
    counters.hash_points = Some(s.hash_points());
    coop_session::dump::state_json_full(
        state,
        &game.0.order,
        run.role,
        now as f64,
        run.seq,
        root,
        live.view.as_ref(),
        &counters,
    )
}

fn write_dump(
    s: &mut Session,
    run: &mut Run,
    state: &world::scripting::GameState,
    game: &GameFiles,
    now: f32,
    live: &LiveNow,
) {
    let Some(path) = run.state_dump.clone() else {
        return;
    };
    let text = dump_text(s, run, state, game, now, live);
    coop_session::out::write_atomic(&path, &text);
    s.log_event(
        "state_dump",
        &[
            ("path", V::S(&path.display().to_string())),
            ("seq", V::U(run.seq)),
        ],
    );
}

fn finish(
    s: &mut Session,
    run: &mut Run,
    state: &world::scripting::GameState,
    game: &GameFiles,
    now: f32,
    reason: &str,
    live: &LiveNow,
) {
    if run.done {
        return;
    }
    write_dump(s, run, state, game, now, live);
    s.log_event(
        "shutdown",
        &[
            ("reason", V::S(reason)),
            ("clean", V::B(true)),
            ("code", V::I(0)),
        ],
    );
    run.done = true;
}

/// End of the frame: collect and send, messages, scripts, dump, quit.
#[allow(clippy::too_many_arguments)]
pub(crate) fn send(
    time: Res<Time>,
    mut link: NonSendMut<CoopLink>,
    mut state: ResMut<DialogueState>,
    game: Res<GameFiles>,
    mut held: ResMut<Held>,
    mut run: ResMut<Run>,
    (mut start_stage, mut start_commands, mut later): (
        ResMut<StartStage>,
        ResMut<StartCommands>,
        ResMut<LaterCommands>,
    ),
    (mut messages, mut notices): (ResMut<HudMessages>, ResMut<Notices>),
    mut exit: EventWriter<AppExit>,
    mut texts: Query<(&mut Text, &mut Visibility), With<CoopText>>,
    (mut bot, live): (ResMut<coop_live::Bot>, Res<LiveNow>),
    scripts: Res<Scripts>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let now = time.elapsed_secs();
    let s = &mut link.0;
    // F6 shows or hides the small players-and-ping indicator (off by default)
    if keys.just_pressed(KeyCode::F6) {
        run.show_text = !run.show_text;
    }
    s.post_frame(&mut state.0, time.delta_secs());
    if !run.loaded_logged {
        run.loaded_logged = true;
        let cell = run.cell.clone();
        s.log_event("loaded", &[("cell", V::S(&cell))]);
    }
    for ev in s.drain_events() {
        if let Some(m) = ev.message() {
            if messages.on {
                messages.queue.push(HudMessage::from(m));
            } else {
                notices.push(m, now);
            }
        }
        if ev == CoopEvent::Ready {
            run.ready_seen = true;
        }
    }
    // The script clock starts at the session's Ready. With `--coop-go-file` it
    // waits for that file too: the harness writes it when every peer is ready,
    // so a host does not act while a guest still loads the host's place.
    let go = match run.go_file.as_deref() {
        Some(p) => p.exists(),
        None => true,
    };
    if run.ready_seen && run.ready_at.is_none() && go {
        run.ready_at = Some(now);
        held.released = Some(now);
        split_special(&mut held, &mut bot, now);
        start_commands.0.extend(std::mem::take(&mut held.local));
        later.lines.extend(std::mem::take(&mut held.local_at));
        later.ready_at = Some(now);
        if run.role == Role::Host {
            if let Some(stage) = held.stage.take() {
                log_script(s, &format!("SetStage {} {}", stage.0, stage.1));
                start_stage.0 = Some(stage);
            }
            for l in std::mem::take(&mut held.run) {
                log_script(s, &l);
                start_commands.0.push(l);
            }
            later.lines.extend(std::mem::take(&mut held.run_at));
            later.ready_at = Some(now);
        } else {
            if let Some((q, n)) = held.stage.take() {
                log_script(s, &format!("SetStage {q} {n}"));
                s.send_command(Command::Stage(q, n));
            }
            for l in std::mem::take(&mut held.run) {
                log_script(s, &l);
                s.send_command(Command::Line(l));
            }
        }
    }
    // A guest's timed lines, from when the session became ready.
    if let (Role::Guest, Some(t0)) = (run.role, held.released) {
        let (due, later_lines): (Vec<_>, Vec<_>) = std::mem::take(&mut held.run_at)
            .into_iter()
            .partition(|(at, _)| now - t0 >= *at);
        held.run_at = later_lines;
        for (_, l) in due {
            log_script(s, &l);
            s.send_command(Command::Line(l));
        }
    }
    // The host carries out what guests asked.
    for (from, cmd) in s.take_commands() {
        let line = cmd.as_line();
        s.log_event(
            "script",
            &[("line", V::S(&line)), ("from", V::U(from.0 as u64))],
        );
        if from.is_host() || !state.0.coop.bank.enabled() {
            start_commands.0.push(line);
        } else {
            // a guest's line works on that guest's own rows (inventory,
            // skills), swapped in for the run
            let order = &game.0.order;
            state.0.with_player(from.0, |g| {
                world::scripting::Runner::new(order, &scripts.0, g).run_source(&line, None, None);
            });
        }
    }
    if now - run.last_dump >= DUMP_EVERY {
        run.last_dump = now;
        write_dump(s, &mut run, &state.0, &game, now, &live);
    }
    let quit = if run.quit_file.as_deref().is_some_and(Path::exists) {
        Some("quit-file")
    } else {
        match (run.quit_after, run.ready_at) {
            (Some(after), Some(t)) if now - t >= after => Some("quit-after"),
            _ => None,
        }
    };
    if let Some(reason) = quit {
        finish(s, &mut run, &state.0, &game, now, reason, &live);
        exit.write(AppExit::Success);
    }
    let line = s.status().compact();
    for (mut text, mut vis) in &mut texts {
        if text.0 != line {
            text.0 = line.clone();
        }
        *vis = if run.show_text {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
    }
}

/// Any other way out (Esc, the window closing): the final dump and the
/// `shutdown` line.
fn at_exit(
    mut events: EventReader<AppExit>,
    time: Res<Time>,
    mut link: NonSendMut<CoopLink>,
    state: Res<DialogueState>,
    game: Res<GameFiles>,
    mut run: ResMut<Run>,
    live: Res<LiveNow>,
) {
    if events.read().next().is_none() {
        return;
    }
    finish(
        &mut link.0,
        &mut run,
        &state.0,
        &game,
        time.elapsed_secs(),
        "exit",
        &live,
    );
}

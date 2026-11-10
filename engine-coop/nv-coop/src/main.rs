//! nv-coop: the game on the upstream engine, with the co-op session in its frame loop.
//!
//! A player starts it as the host or as a guest. Each player runs one `nv-coop`. The loop,
//! the command line, the control port and the state dump are in `coop-app`. This program
//! gives that loop the real game: the upstream engine through the glue.
//!
//! The game's threads run stepped, and each call into the program happens between two
//! frames on the main thread.

use std::io::Write;
use std::sync::atomic::{AtomicI32, Ordering};
use std::time::Instant;

use coop_app::args::{self, Args, USAGE};
use coop_app::{App, Flow, Machine, Screen};
use coop_engine::gate::{Counters, Intent, Rules};
use coop_engine::{Program, Ret};
use coop_engine_glue::{gate, Game};
use coop_session::json::{self, V};

/// The most frames for the game's start and its menu's first frames.
const START_LIMIT: u32 = 20_000;

/// The longest time that one step tells the session, in seconds. A frame that loads a cell
/// can take longer: the session's clock then moves by this much.
const LONGEST_STEP: f32 = 0.25;

/// A step that takes this long gets a line in the log, in milliseconds.
const SLOW_STEP_MS: u128 = 1000;

/// The status of the game's main, when it ended.
static MAIN_STATUS: AtomicI32 = AtomicI32::new(1);

/// The real game as the loop's machine.
struct Real {
    game: Game,
    /// How deep the adapter's bracket is. A gate that comes inside a bracket starts with it.
    own: u32,
}

impl Program for Real {
    fn read(&self, addr: u32, out: &mut [u8]) {
        self.game.read(addr, out);
    }

    fn write(&mut self, addr: u32, bytes: &[u8]) {
        self.game.write(addr, bytes);
    }

    fn call(&mut self, addr: u32, args: &[u32]) -> Ret {
        self.game.call(addr, args)
    }

    fn alloc(&mut self, len: u32) -> u32 {
        self.game.alloc(len)
    }

    fn free(&mut self, addr: u32) {
        self.game.free(addr);
    }

    fn install_gate(&mut self, rules: &Rules, shared_quest: Option<u32>) -> Result<(), String> {
        self.game.install_gate(rules, shared_quest)?;
        // The session's call that asked for the gate is the adapter's own work still.
        for _ in 0..self.own {
            gate::own_enter();
        }
        Ok(())
    }
}

impl Machine for Real {
    fn frame(&mut self) -> bool {
        coop_engine_glue::frame(&mut self.game.0)
    }

    fn screen(&self) -> Screen {
        match coop_engine_glue::screen(&self.game.0) {
            coop_engine_glue::Screen::Play => Screen::Play,
            coop_engine_glue::Screen::Menu => Screen::Menu,
            coop_engine_glue::Screen::Loading => Screen::Loading,
        }
    }

    fn own(&mut self, on: bool) {
        if on {
            self.own += 1;
            gate::own_enter();
        } else {
            self.own = self.own.saturating_sub(1);
            gate::own_leave();
        }
    }

    fn gate_frame(&mut self, play_ms: u32) -> bool {
        gate::frame(play_ms)
    }

    fn take_intents(&mut self) -> Vec<Intent> {
        gate::take_intents()
    }

    fn counters(&self) -> Option<Counters> {
        gate::counters()
    }
}

/// Writes the log of a program that did not reach its session, so that a harness sees the
/// cause: `start`, `error`, `shutdown`.
fn failure_log(args: &Args, message: &str) {
    let Some(path) = &args.log else { return };
    if let Some(dir) = path.parent().filter(|d| !d.as_os_str().is_empty()) {
        let _ = std::fs::create_dir_all(dir);
    }
    let Ok(mut file) = std::fs::File::create(path) else {
        return;
    };
    let role = if args.is_host() { "host" } else { "guest" };
    let lines = [
        json::object(&[
            ("t", V::F(0.0)),
            ("ev", V::S("start")),
            ("role", V::S(role)),
            ("pid", V::U(u64::from(std::process::id()))),
        ]),
        json::object(&[
            ("t", V::F(0.0)),
            ("ev", V::S("error")),
            ("msg", V::S(message)),
        ]),
        json::object(&[
            ("t", V::F(0.0)),
            ("ev", V::S("shutdown")),
            ("reason", V::S("error")),
            ("clean", V::B(false)),
            ("code", V::I(1)),
        ]),
    ];
    for line in lines {
        let _ = writeln!(file, "{line}");
    }
}

/// Ends the program with an error that the player and the harness see.
fn fail(args: &Args, message: &str) -> ! {
    eprintln!("error: {message}");
    failure_log(args, message);
    std::process::exit(1);
}

fn main() {
    let raw: Vec<String> = std::env::args().skip(1).collect();
    if raw.iter().any(|a| a == "--help" || a == "-h") {
        print!("{USAGE}");
        return;
    }
    let args = match args::parse(&raw) {
        Ok(args) => args,
        Err(message) => {
            eprintln!("error: {message}\n\n{USAGE}");
            std::process::exit(2);
        }
    };
    if args.background {
        // The engine reads it at the start: the game's window does not take the focus.
        std::env::set_var("NV_D3D9_BACKGROUND", "1");
    }
    // Stepped threads: a call into the program between two frames needs them.
    let at_end = |status: i32| MAIN_STATUS.store(status, Ordering::SeqCst);
    let mut host = match coop_engine_glue::start(&args.data, true, at_end) {
        Ok(host) => host,
        Err(e) => fail(&args, &e),
    };
    if let Err(e) = coop_engine_glue::reach_menu(&mut host, START_LIMIT) {
        fail(&args, &e);
    }
    let machine = Real {
        game: Game(host),
        own: 0,
    };
    // The load order is the game's own. The names come from the plugin files in that order.
    let files = match coop_engine::files::loaded(&machine) {
        Ok(files) => files,
        Err(e) => fail(&args, &e),
    };
    let names = match coop_app::names::table(&args.data, &files) {
        Ok(names) => names,
        Err(e) => fail(&args, &e),
    };
    let mut app = match App::new(args.clone(), machine, names, &files, None) {
        Ok(app) => app,
        Err(e) => fail(&args, &e),
    };
    if let Some(at) = app.control_addr() {
        println!("nv-coop: the control port is {at}");
    }
    let mut last = Instant::now();
    let mut told_ready = false;
    loop {
        let now = Instant::now();
        let dt = now.duration_since(last).as_secs_f32().min(LONGEST_STEP);
        last = now;
        let flow = app.step(dt);
        // A step that takes long holds the network: the log names it.
        let took = now.elapsed();
        if took.as_millis() >= SLOW_STEP_MS {
            app.session.log_event(
                "slow_step",
                &[
                    ("ms", V::U(took.as_millis() as u64)),
                    ("frame", V::U(app.frames())),
                ],
            );
        }
        match flow {
            Flow::Go => {}
            Flow::Quit(reason) => {
                println!("nv-coop: the program ends ({reason})");
                std::process::exit(0);
            }
            Flow::Ended => {
                let status = MAIN_STATUS.load(Ordering::SeqCst);
                let message = format!("the game's main ended with status {status}");
                eprintln!("error: {message}");
                app.session.log_event("error", &[("msg", V::S(&message))]);
                app.session.log_event(
                    "shutdown",
                    &[
                        ("reason", V::S("game_ended")),
                        ("clean", V::B(false)),
                        ("code", V::I(i64::from(status))),
                    ],
                );
                std::process::exit(if status == 0 { 1 } else { status });
            }
        }
        if app.ready() && !told_ready {
            told_ready = true;
            println!("nv-coop: ready after {} frames", app.frames());
        }
    }
}

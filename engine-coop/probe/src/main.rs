//! coop-probe: the M0 probes of the whole-story co-op plan.
//!
//! The upstream engine runs the game's own translated code. This program starts it from outside
//! the upstream tree, runs the game's frames, and reads and changes game state between frames by
//! program address. It proves that a co-op layer can live out of tree.
//!
//! A probe script gives the steps (`--script FILE`), one step a line:
//!
//! ```text
//! WHEN VERB ARGUMENTS
//! ```
//!
//! `WHEN` is a frame number, `menu+N` (N frames after the first menu frame) or `play+N` (N
//! frames after the first frame of play). The verbs are in [`run_step`].

mod program;

use std::path::PathBuf;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::OnceLock;

use engine::{AbiFn, Engine, Ret};

const USAGE: &str = "\
coop-probe - drive the upstream engine from outside its tree

USAGE:
    coop-probe <DATA FOLDER> [--frames N] [--every N] [--script FILE]

    DATA FOLDER    the game's Data folder (FalloutNV.exe 1.4.0.525 is read from the installation)
    --frames N     end after N of main's frames (default 600)
    --every N      print one status line each N frames (default 60)
    --script FILE  the probe steps: WHEN VERB ARGUMENTS, one a line
    --play         start as the player's game starts (the player's options, real threads),
                   not as a test run (stepped threads)
";

struct Args {
    data: PathBuf,
    frames: u64,
    every: u64,
    script: Option<PathBuf>,
    /// Start as a play run, not as a test run.
    play: bool,
}

fn parse(raw: &[String]) -> Result<Args, String> {
    let mut data = None;
    let mut frames = 600;
    let mut every = 60;
    let mut script = None;
    let mut play = false;
    let mut i = 0;
    while i < raw.len() {
        let number = |i: usize, name: &str| -> Result<u64, String> {
            raw.get(i)
                .and_then(|w| w.parse().ok())
                .ok_or(format!("{name} needs a number"))
        };
        match raw[i].as_str() {
            "--frames" => {
                i += 1;
                frames = number(i, "--frames")?;
            }
            "--every" => {
                i += 1;
                every = number(i, "--every")?.max(1);
            }
            "--script" => {
                i += 1;
                script = Some(PathBuf::from(raw.get(i).ok_or("--script needs a FILE")?));
            }
            "--play" => play = true,
            other if data.is_none() && !other.starts_with('-') => data = Some(PathBuf::from(other)),
            other => return Err(format!("unknown argument {other}")),
        }
        i += 1;
    }
    Ok(Args {
        data: data.ok_or("the game's Data folder is missing")?,
        frames,
        every,
        script,
        play,
    })
}

/// When a step runs.
#[derive(Clone, Copy, Debug, PartialEq)]
enum When {
    Frame(u64),
    /// Frames after the first menu frame.
    Menu(u64),
    /// Frames after the first frame of play.
    Play(u64),
}

/// One line of the probe script.
#[derive(Clone, Debug, PartialEq)]
struct Step {
    when: When,
    verb: String,
    args: Vec<String>,
}

fn parse_script(text: &str) -> Result<Vec<Step>, String> {
    let mut steps = Vec::new();
    for (n, line) in text.lines().enumerate() {
        let line = line.split('#').next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }
        let mut words = line.split_whitespace();
        let when = words.next().unwrap_or("");
        let after = |prefix: &str| {
            when.strip_prefix(prefix)
                .and_then(|w| w.parse::<u64>().ok())
        };
        let when = if let Some(n) = after("menu+") {
            When::Menu(n)
        } else if let Some(n) = after("play+") {
            When::Play(n)
        } else {
            When::Frame(
                when.parse()
                    .map_err(|_| format!("line {}: bad WHEN {when}", n + 1))?,
            )
        };
        let verb = words
            .next()
            .ok_or(format!("line {}: no verb", n + 1))?
            .to_ascii_lowercase();
        steps.push(Step {
            when,
            verb,
            args: words.map(str::to_owned).collect(),
        });
    }
    Ok(steps)
}

/// Where the run is, for [`When`].
#[derive(Default)]
struct Clock {
    frame: u64,
    first_menu: Option<u64>,
    first_play: Option<u64>,
}

impl Clock {
    fn due(&self, when: When) -> bool {
        match when {
            When::Frame(n) => self.frame == n,
            When::Menu(n) => self.first_menu.is_some_and(|f| self.frame == f + n),
            When::Play(n) => self.first_play.is_some_and(|f| self.frame == f + n),
        }
    }
}

fn hex(word: Option<&String>) -> Option<u32> {
    u32::from_str_radix(word?.trim_start_matches("0x"), 16).ok()
}

/// The functions that `count` overrode: the address, the original and the calls seen.
static COUNTED: [(OnceLock<(u32, AbiFn)>, AtomicU32); 4] = [
    (OnceLock::new(), AtomicU32::new(0)),
    (OnceLock::new(), AtomicU32::new(0)),
    (OnceLock::new(), AtomicU32::new(0)),
    (OnceLock::new(), AtomicU32::new(0)),
];

fn counted(slot: usize, e: &mut Engine, args: &[u32]) -> Ret {
    COUNTED[slot].1.fetch_add(1, Ordering::Relaxed);
    let (addr, original) = COUNTED[slot].0.get().expect("set before the override");
    let ret = original(e, args);
    println!("PROBE call {addr:08x} args {args:08x?} -> {:08x}", ret.eax);
    ret
}
fn counted0(e: &mut Engine, a: &[u32]) -> Ret {
    counted(0, e, a)
}
fn counted1(e: &mut Engine, a: &[u32]) -> Ret {
    counted(1, e, a)
}
fn counted2(e: &mut Engine, a: &[u32]) -> Ret {
    counted(2, e, a)
}
fn counted3(e: &mut Engine, a: &[u32]) -> Ret {
    counted(3, e, a)
}
const COUNTERS: [AbiFn; 4] = [counted0, counted1, counted2, counted3];

extern "Rust" {
    /// The engine's translations by address (the upstream `engine` crate defines it).
    fn nv_engine_assembled_funcs(out: &mut Vec<(u32, AbiFn)>);
}

/// The translation at `addr`, from the engine's own list.
fn translation(addr: u32) -> Option<AbiFn> {
    let mut list = Vec::new();
    // SAFETY: a plain Rust function that only appends to `list`.
    unsafe { nv_engine_assembled_funcs(&mut list) };
    list.into_iter().find(|(a, _)| *a == addr).map(|(_, f)| f)
}

/// The quests that `watch` reads each frame, with the last stage seen.
#[derive(Default)]
struct Watch {
    quests: Vec<(u32, Option<u8>)>,
    changes: u32,
}

/// Runs one step. False ends the run.
fn run_step(e: &mut Engine, step: &Step, watch: &mut Watch, frame: u64) -> bool {
    let arg = |i: usize| step.args.get(i);
    match step.verb.as_str() {
        // console LINE: the game's console runs LINE.
        "console" => {
            let line = step.args.join(" ");
            let ok = program::console(e, &line);
            println!("PROBE frame {frame} console {line:?} -> {ok}");
        }
        // stage FORM: the stage and the flags of quest FORM (hex form id).
        "stage" => match hex(arg(0)).map(|id| (id, program::quest(e, id))) {
            Some((id, Some(q))) => println!(
                "PROBE frame {frame} quest {id:08x} at {:08x} stage {} flags {:02x}",
                q.at, q.stage, q.flags
            ),
            Some((id, None)) => println!("PROBE frame {frame} quest {id:08x} not found"),
            None => println!("PROBE frame {frame} stage: needs a form id"),
        },
        // global FORM: the value of global FORM (hex form id).
        "global" => match hex(arg(0)).map(|id| (id, program::global(e, id))) {
            Some((id, Some(v))) => println!("PROBE frame {frame} global {id:08x} = {v}"),
            Some((id, None)) => println!("PROBE frame {frame} global {id:08x} not found"),
            None => println!("PROBE frame {frame} global: needs a form id"),
        },
        // setstage FORM STAGE: the game's own TESQuest::SetStage.
        "setstage" => {
            let stage = arg(1).and_then(|w| w.parse::<u8>().ok());
            match (hex(arg(0)), stage) {
                (Some(id), Some(stage)) => {
                    let done = program::set_stage(e, id, stage);
                    println!("PROBE frame {frame} setstage {id:08x} {stage} -> {done:?}");
                }
                _ => println!("PROBE frame {frame} setstage: needs a form id and a stage"),
            }
        }
        // watch FORM: read the quest's stage each frame, and print each change.
        "watch" => {
            if let Some(id) = hex(arg(0)) {
                watch.quests.push((id, None));
            }
        }
        // count ADDR: override the function at ADDR with one that counts and runs the original.
        "count" => {
            let Some(addr) = hex(arg(0)) else {
                println!("PROBE frame {frame} count: needs an address");
                return true;
            };
            let wraps = e.wraps_at(addr).len();
            let slot = COUNTED.iter().position(|(f, _)| f.get().is_none());
            match (translation(addr), slot) {
                (_, _) if wraps > 0 => {
                    println!("PROBE frame {frame} count {addr:08x}: refused, {wraps} wraps there")
                }
                (Some(original), Some(slot)) => {
                    let _ = COUNTED[slot].0.set((addr, original));
                    e.register(addr, COUNTERS[slot]);
                    println!("PROBE frame {frame} count {addr:08x}: override in slot {slot}");
                }
                (None, _) => println!("PROBE frame {frame} count {addr:08x}: no translation"),
                (_, None) => println!("PROBE frame {frame} count {addr:08x}: no free slot"),
            }
        }
        // counts: the calls each override saw, and the stage changes that `watch` saw.
        "counts" => {
            for (f, n) in &COUNTED {
                if let Some((addr, _)) = f.get() {
                    println!(
                        "PROBE frame {frame} counted {addr:08x} calls {}",
                        n.load(Ordering::Relaxed)
                    );
                }
            }
            println!("PROBE frame {frame} watched changes {}", watch.changes);
        }
        // wraps ADDR: how many wraps the address has (the script extender's must stay).
        "wraps" => {
            if let Some(addr) = hex(arg(0)) {
                println!(
                    "PROBE frame {frame} wraps {addr:08x} {}",
                    e.wraps_at(addr).len()
                );
            }
        }
        "quit" => return false,
        other => println!("PROBE frame {frame} unknown verb {other}"),
    }
    true
}

fn main() {
    let raw: Vec<String> = std::env::args().skip(1).collect();
    let args = match parse(&raw) {
        Ok(args) => args,
        Err(message) => {
            eprintln!("error: {message}\n\n{USAGE}");
            std::process::exit(2);
        }
    };
    let steps = match &args.script {
        Some(path) => match std::fs::read_to_string(path)
            .map_err(|e| e.to_string())
            .and_then(|t| parse_script(&t))
        {
            Ok(steps) => steps,
            Err(e) => {
                eprintln!("error: {}: {e}", path.display());
                std::process::exit(2);
            }
        },
        None => Vec::new(),
    };
    // A test run: the game's threads run stepped unless NV_THREADS says, and the player's
    // Fallout.ini options are not read.
    let choice = match host::fixes::Choice::read_with(!args.play) {
        Ok(choice) => choice,
        Err(e) => {
            eprintln!("error: {e}");
            std::process::exit(1);
        }
    };
    let mut game = match host::Host::start_with(&args.data, &choice) {
        Ok(game) => game,
        Err(e) => {
            eprintln!("error: {e}");
            std::process::exit(1);
        }
    };
    println!("{}", game.describe());
    game.at_main_end = Some(Box::new(
        |_: &host::Host, end: host::player_move::MainEnd| {
            println!("PROBE main ended: status {}", end.status());
            std::process::exit(end.status());
        },
    ));
    game.end_if_main_ended();
    let mut clock = Clock::default();
    let mut watch = Watch::default();
    let started = std::time::Instant::now();
    'run: while clock.frame < args.frames {
        game.set_keys_down(&[]);
        game.mouse([0; 3], [false; 8]);
        if !game.main_frame() {
            eprintln!("PROBE error: the game's main is not running");
            std::process::exit(1);
        }
        let screen = host::uncap::screen(&game.engine);
        match screen {
            host::uncap::Screen::Menu if clock.first_menu.is_none() => {
                clock.first_menu = Some(clock.frame);
                println!("PROBE frame {} first menu", clock.frame);
            }
            host::uncap::Screen::Play if clock.first_play.is_none() => {
                clock.first_play = Some(clock.frame);
                println!("PROBE frame {} first play", clock.frame);
            }
            _ => {}
        }
        for (id, last) in &mut watch.quests {
            let now = program::quest(&mut game.engine, *id).map(|q| q.stage);
            if now != *last {
                println!(
                    "PROBE frame {} watch {id:08x} stage {last:?} -> {now:?}",
                    clock.frame
                );
                if last.is_some() {
                    watch.changes += 1;
                }
                *last = now;
            }
        }
        for step in steps.iter().filter(|s| clock.due(s.when)) {
            if !run_step(&mut game.engine, step, &mut watch, clock.frame) {
                break 'run;
            }
        }
        if clock.frame % args.every == 0 {
            match program::player_place(&game.engine) {
                Some((player, at)) => println!(
                    "PROBE frame {} screen {} player {player:08x} at {:.1} {:.1} {:.1}",
                    clock.frame,
                    screen.name(),
                    at[0],
                    at[1],
                    at[2]
                ),
                None => println!(
                    "PROBE frame {} screen {} player none",
                    clock.frame,
                    screen.name()
                ),
            }
        }
        clock.frame += 1;
    }
    let seconds = started.elapsed().as_secs_f64();
    println!(
        "PROBE done: {} frames in {seconds:.1} s ({:.1} a second); {}",
        clock.frame,
        clock.frame as f64 / seconds.max(0.001),
        game.movement.summary()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_script_parses_its_three_kinds_of_when() {
        let steps = parse_script(
            "# a comment\n12 quit\nmenu+30 console coc Goodsprings # go\nplay+5 stage 00104C1C\n",
        )
        .unwrap();
        assert_eq!(steps.len(), 3);
        assert_eq!(steps[0].when, When::Frame(12));
        assert_eq!(steps[1].when, When::Menu(30));
        assert_eq!(steps[1].args, ["coc", "Goodsprings"]);
        assert_eq!(steps[2].when, When::Play(5));
        assert!(parse_script("soon quit").is_err());
        assert!(parse_script("5").is_err());
    }

    #[test]
    fn a_step_is_due_only_at_its_frame() {
        let mut clock = Clock {
            frame: 40,
            first_menu: Some(10),
            first_play: None,
        };
        assert!(clock.due(When::Frame(40)));
        assert!(clock.due(When::Menu(30)));
        assert!(!clock.due(When::Play(0)));
        clock.first_play = Some(40);
        assert!(clock.due(When::Play(0)));
        assert!(!clock.due(When::Play(1)));
    }
}

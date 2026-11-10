//! coop-engine-peer: one game on the upstream engine behind the peer protocol
//! (`coop_engine::peer`).
//!
//! It starts the game, goes from the main menu into a cell, prints `ready`, and then answers
//! one request line from standard input with one line on standard output. The engine prints
//! its own status lines to standard output too, so each line of the server starts with
//! [`MARK`]: a harness reads only those lines. The game advances only when a request says so
//! (`frames N`), so a harness sees a state that does not move under it.

use std::io::{BufRead, Write};
use std::path::PathBuf;

use coop_engine::peer::{intents_reply, stats_reply, Action, Peer, Role};
use coop_engine_glue::{gate, EngineProgram};

const USAGE: &str = "\
coop-engine-peer - one game behind the peer protocol

USAGE:
    coop-engine-peer <DATA FOLDER> --role host|guest [--cell NAME] [--settle N] [--menu]

    DATA FOLDER  the game's Data folder (FalloutNV.exe 1.4.0.525 is read from the installation)
    --role       host: the game runs the story. guest: it gets the story's state
    --cell NAME  the cell the game goes into from its main menu (default GSDocMitchellHouse)
    --settle N   frames of play before `ready` (default 60)
    --menu       wait at the main menu: `ready` comes there, and the request `enter CELL`
                 goes into the game. A guest gets its gate before its first frame of play

Requests come on standard input, one a line. Answers go to standard output, one a line,
each after the mark `@ `. Other lines there are the engine's own.
";

struct Args {
    data: PathBuf,
    role: Role,
    cell: String,
    settle: u32,
    menu: bool,
}

fn parse(raw: &[String]) -> Result<Args, String> {
    let mut data = None;
    let mut role = None;
    let mut cell = "GSDocMitchellHouse".to_string();
    let mut settle = 60;
    let mut menu = false;
    let mut i = 0;
    while i < raw.len() {
        let value = |i: usize, name: &str| raw.get(i).ok_or(format!("{name} needs a value"));
        match raw[i].as_str() {
            "--role" => {
                i += 1;
                role = Some(match value(i, "--role")?.as_str() {
                    "host" => Role::Host,
                    "guest" => Role::Guest,
                    other => return Err(format!("unknown role {other}")),
                });
            }
            "--cell" => {
                i += 1;
                cell = value(i, "--cell")?.clone();
            }
            "--settle" => {
                i += 1;
                settle = value(i, "--settle")?
                    .parse()
                    .map_err(|_| "--settle needs a number")?;
            }
            "--menu" => menu = true,
            other if data.is_none() && !other.starts_with('-') => data = Some(PathBuf::from(other)),
            other => return Err(format!("unknown argument {other}")),
        }
        i += 1;
    }
    Ok(Args {
        data: data.ok_or("the game's Data folder is missing")?,
        role: role.ok_or("--role is missing")?,
        cell,
        settle,
        menu,
    })
}

/// The most frames for the game's start, its menu's first frames and a cell's load.
const START_LIMIT: u32 = 20_000;

/// The length of one stepped frame of a test run, for the gate's blind alarm.
const FRAME_MS: u32 = 16;

/// What each line of the server starts with. Other lines on standard output are the engine's.
const MARK: &str = "@ ";

/// Sends one answer line.
fn say(text: &str) {
    let mut out = std::io::stdout().lock();
    // The harness reads line by line: a closed pipe ends the server at the next request.
    let _ = writeln!(out, "{MARK}{text}");
    let _ = out.flush();
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
    let at_end = |status: i32| {
        say(&format!("err the game's main ended with status {status}"));
        std::process::exit(status);
    };
    let mut game = match coop_engine_glue::start(&args.data, true, at_end) {
        Ok(game) => game,
        Err(e) => {
            say(&format!("err {e}"));
            std::process::exit(1);
        }
    };
    // The limit covers the start, the menu's first frames and the cell's load.
    let reached = if args.menu {
        coop_engine_glue::reach_menu(&mut game, START_LIMIT)
    } else {
        coop_engine_glue::reach_play(&mut game, &args.cell, args.settle, START_LIMIT)
    };
    match reached {
        Ok(frames) => say(&format!("ready {frames}")),
        Err(e) => {
            say(&format!("err {e}"));
            std::process::exit(1);
        }
    }
    let mut peer = Peer::new(args.role);
    for line in std::io::stdin().lock().lines() {
        let Ok(line) = line else { break };
        peer.last_script = gate::counters().map_or(0, |c| c.last_script);
        // A request is the adapter's own work: what the program runs for it is not gated.
        let action = gate::own(|| peer.handle(&mut EngineProgram(&mut game.engine), &line));
        match action {
            Action::Reply(text) => say(&text),
            Action::Gate {
                rules,
                shared_quest,
            } => {
                let done = gate::install(&mut game.engine, rules)
                    .and_then(|()| gate::self_check(&mut game.engine, shared_quest));
                match done {
                    Ok(()) => say("ok"),
                    Err(why) => say(&format!("err {why}")),
                }
            }
            Action::Intents => say(&intents_reply(&gate::take_intents())),
            Action::GateStats => say(&stats_reply(gate::counters())),
            Action::Line { text, own } => {
                let mut run = || {
                    coop_engine::console::run_line(&mut EngineProgram(&mut game.engine), &text, 0)
                };
                let ran = if own { gate::own(run) } else { run() };
                say(if ran {
                    "ok"
                } else {
                    "err the game has no console manager"
                });
            }
            Action::Enter { cell, settle } => {
                match coop_engine_glue::enter_cell(&mut game, &cell, settle, START_LIMIT) {
                    Ok(frames) => say(&format!("ok {frames}")),
                    Err(why) => say(&format!("err {why}")),
                }
            }
            Action::Frames(n) => {
                let ran = (0..n).all(|_| {
                    let ran = coop_engine_glue::frame(&mut game);
                    if coop_engine_glue::screen(&game) == coop_engine_glue::Screen::Play {
                        gate::frame(FRAME_MS);
                    }
                    ran
                });
                say(if ran {
                    "ok"
                } else {
                    "err the game's main does not run"
                });
            }
            Action::Quit => {
                say("ok");
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn strings(a: &[&str]) -> Vec<String> {
        a.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn the_command_line_needs_a_folder_and_a_role() {
        let args = parse(&strings(&["D:/Data", "--role", "guest", "--settle", "5"])).unwrap();
        assert_eq!(args.role, Role::Guest);
        assert_eq!((args.cell.as_str(), args.settle), ("GSDocMitchellHouse", 5));
        assert!(!args.menu);
        assert!(
            parse(&strings(&["D:/Data", "--role", "guest", "--menu"]))
                .unwrap()
                .menu
        );
        assert!(parse(&strings(&["D:/Data"])).is_err());
        assert!(parse(&strings(&["--role", "host"])).is_err());
        assert!(parse(&strings(&["D:/Data", "--role", "king"])).is_err());
    }
}

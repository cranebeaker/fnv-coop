//! nv-viewer: runs Fallout: New Vegas through the engine port.
//!
//! The player's own FalloutNV.exe (1.4.0.525) is mapped and the game's translated code
//! (`crates/engine`, through `crates/host`) runs its `WinMain` as the main thread; this program
//! only finds the installation, starts the host, gives the game's frames their turn and ends
//! when the game's `main` ends. The game draws in its own window through the real Direct3D 9
//! (`NV_D3D9=real`, docs/adr/0008) and plays its sound through DirectSound (`NV_SOUND=out`);
//! the host reads the keyboard and the mouse from that window.
//!
//! Until 2026-11-03 this was also a hand-written reimplementation of the game drawn with Bevy;
//! that layer was removed (docs/adr/0011-old-layer-removed.md).

use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

const USAGE: &str = "\
nv-viewer - run Fallout: New Vegas through the engine port

USAGE:
    nv-viewer <DATA FOLDER> [OPTIONS]

    DATA FOLDER  the game's Data folder, or the installation folder containing it
                 (FalloutNV.exe 1.4.0.525 is read from the installation)

OPTIONS:
    --movement-missing FILE  write the untranslated functions the game's code reached,
                             with counts, to FILE, its stops to FILE.stops.tsv and the
                             static initializers skipped to FILE.static.tsv (every 5 s
                             and when the game's main ends)
    --movement-probe         for testing: the game's code goes on past functions not
                             translated yet (with zero results) to list more of them
    -h, --help               print this

Other arguments (the removed viewer's place name, --walk, --physics, --wait, --background,
--new-game, --official, ...) are accepted and ignored, so older launchers keep working.

ENVIRONMENT:
    NV_D3D9=real    the game's own window through the real Direct3D 9 (the play build)
    NV_SOUND=out    the game's sound through DirectSound
    docs/PLAYTESTING.md lists the rest (read by crates/host and crates/engine-core).
";

/// The command line.
#[derive(Debug, Default, PartialEq)]
struct Args {
    data: PathBuf,
    movement_missing: Option<PathBuf>,
    movement_probe: bool,
    /// The removed viewer's arguments, ignored.
    ignored: Vec<String>,
}

/// `Ok(None)`: `--help`.
fn parse(raw: &[String]) -> Result<Option<Args>, String> {
    let mut args = Args::default();
    let mut data = None;
    let mut i = 0;
    while i < raw.len() {
        match raw[i].as_str() {
            "-h" | "--help" => return Ok(None),
            "--movement-probe" => args.movement_probe = true,
            "--movement-missing" => {
                i += 1;
                let file = raw.get(i).ok_or("--movement-missing needs a FILE")?;
                args.movement_missing = Some(PathBuf::from(file));
            }
            other if data.is_none() && !other.starts_with('-') => {
                data = Some(PathBuf::from(other));
            }
            other => args.ignored.push(other.to_owned()),
        }
        i += 1;
    }
    args.data = data.ok_or("the game's Data folder is missing")?;
    Ok(Some(args))
}

/// The game's master file, which a Data folder holds.
const MAIN_MASTER: &str = "FalloutNV.esm";

/// The Data folder: `path` itself or its `Data`, whichever holds FalloutNV.esm (any case).
fn find_data_folder(path: &Path) -> Result<PathBuf, String> {
    for folder in [path.to_path_buf(), path.join("Data")] {
        let has_master = std::fs::read_dir(&folder).is_ok_and(|entries| {
            entries.flatten().any(|e| {
                e.file_name()
                    .to_string_lossy()
                    .eq_ignore_ascii_case(MAIN_MASTER)
            })
        });
        if has_master {
            return Ok(folder);
        }
    }
    Err(format!(
        "neither {} nor a Data folder inside it contains {MAIN_MASTER}",
        path.display()
    ))
}

/// `--movement-missing`'s lists, each rewritten whole: the untranslated addresses reached
/// (`path`), the stops (`.stops.tsv`) and the static initializers skipped (`.static.tsv`).
fn write_lists(host: &host::Host, path: &Path) {
    let _ = std::fs::write(path, host.movement.missing_tsv(&|_| None));
    let _ = std::fs::write(path.with_extension("stops.tsv"), host.movement.stops_tsv());
    let _ = std::fs::write(path.with_extension("static.tsv"), host.static_skips_tsv());
}

#[cfg(windows)]
#[link(name = "user32")]
extern "system" {
    fn SetProcessDpiAwarenessContext(context: isize) -> i32;
}

fn main() {
    // With the game's own window (`NV_D3D9=real`) the process is DPI-unaware, as FalloutNV.exe is
    // (no `dpiAware` in its manifest): Windows scales the game's window on a display above 100%
    // from the start, as it does the original's (playtest, 2026-10-30). The same test as the real
    // device's (`d3d9real::wanted`): any case.
    #[cfg(windows)]
    if std::env::var("NV_D3D9").is_ok_and(|v| v.eq_ignore_ascii_case("real")) {
        // DPI_AWARENESS_CONTEXT_UNAWARE.
        // SAFETY: no pointers; a failure leaves the awareness as it was.
        unsafe {
            SetProcessDpiAwarenessContext(-1);
        }
    }
    let raw: Vec<String> = std::env::args().skip(1).collect();
    let args = match parse(&raw) {
        Ok(Some(args)) => args,
        Ok(None) => {
            print!("{USAGE}");
            return;
        }
        Err(message) => {
            eprintln!("error: {message}\n\n{USAGE}");
            std::process::exit(2);
        }
    };
    if !args.ignored.is_empty() {
        println!(
            "Ignored (the removed viewer's arguments): {}",
            args.ignored.join(" ")
        );
    }
    let data = match find_data_folder(&args.data) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("error: {e}");
            std::process::exit(1);
        }
    };
    // The game's own code runs on the player's FalloutNV.exe, read from the same installation
    // as Data: any build but 1.4.0.525 is refused here, with the reason (2026-10-13). Starting
    // runs main's first frame.
    let mut game = match host::Host::start(&data) {
        Ok(host) => host,
        Err(e) => {
            eprintln!("error: {e}");
            std::process::exit(1);
        }
    };
    println!("{}", game.describe());
    game.movement.probe = args.movement_probe;
    // The run ends where the game's `main` ends its frames (`host::Host::at_main_end`), as the
    // original's process does: with main's answer when it returned, with the game's own code
    // when it called `ExitProcess`, and with status 3 when it stopped (a fault, a function not
    // translated, a call the platform does not provide), so the window closes and the host's line
    // naming the end is the log's last. `--movement-missing`'s lists are written first.
    let lists = args.movement_missing.clone();
    game.at_main_end = Some(Box::new(
        move |game: &host::Host, end: host::player_move::MainEnd| {
            if let Some(path) = &lists {
                write_lists(game, path);
            }
            std::process::exit(end.status());
        },
    ));
    // An end in the start-up.
    game.end_if_main_ended();
    // Main's frames. The keyboard and the mouse come from the game's window
    // (`Host::set_keys_down`, `Host::mouse`); scripted runs add `NV_MOVE_KEYS` and
    // `NV_MOVE_MOUSE`. Every 5 s the status line (the heartbeat the launchers watch) and the
    // lists.
    let mut last: Option<Instant> = None;
    loop {
        game.set_keys_down(&[]);
        game.mouse([0; 3], [false; 8]);
        if !game.main_frame() {
            break;
        }
        if last.map_or(true, |at| at.elapsed() >= Duration::from_secs(5)) {
            last = Some(Instant::now());
            println!("{}", game.movement.summary());
            if let Some(path) = &args.movement_missing {
                write_lists(&game, path);
            }
        }
    }
    // No frame and no end: the game's main never ran as the main thread.
    if let Some(path) = &args.movement_missing {
        write_lists(&game, path);
    }
    eprintln!(
        "error: the game's main is not running ({})",
        game.movement.summary()
    );
    std::process::exit(1);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn strings(a: &[&str]) -> Vec<String> {
        a.iter().map(|s| s.to_string()).collect()
    }

    /// play.sh's command line still parses: the Data folder, the old flags ignored.
    #[test]
    fn old_launch_lines_still_parse() {
        let raw = strings(&[
            "C:\\Games\\Data",
            "GSDocMitchellHouse",
            "--background",
            "--walk",
            "--physics",
            "engine",
            "--wait",
            "100000",
            "--movement-missing",
            "m.tsv",
        ]);
        let args = parse(&raw).unwrap().unwrap();
        assert_eq!(args.data, PathBuf::from("C:\\Games\\Data"));
        assert_eq!(args.movement_missing, Some(PathBuf::from("m.tsv")));
        assert!(!args.movement_probe);
        assert_eq!(args.ignored.len(), 7);
    }

    #[test]
    fn help_and_missing_data() {
        assert_eq!(parse(&strings(&["--help"])), Ok(None));
        assert!(parse(&strings(&["--movement-probe"])).is_err());
        assert!(parse(&strings(&["--movement-missing"])).is_err());
    }
}

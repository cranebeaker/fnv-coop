//! The command line: the flags of the viewer contract (`tools/coop-run`).
//!
//! One scenario format drives each program of the contract, so this program knows each flag
//! of it. A flag that only changes how the older program draws has no effect here, and the
//! log says so. A flag that changes what the game or the session does, and that is not
//! built here yet, ends the program: a scenario must not pass because its flag was dropped.

use std::path::PathBuf;

/// What this program does with a flag.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Use {
    /// The program reads it.
    Read,
    /// It changes only how the older program draws or takes input: no effect here.
    NoEffect,
    /// It changes the game or the session, and this program does not do it yet.
    NotBuilt,
}

/// Each flag of the contract: its name, its count of values, and its use.
const FLAGS: &[(&str, usize, Use)] = &[
    ("--coop-host", 1, Use::Read),
    ("--coop-join", 1, Use::Read),
    ("--coop-test-token", 0, Use::Read),
    ("--coop-token", 1, Use::Read),
    ("--coop-name", 1, Use::Read),
    ("--coop-log", 1, Use::Read),
    ("--coop-state-dump", 1, Use::Read),
    ("--coop-control", 1, Use::Read),
    ("--coop-quit-file", 1, Use::Read),
    ("--coop-go-file", 1, Use::Read),
    ("--coop-quit-after", 1, Use::Read),
    ("--coop-no-chargen", 0, Use::Read),
    ("--coop-allow-mismatch", 0, Use::Read),
    ("--coop-net", 1, Use::Read),
    ("--run", 1, Use::Read),
    ("--run-at", 2, Use::Read),
    ("--stage", 2, Use::Read),
    ("--background", 0, Use::Read),
    ("--official", 0, Use::NoEffect),
    ("--keep-root-transforms", 0, Use::NoEffect),
    ("--walk", 0, Use::NoEffect),
    ("--fps", 0, Use::NoEffect),
    ("--movies", 0, Use::NoEffect),
    ("--no-movies", 0, Use::NoEffect),
    ("--no-hud", 0, Use::NoEffect),
    ("--answer-boxes", 0, Use::NoEffect),
    ("--pad", 0, Use::NoEffect),
    ("--brightness", 1, Use::NoEffect),
    ("--screen-size", 1, Use::NoEffect),
    ("--cloud-time", 1, Use::NoEffect),
    ("--weather-region", 1, Use::NoEffect),
    ("--menu-pointer", 1, Use::NoEffect),
    ("--coop-window", 1, Use::NoEffect),
    ("--coop-gamepad", 1, Use::NoEffect),
    ("--coop-ptt-key", 1, Use::NoEffect),
    ("--coop-ping-key", 1, Use::NoEffect),
    ("--coop-voice", 1, Use::NoEffect),
    ("--new-game", 0, Use::NotBuilt),
    ("--talk", 0, Use::NotBuilt),
    ("--freeze-ai", 0, Use::NotBuilt),
    ("--coop-steam", 0, Use::NotBuilt),
    ("--coop-steam-friend", 0, Use::NotBuilt),
    ("--plugins", 1, Use::NotBuilt),
    ("--ini", 1, Use::NotBuilt),
    ("--at", 1, Use::NotBuilt),
    ("--screenshot", 1, Use::NotBuilt),
    ("--wait", 1, Use::NotBuilt),
    ("--choose", 1, Use::NotBuilt),
    ("--load-fos", 1, Use::NotBuilt),
    ("--character", 1, Use::NotBuilt),
    ("--weapon", 1, Use::NotBuilt),
    ("--say", 1, Use::NotBuilt),
    ("--pipboy", 1, Use::NotBuilt),
    ("--pipboy-keys", 1, Use::NotBuilt),
    ("--lockpick", 1, Use::NotBuilt),
    ("--open-menu", 1, Use::NotBuilt),
    ("--use", 1, Use::NotBuilt),
    ("--menu-click", 1, Use::NotBuilt),
    ("--menu-keys", 1, Use::NotBuilt),
    ("--coop-load", 1, Use::NotBuilt),
    ("--coop-character", 1, Use::NotBuilt),
    ("--coop-player-key", 1, Use::NotBuilt),
    ("--coop-couch", 1, Use::NotBuilt),
    ("--coop-difficulty", 1, Use::NotBuilt),
    ("--coop-friendly-fire", 1, Use::NotBuilt),
    ("--key-at", 2, Use::NotBuilt),
];

/// The text of `--help`.
pub const USAGE: &str = "\
nv-coop - the game on the upstream engine, with the co-op session in its frame loop

USAGE:
    nv-coop <DATA FOLDER> [CELL] --coop-host PORT|ADDR:PORT [OPTIONS]
    nv-coop <DATA FOLDER> --coop-join ADDR:PORT --coop-no-chargen [OPTIONS]

    CELL                  the interior cell that a host starts in (editor id)
    --coop-host P         host a session on this UDP port (ADDR:PORT binds one address)
    --coop-join A         join the host at this address or join code
    --coop-test-token     the fixed token of a test session (a plain ADDR:PORT needs it)
    --coop-token HEX      the host's session token, 32 hex digits
    --coop-name NAME      the player's name in the session
    --coop-no-chargen     a guest plays with no character making (needed until it is built)
    --coop-allow-mismatch a host lets a guest with another game version in
    --coop-net PROFILE    a network fault profile for the outgoing link (tests)
    --coop-log FILE       the event log, one JSON object a line
    --coop-state-dump F   the state dump, written again when the state changes
    --coop-control PORT   the control port on 127.0.0.1 (0: a free port)
    --coop-go-file FILE   the timed lines wait until this file is there
    --coop-quit-file F    the program ends when this file is there
    --coop-quit-after S   the program ends S seconds after the session is ready
    --run LINE            a script line, run when the session is ready
    --run-at S LINE       a script line, S seconds after the session is ready
    --stage QUEST N       a quest stage, set when the session is ready
    --background          the game's window does not take the focus
";

/// What the command line asks for.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Args {
    /// The game's Data folder.
    pub data: PathBuf,
    /// The cell that a host starts in (editor id).
    pub cell: Option<String>,
    /// `--coop-host`: the port, or `ADDR:PORT`.
    pub host: Option<String>,
    /// `--coop-join`: the host's address or a join code.
    pub join: Option<String>,
    /// `--coop-test-token`.
    pub test_token: bool,
    /// `--coop-token`: 16 bytes.
    pub token: Option<[u8; 16]>,
    /// `--coop-name`.
    pub name: Option<String>,
    /// `--coop-log`.
    pub log: Option<PathBuf>,
    /// `--coop-state-dump`.
    pub state_dump: Option<PathBuf>,
    /// `--coop-control`.
    pub control: Option<u16>,
    /// `--coop-quit-file`.
    pub quit_file: Option<PathBuf>,
    /// `--coop-go-file`.
    pub go_file: Option<PathBuf>,
    /// `--coop-quit-after`, in seconds.
    pub quit_after: Option<f32>,
    /// `--coop-no-chargen`.
    pub no_chargen: bool,
    /// `--coop-allow-mismatch`.
    pub allow_mismatch: bool,
    /// `--coop-net`: the name of a fault profile.
    pub net: Option<String>,
    /// `--run` and `--run-at`: seconds after the session is ready, and the line.
    pub lines: Vec<(f32, String)>,
    /// `--stage`: a quest's editor id and a stage.
    pub stages: Vec<(String, u16)>,
    /// `--background`.
    pub background: bool,
    /// The flags that have no effect in this program, as given.
    pub no_effect: Vec<String>,
}

impl Args {
    /// True for a host.
    pub fn is_host(&self) -> bool {
        self.host.is_some()
    }
}

/// Parses 32 hex digits into a session token.
fn token(text: &str) -> Option<[u8; 16]> {
    let text = text.trim();
    if text.len() != 32 || !text.is_ascii() {
        return None;
    }
    let mut out = [0u8; 16];
    for (i, b) in out.iter_mut().enumerate() {
        *b = u8::from_str_radix(&text[i * 2..i * 2 + 2], 16).ok()?;
    }
    Some(out)
}

/// Reads the command line (without the program's name). `Err` is the message for the
/// player: the program then ends with status 2.
pub fn parse(raw: &[String]) -> Result<Args, String> {
    let mut args = Args::default();
    let mut places: Vec<&String> = Vec::new();
    let mut i = 0;
    while i < raw.len() {
        let flag = raw[i].as_str();
        if !flag.starts_with("--") {
            places.push(&raw[i]);
            i += 1;
            continue;
        }
        let Some((_, count, kind)) = FLAGS.iter().find(|(name, ..)| *name == flag) else {
            return Err(format!("unknown option {flag}"));
        };
        let values = raw
            .get(i + 1..i + 1 + count)
            .filter(|v| v.len() == *count)
            .ok_or_else(|| format!("{flag} needs {count} value(s)"))?;
        i += 1 + count;
        match kind {
            Use::NotBuilt => {
                return Err(format!(
                    "{flag} is not built in this program yet (the game on the upstream engine)"
                ))
            }
            Use::NoEffect => {
                args.no_effect.push(flag.to_string());
                continue;
            }
            Use::Read => {}
        }
        let seconds = |text: &String| {
            text.parse::<f32>()
                .ok()
                .filter(|s| s.is_finite() && *s >= 0.0)
                .ok_or_else(|| format!("{flag}: {text} is not a count of seconds"))
        };
        match flag {
            "--coop-host" => args.host = Some(values[0].clone()),
            "--coop-join" => args.join = Some(values[0].clone()),
            "--coop-test-token" => args.test_token = true,
            "--coop-token" => {
                args.token = Some(
                    token(&values[0])
                        .ok_or_else(|| format!("{flag}: {} is not 32 hex digits", values[0]))?,
                )
            }
            "--coop-name" => args.name = Some(values[0].clone()),
            "--coop-log" => args.log = Some(PathBuf::from(&values[0])),
            "--coop-state-dump" => args.state_dump = Some(PathBuf::from(&values[0])),
            "--coop-control" => {
                args.control = Some(
                    values[0]
                        .parse()
                        .map_err(|_| format!("{flag}: {} is not a port", values[0]))?,
                )
            }
            "--coop-quit-file" => args.quit_file = Some(PathBuf::from(&values[0])),
            "--coop-go-file" => args.go_file = Some(PathBuf::from(&values[0])),
            "--coop-quit-after" => args.quit_after = Some(seconds(&values[0])?),
            "--coop-no-chargen" => args.no_chargen = true,
            "--coop-allow-mismatch" => args.allow_mismatch = true,
            "--coop-net" => args.net = Some(values[0].clone()),
            "--run" => args.lines.push((0.0, values[0].clone())),
            "--run-at" => args.lines.push((seconds(&values[0])?, values[1].clone())),
            "--stage" => args.stages.push((
                values[0].clone(),
                values[1]
                    .parse()
                    .map_err(|_| format!("{flag}: {} is not a stage", values[1]))?,
            )),
            "--background" => args.background = true,
            other => return Err(format!("{other} has no reader")),
        }
    }
    match places.as_slice() {
        [] => return Err("the game's Data folder is missing".into()),
        [data] => args.data = PathBuf::from(data),
        [data, cell] => {
            args.data = PathBuf::from(data);
            args.cell = Some((*cell).clone());
        }
        _ => return Err("too many arguments".into()),
    }
    match (&args.host, &args.join) {
        (Some(_), Some(_)) | (None, None) => {
            return Err("give one of --coop-host and --coop-join".into())
        }
        (Some(_), None) if args.cell.is_none() => {
            return Err("a host needs the cell to start in".into())
        }
        (None, Some(_)) if !args.no_chargen => {
            return Err(
                "the character making is not built in this program yet: give --coop-no-chargen"
                    .into(),
            )
        }
        _ => {}
    }
    // The lines run in the order of their times. Lines of one time keep their order.
    args.lines.sort_by(|a, b| a.0.total_cmp(&b.0));
    Ok(args)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn line(text: &str) -> Vec<String> {
        text.split_whitespace().map(str::to_string).collect()
    }

    #[test]
    fn a_host_and_a_guest_are_read() {
        let host = parse(&line(
            "D:/Data GSDocMitchellHouse --coop-host 47500 --coop-test-token --coop-log h.jsonl \
             --coop-state-dump h.json --coop-control 0 --run-at 2.5 player.additem --stage VCG03 10 \
             --run first",
        ))
        .unwrap();
        assert!(host.is_host());
        assert_eq!(host.cell.as_deref(), Some("GSDocMitchellHouse"));
        assert_eq!(host.host.as_deref(), Some("47500"));
        assert_eq!(host.control, Some(0));
        assert_eq!(
            host.lines,
            [
                (0.0, "first".to_string()),
                (2.5, "player.additem".to_string())
            ],
            "in the order of their times"
        );
        assert_eq!(host.stages, [("VCG03".to_string(), 10)]);

        let guest = parse(&line(
            "D:/Data --coop-join 127.0.0.1:47500 --coop-test-token --coop-no-chargen \
             --coop-token 000102030405060708090a0b0c0d0e0f",
        ))
        .unwrap();
        assert!(!guest.is_host());
        assert_eq!(guest.token.map(|t| t[15]), Some(15));
    }

    /// A flag with no effect is noted. A flag that is not built ends the program, and so
    /// does a flag that the contract does not have.
    #[test]
    fn a_flag_is_read_or_noted_or_refused() {
        let args = parse(&line("D:/Data Cell --coop-host 1 --no-hud --brightness 2")).unwrap();
        assert_eq!(args.no_effect, ["--no-hud", "--brightness"]);
        for (text, cause) in [
            ("D:/Data Cell --coop-host 1 --new-game", "not built"),
            ("D:/Data Cell --coop-host 1 --coop-load p.sav", "not built"),
            (
                "D:/Data Cell --coop-host 1 --no-such-flag",
                "unknown option",
            ),
            ("D:/Data Cell --coop-host", "needs 1 value"),
            ("D:/Data Cell --coop-host 1 --run-at soon x", "seconds"),
            ("D:/Data Cell", "one of --coop-host and --coop-join"),
            ("D:/Data Cell --coop-host 1 --coop-join a:1", "one of"),
            ("D:/Data --coop-host 1", "needs the cell"),
            ("D:/Data --coop-join a:1", "--coop-no-chargen"),
            ("--coop-host 1", "Data folder"),
            ("a b c --coop-host 1", "too many"),
        ] {
            let why = parse(&line(text)).unwrap_err();
            assert!(why.contains(cause), "{text}: {why}");
        }
    }

    /// Each flag is in the table one time.
    #[test]
    fn the_table_names_each_flag_one_time() {
        for (i, (name, ..)) in FLAGS.iter().enumerate() {
            assert!(
                FLAGS[i + 1..].iter().all(|(other, ..)| other != name),
                "{name}"
            );
        }
    }
}

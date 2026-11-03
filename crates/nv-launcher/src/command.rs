//! Building the exact viewer command line (pure; `--print-command` prints it).

use std::path::PathBuf;

/// The cell a host starts in when not starting a new game.
pub const DEFAULT_CELL: &str = "GSDocMitchellHouse";

#[derive(Debug, Clone, PartialEq)]
pub enum Mode {
    /// Host on `port`. `new_game` starts the opening; otherwise the cell is loaded and the host
    /// can press F9 to load the campaign's quicksave.
    Host { port: u16, new_game: bool },
    /// Join with an NVRS- code (UDP or a Steam lobby).
    Join { addr: String },
    /// Join whichever Steam friend is hosting, no code.
    JoinFriend,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Plan {
    pub viewer: PathBuf,
    pub data: PathBuf,
    pub name: String,
    pub mode: Mode,
    /// The viewer runs here, so its relative save file lands in the campaign's folder.
    pub cwd: PathBuf,
    pub coop_log: PathBuf,
    pub uncap: bool,
    pub windowed: bool,
    pub width: u32,
    pub height: u32,
    /// Pass `--coop-steam` (only when the DLL is there and the viewer advertises the flag).
    pub steam: bool,
    /// Host: the session token, passed as `--coop-token` so the join code matches.
    pub token: Option<[u8; 16]>,
    /// A save to start from (`--coop-load`): the host's Continue; a guest's own character.
    pub load: Option<PathBuf>,
    /// Compatibility override, voice and couch options (see `extras`).
    pub extras: crate::extras::Extras,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Spec {
    pub program: PathBuf,
    pub args: Vec<String>,
    pub env: Vec<(String, String)>,
    pub cwd: PathBuf,
}

pub fn build(p: &Plan) -> Spec {
    let mut args = vec![p.data.to_string_lossy().into_owned()];
    match &p.mode {
        Mode::Host { port, new_game } => {
            if *new_game {
                // the normal opening: Doc Mitchell, name, Vit-o-Matic, tags, traits
                args.push("--new-game".into());
            } else if let Some(l) = &p.load {
                args.push("--coop-load".into());
                args.push(l.to_string_lossy().into_owned());
            } else {
                args.push(DEFAULT_CELL.into());
            }
            args.push("--coop-host".into());
            args.push(port.to_string());
            if let Some(t) = &p.token {
                args.push("--coop-token".into());
                args.push(crate::joincode::token_hex(t));
            }
            // the campaign rules (the party save keeps them)
            args.extend(p.extras.host_args());
        }
        Mode::Join { addr } => {
            args.push("--coop-join".into());
            args.push(addr.clone());
        }
        Mode::JoinFriend => args.push("--coop-steam-friend".into()),
    }
    // a guest brings only its own character from a save; the host's world is never replaced
    if let (Some(l), Mode::Join { .. } | Mode::JoinFriend) = (&p.load, &p.mode) {
        args.push("--coop-load".into());
        args.push(l.to_string_lossy().into_owned());
    }
    args.push("--coop-name".into());
    args.push(p.name.clone());
    args.push("--coop-log".into());
    args.push(p.coop_log.to_string_lossy().into_owned());
    if p.steam && p.mode != Mode::JoinFriend {
        args.push("--coop-steam".into());
    }
    args.extend(p.extras.args());
    let mut env = Vec::new();
    if p.uncap {
        env.push(("NV_UNCAP".to_string(), "1".to_string()));
    }
    // Reserved for the viewer's window options; builds that do not read them ignore them.
    if p.windowed {
        env.push(("NV_WINDOWED".to_string(), "1".to_string()));
        env.push((
            "NV_RESOLUTION".to_string(),
            format!("{}x{}", p.width, p.height),
        ));
    }
    Spec {
        program: p.viewer.clone(),
        args,
        env,
        cwd: p.cwd.clone(),
    }
}

fn quote(a: &str) -> String {
    if !a.is_empty() && !a.contains([' ', '"', '\t']) {
        a.to_string()
    } else {
        format!("\"{}\"", a.replace('"', "\\\""))
    }
}

/// The command as a player would type it: environment, working folder, program and arguments.
pub fn render(s: &Spec) -> String {
    let mut o = String::new();
    for (k, v) in &s.env {
        o.push_str(&format!("env: {k}={v}\n"));
    }
    o.push_str(&format!("cwd: {}\n", s.cwd.display()));
    o.push_str("cmd: ");
    o.push_str(&quote(&s.program.to_string_lossy()));
    for a in &s.args {
        o.push(' ');
        o.push_str(&quote(a));
    }
    o.push('\n');
    o
}

#[cfg(test)]
mod tests {
    use super::*;

    fn plan(mode: Mode) -> Plan {
        Plan {
            viewer: PathBuf::from(r"C:\nv\app\nv-viewer.exe"),
            data: PathBuf::from(r"D:\Games\Fallout New Vegas\Data"),
            name: "Ann".into(),
            mode,
            cwd: PathBuf::from(r"C:\save\host"),
            coop_log: PathBuf::from(r"C:\logs\coop.jsonl"),
            uncap: false,
            windowed: false,
            width: 1280,
            height: 720,
            steam: false,
            token: None,
            load: None,
            extras: Default::default(),
        }
    }

    #[test]
    fn host_new_game() {
        let s = build(&plan(Mode::Host {
            port: 27961,
            new_game: true,
        }));
        assert_eq!(
            s.args,
            vec![
                r"D:\Games\Fallout New Vegas\Data",
                "--new-game",
                "--coop-host",
                "27961",
                "--coop-name",
                "Ann",
                "--coop-log",
                r"C:\logs\coop.jsonl"
            ]
        );
        assert!(s.env.is_empty());
    }

    #[test]
    fn host_without_a_save_uses_the_cell() {
        let s = build(&plan(Mode::Host {
            port: 5000,
            new_game: false,
        }));
        assert_eq!(s.args[1], DEFAULT_CELL);
        assert!(!s.args.contains(&"--new-game".to_string()));
    }

    #[test]
    fn host_continue_loads_the_save() {
        let mut p = plan(Mode::Host {
            port: 5000,
            new_game: false,
        });
        p.load = Some(PathBuf::from(
            r"C:\home\savesb\host-ann
v-rs-quicksave.txt",
        ));
        let s = build(&p);
        assert_eq!(
            &s.args[1..3],
            [
                "--coop-load",
                r"C:\home\savesb\host-ann
v-rs-quicksave.txt"
            ]
        );
        assert_eq!(s.args[3], "--coop-host");
        assert!(!s.args.contains(&DEFAULT_CELL.to_string()));
        // a new game never loads
        let mut n = plan(Mode::Host {
            port: 5000,
            new_game: true,
        });
        n.load = p.load.clone();
        assert!(!build(&n).args.contains(&"--coop-load".to_string()));
    }

    #[test]
    fn a_guest_save_means_own_character() {
        let mut p = plan(Mode::Join {
            addr: "NVRS-X".into(),
        });
        p.load = Some(PathBuf::from("g.txt"));
        let s = build(&p);
        let i = s
            .args
            .iter()
            .position(|a| a == "--coop-load")
            .expect("flag");
        assert_eq!(s.args[i + 1], "g.txt");
        assert!(!s.args.contains(&"--new-game".to_string()));
        assert!(!s.args.contains(&"--coop-host".to_string()));
    }

    #[test]
    fn guest_has_no_cell_and_no_new_game() {
        let s = build(&plan(Mode::Join {
            addr: "1.2.3.4:27961".into(),
        }));
        assert_eq!(s.args[1], "--coop-join");
        assert_eq!(s.args[2], "1.2.3.4:27961");
        assert!(!s
            .args
            .iter()
            .any(|a| a == "--new-game" || a == "--coop-host"));
    }

    #[test]
    fn host_passes_its_token() {
        let mut p = plan(Mode::Host {
            port: 27961,
            new_game: true,
        });
        p.token = Some([0xab; 16]);
        let s = build(&p);
        let i = s
            .args
            .iter()
            .position(|a| a == "--coop-token")
            .expect("token flag");
        assert_eq!(s.args[i + 1], "ab".repeat(16));
        assert!(!s.args.contains(&"--coop-test-token".to_string()));
    }

    #[test]
    fn steam_friend_join_and_steam_host_flags() {
        let mut p = plan(Mode::JoinFriend);
        p.steam = true;
        let s = build(&p);
        assert!(s.args.contains(&"--coop-steam-friend".to_string()));
        assert!(!s.args.contains(&"--coop-join".to_string()));
        assert!(
            !s.args.contains(&"--coop-steam".to_string()),
            "friend join implies steam"
        );
        let mut h = plan(Mode::Host {
            port: 1,
            new_game: false,
        });
        h.steam = true;
        assert!(build(&h).args.contains(&"--coop-steam".to_string()));
        h.steam = false;
        assert!(!build(&h).args.contains(&"--coop-steam".to_string()));
    }

    #[test]
    fn env_and_steam() {
        let mut p = plan(Mode::Host {
            port: 1,
            new_game: true,
        });
        p.uncap = true;
        p.windowed = true;
        p.steam = true;
        let s = build(&p);
        assert!(s.env.contains(&("NV_UNCAP".into(), "1".into())));
        assert!(s.env.contains(&("NV_RESOLUTION".into(), "1280x720".into())));
        assert!(s.args.contains(&"--coop-steam".to_string()));
    }

    #[test]
    fn render_quotes_spaces() {
        let r = render(&build(&plan(Mode::Join { addr: "h:1".into() })));
        assert!(r.contains("cmd: C:\\nv\\app\\nv-viewer.exe \"D:\\Games\\Fallout New Vegas\\Data\" --coop-join h:1"));
        assert!(r.contains("cwd: C:\\save\\host"));
    }
}

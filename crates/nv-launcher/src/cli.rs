//! Command-line flags and the pure step from settings to a viewer command.

use crate::command::{self, Mode, Plan};
use crate::joincode::{self, JoinInput};
use crate::party::{self, Role};
use crate::settings::{clean_name, Settings};
use std::path::{Path, PathBuf};

pub const HELP: &str = "\
nv-launcher: co-op launcher for nv-rs

Usage:
  nv-launcher                      open the menu
  nv-launcher --selftest           check the launcher itself and exit (0 = all passed)
  nv-launcher --print-command MODE print the viewer command line without starting it
        MODE is host or join. Options for it:
        --join ADDR        NVRS- code or HOST[:PORT] (join)
        --continue ID      host: continue campaign ID (16 hex digits); without it a new game
        --campaign ID      join: keep this guest's saves with campaign ID
        --data DIR         the game's Data folder (else from settings)
        --viewer EXE       nv-viewer.exe (else next to the launcher)
        --name NAME  --port N  --uncap
  nv-launcher --diagnostics        collect shareable logs into a folder and print its path
  nv-launcher --port-check [PORT]  local port tests
  nv-launcher --port-listen [PORT] answer probes for 60 seconds
  nv-launcher --port-probe ADDR    send a probe to a listening launcher
  --home DIR                       use DIR instead of %APPDATA%\\nv-rs-coop
  --help, --version
";

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Cli {
    pub help: bool,
    pub version: bool,
    pub selftest: bool,
    pub diagnostics: bool,
    pub print_command: Option<String>,
    pub port_check: Option<Option<u16>>,
    pub port_listen: Option<Option<u16>>,
    pub port_probe: Option<String>,
    pub home: Option<PathBuf>,
    pub data: Option<String>,
    pub viewer: Option<PathBuf>,
    pub name: Option<String>,
    pub port: Option<u16>,
    pub join: Option<String>,
    pub continue_id: Option<String>,
    pub campaign: Option<String>,
    pub uncap: bool,
}

fn take(args: &[String], i: &mut usize, name: &str) -> Result<String, String> {
    *i += 1;
    args.get(*i).cloned().ok_or_else(|| format!("{name} needs a value"))
}

pub fn parse(args: &[String]) -> Result<Cli, String> {
    let mut c = Cli::default();
    let mut i = 0;
    let port_of = |s: &str| -> Result<u16, String> { s.parse::<u16>().ok().filter(|p| *p != 0).ok_or_else(|| format!("'{s}' is not a port number")) };
    while i < args.len() {
        let a = args[i].as_str();
        match a {
            "--help" | "-h" | "/?" => c.help = true,
            "--version" => c.version = true,
            "--selftest" => c.selftest = true,
            "--diagnostics" => c.diagnostics = true,
            "--uncap" => c.uncap = true,
            "--print-command" => {
                let m = take(args, &mut i, "--print-command")?;
                if m != "host" && m != "join" {
                    return Err("--print-command takes host or join".into());
                }
                c.print_command = Some(m);
            }
            "--port-check" | "--port-listen" => {
                let optional = args.get(i + 1).filter(|n| !n.starts_with("--")).cloned();
                let p = match optional {
                    Some(n) => {
                        i += 1;
                        Some(port_of(&n)?)
                    }
                    None => None,
                };
                if a == "--port-check" { c.port_check = Some(p) } else { c.port_listen = Some(p) }
            }
            "--port-probe" => c.port_probe = Some(take(args, &mut i, a)?),
            "--home" => c.home = Some(take(args, &mut i, a)?.into()),
            "--data" => c.data = Some(take(args, &mut i, a)?),
            "--viewer" => c.viewer = Some(take(args, &mut i, a)?.into()),
            "--name" => c.name = Some(take(args, &mut i, a)?),
            "--port" => c.port = Some(port_of(&take(args, &mut i, a)?)?),
            "--join" => c.join = Some(take(args, &mut i, a)?),
            "--continue" => c.continue_id = Some(take(args, &mut i, a)?),
            "--campaign" => c.campaign = Some(take(args, &mut i, a)?),
            other => return Err(format!("unknown option '{other}' (try --help)")),
        }
        i += 1;
    }
    Ok(c)
}

/// Inputs to [`make_plan`].
#[derive(Debug, Clone)]
pub struct LaunchInput {
    pub settings: Settings,
    pub home: PathBuf,
    pub viewer: PathBuf,
    pub host: bool,
    /// Host only: start a new game (else continue `campaign`).
    pub new_game: bool,
    pub join: Option<JoinInput>,
    pub campaign: Option<String>,
    pub random: [u8; 8],
    pub steam: bool,
    /// Host: the session token (random per hosted session).
    pub token: [u8; 16],
    /// Used in log file names.
    pub stamp: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Launch {
    pub plan: Plan,
    pub campaign: String,
    pub role: Role,
    pub player: String,
}

const STEAM_NEEDED: &str =
    "That join needs Steam: Steam must be running and steam_api64.dll must be next to the launcher (see docs/COOP-STEAM.md).";

pub fn make_plan(i: &LaunchInput) -> Result<Launch, String> {
    let s = &i.settings;
    if s.data_path.trim().is_empty() {
        return Err("The game's Data folder is not set. Run Setup first (or pass --data).".into());
    }
    let player = clean_name(&s.player_name);
    let (role, mode, campaign) = if i.host {
        let campaign = match (&i.campaign, i.new_game) {
            (Some(c), _) => party::parse_campaign_id(c).ok_or("The campaign id must be 16 hex digits.")?,
            (None, true) => party::new_campaign_id(i.random),
            (None, false) => return Err("Continue needs a campaign id (--continue ID).".into()),
        };
        (Role::Host, Mode::Host { port: s.port, new_game: i.new_game }, campaign)
    } else {
        let j = i.join.as_ref().ok_or("Join needs a join code.")?;
        let mode = match j {
            JoinInput::Addr(_) => {
                return Err("A plain address has no session token. Ask the host for the join code (NVRS-...).".into())
            }
            JoinInput::Code { target: coop_net::JoinTarget::SteamLobby(_), .. } if !i.steam => {
                return Err(STEAM_NEEDED.into());
            }
            JoinInput::SteamFriend if !i.steam => return Err(STEAM_NEEDED.into()),
            JoinInput::SteamFriend => Mode::JoinFriend,
            JoinInput::Code { .. } => Mode::Join { addr: j.viewer_arg().to_string() },
        };
        let campaign = match &i.campaign {
            Some(c) => party::parse_campaign_id(c).ok_or("The campaign id must be 16 hex digits.")?,
            None => party::new_campaign_id(i.random),
        };
        (Role::Guest, mode, campaign)
    };
    let cwd = i.home.join("saves").join(&campaign).join(party::player_folder(role, &player));
    let plan = Plan {
        viewer: i.viewer.clone(),
        data: PathBuf::from(&s.data_path),
        name: player.clone(),
        mode,
        cwd,
        coop_log: i.home.join("logs").join(format!("coop-{}.jsonl", i.stamp)),
        uncap: s.uncap,
        windowed: s.windowed,
        width: s.width,
        height: s.height,
        steam: i.steam,
        token: if i.host { Some(i.token) } else { None },
    };
    Ok(Launch { plan, campaign, role, player })
}

/// The viewer to run: the given path, else `app\nv-viewer.exe` or `nv-viewer.exe` beside the launcher.
pub fn find_viewer(given: Option<&Path>, exe_dir: &Path) -> PathBuf {
    if let Some(g) = given {
        return g.to_path_buf();
    }
    for rel in [Path::new("app").join("nv-viewer.exe"), PathBuf::from("nv-viewer.exe")] {
        let p = exe_dir.join(rel);
        if p.is_file() {
            return p;
        }
    }
    exe_dir.join("app").join("nv-viewer.exe")
}

/// Builds the `--print-command` text from the flags and loaded settings.
pub fn print_command(c: &Cli, mut settings: Settings, home: &Path, exe_dir: &Path, random: [u8; 8]) -> Result<String, String> {
    let mode = c.print_command.as_deref().ok_or("no mode")?;
    if let Some(d) = &c.data {
        settings.data_path = d.clone();
    }
    if let Some(n) = &c.name {
        settings.player_name = clean_name(n);
    }
    if let Some(p) = c.port {
        settings.port = p;
    }
    if c.uncap {
        settings.uncap = true;
    }
    let host = mode == "host";
    let join = match (&c.join, host) {
        (Some(j), false) => Some(joincode::parse_join(j)?),
        (None, false) => return Err("join mode needs --join".into()),
        _ => None,
    };
    let join_for_steam = join;
    let launch = make_plan(&LaunchInput {
        settings,
        home: home.to_path_buf(),
        viewer: find_viewer(c.viewer.as_deref(), exe_dir),
        host,
        new_game: host && c.continue_id.is_none(),
        join: join_for_steam.clone(),
        campaign: if host { c.continue_id.clone() } else { c.campaign.clone() },
        random,
        // A dry run of a Steam join shows the Steam flags.
        steam: matches!(&join_for_steam, Some(JoinInput::Code { target: coop_net::JoinTarget::SteamLobby(_), .. })),
        token: joincode::token_from_seed(random),
        stamp: "dryrun".into(),
    })?;
    Ok(command::render(&command::build(&launch.plan)))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn v(a: &[&str]) -> Vec<String> {
        a.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn parses_flags() {
        let c = parse(&v(&["--print-command", "join", "--join", "1.2.3.4", "--uncap", "--port", "4000"])).unwrap();
        assert_eq!(c.print_command.as_deref(), Some("join"));
        assert_eq!(c.port, Some(4000));
        assert!(c.uncap);
        assert_eq!(parse(&v(&["--port-check"])).unwrap().port_check, Some(None));
        assert_eq!(parse(&v(&["--port-check", "5000", "--selftest"])).unwrap().port_check, Some(Some(5000)));
        assert!(parse(&v(&["--selftest"])).unwrap().selftest);
    }

    #[test]
    fn rejects_bad_flags() {
        for bad in [v(&["--nope"]), v(&["--port", "0"]), v(&["--port"]), v(&["--print-command", "fly"]), v(&["--port-check", "99999"])] {
            assert!(parse(&bad).is_err(), "{bad:?}");
        }
    }

    fn settings() -> Settings {
        Settings { data_path: r"D:\FNV\Data".into(), player_name: "Ann".into(), ..Settings::default() }
    }

    #[test]
    fn dry_run_host_new_game() {
        let c = parse(&v(&["--print-command", "host", "--viewer", r"C:\nv\nv-viewer.exe", "--uncap"])).unwrap();
        let t = print_command(&c, settings(), Path::new(r"C:\home"), Path::new(r"C:\nv"), [1; 8]).unwrap();
        assert!(t.contains("env: NV_UNCAP=1"));
        assert!(t.contains(r#"cmd: C:\nv\nv-viewer.exe D:\FNV\Data --new-game --coop-host 27961 --coop-token "#));
        assert!(t.contains("--coop-name Ann --coop-log"));
        assert!(!t.contains("--coop-test-token"));
        assert!(t.contains(r"cwd: C:\home\saves\0101010101010101\host-ann"));
    }

    #[test]
    fn dry_run_continue_and_join() {
        let c = parse(&v(&["--print-command", "host", "--continue", "00AB12CD34EF5678", "--viewer", "v.exe"])).unwrap();
        let t = print_command(&c, settings(), Path::new("h"), Path::new("."), [1; 8]).unwrap();
        assert!(t.contains("GSDocMitchellHouse --coop-host"));
        assert!(t.contains("00ab12cd34ef5678"));
        let code = joincode::host_code("192.168.1.20", 27961, [7; 16]).unwrap();
        let c = parse(&v(&["--print-command", "join", "--join", &code, "--viewer", "v.exe", "--name", "Bo"])).unwrap();
        let t = print_command(&c, settings(), Path::new("h"), Path::new("."), [2; 8]).unwrap();
        assert!(t.contains(&format!("--coop-join {code} --coop-name Bo")));
        assert!(!t.contains("--new-game"));
        assert!(t.contains("guest-bo"));
    }

    #[test]
    fn plain_address_join_is_refused_and_steam_codes_need_steam() {
        let c = parse(&v(&["--print-command", "join", "--join", "1.2.3.4:27961", "--viewer", "v.exe"])).unwrap();
        let e = print_command(&c, settings(), Path::new("h"), Path::new("."), [1; 8]).unwrap_err();
        assert!(e.contains("join code"), "{e}");
        let steam = coop_net::JoinCode::new(coop_net::JoinTarget::SteamLobby(109_775_241_000_000_001), [3; 16]).to_string();
        // dry run shows the Steam flags
        let c = parse(&v(&["--print-command", "join", "--join", &steam, "--viewer", "v.exe"])).unwrap();
        let t = print_command(&c, settings(), Path::new("h"), Path::new("."), [2; 8]).unwrap();
        assert!(t.contains("--coop-steam"), "{t}");
        // a real launch without Steam is refused with the reason
        let mut li = LaunchInput {
            settings: settings(),
            home: PathBuf::from("h"),
            viewer: PathBuf::from("v.exe"),
            host: false,
            new_game: false,
            join: Some(joincode::parse_join(&steam).unwrap()),
            campaign: None,
            random: [2; 8],
            steam: false,
            token: [0; 16],
            stamp: "x".into(),
        };
        assert!(make_plan(&li).unwrap_err().contains("Steam"));
        li.steam = true;
        assert!(make_plan(&li).is_ok());
        li.join = Some(JoinInput::SteamFriend);
        assert_eq!(make_plan(&li).unwrap().plan.mode, Mode::JoinFriend);
        li.steam = false;
        assert!(make_plan(&li).is_err());
    }

    #[test]
    fn dry_run_errors() {
        let c = parse(&v(&["--print-command", "join"])).unwrap();
        assert!(print_command(&c, settings(), Path::new("h"), Path::new("."), [1; 8]).is_err());
        let c = parse(&v(&["--print-command", "host"])).unwrap();
        assert!(print_command(&c, Settings::default(), Path::new("h"), Path::new("."), [1; 8]).unwrap_err().contains("Setup"));
        let c = parse(&v(&["--print-command", "host", "--continue", "xyz"])).unwrap();
        assert!(print_command(&c, settings(), Path::new("h"), Path::new("."), [1; 8]).is_err());
    }
}

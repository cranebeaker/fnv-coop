//! nv-coop: story co-op on the upstream engine.
//!
//! It starts one game through the glue, runs it to its main menu, and hosts a session or joins
//! one over UDP (coop-net's endpoint). Between two of the game's frames the session runs its
//! step (`coop-engine-session`): the host sends its story's changes, and a guest applies them to
//! its game with the story's scripts gated. A guest's game gets its gate at the main menu, before
//! its first frame of play.
//!
//! Both players start the same game (a new game, or the same save) from installations whose
//! plugins load in the same order: the handshake refuses a guest whose plugins differ.

use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, Instant};

use coop_core::ids::PlayerId;
use coop_core::sync::HostConfig;
use coop_engine::game::{resolve_policy, EngineState, NameTable};
use coop_engine_glue::{avatar_guard, gate, EngineProgram};
use coop_engine_session::ladder::{Cap, Ladder};
use coop_engine_session::{hello, GuestDriver, HostDriver};
use coop_net::{
    random_token, Endpoint, EndpointConfig, JoinCode, JoinTarget, NetEvent, UdpBackend, UdpConfig,
};

const USAGE: &str = "\
nv-coop - story co-op on the upstream engine

USAGE:
    nv-coop <DATA FOLDER> --names FILE host [PORT] [OPTIONS]
    nv-coop <DATA FOLDER> --names FILE join CODE [OPTIONS]

    DATA FOLDER  the game's Data folder (FalloutNV.exe 1.4.0.525 is read from the installation)
    --names FILE the names of the game's data: a line `P INDEX FILE` for each plugin and
                 `F TYPE FORM EDITOR_ID` for each form, in this load order
    host [PORT]  host the story on this UDP port (default 27961) and print a join code
    join CODE    follow the story of the host whose join code this is

OPTIONS:
    --name NAME  this player's name in the session (default player)
    --live CAP   how far the live ladder goes: log, warp, avatar (default), or off
    --test       a test run: stepped threads, the player's Fallout.ini options not read

Set NV_D3D9=real for a window. The guest presses New Game at its main menu; once it is in the
world, it is moved to the host.
";

/// The UDP port that a host uses unless told another.
const DEFAULT_PORT: u16 = 27961;
/// How long one frame of play is taken to be, for the gate's blind alarm.
const FRAME_MS: u32 = 16;
/// The frames that the start and the main menu may take.
const START_LIMIT: u32 = 20_000;
/// A status line this often.
const STATUS_EVERY: Duration = Duration::from_secs(5);

/// What the command line asks for.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Ask {
    /// Host on this UDP port.
    Host(u16),
    /// Join with this join code.
    Join(String),
}

#[derive(Debug)]
struct Args {
    data: PathBuf,
    names: PathBuf,
    ask: Ask,
    name: String,
    live: Option<Cap>,
    test: bool,
}

fn parse(raw: &[String]) -> Result<Args, String> {
    let mut data = None;
    let mut names = None;
    let mut ask = None;
    let mut name = "player".to_string();
    let mut live = Some(Cap::Avatar);
    let mut test = false;
    let mut i = 0;
    while i < raw.len() {
        let value =
            |i: usize, what: &str| raw.get(i).cloned().ok_or(format!("{what} needs a value"));
        match raw[i].as_str() {
            "--names" => {
                i += 1;
                names = Some(PathBuf::from(value(i, "--names")?));
            }
            "--name" => {
                i += 1;
                name = value(i, "--name")?;
            }
            "--live" => {
                i += 1;
                let v = value(i, "--live")?;
                live = match v.as_str() {
                    "off" => None,
                    "log" | "warp" | "avatar" => Some(Cap::parse(&v)),
                    other => return Err(format!("--live {other}: use log, warp, avatar or off")),
                };
            }
            "--test" => test = true,
            "host" => {
                let port = match raw.get(i + 1).map(|p| p.parse::<u16>()) {
                    Some(Ok(port)) => {
                        i += 1;
                        port
                    }
                    _ => DEFAULT_PORT,
                };
                if port == 0 {
                    return Err("the port must be a number from 1 to 65535".into());
                }
                ask = Some(Ask::Host(port));
            }
            "join" => {
                i += 1;
                let code = value(i, "join")?;
                code.parse::<JoinCode>()
                    .map_err(|e| format!("{code}: not a join code ({e:?})"))?;
                ask = Some(Ask::Join(code));
            }
            other if other.starts_with("--") => return Err(format!("unknown option {other}")),
            other if data.is_none() => data = Some(PathBuf::from(other)),
            other => return Err(format!("unexpected {other}")),
        }
        i += 1;
    }
    Ok(Args {
        data: data.ok_or("the Data folder is missing")?,
        names: names.ok_or("--names FILE is missing")?,
        ask: ask.ok_or("host or join is missing")?,
        name,
        live,
        test,
    })
}

/// This machine's address on its local network, for the join code (a guest on the internet
/// needs the host's public address and the port forwarded).
fn local_ip() -> IpAddr {
    UdpSocket::bind("0.0.0.0:0")
        .and_then(|s| {
            s.connect("192.0.2.1:9")?;
            s.local_addr()
        })
        .map(|a| a.ip())
        .ok()
        .filter(|ip| !ip.is_unspecified())
        .unwrap_or(IpAddr::V4(Ipv4Addr::LOCALHOST))
}

fn say(line: &str) {
    use std::io::Write as _;
    println!("{line}");
    let _ = std::io::stdout().flush();
}

enum Role {
    Host(Box<HostDriver>),
    /// A guest: gated from the main menu, stepping once the host accepted it.
    Guest {
        driver: Box<GuestDriver>,
        joined: bool,
    },
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
    if let Err(e) = run(&args) {
        say(&format!("co-op: {e}"));
        std::process::exit(1);
    }
}

fn run(args: &Args) -> Result<(), String> {
    let text =
        std::fs::read_to_string(&args.names).map_err(|e| format!("reading the name table: {e}"))?;
    let names = NameTable::parse(&text).map_err(|e| format!("the name table: {e}"))?;
    let bundled = coop_policy::PolicyTable::bundled().map_err(|e| format!("the policy: {e:?}"))?;
    let policy = Some(Arc::new(resolve_policy(&bundled, &names)?));
    let plugins = names.plugin_files();

    let at_end = |status: i32| {
        say(&format!(
            "co-op: the game's main ended with status {status}"
        ));
        std::process::exit(status);
    };
    let mut game = coop_engine_glue::start(&args.data, args.test, at_end)?;
    let frames = coop_engine_glue::reach_menu(&mut game, START_LIMIT)?;
    say(&format!("co-op: the main menu after {frames} frames"));

    let cfg = EndpointConfig {
        name: args.name.clone(),
        ..EndpointConfig::default()
    };
    let (mut ep, mut role) = match &args.ask {
        Ask::Host(port) => {
            let token = random_token();
            let bind = SocketAddr::from(([0, 0, 0, 0], *port));
            let b = UdpBackend::host(bind, token, UdpConfig::default())
                .map_err(|e| format!("UDP port {port}: {e}"))?;
            let code = JoinCode::new(
                JoinTarget::Direct(SocketAddr::new(local_ip(), *port)),
                token,
            );
            say(&format!(
                "co-op: hosting on UDP port {port}; join code for this network: {code} (over the internet: forward UDP port {port} and give your public address)"
            ));
            let session = u64::from_le_bytes(token[..8].try_into().expect("8 bytes"));
            let mut host = HostDriver::new(
                HostConfig::new(hello(plugins.clone()), session),
                EngineState::new(names),
                policy,
            );
            // A real game's export reads every quest: about ten times a second is enough.
            host.export_every = 6;
            host.life_every = 30;
            host.ladder = args.live.map(|cap| Ladder::new(false, cap));
            (Endpoint::host(b, token, cfg), Role::Host(Box::new(host)))
        }
        Ask::Join(code) => {
            let code: JoinCode = code.parse().map_err(|e| format!("join code: {e:?}"))?;
            let JoinTarget::Direct(addr) = code.target else {
                return Err("this build joins UDP codes only (not a Steam lobby)".into());
            };
            let b = UdpBackend::guest(addr, code.token, UdpConfig::default())
                .map_err(|e| format!("{addr}: {e}"))?;
            say(&format!("co-op: joining {addr}"));
            // The slot comes with the host's answer; the gate goes on the game now, before
            // its first frame of play.
            let mut guest = GuestDriver::new(
                PlayerId(1),
                hello(plugins.clone()),
                EngineState::new(names),
                policy,
            );
            guest.life_every = 30;
            guest.ladder = args.live.map(|cap| Ladder::new(true, cap));
            gate::own(|| guest.gate(&mut EngineProgram(&mut game.engine)))?;
            say("co-op: this game's story scripts are gated (a guest's game)");
            (
                Endpoint::guest(b, code.token, cfg),
                Role::Guest {
                    driver: Box::new(guest),
                    joined: false,
                },
            )
        }
    };
    if args.live == Some(Cap::Avatar) {
        avatar_guard::install(&mut game.engine)?;
    }
    say(&format!(
        "co-op: live ladder {}",
        args.live
            .map_or("off".to_string(), |c| format!("up to {c:?}"))
    ));

    let started = Instant::now();
    let mut last_status = Instant::now();
    let mut blind_said = false;
    loop {
        if !coop_engine_glue::frame(&mut game) {
            return Err("the game's main does not run".into());
        }
        let playing = coop_engine_glue::screen(&game) == coop_engine_glue::Screen::Play;
        if playing && gate::frame(FRAME_MS) && !blind_said {
            blind_said = true;
            say("co-op: gate blind: no script reached the gate in five seconds of play");
        }
        let now_ms = started.elapsed().as_millis() as u64;
        ep.pump();
        for ev in ep.events() {
            match ev {
                NetEvent::Connected { local } => {
                    say(&format!("co-op: joined as player {}", local.0));
                    if let Role::Guest { driver, joined } = &mut role {
                        if *joined {
                            // Joined again after losing the host: resume from the replica.
                            driver.rejoin(local, &mut ep);
                        } else {
                            driver.sync.player = local;
                            *joined = true;
                        }
                    }
                }
                NetEvent::PeerJoined { player, name } => {
                    say(&format!("co-op: {name} joined (player {})", player.0));
                }
                NetEvent::PeerLeft { player, reason } => {
                    say(&format!("co-op: player {} left ({reason:?})", player.0));
                    match &mut role {
                        Role::Host(h) => h.sync.disconnect(player),
                        Role::Guest { .. } => {
                            say("co-op: lost the host; joining again");
                            ep.rejoin();
                        }
                    }
                }
                other => say(&format!("co-op: {other:?}")),
            }
        }
        // The session's step is the adapter's own work: what the program runs for it is not
        // gated, and its applies hold the scripts back by themselves.
        let stepped = match &mut role {
            Role::Host(h) => {
                gate::own(|| h.step(&mut EngineProgram(&mut game.engine), &mut ep, now_ms))
            }
            Role::Guest {
                driver,
                joined: true,
            } => gate::own(|| driver.step(&mut EngineProgram(&mut game.engine), &mut ep, now_ms)),
            Role::Guest { .. } => Ok(()),
        };
        stepped?;
        // An avatar that left for a save or a load is not this game's any more.
        let ladder = match &mut role {
            Role::Host(h) => h.ladder.as_mut(),
            Role::Guest { driver, .. } => driver.ladder.as_mut(),
        };
        if let Some(ladder) = ladder {
            for (slot, why) in avatar_guard::take_left() {
                ladder.avatar_left(slot, why);
            }
            avatar_guard::set_bodies(ladder.bodies());
        }
        if last_status.elapsed() >= STATUS_EVERY {
            last_status = Instant::now();
            for line in status(&mut role) {
                say(&line);
            }
        }
    }
}

/// The status lines: the story's counters, then what to name once.
fn status(role: &mut Role) -> Vec<String> {
    let mut lines = Vec::new();
    match role {
        Role::Host(h) => {
            lines.push(format!(
                "co-op host: {} guests, store rev {}, claims enacted {} dropped {}, not sendable {}, bad packets {}, failed sends {}",
                h.sync.players().len(),
                h.sync.store.rev().0,
                h.claims_enacted,
                h.claims_dropped,
                h.unencodable,
                h.bad_packets,
                h.send_failures
            ));
            lines.extend(std::mem::take(&mut h.unencodable_notes));
            lines.extend(std::mem::take(&mut h.claim_notes));
            if let Some(l) = &h.ladder {
                lines.push(l.status());
            }
        }
        Role::Guest { driver: g, .. } => {
            lines.push(format!(
                "co-op guest: {:?}, hash checks {}, desyncs {}, healed {}, relisted {}, refused {}, claims sent {}, bad packets {}, failed sends {}",
                g.sync.phase(),
                g.sync.hash_checks,
                g.sync.desyncs_detected,
                g.healed,
                g.relisted,
                g.refused,
                g.claims_sent,
                g.bad_packets,
                g.send_failures
            ));
            lines.extend(std::mem::take(&mut g.healed_notes));
            if let Some(l) = &g.ladder {
                lines.push(l.status());
            }
        }
    }
    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    fn strings(a: &[&str]) -> Vec<String> {
        a.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn the_command_line_asks_to_host_or_join() {
        let a = parse(&strings(&["D:/Data", "--names", "n.txt", "host"])).unwrap();
        assert_eq!(a.ask, Ask::Host(DEFAULT_PORT));
        assert_eq!(a.live, Some(Cap::Avatar));
        let a = parse(&strings(&[
            "D:/Data", "host", "4000", "--names", "n.txt", "--live", "off",
        ]))
        .unwrap();
        assert_eq!((a.ask, a.live), (Ask::Host(4000), None));
        let code = JoinCode::new(
            JoinTarget::Direct(SocketAddr::from(([127, 0, 0, 1], 4000))),
            [3; 16],
        )
        .to_string();
        let a = parse(&strings(&["D:/Data", "--names", "n.txt", "join", &code])).unwrap();
        assert_eq!(a.ask, Ask::Join(code));
        assert!(parse(&strings(&["D:/Data", "--names", "n.txt"])).is_err());
        assert!(parse(&strings(&["D:/Data", "host"])).is_err());
        assert!(parse(&strings(&[
            "D:/Data", "--names", "n.txt", "join", "nonsense"
        ]))
        .is_err());
        assert!(parse(&strings(&[
            "D:/Data", "--names", "n.txt", "host", "--live", "x"
        ]))
        .is_err());
    }
}

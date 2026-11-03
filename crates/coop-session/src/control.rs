//! The live control port: a JSON-lines TCP server (127.0.0.1 only, `std::net`
//! and plain threads) that lets a script drive a running viewer.
//!
//! One request per line, one reply per line, in order, per connection:
//!
//! ```text
//! {"cmd":"state"}
//! {"cmd":"run","line":"SetStage MyQuest 10"}            host: runs it; guest: sends it to the host
//! {"cmd":"run","line":"player.AddItem Caps001 5","local":true}   runs on this machine only
//! {"cmd":"bot","line":"coop teleport 100 200 0"}        the test bot ("coop " is optional)
//! {"cmd":"keys","key":"w","hold":2.0,"delay":0.0}       KEY as --key-at (`w`, `space`, `mouse-left`, ...)
//! {"cmd":"screenshot","path":"C:/x/shot.png"}           replies once the file is written
//! {"cmd":"entities"}
//! {"cmd":"quit"}
//! ```
//!
//! An optional `"id"` is echoed in the reply. Every reply is one object with
//! `"ok":true|false` and `"cmd"`; a failure has `"error"`. The server only
//! parses and forwards: the viewer answers each [`Request`] on its frame.

use std::io::{BufRead, BufReader, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::mpsc::{self, Receiver, Sender};
use std::time::Duration;

use crate::json::{self, esc};

/// How long a connection waits for the viewer to answer a request.
pub const REPLY_WAIT: Duration = Duration::from_secs(30);

/// A command from a client.
#[derive(Clone, Debug, PartialEq)]
pub enum Cmd {
    /// The full state dump plus live figures.
    State,
    /// A game script line; `local` keeps it on this machine.
    Run {
        /// The script line.
        line: String,
        /// Run here even on a guest.
        local: bool,
    },
    /// A test bot line, always with the `coop ` prefix.
    Bot {
        /// The line.
        line: String,
    },
    /// Press a key (as `--key-at`).
    Keys {
        /// `KEY`, as `--key-at` takes it.
        key: String,
        /// Seconds held.
        hold: f32,
        /// Seconds from now.
        delay: f32,
    },
    /// Save a PNG of the current frame.
    Screenshot {
        /// Where.
        path: String,
    },
    /// List the people and puppets.
    Entities,
    /// Leave cleanly.
    Quit,
}

impl Cmd {
    /// The `"cmd"` word.
    pub fn name(&self) -> &'static str {
        match self {
            Cmd::State => "state",
            Cmd::Run { .. } => "run",
            Cmd::Bot { .. } => "bot",
            Cmd::Keys { .. } => "keys",
            Cmd::Screenshot { .. } => "screenshot",
            Cmd::Entities => "entities",
            Cmd::Quit => "quit",
        }
    }
}

/// A request for the viewer, with the way back.
pub struct Request {
    /// What to do.
    pub cmd: Cmd,
    /// Send exactly one reply line (from [`ok`] or [`fail`]) here.
    pub reply: Sender<String>,
}

/// The request's `"id"` as JSON text (a number stays one), if it has one.
pub fn id_of(line: &str) -> Option<String> {
    let kv = json::parse_flat(line).ok()?;
    let (_, v) = kv.into_iter().find(|(k, _)| k == "id")?;
    Some(if v.parse::<f64>().is_ok() {
        v
    } else {
        format!("\"{}\"", esc(&v))
    })
}

/// Parses a request line into its command and the raw `"id"` (as JSON text).
pub fn parse(line: &str) -> Result<(Cmd, Option<String>), String> {
    let kv = json::parse_flat(line)?;
    let get = |k: &str| kv.iter().find(|(a, _)| a == k).map(|(_, v)| v.as_str());
    let need = |k: &str| {
        get(k)
            .filter(|v| !v.is_empty())
            .ok_or_else(|| format!("missing \"{k}\""))
    };
    let num = |k: &str, default: f32| -> Result<f32, String> {
        match get(k) {
            None => Ok(default),
            Some(v) => v
                .parse::<f32>()
                .ok()
                .filter(|n| n.is_finite() && *n >= 0.0)
                .ok_or_else(|| format!("\"{k}\" must be a number of seconds, got {v}")),
        }
    };
    let id = id_of(line);
    let cmd = match need("cmd")? {
        "state" => Cmd::State,
        "entities" => Cmd::Entities,
        "quit" => Cmd::Quit,
        "run" => Cmd::Run {
            line: need("line")?.trim().to_string(),
            local: get("local") == Some("true"),
        },
        "bot" => {
            let l = need("line")?.trim();
            let bare = l
                .strip_prefix("coop ")
                .or_else(|| l.strip_prefix("COOP "))
                .unwrap_or(l)
                .trim();
            if bare.is_empty() {
                return Err("empty bot line".into());
            }
            Cmd::Bot {
                line: format!("coop {bare}"),
            }
        }
        "keys" => Cmd::Keys {
            key: need("key")?.to_string(),
            hold: num("hold", 0.1)?,
            delay: num("delay", 0.0)?,
        },
        "screenshot" => Cmd::Screenshot {
            path: need("path")?.to_string(),
        },
        other => return Err(format!("unknown cmd \"{other}\"")),
    };
    Ok((cmd, id))
}

/// A success reply: `fields` is the inside of the object after `ok` and `cmd`
/// (already JSON, e.g. `"how":"queued"`), or empty.
pub fn ok(cmd: &str, fields: &str) -> String {
    if fields.is_empty() {
        format!("{{\"ok\":true,\"cmd\":\"{}\"}}", esc(cmd))
    } else {
        format!("{{\"ok\":true,\"cmd\":\"{}\",{fields}}}", esc(cmd))
    }
}

/// A failure reply.
pub fn fail(cmd: &str, msg: &str) -> String {
    format!(
        "{{\"ok\":false,\"cmd\":\"{}\",\"error\":\"{}\"}}",
        esc(cmd),
        esc(msg)
    )
}

/// Puts `"id":ID` first in a reply object.
pub fn with_id(reply: &str, id: Option<&str>) -> String {
    match (id, reply.strip_prefix('{')) {
        (Some(id), Some(rest)) => format!("{{\"id\":{id},{rest}"),
        _ => reply.to_string(),
    }
}

/// A person or puppet in the world, for `entities`.
#[derive(Clone, Debug, PartialEq)]
pub struct EntityRow {
    /// The reference's form id.
    pub form: u32,
    /// Its editor id, or the form id in hex.
    pub name: String,
    /// Position (game units).
    pub pos: [f32; 3],
    /// Heading in radians clockwise from north.
    pub heading: f32,
    /// A mesh of it is in a camera's view this frame.
    pub visible: bool,
    /// The published animation bits ([`anim_names`]).
    pub anim: u16,
    /// `puppet` (a remote player's avatar), `remote` (another peer owns
    /// this person) or `npc`.
    pub kind: &'static str,
    /// The player slot for a puppet.
    pub player: Option<u8>,
}

/// Names of the animation bits (1 moving, 2 running, 4 sneaking, 8 fighting);
/// `idle` for none.
pub fn anim_names(anim: u16) -> String {
    let mut v = Vec::new();
    for (bit, name) in [
        (1, "moving"),
        (2, "running"),
        (4, "sneaking"),
        (8, "fighting"),
    ] {
        if anim & bit != 0 {
            v.push(name);
        }
    }
    if v.is_empty() {
        "idle".into()
    } else {
        v.join("+")
    }
}

/// Counts of what is in the world.
pub fn counts(rows: &[EntityRow]) -> (usize, usize, usize, usize) {
    let p = rows.iter().filter(|r| r.kind == "puppet");
    let n = rows.iter().filter(|r| r.kind != "puppet");
    (
        n.clone().count(),
        n.filter(|r| r.visible).count(),
        p.clone().count(),
        p.filter(|r| r.visible).count(),
    )
}

/// The `entities` reply.
pub fn entities_reply(rows: &[EntityRow]) -> String {
    let items: Vec<String> = rows
        .iter()
        .map(|r| {
            let player = r.player.map_or("null".to_string(), |p| p.to_string());
            format!(
                "{{\"form\":\"{:08X}\",\"name\":\"{}\",\"kind\":\"{}\",\"player\":{player},\"pos\":[{:.1},{:.1},{:.1}],\"heading\":{:.3},\"visible\":{},\"anim\":{},\"anim_names\":\"{}\"}}",
                r.form, esc(&r.name), r.kind, r.pos[0], r.pos[1], r.pos[2], r.heading, r.visible, r.anim, anim_names(r.anim)
            )
        })
        .collect();
    let (npcs, npcs_visible, puppets, puppets_visible) = counts(rows);
    ok(
        "entities",
        &format!(
            "\"count\":{},\"npcs\":{npcs},\"npcs_visible\":{npcs_visible},\"puppets\":{puppets},\"puppets_visible\":{puppets_visible},\"entities\":[{}]",
            rows.len(),
            items.join(",")
        ),
    )
}

/// Live figures next to the state dump.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LiveStats {
    /// The player's cell (editor id or hex).
    pub cell: String,
    /// Frames a second (smoothed).
    pub fps: f32,
    /// Seconds since the app started.
    pub t: f32,
    /// This machine is host (else guest).
    pub host: bool,
    /// The place has loaded and the player stands in it.
    pub ready: bool,
    /// People and puppets in the world, and how many are in view.
    pub rows: Vec<EntityRow>,
    /// Dialogue lines played, chat received, travels, hits sent.
    pub counters: [u64; 4],
}

/// The `state` reply: the dump verbatim under `"state"`, live figures beside it.
pub fn state_reply(dump: &str, s: &LiveStats) -> String {
    let (npcs, npcs_visible, puppets, puppets_visible) = counts(&s.rows);
    ok(
        "state",
        &format!(
            "\"role\":\"{}\",\"ready\":{},\"cell\":\"{}\",\"fps\":{:.1},\"t\":{:.2},\"npcs\":{npcs},\"npcs_visible\":{npcs_visible},\"puppets\":{puppets},\"puppets_visible\":{puppets_visible},\"counters\":{{\"lines_played\":{},\"chat_in\":{},\"travels\":{},\"hits_sent\":{}}},\"state\":{}",
            if s.host { "host" } else { "guest" },
            s.ready,
            esc(&s.cell),
            s.fps,
            s.t,
            s.counters[0],
            s.counters[1],
            s.counters[2],
            s.counters[3],
            dump.trim()
        ),
    )
}

/// Starts the server on 127.0.0.1:`port` (0: any free port). Requests come
/// out of the receiver; the address says where it listens.
pub fn serve(port: u16) -> Result<(Receiver<Request>, SocketAddr), String> {
    let listener =
        TcpListener::bind(("127.0.0.1", port)).map_err(|e| format!("control port {port}: {e}"))?;
    let addr = listener.local_addr().map_err(|e| e.to_string())?;
    let (tx, rx) = mpsc::channel::<Request>();
    std::thread::Builder::new()
        .name("coop-control".into())
        .spawn(move || {
            for conn in listener.incoming().flatten() {
                let tx = tx.clone();
                let _ = std::thread::Builder::new()
                    .name("coop-control-conn".into())
                    .spawn(move || connection(conn, tx));
            }
        })
        .map_err(|e| e.to_string())?;
    Ok((rx, addr))
}

fn connection(conn: TcpStream, tx: Sender<Request>) {
    let _ = conn.set_nodelay(true);
    let Ok(mut out) = conn.try_clone() else {
        return;
    };
    for line in BufReader::new(conn).lines() {
        let Ok(line) = line else { return };
        if line.trim().is_empty() {
            continue;
        }
        let reply = match parse(&line) {
            Err(e) => with_id(&fail("error", &e), id_of(&line).as_deref()),
            Ok((cmd, id)) => {
                let name = cmd.name();
                let (rtx, rrx) = mpsc::channel();
                let r = if tx.send(Request { cmd, reply: rtx }).is_err() {
                    fail(name, "the viewer is gone")
                } else {
                    rrx.recv_timeout(REPLY_WAIT)
                        .unwrap_or_else(|_| fail(name, "no answer from the viewer"))
                };
                with_id(&r, id.as_deref())
            }
        };
        if writeln!(out, "{reply}").is_err() {
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_every_command() {
        assert_eq!(parse(r#"{"cmd":"state"}"#).unwrap().0, Cmd::State);
        let (c, id) = parse(r#"{"cmd":"run","line":"SetStage Q 10","local":true,"id":7}"#).unwrap();
        assert_eq!(
            c,
            Cmd::Run {
                line: "SetStage Q 10".into(),
                local: true
            }
        );
        assert_eq!(id.as_deref(), Some("7"));
        assert_eq!(
            parse(r#"{"cmd":"bot","line":"teleport 1 2 3"}"#).unwrap().0,
            Cmd::Bot {
                line: "coop teleport 1 2 3".into()
            }
        );
        assert_eq!(
            parse(r#"{"cmd":"bot","line":"coop say hi"}"#).unwrap().0,
            Cmd::Bot {
                line: "coop say hi".into()
            }
        );
        assert_eq!(
            parse(r#"{"cmd":"keys","key":"w","hold":2}"#).unwrap().0,
            Cmd::Keys {
                key: "w".into(),
                hold: 2.0,
                delay: 0.0
            }
        );
        assert_eq!(
            parse(r#"{"cmd":"screenshot","path":"a.png"}"#).unwrap().0,
            Cmd::Screenshot {
                path: "a.png".into()
            }
        );
        assert!(parse(r#"{"cmd":"run"}"#).is_err());
        assert!(parse(r#"{"cmd":"nope"}"#).unwrap_err().contains("unknown"));
        assert!(parse(r#"{"cmd":"keys","key":"w","hold":-1}"#).is_err());
        assert!(parse("garbage").is_err());
    }

    #[test]
    fn replies_are_exact() {
        assert_eq!(ok("quit", ""), r#"{"ok":true,"cmd":"quit"}"#);
        assert_eq!(
            fail("run", "a \"b\""),
            r#"{"ok":false,"cmd":"run","error":"a \"b\""}"#
        );
        assert_eq!(
            with_id(&ok("quit", ""), Some("\"x\"")),
            r#"{"id":"x","ok":true,"cmd":"quit"}"#
        );
        let rows = vec![
            EntityRow {
                form: 0x104C0,
                name: "Doc".into(),
                pos: [1.0, 2.0, 3.0],
                heading: 0.5,
                visible: true,
                anim: 3,
                kind: "npc",
                player: None,
            },
            EntityRow {
                form: 0xFF000001,
                name: "Guest".into(),
                pos: [0.0; 3],
                heading: 0.0,
                visible: false,
                anim: 0,
                kind: "puppet",
                player: Some(1),
            },
        ];
        assert_eq!(counts(&rows), (1, 1, 1, 0));
        let e = entities_reply(&rows);
        assert!(
            e.contains("\"anim_names\":\"moving+running\"")
                && e.contains("\"player\":1")
                && e.contains("\"npcs_visible\":1"),
            "{e}"
        );
        let s = state_reply(
            "{\"seq\":1}\n",
            &LiveStats {
                cell: "DocHouse".into(),
                host: true,
                rows,
                ..Default::default()
            },
        );
        assert!(
            s.starts_with("{\"ok\":true,\"cmd\":\"state\",\"role\":\"host\"")
                && s.ends_with(",\"state\":{\"seq\":1}}"),
            "{s}"
        );
        // the replies are the flat shape parse_flat cannot read, so just check balance
        assert_eq!(s.matches('{').count(), s.matches('}').count());
    }

    #[test]
    fn serves_requests_over_loopback_in_order() {
        let (rx, addr) = serve(0).unwrap();
        assert!(addr.ip().is_loopback());
        let answer = std::thread::spawn(move || {
            for _ in 0..2 {
                let r = rx.recv().unwrap();
                let name = r.cmd.name();
                r.reply.send(ok(name, "\"n\":1")).unwrap();
            }
        });
        let mut c = TcpStream::connect(addr).unwrap();
        c.set_read_timeout(Some(Duration::from_secs(5))).unwrap();
        writeln!(
            c,
            "{{\"cmd\":\"state\",\"id\":\"a\"}}\nnot json\n{{\"cmd\":\"quit\"}}"
        )
        .unwrap();
        let mut r = BufReader::new(c);
        let mut line = String::new();
        r.read_line(&mut line).unwrap();
        assert_eq!(line.trim(), r#"{"id":"a","ok":true,"cmd":"state","n":1}"#);
        line.clear();
        r.read_line(&mut line).unwrap();
        assert!(
            line.starts_with("{\"ok\":false,\"cmd\":\"error\""),
            "{line}"
        );
        line.clear();
        r.read_line(&mut line).unwrap();
        assert_eq!(line.trim(), r#"{"ok":true,"cmd":"quit","n":1}"#);
        answer.join().unwrap();
    }
}

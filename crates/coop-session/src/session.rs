//! The [`Session`] driver.

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use coop_core::ids::{PlayerId, Tick};
use coop_core::proto::{self, Channel, Hello, Msg, RejectReason, PROTOCOL_VERSION};
use coop_core::sync::{GuestPhase, GuestSync, HostConfig, HostSync};
use coop_core::transport::{ChaosProfile, Transport};
use world::coop::apply::Opts;
use world::coop::{Authority, GuestBridge, HostBridge, WorldBackend};
use world::scripting::GameState;

use crate::ext::{Command, Ext, PeerInfo, Place, MARK};
use crate::json::V;
use crate::net::{self, LinkErrors, Net};
use crate::out::{DumpFile, LogFile};
use crate::target::{parse_host_port, JoinTarget};
use crate::{Error, Role};

/// Milliseconds per protocol tick (20 Hz).
pub const TICK_MS: f64 = 50.0;
/// Ticks between hash broadcasts.
const HASH_EVERY_TICKS: u64 = 10;
/// Ticks between guest pings.
const PING_EVERY_TICKS: u64 = 20;

/// A guest's ping in whole milliseconds, at least 1. The clock counts whole
/// milliseconds, so a pong on a local link can come back in the same one.
/// The status line uses 0 for "no ping yet" (`rtt_ms.unwrap_or(0)`), so a
/// measured ping must not read as 0.
fn ping_ms(sent_ms: u64, now_ms: u64) -> u32 {
    (now_ms.saturating_sub(sent_ms) as u32).max(1)
}

#[cfg(test)]
mod ping_tests {
    #[test]
    fn a_ping_in_the_same_millisecond_reads_as_1() {
        assert_eq!(super::ping_ms(5000, 5000), 1);
        assert_eq!(super::ping_ms(5000, 5017), 17);
        assert_eq!(super::ping_ms(5000, 4990), 1, "a clock step back");
    }
}
/// Hash of the replication rules this build speaks (bump with the schema).
pub const RULES_HASH: u64 = 0x6E76_7273_636F_6F70 ^ PROTOCOL_VERSION as u64;
/// Silence from the host's state stream before a guest reconnects.
const STALE_MS: u64 = 4000;
/// Wait between handshake attempts.
const RETRY_MS: u64 = 4000;
/// Wait before an unacknowledged command is sent again.
const CMD_RESEND_MS: u64 = 2000;
/// Interval of the state dump / sync log line.
const OUT_EVERY_MS: u64 = 1000;
/// Interval of the guest's drift re-check.
const RECHECK_EVERY_MS: u64 = 2000;

/// Everything needed to start a session.
pub struct SessionConfig {
    /// Display name.
    pub name: String,
    /// Host: the port to listen on (`PORT`, `:PORT`, `ADDR:PORT`). Guest:
    /// `HOST:PORT` or a join code. Ignored when `transport` is given.
    pub addr: String,
    /// Build identity quoted in the handshake (peers must match).
    pub build_hash: u64,
    /// Hash of the ordered plugin list (peers must match).
    pub mods_hash: u64,
    /// World lineage id (0: made up from the clock).
    pub session_id: u64,
    /// Most guests (host).
    pub max_guests: usize,
    /// Chaos profile and seed for the outgoing link (testing).
    pub chaos: Option<(ChaosProfile, u64)>,
    /// JSONL event log.
    pub log: Option<PathBuf>,
    /// Debug dump of every shared field (header line, then one line per
    /// field; two peers' dumps diff line by line). Rewritten about once a
    /// second when state changed. Not the harness schema: see [`crate::dump`].
    pub fields_dump: Option<PathBuf>,
    /// A ready transport (tests; overrides `addr`).
    pub transport: Option<Box<dyn Transport>>,
    /// Host: silence after which a guest is dropped.
    pub peer_timeout_ms: u64,
    /// Run the live layer (avatars, actors, loot, dialogue, rules, policy
    /// and per-player shadows). Off for the plain state-sync tests.
    pub live: bool,
    /// The campaign policy table (`None`: the bundled one).
    pub policy: Option<std::sync::Arc<coop_policy::PolicyTable>>,
    /// With `live`: no policy gate and no shadows (one shared player).
    pub no_policy: bool,
    /// With `live`: the host's game runs its own player's dialogue lines
    /// itself (the viewer); the session must not run their scripts again.
    pub native_dialogue: bool,
    /// Executable version, volatile-key hash, plugin list and the
    /// `--coop-allow-mismatch` override (see `coop_core::compat`).
    pub compat: CompatConfig,
    /// Host: the campaign rules the launcher or viewer flags chose
    /// (`--coop-difficulty`, `--coop-friendly-fire`). `None`: a loaded
    /// party save's rules, else Co-op balanced with friendly fire off.
    pub rules: Option<world::coop::party_save::PartyRules>,
}

/// What a peer tells the handshake about its game, plus the host policy.
#[derive(Clone, Debug, Default)]
pub struct CompatConfig {
    /// Game executable version text (empty: unknown).
    pub exe: String,
    /// Hash of the volatile-key set.
    pub volatile_hash: u64,
    /// Ordered plugin file names; when given and `mods_hash` is 0 the hash
    /// is made from them.
    pub plugins: Vec<String>,
    /// Developer override: let a mismatched guest in with a warning.
    pub allow_mismatch: bool,
}

impl Default for SessionConfig {
    fn default() -> Self {
        SessionConfig {
            name: "player".into(),
            addr: String::new(),
            build_hash: 0,
            mods_hash: 0,
            session_id: 0,
            max_guests: 3,
            chaos: None,
            log: None,
            fields_dump: None,
            transport: None,
            peer_timeout_ms: 8000,
            live: false,
            policy: None,
            no_policy: false,
            native_dialogue: false,
            compat: CompatConfig::default(),
            rules: None,
        }
    }
}

/// Where the connection stands.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum State {
    /// Host, running.
    Hosting,
    /// Guest, handshake not finished.
    Connecting,
    /// Guest, receiving the world.
    Joining,
    /// Guest, following the host.
    Connected,
    /// Guest, lost the host and trying again.
    Reconnecting,
    /// The host refused.
    Rejected(RejectReason),
}

/// Something that happened, for a message line or a log.
#[derive(Clone, Debug, PartialEq)]
pub enum Event {
    /// A guest joined (host side: by name; guest side: from the roster).
    PeerJoined(PlayerId, String),
    /// A guest left or timed out.
    PeerLeft(PlayerId, String),
    /// Guest: following the host for the first time.
    Connected,
    /// Guest: the host went quiet.
    Lost,
    /// Guest: back with the host.
    Reconnected,
    /// Guest: the host said no.
    Rejected(RejectReason),
    /// A handshake found differences: lines for the player (the launcher and
    /// the HUD show them). `refused` is false when the override let the guest in.
    Compat {
        /// Player-facing lines.
        lines: Vec<String>,
        /// The guest was turned away.
        refused: bool,
    },
    /// Guest: the state hash differed from the host's.
    Desync,
    /// Guest: a repair was applied.
    Repaired,
    /// Guest: the host moved somewhere else.
    PlaceChanged(Place),
    /// The session is usable: host, a first guest is in; guest, the join
    /// snapshot is applied (what `coop_ready` logs).
    Ready,
}

impl Event {
    /// A line for the player, for the events that deserve one.
    pub fn message(&self) -> Option<String> {
        Some(match self {
            Event::PeerJoined(_, n) => format!("{n} joined the co-op session"),
            Event::PeerLeft(_, n) => format!("{n} left the co-op session"),
            Event::Connected => "Connected to the host".into(),
            Event::Lost => "Lost the host, reconnecting".into(),
            Event::Reconnected => "Reconnected to the host".into(),
            Event::Rejected(r) => format!(
                "The host refused the connection: {}",
                coop_core::compat::reason_text(*r)
            ),
            Event::Compat { lines, .. } => lines.join(" "),
            Event::Desync => "Co-op state differed from the host, repairing".into(),
            Event::Repaired => "Co-op state repaired".into(),
            Event::PlaceChanged(_) | Event::Ready => return None,
        })
    }
}

/// How far back [`Session::hash_points`] gives a host's roots. A guest that
/// trails the host by more than this does not match an older root.
pub const RECENT_ROOTS_MS: u64 = 3_000;

/// The revision of this peer's shared-state hash, and on a host the roots of
/// the last revisions ([`Session::hash_points`]).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct HashPoints {
    /// The store revision that the game state holds (unknown on a guest
    /// whose store has a gap).
    pub rev: Option<u64>,
    /// Host only: `(revision, root)`, newest first.
    pub recent: Vec<(u64, u64)>,
}

/// A snapshot of the session for a status line.
#[derive(Clone, Debug)]
pub struct Status {
    /// Host or guest.
    pub role: Role,
    /// Connection state.
    pub state: State,
    /// This peer's slot.
    pub player: PlayerId,
    /// The other peers (a guest sees the host too).
    pub peers: Vec<PeerInfo>,
    /// Guest: round trip to the host.
    pub rtt_ms: Option<u32>,
    /// Revision of the replicated state held.
    pub rev: u64,
    /// A hash mismatch is open (not yet confirmed clean).
    pub desync: bool,
    /// Mismatches seen so far.
    pub desyncs: u32,
    /// Repairs applied so far.
    pub repairs: u32,
    /// Full snapshots loaded (guest) or sent (host).
    pub snapshots: u32,
    /// Guest commands not yet acknowledged.
    pub pending_commands: usize,
    /// Packets that did not decode.
    pub decode_errors: u64,
    /// Root hash of the shared state last computed.
    pub root: Option<u64>,
    /// Session clock.
    pub now_ms: u64,
}

impl Status {
    /// Players in the session, this one included (a host's `peers` list
    /// holds the host itself, a guest's does not).
    pub fn players(&self) -> usize {
        if self.role == Role::Host {
            self.peers.len()
        } else {
            self.peers.len() + 1
        }
    }

    /// The small indicator: players and ping, nothing else.
    pub fn compact(&self) -> String {
        let warn = if self.desync { " !" } else { "" };
        match (self.role, self.state) {
            (Role::Host, _) => {
                let worst = self
                    .peers
                    .iter()
                    .filter(|p| !p.player.is_host())
                    .map(|p| p.rtt_ms)
                    .max();
                let ping = worst.map_or(String::new(), |r| format!(" {r} ms"));
                format!("{}P{ping}{warn}", self.players())
            }
            (_, State::Connecting | State::Joining | State::Reconnecting) => "...".into(),
            (_, State::Rejected(_)) => "refused".into(),
            _ => {
                let ping = self.rtt_ms.map_or("-".to_string(), |r| format!("{r} ms"));
                format!("{}P {ping}{warn}", self.players())
            }
        }
    }

    /// One line for the HUD.
    pub fn line(&self) -> String {
        let desync = if self.desync { " | DESYNC" } else { "" };
        // the desync dashboard's repair count, once there is one
        let warn = match self.repairs {
            0 => desync.to_string(),
            1 => format!(" | 1 repair{desync}"),
            n => format!(" | {n} repairs{desync}"),
        };
        match (self.role, self.state) {
            (Role::Host, _) => {
                let n = self.peers.iter().filter(|p| !p.player.is_host()).count();
                let who: Vec<String> = self
                    .peers
                    .iter()
                    .filter(|p| !p.player.is_host())
                    .map(|p| format!("{} {} ms", p.name, p.rtt_ms))
                    .collect();
                let tail = if who.is_empty() {
                    String::new()
                } else {
                    format!(" | {}", who.join(", "))
                };
                format!(
                    "CO-OP host | {n} guest{}{tail}{warn}",
                    if n == 1 { "" } else { "s" }
                )
            }
            (_, State::Connecting) => "CO-OP guest | connecting".into(),
            (_, State::Joining) => "CO-OP guest | loading the host's world".into(),
            (_, State::Reconnecting) => "CO-OP guest | host lost, reconnecting".into(),
            (_, State::Rejected(r)) => format!(
                "CO-OP guest | refused: {}",
                coop_core::compat::reason_text(r)
            ),
            _ => {
                let ping = self.rtt_ms.map_or("-".to_string(), |r| format!("{r} ms"));
                format!(
                    "CO-OP guest | ping {ping} | {} in session{warn}",
                    self.players()
                )
            }
        }
    }
}

pub(crate) struct Peer {
    pub(crate) name: Option<String>,
    pub(crate) rtt_ms: u32,
    pub(crate) last_heard_ms: u64,
    pub(crate) seen_ms: u64,
    pub(crate) announced: bool,
    pub(crate) cmds_seen: BTreeSet<u32>,
    /// True once the peer sent an avatar sample with a cell: it plays. Before
    /// that its viewer may still load the cell and not poll (see
    /// [`LOAD_TIMEOUT_MS`]).
    pub(crate) in_game: bool,
}

/// How long the host waits for a live peer that has not played yet (no
/// avatar sample with a cell). A
/// viewer loads its first cell without polling the link, and the normal
/// timeout (`peer_timeout_ms`, 8 s) dropped a guest on a slow load. The
/// host then lost the messages it sent during the load.
pub(crate) const LOAD_TIMEOUT_MS: u64 = 45_000;

/// The host's place as guest `to` gets it: the feet on the guest's own spot
/// beside the host (`SceneModel::ring_spot`). A guest that starts there or
/// follows the host there then does not stand on the host.
fn place_for(pl: Place, to: PlayerId) -> Place {
    use coop_core::roam::{Placement, SceneModel};
    let pos = pl.pos.map(|p| {
        let at = Placement {
            cell: coop_core::ids::CellId(pl.cell),
            pos: p,
            heading: 0.0,
        };
        SceneModel::ring_spot(at, to).pos
    });
    Place { pos, ..pl }
}

pub(crate) struct HostSide {
    pub(crate) sync: HostSync,
    pub(crate) bridge: HostBridge,
    pub(crate) peers: BTreeMap<PlayerId, Peer>,
    pub(crate) commands: Vec<(PlayerId, Command)>,
    pub(crate) place: Option<Place>,
    pub(crate) roster_dirty: bool,
    pub(crate) last_roster_ms: u64,
}

pub(crate) struct GuestSide {
    pub(crate) sync: GuestSync,
    pub(crate) bridge: GuestBridge,
    pub(crate) place: Option<Place>,
    pub(crate) outbox: BTreeMap<u32, (Command, Option<u64>)>,
    pub(crate) next_cmd: u32,
    pub(crate) nonce: u32,
    pub(crate) ping: Option<(u32, u64)>,
    pub(crate) rtt_ms: Option<u32>,
    pub(crate) last_state_ms: u64,
    pub(crate) last_any_ms: u64,
    pub(crate) last_retry_ms: u64,
    pub(crate) last_info_ms: Option<u64>,
    pub(crate) last_recheck_ms: u64,
    pub(crate) ever_connected: bool,
    pub(crate) active: bool,
    pub(crate) lost: bool,
    pub(crate) desync_open: bool,
    pub(crate) seen_desyncs: u32,
    pub(crate) seen_repairs: u32,
    pub(crate) seen_checks: u32,
    pub(crate) seen_snaps: u32,
    pub(crate) reported_reject: bool,
    pub(crate) roster: Vec<PeerInfo>,
    pub(crate) drift_logged: u32,
}

/// A co-op peer: transport, protocol state machines and world bridges.
pub struct Session {
    pub(crate) role: Role,
    pub(crate) name: String,
    pub(crate) net: Net,
    pub(crate) host: Option<HostSide>,
    pub(crate) guest: Option<GuestSide>,
    pub(crate) clock_s: f64,
    pub(crate) now_ms: u64,
    pub(crate) tick_acc_ms: f64,
    pub(crate) tick_no: u64,
    pub(crate) events: Vec<Event>,
    pub(crate) log: Option<LogFile>,
    pub(crate) dump: Option<DumpFile>,
    pub(crate) last_out_ms: u64,
    pub(crate) last_out_key: (u64, u32, u64),
    pub(crate) root: Option<u64>,
    pub(crate) decode_errors: u64,
    pub(crate) peer_timeout_ms: u64,
    pub(crate) link_errors: LinkErrors,
    pub(crate) ready: bool,
    pub(crate) last_error: Option<String>,
    pub(crate) live: Option<crate::live::Layer>,
    pub(crate) start: crate::start::StartSide,
    pub(crate) party: Option<Box<crate::party::PartyLayer>>,
    /// Host: the flags chose the campaign rules (a loaded party save keeps them).
    pub(crate) rules_given: bool,
}

pub(crate) fn send_msg(net: &mut Net, to: PlayerId, m: &Msg) {
    if let Ok(bytes) = proto::encode(m) {
        net.send(to, m.channel(), bytes);
    }
}

pub(crate) fn send_ext(net: &mut Net, to: PlayerId, e: &Ext) {
    net.send(to, Channel::ReliableOrdered, e.encode());
}

pub(crate) fn made_up_id() -> u64 {
    let t = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(1);
    (t ^ ((std::process::id() as u64) << 32)).max(1)
}

impl Session {
    #[allow(clippy::type_complexity)]
    fn common(
        role: Role,
        cfg: &mut SessionConfig,
    ) -> Result<(Net, Option<LogFile>, Option<DumpFile>, Hello, LinkErrors), Error> {
        let (link, errors) = match cfg.transport.take() {
            Some(t) => (t, LinkErrors::default()),
            None => {
                match role {
                    Role::Host => parse_host_port(&cfg.addr)
                        .map(|_| ())
                        .map_err(Error::Target)?,
                    Role::Guest => JoinTarget::parse(&cfg.addr)
                        .map(|_| ())
                        .map_err(Error::Target)?,
                }
                net::open(role, &cfg.name, &cfg.addr)?
            }
        };
        let log = match &cfg.log {
            Some(p) => {
                Some(LogFile::create(p).map_err(|e| Error::Io(format!("{}: {e}", p.display())))?)
            }
            None => None,
        };
        let dump = cfg.fields_dump.clone().map(DumpFile::new);
        // a hello with a name too long or too many names would be refused as malformed: cut to the wire bounds
        let listed: Vec<String> = cfg
            .compat
            .plugins
            .iter()
            .take(coop_core::compat::MAX_PLUGINS)
            .map(|p| p.chars().take(coop_core::compat::MAX_PLUGIN_NAME).collect())
            .collect();
        let hello = Hello {
            proto: PROTOCOL_VERSION,
            build_hash: cfg.build_hash,
            rules_hash: RULES_HASH,
            mods_hash: if cfg.mods_hash == 0 && !listed.is_empty() {
                coop_core::compat::plugin_list_hash(&listed)
            } else {
                cfg.mods_hash
            },
            exe: cfg
                .compat
                .exe
                .chars()
                .take(coop_core::compat::MAX_EXE_LEN)
                .collect(),
            volatile_hash: cfg.compat.volatile_hash,
            plugins: listed,
        };
        Ok((Net::new(link, cfg.chaos.take()), log, dump, hello, errors))
    }

    fn blank(
        role: Role,
        cfg: &SessionConfig,
        net: Net,
        log: Option<LogFile>,
        dump: Option<DumpFile>,
    ) -> Session {
        Session {
            role,
            name: cfg.name.clone(),
            net,
            host: None,
            guest: None,
            clock_s: 0.0,
            now_ms: 0,
            tick_acc_ms: 0.0,
            tick_no: 0,
            events: Vec::new(),
            log,
            dump,
            last_out_ms: 0,
            last_out_key: (u64::MAX, 0, 0),
            root: None,
            decode_errors: 0,
            peer_timeout_ms: cfg.peer_timeout_ms,
            link_errors: LinkErrors::default(),
            ready: false,
            last_error: None,
            live: None,
            start: crate::start::StartSide::new(role),
            party: None,
            rules_given: cfg.rules.is_some(),
        }
    }

    /// Starts hosting.
    pub fn host(mut cfg: SessionConfig) -> Result<Session, Error> {
        let (net, log, dump, hello, errors) = Self::common(Role::Host, &mut cfg)?;
        let id = if cfg.session_id == 0 {
            made_up_id()
        } else {
            cfg.session_id
        };
        let mut hc = HostConfig::new(hello, id);
        hc.max_guests = cfg.max_guests;
        hc.compat.allow_mismatch = cfg.compat.allow_mismatch;
        let rules = cfg.rules.unwrap_or_default();
        hc.mode = rules.mode;
        let mut s = Self::blank(Role::Host, &cfg, net, log, dump);
        s.link_errors = errors;
        s.host = Some(HostSide {
            sync: HostSync::new(hc),
            bridge: HostBridge::default(),
            peers: BTreeMap::new(),
            commands: Vec::new(),
            place: None,
            roster_dirty: false,
            last_roster_ms: 0,
        });
        s.set_rules(rules);
        if cfg.live {
            s.live = Some(crate::live::Layer::new(
                Role::Host,
                cfg.policy.clone(),
                cfg.no_policy,
                cfg.native_dialogue,
            ));
        }
        let pid = std::process::id() as u64;
        s.log_ev(
            "start",
            &[
                ("role", V::S("host")),
                ("pid", V::U(pid)),
                ("name", V::S(&cfg.name)),
            ],
        );
        if let Ok(port) = parse_host_port(&cfg.addr) {
            if !cfg.addr.trim().is_empty() {
                s.log_ev("coop_listening", &[("port", V::U(port as u64))]);
            }
        }
        Ok(s)
    }

    /// Starts joining a host.
    pub fn join(mut cfg: SessionConfig) -> Result<Session, Error> {
        let (net, log, dump, hello, errors) = Self::common(Role::Guest, &mut cfg)?;
        let mut s = Self::blank(Role::Guest, &cfg, net, log, dump);
        s.link_errors = errors;
        let mut sync = GuestSync::new(s.net.local(), hello);
        let first = sync.start();
        s.guest = Some(GuestSide {
            sync,
            bridge: GuestBridge::default(),
            place: None,
            outbox: BTreeMap::new(),
            next_cmd: 1,
            nonce: 0,
            ping: None,
            rtt_ms: None,
            last_state_ms: 0,
            last_any_ms: 0,
            last_retry_ms: 0,
            last_info_ms: None,
            last_recheck_ms: 0,
            ever_connected: false,
            active: false,
            lost: false,
            desync_open: false,
            seen_desyncs: 0,
            seen_repairs: 0,
            seen_checks: 0,
            seen_snaps: 0,
            reported_reject: false,
            roster: Vec::new(),
            drift_logged: 0,
        });
        for m in &first {
            send_msg(&mut s.net, PlayerId::HOST, m);
        }
        if cfg.live {
            s.live = Some(crate::live::Layer::new(
                Role::Guest,
                cfg.policy.clone(),
                cfg.no_policy,
                cfg.native_dialogue,
            ));
        }
        let pid = std::process::id() as u64;
        s.log_ev(
            "start",
            &[
                ("role", V::S("guest")),
                ("pid", V::U(pid)),
                ("name", V::S(&cfg.name)),
            ],
        );
        Ok(s)
    }

    /// Which side this is.
    pub fn role(&self) -> Role {
        self.role
    }

    /// The host's place, once a guest has been told (guest only).
    pub fn host_place(&self) -> Option<Place> {
        self.guest.as_ref().and_then(|g| g.place)
    }

    /// The last error the session logged (a failed join, a refusal).
    pub fn last_error(&self) -> Option<&str> {
        self.last_error.as_deref()
    }

    /// True once `coop_ready` was logged (see [`Event::Ready`]).
    pub fn is_ready(&self) -> bool {
        self.ready
    }

    /// Writes an event to the log file (viewer events: `loaded`, `script`,
    /// `shutdown`, ...). `t` is the session clock in seconds.
    pub fn log_event(&mut self, ev: &str, fields: &[(&str, V)]) {
        self.log_ev(ev, fields);
    }

    /// Errors the network transport reported since the last call.
    pub fn take_link_errors(&mut self) -> Vec<String> {
        std::mem::take(&mut *self.link_errors.borrow_mut())
    }

    /// Takes the events since the last call.
    pub fn drain_events(&mut self) -> Vec<Event> {
        std::mem::take(&mut self.events)
    }

    /// Host: commands guests (or [`Session::send_command`] on the host)
    /// asked for, oldest first. Empty on a guest.
    pub fn take_commands(&mut self) -> Vec<(PlayerId, Command)> {
        self.host
            .as_mut()
            .map(|h| std::mem::take(&mut h.commands))
            .unwrap_or_default()
    }

    /// Asks the host to run an action. On a guest it is sent reliably and
    /// resent until acknowledged; on the host it is queued for
    /// [`Session::take_commands`] directly.
    pub fn send_command(&mut self, cmd: Command) {
        let now = self.now_ms;
        let mut line = None;
        if let Some(h) = &mut self.host {
            h.commands.push((PlayerId::HOST, cmd));
            return;
        }
        if let Some(g) = &mut self.guest {
            let id = g.next_cmd;
            g.next_cmd += 1;
            line = Some((id, cmd.as_line()));
            let mut sent = None;
            if g.sync.phase() == GuestPhase::Active {
                send_ext(
                    &mut self.net,
                    PlayerId::HOST,
                    &Ext::Cmd {
                        id,
                        cmd: cmd.clone(),
                    },
                );
                sent = Some(now);
            }
            g.outbox.insert(id, (cmd, sent));
        }
        if let Some((id, l)) = line {
            self.log_ev("cmd_sent", &[("id", V::U(id as u64)), ("line", V::S(&l))]);
        }
    }

    /// Joins refused because the party was full. Host: the players it turned away
    /// (`Reject(Full)`). Guest: 1 when the host refused this guest for that reason.
    pub fn full_refusals(&self) -> u32 {
        match (&self.host, &self.guest) {
            (Some(h), _) => h.sync.full_refusals,
            (_, Some(g)) => u32::from(g.sync.phase() == GuestPhase::Rejected(RejectReason::Full)),
            _ => 0,
        }
    }

    /// True once a guest is receiving the host's state.
    pub fn is_connected(&self) -> bool {
        match (&self.host, &self.guest) {
            (Some(_), _) => true,
            (_, Some(g)) => g.sync.phase() == GuestPhase::Active && !g.lost,
            _ => false,
        }
    }

    /// A snapshot of the session.
    pub fn status(&self) -> Status {
        let (state, player, peers, rtt, rev, desync, desyncs, repairs, snaps, pending) =
            if let Some(h) = &self.host {
                let mut peers = vec![PeerInfo {
                    player: PlayerId::HOST,
                    name: self.name.clone(),
                    rtt_ms: 0,
                }];
                peers.extend(
                    h.peers
                        .iter()
                        .filter(|(_, p)| p.announced)
                        .map(|(id, p)| PeerInfo {
                            player: *id,
                            name: p.name.clone().unwrap_or_else(|| format!("player {}", id.0)),
                            rtt_ms: p.rtt_ms,
                        }),
                );
                let n = peers.len() - 1;
                let _ = n;
                (
                    State::Hosting,
                    PlayerId::HOST,
                    peers,
                    None,
                    h.sync.store.rev().0,
                    false,
                    0,
                    h.sync.repairs_sent,
                    h.sync.snapshots_sent,
                    0,
                )
            } else {
                let g = self.guest.as_ref().expect("guest");
                let state = match g.sync.phase() {
                    GuestPhase::Rejected(r) => State::Rejected(r),
                    GuestPhase::Active if !g.lost => State::Connected,
                    GuestPhase::Joining => State::Joining,
                    _ if g.ever_connected => State::Reconnecting,
                    GuestPhase::Active => State::Reconnecting,
                    _ => State::Connecting,
                };
                let me = g.sync.player;
                let peers = g
                    .roster
                    .iter()
                    .filter(|p| p.player != me)
                    .cloned()
                    .collect();
                (
                    state,
                    me,
                    peers,
                    g.rtt_ms,
                    g.sync.store.rev().0,
                    g.desync_open,
                    g.sync.desyncs_detected,
                    g.sync.repairs_applied,
                    g.sync.snapshots_loaded,
                    g.outbox.len(),
                )
            };
        Status {
            role: self.role,
            state,
            player,
            peers,
            rtt_ms: rtt,
            rev,
            desync,
            desyncs,
            repairs,
            snapshots: snaps,
            pending_commands: pending,
            decode_errors: self.decode_errors,
            root: self.root,
            now_ms: self.now_ms,
        }
    }

    // ---- frame hooks -------------------------------------------------

    /// Start of the frame: receive; a guest applies the replicated state.
    pub fn pre_frame(&mut self, gs: &mut GameState) {
        self.pre_frame_with(
            gs,
            Opts {
                present: true,
                ctx: None,
            },
        );
    }

    /// [`Session::pre_frame`] with game data for journal text and names.
    pub fn pre_frame_with(&mut self, gs: &mut GameState, opts: Opts) {
        self.ensure_authority(gs);
        self.recv_all();
        self.live_setup(gs, opts.ctx.map(|c| c.order));
        // a guest making its character keeps its own rows: the host's copy of them is stale
        gs.coop.bank.hold_own = self.start.flow.holds_own_rows();
        let now = self.now_ms;
        let mut drift = None;
        if let Some(g) = &mut self.guest {
            if g.sync.phase() == GuestPhase::Active {
                if g.sync.snapshots_loaded != g.seen_snaps {
                    g.seen_snaps = g.sync.snapshots_loaded;
                    g.bridge.reset();
                }
                let recheck = now.saturating_sub(g.last_recheck_ms) >= RECHECK_EVERY_MS;
                if recheck {
                    g.last_recheck_ms = now;
                }
                g.bridge.sync(gs, &g.sync.store, recheck, opts);
                if g.bridge.drift_repairs != g.drift_logged {
                    g.drift_logged = g.bridge.drift_repairs;
                    drift = Some(g.drift_logged);
                }
            }
        }
        if let Some(n) = drift {
            self.log_ev("drift_repair", &[("total", V::U(n as u64))]);
        }
    }

    /// End of the frame: the host mirrors the frame's changes; the protocol
    /// clock runs for `dt` seconds; outgoing messages are sent.
    pub fn post_frame(&mut self, gs: &mut GameState, dt: f32) {
        self.ensure_authority(gs);
        if let Some(h) = &mut self.host {
            h.bridge.collect(gs, &mut h.sync);
        }
        self.publish_place(gs);
        self.host_start_frame(gs);
        self.party_tick(gs);
        self.drive(dt as f64);
        self.write_outputs(gs);
    }

    /// Runs the network without a `GameState` (before the world exists,
    /// e.g. while a guest waits for the host's place).
    pub fn poll(&mut self, dt: f32) {
        self.recv_all();
        self.drive(dt as f64);
    }

    fn ensure_authority(&self, gs: &mut GameState) {
        let want = match self.role {
            Role::Host => Authority::Host,
            Role::Guest => Authority::Guest,
        };
        if gs.authority() != want {
            gs.set_authority(want);
        }
    }

    // ---- receiving ---------------------------------------------------

    fn sync_time(&mut self) {
        let now = self.now_ms;
        if let Some(h) = &mut self.host {
            h.sync.set_time_ms(now);
        }
        if let Some(g) = &mut self.guest {
            g.sync.set_time_ms(now);
        }
    }

    fn recv_all(&mut self) {
        self.sync_time();
        self.net.pump();
        while let Some(p) = self.net.recv() {
            if p.data.first() == Some(&MARK) {
                match Ext::decode(&p.data) {
                    Some(e) => self.on_ext(p.from, e),
                    None => self.decode_errors += 1,
                }
                continue;
            }
            match proto::decode(p.channel, &p.data) {
                Ok(m) => self.on_msg(p.from, m),
                Err(_) => self.decode_errors += 1,
            }
        }
    }

    fn on_msg(&mut self, from: PlayerId, m: Msg) {
        let now = self.now_ms;
        if let Some(h) = &mut self.host {
            // a guest's hit request waits for the friendly-fire guard (the live frame)
            let m = match self.live.as_mut().and_then(|l| l.host.as_mut()) {
                Some(ad) => match ad.hold_guest_hit(from, m) {
                    Some(m) => m,
                    None => return,
                },
                None => m,
            };
            // a sample from a placed player: a guest that still loads its
            // first cell sends samples with no cell (the sync tick sends them)
            let avatar = matches!(
                &m,
                Msg::Avatar(coop_core::avatar::AvatarMsg::State { state, .. }) if state.cell.0 != 0
            );
            let replies = h.sync.on_msg(from, m);
            for note in std::mem::take(&mut h.sync.compat_log) {
                let who = format!("Player {}", note.player.0);
                let lines = coop_core::compat::explain(
                    &note.issues,
                    coop_core::compat::Viewpoint::Host,
                    &who,
                    note.refused,
                );
                self.events.push(Event::Compat {
                    lines,
                    refused: note.refused,
                });
            }
            if h.sync.players().contains(&from) {
                let fresh = !h.peers.contains_key(&from);
                let p = h.peers.entry(from).or_insert_with(|| Peer {
                    name: None,
                    rtt_ms: 0,
                    last_heard_ms: now,
                    seen_ms: now,
                    announced: false,
                    cmds_seen: BTreeSet::new(),
                    in_game: false,
                });
                p.last_heard_ms = now;
                p.in_game |= avatar;
                if fresh {
                    h.roster_dirty = true;
                    if let Some(pl) = h.place {
                        send_ext(&mut self.net, from, &Ext::Place(place_for(pl, from)));
                    }
                }
            }
            for (to, r) in replies {
                send_msg(&mut self.net, to, &r);
            }
            return;
        }
        let Some(g) = &mut self.guest else { return };
        if !from.is_host() {
            return;
        }
        g.last_any_ms = now;
        match &m {
            Msg::Pong { nonce } => {
                if let Some((n, t)) = g.ping {
                    if n == *nonce {
                        g.rtt_ms = Some(ping_ms(t, now));
                        g.ping = None;
                    }
                }
                return;
            }
            Msg::Delta { .. } | Msg::Snapshot(_) | Msg::Hash { .. } | Msg::Repair(_) => {
                g.last_state_ms = now
            }
            Msg::Welcome { player, .. } => {
                // a new slot gets a fresh live state now, before the replays
                // that follow the snapshot. `live_setup` made it on the first
                // full frame, after the viewer loaded the cell, and so dropped
                // what arrived during the load (chat, markers, scenes, votes).
                if g.sync.player != *player {
                    g.sync.live = coop_core::sync::GuestLive::new(*player);
                }
                g.sync.player = *player
            }
            _ => {}
        }
        let incompat = matches!(m, Msg::Incompat(_));
        let out = g.sync.on_msg(m);
        if incompat && !matches!(g.sync.phase(), GuestPhase::Rejected(_)) {
            let lines = coop_core::compat::explain(
                &g.sync.compat_issues(),
                coop_core::compat::Viewpoint::Guest,
                "",
                false,
            );
            self.events.push(Event::Compat {
                lines,
                refused: false,
            });
        }
        for r in &out {
            send_msg(&mut self.net, PlayerId::HOST, r);
        }
        if let GuestPhase::Rejected(r) = g.sync.phase() {
            if !g.reported_reject {
                g.reported_reject = true;
                let found = g.sync.compat_issues();
                let lines = if found.is_empty() {
                    vec![coop_core::compat::reason_text(r)]
                } else {
                    coop_core::compat::explain(
                        &found,
                        coop_core::compat::Viewpoint::Guest,
                        "",
                        true,
                    )
                };
                self.events.push(Event::Rejected(r));
                self.events.push(Event::Compat {
                    lines: lines.clone(),
                    refused: true,
                });
                self.log_ev(
                    "error",
                    &[(
                        "msg",
                        V::S(&format!(
                            "host refused the connection: {r:?}: {}",
                            lines.join(" ")
                        )),
                    )],
                );
            }
        }
    }

    fn on_ext(&mut self, from: PlayerId, e: Ext) {
        let e = match e {
            Ext::Party(b) => {
                self.party_recv(from, &b);
                return;
            }
            other => other,
        };
        let now = self.now_ms;
        if let Some(h) = &mut self.host {
            let Some(p) = h.peers.get_mut(&from) else {
                return;
            };
            p.last_heard_ms = now;
            match e {
                Ext::Info { name, rtt_ms } => {
                    if p.name.as_deref() != Some(&name) {
                        p.name = Some(name);
                        h.roster_dirty = true;
                    }
                    p.rtt_ms = rtt_ms;
                }
                Ext::Character(sheet) => {
                    self.on_sheet(from, sheet);
                    return;
                }
                Ext::Cmd { id, cmd } => {
                    let fresh = p.cmds_seen.insert(id);
                    send_ext(&mut self.net, from, &Ext::CmdAck(id));
                    if fresh {
                        let line = cmd.as_line();
                        h.commands.push((from, cmd));
                        self.log_ev(
                            "cmd_recv",
                            &[
                                ("from", V::U(from.0 as u64)),
                                ("id", V::U(id as u64)),
                                ("line", V::S(&line)),
                            ],
                        );
                    }
                }
                _ => {}
            }
            return;
        }
        let Some(g) = &mut self.guest else { return };
        if !from.is_host() {
            return;
        }
        g.last_any_ms = now;
        match e {
            Ext::Place(p) => {
                if g.place != Some(p) {
                    g.place = Some(p);
                    self.events.push(Event::PlaceChanged(p));
                    self.log_ev(
                        "place",
                        &[
                            ("cell", V::U(p.cell as u64)),
                            ("world", V::U(p.world as u64)),
                        ],
                    );
                }
            }
            Ext::CmdAck(id) => {
                g.outbox.remove(&id);
            }
            Ext::Opening(o) => {
                self.on_opening(o);
            }
            Ext::SheetAck => {
                self.on_sheet_ack();
            }
            Ext::CharNames(list) => {
                self.on_char_names(list);
            }
            Ext::Roster(list) => {
                let me = g.sync.player;
                let old: Vec<PeerInfo> = g.roster.clone();
                let mut joined = Vec::new();
                let mut left = Vec::new();
                for p in list
                    .iter()
                    .filter(|p| p.player != me && !p.player.is_host())
                {
                    if !old.iter().any(|o| o.player == p.player) && !old.is_empty() {
                        joined.push((p.player, p.name.clone()));
                    }
                }
                for o in old.iter().filter(|o| o.player != me && !o.player.is_host()) {
                    if !list.iter().any(|p| p.player == o.player) {
                        left.push((o.player, o.name.clone()));
                    }
                }
                g.roster = list;
                for (p, n) in joined {
                    self.events.push(Event::PeerJoined(p, n));
                }
                for (p, n) in left {
                    self.events.push(Event::PeerLeft(p, n));
                }
            }
            _ => {}
        }
    }

    // ---- the protocol clock ------------------------------------------

    fn publish_place(&mut self, gs: &GameState) {
        let Some(h) = &mut self.host else { return };
        let Some(cell) = gs.player_cell else { return };
        let now_place = Place {
            cell: cell.0,
            world: gs.player_world.map_or(0, |w| w.0),
            pos: gs.player_position,
        };
        let moved = h.place.map(|p| p.key()) != Some(now_place.key());
        h.place = Some(now_place);
        if moved {
            for to in h.peers.keys() {
                send_ext(&mut self.net, *to, &Ext::Place(place_for(now_place, *to)));
            }
            self.log_ev(
                "place",
                &[
                    ("cell", V::U(now_place.cell as u64)),
                    ("world", V::U(now_place.world as u64)),
                ],
            );
        }
    }

    fn drive(&mut self, dt: f64) {
        self.clock_s += dt.clamp(0.0, 1.0);
        self.now_ms = (self.clock_s * 1000.0) as u64;
        self.sync_time();
        self.net.advance(self.now_ms);
        self.tick_acc_ms += dt.clamp(0.0, 1.0) * 1000.0;
        let mut n = 0;
        while self.tick_acc_ms >= TICK_MS && n < 8 {
            self.tick_acc_ms -= TICK_MS;
            self.tick_once();
            n += 1;
        }
        if self.tick_acc_ms >= TICK_MS {
            self.tick_acc_ms = 0.0;
        }
        self.housekeeping();
        self.net.pump();
    }

    fn tick_once(&mut self) {
        self.tick_no += 1;
        let live = self.live.is_some();
        if let Some(h) = &mut self.host {
            // a live guest that has not played yet still loads its first cell
            for (id, p) in &h.peers {
                h.sync.hold(*id, live && !p.in_game);
            }
            for (to, m) in h.sync.tick() {
                send_msg(&mut self.net, to, &m);
            }
            if self.tick_no % HASH_EVERY_TICKS == 0 {
                for (to, m) in h.sync.hash_broadcast() {
                    send_msg(&mut self.net, to, &m);
                }
            }
        }
        let now = self.now_ms;
        if let Some(g) = &mut self.guest {
            for m in g.sync.tick(Tick(self.tick_no as u32)) {
                send_msg(&mut self.net, PlayerId::HOST, &m);
            }
            if self.tick_no % PING_EVERY_TICKS == 0 && g.sync.phase() != GuestPhase::Idle {
                g.nonce = g.nonce.wrapping_add(1);
                g.ping = Some((g.nonce, now));
                send_msg(&mut self.net, PlayerId::HOST, &Msg::Ping { nonce: g.nonce });
            }
        }
    }

    fn housekeeping(&mut self) {
        self.guest_start_tick();
        let now = self.now_ms;
        let mut logs: Vec<(&'static str, Vec<(String, String)>)> = Vec::new();
        let live = self.live.is_some();
        if let Some(h) = &mut self.host {
            let alive = h.sync.players();
            let gone: Vec<PlayerId> = h
                .peers
                .iter()
                .filter(|(id, p)| {
                    let limit = if live && !p.in_game {
                        self.peer_timeout_ms.max(LOAD_TIMEOUT_MS)
                    } else {
                        self.peer_timeout_ms
                    };
                    !alive.contains(id) || now.saturating_sub(p.last_heard_ms) > limit
                })
                .map(|(id, _)| *id)
                .collect();
            for id in gone {
                h.sync.disconnect(id);
                if let Some(p) = h.peers.remove(&id) {
                    if p.announced {
                        let name = p.name.unwrap_or_else(|| format!("player {}", id.0));
                        logs.push((
                            "peer_leave",
                            vec![
                                ("player".into(), id.0.to_string()),
                                ("name".into(), name.clone()),
                            ],
                        ));
                        self.events.push(Event::PeerLeft(id, name));
                    }
                }
                h.roster_dirty = true;
            }
            for (id, p) in h.peers.iter_mut() {
                if !p.announced && (p.name.is_some() || now.saturating_sub(p.seen_ms) > 1500) {
                    p.announced = true;
                    h.roster_dirty = true;
                    let name = p.name.clone().unwrap_or_else(|| format!("player {}", id.0));
                    logs.push((
                        "peer_join",
                        vec![
                            ("player".into(), id.0.to_string()),
                            ("name".into(), name.clone()),
                        ],
                    ));
                    logs.push(("coop_connected", vec![("role".into(), "host".into())]));
                    self.events.push(Event::PeerJoined(*id, name));
                }
            }
            if h.roster_dirty || now.saturating_sub(h.last_roster_ms) >= 5000 {
                h.roster_dirty = false;
                h.last_roster_ms = now;
                let mut list = vec![PeerInfo {
                    player: PlayerId::HOST,
                    name: self.name.clone(),
                    rtt_ms: 0,
                }];
                list.extend(
                    h.peers
                        .iter()
                        .filter(|(_, p)| p.announced)
                        .map(|(id, p)| PeerInfo {
                            player: *id,
                            name: p.name.clone().unwrap_or_else(|| format!("player {}", id.0)),
                            rtt_ms: p.rtt_ms,
                        }),
                );
                let e = Ext::Roster(list);
                for to in h.peers.keys() {
                    send_ext(&mut self.net, *to, &e);
                }
            }
        }
        if let Some(g) = &mut self.guest {
            let phase = g.sync.phase();
            // transitions
            let is_active = phase == GuestPhase::Active && !g.lost;
            if phase == GuestPhase::Active && !g.active {
                g.active = true;
                g.last_state_ms = now;
                if g.lost {
                    g.lost = false;
                }
                if g.ever_connected {
                    self.events.push(Event::Reconnected);
                    logs.push(("reconnected", vec![]));
                } else {
                    self.events.push(Event::Connected);
                    logs.push(("coop_connected", vec![("role".into(), "guest".into())]));
                }
                g.ever_connected = true;
            }
            let _ = is_active;
            match phase {
                GuestPhase::Active => {
                    if now.saturating_sub(g.last_state_ms) > STALE_MS
                        && now.saturating_sub(g.last_retry_ms) >= RETRY_MS - 1000
                    {
                        if !g.lost {
                            g.lost = true;
                            self.events.push(Event::Lost);
                            logs.push(("lost", vec![]));
                        }
                        g.active = false;
                        g.last_retry_ms = now;
                        for m in g.sync.reconnect() {
                            send_msg(&mut self.net, PlayerId::HOST, &m);
                        }
                    }
                }
                GuestPhase::Greeting | GuestPhase::Joining => {
                    let wait = if g.last_any_ms == 0 { 1000 } else { RETRY_MS };
                    if now.saturating_sub(g.last_any_ms.max(g.last_retry_ms)) > wait {
                        g.last_retry_ms = now;
                        for m in g.sync.reconnect() {
                            send_msg(&mut self.net, PlayerId::HOST, &m);
                        }
                    }
                }
                _ => {}
            }
            if matches!(phase, GuestPhase::Joining | GuestPhase::Active)
                && g.last_info_ms
                    .map_or(true, |t| now.saturating_sub(t) >= 2000)
            {
                g.last_info_ms = Some(now);
                let e = Ext::Info {
                    name: self.name.clone(),
                    rtt_ms: g.rtt_ms.unwrap_or(0),
                };
                send_ext(&mut self.net, PlayerId::HOST, &e);
            }
            if g.sync.phase() == GuestPhase::Active {
                let due: Vec<u32> = g
                    .outbox
                    .iter()
                    .filter(|(_, (_, sent))| {
                        sent.map_or(true, |t| now.saturating_sub(t) >= CMD_RESEND_MS)
                    })
                    .map(|(id, _)| *id)
                    .collect();
                for id in due {
                    if let Some((cmd, sent)) = g.outbox.get_mut(&id) {
                        *sent = Some(now);
                        send_ext(
                            &mut self.net,
                            PlayerId::HOST,
                            &Ext::Cmd {
                                id,
                                cmd: cmd.clone(),
                            },
                        );
                    }
                }
            }
            // desync bookkeeping
            let new_desync = g.sync.desyncs_detected > g.seen_desyncs;
            let new_check = g.sync.hash_checks > g.seen_checks;
            if new_desync {
                g.desync_open = true;
                self.events.push(Event::Desync);
                logs.push((
                    "desync",
                    vec![("total".into(), g.sync.desyncs_detected.to_string())],
                ));
            } else if new_check {
                g.desync_open = false;
            }
            g.seen_desyncs = g.sync.desyncs_detected;
            g.seen_checks = g.sync.hash_checks;
            if g.sync.repairs_applied > g.seen_repairs {
                g.seen_repairs = g.sync.repairs_applied;
                self.events.push(Event::Repaired);
                logs.push((
                    "coop_repair",
                    vec![
                        ("kind".into(), "hash".into()),
                        ("subsystem".into(), "shared".into()),
                    ],
                ));
            }
        }
        let ready_now = match (&self.host, &self.guest) {
            (Some(h), _) => h.peers.values().any(|p| p.announced),
            (_, Some(g)) => g.sync.phase() == GuestPhase::Active && !g.lost && g.bridge.applies > 0,
            _ => false,
        };
        if ready_now && !self.ready {
            self.ready = true;
            self.events.push(Event::Ready);
            let peers = if self.host.is_some() {
                self.status().peers.len() as u64
            } else {
                1
            };
            let role = if self.host.is_some() { "host" } else { "guest" };
            self.log_ev(
                "coop_ready",
                &[("role", V::S(role)), ("peers", V::U(peers))],
            );
        }
        let errs = self.take_link_errors();
        for e in errs {
            let ev = if e.starts_with("send error") {
                "link_warn"
            } else {
                "error"
            };
            self.log_ev(ev, &[("msg", V::S(&e))]);
        }
        for (ev, fields) in logs {
            let f: Vec<(&str, V)> = fields
                .iter()
                .map(|(k, v)| (k.as_str(), V::S(v.as_str())))
                .collect();
            self.log_ev(ev, &f);
        }
    }

    // ---- output ------------------------------------------------------

    pub(crate) fn log_ev(&mut self, ev: &str, fields: &[(&str, V)]) {
        if ev == "error" {
            if let Some((_, V::S(m))) = fields.iter().find(|(k, _)| *k == "msg") {
                self.last_error = Some(m.to_string());
            }
        }
        let t = self.now_ms;
        if let Some(l) = &mut self.log {
            l.event(t, ev, fields);
        }
    }

    /// Hash of the shared part of `gs` (what the dump and log record).
    pub fn state_root(gs: &GameState) -> u64 {
        gs.shard_hashes().root()
    }

    /// The revision that [`Session::state_root`] belongs to now. On a host it
    /// also gives the roots of the revisions of the last
    /// [`RECENT_ROOTS_MS`]. Script timers change the game state every frame,
    /// so a guest dumps its state a few revisions behind the host. The
    /// comparison then uses the host's root at the guest's revision.
    pub fn hash_points(&self) -> HashPoints {
        if let Some(h) = &self.host {
            let keep = |f: &coop_core::schema::Field| !world::coop::export::is_tombstone(f);
            let recent: Vec<(u64, u64)> = h
                .sync
                .recent_roots(RECENT_ROOTS_MS, &keep)
                .into_iter()
                .map(|(r, root)| (r.0, root))
                .collect();
            return HashPoints {
                rev: Some(h.sync.store.rev().0),
                recent,
            };
        }
        HashPoints {
            rev: self
                .guest
                .as_ref()
                .and_then(|g| g.bridge.applied_rev())
                .map(|r| r.0),
            recent: Vec::new(),
        }
    }

    /// Writes the dump file and a `sync` log line now, whatever changed.
    pub fn dump_now(&mut self, gs: &GameState) {
        self.last_out_key = (u64::MAX, 0, 0);
        self.last_out_ms = 0;
        self.write_outputs_at(gs, true);
    }

    fn write_outputs(&mut self, gs: &GameState) {
        self.write_outputs_at(gs, false);
    }

    fn write_outputs_at(&mut self, gs: &GameState, force: bool) {
        if self.log.is_none() && self.dump.is_none() {
            return;
        }
        if !force && self.now_ms.saturating_sub(self.last_out_ms) < OUT_EVERY_MS {
            return;
        }
        let (rev, applies) = match (&self.host, &self.guest) {
            (Some(h), _) => (
                h.sync.store.rev().0,
                h.bridge.uncovered_total.values().sum::<u32>() as u64,
            ),
            (_, Some(g)) => (g.sync.store.rev().0, g.bridge.applies as u64),
            _ => (0, 0),
        };
        let key = (rev, applies as u32, gs.coop.stats.effects);
        if key == self.last_out_key && !force {
            self.last_out_ms = self.now_ms;
            return;
        }
        self.last_out_key = key;
        self.last_out_ms = self.now_ms;
        let root = Self::state_root(gs);
        self.root = Some(root);
        let hex = format!("{root:016x}");
        let role = if self.role == Role::Host {
            "host"
        } else {
            "guest"
        };
        let player = self.status().player.0 as u64;
        let now = self.now_ms;
        if let Some(d) = &self.dump {
            let fields: Vec<String> = gs
                .export_fields()
                .iter()
                .map(|(f, v)| format!("{:08X} {v:?}", f.0))
                .collect();
            d.write(
                &[
                    ("role", V::S(role)),
                    ("player", V::U(player)),
                    ("t_ms", V::U(now)),
                    ("rev", V::U(rev)),
                    ("root", V::S(&hex)),
                    ("fields", V::U(fields.len() as u64)),
                ],
                &fields,
            );
        }
        self.log_ev("sync", &[("rev", V::U(rev)), ("root", V::S(&hex))]);
    }
}

impl Session {
    /// Fault injection for tests (the bot's `coop desync`): silently changes
    /// one quest record of this guest's replicated store (same revision,
    /// wrong value), as a garbled update would. Nothing else is touched: the
    /// next hash exchange must find the mismatch and the host must repair it
    /// (the status line's desync and repair counts). False on a host, or
    /// when the store holds no quest record.
    pub fn inject_desync(&mut self) -> bool {
        use coop_core::schema::Field;
        let Some(g) = self.guest.as_mut() else {
            return false;
        };
        let store = &mut g.sync.store;
        let Some(target) = store
            .entries()
            .into_iter()
            .find(|e| matches!(e.field, Field::QuestStage { .. } | Field::QuestFlags { .. }))
        else {
            return false;
        };
        let mut bad = target.clone();
        bad.field = match &target.field {
            Field::QuestStage { stage } => Field::QuestStage {
                stage: stage.wrapping_add(1),
            },
            Field::QuestFlags { flags } => Field::QuestFlags { flags: flags ^ 1 },
            other => other.clone(),
        };
        let shard = coop_core::ids::shard_of(target.form);
        let mut repl: Vec<_> = store
            .entries_in_shards(&[shard])
            .into_iter()
            .filter(|e| e != &target)
            .collect();
        repl.push(bad);
        store.replace_shards(&[shard], repl);
        true
    }
}

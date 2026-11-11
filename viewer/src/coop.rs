//! Co-op in nv-viewer (docs/coop/PLAN.md M3): `NV_COOP=host` (or `host:PORT`) hosts the story,
//! `NV_COOP=join:CODE` follows a host's story with the join code the host printed. Between two of
//! the game's frames the session takes the network's messages and runs its step
//! (`coop_engine_session`): the host sends its story's changes, a guest applies them to its game
//! with the story's scripts gated. `NV_COOP=journal` is the change journal's log
//! (`coop_engine::diag`), not a session.
//!
//! The live ladder (docs/coop/DECISIONS.md D3) runs with a session: `NV_COOP_LIVE=log` reads and
//! sends the players' places only, `warp` also moves a guest to the host's cell, `avatar` (the
//! default) also shows the other players; `off` leaves the session story-only.
//! `NV_COOP_AVATAR=<hex form id>` is the base form of an avatar (default 7, the player's own).
//! `NV_COOP_WORLD=doors` shares doors and locks but no changed-form records, `off` neither
//! (docs/coop/SPECS.md S12).
//!
//! Shared fights (fights; docs/coop/SPECS.md S13) run with the ladder: `NV_COOP_FIGHT=state` shows
//! the host's actors on the guests, `damage` also sends hits through the host, `threat` also lets
//! the host's actors turn on a guest, `revive` (the default) also holds a player's death for a
//! revive; `off` leaves every game running its own fights.
//!
//! Both players start the same game (a new game, or the same save) from installations whose
//! plugins load in the same order; the handshake refuses a guest whose plugins differ.

use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};
use std::path::Path;
use std::time::Instant;

use coop_core::sync::HostConfig;
use coop_engine_session::ladder::{Cap, Ladder, DEFAULT_AVATAR_BASE};
use coop_engine_session::{self as session, GuestDriver, HostDriver};
use coop_net::{
    random_token, Endpoint, EndpointConfig, JoinCode, JoinTarget, NetEvent, UdpBackend, UdpConfig,
};
use engine_core::Engine;

/// The UDP port a host uses unless told another.
pub const DEFAULT_PORT: u16 = 27961;

/// What `NV_COOP` asks for.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ask {
    /// Host on this UDP port.
    Host(u16),
    /// Join with this join code.
    Join(String),
}

/// `NV_COOP`'s value as a session request; `None` for no session (unset, `0`, `journal`).
pub fn parse(v: &str) -> Result<Option<Ask>, String> {
    let v = v.trim();
    if v.is_empty() || v == "0" || v == "journal" {
        return Ok(None);
    }
    if v == "host" {
        return Ok(Some(Ask::Host(DEFAULT_PORT)));
    }
    if let Some(port) = v.strip_prefix("host:") {
        return port
            .trim()
            .parse::<u16>()
            .ok()
            .filter(|p| *p != 0)
            .map(|p| Some(Ask::Host(p)))
            .ok_or_else(|| format!("NV_COOP={v}: the port must be a number from 1 to 65535"));
    }
    if let Some(code) = v.strip_prefix("join:") {
        let code = code.trim();
        code.parse::<JoinCode>()
            .map_err(|e| format!("NV_COOP={v}: not a join code ({e:?})"))?;
        return Ok(Some(Ask::Join(code.to_string())));
    }
    Err(format!(
        "NV_COOP={v}: use host, host:PORT, join:CODE or journal"
    ))
}

/// Globals the game's clock moves by itself on every machine (coop-core's volatile keys and the
/// calendar that turns over with them). They are not story state, and sending them would set each
/// guest's clock back every frame; time is shared separately.
const CLOCK_GLOBALS: &[&str] = &["GameDay", "GameMonth", "GameYear"];

fn is_clock_global(editor_id: &str) -> bool {
    coop_core::compat::VOLATILE_KEYS
        .iter()
        .chain(CLOCK_GLOBALS)
        .any(|k| k.eq_ignore_ascii_case(editor_id))
}

/// The game's data as co-op needs it: the plugin names in load order, and every quest's and every
/// story global's form id, from the game's `Data` folder.
struct GameData {
    plugins: Vec<String>,
    /// The shared story's quests.
    quests: Vec<u32>,
    /// The quests each player runs for themself (the rules' per-player rows, by this load
    /// order): the opening and character creation among them.
    own_quests: Vec<u32>,
    /// Scripts a guest runs itself whatever their quest (the rules' guest-local scripts).
    own_scripts: Vec<u32>,
    globals: Vec<u32>,
    /// The game clock's globals: shared as the clock (the host's, about once a second).
    clock: Vec<u32>,
}

/// The form ids, in the load order `plugins`, of the quests the rules class as per-player. A
/// row's plugin that is not loaded is skipped.
fn own_quest_ids(plugins: &[String]) -> Vec<u32> {
    let Ok(table) = coop_policy::PolicyTable::bundled() else {
        return Vec::new();
    };
    table
        .rows()
        .iter()
        .filter(|r| r.scope == coop_policy::Scope::PerPlayer && !r.synthetic)
        .filter_map(|r| {
            let index = plugins
                .iter()
                .position(|p| p.eq_ignore_ascii_case(&r.plugin))?;
            Some(((index as u32) << 24) | (r.local & 0x00ff_ffff))
        })
        .collect()
}

/// The form ids, in the load order `plugins`, of the scripts the rules let a guest run itself
/// (`guest_local`, written `plugin:hex id`). An entry whose plugin is not loaded is skipped.
fn own_script_ids(plugins: &[String]) -> Vec<u32> {
    let Ok(table) = coop_policy::PolicyTable::bundled() else {
        return Vec::new();
    };
    table
        .rows()
        .iter()
        .flat_map(|r| r.guest_local.iter())
        .filter_map(|s| {
            let (plugin, local) = s.rsplit_once(':')?;
            let index = plugins
                .iter()
                .position(|p| p.eq_ignore_ascii_case(plugin))?;
            let local = u32::from_str_radix(local, 16).ok()?;
            Some(((index as u32) << 24) | (local & 0x00ff_ffff))
        })
        .collect()
}

fn game_data(data: &Path) -> Result<GameData, String> {
    let active = match esm::load_order::default_plugins_txt().and_then(|p| std::fs::read(p).ok()) {
        Some(bytes) => esm::ActivePlugins::List(esm::load_order::parse_plugins_txt(&bytes)),
        None => esm::ActivePlugins::OfficialOnly,
    };
    let order = esm::LoadOrder::from_data_dir(data, &active)
        .map_err(|e| format!("co-op: reading the load order: {e}"))?;
    let plugins: Vec<String> = order.plugins().iter().map(|p| p.name.clone()).collect();
    let own_quests = own_quest_ids(&plugins);
    let own_scripts = own_script_ids(&plugins);
    let quests = order
        .records_of_type(esm::FourCC::new(b"QUST"))
        .map(|r| r.form_id.0)
        .filter(|id| !own_quests.contains(id))
        .collect();
    let (clock, globals): (Vec<_>, Vec<_>) = order
        .records_of_type(esm::FourCC::new(b"GLOB"))
        .map(|r| {
            let clock = r
                .editor_id()
                .ok()
                .flatten()
                .is_some_and(|id| is_clock_global(&id));
            (clock, r.form_id.0)
        })
        .partition(|(clock, _)| *clock);
    let clock = clock.into_iter().map(|(_, id)| id).collect();
    let globals = globals.into_iter().map(|(_, id)| id).collect();
    Ok(GameData {
        plugins,
        quests,
        own_quests,
        own_scripts,
        globals,
        clock,
    })
}

/// This machine's address on its local network, for the join code (a guest on the internet needs
/// the host's public address and the port forwarded).
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

/// The address in a host's join code. `NV_COOP_ADDRESS` gives it when friends reach the host at
/// another address than its own network's: its public address, or its address in a private network
/// app. Anything else there is ignored, and the local network's address stands.
fn join_ip() -> IpAddr {
    address_from(std::env::var("NV_COOP_ADDRESS").ok().as_deref()).unwrap_or_else(local_ip)
}

fn address_from(value: Option<&str>) -> Option<IpAddr> {
    value
        .and_then(|v| v.trim().parse::<IpAddr>().ok())
        .filter(|ip| !ip.is_unspecified())
}

enum Role {
    Host(Box<HostDriver>),
    /// A guest before the host accepted it, then with its driver.
    Guest(Option<Box<GuestDriver>>),
}

/// A running session.
pub struct Coop {
    ep: Endpoint<UdpBackend>,
    role: Role,
    plugins: Vec<String>,
    own_quests: Vec<u32>,
    own_scripts: Vec<u32>,
    started: Instant,
    last_status: Option<Instant>,
    /// The skipped story scripts already named in the log.
    gate_named: std::collections::BTreeSet<u32>,
    // social
    /// This player's name (its party key is a hash of it) and own quests, for a guest's part of
    /// a party save; whether the parts of co-op that found no function to wrap were named.
    social: Social,
}

// social
struct Social {
    name: String,
    own_quests: Vec<u32>,
    missing_named: bool,
}

// social
/// Where a host keeps the party's files: `NV_COOP_PARTY_DIR`, or `coop-party` in the folder the
/// game was started from.
fn party_dir() -> std::path::PathBuf {
    std::env::var_os("NV_COOP_PARTY_DIR")
        .filter(|v| !v.is_empty())
        .map_or_else(|| "coop-party".into(), std::path::PathBuf::from)
}

impl Coop {
    /// The live ladder `NV_COOP_LIVE` and `NV_COOP_AVATAR` ask for; `None` for `off`. Returns the
    /// line that says so.
    fn ladder(guest: bool) -> (Option<Ladder>, String) {
        let live = std::env::var("NV_COOP_LIVE").unwrap_or_default();
        if live.trim() == "off" {
            return (None, "co-op live: off (NV_COOP_LIVE=off)".into());
        }
        let cap = Cap::parse(&live);
        let base = std::env::var("NV_COOP_AVATAR")
            .ok()
            .and_then(|v| u32::from_str_radix(v.trim().trim_start_matches("0x"), 16).ok())
            .unwrap_or(DEFAULT_AVATAR_BASE);
        (
            Some(Ladder::new(guest, cap, base)),
            format!("co-op live: ladder up to {cap:?}, avatar base form {base:08x}"),
        )
    }

    // world
    /// The world sharing's counters, for the status lines; `None` when it is off.
    fn world_status(&self) -> Option<String> {
        match &self.role {
            Role::Host(h) => h.world.as_ref().map(|w| {
                format!(
                    "co-op host world: records sent {}, too large {}, guests' doors and locks done {}, refused {}",
                    w.records, w.too_large, w.enacted, w.refused
                )
            }),
            Role::Guest(Some(g)) => g.world.as_ref().map(|w| {
                format!(
                    "co-op guest world: records loaded {} (own record differs after {}), refused {}, doors and locks set {}, told the host {}, put back {}",
                    w.records_applied, w.records_differ, w.refused, w.set, w.told_host, w.put_back
                )
            }),
            Role::Guest(None) => None,
        }
    }

    // world
    /// What `NV_COOP_WORLD` asks to share of the world beyond the story (everything unless it
    /// says `doors` or `off`).
    fn world_share() -> session::world::Share {
        session::world::Share::parse(&std::env::var("NV_COOP_WORLD").unwrap_or_default())
    }

    /// fights: the shared fights `NV_COOP_FIGHT` asks for; `None` for `off` or without the ladder.
    /// Returns the line that says so.
    fn fight(guest: bool, ladder: bool) -> (Option<session::fight::Fight>, String) {
        let ask = std::env::var("NV_COOP_FIGHT").unwrap_or_default();
        if ask.trim() == "off" || !ladder {
            return (None, "co-op fights: off".into());
        }
        let cap = session::fight::Cap::parse(&ask);
        let mut fight = session::fight::Fight::new(guest, cap);
        // vats: `NV_COOP_VATS=off` leaves V.A.T.S. as the game has it, `slow` only keeps it
        // from slowing the game (docs/coop/SPECS.md S20).
        let vats = session::vats::Mode::parse(&std::env::var("NV_COOP_VATS").unwrap_or_default());
        fight.vats = session::vats::Share::new(vats);
        (
            Some(fight),
            format!("co-op fights: shared up to {cap:?}, V.A.T.S. {vats:?}"),
        )
    }

    /// Opens the session `ask` asks for, for the installation whose `Data` folder is `data`.
    /// Returns the session and the lines to print (the join code).
    pub fn open(ask: &Ask, data: &Path, name: &str) -> Result<(Coop, Vec<String>), String> {
        let GameData {
            plugins,
            quests,
            own_quests,
            own_scripts,
            globals,
            clock,
        } = game_data(data)?;
        let cfg = EndpointConfig {
            name: name.to_string(),
            ..EndpointConfig::default()
        };
        let mut lines = Vec::new();
        let (ep, role) = match ask {
            Ask::Host(port) => {
                let token = random_token();
                let bind = SocketAddr::from(([0, 0, 0, 0], *port));
                let b = UdpBackend::host(bind, token, UdpConfig::default())
                    .map_err(|e| format!("co-op: UDP port {port}: {e}"))?;
                let ip = join_ip();
                let code = JoinCode::new(JoinTarget::Direct(SocketAddr::new(ip, *port)), token);
                lines.push(format!(
                    "co-op: hosting on UDP port {port}; join code for this network: {code} (over the internet: forward UDP port {port} and give your public address)"
                ));
                let session_id = u64::from_le_bytes(token[..8].try_into().expect("8 bytes"));
                let mut host = HostDriver::new(
                    HostConfig::new(session::hello(plugins.clone()), session_id),
                    quests,
                    globals,
                );
                lines.push(format!(
                    "co-op: sharing the game clock ({} globals)",
                    clock.len()
                ));
                host.set_clock(clock);
                // social
                host.social.party.dir = Some(party_dir());
                let (ladder, line) = Self::ladder(false);
                host.ladder = ladder;
                lines.push(line);
                // world
                let share = Self::world_share();
                lines.push(share.line().into());
                if let Some(w) = share.host() {
                    host.share_world(w);
                }
                // fights
                let (fight, line) = Self::fight(false, host.ladder.is_some());
                host.fight = fight;
                lines.push(line);
                (Endpoint::host(b, token, cfg), Role::Host(Box::new(host)))
            }
            Ask::Join(code) => {
                let code: JoinCode = code
                    .parse()
                    .map_err(|e| format!("co-op: join code: {e:?}"))?;
                let JoinTarget::Direct(addr) = code.target else {
                    return Err("co-op: this build joins UDP codes only (not a Steam lobby)".into());
                };
                let b = UdpBackend::guest(addr, code.token, UdpConfig::default())
                    .map_err(|e| format!("co-op: {addr}: {e}"))?;
                lines.push(format!("co-op: joining {addr}"));
                (Endpoint::guest(b, code.token, cfg), Role::Guest(None))
            }
        };
        Ok((
            Coop {
                ep,
                role,
                plugins,
                own_quests: own_quests.clone(),
                own_scripts,
                started: Instant::now(),
                last_status: None,
                gate_named: Default::default(),
                // social
                social: Social {
                    name: name.to_string(),
                    own_quests,
                    missing_named: false,
                },
            },
            lines,
        ))
    }

    /// One step after one of the game's frames (no frame in flight). Returns lines to print.
    pub fn after_frame(&mut self, e: &mut Engine) -> Vec<String> {
        let now_ms = self.started.elapsed().as_millis() as u64;
        let mut lines = Vec::new();
        self.ep.pump();
        for ev in self.ep.events() {
            match ev {
                NetEvent::Connected { local } => {
                    lines.push(format!("co-op: joined as player {}", local.0));
                    match &mut self.role {
                        Role::Guest(slot @ None) => {
                            let mut g =
                                GuestDriver::new(local, session::hello(self.plugins.clone()));
                            let (ladder, line) = Self::ladder(true);
                            g.ladder = ladder;
                            lines.push(line);
                            // world
                            let share = Self::world_share();
                            lines.push(share.line().into());
                            g.world = share.guest();
                            // fights
                            let (fight, line) = Self::fight(true, g.ladder.is_some());
                            g.fight = fight;
                            lines.push(line);
                            // social
                            g.social.party.key = session::party::key_of(&self.social.name);
                            g.social.party.own_quests = self.social.own_quests.clone();
                            *slot = Some(Box::new(g));
                        }
                        // Joined again after losing the host: resume from what this guest has.
                        Role::Guest(Some(g)) => g.rejoin(local, &mut self.ep),
                        Role::Host(_) => {}
                    }
                }
                NetEvent::PeerJoined { player, name } => {
                    lines.push(format!("co-op: {name} joined (player {})", player.0));
                }
                NetEvent::PeerLeft { player, reason } => {
                    lines.push(format!("co-op: player {} left ({reason:?})", player.0));
                    match &mut self.role {
                        Role::Host(h) => h.sync.disconnect(player),
                        // The host is gone: keep the game gated and try to join again.
                        Role::Guest(_) => {
                            lines.push("co-op: lost the host; joining again".into());
                            self.ep.rejoin();
                        }
                    }
                }
                NetEvent::Rejected(code) => {
                    lines.push(format!("co-op: the host refused us ({code:?})"))
                }
                NetEvent::JoinFailed(why) => {
                    lines.push(format!("co-op: could not join: {why}"));
                    // A guest that already follows the story keeps trying to get back.
                    if let Role::Guest(Some(_)) = self.role {
                        self.ep.rejoin();
                    }
                }
                NetEvent::SendError(why) => lines.push(format!("co-op: send failed: {why}")),
            }
        }
        if let Role::Guest(_) = self.role {
            // Gated from the first frame of a join, before the host answers; its own quests (the
            // opening, character creation) run.
            if !self.own_quests.is_empty() {
                coop_engine::gate::set_own_quests(e, std::mem::take(&mut self.own_quests));
                coop_engine::gate::set_own_scripts(e, std::mem::take(&mut self.own_scripts));
            }
            session::gate_guest(e);
        }
        match &mut self.role {
            Role::Host(h) => {
                if h.sync.tick_no().0 == 0 {
                    h.start(e);
                }
                h.step(e, &mut self.ep, now_ms);
                for note in h.unencodable_notes.drain(..) {
                    lines.push(format!("co-op host: {note}"));
                }
                for note in h.claim_notes.drain(..) {
                    lines.push(format!("co-op host: {note}"));
                }
                // world
                for note in h.world.iter_mut().flat_map(|w| w.notes.drain(..)) {
                    lines.push(format!("co-op host world: {note}"));
                }
                // social
                for note in (h.social.party.notes.drain(..)).chain(h.social.enactor.notes.drain(..))
                {
                    lines.push(format!("co-op host: {note}"));
                }
            }
            Role::Guest(Some(g)) => {
                g.step(e, &mut self.ep, now_ms);
                for note in g.healed_notes.drain(..) {
                    lines.push(format!("co-op guest healed: {note}"));
                }
                // world
                for note in g.world.iter_mut().flat_map(|w| w.notes.drain(..)) {
                    lines.push(format!("co-op guest world: {note}"));
                }
                // social
                for note in g.social.party.notes.drain(..) {
                    lines.push(format!("co-op guest: {note}"));
                }
            }
            Role::Guest(None) => {}
        }
        // social
        if !self.social.missing_named {
            self.social.missing_named = true;
            for at in coop_engine::social::missing(e) {
                lines.push(format!(
                    "co-op: no function at {at:08x} to take part in; that part of conversations or rests is off"
                ));
            }
        }
        if self
            .last_status
            .map_or(true, |t| t.elapsed().as_secs() >= 5)
        {
            self.last_status = Some(Instant::now());
            lines.push(self.status(e));
            // world
            lines.extend(self.world_status());
            // fights
            match &self.role {
                Role::Host(h) => {
                    lines.extend(h.fight.iter().map(|f| format!("co-op {}", f.status())))
                }
                Role::Guest(Some(g)) => {
                    lines.extend(g.fight.iter().map(|f| format!("co-op {}", f.status())))
                }
                Role::Guest(None) => {}
            }
            // social
            lines.push(self.social_status(e));
            if let Role::Guest(_) = self.role {
                // Which story scripts this guest holds back, each named once (the first 80): what
                // a guest would have to run itself to have its own start.
                for (id, s) in coop_engine::gate::skipped_scripts(e) {
                    if self.gate_named.len() < 80 && self.gate_named.insert(id) {
                        lines.push(format!(
                            "co-op gate: holding back script {id:08x} (quest script {}, owner quest {:08x})",
                            if s.quest_script { "yes" } else { "no" },
                            s.owner_quest
                        ));
                    }
                }
            }
        }
        lines
    }

    // social
    /// A one-line status of conversations, party saves and rests.
    fn social_status(&self, e: &mut Engine) -> String {
        let (held, own, unknown) = coop_engine::talk::counts(e);
        match &self.role {
            Role::Host(h) => {
                let s = &h.social;
                format!(
                    "co-op host social: conversations {} (guests' results run {}, left {}), lines watched {}, party saves {} ({} files), parts given back {}, rests passed {} ({} h)",
                    s.talker.opened,
                    s.enactor.enacted,
                    s.enactor.left,
                    s.talker.lines_shown,
                    s.party.saved,
                    s.party.files_written,
                    s.party.given_back,
                    s.rester.passed,
                    s.rester.hours
                )
            }
            Role::Guest(Some(g)) => {
                let s = &g.social;
                format!(
                    "co-op guest social: conversations {} (refused {}, not started {}), results held for the host {held}, run here as own {own}, with no quest known {unknown}, lines watched {}, part sent {} time(s), put back {}, rests passed {} ({} h)",
                    s.talker.opened,
                    s.talker.denied,
                    s.talker.blocked,
                    s.talker.lines_shown,
                    s.party.uploads,
                    s.party.restored,
                    s.rester.passed,
                    s.rester.hours
                )
            }
            Role::Guest(None) => "co-op guest social: waiting for the host".into(),
        }
    }

    /// A one-line status: players and the story's revision on a host; on a guest what it applied,
    /// the hash checks, drift healed, refusals and the story scripts it did not run.
    fn status(&self, e: &mut Engine) -> String {
        match &self.role {
            Role::Host(h) => format!(
                "co-op host: {} guest(s), story revision {}, not sendable {}, guests' deaths enacted {} (left {}); {}",
                h.sync.players().len(),
                h.sync.store.rev().0,
                h.unencodable,
                h.claims_enacted,
                h.claims_dropped,
                h.ladder.as_ref().map_or("live: off".into(), Ladder::status)
            ),
            Role::Guest(Some(g)) => {
                let (applying, story) = coop_engine::gate::skipped(e);
                let drift = session::echo(e, &g.sync.store).len();
                format!(
                    "co-op guest: story revision {}, hash checks {} (desyncs {}), game differs from the host's story on {drift} entries, healed {}, refused {} (again on heal {}), story scripts not run {story}, result scripts not rerun {applying}",
                    g.sync.store.rev().0,
                    g.sync.hash_checks,
                    g.sync.desyncs_detected,
                    g.healed,
                    g.refused,
                    g.heal_refused,
                ) + &format!(
                    ", own quest scripts run {}, objectives put back in the Pip-Boy {}, deaths told to the host {}",
                    coop_engine::gate::ran_own(e),
                    g.relisted,
                    g.claims_sent
                )
                    + "; "
                    + &g.ladder.as_ref().map_or("live: off".into(), Ladder::status)
            }
            Role::Guest(None) => "co-op guest: waiting for the host".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_join_address_is_taken_only_from_a_real_address() {
        assert_eq!(
            address_from(Some(" 203.0.113.7 ")),
            Some(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 7)))
        );
        assert_eq!(address_from(Some("")), None);
        assert_eq!(address_from(Some("my-pc")), None);
        assert_eq!(address_from(Some("0.0.0.0")), None);
        assert_eq!(address_from(None), None);
    }

    #[test]
    fn the_opening_quests_are_each_player_s_own_in_any_load_order() {
        let plain = own_quest_ids(&["FalloutNV.esm".into()]);
        // VCG00, VCG01, CGTutorial: the opening and character creation.
        for id in [0x0010_2037, 0x0010_4c1c, 0x0005_9c85] {
            assert!(plain.contains(&id), "{id:08x}");
        }
        // A shared story quest is not among them: the first quest after the opening.
        let table = coop_policy::PolicyTable::bundled().unwrap();
        let shared = table.row_by_edid("VCG03").expect("VCG03 is in the rules");
        assert_eq!(shared.scope, coop_policy::Scope::Shared);
        assert!(!plain.contains(&shared.local));
        // A DLC's rows take that plugin's place in the load order; an unloaded plugin's are skipped.
        let with_dlc = own_quest_ids(&[
            "FalloutNV.esm".into(),
            "HonestHearts.esm".into(),
            "deadmoney.esm".into(),
        ]);
        assert!(with_dlc.len() > plain.len());
        assert!(with_dlc.iter().any(|id| id >> 24 == 2));
        assert!(with_dlc.iter().all(|id| id >> 24 <= 2));
    }

    /// PC run 4, 2026-11-10: the guest got none of the pre-order packs' items and its opening
    /// quest never completed. The quests that hand out the packs and the generic timer that ends
    /// the opening act on the player whose game runs them.
    #[test]
    fn the_pack_quests_and_the_timer_are_each_player_s_own() {
        let own = own_quest_ids(&["FalloutNV.esm".into()]);
        // VDLCPackQuest, VPreorderMercenary, Caravan, Tribal, Classic, VGenericTimer.
        for id in [
            0x0017_912d,
            0x0017_2a1d,
            0x0017_2a1e,
            0x0017_2a1f,
            0x0016_74d4,
            0x0016_8a86,
        ] {
            assert!(own.contains(&id), "{id:08x}");
        }
        let scripts = own_script_ids(&["FalloutNV.esm".into(), "DeadMoney.esm".into()]);
        // The packs' scripts, and a DLC's by its place in the load order.
        for id in [0x0017_2a24, 0x0017_2a27, 0x0017_912e, 0x0100_62b1] {
            assert!(scripts.contains(&id), "{id:08x}");
        }
        assert!(scripts.iter().all(|id| id >> 24 <= 1));
    }

    #[test]
    fn the_clock_globals_are_left_out() {
        for id in [
            "GameHour",
            "gamedayspassed",
            "GameDay",
            "GameMonth",
            "GameYear",
        ] {
            assert!(is_clock_global(id), "{id}");
        }
        for id in ["TimeScale", "GameDaysPassedStart", "PlayerKarma"] {
            assert!(!is_clock_global(id), "{id}");
        }
    }

    #[test]
    fn nv_coop_values() {
        assert_eq!(parse(""), Ok(None));
        assert_eq!(parse("0"), Ok(None));
        assert_eq!(parse("journal"), Ok(None));
        assert_eq!(parse("host"), Ok(Some(Ask::Host(DEFAULT_PORT))));
        assert_eq!(parse("host:4000"), Ok(Some(Ask::Host(4000))));
        assert!(parse("host:0").is_err());
        assert!(parse("host:x").is_err());
        assert!(parse("join:nonsense").is_err());
        assert!(parse("play").is_err());
        let code = JoinCode::new(
            JoinTarget::Direct(SocketAddr::from(([127, 0, 0, 1], 4000))),
            [7; 16],
        )
        .to_string();
        assert_eq!(
            parse(&format!("join:{code}")),
            Ok(Some(Ask::Join(code.clone())))
        );
    }
}

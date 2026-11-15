//! Co-op in nv-viewer (docs/coop/PLAN.md M3): `NV_COOP=host` (or `host:PORT`) hosts the story,
//! `NV_COOP=join:CODE` follows a host's story with the join code the host printed. Between two of
//! the game's frames the session takes the network's messages and runs its step
//! (`coop_engine_session`): the host sends its story's changes, a guest applies them to its game
//! with the story's scripts gated. `NV_COOP=journal` is the change journal's log
//! (`coop_engine::diag`), not a session.
//! `NV_COOP=host:local` and `join:local` (split) are a session of copies on one PC (Couchlink's
//! split-screen seats): the host listens on the loopback address with a fixed token, so the
//! others join without a code; `local:SEAT` is the first for seat 0 and the second for the others.
//!
//! The live ladder (docs/coop/DECISIONS.md D3) runs with a session: `NV_COOP_LIVE=log` reads and
//! sends the players' places only, `warp` also moves a guest to the host's cell, `avatar` (the
//! default) also shows the other players; `off` leaves the session story-only.
//! `NV_COOP_AVATAR=<hex form id>` is the base form of an avatar (default 7, the player's own).
//! `NV_COOP_WORLD=doors` shares doors and locks but no changed-form records, `off` neither
//! (docs/coop/SPECS.md S14).
//!
//! Shared fights (fights; docs/coop/SPECS.md S15) run with the ladder: `NV_COOP_FIGHT=state` shows
//! the host's actors on the guests, `damage` also sends hits through the host, `threat` also lets
//! the host's actors turn on a guest, `revive` (the default) also holds a player's death for a
//! revive; `off` leaves every game running its own fights.
//!
//! Merged from another line of work and off unless asked, each switched on for one PC run at a
//! time: `NV_COOP_VATS=on` (or `slow`) for V.A.T.S. with several players (S21), `NV_COOP_HARM=on`
//! for damage outside `Actor::HitMe` decided as hits are (S22, with fights shared up to damage),
//! `NV_COOP_SMOOTH=on` for held actors moved in steps (S22), `NV_COOP_ITEMS=on` for loot, pick-ups
//! and barter through the host (S20).
//!
//! Off unless asked too: `NV_COOP_LOOK=walk` has the other players' avatars walk and run with the
//! game's animations, `pose` also holds, draws and fires their weapons, `gear` also has them wear
//! their players' items, `face` also gives them their players' faces, bodies and names (S32).
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
    // split
    /// `host:local`: host for the other copies on this PC only ([`LOCAL_TOKEN`]).
    HostLocal,
}

// split
/// The token of a session on one PC (`NV_COOP=host:local` and `join:local`, Couchlink's
/// split-screen seats): fixed, so every copy knows it without a join code. Such a host listens
/// on the loopback address only, so nothing outside this PC can reach it.
pub const LOCAL_TOKEN: [u8; 16] = *b"couch-on-this-pc";

// split
/// `local` after `host:` or `join:`: the session of Couchlink's seats on this PC. `local:SEAT`
/// (Couchlink's `$SEAT`, the same setting for every copy) is `host:local` for seat 0 and
/// `join:local` for the others.
fn local_ask(v: &str) -> Option<Ask> {
    let seat = v
        .strip_prefix("local:")
        .and_then(|s| s.trim().parse::<u8>().ok());
    match (v, seat) {
        ("host:local", _) | (_, Some(0)) => Some(Ask::HostLocal),
        ("join:local", _) | (_, Some(_)) => Some(Ask::Join(
            JoinCode::new(
                JoinTarget::Direct(SocketAddr::from(([127, 0, 0, 1], DEFAULT_PORT))),
                LOCAL_TOKEN,
            )
            .to_string(),
        )),
        _ => None,
    }
}

// split
/// Where a host listens and the token and address of its join code: every address with a new
/// token, or for `host:local` the loopback address with [`LOCAL_TOKEN`].
fn host_socket(ask: &Ask) -> (u16, SocketAddr, coop_net::Token, IpAddr) {
    match ask {
        Ask::HostLocal => {
            let lo = IpAddr::V4(Ipv4Addr::LOCALHOST);
            (
                DEFAULT_PORT,
                SocketAddr::new(lo, DEFAULT_PORT),
                LOCAL_TOKEN,
                lo,
            )
        }
        Ask::Host(port) => (
            *port,
            SocketAddr::from(([0, 0, 0, 0], *port)),
            random_token(),
            join_ip(),
        ),
        Ask::Join(_) => unreachable!("a guest does not host"),
    }
}

/// `NV_COOP`'s value as a session request; `None` for no session (unset, `0`, `journal`).
pub fn parse(v: &str) -> Result<Option<Ask>, String> {
    let v = v.trim();
    if v.is_empty() || v == "0" || v == "journal" {
        return Ok(None);
    }
    // split
    if let Some(ask) = local_ask(v) {
        return Ok(Some(ask));
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
        "NV_COOP={v}: use host, host:PORT, host:local, join:CODE, join:local, local:SEAT or journal"
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
    /// The quests each object script names (its `SCRO` references that are quests).
    object_script_quests: std::collections::BTreeMap<u32, Vec<u32>>,
}

/// For every object script (`SCHR` type 0) of the load order, the quests it names.
fn object_script_quests(order: &esm::LoadOrder) -> std::collections::BTreeMap<u32, Vec<u32>> {
    let quests: std::collections::BTreeSet<u32> = order
        .records_of_type(esm::FourCC::new(b"QUST"))
        .map(|r| r.form_id.0)
        .collect();
    let mut out = std::collections::BTreeMap::new();
    for r in order.records_of_type(esm::FourCC::new(b"SCPT")) {
        let Ok(rec) = r.record() else { continue };
        let object = rec
            .get_all(esm::FourCC::new(b"SCHR"))
            .next()
            .and_then(|h| h.data.get(16..18).map(|t| u16::from_le_bytes([t[0], t[1]])))
            == Some(0);
        if !object {
            continue;
        }
        let named: Vec<u32> = rec
            .get_all(esm::FourCC::new(b"SCRO"))
            .filter_map(|s| {
                s.data
                    .get(..4)
                    .map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
            })
            .map(|local| r.plugin.to_global(esm::FormId(local)).0)
            .filter(|id| quests.contains(id))
            .collect();
        if !named.is_empty() {
            out.insert(r.form_id.0, named);
        }
    }
    out
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

/// The quest each player plays as its own opening, and the stage that ends it: VCG01, "Ain't
/// That a Kick in the Head", stage 200 (set by the generic timer after Doc Mitchell's last line).
/// Until a guest's own game has done it, that guest holds none of the host's actors.
fn opening_quest(plugins: &[String]) -> Option<(u32, u8)> {
    let table = coop_policy::PolicyTable::bundled().ok()?;
    let row = table.rows().iter().find(|r| r.edid == "VCG01")?;
    let index = plugins
        .iter()
        .position(|p| p.eq_ignore_ascii_case(&row.plugin))?;
    Some((((index as u32) << 24) | (row.local & 0x00ff_ffff), 200))
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
    let object_script_quests = object_script_quests(&order);
    Ok(GameData {
        object_script_quests,
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
    object_script_quests: std::collections::BTreeMap<u32, Vec<u32>>,
    scenes_set: bool,
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
        // look: `NV_COOP_LOOK=walk|pose|gear|face` shows the other players' avatars as those
        // players (docs/coop/SPECS.md S35); off unless asked (not yet run on a PC).
        let look = session::look::Cap::parse(&std::env::var("NV_COOP_LOOK").unwrap_or_default());
        let mut ladder = Ladder::new(guest, cap, base);
        ladder.look = look.map(session::look::Look::new);
        (
            Some(ladder),
            format!(
                "co-op live: ladder up to {cap:?}, avatar base form {base:08x}, look {}",
                look.map_or(
                    "off (NV_COOP_LOOK=walk, pose, gear or face)".to_string(),
                    |c| { format!("{c:?}") }
                )
            ),
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

    // items
    /// Whether loot and barter go through the host's game (docs/coop/SPECS.md S20): off unless
    /// `NV_COOP_ITEMS` is `on` (not yet run on a PC).
    fn items_on() -> bool {
        Self::switch_on("NV_COOP_ITEMS")
    }

    /// Whether the switch `name` is `on` (or `1`).
    fn switch_on(name: &str) -> bool {
        matches!(std::env::var(name).unwrap_or_default().trim(), "on" | "1")
    }

    // items
    fn items_line() -> &'static str {
        if Self::items_on() {
            "co-op items: loot, pick-ups and barter through the host's game"
        } else {
            "co-op items: off (NV_COOP_ITEMS=on puts loot, pick-ups and barter through the host's game)"
        }
    }

    // world
    /// What `NV_COOP_WORLD` asks to share of the world beyond the story: `all`, `doors`, or
    /// nothing. Unset is nothing for now: none of it has run on a PC, and the parts are being
    /// switched on one PC run at a time (docs/coop/PLAN.md).
    fn world_share() -> session::world::Share {
        match std::env::var("NV_COOP_WORLD") {
            Ok(v) if !v.trim().is_empty() => session::world::Share::parse(&v),
            _ => session::world::Share::Off,
        }
    }

    /// Whether conversations between actors are the host's (`NV_COOP_SCENES=off` gives every
    /// game its own again). Returns the line that says so.
    fn scenes_on() -> (bool, String) {
        let on = std::env::var("NV_COOP_SCENES").map_or(true, |v| v.trim() != "off");
        let line = if on {
            "co-op scenes: the host's conversations between actors play on the guests (NV_COOP_SCENES=off: each game its own)"
        } else {
            "co-op scenes: off (each game runs its own)"
        };
        (on, line.into())
    }

    /// Whether conversations, party saves and the rest vote are shared: `NV_COOP_TALK=on`. Off
    /// unless asked for, for the same reason. Returns the line that says so.
    fn social_on() -> (bool, String) {
        let on = std::env::var("NV_COOP_TALK").is_ok_and(|v| v.trim() == "on");
        let line = if on {
            "co-op talk, party saves, rest vote: shared (NV_COOP_TALK=on)"
        } else {
            "co-op talk, party saves, rest vote: off (NV_COOP_TALK=on shares them)"
        };
        (on, line.into())
    }

    /// fights: the shared fights `NV_COOP_FIGHT` asks for; `None` for `off` or without the ladder.
    /// Returns the line that says so.
    fn fight(guest: bool, ladder: bool) -> (Option<session::fight::Fight>, String) {
        // Unset is `follow`: the host's actors keep their own AI on a guest and are put at the
        // host's place when they have strayed (PC run 10: held with their AI off they kept the
        // pose they had, in floors and ceilings). `state` and up hold them as before; `off`.
        let ask = match std::env::var("NV_COOP_FIGHT") {
            Ok(v) if !v.trim().is_empty() => v,
            _ => "follow".into(),
        };
        if ask.trim() == "off" || !ladder {
            return (None, "co-op fights: off".into());
        }
        // `follow`: all of following. `follow-plain`: only putting a strayed actor back (no
        // leading, no bringing through doors, no packages). `NV_COOP_PACKAGES=off` turns the
        // packages alone off. Each part has its switch so a fault can be traced to one.
        // `follow-damage`, `follow-threat`, `follow-full`: following, with the fights shared up
        // to that level (hits through the host; the host's actors turning on a guest; downed and
        // revive). Not yet run on a PC, so not the default.
        let shared = match ask.trim() {
            "follow-damage" => Some(session::fight::Cap::Damage),
            "follow-threat" => Some(session::fight::Cap::Threat),
            "follow-full" => Some(session::fight::Cap::Revive),
            _ => None,
        };
        if ask.trim() == "follow" || ask.trim() == "follow-plain" || shared.is_some() {
            let plain = ask.trim() == "follow-plain";
            let packages =
                !plain && std::env::var("NV_COOP_PACKAGES").map_or(true, |v| v.trim() != "off");
            let cap = shared.unwrap_or(session::fight::Cap::State);
            let mut fight = session::fight::Fight::new(guest, cap);
            fight.follow = true;
            // Hand-off: the nearest player's game runs an actor (NV_COOP_HANDOFF=off: the
            // host's game runs every one).
            let handoff = std::env::var("NV_COOP_HANDOFF").map_or(true, |v| v.trim() != "off");
            fight.handoff = handoff;
            fight.lead_on = !plain;
            fight.bring_on = !plain;
            fight.packages_on = packages;
            let extra = Self::fight_extras(&mut fight);
            return (
                Some(fight),
                format!(
                    "co-op fights: the host's actors are followed, fights shared up to {cap:?} (leading {}, through doors {}, the host's packages {}, the nearest player's game runs an actor {}; NV_COOP_FIGHT=off turns it all off); {extra}",
                    if plain { "off" } else { "on" },
                    if plain { "off" } else { "on" },
                    if packages { "on" } else { "off" },
                    if handoff { "on" } else { "off" }
                ),
            );
        }
        let cap = session::fight::Cap::parse(&ask);
        let mut fight = session::fight::Fight::new(guest, cap);
        let extra = Self::fight_extras(&mut fight);
        (
            Some(fight),
            format!("co-op fights: shared up to {cap:?}; {extra}"),
        )
    }

    /// vats: the parts of shared fights merged from another line of work, each off unless asked
    /// (one PC run at a time). Returns the words that say so and how to switch each on.
    fn fight_extras(fight: &mut session::fight::Fight) -> String {
        // `NV_COOP_VATS=on`: V.A.T.S. neither stops nor slows a world other players are in;
        // `slow`: only the playback does not slow it (docs/coop/SPECS.md S21).
        let vats = session::vats::Mode::parse(&std::env::var("NV_COOP_VATS").unwrap_or_default());
        fight.vats = session::vats::Share::new(vats);
        // `NV_COOP_HARM=on`: a fall, a trap or an effect is decided as a hit is (S22; with fights
        // shared up to damage).
        fight.harm_on = Self::switch_on("NV_COOP_HARM");
        // `NV_COOP_SMOOTH=on`: a held actor moves between two states in steps (S22).
        fight.smooth_on = Self::switch_on("NV_COOP_SMOOTH");
        let word = |on: bool| if on { "on" } else { "off" };
        format!(
            "V.A.T.S. {vats:?} (NV_COOP_VATS=on or slow), damage outside hits {} (NV_COOP_HARM=on), held actors in steps {} (NV_COOP_SMOOTH=on)",
            word(fight.harm_on),
            word(fight.smooth_on)
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
            object_script_quests,
        } = game_data(data)?;
        let cfg = EndpointConfig {
            name: name.to_string(),
            ..EndpointConfig::default()
        };
        let mut lines = Vec::new();
        let (ep, role) = match ask {
            Ask::Host(_) | Ask::HostLocal => {
                // split: `host:local` listens on this PC only, with the local token.
                let (port, bind, token, ip) = host_socket(ask);
                let b = UdpBackend::host(bind, token, UdpConfig::default())
                    .map_err(|e| format!("co-op: UDP port {port}: {e}"))?;
                let code = JoinCode::new(JoinTarget::Direct(SocketAddr::new(ip, port)), token);
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
                // items
                lines.push(Self::items_line().into());
                if Self::items_on() {
                    host.items = Some(session::items::HostItems::new());
                }
                // fights
                let (fight, line) = Self::fight(false, host.ladder.is_some());
                host.fight = fight;
                lines.push(line);
                let (on, line) = Self::social_on();
                host.social_on = on;
                // Each player's opening is its own, its conversations too (Doc Mitchell's).
                host.social.talker.opening = opening_quest(&plugins);
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
                object_script_quests,
                scenes_set: false,
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
        if !self.scenes_set {
            self.scenes_set = true;
            let (on, line) = Self::scenes_on();
            coop_engine::commands::set_scenes_on(e, on);
            lines.push(line);
        }
        lines.extend(coop_engine::commands::drain_notes(e));
        self.ep.pump();
        for ev in self.ep.events() {
            match ev {
                NetEvent::Connected { local } => {
                    lines.push(format!("co-op: joined as player {}", local.0));
                    match &mut self.role {
                        Role::Guest(slot @ None) => {
                            let mut g =
                                GuestDriver::new(local, session::hello(self.plugins.clone()));
                            let (mut ladder, line) = Self::ladder(true);
                            if let Some(l) = ladder.as_mut() {
                                l.follow_after = opening_quest(&self.plugins);
                            }
                            g.ladder = ladder;
                            lines.push(line);
                            // world
                            let share = Self::world_share();
                            lines.push(share.line().into());
                            g.world = share.guest();
                            // items
                            lines.push(Self::items_line().into());
                            if Self::items_on() {
                                g.items = Some(session::items::GuestItems::new());
                            }
                            // fights
                            let (mut fight, line) = Self::fight(true, g.ladder.is_some());
                            if let Some(f) = fight.as_mut() {
                                f.hold_after = opening_quest(&self.plugins);
                            }
                            g.fight = fight;
                            lines.push(line);
                            g.proposals_on = std::env::var("NV_COOP_PROPOSE")
                                .map_or(true, |v| v.trim() != "off");
                            g.object_script_quests = self.object_script_quests.clone();
                            lines.push(format!(
                                "co-op proposals: what this game's own object scripts do to the story is {} (NV_COOP_PROPOSE=off puts it back as before)",
                                if g.proposals_on { "proposed to the host" } else { "put back" }
                            ));
                            let (on, line) = Self::social_on();
                            g.social_on = on;
                            lines.push(line);
                            // social
                            g.social.party.key = session::party::key_of(&self.social.name);
                            g.social.party.own_quests = self.social.own_quests.clone();
                            g.social.talker.opening = opening_quest(&self.plugins);
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
                for note in h.stage_notes.drain(..) {
                    lines.push(format!("co-op host: {note}"));
                }
                // world
                for note in h.world.iter_mut().flat_map(|w| w.notes.drain(..)) {
                    lines.push(format!("co-op host world: {note}"));
                }
                // items
                for note in h.items.iter_mut().flat_map(|i| i.notes.drain(..)) {
                    lines.push(format!("co-op host items: {note}"));
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
                    lines.push(note);
                }
                // world
                for note in g.world.iter_mut().flat_map(|w| w.notes.drain(..)) {
                    lines.push(format!("co-op guest world: {note}"));
                }
                // items
                for note in g.items.iter_mut().flat_map(|i| i.notes.drain(..)) {
                    lines.push(format!("co-op guest items: {note}"));
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
            // items
            for at in coop_engine::items::missing(e) {
                lines.push(format!(
                    "co-op: no function at {at:08x} to take part in; that part of loot or barter is off"
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
                    "co-op host social: conversations {} (guests' results run {}, left {}, greetings left to the owner's game {}), lines watched {}, party saves {} ({} files), parts given back {}, rests passed {} ({} h)",
                    s.talker.opened,
                    s.enactor.enacted,
                    s.enactor.left,
                    s.talker.not_greeted,
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
                    "co-op guest social: conversations {} (refused {}, not started {}, greetings left to the owner's game {}), results held for the host {held}, run here as own {own}, with no quest known {unknown}, lines watched {}, part sent {} time(s), put back {}, rests passed {} ({} h)",
                    s.talker.opened,
                    s.talker.denied,
                    s.talker.blocked,
                    s.talker.not_greeted,
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
                    ", own quest scripts run {}, objectives put back in the Pip-Boy {}, deaths told to the host {}, drifts with a named cause {} (unknown {}), the host's scenes played {} (not played {}), own scenes between actors dropped {}",
                    coop_engine::gate::ran_own(e),
                    g.relisted,
                    g.claims_sent,
                    g.drift_named,
                    g.drift_unnamed,
                    g.scenes_played,
                    g.scenes_missed,
                    coop_engine::commands::dropped_scenes(e)
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
    fn the_opening_quest_is_vcg01_to_its_last_stage() {
        assert_eq!(
            opening_quest(&["FalloutNV.esm".into()]),
            Some((0x0010_4c1c, 200))
        );
        assert_eq!(opening_quest(&["Other.esm".into()]), None);
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

    // split
    #[test]
    fn copies_on_one_pc_meet_on_the_loopback_address_without_a_code() {
        assert_eq!(parse("host:local"), Ok(Some(Ask::HostLocal)));
        assert_eq!(parse("local:0"), Ok(Some(Ask::HostLocal)));
        assert_eq!(parse("local:3"), parse("join:local"));
        assert!(parse("local:x").is_err());
        let Ok(Some(Ask::Join(code))) = parse(" join:local ") else {
            panic!("join:local is a join");
        };
        let code: JoinCode = code.parse().unwrap();
        let lo = SocketAddr::from(([127, 0, 0, 1], DEFAULT_PORT));
        assert_eq!(code.target, JoinTarget::Direct(lo));
        assert_eq!(code.token, LOCAL_TOKEN);
        let (port, bind, token, ip) = host_socket(&Ask::HostLocal);
        assert_eq!(
            (port, bind, token, ip),
            (DEFAULT_PORT, lo, LOCAL_TOKEN, lo.ip())
        );
        // A host for the network keeps listening everywhere with a token of its own.
        let (port, bind, token, _) = host_socket(&Ask::Host(4000));
        assert_eq!((port, bind.ip().is_unspecified()), (4000, true));
        assert_ne!(token, LOCAL_TOKEN);
    }
}

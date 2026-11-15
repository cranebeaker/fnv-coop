//! steam: Steam invites for a co-op session (docs/coop/SPECS.md S37), behind the cargo feature
//! `steam` (on by default).
//!
//! - A host with `NV_COOP_STEAM=on` (or `invite`) makes a friends-only Steam lobby on Spacewar
//!   (AppID 480) beside its UDP port: friends join through Steam networking, with no port
//!   forwarded, and the UDP join code works as before for anyone without Steam
//!   (`coop_net::Dual`). `invite` also opens the Steam overlay's invite dialog once this player is
//!   in the world; `NV_COOP_INVITE_KEY` (default F6) opens it again at any time.
//! - A guest with `NV_COOP=steam` joins the lobby of a Steam friend who hosts, or of the first
//!   invite that arrives (from the overlay's dialog or the friends list's "Invite to game"),
//!   waiting up to two minutes; a join code that names a Steam lobby joins that lobby.
//!
//! Valve's `steam_api64.dll` is not in the source: it is looked for at `NV_COOP_STEAM_DLL` (the
//! launcher sets it to the one next to `Play.exe`), then next to this program, then in the folder
//! above (the package's). Missing, the log says so plainly and the session goes on over UDP.
//!
//! The overlay draws in the game's window only when Steam's overlay library is in the process
//! before the game makes its Direct3D 9 device, so Steam is started by [`early`], before the host
//! starts the game. Its key (Shift+Tab unless the player changed it in Steam) reaches it through
//! the window's own messages (`engine_platform::d3d9real`'s window thread). None of this has run
//! against real Steam in this engine.

use coop_engine_session::voice::{dik_of, key_name, Press};
use coop_net::{Backend, JoinCode, Token, UdpBackend};
use engine_core::Engine;

/// What `NV_COOP_STEAM` asks of a host.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HostSteam {
    /// UDP only (the default).
    Off,
    /// A Steam lobby beside the UDP port.
    On,
    /// The lobby, and the overlay's invite dialog opened once in the world.
    Invite,
}

impl HostSteam {
    /// `on` (or `1`), `invite`; anything else is off.
    pub fn parse(v: &str) -> HostSteam {
        match v.trim().to_ascii_lowercase().as_str() {
            "on" | "1" => HostSteam::On,
            "invite" => HostSteam::Invite,
            _ => HostSteam::Off,
        }
    }

    fn from_env() -> HostSteam {
        HostSteam::parse(&std::env::var("NV_COOP_STEAM").unwrap_or_default())
    }
}

/// The key that opens the invite dialog: `NV_COOP_INVITE_KEY`, default F6.
fn invite_key() -> u8 {
    std::env::var("NV_COOP_INVITE_KEY")
        .ok()
        .and_then(|v| dik_of(&v))
        .unwrap_or(0x40)
}

/// What a running session does with Steam each frame: the invite key, the dialog asked for at the
/// start, and what Steam reports.
pub struct SteamSide {
    #[cfg(feature = "steam")]
    handle: coop_net::steam::SteamHandle,
    host: bool,
    key: u8,
    press: Press,
    open_once: bool,
    in_world_since: Option<std::time::Instant>,
    invites_seen: u32,
}

impl SteamSide {
    /// One frame: the key, the first dialog, invites that reached this game.
    pub fn frame(&mut self, e: &mut Engine, lines: &mut Vec<String>) {
        #[cfg(windows)]
        let down = engine_platform::d3d9real::held_keys()[self.key as usize];
        #[cfg(not(windows))]
        let down = false;
        let pressed = self.press.pressed(down);
        let in_world = coop_engine::live::local(e).is_some_and(|l| l.loaded);
        if in_world && self.in_world_since.is_none() {
            self.in_world_since = Some(std::time::Instant::now());
        }
        let first = self.open_once
            && self
                .in_world_since
                .is_some_and(|t| t.elapsed() >= std::time::Duration::from_secs(3));
        if self.host && (pressed || first) {
            self.open_once = false;
            self.open_dialog(e, lines);
        } else if pressed {
            coop_engine::talk::show(e, "Steam invites: only the host invites");
        }
        self.take_invites(lines);
    }

    #[cfg(feature = "steam")]
    fn open_dialog(&mut self, e: &mut Engine, lines: &mut Vec<String>) {
        self.handle.open_overlay();
        coop_engine::talk::show(
            e,
            "Steam: invite a friend in the overlay (if it does not show, use your Steam friends list)",
        );
        lines.push(
            "co-op steam: asked Steam to open the overlay's invite dialog (a friend can also be invited from the Steam friends list: right-click, Invite to game)".into(),
        );
    }

    #[cfg(not(feature = "steam"))]
    fn open_dialog(&mut self, _e: &mut Engine, _lines: &mut Vec<String>) {}

    #[cfg(feature = "steam")]
    fn take_invites(&mut self, lines: &mut Vec<String>) {
        for inv in self.handle.take_invites() {
            self.invites_seen += 1;
            lines.push(format!(
                "co-op steam: an invite to lobby {} arrived; this game is already in a session (start again with Join a Steam friend to follow it)",
                inv.lobby
            ));
        }
    }

    #[cfg(not(feature = "steam"))]
    fn take_invites(&mut self, _lines: &mut Vec<String>) {}

    /// One line for the status.
    pub fn status(&self) -> String {
        #[cfg(feature = "steam")]
        let state = format!(
            "lobby {:?}, phase {:?}, friends online {}, invite dialogs asked {}",
            self.handle.lobby(),
            self.handle.phase(),
            self.handle.friends().len(),
            self.handle.overlays_opened()
        );
        #[cfg(not(feature = "steam"))]
        let state = String::from("not built");
        format!(
            "co-op steam: {state}, invites that arrived {} ({} opens the invite dialog)",
            self.invites_seen,
            key_name(self.key)
        )
    }
}

#[cfg(feature = "steam")]
mod real {
    use std::cell::RefCell;

    use coop_net::steam::{self, link, RealSteam};

    thread_local! {
        /// Steam, started before the game made its window ([`super::early`]).
        pub static STEAM: RefCell<Option<Result<RealSteam, String>>> = const { RefCell::new(None) };
        /// A guest's lobby, found before the game's frames began.
        pub static JOINED: RefCell<Option<Result<link::Joined<RealSteam>, String>>> = const { RefCell::new(None) };
    }

    /// Loads `steam_api64.dll` and starts Steam; the error in the player's words.
    pub fn start() -> Result<RealSteam, String> {
        let explicit = std::env::var_os("NV_COOP_STEAM_DLL").map(std::path::PathBuf::from);
        let exe = std::env::current_exe().ok();
        let places = link::dll_places(explicit.as_deref(), exe.as_deref());
        let dll = link::find_dll(&places, |p| p.is_file())?;
        steam::try_init(Some(&dll)).map_err(|e| match e {
            steam::SteamError::NotRunning => {
                "Steam is not running: start Steam and sign in, then start the game again".into()
            }
            other => other.to_string(),
        })
    }

    /// Takes the Steam started early (or starts it now).
    pub fn take() -> Result<RealSteam, String> {
        STEAM.with(|s| s.borrow_mut().take()).unwrap_or_else(start)
    }
}

/// Whether the session `ask` asks for uses Steam.
fn wants_steam(ask: &crate::coop::Ask) -> bool {
    match ask {
        crate::coop::Ask::Host(_) => HostSteam::from_env() != HostSteam::Off,
        crate::coop::Ask::SteamFriend => true,
        // split: copies on one PC meet on the loopback address, never through Steam.
        crate::coop::Ask::HostLocal => false,
        crate::coop::Ask::Join(code) => code
            .parse::<JoinCode>()
            .is_ok_and(|c| matches!(c.target, coop_net::JoinTarget::SteamLobby(_))),
    }
}

/// Before the host starts the game: starts Steam when the session will use it, so its overlay is
/// in the process before the game's device is made; a guest with no code finds its friend's lobby
/// here (it may wait). Returns the lines to print.
pub fn early(ask: &crate::coop::Ask) -> Vec<String> {
    if !wants_steam(ask) {
        return Vec::new();
    }
    #[cfg(feature = "steam")]
    {
        let mut lines = Vec::new();
        let started = real::start();
        match &started {
            Ok(s) => lines.push(format!(
                "co-op steam: Steam is running (signed in; Spacewar, AppID 480; user {:016x})",
                coop_net::steam::SteamApi::self_id(s)
            )),
            Err(why) => lines.push(format!("co-op steam: {why}")),
        }
        if *ask == crate::coop::Ask::SteamFriend {
            if let Ok(api) = started {
                println!("co-op steam: looking for a Steam friend who hosts, or an invite (up to 2 minutes)...");
                let joined = coop_net::steam::link::guest(
                    api,
                    coop_net::steam::GuestTarget::Friend,
                    &coop_net::steam::Setup::default(),
                );
                if let Ok(j) = &joined {
                    lines.push(format!(
                        "co-op steam: found lobby {} (by {})",
                        j.lobby, j.via
                    ));
                }
                real::JOINED.with(|s| *s.borrow_mut() = Some(joined));
                return lines;
            }
        }
        real::STEAM.with(|s| *s.borrow_mut() = Some(started));
        lines
    }
    #[cfg(not(feature = "steam"))]
    {
        vec![
            "co-op steam: not built into this program (cargo feature steam); the join code works"
                .into(),
        ]
    }
}

/// A host's backend: UDP, and the Steam lobby beside it when asked for and Steam is there.
pub fn host_backend(
    udp: UdpBackend,
    token: Token,
    lines: &mut Vec<String>,
) -> (Box<dyn Backend>, Option<SteamSide>) {
    let mode = HostSteam::from_env();
    if mode == HostSteam::Off {
        lines.push("co-op steam: off (NV_COOP_STEAM=on makes a Steam lobby for invites)".into());
        return (Box::new(udp), None);
    }
    #[cfg(feature = "steam")]
    {
        let hosted = real::take().and_then(|api| {
            coop_net::steam::link::host(api, token, &coop_net::steam::Setup::default())
        });
        match hosted {
            Ok(h) => {
                lines.push(format!(
                    "co-op steam: hosting a friends-only Steam lobby {}; Steam join code: {} (friends join through Steam, no port to forward)",
                    h.lobby, h.code
                ));
                let side = SteamSide {
                    handle: h.handle,
                    host: true,
                    key: invite_key(),
                    press: Press::default(),
                    open_once: mode == HostSteam::Invite,
                    in_world_since: None,
                    invites_seen: 0,
                };
                lines.push(format!(
                    "co-op steam: {} opens the Steam overlay's invite dialog",
                    key_name(side.key)
                ));
                (
                    Box::new(coop_net::Dual::new(udp, h.backend)) as Box<dyn Backend>,
                    Some(side),
                )
            }
            Err(why) => {
                lines.push(format!(
                    "co-op steam: no Steam lobby ({why}); hosting over UDP only, the join code works"
                ));
                (Box::new(udp), None)
            }
        }
    }
    #[cfg(not(feature = "steam"))]
    {
        let _ = token;
        lines.push("co-op steam: not built into this program; hosting over UDP only".into());
        (Box::new(udp), None)
    }
}

/// A guest's Steam backend and the session's token: the lobby of `code` (a Steam join code), or
/// with `None` the friend's lobby [`early`] found.
pub fn guest_backend(
    code: Option<JoinCode>,
    lines: &mut Vec<String>,
) -> Result<(Box<dyn Backend>, Token, SteamSide), String> {
    #[cfg(feature = "steam")]
    {
        let joined = match code {
            Some(c) => real::take().and_then(|api| {
                coop_net::steam::link::guest(
                    api,
                    coop_net::steam::GuestTarget::Code(c),
                    &coop_net::steam::Setup::default(),
                )
            }),
            None => real::JOINED
                .with(|s| s.borrow_mut().take())
                .unwrap_or_else(|| Err("Steam was not started".into())),
        }
        .map_err(|why| format!("co-op steam: {why}"))?;
        lines.push(format!(
            "co-op steam: joining the Steam lobby {} (found by {}); no port to forward",
            joined.lobby, joined.via
        ));
        let side = SteamSide {
            handle: joined.handle,
            host: false,
            key: invite_key(),
            press: Press::default(),
            open_once: false,
            in_world_since: None,
            invites_seen: 0,
        };
        Ok((Box::new(joined.backend), joined.token, side))
    }
    #[cfg(not(feature = "steam"))]
    {
        let _ = (code, lines);
        Err("co-op: this program has no Steam (cargo feature steam): ask the host for a UDP join code".into())
    }
}

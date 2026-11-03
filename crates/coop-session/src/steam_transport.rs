//! Steam (Spacewar, AppID 480) transport for the session. Cargo feature `steam`, off by default.
//!
//! Builds a [`coop_net::steam::SteamBackend`] host or guest endpoint over the runtime-loaded
//! `steam_api64.dll` (looked for next to the viewer exe, never downloaded). When the DLL or
//! Steam is missing, [`open_host`] returns `Err(reason)` and [`crate::link::open`] hosts on
//! UDP with a `transport_fallback` notice. Everything above [`open_host_with`] and
//! [`open_guest_with`] is generic over [`SteamApi`], so tests run on `MockSteam`.
//!
//! Host: friends-only lobby; the join code (`NVRS-...` naming the lobby, carrying the random
//! session token) and the token (lobby data key `token`) are published. [`SteamHandle`]
//! gives the viewer the invite hooks (invite a friend, open the overlay, list online friends,
//! take invites that arrived).
//!
//! Guest: `--coop-join NVRS-...` with a lobby code, or `--coop-steam-friend` which walks the
//! friends list for one in a Spacewar lobby, tries each until one passes the marker check and
//! reads the token from the lobby data.

use std::cell::RefCell;
use std::rc::Rc;
use std::time::{Duration, Instant};

use coop_core::ids::PlayerId;
use coop_core::proto::Channel;
use coop_core::transport::{Packet, Transport, TransportError};
use coop_net::steam::{self, Friend, Invite, SteamApi, SteamBackend, SteamOptions, SteamPhase};
use coop_net::{
    Backend, Clock, Endpoint, EndpointConfig, JoinCode, JoinTarget as NetTarget, Token,
};

use crate::link::{parse_token_hex, token_hex, LinkRequest, Notice, Opened};
use crate::net::{Link, LinkErrors};
use crate::Error;

/// Lobby data key holding the session token (friends-only lobby, so readable by friends only).
pub const TOKEN_KEY: &str = "token";

/// Timing knobs (tests shorten them).
#[derive(Clone, Debug)]
pub struct SteamSettings {
    /// How long a lobby create or join may take.
    pub setup_timeout: Duration,
    /// Poll interval while waiting.
    pub poll: Duration,
    /// Friends list refresh interval.
    pub friends_every: Duration,
    /// Backend options (tag, name, size, send retry).
    pub backend: SteamOptions,
}

impl Default for SteamSettings {
    fn default() -> Self {
        SteamSettings {
            setup_timeout: Duration::from_secs(15),
            poll: Duration::from_millis(5),
            friends_every: Duration::from_secs(2),
            backend: SteamOptions::default(),
        }
    }
}

#[derive(Default)]
struct Shared {
    invite_queue: Vec<u64>,
    overlay: bool,
    friends: Vec<Friend>,
    invites: Vec<Invite>,
    lobby: Option<u64>,
}

/// The viewer's hooks into a running Steam link (cheap to clone, not `Send`).
#[derive(Clone, Default)]
pub struct SteamHandle(Rc<RefCell<Shared>>);

impl SteamHandle {
    /// Host: invite a friend (by SteamID64) into the lobby; sent on the next frame.
    pub fn invite(&self, friend: u64) {
        self.0.borrow_mut().invite_queue.push(friend);
    }
    /// Host: open the Steam overlay invite dialog (may not hook our renderer).
    pub fn open_overlay(&self) {
        self.0.borrow_mut().overlay = true;
    }
    /// Online friends, refreshed every couple of seconds.
    pub fn friends(&self) -> Vec<Friend> {
        self.0.borrow().friends.clone()
    }
    /// Invites that arrived since the last call (Steam invite accepted, Join Game, rich presence).
    pub fn take_invites(&self) -> Vec<Invite> {
        std::mem::take(&mut self.0.borrow_mut().invites)
    }
    /// The lobby id once known.
    pub fn lobby(&self) -> Option<u64> {
        self.0.borrow().lobby
    }
}

/// The Steam link: a normal [`Link`] plus the handle's work done once per frame.
struct SteamLink<A: SteamApi> {
    link: Link<SteamBackend<A>>,
    shared: SteamHandle,
    friends_every: Duration,
    last_refresh: Option<Instant>,
}

impl<A: SteamApi> SteamLink<A> {
    fn service(&mut self) {
        let be = self.link.ep.backend_mut();
        let mut sh = self.shared.0.borrow_mut();
        for f in sh.invite_queue.drain(..) {
            be.invite_friend(f);
        }
        if std::mem::take(&mut sh.overlay) {
            be.open_overlay();
        }
        sh.invites.extend(be.take_invites());
        if self
            .last_refresh
            .map_or(true, |t| t.elapsed() >= self.friends_every)
        {
            self.last_refresh = Some(Instant::now());
            sh.friends = be.online_friends();
        }
    }
}

impl<A: SteamApi> Transport for SteamLink<A> {
    fn local(&self) -> PlayerId {
        self.link.local()
    }
    fn send(
        &mut self,
        to: PlayerId,
        channel: Channel,
        data: Vec<u8>,
    ) -> Result<(), TransportError> {
        self.link.send(to, channel, data)
    }
    fn recv(&mut self) -> Option<Packet> {
        self.service();
        self.link.recv()
    }
}

fn now_ms() -> u64 {
    Clock::system().now()
}

/// Polls `be` until `done(phase)` or the timeout. Returns the final phase.
fn wait_phase<A: SteamApi>(
    be: &mut SteamBackend<A>,
    st: &SteamSettings,
    done: impl Fn(&SteamPhase) -> bool,
) -> SteamPhase {
    let start = Instant::now();
    loop {
        let _ = be.poll(now_ms());
        if done(be.phase()) || start.elapsed() >= st.setup_timeout {
            return be.phase().clone();
        }
        std::thread::sleep(st.poll);
    }
}

fn ep_cfg(req: &LinkRequest) -> EndpointConfig {
    EndpointConfig {
        name: req.name.clone(),
        ..EndpointConfig::default()
    }
}

fn init_real(req: &LinkRequest) -> Result<steam::RealSteam, String> {
    steam::try_init(req.steam_dll.as_deref()).map_err(|e| e.to_string())
}

/// Host over the real DLL. `Err(reason)` means: fall back to UDP.
pub fn open_host(req: &LinkRequest, token: Token) -> Result<Opened, String> {
    let api = init_real(req)?;
    open_host_with(api, req, token, &SteamSettings::default())
}

/// Guest over the real DLL.
pub fn open_guest(req: &LinkRequest, code: Option<JoinCode>) -> Result<Opened, Error> {
    let api = init_real(req).map_err(|why| {
        Error::Target(format!(
            "Steam is not available ({why}). Start Steam and put steam_api64.dll next to the viewer, or ask the host for a UDP join code"
        ))
    })?;
    open_guest_with(api, req, code, &SteamSettings::default())
}

/// Host over any [`SteamApi`] (tests pass a `MockSteam`).
pub fn open_host_with<A: SteamApi + 'static>(
    api: A,
    req: &LinkRequest,
    token: Token,
    st: &SteamSettings,
) -> Result<Opened, String> {
    let mut be = SteamBackend::host(api, st.backend.clone());
    match wait_phase(&mut be, st, |p| {
        matches!(p, SteamPhase::Hosting { .. } | SteamPhase::Failed(_))
    }) {
        SteamPhase::Hosting { .. } => {}
        SteamPhase::Failed(m) => return Err(m),
        _ => return Err("Steam did not create a lobby in time".into()),
    }
    let lobby = be.lobby().ok_or("Steam lobby vanished")?;
    be.api_mut()
        .set_lobby_data(lobby, TOKEN_KEY, &token_hex(&token));
    let code = be.join_code(token).ok_or("no join code")?.to_string();
    let friends_online = be.online_friends().len();
    let errors = LinkErrors::default();
    let shared = SteamHandle::default();
    shared.0.borrow_mut().lobby = Some(lobby);
    let link = Link {
        ep: Endpoint::host(be, token, ep_cfg(req)),
        guest: false,
        errors: errors.clone(),
        retry_at: None,
    };
    let sl = SteamLink {
        link,
        shared: shared.clone(),
        friends_every: st.friends_every,
        last_refresh: None,
    };
    let mut o = Opened::new(Box::new(sl), errors, "steam");
    o.lines.push(format!(
        "Co-op: hosting on Steam (friends-only lobby {lobby})."
    ));
    o.lines.push(format!("Co-op: join code: {code}"));
    o.lines.push("Co-op: invite friends from Steam (or the in-game list), or send them the code. Both of you need Steam running.".into());
    o.notices.push(Notice::new(
        "steam_lobby",
        &[
            ("lobby", &lobby.to_string()),
            ("code", &code),
            ("friends_online", &friends_online.to_string()),
            ("invite", "friends-list,overlay,join-code,rich-presence"),
        ],
    ));
    o.notices.push(Notice::new(
        "coop_join_code",
        &[
            ("transport", "steam"),
            ("code", &code),
            ("token", &token_hex(&token)),
        ],
    ));
    o.join_code = Some(code);
    o.steam = Some(shared);
    Ok(o)
}

/// Guest over any [`SteamApi`]. `code` is the lobby code, or `None` for friend auto-join.
pub fn open_guest_with<A: SteamApi + 'static>(
    api: A,
    req: &LinkRequest,
    code: Option<JoinCode>,
    st: &SteamSettings,
) -> Result<Opened, Error> {
    let (mut candidates, code_token): (Vec<u64>, Option<Token>) = match &code {
        Some(c) => match c.target {
            NetTarget::SteamLobby(id) => (vec![id], Some(c.token)),
            _ => {
                return Err(Error::Target(
                    "this join code does not name a Steam lobby".into(),
                ))
            }
        },
        None => {
            let hosting: Vec<u64> = api
                .friends()
                .into_iter()
                .filter(|f| f.playing_app == Some(480) && f.lobby != 0)
                .map(|f| f.lobby)
                .collect();
            if hosting.is_empty() {
                return Err(Error::Target(
                    "no Steam friend is hosting an nv-rs game right now. Ask your friend to start hosting (and keep Steam running), or use their join code".into(),
                ));
            }
            (hosting, None)
        }
    };
    candidates.dedup();
    let mut it = candidates.into_iter();
    let first = it.next().expect("at least one candidate");
    let mut be = SteamBackend::guest(api, first, st.backend.clone());
    let mut last_why: String;
    let mut lobby = first;
    loop {
        match wait_phase(&mut be, st, |p| {
            matches!(p, SteamPhase::Guest { .. } | SteamPhase::Failed(_))
        }) {
            SteamPhase::Guest { .. } => break,
            SteamPhase::Failed(m) => last_why = m,
            _ => last_why = "Steam did not answer in time".into(),
        }
        match it.next() {
            Some(next) => {
                lobby = next;
                be.retarget(next);
            }
            None => {
                return Err(Error::Target(format!(
                    "could not join over Steam: {last_why}"
                )))
            }
        }
    }
    let token = match code_token {
        Some(t) => t,
        None => be
            .api()
            .get_lobby_data(lobby, TOKEN_KEY)
            .and_then(|h| parse_token_hex(&h))
            .ok_or_else(|| {
                Error::Target(
                    "that Steam lobby has no session token (the host runs another build)".into(),
                )
            })?,
    };
    let errors = LinkErrors::default();
    let shared = SteamHandle::default();
    shared.0.borrow_mut().lobby = Some(lobby);
    let link = Link {
        ep: Endpoint::guest(be, token, ep_cfg(req)),
        guest: true,
        errors: errors.clone(),
        retry_at: None,
    };
    let sl = SteamLink {
        link,
        shared: shared.clone(),
        friends_every: st.friends_every,
        last_refresh: None,
    };
    let mut o = Opened::new(Box::new(sl), errors, "steam");
    o.lines.push(format!("Co-op: joined Steam lobby {lobby}."));
    o.notices.push(Notice::new(
        "steam_joined",
        &[
            ("lobby", &lobby.to_string()),
            ("via", if code.is_some() { "code" } else { "friend" }),
        ],
    ));
    o.steam = Some(shared);
    Ok(o)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Role;
    use coop_net::steam::{MockSteam, MockUniverse};

    fn fast() -> SteamSettings {
        SteamSettings {
            setup_timeout: Duration::from_millis(300),
            poll: Duration::from_millis(1),
            friends_every: Duration::from_millis(1),
            backend: SteamOptions {
                retry_sleep: Duration::from_millis(1),
                ..Default::default()
            },
        }
    }

    fn users() -> (MockUniverse, MockSteam, MockSteam) {
        let uni = MockUniverse::new();
        let h = uni.user(1, "host");
        let g = uni.user(2, "guest");
        uni.befriend(&h, &g);
        (uni, h, g)
    }

    fn pump_both(h: &mut Opened, g: &mut Opened) {
        for _ in 0..30 {
            h.transport.recv();
            g.transport.recv();
        }
    }

    #[test]
    fn host_publishes_a_code_with_the_token_and_the_guest_joins_by_code() {
        let (_u, h, g) = users();
        let token = [9u8; 16];
        let hreq = LinkRequest::new(Role::Host, "H", "27961");
        let mut ho = open_host_with(h, &hreq, token, &fast()).unwrap();
        assert_eq!(ho.kind, "steam");
        let code = ho.join_code.clone().unwrap();
        let parsed: JoinCode = code.parse().unwrap();
        assert_eq!(parsed.token, token);
        assert!(matches!(parsed.target, NetTarget::SteamLobby(_)));
        assert!(ho
            .notices
            .iter()
            .any(|n| n.ev == "steam_lobby" && n.get("code") == Some(code.as_str())));

        let greq = LinkRequest::new(Role::Guest, "G", &code);
        let mut go = open_guest_with(g, &greq, Some(parsed), &fast()).unwrap();
        pump_both(&mut ho, &mut go);
        assert_eq!(
            go.transport.local(),
            PlayerId(1),
            "slot assigned by the handshake"
        );
        let hh = ho.steam.clone().unwrap();
        assert!(hh.lobby().is_some());
    }

    #[test]
    fn friend_auto_join_reads_the_token_from_the_lobby() {
        let (_u, h, g) = users();
        let token = [5u8; 16];
        let hreq = LinkRequest::new(Role::Host, "H", "27961");
        let mut ho = open_host_with(h, &hreq, token, &fast()).unwrap();
        let mut greq = LinkRequest::new(Role::Guest, "G", "");
        greq.steam_friend = true;
        let mut go = open_guest_with(g, &greq, None, &fast()).unwrap();
        assert_eq!(go.notices[0].get("via"), Some("friend"));
        pump_both(&mut ho, &mut go);
        assert_eq!(go.transport.local(), PlayerId(1));
    }

    #[test]
    fn friend_join_with_nobody_hosting_explains() {
        let (_u, _h, g) = users();
        let mut greq = LinkRequest::new(Role::Guest, "G", "");
        greq.steam_friend = true;
        let e = open_guest_with(g, &greq, None, &fast()).err().unwrap();
        assert!(e.to_string().contains("hosting"), "{e}");
    }

    #[test]
    fn a_wrong_lobby_is_reported_not_hung() {
        let (_u, _h, g) = users();
        let code = JoinCode::new(NetTarget::SteamLobby(424242), [1; 16]);
        let greq = LinkRequest::new(Role::Guest, "G", &code.to_string());
        let e = open_guest_with(g, &greq, Some(code), &fast())
            .err()
            .unwrap();
        assert!(e.to_string().contains("could not join"), "{e}");
    }

    #[test]
    fn invite_hook_sends_and_lists_friends() {
        let (_u, h, g) = users();
        let hreq = LinkRequest::new(Role::Host, "H", "27961");
        let mut ho = open_host_with(h, &hreq, [2; 16], &fast()).unwrap();
        let hh = ho.steam.clone().unwrap();
        ho.transport.recv();
        assert!(hh.friends().iter().any(|f| f.name == "guest"));
        hh.invite(SteamApi::self_id(&g));
        ho.transport.recv();
    }

    #[test]
    fn missing_dll_is_a_typed_unavailable_reason() {
        let mut r = LinkRequest::new(Role::Host, "H", "27961");
        r.steam_dll = Some("no-such-dir/steam_api64.dll".into());
        let why = open_host(&r, [0; 16]).err().expect("unavailable");
        assert!(why.contains("steam_api64.dll"), "{why}");
    }
}

//! steam: setting up a session's Steam side, generic over [`SteamApi`] so the tests run on
//! [`super::MockSteam`] (docs/coop/SPECS.md S33). The old viewer's design (its `steam_transport`),
//! moved here so any program can use it.
//!
//! - [`host`]: makes a friends-only lobby, waits for it, writes the session token into the lobby's
//!   data ([`TOKEN_KEY`]; only friends can read a friends-only lobby) and returns the backend, the
//!   [`SteamHandle`] and the lobby's join code.
//! - [`guest`]: joins a lobby named by a join code, or (no code) the lobby of a Steam friend who
//!   hosts, or the lobby of an invite that arrives while it waits; reads the token from the
//!   lobby.
//! - [`Linked`]: the backend with the handle's requests carried out once per poll: invites to
//!   send, the overlay's invite dialog to open, the friends list refreshed, invites taken.

use std::cell::RefCell;
use std::rc::Rc;
use std::time::{Duration, Instant};

use crate::backend::{Backend, BackendError, BackendEvent, Lane, PeerKey, Readiness};
use crate::endpoint::Token;
use crate::join::JoinCode;
use crate::liveness::TOKEN_LEN;

use super::api::{Friend, LobbyId, SteamApi, SteamEvent, SteamId};
use super::backend::{Invite, SteamBackend, SteamOptions, SteamPhase};

/// Lobby data key holding the session token.
pub const TOKEN_KEY: &str = "token";
/// The AppID the friends list shows a host's game under (Spacewar).
pub const SPACEWAR: u32 = 480;

/// The token as lower-case hex.
pub fn token_hex(token: &Token) -> String {
    token.iter().map(|b| format!("{b:02x}")).collect()
}

/// A token from [`token_hex`]'s text.
pub fn parse_token_hex(text: &str) -> Option<Token> {
    let t = text.trim();
    if t.len() != TOKEN_LEN * 2 || !t.is_ascii() {
        return None;
    }
    let mut out = [0u8; TOKEN_LEN];
    for (i, b) in out.iter_mut().enumerate() {
        *b = u8::from_str_radix(&t[i * 2..i * 2 + 2], 16).ok()?;
    }
    Some(out)
}

/// Valve's Steam library, which the program loads at run time (never shipped in the source).
pub const DLL_NAME: &str = "steam_api64.dll";

/// Where the program looks for [`DLL_NAME`], in order: the file `explicit` names
/// (`NV_COOP_STEAM_DLL`, which the launcher sets to the one next to `Play.exe`), then the
/// program's own folder, then the folder above it (the package's, where `Play.exe` is: the game
/// program is `app\FalloutNV.exe`).
pub fn dll_places(
    explicit: Option<&std::path::Path>,
    exe: Option<&std::path::Path>,
) -> Vec<std::path::PathBuf> {
    let mut all = Vec::new();
    if let Some(p) = explicit.filter(|p| !p.as_os_str().is_empty()) {
        all.push(p.to_path_buf());
    }
    if let Some(dir) = exe.and_then(|e| e.parent()) {
        all.push(dir.join(DLL_NAME));
        if let Some(up) = dir.parent() {
            all.push(up.join(DLL_NAME));
        }
    }
    let mut out: Vec<std::path::PathBuf> = Vec::new();
    for p in all {
        if !out.contains(&p) {
            out.push(p);
        }
    }
    out
}

/// The first of `places` that `exists`, or the words that say it is missing and where it was
/// looked for.
pub fn find_dll(
    places: &[std::path::PathBuf],
    exists: impl Fn(&std::path::Path) -> bool,
) -> Result<std::path::PathBuf, String> {
    places.iter().find(|p| exists(p)).cloned().ok_or_else(|| {
        let looked: Vec<String> = places.iter().map(|p| p.display().to_string()).collect();
        format!(
            "{DLL_NAME} is not next to Play.exe (looked at: {}). Steam invites need it: put Valve's {DLL_NAME} (Steamworks SDK, 64-bit) next to Play.exe. The join code works without it",
            if looked.is_empty() { "nowhere".to_string() } else { looked.join(", ") }
        )
    })
}

/// Timing of the setup (tests shorten it).
#[derive(Clone, Debug)]
pub struct Setup {
    /// How long a lobby create or join may take.
    pub timeout: Duration,
    /// How long a guest with no code waits for a friend to host or invite.
    pub wait_for_host: Duration,
    /// Poll interval while waiting.
    pub poll: Duration,
    /// Friends list refresh interval while running (ms).
    pub friends_every_ms: u64,
    /// Backend options (tag, name, size, send retry).
    pub opts: SteamOptions,
}

impl Default for Setup {
    fn default() -> Self {
        Setup {
            timeout: Duration::from_secs(15),
            wait_for_host: Duration::from_secs(120),
            poll: Duration::from_millis(5),
            friends_every_ms: 2000,
            opts: SteamOptions::default(),
        }
    }
}

#[derive(Default)]
struct Shared {
    invite_queue: Vec<SteamId>,
    overlay: bool,
    overlays_opened: u32,
    friends: Vec<Friend>,
    invites: Vec<Invite>,
    lobby: Option<LobbyId>,
    phase: Option<SteamPhase>,
}

/// The program's hooks into a running Steam backend (cheap to clone, not `Send`).
#[derive(Clone, Default)]
pub struct SteamHandle(Rc<RefCell<Shared>>);

impl SteamHandle {
    /// Host: invites a friend (by SteamID64) into the lobby, on the next poll.
    pub fn invite(&self, friend: SteamId) {
        self.0.borrow_mut().invite_queue.push(friend);
    }
    /// Host: opens the Steam overlay's invite dialog on the next poll.
    pub fn open_overlay(&self) {
        self.0.borrow_mut().overlay = true;
    }
    /// Times the invite dialog was asked of Steam.
    pub fn overlays_opened(&self) -> u32 {
        self.0.borrow().overlays_opened
    }
    /// Online friends, refreshed every few seconds.
    pub fn friends(&self) -> Vec<Friend> {
        self.0.borrow().friends.clone()
    }
    /// Invites that arrived since the last call.
    pub fn take_invites(&self) -> Vec<Invite> {
        std::mem::take(&mut self.0.borrow_mut().invites)
    }
    /// The lobby.
    pub fn lobby(&self) -> Option<LobbyId> {
        self.0.borrow().lobby
    }
    /// The backend's phase at the last poll.
    pub fn phase(&self) -> Option<SteamPhase> {
        self.0.borrow().phase.clone()
    }
}

/// A [`SteamBackend`] with its [`SteamHandle`] served on every poll.
pub struct Linked<A: SteamApi> {
    be: SteamBackend<A>,
    shared: SteamHandle,
    friends_every_ms: u64,
    last_refresh: Option<u64>,
}

impl<A: SteamApi> Linked<A> {
    fn new(be: SteamBackend<A>, setup: &Setup) -> (Self, SteamHandle) {
        let shared = SteamHandle::default();
        {
            let mut s = shared.0.borrow_mut();
            s.lobby = be.lobby();
            s.phase = Some(be.phase().clone());
        }
        (
            Linked {
                be,
                shared: shared.clone(),
                friends_every_ms: setup.friends_every_ms,
                last_refresh: None,
            },
            shared,
        )
    }

    /// The backend.
    pub fn backend(&self) -> &SteamBackend<A> {
        &self.be
    }

    fn service(&mut self, now_ms: u64) {
        let mut sh = self.shared.0.borrow_mut();
        for f in sh.invite_queue.drain(..) {
            self.be.invite_friend(f);
        }
        if std::mem::take(&mut sh.overlay) {
            self.be.open_overlay();
            sh.overlays_opened += 1;
        }
        sh.invites.extend(self.be.take_invites());
        if self
            .last_refresh
            .map_or(true, |t| now_ms.saturating_sub(t) >= self.friends_every_ms)
        {
            self.last_refresh = Some(now_ms);
            sh.friends = self.be.online_friends();
        }
        sh.lobby = self.be.lobby();
        sh.phase = Some(self.be.phase().clone());
    }
}

impl<A: SteamApi> Backend for Linked<A> {
    fn send(&mut self, peer: PeerKey, lane: Lane, data: &[u8]) -> Result<(), BackendError> {
        self.be.send(peer, lane, data)
    }
    fn poll(&mut self, now_ms: u64) -> Vec<BackendEvent> {
        let out = self.be.poll(now_ms);
        self.service(now_ms);
        out
    }
    fn readiness(&self) -> Readiness {
        self.be.readiness()
    }
    fn host_peer(&self) -> Option<PeerKey> {
        self.be.host_peer()
    }
    fn disconnect(&mut self, peer: PeerKey) {
        self.be.disconnect(peer)
    }
    fn rtt_ms(&self, peer: PeerKey) -> Option<u32> {
        self.be.rtt_ms(peer)
    }
}

fn clock_ms(start: Instant) -> u64 {
    start.elapsed().as_millis() as u64
}

/// Polls `be` until `done(phase)` or the timeout; the phase it ends in.
fn wait_phase<A: SteamApi>(
    be: &mut SteamBackend<A>,
    setup: &Setup,
    done: impl Fn(&SteamPhase) -> bool,
) -> SteamPhase {
    let start = Instant::now();
    loop {
        let _ = be.poll(clock_ms(start));
        if done(be.phase()) || start.elapsed() >= setup.timeout {
            return be.phase().clone();
        }
        std::thread::sleep(setup.poll);
    }
}

/// A host's Steam side: the backend, its handle and the lobby's join code.
pub struct Hosted<A: SteamApi> {
    /// The backend (put it under the host's endpoint).
    pub backend: Linked<A>,
    /// The program's hooks.
    pub handle: SteamHandle,
    /// The lobby.
    pub lobby: LobbyId,
    /// The join code naming the lobby, with the token.
    pub code: JoinCode,
}

/// Makes a friends-only lobby for a session whose token is `token`. `Err` says why (the program
/// then hosts without Steam).
pub fn host<A: SteamApi>(api: A, token: Token, setup: &Setup) -> Result<Hosted<A>, String> {
    let mut be = SteamBackend::host(api, setup.opts.clone());
    match wait_phase(&mut be, setup, |p| {
        matches!(p, SteamPhase::Hosting { .. } | SteamPhase::Failed(_))
    }) {
        SteamPhase::Hosting { .. } => {}
        SteamPhase::Failed(m) => return Err(m),
        _ => return Err("Steam did not make a lobby in time".into()),
    }
    let lobby = be.lobby().ok_or("the Steam lobby went away")?;
    if !be
        .api_mut()
        .set_lobby_data(lobby, TOKEN_KEY, &token_hex(&token))
    {
        return Err("Steam did not take the session's data for the lobby".into());
    }
    let code = be.join_code(token).ok_or("the Steam lobby has no code")?;
    let (backend, handle) = Linked::new(be, setup);
    Ok(Hosted {
        backend,
        handle,
        lobby,
        code,
    })
}

/// Where a guest joins on Steam.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GuestTarget {
    /// The lobby a join code names, with the code's token.
    Code(JoinCode),
    /// A Steam friend's lobby: one already hosting, or the lobby of an invite that arrives.
    Friend,
}

/// A guest's Steam side.
pub struct Joined<A: SteamApi> {
    /// The backend (put it under the guest's endpoint).
    pub backend: Linked<A>,
    /// The program's hooks.
    pub handle: SteamHandle,
    /// The lobby joined.
    pub lobby: LobbyId,
    /// The session token (from the code, or the lobby's data).
    pub token: Token,
    /// How it was found: `code`, `friend` or `invite`.
    pub via: &'static str,
}

/// Lobbies to try, from what Steam says now: friends in a Spacewar lobby, and invites.
fn candidates<A: SteamApi>(api: &mut A, found: &mut Vec<(LobbyId, &'static str)>) {
    for ev in api.run_frame() {
        let lobby = match ev {
            SteamEvent::JoinRequested { lobby, .. } | SteamEvent::LobbyInvite { lobby, .. } => {
                Some(lobby)
            }
            SteamEvent::RichPresenceJoin { connect, .. } => connect
                .split_whitespace()
                .last()
                .and_then(|s| s.parse::<u64>().ok()),
            _ => None,
        };
        if let Some(l) = lobby.filter(|l| *l != 0) {
            if !found.iter().any(|(f, _)| *f == l) {
                found.push((l, "invite"));
            }
        }
    }
    for f in api.friends() {
        if f.online
            && f.playing_app == Some(SPACEWAR)
            && f.lobby != 0
            && !found.iter().any(|(l, _)| *l == f.lobby)
        {
            found.push((f.lobby, "friend"));
        }
    }
}

/// Joins on Steam. `Err` says why, in words for the player.
pub fn guest<A: SteamApi>(
    mut api: A,
    target: GuestTarget,
    setup: &Setup,
) -> Result<Joined<A>, String> {
    let (mut queue, code_token): (Vec<(LobbyId, &'static str)>, Option<Token>) = match &target {
        GuestTarget::Code(c) => match c.target {
            crate::join::JoinTarget::SteamLobby(id) => (vec![(id, "code")], Some(c.token)),
            _ => return Err("this join code does not name a Steam lobby".into()),
        },
        GuestTarget::Friend => {
            let start = Instant::now();
            let mut found = Vec::new();
            loop {
                candidates(&mut api, &mut found);
                if !found.is_empty() || start.elapsed() >= setup.wait_for_host {
                    break;
                }
                std::thread::sleep(setup.poll.max(Duration::from_millis(1)) * 20);
            }
            if found.is_empty() {
                return Err("no Steam friend hosted or sent an invite in time. Ask your friend to host with Steam (and keep Steam running), then start again, or use their join code".into());
            }
            (found, None)
        }
    };
    queue.reverse();
    let (first, mut via) = queue.pop().expect("at least one lobby");
    let mut lobby = first;
    let mut be = SteamBackend::guest(api, first, setup.opts.clone());
    let mut last_why: String;
    loop {
        match wait_phase(&mut be, setup, |p| {
            matches!(p, SteamPhase::Guest { .. } | SteamPhase::Failed(_))
        }) {
            SteamPhase::Guest { .. } => break,
            SteamPhase::Failed(m) => last_why = m,
            _ => last_why = "Steam did not answer in time".into(),
        }
        match queue.pop() {
            Some((next, how)) => {
                lobby = next;
                via = how;
                be.retarget(next);
            }
            None => return Err(format!("could not join over Steam: {last_why}")),
        }
    }
    let token = match code_token {
        Some(t) => t,
        None => be
            .api()
            .get_lobby_data(lobby, TOKEN_KEY)
            .and_then(|h| parse_token_hex(&h))
            .ok_or("that Steam lobby has no session token (the host runs another build)")?,
    };
    let (backend, handle) = Linked::new(be, setup);
    Ok(Joined {
        backend,
        handle,
        lobby,
        token,
        via,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::endpoint::{Endpoint, EndpointConfig, NetEvent};
    use crate::join::JoinTarget;
    use crate::steam::{MockSteam, MockUniverse};
    use coop_core::ids::PlayerId;

    fn fast() -> Setup {
        Setup {
            timeout: Duration::from_millis(300),
            wait_for_host: Duration::from_millis(200),
            poll: Duration::from_millis(1),
            friends_every_ms: 0,
            opts: SteamOptions {
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

    fn cfg(n: &str) -> EndpointConfig {
        EndpointConfig {
            name: n.into(),
            ..EndpointConfig::default()
        }
    }

    fn pump<A: Backend, B: Backend>(h: &mut Endpoint<A>, g: &mut Endpoint<B>) {
        for _ in 0..30 {
            h.pump();
            g.pump();
        }
    }

    #[test]
    fn the_library_is_looked_for_as_named_then_by_the_program_then_by_play_exe() {
        use std::path::{Path, PathBuf};
        let exe = Path::new("pkg").join("app").join("FalloutNV.exe");
        let named = PathBuf::from("pkg").join(DLL_NAME);
        let places = dll_places(Some(&named), Some(&exe));
        assert_eq!(
            places,
            vec![named.clone(), Path::new("pkg").join("app").join(DLL_NAME),],
            "the package's folder is the named one, not listed twice"
        );
        let places = dll_places(None, Some(&exe));
        assert_eq!(places[1], Path::new("pkg").join(DLL_NAME));
        assert_eq!(
            find_dll(&places, |p| p == Path::new("pkg").join(DLL_NAME)),
            Ok(Path::new("pkg").join(DLL_NAME))
        );
        let e = find_dll(&places, |_| false).unwrap_err();
        assert!(
            e.contains("not next to Play.exe") && e.contains("join code"),
            "{e}"
        );
        assert!(e.contains(&Path::new("pkg").join(DLL_NAME).display().to_string()));
        assert!(find_dll(&[], |_| true).unwrap_err().contains("nowhere"));
    }

    #[test]
    fn tokens_read_back() {
        let t = [0, 1, 0xab, 0xff, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 0x10];
        assert_eq!(parse_token_hex(&token_hex(&t)), Some(t));
        assert_eq!(parse_token_hex("00"), None);
        assert_eq!(parse_token_hex(&"zz".repeat(16)), None);
    }

    #[test]
    fn the_host_publishes_a_code_with_the_token_and_a_guest_joins_by_it() {
        let (_u, h, g) = users();
        let token = [9u8; 16];
        let hosted = host(h, token, &fast()).unwrap();
        assert_eq!(hosted.code.token, token);
        assert_eq!(hosted.code.target, JoinTarget::SteamLobby(hosted.lobby));
        assert_eq!(
            hosted
                .backend
                .backend()
                .api()
                .get_lobby_data(hosted.lobby, TOKEN_KEY),
            Some(token_hex(&token))
        );
        // The code goes through text, as a player pastes it.
        let code: JoinCode = hosted.code.to_string().parse().unwrap();
        let joined = guest(g, GuestTarget::Code(code), &fast()).unwrap();
        assert_eq!((joined.via, joined.lobby), ("code", hosted.lobby));
        let mut he = Endpoint::host(hosted.backend, token, cfg("h"));
        let mut ge = Endpoint::guest(joined.backend, joined.token, cfg("g"));
        pump(&mut he, &mut ge);
        assert!(ge.is_connected());
        assert_eq!(coop_core::transport::Transport::local(&ge), PlayerId(1));
        assert!(he
            .events()
            .iter()
            .any(|e| matches!(e, NetEvent::PeerJoined { name, .. } if name == "g")));
    }

    #[test]
    fn a_friend_with_no_code_finds_the_hosting_friend_and_reads_the_token() {
        let (_u, h, g) = users();
        let token = [5u8; 16];
        let hosted = host(h, token, &fast()).unwrap();
        let joined = guest(g, GuestTarget::Friend, &fast()).unwrap();
        assert_eq!(joined.via, "friend");
        assert_eq!(joined.token, token);
        let mut he = Endpoint::host(hosted.backend, token, cfg("h"));
        let mut ge = Endpoint::guest(joined.backend, joined.token, cfg("g"));
        pump(&mut he, &mut ge);
        assert!(ge.is_connected());
    }

    #[test]
    fn an_invite_that_arrives_while_waiting_is_followed() {
        let (_u, h, g) = users();
        // The host's game shows as something else: only the invite leads to it.
        h.set_playing_app(Some(1));
        let token = [6u8; 16];
        let hosted = host(h, token, &fast()).unwrap();
        hosted.handle.invite(g.self_id());
        let mut he = Endpoint::host(hosted.backend, token, cfg("h"));
        he.pump();
        let joined = guest(g, GuestTarget::Friend, &fast()).unwrap();
        assert_eq!(joined.via, "invite");
        let mut ge = Endpoint::guest(joined.backend, joined.token, cfg("g"));
        pump(&mut he, &mut ge);
        assert!(ge.is_connected());
    }

    #[test]
    fn nobody_hosting_is_said_plainly_not_hung() {
        let (_u, _h, g) = users();
        let e = guest(g, GuestTarget::Friend, &fast()).err().unwrap();
        assert!(e.contains("no Steam friend"), "{e}");
    }

    #[test]
    fn a_wrong_lobby_or_a_udp_code_is_refused() {
        let (_u, _h, g) = users();
        let code = JoinCode::new(JoinTarget::SteamLobby(424_242), [1; 16]);
        let e = guest(g, GuestTarget::Code(code), &fast()).err().unwrap();
        assert!(e.contains("could not join"), "{e}");
        let (_u, _h, g) = users();
        let udp = JoinCode::new(
            JoinTarget::Direct(std::net::SocketAddr::from(([127, 0, 0, 1], 4000))),
            [1; 16],
        );
        let e = guest(g, GuestTarget::Code(udp), &fast()).err().unwrap();
        assert!(e.contains("does not name a Steam lobby"), "{e}");
    }

    #[test]
    fn the_handle_sends_invites_opens_the_dialog_and_lists_friends() {
        let (_u, h, g) = users();
        let mut hosted = host(h, [2; 16], &fast()).unwrap();
        hosted.handle.open_overlay();
        hosted.handle.invite(g.self_id());
        let _ = hosted.backend.poll(10);
        assert_eq!(hosted.handle.overlays_opened(), 1);
        assert!(hosted.handle.friends().iter().any(|f| f.name == "guest"));
        assert!(matches!(
            hosted.handle.phase(),
            Some(SteamPhase::Hosting { .. })
        ));
        // The invite reached the friend's Steam.
        let mut g = g;
        assert!(g
            .run_frame()
            .iter()
            .any(|e| matches!(e, SteamEvent::LobbyInvite { lobby, .. } if *lobby == hosted.lobby)));
        // Asked once, opened once.
        let _ = hosted.backend.poll(20);
        assert_eq!(hosted.handle.overlays_opened(), 1);
    }
}

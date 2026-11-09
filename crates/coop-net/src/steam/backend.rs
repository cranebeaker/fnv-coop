//! `SteamBackend`: lobby create/join/invite logic over any [`SteamApi`] and
//! the lane mapping onto Steam message channels.
//!
//! | Lane | Channel | Flags |
//! |---|---|---|
//! | Reliable | 0 | Reliable + NoNagle (9) |
//! | Control | 1 | Reliable + NoNagle (9) |
//! | Unreliable | 2 | Unreliable + NoNagle (1) |
//!
//! Session rules: the host accepts message sessions only from lobby members,
//! a guest accepts only the lobby owner. Peers vanish on `LobbyChatUpdate`
//! left/disconnected/kicked/banned and on `SessionFailed`.

use std::collections::VecDeque;
use std::time::Duration;

use crate::backend::{Backend, BackendError, BackendEvent, Lane, PeerKey, Readiness};
use crate::endpoint::Token;
use crate::join::{JoinCode, JoinTarget};

use super::api::*;

/// Lobby data key that marks one of our lobbies among all Spacewar users.
pub const MARKER_KEY: &str = "nvrscoop";
/// Highest unreliable payload (frame included) sent over Steam.
const MAX_UNRELIABLE_FRAME: usize = 1200;

/// Settings for the Steam backend.
#[derive(Clone, Debug)]
pub struct SteamOptions {
    /// Protocol/build tag stored in the lobby and checked by joiners.
    pub tag: String,
    /// Lobby display name (not an identity).
    pub name: String,
    /// Lobby size (party size).
    pub max_members: u32,
    /// Wait between retries of a send that hit a full buffer.
    pub retry_sleep: Duration,
    /// Retries before a reliable send gives up.
    pub max_retries: u32,
}

impl Default for SteamOptions {
    fn default() -> Self {
        SteamOptions {
            tag: "1".into(),
            name: "nv-rs co-op".into(),
            max_members: 4,
            retry_sleep: Duration::from_millis(5),
            max_retries: 400,
        }
    }
}

/// Where lobby setup stands.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SteamPhase {
    /// Host: waiting for `LobbyCreated`.
    Creating,
    /// Host: lobby ready.
    Hosting {
        /// The lobby.
        lobby: LobbyId,
    },
    /// Guest: waiting for `LobbyEnter`.
    Joining {
        /// The lobby.
        lobby: LobbyId,
    },
    /// Guest: inside a verified lobby.
    Guest {
        /// The lobby.
        lobby: LobbyId,
        /// The host's SteamID64.
        owner: SteamId,
    },
    /// Setup failed; the text is user-facing.
    Failed(String),
}

/// An invite that reached us (callbacks 333, 337, 503).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Invite {
    /// Lobby to join.
    pub lobby: LobbyId,
    /// Who sent it.
    pub from: SteamId,
}

/// Steam [`Backend`].
pub struct SteamBackend<A: SteamApi> {
    api: A,
    opts: SteamOptions,
    phase: SteamPhase,
    invites: Vec<Invite>,
    /// Sessions refused because the sender was not an allowed lobby member.
    pub rejected_sessions: u64,
    /// Sends that were retried after a full buffer.
    pub buffer_full_retries: u64,
    pending: VecDeque<BackendEvent>,
}

impl<A: SteamApi> SteamBackend<A> {
    /// Host: creates a friends-only lobby.
    pub fn host(mut api: A, opts: SteamOptions) -> Self {
        api.create_lobby(opts.max_members);
        SteamBackend {
            api,
            opts,
            phase: SteamPhase::Creating,
            invites: vec![],
            rejected_sessions: 0,
            buffer_full_retries: 0,
            pending: VecDeque::new(),
        }
    }

    /// Guest: joins `lobby`, then verifies the marker key and tag.
    pub fn guest(mut api: A, lobby: LobbyId, opts: SteamOptions) -> Self {
        api.join_lobby(lobby);
        SteamBackend {
            api,
            opts,
            phase: SteamPhase::Joining { lobby },
            invites: vec![],
            rejected_sessions: 0,
            buffer_full_retries: 0,
            pending: VecDeque::new(),
        }
    }

    /// Guest: points a backend that has not connected (its last attempt
    /// failed or timed out) at another lobby, for "try the next hosting friend".
    pub fn retarget(&mut self, lobby: LobbyId) {
        if let Some(l) = self.lobby() {
            self.api.leave_lobby(l);
        }
        self.api.join_lobby(lobby);
        self.phase = SteamPhase::Joining { lobby };
    }

    /// Current setup phase.
    pub fn phase(&self) -> &SteamPhase {
        &self.phase
    }

    /// The lobby id once known.
    pub fn lobby(&self) -> Option<LobbyId> {
        match self.phase {
            SteamPhase::Hosting { lobby }
            | SteamPhase::Joining { lobby }
            | SteamPhase::Guest { lobby, .. } => Some(lobby),
            _ => None,
        }
    }

    /// The Steam API handle.
    pub fn api(&self) -> &A {
        &self.api
    }
    /// The Steam API handle, mutable.
    pub fn api_mut(&mut self) -> &mut A {
        &mut self.api
    }

    /// Join code for this lobby (copy it to the clipboard as the fallback).
    pub fn join_code(&self, token: Token) -> Option<JoinCode> {
        self.lobby()
            .map(|l| JoinCode::new(JoinTarget::SteamLobby(l), token))
    }

    /// Invites that arrived since the last call.
    pub fn take_invites(&mut self) -> Vec<Invite> {
        std::mem::take(&mut self.invites)
    }

    /// Online friends for the in-game invite list.
    pub fn online_friends(&self) -> Vec<Friend> {
        self.api
            .friends()
            .into_iter()
            .filter(|f| f.online)
            .collect()
    }

    /// Friends who are in a Spacewar lobby: candidates for "Friend X is hosting".
    /// The lobby still has to pass the marker check when joined.
    pub fn hosting_friends(&self) -> Vec<Friend> {
        self.api
            .friends()
            .into_iter()
            .filter(|f| f.playing_app == Some(480) && f.lobby != 0)
            .collect()
    }

    /// Host: sends a Steam invite to a friend.
    pub fn invite_friend(&mut self, friend: SteamId) -> bool {
        match self.phase {
            SteamPhase::Hosting { lobby } => self.api.invite_user(lobby, friend),
            _ => false,
        }
    }

    /// Host: opens the overlay invite dialog (convenience; may not hook our renderer).
    pub fn open_overlay(&mut self) {
        if let SteamPhase::Hosting { lobby } = self.phase {
            self.api.open_invite_overlay(lobby);
        }
    }

    /// Host: updates the lobby `mode` (`lobby` or `game`). It stays joinable
    /// for join-in-progress.
    pub fn set_mode(&mut self, mode: &str) {
        if let SteamPhase::Hosting { lobby } = self.phase {
            self.api.set_lobby_data(lobby, "mode", mode);
        }
    }

    /// Leaves the lobby and clears rich presence.
    pub fn leave(&mut self) {
        if let Some(l) = self.lobby() {
            self.api.leave_lobby(l);
        }
        self.api.clear_rich_presence();
    }

    fn fail(&mut self, msg: &str) {
        if let Some(l) = self.lobby() {
            self.api.leave_lobby(l);
        }
        self.phase = SteamPhase::Failed(msg.into());
    }

    fn session_allowed(&self, from: SteamId) -> bool {
        match self.phase {
            SteamPhase::Hosting { lobby } => self.api.lobby_members(lobby).contains(&from),
            SteamPhase::Guest { owner, .. } => from == owner,
            _ => false,
        }
    }

    fn on_event(&mut self, ev: SteamEvent) {
        match ev {
            SteamEvent::LobbyCreated { result, lobby } => {
                // Delivered twice by real Steam: act only in the Creating phase.
                if self.phase == SteamPhase::Creating {
                    if result != 1 {
                        self.phase = SteamPhase::Failed(format!(
                            "could not create a Steam lobby (result {result})"
                        ));
                        return;
                    }
                    self.api.set_lobby_data(lobby, MARKER_KEY, "1");
                    self.api
                        .set_lobby_data(lobby, "tag", &self.opts.tag.clone());
                    self.api.set_lobby_data(lobby, "mode", "lobby");
                    self.api
                        .set_lobby_data(lobby, "name", &self.opts.name.clone());
                    self.api
                        .set_rich_presence("connect", &format!("+connect_lobby {lobby}"));
                    self.api.set_rich_presence("status", "Hosting nv-rs co-op");
                    self.phase = SteamPhase::Hosting { lobby };
                }
            }
            SteamEvent::LobbyEnter { lobby, response } => {
                if let SteamPhase::Joining { lobby: want } = self.phase {
                    if lobby != want {
                        return;
                    }
                    if response != 1 {
                        let why = match response {
                            2 => "that lobby does not exist any more",
                            3 => {
                                "you are not allowed in that lobby (is the host your Steam friend?)"
                            }
                            4 => "that lobby is full",
                            _ => "could not join the Steam lobby",
                        };
                        self.phase = SteamPhase::Failed(why.into());
                        return;
                    }
                    if self.api.get_lobby_data(lobby, MARKER_KEY).as_deref() != Some("1") {
                        self.fail("that lobby is not an nv-rs co-op game");
                        return;
                    }
                    let tag = self.api.get_lobby_data(lobby, "tag").unwrap_or_default();
                    if tag != self.opts.tag {
                        self.fail(&format!(
                            "that game is a different version (host {tag}, you {})",
                            self.opts.tag
                        ));
                        return;
                    }
                    let owner = self.api.lobby_owner(lobby);
                    if owner == 0 || owner == self.api.self_id() {
                        self.fail("the Steam lobby has no host");
                        return;
                    }
                    self.phase = SteamPhase::Guest { lobby, owner };
                }
            }
            SteamEvent::LobbyChatUpdate {
                lobby,
                user_changed,
                state,
                ..
            } => {
                if Some(lobby) == self.lobby()
                    && state & CHAT_GONE_MASK != 0
                    && user_changed != self.api.self_id()
                {
                    self.pending
                        .push_back(BackendEvent::PeerGone { peer: user_changed });
                }
            }
            SteamEvent::JoinRequested { lobby, friend } => self.invites.push(Invite {
                lobby,
                from: friend,
            }),
            SteamEvent::LobbyInvite { inviter, lobby } => self.invites.push(Invite {
                lobby,
                from: inviter,
            }),
            SteamEvent::RichPresenceJoin { friend, connect } => {
                // "+connect_lobby <id>"
                if let Some(id) = connect
                    .split_whitespace()
                    .last()
                    .and_then(|s| s.parse::<u64>().ok())
                {
                    self.invites.push(Invite {
                        lobby: id,
                        from: friend,
                    });
                }
            }
            SteamEvent::SessionRequest { from } => {
                if self.session_allowed(from) {
                    self.api.accept_session(from);
                } else {
                    self.rejected_sessions += 1;
                }
            }
            SteamEvent::SessionFailed { from } => self
                .pending
                .push_back(BackendEvent::PeerGone { peer: from }),
        }
    }
}

impl<A: SteamApi> Backend for SteamBackend<A> {
    fn send(&mut self, peer: PeerKey, lane: Lane, data: &[u8]) -> Result<(), BackendError> {
        let (channel, flags) = match lane {
            Lane::Reliable | Lane::Control => (lane as i32, SEND_RELIABLE | SEND_NO_NAGLE),
            Lane::Unreliable => {
                if data.len() > MAX_UNRELIABLE_FRAME {
                    return Err(BackendError::TooLarge {
                        len: data.len(),
                        max: MAX_UNRELIABLE_FRAME,
                    });
                }
                (2, SEND_UNRELIABLE | SEND_NO_NAGLE)
            }
        };
        let mut tries = 0;
        loop {
            match self.api.send_message(peer, channel, flags, data) {
                SendResult::Ok => return Ok(()),
                SendResult::BufferFull => {
                    if lane == Lane::Unreliable {
                        return Ok(()); // disposable
                    }
                    tries += 1;
                    self.buffer_full_retries += 1;
                    if tries > self.opts.max_retries {
                        return Err(BackendError::Failed("steam send buffer stayed full".into()));
                    }
                    if !self.opts.retry_sleep.is_zero() {
                        std::thread::sleep(self.opts.retry_sleep);
                    }
                }
                SendResult::Failed(code) => {
                    return Err(BackendError::Failed(format!(
                        "SendMessageToUser result {code}"
                    )))
                }
            }
        }
    }

    fn poll(&mut self, _now_ms: u64) -> Vec<BackendEvent> {
        for ev in self.api.run_frame() {
            self.on_event(ev);
        }
        let mut out: Vec<BackendEvent> = self.pending.drain(..).collect();
        for ch in 0..3 {
            for m in self.api.receive_messages(ch, 64) {
                // Only people we would accept a session from get through.
                if !self.session_allowed(m.from) {
                    self.rejected_sessions += 1;
                    continue;
                }
                if let Some(lane) = Lane::from_u8(ch as u8) {
                    out.push(BackendEvent::Frame {
                        peer: m.from,
                        lane,
                        data: m.data,
                    });
                }
            }
        }
        out
    }

    fn readiness(&self) -> Readiness {
        match &self.phase {
            SteamPhase::Creating | SteamPhase::Joining { .. } => Readiness::Pending,
            SteamPhase::Hosting { .. } | SteamPhase::Guest { .. } => Readiness::Ready,
            SteamPhase::Failed(m) => Readiness::Failed(m.clone()),
        }
    }

    fn host_peer(&self) -> Option<PeerKey> {
        match self.phase {
            SteamPhase::Guest { owner, .. } => Some(owner),
            _ => None,
        }
    }

    fn disconnect(&mut self, peer: PeerKey) {
        self.api.close_session(peer);
    }
}

impl<A: SteamApi> Drop for SteamBackend<A> {
    fn drop(&mut self) {
        self.leave();
    }
}

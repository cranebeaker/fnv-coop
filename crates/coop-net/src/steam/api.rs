//! The safe Steam surface this crate needs. `RealSteam` and `MockSteam`
//! implement it; nothing above it ever sees a pointer or a Valve type.

use std::path::PathBuf;

use thiserror::Error;

/// SteamID64.
pub type SteamId = u64;
/// Lobby id (a SteamID64 of the chat type).
pub type LobbyId = u64;

/// Send flag: unreliable.
pub const SEND_UNRELIABLE: i32 = 0;
/// Send flag: no Nagle batching.
pub const SEND_NO_NAGLE: i32 = 1;
/// Send flag: reliable.
pub const SEND_RELIABLE: i32 = 8;

/// `LobbyChatUpdate_t` state flags that mean "that member is gone".
pub const CHAT_LEFT: u32 = 0x2;
/// Disconnected flag.
pub const CHAT_DISCONNECTED: u32 = 0x4;
/// Kicked flag.
pub const CHAT_KICKED: u32 = 0x8;
/// Banned flag.
pub const CHAT_BANNED: u32 = 0x10;
/// Entered flag.
pub const CHAT_ENTERED: u32 = 0x1;
/// Any flag meaning the member left.
pub const CHAT_GONE_MASK: u32 = CHAT_LEFT | CHAT_DISCONNECTED | CHAT_KICKED | CHAT_BANNED;

/// Steam failures. All are recoverable by falling back to UDP.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum SteamError {
    /// `steam_api64.dll` not found.
    #[error("steam_api64.dll not found at {0:?}; falling back to direct UDP")]
    DllMissing(PathBuf),
    /// The library exists but could not be loaded (wrong architecture, ...).
    #[error("could not load the Steam library: {0}")]
    LoadFailed(String),
    /// The DLL lacks a flat-API export (an old bundled copy).
    #[error("the Steam library does not export {0}; use the pinned steam_api64.dll")]
    MissingExport(String),
    /// The Steam client is not running.
    #[error("Steam is not running")]
    NotRunning,
    /// `SteamAPI_InitFlat` failed.
    #[error("Steam init failed (code {code}): {message}")]
    InitFailed {
        /// 1 generic, 2 Steam not running, 3 version mismatch.
        code: i32,
        /// Text from the DLL.
        message: String,
    },
    /// Steam is running but nobody is logged in.
    #[error("Steam is not logged in")]
    NotLoggedOn,
    /// Writing `steam_appid.txt` failed.
    #[error("could not write steam_appid.txt: {0}")]
    AppId(String),
}

/// A decoded Steam callback.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SteamEvent {
    /// `LobbyCreated_t` (513). Real Steam delivers it twice.
    LobbyCreated {
        /// EResult, 1 = OK.
        result: i32,
        /// New lobby.
        lobby: LobbyId,
    },
    /// `LobbyEnter_t` (504).
    LobbyEnter {
        /// Lobby.
        lobby: LobbyId,
        /// 1 = success, 2 = does not exist, 3 = not allowed, 4 = full.
        response: u32,
    },
    /// `LobbyChatUpdate_t` (506).
    LobbyChatUpdate {
        /// Lobby.
        lobby: LobbyId,
        /// Member whose state changed.
        user_changed: SteamId,
        /// Who caused it.
        making_change: SteamId,
        /// `CHAT_*` flags.
        state: u32,
    },
    /// `GameLobbyJoinRequested_t` (333): a friend invite was accepted or Join Game clicked.
    JoinRequested {
        /// Lobby to join.
        lobby: LobbyId,
        /// The friend.
        friend: SteamId,
    },
    /// `GameRichPresenceJoinRequested_t` (337).
    RichPresenceJoin {
        /// The friend.
        friend: SteamId,
        /// Their `connect` string.
        connect: String,
    },
    /// `LobbyInvite_t` (503): an invite arrived while we run.
    LobbyInvite {
        /// Who invited.
        inviter: SteamId,
        /// Lobby.
        lobby: LobbyId,
    },
    /// `SteamNetworkingMessagesSessionRequest_t` (1251).
    SessionRequest {
        /// Stranger trying to open a session.
        from: SteamId,
    },
    /// `SteamNetworkingMessagesSessionFailed_t` (1252).
    SessionFailed {
        /// Peer the session failed with.
        from: SteamId,
    },
}

/// Result of a message send.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SendResult {
    /// Queued.
    Ok,
    /// Send buffer full; wait ~5 ms and retry.
    BufferFull,
    /// Any other EResult.
    Failed(i32),
}

/// A received message.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RecvMsg {
    /// Sender.
    pub from: SteamId,
    /// Payload.
    pub data: Vec<u8>,
}

/// A friend as seen by the invite UI.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Friend {
    /// SteamID64.
    pub id: SteamId,
    /// Persona name.
    pub name: String,
    /// Persona state is not offline.
    pub online: bool,
    /// AppID the friend is playing, if any.
    pub playing_app: Option<u32>,
    /// Lobby the friend is in, 0 if none.
    pub lobby: LobbyId,
}

/// Everything the backend needs from Steam. One instance per process.
pub trait SteamApi {
    /// Our SteamID64.
    fn self_id(&self) -> SteamId;
    /// Runs the manual-dispatch loop and returns decoded callbacks.
    fn run_frame(&mut self) -> Vec<SteamEvent>;
    /// Starts creating a friends-only lobby (result arrives as `LobbyCreated`).
    fn create_lobby(&mut self, max_members: u32);
    /// Starts joining a lobby (result arrives as `LobbyEnter`).
    fn join_lobby(&mut self, lobby: LobbyId);
    /// Leaves a lobby.
    fn leave_lobby(&mut self, lobby: LobbyId);
    /// Sets lobby metadata (owner only).
    fn set_lobby_data(&mut self, lobby: LobbyId, key: &str, value: &str) -> bool;
    /// Reads lobby metadata; `None` when missing or empty.
    fn get_lobby_data(&self, lobby: LobbyId, key: &str) -> Option<String>;
    /// Opens or closes the lobby to new members.
    fn set_lobby_joinable(&mut self, lobby: LobbyId, joinable: bool);
    /// Lobby owner, 0 if unknown.
    fn lobby_owner(&self, lobby: LobbyId) -> SteamId;
    /// Current lobby members.
    fn lobby_members(&self, lobby: LobbyId) -> Vec<SteamId>;
    /// Sends a Steam invite to a friend.
    fn invite_user(&mut self, lobby: LobbyId, friend: SteamId) -> bool;
    /// Rich presence key/value.
    fn set_rich_presence(&mut self, key: &str, value: &str);
    /// Clears rich presence.
    fn clear_rich_presence(&mut self);
    /// Opens the overlay invite dialog (may not hook our renderer).
    fn open_invite_overlay(&mut self, lobby: LobbyId);
    /// Immediate friends.
    fn friends(&self) -> Vec<Friend>;
    /// `SendMessageToUser`.
    fn send_message(&mut self, to: SteamId, channel: i32, flags: i32, data: &[u8]) -> SendResult;
    /// Up to `max` messages from a local channel.
    fn receive_messages(&mut self, channel: i32, max: usize) -> Vec<RecvMsg>;
    /// Accepts a session request.
    fn accept_session(&mut self, from: SteamId) -> bool;
    /// Closes a session.
    fn close_session(&mut self, from: SteamId);
}

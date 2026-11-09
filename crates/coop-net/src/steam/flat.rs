//! The Steam flat C API, loaded at runtime. Sources: the hub playbook
//! `networking-steam-spacewar.md` (export names, callback ids, layouts) and
//! the plan in `research/07`. Nothing here is verified against a real DLL;
//! see the verification list in the final report.
//!
//! All callback and message decoding is done by pure functions over byte
//! slices so it is unit tested without a DLL.

use std::ffi::{c_char, c_int, c_void};
use std::path::Path;

use libloading::Library;

use super::api::{SteamError, SteamEvent};

/// `GameLobbyJoinRequested_t`.
pub const CB_GAME_LOBBY_JOIN_REQUESTED: i32 = 333;
/// `GameRichPresenceJoinRequested_t`.
pub const CB_RICH_PRESENCE_JOIN_REQUESTED: i32 = 337;
/// `LobbyInvite_t`.
pub const CB_LOBBY_INVITE: i32 = 503;
/// `LobbyEnter_t`.
pub const CB_LOBBY_ENTER: i32 = 504;
/// `LobbyChatUpdate_t`.
pub const CB_LOBBY_CHAT_UPDATE: i32 = 506;
/// `LobbyCreated_t`.
pub const CB_LOBBY_CREATED: i32 = 513;
/// `SteamAPICallCompleted_t`.
pub const CB_API_CALL_COMPLETED: i32 = 703;
/// `SteamNetworkingMessagesSessionRequest_t`.
pub const CB_SESSION_REQUEST: i32 = 1251;
/// `SteamNetworkingMessagesSessionFailed_t`.
pub const CB_SESSION_FAILED: i32 = 1252;

/// Size of the opaque `SteamNetworkingIdentity` (we allocate more).
pub const IDENTITY_SIZE: usize = 136;
/// Identity buffer we hand to the DLL (160 bytes, 8-aligned).
pub type IdentityBuf = [u64; 20];

/// `ESteamNetworkingConfig_SendBufferSize` (UNVERIFIED value).
pub const CFG_SEND_BUFFER_SIZE: c_int = 9;
/// `ESteamNetworkingConfig_RecvBufferSize` (UNVERIFIED value).
pub const CFG_RECV_BUFFER_SIZE: c_int = 47;
/// `ESteamNetworkingConfig_RecvMaxMessageSize` (UNVERIFIED value).
pub const CFG_RECV_MAX_MESSAGE_SIZE: c_int = 49;
/// `k_EResultOK`.
pub const RESULT_OK: c_int = 1;
/// `k_EResultLimitExceeded`, taken to mean "send buffer full" (UNVERIFIED).
pub const RESULT_LIMIT_EXCEEDED: c_int = 25;

/// `CallbackMsg_t` (pack 8).
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CallbackMsg {
    /// `HSteamUser`.
    pub user: i32,
    /// Callback id.
    pub callback: i32,
    /// Parameter bytes (valid until `FreeLastCallback`).
    pub param: *mut u8,
    /// Length of `param`.
    pub param_size: i32,
}

/// `SteamAPICallCompleted_t`.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ApiCallCompleted {
    /// `SteamAPICall_t`.
    pub call: u64,
    /// Callback id of the result.
    pub callback: i32,
    /// Size of the result.
    pub param_size: u32,
}

/// `FriendGameInfo_t`.
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct FriendGameInfo {
    /// CGameID; AppID in the low 24 bits.
    pub game_id: u64,
    /// Server IP.
    pub game_ip: u32,
    /// Server port.
    pub game_port: u16,
    /// Query port.
    pub query_port: u16,
    /// Lobby the friend is in (offset 16).
    pub lobby: u64,
}

#[cfg(target_pointer_width = "64")]
const _: () = {
    assert!(std::mem::size_of::<CallbackMsg>() == 24);
    assert!(std::mem::offset_of!(CallbackMsg, param) == 8);
    assert!(std::mem::offset_of!(CallbackMsg, param_size) == 16);
};
const _: () = {
    assert!(std::mem::size_of::<ApiCallCompleted>() == 16);
    assert!(std::mem::offset_of!(ApiCallCompleted, param_size) == 12);
    assert!(std::mem::size_of::<FriendGameInfo>() == 24);
    assert!(std::mem::offset_of!(FriendGameInfo, lobby) == 16);
    assert!(std::mem::size_of::<IdentityBuf>() >= IDENTITY_SIZE);
};

// Offsets inside callback parameter blocks (pack 8).
const LOBBY_CREATED_LOBBY: usize = 8;
const LOBBY_ENTER_RESPONSE: usize = 16;
const CHAT_UPDATE_STATE: usize = 24;

/// Minimum lengths of the callback parameter blocks we decode.
pub const LEN_LOBBY_CREATED: usize = 16;
/// `LobbyEnter_t` size.
pub const LEN_LOBBY_ENTER: usize = 24;
/// `LobbyChatUpdate_t` size.
pub const LEN_LOBBY_CHAT_UPDATE: usize = 32;

fn u64_at(b: &[u8], off: usize) -> Option<u64> {
    b.get(off..off + 8)
        .map(|s| u64::from_le_bytes(s.try_into().unwrap()))
}
fn u32_at(b: &[u8], off: usize) -> Option<u32> {
    b.get(off..off + 4)
        .map(|s| u32::from_le_bytes(s.try_into().unwrap()))
}

/// Decodes a `SteamAPICallCompleted_t` block.
pub fn decode_call_completed(b: &[u8]) -> Option<ApiCallCompleted> {
    Some(ApiCallCompleted {
        call: u64_at(b, 0)?,
        callback: u32_at(b, 8)? as i32,
        param_size: u32_at(b, 12)?,
    })
}

/// Pure fallback decoder of a `SteamNetworkingIdentity` holding a SteamID
/// (`type == 16`, `size == 8`, id at offset 8). `RealSteam` prefers the DLL's
/// own `GetSteamID64`; this exists for tests and as a cross-check.
pub fn identity_steam_id(b: &[u8]) -> u64 {
    match (u32_at(b, 0), u32_at(b, 4)) {
        (Some(16), Some(8)) => u64_at(b, 8).unwrap_or(0),
        _ => 0,
    }
}

/// Decodes one callback by id. `ident` extracts a SteamID64 from identity bytes.
pub fn decode_callback(id: i32, b: &[u8], ident: &dyn Fn(&[u8]) -> u64) -> Option<SteamEvent> {
    match id {
        CB_LOBBY_CREATED if b.len() >= LEN_LOBBY_CREATED => Some(SteamEvent::LobbyCreated {
            result: u32_at(b, 0)? as i32,
            lobby: u64_at(b, LOBBY_CREATED_LOBBY)?,
        }),
        CB_LOBBY_ENTER if b.len() >= LEN_LOBBY_ENTER => Some(SteamEvent::LobbyEnter {
            lobby: u64_at(b, 0)?,
            response: u32_at(b, LOBBY_ENTER_RESPONSE)?,
        }),
        CB_LOBBY_CHAT_UPDATE if b.len() >= LEN_LOBBY_CHAT_UPDATE => {
            Some(SteamEvent::LobbyChatUpdate {
                lobby: u64_at(b, 0)?,
                user_changed: u64_at(b, 8)?,
                making_change: u64_at(b, 16)?,
                state: u32_at(b, CHAT_UPDATE_STATE)?,
            })
        }
        CB_GAME_LOBBY_JOIN_REQUESTED if b.len() >= 16 => Some(SteamEvent::JoinRequested {
            lobby: u64_at(b, 0)?,
            friend: u64_at(b, 8)?,
        }),
        CB_RICH_PRESENCE_JOIN_REQUESTED if b.len() >= 16 => {
            let s = &b[8..];
            let end = s.iter().position(|&c| c == 0).unwrap_or(s.len()).min(256);
            Some(SteamEvent::RichPresenceJoin {
                friend: u64_at(b, 0)?,
                connect: String::from_utf8_lossy(&s[..end]).into_owned(),
            })
        }
        CB_LOBBY_INVITE if b.len() >= 16 => Some(SteamEvent::LobbyInvite {
            inviter: u64_at(b, 0)?,
            lobby: u64_at(b, 8)?,
        }),
        CB_SESSION_REQUEST if b.len() >= IDENTITY_SIZE => Some(SteamEvent::SessionRequest {
            from: ident(&b[..IDENTITY_SIZE]),
        }),
        CB_SESSION_FAILED if b.len() >= IDENTITY_SIZE => Some(SteamEvent::SessionFailed {
            from: ident(&b[..IDENTITY_SIZE]),
        }),
        _ => None,
    }
}

/// Sender-identity offsets inside `SteamNetworkingMessage_t`, in probe order:
/// 16 (x64 per the playbook; also what 8-byte packing suggests) then 12 (what the
/// 32-bit DLL really uses, confirmed between two accounts in Jericho).
pub const SENDER_OFFSET_CANDIDATES: [usize; 2] = [16, 12];
/// `data` offset in `SteamNetworkingMessage_t` (x64).
pub const MSG_DATA_OFFSET: usize = 0;
/// `size` offset on x64 (4 on x86).
pub const MSG_SIZE_OFFSET: usize = 8;

/// Probes `msg` (the message struct bytes) for a non-zero sender SteamID,
/// trying `preferred` first, then the candidates. Returns `(offset, id)`.
pub fn probe_sender(
    msg: &[u8],
    preferred: Option<usize>,
    ident: &dyn Fn(&[u8]) -> u64,
) -> Option<(usize, u64)> {
    let order = preferred
        .into_iter()
        .chain(SENDER_OFFSET_CANDIDATES.iter().copied());
    for off in order {
        if let Some(slice) = msg.get(off..off + IDENTITY_SIZE) {
            let id = ident(slice);
            if id != 0 {
                return Some((off, id));
            }
        }
    }
    None
}

type P = *mut c_void;

/// Resolved flat-API function pointers plus the library that owns them.
#[allow(missing_docs)]
pub struct FlatLib {
    pub is_steam_running: unsafe extern "C" fn() -> bool,
    pub init_flat: unsafe extern "C" fn(*mut c_char) -> c_int,
    pub get_hsteam_pipe: unsafe extern "C" fn() -> i32,
    pub shutdown: unsafe extern "C" fn(),
    pub md_init: unsafe extern "C" fn(),
    pub md_run_frame: unsafe extern "C" fn(i32),
    pub md_get_next: unsafe extern "C" fn(i32, *mut CallbackMsg) -> bool,
    pub md_free_last: unsafe extern "C" fn(i32),
    pub md_get_result: unsafe extern "C" fn(i32, u64, *mut c_void, c_int, c_int, *mut bool) -> bool,
    pub steam_user: unsafe extern "C" fn() -> P,
    pub steam_matchmaking: unsafe extern "C" fn() -> P,
    pub steam_friends: unsafe extern "C" fn() -> P,
    pub steam_messages: unsafe extern "C" fn() -> P,
    pub steam_utils: unsafe extern "C" fn() -> P,
    pub user_get_steam_id: unsafe extern "C" fn(P) -> u64,
    pub user_logged_on: unsafe extern "C" fn(P) -> bool,
    pub mm_create_lobby: unsafe extern "C" fn(P, c_int, c_int) -> u64,
    pub mm_join_lobby: unsafe extern "C" fn(P, u64) -> u64,
    pub mm_leave_lobby: unsafe extern "C" fn(P, u64),
    pub mm_set_lobby_data: unsafe extern "C" fn(P, u64, *const c_char, *const c_char) -> bool,
    pub mm_get_lobby_data: unsafe extern "C" fn(P, u64, *const c_char) -> *const c_char,
    pub mm_set_joinable: unsafe extern "C" fn(P, u64, bool) -> bool,
    pub mm_get_owner: unsafe extern "C" fn(P, u64) -> u64,
    pub mm_num_members: unsafe extern "C" fn(P, u64) -> c_int,
    pub mm_member_by_index: unsafe extern "C" fn(P, u64, c_int) -> u64,
    pub mm_invite: unsafe extern "C" fn(P, u64, u64) -> bool,
    pub fr_set_rich_presence: unsafe extern "C" fn(P, *const c_char, *const c_char) -> bool,
    pub fr_clear_rich_presence: unsafe extern "C" fn(P),
    pub fr_overlay_invite: unsafe extern "C" fn(P, u64),
    pub fr_friend_count: unsafe extern "C" fn(P, c_int) -> c_int,
    pub fr_friend_by_index: unsafe extern "C" fn(P, c_int, c_int) -> u64,
    pub fr_persona_state: unsafe extern "C" fn(P, u64) -> c_int,
    pub fr_persona_name: unsafe extern "C" fn(P, u64) -> *const c_char,
    pub fr_game_played: unsafe extern "C" fn(P, u64, *mut FriendGameInfo) -> bool,
    pub ident_set_steam_id: unsafe extern "C" fn(*mut c_void, u64),
    pub ident_get_steam_id: unsafe extern "C" fn(*const c_void) -> u64,
    pub msgs_send:
        unsafe extern "C" fn(P, *const c_void, *const c_void, u32, c_int, c_int) -> c_int,
    pub msgs_receive: unsafe extern "C" fn(P, c_int, *mut *mut c_void, c_int) -> c_int,
    pub msgs_accept: unsafe extern "C" fn(P, *const c_void) -> bool,
    pub msgs_close: unsafe extern "C" fn(P, *const c_void) -> bool,
    pub msg_release: unsafe extern "C" fn(*mut c_void),
    pub utils_set_i32: unsafe extern "C" fn(P, c_int, i32) -> bool,
    _lib: Library,
}

unsafe fn sym<T: Copy>(lib: &Library, name: &str) -> Result<T, SteamError> {
    let mut n = name.as_bytes().to_vec();
    n.push(0);
    match lib.get::<T>(&n) {
        Ok(s) => Ok(*s),
        Err(_) => Err(SteamError::MissingExport(name.to_string())),
    }
}

impl FlatLib {
    /// Loads the DLL at `path` and resolves every export.
    /// A missing file is `DllMissing`; a missing export is `MissingExport`.
    pub fn load(path: &Path) -> Result<FlatLib, SteamError> {
        if !path.exists() {
            return Err(SteamError::DllMissing(path.to_path_buf()));
        }
        // SAFETY: loading Valve's DLL runs its DllMain; that is the point.
        let lib =
            unsafe { Library::new(path) }.map_err(|e| SteamError::LoadFailed(e.to_string()))?;
        // SAFETY: every symbol type below matches the flat API declarations.
        unsafe {
            Ok(FlatLib {
                is_steam_running: sym(&lib, "SteamAPI_IsSteamRunning")?,
                init_flat: sym(&lib, "SteamAPI_InitFlat")?,
                get_hsteam_pipe: sym(&lib, "SteamAPI_GetHSteamPipe")?,
                shutdown: sym(&lib, "SteamAPI_Shutdown")?,
                md_init: sym(&lib, "SteamAPI_ManualDispatch_Init")?,
                md_run_frame: sym(&lib, "SteamAPI_ManualDispatch_RunFrame")?,
                md_get_next: sym(&lib, "SteamAPI_ManualDispatch_GetNextCallback")?,
                md_free_last: sym(&lib, "SteamAPI_ManualDispatch_FreeLastCallback")?,
                md_get_result: sym(&lib, "SteamAPI_ManualDispatch_GetAPICallResult")?,
                steam_user: sym(&lib, "SteamAPI_SteamUser_v023")?,
                steam_matchmaking: sym(&lib, "SteamAPI_SteamMatchmaking_v009")?,
                steam_friends: sym(&lib, "SteamAPI_SteamFriends_v018")?,
                steam_messages: sym(&lib, "SteamAPI_SteamNetworkingMessages_SteamAPI_v002")?,
                steam_utils: sym(&lib, "SteamAPI_SteamNetworkingUtils_SteamAPI_v004")?,
                user_get_steam_id: sym(&lib, "SteamAPI_ISteamUser_GetSteamID")?,
                user_logged_on: sym(&lib, "SteamAPI_ISteamUser_BLoggedOn")?,
                mm_create_lobby: sym(&lib, "SteamAPI_ISteamMatchmaking_CreateLobby")?,
                mm_join_lobby: sym(&lib, "SteamAPI_ISteamMatchmaking_JoinLobby")?,
                mm_leave_lobby: sym(&lib, "SteamAPI_ISteamMatchmaking_LeaveLobby")?,
                mm_set_lobby_data: sym(&lib, "SteamAPI_ISteamMatchmaking_SetLobbyData")?,
                mm_get_lobby_data: sym(&lib, "SteamAPI_ISteamMatchmaking_GetLobbyData")?,
                mm_set_joinable: sym(&lib, "SteamAPI_ISteamMatchmaking_SetLobbyJoinable")?,
                mm_get_owner: sym(&lib, "SteamAPI_ISteamMatchmaking_GetLobbyOwner")?,
                mm_num_members: sym(&lib, "SteamAPI_ISteamMatchmaking_GetNumLobbyMembers")?,
                mm_member_by_index: sym(&lib, "SteamAPI_ISteamMatchmaking_GetLobbyMemberByIndex")?,
                mm_invite: sym(&lib, "SteamAPI_ISteamMatchmaking_InviteUserToLobby")?,
                fr_set_rich_presence: sym(&lib, "SteamAPI_ISteamFriends_SetRichPresence")?,
                fr_clear_rich_presence: sym(&lib, "SteamAPI_ISteamFriends_ClearRichPresence")?,
                fr_overlay_invite: sym(
                    &lib,
                    "SteamAPI_ISteamFriends_ActivateGameOverlayInviteDialog",
                )?,
                fr_friend_count: sym(&lib, "SteamAPI_ISteamFriends_GetFriendCount")?,
                fr_friend_by_index: sym(&lib, "SteamAPI_ISteamFriends_GetFriendByIndex")?,
                fr_persona_state: sym(&lib, "SteamAPI_ISteamFriends_GetFriendPersonaState")?,
                fr_persona_name: sym(&lib, "SteamAPI_ISteamFriends_GetFriendPersonaName")?,
                fr_game_played: sym(&lib, "SteamAPI_ISteamFriends_GetFriendGamePlayed")?,
                ident_set_steam_id: sym(&lib, "SteamAPI_SteamNetworkingIdentity_SetSteamID64")?,
                ident_get_steam_id: sym(&lib, "SteamAPI_SteamNetworkingIdentity_GetSteamID64")?,
                msgs_send: sym(&lib, "SteamAPI_ISteamNetworkingMessages_SendMessageToUser")?,
                msgs_receive: sym(
                    &lib,
                    "SteamAPI_ISteamNetworkingMessages_ReceiveMessagesOnChannel",
                )?,
                msgs_accept: sym(
                    &lib,
                    "SteamAPI_ISteamNetworkingMessages_AcceptSessionWithUser",
                )?,
                msgs_close: sym(
                    &lib,
                    "SteamAPI_ISteamNetworkingMessages_CloseSessionWithUser",
                )?,
                msg_release: sym(&lib, "SteamAPI_SteamNetworkingMessage_t_Release")?,
                utils_set_i32: sym(
                    &lib,
                    "SteamAPI_ISteamNetworkingUtils_SetGlobalConfigValueInt32",
                )?,
                _lib: lib,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ident_buf(id: u64) -> Vec<u8> {
        let mut b = vec![0u8; IDENTITY_SIZE];
        b[0..4].copy_from_slice(&16u32.to_le_bytes());
        b[4..8].copy_from_slice(&8u32.to_le_bytes());
        b[8..16].copy_from_slice(&id.to_le_bytes());
        b
    }

    #[test]
    fn missing_dll_is_a_clean_error() {
        let p = std::path::PathBuf::from("definitely/not/here/steam_api64.dll");
        match FlatLib::load(&p) {
            Err(SteamError::DllMissing(x)) => assert_eq!(x, p),
            Err(e) => panic!("wrong error {e:?}"),
            Ok(_) => panic!("loaded a missing dll"),
        }
    }

    #[test]
    fn not_a_dll_is_load_failed() {
        let dir = std::env::temp_dir().join("coop_net_flat_test");
        std::fs::create_dir_all(&dir).unwrap();
        let p = dir.join("steam_api64.dll");
        std::fs::write(&p, b"not a library").unwrap();
        assert!(matches!(FlatLib::load(&p), Err(SteamError::LoadFailed(_))));
        let _ = std::fs::remove_file(&p);
    }

    #[test]
    fn decode_lobby_callbacks() {
        let mut b = vec![0u8; 16];
        b[0..4].copy_from_slice(&1u32.to_le_bytes());
        b[8..16].copy_from_slice(&0x1234_5678_9abc_def0u64.to_le_bytes());
        assert_eq!(
            decode_callback(CB_LOBBY_CREATED, &b, &identity_steam_id),
            Some(SteamEvent::LobbyCreated {
                result: 1,
                lobby: 0x1234_5678_9abc_def0
            })
        );

        let mut e = vec![0u8; 24];
        e[0..8].copy_from_slice(&77u64.to_le_bytes());
        e[16..20].copy_from_slice(&1u32.to_le_bytes());
        assert_eq!(
            decode_callback(CB_LOBBY_ENTER, &e, &identity_steam_id),
            Some(SteamEvent::LobbyEnter {
                lobby: 77,
                response: 1
            })
        );

        let mut c = vec![0u8; 32];
        c[0..8].copy_from_slice(&5u64.to_le_bytes());
        c[8..16].copy_from_slice(&6u64.to_le_bytes());
        c[16..24].copy_from_slice(&7u64.to_le_bytes());
        c[24..28].copy_from_slice(&4u32.to_le_bytes());
        assert_eq!(
            decode_callback(CB_LOBBY_CHAT_UPDATE, &c, &identity_steam_id),
            Some(SteamEvent::LobbyChatUpdate {
                lobby: 5,
                user_changed: 6,
                making_change: 7,
                state: 4
            })
        );
        // Short blocks are ignored, not panicked on.
        assert_eq!(
            decode_callback(CB_LOBBY_ENTER, &e[..10], &identity_steam_id),
            None
        );
        assert_eq!(decode_callback(9999, &e, &identity_steam_id), None);
    }

    #[test]
    fn decode_join_and_rich_presence() {
        let mut b = vec![0u8; 16];
        b[0..8].copy_from_slice(&11u64.to_le_bytes());
        b[8..16].copy_from_slice(&22u64.to_le_bytes());
        assert_eq!(
            decode_callback(CB_GAME_LOBBY_JOIN_REQUESTED, &b, &identity_steam_id),
            Some(SteamEvent::JoinRequested {
                lobby: 11,
                friend: 22
            })
        );
        assert_eq!(
            decode_callback(CB_LOBBY_INVITE, &b, &identity_steam_id),
            Some(SteamEvent::LobbyInvite {
                inviter: 11,
                lobby: 22
            })
        );
        let mut r = vec![0u8; 264];
        r[0..8].copy_from_slice(&9u64.to_le_bytes());
        let s = b"+connect_lobby 12345";
        r[8..8 + s.len()].copy_from_slice(s);
        assert_eq!(
            decode_callback(CB_RICH_PRESENCE_JOIN_REQUESTED, &r, &identity_steam_id),
            Some(SteamEvent::RichPresenceJoin {
                friend: 9,
                connect: "+connect_lobby 12345".into()
            })
        );
    }

    #[test]
    fn decode_session_callbacks_use_identity_at_zero() {
        let mut b = ident_buf(4242);
        b.resize(IDENTITY_SIZE + 8, 0);
        assert_eq!(
            decode_callback(CB_SESSION_REQUEST, &b, &identity_steam_id),
            Some(SteamEvent::SessionRequest { from: 4242 })
        );
        assert_eq!(
            decode_callback(CB_SESSION_FAILED, &b, &identity_steam_id),
            Some(SteamEvent::SessionFailed { from: 4242 })
        );
    }

    #[test]
    fn call_completed_layout() {
        let mut b = vec![0u8; 16];
        b[0..8].copy_from_slice(&99u64.to_le_bytes());
        b[8..12].copy_from_slice(&513i32.to_le_bytes());
        b[12..16].copy_from_slice(&16u32.to_le_bytes());
        assert_eq!(
            decode_call_completed(&b),
            Some(ApiCallCompleted {
                call: 99,
                callback: 513,
                param_size: 16
            })
        );
        assert_eq!(decode_call_completed(&b[..8]), None);
    }

    #[test]
    fn sender_probe_prefers_whichever_decodes() {
        // x64-style: identity at 16.
        let mut m = vec![0u8; 200];
        m[16..16 + IDENTITY_SIZE].copy_from_slice(&ident_buf(777));
        assert_eq!(probe_sender(&m, None, &identity_steam_id), Some((16, 777)));
        // x86-style: identity at 12; reading 16 yields 0 and the probe moves on.
        let mut m = vec![0u8; 200];
        m[12..12 + IDENTITY_SIZE].copy_from_slice(&ident_buf(888));
        assert_eq!(probe_sender(&m, None, &identity_steam_id), Some((12, 888)));
        // A remembered winner is tried first.
        assert_eq!(
            probe_sender(&m, Some(12), &identity_steam_id),
            Some((12, 888))
        );
        // Neither: no sender, the message must be dropped and logged by the caller.
        assert_eq!(probe_sender(&[0u8; 200], None, &identity_steam_id), None);
        assert_eq!(probe_sender(&[0u8; 10], None, &identity_steam_id), None);
    }
}

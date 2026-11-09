//! `RealSteam`: [`SteamApi`] over the runtime-loaded `steam_api64.dll`.
//!
//! Never exercised in automated tests (no DLL, no Steam). Everything that can
//! be tested without the DLL lives in [`super::flat`] as pure functions.

use std::ffi::{CStr, CString};
use std::path::{Path, PathBuf};

use super::api::*;
use super::flat::*;

/// Steam client AppID used for the Spacewar sample app.
pub const SPACEWAR_APPID: &str = "480";

/// Writes `steam_appid.txt` (480) into each directory if missing.
pub fn write_appid_files(dirs: &[&Path]) -> Result<(), SteamError> {
    for d in dirs {
        let p = d.join("steam_appid.txt");
        if !p.exists() {
            std::fs::write(&p, SPACEWAR_APPID)
                .map_err(|e| SteamError::AppId(format!("{}: {e}", p.display())))?;
        }
    }
    Ok(())
}

/// Default DLL location: next to the executable, else the working directory.
pub fn default_dll_path() -> PathBuf {
    let name = if cfg!(windows) {
        "steam_api64.dll"
    } else {
        "libsteam_api.so"
    };
    if let Some(dir) = std::env::current_exe()
        .ok()
        .and_then(|e| e.parent().map(|p| p.to_path_buf()))
    {
        let p = dir.join(name);
        if p.exists() {
            return p;
        }
    }
    PathBuf::from(name)
}

/// Loads and initialises Steam. On any error the caller falls back to UDP.
pub fn try_init(dll: Option<&Path>) -> Result<RealSteam, SteamError> {
    let path = dll
        .map(|p| p.to_path_buf())
        .unwrap_or_else(default_dll_path);
    let lib = FlatLib::load(&path)?;
    // SAFETY: plain flat-API calls with valid arguments.
    unsafe {
        if !(lib.is_steam_running)() {
            return Err(SteamError::NotRunning);
        }
        let mut dirs: Vec<PathBuf> = Vec::new();
        if let Some(d) = std::env::current_exe()
            .ok()
            .and_then(|e| e.parent().map(|p| p.to_path_buf()))
        {
            dirs.push(d);
        }
        if let Ok(d) = std::env::current_dir() {
            if !dirs.contains(&d) {
                dirs.push(d);
            }
        }
        let refs: Vec<&Path> = dirs.iter().map(|p| p.as_path()).collect();
        write_appid_files(&refs)?;

        let mut err = [0 as std::ffi::c_char; 1024];
        let code = (lib.init_flat)(err.as_mut_ptr());
        if code != 0 {
            let message = CStr::from_ptr(err.as_ptr()).to_string_lossy().into_owned();
            return Err(SteamError::InitFailed { code, message });
        }
        (lib.md_init)();
        let user = (lib.steam_user)();
        if user.is_null() || !(lib.user_logged_on)(user) {
            (lib.shutdown)();
            return Err(SteamError::NotLoggedOn);
        }
        let mm = (lib.steam_matchmaking)();
        let fr = (lib.steam_friends)();
        let msgs = (lib.steam_messages)();
        let utils = (lib.steam_utils)();
        let me = (lib.user_get_steam_id)(user);
        let pipe = (lib.get_hsteam_pipe)();
        // Large messages: 32 MB buffers. A failure here is not fatal.
        let mut cfg_ok = true;
        for c in [
            CFG_SEND_BUFFER_SIZE,
            CFG_RECV_BUFFER_SIZE,
            CFG_RECV_MAX_MESSAGE_SIZE,
        ] {
            cfg_ok &= (lib.utils_set_i32)(utils, c, 32 * 1024 * 1024);
        }
        Ok(RealSteam {
            lib,
            mm,
            fr,
            msgs,
            pipe,
            me,
            sender_offset: None,
            config_ok: cfg_ok,
            probe_failures: 0,
        })
    }
}

/// [`SteamApi`] over the DLL.
pub struct RealSteam {
    lib: FlatLib,
    mm: *mut std::ffi::c_void,
    fr: *mut std::ffi::c_void,
    msgs: *mut std::ffi::c_void,
    pipe: i32,
    me: SteamId,
    sender_offset: Option<usize>,
    config_ok: bool,
    /// Messages dropped because no candidate offset decoded to a SteamID.
    pub probe_failures: u64,
}

impl RealSteam {
    /// The sender-identity offset that decoded, once a message has arrived.
    /// Check this in the two-account test log: 16 expected on x64.
    pub fn sender_offset_won(&self) -> Option<usize> {
        self.sender_offset
    }

    /// False if setting the 32 MB buffer sizes failed (config ids unverified).
    pub fn buffer_config_applied(&self) -> bool {
        self.config_ok
    }

    fn ident_of(&self, id: u64) -> IdentityBuf {
        let mut b: IdentityBuf = [0; 20];
        // SAFETY: b is a 160-byte, 8-aligned buffer, larger than the identity.
        unsafe { (self.lib.ident_set_steam_id)(b.as_mut_ptr() as *mut _, id) };
        b
    }

    fn decode_ident(&self, bytes: &[u8]) -> u64 {
        let mut b: IdentityBuf = [0; 20];
        let n = bytes.len().min(IDENTITY_SIZE);
        // SAFETY: copying n <= 136 bytes into a 160-byte buffer, then asking the DLL to read it.
        unsafe {
            std::ptr::copy_nonoverlapping(bytes.as_ptr(), b.as_mut_ptr() as *mut u8, n);
            (self.lib.ident_get_steam_id)(b.as_ptr() as *const _)
        }
    }

    fn cstr(s: &str) -> CString {
        CString::new(s.replace('\0', "")).unwrap_or_default()
    }
}

impl SteamApi for RealSteam {
    fn self_id(&self) -> SteamId {
        self.me
    }

    fn run_frame(&mut self) -> Vec<SteamEvent> {
        let mut out = Vec::new();
        // SAFETY: standard manual-dispatch loop; param bytes are copied before FreeLastCallback.
        unsafe {
            (self.lib.md_run_frame)(self.pipe);
            let mut msg = CallbackMsg {
                user: 0,
                callback: 0,
                param: std::ptr::null_mut(),
                param_size: 0,
            };
            while (self.lib.md_get_next)(self.pipe, &mut msg) {
                let params: Vec<u8> = if msg.param.is_null() || msg.param_size <= 0 {
                    Vec::new()
                } else {
                    std::slice::from_raw_parts(msg.param, msg.param_size as usize).to_vec()
                };
                let ident = |b: &[u8]| self.decode_ident(b);
                if msg.callback == CB_API_CALL_COMPLETED {
                    if let Some(done) = decode_call_completed(&params) {
                        if done.param_size <= 1 << 20 {
                            let mut buf = vec![0u8; done.param_size as usize];
                            let mut failed = false;
                            let ok = (self.lib.md_get_result)(
                                self.pipe,
                                done.call,
                                buf.as_mut_ptr() as *mut _,
                                buf.len() as i32,
                                done.callback,
                                &mut failed,
                            );
                            if ok && !failed {
                                out.extend(decode_callback(done.callback, &buf, &ident));
                            }
                        }
                    }
                } else {
                    out.extend(decode_callback(msg.callback, &params, &ident));
                }
                (self.lib.md_free_last)(self.pipe);
            }
        }
        out
    }

    fn create_lobby(&mut self, max_members: u32) {
        // Type 1 = friends-only. The call handle is ignored: the result comes
        // through callback 703 and the 513 broadcast.
        unsafe { (self.lib.mm_create_lobby)(self.mm, 1, max_members as i32) };
    }

    fn join_lobby(&mut self, lobby: LobbyId) {
        unsafe { (self.lib.mm_join_lobby)(self.mm, lobby) };
    }

    fn leave_lobby(&mut self, lobby: LobbyId) {
        unsafe { (self.lib.mm_leave_lobby)(self.mm, lobby) };
    }

    fn set_lobby_data(&mut self, lobby: LobbyId, key: &str, value: &str) -> bool {
        let (k, v) = (Self::cstr(key), Self::cstr(value));
        unsafe { (self.lib.mm_set_lobby_data)(self.mm, lobby, k.as_ptr(), v.as_ptr()) }
    }

    fn get_lobby_data(&self, lobby: LobbyId, key: &str) -> Option<String> {
        let k = Self::cstr(key);
        // SAFETY: returns a NUL-terminated string owned by Steam, copied immediately.
        unsafe {
            let p = (self.lib.mm_get_lobby_data)(self.mm, lobby, k.as_ptr());
            if p.is_null() {
                return None;
            }
            let s = CStr::from_ptr(p).to_string_lossy().into_owned();
            (!s.is_empty()).then_some(s)
        }
    }

    fn set_lobby_joinable(&mut self, lobby: LobbyId, joinable: bool) {
        unsafe { (self.lib.mm_set_joinable)(self.mm, lobby, joinable) };
    }

    fn lobby_owner(&self, lobby: LobbyId) -> SteamId {
        unsafe { (self.lib.mm_get_owner)(self.mm, lobby) }
    }

    fn lobby_members(&self, lobby: LobbyId) -> Vec<SteamId> {
        unsafe {
            let n = (self.lib.mm_num_members)(self.mm, lobby).max(0);
            (0..n)
                .map(|i| (self.lib.mm_member_by_index)(self.mm, lobby, i))
                .collect()
        }
    }

    fn invite_user(&mut self, lobby: LobbyId, friend: SteamId) -> bool {
        unsafe { (self.lib.mm_invite)(self.mm, lobby, friend) }
    }

    fn set_rich_presence(&mut self, key: &str, value: &str) {
        let (k, v) = (Self::cstr(key), Self::cstr(value));
        unsafe { (self.lib.fr_set_rich_presence)(self.fr, k.as_ptr(), v.as_ptr()) };
    }

    fn clear_rich_presence(&mut self) {
        unsafe { (self.lib.fr_clear_rich_presence)(self.fr) };
    }

    fn open_invite_overlay(&mut self, lobby: LobbyId) {
        unsafe { (self.lib.fr_overlay_invite)(self.fr, lobby) };
    }

    fn friends(&self) -> Vec<Friend> {
        // flag 4 = immediate friends
        unsafe {
            let n = (self.lib.fr_friend_count)(self.fr, 4).max(0);
            (0..n)
                .map(|i| {
                    let id = (self.lib.fr_friend_by_index)(self.fr, i, 4);
                    let online = (self.lib.fr_persona_state)(self.fr, id) != 0;
                    let name_p = (self.lib.fr_persona_name)(self.fr, id);
                    let name = if name_p.is_null() {
                        String::new()
                    } else {
                        CStr::from_ptr(name_p).to_string_lossy().into_owned()
                    };
                    let mut info = FriendGameInfo::default();
                    let playing = (self.lib.fr_game_played)(self.fr, id, &mut info);
                    let (playing_app, lobby) = if playing {
                        (Some((info.game_id & 0xFF_FFFF) as u32), info.lobby)
                    } else {
                        (None, 0)
                    };
                    Friend {
                        id,
                        name,
                        online,
                        playing_app,
                        lobby,
                    }
                })
                .collect()
        }
    }

    fn send_message(&mut self, to: SteamId, channel: i32, flags: i32, data: &[u8]) -> SendResult {
        let ident = self.ident_of(to);
        // SAFETY: ident and data outlive the call; Steam copies the payload.
        let r = unsafe {
            (self.lib.msgs_send)(
                self.msgs,
                ident.as_ptr() as *const _,
                data.as_ptr() as *const _,
                data.len() as u32,
                flags,
                channel,
            )
        };
        match r {
            RESULT_OK => SendResult::Ok,
            RESULT_LIMIT_EXCEEDED => SendResult::BufferFull,
            other => SendResult::Failed(other),
        }
    }

    fn receive_messages(&mut self, channel: i32, max: usize) -> Vec<RecvMsg> {
        let max = max.clamp(1, 64);
        let mut ptrs: [*mut std::ffi::c_void; 64] = [std::ptr::null_mut(); 64];
        let mut out = Vec::new();
        // SAFETY: Steam fills `n` message pointers; each is read within the
        // struct's known size (>= 168 bytes on x64) and released exactly once.
        unsafe {
            let n = (self.lib.msgs_receive)(self.msgs, channel, ptrs.as_mut_ptr(), max as i32)
                .max(0) as usize;
            for &p in ptrs.iter().take(n) {
                if p.is_null() {
                    continue;
                }
                let base = p as *const u8;
                let data_ptr =
                    std::ptr::read_unaligned(base.add(MSG_DATA_OFFSET) as *const *const u8);
                let size = std::ptr::read_unaligned(base.add(MSG_SIZE_OFFSET) as *const i32);
                let head = std::slice::from_raw_parts(base, 16 + IDENTITY_SIZE + 8);
                let ident = |b: &[u8]| self.decode_ident(b);
                match probe_sender(head, self.sender_offset, &ident) {
                    Some((off, from)) => {
                        if self.sender_offset != Some(off) {
                            eprintln!(
                                "coop-net steam: message sender identity decoded at offset {off}"
                            );
                            self.sender_offset = Some(off);
                        }
                        let data = if data_ptr.is_null() || size < 0 {
                            Vec::new()
                        } else {
                            std::slice::from_raw_parts(data_ptr, size as usize).to_vec()
                        };
                        out.push(RecvMsg { from, data });
                    }
                    None => {
                        self.probe_failures += 1;
                        if self.probe_failures == 1 {
                            eprintln!("coop-net steam: no candidate offset decoded a sender SteamID; dropping messages");
                        }
                    }
                }
                (self.lib.msg_release)(p);
            }
        }
        out
    }

    fn accept_session(&mut self, from: SteamId) -> bool {
        let ident = self.ident_of(from);
        unsafe { (self.lib.msgs_accept)(self.msgs, ident.as_ptr() as *const _) }
    }

    fn close_session(&mut self, from: SteamId) {
        let ident = self.ident_of(from);
        unsafe { (self.lib.msgs_close)(self.msgs, ident.as_ptr() as *const _) };
    }
}

impl Drop for RealSteam {
    fn drop(&mut self) {
        unsafe { (self.lib.shutdown)() };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_init_with_missing_dll_reports_and_allows_fallback() {
        let r = try_init(Some(Path::new("nope/steam_api64.dll")));
        match r {
            Err(e @ SteamError::DllMissing(_)) => assert!(e.to_string().contains("falling back")),
            other => panic!("expected DllMissing, got {:?}", other.err()),
        }
    }

    #[test]
    fn appid_file_written_once() {
        let dir = std::env::temp_dir().join("coop_net_appid_test");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        write_appid_files(&[&dir]).unwrap();
        assert_eq!(
            std::fs::read_to_string(dir.join("steam_appid.txt")).unwrap(),
            "480"
        );
        std::fs::write(dir.join("steam_appid.txt"), "999").unwrap();
        write_appid_files(&[&dir]).unwrap();
        assert_eq!(
            std::fs::read_to_string(dir.join("steam_appid.txt")).unwrap(),
            "999"
        );
        let _ = std::fs::remove_dir_all(&dir);
    }
}

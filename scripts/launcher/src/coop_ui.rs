//! The co-op window (docs/coop/SPECS.md S38): `Play Co-op.exe` (the package's second copy of
//! Play.exe) or `Play.exe --coop` shows it before the game starts. The player types a name, a
//! join code or the address for their own code, picks voice chat and its key and the parts that
//! are on unless unticked, and presses Play alone, Host, Join, Invite a Steam friend or Join a
//! Steam friend. The choices are kept in `userdata\coop.ini` ([`crate::coop`]). Once a host's game
//! has written its join code to its log, [`CodeWatch`] puts it on the clipboard and shows it.
//!
//! Plain Win32 controls in the style of `ui.rs`, no dependencies. With `NV_RS_NO_WINDOWS=1`
//! nothing is shown and the environment stays as it is (as `Play Co-op.cmd` sets it).

use crate::coop::{self, Choice, Mode, PlayerVoice, Voice, LIST_KEYS, PTT_KEYS};
use crate::win::{from_wide, wide_str};
use std::cell::RefCell;
use std::ffi::c_void;
use std::path::{Path, PathBuf};

#[link(name = "user32")]
extern "system" {
    fn RegisterClassW(class: *const WndClassW) -> u16;
    fn CreateWindowExW(
        ex_style: u32,
        class: *const u16,
        title: *const u16,
        style: u32,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        parent: *mut c_void,
        menu: *mut c_void,
        instance: *mut c_void,
        param: *mut c_void,
    ) -> *mut c_void;
    fn DefWindowProcW(window: *mut c_void, message: u32, w: usize, l: isize) -> isize;
    fn GetMessageW(message: *mut [usize; 8], window: *mut c_void, min: u32, max: u32) -> i32;
    fn IsDialogMessageW(window: *mut c_void, message: *mut [usize; 8]) -> i32;
    fn TranslateMessage(message: *const [usize; 8]) -> i32;
    fn DispatchMessageW(message: *const [usize; 8]) -> isize;
    fn PostQuitMessage(code: i32);
    fn DestroyWindow(window: *mut c_void) -> i32;
    fn GetDlgItem(window: *mut c_void, id: i32) -> *mut c_void;
    fn GetWindowTextW(window: *mut c_void, text: *mut u16, max: i32) -> i32;
    fn SendMessageW(window: *mut c_void, message: u32, w: usize, l: isize) -> isize;
    fn EnableWindow(window: *mut c_void, enable: i32) -> i32;
    fn SetForegroundWindow(window: *mut c_void) -> i32;
    fn MessageBoxW(owner: *mut c_void, text: *const u16, title: *const u16, kind: u32) -> i32;
    fn GetSystemMetrics(index: i32) -> i32;
    fn GetSysColorBrush(index: i32) -> *mut c_void;
}
#[link(name = "gdi32")]
extern "system" {
    fn GetStockObject(index: i32) -> *mut c_void;
}
#[link(name = "kernel32")]
extern "system" {
    fn GetModuleHandleW(name: *const u16) -> *mut c_void;
}

#[repr(C)]
struct WndClassW {
    style: u32,
    procedure: unsafe extern "system" fn(*mut c_void, u32, usize, isize) -> isize,
    class_extra: i32,
    window_extra: i32,
    instance: *mut c_void,
    icon: *mut c_void,
    cursor: *mut c_void,
    background: *mut c_void,
    menu_name: *const u16,
    class_name: *const u16,
}

const TITLE: &str = "FNV Co-Op";

// Control ids.
const NAME: i32 = 101;
const CODE: i32 = 102;
const ADDRESS: i32 = 103;
const STEAM: i32 = 104;
const VOICE: i32 = 105;
const KEY: i32 = 106;
const ITEMS: i32 = 107;
const VATS: i32 = 108;
const HARM: i32 = 109;
// voiceui: the players list's keys, and each known player's mute and volume.
const PLAYERS_KEY: i32 = 110;
const MUTE_KEY: i32 = 111;
const DEAFEN_KEY: i32 = 112;
const PLAYER_MUTE: i32 = 300;
const PLAYER_VOLUME: i32 = 340;
/// The players the window lists at most (the others keep what the file says).
const SHOWN_PLAYERS: usize = 6;
const ALONE: i32 = 201;
const HOST: i32 = 202;
const JOIN: i32 = 203;
const INVITE: i32 = 204;
const STEAM_JOIN: i32 = 205;

// Messages, styles.
const WM_DESTROY: u32 = 0x0002;
const WM_CLOSE: u32 = 0x0010;
const WM_SETFONT: u32 = 0x0030;
const WM_COMMAND: u32 = 0x0111;
const BM_GETCHECK: u32 = 0x00f0;
const BM_SETCHECK: u32 = 0x00f1;
const CB_ADDSTRING: u32 = 0x0143;
const CB_GETCURSEL: u32 = 0x0147;
const CB_SETCURSEL: u32 = 0x014e;
const EM_LIMITTEXT: u32 = 0x00c5;
const CHILD: u32 = 0x5000_0000; // WS_CHILD | WS_VISIBLE
const TABSTOP: u32 = 0x0001_0000;
const WS_VSCROLL: u32 = 0x0020_0000;
const ES_AUTOHSCROLL: u32 = 0x0080;
const BS_DEFPUSHBUTTON: u32 = 0x0001;
const BS_AUTOCHECKBOX: u32 = 0x0003;
const CBS_DROPDOWNLIST: u32 = 0x0003;
const WS_EX_CLIENTEDGE: u32 = 0x0200;

/// What the player picked: the button, the choices, and the game's environment from them.
pub struct Picked {
    pub mode: Mode,
    pub choice: Choice,
    pub env: Vec<(&'static str, Option<String>)>,
}

struct State {
    dll: Option<PathBuf>,
    picked: Option<Picked>,
    // voiceui
    /// The players the file named, the first [`SHOWN_PLAYERS`] of them listed.
    players: Vec<PlayerVoice>,
}

thread_local! {
    static STATE: RefCell<Option<State>> = const { RefCell::new(None) };
}

/// Whether this copy of the launcher is the co-op one by its name (`Play Co-op.exe`).
pub fn named_for_coop() -> bool {
    std::env::current_exe()
        .ok()
        .and_then(|e| {
            e.file_stem()
                .map(|s| s.to_string_lossy().to_ascii_lowercase())
        })
        .is_some_and(|s| s.contains("co-op") || s.contains("coop"))
}

/// The saved choices of the package at `userdata`.
pub fn saved(userdata: &Path) -> Choice {
    std::fs::read(userdata.join(coop::FILE))
        .map(|b| Choice::parse(&String::from_utf8_lossy(&b)))
        .unwrap_or_default()
}

/// Shows the window; `None` when the player closed it (or no windows may be shown).
pub fn choose(root: &Path, userdata: &Path) -> Option<Picked> {
    if !crate::ui::windows_allowed() {
        return None;
    }
    let dll = coop::steam_dll(root);
    STATE.with(|s| {
        *s.borrow_mut() = Some(State {
            dll: dll.clone(),
            picked: None,
            players: saved(userdata).players, // voiceui
        })
    });
    // SAFETY: the window and its controls live on this thread until the loop below ends; every
    // string passed lives through its call.
    unsafe { show(&saved(userdata), dll.is_some()) };
    STATE.with(|s| s.borrow_mut().take().and_then(|st| st.picked))
}

unsafe fn text_of(window: *mut c_void, id: i32) -> String {
    let mut buffer = vec![0u16; 512];
    GetWindowTextW(
        GetDlgItem(window, id),
        buffer.as_mut_ptr(),
        buffer.len() as i32,
    );
    from_wide(&buffer).to_string_lossy().into_owned()
}

unsafe fn checked(window: *mut c_void, id: i32) -> bool {
    SendMessageW(GetDlgItem(window, id), BM_GETCHECK, 0, 0) == 1
}

unsafe fn selected(window: *mut c_void, id: i32) -> usize {
    SendMessageW(GetDlgItem(window, id), CB_GETCURSEL, 0, 0).max(0) as usize
}

/// The choices as the window holds them now.
unsafe fn read(window: *mut c_void) -> Choice {
    Choice {
        name: text_of(window, NAME),
        address: text_of(window, ADDRESS),
        code: text_of(window, CODE),
        steam: checked(window, STEAM),
        voice: Voice::ALL[selected(window, VOICE).min(Voice::ALL.len() - 1)],
        ptt_key: PTT_KEYS[selected(window, KEY).min(PTT_KEYS.len() - 1)].into(),
        items: checked(window, ITEMS),
        vats: checked(window, VATS),
        harm: checked(window, HARM),
        // voiceui
        players_key: LIST_KEYS[selected(window, PLAYERS_KEY).min(LIST_KEYS.len() - 1)].into(),
        mute_key: LIST_KEYS[selected(window, MUTE_KEY).min(LIST_KEYS.len() - 1)].into(),
        deafen_key: LIST_KEYS[selected(window, DEAFEN_KEY).min(LIST_KEYS.len() - 1)].into(),
        players: STATE.with(|s| {
            let known = s
                .borrow()
                .as_ref()
                .map(|st| st.players.clone())
                .unwrap_or_default();
            coop::players_from_window(&known, SHOWN_PLAYERS, |i| {
                (
                    checked(window, PLAYER_MUTE + i as i32),
                    selected(window, PLAYER_VOLUME + i as i32),
                )
            })
        }),
    }
}

unsafe extern "system" fn procedure(
    window: *mut c_void,
    message: u32,
    w: usize,
    l: isize,
) -> isize {
    match message {
        WM_COMMAND if (w >> 16) & 0xffff == 0 => {
            let mode = match (w & 0xffff) as i32 {
                ALONE => Mode::Alone,
                HOST => Mode::Host,
                JOIN => Mode::Join,
                INVITE => Mode::InviteSteamFriend,
                STEAM_JOIN => Mode::JoinSteamFriend,
                _ => return 0,
            };
            let choice = read(window);
            let dll = STATE.with(|s| s.borrow().as_ref().and_then(|st| st.dll.clone()));
            match choice.environment(mode, dll.as_deref()) {
                Ok(env) => {
                    STATE.with(|s| {
                        if let Some(st) = s.borrow_mut().as_mut() {
                            st.picked = Some(Picked { mode, choice, env });
                        }
                    });
                    DestroyWindow(window);
                }
                Err(why) => {
                    let (text, title) = (wide_str(&why), wide_str(TITLE));
                    // MB_OK | MB_ICONWARNING.
                    MessageBoxW(window, text.as_ptr(), title.as_ptr(), 0x30);
                }
            }
            0
        }
        WM_CLOSE => {
            DestroyWindow(window);
            0
        }
        WM_DESTROY => {
            PostQuitMessage(0);
            0
        }
        _ => DefWindowProcW(window, message, w, l),
    }
}

unsafe fn show(saved: &Choice, steam: bool) {
    let class_name = wide_str("fnv-coop-launcher");
    let instance = GetModuleHandleW(std::ptr::null());
    let class = WndClassW {
        style: 0,
        procedure,
        class_extra: 0,
        window_extra: 0,
        instance,
        icon: std::ptr::null_mut(),
        cursor: std::ptr::null_mut(),
        // COLOR_BTNFACE.
        background: GetSysColorBrush(15),
        menu_name: std::ptr::null(),
        class_name: class_name.as_ptr(),
    };
    RegisterClassW(&class);
    // voiceui: taller by the known players' rows.
    let rows = saved.players.len().min(SHOWN_PLAYERS) as i32;
    let (width, height) = (560, 600 + if rows > 0 { 26 + 24 * rows } else { 26 });
    let (x, y) = (
        (GetSystemMetrics(0) - width) / 2,
        (GetSystemMetrics(1) - height) / 2,
    );
    let title = wide_str(TITLE);
    // WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_MINIMIZEBOX | WS_VISIBLE.
    let window = CreateWindowExW(
        0,
        class_name.as_ptr(),
        title.as_ptr(),
        0x10ca_0000,
        x,
        y,
        width,
        height,
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        instance,
        std::ptr::null_mut(),
    );
    if window.is_null() {
        return;
    }
    // DEFAULT_GUI_FONT.
    let font = GetStockObject(17) as usize;
    let add = |class: &str, text: &str, style: u32, ex: u32, id: i32, r: [i32; 4]| {
        let (c, t) = (wide_str(class), wide_str(text));
        let h = CreateWindowExW(
            ex,
            c.as_ptr(),
            t.as_ptr(),
            CHILD | style,
            r[0],
            r[1],
            r[2],
            r[3],
            window,
            id as usize as *mut c_void,
            instance,
            std::ptr::null_mut(),
        );
        SendMessageW(h, WM_SETFONT, font, 1);
        h
    };
    let label = |text: &str, r: [i32; 4]| add("STATIC", text, 0, 0, 0, r);
    let edit = |text: &str, id: i32, r: [i32; 4]| {
        let h = add(
            "EDIT",
            text,
            TABSTOP | ES_AUTOHSCROLL,
            WS_EX_CLIENTEDGE,
            id,
            r,
        );
        SendMessageW(h, EM_LIMITTEXT, 200, 0);
        h
    };
    let check = |text: &str, id: i32, on: bool, r: [i32; 4]| {
        let h = add("BUTTON", text, TABSTOP | BS_AUTOCHECKBOX, 0, id, r);
        SendMessageW(h, BM_SETCHECK, usize::from(on), 0);
        h
    };
    let list = |id: i32, items: &[&str], at: usize, r: [i32; 4]| {
        let h = add(
            "COMBOBOX",
            "",
            TABSTOP | CBS_DROPDOWNLIST | WS_VSCROLL,
            0,
            id,
            r,
        );
        for item in items {
            let t = wide_str(item);
            SendMessageW(h, CB_ADDSTRING, 0, t.as_ptr() as isize);
        }
        SendMessageW(h, CB_SETCURSEL, at, 0);
        h
    };
    let button = |text: &str, id: i32, default: bool, r: [i32; 4]| {
        let style = TABSTOP | if default { BS_DEFPUSHBUTTON } else { 0 };
        add("BUTTON", text, style, 0, id, r)
    };

    let full = 520;
    label(
        "Play the story of Fallout: New Vegas together. One player hosts, the others join.",
        [16, 12, full, 18],
    );
    label("Your name:", [16, 44, 150, 18]);
    edit(&saved.name, NAME, [170, 41, 220, 22]);
    label("Join code (to join):", [16, 76, 150, 18]);
    edit(&saved.code, CODE, [170, 73, 366, 22]);
    label(
        "Address for your join code (to host). Empty: friends in your home network. Over the internet: your public address (forward UDP port 27961) or your address in a private network app.",
        [16, 104, full, 46],
    );
    edit(&saved.address, ADDRESS, [16, 152, 260, 22]);
    let steam_box = check(
        "Host on Steam too: Steam friends join with no port to forward",
        STEAM,
        saved.steam && steam,
        [16, 184, full, 20],
    );
    let steam_text = if steam {
        "Steam: steam_api64.dll is next to Play.exe. Keep Steam running and signed in."
    } else {
        "Steam: steam_api64.dll is not next to Play.exe, so the Steam choices are off. The join code works without it."
    };
    label(steam_text, [16, 206, full, 32]);
    label("Voice chat:", [16, 246, 150, 18]);
    let voices: Vec<&str> = Voice::ALL.iter().map(|v| v.label()).collect();
    let at = Voice::ALL
        .iter()
        .position(|v| *v == saved.voice)
        .unwrap_or(0);
    list(VOICE, &voices, at, [170, 243, 200, 200]);
    label("Push-to-talk key:", [16, 278, 150, 18]);
    let key_at = PTT_KEYS
        .iter()
        .position(|k| k.eq_ignore_ascii_case(&saved.ptt_key))
        .unwrap_or(0);
    list(KEY, &PTT_KEYS, key_at, [170, 275, 200, 240]);
    // voiceui: the players list's key, the mute and deafen keys, and the known players.
    let key_index = |k: &str, default: &str| {
        let at = |n: &str| LIST_KEYS.iter().position(|x| x.eq_ignore_ascii_case(n));
        at(k).or_else(|| at(default)).unwrap_or(0)
    };
    label("Players list key (in the game):", [16, 306, 150, 18]);
    list(
        PLAYERS_KEY,
        &LIST_KEYS,
        key_index(&saved.players_key, "F11"),
        [170, 303, 90, 240],
    );
    label("Microphone off and on:", [16, 334, 150, 18]);
    list(
        MUTE_KEY,
        &LIST_KEYS,
        key_index(&saved.mute_key, "F8"),
        [170, 331, 90, 240],
    );
    label("Other voices off and on:", [276, 334, 140, 18]);
    list(
        DEAFEN_KEY,
        &LIST_KEYS,
        key_index(&saved.deafen_key, "F7"),
        [420, 331, 90, 240],
    );
    let mut y = 362;
    let shown: Vec<&PlayerVoice> = saved.players.iter().take(SHOWN_PLAYERS).collect();
    if shown.is_empty() {
        label(
            "Players you mute or turn up or down in the game's players list are kept here by name.",
            [16, y, full, 18],
        );
        y += 26;
    } else {
        label(
            "Players you have set (the game's players list changes them too):",
            [16, y, full, 18],
        );
        y += 22;
        let volumes = coop::volume_labels();
        let volume_refs: Vec<&str> = volumes.iter().map(String::as_str).collect();
        for (i, p) in shown.iter().enumerate() {
            label(&p.name, [28, y + 2, 180, 18]);
            check("Muted", PLAYER_MUTE + i as i32, p.muted, [216, y, 80, 20]);
            list(
                PLAYER_VOLUME + i as i32,
                &volume_refs,
                coop::volume_index(p.volume),
                [300, y - 2, 90, 300],
            );
            y += 24;
        }
        y += 4;
    }
    label(
        "Parts on for everyone, untick to turn off (all players tick the same):",
        [16, y, full, 18],
    );
    check(
        "Loot, pick-ups and barter through the host",
        ITEMS,
        saved.items,
        [28, y + 20, 500, 20],
    );
    check(
        "V.A.T.S. with several players (it does not stop the others' world)",
        VATS,
        saved.vats,
        [28, y + 42, 500, 20],
    );
    check(
        "Falls, traps and effects decided by the host, as hits are",
        HARM,
        saved.harm,
        [28, y + 64, 500, 20],
    );
    let y = y + 98;
    button("Play alone", ALONE, false, [16, y, 120, 30]);
    button("Host", HOST, true, [144, y, 120, 30]);
    button("Join", JOIN, false, [272, y, 120, 30]);
    let invite = button(
        "Invite a Steam friend",
        INVITE,
        false,
        [16, y + 36, 182, 30],
    );
    let steam_join = button(
        "Join a Steam friend",
        STEAM_JOIN,
        false,
        [206, y + 36, 182, 30],
    );
    if !steam {
        for h in [steam_box, invite, steam_join] {
            EnableWindow(h, 0);
        }
    }
    SetForegroundWindow(window);
    let mut message = [0usize; 8];
    while GetMessageW(&mut message, std::ptr::null_mut(), 0, 0) > 0 {
        if IsDialogMessageW(window, &mut message) == 0 {
            TranslateMessage(&message);
            DispatchMessageW(&message);
        }
    }
}

/// Puts `text` on the clipboard (Windows' `clip.exe`).
fn to_clipboard(text: &str) {
    use std::io::Write;
    use std::os::windows::process::CommandExt;
    if let Ok(mut child) = std::process::Command::new("clip.exe")
        .stdin(std::process::Stdio::piped())
        .creation_flags(0x0800_0000)
        .spawn()
    {
        if let Some(mut input) = child.stdin.take() {
            let _ = input.write_all(text.as_bytes());
        }
        let _ = child.wait();
    }
}

/// Watches a host's game log for its join codes, then copies the code and shows it once.
pub struct CodeWatch {
    log: PathBuf,
    started: std::time::Instant,
    last: Option<std::time::Instant>,
    done: bool,
}

impl CodeWatch {
    pub fn new(log: &Path) -> CodeWatch {
        CodeWatch {
            log: log.to_path_buf(),
            started: std::time::Instant::now(),
            last: None,
            done: false,
        }
    }

    /// Looks again (about once a second, for 15 minutes); the log line to write when found.
    pub fn poll(&mut self) -> Option<String> {
        if self.done
            || self.started.elapsed() > std::time::Duration::from_secs(900)
            || self
                .last
                .is_some_and(|t| t.elapsed() < std::time::Duration::from_secs(1))
        {
            return None;
        }
        self.last = Some(std::time::Instant::now());
        let bytes = std::fs::read(&self.log).ok()?;
        let head = &bytes[..bytes.len().min(512 * 1024)];
        let codes = coop::codes_in(&String::from_utf8_lossy(head));
        let text = coop::codes_message(&codes)?;
        self.done = true;
        if let Some(code) = &codes.direct {
            to_clipboard(code);
        }
        if crate::ui::windows_allowed() {
            // Its own thread: the launcher goes on watching the game meanwhile.
            std::thread::spawn(move || crate::ui::info(&text));
        }
        Some(format!(
            "co-op: the game made its join code ({}); it is on the clipboard",
            codes.direct.as_deref().unwrap_or("")
        ))
    }
}

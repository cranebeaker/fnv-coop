//! The platform's window side (USER32, SHELL32 and the KERNEL32 pieces `main` uses around its
//! window): the single-instance mutex, the command line, window classes, windows, their styles
//! and borders, and the message queue whose emptiness ends a frame of the game's main loop.
//!
//! The windows are records only: the window on screen is the viewer's. What Windows would answer
//! that depends on the machine (the screen metrics, the style a new window reads back, the border
//! `AdjustWindowRectEx` adds) comes from the player's adapter table (`super::d3d9`, made by
//! `scripts/d3d9-capture` on the player's machine); a style the table doesn't have stops the run.
//!
//! Not provided (a call stops the run, or, where noted, the effect is missing): the messages
//! Windows sends a window's procedure while it is created or moved (`WM_CREATE`,
//! `WM_WINDOWPOSCHANGED`, `WM_ACTIVATE`, ...): the game's procedure (`0086a0a0`) only acts on them
//! once `Main` exists, and `main` makes its window before `Main`, but a window made or moved
//! later gets none of them (recorded in the module's functions); message boxes (the game shows
//! one only to stop: the run stops with its text); `ExitProcess`; `ShellExecuteA`.

use super::KernelObject;
use crate::{Engine, Ptr};
use std::collections::{BTreeMap, VecDeque};

/// `ERROR_ALREADY_EXISTS`, `ERROR_INVALID_WINDOW_HANDLE`, `ERROR_CLASS_ALREADY_EXISTS`.
pub const ERROR_ALREADY_EXISTS: u32 = 183;
pub const ERROR_INVALID_WINDOW_HANDLE: u32 = 1400;
pub const ERROR_CLASS_ALREADY_EXISTS: u32 = 1410;
/// `GWL_STYLE`, `GWL_EXSTYLE`, `GCL_MENUNAME`.
const GWL_STYLE: i32 = -16;
const GWL_EXSTYLE: i32 = -20;
const GCL_MENUNAME: i32 = -8;
/// `WS_CHILD`, `WS_VISIBLE`.
const WS_CHILD: u32 = 0x4000_0000;
const WS_VISIBLE: u32 = 0x1000_0000;
/// `PM_REMOVE`.
const PM_REMOVE: u32 = 1;
/// Class atoms start here (`RegisterClassA`'s answers).
const FIRST_ATOM: u32 = 0xc000;

/// A registered window class.
#[derive(Debug, Clone)]
pub struct Class {
    pub atom: u32,
    pub name: Vec<u8>,
    pub style: u32,
    pub procedure: u32,
    pub instance: u32,
    pub menu_name: u32,
}

/// A window: its class, styles, parent and rectangle (screen or parent coordinates).
#[derive(Debug, Clone)]
pub struct Window {
    pub class: u32,
    pub title: Vec<u8>,
    pub style: u32,
    pub ex_style: u32,
    pub parent: u32,
    pub rect: [i32; 4],
}

/// The window side's state (`Kernel::windows`).
#[derive(Debug, Default)]
pub struct Windows {
    pub classes: Vec<Class>,
    pub windows: BTreeMap<u32, Window>,
    /// Messages posted and not taken yet (`MSG`'s seven words).
    pub queue: VecDeque<[u32; 7]>,
    /// The command line `GetCommandLineW` answers (UTF-16, made once), and where it is.
    pub command_line: Vec<u16>,
    command_line_at: u32,
    /// Frames the main loop ended on an empty queue ([`peek_message_a`]).
    pub frames: u64,
}

fn last_error(e: &mut Engine, code: u32) {
    e.kernel.last_error = code;
}

// Platform: replaces KERNEL32.DLL CreateMutexA (import)
/// `CreateMutexA(security, initial owner, name)`: a new mutex (owned by the calling thread when
/// asked), last error 0; a name already in use answers a new handle to the same mutex with
/// `ERROR_ALREADY_EXISTS` (the game's single-instance check).
pub fn create_mutex_a(e: &mut Engine, _security: u32, initial_owner: u32, name: Ptr) -> u32 {
    let name = if name.is_null() {
        Vec::new()
    } else {
        e.mem.cstr(name.addr())
    };
    let exists = !name.is_empty()
        && e.kernel
            .objects
            .values()
            .any(|o| matches!(o, KernelObject::Mutex { name: n, .. } if *n == name));
    let owner = if initial_owner != 0 && !exists {
        crate::thread_host::current_id(e)
    } else {
        0
    };
    let handle = e.kernel.insert(KernelObject::Mutex {
        name,
        owner,
        count: u32::from(owner != 0),
    });
    last_error(e, if exists { ERROR_ALREADY_EXISTS } else { 0 });
    handle
}

// Platform: replaces USER32.DLL MessageBoxA (import)
/// `MessageBoxA(owner, text, caption, type)`: the game shows one when it gives up (another
/// instance running, a remote session, a failed renderer): the run stops with its text.
pub fn message_box_a(e: &mut Engine, _owner: u32, text: Ptr, caption: Ptr, _kind: u32) -> u32 {
    let text = String::from_utf8_lossy(&e.mem.cstr(text.addr())).into_owned();
    let caption = String::from_utf8_lossy(&e.mem.cstr(caption.addr())).into_owned();
    panic!("MessageBoxA({caption:?}, {text:?}): the game gave up (message boxes not provided)")
}

// Platform: replaces KERNEL32.DLL ExitProcess (import)
/// `ExitProcess(code)`: the game ending its process: the run stops.
pub fn exit_process(_e: &mut Engine, code: u32) {
    panic!("ExitProcess({code}): the game ended its process (not provided)")
}

// Platform: replaces SHELL32.DLL ShellExecuteA (import)
/// `ShellExecuteA`: starting another program (the launcher) is not provided.
pub fn shell_execute_a(
    e: &mut Engine,
    _w: u32,
    _op: u32,
    file: Ptr,
    _p: u32,
    _d: u32,
    _s: u32,
) -> u32 {
    let file = String::from_utf8_lossy(&e.mem.cstr(file.addr())).into_owned();
    panic!("ShellExecuteA({file:?}) is not provided")
}

// Platform: replaces ADVAPI32.DLL GetUserNameA (import)
/// `GetUserNameA(buffer, size)`: the player's user name (the `USERNAME` the process runs with),
/// zero-terminated, with `*size` set to its length plus one; false with
/// `ERROR_INSUFFICIENT_BUFFER` (122) and the size needed when it doesn't fit.
pub fn get_user_name_a(e: &mut Engine, buffer: Ptr, size: Ptr) -> u32 {
    let name = std::env::var("USERNAME").unwrap_or_default().into_bytes();
    let needed = name.len() as u32 + 1;
    let room = e.mem.u32(size.addr());
    e.mem.set_u32(size.addr(), needed);
    if room < needed {
        last_error(e, 122);
        return 0;
    }
    e.mem.set_cstr(buffer.addr(), &name);
    1
}

// Platform: replaces STEAM_API.DLL SteamAPI_Init (import)
/// `SteamAPI_Init()`: false, the answer when no Steam client runs: the Steam client is not
/// provided, and the game's own code goes on without it (`006ff690` keeps the false at +0x3c of
/// its Steam object and skips `SteamUtils`/`SteamUserStats`).
pub fn steam_api_init(_e: &mut Engine) -> u32 {
    0
}

// Platform: replaces STEAM_API.DLL SteamAPI_RegisterCallback (import)
/// `SteamAPI_RegisterCallback(callback, id)` (`cdecl`, two words, no answer the game reads):
/// the game registers its Steam callbacks before `SteamAPI_Init` (`006ff690` through
/// `006ffb20`/`006ffb90`/`006ffc20`, id 0x44d and others). With no Steam client nothing ever
/// calls them back, so nothing is kept.
pub fn steam_api_register_callback(_e: &mut Engine, _callback: Ptr, _id: u32) {}

// Platform: replaces STEAM_API.DLL SteamAPI_UnregisterCallback (import)
/// `SteamAPI_UnregisterCallback(callback)` (`cdecl`, one word; `006ffc00`): nothing was kept.
pub fn steam_api_unregister_callback(_e: &mut Engine, _callback: Ptr) {}

// Platform: replaces STEAM_API.DLL SteamAPI_IsSteamRunning (import)
/// `SteamAPI_IsSteamRunning()` (`cdecl`, no arguments, a `bool` in `AL`): false, the answer of
/// the state this layer keeps, no Steam client (as [`steam_api_init`] answers). Not measured in
/// that state: a Steam client ran on the measuring machine, and the player's `steam_api.dll`
/// answered 1 there (2026-10-27, `build43-scratch/measure/platform5`).
pub fn steam_api_is_steam_running(_e: &mut Engine) -> u32 {
    0
}

// Platform: replaces STEAM_API.DLL SteamUserStats (import)
/// `SteamUserStats()` (`cdecl`, no arguments): null. Measured (2026-10-27,
/// `build43-scratch/measure/platform5`): the player's `steam_api.dll` in a 32-bit process
/// answers null for `SteamUserStats`, `SteamFriends`, `SteamUtils`, `SteamUser` and `SteamApps`
/// while `SteamAPI_Init` has not succeeded, the state this layer keeps.
pub fn steam_user_stats(_e: &mut Engine) -> u32 {
    0
}

// Platform: replaces STEAM_API.DLL SteamFriends (import)
/// `SteamFriends()`: null, as measured ([`steam_user_stats`]).
pub fn steam_friends(_e: &mut Engine) -> u32 {
    0
}

// Platform: replaces STEAM_API.DLL SteamUtils (import)
/// `SteamUtils()`: null, as measured ([`steam_user_stats`]).
pub fn steam_utils(_e: &mut Engine) -> u32 {
    0
}

// Platform: replaces STEAM_API.DLL SteamUser (import)
/// `SteamUser()`: null, as measured ([`steam_user_stats`]).
pub fn steam_user(_e: &mut Engine) -> u32 {
    0
}

// Platform: replaces STEAM_API.DLL SteamAPI_RunCallbacks (import)
/// `SteamAPI_RunCallbacks()` (`cdecl`, no arguments, no answer the game reads; called every frame,
/// `006ff860`): does nothing in the state this layer keeps (no successful `SteamAPI_Init`).
/// Measured (2026-10-27, `build43-scratch/measure/platform5`): the player's `steam_api.dll` in a
/// 32-bit process without `SteamAPI_Init` returned from 1000 calls with `EAX` 0, the last error
/// left as set, and `SteamUserStats` still null afterwards.
pub fn steam_api_run_callbacks(_e: &mut Engine) {}

// Platform: replaces STEAM_API.DLL SteamAPI_Shutdown (import)
/// `SteamAPI_Shutdown()` (`cdecl`, no arguments, no answer the game reads; called once at exit,
/// `006ff7b0`): does nothing in the state this layer keeps (no successful `SteamAPI_Init`).
/// Measured (2026-10-30, `build43-scratch/measure/platform5`, `shutdown`): the player's
/// `steam_api.dll` in a 32-bit process without `SteamAPI_Init` returned from two calls with
/// `EAX` 0 and the last error left as set; `SteamAPI_RunCallbacks` and `SteamUserStats` answered
/// as before afterwards.
pub fn steam_api_shutdown(_e: &mut Engine) {}

// Platform: replaces STEAM_API.DLL SteamApps (import)
/// `SteamApps()`: null, as measured ([`steam_user_stats`]).
pub fn steam_apps(_e: &mut Engine) -> u32 {
    0
}

// Platform: replaces KERNEL32.DLL IsDebuggerPresent (import)
/// `IsDebuggerPresent()`: no debugger.
pub fn is_debugger_present(_e: &mut Engine) -> u32 {
    0
}

fn class_name_matches(e: &Engine, class: &Class, name: u32) -> bool {
    if name < 0x1_0000 {
        return class.atom == name;
    }
    class.name.eq_ignore_ascii_case(&e.mem.cstr(name))
}

// Platform: replaces USER32.DLL FindWindowA (import)
/// `FindWindowA(class, title)`: the first top-level window of this process with that class (and
/// title, when given); 0 when there is none. Only this process's windows exist here.
pub fn find_window_a(e: &mut Engine, class: u32, title: Ptr) -> u32 {
    let title = (!title.is_null()).then(|| e.mem.cstr(title.addr()));
    let found = e.kernel.windows.windows.iter().find(|(_, w)| {
        let class_ok = class == 0
            || e.kernel
                .windows
                .classes
                .iter()
                .any(|c| c.atom == w.class && class_name_matches(e, c, class));
        w.parent == 0 && class_ok && title.as_ref().map_or(true, |t| *t == w.title)
    });
    found.map_or(0, |(&h, _)| h)
}

// Platform: replaces USER32.DLL SetForegroundWindow (import)
/// `SetForegroundWindow(window)`: true for a window of this layer.
pub fn set_foreground_window(e: &mut Engine, window: u32) -> u32 {
    u32::from(e.kernel.windows.windows.contains_key(&window))
}

// Platform: replaces KERNEL32.DLL GetCommandLineW (import)
/// `GetCommandLineW()`: the process's command line, made once: the executable's path, quoted,
/// as Steam starts the game (`Kernel::current_directory` + `FalloutNV.exe`), no arguments.
pub fn get_command_line_w(e: &mut Engine) -> u32 {
    if e.kernel.windows.command_line_at == 0 {
        if e.kernel.windows.command_line.is_empty() {
            let mut text = String::from("\"");
            text.push_str(&String::from_utf8_lossy(&e.kernel.current_directory));
            if !text.ends_with('\\') && text.len() > 1 {
                text.push('\\');
            }
            text.push_str("FalloutNV.exe\"");
            e.kernel.windows.command_line = text.encode_utf16().collect();
        }
        let units = e.kernel.windows.command_line.clone();
        let at = e.mem.alloc(2 * (units.len() as u32 + 1));
        for (i, u) in units.iter().enumerate() {
            e.mem.set_u16(at + 2 * i as u32, *u);
        }
        e.mem.set_u16(at + 2 * units.len() as u32, 0);
        e.kernel.windows.command_line_at = at;
    }
    e.kernel.windows.command_line_at
}

/// A NUL-terminated UTF-16 string's code units.
fn wide(e: &Engine, mut at: u32) -> Vec<u16> {
    let mut out = Vec::new();
    loop {
        let u = e.mem.u16(at);
        if u == 0 {
            return out;
        }
        out.push(u);
        at += 2;
    }
}

/// Splits a command line as `CommandLineToArgvW` does: the program name runs to the closing
/// quote (when it starts with one) or the first space or tab, with no escapes; after it,
/// arguments are separated by spaces and tabs, quotes group, `2n` backslashes before a quote
/// become `n` and the quote groups, `2n+1` become `n` and a literal quote, a doubled quote inside
/// a quoted part is a literal quote, other backslashes are literal.
pub fn split_command_line(line: &[u16]) -> Vec<Vec<u16>> {
    const QUOTE: u16 = b'"' as u16;
    const BACKSLASH: u16 = b'\\' as u16;
    let blank = |u: u16| u == b' ' as u16 || u == b'\t' as u16;
    let mut args = Vec::new();
    let mut i = 0;
    let mut name = Vec::new();
    if line.first() == Some(&QUOTE) {
        i = 1;
        while i < line.len() && line[i] != QUOTE {
            name.push(line[i]);
            i += 1;
        }
        i += 1;
    } else {
        while i < line.len() && !blank(line[i]) {
            name.push(line[i]);
            i += 1;
        }
    }
    args.push(name);
    loop {
        while i < line.len() && blank(line[i]) {
            i += 1;
        }
        if i >= line.len() {
            return args;
        }
        let mut arg = Vec::new();
        let mut quoted = false;
        while i < line.len() && (quoted || !blank(line[i])) {
            if line[i] == BACKSLASH {
                let start = i;
                while i < line.len() && line[i] == BACKSLASH {
                    i += 1;
                }
                let n = i - start;
                if i < line.len() && line[i] == QUOTE {
                    arg.extend(std::iter::repeat(BACKSLASH).take(n / 2));
                    if n % 2 == 1 {
                        arg.push(QUOTE);
                        i += 1;
                    }
                } else {
                    arg.extend(std::iter::repeat(BACKSLASH).take(n));
                }
            } else if line[i] == QUOTE {
                if quoted && line.get(i + 1) == Some(&QUOTE) {
                    arg.push(QUOTE);
                    i += 2;
                } else {
                    quoted = !quoted;
                    i += 1;
                }
            } else {
                arg.push(line[i]);
                i += 1;
            }
        }
        args.push(arg);
    }
}

// Platform: replaces SHELL32.DLL CommandLineToArgvW (import)
/// `CommandLineToArgvW(line, count out)`: the arguments ([`split_command_line`]) in one block
/// (the pointer array, then the strings) to be freed with `LocalFree`; the count through
/// `count`. An empty line is not provided (Windows answers the program's path then).
pub fn command_line_to_argv_w(e: &mut Engine, line: Ptr, count: Ptr) -> u32 {
    let units = wide(e, line.addr());
    assert!(
        !units.is_empty(),
        "CommandLineToArgvW with an empty line is not provided"
    );
    let args = split_command_line(&units);
    let pointers = 4 * args.len() as u32;
    let strings: u32 = args.iter().map(|a| 2 * (a.len() as u32 + 1)).sum();
    let block = e.mem.alloc(pointers + strings);
    let mut at = block + pointers;
    for (i, arg) in args.iter().enumerate() {
        e.mem.set_u32(block + 4 * i as u32, at);
        for u in arg {
            e.mem.set_u16(at, *u);
            at += 2;
        }
        e.mem.set_u16(at, 0);
        at += 2;
    }
    if !count.is_null() {
        e.mem.set_u32(count.addr(), args.len() as u32);
    }
    block
}

// Platform: replaces KERNEL32.DLL LocalFree (import)
/// `LocalFree(block)`: frees a block this layer handed out; answers 0 (success).
pub fn local_free(e: &mut Engine, block: u32) -> u32 {
    if block != 0 {
        e.mem.free(block);
    }
    0
}

// Platform: replaces USER32.DLL LoadIconA (import)
/// `LoadIconA(instance, name)`: a handle for the icon resource (the icon itself isn't loaded:
/// the viewer's window has its own).
pub fn load_icon_a(e: &mut Engine, _instance: u32, _name: u32) -> u32 {
    e.kernel.insert(KernelObject::UserObject("icon"))
}

// Platform: replaces GDI32.DLL GetStockObject (import)
/// `GetStockObject(index)`: the stock object's handle (the same one for the same index).
pub fn get_stock_object(e: &mut Engine, index: u32) -> u32 {
    let name: &'static str = match index {
        0 => "white brush",
        1 => "light gray brush",
        2 => "gray brush",
        3 => "dark gray brush",
        4 => "black brush",
        5 => "null brush",
        _ => panic!("GetStockObject({index}) is not provided"),
    };
    if let Some((&h, _)) = e
        .kernel
        .objects
        .iter()
        .find(|(_, o)| **o == KernelObject::UserObject(name))
    {
        return h;
    }
    e.kernel.insert(KernelObject::UserObject(name))
}

// Platform: replaces USER32.DLL RegisterClassA (import)
/// `RegisterClassA(class)`: records the `WNDCLASSA` (style, procedure, instance, menu name,
/// name) and answers its atom; 0 with `ERROR_CLASS_ALREADY_EXISTS` for a name in use.
pub fn register_class_a(e: &mut Engine, class: Ptr) -> u32 {
    let c = class.addr();
    let name_word = e.mem.u32(c + 36);
    assert!(
        name_word >= 0x1_0000,
        "RegisterClassA by atom is not provided"
    );
    let name = e.mem.cstr(name_word);
    if e.kernel
        .windows
        .classes
        .iter()
        .any(|k| k.name.eq_ignore_ascii_case(&name))
    {
        last_error(e, ERROR_CLASS_ALREADY_EXISTS);
        return 0;
    }
    let atom = FIRST_ATOM + e.kernel.windows.classes.len() as u32;
    let record = Class {
        atom,
        name,
        style: e.mem.u32(c),
        procedure: e.mem.u32(c + 4),
        instance: e.mem.u32(c + 16),
        menu_name: e.mem.u32(c + 32),
    };
    e.kernel.windows.classes.push(record);
    atom
}

/// The border `AdjustWindowRectEx` adds for this style, menu and extended style, from the table.
fn border(e: &mut Engine, style: u32, menu: u32, ex_style: u32) -> [i32; 4] {
    match super::d3d9::window_border(e, style, u32::from(menu != 0), ex_style) {
        Some(b) => b,
        None => panic!(
            "AdjustWindowRectEx(style {style:#x}, menu {menu}, ex style {ex_style:#x}) was not captured in the adapter table: not provided"
        ),
    }
}

// Platform: replaces USER32.DLL AdjustWindowRectEx (import)
/// `AdjustWindowRectEx(rect, style, menu, ex style)`: the rectangle grown by the window border
/// Windows adds for that style (captured, [`border`]); true.
pub fn adjust_window_rect_ex(
    e: &mut Engine,
    rect: Ptr,
    style: u32,
    menu: u32,
    ex_style: u32,
) -> u32 {
    let b = border(e, style, menu, ex_style);
    for (i, d) in b.iter().enumerate() {
        let at = rect.addr() + 4 * i as u32;
        let v = e.mem.i32(at).wrapping_add(*d);
        e.mem.set_i32(at, v);
    }
    1
}

// Platform: replaces USER32.DLL AdjustWindowRect (import)
/// `AdjustWindowRect(rect, style, menu)`: `AdjustWindowRectEx` with no extended style.
pub fn adjust_window_rect(e: &mut Engine, rect: Ptr, style: u32, menu: u32) -> u32 {
    adjust_window_rect_ex(e, rect, style, menu, 0)
}

// Platform: replaces USER32.DLL CreateWindowExA (import)
/// `CreateWindowExA(ex style, class, title, style, x, y, width, height, parent, menu, instance,
/// parameter)`: a window record of a registered class (0 with `ERROR_CANNOT_FIND_WND_CLASS`
/// otherwise), with the style and extended style Windows gives it (captured: a top-level
/// `WS_VISIBLE` window also gets `WS_CLIPSIBLINGS | WS_CAPTION` and `WS_EX_WINDOWEDGE`); the
/// first visible top-level window becomes the active one. The creation messages are not sent
/// (module documentation).
#[allow(clippy::too_many_arguments)]
pub fn create_window_ex_a(
    e: &mut Engine,
    ex_style: u32,
    class: u32,
    title: Ptr,
    style: u32,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    parent: u32,
    _menu: u32,
    _instance: u32,
    _parameter: u32,
) -> u32 {
    let Some(atom) = e
        .kernel
        .windows
        .classes
        .iter()
        .find(|c| class_name_matches(e, c, class))
        .map(|c| c.atom)
    else {
        last_error(e, 1407);
        return 0;
    };
    let child = style & WS_CHILD != 0;
    let (style, ex_style) = match super::d3d9::window_style(e, style, u32::from(child)) {
        Some((s, x)) => (s, x | ex_style),
        None => panic!(
            "CreateWindowExA with style {style:#x} ({}) was not captured in the adapter table: not provided",
            if child { "child" } else { "top-level" }
        ),
    };
    let title = if title.is_null() {
        Vec::new()
    } else {
        e.mem.cstr(title.addr())
    };
    let handle = e.kernel.insert(KernelObject::Window);
    let rect = [x, y, x.wrapping_add(width), y.wrapping_add(height)];
    #[cfg(windows)]
    super::d3d9real::window_created(
        e,
        handle,
        &title,
        style,
        ex_style,
        rect,
        if child { parent } else { 0 },
    );
    e.kernel.windows.windows.insert(
        handle,
        Window {
            class: atom,
            title,
            style,
            ex_style,
            parent,
            rect,
        },
    );
    if !child && style & WS_VISIBLE != 0 && e.kernel.main_window == 0 {
        e.kernel.main_window = handle;
    }
    last_error(e, 0);
    handle
}

fn window(e: &mut Engine, handle: u32) -> Option<&mut Window> {
    e.kernel.windows.windows.get_mut(&handle)
}

// Platform: replaces USER32.DLL GetWindowLongA (import)
/// `GetWindowLongA(window, index)`: its style (`GWL_STYLE`) or extended style (`GWL_EXSTYLE`);
/// 0 with `ERROR_INVALID_WINDOW_HANDLE` for another handle. Other indexes are not provided.
pub fn get_window_long_a(e: &mut Engine, handle: u32, index: i32) -> u32 {
    let Some(w) = window(e, handle).cloned() else {
        last_error(e, ERROR_INVALID_WINDOW_HANDLE);
        return 0;
    };
    match index {
        GWL_STYLE => w.style,
        GWL_EXSTYLE => w.ex_style,
        _ => panic!("GetWindowLongA({index}) is not provided"),
    }
}

// Platform: replaces USER32.DLL GetClassLongA (import)
/// `GetClassLongA(window, index)`: the window's class menu name (`GCL_MENUNAME`); other indexes
/// are not provided.
pub fn get_class_long_a(e: &mut Engine, handle: u32, index: i32) -> u32 {
    let Some(w) = window(e, handle).cloned() else {
        last_error(e, ERROR_INVALID_WINDOW_HANDLE);
        return 0;
    };
    let class = e
        .kernel
        .windows
        .classes
        .iter()
        .find(|c| c.atom == w.class)
        .cloned()
        .expect("the window's class");
    match index {
        GCL_MENUNAME => class.menu_name,
        _ => panic!("GetClassLongA({index}) is not provided"),
    }
}

// Platform: replaces USER32.DLL SetWindowTextA (import)
/// `SetWindowTextA(window, text)`: the record's title becomes the zero-terminated ANSI `text`;
/// true (documented: nonzero on success). A handle that is no window answers `FALSE` with
/// `ERROR_INVALID_WINDOW_HANDLE`. With the real window (`NV_D3D9=real`) its title changes too.
/// The `WM_SETTEXT` it would send is not sent (module documentation); a null `text` (not
/// documented) is not provided. The model loader's status (`00446cb0`) titles the window with it.
pub fn set_window_text_a(e: &mut Engine, handle: u32, text: Ptr) -> u32 {
    assert!(
        !text.is_null(),
        "SetWindowTextA with a null text is not provided"
    );
    let title = e.mem.cstr(text.addr());
    let Some(w) = window(e, handle) else {
        last_error(e, ERROR_INVALID_WINDOW_HANDLE);
        return 0;
    };
    w.title = title.clone();
    #[cfg(windows)]
    super::d3d9real::window_titled(e, handle, &title);
    #[cfg(not(windows))]
    let _ = title;
    1
}

// Platform: replaces USER32.DLL SetWindowPos (import)
/// `SetWindowPos(window, after, x, y, width, height, flags)`: moves and sizes the record
/// (`SWP_NOSIZE` 1 and `SWP_NOMOVE` 2 honoured; the z order is not kept); true. The messages it
/// would send are not sent (module documentation).
#[allow(clippy::too_many_arguments)]
pub fn set_window_pos(
    e: &mut Engine,
    handle: u32,
    _after: u32,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    flags: u32,
) -> u32 {
    let Some(w) = window(e, handle) else {
        last_error(e, ERROR_INVALID_WINDOW_HANDLE);
        return 0;
    };
    let [left, top, right, bottom] = w.rect;
    let (mut size_w, mut size_h) = (right.wrapping_sub(left), bottom.wrapping_sub(top));
    let (mut nx, mut ny) = (left, top);
    if flags & 2 == 0 {
        nx = x;
        ny = y;
    }
    if flags & 1 == 0 {
        size_w = width;
        size_h = height;
    }
    w.rect = [nx, ny, nx.wrapping_add(size_w), ny.wrapping_add(size_h)];
    #[cfg(windows)]
    {
        let rect = w.rect;
        super::d3d9real::window_moved(e, handle, rect, flags);
    }
    1
}

// Platform: replaces USER32.DLL PeekMessageA (import)
/// `PeekMessageA(msg, window, first, last, remove)`: the next posted message into `msg`
/// (removed with `PM_REMOVE`), true; with none, the frame ends: the game's main thread hands
/// the engine back to the host until its next frame (`thread_host::end_main_frame`), then
/// false. Filtering by window or message range is not provided.
pub fn peek_message_a(
    e: &mut Engine,
    msg: Ptr,
    window: u32,
    first: u32,
    last: u32,
    remove: u32,
) -> u32 {
    assert!(
        window == 0 && first == 0 && last == 0,
        "PeekMessageA with a filter is not provided"
    );
    let next = if remove & PM_REMOVE != 0 {
        e.kernel.windows.queue.pop_front()
    } else {
        e.kernel.windows.queue.front().copied()
    };
    if let Some(words) = next {
        for (i, w) in words.iter().enumerate() {
            e.mem.set_u32(msg.addr() + 4 * i as u32, *w);
        }
        return 1;
    }
    e.kernel.windows.frames += 1;
    crate::thread_host::end_main_frame(e);
    0
}

// Platform: replaces USER32.DLL TranslateMessage (import)
/// `TranslateMessage(msg)`: key messages becoming character messages is not provided (the
/// queue holds no key messages: the game reads the keyboard through DirectInput).
pub fn translate_message(e: &mut Engine, msg: Ptr) -> u32 {
    let message = e.mem.u32(msg.addr() + 4);
    assert!(
        !(0x100..=0x109).contains(&message),
        "TranslateMessage of a key message is not provided"
    );
    0
}

// Platform: replaces USER32.DLL DispatchMessageA (import)
/// `DispatchMessageA(msg)`: calls the window's class procedure with the message's four words
/// and answers its result.
pub fn dispatch_message_a(e: &mut Engine, msg: Ptr) -> u32 {
    let m = msg.addr();
    let (window, message, w, l) = (
        e.mem.u32(m),
        e.mem.u32(m + 4),
        e.mem.u32(m + 8),
        e.mem.u32(m + 12),
    );
    let Some(class) = e.kernel.windows.windows.get(&window).map(|w| w.class) else {
        return 0;
    };
    let procedure = e
        .kernel
        .windows
        .classes
        .iter()
        .find(|c| c.atom == class)
        .map(|c| c.procedure)
        .expect("the window's class");
    e.call(procedure, &[window, message, w, l]).u32()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn wide_of(text: &str) -> Vec<u16> {
        text.encode_utf16().collect()
    }

    fn strings(args: Vec<Vec<u16>>) -> Vec<String> {
        args.into_iter()
            .map(|a| String::from_utf16_lossy(&a))
            .collect()
    }

    #[test]
    fn the_command_line_splits_as_windows_does() {
        assert_eq!(
            strings(split_command_line(&wide_of(
                "\"C:\\Games\\FalloutNV.exe\" -nocloud"
            ))),
            ["C:\\Games\\FalloutNV.exe", "-nocloud"]
        );
        assert_eq!(
            strings(split_command_line(&wide_of(
                r#"a.exe "b c" d\\"e" f\"g h"""i"#
            ))),
            ["a.exe", "b c", "d\\e", "f\"g", "h\"i"]
        );
        assert_eq!(strings(split_command_line(&wide_of("a.exe"))), ["a.exe"]);
    }

    /// `SetWindowTextA` titles the record and answers true; a handle that is no window answers
    /// `FALSE` with `ERROR_INVALID_WINDOW_HANDLE` and titles nothing.
    #[test]
    fn set_window_text_titles_the_record() {
        let mut e = Engine::new();
        e.kernel.windows.windows.insert(
            0x10,
            Window {
                class: 1,
                title: b"Fallout: New Vegas".to_vec(),
                style: 0,
                ex_style: 0,
                parent: 0,
                rect: [0, 0, 640, 480],
            },
        );
        let text = e.mem.alloc(32);
        e.mem.write(text, b"Loading 12/40\0");
        assert_eq!(set_window_text_a(&mut e, 0x10, Ptr::new(text)), 1);
        assert_eq!(e.kernel.windows.windows[&0x10].title, b"Loading 12/40");
        assert_eq!(set_window_text_a(&mut e, 0x99, Ptr::new(text)), 0);
        assert_eq!(e.kernel.last_error, ERROR_INVALID_WINDOW_HANDLE);
    }

    #[test]
    fn command_line_to_argv_makes_one_block() {
        let mut e = Engine::new();
        e.kernel.windows.command_line = wide_of("\"C:\\G\\FalloutNV.exe\" -nocloud");
        let line = get_command_line_w(&mut e);
        let count = e.mem.alloc(4);
        let block = command_line_to_argv_w(&mut e, Ptr::new(line), Ptr::new(count));
        assert_eq!(e.mem.u32(count), 2);
        let second = e.mem.u32(block + 4);
        assert_eq!(e.mem.u16(second), b'-' as u16);
        assert_eq!(wide(&e, second), wide_of("-nocloud"));
        assert_eq!(local_free(&mut e, block), 0);
    }

    #[test]
    fn a_second_mutex_of_the_same_name_reports_it_exists() {
        let mut e = Engine::new();
        let name = e.mem.alloc(32);
        e.mem.set_cstr(name, b"FalloutNVGameMutex");
        assert_ne!(create_mutex_a(&mut e, 0, 0, Ptr::new(name)), 0);
        assert_eq!(e.kernel.last_error, 0);
        create_mutex_a(&mut e, 0, 0, Ptr::new(name));
        assert_eq!(e.kernel.last_error, ERROR_ALREADY_EXISTS);
    }

    #[test]
    fn classes_register_once_and_find_window_finds_their_windows() {
        let mut e = Engine::new();
        let name = e.mem.alloc(32);
        e.mem.set_cstr(name, b"Fallout: New Vegas");
        let class = e.mem.alloc(40);
        e.mem.set_u32(class, 3);
        e.mem.set_u32(class + 4, 0x0086_a0a0);
        e.mem.set_u32(class + 36, name);
        assert_eq!(register_class_a(&mut e, Ptr::new(class)), FIRST_ATOM);
        assert_eq!(register_class_a(&mut e, Ptr::new(class)), 0);
        assert_eq!(e.kernel.last_error, ERROR_CLASS_ALREADY_EXISTS);
        assert_eq!(find_window_a(&mut e, name, Ptr::new(0)), 0);
    }

    #[test]
    fn peek_message_without_a_driver_answers_no_message() {
        let mut e = Engine::new();
        let msg = e.mem.alloc(28);
        assert_eq!(peek_message_a(&mut e, Ptr::new(msg), 0, 0, 0, 1), 0);
        assert_eq!(e.kernel.windows.frames, 1);
        e.kernel.windows.queue.push_back([7, 0x12, 0, 0, 0, 0, 0]);
        assert_eq!(peek_message_a(&mut e, Ptr::new(msg), 0, 0, 0, 1), 1);
        assert_eq!(e.mem.u32(msg + 4), 0x12);
    }

    #[test]
    #[should_panic(expected = "the game gave up")]
    fn a_message_box_stops_the_run_with_its_text() {
        let mut e = Engine::new();
        let text = e.mem.alloc(16);
        e.mem.set_cstr(text, b"no");
        message_box_a(&mut e, 0, Ptr::new(text), Ptr::new(text), 0);
    }
}

#[cfg(test)]
mod tests_steam {
    use super::*;

    /// The Steam imports answer the no-client state, registered under their names.
    #[test]
    fn the_steam_accessors_answer_null_and_steam_is_not_running() {
        let mut e = Engine::new();
        for name in [
            "SteamAPI_IsSteamRunning",
            "SteamUserStats",
            "SteamFriends",
            "SteamUtils",
            "SteamUser",
            "SteamApps",
        ] {
            let (_, _, f) = super::super::imports()
                .into_iter()
                .find(|(dll, n, _)| *dll == "steam_api.dll" && *n == name)
                .unwrap_or_else(|| panic!("{name} is not registered"));
            assert_eq!(f(&mut e, &[]).eax, 0, "{name}");
        }
    }

    /// `SteamAPI_RunCallbacks` is registered and leaves the state as it is.
    #[test]
    fn run_callbacks_does_nothing() {
        let mut e = Engine::new();
        let (_, _, f) = super::super::imports()
            .into_iter()
            .find(|(dll, n, _)| *dll == "steam_api.dll" && *n == "SteamAPI_RunCallbacks")
            .expect("SteamAPI_RunCallbacks is registered");
        e.kernel.last_error = 0x1234;
        for _ in 0..3 {
            assert_eq!(f(&mut e, &[]).eax, 0);
        }
        assert_eq!(e.kernel.last_error, 0x1234);
        assert_eq!(steam_user_stats(&mut e), 0);
    }

    /// `SteamAPI_Shutdown` is registered and, without `SteamAPI_Init`, leaves the state as it is.
    #[test]
    fn shutdown_without_init_does_nothing() {
        let mut e = Engine::new();
        let (_, _, f) = super::super::imports()
            .into_iter()
            .find(|(dll, n, _)| *dll == "steam_api.dll" && *n == "SteamAPI_Shutdown")
            .expect("SteamAPI_Shutdown is registered");
        e.kernel.last_error = 0x4321;
        for _ in 0..2 {
            assert_eq!(f(&mut e, &[]).eax, 0);
        }
        assert_eq!(e.kernel.last_error, 0x4321);
        assert_eq!(steam_user_stats(&mut e), 0);
    }
}

//! The platform's Direct3D 9 layer forwarded to the real `d3d9.dll` (ADR-0008), Windows only,
//! switched on with `NV_D3D9=real`.
//!
//! The game's code keeps calling the modelled objects of `super::d3d9`, `super::d3d9device` and
//! `super::d3d9resource`, and keeps getting the modelled answers (they were measured against the
//! same runtime). Each modelled call that changes what is drawn is also recorded here as a command
//! of the Direct3D 9 call stream (`d3d9_stream`), with the data it points at, for a backend that
//! makes it on a real counterpart: the real device (`super::d3d9backend`). The modelled window
//! gets a real Win32 window (here), the modelled device a real device on it, each modelled
//! resource a real resource whose contents follow the game's unlocks. Where the real runtime
//! answers differently from the model, the call is printed once (`NV_D3D9=real: ...`), and the
//! model's answer stands.
//!
//! Where the backend runs (`d3d9_stream::Stream`): on the calling thread, each command carried
//! out at once (the default: the same real calls in the same place as before the stream), or with
//! `NV_RENDER_THREAD=1` on a render thread of its own, so the game's threads only record (an
//! improvement over the original, which makes its Direct3D calls on its main thread: off by
//! default until measured). A call whose answer comes from the real device waits for it after
//! every earlier call (the device's creation, the read-back of a texture level the real device
//! keeps, `GetRenderTargetData`, a query's result); `Present` returns with at most one frame
//! left to draw (`NV_RENDER_THREAD_FRAMES`, default 1). `NV_D3D9_RECORD=FILE` writes every
//! command to a file (`d3d9_stream::wire`), `NV_D3D9_STREAM_CHECK=1` hashes the commands as
//! recorded and as carried out on the render thread and stops the run at a difference,
//! `NV_RENDER_THREAD_STATS=1` prints the hand-overs and waits once a second.
//!
//! The real window lives on a thread of its own that only creates it and pumps its messages, so
//! it never waits on the game's threads. It is shown without taking the focus: the viewer's window
//! keeps the keyboard and mouse the game reads through DirectInput.
//!
//! The real device is created with `D3DCREATE_FPU_PRESERVE` added to the game's flags: the
//! game's x87 precision switch is modelled (`crate::direct3d_device_created`,
//! from the game's own flags), and the backend's thread's floating-point state is not the game's.

use crate::Engine;
use d3d9_stream::{Cmd, Data, Reply, Stream};
use std::collections::{HashMap, HashSet};
use windows::core::PCSTR;
use windows::Win32::Foundation::{BOOL, HWND, LPARAM, LRESULT, POINT, RECT, WPARAM};
use windows::Win32::Graphics::Direct3D9::*;
use windows::Win32::Graphics::Gdi::ClientToScreen;
use windows::Win32::System::LibraryLoader::GetModuleHandleA;
use windows::Win32::UI::HiDpi::{
    SetThreadDpiAwarenessContext, DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2,
};
use windows::Win32::UI::Input::{
    GetRawInputData, RegisterRawInputDevices, HRAWINPUT, MOUSE_MOVE_ABSOLUTE, RAWINPUT,
    RAWINPUTDEVICE, RAWINPUTDEVICE_FLAGS, RAWINPUTHEADER, RID_INPUT, RIM_TYPEMOUSE,
};
use windows::Win32::UI::WindowsAndMessaging::*;

/// Whether the forwarding is asked for (`NV_D3D9=real`).
pub fn wanted() -> bool {
    std::env::var("NV_D3D9").is_ok_and(|v| v.eq_ignore_ascii_case("real"))
}

/// The forwarding's side on the game's threads (`Graphics::real`): the real windows by modelled
/// handle, the stream to the backend, and what was asked of it.
#[derive(Default)]
pub struct Real {
    windows: HashMap<u32, isize>,
    /// The stream to the backend, made with the first `IDirect3D9`.
    stream: Option<Stream>,
    /// The modelled devices the backend has a device for, each with the real device it handed to
    /// the layer's side channel (D3DX's texture loading on the game's threads), if any.
    devices: Vec<(u32, Option<IDirect3DDevice9>)>,
    /// The modelled objects whose counterpart was asked of the backend (made on a device it has):
    /// their binds, uploads and read-backs are forwarded. A counterpart the backend could not
    /// make answers for itself (`d3d9backend`: its draws skipped, its levels' bytes kept).
    forwarded: engine_core::AddrSet<u32>,
    /// Open locks of forwarded buffers: (offset, size, flags), innermost last.
    locks: engine_core::AddrMap<u32, Vec<(u32, u32, u32)>>,
    /// With the render thread: the real queries as the backend carried them out
    /// (`d3d9backend::QueryBoard`), and how many `D3DISSUE_END`s of each the game recorded.
    query_board: Option<std::sync::Arc<super::d3d9backend::QueryBoard>>,
    query_ends: engine_core::AddrMap<u32, u64>,
    /// How many queries the game made at each address (the board's `made`: a query made again at
    /// a destroyed one's address is not read as the old one).
    queries_made: engine_core::AddrMap<u32, u64>,
    /// The game's side's notes already printed.
    logged: HashSet<&'static str>,
    /// Tests: the real backend behind the stream, to look at ([`real_texture`]).
    #[cfg(test)]
    test_backend: Option<std::sync::Arc<std::sync::Mutex<super::d3d9backend::RealBackend>>>,
}

impl std::fmt::Debug for Real {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Real")
            .field("windows", &self.windows)
            .field("stream", &self.stream)
            .field("devices", &self.devices.len())
            .field("forwarded", &self.forwarded.len())
            .finish()
    }
}

impl Real {
    /// Tests: whether no answer of the real device has differed from the model's, once every
    /// command recorded is carried out.
    #[cfg(test)]
    pub(crate) fn logged_is_empty(&mut self) -> bool {
        if let Some(s) = self.stream.as_mut() {
            s.finish();
        }
        self.logged.is_empty()
            && self
                .test_backend
                .as_ref()
                .map_or(true, |b| b.lock().unwrap().logged.is_empty())
    }
}

fn real(e: &mut Engine) -> &mut Real {
    debug_assert!(
        engine_core::real_threads::device_lock_held(e),
        "the Direct3D 9 state reached without the device lock"
    );
    &mut state!(e).graphics.real
}

/// The real window of a modelled window handle, when it has one (under the device lock with real
/// threads: the window functions make them, the device presents in them).
pub fn real_window(e: &Engine, handle: u32) -> Option<isize> {
    let _device = engine_core::real_threads::device_guard(e);
    state_ref!(e).graphics.real.windows.get(&handle).copied()
}

/// Prints once a call the game's side could not forward (`NV_D3D9=real: ...`, as the backend
/// prints a call the real runtime answered otherwise).
#[cold]
#[inline(never)]
fn missing(e: &mut Engine, call: &'static str) {
    if real(e).logged.insert(call) {
        eprintln!(
            "NV_D3D9=real: {call}: the model answers 0x00000000, the real runtime 0xffffffff"
        );
    }
}

/// The `N` words at `at` (a structure a call passes by address), on the stack.
fn words<const N: usize>(e: &Engine, at: u32) -> [u32; N] {
    std::array::from_fn(|i| e.mem.u32(at + 4 * i as u32))
}

/// The stream, when the backend has a device for the modelled device `this` (the game makes
/// one: one compare).
#[inline]
fn stream_of(r: &mut Real, this: u32) -> Option<&mut Stream> {
    if r.devices.iter().any(|(d, _)| *d == this) {
        r.stream.as_mut()
    } else {
        None
    }
}

/// Records `cmd`, a call on the modelled device `this`, when the backend has its device.
#[inline]
fn record(e: &mut Engine, this: u32, cmd: Cmd<Data<'_>>) {
    if let Some(s) = stream_of(real(e), this) {
        s.submit(cmd);
    }
}

/// Records `cmd`, a call on the modelled object `object` (not a device), when its counterpart
/// was asked of the backend.
#[inline]
fn record_on(e: &mut Engine, object: u32, cmd: Cmd<Data<'_>>) {
    let r = real(e);
    if r.forwarded.contains(&object) {
        if let Some(s) = r.stream.as_mut() {
            s.submit(cmd);
        }
    }
}

/// A sync command's reply (`None` without a stream).
fn call(e: &mut Engine, cmd: Cmd<Data<'_>>) -> Option<Reply> {
    real(e).stream.as_mut().map(|s| s.submit(cmd))
}

/// The real device behind the modelled device `this`, counted, for a caller that hands it to
/// other code (the D3DX texture loader in `super::d3dxreal`, on the calling thread: the device
/// is `D3DCREATE_MULTITHREADED`); `None` when the forwarding is off, the real `CreateDevice`
/// failed, or the backend has no such device.
pub fn real_device(e: &Engine, this: u32) -> Option<IDirect3DDevice9> {
    let _device = engine_core::real_threads::device_guard(e);
    state_ref!(e)
        .graphics
        .real
        .devices
        .iter()
        .find(|(d, _)| *d == this)
        .and_then(|(_, real)| real.clone())
}

/// Whether any of the game's modelled devices has a real one behind it (for a D3DX call that
/// takes no device, the image info).
pub fn any_real_device(e: &Engine) -> bool {
    let _device = engine_core::real_threads::device_guard(e);
    state_ref!(e)
        .graphics
        .real
        .devices
        .iter()
        .any(|(_, real)| real.is_some())
}

/// Forgets the real counterpart of a modelled object that is destroyed (its address can be
/// reused): the backend lets it go when it reaches the command (the real one keeps it until the
/// next `Present`, `d3d9backend`).
pub fn forget(e: &mut Engine, object: u32) {
    let r = real(e);
    r.locks.remove(&object);
    r.query_ends.remove(&object);
    let device = match r.devices.iter().position(|(d, _)| *d == object) {
        Some(at) => {
            r.devices.remove(at);
            true
        }
        None => false,
    };
    if r.forwarded.remove(&object) || device {
        if let Some(s) = r.stream.as_mut() {
            s.submit(Cmd::Destroy { object });
        }
    }
}

/// Whether the modelled object's counterpart was asked of the backend.
fn is_forwarded(e: &Engine, object: u32) -> bool {
    state_ref!(e).graphics.real.forwarded.contains(&object)
}

// The window.

const CLASS_NAME: PCSTR = PCSTR(b"nv-rs-d3d9\0".as_ptr());

extern "system" fn window_procedure(
    window: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    // Closing the game's window ends the run (the player's way out, also when the game has
    // stopped); the window itself is not destroyed first.
    if message == WM_CLOSE {
        eprintln!("NV_D3D9=real: the game's window was closed: the run ends");
        std::process::exit(0);
    }
    match message {
        WM_KEYDOWN | WM_SYSKEYDOWN | WM_KEYUP | WM_SYSKEYUP | WM_LBUTTONDOWN | WM_LBUTTONUP
        | WM_RBUTTONDOWN | WM_RBUTTONUP | WM_MBUTTONDOWN | WM_MBUTTONUP | WM_XBUTTONDOWN
        | WM_XBUTTONUP | WM_MOUSEWHEEL => {
            if matches!(message, WM_LBUTTONDOWN | WM_RBUTTONDOWN | WM_MBUTTONDOWN) {
                hold_cursor(window);
            }
            window_input(window, message, wparam, lparam)
        }
        WM_INPUT => raw_mouse(lparam),
        // While the window is active the cursor stays in it, hidden (the game's DirectInput
        // mouse is exclusive and foreground: Windows hides and holds the cursor for it). It is
        // taken on a keyboard activation (Alt+Tab, WA_ACTIVE) or a click in the client area
        // (above); a click that activates the window elsewhere (WA_CLICKACTIVE, e.g. on the
        // title bar) leaves it free, and so does moving or sizing the window: holding it in the
        // client area there pulled the cursor below the title bar, the move loop followed it,
        // and the window slid down the screen (playtest, 2026-10-30).
        WM_ACTIVATE if wparam.0 & 0xffff == WA_ACTIVE as usize => hold_cursor(window),
        WM_ENTERSIZEMOVE => let_go_of_cursor(),
        WM_SIZE | WM_MOVE if cursor_held() => hold_cursor(window),
        WM_SETCURSOR if cursor_held() && lparam.0 as u32 & 0xffff == HTCLIENT => {
            unsafe { SetCursor(None) };
            return LRESULT(1);
        }
        // Losing the focus lets go of everything this window held.
        WM_ACTIVATE if wparam.0 & 0xffff == 0 => release_input(),
        WM_KILLFOCUS => release_input(),
        _ => {}
    }
    // Alt and F10 are the game's keys: they do not open the window menu (Alt+F4 still closes
    // nothing, as above).
    if message == WM_SYSKEYDOWN || message == WM_SYSKEYUP {
        return LRESULT(0);
    }
    unsafe { DefWindowProcA(window, message, wparam, lparam) }
}

// The real window as an input source (ADR-0008): while it has the focus, its keyboard and mouse
// (from the window's own messages) are held here; the host merges them into the
// game's DirectInput devices at each frame ([`take_input`]).

/// What the real window's keyboard and mouse did since the last frame.
#[derive(Debug, Clone, Copy)]
pub struct Input {
    /// The keys held, by DirectInput key code (the scan code, 0x80 added for extended keys).
    pub keys: [bool; 256],
    /// The mouse's relative motion and wheel (120 a notch) since the last frame.
    pub motion: [i32; 3],
    /// The buttons held, in DirectInput's order (left, right, middle, X1, X2).
    pub buttons: [bool; 8],
}

impl Default for Input {
    fn default() -> Self {
        Input {
            keys: [false; 256],
            motion: [0; 3],
            buttons: [false; 8],
        }
    }
}

fn input() -> &'static std::sync::Mutex<Input> {
    static INPUT: std::sync::OnceLock<std::sync::Mutex<Input>> = std::sync::OnceLock::new();
    INPUT.get_or_init(|| std::sync::Mutex::new(Input::default()))
}

fn release_input() {
    if let Ok(mut i) = input().lock() {
        i.keys = [false; 256];
        i.buttons = [false; 8];
    }
    let_go_of_cursor();
}

/// Frees the cursor [`hold_cursor`] held (no clip; Windows shows it again).
fn let_go_of_cursor() {
    if CURSOR_HELD.swap(false, std::sync::atomic::Ordering::Relaxed) {
        unsafe {
            let _ = ClipCursor(None);
            ShowCursor(true);
        }
    }
}

/// The centre of the held client area, in screen coordinates: the cursor is put back there after
/// each raw mouse motion, so it never rests on the area's edge (where it showed, playtest
/// 2026-11-03). The motion the game reads comes from raw input, not the cursor's position.
static CURSOR_CENTRE: [std::sync::atomic::AtomicI32; 2] = [
    std::sync::atomic::AtomicI32::new(0),
    std::sync::atomic::AtomicI32::new(0),
];

/// Whether the window holds the cursor (clipped to its client area, hidden over it).
static CURSOR_HELD: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

fn cursor_held() -> bool {
    CURSOR_HELD.load(std::sync::atomic::Ordering::Relaxed)
}

/// Holds the cursor in the window's client area and hides it there, as Windows does for a
/// program whose DirectInput mouse is exclusive and in the foreground (the game's:
/// `SetCooperativeLevel` with `DISCL_EXCLUSIVE | DISCL_FOREGROUND`).
fn hold_cursor(window: HWND) {
    unsafe {
        let mut rect = RECT::default();
        if GetClientRect(window, &mut rect).is_err() {
            return;
        }
        let mut corners = [
            POINT {
                x: rect.left,
                y: rect.top,
            },
            POINT {
                x: rect.right,
                y: rect.bottom,
            },
        ];
        for corner in &mut corners {
            let _ = ClientToScreen(window, corner);
        }
        let screen = RECT {
            left: corners[0].x,
            top: corners[0].y,
            right: corners[1].x,
            bottom: corners[1].y,
        };
        if ClipCursor(Some(&screen)).is_ok() {
            let centre = [
                (screen.left + screen.right) / 2,
                (screen.top + screen.bottom) / 2,
            ];
            for (slot, value) in CURSOR_CENTRE.iter().zip(centre) {
                slot.store(value, std::sync::atomic::Ordering::Relaxed);
            }
            if !CURSOR_HELD.swap(true, std::sync::atomic::Ordering::Relaxed) {
                ShowCursor(false);
            }
            SetCursor(None);
            let _ = SetCursorPos(centre[0], centre[1]);
        }
    }
}

/// Registers the window for the mouse's raw input: its relative counts are what DirectInput
/// hands an exclusive mouse (no pointer acceleration, not stopped at the screen's edge).
fn register_raw_mouse(window: HWND) {
    let device = RAWINPUTDEVICE {
        usUsagePage: 1,
        usUsage: 2,
        dwFlags: RAWINPUTDEVICE_FLAGS(0),
        hwndTarget: window,
    };
    let _ =
        unsafe { RegisterRawInputDevices(&[device], std::mem::size_of::<RAWINPUTDEVICE>() as u32) };
}

/// A raw input message: a mouse's relative motion is added to the frame's.
fn raw_mouse(lparam: LPARAM) {
    let mut raw = RAWINPUT::default();
    let mut size = std::mem::size_of::<RAWINPUT>() as u32;
    let read = unsafe {
        GetRawInputData(
            HRAWINPUT(lparam.0 as *mut _),
            RID_INPUT,
            Some(&mut raw as *mut RAWINPUT as *mut core::ffi::c_void),
            &mut size,
            std::mem::size_of::<RAWINPUTHEADER>() as u32,
        )
    };
    if read == u32::MAX || raw.header.dwType != RIM_TYPEMOUSE.0 {
        return;
    }
    let mouse = unsafe { raw.data.mouse };
    if mouse.usFlags.0 & MOUSE_MOVE_ABSOLUTE.0 != 0 {
        return;
    }
    if let Ok(mut i) = input().lock() {
        i.motion[0] += mouse.lLastX;
        i.motion[1] += mouse.lLastY;
    }
    if cursor_held() {
        let x = CURSOR_CENTRE[0].load(std::sync::atomic::Ordering::Relaxed);
        let y = CURSOR_CENTRE[1].load(std::sync::atomic::Ordering::Relaxed);
        let _ = unsafe { SetCursorPos(x, y) };
    }
}

/// The keys the real window holds, by DirectInput key code.
pub fn held_keys() -> [bool; 256] {
    input().lock().map(|i| i.keys).unwrap_or([false; 256])
}

/// The real window's input for this frame: the keys and buttons held, the motion since the last
/// call (taken: it starts again from 0).
pub fn take_input() -> Input {
    match input().lock() {
        Ok(mut i) => {
            let now = *i;
            i.motion = [0; 3];
            now
        }
        Err(_) => Input::default(),
    }
}

/// The DirectInput key code of a key message (`lParam`: scan code in bits 16..23, the extended
/// bit 24); `None` for the fake shifts Windows sends around some extended keys.
fn dik(lparam: isize) -> Option<u8> {
    let scan = ((lparam >> 16) & 0xff) as u8;
    let extended = lparam & (1 << 24) != 0;
    if scan == 0 {
        return None;
    }
    if extended {
        if scan == 0x2a || scan == 0x36 {
            return None;
        }
        // NumLock reports as extended 0x45 (DIK_NUMLOCK); others get 0x80 (DIK_RCONTROL,
        // DIK_RMENU, the arrows, DIK_NUMPADENTER, DIK_DIVIDE, ...).
        if scan == 0x45 {
            return Some(0x45);
        }
        return Some((scan & 0x7f) | 0x80);
    }
    // Pause reports as 0x45 without the extended bit (DIK_PAUSE 0xC5).
    if scan == 0x45 {
        return Some(0xc5);
    }
    Some(scan & 0x7f)
}

/// A message of the real window that is input (the mouse's motion comes from its raw input,
/// [`raw_mouse`]).
fn window_input(_window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) {
    let Ok(mut i) = input().lock() else { return };
    let button = |i: &mut Input, b: usize, down: bool| i.buttons[b] = down;
    match message {
        WM_KEYDOWN | WM_SYSKEYDOWN | WM_KEYUP | WM_SYSKEYUP => {
            // F12 (the first press, not its repeats): a report for the player to send
            // ([`shot::report_at_present`]). The key still reaches the game, as every key does.
            if message == WM_KEYDOWN && wparam.0 == 0x7b && lparam.0 & (1 << 30) == 0 {
                shot::report_asked();
            }
            if let Some(code) = dik(lparam.0) {
                i.keys[code as usize] = message == WM_KEYDOWN || message == WM_SYSKEYDOWN;
            }
        }
        WM_LBUTTONDOWN | WM_LBUTTONUP => button(&mut i, 0, message == WM_LBUTTONDOWN),
        WM_RBUTTONDOWN | WM_RBUTTONUP => button(&mut i, 1, message == WM_RBUTTONDOWN),
        WM_MBUTTONDOWN | WM_MBUTTONUP => button(&mut i, 2, message == WM_MBUTTONDOWN),
        WM_XBUTTONDOWN | WM_XBUTTONUP => {
            let b = if (wparam.0 >> 16) & 0xffff == 2 { 4 } else { 3 };
            button(&mut i, b, message == WM_XBUTTONDOWN);
        }
        WM_MOUSEWHEEL => i.motion[2] += ((wparam.0 >> 16) & 0xffff) as i16 as i32,
        _ => {}
    }
}

/// A window to make on the window thread: title, style, extended style, rectangle, the real
/// parent (0 for a top-level window), and where the handle goes (0 when it failed).
struct Request {
    title: Vec<u8>,
    style: u32,
    ex_style: u32,
    rect: [i32; 4],
    parent: isize,
    reply: std::sync::mpsc::Sender<isize>,
}

/// The window thread: it makes every real window (a child window's messages go to the thread
/// that made it, so parent and children share it) and pumps their messages.
fn window_thread() -> &'static std::sync::Mutex<std::sync::mpsc::Sender<Request>> {
    static THREAD: std::sync::OnceLock<std::sync::Mutex<std::sync::mpsc::Sender<Request>>> =
        std::sync::OnceLock::new();
    THREAD.get_or_init(|| {
        let (tx, rx) = std::sync::mpsc::channel::<Request>();
        let _ = std::thread::Builder::new()
            .name("nv-d3d9-window".into())
            .spawn(move || unsafe { run_window_thread(rx) });
        std::sync::Mutex::new(tx)
    })
}

/// Whether the game's window is shown without taking the focus (`NV_D3D9_BACKGROUND`, for the
/// scripted runs, so that one does not take the keyboard and the cursor from whoever works
/// beside it), and at the bottom of the window stack, behind that person's windows. Off, it is
/// shown as the game's `WS_VISIBLE` window is: activated.
fn shown_in_background() -> bool {
    std::env::var_os("NV_D3D9_BACKGROUND").is_some()
}

/// Whether the game's window is borderless and covers the primary monitor (`NV_BORDERLESS`, the
/// play launcher's setting; not the original's: its windowed mode keeps the frame). Direct3D
/// stretches the back buffer, at the game's own resolution, over it. The window thread is then
/// DPI aware, so the window covers the monitor's real pixels (with display scaling, an unaware
/// one is told the scaled size: 2048x1152 for 2560x1440 at 125%), and the game's child window
/// (the one it draws in, made at its resolution) is sized to fill it.
fn borderless() -> bool {
    std::env::var("NV_BORDERLESS").is_ok_and(|v| v == "1")
}

/// The borderless window ([`borderless`]), which keeps the monitor's size when the game moves or
/// sizes its window (0: none).
static BORDERLESS_WINDOW: std::sync::atomic::AtomicIsize = std::sync::atomic::AtomicIsize::new(0);

unsafe fn run_window_thread(rx: std::sync::mpsc::Receiver<Request>) {
    let module = GetModuleHandleA(None).unwrap_or_default();
    let class = WNDCLASSA {
        lpfnWndProc: Some(window_procedure),
        hInstance: module.into(),
        hCursor: LoadCursorW(None, IDC_ARROW).unwrap_or_default(),
        lpszClassName: CLASS_NAME,
        ..Default::default()
    };
    RegisterClassA(&class);
    if borderless() {
        SetThreadDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2);
    }
    loop {
        while let Ok(request) = rx.try_recv() {
            let mut title = request.title;
            title.push(0);
            let [left, top, right, bottom] = request.rect;
            let child = request.parent != 0;
            // A top-level window is shown afterwards (activated, or in the background for the
            // scripted runs: [`shown_in_background`]).
            let borderless = !child && borderless();
            let fills = child
                && request.parent != 0
                && request.parent == BORDERLESS_WINDOW.load(std::sync::atomic::Ordering::Relaxed);
            let style = if child {
                request.style
            } else if borderless {
                WS_POPUP.0
            } else {
                request.style & !WS_VISIBLE.0
            };
            let (left, top, right, bottom) = if borderless || fills {
                (
                    0,
                    0,
                    GetSystemMetrics(SM_CXSCREEN),
                    GetSystemMetrics(SM_CYSCREEN),
                )
            } else {
                (left, top, right, bottom)
            };
            let ex_style = if borderless { 0 } else { request.ex_style };
            let window = CreateWindowExA(
                WINDOW_EX_STYLE(ex_style),
                CLASS_NAME,
                PCSTR(title.as_ptr()),
                WINDOW_STYLE(style),
                left,
                top,
                right.wrapping_sub(left),
                bottom.wrapping_sub(top),
                HWND(request.parent as *mut _),
                None,
                module,
                None,
            );
            let handle = match window {
                Ok(w) => {
                    if !child {
                        register_raw_mouse(w);
                    }
                    if borderless {
                        BORDERLESS_WINDOW.store(w.0 as isize, std::sync::atomic::Ordering::Relaxed);
                    }
                    if !child && request.style & WS_VISIBLE.0 != 0 {
                        let show = if shown_in_background() {
                            SW_SHOWNOACTIVATE
                        } else {
                            SW_SHOW
                        };
                        let _ = ShowWindow(w, show);
                        // In the background a new window still lands on top of the stack, in
                        // front of whatever the person at the machine is using: put it at the
                        // bottom instead (it keeps drawing there; the game's own `SetWindowPos`
                        // calls keep the order, `SWP_NOZORDER` in `window_moved`).
                        if shown_in_background() {
                            let _ = SetWindowPos(
                                w,
                                HWND_BOTTOM,
                                0,
                                0,
                                0,
                                0,
                                SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE,
                            );
                        }
                    }
                    w.0 as isize
                }
                Err(_) => 0,
            };
            let _ = request.reply.send(handle);
        }
        MsgWaitForMultipleObjects(None, false, 10, QS_ALLINPUT);
        let mut msg = MSG::default();
        while PeekMessageA(&mut msg, None, 0, 0, PM_REMOVE).as_bool() {
            let _ = TranslateMessage(&msg);
            DispatchMessageA(&msg);
        }
    }
}

/// `CreateWindowExA` made the modelled window `handle` (a child of `parent` when that is not
/// 0): a real window of the same title, styles and rectangle (a child of the parent's real
/// window).
#[allow(clippy::too_many_arguments)]
pub fn window_created(
    e: &mut Engine,
    handle: u32,
    title: &[u8],
    style: u32,
    ex_style: u32,
    rect: [i32; 4],
    parent: u32,
) {
    if !wanted() {
        return;
    }
    let real_parent = if parent != 0 {
        match real_window(e, parent) {
            Some(p) => p,
            None => {
                eprintln!("NV_D3D9=real: a child window of a window without a real one");
                return;
            }
        }
    } else {
        0
    };
    let (reply, answer) = std::sync::mpsc::channel();
    let request = Request {
        title: title.to_vec(),
        style,
        ex_style,
        rect,
        parent: real_parent,
        reply,
    };
    let sent = window_thread()
        .lock()
        .map(|tx| tx.send(request).is_ok())
        .unwrap_or(false);
    match answer.recv().ok().filter(|&w| sent && w != 0) {
        Some(window) => {
            eprintln!(
                "NV_D3D9=real: window {handle:#x} (style {style:#x}, parent {parent:#x}, {rect:?}) is real window {window:#x}"
            );
            let _device = engine_core::real_threads::device_guard(e);
            real(e).windows.insert(handle, window);
        }
        None => eprintln!("NV_D3D9=real: the real window could not be made"),
    }
}

/// The real window takes the title the game gave its record (`SetWindowTextA`).
pub fn window_titled(e: &mut Engine, handle: u32, title: &[u8]) {
    let Some(window) = real_window(e, handle) else {
        return;
    };
    let mut text = title.to_vec();
    text.push(0);
    unsafe {
        let _ = SetWindowTextA(HWND(window as *mut _), PCSTR(text.as_ptr()));
    }
}

/// `SetWindowPos` moved or sized the modelled window `handle` (`SWP_NOSIZE`, `SWP_NOMOVE` in
/// `flags`): the real window follows.
pub fn window_moved(e: &mut Engine, handle: u32, rect: [i32; 4], flags: u32) {
    let Some(window) = real_window(e, handle) else {
        return;
    };
    let borderless = BORDERLESS_WINDOW.load(std::sync::atomic::Ordering::Relaxed);
    // The borderless window and the child filling it keep the monitor's size.
    if borderless != 0
        && (window == borderless
            || unsafe { GetParent(HWND(window as *mut _)) }
                .is_ok_and(|p| p.0 as isize == borderless))
    {
        return;
    }
    let [left, top, right, bottom] = rect;
    unsafe {
        let _ = SetWindowPos(
            HWND(window as *mut _),
            None,
            left,
            top,
            right.wrapping_sub(left),
            bottom.wrapping_sub(top),
            SET_WINDOW_POS_FLAGS(flags & 3) | SWP_NOZORDER | SWP_NOACTIVATE,
        );
    }
}

// The device.

/// `D3DCREATE_MULTITHREADED`: the real device takes calls from any thread.
const MULTITHREADED: u32 = 4;

/// `Direct3DCreate9` made a modelled `IDirect3D9`: the stream to the real backend (once), on the
/// calling thread or on the render thread (`NV_RENDER_THREAD=1`), written to a file with
/// `NV_D3D9_RECORD=FILE`.
pub fn direct3d_created(e: &mut Engine) {
    if !wanted() || real(e).stream.is_some() {
        return;
    }
    // `NV_FACE_CHECK` (a diagnostic) follows calls in the game's order on its own thread: it keeps
    // the calls on the game's threads, as diagnostics keep the threads stepped.
    let refused = d3d9_stream::render_thread_wanted() && face_check::enabled();
    if refused {
        eprintln!(
            "NV_D3D9=real: NV_RENDER_THREAD=1 is not taken with NV_FACE_CHECK: the calls stay on the game's threads"
        );
    }
    let mut stream = if d3d9_stream::render_thread_wanted() && !refused {
        let settings = d3d9_stream::Settings::from_env();
        let board = std::sync::Arc::new(super::d3d9backend::QueryBoard::default());
        let backend = Box::new(super::d3d9backend::RealBackend::with_query_board(
            std::sync::Arc::clone(&board),
        ));
        real(e).query_board = Some(board);
        eprintln!(
            "NV_D3D9=real: the render thread is on (NV_RENDER_THREAD=1; at most {} frame(s) in flight{})",
            settings.frames_in_flight,
            if settings.check { ", the stream checked" } else { "" }
        );
        Stream::threaded(backend, settings)
    } else {
        Stream::direct(Box::new(super::d3d9backend::RealBackend::new()))
    };
    if let Some(path) = std::env::var_os("NV_D3D9_RECORD") {
        let recording = std::fs::File::create(&path).and_then(|file| {
            stream.record_to(Box::new(std::io::BufWriter::with_capacity(1 << 20, file)))
        });
        match recording {
            Ok(()) => eprintln!(
                "NV_D3D9_RECORD: every Direct3D 9 command goes to {}",
                std::path::Path::new(&path).display()
            ),
            Err(error) => eprintln!(
                "NV_D3D9_RECORD: {} could not be written ({error}): not recorded",
                std::path::Path::new(&path).display()
            ),
        }
    }
    real(e).stream = Some(stream);
}

/// The game's `D3DPRESENT_PARAMETERS` (0x38 bytes) as words.
fn present_words(present: &[u8]) -> [u32; 14] {
    std::array::from_fn(|i| {
        present
            .get(4 * i..4 * i + 4)
            .map_or(0, |b| u32::from_le_bytes(b.try_into().expect("4 bytes")))
    })
}

/// `CreateDevice` made the modelled device `device` with these present parameters (as the
/// model filled them in), back buffers and automatic depth-stencil surface (0 for none): the
/// backend's device on the real window, its back buffers and depth surface the counterparts.
pub fn device_created(
    e: &mut Engine,
    device: u32,
    focus: u32,
    behavior: u32,
    present: &[u8],
    back_buffers: &[u32],
    depth_stencil: u32,
) {
    if real(e).stream.is_none() {
        return;
    }
    let words = present_words(present);
    let map = |e: &Engine, h: u32| -> Option<isize> {
        if h == 0 {
            return Some(0);
        }
        real_window(e, h)
    };
    let (Some(window), Some(focus)) = (map(e, words[7]), map(e, focus)) else {
        eprintln!("NV_D3D9=real: CreateDevice: the game's window has no real window");
        return;
    };
    // The real device takes calls from more than one thread at once: the D3DX side channel's
    // texture loading on the game's threads (thread 12) beside the device's own calls, under the
    // device lock or on the render thread, not under one lock with them. It must lock itself: the
    // game's own flags have that (0x54), and it is kept whatever flags a caller passes.
    let behavior = behavior | MULTITHREADED;
    let reply = call(
        e,
        Cmd::CreateDevice {
            device,
            focus,
            window,
            behavior,
            present: Data::from_words(&words),
            back_buffers: Data::from_words(back_buffers),
            depth_stencil,
        },
    );
    let Some(Reply::Device {
        created: true,
        native,
    }) = reply
    else {
        return;
    };
    let shared = native
        .and_then(|n| n.0.downcast::<super::d3d9backend::SharedDevice>().ok())
        .map(|s| s.0);
    let r = real(e);
    r.devices.retain(|(d, _)| *d != device);
    r.devices.push((device, shared));
    r.forwarded.extend(back_buffers.iter().copied());
    if depth_stencil != 0 {
        r.forwarded.insert(depth_stencil);
    }
}

/// `Reset` succeeded on the modelled device: the real device is reset with the same present
/// parameters (the real window), and its new back buffers and depth surface are the
/// counterparts of the modelled ones.
pub fn device_reset(
    e: &mut Engine,
    this: u32,
    present: &[u8],
    back_buffers: &[u32],
    depth_stencil: u32,
) {
    if stream_of(real(e), this).is_none() {
        return;
    }
    let words = present_words(present);
    let window = match words[7] {
        0 => 0,
        h => match real_window(e, h) {
            Some(w) => w,
            None => return missing(e, "Reset with a window without a real one"),
        },
    };
    record(
        e,
        this,
        Cmd::Reset {
            device: this,
            window,
            present: Data::from_words(&words),
            back_buffers: Data::from_words(back_buffers),
            depth_stencil,
        },
    );
    let r = real(e);
    r.forwarded.extend(back_buffers.iter().copied());
    if depth_stencil != 0 {
        r.forwarded.insert(depth_stencil);
    }
}

pub fn begin_scene(e: &mut Engine, this: u32, model: u32) {
    record(
        e,
        this,
        Cmd::BeginScene {
            device: this,
            model,
        },
    );
}

pub fn end_scene(e: &mut Engine, this: u32, model: u32) {
    record(
        e,
        this,
        Cmd::EndScene {
            device: this,
            model,
        },
    );
}

#[allow(clippy::too_many_arguments)]
pub fn clear(
    e: &mut Engine,
    this: u32,
    count: u32,
    rects: u32,
    flags: u32,
    colour: u32,
    z: u32,
    stencil: u32,
    model: u32,
) {
    if stream_of(real(e), this).is_none() {
        return;
    }
    // The rectangles' words on the stack up to 16 rectangles (the game clears one or none).
    let mut inline = [0u32; 64];
    let mut heap = Vec::new();
    let n = if rects != 0 { (4 * count) as usize } else { 0 };
    let w: &mut [u32] = if n <= inline.len() {
        &mut inline[..n]
    } else {
        heap.resize(n, 0);
        &mut heap
    };
    for (i, word) in w.iter_mut().enumerate() {
        *word = e.mem.u32(rects.wrapping_add(4 * i as u32));
    }
    let rect_words: Option<&[u32]> = (rects != 0).then_some(&*w);
    record(
        e,
        this,
        Cmd::Clear {
            device: this,
            count,
            rects: rect_words.map(Data::from_words),
            flags,
            colour,
            z,
            stencil,
            model,
        },
    );
}

/// The rectangle at `at` (left, top, right, bottom), none for a null pointer.
fn rect_words(e: &Engine, at: u32) -> Option<[u32; 4]> {
    (at != 0).then(|| words(e, at))
}

pub fn present(
    e: &mut Engine,
    this: u32,
    source: u32,
    dest: u32,
    window: u32,
    dirty: u32,
    model: u32,
) {
    if stream_of(real(e), this).is_none() {
        return;
    }
    let window = match window {
        0 => 0,
        h => match real_window(e, h) {
            Some(w) => w,
            None => return missing(e, "Present to a window without a real one"),
        },
    };
    let (source, dest) = (rect_words(e, source), rect_words(e, dest));
    record(
        e,
        this,
        Cmd::Present {
            device: this,
            source: source.as_ref().map(|r| Data::from_words(r)),
            dest: dest.as_ref().map(|r| Data::from_words(r)),
            window,
            dirty: dirty != 0,
            model,
            frame: 0,
        },
    );
}

// The device's state.

pub fn set_render_target(e: &mut Engine, this: u32, index: u32, surface: u32, model: u32) {
    let _ = model;
    record(
        e,
        this,
        Cmd::SetRenderTarget {
            device: this,
            index,
            surface,
        },
    );
}

pub fn set_depth_stencil_surface(e: &mut Engine, this: u32, surface: u32, model: u32) {
    let _ = model;
    record(
        e,
        this,
        Cmd::SetDepthStencilSurface {
            device: this,
            surface,
        },
    );
}

pub fn set_viewport(e: &mut Engine, this: u32, viewport: u32, model: u32) {
    let _ = model;
    if viewport == 0 {
        return;
    }
    let viewport = words(e, viewport);
    record(
        e,
        this,
        Cmd::SetViewport {
            device: this,
            viewport,
        },
    );
}

pub fn set_transform(e: &mut Engine, this: u32, state: u32, matrix: u32, model: u32) {
    let _ = model;
    if matrix == 0 {
        return;
    }
    let m: [u32; 16] = words(e, matrix);
    record(
        e,
        this,
        Cmd::SetTransform {
            device: this,
            state,
            matrix: Data::from_words(&m),
        },
    );
}

pub fn set_scissor_rect(e: &mut Engine, this: u32, at: u32, model: u32) {
    let _ = model;
    let Some(r) = rect_words(e, at) else { return };
    record(
        e,
        this,
        Cmd::SetScissorRect {
            device: this,
            rect: r.map(|w| w as i32),
        },
    );
}

/// `SetGammaRamp` on the real device with the game's ramp (`void` in the SDK: nothing to
/// compare with the model's answer).
pub fn set_gamma_ramp(
    e: &mut Engine,
    this: u32,
    swap_chain: u32,
    flags: u32,
    ramp: u32,
    model: u32,
) {
    let _ = model;
    if stream_of(real(e), this).is_none() {
        return;
    }
    let bytes = e.mem.bytes(ramp, 0x600);
    record(
        e,
        this,
        Cmd::SetGammaRamp {
            device: this,
            swap_chain,
            flags,
            ramp: Data::Bytes(&bytes),
        },
    );
}

pub fn set_clip_plane(e: &mut Engine, this: u32, index: u32, plane: u32, model: u32) {
    let _ = model;
    if plane == 0 {
        return;
    }
    let plane = words(e, plane);
    record(
        e,
        this,
        Cmd::SetClipPlane {
            device: this,
            index,
            plane,
        },
    );
}

pub fn set_render_state(e: &mut Engine, this: u32, state: u32, value: u32, model: u32) {
    let _ = model;
    record(
        e,
        this,
        Cmd::SetRenderState {
            device: this,
            state,
            value,
        },
    );
}

pub fn set_texture_stage_state(
    e: &mut Engine,
    this: u32,
    stage: u32,
    kind: u32,
    value: u32,
    model: u32,
) {
    let _ = model;
    record(
        e,
        this,
        Cmd::SetTextureStageState {
            device: this,
            stage,
            kind,
            value,
        },
    );
}

pub fn set_sampler_state(
    e: &mut Engine,
    this: u32,
    sampler: u32,
    kind: u32,
    value: u32,
    model: u32,
) {
    let _ = model;
    record(
        e,
        this,
        Cmd::SetSamplerState {
            device: this,
            sampler,
            kind,
            value,
        },
    );
}

pub fn set_texture(e: &mut Engine, this: u32, sampler: u32, texture: u32, model: u32) {
    let _ = model;
    record(
        e,
        this,
        Cmd::SetTexture {
            device: this,
            sampler,
            texture,
        },
    );
}

// Textures and surfaces.

/// `CreateTexture` made the modelled texture `texture` with these levels (largest first): the
/// real texture with the same arguments, its levels the counterparts of the modelled levels.
#[allow(clippy::too_many_arguments)]
pub fn texture_created(
    e: &mut Engine,
    this: u32,
    texture: u32,
    levels: &[u32],
    width: u32,
    height: u32,
    usage: u32,
    format: u32,
    pool: u32,
) {
    let r = real(e);
    let Some(s) = stream_of(r, this) else { return };
    s.submit(Cmd::CreateTexture {
        device: this,
        texture,
        levels: Data::from_words(levels),
        width,
        height,
        usage,
        format,
        pool,
    });
    r.forwarded.insert(texture);
    r.forwarded.extend(levels.iter().copied());
}

/// `CreateCubeTexture` made the modelled cube texture `texture` with these surfaces, face by
/// face (each face's chain largest first).
#[allow(clippy::too_many_arguments)]
pub fn cube_created(
    e: &mut Engine,
    this: u32,
    texture: u32,
    surfaces: &[u32],
    edge: u32,
    usage: u32,
    format: u32,
    pool: u32,
) {
    let r = real(e);
    let Some(s) = stream_of(r, this) else { return };
    s.submit(Cmd::CreateCubeTexture {
        device: this,
        texture,
        surfaces: Data::from_words(surfaces),
        edge,
        usage,
        format,
        pool,
    });
    r.forwarded.insert(texture);
    r.forwarded.extend(surfaces.iter().copied());
}

/// `CreateRenderTarget` (`depth` false) or `CreateDepthStencilSurface` made the modelled
/// surface `surface`; `flag` is `Lockable` or `Discard`.
#[allow(clippy::too_many_arguments)]
pub fn surface_created(
    e: &mut Engine,
    this: u32,
    surface: u32,
    depth: bool,
    width: u32,
    height: u32,
    format: u32,
    multisample: u32,
    quality: u32,
    flag: u32,
) {
    let r = real(e);
    let Some(s) = stream_of(r, this) else { return };
    s.submit(Cmd::CreateSurface {
        device: this,
        surface,
        depth,
        width,
        height,
        format,
        multisample,
        quality,
        flag,
    });
    r.forwarded.insert(surface);
}

/// Whether the modelled surface `surface` has a real counterpart (asked of the backend; one it
/// could not make keeps its bytes there, so the level reads back what it was given either way).
pub fn has_surface(e: &Engine, surface: u32) -> bool {
    state_ref!(e).graphics.real.forwarded.contains(&surface)
}

/// The modelled `surface` as the backend keeps it read into engine memory at `at` (`size`
/// bytes, rows packed): waits for every earlier command; false without a counterpart, or when
/// its size differs or it can't be read.
pub fn read_surface(e: &mut Engine, surface: u32, at: u32, size: u32) -> bool {
    if !has_surface(e, surface) {
        return false;
    }
    match call(e, Cmd::ReadSurface { surface, size }) {
        Some(Reply::Bytes(Some(bytes))) => {
            e.mem.write(at, &bytes);
            true
        }
        _ => false,
    }
}

/// Fills `into` with the bytes from `offset` of `rows` rows of `row` bytes lying `pitch` apart
/// at `bits` (as if packed).
fn read_rows(
    mem: &engine_core::Mem,
    bits: u32,
    pitch: u32,
    row: u32,
    offset: u32,
    into: &mut [u8],
) {
    if pitch == row || row == 0 {
        mem.read(bits + offset, into);
        return;
    }
    let mut done = 0usize;
    while done < into.len() {
        let at = offset + done as u32;
        let (y, x) = (at / row, at % row);
        let n = ((row - x) as usize).min(into.len() - done);
        mem.read(bits + y * pitch + x, &mut into[done..done + n]);
        done += n;
    }
}

/// The game unlocked the modelled surface `surface`, whose memory in engine memory is `bits`
/// (`rows` rows of `row` bytes, `pitch` apart): the bytes go to its counterpart. Whether they
/// went (false without one).
pub fn surface_unlocked(
    e: &mut Engine,
    surface: u32,
    bits: u32,
    pitch: u32,
    row: u32,
    rows: u32,
) -> bool {
    if !has_surface(e, surface) {
        return false;
    }
    let mem = &e.mem;
    let r = &mut crate::state_mut(&mut e.kernel.platform).graphics.real;
    let Some(s) = r.stream.as_mut() else {
        return false;
    };
    let read = |offset: u32, into: &mut [u8]| read_rows(mem, bits, pitch, row, offset, into);
    s.submit(Cmd::UploadSurface {
        surface,
        row,
        rows,
        data: Data::Read {
            len: row * rows,
            read: &read,
        },
    });
    true
}

// Buffers, shaders, declarations.

/// `D3DLOCK_READONLY`.
const LOCK_READONLY: u32 = 0x10;

/// `CreateVertexBuffer` (`index` false) or `CreateIndexBuffer` made the modelled buffer
/// `buffer`; `format` is the FVF or the index format.
#[allow(clippy::too_many_arguments)]
pub fn buffer_created(
    e: &mut Engine,
    this: u32,
    buffer: u32,
    index: bool,
    length: u32,
    usage: u32,
    format: u32,
    pool: u32,
) {
    let r = real(e);
    let Some(s) = stream_of(r, this) else { return };
    s.submit(Cmd::CreateBuffer {
        device: this,
        buffer,
        index,
        length,
        usage,
        format,
        pool,
    });
    r.forwarded.insert(buffer);
    r.locks.remove(&buffer);
}

/// The game locked the modelled buffer `buffer` (recorded here for its unlock).
pub fn buffer_locked(e: &mut Engine, buffer: u32, offset: u32, size: u32, flags: u32, model: u32) {
    let r = real(e);
    if model != 0 || !r.forwarded.contains(&buffer) {
        return;
    }
    r.locks
        .entry(buffer)
        .or_default()
        .push((offset, size, flags));
}

/// The game unlocked the modelled buffer `buffer`, whose memory is `bits` (`length` bytes):
/// the locked range goes to its counterpart.
pub fn buffer_unlocked(e: &mut Engine, buffer: u32, bits: u32, length: u32, model: u32) {
    if model != 0 {
        return;
    }
    let mem = &e.mem;
    let r = &mut crate::state_mut(&mut e.kernel.platform).graphics.real;
    let Some((offset, size, flags)) = r.locks.get_mut(&buffer).and_then(|l| l.pop()) else {
        return;
    };
    if flags & LOCK_READONLY != 0 {
        return;
    }
    let Some(s) = r.stream.as_mut() else { return };
    let size = if size == 0 { length - offset } else { size };
    let read = |at: u32, into: &mut [u8]| mem.read(bits + offset + at, into);
    s.submit(Cmd::UploadBuffer {
        buffer,
        offset,
        flags,
        data: Data::Read {
            len: size,
            read: &read,
        },
    });
}

/// `CreateVertexShader` / `CreatePixelShader` made the modelled shader `shader` from these
/// tokens.
pub fn shader_created(e: &mut Engine, this: u32, shader: u32, pixel: bool, tokens: &[u8]) {
    let r = real(e);
    let Some(s) = stream_of(r, this) else { return };
    // The tokens end with the end token: whole words (a stray tail padded, as the real call
    // reads words).
    let words = as_words(tokens);
    s.submit(Cmd::CreateShader {
        device: this,
        shader,
        pixel,
        tokens: Data::from_words(&words),
    });
    r.forwarded.insert(shader);
}

fn as_words(bytes: &[u8]) -> Vec<u32> {
    bytes
        .chunks(4)
        .map(|c| {
            let mut w = [0u8; 4];
            w[..c.len()].copy_from_slice(c);
            u32::from_le_bytes(w)
        })
        .collect()
}

/// `CreateVertexDeclaration` made the modelled declaration `declaration` from these elements
/// (`D3DDECL_END` included).
pub fn declaration_created(e: &mut Engine, this: u32, declaration: u32, elements: &[u8]) {
    let r = real(e);
    let Some(s) = stream_of(r, this) else { return };
    s.submit(Cmd::CreateDeclaration {
        device: this,
        declaration,
        elements: Data::Bytes(elements),
    });
    r.forwarded.insert(declaration);
}

pub fn set_vertex_declaration(e: &mut Engine, this: u32, declaration: u32, model: u32) {
    let _ = model;
    record(
        e,
        this,
        Cmd::SetVertexDeclaration {
            device: this,
            declaration,
        },
    );
}

pub fn set_fvf(e: &mut Engine, this: u32, fvf: u32, model: u32) {
    let _ = model;
    record(e, this, Cmd::SetFvf { device: this, fvf });
}

pub fn set_shader(e: &mut Engine, this: u32, shader: u32, pixel: bool, model: u32) {
    let _ = model;
    record(
        e,
        this,
        Cmd::SetShader {
            device: this,
            pixel,
            shader,
        },
    );
}

/// `Set{Vertex,Pixel}ShaderConstant{F,I,B}`: kinds 0..2 vertex float, int, bool; 3..5 pixel.
/// Called once the model took the call (`D3D_OK`): the model has just read the registers'
/// words from `data` into the device's bindings, and nothing ran since, so the command carries
/// those words (`count` registers of 4 words, or of one word for booleans) without reading the
/// program's memory again.
pub fn set_constants(
    e: &mut Engine,
    this: u32,
    kind: u8,
    start: u32,
    data: u32,
    count: u32,
    model: u32,
) {
    let _ = model;
    if data == 0 {
        return;
    }
    let mem = &e.mem;
    let g = &mut crate::state_mut(&mut e.kernel.platform).graphics;
    let Some(s) = stream_of(&mut g.real, this) else {
        return;
    };
    let registers = super::d3d9resource::constant_registers(
        &g.device_objects.resources,
        this,
        kind,
        start,
        count,
    );
    let Some(registers) = registers else {
        return set_constants_as_given(mem, s, this, kind, start, data, count);
    };
    if kind % 3 == 2 {
        // A boolean register keeps its word first: packed (at most 16 registers).
        let mut packed = [0u32; 16];
        for (w, r) in packed.iter_mut().zip(registers) {
            *w = r[0];
        }
        s.submit(Cmd::SetConstants {
            device: this,
            kind,
            start,
            count,
            data: Data::from_words(&packed[..registers.len().min(16)]),
        });
    } else {
        // SAFETY: `[u32; 4]`s are 4 words each, laid out one after the other.
        let flat = unsafe {
            std::slice::from_raw_parts(registers.as_ptr().cast::<u32>(), 4 * registers.len())
        };
        s.submit(Cmd::SetConstants {
            device: this,
            kind,
            start,
            count,
            data: Data::from_words(flat),
        });
    }
}

/// [`set_constants`] when the model holds no registers for the range (not reached: the model
/// keeps every register it took): the words read as given, out of the common path.
#[cold]
#[inline(never)]
#[allow(clippy::too_many_arguments)]
fn set_constants_as_given(
    mem: &engine_core::Mem,
    s: &mut Stream,
    this: u32,
    kind: u8,
    start: u32,
    data: u32,
    count: u32,
) {
    let per = if kind % 3 == 2 { 1 } else { 4 };
    let words: Vec<u32> = (0..per * count).map(|i| mem.u32(data + 4 * i)).collect();
    s.submit(Cmd::SetConstants {
        device: this,
        kind,
        start,
        count,
        data: Data::from_words(&words),
    });
}

pub fn set_stream_source(
    e: &mut Engine,
    this: u32,
    stream: u32,
    buffer: u32,
    offset: u32,
    stride: u32,
    model: u32,
) {
    let _ = model;
    record(
        e,
        this,
        Cmd::SetStreamSource {
            device: this,
            stream,
            buffer,
            offset,
            stride,
        },
    );
}

pub fn set_indices(e: &mut Engine, this: u32, buffer: u32, model: u32) {
    let _ = model;
    record(
        e,
        this,
        Cmd::SetIndices {
            device: this,
            buffer,
        },
    );
}

// Draws.

pub fn draw_primitive(e: &mut Engine, this: u32, kind: u32, start: u32, count: u32, model: u32) {
    let _ = model;
    record(
        e,
        this,
        Cmd::DrawPrimitive {
            device: this,
            kind,
            start,
            count,
        },
    );
}

#[allow(clippy::too_many_arguments)]
pub fn draw_indexed_primitive(
    e: &mut Engine,
    this: u32,
    kind: u32,
    base_vertex: u32,
    min_index: u32,
    vertices: u32,
    start_index: u32,
    count: u32,
    model: u32,
) {
    let _ = model;
    record(
        e,
        this,
        Cmd::DrawIndexedPrimitive {
            device: this,
            kind,
            base_vertex: base_vertex as i32,
            min_index,
            vertices,
            start_index,
            count,
        },
    );
}

/// `DrawPrimitiveUP`: the `vertices * stride` bytes at `data` from engine memory.
#[allow(clippy::too_many_arguments)]
pub fn draw_primitive_up(
    e: &mut Engine,
    this: u32,
    kind: u32,
    count: u32,
    data: u32,
    stride: u32,
    vertices: u32,
    model: u32,
) {
    let _ = model;
    let mem = &e.mem;
    let r = &mut crate::state_mut(&mut e.kernel.platform).graphics.real;
    let Some(s) = stream_of(r, this) else { return };
    let read = |at: u32, into: &mut [u8]| mem.read(data + at, into);
    s.submit(Cmd::DrawPrimitiveUp {
        device: this,
        kind,
        count,
        stride,
        vertices: Data::Read {
            len: vertices * stride,
            read: &read,
        },
    });
}

// Copies between resources, fixed-function lighting.

pub fn update_texture(e: &mut Engine, this: u32, source: u32, destination: u32, model: u32) {
    let _ = model;
    record(
        e,
        this,
        Cmd::UpdateTexture {
            device: this,
            source,
            destination,
        },
    );
}

#[allow(clippy::too_many_arguments)]
pub fn stretch_rect(
    e: &mut Engine,
    this: u32,
    source: u32,
    source_rect: u32,
    destination: u32,
    destination_rect: u32,
    filter: u32,
    model: u32,
) {
    let _ = model;
    let (a, b) = (rect_words(e, source_rect), rect_words(e, destination_rect));
    record(
        e,
        this,
        Cmd::StretchRect {
            device: this,
            source,
            source_rect: a.as_ref().map(|r| Data::from_words(r)),
            destination,
            destination_rect: b.as_ref().map(|r| Data::from_words(r)),
            filter,
        },
    );
}

pub fn set_material(e: &mut Engine, this: u32, material: u32, model: u32) {
    let _ = model;
    if material == 0 {
        return;
    }
    let m: [u32; 17] = words(e, material);
    record(
        e,
        this,
        Cmd::SetMaterial {
            device: this,
            material: Data::from_words(&m),
        },
    );
}

pub fn set_light(e: &mut Engine, this: u32, index: u32, light: u32, model: u32) {
    let _ = model;
    if light == 0 {
        return;
    }
    let l: [u32; 26] = words(e, light);
    record(
        e,
        this,
        Cmd::SetLight {
            device: this,
            index,
            light: Data::from_words(&l),
        },
    );
}

pub fn light_enable(e: &mut Engine, this: u32, index: u32, enable: u32, model: u32) {
    let _ = model;
    record(
        e,
        this,
        Cmd::LightEnable {
            device: this,
            index,
            enable,
        },
    );
}

pub fn set_software_vertex_processing(e: &mut Engine, this: u32, software: u32, model: u32) {
    let _ = model;
    record(
        e,
        this,
        Cmd::SetSoftwareVertexProcessing {
            device: this,
            software,
        },
    );
}

// Queries, offscreen surfaces, fills and read-backs.

/// `CreateQuery` made the modelled query `query` of type `kind`: the real query.
pub fn query_created(e: &mut Engine, this: u32, query: u32, kind: u32) {
    let r = real(e);
    let Some(s) = stream_of(r, this) else { return };
    s.submit(Cmd::CreateQuery {
        device: this,
        query,
        kind,
    });
    r.forwarded.insert(query);
    *r.queries_made.entry(query).or_default() += 1;
}

/// `D3DISSUE_END`, `D3DGETDATA_FLUSH`, `S_FALSE`.
const ISSUE_END: u32 = 1;
const GETDATA_FLUSH: u32 = 1;
const S_FALSE: u32 = 1;

pub fn query_issue(e: &mut Engine, query: u32, flags: u32) {
    if flags == ISSUE_END && is_forwarded(e, query) {
        *real(e).query_ends.entry(query).or_default() += 1;
    }
    record_on(e, query, Cmd::QueryIssue { query, flags });
}

/// `GetData` of the real query after the game's `END`: its answer and its 4-byte result; `None`
/// without a real query.
///
/// Carried out at once, it is the real query's answer after every earlier call. With the render
/// thread (an improvement, not the original's order) it does not wait for the render thread to
/// catch up where it needs not: once the render thread has issued the game's last `END`, the real
/// query answers at once, the same call with the same flags (`S_FALSE` while the GPU is not done).
/// Before that, a call without `D3DGETDATA_FLUSH` answers `S_FALSE`, which Direct3D gives whenever
/// the result is not in yet, and the commands recorded so far are handed over so it comes; a call
/// with `D3DGETDATA_FLUSH` (the game's sun tests poll with it once a frame, its reference volumes
/// in a loop until ready: `00b63540`, `00b63680`) waits for the render thread, as before.
pub fn query_get_data(e: &mut Engine, query: u32, flags: u32) -> Option<(u32, u32)> {
    if !is_forwarded(e, query) {
        return None;
    }
    if let Some(answer) = query_without_waiting(real(e), query, flags) {
        return answer;
    }
    match call(e, Cmd::QueryGetData { query, flags }) {
        Some(Reply::Query(answer)) => answer,
        _ => None,
    }
}

/// [`query_get_data`] with the render thread, without waiting for it: `None` without the render
/// thread, or for a `D3DGETDATA_FLUSH` read whose `END` the real device has not issued yet (the
/// call waits then).
fn query_without_waiting(r: &mut Real, query: u32, flags: u32) -> Option<Option<(u32, u32)>> {
    let board = r.query_board.as_ref()?;
    let recorded = r.query_ends.get(&query).copied().unwrap_or(0);
    let made = r.queries_made.get(&query).copied().unwrap_or(0);
    let ready = match board.lock().get(&query) {
        // This query (not an earlier one at its address): its creation failed on the real
        // device: no real query, as before; or its last `END` issued there.
        Some(q) if q.made == made => match &q.query {
            None => return Some(None),
            Some(real) if q.ends >= recorded => Some(real.0.clone()),
            Some(_) => None,
        },
        // Not made, or the game's last `END` not issued, on the real device yet.
        _ => None,
    };
    match ready {
        Some(q) => {
            if let Some(s) = r.stream.as_mut() {
                s.note("query results read at once");
            }
            Some(Some(super::d3d9backend::query_data(&q, flags)))
        }
        None if flags & GETDATA_FLUSH != 0 => None,
        None => {
            if let Some(s) = r.stream.as_mut() {
                s.flush();
                s.note("query results not in yet");
            }
            Some(Some((S_FALSE, 0)))
        }
    }
}

/// `CreateOffscreenPlainSurface` made the modelled surface `surface`.
pub fn offscreen_created(
    e: &mut Engine,
    this: u32,
    surface: u32,
    width: u32,
    height: u32,
    format: u32,
    pool: u32,
) {
    let r = real(e);
    let Some(s) = stream_of(r, this) else { return };
    s.submit(Cmd::CreateOffscreenSurface {
        device: this,
        surface,
        width,
        height,
        format,
        pool,
    });
    r.forwarded.insert(surface);
}

pub fn color_fill(e: &mut Engine, this: u32, surface: u32, at: u32, colour: u32, model: u32) {
    let _ = model;
    let area = rect_words(e, at);
    record(
        e,
        this,
        Cmd::ColorFill {
            device: this,
            surface,
            rect: area.as_ref().map(|r| Data::from_words(r)),
            colour,
        },
    );
}

/// `GetRenderTargetData` from the real render target into the real destination, whose rows are
/// then copied into the destination's memory at `bits` (`pitch` apart; nothing copied for 0),
/// after every earlier command; false without real surfaces (the caller stops).
pub fn get_render_target_data(
    e: &mut Engine,
    this: u32,
    source: u32,
    destination: u32,
    bits: u32,
    pitch: u32,
) -> bool {
    if stream_of(real(e), this).is_none() {
        return false;
    }
    let reply = call(
        e,
        Cmd::GetRenderTargetData {
            device: this,
            source,
            destination,
            pitch: if bits == 0 { 0 } else { pitch },
        },
    );
    let Some(Reply::Rows(Some(rows))) = reply else {
        return false;
    };
    if bits != 0 && rows.row != 0 {
        for (y, row) in rows.bytes.chunks(rows.row as usize).enumerate() {
            e.mem.write(bits + y as u32 * pitch, row);
        }
    }
    true
}

/// `SetLOD` on a managed modelled texture: the real texture's `SetLOD` with the same level; a
/// different previous level than the model's is printed once.
pub fn texture_set_lod(e: &mut Engine, texture: u32, lod: u32, model: u32) {
    if !is_forwarded(e, texture) {
        return missing(e, "SetLOD of a texture without a real one");
    }
    record_on(
        e,
        texture,
        Cmd::SetLod {
            texture,
            lod,
            model,
        },
    );
}

// ---------------------------------------------------------------------------------------------
// `NV_SHOT=DIR[,every_ms]`: diagnostic only, not exe behaviour. Screenshots of the game's own
// frames on the real device: at a `Present`, at most every `every_ms` (default 5000) of wall
// time, the back buffer is copied to a system-memory surface (`GetRenderTargetData`; a
// multisampled back buffer is first resolved into a plain render target with `StretchRect`)
// and written to `DIR` as a 32-bit BMP named `frame-NNNNNN-<unix ms>.bmp`. `NV_SHOT_FRAMES=FROM-TO`
// takes them only at those presents (counted from 1), so a short stretch can be taken every frame.
// Nothing is read or made when `NV_SHOT` is unset; the game's state and the model are not touched.
// ---------------------------------------------------------------------------------------------
pub(super) mod shot {
    use super::*;
    use std::path::PathBuf;
    use std::sync::{Mutex, OnceLock};
    use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

    /// The directory and the interval (`NV_SHOT=DIR[,every_ms]`), read once.
    pub(crate) struct Shots {
        pub dir: PathBuf,
        pub every: Duration,
        /// `NV_SHOT_FRAMES=FROM-TO`: only these presents (both included).
        pub frames: Option<(u64, u64)>,
    }

    /// `NV_SHOT_FRAMES=FROM-TO`: the first and last present to take.
    pub(super) fn parse_frames(value: &str) -> Option<(u64, u64)> {
        let (from, to) = value.trim().split_once('-')?;
        Some((from.trim().parse().ok()?, to.trim().parse().ok()?))
    }

    /// `NV_SHOT=DIR[,every_ms]`: the directory and interval (5000 ms when not given).
    pub(super) fn parse(value: &str) -> Option<Shots> {
        let value = value.trim();
        if value.is_empty() {
            return None;
        }
        let (dir, every) = match value.rsplit_once(',') {
            Some((dir, ms)) => match ms.trim().parse::<u64>() {
                Ok(ms) => (dir, ms),
                Err(_) => (value, 5000),
            },
            None => (value, 5000),
        };
        Some(Shots {
            dir: PathBuf::from(dir),
            every: Duration::from_millis(every.max(1)),
            frames: None,
        })
    }

    pub(crate) fn wanted() -> Option<&'static Shots> {
        static SHOTS: OnceLock<Option<Shots>> = OnceLock::new();
        SHOTS
            .get_or_init(|| {
                let mut shots = std::env::var("NV_SHOT").ok().and_then(|v| parse(&v))?;
                shots.frames = std::env::var("NV_SHOT_FRAMES")
                    .ok()
                    .and_then(|v| parse_frames(&v));
                let _ = std::fs::create_dir_all(&shots.dir);
                Some(shots)
            })
            .as_ref()
    }

    /// When the last shot was taken.
    fn timing() -> std::sync::MutexGuard<'static, Option<Instant>> {
        static T: OnceLock<Mutex<Option<Instant>>> = OnceLock::new();
        T.get_or_init(Default::default)
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    /// A top-down 32-bit BMP of `width` x `height` B, G, R, X pixels.
    pub(super) fn bmp(width: u32, height: u32, bgrx: &[u8]) -> Vec<u8> {
        let image = width * height * 4;
        let mut out = Vec::with_capacity(54 + image as usize);
        out.extend_from_slice(b"BM");
        out.extend_from_slice(&(54 + image).to_le_bytes());
        out.extend_from_slice(&0u32.to_le_bytes());
        out.extend_from_slice(&54u32.to_le_bytes());
        out.extend_from_slice(&40u32.to_le_bytes());
        out.extend_from_slice(&(width as i32).to_le_bytes());
        // A negative height: rows top first.
        out.extend_from_slice(&(-(height as i32)).to_le_bytes());
        out.extend_from_slice(&1u16.to_le_bytes());
        out.extend_from_slice(&32u16.to_le_bytes());
        out.extend_from_slice(&0u32.to_le_bytes());
        out.extend_from_slice(&image.to_le_bytes());
        out.extend_from_slice(&[0u8; 16]);
        out.extend_from_slice(bgrx);
        out
    }

    /// At `Present` number `frame` (the stream's count from 1, the same for every backend of a
    /// run): one shot when the interval has passed since the last.
    pub(crate) fn at_present(d: &IDirect3DDevice9, shots: &Shots, frame: u32) {
        let frame = u64::from(frame);
        {
            let mut last = timing();
            if shots
                .frames
                .is_some_and(|(from, to)| frame < from || frame > to)
            {
                return;
            }
            if last.is_some_and(|last| last.elapsed() < shots.every) {
                return;
            }
            *last = Some(Instant::now());
        }
        let wall = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |d| d.as_millis());
        let path = shots.dir.join(format!("frame-{frame:06}-{wall}.bmp"));
        match capture(d) {
            Ok((width, height, pixels)) => {
                if let Err(error) = std::fs::write(&path, bmp(width, height, &pixels)) {
                    eprintln!("NV_SHOT: {} not written: {error}", path.display());
                }
            }
            Err(error) => eprintln!("NV_SHOT: frame {frame}: {error}"),
        }
    }

    /// `NV_REPORT_DIR=DIR`: where F12 puts a report for the player to send, read once.
    fn report_dir() -> Option<&'static PathBuf> {
        static DIR: OnceLock<Option<PathBuf>> = OnceLock::new();
        DIR.get_or_init(|| std::env::var_os("NV_REPORT_DIR").map(PathBuf::from))
            .as_ref()
    }

    static REPORT_ASKED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

    /// F12 on the window's thread: the next `Present` makes the report.
    pub(super) fn report_asked() {
        if report_dir().is_some() {
            REPORT_ASKED.store(true, std::sync::atomic::Ordering::Relaxed);
        }
    }

    /// The first unused numbered folder under `dir`, made (`001`, `002`, ...).
    pub(super) fn new_report_folder(dir: &std::path::Path) -> Option<PathBuf> {
        std::fs::create_dir_all(dir).ok()?;
        (1..10_000).find_map(|n| {
            let folder = dir.join(format!("{n:03}"));
            std::fs::create_dir(&folder).ok().map(|()| folder)
        })
    }

    /// The first `bytes` of the file at `path`, to the last whole line.
    pub(super) fn head(path: &std::path::Path, bytes: u64) -> Option<Vec<u8>> {
        use std::io::Read;
        let mut text = Vec::new();
        std::fs::File::open(path)
            .ok()?
            .take(bytes)
            .read_to_end(&mut text)
            .ok()?;
        if text.len() as u64 == bytes {
            let end = text
                .iter()
                .rposition(|&b| b == b'\n')
                .map_or(0, |at| at + 1);
            text.truncate(end);
        }
        Some(text)
    }

    /// The last `bytes` of the file at `path`, from a line's start.
    pub(super) fn tail(path: &std::path::Path, bytes: u64) -> Option<Vec<u8>> {
        use std::io::{Read, Seek, SeekFrom};
        let mut file = std::fs::File::open(path).ok()?;
        let size = file.metadata().ok()?.len();
        file.seek(SeekFrom::Start(size.saturating_sub(bytes)))
            .ok()?;
        let mut text = Vec::new();
        file.read_to_end(&mut text).ok()?;
        if size > bytes {
            let line = text.iter().position(|&b| b == b'\n').map_or(0, |at| at + 1);
            text.drain(..line);
        }
        Some(text)
    }

    /// At a `Present` after F12: `NV_REPORT_DIR\NNN\` with the frame (`picture.bmp`), the log's
    /// end (`log.txt`, from the file `NV_REPORT_LOG` names) and a `note.txt` for the player's own
    /// words; a beep says it is done. The game's state and the model are not touched.
    pub(crate) fn report_at_present(d: &IDirect3DDevice9) {
        if !REPORT_ASKED.swap(false, std::sync::atomic::Ordering::Relaxed) {
            return;
        }
        let Some(folder) = report_dir().and_then(|dir| new_report_folder(dir)) else {
            eprintln!("F12: the report's folder could not be made");
            return;
        };
        match capture(d) {
            Ok((width, height, pixels)) => {
                let _ = std::fs::write(folder.join("picture.bmp"), bmp(width, height, &pixels));
            }
            Err(error) => eprintln!("F12: no picture: {error}"),
        }
        if let Some(path) = std::env::var_os("NV_REPORT_LOG") {
            let path = std::path::Path::new(&path);
            if let Some(text) = tail(path, 256 * 1024) {
                let _ = std::fs::write(folder.join("log.txt"), text);
            }
            // The log's start too (`log-start.txt`): the run's preset, fixes and threads lines,
            // which a long session's end no longer reaches.
            if let Some(text) = head(path, 8 * 1024) {
                let _ = std::fs::write(folder.join("log-start.txt"), text);
            }
        }
        let _ = std::fs::write(
            folder.join("note.txt"),
            "Write here what went wrong and what you did just before it:\r\n\r\n",
        );
        eprintln!("F12: report in {}", folder.display());
        #[link(name = "user32")]
        extern "system" {
            fn MessageBeep(kind: u32) -> i32;
        }
        // SAFETY: no pointers; 0 is `MB_OK`, the default sound.
        unsafe {
            MessageBeep(0);
        }
    }

    /// The back buffer's pixels (rows top first, B, G, R, X) through a system-memory copy.
    fn capture(d: &IDirect3DDevice9) -> Result<(u32, u32, Vec<u8>), String> {
        let back = unsafe { d.GetBackBuffer(0, 0, D3DBACKBUFFER_TYPE_MONO) }
            .map_err(|e| format!("GetBackBuffer: {e}"))?;
        let mut desc = D3DSURFACE_DESC::default();
        unsafe { back.GetDesc(&mut desc) }.map_err(|e| format!("GetDesc: {e}"))?;
        if desc.Format != D3DFMT_X8R8G8B8 && desc.Format != D3DFMT_A8R8G8B8 {
            return Err(format!("back buffer format {} not provided", desc.Format.0));
        }
        let (width, height) = (desc.Width, desc.Height);
        // A multisampled back buffer is resolved into a plain render target first.
        let source = if desc.MultiSampleType != D3DMULTISAMPLE_NONE {
            let mut plain = None;
            unsafe {
                d.CreateRenderTarget(
                    width,
                    height,
                    desc.Format,
                    D3DMULTISAMPLE_NONE,
                    0,
                    BOOL(0),
                    &mut plain,
                    std::ptr::null_mut(),
                )
            }
            .map_err(|e| format!("CreateRenderTarget: {e}"))?;
            let plain = plain.ok_or("CreateRenderTarget: no surface")?;
            unsafe {
                d.StretchRect(
                    &back,
                    std::ptr::null(),
                    &plain,
                    std::ptr::null(),
                    D3DTEXF_NONE,
                )
            }
            .map_err(|e| format!("StretchRect: {e}"))?;
            plain
        } else {
            back
        };
        let mut copy = None;
        unsafe {
            d.CreateOffscreenPlainSurface(
                width,
                height,
                desc.Format,
                D3DPOOL_SYSTEMMEM,
                &mut copy,
                std::ptr::null_mut(),
            )
        }
        .map_err(|e| format!("CreateOffscreenPlainSurface: {e}"))?;
        let copy = copy.ok_or("CreateOffscreenPlainSurface: no surface")?;
        unsafe { d.GetRenderTargetData(&source, &copy) }
            .map_err(|e| format!("GetRenderTargetData: {e}"))?;
        let mut locked = D3DLOCKED_RECT::default();
        unsafe { copy.LockRect(&mut locked, std::ptr::null(), D3DLOCK_READONLY as u32) }
            .map_err(|e| format!("LockRect: {e}"))?;
        let row = width as usize * 4;
        let mut pixels = Vec::with_capacity(row * height as usize);
        for y in 0..height as usize {
            let line = unsafe {
                std::slice::from_raw_parts(
                    (locked.pBits as *const u8).add(y * locked.Pitch as usize),
                    row,
                )
            };
            pixels.extend_from_slice(line);
        }
        let _ = unsafe { copy.UnlockRect() };
        Ok((width, height, pixels))
    }
}

// ---------------------------------------------------------------------------------------------
// `NV_FACE_CHECK`: diagnostic only, not exe behaviour. Reads, never writes, game memory or the
// real device's state (the read-back locks the real surface read-only).
//
// Follows the player's FaceGen face texture on the real device. Nothing is printed until the
// FaceGen side has built a face image (`0064ceb0`, which stores its size in
// `platform::FACE_CHECK_IMAGES`); the first uncompressed texture of that size created after it is
// the face texture (NiDX9SourceTextureData's CreateSurf), and only it is followed, with its own
// line budget:
// - `d3d face texture` line: its size, format, levels, usage, pool.
// - `d3d face upload` lines: each level copied to the real surface (rows, the model's row bytes,
//   the real pitch, the byte means of the copied data and of the real surface read back; with
//   `NV_FACE_CHECK_DIR` levels 0 and 3 are written there as TGA files read back from the real
//   surface).
// - `d3d face bind` lines: it bound to a sampler; then `d3d face draw` for every indexed draw while
//   it is bound and for the next 16 draws after it leaves a sampler (the hair and eyes follow the
//   face): every sampler's texture (size, format, levels, and after `|` the file each was made
//   from, `platform::FACE_CHECK_NAMES`), the pixel and vertex shaders and the draw's counts;
//   each distinct draw at most twice.
//   Each such draw line also gives the primitive type, base vertex, min index and start index, and
//   whether the real draw was skipped (a binding without a real object); it is followed by
//   `d3d face states` (z, alpha test and blend, cull, stencil, colour write states as forwarded),
//   `d3d face pixel constants` (c0..c31) and `d3d face vertex constants` (c0..c95), every
//   register set so far, as floats.
//   Meshes once drawn with the face texture are followed in their other passes too (`face mesh,
//   another pass`: the head's first, depth-writing pass binds no face texture). For the first 12
//   frames after the face texture exists, `d3d face frame N` counts the indexed draws, those with
//   ZWRITEENABLE on and those skipped; once, `d3d face projection transform` prints D3DTS_PROJECTION.
//   Each reported draw also gets `d3d face states set since the previous draw` (the
//   SetRenderState calls between the previous indexed draw and this one: a state missing there was
//   left from an earlier draw), and other meshes drawn with a pixel shader a face mesh used are
//   reported too (`another mesh, same pixel shader as a face pass`: an NPC head to compare with).
// ---------------------------------------------------------------------------------------------
pub(super) mod face_check {
    use std::collections::HashMap;
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::sync::{Mutex, OnceLock};

    pub(crate) fn enabled() -> bool {
        static ON: OnceLock<bool> = OnceLock::new();
        *ON.get_or_init(|| std::env::var_os("NV_FACE_CHECK").is_some())
    }

    static LINES: AtomicU32 = AtomicU32::new(0);
    const LIMIT: u32 = 3000;

    pub(super) fn line(text: std::fmt::Arguments) {
        if LINES.fetch_add(1, Ordering::Relaxed) < LIMIT {
            println!("face check: {text}");
        }
    }

    #[derive(Default)]
    pub(crate) struct State {
        /// Every texture created: (width, height, format, levels).
        pub textures: HashMap<u32, (u32, u32, u32, u32)>,
        /// The followed face textures and their level surfaces: surface -> (texture, level).
        pub faces: Vec<u32>,
        pub levels: HashMap<u32, (u32, u32)>,
        /// The modelled texture each sampler holds (as forwarded).
        pub samplers: HashMap<u32, u32>,
        pub pixel_shader: u32,
        pub vertex_shader: u32,
        /// Draws still to report after the face texture left the samplers.
        pub after: u32,
        pub dumps: u32,
        /// How often each draw (counts, shaders, samplers) was reported: each at most twice.
        pub seen: HashMap<String, u32>,
        /// The render states and the float shader constants as forwarded (register -> 4 words).
        pub render_states: HashMap<u32, u32>,
        pub pixel_constants: HashMap<u32, [u32; 4]>,
        pub vertex_constants: HashMap<u32, [u32; 4]>,
        /// The transforms set (`D3DTRANSFORMSTATETYPE` -> 16 words).
        pub transforms: HashMap<u32, Vec<u32>>,
        /// The render states set since the last indexed draw (in order), and the pixel shaders the
        /// face meshes were drawn with (other meshes drawn with them are reported for comparison).
        pub states_since_draw: Vec<(u32, u32)>,
        pub face_shaders: std::collections::HashSet<u32>,
        /// The meshes drawn with the face texture, by (vertices, primitives): their other passes are
        /// reported too.
        pub face_meshes: std::collections::HashSet<(u32, u32)>,
        /// This frame's indexed draws: all, with ZWRITEENABLE on, skipped; frames summed so far.
        pub frame_draws: [u32; 3],
        pub frames: u32,
        pub projection_shown: bool,
    }

    pub(crate) fn state() -> std::sync::MutexGuard<'static, State> {
        static STATE: OnceLock<Mutex<State>> = OnceLock::new();
        STATE
            .get_or_init(Default::default)
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    /// Whether a `D3DFORMAT` is a four-character DXT code.
    pub(super) fn compressed(format: u32) -> bool {
        (u32::from_le_bytes(*b"DXT1")..=u32::from_le_bytes(*b"DXT5")).contains(&format)
            && format.to_le_bytes()[..3] == *b"DXT"
    }

    /// Whether a texture of this size and format is the face texture of the face image `image`
    /// (`width << 32 | height`, 0 for none yet).
    pub(super) fn is_face(image: u64, width: u32, height: u32, format: u32) -> bool {
        image != 0 && !compressed(format) && (u64::from(width) << 32 | u64::from(height)) == image
    }

    /// The mean of each of the four bytes of 32-bit pixels.
    pub(super) fn means(bytes: &[u8]) -> [u64; 4] {
        let mut sum = [0u64; 4];
        let mut n = 0u64;
        for pixel in bytes.chunks_exact(4) {
            for c in 0..4 {
                sum[c] += u64::from(pixel[c]);
            }
            n += 1;
        }
        sum.map(|s| s / n.max(1))
    }

    /// A 32-bit B, G, R, A image as an uncompressed TGA (rows top first).
    pub(super) fn tga(width: u32, height: u32, bgra: &[u8]) -> Vec<u8> {
        let mut file = vec![0u8; 18];
        file[2] = 2;
        file[12..14].copy_from_slice(&(width as u16).to_le_bytes());
        file[14..16].copy_from_slice(&(height as u16).to_le_bytes());
        file[16] = 32;
        file[17] = 0x28;
        file.extend_from_slice(bgra);
        file
    }
}

/// `NV_FACE_CHECK`: a texture was created (diagnostic only); the face texture is followed.
pub(super) fn face_check_texture(
    texture: u32,
    levels: &[u32],
    size: (u32, u32),
    format: u32,
    usage: u32,
    pool: u32,
) {
    let mut s = face_check::state();
    s.textures
        .insert(texture, (size.0, size.1, format, levels.len() as u32));
    // One texture per face image: the first one of its size made after it.
    let matched = {
        let mut pending = super::FACE_CHECK_IMAGES
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let found = pending
            .iter()
            .position(|&image| face_check::is_face(image, size.0, size.1, format));
        found.map(|at| pending.remove(at))
    };
    if matched.is_none() {
        return;
    }
    s.faces.push(texture);
    for (i, &level) in levels.iter().enumerate() {
        s.levels.insert(level, (texture, i as u32));
    }
    drop(s);
    face_check::line(format_args!(
        "d3d face texture {texture:#x} {}x{} format {format} levels {} usage {usage:#x} pool {pool}",
        size.0,
        size.1,
        levels.len()
    ));
}

/// `NV_FACE_CHECK`: a level of the face texture went to the real surface.
pub(super) fn face_check_upload(
    surface: u32,
    real_surface: &IDirect3DSurface9,
    model: &[u8],
    row: u32,
    rows: u32,
) {
    let Some(&(texture, level)) = face_check::state().levels.get(&surface) else {
        return;
    };
    let (width, height, format, _) = face_check::state()
        .textures
        .get(&texture)
        .copied()
        .unwrap_or_default();
    let mut desc = D3DSURFACE_DESC::default();
    let _ = unsafe { real_surface.GetDesc(&mut desc) };
    let mut locked = D3DLOCKED_RECT::default();
    let read =
        unsafe { real_surface.LockRect(&mut locked, std::ptr::null(), D3DLOCK_READONLY as u32) };
    let mut back = Vec::new();
    let mut pitch = 0;
    if read.is_ok() {
        pitch = locked.Pitch;
        for y in 0..rows {
            let from = unsafe {
                std::slice::from_raw_parts(
                    (locked.pBits as *const u8).add(y as usize * locked.Pitch as usize),
                    row as usize,
                )
            };
            back.extend_from_slice(from);
        }
        let _ = unsafe { real_surface.UnlockRect() };
    }
    face_check::line(format_args!(
        "d3d face upload texture {texture:#x} level {level} ({}x{} of {width}x{height}, format {format}): {rows} rows of {row} bytes, real pitch {pitch} format {}, model bytes mean {:?}, real read back {} mean {:?}, same {}",
        desc.Width,
        desc.Height,
        desc.Format.0,
        face_check::means(model),
        if read.is_ok() { "ok" } else { "failed" },
        face_check::means(&back),
        back == model,
    ));
    let bytes_per_pixel = row.checked_div(desc.Width).unwrap_or(0);
    if (level == 0 || level == 3) && bytes_per_pixel == 4 && back.len() == model.len() {
        if let Some(dir) = std::env::var_os("NV_FACE_CHECK_DIR") {
            let n = {
                let mut s = face_check::state();
                s.dumps += 1;
                s.dumps
            };
            if n <= 40 {
                let path = std::path::Path::new(&dir).join(format!(
                    "d3d-face-{n}-{texture:x}-level{level}-{}x{}.tga",
                    desc.Width, desc.Height
                ));
                let _ = std::fs::write(&path, face_check::tga(desc.Width, desc.Height, &back));
                face_check::line(format_args!(
                    "d3d face upload written to {}",
                    path.display()
                ));
            }
        }
    }
}

/// `NV_FACE_CHECK`: a sampler took a texture.
pub(super) fn face_check_bind(sampler: u32, texture: u32) {
    let mut s = face_check::state();
    let old = s.samplers.insert(sampler, texture).unwrap_or(0);
    if s.faces.is_empty() {
        return;
    }
    let is_face = s.faces.contains(&texture);
    if !is_face && s.faces.contains(&old) {
        // The face texture leaves this sampler: the next draws (the hair, the eyes) are reported.
        s.after = 16;
    }
    drop(s);
    if is_face {
        face_check::line(format_args!(
            "d3d face bind texture {texture:#x} to sampler {sampler}"
        ));
    }
}

/// `NV_FACE_CHECK`: a shader was set.
pub(super) fn face_check_shader(shader: u32, pixel: bool) {
    let mut s = face_check::state();
    if pixel {
        s.pixel_shader = shader;
    } else {
        s.vertex_shader = shader;
    }
}

/// `NV_FACE_CHECK`: a frame ends: its indexed draw counts (the first 12 frames once the face texture
/// exists) and, once, the projection transform as set.
pub(super) fn face_check_present() {
    let mut s = face_check::state();
    if s.faces.is_empty() {
        return;
    }
    let counts = std::mem::take(&mut s.frame_draws);
    s.frames += 1;
    let frame = s.frames;
    let projection = if s.projection_shown {
        None
    } else {
        s.projection_shown = true;
        Some(s.transforms.get(&3).cloned())
    };
    drop(s);
    if frame <= 12 {
        face_check::line(format_args!(
            "d3d face frame {frame}: {} indexed draws, {} with ZWRITEENABLE on, {} skipped",
            counts[0], counts[1], counts[2]
        ));
    }
    if let Some(projection) = projection {
        let text = match projection {
            Some(m) => m
                .iter()
                .map(|w| format!("{:.4}", f32::from_bits(*w)))
                .collect::<Vec<_>>()
                .join(","),
            None => "never set".into(),
        };
        face_check::line(format_args!("d3d face projection transform (rows): {text}"));
    }
}

/// `NV_FACE_CHECK`: an indexed draw with the face texture bound, or one of the few after it.
/// `NV_FACE_CHECK`: a render state was set.
pub(super) fn face_check_render_state(state: u32, value: u32) {
    let mut s = face_check::state();
    s.render_states.insert(state, value);
    s.states_since_draw.push((state, value));
}

/// `NV_FACE_CHECK`: float shader constants were set (`pixel` for the pixel shader's).
/// `NV_FACE_CHECK`: float constants set (`words`: the registers, 4 words each, as the call gave
/// them).
pub(super) fn face_check_constants(pixel: bool, start: u32, words: &[u32]) {
    let mut s = face_check::state();
    for (i, register) in words.chunks_exact(4).take(256).enumerate() {
        let i = i as u32;
        let value = [register[0], register[1], register[2], register[3]];
        let map = if pixel {
            &mut s.pixel_constants
        } else {
            &mut s.vertex_constants
        };
        map.insert(start + i, value);
    }
}

/// The render states the face check prints, with their `D3DRENDERSTATETYPE` numbers.
const FACE_CHECK_STATES: [(&str, u32); 14] = [
    ("ZENABLE", 7),
    ("FILLMODE", 8),
    ("ZWRITEENABLE", 14),
    ("ALPHATESTENABLE", 15),
    ("SRCBLEND", 19),
    ("DESTBLEND", 20),
    ("CULLMODE", 22),
    ("ZFUNC", 23),
    ("ALPHAREF", 24),
    ("ALPHAFUNC", 25),
    ("ALPHABLENDENABLE", 27),
    ("STENCILENABLE", 52),
    ("COLORWRITEENABLE", 168),
    ("BLENDOP", 171),
];

/// The determinant of the upper-left 3x3 of three rows of four float words (a negative one
/// mirrors, flipping the triangles' winding).
fn face_check_det3(rows: [&[u32]; 3]) -> f32 {
    let m = |r: usize, c: usize| f32::from_bits(rows[r][c]);
    m(0, 0) * (m(1, 1) * m(2, 2) - m(1, 2) * m(2, 1))
        - m(0, 1) * (m(1, 0) * m(2, 2) - m(1, 2) * m(2, 0))
        + m(0, 2) * (m(1, 0) * m(2, 1) - m(1, 1) * m(2, 0))
}

/// The determinants the face check prints: view (2), projection (3), world (256) when set, and
/// of vertex constants c0..c2 (rows) when set.
pub(super) fn face_check_determinants(s: &face_check::State) -> String {
    let mut parts = Vec::new();
    for (name, state) in [("view", 2u32), ("projection", 3), ("world", 256)] {
        if let Some(m) = s.transforms.get(&state) {
            parts.push(format!(
                "{name} {:.4}",
                face_check_det3([&m[0..4], &m[4..8], &m[8..12]])
            ));
        }
    }
    if let (Some(a), Some(b), Some(c)) = (
        s.vertex_constants.get(&0),
        s.vertex_constants.get(&1),
        s.vertex_constants.get(&2),
    ) {
        parts.push(format!("vertex c0..c2 {:.4}", face_check_det3([a, b, c])));
    }
    parts.join(", ")
}

/// One line of float constants: `cN=(x,y,z,w)` for each register set so far, in order.
pub(super) fn face_check_constant_list(map: &HashMap<u32, [u32; 4]>, below: u32) -> String {
    let mut registers: Vec<u32> = map.keys().copied().filter(|&r| r < below).collect();
    registers.sort();
    registers
        .iter()
        .map(|r| {
            let v = map[r].map(f32::from_bits);
            format!("c{r}=({:.3},{:.3},{:.3},{:.3})", v[0], v[1], v[2], v[3])
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// `NV_FACE_CHECK`: an indexed draw with the face texture bound, or one of the few after it.
#[allow(clippy::too_many_arguments)]
pub(super) fn face_check_draw(
    kind: u32,
    base_vertex: u32,
    min_index: u32,
    vertices: u32,
    start_index: u32,
    count: u32,
    skipped: bool,
) {
    let mut s = face_check::state();
    let since = std::mem::take(&mut s.states_since_draw);
    if s.faces.is_empty() {
        return;
    }
    let zwrite = s.render_states.get(&14).copied().unwrap_or(1) != 0;
    s.frame_draws[0] += 1;
    s.frame_draws[1] += u32::from(zwrite);
    s.frame_draws[2] += u32::from(skipped);
    let bound = s.samplers.values().any(|t| s.faces.contains(t));
    if bound {
        s.face_meshes.insert((vertices, count));
    }
    let face_mesh = s.face_meshes.contains(&(vertices, count));
    if face_mesh {
        let shader = s.pixel_shader;
        s.face_shaders.insert(shader);
    }
    let label = if bound {
        "with the face texture"
    } else if face_mesh {
        "face mesh, another pass"
    } else if s.after > 0 {
        s.after -= 1;
        "after the face"
    } else if s.face_shaders.contains(&s.pixel_shader) {
        "another mesh, same pixel shader as a face pass"
    } else {
        return;
    };
    let mut samplers: Vec<(u32, u32)> = s.samplers.iter().map(|(&k, &v)| (k, v)).collect();
    samplers.sort();
    let list: Vec<String> = samplers
        .iter()
        .filter(|&&(_, t)| t != 0)
        .map(|&(sampler, t)| {
            let face = if s.faces.contains(&t) { " FACE" } else { "" };
            match s.textures.get(&t) {
                Some(&(w, h, f, l)) => format!("s{sampler}={t:#x} {w}x{h} f{f} l{l}{face}"),
                None => format!("s{sampler}={t:#x} ?{face}"),
            }
        })
        .collect();
    let (pixel, vertex) = (s.pixel_shader, s.vertex_shader);
    let key = format!(
        "{vertices} {count} {start_index} {pixel:#x} {vertex:#x} {}",
        list.join(", ")
    );
    let times = {
        let n = s.seen.entry(key).or_insert(0);
        *n += 1;
        *n
    };
    if times > 2 {
        return;
    }
    let states: Vec<String> = FACE_CHECK_STATES
        .iter()
        .map(|&(name, number)| match s.render_states.get(&number) {
            Some(value) => format!("{name}={value:#x}"),
            None => format!("{name}=unset"),
        })
        .collect();
    let determinants = face_check_determinants(&s);
    let pixel_constants = face_check_constant_list(&s.pixel_constants, 32);
    let vertex_constants = face_check_constant_list(&s.vertex_constants, 96);
    drop(s);
    // The file names the textures were made from (`00e68ef0`).
    let names: Vec<String> = {
        let known = super::FACE_CHECK_NAMES
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        samplers
            .iter()
            .filter(|&&(_, t)| t != 0)
            .map(|&(sampler, t)| {
                format!("s{sampler} {}", known.get(&t).map_or("?", String::as_str))
            })
            .collect()
    };
    face_check::line(format_args!(
        "d3d face draw ({label}) {vertices} vertices {count} triangles, primitive {kind} base vertex {base_vertex} min index {min_index} start index {start_index}{}, pixel shader {pixel:#x} vertex shader {vertex:#x}: {} | {}",
        if skipped { " SKIPPED on the real device" } else { "" },
        list.join(", "),
        names.join("; ")
    ));
    face_check::line(format_args!(
        "d3d face states: {} | determinants: {determinants}",
        states.join(" ")
    ));
    let since: Vec<String> = since
        .iter()
        .map(|(state, value)| format!("{state}={value:#x}"))
        .collect();
    face_check::line(format_args!(
        "d3d face states set since the previous draw: {}",
        since.join(" ")
    ));
    face_check::line(format_args!("d3d face pixel constants: {pixel_constants}"));
    face_check::line(format_args!(
        "d3d face vertex constants: {vertex_constants}"
    ));
}

/// Test only: a hidden window of the window thread (64 by 64), for a real device; `None` when it
/// could not be made.
#[cfg(test)]
pub(crate) fn test_window() -> Option<isize> {
    let (reply, answer) = std::sync::mpsc::channel();
    let sent = window_thread()
        .lock()
        .map(|tx| {
            tx.send(Request {
                title: b"nv-rs test".to_vec(),
                style: 0,
                ex_style: 0,
                rect: [0, 0, 64, 64],
                parent: 0,
                reply,
            })
            .is_ok()
        })
        .unwrap_or(false);
    answer.recv().ok().filter(|&w| sent && w != 0)
}

/// Test only: the 64 by 64 windowed `D3DPRESENT_PARAMETERS` words a test device is made with
/// (one back buffer, `D3DSWAPEFFECT_DISCARD`).
#[cfg(test)]
pub(crate) fn test_present() -> [u32; 14] {
    let mut p = [0u32; 14];
    p[0] = 64;
    p[1] = 64;
    p[3] = 1;
    p[6] = 1;
    p[8] = 1;
    p
}

/// Test only: a real device on a hidden window of the window thread (`behavior`: the creation
/// flags, the game's are 0x54, with `D3DCREATE_FPU_PRESERVE` as the backend adds), made the real
/// counterpart of the modelled device `at` through the engine's stream (a direct one, made here
/// when the engine has none); `None` where Direct3D 9 has no device (no display adapter, as on
/// CI).
#[cfg(test)]
pub(crate) fn test_real_device(e: &mut Engine, at: u32, behavior: u32) -> Option<IDirect3DDevice9> {
    test_real_device_on(e, at, behavior, false)
}

/// Test only: [`test_real_device`], the backend on the render thread with `threaded`.
#[cfg(test)]
pub(crate) fn test_real_device_on(
    e: &mut Engine,
    at: u32,
    behavior: u32,
    threaded: bool,
) -> Option<IDirect3DDevice9> {
    let window = test_window()?;
    if real(e).stream.is_none() {
        // With the render thread, the queries' board too, as `direct3d_created` makes it.
        let board = std::sync::Arc::new(super::d3d9backend::QueryBoard::default());
        let made = if threaded {
            real(e).query_board = Some(std::sync::Arc::clone(&board));
            super::d3d9backend::RealBackend::with_query_board(board)
        } else {
            super::d3d9backend::RealBackend::new()
        };
        let backend = std::sync::Arc::new(std::sync::Mutex::new(made));
        let shared = Box::new(SharedBackend(std::sync::Arc::clone(&backend)));
        real(e).stream = Some(if threaded {
            Stream::threaded(shared, d3d9_stream::Settings::default())
        } else {
            Stream::direct(shared)
        });
        real(e).test_backend = Some(backend);
    }
    let present = test_present();
    let reply = call(
        e,
        Cmd::CreateDevice {
            device: at,
            focus: window,
            window,
            behavior,
            present: Data::from_words(&present),
            back_buffers: Data::from_words(&[]),
            depth_stencil: 0,
        },
    );
    let Some(Reply::Device {
        created: true,
        native,
    }) = reply
    else {
        return None;
    };
    let device = native?
        .0
        .downcast::<super::d3d9backend::SharedDevice>()
        .ok()?
        .0;
    real(e).devices.push((at, Some(device.clone())));
    Some(device)
}

/// Test only: `backend` as the engine's forwarding (carried out at once, or on a render thread
/// with `threaded`), with the modelled `devices` and `objects` given counterparts there (the
/// stand-ins of `d3d9backend::stub`).
#[cfg(test)]
pub(crate) fn test_forwarding(
    e: &mut Engine,
    backend: Box<dyn d3d9_stream::Backend>,
    threaded: bool,
    devices: &[u32],
    objects: &[u32],
) {
    let r = real(e);
    r.stream = Some(if threaded {
        Stream::threaded(backend, d3d9_stream::Settings::default())
    } else {
        Stream::direct(backend)
    });
    r.devices = devices.iter().map(|&d| (d, None)).collect();
    r.forwarded = objects.iter().copied().collect();
}

/// Test only: returns once every command recorded is carried out (the render thread's too).
#[cfg(test)]
pub(crate) fn test_finish(e: &mut Engine) {
    if let Some(s) = real(e).stream.as_mut() {
        s.finish();
    }
}

/// Test only: a backend shared with the test, which looks at it afterwards (its stand-ins' log,
/// its real objects).
#[cfg(test)]
pub(crate) struct SharedBackend(
    pub(crate) std::sync::Arc<std::sync::Mutex<super::d3d9backend::RealBackend>>,
);

#[cfg(test)]
impl d3d9_stream::Backend for SharedBackend {
    fn execute(&mut self, cmd: Cmd<Data<'_>>) -> Reply {
        self.0.lock().unwrap().execute(cmd)
    }
}

/// Test only: the real counterpart of the modelled texture or cube texture `texture`, after
/// [`test_real_device`].
#[cfg(test)]
pub(crate) fn real_texture(e: &mut Engine, texture: u32) -> Option<IDirect3DBaseTexture9> {
    test_finish(e);
    let backend = real(e).test_backend.clone()?;
    let b = backend.lock().unwrap();
    b.test_texture(texture)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// An F12 report's `log-start.txt`: the log's first bytes, cut back to a whole line.
    #[test]
    fn the_logs_start_ends_at_a_whole_line() {
        let path = std::env::temp_dir().join(format!("nv-head-{}.txt", std::process::id()));
        std::fs::write(
            &path,
            "Preset fixes: x; threads real\nthreads: real\nlater\n",
        )
        .unwrap();
        let start = shot::head(&path, 40).unwrap();
        assert_eq!(start, b"Preset fixes: x; threads real\n");
        let all = shot::head(&path, 4096).unwrap();
        assert!(all.ends_with(b"later\n"));
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn shot_settings_and_bmp() {
        let s = shot::parse("C:\\shots,2500").unwrap();
        assert_eq!(s.dir, std::path::PathBuf::from("C:\\shots"));
        assert_eq!(s.every, std::time::Duration::from_millis(2500));
        let s = shot::parse("C:\\shots").unwrap();
        assert_eq!(s.every, std::time::Duration::from_millis(5000));
        let s = shot::parse("D:\\a,b").unwrap();
        assert_eq!(s.dir, std::path::PathBuf::from("D:\\a,b"));
        assert!(shot::parse(" ").is_none());
        assert_eq!(shot::parse_frames("17100-17500"), Some((17100, 17500)));
        assert_eq!(shot::parse_frames(" 5 - 9 "), Some((5, 9)));
        assert!(shot::parse_frames("17100").is_none());
        let file = shot::bmp(2, 1, &[1, 2, 3, 0, 4, 5, 6, 0]);
        assert_eq!(&file[0..2], b"BM");
        assert_eq!(file.len(), 54 + 8);
        assert_eq!(i32::from_le_bytes(file[22..26].try_into().unwrap()), -1);
        assert_eq!(u16::from_le_bytes(file[28..30].try_into().unwrap()), 32);
        assert_eq!(&file[54..], &[1, 2, 3, 0, 4, 5, 6, 0]);
    }

    #[test]
    fn face_check_helpers() {
        assert!(face_check::compressed(u32::from_le_bytes(*b"DXT1")));
        assert!(face_check::compressed(u32::from_le_bytes(*b"DXT5")));
        assert!(!face_check::compressed(21));
        // Only an uncompressed texture of the face image's size, once an image was made.
        let image = (256u64 << 32) | 256;
        assert!(face_check::is_face(image, 256, 256, 21));
        assert!(!face_check::is_face(0, 256, 256, 21));
        assert!(!face_check::is_face(image, 512, 512, 21));
        assert!(!face_check::is_face(
            image,
            256,
            256,
            u32::from_le_bytes(*b"DXT5")
        ));
        assert_eq!(face_check::means(&[1, 2, 3, 4, 3, 4, 5, 6]), [2, 3, 4, 5]);
        let file = face_check::tga(1, 1, &[9, 8, 7, 6]);
        assert_eq!(file.len(), 22);
        assert_eq!((file[2], file[12], file[14], file[16]), (2, 1, 1, 32));
        assert_eq!(&file[18..], &[9, 8, 7, 6]);
        let mut map = HashMap::new();
        map.insert(3u32, [1f32.to_bits(), 0, 0, 0]);
        map.insert(1u32, [0, 2f32.to_bits(), 0, 0]);
        map.insert(40u32, [0; 4]);
        let one = 1f32.to_bits();
        let minus = (-1f32).to_bits();
        assert_eq!(
            face_check_det3([&[one, 0, 0], &[0, one, 0], &[0, 0, one]]),
            1.0
        );
        assert_eq!(
            face_check_det3([&[minus, 0, 0], &[0, one, 0], &[0, 0, one]]),
            -1.0
        );
        assert_eq!(
            face_check_constant_list(&map, 32),
            "c1=(0.000,2.000,0.000,0.000) c3=(1.000,0.000,0.000,0.000)"
        );
    }

    /// A key message's `lParam` with this scan code (extended or not).
    fn key(scan: isize, extended: bool) -> isize {
        (scan << 16) | if extended { 1 << 24 } else { 0 }
    }

    #[test]
    fn key_messages_give_directinput_key_codes() {
        // W, Escape, left Ctrl and Alt: the scan code.
        assert_eq!(dik(key(0x11, false)), Some(0x11));
        assert_eq!(dik(key(0x01, false)), Some(0x01));
        assert_eq!(dik(key(0x1d, false)), Some(0x1d));
        assert_eq!(dik(key(0x38, false)), Some(0x38));
        // Right Ctrl and Alt, the arrows, numpad Enter: extended, 0x80 added.
        assert_eq!(dik(key(0x1d, true)), Some(0x9d));
        assert_eq!(dik(key(0x38, true)), Some(0xb8));
        assert_eq!(dik(key(0x48, true)), Some(0xc8));
        assert_eq!(dik(key(0x50, true)), Some(0xd0));
        assert_eq!(dik(key(0x1c, true)), Some(0x9c));
        // The numpad's own keys are not extended (DIK_NUMPAD8 0x48).
        assert_eq!(dik(key(0x48, false)), Some(0x48));
        // NumLock (extended 0x45) and Pause (plain 0x45).
        assert_eq!(dik(key(0x45, true)), Some(0x45));
        assert_eq!(dik(key(0x45, false)), Some(0xc5));
        // The fake shifts around extended keys are dropped.
        assert_eq!(dik(key(0x2a, true)), None);
        assert_eq!(dik(key(0, false)), None);
    }

    /// With the render thread, a query read without `D3DGETDATA_FLUSH` before the real device has
    /// the game's `END` answers `S_FALSE` (the result is not in) without waiting; one the real
    /// device could not make answers as before (no real query); once the `END` is issued, the
    /// real query answers.
    #[test]
    fn query_results_not_in_yet_answer_not_ready_without_waiting() {
        let mut e = Engine::new();
        let board = std::sync::Arc::new(super::super::d3d9backend::QueryBoard::default());
        {
            let r = real(&mut e);
            r.stream = Some(Stream::direct(Box::new(
                super::super::d3d9backend::RealBackend::new(),
            )));
            r.query_board = Some(std::sync::Arc::clone(&board));
            r.forwarded.extend([0x10, 0x20]);
        }
        query_issue(&mut e, 0x10, 2);
        query_issue(&mut e, 0x10, ISSUE_END);
        assert_eq!(real(&mut e).query_ends.get(&0x10), Some(&1));
        // Not made on the real device yet.
        assert_eq!(query_get_data(&mut e, 0x10, 0), Some((S_FALSE, 0)));
        // A reader that waits (D3DGETDATA_FLUSH) asks the backend after every earlier command
        // instead: here it has no such query.
        assert_eq!(query_get_data(&mut e, 0x10, GETDATA_FLUSH), None);
        // Its creation failed there.
        let failed = || super::super::d3d9backend::BoardQuery {
            made: 1,
            query: None,
            ends: 0,
        };
        real(&mut e).queries_made.insert(0x20, 1);
        board.lock().insert(0x20, failed());
        query_issue(&mut e, 0x20, ISSUE_END);
        assert_eq!(query_get_data(&mut e, 0x20, 0), None);
        // Destroyed and made again at the same address before the backend got there: the board
        // still holds the old one, which is not this one's answer.
        forget(&mut e, 0x20);
        real(&mut e).forwarded.insert(0x20);
        real(&mut e).queries_made.insert(0x20, 2);
        query_issue(&mut e, 0x20, ISSUE_END);
        assert_eq!(query_get_data(&mut e, 0x20, 0), Some((S_FALSE, 0)));
        // Not a forwarded query: none, as before.
        assert_eq!(query_get_data(&mut e, 0x30, 0), None);
        forget(&mut e, 0x10);
        assert_eq!(real(&mut e).query_ends.get(&0x10), None);
    }

    /// The same on a real device with the render thread: the occlusion query's result comes
    /// (`S_FALSE` until then, never a wait), and a reader with `D3DGETDATA_FLUSH` gets the real
    /// answer after waiting. Skipped where Direct3D 9 has no device.
    #[test]
    fn a_real_query_answers_without_waiting_for_the_render_thread() {
        let mut e = Engine::new();
        let device = 0x1000;
        if test_real_device_on(&mut e, device, 0x54, true).is_none() {
            return;
        }
        let query = 0x2000;
        query_created(&mut e, device, query, 9);
        for _ in 0..3 {
            query_issue(&mut e, query, 2);
            query_issue(&mut e, query, ISSUE_END);
            // At once: not in yet, or the real answer (nothing was drawn: no pixels passed).
            let at_once = query_get_data(&mut e, query, 0);
            assert!(
                matches!(at_once, Some((S_FALSE, 0)) | Some((0, 0))),
                "{at_once:?}"
            );
            // A reader that waits (D3DGETDATA_FLUSH, as the game's loops read): the real answer
            // after every earlier command, until the GPU is done.
            let mut answer = None;
            for _ in 0..2000 {
                answer = query_get_data(&mut e, query, GETDATA_FLUSH);
                if answer != Some((S_FALSE, 0)) {
                    break;
                }
                std::thread::sleep(std::time::Duration::from_millis(1));
            }
            assert_eq!(answer, Some((0, 0)));
            // Then the result reads at once.
            assert_eq!(query_get_data(&mut e, query, 0), Some((0, 0)));
        }
        assert!(state!(e).graphics.real.logged_is_empty());
    }

    /// A level's rows read as if packed, from rows lying `pitch` apart.
    #[test]
    fn rows_are_read_packed_whatever_their_pitch() {
        let mut e = Engine::new();
        let bits = e.mem.alloc(64);
        for i in 0..64u32 {
            e.mem.set_u8(bits + i, i as u8);
        }
        // Three rows of 5 bytes, 16 apart.
        let mut out = [0u8; 15];
        read_rows(&e.mem, bits, 16, 5, 0, &mut out);
        assert_eq!(out, [0, 1, 2, 3, 4, 16, 17, 18, 19, 20, 32, 33, 34, 35, 36]);
        let mut part = [0u8; 4];
        read_rows(&e.mem, bits, 16, 5, 3, &mut part);
        assert_eq!(part, [3, 4, 16, 17]);
        let mut packed = [0u8; 6];
        read_rows(&e.mem, bits, 5, 5, 4, &mut packed);
        assert_eq!(packed, [4, 5, 6, 7, 8, 9]);
    }

    /// Not a check: the host heap allocations of the forwarded per-frame calls on a real device,
    /// recorded and carried out at once (`cargo test --release -p engine-platform --lib
    /// allocations_per -- --ignored --nocapture`); nothing is listed where Direct3D 9 has no
    /// device.
    #[test]
    #[ignore]
    fn allocations_per_operation_of_the_real_device() {
        use crate::alloc_count::{per_run, row};
        let mut e = Engine::new();
        let device = 0x1000;
        if test_real_device(&mut e, device, 0x54).is_none() {
            return;
        }
        let matrix = e.mem.alloc(64);
        for i in 0..4 {
            e.mem.set_u32(matrix + 20 * i, 1f32.to_bits());
        }
        let viewport = e.mem.alloc(24);
        for (i, w) in [0u32, 0, 64, 64, 0, 1f32.to_bits()].iter().enumerate() {
            e.mem.set_u32(viewport + 4 * i as u32, *w);
        }
        let rects = e.mem.alloc(32);
        for (i, w) in [0u32, 0, 32, 32, 32, 32, 64, 64].iter().enumerate() {
            e.mem.set_u32(rects + 4 * i as u32, *w);
        }
        let block = e.mem.alloc(0x1000);
        let buffer = 0x2000;
        // D3DUSAGE_DYNAMIC | D3DUSAGE_WRITEONLY, default pool, 0x900 bytes, FVF XYZ|DIFFUSE.
        buffer_created(&mut e, device, buffer, false, 0x900, 0x208, 0x42, 0);
        let mut rows: Vec<(&str, crate::alloc_count::Per)> = Vec::new();
        rows.push((
            "SetTransform",
            per_run(1000, || set_transform(&mut e, device, 256, matrix, 0)),
        ));
        rows.push((
            "SetViewport",
            per_run(1000, || set_viewport(&mut e, device, viewport, 0)),
        ));
        rows.push((
            "Clear, no rectangles",
            per_run(1000, || clear(&mut e, device, 0, 0, 1, 0, 0, 0, 0)),
        ));
        rows.push((
            "Clear, two rectangles",
            per_run(1000, || clear(&mut e, device, 2, rects, 1, 0, 0, 0, 0)),
        ));
        rows.push((
            "SetClipPlane",
            per_run(1000, || set_clip_plane(&mut e, device, 0, block, 0)),
        ));
        rows.push((
            "SetMaterial",
            per_run(1000, || set_material(&mut e, device, block, 0)),
        ));
        rows.push((
            "SetRenderState",
            per_run(1000, || set_render_state(&mut e, device, 7, 1, 0)),
        ));
        rows.push((
            "SetStreamSource",
            per_run(1000, || {
                set_stream_source(&mut e, device, 0, buffer, 0, 16, 0)
            }),
        ));
        rows.push((
            "buffer Lock + Unlock, 0x100 bytes",
            per_run(1000, || {
                buffer_locked(&mut e, buffer, 0, 0x100, 0x2000, 0);
                buffer_unlocked(&mut e, buffer, block, 0x900, 0);
            }),
        ));
        rows.push((
            "DrawPrimitiveUP, 2 triangles of 16-byte vertices",
            per_run(1000, || {
                begin_scene(&mut e, device, 0);
                draw_primitive_up(&mut e, device, 4, 2, block, 16, 6, 0);
                end_scene(&mut e, device, 0);
            }),
        ));
        for (what, per) in rows {
            row(&format!("real IDirect3DDevice9 {what}"), per);
        }
        // SetLight needs a light the runtime takes: a directional one.
        e.mem.set_u32(block, 3);
        e.mem.set_u32(block + 0x40, 1f32.to_bits());
        row(
            "real IDirect3DDevice9 SetLight",
            per_run(1000, || set_light(&mut e, device, 0, block, 0)),
        );
    }
}

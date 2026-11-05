//! `--background` test runs stay out of the user's way: the window opens
//! behind the others without the focus and never holds the mouse
//! (`main.rs`). A thread hook refuses every activation of the viewer's
//! window that is not a click ([`keep_inactive`]). If the window still takes
//! the focus, the focus goes straight back to the window that had it before
//! this one opened.

use bevy::prelude::*;
use bevy::window::WindowFocused;

/// The window in front when the viewer started (Windows' window handle).
#[derive(Resource, Clone, Copy)]
pub struct UserWindow(pub isize);

#[cfg(windows)]
mod win {
    /// `WH_CBT`: the hook that sees a window's activation before it happens.
    pub const WH_CBT: i32 = 5;
    /// `HCBT_ACTIVATE`: a window is about to be activated.
    pub const HCBT_ACTIVATE: i32 = 5;

    /// `CBTACTIVATESTRUCT`.
    #[repr(C)]
    pub struct CbtActivate {
        /// Nonzero when a mouse click activates the window.
        pub mouse: i32,
        /// The window active now (not read).
        pub _active: isize,
    }

    /// `WH_CALLWNDPROC`: the hook that sees a message before the window does.
    pub const WH_CALLWNDPROC: i32 = 4;
    /// `WM_SHOWWINDOW`: a window is about to be shown or hidden.
    pub const WM_SHOWWINDOW: u32 = 0x0018;
    pub const GWL_STYLE: i32 = -16;
    pub const GWL_EXSTYLE: i32 = -20;
    pub const WS_CHILD: i32 = 0x4000_0000;
    pub const WS_EX_NOACTIVATE: i32 = 0x0800_0000;

    /// `CWPSTRUCT`.
    #[repr(C)]
    pub struct CallWndProc {
        pub _lparam: isize,
        /// For `WM_SHOWWINDOW`: nonzero when the window is shown.
        pub wparam: usize,
        pub message: u32,
        pub window: isize,
    }

    pub type HookProc = unsafe extern "system" fn(i32, usize, isize) -> isize;

    #[link(name = "user32")]
    extern "system" {
        pub fn GetForegroundWindow() -> isize;
        pub fn SetForegroundWindow(window: isize) -> i32;
        pub fn SetWindowsHookExW(id: i32, hook: HookProc, module: isize, thread: u32) -> isize;
        pub fn CallNextHookEx(hook: isize, code: i32, wparam: usize, lparam: isize) -> isize;
        pub fn GetWindowLongW(window: isize, index: i32) -> i32;
        pub fn SetWindowLongW(window: isize, index: i32, value: i32) -> i32;
    }

    #[link(name = "kernel32")]
    extern "system" {
        pub fn GetCurrentThreadId() -> u32;
    }
}

/// `--background` on Windows: the windows of the calling thread are not
/// activated by the program or by Windows, only by a click of the user. Call it
/// on the thread that makes the viewer's window (the main thread, winit's
/// event loop), before the app runs. Without it, Bevy's window took the focus
/// when it opened: bevy_winit 0.16 does not give `Window::focused` to winit's
/// `with_active`, so winit shows the window with `SW_SHOW`. The game's window
/// has its own thread and `NV_D3D9_BACKGROUND`.
pub fn keep_inactive(user_window: isize) {
    #[cfg(windows)]
    {
        USER_WINDOW.store(user_window, std::sync::atomic::Ordering::Relaxed);
        // SAFETY: a hook procedure for this thread only (no module), as the
        // Win32 documentation of `SetWindowsHookExW` allows. It reads the
        // `CBTACTIVATESTRUCT` that Windows passes with `HCBT_ACTIVATE`.
        let hook = unsafe {
            win::SetWindowsHookExW(win::WH_CBT, refuse_activation, 0, win::GetCurrentThreadId())
        };
        // SAFETY: as above. It changes only the extended style of this
        // thread's top-level windows when they are shown.
        let show = unsafe {
            win::SetWindowsHookExW(
                win::WH_CALLWNDPROC,
                show_inactive,
                0,
                win::GetCurrentThreadId(),
            )
        };
        if hook == 0 || show == 0 {
            println!("--background: no activation hook; a window may take the focus.");
        }
    }
    #[cfg(not(windows))]
    let _ = user_window;
}

/// The message hook of [`keep_inactive`]: a top-level window of this thread
/// that is about to be shown gets `WS_EX_NOACTIVATE`, so that the show does
/// not activate it. winit shows its window with `SW_SHOW` (see
/// [`keep_inactive`]) and then writes the window's extended style again, which
/// removes the style: a click then activates the window as usual.
#[cfg(windows)]
unsafe extern "system" fn show_inactive(code: i32, wparam: usize, lparam: isize) -> isize {
    let message = lparam as *const win::CallWndProc;
    // SAFETY: with WH_CALLWNDPROC and code 0 (HC_ACTION), lparam points to a CWPSTRUCT.
    if code == 0 && !message.is_null() {
        let m = unsafe { &*message };
        if m.message == win::WM_SHOWWINDOW && m.wparam != 0 {
            // SAFETY: the window the message is for.
            unsafe {
                if win::GetWindowLongW(m.window, win::GWL_STYLE) & win::WS_CHILD == 0 {
                    let ex = win::GetWindowLongW(m.window, win::GWL_EXSTYLE);
                    win::SetWindowLongW(m.window, win::GWL_EXSTYLE, ex | win::WS_EX_NOACTIVATE);
                }
            }
        }
    }
    // SAFETY: passes the hook's arguments on unchanged.
    unsafe { win::CallNextHookEx(0, code, wparam, lparam) }
}

/// The window that had the focus when the viewer started ([`keep_inactive`]).
#[cfg(windows)]
static USER_WINDOW: std::sync::atomic::AtomicIsize = std::sync::atomic::AtomicIsize::new(0);

/// The CBT hook of [`keep_inactive`]: an activation that is not a click is
/// refused. Windows takes the focus from the user's window before it asks to
/// activate this one. With the activation refused, no window has the focus
/// (measured: 5 s with no foreground window). So the focus goes back to the
/// user's window at once, when no other window took it meanwhile.
#[cfg(windows)]
unsafe extern "system" fn refuse_activation(code: i32, wparam: usize, lparam: isize) -> isize {
    if code == win::HCBT_ACTIVATE {
        let info = lparam as *const win::CbtActivate;
        // SAFETY: with HCBT_ACTIVATE, lparam points to a CBTACTIVATESTRUCT.
        if info.is_null() || unsafe { (*info).mouse } == 0 {
            let user = USER_WINDOW.load(std::sync::atomic::Ordering::Relaxed);
            if user != 0 {
                // after the refused activation ends (the hook runs inside it)
                std::thread::spawn(move || {
                    std::thread::sleep(std::time::Duration::from_millis(50));
                    // SAFETY: a window handle from Windows; a stale one makes the call fail.
                    unsafe {
                        if win::GetForegroundWindow() == 0 {
                            win::SetForegroundWindow(user);
                        }
                    }
                });
            }
            return 1;
        }
    }
    // SAFETY: passes the hook's arguments on unchanged.
    unsafe { win::CallNextHookEx(0, code, wparam, lparam) }
}

/// The window in front now (0 off Windows).
pub fn foreground() -> isize {
    #[cfg(windows)]
    {
        // SAFETY: no arguments; returns a handle or 0.
        unsafe { win::GetForegroundWindow() }
    }
    #[cfg(not(windows))]
    {
        0
    }
}

/// Gives the focus back when the window takes it.
pub fn give_focus_back(mut focused: EventReader<WindowFocused>, user: Option<Res<UserWindow>>) {
    let Some(user) = user else {
        focused.clear();
        return;
    };
    for event in focused.read() {
        if event.focused && user.0 != 0 {
            #[cfg(windows)]
            // SAFETY: a window handle taken from Windows; a stale one only
            // makes the call fail.
            unsafe {
                win::SetForegroundWindow(user.0);
            }
            println!("--background: the window took the focus; handed back.");
        }
    }
}

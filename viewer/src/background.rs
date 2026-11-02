//! `--background` test runs stay out of the user's way: the window opens
//! behind the others without the focus and never holds the mouse
//! (`main.rs`). Windows still hands the focus to a new window when the
//! user hasn't typed for a while; when that happens the focus goes straight
//! back to the window that had it before this one opened.

use bevy::prelude::*;
use bevy::window::WindowFocused;

/// The window in front when the viewer started (Windows' window handle).
#[derive(Resource, Clone, Copy)]
pub struct UserWindow(pub isize);

#[cfg(windows)]
mod win {
    #[link(name = "user32")]
    extern "system" {
        pub fn GetForegroundWindow() -> isize;
        pub fn SetForegroundWindow(window: isize) -> i32;
    }
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

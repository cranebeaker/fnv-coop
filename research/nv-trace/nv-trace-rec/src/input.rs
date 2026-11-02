//! The key script: keys pressed or released at the start of counted frames (entries of the
//! list's `frame` function), fed into the game's own DirectInput keyboard ([`crate::dinput`]);
//! no system-wide input is synthesised. The keyboard device reads only while the game's
//! window is in front, so the window is brought there before each key.
//!
//! The port's frame numbers (`NV_MOVE_KEYS`) count frames of a run at about 58 frames a
//! second; the real game may run much faster (no frame cap in the player's settings), so a
//! frame number means an earlier moment there. With `keys_fps=N` the script's frame numbers
//! are read as times instead (frame / N seconds after the first counted frame), and each key
//! change is sent at the first frame at or after its time.

use crate::sys;
use nv_trace_core::keys::{self, Press};
use std::sync::atomic::{AtomicBool, Ordering::Relaxed};
use std::sync::{Mutex, OnceLock};

struct Script {
    presses: Vec<Press>,
    /// 0: by frame number; else frames per second of the port's run.
    fps: u32,
    /// Time mode: the key changes as (ms after frame 0, dik, down), sorted, and how many
    /// were sent.
    timed: Vec<(u64, u8, bool)>,
    next: usize,
    start_tick: u32,
    /// The frame the last key change went out.
    last_frame: u64,
    /// Frames at which to take a screenshot (after each key press).
    shots: Vec<u64>,
    /// The clock starts at the first call of this function (time mode); `None`: at frame 0.
    after: Option<u32>,
    /// Key changes still to send.
    active: bool,
}

/// How the key script is timed.
#[derive(Clone, Copy, Default)]
pub struct KeyTiming {
    /// 0: by frame number; else the port's frames per second (frame numbers become times).
    pub fps: u32,
    /// Added to every time.
    pub delay_ms: u64,
    /// Start the clock at the first call of this function instead of frame 0, and count the
    /// script's frames from this port frame (the port's frame of the same event).
    pub after: Option<(u32, u64)>,
    /// A screenshot this often (0: none).
    pub shot_every_ms: u32,
}

static SHOT_EVERY_MS: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
static LAST_SHOT: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
/// Functions whose first call starts a script's clock (0: none), by script slot.
pub static KEYS_AFTER: [std::sync::atomic::AtomicU32; SLOTS] = [
    std::sync::atomic::AtomicU32::new(0),
    std::sync::atomic::AtomicU32::new(0),
];

/// Two key scripts can run (`keys` and `keys2`, e.g. the menu from the start and the world
/// from its first frame), each on its own clock.
pub const SLOTS: usize = 2;
static SCRIPTS: [OnceLock<Mutex<Script>>; SLOTS] = [OnceLock::new(), OnceLock::new()];
/// Some script still has key changes to send (the mouse is muted meanwhile, [`crate::dinput`]).
static SCRIPTED: AtomicBool = AtomicBool::new(false);

/// Whether a key script is still running.
pub fn scripted() -> bool {
    SCRIPTED.load(Relaxed)
}

/// Keep the mouse muted for the whole run, not only while a script runs (`mouse_mute=always`).
pub static MUTE_ALWAYS: AtomicBool = AtomicBool::new(false);

/// The real keyboard reads nothing for the whole run, only the scripts' keys
/// (`keyboard_mute=always`): nobody at the machine can change a scripted run.
pub static KEYBOARD_MUTE: AtomicBool = AtomicBool::new(false);

/// Whether the game's DirectInput mouse reads nothing now.
pub fn mouse_muted() -> bool {
    scripted() || MUTE_ALWAYS.load(Relaxed)
}

fn update_scripted() {
    let any = SCRIPTS.iter().any(|s| {
        s.get()
            .is_some_and(|m| m.try_lock().map_or(true, |s| s.active))
    });
    SCRIPTED.store(any, Relaxed);
}

pub fn set_script(slot: usize, presses: Vec<Press>, timing: KeyTiming) {
    let KeyTiming {
        fps,
        delay_ms,
        after,
        shot_every_ms,
    } = timing;
    if shot_every_ms != 0 {
        SHOT_EVERY_MS.store(shot_every_ms, Relaxed);
    }
    KEYS_AFTER[slot].store(after.map_or(0, |a| a.0), Relaxed);
    let base = after.map_or(0, |a| a.1);
    if !presses.is_empty() {
        SCRIPTED.store(true, Relaxed);
    }
    let active = !presses.is_empty();
    let mut timed = Vec::new();
    if fps > 0 {
        let ms = |f: u64| f.saturating_sub(base) * 1000 / u64::from(fps);
        for p in &presses {
            timed.push((delay_ms + ms(p.frame), p.dik, true));
            timed.push((delay_ms + ms(p.frame + p.hold.max(1)), p.dik, false));
        }
        // Releases before presses at the same time, as `keys::changes_at` orders them.
        timed.sort_by_key(|&(t, _, down)| (t, down));
    }
    let _ = SCRIPTS[slot].set(Mutex::new(Script {
        active,
        presses,
        fps,
        timed,
        next: 0,
        start_tick: 0,
        last_frame: u64::MAX,
        shots: Vec::new(),
        after: after.map(|a| a.0),
    }));
}

/// A call of the function the clock waits for ([`KeyTiming::after`]): the first starts it.
pub fn event(addr: u32) {
    for script in SCRIPTS.iter().filter_map(|s| s.get()) {
        let Ok(mut s) = script.lock() else { continue };
        if s.after != Some(addr) {
            continue;
        }
        // SAFETY: plain query.
        s.start_tick = unsafe { nv_win::GetTickCount() }.max(1);
        s.after = None;
        crate::log(&format!(
            "keys: clock started at the first call of {addr:08x}"
        ));
    }
}

fn ours_in_front() -> bool {
    // SAFETY: plain queries.
    unsafe {
        let w = sys::GetForegroundWindow();
        if w.is_null() {
            return false;
        }
        let mut pid = 0u32;
        sys::GetWindowThreadProcessId(w, &mut pid);
        pid == nv_win::GetCurrentProcessId()
    }
}

/// The game's own visible top-level window (null if none).
fn game_window() -> sys::Handle {
    unsafe extern "system" fn each(w: sys::Handle, out: isize) -> i32 {
        let mut pid = 0u32;
        sys::GetWindowThreadProcessId(w, &mut pid);
        if pid == nv_win::GetCurrentProcessId() && sys::IsWindowVisible(w) != 0 {
            *(out as *mut sys::Handle) = w;
            return 0;
        }
        1
    }
    let mut found: sys::Handle = std::ptr::null_mut();
    // SAFETY: the callback writes one handle into `found`, which outlives the call.
    unsafe { sys::EnumWindows(each, &mut found as *mut sys::Handle as isize) };
    found
}

/// The game's window brought to the front. Returns whether it is in front afterwards.
fn bring_to_front() -> bool {
    let found = game_window();
    if found.is_null() {
        return false;
    }
    // SAFETY: plain window calls on the game's own window.
    unsafe {
        sys::ShowWindow(found, 9);
        sys::BringWindowToTop(found);
        sys::SetForegroundWindow(found);
    }
    ours_in_front()
}

fn send(frame: u64, dik: u8, down: bool) {
    let updown = if down { "down" } else { "up" };
    // The keyboard device is foreground-only: it reads nothing while another window is in
    // front, so the game's window is brought there first.
    if !ours_in_front() {
        let brought = bring_to_front();
        crate::log(&format!(
            "keys: frame {frame}: the game window was not in front; brought it there: {brought}"
        ));
    }
    let device = crate::dinput::key(dik, down);
    crate::log(&format!(
        "keys: frame {frame}: key {dik:02x} {updown} queued (keyboard device seen: {device})"
    ));
}

/// A keyboard read of the game (`GetDeviceState`): in time mode the script also advances
/// here, because some loops (the intro movie) read the keyboard without running the frame
/// function.
pub fn on_poll() {
    if !scripted() {
        return;
    }
    let frame = crate::hooks::FRAMES.load(Relaxed);
    tick(frame, false);
}

pub fn on_frame(frame: u64) {
    periodic_shot(frame);
    tick(frame, true);
}

/// A screenshot every [`KeyTiming::shot_every_ms`] (`shots/t-NNNN.bmp`, seconds since the
/// first frame).
fn periodic_shot(frame: u64) {
    let every = SHOT_EVERY_MS.load(Relaxed);
    if every == 0 {
        return;
    }
    // SAFETY: plain query.
    let now = unsafe { nv_win::GetTickCount() };
    static FIRST: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
    if frame == 0 {
        FIRST.store(now, Relaxed);
        LAST_SHOT.store(now, Relaxed);
        return;
    }
    if now.wrapping_sub(LAST_SHOT.load(Relaxed)) >= every {
        let secs = now.wrapping_sub(FIRST.load(Relaxed)) / 1000;
        if crate::shot::take(game_window(), &format!("t-{secs:04}")) {
            LAST_SHOT.store(now, Relaxed);
            crate::log(&format!("shot: {secs} s (frame {frame})"));
        } else {
            // No frame this time (an empty PrintWindow): again in 200 ms.
            LAST_SHOT.store(now.wrapping_sub(every.saturating_sub(200)), Relaxed);
        }
    }
}

fn tick(frame: u64, from_frame: bool) {
    for script in SCRIPTS.iter().filter_map(|s| s.get()) {
        tick_one(script, frame, from_frame);
    }
    update_scripted();
}

fn tick_one(script: &Mutex<Script>, frame: u64, from_frame: bool) {
    let Ok(mut s) = (if from_frame {
        script.lock().map_err(|_| ())
    } else {
        script.try_lock().map_err(|_| ())
    }) else {
        return;
    };
    if s.presses.is_empty() || (!from_frame && (s.fps == 0 || s.start_tick == 0)) {
        return;
    }
    if from_frame && frame % 600 == 0 {
        crate::log(&format!("frame {frame}"));
    }
    if let Some(i) = s.shots.iter().position(|&f| from_frame && f <= frame) {
        s.shots.remove(i);
        let ok = crate::shot::take(game_window(), &format!("frame-{frame:06}"));
        crate::log(&format!("shot: frame {frame}: {ok}"));
    }
    if s.fps == 0 {
        for (dik, down) in keys::changes_at(&s.presses, frame) {
            send(frame, dik, down);
        }
        if s.presses.iter().all(|p| p.frame + p.hold < frame) {
            s.active = false;
        }
        return;
    }
    // SAFETY: plain query.
    let now = unsafe { nv_win::GetTickCount() };
    if frame == 0 && from_frame && s.after.is_none() && s.start_tick == 0 {
        s.start_tick = now.max(1);
    }
    if s.start_tick == 0 {
        // Waiting for the event that starts the clock.
        return;
    }
    let elapsed = u64::from(now.wrapping_sub(s.start_tick));
    while let Some(&(at, dik, down)) = s.timed.get(s.next) {
        if at > elapsed {
            break;
        }
        // A press and its release never go out in the same frame: the release waits a frame.
        let pressed_now = s.next > 0
            && s.last_frame == frame
            && s.timed[s.next - 1].1 == dik
            && s.timed[s.next - 1].2;
        if !down && pressed_now {
            break;
        }
        if s.next == 0 {
            let ok = crate::shot::take(game_window(), &format!("frame-{frame:06}-before"));
            crate::log(&format!("shot: frame {frame} before the first key: {ok}"));
        }
        send(frame, dik, down);
        if down {
            // Where the highlight went: a few real frames after the press.
            s.shots.push(frame + 20);
        }
        s.last_frame = frame;
        s.next += 1;
    }
    if s.next >= s.timed.len() {
        s.active = false;
    }
}

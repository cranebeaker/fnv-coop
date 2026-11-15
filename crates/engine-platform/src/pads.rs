//! The game's XInput controller from the PC's real controllers (docs/coop/SPECS.md S39,
//! "controllers").
//!
//! What the game calls (FalloutNV.exe 1.4.0.525, from this repository's translations):
//! `Controls::Poll` (`00a23010`, through the thunk `009f996e`) asks `XInputGetState(0, 011f35a8)`
//! every frame and counts the pad as connected when that answers 0; the rumble (`00a255b0`,
//! through `00ee820a`) sends `XInputSetState(0, vibration)`. Both are imports of `XINPUT1_3.dll`
//! by ordinal (2 and 3), answered in `engine-core`'s `units/platform.rs`, which asks this module
//! through its `Pads` hook ([`HOOK`]). Ordinal 4, `XInputGetCapabilities`, is answered too
//! ([`get_capabilities`]), although none of the repository's translations calls it. The game only
//! ever names user index 0.
//!
//! The answers come from the system's XInput, loaded in this (64-bit) process: `xinput1_4.dll`,
//! else `xinput1_3.dll`, else `xinput9_1_0.dll`. The structures are the same bytes in a 32-bit
//! and a 64-bit process (Microsoft's `XINPUT_STATE`, 16 bytes: the packet number, then
//! `XINPUT_GAMEPAD`, the buttons word, the two trigger bytes and the four thumb-stick words;
//! `XINPUT_VIBRATION`, 4 bytes: the two motor speeds; `XINPUT_CAPABILITIES`, 20 bytes: the type
//! and subtype bytes, the flags word, an `XINPUT_GAMEPAD` and an `XINPUT_VIBRATION`), so they are
//! copied as they are between the game's memory and the system's calls.
//!
//! Which controller the game's user 0 is: the setting `NV_PAD` ([`Seat::parse`]):
//! - unset: the first controller that answers (0 to 3), read only while one of this process's
//!   windows is in front, so a second copy of the game or an alt-tabbed game does not move with
//!   the player's pad (the game itself would pause there, but this engine's game never hears
//!   that its window went to the back: `window.rs`);
//! - `0` to `3`: that controller, also while another window is in front (a Couchlink seat:
//!   `NV_PAD=$SEAT`);
//! - `any`: the first controller that answers, also in the background;
//! - `none`: no controller.
//!
//! The game's controller is "not connected" (`ERROR_DEVICE_NOT_CONNECTED`, the state left as it
//! is) when the setting finds none, when no XInput DLL loads, in a repeatable run
//! (`NV_REPEATABLE`, so two runs stay equal), and in a scripted run (`NV_MOVE_KEYS` or
//! `NV_MOVE_MOUSE`) unless `NV_MOVE_REAL_INPUT=1`, as for the real keyboard and mouse (the host's
//! `real_input_reaches_game`). A controller that does not answer is asked again only after
//! [`RETRY_CALLS`] more of the game's calls: asking for a missing controller is slow in XInput.
//! When the controller stops being read because the game's window went to the back, its motors
//! are stopped.
//!
//! Tests use a fake XInput ([`use_xinput`]); the system's is never loaded by a test.

use crate::{Engine, Ptr};

/// `ERROR_SUCCESS` and `ERROR_DEVICE_NOT_CONNECTED` (winerror.h).
pub const ERROR_SUCCESS: u32 = 0;
pub const ERROR_DEVICE_NOT_CONNECTED: u32 = 0x48f;
/// The sizes of `XINPUT_STATE`, `XINPUT_VIBRATION` and `XINPUT_CAPABILITIES` (xinput.h).
pub const STATE_SIZE: usize = 16;
pub const VIBRATION_SIZE: usize = 4;
pub const CAPABILITIES_SIZE: usize = 20;
/// XInput's controllers: user indices 0 to 3 (`XUSER_MAX_COUNT`).
pub const MAX_PADS: u32 = 4;
/// How many of the game's calls go by before a controller that did not answer is asked again
/// (about a second at 60 frames a second).
pub const RETRY_CALLS: u32 = 60;

/// Which controller this copy of the game reads (`NV_PAD`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Seat {
    /// The first controller that answers; `background`: also while another program's window is
    /// in front (`NV_PAD=any`), else only while this process's is (`NV_PAD` unset).
    Any { background: bool },
    /// That controller (0 to 3), also in the background (`NV_PAD=N`).
    Pad(u32),
    /// No controller (`NV_PAD=none`).
    None,
}

impl Seat {
    /// `NV_PAD`'s value (`None` when unset): `0` to `3`, `any` or `none` (any case, spaces
    /// around ignored); unset or empty is the default. Another value is an error that names it.
    pub fn parse(value: Option<&str>) -> Result<Seat, String> {
        let Some(value) = value.map(str::trim).filter(|v| !v.is_empty()) else {
            return Ok(Seat::Any { background: false });
        };
        if value.eq_ignore_ascii_case("any") {
            return Ok(Seat::Any { background: true });
        }
        if value.eq_ignore_ascii_case("none") {
            return Ok(Seat::None);
        }
        match value.parse::<u32>() {
            Ok(pad) if pad < MAX_PADS => Ok(Seat::Pad(pad)),
            _ => Err(format!(
                "NV_PAD={value}: give 0, 1, 2, 3, any or none; no controller is read"
            )),
        }
    }

    /// Whether the controller is read while another program's window is in front.
    fn background(self) -> bool {
        !matches!(self, Seat::Any { background: false })
    }
}

/// The system's XInput as this module asks it (a fake one in tests): the structures as their
/// bytes, the answers as XInput's result codes.
pub trait XInput: Send {
    /// `XInputGetState(pad, state)`.
    fn get_state(&mut self, pad: u32, state: &mut [u8; STATE_SIZE]) -> u32;
    /// `XInputSetState(pad, vibration)`.
    fn set_state(&mut self, pad: u32, vibration: &[u8; VIBRATION_SIZE]) -> u32;
    /// `XInputGetCapabilities(pad, flags, capabilities)`.
    fn get_capabilities(
        &mut self,
        pad: u32,
        flags: u32,
        capabilities: &mut [u8; CAPABILITIES_SIZE],
    ) -> u32;
    /// Whether one of this process's windows is in front (has the keyboard focus).
    fn in_front(&mut self) -> bool;
}

/// This module's state in each engine (`State::pads`).
#[derive(Default)]
pub struct Pads {
    /// The setting and the XInput, once the first call chose them ([`start`]).
    started: Option<(Seat, Option<Box<dyn XInput>>)>,
    /// The real controller that last answered for the game's user 0.
    pad: Option<u32>,
    /// For each real controller that did not answer, the calls left before it is asked again.
    wait: [u32; MAX_PADS as usize],
    /// Whether the last vibration sent was not zero.
    rumbling: bool,
}

impl std::fmt::Debug for Pads {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Pads")
            .field("seat", &self.started.as_ref().map(|(seat, _)| *seat))
            .field("pad", &self.pad)
            .field("wait", &self.wait)
            .field("rumbling", &self.rumbling)
            .finish()
    }
}

/// What `engine-core` asks for the game's XInput imports (installed by `units::funcs`).
pub static HOOK: engine_core::units::platform::Pads = engine_core::units::platform::Pads {
    get_state,
    set_state,
};

/// Uses `xinput` and `seat` for the engine's controller from now on (tests: a fake XInput).
pub fn use_xinput(e: &mut Engine, seat: Seat, xinput: Box<dyn XInput>) {
    state!(e).pads = Pads {
        started: Some((seat, Some(xinput))),
        ..Pads::default()
    };
}

/// Whether a scripted run keeps the real input away from the game: `NV_MOVE_KEYS` or
/// `NV_MOVE_MOUSE` set, without `NV_MOVE_REAL_INPUT=1` (the host's rule for the keyboard and
/// mouse, `player_move.rs` `real_input_reaches_game`).
fn scripted(env: impl Fn(&str) -> Option<String>) -> bool {
    let set = |name| env(name).is_some_and(|v| !v.trim().is_empty());
    (set("NV_MOVE_KEYS") || set("NV_MOVE_MOUSE"))
        && env("NV_MOVE_REAL_INPUT").as_deref() != Some("1")
}

/// The setting and the XInput for a run with this environment (the system's XInput is loaded
/// only when a controller may be read).
fn choose(env: impl Fn(&str) -> Option<String>) -> (Seat, Option<Box<dyn XInput>>) {
    if scripted(&env) {
        return (Seat::None, None);
    }
    let seat = Seat::parse(env("NV_PAD").as_deref()).unwrap_or_else(|why| {
        eprintln!("{why}");
        Seat::None
    });
    if seat == Seat::None {
        return (seat, None);
    }
    match system::load() {
        Some((xinput, dll)) => {
            eprintln!("controllers: XInput from {dll}, {}", describe(seat));
            (seat, Some(xinput))
        }
        None => {
            eprintln!(
                "controllers: no XInput DLL loaded (xinput1_4, xinput1_3, xinput9_1_0): \
                 no controller is read"
            );
            (seat, None)
        }
    }
}

/// The log's words for a setting.
fn describe(seat: Seat) -> String {
    match seat {
        Seat::Any { background: false } => {
            "the first controller, while the game's window is in front".to_string()
        }
        Seat::Any { background: true } => "the first controller, also in the background".into(),
        Seat::Pad(pad) => format!("controller {pad} (NV_PAD), also in the background"),
        Seat::None => "no controller".into(),
    }
}

/// The engine's [`Pads`], started on the first call ([`choose`] from the environment).
fn pads(e: &mut Engine) -> &mut Pads {
    let pads = &mut state!(e).pads;
    if pads.started.is_none() {
        // A test that did not give a fake XInput ([`use_xinput`]) gets no controller.
        pads.started = Some(if cfg!(test) {
            (Seat::None, None)
        } else {
            choose(|name| std::env::var(name).ok())
        });
    }
    pads
}

impl Pads {
    /// The setting and the XInput, when the game's user `user` may reach a controller now: user
    /// 0, a setting that reads one, an XInput, and (for the default setting) this process's
    /// window in front. When the window has gone to the back the motors are stopped first.
    fn open(&mut self, user: u32) -> Option<(Seat, &mut Box<dyn XInput>)> {
        let (seat, xinput) = self.started.as_mut()?;
        let seat = *seat;
        let xinput = xinput.as_mut()?;
        if user != 0 || seat == Seat::None {
            return None;
        }
        if !seat.background() && !xinput.in_front() {
            if std::mem::take(&mut self.rumbling) {
                if let Some(pad) = self.pad {
                    xinput.set_state(pad, &[0; VIBRATION_SIZE]);
                }
            }
            return None;
        }
        Some((seat, xinput))
    }

    /// The state of the game's user `user` from its real controller (the one it had while that
    /// answers, else the first that answers), `None` when none does.
    fn read(&mut self, user: u32) -> Option<[u8; STATE_SIZE]> {
        if user == 0 {
            for wait in &mut self.wait {
                *wait = wait.saturating_sub(1);
            }
        }
        let last = self.pad;
        let mut wait = self.wait;
        let (seat, xinput) = self.open(user)?;
        let order: Vec<u32> = match seat {
            Seat::Pad(pad) => vec![pad],
            Seat::Any { .. } => last
                .into_iter()
                .chain((0..MAX_PADS).filter(|&p| Some(p) != last))
                .collect(),
            Seat::None => Vec::new(),
        };
        let mut found = None;
        for pad in order {
            if wait[pad as usize] > 0 {
                continue;
            }
            let mut state = [0; STATE_SIZE];
            if xinput.get_state(pad, &mut state) == ERROR_SUCCESS {
                found = Some((pad, state));
                break;
            }
            wait[pad as usize] = RETRY_CALLS;
        }
        self.wait = wait;
        match found {
            Some((pad, state)) => {
                self.pad = Some(pad);
                Some(state)
            }
            None => {
                self.pad = None;
                None
            }
        }
    }

    /// The real controller the game's user `user` sends to (rumble, capabilities): the one it
    /// read last, or the setting's own controller.
    fn target(&mut self, user: u32) -> Option<(u32, &mut Box<dyn XInput>)> {
        let last = self.pad;
        let wait = self.wait;
        let (seat, xinput) = self.open(user)?;
        let pad = match seat {
            Seat::Pad(pad) if wait[pad as usize] == 0 => Some(pad),
            Seat::Any { .. } => last,
            _ => None,
        }?;
        Some((pad, xinput))
    }
}

// Platform: replaces XINPUT1_3.DLL XInputGetState (import by ordinal 2; `Controls::Poll`
// `00a23010` calls it through the thunk `009f996e`), through `engine-core`'s hook
/// `XInputGetState(user, state)`: the 16 bytes of the real controller's `XINPUT_STATE` written
/// at `state` and `ERROR_SUCCESS`; `ERROR_DEVICE_NOT_CONNECTED` with the state left as it is when
/// the game's user has no controller now (see the module's text). A null `state` with a
/// controller there is not provided.
pub fn get_state(e: &mut Engine, user: u32, state: Ptr) -> u32 {
    if e.repeatable {
        return ERROR_DEVICE_NOT_CONNECTED;
    }
    let Some(bytes) = pads(e).read(user) else {
        return ERROR_DEVICE_NOT_CONNECTED;
    };
    assert!(
        !state.is_null(),
        "XInputGetState with a null state is not provided"
    );
    e.mem.write(state.addr(), &bytes);
    ERROR_SUCCESS
}

// Platform: replaces XINPUT1_3.DLL XInputSetState (import by ordinal 3, slot `00fdf398`; the
// rumble `00a255b0` calls it through the thunk `00ee820a`), through `engine-core`'s hook
/// `XInputSetState(user, vibration)`: the 4 bytes of the game's `XINPUT_VIBRATION` sent to the
/// game's user's real controller and its answer; `ERROR_DEVICE_NOT_CONNECTED` when there is
/// none now. A null `vibration` with a controller there is not provided.
pub fn set_state(e: &mut Engine, user: u32, vibration: Ptr) -> u32 {
    if e.repeatable {
        return ERROR_DEVICE_NOT_CONNECTED;
    }
    let pads = pads(e);
    if pads.target(user).is_none() {
        return ERROR_DEVICE_NOT_CONNECTED;
    }
    assert!(
        !vibration.is_null(),
        "XInputSetState with a null vibration is not provided"
    );
    let mut bytes = [0; VIBRATION_SIZE];
    bytes.copy_from_slice(&e.mem.bytes(vibration.addr(), VIBRATION_SIZE as u32));
    let pads = &mut state!(e).pads;
    let Some((pad, xinput)) = pads.target(user) else {
        return ERROR_DEVICE_NOT_CONNECTED;
    };
    let answer = xinput.set_state(pad, &bytes);
    if answer == ERROR_SUCCESS {
        pads.rumbling = bytes != [0; VIBRATION_SIZE];
    }
    answer
}

// Platform: replaces XINPUT1_3.DLL XInputGetCapabilities (import by ordinal 4; no call in the
// repository's translations)
/// `XInputGetCapabilities(user, flags, capabilities)`: the real controller's 20 bytes of
/// `XINPUT_CAPABILITIES` at `capabilities` and XInput's answer (written only on
/// `ERROR_SUCCESS`); `ERROR_DEVICE_NOT_CONNECTED` when the game's user has no controller now.
/// With the default or `any` setting and no controller read yet, the first that answers is
/// chosen first.
pub fn get_capabilities(e: &mut Engine, user: u32, flags: u32, capabilities: Ptr) -> u32 {
    if e.repeatable {
        return ERROR_DEVICE_NOT_CONNECTED;
    }
    let pads = pads(e);
    if pads.target(user).is_none() && pads.read(user).is_none() {
        return ERROR_DEVICE_NOT_CONNECTED;
    }
    let Some((pad, xinput)) = pads.target(user) else {
        return ERROR_DEVICE_NOT_CONNECTED;
    };
    let mut bytes = [0; CAPABILITIES_SIZE];
    let answer = xinput.get_capabilities(pad, flags, &mut bytes);
    if answer == ERROR_SUCCESS {
        assert!(
            !capabilities.is_null(),
            "XInputGetCapabilities with null capabilities is not provided"
        );
        e.mem.write(capabilities.addr(), &bytes);
    }
    answer
}

/// The XInput imports this layer adds to the table (`lib.rs` `layer_imports`): ordinal 4. The
/// core answers ordinals 2 and 3 and asks [`HOOK`].
pub fn imports() -> Vec<(&'static str, &'static str, crate::AbiFn)> {
    vec![(
        "XINPUT1_3.dll",
        "#4",
        crate::entry!(0, get_capabilities(u32, u32, Ptr) -> u32).1,
    )]
}

/// The system's XInput DLL (Windows).
#[cfg(windows)]
mod system {
    use super::{XInput, CAPABILITIES_SIZE, STATE_SIZE, VIBRATION_SIZE};
    use windows::core::{s, w, PCWSTR};
    use windows::Win32::System::LibraryLoader::{GetProcAddress, LoadLibraryW};
    use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowThreadProcessId};

    type GetState = unsafe extern "system" fn(u32, *mut u8) -> u32;
    type SetState = unsafe extern "system" fn(u32, *mut u8) -> u32;
    type GetCapabilities = unsafe extern "system" fn(u32, u32, *mut u8) -> u32;
    /// What `GetProcAddress` answers.
    type Proc = unsafe extern "system" fn() -> isize;

    /// A structure's bytes at the alignment of its widest field (4: `XINPUT_STATE`'s packet
    /// number).
    #[repr(C, align(4))]
    struct Aligned<const N: usize>([u8; N]);

    /// The DLL's three functions.
    #[derive(Clone, Copy)]
    struct System {
        get_state: GetState,
        set_state: SetState,
        get_capabilities: Option<GetCapabilities>,
    }

    /// The first of the DLLs that loads with `XInputGetState` and `XInputSetState`, and its
    /// name, once for the process.
    fn system() -> Option<(System, &'static str)> {
        static SYSTEM: std::sync::OnceLock<Option<(System, &'static str)>> =
            std::sync::OnceLock::new();
        *SYSTEM.get_or_init(|| {
            let dlls: [(PCWSTR, &'static str); 3] = [
                (w!("xinput1_4.dll"), "xinput1_4.dll"),
                (w!("xinput1_3.dll"), "xinput1_3.dll"),
                (w!("xinput9_1_0.dll"), "xinput9_1_0.dll"),
            ];
            dlls.into_iter().find_map(|(wide, name)| {
                // SAFETY: Microsoft's system DLLs, found by Windows' search order for system
                // DLLs; the functions are cast to their documented signatures (xinput.h).
                unsafe {
                    let module = LoadLibraryW(wide).ok()?;
                    let get_state = GetProcAddress(module, s!("XInputGetState"))?;
                    let set_state = GetProcAddress(module, s!("XInputSetState"))?;
                    let get_capabilities = GetProcAddress(module, s!("XInputGetCapabilities"));
                    Some((
                        System {
                            get_state: std::mem::transmute::<Proc, GetState>(get_state),
                            set_state: std::mem::transmute::<Proc, SetState>(set_state),
                            get_capabilities: get_capabilities
                                .map(|f| std::mem::transmute::<Proc, GetCapabilities>(f)),
                        },
                        name,
                    ))
                }
            })
        })
    }

    /// The system's XInput and the DLL's name, `None` when none loads.
    pub fn load() -> Option<(Box<dyn XInput>, &'static str)> {
        system().map(|(s, name)| (Box::new(s) as Box<dyn XInput>, name))
    }

    impl XInput for System {
        fn get_state(&mut self, pad: u32, state: &mut [u8; STATE_SIZE]) -> u32 {
            let mut out = Aligned([0; STATE_SIZE]);
            // SAFETY: `out` is an `XINPUT_STATE`'s size and alignment.
            let answer = unsafe { (self.get_state)(pad, out.0.as_mut_ptr()) };
            *state = out.0;
            answer
        }

        fn set_state(&mut self, pad: u32, vibration: &[u8; VIBRATION_SIZE]) -> u32 {
            let mut v = Aligned(*vibration);
            // SAFETY: `v` is an `XINPUT_VIBRATION`'s size and alignment.
            unsafe { (self.set_state)(pad, v.0.as_mut_ptr()) }
        }

        fn get_capabilities(
            &mut self,
            pad: u32,
            flags: u32,
            capabilities: &mut [u8; CAPABILITIES_SIZE],
        ) -> u32 {
            let Some(f) = self.get_capabilities else {
                return super::ERROR_DEVICE_NOT_CONNECTED;
            };
            let mut out = Aligned([0; CAPABILITIES_SIZE]);
            // SAFETY: `out` is an `XINPUT_CAPABILITIES`'s size and alignment.
            let answer = unsafe { f(pad, flags, out.0.as_mut_ptr()) };
            *capabilities = out.0;
            answer
        }

        fn in_front(&mut self) -> bool {
            // SAFETY: plain queries of the foreground window and its process.
            unsafe {
                let window = GetForegroundWindow();
                if window.0.is_null() {
                    return false;
                }
                let mut process = 0u32;
                GetWindowThreadProcessId(window, Some(&mut process));
                process == std::process::id()
            }
        }
    }
}

/// No XInput off Windows.
#[cfg(not(windows))]
mod system {
    pub fn load() -> Option<(Box<dyn super::XInput>, &'static str)> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    /// What the fake XInput holds and saw: the controllers that answer and their states, the
    /// window in front or not, and every call made.
    #[derive(Default)]
    struct Machine {
        states: [Option<[u8; STATE_SIZE]>; 4],
        capabilities: [u8; CAPABILITIES_SIZE],
        in_front: bool,
        reads: Vec<u32>,
        sent: Vec<(u32, [u8; VIBRATION_SIZE])>,
        asked: Vec<(u32, u32)>,
    }

    #[derive(Clone, Default)]
    struct Fake(Arc<Mutex<Machine>>);

    impl XInput for Fake {
        fn get_state(&mut self, pad: u32, state: &mut [u8; STATE_SIZE]) -> u32 {
            let mut m = self.0.lock().unwrap();
            m.reads.push(pad);
            match m.states[pad as usize] {
                Some(s) => {
                    *state = s;
                    ERROR_SUCCESS
                }
                None => ERROR_DEVICE_NOT_CONNECTED,
            }
        }
        fn set_state(&mut self, pad: u32, vibration: &[u8; VIBRATION_SIZE]) -> u32 {
            let mut m = self.0.lock().unwrap();
            m.sent.push((pad, *vibration));
            if m.states[pad as usize].is_some() {
                ERROR_SUCCESS
            } else {
                ERROR_DEVICE_NOT_CONNECTED
            }
        }
        fn get_capabilities(
            &mut self,
            pad: u32,
            flags: u32,
            capabilities: &mut [u8; CAPABILITIES_SIZE],
        ) -> u32 {
            let mut m = self.0.lock().unwrap();
            m.asked.push((pad, flags));
            if m.states[pad as usize].is_some() {
                *capabilities = m.capabilities;
                ERROR_SUCCESS
            } else {
                ERROR_DEVICE_NOT_CONNECTED
            }
        }
        fn in_front(&mut self) -> bool {
            self.0.lock().unwrap().in_front
        }
    }

    /// An `XINPUT_STATE`: packet number 0x01020304, the A button (0x1000), the left trigger
    /// full, the left stick right (0x7fff) and the right stick down (-0x8000).
    fn pressed(tag: u8) -> [u8; STATE_SIZE] {
        [
            0x04, 0x03, 0x02, tag, 0x00, 0x10, 0xff, 0x00, 0xff, 0x7f, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x80,
        ]
    }

    fn engine(seat: Seat, fake: &Fake) -> (Engine, u32) {
        let mut e = Engine::new();
        use_xinput(&mut e, seat, Box::new(fake.clone()));
        let at = e.mem.alloc(0x40);
        e.mem.write(at, &[0xaa; 0x40]);
        (e, at)
    }

    #[test]
    fn the_setting_is_read() {
        assert_eq!(Seat::parse(None), Ok(Seat::Any { background: false }));
        assert_eq!(Seat::parse(Some(" ")), Ok(Seat::Any { background: false }));
        assert_eq!(Seat::parse(Some("ANY")), Ok(Seat::Any { background: true }));
        assert_eq!(Seat::parse(Some("none")), Ok(Seat::None));
        for pad in 0..4 {
            assert_eq!(Seat::parse(Some(&format!(" {pad} "))), Ok(Seat::Pad(pad)));
        }
        for bad in ["4", "-1", "pad1", "1.0"] {
            assert!(Seat::parse(Some(bad)).unwrap_err().contains(bad));
        }
    }

    /// A scripted run keeps the controllers away, as the keyboard and mouse, unless it asks for
    /// the real input; a bad setting reads none.
    #[test]
    fn a_scripted_run_or_a_bad_setting_reads_no_controller() {
        let env = |pairs: &'static [(&'static str, &'static str)]| {
            move |name: &str| {
                pairs
                    .iter()
                    .find(|(n, _)| *n == name)
                    .map(|(_, v)| v.to_string())
            }
        };
        assert!(scripted(env(&[("NV_MOVE_KEYS", "10:11")])));
        assert!(scripted(env(&[("NV_MOVE_MOUSE", "1:2:3:4")])));
        assert!(!scripted(env(&[
            ("NV_MOVE_KEYS", "10:11"),
            ("NV_MOVE_REAL_INPUT", "1")
        ])));
        assert!(!scripted(env(&[("NV_MOVE_KEYS", " ")])));
        assert!(!scripted(env(&[])));
        let (seat, xinput) = choose(env(&[("NV_MOVE_KEYS", "10:11"), ("NV_PAD", "1")]));
        assert_eq!(seat, Seat::None);
        assert!(xinput.is_none());
        let (seat, xinput) = choose(env(&[("NV_PAD", "7")]));
        assert_eq!(seat, Seat::None);
        assert!(xinput.is_none());
        let (seat, xinput) = choose(env(&[("NV_PAD", "none")]));
        assert_eq!(seat, Seat::None);
        assert!(xinput.is_none());
    }

    /// No controller: not connected, the game's state untouched (as before this module).
    #[test]
    fn without_a_controller_nothing_is_connected_and_nothing_written() {
        let fake = Fake::default();
        fake.0.lock().unwrap().in_front = true;
        for seat in [
            Seat::Any { background: false },
            Seat::Any { background: true },
            Seat::Pad(2),
            Seat::None,
        ] {
            let (mut e, at) = engine(seat, &fake);
            assert_eq!(
                get_state(&mut e, 0, Ptr::new(at)),
                ERROR_DEVICE_NOT_CONNECTED
            );
            assert_eq!(
                set_state(&mut e, 0, Ptr::new(at)),
                ERROR_DEVICE_NOT_CONNECTED
            );
            assert_eq!(
                get_capabilities(&mut e, 0, 1, Ptr::new(at)),
                ERROR_DEVICE_NOT_CONNECTED
            );
            assert_eq!(e.mem.bytes(at, 0x40), vec![0xaa; 0x40]);
        }
        assert!(fake.0.lock().unwrap().sent.is_empty());
    }

    /// `NV_PAD=1`: the game's user 0 is controller 1, its 16 bytes copied as they are; the other
    /// users have none, and controller 0 is never asked.
    #[test]
    fn a_seat_reads_its_own_controller_as_the_games_first() {
        let fake = Fake::default();
        {
            let mut m = fake.0.lock().unwrap();
            m.states[0] = Some(pressed(0xa0));
            m.states[1] = Some(pressed(0xa1));
        }
        let (mut e, at) = engine(Seat::Pad(1), &fake);
        assert_eq!(get_state(&mut e, 0, Ptr::new(at)), ERROR_SUCCESS);
        assert_eq!(e.mem.bytes(at, 16), pressed(0xa1).to_vec());
        assert_eq!(e.mem.bytes(at + 16, 4), vec![0xaa; 4]);
        for user in 1..4 {
            assert_eq!(
                get_state(&mut e, user, Ptr::new(at + 0x20)),
                ERROR_DEVICE_NOT_CONNECTED
            );
        }
        assert_eq!(fake.0.lock().unwrap().reads, vec![1]);
    }

    /// The default reads the first controller that answers and keeps it while it answers, even
    /// when a lower one appears; a lost one gives way to the next.
    #[test]
    fn any_takes_the_first_controller_and_keeps_it() {
        let fake = Fake::default();
        {
            let mut m = fake.0.lock().unwrap();
            m.in_front = true;
            m.states[2] = Some(pressed(0xa2));
        }
        let (mut e, at) = engine(Seat::Any { background: false }, &fake);
        assert_eq!(get_state(&mut e, 0, Ptr::new(at)), ERROR_SUCCESS);
        assert_eq!(e.mem.bytes(at, 16), pressed(0xa2).to_vec());
        assert_eq!(fake.0.lock().unwrap().reads, vec![0, 1, 2]);
        fake.0.lock().unwrap().states[0] = Some(pressed(0xa0));
        fake.0.lock().unwrap().reads.clear();
        assert_eq!(get_state(&mut e, 0, Ptr::new(at)), ERROR_SUCCESS);
        assert_eq!(e.mem.u8(at + 3), 0xa2);
        assert_eq!(fake.0.lock().unwrap().reads, vec![2]);
        // Controller 2 goes: controller 0 (no longer waited for) takes over.
        fake.0.lock().unwrap().states[2] = None;
        for _ in 0..RETRY_CALLS {
            get_state(&mut e, 0, Ptr::new(at));
        }
        assert_eq!(get_state(&mut e, 0, Ptr::new(at)), ERROR_SUCCESS);
        assert_eq!(e.mem.u8(at + 3), 0xa0);
    }

    /// A controller that does not answer is asked again only after `RETRY_CALLS` calls.
    #[test]
    fn a_missing_controller_is_asked_again_only_after_a_while() {
        let fake = Fake::default();
        let (mut e, at) = engine(Seat::Pad(3), &fake);
        for _ in 0..RETRY_CALLS {
            assert_eq!(
                get_state(&mut e, 0, Ptr::new(at)),
                ERROR_DEVICE_NOT_CONNECTED
            );
        }
        assert_eq!(fake.0.lock().unwrap().reads, vec![3]);
        fake.0.lock().unwrap().states[3] = Some(pressed(0xa3));
        assert_eq!(get_state(&mut e, 0, Ptr::new(at)), ERROR_SUCCESS);
        assert_eq!(fake.0.lock().unwrap().reads, vec![3, 3]);
        assert_eq!(e.mem.u8(at + 3), 0xa3);
    }

    /// The rumble goes to the controller the game reads, as the game's 4 bytes.
    #[test]
    fn the_rumble_goes_to_the_seats_controller() {
        let fake = Fake::default();
        fake.0.lock().unwrap().states[1] = Some(pressed(0xa1));
        let (mut e, at) = engine(Seat::Pad(1), &fake);
        e.mem.write(at, &[0x34, 0x12, 0xff, 0xff]);
        assert_eq!(set_state(&mut e, 0, Ptr::new(at)), ERROR_SUCCESS);
        assert_eq!(
            set_state(&mut e, 2, Ptr::new(at)),
            ERROR_DEVICE_NOT_CONNECTED
        );
        assert_eq!(
            fake.0.lock().unwrap().sent,
            vec![(1, [0x34, 0x12, 0xff, 0xff])]
        );
    }

    /// The capabilities of the game's controller, with the game's flags, 20 bytes as they are.
    #[test]
    fn the_capabilities_are_the_controllers() {
        let fake = Fake::default();
        let caps: [u8; CAPABILITIES_SIZE] = std::array::from_fn(|i| i as u8 + 1);
        {
            let mut m = fake.0.lock().unwrap();
            m.states[1] = Some(pressed(0xa1));
            m.capabilities = caps;
        }
        let (mut e, at) = engine(Seat::Any { background: true }, &fake);
        assert_eq!(get_capabilities(&mut e, 0, 1, Ptr::new(at)), ERROR_SUCCESS);
        assert_eq!(e.mem.bytes(at, 20), caps.to_vec());
        assert_eq!(e.mem.u8(at + 20), 0xaa);
        assert_eq!(fake.0.lock().unwrap().asked, vec![(1, 1)]);
    }

    /// The default reads only while this process's window is in front, and stops the motors when
    /// it goes to the back; a seat (`NV_PAD` set) reads in the background too.
    #[test]
    fn only_a_seat_reads_in_the_background() {
        let fake = Fake::default();
        {
            let mut m = fake.0.lock().unwrap();
            m.states[0] = Some(pressed(0xa0));
            m.in_front = true;
        }
        let (mut e, at) = engine(Seat::Any { background: false }, &fake);
        assert_eq!(get_state(&mut e, 0, Ptr::new(at)), ERROR_SUCCESS);
        let v = e.mem.alloc(4);
        e.mem.write(v, &[0, 0x80, 0, 0x80]);
        assert_eq!(set_state(&mut e, 0, Ptr::new(v)), ERROR_SUCCESS);
        fake.0.lock().unwrap().in_front = false;
        e.mem.write(at, &[0xaa; 16]);
        assert_eq!(
            get_state(&mut e, 0, Ptr::new(at)),
            ERROR_DEVICE_NOT_CONNECTED
        );
        assert_eq!(e.mem.bytes(at, 16), vec![0xaa; 16]);
        assert_eq!(
            set_state(&mut e, 0, Ptr::new(v)),
            ERROR_DEVICE_NOT_CONNECTED
        );
        assert_eq!(
            fake.0.lock().unwrap().sent,
            vec![(0, [0, 0x80, 0, 0x80]), (0, [0; 4])]
        );
        for seat in [Seat::Pad(0), Seat::Any { background: true }] {
            let (mut e, at) = engine(seat, &fake);
            assert_eq!(get_state(&mut e, 0, Ptr::new(at)), ERROR_SUCCESS);
        }
    }

    /// `NV_REPEATABLE`: never connected, XInput never asked, so two runs stay equal.
    #[test]
    fn a_repeatable_run_has_no_controller() {
        let fake = Fake::default();
        fake.0.lock().unwrap().states[0] = Some(pressed(0xa0));
        let (mut e, at) = engine(Seat::Pad(0), &fake);
        e.start_repeatable_clock();
        assert_eq!(
            get_state(&mut e, 0, Ptr::new(at)),
            ERROR_DEVICE_NOT_CONNECTED
        );
        assert_eq!(
            set_state(&mut e, 0, Ptr::new(at)),
            ERROR_DEVICE_NOT_CONNECTED
        );
        assert_eq!(
            get_capabilities(&mut e, 0, 0, Ptr::new(at)),
            ERROR_DEVICE_NOT_CONNECTED
        );
        let m = fake.0.lock().unwrap();
        assert!(m.reads.is_empty() && m.sent.is_empty() && m.asked.is_empty());
    }

    /// The game's imports reach this module: ordinals 2 and 3 through the core's hook, 4 from
    /// this layer's table, at their import slots.
    #[test]
    fn the_games_imports_are_answered_here() {
        let fake = Fake::default();
        fake.0.lock().unwrap().states[0] = Some(pressed(0xa0));
        let (mut e, at) = engine(Seat::Pad(0), &fake);
        e.mem.map(0x00fd_f000, 0x1000);
        let import = |slot: u32, name: &str| crate::exe::Import {
            slot,
            dll: "XINPUT1_3.dll".to_string(),
            name: name.to_string(),
        };
        let imports = [
            import(0x00fd_f394, "#2"),
            import(0x00fd_f398, "#3"),
            import(0x00fd_f39c, "#4"),
        ];
        assert_eq!(e.register_imports(&imports), 3);
        assert_eq!(
            e.call(0x00fd_f394, &crate::args![0u32, at]).u32(),
            ERROR_SUCCESS
        );
        assert_eq!(e.mem.bytes(at, 16), pressed(0xa0).to_vec());
        e.mem.write(at, &[1, 0, 2, 0]);
        assert_eq!(
            e.call(0x00fd_f398, &crate::args![0u32, at]).u32(),
            ERROR_SUCCESS
        );
        assert_eq!(
            e.call(0x00fd_f39c, &crate::args![0u32, 1u32, at]).u32(),
            ERROR_SUCCESS
        );
        assert_eq!(fake.0.lock().unwrap().sent, vec![(0, [1, 0, 2, 0])]);
    }
}

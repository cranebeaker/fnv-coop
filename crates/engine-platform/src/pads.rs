//! The game's controllers from the PC's real ones (docs/coop/SPECS.md S39, "controllers"): its
//! XInput pad here, its DirectInput joysticks in [`crate::joysticks`].
//!
//! What the game calls (FalloutNV.exe 1.4.0.525, from this repository's translations):
//! `Controls::Poll` (`00a23010`, through the thunk `009f996e`) asks `XInputGetState(0, 011f35a8)`
//! every frame and counts the pad as connected when that answers 0; the rumble (`00a255b0`,
//! through `00ee820a`) sends `XInputSetState(0, vibration)`. Both are imports of `XINPUT1_3.dll`
//! by ordinal (2 and 3), answered in `engine-core`'s `units/platform.rs`, which asks this module
//! through its `Pads` hook ([`HOOK`]). Ordinal 4, `XInputGetCapabilities`, is answered too
//! ([`get_capabilities`]), although none of the repository's translations calls it. The game only
//! ever names user index 0. Its input start-up also enumerates DirectInput game controllers
//! (`00a252d0`, `00a24830`) and `Controls::Poll` reads each one's state: [`crate::joysticks`].
//!
//! The XInput answers come from the system's XInput, loaded in this (64-bit) process:
//! `xinput1_4.dll`, else `xinput1_3.dll`, else `xinput9_1_0.dll`. The structures are the same
//! bytes in a 32-bit and a 64-bit process (Microsoft's `XINPUT_STATE`, 16 bytes: the packet
//! number, then `XINPUT_GAMEPAD`, the buttons word, the two trigger bytes and the four thumb-stick
//! words; `XINPUT_VIBRATION`, 4 bytes: the two motor speeds; `XINPUT_CAPABILITIES`, 20 bytes: the
//! type and subtype bytes, the flags word, an `XINPUT_GAMEPAD` and an `XINPUT_VIBRATION`), so
//! they are copied as they are between the game's memory and the system's calls.
//!
//! The PC's controllers, in this order: the XInput controllers (user indices 0 to 3), then the
//! DirectInput game controllers that are not XInput controllers (an Xbox pad is listed by both;
//! its DirectInput side is left out, so it is read once), in DirectInput's order. Which of them
//! this copy reads is the setting `NV_PAD` ([`Seat::parse`]):
//! - unset: every controller (the first XInput controller that answers, every DirectInput one),
//!   read only while one of this process's windows is in front, so a second copy of the game or
//!   an alt-tabbed game does not move with the player's pad (the game itself would pause there,
//!   but this engine's game never hears that its window went to the back: `window.rs`);
//! - `0` to `15`: the controller at that place in the list, as it stands when the game first
//!   asks (connect the controllers before the game starts), also while another window is in
//!   front (a Couchlink seat: `NV_PAD=$SEAT`). An XInput one is the game's XInput pad, a
//!   DirectInput one its only joystick;
//! - `any`: every controller, as unset, also in the background;
//! - `none`: no controller.
//!
//! No controller is there for the game (XInput's `ERROR_DEVICE_NOT_CONNECTED` with the state left
//! as it is, no joystick listed) when the setting finds none, when the system has none, in a
//! repeatable run (`NV_REPEATABLE`, so two runs stay equal), and in a scripted run
//! (`NV_MOVE_KEYS` or `NV_MOVE_MOUSE`) unless `NV_MOVE_REAL_INPUT=1`, as for the real keyboard and
//! mouse (the host's `real_input_reaches_game`). An XInput controller that does not answer is
//! asked again only after [`RETRY_CALLS`] more of the game's calls: asking for a missing
//! controller is slow in XInput. When the controller stops being read because the game's window
//! went to the back, its motors are stopped.
//!
//! Tests use a fake machine ([`use_controllers`]); the system's XInput and DirectInput are never
//! loaded by a test.

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
/// The places `NV_PAD` can name (0 to 15).
pub const MAX_SEATS: u32 = 16;
/// How many of the game's calls go by before a controller that did not answer is asked again
/// (about a second at 60 frames a second).
pub const RETRY_CALLS: u32 = 60;

/// Which controllers this copy of the game reads (`NV_PAD`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Seat {
    /// Every controller; `background`: also while another program's window is in front
    /// (`NV_PAD=any`), else only while this process's is (`NV_PAD` unset).
    Any { background: bool },
    /// The controller at that place in the list (0 to 15), also in the background (`NV_PAD=N`).
    Pad(u32),
    /// No controller (`NV_PAD=none`).
    None,
}

impl Seat {
    /// `NV_PAD`'s value (`None` when unset): `0` to `15`, `any` or `none` (any case, spaces
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
            Ok(pad) if pad < MAX_SEATS => Ok(Seat::Pad(pad)),
            _ => Err(format!(
                "NV_PAD={value}: give 0 to 15, any or none; no controller is read"
            )),
        }
    }

    /// Whether the controllers are read while another program's window is in front.
    pub fn background(self) -> bool {
        !matches!(self, Seat::Any { background: false })
    }
}

/// One DirectInput game controller of the PC: its `DIDEVICEINSTANCEA` (0x244 bytes, no pointers:
/// the same in a 32-bit and a 64-bit process) and whether it is also an XInput controller.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JoystickInfo {
    pub instance: Vec<u8>,
    pub xinput: bool,
}

impl JoystickInfo {
    /// The instance GUID (`guidInstance`, `+4`).
    pub fn guid(&self) -> [u8; 16] {
        let mut guid = [0; 16];
        guid.copy_from_slice(&self.instance[4..0x14]);
        guid
    }

    /// The product name (`tszProductName`, `+0x12c`), for the log.
    pub fn name(&self) -> String {
        let name = &self.instance[0x12c..0x230];
        let end = name.iter().position(|&b| b == 0).unwrap_or(name.len());
        String::from_utf8_lossy(&name[..end]).into_owned()
    }
}

/// The PC's controllers as this module asks them (a fake machine in tests): the structures as
/// their bytes, the answers as XInput's and DirectInput's result codes.
pub trait Controllers: Send {
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
    /// The attached DirectInput game controllers, in DirectInput's order.
    fn joysticks(&mut self) -> Vec<JoystickInfo>;
    /// A DirectInput device on the game controller with this instance GUID.
    fn open_joystick(&mut self, guid: &[u8; 16]) -> Option<Box<dyn crate::joysticks::Joystick>>;
    /// Whether one of this process's windows is in front (has the keyboard focus).
    fn in_front(&mut self) -> bool;
}

/// A controller of the list `NV_PAD` numbers: an XInput user index, or a place among the
/// DirectInput game controllers that are not XInput ones.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Controller {
    XInput(u32),
    DirectInput(usize),
}

/// The setting and the machine, once the first call chose them ([`pads`]).
struct Started {
    seat: Seat,
    machine: Option<Box<dyn Controllers>>,
    /// The DirectInput game controllers that are not XInput ones, once listed.
    joysticks: Option<Vec<JoystickInfo>>,
    /// `NV_PAD=N`'s controller, once worked out.
    chosen: Option<Option<Controller>>,
}

/// This module's state in each engine (`State::pads`).
#[derive(Default)]
pub struct Pads {
    started: Option<Started>,
    /// The XInput controller that last answered for the game's user 0.
    pad: Option<u32>,
    /// For each XInput controller that did not answer, the calls left before it is asked again.
    wait: [u32; MAX_PADS as usize],
    /// Whether the last vibration sent was not zero.
    rumbling: bool,
    /// The game's DirectInput joystick devices ([`crate::joysticks`]).
    pub(crate) joysticks: crate::joysticks::Devices,
}

impl std::fmt::Debug for Pads {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Pads")
            .field("seat", &self.started.as_ref().map(|s| s.seat))
            .field("pad", &self.pad)
            .field("wait", &self.wait)
            .field("rumbling", &self.rumbling)
            .field("joysticks", &self.joysticks)
            .finish()
    }
}

/// What `engine-core` asks for the game's XInput imports (installed by `units::funcs`).
pub static HOOK: engine_core::units::platform::Pads = engine_core::units::platform::Pads {
    get_state,
    set_state,
};

/// Uses `machine` and `seat` for the engine's controllers from now on (tests: a fake machine).
pub fn use_controllers(e: &mut Engine, seat: Seat, machine: Box<dyn Controllers>) {
    state!(e).pads = Pads {
        started: Some(Started {
            seat,
            machine: Some(machine),
            joysticks: None,
            chosen: None,
        }),
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

/// The setting and the machine for a run with this environment (the system's XInput and
/// DirectInput are reached only when a controller may be read).
fn choose(env: impl Fn(&str) -> Option<String>) -> (Seat, Option<Box<dyn Controllers>>) {
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
    let (machine, xinput) = crate::joysticks::system();
    match xinput {
        Some(dll) => eprintln!("controllers: XInput from {dll}, {}", describe(seat)),
        None => eprintln!(
            "controllers: no XInput DLL loaded (xinput1_4, xinput1_3, xinput9_1_0): only \
             DirectInput game controllers, {}",
            describe(seat)
        ),
    }
    (seat, machine)
}

/// The log's words for a setting.
fn describe(seat: Seat) -> String {
    match seat {
        Seat::Any { background: false } => {
            "every controller, while the game's window is in front".to_string()
        }
        Seat::Any { background: true } => "every controller, also in the background".into(),
        Seat::Pad(pad) => format!("controller {pad} of the list (NV_PAD), also in the background"),
        Seat::None => "no controller".into(),
    }
}

/// The engine's [`Pads`], started on the first call ([`choose`] from the environment).
pub(crate) fn pads(e: &mut Engine) -> &mut Pads {
    let pads = &mut state!(e).pads;
    if pads.started.is_none() {
        // A test that did not give a fake machine ([`use_controllers`]) gets no controller.
        let (seat, machine) = if cfg!(test) {
            (Seat::None, None)
        } else {
            choose(|name| std::env::var(name).ok())
        };
        pads.started = Some(Started {
            seat,
            machine,
            joysticks: None,
            chosen: None,
        });
    }
    pads
}

impl Pads {
    /// The setting, when it reads any controller and there is a machine.
    fn seat(&self) -> Option<Seat> {
        let started = self.started.as_ref()?;
        started.machine.as_ref()?;
        (started.seat != Seat::None).then_some(started.seat)
    }

    /// Whether the controllers may be read now: a setting that reads any, a machine, and (for
    /// the default setting) this process's window in front. When the window has gone to the back
    /// the motors are stopped first.
    pub(crate) fn readable(&mut self) -> bool {
        let Some(seat) = self.seat() else {
            return false;
        };
        let machine = self.machine();
        if seat.background() || machine.in_front() {
            return true;
        }
        if std::mem::take(&mut self.rumbling) {
            if let Some(pad) = self.pad {
                self.machine().set_state(pad, &[0; VIBRATION_SIZE]);
            }
        }
        false
    }

    /// The machine (only after [`Pads::seat`] found one).
    fn machine(&mut self) -> &mut Box<dyn Controllers> {
        self.started
            .as_mut()
            .and_then(|s| s.machine.as_mut())
            .expect("a machine")
    }

    /// The DirectInput game controllers that are not XInput ones, listed once.
    fn all_joysticks(&mut self) -> Vec<JoystickInfo> {
        let Some(started) = self.started.as_mut() else {
            return Vec::new();
        };
        let Some(machine) = started.machine.as_mut() else {
            return Vec::new();
        };
        started
            .joysticks
            .get_or_insert_with(|| {
                machine
                    .joysticks()
                    .into_iter()
                    .filter(|j| !j.xinput)
                    .collect()
            })
            .clone()
    }

    /// `NV_PAD=N`'s controller: place N of the XInput controllers that answer now (by user
    /// index), then the DirectInput ones; worked out once and logged.
    fn chosen(&mut self) -> Option<Controller> {
        let Some(Seat::Pad(n)) = self.seat() else {
            return None;
        };
        if let Some(chosen) = self.started.as_ref().and_then(|s| s.chosen) {
            return chosen;
        }
        let mut list: Vec<Controller> = (0..MAX_PADS)
            .filter(|&pad| self.machine().get_state(pad, &mut [0; STATE_SIZE]) == ERROR_SUCCESS)
            .map(Controller::XInput)
            .collect();
        let joysticks = self.all_joysticks();
        list.extend((0..joysticks.len()).map(Controller::DirectInput));
        let chosen = list.get(n as usize).copied();
        match chosen {
            Some(Controller::XInput(pad)) => {
                eprintln!("controllers: NV_PAD={n} is XInput controller {pad}")
            }
            Some(Controller::DirectInput(k)) => eprintln!(
                "controllers: NV_PAD={n} is the DirectInput game controller {:?}",
                joysticks[k].name()
            ),
            None => eprintln!(
                "controllers: NV_PAD={n}, but the PC has {} controllers: none is read",
                list.len()
            ),
        }
        if let Some(started) = self.started.as_mut() {
            started.chosen = Some(chosen);
        }
        chosen
    }

    /// The DirectInput game controllers the game is shown: none without a setting; the chosen
    /// one for `NV_PAD=N`; every one (not XInput) otherwise.
    pub(crate) fn game_joysticks(&mut self) -> Vec<JoystickInfo> {
        match self.seat() {
            None | Some(Seat::None) => Vec::new(),
            Some(Seat::Pad(_)) => match self.chosen() {
                Some(Controller::DirectInput(k)) => vec![self.all_joysticks()[k].clone()],
                _ => Vec::new(),
            },
            Some(Seat::Any { .. }) => self.all_joysticks(),
        }
    }

    /// A DirectInput device on a game controller the game was shown, by its instance GUID.
    pub(crate) fn open_joystick(
        &mut self,
        guid: &[u8; 16],
    ) -> Option<Box<dyn crate::joysticks::Joystick>> {
        if !self.game_joysticks().iter().any(|j| &j.guid() == guid) {
            return None;
        }
        self.machine().open_joystick(guid)
    }

    /// The XInput controllers the game's user `user` may read now, in the order to ask them.
    fn xinput_order(&mut self, user: u32) -> Vec<u32> {
        if user != 0 || !self.readable() {
            return Vec::new();
        }
        match self.seat() {
            Some(Seat::Pad(_)) => match self.chosen() {
                Some(Controller::XInput(pad)) => vec![pad],
                _ => Vec::new(),
            },
            Some(Seat::Any { .. }) => {
                let last = self.pad;
                last.into_iter()
                    .chain((0..MAX_PADS).filter(|&p| Some(p) != last))
                    .collect()
            }
            _ => Vec::new(),
        }
    }

    /// The state of the game's user `user` from its XInput controller (the one it had while
    /// that answers, else the first that answers), `None` when none does.
    fn read(&mut self, user: u32) -> Option<[u8; STATE_SIZE]> {
        if user == 0 {
            for wait in &mut self.wait {
                *wait = wait.saturating_sub(1);
            }
        }
        let order = self.xinput_order(user);
        for pad in order {
            if self.wait[pad as usize] > 0 {
                continue;
            }
            let mut state = [0; STATE_SIZE];
            if self.machine().get_state(pad, &mut state) == ERROR_SUCCESS {
                self.pad = Some(pad);
                return Some(state);
            }
            self.wait[pad as usize] = RETRY_CALLS;
        }
        self.pad = None;
        None
    }

    /// The XInput controller the game's user `user` sends to (rumble, capabilities): the one it
    /// read last, or `NV_PAD`'s own XInput controller.
    fn target(&mut self, user: u32) -> Option<u32> {
        let last = self.pad;
        let order = self.xinput_order(user);
        match self.seat()? {
            Seat::Pad(_) => order
                .first()
                .copied()
                .filter(|&p| self.wait[p as usize] == 0),
            Seat::Any { .. } => last.filter(|_| !order.is_empty()),
            Seat::None => None,
        }
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
    let Some(pad) = pads(e).target(user) else {
        return ERROR_DEVICE_NOT_CONNECTED;
    };
    assert!(
        !vibration.is_null(),
        "XInputSetState with a null vibration is not provided"
    );
    let mut bytes = [0; VIBRATION_SIZE];
    bytes.copy_from_slice(&e.mem.bytes(vibration.addr(), VIBRATION_SIZE as u32));
    let pads = pads(e);
    let answer = pads.machine().set_state(pad, &bytes);
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
    let Some(pad) = pads.target(user) else {
        return ERROR_DEVICE_NOT_CONNECTED;
    };
    let mut bytes = [0; CAPABILITIES_SIZE];
    let answer = pads.machine().get_capabilities(pad, flags, &mut bytes);
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

/// The fake machine the tests of this module and of [`crate::joysticks`] use.
#[cfg(test)]
pub(crate) mod fake {
    use super::*;
    use crate::joysticks::{DataFormat, Joystick};
    use std::sync::{Arc, Mutex};

    /// One fake DirectInput game controller: its record, whether it is an XInput one, its
    /// capabilities, objects and state, and what the game did to it.
    #[derive(Default, Clone)]
    pub struct Stick {
        pub info: Option<JoystickInfo>,
        pub capabilities: Vec<u8>,
        pub objects: Vec<Vec<u8>>,
        pub state: Vec<u8>,
        pub format: Option<DataFormat>,
        pub properties: Vec<(u32, Vec<u8>)>,
        pub acquired: bool,
        pub polls: u32,
        pub open: u32,
    }

    /// What the fake machine holds and saw.
    #[derive(Default)]
    pub struct Machine {
        pub states: [Option<[u8; STATE_SIZE]>; 4],
        pub capabilities: [u8; CAPABILITIES_SIZE],
        pub in_front: bool,
        pub reads: Vec<u32>,
        pub sent: Vec<(u32, [u8; VIBRATION_SIZE])>,
        pub asked: Vec<(u32, u32)>,
        pub sticks: Vec<Stick>,
        pub listings: u32,
    }

    #[derive(Clone, Default)]
    pub struct Fake(pub Arc<Mutex<Machine>>);

    impl Fake {
        pub fn machine(&self) -> std::sync::MutexGuard<'_, Machine> {
            self.0.lock().unwrap()
        }
    }

    /// A `DIDEVICEINSTANCEA` with instance GUID `tag` repeated, type `kind` and a name.
    pub fn instance(tag: u8, kind: u32, name: &str) -> Vec<u8> {
        let mut r = vec![0u8; 0x244];
        r[0..4].copy_from_slice(&0x244u32.to_le_bytes());
        r[4..0x14].copy_from_slice(&[tag; 16]);
        r[0x14..0x24].copy_from_slice(&[tag ^ 0xff; 16]);
        r[0x24..0x28].copy_from_slice(&kind.to_le_bytes());
        r[0x28..0x28 + name.len()].copy_from_slice(name.as_bytes());
        r[0x12c..0x12c + name.len()].copy_from_slice(name.as_bytes());
        r
    }

    impl Controllers for Fake {
        fn get_state(&mut self, pad: u32, state: &mut [u8; STATE_SIZE]) -> u32 {
            let mut m = self.machine();
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
            let mut m = self.machine();
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
            let mut m = self.machine();
            m.asked.push((pad, flags));
            if m.states[pad as usize].is_some() {
                *capabilities = m.capabilities;
                ERROR_SUCCESS
            } else {
                ERROR_DEVICE_NOT_CONNECTED
            }
        }
        fn joysticks(&mut self) -> Vec<JoystickInfo> {
            let mut m = self.machine();
            m.listings += 1;
            m.sticks.iter().filter_map(|s| s.info.clone()).collect()
        }
        fn open_joystick(&mut self, guid: &[u8; 16]) -> Option<Box<dyn Joystick>> {
            let mut m = self.machine();
            let at = m
                .sticks
                .iter()
                .position(|s| s.info.as_ref().is_some_and(|i| &i.guid() == guid))?;
            m.sticks[at].open += 1;
            Some(Box::new(FakeStick {
                fake: self.clone(),
                at,
            }))
        }
        fn in_front(&mut self) -> bool {
            self.machine().in_front
        }
    }

    /// A device on fake stick `at`.
    pub struct FakeStick {
        fake: Fake,
        at: usize,
    }

    impl FakeStick {
        fn with<T>(&self, f: impl FnOnce(&mut Stick) -> T) -> T {
            f(&mut self.fake.machine().sticks[self.at])
        }
    }

    impl Drop for FakeStick {
        fn drop(&mut self) {
            self.with(|s| s.open -= 1);
        }
    }

    impl Joystick for FakeStick {
        fn set_data_format(&mut self, format: &DataFormat) -> u32 {
            self.with(|s| s.format = Some(format.clone()));
            0
        }
        fn capabilities(&mut self, caps: &mut [u8]) -> u32 {
            self.with(|s| caps.copy_from_slice(&s.capabilities));
            0
        }
        fn objects(&mut self, _flags: u32) -> Result<Vec<Vec<u8>>, u32> {
            Ok(self.with(|s| s.objects.clone()))
        }
        fn set_property(&mut self, property: u32, block: &[u8]) -> u32 {
            self.with(|s| s.properties.push((property, block.to_vec())));
            0
        }
        fn acquire(&mut self) -> u32 {
            self.with(|s| u32::from(std::mem::replace(&mut s.acquired, true)))
        }
        fn unacquire(&mut self) -> u32 {
            self.with(|s| u32::from(!std::mem::replace(&mut s.acquired, false)))
        }
        fn poll(&mut self) -> u32 {
            self.with(|s| s.polls += 1);
            0
        }
        fn state(&mut self, data: &mut [u8]) -> u32 {
            self.with(|s| {
                if !s.acquired {
                    return crate::dinput::DIERR_NOTACQUIRED;
                }
                data.copy_from_slice(&s.state[..data.len()]);
                0
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::fake::{instance, Fake, Stick};
    use super::*;

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
        use_controllers(&mut e, seat, Box::new(fake.clone()));
        let at = e.mem.alloc(0x40);
        e.mem.write(at, &[0xaa; 0x40]);
        (e, at)
    }

    fn stick(tag: u8, xinput: bool) -> Stick {
        Stick {
            info: Some(JoystickInfo {
                instance: instance(tag, 0x0115, "Pad"),
                xinput,
            }),
            ..Stick::default()
        }
    }

    #[test]
    fn the_setting_is_read() {
        assert_eq!(Seat::parse(None), Ok(Seat::Any { background: false }));
        assert_eq!(Seat::parse(Some(" ")), Ok(Seat::Any { background: false }));
        assert_eq!(Seat::parse(Some("ANY")), Ok(Seat::Any { background: true }));
        assert_eq!(Seat::parse(Some("none")), Ok(Seat::None));
        for pad in [0, 1, 3, 4, 15] {
            assert_eq!(Seat::parse(Some(&format!(" {pad} "))), Ok(Seat::Pad(pad)));
        }
        for bad in ["16", "-1", "pad1", "1.0"] {
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
        for pairs in [
            &[("NV_MOVE_KEYS", "10:11"), ("NV_PAD", "1")][..],
            &[("NV_PAD", "16")][..],
            &[("NV_PAD", "none")][..],
        ] {
            let (seat, machine) = choose(env(pairs));
            assert_eq!(seat, Seat::None);
            assert!(machine.is_none());
        }
    }

    /// No controller: not connected, the game's state untouched (as before this module).
    #[test]
    fn without_a_controller_nothing_is_connected_and_nothing_written() {
        let fake = Fake::default();
        fake.machine().in_front = true;
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
            assert!(pads(&mut e).game_joysticks().is_empty());
        }
        assert!(fake.machine().sent.is_empty());
    }

    /// `NV_PAD=1` with XInput controllers 0 and 1: the game's user 0 is controller 1, its 16 bytes
    /// copied as they are; the other users have none, and controller 0 is not read after the
    /// list was made.
    #[test]
    fn a_seat_reads_its_own_controller_as_the_games_first() {
        let fake = Fake::default();
        {
            let mut m = fake.machine();
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
        // The list (0, 1, 2, 3 asked once), then controller 1 only.
        assert_eq!(fake.machine().reads, vec![0, 1, 2, 3, 1]);
        assert!(pads(&mut e).game_joysticks().is_empty());
    }

    /// The list: the XInput controllers that answer, then the DirectInput ones that are not
    /// XInput ones. `NV_PAD=1` with XInput controller 2 and two DirectInput controllers (the
    /// first an Xbox pad's own DirectInput side) is the second DirectInput one: the game's only
    /// joystick, and no XInput pad.
    #[test]
    fn the_list_puts_xinput_first_and_leaves_out_an_xbox_pads_directinput_side() {
        let fake = Fake::default();
        {
            let mut m = fake.machine();
            m.states[2] = Some(pressed(0xa2));
            m.sticks = vec![stick(0x11, true), stick(0x22, false), stick(0x33, false)];
        }
        let (mut e, at) = engine(Seat::Pad(1), &fake);
        let shown = pads(&mut e).game_joysticks();
        assert_eq!(shown.len(), 1);
        assert_eq!(shown[0].guid(), [0x22; 16]);
        assert_eq!(
            get_state(&mut e, 0, Ptr::new(at)),
            ERROR_DEVICE_NOT_CONNECTED
        );
        assert!(pads(&mut e).open_joystick(&[0x33; 16]).is_none());
        assert!(pads(&mut e).open_joystick(&[0x11; 16]).is_none());
        assert!(pads(&mut e).open_joystick(&[0x22; 16]).is_some());
        // NV_PAD=0 is the XInput controller; the default shows both DirectInput ones (not the
        // Xbox pad's).
        let (mut e, at) = engine(Seat::Pad(0), &fake);
        assert_eq!(get_state(&mut e, 0, Ptr::new(at)), ERROR_SUCCESS);
        assert!(pads(&mut e).game_joysticks().is_empty());
        let (mut e, _) = engine(Seat::Any { background: true }, &fake);
        let guids: Vec<_> = pads(&mut e)
            .game_joysticks()
            .iter()
            .map(|j| j.guid()[0])
            .collect();
        assert_eq!(guids, vec![0x22, 0x33]);
        // A place past the list: nothing.
        let (mut e, at) = engine(Seat::Pad(3), &fake);
        assert_eq!(
            get_state(&mut e, 0, Ptr::new(at)),
            ERROR_DEVICE_NOT_CONNECTED
        );
        assert!(pads(&mut e).game_joysticks().is_empty());
    }

    /// The default reads the first controller that answers and keeps it while it answers, even
    /// when a lower one appears; a lost one gives way to the next.
    #[test]
    fn any_takes_the_first_controller_and_keeps_it() {
        let fake = Fake::default();
        {
            let mut m = fake.machine();
            m.in_front = true;
            m.states[2] = Some(pressed(0xa2));
        }
        let (mut e, at) = engine(Seat::Any { background: false }, &fake);
        assert_eq!(get_state(&mut e, 0, Ptr::new(at)), ERROR_SUCCESS);
        assert_eq!(e.mem.bytes(at, 16), pressed(0xa2).to_vec());
        assert_eq!(fake.machine().reads, vec![0, 1, 2]);
        fake.machine().states[0] = Some(pressed(0xa0));
        fake.machine().reads.clear();
        assert_eq!(get_state(&mut e, 0, Ptr::new(at)), ERROR_SUCCESS);
        assert_eq!(e.mem.u8(at + 3), 0xa2);
        assert_eq!(fake.machine().reads, vec![2]);
        // Controller 2 goes: controller 0 (no longer waited for) takes over.
        fake.machine().states[2] = None;
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
        fake.machine().in_front = true;
        let (mut e, at) = engine(Seat::Any { background: false }, &fake);
        for _ in 0..RETRY_CALLS {
            assert_eq!(
                get_state(&mut e, 0, Ptr::new(at)),
                ERROR_DEVICE_NOT_CONNECTED
            );
        }
        assert_eq!(fake.machine().reads, vec![0, 1, 2, 3]);
        fake.machine().states[3] = Some(pressed(0xa3));
        assert_eq!(get_state(&mut e, 0, Ptr::new(at)), ERROR_SUCCESS);
        assert_eq!(fake.machine().reads, vec![0, 1, 2, 3, 0, 1, 2, 3]);
        assert_eq!(e.mem.u8(at + 3), 0xa3);
    }

    /// The rumble goes to the controller the game reads, as the game's 4 bytes.
    #[test]
    fn the_rumble_goes_to_the_seats_controller() {
        let fake = Fake::default();
        fake.machine().states[1] = Some(pressed(0xa1));
        let (mut e, at) = engine(Seat::Pad(0), &fake);
        e.mem.write(at, &[0x34, 0x12, 0xff, 0xff]);
        assert_eq!(set_state(&mut e, 0, Ptr::new(at)), ERROR_SUCCESS);
        assert_eq!(
            set_state(&mut e, 2, Ptr::new(at)),
            ERROR_DEVICE_NOT_CONNECTED
        );
        assert_eq!(fake.machine().sent, vec![(1, [0x34, 0x12, 0xff, 0xff])]);
    }

    /// The capabilities of the game's controller, with the game's flags, 20 bytes as they are.
    #[test]
    fn the_capabilities_are_the_controllers() {
        let fake = Fake::default();
        let caps: [u8; CAPABILITIES_SIZE] = std::array::from_fn(|i| i as u8 + 1);
        {
            let mut m = fake.machine();
            m.states[1] = Some(pressed(0xa1));
            m.capabilities = caps;
        }
        let (mut e, at) = engine(Seat::Any { background: true }, &fake);
        assert_eq!(get_capabilities(&mut e, 0, 1, Ptr::new(at)), ERROR_SUCCESS);
        assert_eq!(e.mem.bytes(at, 20), caps.to_vec());
        assert_eq!(e.mem.u8(at + 20), 0xaa);
        assert_eq!(fake.machine().asked, vec![(1, 1)]);
    }

    /// The default reads only while this process's window is in front, and stops the motors when
    /// it goes to the back; a seat (`NV_PAD` set) reads in the background too.
    #[test]
    fn only_a_seat_reads_in_the_background() {
        let fake = Fake::default();
        {
            let mut m = fake.machine();
            m.states[0] = Some(pressed(0xa0));
            m.in_front = true;
        }
        let (mut e, at) = engine(Seat::Any { background: false }, &fake);
        assert_eq!(get_state(&mut e, 0, Ptr::new(at)), ERROR_SUCCESS);
        let v = e.mem.alloc(4);
        e.mem.write(v, &[0, 0x80, 0, 0x80]);
        assert_eq!(set_state(&mut e, 0, Ptr::new(v)), ERROR_SUCCESS);
        fake.machine().in_front = false;
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
            fake.machine().sent,
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
        fake.machine().states[0] = Some(pressed(0xa0));
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
        let m = fake.machine();
        assert!(m.reads.is_empty() && m.sent.is_empty() && m.asked.is_empty());
    }

    /// The game's imports reach this module: ordinals 2 and 3 through the core's hook, 4 from
    /// this layer's table, at their import slots.
    #[test]
    fn the_games_imports_are_answered_here() {
        let fake = Fake::default();
        fake.machine().states[0] = Some(pressed(0xa0));
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
        assert_eq!(fake.machine().sent, vec![(0, [1, 0, 2, 0])]);
    }
}

//! The controls' buffer: where in the frame the controls are polled, and
//! whether the player's update and the interface's idle see this frame's
//! poll or the previous one (docs/FRAME_SKELETON.md, "PR 8 result").
//!
//! **The poll.** `Main::OnIdle_PollControls` (`0086f390`, frame step 33,
//! called at `0086e88c`) calls `Controls::Poll` (`00a23010`, at `0086f39e`)
//! on the controls' object (`Controls::GetInstance`, `007fdf30`) once a
//! frame. `Controls::Poll` first copies each device's current state over
//! its previous one, then reads each device's state anew (DirectInput's
//! `GetDeviceState`, the device's slot +0x24, after `Acquire`, slot +0x64;
//! a device that is lost reads as nothing): the XInput pad (`011f35a8` to
//! `011f35b8`, then `XInputGetState`), each joystick (`+0x34` to `+0x84`, 0x50
//! bytes each, 0xa0 apart), the keyboard (256 bytes, `+0x18f8` to `+0x19f8`)
//! and the mouse (`DIMOUSESTATE2`'s first 0x14 bytes, `+0x1b24` to
//! `+0x1b38`: the three axes' counts and the buttons) ([`PARTS`]).
//!
//! **The readers.** The controls' queries read only that buffer
//! ([`QUERIES`]): a control (`00a24660`, the control's bound key, mouse
//! button or pad button through the maps at `+0x1b94`) held (state 0), gone
//! down (1: down now, up in the previous copy), come up (2) or changed (3);
//! a key (`00a24180`), a mouse button (`00a23a50`) the same way; the mouse's
//! axes (`00a239e0`: the counts the poll read). Who reads them:
//!
//! - the player's update, `Main::OnIdle_UpdatePlayer` (`0086f940`, step 14):
//!   `PlayerCharacter::Update` (`0093e860`, through the player's slot
//!   +0x2f8), which reads the movement, attack, aim, Activate, Grab, Quick
//!   Save (`0094285d`) and Quick Load (`009403aa`, `00942878`) controls,
//!   or with the free camera `009466d0` (`0086f9c7`, the same Quick Save and
//!   Load at `00947aa5`, `00947ad9`). Step 14 comes before step 33, so the
//!   player reads what the previous frame's poll left ([`Poll::Previous`]).
//! - the interface's idle (`Main::OnIdle_DoInterfaceIdle`, `0086fd70`, its
//!   `InterfaceManager::Idle`, `0070c4a0`): from `Main::OnIdle` before the AI
//!   work with one thread (step 80, `0086eb36`), on the AI linear task
//!   thread as its first call when the AI work runs with more (`008c7be9`,
//!   `008c7db9`; the threads start at `StartThreads`, step 101), or from
//!   `Main::OnIdle` after `Main::OnIdle_UpdateAnimationsAndEffects` when the
//!   AI work doesn't run (step 105, `0086ecba`) ([`super::interface::idle_caller`]).
//!   Every one of these comes after step 33: the interface reads this
//!   frame's poll ([`Poll::This`]).
//!
//! So each poll is met first by the interface (in its frame) and then by
//! the player (in the next frame), and what the interface does to the
//! buffer in between (`Controls::ClearKeystrokes` as the menus take over or
//! the console's key is pressed, `Controls::ClearUserActions` in
//! `Idle` at `0070e9ac` as the Pip-Boy closes) is what the player sees.
//! The poll itself clears the user actions (`0086f43d`) while V.A.T.S. plays
//! back or the Pip-Boy comes up. The game's modal message box loop
//! (`00704010`, used while loading a save's header) polls (`007040fc`) and
//! runs the three idle calls right after it, the same order.
//!
//! **Not the buffer.** Two reads go to the devices themselves: the frame's
//! Tab and Alt test (`GetAsyncKeyState`, `0086e682`, `0086e69a`) before step
//! 1 ends, and the keyboard's queue of key events (DirectInput's
//! `GetDeviceData`, slot +0x28, in `00a23820`), which `InterfaceManager::Idle`
//! drains at `0070dfd8`, `0070e017`, `0070e0c5` and `0070e104` and hands to
//! the console and the frontmost menu (slot +0x30) ([`DEVICE_READS`]).
//!
//! Names are Xbox PDB names (ADR-0002) where the engine map has them; every
//! address is from the disassembly of FalloutNV.exe 1.4.0.525 (the read-only
//! Ghidra server).

// `Controls::Poll`'s copies and reads ([`PARTS`]) are read from its
// decompilation (00a23010, FalloutNV.exe 1.4.0.525); the function itself is
// not translated here.

use super::{ai_stage, interface, step_of, FrameState, STEPS};

/// `Main::OnIdle_PollControls`.
pub const POLL_CONTROLS: u32 = interface::POLL_CONTROLS;
/// `Main::OnIdle`'s call of it.
pub const POLL_CONTROLS_SITE: u32 = 0x0086_e88c;
/// `Controls::Poll`.
pub const CONTROLS_POLL: u32 = 0x00a2_3010;
/// `Main::OnIdle_PollControls`' call of it.
pub const CONTROLS_POLL_SITE: u32 = 0x0086_f39e;
/// `Main::OnIdle_UpdatePlayer`.
pub const UPDATE_PLAYER: u32 = super::player::UPDATE_PLAYER_ADDRESS;
/// `PlayerCharacter::Update`.
pub const PLAYER_UPDATE: u32 = super::player::UPDATE_ADDRESS;
/// The free camera's update with the player's controls (`0086f9c7`).
pub const FREE_CAMERA_INPUT: u32 = 0x0094_66d0;
/// `InterfaceManager::Idle`.
pub const MANAGER_IDLE: u32 = interface::MANAGER_IDLE;

/// One part of the controls' buffer: `Controls::Poll` copies `current`
/// over `previous` (`len` bytes, offsets into the controls' object) and
/// then reads the device into `current`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BufferPart {
    pub device: &'static str,
    pub current: u32,
    pub previous: u32,
    pub len: u32,
}

/// The parts `Controls::Poll` copies and refills, in its order (the
/// joysticks' first; there are `[+0x18f4]` of them, 0xa0 apart). The
/// XInput pad's state is in globals (`011f35a8` to `011f35b8`), not the
/// object.
pub const PARTS: [BufferPart; 3] = [
    BufferPart {
        device: "joystick 0",
        current: 0x34,
        previous: 0x84,
        len: 0x50,
    },
    BufferPart {
        device: "keyboard",
        current: 0x18f8,
        previous: 0x19f8,
        len: 0x100,
    },
    BufferPart {
        device: "mouse",
        current: 0x1b24,
        previous: 0x1b38,
        len: 0x14,
    },
];

/// A query of the buffer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Query {
    pub address: u32,
    pub reads: &'static str,
}

/// The controls' queries; each reads only the buffer.
pub const QUERIES: [Query; 4] = [
    Query {
        address: 0x00a2_4660,
        reads: "a control: its bound key, mouse button or pad button",
    },
    Query {
        address: 0x00a2_4180,
        reads: "a key: +0x18f8 against +0x19f8",
    },
    Query {
        address: 0x00a2_3a50,
        reads: "a mouse button: +0x1b30 against +0x1b44",
    },
    Query {
        address: 0x00a2_39e0,
        reads: "a mouse axis: +0x1b24, +0x1b28, +0x1b2c",
    },
];

/// A read of a device itself, not the buffer: the function and its call
/// sites.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeviceRead {
    pub what: &'static str,
    pub function: u32,
    pub sites: &'static [u32],
}

/// The reads that bypass the buffer.
pub const DEVICE_READS: [DeviceRead; 2] = [
    DeviceRead {
        what: "Tab and Alt (GetAsyncKeyState), the frame's first test",
        function: super::interface::ON_IDLE,
        sites: &[0x0086_e682, 0x0086_e69a],
    },
    DeviceRead {
        what: "the keyboard's key events (GetDeviceData), for the console and the frontmost menu",
        function: MANAGER_IDLE,
        sites: &[0x0070_dfd8, 0x0070_e017, 0x0070_e0c5, 0x0070_e104],
    },
];

/// Which poll a reader sees.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Poll {
    /// The one the previous frame's step 33 made.
    Previous,
    /// The one this frame's step 33 made.
    This,
}

/// A reader of the buffer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Reader {
    /// `Main::OnIdle_UpdatePlayer` (step 14).
    Player,
    /// The interface's idle, wherever this frame calls it.
    InterfaceIdle,
}

/// The index into [`STEPS`] of the poll (step 33).
pub fn poll_step() -> usize {
    step_of(POLL_CONTROLS).expect("Main::OnIdle_PollControls is a step")
}

/// The index into [`STEPS`] through which this frame reaches `reader`:
/// the player's step; for the interface idle, the step that calls it
/// (80 or 105) or, when the AI linear task threads call it, the step that
/// starts them (`StartThreads`). `None` when the frame doesn't reach it
/// (Tab and Alt held).
pub fn reader_step(reader: Reader, frame: &FrameState) -> Option<usize> {
    if frame.alt_tab_held {
        return None;
    }
    match reader {
        Reader::Player => step_of(UPDATE_PLAYER),
        Reader::InterfaceIdle => {
            let caller = interface::idle_caller(frame)?;
            if caller.function == interface::ON_IDLE {
                // The n-th call of the idle from `Main::OnIdle`, by site.
                let n = interface::FUNCTIONS[0]
                    .callers
                    .iter()
                    .filter(|c| c.function == interface::ON_IDLE && c.site < caller.site)
                    .count();
                STEPS
                    .iter()
                    .enumerate()
                    .filter(|(_, s)| s.address == interface::DO_INTERFACE_IDLE)
                    .nth(n)
                    .map(|(i, _)| i)
            } else {
                step_of(ai_stage::START_THREADS)
            }
        }
    }
}

/// Which poll `reader` sees in a frame with these inputs.
pub fn poll_seen(reader: Reader, frame: &FrameState) -> Option<Poll> {
    let at = reader_step(reader, frame)?;
    Some(if at < poll_step() {
        Poll::Previous
    } else {
        Poll::This
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frame::Gate;

    /// `research/engine-map/frame.tsv`, the committed call tree.
    const FRAME_TSV: &str = include_str!("../../../../research/engine-map/frame.tsv");

    /// (depth, address) of each row.
    fn rows() -> Vec<(u32, u32)> {
        FRAME_TSV
            .lines()
            .filter(|l| !l.starts_with('#') && !l.starts_with("seq\t"))
            .map(|l| {
                let f: Vec<&str> = l.split('\t').collect();
                (
                    f[1].parse().unwrap(),
                    u32::from_str_radix(f[3], 16).unwrap(),
                )
            })
            .collect()
    }

    fn frame(threads: i32, menu: bool) -> FrameState {
        FrameState {
            threads,
            in_menu_mode: menu,
            ..FrameState::default()
        }
    }

    /// The poll is step 33, `Main::OnIdle_PollControls`, ungated, and its
    /// call of `Controls::Poll` is the interface model's first sub-step and
    /// `frame.tsv`'s row under it (`0086f39e`).
    #[test]
    fn the_poll_is_step_33() {
        let i = poll_step();
        assert_eq!(i + 1, 33);
        assert_eq!(STEPS[i].gate, Gate::Always);
        assert_eq!(STEPS[i].name, Some("Main::OnIdle_PollControls"));
        let f = interface::function(POLL_CONTROLS).expect("modelled");
        assert_eq!(f.callers.len(), 1);
        assert_eq!(f.callers[0].function, interface::ON_IDLE);
        assert_eq!(f.callers[0].site, POLL_CONTROLS_SITE);
        let k = interface::step_at(f, CONTROLS_POLL_SITE).expect("the poll's sub-step");
        assert_eq!(k, 0);
        assert_eq!(f.steps[0].name, Some("Controls::Poll"));
        assert!(matches!(
            f.steps[0].callee,
            interface::Callee::Direct(CONTROLS_POLL)
        ));
        let rows = rows();
        let p = rows
            .iter()
            .position(|&r| r == (1, POLL_CONTROLS))
            .expect("frame.tsv's row");
        let under: Vec<u32> = rows[p + 1..]
            .iter()
            .take_while(|r| r.0 > 1)
            .filter(|r| r.0 == 2)
            .map(|r| r.1)
            .collect();
        assert!(under.contains(&CONTROLS_POLL));
        // Only once a frame: no other step calls it.
        assert_eq!(
            STEPS.iter().filter(|s| s.address == POLL_CONTROLS).count(),
            1
        );
    }

    /// The player's step (14) comes before the poll, so the player reads
    /// the previous frame's poll, in every mode the frame reaches it.
    #[test]
    fn the_player_reads_the_previous_poll() {
        assert_eq!(step_of(UPDATE_PLAYER).map(|i| i + 1), Some(14));
        for threads in [1, 2, 8] {
            for menu in [false, true] {
                assert_eq!(
                    poll_seen(Reader::Player, &frame(threads, menu)),
                    Some(Poll::Previous)
                );
            }
        }
        // `PlayerCharacter::Update` and the free camera's input are the
        // player step's calls (slot +0x2f8 and `0086f9c7`).
        use crate::frame::player;
        assert!(player::UPDATE_PLAYER.iter().any(
            |c| c.site == 0x0086_f9c7 && c.callee == player::Callee::Direct(FREE_CAMERA_INPUT)
        ));
        assert!(player::UPDATE_PLAYER
            .iter()
            .any(|c| c.callee == player::Callee::Player(0x2f8)));
    }

    /// Every way the frame calls the interface idle comes after the poll:
    /// step 80 with one thread, `StartThreads` (step 101) when the AI threads
    /// run, step 105 when they don't (menu mode).
    #[test]
    fn the_interface_reads_this_frames_poll() {
        let step = |f: &FrameState| reader_step(Reader::InterfaceIdle, f).map(|i| i + 1);
        assert_eq!(step(&frame(1, false)), Some(80));
        assert_eq!(step(&frame(1, true)), Some(80));
        assert_eq!(step(&frame(2, false)), Some(101));
        assert_eq!(step(&frame(8, false)), Some(101));
        assert_eq!(step(&frame(2, true)), Some(105));
        assert_eq!(step(&frame(8, true)), Some(105));
        assert_eq!(
            STEPS[100].address,
            ai_stage::START_THREADS,
            "step 101 starts the AI linear task threads"
        );
        for threads in [1, 2, 8] {
            for menu in [false, true] {
                assert_eq!(
                    poll_seen(Reader::InterfaceIdle, &frame(threads, menu)),
                    Some(Poll::This)
                );
            }
        }
        // The idle's four callers, each placed after the poll: the two
        // `Main::OnIdle` sites are steps 80 and 105, in site order.
        let callers = interface::FUNCTIONS[0].callers;
        assert_eq!(callers.len(), 4);
        let sites: Vec<u32> = callers
            .iter()
            .filter(|c| c.function == interface::ON_IDLE)
            .map(|c| c.site)
            .collect();
        assert_eq!(sites, [0x0086_eb36, 0x0086_ecba]);
        assert!(sites.iter().all(|&s| s > POLL_CONTROLS_SITE));
        let threads: Vec<u32> = callers
            .iter()
            .filter(|c| c.function != interface::ON_IDLE)
            .map(|c| c.function)
            .collect();
        assert_eq!(threads, [interface::COMBINED_THREAD, interface::THREAD_1]);
    }

    /// Tab and Alt held: the frame returns after step 1, before both
    /// readers and the poll.
    #[test]
    fn tab_and_alt_reach_neither() {
        let held = FrameState {
            alt_tab_held: true,
            ..frame(8, false)
        };
        assert_eq!(poll_seen(Reader::Player, &held), None);
        assert_eq!(poll_seen(Reader::InterfaceIdle, &held), None);
        assert!(!crate::frame::step_runs(&held, poll_step()));
    }

    /// The buffer's parts don't overlap, each previous copy follows its
    /// current one, and the device reads named are inside their functions'
    /// call ranges.
    #[test]
    fn the_buffer_and_the_device_reads() {
        for p in &PARTS {
            assert_eq!(p.previous, p.current + p.len, "{}", p.device);
        }
        for w in PARTS.windows(2) {
            assert!(w[0].previous + w[0].len <= w[1].current);
        }
        assert_eq!(PARTS[1].len, 256, "DirectInput's keyboard state");
        // The frame's Tab and Alt test is before the poll's call; the key
        // events are drained inside `InterfaceManager::Idle`.
        assert!(DEVICE_READS[0]
            .sites
            .iter()
            .all(|&s| (interface::ON_IDLE..POLL_CONTROLS_SITE).contains(&s)));
        let idle = interface::function(MANAGER_IDLE).expect("modelled");
        assert!(DEVICE_READS[1].sites.iter().all(|&s| idle.contains(s)));
        let mut queries: Vec<u32> = QUERIES.iter().map(|q| q.address).collect();
        queries.dedup();
        assert_eq!(queries.len(), QUERIES.len());
    }
}

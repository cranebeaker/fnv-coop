//! `NV_FRAME_LOG=FILE`: one line for each of main's frames, a measuring aid (docs/FIXES.md): the
//! microsecond clock, the main timer's step, the Havok accumulator, the player's position and
//! whether the frame shows play, a menu or a loading screen (`uncap::screen`).
//! It reads guest memory between frames, so unlike a trace it sends no call down the slow path and
//! the frame rate stays the player's. Off, nothing runs.

use engine::Engine;
use std::io::Write;

/// The main timer's `fDelta`.
const MAIN_TIMER_DELTA: u32 = 0x011f_6394 + 0xc;
/// The Havok step count and the time the frame's marker moves by.
const STEP_COUNT: u32 = 0x0126_77b4;
const STEP_TIME: u32 = 0x011a_fe64;
/// The player (`PlayerCharacter`), and where a reference keeps its position.
const PLAYER: u32 = 0x011d_ea3c;
const POSITION: u32 = 0x30;

/// The open log.
pub struct FrameLog {
    out: std::io::BufWriter<std::fs::File>,
    frame: u64,
}

impl FrameLog {
    /// The log `NV_FRAME_LOG` names, if any. A file that can't be made is said once and no log.
    pub fn from_env() -> Option<FrameLog> {
        let path = std::env::var_os("NV_FRAME_LOG")?;
        let file = match std::fs::File::create(&path) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("NV_FRAME_LOG: {e}");
                return None;
            }
        };
        let mut out = std::io::BufWriter::new(file);
        let _ = writeln!(
            out,
            "frame\tus\tm_fdelta\thk_count\thk_time\tpx\tpy\tpz\tscreen"
        );
        Some(FrameLog { out, frame: 0 })
    }

    /// One frame's line.
    pub fn record(&mut self, e: &mut Engine) {
        let us = e.clock_now_us();
        let delta = e.mem.f32(MAIN_TIMER_DELTA);
        let count = e.mem.u32(STEP_COUNT);
        let time = e.mem.f32(STEP_TIME);
        let player = e.mem.u32(PLAYER);
        let at = if player == 0 {
            [0.0; 3]
        } else {
            [0, 4, 8].map(|o| e.mem.f32(player + POSITION + o))
        };
        let _ = writeln!(
            self.out,
            "{}\t{us}\t{delta}\t{count}\t{time}\t{}\t{}\t{}\t{}",
            self.frame,
            at[0],
            at[1],
            at[2],
            crate::uncap::screen(e).name()
        );
        self.frame += 1;
    }
}

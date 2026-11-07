//! The frame-timing fixes of the `fixes` preset ([`crate::fixes`], docs/FIXES.md): **not 1:1 with
//! FalloutNV.exe**, and in the `vanilla` preset nothing here runs.
//!
//! **The uncapped frame rate (`NV_UNCAP`).** New Vegas Tick Fix (carxt, lStewieAl, jazzisparis) showed the
//! way; this is its idea in nv-rs's own code ([docs/FIXES.md](../../../docs/FIXES.md) has the credits).
//!
//! The game paces its frames with `BSTimer`'s step (`00aa4ee0`, engine-bs `bscore/bstimer.rs`):
//! for a real-time timer (`fClamp` 0) a step that comes less than 10 ms (by `GetTickCount`)
//! after the timer's last one sleeps the difference, and every step reports at least 10 ms.
//! In game three timers step each frame (Main::OnIdle's through `00771520`, `HUDMainMenu::
//! DoIdle`'s and `MessageMenu::DoIdle`'s), so with W the frame's other work the frame takes
//! (W + 30) / 4 ms: at most 133 frames a second however fast the rest is (a29's profile,
//! 2026-10-28). The original game is held the same way.
//!
//! With the switch, the step at `00aa4ee0` is replaced (registered over the translation by
//! [`install`], after the play-loop patches): a real-time step neither sleeps nor rounds up to
//! 10 ms. It takes the time since that timer's own last step from the platform's microsecond
//! clock (what `QueryPerformanceCounter` answers), not from `GetTickCount`'s 15.625 ms ticks,
//! so the game's time keeps pace with the real time however short the frames are. The rest is
//! the translation's: a disabled timer and a fixed step (`fClamp`) run the translation itself;
//! the step is at most 166 ms; `iLastTime` moves on as before; `fDeltaUnmodified` and the global
//! time multiplier as before. A timer's first step (or its first after being disabled or
//! clamped) answers as the translation does from the tick count's milliseconds (at least 10, at most 166)
//! without the sleep. Where the translation sleeps, the other threads get a `Sleep(0)` pass.
//!
//! The game was made for steps of 10 ms and more; shorter ones are what mods like NVTF give it.
//!
//! **The high-frame-rate landing (`NV_HF_LANDING`).** A character's contact list reaches about a
//! Havok unit (7 game units) below it. The first frame with the floor in it, the controller says
//! supported and goes to the ground state, and that frame moves it once more at its falling speed;
//! the next frame, standing still on the ground, it has no vertical speed and its contact is
//! unchanged, so `Move` stops integrating it for good. At 16 ms frames a falling character covers
//! those units in a frame or two, so it ends within about a unit of the floor; at 5 ms it stays
//! 7 units up (measured, docs/FIXES.md). The override runs `00c73170` and, on such a landing of the player's character (the only one measured) in a frame
//! under 16 ms, integrates once more for the rest of two 16 ms frames. Other frames run `Move` as
//! it is, and the world's own Havok steps are not touched: they are 16 ms at any frame rate.
//!
//! **The menu frame limit (`NV_MENU_CAP`).** Uncapped, a menu runs at 170 frames a second and more
//! for nothing. With this row the uncapped step paces the main timer by what the frame shows, the
//! way the game itself tells (`Main::OnIdle`, `0086e650`): a menu is the menu-mode byte `011dea2b`
//! the frame set at its start (`Interface::IsInMenuMode` or the Pip-Boy opening); loading is the
//! loading menu up (`LoadingMenu::pMe`) with neither the main menu (`XUIIsUp`) nor the credits
//! (`0086efa0`) over it, the frames in which `OnIdle` asks whether the load is done. A menu runs at
//! most `MenuFrameLimit` frames a second (120 unless set, 0 for none; the player's `FrameLimit`
//! when lower), a loading screen with no limit at all, and play at the player's `FrameLimit`. The
//! row registers nothing of its own: it rides on the uncapped step, so with that off the original's
//! own pacing runs and this does nothing.
//!
//! **Dragged bodies (`NV_GRAB_PUSH`).** The grab update (`00960520`, run once a rendered frame by
//! the player's update, the dropped-object spring and telekinesis) gives a held body that is more
//! than `fZKeyComplexHelperMinDistance` (6) units from where it should be a velocity of
//! `(distance - 6) x fZKeyComplexHelperScale (2) / distance` times -1.0 times a weight factor, added
//! to the body's velocity once a frame: at 144 frames a second it is pulled 2.4 times as hard as at
//! 60. The override runs the grab update's three entries with `fZKeyComplexHelperScale` scaled by
//! the frame's seconds times 60, at most 4 (TEMPO's rule in Fallout 3), and gives the setting back.
//! The grab update is the setting's only reader.
//!

use crate::fixes::Fix;
use engine::prelude::*;
use engine::units::bscore::bstimer::{fn_00aa4e40, fn_00aa4ee0, BSTimer};
use engine::units::bshavok::bhkcharactercontroller::{
    bhk_character_controller_move, bhk_character_proxy_integrate,
    BhkCharacterController as Controller,
};
use engine::units::fallout_ai::playercharacter_p3::{
    fn_0095f6c0, fn_00960520, player_character_update_drop_spring,
};
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};

/// `BSTimer`'s step (the translation: engine-bs `bstimer.rs`).
pub const BS_TIMER_STEP: u32 = 0x00aa_4ee0;
/// `Sleep` (KERNEL32 import slot).
const SLEEP_IMPORT: u32 = 0x00fd_f104;
/// The main timer (`TimeGlobal`), the one `Main::OnIdle` steps at `0086f296`.
const MAIN_TIMER: u32 = 0x011f_6394;
/// `0.0` (`double`), the value the translation compares `fClamp` with.
const ZERO: u32 = 0x0101_2060;
/// `BSTimer::fGlobalTimeMultiplier` (Xbox PDB static `float`).
const GLOBAL_TIME_MULTIPLIER: u32 = 0x011a_c3a0;
/// The longest step the translation takes (`0xa6` ms), in microseconds.
const LONGEST_STEP_US: u64 = 166_000;

/// The row in [`crate::fixes::FIXES`].
pub const FIX: Fix = Fix {
    name: "uncapped frame rate",
    env: "NV_UNCAP",
    addresses: &[BS_TIMER_STEP],
    summary: "BSTimer's step without its sleep, timed on the microsecond clock",
    install,
};

/// Registers the uncapped step over `00aa4ee0`.
pub fn install(e: &mut Engine) {
    e.register(BS_TIMER_STEP, uncapped_entry);
}

fn uncapped_entry(e: &mut Engine, a: &[u32]) -> Ret {
    uncapped_step(e, Ptr::new(a[0]), a[1]);
    Ret::default()
}

/// Frames a second at most (0: no limit), bound once before the first frame
/// ([`crate::fixes::install`]). A run has one host.
static FRAME_LIMIT: AtomicU32 = AtomicU32::new(0);
/// The menu frame limit when [`MENU_FIX`] is on (0: none), [`MENU_LIMIT_OFF`] when it is off.
static MENU_FRAME_LIMIT: AtomicU32 = AtomicU32::new(MENU_LIMIT_OFF);
const MENU_LIMIT_OFF: u32 = u32::MAX;
/// When the next frame may start, on the microsecond clock.
static NEXT_FRAME_US: AtomicU64 = AtomicU64::new(0);

/// Binds the frame limit.
pub fn set_frame_limit(frames_a_second: u32) {
    FRAME_LIMIT.store(frames_a_second, Ordering::Relaxed);
}

/// Binds the menu frame limit: `None` when [`MENU_FIX`] is off, else frames a second (0: none).
pub fn set_menu_frame_limit(frames_a_second: Option<u32>) {
    MENU_FRAME_LIMIT.store(
        frames_a_second.map_or(MENU_LIMIT_OFF, |n| n.min(MENU_LIMIT_OFF - 1)),
        Ordering::Relaxed,
    );
}

/// The row in [`crate::fixes::FIXES`].
pub const MENU_FIX: Fix = Fix {
    name: "menu frame limit",
    env: "NV_MENU_CAP",
    addresses: &[],
    summary: "menus at most MenuFrameLimit frames a second, loading screens unlimited",
    install: install_menu_limit,
};

/// Registers nothing: [`crate::fixes::install`] binds the limit, which the uncapped step reads.
pub fn install_menu_limit(_: &mut Engine) {}

/// What a frame of the main loop shows, as the game tells (module documentation).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Screen {
    Play,
    Menu,
    Loading,
}

impl Screen {
    /// Its name in the frame log.
    pub fn name(self) -> &'static str {
        match self {
            Screen::Play => "play",
            Screen::Menu => "menu",
            Screen::Loading => "loading",
        }
    }
}

/// The menu-mode byte `Main::OnIdle` sets at the start of each frame.
const MENU_MODE: u32 = 0x011d_ea2b;
/// `LoadingMenu::pMe`.
const LOADING_MENU: u32 = 0x011d_a0c0;
/// `StartMenu::pMe` and its flag word: `XUIIsUp` (`0070edf0`) is the menu there without bit 0 (the
/// main menu, not the pause menu), `0086efa0` the menu with bit `0x10000` (the credits).
const START_MENU: u32 = 0x011d_aac0;
const START_MENU_FLAGS: u32 = 0x1a8;
const START_MENU_IN_GAME: u32 = 1;
const START_MENU_CREDITS: u32 = 0x1_0000;

/// What this frame shows: `OnIdle`'s own tests, read from memory (`0086e89c` to `0086e8d3` for
/// loading, `011dea2b` for a menu).
pub fn screen(e: &Engine) -> Screen {
    if e.mem.u32(LOADING_MENU) != 0 {
        let start = e.mem.u32(START_MENU);
        let flags = if start == 0 {
            0
        } else {
            e.mem.u32(start + START_MENU_FLAGS)
        };
        let credits = start != 0 && flags & START_MENU_CREDITS != 0;
        let main_menu = start != 0 && flags & START_MENU_IN_GAME == 0;
        if !credits && !main_menu {
            return Screen::Loading;
        }
    }
    if e.mem.u8(MENU_MODE) != 0 {
        Screen::Menu
    } else {
        Screen::Play
    }
}

/// Frames a second at most for a frame showing `screen` (0: no limit): the player's limit in play,
/// the lower of the two in a menu (either 0 is no limit of its own), none while loading.
pub fn frame_limit_for(screen: Screen, player: u32, menu: u32) -> u32 {
    match screen {
        Screen::Play => player,
        Screen::Loading => 0,
        Screen::Menu => match (player, menu) {
            (0, menu) => menu,
            (player, 0) => player,
            (player, menu) => player.min(menu),
        },
    }
}

/// The main timer's limit this frame.
fn main_frame_limit(e: &Engine) -> u32 {
    let menu = match MENU_FRAME_LIMIT.load(Ordering::Relaxed) {
        MENU_LIMIT_OFF => None,
        menu => Some(menu),
    };
    limit_this_frame(e, FRAME_LIMIT.load(Ordering::Relaxed), menu)
}

/// The limit for this frame with the player's limit and the menu limit (`None`: the row is off, and
/// the player's limit holds everywhere, as before it).
fn limit_this_frame(e: &Engine, player: u32, menu: Option<u32>) -> u32 {
    match menu {
        None => player,
        Some(menu) => frame_limit_for(screen(e), player, menu),
    }
}

/// The player's frame limit, at the main timer's step: gives the other game threads their turn
/// first (`Sleep(0)`), since the wait that follows is theirs to use (the original's threads run
/// side by side), then waits for the frame's start time. That time moves by whole frames, so a
/// late wake-up doesn't lower the rate.
fn wait_for_frame_limit(e: &mut Engine, limit: u32) {
    let frame_us = 1_000_000 / u64::from(limit);
    e.call(SLEEP_IMPORT, &args![0u32]);
    let now_us = e.clock_now_us();
    let next = NEXT_FRAME_US.load(Ordering::Relaxed);
    // A start time more than a frame ahead means the clock's `DWORD` wrapped (every 49.7 days of
    // uptime): start the count again rather than wait for the old time.
    let next = if next > now_us + frame_us {
        now_us
    } else {
        next
    };
    if next > now_us {
        e.wait_clock_until_us(next);
    }
    // More than a whole frame late starts the count again, without catching up.
    let start = if next + frame_us < now_us {
        now_us
    } else {
        next
    };
    NEXT_FRAME_US.store(start + frame_us, Ordering::Relaxed);
}

/// The five padding bytes of a timer's guest object (`+1` to `+3`, `+0x1e`, `+0x1f`), which no exe
/// code reads (no instruction names them, and the timers are only read and written by field),
/// hold the time of its last real-time step: 0 for none, else 1 plus the clock's milliseconds
/// (the low 30 bits) times 1024 plus the microsecond part. Only the difference of two readings is
/// used, taken modulo 2^30 ms (12.4 days; 2^30 divides the clock's own `DWORD` wrap), so it does
/// not matter how long the machine has been up.
const STEP_MS_BITS: u32 = 30;
const STEP_MS_MASK: u64 = (1 << STEP_MS_BITS) - 1;

fn last_step_raw(e: &Engine, this: Ptr<BSTimer>) -> u64 {
    let at = this.addr();
    [1, 2, 3, 0x1e, 0x1f]
        .iter()
        .rev()
        .fold(0, |word, &o| (word << 8) | u64::from(e.mem.u8(at + o)))
}

fn set_last_step_raw(e: &mut Engine, this: Ptr<BSTimer>, raw: u64) {
    let at = this.addr();
    let mut word = raw;
    for o in [1, 2, 3, 0x1e, 0x1f] {
        // The low byte of what's left.
        e.mem.set_u8(at + o, word.to_le_bytes()[0]);
        word >>= 8;
    }
}

/// The stored last step as a clock reading in microseconds modulo 2^30 ms, if there is one.
fn last_step_us(e: &Engine, this: Ptr<BSTimer>) -> Option<u64> {
    let raw = last_step_raw(e, this);
    (raw != 0).then(|| {
        let kept = raw - 1;
        (kept >> 10) * 1000 + (kept & 0x3ff)
    })
}

fn set_last_step_us(e: &mut Engine, this: Ptr<BSTimer>, us: Option<u64>) {
    let raw = us.map_or(0, |us| {
        let ms = (us / 1000) & STEP_MS_MASK;
        1 + ((ms << 10) | (us % 1000))
    });
    set_last_step_raw(e, this, raw);
}

/// Microseconds from `before` to `now` (both clock readings, `before` as [`last_step_us`] keeps it).
fn step_since(now_us: u64, before: u64) -> u64 {
    let ms = ((now_us / 1000) & STEP_MS_MASK).wrapping_sub(before / 1000) & STEP_MS_MASK;
    (ms * 1000 + now_us % 1000).saturating_sub(before % 1000)
}

/// `00aa4ee0` without the sleep (module documentation).
pub fn uncapped_step(e: &mut Engine, this: Ptr<BSTimer>, now: u32) {
    let zero: f64 = e.global(ZERO);
    let real_time = e.get(this, BSTimer::cDisableCounter) == 0
        && f64::from(e.get(this, BSTimer::fClamp)) == zero;
    if !real_time {
        set_last_step_us(e, this, None);
        fn_00aa4ee0(e, this, now);
        return;
    }
    let limit = if this.addr() == MAIN_TIMER {
        main_frame_limit(e)
    } else {
        0
    };
    let paced = limit != 0;
    if paced {
        wait_for_frame_limit(e, limit);
    }
    let now_us = e.clock_now_us();
    let time = now.wrapping_sub(e.get(this, BSTimer::iFirstTime));
    let elapsed_ms = time.wrapping_sub(e.get(this, BSTimer::iLastTime));
    // Where the original sleeps (a step under 10 ms by the tick count) the other threads get a
    // pass, without a real sleep: about one frame in three at the original's pace.
    if elapsed_ms < 10 && !paced {
        e.call(SLEEP_IMPORT, &args![0u32]);
    }
    let before = last_step_us(e, this);
    set_last_step_us(e, this, Some(now_us));
    let elapsed_us = match before {
        Some(before) => step_since(now_us, before).min(LONGEST_STEP_US),
        // No step of ours yet (the first, or the first after a disable or a fixed step): the
        // tick count's milliseconds, as the original answers: at most 166, at least 10 ms, and
        // none passed reads as a zero delta.
        None => u64::from(elapsed_ms.clamp(10, 166)) * 1000,
    };
    let zero_delta = match before {
        Some(_) => elapsed_us == 0,
        None => elapsed_ms == 0,
    };
    e.set(this, BSTimer::bZeroDelta, zero_delta);
    e.set(this, BSTimer::fDelta, (elapsed_us as f64 / 1e6) as f32);
    e.set(this, BSTimer::iLastTime, time);
    // From here as the translation.
    let delta = e.get(this, BSTimer::fDelta);
    e.set(
        this,
        BSTimer::fDeltaUnmodified,
        f32::from_bits(x87_quiet_bits(delta.to_bits())),
    );
    let multiplier: u32 = e.global(GLOBAL_TIME_MULTIPLIER);
    let scaled = x87_mul(x87_load32(delta.to_bits()), x87_m32(multiplier));
    e.set(this, BSTimer::fDelta, f32::from_bits(x87_store32(scaled)));
}

/// The row in [`crate::fixes::FIXES`].
pub const PHYSICS_FIX: Fix = Fix {
    name: "high-frame-rate landing",
    env: "NV_HF_LANDING",
    addresses: &[CHARACTER_MOVE],
    summary: "a character that lands standing still falls on as far as at 16 ms frames",
    install: install_landing,
};

/// `bhkCharacterController::Move`.
const CHARACTER_MOVE: u32 = 0x00c7_3170;
/// `hkpCharacterProxy::setLinearVelocity`.
const SET_LINEAR_VELOCITY: u32 = 0x00ca_c960;
/// The controller's states (`hkpCharacterContext::m_currentState`).
const STATE_ON_GROUND: u32 = 0;
const STATE_IN_AIR: u32 = 2;
/// `iFlags` bit: the controller was asked to move this step.
const FLAG_MOVING: u32 = 8;
/// The frame the game's character code settles to: longer frames land as they always did.
const SETTLED_FRAME_S: f32 = 0.016;

/// Registers the landing over `00c73170`.
pub fn install_landing(e: &mut Engine) {
    e.register(CHARACTER_MOVE, landing_entry);
}

fn landing_entry(e: &mut Engine, a: &[u32]) -> Ret {
    landing_move(e, Ptr::new(a[0]), Ptr::new(a[1])).into_ret()
}

/// How much longer a landing character falls, in seconds: the rest of two 16 ms frames, when it
/// was in the air falling, is now on the ground, wasn't asked to move and this frame was short.
fn landing_time(step: f32, before: u32, after: u32, moving: bool, fall_speed: f32) -> Option<f32> {
    let lands = before == STATE_IN_AIR && after == STATE_ON_GROUND && !moving && fall_speed < 0.0;
    (lands && step > 0.0 && step < SETTLED_FRAME_S).then_some(2.0 * SETTLED_FRAME_S - step)
}

/// `PlayerCharacter` (the global's pointer) and `MobileObject::GetCharController`.
const PLAYER: u32 = 0x011d_ea3c;
const GET_CHAR_CONTROLLER: u32 = 0x0093_06d0;
/// `iFlags` bit 20: the controller is a large creature's (`Move` hands `Integrate` 1 instead of 4
/// at the proxy's `+0xac` then).
const FLAG_LARGE: u32 = 1 << 20;
/// The proxy's integration mode word, which `Move` sets around its `Integrate` call.
const PROXY_MODE: u32 = 0xac;

/// Whether `this` is the player's controller. The landing was measured on the player's: other
/// characters' landings have no measurement, so they run `Move` as it is.
fn is_player_controller(e: &mut Engine, this: Ptr<Controller>) -> bool {
    let player: u32 = e.global(PLAYER);
    player != 0 && e.call(GET_CHAR_CONTROLLER, &args![player]).u32() == this.addr()
}

/// `00c73170`, then the landing (module documentation).
fn landing_move(e: &mut Engine, this: Ptr<Controller>, params: Ptr) -> u32 {
    let step = e.mem.f32(params.addr());
    let before = e.get(this, Controller::m_currentState);
    let velocity_at = this.addr() + Controller::OutVelocity.off;
    let fall_speed = e.mem.f32(velocity_at + 8);
    let result = bhk_character_controller_move(e, this, params);
    let after = e.get(this, Controller::m_currentState);
    let moving = e.get(this, Controller::iFlags) & FLAG_MOVING != 0;
    if let Some(time) = landing_time(step, before, after, moving, fall_speed) {
        if is_player_controller(e, this) {
            fall_on(e, this, fall_speed, time);
        }
    }
    result
}

/// One more integration of the character's proxy, the way `Move` does its own: the velocity
/// `Move` gave it (a surface's sideways motion included) with the vertical part the falling speed,
/// the proxy's mode word 1 for a large creature and 4 otherwise, the step as given. It stops at the
/// surface as any other does. Afterwards the step, the mode word and both velocities (the
/// controller's and the proxy's) are as `Move` left them. `Move` writes the step info's delta only;
/// its start and end times stay as they are, as here.
fn fall_on(e: &mut Engine, this: Ptr<Controller>, speed: f32, time: f32) {
    let me = this.addr();
    let proxy = e.get(this, Controller::phkObject);
    if proxy == 0 {
        return;
    }
    let velocity_at = me + Controller::OutVelocity.off;
    let kept = [0, 4, 8, 12].map(|o| e.mem.u32(velocity_at + o));
    let step = e.get(this, Controller::StepDeltaTime);
    let inverse = e.get(this, Controller::StepInvDeltaTime);
    let large = e.get(this, Controller::iFlags) & FLAG_LARGE != 0;
    e.mem.set_u32(velocity_at + 8, speed.to_bits());
    e.call(SET_LINEAR_VELOCITY, &args![proxy, velocity_at]);
    e.set(this, Controller::StepDeltaTime, time);
    e.set(this, Controller::StepInvDeltaTime, 1.0 / time);
    e.mem
        .set_u32(proxy.wrapping_add(PROXY_MODE), if large { 1 } else { 4 });
    bhk_character_proxy_integrate(e, this.cast(), me + Controller::StepInfo.off);
    e.mem.set_u32(proxy.wrapping_add(PROXY_MODE), 4);
    e.set(this, Controller::StepDeltaTime, step);
    e.set(this, Controller::StepInvDeltaTime, inverse);
    for (o, word) in [0, 4, 8, 12].into_iter().zip(kept) {
        e.mem.set_u32(velocity_at + o, word);
    }
    e.call(SET_LINEAR_VELOCITY, &args![proxy, velocity_at]);
}

/// The row in [`crate::fixes::FIXES`].
pub const SLOW_MOTION_FIX: Fix = Fix {
    name: "high-frame-rate slow motion",
    env: "NV_HF_SLOWMO",
    addresses: &[TIME_MULTIPLIER_RAMP],
    summary: "the time multiplier moves to its target by the time passed, not by the frame",
    install: install_slow_motion,
};

/// `00aa4e40`: moves `fGlobalTimeMultiplier` towards its target once a frame.
const TIME_MULTIPLIER_RAMP: u32 = 0x00aa_4e40;
/// `fGlobalTimeMultiplierTarget` and the rate of the move.
const TIME_MULTIPLIER_TARGET: u32 = 0x011a_c3a4;
const TIME_MULTIPLIER_RATE: u32 = 0x011a_c3ac;

/// Registers the ramp over `00aa4e40`.
pub fn install_slow_motion(e: &mut Engine) {
    e.register(TIME_MULTIPLIER_RAMP, ramp_entry);
}

fn ramp_entry(e: &mut Engine, _: &[u32]) -> Ret {
    time_multiplier_ramp(e);
    Ret::default()
}

/// `00aa4e40` with the rate of a 16 ms frame: in the last frame's share of 16 ms the multiplier
/// moves as far as the exe moves it in a whole one, so V.A.T.S.'s slow motion comes on in the same
/// time at any frame rate. Frames of 16 ms and more run the translation.
fn time_multiplier_ramp(e: &mut Engine) {
    let last_step = e.mem.f32(MAIN_TIMER + BSTimer::fDeltaUnmodified.off);
    if !(last_step > 0.0 && last_step < SETTLED_FRAME_S) {
        fn_00aa4e40(e);
        return;
    }
    let share = f64::from(last_step) / f64::from(SETTLED_FRAME_S);
    let rate = f64::from(e.mem.f32(TIME_MULTIPLIER_RATE));
    let target = e.mem.f32(TIME_MULTIPLIER_TARGET);
    let multiplier = e.mem.f32(GLOBAL_TIME_MULTIPLIER);
    let moved = if target > multiplier {
        ((f64::from(multiplier) * (1.0 + rate).powf(share)) as f32).min(target)
    } else if target < multiplier {
        ((f64::from(multiplier) * (1.0 - rate).powf(share)) as f32).max(target)
    } else {
        return;
    };
    e.mem.set_f32(GLOBAL_TIME_MULTIPLIER, moved);
}

/// The row in [`crate::fixes::FIXES`].
pub const GRAB_FIX: Fix = Fix {
    name: "dragged bodies by time",
    env: "NV_GRAB_PUSH",
    addresses: &[PLAYER_GRAB_UPDATE, DROP_SPRING_UPDATE, GRAB_UPDATE],
    summary: "a held body's per-frame pull scaled by the frame's time, as at 60 frames a second",
    install: install_grab,
};

/// The player's grab handling (`0095f6c0`, `PlayerCharacter::Update` once a frame), the dropped-object
/// spring (`PlayerCharacter::UpdateDropSpring`) and the grab update itself, which telekinesis
/// (`TelekinesisEffect::Update`) calls: the three entries to the pull, each given the frame's seconds.
/// The first two call the grab update in Rust, so each is wrapped.
const PLAYER_GRAB_UPDATE: u32 = 0x0095_f6c0;
const DROP_SPRING_UPDATE: u32 = 0x0096_13c0;
const GRAB_UPDATE: u32 = 0x0096_0520;
/// `fZKeyComplexHelperScale`'s value (the setting object at `011d0808`, its float at `+4`): read
/// only by the grab update's pull (`00960e2a`).
const HELPER_SCALE: u32 = 0x011d_0808 + 4;
/// The frame the pull was made for, and TEMPO's cap on the scale.
const GRAB_FRAMES_A_SECOND: f32 = 60.0;
const MOST_GRAB_SCALE: f32 = 4.0;
thread_local! {
    /// Entries under way on this thread: a nested one (none today) keeps the outer one's scale.
    static GRAB_DEPTH: std::cell::Cell<u32> = const { std::cell::Cell::new(0) };
}

/// Registers the three entries.
pub fn install_grab(e: &mut Engine) {
    e.register(PLAYER_GRAB_UPDATE, |e, a| {
        let seconds = f32::from_bits(a[1]);
        with_grab_scale(e, seconds, |e| fn_0095f6c0(e, Ptr::new(a[0]), seconds));
        Ret::default()
    });
    e.register(DROP_SPRING_UPDATE, |e, a| {
        let seconds = f32::from_bits(a[1]);
        with_grab_scale(e, seconds, |e| {
            player_character_update_drop_spring(e, Ptr::new(a[0]), seconds)
        })
        .into_ret()
    });
    e.register(GRAB_UPDATE, |e, a| {
        let seconds = f32::from_bits(a[1]);
        with_grab_scale(e, seconds, |e| fn_00960520(e, Ptr::new(a[0]), seconds));
        Ret::default()
    });
}

/// How much of the 60-frames-a-second pull this frame gets: its seconds times 60, at most 4; a time
/// that isn't a number keeps the pull as it is.
fn grab_scale(seconds: f32) -> f32 {
    if seconds.is_nan() {
        return 1.0;
    }
    (seconds * GRAB_FRAMES_A_SECOND).clamp(0.0, MOST_GRAB_SCALE)
}

/// Runs `run` with `fZKeyComplexHelperScale` scaled for this frame, then gives the setting back
/// (unless something set it meanwhile), also when a C++ throw unwinds through.
fn with_grab_scale<R>(e: &mut Engine, seconds: f32, run: impl FnOnce(&mut Engine) -> R) -> R {
    let outer = GRAB_DEPTH.with(|d| d.replace(d.get() + 1)) == 0;
    let kept = outer.then(|| {
        let base = e.mem.f32(HELPER_SCALE);
        let scaled = (f64::from(base) * f64::from(grab_scale(seconds))) as f32;
        e.mem.set_f32(HELPER_SCALE, scaled);
        (base, scaled)
    });
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| run(&mut *e)));
    if let Some((base, scaled)) = kept {
        if e.mem.f32(HELPER_SCALE).to_bits() == scaled.to_bits() {
            e.mem.set_f32(HELPER_SCALE, base);
        }
    }
    GRAB_DEPTH.with(|d| d.set(d.get() - 1));
    result.unwrap_or_else(|p| std::panic::resume_unwind(p))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SLEEP: u32 = 0x00fd_f104;
    const MILLISECONDS_TO_SECONDS: u32 = 0x0101_6978;

    /// An engine with the globals the step reads, a `Sleep` that moves the clock and logs its
    /// time, and a real-time timer whose steps start at clock 1000 ms.
    fn timer_engine() -> (
        Engine,
        Ptr<BSTimer>,
        std::rc::Rc<std::cell::RefCell<Vec<u32>>>,
    ) {
        let mut e = Engine::new();
        e.map(ZERO, 8);
        e.map(MILLISECONDS_TO_SECONDS, 8);
        e.mem.set_u64(MILLISECONDS_TO_SECONDS, 0.001f64.to_bits());
        e.map(GLOBAL_TIME_MULTIPLIER, 4);
        e.mem.set_f32(GLOBAL_TIME_MULTIPLIER, 1.0);
        let slept = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let log = slept.clone();
        e.register_double(SLEEP, move |e, a| {
            log.borrow_mut().push(a[0]);
            e.advance_clock_us(u64::from(a[0]) * 1000);
            Ret::default()
        });
        let timer = e.new_object::<BSTimer>();
        e.clock_ms = 1000;
        e.set(timer, BSTimer::iFirstTime, 0);
        e.set(timer, BSTimer::iLastTime, 1000);
        (e, timer, slept)
    }

    /// The translation lifts steps 4 ms apart to 10 ms by sleeping and reports 10 ms; the uncapped step
    /// neither sleeps nor rounds up: 4 ms each (the first, with nothing before it, answers as the
    /// translation does: 10 ms).
    #[test]
    fn the_uncapped_step_does_not_sleep_and_reports_the_real_time() {
        let (mut e, timer, slept) = timer_engine();
        for _ in 0..3 {
            e.advance_clock_us(4000);
            let now = e.clock_ms;
            e.call(BS_TIMER_STEP, &[timer.addr(), now]);
            assert_eq!(e.get(timer, BSTimer::fDelta), 0.01);
        }
        // The second step's 10 ms include the first one's sleep.
        assert_eq!(*slept.borrow(), [6, 6]);

        let (mut e, timer, slept) = timer_engine();
        install(&mut e);
        for step in 0..3 {
            e.advance_clock_us(4000);
            let now = e.clock_ms;
            e.call(BS_TIMER_STEP, &[timer.addr(), now]);
            let expected = if step == 0 { 0.01 } else { 0.004 };
            assert_eq!(e.get(timer, BSTimer::fDelta), expected);
            assert_eq!(e.get(timer, BSTimer::fDeltaUnmodified), expected);
            assert_eq!(e.get(timer, BSTimer::iLastTime), now);
            assert!(!e.get(timer, BSTimer::bZeroDelta));
        }
        // No real sleep: only a pass for the other threads where the original sleeps.
        assert_eq!(*slept.borrow(), [0, 0, 0]);
        // Under a millisecond: the microsecond clock still sees it.
        e.advance_clock_us(250);
        let now = e.clock_ms;
        e.call(BS_TIMER_STEP, &[timer.addr(), now]);
        assert_eq!(e.get(timer, BSTimer::fDelta), 0.00025);
        // A long pause is cut to 166 ms, as the translation cuts it.
        e.advance_clock_us(2_000_000);
        let now = e.clock_ms;
        e.call(BS_TIMER_STEP, &[timer.addr(), now]);
        assert_eq!(e.get(timer, BSTimer::fDelta), 0.166);
        // Steps of 10 ms and more by the tick count get no pass: the original doesn't sleep there.
        let passes = slept.borrow().len();
        e.advance_clock_us(12_000);
        let now = e.clock_ms;
        e.call(BS_TIMER_STEP, &[timer.addr(), now]);
        assert_eq!(e.get(timer, BSTimer::fDelta), 0.012);
        assert_eq!(slept.borrow().len(), passes);
    }

    /// The first step after a disable gives the translation's answer, not the tick count's raw
    /// milliseconds: none passed is a zero delta of 10 ms, a few are 10 ms, many at most 166.
    #[test]
    fn the_first_step_answers_as_the_original_does() {
        let first = |gap_ms: u32| {
            let (mut e, timer, _) = timer_engine();
            install(&mut e);
            e.set(timer, BSTimer::cDisableCounter, 1);
            e.call(BS_TIMER_STEP, &[timer.addr(), e.clock_ms]);
            e.set(timer, BSTimer::cDisableCounter, 0);
            e.advance_clock_us(u64::from(gap_ms) * 1000);
            let now = e.clock_ms;
            e.call(BS_TIMER_STEP, &[timer.addr(), now]);
            (
                e.get(timer, BSTimer::fDelta),
                e.get(timer, BSTimer::bZeroDelta),
            )
        };
        assert_eq!(first(0), (0.01, true));
        assert_eq!(first(3), (0.01, false));
        assert_eq!(first(40), (0.04, false));
        assert_eq!(first(900), (0.166, false));
    }

    /// A disabled timer and a fixed step run the translation itself.
    #[test]
    fn disabled_and_fixed_step_timers_run_the_translation() {
        let (mut e, timer, slept) = timer_engine();
        install(&mut e);
        e.set(timer, BSTimer::cDisableCounter, 1);
        e.advance_clock_us(4000);
        e.call(BS_TIMER_STEP, &[timer.addr(), e.clock_ms]);
        assert_eq!(e.get(timer, BSTimer::fDelta), 0.0);
        e.set(timer, BSTimer::cDisableCounter, 0);
        e.set(timer, BSTimer::fClamp, 16.0);
        e.call(BS_TIMER_STEP, &[timer.addr(), e.clock_ms]);
        assert_eq!(e.get(timer, BSTimer::fDelta), 0.016);
        assert!(slept.borrow().is_empty());
    }

    /// The padding bytes hold the last step, and no field moves.
    #[test]
    fn the_last_step_lives_in_padding() {
        let (mut e, timer, _) = timer_engine();
        e.set(timer, BSTimer::fClamp, 2.5);
        e.set(timer, BSTimer::bZeroDelta, true);
        let us = 0x1234_5678 * 1000 + 789;
        set_last_step_us(&mut e, timer, Some(us));
        assert_eq!(last_step_us(&e, timer), Some(us));
        set_last_step_us(&mut e, timer, Some(0));
        assert_eq!(last_step_us(&e, timer), Some(0));
        set_last_step_us(&mut e, timer, None);
        assert_eq!(last_step_us(&e, timer), None);
        assert_eq!(e.get(timer, BSTimer::fClamp), 2.5);
        assert!(e.get(timer, BSTimer::bZeroDelta));
        assert_eq!(e.get(timer, BSTimer::cDisableCounter), 0);
    }

    /// The machine's uptime in milliseconds (the clock's start) times 1000 passes 2^40 after 12.7
    /// days, and the clock itself wraps at 49.7 days: the step stays what really passed.
    #[test]
    fn long_uptime_keeps_the_step() {
        for start_ms in [1_200_000_000u32, 3_000_000_000, u32::MAX - 6, 0] {
            let (mut e, timer, _) = timer_engine();
            install(&mut e);
            e.clock_ms = start_ms;
            e.set(timer, BSTimer::iFirstTime, 0);
            e.set(timer, BSTimer::iLastTime, start_ms);
            for step in 0..6 {
                e.advance_clock_us(4000);
                let now = e.clock_ms;
                e.call(BS_TIMER_STEP, &[timer.addr(), now]);
                let expected = if step == 0 { 0.01 } else { 0.004 };
                assert_eq!(e.get(timer, BSTimer::fDelta), expected, "{start_ms} {step}");
            }
        }
    }

    /// An engine with the pages `screen` reads, and a start menu object at `0x7000`.
    fn screen_engine() -> Engine {
        let mut e = Engine::new();
        for page in [LOADING_MENU, START_MENU, MENU_MODE] {
            e.map(page & !0xfff, 0x1000);
        }
        e.map(0x7000, 0x1000);
        e
    }

    /// `OnIdle`'s own tests: the menu byte is a menu; the loading menu is loading unless the main
    /// menu or the credits are over it (the pause menu isn't).
    #[test]
    fn the_screen_is_what_the_main_loop_tests() {
        let mut e = screen_engine();
        assert_eq!(screen(&e), Screen::Play);
        e.mem.set_u8(MENU_MODE, 1);
        assert_eq!(screen(&e), Screen::Menu);
        // A load: menu mode too, but loading first.
        e.mem.set_u32(LOADING_MENU, 0x5000);
        assert_eq!(screen(&e), Screen::Loading);
        // The main menu over the loading menu (its background): a menu.
        e.mem.set_u32(START_MENU, 0x7000);
        e.mem.set_u32(0x7000 + START_MENU_FLAGS, 0);
        assert_eq!(screen(&e), Screen::Menu);
        // The pause menu (bit 0) doesn't hide a load; the credits do.
        e.mem.set_u32(0x7000 + START_MENU_FLAGS, START_MENU_IN_GAME);
        assert_eq!(screen(&e), Screen::Loading);
        e.mem.set_u32(
            0x7000 + START_MENU_FLAGS,
            START_MENU_IN_GAME | START_MENU_CREDITS,
        );
        assert_eq!(screen(&e), Screen::Menu);
        e.mem.set_u32(LOADING_MENU, 0);
        e.mem.set_u8(MENU_MODE, 0);
        assert_eq!(screen(&e), Screen::Play);
    }

    /// Menus at the menu limit unless the player's is lower, loading never limited, play at the
    /// player's limit; either 0 is no limit of its own.
    #[test]
    fn the_limit_is_chosen_by_the_screen() {
        use Screen::*;
        assert_eq!(frame_limit_for(Play, 0, 120), 0);
        assert_eq!(frame_limit_for(Play, 144, 120), 144);
        assert_eq!(frame_limit_for(Menu, 0, 120), 120);
        assert_eq!(frame_limit_for(Menu, 60, 120), 60);
        assert_eq!(frame_limit_for(Menu, 144, 120), 120);
        assert_eq!(frame_limit_for(Menu, 144, 0), 144);
        assert_eq!(frame_limit_for(Menu, 0, 0), 0);
        assert_eq!(frame_limit_for(Loading, 0, 120), 0);
        assert_eq!(frame_limit_for(Loading, 60, 30), 0);

        // With the row off the player's limit holds on every screen, as before it.
        let mut e = screen_engine();
        e.mem.set_u8(MENU_MODE, 1);
        assert_eq!(limit_this_frame(&e, 0, None), 0);
        assert_eq!(limit_this_frame(&e, 90, None), 90);
        assert_eq!(limit_this_frame(&e, 0, Some(120)), 120);
        e.mem.set_u32(LOADING_MENU, 0x5000);
        assert_eq!(limit_this_frame(&e, 90, None), 90);
        assert_eq!(limit_this_frame(&e, 90, Some(120)), 0);
        e.mem.set_u32(LOADING_MENU, 0);
        e.mem.set_u8(MENU_MODE, 0);
        assert_eq!(limit_this_frame(&e, 90, Some(120)), 90);
    }

    /// The pull of a held body: the setting scaled by the frame's seconds times 60 while the grab
    /// update runs, at most 4, given back afterwards.
    #[test]
    fn the_pull_follows_the_frame_time() {
        let mut e = Engine::new();
        e.map(HELPER_SCALE & !0xfff, 0x1000);
        e.mem.set_f32(HELPER_SCALE, 2.0);
        let seen =
            |e: &mut Engine, seconds: f32| with_grab_scale(e, seconds, |e| e.mem.f32(HELPER_SCALE));
        assert!((seen(&mut e, 1.0 / 60.0) - 2.0).abs() < 1e-6);
        let at_144 = seen(&mut e, 1.0 / 144.0);
        assert!((at_144 - 2.0 * 60.0 / 144.0).abs() < 1e-6, "{at_144}");
        // A second's pull is the same: 144 frames of it against 60 at 60 frames a second.
        assert!((144.0 * at_144 - 60.0 * 2.0).abs() < 1e-3);
        // A slow frame (50 ms) pulls three frames' worth; 100 ms stops at 4 (TEMPO's cap).
        assert!((seen(&mut e, 0.05) - 6.0).abs() < 1e-5);
        assert_eq!(seen(&mut e, 0.1), 8.0);
        // No time passed: no pull; a time that isn't a number: the pull as it is.
        assert_eq!(seen(&mut e, 0.0), 0.0);
        assert_eq!(seen(&mut e, f32::NAN), 2.0);
        assert_eq!(e.mem.f32(HELPER_SCALE), 2.0);
        // A nested entry keeps the outer scale.
        let nested = with_grab_scale(&mut e, 1.0 / 120.0, |e| {
            with_grab_scale(e, 1.0 / 30.0, |e| e.mem.f32(HELPER_SCALE))
        });
        assert_eq!(nested, 1.0);
        assert_eq!(e.mem.f32(HELPER_SCALE), 2.0);
        // Something that sets the setting meanwhile keeps its value.
        with_grab_scale(&mut e, 1.0 / 144.0, |e| e.mem.set_f32(HELPER_SCALE, 3.0));
        assert_eq!(e.mem.f32(HELPER_SCALE), 3.0);
        // A throw through it gives the setting back.
        e.mem.set_f32(HELPER_SCALE, 2.0);
        let thrown = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            with_grab_scale(&mut e, 1.0 / 144.0, |_| panic!("thrown"))
        }));
        assert!(thrown.is_err());
        assert_eq!(e.mem.f32(HELPER_SCALE), 2.0);
        assert_eq!(seen(&mut e, 1.0 / 30.0), 4.0);
    }

    /// The limit holds whole frames at the main timer only, gives the other threads their turn
    /// first, and doesn't catch up after a long frame.
    #[test]
    fn the_frame_limit_paces_the_main_timer() {
        let (mut e, _, slept) = timer_engine();
        e.map(MAIN_TIMER & !0xfff, 0x1000);
        // The pages the menu frame limit reads, should another test have bound it: all zero, play.
        for page in [LOADING_MENU, START_MENU, MENU_MODE] {
            e.map(page & !0xfff, 0x1000);
        }
        let main: Ptr<BSTimer> = Ptr::new(MAIN_TIMER);
        e.set(main, BSTimer::iLastTime, 1000);
        NEXT_FRAME_US.store(0, Ordering::Relaxed);
        let mut starts = Vec::new();
        for work_us in [2000, 2000, 2000, 35_000, 2000] {
            e.advance_clock_us(work_us);
            wait_for_frame_limit(&mut e, 100);
            starts.push(e.clock_now_us());
            let now = e.clock_ms;
            uncapped_step(&mut e, main, now);
        }
        assert_eq!(starts[1] - starts[0], 10_000);
        assert_eq!(starts[2] - starts[1], 10_000);
        assert_eq!(starts[3] - starts[2], 35_000);
        assert_eq!(starts[4] - starts[3], 10_000);
        assert_eq!(e.get(main, BSTimer::fDeltaUnmodified), 0.01);
        // One pass per frame from the wait; the step itself adds one where the original would sleep.
        assert!(slept.borrow().len() >= 5);
        assert!(slept.borrow().iter().all(|&ms| ms == 0));
        // The clock wraps: the old start time is about 49.7 days ahead, and nothing waits for it.
        NEXT_FRAME_US.store(u64::from(u32::MAX) * 1000, Ordering::Relaxed);
        let before = e.clock_now_us();
        wait_for_frame_limit(&mut e, 100);
        assert_eq!(e.clock_now_us(), before);
        assert_eq!(NEXT_FRAME_US.load(Ordering::Relaxed), before + 10_000);
    }

    #[test]
    fn only_a_short_standing_landing_falls_on() {
        let ground = STATE_ON_GROUND;
        let air = STATE_IN_AIR;
        // A 5 ms frame: the rest of two 16 ms frames.
        let rest = landing_time(0.005, air, ground, false, -64.0).unwrap();
        assert!((rest - 0.027).abs() < 1e-6, "{rest}");
        // 16 ms frames and longer land as they always did.
        assert_eq!(landing_time(0.016, air, ground, false, -64.0), None);
        assert_eq!(landing_time(0.031, air, ground, false, -64.0), None);
        // Asked to move: the ground state's own velocity takes it down.
        assert_eq!(landing_time(0.005, air, ground, true, -64.0), None);
        // Going up, already on the ground, still in the air: nothing.
        assert_eq!(landing_time(0.005, air, ground, false, 3.0), None);
        assert_eq!(landing_time(0.005, ground, ground, false, -64.0), None);
        assert_eq!(landing_time(0.005, air, air, false, -64.0), None);
        // A controller that didn't step.
        assert_eq!(landing_time(0.0, air, ground, false, -64.0), None);
    }

    /// A controller whose proxy logs what `fall_on` does to it, in order.
    fn landing_setup(
        flags: u32,
    ) -> (
        Engine,
        Ptr<Controller>,
        std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    ) {
        const HALF: u32 = 0x0900_0094;
        const PROXY: u32 = 0x7700;
        let mut e = Engine::new();
        let this: Ptr<Controller> = e.new_object();
        let me = this.addr();
        let log = std::rc::Rc::new(std::cell::RefCell::new(Vec::<String>::new()));
        e.put_vtable(0x0900_0000, &[HALF; 0x26]);
        e.mem.set_u32(me, 0x0900_0000);
        e.map(PROXY & !0xfff, 0x1000);
        e.mem.set_u32(PROXY + PROXY_MODE, 4);
        e.set(this, Controller::phkObject, PROXY);
        e.set(this, Controller::iFlags, flags);
        e.set(this, Controller::StepDeltaTime, 0.005);
        e.set(this, Controller::StepInvDeltaTime, 200.0);
        let velocity = me + Controller::OutVelocity.off;
        e.mem.set_f32(velocity, 1.5);
        e.mem.set_f32(velocity + 4, 3.0);
        e.register_double(HALF, |_, _| 0x6600u32.into_ret());
        for address in [0x00ca_c110, 0x00cd_3410, 0x00cd_32b0, 0x00cd_3490] {
            e.register_double(address, |_, _| Ret::default());
        }
        let seen = log.clone();
        e.register_double(SET_LINEAR_VELOCITY, move |e, a| {
            assert_eq!(a[0], PROXY);
            seen.borrow_mut().push(format!(
                "velocity {} {} {}",
                e.mem.f32(a[1]),
                e.mem.f32(a[1] + 4),
                e.mem.f32(a[1] + 8)
            ));
            Ret::default()
        });
        let seen = log.clone();
        e.register_double(0x00ca_fbd0, move |e, a| {
            seen.borrow_mut().push(format!(
                "integrate {} {} mode {}",
                e.mem.f32(a[1] + 8),
                e.mem.f32(a[1] + 12),
                e.mem.u32(PROXY + PROXY_MODE)
            ));
            Ret::default()
        });
        (e, this, log)
    }

    /// One more integration the way `Move` does its own: the sideways velocity `Move` left, the fall
    /// speed down, the proxy's mode word 4 (1 for a large creature) and only the step's delta
    /// changed; then the step, the mode and both velocities as `Move` left them.
    #[test]
    fn falling_on_integrates_once_as_move_does() {
        let (mut e, this, log) = landing_setup(0);
        let velocity = this.addr() + Controller::OutVelocity.off;
        fall_on(&mut e, this, -64.0, 0.025);
        assert_eq!(
            *log.borrow(),
            [
                "velocity 1.5 3 -64",
                "integrate 0.025 40 mode 4",
                "velocity 1.5 3 0"
            ]
        );
        assert_eq!(e.get(this, Controller::StepDeltaTime), 0.005);
        assert_eq!(e.get(this, Controller::StepInvDeltaTime), 200.0);
        assert_eq!(e.mem.f32(velocity), 1.5);
        assert_eq!(e.mem.f32(velocity + 4), 3.0);
        assert_eq!(e.mem.f32(velocity + 8), 0.0);
        assert_eq!(e.mem.u32(0x7700 + PROXY_MODE), 4);

        // A large creature integrates in mode 1, and the mode word goes back to 4.
        let (mut e, this, log) = landing_setup(FLAG_LARGE);
        fall_on(&mut e, this, -64.0, 0.025);
        assert_eq!(log.borrow()[1], "integrate 0.025 40 mode 1");
        assert_eq!(e.mem.u32(0x7700 + PROXY_MODE), 4);
    }

    /// The proxy's velocity goes back to the controller's out-velocity, not to zero: a character
    /// standing on something that moves keeps the motion `Move` gave it.
    #[test]
    fn falling_on_gives_the_proxy_move_s_velocity_back() {
        let (mut e, this, log) = landing_setup(0);
        let velocity = this.addr() + Controller::OutVelocity.off;
        e.mem.set_f32(velocity + 8, 2.0);
        fall_on(&mut e, this, -64.0, 0.025);
        assert_eq!(log.borrow().last().unwrap(), "velocity 1.5 3 2");
    }

    /// Only the player's controller gets the landing (the others have no measurement).
    #[test]
    fn only_the_players_controller_is_a_landing_candidate() {
        let (mut e, this, _) = landing_setup(0);
        e.map(PLAYER & !0xfff, 0x1000);
        assert!(!is_player_controller(&mut e, this));
        e.mem.set_u32(PLAYER, 0x5500);
        let me = this.addr();
        e.register_double(GET_CHAR_CONTROLLER, move |_, a| {
            assert_eq!(a[0], 0x5500);
            me.into_ret()
        });
        assert!(is_player_controller(&mut e, this));
        let other: Ptr<Controller> = e.new_object();
        assert!(!is_player_controller(&mut e, other));
    }

    /// The multiplier's frame-rate-free move: three 5.33 ms frames go as far as one 16 ms frame
    /// of the translation, and 16 ms frames are the translation's.
    #[test]
    fn the_time_multiplier_moves_by_time() {
        let setup = |step: f32| {
            let mut e = Engine::new();
            e.map(MAIN_TIMER & !0xfff, 0x1000);
            e.map(GLOBAL_TIME_MULTIPLIER & !0xfff, 0x1000);
            e.mem.set_f32(GLOBAL_TIME_MULTIPLIER, 1.0);
            e.mem.set_f32(TIME_MULTIPLIER_TARGET, 0.25);
            e.mem.set_f32(TIME_MULTIPLIER_RATE, 0.1);
            e.mem
                .set_f32(MAIN_TIMER + BSTimer::fDeltaUnmodified.off, step);
            e
        };
        let mut e = setup(0.016);
        e.call(TIME_MULTIPLIER_RAMP, &args![]);
        let one_frame = e.mem.f32(GLOBAL_TIME_MULTIPLIER);
        assert_eq!(one_frame, 0.9);
        install_slow_motion(&mut e);
        e.mem.set_f32(GLOBAL_TIME_MULTIPLIER, 1.0);
        e.call(TIME_MULTIPLIER_RAMP, &args![]);
        assert_eq!(e.mem.f32(GLOBAL_TIME_MULTIPLIER), one_frame);

        let mut e = setup(0.016 / 3.0);
        install_slow_motion(&mut e);
        for _ in 0..3 {
            e.call(TIME_MULTIPLIER_RAMP, &args![]);
        }
        assert!((e.mem.f32(GLOBAL_TIME_MULTIPLIER) - 0.9).abs() < 1e-5);
        // Up to the target and no further; at the target it stays.
        e.mem.set_f32(GLOBAL_TIME_MULTIPLIER, 0.2499);
        e.call(TIME_MULTIPLIER_RAMP, &args![]);
        assert_eq!(e.mem.f32(GLOBAL_TIME_MULTIPLIER), 0.25);
        e.call(TIME_MULTIPLIER_RAMP, &args![]);
        assert_eq!(e.mem.f32(GLOBAL_TIME_MULTIPLIER), 0.25);
    }
}

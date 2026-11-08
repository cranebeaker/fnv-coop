//! The main loop's tick (ADR-0016 §9; build step I-8), translated from xNVSE's
//! `HandleMainLoopHook` (`Hooks_Gameplay.cpp` (xNVSE)), run at the seam `0086B386` in `main` on
//! every pass of the loop with the game in the foreground (lost-device passes included), and the
//! delayed calls it runs (`CallAfterSeconds`, `CallAfterFrames`, `CallForSeconds`, `CallWhile`,
//! `CallWhen`, `CallWhilePerSeconds`, `Commands_Script.cpp` (xNVSE)).
//!
//! In xNVSE's order:
//! - **The first frame:** the main thread is recorded (from then on xNVSE prints to the
//!   console), the editor ids of ten vanilla forms are set (`ApplyGECKEditorIDs`) and xNVSE's
//!   version line is printed.
//! - **Every frame:** the temporary strings and
//!   arrays, the lists kept for lambdas no longer needed, then the delayed calls. Game time for
//!   them: the frame's seconds (`TimeGlobal` `0x011F6394`, `+0xC`) times V.A.T.S.'s multiplier
//!   (`009c8cc0` on `0x011F2250`), summed (`g_gameSecondsPassed`); menu mode from `00702360`;
//!   "paused" when the start menu (`0x011DAAC0`) or the console is open.
//!
//! Not here yet: the first frame's `bWarnScriptErrors` choice (read from `nvse_config.ini` or
//! `MyGECKMods.txt`; off, as without either file), the plugins' load errors and their
//! `DeferredInit` and `MainGameLoop` messages (JIP, I-13), the user function files and the
//! locale fix (with the run-time compiler, I-14), the inventory references' clean-up (I-12).

use crate::expr::{Eval, Tok};
use crate::functions::{self, Caller};
use crate::table::{Command, Param, Returns};
use crate::vars::{self, Data};
use crate::{handled, lambda, CommandArgs, Nvse};
use engine_core::extend::{Owner, SiteSlot};
use engine_core::prelude::*;

/// The seam at `0086B386` in `main` (before the getter call xNVSE's `MainLoopHook` replaces).
pub const TICK: SiteSlot = SiteSlot::new(Owner::Nvse, 0x0086_b386);

/// `g_timeGlobal` and its `secondsPassed`; V.A.T.S.'s time multiplier (`thiscall`, returns a
/// `double`) and its object; `MenuMode`; the start menu (`g_startMenu`); the console's
/// singleton getter and `IsConsoleOpen`.
const TIME_GLOBAL: u32 = 0x011f_6394;
const VATS_TIME_MULT: u32 = 0x009c_8cc0;
const VATS_TIME_OBJECT: u32 = 0x011f_2250;
const MENU_MODE: u32 = 0x0070_2360;
const START_MENU: u32 = 0x011d_aac0;
const CONSOLE_INSTANCE: u32 = 0x0071_b160;
const CONSOLE_IS_OPEN: u32 = 0x004a_4020;
/// `LookupFormByID` and `TESForm::SetEditorID_AtRuntime` (the `+0x134` virtual function).
const LOOKUP_FORM: u32 = 0x0048_39c0;
const SET_EDITOR_ID: u32 = 0x134;

/// `ApplyGECKEditorIDs`.
const GECK_EDITOR_IDS: [(u32, &str); 10] = [
    (0x14, "PlayerRef"),
    (0xa, "Lockpick"),
    (0x2d, "MaleAdult01Default"),
    (0x2e, "FemaleAdult01Default"),
    (0x33, "RadiationMarker"),
    (0x3d, "DefaultCombatStyle"),
    (0x147, "PipBoyLight"),
    (0x163, "HelpManual"),
    (0x1f5, "DefaultWaterExplosion"),
    (0x1f6, "GasTrapDummy"),
];

/// `DelayedCallInfo` (`CallAfterSeconds`, `CallAfterFrames`, `CallForSeconds`): the function,
/// the time (game seconds, or frames left), the calling reference, the mode (0 game mode only,
/// 1 also in menu mode, 2 and up also not while paused) and the arguments.
#[derive(Clone, Debug, PartialEq)]
pub struct Delayed {
    pub script: u32,
    pub time: f32,
    pub this_obj: u32,
    pub mode: u8,
    pub args: Vec<Data>,
    /// Which entry this is, while lists change under a pass.
    pub serial: u64,
}

impl Delayed {
    fn should_run(&self, menu: bool, paused: bool) -> bool {
        !(menu && self.mode == 0 || paused && self.mode >= 2)
    }
}

/// `CallWhileInfo` (`CallWhile`, `CallWhen`) and `DelayedCallWhileInfo`
/// (`CallWhilePerSeconds`: with the interval and the last time it ran).
#[derive(Clone, Debug, PartialEq)]
pub struct While {
    pub call: u32,
    pub condition: u32,
    pub this_obj: u32,
    /// Bit 0: arguments to the function; 1: to the condition; 2: not in menu mode; 3: not in
    /// game mode; 4: not while paused.
    pub flags: u8,
    pub args: Vec<Data>,
    /// Which entry this is, while lists change under a pass.
    pub serial: u64,
    pub interval: f32,
    pub old_time: f32,
}

impl While {
    fn should_run(&self, menu: bool, paused: bool) -> bool {
        !(menu && self.flags & 4 != 0
            || !menu && self.flags & 8 != 0
            || paused && self.flags & 16 != 0)
    }
}

/// The tick's state.
#[derive(Default)]
pub struct State {
    /// `g_mainThreadID`.
    pub main_thread: u32,
    /// `g_gameSecondsPassed`.
    pub game_seconds: f32,
    pub after: Vec<Delayed>,
    pub after_frames: Vec<Delayed>,
    pub for_seconds: Vec<Delayed>,
    pub while_: Vec<While>,
    pub when: Vec<While>,
    pub while_per_seconds: Vec<While>,
    pub next_serial: u64,
}

fn state(e: &mut Engine) -> &mut State {
    &mut e.ext::<Nvse>().tick
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Hooks_Gameplay.cpp: HandleMainLoopHook.
fn tick(e: &mut Engine, _: &[u32]) -> Option<u32> {
    if !e.ext::<Nvse>().main_thread_recorded {
        first_frame(e);
    }
    vars::clean_arrays(e);
    vars::clean_strings(e);
    lambda::erase_unused_saved_lists(e);
    let mult = e.call(VATS_TIME_MULT, &args![VATS_TIME_OBJECT]).f64();
    let seconds = e.mem.f32(TIME_GLOBAL + 0xc);
    let delta = seconds * mult as f32;
    let menu = e.call(MENU_MODE, &[]).bool();
    state(e).game_seconds += delta;
    run_while(e, WhileList::While, menu);
    run_while(e, WhileList::When, menu);
    run_seconds(e, DelayedList::After, delta, menu);
    run_seconds(e, DelayedList::For, delta, menu);
    run_while_per_seconds(e, delta, menu);
    run_after_frames(e, menu);
    None
}

/// The first frame's part.
fn first_frame(e: &mut Engine) {
    for (id, name) in GECK_EDITOR_IDS {
        let form = e.call(LOOKUP_FORM, &args![id]).u32();
        if form != 0 {
            let text = crate::table::guest_str(e, name.as_bytes());
            e.vcall(form, SET_EDITOR_ID, &args![text]);
        }
    }
    e.ext::<Nvse>().main_thread_recorded = true;
    let (major, minor, beta) = crate::VERSION;
    let line = format!("xNVSE {major}.{minor}.{beta}");
    crate::errors::print_long(e, line.as_bytes());
    let thread = engine_core::thread_host::current_id(e);
    state(e).main_thread = thread;
}

/// `IsGamePaused`: the start menu or the console is open.
fn is_paused(e: &mut Engine) -> bool {
    if e.mem.u32(START_MENU) != 0 {
        return true;
    }
    let console = e.call(CONSOLE_INSTANCE, &args![1u32]).u32();
    console != 0 && e.call(CONSOLE_IS_OPEN, &args![console]).bool()
}

fn call_with(e: &mut Engine, script: u32, this_obj: u32, args: Option<Vec<Data>>) -> Option<Tok> {
    functions::call(
        e,
        Caller::Elements {
            script,
            this_obj,
            containing: 0,
            args,
        },
    )
}

/// `GetBool` of a function's result.
fn truth(t: &Tok) -> bool {
    match t {
        Tok::Number { v, .. } => *v != 0.0,
        Tok::Form(id) | Tok::Array(id) => *id != 0,
        Tok::Str(s) => !s.is_empty(),
        Tok::Bool(b) => *b,
        _ => false,
    }
}

/// The arguments as `SelfOwningArrayElement`s hold them: an array argument keeps a reference
/// from the array's own mod while the call waits (`SelfOwningArrayElement::SetArray`); a
/// missing array is 0.
fn hold_arguments(e: &mut Engine, args: &mut [Data]) {
    for a in args.iter_mut() {
        if let Data::Array(id) = a {
            let to = *id;
            let mut held = 0;
            let owner = vars::array_owning_mod(e, to);
            vars::add_reference(e, &mut held, to, owner);
            *id = held;
        }
    }
}

/// `~SelfOwningArrayElement`: each array argument's reference given up.
fn release_arguments(e: &mut Engine, args: &[Data]) {
    for a in args {
        if let Data::Array(id) = a {
            let mut held = *id;
            let owner = vars::array_owning_mod(e, held);
            vars::remove_reference(e, &mut held, owner);
        }
    }
}

/// Gives up what a call kept (`~DelayedCallInfo`: its lambda's variables, its arguments).
fn release_delayed(e: &mut Engine, d: &Delayed) {
    lambda::unsave_variables(e, d.script);
    release_arguments(e, &d.args);
}

fn release_while(e: &mut Engine, w: &While) {
    lambda::unsave_variables(e, w.call);
    lambda::unsave_variables(e, w.condition);
    release_arguments(e, &w.args);
}

/// The lists, by kind.
#[derive(Clone, Copy, PartialEq, Eq)]
enum DelayedList {
    After,
    AfterFrames,
    For,
}

fn delayed_list(e: &mut Engine, which: DelayedList) -> &mut Vec<Delayed> {
    let st = state(e);
    match which {
        DelayedList::After => &mut st.after,
        DelayedList::AfterFrames => &mut st.after_frames,
        DelayedList::For => &mut st.for_seconds,
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum WhileList {
    While,
    When,
    PerSeconds,
}

fn while_list(e: &mut Engine, which: WhileList) -> &mut Vec<While> {
    let st = state(e);
    match which {
        WhileList::While => &mut st.while_,
        WhileList::When => &mut st.when,
        WhileList::PerSeconds => &mut st.while_per_seconds,
    }
}

/// Where the entry with `serial` is now: a function the entry called may have added to or
/// cleared the lists (a load or a new game started by a script clears them), so positions are
/// looked up again after every call, never assumed.
fn delayed_position(e: &mut Engine, which: DelayedList, serial: u64) -> Option<usize> {
    delayed_list(e, which)
        .iter()
        .position(|d| d.serial == serial)
}

fn while_position(e: &mut Engine, which: WhileList, serial: u64) -> Option<usize> {
    while_list(e, which).iter().position(|w| w.serial == serial)
}

fn remove_delayed(e: &mut Engine, which: DelayedList, serial: u64) {
    if let Some(i) = delayed_position(e, which, serial) {
        let gone = delayed_list(e, which).remove(i);
        release_delayed(e, &gone);
    }
}

fn remove_while(e: &mut Engine, which: WhileList, serial: u64) {
    if let Some(i) = while_position(e, which, serial) {
        let gone = while_list(e, which).remove(i);
        release_while(e, &gone);
    }
}

/// The entry after the one with serial `after` (0: the first), and its position. xNVSE keeps
/// each list as a `std::list` and walks it with an iterator (`Hooks_Gameplay.cpp`), so an entry
/// a call appends during the pass (`emplace_back`) is reached in the same pass, and one that
/// reschedules itself with `CallAfterFrames 1` or `CallAfterSeconds 0` runs again and again
/// within one frame (the game hangs, as with xNVSE). Entries are only appended, with rising
/// serials, and removed, so each list stays ordered by serial and "the next entry" is the first
/// with a larger serial: that reaches appended entries and stays right when a call removed or
/// cleared entries (a load or a new game started by a script).
fn next_delayed(e: &mut Engine, which: DelayedList, after: u64) -> Option<(usize, u64)> {
    let list = delayed_list(e, which);
    let i = list.iter().position(|d| d.serial > after)?;
    Some((i, list[i].serial))
}

fn next_while(e: &mut Engine, which: WhileList, after: u64) -> Option<(usize, u64)> {
    let list = while_list(e, which);
    let i = list.iter().position(|w| w.serial > after)?;
    Some((i, list[i].serial))
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Hooks_Gameplay.cpp: HandleDelayedCall,
// HandleCallForScripts.
/// `CallAfterSeconds` (runs once when due) and `CallForSeconds` (every frame until due).
fn run_seconds(e: &mut Engine, which: DelayedList, delta: f32, menu: bool) {
    if delayed_list(e, which).is_empty() {
        return;
    }
    let paused = is_paused(e);
    let mut after = 0;
    while let Some((i, serial)) = next_delayed(e, which, after) {
        after = serial;
        if !delayed_list(e, which)[i].should_run(menu, paused) {
            delayed_list(e, which)[i].time += delta;
        }
        let d = delayed_list(e, which)[i].clone();
        let due = state(e).game_seconds >= d.time;
        if due == (which == DelayedList::After) {
            call_with(e, d.script, d.this_obj, Some(d.args.clone()));
        }
        if due {
            remove_delayed(e, which, serial);
        }
    }
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Hooks_Gameplay.cpp:
// HandleCallAfterFramesScripts.
fn run_after_frames(e: &mut Engine, menu: bool) {
    let which = DelayedList::AfterFrames;
    if delayed_list(e, which).is_empty() {
        return;
    }
    let paused = is_paused(e);
    let mut after = 0;
    while let Some((i, serial)) = next_delayed(e, which, after) {
        after = serial;
        if !delayed_list(e, which)[i].should_run(menu, paused) {
            continue;
        }
        delayed_list(e, which)[i].time -= 1.0;
        let d = delayed_list(e, which)[i].clone();
        if d.time <= 0.0 {
            call_with(e, d.script, d.this_obj, Some(d.args.clone()));
            remove_delayed(e, which, serial);
        }
    }
}

/// The condition, then the function: with `args` when the flag says so.
fn condition_then_call(e: &mut Engine, w: &While) -> bool {
    let cond_args = (w.flags & 2 != 0).then(|| w.args.clone());
    let ok = call_with(e, w.condition, w.this_obj, cond_args).is_some_and(|t| truth(&t));
    if ok {
        let call_args = (w.flags & 1 != 0).then(|| w.args.clone());
        call_with(e, w.call, w.this_obj, call_args);
    }
    ok
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Hooks_Gameplay.cpp:
// HandleCallWhileScripts, HandleCallWhenScripts.
/// `CallWhile` (until the condition fails) and `CallWhen` (once, when it holds).
fn run_while(e: &mut Engine, which: WhileList, menu: bool) {
    if while_list(e, which).is_empty() {
        return;
    }
    let paused = is_paused(e);
    let mut after = 0;
    while let Some((i, serial)) = next_while(e, which, after) {
        after = serial;
        let w = while_list(e, which)[i].clone();
        if !w.should_run(menu, paused) {
            continue;
        }
        let ran = condition_then_call(e, &w);
        if ran == (which == WhileList::When) {
            remove_while(e, which, serial);
        }
    }
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Hooks_Gameplay.cpp:
// HandleCallWhilePerSecondsScripts.
fn run_while_per_seconds(e: &mut Engine, delta: f32, menu: bool) {
    let which = WhileList::PerSeconds;
    if while_list(e, which).is_empty() {
        return;
    }
    let paused = is_paused(e);
    let mut after = 0;
    while let Some((i, serial)) = next_while(e, which, after) {
        after = serial;
        let w = while_list(e, which)[i].clone();
        if !w.should_run(menu, paused) {
            while_list(e, which)[i].old_time = w.old_time + delta;
            continue;
        }
        if state(e).game_seconds - w.old_time >= w.interval {
            if condition_then_call(e, &w) {
                let now = state(e).game_seconds;
                if let Some(i) = while_position(e, which, serial) {
                    while_list(e, which)[i].old_time = now;
                }
            } else {
                remove_while(e, which, serial);
            }
        }
    }
}

/// `ClearDelayedCalls`: every delayed call dropped (xNVSE does it on a new game and before a
/// game is loaded, [`crate::saveload`]).
pub fn clear_delayed_calls(e: &mut Engine) {
    let st = state(e);
    let delayed: Vec<Delayed> = st
        .after
        .drain(..)
        .chain(st.after_frames.drain(..))
        .chain(st.for_seconds.drain(..))
        .collect();
    let whiles: Vec<While> = st
        .while_
        .drain(..)
        .chain(st.when.drain(..))
        .chain(st.while_per_seconds.drain(..))
        .collect();
    for d in &delayed {
        release_delayed(e, d);
    }
    for w in &whiles {
        release_while(e, w);
    }
}

/// A new entry's serial.
fn next_serial(e: &mut Engine) -> u64 {
    let st = state(e);
    st.next_serial += 1;
    st.next_serial
}

// ---- the commands ----

/// `kNVSEParamType_Number`, `_Form`, `_BasicType`.
const NUMBER: u32 = 0x8001;
const FORM: u32 = 0x8008;
const BASIC: u32 = 0x804d;

const fn p(name: &'static str, kind: u32, optional: bool) -> Param {
    Param {
        name,
        kind,
        optional,
    }
}
const ELEMENT: Param = p("element", BASIC, true);
/// `kNVSEParams_CallAfter`: seconds, function, flags, 15 elements.
const CALL_AFTER: &[Param] = &[
    p("seconds", NUMBER, false),
    p("function", FORM, false),
    p("flags", NUMBER, true),
    ELEMENT,
    ELEMENT,
    ELEMENT,
    ELEMENT,
    ELEMENT,
    ELEMENT,
    ELEMENT,
    ELEMENT,
    ELEMENT,
    ELEMENT,
    ELEMENT,
    ELEMENT,
    ELEMENT,
    ELEMENT,
    ELEMENT,
];
/// `kNVSEParams_CallWhile`: function, condition, flags, 15 elements.
const CALL_WHILE: &[Param] = &[
    p("function", FORM, false),
    p("condition", FORM, false),
    p("flags", NUMBER, true),
    ELEMENT,
    ELEMENT,
    ELEMENT,
    ELEMENT,
    ELEMENT,
    ELEMENT,
    ELEMENT,
    ELEMENT,
    ELEMENT,
    ELEMENT,
    ELEMENT,
    ELEMENT,
    ELEMENT,
    ELEMENT,
    ELEMENT,
];
/// `kNVSEParams_DelayedCallWhile`: seconds, function, condition, flags, 15 elements.
const DELAYED_CALL_WHILE: &[Param] = &[
    p("seconds", NUMBER, false),
    p("function", FORM, false),
    p("condition", FORM, false),
    p("flags", NUMBER, true),
    ELEMENT,
    ELEMENT,
    ELEMENT,
    ELEMENT,
    ELEMENT,
    ELEMENT,
    ELEMENT,
    ELEMENT,
    ELEMENT,
    ELEMENT,
    ELEMENT,
    ELEMENT,
    ELEMENT,
    ELEMENT,
    ELEMENT,
];
/// `kParams_CallAfter_OLD` and `kParams_CallWhile_OLD` (vanilla-format: `float`, any form,
/// optional integer).
const CALL_AFTER_OLD: &[Param] = &[
    p("seconds", 0x02, false),
    p("function", 0x3d, false),
    p("runs in menumode", 0x01, true),
];
const CALL_WHILE_OLD: &[Param] = &[p("function", 0x3d, false), p("condition", 0x3d, false)];

const fn cmd(
    opcode: u32,
    name: &'static str,
    short: &'static str,
    help: &'static str,
    params: &'static [Param],
    execute: AbiFn,
) -> Command {
    Command {
        opcode,
        name,
        short,
        help,
        needs_ref: false,
        params,
        execute,
        eval: None,
        returns: Returns::Default,
    }
}

pub static COMMANDS: &[Command] = &[
    cmd(
        0x1622,
        "CallAfterSeconds_OLD",
        "CallAfter_OLD",
        "deprecated",
        CALL_AFTER_OLD,
        call_after_old,
    ),
    cmd(
        0x1623,
        "CallWhile_OLD",
        "",
        "deprecated",
        CALL_WHILE_OLD,
        call_while_old,
    ),
    cmd(
        0x1624,
        "CallForSeconds_OLD",
        "CallFor_OLD",
        "deprecated",
        CALL_AFTER_OLD,
        call_for_old,
    ),
    cmd(
        0x1637,
        "CallWhen_OLD",
        "",
        "deprecated",
        CALL_WHILE_OLD,
        call_when_old,
    ),
    cmd(
        0x1641,
        "CallAfterSeconds",
        "CallAfter",
        "calls UDF after argument number of seconds",
        CALL_AFTER,
        call_after,
    ),
    cmd(
        0x1642,
        "CallForSeconds",
        "CallFor",
        "calls UDF each frame for argument number of seconds",
        CALL_AFTER,
        call_for,
    ),
    cmd(
        0x1643,
        "CallWhile",
        "",
        "calls UDF each frame while condition is met",
        CALL_WHILE,
        call_while,
    ),
    cmd(
        0x1644,
        "CallWhen",
        "",
        "calls UDF once when a condition is met which is polled each frame",
        CALL_WHILE,
        call_when,
    ),
    cmd(
        0x164d,
        "CallWhilePerSeconds",
        "",
        "calls UDF every couple of seconds, until the condition UDF returns false.",
        DELAYED_CALL_WHILE,
        call_while_per_seconds,
    ),
    cmd(
        0x164e,
        "CallAfterFrames",
        "",
        "calls UDF after argument number of frame",
        CALL_AFTER,
        call_after_frames,
    ),
];

/// The evaluated arguments of an expression-format command, or `None` when one failed.
fn extract(e: &mut Engine, a: &CommandArgs) -> Option<(Eval, Vec<Tok>)> {
    let mut ev = Eval::new(e, a);
    let toks = crate::expr::with_evaluator(e, |e| ev.extract_args(e));
    crate::expr::report(e, &ev);
    toks.map(|t| (ev, t))
}

/// `GetUserFunction`: the script the token gives, or 0.
fn user_function(e: &mut Engine, ev: &Eval, t: &Tok) -> u32 {
    let form = ev.get_form(e, t);
    if form != 0 && e.mem.u8(form + 4) == 0x11 {
        form
    } else {
        0
    }
}

/// `BasicTokenToElem` for each argument from `from` on, stopping at the first that has no
/// basic value.
fn elements(e: &mut Engine, ev: &Eval, toks: &[Tok], from: usize) -> Vec<Data> {
    let mut out = Vec::new();
    for t in toks.iter().skip(from) {
        let Some(basic) = functions::to_basic(e, ev, t) else {
            // An element left invalid (`SelfOwningArrayElement` default).
            out.push(Data::Invalid);
            continue;
        };
        out.push(match basic {
            Tok::Number { v, .. } => Data::Number(v),
            Tok::Str(s) => Data::Str(s),
            Tok::Form(id) => Data::Form(id),
            Tok::Array(id) => Data::Array(id),
            _ => Data::Invalid,
        });
    }
    // `BasicTokenToElem` into a `SelfOwningArrayElement`: an array is held.
    hold_arguments(e, &mut out);
    out
}

/// Which list a `DelayedCallInfo` goes to.
#[derive(Clone, Copy)]
enum AfterKind {
    Seconds,
    Frames,
    For,
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Script.cpp: ExtractCallAfterInfo.
fn add_after(e: &mut Engine, a: &CommandArgs, kind: AfterKind) -> bool {
    let Some((ev, toks)) = extract(e, a) else {
        return false;
    };
    // xNVSE reads both without a check; the compiler requires them.
    if toks.len() < 2 {
        return false;
    }
    let time = ev.get_number(e, &toks[0]) as f32;
    let script = user_function(e, &ev, &toks[1]);
    if script == 0 {
        return false;
    }
    let mut mode = 0;
    let mut args = Vec::new();
    if toks.len() > 2 {
        mode = to_u8(ev.get_number(e, &toks[2]));
        args = elements(e, &ev, &toks, 3);
    }
    let time = match kind {
        AfterKind::Frames => time,
        _ => state(e).game_seconds + time,
    };
    lambda::save_variables(e, script);
    let d = Delayed {
        script,
        time,
        this_obj: ev.this_obj,
        serial: next_serial(e),
        mode,
        args,
    };
    match kind {
        AfterKind::Seconds => state(e).after.push(d),
        AfterKind::Frames => state(e).after_frames.push(d),
        AfterKind::For => state(e).for_seconds.push(d),
    }
    true
}

/// A number to the `UInt8` of an enum, as MSVC converts (through `int`).
fn to_u8(v: f64) -> u8 {
    crate::args::f64_to_i32(v) as u8
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Script.cpp:
// ExtractCallAfterInfo_OLD.
fn add_after_old(e: &mut Engine, a: &CommandArgs, for_seconds: bool) -> bool {
    let read = crate::commands::Read::of(e, a);
    let script = read.form(1);
    if !read.ok || script == 0 || e.mem.u8(script + 4) != 0x11 {
        return false;
    }
    let time = read.float(0, 0.0);
    let mode = u8::from(read.int(2, 0) != 0);
    lambda::save_variables(e, script);
    let d = Delayed {
        script,
        time: state(e).game_seconds + time,
        this_obj: a.this_obj,
        serial: next_serial(e),
        mode,
        args: Vec::new(),
    };
    if for_seconds {
        state(e).for_seconds.push(d);
    } else {
        state(e).after.push(d);
    }
    true
}

/// Which list a `CallWhileInfo` goes to.
#[derive(Clone, Copy)]
enum WhileKind {
    While,
    When,
    PerSeconds,
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Script.cpp: ExtractCallWhileInfo,
// ExtractDelayedCallWhileInfo.
fn add_while(e: &mut Engine, a: &CommandArgs, kind: WhileKind) -> bool {
    let Some((ev, toks)) = extract(e, a) else {
        return false;
    };
    let first = usize::from(matches!(kind, WhileKind::PerSeconds));
    if toks.len() < first + 2 {
        return false;
    }
    let interval = if first == 1 {
        ev.get_number(e, &toks[0]) as f32
    } else {
        0.0
    };
    let call = user_function(e, &ev, &toks[first]);
    let condition = user_function(e, &ev, &toks[first + 1]);
    if call == 0 || condition == 0 {
        return false;
    }
    let mut flags = 0;
    let mut args = Vec::new();
    if toks.len() > first + 2 {
        flags = to_u8(ev.get_number(e, &toks[first + 2]));
        args = elements(e, &ev, &toks, first + 3);
    }
    lambda::save_variables(e, call);
    lambda::save_variables(e, condition);
    let old_time = state(e).game_seconds;
    let w = While {
        call,
        condition,
        this_obj: ev.this_obj,
        serial: next_serial(e),
        flags,
        args,
        interval,
        old_time,
    };
    match kind {
        WhileKind::While => state(e).while_.push(w),
        WhileKind::When => state(e).when.push(w),
        WhileKind::PerSeconds => state(e).while_per_seconds.push(w),
    }
    true
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Script.cpp:
// ExtractCallWhileInfo_OLD.
fn add_while_old(e: &mut Engine, a: &CommandArgs, when: bool) -> bool {
    let read = crate::commands::Read::of(e, a);
    if !read.ok {
        return false;
    }
    let (call, condition) = (read.form(0), read.form(1));
    for f in [call, condition] {
        if f == 0 || e.mem.u8(f + 4) != 0x11 {
            return false;
        }
    }
    lambda::save_variables(e, call);
    lambda::save_variables(e, condition);
    let w = While {
        call,
        condition,
        this_obj: a.this_obj,
        serial: next_serial(e),
        flags: 0,
        args: Vec::new(),
        interval: 0.0,
        old_time: 0.0,
    };
    if when {
        state(e).when.push(w);
    } else {
        state(e).while_.push(w);
    }
    true
}

fn result_of(e: &mut Engine, w: &[u32], f: impl FnOnce(&mut Engine, &CommandArgs) -> bool) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    let ok = f(e, &a);
    a.set_result(e, f64::from(u8::from(ok)));
    handled(true)
}

fn call_after(e: &mut Engine, w: &[u32]) -> Ret {
    result_of(e, w, |e, a| add_after(e, a, AfterKind::Seconds))
}
fn call_after_frames(e: &mut Engine, w: &[u32]) -> Ret {
    result_of(e, w, |e, a| add_after(e, a, AfterKind::Frames))
}
fn call_for(e: &mut Engine, w: &[u32]) -> Ret {
    result_of(e, w, |e, a| add_after(e, a, AfterKind::For))
}
fn call_while(e: &mut Engine, w: &[u32]) -> Ret {
    result_of(e, w, |e, a| add_while(e, a, WhileKind::While))
}
fn call_when(e: &mut Engine, w: &[u32]) -> Ret {
    result_of(e, w, |e, a| add_while(e, a, WhileKind::When))
}
fn call_while_per_seconds(e: &mut Engine, w: &[u32]) -> Ret {
    result_of(e, w, |e, a| add_while(e, a, WhileKind::PerSeconds))
}
fn call_after_old(e: &mut Engine, w: &[u32]) -> Ret {
    result_of(e, w, |e, a| add_after_old(e, a, false))
}
fn call_for_old(e: &mut Engine, w: &[u32]) -> Ret {
    result_of(e, w, |e, a| add_after_old(e, a, true))
}
fn call_while_old(e: &mut Engine, w: &[u32]) -> Ret {
    result_of(e, w, |e, a| add_while_old(e, a, false))
}
fn call_when_old(e: &mut Engine, w: &[u32]) -> Ret {
    result_of(e, w, |e, a| add_while_old(e, a, true))
}

/// Installs the tick's handler.
pub fn install(e: &mut Engine) {
    e.on_site(TICK, tick);
}

#[cfg(test)]
#[path = "tick_tests.rs"]
mod tests;

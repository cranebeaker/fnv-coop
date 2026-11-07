//! `NV_PROFILE=1`: a sampling profiler of the running game (a diagnostic of the host, not exe
//! behaviour; nothing here changes what a translated function does).
//!
//! An OS thread of its own looks, about a thousand times a second, at:
//! - the **phase**: the game's code holds the engine ([`Phase::Game`]); the engine is being handed
//!   from one of the thread host's OS threads to another ([`Phase::Switch`], `thread_host`'s
//!   channel hand-offs); the game's code waits on the real clock ([`Phase::Sleep`]: a `Sleep` or a
//!   wait the host really sleeps through, `Engine::wait_clock_until_us`); or none of the game's
//!   code runs ([`Phase::Viewer`]: between two of the game's frames, the viewer's own work);
//! - the **game thread** holding the engine (1 is main);
//! - the **exe-address call stack** of the functions entered through `Engine::call`, mirrored
//!   into a lock-free shadow stack ([`push`], [`pop`], [`resync`]);
//! - on 64-bit Windows, the **instruction pointer** of the OS thread holding the engine (between
//!   the game's frames, of the thread that runs them: the viewer's), named through `dbghelp` when
//!   the table is printed (the program's private symbols when its PDB has them, else its public
//!   ones; a time inside an inlined helper counts under the function it was inlined into).
//!
//! - on 64-bit Windows, in the game's phases, the **native call stack** of that OS thread, walked
//!   while it is suspended (`NV_PROFILE_STACK` frames, default 64, 0 off): each image's own
//!   unwind table (`.pdata`, binary-searched here: `RtlLookupFunctionEntry` can take the loader's
//!   lock, which the suspended thread may hold) and `RtlVirtualUnwind`, which checks no bounds:
//!   before each step the walk goes over the stack reads the unwind will make (from the
//!   function's unwind codes, or its epilogue's instructions) and ends as a bad frame if one
//!   would leave the thread's stack, or at a return into code with no unwind data (only the
//!   innermost function can be a leaf). `RtlVirtualUnwind` takes no lock except at an epilogue
//!   that ends in a jump out of its function's range (a tail call), whose target it looks up to
//!   tell a tail call from a jump to another part of the same function: such a frame the walk
//!   unwinds itself (its own lookup decides, then the epilogue's pops and return are emulated),
//!   or ends the walk when the target is the same function. Nothing allocates while the thread
//!   is suspended.
//!   Each code address's lookup is kept for the next walks. 2026-11-01, Doc's room, 51 frames
//!   a walk (before the read checks): the thread stood suspended
//!   67 us a walk without the kept lookups, 51 us with them, and 36 us with one frame
//!   (`NV_PROFILE_STACK=1`: suspending it and reading its registers, which every sample pays).
//!   Not measured in Doc's room since; on a thread 56 frames of plain functions deep
//!   (2026-11-02, a standalone build of this module, three runs) a walk with the read checks kept
//!   it suspended 14.4-15.3 us (median) against 12.9-14.6 us without them.
//!
//! Every call through `Engine::call` (and `Engine::vcall`, which dispatches through the same body)
//! is also counted by its exe address ([`count_target`]: an open-addressed table hashed like the
//! call cache, written only by the thread holding the engine; `NV_PROFILE_CALLS=0` turns it off).
//! With `NV_PROFILE_THREAD_CALLS=<file>` each target's calls are also counted by game thread, and the
//! table of where each function runs (main only, the other game threads only, both) goes to the file
//! ([`thread_calls_tsv`]).
//!
//! The profiler on, every call takes `Engine::call_full` (the shadow stack, the count): calls cost
//! more than in a run without it, and `call_full` stands in the native tables for the fast path
//! `Engine::call` inlines into its callers in such a run.
//!
//! Every `NV_PROFILE_SECONDS` (default 10) it prints the phases, the game threads, where the game's
//! own time goes by kind (exe code, the Direct3D 9 / DirectSound / DirectInput platform objects,
//! imported functions by name), the top `NV_PROFILE_TOP` (default 25) exe functions by self and
//! inclusive samples, the top native functions in the game's and the viewer's time, the native
//! functions by inclusive samples and the callers of the 15 busiest (from the walked stacks), and
//! the top `NV_PROFILE_CALLS` (default 60) call targets with their calls a frame.
//!
//! Off (the default), each `Engine::call` pays one relaxed load of [`on`].

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, AtomicU8, Ordering::Relaxed};
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};

/// The most native frames a walk records (`NV_PROFILE_STACK` is capped at it).
pub const MAX_FRAMES: usize = 256;

/// Why a native stack walk ended.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum WalkEnd {
    /// The thread's first function (its return address is 0).
    Start = 0,
    /// The frame limit (`NV_PROFILE_STACK`).
    Limit = 1,
    /// Code in no image the sampler knows (an image loaded since its last look, or no image).
    Unknown = 2,
    /// A frame that would leave the thread's stack (or whose unwind would read outside it), or
    /// not move up it; or a return into code with no unwind data, which only the innermost
    /// function may lack.
    Bad = 3,
}

impl WalkEnd {
    /// Each end with its name in the table.
    pub const ALL: [(&'static str, WalkEnd); 4] = [
        ("the thread's start", WalkEnd::Start),
        ("the frame limit", WalkEnd::Limit),
        ("unknown code", WalkEnd::Unknown),
        ("a bad frame", WalkEnd::Bad),
    ];
}

/// One native stack walk ([`native::Sampler::walk`]).
#[derive(Debug, Clone, Copy)]
pub struct Walk {
    /// The thread's instruction pointer.
    pub rip: u64,
    /// The frames recorded (`Sampler::keys`).
    pub frames: usize,
    pub end: WalkEnd,
    /// How long the thread stood suspended.
    pub suspended_ns: u64,
}

/// Whether the profiler is on (`NV_PROFILE` set to anything but `0` or empty), read once by
/// [`init_from_env`].
static ON: AtomicBool = AtomicBool::new(false);

/// How deep the shadow stack records (deeper entries are counted but not kept).
pub const MAX_DEPTH: usize = 256;

#[allow(clippy::declare_interior_mutable_const)]
const ZERO: AtomicU32 = AtomicU32::new(0);
static STACK: [AtomicU32; MAX_DEPTH] = [ZERO; MAX_DEPTH];
static DEPTH: AtomicU32 = AtomicU32::new(0);
static PHASE: AtomicU8 = AtomicU8::new(Phase::Viewer as u8);
/// The game thread holding the engine (`thread_host::GameThread::id`).
static GAME_THREAD: AtomicU32 = AtomicU32::new(1);
/// The OS thread holding the engine (Windows thread id; 0 unknown).
static OS_THREAD: AtomicU32 = AtomicU32::new(0);
/// The game's frames ended so far (`thread_host::run_main_frame`).
static FRAMES: AtomicU64 = AtomicU64::new(0);
/// Calls through `Engine::call` so far (profiler on only; only the thread holding the engine
/// writes it, so a plain load and store).
static CALLS: AtomicU64 = AtomicU64::new(0);
/// Of those, the ones `Engine::call`'s direct-mapped cache did not answer (the same writer).
static CACHE_MISSES: AtomicU64 = AtomicU64::new(0);

/// How many slots the per-target call counts have (a power of two, twice the call cache's: a
/// run calls some tens of thousands of distinct functions).
const TARGET_SLOTS: usize = 1 << 17;
/// How far [`count_target`] probes past an address's slot before counting the call as
/// uncounted.
const TARGET_PROBES: usize = 64;
#[allow(clippy::declare_interior_mutable_const)]
const ZERO64: AtomicU64 = AtomicU64::new(0);
/// The per-target call counts: the exe address in a slot (0 empty) and its calls so far. Only
/// the thread holding the engine writes them (plain loads and stores); the sampler reads them.
static TARGET_ADDR: [AtomicU32; TARGET_SLOTS] = [ZERO; TARGET_SLOTS];
static TARGET_CALLS: [AtomicU64; TARGET_SLOTS] = [ZERO64; TARGET_SLOTS];
/// Calls [`count_target`] found no slot for (address 0, or the probes ran out).
static TARGET_UNCOUNTED: AtomicU64 = AtomicU64::new(0);
/// `NV_PROFILE_THREAD_CALLS=<file>` (with `NV_PROFILE=1`): [`count_target`] also counts each target's
/// calls by the game thread holding the engine, and the table ([`thread_calls_tsv`], written to the
/// file with every profile table and by `NV_CENSUS`) says which functions run on main only, on the other game threads only, or on both (the typed migration's
/// float rule picks the precision by that, ADR-0014). Off, [`count_target`] pays one relaxed load.
static COUNT_THREADS: AtomicBool = AtomicBool::new(false);
/// Per target slot: calls on main, calls on the other game threads, and a bit per game thread id
/// that called it (ids of 63 and more share the top bit).
static TARGET_MAIN: [AtomicU64; TARGET_SLOTS] = [ZERO64; TARGET_SLOTS];
static TARGET_OTHER: [AtomicU64; TARGET_SLOTS] = [ZERO64; TARGET_SLOTS];
static TARGET_THREADS: [AtomicU64; TARGET_SLOTS] = [ZERO64; TARGET_SLOTS];

/// `NV_PROFILE_THREAD_CALLS`: the file the per-thread table goes to.
fn thread_calls_file() -> Option<&'static str> {
    static FILE: OnceLock<Option<String>> = OnceLock::new();
    FILE.get_or_init(|| {
        std::env::var("NV_PROFILE_THREAD_CALLS")
            .ok()
            .filter(|v| !v.is_empty() && v != "0")
    })
    .as_deref()
}

/// Whether calls are counted by thread too (`NV_PROFILE_THREAD_CALLS`).
pub fn counting_threads() -> bool {
    on() && COUNT_THREADS.load(Relaxed)
}

/// Counts a call in slot `slot` for the game thread holding the engine.
#[inline]
fn count_thread(slot: usize) {
    let id = GAME_THREAD.load(Relaxed);
    let n = if id == crate::thread_host::MAIN_THREAD_ID {
        &TARGET_MAIN[slot]
    } else {
        &TARGET_OTHER[slot]
    };
    n.store(n.load(Relaxed).wrapping_add(1), Relaxed);
    let bit = 1u64 << id.min(63);
    let t = &TARGET_THREADS[slot];
    let held = t.load(Relaxed);
    if held & bit == 0 {
        t.store(held | bit, Relaxed);
    }
}

/// Each target counted by thread so far: (address, calls on main, calls elsewhere, thread bits).
pub fn thread_calls() -> Vec<(u32, u64, u64, u64)> {
    let mut v: Vec<_> = (0..TARGET_SLOTS)
        .filter_map(|s| {
            let a = TARGET_ADDR[s].load(Relaxed);
            let (m, o) = (TARGET_MAIN[s].load(Relaxed), TARGET_OTHER[s].load(Relaxed));
            (a != 0 && m + o > 0).then(|| (a, m, o, TARGET_THREADS[s].load(Relaxed)))
        })
        .collect();
    v.sort_by(|a, b| (b.1 + b.2).cmp(&(a.1 + a.2)).then(a.0.cmp(&b.0)));
    v
}

/// Game thread id -> its start address, as the thread host makes them ([`name_thread`]).
fn thread_starts() -> &'static Mutex<std::collections::BTreeMap<u32, u32>> {
    static STARTS: OnceLock<Mutex<std::collections::BTreeMap<u32, u32>>> = OnceLock::new();
    STARTS.get_or_init(Default::default)
}

/// Game thread `id` starts at exe address `start` (named in `threads.tsv`).
pub fn name_thread(id: u32, start: u32) {
    if !on() {
        return;
    }
    if let Ok(mut s) = thread_starts().lock() {
        s.insert(id, start);
    }
}

/// [`thread_calls`] as a table: the game threads' starts, then one row a target with its calls on
/// main and elsewhere, the threads, and where it runs (`main`, `other` or `both`).
pub fn thread_calls_tsv() -> String {
    use std::fmt::Write;
    let mut out = String::from("# game threads (id start):");
    if let Ok(s) = thread_starts().lock() {
        for (id, start) in s.iter() {
            let _ = write!(out, " {id} {start:08x};");
        }
    }
    out.push_str("\naddr\tcalls\tmain\tother\tthreads\twhere\n");
    for (a, m, o, bits) in thread_calls() {
        let ids: Vec<String> = (0..64)
            .filter(|i| bits >> i & 1 != 0)
            .map(|i| {
                if i == 63 {
                    "63+".to_string()
                } else {
                    i.to_string()
                }
            })
            .collect();
        let place = match (m > 0, o > 0) {
            (true, false) => "main",
            (false, true) => "other",
            _ => "both",
        };
        let _ = writeln!(
            out,
            "{a:08x}\t{}\t{m}\t{o}\t{}\t{place}",
            m + o,
            ids.join(",")
        );
    }
    out
}

/// Whether [`push`] counts calls by target: from [`start`] on, unless `NV_PROFILE_CALLS=0` (the
/// count costs the profiled run about a millisecond a frame, inside `Engine::call_full`).
static COUNT_TARGETS: AtomicBool = AtomicBool::new(false);

/// What the engine's OS threads are doing (module documentation).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Phase {
    Viewer = 0,
    Game = 1,
    Switch = 2,
    Sleep = 3,
}

const PHASES: [(&str, Phase); 4] = [
    ("game", Phase::Game),
    ("viewer", Phase::Viewer),
    ("switch", Phase::Switch),
    ("sleep", Phase::Sleep),
];

impl Phase {
    fn from_u8(v: u8) -> Phase {
        match v {
            1 => Phase::Game,
            2 => Phase::Switch,
            3 => Phase::Sleep,
            _ => Phase::Viewer,
        }
    }
}

/// Whether the profiler is on.
#[inline(always)]
pub fn on() -> bool {
    ON.load(Relaxed)
}

/// Reads `NV_PROFILE` once (each `Engine::new` asks; only the first reads it).
pub fn init_from_env() {
    static READ: OnceLock<()> = OnceLock::new();
    READ.get_or_init(|| {
        let wanted = std::env::var("NV_PROFILE").is_ok_and(|v| !v.is_empty() && v != "0");
        ON.store(wanted, Relaxed);
        COUNT_THREADS.store(wanted && thread_calls_file().is_some(), Relaxed);
    });
}

/// A function entered through `Engine::call` (profiler on only).
#[inline]
pub fn push(addr: u32) {
    CALLS.store(CALLS.load(Relaxed).wrapping_add(1), Relaxed);
    if COUNT_TARGETS.load(Relaxed) {
        count_target(addr);
    }
    let d = DEPTH.load(Relaxed);
    if let Some(slot) = STACK.get(d as usize) {
        slot.store(addr, Relaxed);
    }
    DEPTH.store(d.wrapping_add(1), Relaxed);
}

/// The first slot [`count_target`] tries for `addr` (Fibonacci hashing, as `Engine::call`'s
/// cache).
#[inline(always)]
const fn target_slot(addr: u32) -> usize {
    (addr.wrapping_mul(0x9e37_79b1) >> (32 - TARGET_SLOTS.trailing_zeros())) as usize
}

/// One more call to exe address `addr` (profiler on only; [`push`]): its slot's count goes up,
/// or the first empty slot from its hash on becomes its own.
#[inline]
pub fn count_target(addr: u32) {
    let mut slot = target_slot(addr);
    if addr != 0 {
        for _ in 0..TARGET_PROBES {
            let held = TARGET_ADDR[slot].load(Relaxed);
            if held == addr {
                let n = &TARGET_CALLS[slot];
                n.store(n.load(Relaxed).wrapping_add(1), Relaxed);
                if COUNT_THREADS.load(Relaxed) {
                    count_thread(slot);
                }
                return;
            }
            if held == 0 {
                TARGET_CALLS[slot].store(1, Relaxed);
                TARGET_ADDR[slot].store(addr, Relaxed);
                if COUNT_THREADS.load(Relaxed) {
                    count_thread(slot);
                }
                return;
            }
            slot = (slot + 1) & (TARGET_SLOTS - 1);
        }
    }
    TARGET_UNCOUNTED.store(TARGET_UNCOUNTED.load(Relaxed).wrapping_add(1), Relaxed);
}

/// Every call target counted so far, with its calls.
fn target_counts() -> HashMap<u32, u64> {
    TARGET_ADDR
        .iter()
        .zip(&TARGET_CALLS)
        .filter_map(|(a, n)| {
            let a = a.load(Relaxed);
            (a != 0).then(|| (a, n.load(Relaxed)))
        })
        .collect()
}

/// The calls of each target from `before` to `now` (two [`target_counts`]), targets with none
/// left out.
fn target_calls_since(now: &HashMap<u32, u64>, before: &HashMap<u32, u64>) -> HashMap<u32, u64> {
    now.iter()
        .filter_map(|(&a, &n)| {
            let d = n.wrapping_sub(before.get(&a).copied().unwrap_or(0));
            (d > 0).then_some((a, d))
        })
        .collect()
}

/// `Engine::call`'s cache did not have the callee (profiler on only).
#[inline]
pub fn cache_miss() {
    CACHE_MISSES.store(CACHE_MISSES.load(Relaxed).wrapping_add(1), Relaxed);
}

/// The innermost function returned (profiler on only).
#[inline]
pub fn pop() {
    let d = DEPTH.load(Relaxed);
    DEPTH.store(d.saturating_sub(1), Relaxed);
}

/// The shadow stack becomes `stack` (the thread host swapped call stacks, or a stop unwound).
pub fn resync(stack: &[u32]) {
    if on() {
        fill(stack);
    }
}

fn fill(stack: &[u32]) {
    for (slot, &addr) in STACK.iter().zip(stack) {
        slot.store(addr, Relaxed);
    }
    DEPTH.store(stack.len() as u32, Relaxed);
}

/// Sets the phase, returning the one before (profiler on only; `Viewer` otherwise).
#[inline]
pub fn set_phase(phase: Phase) -> Phase {
    if !on() {
        return Phase::Viewer;
    }
    Phase::from_u8(PHASE.swap(phase as u8, Relaxed))
}

/// The game thread holding the engine.
pub fn game_thread() -> u32 {
    GAME_THREAD.load(Relaxed)
}

/// The calling OS thread now holds the engine and runs game thread `game_thread`'s code.
pub fn holding(game_thread: u32) {
    if !on() {
        return;
    }
    GAME_THREAD.store(game_thread, Relaxed);
    OS_THREAD.store(native::current_thread_id(), Relaxed);
    PHASE.store(Phase::Game as u8, Relaxed);
}

/// The calling OS thread (the host's, which runs the game's frames) has the engine back between
/// two frames: the viewer's time from now.
pub fn back_to_host() {
    if !on() {
        return;
    }
    OS_THREAD.store(native::current_thread_id(), Relaxed);
    PHASE.store(Phase::Viewer as u8, Relaxed);
    FRAMES.fetch_add(1, Relaxed);
}

/// The import slot `slot` is `dll!name` (named in the table).
pub fn name_import(slot: u32, dll: &str, name: &str) {
    if !on() {
        return;
    }
    if let Ok(mut names) = import_names().lock() {
        names.insert(slot, format!("{dll}!{name}"));
    }
}

fn import_names() -> &'static Mutex<HashMap<u32, String>> {
    static NAMES: OnceLock<Mutex<HashMap<u32, String>>> = OnceLock::new();
    NAMES.get_or_init(Default::default)
}

/// Starts the sampling thread once (profiler on only).
pub fn start() {
    if !on() {
        return;
    }
    static STARTED: OnceLock<()> = OnceLock::new();
    STARTED.get_or_init(|| {
        let period = table_period(std::env::var("NV_PROFILE_SECONDS").ok().as_deref());
        let top = std::env::var("NV_PROFILE_TOP")
            .ok()
            .and_then(|v| v.parse::<usize>().ok())
            .unwrap_or(25);
        let calls_top = std::env::var("NV_PROFILE_CALLS")
            .ok()
            .and_then(|v| v.parse::<usize>().ok())
            .unwrap_or(60);
        let depth = std::env::var("NV_PROFILE_STACK")
            .ok()
            .and_then(|v| v.parse::<usize>().ok())
            .unwrap_or(64)
            .min(MAX_FRAMES);
        println!(
            "NV_PROFILE: sampling every millisecond, a table every {} s, \
             native stacks to {depth} frames, the top {calls_top} call targets",
            period.as_secs_f64()
        );
        COUNT_TARGETS.store(calls_top > 0, Relaxed);
        let settings = Settings {
            period,
            top,
            calls_top,
            depth,
        };
        let _ = std::thread::Builder::new()
            .name("nv-profile".into())
            .spawn(move || sampler(settings));
    });
}

/// How often a table is printed: `NV_PROFILE_SECONDS` (`value`) when it is a number of seconds
/// above 0 that a [`Duration`] holds, else 10 s. (`Duration::from_secs_f64` panics on `inf`, or
/// on 1e20: the panic of the viewer's first game frame, which starts the sampler.)
fn table_period(value: Option<&str>) -> Duration {
    value
        .and_then(|v| v.parse::<f64>().ok())
        .filter(|s| *s > 0.0)
        .and_then(|s| Duration::try_from_secs_f64(s).ok())
        .unwrap_or(Duration::from_secs(10))
}

/// What the environment asks of the sampler ([`start`]).
#[derive(Clone, Copy)]
struct Settings {
    /// How often a table is printed (`NV_PROFILE_SECONDS`).
    period: Duration,
    /// Rows of the function tables (`NV_PROFILE_TOP`).
    top: usize,
    /// Rows of the call-target table (`NV_PROFILE_CALLS`).
    calls_top: usize,
    /// Native frames walked a sample (`NV_PROFILE_STACK`, 0 none).
    depth: usize,
}

/// What the platform address `addr` is, by the ranges the platform layer puts its objects' code
/// at (`units::platform`), or `None` for the exe's own code.
pub fn kind_of(addr: u32) -> Option<String> {
    let kind = match addr {
        0x7e02_0000..=0x7e03_ffff => "D3D9/D3DX platform object",
        0x7e00_0000..=0x7e00_03ff => "DirectInput platform object",
        0x7e00_0400..=0x7e00_07ff => "DirectSound platform object",
        0x7e00_0800..=0x7e00_ffff => "DirectShow platform object",
        0x7e01_0000..=0x7e01_ffff => "loader platform procedure",
        0x7e04_0000..=0x7e04_ffff => "NVAPI platform object",
        0x7e05_0000..=0x7fff_ffff => "other platform object",
        _ => {
            return import_names()
                .lock()
                .ok()
                .and_then(|n| n.get(&addr).map(|name| format!("import {name}")));
        }
    };
    Some(kind.to_string())
}

/// The samples of one table.
#[derive(Default)]
struct Table {
    samples: u64,
    phases: [u64; 4],
    threads: HashMap<u32, u64>,
    /// Game-phase samples by the innermost function's kind.
    kinds: HashMap<String, u64>,
    selfs: HashMap<u32, u64>,
    inclusive: HashMap<u32, u64>,
    /// Native instruction pointers: in the game's phases (game, switch, sleep) and the viewer's.
    native_game: HashMap<u64, u64>,
    native_viewer: HashMap<u64, u64>,
    /// Samples with no stack in the game phase (the host's own code between calls).
    no_stack: u64,
    /// Sleep-phase samples by the innermost exe calls (up to four, innermost first): who waits on
    /// the real clock.
    sleeps: HashMap<[u32; 4], u64>,
    /// `NV_PROFILE_THREADS=1`: every other thread of the process, every fourth tick: samples
    /// by thread, and by (thread, instruction pointer).
    other_samples: HashMap<u32, u64>,
    others: HashMap<(u32, u64), u64>,
    /// The native stacks walked in the game's phases: the function of each frame, innermost
    /// first (`native::Sampler::walk`), and its samples.
    stacks: HashMap<Box<[u64]>, u64>,
    /// The walks: how many, their frames in all, how each ended (by [`WalkEnd`]) and the time
    /// the walked thread stood suspended, in nanoseconds.
    walks: u64,
    walk_frames: u64,
    walk_ends: [u64; 4],
    suspended_ns: u64,
    /// The calls through `Engine::call` in this table's frames by exe address, and the calls not
    /// counted by address (filled when the table is printed).
    targets: HashMap<u32, u64>,
    targets_uncounted: u64,
    /// All calls through `Engine::call` in this table's frames.
    calls: u64,
}

fn sampler(settings: Settings) {
    let Settings {
        period,
        top,
        calls_top,
        depth: stack_depth,
    } = settings;
    let mut table = Table::default();
    let mut started = Instant::now();
    let mut frames_at = FRAMES.load(Relaxed);
    let mut calls_at = CALLS.load(Relaxed);
    let mut misses_at = CACHE_MISSES.load(Relaxed);
    let mut targets_at = target_counts();
    let mut uncounted_at = TARGET_UNCOUNTED.load(Relaxed);
    let mut stack = Vec::with_capacity(MAX_DEPTH);
    let mut seen: Vec<u32> = Vec::with_capacity(MAX_DEPTH);
    let mut sampler = native::Sampler::new();
    let all_threads = std::env::var("NV_PROFILE_THREADS").is_ok_and(|v| v == "1");
    let mut tick = 0u64;
    let mut others: Vec<u32> = Vec::new();
    let mut others_at: Option<Instant> = None;
    let mut symbols_noted = false;
    loop {
        sampler.wait_tick();
        tick += 1;
        let phase = Phase::from_u8(PHASE.load(Relaxed));
        let os_thread = OS_THREAD.load(Relaxed);
        // In the game's phases the thread's native stack is walked too (its first frame is the
        // instruction pointer); the viewer's phase reads the instruction pointer only.
        let walk = (phase != Phase::Viewer && stack_depth > 0)
            .then(|| sampler.walk(os_thread, stack_depth))
            .flatten();
        let rip = match &walk {
            Some(w) => Some(w.rip),
            None => sampler.instruction_pointer(os_thread),
        };
        if let Some(w) = walk {
            table.walks += 1;
            table.walk_frames += w.frames as u64;
            table.walk_ends[w.end as usize] += 1;
            table.suspended_ns += w.suspended_ns;
            let keys = sampler.keys(w.frames);
            match table.stacks.get_mut(keys) {
                Some(n) => *n += 1,
                None if !keys.is_empty() => {
                    table.stacks.insert(keys.into(), 1);
                }
                None => {}
            }
        }
        let game_thread = GAME_THREAD.load(Relaxed);
        let depth = DEPTH.load(Relaxed) as usize;
        stack.clear();
        stack.extend(STACK.iter().take(depth).map(|s| s.load(Relaxed)));

        table.samples += 1;
        table.phases[phase as usize] += 1;
        if all_threads && tick % 4 == 0 {
            if others_at.map_or(true, |t| t.elapsed() >= Duration::from_secs(1)) {
                others = sampler.process_threads();
                others_at = Some(Instant::now());
            }
            for &id in &others {
                if id == os_thread {
                    continue;
                }
                *table.other_samples.entry(id).or_default() += 1;
                if let Some(rip) = sampler.instruction_pointer(id) {
                    *table.others.entry((id, rip)).or_default() += 1;
                }
            }
        }
        if phase == Phase::Viewer {
            if let Some(rip) = rip {
                *table.native_viewer.entry(rip).or_default() += 1;
            }
        } else {
            *table.threads.entry(game_thread).or_default() += 1;
            if let Some(rip) = rip {
                *table.native_game.entry(rip).or_default() += 1;
            }
        }
        if phase == Phase::Sleep {
            let mut chain = [0u32; 4];
            for (slot, &a) in chain.iter_mut().zip(stack.iter().rev()) {
                *slot = a;
            }
            *table.sleeps.entry(chain).or_default() += 1;
        }
        if phase == Phase::Game {
            match stack.last() {
                Some(&inner) => {
                    let kind = kind_of(inner).unwrap_or_else(|| "exe code".into());
                    *table.kinds.entry(kind).or_default() += 1;
                    *table.selfs.entry(inner).or_default() += 1;
                    seen.clear();
                    for &a in &stack {
                        if !seen.contains(&a) {
                            seen.push(a);
                            *table.inclusive.entry(a).or_default() += 1;
                        }
                    }
                }
                None => table.no_stack += 1,
            }
        }

        let elapsed = started.elapsed();
        if elapsed >= period {
            let frames = FRAMES.load(Relaxed);
            let calls = CALLS.load(Relaxed).wrapping_sub(calls_at);
            let targets_now = target_counts();
            let uncounted_now = TARGET_UNCOUNTED.load(Relaxed);
            table.targets = target_calls_since(&targets_now, &targets_at);
            table.targets_uncounted = uncounted_now.wrapping_sub(uncounted_at);
            table.calls = calls;
            if !symbols_noted {
                symbols_noted = true;
                println!("NV_PROFILE: {}", sampler.symbol_source());
            }
            let text = report(
                &table,
                elapsed,
                frames - frames_at,
                Rows {
                    top,
                    calls: calls_top,
                },
                &mut sampler,
            );
            // The frame numbers (the host's count of main's frames) place the table in a run.
            let per_frame = calls / (frames - frames_at).max(1);
            let misses =
                CACHE_MISSES.load(Relaxed).wrapping_sub(misses_at) / (frames - frames_at).max(1);
            println!(
                "== NV_PROFILE: the game's frames {frames_at} to {frames}, \
                 {per_frame} calls through Engine::call a frame ({misses} not in its cache)\n{text}"
            );
            if let (Some(file), true) = (thread_calls_file(), counting_threads()) {
                if let Err(e) = std::fs::write(file, thread_calls_tsv()) {
                    println!("NV_PROFILE_THREAD_CALLS: {file}: {e}");
                }
            }
            table = Table::default();
            started = Instant::now();
            frames_at = FRAMES.load(Relaxed);
            calls_at = CALLS.load(Relaxed);
            misses_at = CACHE_MISSES.load(Relaxed);
            targets_at = target_counts();
            uncounted_at = TARGET_UNCOUNTED.load(Relaxed);
        }
    }
}

fn pct(n: u64, of: u64) -> f64 {
    if of == 0 {
        0.0
    } else {
        100.0 * n as f64 / of as f64
    }
}

fn sorted<K: Copy + Ord>(map: &HashMap<K, u64>) -> Vec<(K, u64)> {
    let mut rows: Vec<(K, u64)> = map.iter().map(|(k, v)| (*k, *v)).collect();
    rows.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    rows
}

/// How many rows a table's lists print.
#[derive(Clone, Copy)]
struct Rows {
    /// The function lists (`NV_PROFILE_TOP`).
    top: usize,
    /// The call-target list (`NV_PROFILE_CALLS`).
    calls: usize,
}

fn report(
    t: &Table,
    elapsed: Duration,
    frames: u64,
    rows: Rows,
    sampler: &mut native::Sampler,
) -> String {
    use std::fmt::Write;
    let top = rows.top;
    let secs = elapsed.as_secs_f64();
    let n = t.samples.max(1);
    let mut out = String::new();
    let fps = frames as f64 / secs;
    let ms_per_frame = |samples: u64| {
        if frames == 0 {
            0.0
        } else {
            1000.0 * secs * (samples as f64 / n as f64) / frames as f64
        }
    };
    let _ = writeln!(
        out,
        "== NV_PROFILE: {secs:.1} s, {frames} game frames ({fps:.1} frames/s, {:.2} ms a frame), {} samples",
        if frames == 0 { 0.0 } else { 1000.0 * secs / frames as f64 },
        t.samples
    );
    let _ = write!(out, "phases:");
    for (name, p) in PHASES {
        let s = t.phases[p as usize];
        let _ = write!(
            out,
            "  {name} {:.1}% ({:.2} ms/frame)",
            pct(s, n),
            ms_per_frame(s)
        );
    }
    let _ = writeln!(out);
    let engine = n - t.phases[Phase::Viewer as usize];
    let _ = write!(out, "game threads (of the game's phases):");
    for (id, s) in sorted(&t.threads) {
        let _ = write!(out, "  {id}: {:.1}%", pct(s, engine));
    }
    let _ = writeln!(out);
    let game = t.phases[Phase::Game as usize];
    let _ = write!(out, "game phase by innermost kind:");
    for (kind, s) in sorted_strings(&t.kinds) {
        let _ = write!(out, "  {kind} {:.1}%", pct(s, game));
    }
    if t.no_stack > 0 {
        let _ = write!(out, "  (no call) {:.1}%", pct(t.no_stack, game));
    }
    let _ = writeln!(out);
    let _ = writeln!(out, "top exe functions, self (% of all samples, ms/frame):");
    for (addr, s) in sorted(&t.selfs).into_iter().take(top) {
        let kind = kind_of(addr).map_or(String::new(), |k| format!("  {k}"));
        let _ = writeln!(
            out,
            "  {:5.1}% {:6.3}  {addr:08x}{kind}",
            pct(s, n),
            ms_per_frame(s)
        );
    }
    let _ = writeln!(
        out,
        "top exe functions, inclusive (% of all samples, ms/frame):"
    );
    for (addr, s) in sorted(&t.inclusive).into_iter().take(top) {
        let kind = kind_of(addr).map_or(String::new(), |k| format!("  {k}"));
        let _ = writeln!(
            out,
            "  {:5.1}% {:6.3}  {addr:08x}{kind}",
            pct(s, n),
            ms_per_frame(s)
        );
    }
    if !t.sleeps.is_empty() {
        let _ = writeln!(
            out,
            "sleep phase by the innermost calls (% of all samples, ms/frame):"
        );
        for (chain, s) in sorted(&t.sleeps).into_iter().take(10) {
            let calls: Vec<String> = chain
                .iter()
                .filter(|&&a| a != 0)
                .map(|a| format!("{a:08x}"))
                .collect();
            let _ = writeln!(
                out,
                "  {:5.1}% {:6.3}  {}",
                pct(s, n),
                ms_per_frame(s),
                calls.join(" < ")
            );
        }
    }
    for (title, map) in [
        ("native, the game's phases", &t.native_game),
        ("native, the viewer's phase", &t.native_viewer),
    ] {
        if map.is_empty() {
            continue;
        }
        let mut by_name: HashMap<String, u64> = HashMap::new();
        let mut by_module: HashMap<String, u64> = HashMap::new();
        for (&rip, &s) in map {
            let (module, symbol) = sampler.name(rip);
            *by_module.entry(module.clone()).or_default() += s;
            *by_name.entry(format!("{module}!{symbol}")).or_default() += s;
        }
        let _ = write!(out, "{title} by module (% of all samples):");
        for (m, s) in sorted_strings(&by_module).into_iter().take(8) {
            let _ = write!(out, "  {m} {:.1}%", pct(s, n));
        }
        let _ = writeln!(out);
        let _ = writeln!(out, "{title}, top functions (% of all samples, ms/frame):");
        for (name, s) in sorted_strings(&by_name).into_iter().take(top) {
            let _ = writeln!(out, "  {:5.1}% {:6.3}  {name}", pct(s, n), ms_per_frame(s));
        }
    }
    if t.walks > 0 {
        native_stacks(&mut out, t, top, &ms_per_frame, &mut |k: u64| {
            let (module, symbol) = sampler.name(k);
            format!("{module}!{symbol}")
        });
    }
    if !t.targets.is_empty() {
        call_targets(&mut out, t, frames, rows.calls, &ms_per_frame);
    }
    if !t.other_samples.is_empty() {
        other_threads(&mut out, t, top, &ms_per_frame, sampler);
    }
    out
}

/// How many of the busiest native functions (by self samples) list their callers.
const CALLER_FUNCTIONS: usize = 15;
/// How many callers each of them lists.
const CALLERS: usize = 8;

/// The walked native stacks' part of a table: how the walks went, the native functions by
/// inclusive samples (a function counted once a sample however often it is on the stack), and
/// the callers of the [`CALLER_FUNCTIONS`] busiest functions by self samples (the nearest frame
/// below the innermost with another name). Functions are told apart by `name_of` their key
/// (several keys can share a name: copies of one generic function, say).
fn native_stacks(
    out: &mut String,
    t: &Table,
    top: usize,
    ms_per_frame: &dyn Fn(u64) -> f64,
    name_of: &mut dyn FnMut(u64) -> String,
) {
    use std::fmt::Write;
    let n = t.samples.max(1);
    let walks = t.walks.max(1);
    let _ = write!(
        out,
        "native stacks, the game's phases: {} walked ({:.1}% of all samples), {:.1} frames on \
         average, the thread suspended {:.1} us a walk; ended at:",
        t.walks,
        pct(t.walks, n),
        t.walk_frames as f64 / walks as f64,
        t.suspended_ns as f64 / walks as f64 / 1000.0,
    );
    for (name, end) in WalkEnd::ALL {
        let _ = write!(
            out,
            "  {name} {:.1}%",
            pct(t.walk_ends[end as usize], walks)
        );
    }
    let _ = writeln!(out);
    // Each key's name, interned: stacks become name ids.
    let mut ids: HashMap<u64, usize> = HashMap::new();
    let mut names: Vec<String> = Vec::new();
    let mut index: HashMap<String, usize> = HashMap::new();
    for stack in t.stacks.keys() {
        for &k in stack.iter() {
            if ids.contains_key(&k) {
                continue;
            }
            let name = name_of(k);
            let id = *index.entry(name.clone()).or_insert_with(|| {
                names.push(name);
                names.len() - 1
            });
            ids.insert(k, id);
        }
    }
    let mut inclusive = vec![0u64; names.len()];
    let mut selfs = vec![0u64; names.len()];
    let mut callers: HashMap<(usize, usize), u64> = HashMap::new();
    // `stamp[id] == pass`: the name was counted for this stack already.
    let mut stamp = vec![0usize; names.len()];
    let none = names.len();
    for (pass, (stack, &s)) in t.stacks.iter().enumerate() {
        let pass = pass + 1;
        let inner = ids[&stack[0]];
        selfs[inner] += s;
        let caller = stack[1..]
            .iter()
            .map(|k| ids[k])
            .find(|&id| id != inner)
            .unwrap_or(none);
        *callers.entry((inner, caller)).or_default() += s;
        for k in stack.iter() {
            let id = ids[k];
            if stamp[id] != pass {
                stamp[id] = pass;
                inclusive[id] += s;
            }
        }
    }
    let by_count = |counts: &[u64]| {
        let mut rows: Vec<(usize, u64)> = counts.iter().copied().enumerate().collect();
        rows.sort_by(|a, b| b.1.cmp(&a.1).then(names[a.0].cmp(&names[b.0])));
        rows
    };
    let _ = writeln!(
        out,
        "native, the game's phases, inclusive (% of all samples, ms/frame):"
    );
    for (id, s) in by_count(&inclusive).into_iter().take(top) {
        let _ = writeln!(
            out,
            "  {:5.1}% {:6.3}  {}",
            pct(s, n),
            ms_per_frame(s),
            names[id]
        );
    }
    let _ = writeln!(
        out,
        "native, the game's phases, callers of the top {CALLER_FUNCTIONS} by self \
         (% of all samples, ms/frame; each caller's share of the function's self time):"
    );
    for (id, s) in by_count(&selfs).into_iter().take(CALLER_FUNCTIONS) {
        if s == 0 {
            break;
        }
        let _ = writeln!(
            out,
            "  {:5.1}% {:6.3}  {}",
            pct(s, n),
            ms_per_frame(s),
            names[id]
        );
        let mut from: Vec<(usize, u64)> = callers
            .iter()
            .filter(|((inner, _), _)| *inner == id)
            .map(|(&(_, caller), &c)| (caller, c))
            .collect();
        from.sort_by(|a, b| {
            b.1.cmp(&a.1).then_with(|| {
                let name = |i: usize| names.get(i).map_or("", String::as_str);
                name(a.0).cmp(name(b.0))
            })
        });
        for (caller, c) in from.into_iter().take(CALLERS) {
            let name = names.get(caller).map_or("(no caller)", String::as_str);
            let _ = writeln!(
                out,
                "        {:5.1}% {:6.3} {:5.1}%  < {name}",
                pct(c, n),
                ms_per_frame(c),
                pct(c, s)
            );
        }
    }
}

/// The call-target part of a table: the calls through `Engine::call` (and `Engine::vcall`) a
/// frame to each of the `rows` most called exe addresses, their share of all calls, and the
/// self time the exe samples give each (ms a frame, and ns a call). With the profiler on every
/// call takes `Engine::call`'s full path, so a call costs more than in a run without it.
fn call_targets(
    out: &mut String,
    t: &Table,
    frames: u64,
    rows: usize,
    ms_per_frame: &dyn Fn(u64) -> f64,
) {
    use std::fmt::Write;
    let frames = frames.max(1) as f64;
    let counted: u64 = t.targets.values().sum();
    let all = t.calls.max(counted).max(1);
    let _ = writeln!(
        out,
        "call targets: {} distinct, {:.0} calls a frame ({:.0} not counted by target); \
         top {rows} (calls a frame, % of calls, self ms/frame, self ns a call, inclusive \
         ms/frame):",
        t.targets.len(),
        all as f64 / frames,
        t.targets_uncounted as f64 / frames,
    );
    for (addr, c) in sorted(&t.targets).into_iter().take(rows) {
        let per_frame = c as f64 / frames;
        let self_ms = ms_per_frame(t.selfs.get(&addr).copied().unwrap_or(0));
        let inclusive_ms = ms_per_frame(t.inclusive.get(&addr).copied().unwrap_or(0));
        let ns_a_call = if per_frame > 0.0 {
            self_ms * 1e6 / per_frame
        } else {
            0.0
        };
        let kind = kind_of(addr).map_or(String::new(), |k| format!("  {k}"));
        let _ = writeln!(
            out,
            "  {per_frame:10.1} {:5.2}% {self_ms:6.3} {ns_a_call:7.1} {inclusive_ms:6.3}  \
             {addr:08x}{kind}",
            pct(c, all),
        );
    }
}

/// The other threads' part of a table (`NV_PROFILE_THREADS=1`): each group of threads (the
/// name without its number) with the CPU time it spent busy (samples not in a wait), and the
/// busiest functions.
fn other_threads(
    out: &mut String,
    t: &Table,
    top: usize,
    ms_per_frame: &dyn Fn(u64) -> f64,
    sampler: &mut native::Sampler,
) {
    use std::fmt::Write;
    // Each thread is sampled every fourth tick: one of its samples stands for four ticks.
    let scale = |s: u64| s * 4;
    let mut groups: HashMap<String, (u64, u64, Vec<u32>)> = HashMap::new();
    let mut group_of: HashMap<u32, String> = HashMap::new();
    for &id in t.other_samples.keys() {
        let group = thread_group(&sampler.thread_name(id));
        group_of.insert(id, group.clone());
        let g = groups.entry(group).or_default();
        g.0 += t.other_samples[&id];
        g.2.push(id);
    }
    let mut busy_functions: HashMap<String, u64> = HashMap::new();
    for (&(id, rip), &s) in &t.others {
        let (module, symbol) = sampler.name(rip);
        if is_waiting(&module, &symbol) {
            continue;
        }
        let group = group_of.get(&id).cloned().unwrap_or_default();
        if let Some(g) = groups.get_mut(&group) {
            g.1 += s;
        }
        *busy_functions
            .entry(format!("{group}: {module}!{symbol}"))
            .or_default() += s;
    }
    let _ = writeln!(
        out,
        "other threads, busy (not waiting), CPU ms a frame (every 4th tick):"
    );
    let mut rows: Vec<_> = groups.into_iter().collect();
    rows.sort_by(|a, b| b.1 .1.cmp(&a.1 .1).then(a.0.cmp(&b.0)));
    for (group, (_, busy, ids)) in rows.into_iter().take(top) {
        let _ = writeln!(
            out,
            "  {:6.3}  {group} ({} thread(s))",
            ms_per_frame(scale(busy)),
            ids.len()
        );
    }
    let _ = writeln!(out, "other threads, busiest functions (CPU ms a frame):");
    for (name, s) in sorted_strings(&busy_functions).into_iter().take(top) {
        let _ = writeln!(out, "  {:6.3}  {name}", ms_per_frame(scale(s)));
    }
}

/// A thread's group: its name without a trailing number (`Compute Task Pool (3)`, `game thread
/// 12`), `unnamed` without a name.
pub fn thread_group(name: &str) -> String {
    let trimmed = name
        .trim_end_matches(|c: char| c.is_ascii_digit() || c == ')' || c == '(' || c == ' ')
        .trim();
    if trimmed.is_empty() {
        "unnamed".into()
    } else {
        trimmed.to_string()
    }
}

/// Whether a sampled function is a wait of the OS (the thread is idle there).
pub fn is_waiting(module: &str, symbol: &str) -> bool {
    (module == "ntdll.dll" || module == "win32u.dll" || module == "kernelbase.dll")
        && [
            "Wait",
            "Delay",
            "RemoveIoCompletion",
            "AlertByThreadId",
            "SignalAndWait",
        ]
        .iter()
        .any(|w| symbol.contains(w))
}

fn sorted_strings(map: &HashMap<String, u64>) -> Vec<(String, u64)> {
    let mut rows: Vec<(String, u64)> = map.iter().map(|(k, v)| (k.clone(), *v)).collect();
    rows.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    rows
}

/// A Rust v0 or legacy symbol made readable enough for the table: the hashes, disambiguators
/// and back references dropped, the identifiers joined with `::` (generic arguments become more
/// path segments). Not a full demangler.
pub fn readable_symbol(raw: &str) -> String {
    let s = raw.split(".llvm.").next().unwrap_or(raw);
    let (body, legacy) = if let Some(rest) = s.strip_prefix("_R") {
        (rest.as_bytes(), false)
    } else if let Some(rest) = s.strip_prefix("_ZN") {
        (rest.as_bytes(), true)
    } else {
        return s.to_string();
    };
    let mut parts: Vec<&str> = Vec::new();
    let mut i = 0;
    while i < body.len() {
        let c = body[i];
        if !legacy && (c == b'C' || c == b's') && i + 1 < body.len() {
            // `Cs<base62>_` (a crate's disambiguator) and `s<base62>_` (a nested one).
            let start = if c == b'C' && body[i + 1] == b's' {
                i + 2
            } else if c == b's' {
                i + 1
            } else {
                i += 1;
                continue;
            };
            let mut j = start;
            while j < body.len() && body[j].is_ascii_alphanumeric() {
                j += 1;
            }
            if j < body.len() && body[j] == b'_' {
                i = j + 1;
                continue;
            }
            i += 1;
            continue;
        }
        if !legacy && c == b'B' {
            // A back reference: `B<base62>_`.
            let mut j = i + 1;
            while j < body.len() && body[j].is_ascii_alphanumeric() {
                j += 1;
            }
            i = if j < body.len() && body[j] == b'_' {
                j + 1
            } else {
                i + 1
            };
            continue;
        }
        if c.is_ascii_digit() && c != b'0' {
            let mut j = i;
            let mut len = 0usize;
            while j < body.len() && body[j].is_ascii_digit() {
                len = len * 10 + usize::from(body[j] - b'0');
                j += 1;
            }
            // v0 puts a `_` between the length and an identifier that starts with a digit or `_`.
            if !legacy && j < body.len() && body[j] == b'_' {
                j += 1;
            }
            if j + len <= body.len() && len > 0 {
                let ident = &s[s.len() - body.len() + j..s.len() - body.len() + j + len];
                // Legacy symbols end with a `17h<hash>` segment.
                if !(legacy && ident.len() == 17 && ident.starts_with('h')) {
                    parts.push(ident);
                }
                i = j + len;
                continue;
            }
        }
        i += 1;
    }
    if parts.is_empty() {
        return s.to_string();
    }
    parts.join("::")
}

#[cfg(all(windows, target_arch = "x86_64"))]
mod native {
    //! The OS thread holding the engine: suspended for a moment, its instruction pointer read
    //! (or its stack walked, [`Sampler::walk`]), resumed (nothing that could take a lock runs
    //! while it is suspended); named through `dbghelp.dll`, loaded when the first table is
    //! printed.
    use super::{Walk, WalkEnd, MAX_FRAMES};
    use std::collections::HashMap;
    use std::ffi::c_void;
    use std::time::{Duration, Instant};

    type Handle = *mut c_void;

    #[link(name = "kernel32")]
    extern "system" {
        fn GetCurrentThreadId() -> u32;
        fn GetCurrentProcess() -> Handle;
        fn OpenThread(access: u32, inherit: i32, id: u32) -> Handle;
        fn SuspendThread(thread: Handle) -> u32;
        fn ResumeThread(thread: Handle) -> u32;
        fn GetThreadContext(thread: Handle, context: *mut Context) -> i32;
        fn CreateWaitableTimerExW(
            attributes: *mut c_void,
            name: *const u16,
            flags: u32,
            access: u32,
        ) -> Handle;
        fn SetWaitableTimer(
            timer: Handle,
            due: *const i64,
            period: i32,
            routine: *mut c_void,
            arg: *mut c_void,
            resume: i32,
        ) -> i32;
        fn WaitForSingleObject(handle: Handle, ms: u32) -> u32;
        fn LoadLibraryA(name: *const u8) -> Handle;
        fn FreeLibrary(module: Handle) -> i32;
        fn GetProcAddress(module: Handle, name: *const u8) -> *mut c_void;
        fn GetModuleHandleExA(flags: u32, address: *const u8, module: *mut Handle) -> i32;
        fn GetModuleFileNameA(module: Handle, name: *mut u8, size: u32) -> u32;
        fn CreateToolhelp32Snapshot(flags: u32, process: u32) -> Handle;
        fn Thread32First(snapshot: Handle, entry: *mut ThreadEntry) -> i32;
        fn Thread32Next(snapshot: Handle, entry: *mut ThreadEntry) -> i32;
        fn CloseHandle(handle: Handle) -> i32;
        fn GetCurrentProcessId() -> u32;
        fn LocalFree(memory: *mut c_void) -> *mut c_void;
        fn GetModuleHandleA(name: *const u8) -> Handle;
        fn GetProcessIdOfThread(thread: Handle) -> u32;
        fn K32EnumProcessModules(
            process: Handle,
            modules: *mut Handle,
            bytes: u32,
            needed: *mut u32,
        ) -> i32;
        fn ReadProcessMemory(
            process: Handle,
            base: *const c_void,
            buffer: *mut c_void,
            size: usize,
            read: *mut usize,
        ) -> i32;
        #[allow(clippy::too_many_arguments)]
        fn RtlVirtualUnwind(
            handler_type: u32,
            image_base: u64,
            control_pc: u64,
            entry: *const RuntimeFunction,
            context: *mut Context,
            handler_data: *mut *mut c_void,
            establisher_frame: *mut u64,
            context_pointers: *mut c_void,
        ) -> *mut c_void;
    }

    /// `NtQueryInformationThread` (ntdll, looked up by name).
    type QueryThread = unsafe extern "system" fn(Handle, u32, *mut c_void, u32, *mut u32) -> i32;

    /// `RUNTIME_FUNCTION`: one function's code range and unwind information (image offsets).
    #[repr(C)]
    #[derive(Clone, Copy)]
    struct RuntimeFunction {
        begin: u32,
        end: u32,
        unwind: u32,
    }

    /// A loaded image and its unwind table (`.pdata`, sorted by `begin`). A listed image holds a
    /// reference to its module (`module`, taken by [`listed_image`]; null: none), let go when
    /// the image is dropped: walks read its unwind table and code in place, so it stays loaded
    /// while it is listed.
    struct Image {
        base: u64,
        end: u64,
        table: *const RuntimeFunction,
        len: usize,
        module: Handle,
    }

    impl Drop for Image {
        fn drop(&mut self) {
            if !self.module.is_null() {
                // SAFETY: the reference `listed_image` took, let go once.
                unsafe { FreeLibrary(self.module) };
            }
        }
    }

    /// `N` bytes of this process's memory at `at`, `None` where any of them is not readable
    /// (`ReadProcessMemory`, which answers a failure there instead of faulting).
    fn read_bytes<const N: usize>(at: u64) -> Option<[u8; N]> {
        let mut bytes = [0u8; N];
        let mut read = 0usize;
        // SAFETY: the buffer is ours and `N` bytes long; the system checks the source range.
        let ok = unsafe {
            ReadProcessMemory(
                GetCurrentProcess(),
                at as *const c_void,
                bytes.as_mut_ptr() as *mut c_void,
                N,
                &mut read,
            )
        };
        (ok != 0 && read == N).then_some(bytes)
    }

    /// The image at `base` (a module handle), read from its PE headers; `None` when they are not
    /// a 64-bit image's, or cannot be read: a module listed by `K32EnumProcessModules` can be
    /// unloaded before its headers are read (another thread's `FreeLibrary`), and reading them
    /// in place then faulted (in a probe, once in 10^4 to 10^5 listings while a thread loaded
    /// and freed a DLL in a loop). The image holds no reference to its module: walks read its
    /// unwind table in place, which only [`listed_image`]'s reference keeps loaded.
    fn image_at(base: u64) -> Option<Image> {
        let dos = read_bytes::<0x40>(base)?;
        let u16_in = |b: &[u8], at: usize| u16::from_le_bytes([b[at], b[at + 1]]);
        let u32_in =
            |b: &[u8], at: usize| u32::from_le_bytes(b[at..at + 4].try_into().expect("4 bytes"));
        if u16_in(&dos, 0) != 0x5a4d {
            return None;
        }
        let nt = u32_in(&dos, 0x3c);
        // "PE\0\0", the 20-byte file header, then the optional header of a PE32+ image up to
        // the exception directory (data directory 3, at 136 in it).
        let headers = read_bytes::<{ 24 + 144 }>(base + u64::from(nt))?;
        if u32_in(&headers, 0) != 0x4550 || u16_in(&headers, 24) != 0x20b {
            return None;
        }
        let optional = 24;
        let size = u64::from(u32_in(&headers, optional + 56));
        let directories = u32_in(&headers, optional + 108);
        let (rva, bytes) = if directories > 3 {
            (
                u32_in(&headers, optional + 136),
                u32_in(&headers, optional + 140),
            )
        } else {
            (0, 0)
        };
        Some(Image {
            base,
            end: base + size,
            table: (base + u64::from(rva)) as *const RuntimeFunction,
            len: if rva == 0 {
                0
            } else {
                bytes as usize / std::mem::size_of::<RuntimeFunction>()
            },
            module: std::ptr::null_mut(),
        })
    }

    /// The image at `base`, a module of a listing, held loaded while the image lives: a
    /// reference to the module is taken first (`GetModuleHandleExA` by address, the count
    /// raised), then its headers are read ([`image_at`]). Walks read its unwind table and code
    /// in place until the next listing; without the reference an image unloaded since the
    /// listing stayed in it, and a walk read its old unwind table (in a probe, a DLL freed after
    /// a listing and another loaded at its base: the walk read past the new module's end, an
    /// access violation in the sampler with the walked thread suspended). `None` when the
    /// module is gone already, or `base` now lies in another module.
    fn listed_image(base: u64) -> Option<Image> {
        let mut module: Handle = std::ptr::null_mut();
        // SAFETY: a module looked up by an address, its reference count raised; one already
        // unloaded answers a failure.
        if unsafe { GetModuleHandleExA(FROM_ADDRESS, base as *const u8, &mut module) } == 0 {
            return None;
        }
        match (module as u64 == base).then(|| image_at(base)).flatten() {
            Some(mut image) => {
                image.module = module;
                Some(image)
            }
            None => {
                // SAFETY: the reference just taken, let go once.
                unsafe { FreeLibrary(module) };
                None
            }
        }
    }

    /// One code address a walk looked up ([`walk_stack`]), kept for the next walks: the stacks of
    /// successive samples share most of their frames, and a lookup in a large image's unwind
    /// table (hundreds of thousands of entries, cold in the cache) cost about a microsecond.
    #[derive(Clone, Copy)]
    struct Found {
        /// The address looked up (0: an empty slot).
        at: u64,
        /// Its image's base and end, its unwind entry (null: none, a leaf function) and its
        /// function's start (the frame's key).
        base: u64,
        end: u64,
        entry: *const RuntimeFunction,
        key: u64,
    }

    impl Found {
        const EMPTY: Found = Found {
            at: 0,
            base: 0,
            end: 0,
            entry: std::ptr::null(),
            key: 0,
        };
    }

    /// How many lookups [`Lookups`] keeps (a power of two; direct-mapped).
    const FOUND_SLOTS: usize = 1 << 12;

    /// The walks' kept lookups ([`Found`]), by a hash of the address.
    struct Lookups {
        slots: [Found; FOUND_SLOTS],
        /// How many lookups were answered from `slots`, how many frames were handed to
        /// `RtlVirtualUnwind`, and how many tail-call epilogues the walk unwound itself (what
        /// the tests count).
        #[cfg(test)]
        hits: u64,
        #[cfg(test)]
        unwinds: u64,
        #[cfg(test)]
        tail_calls: u64,
    }

    impl Lookups {
        fn new() -> Box<Self> {
            Box::new(Lookups {
                slots: [Found::EMPTY; FOUND_SLOTS],
                #[cfg(test)]
                hits: 0,
                #[cfg(test)]
                unwinds: 0,
                #[cfg(test)]
                tail_calls: 0,
            })
        }
    }

    /// The lookup of code address `at` (`None`: in no known image), from `found` or made and
    /// kept there.
    ///
    /// # Safety
    /// The images are loaded.
    unsafe fn look_up(images: &[Image], found: &mut Lookups, at: u64) -> Option<Found> {
        let slot = (at.wrapping_mul(0x9e37_79b9_7f4a_7c15) >> (64 - FOUND_SLOTS.trailing_zeros()))
            as usize;
        if found.slots[slot].at == at {
            #[cfg(test)]
            {
                found.hits += 1;
            }
            return Some(found.slots[slot]);
        }
        let im = image_of(images, at)?;
        let entry = function_entry(im, at);
        let f = Found {
            at,
            base: im.base,
            end: im.end,
            entry: entry.unwrap_or(std::ptr::null()),
            key: entry.map_or(at, |f| function_start(im.base, f)),
        };
        found.slots[slot] = f;
        Some(f)
    }

    /// The image holding `pc`.
    fn image_of(images: &[Image], pc: u64) -> Option<&Image> {
        let i = images.partition_point(|im| im.base <= pc).checked_sub(1)?;
        let im = &images[i];
        (pc < im.end).then_some(im)
    }

    /// The unwind-table entry of the function holding `pc` in `im` (an indirect entry followed),
    /// `None` for a leaf function (it has none).
    ///
    /// # Safety
    /// `im` is a loaded image ([`image_at`]).
    unsafe fn function_entry(im: &Image, pc: u64) -> Option<*const RuntimeFunction> {
        if im.len == 0 {
            return None;
        }
        let rva = (pc - im.base) as u32;
        let table = std::slice::from_raw_parts(im.table, im.len);
        let f = &table[table.partition_point(|f| f.begin <= rva).checked_sub(1)?];
        if rva >= f.end {
            return None;
        }
        let mut f = f as *const RuntimeFunction;
        // `RUNTIME_FUNCTION_INDIRECT`: the entry names another entry.
        if (*f).unwind & 1 != 0 {
            f = (im.base + u64::from((*f).unwind & !1)) as *const RuntimeFunction;
        }
        Some(f)
    }

    /// The start of the function `f` belongs to: a part of a function split up (its unwind
    /// information chained, `UNW_FLAG_CHAININFO`) names the entry before it, back to the first.
    ///
    /// # Safety
    /// `f` is an entry of the image at `base` ([`function_entry`]).
    unsafe fn function_start(base: u64, mut f: *const RuntimeFunction) -> u64 {
        for _ in 0..8 {
            let info = (base + u64::from((*f).unwind & !1)) as *const u8;
            if (*info >> 3) & 0x4 == 0 {
                break;
            }
            let codes = usize::from(*info.add(2));
            f = info.add(4 + 2 * ((codes + 1) & !1)) as *const RuntimeFunction;
        }
        base + u64::from((*f).begin)
    }

    /// The x64 `CONTEXT`'s general registers (`Rax` at 0x78, then `Rcx`, `Rdx`, `Rbx`, `Rsp`,
    /// `Rbp`, `Rsi`, `Rdi`, `R8` to `R15`, in the unwind codes' order) and `Rip` (0xf8).
    impl Context {
        fn word(&self, at: usize) -> u64 {
            u64::from_le_bytes(self.0[at..at + 8].try_into().expect("8 bytes"))
        }
        fn set_word(&mut self, at: usize, v: u64) {
            self.0[at..at + 8].copy_from_slice(&v.to_le_bytes());
        }
        fn register(&self, n: u8) -> u64 {
            self.word(0x78 + 8 * usize::from(n & 0xf))
        }
        fn set_register(&mut self, n: u8, v: u64) {
            self.set_word(0x78 + 8 * usize::from(n & 0xf), v);
        }
        fn rsp(&self) -> u64 {
            self.word(0x98)
        }
        fn rip(&self) -> u64 {
            self.word(0xf8)
        }
    }

    /// `UNW_FLAG_CHAININFO`: the unwind information ends with the entry of the function part it
    /// continues.
    const CHAIN_INFO: u8 = 4;
    /// The most chained parts [`prologue_reads_fit`] follows.
    const CHAIN_LIMIT: usize = 32;

    /// The unwind information of `f`, an entry of the image at `base`.
    ///
    /// # Safety
    /// As [`function_start`].
    unsafe fn unwind_info(base: u64, f: *const RuntimeFunction) -> *const u8 {
        (base + u64::from((*f).unwind & !1)) as *const u8
    }

    /// How the walk unwinds a frame ([`unwind_step`]).
    #[derive(Debug, PartialEq, Eq)]
    enum Step {
        /// Not at all: the walk ends as a bad frame.
        Bad,
        /// Through `RtlVirtualUnwind`: every stack read it makes lies in the stack, and it takes
        /// no lock.
        System,
        /// By itself, at an epilogue that ends in a tail call ([`Epilogue::TailCall`]): the
        /// stack pointer `rsp` after the epilogue's `add rsp` or `lea rsp`, then `pops` pops
        /// from the instruction at `pops_at` on, then the return ([`emulate_epilogue`]).
        TailCall { rsp: u64, pops: u64, pops_at: u64 },
    }

    /// How to unwind the frame of `f` at `pc` with the stack `low..high`. `RtlVirtualUnwind`
    /// reads the saved registers and the return address where the function's unwind codes (or,
    /// in an epilogue, its instructions) put them, and checks no bounds itself: a frame taken
    /// from a stray word (one that code without unwind data pushed, say) can send those reads
    /// past the thread's stack, an access violation in the sampler while the engine's thread
    /// stands suspended. This goes over the same reads without making them, as
    /// `RtlVirtualUnwind` makes them: in an epilogue, the pops it emulates and the return address
    /// after them ([`epilogue_at`]); elsewhere the unwind codes the function has executed at
    /// `pc`, with its chained parts, then the return address ([`prologue_reads_fit`]). Unwind
    /// information of version 2 can also describe the function's epilogues, which the system may
    /// go by instead of the code: there an epilogue found in the code is checked both ways.
    ///
    /// At an epilogue that ends in a jump out of the function's range, `RtlVirtualUnwind` looks
    /// up the jump's target (`RtlLookupFunctionEntry`) to tell a tail call from a jump into
    /// another part of the same function, and that lookup waits on a lock the loader holds while
    /// it loads or frees a DLL: with the engine's thread suspended inside the loader, the
    /// sampler would wait for good (a probe of a thread loading and freeing a DLL in a loop got
    /// stuck 3 times in 3, within 9,000 to 95,000 walks). Such a frame never goes to the system:
    /// the walk looks the target up in its own tables, unwinds a tail call itself (a jump to the
    /// function's own start among them, which the system also takes for one), and ends at a
    /// jump elsewhere in the function. Checked against `RtlVirtualUnwind` (2026-11-01, every
    /// function of 40-odd loaded images at many pcs and stack pointers, nothing suspended): the
    /// emulation read nothing outside the stack in 445K tail-call frames and gave the system's
    /// registers in all but 298, all at pcs inside an instruction (an `e9` or `eb` that is a
    /// ModRM byte) of functions with version 2 unwind information, which the system unwinds by
    /// its epilogue descriptors instead of the code.
    ///
    /// The epilogue decoder ([`epilogue_at`]) follows ntdll 10.0.26200's (disassembled: the
    /// unwinder's core at ntdll+0x1F440), down to its skip of one BND prefix (F2) before the
    /// return or jump. Read as no epilogue, a `bnd jmp` out of its function went to the system,
    /// which looked the target up (in a probe, 10 walks of 30,163 waited on the loader).
    ///
    /// # Safety
    /// `f` is the unwind entry `code` names ([`look_up`]); the images are loaded; `low..high` is
    /// committed memory.
    unsafe fn unwind_step(
        images: &[Image],
        found: &mut Lookups,
        code: &Found,
        f: *const RuntimeFunction,
        pc: u64,
        ctx: &Context,
        (low, high): (u64, u64),
    ) -> Step {
        let fits =
            |at: u64, len: u64| at >= low && at.checked_add(len).is_some_and(|end| end <= high);
        let epilogue_fits = |rsp: u64, pops: u64| fits(rsp, 8 * pops + 8);
        let prologue_fits = || prologue_reads_fit(code.base, f, pc, ctx, &fits);
        let system = |ok: bool| if ok { Step::System } else { Step::Bad };
        let version = *unwind_info(code.base, f) & 7;
        match epilogue_at(code, f, pc, ctx) {
            None => Step::Bad,
            Some(Epilogue::Yes { rsp, pops }) if version == 1 => system(epilogue_fits(rsp, pops)),
            Some(Epilogue::Yes { rsp, pops }) => {
                system(epilogue_fits(rsp, pops) && prologue_fits())
            }
            Some(Epilogue::TailCall {
                rsp,
                pops,
                pops_at,
                target,
            }) => {
                // The system's test (ntdll's, disassembled): the jump stays in the
                // function, and the code is no epilogue, when the target's unwind entry leads
                // (through any chained parts) to the same first part as this one's and the
                // target is not that part's start; a jump to the function's own start is a tail
                // call (recursion). A target with no entry, or in no image the walk knows, is
                // another function.
                let within = look_up(images, found, target).is_some_and(|t| {
                    !t.entry.is_null()
                        && t.base == code.base
                        && t.key == code.key
                        && target != code.key
                });
                if within || !epilogue_fits(rsp, pops) {
                    Step::Bad
                } else {
                    Step::TailCall { rsp, pops, pops_at }
                }
            }
            Some(Epilogue::No) => system(prologue_fits()),
        }
    }

    /// Unwinds a frame at an epilogue that ends in a tail call, as the processor would run it
    /// ([`Step::TailCall`]): each pop from `pops_at` on reads its register from the stack, then
    /// the return address is the instruction pointer and the stack pointer is above it. The
    /// reads lie in the stack and no pop is into the stack pointer ([`unwind_step`],
    /// [`epilogue_at`]).
    ///
    /// # Safety
    /// As [`unwind_step`], for the reads it allowed.
    unsafe fn emulate_epilogue(ctx: &mut Context, mut rsp: u64, pops: u64, mut at: u64) {
        for _ in 0..pops {
            let mut b = *(at as *const u8);
            let mut register = 0;
            if b & 0xf0 == 0x40 {
                register = (b & 1) << 3;
                at += 1;
                b = *(at as *const u8);
            }
            at += 1;
            ctx.set_register(register | (b & 7), std::ptr::read(rsp as *const u64));
            rsp += 8;
        }
        ctx.set_word(0xf8, std::ptr::read(rsp as *const u64));
        ctx.set_word(0x98, rsp + 8);
    }

    /// Whether `RtlVirtualUnwind` takes the code at `pc` for an epilogue ([`epilogue_at`]).
    enum Epilogue {
        /// No: it undoes the prologue's unwind codes.
        No,
        /// Yes: it emulates the rest of the epilogue, `pops` words read from `rsp` on (after the
        /// epilogue's `add rsp` or `lea rsp`) and the return address after them.
        Yes { rsp: u64, pops: u64 },
        /// Yes unless the jump to `target` that ends it lands in another part of the same
        /// function, which it looks up ([`unwind_step`]); the pops start at `pops_at`.
        TailCall {
            rsp: u64,
            pops: u64,
            pops_at: u64,
            target: u64,
        },
    }

    /// What `RtlVirtualUnwind` makes of the code at `pc` in `f`, read from the same bytes: an
    /// epilogue is an optional `add rsp, imm` or `lea rsp, [frame register + disp]`, any number
    /// of pops, then a return, or a jump out of the function (a tail call), either with one BND
    /// prefix (F2) or none. `None` when a byte it would read lies past the image, or a pop is
    /// into the stack pointer: the system would read the rest of the epilogue from wherever the
    /// popped word points. Compilers emit no `pop rsp`, but a pc on the second byte of `pop r12`
    /// (41 5c) reads as one, and the walk can meet such a pc through a stray return address.
    ///
    /// # Safety
    /// As [`unwind_step`].
    unsafe fn epilogue_at(
        code: &Found,
        f: *const RuntimeFunction,
        pc: u64,
        ctx: &Context,
    ) -> Option<Epilogue> {
        let byte = |at: u64| (at >= code.base && at < code.end).then(|| *(at as *const u8));
        let disp32 = |at: u64| -> Option<u64> {
            let b = [byte(at)?, byte(at + 1)?, byte(at + 2)?, byte(at + 3)?];
            Some(i64::from(i32::from_le_bytes(b)) as u64)
        };
        let disp8 = |at: u64| byte(at).map(|b| i64::from(b as i8) as u64);
        let info = unwind_info(code.base, f);
        let frame_register = *info.add(3) & 0xf;
        let mut at = pc;
        let mut rsp = ctx.rsp();
        let first = byte(at)?;
        if first == 0x48 && byte(at + 1)? == 0x83 && byte(at + 2)? == 0xc4 {
            // add rsp, imm8
            rsp = rsp.wrapping_add(disp8(at + 3)?);
            at += 4;
        } else if first == 0x48 && byte(at + 1)? == 0x81 && byte(at + 2)? == 0xc4 {
            // add rsp, imm32
            rsp = rsp.wrapping_add(disp32(at + 3)?);
            at += 7;
        } else if first & 0xfe == 0x48 && byte(at + 1)? == 0x8d {
            // lea rsp, [frame register + disp8 or disp32]
            let modrm = byte(at + 2)?;
            let register = ((first & 1) << 3) | (modrm & 7);
            if register != 0 && register == frame_register {
                if modrm & 0xf8 == 0x60 {
                    rsp = ctx.register(register).wrapping_add(disp8(at + 3)?);
                    at += 4;
                } else if modrm & 0xf8 == 0xa0 {
                    rsp = ctx.register(register).wrapping_add(disp32(at + 3)?);
                    at += 7;
                }
            }
        }
        let mut pops = 0;
        let pops_at = at;
        loop {
            let b = byte(at)?;
            let register = if b & 0xf8 == 0x58 {
                at += 1;
                b & 7
            } else if b & 0xf0 == 0x40 && byte(at + 1)? & 0xf8 == 0x58 {
                at += 2;
                ((b & 1) << 3) | (byte(at - 1)? & 7)
            } else {
                break;
            };
            if register == 4 {
                return None;
            }
            pops += 1;
        }
        // A BND prefix (F2) before the return or jump (`bnd ret`, `bnd jmp`): the system skips
        // one, as the processor does, then reads the rest as without it.
        if byte(at)? == 0xf2 {
            at += 1;
        }
        let b = byte(at)?;
        let begin = code.base + u64::from((*f).begin);
        let end = code.base + u64::from((*f).end);
        let found = if b == 0xc3 || b == 0xc2 || (b == 0xf3 && byte(at + 1)? == 0xc3) {
            // A return.
            Epilogue::Yes { rsp, pops }
        } else if b == 0xeb || b == 0xe9 {
            let target = if b == 0xeb {
                (at + 2).wrapping_add(disp8(at + 1)?)
            } else {
                (at + 5).wrapping_add(disp32(at + 1)?)
            };
            if !(begin..end).contains(&target) {
                Epilogue::TailCall {
                    rsp,
                    pops,
                    pops_at,
                    target,
                }
            } else if target == begin && (*info >> 3) & CHAIN_INFO == 0 {
                Epilogue::Yes { rsp, pops }
            } else {
                Epilogue::No
            }
        } else if (b == 0xff && byte(at + 1)? == 0x25)
            || (b & 0xf8 == 0x48 && byte(at + 1)? == 0xff && byte(at + 2)? & 0x38 == 0x20)
        {
            // An indirect jump (through the import table, or with REX.W): a tail call.
            Epilogue::Yes { rsp, pops }
        } else {
            Epilogue::No
        };
        Some(found)
    }

    /// The slot of the first unwind code of `info` (its `count` slots) that sets the frame
    /// register, looked for as ntdll 10.0.26200 does (disassembled: the unwinder's core at
    /// ntdll+0x1FC7C, the slot counts at ntdll+0xF0118): code by code, each code's slots from
    /// one table for every version (1 2 1 1 2 3 2 3 2 3 1 by operation, one more for a large
    /// allocation with a 32-bit size). `count` when no code sets it (the system then compares
    /// against the byte after the codes). `None` when the system would raise an exception (an
    /// operation of 11 or more) or compare against a slot past that byte (a code whose slots run
    /// past the end): only malformed unwind data has either. Read slot by slot, as the walk
    /// first did, a save's offset whose high byte ends in 3 (a save at 0x3610: 61 03) reads as
    /// that code, and in a frame whose frame register does not match its stack pointer the
    /// system's save reads were checked at the wrong base: LLVM functions that save XMM
    /// registers after setting rbp have such offsets.
    ///
    /// # Safety
    /// `info` is an image's unwind information, its `count` slots readable.
    unsafe fn frame_code_slot(info: *const u8, count: usize) -> Option<usize> {
        const SLOTS: [usize; 11] = [1, 2, 1, 1, 2, 3, 2, 3, 2, 3, 1];
        let mut i = 0;
        while i < count {
            let b = *info.add(5 + 2 * i);
            let op = usize::from(b & 0xf);
            if op == 3 {
                return Some(i);
            }
            i += SLOTS.get(op)? + usize::from(op == 1 && b >> 4 != 0);
        }
        (i == count).then_some(i)
    }

    /// Whether the reads `RtlVirtualUnwind` makes outside an epilogue lie where `fits` allows:
    /// it undoes the unwind codes of `f` that the prologue has executed at `pc` (every code of a
    /// chained part), reading each pushed register at the stack pointer and each saved one at
    /// the frame base (the frame register less its offset once the prologue has set it, else
    /// the stack pointer), then the return address, unless a machine frame gave the next
    /// instruction and stack pointers.
    ///
    /// # Safety
    /// As [`unwind_step`].
    unsafe fn prologue_reads_fit(
        base: u64,
        f: *const RuntimeFunction,
        pc: u64,
        ctx: &Context,
        fits: &dyn Fn(u64, u64) -> bool,
    ) -> bool {
        // The registers as the unwind restores them (a push or a save read back from the stack,
        // once the read is known to be in it).
        let mut registers = [0u64; 16];
        for (n, r) in registers.iter_mut().enumerate() {
            *r = ctx.register(n as u8);
        }
        let mut rsp = ctx.rsp();
        let offset_in = |entry: *const RuntimeFunction| {
            pc.wrapping_sub(base + u64::from((*entry).begin)) as u32
        };
        let info = unwind_info(base, f);
        let frame = *info.add(3);
        let count = usize::from(*info.add(2));
        let set_frame =
            registers[usize::from(frame & 0xf)].wrapping_sub(16 * u64::from(frame >> 4));
        let frame_base = if frame & 0xf == 0 {
            rsp
        } else if offset_in(f) >= u32::from(*info.add(1)) || (*info >> 3) & CHAIN_INFO != 0 {
            // The frame register is set once the prologue is over (or in a chained part).
            set_frame
        } else {
            // Within the prologue, from the first code that sets it on.
            let Some(set) = frame_code_slot(info, count) else {
                return false;
            };
            if offset_in(f) >= u32::from(*info.add(4 + 2 * set)) {
                set_frame
            } else {
                rsp
            }
        };
        let mut machine_frame = false;
        let mut entry = f;
        for _ in 0..CHAIN_LIMIT {
            let info = unwind_info(base, entry);
            let version = *info & 7;
            let count = usize::from(*info.add(2));
            let frame = *info.add(3);
            let slot = |i: usize| u16::from_le_bytes([*info.add(4 + 2 * i), *info.add(5 + 2 * i)]);
            let slot32 = |i: usize| u32::from(slot(i)) | (u32::from(slot(i + 1)) << 16);
            let offset = offset_in(entry);
            let mut i = 0;
            while i < count {
                let (at, op) = (*info.add(4 + 2 * i), *info.add(5 + 2 * i));
                let (op, op_info) = (op & 0xf, op >> 4);
                let slots = match op {
                    0 | 2 | 3 | 10 => 1,
                    1 if op_info == 0 => 2,
                    1 => 3,
                    4 | 8 => 2,
                    5 | 9 => 3,
                    // Version 2: an epilogue's description (one slot) and a spare code.
                    6 if version >= 2 => 1,
                    7 if version >= 2 => 2,
                    // Version 1: an XMM register's low half saved, near and far.
                    6 => 2,
                    7 => 3,
                    _ => return false,
                };
                if i + slots > count {
                    return false;
                }
                // A push or a save of the stack pointer itself (only malformed unwind data has
                // one): restored, it would send every later read wherever the word points.
                if matches!(op, 0 | 4 | 5) && op_info == 4 {
                    return false;
                }
                // A code the prologue has not reached at `pc` is not undone.
                if offset >= u32::from(at) {
                    let saved = |at: u64, len: u64, registers: &mut [u64; 16]| {
                        if !fits(at, len) {
                            return false;
                        }
                        if len == 8 {
                            registers[usize::from(op_info)] = std::ptr::read(at as *const u64);
                        }
                        true
                    };
                    let ok = match op {
                        // A push: popped from the stack pointer.
                        0 => {
                            let ok = saved(rsp, 8, &mut registers);
                            rsp = rsp.wrapping_add(8);
                            ok
                        }
                        1 => {
                            let size = if op_info == 0 {
                                8 * u64::from(slot(i + 1))
                            } else {
                                u64::from(slot32(i + 1))
                            };
                            rsp = rsp.wrapping_add(size);
                            true
                        }
                        2 => {
                            rsp = rsp.wrapping_add(8 * u64::from(op_info) + 8);
                            true
                        }
                        3 => {
                            rsp = registers[usize::from(frame & 0xf)]
                                .wrapping_sub(16 * u64::from(frame >> 4));
                            true
                        }
                        4 => saved(
                            frame_base.wrapping_add(8 * u64::from(slot(i + 1))),
                            8,
                            &mut registers,
                        ),
                        5 => saved(
                            frame_base.wrapping_add(u64::from(slot32(i + 1))),
                            8,
                            &mut registers,
                        ),
                        6 if version >= 2 => true,
                        7 if version >= 2 => false,
                        6 => fits(frame_base.wrapping_add(8 * u64::from(slot(i + 1))), 16),
                        7 => fits(frame_base.wrapping_add(u64::from(slot32(i + 1))), 16),
                        8 => fits(frame_base.wrapping_add(16 * u64::from(slot(i + 1))), 16),
                        9 => fits(frame_base.wrapping_add(u64::from(slot32(i + 1))), 16),
                        // A machine frame: the instruction pointer at the stack pointer, the
                        // stack pointer three words above it (after any error code).
                        _ => {
                            rsp = rsp.wrapping_add(8 * u64::from(op_info));
                            if !fits(rsp, 0x20) {
                                return false;
                            }
                            rsp = std::ptr::read((rsp + 0x18) as *const u64);
                            machine_frame = true;
                            true
                        }
                    };
                    if !ok {
                        return false;
                    }
                }
                i += slots;
            }
            if (*info >> 3) & CHAIN_INFO == 0 {
                return machine_frame || fits(rsp, 8);
            }
            entry = info.add(4 + 2 * ((count + 1) & !1)) as *const RuntimeFunction;
        }
        false
    }

    /// Walks the suspended thread whose registers are `ctx` (and whose stack is `low..high`) up
    /// to `keys.len()` frames: each frame's function into `keys` (its start where its image's
    /// unwind table has it, else the address itself) and its address into `pcs` (the
    /// instruction pointer, then return addresses), the lookups kept in `found`. Returns the
    /// frames and why the walk ended. Takes no lock and allocates nothing, and reads nothing
    /// outside the stack and the images: a frame whose unwind would ([`unwind_step`]), one that
    /// `RtlVirtualUnwind` could unwind only by looking up a function entry (a jump within the
    /// function at an epilogue's end), and a return into code without unwind data end the walk
    /// as a bad frame; a tail call at an epilogue's end the walk unwinds itself.
    ///
    /// # Safety
    /// The images are loaded; `low..high` is the thread's committed stack; the thread stays
    /// suspended throughout.
    unsafe fn walk_stack(
        images: &[Image],
        found: &mut Lookups,
        ctx: &mut Context,
        (low, high): (u64, u64),
        keys: &mut [u64],
        pcs: &mut [u64],
    ) -> (usize, WalkEnd) {
        let limit = keys.len().min(pcs.len());
        let mut n = 0;
        loop {
            if n >= limit {
                return (n, WalkEnd::Limit);
            }
            let pc = ctx.rip();
            let rsp = ctx.rsp();
            // A return address points after its call: the call is at the byte before.
            let at = if n == 0 { pc } else { pc - 1 };
            pcs[n] = pc;
            keys[n] = at;
            n += 1;
            if !(low..high).contains(&rsp) || rsp & 7 != 0 {
                return (n, WalkEnd::Bad);
            }
            let Some(code) = look_up(images, found, at) else {
                return (n, WalkEnd::Unknown);
            };
            keys[n - 1] = code.key;
            match (!code.entry.is_null()).then_some(code.entry) {
                Some(f) => match unwind_step(images, found, &code, f, pc, ctx, (low, high)) {
                    Step::Bad => return (n, WalkEnd::Bad),
                    Step::TailCall { rsp, pops, pops_at } => {
                        #[cfg(test)]
                        {
                            found.tail_calls += 1;
                        }
                        emulate_epilogue(ctx, rsp, pops, pops_at);
                    }
                    Step::System => {
                        #[cfg(test)]
                        {
                            found.unwinds += 1;
                        }
                        let mut handler_data = std::ptr::null_mut();
                        let mut establisher = 0u64;
                        // UNW_FLAG_NHANDLER: the frame is only unwound.
                        RtlVirtualUnwind(
                            0,
                            code.base,
                            pc,
                            f,
                            ctx,
                            &mut handler_data,
                            &mut establisher,
                            std::ptr::null_mut(),
                        );
                    }
                },
                None if n == 1 => {
                    // A leaf function (only the innermost frame can be one): its return address
                    // is at the stack pointer.
                    if rsp + 8 > high {
                        return (n, WalkEnd::Bad);
                    }
                    ctx.set_word(0xf8, std::ptr::read(rsp as *const u64));
                    ctx.set_word(0x98, rsp + 8);
                }
                // A return address into code without unwind data: that code called, so it is no
                // leaf, and where it keeps its own return address is not known (taking the word
                // at the stack pointer, as for a leaf, would follow a word it pushed).
                None => return (n, WalkEnd::Bad),
            }
            if ctx.rip() == 0 {
                return (n, WalkEnd::Start);
            }
            // Each frame moves up the stack.
            let up = ctx.rsp();
            if up <= rsp || up >= high {
                return (n, WalkEnd::Bad);
            }
        }
    }

    /// `THREADENTRY32`.
    #[repr(C)]
    #[derive(Default)]
    struct ThreadEntry {
        size: u32,
        usage: u32,
        thread: u32,
        process: u32,
        base_priority: i32,
        delta_priority: i32,
        flags: u32,
    }

    const SNAP_THREAD: u32 = 0x4;
    type GetThreadDescription = unsafe extern "system" fn(Handle, *mut *mut u16) -> i32;

    const THREAD_SUSPEND_RESUME: u32 = 0x0002;
    const THREAD_GET_CONTEXT: u32 = 0x0008;
    const THREAD_QUERY_INFORMATION: u32 = 0x0040;
    const CONTEXT_CONTROL: u32 = 0x0010_0001;
    const CONTEXT_INTEGER: u32 = 0x0010_0002;
    const CREATE_WAITABLE_TIMER_HIGH_RESOLUTION: u32 = 0x2;
    const TIMER_ALL_ACCESS: u32 = 0x1f_0003;
    const FROM_ADDRESS: u32 = 0x4;
    const UNCHANGED_REFCOUNT: u32 = 0x2;

    /// The x64 `CONTEXT` (1232 bytes, 16-byte aligned): `ContextFlags` at 0x30, `Rip` at 0xf8.
    #[repr(C, align(16))]
    struct Context([u8; 0x4d0]);

    pub fn current_thread_id() -> u32 {
        thread_local! {
            static ID: u32 = unsafe { GetCurrentThreadId() };
        }
        ID.with(|id| *id)
    }

    type SymInitialize = unsafe extern "system" fn(Handle, *const u8, i32) -> i32;
    type SymSetOptions = unsafe extern "system" fn(u32) -> u32;
    type SymFromAddr = unsafe extern "system" fn(Handle, u64, *mut u64, *mut u8) -> i32;
    type SymGetModuleInfo = unsafe extern "system" fn(Handle, u64, *mut u8) -> i32;

    pub struct Sampler {
        threads: HashMap<u32, Handle>,
        timer: Handle,
        context: Box<Context>,
        sym_from_addr: Option<SymFromAddr>,
        sym_module_info: Option<SymGetModuleInfo>,
        sym_tried: bool,
        names: HashMap<u64, (String, String)>,
        modules: HashMap<usize, String>,
        thread_names: HashMap<u32, String>,
        /// The loaded images by base, with their unwind tables, each held loaded until the next
        /// listing ([`Sampler::refresh_images`]), when they were last listed, and whether a walk
        /// met code in none of them since.
        images: Vec<Image>,
        images_at: Option<Instant>,
        images_missed: bool,
        /// The walks' lookups of code addresses (emptied when the images are listed again).
        found: Box<Lookups>,
        /// Each walked thread's TEB (its stack's bounds are at 8 and 0x10).
        tebs: HashMap<u32, u64>,
        query_thread: Option<QueryThread>,
        /// The last walk's frames: each one's function and address ([`walk_stack`]).
        keys: Box<[u64; MAX_FRAMES]>,
        pcs: Box<[u64; MAX_FRAMES]>,
    }

    impl Sampler {
        pub fn new() -> Self {
            // SAFETY: plain Win32 calls with valid arguments; a null timer falls back to sleep.
            let timer = unsafe {
                CreateWaitableTimerExW(
                    std::ptr::null_mut(),
                    std::ptr::null(),
                    CREATE_WAITABLE_TIMER_HIGH_RESOLUTION,
                    TIMER_ALL_ACCESS,
                )
            };
            // SAFETY: ntdll is always loaded; NtQueryInformationThread is looked up by name and
            // has the signature of `QueryThread`.
            let query_thread = unsafe {
                let f = GetProcAddress(
                    GetModuleHandleA(b"ntdll.dll\0".as_ptr()),
                    b"NtQueryInformationThread\0".as_ptr(),
                );
                (!f.is_null()).then(|| std::mem::transmute::<*mut c_void, QueryThread>(f))
            };
            Sampler {
                threads: HashMap::new(),
                timer,
                context: Box::new(Context([0; 0x4d0])),
                sym_from_addr: None,
                sym_module_info: None,
                sym_tried: false,
                names: HashMap::new(),
                modules: HashMap::new(),
                thread_names: HashMap::new(),
                images: Vec::new(),
                images_at: None,
                images_missed: false,
                found: Lookups::new(),
                tebs: HashMap::new(),
                query_thread,
                keys: Box::new([0; MAX_FRAMES]),
                pcs: Box::new([0; MAX_FRAMES]),
            }
        }

        /// A handle to OS thread `id` with suspend, context and query access, kept open: while it
        /// is open the system gives the id to no other thread, so the handle stays the thread's
        /// own. A walked thread's stays open for good (its TEB is kept by id); another's is
        /// closed once the thread has ended ([`Sampler::forget`]). A thread of another process
        /// (an id of this process's thread that ended before it was first opened, since given
        /// to a thread elsewhere) is not opened: its TEB would be read as this process's memory.
        /// A refusal is not kept, so the id is tried again when it comes back (given to a thread
        /// of this process). A name kept under the id is dropped when it is opened: the id was
        /// free (no handle of ours held it), so the name may be an ended thread's.
        fn thread(&mut self, id: u32) -> Option<Handle> {
            if let Some(&handle) = self.threads.get(&id) {
                return Some(handle);
            }
            // SAFETY: OpenThread with a thread id; a null handle is checked, and a handle to
            // another process's thread is closed and not kept.
            let handle = unsafe {
                let handle = OpenThread(
                    THREAD_SUSPEND_RESUME | THREAD_GET_CONTEXT | THREAD_QUERY_INFORMATION,
                    0,
                    id,
                );
                if handle.is_null() {
                    return None;
                }
                if GetProcessIdOfThread(handle) != GetCurrentProcessId() {
                    CloseHandle(handle);
                    return None;
                }
                handle
            };
            self.thread_names.remove(&id);
            self.threads.insert(id, handle);
            Some(handle)
        }

        /// Closes the handle to OS thread `id` (one that has ended, or that a listing of the
        /// process no longer shows), unless the thread was walked: its TEB is kept by id, so its
        /// handle stays open and the id stays its own. Its name is read first, while the handle
        /// still holds the id (measured: an ended thread's name reads through it, and once the
        /// handle is closed the id opens no more), and kept for the table that holds its
        /// samples; [`Sampler::thread`] drops it if the id is opened again.
        fn forget(&mut self, id: u32) {
            if self.tebs.contains_key(&id) || !self.threads.contains_key(&id) {
                return;
            }
            self.thread_name(id);
            if let Some(handle) = self.threads.remove(&id) {
                // SAFETY: the handle was opened by `thread` and is closed once.
                unsafe { CloseHandle(handle) };
            }
        }

        /// OS thread `id`'s TEB (`NtQueryInformationThread`'s basic information, the TEB's
        /// address at 8), 0 unknown.
        fn teb(&mut self, id: u32, handle: Handle) -> u64 {
            if let Some(&teb) = self.tebs.get(&id) {
                return teb;
            }
            let mut info = [0u64; 6];
            let teb = match self.query_thread {
                // SAFETY: ThreadBasicInformation (0) into a buffer of its 48 bytes.
                Some(query) => unsafe {
                    if query(
                        handle,
                        0,
                        info.as_mut_ptr() as *mut c_void,
                        48,
                        std::ptr::null_mut(),
                    ) >= 0
                    {
                        info[1]
                    } else {
                        0
                    }
                },
                None => 0,
            };
            self.tebs.insert(id, teb);
            teb
        }

        /// Lists the loaded images again (`K32EnumProcessModules`) with their unwind tables:
        /// at the first walk, every five seconds (an image unloaded is dropped), and a quarter
        /// of a second after a walk met code in no known image (one loaded since). Each listed
        /// image is held loaded until the next listing ([`listed_image`]): with the profiler
        /// walking, a DLL the program frees unloads up to five seconds later.
        fn refresh_images(&mut self) {
            let due = match self.images_at {
                None => true,
                Some(at) => {
                    let since = at.elapsed();
                    since >= Duration::from_secs(5)
                        || (self.images_missed && since >= Duration::from_millis(250))
                }
            };
            if !due {
                return;
            }
            self.images_at = Some(Instant::now());
            self.images_missed = false;
            // The lookups point into the listed images' unwind tables.
            self.found.slots.fill(Found::EMPTY);
            // The last listing's references are let go before the new ones are taken: an image
            // its users have freed since is unloaded here (taken again first, it would stay
            // loaded for good). No thread is suspended meanwhile, so waiting on the loader's
            // lock, as letting go and taking references do, ends.
            self.images.clear();
            let mut handles: Vec<Handle> = vec![std::ptr::null_mut(); 512];
            // SAFETY: the buffer's size in bytes is passed. A module listed may be unloaded
            // before its reference is taken: `listed_image` skips one that is gone.
            unsafe {
                loop {
                    let bytes = (handles.len() * std::mem::size_of::<Handle>()) as u32;
                    let mut needed = 0u32;
                    if K32EnumProcessModules(
                        GetCurrentProcess(),
                        handles.as_mut_ptr(),
                        bytes,
                        &mut needed,
                    ) == 0
                    {
                        return;
                    }
                    let count = needed as usize / std::mem::size_of::<Handle>();
                    if count <= handles.len() {
                        handles.truncate(count);
                        break;
                    }
                    handles.resize(count, std::ptr::null_mut());
                }
                self.images = handles
                    .iter()
                    .filter_map(|&h| listed_image(h as u64))
                    .collect();
            }
            self.images.sort_by_key(|im| im.base);
        }

        /// Walks OS thread `id`'s native stack, the thread suspended meanwhile: up to `depth`
        /// frames into [`Sampler::keys`] ([`walk_stack`]). `None` when the thread cannot be
        /// suspended or read.
        pub fn walk(&mut self, id: u32, depth: usize) -> Option<Walk> {
            if id == 0 || id == current_thread_id() {
                return None;
            }
            let handle = self.thread(id)?;
            let teb = self.teb(id, handle);
            if teb == 0 {
                return None;
            }
            self.refresh_images();
            let depth = depth.clamp(1, MAX_FRAMES);
            let ctx = &mut *self.context;
            ctx.0[0x30..0x34].copy_from_slice(&(CONTEXT_CONTROL | CONTEXT_INTEGER).to_le_bytes());
            let started = Instant::now();
            // SAFETY: the handle has suspend/resume and get-context access; between the suspend
            // and the resume only `walk_stack` runs (no lock, no allocation: its buffers are
            // ours), on the thread's committed stack as its TEB gives it (base at 8, limit at
            // 0x10; the TEB is this process's memory).
            let walked = unsafe {
                // A thread that has ended cannot be suspended (measured: SuspendThread answers
                // -1 with access denied), so this is the end of its walks. Its handle stays open,
                // which keeps its id from being given to another thread, here or in another
                // process ([`Sampler::thread`]).
                if SuspendThread(handle) == u32::MAX {
                    return None;
                }
                let walked = if GetThreadContext(handle, ctx as *mut Context) != 0 {
                    let high = std::ptr::read_volatile((teb + 8) as *const u64);
                    let low = std::ptr::read_volatile((teb + 0x10) as *const u64);
                    let rip = ctx.rip();
                    let (frames, end) = walk_stack(
                        &self.images,
                        &mut self.found,
                        ctx,
                        (low, high),
                        &mut self.keys[..depth],
                        &mut self.pcs[..depth],
                    );
                    Some((rip, frames, end))
                } else {
                    None
                };
                ResumeThread(handle);
                walked
            };
            let suspended_ns = started.elapsed().as_nanos() as u64;
            let (rip, frames, end) = walked?;
            if end == WalkEnd::Unknown {
                self.images_missed = true;
            }
            Some(Walk {
                rip,
                frames,
                end,
                suspended_ns,
            })
        }

        /// The first `n` frames of the last walk: each one's function (its start address, or
        /// the address itself where its image has no unwind entry for it), innermost first.
        pub fn keys(&self, n: usize) -> &[u64] {
            &self.keys[..n.min(MAX_FRAMES)]
        }

        /// The first `n` frames of the last walk: the instruction pointer, then the return
        /// addresses.
        #[cfg(test)]
        pub fn pcs(&self, n: usize) -> &[u64] {
            &self.pcs[..n.min(MAX_FRAMES)]
        }

        /// How many of the walks' lookups were answered from the kept ones, and how many frames
        /// were handed to `RtlVirtualUnwind`, so far.
        #[cfg(test)]
        pub fn lookup_counts(&self) -> (u64, u64) {
            (self.found.hits, self.found.unwinds)
        }

        /// Whether the code at `pc` has an unwind entry in its image (`None`: in no image the
        /// sampler knows).
        #[cfg(test)]
        pub fn has_unwind_entry(&mut self, pc: u64) -> Option<bool> {
            self.refresh_images();
            // SAFETY: the images were just listed (loaded modules).
            unsafe { image_of(&self.images, pc).map(|im| function_entry(im, pc).is_some()) }
        }

        /// Waits about a millisecond.
        pub fn wait_tick(&mut self) {
            if self.timer.is_null() {
                std::thread::sleep(Duration::from_millis(1));
                return;
            }
            // Relative, in 100 ns units.
            let due: i64 = -10_000;
            // SAFETY: the timer handle is ours and valid; `due` outlives the call.
            unsafe {
                if SetWaitableTimer(
                    self.timer,
                    &due,
                    0,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    0,
                ) == 0
                {
                    std::thread::sleep(Duration::from_millis(1));
                    return;
                }
                WaitForSingleObject(self.timer, 0xffff_ffff);
            }
        }

        /// The instruction pointer of OS thread `id` (0: none).
        pub fn instruction_pointer(&mut self, id: u32) -> Option<u64> {
            if id == 0 || id == current_thread_id() {
                return None;
            }
            let handle = self.thread(id)?;
            let ctx = &mut *self.context;
            ctx.0[0x30..0x34].copy_from_slice(&CONTEXT_CONTROL.to_le_bytes());
            // SAFETY: the handle has suspend/resume and get-context access; nothing between
            // the suspend and the resume allocates or takes a lock (the context buffer is ours).
            let ok = unsafe {
                // A thread that has ended cannot be suspended: its handle is closed (nothing of
                // the thread is read here, so its id need not stay its own).
                if SuspendThread(handle) == u32::MAX {
                    self.forget(id);
                    return None;
                }
                let ok = GetThreadContext(handle, ctx as *mut Context) != 0;
                ResumeThread(handle);
                ok
            };
            ok.then(|| u64::from_le_bytes(ctx.0[0xf8..0x100].try_into().expect("8 bytes")))
        }

        /// The ids of every thread of this process but the sampler's own. The handles kept for
        /// threads not listed (ended since) are closed ([`Sampler::forget`]).
        pub fn process_threads(&mut self) -> Vec<u32> {
            let mut ids = Vec::new();
            // SAFETY: a Toolhelp snapshot walked with a correctly sized entry, then closed.
            unsafe {
                let snapshot = CreateToolhelp32Snapshot(SNAP_THREAD, 0);
                if snapshot.is_null() || snapshot as isize == -1 {
                    return ids;
                }
                let process = GetCurrentProcessId();
                let mut entry = ThreadEntry {
                    size: std::mem::size_of::<ThreadEntry>() as u32,
                    ..Default::default()
                };
                let mut more = Thread32First(snapshot, &mut entry) != 0;
                while more {
                    if entry.process == process && entry.thread != current_thread_id() {
                        ids.push(entry.thread);
                    }
                    more = Thread32Next(snapshot, &mut entry) != 0;
                }
                CloseHandle(snapshot);
            }
            let gone: Vec<u32> = self
                .threads
                .keys()
                .copied()
                .filter(|id| !ids.contains(id))
                .collect();
            for id in gone {
                self.forget(id);
            }
            ids
        }

        /// A thread's name (`GetThreadDescription`), empty without one; kept by id until the id
        /// is opened afresh ([`Sampler::thread`]).
        pub fn thread_name(&mut self, id: u32) -> String {
            if let Some(name) = self.thread_names.get(&id) {
                return name.clone();
            }
            let mut name = String::new();
            // SAFETY: GetThreadDescription looked up in kernel32 (Windows 10 1607 and later)
            // and called on a handle with query access; its string is freed with LocalFree.
            unsafe {
                let handle = OpenThread(THREAD_QUERY_INFORMATION, 0, id);
                let kernel = GetModuleHandleA(b"kernel32.dll\0".as_ptr());
                let f = GetProcAddress(kernel, b"GetThreadDescription\0".as_ptr());
                if !handle.is_null() && !f.is_null() {
                    let f: GetThreadDescription = std::mem::transmute(f);
                    let mut text: *mut u16 = std::ptr::null_mut();
                    if f(handle, &mut text) >= 0 && !text.is_null() {
                        let mut n = 0;
                        while *text.add(n) != 0 && n < 256 {
                            n += 1;
                        }
                        name = String::from_utf16_lossy(std::slice::from_raw_parts(text, n));
                        LocalFree(text as *mut c_void);
                    }
                }
                if !handle.is_null() {
                    CloseHandle(handle);
                }
            }
            self.thread_names.insert(id, name.clone());
            name
        }

        fn load_symbols(&mut self) {
            if self.sym_tried {
                return;
            }
            self.sym_tried = true;
            // SAFETY: dbghelp.dll is a system DLL; the procedures are looked up by name and
            // called with their documented signatures. Only this thread uses dbghelp.
            unsafe {
                let dll = LoadLibraryA(b"dbghelp.dll\0".as_ptr());
                if dll.is_null() {
                    return;
                }
                let set_options = GetProcAddress(dll, b"SymSetOptions\0".as_ptr());
                let init = GetProcAddress(dll, b"SymInitialize\0".as_ptr());
                let from_addr = GetProcAddress(dll, b"SymFromAddr\0".as_ptr());
                if init.is_null() || from_addr.is_null() {
                    return;
                }
                if !set_options.is_null() {
                    let set_options: SymSetOptions = std::mem::transmute(set_options);
                    // SYMOPT_UNDNAME | SYMOPT_DEFERRED_LOADS
                    set_options(0x2 | 0x4);
                }
                // dbghelp looks for a module's PDB by the name the module was linked with
                // (`nv_viewer.pdb`) at the path it was linked with, then on its search path, not
                // next to the module: the folder `<program>.sym` beside the program (a copied
                // program's own PDB, so copies of several builds can share a folder), the
                // program's folder, then `NV_PROFILE_SYMBOLS` (folders separated by `;`). A PDB
                // of another build is refused (its signature differs).
                let exe = std::env::current_exe().ok();
                let mut path = exe
                    .as_ref()
                    .and_then(|p| {
                        let dir = p.parent()?.to_string_lossy().into_owned();
                        let stem = p.file_stem()?.to_string_lossy().into_owned();
                        Some(format!("{dir}\\{stem}.sym;{dir}"))
                    })
                    .unwrap_or_default();
                if let Ok(more) = std::env::var("NV_PROFILE_SYMBOLS") {
                    path = format!("{path};{more}");
                }
                let path = std::ffi::CString::new(path).unwrap_or_default();
                let init: SymInitialize = std::mem::transmute(init);
                if init(GetCurrentProcess(), path.as_ptr() as *const u8, 1) == 0 {
                    return;
                }
                self.sym_from_addr =
                    Some(std::mem::transmute::<*mut c_void, SymFromAddr>(from_addr));
                let module_info = GetProcAddress(dll, b"SymGetModuleInfo64\0".as_ptr());
                self.sym_module_info = (!module_info.is_null())
                    .then(|| std::mem::transmute::<*mut c_void, SymGetModuleInfo>(module_info));
            }
        }

        /// Where the program's symbols came from: the PDB dbghelp loaded for it, and whether it
        /// has line numbers and private symbols (a build with `debug = 0` has public symbols
        /// only, and a time inside a function without one counts under the nearest one before).
        pub fn symbol_source(&mut self) -> String {
            // SAFETY: GetModuleHandleA(null) is the program's base.
            let base = unsafe { GetModuleHandleA(std::ptr::null()) } as u64;
            // A lookup in the program loads its symbols (they load on first use).
            let _ = self.name(base + 0x1000);
            let Some(info) = self.sym_module_info else {
                return "no dbghelp".into();
            };
            // IMAGEHLP_MODULE64: 1680 bytes; SymType at 32, LoadedPdbName at 580 (256 bytes),
            // LineNumbers at 1652, GlobalSymbols at 1656.
            let mut buf = vec![0u64; 1680 / 8];
            let bytes = buf.as_mut_ptr() as *mut u8;
            // SAFETY: `buf` is 8-aligned and 1680 bytes, its size in its first word.
            unsafe {
                *(bytes as *mut u32) = 1680;
                if info(GetCurrentProcess(), base, bytes) == 0 {
                    return "no symbols for the program".into();
                }
                let all = std::slice::from_raw_parts(bytes, 1680);
                let word = |at: usize| u32::from_le_bytes(all[at..at + 4].try_into().expect("4"));
                let pdb = &all[580..836];
                let pdb = String::from_utf8_lossy(
                    &pdb[..pdb.iter().position(|&c| c == 0).unwrap_or(256)],
                )
                .into_owned();
                format!(
                    "symbols from {} (type {}, line numbers {}, private symbols {})",
                    if pdb.is_empty() { "(none)" } else { &pdb },
                    word(32),
                    if word(1652) != 0 { "yes" } else { "no" },
                    if word(1656) != 0 { "yes" } else { "no" },
                )
            }
        }

        fn module(&mut self, addr: u64) -> String {
            let mut module: Handle = std::ptr::null_mut();
            // SAFETY: GetModuleHandleExA with FROM_ADDRESS reads only the address's module.
            let found = unsafe {
                GetModuleHandleExA(
                    FROM_ADDRESS | UNCHANGED_REFCOUNT,
                    addr as *const u8,
                    &mut module,
                ) != 0
            };
            if !found {
                return "?".into();
            }
            self.modules
                .entry(module as usize)
                .or_insert_with(|| {
                    let mut name = [0u8; 520];
                    // SAFETY: the buffer is ours and its size is passed.
                    let n =
                        unsafe { GetModuleFileNameA(module, name.as_mut_ptr(), name.len() as u32) }
                            as usize;
                    let path = String::from_utf8_lossy(&name[..n.min(name.len())]).to_string();
                    path.rsplit(['\\', '/'])
                        .next()
                        .unwrap_or(&path)
                        .to_ascii_lowercase()
                })
                .clone()
        }

        /// (module, function) of a native address.
        pub fn name(&mut self, addr: u64) -> (String, String) {
            if let Some(n) = self.names.get(&addr) {
                return n.clone();
            }
            self.load_symbols();
            let module = self.module(addr);
            let mut symbol = String::from("?");
            if let Some(from_addr) = self.sym_from_addr {
                // SYMBOL_INFO: 88 bytes, then the name (MaxNameLen at 80, NameLen at 76).
                let mut buf = vec![0u64; (88 + 1024) / 8 + 1];
                let bytes = buf.as_mut_ptr() as *mut u8;
                let mut displacement = 0u64;
                // SAFETY: `buf` is 8-aligned and holds the structure plus 1024 name bytes.
                unsafe {
                    *(bytes as *mut u32) = 88;
                    *(bytes.add(80) as *mut u32) = 1024;
                    if from_addr(GetCurrentProcess(), addr, &mut displacement, bytes) != 0 {
                        let len = (*(bytes.add(76) as *const u32)).min(1024) as usize;
                        let name = std::slice::from_raw_parts(bytes.add(84), len);
                        symbol = super::readable_symbol(&String::from_utf8_lossy(name));
                    }
                }
            }
            let n = (module, symbol);
            self.names.insert(addr, n.clone());
            n
        }
    }

    /// Walks of made-up threads: a context and a stack of the test's own, so that each case of
    /// an unwind's reads is set up exactly, and threads of other processes.
    #[cfg(test)]
    mod tests {
        use super::*;

        // A function with a frame register (rbp = rsp + 32 after the prologue), with labels at
        // its body and at each instruction of its epilogue.
        std::arch::global_asm!(
            ".globl nv_profile_test_framed",
            ".globl nv_profile_test_framed_body",
            ".globl nv_profile_test_framed_add",
            ".globl nv_profile_test_framed_pop",
            ".globl nv_profile_test_framed_ret",
            ".seh_proc nv_profile_test_framed",
            "nv_profile_test_framed:",
            "push rbp",
            ".seh_pushreg rbp",
            "sub rsp, 32",
            ".seh_stackalloc 32",
            "lea rbp, [rsp + 32]",
            ".seh_setframe rbp, 32",
            ".seh_endprologue",
            "nv_profile_test_framed_body:",
            "nop",
            "nv_profile_test_framed_add:",
            "add rsp, 32",
            "nv_profile_test_framed_pop:",
            "pop rbp",
            "nv_profile_test_framed_ret:",
            "ret",
            ".seh_endproc",
        );

        // A function with 64 KB of locals and no frame register (never called: its address is
        // all the tests use).
        std::arch::global_asm!(
            ".globl nv_profile_test_big",
            ".globl nv_profile_test_big_body",
            ".seh_proc nv_profile_test_big",
            "nv_profile_test_big:",
            "push rbx",
            ".seh_pushreg rbx",
            "sub rsp, 0x10000",
            ".seh_stackalloc 0x10000",
            ".seh_endprologue",
            "nv_profile_test_big_body:",
            "nop",
            "add rsp, 0x10000",
            "pop rbx",
            "ret",
            ".seh_endproc",
        );

        // A function whose epilogue starts with `lea rsp, [rbp + 8]` (its frame register set to
        // rsp + 32 after 40 bytes of locals, below the saved rbx and rbp).
        std::arch::global_asm!(
            ".globl nv_profile_test_lea",
            ".globl nv_profile_test_lea_epilogue",
            ".seh_proc nv_profile_test_lea",
            "nv_profile_test_lea:",
            "push rbp",
            ".seh_pushreg rbp",
            "push rbx",
            ".seh_pushreg rbx",
            "sub rsp, 40",
            ".seh_stackalloc 40",
            "lea rbp, [rsp + 32]",
            ".seh_setframe rbp, 32",
            ".seh_endprologue",
            "nop",
            "nv_profile_test_lea_epilogue:",
            "lea rsp, [rbp + 8]",
            "pop rbx",
            "pop rbp",
            "ret",
            ".seh_endproc",
        );

        // Functions whose epilogue ends in a tail call: `add rsp, 32`, `pop rbx`, then a jump
        // to the start of another function (one with a BND prefix, `bnd jmp`).
        std::arch::global_asm!(
            ".globl nv_profile_test_tail",
            ".globl nv_profile_test_tail_epilogue",
            ".seh_proc nv_profile_test_tail",
            "nv_profile_test_tail:",
            "push rbx",
            ".seh_pushreg rbx",
            "sub rsp, 32",
            ".seh_stackalloc 32",
            ".seh_endprologue",
            "nop",
            "nv_profile_test_tail_epilogue:",
            "add rsp, 32",
            "pop rbx",
            "jmp nv_profile_test_tail_target",
            ".seh_endproc",
            // The same with a BND prefix (F2) on the jump.
            ".globl nv_profile_test_bnd_tail",
            ".globl nv_profile_test_bnd_tail_epilogue",
            ".seh_proc nv_profile_test_bnd_tail",
            "nv_profile_test_bnd_tail:",
            "push rbx",
            ".seh_pushreg rbx",
            "sub rsp, 32",
            ".seh_stackalloc 32",
            ".seh_endprologue",
            "nop",
            "nv_profile_test_bnd_tail_epilogue:",
            "add rsp, 32",
            "pop rbx",
            ".byte 0xf2, 0xe9",
            ".long nv_profile_test_tail_target - . - 4",
            ".seh_endproc",
            ".seh_proc nv_profile_test_tail_target",
            "nv_profile_test_tail_target:",
            ".seh_endprologue",
            "ret",
            ".seh_endproc",
        );

        // (These two chained fixtures assemble only where `.seh_startchained` is known: Rust 1.99's

        // assembler rejects it, so they are built with `--cfg nv_chained_unwind_asm`.)

        // A function in two parts, the second with chained unwind information: the second ends
        // in a jump back into the first, out of its own range but within the same function.
        // (The assembler gives the first part's entry the whole function's range, so the jump
        // that leaves its part's range is the second part's.)
        #[cfg(nv_chained_unwind_asm)]
        std::arch::global_asm!(
            ".globl nv_profile_test_split",
            ".globl nv_profile_test_split_cold",
            ".globl nv_profile_test_split_jump",
            ".seh_proc nv_profile_test_split",
            "nv_profile_test_split:",
            "push rbx",
            ".seh_pushreg rbx",
            ".seh_endprologue",
            "nv_profile_test_split_hot:",
            "pop rbx",
            "ret",
            ".seh_startchained",
            ".seh_endprologue",
            "nv_profile_test_split_cold:",
            "nop",
            "nv_profile_test_split_jump:",
            "jmp nv_profile_test_split_hot",
            ".seh_endchained",
            ".seh_endproc",
        );

        // A function in two parts whose second (chained) part ends in `pop rbx` and a jump to
        // the function's own start: a recursive tail call (never called).
        #[cfg(nv_chained_unwind_asm)]
        std::arch::global_asm!(
            ".globl nv_profile_test_recurse",
            ".globl nv_profile_test_recurse_pop",
            ".seh_proc nv_profile_test_recurse",
            "nv_profile_test_recurse:",
            "push rbx",
            ".seh_pushreg rbx",
            ".seh_endprologue",
            "pop rbx",
            "ret",
            ".seh_startchained",
            ".seh_endprologue",
            "nop",
            "nv_profile_test_recurse_pop:",
            "pop rbx",
            ".byte 0xe9",
            ".long nv_profile_test_recurse - . - 4",
            ".seh_endchained",
            ".seh_endproc",
        );

        // A function that saves r12: its epilogue's `pop r12` is 41 5c, and the 5c alone reads
        // as `pop rsp`.
        std::arch::global_asm!(
            ".globl nv_profile_test_r12",
            ".globl nv_profile_test_r12_pop",
            ".seh_proc nv_profile_test_r12",
            "nv_profile_test_r12:",
            "push r12",
            ".seh_pushreg r12",
            ".seh_endprologue",
            "nop",
            "nv_profile_test_r12_pop:",
            "pop r12",
            "ret",
            ".seh_endproc",
        );

        // A function that sets its frame register (rbp = rsp + 128) and then saves xmm6 at 0x3610
        // above the frame base, as LLVM code does: the save's second slot holds 0x361 (61 03),
        // whose high byte read alone is the code that sets the frame register. Labelled after
        // the save, still in the prologue (never called).
        std::arch::global_asm!(
            ".globl nv_profile_test_fpreg",
            ".globl nv_profile_test_fpreg_saved",
            ".seh_proc nv_profile_test_fpreg",
            "nv_profile_test_fpreg:",
            "push rbp",
            ".seh_pushreg rbp",
            "sub rsp, 0x3700",
            ".seh_stackalloc 0x3700",
            "lea rbp, [rsp + 128]",
            ".seh_setframe rbp, 128",
            "movaps xmmword ptr [rsp + 0x3610], xmm6",
            ".seh_savexmm xmm6, 0x3610",
            "nv_profile_test_fpreg_saved:",
            "nop",
            ".seh_endprologue",
            "movaps xmm6, xmmword ptr [rsp + 0x3610]",
            "add rsp, 0x3700",
            "pop rbp",
            "ret",
            ".seh_endproc",
        );

        extern "C" {
            fn nv_profile_test_framed_body();
            fn nv_profile_test_framed_add();
            fn nv_profile_test_framed_pop();
            fn nv_profile_test_framed_ret();
            fn nv_profile_test_big_body();
            fn nv_profile_test_lea_epilogue();
            fn nv_profile_test_tail_epilogue();
            fn nv_profile_test_bnd_tail_epilogue();
            #[cfg(nv_chained_unwind_asm)]
            fn nv_profile_test_split();
            #[cfg(nv_chained_unwind_asm)]
            fn nv_profile_test_split_jump();
            #[cfg(nv_chained_unwind_asm)]
            fn nv_profile_test_split_cold();
            #[cfg(nv_chained_unwind_asm)]
            fn nv_profile_test_recurse();
            #[cfg(nv_chained_unwind_asm)]
            fn nv_profile_test_recurse_pop();
            fn nv_profile_test_r12_pop();
            fn nv_profile_test_fpreg();
            fn nv_profile_test_fpreg_saved();
        }

        type Code = unsafe extern "C" fn();

        /// A walk of a made-up thread at `rip` with the stack pointer `rsp` and the frame
        /// register (rbp) `rbp`, its stack `low..high` (the test's own memory): the frames and
        /// how the walk ended, how many frames went to `RtlVirtualUnwind`, and the stack pointer
        /// the walk ended with.
        fn walk(
            s: &mut Sampler,
            rip: Code,
            rsp: u64,
            rbp: u64,
            stack: (u64, u64),
        ) -> (usize, WalkEnd, u64, u64) {
            s.refresh_images();
            let walked = walk_at(s, rip as usize as u64, rsp, rbp, stack);
            (walked.0, walked.1, walked.2, walked.4.rsp())
        }

        /// [`walk`] at any address `rip`, the images as last listed: also how many frames the
        /// walk unwound itself at a tail call, and the registers it ended with.
        fn walk_at(
            s: &mut Sampler,
            rip: u64,
            rsp: u64,
            rbp: u64,
            stack: (u64, u64),
        ) -> (usize, WalkEnd, u64, u64, Box<Context>) {
            let mut ctx = Box::new(Context([0; 0x4d0]));
            ctx.set_word(0xf8, rip);
            ctx.set_word(0x98, rsp);
            ctx.set_word(0xa0, rbp);
            let (mut keys, mut pcs) = ([0u64; 4], [0u64; 4]);
            let (unwinds, tail_calls) = (s.found.unwinds, s.found.tail_calls);
            // SAFETY: the images are listed; `stack` is memory of the test's own.
            let (n, end) = unsafe {
                walk_stack(
                    &s.images,
                    &mut s.found,
                    &mut ctx,
                    stack,
                    &mut keys,
                    &mut pcs,
                )
            };
            (
                n,
                end,
                s.found.unwinds - unwinds,
                s.found.tail_calls - tail_calls,
                ctx,
            )
        }

        /// A function with a frame register, in its body and at each instruction of its
        /// epilogue, unwinds to its caller when the stack holds its frame exactly, whatever the
        /// frame register holds once the epilogue has popped it (the caller's value, here not a
        /// stack address); with one word less, the walk ends as a bad frame and nothing goes to
        /// `RtlVirtualUnwind`.
        #[test]
        fn a_frame_register_function_unwinds_at_every_step_of_its_epilogue() {
            let mut s = Sampler::new();
            let mut stack = vec![0u64; 0x40];
            let low = stack.as_mut_ptr() as u64;
            // The body's frame: 32 bytes of locals, the caller's rbp, the return address (0:
            // the thread's start).
            let rsp = low + 0x80;
            stack[(0x80 + 32) / 8] = 0x5eed;
            let high = rsp + 48;
            for (step, rip, at, rbp) in [
                ("body", nv_profile_test_framed_body as Code, rsp, rsp + 32),
                ("add rsp", nv_profile_test_framed_add, rsp, rsp + 32),
                ("pop rbp", nv_profile_test_framed_pop, rsp + 32, rsp + 32),
                ("ret", nv_profile_test_framed_ret, rsp + 40, 0x5eed),
            ] {
                assert_eq!(
                    walk(&mut s, rip, at, rbp, (low, high)),
                    (1, WalkEnd::Start, 1, rsp + 48),
                    "{step}"
                );
                assert_eq!(
                    walk(&mut s, rip, at, rbp, (low, high - 8)),
                    (1, WalkEnd::Bad, 0, at),
                    "{step}, one word short"
                );
            }
        }

        /// A frame bigger than the stack above the stack pointer (64 KB of locals in a 4 KB
        /// stack) ends the walk as a bad frame without `RtlVirtualUnwind`, which would read
        /// the saved register and the return address 64 KB up, past the stack; with a stack to
        /// hold it, it unwinds.
        #[test]
        fn a_frame_reaching_past_the_stack_is_not_unwound() {
            let mut s = Sampler::new();
            let mut stack = vec![0u64; 0x2100];
            let low = stack.as_mut_ptr() as u64;
            let rsp = low + 0x80;
            let rip = nv_profile_test_big_body as Code;
            assert_eq!(
                walk(&mut s, rip, rsp, 0, (low, low + 0x1000)),
                (1, WalkEnd::Bad, 0, rsp)
            );
            assert_eq!(
                walk(&mut s, rip, rsp, 0, (low, rsp + 0x10010)),
                (1, WalkEnd::Start, 1, rsp + 0x10010)
            );
        }

        /// An epilogue that starts with `lea rsp, [rbp + 8]` pops from where the frame register
        /// says, wherever the stack pointer is: unwound when the pops and the return address
        /// lie in the stack, a bad frame when they reach past it or the frame register points
        /// elsewhere.
        #[test]
        fn a_lea_epilogue_reads_where_the_frame_register_points() {
            let mut s = Sampler::new();
            let mut stack = vec![0u64; 0x40];
            let low = stack.as_mut_ptr() as u64;
            // The body's frame: 40 bytes of locals (rbp = rsp + 32), the saved rbx and rbp, the
            // return address (0).
            let rsp = low + 0x80;
            stack[(0x80 + 40) / 8] = 0xb0b;
            stack[(0x80 + 48) / 8] = 0x5eed;
            let high = rsp + 64;
            let rip = nv_profile_test_lea_epilogue as Code;
            let below = rsp - 0x40;
            assert_eq!(
                walk(&mut s, rip, below, rsp + 32, (low, high)),
                (1, WalkEnd::Start, 1, rsp + 64)
            );
            assert_eq!(
                walk(&mut s, rip, below, rsp + 32, (low, high - 8)),
                (1, WalkEnd::Bad, 0, below)
            );
            assert_eq!(
                walk(&mut s, rip, below, 0x5eed_0000, (low, high)),
                (1, WalkEnd::Bad, 0, below)
            );
        }

        /// At an epilogue that ends in a tail call, the walk unwinds the frame itself (the pops
        /// and the return, as the processor runs them), not through `RtlVirtualUnwind`, which
        /// would look up the jump's target and can wait there on the loader's lock. Exactly the
        /// frame's words in the stack unwind it; with one word less the walk ends as a bad
        /// frame. The same with a BND prefix on the jump (`bnd jmp`), which the system skips
        /// as it decodes the epilogue: read as no epilogue, it went to the system, which looked
        /// the target up.
        #[test]
        fn a_tail_call_epilogue_is_unwound_by_the_walk() {
            let mut s = Sampler::new();
            s.refresh_images();
            let mut stack = vec![0u64; 0x40];
            let low = stack.as_mut_ptr() as u64;
            // After the `add rsp, 32`: the caller's rbx, then the return address (0: the
            // thread's start).
            let rsp = low + 0x80;
            stack[(0x80 + 32) / 8] = 0xb0b;
            let high = rsp + 48;
            for (jump, rip) in [
                ("jmp", nv_profile_test_tail_epilogue as Code),
                ("bnd jmp", nv_profile_test_bnd_tail_epilogue),
            ] {
                let rip = rip as usize as u64;
                let (n, end, unwinds, tail_calls, ctx) = walk_at(&mut s, rip, rsp, 0, (low, high));
                assert_eq!(
                    (n, end, unwinds, tail_calls),
                    (1, WalkEnd::Start, 0, 1),
                    "{jump}"
                );
                assert_eq!((ctx.rsp(), ctx.register(3)), (rsp + 48, 0xb0b), "{jump}");
                let (n, end, unwinds, tail_calls, ctx) =
                    walk_at(&mut s, rip, rsp, 0, (low, high - 8));
                assert_eq!(
                    (n, end, unwinds, tail_calls, ctx.rsp()),
                    (1, WalkEnd::Bad, 0, 0, rsp),
                    "{jump}, one word short"
                );
            }
        }

        /// A jump out of a part of a function (its unwind information chained to the first
        /// part's) into another part of the same function is no tail call, and
        /// `RtlVirtualUnwind` could tell only by looking the target up: the walk ends there as a
        /// bad frame. The two parts have two unwind entries and one function start.
        #[cfg(nv_chained_unwind_asm)]
        #[test]
        fn a_jump_to_another_part_of_the_function_ends_the_walk() {
            let mut s = Sampler::new();
            s.refresh_images();
            let split = nv_profile_test_split as Code as usize as u64;
            let cold = nv_profile_test_split_cold as Code as usize as u64;
            // SAFETY: the images were just listed.
            let (first, second) = unsafe {
                (
                    look_up(&s.images, &mut s.found, split).expect("in this program"),
                    look_up(&s.images, &mut s.found, cold).expect("in this program"),
                )
            };
            assert!(!first.entry.is_null() && !second.entry.is_null());
            assert_ne!(first.entry, second.entry, "two unwind entries");
            assert_eq!((first.key, second.key), (split, split), "one function");
            let mut stack = vec![0u64; 0x40];
            let low = stack.as_mut_ptr() as u64;
            let rsp = low + 0x80;
            let rip = nv_profile_test_split_jump as Code as usize as u64;
            let (n, end, unwinds, tail_calls, _) = walk_at(&mut s, rip, rsp, 0, (low, rsp + 16));
            assert_eq!((n, end, unwinds, tail_calls), (1, WalkEnd::Bad, 0, 0));
        }

        /// A jump from a chained part to its function's own start is a tail call (recursion):
        /// `RtlVirtualUnwind` takes a target in the same function for an epilogue when it is the
        /// first part's start, and only a jump elsewhere in the function for none. The walk
        /// unwinds such a frame itself, to the registers the system gives (asked here directly,
        /// with nothing suspended); with one word less the walk ends as a bad frame.
        #[cfg(nv_chained_unwind_asm)]
        #[test]
        fn a_jump_to_the_functions_own_start_is_a_tail_call() {
            let mut s = Sampler::new();
            s.refresh_images();
            let start = nv_profile_test_recurse as Code as usize as u64;
            let rip = nv_profile_test_recurse_pop as Code as usize as u64;
            // SAFETY: the images were just listed.
            let (first, code) = unsafe {
                (
                    look_up(&s.images, &mut s.found, start).expect("in this program"),
                    look_up(&s.images, &mut s.found, rip).expect("in this program"),
                )
            };
            assert!(!first.entry.is_null() && !code.entry.is_null());
            assert_ne!(first.entry, code.entry, "the pop is in the chained part");
            assert_eq!((first.key, code.key), (start, start), "one function");
            let mut stack = vec![0u64; 0x40];
            let low = stack.as_mut_ptr() as u64;
            // The saved rbx, then the return address (0: the thread's start).
            let rsp = low + 0x80;
            stack[0x80 / 8] = 0xb0b;
            let high = rsp + 16;
            let (n, end, unwinds, tail_calls, ctx) = walk_at(&mut s, rip, rsp, 0, (low, high));
            assert_eq!((n, end, unwinds, tail_calls), (1, WalkEnd::Start, 0, 1));
            assert_eq!((ctx.rsp(), ctx.register(3)), (rsp + 16, 0xb0b));
            let mut system = Box::new(Context([0; 0x4d0]));
            system.set_word(0xf8, rip);
            system.set_word(0x98, rsp);
            let mut handler_data = std::ptr::null_mut();
            let mut establisher = 0u64;
            // SAFETY: the frame's words lie in `stack`; no thread is suspended, so the target's
            // lookup cannot wait on a loader lock this thread needs.
            unsafe {
                RtlVirtualUnwind(
                    0,
                    code.base,
                    rip,
                    code.entry,
                    &mut *system,
                    &mut handler_data,
                    &mut establisher,
                    std::ptr::null_mut(),
                );
            }
            assert_eq!(
                (system.rip(), system.rsp(), system.register(3)),
                (ctx.rip(), ctx.rsp(), ctx.register(3)),
                "the system's unwind"
            );
            let (n, end, unwinds, tail_calls, ctx) = walk_at(&mut s, rip, rsp, 0, (low, high - 8));
            assert_eq!(
                (n, end, unwinds, tail_calls, ctx.rsp()),
                (1, WalkEnd::Bad, 0, 0, rsp),
                "one word short"
            );
        }

        /// A pc on the second byte of `pop r12` (41 5c) reads as `pop rsp`: `RtlVirtualUnwind`
        /// would take the popped word for the stack pointer and read the return address
        /// wherever it points (here memory of the test's own outside the stack). The walk ends
        /// there as a bad frame.
        #[test]
        fn a_pop_into_the_stack_pointer_ends_the_walk() {
            let mut s = Sampler::new();
            s.refresh_images();
            let mut stack = vec![0u64; 0x40];
            let mut elsewhere = vec![0u64; 4];
            let low = stack.as_mut_ptr() as u64;
            let rsp = low + 0x80;
            stack[0x80 / 8] = elsewhere.as_mut_ptr() as u64;
            let rip = nv_profile_test_r12_pop as Code as usize as u64 + 1;
            // SAFETY: the code of a function of this program.
            assert_eq!(unsafe { *(rip as *const u8) }, 0x5c);
            let (n, end, unwinds, tail_calls, ctx) = walk_at(&mut s, rip, rsp, 0, (low, rsp + 48));
            assert_eq!(
                (n, end, unwinds, tail_calls, ctx.rsp()),
                (1, WalkEnd::Bad, 0, 0, rsp)
            );
        }

        /// In a prologue that sets the frame register before a save whose offset's high byte
        /// ends in 3, the system finds the code that sets it code by code ([`frame_code_slot`]),
        /// so at a pc after the save it reads the save at the frame register less its offset.
        /// With the frame register 0x3680 bytes below the stack (a stray frame), that read is
        /// below the stack and the walk ends as a bad frame. Looked for slot by slot, the save's
        /// offset was taken for that code: the save's read was checked at the stack pointer, and
        /// the frame went to `RtlVirtualUnwind`, which read 0xf0 bytes below the stack (here
        /// memory of the test's own). With the frame register where the prologue set it, the
        /// frame unwinds.
        #[test]
        fn the_code_that_sets_the_frame_register_is_found_code_by_code() {
            let mut s = Sampler::new();
            s.refresh_images();
            let start = nv_profile_test_fpreg as Code as usize as u64;
            let rip = nv_profile_test_fpreg_saved as Code as usize as u64;
            // SAFETY: the images were just listed; the unwind information is this program's.
            unsafe {
                let code = look_up(&s.images, &mut s.found, rip).expect("in this program");
                assert!(!code.entry.is_null());
                let info = unwind_info(code.base, code.entry);
                let count = usize::from(*info.add(2));
                // The save (two slots, the second's high byte 03), then the code that sets the
                // frame register.
                let ops = (*info.add(5) & 0xf, *info.add(7), *info.add(9) & 0xf);
                assert_eq!((count, ops), (6, (8, 0x03, 3)));
                assert_eq!(frame_code_slot(info, count), Some(2));
                // The pc is past both codes and still in the prologue.
                let offset = rip - start;
                assert!(offset >= u64::from(*info.add(4)) && offset >= u64::from(*info.add(8)));
                assert!(offset < u64::from(*info.add(1)));
            }
            let mut memory = vec![0u64; 0x1000];
            let low = memory.as_mut_ptr() as u64 + 0x4000;
            let rsp = low + 0x100;
            // The frame: the save at 0x3610, the saved rbp at 0x3700, the return address (0).
            let high = rsp + 0x3710;
            // The frame base 0x3700 below the stack: the save 0xf0 below it, the saved rbp and
            // the return address at its bottom.
            let stray = low - 0x3680;
            let (n, end, unwinds, tail_calls, ctx) = walk_at(&mut s, rip, rsp, stray, (low, high));
            assert_eq!(
                (n, end, unwinds, tail_calls, ctx.rsp()),
                (1, WalkEnd::Bad, 0, 0, rsp)
            );
            let set = rsp + 128;
            let (n, end, unwinds, _, ctx) = walk_at(&mut s, rip, rsp, set, (low, high));
            assert_eq!((n, end, unwinds, ctx.rsp()), (1, WalkEnd::Start, 1, high));
            let (n, end, unwinds, _, _) = walk_at(&mut s, rip, rsp, set, (low, high - 8));
            assert_eq!((n, end, unwinds), (1, WalkEnd::Bad, 0));
        }

        /// A walk never waits on the loader. A thread of this program loads and frees a DLL in a
        /// loop and is suspended, as the sampler suspends the engine's thread, while walks at
        /// tail-call epilogues run (a plain `jmp` and a `bnd jmp` in turn): every walk comes
        /// back. Through `RtlVirtualUnwind`, a probe of the same loop got stuck within 9,000 to
        /// 95,000 walks (the `bnd jmp` alone, 10 times in 30,163 walks); here a watchdog resumes
        /// the thread after two seconds without a walk coming back, and the test fails.
        #[test]
        fn a_walk_at_a_tail_call_never_waits_on_the_loader() {
            use std::sync::atomic::{AtomicBool, AtomicU64, Ordering::Relaxed};
            use std::sync::Arc;
            // The images are listed before the loader starts: the walk needs only this
            // program's.
            let mut s = Sampler::new();
            s.refresh_images();
            let stop = Arc::new(AtomicBool::new(false));
            let (tx, rx) = std::sync::mpsc::channel();
            let loader = {
                let stop = stop.clone();
                std::thread::spawn(move || {
                    tx.send(current_thread_id()).expect("the test waits");
                    while !stop.load(Relaxed) {
                        // SAFETY: a system DLL loaded by name and freed again.
                        unsafe {
                            let module = LoadLibraryA(b"msimg32.dll\0".as_ptr());
                            if !module.is_null() {
                                FreeLibrary(module);
                            }
                        }
                    }
                })
            };
            let id = rx.recv().expect("the loader's id");
            let handle = s.thread(id).expect("a thread of this program");
            let mut stack = vec![0u64; 0x40];
            let low = stack.as_mut_ptr() as u64;
            let rsp = low + 0x80;
            // Walks at the plain and the BND-prefixed tail-call epilogue, in turn.
            let rips = [
                nv_profile_test_tail_epilogue as Code as usize as u64,
                nv_profile_test_bnd_tail_epilogue as Code as usize as u64,
            ];
            let steps = Arc::new(AtomicU64::new(0));
            let stuck = Arc::new(AtomicBool::new(false));
            // The watchdog must be running before the first suspend: a thread starting up takes
            // the loader's lock, which the suspended loader may hold.
            let (up_tx, up_rx) = std::sync::mpsc::channel();
            let watchdog = {
                let (stop, steps, stuck) = (stop.clone(), steps.clone(), stuck.clone());
                let handle = handle as usize;
                std::thread::spawn(move || {
                    up_tx.send(()).expect("the test waits");
                    let (mut last, mut still) = (0, 0);
                    while !stop.load(Relaxed) {
                        std::thread::sleep(Duration::from_millis(100));
                        let now = steps.load(Relaxed);
                        if now != last {
                            (last, still) = (now, 0);
                            continue;
                        }
                        still += 1;
                        if still == 20 {
                            stuck.store(true, Relaxed);
                            // SAFETY: the handle is open until the test ends.
                            unsafe { ResumeThread(handle as Handle) };
                            still = 0;
                        }
                    }
                })
            };
            up_rx.recv().expect("the watchdog runs");
            // Nothing allocates while the loader stands suspended (it may hold the heap's lock).
            let mut ctx = Box::new(Context([0; 0x4d0]));
            let (mut keys, mut pcs) = ([0u64; 4], [0u64; 4]);
            let mut wrong = 0u64;
            let started = Instant::now();
            while started.elapsed() < Duration::from_millis(1500) {
                ctx.set_word(0xf8, rips[(steps.load(Relaxed) % 2) as usize]);
                ctx.set_word(0x98, rsp);
                // SAFETY: the handle has suspend access; the images are listed and `stack` is
                // the test's own; only the walk runs between the suspend and the resume.
                let walked = unsafe {
                    if SuspendThread(handle) == u32::MAX {
                        break;
                    }
                    let walked = walk_stack(
                        &s.images,
                        &mut s.found,
                        &mut ctx,
                        (low, rsp + 48),
                        &mut keys,
                        &mut pcs,
                    );
                    ResumeThread(handle);
                    walked
                };
                wrong += u64::from(walked != (1, WalkEnd::Start));
                steps.fetch_add(1, Relaxed);
            }
            stop.store(true, Relaxed);
            loader.join().expect("the loader ends");
            watchdog.join().expect("the watchdog ends");
            let steps = steps.load(Relaxed);
            assert!(!stuck.load(Relaxed), "a walk waited after {steps} walks");
            assert!(steps > 100, "{steps} walks");
            assert_eq!(wrong, 0);
        }

        /// A handle the sampler opened only to read a thread's instruction pointer is closed
        /// once the thread has ended: when it can no longer be suspended, or when a listing of
        /// the process no longer shows it. A walked thread's handle stays open, its TEB being
        /// kept by its id.
        #[test]
        fn handles_of_ended_threads_are_closed_unless_walked() {
            let mut s = Sampler::new();
            for how in ["suspend", "listing", "walked"] {
                let (tx, rx) = std::sync::mpsc::channel();
                let (end_tx, end_rx) = std::sync::mpsc::channel::<()>();
                let t = std::thread::spawn(move || {
                    tx.send(current_thread_id()).expect("the test waits");
                    let _ = end_rx.recv();
                });
                let id = rx.recv().expect("the thread's id");
                assert!(s.instruction_pointer(id).is_some(), "{how}");
                if how == "walked" {
                    assert!(s.walk(id, 4).is_some());
                }
                assert!(s.threads.contains_key(&id), "{how}");
                end_tx.send(()).expect("the thread waits");
                t.join().expect("the thread ends");
                if how == "listing" {
                    assert!(!s.process_threads().contains(&id));
                } else {
                    assert_eq!(s.instruction_pointer(id), None, "{how}");
                }
                assert_eq!(s.threads.contains_key(&id), how == "walked", "{how}");
            }
        }

        /// A thread whose handle is closed once it has ended keeps its name for the table that
        /// holds its samples (read before the close: the id opens no more after it). Its id is
        /// then free, so a thread later given it is named afresh when the sampler opens it,
        /// not under the ended thread's name (as happened once the handles were closed: in a
        /// probe the 641st to 656th thread started got the id and was named as the ended one).
        #[test]
        fn a_closed_threads_name_is_kept_until_its_id_is_opened_again() {
            let named = |name: &str| {
                let (tx, rx) = std::sync::mpsc::channel();
                let (end_tx, end_rx) = std::sync::mpsc::channel::<()>();
                let t = std::thread::Builder::new()
                    .name(name.into())
                    .spawn(move || {
                        tx.send(current_thread_id()).expect("the test waits");
                        let _ = end_rx.recv();
                    })
                    .expect("a thread");
                (rx.recv().expect("the thread's id"), end_tx, t)
            };
            let mut s = Sampler::new();
            let (id, end, t) = named("closed probe");
            assert!(s.instruction_pointer(id).is_some());
            end.send(()).expect("the thread waits");
            t.join().expect("the thread ends");
            assert!(!s.process_threads().contains(&id));
            assert!(!s.threads.contains_key(&id), "the handle is closed");
            assert_eq!(s.thread_name(id), "closed probe");
            // A thread given an id under which another's name is kept (as the ended thread's
            // stays under its id) is named afresh once it is opened.
            let (other, end, t) = named("reused probe");
            s.thread_names.insert(other, "closed probe".into());
            assert!(s.instruction_pointer(other).is_some());
            assert_eq!(s.thread_name(other), "reused probe");
            end.send(()).expect("the thread waits");
            t.join().expect("the thread ends");
        }

        /// Headers that cannot be read are no image, not a fault: a module the listing names
        /// can be unloaded before they are read (another thread's `FreeLibrary`), its pages
        /// gone, as the first page of the address space always is. Read in place, this faulted
        /// and ended the whole test program. This program's own image still reads.
        #[test]
        fn headers_that_cannot_be_read_are_no_image() {
            assert!(image_at(0).is_none());
            assert!(image_at(0x1000).is_none());
            // SAFETY: GetModuleHandleA with no name answers this program's own image.
            let me = unsafe { GetModuleHandleA(std::ptr::null()) } as u64;
            let image = image_at(me).expect("this program's image");
            assert!(image.end > me && image.len > 0);
            let here = headers_that_cannot_be_read_are_no_image as fn() as usize as u64;
            assert!((me..image.end).contains(&here));
        }

        /// A listed image stays loaded until the next listing: walks read its unwind table and
        /// code in place, and one unloaded since the listing had them read memory that was gone
        /// or another module's (in a probe, a DLL freed after a listing and another loaded at
        /// its base: the walk read the old unwind table past the new module's end, an access
        /// violation in the sampler with the walked thread suspended). Freed by its user
        /// meanwhile, it is let go at the next listing, and when the sampler is dropped.
        #[test]
        fn a_listed_image_stays_loaded_until_the_next_listing() {
            // A system DLL nothing else in this program loads.
            const NAME: &[u8] = b"msdelta.dll\0";
            let loaded = || {
                // SAFETY: a module looked up by name; nothing is loaded or freed.
                !unsafe { GetModuleHandleA(NAME.as_ptr()) }.is_null()
            };
            // Whether the DLL, freed by its user, is unloaded within a minute: another test's
            // sampler that listed it meanwhile holds it until its own next listing or its end,
            // and `relist` lists again (letting go of what it holds) each time it looks.
            let unloaded_within_a_minute = |relist: &mut dyn FnMut()| {
                let started = Instant::now();
                loop {
                    relist();
                    if !loaded() {
                        return true;
                    }
                    if started.elapsed() > Duration::from_secs(60) {
                        return false;
                    }
                    std::thread::sleep(Duration::from_millis(50));
                }
            };
            assert!(!loaded(), "not loaded before the test");
            // SAFETY: a system DLL loaded by name and freed once.
            let module = unsafe { LoadLibraryA(NAME.as_ptr()) };
            assert!(!module.is_null());
            let mut s = Sampler::new();
            s.refresh_images();
            let im = s.images.iter().find(|im| im.base == module as u64);
            let im = im.expect("listed");
            assert!(im.len > 0);
            // SAFETY: the first entry of the unwind table of an image just listed.
            let pc = im.base + u64::from(unsafe { (*im.table).begin });
            // SAFETY: the reference the test took.
            unsafe { FreeLibrary(module) };
            // Checked before its unwind table is read: unloaded, the read would fault.
            assert!(loaded(), "held loaded while listed");
            assert_eq!(s.has_unwind_entry(pc), Some(true));
            // The next listing lets it go: it is unloaded and no longer listed.
            assert!(
                unloaded_within_a_minute(&mut || {
                    s.images_at = None;
                    s.refresh_images();
                }),
                "let go at the next listing"
            );
            assert!(s.images.iter().all(|im| im.base != module as u64));
            // Listed again while loaded, then let go with the sampler.
            // SAFETY: as above.
            let module = unsafe { LoadLibraryA(NAME.as_ptr()) };
            assert!(!module.is_null());
            s.images_at = None;
            s.refresh_images();
            // SAFETY: the reference the test took.
            unsafe { FreeLibrary(module) };
            assert!(loaded(), "held loaded while listed");
            drop(s);
            assert!(
                unloaded_within_a_minute(&mut || {}),
                "let go with the sampler"
            );
        }

        /// The ids of the threads of process `pid` (a Toolhelp snapshot).
        fn threads_of(pid: u32) -> Vec<u32> {
            let mut ids = Vec::new();
            // SAFETY: a Toolhelp snapshot walked with a correctly sized entry, then closed.
            unsafe {
                let snapshot = CreateToolhelp32Snapshot(SNAP_THREAD, 0);
                assert!(!snapshot.is_null() && snapshot as isize != -1);
                let mut entry = ThreadEntry {
                    size: std::mem::size_of::<ThreadEntry>() as u32,
                    ..Default::default()
                };
                let mut more = Thread32First(snapshot, &mut entry) != 0;
                while more {
                    if entry.process == pid {
                        ids.push(entry.thread);
                    }
                    more = Thread32Next(snapshot, &mut entry) != 0;
                }
                CloseHandle(snapshot);
            }
            ids
        }

        /// The sampler opens no thread of another process (an id that a thread of this process
        /// held, given to another process's thread after it ended): it would suspend that thread
        /// and read its stack's bounds as this process's memory. Its own threads it opens.
        #[test]
        fn a_thread_of_another_process_is_not_opened() {
            let mut child = std::process::Command::new("ping")
                .args(["-n", "30", "127.0.0.1"])
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .spawn()
                .expect("ping starts");
            let theirs = threads_of(child.id());
            let mut s = Sampler::new();
            let opened: Vec<u32> = theirs
                .iter()
                .copied()
                .filter(|&id| s.thread(id).is_some())
                .collect();
            let _ = child.kill();
            let _ = child.wait();
            assert!(!theirs.is_empty());
            assert_eq!(opened, Vec::<u32>::new());
            // The refusal is not kept: the id is tried again if a thread of this process gets
            // it later.
            let kept: Vec<u32> = theirs
                .iter()
                .copied()
                .filter(|id| s.threads.contains_key(id))
                .collect();
            assert_eq!(kept, Vec::<u32>::new());
            assert!(s.thread(current_thread_id()).is_some());
        }
    }
}

#[cfg(not(all(windows, target_arch = "x86_64")))]
mod native {
    //! No native sampling here: the exe-address samples only.
    pub fn current_thread_id() -> u32 {
        0
    }

    pub struct Sampler;

    impl Sampler {
        pub fn new() -> Self {
            Sampler
        }
        pub fn wait_tick(&mut self) {
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
        pub fn instruction_pointer(&mut self, _id: u32) -> Option<u64> {
            None
        }
        pub fn name(&mut self, _addr: u64) -> (String, String) {
            ("?".into(), "?".into())
        }
        pub fn process_threads(&mut self) -> Vec<u32> {
            Vec::new()
        }
        pub fn thread_name(&mut self, _id: u32) -> String {
            String::new()
        }
        pub fn walk(&mut self, _id: u32, _depth: usize) -> Option<super::Walk> {
            None
        }
        pub fn keys(&self, _n: usize) -> &[u64] {
            &[]
        }
        pub fn symbol_source(&mut self) -> String {
            "no native symbols on this platform".into()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn symbols_read_as_paths() {
        assert_eq!(
            readable_symbol("_RNvNtNtNtCshZYmvcS8kkr_11engine_core5units3crt4math9floor_x87"),
            "engine_core::units::crt::math::floor_x87"
        );
        assert_eq!(
            readable_symbol(
                "_RNvNtNtCshZYmvcS8kkr_11engine_core5units3crt20set_x87_control_word.llvm.123"
            ),
            "engine_core::units::crt::set_x87_control_word"
        );
        assert_eq!(
            readable_symbol("_ZN4core3ptr13drop_in_place17h0123456789abcdefE"),
            "core::ptr::drop_in_place"
        );
        assert_eq!(readable_symbol("memcpy"), "memcpy");
    }

    #[test]
    fn platform_addresses_have_kinds() {
        assert_eq!(
            kind_of(0x7e02_0210).as_deref(),
            Some("D3D9/D3DX platform object")
        );
        assert_eq!(
            kind_of(0x7e00_0410).as_deref(),
            Some("DirectSound platform object")
        );
        assert_eq!(kind_of(0x0086_e650), None);
    }

    /// The table counts phases, threads, kinds and self and inclusive samples as sampled.
    #[test]
    fn the_table_reads_back_its_samples() {
        let mut t = Table {
            samples: 4,
            phases: [1, 3, 0, 0],
            ..Default::default()
        };
        t.threads.insert(1, 3);
        t.kinds.insert("exe code".into(), 2);
        t.kinds.insert("D3D9/D3DX platform object".into(), 1);
        t.selfs.insert(0x0086_e650, 2);
        t.selfs.insert(0x7e02_0210, 1);
        t.inclusive.insert(0x0086_e650, 3);
        let text = report(
            &t,
            Duration::from_secs(1),
            10,
            Rows { top: 5, calls: 5 },
            &mut native::Sampler::new(),
        );
        assert!(
            text.contains("10 game frames (10.0 frames/s, 100.00 ms a frame)"),
            "{text}"
        );
        assert!(text.contains("game 75.0% (75.00 ms/frame)"), "{text}");
        assert!(text.contains("viewer 25.0%"), "{text}");
        assert!(text.contains("exe code 66.7%"), "{text}");
        assert!(
            text.contains(
                " 50.0% 50.000  0086e650
"
            ),
            "{text}"
        );
        assert!(
            text.contains("7e020210  D3D9/D3DX platform object"),
            "{text}"
        );
        assert!(
            text.contains(
                " 75.0% 75.000  0086e650
"
            ),
            "{text}"
        );
    }

    #[test]
    fn threads_group_by_name_and_waits_are_idle() {
        assert_eq!(thread_group("Compute Task Pool (3)"), "Compute Task Pool");
        assert_eq!(thread_group("game thread 12"), "game thread");
        assert_eq!(thread_group(""), "unnamed");
        assert!(is_waiting("ntdll.dll", "ZwWaitForAlertByThreadId"));
        assert!(is_waiting("ntdll.dll", "NtDelayExecution"));
        assert!(!is_waiting("ntdll.dll", "RtlAllocateHeap"));
        assert!(!is_waiting("nv-viewer.exe", "Wait"));
    }

    /// The process's threads are listed, and a named one reads back its name.
    #[cfg(all(windows, target_arch = "x86_64"))]
    #[test]
    fn the_process_threads_are_listed_with_their_names() {
        let (tx, rx) = std::sync::mpsc::channel();
        let (stop_tx, stop_rx) = std::sync::mpsc::channel::<()>();
        let named = std::thread::Builder::new()
            .name("a29 probe (7)".into())
            .spawn(move || {
                tx.send(native::current_thread_id()).unwrap();
                let _ = stop_rx.recv();
            })
            .unwrap();
        let id = rx.recv().unwrap();
        let mut sampler = native::Sampler::new();
        let ids = sampler.process_threads();
        assert!(ids.contains(&id), "{ids:?}");
        assert_eq!(thread_group(&sampler.thread_name(id)), "a29 probe");
        stop_tx.send(()).unwrap();
        named.join().unwrap();
    }

    /// The native sampler reads a busy thread's instruction pointer, inside this program.
    #[cfg(all(windows, target_arch = "x86_64"))]
    #[test]
    fn a_busy_thread_is_sampled_in_this_program() {
        use std::sync::atomic::AtomicBool;
        use std::sync::Arc;
        let stop = Arc::new(AtomicBool::new(false));
        let id = Arc::new(AtomicU32::new(0));
        let (s, i) = (stop.clone(), id.clone());
        let busy = std::thread::spawn(move || {
            i.store(native::current_thread_id(), Relaxed);
            let mut n = 0u64;
            while !s.load(Relaxed) {
                n = std::hint::black_box(n.wrapping_add(1));
            }
            n
        });
        while id.load(Relaxed) == 0 {
            std::thread::yield_now();
        }
        let mut sampler = native::Sampler::new();
        let rip = sampler.instruction_pointer(id.load(Relaxed));
        stop.store(true, Relaxed);
        busy.join().unwrap();
        let rip = rip.expect("an instruction pointer");
        let (module, _) = sampler.name(rip);
        let exe = std::env::current_exe().unwrap();
        let exe = exe
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_ascii_lowercase();
        assert_eq!(module, exe);
        let (_, symbol) = sampler.name(readable_symbol as fn(&str) -> String as usize as u64 + 1);
        // Named from the test program's PDB, found next to it.
        assert!(symbol.ends_with("readable_symbol"), "{symbol}");
        // The sampler never samples itself.
        assert_eq!(
            sampler.instruction_pointer(native::current_thread_id()),
            None
        );
    }

    /// Calls are counted by target: a target's own slot, and the next free one for a target
    /// whose slot another holds.
    #[test]
    fn calls_are_counted_by_target() {
        let a = 0x00c1_2340u32;
        let b = (a + 1..)
            .find(|&b| target_slot(b) == target_slot(a))
            .expect("an address with the same slot");
        let before = target_counts();
        for _ in 0..3 {
            count_target(a);
        }
        count_target(b);
        let uncounted = TARGET_UNCOUNTED.load(Relaxed);
        count_target(0);
        assert_eq!(TARGET_UNCOUNTED.load(Relaxed), uncounted + 1);
        let calls = target_calls_since(&target_counts(), &before);
        assert_eq!(calls.get(&a), Some(&3));
        assert_eq!(calls.get(&b), Some(&1));
        assert_eq!(calls.get(&0), None);
    }

    /// With the thread count on, a target's calls are split between main and the other game
    /// threads, with the threads that called it, and the table says where it runs.
    #[test]
    fn calls_are_counted_by_thread() {
        let (both, main_only, other_only) = (0x00c5_6780u32, 0x00c5_6790u32, 0x00c5_67a0u32);
        let was = COUNT_THREADS.swap(true, Relaxed);
        let thread = GAME_THREAD.load(Relaxed);
        GAME_THREAD.store(crate::thread_host::MAIN_THREAD_ID, Relaxed);
        count_target(both);
        count_target(both);
        count_target(main_only);
        GAME_THREAD.store(7, Relaxed);
        count_target(both);
        count_target(other_only);
        GAME_THREAD.store(70, Relaxed);
        count_target(other_only);
        GAME_THREAD.store(thread, Relaxed);
        COUNT_THREADS.store(was, Relaxed);
        let rows = thread_calls();
        let row = |a: u32| rows.iter().find(|r| r.0 == a).copied();
        assert_eq!(row(both), Some((both, 2, 1, 1 << 1 | 1 << 7)));
        assert_eq!(row(main_only), Some((main_only, 1, 0, 1 << 1)));
        assert_eq!(row(other_only), Some((other_only, 0, 2, 1 << 7 | 1 << 63)));
        let text = thread_calls_tsv();
        assert!(text.contains("\n00c56780\t3\t2\t1\t1,7\tboth\n"), "{text}");
        assert!(text.contains("\n00c56790\t1\t1\t0\t1\tmain\n"), "{text}");
        assert!(
            text.contains("\n00c567a0\t2\t0\t2\t7,63+\tother\n"),
            "{text}"
        );
    }

    /// The call-target table: calls a frame, share, self time a frame and a call, inclusive.
    #[test]
    fn call_targets_read_back_their_counts() {
        let mut t = Table {
            samples: 4,
            phases: [1, 3, 0, 0],
            calls: 20_000,
            ..Default::default()
        };
        t.targets.insert(0x00b7_dab0, 10_000);
        t.targets.insert(0x7e02_0210, 5_000);
        t.selfs.insert(0x00b7_dab0, 2);
        t.inclusive.insert(0x00b7_dab0, 3);
        let text = report(
            &t,
            Duration::from_secs(1),
            10,
            Rows { top: 5, calls: 5 },
            &mut native::Sampler::new(),
        );
        assert!(
            text.contains("call targets: 2 distinct, 2000 calls a frame (0 not counted"),
            "{text}"
        );
        assert!(
            text.contains("      1000.0 50.00% 50.000 50000.0 75.000  00b7dab0\n"),
            "{text}"
        );
        assert!(
            text.contains(
                "       500.0 25.00%  0.000     0.0  0.000  7e020210  D3D9/D3DX platform object\n"
            ),
            "{text}"
        );
    }

    /// The walked stacks' tables: inclusive once a sample, self by the innermost frame, callers
    /// by the nearest frame with another name, keys of one name counted as one function.
    #[test]
    fn native_stacks_count_inclusive_self_and_callers() {
        let mut t = Table {
            samples: 10,
            walks: 5,
            walk_frames: 16,
            walk_ends: [4, 1, 0, 0],
            suspended_ns: 50_000,
            ..Default::default()
        };
        // 1: "leaf", 2: "mid", 3 and 4: "root" (two copies), 5: "other".
        t.stacks.insert(vec![1, 2, 3].into(), 2);
        t.stacks.insert(vec![1, 5].into(), 1);
        t.stacks.insert(vec![1, 1, 2, 2, 4, 3].into(), 1);
        t.stacks.insert(vec![5, 3].into(), 1);
        let name = |k: u64| match k {
            1 => "leaf",
            2 => "mid",
            3 | 4 => "root",
            _ => "other",
        };
        let mut out = String::new();
        native_stacks(&mut out, &t, 10, &|s: u64| s as f64, &mut |k: u64| {
            name(k).to_string()
        });
        assert!(
            out.contains(
                "5 walked (50.0% of all samples), 3.2 frames on average, the thread \
                 suspended 10.0 us a walk; ended at:  the thread's start 80.0%  the frame \
                 limit 20.0%"
            ),
            "{out}"
        );
        // Inclusive: root 4 (once in the stack with both copies), leaf 4, mid 3, other 2.
        assert!(out.contains("   40.0%  4.000  leaf\n"), "{out}");
        assert!(out.contains("   40.0%  4.000  root\n"), "{out}");
        assert!(out.contains("   30.0%  3.000  mid\n"), "{out}");
        assert!(out.contains("   20.0%  2.000  other\n"), "{out}");
        // leaf's callers: mid 3 (the repeated leaf frame skipped), other 1.
        let callers = out.split("by self").nth(1).expect("the callers table");
        assert!(
            callers.contains(
                "   40.0%  4.000  leaf\n         30.0%  3.000  75.0%  < mid\n         10.0%  1.000  25.0%  < other\n"
            ),
            "{out}"
        );
        assert!(
            callers.contains("   10.0%  1.000  other\n         10.0%  1.000 100.0%  < root\n"),
            "{out}"
        );
    }

    #[cfg(all(windows, target_arch = "x86_64"))]
    mod walks {
        use super::super::*;
        use std::sync::atomic::AtomicBool;
        use std::sync::Arc;

        #[link(name = "kernel32")]
        extern "system" {
            fn RtlCaptureStackBackTrace(
                skip: u32,
                count: u32,
                frames: *mut *mut std::ffi::c_void,
                hash: *mut u32,
            ) -> u16;
        }

        // A leaf without unwind data (no `.seh_` directives, so no unwind entry): stores 1 in
        // the word `rdx` points at, then spins until the word `rcx` points at is not 0.
        std::arch::global_asm!(
            ".globl nv_profile_test_leaf",
            "nv_profile_test_leaf:",
            "mov dword ptr [rdx], 1",
            "2:",
            "pause",
            "cmp dword ptr [rcx], 0",
            "je 2b",
            "ret",
        );

        // Code without unwind data that is no leaf (as a hook's stub can be): it pushes `rcx`
        // five times (the callee's home area and the stack's alignment), calls `r9` with `rdx`
        // and `r8` as its first two arguments, and returns.
        std::arch::global_asm!(
            ".globl nv_profile_test_stub",
            "nv_profile_test_stub:",
            "push rcx",
            "push rcx",
            "push rcx",
            "push rcx",
            "push rcx",
            "mov rcx, rdx",
            "mov rdx, r8",
            "call r9",
            "add rsp, 40",
            "ret",
        );

        type Leaf = unsafe extern "C" fn(*const AtomicU32, *const AtomicU32);
        type Stub = unsafe extern "C" fn(u64, *const AtomicU32, *const AtomicU32, Leaf);

        extern "C" {
            fn nv_profile_test_leaf(stop: *const AtomicU32, ready: *const AtomicU32);
            fn nv_profile_test_stub(
                word: u64,
                stop: *const AtomicU32,
                ready: *const AtomicU32,
                leaf: Leaf,
            );
        }

        static STOP: AtomicBool = AtomicBool::new(false);
        static READY: AtomicBool = AtomicBool::new(false);
        static CAPTURED: Mutex<Vec<u64>> = Mutex::new(Vec::new());

        /// Where a function's code is (a jump stub's target, if the linker made one).
        fn code_of(f: usize) -> u64 {
            // SAFETY: `f` is a function's address; its first five bytes are code.
            unsafe {
                let p = f as *const u8;
                if *p == 0xe9 {
                    let rel = std::ptr::read_unaligned(p.add(1) as *const i32);
                    return (f as i64 + 5 + i64::from(rel)) as u64;
                }
            }
            f as u64
        }

        /// The return addresses above the calling function, as the system's own walk
        /// (`RtlCaptureStackBackTrace`) gives them: the first is the calling function's own.
        #[inline(never)]
        fn capture() -> Vec<u64> {
            let mut frames = [std::ptr::null_mut(); 62];
            // SAFETY: the buffer holds the 62 frames asked for.
            let n = unsafe {
                RtlCaptureStackBackTrace(2, 62, frames.as_mut_ptr(), std::ptr::null_mut())
            };
            frames[..usize::from(n)].iter().map(|&p| p as u64).collect()
        }

        /// The bottom of the walked thread: its own backtrace taken, then a spin.
        #[inline(never)]
        fn spin_here() -> u64 {
            *CAPTURED.lock().unwrap() = capture();
            READY.store(true, Relaxed);
            let mut n = 0u64;
            while !STOP.load(Relaxed) {
                n = std::hint::black_box(n.wrapping_add(1));
            }
            n
        }

        #[inline(never)]
        fn inner_frame() -> u64 {
            std::hint::black_box(spin_here()).wrapping_add(1)
        }

        #[inline(never)]
        fn outer_frame() -> u64 {
            std::hint::black_box(inner_frame()).wrapping_add(2)
        }

        /// A busy thread's walked stack has its functions in order (keys at their starts) and
        /// the same return addresses as the thread's own `RtlCaptureStackBackTrace`, down to
        /// its start, in every walk; the walks after the first answer their lookups from the
        /// kept ones.
        #[test]
        fn a_busy_threads_stack_is_walked_to_its_start() {
            let id = Arc::new(AtomicU32::new(0));
            let i = id.clone();
            let busy = std::thread::spawn(move || {
                i.store(native::current_thread_id(), Relaxed);
                outer_frame()
            });
            while !READY.load(Relaxed) {
                std::thread::yield_now();
            }
            let captured = CAPTURED.lock().unwrap().clone();
            let spin = code_of(spin_here as fn() -> u64 as usize);
            let inner = code_of(inner_frame as fn() -> u64 as usize);
            let outer = code_of(outer_frame as fn() -> u64 as usize);
            let mut sampler = native::Sampler::new();
            let mut failed = None;
            for walk in 0..20 {
                let (hits, _) = sampler.lookup_counts();
                let w = sampler.walk(id.load(Relaxed), 64);
                let (now, _) = sampler.lookup_counts();
                let Some(w) = w else {
                    failed = Some(format!("walk {walk}: the thread could not be walked"));
                    break;
                };
                let keys = sampler.keys(w.frames).to_vec();
                let pcs = sampler.pcs(w.frames).to_vec();
                // The thread spins in `spin_here` (the innermost frame, or one after it when the
                // walk catches the thread in a helper the spin calls, as unoptimised builds do:
                // the atomic load, `black_box`); the capture's first address is the return into
                // `inner_frame`. Every frame but the innermost (its address moves in the spin)
                // was looked up in an earlier walk, though a few can have lost their slot to
                // another frame of the walk.
                let at = keys.iter().position(|&k| k == spin).unwrap_or(usize::MAX);
                let fine = w.end == WalkEnd::Start
                    && keys.len() >= at.saturating_add(3)
                    && keys[at..at + 3] == [spin, inner, outer]
                    && pcs[at + 1..] == captured[..]
                    && (walk == 0 || now - hits >= (w.frames as u64 - 1) / 2);
                if !fine {
                    failed = Some(format!(
                        "walk {walk}: {w:?}\nkeys {keys:x?}\npcs {pcs:x?}\ncaptured {captured:x?}\n\
                         kept lookups used {}",
                        now - hits
                    ));
                    break;
                }
            }
            STOP.store(true, Relaxed);
            busy.join().unwrap();
            assert_eq!(failed, None);
        }

        /// The thread state the leaf and stub tests share with their walked thread: its stop and
        /// ready words, its id and its capture.
        #[derive(Default)]
        struct Walked {
            stop: AtomicU32,
            ready: AtomicU32,
            id: AtomicU32,
            captured: Mutex<Vec<u64>>,
        }

        /// Starts `body` on a thread of its own and waits until it is ready to be walked.
        fn start_walked(body: fn(&Walked)) -> (Arc<Walked>, std::thread::JoinHandle<()>) {
            let state = Arc::new(Walked::default());
            let s = state.clone();
            let thread = std::thread::spawn(move || {
                s.id.store(native::current_thread_id(), Relaxed);
                body(&s);
            });
            while state.ready.load(Relaxed) == 0 {
                std::thread::yield_now();
            }
            (state, thread)
        }

        /// The walked thread of the leaf test: its own backtrace taken, then the leaf called.
        #[inline(never)]
        fn call_the_leaf(s: &Walked) {
            *s.captured.lock().unwrap() = capture();
            // SAFETY: the leaf reads `stop` and writes `ready`, both live for the call.
            unsafe { nv_profile_test_leaf(&s.stop, &s.ready) };
            std::hint::black_box(s);
        }

        /// A thread stopped in a leaf (here code without unwind data) is walked from the return
        /// address at its stack pointer: the leaf's frame keeps its own address (there is no
        /// unwind entry to name its function) and the frames after it are its caller's and the
        /// return addresses the thread's own capture has.
        #[test]
        fn a_leaf_at_the_innermost_frame_is_unwound_from_the_stack_pointer() {
            let leaf = code_of(nv_profile_test_leaf as Leaf as usize);
            let caller = code_of(call_the_leaf as fn(&Walked) as usize);
            let (state, thread) = start_walked(call_the_leaf);
            let mut sampler = native::Sampler::new();
            let walks: Vec<_> = (0..20)
                .map(|_| {
                    let w = sampler.walk(state.id.load(Relaxed), 64)?;
                    Some((
                        w,
                        sampler.keys(w.frames).to_vec(),
                        sampler.pcs(w.frames).to_vec(),
                    ))
                })
                .collect();
            state.stop.store(1, Relaxed);
            thread.join().unwrap();
            let captured = state.captured.lock().unwrap().clone();
            assert_eq!(sampler.has_unwind_entry(leaf), Some(false));
            for walk in walks {
                let (w, keys, pcs) = walk.expect("a walk");
                let last = format!("{w:?}\nkeys {keys:x?}\npcs {pcs:x?}\ncaptured {captured:x?}");
                assert_eq!(w.end, WalkEnd::Start, "{last}");
                assert!((leaf..leaf + 16).contains(&pcs[0]), "{last}");
                assert_eq!(keys[0], pcs[0], "{last}");
                assert_eq!(keys[1], caller, "{last}");
                assert_eq!(&pcs[2..], &captured[..], "{last}");
            }
        }

        /// The walked thread of the stub test: the stub pushes a word that looks like a return
        /// address (one into `call_the_leaf`, a function with an unwind entry) and calls the
        /// leaf.
        #[inline(never)]
        fn call_the_stub(s: &Walked) {
            let word = code_of(call_the_leaf as fn(&Walked) as usize) + 8;
            // SAFETY: the stub calls the leaf with `stop` and `ready`, both live for the call.
            unsafe { nv_profile_test_stub(word, &s.stop, &s.ready, nv_profile_test_leaf) };
            std::hint::black_box(s);
        }

        /// A return into code without unwind data ends the walk as a bad frame: that code
        /// called, so it is no leaf, and the word at the stack pointer there is one it pushed,
        /// not its return address (taken for one, it would send the walk into whatever function
        /// the word points into, and `RtlVirtualUnwind` on past the thread's stack).
        #[test]
        fn a_return_into_code_without_unwind_data_ends_the_walk() {
            let stub = code_of(nv_profile_test_stub as Stub as usize);
            let (state, thread) = start_walked(call_the_stub);
            let mut sampler = native::Sampler::new();
            let walks: Vec<_> = (0..20)
                .map(|_| {
                    let w = sampler.walk(state.id.load(Relaxed), 64)?;
                    Some((w, sampler.pcs(w.frames).to_vec()))
                })
                .collect();
            state.stop.store(1, Relaxed);
            thread.join().unwrap();
            assert_eq!(sampler.has_unwind_entry(stub), Some(false));
            for walk in walks {
                let (w, pcs) = walk.expect("a walk");
                assert_eq!((w.frames, w.end), (2, WalkEnd::Bad), "{w:?} {pcs:x?}");
                assert!((stub..stub + 32).contains(&pcs[1]), "{w:?} {pcs:x?}");
            }
        }

        /// A thread waiting in the OS (its innermost function a system-call stub of ntdll, which
        /// has an unwind entry) is walked to its start.
        #[test]
        fn a_waiting_threads_stack_is_walked_to_its_start() {
            let (id_tx, id_rx) = std::sync::mpsc::channel();
            let (stop_tx, stop_rx) = std::sync::mpsc::channel::<()>();
            let waiting = std::thread::spawn(move || {
                id_tx.send(native::current_thread_id()).unwrap();
                let _ = stop_rx.recv();
            });
            let id = id_rx.recv().unwrap();
            let mut sampler = native::Sampler::new();
            let mut last = String::new();
            let mut reached = false;
            for _ in 0..200 {
                std::thread::sleep(Duration::from_millis(1));
                if let Some(w) = sampler.walk(id, 64) {
                    last = format!("{w:?} {:x?}", sampler.keys(w.frames));
                    if w.end == WalkEnd::Start && w.frames >= 4 {
                        reached = true;
                        break;
                    }
                }
            }
            stop_tx.send(()).unwrap();
            waiting.join().unwrap();
            assert!(reached, "{last}");
        }
    }

    /// `NV_PROFILE_SECONDS` takes any number of seconds a `Duration` holds; anything else
    /// (infinite, too big, not a number, not above 0, unset) gives the 10 s default instead of
    /// stopping the frame that starts the sampler.
    #[test]
    fn the_table_period_takes_any_value() {
        assert_eq!(table_period(None), Duration::from_secs(10));
        assert_eq!(table_period(Some("2.5")), Duration::from_millis(2500));
        assert_eq!(
            table_period(Some("1e9")),
            Duration::from_secs(1_000_000_000)
        );
        for value in ["inf", "1e20", "NaN", "-1", "0", "ten", ""] {
            assert_eq!(
                table_period(Some(value)),
                Duration::from_secs(10),
                "{value}"
            );
        }
    }

    /// The shadow stack follows push, pop and resync (one test: the stack is process-wide).
    #[test]
    fn the_shadow_stack_follows_the_calls() {
        push(0x0040_1000);
        push(0x0040_2000);
        assert_eq!(
            STACK[DEPTH.load(Relaxed) as usize - 1].load(Relaxed),
            0x0040_2000
        );
        pop();
        assert_eq!(
            STACK[DEPTH.load(Relaxed) as usize - 1].load(Relaxed),
            0x0040_1000
        );
        pop();
        pop();
        assert_eq!(DEPTH.load(Relaxed), 0);
        fill(&[1, 2, 3]);
        assert_eq!(DEPTH.load(Relaxed), 3);
        assert_eq!(STACK[2].load(Relaxed), 3);
        DEPTH.store(0, Relaxed);
    }
}

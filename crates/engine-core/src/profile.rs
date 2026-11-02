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
//!   the table is printed (the exe's public symbols, so a time inside an inlined helper counts
//!   under the function it was inlined into).
//!
//! Every `NV_PROFILE_SECONDS` (default 10) it prints the phases, the game threads, where the game's
//! own time goes by kind (exe code, the Direct3D 9 / DirectSound / DirectInput platform objects,
//! imported functions by name), the top `NV_PROFILE_TOP` (default 25) exe functions by self and
//! inclusive samples, and the top native functions in the game's and the viewer's time.
//!
//! Off (the default), each `Engine::call` pays one relaxed load of [`on`].

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, AtomicU8, Ordering::Relaxed};
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};

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
    });
}

/// A function entered through `Engine::call` (profiler on only).
#[inline]
pub fn push(addr: u32) {
    CALLS.store(CALLS.load(Relaxed).wrapping_add(1), Relaxed);
    let d = DEPTH.load(Relaxed);
    if let Some(slot) = STACK.get(d as usize) {
        slot.store(addr, Relaxed);
    }
    DEPTH.store(d.wrapping_add(1), Relaxed);
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
        let seconds = std::env::var("NV_PROFILE_SECONDS")
            .ok()
            .and_then(|v| v.parse::<f64>().ok())
            .filter(|s| *s > 0.0)
            .unwrap_or(10.0);
        let top = std::env::var("NV_PROFILE_TOP")
            .ok()
            .and_then(|v| v.parse::<usize>().ok())
            .unwrap_or(25);
        println!("NV_PROFILE: sampling every millisecond, a table every {seconds} s");
        let _ = std::thread::Builder::new()
            .name("nv-profile".into())
            .spawn(move || sampler(Duration::from_secs_f64(seconds), top));
    });
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
}

fn sampler(period: Duration, top: usize) {
    let mut table = Table::default();
    let mut started = Instant::now();
    let mut frames_at = FRAMES.load(Relaxed);
    let mut calls_at = CALLS.load(Relaxed);
    let mut misses_at = CACHE_MISSES.load(Relaxed);
    let mut stack = Vec::with_capacity(MAX_DEPTH);
    let mut seen: Vec<u32> = Vec::with_capacity(MAX_DEPTH);
    let mut sampler = native::Sampler::new();
    let all_threads = std::env::var("NV_PROFILE_THREADS").is_ok_and(|v| v == "1");
    let mut tick = 0u64;
    let mut others: Vec<u32> = Vec::new();
    let mut others_at: Option<Instant> = None;
    loop {
        sampler.wait_tick();
        tick += 1;
        let phase = Phase::from_u8(PHASE.load(Relaxed));
        let os_thread = OS_THREAD.load(Relaxed);
        let rip = sampler.instruction_pointer(os_thread);
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
            let text = report(&table, elapsed, frames - frames_at, top, &mut sampler);
            // The frame numbers (the host's count of main's frames) place the table in a run.
            let calls = CALLS.load(Relaxed).wrapping_sub(calls_at);
            let per_frame = calls / (frames - frames_at).max(1);
            let misses =
                CACHE_MISSES.load(Relaxed).wrapping_sub(misses_at) / (frames - frames_at).max(1);
            println!(
                "== NV_PROFILE: the game's frames {frames_at} to {frames}, \
                 {per_frame} calls through Engine::call a frame ({misses} not in its cache)\n{text}"
            );
            table = Table::default();
            started = Instant::now();
            frames_at = FRAMES.load(Relaxed);
            calls_at = CALLS.load(Relaxed);
            misses_at = CACHE_MISSES.load(Relaxed);
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

fn report(
    t: &Table,
    elapsed: Duration,
    frames: u64,
    top: usize,
    sampler: &mut native::Sampler,
) -> String {
    use std::fmt::Write;
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
    if !t.other_samples.is_empty() {
        other_threads(&mut out, t, top, &ms_per_frame, sampler);
    }
    out
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
    //! The OS thread holding the engine: suspended for a moment, its instruction pointer read,
    //! resumed (nothing that could take a lock runs while it is suspended); named through
    //! `dbghelp.dll`, loaded when the first table is printed.
    use std::collections::HashMap;
    use std::ffi::c_void;
    use std::time::Duration;

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

    pub struct Sampler {
        threads: HashMap<u32, Handle>,
        timer: Handle,
        context: Box<Context>,
        sym_from_addr: Option<SymFromAddr>,
        sym_tried: bool,
        names: HashMap<u64, (String, String)>,
        modules: HashMap<usize, String>,
        thread_names: HashMap<u32, String>,
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
            Sampler {
                threads: HashMap::new(),
                timer,
                context: Box::new(Context([0; 0x4d0])),
                sym_from_addr: None,
                sym_tried: false,
                names: HashMap::new(),
                modules: HashMap::new(),
                thread_names: HashMap::new(),
            }
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
            let handle = *self.threads.entry(id).or_insert_with(|| {
                // SAFETY: OpenThread with a thread id; a null handle is checked below.
                unsafe {
                    OpenThread(
                        THREAD_SUSPEND_RESUME | THREAD_GET_CONTEXT | THREAD_QUERY_INFORMATION,
                        0,
                        id,
                    )
                }
            });
            if handle.is_null() {
                return None;
            }
            let ctx = &mut *self.context;
            ctx.0[0x30..0x34].copy_from_slice(&CONTEXT_CONTROL.to_le_bytes());
            // SAFETY: the handle has suspend/resume and get-context access; nothing between
            // the suspend and the resume allocates or takes a lock (the context buffer is ours).
            let ok = unsafe {
                if SuspendThread(handle) == u32::MAX {
                    return None;
                }
                let ok = GetThreadContext(handle, ctx as *mut Context) != 0;
                ResumeThread(handle);
                ok
            };
            ok.then(|| u64::from_le_bytes(ctx.0[0xf8..0x100].try_into().expect("8 bytes")))
        }

        /// The ids of every thread of this process but the sampler's own.
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
            ids
        }

        /// A thread's name (`GetThreadDescription`), empty without one.
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
                // dbghelp looks for a module's PDB on its search path, not next to the module: the
                // program's folder, then `NV_PROFILE_SYMBOLS` (folders separated by `;`).
                let mut path = std::env::current_exe()
                    .ok()
                    .and_then(|p| p.parent().map(|d| d.to_string_lossy().into_owned()))
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
            5,
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

//! `NV_QUEUE_CHECK`: diagnostic only, not exe behaviour. Reads, never writes, game memory.
//!
//! Follows the game's background tasks (`BSTask`, the base of `IOTask` and the queued files:
//! constructor `00449090`, destructor `004490e0`; reference count at +8, `eState` at +0xC, the
//! 64-bit key at +0x10) to tell whether queued loads stall, and where.
//!
//! By default (`NV_QUEUE_CHECK=1`) only the summaries print (own budget): every 5 s the alive
//! tasks and what ended so far by type and final state (the last one stands for the run's end), and
//! once per unfinished task (`eState` below 4, `00449110`) an `old task` line when it is alive 3 s (state, type, file, the thread that last
//! changed its state). `NV_QUEUE_CHECK=verbose` adds the per-task lines:
//! - `task new` (constructor): the task, the thread making it, the frame;
//! - `task state` (each change of `eState` through `InterlockedCompareExchange`, the way the
//!   task code moves it): old and new state and the thread doing it;
//! - `task state seen` (a change the frame's sampling found that no exchange made: a plain
//!   store);
//! - `task gone` (destructor): its type, last state, age, whether it ever left its first state;
//!
//! The 5 s summary (at each `Present`, game clock) gives the alive tasks by type
//! and state, the 40 oldest with age and file, the ended counts and the game threads
//! (`thread_host::describe`: who is waiting on what, sleeping, parked).
//!
//! The type is the RTTI name behind the task's virtual table; the file is the `QueuedFileEntry`
//! name (`pFileName`, +0x28) when the type is a queued file. Lines start with `queue check:`
//! (stdout): at most 8000 per-task lines and 6000 summary lines.

use crate::Engine;
use std::collections::BTreeMap;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Mutex, OnceLock};

/// Whether `NV_QUEUE_CHECK` is set (read once).
pub fn enabled() -> bool {
    static ON: OnceLock<bool> = OnceLock::new();
    *ON.get_or_init(|| std::env::var_os("NV_QUEUE_CHECK").is_some())
}

/// Whether `NV_QUEUE_CHECK=verbose`: the per-task lines too.
fn verbose() -> bool {
    static ON: OnceLock<bool> = OnceLock::new();
    *ON.get_or_init(|| {
        std::env::var("NV_QUEUE_CHECK").is_ok_and(|v| v.eq_ignore_ascii_case("verbose"))
    })
}

/// The per-task lines (verbose only) and the summaries each have their own budget.
static TASK_LINES: AtomicU32 = AtomicU32::new(0);
static SUMMARY_LINES: AtomicU32 = AtomicU32::new(0);
const TASK_LIMIT: u32 = 8000;
const SUMMARY_LIMIT: u32 = 6000;

/// A per-task line (`NV_QUEUE_CHECK=verbose` only).
fn task_line(text: std::fmt::Arguments) {
    if verbose() && TASK_LINES.fetch_add(1, Ordering::Relaxed) < TASK_LIMIT {
        println!("queue check: {text}");
    }
}

/// A summary line (always).
fn line(text: std::fmt::Arguments) {
    if SUMMARY_LINES.fetch_add(1, Ordering::Relaxed) < SUMMARY_LIMIT {
        println!("queue check: {text}");
    }
}

/// `BSTask`'s fields the check reads.
const REF_COUNT: u32 = 0x8;
const STATE: u32 = 0xc;
const KEY: u32 = 0x10;
/// `QueuedFileEntry::pFileName` (Xbox PDB).
const FILE_NAME: u32 = 0x28;
/// How often the alive tasks are listed (game clock, microseconds), and how many are named.
const REPORT_EVERY_US: u64 = 5_000_000;
const REPORT_OLDEST: usize = 40;
/// A task alive this long (game clock) is reported once on its own.
const STALE_US: u64 = 3_000_000;
/// The first finished `eState` (`00449110`: finished when the state is 4 or more, signed).
const FINISHED: i32 = 4;

#[derive(Debug, Clone)]
struct Task {
    created_us: u64,
    created_frame: u64,
    created_thread: u32,
    first_state: i32,
    state: i32,
    changes: u32,
    /// The type and file once the derived constructor set its table (the base `BSTask` table
    /// `01017138` until then).
    kind: Option<String>,
    /// The thread that last changed its state (0: none yet) and whether it was reported as old.
    last_thread: u32,
    reported_old: bool,
}

/// `BSTask<__int64>`'s own virtual table (the base constructor's and destructor's).
const BS_TASK_VTABLE: u32 = 0x0101_7138;

#[derive(Default)]
struct State {
    tasks: BTreeMap<u32, Task>,
    frame: u64,
    last_report_us: u64,
    /// The tasks ended so far, by type and final state.
    ended: BTreeMap<(String, i32), u32>,
}

fn state() -> std::sync::MutexGuard<'static, State> {
    static STATE: OnceLock<Mutex<State>> = OnceLock::new();
    STATE
        .get_or_init(Default::default)
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

/// The C string at `at` when it is mapped and printable, at most `max` characters.
fn text_at(e: &Engine, at: u32, max: usize) -> Option<String> {
    if at == 0 || !e.mem.is_mapped(at) {
        return None;
    }
    let mut out = String::new();
    for i in 0..max as u32 {
        let a = at.wrapping_add(i);
        if !e.mem.is_mapped(a) {
            return None;
        }
        let c = e.mem.u8(a);
        if c == 0 {
            return Some(out);
        }
        if !(0x20..0x7f).contains(&c) {
            return None;
        }
        out.push(c as char);
    }
    Some(out)
}

/// The RTTI type name behind an object's virtual table (`.?AVName@@` shortened to `Name`).
fn type_name(e: &Engine, object: u32) -> String {
    let read = |a: u32| e.mem.is_mapped(a).then(|| e.mem.u32(a));
    let name = (|| {
        let vtable = read(object)?;
        let locator = read(vtable.checked_sub(4)?)?;
        let descriptor = read(locator.wrapping_add(0xc))?;
        text_at(e, descriptor.wrapping_add(8), 96)
    })();
    match name {
        Some(n) => short_type(&n),
        None => format!("vtable {:#x}", read(object).unwrap_or(0)),
    }
}

/// `.?AVQueuedTexture@@` -> `QueuedTexture`; anything else unchanged.
fn short_type(name: &str) -> String {
    let n = name
        .strip_prefix(".?AV")
        .or_else(|| name.strip_prefix(".?AU"))
        .unwrap_or(name);
    n.strip_suffix("@@").unwrap_or(n).to_string()
}

/// The task's file (queued files only).
fn file_of(e: &Engine, task: u32, kind: &str) -> String {
    if !kind.contains("Queued") {
        return String::new();
    }
    let name = task.wrapping_add(FILE_NAME);
    if !e.mem.is_mapped(name) {
        return String::new();
    }
    text_at(e, e.mem.u32(name), 160)
        .map(|f| format!(" '{f}'"))
        .unwrap_or_default()
}

/// The `BSTask` constructor (`00449090`) made `task`.
pub fn created(e: &mut Engine, task: u32) {
    let now = e.clock_now_us();
    let thread = crate::thread_host::current_id(e);
    let state0 = e.mem.i32(task.wrapping_add(STATE));
    let mut s = state();
    let frame = s.frame;
    s.tasks.insert(
        task,
        Task {
            created_us: now,
            created_frame: frame,
            created_thread: thread,
            first_state: state0,
            state: state0,
            changes: 0,
            kind: None,
            last_thread: 0,
            reported_old: false,
        },
    );
    drop(s);
    task_line(format_args!(
        "task new {task:#x} by thread {thread} at frame {frame}"
    ));
}

/// The `BSTask` destructor (`004490e0`) ends `task` (its vtable still the derived one's when
/// the derived destructors have not reset it; read before the base resets it).
pub fn destroyed(e: &mut Engine, task: u32) {
    let Some(t) = state().tasks.remove(&task) else {
        return;
    };
    let now = e.clock_now_us();
    let thread = crate::thread_host::current_id(e);
    let state_now = e.mem.i32(task.wrapping_add(STATE));
    let kind = t.kind.clone().unwrap_or_else(|| "BSTask".into());
    let type_only = kind.split(" '").next().unwrap_or_default().to_string();
    *state().ended.entry((type_only, state_now)).or_insert(0) += 1;
    task_line(format_args!(
        "task gone {task:#x} {kind}: state {} (first {}, {} changes), age {} ms, made by thread {} at frame {}, ended by thread {thread}",
        state_now,
        t.first_state,
        t.changes,
        now.saturating_sub(t.created_us) / 1000,
        t.created_thread,
        t.created_frame
    ));
}

/// `InterlockedCompareExchange(target, exchange, comparand)` answered `old` (before the
/// exchange took place): a change of a known task's `eState` is reported.
pub fn exchange(e: &mut Engine, target: u32, exchange: i32, comparand: i32, old: i32) {
    if old != comparand || old == exchange {
        return;
    }
    let task = target.wrapping_sub(STATE);
    let thread = crate::thread_host::current_id(e);
    let frame;
    {
        let mut s = state();
        frame = s.frame;
        let Some(t) = s.tasks.get_mut(&task) else {
            return;
        };
        t.state = exchange;
        t.changes += 1;
        t.last_thread = thread;
    }
    if !verbose() {
        return;
    }
    let kind = type_name(e, task);
    let file = file_of(e, task, &kind);
    task_line(format_args!(
        "task state {task:#x} {kind}{file}: {old} -> {exchange} by thread {thread} at frame {frame}"
    ));
}

/// A frame ends (the device's `Present`): samples every task's state (changes no exchange made), and
/// every 5 s lists what is still alive and what the threads do.
pub fn frame(e: &mut Engine) {
    let now = e.clock_now_us();
    let (frame, changed, stale, report) = {
        let mut s = state();
        s.frame += 1;
        let frame = s.frame;
        let mut changed = Vec::new();
        let mut stale = Vec::new();
        for (&task, t) in s.tasks.iter_mut() {
            // Finished tasks (`eState` 4 or more, the exe's test `00449110`) stay alive while
            // their owner holds them (the loaded-model and texture maps): not reported.
            if !t.reported_old && t.state < FINISHED && now.saturating_sub(t.created_us) >= STALE_US
            {
                t.reported_old = true;
                stale.push((task, t.clone()));
            }
            if t.kind.is_none() && e.mem.u32(task) != BS_TASK_VTABLE {
                let kind = type_name(e, task);
                let file = file_of(e, task, &kind);
                t.kind = Some(format!("{kind}{file}"));
            }
            let current = e.mem.i32(task.wrapping_add(STATE));
            if current != t.state {
                changed.push((task, t.state, current));
                t.state = current;
                t.changes += 1;
            }
        }
        let report = now.saturating_sub(s.last_report_us) >= REPORT_EVERY_US;
        if report {
            s.last_report_us = now;
        }
        (frame, changed, stale, report)
    };
    for (task, t) in stale {
        let kind = type_name(e, task);
        let file = file_of(e, task, &kind);
        let refs = e.mem.i32(task.wrapping_add(REF_COUNT));
        line(format_args!(
            "old task {task:#x} {kind}{file}: alive {} ms, state {} (first {}, {} changes), last changed by thread {}, refs {refs}, made by thread {} at frame {}",
            now.saturating_sub(t.created_us) / 1000,
            t.state,
            t.first_state,
            t.changes,
            t.last_thread,
            t.created_thread,
            t.created_frame
        ));
    }
    for (task, old, new) in changed {
        if !verbose() {
            break;
        }
        let kind = type_name(e, task);
        let file = file_of(e, task, &kind);
        task_line(format_args!(
            "task state seen {task:#x} {kind}{file}: {old} -> {new} (no exchange) by frame {frame}"
        ));
    }
    if report {
        report_alive(e, now, frame);
    }
}

fn report_alive(e: &mut Engine, now: u64, frame: u64) {
    let tasks: Vec<(u32, Task)> = state().tasks.iter().map(|(&k, v)| (k, v.clone())).collect();
    let mut by_kind: BTreeMap<(String, i32), u32> = BTreeMap::new();
    let mut rows = Vec::new();
    for (task, t) in &tasks {
        let kind = type_name(e, *task);
        *by_kind.entry((kind.clone(), t.state)).or_insert(0) += 1;
        rows.push((t.created_us, *task, kind, t.clone()));
    }
    rows.sort_by_key(|r| r.0);
    let summary: Vec<String> = by_kind
        .iter()
        .map(|((kind, st), n)| format!("{kind} state {st}: {n}"))
        .collect();
    line(format_args!(
        "alive at frame {frame}: {} tasks; {}",
        tasks.len(),
        summary.join(", ")
    ));
    for (created, task, kind, t) in rows.iter().take(REPORT_OLDEST) {
        let file = file_of(e, *task, kind);
        let refs = e.mem.i32(task.wrapping_add(REF_COUNT));
        let key = e.mem.u64(task.wrapping_add(KEY));
        line(format_args!(
            "  oldest {task:#x} {kind}{file}: state {} (first {}, {} changes), refs {refs}, key {key:#x}, age {} ms, made by thread {} at frame {}",
            t.state,
            t.first_state,
            t.changes,
            now.saturating_sub(*created) / 1000,
            t.created_thread,
            t.created_frame
        ));
    }
    let ended: Vec<String> = state()
        .ended
        .iter()
        .map(|((kind, st), n)| format!("{kind} state {st}: {n}"))
        .collect();
    line(format_args!("ended so far: {}", ended.join(", ")));
    let threads = crate::thread_host::describe(e);
    line(format_args!("threads: {threads}"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rtti_names_are_shortened() {
        assert_eq!(short_type(".?AVQueuedTexture@@"), "QueuedTexture");
        assert_eq!(short_type(".?AUThing@@"), "Thing");
        assert_eq!(short_type("other"), "other");
    }

    #[test]
    fn text_needs_mapped_printable_bytes() {
        let mut e = Engine::new();
        let at = e.mem.alloc(16);
        e.mem.set_cstr(at, b"meshes\\a.nif");
        assert_eq!(text_at(&e, at, 64).as_deref(), Some("meshes\\a.nif"));
        e.mem.set_u8(at, 1);
        assert_eq!(text_at(&e, at, 64), None);
        assert_eq!(text_at(&e, 0, 64), None);
    }

    #[test]
    fn type_name_reads_the_rtti_locator() {
        let mut e = Engine::new();
        let object = e.mem.alloc(4);
        let vtable = e.mem.alloc(8) + 4;
        let locator = e.mem.alloc(0x10);
        let descriptor = e.mem.alloc(0x20);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(vtable - 4, locator);
        e.mem.set_u32(locator + 0xc, descriptor);
        e.mem.set_cstr(descriptor + 8, b".?AVQueuedFileEntry@@");
        assert_eq!(type_name(&e, object), "QueuedFileEntry");
    }
}

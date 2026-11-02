//! The thread host: the game's threads, stepped cooperatively.
//!
//! The game makes its threads with `CreateThread` (every one through
//! `BSThread::Initialize`, `00aa6430`, whose start `00aa64d0` calls the
//! `BSThread`'s `ThreadProc`, vtable +4) and coordinates them with events and
//! semaphores. Here they run on the main thread's turn, one at a time, behind
//! the platform imports in `units/platform.rs` (`CreateThread`,
//! `ResumeThread`, `SuspendThread`, `ExitThread`, `WaitForSingleObject`,
//! `WaitForMultipleObjects`, `Sleep`, `SetEvent`, `ReleaseSemaphore`,
//! `GetCurrentThreadId`), which dispatch to the [`ThreadHost`] the engine uses
//! ([`host`]). Real OS threads can replace it behind the same trait.
//!
//! **One runs at a time.** Each game thread runs as a coroutine. Its stack is
//! an OS thread used strictly as a coroutine: it holds a baton (the engine,
//! handed over with it) only between being resumed and yielding, and the main
//! thread waits while it runs, so exactly one thread touches the engine at
//! any moment. This avoids stack switching by hand (no `unsafe` assembly, no
//! platform-specific context code); the cost is one OS thread per game thread
//! and a channel hand-off per switch.
//!
//! **When threads run (the switch points), in a fixed order:**
//! - The main thread waits on an object that is not signalled
//!   (`WaitForSingleObject`, `WaitForMultipleObjects`, any timeout): the host
//!   runs the other threads ([`run_until`]) until the object is signalled, or,
//!   for a timed wait, after one pass, once the clock passes its timeout. When
//!   a whole pass changes no object's state, main waits and every other thread
//!   sleeps or waits: the host waits until the earliest wake time (a sleeping
//!   thread's, or the wait's own deadline) and the passes go on. With
//!   no wake time ahead (or ten minutes of such waits without a change) the
//!   wait ends as one nothing can signal.
//! - The main thread calls `Sleep(n)`: the host waits `n` ms, then the others
//!   get one pass.
//! - A game thread waits on an object that is not signalled, or sleeps: it
//!   yields back to the main thread, which goes on with the next thread of the
//!   pass. A game thread's `Sleep(n)` does not move the clock: the thread is
//!   runnable again once the clock reaches its wake time (the clock when it
//!   slept, plus `n` ms).
//! - The host's own frame points call [`run_threads`].
//!
//! **Waiting** (`Engine::wait_clock_until_us`): with the real clock on (the
//! running game) the host really sleeps until the wake time, so the clock
//! (`Engine::clock_now_us`) never runs ahead of the real time. With it off
//! (the tests) the clock jumps to the wake time at once, so runs stay
//! deterministic.
//!
//! A pass resumes, in creation order, every thread that is runnable: started
//! for the first time (`CreateThread` without `CREATE_SUSPENDED`, or resumed
//! to a suspend count of 0), sleeping past its wake time, waiting on
//! objects that are now signalled, or (with [`Scheme::timed_waits`]) waiting
//! with a timeout whose deadline the clock has reached. `CreateThread` and
//! `ResumeThread` never switch by themselves: what they make runnable runs at
//! the next switch point. `SetEvent` and `ReleaseSemaphore` do the same under
//! [`Scheme::SLICE`]; under [`Scheme::WAKE`] see below. So runs are
//! deterministic on the platform's own clock (the tests'); with the real clock
//! on, when a sleeping thread wakes depends on the real time that passed.
//!
//! **Two schemes** ([`Scheme`], the host's `NV_THREADS`): the order in which the
//! cooperative host lets threads run decides which of the orders the game's own
//! synchronisation allows a run actually takes. Both keep every order the exe
//! enforces; they differ in the ones it leaves open.
//!
//! The exe evidence (FalloutNV.exe 1.4.0.525). With two AI linear task threads
//! (`AILinearTaskThreadManager::CreateThreads` `008c7290`: 12 + 8 manual-reset
//! events, threads `008c7da0` and `008c7f50`), the main loop body `0086e650`
//! kicks them at `0086ec87` (`008c78c0`: `ResetEvent` on all 20 events, then
//! `ReleaseSemaphore` on each thread's start semaphore, thread+0x30, through
//! `008c9fb0`). It then goes on with its own work, rendering and, in `008705d0`
//! -> `0086f670` -> `00455490`, every object and quest script. Only at
//! `0086ee4e` (`008c7990` -> `008c7490`) does it wait for each thread's done
//! semaphore (thread+0x3c, released by `BSLinearTaskThread::ThreadProc`
//! `008c7720` after each run). Nothing between the kick and that wait orders
//! main's scripts against the workers' steps: the workers order themselves
//! only against each other (`008c79e0` `SetEvent`, `008c7a70` waits), and main
//! waits on one of their events only in `00713c70` (event (0,1)). On Windows a
//! kicked worker starts at once on a free core and its few milliseconds of
//! work are long done before main reaches its scripts; the game relies on that
//! timing, not on a lock. Example: the speech keyframe lists of
//! `BSFaceGenAnimationData` (cleared under its lock at +0x1a0 by `0064a310`,
//! read under it by `0064b630`) are filled without the lock by `004d5930` from
//! `Speak` (`008a20d0`, `009344d0`), which both main's scripts and worker 1's
//! package step (`0096bcd0`) call.
//!
//! [`Scheme::WAKE`] (the default) has three parts, each of which the host's
//! `NV_THREADS` can change (`nowake`, `platformcall` or `anycall`, `notimeout`,
//! comma-separated; `slice` turns off all three: [`Scheme::SLICE`], the scheme
//! before):
//! - [`Scheme::wake`]: a `SetEvent` or `ReleaseSemaphore` on the main thread
//!   that lets a waiting thread go on runs it, and those it lets go on in turn,
//!   soon after ([`request_wake`], [`wake_waiters`]): once main has made
//!   [`Threads::wake_delay`] more calls (the host's `NV_WAKE_DELAY`, default
//!   [`WAKE_DELAY_CALLS`]), at its next platform call. Kicked workers so run
//!   early in main's frame, as on a free core. The delay is the wake-up latency
//!   of Windows: the signalling thread's next steps come first there, and the
//!   game relies on that too. `00b00df0` (a tasklet group done) releases the
//!   group's semaphore, then counts it (`InterlockedIncrement`), sets its byte
//!   +5 and calls its slot +0x24; a waiter run inside the `ReleaseSemaphore`
//!   (delay 0, the first version of this scheme, p178/p179) sees the group
//!   before those steps. A delay of 0 runs them inside the signalling call.
//! - [`Scheme::switch_at`] ([`SwitchAt`]): where a used-up time slice ends.
//!   [`SwitchAt::Blocking`] (the default): only at a blocking platform call
//!   ([`blocking_call`]: `WaitForSingleObject`, `WaitForMultipleObjects`, with
//!   any timeout, even an object already signalled; `Sleep`, also `Sleep(0)`,
//!   and a contended `EnterCriticalSection` switch by themselves), where
//!   Windows would most plausibly run another thread; never inside a stretch of
//!   the game's code between them, such as `004d5930`'s keyframe adds, which
//!   allocate (and so call interlocked operations) without the face data's
//!   lock (p183: `0064b630` reading a keyframe freed under it). A turn that
//!   reaches no blocking call within [`HARD_CAP_SLICES`] slices ends at its next
//!   call, so nothing starves. [`SwitchAt::Platform`] (`NV_THREADS=platformcall`,
//!   the version before): at the next call of any platform import
//!   (`Engine::platform_calls`). [`SwitchAt::Any`] (`anycall`): at whatever call
//!   comes next.
//!
//!   The game's own spin waits all reach a blocking call: `BSSpinLock`
//!   (`0040fbf0`) retries its `InterlockedCompareExchange` (`0043b460`) with
//!   `Sleep(0)` (`0040fca0` -> `[00fdf104]`), `Sleep(1)` after 10000 tries;
//!   `BSReadWriteLock` sleeps likewise; Havok's critical sections are
//!   `EnterCriticalSection`; task and tasklet waits are semaphore waits
//!   (`00b020a0` polls with a timeout of 0, still a wait). One exception found:
//!   the free-list lock of `006ecd60` (pathing messages) spins on `0043b460`
//!   alone. Its holder keeps the lock only over a few plain steps, so a spinner
//!   meets a held lock only when the holder's turn was ended there by the hard
//!   cap; it then spins out its own cap before the holder runs again (slow, not
//!   stuck).
//! - [`Scheme::timed_waits`]: a game thread's timed wait times out at its
//!   deadline (the clock jumps to it when nothing else can run, as for a
//!   sleep).
//!
//! Without them ([`Scheme::SLICE`]): signals switch nothing, a used-up slice
//! switches at whatever call comes next, and a game thread's timed wait goes
//! on only when its object is signalled (once resumed it then answers at
//! once).
//!
//! The switch itself is the same however a thread comes to run ([`switch_to`]
//! from the main thread: its TLS block, call stack, thread id and x87 control
//! word swapped in, the slice started again).
//!
//! **Per thread** ([`GameThread`]): its id, handle, start and parameter, its
//! suspend count, its run state, its TLS block (`Engine::tls`) and the call
//! stack of the functions it has entered (`Engine::call_stack`), swapped in
//! when it runs; the TLS slot values (`TlsGetValue`) are keyed by its id. Its
//! x87 control word is the word in its TLS block (`units::crt::x87_control_word`), loaded
//! into the FPU register the x87 helpers read whenever it runs (`load_fpu`).
//!
//! A game thread that stops (an untranslated function, a fault) is marked
//! stopped and the stop is raised on the main thread, naming the thread, so
//! the run that switched to it stops with it.

use crate::Engine;
use std::cell::RefCell;
use std::sync::mpsc::{channel, Receiver, Sender};

/// The main thread's id (`GetCurrentThreadId` on the main thread).
pub const MAIN_THREAD_ID: u32 = 1;
/// The stack an OS thread gets as a game thread's coroutine (the game's code
/// nests deeply in debug builds).
const COROUTINE_STACK: usize = 64 << 20;

/// What a game thread is doing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RunState {
    /// Made, not run yet.
    NotStarted,
    /// Running, or paused at a yield it will go on from when resumed.
    Running,
    /// Waiting on these handles (all of them, or any), until the clock time `deadline` for a
    /// timed wait (`None`: `INFINITE`).
    Waiting {
        handles: Vec<u32>,
        all: bool,
        deadline: Option<u64>,
    },
    /// Gave up its turn (`Sleep`): runnable at the first pass once the clock
    /// ([`Engine::clock_now_us`]) reaches `until`.
    Sleeping { until: u64 },
    /// Its start function returned or it called `ExitThread`.
    Finished { code: u32 },
    /// It stopped (an untranslated function, a fault): the first line.
    Stopped(String),
}

/// One game thread, as the platform seam keeps it. The main thread is the
/// first, made when first needed.
#[derive(Debug)]
pub struct GameThread {
    pub id: u32,
    /// Its handle (0 for the main thread).
    pub handle: u32,
    pub start: u32,
    pub parameter: u32,
    pub suspend_count: u32,
    pub state: RunState,
    /// Its TLS block (0 until it first runs) and its entered functions,
    /// kept here while another thread runs.
    tls: u32,
    call_stack: Vec<u32>,
    /// The handle list of its last wait, kept for the next one (no allocation per wait).
    spare_handles: Vec<u32>,
    /// The coroutine (made when it first runs).
    link: Option<Link>,
    /// How many times it was switched to (a diagnostic, [`describe`]).
    pub turns: u64,
}

/// The engine, handed to the coroutine that runs next.
struct Baton(*mut Engine);
// SAFETY: the engine is used by exactly one OS thread at a time: the main
// thread blocks on the coroutine's reply from handing the baton over until
// the coroutine yields it back (module documentation). The channels order
// every access of one thread before the next thread's.
unsafe impl Send for Baton {}

/// What a coroutine hands back.
#[derive(Debug)]
enum Yield {
    Paused,
    Finished(u32),
    Stopped(String),
}

/// The main thread's end of a coroutine.
struct Link {
    resume: Sender<Baton>,
    yielded: Receiver<Yield>,
}

impl std::fmt::Debug for Link {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Link")
    }
}

/// The game threads (`Kernel::threads`).
#[derive(Debug)]
pub struct Threads {
    /// Every thread, in creation order; the main thread first.
    pub list: Vec<GameThread>,
    /// The running thread's index in [`Threads::list`].
    pub current: usize,
    /// Counts every change of an object's state that can let a waiter go on
    /// (an event set, a semaphore released, a wait satisfied, a thread
    /// ended): a pass that leaves it unchanged made no progress.
    pub signals: u64,
    /// Counts the turns a thread gave back at the end of its time slice ([`preempt`]): it was
    /// running, not stuck, so a pass with one made progress even when no object changed.
    pub preempted: u64,
    /// Which orders the host lets threads run in (module documentation).
    pub scheme: Scheme,
    /// [`Scheme::wake`]: how many calls the main thread makes after a signal that woke a
    /// thread before that thread runs (at main's next platform call); 0: inside the signalling
    /// call itself. The host's `NV_WAKE_DELAY`.
    pub wake_delay: u64,
    /// A signal from the main thread woke a thread that has not run yet ([`request_wake`]).
    pub wake_pending: bool,
    /// [`SwitchAt::Blocking`]: the running turn used up its slice and goes on until its next
    /// blocking call ([`blocking_call`]) or the hard cap ([`preempt`]).
    pub overrun: bool,
}

impl Default for Threads {
    fn default() -> Self {
        Threads {
            list: Vec::new(),
            current: 0,
            signals: 0,
            preempted: 0,
            scheme: Scheme::default(),
            wake_delay: WAKE_DELAY_CALLS,
            wake_pending: false,
            overrun: false,
        }
    }
}

/// [`SwitchAt::Blocking`]'s fallback: a turn that has used up this many slices without a
/// blocking call ends at its next call, so a thread that never blocks does not starve the
/// others (a host setting, not the game's).
pub const HARD_CAP_SLICES: u64 = 50;

/// Where a turn that used up its time slice ends ([`Scheme::switch_at`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SwitchAt {
    /// At its next blocking platform call ([`blocking_call`]), or at the hard cap.
    Blocking,
    /// At its next call of a platform import (`Engine::platform_calls`).
    Platform,
    /// At its next call.
    Any,
}

/// [`Threads::wake_delay`]'s default: a few microseconds of the game's code, about the time
/// Windows takes to start a woken thread on another core (a host setting, not the game's).
pub const WAKE_DELAY_CALLS: u64 = 1000;

/// How the cooperative host orders the game's threads (module documentation): which parts of
/// [`Scheme::WAKE`] are on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Scheme {
    /// A signal from the main thread runs the threads it woke soon after ([`request_wake`]).
    pub wake: bool,
    /// Where a used-up slice ends.
    pub switch_at: SwitchAt,
    /// A game thread's timed wait times out at its deadline.
    pub timed_waits: bool,
}

impl Default for Scheme {
    fn default() -> Self {
        Scheme::WAKE
    }
}

impl Scheme {
    /// The default: every part on.
    pub const WAKE: Scheme = Scheme {
        wake: true,
        switch_at: SwitchAt::Blocking,
        timed_waits: true,
    };
    /// The scheme before: every part off.
    pub const SLICE: Scheme = Scheme {
        wake: false,
        switch_at: SwitchAt::Any,
        timed_waits: false,
    };

    /// The scheme the host's `NV_THREADS` names: unset or empty [`Scheme::WAKE`]; `slice`
    /// [`Scheme::SLICE`]; otherwise [`Scheme::WAKE`] changed by the words of a
    /// comma-separated list (`nowake`; `platformcall` or `anycall` for [`Scheme::switch_at`];
    /// `notimeout`; `wake` changes nothing). Case and spaces are ignored; an unknown word is
    /// an error.
    pub fn from_name(name: Option<&str>) -> Result<Scheme, String> {
        let mut scheme = Scheme::WAKE;
        for word in name.unwrap_or("").split(',').map(str::trim) {
            match word.to_ascii_lowercase().as_str() {
                "" | "wake" => {}
                "slice" => scheme = Scheme::SLICE,
                "nowake" => scheme.wake = false,
                "platformcall" => scheme.switch_at = SwitchAt::Platform,
                "anycall" => scheme.switch_at = SwitchAt::Any,
                "notimeout" => scheme.timed_waits = false,
                other => {
                    let known = "slice, wake, nowake, platformcall, anycall, notimeout";
                    return Err(format!("NV_THREADS: unknown part {other:?} ({known})"));
                }
            }
        }
        Ok(scheme)
    }
}

thread_local! {
    /// A coroutine's end of its link, on its own OS thread.
    static COROUTINE: RefCell<Option<(Receiver<Baton>, Sender<Yield>)>> = const { RefCell::new(None) };
}

/// Whether an object can let a waiter go on, without taking it.
pub type Signalled = fn(&Engine, u32) -> bool;

/// The seam: how game threads run. The cooperative host is the one in use.
pub trait ThreadHost {
    /// The current thread would wait on `handles` (all or any); `deadline` is
    /// the clock time ([`Engine::clock_now_us`]) a timed wait ends at, `None`
    /// for `INFINITE`. Returns whether they may now be signalled (the caller
    /// takes them and tries again), false when nothing can signal them or the
    /// wait timed out.
    fn block(
        &self,
        e: &mut Engine,
        handles: &[u32],
        all: bool,
        deadline: Option<u64>,
        signalled: Signalled,
    ) -> bool;
    /// The current thread gives up its turn for `ms` milliseconds (`Sleep`).
    fn sleep(&self, e: &mut Engine, ms: u32, signalled: Signalled);
    /// The current game thread ends (`ExitThread`).
    fn exit(&self, e: &mut Engine, code: u32);
}

/// The cooperative host (module documentation).
pub struct Cooperative;

/// The host the platform imports use.
pub fn host() -> &'static dyn ThreadHost {
    &Cooperative
}

fn ensure_main(e: &mut Engine) {
    if e.kernel.threads.list.is_empty() {
        e.kernel.threads.list.push(GameThread {
            id: MAIN_THREAD_ID,
            handle: 0,
            start: 0,
            parameter: 0,
            suspend_count: 0,
            state: RunState::Running,
            tls: 0,
            call_stack: Vec::new(),
            spare_handles: Vec::new(),
            link: None,
            turns: 0,
        });
    }
}

/// The game threads as one line (a diagnostic): each thread's id, start, run state (a sleeper's
/// wake time from now, in ms), suspend count when not 0, and how many turns it has had.
pub fn describe(e: &mut Engine) -> String {
    let now = e.clock_now_us();
    e.kernel
        .threads
        .list
        .iter()
        .map(|t| {
            let state = match &t.state {
                RunState::Sleeping { until } => {
                    format!("sleeping {:+} ms", (*until as i64 - now as i64) / 1000)
                }
                RunState::Waiting {
                    handles,
                    all,
                    deadline,
                } => format!(
                    "waiting {} {:x?}{}",
                    if *all { "all" } else { "any" },
                    handles,
                    deadline.map_or(String::new(), |d| format!(
                        " until {:+} ms",
                        (d as i64 - now as i64) / 1000
                    ))
                ),
                other => format!("{other:?}"),
            };
            let suspended = if t.suspend_count != 0 {
                format!(" suspended {}", t.suspend_count)
            } else {
                String::new()
            };
            format!(
                "{}@{:08x} {state}{suspended} turns {}",
                t.id, t.start, t.turns
            )
        })
        .collect::<Vec<_>>()
        .join("; ")
}

/// The running thread's id.
pub fn current_id(e: &Engine) -> u32 {
    e.kernel
        .threads
        .list
        .get(e.kernel.threads.current)
        .map_or(MAIN_THREAD_ID, |t| t.id)
}

/// Whether the main thread is running.
pub fn on_main(e: &Engine) -> bool {
    e.kernel.threads.current == 0
}

/// Records a new thread (it runs at the next switch point once its suspend
/// count is 0). Returns its id.
pub fn create(e: &mut Engine, handle: u32, start: u32, parameter: u32, suspended: bool) -> u32 {
    ensure_main(e);
    let id = MAIN_THREAD_ID + e.kernel.threads.list.len() as u32;
    e.kernel.threads.list.push(GameThread {
        id,
        handle,
        start,
        parameter,
        suspend_count: u32::from(suspended),
        state: RunState::NotStarted,
        tls: 0,
        call_stack: Vec::new(),
        spare_handles: Vec::new(),
        link: None,
        turns: 0,
    });
    id
}

/// The thread behind a handle.
pub fn by_handle(e: &mut Engine, handle: u32) -> Option<&mut GameThread> {
    e.kernel
        .threads
        .list
        .iter_mut()
        .find(|t| t.handle == handle && t.handle != 0)
}

/// Whether a thread handle is signalled (its thread ended).
pub fn thread_ended(e: &Engine, handle: u32) -> Option<bool> {
    e.kernel
        .threads
        .list
        .iter()
        .find(|t| t.handle == handle && t.handle != 0)
        .map(|t| matches!(t.state, RunState::Finished { .. } | RunState::Stopped(_)))
}

/// Notes a change that can let a waiter go on.
pub fn signal(e: &mut Engine) {
    e.kernel.threads.signals = e.kernel.threads.signals.wrapping_add(1);
}

fn runnable(e: &mut Engine, i: usize, signalled: Signalled) -> bool {
    if let RunState::Sleeping { until } = e.kernel.threads.list[i].state {
        if e.kernel.threads.list[i].suspend_count != 0 {
            return false;
        }
        return e.clock_now_us() >= until;
    }
    let t = &e.kernel.threads.list[i];
    if t.suspend_count != 0 {
        return false;
    }
    match &t.state {
        RunState::NotStarted => true,
        RunState::Waiting { deadline, .. } => {
            if waiting_signalled(e, i, signalled) {
                return true;
            }
            let deadline = *deadline;
            e.kernel.threads.scheme.timed_waits && deadline.is_some_and(|d| e.clock_now_us() >= d)
        }
        _ => false,
    }
}

/// Whether thread `i` waits (not suspended) on objects that are now signalled.
fn waiting_signalled(e: &Engine, i: usize, signalled: Signalled) -> bool {
    let t = &e.kernel.threads.list[i];
    if t.suspend_count != 0 {
        return false;
    }
    match &t.state {
        RunState::Waiting { handles, all, .. } => {
            if *all {
                handles.iter().all(|&h| signalled(e, h))
            } else {
                handles.iter().any(|&h| signalled(e, h))
            }
        }
        _ => false,
    }
}

/// How many rounds [`wake_waiters`] runs at most: a safeguard of the host (not the game's
/// behaviour) against threads that wake each other for ever, so main still goes on.
const WAKE_ROUNDS: usize = 64;

/// [`Scheme::wake`]: the main thread signalled an object (`SetEvent`, `ReleaseSemaphore`).
/// When that lets a waiting thread go on, it runs ([`wake_waiters`]) once main has made
/// [`Threads::wake_delay`] more calls, at its next platform call ([`preempt`]); with a delay
/// of 0 at once, inside the signalling call. Nothing on a game thread (it cannot switch to
/// another thread itself; what it wakes runs at the next switch point) or without
/// [`Scheme::wake`].
pub fn request_wake(e: &mut Engine, signalled: Signalled) {
    let threads = &e.kernel.threads;
    if !threads.scheme.wake || threads.list.is_empty() || !on_main(e) {
        return;
    }
    if !(1..e.kernel.threads.list.len()).any(|i| waiting_signalled(e, i, signalled)) {
        return;
    }
    if e.kernel.threads.wake_delay == 0 {
        wake_waiters(e, signalled);
        return;
    }
    mark_wake(e);
}

/// Sets a wake pending ([`Threads::wake_pending`]): the main thread runs the others once it
/// has made [`Threads::wake_delay`] more calls, at its next platform call ([`preempt`]).
fn mark_wake(e: &mut Engine) {
    e.kernel.threads.wake_pending = true;
    if e.slice_hook.is_none() {
        // No time slice (`NV_SLICE=0`, the tests): the hook only for the wake.
        e.slice_hook = Some(preempt);
        e.slice_left = u64::MAX;
    }
    e.slice_left = e.slice_left.min(e.kernel.threads.wake_delay);
}

/// Whether a thread other than main could run now ([`runnable`]): started, waiting on objects
/// now signalled or past its deadline, or asleep past its wake time (a `Sleep(0)`, a turn
/// ended at its slice).
fn others_runnable(e: &mut Engine) -> bool {
    for i in 1..e.kernel.threads.list.len() {
        if runnable(e, i, crate::units::platform::signalled) {
            return true;
        }
    }
    false
}

/// [`Scheme::wake`]: the main thread has the engine back (or used up its slice) while another
/// thread could run. On Windows that thread would go on at once on its own core; here it gets a
/// turn soon ([`mark_wake`]), not only at main's next blocking call, which may be a whole phase
/// of main's frame later: a worker parked in a spin lock's `Sleep(0)` (`0040fca0`) or at its
/// hard cap must not sit through main's scripts (`00455490`) in the middle of its own work
/// (p187).
fn wake_parked(e: &mut Engine) {
    if e.kernel.threads.scheme.wake
        && !e.kernel.threads.wake_pending
        && on_main(e)
        && others_runnable(e)
    {
        mark_wake(e);
    }
}

/// Runs the threads that can go on, from the main thread ([`request_wake`], [`wake_parked`]):
/// first every runnable thread once ([`runnable`]: woken, started, past a sleep or a deadline),
/// in creation order; then, round after round, every thread waiting on objects that are now
/// signalled, until a round finds none (those the first ones woke, in turn). A thread that
/// sleeps again or uses up its slice waits for the next switch point. Main thread only.
pub fn wake_waiters(e: &mut Engine, signalled: Signalled) {
    e.kernel.threads.wake_pending = false;
    if e.kernel.threads.list.is_empty() || !on_main(e) {
        return;
    }
    for round in 0..WAKE_ROUNDS {
        let mut ran = false;
        let mut i = 1;
        while i < e.kernel.threads.list.len() {
            let go = if round == 0 {
                runnable(e, i, signalled)
            } else {
                waiting_signalled(e, i, signalled)
            };
            if go {
                switch_to(e, i);
                ran = true;
            }
            i += 1;
        }
        if !ran {
            return;
        }
    }
}

/// Runs the other threads from the main thread, pass after pass in creation
/// order, until `until` holds or a pass changes no object's state. Returns
/// `until`'s last answer.
pub fn run_until(
    e: &mut Engine,
    signalled: Signalled,
    until: &mut dyn FnMut(&mut Engine) -> bool,
) -> bool {
    ensure_main(e);
    assert!(
        on_main(e),
        "the thread host runs threads from the main thread only"
    );
    loop {
        if until(e) {
            return true;
        }
        let before = e.kernel.threads.signals;
        let before_preempted = e.kernel.threads.preempted;
        let mut ran = false;
        let mut i = 1;
        while i < e.kernel.threads.list.len() {
            if runnable(e, i, signalled) {
                switch_to(e, i);
                ran = true;
                if until(e) {
                    return true;
                }
            }
            i += 1;
        }
        if !ran
            || (e.kernel.threads.signals == before
                && e.kernel.threads.preempted == before_preempted)
        {
            return until(e);
        }
    }
}

/// How far (microseconds of clock) the main thread's wait jumps the clock to sleepers' wake
/// times without any object changing state before it gives up (the wait then times out or
/// stops as one nothing can signal): ten minutes of game time.
const JUMP_LIMIT_US: u64 = 600_000_000;

/// The earliest time a thread wakes: a sleeping thread that is not suspended, under
/// [`Scheme::timed_waits`] the deadline of a timed wait, or `deadline`.
fn earliest_wake(e: &Engine, deadline: Option<u64>) -> Option<u64> {
    let timed = e.kernel.threads.scheme.timed_waits;
    e.kernel
        .threads
        .list
        .iter()
        .filter(|t| t.suspend_count == 0)
        .filter_map(|t| match t.state {
            RunState::Sleeping { until } => Some(until),
            RunState::Waiting {
                deadline: Some(d), ..
            } if timed => Some(d),
            _ => None,
        })
        .chain(deadline)
        .min()
}

/// One pass from the main thread: each runnable thread, in creation order, runs until it yields.
fn one_pass(e: &mut Engine, signalled: Signalled) {
    let mut i = 1;
    while i < e.kernel.threads.list.len() {
        if runnable(e, i, signalled) {
            switch_to(e, i);
        }
        i += 1;
    }
}

/// Runs the game threads until none can go on (the host's frame points).
pub fn run_threads(e: &mut Engine, signalled: Signalled) {
    run_until(e, signalled, &mut |_| false);
}

/// Swaps the running thread's TLS block and call stack for thread `to`'s.
fn swap_context(e: &mut Engine, from: usize, to: usize) {
    // A thread switched to starts a whole time slice.
    e.slice_left = turn_len(e);
    e.kernel.threads.overrun = false;
    let mut to_tls = e.kernel.threads.list[to].tls;
    if to_tls == 0 {
        to_tls = e.new_tls_block();
        e.kernel.threads.list[to].tls = to_tls;
    }
    let from_tls = e.swap_tls(to_tls);
    let to_stack = std::mem::take(&mut e.kernel.threads.list[to].call_stack);
    let from_stack = std::mem::replace(&mut e.call_stack, to_stack);
    let from_thread = &mut e.kernel.threads.list[from];
    from_thread.tls = from_tls;
    from_thread.call_stack = from_stack;
}

/// The address [`Engine::trace`] records a thread switch under (no exe address): the words are
/// the thread that gave up the engine and the one that takes it, `EAX` the latter, so a stopped
/// run's trace (`NV_MOVE_TRACE`) reads `fffffff0(00000001,00000011) -> 00000011` where main
/// handed over to thread 17.
pub const TRACE_SWITCH: u32 = 0xffff_fff0;

/// Records a switch from thread `from` to thread `to` in the trace, when one is kept.
fn trace_switch(e: &mut Engine, from: u32, to: u32) {
    let keep = e.trace_keep;
    if let Some(trace) = &mut e.trace {
        if keep == 0 {
            return;
        }
        if trace.len() >= keep {
            trace.pop_front();
        }
        trace.push_back((TRACE_SWITCH, vec![from, to], to));
    }
}

/// Hands the engine to thread `i` until it yields (main thread only).
fn switch_to(e: &mut Engine, i: usize) {
    let link = match e.kernel.threads.list[i].link.take() {
        Some(link) => link,
        None => {
            let t = &e.kernel.threads.list[i];
            spawn(t.id, t.start, t.parameter)
        }
    };
    swap_context(e, 0, i);
    let id = e.kernel.threads.list[i].id;
    trace_switch(e, MAIN_THREAD_ID, id);
    e.kernel.threads.current = i;
    e.kernel.threads.list[i].turns += 1;
    // A wait's handle list is kept for the thread's next wait.
    let t = &mut e.kernel.threads.list[i];
    if let RunState::Waiting { handles, .. } = std::mem::replace(&mut t.state, RunState::Running) {
        t.spare_handles = handles;
    }
    let baton = Baton(e as *mut Engine);
    // `NV_PROFILE`: the hand-off, with the thread's own call stack.
    crate::profile::resync(&e.call_stack);
    crate::profile::set_phase(crate::profile::Phase::Switch);
    let reply = match link.resume.send(baton) {
        Ok(()) => link.yielded.recv().ok(),
        Err(_) => None,
    };
    // Back on the main thread.
    e.kernel.threads.current = 0;
    trace_switch(e, id, MAIN_THREAD_ID);
    swap_context_back(e, i);
    crate::profile::resync(&e.call_stack);
    crate::profile::holding(MAIN_THREAD_ID);
    load_fpu(e);
    e.kernel.threads.list[i].link = Some(link);
    match reply {
        Some(Yield::Paused) => {}
        Some(Yield::Finished(code)) => {
            e.kernel.threads.list[i].state = RunState::Finished { code };
            signal(e);
        }
        Some(Yield::Stopped(why)) => {
            let t = &mut e.kernel.threads.list[i];
            t.state = RunState::Stopped(why.clone());
            let (id, start) = (t.id, t.start);
            signal(e);
            panic!("{why} (in game thread {id}, start {start:08x})");
        }
        None => {
            let t = &mut e.kernel.threads.list[i];
            t.state = RunState::Stopped("its coroutine ended".into());
            signal(e);
        }
    }
}

/// Loads the FPU control register the x87 helpers round with (`crate::x87::set_control_word`,
/// a per-OS-thread value) from the running game thread's own word in its TLS block
/// (`units::crt::x87_control_word`), on the OS thread that runs it next: each game thread
/// keeps its own x87 precision (the AI linear task threads `0x027F`, the main thread `0x007F`
/// once the renderer's device exists, `units::platform::direct3d_device_created`).
fn load_fpu(e: &mut Engine) {
    let word = crate::units::crt::x87_control_word(e);
    crate::x87::set_control_word(word);
}

/// [`swap_context`] from thread `from` back to the main thread.
fn swap_context_back(e: &mut Engine, from: usize) {
    start_main_turn(e);
    let main_tls = e.kernel.threads.list[0].tls;
    let from_tls = e.swap_tls(main_tls);
    let main_stack = std::mem::take(&mut e.kernel.threads.list[0].call_stack);
    let from_stack = std::mem::replace(&mut e.call_stack, main_stack);
    let t = &mut e.kernel.threads.list[from];
    t.tls = from_tls;
    t.call_stack = from_stack;
}

/// Makes thread `id`'s coroutine: an OS thread that waits for the baton,
/// runs `start(parameter)` on the engine and hands back how it ended.
fn spawn(id: u32, start: u32, parameter: u32) -> Link {
    let (resume, resumed) = channel::<Baton>();
    let (yields, yielded) = channel::<Yield>();
    std::thread::Builder::new()
        .name(format!("game thread {id}"))
        .stack_size(COROUTINE_STACK)
        .spawn(move || {
            let Ok(Baton(engine)) = resumed.recv() else {
                return;
            };
            COROUTINE.with(|c| *c.borrow_mut() = Some((resumed, yields.clone())));
            // SAFETY: see `Baton`.
            let e = unsafe { &mut *engine };
            crate::profile::holding(id);
            load_fpu(e);
            let out = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                e.call(start, &[parameter]).eax
            }));
            let reply = match out {
                Ok(code) => Yield::Finished(code),
                Err(payload) => {
                    // The functions the thread was in when it stopped, innermost first, as
                    // for the main thread below.
                    let path: Vec<String> = e
                        .call_stack
                        .iter()
                        .rev()
                        .take(6)
                        .map(|a| format!("{a:08x}"))
                        .collect();
                    let first = panic_line(payload);
                    if path.is_empty() {
                        Yield::Stopped(first)
                    } else {
                        Yield::Stopped(format!("{first} (in {})", path.join(" < ")))
                    }
                }
            };
            let _ = yields.send(reply);
        })
        .expect("a game thread's coroutine");
    Link { resume, yielded }
}

fn panic_line(payload: Box<dyn std::any::Any + Send>) -> String {
    let text = payload
        .downcast_ref::<String>()
        .cloned()
        .or_else(|| payload.downcast_ref::<&str>().map(|s| s.to_string()))
        .unwrap_or_default();
    text.lines().next().unwrap_or("").to_string()
}

/// On a coroutine: hands the engine back to the main thread and waits to be
/// resumed. When the engine is gone (the main thread dropped it), the
/// coroutine parks for good: its stack still holds frames of the dropped
/// engine's calls, so it must never run them.
fn yield_to_main(reply: Yield) {
    let me = crate::profile::on().then(crate::profile::game_thread);
    crate::profile::set_phase(crate::profile::Phase::Switch);
    COROUTINE.with(|c| {
        let c = c.borrow();
        let (resumed, yields) = c
            .as_ref()
            .expect("a coroutine yields only from its own thread");
        if yields.send(reply).is_err() || resumed.recv().is_err() {
            loop {
                std::thread::park();
            }
        }
    });
    if let Some(id) = me {
        crate::profile::holding(id);
    }
}

impl ThreadHost for Cooperative {
    fn block(
        &self,
        e: &mut Engine,
        handles: &[u32],
        all: bool,
        deadline: Option<u64>,
        signalled: Signalled,
    ) -> bool {
        let ready = |e: &Engine| {
            if all {
                handles.iter().all(|&h| signalled(e, h))
            } else {
                handles.iter().any(|&h| signalled(e, h))
            }
        };
        ensure_main(e);
        if on_main(e) {
            // One pass gives the others a turn; then they run on until the
            // objects are signalled or the clock passes the deadline. A timed
            // wait (the timeout 0 poll in 00b020a0) so ends even while a
            // thread sleeps in a loop (the loading menu's, Sleep(50) a round).
            one_pass(e, signalled);
            let past = |e: &mut Engine| deadline.is_some_and(|d| e.clock_now_us() >= d);
            let mut jumped = 0u64;
            loop {
                let preempted = e.kernel.threads.preempted;
                let signals = e.kernel.threads.signals;
                run_until(e, signalled, &mut |e| ready(e) || past(e));
                if ready(e) || past(e) {
                    return ready(e);
                }
                // A thread that used up its time slice was working: give it more turns.
                if e.kernel.threads.preempted != preempted {
                    continue;
                }
                if e.kernel.threads.signals != signals {
                    jumped = 0;
                }
                // No thread can go on now: main waits and every other thread sleeps or
                // waits. The clock jumps to the earliest wake time (a sleeper's, or this
                // wait's deadline). A thread due now that made no progress, or
                // `JUMP_LIMIT` of jumps without a signal, ends the wait as before.
                let now = e.clock_now_us();
                let Some(wake) = earliest_wake(e, deadline) else {
                    return false;
                };
                if wake <= now || jumped >= JUMP_LIMIT_US {
                    return false;
                }
                jumped += wake - now;
                e.wait_clock_until_us(wake);
            }
        }
        let i = e.kernel.threads.current;
        loop {
            let t = &mut e.kernel.threads.list[i];
            // The last wait's list, given back when the thread was resumed ([`switch_to`]).
            let mut list = std::mem::take(&mut t.spare_handles);
            list.clear();
            list.extend_from_slice(handles);
            t.state = RunState::Waiting {
                handles: list,
                all,
                deadline,
            };
            yield_to_main(Yield::Paused);
            load_fpu(e);
            if ready(e) {
                return true;
            }
            let Some(d) = deadline else { continue };
            // A timed wait resumed without its objects: under `Wake` it was resumed at its
            // deadline (or before it, the objects taken by another thread first: it waits on);
            // under `Slice` it times out at once.
            if !e.kernel.threads.scheme.timed_waits || e.clock_now_us() >= d {
                return false;
            }
        }
    }

    fn sleep(&self, e: &mut Engine, ms: u32, signalled: Signalled) {
        ensure_main(e);
        let until = e.clock_now_us() + u64::from(ms) * 1000;
        if on_main(e) {
            // The main thread's own sleep waits until its wake time (the host really
            // sleeps with the real clock on; the tests' clock moves on), then gives the
            // others a pass.
            e.wait_clock_until_us(until);
            one_pass(e, signalled);
            // A switch point: main starts a new turn.
            start_main_turn(e);
            return;
        }
        // Another thread's sleep leaves the clock alone: it runs again at the first pass
        // once the clock reaches its wake time.
        let i = e.kernel.threads.current;
        e.kernel.threads.list[i].state = RunState::Sleeping { until };
        yield_to_main(Yield::Paused);
        load_fpu(e);
    }

    fn exit(&self, e: &mut Engine, code: u32) {
        ensure_main(e);
        assert!(
            !on_main(e),
            "ExitThread on the main thread ends the process (not provided)"
        );
        yield_to_main(Yield::Finished(code));
        // A finished thread is never resumed.
        loop {
            std::thread::park();
        }
    }
}

/// What the game's main thread did in a turn ([`run_main_frame`]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MainTurn {
    /// It reached the end of a frame (its message loop found the queue empty,
    /// [`end_main_frame`]) and waits for the next.
    Frame,
    /// Its start function (`main`) returned this.
    Finished(u32),
    /// It stopped (an untranslated function, a fault): the first line, with the innermost
    /// functions it was in.
    Stopped(String),
}

/// The game's main thread run as a coroutine the host drives frame by frame: `main` (the
/// game's `WinMain`) runs on its own OS thread, as the game threads do (module
/// documentation), and is the thread host's main thread while it runs (it switches to the
/// other game threads itself). Its message loop is the frame driver: when it finds no message
/// (`PeekMessageA`, [`end_main_frame`]) it hands the engine back to the host, which resumes it
/// for the next frame. Between frames its entered functions are kept here.
pub struct MainThread {
    link: Link,
    call_stack: Vec<u32>,
    /// How its last turn ended (`None` before the first).
    pub last: Option<MainTurn>,
    /// The host's own call stack while a frame of main's runs ([`begin_main_frame`]).
    host_stack: Vec<u32>,
    /// A frame was handed to main and not finished yet.
    in_flight: bool,
}

impl MainThread {
    /// Whether a frame was handed to main ([`begin_main_frame`]) and not finished yet
    /// ([`finish_main_frame`]): until then the engine is main's.
    pub fn in_flight(&self) -> bool {
        self.in_flight
    }
}

impl std::fmt::Debug for MainThread {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MainThread")
            .field("last", &self.last)
            .finish()
    }
}

thread_local! {
    /// The main thread's coroutine end of its link, on its own OS thread.
    static MAIN_COROUTINE: RefCell<Option<(Receiver<Baton>, Sender<Yield>)>> = const { RefCell::new(None) };
}

/// Makes the game's main thread: `start(words)` (`main`'s address and its four words) on its
/// own OS thread, run from the first [`run_main_frame`].
pub fn start_main_thread(start: u32, words: Vec<u32>) -> MainThread {
    let (resume, resumed) = channel::<Baton>();
    let (yields, yielded) = channel::<Yield>();
    std::thread::Builder::new()
        .name("game main thread".into())
        .stack_size(COROUTINE_STACK)
        .spawn(move || {
            let Ok(Baton(engine)) = resumed.recv() else {
                return;
            };
            MAIN_COROUTINE.with(|c| *c.borrow_mut() = Some((resumed, yields.clone())));
            // SAFETY: see `Baton`.
            let e = unsafe { &mut *engine };
            crate::profile::holding(MAIN_THREAD_ID);
            load_fpu(e);
            let out = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                e.call(start, &words).eax
            }));
            let reply = match out {
                Ok(code) => Yield::Finished(code),
                Err(payload) => {
                    // The functions it was in when it stopped (`Engine::call` pops only on
                    // return), innermost first.
                    let path: Vec<String> = e
                        .call_stack
                        .iter()
                        .rev()
                        .take(6)
                        .map(|a| format!("{a:08x}"))
                        .collect();
                    let first = panic_line(payload);
                    if path.is_empty() {
                        Yield::Stopped(first)
                    } else {
                        Yield::Stopped(format!("{first} (in {})", path.join(" < ")))
                    }
                }
            };
            let _ = yields.send(reply);
        })
        .expect("the game main thread's coroutine");
    MainThread {
        link: Link { resume, yielded },
        call_stack: Vec::new(),
        last: None,
        host_stack: Vec::new(),
        in_flight: false,
    }
}

/// Runs the game's main thread for one turn: until the end of its next frame, its return or
/// its stop. Called from the host's thread, never from a game thread. A thread that finished or
/// stopped is not resumed (its last turn is answered again).
pub fn run_main_frame(e: &mut Engine, main: &mut MainThread) -> MainTurn {
    if let Some(last @ (MainTurn::Finished(_) | MainTurn::Stopped(_))) = &main.last {
        return last.clone();
    }
    begin_main_frame(e, main);
    finish_main_frame(e, main)
}

/// The first half of [`run_main_frame`]: hands the engine to the game's main thread for its next
/// frame and returns at once, the frame running on main's own OS thread meanwhile. The caller
/// must not touch the engine (nor anything the game's code reaches) until
/// [`finish_main_frame`]: the engine is main's until then, as it is while the host waits in
/// [`run_main_frame`]. False (nothing handed over) when main finished or stopped; true too when
/// a frame is already in flight.
pub fn begin_main_frame(e: &mut Engine, main: &mut MainThread) -> bool {
    if main.in_flight {
        return true;
    }
    if let Some(MainTurn::Finished(_) | MainTurn::Stopped(_)) = &main.last {
        return false;
    }
    ensure_main(e);
    e.kernel.threads.current = 0;
    main.host_stack = std::mem::replace(&mut e.call_stack, std::mem::take(&mut main.call_stack));
    // `NV_PROFILE`: the sampler starts with the first frame; the hand-off to main.
    crate::profile::start();
    crate::profile::resync(&e.call_stack);
    crate::profile::set_phase(crate::profile::Phase::Switch);
    let baton = Baton(e as *mut Engine);
    // A coroutine that is gone answers its end at the finish (its channel is closed).
    let _ = main.link.resume.send(baton);
    main.in_flight = true;
    true
}

/// The second half of [`run_main_frame`]: waits until the frame [`begin_main_frame`] handed
/// over ends (main's message loop found its queue empty), returns or stops, and takes the engine
/// back. Without a frame in flight it answers the last turn again.
pub fn finish_main_frame(e: &mut Engine, main: &mut MainThread) -> MainTurn {
    if !main.in_flight {
        return main
            .last
            .clone()
            .unwrap_or_else(|| MainTurn::Stopped("no frame was handed to main".into()));
    }
    main.in_flight = false;
    let reply = main.link.yielded.recv().ok();
    main.call_stack = std::mem::replace(&mut e.call_stack, std::mem::take(&mut main.host_stack));
    crate::profile::resync(&e.call_stack);
    crate::profile::back_to_host();
    load_fpu(e);
    let turn = match reply {
        Some(Yield::Paused) => MainTurn::Frame,
        Some(Yield::Finished(code)) => MainTurn::Finished(code),
        Some(Yield::Stopped(why)) => MainTurn::Stopped(why),
        None => MainTurn::Stopped("the main thread's coroutine ended".into()),
    };
    main.last = Some(turn.clone());
    turn
}

/// The end of a frame on the game's main thread (its message loop found the queue empty): when
/// the main thread runs as the host's coroutine ([`start_main_thread`]) it hands the engine
/// back to the host and waits for the next frame, then answers true; anywhere else (a test
/// calling the game's code directly) it answers false at once.
pub fn end_main_frame(e: &mut Engine) -> bool {
    let on_coroutine = MAIN_COROUTINE.with(|c| c.borrow().is_some());
    if !on_coroutine || !on_main(e) {
        return false;
    }
    crate::profile::set_phase(crate::profile::Phase::Switch);
    MAIN_COROUTINE.with(|c| {
        let c = c.borrow();
        let (resumed, yields) = c.as_ref().expect("the main coroutine's link");
        if yields.send(Yield::Paused).is_err() || resumed.recv().is_err() {
            // The host dropped the engine: never run the frames on this stack again.
            loop {
                std::thread::park();
            }
        }
    });
    crate::profile::holding(MAIN_THREAD_ID);
    load_fpu(e);
    true
}

/// The time slice's end (`Engine::slice_hook`; turned on by [`start_slicing`]): another switch
/// point. On Windows the game's threads run beside each other, so a thread that never waits or
/// sleeps (a worker updating the scene, the main thread polling a task queue for the workers'
/// results) does not keep the others from running. Here, after `Engine::slice_len` calls of one
/// turn: a game thread other than main gives its turn back, runnable at the next pass as after
/// `Sleep(0)`; the main thread gives the others one pass. Counted in calls, so runs stay
/// deterministic. Where the turn ends follows [`Scheme::switch_at`]: with
/// [`SwitchAt::Platform`] only at a call of a platform import (`addr` in
/// `Engine::platform_calls`; until then each call asks again); with [`SwitchAt::Blocking`]
/// the turn goes on ([`Threads::overrun`]) until its next blocking call ([`blocking_call`])
/// or, [`HARD_CAP_SLICES`] slices in all, its next call. On the main thread with a wake
/// pending ([`request_wake`]) the end of the delay runs the woken threads instead, at a
/// platform call.
pub fn preempt(e: &mut Engine, addr: u32) {
    if e.kernel.threads.wake_pending && on_main(e) {
        if !e.platform_calls.contains(&addr) {
            return;
        }
        // The main thread's wake delay is over ([`request_wake`]): the woken threads run, and
        // main starts a new turn.
        e.slice_left = turn_len(e);
        e.kernel.threads.overrun = false;
        wake_waiters(e, crate::units::platform::signalled);
        return;
    }
    if e.slice_len == 0 {
        // No time slice: the hook was there only for a wake.
        e.slice_left = u64::MAX;
        return;
    }
    match e.kernel.threads.scheme.switch_at {
        SwitchAt::Any => {}
        SwitchAt::Platform => {
            if !e.platform_calls.contains(&addr) {
                return;
            }
        }
        SwitchAt::Blocking => {
            if !e.kernel.threads.overrun {
                // The slice is used up: on to the next blocking call, or to the hard cap.
                e.kernel.threads.overrun = true;
                e.slice_left = e.slice_len.saturating_mul(HARD_CAP_SLICES - 1);
                // A sleeper whose time came while main ran gets its turn soon.
                wake_parked(e);
                return;
            }
            // The hard cap: the turn ends here.
        }
    }
    end_turn(e);
}

/// [`SwitchAt::Blocking`]: the running thread is about to make a blocking platform call
/// (`WaitForSingleObject`, `WaitForMultipleObjects`; `Sleep` and a contended
/// `EnterCriticalSection` switch by themselves). A turn that has used up its slice
/// ([`Threads::overrun`]) ends here, before the call.
pub fn blocking_call(e: &mut Engine) {
    if e.kernel.threads.overrun && e.kernel.threads.scheme.switch_at == SwitchAt::Blocking {
        end_turn(e);
    }
}

/// Ends the running turn (the slice's end, [`preempt`], [`blocking_call`]): a game thread
/// other than main gives its turn back, runnable at the next pass as after `Sleep(0)`; the
/// main thread gives the others one pass.
fn end_turn(e: &mut Engine) {
    e.slice_left = turn_len(e);
    e.kernel.threads.overrun = false;
    if e.kernel.threads.list.is_empty() {
        return;
    }
    if on_main(e) {
        one_pass(e, crate::units::platform::signalled);
        return;
    }
    e.kernel.threads.preempted = e.kernel.threads.preempted.wrapping_add(1);
    let until = e.clock_now_us();
    let i = e.kernel.threads.current;
    e.kernel.threads.list[i].state = RunState::Sleeping { until };
    yield_to_main(Yield::Paused);
    load_fpu(e);
}

/// The main thread starts a new turn: a whole slice, or a pending wake's delay at most.
fn start_main_turn(e: &mut Engine) {
    e.slice_left = turn_len(e);
    e.kernel.threads.overrun = false;
    // A wake still pending is decided again: only while another thread can run now (the
    // switch may have run the ones it was for).
    e.kernel.threads.wake_pending = false;
    wake_parked(e);
}

/// The calls a turn starts with: the slice, or no end without one (`slice_len` 0).
fn turn_len(e: &Engine) -> u64 {
    if e.slice_len == 0 {
        u64::MAX
    } else {
        e.slice_len
    }
}

/// Turns the time slice on: [`preempt`] after every `calls` calls of a turn.
pub fn start_slicing(e: &mut Engine, calls: u64) {
    e.slice_len = calls;
    e.slice_left = calls;
    e.slice_hook = Some(preempt);
}

#[cfg(test)]
mod main_thread_tests {
    use super::*;

    /// A stand-in `main` whose loop ends a frame three times, then returns 7.
    fn frames(e: &mut Engine, _: &[u32]) -> crate::Ret {
        for _ in 0..3 {
            let n = e.mem.u32(0x2000);
            e.mem.set_u32(0x2000, n + 1);
            assert!(end_main_frame(e));
        }
        crate::Ret {
            eax: 7,
            ..crate::Ret::default()
        }
    }

    #[test]
    fn the_main_thread_runs_frame_by_frame() {
        let mut e = Engine::new();
        e.mem.map(0x2000, 0x1000);
        e.register(0x0040_0000, frames);
        let mut main = start_main_thread(0x0040_0000, vec![]);
        for frame in 1..=3 {
            assert_eq!(run_main_frame(&mut e, &mut main), MainTurn::Frame);
            assert_eq!(e.mem.u32(0x2000), frame);
        }
        assert_eq!(run_main_frame(&mut e, &mut main), MainTurn::Finished(7));
        assert_eq!(run_main_frame(&mut e, &mut main), MainTurn::Finished(7));
        // Outside the coroutine a frame's end doesn't wait.
        assert!(!end_main_frame(&mut e));
    }

    /// A frame handed over with `begin_main_frame` runs while the host goes on; the finish waits
    /// for it and takes the engine back, as `run_main_frame` does in one.
    #[test]
    fn a_frame_runs_between_its_begin_and_its_finish() {
        let mut e = Engine::new();
        e.mem.map(0x2000, 0x1000);
        e.register(0x0040_0000, frames);
        let mut main = start_main_thread(0x0040_0000, vec![]);
        e.call_stack.push(0x0012_3450);
        for frame in 1..=3 {
            assert!(begin_main_frame(&mut e, &mut main));
            assert!(main.in_flight());
            // Begun twice: still the one frame.
            assert!(begin_main_frame(&mut e, &mut main));
            assert_eq!(finish_main_frame(&mut e, &mut main), MainTurn::Frame);
            assert!(!main.in_flight());
            assert_eq!(e.mem.u32(0x2000), frame);
            assert_eq!(e.call_stack, [0x0012_3450], "the host's own stack is back");
        }
        assert!(begin_main_frame(&mut e, &mut main));
        assert_eq!(finish_main_frame(&mut e, &mut main), MainTurn::Finished(7));
        // Finished: nothing more is handed over, and the finish answers the end again.
        assert!(!begin_main_frame(&mut e, &mut main));
        assert_eq!(finish_main_frame(&mut e, &mut main), MainTurn::Finished(7));
    }

    #[test]
    fn a_stop_on_the_main_thread_comes_back_with_where() {
        let mut e = Engine::new();
        e.register(0x0040_0000, |e, _| e.call(0x00ab_cdef, &[]));
        let mut main = start_main_thread(0x0040_0000, vec![]);
        let MainTurn::Stopped(why) = run_main_frame(&mut e, &mut main) else {
            panic!("a stop");
        };
        assert!(why.contains("00abcdef"), "{why}");
    }
}

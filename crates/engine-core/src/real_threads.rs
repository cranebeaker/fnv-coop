//! Real threads (the default in play, `NV_THREADS=real`; ADR-0015, migration steps 5, 7, 8 and 9):
//! the game's threads run side by side on their own cores, as they do in the original, instead
//! of one at a time on the stepped host ([`crate::thread_host`]: `NV_THREADS=stepped`, the
//! engine's own default and the reference mode for tests, recordings and traces).
//!
//! **Per-thread context, shared world (step 5).** Each game thread runs on its own OS thread with
//! its own [`Engine`]: its registers live in its Rust frames as before, and the engine holds its
//! call stack, call cache, TLS block, guest stack for locals, x87 control word (the FPU register
//! of its OS thread, loaded from its TLS block), thread-local storage values, last-error code and
//! clock reading ([`Ctx`], [`Engine::real`]). What they share: guest memory (each engine holds a
//! view of the one memory, `Mem::start_sharing`: the same block, so loads and stores are the same
//! instructions as before), the kernel objects (`Engine::kernel`, one table every engine points
//! at, changed only under the kernel lock below), the function table (each engine has a copy made
//! from a template at start; functions registered later reach every engine, [`registered`]) and
//! the clock (every engine counts from the same real-time mark). The main thread's engine is the
//! host's own; the host touches it only between frames, as with stepped threads.
//!
//! **Real synchronisation (step 7).** The exe has no inline `lock`-prefixed instruction and no
//! memory `xchg` outside the C runtime's exception-handler cache lock (`0127073c`, counted over the
//! unpacked exe's code, measure/real-threads): every interlocked operation of the game goes
//! through the `Interlocked*` imports, which here are real atomic operations on the guest word
//! (sequentially consistent, as `lock`-prefixed instructions are; a word not 4-aligned is
//! serialised by a lock of its own). Critical sections keep their owner, holds, waiters and spin
//! count in their own guest `RTL_CRITICAL_SECTION` words, as Windows does: an uncontended enter
//! or leave is one atomic operation on the section and touches nothing shared; a contended enter
//! spins the section's spin count, then parks until the leave wakes it. `TlsGetValue`,
//! `TlsSetValue`, `GetCurrentThreadId`, `GetLastError` and `SetLastError` answer from the calling
//! thread's context, and so do `FlsGetValue` and `FlsSetValue` (found by name; the C runtime reads
//! its per-thread data through them on every thread); none of them takes the kernel lock. `Sleep`
//! sleeps for real (`Sleep(0)` yields). Events, semaphores and thread handles stay in the kernel
//! table: a wait checks them under the kernel lock and, when not signalled, the thread parks; a
//! change of an object (`SetEvent`, `ReleaseSemaphore`, a thread's end:
//! `thread_host::signal_object`) wakes only the threads waiting on that object, as Windows wakes
//! them. (Waking every waiter at every change kept about 12 cores busy in Goodsprings.)
//!
//! **Allocator and platform locks (step 8).** The heap (and the asked sizes `_msize` reads) is
//! behind one lock (`Mem`'s shared state). Every other platform function, the Direct3D 9, sound,
//! file, window and process ones, runs under one re-entrant kernel lock ([`locked_call`]), so the
//! platform layer's state (`Kernel::platform`) is changed by one thread at a time. The exe creates
//! its Direct3D device with `D3DCREATE_MULTITHREADED` (behaviour flags 0x54, `d3dxreal.rs`'s
//! `GAME_BEHAVIOR`), so the device itself serialises calls too; the lock keeps the platform's own
//! bookkeeping right. Pure functions are left outside it: the clock imports and D3DX's matrix,
//! vector and plane maths. The Direct3D 9 device and its objects (the layer's `device_calls`,
//! about 7,000 calls a frame) take a lock of their own instead, the device lock
//! ([`device_call`]); kernel-locked code that reaches the device's state takes it inside the
//! kernel lock ([`device_guard`]). **Lock order: the kernel lock, then the device lock; never the
//! kernel lock while the device lock is held.** Device functions touch no kernel object and no
//! last-error code, so they never need the kernel lock.
//!
//! **The host (step 9).** [`start`] turns real threads on for an engine before the game's `main`
//! runs; from then on `CreateThread` starts a real OS thread at once (or at `ResumeThread` to 0).
//! A thread that stops (an untranslated function, a fault) ends the run at main's next frame end,
//! naming the thread. **Diagnostics force stepped threads** (R6): a trace, a recording, a profile,
//! the heap census, the touch census, a watch, a per-call hook: [`start`] refuses and the run goes
//! on stepped. Three run with real threads: the block check (`NV_BLOCK_CHECK`; it sees allocations
//! and frees, each named with its thread, not stores), `NV_REAL_DUMP` ([`dump_threads`]) and
//! `NV_LOCK_STATS` (each frame's waits for the locks, [`LockStats`]).
//!
//! **Page-table changes.** Mapping pages is safe while other threads run (the page is committed
//! before the table says so). Handing pages back to the system is not: another thread may be
//! between its page test and its access. So unmapped pages stay committed (marked kept) and go
//! back only at a barrier ([`World::barrier`]): every game thread stopped at a safe point (a
//! platform call, an interlocked operation, a wait or sleep), done by the host between frames when
//! enough pages wait (`HAND_BACK_AT`).
//!
//! **What Rust's model says.** Two threads that access the same guest bytes without the game's
//! own synchronisation, one of them writing, are a data race: undefined behaviour in Rust's and
//! LLVM's model, where the original's races were defined by the processor (`mem.rs`, "Threads").
//! We rely on LLVM emitting plain moves for raw-pointer accesses and moving none across a call it
//! cannot see through; the game's synchronisation is such calls. A loop in typed code that waits
//! on guest memory must make such a call (docs/ENGINE_CRATE.md, "A risk for later").

use crate::abi::{AbiFn, Ret};
use crate::engine::AddrMap;
use crate::ptr::Ptr;
use crate::thread_host::{RunState, Signalled, ThreadHost, MAIN_THREAD_ID};
use crate::Engine;
use std::cell::Cell;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, AtomicUsize, Ordering};
use std::sync::{Condvar, Mutex, MutexGuard, PoisonError};
use std::time::Duration;

/// The kept pages over the limit that make the host hand them back at a frame's end (64 MB): a
/// barrier is a few hundred microseconds, so not every frame.
const HAND_BACK_AT: usize = 16384;

/// The synchronisation imports real threads replace with their own ([`sync_import`]).
const SYNC_IMPORTS: [&str; 17] = [
    "InterlockedIncrement",
    "InterlockedDecrement",
    "InterlockedExchange",
    "InterlockedExchangeAdd",
    "InterlockedCompareExchange",
    "TlsGetValue",
    "TlsSetValue",
    "GetCurrentThreadId",
    "GetLastError",
    "SetLastError",
    "EnterCriticalSection",
    "LeaveCriticalSection",
    "TryEnterCriticalSection",
    "InitializeCriticalSection",
    "DeleteCriticalSection",
    "InitializeCriticalSectionAndSpinCount",
    "Sleep",
];

/// Imports that only read the calling thread's clock or work on guest memory they are given:
/// they run outside the kernel lock as they are.
fn unlocked_import(name: &str) -> bool {
    matches!(
        name,
        "GetTickCount" | "timeGetTime" | "QueryPerformanceCounter" | "QueryPerformanceFrequency"
    ) || name.starts_with("D3DXMatrix")
        || name.starts_with("D3DXVec")
        || name.starts_with("D3DXPlane")
}

/// The bytes of a real game thread's OS stack, as the stepped host's coroutines have.
const THREAD_STACK: usize = 64 << 20;

fn guard<T>(m: &Mutex<T>) -> MutexGuard<'_, T> {
    m.lock().unwrap_or_else(PoisonError::into_inner)
}

fn wait<'a, T>(cv: &Condvar, g: MutexGuard<'a, T>) -> MutexGuard<'a, T> {
    cv.wait(g).unwrap_or_else(PoisonError::into_inner)
}

fn wait_for<'a, T>(cv: &Condvar, g: MutexGuard<'a, T>, t: Duration) -> MutexGuard<'a, T> {
    cv.wait_timeout(g, t)
        .unwrap_or_else(PoisonError::into_inner)
        .0
}

thread_local! {
    /// This OS thread's token for the kernel lock's owner word (never 0).
    static TOKEN: u64 = {
        static NEXT: AtomicU64 = AtomicU64::new(1);
        NEXT.fetch_add(1, Ordering::Relaxed)
    };
    /// Whether this OS thread is counted as running game code ([`World::enter_guest`]).
    static COUNTED: Cell<bool> = const { Cell::new(false) };
    /// This OS thread runs a game thread other than main ([`run_thread`]); [`LockStats`] counts
    /// its waits apart from main's (and the host's).
    static GAME_THREAD: Cell<bool> = const { Cell::new(false) };
}

/// [`LockStats`]' row for this OS thread: 0 the main thread (or the host's), 1 another game thread.
fn role() -> usize {
    usize::from(GAME_THREAD.with(Cell::get))
}

/// One kind of wait in [`LockStats`]: the time, how many and the longest, since the last frame.
#[derive(Default)]
struct WaitSum {
    ns: AtomicU64,
    count: AtomicU64,
    max_ns: AtomicU64,
}

impl WaitSum {
    fn add(&self, d: Duration) {
        let ns = d.as_nanos().min(u128::from(u64::MAX)) as u64;
        self.ns.fetch_add(ns, Ordering::Relaxed);
        self.count.fetch_add(1, Ordering::Relaxed);
        self.max_ns.fetch_max(ns, Ordering::Relaxed);
    }

    /// The sums since the last take, in microseconds (time, count, longest), and a new start.
    fn take(&self) -> (u64, u64, u64) {
        (
            self.ns.swap(0, Ordering::Relaxed) / 1000,
            self.count.swap(0, Ordering::Relaxed),
            self.max_ns.swap(0, Ordering::Relaxed) / 1000,
        )
    }
}

/// `NV_LOCK_STATS=FILE` (a diagnostic of real threads): per main frame, how long the main thread
/// and the other game threads waited for the kernel lock, for critical sections (all behind one
/// table and one condition variable), for kernel objects (events, semaphores, threads) and in
/// sleeps; how many section leaves woke waiters and how many of those waiters found the section
/// still held; and how long the host held the kernel lock between frames. One line a frame
/// ([`Ctx::frame_ended`]), in microseconds.
struct LockStats {
    kernel: [WaitSum; 2],
    section: [WaitSum; 2],
    object: [WaitSum; 2],
    sleep: [WaitSum; 2],
    notifies: AtomicU64,
    spurious: AtomicU64,
    /// How many times a thread blocked on objects woke ([`World::wait_for_objects`]): main, others.
    wakes: [AtomicU64; 2],
    host_hold: WaitSum,
    host_since: Mutex<Option<std::time::Instant>>,
    out: Mutex<std::io::BufWriter<std::fs::File>>,
    frames: AtomicU64,
}

/// Whether `NV_LOCK_STATS` is on ([`LockStats`]).
pub(crate) fn lock_stats_on() -> bool {
    lock_stats().is_some()
}

/// [`LockStats`] when `NV_LOCK_STATS` names a file that could be made; read once.
fn lock_stats() -> Option<&'static LockStats> {
    static STATS: std::sync::OnceLock<Option<LockStats>> = std::sync::OnceLock::new();
    STATS
        .get_or_init(|| {
            let path = std::env::var_os("NV_LOCK_STATS")?;
            let file = std::fs::File::create(&path)
                .map_err(|why| eprintln!("NV_LOCK_STATS: {}: {why}", path.to_string_lossy()))
                .ok()?;
            let mut out = std::io::BufWriter::new(file);
            use std::io::Write;
            let _ = writeln!(
                out,
                "frame\tclock_us\tmain_kernel_us\tmain_kernel_n\tmain_kernel_max_us\tgame_kernel_us\t\
                 game_kernel_n\tgame_kernel_max_us\tmain_section_us\tmain_section_n\t\
                 main_section_max_us\tgame_section_us\tgame_section_n\tgame_section_max_us\t\
                 main_object_us\tmain_object_n\tmain_object_max_us\tgame_object_us\t\
                 game_object_n\tgame_object_max_us\tmain_sleep_us\tmain_sleep_n\tgame_sleep_us\t\
                 game_sleep_n\tsection_notifies\tsection_spurious\thost_hold_us\tmain_wakes\tgame_wakes"
            );
            Some(LockStats {
                kernel: Default::default(),
                section: Default::default(),
                object: Default::default(),
                sleep: Default::default(),
                notifies: AtomicU64::new(0),
                spurious: AtomicU64::new(0),
                wakes: Default::default(),
                host_hold: WaitSum::default(),
                host_since: Mutex::new(None),
                out: Mutex::new(out),
                frames: AtomicU64::new(0),
            })
        })
        .as_ref()
}

impl LockStats {
    /// Main's frame ended (the host took the kernel lock): the frame's line, and the hold starts.
    fn frame_ended(&self, clock_us: u64) {
        use std::io::Write;
        let frame = self.frames.fetch_add(1, Ordering::Relaxed) + 1;
        let mut line = format!("{frame}\t{clock_us}");
        for sums in [&self.kernel, &self.section, &self.object] {
            for s in sums {
                let (us, n, max) = s.take();
                line.push_str(&format!("\t{us}\t{n}\t{max}"));
            }
        }
        for s in &self.sleep {
            let (us, n, _) = s.take();
            line.push_str(&format!("\t{us}\t{n}"));
        }
        let (hold, _, _) = self.host_hold.take();
        line.push_str(&format!(
            "\t{}\t{}\t{hold}\t{}\t{}",
            self.notifies.swap(0, Ordering::Relaxed),
            self.spurious.swap(0, Ordering::Relaxed),
            self.wakes[0].swap(0, Ordering::Relaxed),
            self.wakes[1].swap(0, Ordering::Relaxed)
        ));
        let mut out = guard(&self.out);
        let _ = writeln!(out, "{line}");
        if frame % 60 == 0 {
            let _ = out.flush();
        }
        drop(out);
        *guard(&self.host_since) = Some(std::time::Instant::now());
    }

    /// The host gives the kernel lock back for main's next frame.
    fn frame_begins(&self) {
        if let Some(t) = guard(&self.host_since).take() {
            self.host_hold.add(t.elapsed());
        }
    }
}

fn token() -> u64 {
    TOKEN.with(|t| *t)
}

/// The kernel lock: re-entrant (a platform function that calls game code that calls another
/// takes it again), released whole by a wait ([`KernelLock::wait_change`]), whose condition
/// variable every change of an object's state wakes.
struct KernelLock {
    /// The holder's [`token`], 0 when free.
    owner: AtomicU64,
    /// How many times the holder holds it (only the holder changes it).
    depth: AtomicU32,
    /// Threads waiting to take it.
    waiters: AtomicU32,
    /// An object's state changed while it was held ([`Ctx::changed`]): its release wakes waiters.
    dirty: AtomicBool,
    m: Mutex<()>,
    free: Condvar,
    changed: Condvar,
}

impl KernelLock {
    fn new() -> Self {
        KernelLock {
            owner: AtomicU64::new(0),
            depth: AtomicU32::new(0),
            waiters: AtomicU32::new(0),
            dirty: AtomicBool::new(false),
            m: Mutex::new(()),
            free: Condvar::new(),
            changed: Condvar::new(),
        }
    }

    fn held_here(&self) -> bool {
        self.owner.load(Ordering::Relaxed) == token()
    }

    /// Takes it (again, when this thread holds it). A safe point for the barrier.
    fn lock(&self, w: &World) {
        let me = token();
        if self.owner.load(Ordering::Relaxed) == me {
            self.depth
                .store(self.depth.load(Ordering::Relaxed) + 1, Ordering::Relaxed);
            return;
        }
        w.safe_point();
        if self
            .owner
            .compare_exchange(0, me, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            let was = w.leave_guest();
            self.acquire(me);
            if was {
                w.enter_guest();
            }
        }
        self.depth.store(1, Ordering::Relaxed);
    }

    /// Waits until it is free and takes it (not held by this thread). With `NV_LOCK_STATS` the
    /// wait is counted ([`LockStats`]).
    #[cold]
    fn acquire(&self, me: u64) {
        let started = lock_stats().map(|_| std::time::Instant::now());
        self.acquire_here(me);
        if let (Some(s), Some(t)) = (lock_stats(), started) {
            s.kernel[role()].add(t.elapsed());
        }
    }

    fn acquire_here(&self, me: u64) {
        for _ in 0..64 {
            std::hint::spin_loop();
            if self.owner.load(Ordering::Relaxed) == 0
                && self
                    .owner
                    .compare_exchange(0, me, Ordering::Acquire, Ordering::Relaxed)
                    .is_ok()
            {
                return;
            }
        }
        let mut g = guard(&self.m);
        self.waiters.fetch_add(1, Ordering::SeqCst);
        while self
            .owner
            .compare_exchange(0, me, Ordering::SeqCst, Ordering::SeqCst)
            .is_err()
        {
            g = wait(&self.free, g);
        }
        self.waiters.fetch_sub(1, Ordering::SeqCst);
    }

    /// Gives one hold back; the last wakes the waiters of a change made under it and one thread
    /// waiting to take it.
    fn unlock(&self) {
        let depth = self.depth.load(Ordering::Relaxed) - 1;
        self.depth.store(depth, Ordering::Relaxed);
        if depth > 0 {
            return;
        }
        // Only the holder sets it, so a plain load answers for the common case (nothing changed)
        // without an atomic exchange on every release.
        if self.dirty.load(Ordering::Relaxed) {
            self.dirty.store(false, Ordering::Relaxed);
            let _g = guard(&self.m);
            self.changed.notify_all();
        }
        self.owner.store(0, Ordering::SeqCst);
        if self.waiters.load(Ordering::SeqCst) > 0 {
            let _g = guard(&self.m);
            self.free.notify_one();
        }
    }

    /// Gives every hold of this thread back, answering how many (0 when it holds none).
    fn release_all(&self) -> u32 {
        if !self.held_here() {
            return 0;
        }
        let depth = self.depth.load(Ordering::Relaxed);
        self.depth.store(1, Ordering::Relaxed);
        self.unlock();
        depth
    }

    /// Takes it again with `depth` holds ([`KernelLock::release_all`]); nothing for 0.
    fn take_again(&self, w: &World, depth: u32) {
        if depth > 0 {
            self.lock(w);
            self.depth.store(depth, Ordering::Relaxed);
        }
    }

    /// Held by this thread: gives it back whole, waits until an object changes state (or
    /// `timeout`), and takes it again. The thread is not running game code meanwhile.
    fn wait_change(&self, w: &World, timeout: Option<Duration>) {
        let me = token();
        debug_assert!(self.held_here());
        let depth = self.depth.load(Ordering::Relaxed);
        let mut g = guard(&self.m);
        // Released while `m` is held, so a change made by the next holder is notified only once
        // this thread waits (it takes `m` to notify).
        if self.dirty.swap(false, Ordering::Relaxed) {
            self.changed.notify_all();
        }
        self.depth.store(0, Ordering::Relaxed);
        self.owner.store(0, Ordering::SeqCst);
        if self.waiters.load(Ordering::SeqCst) > 0 {
            self.free.notify_one();
        }
        let was = w.leave_guest();
        g = match timeout {
            Some(t) => wait_for(&self.changed, g, t),
            None => wait(&self.changed, g),
        };
        drop(g);
        if self
            .owner
            .compare_exchange(0, me, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            self.acquire(me);
        }
        self.depth.store(depth, Ordering::Relaxed);
        if was {
            w.enter_guest();
        }
    }
}

/// A critical section's owner (a thread id, 0 when free) and its count of holds.
#[derive(Default, Clone, Copy)]
struct Section {
    owner: u32,
    count: u32,
}

/// An engine moved to the OS thread that runs it (or kept as the template).
struct SendEngine(Box<Engine>);

// SAFETY: a thread's engine is used by the one OS thread it is moved to; what it shares with other
// engines (memory, the kernel table, the template's tables) is reached under the locks the module
// documentation names. Its `Rc`s (test doubles, the open handler) are not copied into it
// (`Engine::thread_context`).
unsafe impl Send for SendEngine {}

/// What the real threads of one run share.
pub struct World {
    kernel: KernelLock,
    /// The device lock: the Direct3D 9 platform functions' own ([`device_call`], the layer's
    /// `device_calls`), so the thousands of device calls a frame do not take the kernel lock.
    /// Taken inside the kernel lock where both are held (a kernel-locked function that reaches the
    /// device's state: [`device_guard`]), never the other way round.
    device: KernelLock,
    /// Critical sections not 4-aligned (a fallback): their owners and holds. Aligned ones keep
    /// theirs in their own guest words ([`section_words`]).
    sections: Mutex<AddrMap<u32, Section>>,
    /// The threads parked waiting for a critical section, each with its section's address.
    section_parkers: Mutex<Vec<std::sync::Arc<Waiter>>>,
    /// Interlocked operations on words that are not 4-aligned.
    unaligned: Mutex<()>,
    /// Game threads running game code ([`World::enter_guest`]).
    running: AtomicUsize,
    /// A barrier is on: threads stop at their next safe point.
    stop: AtomicBool,
    barrier: Mutex<()>,
    barrier_moved: Condvar,
    /// Functions registered after the start, in order ([`registered`]).
    late: Mutex<Vec<(u32, AbiFn)>>,
    late_count: AtomicUsize,
    /// The import slots and their names (the kernel table's, copied at the start).
    import_names: AddrMap<u32, &'static str>,
    /// The engine new threads' engines are made from.
    template: Mutex<Option<SendEngine>>,
    /// The first stop of a game thread other than main, for main's next frame end.
    stopped: Mutex<Option<String>>,
    /// A game thread that has run has a suspend count above 0 (`SuspendThread`): threads look at
    /// their own at their next platform call, wait or sleep and park there ([`park_if_suspended`]):
    /// each thread's flag by its index in the thread host's list ([`Ctx::suspend`]), set under
    /// the kernel lock ([`suspends_changed`]).
    suspend_flags: Mutex<Vec<(usize, std::sync::Arc<AtomicBool>)>>,
    /// The threads blocked in a wait on kernel objects, each with its objects
    /// ([`World::wait_for_objects`]): a change of an object wakes only its own waiters.
    waiters: Mutex<Vec<std::sync::Arc<Waiter>>>,
    /// How many times blocked threads woke from waits on objects (all threads, the whole run).
    wakes: AtomicU64,
    /// Threads parked by a suspend now, and parks so far ([`park_if_suspended`]).
    parked: AtomicU32,
    parks: AtomicU32,
    /// `SuspendThread` calls so far ([`suspend_called`]).
    suspends: AtomicU32,
    /// [`World::stopped`] holds a stop: the main thread's check before each sleep's end is one
    /// load ([`main_stop_check`]).
    any_stop: AtomicBool,
    /// [`start`] took the kernel lock on the host's thread for the start-up (the C runtime's, the
    /// static initializers, the host's own use of the kernel table); the first frame gives it back
    /// ([`Ctx::frame_begins`]).
    held_from_start: AtomicBool,
    /// Each thread-local storage index's generation: `TlsAlloc` and `TlsFree` move it on, so a
    /// value a thread set before reads 0 ([`Ctx::tls`]).
    tls_gen: Box<[AtomicU32]>,
    /// The same for fiber-local storage (`FlsAlloc`, `FlsFree`), and which indexes are allocated
    /// (`FlsSetValue` answers 0 for one that is not): real threads keep FLS values per thread too
    /// ([`Ctx::fls`]), off the kernel lock.
    fls_gen: Box<[AtomicU32]>,
    fls_in_use: Box<[AtomicBool]>,
    /// `NV_REAL_DUMP` (a diagnostic): every thread's record, for [`dump_threads`].
    dump: Option<Dump>,
}

impl World {
    fn new(import_names: AddrMap<u32, &'static str>) -> Self {
        World {
            kernel: KernelLock::new(),
            device: KernelLock::new(),
            sections: Mutex::new(AddrMap::default()),
            section_parkers: Mutex::new(Vec::new()),
            unaligned: Mutex::new(()),
            running: AtomicUsize::new(0),
            stop: AtomicBool::new(false),
            barrier: Mutex::new(()),
            barrier_moved: Condvar::new(),
            late: Mutex::new(Vec::new()),
            late_count: AtomicUsize::new(0),
            import_names,
            template: Mutex::new(None),
            stopped: Mutex::new(None),
            suspend_flags: Mutex::new(Vec::new()),
            waiters: Mutex::new(Vec::new()),
            wakes: AtomicU64::new(0),
            parked: AtomicU32::new(0),
            parks: AtomicU32::new(0),
            suspends: AtomicU32::new(0),
            any_stop: AtomicBool::new(false),
            held_from_start: AtomicBool::new(false),
            tls_gen: (0..TLS_INDEXES).map(|_| AtomicU32::new(0)).collect(),
            fls_gen: (0..FLS_INDEXES).map(|_| AtomicU32::new(0)).collect(),
            fls_in_use: (0..FLS_INDEXES).map(|_| AtomicBool::new(false)).collect(),
            dump: dump_from_env().then(Dump::default),
        }
    }

    /// This OS thread runs game code from now on (counted for the barrier); waits while a barrier
    /// is on.
    fn enter_guest(&self) {
        loop {
            if self.stop.load(Ordering::SeqCst) {
                let mut g = guard(&self.barrier);
                while self.stop.load(Ordering::SeqCst) {
                    g = wait(&self.barrier_moved, g);
                }
            }
            self.running.fetch_add(1, Ordering::SeqCst);
            if !self.stop.load(Ordering::SeqCst) {
                break;
            }
            self.running.fetch_sub(1, Ordering::SeqCst);
            let _g = guard(&self.barrier);
            self.barrier_moved.notify_all();
        }
        COUNTED.with(|c| c.set(true));
    }

    /// This OS thread stops running game code (a wait, a sleep, its end); answers whether it was
    /// counted (the host's own thread never is).
    fn leave_guest(&self) -> bool {
        // No reference into a shared extension slot is live here: give them back.
        if crate::extend::holds_shared() {
            crate::extend::release_held();
        }
        if !COUNTED.with(|c| c.replace(false)) {
            return false;
        }
        self.running.fetch_sub(1, Ordering::SeqCst);
        if self.stop.load(Ordering::SeqCst) {
            let _g = guard(&self.barrier);
            self.barrier_moved.notify_all();
        }
        true
    }

    /// A point where a game thread may stop for a barrier: platform calls, interlocked
    /// operations, waits and sleeps. One load when no barrier is on.
    #[inline]
    fn safe_point(&self) {
        if crate::extend::holds_shared() {
            crate::extend::release_held();
        }
        if self.stop.load(Ordering::Relaxed) && self.leave_guest() {
            self.enter_guest();
        }
    }

    /// Runs `f` while every game thread is stopped at a safe point or waiting (none running game
    /// code). Called by the host between frames (main is waiting then), holding the kernel lock
    /// (so no thread stopped here holds it).
    pub fn barrier<R>(&self, f: impl FnOnce() -> R) -> R {
        let was = self.leave_guest();
        let mut g = guard(&self.barrier);
        self.stop.store(true, Ordering::SeqCst);
        while self.running.load(Ordering::SeqCst) > 0 {
            g = wait_for(&self.barrier_moved, g, Duration::from_millis(1));
        }
        let r = f();
        self.stop.store(false, Ordering::SeqCst);
        self.barrier_moved.notify_all();
        drop(g);
        if was {
            self.enter_guest();
        }
        r
    }

    /// The engine of a new game thread `id` (index `index` in the thread host's list).
    fn context(&'static self, id: u32, index: usize) -> Engine {
        let template = guard(&self.template);
        let template = &template
            .as_ref()
            .expect("real threads' template (made by `start`)")
            .0;
        let mut ctx = template
            .real
            .as_ref()
            .expect("the template is a real thread's engine")
            .for_thread(id, index);
        ctx.record = self.new_record(id);
        guard(&self.suspend_flags).push((index, std::sync::Arc::clone(&ctx.suspend)));
        let mut e = template.thread_context(ctx);
        e.mem.set_view_thread(id);
        if self.dump.is_some() {
            e.set_call_hook(Some(record_call));
        }
        e
    }

    /// `NV_REAL_DUMP`: a new record for thread `id`, listed for [`dump_threads`].
    fn new_record(&self, id: u32) -> Option<std::sync::Arc<ThreadRecord>> {
        let dump = self.dump.as_ref()?;
        let record = std::sync::Arc::new(ThreadRecord::new(id));
        guard(&dump.records).push(record.clone());
        Some(record)
    }

    /// Whether the platform function at `addr` (not a replaced synchronisation import) runs under
    /// the kernel lock.
    fn locked_addr(&self, addr: u32) -> bool {
        match self.import_names.get(&addr) {
            Some(name) => !SYNC_IMPORTS.contains(name) && !unlocked_import(name),
            // A platform object's code (COM methods, `GetProcAddress` answers). Game addresses are
            // the game's code, even where a platform version stands in (the memory manager, the
            // C runtime's heap: on the heap's own lock).
            None => crate::mem::PLATFORM_AREA.contains(&addr),
        }
    }
}

/// A thread blocked in a wait on kernel objects ([`World::wait_for_objects`]).
struct Waiter {
    thread: std::thread::Thread,
    handles: Vec<u32>,
    /// One of its objects changed since it last looked (set under the kernel lock).
    woken: AtomicBool,
}

impl World {
    /// Held by this thread (the kernel lock): gives it back whole, parks until one of `me`'s
    /// objects changes ([`object_signalled`]) or `timeout`, and takes it again. The thread is not
    /// running game code meanwhile. A change made after the flag is cleared here unparks it (it
    /// sets the flag under the lock, then unparks), so none is missed.
    fn wait_for_objects(&self, me: &Waiter, timeout: Option<Duration>) {
        me.woken.store(false, Ordering::SeqCst);
        let held = self.kernel.release_all();
        let was = self.leave_guest();
        if !me.woken.load(Ordering::SeqCst) {
            match timeout {
                Some(t) => std::thread::park_timeout(t),
                None => std::thread::park(),
            }
        }
        if was {
            self.enter_guest();
        }
        self.kernel.take_again(self, held);
        self.wakes.fetch_add(1, Ordering::Relaxed);
        if let Some(s) = lock_stats() {
            s.wakes[role()].fetch_add(1, Ordering::Relaxed);
        }
    }

    /// Wakes every thread blocked in a wait (a game thread stopped: the main thread ends with it).
    fn wake_all_waiters(&self) {
        for o in guard(&self.waiters)
            .iter()
            .chain(guard(&self.section_parkers).iter())
        {
            o.woken.store(true, Ordering::SeqCst);
            o.thread.unpark();
        }
    }
}

/// The object `handle` changed so that waiters on it may go on (under the kernel lock,
/// `thread_host::signal_object`): wakes the threads waiting on it, and only them.
pub(crate) fn object_signalled(e: &Engine, handle: u32) {
    let Some(r) = &e.real else { return };
    for o in guard(&r.world.waiters).iter() {
        if o.handles.contains(&handle) {
            o.woken.store(true, Ordering::SeqCst);
            o.thread.unpark();
        }
    }
}

/// One game thread's own part of real threads (`Engine::real`).
pub struct Ctx {
    /// Its id (`GetCurrentThreadId`) and its index in the thread host's list.
    pub id: u32,
    pub index: usize,
    /// Its last-error code (`GetLastError`), swapped into the kernel table around each locked
    /// platform call.
    pub last_error: u32,
    /// Its thread-local storage values (`TlsGetValue`), by index, each with the index's generation
    /// when it was set ([`World::tls_gen`]): a value from before the index was freed or handed out
    /// again reads 0, as Windows clears an index on every thread.
    tls: Vec<(u32, u32)>,
    /// Its fiber-local storage values (`FlsGetValue`), as [`Ctx::tls`] (each game thread is one
    /// fiber).
    fls: Vec<(u32, u32)>,
    /// `NV_REAL_DUMP`: what this thread is doing, for [`dump_threads`]; `None` otherwise.
    record: Option<std::sync::Arc<ThreadRecord>>,
    /// This thread is suspended (`SuspendThread`) and parks at its next platform call, wait or
    /// sleep ([`World::suspend_flags`]).
    suspend: std::sync::Arc<AtomicBool>,
    /// The platform functions behind the kernel lock, by address: what [`locked_call`] runs.
    locked: AddrMap<u32, AbiFn>,
    /// How many of [`World::late`] this engine has taken.
    late_seen: usize,
    world: &'static World,
}

impl Ctx {
    /// A context like this one for game thread `id` at `index`: no TLS values, no last error.
    fn for_thread(&self, id: u32, index: usize) -> Ctx {
        Ctx {
            id,
            index,
            last_error: 0,
            tls: Vec::new(),
            fls: Vec::new(),
            record: None,
            suspend: std::sync::Arc::new(AtomicBool::new(false)),
            locked: self.locked.clone(),
            late_seen: self.late_seen,
            world: self.world,
        }
    }

    /// An object's state changed (under the kernel lock): its waiters look again when it is
    /// released (`thread_host::signal`).
    pub(crate) fn changed(&self) {
        // Only the holder sets the flag, which is what lets the release read it plainly.
        debug_assert!(
            self.world.kernel.held_here(),
            "an object changed without the kernel lock"
        );
        self.world.kernel.dirty.store(true, Ordering::Relaxed);
    }

    /// The main thread starts running game code (`true`, counted for the barrier) or ends.
    pub(crate) fn main_runs(&self, runs: bool) {
        if runs {
            self.world.enter_guest();
        } else {
            self.world.leave_guest();
        }
    }

    /// The main thread at its frame's end: gives the kernel lock back whole and stops running game
    /// code (`thread_host::end_main_frame`). Answers the holds to take again.
    pub(crate) fn release_kernel(&self) -> u32 {
        let depth = self.world.kernel.release_all();
        self.world.leave_guest();
        depth
    }

    /// The main thread at its next frame: runs game code again and takes the kernel lock with
    /// `depth` holds ([`Ctx::release_kernel`]).
    pub(crate) fn take_kernel(&self, depth: u32) {
        self.world.enter_guest();
        self.world.kernel.take_again(self.world, depth);
    }

    /// The host hands a frame to main (`thread_host::begin_main_frame`): the kernel lock it took
    /// at the last frame's end goes back.
    pub(crate) fn frame_begins(&self, host_holds: bool) {
        if let Some(s) = lock_stats() {
            s.frame_begins();
        }
        if host_holds {
            self.world.kernel.unlock();
        }
        // The hold [`start`] took for the start-up before the first frame.
        if self.world.held_from_start.swap(false, Ordering::SeqCst) {
            self.world.kernel.unlock();
        }
    }

    /// Main's frame ended (`thread_host::finish_main_frame`, on the host's thread): the host takes
    /// the kernel lock while it has the engine, and hands deferred pages back at a barrier when
    /// enough wait. Answers a game thread's stop, if one stopped.
    pub(crate) fn frame_ended(&self, mem: &mut crate::Mem, clock_us: u64) -> Option<String> {
        let w = self.world;
        w.kernel.lock(w);
        if let Some(s) = lock_stats() {
            s.frame_ended(clock_us);
        }
        if mem.deferred_pages() >= HAND_BACK_AT {
            w.barrier(|| mem.hand_back_deferred());
        }
        guard(&w.stopped).clone()
    }
}

/// Runs `f`, a wait outside the platform layer (a shared extension slot another thread holds,
/// `extend::hold`), as a thread that runs no game code meanwhile, so a barrier does not wait for
/// it; with stepped threads (`real` none) it just runs `f`.
pub(crate) fn blocking<R>(real: Option<&Ctx>, f: impl FnOnce() -> R) -> R {
    match real {
        Some(c) => {
            let was = c.world.leave_guest();
            let r = f();
            if was {
                c.world.enter_guest();
            }
            r
        }
        None => f(),
    }
}

/// Turns real threads on for `e`, before the game makes its threads (the host, before the
/// game's `main`): its memory becomes shared, the synchronisation imports become real ones, the
/// other platform functions go behind the kernel lock, and `e` becomes the main thread's engine.
/// Refused (stepped threads stay) while a diagnostic is on, or once a game thread ran stepped; a
/// thread made earlier that has not run yet starts here, on its own OS thread (one made suspended
/// starts when resumed).
///
/// Returns holding the kernel lock on the calling (host's) thread: the start-up before the first
/// frame (the C runtime's, the static initializers, the host's own use of the kernel table) runs
/// with it, and game threads made then wait for it at their first platform call. The first
/// `Ctx::frame_begins` (`thread_host::begin_main_frame`) gives it back. A caller that runs no frames
/// (a test) keeps it: game threads then take it only while the caller waits or sleeps.
pub fn start(e: &mut Engine) -> Result<(), String> {
    if e.real.is_some() {
        return Ok(());
    }
    if let Some(why) = diagnostic_on(e) {
        return Err(format!(
            "NV_THREADS=real: {why} needs stepped threads (they run stepped)"
        ));
    }
    crate::thread_host::ensure_main(e);
    if let Some(t) = e.kernel.threads.list[1..]
        .iter()
        .find(|t| t.state != crate::thread_host::RunState::NotStarted)
    {
        return Err(format!(
            "NV_THREADS=real: game thread {} (start {:08x}) ran stepped already (they run stepped)",
            t.id, t.start
        ));
    }
    // No time slices and no wakes: the threads run beside each other.
    e.slice_hook = None;
    e.slice_len = 0;
    e.slice_left = 0;
    e.kernel.threads.wake_pending = false;
    e.kernel.threads.overrun = false;
    // The kernel table, the memory and the world below live for good: game threads may still use
    // them after the main engine is gone.
    e.kernel.leak();
    e.mem.start_sharing();
    let world: &'static World = Box::leak(Box::new(World::new(e.kernel.import_names.clone())));
    let mut locked = AddrMap::default();
    let mut addrs = e.registered_addrs();
    addrs.sort_unstable();
    for addr in addrs {
        if let Some(name) = world.import_names.get(&addr) {
            if let Some(f) = sync_import(name) {
                e.register_here(addr, f);
                continue;
            }
        }
        if let Some(f) = real_fls(addr) {
            e.register_here(addr, f);
            continue;
        }
        if world.locked_addr(addr) {
            if let Some(f) = e.registered_fn(addr) {
                locked.insert(addr, f);
                e.register_here(addr, locked_call);
            }
        }
    }
    let tls = crate::units::platform::tls_values_of(e, MAIN_THREAD_ID)
        .into_iter()
        .map(|v| (v, 0))
        .collect();
    e.real = Some(Box::new(Ctx {
        id: MAIN_THREAD_ID,
        index: 0,
        last_error: e.kernel.last_error,
        tls,
        fls: Vec::new(),
        record: world.new_record(MAIN_THREAD_ID),
        suspend: std::sync::Arc::new(AtomicBool::new(false)),
        locked,
        late_seen: 0,
        world,
    }));
    e.mem.set_view_thread(MAIN_THREAD_ID);
    if world.dump.is_some() {
        e.set_call_hook(Some(record_call));
    }
    let template = {
        let ctx = e.real.as_ref().map(|r| r.for_thread(0, 0));
        e.thread_context(ctx.expect("the main thread's context"))
    };
    *guard(&world.template) = Some(SendEngine(Box::new(template)));
    // The host keeps the kernel table to itself between frames; from here until the first frame
    // too (threads made at start-up wait for it at their first platform call).
    world.kernel.lock(world);
    world.held_from_start.store(true, Ordering::SeqCst);
    for i in 1..e.kernel.threads.list.len() {
        if e.kernel.threads.list[i].suspend_count == 0 {
            start_thread(e, i);
        }
    }
    Ok(())
}

/// The diagnostic that keeps real threads off (R6), if one is on.
fn diagnostic_on(e: &Engine) -> Option<&'static str> {
    if crate::profile::on() {
        return Some("the profile (NV_PROFILE)");
    }
    if e.call_trace().is_some() {
        return Some("an nv-trace record (NV_TRACE_OUT)");
    }
    if e.trace().is_some() {
        return Some("the return trace (NV_MOVE_TRACE)");
    }
    if e.call_log.is_some()
        || e.call_budget().is_some()
        || e.call_counts().is_some()
        || e.call_hook().is_some()
        || e.free_hook.is_some()
    {
        return Some("a per-call diagnostic");
    }
    if crate::census::settings().is_some() {
        return Some("the heap census (NV_CENSUS)");
    }
    if e.mem.diagnostics_on() {
        return Some("a memory diagnostic (NV_TOUCH, NV_WATCH, NV_RACE_CENSUS)");
    }
    if e.kernel.threads.order.is_some() {
        return Some("the thread order (NV_MOVE_THREADS)");
    }
    if crate::queue_check::enabled() {
        return Some("the queue check (NV_QUEUE_CHECK)");
    }
    None
}

/// Starts game thread `i` of the thread host's list on its own OS thread (under the kernel lock:
/// `CreateThread`, `ResumeThread`).
pub(crate) fn start_thread(e: &mut Engine, i: usize) {
    let world = e
        .real
        .as_ref()
        .expect("a real thread starts from a real thread's engine")
        .world;
    let t = &mut e.kernel.threads.list[i];
    t.state = RunState::Running;
    let (id, start, parameter) = (t.id, t.start, t.parameter);
    let engine = SendEngine(Box::new(world.context(id, i)));
    std::thread::Builder::new()
        .name(format!("game thread {id}"))
        .stack_size(THREAD_STACK)
        .spawn(move || run_thread(engine, world, i, start, parameter))
        .expect("a real game thread");
}

/// `ExitThread`'s unwind (no panic message: [`std::panic::resume_unwind`]).
struct ThreadExit(u32);

/// A real game thread's body: `start(parameter)` on its own engine, then its end in the thread
/// host's list.
fn run_thread(engine: SendEngine, world: &'static World, i: usize, start: u32, parameter: u32) {
    let mut e = engine.0;
    GAME_THREAD.with(|g| g.set(true));
    // What was registered since the template was made, replacements included (a call finds only
    // a missing function by itself).
    sync_registered(&mut e);
    world.enter_guest();
    let tls = e.new_tls_block();
    e.swap_tls(tls);
    crate::x87::set_control_word(crate::units::crt::x87_control_word(&mut e));
    let id = e.real.as_ref().map_or(0, |r| r.id);
    let out = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        e.call(start, &[parameter]).eax
    }));
    let state = match out {
        Ok(code) => RunState::Finished { code },
        Err(payload) => match payload.downcast::<ThreadExit>() {
            Ok(exit) => RunState::Finished { code: exit.0 },
            Err(payload) => {
                let text = payload
                    .downcast_ref::<String>()
                    .cloned()
                    .or_else(|| payload.downcast_ref::<&str>().map(|s| s.to_string()))
                    .unwrap_or_default();
                let first = text.lines().next().unwrap_or("").to_string();
                let path: Vec<String> = e
                    .call_stack
                    .iter()
                    .rev()
                    .take(6)
                    .map(|a| format!("{a:08x}"))
                    .collect();
                RunState::Stopped(if path.is_empty() {
                    first
                } else {
                    format!("{first} (in {})", path.join(" < "))
                })
            }
        },
    };
    if let RunState::Stopped(why) = &state {
        dump_threads(world, &format!("game thread {id} stopped: {why}"));
    }
    world.leave_guest();
    world.kernel.lock(world);
    if let RunState::Stopped(why) = &state {
        let line = format!("{why} (in game thread {id}, start {start:08x})");
        eprintln!("real threads: {line}");
        guard(&world.stopped).get_or_insert(line);
        world.any_stop.store(true, Ordering::SeqCst);
        // Wake the main thread if it waits for a section this thread held, or on an object.
        world.wake_all_waiters();
    }
    let t = &mut e.kernel.threads.list[i];
    t.state = state;
    let handle = t.handle;
    crate::thread_host::signal(&mut e);
    // Its handle is signalled now: its waiters go on.
    object_signalled(&e, handle);
    world.kernel.unlock();
}

/// A platform function under the kernel lock: the one registered at the called address
/// ([`Ctx::locked`]), with the thread's last-error code in the kernel table while it runs.
fn locked_call(e: &mut Engine, args: &[u32]) -> Ret {
    let addr = *e
        .call_stack
        .last()
        .expect("a platform call is on the call stack");
    let world = e
        .real
        .as_ref()
        .expect("a locked platform call on a real thread's engine")
        .world;
    world.kernel.lock(world);
    let _held = Held(world);
    park_if_suspended(e);
    if world.late_count.load(Ordering::Acquire) != e.real.as_ref().map_or(0, |r| r.late_seen) {
        sync_registered(e);
    }
    let (f, mine) = {
        let r = e.real.as_ref().expect("checked above");
        let f = *r
            .locked
            .get(&addr)
            .unwrap_or_else(|| panic!("{addr:08x}: no locked platform function"));
        (f, r.last_error)
    };
    e.kernel.last_error = mine;
    let ret = f(e, args);
    let now = e.kernel.last_error;
    if let Some(r) = &mut e.real {
        r.last_error = now;
    }
    ret
}

/// The kernel lock's hold of [`locked_call`], given back however the call ends.
struct Held(&'static World);

impl Drop for Held {
    fn drop(&mut self) {
        self.0.kernel.unlock();
    }
}

/// Whether the platform function at `addr` runs under the device lock ([`device_call`]): one of
/// the layer's `device_calls`.
fn device_addr(addr: u32) -> bool {
    crate::units::platform::layer_device_calls()
        .iter()
        .any(|r| r.contains(&addr))
}

/// A platform function of the Direct3D 9 device or one of its objects: under the device lock,
/// not the kernel lock. Their state (the device model and its forwarding, engine-platform's
/// `Graphics`) is reached only under it: by these functions, and by the kernel-locked ones that
/// reach it through [`device_guard`]. They touch no kernel object and no last-error code. A
/// suspended thread (`SuspendThread`, the game's task threads at quit) parks here before it takes
/// the device lock, as it does at a kernel-locked call; not while it holds the device lock (a
/// device function calling another), where parking would take the kernel lock inside it.
fn device_call(e: &mut Engine, args: &[u32]) -> Ret {
    let addr = *e
        .call_stack
        .last()
        .expect("a platform call is on the call stack");
    let world = e
        .real
        .as_ref()
        .expect("a device platform call on a real thread's engine")
        .world;
    if !world.device.held_here() {
        park_here(e);
    }
    world.device.lock(world);
    let _held = DeviceGuard(world, std::marker::PhantomData);
    if world.late_count.load(Ordering::Acquire) != e.real.as_ref().map_or(0, |r| r.late_seen) {
        sync_registered(e);
    }
    let f = *e
        .real
        .as_ref()
        .expect("checked above")
        .locked
        .get(&addr)
        .unwrap_or_else(|| panic!("{addr:08x}: no device platform function"));
    f(e, args)
}

/// A hold of the device lock, given back when dropped ([`device_guard`], [`device_call`]); not
/// `Send`: the lock is the taking thread's, and only it can give it back.
pub struct DeviceGuard(&'static World, std::marker::PhantomData<*const ()>);

impl Drop for DeviceGuard {
    fn drop(&mut self) {
        self.0.device.unlock();
    }
}

/// The device lock for a platform function outside the device's own that reaches the device's
/// state (the windows the device presents in, D3DX's texture calls asking for the real device,
/// NvAPI's check of an `IDirect3D9`), held until the guard drops; `None` without real threads,
/// where one game thread runs at a time. Re-entrant: a device function that calls back into such
/// code takes it again. Take it inside the kernel lock, never the kernel lock inside it.
pub fn device_guard(e: &Engine) -> Option<DeviceGuard> {
    let world = e.real.as_ref()?.world;
    world.device.lock(world);
    Some(DeviceGuard(world, std::marker::PhantomData))
}

/// Runs `f`, pure work inside a kernel-locked platform function that reaches no platform state
/// and no game memory (D3DX's texture translation, on its own copy of the file), with the kernel
/// lock given back meanwhile, so other threads' platform calls go on; then takes it again with
/// the same holds (also if `f` panics, for the platform function's own release). The thread stays
/// counted as running game code, so a barrier waits for it. Not while this thread holds the device
/// lock, which must never wait for the kernel lock: then `f` runs with both held. With stepped
/// threads it just runs `f`.
pub fn without_kernel_lock<R>(e: &Engine, f: impl FnOnce() -> R) -> R {
    let Some(world) = e.real.as_ref().map(|r| r.world) else {
        return f();
    };
    if world.device.held_here() || !world.kernel.held_here() {
        return f();
    }
    /// Takes the kernel lock back with the holds given up, however `f` ends.
    struct Retake(&'static World, u32);
    impl Drop for Retake {
        fn drop(&mut self) {
            self.0.kernel.take_again(self.0, self.1);
        }
    }
    let _retake = Retake(world, world.kernel.release_all());
    f()
}

/// Whether this thread may reach the device's state: it holds the device lock, or there are no
/// real threads (one game thread runs at a time). For debug assertions in the platform layer.
pub fn device_lock_held(e: &Engine) -> bool {
    e.real.as_ref().map_or(true, |r| r.world.device.held_here())
}

/// `f` registered at `addr` on `e`, as real threads keep it: behind the kernel lock when it is a
/// platform function there.
fn install(e: &mut Engine, addr: u32, f: AbiFn) {
    if let (Some(real), true) = (real_fls(addr), e.real.is_some()) {
        e.register_here(addr, real);
        return;
    }
    let Some(r) = &mut e.real else { return };
    if r.world.locked_addr(addr) {
        r.locked.insert(addr, f);
        let wrapper = if device_addr(addr) {
            device_call
        } else {
            locked_call
        };
        e.register_here(addr, wrapper);
    } else {
        e.register_here(addr, f);
    }
}

/// `Engine::register` on a real thread's engine: kept there (under the kernel lock when it is a
/// platform function) and handed to every other engine ([`sync_registered`]).
pub(crate) fn registered(e: &mut Engine, addr: u32, f: AbiFn) {
    sync_registered(e);
    install(e, addr, f);
    let Some(r) = &mut e.real else { return };
    let mut late = guard(&r.world.late);
    late.push((addr, f));
    r.late_seen = late.len();
    r.world.late_count.store(late.len(), Ordering::Release);
}

/// Takes the functions other engines registered since this one last looked; answers whether
/// there were any.
pub(crate) fn sync_registered(e: &mut Engine) -> bool {
    let Some(r) = &mut e.real else { return false };
    let new: Vec<(u32, AbiFn)> = {
        let late = guard(&r.world.late);
        let new = late[r.late_seen.min(late.len())..].to_vec();
        r.late_seen = late.len();
        new
    };
    let any = !new.is_empty();
    for (addr, f) in new {
        install(e, addr, f);
    }
    any
}

/// The real version of a synchronisation import, by name.
fn sync_import(name: &str) -> Option<AbiFn> {
    Some(match name {
        "InterlockedIncrement" => crate::entry!(0, interlocked_increment(Ptr) -> i32).1,
        "InterlockedDecrement" => crate::entry!(0, interlocked_decrement(Ptr) -> i32).1,
        "InterlockedExchange" => crate::entry!(0, interlocked_exchange(Ptr, i32) -> i32).1,
        "InterlockedExchangeAdd" => crate::entry!(0, interlocked_exchange_add(Ptr, i32) -> i32).1,
        "InterlockedCompareExchange" => {
            crate::entry!(0, interlocked_compare_exchange(Ptr, i32, i32) -> i32).1
        }
        "TlsGetValue" => crate::entry!(0, tls_get_value(u32) -> u32).1,
        "TlsSetValue" => crate::entry!(0, tls_set_value(u32, u32) -> i32).1,
        "GetCurrentThreadId" => crate::entry!(0, get_current_thread_id() -> u32).1,
        "GetLastError" => crate::entry!(0, get_last_error() -> u32).1,
        "SetLastError" => crate::entry!(0, set_last_error(u32)).1,
        "EnterCriticalSection" => crate::entry!(0, enter_critical_section(Ptr)).1,
        "LeaveCriticalSection" => crate::entry!(0, leave_critical_section(Ptr)).1,
        "TryEnterCriticalSection" => crate::entry!(0, try_enter_critical_section(Ptr) -> i32).1,
        "InitializeCriticalSection" => crate::entry!(0, initialize_critical_section(Ptr)).1,
        "DeleteCriticalSection" => crate::entry!(0, delete_critical_section(Ptr)).1,
        "InitializeCriticalSectionAndSpinCount" => {
            crate::entry!(0, initialize_critical_section_and_spin_count(Ptr, u32) -> i32).1
        }
        "Sleep" => crate::entry!(0, sleep(u32)).1,
        _ => return None,
    })
}

fn ctx(e: &Engine) -> &Ctx {
    e.real
        .as_deref()
        .expect("a real synchronisation import on a real thread's engine")
}

fn ctx_mut(e: &mut Engine) -> &mut Ctx {
    e.real
        .as_deref_mut()
        .expect("a real synchronisation import on a real thread's engine")
}

/// An interlocked operation on the word at `a`: `op` on the atomic word when it is 4-aligned and
/// mapped, else `plain` on the word under the lock for words that are not (the access faults as
/// any does when unmapped). A safe point. Answers the word's old value.
fn interlocked(
    e: &Engine,
    a: u32,
    op: impl FnOnce(&AtomicU32) -> u32,
    plain: impl FnOnce(u32) -> Option<u32>,
) -> u32 {
    let w = ctx(e).world;
    w.safe_point();
    park_here(e);
    if let Some(word) = e.mem.atomic_u32(a) {
        return op(word);
    }
    let _g = guard(&w.unaligned);
    let old = e.mem.u32(a);
    if let Some(new) = plain(old) {
        e.mem.set_u32(a, new);
    }
    old
}

// Real threads: replaces KERNEL32.DLL InterlockedIncrement (import)
fn interlocked_increment(e: &mut Engine, target: Ptr) -> i32 {
    let old = interlocked(
        e,
        target.addr(),
        |w| w.fetch_add(1, Ordering::SeqCst),
        |v| Some(v.wrapping_add(1)),
    );
    old.wrapping_add(1) as i32
}

// Real threads: replaces KERNEL32.DLL InterlockedDecrement (import)
fn interlocked_decrement(e: &mut Engine, target: Ptr) -> i32 {
    let old = interlocked(
        e,
        target.addr(),
        |w| w.fetch_sub(1, Ordering::SeqCst),
        |v| Some(v.wrapping_sub(1)),
    );
    old.wrapping_sub(1) as i32
}

// Real threads: replaces KERNEL32.DLL InterlockedExchange (import)
fn interlocked_exchange(e: &mut Engine, target: Ptr, value: i32) -> i32 {
    let value = value as u32;
    interlocked(
        e,
        target.addr(),
        |w| w.swap(value, Ordering::SeqCst),
        |_| Some(value),
    ) as i32
}

// Real threads: replaces KERNEL32.DLL InterlockedExchangeAdd (import)
fn interlocked_exchange_add(e: &mut Engine, target: Ptr, value: i32) -> i32 {
    let value = value as u32;
    interlocked(
        e,
        target.addr(),
        |w| w.fetch_add(value, Ordering::SeqCst),
        |v| Some(v.wrapping_add(value)),
    ) as i32
}

// Real threads: replaces KERNEL32.DLL InterlockedCompareExchange (import)
fn interlocked_compare_exchange(e: &mut Engine, target: Ptr, exchange: i32, comparand: i32) -> i32 {
    let (exchange, comparand) = (exchange as u32, comparand as u32);
    interlocked(
        e,
        target.addr(),
        |w| match w.compare_exchange(comparand, exchange, Ordering::SeqCst, Ordering::SeqCst) {
            Ok(old) | Err(old) => old,
        },
        |v| (v == comparand).then_some(exchange),
    ) as i32
}

/// `TLS_MINIMUM_AVAILABLE` plus the expansion slots, as `units/platform.rs` has them.
const TLS_INDEXES: usize = 1088;

// Real threads: replaces KERNEL32.DLL TlsGetValue (import)
fn tls_get_value(e: &mut Engine, index: u32) -> u32 {
    let r = ctx(e);
    match r.tls.get(index as usize) {
        Some(&(value, generation)) if value != 0 => {
            if r.world.tls_gen[index as usize].load(Ordering::Acquire) == generation {
                value
            } else {
                stale_tls(e, index, value);
                0
            }
        }
        _ => 0,
    }
}

// Real threads: replaces KERNEL32.DLL TlsSetValue (import)
fn tls_set_value(e: &mut Engine, index: u32, value: u32) -> i32 {
    if index as usize >= TLS_INDEXES {
        return 0;
    }
    let r = ctx_mut(e);
    let generation = r.world.tls_gen[index as usize].load(Ordering::Acquire);
    if r.tls.len() <= index as usize {
        r.tls.resize(index as usize + 1, (0, 0));
    }
    r.tls[index as usize] = (value, generation);
    1
}

/// Fiber-local storage indexes (`units/platform/loader.rs`).
const FLS_INDEXES: usize = crate::units::platform::loader::FLS_INDEX_COUNT;

/// The real version of `FlsGetValue` or `FlsSetValue` at `addr` (the code addresses
/// `GetProcAddress` gives them), if it is one of them: answered from the calling thread's own
/// values, off the kernel lock, as TLS is. (The C runtime reads its per-thread data through
/// `FlsGetValue` on every thread; under the kernel lock that was a large part of the game threads'
/// waits for it.)
fn real_fls(addr: u32) -> Option<AbiFn> {
    use crate::units::platform::loader::{FLS_GET_VALUE, FLS_SET_VALUE};
    match addr {
        FLS_GET_VALUE => Some(crate::entry!(0, fls_get_value(u32) -> u32).1),
        FLS_SET_VALUE => Some(crate::entry!(0, fls_set_value(u32, u32) -> i32).1),
        _ => None,
    }
}

/// `FlsAlloc` (`in_use`) or `FlsFree` of `index` (the platform's, under the kernel lock): with
/// real threads the index's value on every thread reads 0 from now on, and `FlsSetValue` takes
/// it only while allocated (as Windows, and the stepped host's table).
pub(crate) fn fls_index_changed(e: &Engine, index: u32, in_use: bool) {
    if let Some(r) = &e.real {
        let w = r.world;
        if let (Some(g), Some(u)) = (
            w.fls_gen.get(index as usize),
            w.fls_in_use.get(index as usize),
        ) {
            g.fetch_add(1, Ordering::AcqRel);
            u.store(in_use, Ordering::Release);
        }
    }
}

// Real threads: replaces KERNEL32.DLL FlsGetValue (found by name)
fn fls_get_value(e: &mut Engine, index: u32) -> u32 {
    let r = ctx(e);
    match (
        r.fls.get(index as usize),
        r.world.fls_gen.get(index as usize),
    ) {
        (Some(&(value, generation)), Some(g)) if g.load(Ordering::Acquire) == generation => value,
        _ => 0,
    }
}

// Real threads: replaces KERNEL32.DLL FlsSetValue (found by name)
fn fls_set_value(e: &mut Engine, index: u32, value: u32) -> i32 {
    let r = ctx_mut(e);
    let Some(generation) = r
        .world
        .fls_in_use
        .get(index as usize)
        .filter(|u| u.load(Ordering::Acquire))
        .map(|_| r.world.fls_gen[index as usize].load(Ordering::Acquire))
    else {
        return 0;
    };
    if r.fls.len() <= index as usize {
        r.fls.resize(index as usize + 1, (0, 0));
    }
    r.fls[index as usize] = (value, generation);
    1
}

// Real threads: replaces KERNEL32.DLL GetCurrentThreadId (import)

fn get_current_thread_id(e: &mut Engine) -> u32 {
    ctx(e).id
}

// Real threads: replaces KERNEL32.DLL GetLastError (import)
fn get_last_error(e: &mut Engine) -> u32 {
    ctx(e).last_error
}

// Real threads: replaces KERNEL32.DLL SetLastError (import)
fn set_last_error(e: &mut Engine, code: u32) {
    ctx_mut(e).last_error = code;
}

/// The spin count a section uses at most before it parks ([`enter_critical_section`]).
const SECTION_SPIN_MAX: u32 = 4000;

/// A critical section's own words in its guest `RTL_CRITICAL_SECTION`, as Windows keeps them there:
/// the owner (`OwningThread`, +0xc: a thread id, 0 when free) and the threads waiting for it
/// (+4, where Windows keeps its `LockCount`), as atomics; the holds (`RecursionCount`, +8) and the
/// spin count (`SpinCount`, +0x14) are plain words only the owner (or the initialiser) writes.
/// `None` for a section not 4-aligned (the shared table then keeps it: [`World::sections`]).
fn section_words(e: &Engine, a: u32) -> Option<(&AtomicU32, &AtomicU32)> {
    Some((
        e.mem.atomic_u32(a.wrapping_add(0xc))?,
        e.mem.atomic_u32(a.wrapping_add(4))?,
    ))
}

/// Tries to make `me` the owner of a free section: one compare-exchange on its owner word.
fn take_section(owner: &AtomicU32, me: u32) -> bool {
    owner.load(Ordering::Relaxed) == 0
        && owner
            .compare_exchange(0, me, Ordering::Acquire, Ordering::Relaxed)
            .is_ok()
}

// Real threads: replaces KERNEL32.DLL EnterCriticalSection (import)
/// Takes the section for the calling thread (again, counted, when it holds it), as Windows does:
/// one compare-exchange on the section's own owner word when it is free (no shared lock), else
/// its spin count of tries, then the thread parks until the owner's leave wakes it. The kernel
/// lock, if this thread holds it, is given back while it waits.
fn enter_critical_section(e: &mut Engine, section: Ptr) {
    let r = ctx(e);
    let (w, me) = (r.world, r.id);
    w.safe_point();
    park_here(e);
    let a = section.addr();
    let Some((owner, waiting)) = section_words(e, a) else {
        return enter_section_in_table(e, section);
    };
    let count_at = a.wrapping_add(8);
    if owner.load(Ordering::Relaxed) == me {
        e.mem.set_u32(count_at, e.mem.u32(count_at).wrapping_add(1));
        return;
    }
    if take_section(owner, me) {
        e.mem.set_u32(count_at, 1);
        return;
    }
    // The spin count's low 24 bits, capped; Windows' dynamic-spin flag (`0x02000000`, which
    // `DEFAULT_SECTION_SPIN` carries) is not followed: Windows tunes the count from how often
    // spinning paid off, in ntdll code that is not the game's and not documented, so the count it
    // starts with is kept. No spin while this thread holds the kernel lock (inside a platform
    // call): it would hold up every other thread's platform calls meanwhile, so it parks at once,
    // giving the lock back.
    let spins = if w.kernel.held_here() {
        0
    } else {
        (e.mem.u32(a.wrapping_add(0x14)) & 0x00ff_ffff).min(SECTION_SPIN_MAX)
    };
    for _ in 0..spins {
        std::hint::spin_loop();
        if take_section(owner, me) {
            e.mem.set_u32(count_at, 1);
            return;
        }
    }
    note_state(e, || {
        format!(
            "waiting for critical section {a:08x} (held by thread {})",
            owner.load(Ordering::Relaxed)
        )
    });
    let held = w.kernel.release_all();
    let was = w.leave_guest();
    let started = lock_stats().map(|_| std::time::Instant::now());
    // Counted, then listed, then the owner word tried: a leave that frees it after the count sees
    // the count and wakes the listed (`leave_critical_section`), so no wake is missed.
    waiting.fetch_add(1, Ordering::SeqCst);
    let parker = std::sync::Arc::new(Waiter {
        thread: std::thread::current(),
        handles: vec![a],
        woken: AtomicBool::new(false),
    });
    guard(&w.section_parkers).push(std::sync::Arc::clone(&parker));
    // The main thread ends the run when a game thread stopped (it may have stopped holding this
    // section, which then never comes free): the stop wakes the section's waiters.
    let mut stop = None;
    let mut woken = false;
    loop {
        if ctx(e).suspend.load(Ordering::Relaxed) {
            // Suspended meanwhile: it parks without taking the section.
            park_here(e);
        }
        parker.woken.store(false, Ordering::SeqCst);
        // Ordered before the owner word's load below (a Relaxed load may otherwise be read
        // before the store): a leave that read `woken` as still set, and so woke another
        // waiter, freed the section before this fence, and the load sees it free.
        std::sync::atomic::fence(Ordering::SeqCst);
        if take_section(owner, me) {
            break;
        }
        if woken {
            if let Some(stats) = lock_stats() {
                stats.spurious.fetch_add(1, Ordering::Relaxed);
            }
        }
        woken = true;
        if me == MAIN_THREAD_ID && w.any_stop.load(Ordering::SeqCst) {
            stop = guard(&w.stopped).clone();
            break;
        }
        if !parker.woken.load(Ordering::SeqCst) {
            std::thread::park();
        }
    }
    waiting.fetch_sub(1, Ordering::SeqCst);
    guard(&w.section_parkers).retain(|o| !std::sync::Arc::ptr_eq(o, &parker));
    if stop.is_none() {
        e.mem.set_u32(count_at, 1);
    }
    if let (Some(stats), Some(t)) = (lock_stats(), started) {
        stats.section[role()].add(t.elapsed());
    }
    if was {
        w.enter_guest();
    }
    w.kernel.take_again(w, held);
    note_state(e, String::new);
    if let Some(why) = stop {
        panic!("{why}");
    }
}

// Real threads: replaces KERNEL32.DLL LeaveCriticalSection (import)
/// Counts a hold down (as on Windows and the stepped host, whoever calls it); at 0 the section is
/// free, and one thread waiting for it wakes to try for it again, as Windows wakes one: the first
/// not woken already (one woken that has not tried yet will try anyway). A waiter that loses the
/// section to another thread parks again, and that thread's leave wakes the next.
fn leave_critical_section(e: &mut Engine, section: Ptr) {
    let w = ctx(e).world;
    let a = section.addr();
    let Some((owner, waiting)) = section_words(e, a) else {
        return leave_section_in_table(e, section);
    };
    let count_at = a.wrapping_add(8);
    let count = e.mem.u32(count_at);
    if count == 0 {
        return;
    }
    e.mem.set_u32(count_at, count - 1);
    if count == 1 {
        owner.store(0, Ordering::SeqCst);
        if waiting.load(Ordering::SeqCst) > 0 {
            if let Some(stats) = lock_stats() {
                stats.notifies.fetch_add(1, Ordering::Relaxed);
            }
            let parkers = guard(&w.section_parkers);
            if let Some(o) = parkers
                .iter()
                .find(|o| o.handles[0] == a && !o.woken.load(Ordering::SeqCst))
            {
                o.woken.store(true, Ordering::SeqCst);
                o.thread.unpark();
            }
        }
    }
}

// Real threads: replaces KERNEL32.DLL TryEnterCriticalSection (import)
fn try_enter_critical_section(e: &mut Engine, section: Ptr) -> i32 {
    let me = ctx(e).id;
    let a = section.addr();
    let Some((owner, _)) = section_words(e, a) else {
        return try_enter_section_in_table(e, section);
    };
    let count_at = a.wrapping_add(8);
    if owner.load(Ordering::Relaxed) == me {
        e.mem.set_u32(count_at, e.mem.u32(count_at).wrapping_add(1));
        return 1;
    }
    if take_section(owner, me) {
        e.mem.set_u32(count_at, 1);
        return 1;
    }
    0
}

/// A section's words made free with `spin` as its spin count (`InitializeCriticalSection*`,
/// `DeleteCriticalSection`).
fn section_reset(e: &mut Engine, section: Ptr, spin: u32) {
    let a = section.addr();
    match section_words(e, a) {
        Some((owner, waiting)) => {
            owner.store(0, Ordering::SeqCst);
            waiting.store(0, Ordering::SeqCst);
            e.mem.set_u32(a.wrapping_add(8), 0);
            e.mem.set_u32(a.wrapping_add(0x14), spin);
        }
        None => {
            guard(&ctx(e).world.sections).remove(&a);
        }
    }
}

/// The spin word Windows writes for a section given no spin count of its own
/// (`InitializeCriticalSection`, or `InitializeCriticalSectionAndSpinCount` with 0): 2000 tries,
/// with its dynamic-spin flag in the top byte. Measured on Windows 11, 32- and 64-bit
/// (measure/real-threads/cs-spin).
const DEFAULT_SECTION_SPIN: u32 = 0x0200_07d0;

// Real threads: replaces KERNEL32.DLL InitializeCriticalSection (import)
fn initialize_critical_section(e: &mut Engine, section: Ptr) {
    section_reset(e, section, DEFAULT_SECTION_SPIN);
}

// Real threads: replaces KERNEL32.DLL DeleteCriticalSection (import)
fn delete_critical_section(e: &mut Engine, section: Ptr) {
    section_reset(e, section, 0);
}

// Real threads: replaces KERNEL32.DLL InitializeCriticalSectionAndSpinCount (import)
fn initialize_critical_section_and_spin_count(e: &mut Engine, section: Ptr, spin: u32) -> i32 {
    let spin = if spin == 0 {
        DEFAULT_SECTION_SPIN
    } else {
        spin
    };
    section_reset(e, section, spin);
    1
}

/// [`enter_critical_section`] for a section not 4-aligned: the shared table ([`World::sections`]),
/// waiting by sleeping 0 ms (the exe's sections are aligned; this is a fallback).
fn enter_section_in_table(e: &mut Engine, section: Ptr) {
    let r = ctx(e);
    let (w, me) = (r.world, r.id);
    loop {
        {
            let mut table = guard(&w.sections);
            let s = table.entry(section.addr()).or_default();
            if s.owner == 0 || s.owner == me {
                s.owner = me;
                s.count += 1;
                return;
            }
        }
        real_sleep(w, 0);
        main_stop_check(e);
    }
}

fn leave_section_in_table(e: &mut Engine, section: Ptr) {
    let w = ctx(e).world;
    let mut table = guard(&w.sections);
    if let Some(s) = table.get_mut(&section.addr()) {
        if s.count > 0 {
            s.count -= 1;
            if s.count == 0 {
                s.owner = 0;
            }
        }
    }
}

fn try_enter_section_in_table(e: &mut Engine, section: Ptr) -> i32 {
    let r = ctx(e);
    let (w, me) = (r.world, r.id);
    let mut table = guard(&w.sections);
    let s = table.entry(section.addr()).or_default();
    if s.owner == 0 || s.owner == me {
        s.owner = me;
        s.count += 1;
        1
    } else {
        0
    }
}

// Real threads: replaces KERNEL32.DLL Sleep (import)
/// Sleeps for real (`Sleep(0)` gives the rest of the time slice up); a safe point.
fn sleep(e: &mut Engine, ms: u32) {
    real_sleep(ctx(e).world, ms);
    park_here(e);
    main_stop_check(e);
}

/// `SuspendThread` and `ResumeThread` with real threads (`units/platform.rs`, under the kernel
/// lock): whether any game thread that has run is now suspended, and a change, so a parked thread
/// looks at its count again.
pub(crate) fn suspends_changed(e: &mut Engine) {
    let Some(w) = e.real.as_ref().map(|r| r.world) else {
        return;
    };
    for (i, flag) in guard(&w.suspend_flags).iter() {
        let on = e.kernel.threads.list.get(*i).is_some_and(|t| {
            t.suspend_count > 0
                && !matches!(
                    t.state,
                    RunState::NotStarted | RunState::Finished { .. } | RunState::Stopped(_)
                )
        });
        flag.store(on, Ordering::SeqCst);
    }
    crate::thread_host::signal(e);
}

/// Under the kernel lock: while this game thread's suspend count is above 0 (`SuspendThread`),
/// it waits, not running game code, until `ResumeThread` brings it to 0. Windows stops a
/// suspended thread wherever it is; here it stops at its next platform call, wait or sleep (the
/// game suspends its task threads to end them at quit, `00c42f00`, then closes their handles).
fn park_if_suspended(e: &Engine) {
    let Some(r) = &e.real else { return };
    let (w, i, id) = (r.world, r.index, r.id);
    if i == 0 || !r.suspend.load(Ordering::Relaxed) {
        return;
    }
    let mut parked = false;
    while e
        .kernel
        .threads
        .list
        .get(i)
        .is_some_and(|t| t.suspend_count > 0)
    {
        if !parked {
            parked = true;
            note_state(e, || "suspended (SuspendThread)".to_string());
            // The suspend path ran (a line for the first few: the game suspends its task
            // threads at quit).
            if w.parks.fetch_add(1, Ordering::Relaxed) < PARK_REPORTS {
                eprintln!("real threads: game thread {id} suspended (SuspendThread), parked");
            }
            w.parked.fetch_add(1, Ordering::SeqCst);
        }
        w.kernel.wait_change(w, None);
    }
    if parked {
        w.parked.fetch_sub(1, Ordering::SeqCst);
        note_state(e, String::new);
    }
}

/// `SuspendThread` with real threads (`units/platform.rs`): a line for the first few calls, so a
/// run shows the suspend path ran (the game suspends its task threads at quit, `00c42f00`); the
/// thread then parks at its next platform call, wait or sleep ([`park_if_suspended`]).
pub(crate) fn suspend_called(e: &Engine, handle: u32, count: u32) {
    let Some(r) = &e.real else { return };
    if r.world.suspends.fetch_add(1, Ordering::Relaxed) < PARK_REPORTS {
        let id = e
            .kernel
            .threads
            .list
            .iter()
            .find(|t| t.handle == handle)
            .map_or(0, |t| t.id);
        eprintln!(
            "real threads: SuspendThread on game thread {id} (handle {handle:#x}, count {count}) by thread {}",
            r.id
        );
    }
}

/// How many park lines [`park_if_suspended`] prints at most.
const PARK_REPORTS: u32 = 16;

/// Not under the kernel lock (an interlocked operation, a section enter, after a sleep): parks
/// the thread if it was suspended. One load when it is not.
fn park_here(e: &Engine) {
    let Some(r) = &e.real else { return };
    let w = r.world;
    if r.suspend.load(Ordering::Relaxed) {
        w.kernel.lock(w);
        park_if_suspended(e);
        w.kernel.unlock();
    }
}

/// On the main thread: a game thread that stopped ends the run now (main's next frame end would,
/// but a loop waiting for that thread, such as a spin lock it held, never gets there).
fn main_stop_check(e: &Engine) {
    if let Some(r) = &e.real {
        if r.id == MAIN_THREAD_ID && r.world.any_stop.load(Ordering::Relaxed) {
            if let Some(why) = guard(&r.world.stopped).clone() {
                panic!("{why}");
            }
        }
    }
}

fn real_sleep(w: &World, ms: u32) {
    let started = lock_stats().map(|_| std::time::Instant::now());
    real_sleep_here(w, ms);
    if let (Some(stats), Some(t)) = (lock_stats(), started) {
        stats.sleep[role()].add(t.elapsed());
    }
}

fn real_sleep_here(w: &World, ms: u32) {
    w.safe_point();
    let held = w.kernel.release_all();
    let was = w.leave_guest();
    if ms == 0 {
        std::thread::yield_now();
    } else {
        std::thread::sleep(Duration::from_millis(u64::from(ms)));
    }
    if was {
        w.enter_guest();
    }
    w.kernel.take_again(w, held);
}

/// The real threads' host (`thread_host::host_for`): waits block for real.
pub struct Real;

impl ThreadHost for Real {
    /// Under the kernel lock (the wait imports hold it): until the objects are signalled, the
    /// thread parks, and only a change of one of its objects wakes it ([`World::wait_for_objects`],
    /// [`object_signalled`]), as Windows wakes a thread blocked in `WaitForSingleObject`; a timed wait
    /// ends at its deadline. On the main thread a game thread's stop ends the wait (the run stops).
    fn block(
        &self,
        e: &mut Engine,
        handles: &[u32],
        all: bool,
        deadline: Option<u64>,
        signalled: Signalled,
    ) -> bool {
        let w = ctx(e).world;
        let i = e.thread_index();
        let ready = |e: &Engine| {
            if all {
                handles.iter().all(|&h| signalled(e, h))
            } else {
                handles.iter().any(|&h| signalled(e, h))
            }
        };
        let mut waiting = false;
        let mut started = None;
        let mut waiter: Option<std::sync::Arc<Waiter>> = None;
        let ready = loop {
            park_if_suspended(e);
            if ready(e) {
                break true;
            }
            let now = e.clock_now_us();
            if deadline.is_some_and(|d| now >= d) {
                break false;
            }
            if i == 0 {
                if let Some(why) = guard(&w.stopped).clone() {
                    panic!("{why}");
                }
            }
            if !waiting {
                waiting = true;
                started = lock_stats().map(|_| std::time::Instant::now());
                note_state(e, || {
                    let list: Vec<String> = handles.iter().map(|h| format!("{h:x}")).collect();
                    format!(
                        "waiting on {} of handles [{}]",
                        if all { "all" } else { "any" },
                        list.join(", ")
                    )
                });
                if let Some(t) = e.kernel.threads.list.get_mut(i) {
                    t.state = RunState::Waiting {
                        handles: handles.to_vec(),
                        all,
                        deadline,
                    };
                }
            }
            let timeout = deadline.map(|d| Duration::from_micros(d - now));
            let me = waiter.get_or_insert_with(|| {
                let me = std::sync::Arc::new(Waiter {
                    thread: std::thread::current(),
                    handles: handles.to_vec(),
                    woken: AtomicBool::new(false),
                });
                guard(&w.waiters).push(std::sync::Arc::clone(&me));
                me
            });
            w.wait_for_objects(me, timeout);
        };
        if let Some(me) = waiter {
            guard(&w.waiters).retain(|o| !std::sync::Arc::ptr_eq(o, &me));
        }
        if let (Some(stats), Some(t)) = (lock_stats(), started) {
            stats.object[role()].add(t.elapsed());
        }
        if waiting {
            note_state(e, String::new);
            if let Some(t) = e.kernel.threads.list.get_mut(i) {
                t.state = RunState::Running;
            }
        }
        ready
    }

    fn sleep(&self, e: &mut Engine, ms: u32, _signalled: Signalled) {
        real_sleep(ctx(e).world, ms);
        park_here(e);
        main_stop_check(e);
    }

    fn exit(&self, e: &mut Engine, code: u32) {
        assert!(
            !crate::thread_host::on_main(e),
            "ExitThread on the main thread ends the process (not provided)"
        );
        std::panic::resume_unwind(Box::new(ThreadExit(code)));
    }
}

/// `TlsAlloc` or `TlsFree` of `index` (the platform's, under the kernel lock): with real threads
/// the index's value on every thread reads 0 from now on, as Windows clears it on every thread
/// (the stepped host's table forgets them, `units/platform.rs`). The lock-free maps
/// (`0044dfc0`) keep each thread's accessor in an index of their own and free it with the map,
/// so a later map that gets the same index must not find the old map's freed accessor.
pub(crate) fn tls_index_reset(e: &Engine, index: u32) {
    if let Some(r) = &e.real {
        if let Some(generation) = r.world.tls_gen.get(index as usize) {
            generation.fetch_add(1, Ordering::AcqRel);
        }
    }
}

/// How many words of a thread's last calls [`ThreadRecord`] keeps, and of its call stack.
const RECORD_CALLS: usize = 32;
const RECORD_STACK: usize = 64;

/// `NV_REAL_DUMP` (a diagnostic, set to anything): every game thread keeps a record of what it
/// does, printed for all threads when one stops ([`dump_threads`]); with it, a value a thread
/// read from a freed thread-local storage index is reported ([`stale_tls`]). Read once.
fn dump_from_env() -> bool {
    static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ON.get_or_init(|| std::env::var_os("NV_REAL_DUMP").is_some())
}

/// The records of every game thread ([`World::dump`]).
#[derive(Default)]
struct Dump {
    records: Mutex<Vec<std::sync::Arc<ThreadRecord>>>,
    /// Dumps printed (at most [`DUMPS`]) and stale values reported (at most [`STALE_REPORTS`]).
    dumps: AtomicU32,
    stale: AtomicU32,
}

const DUMPS: u32 = 3;
const STALE_REPORTS: u32 = 40;

/// What one game thread is doing, written by it and read by [`dump_threads`] on any thread: its
/// last calls (the functions it entered, oldest overwritten), its call stack, the kernel lock
/// token of the OS thread it last ran on and what it waits for.
struct ThreadRecord {
    id: u32,
    calls: [AtomicU32; RECORD_CALLS],
    count: AtomicU32,
    stack: [AtomicU32; RECORD_STACK],
    depth: AtomicU32,
    token: AtomicU64,
    state: Mutex<String>,
}

impl ThreadRecord {
    fn new(id: u32) -> Self {
        ThreadRecord {
            id,
            calls: std::array::from_fn(|_| AtomicU32::new(0)),
            count: AtomicU32::new(0),
            stack: std::array::from_fn(|_| AtomicU32::new(0)),
            depth: AtomicU32::new(0),
            token: AtomicU64::new(0),
            state: Mutex::new(String::new()),
        }
    }
}

/// `NV_REAL_DUMP`'s call hook (`Engine::call_hook`): a call entered (its function goes on the
/// record's last calls and call stack) or left.
fn record_call(e: &mut Engine, addr: u32, leaving: bool) {
    let Some(record) = e.real.as_ref().and_then(|r| r.record.as_deref()) else {
        return;
    };
    let depth = e.call_stack.len();
    if leaving {
        record.depth.store(depth as u32, Ordering::Relaxed);
        return;
    }
    let n = record.count.fetch_add(1, Ordering::Relaxed);
    record.calls[n as usize % RECORD_CALLS].store(addr, Ordering::Relaxed);
    if depth < RECORD_STACK {
        record.stack[depth].store(addr, Ordering::Relaxed);
    }
    record.depth.store(depth as u32 + 1, Ordering::Relaxed);
    record.token.store(token(), Ordering::Relaxed);
}

/// `NV_REAL_DUMP`: what the thread of `e` waits for from now on (empty: nothing).
fn note_state(e: &Engine, state: impl FnOnce() -> String) {
    if let Some(record) = e.real.as_ref().and_then(|r| r.record.as_deref()) {
        *guard(&record.state) = state();
    }
}

/// `NV_REAL_DUMP`: a thread read a thread-local storage value set before its index was freed or
/// handed out again (it reads 0, as on Windows): reported with the block the old value pointed
/// at, when the freed-block check knows it (`NV_BLOCK_CHECK=...,freed`).
#[cold]
fn stale_tls(e: &Engine, index: u32, value: u32) {
    let Some(r) = &e.real else { return };
    let Some(dump) = &r.world.dump else { return };
    if dump.stale.fetch_add(1, Ordering::Relaxed) >= STALE_REPORTS {
        return;
    }
    let block = e
        .mem
        .freed_block_info(value)
        .map_or_else(String::new, |b| format!("; {b}"));
    let at: Vec<String> = e
        .call_stack
        .iter()
        .rev()
        .take(6)
        .map(|a| format!("{a:08x}"))
        .collect();
    eprintln!(
        "real threads: thread {} read TLS index {index} after it was freed or handed out again \
         (in {}): its old value {value:08x} reads 0{block}",
        r.id,
        at.join(" < ")
    );
}

/// `NV_REAL_DUMP`: prints every game thread's record (the function it is in, its call stack's
/// top, its last calls, what it waits for), the critical sections each holds and the kernel
/// lock's holder, headed by `why`. At most [`DUMPS`] times a run.
pub(crate) fn dump_threads(w: &World, why: &str) {
    let Some(dump) = &w.dump else { return };
    if dump.dumps.fetch_add(1, Ordering::Relaxed) >= DUMPS {
        return;
    }
    let hex = |v: &[u32]| {
        v.iter()
            .map(|a| format!("{a:08x}"))
            .collect::<Vec<_>>()
            .join(" ")
    };
    let owner = w.kernel.owner.load(Ordering::Relaxed);
    let sections: Vec<(u32, Section)> = lock_briefly(&w.sections)
        .map(|table| table.iter().map(|(&a, &s)| (a, s)).collect())
        .unwrap_or_default();
    let records: Vec<std::sync::Arc<ThreadRecord>> = lock_briefly(&dump.records)
        .map(|list| list.clone())
        .unwrap_or_default();
    let mut out = format!("real threads: dump ({why})\n");
    for r in &records {
        let depth = r.depth.load(Ordering::Relaxed) as usize;
        let stack: Vec<u32> = (0..depth.min(RECORD_STACK))
            .rev()
            .take(12)
            .map(|i| r.stack[i].load(Ordering::Relaxed))
            .collect();
        let n = r.count.load(Ordering::Relaxed) as usize;
        let calls: Vec<u32> = (n.saturating_sub(RECORD_CALLS)..n)
            .map(|k| r.calls[k % RECORD_CALLS].load(Ordering::Relaxed))
            .collect();
        let held: Vec<String> = sections
            .iter()
            .filter(|(_, s)| s.owner == r.id && s.count > 0)
            .map(|(a, s)| format!("{a:08x} x{}", s.count))
            .collect();
        let state = lock_briefly(&r.state)
            .map(|s| s.clone())
            .unwrap_or_default();
        let kernel = if owner != 0 && owner == r.token.load(Ordering::Relaxed) {
            ", holds the kernel lock"
        } else {
            ""
        };
        out.push_str(&format!(
            "  thread {}: in {} (depth {depth}){kernel}; {}; critical sections held: [{}]\n    \
             call stack, innermost first: {}\n    last {} calls, oldest first: {}\n",
            r.id,
            stack
                .first()
                .map_or_else(|| "-".to_string(), |a| format!("{a:08x}")),
            if state.is_empty() { "running" } else { &state },
            held.join(", "),
            hex(&stack),
            calls.len(),
            hex(&calls)
        ));
    }
    eprint!("{out}");
}

/// `m` locked, waiting up to 100 ms for it ([`dump_threads`] runs on a stopping thread: a lock
/// held by a thread that is stuck must not stop the dump); `None` when it stays held.
fn lock_briefly<T>(m: &Mutex<T>) -> Option<MutexGuard<'_, T>> {
    for _ in 0..100 {
        match m.try_lock() {
            Ok(g) => return Some(g),
            Err(std::sync::TryLockError::Poisoned(p)) => return Some(p.into_inner()),
            Err(std::sync::TryLockError::WouldBlock) => {
                std::thread::sleep(Duration::from_millis(1));
            }
        }
    }
    None
}

/// The main thread stopped (`thread_host`'s main coroutine): [`dump_threads`] when the dump is on.
pub(crate) fn main_stopped(e: &Engine, why: &str) {
    if let Some(r) = &e.real {
        dump_threads(r.world, &format!("main stopped: {why}"));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::args;

    /// Import slots of a test engine, by name (the order of `units::platform::imports`).
    fn engine_with_imports() -> (Engine, impl Fn(&str) -> u32) {
        let mut e = Engine::new();
        let names: Vec<&'static str> = crate::units::platform::imports()
            .iter()
            .map(|(_, n, _)| *n)
            .collect();
        let slots: Vec<(u32, &'static str)> = names
            .iter()
            .enumerate()
            .map(|(i, n)| (0x00fd_f000 + 4 * i as u32, *n))
            .collect();
        e.mem.map(0x00fd_f000, 0x2000);
        let imports: Vec<crate::exe::Import> = crate::units::platform::imports()
            .iter()
            .enumerate()
            .map(|(i, (dll, name, _))| crate::exe::Import {
                dll: dll.to_string(),
                name: name.to_string(),
                slot: 0x00fd_f000 + 4 * i as u32,
            })
            .collect();
        e.register_imports(&imports);
        e.start_real_clock();
        let slot = move |name: &str| {
            slots
                .iter()
                .find(|(_, n)| *n == name)
                .unwrap_or_else(|| panic!("no import {name}"))
                .0
        };
        (e, slot)
    }

    /// A game thread's start: `count` (word at +0) increments of the word at `param + 4` with
    /// `InterlockedIncrement` and `count` plain increments under the critical section at
    /// `param + 0x60` (its 24 bytes clear of the other words), the word at `param + 0x40`; then it
    /// ends with the thread's id.
    fn worker(e: &mut Engine, a: &[u32]) -> Ret {
        let p = a[0];
        let n = e.mem.u32(p);
        let inc = e.mem.u32(p + 0x10);
        let enter = e.mem.u32(p + 0x14);
        let leave = e.mem.u32(p + 0x18);
        let id = e.mem.u32(p + 0x1c);
        for _ in 0..n {
            e.call(inc, &args![p + 4]);
            e.call(enter, &args![p + 0x60]);
            let v = e.mem.u32(p + 0x40);
            std::hint::black_box(());
            e.mem.set_u32(p + 0x40, v + 1);
            e.call(leave, &args![p + 0x60]);
        }
        Ret {
            eax: e.call(id, &[]).eax,
            ..Ret::default()
        }
    }

    /// Real threads count with real atomics and real critical sections: four threads, each 20000
    /// increments of each kind, lose none; the main thread waits on their handles for real.
    #[test]
    fn real_threads_count_without_losing_increments() {
        let (mut e, slot) = engine_with_imports();
        start(&mut e).expect("real threads on");
        e.mem.map(0x0040_0000, 0x1000);
        e.register(0x0040_0000, worker);
        let p = e.mem.alloc(0x100);
        e.mem.set_u32(p, 20_000);
        e.mem.set_u32(p + 0x10, slot("InterlockedIncrement"));
        e.mem.set_u32(p + 0x14, slot("EnterCriticalSection"));
        e.mem.set_u32(p + 0x18, slot("LeaveCriticalSection"));
        e.mem.set_u32(p + 0x1c, slot("GetCurrentThreadId"));
        let handles = e.mem.alloc(16);
        for k in 0..4 {
            let h = e
                .call(
                    slot("CreateThread"),
                    &args![0u32, 0u32, 0x0040_0000u32, p, 0u32, 0u32],
                )
                .eax;
            assert_ne!(h, 0);
            e.mem.set_u32(handles + 4 * k, h);
        }
        let waited = e
            .call(
                slot("WaitForMultipleObjects"),
                &args![4u32, handles, 1u32, 0xffff_ffffu32],
            )
            .eax;
        assert_eq!(waited, 0, "every thread ended");
        assert_eq!(e.mem.u32(p + 4), 80_000, "interlocked increments");
        assert_eq!(e.mem.u32(p + 0x40), 80_000, "increments under the section");
        let code = e.mem.alloc(4);
        let h = e.mem.u32(handles + 8);
        e.call(slot("GetExitCodeThread"), &args![h, code]);
        assert_eq!(e.mem.u32(code), 4, "the third thread made is id 4");
    }

    /// Events and semaphores block for real: a thread waits on an event the main thread sets,
    /// then releases a semaphore main waits on.
    #[test]
    fn real_waits_block_until_signalled() {
        fn ping(e: &mut Engine, a: &[u32]) -> Ret {
            let p = a[0];
            let (wait, release) = (e.mem.u32(p), e.mem.u32(p + 4));
            let (event, semaphore) = (e.mem.u32(p + 8), e.mem.u32(p + 12));
            e.call(wait, &args![event, 0xffff_ffffu32]);
            e.mem.set_u32(p + 16, 0x5a5a);
            e.call(release, &args![semaphore, 1u32, 0u32]);
            Ret::default()
        }
        let (mut e, slot) = engine_with_imports();
        start(&mut e).expect("real threads on");
        e.register(0x0040_1000, ping);
        let p = e.mem.alloc(0x20);
        let event = e
            .call(slot("CreateEventA"), &args![0u32, 0u32, 0u32, 0u32])
            .eax;
        let semaphore = e
            .call(slot("CreateSemaphoreA"), &args![0u32, 0u32, 1u32, 0u32])
            .eax;
        e.mem.set_u32(p, slot("WaitForSingleObject"));
        e.mem.set_u32(p + 4, slot("ReleaseSemaphore"));
        e.mem.set_u32(p + 8, event);
        e.mem.set_u32(p + 12, semaphore);
        e.call(
            slot("CreateThread"),
            &args![0u32, 0u32, 0x0040_1000u32, p, 0u32, 0u32],
        );
        // The thread is blocked on the event: a timed wait on the semaphore times out.
        let wait = slot("WaitForSingleObject");
        assert_eq!(e.call(wait, &args![semaphore, 20u32]).eax, 0x102);
        assert_eq!(e.mem.u32(p + 16), 0);
        e.call(slot("SetEvent"), &args![event]);
        assert_eq!(e.call(wait, &args![semaphore, 0xffff_ffffu32]).eax, 0);
        assert_eq!(e.mem.u32(p + 16), 0x5a5a);
    }

    /// Work outside the kernel lock: another thread takes the lock while it runs, and the holds
    /// come back after it (also after a panic); with the device lock held it stays held.
    #[test]
    fn work_without_the_kernel_lock_lets_others_take_it() {
        let (mut e, _) = engine_with_imports();
        start(&mut e).expect("real threads on");
        let world = e.real.as_ref().expect("real threads").world;
        world.kernel.lock(world);
        assert_eq!(world.kernel.depth.load(Ordering::Relaxed), 2);
        let other = without_kernel_lock(&e, || {
            assert!(!world.kernel.held_here());
            std::thread::spawn(move || {
                world.kernel.lock(world);
                world.kernel.unlock();
                true
            })
            .join()
            .expect("the other thread")
        });
        assert!(other, "another thread took the kernel lock meanwhile");
        assert!(world.kernel.held_here());
        assert_eq!(world.kernel.depth.load(Ordering::Relaxed), 2);
        let caught = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            without_kernel_lock(&e, || panic!("in the work"))
        }));
        assert!(caught.is_err());
        assert!(world.kernel.held_here(), "taken back after a panic");
        assert_eq!(world.kernel.depth.load(Ordering::Relaxed), 2);
        world.device.lock(world);
        without_kernel_lock(&e, || {
            assert!(world.kernel.held_here(), "kept under the device lock")
        });
        world.device.unlock();
        world.kernel.unlock();
    }

    /// Each thread has its own TLS values, thread id and last error; the heap serves every thread.
    #[test]
    fn per_thread_state_stays_per_thread() {
        fn own(e: &mut Engine, a: &[u32]) -> Ret {
            let p = a[0];
            let (get, set) = (e.mem.u32(p), e.mem.u32(p + 4));
            let (id, set_error, get_error) =
                (e.mem.u32(p + 8), e.mem.u32(p + 12), e.mem.u32(p + 16));
            let me = e.call(id, &[]).eax;
            e.call(set, &args![3u32, me * 100]);
            e.call(set_error, &args![me]);
            for _ in 0..1000 {
                let b = e.mem.alloc(24);
                e.mem.set_u32(b, me);
                std::hint::black_box(e.mem.u32(b));
                e.mem.free(b);
            }
            let ok = e.call(get, &args![3u32]).eax == me * 100 && e.call(get_error, &[]).eax == me;
            Ret {
                eax: u32::from(ok),
                ..Ret::default()
            }
        }
        let (mut e, slot) = engine_with_imports();
        start(&mut e).expect("real threads on");
        e.register(0x0040_2000, own);
        let p = e.mem.alloc(0x20);
        for (k, name) in [
            "TlsGetValue",
            "TlsSetValue",
            "GetCurrentThreadId",
            "SetLastError",
            "GetLastError",
        ]
        .iter()
        .enumerate()
        {
            e.mem.set_u32(p + 4 * k as u32, slot(name));
        }
        let handles = e.mem.alloc(32);
        for k in 0..8 {
            let h = e
                .call(
                    slot("CreateThread"),
                    &args![0u32, 0u32, 0x0040_2000u32, p, 0u32, 0u32],
                )
                .eax;
            e.mem.set_u32(handles + 4 * k, h);
        }
        e.call(
            slot("WaitForMultipleObjects"),
            &args![8u32, handles, 1u32, 0xffff_ffffu32],
        );
        let code = e.mem.alloc(4);
        for k in 0..8 {
            let h = e.mem.u32(handles + 4 * k);
            e.call(slot("GetExitCodeThread"), &args![h, code]);
            assert_eq!(e.mem.u32(code), 1, "thread {k} kept its own values");
        }
        assert_eq!(e.call(slot("GetCurrentThreadId"), &[]).eax, MAIN_THREAD_ID);
    }

    /// A thread-local storage index freed and handed out again reads 0 on every thread, as on
    /// Windows: a lock-free map keeps each thread's accessor in its own index, so after a map is
    /// destroyed the next map that gets the index must not find the old map's freed accessor (the
    /// save-load stop of 2026-11-07: main walked the path manager's request map with a freed
    /// accessor whose block then held another map).
    #[test]
    fn a_freed_tls_index_reads_0_on_every_thread_when_handed_out_again() {
        /// A game thread: sets its value at the index (word +0) to 0x7777, signals the event at +4,
        /// waits for the event at +8, then stores what the index reads at +12.
        fn keeper(e: &mut Engine, a: &[u32]) -> Ret {
            let p = a[0];
            let call = |e: &mut Engine, at: u32| e.mem.u32(p + at);
            let (get, set, signal, wait) = (call(e, 16), call(e, 20), call(e, 24), call(e, 28));
            let index = e.mem.u32(p);
            e.call(set, &args![index, 0x7777u32]);
            e.call(signal, &args![e.mem.u32(p + 4)]);
            e.call(wait, &args![e.mem.u32(p + 8), 0xffff_ffffu32]);
            let now = e.call(get, &args![index]).eax;
            e.mem.set_u32(p + 12, now);
            Ret::default()
        }
        let (mut e, slot) = engine_with_imports();
        start(&mut e).expect("real threads on");
        e.register(0x0040_5000, keeper);
        let index = e.call(slot("TlsAlloc"), &[]).eax;
        e.call(slot("TlsSetValue"), &args![index, 0x1234u32]);
        let p = e.mem.alloc(0x20);
        e.mem.set_u32(p, index);
        for (at, mapped) in [(4u32, 0u32), (8, 0)] {
            let event = e
                .call(slot("CreateEventA"), &args![0u32, mapped, 0u32, 0u32])
                .eax;
            e.mem.set_u32(p + at, event);
        }
        for (at, name) in [
            (16u32, "TlsGetValue"),
            (20, "TlsSetValue"),
            (24, "SetEvent"),
            (28, "WaitForSingleObject"),
        ] {
            e.mem.set_u32(p + at, slot(name));
        }
        let h = e
            .call(
                slot("CreateThread"),
                &args![0u32, 0u32, 0x0040_5000u32, p, 0u32, 0u32],
            )
            .eax;
        let set = e.mem.u32(p + 4);
        e.call(slot("WaitForSingleObject"), &args![set, 0xffff_ffffu32]);
        // The map goes and the next one gets the same index.
        e.call(slot("TlsFree"), &args![index]);
        assert_eq!(e.call(slot("TlsAlloc"), &[]).eax, index);
        assert_eq!(e.call(slot("TlsGetValue"), &args![index]).eax, 0, "main");
        e.call(slot("SetEvent"), &args![e.mem.u32(p + 8)]);
        e.call(slot("WaitForSingleObject"), &args![h, 0xffff_ffffu32]);
        assert_eq!(e.mem.u32(p + 12), 0, "the game thread");
        // A value set after the new allocation reads back.
        e.call(slot("TlsSetValue"), &args![index, 0x5678u32]);
        assert_eq!(e.call(slot("TlsGetValue"), &args![index]).eax, 0x5678);
    }

    /// With real threads the freed-block check still names who allocated and freed a block, and
    /// on which game thread: a game thread frees a block main allocated.
    #[test]
    fn the_freed_block_check_names_the_thread_that_freed_a_block() {
        fn free_it(e: &mut Engine, a: &[u32]) -> Ret {
            e.mem.set_check_frame(0x0040_6000, 0);
            e.mem.note_path(&[0x0040_6100, 0x0040_6000]);
            e.mem.free(a[0]);
            Ret::default()
        }
        let (mut e, slot) = engine_with_imports();
        e.mem.enable_freed_check(false);
        start(&mut e).expect("real threads with the block check");
        e.register(0x0040_6000, free_it);
        e.mem.set_check_frame(0x0040_7000, 0);
        let block = e.mem.alloc(0x40);
        let h = e
            .call(
                slot("CreateThread"),
                &args![0u32, 0u32, 0x0040_6000u32, block, 0u32, 0u32],
            )
            .eax;
        e.call(slot("WaitForSingleObject"), &args![h, 0xffff_ffffu32]);
        let info = e
            .mem
            .freed_block_info(block)
            .expect("the block waits, freed");
        assert!(
            info.contains("allocated while 00407000 ran") && info.contains("freed while 00406000"),
            "{info}"
        );
        assert!(
            info.contains("allocated on game thread 1, freed on game thread 2"),
            "{info}"
        );
        // A pointer read from it leads to unmapped memory, named for the block.
        let stale = e.mem.u32(block);
        let stop = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| e.mem.u32(stale)));
        let text = stop
            .err()
            .and_then(|p| p.downcast_ref::<String>().cloned())
            .unwrap_or_default();
        assert!(
            text.contains(&format!("the freed block {block:08x}")),
            "{text}"
        );
    }

    /// `SuspendThread` parks a running game thread at its next platform call, wait or sleep, and
    /// `ResumeThread` lets it go on (the game suspends its task threads to end them at quit).
    #[test]
    fn a_suspended_thread_parks_until_resumed() {
        /// Counts the word at +4 up, sleeping 0 ms each time, until the word at +8 is set.
        fn counter(e: &mut Engine, a: &[u32]) -> Ret {
            let p = a[0];
            let sleep = e.mem.u32(p);
            while e.mem.u32(p + 8) == 0 {
                let n = e.mem.u32(p + 4);
                e.mem.set_u32(p + 4, n + 1);
                e.call(sleep, &args![0u32]);
            }
            Ret::default()
        }
        let (mut e, slot) = engine_with_imports();
        start(&mut e).expect("real threads on");
        // As the host's first frame: the kernel lock the start took goes back.
        ctx(&e).frame_begins(false);
        e.register(0x0040_8000, counter);
        let p = e.mem.alloc(0x10);
        e.mem.set_u32(p, slot("Sleep"));
        let h = e
            .call(
                slot("CreateThread"),
                &args![0u32, 0u32, 0x0040_8000u32, p, 0u32, 0u32],
            )
            .eax;
        while e.mem.u32(p + 4) < 100 {
            std::thread::yield_now();
        }
        assert_eq!(e.call(slot("SuspendThread"), &args![h]).eax, 0);
        // It parks at its next sleep: wait for that, not for a time.
        let world = ctx(&e).world;
        let since = std::time::Instant::now();
        while world.parked.load(Ordering::SeqCst) == 0 {
            assert!(
                since.elapsed() < Duration::from_secs(10),
                "the thread never parked"
            );
            std::thread::yield_now();
        }
        let parked = e.mem.u32(p + 4);
        std::thread::sleep(Duration::from_millis(30));
        assert_eq!(
            e.mem.u32(p + 4),
            parked,
            "a suspended thread runs no game code"
        );
        assert_eq!(e.call(slot("ResumeThread"), &args![h]).eax, 1);
        while e.mem.u32(p + 4) <= parked {
            std::thread::yield_now();
        }
        e.mem.set_u32(p + 8, 1);
        let wait = slot("WaitForSingleObject");
        assert_eq!(e.call(wait, &args![h, 0xffff_ffffu32]).eax, 0);
    }

    /// A game thread that stops while it holds a critical section ends the main thread's wait for
    /// that section with the stop, instead of leaving it waiting for good.
    #[test]
    fn a_stop_holding_a_section_ends_mains_wait_for_it() {
        /// Enters the section at +0x40, sets the word at +4, sleeps 30 ms, then calls an address
        /// nothing is registered at (a stop).
        fn holder(e: &mut Engine, a: &[u32]) -> Ret {
            let p = a[0];
            let (enter, sleep) = (e.mem.u32(p + 8), e.mem.u32(p + 12));
            e.call(enter, &args![p + 0x40]);
            e.mem.set_u32(p + 4, 1);
            e.call(sleep, &args![30u32]);
            e.call(0x0040_9990, &[]);
            Ret::default()
        }
        let (mut e, slot) = engine_with_imports();
        start(&mut e).expect("real threads on");
        e.register(0x0040_9000, holder);
        let p = e.mem.alloc(0x80);
        e.mem.set_u32(p + 8, slot("EnterCriticalSection"));
        e.mem.set_u32(p + 12, slot("Sleep"));
        e.call(
            slot("CreateThread"),
            &args![0u32, 0u32, 0x0040_9000u32, p, 0u32, 0u32],
        );
        while e.mem.u32(p + 4) == 0 {
            std::thread::yield_now();
        }
        let enter = slot("EnterCriticalSection");
        let stop = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            e.call(enter, &args![p + 0x40]);
        }));
        let text = stop
            .err()
            .and_then(|p| p.downcast_ref::<String>().cloned())
            .unwrap_or_default();
        assert!(
            text.contains("open function 00409990") && text.contains("in game thread 2"),
            "{text}"
        );
    }

    /// `NV_LOCK_STATS`' sums: time, count and longest since the last take, then a new start.
    #[test]
    fn wait_sums_add_up_and_start_again() {
        let s = WaitSum::default();
        s.add(Duration::from_micros(30));
        s.add(Duration::from_micros(500));
        assert_eq!(s.take(), (530, 2, 500));
        assert_eq!(s.take(), (0, 0, 0));
    }

    /// Fiber-local storage is per thread off the kernel lock, as TLS is: a game thread starts with
    /// none of main's values, keeps its own, and an index freed and handed out again reads 0.
    #[test]
    fn fiber_local_values_are_per_thread() {
        use crate::units::platform::loader::KERNEL32_MODULE;
        /// Stores what the index (+0) reads at +4, sets 0x22, stores what it reads then at +8.
        fn own(e: &mut Engine, a: &[u32]) -> Ret {
            let p = a[0];
            let (index, get, set) = (e.mem.u32(p), e.mem.u32(p + 12), e.mem.u32(p + 16));
            let before = e.call(get, &args![index]).eax;
            e.mem.set_u32(p + 4, before);
            e.call(set, &args![index, 0x22u32]);
            let after = e.call(get, &args![index]).eax;
            e.mem.set_u32(p + 8, after);
            Ret::default()
        }
        let (mut e, slot) = engine_with_imports();
        start(&mut e).expect("real threads on");
        e.register(0x0040_b000, own);
        let names = e.mem.alloc(0x40);
        let proc = |e: &mut Engine, name: &str| {
            e.mem.write(names, name.as_bytes());
            e.mem.set_u8(names + name.len() as u32, 0);
            e.call(slot("GetProcAddress"), &args![KERNEL32_MODULE, names])
                .eax
        };
        let (alloc, get, set, free) = (
            proc(&mut e, "FlsAlloc"),
            proc(&mut e, "FlsGetValue"),
            proc(&mut e, "FlsSetValue"),
            proc(&mut e, "FlsFree"),
        );
        let index = e.call(alloc, &args![0u32]).eax;
        assert_eq!(e.call(set, &args![index, 0x11u32]).eax, 1);
        let p = e.mem.alloc(0x20);
        e.mem.set_u32(p, index);
        e.mem.set_u32(p + 12, get);
        e.mem.set_u32(p + 16, set);
        let h = e
            .call(
                slot("CreateThread"),
                &args![0u32, 0u32, 0x0040_b000u32, p, 0u32, 0u32],
            )
            .eax;
        e.call(slot("WaitForSingleObject"), &args![h, 0xffff_ffffu32]);
        assert_eq!(
            (e.mem.u32(p + 4), e.mem.u32(p + 8)),
            (0, 0x22),
            "the thread's own"
        );
        assert_eq!(e.call(get, &args![index]).eax, 0x11, "main's own");
        e.call(free, &args![index]);
        assert_eq!(
            e.call(set, &args![index, 5u32]).eax,
            0,
            "a freed index takes nothing"
        );
        assert_eq!(e.call(alloc, &args![0u32]).eax, index);
        assert_eq!(e.call(get, &args![index]).eax, 0, "handed out again: 0");
    }

    /// A thread blocked on one event wakes when that event is set, not when others change (as
    /// Windows wakes it; every change woke every waiter before, a storm of wakes and kernel lock
    /// takes with the game's task threads, about 12 cores busy in Goodsprings).
    #[test]
    fn a_waiter_wakes_only_for_its_own_object() {
        /// Waits on the event at +0, then stores 1 at +4.
        fn waits(e: &mut Engine, a: &[u32]) -> Ret {
            let p = a[0];
            let wait = e.mem.u32(p + 8);
            e.call(wait, &args![e.mem.u32(p), 0xffff_ffffu32]);
            e.mem.set_u32(p + 4, 1);
            Ret::default()
        }
        let (mut e, slot) = engine_with_imports();
        start(&mut e).expect("real threads on");
        ctx(&e).frame_begins(false);
        e.register(0x0040_c000, waits);
        let create = slot("CreateEventA");
        let (mine, other) = (
            e.call(create, &args![0u32, 0u32, 0u32, 0u32]).eax,
            e.call(create, &args![0u32, 0u32, 0u32, 0u32]).eax,
        );
        let p = e.mem.alloc(0x10);
        e.mem.set_u32(p, mine);
        e.mem.set_u32(p + 8, slot("WaitForSingleObject"));
        let h = e
            .call(
                slot("CreateThread"),
                &args![0u32, 0u32, 0x0040_c000u32, p, 0u32, 0u32],
            )
            .eax;
        let world = ctx(&e).world;
        let since = std::time::Instant::now();
        while guard(&world.waiters).is_empty() {
            assert!(
                since.elapsed() < Duration::from_secs(10),
                "the thread never waited"
            );
            std::thread::yield_now();
        }
        let before = world.wakes.load(Ordering::SeqCst);
        let (set, reset) = (slot("SetEvent"), slot("ResetEvent"));
        for _ in 0..1000 {
            e.call(set, &args![other]);
            e.call(reset, &args![other]);
        }
        std::thread::sleep(Duration::from_millis(20));
        assert_eq!(e.mem.u32(p + 4), 0, "still waiting");
        let woken = world.wakes.load(Ordering::SeqCst) - before;
        assert!(woken <= 2, "woke {woken} times for another event's changes");
        e.call(set, &args![mine]);
        assert_eq!(
            e.call(slot("WaitForSingleObject"), &args![h, 0xffff_ffffu32])
                .eax,
            0
        );
        assert_eq!(e.mem.u32(p + 4), 1);
    }

    /// A wait that should have ended but timed out (a lost wake) shows as `WAIT_TIMEOUT` after
    /// this long in the stress tests below, not as a hang.
    const STRESS_TIMEOUT_MS: u32 = 10_000;

    /// Lost wakes: 100,000 round trips of two auto-reset events between the main thread and a
    /// game thread, every wait with a timeout. Each side sets the other's event and waits for its
    /// own, so every round needs both wakes; one lost shows as a timed-out wait.
    #[test]
    fn event_ping_pong_loses_no_wake() {
        const ROUNDS: u32 = 100_000;
        /// The block at `a[0]`: +0 ping, +4 pong, +8 `WaitForSingleObject`, +0xc `SetEvent`;
        /// it writes +0x10 rounds done and +0x14 timed-out waits.
        fn pong(e: &mut Engine, a: &[u32]) -> Ret {
            let p = a[0];
            let (ping, pong) = (e.mem.u32(p), e.mem.u32(p + 4));
            let (wait, set) = (e.mem.u32(p + 8), e.mem.u32(p + 0xc));
            let mut done = 0;
            for _ in 0..ROUNDS {
                if e.call(wait, &args![ping, STRESS_TIMEOUT_MS]).eax != 0 {
                    e.mem.set_u32(p + 0x14, 1);
                    break;
                }
                done += 1;
                e.call(set, &args![pong]);
            }
            e.mem.set_u32(p + 0x10, done);
            Ret::default()
        }
        let (mut e, slot) = engine_with_imports();
        start(&mut e).expect("real threads on");
        ctx(&e).frame_begins(false);
        e.register(0x0040_d000, pong);
        let create = slot("CreateEventA");
        // Auto-reset, not set.
        let ping = e.call(create, &args![0u32, 0u32, 0u32, 0u32]).eax;
        let pong_event = e.call(create, &args![0u32, 0u32, 0u32, 0u32]).eax;
        let (wait, set) = (slot("WaitForSingleObject"), slot("SetEvent"));
        let p = e.mem.alloc(0x20);
        for (at, word) in [(0, ping), (4, pong_event), (8, wait), (0xc, set)] {
            e.mem.set_u32(p + at, word);
        }
        let h = e
            .call(
                slot("CreateThread"),
                &args![0u32, 0u32, 0x0040_d000u32, p, 0u32, 0u32],
            )
            .eax;
        let world = ctx(&e).world;
        let before = world.wakes.load(Ordering::SeqCst);
        for round in 0..ROUNDS {
            e.call(set, &args![ping]);
            let got = e.call(wait, &args![pong_event, STRESS_TIMEOUT_MS]).eax;
            assert_eq!(got, 0, "round {round}: no pong (a lost wake)");
        }
        assert_eq!(e.call(wait, &args![h, STRESS_TIMEOUT_MS]).eax, 0);
        assert_eq!(e.mem.u32(p + 0x14), 0, "the game thread's wait timed out");
        assert_eq!(e.mem.u32(p + 0x10), ROUNDS);
        // The waits blocked (parked and were woken), so the wakes were exercised.
        let blocked = world.wakes.load(Ordering::SeqCst) - before;
        eprintln!("event ping-pong: {blocked} waits blocked in {ROUNDS} rounds");
        assert!(
            blocked > u64::from(ROUNDS) / 10,
            "only {blocked} waits blocked"
        );
    }

    /// Four game threads taking units from one semaphore, each wait with the block's timeout
    /// (`STRESS_TIMEOUT_MS` or a short one to retry). The block at `a[0]`: +0 the semaphore, +4
    /// `WaitForSingleObject`, +8 `InterlockedIncrement`, +0xc the timeout, +0x10 the units it
    /// takes, +0x14 the shared count of units taken; it writes +0x18 its long waits timed out
    /// and +0x1c its short ones.
    fn semaphore_taker(e: &mut Engine, a: &[u32]) -> Ret {
        let p = a[0];
        let (sem, wait, increment) = (e.mem.u32(p), e.mem.u32(p + 4), e.mem.u32(p + 8));
        let (timeout, quota, count) =
            (e.mem.u32(p + 0xc), e.mem.u32(p + 0x10), e.mem.u32(p + 0x14));
        let (mut taken, mut short) = (0, 0);
        while taken < quota {
            if e.call(wait, &args![sem, timeout]).eax == 0 {
                taken += 1;
                e.call(increment, &args![count]);
            } else if timeout < STRESS_TIMEOUT_MS {
                short += 1;
            } else {
                e.mem.set_u32(p + 0x18, 1);
                break;
            }
        }
        e.mem.set_u32(p + 0x1c, short);
        Ret::default()
    }

    /// Starts four [`semaphore_taker`]s on a new semaphore, `quota` units each, with `timeouts`;
    /// answers the semaphore, the shared count's address, the threads' blocks and handles.
    fn semaphore_takers(
        e: &mut Engine,
        slot: &impl Fn(&str) -> u32,
        quota: u32,
        timeouts: [u32; 4],
    ) -> (u32, u32, Vec<(u32, u32)>) {
        e.register(0x0040_e000, semaphore_taker);
        let sem = e
            .call(
                slot("CreateSemaphoreA"),
                &args![0u32, 0u32, 0x7fff_ffffu32, 0u32],
            )
            .eax;
        let count = e.mem.alloc(4);
        let threads = timeouts
            .iter()
            .map(|&timeout| {
                let p = e.mem.alloc(0x20);
                let words = [
                    sem,
                    slot("WaitForSingleObject"),
                    slot("InterlockedIncrement"),
                    timeout,
                    quota,
                    count,
                ];
                for (i, word) in words.into_iter().enumerate() {
                    e.mem.set_u32(p + 4 * i as u32, word);
                }
                let h = e
                    .call(
                        slot("CreateThread"),
                        &args![0u32, 0u32, 0x0040_e000u32, p, 0u32, 0u32],
                    )
                    .eax;
                (p, h)
            })
            .collect();
        (sem, count, threads)
    }

    /// The shared count at `a`, as the other threads left it.
    fn shared_count(e: &Engine, a: u32) -> u32 {
        e.mem
            .atomic_u32(a)
            .expect("an aligned word")
            .load(Ordering::SeqCst)
    }

    /// Lost wakes with several waiters: the main thread releases one or two units at a time to
    /// four game threads waiting on one semaphore, and each time waits until they are taken. A
    /// release that wakes nobody leaves its unit untaken (and a waiter's long wait times out).
    #[test]
    fn a_semaphore_with_several_waiters_loses_no_wake() {
        const EACH: u32 = 5_000;
        let (mut e, slot) = engine_with_imports();
        start(&mut e).expect("real threads on");
        ctx(&e).frame_begins(false);
        let (sem, count, threads) = semaphore_takers(&mut e, &slot, EACH, [STRESS_TIMEOUT_MS; 4]);
        let release = slot("ReleaseSemaphore");
        let mut released = 0;
        while released < 4 * EACH {
            let units = (if released % 3 == 0 { 2 } else { 1 }).min(4 * EACH - released);
            e.call(release, &args![sem, units, 0u32]);
            released += units;
            let since = std::time::Instant::now();
            while shared_count(&e, count) < released {
                assert!(
                    since.elapsed() < Duration::from_millis(STRESS_TIMEOUT_MS.into()),
                    "unit {released} never taken (a lost wake)"
                );
                std::thread::yield_now();
            }
        }
        let wait = slot("WaitForSingleObject");
        for (p, h) in threads {
            assert_eq!(e.call(wait, &args![h, STRESS_TIMEOUT_MS]).eax, 0);
            assert_eq!(e.mem.u32(p + 0x18), 0, "a long wait timed out");
        }
        assert_eq!(shared_count(&e, count), 4 * EACH);
        assert_eq!(
            e.call(wait, &args![sem, 0u32]).eax,
            0x102,
            "a unit left over"
        );
    }

    /// Timed waits racing releases: four game threads retry 1-3 ms waits on one semaphore while
    /// the main thread releases 20,000 units one at a time without waiting. Each unit is taken
    /// once: none lost to a wait timing out as it is released, none taken twice.
    #[test]
    fn timed_waits_racing_releases_take_each_unit_once() {
        const EACH: u32 = 5_000;
        let (mut e, slot) = engine_with_imports();
        start(&mut e).expect("real threads on");
        ctx(&e).frame_begins(false);
        let (sem, count, threads) = semaphore_takers(&mut e, &slot, EACH, [1, 1, 2, 3]);
        let release = slot("ReleaseSemaphore");
        for i in 0..4 * EACH {
            e.call(release, &args![sem, 1u32, 0u32]);
            if i % 64 == 0 {
                std::thread::yield_now();
            }
        }
        let wait = slot("WaitForSingleObject");
        let mut short = 0;
        for (p, h) in threads {
            assert_eq!(
                e.call(wait, &args![h, STRESS_TIMEOUT_MS]).eax,
                0,
                "a thread ended"
            );
            short += e.mem.u32(p + 0x1c);
        }
        assert_eq!(shared_count(&e, count), 4 * EACH);
        assert_eq!(
            e.call(wait, &args![sem, 0u32]).eax,
            0x102,
            "a unit left over"
        );
        eprintln!("timed waits racing releases: {short} short waits timed out");
    }

    /// Turns a hang of the main thread into a failure: after `secs`, unless the answered flag is
    /// set, the world is stopped as a game thread's stop stops it, which ends the main thread's
    /// waits for a section or an object with that stop (a panic in the test).
    fn watchdog(world: &'static World, secs: u64) -> std::sync::Arc<AtomicBool> {
        let done = std::sync::Arc::new(AtomicBool::new(false));
        let flag = std::sync::Arc::clone(&done);
        std::thread::spawn(move || {
            let until = std::time::Instant::now() + Duration::from_secs(secs);
            while std::time::Instant::now() < until {
                if flag.load(Ordering::SeqCst) {
                    return;
                }
                std::thread::sleep(Duration::from_millis(20));
            }
            guard(&world.stopped)
                .get_or_insert_with(|| "watchdog: a wait that never ended (a lost wake)".into());
            world.any_stop.store(true, Ordering::SeqCst);
            world.wake_all_waiters();
        });
        done
    }

    /// Four game threads and the main thread enter and leave one section 20,000 times each,
    /// adding 1 to a plain word inside it, the section made with `spin` (`None`: plain
    /// `InitializeCriticalSection`, Windows' 2000): the sum is exact (one holder at a time), and
    /// every thread gets through (a leave wakes one waiter, and none is left parked while the
    /// section is free). The main thread left asleep fails the test through [`watchdog`].
    fn contended_section(spin: Option<u32>) {
        const EACH: u32 = 20_000;
        /// The block at `a[0]`: +0 the section, +4 `EnterCriticalSection`, +8
        /// `LeaveCriticalSection`, +0xc the word it adds to.
        fn adder(e: &mut Engine, a: &[u32]) -> Ret {
            let p = a[0];
            let (section, enter) = (e.mem.u32(p), e.mem.u32(p + 4));
            let (leave, word) = (e.mem.u32(p + 8), e.mem.u32(p + 0xc));
            for _ in 0..EACH {
                e.call(enter, &args![section]);
                let v = e.mem.u32(word);
                std::hint::spin_loop();
                e.mem.set_u32(word, v + 1);
                e.call(leave, &args![section]);
            }
            Ret::default()
        }
        let (mut e, slot) = engine_with_imports();
        start(&mut e).expect("real threads on");
        ctx(&e).frame_begins(false);
        e.register(0x0040_f000, adder);
        let section = e.mem.alloc(0x20);
        match spin {
            Some(spin) => {
                let init = slot("InitializeCriticalSectionAndSpinCount");
                e.call(init, &args![section, spin]);
            }
            None => {
                e.call(slot("InitializeCriticalSection"), &args![section]);
            }
        }
        let word = e.mem.alloc(4);
        let p = e.mem.alloc(0x10);
        let words = [
            section,
            slot("EnterCriticalSection"),
            slot("LeaveCriticalSection"),
            word,
        ];
        for (i, w) in words.into_iter().enumerate() {
            e.mem.set_u32(p + 4 * i as u32, w);
        }
        let handles: Vec<u32> = (0..4)
            .map(|_| {
                e.call(
                    slot("CreateThread"),
                    &args![0u32, 0u32, 0x0040_f000u32, p, 0u32, 0u32],
                )
                .eax
            })
            .collect();
        let done = watchdog(ctx(&e).world, 30);
        adder(&mut e, &[p]);
        let wait = slot("WaitForSingleObject");
        for h in handles {
            assert_eq!(
                e.call(wait, &args![h, STRESS_TIMEOUT_MS]).eax,
                0,
                "a thread got stuck"
            );
        }
        done.store(true, Ordering::SeqCst);
        assert_eq!(e.mem.u32(word), 5 * EACH);
        assert_eq!(e.mem.u32(section + 0xc), 0, "the section is free");
    }

    /// [`contended_section`] with a spin count of 1: most waits park.
    #[test]
    fn a_contended_section_lets_one_in_at_a_time_and_loses_no_waiter() {
        contended_section(Some(1));
    }

    /// [`contended_section`] with Windows' default spin (2000 tries before parking).
    #[test]
    fn a_contended_section_with_the_default_spin_loses_no_waiter() {
        contended_section(None);
    }

    /// Rounds of the `WaitForMultipleObjects` stress tests. Their block: +0 and +4 two
    /// auto-reset events (the handle array), +8 the acknowledgement event, +0xc
    /// `WaitForMultipleObjects`, +0x10 `SetEvent`, +0x14 `WaitForSingleObject`, +0x18 wait-all
    /// (1) or wait-any (0); the game thread writes +0x1c rounds done, +0x20 wrong answers, +0x24
    /// 1 when a wait timed out.
    const MULTI_ROUNDS: u32 = 20_000;

    /// The game thread of the `WaitForMultipleObjects` stress tests ([`MULTI_ROUNDS`] waits): a
    /// wait-any must answer the event the main thread set this round (alternately the first and
    /// the second); a wait-all must have taken both (neither still set after it).
    fn multi_waiter(e: &mut Engine, a: &[u32]) -> Ret {
        let p = a[0];
        let (first, second, ack) = (e.mem.u32(p), e.mem.u32(p + 4), e.mem.u32(p + 8));
        let (wait_many, set) = (e.mem.u32(p + 0xc), e.mem.u32(p + 0x10));
        let (wait, all) = (e.mem.u32(p + 0x14), e.mem.u32(p + 0x18));
        let (mut done, mut wrong) = (0, 0);
        for round in 0..MULTI_ROUNDS {
            let got = e
                .call(wait_many, &args![2u32, p, all, STRESS_TIMEOUT_MS])
                .eax;
            if got == 0x102 {
                e.mem.set_u32(p + 0x24, 1);
                break;
            }
            let right = if all != 0 {
                got == 0
                    && e.call(wait, &args![first, 0u32]).eax == 0x102
                    && e.call(wait, &args![second, 0u32]).eax == 0x102
            } else {
                got == round % 2
            };
            if !right {
                wrong += 1;
            }
            done += 1;
            e.call(set, &args![ack]);
        }
        e.mem.set_u32(p + 0x1c, done);
        e.mem.set_u32(p + 0x20, wrong);
        Ret::default()
    }

    /// Runs [`multi_waiter`] (`all`: wait-all) against the main thread's rounds: wait-any, one
    /// event set a round; wait-all, the first set, then (the waiter must not have answered yet)
    /// the second. Every main-thread wait has a timeout, so a lost wake fails.
    fn wait_for_multiple(all: bool) {
        let (mut e, slot) = engine_with_imports();
        start(&mut e).expect("real threads on");
        ctx(&e).frame_begins(false);
        e.register(0x0040_a800, multi_waiter);
        let create = slot("CreateEventA");
        let p = e.mem.alloc(0x40);
        for at in [0, 4, 8] {
            let event = e.call(create, &args![0u32, 0u32, 0u32, 0u32]).eax;
            e.mem.set_u32(p + at, event);
        }
        let (set, wait) = (slot("SetEvent"), slot("WaitForSingleObject"));
        e.mem.set_u32(p + 0xc, slot("WaitForMultipleObjects"));
        e.mem.set_u32(p + 0x10, set);
        e.mem.set_u32(p + 0x14, wait);
        e.mem.set_u32(p + 0x18, u32::from(all));
        let h = e
            .call(
                slot("CreateThread"),
                &args![0u32, 0u32, 0x0040_a800u32, p, 0u32, 0u32],
            )
            .eax;
        let (first, second, ack) = (e.mem.u32(p), e.mem.u32(p + 4), e.mem.u32(p + 8));
        let mut early = 0;
        for round in 0..MULTI_ROUNDS {
            if all {
                e.call(set, &args![first]);
                std::thread::yield_now();
                if e.call(wait, &args![ack, 0u32]).eax == 0 {
                    early += 1;
                }
                e.call(set, &args![second]);
            } else {
                let event = if round % 2 == 0 { first } else { second };
                e.call(set, &args![event]);
            }
            let got = e.call(wait, &args![ack, STRESS_TIMEOUT_MS]).eax;
            assert_eq!(got, 0, "round {round}: no answer (a lost wake)");
        }
        assert_eq!(e.call(wait, &args![h, STRESS_TIMEOUT_MS]).eax, 0);
        assert_eq!(e.mem.u32(p + 0x24), 0, "the game thread's wait timed out");
        assert_eq!(e.mem.u32(p + 0x1c), MULTI_ROUNDS);
        assert_eq!(e.mem.u32(p + 0x20), 0, "wrong answers");
        assert_eq!(early, 0, "a wait-all ended with one event set");
    }

    /// `WaitForMultipleObjects` for any of two events, 20,000 rounds: each wakes for the event
    /// set and names it.
    #[test]
    fn waiting_for_any_of_two_events_wakes_for_each_and_names_it() {
        wait_for_multiple(false);
    }

    /// `WaitForMultipleObjects` for all of two events, 20,000 rounds: it ends only when both are
    /// set, and takes both.
    #[test]
    fn waiting_for_all_of_two_events_takes_both_at_once() {
        wait_for_multiple(true);
    }

    /// Every game thread's engine context has the run's one kernel table (`KernelRef::id`), while
    /// its engine and its memory view are its own: the key the host's fixes keep run-wide state by
    /// (keyed by the memory view's address, the dialogue topic index answered 0 on other threads).
    #[test]
    fn every_thread_has_the_runs_kernel_table_and_a_memory_view_of_its_own() {
        /// Writes its kernel table's id and its memory view's address at `a[0]`.
        fn ids(e: &mut Engine, a: &[u32]) -> Ret {
            let kernel = e.kernel.id() as u64;
            let view = std::ptr::from_ref(&e.mem) as usize as u64;
            for (i, word) in [kernel, kernel >> 32, view, view >> 32]
                .into_iter()
                .enumerate()
            {
                e.mem.set_u32(a[0] + 4 * i as u32, word as u32);
            }
            Ret::default()
        }
        let (mut e, slot) = engine_with_imports();
        start(&mut e).expect("real threads on");
        ctx(&e).frame_begins(false);
        e.register(0x0040_a000, ids);
        let p = e.mem.alloc(0x10);
        let h = e
            .call(
                slot("CreateThread"),
                &args![0u32, 0u32, 0x0040_a000u32, p, 0u32, 0u32],
            )
            .eax;
        let wait = slot("WaitForSingleObject");
        assert_eq!(e.call(wait, &args![h, STRESS_TIMEOUT_MS]).eax, 0);
        let word = |e: &Engine, at: u32| {
            u64::from(e.mem.u32(p + at)) | (u64::from(e.mem.u32(p + at + 4)) << 32)
        };
        assert_eq!(word(&e, 0), e.kernel.id() as u64, "the run's kernel table");
        assert_ne!(
            word(&e, 8),
            std::ptr::from_ref(&e.mem) as usize as u64,
            "a memory view of its own"
        );
    }

    /// A section given no spin count of its own gets the one Windows writes: 2000 tries, with the
    /// dynamic-spin flag (measured, `DEFAULT_SECTION_SPIN`); a spin count given is kept.
    #[test]
    fn a_section_without_a_spin_count_gets_the_windows_default() {
        let (mut e, slot) = engine_with_imports();
        start(&mut e).expect("real threads on");
        ctx(&e).frame_begins(false);
        let section = e.mem.alloc(0x20);
        e.call(slot("InitializeCriticalSection"), &args![section]);
        assert_eq!(e.mem.u32(section + 0x14), 0x0200_07d0);
        let and_spin = slot("InitializeCriticalSectionAndSpinCount");
        e.call(and_spin, &args![section, 0u32]);
        assert_eq!(e.mem.u32(section + 0x14), 0x0200_07d0);
        e.call(and_spin, &args![section, 4000u32]);
        assert_eq!(e.mem.u32(section + 0x14), 4000);
    }

    /// A thread the game made before real threads started, not run yet, starts real with them.

    #[test]
    fn a_thread_made_before_the_start_runs_real() {
        fn mark(e: &mut Engine, a: &[u32]) -> Ret {
            e.mem.set_u32(a[0], 0x5a5a);
            Ret {
                eax: 9,
                ..Ret::default()
            }
        }
        let (mut e, slot) = engine_with_imports();
        e.register(0x0040_4000, mark);
        let p = e.mem.alloc(4);
        let h = e
            .call(
                slot("CreateThread"),
                &args![0u32, 0u32, 0x0040_4000u32, p, 0u32, 0u32],
            )
            .eax;
        assert_ne!(h, 0);
        start(&mut e).expect("real threads on");
        let waited = e
            .call(slot("WaitForSingleObject"), &args![h, 0xffff_ffffu32])
            .eax;
        assert_eq!(waited, 0, "the thread ended");
        assert_eq!(e.mem.u32(p), 0x5a5a);
    }

    /// A diagnostic keeps real threads off.
    #[test]
    fn a_diagnostic_keeps_threads_stepped() {
        let (mut e, _) = engine_with_imports();
        e.call_log = Some(Vec::new());
        assert!(start(&mut e).is_err());
        assert!(e.real.is_none());
    }

    /// The barrier runs while no game thread runs game code: a thread spinning on interlocked
    /// operations stops at one, and goes on after.
    #[test]
    fn the_barrier_stops_spinning_threads_at_a_safe_point() {
        fn spin(e: &mut Engine, a: &[u32]) -> Ret {
            let p = a[0];
            let inc = e.mem.u32(p);
            while e.mem.u32(p + 8) == 0 {
                e.call(inc, &args![p + 4]);
            }
            Ret::default()
        }
        let (mut e, slot) = engine_with_imports();
        start(&mut e).expect("real threads on");
        e.register(0x0040_3000, spin);
        let p = e.mem.alloc(0x10);
        e.mem.set_u32(p, slot("InterlockedIncrement"));
        let h = e
            .call(
                slot("CreateThread"),
                &args![0u32, 0u32, 0x0040_3000u32, p, 0u32, 0u32],
            )
            .eax;
        while e.mem.u32(p + 4) < 1000 {
            std::thread::yield_now();
        }
        let world = ctx(&e).world;
        let (before, after) = world.barrier(|| {
            let before = e.mem.u32(p + 4);
            std::thread::sleep(Duration::from_millis(20));
            (before, e.mem.u32(p + 4))
        });
        assert_eq!(before, after, "nothing ran during the barrier");
        e.mem.set_u32(p + 8, 1);
        let wait = slot("WaitForSingleObject");
        assert_eq!(e.call(wait, &args![h, 0xffff_ffffu32]).eax, 0);
    }
}

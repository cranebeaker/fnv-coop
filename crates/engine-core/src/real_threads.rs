//! Real threads (`NV_THREADS=real`; ADR-0015, migration steps 5, 7, 8 and 9): the game's threads
//! run side by side on their own cores, as they do in the original, instead of one at a time on
//! the stepped host ([`crate::thread_host`], still the default and the reference mode for tests,
//! recordings and traces).
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
//! serialised by a lock of its own). Critical sections are a table of owners with a condition
//! variable: a contended `EnterCriticalSection` blocks for real. `TlsGetValue`, `TlsSetValue`,
//! `GetCurrentThreadId`, `GetLastError` and `SetLastError` answer from the calling thread's
//! context; `Sleep` sleeps for real (`Sleep(0)` yields). Events, semaphores, mutexes and thread
//! handles stay in the kernel table: a wait checks them under the kernel lock and, when not
//! signalled, waits on a condition variable that every change of state wakes
//! (`thread_host::signal`).
//!
//! **Allocator and platform locks (step 8).** The heap (and the asked sizes `_msize` reads) is
//! behind one lock (`Mem`'s shared state). Every other platform function, the Direct3D 9, sound,
//! file, window and process ones, runs under one re-entrant kernel lock ([`locked_call`]), so the
//! platform layer's state (`Kernel::platform`) is changed by one thread at a time. The exe creates
//! its Direct3D device with `D3DCREATE_MULTITHREADED` (behaviour flags 0x54, `d3dxreal.rs`'s
//! `GAME_BEHAVIOR`), so the device itself serialises calls too; the lock keeps the platform's own
//! bookkeeping right. Pure functions are left outside it: the clock imports and D3DX's matrix,
//! vector and plane maths.
//!
//! **The host (step 9).** [`start`] turns real threads on for an engine before the game's `main`
//! runs; from then on `CreateThread` starts a real OS thread at once (or at `ResumeThread` to 0).
//! A thread that stops (an untranslated function, a fault) ends the run at main's next frame end,
//! naming the thread. **Diagnostics force stepped threads** (R6): a trace, a recording, a profile,
//! the heap census, the touch census, a watch, a per-call hook: [`start`] refuses and the run goes
//! on stepped. Two run with real threads: the block check (`NV_BLOCK_CHECK`; it sees allocations
//! and frees, each named with its thread, not stores) and `NV_REAL_DUMP` ([`dump_threads`]).
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

    /// Waits until it is free and takes it (not held by this thread).
    #[cold]
    fn acquire(&self, me: u64) {
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
        if self.dirty.swap(false, Ordering::Relaxed) {
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
    sections: Mutex<AddrMap<u32, Section>>,
    section_freed: Condvar,
    /// Threads waiting for a critical section.
    section_waiters: AtomicU32,
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
    /// their own at their next platform call, wait or sleep and park there ([`park_if_suspended`]).
    suspended: AtomicBool,
    /// [`start`] took the kernel lock on the host's thread for the start-up (the C runtime's, the
    /// static initializers, the host's own use of the kernel table); the first frame gives it back
    /// ([`Ctx::frame_begins`]).
    held_from_start: AtomicBool,
    /// Each thread-local storage index's generation: `TlsAlloc` and `TlsFree` move it on, so a
    /// value a thread set before reads 0 ([`Ctx::tls`]).
    tls_gen: Box<[AtomicU32]>,
    /// `NV_REAL_DUMP` (a diagnostic): every thread's record, for [`dump_threads`].
    dump: Option<Dump>,
}

impl World {
    fn new(import_names: AddrMap<u32, &'static str>) -> Self {
        World {
            kernel: KernelLock::new(),
            sections: Mutex::new(AddrMap::default()),
            section_freed: Condvar::new(),
            section_waiters: AtomicU32::new(0),
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
            suspended: AtomicBool::new(false),
            held_from_start: AtomicBool::new(false),
            tls_gen: (0..TLS_INDEXES).map(|_| AtomicU32::new(0)).collect(),
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
        let mut e = template.thread_context(ctx);
        e.mem.set_view_thread(id);
        if self.dump.is_some() {
            e.call_hook = Some(record_call);
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
    /// `NV_REAL_DUMP`: what this thread is doing, for [`dump_threads`]; `None` otherwise.
    record: Option<std::sync::Arc<ThreadRecord>>,
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
            record: None,
            locked: self.locked.clone(),
            late_seen: self.late_seen,
            world: self.world,
        }
    }

    /// An object's state changed (under the kernel lock): its waiters look again when it is
    /// released (`thread_host::signal`).
    pub(crate) fn changed(&self) {
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
    pub(crate) fn frame_ended(&self, mem: &mut crate::Mem) -> Option<String> {
        let w = self.world;
        w.kernel.lock(w);
        if mem.deferred_pages() >= HAND_BACK_AT {
            w.barrier(|| mem.hand_back_deferred());
        }
        guard(&w.stopped).clone()
    }
}

/// Turns real threads on for `e`, before the game makes its threads (the host, before the
/// game's `main`): its memory becomes shared, the synchronisation imports become real ones, the
/// other platform functions go behind the kernel lock, and `e` becomes the main thread's engine.
/// Refused (stepped threads stay) while a diagnostic is on, or once a game thread ran stepped; a
/// thread made earlier that has not run yet starts here, on its own OS thread (one made suspended
/// starts when resumed).
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
        record: world.new_record(MAIN_THREAD_ID),
        locked,
        late_seen: 0,
        world,
    }));
    e.mem.set_view_thread(MAIN_THREAD_ID);
    if world.dump.is_some() {
        e.call_hook = Some(record_call);
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
    if e.call_trace.is_some() {
        return Some("an nv-trace record (NV_TRACE_OUT)");
    }
    if e.trace.is_some() {
        return Some("the return trace (NV_MOVE_TRACE)");
    }
    if e.call_log.is_some()
        || e.call_budget.is_some()
        || e.call_counts.is_some()
        || e.call_hook.is_some()
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
        // Wake the main thread if it waits for a section this thread held.
        let _sections = guard(&world.sections);
        world.section_freed.notify_all();
    }
    let t = &mut e.kernel.threads.list[i];
    t.state = state;
    crate::thread_host::signal(&mut e);
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

/// `f` registered at `addr` on `e`, as real threads keep it: behind the kernel lock when it is a
/// platform function there.
fn install(e: &mut Engine, addr: u32, f: AbiFn) {
    let Some(r) = &mut e.real else { return };
    if r.world.locked_addr(addr) {
        r.locked.insert(addr, f);
        e.register_here(addr, locked_call);
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
        "InitializeCriticalSection" | "DeleteCriticalSection" => {
            crate::entry!(0, critical_section_reset(Ptr)).1
        }
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

// Real threads: replaces KERNEL32.DLL EnterCriticalSection (import)
/// Takes the section for the calling thread (again, counted, when it holds it), blocking while
/// another holds it. The kernel lock, if this thread holds it, is given back meanwhile.
fn enter_critical_section(e: &mut Engine, section: Ptr) {
    let r = ctx(e);
    let (w, me) = (r.world, r.id);
    w.safe_point();
    let mut table = guard(&w.sections);
    let s = table.entry(section.addr()).or_default();
    if s.owner == 0 || s.owner == me {
        s.owner = me;
        s.count += 1;
        return;
    }
    let owner = s.owner;
    drop(table);
    note_state(e, || {
        format!(
            "waiting for critical section {:08x} (held by thread {owner})",
            section.addr()
        )
    });
    let held = w.kernel.release_all();
    let was = w.leave_guest();
    let mut table = guard(&w.sections);
    w.section_waiters.fetch_add(1, Ordering::SeqCst);
    // The main thread ends the run when a game thread stopped (it may have stopped holding this
    // section, which then never comes free): the stop wakes the section's waiters.
    let mut stop = None;
    loop {
        let s = table.entry(section.addr()).or_default();
        if s.owner == 0 {
            s.owner = me;
            s.count = 1;
            break;
        }
        if me == MAIN_THREAD_ID {
            stop = guard(&w.stopped).clone();
            if stop.is_some() {
                break;
            }
        }
        table = wait(&w.section_freed, table);
    }
    w.section_waiters.fetch_sub(1, Ordering::SeqCst);
    drop(table);
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
/// Counts a hold down; at 0 the section is free and a waiter may take it.
fn leave_critical_section(e: &mut Engine, section: Ptr) {
    // As on Windows (and the stepped host): a leave counts the hold down whoever calls it.
    let w = ctx(e).world;
    let mut table = guard(&w.sections);
    if let Some(s) = table.get_mut(&section.addr()) {
        if s.count > 0 {
            s.count -= 1;
            if s.count == 0 {
                s.owner = 0;
                if w.section_waiters.load(Ordering::SeqCst) > 0 {
                    w.section_freed.notify_all();
                }
            }
        }
    }
}

// Real threads: replaces KERNEL32.DLL TryEnterCriticalSection (import)
fn try_enter_critical_section(e: &mut Engine, section: Ptr) -> i32 {
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

// Real threads: replaces KERNEL32.DLL InitializeCriticalSection and DeleteCriticalSection (imports)
fn critical_section_reset(e: &mut Engine, section: Ptr) {
    guard(&ctx(e).world.sections).remove(&section.addr());
}

// Real threads: replaces KERNEL32.DLL InitializeCriticalSectionAndSpinCount (import)
fn initialize_critical_section_and_spin_count(e: &mut Engine, section: Ptr, _spin: u32) -> i32 {
    critical_section_reset(e, section);
    1
}

// Real threads: replaces KERNEL32.DLL Sleep (import)
/// Sleeps for real (`Sleep(0)` gives the rest of the time slice up); a safe point.
fn sleep(e: &mut Engine, ms: u32) {
    real_sleep(ctx(e).world, ms);
    park_after_sleep(e);
    main_stop_check(e);
}

/// After a sleep (not under the kernel lock): parks the thread if it was suspended.
fn park_after_sleep(e: &mut Engine) {
    let w = ctx(e).world;
    if w.suspended.load(Ordering::Relaxed) {
        w.kernel.lock(w);
        park_if_suspended(e);
        w.kernel.unlock();
    }
}

/// `SuspendThread` and `ResumeThread` with real threads (`units/platform.rs`, under the kernel
/// lock): whether any game thread that has run is now suspended, and a change, so a parked thread
/// looks at its count again.
pub(crate) fn suspends_changed(e: &mut Engine) {
    let Some(w) = e.real.as_ref().map(|r| r.world) else {
        return;
    };
    let any = e.kernel.threads.list.iter().skip(1).any(|t| {
        t.suspend_count > 0
            && !matches!(
                t.state,
                RunState::NotStarted | RunState::Finished { .. } | RunState::Stopped(_)
            )
    });
    w.suspended.store(any, Ordering::SeqCst);
    crate::thread_host::signal(e);
}

/// Under the kernel lock: while this game thread's suspend count is above 0 (`SuspendThread`),
/// it waits, not running game code, until `ResumeThread` brings it to 0. Windows stops a
/// suspended thread wherever it is; here it stops at its next platform call, wait or sleep (the
/// game suspends its task threads to end them at quit, `00c42f00`, then closes their handles).
fn park_if_suspended(e: &mut Engine) {
    let Some(r) = &e.real else { return };
    let (w, i) = (r.world, r.index);
    if i == 0 || !w.suspended.load(Ordering::Relaxed) {
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
        }
        w.kernel.wait_change(w, None);
    }
    if parked {
        note_state(e, String::new);
    }
}

/// On the main thread: a game thread that stopped ends the run now (main's next frame end would,
/// but a loop waiting for that thread, such as a spin lock it held, never gets there).
fn main_stop_check(e: &Engine) {
    if let Some(r) = &e.real {
        if r.id == MAIN_THREAD_ID {
            if let Some(why) = guard(&r.world.stopped).clone() {
                panic!("{why}");
            }
        }
    }
}

fn real_sleep(w: &World, ms: u32) {
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
    /// Under the kernel lock (the wait imports hold it): until the objects are signalled, waiting
    /// on the condition variable every change wakes; a timed wait ends at its deadline. On the
    /// main thread a game thread's stop ends the wait (the run stops).
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
            w.kernel.wait_change(w, timeout);
        };
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
        park_after_sleep(e);
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
    /// `param + 8`, the word at `param + 0x40`; then it ends with the thread's id.
    fn worker(e: &mut Engine, a: &[u32]) -> Ret {
        let p = a[0];
        let n = e.mem.u32(p);
        let inc = e.mem.u32(p + 0x10);
        let enter = e.mem.u32(p + 0x14);
        let leave = e.mem.u32(p + 0x18);
        let id = e.mem.u32(p + 0x1c);
        for _ in 0..n {
            e.call(inc, &args![p + 4]);
            e.call(enter, &args![p + 8]);
            let v = e.mem.u32(p + 0x40);
            std::hint::black_box(());
            e.mem.set_u32(p + 0x40, v + 1);
            e.call(leave, &args![p + 8]);
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
        std::thread::sleep(Duration::from_millis(20));
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
        /// Enters the section at +0, sets the word at +4, sleeps 30 ms, then calls an address
        /// nothing is registered at (a stop).
        fn holder(e: &mut Engine, a: &[u32]) -> Ret {
            let p = a[0];
            let (enter, sleep) = (e.mem.u32(p + 8), e.mem.u32(p + 12));
            e.call(enter, &args![p]);
            e.mem.set_u32(p + 4, 1);
            e.call(sleep, &args![30u32]);
            e.call(0x0040_9990, &[]);
            Ret::default()
        }
        let (mut e, slot) = engine_with_imports();
        start(&mut e).expect("real threads on");
        e.register(0x0040_9000, holder);
        let p = e.mem.alloc(0x20);
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
            e.call(enter, &args![p]);
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

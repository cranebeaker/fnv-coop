//! Real threads' events and semaphores, off the kernel lock (`real_threads`'s `sync`).
//!
//! With the kernel lock, every `SetEvent`, `ReleaseSemaphore` and wait was a platform call under
//! the one lock, and a signal woke every thread waiting on the object: a `ReleaseSemaphore(1)`
//! with the exe's six tasklet workers waiting (`00b023f0`) woke all six, five of them to wait
//! again, each through the lock twice. In a frame where the workers ran about 16,000 pathing jobs
//! that was 62,487 contended takes of the lock by the game threads and 8,764 by main, 113 ms
//! (measure/stutter/REAL-THREADS.md).
//!
//! Here each event and semaphore has its state in an atomic word (an event's 1 or 0, a
//! semaphore's count) and its own small lock for the threads queued on it. Taking a free unit,
//! setting an event or releasing a semaphore nobody waits on touches no lock; a release wakes as
//! many queued threads as it gives units (one for an auto-reset event, all for a manual-reset
//! one), each the first not woken already, and a woken thread that does not use its wake (it took
//! another object, timed out, or lost the unit to a thread that never waited) passes it on as it
//! leaves. A waiter is queued, then looks at the state, then parks; a release changes the state,
//! then looks at the queue's count: one of the two sees the other (both sequentially
//! consistent), so no wake is lost.
//!
//! `CreateEventA` and `CreateSemaphoreA` still make their kernel-table entries (handles,
//! `CloseHandle`), under the kernel lock as before; the entry's own state is not used with real
//! threads. Waits on anything else (threads, mutexes, files), or on a mix, take the kernel lock's
//! path as before, whose `signalled` and `take` read these objects' state, and a release here
//! wakes that path's waiters on the handle too. Stepped threads keep the kernel table as it was.

use super::*;
use std::sync::Arc;

const WAIT_OBJECT_0: u32 = 0;
const WAIT_TIMEOUT: u32 = 0x102;
const INFINITE: u32 = u32::MAX;

/// The imports this module replaces with real threads.
const IMPORTS: [&str; 7] = [
    "CreateEventA",
    "SetEvent",
    "ResetEvent",
    "CreateSemaphoreA",
    "ReleaseSemaphore",
    "WaitForSingleObject",
    "WaitForMultipleObjects",
];

/// The replacement of import `name`, if it is one of [`IMPORTS`]. Each falls back to the platform
/// function under the kernel lock ([`locked_call`]) for a handle that is not an event or a
/// semaphore of this module.
pub(super) fn import(name: &str) -> Option<AbiFn> {
    if !IMPORTS.contains(&name) {
        return None;
    }
    Some(match name {
        "CreateEventA" => crate::entry!(0, create_event_a(u32, u32, u32, u32) -> u32).1,
        "SetEvent" => crate::entry!(0, set_event(u32) -> u32).1,
        "ResetEvent" => crate::entry!(0, reset_event(u32) -> u32).1,
        "CreateSemaphoreA" => crate::entry!(0, create_semaphore_a(u32, u32, u32, u32) -> u32).1,
        "ReleaseSemaphore" => crate::entry!(0, release_semaphore(u32, u32, Ptr) -> u32).1,
        "WaitForSingleObject" => crate::entry!(0, wait_for_single_object(u32, u32) -> u32).1,
        _ => crate::entry!(0, wait_for_multiple_objects(u32, Ptr, u32, u32) -> u32).1,
    })
}

#[derive(Clone, Copy)]
enum Kind {
    Event { manual: bool },
    Semaphore { maximum: i32 },
}

/// An event or a semaphore of real threads.
pub(crate) struct Object {
    kind: Kind,
    /// An event's state (1 set, 0 reset), a semaphore's count.
    state: std::sync::atomic::AtomicI32,
    /// How many threads are in [`Object::queue`], read without its lock.
    waiting: AtomicU32,
    /// The threads waiting on it.
    queue: Mutex<Vec<Arc<Waiter>>>,
}

impl Object {
    fn new(kind: Kind, state: i32) -> Self {
        Object {
            kind,
            state: std::sync::atomic::AtomicI32::new(state),
            waiting: AtomicU32::new(0),
            queue: Mutex::new(Vec::new()),
        }
    }

    /// Whether a waiter would go on now (an event set, a semaphore's count above 0).
    pub(crate) fn available(&self) -> bool {
        self.state.load(Ordering::SeqCst) > 0
    }

    /// Takes it if it is available: an auto-reset event resets, a semaphore counts down, a
    /// manual-reset event stays set. Whether it was.
    pub(crate) fn try_take(&self) -> bool {
        match self.kind {
            Kind::Event { manual: true } => self.available(),
            _ => update(&self.state, |s| (s > 0).then(|| s - 1)).is_some(),
        }
    }

    /// Gives back what [`Object::try_take`] took (a wait for all that could not take every
    /// object), waking a waiter for it.
    /// An auto-reset event is set again only if it is still reset (0 to 1, once); a semaphore's
    /// count goes up by one but never past its maximum.
    fn give_back(&self) {
        let back = match self.kind {
            Kind::Event { manual: true } => false,
            Kind::Event { manual: false } => self
                .state
                .compare_exchange(0, 1, Ordering::SeqCst, Ordering::SeqCst)
                .is_ok(),
            Kind::Semaphore { maximum } => {
                update(&self.state, |c| (c < maximum).then(|| c + 1)).is_some()
            }
        };
        if back {
            self.wake(1);
        }
    }

    /// Wakes up to `n` of its queued threads waiting for any of their objects, each the first not
    /// woken already, and every one waiting for all of its objects (it takes this one only with the
    /// others, so it may leave it to the rest: woken, it looks and goes on or waits again, and
    /// passes nothing on).
    fn wake(&self, n: usize) {
        if n == 0 || self.waiting.load(Ordering::SeqCst) == 0 {
            return;
        }
        let queue = guard(&self.queue);
        let mut left = n;
        for o in queue.iter() {
            if o.all {
                if !o.woken.swap(true, Ordering::SeqCst) {
                    o.thread.unpark();
                }
            } else if left > 0 && !o.woken.swap(true, Ordering::SeqCst) {
                o.thread.unpark();
                left -= 1;
            }
        }
    }

    /// How many threads wait on it now.
    #[cfg(test)]
    pub(crate) fn queued(&self) -> u32 {
        self.waiting.load(Ordering::SeqCst)
    }

    /// Wakes every queued thread (a game thread stopped: the main thread ends with it).
    pub(super) fn wake_all(&self) {
        for o in guard(&self.queue).iter() {
            o.woken.store(true, Ordering::SeqCst);
            o.thread.unpark();
        }
    }
}

/// `f` applied to `word` (compare and swap, again while another thread changes it in between):
/// the value it changed, or `None` when `f` answers `None`.
fn update(word: &std::sync::atomic::AtomicI32, f: impl Fn(i32) -> Option<i32>) -> Option<i32> {
    let mut old = word.load(Ordering::SeqCst);
    loop {
        let new = f(old)?;
        match word.compare_exchange_weak(old, new, Ordering::SeqCst, Ordering::SeqCst) {
            Ok(_) => return Some(old),
            Err(now) => old = now,
        }
    }
}

impl World {
    /// The event or semaphore `handle` names, from the run's table (the kernel lock's path:
    /// [`sync_object`]).
    pub(super) fn sync_object(&self, handle: u32) -> Option<Arc<Object>> {
        guard(&self.sync).get(&handle).cloned()
    }
}

/// The event or semaphore `handle` names with real threads (`None` for any other handle, and
/// with stepped threads): for the kernel lock's path of waits (`units/platform.rs`'s `signalled`
/// and `take`).
pub(crate) fn sync_object(e: &Engine, handle: u32) -> Option<Arc<Object>> {
    e.real.as_ref()?.world.sync_object(handle)
}

/// `CloseHandle` (under the kernel lock) closed `handle`: an event or a semaphore leaves the
/// table, and every thread's cache of it starts again.
pub(crate) fn handle_closed(e: &Engine, handle: u32) {
    let Some(r) = &e.real else { return };
    if guard(&r.world.sync).remove(&handle).is_some() {
        r.world.sync_epoch.fetch_add(1, Ordering::SeqCst);
    }
}

/// The event or semaphore `handle` names, through this thread's cache.
fn object(e: &mut Engine, handle: u32) -> Option<Arc<Object>> {
    let r = ctx_mut(e);
    let epoch = r.world.sync_epoch.load(Ordering::SeqCst);
    if r.sync_seen != epoch {
        r.sync_cache.clear();
        r.sync_seen = epoch;
    }
    if let Some(o) = r.sync_cache.get(&handle) {
        return o.clone();
    }
    let o = r.world.sync_object(handle);
    r.sync_cache.insert(handle, o.clone());
    o
}

/// A new event or semaphore at `handle`.
fn made(e: &Engine, handle: u32, object: Object) {
    let w = ctx(e).world;
    guard(&w.sync).insert(handle, Arc::new(object));
    w.sync_epoch.fetch_add(1, Ordering::SeqCst);
}

/// The kernel lock's path's waiters on `handle` (a wait on a mix of objects) look again.
fn others_look(e: &Engine, handle: u32) {
    if ctx(e).world.waiters_n.load(Ordering::SeqCst) > 0 {
        object_signalled(e, handle);
    }
}

// Real threads: replaces KERNEL32.DLL CreateEventA (import)
/// The platform's `CreateEventA` (its handle, its checks), then the event here.
fn create_event_a(e: &mut Engine, security: u32, manual: u32, initial: u32, name: u32) -> u32 {
    let handle = locked_call(e, &[security, manual, initial, name]).eax;
    if handle != 0 {
        made(
            e,
            handle,
            Object::new(
                Kind::Event {
                    manual: manual != 0,
                },
                i32::from(initial != 0),
            ),
        );
    }
    handle
}

// Real threads: replaces KERNEL32.DLL CreateSemaphoreA (import)
/// The platform's `CreateSemaphoreA` (its handle; 0 for counts Windows refuses), then the
/// semaphore here.
fn create_semaphore_a(e: &mut Engine, security: u32, initial: u32, maximum: u32, name: u32) -> u32 {
    let handle = locked_call(e, &[security, initial, maximum, name]).eax;
    if handle != 0 {
        made(
            e,
            handle,
            Object::new(
                Kind::Semaphore {
                    maximum: maximum as i32,
                },
                initial as i32,
            ),
        );
    }
    handle
}

// Real threads: replaces KERNEL32.DLL SetEvent (import)
fn set_event(e: &mut Engine, handle: u32) -> u32 {
    let Some(o) = object(e, handle) else {
        return locked_call(e, &[handle]).eax;
    };
    let Kind::Event { manual } = o.kind else {
        return 0;
    };
    o.state.store(1, Ordering::SeqCst);
    o.wake(if manual { usize::MAX } else { 1 });
    others_look(e, handle);
    1
}

// Real threads: replaces KERNEL32.DLL ResetEvent (import)
fn reset_event(e: &mut Engine, handle: u32) -> u32 {
    let Some(o) = object(e, handle) else {
        return locked_call(e, &[handle]).eax;
    };
    let Kind::Event { .. } = o.kind else {
        return 0;
    };
    o.state.store(0, Ordering::SeqCst);
    1
}

// Real threads: replaces KERNEL32.DLL ReleaseSemaphore (import)
/// Adds `release` to the count, as the platform's: 0 without a change when the count is not
/// positive or would pass the maximum; the previous count to `previous` (if not null).
fn release_semaphore(e: &mut Engine, handle: u32, release: u32, previous: Ptr) -> u32 {
    let Some(o) = object(e, handle) else {
        return locked_call(e, &[handle, release, previous.addr()]).eax;
    };
    let Kind::Semaphore { maximum } = o.kind else {
        return 0;
    };
    let release = release as i32;
    let Some(old) = update(&o.state, |count| {
        (release > 0 && i64::from(release) <= i64::from(maximum) - i64::from(count))
            .then(|| count + release)
    }) else {
        return 0;
    };
    o.wake(release as usize);
    others_look(e, handle);
    if !previous.is_null() {
        e.mem.set_i32(previous.addr(), old);
    }
    1
}

// Real threads: replaces KERNEL32.DLL WaitForSingleObject (import)
fn wait_for_single_object(e: &mut Engine, handle: u32, timeout: u32) -> u32 {
    let Some(o) = object(e, handle) else {
        return locked_call(e, &[handle, timeout]).eax;
    };
    wait(e, &[handle], &[o], false, timeout)
}

// Real threads: replaces KERNEL32.DLL WaitForMultipleObjects (import)
/// Here when every handle is an event or a semaphore of this module; else the platform's.
fn wait_for_multiple_objects(
    e: &mut Engine,
    count: u32,
    handles: Ptr,
    all: u32,
    timeout: u32,
) -> u32 {
    let fallback = |e: &mut Engine| locked_call(e, &[count, handles.addr(), all, timeout]).eax;
    if count == 0 || count > 64 {
        return fallback(e);
    }
    let list: Vec<u32> = (0..count)
        .map(|i| e.mem.u32(handles.addr() + 4 * i))
        .collect();
    let mut objects = Vec::with_capacity(list.len());
    for &h in &list {
        match object(e, h) {
            Some(o) => objects.push(o),
            None => return fallback(e),
        }
    }
    wait(e, &list, &objects, all != 0, timeout)
}

/// Takes one of `objects` (the first that is free, by index) or, `all`, every one of them at
/// once: the index taken (0 for all), or `None`.
fn take(objects: &[Arc<Object>], all: bool) -> Option<u32> {
    if !all {
        return objects.iter().position(|o| o.try_take()).map(|i| i as u32);
    }
    if !objects.iter().all(|o| o.available()) {
        return None;
    }
    for (i, o) in objects.iter().enumerate() {
        if !o.try_take() {
            // Another thread took it meanwhile: what this one took goes back.
            for p in &objects[..i] {
                p.give_back();
            }
            return None;
        }
    }
    Some(0)
}

/// A wait on `objects` (`handles`, for the state shown by `NV_REAL_DUMP`): taken at once when it
/// can be; else queued on each, watching briefly ([`watch_briefly`]) and parking until woken, the
/// timeout, or (on the main thread) a game thread's stop, which ends the run as the kernel lock's
/// path does. A safe point; a suspended thread parks here first.
fn wait(e: &mut Engine, handles: &[u32], objects: &[Arc<Object>], all: bool, timeout: u32) -> u32 {
    let w = ctx(e).world;
    w.safe_point();
    park_here(e);
    if let Some(i) = take(objects, all) {
        return WAIT_OBJECT_0 + i;
    }
    if timeout == 0 {
        return WAIT_TIMEOUT;
    }
    let deadline = (timeout != INFINITE).then(|| e.clock_now_us() + u64::from(timeout) * 1000);
    let main = ctx(e).id == MAIN_THREAD_ID;
    let started = lock_stats().map(|_| std::time::Instant::now());
    note_state(e, || {
        let list: Vec<String> = handles.iter().map(|h| format!("{h:x}")).collect();
        format!(
            "waiting on {} of handles [{}]",
            if all { "all" } else { "any" },
            list.join(", ")
        )
    });
    let me = Arc::new(Waiter {
        thread: std::thread::current(),
        handles: handles.to_vec(),
        woken: AtomicBool::new(false),
        all,
    });
    for o in objects {
        guard(&o.queue).push(Arc::clone(&me));
        o.waiting.fetch_add(1, Ordering::SeqCst);
    }
    // A wait inside a kernel-locked platform call (a window procedure the game runs from
    // `DispatchMessageA`, say) gives the kernel lock back while it waits, as the kernel lock's
    // path does, and takes it again after.
    let held = w.kernel.release_all();
    let was = w.leave_guest();
    let mut stop = None;
    let taken = loop {
        if ctx(e).suspend.load(Ordering::Relaxed) {
            park_here(e);
        }
        me.woken.store(false, Ordering::SeqCst);
        std::sync::atomic::fence(Ordering::SeqCst);
        if let Some(i) = take(objects, all) {
            break Some(i);
        }
        if main && w.any_stop.load(Ordering::SeqCst) {
            stop = guard(&w.stopped).clone();
            if stop.is_some() {
                break None;
            }
        }
        let left = match deadline {
            Some(d) => {
                let now = e.clock_now_us();
                if now >= d {
                    break None;
                }
                Some(Duration::from_micros(d - now))
            }
            None => None,
        };
        let watched = std::time::Instant::now();
        if !watch_briefly(&me.woken, left) {
            match left {
                Some(t) => {
                    let rest = t.saturating_sub(watched.elapsed());
                    if !rest.is_zero() {
                        std::thread::park_timeout(rest);
                    }
                }
                None => std::thread::park(),
            }
        }
        // A pass of the blocked wait (watched or parked), counted as the kernel lock's path counts
        // its passes.
        w.wakes.fetch_add(1, Ordering::Relaxed);
        if let Some(s) = lock_stats() {
            s.wakes[role()].fetch_add(1, Ordering::Relaxed);
        }
    };
    for o in objects {
        o.waiting.fetch_sub(1, Ordering::SeqCst);
        guard(&o.queue).retain(|x| !Arc::ptr_eq(x, &me));
    }
    // A wake this thread got and did not use (it took another object, timed out, or another
    // thread took the unit first) goes on to the next waiter.
    for o in objects {
        if o.available() {
            o.wake(1);
        }
    }
    if let (Some(stats), Some(t)) = (lock_stats(), started) {
        stats.object[role()].add(t.elapsed());
        let (kind, arg) = if role() == 0 {
            (1, handles.first().copied().unwrap_or(0))
        } else {
            (3, ctx(e).id)
        };
        stats.wait_at(kind, wait_site(e), arg, t.elapsed());
    }
    if was {
        w.enter_guest();
    }
    w.kernel.take_again(w, held);
    note_state(e, String::new);
    if let Some(why) = stop {
        panic!("{why}");
    }
    taken.map_or(WAIT_TIMEOUT, |i| WAIT_OBJECT_0 + i)
}

/// At real threads' start: the kernel table's events and semaphores so far become this module's,
/// with their state.
pub(super) fn adopt(world: &World, e: &Engine) {
    use crate::units::platform::KernelObject;
    let mut table = guard(&world.sync);
    for (&handle, object) in e.kernel.objects.iter() {
        let made = match *object {
            KernelObject::Event {
                manual_reset,
                signalled,
            } => Object::new(
                Kind::Event {
                    manual: manual_reset,
                },
                i32::from(signalled),
            ),
            KernelObject::Semaphore { count, maximum } => {
                Object::new(Kind::Semaphore { maximum }, count)
            }
            _ => continue,
        };
        table.insert(handle, Arc::new(made));
    }
}

/// Whether events and semaphores go off the kernel lock: unless `NV_REAL_SYNC=0` (the kernel
/// lock's path for them too, as before, for a comparison). Read at real threads' start.
pub(super) fn wanted() -> bool {
    std::env::var("NV_REAL_SYNC").map_or(true, |v| v.trim() != "0")
}

/// For the kernel lock's path of a wait for all (`units/platform.rs`): takes every event and
/// semaphore of this module among `handles` at once, or none (a thread off the kernel lock may
/// have taken one since it was seen free). True when there is none of them.
pub(crate) fn take_all_sync(e: &Engine, handles: &[u32]) -> bool {
    let objects: Vec<Arc<Object>> = handles.iter().filter_map(|&h| sync_object(e, h)).collect();
    objects.is_empty() || take(&objects, true).is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// What a wait for all gives back goes back once and within bounds: a semaphore's count not
    /// past its maximum (a release came in between), an auto-reset event set once.
    #[test]
    fn a_give_back_stays_within_bounds() {
        let s = Object::new(Kind::Semaphore { maximum: 1 }, 1);
        assert!(s.try_take());
        s.state.store(1, Ordering::SeqCst);
        s.give_back();
        assert_eq!(s.state.load(Ordering::SeqCst), 1, "past the maximum");
        let e = Object::new(Kind::Event { manual: false }, 1);
        assert!(e.try_take());
        e.give_back();
        assert_eq!(e.state.load(Ordering::SeqCst), 1);
        e.give_back();
        assert_eq!(e.state.load(Ordering::SeqCst), 1);
    }
}

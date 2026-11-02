//! Per-thread ring buffers and the writer thread.
//!
//! Each game thread that enters a hooked function gets its own ring (single producer: that
//! thread; single consumer: the writer thread). The producer copies a record in and publishes
//! the new head with a release store; it never takes a lock. The writer thread, every few
//! milliseconds, takes what each ring holds and appends it to the file as one block for that
//! thread (format in `nv_trace_core::format`). A full ring makes its producer wait a little
//! for the writer; after about a second it drops the record and counts it (the count is
//! written at the end).
//!
//! The end of the run: the exe's `ExitProcess` import is redirected to [`finish`], which stops
//! the writer, drains every ring and closes the file before the process ends. A process that
//! ends another way (a crash) loses at most what the writer had not yet taken (a few ms).

use nv_win as win;
use std::io::Write;
use std::sync::atomic::{AtomicBool, AtomicPtr, AtomicU64, AtomicUsize, Ordering::*};
use std::sync::Mutex;

const RING_BYTES: usize = 1 << 20;

pub struct Ring {
    tid: u32,
    buf: Box<[u8]>,
    /// Bytes ever written (producer) and ever taken (consumer).
    head: AtomicUsize,
    tail: AtomicUsize,
    next: *mut Ring,
}

// SAFETY: a ring is shared between its producer thread and the writer; the indices are
// atomics and each byte range is touched by one side at a time (head/tail protocol).
unsafe impl Sync for Ring {}
unsafe impl Send for Ring {}

static RINGS: AtomicPtr<Ring> = AtomicPtr::new(std::ptr::null_mut());
static DROPPED: AtomicU64 = AtomicU64::new(0);
static STOP: AtomicBool = AtomicBool::new(false);
static FINISHED: AtomicBool = AtomicBool::new(false);
static FILE: Mutex<Option<std::fs::File>> = Mutex::new(None);
static WRITER: AtomicUsize = AtomicUsize::new(0);

/// A new ring for the calling thread, registered with the writer. Never freed.
pub fn new_ring() -> *mut Ring {
    // SAFETY: plain query.
    let tid = unsafe { win::GetCurrentThreadId() };
    let ring = Box::into_raw(Box::new(Ring {
        tid,
        buf: vec![0u8; RING_BYTES].into_boxed_slice(),
        head: AtomicUsize::new(0),
        tail: AtomicUsize::new(0),
        next: std::ptr::null_mut(),
    }));
    let mut head = RINGS.load(Acquire);
    loop {
        // SAFETY: `ring` is ours until published.
        unsafe { (*ring).next = head };
        match RINGS.compare_exchange_weak(head, ring, AcqRel, Acquire) {
            Ok(_) => return ring,
            Err(h) => head = h,
        }
    }
}

/// Appends one record. Only the ring's own thread calls this.
pub fn push(ring: &Ring, record: &[u8]) {
    if FINISHED.load(Acquire) {
        return;
    }
    let len = record.len();
    let head = ring.head.load(Relaxed);
    let mut waited = 0;
    while RING_BYTES - (head - ring.tail.load(Acquire)) < len {
        if waited >= 1000 || STOP.load(Acquire) {
            DROPPED.fetch_add(1, Relaxed);
            return;
        }
        // SAFETY: plain sleep.
        unsafe { win::Sleep(1) };
        waited += 1;
    }
    let at = head % RING_BYTES;
    let first = len.min(RING_BYTES - at);
    // SAFETY: the bytes from head to head + len are free (checked above) and only this
    // thread writes them; the writer reads them only after the release store below.
    unsafe {
        let base = ring.buf.as_ptr() as *mut u8;
        std::ptr::copy_nonoverlapping(record.as_ptr(), base.add(at), first);
        std::ptr::copy_nonoverlapping(record.as_ptr().add(first), base, len - first);
    }
    ring.head.store(head + len, Release);
}

/// Takes every ring's new bytes into one buffer of blocks.
fn collect(out: &mut Vec<u8>) {
    let mut p = RINGS.load(Acquire);
    while !p.is_null() {
        // SAFETY: rings are never freed.
        let ring = unsafe { &*p };
        let head = ring.head.load(Acquire);
        let tail = ring.tail.load(Relaxed);
        if head > tail {
            let len = head - tail;
            let at = tail % RING_BYTES;
            let first = len.min(RING_BYTES - at);
            out.extend_from_slice(&ring.tid.to_le_bytes());
            out.extend_from_slice(&(len as u32).to_le_bytes());
            out.extend_from_slice(&ring.buf[at..at + first]);
            out.extend_from_slice(&ring.buf[..len - first]);
            ring.tail.store(head, Release);
        }
        p = ring.next;
    }
}

fn drain() {
    let mut out = Vec::new();
    collect(&mut out);
    if out.is_empty() {
        return;
    }
    if let Ok(mut f) = FILE.lock() {
        if let Some(f) = f.as_mut() {
            let _ = f.write_all(&out);
        }
    }
}

static STOP_EVENT: AtomicUsize = AtomicUsize::new(0);

unsafe extern "system" fn writer(_: *mut std::ffi::c_void) -> u32 {
    let ev = STOP_EVENT.load(Acquire) as win::Handle;
    while !STOP.load(Acquire) {
        drain();
        if ev.is_null() {
            win::Sleep(4);
        } else if win::WaitForSingleObject(ev, 4) == win::WAIT_OBJECT_0 {
            // The launcher's time is up: write everything now (it ends the game next).
            crate::log("stop requested: trace drained and closed");
            finish(false);
            return 0;
        }
    }
    0
}

/// Opens the output with its header and starts the writer thread.
pub fn start(path: &std::path::Path, header: &[u8]) -> Result<(), String> {
    let mut f = std::fs::File::create(path).map_err(|e| format!("{}: {e}", path.display()))?;
    f.write_all(header)
        .map_err(|e| format!("{}: {e}", path.display()))?;
    *FILE.lock().unwrap_or_else(|e| e.into_inner()) = Some(f);
    // The launcher's stop event, if it made one (SYNCHRONIZE access).
    let name: Vec<u16> = format!(r"Local\nv-trace-rec-{}-stop", unsafe {
        win::GetCurrentProcessId()
    })
    .encode_utf16()
    .chain(std::iter::once(0))
    .collect();
    // SAFETY: a NUL-terminated name.
    let ev = unsafe { crate::sys::OpenEventW(0x0010_0000, 0, name.as_ptr()) };
    STOP_EVENT.store(ev as usize, Release);
    // SAFETY: a thread that only runs `writer`.
    let h = unsafe {
        win::CreateThread(
            std::ptr::null_mut(),
            0,
            writer,
            std::ptr::null_mut(),
            0,
            std::ptr::null_mut(),
        )
    };
    if h.is_null() {
        return Err("cannot start the writer thread".into());
    }
    WRITER.store(h as usize, Release);
    Ok(())
}

/// Stops the writer, drains every ring and closes the file (once). `wait` waits for the
/// writer thread to finish its current pass (false inside `DllMain`'s detach, where the
/// other threads are already gone).
pub fn finish(wait: bool) {
    if FINISHED.swap(true, AcqRel) {
        return;
    }
    STOP.store(true, Release);
    let h = WRITER.load(Acquire);
    if wait && h != 0 {
        // SAFETY: the writer's handle, kept open.
        unsafe { win::WaitForSingleObject(h as win::Handle, 2000) };
    }
    // After a kill in DllMain's detach the writer may have died holding the lock: then the
    // tail is lost rather than the process hung.
    if let Ok(mut f) = FILE.try_lock() {
        let mut out = Vec::new();
        collect(&mut out);
        if let Some(file) = f.as_mut() {
            let _ = file.write_all(&out);
            let _ = file.flush();
        }
        *f = None;
    }
}

pub fn dropped() -> u64 {
    DROPPED.load(Relaxed)
}

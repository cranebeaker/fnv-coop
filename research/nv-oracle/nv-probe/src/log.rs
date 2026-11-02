//! The probe's log: JSON lines, buffered behind a mutex.

use std::fs::File;
use std::io::Write;
use std::sync::{Mutex, MutexGuard};

pub struct Logger {
    file: File,
    buf: Vec<u8>,
    last_flush: u32,
    flush_ms: u32,
    seq: u64,
}

static LOGGER: Mutex<Option<Logger>> = Mutex::new(None);

const BUFFER_LIMIT: usize = 64 * 1024;

fn tick() -> u32 {
    // SAFETY: plain query.
    unsafe { nv_win::GetTickCount() }
}

impl Logger {
    pub fn open(path: &std::path::Path, flush_ms: u32) -> std::io::Result<Logger> {
        Ok(Logger {
            file: File::create(path)?,
            buf: Vec::with_capacity(BUFFER_LIMIT + 4096),
            last_flush: tick(),
            flush_ms,
            seq: 0,
        })
    }

    /// Add one line and write the buffer out if it is full or `flush_ms`
    /// has passed since the last write.
    pub fn push_line(&mut self, line: &str) {
        self.buf.extend_from_slice(line.as_bytes());
        self.buf.push(b'\n');
        let now = tick();
        if self.buf.len() >= BUFFER_LIMIT
            || self.flush_ms == 0
            || now.wrapping_sub(self.last_flush) >= self.flush_ms
        {
            self.flush();
        }
    }

    pub fn flush(&mut self) {
        if !self.buf.is_empty() {
            let _ = self.file.write_all(&self.buf);
            let _ = self.file.flush();
            self.buf.clear();
        }
        self.last_flush = tick();
    }
}

/// Lock the logger. Held during installation so that no hook record can
/// appear before the header line.
pub fn lock() -> MutexGuard<'static, Option<Logger>> {
    LOGGER.lock().unwrap_or_else(|e| e.into_inner())
}

pub fn append(line: &str) {
    if let Some(l) = lock().as_mut() {
        l.push_line(line);
    }
}

/// Add a record whose line is built from its sequence number. The number is
/// handed out under the lock, so sequence numbers increase in file order even
/// with several threads logging. Returns the number (0 if nothing is logging).
pub fn append_with(build: impl FnOnce(u64) -> String) -> u64 {
    match lock().as_mut() {
        Some(l) => {
            l.seq += 1;
            let seq = l.seq;
            let line = build(seq);
            l.push_line(&line);
            seq
        }
        None => 0,
    }
}

/// Write out buffered lines. Gives up instead of waiting if another thread
/// holds the lock (used while the process is exiting).
pub fn flush_now(wait: bool) {
    let guard = if wait {
        Some(lock())
    } else {
        match LOGGER.try_lock() {
            Ok(g) => Some(g),
            Err(std::sync::TryLockError::Poisoned(p)) => Some(p.into_inner()),
            Err(std::sync::TryLockError::WouldBlock) => None,
        }
    };
    if let Some(mut g) = guard {
        if let Some(l) = g.as_mut() {
            l.flush();
        }
    }
}

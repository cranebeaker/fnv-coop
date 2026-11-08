//! `NV_RACE_CENSUS=FILE` (a diagnostic of the stepped host, not game behaviour; ADR-0015's race
//! gate): which guest words more than one game thread writes, and whether the writes are under a
//! lock. Each store (the memory's store diagnostics) is counted against the word (4 bytes) it
//! starts in, with the writing thread (the stepped host's running thread, [`RaceCensus::set_thread`])
//! and the critical sections it holds then ([`RaceCensus::lock_taken`]). Stores of the interlocked
//! imports are the game's synchronisation, not counted. A word another thread wrote last is
//! *shared*; the report lists the shared words, with how often the writer changed, how many writes
//! held no critical section (the game's own spin locks are not seen: such writes count as
//! unlocked) and the functions that wrote them, grouped by those functions. That sorts them, by
//! reading, into the original's own benign races (counters, flags it rechecks), lock-free data
//! the game orders another way, and real races (`docs/ENGINE_CRATE.md`, real threads).
//!
//! It needs the block check (turned on at frees, reporting, when it is off: `Engine::new`) for the
//! running function. A freshly allocated block forgets its words' writers. The report is written to
//! `FILE` every [`REPORT_EVERY`] of main's frames and when asked ([`RaceCensus::write`]).

use crate::engine::AddrMap;
use std::collections::BTreeMap;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};
use std::sync::Mutex;

/// Main's frames between two writes of the report.
pub const REPORT_EVERY: u64 = 600;

/// What the census keeps of a shared word.
#[derive(Default, Clone)]
struct Shared {
    /// The writing threads (bit `id`, ids below 64).
    threads: u64,
    writes: u64,
    /// Writes with no critical section held.
    unlocked: u64,
    /// Writes by another thread than the one before.
    changes: u64,
    /// The writing functions (at most 6).
    functions: Vec<u32>,
}

#[derive(Default)]
struct State {
    /// The last writer of each written word, by page (a byte per word: the thread id, 0 none).
    last: AddrMap<u32, Box<[u8; 1024]>>,
    shared: AddrMap<u32, Shared>,
    /// Critical sections held, by thread id.
    held: AddrMap<u32, u32>,
}

/// The census (module documentation); kept by the memory (`Mem::enable_race_census`).
pub struct RaceCensus {
    path: String,
    current: AtomicU32,
    interlocked: AtomicBool,
    frames: AtomicU64,
    state: Mutex<State>,
}

/// `NV_RACE_CENSUS`, read once.
pub fn path_from_env() -> Option<String> {
    static PATH: std::sync::OnceLock<Option<String>> = std::sync::OnceLock::new();
    PATH.get_or_init(|| {
        std::env::var("NV_RACE_CENSUS")
            .ok()
            .filter(|p| !p.is_empty())
    })
    .clone()
}

impl RaceCensus {
    pub fn new(path: String) -> Self {
        RaceCensus {
            path,
            current: AtomicU32::new(crate::thread_host::MAIN_THREAD_ID),
            interlocked: AtomicBool::new(false),
            frames: AtomicU64::new(0),
            state: Mutex::new(State::default()),
        }
    }

    fn state(&self) -> std::sync::MutexGuard<'_, State> {
        self.state
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    /// The stepped host switched to thread `id`.
    pub fn set_thread(&self, id: u32) {
        self.current.store(id, Ordering::Relaxed);
    }

    /// An interlocked import runs (`true`) or ended: its store is not counted.
    pub fn interlocked(&self, on: bool) {
        self.interlocked.store(on, Ordering::Relaxed);
    }

    /// The running thread took a critical section.
    pub fn lock_taken(&self) {
        let id = self.current.load(Ordering::Relaxed);
        *self.state().held.entry(id).or_default() += 1;
    }

    /// The running thread gave a critical section back.
    pub fn lock_released(&self) {
        let id = self.current.load(Ordering::Relaxed);
        if let Some(n) = self.state().held.get_mut(&id) {
            *n = n.saturating_sub(1);
        }
    }

    /// A block was allocated at `addr`: its words have no writer yet.
    pub fn forget(&self, addr: u32, len: u32) {
        let mut s = self.state();
        let end = u64::from(addr) + u64::from(len);
        for a in (u64::from(addr & !3)..end).step_by(4) {
            let a = a as u32;
            if let Some(page) = s.last.get_mut(&(a >> 12)) {
                page[((a >> 2) & 1023) as usize] = 0;
            }
            s.shared.remove(&a);
        }
    }

    /// A store of `len` bytes at `a` while `function` runs.
    pub fn store(&self, a: u32, _len: u32, function: u32) {
        if self.interlocked.load(Ordering::Relaxed) {
            return;
        }
        let id = self.current.load(Ordering::Relaxed).min(255);
        let word = a & !3;
        let mut s = self.state();
        let locked = s.held.get(&id).copied().unwrap_or(0) > 0;
        let page = s
            .last
            .entry(word >> 12)
            .or_insert_with(|| Box::new([0; 1024]));
        let slot = &mut page[((word >> 2) & 1023) as usize];
        let before = u32::from(*slot);
        *slot = id as u8;
        if before == 0 || (before == id && !s.shared.contains_key(&word)) {
            return;
        }
        let w = s.shared.entry(word).or_default();
        w.threads |= 1u64 << (before.min(63)) | 1u64 << id.min(63);
        w.writes += 1;
        w.unlocked += u64::from(!locked);
        w.changes += u64::from(before != id);
        if w.functions.len() < 6 && !w.functions.contains(&function) {
            w.functions.push(function);
        }
    }

    /// One of main's frames ended: the report is written every [`REPORT_EVERY`].
    pub fn frame_ended(&self) {
        if self.frames.fetch_add(1, Ordering::Relaxed) % REPORT_EVERY == REPORT_EVERY - 1 {
            self.write();
        }
    }

    /// The report as text (module documentation).
    pub fn report(&self) -> String {
        let s = self.state();
        let mut groups: BTreeMap<Vec<u32>, (u64, u64, u64, u64, u64)> = BTreeMap::new();
        let (mut words, mut all_locked) = (0u64, 0u64);
        for w in s.shared.values().filter(|w| w.changes > 0) {
            words += 1;
            all_locked += u64::from(w.unlocked == 0);
            let mut f = w.functions.clone();
            f.sort_unstable();
            let g = groups.entry(f).or_default();
            g.0 += 1;
            g.1 += w.writes;
            g.2 += w.unlocked;
            g.3 += w.changes;
            g.4 |= w.threads;
        }
        let mut rows: Vec<_> = groups.into_iter().collect();
        rows.sort_by_key(|r| std::cmp::Reverse(r.1 .3));
        let mut out = format!(
            "race census: {words} words written by two or more threads, {all_locked} of them \
             only under a critical section; {} groups by writing functions, most writer changes \
             first\nwords\twrites\tunlocked\tchanges\tthreads\tfunctions\n",
            rows.len()
        );
        for (f, (n, writes, unlocked, changes, threads)) in rows {
            let ids: Vec<String> = (0..64)
                .filter(|i| threads & (1u64 << i) != 0)
                .map(|i| i.to_string())
                .collect();
            let fs: Vec<String> = f.iter().map(|a| format!("{a:08x}")).collect();
            out.push_str(&format!(
                "{n}\t{writes}\t{unlocked}\t{changes}\t{}\t{}\n",
                ids.join(","),
                fs.join(" ")
            ));
        }
        out
    }

    /// Writes the report to the census's file.
    pub fn write(&self) {
        if let Err(err) = std::fs::write(&self.path, self.report()) {
            eprintln!("NV_RACE_CENSUS: {}: {err}", self.path);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn words_two_threads_write_are_counted_with_their_locks() {
        let c = RaceCensus::new(String::new());
        c.store(0x1000, 4, 0x0040_0000);
        c.store(0x1000, 4, 0x0040_0000);
        c.set_thread(2);
        c.store(0x1002, 2, 0x0050_0000);
        c.lock_taken();
        c.store(0x2000, 4, 0x0060_0000);
        c.set_thread(1);
        c.store(0x2000, 4, 0x0060_0000);
        c.interlocked(true);
        c.store(0x3000, 4, 0x0070_0000);
        c.set_thread(2);
        c.store(0x3000, 4, 0x0070_0000);
        c.interlocked(false);
        let r = c.report();
        assert!(r.starts_with("race census: 2 words"), "{r}");
        assert!(r.contains("1\t1\t1\t1\t1,2\t00500000\n"), "{r}");
        // Thread 1's write to 0x2000 held no section.
        assert!(r.contains("1\t1\t1\t1\t1,2\t00600000"), "{r}");
        c.forget(0x1000, 8);
        assert!(c.report().starts_with("race census: 1 words"));
    }
}

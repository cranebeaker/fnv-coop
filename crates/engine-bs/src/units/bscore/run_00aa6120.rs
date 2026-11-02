//! `BSCore/run_00aa6120` (Xbox PDB source unit), subsystem `BSCore`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Operations on an 8-byte read-write lock laid out like the Xbox PDB's
//! `BSReadWriteLock` (`uiLock` +0x00: reader count in the low 28 bits, bit
//! 31 the write flag; `uiWriterThread` +0x04: the writing thread's id).
//! These functions follow `BSReadWriteLock::LockForWrite` (`00aa6060`) in
//! the exe; their own names are not in the engine map, so they are
//! described by what they do. Win32 functions are called through their
//! import slots.

#[allow(unused_imports)]
use crate::prelude::*;

/// Import slot of `GetCurrentThreadId`.
const GET_CURRENT_THREAD_ID: u32 = 0x00fd_f1d0;
/// Import slot of `InterlockedCompareExchange` (destination, exchange, comparand).
const INTERLOCKED_COMPARE_EXCHANGE: u32 = 0x00fd_f1bc;
/// Import slot of `InterlockedDecrement`.
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// Import slot of `Sleep`.
const SLEEP: u32 = 0x00fd_f104;

/// Bit 31 of the lock word: held for writing.
const WRITE_FLAG: u32 = 0x8000_0000;
/// The reader-count bits the read path compares against.
const READER_MASK: u32 = 0x0fff_ffff;

/// The lock word (`uiLock`, +0x00).
fn lock_word(e: &Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr())
}

/// The writer thread id (`uiWriterThread`, +0x04).
fn writer(e: &Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr().wrapping_add(4))
}

fn set_writer(e: &mut Engine, this: Ptr, thread: u32) {
    e.mem.set_u32(this.addr().wrapping_add(4), thread);
}

fn current_thread_id(e: &mut Engine) -> u32 {
    e.call(GET_CURRENT_THREAD_ID, &[]).u32()
}

fn compare_exchange(e: &mut Engine, this: Ptr, exchange: u32, comparand: u32) -> u32 {
    e.call(
        INTERLOCKED_COMPARE_EXCHANGE,
        &args![this, exchange, comparand],
    )
    .u32()
}

// Translated from 00aa6120 (decompiled, FalloutNV.exe 1.4.0.525)
/// Takes the lock for reading. The writing thread itself just increments
/// the word. Others add one reader with a compare-exchange against the
/// word's reader bits, retrying after `Sleep(0)` until it succeeds; false
/// when the write flag is found set after a failed exchange.
pub fn fn_00aa6120(e: &mut Engine, this: Ptr) -> bool {
    let thread = current_thread_id(e);
    if writer(e, this) == thread {
        let word = lock_word(e, this).wrapping_add(1);
        e.mem.set_u32(this.addr(), word);
        return true;
    }
    let mut readers = lock_word(e, this) & READER_MASK;
    loop {
        if compare_exchange(e, this, readers.wrapping_add(1), readers) == readers {
            return true;
        }
        if lock_word(e, this) & WRITE_FLAG != 0 {
            return false;
        }
        e.call(SLEEP, &args![0u32]);
        readers = lock_word(e, this) & READER_MASK;
    }
}

// Translated from 00aa61b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tries once to take the lock for writing. The writing thread itself just
/// increments the word; another thread succeeds only when the word is 0,
/// exchanging it for the write flag plus one and becoming the writer.
pub fn fn_00aa61b0(e: &mut Engine, this: Ptr) -> bool {
    let thread = current_thread_id(e);
    if writer(e, this) == thread {
        let word = lock_word(e, this).wrapping_add(1);
        e.mem.set_u32(this.addr(), word);
        return true;
    }
    let acquired = compare_exchange(e, this, WRITE_FLAG | 1, 0) == 0;
    if acquired {
        set_writer(e, this, thread);
    }
    acquired
}

// Translated from 00aa6220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tries once to take the lock for writing while readers may hold it. The
/// writing thread itself just increments the word. Otherwise, when there
/// is no writer: sets the write flag on the current word (compare-exchange
/// with the value read first; the flagged value comes from a second read),
/// and if that worked, exchanges a bare write flag (no readers left) for
/// the write flag plus one and becomes the writer. False in every other
/// case.
pub fn fn_00aa6220(e: &mut Engine, this: Ptr) -> bool {
    let thread = current_thread_id(e);
    if writer(e, this) == thread {
        let word = lock_word(e, this).wrapping_add(1);
        e.mem.set_u32(this.addr(), word);
        return true;
    }
    let seen = lock_word(e, this);
    if writer(e, this) != 0 {
        return false;
    }
    let flagged = lock_word(e, this) | WRITE_FLAG;
    if compare_exchange(e, this, flagged, seen) != seen {
        return false;
    }
    if compare_exchange(e, this, WRITE_FLAG | 1, WRITE_FLAG) == WRITE_FLAG {
        set_writer(e, this, thread);
        true
    } else {
        false
    }
}

// Translated from 00aa62c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Releases a read hold: `InterlockedDecrement` of the lock word.
pub fn fn_00aa62c0(e: &mut Engine, this: Ptr) {
    e.call(INTERLOCKED_DECREMENT, &args![this]);
}

// Translated from 00aa62e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Releases a write hold: decrements the word (not interlocked); when only
/// the write flag is left, exchanges it for 0 (compare-exchange against
/// the word read again) and clears the writer.
pub fn fn_00aa62e0(e: &mut Engine, this: Ptr) {
    let word = lock_word(e, this).wrapping_sub(1);
    e.mem.set_u32(this.addr(), word);
    if lock_word(e, this) == WRITE_FLAG {
        let current = lock_word(e, this);
        compare_exchange(e, this, 0, current);
        set_writer(e, this, 0);
    }
}

// Translated from 00aa6330 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when the current thread is the lock's writer.
pub fn fn_00aa6330(e: &mut Engine, this: Ptr) -> bool {
    let thread = current_thread_id(e);
    thread == writer(e, this)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00aa6120, fn_00aa6120(Ptr) -> bool),
        entry!(0x00aa61b0, fn_00aa61b0(Ptr) -> bool),
        entry!(0x00aa6220, fn_00aa6220(Ptr) -> bool),
        entry!(0x00aa62c0, fn_00aa62c0(Ptr)),
        entry!(0x00aa62e0, fn_00aa62e0(Ptr)),
        entry!(0x00aa6330, fn_00aa6330(Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const THREAD: u32 = 77;

    /// Doubles: thread id 77, a real single-threaded compare-exchange and
    /// decrement, and a `Sleep` that changes nothing.
    fn lock_engine() -> Engine {
        let mut e = Engine::new();
        e.register(GET_CURRENT_THREAD_ID, |_, _| THREAD.into_ret());
        e.register(INTERLOCKED_COMPARE_EXCHANGE, |e, a| {
            let old = e.mem.u32(a[0]);
            if old == a[2] {
                e.mem.set_u32(a[0], a[1]);
            }
            old.into_ret()
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        e.register(SLEEP, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e
    }

    fn lock(e: &mut Engine, word: u32, writer: u32) -> Ptr {
        let p = Ptr::new(e.mem.alloc(8));
        e.mem.set_u32(p.addr(), word);
        e.mem.set_u32(p.addr() + 4, writer);
        p
    }

    fn count(e: &Engine, addr: u32) -> usize {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .count()
    }

    #[test]
    fn read_lock_paths() {
        let mut e = lock_engine();
        // The writer re-enters.
        let l = lock(&mut e, WRITE_FLAG | 1, THREAD);
        assert!(e.call(0x00aa6120, &args![l]).bool());
        assert_eq!(e.mem.u32(l.addr()), WRITE_FLAG | 2);
        // Free: one reader added.
        let l = lock(&mut e, 2, 0);
        assert!(e.call(0x00aa6120, &args![l]).bool());
        assert_eq!(e.mem.u32(l.addr()), 3);
        // Held for writing by someone else: fails without sleeping.
        let l = lock(&mut e, WRITE_FLAG | 1, 5);
        assert!(!e.call(0x00aa6120, &args![l]).bool());
        assert_eq!(e.mem.u32(l.addr()), WRITE_FLAG | 1);
        assert_eq!(count(&e, SLEEP), 0);
    }

    #[test]
    fn read_lock_retries_after_sleeping() {
        let mut e = lock_engine();
        // Bits 28..30 set: the first exchange (against the masked count)
        // fails, `Sleep(0)` clears them, and the retry succeeds.
        let l = lock(&mut e, 0x1000_0002, 0);
        let addr = l.addr();
        e.register_double(SLEEP, move |e, _| {
            let word = e.mem.u32(addr);
            e.mem.set_u32(addr, word & READER_MASK);
            Ret::default()
        });
        assert!(e.call(0x00aa6120, &args![l]).bool());
        assert_eq!(e.mem.u32(l.addr()), 3);
        assert_eq!(count(&e, SLEEP), 1);
    }

    #[test]
    fn try_write_lock_paths() {
        let mut e = lock_engine();
        let l = lock(&mut e, WRITE_FLAG | 1, THREAD);
        assert!(e.call(0x00aa61b0, &args![l]).bool());
        assert_eq!(e.mem.u32(l.addr()), WRITE_FLAG | 2);

        let l = lock(&mut e, 0, 0);
        assert!(e.call(0x00aa61b0, &args![l]).bool());
        assert_eq!(e.mem.u32(l.addr()), WRITE_FLAG | 1);
        assert_eq!(e.mem.u32(l.addr() + 4), THREAD);

        let l = lock(&mut e, 1, 0);
        assert!(!e.call(0x00aa61b0, &args![l]).bool());
        assert_eq!(e.mem.u32(l.addr()), 1);
        assert_eq!(e.mem.u32(l.addr() + 4), 0);
    }

    #[test]
    fn upgrade_lock_paths() {
        let mut e = lock_engine();
        // The writer re-enters.
        let l = lock(&mut e, WRITE_FLAG | 1, THREAD);
        assert!(e.call(0x00aa6220, &args![l]).bool());
        assert_eq!(e.mem.u32(l.addr()), WRITE_FLAG | 2);
        // Another writer: nothing tried.
        let l = lock(&mut e, WRITE_FLAG | 1, 5);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00aa6220, &args![l]).bool());
        assert_eq!(count(&e, INTERLOCKED_COMPARE_EXCHANGE), 0);
        // No readers: flag set, then taken.
        let l = lock(&mut e, 0, 0);
        assert!(e.call(0x00aa6220, &args![l]).bool());
        assert_eq!(e.mem.u32(l.addr()), WRITE_FLAG | 1);
        assert_eq!(e.mem.u32(l.addr() + 4), THREAD);
        // Readers left: the flag stays set, the second exchange fails.
        let l = lock(&mut e, 2, 0);
        assert!(!e.call(0x00aa6220, &args![l]).bool());
        assert_eq!(e.mem.u32(l.addr()), WRITE_FLAG | 2);
        assert_eq!(e.mem.u32(l.addr() + 4), 0);
    }

    #[test]
    fn upgrade_fails_when_the_word_changes() {
        let mut e = lock_engine();
        let l = lock(&mut e, 0, 0);
        // The first exchange sees another value: nothing more.
        e.register_double(INTERLOCKED_COMPARE_EXCHANGE, |_, _| 9u32.into_ret());
        assert!(!e.call(0x00aa6220, &args![l]).bool());
        assert_eq!(count(&e, INTERLOCKED_COMPARE_EXCHANGE), 1);
        assert_eq!(e.mem.u32(l.addr() + 4), 0);
    }

    #[test]
    fn read_unlock_decrements() {
        let mut e = lock_engine();
        let l = lock(&mut e, 3, 0);
        e.call(0x00aa62c0, &args![l]);
        assert_eq!(e.mem.u32(l.addr()), 2);
    }

    #[test]
    fn write_unlock_releases_on_the_last_hold() {
        let mut e = lock_engine();
        let l = lock(&mut e, WRITE_FLAG | 2, THREAD);
        e.call(0x00aa62e0, &args![l]);
        assert_eq!(e.mem.u32(l.addr()), WRITE_FLAG | 1);
        assert_eq!(e.mem.u32(l.addr() + 4), THREAD);
        assert_eq!(count(&e, INTERLOCKED_COMPARE_EXCHANGE), 0);
        e.call(0x00aa62e0, &args![l]);
        assert_eq!(e.mem.u32(l.addr()), 0);
        assert_eq!(e.mem.u32(l.addr() + 4), 0);
        assert_eq!(count(&e, INTERLOCKED_COMPARE_EXCHANGE), 1);
    }

    #[test]
    fn is_writer() {
        let mut e = lock_engine();
        let l = lock(&mut e, 0, THREAD);
        assert!(e.call(0x00aa6330, &args![l]).bool());
        let l = lock(&mut e, 0, 5);
        assert!(!e.call(0x00aa6330, &args![l]).bool());
    }
}

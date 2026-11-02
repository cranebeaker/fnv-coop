//! `BSCore/bsreadwritelock.cpp` (Xbox PDB source unit), subsystem `BSCore`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Import slots: `GetCurrentThreadId` (`00fdf1d0`),
/// `InterlockedCompareExchange` (`00fdf1bc`: destination, exchange,
/// comparand; the old value) and `Sleep` (`00fdf104`).
const GET_CURRENT_THREAD_ID: u32 = 0x00fd_f1d0;
const INTERLOCKED_COMPARE_EXCHANGE: u32 = 0x00fd_f1bc;
const SLEEP: u32 = 0x00fd_f104;
/// `BSReadWriteLock` (as the code uses it): the lock word at +0 (bit 31 the
/// writer flag, the low 28 bits the reader count) and the writing thread's
/// id at +4.
const LOCK_WORD: u32 = 0x00;
const WRITER_THREAD: u32 = 0x04;

// Translated from 00aa6060 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSReadWriteLock::LockForWrite` (engine map): the thread that already
/// writes only counts the lock word up. Otherwise it sets the writer flag
/// on the lock word with its reader count masked to 28 bits
/// (`InterlockedCompareExchange`, `Sleep(0)` and a fresh read between
/// tries), then waits until the word is exactly `0x80000000` (no readers
/// left) and makes it `0x80000001`, and records its thread id.
pub fn bs_read_write_lock_lock_for_write(e: &mut Engine, this: Ptr) {
    let lock = this.addr() + LOCK_WORD;
    let thread = e.call(GET_CURRENT_THREAD_ID, &args![]).u32();
    if e.mem.u32(this.addr() + WRITER_THREAD) == thread {
        let count = e.mem.u32(lock).wrapping_add(1);
        e.mem.set_u32(lock, count);
        return;
    }
    let mut expected = e.mem.u32(lock) & 0x0fff_ffff;
    let mut desired = e.mem.u32(lock) | 0x8000_0000;
    loop {
        let old = e
            .call(
                INTERLOCKED_COMPARE_EXCHANGE,
                &args![lock, desired, expected],
            )
            .u32();
        if old == expected {
            break;
        }
        e.call(SLEEP, &args![0u32]);
        expected = e.mem.u32(lock) & 0x0fff_ffff;
        desired = e.mem.u32(lock) | 0x8000_0000;
    }
    loop {
        let old = e
            .call(
                INTERLOCKED_COMPARE_EXCHANGE,
                &args![lock, 0x8000_0001u32, 0x8000_0000u32],
            )
            .u32();
        if old == 0x8000_0000 {
            break;
        }
        e.call(SLEEP, &args![0u32]);
    }
    e.mem.set_u32(this.addr() + WRITER_THREAD, thread);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00aa_6060, bs_read_write_lock_lock_for_write(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Doubles the imports: thread 7; the exchange succeeds on a match; the
    /// first `Sleep` takes the readers away (the word's low bits cleared).
    fn setup(e: &mut Engine) {
        e.register(GET_CURRENT_THREAD_ID, |_, _| 7u32.into_ret());
        e.register(INTERLOCKED_COMPARE_EXCHANGE, |e, a| {
            let old = e.mem.u32(a[0]);
            if old == a[2] {
                e.mem.set_u32(a[0], a[1]);
            }
            old.into_ret()
        });
        e.register(SLEEP, |e, _| {
            let lock = e.mem.u32(0x0370_0000);
            let word = e.mem.u32(lock);
            e.mem.set_u32(lock, word & 0x8000_0000);
            Ret::default()
        });
        e.map(0x0370_0000, 4);
    }

    #[test]
    fn the_writing_thread_counts_up() {
        let mut e = Engine::new();
        setup(&mut e);
        let lock = e.mem.alloc(8);
        e.mem.set_u32(lock, 0x8000_0001);
        e.mem.set_u32(lock + 4, 7);
        e.call(0x00aa_6060, &args![lock]);
        assert_eq!(e.mem.u32(lock), 0x8000_0002);
    }

    #[test]
    fn a_free_lock_is_taken_at_once() {
        let mut e = Engine::new();
        setup(&mut e);
        let lock = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        e.call(0x00aa_6060, &args![lock]);
        assert_eq!(e.mem.u32(lock), 0x8000_0001);
        assert_eq!(e.mem.u32(lock + 4), 7);
        let log = e.call_log.take().unwrap();
        let exchanges: Vec<_> = log
            .iter()
            .filter(|(a, _)| *a == INTERLOCKED_COMPARE_EXCHANGE)
            .map(|(_, w)| w.clone())
            .collect();
        assert_eq!(
            exchanges,
            vec![
                vec![lock, 0x8000_0000, 0],
                vec![lock, 0x8000_0001, 0x8000_0000]
            ]
        );
        assert!(!log.iter().any(|(a, _)| *a == SLEEP));
    }

    #[test]
    fn a_read_lock_flags_then_waits_for_the_readers() {
        let mut e = Engine::new();
        setup(&mut e);
        let lock = e.mem.alloc(8);
        e.mem.set_u32(0x0370_0000, lock);
        // Two readers, a stale bit 28 masked from the comparand.
        e.mem.set_u32(lock, 0x1000_0002);
        e.call_log = Some(vec![]);
        e.call(0x00aa_6060, &args![lock]);
        assert_eq!(e.mem.u32(lock), 0x8000_0001);
        let log = e.call_log.take().unwrap();
        let exchanges: Vec<_> = log
            .iter()
            .filter(|(a, _)| *a == INTERLOCKED_COMPARE_EXCHANGE)
            .map(|(_, w)| w.clone())
            .collect();
        // The first try fails on the masked comparand; after the sleep the
        // word is 0 and the flag goes on, then the writer takes it.
        assert_eq!(
            exchanges,
            vec![
                vec![lock, 0x9000_0002, 0x0000_0002],
                vec![lock, 0x8000_0000, 0],
                vec![lock, 0x8000_0001, 0x8000_0000]
            ]
        );
    }
}

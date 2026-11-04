//! `fallout shared/run_0040fbf0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The game's recursive lock: the word at `+0` of the lock holds the id of
//! the owning thread (0 when free) and the word at `+4` the recursion count.
//! Acquiring is `0040fbf0`, releasing `0040fba0`.

#[allow(unused_imports)]
use crate::prelude::*;

/// `GetCurrentThreadId` wrapper (`0040fc90`, no arguments).
const CURRENT_THREAD_ID: u32 = 0x0040_fc90;
/// `InterlockedCompareExchange` wrapper (`0043b460`, `cdecl`): arguments
/// (destination, comparand, exchange); returns the previous value.
const COMPARE_EXCHANGE: u32 = 0x0043_b460;
/// `Sleep` wrapper (`0040fca0`, `cdecl`): one argument, the milliseconds.
const SLEEP: u32 = 0x0040_fca0;
/// An empty function (`0040fbe0`) that the lock calls after acquiring.
const AFTER_ACQUIRE: u32 = 0x0040_fbe0;

/// Failed attempts after which the wait sleeps a millisecond instead of
/// yielding (`0x2710`).
const SPINS_BEFORE_SLEEPING: u32 = 0x2710;

// Translated from 0040fbf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Acquires the recursive lock. `__thiscall` with one stack word
/// (`RET 4`) that the function never reads (callers pass the address of a
/// lock-name string). If the calling thread already owns the lock only the
/// count at `+4` is incremented. Otherwise the owner word is claimed with a
/// compare-and-exchange (0 to the thread id); while it fails the thread
/// sleeps 0 ms, and 1 ms once more than `0x2710` attempts have failed. The
/// count is then set to 1.
pub fn fn_0040fbf0(e: &mut Engine, this: Ptr, _unused_1: u32) {
    let thread = e.call(CURRENT_THREAD_ID, &args![]).u32();
    if e.mem.u32(this.addr()) == thread {
        let count = e.mem.u32(this.addr() + 4).wrapping_add(1);
        e.mem.set_u32(this.addr() + 4, count);
    } else {
        let mut attempts = 0u32;
        while e.call(COMPARE_EXCHANGE, &args![this, 0u32, thread]).u32() != 0 {
            attempts = attempts.wrapping_add(1);
            if attempts > SPINS_BEFORE_SLEEPING {
                e.call(SLEEP, &args![1u32]);
            } else {
                e.call(SLEEP, &args![0u32]);
            }
        }
        e.call(AFTER_ACQUIRE, &args![]);
        e.mem.set_u32(this.addr() + 4, 1);
    }
}

/// The import slot of `KERNEL32!Sleep` (`00fdf104`).
const SLEEP_IMPORT: u32 = 0x00fd_f104;

// Translated from 0040fca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Sleep` wrapper (unnamed in the map; `cdecl`, one word): calls the
/// imported `Sleep` with the milliseconds.
pub fn fn_0040fca0(e: &mut Engine, milliseconds: u32) {
    e.call(SLEEP_IMPORT, &args![milliseconds]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0040fbf0, fn_0040fbf0(Ptr, u32)),
        entry!(0x0040fca0, fn_0040fca0(u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const THREAD: u32 = 7;

    type Shared<T> = Rc<RefCell<T>>;

    /// Doubles for the callees; `busy` is how many compare-exchange attempts
    /// fail before one succeeds. The returned log holds the sleeps (the
    /// milliseconds) and the number of attempts.
    fn lock_engine(busy: u32) -> (Engine, Shared<Vec<u32>>, Shared<u32>) {
        let mut e = Engine::new();
        e.register(CURRENT_THREAD_ID, |_, _| Ret {
            eax: THREAD,
            ..Ret::default()
        });
        let attempts = Rc::new(RefCell::new(0u32));
        let counter = attempts.clone();
        e.register_double(COMPARE_EXCHANGE, move |e, a| {
            *counter.borrow_mut() += 1;
            if *counter.borrow() <= busy {
                return Ret {
                    eax: 99,
                    ..Ret::default()
                };
            }
            // The exchange succeeds only on a free lock: (dest, comparand, new).
            let old = e.mem.u32(a[0]);
            if old == a[1] {
                e.mem.set_u32(a[0], a[2]);
            }
            Ret {
                eax: old,
                ..Ret::default()
            }
        });
        let sleeps = Rc::new(RefCell::new(Vec::new()));
        let log = sleeps.clone();
        e.register_double(SLEEP, move |_, a| {
            log.borrow_mut().push(a[0]);
            Ret::default()
        });
        e.register(AFTER_ACQUIRE, |_, _| Ret::default());
        (e, sleeps, attempts)
    }

    #[test]
    fn a_free_lock_is_taken_with_count_one() {
        let (mut e, sleeps, attempts) = lock_engine(0);
        let lock: Ptr = Ptr::new(e.mem.alloc(8));
        e.call_log = Some(vec![]);
        e.call(0x0040_fbf0, &args![lock, 0x0101_4304u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(lock.addr()), THREAD);
        assert_eq!(e.mem.u32(lock.addr() + 4), 1);
        assert!(sleeps.borrow().is_empty());
        assert_eq!(*attempts.borrow(), 1);
        // The empty function runs after the acquisition.
        assert_eq!(log.last().unwrap().0, AFTER_ACQUIRE);
    }

    #[test]
    fn the_owner_only_counts_up() {
        let (mut e, _, attempts) = lock_engine(0);
        let lock: Ptr = Ptr::new(e.mem.alloc(8));
        e.mem.set_u32(lock.addr(), THREAD);
        e.mem.set_u32(lock.addr() + 4, 2);
        e.call(0x0040_fbf0, &args![lock, 0u32]);
        assert_eq!(e.mem.u32(lock.addr() + 4), 3);
        assert_eq!(*attempts.borrow(), 0);
    }

    #[test]
    fn a_contended_lock_yields_then_sleeps_after_the_spin_limit() {
        let busy = SPINS_BEFORE_SLEEPING + 2;
        let (mut e, sleeps, attempts) = lock_engine(busy);
        let lock: Ptr = Ptr::new(e.mem.alloc(8));
        e.mem.set_u32(lock.addr(), 5);
        // Another thread (5) owns it until the double lets the exchange work:
        // free it when the busy attempts are over.
        e.register_double(COMPARE_EXCHANGE, {
            let counter = attempts.clone();
            move |e, a| {
                *counter.borrow_mut() += 1;
                if *counter.borrow() <= busy {
                    Ret {
                        eax: 5,
                        ..Ret::default()
                    }
                } else {
                    e.mem.set_u32(a[0], a[2]);
                    Ret::default()
                }
            }
        });
        e.call(0x0040_fbf0, &args![lock, 0u32]);
        let sleeps = sleeps.borrow();
        assert_eq!(sleeps.len() as u32, busy);
        assert_eq!(sleeps[0], 0);
        assert_eq!(sleeps[SPINS_BEFORE_SLEEPING as usize - 1], 0);
        assert_eq!(sleeps[SPINS_BEFORE_SLEEPING as usize], 1);
        assert_eq!(*sleeps.last().unwrap(), 1);
        assert_eq!(e.mem.u32(lock.addr()), THREAD);
        assert_eq!(e.mem.u32(lock.addr() + 4), 1);
    }

    #[test]
    fn sleep_wrapper_passes_the_milliseconds_to_the_import() {
        let mut e = Engine::new();
        e.register(SLEEP_IMPORT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(SLEEP, &args![25u32]);
        let log = e.call_log.unwrap();
        let sleeps: Vec<_> = log.iter().filter(|c| c.0 == SLEEP_IMPORT).collect();
        assert_eq!(sleeps.len(), 1);
        assert_eq!(sleeps[0].1, vec![25]);
    }
}

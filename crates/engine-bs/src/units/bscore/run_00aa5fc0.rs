//! `BSCore/run_00aa5fc0` (Xbox PDB source unit), subsystem `BSCore`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00aa5fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: clears the two words at `this` and returns
/// `this` (a two-word constructor; `CombatCoverMap`'s constructor builds its
/// +0x14 member with it).
pub fn fn_00aa5fc0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), 0);
    e.mem.set_u32(this.addr() + 4, 0);
    this
}

/// Import slots of KERNEL32 `GetCurrentThreadId` (no words), `InterlockedCompareExchange`
/// (destination, exchange, comparand) and `Sleep` (one word); all stdcall.
const GET_CURRENT_THREAD_ID: u32 = 0x00fd_f1d0;
const INTERLOCKED_COMPARE_EXCHANGE: u32 = 0x00fd_f1bc;
const SLEEP: u32 = 0x00fd_f104;

// Translated from 00aa5ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a lock's enter on the two words `00aa5fc0` clears
/// (`thiscall`, no stack words): when the second word is the calling thread's id the count
/// (first word) goes up by one; otherwise it raises the count's low 28 bits by one with
/// `InterlockedCompareExchange`, calling `Sleep(0)` and reading the count again until the
/// exchange finds the value it expected. The owner word is not written here.
pub fn fn_00aa5ff0(e: &mut Engine, this: Ptr) {
    let lock = this.addr();
    let thread = e.call(GET_CURRENT_THREAD_ID, &args![]).u32();
    if e.mem.u32(lock + 4) == thread {
        let count = e.mem.u32(lock);
        e.mem.set_u32(lock, count.wrapping_add(1));
        return;
    }
    let mut expected = e.mem.u32(lock) & 0x0fff_ffff;
    loop {
        let found = e
            .call(
                INTERLOCKED_COMPARE_EXCHANGE,
                &args![lock, expected.wrapping_add(1), expected],
            )
            .u32();
        if found == expected {
            return;
        }
        e.call(SLEEP, &args![0u32]);
        expected = e.mem.u32(lock) & 0x0fff_ffff;
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00aa_5ff0, fn_00aa5ff0(Ptr)),
        entry!(0x00aa_5fc0, fn_00aa5fc0(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clears_both_words_and_returns_this() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xc);
        for i in 0..3 {
            e.mem.set_u32(this + 4 * i, 0xffff_ffff);
        }
        assert_eq!(e.call(0x00aa_5fc0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), 0);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u32(this + 8), 0xffff_ffff);
    }
}

#[cfg(test)]
mod n082_tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    #[test]
    fn fn_00aa5ff0_counts_up_for_the_owner_and_exchanges_otherwise() {
        let mut e = Engine::new();
        e.register(GET_CURRENT_THREAD_ID, |_, _| 0x44u32.into_ret());
        e.register(SLEEP, |_, _| Ret::default());
        // The exchange fails `failures` times (another thread bumped the count), then works.
        let failures = Rc::new(Cell::new(0u32));
        let left = failures.clone();
        e.register_double(INTERLOCKED_COMPARE_EXCHANGE, move |e, a| {
            let old = e.mem.u32(a[0]);
            if left.get() > 0 {
                left.set(left.get() - 1);
                e.mem.set_u32(a[0], old + 1);
                return (old + 1).into_ret();
            }
            if old == a[2] {
                e.mem.set_u32(a[0], a[1]);
            }
            old.into_ret()
        });
        let lock = e.mem.alloc(8);
        e.mem.set_u32(lock, 2);
        e.mem.set_u32(lock + 4, 0x44);
        e.call_log = Some(vec![]);
        e.call(0x00aa_5ff0, &args![lock]);
        assert_eq!(e.mem.u32(lock), 3);
        assert!(!e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .any(|c| c.0 == INTERLOCKED_COMPARE_EXCHANGE));
        // Another owner: exchanges until the count it read is still there.
        e.mem.set_u32(lock, 5);
        e.mem.set_u32(lock + 4, 0x99);
        failures.set(2);
        e.call_log = Some(vec![]);
        e.call(0x00aa_5ff0, &args![lock]);
        let log = e.call_log.take().unwrap();
        let exchanges: Vec<_> = log
            .iter()
            .filter(|c| c.0 == INTERLOCKED_COMPARE_EXCHANGE)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(
            exchanges,
            vec![vec![lock, 6, 5], vec![lock, 7, 6], vec![lock, 8, 7],]
        );
        assert_eq!(log.iter().filter(|c| c.0 == SLEEP).count(), 2);
        assert_eq!(e.mem.u32(lock), 8);
        assert_eq!(e.mem.u32(lock + 4), 0x99);
    }
}

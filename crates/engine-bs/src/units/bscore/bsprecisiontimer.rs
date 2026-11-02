//! `BSCore/bsprecisiontimer.cpp` (Xbox PDB source unit), subsystem `BSCore`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `QueryPerformanceCounter` import slot (`stdcall`, one word: the address of the 64-bit
/// counter).
const QUERY_PERFORMANCE_COUNTER: u32 = 0x00fd_f0a0;

// Translated from 00aa4d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSPrecisionTimer::GetTimer` (Xbox PDB): no arguments; reads the 64-bit performance counter
/// into a zero-initialised local and answers it in `EDX:EAX`.
pub fn bs_precision_timer_get_timer(e: &mut Engine) -> u64 {
    e.with_stack(8, |e, counter| {
        e.mem.set_u64(counter.addr(), 0);
        e.call(QUERY_PERFORMANCE_COUNTER, &args![counter]);
        e.mem.u64(counter.addr())
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00aa_4d80, bs_precision_timer_get_timer() -> u64)]
}

#[cfg(test)]
mod physics_work_ahead_tests {
    use super::*;

    #[test]
    fn at_00aa4d80_answers_the_counter_the_import_wrote() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(QUERY_PERFORMANCE_COUNTER, |e, a| {
            // The local starts out zero.
            assert_eq!(e.mem.u64(a[0]), 0);
            e.mem.set_u64(a[0], 0x0000_0123_89ab_cdef);
            1u32.into_ret()
        });
        assert_eq!(e.call(0x00aa_4d80, &args![]).u64(), 0x0000_0123_89ab_cdef);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log.iter()
                .filter(|c| c.0 == QUERY_PERFORMANCE_COUNTER)
                .count(),
            1
        );
    }

    #[test]
    fn at_00aa4d80_answers_zero_when_the_import_writes_nothing() {
        let mut e = Engine::new();
        e.register(QUERY_PERFORMANCE_COUNTER, |_, _| Ret::default());
        assert_eq!(e.call(0x00aa_4d80, &args![]).u64(), 0);
    }
}

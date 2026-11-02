//! `(unplaced)/run_0061b9d0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `__time64` (`00ecadda`, `units::crt`): `cdecl`, one stack word, the 64-bit time in `EDX:EAX`.
const TIME64: u32 = 0x00ec_adda;

// Translated from 0061b9d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `cdecl` wrapper of `__time64` (`00ecadda`): passes its one stack
/// word, the address that also receives the time (it may be null), and returns the 64-bit time.
pub fn fn_0061b9d0(e: &mut Engine, time: Ptr) -> u64 {
    e.call(TIME64, &args![time]).u64()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0061b9d0, fn_0061b9d0(Ptr) -> u64)]
}

#[cfg(test)]
mod physics_work_ahead_tests {
    use super::*;

    #[test]
    fn at_0061b9d0_forwards_the_pointer_and_returns_the_64_bit_time() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00ec_adda, |_, _| Ret {
            eax: 0x89ab_cdef,
            edx: 0x0123_4567,
            st0: 0.0,
        });
        let time = e.call(0x0061_b9d0, &args![0x1234_5678u32]).u64();
        assert_eq!(time, 0x0123_4567_89ab_cdef);
        let log = e.call_log.take().unwrap();
        let forwarded: Vec<Vec<u32>> = log
            .iter()
            .filter(|c| c.0 == 0x00ec_adda)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(forwarded, vec![vec![0x1234_5678]]);
    }
}

//! `fallout shared/havok/run_0062e230` (Xbox PDB source unit), subsystem `fallout shared/havok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `004a4960(this, first, second, third)` (`RET 0xc`): stores `first` at
/// `+0`, `second` at `+4` and `third | 0x80000000` at `+8` (the top bit
/// marks storage the array does not own).
const ARRAY_INIT: u32 = 0x004a_4960;

// Translated from 0062e230 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets up an array header over storage it does not own: passes its three
/// words (data pointer, size, capacity) on to `004a4960`, which stores them
/// at `+0`, `+4` and `+8`. Returns `this`. (The decompiler shows this
/// address as `CStringT<>` because the linker folded it with a function
/// of that shape.)
pub fn fn_0062e230(e: &mut Engine, this: Ptr, data: u32, size: u32, capacity: u32) -> Ptr {
    e.call(ARRAY_INIT, &args![this, data, size, capacity]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0062_e230, fn_0062e230(Ptr, u32, u32, u32) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn words_go_to_the_initializer_in_order_and_this_comes_back() {
        let mut e = Engine::new();
        e.register(ARRAY_INIT, |_, _| 0xdead_beefu32.into_ret());
        e.call_log = Some(vec![]);
        let result = e.call(0x0062_e230, &args![0x4000u32, 0x4010u32, 3u32, 8u32]);
        assert_eq!(result.u32(), 0x4000);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == ARRAY_INIT).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, vec![0x4000, 0x4010, 3, 8]);
    }
}

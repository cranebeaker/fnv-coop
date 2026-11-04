//! `fallout/magic/run_00825740` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00825740 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `0080d930` of unit run_0080d880, which is itself
/// a forwarder to `00803ed0`, and returns nothing. Parameters: one word, `this`; the
/// disassembly of the function ends in a plain `RET` and passes only `ECX` on. `0080d930` is
/// translated in this family as `fn_0080d930(Ptr)` and returns nothing.
pub fn fn_00825740(e: &mut Engine, this: Ptr) {
    e.call(0x0080_d930, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0082_5740, fn_00825740(Ptr))]
}

#[cfg(test)]
mod thunk_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn fwd_a03_00825740_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x0080_d930, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0082_5740, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x0080_d930), vec![vec![0x5000]]);
    }
}

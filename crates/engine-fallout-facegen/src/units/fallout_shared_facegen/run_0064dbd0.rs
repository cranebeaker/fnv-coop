//! `fallout shared/facegen/run_0064dbd0` (Xbox PDB source unit), subsystem `fallout shared/facegen`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0064dbd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `0064dda0`, which checks two conditions (calling
/// `00ec7c56` when one fails) and answers the word at `+4`, and returns its result (a
/// `u32`). Parameters: one word, `this`; the disassembly of the function ends in a plain
/// `RET` and passes only `ECX` on. The return type (`u32`) is from `0064dda0`'s disassembly,
/// which ends `MOV EAX,[EAX+4]` (the word at `+4` of `this`).
pub fn fn_0064dbd0(e: &mut Engine, this: Ptr) -> u32 {
    e.call(0x0064_dda0, &args![this]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0064_dbd0, fn_0064dbd0(Ptr) -> u32)]
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
    fn fwd_a03_0064dbd0_returns_the_callees_word() {
        let mut e = Engine::new();
        e.register(0x0064_dda0, |_, _| Ret {
            eax: 0x1234_5678,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0064_dbd0, &args![0x0000_5000u32]).u32(),
            0x1234_5678
        );
        assert_eq!(calls_to(&e, 0x0064_dda0), vec![vec![0x5000]]);
    }
}

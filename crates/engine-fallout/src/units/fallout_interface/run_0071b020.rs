//! `fallout/interface/run_0071b020` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0071b020 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `0071dec0` of unit run_0071dd50 (the string
/// list's destructor), and returns nothing. Parameters: one word, `this`; the disassembly of
/// the function ends in a plain `RET` and passes only `ECX` on. `0071dec0`'s translated
/// signature `fn_0071dec0(Ptr<NiTPointerList>)` returns nothing.
pub fn fn_0071b020(e: &mut Engine, this: Ptr) {
    e.call(0x0071_dec0, &args![this]);
}

/// `007b3fa0` (`thiscall`, one word: the flags): a scalar deleting destructor whose body is
/// empty (unit `racesexmenu.cpp`).
const DELETE_007B3FA0: u32 = 0x007b_3fa0;

// Translated from 0071b040 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): a destructor body. When the word
/// at +0 of `this` holds an object, deletes it (`007b3fa0` with 1); then destroys the string
/// lists at +0x10 and at +4 ([`fn_0071b020`]).
// Translation incomplete: 0071b040 - the C++ exception frame (FS:[0] chain, unwind states 1 and 0 for the two lists) is not translated.
pub fn fn_0071b040(e: &mut Engine, this: Ptr) {
    let object = e.mem.u32(this.addr());
    if object != 0 {
        e.call(DELETE_007B3FA0, &args![object, 1u32]);
    }
    fn_0071b020(e, this.byte_add(0x10));
    fn_0071b020(e, this.byte_add(4));
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0071_b020, fn_0071b020(Ptr)),
        entry!(0x0071_b040, fn_0071b040(Ptr)),
    ]
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
    fn fwd_a03_0071b020_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x0071_dec0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0071_b020, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x0071_dec0), vec![vec![0x5000]]);
    }
}

#[cfg(test)]
mod destructor_tests {
    use super::*;

    #[test]
    fn fn_0071b040_deletes_the_object_then_destroys_both_lists() {
        for (object, deletes) in [(0x0e00_9000u32, true), (0, false)] {
            let mut e = Engine::new();
            e.register(DELETE_007B3FA0, |_, _| Ret::default());
            e.register(0x0071_dec0, |_, _| Ret::default());
            let this = e.mem.alloc(0x20);
            e.mem.set_u32(this, object);
            e.call_log = Some(vec![]);
            e.call(0x0071_b040, &args![this]);
            let log = e.call_log.take().unwrap();
            let mut expected = vec![];
            if deletes {
                expected.push((DELETE_007B3FA0, vec![object, 1]));
            }
            expected.push((0x0071_dec0, vec![this + 0x10]));
            expected.push((0x0071_dec0, vec![this + 4]));
            assert_eq!(log[1..].to_vec(), expected);
        }
    }
}

//! `fallout/ai/run_0096a850` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0096a850 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0096a850 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of an object of this unit (the engine map gives it no name; Ghidra shows one
/// word, `this` in `ECX`, and the function ends in a plain `RET`). It runs the array
/// constructor `0096af30(this, 0, 1)` (capacity 0, second word 1), sizes the array to 1000
/// elements with `0096ad30`, stores a null pointer into each of the 1000 slots with `0096ae90`
/// (a null pointer kept in a stack word passed by address), then clears three tables of four
/// words at `+0x10`, `+0x20` and `+0x30`. Returns `this`. The exception-unwinding frame is not
/// translated.
pub fn fn_0096a850(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x0096_af30, &args![this, 0u32, 1u32]);
    e.call(0x0096_ad30, &args![this, 1000u32]);
    e.with_stack(4, |e, null_word| {
        for index in 0..1000u32 {
            e.mem.set_u32(null_word.addr(), 0);
            e.call(0x0096_ae90, &args![this, index, null_word]);
        }
    });
    let base = this.addr();
    for index in 0..4u32 {
        e.mem.set_u32(base + 0x10 + index * 4, 0);
        e.mem.set_u32(base + 0x20 + index * 4, 0);
        e.mem.set_u32(base + 0x30 + index * 4, 0);
    }
    this
}

// Translated from 0096a930 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `0096ad00`, which stores the vtable `0108c0c8` at
/// `+0` and frees the block held at `+4` through the cdecl `004ede70`, and returns nothing.
/// Parameters: one word, `this`; the disassembly of the function ends in a plain `RET` and
/// passes only `ECX` on. `0096ad00`'s disassembly ends with the stack cleanup of that call
/// (`ADD ESP,4`) and computes no result (Ghidra types it void).
pub fn fn_0096a930(e: &mut Engine, this: Ptr) {
    e.call(0x0096_ad00, &args![this]);
}

// Translated from 0096a950 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `0096a930` of this unit, and returns nothing.
/// Parameters: one word, `this`; the disassembly of the function ends in a plain `RET` and
/// passes only `ECX` on. `0096a930` is a function of this unit and returns nothing (see its
/// own doc).
pub fn fn_0096a950(e: &mut Engine, this: Ptr) {
    fn_0096a930(e, this);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0096_a850, fn_0096a850(Ptr) -> Ptr),
        entry!(0x0096_a930, fn_0096a930(Ptr)),
        entry!(0x0096_a950, fn_0096a950(Ptr)),
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
    fn fwd_a03_0096a930_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x0096_ad00, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0096_a930, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x0096_ad00), vec![vec![0x5000]]);
    }

    #[test]
    fn fwd_a03_0096a950_runs_the_unit_function_and_through_it_the_destructor() {
        let mut e = Engine::new();
        e.register(0x0096_ad00, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0096_a950, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x0096_ad00), vec![vec![0x5000]]);
        // The call to the unit's own function is direct, not through the table.
        assert!(calls_to(&e, 0x0096_a930).is_empty());
    }
}

#[cfg(test)]
mod process_list_tests {
    use super::*;

    #[test]
    fn constructor_builds_the_array_and_clears_the_tables() {
        let mut e = Engine::new();
        e.register(0x0096_af30, |_, _| Ret::default());
        e.register(0x0096_ad30, |_, _| Ret::default());
        e.register_double(0x0096_ae90, |e, a| {
            // The null word is passed by address and holds 0 at each call.
            assert_eq!(e.mem.u32(a[2]), 0);
            e.mem.set_u32(a[2], 0xdead_beef);
            Ret::default()
        });
        let this = e.mem.alloc(0x40);
        e.mem.write(this, &[0xaa; 0x40]);
        e.call_log = Some(vec![]);
        let result = e.call(0x0096_a850, &args![this]).u32();
        assert_eq!(result, this);
        let log = e.call_log.take().unwrap();
        let to = |address: u32| -> Vec<Vec<u32>> {
            log.iter()
                .filter(|(a, _)| *a == address)
                .map(|(_, w)| w.clone())
                .collect()
        };
        assert_eq!(to(0x0096_af30), vec![vec![this, 0, 1]]);
        assert_eq!(to(0x0096_ad30), vec![vec![this, 1000]]);
        let sets = to(0x0096_ae90);
        assert_eq!(sets.len(), 1000);
        assert_eq!(sets[0][..2], [this, 0]);
        assert_eq!(sets[999][..2], [this, 999]);
        for index in 0..4 {
            assert_eq!(e.mem.u32(this + 0x10 + index * 4), 0);
            assert_eq!(e.mem.u32(this + 0x20 + index * 4), 0);
            assert_eq!(e.mem.u32(this + 0x30 + index * 4), 0);
        }
        // Bytes outside the tables are untouched.
        assert_eq!(e.mem.u32(this + 4), 0xaaaa_aaaa);
    }
}

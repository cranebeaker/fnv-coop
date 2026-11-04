//! `fallout/ai/run_0096a700` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0096a730 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `0096a750`, which stores the vtable `0108b4e4` at
/// `+0` and frees the block held at `+4` through the cdecl `004ede70`, and returns nothing.
/// Parameters: one word, `this`; the disassembly of the function ends in a plain `RET` and
/// passes only `ECX` on. `0096a750`'s disassembly ends with the stack cleanup of that call
/// (`ADD ESP,4`) and computes no result (Ghidra types it void).
pub fn fn_0096a730(e: &mut Engine, this: Ptr) {
    e.call(0x0096_a750, &args![this]);
}

/// `operator delete` (cdecl, one word).
const OPERATOR_DELETE: u32 = 0x0040_1030;

// Translated from 0096a700 (decompiled, FalloutNV.exe 1.4.0.525)
/// A scalar deleting destructor: `fn_0096a730` (the body), then
/// `operator delete(this)` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_0096a700(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_0096a730(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 0096a750 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor body of the `NiTArray<TESRegion *,
/// NiTMallocInterface<TESRegion *>>` whose vtable is `0108b4e4` (RTTI read at
/// the locator): the vtable, then the element block (+4) freed through the
/// cdecl `004ede70`.
pub fn fn_0096a750(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0108_b4e4);
    let block = e.mem.u32(this.addr() + 4);
    e.call(0x004e_de70, &args![block]);
}

// Translated from 0096a780 (decompiled, FalloutNV.exe 1.4.0.525)
/// The array's scalar deleting destructor: `fn_0096a750`, then
/// `operator delete(this)` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_0096a780(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_0096a750(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0096_a730, fn_0096a730(Ptr)),
        entry!(0x0096_a700, fn_0096a700(Ptr, u32) -> Ptr),
        entry!(0x0096_a750, fn_0096a750(Ptr)),
        entry!(0x0096_a780, fn_0096a780(Ptr, u32) -> Ptr),
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
    fn fwd_a03_0096a730_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x0096_a750, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0096_a730, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x0096_a750), vec![vec![0x5000]]);
    }
}

#[cfg(test)]
mod n028_tests {
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
    fn the_array_destructors_free_the_block_then_maybe_this() {
        let mut e = Engine::new();
        e.register(0x004e_de70, |_, _| Ret::default());
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 4, 0xb10c);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0096_a780, &[this, 0]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x0108_b4e4);
        assert_eq!(calls_to(&e, 0x004e_de70), vec![vec![0xb10c]]);
        assert!(calls_to(&e, OPERATOR_DELETE).is_empty());
        assert_eq!(e.call(0x0096_a780, &[this, 1]).u32(), this);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![this]]);
    }

    #[test]
    fn fn_0096a700_runs_the_forwarder_then_maybe_deletes() {
        let mut e = Engine::new();
        e.register(0x0096_a750, |_, _| Ret::default());
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0096_a700, &[0x9000, 1]);
        assert_eq!(calls_to(&e, 0x0096_a750), vec![vec![0x9000]]);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![0x9000]]);
        e.call(0x0096_a700, &[0x9000, 2]);
        assert_eq!(calls_to(&e, OPERATOR_DELETE).len(), 1);
    }
}

//! `fallout shared/pathfinding/run_006a68b0` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 006a68b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `006a68d0`, which calls the smart-pointer
/// destructor `0045cec0` on the member at `this + 8`, and returns nothing. Parameters: one
/// word, `this`; the disassembly of the function ends in a plain `RET` and passes only `ECX`
/// on. `006a68d0` ends right after that call, and `0045cec0`'s translated signature
/// `fn_0045cec0(Ptr)` returns nothing.
pub fn fn_006a68b0(e: &mut Engine, this: Ptr) {
    e.call(0x006a_68d0, &args![this]);
}

// Translated from 006a68d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys the smart pointer at +8 of `this` (`0045cec0`; the decompiler's
/// library name for it does not apply).
pub fn fn_006a68d0(e: &mut Engine, this: Ptr) {
    e.call(0x0045_cec0, &args![this.addr().wrapping_add(8)]);
}

/// `006e6da0` (cdecl: size, memory): placement new, returns the memory.
const PLACEMENT_NEW: u32 = 0x006e_6da0;
/// `00414430` (`thiscall`; four floats; `RET 0x10`).
const CONSTRUCT_FOUR_FLOATS: u32 = 0x0041_4430;

// Translated from 006a68f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006a68f0 - the exception-unwinding frame (FS:[0]) is not translated.
/// Constructs `count` 0x10-byte elements at `storage` (`thiscall`, `ECX` not
/// read; storage, count; `RET 8`): each through placement new (`006e6da0`
/// with 0x10) and, when that gives memory, `00414430` with four floats 0.0.
/// The engine map gives it no name.
pub fn fn_006a68f0(e: &mut Engine, _unused_0: Ptr, storage: u32, count: u32) {
    let mut index = 0u32;
    while index < count {
        let memory = e
            .call(
                PLACEMENT_NEW,
                &args![0x10u32, (index << 4).wrapping_add(storage)],
            )
            .u32();
        if memory != 0 {
            e.call(
                CONSTRUCT_FOUR_FLOATS,
                &args![memory, 0.0f32, 0.0f32, 0.0f32, 0.0f32],
            );
        }
        index += 1;
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006a_68b0, fn_006a68b0(Ptr)),
        entry!(0x006a_68d0, fn_006a68d0(Ptr)),
        entry!(0x006a_68f0, fn_006a68f0(Ptr, u32, u32)),
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
    fn fwd_a03_006a68b0_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x006a_68d0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x006a_68b0, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x006a_68d0), vec![vec![0x5000]]);
    }
}

#[cfg(test)]
mod b039_tests {
    use super::*;

    #[test]
    fn fn_006a68d0_destroys_the_pointer_at_8() {
        let mut e = Engine::new();
        e.register(0x0045_cec0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x006a_68d0, &args![0x5000u32]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == 0x0045_cec0).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, vec![0x5008]);
    }

    #[test]
    fn at_006a68f0_constructs_each_element_with_zeros() {
        let mut e = Engine::new();
        e.register(PLACEMENT_NEW, |_, a| Ret {
            eax: if a[1] == 0x9010 { 0 } else { a[1] },
            ..Ret::default()
        });
        e.register(CONSTRUCT_FOUR_FLOATS, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x006a_68f0, &args![0u32, 0x9000u32, 3u32]);
        let log = e.call_log.take().unwrap();
        let calls_to = |callee: u32| -> Vec<Vec<u32>> {
            log.iter()
                .filter(|(address, _)| *address == callee)
                .map(|(_, words)| words.clone())
                .collect()
        };
        assert_eq!(
            calls_to(PLACEMENT_NEW),
            vec![vec![0x10, 0x9000], vec![0x10, 0x9010], vec![0x10, 0x9020]]
        );
        // The second placement gave no memory: no constructor for it.
        assert_eq!(
            calls_to(CONSTRUCT_FOUR_FLOATS),
            vec![vec![0x9000, 0, 0, 0, 0], vec![0x9020, 0, 0, 0, 0]]
        );
    }
}

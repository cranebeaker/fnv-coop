//! `fallout/audio/run_008373f0` (Xbox PDB source unit), subsystem `fallout/audio`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 008373f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `00837410`, which frees the chain of nodes
/// starting at `+4` (through `00837480`) and zeroes the float at `+0`, and returns nothing.
/// Parameters: one word, `this`; the disassembly of the function ends in a plain `RET` and
/// passes only `ECX` on. `00837410`'s disassembly ends on `FLDZ; FSTP float ptr [EDX]` (a
/// store) with no result (Ghidra types it void).
pub fn fn_008373f0(e: &mut Engine, this: Ptr) {
    e.call(0x0083_7410, &args![this]);
}

// Translated from 00837410 (decompiled, FalloutNV.exe 1.4.0.525)
/// Frees the chain of nodes whose first is the word at `+4`: for each node
/// it takes the node's next (`+4`), clears that word, destroys and frees the
/// node (`00837480` with flag 1) and moves on; then the float at `+0`
/// becomes 0.0.
pub fn fn_00837410(e: &mut Engine, this: Ptr) {
    let head = this.addr().wrapping_add(4);
    while e.mem.u32(head) != 0 {
        let node = e.mem.u32(head);
        let next = e.mem.u32(node.wrapping_add(4));
        e.mem.set_u32(node.wrapping_add(4), 0);
        let node = e.mem.u32(head);
        if node != 0 {
            fn_00837480(e, Ptr::new(node), 1);
        }
        e.mem.set_u32(head, next);
    }
    e.mem.set_u32(this.addr(), 0.0f32.to_bits());
}

// Translated from 00837480 (decompiled, FalloutNV.exe 1.4.0.525)
/// The node's scalar deleting destructor: clears it (`008373f0`, which
/// frees its own chain and zeroes its float) and, with bit 0 of `flags`,
/// frees it (`operator delete`, `00401030`). Returns `this`.
pub fn fn_00837480(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_008373f0(e, this);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0083_73f0, fn_008373f0(Ptr)),
        entry!(0x0083_7410, fn_00837410(Ptr)),
        entry!(0x0083_7480, fn_00837480(Ptr, u32) -> Ptr),
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
    fn fwd_a03_008373f0_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x0083_7410, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0083_73f0, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x0083_7410), vec![vec![0x5000]]);
    }
}

#[cfg(test)]
mod chain_tests {
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
    fn chain_is_freed_node_by_node() {
        let mut e = Engine::new();
        e.register(0x0040_1030, |_, _| Ret::default());
        let owner = e.mem.alloc(8);
        let a = e.mem.alloc(8);
        let b = e.mem.alloc(8);
        e.mem.set_f32(owner, 3.5);
        e.mem.set_u32(owner + 4, a);
        e.mem.set_u32(a, 1.0f32.to_bits());
        e.mem.set_u32(a + 4, b);
        e.mem.set_u32(b, 2.0f32.to_bits());
        e.call_log = Some(vec![]);
        e.call(0x0083_7410, &args![owner]);
        assert_eq!(calls_to(&e, 0x0040_1030), vec![vec![a], vec![b]]);
        assert_eq!(e.mem.u32(owner), 0);
        assert_eq!(e.mem.u32(owner + 4), 0);
        assert_eq!(e.mem.u32(a), 0);
        assert_eq!(e.mem.u32(a + 4), 0);
        assert_eq!(e.mem.u32(b), 0);
    }

    #[test]
    fn deleting_destructor_frees_only_with_bit_0() {
        let mut e = Engine::new();
        e.register(0x0040_1030, |_, _| Ret::default());
        let node = e.mem.alloc(8);
        e.mem.set_f32(node, 4.0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0083_7480, &args![node, 0u32]).u32(), node);
        assert!(calls_to(&e, 0x0040_1030).is_empty());
        assert_eq!(e.mem.u32(node), 0);
        assert_eq!(e.call(0x0083_7480, &args![node, 1u32]).u32(), node);
        assert_eq!(calls_to(&e, 0x0040_1030), vec![vec![node]]);
    }
}

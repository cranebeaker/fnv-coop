//! `fallout/interface/run_007a8790` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// A list's item count (`005ae380`), a node's item slot (`006815c0`), the item store
/// (`00726c60`, `RET 4`: the address of the word; stores it unless 0) and the next node
/// (`00726070`).
const LIST_COUNT: u32 = 0x005a_e380;
const NODE_SELF: u32 = 0x0068_15c0;
const NODE_STORE: u32 = 0x0072_6c60;
const NODE_NEXT: u32 = 0x0072_6070;

// Translated from 007a8790 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall` on a list's first node, `RET 4`: a cdecl comparison of two
/// items): one bubble pass over the list: for each node after the first, when
/// `compare(previous item, item)` is above 0 the two items are swapped (`00726c60` on
/// each node). The outer loop runs while the count (`005ae380`) lasts and a swap was
/// made, but the node is not reset, so later rounds walk nothing, as the code does.
pub fn fn_007a8790(e: &mut Engine, this: Ptr, compare: u32) {
    let mut node = this.addr();
    let mut previous = 0u32;
    if node == 0 {
        return;
    }
    let mut count = e.call(LIST_COUNT, &args![node]).u32();
    let mut swapped = true;
    // The two item locals at `EBP - 0x14` and `EBP - 0x18`.
    e.with_stack(8, |e, items| {
        let (item, before) = (items.addr() + 4, items.addr());
        while count != 0 && swapped {
            swapped = false;
            while node != 0 {
                if previous != 0 {
                    let slot = e.call(NODE_SELF, &args![node]).u32();
                    let value = e.mem.u32(slot);
                    e.mem.set_u32(item, value);
                    let slot = e.call(NODE_SELF, &args![previous]).u32();
                    let value = e.mem.u32(slot);
                    e.mem.set_u32(before, value);
                    let (a, b) = (e.mem.u32(before), e.mem.u32(item));
                    if e.call(compare, &args![a, b]).i32() > 0 {
                        e.call(NODE_STORE, &args![node, before]);
                        e.call(NODE_STORE, &args![previous, item]);
                        swapped = true;
                    }
                }
                previous = node;
                node = e.call(NODE_NEXT, &args![node]).u32();
            }
            count = count.wrapping_sub(1);
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x007a_8790, fn_007a8790(Ptr, u32))]
}

#[cfg(test)]
mod n059_tests {
    use super::*;

    const COMPARE: u32 = 0x0e00_0000;

    #[test]
    fn one_bubble_pass() {
        let mut e = Engine::new();
        e.register(NODE_SELF, |_, a| a[0].into_ret());
        e.register(NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(NODE_STORE, |e, a| {
            let value = e.mem.u32(a[1]);
            if value != 0 {
                e.mem.set_u32(a[0], value);
            }
            Ret::default()
        });
        e.register(LIST_COUNT, |_, _| 3u32.into_ret());
        e.register(COMPARE, |_, a| (a[0] as i32 - a[1] as i32).into_ret());
        let mut next = 0;
        let mut nodes = vec![];
        for value in [3u32, 1, 2].iter().rev() {
            let node = e.mem.alloc(8);
            e.mem.set_u32(node, *value);
            e.mem.set_u32(node + 4, next);
            next = node;
            nodes.insert(0, node);
        }
        e.call(0x007a_8790, &args![nodes[0], COMPARE]);
        let values: Vec<u32> = nodes.iter().map(|&n| e.mem.u32(n)).collect();
        // One pass only: the 3 bubbles to the end.
        assert_eq!(values, vec![1, 2, 3]);
        e.mem.set_u32(nodes[0], 3);
        e.mem.set_u32(nodes[1], 2);
        e.mem.set_u32(nodes[2], 1);
        e.call(0x007a_8790, &args![nodes[0], COMPARE]);
        let values: Vec<u32> = nodes.iter().map(|&n| e.mem.u32(n)).collect();
        assert_eq!(values, vec![2, 1, 3]);
        e.call(0x007a_8790, &args![0u32, COMPARE]);
    }
}

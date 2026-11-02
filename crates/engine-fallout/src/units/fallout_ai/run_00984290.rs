//! `fallout/ai/run_00984290` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `CombatGoal`'s vtable (`0108d440`, named by its RTTI locator).
const COMBAT_GOAL_VTABLE: u32 = 0x0108_d440;
/// Returns the address of a list head's item (`006815c0`, `thiscall`).
const NODE_ITEM: u32 = 0x0068_15c0;
/// Removes the head of a list (`0063f7b0`, `thiscall`).
const LIST_REMOVE_HEAD: u32 = 0x0063_f7b0;
/// The list destructor (`007b9000`, `thiscall`).
const LIST_DESTRUCT: u32 = 0x007b_9000;
/// `operator delete` (`00401030`, `cdecl(block)`).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// Adds a pair to a list (`009aa2b0`, `thiscall(a, b)`).
const LIST_ADD: u32 = 0x009a_a2b0;

/// Frees the entry at the head of the list at `list` and removes the head
/// until the list is empty (the loop the destructors share).
fn drain(e: &mut Engine, list: u32) {
    // The exe tests `list != 0` before every step; the address never
    // changes, so one test is the same.
    if list == 0 {
        return;
    }
    loop {
        let slot = e.call(NODE_ITEM, &args![list]).u32();
        if e.mem.u32(slot) == 0 {
            break;
        }
        let slot = e.call(NODE_ITEM, &args![list]).u32();
        let entry = e.mem.u32(slot);
        e.call(OPERATOR_DELETE, &args![entry]);
        e.call(LIST_REMOVE_HEAD, &args![list]);
    }
}

// Translated from 00984290 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00984290 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatGoal` destructor body (by its vtable, `0108d440`): frees every
/// entry of the lists at +0x04 and +0x0C, then destroys the lists (+0x0C,
/// then +0x04, `007b9000`).
pub fn fn_00984290(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, COMBAT_GOAL_VTABLE);
    drain(e, base.wrapping_add(4));
    drain(e, base.wrapping_add(0xc));
    e.call(LIST_DESTRUCT, &args![base.wrapping_add(0xc)]);
    e.call(LIST_DESTRUCT, &args![base.wrapping_add(4)]);
}

// Translated from 00984390 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds `(a, b)` to the list at +0x0C (`009aa2b0`; `thiscall`, `RET 8`).
pub fn fn_00984390(e: &mut Engine, this: Ptr, a: u32, b: u32) {
    e.call(LIST_ADD, &args![this.addr() + 0xc, a, b]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0098_4290, fn_00984290(Ptr)),
        entry!(0x0098_4390, fn_00984390(Ptr, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// A list is a word holding a chain of nodes (entry, next); the item
    /// address of an empty list is a zero word kept for it.
    fn list_doubles(e: &mut Engine) {
        let empty = e.mem.alloc(4);
        e.register_double(NODE_ITEM, move |e, a| {
            let node = e.mem.u32(a[0]);
            if node == 0 {
                empty.into_ret()
            } else {
                node.into_ret()
            }
        });
        e.register(LIST_REMOVE_HEAD, |e, a| {
            let node = e.mem.u32(a[0]);
            let next = e.mem.u32(node + 4);
            e.mem.set_u32(a[0], next);
            Ret::default()
        });
        e.register(LIST_DESTRUCT, |_, _| Ret::default());
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
    }

    #[test]
    fn destructor_frees_both_lists() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        let this = e.mem.alloc(0x14);
        let second = e.mem.alloc(8);
        e.mem.set_u32(second, 0x6200);
        let first = e.mem.alloc(8);
        e.mem.set_u32(first, 0x6100);
        e.mem.set_u32(first + 4, second);
        e.mem.set_u32(this + 4, first);
        let other = e.mem.alloc(8);
        e.mem.set_u32(other, 0x6300);
        e.mem.set_u32(this + 0xc, other);
        e.call_log = Some(vec![]);
        e.call(0x0098_4290, &args![this]);
        assert_eq!(e.mem.u32(this), COMBAT_GOAL_VTABLE);
        assert_eq!(
            calls(&e, OPERATOR_DELETE),
            vec![vec![0x6100], vec![0x6200], vec![0x6300]]
        );
        assert_eq!(
            calls(&e, LIST_DESTRUCT),
            vec![vec![this + 0xc], vec![this + 4]]
        );
    }

    #[test]
    fn adder_uses_the_list_at_0c() {
        let mut e = Engine::new();
        e.register(LIST_ADD, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0098_4390, &args![0x100u32, 1u32, 2u32]);
        assert_eq!(calls(&e, LIST_ADD), vec![vec![0x10c, 1, 2]]);
    }
}

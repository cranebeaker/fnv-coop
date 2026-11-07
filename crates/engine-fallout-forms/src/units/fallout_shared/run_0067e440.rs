//! `fallout shared/run_0067e440` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `PackageList::FreePackageList` (`0067e400`, engine map; `thiscall`).
const FREE_PACKAGE_LIST: u32 = 0x0067_e400;
/// `006815c0` (`thiscall`): returns `this`; here a node's item word.
#[cfg(test)]
const NODE_ITEM: u32 = 0x0068_15c0;
/// `008256d0` (`thiscall`): whether a list node is empty (no item and no
/// next node).
#[cfg(test)]
const NODE_IS_EMPTY: u32 = 0x0082_56d0;
/// `0067e420` (`thiscall`, one word; engine map
/// `BSSimpleList<TESConditionItem_P>::AddTail`, a shared body): appends the
/// word to the list.
const ADD_TAIL: u32 = 0x0067_e420;
/// `00726070` (`thiscall`): the word at +4; here the next node.
#[cfg(test)]
const NODE_NEXT: u32 = 0x0072_6070;

// Translated from 0067e440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies a package list (`thiscall (this; other)`; no Xbox PDB name): frees
/// this list (`PackageList::FreePackageList`), then, unless `other`'s first
/// node is empty, appends each node's item from `other` until the node
/// pointer is null.
pub fn fn_0067e440(e: &mut Engine, this: Ptr, other: Ptr) {
    e.call(FREE_PACKAGE_LIST, &args![this]);
    let mut node = typed_call!(leaf::FAMILY, 0x0068_15c0, leaf::same_word(e, other));
    if typed_call!(leaf::FAMILY, 0x0082_56d0, leaf::both_words_zero(e, node)) {
        return;
    }
    while node != 0 {
        let slot = typed_call!(leaf::FAMILY, 0x0068_15c0, leaf::same_word(e, node));
        let item = e.mem.u32(slot);
        e.call(ADD_TAIL, &args![this, item]);
        node = typed_call!(leaf::FAMILY, 0x0072_6070, leaf::word_at_4(e, node));
    }
}

// Translated from 0067e4a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether two package lists differ (`thiscall (this; other)`, `RET 4`;
/// no Xbox PDB name): with this list's head (`006815c0`) empty
/// (`008256d0`), they differ unless the other's is empty too; otherwise
/// the items are compared pair by pair while both nodes are set and hold
/// an item, a different pair making them differ, and they differ when
/// either list has a node left.
pub fn fn_0067e4a0(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    let mut mine = typed_call!(leaf::FAMILY, 0x0068_15c0, leaf::same_word(e, this));
    let mut theirs = typed_call!(leaf::FAMILY, 0x0068_15c0, leaf::same_word(e, other));
    if typed_call!(leaf::FAMILY, 0x0082_56d0, leaf::both_words_zero(e, mine)) {
        return !typed_call!(leaf::FAMILY, 0x0082_56d0, leaf::both_words_zero(e, theirs));
    }
    let item = |e: &mut Engine, node: u32| {
        let slot = typed_call!(leaf::FAMILY, 0x0068_15c0, leaf::same_word(e, node));
        e.mem.u32(slot)
    };
    while mine != 0 && item(e, mine) != 0 && theirs != 0 && item(e, theirs) != 0 {
        let my_item = item(e, mine);
        let their_item = item(e, theirs);
        if my_item != their_item {
            return true;
        }
        mine = typed_call!(leaf::FAMILY, 0x0072_6070, leaf::word_at_4(e, mine));
        theirs = typed_call!(leaf::FAMILY, 0x0072_6070, leaf::word_at_4(e, theirs));
    }
    mine != 0 || theirs != 0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0067_e440, fn_0067e440(Ptr, Ptr)),
        entry!(0x0067_e4a0, fn_0067e4a0(Ptr, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn word(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn doubles(e: &mut Engine) {
        e.register(FREE_PACKAGE_LIST, |_, _| Ret::default());
        e.register(NODE_ITEM, |_, a| word(a[0]));
        e.register(NODE_IS_EMPTY, |e, a| {
            word(u32::from(e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0))
        });
        e.register(ADD_TAIL, |_, _| Ret::default());
        e.register(NODE_NEXT, |e, a| word(e.mem.u32(a[0] + 4)));
    }

    #[test]
    fn copies_every_item_of_the_other_list() {
        let mut e = Engine::new();
        doubles(&mut e);
        let second = e.mem.alloc(8);
        e.mem.set_u32(second, 0x22);
        let other = e.mem.alloc(8);
        e.mem.set_u32(other, 0x11);
        e.mem.set_u32(other + 4, second);
        e.call_log = Some(vec![]);
        e.call(0x0067_e440, &args![0x500u32, other]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (FREE_PACKAGE_LIST, vec![0x500]));
        let added: Vec<_> = log
            .iter()
            .filter(|c| c.0 == ADD_TAIL)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(added, vec![vec![0x500, 0x11], vec![0x500, 0x22]]);
    }

    #[test]
    fn an_empty_list_adds_nothing() {
        let mut e = Engine::new();
        doubles(&mut e);
        let other = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        e.call(0x0067_e440, &args![0x500u32, other]);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|c| c.0 != ADD_TAIL));
        assert_eq!(log[1], (FREE_PACKAGE_LIST, vec![0x500]));
    }
}

#[cfg(test)]
mod tests_n042 {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(NODE_ITEM, |_, a| a[0].into_ret());
        e.register(NODE_IS_EMPTY, |e, a| {
            u32::from(e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0).into_ret()
        });
        e.register(NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e
    }

    fn list(e: &mut Engine, items: &[u32]) -> u32 {
        let mut next = 0;
        for item in items.iter().rev() {
            let node = e.mem.alloc(8);
            e.mem.set_u32(node, *item);
            e.mem.set_u32(node + 4, next);
            next = node;
        }
        if next == 0 {
            next = e.mem.alloc(8);
        }
        next
    }

    #[test]
    fn lists_differ_by_items_and_length() {
        let mut e = engine();
        let differ = |e: &mut Engine, a: &[u32], b: &[u32]| {
            let a = list(e, a);
            let b = list(e, b);
            e.call(0x0067_e4a0, &args![a, b]).bool()
        };
        assert!(!differ(&mut e, &[1, 2], &[1, 2]));
        assert!(differ(&mut e, &[1, 2], &[1, 3]));
        assert!(differ(&mut e, &[1, 2], &[1]));
        assert!(differ(&mut e, &[1], &[1, 2]));
        assert!(!differ(&mut e, &[], &[]));
        assert!(differ(&mut e, &[], &[5]));
        // A node without an item stops the walk with a node left.
        assert!(differ(&mut e, &[1, 0, 3], &[1, 0, 3]));
    }
}

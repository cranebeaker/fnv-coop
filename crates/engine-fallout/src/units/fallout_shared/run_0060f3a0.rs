//! `fallout shared/run_0060f3a0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The quest stage's item clean-up: [`fn_0060f3a0`] deletes every item of
//! the stage's item list, [`fn_0060f410`] is the item's scalar deleting
//! destructor.

#[allow(unused_imports)]
use crate::prelude::*;

/// The item list of a stage (`00717e50`, `this` = the stage; the engine
/// map's `BaseProcess::GetActorPackageThatIsRunning` is folded code).
const ITEM_LIST: u32 = 0x0071_7e50;
/// True for an empty list node (`008256d0`).
const NODE_IS_EMPTY: u32 = 0x0082_56d0;
/// The item slot of a node (`006815c0`, returns the node itself; its first
/// word is the item).
const NODE_ITEM_SLOT: u32 = 0x0068_15c0;
/// Removes the first node of the list (`0063f7b0`, `this` = the list).
const LIST_REMOVE_HEAD: u32 = 0x0063_f7b0;
/// The stage item's destructor body (`0060f5f0`).
const ITEM_DESTRUCT: u32 = 0x0060_f5f0;
/// `operator delete` (`00401030`, cdecl).
const OPERATOR_DELETE: u32 = 0x0040_1030;

// Translated from 0060f3a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empties the item list of `this` (a quest stage; the list is fetched
/// once through `00717e50`): while the list is non-null and not empty
/// (`008256d0`), deletes its first item when there is one (`0060f410` with
/// 1) and removes the first node (`0063f7b0`). The first call of the stage
/// clean-up `0060f260`.
pub fn fn_0060f3a0(e: &mut Engine, this: Ptr) {
    let list = e.call(ITEM_LIST, &args![this]).u32();
    if list == 0 {
        return;
    }
    while !e.call(NODE_IS_EMPTY, &args![list]).bool() {
        let slot = e.call(NODE_ITEM_SLOT, &args![list]).u32();
        let item = e.mem.u32(slot);
        if item != 0 {
            fn_0060f410(e, Ptr::new(item), 1);
        }
        e.call(LIST_REMOVE_HEAD, &args![list]);
    }
}

// Translated from 0060f410 (decompiled, FalloutNV.exe 1.4.0.525)
/// The quest stage item's scalar deleting destructor: runs the destructor
/// body (`0060f5f0`) and frees the item (`operator delete`) when bit 0 of
/// `flags` is set. Returns `this`.
pub fn fn_0060f410(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(ITEM_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0060f3a0, fn_0060f3a0(Ptr)),
        entry!(0x0060f410, fn_0060f410(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn eax(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// The list is a head node of two words (item, next node); removing
    /// the head copies the next node's words into it.
    fn list_doubles(e: &mut Engine) {
        e.register(ITEM_LIST, |e, a| eax(e.mem.u32(a[0])));
        e.register(NODE_IS_EMPTY, |e, a| {
            eax((e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0) as u32)
        });
        e.register(NODE_ITEM_SLOT, |_, a| eax(a[0]));
        e.register(LIST_REMOVE_HEAD, |e, a| {
            let next = e.mem.u32(a[0] + 4);
            let (item, after) = if next == 0 {
                (0, 0)
            } else {
                (e.mem.u32(next), e.mem.u32(next + 4))
            };
            e.mem.set_u32(a[0], item);
            e.mem.set_u32(a[0] + 4, after);
            Ret::default()
        });
        e.register(ITEM_DESTRUCT, |_, _| Ret::default());
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
    }

    #[test]
    fn deletes_every_item_and_skips_null_items() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        let stage = e.mem.alloc(4);
        let head = e.mem.alloc(8);
        let second = e.mem.alloc(8);
        let third = e.mem.alloc(8);
        e.mem.set_u32(stage, head);
        e.mem.set_u32(head, 0x7000);
        e.mem.set_u32(head + 4, second);
        // A node with a null item: removed without a delete.
        e.mem.set_u32(second + 4, third);
        e.mem.set_u32(third, 0x7100);
        e.call_log = Some(vec![]);
        e.call(0x0060_f3a0, &[stage]);
        assert_eq!(calls_to(&e, ITEM_DESTRUCT), [[0x7000], [0x7100]]);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), [[0x7000], [0x7100]]);
        assert_eq!(calls_to(&e, LIST_REMOVE_HEAD).len(), 3);
        assert_eq!(calls_to(&e, ITEM_LIST), [[stage]]);
    }

    #[test]
    fn null_list_does_nothing() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        let stage = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        e.call(0x0060_f3a0, &[stage]);
        assert!(calls_to(&e, NODE_IS_EMPTY).is_empty());
    }

    #[test]
    fn deleting_destructor_frees_only_with_bit_0() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0060_f410, &[0x7000, 1]).u32(), 0x7000);
        assert_eq!(e.call(0x0060_f410, &[0x7100, 2]).u32(), 0x7100);
        assert_eq!(calls_to(&e, ITEM_DESTRUCT), [[0x7000], [0x7100]]);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), [[0x7000]]);
    }
}

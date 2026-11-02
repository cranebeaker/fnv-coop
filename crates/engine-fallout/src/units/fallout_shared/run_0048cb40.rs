//! `fallout shared/run_0048cb40` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `BaseFormComponent`'s constructor (`00402d30`: sets the base vtable).
const BASE_COMPONENT_CONSTRUCT: u32 = 0x0040_2d30;
/// This class's vtable.
const VTABLE: u32 = 0x0101_d518;

// Translated from 0048cc50 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of `TESScriptableForm` (the engine map gives it no name;
/// the Xbox PDB's `TESScriptableForm` is a `BaseFormComponent` followed by
/// `pFormScript` and `bIsInitialized`, which is what `0048cc80` clears):
/// runs the base constructor, sets the vtable and clears the script fields.
/// Returns `this`.
pub fn fn_0048cc50(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(BASE_COMPONENT_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), VTABLE);
    fn_0048cc80(e, this);
    this
}

// Translated from 0048cc80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the script pointer (`pFormScript`, +4) and the initialized flag
/// (`bIsInitialized`, +8, a byte) of a `TESScriptableForm` part. The decompiler
/// names it after a library function (`CDllIsolationWrapperBase::CommonConstruct`),
/// a false match.
pub fn fn_0048cc80(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr() + 4, 0);
    e.mem.set_u8(this.addr() + 8, 0);
}

/// `00428110` (`thiscall`, `RET 4`; unit `extradatalist.cpp` by the engine
/// map): the load buffer's changed-form data, written to the given out
/// word; answers the object `004280f0` is asked on.
const CHANGE_DATA: u32 = 0x0042_8110;
/// `004280f0` (`thiscall`, `RET 4`): whether the change flags hold the
/// given bits (`AL`).
const HAS_CHANGE_FLAGS: u32 = 0x0042_80f0;
/// `008256d0` (`thiscall`): whether the list node is empty (`AL`).
const NODE_IS_EMPTY: u32 = 0x0082_56d0;
/// `006815c0` (`thiscall`): the address of the node's item.
const NODE_ITEM_ADDRESS: u32 = 0x0068_15c0;
/// `00726070` (`thiscall`): the node's next node.
const NODE_NEXT: u32 = 0x0072_6070;
/// `0063f7b0` (`thiscall`): removes the list's head.
const LIST_REMOVE_HEAD: u32 = 0x0063_f7b0;
/// `00905330` (`thiscall`, `RET 4`): removes, after the node, the item
/// whose word is at the given address.
const LIST_REMOVE_ITEM: u32 = 0x0090_5330;
/// `004839c0` (`cdecl`, one word): the form with the given id.
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// `operator delete` (`00401030`, `cdecl`).
const FREE: u32 = 0x0040_1030;

// Translated from 0048cb40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resolves the list at +4 after a load (`thiscall`, `RET 4`, the load
/// buffer; the engine map gives it no name): only when the buffer's change
/// data (`00428110`, with a stack out word) has flag 4 (`004280f0`). Each
/// item of the list holds a form id in its first word, replaced by the
/// form (`004839c0`; 0 stays 0). An empty item, or one whose form is not
/// found (the item is freed, `00401030`), is unlinked: after the previous
/// node (`00905330`, the walk going on from the previous node's next) or,
/// at the head, by removing the head (`0063f7b0`, the walk staying on the
/// node). The walk stops at a null or empty node (`008256d0`).
pub fn fn_0048cb40(e: &mut Engine, this: Ptr, buffer: u32) {
    let flagged = e.with_stack(4, |e, out| {
        let data = e.call(CHANGE_DATA, &args![buffer, out]).u32();
        e.call(HAS_CHANGE_FLAGS, &args![data, 4u32]).bool()
    });
    if !flagged {
        return;
    }
    let mut node = this.addr().wrapping_add(4);
    let mut previous = 0u32;
    while node != 0 && !e.call(NODE_IS_EMPTY, &args![node]).bool() {
        let item_at = e.call(NODE_ITEM_ADDRESS, &args![node]).u32();
        let item = e.mem.u32(item_at);
        let mut remove = false;
        if item == 0 {
            remove = true;
        } else {
            let id = e.mem.u32(item);
            let form = if id != 0 {
                e.call(LOOKUP_FORM, &args![id]).u32()
            } else {
                0
            };
            e.mem.set_u32(item, form);
            if e.mem.u32(item) == 0 {
                e.call(FREE, &args![item]);
                remove = true;
            }
        }
        if !remove {
            previous = node;
            node = e.call(NODE_NEXT, &args![node]).u32();
        } else if previous != 0 {
            let item_at = e.call(NODE_ITEM_ADDRESS, &args![node]).u32();
            e.call(LIST_REMOVE_ITEM, &args![previous, item_at]);
            node = e.call(NODE_NEXT, &args![previous]).u32();
        } else {
            e.call(LIST_REMOVE_HEAD, &args![node]);
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0048_cb40, fn_0048cb40(Ptr, u32)),
        entry!(0x0048cc50, fn_0048cc50(Ptr) -> Ptr),
        entry!(0x0048cc80, fn_0048cc80(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_constructor_sets_the_vtable_and_clears_the_script_fields() {
        let mut e = Engine::new();
        e.register(BASE_COMPONENT_CONSTRUCT, |_, _| Ret::default());
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object + 4, 0x1111);
        e.mem.set_u8(object + 8, 1);
        e.mem.set_u8(object + 9, 7);
        e.call_log = Some(vec![]);
        let back = e.call(0x0048_cc50, &args![object]).u32();
        assert_eq!(back, object);
        assert_eq!(e.mem.u32(object), VTABLE);
        assert_eq!(e.mem.u32(object + 4), 0);
        assert_eq!(e.mem.u8(object + 8), 0);
        // Only the flag byte is cleared, not its neighbour.
        assert_eq!(e.mem.u8(object + 9), 7);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (BASE_COMPONENT_CONSTRUCT, vec![object]));
    }

    #[test]
    fn the_clearing_step_leaves_the_vtable_alone() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, 0x2222);
        e.mem.set_u32(object + 4, 0x3333);
        e.call(0x0048_cc80, &args![object]);
        assert_eq!(e.mem.u32(object), 0x2222);
        assert_eq!(e.mem.u32(object + 4), 0);
    }
}

#[cfg(test)]
mod n038_tests {
    use super::*;

    const FLAG: u32 = 0x0f00_0000;

    fn word(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    /// A list of nodes `[item, next]`; an empty node has item 0 and next 0
    /// and is the end. The removals relink the nodes as the list would.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.mem.map(FLAG, 4);
        e.register(CHANGE_DATA, |_, _| word(0x6000));
        e.register(HAS_CHANGE_FLAGS, |e, _| word(e.mem.u32(FLAG)));
        e.register(NODE_IS_EMPTY, |e, a| {
            word(u32::from(e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0))
        });
        e.register(NODE_ITEM_ADDRESS, |_, a| word(a[0]));
        e.register(NODE_NEXT, |e, a| word(e.mem.u32(a[0] + 4)));
        e.register(LOOKUP_FORM, |_, a| {
            word(if a[0] == 0x99 { 0 } else { a[0] + 0x1000 })
        });
        e.register(FREE, |_, _| Ret::default());
        // Unlink the node after `previous`.
        e.register(LIST_REMOVE_ITEM, |e, a| {
            let gone = e.mem.u32(a[0] + 4);
            let next = e.mem.u32(gone + 4);
            e.mem.set_u32(a[0] + 4, next);
            Ret::default()
        });
        // Remove the head: copy the second node into the head.
        e.register(LIST_REMOVE_HEAD, |e, a| {
            let second = e.mem.u32(a[0] + 4);
            let (item, next) = if second == 0 {
                (0, 0)
            } else {
                (e.mem.u32(second), e.mem.u32(second + 4))
            };
            e.mem.set_u32(a[0], item);
            e.mem.set_u32(a[0] + 4, next);
            Ret::default()
        });
        e
    }

    fn item(e: &mut Engine, id: u32) -> u32 {
        let i = e.mem.alloc(4);
        e.mem.set_u32(i, id);
        i
    }

    #[test]
    fn at_0048cb40_resolves_and_unlinks() {
        let mut e = engine();
        let this = e.mem.alloc(0xc);
        // Head: an item that is not found; then a good one, a missing
        // item, another good one.
        let bad = item(&mut e, 0x99);
        let good = item(&mut e, 0x10);
        let good2 = item(&mut e, 0x20);
        let n2 = e.mem.alloc(8);
        let n3 = e.mem.alloc(8);
        let n4 = e.mem.alloc(8);
        let end = e.mem.alloc(8);
        e.mem.set_u32(this + 4, bad);
        e.mem.set_u32(this + 8, n2);
        e.mem.set_u32(n2, good);
        e.mem.set_u32(n2 + 4, n3);
        e.mem.set_u32(n3 + 4, n4);
        e.mem.set_u32(n4, good2);
        e.mem.set_u32(n4 + 4, end);
        // Not flagged: nothing.
        e.call_log = Some(vec![]);
        e.call(0x0048_cb40, &args![this, 0x5000u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 3);
        e.mem.set_u32(FLAG, 1);
        e.call_log = Some(vec![]);
        e.call(0x0048_cb40, &args![this, 0x5000u32]);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(FREE, vec![bad])));
        assert!(log.contains(&(LIST_REMOVE_HEAD, vec![this + 4])));
        assert!(log.contains(&(LIST_REMOVE_ITEM, vec![this + 4, n3])));
        assert_eq!(e.mem.u32(good), 0x1010);
        assert_eq!(e.mem.u32(good2), 0x1020);
        // The list is now good, good2.
        assert_eq!(e.mem.u32(this + 4), good);
        assert_eq!(e.mem.u32(this + 8), n4);
    }
}

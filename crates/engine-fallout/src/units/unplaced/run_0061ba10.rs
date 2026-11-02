//! `(unplaced)/run_0061ba10` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::nimain::nistream::NiTLargeArray;

// Translated from 0061bd20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `SetAt` of an `NiTLargeArray` of plain words
/// (layout [`NiTLargeArray`]; `thiscall`, two stack words, `RET 8`): stores
/// the word `*element` at `index`, without a capacity check. Below the used
/// size, the non-empty count goes up when a zero slot gets a non-zero word
/// and down when a non-zero slot gets zero; at or above it, the used size
/// becomes `index + 1` and the count goes up when the word is non-zero.
pub fn fn_0061bd20(e: &mut Engine, this: Ptr<NiTLargeArray>, index: u32, element: Ptr) {
    let slot_of = |e: &Engine| {
        e.get(this, NiTLargeArray::m_pBase)
            .wrapping_add(index.wrapping_mul(4))
    };
    if index < e.get(this, NiTLargeArray::m_uiSize) {
        if e.mem.u32(element.addr()) == 0 {
            if e.mem.u32(slot_of(e)) != 0 {
                let present = e.get(this, NiTLargeArray::m_uiESize);
                e.set(this, NiTLargeArray::m_uiESize, present.wrapping_sub(1));
            }
        } else if e.mem.u32(slot_of(e)) == 0 {
            let present = e.get(this, NiTLargeArray::m_uiESize);
            e.set(this, NiTLargeArray::m_uiESize, present.wrapping_add(1));
        }
    } else {
        e.set(this, NiTLargeArray::m_uiSize, index.wrapping_add(1));
        if e.mem.u32(element.addr()) != 0 {
            let present = e.get(this, NiTLargeArray::m_uiESize);
            e.set(this, NiTLargeArray::m_uiESize, present.wrapping_add(1));
        }
    }
    let value = e.mem.u32(element.addr());
    let slot = slot_of(e);
    e.mem.set_u32(slot, value);
}

/// The global whose list `0061ba10` walks (`011c3f2c`).
const LIST_OWNER: u32 = 0x011c_3f2c;
/// Returns the first node of the owner's list (`004612e0`, `thiscall`).
const FIRST_NODE: u32 = 0x0046_12e0;
/// Returns the address of a node's item (`006815c0`, `thiscall`).
const NODE_ITEM: u32 = 0x0068_15c0;
/// Returns a node's next node (`00726070`, `thiscall`).
const NODE_NEXT: u32 = 0x0072_6070;
/// Stores `*item` into a node (`00726c60`, `thiscall(item pointer)`).
const NODE_SET_ITEM: u32 = 0x0072_6c60;
/// The filter `0061ba10` asks of each item when given an argument
/// (`00619bc0`, `thiscall(argument, 0)`, `bool`).
const ITEM_MATCHES: u32 = 0x0061_9bc0;
/// Returns the list `0061ba90` sorts (`005d8a70`, `thiscall`).
const ITEM_LIST: u32 = 0x005d_8a70;
/// Counts a list's items (`005ae380`, `thiscall`; the engine map calls it
/// `VATS::GetCount`).
const LIST_COUNT: u32 = 0x005a_e380;
/// The sort key of an element (`005c3bd0`, `thiscall`).
const ELEMENT_KEY: u32 = 0x005c_3bd0;
/// `NiTLargeArray` growth helpers: `SetAtGrow(index, element pointer)`
/// (`00864240`) and `SetSize(size)` (`00864280`).
const LARGE_ARRAY_SET_AT_GROW: u32 = 0x0086_4240;
const LARGE_ARRAY_SET_SIZE: u32 = 0x0086_4280;
/// `memmove` (`00ec7230`, `cdecl(dst, src, count)`).
const MEMMOVE: u32 = 0x00ec_7230;
/// Allocates `count` words (`0096afc0`, `cdecl(count)`).
const ALLOCATE_WORDS: u32 = 0x0096_afc0;
/// Frees an element block (`004ede70`, `cdecl(block)`).
const FREE_WORDS: u32 = 0x004e_de70;
/// `NiTLargeArray<TESTopicInfo*, NiTMallocInterface<TESTopicInfo*>>`'s
/// vtable (`0104d448`, named by its RTTI locator).
const TOPIC_INFO_ARRAY_VTABLE: u32 = 0x0104_d448;
/// The two calls of `0061bcd0`: `00483710` (`thiscall`, an empty body in
/// the exe) and the string setter `00619010` (`thiscall(text)`), given the
/// empty string at `01011584`.
const EMPTY_BODY: u32 = 0x0048_3710;
const SET_TEXT: u32 = 0x0061_9010;
const EMPTY_STRING: u32 = 0x0101_1584;

fn node_item(e: &mut Engine, node: u32) -> u32 {
    let slot = e.call(NODE_ITEM, &args![node]).u32();
    e.mem.u32(slot)
}

// Translated from 0061ba10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `cdecl(filter)`: walks the list of the object at `011c3f2c` and runs
/// `0061ba90` on each item (only those `00619bc0(item, filter, 0)` accepts
/// when `filter` is not null), while each run returns true. Stops at a null
/// node or a node without an item; returns the last run's result (true when
/// none ran).
pub fn fn_0061ba10(e: &mut Engine, filter: u32) -> bool {
    let mut ok = true;
    let owner: u32 = e.global(LIST_OWNER);
    let mut node = e.call(FIRST_NODE, &args![owner]).u32();
    while node != 0 {
        if node_item(e, node) == 0 || !ok {
            break;
        }
        let runs = if filter != 0 {
            let item = node_item(e, node);
            e.call(ITEM_MATCHES, &args![item, filter, 0u32]).u8() != 0
        } else {
            true
        };
        if runs {
            let item = node_item(e, node);
            ok = fn_0061ba90(e, Ptr::new(item));
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    ok
}

/// The key `0061ba90` sorts by: `005c3bd0` of the element's first word, or
/// -1 when that word is null.
fn element_key(e: &mut Engine, element: u32) -> i32 {
    let first = e.mem.u32(element);
    if first == 0 {
        return -1;
    }
    e.call(ELEMENT_KEY, &args![first]).i32()
}

// Translated from 0061ba90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `this`'s vtable slot 0x88, then bubble-sorts the list `005d8a70`
/// returns by descending key (`count - 1` passes; neighbours whose key is
/// higher than the one before them are swapped through `00726c60`). Pairs
/// with a null element are left in place. Returns true.
pub fn fn_0061ba90(e: &mut Engine, this: Ptr) -> bool {
    e.vcall(this.addr(), 0x88, &args![]);
    let list = e.call(ITEM_LIST, &args![this]).u32();
    let count = e.call(LIST_COUNT, &args![list]).i32();
    let mut pass = 0i32;
    while pass < count.wrapping_sub(1) {
        let mut node = e.call(ITEM_LIST, &args![this]).u32();
        while node != 0 && node_item(e, node) != 0 {
            if e.call(NODE_NEXT, &args![node]).u32() != 0 {
                let first = node_item(e, node);
                let next = e.call(NODE_NEXT, &args![node]).u32();
                let second = node_item(e, next);
                if first != 0 && second != 0 {
                    let first_key = element_key(e, first);
                    let second_key = element_key(e, second);
                    if second_key > first_key {
                        // The game's locals: `second` at EBP-0x14, `first`
                        // at EBP-0x10.
                        e.with_stack(8, |e, locals| {
                            e.mem.set_u32(locals.addr(), second);
                            e.mem.set_u32(locals.addr() + 4, first);
                            e.call(NODE_SET_ITEM, &args![node, locals]);
                            let next = e.call(NODE_NEXT, &args![node]).u32();
                            e.call(NODE_SET_ITEM, &args![next, locals.addr() + 4]);
                        });
                    }
                }
            }
            node = e.call(NODE_NEXT, &args![node]).u32();
        }
        pass += 1;
    }
    true
}

// Translated from 0061bc10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTLargeArray<T>` insert of a word at `index` (unnamed in the engine
/// map; `thiscall(index, value)`, `RET 8`): compacts the array first
/// (`0061bdc0`); past the used size it is `SetAtGrow(index, &value)`
/// (`00864240`), otherwise it grows the capacity (`SetSize(size + 1 +
/// growBy)`, `00864280`) when full, moves the words from `index` up by one,
/// counts the new word in both counts and stores it.
pub fn fn_0061bc10(e: &mut Engine, this: Ptr<NiTLargeArray>, index: u32, value: u32) {
    fn_0061bdc0(e, this);
    if index > e.get(this, NiTLargeArray::m_uiSize) {
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), value);
            e.call(LARGE_ARRAY_SET_AT_GROW, &args![this, index, slot]);
        });
        return;
    }
    let needed = e.get(this, NiTLargeArray::m_uiSize).wrapping_add(1);
    if e.get(this, NiTLargeArray::m_uiMaxSize) < needed {
        let grow_by = e.get(this, NiTLargeArray::m_uiGrowBy);
        e.call(
            LARGE_ARRAY_SET_SIZE,
            &args![this, needed.wrapping_add(grow_by)],
        );
    }
    let size = e.get(this, NiTLargeArray::m_uiSize);
    let base = e.get(this, NiTLargeArray::m_pBase);
    let from = base.wrapping_add(index.wrapping_mul(4));
    e.call(
        MEMMOVE,
        &args![
            from.wrapping_add(4),
            from,
            size.wrapping_sub(index).wrapping_mul(4)
        ],
    );
    let size = e.get(this, NiTLargeArray::m_uiSize).wrapping_add(1);
    e.set(this, NiTLargeArray::m_uiSize, size);
    e.set(this, NiTLargeArray::m_uiESize, size);
    let base = e.get(this, NiTLargeArray::m_pBase);
    e.mem
        .set_u32(base.wrapping_add(index.wrapping_mul(4)), value);
}

// Translated from 0061bcd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `00483710` on `this` (an empty body), then sets `this`'s text to
/// the empty string at `01011584` (`00619010`).
pub fn fn_0061bcd0(e: &mut Engine, this: Ptr) {
    e.call(EMPTY_BODY, &args![this]);
    e.call(SET_TEXT, &args![this, EMPTY_STRING]);
}

// Translated from 0061bcf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTLargeArray<TESTopicInfo*>` destructor body (by its vtable,
/// `0104d448`): restores the vtable and frees the element block
/// (`004ede70`).
pub fn fn_0061bcf0(e: &mut Engine, this: Ptr<NiTLargeArray>) {
    e.mem.set_u32(this.addr(), TOPIC_INFO_ARRAY_VTABLE);
    let base = e.get(this, NiTLargeArray::m_pBase);
    e.call(FREE_WORDS, &args![base]);
}

// Translated from 0061bdc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTLargeArray<T>` compaction (unnamed in the engine map): when the
/// non-empty count differs from the used size, moves the non-null words to
/// the front (when there are any), sets size and capacity to the non-empty
/// count, copies the words into a new block of that many words (`0096afc0`;
/// a null block when the count is 0) and frees the old block (`004ede70`).
pub fn fn_0061bdc0(e: &mut Engine, this: Ptr<NiTLargeArray>) {
    if e.get(this, NiTLargeArray::m_uiESize) == e.get(this, NiTLargeArray::m_uiSize) {
        return;
    }
    if e.get(this, NiTLargeArray::m_uiESize) != 0 {
        let mut kept = 0u32;
        let mut i = 0u32;
        while i < e.get(this, NiTLargeArray::m_uiSize) {
            let base = e.get(this, NiTLargeArray::m_pBase);
            let word = e.mem.u32(base.wrapping_add(i.wrapping_mul(4)));
            if word != 0 {
                let target = base.wrapping_add(kept.wrapping_mul(4));
                if e.mem.u32(target) != word {
                    e.mem.set_u32(target, word);
                }
                kept = kept.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
    }
    let old = e.get(this, NiTLargeArray::m_pBase);
    let present = e.get(this, NiTLargeArray::m_uiESize);
    e.set(this, NiTLargeArray::m_uiSize, present);
    e.set(this, NiTLargeArray::m_uiMaxSize, present);
    let capacity = e.get(this, NiTLargeArray::m_uiMaxSize);
    if capacity == 0 {
        e.set(this, NiTLargeArray::m_pBase, 0);
    } else {
        let block = e.call(ALLOCATE_WORDS, &args![capacity]).u32();
        e.set(this, NiTLargeArray::m_pBase, block);
        let mut i = 0u32;
        while i < e.get(this, NiTLargeArray::m_uiSize) {
            let word = e.mem.u32(old.wrapping_add(i.wrapping_mul(4)));
            let base = e.get(this, NiTLargeArray::m_pBase);
            e.mem.set_u32(base.wrapping_add(i.wrapping_mul(4)), word);
            i = i.wrapping_add(1);
        }
    }
    e.call(FREE_WORDS, &args![old]);
}

// Translated from 0061bbd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A vtable slot (`0104d2bc`; unnamed in the engine map; `ECX` not read, one byte word,
/// `RET 4`): [`fn_0061bbf0`] on the byte.
pub fn fn_0061bbd0(e: &mut Engine, _this: Ptr, kind: u8) -> bool {
    fn_0061bbf0(e, kind)
}

// Translated from 0061bbf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the byte is 0x46 (unnamed in the engine map; `cdecl`, one byte word).
pub fn fn_0061bbf0(_e: &mut Engine, kind: u8) -> bool {
    kind == 0x46
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0061_bbd0, fn_0061bbd0(Ptr, u8) -> bool),
        entry!(0x0061_bbf0, fn_0061bbf0(u8) -> bool),
        entry!(0x0061bd20, fn_0061bd20(Ptr<NiTLargeArray>, u32, Ptr)),
        entry!(0x0061_ba10, fn_0061ba10(u32) -> bool),
        entry!(0x0061_ba90, fn_0061ba90(Ptr) -> bool),
        entry!(0x0061_bc10, fn_0061bc10(Ptr<NiTLargeArray>, u32, u32)),
        entry!(0x0061_bcd0, fn_0061bcd0(Ptr)),
        entry!(0x0061_bcf0, fn_0061bcf0(Ptr<NiTLargeArray>)),
        entry!(0x0061_bdc0, fn_0061bdc0(Ptr<NiTLargeArray>)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// An array of capacity 4 holding `items`, and a word holding `value`.
    fn fixture(items: &[u32], value: u32) -> (Engine, Ptr<NiTLargeArray>, u32, u32) {
        let mut e = Engine::new();
        let array = e.new_object::<NiTLargeArray>();
        let base = e.mem.alloc(16);
        for (i, item) in items.iter().enumerate() {
            e.mem.set_u32(base + i as u32 * 4, *item);
        }
        e.set(array, NiTLargeArray::m_pBase, base);
        e.set(array, NiTLargeArray::m_uiMaxSize, 4);
        e.set(array, NiTLargeArray::m_uiSize, items.len() as u32);
        let present = items.iter().filter(|item| **item != 0).count();
        e.set(array, NiTLargeArray::m_uiESize, present as u32);
        let element = e.mem.alloc(4);
        e.mem.set_u32(element, value);
        (e, array, base, element)
    }

    fn counts(e: &Engine, array: Ptr<NiTLargeArray>) -> (u32, u32) {
        (
            e.get(array, NiTLargeArray::m_uiSize),
            e.get(array, NiTLargeArray::m_uiESize),
        )
    }

    #[test]
    fn filling_an_empty_slot_counts_it() {
        let (mut e, array, base, element) = fixture(&[7, 0], 9);
        e.call(0x0061_bd20, &args![array, 1u32, element]);
        assert_eq!(e.mem.u32(base + 4), 9);
        assert_eq!(counts(&e, array), (2, 2));
    }

    #[test]
    fn clearing_a_used_slot_uncounts_it() {
        let (mut e, array, base, element) = fixture(&[7, 8], 0);
        e.call(0x0061_bd20, &args![array, 0u32, element]);
        assert_eq!(e.mem.u32(base), 0);
        assert_eq!(counts(&e, array), (2, 1));
    }

    #[test]
    fn replacing_a_used_slot_keeps_the_count() {
        let (mut e, array, base, element) = fixture(&[7, 8], 5);
        e.call(0x0061_bd20, &args![array, 1u32, element]);
        assert_eq!(e.mem.u32(base + 4), 5);
        assert_eq!(counts(&e, array), (2, 2));
        // Zero into a zero slot changes nothing either.
        let (mut e, array, _, element) = fixture(&[0, 8], 0);
        e.call(0x0061_bd20, &args![array, 0u32, element]);
        assert_eq!(counts(&e, array), (2, 1));
    }

    #[test]
    fn past_the_used_size_grows_it() {
        let (mut e, array, base, element) = fixture(&[7], 6);
        e.call(0x0061_bd20, &args![array, 2u32, element]);
        assert_eq!(e.mem.u32(base + 8), 6);
        assert_eq!(counts(&e, array), (3, 2));
        let (mut e, array, _, element) = fixture(&[7], 0);
        e.call(0x0061_bd20, &args![array, 3u32, element]);
        assert_eq!(counts(&e, array), (4, 1));
    }
}

#[cfg(test)]
mod list_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// Nodes are 8-byte blocks: item word at +0, next node at +4.
    fn register_nodes(e: &mut Engine) {
        e.register(NODE_ITEM, |_, a| a[0].into_ret());
        e.register(NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(NODE_SET_ITEM, |e, a| {
            let value = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], value);
            Ret::default()
        });
    }

    fn chain(e: &mut Engine, items: &[u32]) -> u32 {
        let mut next = 0;
        for item in items.iter().rev() {
            let node = e.mem.alloc(8);
            e.mem.set_u32(node, *item);
            e.mem.set_u32(node + 4, next);
            next = node;
        }
        next
    }

    fn items(e: &Engine, mut node: u32) -> Vec<u32> {
        let mut out = vec![];
        while node != 0 {
            out.push(e.mem.u32(node));
            node = e.mem.u32(node + 4);
        }
        out
    }

    /// An item `0061ba90` can run on: its vtable slot 0x88 is `SLOT_88`, and
    /// the list `005d8a70` returns for it is empty (doubled to null).
    const SLOT_88: u32 = 0x0066_0000;

    fn sortable(e: &mut Engine) -> u32 {
        let vtable = e.mem.alloc(0x90);
        e.mem.set_u32(vtable + 0x88, SLOT_88);
        let item = e.mem.alloc(4);
        e.mem.set_u32(item, vtable);
        item
    }

    #[test]
    fn at_0061ba10_runs_each_accepted_item() {
        let mut e = Engine::new();
        register_nodes(&mut e);
        e.register(SLOT_88, |_, _| Ret::default());
        e.register(ITEM_LIST, |_, _| 0u32.into_ret());
        e.register(LIST_COUNT, |_, _| 0u32.into_ret());
        e.map(0x011c_3000, 0x1000);
        e.set_global(LIST_OWNER, 0x4242u32);
        let first = sortable(&mut e);
        let second = sortable(&mut e);
        let third = sortable(&mut e);
        let head = chain(&mut e, &[first, second, third, 0, first]);
        e.register_double(FIRST_NODE, move |_, _| head.into_ret());
        // The filter rejects the second item.
        e.register_double(ITEM_MATCHES, move |_, a| {
            u32::from(a[0] != second).into_ret()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0061_ba10, &args![7u32]).u8(), 1);
        assert_eq!(calls(&e, FIRST_NODE), vec![vec![0x4242]]);
        // The node without an item ends the walk.
        assert_eq!(
            calls(&e, ITEM_MATCHES),
            vec![vec![first, 7, 0], vec![second, 7, 0], vec![third, 7, 0]]
        );
        assert_eq!(calls(&e, SLOT_88), vec![vec![first], vec![third]]);
        // Without a filter every item runs.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0061_ba10, &args![0u32]).u8(), 1);
        assert!(calls(&e, ITEM_MATCHES).is_empty());
        assert_eq!(calls(&e, SLOT_88).len(), 3);
    }

    #[test]
    fn at_0061ba90_sorts_by_descending_key() {
        let mut e = Engine::new();
        register_nodes(&mut e);
        e.register(LIST_COUNT, |_, _| 4u32.into_ret());
        // Elements hold a pointer to their key word (null: key -1).
        e.register(ELEMENT_KEY, |e, a| e.mem.u32(a[0]).into_ret());
        let mut elements = vec![];
        for key in [Some(1u32), Some(5), None, Some(3)] {
            let element = e.mem.alloc(4);
            if let Some(key) = key {
                let word = e.mem.alloc(4);
                e.mem.set_u32(word, key);
                e.mem.set_u32(element, word);
            }
            elements.push(element);
        }
        let head = chain(&mut e, &elements);
        e.register_double(ITEM_LIST, move |_, _| head.into_ret());
        let vtable = e.mem.alloc(0x90);
        e.mem.set_u32(vtable + 0x88, 0x0066_0000);
        e.register(0x0066_0000, |_, _| Ret::default());
        let this = e.mem.alloc(4);
        e.mem.set_u32(this, vtable);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0061_ba90, &args![this]).u8(), 1);
        assert_eq!(calls(&e, 0x0066_0000), vec![vec![this]]);
        let sorted = items(&e, head);
        assert_eq!(
            sorted,
            vec![elements[1], elements[3], elements[0], elements[2]]
        );
    }

    #[test]
    fn at_0061ba90_skips_pairs_with_a_null_element() {
        let mut e = Engine::new();
        register_nodes(&mut e);
        e.register(LIST_COUNT, |_, _| 2u32.into_ret());
        e.register(ELEMENT_KEY, |e, a| e.mem.u32(a[0]).into_ret());
        let element = e.mem.alloc(4);
        let word = e.mem.alloc(4);
        e.mem.set_u32(word, 9);
        e.mem.set_u32(element, word);
        // A node with an item, then a node without one: the walk stops.
        let head = chain(&mut e, &[element, 0]);
        e.register_double(ITEM_LIST, move |_, _| head.into_ret());
        let vtable = e.mem.alloc(0x90);
        e.mem.set_u32(vtable + 0x88, 0x0066_0000);
        e.register(0x0066_0000, |_, _| Ret::default());
        let this = e.mem.alloc(4);
        e.mem.set_u32(this, vtable);
        e.call_log = Some(vec![]);
        e.call(0x0061_ba90, &args![this]);
        assert!(calls(&e, ELEMENT_KEY).is_empty());
        assert!(calls(&e, NODE_SET_ITEM).is_empty());
    }

    /// A large array with capacity `capacity` holding `words`.
    fn array(e: &mut Engine, words: &[u32], capacity: u32, present: u32) -> Ptr<NiTLargeArray> {
        let this = e.new_object::<NiTLargeArray>();
        let base = e.mem.alloc(4 * capacity.max(1));
        for (i, w) in words.iter().enumerate() {
            e.mem.set_u32(base + 4 * i as u32, *w);
        }
        e.set(this, NiTLargeArray::m_pBase, base);
        e.set(this, NiTLargeArray::m_uiMaxSize, capacity);
        e.set(this, NiTLargeArray::m_uiSize, words.len() as u32);
        e.set(this, NiTLargeArray::m_uiESize, present);
        e.set(this, NiTLargeArray::m_uiGrowBy, 2);
        this
    }

    fn words(e: &Engine, this: Ptr<NiTLargeArray>) -> Vec<u32> {
        let base = e.get(this, NiTLargeArray::m_pBase);
        (0..e.get(this, NiTLargeArray::m_uiSize))
            .map(|i| e.mem.u32(base + 4 * i))
            .collect()
    }

    fn array_engine() -> Engine {
        let mut e = Engine::new();
        e.register(ALLOCATE_WORDS, |e, a| e.mem.alloc(4 * a[0]).into_ret());
        e.register(FREE_WORDS, |_, _| Ret::default());
        e.register(MEMMOVE, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        e.register(LARGE_ARRAY_SET_SIZE, |e, a| {
            let this: Ptr<NiTLargeArray> = Ptr::new(a[0]);
            let old = e.get(this, NiTLargeArray::m_pBase);
            let block = e.mem.alloc(4 * a[1]);
            let size = e.get(this, NiTLargeArray::m_uiSize);
            let bytes = e.mem.bytes(old, 4 * size);
            e.mem.write(block, &bytes);
            e.set(this, NiTLargeArray::m_pBase, block);
            e.set(this, NiTLargeArray::m_uiMaxSize, a[1]);
            Ret::default()
        });
        e.register(LARGE_ARRAY_SET_AT_GROW, |_, _| Ret::default());
        e
    }

    #[test]
    fn at_0061bdc0_compacts_into_a_new_block() {
        let mut e = array_engine();
        let this = array(&mut e, &[0, 4, 0, 6, 7], 8, 3);
        let old = e.get(this, NiTLargeArray::m_pBase);
        e.call_log = Some(vec![]);
        e.call(0x0061_bdc0, &args![this]);
        assert_eq!(words(&e, this), vec![4, 6, 7]);
        assert_eq!(e.get(this, NiTLargeArray::m_uiMaxSize), 3);
        assert_eq!(e.get(this, NiTLargeArray::m_uiESize), 3);
        assert_ne!(e.get(this, NiTLargeArray::m_pBase), old);
        assert_eq!(calls(&e, ALLOCATE_WORDS), vec![vec![3]]);
        assert_eq!(calls(&e, FREE_WORDS), vec![vec![old]]);
        // The old block was compacted in place first.
        assert_eq!(e.mem.u32(old), 4);
        assert_eq!(e.mem.u32(old + 4), 6);
        assert_eq!(e.mem.u32(old + 8), 7);
    }

    #[test]
    fn at_0061bdc0_leaves_a_full_array_and_nulls_an_empty_one() {
        let mut e = array_engine();
        let full = array(&mut e, &[1, 2], 4, 2);
        e.call_log = Some(vec![]);
        e.call(0x0061_bdc0, &args![full]);
        assert!(calls(&e, FREE_WORDS).is_empty());
        assert_eq!(e.get(full, NiTLargeArray::m_uiMaxSize), 4);
        let empty = array(&mut e, &[0, 0], 4, 0);
        let old = e.get(empty, NiTLargeArray::m_pBase);
        e.call(0x0061_bdc0, &args![empty]);
        assert_eq!(e.get(empty, NiTLargeArray::m_pBase), 0);
        assert_eq!(e.get(empty, NiTLargeArray::m_uiSize), 0);
        assert!(calls(&e, ALLOCATE_WORDS).is_empty());
        assert_eq!(calls(&e, FREE_WORDS), vec![vec![old]]);
    }

    #[test]
    fn at_0061bc10_inserts_and_grows() {
        let mut e = array_engine();
        let this = array(&mut e, &[1, 2, 3], 3, 3);
        e.call_log = Some(vec![]);
        e.call(0x0061_bc10, &args![this, 1u32, 9u32]);
        assert_eq!(words(&e, this), vec![1, 9, 2, 3]);
        assert_eq!(e.get(this, NiTLargeArray::m_uiESize), 4);
        // Full at size 3: SetSize(3 + 1 + growBy 2).
        assert_eq!(calls(&e, LARGE_ARRAY_SET_SIZE), vec![vec![this.addr(), 6]]);
        // Room left: no growth; inserting at the end moves nothing.
        e.call_log = Some(vec![]);
        e.call(0x0061_bc10, &args![this, 4u32, 8u32]);
        assert_eq!(words(&e, this), vec![1, 9, 2, 3, 8]);
        assert!(calls(&e, LARGE_ARRAY_SET_SIZE).is_empty());
        let moves = calls(&e, MEMMOVE);
        assert_eq!(moves[0][2], 0);
    }

    #[test]
    fn at_0061bc10_past_the_end_uses_set_at_grow() {
        let mut e = array_engine();
        let seen: Rc<RefCell<Vec<(u32, u32)>>> = Rc::default();
        let record = seen.clone();
        e.register_double(LARGE_ARRAY_SET_AT_GROW, move |e, a| {
            record.borrow_mut().push((a[1], e.mem.u32(a[2])));
            Ret::default()
        });
        let this = array(&mut e, &[1, 2], 4, 2);
        e.call_log = Some(vec![]);
        e.call(0x0061_bc10, &args![this, 3u32, 0x55u32]);
        assert_eq!(*seen.borrow(), vec![(3, 0x55)]);
        assert!(calls(&e, MEMMOVE).is_empty());
        assert_eq!(words(&e, this), vec![1, 2]);
    }

    #[test]
    fn at_0061bcd0_and_0061bcf0() {
        let mut e = Engine::new();
        e.register(EMPTY_BODY, |_, _| Ret::default());
        e.register(SET_TEXT, |_, _| Ret::default());
        e.register(FREE_WORDS, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0061_bcd0, &args![0x5000u32]);
        assert_eq!(calls(&e, EMPTY_BODY), vec![vec![0x5000]]);
        assert_eq!(calls(&e, SET_TEXT), vec![vec![0x5000, EMPTY_STRING]]);
        let this = e.new_object::<NiTLargeArray>();
        e.set(this, NiTLargeArray::m_pBase, 0x6000);
        e.call(0x0061_bcf0, &args![this]);
        assert_eq!(e.mem.u32(this.addr()), TOPIC_INFO_ARRAY_VTABLE);
        assert_eq!(calls(&e, FREE_WORDS), vec![vec![0x6000]]);
    }
}

#[cfg(test)]
mod n075_tests {
    use super::*;

    #[test]
    fn fn_0061bbd0_and_0061bbf0_test_for_0x46() {
        let mut e = Engine::new();
        assert!(e.call(0x0061_bbf0, &args![0x146u32]).bool());
        assert!(!e.call(0x0061_bbf0, &args![0x45u32]).bool());
        assert!(e.call(0x0061_bbd0, &args![0x1000u32, 0x46u32]).bool());
        assert!(!e.call(0x0061_bbd0, &args![0x1000u32, 0x47u32]).bool());
    }
}

//! `fallout shared/tempeffects/run_0068a8d0` (Xbox PDB source unit), subsystem `fallout shared/tempeffects`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00653270` (unit `bsfacegenmanager.cpp`, `cdecl`: a table, an object; 0 for a null object):
/// the lookup `fn_0068a8f0` makes in the tables at `012024e0` (for each node) and `01202548`
/// (for each entry).
const LOOKUP: u32 = 0x0065_3270;
const NODE_TABLE_012024E0: u32 = 0x0120_24e0;
const ENTRY_TABLE_01202548: u32 = 0x0120_2548;
/// The found object's entry count (`004d7d50`, `thiscall`, no stack words) and entry at an
/// index (`004d7d70`, `thiscall`: the index).
const ENTRY_COUNT: u32 = 0x004d_7d50;
const ENTRY_AT: u32 = 0x004d_7d70;
/// The value an entry's object gives (`009611e0`, `thiscall`: its word at +0x18).
const ENTRY_VALUE: u32 = 0x0096_11e0;
/// The output array's element address (`00877a30`, `thiscall`: index) and its store
/// (`0096ae90`, `thiscall`: index, the address of the value).
const ELEMENT_ADDRESS: u32 = 0x0087_7a30;
const ARRAY_SET_AT: u32 = 0x0096_ae90;
/// A node's child count (`0043b480`, `thiscall`) and child at an index (`0043b4a0`,
/// `thiscall`: index), both on its array at +0x9c.
const CHILD_COUNT: u32 = 0x0043_b480;
const CHILD_AT: u32 = 0x0043_b4a0;
/// The vtable slot +0xc the code asks each node (`IsNode` in the Xbox PDB's `NiObject`
/// vtable order): not null for a node with children to visit.
const AS_NODE_SLOT: u32 = 0xc;

// Translated from 0068a8f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, three stack words, `RET 0xc`; `this` is only passed
/// on): a recursive walk from `node` that collects values into the array `list`, whose used
/// length is the word at `count`. For a non-null `node`: when the lookup in the table at
/// `012024e0` finds an object, each of its entries that the table at `01202548` maps to an
/// object adds that object's value (`009611e0`) at index `*count` (`0096ae90`) and counts it,
/// unless a search from the end of the first `*count` elements finds it already. Then, when the
/// node's vtable slot +0xc answers not null, the walk goes on into each of its children.
pub fn fn_0068a8f0(e: &mut Engine, this: Ptr, node: u32, list: u32, count: Ptr) {
    if node == 0 {
        return;
    }
    let found = e.call(LOOKUP, &args![NODE_TABLE_012024E0, node]).u32();
    if found != 0 {
        let mut index: u32 = 0;
        while index < e.call(ENTRY_COUNT, &args![found]).u32() {
            let entry = e.call(ENTRY_AT, &args![found, index]).u32();
            let item = e.call(LOOKUP, &args![ENTRY_TABLE_01202548, entry]).u32();
            if item != 0 {
                let mut missing = true;
                let mut slot = (e.mem.u32(count.addr()) as i32).wrapping_sub(1);
                while missing && slot >= 0 {
                    let element = e.call(ELEMENT_ADDRESS, &args![list, slot as u32]).u32();
                    let value = e.call(ENTRY_VALUE, &args![item]).u32();
                    if e.mem.u32(element) == value {
                        missing = false;
                    }
                    slot = slot.wrapping_sub(1);
                }
                if missing {
                    let value = e.call(ENTRY_VALUE, &args![item]).u32();
                    e.with_stack(4, |e, local| {
                        e.mem.set_u32(local.addr(), value);
                        let at = e.mem.u32(count.addr());
                        e.call(ARRAY_SET_AT, &args![list, at, local]);
                    });
                    let counted = e.mem.u32(count.addr()).wrapping_add(1);
                    e.mem.set_u32(count.addr(), counted);
                }
            }
            index = index.wrapping_add(1);
        }
    }
    let as_node = e.vcall(node, AS_NODE_SLOT, &args![]).u32();
    if as_node != 0 {
        let mut child: u32 = 0;
        while child < e.call(CHILD_COUNT, &args![as_node]).u32() {
            let next = e.call(CHILD_AT, &args![as_node, child]).u32();
            fn_0068a8f0(e, this, next, list, count);
            child = child.wrapping_add(1);
        }
    }
}

/// `0068a600` (unit `bstempeffectparticle.cpp`): restarts the particle's impact sounds.
const RESTART_IMPACT_SOUNDS_0068A600: u32 = 0x0068_a600;

// Translated from 0068a8d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, `RET 4`): stores the impact data at +0x58 of the
/// particle effect and restarts its impact sounds (`0068a600`).
pub fn fn_0068a8d0(e: &mut Engine, this: Ptr, impact_data: u32) {
    e.mem.set_u32(this.addr() + 0x58, impact_data);
    e.call(RESTART_IMPACT_SOUNDS_0068A600, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0068_a8f0, fn_0068a8f0(Ptr, u32, u32, Ptr)),
        entry!(0x0068_a8d0, fn_0068a8d0(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const AS_NODE: u32 = 0x0e10_0000;

    /// Doubles that read the test's memory: a looked-up object is the word at +0x10 of the key,
    /// an object's entries are counted at +0 and listed from +4, a node's children likewise
    /// from the block its slot +0xc answers (the word at +0x14 of the node).
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(LOOKUP, |e, a| e.mem.u32(a[1] + 0x10).into_ret());
        e.register(ENTRY_COUNT, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(ENTRY_AT, |e, a| e.mem.u32(a[0] + 4 + 4 * a[1]).into_ret());
        e.register(ENTRY_VALUE, |e, a| e.mem.u32(a[0] + 0x18).into_ret());
        e.register(ELEMENT_ADDRESS, |e, a| {
            (e.mem.u32(a[0] + 4) + 4 * a[1]).into_ret()
        });
        e.register(ARRAY_SET_AT, |e, a| {
            let value = e.mem.u32(a[2]);
            let base = e.mem.u32(a[0] + 4);
            e.mem.set_u32(base + 4 * a[1], value);
            Ret::default()
        });
        e.register(AS_NODE, |e, a| e.mem.u32(a[0] + 0x14).into_ret());
        e.register(CHILD_COUNT, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(CHILD_AT, |e, a| e.mem.u32(a[0] + 4 + 4 * a[1]).into_ret());
        e
    }

    /// A block holding `words` from +0 on.
    fn block(e: &mut Engine, words: &[u32]) -> u32 {
        let block = e.mem.alloc(0x40);
        for (i, word) in words.iter().enumerate() {
            e.mem.set_u32(block + 4 * i as u32, *word);
        }
        block
    }

    /// A key (node or entry) whose lookup answers `found`; a node also answers `children`.
    fn key(e: &mut Engine, vtable: u32, found: u32, children: u32) -> u32 {
        let key = e.mem.alloc(0x20);
        e.mem.set_u32(key, vtable);
        e.mem.set_u32(key + 0x10, found);
        e.mem.set_u32(key + 0x14, children);
        key
    }

    /// An entry object with the value `value` at +0x18.
    fn item(e: &mut Engine, value: u32) -> u32 {
        let item = e.mem.alloc(0x20);
        e.mem.set_u32(item + 0x18, value);
        item
    }

    #[test]
    fn fn_0068a8f0_collects_new_values_through_the_children() {
        let mut e = engine();
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable + AS_NODE_SLOT, AS_NODE);
        let elements = block(&mut e, &[0x111]);
        let list = block(&mut e, &[0, elements]);
        let count = block(&mut e, &[1]);
        // The child: one entry with a new value, no children of its own.
        let new_child = item(&mut e, 0x333);
        let child_entry = key(&mut e, 0, new_child, 0);
        let child_found = block(&mut e, &[1, child_entry]);
        let child = key(&mut e, vtable, child_found, 0);
        // The root: a known value, an entry without an object, a new value; one child.
        let known = item(&mut e, 0x111);
        let new_root = item(&mut e, 0x222);
        let entries = [
            key(&mut e, 0, known, 0),
            key(&mut e, 0, 0, 0),
            key(&mut e, 0, new_root, 0),
        ];
        let found = block(&mut e, &[3, entries[0], entries[1], entries[2]]);
        let children = block(&mut e, &[1, child]);
        let root = key(&mut e, vtable, found, children);
        e.call(0x0068_a8f0, &args![0x0e00_0000u32, root, list, count]);
        assert_eq!(e.mem.u32(count), 3);
        assert_eq!(
            [0, 4, 8].map(|offset| e.mem.u32(elements + offset)),
            [0x111, 0x222, 0x333]
        );

        // A null node: nothing is called.
        e.call_log = Some(vec![]);
        e.call(0x0068_a8f0, &args![0x0e00_0000u32, 0u32, list, count]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }
}

#[cfg(test)]
mod tests_n073 {
    use super::*;

    #[test]
    fn fn_0068a8d0_stores_the_impact_data_and_restarts_the_sounds() {
        let mut e = Engine::new();
        e.register(RESTART_IMPACT_SOUNDS_0068A600, |_, _| Ret::default());
        let this = e.mem.alloc(0x74);
        e.call_log = Some(vec![]);
        e.call(0x0068_a8d0, &args![this, 0x1d1du32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this + 0x58), 0x1d1d);
        assert_eq!(log[1..], [(RESTART_IMPACT_SOUNDS_0068A600, vec![this])]);
    }
}

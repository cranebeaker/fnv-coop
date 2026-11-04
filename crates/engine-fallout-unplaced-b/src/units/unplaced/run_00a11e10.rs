//! `(unplaced)/run_00a11e10` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Three template bodies of the debug-text code (`BSMenu/debugtext.cpp` calls them: `00a11c40`
//! and `00a11d20`): an `NiTPointerList` node removal, its tail removal and a `BSSimpleArray`
//! growth step. Unoptimised code (frame pointer, every value through a stack slot).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::{BSSimpleArray, NiTListItem, NiTPointerList};
use crate::units::platform::MEMORY_MANAGER;

/// The memory manager's free method `00aa4060` (thiscall on `011f6238`, one word: the block).
const MEMORY_FREE: u32 = 0x00aa_4060;
/// `00a1c080` (thiscall on the list, no stack words; `BSMenu/font.cpp` in the engine map): the
/// list's head removal.
const LIST_REMOVE_HEAD: u32 = 0x00a1_c080;
/// `00a0d0c0` (thiscall, two words: the new capacity and the current size; `BSMenu/tile.cpp` in
/// the engine map): the array's reallocation.
const ARRAY_RESIZE: u32 = 0x00a0_d0c0;

fn node_ptr(node: u32) -> Ptr<NiTListItem> {
    Ptr::new(node)
}

// Translated from 00a11e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPointerList` node removal (thiscall, `RET 4`; called by `00a11c40` with the address of a
/// slot holding the node): the head node moves `*position` to its next and is removed by
/// `00a1c080`, the tail node clears `*position` and is removed by `00a11fa0` (each returning
/// what that returns). Any other node is unlinked from its neighbours (`*position` = its next),
/// its element (+8) is taken and cleared, the node is freed through the memory manager
/// (`00aa4060`) and the count drops. Returns the element.
pub fn fn_00a11e10(e: &mut Engine, this: Ptr<NiTPointerList>, position: Ptr) -> u32 {
    let node = e.mem.u32(position.addr());
    if node == e.get(this, NiTPointerList::m_pkHead) {
        let next = e.get(node_ptr(node), NiTListItem::m_pkNext);
        e.mem.set_u32(position.addr(), next);
        return e.call(LIST_REMOVE_HEAD, &args![this]).u32();
    }
    if node == e.get(this, NiTPointerList::m_pkTail) {
        e.mem.set_u32(position.addr(), 0);
        return fn_00a11fa0(e, this);
    }
    let previous = e.get(node_ptr(node), NiTListItem::m_pkPrev);
    let next = e.get(node_ptr(node), NiTListItem::m_pkNext);
    e.mem.set_u32(position.addr(), next);
    if previous != 0 {
        e.set(node_ptr(previous), NiTListItem::m_pkNext, next);
    }
    if next != 0 {
        e.set(node_ptr(next), NiTListItem::m_pkPrev, previous);
    }
    let element = e.get(node_ptr(node), NiTListItem::m_element);
    e.set(node_ptr(node), NiTListItem::m_element, 0);
    e.call(MEMORY_FREE, &args![MEMORY_MANAGER, node]);
    let count = e.get(this, NiTPointerList::m_uiCount).wrapping_sub(1);
    e.set(this, NiTPointerList::m_uiCount, count);
    element
}

// Translated from 00a11ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray` growth step (thiscall, no stack words; called by `00a11d20`): when the size
/// has reached the capacity, an empty array gets a 4-element buffer from its own allocator
/// (vtable +4 with 4) and a capacity of 4; a full one is reallocated (`00a0d0c0` with the new
/// capacity and the size) to capacity + 0x400 above 0x400, else twice the capacity. The size
/// then grows by one; returns the new element's index (size - 1).
pub fn fn_00a11ee0(e: &mut Engine, this: Ptr<BSSimpleArray>) -> u32 {
    if e.get(this, BSSimpleArray::iSize) == e.get(this, BSSimpleArray::iReservedSize) {
        let reserved = e.get(this, BSSimpleArray::iReservedSize);
        if reserved == 0 {
            let buffer = e.vcall(this.addr(), 0x4, &args![4u32]).u32();
            e.set(this, BSSimpleArray::pBuffer, buffer);
            e.set(this, BSSimpleArray::iReservedSize, 4);
        } else {
            let grown = if reserved > 0x400 {
                reserved.wrapping_add(0x400)
            } else {
                reserved << 1
            };
            let size = e.get(this, BSSimpleArray::iSize);
            e.call(ARRAY_RESIZE, &args![this, grown, size]);
            e.set(this, BSSimpleArray::iReservedSize, grown);
        }
    }
    let size = e.get(this, BSSimpleArray::iSize).wrapping_add(1);
    e.set(this, BSSimpleArray::iSize, size);
    e.get(this, BSSimpleArray::iSize).wrapping_sub(1)
}

// Translated from 00a11fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPointerList` tail removal (thiscall, no stack words; called by `00a11e10`): the tail's
/// previous node becomes the tail (its next cleared) or, when there is none, the head is
/// cleared; the old tail's element (+8) is taken and cleared, the node freed through the memory
/// manager (`00aa4060`) and the count drops. Returns the element.
pub fn fn_00a11fa0(e: &mut Engine, this: Ptr<NiTPointerList>) -> u32 {
    let tail = e.get(this, NiTPointerList::m_pkTail);
    let previous = e.get(node_ptr(tail), NiTListItem::m_pkPrev);
    e.set(this, NiTPointerList::m_pkTail, previous);
    let new_tail = e.get(this, NiTPointerList::m_pkTail);
    if new_tail != 0 {
        e.set(node_ptr(new_tail), NiTListItem::m_pkNext, 0);
    } else {
        e.set(this, NiTPointerList::m_pkHead, 0);
    }
    let element = e.get(node_ptr(tail), NiTListItem::m_element);
    e.set(node_ptr(tail), NiTListItem::m_element, 0);
    e.call(MEMORY_FREE, &args![MEMORY_MANAGER, tail]);
    let count = e.get(this, NiTPointerList::m_uiCount).wrapping_sub(1);
    e.set(this, NiTPointerList::m_uiCount, count);
    element
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a1_1e10, fn_00a11e10(Ptr<NiTPointerList>, Ptr) -> u32),
        entry!(0x00a1_1ee0, fn_00a11ee0(Ptr<BSSimpleArray>) -> u32),
        entry!(0x00a1_1fa0, fn_00a11fa0(Ptr<NiTPointerList>) -> u32),
    ]
}

#[cfg(test)]
mod n078_tests {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    /// A list of three nodes (elements 0x11, 0x22, 0x33) and its node addresses.
    fn list(e: &mut Engine) -> (u32, [u32; 3]) {
        let this = e.mem.alloc(0xc);
        let nodes = [e.mem.alloc(0xc), e.mem.alloc(0xc), e.mem.alloc(0xc)];
        for (i, &n) in nodes.iter().enumerate() {
            let next = if i < 2 { nodes[i + 1] } else { 0 };
            let prev = if i > 0 { nodes[i - 1] } else { 0 };
            e.mem.set_u32(n, next);
            e.mem.set_u32(n + 4, prev);
            e.mem.set_u32(n + 8, 0x11 * (i as u32 + 1));
        }
        e.mem.set_u32(this, nodes[0]);
        e.mem.set_u32(this + 4, nodes[2]);
        e.mem.set_u32(this + 8, 3);
        (this, nodes)
    }

    fn rig() -> Engine {
        let mut e = Engine::new();
        e.register(MEMORY_FREE, |_, _| ret(0));
        e.register(LIST_REMOVE_HEAD, |_, _| ret(0x5150));
        e.register(ARRAY_RESIZE, |_, _| ret(0));
        e.call_log = Some(vec![]);
        e
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn remove_node_00a11e10_unlinks_a_middle_node_and_hands_on_the_ends() {
        let mut e = rig();
        let (this, nodes) = list(&mut e);
        let slot = e.mem.alloc(4);
        // Middle node: unlinked, freed, element returned.
        e.mem.set_u32(slot, nodes[1]);
        assert_eq!(e.call(0x00a1_1e10, &args![this, slot]).u32(), 0x22);
        assert_eq!(e.mem.u32(slot), nodes[2]);
        assert_eq!(e.mem.u32(nodes[0]), nodes[2]);
        assert_eq!(e.mem.u32(nodes[2] + 4), nodes[0]);
        assert_eq!(e.mem.u32(nodes[1] + 8), 0);
        assert_eq!(e.mem.u32(this + 8), 2);
        assert_eq!(calls(&e, MEMORY_FREE), vec![vec![MEMORY_MANAGER, nodes[1]]]);
        // Head: the slot moves on and `00a1c080`'s answer comes back.
        e.mem.set_u32(slot, nodes[0]);
        assert_eq!(e.call(0x00a1_1e10, &args![this, slot]).u32(), 0x5150);
        assert_eq!(e.mem.u32(slot), nodes[2]);
        assert_eq!(calls(&e, LIST_REMOVE_HEAD), vec![vec![this]]);
        // Tail: the slot is cleared and the tail removal runs.
        e.mem.set_u32(slot, nodes[2]);
        assert_eq!(e.call(0x00a1_1e10, &args![this, slot]).u32(), 0x33);
        assert_eq!(e.mem.u32(slot), 0);
        assert_eq!(e.mem.u32(this + 4), nodes[0]);
        assert_eq!(e.mem.u32(nodes[0]), 0);
        assert_eq!(e.mem.u32(this + 8), 1);
    }

    #[test]
    fn remove_tail_00a11fa0_clears_the_head_of_a_one_node_list() {
        let mut e = rig();
        let this = e.mem.alloc(0xc);
        let node = e.mem.alloc(0xc);
        e.mem.set_u32(node + 8, 0x77);
        e.mem.set_u32(this, node);
        e.mem.set_u32(this + 4, node);
        e.mem.set_u32(this + 8, 1);
        assert_eq!(e.call(0x00a1_1fa0, &args![this]).u32(), 0x77);
        assert_eq!(e.mem.u32(this), 0);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(node + 8), 0);
        assert_eq!(calls(&e, MEMORY_FREE), vec![vec![MEMORY_MANAGER, node]]);
    }

    #[test]
    fn grow_00a11ee0_allocates_doubles_and_steps_by_0x400() {
        let mut e = rig();
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(vtable + 4, 0x7000_0000);
        e.register(0x7000_0000, |_, a| ret(0x4000 + a[1]));
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this, vtable);
        // Empty: a 4-element buffer.
        assert_eq!(e.call(0x00a1_1ee0, &args![this]).u32(), 0);
        assert_eq!(e.mem.u32(this + 4), 0x4004);
        assert_eq!(e.mem.u32(this + 0xc), 4);
        assert_eq!(e.mem.u32(this + 8), 1);
        // Room left: only the size grows.
        assert_eq!(e.call(0x00a1_1ee0, &args![this]).u32(), 1);
        assert!(calls(&e, ARRAY_RESIZE).is_empty());
        // Full at 4: doubled.
        e.mem.set_u32(this + 8, 4);
        assert_eq!(e.call(0x00a1_1ee0, &args![this]).u32(), 4);
        assert_eq!(calls(&e, ARRAY_RESIZE), vec![vec![this, 8, 4]]);
        assert_eq!(e.mem.u32(this + 0xc), 8);
        // Full above 0x400: + 0x400.
        e.mem.set_u32(this + 8, 0x401);
        e.mem.set_u32(this + 0xc, 0x401);
        assert_eq!(e.call(0x00a1_1ee0, &args![this]).u32(), 0x401);
        assert_eq!(e.mem.u32(this + 0xc), 0x801);
        assert_eq!(e.mem.u32(this + 8), 0x402);
    }
}

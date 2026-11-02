//! `BSMain/run_00c3a3f0` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::{NiTListItem, NiTPointerList};

/// `0045cee0` (`thiscall` on the list's allocator, +8; `RET 4`): gives a node back.
const FREE_NODE: u32 = 0x0045_cee0;

/// The common tail of the three removals (the node is already unlinked): takes the node's
/// element, clears it, gives the node back (`0045cee0`) and counts the list down. Answers the
/// element.
fn take_element(e: &mut Engine, this: Ptr<NiTPointerList>, node: u32) -> u32 {
    let item = Ptr::<NiTListItem>::new(node);
    let element = e.get(item, NiTListItem::m_element);
    e.set(item, NiTListItem::m_element, 0);
    e.call(FREE_NODE, &args![this.addr().wrapping_add(8), node]);
    let count = e.get(this, NiTPointerList::m_uiCount).wrapping_sub(1);
    e.set(this, NiTPointerList::m_uiCount, count);
    element
}

// Translated from 00c3a3f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A pointer list's `RemovePos` (unnamed in the engine map; `thiscall`, `RET 4`: address of
/// the position). The head goes through [`fn_00c3a4b0`] (the position becomes the next
/// node) and the tail through [`fn_00c3a530`] (the position becomes null); any other node is
/// unlinked here, the position moving to the next node. Answers the removed element.
pub fn fn_00c3a3f0(e: &mut Engine, this: Ptr<NiTPointerList>, position: Ptr) -> u32 {
    let node = e.mem.u32(position.addr());
    let item = Ptr::<NiTListItem>::new(node);
    if node == e.get(this, NiTPointerList::m_pkHead) {
        let next = e.get(item, NiTListItem::m_pkNext);
        e.mem.set_u32(position.addr(), next);
        return fn_00c3a4b0(e, this);
    }
    if node == e.get(this, NiTPointerList::m_pkTail) {
        e.mem.set_u32(position.addr(), 0);
        return fn_00c3a530(e, this);
    }
    let previous = e.get(item, NiTListItem::m_pkPrev);
    let next = e.get(item, NiTListItem::m_pkNext);
    e.mem.set_u32(position.addr(), next);
    if previous != 0 {
        e.set(
            Ptr::<NiTListItem>::new(previous),
            NiTListItem::m_pkNext,
            next,
        );
    }
    if next != 0 {
        e.set(
            Ptr::<NiTListItem>::new(next),
            NiTListItem::m_pkPrev,
            previous,
        );
    }
    take_element(e, this, node)
}

// Translated from 00c3a4b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A pointer list's `RemoveHead` (unnamed in the engine map; `thiscall`): the head moves to
/// the next node (whose previous link is cleared, or the tail when the list empties) and the
/// old head's element is answered, the node given back.
pub fn fn_00c3a4b0(e: &mut Engine, this: Ptr<NiTPointerList>) -> u32 {
    let node = e.get(this, NiTPointerList::m_pkHead);
    let next = e.get(Ptr::<NiTListItem>::new(node), NiTListItem::m_pkNext);
    e.set(this, NiTPointerList::m_pkHead, next);
    let head = e.get(this, NiTPointerList::m_pkHead);
    if head != 0 {
        e.set(Ptr::<NiTListItem>::new(head), NiTListItem::m_pkPrev, 0);
    } else {
        e.set(this, NiTPointerList::m_pkTail, 0);
    }
    take_element(e, this, node)
}

// Translated from 00c3a530 (decompiled, FalloutNV.exe 1.4.0.525)
/// A pointer list's `RemoveTail` (unnamed in the engine map; `thiscall`): the tail moves to
/// the previous node (whose next link is cleared, or the head when the list empties) and the
/// old tail's element is answered, the node given back.
pub fn fn_00c3a530(e: &mut Engine, this: Ptr<NiTPointerList>) -> u32 {
    let node = e.get(this, NiTPointerList::m_pkTail);
    let previous = e.get(Ptr::<NiTListItem>::new(node), NiTListItem::m_pkPrev);
    e.set(this, NiTPointerList::m_pkTail, previous);
    let tail = e.get(this, NiTPointerList::m_pkTail);
    if tail != 0 {
        e.set(Ptr::<NiTListItem>::new(tail), NiTListItem::m_pkNext, 0);
    } else {
        e.set(this, NiTPointerList::m_pkHead, 0);
    }
    take_element(e, this, node)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c3_a3f0, fn_00c3a3f0(Ptr<NiTPointerList>, Ptr) -> u32),
        entry!(0x00c3_a4b0, fn_00c3a4b0(Ptr<NiTPointerList>) -> u32),
        entry!(0x00c3_a530, fn_00c3a530(Ptr<NiTPointerList>) -> u32),
    ]
}

#[cfg(test)]
mod n069_tests {
    use super::*;

    /// A list of the given elements; answers the list and its nodes.
    fn list(e: &mut Engine, elements: &[u32]) -> (u32, Vec<u32>) {
        let list = e.mem.alloc(0xc);
        let nodes: Vec<u32> = elements.iter().map(|_| e.mem.alloc(0xc)).collect();
        for (i, node) in nodes.iter().enumerate() {
            e.mem.set_u32(node + 8, elements[i]);
            if i + 1 < nodes.len() {
                e.mem.set_u32(*node, nodes[i + 1]);
            }
            if i > 0 {
                e.mem.set_u32(node + 4, nodes[i - 1]);
            }
        }
        e.mem.set_u32(list, nodes.first().copied().unwrap_or(0));
        e.mem.set_u32(list + 4, nodes.last().copied().unwrap_or(0));
        e.mem.set_u32(list + 8, nodes.len() as u32);
        (list, nodes)
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(FREE_NODE, |_, _| Ret::default());
        e
    }

    fn remove_at(e: &mut Engine, list: u32, node: u32) -> (u32, u32) {
        let position = e.mem.alloc(4);
        e.mem.set_u32(position, node);
        let element = e.call(0x00c3_a3f0, &args![list, position]).u32();
        (element, e.mem.u32(position))
    }

    #[test]
    fn remove_position_unlinks_a_middle_node() {
        let mut e = engine();
        let (list, nodes) = list(&mut e, &[0x10, 0x20, 0x30]);
        e.call_log = Some(vec![]);
        assert_eq!(remove_at(&mut e, list, nodes[1]), (0x20, nodes[2]));
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(FREE_NODE, vec![list + 8, nodes[1]])));
        assert_eq!(e.mem.u32(nodes[0]), nodes[2]);
        assert_eq!(e.mem.u32(nodes[2] + 4), nodes[0]);
        assert_eq!(e.mem.u32(nodes[1] + 8), 0);
        assert_eq!(e.mem.u32(list + 8), 2);
    }

    #[test]
    fn remove_position_of_the_head_and_the_tail() {
        let mut e = engine();
        let (list, nodes) = list(&mut e, &[0x10, 0x20, 0x30]);
        assert_eq!(remove_at(&mut e, list, nodes[0]), (0x10, nodes[1]));
        assert_eq!(e.mem.u32(list), nodes[1]);
        assert_eq!(e.mem.u32(nodes[1] + 4), 0);
        assert_eq!(remove_at(&mut e, list, nodes[2]), (0x30, 0));
        assert_eq!(e.mem.u32(list + 4), nodes[1]);
        assert_eq!(e.mem.u32(nodes[1]), 0);
        assert_eq!(e.mem.u32(list + 8), 1);
    }

    #[test]
    fn removing_the_last_node_empties_the_list_both_ways() {
        let mut e = engine();
        let (list, _) = list(&mut e, &[0x10]);
        assert_eq!(e.call(0x00c3_a4b0, &args![list]).u32(), 0x10);
        assert_eq!(
            (e.mem.u32(list), e.mem.u32(list + 4), e.mem.u32(list + 8)),
            (0, 0, 0)
        );
        let (list, _) = self::list(&mut e, &[0x20]);
        assert_eq!(e.call(0x00c3_a530, &args![list]).u32(), 0x20);
        assert_eq!(
            (e.mem.u32(list), e.mem.u32(list + 4), e.mem.u32(list + 8)),
            (0, 0, 0)
        );
    }
}

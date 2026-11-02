//! `BSMain/run_00c3a250` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::{NiTListItem, NiTPointerList};

/// The list's `RemovePos` (`00c3a3f0`, `thiscall`, `RET 4`: address of the position; answers
/// the removed element).
const REMOVE_POSITION: u32 = 0x00c3_a3f0;
/// `0066b0d0` (`thiscall`, `RET 4`): assigns a counted pointer (the node's element) the
/// pointer given.
const ASSIGN_POINTER: u32 = 0x0066_b0d0;
/// `0045cee0` (`thiscall` on the list's allocator, +8; `RET 4`): gives a node back.
const FREE_NODE: u32 = 0x0045_cee0;

// Translated from 00c3a250 (decompiled, FalloutNV.exe 1.4.0.525)
/// A list's `Remove` (unnamed in the engine map; `thiscall`, `RET 4`: address of the
/// element). Finds the first node holding the element (the compiler set the cursor to 0 and
/// then tested it before loading the head) and answers what `RemovePos` (`00c3a3f0`, given
/// the address of a local holding the node) answers; without such a node, the element.
pub fn fn_00c3a250(e: &mut Engine, this: Ptr<NiTPointerList>, element: Ptr) -> u32 {
    let mut node = std::hint::black_box(0u32);
    if node == 0 {
        node = e.get(this, NiTPointerList::m_pkHead);
    }
    let mut found = 0u32;
    while node != 0 {
        let current = node;
        let item = Ptr::<NiTListItem>::new(node);
        node = e.get(item, NiTListItem::m_pkNext);
        if e.mem.u32(element.addr()) == e.get(item, NiTListItem::m_element) {
            found = current;
            break;
        }
    }
    if found == 0 {
        return e.mem.u32(element.addr());
    }
    e.with_stack(4, |e, position| {
        e.mem.set_u32(position.addr(), found);
        e.call(REMOVE_POSITION, &args![this, position]).u32()
    })
}

// Translated from 00c3a2e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The list of counted pointers' `RemoveAll` (unnamed in the engine map; `thiscall`): for
/// each node from the head, its element is set to null (`0066b0d0`) and the node given back
/// (`0045cee0`); then the count, the head and the tail are cleared.
pub fn fn_00c3a2e0(e: &mut Engine, this: Ptr<NiTPointerList>) {
    let mut node = e.get(this, NiTPointerList::m_pkHead);
    while node != 0 {
        let current = node;
        node = e.get(Ptr::<NiTListItem>::new(current), NiTListItem::m_pkNext);
        e.call(ASSIGN_POINTER, &args![current.wrapping_add(8), 0u32]);
        e.call(FREE_NODE, &args![this.addr().wrapping_add(8), current]);
    }
    e.set(this, NiTPointerList::m_uiCount, 0);
    e.set(this, NiTPointerList::m_pkHead, 0);
    e.set(this, NiTPointerList::m_pkTail, 0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c3_a250, fn_00c3a250(Ptr<NiTPointerList>, Ptr) -> u32),
        entry!(0x00c3_a2e0, fn_00c3a2e0(Ptr<NiTPointerList>)),
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

    #[test]
    fn remove_finds_the_node_or_answers_the_element() {
        let mut e = Engine::new();
        let seen = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let inner = seen.clone();
        e.register_double(REMOVE_POSITION, move |e, a| {
            inner.borrow_mut().push((a[0], e.mem.u32(a[1])));
            0x55u32.into_ret()
        });
        let (list, nodes) = list(&mut e, &[0x10, 0x20, 0x20]);
        let element = e.mem.alloc(4);
        e.mem.set_u32(element, 0x20);
        assert_eq!(e.call(0x00c3_a250, &args![list, element]).u32(), 0x55);
        assert_eq!(*seen.borrow(), vec![(list, nodes[1])]);
        e.mem.set_u32(element, 0x30);
        assert_eq!(e.call(0x00c3_a250, &args![list, element]).u32(), 0x30);
        assert_eq!(seen.borrow().len(), 1);
    }

    #[test]
    fn remove_all_releases_every_node() {
        let mut e = Engine::new();
        e.register(ASSIGN_POINTER, |_, _| Ret::default());
        e.register(FREE_NODE, |_, _| Ret::default());
        let (list, nodes) = list(&mut e, &[0x10, 0x20]);
        e.call_log = Some(vec![]);
        e.call(0x00c3_a2e0, &args![list]);
        let log: Vec<_> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|c| c.0 != 0x00c3_a2e0)
            .collect();
        assert_eq!(
            log,
            vec![
                (ASSIGN_POINTER, vec![nodes[0] + 8, 0]),
                (FREE_NODE, vec![list + 8, nodes[0]]),
                (ASSIGN_POINTER, vec![nodes[1] + 8, 0]),
                (FREE_NODE, vec![list + 8, nodes[1]]),
            ]
        );
        assert_eq!(e.mem.u32(list), 0);
        assert_eq!(e.mem.u32(list + 4), 0);
        assert_eq!(e.mem.u32(list + 8), 0);
    }
}

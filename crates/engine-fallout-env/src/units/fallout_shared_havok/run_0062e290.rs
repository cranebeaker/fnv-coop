//! `fallout shared/havok/run_0062e290` (Xbox PDB source unit), subsystem `fallout shared/havok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Offset of the first node's pointer in the list object.
const HEAD: u32 = 0x04;
/// Offset of the word cleared after the head (the last node's pointer in
/// the shape of an `NiTList`).
const TAIL: u32 = 0x08;
/// Offset of the word cleared first (the element count in the shape of an
/// `NiTList`).
const COUNT: u32 = 0x0c;
/// The list's virtual function at `+8` that frees a node.
const SLOT_FREE_NODE: u32 = 0x08;

// Translated from 0062e290 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empties a list: walks the nodes from the pointer at `+4` (a node's own
/// first word is its next node, read before the node is freed), calls the
/// list's virtual function at `+8` on each node, then clears `+0x0c`, `+4`
/// and `+8` in that order. The layout is that of an `NiTList` (vtable,
/// head, tail, count); the exe does not name this function.
pub fn fn_0062e290(e: &mut Engine, this: Ptr) {
    let mut node = e.mem.u32(this.addr() + HEAD);
    while node != 0 {
        let next = e.mem.u32(node);
        e.vcall(this.addr(), SLOT_FREE_NODE, &args![node]);
        node = next;
    }
    e.mem.set_u32(this.addr() + COUNT, 0);
    e.mem.set_u32(this.addr() + HEAD, 0);
    e.mem.set_u32(this.addr() + TAIL, 0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0062_e290, fn_0062e290(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FREE_NODE: u32 = 0x0700_0008;

    /// What the free-node double saw: (list, node, the list's `+0x0c` word).
    type Seen = Rc<RefCell<Vec<(u32, u32, u32)>>>;

    /// A list with `count` nodes (each a 0xc-byte block whose first word is
    /// the next node), a vtable whose slot 8 is `FREE_NODE`, and a double
    /// that records the nodes it is given and what the list's count word
    /// held at that time, then overwrites the node's link as freeing would.
    fn setup(count: usize) -> (Engine, u32, Vec<u32>, Seen) {
        let mut e = Engine::new();
        let list = e.mem.alloc(0x10);
        let table = e.mem.alloc(0x10);
        e.mem.set_u32(table + 8, FREE_NODE);
        e.mem.set_u32(list, table);
        let nodes: Vec<u32> = (0..count).map(|_| e.mem.alloc(0xc)).collect();
        for pair in nodes.windows(2) {
            e.mem.set_u32(pair[0], pair[1]);
        }
        if let Some(first) = nodes.first() {
            e.mem.set_u32(list + 4, *first);
            e.mem.set_u32(list + 8, *nodes.last().unwrap());
            e.mem.set_u32(list + 0xc, count as u32);
        }
        let seen = Rc::new(RefCell::new(Vec::new()));
        let record = Rc::clone(&seen);
        e.register_double(FREE_NODE, move |e, a| {
            record
                .borrow_mut()
                .push((a[0], a[1], e.mem.u32(a[0] + 0xc)));
            e.mem.set_u32(a[1], 0xdead_beef);
            Ret::default()
        });
        (e, list, nodes, seen)
    }

    #[test]
    fn every_node_is_freed_in_order_even_if_freeing_destroys_its_link() {
        let (mut e, list, nodes, seen) = setup(3);
        e.call(0x0062_e290, &args![list]);
        let freed: Vec<u32> = seen.borrow().iter().map(|r| r.1).collect();
        assert_eq!(freed, nodes);
        assert!(seen.borrow().iter().all(|r| r.0 == list));
        // The list's count was still set while the nodes were freed.
        assert!(seen.borrow().iter().all(|r| r.2 == 3));
        assert_eq!(e.mem.u32(list + 4), 0);
        assert_eq!(e.mem.u32(list + 8), 0);
        assert_eq!(e.mem.u32(list + 0xc), 0);
    }

    #[test]
    fn an_empty_list_frees_nothing_and_stays_cleared() {
        let (mut e, list, _, seen) = setup(0);
        e.mem.set_u32(list + 8, 0x77);
        e.mem.set_u32(list + 0xc, 5);
        e.call(0x0062_e290, &args![list]);
        assert!(seen.borrow().is_empty());
        assert_eq!(e.mem.u32(list + 4), 0);
        assert_eq!(e.mem.u32(list + 8), 0);
        assert_eq!(e.mem.u32(list + 0xc), 0);
    }
}

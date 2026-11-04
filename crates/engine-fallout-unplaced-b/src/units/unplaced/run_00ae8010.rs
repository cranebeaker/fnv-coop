//! `(unplaced)/run_00ae8010` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Both functions work on an `NiTPointerList` (head `+0`, tail `+4`, then its allocator at
//! `+8`, whose first word is the count; a node holds its next node at `+0`, its previous
//! node at `+4` and its item at `+8`).

#[allow(unused_imports)]
use crate::prelude::*;

/// `0045cee0` (`thiscall` on the allocator at `+8`, `RET 4`): frees a node.
const FREE_NODE: u32 = 0x0045_cee0;
/// `NiTPointerListBase<..>::RemoveHead` (engine map, `00ae7f90`, `thiscall`, no stack word).
const REMOVE_HEAD: u32 = 0x00ae_7f90;

// Translated from 00ae8010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (the list's `RemoveTail`, `ECX` only): unlinks the tail node
/// (the previous node becomes the tail, its next link cleared, or the head is cleared when
/// there is none), clears and answers its item, frees the node (`0045cee0`) and counts one
/// fewer.
pub fn fn_00ae8010(e: &mut Engine, this: Ptr) -> u32 {
    let list = this.addr();
    let node = e.mem.u32(list + 4);
    let previous = e.mem.u32(e.mem.u32(list + 4) + 4);
    e.mem.set_u32(list + 4, previous);
    if e.mem.u32(list + 4) == 0 {
        e.mem.set_u32(list, 0);
    } else {
        let tail = e.mem.u32(list + 4);
        e.mem.set_u32(tail, 0);
    }
    let item = e.mem.u32(node + 8);
    e.mem.set_u32(node + 8, 0);
    e.call(FREE_NODE, &args![list + 8, node]);
    let count = e.mem.u32(list + 8).wrapping_sub(1);
    e.mem.set_u32(list + 8, count);
    item
}

// Translated from 00ae8090 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (the list's `RemovePos`, `thiscall`, `RET 4`: the address of a
/// position word): removes the node at the position and answers its item. The head goes
/// through `RemoveHead` (`00ae7f90`, the position moved to the next node), the tail through
/// `00ae8010` (the position cleared); a node inside is unlinked here (the position moved to
/// the next node), its item cleared, the node freed and the count lowered.
pub fn fn_00ae8090(e: &mut Engine, this: Ptr, position: Ptr) -> u32 {
    let (list, at) = (this.addr(), position.addr());
    let node = e.mem.u32(at);
    if node == e.mem.u32(list) {
        let next = e.mem.u32(node);
        e.mem.set_u32(at, next);
        return e.call(REMOVE_HEAD, &args![this]).u32();
    }
    if node == e.mem.u32(list + 4) {
        e.mem.set_u32(at, 0);
        return fn_00ae8010(e, this);
    }
    let previous = e.mem.u32(node + 4);
    let next = e.mem.u32(node);
    e.mem.set_u32(at, next);
    if previous != 0 {
        e.mem.set_u32(previous, next);
    }
    if next != 0 {
        e.mem.set_u32(next + 4, previous);
    }
    let item = e.mem.u32(node + 8);
    e.mem.set_u32(node + 8, 0);
    e.call(FREE_NODE, &args![list + 8, node]);
    let count = e.mem.u32(list + 8).wrapping_sub(1);
    e.mem.set_u32(list + 8, count);
    item
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ae_8010, fn_00ae8010(Ptr) -> u32),
        entry!(0x00ae_8090, fn_00ae8090(Ptr, Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A list of three nodes with items 1, 2 and 3.
    fn three(e: &mut Engine) -> (u32, [u32; 3]) {
        let nodes = [e.mem.alloc(0xc), e.mem.alloc(0xc), e.mem.alloc(0xc)];
        for (index, node) in nodes.iter().enumerate() {
            let next = if index < 2 { nodes[index + 1] } else { 0 };
            let previous = if index > 0 { nodes[index - 1] } else { 0 };
            e.mem.set_u32(*node, next);
            e.mem.set_u32(node + 4, previous);
            e.mem.set_u32(node + 8, index as u32 + 1);
        }
        let list = e.mem.alloc(0xc);
        e.mem.set_u32(list, nodes[0]);
        e.mem.set_u32(list + 4, nodes[2]);
        e.mem.set_u32(list + 8, 3);
        (list, nodes)
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(FREE_NODE, |_, _| Ret::default());
        e.register(REMOVE_HEAD, |_, _| 0x99u32.into_ret());
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn removing_the_tail() {
        let mut e = engine();
        let (list, nodes) = three(&mut e);
        assert_eq!(e.call(0x00ae_8010, &args![list]).u32(), 3);
        assert_eq!(e.mem.u32(list + 4), nodes[1]);
        assert_eq!(e.mem.u32(nodes[1]), 0);
        assert_eq!(e.mem.u32(nodes[2] + 8), 0);
        assert_eq!(e.mem.u32(list + 8), 2);
        // A single node: the head is cleared too.
        let single = e.mem.alloc(0xc);
        let node = e.mem.alloc(0xc);
        e.mem.set_u32(node + 8, 5);
        e.mem.set_u32(single, node);
        e.mem.set_u32(single + 4, node);
        e.mem.set_u32(single + 8, 1);
        assert_eq!(e.call(0x00ae_8010, &args![single]).u32(), 5);
        assert_eq!((e.mem.u32(single), e.mem.u32(single + 4)), (0, 0));
    }

    #[test]
    fn removing_by_position() {
        let mut e = engine();
        let (list, nodes) = three(&mut e);
        let position = e.mem.alloc(4);
        // The middle node: unlinked here.
        e.mem.set_u32(position, nodes[1]);
        assert_eq!(e.call(0x00ae_8090, &args![list, position]).u32(), 2);
        assert_eq!(e.mem.u32(position), nodes[2]);
        assert_eq!(
            (e.mem.u32(nodes[0]), e.mem.u32(nodes[2] + 4)),
            (nodes[2], nodes[0])
        );
        assert_eq!(e.mem.u32(list + 8), 2);
        // The tail: through 00ae8010, the position cleared.
        assert_eq!(e.call(0x00ae_8090, &args![list, position]).u32(), 3);
        assert_eq!(e.mem.u32(position), 0);
        // The head: through RemoveHead, the position moved on.
        e.mem.set_u32(position, nodes[0]);
        assert_eq!(e.call(0x00ae_8090, &args![list, position]).u32(), 0x99);
        assert_eq!(e.mem.u32(position), 0);
        let frees = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == FREE_NODE)
            .count();
        assert_eq!(frees, 2);
    }
}

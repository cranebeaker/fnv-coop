//! `fallout shared/run_0049c5d0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The first node of the list (`00559450`: the word at `+0` of the list).
const FIRST_NODE: u32 = 0x0055_9450;
/// Steps through the list (`0057cbe0`, `RET 4`): takes the address of a
/// node-pointer variable, advances it to the node's next pointer (the word
/// at the node's `+0`) and returns the address of the node's value (`+8`).
const NEXT_VALUE: u32 = 0x0057_cbe0;
/// Frees a list node (`00559c90`, `thiscall` on the list, one stack word:
/// the node).
const FREE_NODE: u32 = 0x0055_9c90;

// Translated from 0049c5d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes the node that the node-pointer variable at `cursor` points at
/// from the doubly linked list `this` (head at `+0`, tail at `+4`, count at
/// `+8`; node: next `+0`, previous `+4`, value `+8`), leaves the variable on
/// the following node (null when the tail was removed) and returns the
/// removed node's value. The head and the tail go through their own
/// functions (`0049c6d0`, `0049c740`). `__thiscall`, one stack word.
/// The class is unconfirmed (no engine-map name).
pub fn fn_0049c5d0(e: &mut Engine, this: Ptr, cursor: Ptr) -> u32 {
    let list = this.addr();
    let node = e.mem.u32(cursor.addr());
    if node == e.mem.u32(list) {
        let next = e.mem.u32(node);
        e.mem.set_u32(cursor.addr(), next);
        return fn_0049c6d0(e, this);
    }
    if node == e.mem.u32(list + 4) {
        e.mem.set_u32(cursor.addr(), 0);
        return fn_0049c740(e, this);
    }
    let previous = e.mem.u32(node + 4);
    let next = e.mem.u32(node);
    e.mem.set_u32(cursor.addr(), next);
    if previous != 0 {
        e.mem.set_u32(previous, next);
    }
    if next != 0 {
        e.mem.set_u32(next + 4, previous);
    }
    let value = e.mem.u32(node + 8);
    e.call(FREE_NODE, &args![this, node]);
    let count = e.mem.u32(list + 8);
    e.mem.set_u32(list + 8, count.wrapping_sub(1));
    value
}

// Translated from 0049c680 (decompiled, FalloutNV.exe 1.4.0.525)
/// Finds the first node of the list `this`, from `start` (or from the first
/// node when `start` is null), whose value word equals the word at `wanted`;
/// returns that node, or null. `__thiscall`, two stack words (`RET 8`).
pub fn fn_0049c680(e: &mut Engine, this: Ptr, wanted: Ptr, start: Ptr) -> Ptr {
    let mut node = start.addr();
    if node == 0 {
        node = e.call(FIRST_NODE, &args![this]).u32();
    }
    while node != 0 {
        let current = node;
        // The game passes the address of its local `node`, which the step
        // advances.
        let value = e.with_stack(4, |e, variable| {
            e.mem.set_u32(variable.addr(), node);
            let value = e.call(NEXT_VALUE, &args![this, variable]).u32();
            node = e.mem.u32(variable.addr());
            value
        });
        if e.mem.u32(wanted.addr()) == e.mem.u32(value) {
            return Ptr::new(current);
        }
    }
    Ptr::NULL
}

// Translated from 0049c6d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes the head node of the list `this` (layout as in `0049c5d0`):
/// the next node becomes the head (its previous link is cleared; an empty
/// list gets a null tail), the node is freed, the count drops by one and the
/// node's value is returned. `__thiscall`, no stack words. An empty list is
/// not checked (the game reads through the null head).
pub fn fn_0049c6d0(e: &mut Engine, this: Ptr) -> u32 {
    let list = this.addr();
    let node = e.mem.u32(list);
    let next = e.mem.u32(node);
    e.mem.set_u32(list, next);
    if next != 0 {
        e.mem.set_u32(next + 4, 0);
    } else {
        e.mem.set_u32(list + 4, 0);
    }
    let value = e.mem.u32(node + 8);
    e.call(FREE_NODE, &args![this, node]);
    let count = e.mem.u32(list + 8);
    e.mem.set_u32(list + 8, count.wrapping_sub(1));
    value
}

// Translated from 0049c740 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes the tail node of the list `this` (layout as in `0049c5d0`): the
/// previous node becomes the tail (its next link is cleared; an empty list
/// gets a null head), the node is freed, the count drops by one and the
/// node's value is returned. `__thiscall`, no stack words. An empty list is
/// not checked.
pub fn fn_0049c740(e: &mut Engine, this: Ptr) -> u32 {
    let list = this.addr();
    let node = e.mem.u32(list + 4);
    let previous = e.mem.u32(node + 4);
    e.mem.set_u32(list + 4, previous);
    if previous != 0 {
        e.mem.set_u32(previous, 0);
    } else {
        e.mem.set_u32(list, 0);
    }
    let value = e.mem.u32(node + 8);
    e.call(FREE_NODE, &args![this, node]);
    let count = e.mem.u32(list + 8);
    e.mem.set_u32(list + 8, count.wrapping_sub(1));
    value
}

/// Three-way compare of two `float`s as the code's `FCOMPP` tests do:
/// 1 when `first > second`, else -1 when `first < second`, else 0 (equal or
/// unordered: a NaN on either side gives 0).
fn compare_floats(first: f32, second: f32) -> u32 {
    if first > second {
        1
    } else if first < second {
        u32::MAX
    } else {
        0
    }
}

// Translated from 0049c7b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Comparison function (`cdecl`, two stack words) for sorting: the
/// arguments are addresses of `float`s; returns 1, -1 or 0 as the first is
/// greater than, less than, or equal to / unordered with the second. The
/// engine map has no name for it (it is passed as a sort callback).
pub fn fn_0049c7b0(e: &mut Engine, first: Ptr, second: Ptr) -> u32 {
    compare_floats(e.mem.f32(first.addr()), e.mem.f32(second.addr()))
}

// Translated from 0049c800 (decompiled, FalloutNV.exe 1.4.0.525)
/// Same comparison as `0049c7b0`, on the `float` at `+4` of each argument.
pub fn fn_0049c800(e: &mut Engine, first: Ptr, second: Ptr) -> u32 {
    compare_floats(e.mem.f32(first.addr() + 4), e.mem.f32(second.addr() + 4))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0049c5d0, fn_0049c5d0(Ptr, Ptr) -> u32),
        entry!(0x0049c680, fn_0049c680(Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x0049c6d0, fn_0049c6d0(Ptr) -> u32),
        entry!(0x0049c740, fn_0049c740(Ptr) -> u32),
        entry!(0x0049c7b0, fn_0049c7b0(Ptr, Ptr) -> u32),
        entry!(0x0049c800, fn_0049c800(Ptr, Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// A list of three nodes (next at +0, value at +8) holding 10, 20, 30,
    /// with doubles for the two list functions. Returns (list, nodes).
    fn list(e: &mut Engine) -> (Ptr, [u32; 3]) {
        let nodes = [e.mem.alloc(12), e.mem.alloc(12), e.mem.alloc(12)];
        for (i, node) in nodes.iter().enumerate() {
            e.mem.set_u32(*node, nodes.get(i + 1).copied().unwrap_or(0));
            e.mem.set_u32(*node + 8, 10 * (i as u32 + 1));
        }
        let list: Ptr = Ptr::new(e.mem.alloc(4));
        e.mem.set_u32(list.addr(), nodes[0]);
        e.register(FIRST_NODE, |e, a| Ret {
            eax: e.mem.u32(a[0]),
            ..Ret::default()
        });
        e.register(NEXT_VALUE, |e, a| {
            let node = e.mem.u32(a[1]);
            e.mem.set_u32(a[1], e.mem.u32(node));
            Ret {
                eax: node + 8,
                ..Ret::default()
            }
        });
        (list, nodes)
    }

    fn wanted(e: &mut Engine, value: u32) -> Ptr {
        let p: Ptr = Ptr::new(e.mem.alloc(4));
        e.mem.set_u32(p.addr(), value);
        p
    }

    #[test]
    fn finds_the_node_holding_the_value() {
        let mut e = Engine::new();
        let (list, nodes) = list(&mut e);
        let twenty = wanted(&mut e, 20);
        let found = e.call(0x0049_c680, &args![list, twenty, Ptr::<()>::NULL]);
        assert_eq!(found.u32(), nodes[1]);
    }

    #[test]
    fn a_start_node_skips_the_earlier_ones() {
        let mut e = Engine::new();
        let (list, nodes) = list(&mut e);
        let ten = wanted(&mut e, 10);
        // Starting from the second node the first (10) is never seen.
        let found = e.call(0x0049_c680, &args![list, ten, Ptr::<()>::new(nodes[1])]);
        assert_eq!(found.u32(), 0);
        let thirty = wanted(&mut e, 30);
        let found = e.call(0x0049_c680, &args![list, thirty, Ptr::<()>::new(nodes[1])]);
        assert_eq!(found.u32(), nodes[2]);
    }

    #[test]
    fn a_missing_value_gives_null() {
        let mut e = Engine::new();
        let (list, _) = list(&mut e);
        let missing = wanted(&mut e, 99);
        assert_eq!(
            e.call(0x0049_c680, &args![list, missing, Ptr::<()>::NULL])
                .u32(),
            0
        );
    }

    /// A three-node doubly linked list (head, middle, tail; values 10, 20,
    /// 30) with a double for the node free that records the freed nodes.
    fn removal_list(e: &mut Engine) -> (Ptr, [u32; 3], Rc<RefCell<Vec<u32>>>) {
        let nodes = [e.mem.alloc(12), e.mem.alloc(12), e.mem.alloc(12)];
        for (i, node) in nodes.iter().enumerate() {
            e.mem.set_u32(*node, nodes.get(i + 1).copied().unwrap_or(0));
            e.mem
                .set_u32(*node + 4, if i == 0 { 0 } else { nodes[i - 1] });
            e.mem.set_u32(*node + 8, 10 * (i as u32 + 1));
        }
        let list: Ptr = Ptr::new(e.mem.alloc(12));
        e.mem.set_u32(list.addr(), nodes[0]);
        e.mem.set_u32(list.addr() + 4, nodes[2]);
        e.mem.set_u32(list.addr() + 8, 3);
        let freed = Rc::new(RefCell::new(Vec::new()));
        let log = freed.clone();
        e.register_double(FREE_NODE, move |_, a| {
            log.borrow_mut().push(a[1]);
            Ret::default()
        });
        (list, nodes, freed)
    }

    fn cursor(e: &mut Engine, node: u32) -> Ptr {
        let p: Ptr = Ptr::new(e.mem.alloc(4));
        e.mem.set_u32(p.addr(), node);
        p
    }

    #[test]
    fn removing_the_head() {
        let mut e = Engine::new();
        let (list, nodes, freed) = removal_list(&mut e);
        let at = cursor(&mut e, nodes[0]);
        assert_eq!(e.call(0x0049_c5d0, &args![list, at]).u32(), 10);
        assert_eq!(e.mem.u32(at.addr()), nodes[1]);
        assert_eq!(e.mem.u32(list.addr()), nodes[1]);
        assert_eq!(e.mem.u32(nodes[1] + 4), 0);
        assert_eq!(e.mem.u32(list.addr() + 8), 2);
        assert_eq!(*freed.borrow(), vec![nodes[0]]);
    }

    #[test]
    fn removing_the_tail_nulls_the_cursor() {
        let mut e = Engine::new();
        let (list, nodes, freed) = removal_list(&mut e);
        let at = cursor(&mut e, nodes[2]);
        assert_eq!(e.call(0x0049_c5d0, &args![list, at]).u32(), 30);
        assert_eq!(e.mem.u32(at.addr()), 0);
        assert_eq!(e.mem.u32(list.addr() + 4), nodes[1]);
        assert_eq!(e.mem.u32(nodes[1]), 0);
        assert_eq!(e.mem.u32(list.addr() + 8), 2);
        assert_eq!(*freed.borrow(), vec![nodes[2]]);
    }

    #[test]
    fn removing_a_middle_node_relinks_its_neighbours() {
        let mut e = Engine::new();
        let (list, nodes, freed) = removal_list(&mut e);
        let at = cursor(&mut e, nodes[1]);
        assert_eq!(e.call(0x0049_c5d0, &args![list, at]).u32(), 20);
        assert_eq!(e.mem.u32(at.addr()), nodes[2]);
        assert_eq!(e.mem.u32(nodes[0]), nodes[2]);
        assert_eq!(e.mem.u32(nodes[2] + 4), nodes[0]);
        assert_eq!(e.mem.u32(list.addr() + 8), 2);
        assert_eq!(*freed.borrow(), vec![nodes[1]]);
    }

    #[test]
    fn removing_the_only_node_empties_the_list() {
        let mut e = Engine::new();
        let (list, nodes, _) = removal_list(&mut e);
        e.mem.set_u32(nodes[0], 0);
        e.mem.set_u32(list.addr() + 4, nodes[0]);
        e.mem.set_u32(list.addr() + 8, 1);
        assert_eq!(e.call(0x0049_c6d0, &args![list]).u32(), 10);
        assert_eq!(e.mem.u32(list.addr()), 0);
        assert_eq!(e.mem.u32(list.addr() + 4), 0);
        e.mem.set_u32(list.addr(), nodes[1]);
        e.mem.set_u32(list.addr() + 4, nodes[1]);
        e.mem.set_u32(nodes[1] + 4, 0);
        assert_eq!(e.call(0x0049_c740, &args![list]).u32(), 20);
        assert_eq!(e.mem.u32(list.addr()), 0);
        assert_eq!(e.mem.u32(list.addr() + 4), 0);
    }

    fn float_pair(e: &mut Engine, first: f32, second: f32, at: u32) -> (Ptr, Ptr) {
        let a = e.mem.alloc(8);
        let b = e.mem.alloc(8);
        e.mem.set_f32(a + at, first);
        e.mem.set_f32(b + at, second);
        (Ptr::new(a), Ptr::new(b))
    }

    #[test]
    fn float_comparison_orders_and_treats_nan_as_equal() {
        let mut e = Engine::new();
        for (addr, at) in [(0x0049_c7b0u32, 0u32), (0x0049_c800, 4)] {
            for (x, y, want) in [
                (2.0, 1.0, 1),
                (1.0, 2.0, u32::MAX),
                (1.5, 1.5, 0),
                (f32::NAN, 1.0, 0),
                (1.0, f32::NAN, 0),
            ] {
                let (a, b) = float_pair(&mut e, x, y, at);
                assert_eq!(e.call(addr, &args![a, b]).u32(), want, "{x} {y}");
            }
        }
    }
}

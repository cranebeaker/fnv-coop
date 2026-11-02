//! `BSMain/run_00c5bfb0` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Import slots of `InterlockedIncrement` and `InterlockedDecrement` (`stdcall`, one word).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `0066b0d0` (`thiscall`, `RET 4`): assigns the counted pointer at the address it is called
/// on.
const ASSIGN_POINTER: u32 = 0x0066_b0d0;
/// `0045cee0` (`thiscall` on the list's +8, `RET 4`): gives the node back.
const FREE_NODE: u32 = 0x0045_cee0;
/// `00c5c290` (`thiscall`, `RET 4`): removes the tail node into the word given.
const REMOVE_TAIL: u32 = 0x00c5_c290;

/// The common tail of both removals (the node is already unlinked): holds the node's item
/// (counted), clears the node's pointer (`0066b0d0`), gives the node back (`0045cee0`),
/// counts the list down, stores the item, counted, at `result`, and drops the held reference.
fn take_item(e: &mut Engine, list: u32, node: u32, result: u32) {
    let item = e.mem.u32(node + 8);
    if item != 0 {
        e.call(INTERLOCKED_INCREMENT, &args![item.wrapping_add(4)]);
    }
    e.call(ASSIGN_POINTER, &args![node + 8, 0u32]);
    e.call(FREE_NODE, &args![list + 8, node]);
    let count = e.mem.u32(list + 8).wrapping_sub(1);
    e.mem.set_u32(list + 8, count);
    e.mem.set_u32(result, item);
    let stored = e.mem.u32(result);
    if stored != 0 {
        e.call(INTERLOCKED_INCREMENT, &args![stored.wrapping_add(4)]);
    }
    if item != 0
        && e.call(INTERLOCKED_DECREMENT, &args![item.wrapping_add(4)])
            .u32()
            == 0
    {
        e.vcall(item, 4, &args![]);
    }
}

// Translated from 00c5bfb0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c5bfb0 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f2fc71, state variable) is not translated
/// Unnamed in the engine map (`thiscall`, `RET 8`): removes the node the iterator at
/// `iterator` points to from the doubly linked list `this` (head +0, tail +4, count +8;
/// nodes: next +0, previous +4, counted item +8) and stores its item, counted, at `result`.
/// The head goes through [`fn_00c5c160`] and the tail through `00c5c290` (the iterator
/// becomes the next node, or null for the tail); any other node is unlinked here, the
/// iterator moving to the next node. Returns `result`.
pub fn fn_00c5bfb0(e: &mut Engine, this: Ptr, result: u32, iterator: u32) -> u32 {
    let list = this.addr();
    let node = e.mem.u32(iterator);
    if node == e.mem.u32(list) {
        let next = e.mem.u32(node);
        e.mem.set_u32(iterator, next);
        fn_00c5c160(e, this, result);
        return result;
    }
    if node == e.mem.u32(list + 4) {
        e.mem.set_u32(iterator, 0);
        e.call(REMOVE_TAIL, &args![this, result]);
        return result;
    }
    let previous = e.mem.u32(node + 4);
    let next = e.mem.u32(node);
    e.mem.set_u32(iterator, next);
    if previous != 0 {
        e.mem.set_u32(previous, next);
    }
    if next != 0 {
        e.mem.set_u32(next + 4, previous);
    }
    take_item(e, list, node, result);
    result
}

// Translated from 00c5c160 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c5c160 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f2fcb1, state variable) is not translated
/// Unnamed in the engine map (`thiscall`, `RET 4`): removes the head node of the list `this`
/// (the next node becomes the head with no previous node, or the tail is cleared when there
/// is none) and stores its item, counted, at `result`; the node goes back through `0045cee0`
/// and the count goes down. Returns `result`.
pub fn fn_00c5c160(e: &mut Engine, this: Ptr, result: u32) -> u32 {
    let list = this.addr();
    let node = e.mem.u32(list);
    let next = e.mem.u32(e.mem.u32(list));
    e.mem.set_u32(list, next);
    if e.mem.u32(list) != 0 {
        let head = e.mem.u32(list);
        e.mem.set_u32(head + 4, 0);
    } else {
        e.mem.set_u32(list + 4, 0);
    }
    take_item(e, list, node, result);
    result
}

// Translated from 00c5c290 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c5c290 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f2fcb1, state variable) is not translated
/// Unnamed in the engine map (`thiscall`, `RET 4`): removes the tail node of the list `this`
/// (the previous node becomes the tail with no next node, or the head is cleared when there
/// is none) and stores its item, counted, at `result`; the node goes back through `0045cee0`
/// and the count goes down. Returns `result`.
pub fn fn_00c5c290(e: &mut Engine, this: Ptr, result: u32) -> u32 {
    let list = this.addr();
    let node = e.mem.u32(list + 4);
    let previous = e.mem.u32(e.mem.u32(list + 4) + 4);
    e.mem.set_u32(list + 4, previous);
    if e.mem.u32(list + 4) != 0 {
        let tail = e.mem.u32(list + 4);
        e.mem.set_u32(tail, 0);
    } else {
        e.mem.set_u32(list, 0);
    }
    take_item(e, list, node, result);
    result
}

/// `00c1c280` (unit `nipsysemitterctlr.cpp`; `thiscall`, `RET 4`): the base constructor.
const EMITTER_CONTROLLER_CONSTRUCTOR: u32 = 0x00c1_c280;
/// The `BSPSysMultiTargetEmitterCtlr` virtual table (`010c2d54`, by RTTI).
const MULTI_TARGET_EMITTER_VTABLE: u32 = 0x010c_2d54;

// Translated from 00c5c3c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSPSysMultiTargetEmitterCtlr` constructor (by its virtual table; unnamed in the
/// engine map; `thiscall`, `RET 0xC`): the base constructor (`00c1c280`) with the first word,
/// the class's virtual table, the second word at +0x5C and the low 16 bits of the third at
/// +0x60. Answers `this`.
pub fn fn_00c5c3c0(e: &mut Engine, this: Ptr, base_argument: u32, word: u32, count: u16) -> Ptr {
    e.call(EMITTER_CONTROLLER_CONSTRUCTOR, &args![this, base_argument]);
    e.mem.set_u32(this.addr(), MULTI_TARGET_EMITTER_VTABLE);
    e.mem.set_u32(this.addr() + 0x5c, word);
    e.mem.set_u16(this.addr() + 0x60, count);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c5_bfb0, fn_00c5bfb0(Ptr, u32, u32) -> u32),
        entry!(0x00c5_c160, fn_00c5c160(Ptr, u32) -> u32),
        entry!(0x00c5_c290, fn_00c5c290(Ptr, u32) -> u32),
        entry!(0x00c5_c3c0, fn_00c5c3c0(Ptr, u32, u32, u16) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    struct List {
        list: u32,
        nodes: [u32; 3],
        items: [u32; 3],
    }

    fn fixture(e: &mut Engine) -> List {
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]) + 1;
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]) - 1;
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(ASSIGN_POINTER, |_, _| Ret::default());
        e.register(FREE_NODE, |_, _| Ret::default());
        e.register(REMOVE_TAIL, |_, _| Ret::default());
        e.put_vtable(0x0150_0000, &[0, 0x0150_1000]);
        e.register(0x0150_1000, |_, _| Ret::default());
        let list = e.mem.alloc(0xc);
        let nodes = [e.mem.alloc(0xc), e.mem.alloc(0xc), e.mem.alloc(0xc)];
        let items = [e.mem.alloc(8), e.mem.alloc(8), e.mem.alloc(8)];
        for index in 0..3 {
            e.mem.set_u32(items[index], 0x0150_0000);
            e.mem.set_u32(items[index] + 4, 1);
            e.mem.set_u32(nodes[index] + 8, items[index]);
            if index < 2 {
                e.mem.set_u32(nodes[index], nodes[index + 1]);
                e.mem.set_u32(nodes[index + 1] + 4, nodes[index]);
            }
        }
        e.mem.set_u32(list, nodes[0]);
        e.mem.set_u32(list + 4, nodes[2]);
        e.mem.set_u32(list + 8, 3);
        List { list, nodes, items }
    }

    #[test]
    fn removing_a_middle_node_relinks_its_neighbours() {
        let mut e = Engine::new();
        let l = fixture(&mut e);
        let result = e.mem.alloc(4);
        let iterator = e.mem.alloc(4);
        e.mem.set_u32(iterator, l.nodes[1]);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00c5_bfb0, &args![l.list, result, iterator]).u32(),
            result
        );
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(ASSIGN_POINTER, vec![l.nodes[1] + 8, 0])));
        assert!(log.contains(&(FREE_NODE, vec![l.list + 8, l.nodes[1]])));
        assert_eq!(e.mem.u32(iterator), l.nodes[2]);
        assert_eq!(e.mem.u32(l.nodes[0]), l.nodes[2]);
        assert_eq!(e.mem.u32(l.nodes[2] + 4), l.nodes[0]);
        assert_eq!(e.mem.u32(l.list + 8), 2);
        assert_eq!(e.mem.u32(result), l.items[1]);
        // One reference more for the result.
        assert_eq!(e.mem.u32(l.items[1] + 4), 2);
    }

    #[test]
    fn the_head_and_the_tail_go_through_their_own_removals() {
        let mut e = Engine::new();
        let l = fixture(&mut e);
        let result = e.mem.alloc(4);
        let iterator = e.mem.alloc(4);
        e.mem.set_u32(iterator, l.nodes[0]);
        e.call(0x00c5_bfb0, &args![l.list, result, iterator]);
        assert_eq!(e.mem.u32(iterator), l.nodes[1]);
        assert_eq!(e.mem.u32(l.list), l.nodes[1]);
        assert_eq!(e.mem.u32(l.nodes[1] + 4), 0);
        assert_eq!(e.mem.u32(result), l.items[0]);
        e.mem.set_u32(iterator, l.nodes[2]);
        e.call_log = Some(vec![]);
        e.call(0x00c5_bfb0, &args![l.list, result, iterator]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(REMOVE_TAIL, vec![l.list, result])]);
        assert_eq!(e.mem.u32(iterator), 0);
    }

    #[test]
    fn removing_the_last_head_clears_the_tail() {
        let mut e = Engine::new();
        let l = fixture(&mut e);
        e.mem.set_u32(l.list, l.nodes[2]);
        let result = e.mem.alloc(4);
        e.call(0x00c5_c160, &args![l.list, result]);
        assert_eq!(e.mem.u32(l.list), 0);
        assert_eq!(e.mem.u32(l.list + 4), 0);
        assert_eq!(e.mem.u32(result), l.items[2]);
        // A null item: nothing counted, null stored.
        let l = fixture(&mut e);
        e.mem.set_u32(l.nodes[0] + 8, 0);
        e.mem.set_u32(result, 5);
        e.call(0x00c5_c160, &args![l.list, result]);
        assert_eq!(e.mem.u32(result), 0);
        assert_eq!(e.mem.u32(l.list + 8), 2);
    }
}

#[cfg(test)]
mod n069_tests {
    use super::*;

    #[test]
    fn removing_the_tail_unlinks_it_and_hands_over_its_item() {
        let mut e = Engine::new();
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]) + 1;
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]) - 1;
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(ASSIGN_POINTER, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(FREE_NODE, |_, _| Ret::default());
        let item = e.mem.alloc(8);
        e.mem.set_u32(item + 4, 1);
        let list = e.mem.alloc(0xc);
        let first = e.mem.alloc(0xc);
        let last = e.mem.alloc(0xc);
        e.mem.set_u32(first, last);
        e.mem.set_u32(last + 4, first);
        e.mem.set_u32(last + 8, item);
        e.mem.set_u32(list, first);
        e.mem.set_u32(list + 4, last);
        e.mem.set_u32(list + 8, 2);
        let result = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c5_c290, &args![list, result]).u32(), result);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(FREE_NODE, vec![list + 8, last])));
        assert_eq!(e.mem.u32(list + 4), first);
        assert_eq!(e.mem.u32(first), 0);
        assert_eq!(e.mem.u32(list + 8), 1);
        assert_eq!(e.mem.u32(result), item);
        // The node's reference went to the result.
        assert_eq!(e.mem.u32(item + 4), 2);
        // The last node: the head is cleared too.
        e.mem.set_u32(first + 8, 0);
        e.call(0x00c5_c290, &args![list, result]);
        assert_eq!((e.mem.u32(list), e.mem.u32(list + 4)), (0, 0));
        assert_eq!(e.mem.u32(result), 0);
    }

    #[test]
    fn the_multi_target_emitter_constructor_stores_its_words() {
        let mut e = Engine::new();
        e.register(EMITTER_CONTROLLER_CONSTRUCTOR, |_, _| Ret::default());
        let this = e.mem.alloc(0x64);
        e.call_log = Some(vec![]);
        let result = e
            .call(0x00c5_c3c0, &args![this, 0x11u32, 0x22u32, 0x5_0033u32])
            .u32();
        let log = e.call_log.take().unwrap();
        assert_eq!(result, this);
        assert!(log.contains(&(EMITTER_CONTROLLER_CONSTRUCTOR, vec![this, 0x11])));
        assert_eq!(e.mem.u32(this), MULTI_TARGET_EMITTER_VTABLE);
        assert_eq!(e.mem.u32(this + 0x5c), 0x22);
        assert_eq!(e.mem.u32(this + 0x60), 0x33);
    }
}

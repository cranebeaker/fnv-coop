//! `fallout shared/run_004a4460` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! A doubly linked list (head `+0`, tail `+4`, count `+8`; node: next `+0`,
//! previous `+4`, value `+8`). The class is unconfirmed: the engine map has
//! no name for any of these functions.

#[allow(unused_imports)]
use crate::prelude::*;

/// Removes the head node of the list and stores its value at the address
/// given (`004a4860`, `thiscall`, one stack word).
const REMOVE_HEAD: u32 = 0x004a_4860;
/// Removes the tail node of the list and stores its value at the address
/// given (`0057cc40`, `thiscall`, one stack word).
const REMOVE_TAIL: u32 = 0x0057_cc40;
/// Copy-constructs a 4-byte value at `this` from the value whose address is
/// the stack word (`00559a40`).
const COPY_VALUE: u32 = 0x0055_9a40;
/// Frees one node of the list (`004a4930`, `thiscall` on the list, one stack
/// word: the node).
const FREE_NODE: u32 = 0x004a_4930;
/// Destroys the 4-byte value at `this` (`0045cec0`).
const DESTROY_VALUE: u32 = 0x0045_cec0;
/// Base-class destructor body run last by the list destructors (`00483710`,
/// `thiscall`; the engine map puts it in `tesform.cpp`).
const BASE_DESTRUCTOR: u32 = 0x0048_3710;
/// Destroys the contents of the other list kind (`004a4810`, `thiscall`).
const OTHER_LIST_CLEAR: u32 = 0x004a_4810;
/// Asks a state question of the object, one stack word (`00822510`; the
/// engine map puts it in `magicshaderhiteffect.cpp`, which is unconfirmed).
const STATE_QUERY: u32 = 0x0082_2510;

// Translated from 004a4460 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when the word at `+4` of `this` is zero and the query `00822510`
/// (with argument 0) answers non-zero. `__fastcall` on `this`, no stack
/// words.
pub fn fn_004a4460(e: &mut Engine, this: Ptr) -> bool {
    if e.mem.u32(this.addr() + 4) == 0 && e.call(STATE_QUERY, &args![this, 0u32]).bool() {
        return true;
    }
    false
}

// Translated from 004a44a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004a44a0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Destructor of the list: clears it (`004a4650`), then runs the base
/// destructor body (`00483710`). The exception-unwinding frame is not
/// translated.
pub fn fn_004a44a0(e: &mut Engine, this: Ptr) {
    fn_004a4650(e, this);
    e.call(BASE_DESTRUCTOR, &args![this]);
}

// Translated from 004a4500 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004a4500 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Removes the node that the node-pointer variable at `cursor` points at
/// from the list `this` and stores the removed node's value at `result`
/// (returned). The variable ends up on the following node (null when the
/// tail was removed). The head and the tail go through `004a4860` and
/// `0057cc40`; any other node is unlinked, its value is copied through a
/// 4-byte temporary (`00559a40`), the node freed (`004a4930`), the count
/// dropped, the temporary copied to `result` and destroyed (`0045cec0`).
/// `__thiscall`, two stack words (`RET 8`). The exception-unwinding frame
/// is not translated.
pub fn fn_004a4500(e: &mut Engine, this: Ptr, result: Ptr, cursor: Ptr) -> Ptr {
    let list = this.addr();
    let node = e.mem.u32(cursor.addr());
    if node == e.mem.u32(list) {
        let next = e.mem.u32(node);
        e.mem.set_u32(cursor.addr(), next);
        e.call(REMOVE_HEAD, &args![this, result]);
        return result;
    }
    if node == e.mem.u32(list + 4) {
        e.mem.set_u32(cursor.addr(), 0);
        e.call(REMOVE_TAIL, &args![this, result]);
        return result;
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
    e.with_stack(4, |e, copy| {
        e.call(COPY_VALUE, &args![copy, node + 8]);
        e.call(FREE_NODE, &args![this, node]);
        let count = e.mem.u32(list + 8);
        e.mem.set_u32(list + 8, count.wrapping_sub(1));
        e.call(COPY_VALUE, &args![result, copy]);
        e.call(DESTROY_VALUE, &args![copy]);
    });
    result
}

// Translated from 004a4650 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empties the list `this`: frees every node from the head on (`004a4930`),
/// then sets the count, the head and the tail to zero. `__fastcall`.
pub fn fn_004a4650(e: &mut Engine, this: Ptr) {
    let list = this.addr();
    let mut node = e.mem.u32(list);
    while node != 0 {
        let current = node;
        node = e.mem.u32(node);
        e.call(FREE_NODE, &args![this, current]);
    }
    e.mem.set_u32(list + 8, 0);
    e.mem.set_u32(list, 0);
    e.mem.set_u32(list + 4, 0);
}

// Translated from 004a46b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Address of element `index` of the array whose base address is the word at
/// `+0` of `this`, with 0x60-byte elements. `__thiscall`, one stack word.
pub fn fn_004a46b0(e: &mut Engine, this: Ptr, index: i32) -> u32 {
    let base = e.mem.u32(this.addr());
    index.wrapping_mul(0x60).wrapping_add(base as i32) as u32
}

// Translated from 004a46d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004a46d0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Destructor of the other list kind: runs its contents' destructor
/// (`004a4810`), then the base destructor body (`00483710`). The
/// exception-unwinding frame is not translated.
pub fn fn_004a46d0(e: &mut Engine, this: Ptr) {
    e.call(OTHER_LIST_CLEAR, &args![this]);
    e.call(BASE_DESTRUCTOR, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004a4460, fn_004a4460(Ptr) -> bool),
        entry!(0x004a44a0, fn_004a44a0(Ptr)),
        entry!(0x004a4500, fn_004a4500(Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x004a4650, fn_004a4650(Ptr)),
        entry!(0x004a46b0, fn_004a46b0(Ptr, i32) -> u32),
        entry!(0x004a46d0, fn_004a46d0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// Registers recording doubles for every callee; returns the shared log.
    fn doubles(e: &mut Engine) -> Log {
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        for addr in [
            REMOVE_HEAD,
            REMOVE_TAIL,
            COPY_VALUE,
            FREE_NODE,
            DESTROY_VALUE,
            BASE_DESTRUCTOR,
            OTHER_LIST_CLEAR,
        ] {
            let log = log.clone();
            e.register_double(addr, move |_, a| {
                log.borrow_mut().push((addr, a.to_vec()));
                Ret::default()
            });
        }
        log
    }

    fn chain(e: &mut Engine) -> (Ptr, [u32; 3]) {
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
        (list, nodes)
    }

    #[test]
    fn the_query_needs_a_zero_word_and_a_yes() {
        let mut e = Engine::new();
        e.register(STATE_QUERY, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        let object: Ptr = Ptr::new(e.mem.alloc(8));
        assert!(e.call(0x004a_4460, &args![object]).bool());
        e.mem.set_u32(object.addr() + 4, 5);
        assert!(!e.call(0x004a_4460, &args![object]).bool());
        e.mem.set_u32(object.addr() + 4, 0);
        e.register(STATE_QUERY, |_, a| {
            assert_eq!(a[1], 0);
            Ret::default()
        });
        assert!(!e.call(0x004a_4460, &args![object]).bool());
    }

    #[test]
    fn clearing_frees_every_node_and_zeroes_the_list() {
        let mut e = Engine::new();
        let log = doubles(&mut e);
        let (list, nodes) = chain(&mut e);
        e.call(0x004a_4650, &args![list]);
        let freed: Vec<u32> = log.borrow().iter().map(|(_, a)| a[1]).collect();
        assert_eq!(freed, nodes);
        for offset in [0, 4, 8] {
            assert_eq!(e.mem.u32(list.addr() + offset), 0);
        }
    }

    #[test]
    fn the_destructors_clear_then_run_the_base() {
        let mut e = Engine::new();
        let log = doubles(&mut e);
        let (list, _) = chain(&mut e);
        e.call(0x004a_44a0, &args![list]);
        assert_eq!(log.borrow().last().unwrap().0, BASE_DESTRUCTOR);
        assert_eq!(log.borrow().iter().filter(|c| c.0 == FREE_NODE).count(), 3);
        log.borrow_mut().clear();
        e.call(0x004a_46d0, &args![list]);
        let calls: Vec<u32> = log.borrow().iter().map(|c| c.0).collect();
        assert_eq!(calls, vec![OTHER_LIST_CLEAR, BASE_DESTRUCTOR]);
    }

    #[test]
    fn element_address_uses_0x60_byte_elements() {
        let mut e = Engine::new();
        let array: Ptr = Ptr::new(e.mem.alloc(4));
        e.mem.set_u32(array.addr(), 0x1000);
        assert_eq!(e.call(0x004a_46b0, &args![array, 3i32]).u32(), 0x1120);
        assert_eq!(e.call(0x004a_46b0, &args![array, -1i32]).u32(), 0xfa0);
    }

    #[test]
    fn removing_the_head_or_tail_delegates() {
        let mut e = Engine::new();
        let log = doubles(&mut e);
        let (list, nodes) = chain(&mut e);
        let result: Ptr = Ptr::new(e.mem.alloc(4));
        let cursor: Ptr = Ptr::new(e.mem.alloc(4));
        e.mem.set_u32(cursor.addr(), nodes[0]);
        let r = e.call(0x004a_4500, &args![list, result, cursor]);
        assert_eq!(r.u32(), result.addr());
        assert_eq!(e.mem.u32(cursor.addr()), nodes[1]);
        assert_eq!(log.borrow().last().unwrap().0, REMOVE_HEAD);
        e.mem.set_u32(cursor.addr(), nodes[2]);
        e.call(0x004a_4500, &args![list, result, cursor]);
        assert_eq!(e.mem.u32(cursor.addr()), 0);
        assert_eq!(log.borrow().last().unwrap().0, REMOVE_TAIL);
    }

    #[test]
    fn removing_a_middle_node_copies_its_value_out() {
        let mut e = Engine::new();
        let log = doubles(&mut e);
        let (list, nodes) = chain(&mut e);
        let result: Ptr = Ptr::new(e.mem.alloc(4));
        let cursor: Ptr = Ptr::new(e.mem.alloc(4));
        e.mem.set_u32(cursor.addr(), nodes[1]);
        e.call(0x004a_4500, &args![list, result, cursor]);
        assert_eq!(e.mem.u32(cursor.addr()), nodes[2]);
        assert_eq!(e.mem.u32(nodes[0]), nodes[2]);
        assert_eq!(e.mem.u32(nodes[2] + 4), nodes[0]);
        assert_eq!(e.mem.u32(list.addr() + 8), 2);
        let calls = log.borrow();
        let order: Vec<u32> = calls.iter().map(|c| c.0).collect();
        assert_eq!(
            order,
            vec![COPY_VALUE, FREE_NODE, COPY_VALUE, DESTROY_VALUE]
        );
        assert_eq!(calls[0].1[1], nodes[1] + 8);
        assert_eq!(calls[2].1[0], result.addr());
        assert_eq!(calls[2].1[1], calls[0].1[0]);
    }
}

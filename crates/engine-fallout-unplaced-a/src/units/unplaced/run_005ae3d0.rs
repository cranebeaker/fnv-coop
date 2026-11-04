//! `(unplaced)/run_005ae3d0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `operator new` (`00401000`).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// The chain node constructor (`00470440`, `__thiscall`, `RET 4`): copies the
/// head value of the container it is given into the new node's `+0` and
/// zeroes the node's `+4`; returns the node.
const NODE_CONSTRUCT: u32 = 0x0047_0440;

// Translated from 005ae3d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Takes over the head value of another chained container (head value at
/// `+0`, chain of 8-byte nodes starting at `+4`, see `00470040`). Nothing
/// happens when `other` has no head value (the word at its `+0` is 0). If
/// `this` has none either, it simply gets `other`'s. Otherwise `this`'s
/// current head value is first pushed onto the front of its chain in a new
/// 8-byte node, then `this` gets `other`'s head value. `__thiscall`, one
/// stack word (`RET 4`).
///
/// The compiler's exception-unwinding frame is not translated. The game
/// writes through the node even when the allocation or construction yields
/// null (a fault at address 4); so does this.
pub fn fn_005ae3d0(e: &mut Engine, this: Ptr, other: Ptr) {
    let incoming = e.mem.u32(other.addr());
    if incoming == 0 {
        return;
    }
    if e.mem.u32(this.addr()) != 0 {
        let block = e.call(OPERATOR_NEW, &args![8u32]).u32();
        let node = if block == 0 {
            0
        } else {
            e.call(NODE_CONSTRUCT, &args![block, this]).u32()
        };
        let first = e.mem.u32(this.addr() + 4);
        e.mem.set_u32(node + 4, first);
        e.mem.set_u32(this.addr() + 4, node);
    }
    e.mem.set_u32(this.addr(), incoming);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x005ae3d0, fn_005ae3d0(Ptr, Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn containers(e: &mut Engine, this_head: u32, other_head: u32) -> (Ptr, Ptr) {
        let this: Ptr = Ptr::new(e.mem.alloc(8));
        let other: Ptr = Ptr::new(e.mem.alloc(8));
        e.mem.set_u32(this.addr(), this_head);
        e.mem.set_u32(other.addr(), other_head);
        (this, other)
    }

    /// Doubles for the allocation (a fresh 8-byte block) and the node
    /// constructor (copies the container's head value, zeroes the link).
    fn allocation_doubles(e: &mut Engine) {
        e.register(OPERATOR_NEW, |e, _| Ret {
            eax: e.mem.alloc(8),
            ..Ret::default()
        });
        e.register(NODE_CONSTRUCT, |e, a| {
            let value = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], value);
            e.mem.set_u32(a[0] + 4, 0);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
    }

    #[test]
    fn nothing_to_take_over_changes_nothing() {
        let mut e = Engine::new();
        let (this, other) = containers(&mut e, 5, 0);
        e.call_log = Some(vec![]);
        e.call(0x005a_e3d0, &args![this, other]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        assert_eq!(e.mem.u32(this.addr()), 5);
    }

    #[test]
    fn an_empty_target_just_gets_the_head_value() {
        let mut e = Engine::new();
        let (this, other) = containers(&mut e, 0, 7);
        e.call_log = Some(vec![]);
        e.call(0x005a_e3d0, &args![this, other]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        assert_eq!(e.mem.u32(this.addr()), 7);
        assert_eq!(e.mem.u32(this.addr() + 4), 0);
    }

    #[test]
    fn the_old_head_value_is_pushed_onto_the_front_of_the_chain() {
        let mut e = Engine::new();
        allocation_doubles(&mut e);
        let (this, other) = containers(&mut e, 5, 7);
        let old_first = e.mem.alloc(8);
        e.mem.set_u32(this.addr() + 4, old_first);
        e.call_log = Some(vec![]);
        e.call(0x005a_e3d0, &args![this, other]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (OPERATOR_NEW, vec![8]));
        assert_eq!(log[2].0, NODE_CONSTRUCT);
        assert_eq!(log[2].1[1], this.addr());
        assert_eq!(e.mem.u32(this.addr()), 7);
        let node = e.mem.u32(this.addr() + 4);
        assert_ne!(node, old_first);
        // The new node holds the old head value and links to the old chain.
        assert_eq!(e.mem.u32(node), 5);
        assert_eq!(e.mem.u32(node + 4), old_first);
    }
}

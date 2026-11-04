//! `fallout shared/run_0049f8b0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `0043a010` (`thiscall`, no stack word; engine map:
/// `NiTPointerListBase<NiTPointerAllocator<unsigned int>,char *>::AddHead`, a body shared by
/// folding): answers a new node.
const ALLOCATE_NODE: u32 = 0x0043_a010;
/// `006e6da0` (`cdecl`, `(size, place)`: the placement `new`, answers `place`) and
/// `006815c0` (`thiscall`, no stack word): the constructor run on each 12-byte element.
const PLACEMENT_NEW: u32 = 0x006e_6da0;
const ELEMENT_CONSTRUCTOR: u32 = 0x0068_15c0;

// Translated from 0049fa80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only): the node `0043a010` makes for the list part at
/// `this + 8` (its answer passes through in `EAX`).
pub fn fn_0049fa80(e: &mut Engine, this: Ptr) -> u32 {
    e.call(ALLOCATE_NODE, &args![this.addr().wrapping_add(8)])
        .u32()
}

// Translated from 0049fa20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): puts `node` at the head of the list at
/// `this` (head `+0`, tail `+4`, count `+8`; a node's next at `+0`, previous at `+4`): no
/// previous node, the old head as next (its previous link set to `node`, or the tail set
/// when the list was empty), and the count up by one.
pub fn fn_0049fa20(e: &mut Engine, this: Ptr, node: Ptr) {
    let (list, new) = (this.addr(), node.addr());
    e.mem.set_u32(new + 4, 0);
    let head = e.mem.u32(list);
    e.mem.set_u32(new, head);
    if e.mem.u32(list) == 0 {
        e.mem.set_u32(list + 4, new);
    } else {
        let head = e.mem.u32(list);
        e.mem.set_u32(head + 4, new);
    }
    e.mem.set_u32(list, new);
    let count = e.mem.u32(list + 8).wrapping_add(1);
    e.mem.set_u32(list + 8, count);
}

// Translated from 0049f990 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`): sets up the array at `this` (buffer
/// `+4`, size `+8`, capacity `+0xc`) for `size` elements with room for `capacity` (at least
/// `size`): the buffer comes from the object's own slot `+4` with the capacity (none for a
/// capacity of 0), and `size` elements are constructed in it (`0049f8f0`).
pub fn fn_0049f990(e: &mut Engine, this: Ptr, capacity: u32, size: u32) {
    let array = this.addr();
    e.mem.set_u32(array + 4, 0);
    e.mem.set_u32(array + 8, 0);
    e.mem.set_u32(array + 0xc, 0);
    let capacity = if size > capacity { size } else { capacity };
    if capacity != 0 {
        let buffer = e.vcall(array, 4, &args![capacity]).u32();
        e.mem.set_u32(array + 4, buffer);
        e.mem.set_u32(array + 0xc, capacity);
    }
    if size != 0 {
        let buffer = e.mem.u32(array + 4);
        fn_0049f8f0(e, this, buffer, size);
        e.mem.set_u32(array + 8, size);
    }
}

// Translated from 0049f8f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`, `this` not read): constructs `count`
/// 12-byte elements at `elements`: each slot through the placement `new` (`006e6da0`) and,
/// when that answers non-null, the element constructor (`006815c0`).
// Translation incomplete: 0049f8f0 - the C++ exception frame (FS:[0] chain, unwind state 0) and the /GS cookie word are not translated.
pub fn fn_0049f8f0(e: &mut Engine, _this: Ptr, elements: u32, count: u32) {
    for index in 0..count {
        let place = index.wrapping_mul(0xc).wrapping_add(elements);
        let element = e.call(PLACEMENT_NEW, &args![0xcu32, place]).u32();
        if element != 0 {
            e.call(ELEMENT_CONSTRUCTOR, &args![element]);
        }
    }
}

// Translated from 0049f8b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the words at +4, +8 and +0xc of `other` into `this` (`thiscall`,
/// `RET 4`; the engine map gives it no name). Returns `this`.
pub fn fn_0049f8b0(e: &mut Engine, this: Ptr, other: u32) -> Ptr {
    for offset in [4u32, 8, 0xc] {
        let value = e.mem.u32(other.wrapping_add(offset));
        e.mem.set_u32(this.addr() + offset, value);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0049_f8b0, fn_0049f8b0(Ptr, u32) -> Ptr),
        entry!(0x0049_fa80, fn_0049fa80(Ptr) -> u32),
        entry!(0x0049_fa20, fn_0049fa20(Ptr, Ptr)),
        entry!(0x0049_f990, fn_0049f990(Ptr, u32, u32)),
        entry!(0x0049_f8f0, fn_0049f8f0(Ptr, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn a_node_for_the_list_part() {
        let mut e = Engine::new();
        e.register(ALLOCATE_NODE, |_, a| (a[0] + 0x100).into_ret());
        assert_eq!(e.call(0x0049_fa80, &args![0x5000u32]).u32(), 0x5108);
    }

    #[test]
    fn adding_at_the_head() {
        let mut e = Engine::new();
        let list = e.mem.alloc(0xc);
        let (first, second) = (e.mem.alloc(0xc), e.mem.alloc(0xc));
        e.mem.set_u32(first + 4, 0xdead);
        e.call(0x0049_fa20, &args![list, first]);
        assert_eq!(
            (e.mem.u32(list), e.mem.u32(list + 4), e.mem.u32(list + 8)),
            (first, first, 1)
        );
        assert_eq!((e.mem.u32(first), e.mem.u32(first + 4)), (0, 0));
        e.call(0x0049_fa20, &args![list, second]);
        assert_eq!(
            (e.mem.u32(list), e.mem.u32(list + 4), e.mem.u32(list + 8)),
            (second, first, 2)
        );
        assert_eq!((e.mem.u32(second), e.mem.u32(first + 4)), (first, second));
    }

    #[test]
    fn setting_up_an_array_allocates_and_constructs() {
        let mut e = Engine::new();
        const VTABLE: u32 = 0x0300_0000;
        const ALLOCATE: u32 = 0x0300_1000;
        e.put_vtable(VTABLE, &[0, ALLOCATE]);
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[1] * 0xc).into_ret());
        e.register(PLACEMENT_NEW, |_, a| a[1].into_ret());
        e.register(ELEMENT_CONSTRUCTOR, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this, VTABLE);
        // Capacity 1 below the size 2: room for 2.
        e.call(0x0049_f990, &args![this, 1u32, 2u32]);
        let buffer = e.mem.u32(this + 4);
        assert_eq!(calls(&e, ALLOCATE), vec![vec![this, 2]]);
        assert_eq!((e.mem.u32(this + 8), e.mem.u32(this + 0xc)), (2, 2));
        assert_eq!(
            calls(&e, ELEMENT_CONSTRUCTOR),
            vec![vec![buffer], vec![buffer + 0xc]]
        );
        // Nothing asked: nothing made.
        e.call(0x0049_f990, &args![this, 0u32, 0u32]);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(calls(&e, ALLOCATE).len(), 1);
    }

    #[test]
    fn construction_skips_a_null_placement() {
        let mut e = Engine::new();
        e.register(PLACEMENT_NEW, |_, a| {
            if a[1] == 0x600c { 0u32 } else { a[1] }.into_ret()
        });
        e.register(ELEMENT_CONSTRUCTOR, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0049_f8f0, &args![0u32, 0x6000u32, 3u32]);
        assert_eq!(
            calls(&e, ELEMENT_CONSTRUCTOR),
            vec![vec![0x6000], vec![0x6018]]
        );
        assert_eq!(calls(&e, PLACEMENT_NEW).len(), 3);
    }
}

#[cfg(test)]
mod n038_tests {
    use super::*;

    #[test]
    fn at_0049f8b0_copies_three_words() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        let other = e.mem.alloc(0x10);
        for i in 0..4 {
            e.mem.set_u32(other + 4 * i, 0x10 + i);
        }
        e.mem.set_u32(this, 0x99);
        assert_eq!(e.call(0x0049_f8b0, &args![this, other]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x99);
        assert_eq!(e.mem.u32(this + 4), 0x11);
        assert_eq!(e.mem.u32(this + 8), 0x12);
        assert_eq!(e.mem.u32(this + 0xc), 0x13);
    }
}

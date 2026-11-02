//! `Havok SDK/run_00e05c20` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use super::hkaabbtree::{allocator, release_array};

// Translated from 00e05c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: releases a node and its children (`this` in `ECX`). The node
/// holds two `hkArray`-shaped triples (data, size, capacity and flags) of pointers: the first at
/// `+0` and the second at `+0xc`. When the first array owns its storage (capacity word not
/// negative) every non-null pointer in it is returned to the allocator (slot `8`, `0x200`
/// bytes). When the second one does, every non-null pointer in it is released recursively (this
/// function on it) and then returned (`0x20` bytes). Then both arrays release their storage
/// (4-byte elements), the second one first, as `hkArray::clearAndDeallocate`. The class is
/// unconfirmed.
pub fn fn_00e05c20(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    if (e.mem.u32(base + 8) as i32) >= 0 {
        let mut index = 0u32;
        while (index as i32) < (e.mem.u32(base + 4) as i32) {
            let element = e.mem.u32(e.mem.u32(base) + index * 4);
            if element != 0 {
                let heap = allocator(e);
                e.vcall(heap, 8, &args![element, 0x200u32]);
            }
            index += 1;
        }
    }
    if (e.mem.u32(base + 0x14) as i32) >= 0 {
        let mut index = 0u32;
        while (index as i32) < (e.mem.u32(base + 0x10) as i32) {
            let element = e.mem.u32(e.mem.u32(base + 0xc) + index * 4);
            if element != 0 {
                fn_00e05c20(e, Ptr::new(element));
                let heap = allocator(e);
                e.vcall(heap, 8, &args![element, 0x20u32]);
            }
            index += 1;
        }
    }
    release_array(e, base + 0xc, 4);
    release_array(e, base, 4);
}

// Translated from 00e05d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: a destructor (`this` in `ECX`). It writes the vtable word
/// `010d77d8`, releases the referenced objects at `+8` and `+0xc` when they are not null
/// (`00c905b0`, `this` = the object), releases the tree at `this + 0x114` (`00e05c20`) and
/// empties three `hkArray`-shaped triples: at `+0x68` (1-byte elements), `+0x5c` (32-byte
/// elements) and `+0x50` (1-byte elements); the last step is the vtable word `0102e388`. The
/// class is unconfirmed.
pub fn fn_00e05d10(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, 0x010d_77d8);
    let first = e.mem.u32(base + 8);
    if first != 0 {
        e.call(0x00c9_05b0, &args![first]);
    }
    let second = e.mem.u32(base + 0xc);
    if second != 0 {
        e.call(0x00c9_05b0, &args![second]);
    }
    fn_00e05c20(e, Ptr::new(base + 0x114));
    release_array(e, base + 0x68, 1);
    release_array(e, base + 0x5c, 32);
    release_array(e, base + 0x50, 1);
    e.mem.set_u32(base, 0x0102_e388);
}

// Translated from 00e05de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: reverses the bytes of each of the four words (16 bytes) at
/// `words` (an endianness swap). The function takes one stack word and pops none (`RET`) and
/// returns nothing.
pub fn fn_00e05de0(e: &mut Engine, words: Ptr) {
    for word in 0..4 {
        let at = words.addr().wrapping_add(4 * word);
        let value = e.mem.u32(at);
        e.mem.set_u32(at, value.swap_bytes());
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e0_5c20, fn_00e05c20(Ptr)),
        entry!(0x00e0_5d10, fn_00e05d10(Ptr)),
        entry!(0x00e0_5de0, fn_00e05de0(Ptr)),
    ]
}

#[cfg(test)]
mod tests_a05 {
    use crate::prelude::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FREE_SLOT_DOUBLE: u32 = 0x0900_0011;
    const BLOCK_SLOT_DOUBLE: u32 = 0x0900_0021;

    type Log = Rc<RefCell<Vec<Vec<u32>>>>;

    /// An allocator whose slot 8 (block free) logs `[8, block, size]` and whose slot 0x10 (array
    /// storage free) logs `[0x10, data, bytes]`.
    fn engine() -> (Engine, Log) {
        let mut e = Engine::new();
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0x10, FREE_SLOT_DOUBLE);
        e.mem.set_u32(vtable + 8, BLOCK_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(0x00c8_5750, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        for (tag, slot) in [(0x10u32, FREE_SLOT_DOUBLE), (8, BLOCK_SLOT_DOUBLE)] {
            let slot_log = Rc::clone(&log);
            e.register_double(slot, move |_, a| {
                slot_log.borrow_mut().push(vec![tag, a[1], a[2]]);
                Ret::default()
            });
        }
        (e, log)
    }

    fn set_array(e: &mut Engine, array: u32, data: u32, size: u32, capacity: u32) {
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 4, size);
        e.mem.set_u32(array + 8, capacity);
    }

    /// A node with two owned elements in the first array (one null), and one child in the
    /// second array that has an element of its own: block frees in order, then both arrays'
    /// storage, the second first.
    #[test]
    fn fn_00e05c20_frees_the_elements_and_children_then_the_arrays() {
        let (mut e, log) = engine();
        let node = e.mem.alloc(0x20);
        let child = e.mem.alloc(0x20);
        // The child: first array holds one element, second array not owned and empty.
        let child_data = e.mem.alloc(0x10);
        e.mem.set_u32(child_data, 0xc1);
        set_array(&mut e, child, child_data, 1, 1);
        set_array(&mut e, child + 0xc, 0, 0, 0x8000_0000);
        let first_data = e.mem.alloc(0x10);
        e.mem.set_u32(first_data, 0xa1);
        e.mem.set_u32(first_data + 4, 0);
        e.mem.set_u32(first_data + 8, 0xa3);
        set_array(&mut e, node, first_data, 3, 4);
        let second_data = e.mem.alloc(0x10);
        e.mem.set_u32(second_data, child);
        set_array(&mut e, node + 0xc, second_data, 1, 2);
        e.call(0x00e0_5c20, &args![node]);
        assert_eq!(
            *log.borrow(),
            vec![
                vec![8, 0xa1, 0x200],
                vec![8, 0xa3, 0x200],
                // The child: its element, then its first array's storage (4 bytes * 1).
                vec![8, 0xc1, 0x200],
                vec![0x10, child_data, 4],
                vec![8, child, 0x20],
                vec![0x10, second_data, 8],
                vec![0x10, first_data, 16],
            ]
        );
        assert_eq!(e.mem.u32(node), 0);
        assert_eq!(e.mem.u32(node + 4), 0);
        assert_eq!(e.mem.u32(node + 8), 0x8000_0000);
        assert_eq!(e.mem.u32(node + 0xc), 0);
        assert_eq!(e.mem.u32(node + 0x10), 0);
        assert_eq!(e.mem.u32(node + 0x14), 0x8000_0000);
    }

    /// Arrays that do not own their storage keep their elements and free nothing.
    #[test]
    fn fn_00e05c20_leaves_unowned_arrays_alone() {
        let (mut e, log) = engine();
        let node = e.mem.alloc(0x20);
        let data = e.mem.alloc(0x10);
        e.mem.set_u32(data, 0xa1);
        set_array(&mut e, node, data, 1, 0x8000_0001);
        set_array(&mut e, node + 0xc, data, 1, 0x8000_0001);
        e.call(0x00e0_5c20, &args![node]);
        assert!(log.borrow().is_empty());
        assert_eq!(e.mem.u32(node), 0);
        assert_eq!(e.mem.u32(node + 8), 0x8000_0000);
        assert_eq!(e.mem.u32(node + 0x14), 0x8000_0000);
    }

    /// The destructor releases the two references, the tree and the three arrays, then writes
    /// the final vtable word; null references are skipped.
    #[test]
    fn fn_00e05d10_releases_references_tree_and_arrays() {
        let (mut e, log) = engine();
        let release_log = Rc::clone(&log);
        e.register_double(0x00c9_05b0, move |_, a| {
            release_log.borrow_mut().push(vec![0x5b0, a[0]]);
            Ret::default()
        });
        let this = e.mem.alloc(0x130);
        e.mem.set_u32(this + 8, 0xb1);
        e.mem.set_u32(this + 0xc, 0);
        // The tree at +0x114 is empty and not owned.
        e.mem.set_u32(this + 0x114 + 8, 0x8000_0000);
        e.mem.set_u32(this + 0x114 + 0x14, 0x8000_0000);
        set_array(&mut e, this + 0x68, 0xd1, 2, 3);
        set_array(&mut e, this + 0x5c, 0xd2, 2, 5);
        set_array(&mut e, this + 0x50, 0xd3, 2, 0x8000_0002);
        e.call(0x00e0_5d10, &args![this]);
        assert_eq!(
            *log.borrow(),
            vec![
                vec![0x5b0, 0xb1],
                vec![0x10, 0xd1, 3],
                vec![0x10, 0xd2, 160]
            ]
        );
        assert_eq!(e.mem.u32(this), 0x0102_e388);
        for (data, size, capacity) in [(0x68, 0x6c, 0x70), (0x5c, 0x60, 0x64), (0x50, 0x54, 0x58)] {
            assert_eq!(e.mem.u32(this + data), 0);
            assert_eq!(e.mem.u32(this + size), 0);
            assert_eq!(e.mem.u32(this + capacity), 0x8000_0000);
        }
    }

    /// Four words are byte-reversed; the fifth is untouched.
    #[test]
    fn fn_00e05de0_swaps_four_words() {
        let mut e = Engine::new();
        let words = e.mem.alloc(0x20);
        for (i, value) in [0x0102_0304u32, 0xa1b2_c3d4, 0, 0xffff_0001, 0x1122_3344]
            .into_iter()
            .enumerate()
        {
            e.mem.set_u32(words + 4 * i as u32, value);
        }
        e.call(0x00e0_5de0, &args![words]);
        let got: Vec<u32> = (0..5).map(|i| e.mem.u32(words + 4 * i)).collect();
        assert_eq!(
            got,
            vec![0x0403_0201, 0xd4c3_b2a1, 0, 0x0100_ffff, 0x1122_3344]
        );
    }
}

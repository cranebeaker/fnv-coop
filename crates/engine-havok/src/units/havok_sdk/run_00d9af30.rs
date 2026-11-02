//! `Havok SDK/run_00d9af30` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::havok_sdk::run_00d6bf60::{load_vec, maxps, minps, store_vec, Vec4};

/// The wide node accessor (`00d9a4c0`, `thiscall(manager, index)`): the address of node `index`
/// (0x30-byte nodes) in the manager at `tree + 0x1c`.
const NODE_AT: u32 = 0x00d9_a4c0;

// Translated from 00d9af30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The root fix-up of the wide tree (`hkaabbtree.rs` calls it as `FIX_UP_WIDE`): thiscall, three
/// stack words `(node, tag, last)`, `RET 0xc`. Stores `last` at `node + 0x2c`; an inner node (bit
/// 31 of `[node + 0x28]` clear) then looks its two children up through `00d9a4c0` on the manager
/// at `this + 0x1c` (their indices at `+0x20` and `+0x24`), puts `tag` into their `+0x28` words
/// (keeping bit 31), recurses into the first with `(first index, second index)` and into the
/// second with `(second index, last)`, and stores the lane-wise minimum of their boxes' first
/// vectors at `node` and the maximum of their second vectors at `node + 0x10`. The class is the
/// `hkAabbTree` of `hkaabbtree.rs`.
pub fn fn_00d9af30(e: &mut Engine, this: Ptr, node: u32, tag: u32, last: u32) {
    let this = this.addr();
    let flags = e.mem.u32(node + 0x28);
    e.mem.set_u32(node + 0x2c, last);
    if flags & 0x8000_0000 != 0 {
        return;
    }
    let manager = this.wrapping_add(0x1c);
    let first_index = e.mem.u32(node + 0x20);
    let first = e.call(NODE_AT, &args![manager, first_index]).u32();
    let second_index = e.mem.u32(node + 0x24);
    let second = e.call(NODE_AT, &args![manager, second_index]).u32();
    let word = (e.mem.u32(first + 0x28) & 0x8000_0000) | tag;
    e.mem.set_u32(first + 0x28, word);
    let word = (e.mem.u32(second + 0x28) & 0x8000_0000) | tag;
    e.mem.set_u32(second + 0x28, word);
    let first_index = e.mem.u32(node + 0x20);
    let second_index = e.mem.u32(node + 0x24);
    fn_00d9af30(e, Ptr::new(this), first, first_index, second_index);
    let second_index = e.mem.u32(node + 0x24);
    fn_00d9af30(e, Ptr::new(this), second, second_index, last);
    let second_min: Vec4 = load_vec(e, second);
    let first_min: Vec4 = load_vec(e, first);
    store_vec(e, node, minps(first_min, second_min));
    let second_max: Vec4 = load_vec(e, second + 0x10);
    let first_max: Vec4 = load_vec(e, first + 0x10);
    store_vec(e, node + 0x10, maxps(first_max, second_max));
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00d9_af30, fn_00d9af30(Ptr, u32, u32, u32))]
}

#[cfg(test)]
mod a09_tests {
    use super::*;

    /// A manager whose nodes (0x30 bytes) hold: min vector, max vector, child indices and the
    /// flag word.
    struct Tree {
        this: u32,
        nodes: u32,
    }

    fn tree(e: &mut Engine) -> Tree {
        // The real accessor `00d9a4c0` is not registered here: stand in for it.
        e.register(NODE_AT, |e, a| Ret {
            eax: e.mem.u32(a[0]) + 0x30 * a[1],
            ..Ret::default()
        });
        let this = e.mem.alloc(0x40);
        let nodes = e.mem.alloc(0x30 * 8);
        e.mem.set_u32(this + 0x1c, nodes);
        Tree { this, nodes }
    }

    fn node(tree: &Tree, index: u32) -> u32 {
        tree.nodes + 0x30 * index
    }

    fn set_box(e: &mut Engine, node: u32, min: Vec4, max: Vec4) {
        store_vec(e, node, min);
        store_vec(e, node + 0x10, max);
    }

    #[test]
    fn an_inner_node_takes_the_union_of_its_children_and_tags_them() {
        let mut e = Engine::new();
        let tree = tree(&mut e);
        let root = node(&tree, 0);
        let (left, right) = (node(&tree, 1), node(&tree, 2));
        e.mem.set_u32(root + 0x20, 1);
        e.mem.set_u32(root + 0x24, 2);
        e.mem.set_u32(left + 0x28, 0x8000_0000);
        e.mem.set_u32(right + 0x28, 0x8000_0000);
        set_box(&mut e, left, [1.0, 5.0, 3.0, 0.0], [4.0, 6.0, 9.0, 1.0]);
        set_box(&mut e, right, [2.0, 4.0, 2.0, 0.5], [3.0, 8.0, 10.0, 0.0]);
        e.call(
            0x00d9_af30,
            &args![tree.this, root, 0x77u32, 0x7fff_ffffu32],
        );
        assert_eq!(load_vec(&e, root), [1.0, 4.0, 2.0, 0.0]);
        assert_eq!(load_vec(&e, root + 0x10), [4.0, 8.0, 10.0, 1.0]);
        // The children keep their leaf bit and take the tag; their last words are the indices.
        assert_eq!(e.mem.u32(left + 0x28), 0x8000_0077);
        assert_eq!(e.mem.u32(right + 0x28), 0x8000_0077);
        assert_eq!(e.mem.u32(root + 0x2c), 0x7fff_ffff);
        assert_eq!(e.mem.u32(left + 0x2c), 2);
        assert_eq!(e.mem.u32(right + 0x2c), 0x7fff_ffff);
    }

    #[test]
    fn a_leaf_only_records_its_last_word() {
        let mut e = Engine::new();
        let tree = tree(&mut e);
        let leaf = node(&tree, 3);
        e.mem.set_u32(leaf + 0x28, 0x8000_0001);
        set_box(&mut e, leaf, [1.0; 4], [2.0; 4]);
        e.call(0x00d9_af30, &args![tree.this, leaf, 9u32, 0x55u32]);
        assert_eq!(e.mem.u32(leaf + 0x2c), 0x55);
        assert_eq!(load_vec(&e, leaf), [1.0; 4]);
        assert_eq!(e.mem.u32(leaf + 0x28), 0x8000_0001);
    }

    #[test]
    fn a_deeper_tree_is_fixed_up_from_the_leaves() {
        let mut e = Engine::new();
        let tree = tree(&mut e);
        let (root, mid, a, b, c) = (
            node(&tree, 0),
            node(&tree, 1),
            node(&tree, 2),
            node(&tree, 3),
            node(&tree, 4),
        );
        e.mem.set_u32(root + 0x20, 1);
        e.mem.set_u32(root + 0x24, 4);
        e.mem.set_u32(mid + 0x20, 2);
        e.mem.set_u32(mid + 0x24, 3);
        for leaf in [a, b, c] {
            e.mem.set_u32(leaf + 0x28, 0x8000_0000);
        }
        set_box(&mut e, a, [0.0; 4], [1.0; 4]);
        set_box(&mut e, b, [5.0; 4], [6.0; 4]);
        set_box(&mut e, c, [-3.0; 4], [2.0; 4]);
        e.call(0x00d9_af30, &args![tree.this, root, 1u32, 0u32]);
        assert_eq!(load_vec(&e, mid), [0.0; 4]);
        assert_eq!(load_vec(&e, mid + 0x10), [6.0; 4]);
        assert_eq!(load_vec(&e, root), [-3.0; 4]);
        assert_eq!(load_vec(&e, root + 0x10), [6.0; 4]);
    }
}

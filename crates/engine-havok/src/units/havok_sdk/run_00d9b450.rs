//! `Havok SDK/run_00d9b450` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The narrow node accessor (`00d9a4d0`, `thiscall(manager, index)`): the address of node `index`
/// (16-bit index, 0x14-byte nodes) in the manager at `tree + 0x1c`.
const NODE_AT: u32 = 0x00d9_a4d0;

/// The minimum of two 16-bit words, the exe's way: the first when it is below the second
/// (unsigned), otherwise the second.
fn lower(first: u16, second: u16) -> u16 {
    if first < second {
        first
    } else {
        second
    }
}

/// The maximum of two 16-bit words, the exe's way: the first when it is above the second,
/// otherwise the second.
fn higher(first: u16, second: u16) -> u16 {
    if first > second {
        first
    } else {
        second
    }
}

// Translated from 00d9b450 (decompiled, FalloutNV.exe 1.4.0.525)
/// The root fix-up of the narrow tree (`hkaabbtree.rs` calls it as `FIX_UP_NARROW`): thiscall,
/// three stack words `(node, tag, last)` of which only the low 16 bits are used, `RET 0xc`.
/// Stores `last` at `node + 0x12`; an inner node (bit 15 of the 16-bit word at `+0x10` clear)
/// looks its two children up through `00d9a4d0` on the manager at `this + 0x1c` (their indices
/// are the words at `+0xc` and `+0xe`), puts `tag` into their `+0x10` words (keeping bit 15),
/// recurses into the first with `(first index, second index)` and into the second with `(second
/// index, last)` and takes the box of the children: the 16-bit words at `+0`, `+2` and `+4` are
/// the larger of the two children's, those at `+6`, `+8` and `+0xa` the smaller. The class is
/// the narrow `hkAabbTree` of `hkaabbtree.rs`.
pub fn fn_00d9b450(e: &mut Engine, this: Ptr, node: u32, tag: u32, last: u32) {
    let this = this.addr();
    e.mem.set_u16(node + 0x12, last as u16);
    if e.mem.u16(node + 0x10) & 0x8000 != 0 {
        return;
    }
    let manager = this.wrapping_add(0x1c);
    let first_index = u32::from(e.mem.u16(node + 0xc));
    let first = e.call(NODE_AT, &args![manager, first_index]).u32();
    let second_index = u32::from(e.mem.u16(node + 0xe));
    let second = e.call(NODE_AT, &args![manager, second_index]).u32();
    let word = (e.mem.u16(first + 0x10) & 0x8000) | (tag as u16);
    e.mem.set_u16(first + 0x10, word);
    let word = (e.mem.u16(second + 0x10) & 0x8000) | (tag as u16);
    e.mem.set_u16(second + 0x10, word);
    let first_index = u32::from(e.mem.u16(node + 0xc));
    let second_index = u32::from(e.mem.u16(node + 0xe));
    fn_00d9b450(e, Ptr::new(this), first, first_index, second_index);
    let second_index = u32::from(e.mem.u16(node + 0xe));
    fn_00d9b450(e, Ptr::new(this), second, second_index, last);
    for (offset, take_lower) in [
        (6u32, true),
        (0, false),
        (8, true),
        (2, false),
        (0xa, true),
        (4, false),
    ] {
        let from_second = e.mem.u16(second + offset);
        let from_first = e.mem.u16(first + offset);
        let value = if take_lower {
            lower(from_first, from_second)
        } else {
            higher(from_first, from_second)
        };
        e.mem.set_u16(node + offset, value);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00d9_b450, fn_00d9b450(Ptr, u32, u32, u32))]
}

#[cfg(test)]
mod a09_tests {
    use super::*;

    struct Tree {
        this: u32,
        nodes: u32,
    }

    fn tree(e: &mut Engine) -> Tree {
        e.register(NODE_AT, |e, a| Ret {
            eax: e.mem.u32(a[0]) + 0x14 * (a[1] & 0xffff),
            ..Ret::default()
        });
        let this = e.mem.alloc(0x40);
        let nodes = e.mem.alloc(0x14 * 8);
        e.mem.set_u32(this + 0x1c, nodes);
        Tree { this, nodes }
    }

    fn node(tree: &Tree, index: u32) -> u32 {
        tree.nodes + 0x14 * index
    }

    fn set_box(e: &mut Engine, node: u32, words: [u16; 6]) {
        for (i, value) in words.iter().enumerate() {
            // The box words are stored in the order +6, +0, +8, +2, +0xa, +4 of the exe's use:
            // here `words` are (x_high, y_high, z_high, x_low, y_low, z_low) by offset.
            let offset = [0u32, 2, 4, 6, 8, 0xa][i];
            e.mem.set_u16(node + offset, *value);
        }
    }

    fn get_box(e: &Engine, node: u32) -> [u16; 6] {
        [0u32, 2, 4, 6, 8, 0xa].map(|offset| e.mem.u16(node + offset))
    }

    #[test]
    fn the_parent_takes_larger_first_words_and_smaller_second_words() {
        let mut e = Engine::new();
        let tree = tree(&mut e);
        let root = node(&tree, 0);
        let (left, right) = (node(&tree, 1), node(&tree, 2));
        e.mem.set_u16(root + 0xc, 1);
        e.mem.set_u16(root + 0xe, 2);
        e.mem.set_u16(left + 0x10, 0x8000);
        e.mem.set_u16(right + 0x10, 0x8000);
        set_box(&mut e, left, [10, 20, 30, 5, 6, 7]);
        set_box(&mut e, right, [12, 18, 31, 4, 9, 3]);
        e.call(0x00d9_b450, &args![tree.this, root, 0x1_0044u32, 0x7fffu32]);
        // Words +0, +2, +4: the larger; words +6, +8, +0xa: the smaller.
        assert_eq!(get_box(&e, root), [12, 20, 31, 4, 6, 3]);
        assert_eq!(e.mem.u16(left + 0x10), 0x8044);
        assert_eq!(e.mem.u16(right + 0x10), 0x8044);
        assert_eq!(e.mem.u16(root + 0x12), 0x7fff);
        assert_eq!(e.mem.u16(left + 0x12), 2);
        assert_eq!(e.mem.u16(right + 0x12), 0x7fff);
    }

    #[test]
    fn equal_words_take_the_second_childs_value_and_leaves_are_left_alone() {
        let mut e = Engine::new();
        let tree = tree(&mut e);
        let leaf = node(&tree, 3);
        e.mem.set_u16(leaf + 0x10, 0x8001);
        set_box(&mut e, leaf, [1, 2, 3, 4, 5, 6]);
        e.call(0x00d9_b450, &args![tree.this, leaf, 9u32, 0xffff_0123u32]);
        assert_eq!(e.mem.u16(leaf + 0x12), 0x0123);
        assert_eq!(get_box(&e, leaf), [1, 2, 3, 4, 5, 6]);
        // The helpers: ties go to the second operand.
        assert_eq!(lower(3, 3), 3);
        assert_eq!(higher(5, 5), 5);
        assert_eq!(lower(2, 9), 2);
        assert_eq!(higher(2, 9), 9);
        assert_eq!(lower(0x9000, 0x10), 0x10);
    }

    #[test]
    fn a_two_level_tree_is_combined_bottom_up() {
        let mut e = Engine::new();
        let tree = tree(&mut e);
        let (root, mid, a, b, c) = (
            node(&tree, 0),
            node(&tree, 1),
            node(&tree, 2),
            node(&tree, 3),
            node(&tree, 4),
        );
        e.mem.set_u16(root + 0xc, 1);
        e.mem.set_u16(root + 0xe, 4);
        e.mem.set_u16(mid + 0xc, 2);
        e.mem.set_u16(mid + 0xe, 3);
        for leaf in [a, b, c] {
            e.mem.set_u16(leaf + 0x10, 0x8000);
        }
        set_box(&mut e, a, [1, 1, 1, 0, 0, 0]);
        set_box(&mut e, b, [5, 5, 5, 3, 3, 3]);
        set_box(&mut e, c, [7, 2, 2, 9, 1, 1]);
        e.call(0x00d9_b450, &args![tree.this, root, 1u32, 0u32]);
        assert_eq!(get_box(&e, mid), [5, 5, 5, 0, 0, 0]);
        assert_eq!(get_box(&e, root), [7, 5, 5, 0, 0, 0]);
    }
}

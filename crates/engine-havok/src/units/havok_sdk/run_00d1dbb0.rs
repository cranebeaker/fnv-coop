//! `Havok SDK/run_00d1dbb0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`00c85750`): its field `+0x10` is the allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// The one-element grow of an `hkArray` (`00c90940`, cdecl `(allocator, array, element size)`).
const ARRAY_GROW: u32 = 0x00c9_0940;
/// The node of a tree by index (`00d9a4c0`, thiscall on the container at `tree + 0x1c`), the box
/// distance function of this unit (`00d1bad0`), and the float `0x7f7fffee` (`0104da48`) the best
/// distance starts from.
const NODE_BY_INDEX: u32 = 0x00d9_a4c0;
const BOX_DISTANCE: u32 = 0x00d1_bad0;
const LARGE_FLOAT: u32 = 0x0104_da48;

/// Pushes `node` on the `hkArray` of pointers at `array`: after `00c85750`, the array grows through
/// `00c90940` with `(allocator, array, 4)` when its size equals the capacity bits, then the
/// pointer is stored and the size incremented.
fn push_node_00d1dbb0(e: &mut Engine, array: u32, node: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
    if e.mem.u32(array + 4) == capacity {
        let allocator = e.mem.u32(router + 0x10);
        e.call(ARRAY_GROW, &args![allocator, array, 4u32]);
    }
    let size = e.mem.u32(array + 4);
    let data = e.mem.u32(array);
    e.mem.set_u32(data.wrapping_add(size.wrapping_mul(4)), node);
    e.mem.set_u32(array + 4, size.wrapping_add(1));
}

// Translated from 00d1dbb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (cdecl, three words `tree`, `point`, `callback`; the names are descriptions, not
/// confirmed): a nearest-first walk of a bounding-volume tree. The root is the word at
/// `tree + 0x10` (nothing happens when it is 0). Nodes are 0x30 bytes: the box (minimum at `+0`,
/// maximum at `+0x10`), the child indices at `+0x20` and `+0x24`, and a word at `+0x28` whose top
/// bit marks a leaf; `00d9a4c0` (thiscall on `tree + 0x1c`) answers a node by index. A leaf is
/// given to slot 4 of `callback`, which answers the new best distance as a float: 0.0 ends the
/// walk. The walk keeps its own stack (an `hkArray` of node pointers with inline room for 0x80,
/// grown through `00c90940`), starting with the root, and a best distance starting at the float
/// at `0104da48`. For an inner node the two children's boxes are compared with `point` through
/// `00d1bad0` (squared distances `a` and `b`, stored as floats): with `best > a` (bit 0) and `b <
/// best` (bit 1) both set the farther child is pushed first and the nearer last (for equal or
/// unordered distances `a` first, then `b`); with only `best > a` just `a` is pushed, with only `b <
/// best` just `b`, and with neither none. The walk ends when the stack is empty, a leaf answers 0,
/// or immediately after the callback when the root itself is a leaf. The stack's data is freed when
/// it was grown.
pub fn fn_00d1dbb0(e: &mut Engine, tree: Ptr, point: u32, callback: u32) {
    let root = e.mem.u32(tree.addr() + 0x10);
    if root == 0 {
        return;
    }
    let large = x87_quiet_bits(e.mem.u32(LARGE_FLOAT));
    e.with_stack(0x270, |e, frame| {
        let t = frame.addr();
        e.mem.set_u32(t + 0x14, large);
        e.mem.set_u32(t + 0x20, t + 0x2c);
        e.mem.set_u32(t + 0x24, 0);
        e.mem.set_u32(t + 0x28, 0x8000_0080);
        let stack = t + 0x20;
        if e.mem.u32(root + 0x28) & 0x8000_0000 != 0 {
            e.vcall(callback, 4, &args![root]);
        } else {
            push_node_00d1dbb0(e, stack, root);
            loop {
                let size = e.mem.u32(stack + 4);
                let data = e.mem.u32(stack);
                let node = e
                    .mem
                    .u32(data.wrapping_add(size.wrapping_mul(4)).wrapping_sub(4));
                e.mem.set_u32(stack + 4, size.wrapping_sub(1));
                if e.mem.u32(node + 0x28) & 0x8000_0000 != 0 {
                    let answer = e.vcall(callback, 4, &args![node]).f64();
                    let best_bits = x87_store32(answer);
                    e.mem.set_u32(t + 0x14, best_bits);
                    if x87_load32(best_bits) == 0.0 {
                        break;
                    }
                } else {
                    let container = tree.addr() + 0x1c;
                    let first_index = e.mem.u32(node + 0x20);
                    let first = e.call(NODE_BY_INDEX, &args![container, first_index]).u32();
                    let second_index = e.mem.u32(node + 0x24);
                    let second = e.call(NODE_BY_INDEX, &args![container, second_index]).u32();
                    for lane in 0..4 {
                        let low = e.mem.u32(first + lane * 4);
                        let high = e.mem.u32(first + 0x10 + lane * 4);
                        e.mem.set_u32(t + 0x230 + lane * 4, low);
                        e.mem.set_u32(t + 0x240 + lane * 4, high);
                    }
                    for lane in 0..4 {
                        let low = e.mem.u32(second + lane * 4);
                        let high = e.mem.u32(second + 0x10 + lane * 4);
                        e.mem.set_u32(t + 0x250 + lane * 4, low);
                        e.mem.set_u32(t + 0x260 + lane * 4, high);
                    }
                    let first_distance = e.call(BOX_DISTANCE, &args![t + 0x230, point]).f64();
                    e.mem.set_u32(t + 0x1c, x87_store32(first_distance));
                    let second_distance = e.call(BOX_DISTANCE, &args![t + 0x250, point]).f64();
                    e.mem.set_u32(t + 0x18, x87_store32(second_distance));
                    let a = x87_load32(e.mem.u32(t + 0x1c));
                    let best = x87_load32(e.mem.u32(t + 0x14));
                    let b = x87_load32(e.mem.u32(t + 0x18));
                    let selector = (u32::from(best > a) + if b < best { 2 } else { 0 }) & 3;
                    match selector {
                        1 => push_node_00d1dbb0(e, stack, first),
                        2 => push_node_00d1dbb0(e, stack, second),
                        3 => {
                            if b > a {
                                push_node_00d1dbb0(e, stack, second);
                                push_node_00d1dbb0(e, stack, first);
                            } else {
                                push_node_00d1dbb0(e, stack, first);
                                push_node_00d1dbb0(e, stack, second);
                            }
                        }
                        _ => {}
                    }
                }
                if e.mem.u32(stack + 4) == 0 {
                    break;
                }
            }
        }
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let capacity = e.mem.u32(stack + 8);
        e.mem.set_u32(stack + 4, 0);
        if (capacity as i32) >= 0 {
            let data = e.mem.u32(stack);
            let bytes = (capacity & 0x3fff_ffff).wrapping_mul(4);
            e.vcall(allocator, 0x10, &args![data, bytes]);
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00d1_dbb0, fn_00d1dbb0(Ptr, u32, u32))]
}

#[cfg(test)]
mod havok_a04_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const NODE_SIZE: u32 = 0x30;

    struct Scene {
        tree: u32,
        point: u32,
        /// The leaf nodes the callback was given, by node address.
        leaves: Rc<RefCell<Vec<u32>>>,
        nodes: u32,
    }

    /// Nodes: 0 root (children 1 and 2), 1 leaf `L1` (box 0..1), 2 inner `N2` (box 2..3, children
    /// 3 and 4), 3 leaf `C` (box 1..2 on x), 4 leaf `D` (box 10..11 on x). The point is the origin,
    /// so the squared distances are `L1` 0, `N2` 4, `C` 1 and `D` 100. The callback answers 5.0
    /// for the first leaf and what `second` says for the next ones.
    fn scene(e: &mut Engine, answers: [f64; 2]) -> Scene {
        e.map(0x0100_0000, 0x0030_0000);
        e.mem.set_u32(LARGE_FLOAT, 0x7f7f_ffee);
        let nodes = e.mem.alloc(NODE_SIZE * 5);
        let boxes: [([f32; 3], [f32; 3]); 5] = [
            ([0.0; 3], [0.0; 3]),
            ([0.0, 0.0, 0.0], [1.0, 1.0, 1.0]),
            ([2.0, 0.0, 0.0], [3.0, 1.0, 1.0]),
            ([1.0, 0.0, 0.0], [2.0, 1.0, 1.0]),
            ([10.0, 0.0, 0.0], [11.0, 1.0, 1.0]),
        ];
        for (index, (low, high)) in boxes.iter().enumerate() {
            let at = nodes + index as u32 * NODE_SIZE;
            for lane in 0..3 {
                e.mem.set_f32(at + lane as u32 * 4, low[lane]);
                e.mem.set_f32(at + 0x10 + lane as u32 * 4, high[lane]);
            }
        }
        for (index, children) in [(0u32, (1u32, 2u32)), (2, (3, 4))] {
            e.mem.set_u32(nodes + index * NODE_SIZE + 0x20, children.0);
            e.mem.set_u32(nodes + index * NODE_SIZE + 0x24, children.1);
        }
        for index in [1u32, 3, 4] {
            e.mem.set_u32(nodes + index * NODE_SIZE + 0x28, 0x8000_0000);
        }
        let tree = e.mem.alloc(0x40);
        e.mem.set_u32(tree + 0x10, nodes);
        e.register_double(NODE_BY_INDEX, move |_, a| Ret {
            eax: nodes + a[1] * NODE_SIZE,
            ..Ret::default()
        });
        let router = e.mem.alloc(0x20);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 4, 0x0900_0161);
        let callback = e.mem.alloc(0x10);
        e.mem.set_u32(callback, vtable);
        let leaves = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&leaves);
        e.register_double(0x0900_0161, move |_, a| {
            log.borrow_mut().push(a[1]);
            let seen = log.borrow().len();
            Ret {
                st0: if seen == 1 { answers[0] } else { answers[1] },
                ..Ret::default()
            }
        });
        let point = e.mem.alloc(0x10);
        e.mem.set_u32(tree + 0x3c, callback);
        Scene {
            tree,
            point,
            leaves,
            nodes,
        }
    }

    fn run(e: &mut Engine, s: &Scene) {
        let callback = e.mem.u32(s.tree + 0x3c);
        e.call(0x00d1_dbb0, &args![s.tree, s.point, callback]);
    }

    #[test]
    fn fn_00d1dbb0_visits_the_nearest_leaf_first_and_prunes_the_far_ones() {
        let mut e = Engine::new();
        // The first leaf answers 5.0, the second 0.0: the walk ends there.
        let s = scene(&mut e, [5.0, 0.0]);
        run(&mut e, &s);
        let leaf = |index: u32| s.nodes + index * NODE_SIZE;
        // L1 (distance 0) first; then N2 is expanded: C (distance 1 < 5) is pushed, D (100) is not.
        assert_eq!(*s.leaves.borrow(), vec![leaf(1), leaf(3)]);
    }

    #[test]
    fn fn_00d1dbb0_walks_every_leaf_while_the_answers_stay_above_zero() {
        let mut e = Engine::new();
        // The best distance stays huge, so every box is pushed: L1, then the nearer of N2's
        // children (C), then D.
        let s = scene(&mut e, [1.0e30, 1.0e30]);
        run(&mut e, &s);
        let leaf = |index: u32| s.nodes + index * NODE_SIZE;
        assert_eq!(*s.leaves.borrow(), vec![leaf(1), leaf(3), leaf(4)]);
    }

    #[test]
    fn fn_00d1dbb0_calls_back_once_for_a_leaf_root_and_not_at_all_without_a_root() {
        let mut e = Engine::new();
        let s = scene(&mut e, [5.0, 5.0]);
        // A leaf root.
        e.mem.set_u32(s.tree + 0x10, s.nodes + NODE_SIZE);
        run(&mut e, &s);
        assert_eq!(*s.leaves.borrow(), vec![s.nodes + NODE_SIZE]);
        // No root.
        e.mem.set_u32(s.tree + 0x10, 0);
        e.call_log = Some(vec![]);
        run(&mut e, &s);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        assert_eq!(s.leaves.borrow().len(), 1);
    }

    #[test]
    fn fn_00d1dbb0_grows_and_frees_a_deep_stack() {
        // A chain of `DEPTH` inner nodes: each has a far leaf (pushed first) and the next inner
        // node, or a final leaf, as the near child (pushed last, so walked first); every far leaf
        // stays on the stack, which therefore reaches the inline capacity of 0x80 and grows.
        const DEPTH: u32 = 130;
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.mem.set_u32(LARGE_FLOAT, 0x7f7f_ffee);
        let nodes = e.mem.alloc(NODE_SIZE * (2 * DEPTH + 1));
        let node = |index: u32| nodes + index * NODE_SIZE;
        // Far leaves are the nodes DEPTH + 1 + i, with a box at x = 10.
        for i in 0..DEPTH {
            e.mem.set_f32(node(DEPTH + 1 + i), 10.0);
            e.mem.set_f32(node(DEPTH + 1 + i) + 0x10, 11.0);
            e.mem.set_u32(node(DEPTH + 1 + i) + 0x28, 0x8000_0000);
        }
        e.mem.set_u32(node(DEPTH) + 0x28, 0x8000_0000);
        for i in 0..DEPTH {
            let near = if i + 1 < DEPTH { i + 1 } else { DEPTH };
            e.mem.set_u32(node(i) + 0x20, DEPTH + 1 + i);
            e.mem.set_u32(node(i) + 0x24, near);
        }
        let tree = e.mem.alloc(0x40);
        e.mem.set_u32(tree + 0x10, node(0));
        e.register_double(NODE_BY_INDEX, move |_, a| Ret {
            eax: nodes + a[1] * NODE_SIZE,
            ..Ret::default()
        });
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0x10, 0x0900_0162);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        // The grow double moves the array to the heap and makes the capacity word plain.
        e.register(ARRAY_GROW, |e, a| {
            let size = e.mem.u32(a[1] + 4);
            let old = e.mem.u32(a[1]);
            let block = e.mem.alloc(0x400);
            for word in 0..size {
                let value = e.mem.u32(old + word * 4);
                e.mem.set_u32(block + word * 4, value);
            }
            e.mem.set_u32(a[1], block);
            e.mem.set_u32(a[1] + 8, 0x100);
            Ret {
                eax: block,
                ..Ret::default()
            }
        });
        e.register(0x0900_0162, |_, _| Ret::default());
        let callback_vtable = e.mem.alloc(0x20);
        e.mem.set_u32(callback_vtable + 4, 0x0900_0163);
        let callback = e.mem.alloc(0x10);
        e.mem.set_u32(callback, callback_vtable);
        let visited = Rc::new(RefCell::new(0u32));
        let count = Rc::clone(&visited);
        e.register_double(0x0900_0163, move |_, _| {
            *count.borrow_mut() += 1;
            Ret {
                st0: 1.0e30,
                ..Ret::default()
            }
        });
        let point = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x00d1_dbb0, &args![tree, point, callback]);
        // Every leaf: the final one and the DEPTH far ones.
        assert_eq!(*visited.borrow(), DEPTH + 1);
        let log = e.call_log.take().unwrap();
        let grows: Vec<_> = log.iter().filter(|(a, _)| *a == ARRAY_GROW).collect();
        assert_eq!(grows.len(), 1);
        assert_eq!(grows[0].1[0], allocator);
        assert_eq!(grows[0].1[2], 4);
        let frees: Vec<_> = log.iter().filter(|(a, _)| *a == 0x0900_0162).collect();
        assert_eq!(frees.len(), 1);
        assert_eq!(frees[0].1[0], allocator);
        assert_eq!(frees[0].1[2], 0x100 * 4);
    }
}

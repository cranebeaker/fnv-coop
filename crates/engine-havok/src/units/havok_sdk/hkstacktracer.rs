//! `Havok SDK/hkstacktracer.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkStackTracer` (RTTI vtable `010d4de4`) is a singleton kept in static storage at `01269a90`;
//! the part at `+0xc` is the debug-help loader state that `00da2840` fills. The call tree
//! (`hkStackTracer::CallTree`) is an `hkArray` of 16-byte nodes, data at `+0`, size at `+4`,
//! capacity and flags at `+8`, followed by the allocator pointer at `+0xc`. A node is
//! `(address, parent index, first child index, next sibling index)`; index 0 is the root and a
//! link of -1 (or any value of 0 or less) means none.

#[allow(unused_imports)]
use crate::prelude::*;

/// The singleton's storage and the loader part inside it (`01269a9c`, the address the object keeps
/// at `+8`).
const STACK_TRACER_OBJECT: u32 = 0x0126_9a90;
const STACK_TRACER_LOADER_STATE: u32 = 0x0126_9a9c;
/// `hkStackTracer`'s vtable (RTTI-confirmed).
const STACK_TRACER_VTABLE: u32 = 0x010d_4de4;
/// The loader constructor of the debug-help part (`00da2840`, `thiscall`, no stack words).
const LOADER_CONSTRUCTOR: u32 = 0x00da_2840;
/// `hkArrayUtil::_reserve` (cdecl: allocator, array, count, element size) and `00c90940`
/// (cdecl: allocator, array, element size; grows by one step).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
const ARRAY_RESERVE_MORE: u32 = 0x00c9_0940;

/// Size of a call-tree node.
const NODE_SIZE: u32 = 0x10;
const CAPACITY_MASK: u32 = 0x3fff_ffff;

// Translated from 00da2b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStackTracer::getStackTrace` (Xbox PDB), virtual, `RET 8` (two words, not read; `ECX` is the
/// tracer, not read): answers 0 (no frames).
pub fn hk_stack_tracer_get_stack_trace(
    _e: &mut Engine,
    _this: Ptr,
    _unused_1: u32,
    _unused_2: u32,
) -> u32 {
    0
}

// Translated from 00da2b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStackTracer::CallTree::getCallStack` (Xbox PDB), `thiscall`, `RET 0xc`: walks from the node
/// `node` up through the parents, storing each node's address in the next word of `buffer`, until
/// the node index is 0 or less or `max` addresses are stored; at most `max` words are written.
/// Answers (in `EAX`; the decompiler shows no result) the number written.
pub fn hk_stack_tracer_call_tree_get_call_stack(
    e: &mut Engine,
    this: Ptr,
    node: i32,
    buffer: u32,
    max: i32,
) -> i32 {
    let tree = this.addr();
    let mut count: i32 = 0;
    let mut node = node;
    if max > 0 {
        loop {
            if node <= 0 {
                break;
            }
            let offset = (node as u32).wrapping_shl(4);
            let data = e.mem.u32(tree);
            let address = e.mem.u32(offset.wrapping_add(data));
            e.mem
                .set_u32(buffer.wrapping_add((count as u32).wrapping_mul(4)), address);
            let data = e.mem.u32(tree);
            node = e.mem.i32(offset.wrapping_add(data).wrapping_add(4));
            count += 1;
            if count >= max {
                break;
            }
        }
    }
    count
}

// Translated from 00da2b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStackTracer`'s constructor (no map name; the vtable it stores is RTTI-confirmed
/// `hkStackTracer`), `ECX` = `this`, no stack words: stores the 16-bit word 1 at `+6`, the vtable
/// at `+0` and the address `01269a9c` at `+8`, then runs the loader constructor (`00da2840`) on
/// that address. Returns `this`.
pub fn hk_stack_tracer_constructor(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u16(base + 6, 1);
    e.mem.set_u32(base, STACK_TRACER_VTABLE);
    e.mem.set_u32(base + 8, STACK_TRACER_LOADER_STATE);
    e.call(LOADER_CONSTRUCTOR, &args![STACK_TRACER_LOADER_STATE]);
    this
}

// Translated from 00da2bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStackTracer::create` (Xbox PDB), no arguments: runs the constructor on the static storage at
/// `01269a90` (a tail jump) and answers its result, that address.
pub fn hk_stack_tracer_create(e: &mut Engine) -> Ptr {
    hk_stack_tracer_constructor(e, Ptr::new(STACK_TRACER_OBJECT))
}

/// Fills the node at `node` with address 0 and no parent, child or sibling (-1), unless `node` is
/// null, as the code does for every node it adds.
fn clear_node(e: &mut Engine, node: u32) {
    if node != 0 {
        e.mem.set_u32(node, 0);
        e.mem.set_u32(node + 4, 0xffff_ffff);
        e.mem.set_u32(node + 8, 0xffff_ffff);
        e.mem.set_u32(node + 0xc, 0xffff_ffff);
    }
}

// Translated from 00da2bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStackTracer::CallTree::insertCallStack` (Xbox PDB), `thiscall`, `RET 8`: `frames` is an
/// array of `depth` return addresses. First, an empty tree gets its root node (reserving room for
/// one node when the capacity is below 1: `_reserve(allocator, array, max(2 * capacity, 1), 0x10)`;
/// the root has address 0, parent -1 and no children or siblings). Then the frames are followed
/// from the last (`frames[depth - 1]`) to the first, starting at the root: at each step the node's
/// children (first child at `+8`, then each next sibling at `+0xc`) are searched for the frame's
/// address; a missing one is added at the end of the array (growing it through `00c90940` when
/// full) as the first child of the current node, the old first child becoming its next sibling.
/// Answers the index of the node of `frames[0]` (0 when `depth` is 0 or less).
pub fn hk_stack_tracer_call_tree_insert_call_stack(
    e: &mut Engine,
    this: Ptr,
    frames: u32,
    depth: i32,
) -> i32 {
    let tree = this.addr();
    if e.mem.u32(tree + 4) == 0 {
        let capacity = (e.mem.u32(tree + 8) & CAPACITY_MASK) as i32;
        let allocator = e.mem.u32(tree + 0xc);
        if capacity < 1 {
            let doubled = capacity.wrapping_add(capacity);
            let wanted = if doubled > 1 { doubled } else { 1 };
            e.call(ARRAY_RESERVE, &args![allocator, tree, wanted, NODE_SIZE]);
        }
        let size = e.mem.i32(tree + 4);
        let mut missing = 1i32.wrapping_sub(size);
        let data = e.mem.u32(tree);
        let mut at = (size as u32).wrapping_mul(NODE_SIZE).wrapping_add(data);
        while missing > 0 {
            clear_node(e, at);
            at = at.wrapping_add(NODE_SIZE);
            missing -= 1;
        }
        e.mem.set_u32(tree + 4, 1);
    }
    let mut counter = depth.wrapping_sub(1);
    let mut current: i32 = 0;
    if counter < 0 {
        return current;
    }
    loop {
        let frame = e
            .mem
            .u32(frames.wrapping_add((counter as u32).wrapping_mul(4)));
        let data = e.mem.u32(tree);
        let parent_offset = (current as u32).wrapping_shl(4);
        let mut child = e.mem.i32(parent_offset.wrapping_add(data).wrapping_add(8));
        let mut found = None;
        if child > 0 {
            loop {
                let node = (child as u32).wrapping_shl(4).wrapping_add(data);
                if e.mem.u32(node) == frame {
                    found = Some(child);
                    break;
                }
                child = e.mem.i32(node + 0xc);
                if child <= 0 {
                    break;
                }
            }
        }
        // A found child of -1 (not possible: it is above 0) would fall into the insertion.
        let last = if let Some(index) = found.filter(|index| *index != -1) {
            index
        } else {
            let size = e.mem.i32(tree + 4);
            let capacity = (e.mem.u32(tree + 8) & CAPACITY_MASK) as i32;
            if size == capacity {
                let allocator = e.mem.u32(tree + 0xc);
                e.call(ARRAY_RESERVE_MORE, &args![allocator, tree, NODE_SIZE]);
            }
            let data = e.mem.u32(tree);
            let new_node = (e.mem.u32(tree + 4).wrapping_shl(4)).wrapping_add(data);
            clear_node(e, new_node);
            let new_index = e.mem.u32(tree + 4);
            let data = e.mem.u32(tree);
            let new_node = (new_index.wrapping_shl(4)).wrapping_add(data);
            e.mem.set_u32(tree + 4, new_index.wrapping_add(1));
            e.mem.set_u32(new_node, frame);
            e.mem.set_u32(new_node + 4, current as u32);
            let data = e.mem.u32(tree);
            let first_child = e.mem.u32(parent_offset.wrapping_add(data).wrapping_add(8));
            e.mem.set_u32(new_node + 0xc, first_child);
            let data = e.mem.u32(tree);
            e.mem
                .set_u32(parent_offset.wrapping_add(data).wrapping_add(8), new_index);
            new_index as i32
        };
        counter = counter.wrapping_sub(1);
        current = last;
        if counter < 0 {
            break last;
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00da2b20, hk_stack_tracer_get_stack_trace(Ptr, u32, u32) -> u32),
        entry!(0x00da2b50, hk_stack_tracer_call_tree_get_call_stack(Ptr, i32, u32, i32) -> i32),
        entry!(0x00da2b90, hk_stack_tracer_constructor(Ptr) -> Ptr),
        entry!(0x00da2bc0, hk_stack_tracer_create() -> Ptr),
        entry!(0x00da2bd0, hk_stack_tracer_call_tree_insert_call_stack(Ptr, u32, i32) -> i32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A call tree object (data, size, capacity and flags, allocator `0x7777`), empty and not
    /// owning storage.
    fn tree(e: &mut Engine) -> u32 {
        let tree = e.mem.alloc(0x10);
        e.mem.set_u32(tree + 8, 0x8000_0000);
        e.mem.set_u32(tree + 0xc, 0x7777);
        tree
    }

    /// Doubles for the two reserve functions: both give the array a new block holding the old
    /// nodes; `_reserve` sets the requested capacity, the grow step doubles it (at least 1).
    fn install_reserves(e: &mut Engine) {
        e.register(ARRAY_RESERVE, |e, a| {
            let (array, count) = (a[1], a[2]);
            let data = e.mem.alloc(count * 0x10);
            let size = e.mem.u32(array + 4);
            if size > 0 {
                let old = e.mem.u32(array);
                let kept = e.mem.bytes(old, size * 0x10);
                e.mem.write(data, &kept);
            }
            e.mem.set_u32(array, data);
            e.mem.set_u32(array + 8, count);
            Ret::default()
        });
        e.register(ARRAY_RESERVE_MORE, |e, a| {
            let array = a[1];
            let size = e.mem.u32(array + 4);
            let count = (size * 2).max(1);
            let data = e.mem.alloc(count * 0x10);
            if size > 0 {
                let old = e.mem.u32(array);
                let kept = e.mem.bytes(old, size * 0x10);
                e.mem.write(data, &kept);
            }
            e.mem.set_u32(array, data);
            e.mem.set_u32(array + 8, count);
            Ret::default()
        });
    }

    fn node(e: &Engine, tree: u32, index: u32) -> [i32; 4] {
        let data = e.mem.u32(tree);
        [0, 4, 8, 0xc].map(|offset| e.mem.i32(data + 0x10 * index + offset))
    }

    fn insert(e: &mut Engine, tree: u32, frames: &[u32]) -> i32 {
        let block = e.mem.alloc(4 * frames.len().max(1) as u32);
        for (i, frame) in frames.iter().enumerate() {
            e.mem.set_u32(block + 4 * i as u32, *frame);
        }
        e.call(0x00da_2bd0, &args![tree, block, frames.len() as u32])
            .i32()
    }

    #[test]
    fn get_stack_trace_answers_nothing() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00da_2b20, &args![0x10u32, 1u32, 2u32]).u32(), 0);
    }

    #[test]
    fn get_call_stack_walks_up_to_the_root_or_the_limit() {
        let mut e = Engine::new();
        install_reserves(&mut e);
        let tree = tree(&mut e);
        // Frames [0xa3, 0xa2, 0xa1] from the root: nodes 1, 2, 3 with addresses 0xa1.. as inserted
        // (the last frame first).
        assert_eq!(insert(&mut e, tree, &[0xa3, 0xa2, 0xa1]), 3);
        let buffer = e.mem.alloc(16);
        let got = e.call(0x00da_2b50, &args![tree, 3i32, buffer, 10i32]).i32();
        assert_eq!(got, 3);
        assert_eq!(
            e.mem.bytes(buffer, 12),
            [0xa3u32, 0xa2, 0xa1]
                .iter()
                .flat_map(|w| w.to_le_bytes())
                .collect::<Vec<u8>>()
        );
        // A limit of 2 stops early; of 0 writes nothing; the root writes nothing.
        let buffer = e.mem.alloc(16);
        assert_eq!(
            e.call(0x00da_2b50, &args![tree, 3i32, buffer, 2i32]).i32(),
            2
        );
        assert_eq!(e.mem.u32(buffer), 0xa3);
        assert_eq!(e.mem.u32(buffer + 4), 0xa2);
        assert_eq!(e.mem.u32(buffer + 8), 0);
        assert_eq!(
            e.call(0x00da_2b50, &args![tree, 3i32, buffer, 0i32]).i32(),
            0
        );
        assert_eq!(
            e.call(0x00da_2b50, &args![tree, 0i32, buffer, 5i32]).i32(),
            0
        );
        assert_eq!(
            e.call(0x00da_2b50, &args![tree, -1i32, buffer, 5i32]).i32(),
            0
        );
    }

    #[test]
    fn the_constructor_and_create_build_the_singleton() {
        let mut e = Engine::new();
        e.map(0x0126_9000, 0x1000);
        e.register(LOADER_CONSTRUCTOR, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let object = e.call(0x00da_2bc0, &args![]).u32();
        assert_eq!(object, STACK_TRACER_OBJECT);
        assert_eq!(e.mem.u32(object), STACK_TRACER_VTABLE);
        assert_eq!(e.mem.u16(object + 6), 1);
        assert_eq!(e.mem.u32(object + 8), STACK_TRACER_LOADER_STATE);
        let log = e.call_log.take().unwrap();
        let loader: Vec<_> = log.iter().filter(|c| c.0 == LOADER_CONSTRUCTOR).collect();
        assert_eq!(loader.len(), 1);
        assert_eq!(loader[0].1, vec![STACK_TRACER_LOADER_STATE]);
        let other = e.mem.alloc(0x40);
        assert_eq!(e.call(0x00da_2b90, &args![other]).u32(), other);
        assert_eq!(e.mem.u32(other), STACK_TRACER_VTABLE);
    }

    #[test]
    fn insert_call_stack_builds_the_root_and_a_chain() {
        let mut e = Engine::new();
        install_reserves(&mut e);
        let tree = tree(&mut e);
        e.call_log = Some(vec![]);
        // A depth of 0 only makes the root (reserving room for one node: max(2 * 0, 1)).
        assert_eq!(insert(&mut e, tree, &[]), 0);
        assert_eq!(e.mem.u32(tree + 4), 1);
        assert_eq!(node(&e, tree, 0), [0, -1, -1, -1]);
        let reserve: Vec<_> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == ARRAY_RESERVE)
            .collect();
        assert_eq!(reserve.len(), 1);
        assert_eq!(reserve[0].1, vec![0x7777, tree, 1, 0x10]);
        // The frames are followed last first: root -> 30 -> 20 -> 10.
        assert_eq!(insert(&mut e, tree, &[10, 20, 30]), 3);
        assert_eq!(e.mem.u32(tree + 4), 4);
        assert_eq!(node(&e, tree, 0), [0, -1, 1, -1]);
        assert_eq!(node(&e, tree, 1), [30, 0, 2, -1]);
        assert_eq!(node(&e, tree, 2), [20, 1, 3, -1]);
        assert_eq!(node(&e, tree, 3), [10, 2, -1, -1]);
    }

    #[test]
    fn insert_call_stack_reuses_nodes_and_adds_siblings_at_the_front() {
        let mut e = Engine::new();
        install_reserves(&mut e);
        let tree = tree(&mut e);
        assert_eq!(insert(&mut e, tree, &[10, 20, 30]), 3);
        // Shares 30 and 20; the new leaf 11 becomes the first child of node 2, 10 its sibling.
        assert_eq!(insert(&mut e, tree, &[11, 20, 30]), 4);
        assert_eq!(node(&e, tree, 2), [20, 1, 4, -1]);
        assert_eq!(node(&e, tree, 4), [11, 2, -1, 3]);
        // The same stack again finds the leaf through the sibling link and adds nothing.
        assert_eq!(insert(&mut e, tree, &[10, 20, 30]), 3);
        assert_eq!(insert(&mut e, tree, &[11, 20, 30]), 4);
        assert_eq!(e.mem.u32(tree + 4), 5);
        // A different outermost frame is a new child of the root, in front of node 1.
        assert_eq!(insert(&mut e, tree, &[10, 99]), 6);
        assert_eq!(node(&e, tree, 0), [0, -1, 5, -1]);
        assert_eq!(node(&e, tree, 5), [99, 0, 6, 1]);
        assert_eq!(node(&e, tree, 6), [10, 5, -1, -1]);
    }

    #[test]
    fn insert_call_stack_grows_a_full_array_and_keeps_an_owned_one_with_room() {
        let mut e = Engine::new();
        install_reserves(&mut e);
        let tree = tree(&mut e);
        e.call_log = Some(vec![]);
        insert(&mut e, tree, &[1]);
        // Root (capacity 1) plus one node: full at the second node, so one grow step.
        let grows: Vec<_> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == ARRAY_RESERVE_MORE)
            .collect();
        assert_eq!(grows.len(), 1);
        assert_eq!(grows[0].1, vec![0x7777, tree, 0x10]);
        assert_eq!(e.mem.u32(tree + 8), 2);
        // The root and the first node fill the grown array: the second node needs another step.
        insert(&mut e, tree, &[2]);
        assert_eq!(e.mem.u32(tree + 4), 3);
        let count = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == ARRAY_RESERVE_MORE)
            .count();
        assert_eq!(count, 2);
        // A tree that already has nodes does not make a root again.
        let before = e.mem.u32(tree + 4);
        insert(&mut e, tree, &[]);
        assert_eq!(e.mem.u32(tree + 4), before);
    }
}

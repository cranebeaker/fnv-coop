//! `NiCollision/run_00eb9fa0` (Xbox PDB source unit), subsystem `NiCollision`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Both functions build `NiOBBNode` trees (the class named by the RTTI of the
//! vtable `010f1b58` they store: `.?AVNiOBBNode@@`). `00eba1b0` is a
//! constructor of that class; `00eb9fa0` builds a node's subtree over a range
//! of triangles, with `NiOBBLeaf` leaves (vtable `010f1b70`, RTTI
//! `.?AVNiOBBLeaf@@`, made by `00eb6b90`). The engine map has no Xbox names
//! for either function, so they are `fn_<addr>`.
//!
//! `NiOBBNode` belongs to `niobbnode.cpp`, which has no layout yet: its fields
//! are read at their offsets. Xbox PDB layout (size 0x8C, as the PC code
//! allocates): `m_box` (`NiOBBox`) at +0x04, whose `m_pRoot` is at +0x78
//! (node +0x7C); `m_pLeft` +0x80; `m_pRight` +0x84; `m_uiChangeStamp` +0x88.

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory manager (`011f6238`) with its allocate (`00aa3e40`) and free
/// (`00aa4060`), both `thiscall`, one word.
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_MANAGER_FREE: u32 = 0x00aa_4060;
/// The block allocation helpers, both `cdecl`: `00aa1070` (`(size)`) and
/// `00aa10f0` (`(block)`).
const BLOCK_ALLOCATE: u32 = 0x00aa_1070;
const BLOCK_FREE: u32 = 0x00aa_10f0;
/// The CRT square root (`00ec6040`): the value in ST0 (a leading `f64`), the
/// result in ST0.
const CRT_SQRT: u32 = 0x00ec_6040;

/// `00eb94e0` (`niobbnode.cpp` in the engine map), `thiscall` on a node, 8
/// words: `(root, triangles, vertices, areas, centroids, first, last,
/// indices)`. Called once on every node `00eb9fa0` builds, before the node is
/// split; what it computes is that function's business.
const NODE_RANGE_SETUP: u32 = 0x00eb_94e0;
/// `00eb8dc0` (unit `run_00eb8dc0`), `thiscall` on a node, 7 words:
/// `(centroids, first, last, indices, &left_last, &right_first, out)`. It
/// writes the two out words first; `00eb9fa0` reads them back as the last
/// index of the left range and the first of the right range, and reads the
/// triangle numbers of both ranges from `out`.
const SPLIT_RANGE: u32 = 0x00eb_8dc0;
/// `00eb6b90`, the `NiOBBLeaf` constructor (it stores `NiOBBLeaf`'s vtable),
/// `thiscall`, 5 words: `(root, triangles, vertices, leaf_vertices,
/// triangle)`; returns `this`.
const LEAF_CONSTRUCTOR: u32 = 0x00eb_6b90;

/// `NiOBBNode`'s vtable (RTTI `.?AVNiOBBNode@@`) and size (0x8C, the
/// allocation `00eb5f10` and `00eb9fa0` make).
const NODE_VTABLE: u32 = 0x010f_1b58;
const NODE_SIZE: u32 = 0x8c;
/// `NiOBBLeaf`'s size (0x98, Xbox PDB and the PC allocation).
const LEAF_SIZE: u32 = 0x98;
/// `NiOBBNode` fields (Xbox PDB names): `m_box.m_pRoot` +0x7C, `m_pLeft`
/// +0x80, `m_pRight` +0x84, `m_uiChangeStamp` +0x88.
const NODE_BOX_ROOT: u32 = 0x7c;
const NODE_LEFT: u32 = 0x80;
const NODE_RIGHT: u32 = 0x84;
const NODE_CHANGE_STAMP: u32 = 0x88;

/// `0.5` (`double`, read by `FMUL qword`).
const HALF: u32 = 0x0101_1588;
/// The `double` `0x3FD5555560000000` (the float nearest 1/3, widened), read
/// by `FLD qword`.
const ONE_THIRD: u32 = 0x0109_9ee0;

/// The inlined `NiOBBNode` allocation of `00eb9fa0` (`00eba00b` and
/// `00eba0c5`): 0x8C bytes from the memory manager; when it is not null its
/// vtable is set and `m_box.m_pRoot`, `m_pLeft`, `m_pRight` and
/// `m_uiChangeStamp` are cleared. Returns the node or 0.
fn new_node(e: &mut Engine) -> u32 {
    let node = e
        .call(MEMORY_MANAGER_ALLOCATE, &args![MEMORY_MANAGER, NODE_SIZE])
        .u32();
    if node != 0 {
        e.mem.set_u32(node, NODE_VTABLE);
        e.mem.set_u32(node + NODE_BOX_ROOT, 0);
        e.mem.set_u32(node + NODE_LEFT, 0);
        e.mem.set_u32(node + NODE_RIGHT, 0);
        e.mem.set_u32(node + NODE_CHANGE_STAMP, 0);
    }
    node
}

/// The inlined leaf allocation of `00eb9fa0` (`00eba077` and `00eba157`):
/// 0x98 bytes from the memory manager; when not null, the `NiOBBLeaf`
/// constructor runs on it with the triangle number in the low 16 bits of
/// `out[slot]` (`MOVZX word`). Returns the constructor's result or 0.
fn new_leaf(
    e: &mut Engine,
    root: Ptr,
    triangles: Ptr,
    vertices: Ptr,
    leaf_vertices: Ptr,
    out: Ptr,
    slot: i32,
) -> u32 {
    let leaf = e
        .call(MEMORY_MANAGER_ALLOCATE, &args![MEMORY_MANAGER, LEAF_SIZE])
        .u32();
    if leaf == 0 {
        return 0;
    }
    let triangle = u32::from(
        e.mem
            .u16(out.addr().wrapping_add((slot as u32).wrapping_mul(4))),
    );
    e.call(
        LEAF_CONSTRUCTOR,
        &args![leaf, root, triangles, vertices, leaf_vertices, triangle],
    )
    .u32()
}

// Translated from 00eb9fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Builds the `NiOBBNode` `this` over the triangles `indices[first..=last]`
/// (`thiscall`, 11 words, `RET 0x2C`; nothing returned).
///
/// `00eb94e0` runs on the node with `(root, triangles, vertices, areas,
/// centroids, first, last, indices)`. When `last - first` (signed) is below
/// `min_split` the node's `m_pLeft` and `m_pRight` are cleared and that is
/// all. Otherwise `00eb8dc0` splits the range, writing the left range's last
/// index and the right range's first index into two of this function's own
/// argument slots (those of `indices` and `first`) and the triangle numbers
/// into `scratch`. Then, for the left range `first..=left_last`: if it holds
/// more than one triangle, a new node (inlined allocation) becomes `m_pLeft`
/// and is built recursively with `indices` and `scratch` swapped; if it holds
/// one, a leaf for the triangle in `scratch[first]` becomes `m_pLeft` (0 when
/// the allocation fails); if `left_last < first`, `m_pLeft` is left alone. The
/// right range `right_first..=last` is the same with `m_pRight` and
/// `scratch[last]` for a single triangle, except that a new right node is
/// built by the same loop (the compiler turned that recursion into a jump):
/// `00eb94e0` runs on it with `scratch` as its index array, and the loop goes
/// on with that node, `right_first` as its first index and the two arrays
/// swapped. A right range with `right_first > last` leaves `m_pRight` alone.
///
/// A failed node allocation gives a null node, which the exe goes on to use
/// (the recursion or the loop writes through it); the translation does the
/// same.
pub fn fn_00eb9fa0(
    e: &mut Engine,
    this: Ptr,
    root: Ptr,
    triangles: Ptr,
    vertices: Ptr,
    leaf_vertices: Ptr,
    areas: Ptr,
    centroids: Ptr,
    first: i32,
    last: i32,
    indices: Ptr,
    scratch: Ptr,
    min_split: i32,
) {
    // The argument slots of `first`, `last` and `indices` (stack words 7 to
    // 9): `00eb8dc0` writes the right range's first index into the first and
    // the left range's last index into the third; the code reads all three
    // back from there.
    e.with_stack(12, |e, slots| {
        let first_slot = slots.addr();
        let last_slot = slots.addr() + 4;
        let left_last_slot = slots.addr() + 8;
        e.mem.set_u32(first_slot, first as u32);
        e.mem.set_u32(last_slot, last as u32);
        e.mem.set_u32(left_last_slot, indices.addr());

        e.call(
            NODE_RANGE_SETUP,
            &args![this, root, triangles, vertices, areas, centroids, first, last, indices],
        );
        let mut node = this;
        // ESI, EBX and EBP of the loop.
        let mut low = first;
        let mut indices = indices;
        let mut out = scratch;
        if last.wrapping_sub(low) < min_split {
            e.mem.set_u32(node.addr() + NODE_LEFT, 0);
            e.mem.set_u32(node.addr() + NODE_RIGHT, 0);
            return;
        }
        loop {
            let high = e.mem.u32(last_slot) as i32;
            e.call(
                SPLIT_RANGE,
                &args![
                    node,
                    centroids,
                    low,
                    high,
                    indices,
                    left_last_slot,
                    first_slot,
                    out
                ],
            );
            let left_last = e.mem.u32(left_last_slot) as i32;
            if low < left_last {
                let child = new_node(e);
                e.mem.set_u32(node.addr() + NODE_LEFT, child);
                fn_00eb9fa0(
                    e,
                    Ptr::new(child),
                    root,
                    triangles,
                    vertices,
                    leaf_vertices,
                    areas,
                    centroids,
                    low,
                    left_last,
                    out,
                    indices,
                    min_split,
                );
            } else if low == left_last {
                let leaf = new_leaf(e, root, triangles, vertices, leaf_vertices, out, low);
                e.mem.set_u32(node.addr() + NODE_LEFT, leaf);
            }

            let right_first = e.mem.u32(first_slot) as i32;
            let high = e.mem.u32(last_slot) as i32;
            if right_first >= high {
                if right_first == high {
                    let leaf = new_leaf(e, root, triangles, vertices, leaf_vertices, out, high);
                    e.mem.set_u32(node.addr() + NODE_RIGHT, leaf);
                }
                return;
            }
            let child = new_node(e);
            let high = e.mem.u32(last_slot) as i32;
            e.mem.set_u32(node.addr() + NODE_RIGHT, child);
            // The right node takes `out` as its index array and the old
            // index array as its output.
            std::mem::swap(&mut indices, &mut out);
            node = Ptr::new(child);
            low = right_first;
            e.call(
                NODE_RANGE_SETUP,
                &args![node, root, triangles, vertices, areas, centroids, low, high, indices],
            );
            let span = (e.mem.u32(last_slot) as i32).wrapping_sub(e.mem.u32(first_slot) as i32);
            if span < min_split {
                e.mem.set_u32(node.addr() + NODE_LEFT, 0);
                e.mem.set_u32(node.addr() + NODE_RIGHT, 0);
                return;
            }
        }
    })
}

// Translated from 00eba1b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// An `NiOBBNode` constructor (`thiscall`, 6 words, `RET 0x18`; returns
/// `this`): builds the node and its subtree over `count` triangles
/// (`triangles`: three `u16` vertex numbers each; `vertices`: 12-byte points).
///
/// Sets the vtable and clears `m_box.m_pRoot`, then makes per-triangle tables:
/// `areas` (`00aa1070`, `count` floats) gets half the length of the cross
/// product `(p1 - p0) x (p2 - p0)`, through the CRT square root, and
/// `centroids` (memory manager, `count` 12-byte points; the size is all ones
/// if `count * 12` overflows, which a 16-bit count cannot) gets
/// `(p2 + (p0 + p1)) * 1/3` per axis, all as the x87 code computes them. The
/// vertex numbers are read at the low 16 bits of a running word index
/// (`MOVZX BP`). Then an index array `0..count` and a scratch array (both
/// `00aa1070`) and `00eb9fa0` builds the tree over `0..=count - 1` with
/// `min_split`; the four tables are freed (scratch, indices, centroids
/// through the memory manager, areas) and `m_uiChangeStamp` is cleared.
/// `root` is only passed on; the constructor itself leaves `m_pRoot` 0.
pub fn fn_00eba1b0(
    e: &mut Engine,
    this: Ptr,
    root: Ptr,
    count: u16,
    triangles: Ptr,
    vertices: Ptr,
    leaf_vertices: Ptr,
    min_split: i32,
) -> Ptr {
    let count = u32::from(count);
    e.mem.set_u32(this.addr(), NODE_VTABLE);
    e.mem.set_u32(this.addr() + NODE_BOX_ROOT, 0);
    let areas = e.call(BLOCK_ALLOCATE, &args![count * 4]).u32();
    let centroids = e
        .call(
            MEMORY_MANAGER_ALLOCATE,
            &args![MEMORY_MANAGER, count.checked_mul(12).unwrap_or(u32::MAX)],
        )
        .u32();

    let mut index: u32 = 0;
    let mut area = areas;
    let mut centroid = centroids;
    for _ in 0..count {
        let vertex = |e: &Engine, index: u32| {
            let number = u32::from(e.mem.u16(triangles.addr() + (index & 0xffff) * 2));
            vertices.addr().wrapping_add(number * 12)
        };
        let p0 = vertex(e, index);
        index = index.wrapping_add(1);
        let p1 = vertex(e, index);
        index = index.wrapping_add(1);
        let p2 = vertex(e, index);
        index = index.wrapping_add(1);
        let component = |e: &Engine, point: u32, axis: u32| e.mem.u32(point + axis * 4);

        // The two edges, each component stored as a float.
        let mut edge1 = [0u32; 3];
        let mut edge2 = [0u32; 3];
        for axis in 0..3 {
            edge1[axis as usize] = x87_store32(x87_sub(
                x87_load32(component(e, p1, axis)),
                x87_m32(component(e, p0, axis)),
            ));
        }
        for axis in 0..3 {
            edge2[axis as usize] = x87_store32(x87_sub(
                x87_load32(component(e, p2, axis)),
                x87_m32(component(e, p0, axis)),
            ));
        }
        let [e1x, e1y, e1z] = edge1.map(x87_load32);
        let [e2x, e2y, e2z] = edge2.map(x87_load32);
        // The cross product (`00eba2a9`..`00eba2ed`), in the stack's order.
        let normal_x = x87_store32(x87_sub(x87_mul(e2z, e1y), x87_mul(e1z, e2y)));
        let normal_y = x87_store32(x87_sub(x87_mul(e1z, e2x), x87_mul(e2z, e1x)));
        let normal_z = x87_store32(x87_sub(x87_mul(e2y, e1x), x87_mul(e2x, e1y)));
        let nx = x87_load32(normal_x);
        let ny = x87_load32(normal_y);
        let nz = x87_load32(normal_z);
        let squares = x87_add(x87_mul(ny, ny), x87_mul(nx, nx));
        let length_squared = x87_store32(x87_add(squares, x87_mul(nz, nz)));
        let length = e.call(CRT_SQRT, &args![x87_load32(length_squared)]).f64();
        let length = x87_load32(x87_store32(length));
        let half = e.global::<f64>(HALF);
        e.mem.set_u32(area, x87_store32(x87_mul(length, half)));
        area = area.wrapping_add(4);

        // The centroid: `p0 + p1`, then `p2 +` that, then each times 1/3.
        let mut pair = [0u32; 3];
        for axis in 0..3 {
            pair[axis as usize] = x87_store32(x87_add(
                x87_load32(component(e, p0, axis)),
                x87_m32(component(e, p1, axis)),
            ));
        }
        let mut sum = [0u32; 3];
        for axis in 0..3 {
            sum[axis as usize] = x87_store32(x87_add(
                x87_load32(component(e, p2, axis)),
                x87_m32(pair[axis as usize]),
            ));
        }
        let third = x87_load64(e.global::<u64>(ONE_THIRD));
        // `FMUL ST1,ST0` (DC C9), `FMUL ST0,ST1` (D8 C9), `FMUL dword`.
        let centroid_x = x87_store32(x87_mul(x87_load32(sum[0]), third));
        let centroid_y = x87_store32(x87_mul(x87_load32(sum[1]), third));
        let centroid_z = x87_store32(x87_mul(third, x87_m32(sum[2])));
        e.mem.set_u32(centroid, centroid_x);
        e.mem.set_u32(centroid + 4, centroid_y);
        e.mem.set_u32(centroid + 8, centroid_z);
        centroid = centroid.wrapping_add(12);
    }

    let indices = e.call(BLOCK_ALLOCATE, &args![count * 4]).u32();
    for i in 0..count {
        e.mem.set_u32(indices.wrapping_add(i * 4), i);
    }
    let scratch = e.call(BLOCK_ALLOCATE, &args![count * 4]).u32();
    fn_00eb9fa0(
        e,
        this,
        root,
        triangles,
        vertices,
        leaf_vertices,
        Ptr::new(areas),
        Ptr::new(centroids),
        0,
        count.wrapping_sub(1) as i32,
        Ptr::new(indices),
        Ptr::new(scratch),
        min_split,
    );
    e.call(BLOCK_FREE, &args![scratch]);
    e.call(BLOCK_FREE, &args![indices]);
    e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER, centroids]);
    e.call(BLOCK_FREE, &args![areas]);
    e.mem.set_u32(this.addr() + NODE_CHANGE_STAMP, 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00eb9fa0,
            fn_00eb9fa0(Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, i32, i32, Ptr, Ptr, i32)
        ),
        entry!(
            0x00eba1b0,
            fn_00eba1b0(Ptr, Ptr, u16, Ptr, Ptr, Ptr, i32) -> Ptr
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// An engine with the memory manager, the block allocator and the leaf
    /// constructor doubled (the leaf keeps its triangle number at +0x8C), and
    /// `00eb94e0` doing nothing. `00eb8dc0` is doubled per test.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(MEMORY_MANAGER_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(MEMORY_MANAGER_FREE, |_, _| Ret::default());
        e.register(BLOCK_ALLOCATE, |e, a| e.mem.alloc(a[0].max(4)).into_ret());
        e.register(BLOCK_FREE, |_, _| Ret::default());
        e.register(NODE_RANGE_SETUP, |_, _| Ret::default());
        e.register(LEAF_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0] + 0x8c, a[5]);
            a[0].into_ret()
        });
        e
    }

    /// `00eb8dc0` double splitting `first..=last` in the middle: left
    /// `first..=(first + last) / 2`, right from the next index.
    fn split_in_the_middle(e: &mut Engine) {
        e.register(SPLIT_RANGE, |e, a| {
            let (first, last) = (a[2] as i32, a[3] as i32);
            let middle = (first + last) / 2;
            e.mem.set_u32(a[5], middle as u32);
            e.mem.set_u32(a[6], (middle + 1) as u32);
            Ret::default()
        });
    }

    /// An array of words.
    fn words(e: &mut Engine, values: &[u32]) -> Ptr {
        let array = e.mem.alloc(4 * values.len() as u32);
        for (i, value) in values.iter().enumerate() {
            e.mem.set_u32(array + 4 * i as u32, *value);
        }
        Ptr::new(array)
    }

    /// `00eb9fa0` on `first..=last` with fixed root/triangles/vertices/areas/
    /// centroids words; returns the node.
    fn build(
        e: &mut Engine,
        first: i32,
        last: i32,
        indices: Ptr,
        scratch: Ptr,
        min_split: i32,
    ) -> u32 {
        let node = e.mem.alloc(NODE_SIZE);
        e.mem.set_u32(node + NODE_LEFT, 0xaaaa_aaaa);
        e.mem.set_u32(node + NODE_RIGHT, 0xbbbb_bbbb);
        e.call(
            0x00eb_9fa0,
            &args![
                node, 0x100u32, 0x200u32, 0x300u32, 0x400u32, 0x500u32, 0x600u32, first, last,
                indices, scratch, min_split
            ],
        );
        node
    }

    #[test]
    fn build_splits_into_nodes_and_leaves() {
        let mut e = engine();
        split_in_the_middle(&mut e);
        let indices = words(&mut e, &[10, 11, 12, 13]);
        let scratch = words(&mut e, &[20, 21, 22, 23]);
        e.call_log = Some(vec![]);
        let node = build(&mut e, 0, 3, indices, scratch, 1);
        let log = e.call_log.take().unwrap();

        // 0..=3 splits into 0..=1 (a new left node, arrays swapped) and
        // 2..=3 (a new right node continued by the loop).
        let left = e.mem.u32(node + NODE_LEFT);
        let right = e.mem.u32(node + NODE_RIGHT);
        for child in [left, right] {
            assert_eq!(e.mem.u32(child), NODE_VTABLE);
            assert_eq!(e.mem.u32(child + NODE_BOX_ROOT), 0);
            assert_eq!(e.mem.u32(child + NODE_CHANGE_STAMP), 0);
        }
        // The left node's split writes into its `out`, the top's `indices`.
        let leaf = |e: &Engine, node: u32, side: u32| e.mem.u32(e.mem.u32(node + side) + 0x8c);
        assert_eq!(leaf(&e, left, NODE_LEFT), 10);
        assert_eq!(leaf(&e, left, NODE_RIGHT), 11);
        // The right node's loop pass writes into the old index array too.
        assert_eq!(leaf(&e, right, NODE_LEFT), 12);
        assert_eq!(leaf(&e, right, NODE_RIGHT), 13);

        let setups: Vec<_> = log
            .iter()
            .filter(|(addr, _)| *addr == NODE_RANGE_SETUP)
            .map(|(_, a)| a.clone())
            .collect();
        assert_eq!(
            setups,
            vec![
                vec![
                    node,
                    0x100,
                    0x200,
                    0x300,
                    0x500,
                    0x600,
                    0,
                    3,
                    indices.addr()
                ],
                vec![
                    left,
                    0x100,
                    0x200,
                    0x300,
                    0x500,
                    0x600,
                    0,
                    1,
                    scratch.addr()
                ],
                vec![
                    right,
                    0x100,
                    0x200,
                    0x300,
                    0x500,
                    0x600,
                    2,
                    3,
                    scratch.addr()
                ],
            ]
        );
        let splits: Vec<_> = log
            .iter()
            .filter(|(addr, _)| *addr == SPLIT_RANGE)
            .map(|(_, a)| (a[0], a[1], a[2], a[3], a[4], a[7]))
            .collect();
        assert_eq!(
            splits,
            vec![
                (node, 0x600, 0, 3, indices.addr(), scratch.addr()),
                (left, 0x600, 0, 1, scratch.addr(), indices.addr()),
                (right, 0x600, 2, 3, scratch.addr(), indices.addr()),
            ]
        );
        let leaves: Vec<_> = log
            .iter()
            .filter(|(addr, _)| *addr == LEAF_CONSTRUCTOR)
            .map(|(_, a)| a[1..].to_vec())
            .collect();
        assert_eq!(leaves.len(), 4);
        assert_eq!(leaves[0], vec![0x100, 0x200, 0x300, 0x400, 10]);
    }

    #[test]
    fn build_makes_leaves_from_the_low_half_of_out() {
        let mut e = engine();
        split_in_the_middle(&mut e);
        let indices = words(&mut e, &[10, 11]);
        let scratch = words(&mut e, &[0x0002_0014, 0xffff_0015]);
        let node = build(&mut e, 0, 1, indices, scratch, 1);
        let leaf = |e: &Engine, side: u32| e.mem.u32(e.mem.u32(node + side) + 0x8c);
        assert_eq!(leaf(&e, NODE_LEFT), 0x14);
        assert_eq!(leaf(&e, NODE_RIGHT), 0x15);
    }

    #[test]
    fn build_below_the_minimum_clears_the_children() {
        let mut e = engine();
        e.register(SPLIT_RANGE, |_, _| panic!("no split below the minimum"));
        let indices = words(&mut e, &[0; 4]);
        let scratch = words(&mut e, &[0; 4]);
        e.call_log = Some(vec![]);
        let node = build(&mut e, 0, 3, indices, scratch, 4);
        assert_eq!(e.mem.u32(node + NODE_LEFT), 0);
        assert_eq!(e.mem.u32(node + NODE_RIGHT), 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log.iter().filter(|(a, _)| *a == NODE_RANGE_SETUP).count(),
            1
        );
    }

    #[test]
    fn build_failed_leaf_allocation_and_empty_ranges() {
        let mut e = engine();
        // Leaves fail to allocate: single-triangle ranges give 0.
        e.register(MEMORY_MANAGER_ALLOCATE, |e, a| {
            if a[1] == LEAF_SIZE {
                0u32.into_ret()
            } else {
                e.mem.alloc(a[1]).into_ret()
            }
        });
        split_in_the_middle(&mut e);
        let indices = words(&mut e, &[0; 2]);
        let scratch = words(&mut e, &[0; 2]);
        let node = build(&mut e, 0, 1, indices, scratch, 1);
        assert_eq!(e.mem.u32(node + NODE_LEFT), 0);
        assert_eq!(e.mem.u32(node + NODE_RIGHT), 0);

        // A split with both ranges empty leaves both children alone.
        e.register(SPLIT_RANGE, |e, a| {
            e.mem.set_u32(a[5], a[2].wrapping_sub(1));
            e.mem.set_u32(a[6], a[3] + 1);
            Ret::default()
        });
        let node = build(&mut e, 0, 1, indices, scratch, 1);
        assert_eq!(e.mem.u32(node + NODE_LEFT), 0xaaaa_aaaa);
        assert_eq!(e.mem.u32(node + NODE_RIGHT), 0xbbbb_bbbb);
    }

    #[test]
    fn constructor_computes_areas_and_centroids_and_frees_them() {
        let mut e = engine();
        e.map(0x0101_1000, 0x1000);
        e.map(0x0109_9000, 0x1000);
        e.set_global(HALF, 0.5f64);
        e.set_global(ONE_THIRD, 0x3fd5_5555_6000_0000u64);
        e.register(CRT_SQRT, |_, a| f64::take(a, &mut 0).sqrt().into_ret());
        // The build's first node setup sees the tables.
        type Tables = std::rc::Rc<std::cell::RefCell<Vec<u32>>>;
        let seen: Tables = Default::default();
        let record = seen.clone();
        e.register_double(NODE_RANGE_SETUP, move |e, a| {
            let mut record = record.borrow_mut();
            for i in 0..2 {
                record.push(e.mem.u32(a[4] + 4 * i));
            }
            for i in 0..6 {
                record.push(e.mem.u32(a[5] + 4 * i));
            }
            for i in 0..2 {
                record.push(e.mem.u32(a[8] + 4 * i));
            }
            record.extend_from_slice(&a[6..8]);
            Ret::default()
        });
        let points: [[f32; 3]; 4] = [
            [0.0, 0.0, 0.0],
            [3.0, 0.0, 0.0],
            [0.0, 3.0, 0.0],
            [0.0, 0.0, 3.0],
        ];
        let vertices = e.mem.alloc(48);
        for (i, point) in points.iter().enumerate() {
            for (axis, value) in point.iter().enumerate() {
                e.mem
                    .set_f32(vertices + 12 * i as u32 + 4 * axis as u32, *value);
            }
        }
        let triangles = e.mem.alloc(12);
        for (i, number) in [0u16, 1, 2, 1, 2, 3].iter().enumerate() {
            e.mem.set_u16(triangles + 2 * i as u32, *number);
        }
        let node = e.mem.alloc(NODE_SIZE);
        e.mem.set_u32(node + NODE_BOX_ROOT, 0x1234);
        e.mem.set_u32(node + NODE_CHANGE_STAMP, 0x5678);
        e.call_log = Some(vec![]);
        let back = e
            .call(
                0x00eb_a1b0,
                &args![node, 0x100u32, 2u32, triangles, vertices, 0x400u32, 5i32],
            )
            .u32();
        let log = e.call_log.take().unwrap();
        assert_eq!(back, node);
        assert_eq!(e.mem.u32(node), NODE_VTABLE);
        assert_eq!(e.mem.u32(node + NODE_BOX_ROOT), 0);
        assert_eq!(e.mem.u32(node + NODE_CHANGE_STAMP), 0);
        // 1..=0 is below the minimum: no children.
        assert_eq!(e.mem.u32(node + NODE_LEFT), 0);
        assert_eq!(e.mem.u32(node + NODE_RIGHT), 0);

        let seen = seen.borrow();
        // Areas: |(3,0,0) x (0,3,0)| / 2 and |(-3,3,0) x (-3,0,3)| / 2.
        let second_area = ((243.0f64.sqrt() as f32) as f64 * 0.5) as f32;
        assert_eq!(seen[0], 4.5f32.to_bits());
        assert_eq!(seen[1], second_area.to_bits());
        // Centroids: 3 * 0.33333334 rounds to 1.0 as a float.
        let centroids: Vec<u32> = [1.0f32, 1.0, 0.0, 1.0, 1.0, 1.0]
            .iter()
            .map(|v| v.to_bits())
            .collect();
        assert_eq!(&seen[2..8], &centroids[..]);
        // The index array, then the range 0..=1.
        assert_eq!(&seen[8..12], &[0, 1, 0, 1]);

        let areas = log.iter().find(|(a, _)| *a == BLOCK_ALLOCATE).unwrap().1[0];
        assert_eq!(areas, 8);
        let tables = log
            .iter()
            .find(|(a, _)| *a == MEMORY_MANAGER_ALLOCATE)
            .unwrap();
        assert_eq!(tables.1, vec![MEMORY_MANAGER, 24]);
        let frees = log.iter().filter(|(a, _)| *a == BLOCK_FREE).count();
        assert_eq!(frees, 3);
        let manager_free = log.iter().find(|(a, _)| *a == MEMORY_MANAGER_FREE).unwrap();
        assert_eq!(manager_free.1[0], MEMORY_MANAGER);
    }

    #[test]
    fn constructor_without_triangles_builds_over_an_empty_range() {
        let mut e = engine();
        let node = e.mem.alloc(NODE_SIZE);
        e.call_log = Some(vec![]);
        e.call(
            0x00eb_a1b0,
            &args![node, 0x100u32, 0u32, 0x200u32, 0x300u32, 0x400u32, 1i32],
        );
        let log = e.call_log.take().unwrap();
        let setup = log.iter().find(|(a, _)| *a == NODE_RANGE_SETUP).unwrap();
        // last = count - 1 = -1; -1 - 0 is below the minimum.
        assert_eq!(&setup.1[6..8], &[0, u32::MAX]);
        assert_eq!(e.mem.u32(node + NODE_LEFT), 0);
        assert!(log.iter().all(|(a, _)| *a != SPLIT_RANGE));
    }
}

//! `Havok SDK/hkkdtreebuildingutils.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its
//! functions in FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkKdTreeBuildingUtils` is a class of static (cdecl) functions around the kd-tree builder
//! (`hkKdTreeBuilder<hkKdTreeProjectedEntry const*, ...>`, whose own functions are in the
//! neighbouring units). The Xbox PDB has no type for the structures they share, so they are
//! described from the code:
//!
//! - the *source* (`hkKdTreeBuilder` input, a virtual object): slot `0xc` answers the number of
//!   primitives, slot `0x10` a primitive's key from its index, slot `0x14` writes a primitive's
//!   16-byte projected entry from its key (`(key, address of entry)`);
//! - a *projected entry* is 16 bytes: three `u16` low bounds, the key's low half, three `u16`
//!   high bounds, the key's high half;
//! - the *buffers* are two blocks of `16 * count` bytes, their addresses in two words;
//! - the *builder context* `buildTree` makes on its stack and gives the recursive builder
//!   (`00df56a0`): `+0` the buffers, `+4` 0, `+8` and `+0x10` two words it is given, `+0xc` 0,
//!   `+0x20` the root's projected bounds (all low bounds 0, all high bounds `0xffff`, the two
//!   key halves uninitialised stack in the exe and 0 here: they are padding of the bounds),
//!   `+0x30` the sparse writer's address and `+0x40` the 16-byte constant at `012684d0`;
//! - the *tree* `buildTreeDistributed` and `buildTree_ov2` work on: `+0` the node array, `+4`
//!   the first node slot, `+0xc` the highest slot index (written by
//!   `completeDistributedBuild`), `+0x10` the count of the primitive range, `+0x18` a word
//!   passed on, `+0x1c` the leaf data.
//!
//! The functions that take a stack frame in the exe (`buildTree`, `buildTreeDistributed`)
//! build it in one `with_stack` block at the exe's offsets, because the recursive builder and
//! `splitTreeBuilding` read and write it by address. The 16-byte copies the exe does with
//! `MOVAPS` are four words.

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`00c85750`): its field `+0x10` is the heap allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// The sparse writer's constructor (`00df3110`, `this` in `ECX`) and flush (`00df3160`).
const WRITER_CONSTRUCT: u32 = 0x00df_3110;
const WRITER_FLUSH: u32 = 0x00df_3160;
/// `hkKdTreeBuilder<..>::buildTreeRecursive` (`00df56a0`, cdecl `(context, address of the node
/// array pointer, leaf data, address of a word holding 1, address of a counter)`).
const BUILD_TREE_RECURSIVE: u32 = 0x00df_56a0;
/// `hkKdTreeBuilder<..>::splitTreeBuilding` (`00df37b0`, cdecl, seven words).
const SPLIT_TREE_BUILDING: u32 = 0x00df_37b0;
/// The 16-byte constant the builder contexts carry (`012684d0`).
const BOUNDS_CONSTANT: u32 = 0x0126_84d0;
/// `hkArray` capacity word: the size bits, and the value of an array that owns nothing.
const CAPACITY_MASK: u32 = 0x3fff_ffff;
const EMPTY_CAPACITY: u32 = 0x8000_0000;

/// The heap allocator: field `+0x10` of the memory router. Its slot `4` allocates `(bytes)`,
/// `8` frees `(block, bytes)`, `0xc` allocates through the address of the byte count (the
/// allocator may grant more) and `0x10` frees.
fn heap_allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router.wrapping_add(0x10))
}

/// Copies the 16-byte constant at `012684d0` (a `MOVAPS`) to `destination`.
fn copy_bounds_constant(e: &mut Engine, destination: u32) {
    for word in 0..4 {
        let value = e.mem.u32(BOUNDS_CONSTANT + 4 * word);
        e.mem.set_u32(destination + 4 * word, value);
    }
}

/// Copies 16 bytes (a `MOVAPS`) from `source` to `destination`.
fn copy_16(e: &mut Engine, destination: u32, source: u32) {
    for word in 0..4 {
        let value = e.mem.u32(source.wrapping_add(4 * word));
        e.mem.set_u32(destination.wrapping_add(4 * word), value);
    }
}

// Translated from 00d9d800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkKdTreeBuildingUtils::allocateBuffers` (Xbox PDB), cdecl `(buffers, count)`: allocates two
/// blocks of `count * 16` bytes from the heap allocator (slot `4`) and stores their addresses
/// at `buffers + 0` and `buffers + 4`.
pub fn hk_kd_tree_building_utils_allocate_buffers(e: &mut Engine, buffers: Ptr, count: i32) {
    let bytes = (count as u32) << 4;
    let allocator = heap_allocator(e);
    let first = e.vcall(allocator, 4, &args![bytes]).u32();
    e.mem.set_u32(buffers.addr(), first);
    let allocator = heap_allocator(e);
    let second = e.vcall(allocator, 4, &args![bytes]).u32();
    e.mem.set_u32(buffers.addr() + 4, second);
}

// Translated from 00d9d840 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkKdTreeBuildingUtils::deallocateBuffers` (Xbox PDB), cdecl `(buffers, count)`: when `count`
/// is positive frees the block at `buffers + 4` and then the one at `buffers + 0`, `count * 16`
/// bytes each (allocator slot `8`).
pub fn hk_kd_tree_building_utils_deallocate_buffers(e: &mut Engine, buffers: Ptr, count: i32) {
    if count > 0 {
        let second = e.mem.u32(buffers.addr() + 4);
        let bytes = (count as u32) << 4;
        let allocator = heap_allocator(e);
        e.vcall(allocator, 8, &args![second, bytes]);
        let first = e.mem.u32(buffers.addr());
        let allocator = heap_allocator(e);
        e.vcall(allocator, 8, &args![first, bytes]);
    }
}

// Translated from 00d9d880 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkKdTreeBuildingUtils::buildTree` (Xbox PDB), cdecl, seven words: builds the builder
/// context on a stack frame (see the module notes), constructs the sparse writer
/// (`00df3110`), runs the recursive builder (`00df56a0` with the context, the address of
/// `nodes`, `leaf_data`, the address of a word holding 1 and the address of `counter`),
/// stores its answer through `result`, and flushes the writer (`00df3160`). The context
/// gets `context_word` at `+8`, `count` at `+0x10` and the buffers at `+0`. The names of
/// `context_word` and `counter` are descriptions: the exe copies them into the frame.
pub fn hk_kd_tree_building_utils_build_tree(
    e: &mut Engine,
    buffers: Ptr,
    nodes: u32,
    leaf_data: u32,
    context_word: u32,
    count: u32,
    counter: u32,
    result: Ptr,
) {
    e.with_stack(0x180, |e, frame| {
        let f = frame.addr();
        // The root's projected entry (16 bytes at +0x10): low bounds 0, high bounds 0xffff; the
        // two key halves (+0x16, +0x1e) are uninitialised in the exe.
        for offset in [0x10, 0x12, 0x14] {
            e.mem.set_u16(f + offset, 0);
        }
        for offset in [0x18, 0x1a, 0x1c] {
            e.mem.set_u16(f + offset, 0xffff);
        }
        e.mem.set_u32(f + 4, nodes);
        e.mem.set_u32(f + 0x20, buffers.addr());
        e.mem.set_u32(f + 0x24, 0);
        e.mem.set_u32(f + 0x2c, 0);
        e.mem.set_u32(f + 0x30, count);
        e.mem.set_u32(f + 0x28, context_word);
        copy_bounds_constant(e, f + 0x60);
        copy_16(e, f + 0x40, f + 0x10);
        e.mem.set_u32(f + 8, 1);
        e.call(WRITER_CONSTRUCT, &args![f + 0x70]);
        e.mem.set_u32(f + 0x50, f + 0x70);
        e.mem.set_u32(f + 0xc, counter);
        let built = e
            .call(
                BUILD_TREE_RECURSIVE,
                &args![f + 0x20, f + 4, leaf_data, f + 8, f + 0xc],
            )
            .u32();
        e.mem.set_u32(result.addr(), built);
        e.call(WRITER_FLUSH, &args![f + 0x70]);
    });
}

/// Relocates the tree node at `source` to the 8-byte slot at `base + destination`: a node
/// whose first word is non-negative is copied; one with the top bit set holds a relative offset
/// (in bytes, low three bits flags) that is re-based for its new place.
fn move_node(e: &mut Engine, base: u32, destination: u32, source: u32) {
    let word = e.mem.u32(source);
    let target = base.wrapping_add(destination);
    if (word as i32) >= 0 {
        e.mem.set_u32(target, word);
        let second = e.mem.u32(source.wrapping_add(4));
        e.mem.set_u32(target + 4, second);
    } else {
        e.mem.set_u32(target, word);
        let second = e.mem.u32(source.wrapping_add(4));
        e.mem.set_u32(target + 4, second);
        let offset = (word & 0x7fff_fffc)
            .wrapping_add(source)
            .wrapping_sub(base)
            .wrapping_sub(destination);
        let relative = (((offset as i32) >> 3) as u32).wrapping_mul(8);
        e.mem.set_u32(target, relative | (word & 0x8000_0003));
    }
}

// Translated from 00d9d950 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkKdTreeBuildingUtils::completeDistributedBuild` (Xbox PDB), cdecl `(tree, count, bounds,
/// offsets)`: when `count` is at least 8, joins the four sub-trees of a distributed build
/// under the root: the nodes at slots 8, 8 + `offsets[0]`, 8 + `offsets[0]` + `offsets[1]` and
/// 8 + `offsets[0..=2]` (8 bytes each, from the node array at `[tree]`) are moved to the slots at
/// `+0x20`, `+0x28`, `+0x30` and `+0x38` (see [`move_node`]; the first is read from `+0x40`).
/// Then, when `bounds` is not null, stores the largest of its four ints at `tree + 0xc`.
pub fn hk_kd_tree_building_utils_complete_distributed_build(
    e: &mut Engine,
    tree: Ptr,
    count: i32,
    bounds: Ptr,
    offsets: Ptr,
) {
    if count >= 8 {
        let base = e.mem.u32(tree.addr());
        move_node(e, base, 0x20, base.wrapping_add(0x40));
        let mut slot = e.mem.u32(offsets.addr()).wrapping_add(8);
        move_node(e, base, 0x28, base.wrapping_add(slot.wrapping_mul(8)));
        slot = slot.wrapping_add(e.mem.u32(offsets.addr() + 4));
        move_node(e, base, 0x30, base.wrapping_add(slot.wrapping_mul(8)));
        slot = slot.wrapping_add(e.mem.u32(offsets.addr() + 8));
        move_node(e, base, 0x38, base.wrapping_add(slot.wrapping_mul(8)));
    }
    if bounds.addr() != 0 {
        let b = bounds.addr();
        let mut high = e.mem.i32(b + 8);
        if high <= e.mem.i32(b + 0xc) {
            high = e.mem.i32(b + 0xc);
        }
        let mut low = e.mem.i32(b);
        if low <= e.mem.i32(b + 4) {
            low = e.mem.i32(b + 4);
        }
        if low <= high {
            low = high;
        }
        e.mem.set_i32(tree.addr() + 0xc, low);
    }
}

// Translated from 00d9dab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkKdTreeBuildingUtils::projectPrimitives` (Xbox PDB), cdecl `(source, start, end, entries)`:
/// for each primitive index from `start` below `end` asks the source (a virtual object) for
/// its key (slot `0x10`) and then for its projected entry (slot `0x14`, `(key, address of the
/// entry)`), puts the key's low half at `+6` and high half at `+0xe` of the entry, and stores
/// the 16-byte entry at `entries + 16 * index`. The entry buffer is one stack block that
/// keeps what the source left in it between primitives.
pub fn hk_kd_tree_building_utils_project_primitives(
    e: &mut Engine,
    source: Ptr,
    start: i32,
    end: i32,
    entries: Ptr,
) {
    let mut destination = ((start as u32) << 4).wrapping_add(entries.addr());
    let mut index = start;
    if start < end {
        e.with_stack(16, |e, entry| {
            while index < end {
                let key = e.vcall(source.addr(), 0x10, &args![index as u32]).u32();
                e.vcall(source.addr(), 0x14, &args![key, entry]);
                e.mem.set_u16(entry.addr() + 6, key as u16);
                e.mem.set_u16(entry.addr() + 0xe, (key >> 16) as u16);
                index += 1;
                copy_16(e, destination, entry.addr());
                destination = destination.wrapping_add(0x10);
            }
        });
    }
}

// Translated from 00d9db30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkKdTreeBuildingUtils::buildTree_ov2` (Xbox PDB), cdecl `(buffers, job)`: when the job's
/// count (`+0x10`) is positive, builds the tree (`buildTree`) from the job's words: the node
/// array `+0`, the leaf data `+0x1c`, `+4`, the count, `+0x18` and the address of the job's
/// `+0xc` word for the result.
pub fn hk_kd_tree_building_utils_build_tree_ov2(e: &mut Engine, buffers: Ptr, job: Ptr) {
    let j = job.addr();
    let count = e.mem.i32(j + 0x10);
    if count > 0 {
        let counter = e.mem.u32(j + 0x18);
        let leaf_data = e.mem.u32(j + 0x1c);
        let context_word = e.mem.u32(j + 4);
        let nodes = e.mem.u32(j);
        hk_kd_tree_building_utils_build_tree(
            e,
            buffers,
            nodes,
            leaf_data,
            context_word,
            count as u32,
            counter,
            Ptr::new(j + 0xc),
        );
    }
}

/// Allocates a local array of `16 * count` bytes the way an `hkLocalArray` does: asks the
/// allocator (slot `0xc`) with the byte count at `request`, takes the granted bytes divided
/// by 16 (a signed division) as the capacity, or the "owns nothing" word when that is 0 or
/// no block was wanted. Answers `(data, capacity word)`.
fn allocate_scratch_entries(e: &mut Engine, count: i32, request: u32) -> (u32, u32) {
    let allocator = heap_allocator(e);
    if count == 0 {
        return (0, EMPTY_CAPACITY);
    }
    e.mem.set_u32(request, (count as u32) << 4);
    let data = e.vcall(allocator, 0xc, &args![request]).u32();
    let capacity = e.mem.i32(request) / 16;
    (
        data,
        if capacity == 0 {
            EMPTY_CAPACITY
        } else {
            capacity as u32
        },
    )
}

/// Frees a local array's block (allocator slot `0x10`, `(data, bytes)`) when it owns it.
fn free_scratch_entries(e: &mut Engine, data: u32, capacity: u32) {
    let allocator = heap_allocator(e);
    if (capacity as i32) >= 0 {
        e.vcall(
            allocator,
            0x10,
            &args![data, (capacity & CAPACITY_MASK) << 4],
        );
    }
}

// Translated from 00d9db60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkKdTreeBuildingUtils::buildTreeDistributed` (Xbox PDB), cdecl `(source, tree)`: allocates
/// two local entry arrays for the source's primitive count (slot `0xc`), projects the
/// primitives into the first (`projectPrimitives`), and builds the tree. Fewer than 8
/// primitives are built in one piece (`buildTree` with the two buffers and the tree's words);
/// otherwise the root's four sub-trees are built apart: `splitTreeBuilding` (`00df37b0`) cuts
/// the primitives in four, each part gets a quarter of the count at `tree + 0x18` (the last the
/// rest), its node region (starting at slot 8, advanced by each part's rounded-up size), its
/// own sparse writer and recursive build (`00df56a0`); then `completeDistributedBuild` joins
/// them. Both local arrays are freed at the end.
///
/// The exe keeps everything in one 0x344-byte stack frame the callees also address, so this
/// does too (offsets from the frame start: the buffers at `+0x28`, the root entry at `+0x34`,
/// the four 0x50-byte part contexts from `+0xf4`, their nodes at `+0x74`, the offsets at
/// `+0x54`, the sizes at `+0x64`, the results at `+0x94`, the writer at `+0x234`). The words
/// of the part contexts and of the root entry that the exe never writes (padding) are 0.
pub fn hk_kd_tree_building_utils_build_tree_distributed(e: &mut Engine, source: Ptr, tree: Ptr) {
    e.with_stack(0x344, |e, frame| {
        let f = frame.addr();
        let t = tree.addr();
        let root = e.mem.u32(t);
        e.mem.set_u32(f + 0x10, root);
        let count = e.vcall(source.addr(), 0xc, &args![]).i32();
        let (data_a, capacity_a) = allocate_scratch_entries(e, count, f + 0x20);
        e.mem.set_u32(f + 0x14, data_a);
        e.mem.set_u32(f + 0x18, capacity_a);
        let count = e.vcall(source.addr(), 0xc, &args![]).i32();
        let (data_b, capacity_b) = allocate_scratch_entries(e, count, f + 0x1c);
        e.mem.set_u32(f + 0xc, data_b);
        e.mem.set_u32(f + 4, capacity_b);
        e.mem.set_u32(f + 0x28, data_a);
        e.mem.set_u32(f + 0x2c, data_b);
        let primitives = e.vcall(source.addr(), 0xc, &args![]).i32();
        hk_kd_tree_building_utils_project_primitives(e, source, 0, primitives, Ptr::new(data_a));
        // The root entry's low and high bounds (the key halves at +0x3a and +0x42 are padding).
        for offset in [0x34, 0x36, 0x38] {
            e.mem.set_u16(f + offset, 0);
        }
        for offset in [0x3c, 0x3e, 0x40] {
            e.mem.set_u16(f + offset, 0xffff);
        }
        let primitives = e.vcall(source.addr(), 0xc, &args![]).i32();
        if primitives < 8 {
            let first = e.mem.u32(t + 4);
            let counter = e.mem.u32(t + 0x18);
            let leaf_data = e.mem.u32(t + 0x1c);
            let root = e.mem.u32(t);
            let built = e.vcall(source.addr(), 0xc, &args![]).u32();
            hk_kd_tree_building_utils_build_tree(
                e,
                Ptr::new(f + 0x28),
                root,
                leaf_data,
                first,
                built,
                counter,
                Ptr::new(t + 0xc),
            );
            free_scratch_entries(e, data_b, capacity_b);
        } else {
            build_in_four_parts(e, source, tree, f, root, data_b, capacity_b);
        }
        free_scratch_entries(e, data_a, capacity_a);
    });
}

/// The `>= 8` primitives branch of `buildTreeDistributed` (see there).
fn build_in_four_parts(
    e: &mut Engine,
    source: Ptr,
    tree: Ptr,
    f: u32,
    root: u32,
    data_b: u32,
    capacity_b: u32,
) {
    let t = tree.addr();
    let first = e.mem.u32(t + 4);
    // The four part contexts start with their node pointer and a -1 word at +8 and carry the
    // 16-byte constant at +0x40.
    for part in 0..4u32 {
        let context = f + 0xf4 + 0x50 * part;
        e.mem.set_u32(context + 8, 0xffff_ffff);
        e.mem.set_u32(context, f + 0x74 + 8 * part);
        copy_bounds_constant(e, context + 0x40);
    }
    let primitives = e.vcall(source.addr(), 0xc, &args![]).u32();
    e.call(
        SPLIT_TREE_BUILDING,
        &args![
            f + 0x28,
            root,
            first,
            primitives,
            f + 0xf4,
            f + 0x54,
            f + 0x34
        ],
    );
    let size = e.mem.i32(t + 0x18);
    let quarter = size / 4;
    let rest = size
        .wrapping_sub(quarter)
        .wrapping_sub(quarter)
        .wrapping_sub(quarter);
    e.mem.set_i32(f + 0x68, quarter);
    e.mem.set_i32(f + 0x64, quarter);
    e.mem.set_i32(f + 0x6c, quarter);
    e.mem.set_i32(f + 0x70, rest);
    e.mem.set_u32(f + 8, 8);
    for part in 0..4u32 {
        let context = f + 0xf4 + 0x50 * part;
        let node = e.mem.u32(context);
        let word0 = e.mem.u32(node);
        let word1 = e.mem.u32(node + 4);
        e.mem.set_u32(f + 0x4c, word0);
        let sibling = e.mem.u32(context + 4);
        e.mem.set_u32(f + 0xa8, sibling);
        let first = e.mem.u32(t + 4);
        e.mem.set_u32(f + 0xa4, f + 0x4c);
        e.mem.set_u32(f + 0x50, word1);
        let range_start = e.mem.u32(context + 0xc);
        e.mem.set_u32(f + 0xac, first);
        let range_end = e.mem.u32(context + 0x10);
        copy_bounds_constant(e, f + 0xe4);
        e.mem.set_u32(f + 0xb0, range_start);
        e.mem.set_u32(f + 0xb4, range_end);
        copy_16(e, f + 0xc4, context + 0x20);
        e.call(WRITER_CONSTRUCT, &args![f + 0x234]);
        let running = e.mem.u32(f + 8);
        let nodes = e.mem.u32(f + 0x10);
        e.mem.set_u32(f + 0xd4, f + 0x234);
        e.mem
            .set_u32(f + 0x24, nodes.wrapping_add(running.wrapping_mul(8)));
        let part_size = e.mem.u32(f + 0x64 + 4 * part);
        let rounded = (part_size & 1).wrapping_add(part_size);
        let offset = e.mem.u32(f + 0x54 + 4 * part).wrapping_add(rounded);
        e.mem.set_u32(f + 0x54 + 4 * part, offset);
        e.mem.set_u32(f + 8, running.wrapping_add(offset));
        let leaf_data = e.mem.u32(t + 0x1c);
        e.mem.set_u32(f + 0x30, 1);
        let built = e
            .call(
                BUILD_TREE_RECURSIVE,
                &args![f + 0xa4, f + 0x24, leaf_data, f + 0x30, f + 0x64 + 4 * part],
            )
            .u32();
        e.mem.set_u32(f + 0x94 + 4 * part, built.wrapping_add(2));
        e.call(WRITER_FLUSH, &args![f + 0x234]);
    }
    let primitives = e.vcall(source.addr(), 0xc, &args![]).i32();
    hk_kd_tree_building_utils_complete_distributed_build(
        e,
        tree,
        primitives,
        Ptr::new(f + 0x94),
        Ptr::new(f + 0x54),
    );
    free_scratch_entries(e, data_b, capacity_b);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00d9d800,
            hk_kd_tree_building_utils_allocate_buffers(Ptr, i32)
        ),
        entry!(
            0x00d9d840,
            hk_kd_tree_building_utils_deallocate_buffers(Ptr, i32)
        ),
        entry!(
            0x00d9d880,
            hk_kd_tree_building_utils_build_tree(Ptr, u32, u32, u32, u32, u32, Ptr)
        ),
        entry!(
            0x00d9d950,
            hk_kd_tree_building_utils_complete_distributed_build(Ptr, i32, Ptr, Ptr)
        ),
        entry!(
            0x00d9dab0,
            hk_kd_tree_building_utils_project_primitives(Ptr, i32, i32, Ptr)
        ),
        entry!(
            0x00d9db30,
            hk_kd_tree_building_utils_build_tree_ov2(Ptr, Ptr)
        ),
        entry!(
            0x00d9db60,
            hk_kd_tree_building_utils_build_tree_distributed(Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const ROUTER_CELL: u32 = 0x0127_0800;
    const ALLOCATOR_VTABLE: u32 = 0x0128_0000;
    const SOURCE_VTABLE: u32 = 0x0128_1000;

    type Log = Rc<RefCell<Vec<Vec<u32>>>>;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0126_8000, 0x1000);
        e.map(0x0127_0000, 0x1000);
        for (i, word) in [0x1111_1111u32, 0x2222_2222, 0x3333_3333, 0x4444_4444]
            .into_iter()
            .enumerate()
        {
            e.mem.set_u32(BOUNDS_CONSTANT + 4 * i as u32, word);
        }
        e.put_vtable(
            ALLOCATOR_VTABLE,
            &[0, 0x0900_0001, 0x0900_0002, 0x0900_0003, 0x0900_0004],
        );
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, ALLOCATOR_VTABLE);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.mem.set_u32(ROUTER_CELL, router);
        e.register(MEMORY_ROUTER, |e, _| e.mem.u32(ROUTER_CELL).into_ret());
        e.register(0x0900_0001, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(0x0900_0002, |e, a| {
            e.mem.free(a[1]);
            Ret::default()
        });
        e.register(0x0900_0003, |e, a| {
            let bytes = e.mem.u32(a[1]);
            e.mem.alloc(bytes).into_ret()
        });
        e.register(0x0900_0004, |e, a| {
            e.mem.free(a[1]);
            Ret::default()
        });
        e.register(WRITER_CONSTRUCT, |_, _| Ret::default());
        e.register(WRITER_FLUSH, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e
    }

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// A source: `primitives` primitives (slot `0xc`), key `0x00030000 + 2 * index` (slot
    /// `0x10`), and an entry writer (slot `0x14`) that puts `key & 0xffff` and 0x11 in the
    /// first two words.
    fn source(e: &mut Engine, primitives: u32) -> u32 {
        e.put_vtable(
            SOURCE_VTABLE,
            &[0, 0, 0, 0x0900_0010, 0x0900_0011, 0x0900_0012],
        );
        e.mem.set_u32(SOURCE_VTABLE + 0x40, primitives);
        e.register(0x0900_0010, |e, _| {
            e.mem.u32(SOURCE_VTABLE + 0x40).into_ret()
        });
        e.register(0x0900_0011, |_, a| (0x0003_0000 + 2 * a[1]).into_ret());
        e.register(0x0900_0012, |e, a| {
            // (this, key, entry)
            e.mem.set_u16(a[2], a[1] as u16);
            e.mem.set_u16(a[2] + 2, 0x11);
            Ret::default()
        });
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, SOURCE_VTABLE);
        object
    }

    /// Records the words at the context of a recursive-build call (the 0x50 bytes from the
    /// context, plus what its first word points to) and answers `answer + call number`.
    fn recorder(e: &mut Engine, log: &Log, answer: u32) {
        let log = Rc::clone(log);
        let mut calls = 0;
        e.register_double(BUILD_TREE_RECURSIVE, move |e, a| {
            let mut words: Vec<u32> = a.to_vec();
            words.extend((0..0x14).map(|i| e.mem.u32(a[0] + 4 * i)));
            let node = e.mem.u32(a[0]);
            words.extend([e.mem.u32(node), e.mem.u32(node + 4)]);
            words.extend([e.mem.u32(a[1]), e.mem.u32(a[3]), e.mem.u32(a[4])]);
            log.borrow_mut().push(words);
            calls += 1;
            (answer + calls - 1).into_ret()
        });
    }

    #[test]
    fn allocate_buffers_takes_two_blocks_of_sixteen_bytes_per_entry() {
        let mut e = engine();
        let buffers = e.mem.alloc(8);
        e.call(0x00d9_d800, &args![buffers, 5u32]);
        let (first, second) = (e.mem.u32(buffers), e.mem.u32(buffers + 4));
        assert_ne!(first, second);
        assert_eq!(e.mem.block_size(first), Some(80));
        assert_eq!(e.mem.block_size(second), Some(80));
        assert_eq!(calls(&e, 0x0900_0001).len(), 2);
        assert_eq!(calls(&e, 0x0900_0001)[1][1], 80);
    }

    #[test]
    fn deallocate_buffers_frees_the_second_block_then_the_first() {
        let mut e = engine();
        let buffers = e.mem.alloc(8);
        e.call(0x00d9_d800, &args![buffers, 4u32]);
        let (first, second) = (e.mem.u32(buffers), e.mem.u32(buffers + 4));
        e.call_log = Some(vec![]);
        e.call(0x00d9_d840, &args![buffers, 4u32]);
        let frees = calls(&e, 0x0900_0002);
        assert_eq!(frees.len(), 2);
        assert_eq!(frees[0][1..], [second, 64]);
        assert_eq!(frees[1][1..], [first, 64]);
        assert_eq!(e.mem.block_size(first), None);
        // Nothing to free for a count that is not positive.
        e.call_log = Some(vec![]);
        e.call(0x00d9_d840, &args![buffers, 0u32]);
        e.call(0x00d9_d840, &args![buffers, (-3i32) as u32]);
        assert!(calls(&e, 0x0900_0002).is_empty());
    }

    #[test]
    fn build_tree_builds_the_context_and_runs_the_writer_around_the_recursion() {
        let mut e = engine();
        let log: Log = Rc::default();
        recorder(&mut e, &log, 40);
        let buffers = e.mem.alloc(8);
        let result = e.mem.alloc(4);
        let nodes = e.mem.alloc(16);
        e.call(
            0x00d9_d880,
            &args![buffers, nodes, 0xaaa0u32, 0xbbb0u32, 9u32, 0xccc0u32, result],
        );
        assert_eq!(e.mem.u32(result), 40);
        let log = log.borrow();
        let call = &log[0];
        // (context, nodes address, leaf data, address of 1, address of counter)
        assert_eq!(call[2], 0xaaa0);
        // Words read from the context while it was live.
        let words = &call[5..5 + 0x14];
        assert_eq!(words[0], buffers);
        assert_eq!(words[1], 0);
        assert_eq!(words[2], 0xbbb0);
        assert_eq!(words[3], 0);
        assert_eq!(words[4], 9);
        // The root entry at +0x20: low bounds 0, high bounds 0xffff.
        assert_eq!(words[8] & 0xffff, 0);
        assert_eq!(words[10] >> 16, 0xffff);
        assert_eq!(words[10] & 0xffff, 0xffff);
        assert_eq!(words[11] & 0xffff, 0xffff);
        // The writer's address and the constant.
        let writer = calls(&e, WRITER_CONSTRUCT)[0][0];
        assert_eq!(words[12], writer);
        assert_eq!(
            words[16..20],
            [0x1111_1111, 0x2222_2222, 0x3333_3333, 0x4444_4444]
        );
        // nodes pointer cell, the word 1 and the counter cell.
        assert_eq!(call[5 + 0x14 + 2], nodes);
        assert_eq!(call[5 + 0x14 + 3], 1);
        assert_eq!(call[5 + 0x14 + 4], 0xccc0);
        assert_eq!(calls(&e, WRITER_FLUSH), vec![vec![writer]]);
    }

    #[test]
    fn complete_distributed_build_moves_the_four_sub_trees_under_the_root() {
        let mut e = engine();
        let base = e.mem.alloc(0x200);
        let tree = e.mem.alloc(0x20);
        e.mem.set_u32(tree, base);
        let offsets = e.mem.alloc(12);
        for (i, offset) in [2u32, 3, 1].into_iter().enumerate() {
            e.mem.set_u32(offsets + 4 * i as u32, offset);
        }
        let put = |e: &mut Engine, at: u32, first: u32, second: u32| {
            e.mem.set_u32(base + at, first);
            e.mem.set_u32(base + at + 4, second);
        };
        // Slots: 8 (+0x40), 10 (+0x50), 13 (+0x68), 14 (+0x70).
        put(&mut e, 0x40, 0x8000_0021, 0x0a);
        put(&mut e, 0x50, 6, 0x66);
        put(&mut e, 0x68, 0x8000_0100, 0x0b);
        put(&mut e, 0x70, 9, 0x99);
        let bounds = e.mem.alloc(16);
        for (i, value) in [3i32, 9, 4, 2].into_iter().enumerate() {
            e.mem.set_i32(bounds + 4 * i as u32, value);
        }
        e.call(0x00d9_d950, &args![tree, 8u32, bounds, offsets]);
        let at = |e: &Engine, offset: u32| (e.mem.u32(base + offset), e.mem.u32(base + offset + 4));
        // Negative nodes hold relative offsets that are re-based for the move.
        assert_eq!(at(&e, 0x20), (0x8000_0041, 0x0a));
        assert_eq!(at(&e, 0x28), (6, 0x66));
        assert_eq!(at(&e, 0x30), (0x8000_0138, 0x0b));
        assert_eq!(at(&e, 0x38), (9, 0x99));
        assert_eq!(e.mem.u32(tree + 0xc), 9);
    }

    #[test]
    fn complete_distributed_build_does_nothing_for_small_counts_and_a_null_bounds() {
        let mut e = engine();
        let base = e.mem.alloc(0x200);
        let tree = e.mem.alloc(0x20);
        e.mem.set_u32(tree, base);
        e.mem.set_u32(tree + 0xc, 0x55);
        e.mem.set_u32(base + 0x20, 0x1234);
        e.call(0x00d9_d950, &args![tree, 7u32, 0u32, 0u32]);
        assert_eq!(e.mem.u32(base + 0x20), 0x1234);
        assert_eq!(e.mem.u32(tree + 0xc), 0x55);
        // The largest of the four bounds (signed), whichever slot it is in.
        let bounds = e.mem.alloc(16);
        for (i, value) in [-5i32, -9, -2, -7].into_iter().enumerate() {
            e.mem.set_i32(bounds + 4 * i as u32, value);
        }
        e.call(0x00d9_d950, &args![tree, 0u32, bounds, 0u32]);
        assert_eq!(e.mem.i32(tree + 0xc), -2);
        e.mem.set_i32(bounds, 12);
        e.call(0x00d9_d950, &args![tree, 0u32, bounds, 0u32]);
        assert_eq!(e.mem.i32(tree + 0xc), 12);
    }

    #[test]
    fn project_primitives_stores_entries_with_the_key_halves() {
        let mut e = engine();
        let object = source(&mut e, 0);
        let entries = e.mem.alloc(48);
        e.call(0x00d9_dab0, &args![object, 1u32, 3u32, entries]);
        for index in 1..3u32 {
            let entry = entries + 16 * index;
            let key = 0x0003_0000 + 2 * index;
            assert_eq!(e.mem.u16(entry), key as u16);
            assert_eq!(e.mem.u16(entry + 2), 0x11);
            assert_eq!(e.mem.u16(entry + 6), key as u16);
            assert_eq!(e.mem.u16(entry + 0xe), 3);
        }
        // Slot 0 was not asked for.
        assert_eq!(e.mem.u32(entries), 0);
        assert_eq!(calls(&e, 0x0900_0011).len(), 2);
        assert_eq!(calls(&e, 0x0900_0012).len(), 2);
        // An empty range asks for nothing.
        e.call_log = Some(vec![]);
        e.call(0x00d9_dab0, &args![object, 3u32, 3u32, entries]);
        assert!(calls(&e, 0x0900_0011).is_empty());
    }

    #[test]
    fn build_tree_ov2_builds_only_a_non_empty_job() {
        let mut e = engine();
        let log: Log = Rc::default();
        recorder(&mut e, &log, 5);
        let job = e.mem.alloc(0x20);
        let nodes = e.mem.alloc(16);
        let buffers = e.mem.alloc(8);
        e.mem.set_u32(job, nodes);
        e.mem.set_u32(job + 4, 0xb4);
        e.mem.set_u32(job + 0x10, 0);
        e.call(0x00d9_db30, &args![buffers, job]);
        assert!(log.borrow().is_empty());
        e.mem.set_u32(job + 0x10, 6);
        e.mem.set_u32(job + 0x18, 0xc8);
        e.mem.set_u32(job + 0x1c, 0xd8);
        e.call(0x00d9_db30, &args![buffers, job]);
        assert_eq!(log.borrow().len(), 1);
        assert_eq!(log.borrow()[0][2], 0xd8);
        // The context word and the count reach the context; the result lands in the job.
        assert_eq!(log.borrow()[0][5 + 2], 0xb4);
        assert_eq!(log.borrow()[0][5 + 4], 6);
        assert_eq!(e.mem.u32(job + 0xc), 5);
    }

    #[test]
    fn build_tree_distributed_builds_a_small_tree_in_one_piece() {
        let mut e = engine();
        let log: Log = Rc::default();
        recorder(&mut e, &log, 77);
        let object = source(&mut e, 3);
        let tree = e.mem.alloc(0x20);
        let nodes = e.mem.alloc(0x40);
        e.mem.set_u32(tree, nodes);
        e.mem.set_u32(tree + 4, 0x44);
        e.mem.set_u32(tree + 0x18, 0x66);
        e.mem.set_u32(tree + 0x1c, 0x88);
        e.call(0x00d9_db60, &args![object, tree]);
        assert_eq!(e.mem.u32(tree + 0xc), 77);
        let log = log.borrow();
        assert_eq!(log.len(), 1);
        // leaf data, the context word (the tree's +4) and the count (the source's 3).
        assert_eq!(log[0][2], 0x88);
        assert_eq!(log[0][5 + 2], 0x44);
        assert_eq!(log[0][5 + 4], 3);
        // The buffers word points at the two local arrays: the first holds the projected entries.
        let buffers = log[0][5];
        assert_ne!(buffers, 0);
        // Both local arrays (48 bytes each) were allocated and freed.
        assert_eq!(calls(&e, 0x0900_0003).len(), 2);
        assert_eq!(calls(&e, 0x0900_0004).len(), 2);
        assert_eq!(calls(&e, 0x0900_0004)[0][2], 48);
        assert_eq!(calls(&e, 0x0900_0004)[1][2], 48);
    }

    #[test]
    fn build_tree_distributed_with_no_primitives_has_nothing_to_free() {
        let mut e = engine();
        let log: Log = Rc::default();
        recorder(&mut e, &log, 1);
        let object = source(&mut e, 0);
        let tree = e.mem.alloc(0x20);
        let nodes = e.mem.alloc(0x40);
        e.mem.set_u32(tree, nodes);
        e.call(0x00d9_db60, &args![object, tree]);
        assert!(calls(&e, 0x0900_0003).is_empty());
        assert!(calls(&e, 0x0900_0004).is_empty());
        assert_eq!(log.borrow().len(), 1);
        assert_eq!(log.borrow()[0][5 + 4], 0);
    }

    #[test]
    fn build_tree_distributed_cuts_a_large_tree_in_four_parts() {
        let mut e = engine();
        let log: Log = Rc::default();
        recorder(&mut e, &log, 10);
        let object = source(&mut e, 8);
        let tree = e.mem.alloc(0x20);
        let nodes = e.mem.alloc(0x400);
        e.mem.set_u32(tree, nodes);
        e.mem.set_u32(tree + 4, 0x44);
        e.mem.set_u32(tree + 0x18, 10);
        e.mem.set_u32(tree + 0x1c, 0x88);
        // splitTreeBuilding fills each part's context: its node (two words), a sibling word,
        // the primitive range and its bounds, and the four offsets.
        e.register(SPLIT_TREE_BUILDING, |e, a| {
            let (contexts, offsets) = (a[4], a[5]);
            for part in 0..4u32 {
                let context = contexts + 0x50 * part;
                let node = e.mem.u32(context);
                e.mem.set_u32(node, 0x100 + part);
                e.mem.set_u32(node + 4, 0x200 + part);
                e.mem.set_u32(context + 4, 0x300 + part);
                e.mem.set_u32(context + 0xc, 10 * part);
                e.mem.set_u32(context + 0x10, 10 * part + 10);
                e.mem.set_u32(context + 0x20, 0x400 + part);
                e.mem.set_u32(offsets + 4 * part, 1);
            }
            Ret::default()
        });
        e.call(0x00d9_db60, &args![object, tree]);
        let split = calls(&e, SPLIT_TREE_BUILDING);
        assert_eq!(split.len(), 1);
        // (buffers, nodes, the tree's +4, primitives, contexts, offsets, root entry)
        assert_eq!(split[0][1..4], [nodes, 0x44, 8]);
        let log = log.borrow();
        assert_eq!(log.len(), 4);
        // Each part: its node words and sibling copied into the call's context, the tree's +4,
        // the range, the leaf data and the counter holding the part's size.
        let sizes = [2u32, 2, 2, 4];
        let mut running = 8u32;
        for (part, call) in log.iter().enumerate() {
            let part_u = part as u32;
            assert_eq!(call[2], 0x88);
            assert_eq!(call[5 + 1], 0x300 + part_u);
            assert_eq!(call[5 + 2], 0x44);
            assert_eq!(call[5 + 3], 10 * part_u);
            assert_eq!(call[5 + 4], 10 * part_u + 10);
            assert_eq!(call[5 + 8], 0x400 + part_u);
            assert_eq!(call[5 + 0x14], 0x100 + part_u);
            assert_eq!(call[5 + 0x14 + 1], 0x200 + part_u);
            // The node array position of the part: the root plus the running slot * 8.
            assert_eq!(call[5 + 0x14 + 2], nodes + running * 8);
            assert_eq!(call[5 + 0x14 + 3], 1);
            assert_eq!(call[5 + 0x14 + 4], sizes[part]);
            // The offset counts the part's size rounded up to even, plus the 1 split gave.
            running += 1 + sizes[part] + (sizes[part] & 1);
        }
        // completeDistributedBuild got the answers + 2 (12..15): the largest lands at +0xc.
        assert_eq!(e.mem.u32(tree + 0xc), 15);
        assert_eq!(calls(&e, WRITER_CONSTRUCT).len(), 4);
        assert_eq!(calls(&e, WRITER_FLUSH).len(), 4);
        // The two local arrays (8 entries of 16 bytes) were freed.
        let frees = calls(&e, 0x0900_0004);
        assert_eq!(frees.len(), 2);
        assert_eq!(frees[0][2], 128);
    }
}

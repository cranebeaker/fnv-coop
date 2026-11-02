//! `Havok SDK/hklargeblockallocator.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The Xbox PDB has no entry for `hkLargeBlockAllocator`, so the offsets are
//! described where the functions use them.

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkLargeBlockAllocator`'s virtual table (`010d7c34`; the class name is
/// confirmed by the RTTI record before it).
const VTABLE: u32 = 0x010d_7c34;

// Translated from 00dd7f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack arguments): the state set-up of
/// the large block allocator. Clears the words at `+8` and `+0xc`, `+0x14`,
/// `+0x18` to `+0x94`, `+0x9c`, `+0xbc` and `+0xc0`; stores `3` at `+0xa0`; points
/// `+0x10` at `this + 0x9c` and `+0xa8` at `this + 0xa4`, and the word at `+0xa4`
/// at itself (a list head linked to itself).
pub fn fn_00dd7f10(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    for offset in [0xc0, 0xbc, 0x08, 0x0c] {
        e.mem.set_u32(base + offset, 0);
    }
    for offset in (0x18..=0x94).step_by(4) {
        e.mem.set_u32(base + offset, 0);
    }
    e.mem.set_u32(base + 0x9c, 0);
    e.mem.set_u32(base + 0x14, 0);
    e.mem.set_u32(base + 0xa0, 3);
    e.mem.set_u32(base + 0x10, base + 0x9c);
    e.mem.set_u32(base + 0xa8, base + 0xa4);
    e.mem.set_u32(base + 0xa4, base + 0xa4);
}

// Translated from 00dd8d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkLargeBlockAllocator::hkLargeBlockAllocator` (RTTI-confirmed class; the map
/// gives the address no name), `thiscall`, `RET 4`: stores the virtual table, clears
/// `+4`, stores `parent` (the memory allocator it takes its memory from) at
/// `+0x98` and sets the rest up (`00dd7f10`). Returns `this`.
pub fn hk_large_block_allocator_hk_large_block_allocator(
    e: &mut Engine,
    this: Ptr,
    parent: u32,
) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE);
    e.mem.set_u32(base + 4, 0);
    e.mem.set_u32(base + 0x98, parent);
    fn_00dd7f10(e, this);
    this
}

/// `operator delete` (`00401030`, cdecl `(block)`).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// The virtual table of the server base class (`010d7c10`), which the constructor and the
/// destructor of the allocator store before and after their own.
const BASE_VTABLE: u32 = 0x010d_7c10;

// The allocator's words, from the code: `+8` bit mask of the used tree bins, `+4` the optional
// fallback object (virtual slot `4` makes room, `8` gives it up), `+0x10` the top chunk and
// `+0x14` its size, `+0x18 + 4 * i` the 32 tree bins, `+0x98` the server (virtual slot `4`
// answers a flag through a pointer, `8` rounds a request, `0xc` allocates `(rounded, address
// of the granted size)`, `0x10` frees `(block, size)`, `0x14` resizes, `0x24` and `0x28` are
// the critical allocate and free), `+0x9c` the empty-top marker, `+0xa4` the sentinel of the
// ring of server segments (`+0xa8` its first), `+0xbc` the bytes handed out including
// headers and `+0xc0` without. A segment: `+0` previous, `+4` next, `+8` 0, `+0xc` its size,
// `+0x10` its first chunk, `+0x14` its end. A chunk: `+0` the previous chunk's size (when that
// is free), `+4` its size with bit 0 "previous chunk in use" and bit 1 "this chunk in use";
// free chunks hold the tree links (`+8` previous and `+0xc` next of the same size, `+0x10` and
// `+0x14` the two children, `+0x18` the parent, `+0x1c` the bin).

/// Reads a word.
fn word(e: &Engine, address: u32) -> u32 {
    e.mem.u32(address)
}

/// Writes a word.
fn put(e: &mut Engine, address: u32, value: u32) {
    e.mem.set_u32(address, value);
}

/// The address of tree bin `index`.
fn bin_address(base: u32, index: u32) -> u32 {
    base.wrapping_add(0x18).wrapping_add(index.wrapping_mul(4))
}

/// Frees every server segment, first to last, through server slot `0x10`.
fn free_segments(e: &mut Engine, base: u32) {
    let sentinel = base + 0xa4;
    let mut segment = word(e, base + 0xa8);
    if segment != sentinel {
        loop {
            let size = word(e, segment + 0xc);
            let server = word(e, base + 0x98);
            let next = word(e, segment + 4);
            e.vcall(server, 0x10, &args![segment, size]);
            segment = next;
            if next == sentinel {
                break;
            }
        }
    }
}

// Translated from 00dd7de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkFreeListAllocatorServer::_scalar_deleting_destructor_` (the engine map's name for this
/// body, `thiscall`, `RET 4`): stores the base virtual table (`010d7c10`) and, when bit 0 of
/// `flags` is set, deletes the object (`00401030`). Answers `this`.
pub fn hk_free_list_allocator_server_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u8,
) -> Ptr {
    e.mem.set_u32(this.addr(), BASE_VTABLE);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00dd7e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkLargeBlockAllocator::_computeTreeIndex` (Xbox PDB), cdecl `(size)`: the tree bin of a
/// chunk size: 0 below 256, 31 from 16 MiB, else twice the position of the top set bit of
/// `size >> 8` plus 14 plus the next bit of `size`.
pub fn hk_large_block_allocator_compute_tree_index(_e: &mut Engine, size: u32) -> i32 {
    let high = size >> 8;
    if high == 0 {
        return 0;
    }
    if high > 0xffff {
        return 0x1f;
    }
    let first = (high.wrapping_add(0xffff_ff00) >> 16) & 8;
    let mut value = high << first;
    let second = (value.wrapping_add(0xffff_f000) >> 16) & 4;
    value <<= second;
    let third = (value.wrapping_add(0xffff_c000) >> 16) & 2;
    value <<= third;
    let position = (value >> 15)
        .wrapping_sub(third)
        .wrapping_sub(first.wrapping_add(second))
        .wrapping_add(14);
    let next_bit = (size >> (position.wrapping_add(7) & 31)) & 1;
    next_bit.wrapping_add(position.wrapping_mul(2)) as i32
}

// Translated from 00dd7e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl `(value)`): the bit-scan helper the allocator calls on an
/// isolated bit (`-mask & mask`): the base-2 logarithm of a power of two.
pub fn fn_00dd7e70(_e: &mut Engine, value: u32) -> u32 {
    let mut v = value.wrapping_sub(1);
    let mut sum = (v >> 12) & 0x10;
    v >>= sum;
    let step = (v >> 5) & 8;
    v >>= step;
    sum += step;
    let step = (v >> 2) & 4;
    v >>= step;
    sum += step;
    let step = (v >> 1) & 2;
    v >>= step;
    sum += step;
    let last = (v >> 1) & 1;
    v >>= last;
    v.wrapping_add(sum).wrapping_add(last)
}

// Translated from 00dd7ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkLargeBlockAllocator::~hkLargeBlockAllocator` (Xbox PDB), `fastcall`: stores the
/// allocator's virtual table, gives every server segment back (server slot `0x10`, `(segment,
/// its size)`), and stores the base virtual table.
pub fn hk_large_block_allocator_destructor(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE);
    free_segments(e, this.addr());
    e.mem.set_u32(this.addr(), BASE_VTABLE);
}

// Translated from 00dd7fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`fastcall`): gives every server segment back (as the destructor
/// does) and then resets the allocator's state (`00dd7f10`).
pub fn fn_00dd7fc0(e: &mut Engine, this: Ptr) {
    free_segments(e, this.addr());
    fn_00dd7f10(e, this);
}

// Translated from 00dd8010 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkLargeBlockAllocator::_insertLargeChunk` (Xbox PDB), `thiscall`, `RET 8`: puts the free
/// chunk `chunk` of `size` bytes into its tree bin (index from `_computeTreeIndex`). An empty
/// bin takes it as the root and sets its bit; otherwise the tree is walked by the bits of the
/// size: a chunk of the same size joins that node's ring, else it becomes a child where the
/// walk ends.
pub fn hk_large_block_allocator_insert_large_chunk(
    e: &mut Engine,
    this: Ptr,
    chunk: Ptr,
    size: u32,
) {
    let base = this.addr();
    let node = chunk.addr();
    let index = hk_large_block_allocator_compute_tree_index(e, size) as u32;
    put(e, node + 0x14, 0);
    put(e, node + 0x10, 0);
    let bit = 1u32.wrapping_shl(index);
    put(e, node + 0x1c, index);
    let bitmap = word(e, base + 8);
    let bin = bin_address(base, index);
    if bit & bitmap == 0 {
        put(e, base + 8, bitmap | bit);
        put(e, bin, node);
        put(e, node + 0x18, bin);
        put(e, node + 8, node);
        put(e, node + 0xc, node);
        return;
    }
    let mut parent = word(e, bin);
    let shift = if index == 0x1f {
        0
    } else {
        0x19 - (index >> 1)
    };
    let mut key = size.wrapping_shl(shift);
    loop {
        if word(e, parent + 4) & 0xffff_fffc == size {
            let previous = word(e, parent + 8);
            put(e, previous + 0xc, node);
            put(e, parent + 8, node);
            put(e, node + 0xc, parent);
            put(e, node + 8, previous);
            put(e, node + 0x18, 0);
            return;
        }
        let slot = parent
            .wrapping_add(0x10)
            .wrapping_add((key >> 31).wrapping_mul(4));
        let child = word(e, slot);
        key = key.wrapping_add(key);
        if child == 0 {
            put(e, slot, node);
            put(e, node + 0x18, parent);
            put(e, node + 8, node);
            put(e, node + 0xc, node);
            return;
        }
        parent = child;
    }
}

// Translated from 00dd80d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkLargeBlockAllocator::_unlinkLargeChunk` (Xbox PDB), `thiscall`, `RET 4`: takes the free
/// chunk `chunk` out of the tree. A chunk that shares its size with others leaves its ring (the
/// next one takes its place in the tree when it was the tree node); the only chunk of its size
/// is replaced in the tree by its deepest descendant (found by following the second child
/// first), which is cut out of its own parent. The replacement inherits the parent and the
/// children; a chunk that was a bin's root empties the bin (clearing its bit when nothing
/// replaces it).
pub fn hk_large_block_allocator_unlink_large_chunk(e: &mut Engine, this: Ptr, chunk: Ptr) {
    let base = this.addr();
    let node = chunk.addr();
    let mut replacement = word(e, node + 0xc);
    let parent = word(e, node + 0x18);
    if replacement == node {
        let mut slot = node + 0x14;
        let mut current = word(e, slot);
        let mut has_child = true;
        if current == 0 {
            slot = node + 0x10;
            current = word(e, slot);
            if current == 0 {
                replacement = 0;
                has_child = false;
            }
        }
        if has_child {
            loop {
                if word(e, current + 0x14) != 0 {
                    slot = current + 0x14;
                } else if word(e, current + 0x10) != 0 {
                    slot = current + 0x10;
                } else {
                    break;
                }
                current = word(e, slot);
            }
            put(e, slot, 0);
            replacement = current;
        }
    } else {
        let previous = word(e, node + 8);
        put(e, previous + 0xc, replacement);
        put(e, replacement + 8, previous);
    }
    if parent == 0 {
        return;
    }
    let index = word(e, node + 0x1c);
    let bin = bin_address(base, index);
    if node == word(e, bin) {
        put(e, bin, replacement);
        if replacement == 0 {
            let bitmap = word(e, base + 8) & !1u32.wrapping_shl(index);
            put(e, base + 8, bitmap);
            return;
        }
    } else {
        if word(e, parent + 0x10) == node {
            put(e, parent + 0x10, replacement);
        } else {
            put(e, parent + 0x14, replacement);
        }
        if replacement == 0 {
            return;
        }
    }
    put(e, replacement + 0x18, parent);
    let first_child = word(e, node + 0x10);
    if first_child != 0 {
        put(e, replacement + 0x10, first_child);
        put(e, first_child + 0x18, replacement);
    }
    let second_child = word(e, node + 0x14);
    if second_child != 0 {
        put(e, replacement + 0x14, second_child);
        put(e, second_child + 0x18, replacement);
    }
}

/// The steps of `_allocLarge`'s search (the exe's labels `81d0`, `820e`, `8218`, `8245`).
enum Search {
    /// Walking the bin's tree from `node` by the bits of the size.
    Tree,
    /// The bin walk ended (with `node` the best lower candidate, or 0).
    TreeEnd,
    /// Looking for the next used bin above.
    Bitmap,
    /// Scanning a subtree for a smaller fit.
    Scan,
    /// Nothing more to look at.
    Done,
    /// A chunk was chosen.
    Found,
}

// Translated from 00dd8180 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkLargeBlockAllocator::_allocLarge` (Xbox PDB), `thiscall`, `RET 4`: best-fit search of
/// the free tree for a chunk of `size` bytes. It walks the tree of the size's bin by the
/// size's bits keeping the chunk with the smallest surplus (an exact fit ends the search), then
/// the subtrees the walk passed and, failing that, the first used bin above; the chosen chunk
/// is unlinked and handed out: whole when the surplus is under 256 bytes (the next chunk's
/// "previous in use" bit is set), else split with the surplus put back (`_insertLargeChunk`).
/// The two counters at `+0xbc` and `+0xc0` grow. Answers the user address (`chunk + 0x10`) or 0.
pub fn hk_large_block_allocator_alloc_large(e: &mut Engine, this: Ptr, size: u32) -> u32 {
    let base = this.addr();
    let mut best = 0u32;
    let mut surplus = size.wrapping_neg();
    let index = hk_large_block_allocator_compute_tree_index(e, size) as u32;
    let mut node = word(e, bin_address(base, index));
    let mut lower = 0u32;
    let mut key = 0u32;
    let mut step = Search::Bitmap;
    if node != 0 {
        let shift = if index == 0x1f {
            0
        } else {
            0x19 - (index >> 1)
        };
        key = size.wrapping_shl(shift);
        step = Search::Tree;
    }
    loop {
        match step {
            Search::Tree => {
                let candidate = (word(e, node + 4) & 0xffff_fffc).wrapping_sub(size);
                if candidate < surplus {
                    best = node;
                    surplus = candidate;
                    if candidate == 0 {
                        step = Search::TreeEnd;
                        continue;
                    }
                }
                let right = word(e, node + 0x14);
                let next = word(e, node.wrapping_add((key >> 31).wrapping_mul(4)) + 0x10);
                if right != 0 && right != next {
                    lower = right;
                }
                if next != 0 {
                    key = key.wrapping_add(key);
                    node = next;
                    continue;
                }
                node = lower;
                step = Search::TreeEnd;
            }
            Search::TreeEnd => {
                step = if node != 0 {
                    Search::Scan
                } else if best != 0 {
                    Search::Found
                } else {
                    Search::Bitmap
                };
            }
            Search::Bitmap => {
                let bit = 1u32.wrapping_shl(index);
                let doubled = bit.wrapping_add(bit);
                let mask = (doubled.wrapping_neg() | doubled) & word(e, base + 8);
                node = 0;
                if mask != 0 {
                    let lowest = mask.wrapping_neg() & mask;
                    let bin = fn_00dd7e70(e, lowest);
                    node = word(e, bin_address(base, bin));
                }
                step = if node != 0 {
                    Search::Scan
                } else {
                    Search::Done
                };
            }
            Search::Scan => {
                let candidate = (word(e, node + 4) & 0xffff_fffc).wrapping_sub(size);
                if candidate < surplus {
                    surplus = candidate;
                    best = node;
                }
                let first = word(e, node + 0x10);
                node = if first != 0 {
                    first
                } else {
                    word(e, node + 0x14)
                };
                step = if node != 0 {
                    Search::Scan
                } else {
                    Search::Done
                };
            }
            Search::Done => {
                if best == 0 {
                    return 0;
                }
                step = Search::Found;
            }
            Search::Found => {
                let chunk = best;
                let remainder = chunk.wrapping_add(size);
                hk_large_block_allocator_unlink_large_chunk(e, this, Ptr::new(chunk));
                if surplus < 0x100 {
                    let whole = surplus.wrapping_add(size);
                    put(e, chunk + 4, whole | 3);
                    let next_header = whole.wrapping_add(chunk) + 4;
                    let header = word(e, next_header) | 1;
                    put(e, next_header, header);
                    let handed = word(e, base + 0xbc).wrapping_add(whole);
                    put(e, base + 0xbc, handed);
                    let payload = word(e, base + 0xc0).wrapping_add(whole.wrapping_sub(0x10));
                    put(e, base + 0xc0, payload);
                    return chunk + 0x10;
                }
                put(e, chunk + 4, size | 3);
                let handed = word(e, base + 0xbc).wrapping_add(size);
                put(e, base + 0xbc, handed);
                let payload = word(e, base + 0xc0).wrapping_add(size.wrapping_sub(0x10));
                put(e, base + 0xc0, payload);
                put(e, remainder + 4, surplus | 1);
                put(e, surplus.wrapping_add(remainder), surplus);
                hk_large_block_allocator_insert_large_chunk(e, this, Ptr::new(remainder), surplus);
                return chunk + 0x10;
            }
        }
    }
}

// Translated from 00dd8310 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, `RET 8`: `(node, largest)`): the largest chunk size in the
/// subtree below `node` (following the second child, else the first), starting from `largest`.
pub fn fn_00dd8310(e: &mut Engine, node: Ptr, largest: u32) -> u32 {
    let mut largest = largest;
    let mut node = node.addr();
    while node != 0 {
        let size = word(e, node + 4) & 0xffff_fffc;
        if size > largest {
            largest = size;
        }
        let second = word(e, node + 0x14);
        node = if second != 0 {
            second
        } else {
            word(e, node + 0x10)
        };
    }
    largest
}

// Translated from 00dd8350 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`fastcall`): the largest free chunk size: the top chunk's size, or
/// the largest chunk of the highest used bin when that is bigger (`00dd8310`).
pub fn fn_00dd8350(e: &mut Engine, this: Ptr) -> u32 {
    let base = this.addr();
    let top = word(e, base + 0x14);
    let first = hk_large_block_allocator_compute_tree_index(e, top);
    let mut bin = 0x1fi32;
    if first <= 0x1f {
        let mut slot = base + 0x94;
        loop {
            let root = word(e, slot);
            if root != 0 {
                return fn_00dd8310(e, Ptr::new(root), top);
            }
            bin -= 1;
            slot = slot.wrapping_sub(4);
            if bin < first {
                break;
            }
        }
    }
    top
}

// Translated from 00dd83a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`fastcall`): writes the top chunk's header (its size with the
/// "previous in use" bit) and its footer (its size, at the chunk's end).
pub fn fn_00dd83a0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let top = word(e, base + 0x10);
    let size = word(e, base + 0x14);
    put(e, top + 4, size | 1);
    let size = word(e, base + 0x14);
    let top = word(e, base + 0x10);
    put(e, size.wrapping_add(top), size);
}

// Translated from 00dd83c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): carves `size` bytes off the front of the top
/// chunk: counts them in `+0xbc` and (less the header) `+0xc0`, moves the top on, writes the
/// new top's footer and header and the handed-out chunk's header (`size | 3`). Answers the
/// user address (`chunk + 0x10`).
pub fn fn_00dd83c0(e: &mut Engine, this: Ptr, size: u32) -> u32 {
    let base = this.addr();
    let top_size = word(e, base + 0x14).wrapping_sub(size);
    put(e, base + 0x14, top_size);
    let handed = word(e, base + 0xbc).wrapping_add(size);
    put(e, base + 0xbc, handed);
    let chunk = word(e, base + 0x10);
    let payload = word(e, base + 0xc0).wrapping_add(size.wrapping_sub(0x10));
    put(e, base + 0xc0, payload);
    let top_size = word(e, base + 0x14);
    let new_top = chunk.wrapping_add(size);
    put(e, base + 0x10, new_top);
    put(e, new_top, size);
    put(e, new_top + 4, top_size | 1);
    put(e, chunk + 4, size | 3);
    chunk + 0x10
}

// Translated from 00dd8400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkLargeBlockAllocator::_resizeSingleBlockServerPage` (Xbox PDB), `thiscall`, `RET 8`
/// `(out, new_size)`: asks the server (slot `0x14`) to resize the first segment to `new_size`
/// bytes, passing it five words: the address of a cell for its answer, the segment, the
/// segment's size, `new_size` and the address of a cell that holds `this` until the server
/// stores the size it granted there. When the server's answer (a byte read through the
/// pointer it returns) is 0 the byte at `out` is set to 0; otherwise the granted size grows the
/// segment and the top chunk by the difference rounded down to 16, the fence header after the
/// top is rewritten, and the byte at `out` is set to 1. Answers `out`.
pub fn hk_large_block_allocator_resize_single_block_server_page(
    e: &mut Engine,
    this: Ptr,
    out: Ptr,
    new_size: u32,
) -> u32 {
    let base = this.addr();
    e.with_stack(8, |e, cells| {
        let granted_cell = cells.addr();
        let answer_cell = cells.addr() + 4;
        e.mem.set_u32(granted_cell, base);
        e.mem.set_u32(answer_cell, new_size);
        let segment = word(e, base + 0xa8);
        let server = word(e, base + 0x98);
        let segment_size = word(e, segment + 0xc);
        let answer = e
            .vcall(
                server,
                0x14,
                &args![answer_cell, segment, segment_size, new_size, granted_cell],
            )
            .u32();
        if e.mem.u8(answer) == 0 {
            e.mem.set_u8(out.addr(), 0);
            return out.addr();
        }
        let top_size = word(e, base + 0x14);
        let top = word(e, base + 0x10);
        let fence = word(e, top.wrapping_add(top_size) + 4);
        let granted = word(e, granted_cell);
        let delta = granted.wrapping_sub(word(e, segment + 0xc)) & 0xffff_fff0;
        let top_size = top_size.wrapping_add(delta);
        put(e, base + 0x14, top_size);
        let end = word(e, segment + 0x14).wrapping_add(delta);
        put(e, segment + 0x14, end);
        put(e, segment + 0xc, granted);
        let top = word(e, base + 0x10);
        let header = (fence & 0xffff_fffc).wrapping_sub(delta) | 2;
        let top_size = word(e, base + 0x14);
        put(e, top.wrapping_add(top_size) + 4, header);
        e.mem.set_u8(out.addr(), 1);
        out.addr()
    })
}

// Translated from 00dd8480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkLargeBlockAllocator::_alloc` (Xbox PDB), `thiscall`, `RET 4`: allocates `size` bytes
/// (already rounded): when any bin is in use tries the tree (`_allocLarge`); otherwise, or
/// failing, carves from the top chunk (`00dd83c0`) when it is more than 256 bytes larger than
/// `size`. Answers the user address or 0.
pub fn hk_large_block_allocator_underscore_alloc(e: &mut Engine, this: Ptr, size: u32) -> u32 {
    let base = this.addr();
    if word(e, base + 8) != 0 {
        let found = hk_large_block_allocator_alloc_large(e, this, size);
        if found != 0 {
            return found;
        }
    }
    if size.wrapping_add(0x100) >= word(e, base + 0x14) {
        return 0;
    }
    fn_00dd83c0(e, this, size)
}

/// Asks the server to round `wanted` (slot `8`) and for a segment of that size (slot `0xc`,
/// `(rounded, address of the cell that receives the granted size)`). Answers the segment or 0.
fn new_server_segment(e: &mut Engine, base: u32, wanted: u32, granted_cell: u32) -> u32 {
    let server = word(e, base + 0x98);
    let rounded = e.vcall(server, 8, &args![wanted]).u32();
    put(e, granted_cell, rounded);
    let server = word(e, base + 0x98);
    e.vcall(server, 0xc, &args![rounded, granted_cell]).u32()
}

// Translated from 00dd84c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkLargeBlockAllocator::alloc` (Xbox PDB, virtual slot `4`), `thiscall`, `RET 4`: rounds
/// `size + 0x1f` down to 16 (at least 256) and tries `_alloc`. When that fails: if the
/// server (slot `4`, a flag answered through the pointer it returns) says so and there is a
/// segment, the first segment is grown by the request (`_resizeSingleBlockServerPage`), and on
/// success the request is carved from the top; if growing fails the fallback object (`+4`) is
/// asked to make room (slot `4`) and both are tried again, ending with its slot `8` and a 0
/// answer. Without a resizable segment the server is asked for a new one (`request + 0x138`
/// bytes rounded by slot `8`, then slot `0xc`), with the same fallback retry. A new segment is
/// set up (first chunk 16-byte aligned after its header, end aligned down), linked into the
/// address-ordered ring, the old top chunk is put back in the tree, the new range becomes the
/// top (with its footer, the closing fence header and the neighbours' links to it) and the request
/// is carved from it. Answers the user address or 0.
///
/// The server's flag is written into the stack word that held `size`, and the granted size
/// lands in a local that held `this` first; they are two scratch cells here.
pub fn hk_large_block_allocator_alloc(e: &mut Engine, this: Ptr, size: u32) -> u32 {
    let base = this.addr();
    e.with_stack(8, |e, cells| {
        let flag_cell = cells.addr();
        let granted_cell = cells.addr() + 4;
        e.mem.set_u32(flag_cell, size);
        e.mem.set_u32(granted_cell, base);
        let mut rounded = size.wrapping_add(0x1f) & 0xffff_fff0;
        if rounded < 0x100 {
            rounded = 0x100;
        }
        let found = hk_large_block_allocator_underscore_alloc(e, this, rounded);
        if found != 0 {
            return found;
        }
        let server = word(e, base + 0x98);
        let answer = e.vcall(server, 4, &args![flag_cell]).u32();
        if e.mem.u8(answer) != 0 && word(e, base + 0xa8) != base + 0xa4 {
            let segment = word(e, base + 0xa8);
            let wanted = word(e, segment + 0xc).wrapping_add(rounded);
            let answer = hk_large_block_allocator_resize_single_block_server_page(
                e,
                this,
                Ptr::new(flag_cell),
                wanted,
            );
            if e.mem.u8(answer) != 0 {
                return fn_00dd83c0(e, this, rounded);
            }
            let fallback = word(e, base + 4);
            if fallback == 0 {
                return 0;
            }
            e.vcall(fallback, 4, &args![rounded]);
            let found = hk_large_block_allocator_underscore_alloc(e, this, rounded);
            if found != 0 {
                return found;
            }
            let wanted = word(e, segment + 0xc).wrapping_add(rounded);
            let answer = hk_large_block_allocator_resize_single_block_server_page(
                e,
                this,
                Ptr::new(flag_cell),
                wanted,
            );
            if e.mem.u8(answer) != 0 {
                return fn_00dd83c0(e, this, rounded);
            }
            let fallback = word(e, base + 4);
            e.vcall(fallback, 8, &args![rounded]);
            return 0;
        }
        let wanted = rounded.wrapping_add(0x138);
        let mut segment = new_server_segment(e, base, wanted, granted_cell);
        if segment == 0 {
            let fallback = word(e, base + 4);
            if fallback == 0 {
                return 0;
            }
            e.vcall(fallback, 4, &args![rounded]);
            let found = hk_large_block_allocator_underscore_alloc(e, this, rounded);
            if found != 0 {
                return found;
            }
            segment = new_server_segment(e, base, wanted, granted_cell);
            if segment == 0 {
                let fallback = word(e, base + 4);
                e.vcall(fallback, 8, &args![rounded]);
                return 0;
            }
        }
        // The new segment: its first chunk and its end.
        put(e, segment + 8, 0);
        let granted = word(e, granted_cell);
        put(e, segment + 0x10, segment + 0x18);
        put(e, segment + 0xc, granted);
        let first = (segment + 0x18).wrapping_add(0xf) & 0xffff_fff0;
        put(e, segment + 0x14, segment.wrapping_add(granted));
        let end = word(e, segment + 0x14) & 0xffff_fff0;
        put(e, segment + 0x14, end);
        put(e, segment + 0x10, first);
        // Link it into the ring, ordered by address.
        let sentinel = base + 0xa4;
        let mut after = word(e, base + 0xa8);
        while after != sentinel && after < segment {
            after = word(e, after + 4);
        }
        put(e, segment + 4, after);
        let before = word(e, after);
        put(e, segment, before);
        put(e, before + 4, segment);
        put(e, after, segment);
        // The old top chunk goes back in the tree; the new range becomes the top.
        if word(e, base + 0x10) != base + 0x9c {
            fn_00dd83a0(e, this);
            let top = word(e, base + 0x10);
            let top_size = word(e, top + 4) & 0xffff_fffc;
            hk_large_block_allocator_insert_large_chunk(e, this, Ptr::new(top), top_size);
        }
        let usable = word(e, segment + 0x14)
            .wrapping_sub(word(e, segment + 0x10))
            .wrapping_sub(0x10);
        put(e, base + 0x14, usable);
        let first = word(e, segment + 0x10);
        put(e, base + 0x10, first);
        fn_00dd83a0(e, this);
        let fence = word(e, segment + 0x14).wrapping_sub(0x10);
        put(e, fence + 4, 2);
        let next = word(e, segment + 4);
        if next != sentinel {
            let next_first = word(e, next + 0x10);
            put(e, fence + 4, next_first.wrapping_sub(fence) | 2);
        }
        let previous = word(e, segment);
        if previous != sentinel {
            let end = word(e, previous + 0x14);
            let top = word(e, base + 0x10);
            let flags = word(e, end.wrapping_sub(0xc)) & 3;
            let fence = end.wrapping_sub(0x10);
            put(e, fence + 4, top.wrapping_sub(fence) | flags);
        }
        fn_00dd83c0(e, this, rounded)
    })
}

// Translated from 00dd8700 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkLargeBlockAllocator::criticalAlloc` (Xbox PDB, virtual slot `0x18`), `thiscall`,
/// `RET 4`: rounds `size + 0x1f` down to 16 and allocates through the object's own slot `4`
/// (`alloc`); the block's header word `+0` is set to 1. Failing that the server's slot `0x24`
/// is tried (header word 0). Answers the user address (`+0x10`) or 0.
pub fn hk_large_block_allocator_critical_alloc(e: &mut Engine, this: Ptr, size: u32) -> u32 {
    let rounded = size.wrapping_add(0x1f) & 0xffff_fff0;
    let block = e.vcall(this.addr(), 4, &args![rounded]).u32();
    if block != 0 {
        put(e, block, 1);
        return block + 0x10;
    }
    let server = word(e, this.addr() + 0x98);
    let block = e.vcall(server, 0x24, &args![rounded]).u32();
    if block != 0 {
        put(e, block, 0);
        return block + 0x10;
    }
    0
}

// Translated from 00dd8750 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkLargeBlockAllocator::criticalFree` (Xbox PDB, virtual slot `0x1c`), `thiscall`, `RET 8`
/// `(block, size)`: looks at the header word before the block: 0 gives the block back to the
/// server (slot `0x28`, `(block - 0x10, (size + 0x1f) & -16)`); 1 frees it through the object's
/// own slot `8`; anything else does nothing.
pub fn hk_large_block_allocator_critical_free(e: &mut Engine, this: Ptr, block: Ptr, size: u32) {
    let header = block.addr().wrapping_sub(0x10);
    match word(e, header) {
        0 => {
            let server = word(e, this.addr() + 0x98);
            let rounded = size.wrapping_add(0x1f) & 0xffff_fff0;
            e.vcall(server, 0x28, &args![header, rounded]);
        }
        1 => {
            e.vcall(this.addr(), 8, &args![header]);
        }
        _ => {}
    }
}

// Translated from 00dd8790 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkLargeBlockAllocator::garbageCollect` (Xbox PDB), `fastcall`: writes the top chunk's
/// header (`00dd83a0`), then gives back every server segment whose single free chunk fills it
/// (unlinked from the ring and the tree, or reset when it is the top; server slot `0x10`).
/// With no segment left the top is reset. Otherwise, if the server (slot `4`) can resize,
/// a top chunk of at least 0x8000 bytes shrinks the first segment (`_resize...`, to the
/// top's offset plus 0x120); if it cannot, the top is put back in the tree and the largest free
/// chunk that ends a segment becomes the top (or the top is reset when there is none).
pub fn hk_large_block_allocator_garbage_collect(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let sentinel = base + 0xa4;
    let empty_top = base + 0x9c;
    fn_00dd83a0(e, this);
    let mut segment = word(e, base + 0xa8);
    while segment != sentinel {
        let chunk = word(e, segment + 0x10);
        let header = word(e, chunk + 4);
        let fence = word(e, segment + 0x14).wrapping_sub(0x10);
        if (header >> 1) & 1 != 0 || (header & 0xffff_fffc).wrapping_add(chunk) != fence {
            segment = word(e, segment + 4);
            continue;
        }
        let previous = word(e, segment);
        let next = word(e, segment + 4);
        put(e, previous + 4, next);
        let previous = word(e, segment);
        put(e, word(e, segment + 4), previous);
        if chunk == word(e, base + 0x10) {
            put(e, base + 0x10, empty_top);
            put(e, base + 0x14, 0);
        } else {
            hk_large_block_allocator_unlink_large_chunk(e, this, Ptr::new(chunk));
        }
        let size = word(e, segment + 0xc);
        let server = word(e, base + 0x98);
        e.vcall(server, 0x10, &args![segment, size]);
        segment = next;
    }
    if word(e, base + 0xa8) == sentinel {
        put(e, base + 0x10, empty_top);
        put(e, base + 0x14, 0);
        return;
    }
    // The flag is answered into the last byte of a local that held `this`.
    e.with_stack(4, |e, cell| {
        e.mem.set_u32(cell.addr(), base);
        let flag = cell.addr() + 3;
        let server = word(e, base + 0x98);
        let answer = e.vcall(server, 4, &args![flag]).u32();
        if e.mem.u8(answer) != 0 {
            if word(e, base + 0x14) < 0x8000 {
                return;
            }
            let first = word(e, base + 0xa8);
            let offset = word(e, base + 0x10).wrapping_sub(word(e, first + 0x10));
            hk_large_block_allocator_resize_single_block_server_page(
                e,
                this,
                Ptr::new(flag),
                offset.wrapping_add(0x120),
            );
            return;
        }
        let top = word(e, base + 0x10);
        if top != empty_top {
            let top_size = word(e, base + 0x14);
            hk_large_block_allocator_insert_large_chunk(e, this, Ptr::new(top), top_size);
            put(e, base + 0x10, empty_top);
            put(e, base + 0x14, 0);
        }
        let mut segment = word(e, base + 0xa8);
        let mut largest = 0u32;
        while segment != sentinel {
            let last = word(e, segment + 0x14).wrapping_sub(0x10);
            if word(e, last + 4) & 1 == 0 {
                let start = last.wrapping_sub(word(e, last));
                if largest == 0
                    || word(e, largest + 4) & 0xffff_fffc < word(e, start + 4) & 0xffff_fffc
                {
                    largest = start;
                }
            }
            segment = word(e, segment + 4);
        }
        if largest != 0 {
            hk_large_block_allocator_unlink_large_chunk(e, this, Ptr::new(largest));
            put(e, base + 0x10, largest);
            let size = word(e, largest + 4) & 0xffff_fffc;
            put(e, base + 0x14, size);
            return;
        }
        put(e, base + 0x10, empty_top);
        put(e, base + 0x14, 0);
    });
}

// Translated from 00dd8910 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkLargeBlockAllocator::isValidAlloc` (Xbox PDB, virtual slot `0xc`), `thiscall`, `RET 8`
/// `(out, block)`: stores in the byte at `out` whether `block` is a live allocation: 16-byte
/// aligned, its chunk marked in use, inside a segment, and on the chunk boundary reached by
/// walking that segment's chunks from its first. Answers `out`.
pub fn hk_large_block_allocator_is_valid_alloc(
    e: &mut Engine,
    this: Ptr,
    out: Ptr,
    block: u32,
) -> u32 {
    let result = out.addr();
    if block & 0xf != 0 {
        e.mem.set_u8(result, 0);
        return result;
    }
    let chunk = block.wrapping_sub(0x10);
    if (word(e, block.wrapping_sub(0xc)) >> 1) & 1 == 0 {
        e.mem.set_u8(result, 0);
        return result;
    }
    let sentinel = this.addr() + 0xa4;
    let mut segment = word(e, this.addr() + 0xa8);
    loop {
        if segment == sentinel {
            e.mem.set_u8(result, 0);
            return result;
        }
        let start = word(e, segment + 0x10);
        let end = word(e, segment + 0x14);
        if start <= block && block < end {
            let fence = end.wrapping_sub(0x10);
            if start == fence {
                e.mem.set_u8(result, 0);
                return result;
            }
            let mut walk = start;
            loop {
                if walk == chunk {
                    e.mem.set_u8(result, 1);
                    return result;
                }
                walk = walk.wrapping_add(word(e, walk + 4) & 0xffff_fffc);
                if walk == fence {
                    break;
                }
            }
            e.mem.set_u8(result, 0);
            return result;
        }
        segment = word(e, segment + 4);
    }
}

// Translated from 00dd89b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkLargeBlockAllocator::free` (Xbox PDB, virtual slot `8`), `thiscall`, `RET 4`: frees a
/// block (null does nothing). The two counters (`+0xbc`, `+0xc0`) go down by its size; a free
/// chunk before it (header bit 0 clear) is unlinked and merged; a free chunk after it is merged
/// too (when that is the top chunk the merged chunk becomes the top), else the next chunk's
/// "previous in use" bit is cleared; the merged chunk gets its header and footer and goes back
/// in the tree (`_insertLargeChunk`).
pub fn hk_large_block_allocator_free(e: &mut Engine, this: Ptr, block: u32) {
    if block == 0 {
        return;
    }
    let base = this.addr();
    let mut chunk = block.wrapping_sub(0x10);
    let mut size = word(e, chunk + 4) & 0xffff_fffc;
    let handed = word(e, base + 0xbc).wrapping_sub(size);
    put(e, base + 0xbc, handed);
    let header_size = word(e, chunk + 4) & 0xffff_fffc;
    let payload = word(e, base + 0xc0).wrapping_add(0x10u32.wrapping_sub(header_size));
    put(e, base + 0xc0, payload);
    let next = size.wrapping_add(chunk);
    if word(e, chunk + 4) & 1 == 0 {
        let previous_size = word(e, chunk);
        chunk = chunk.wrapping_sub(previous_size);
        hk_large_block_allocator_unlink_large_chunk(e, this, Ptr::new(chunk));
        size = size.wrapping_add(previous_size);
    }
    let next_header = word(e, next + 4);
    if (next_header >> 1) & 1 == 0 {
        if next == word(e, base + 0x10) {
            let top_size = word(e, base + 0x14).wrapping_add(size);
            put(e, base + 0x14, top_size);
            let top_size = word(e, base + 0x14);
            put(e, base + 0x10, chunk);
            put(e, chunk + 4, top_size | 1);
            return;
        }
        size = size.wrapping_add(next_header & 0xffff_fffc);
        hk_large_block_allocator_unlink_large_chunk(e, this, Ptr::new(next));
        put(e, chunk + 4, size | 1);
        put(e, size.wrapping_add(chunk), size);
        hk_large_block_allocator_insert_large_chunk(e, this, Ptr::new(chunk), size);
        return;
    }
    put(e, next + 4, next_header & 0xffff_fffe);
    put(e, chunk + 4, size | 1);
    put(e, size.wrapping_add(chunk), size);
    hk_large_block_allocator_insert_large_chunk(e, this, Ptr::new(chunk), size);
}

// Translated from 00dd8a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkLargeBlockAllocator::getAllocatedSize` (Xbox PDB, virtual slot `0x10`), `RET 8` (the
/// second word is never read): the block's chunk size less the 16-byte header.
pub fn hk_large_block_allocator_get_allocated_size(
    e: &mut Engine,
    _this: Ptr,
    block: u32,
    _unused_2: u32,
) -> u32 {
    (word(e, block.wrapping_sub(0xc)) & 0xffff_fffc).wrapping_sub(0x10)
}

// Translated from 00dd8a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkLargeBlockAllocator::getAllocatedSizeWithOverhead` (Xbox PDB, virtual slot `0x14`),
/// `RET 8` (the second word is never read): the block's chunk size.
pub fn hk_large_block_allocator_get_allocated_size_with_overhead(
    e: &mut Engine,
    _this: Ptr,
    block: u32,
    _unused_2: u32,
) -> u32 {
    word(e, block.wrapping_sub(0xc)) & 0xffff_fffc
}

// Translated from 00dd8b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): starts a walk over the heap's chunks:
/// writes the top chunk's header (its size with bit 0), and fills `out` with the first segment's
/// first chunk (`+0`), the segment (`+4`) and its fence (`+8`, the segment's end less 16), or
/// stores 0 at `out` when there is no segment.
pub fn fn_00dd8b20(e: &mut Engine, this: Ptr, out: Ptr) {
    let base = this.addr();
    let top = word(e, base + 0x10);
    let size = word(e, base + 0x14);
    put(e, top + 4, size | 1);
    let segment = word(e, base + 0xa8);
    if segment == base + 0xa4 {
        put(e, out.addr(), 0);
        return;
    }
    let first = word(e, segment + 0x10);
    put(e, out.addr() + 4, segment);
    let end = word(e, segment + 0x14).wrapping_sub(0x10);
    put(e, out.addr(), first);
    put(e, out.addr() + 8, end);
}

// Translated from 00dd8b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8` `(out, walk)`): steps the chunk walk of
/// `00dd8b20` (`walk`: chunk, segment, fence) to the next chunk, moving to the next segment's
/// first chunk at a fence, and stores 1 at the byte `out` (0 when the walk was over, or ends:
/// the chunk word is then set to 0). Answers `out`.
pub fn fn_00dd8b60(e: &mut Engine, this: Ptr, out: Ptr, walk: Ptr) -> u32 {
    let result = out.addr();
    let walk = walk.addr();
    let chunk = word(e, walk);
    if chunk == 0 {
        e.mem.set_u8(result, 0);
        return result;
    }
    let chunk = chunk.wrapping_add(word(e, chunk + 4) & 0xffff_fffc);
    put(e, walk, chunk);
    if chunk == word(e, walk + 8) {
        let segment = word(e, word(e, walk + 4) + 4);
        if segment == this.addr() + 0xa4 {
            put(e, walk, 0);
            e.mem.set_u8(result, 0);
            return result;
        }
        put(e, walk + 4, segment);
        let first = word(e, segment + 0x10);
        put(e, walk, first);
        let end = word(e, segment + 0x14).wrapping_sub(0x10);
        put(e, walk + 8, end);
    }
    e.mem.set_u8(result, 1);
    result
}

// Translated from 00dd8c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`fastcall`): the total size of the server segments (the sum of
/// the size word at `+0xc` of each).
pub fn fn_00dd8c20(e: &mut Engine, this: Ptr) -> u32 {
    let sentinel = this.addr() + 0xa4;
    let mut segment = word(e, this.addr() + 0xa8);
    let mut total = 0u32;
    while segment != sentinel {
        total = total.wrapping_add(word(e, segment + 0xc));
        segment = word(e, segment + 4);
    }
    total
}

// Translated from 00dd8d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): fills a statistics block at `out`: the
/// total segment size (`+0`), the bytes handed out without headers (`+4`, from `+0xc0`), the
/// difference of the two (`+0xc`), the largest free chunk (`+0x14`, `00dd8350`) and 0 at `+0x10`.
pub fn fn_00dd8d70(e: &mut Engine, this: Ptr, out: Ptr) {
    let total = fn_00dd8c20(e, this);
    put(e, out.addr(), total);
    let handed = word(e, this.addr() + 0xc0);
    put(e, out.addr() + 4, handed);
    let free = total.wrapping_sub(word(e, this.addr() + 0xc0));
    put(e, out.addr() + 0xc, free);
    let largest = fn_00dd8350(e, this);
    put(e, out.addr() + 0x14, largest);
    put(e, out.addr() + 0x10, 0);
}

// Translated from 00dd8df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkLargeBlockAllocator::_scalar_deleting_destructor_` (Xbox PDB, virtual slot 0), `RET 4`
/// (the flags word is never read, so the object is not freed here): runs the destructor.
/// Answers `this`.
pub fn hk_large_block_allocator_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    _unused_1: u32,
) -> Ptr {
    hk_large_block_allocator_destructor(e, this);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00dd7f10, fn_00dd7f10(Ptr)),
        entry!(
            0x00dd8d40,
            hk_large_block_allocator_hk_large_block_allocator(Ptr, u32) -> Ptr
        ),
        entry!(
            0x00dd7de0,
            hk_free_list_allocator_server_scalar_deleting_destructor(Ptr, u8) -> Ptr
        ),
        entry!(
            0x00dd7e00,
            hk_large_block_allocator_compute_tree_index(u32) -> i32
        ),
        entry!(0x00dd7e70, fn_00dd7e70(u32) -> u32),
        entry!(0x00dd7ec0, hk_large_block_allocator_destructor(Ptr)),
        entry!(0x00dd7fc0, fn_00dd7fc0(Ptr)),
        entry!(
            0x00dd8010,
            hk_large_block_allocator_insert_large_chunk(Ptr, Ptr, u32)
        ),
        entry!(
            0x00dd80d0,
            hk_large_block_allocator_unlink_large_chunk(Ptr, Ptr)
        ),
        entry!(
            0x00dd8180,
            hk_large_block_allocator_alloc_large(Ptr, u32) -> u32
        ),
        entry!(0x00dd8310, fn_00dd8310(Ptr, u32) -> u32),
        entry!(0x00dd8350, fn_00dd8350(Ptr) -> u32),
        entry!(0x00dd83a0, fn_00dd83a0(Ptr)),
        entry!(0x00dd83c0, fn_00dd83c0(Ptr, u32) -> u32),
        entry!(
            0x00dd8400,
            hk_large_block_allocator_resize_single_block_server_page(Ptr, Ptr, u32) -> u32
        ),
        entry!(
            0x00dd8480,
            hk_large_block_allocator_underscore_alloc(Ptr, u32) -> u32
        ),
        entry!(0x00dd84c0, hk_large_block_allocator_alloc(Ptr, u32) -> u32),
        entry!(
            0x00dd8700,
            hk_large_block_allocator_critical_alloc(Ptr, u32) -> u32
        ),
        entry!(
            0x00dd8750,
            hk_large_block_allocator_critical_free(Ptr, Ptr, u32)
        ),
        entry!(0x00dd8790, hk_large_block_allocator_garbage_collect(Ptr)),
        entry!(
            0x00dd8910,
            hk_large_block_allocator_is_valid_alloc(Ptr, Ptr, u32) -> u32
        ),
        entry!(0x00dd89b0, hk_large_block_allocator_free(Ptr, u32)),
        entry!(
            0x00dd8a80,
            hk_large_block_allocator_get_allocated_size(Ptr, u32, u32) -> u32
        ),
        entry!(
            0x00dd8a90,
            hk_large_block_allocator_get_allocated_size_with_overhead(Ptr, u32, u32) -> u32
        ),
        entry!(0x00dd8b20, fn_00dd8b20(Ptr, Ptr)),
        entry!(0x00dd8b60, fn_00dd8b60(Ptr, Ptr, Ptr) -> u32),
        entry!(0x00dd8c20, fn_00dd8c20(Ptr) -> u32),
        entry!(0x00dd8d70, fn_00dd8d70(Ptr, Ptr)),
        entry!(
            0x00dd8df0,
            hk_large_block_allocator_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const SIZE: u32 = 0xd0;

    #[test]
    fn the_set_up_clears_the_state_and_links_the_list_head() {
        let mut e = Engine::new();
        let object = e.mem.alloc(SIZE);
        e.mem.write(object, &[0xcd; SIZE as usize]);
        e.call(0x00dd_7f10, &args![object]);
        for offset in [0x08, 0x0c, 0x14, 0x18, 0x50, 0x94, 0x9c, 0xbc, 0xc0] {
            assert_eq!(e.mem.u32(object + offset), 0, "offset {offset:#x}");
        }
        assert_eq!(e.mem.u32(object + 0xa0), 3);
        assert_eq!(e.mem.u32(object + 0x10), object + 0x9c);
        assert_eq!(e.mem.u32(object + 0xa4), object + 0xa4);
        assert_eq!(e.mem.u32(object + 0xa8), object + 0xa4);
        // Untouched: the vtable word, +4, +0x98, +0xac, +0xb0.
        for offset in [0x00, 0x04, 0x98, 0xac, 0xb0, 0xc4] {
            assert_eq!(
                e.mem.u32(object + offset),
                0xcdcd_cdcd,
                "offset {offset:#x}"
            );
        }
    }

    #[test]
    fn the_constructor_stores_the_table_and_the_parent() {
        let mut e = Engine::new();
        let object = e.mem.alloc(SIZE);
        e.mem.write(object, &[0xcd; SIZE as usize]);
        assert_eq!(
            e.call(0x00dd_8d40, &args![object, 0x1234_0000u32]).u32(),
            object
        );
        assert_eq!(e.mem.u32(object), VTABLE);
        assert_eq!(e.mem.u32(object + 4), 0);
        assert_eq!(e.mem.u32(object + 0x98), 0x1234_0000);
        assert_eq!(e.mem.u32(object + 0xa4), object + 0xa4);
    }
}

#[cfg(test)]
mod heap_tests {
    use super::*;

    const SERVER_VTABLE: u32 = 0x0128_2000;
    const FALLBACK_VTABLE: u32 = 0x0128_2100;
    /// Test switches: `+0` the flag the server answers (slot 4), `+4` whether it resizes,
    /// `+8` non-zero makes it refuse to allocate segments.
    const SWITCHES: u32 = 0x0127_0900;
    const SERVER_FLAG: u32 = 0x0900_3001;
    const SERVER_ROUND: u32 = 0x0900_3002;
    const SERVER_ALLOC: u32 = 0x0900_3003;
    const SERVER_FREE: u32 = 0x0900_3004;
    const SERVER_RESIZE: u32 = 0x0900_3005;
    const SERVER_CRITICAL_ALLOC: u32 = 0x0900_3006;
    const SERVER_CRITICAL_FREE: u32 = 0x0900_3007;
    const FALLBACK_ROOM: u32 = 0x0900_3008;
    const FALLBACK_GIVE_UP: u32 = 0x0900_3009;

    fn noop(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0127_0000, 0x1000);
        e.put_vtable(
            VTABLE,
            &[
                0x00dd_8df0,
                0x00dd_84c0,
                0x00dd_89b0,
                0x00dd_8910,
                0x00dd_8a80,
                0x00dd_8a90,
                0x00dd_8700,
                0x00dd_8750,
            ],
        );
        e.put_vtable(
            SERVER_VTABLE,
            &[
                0,
                SERVER_FLAG,
                SERVER_ROUND,
                SERVER_ALLOC,
                SERVER_FREE,
                SERVER_RESIZE,
                0,
                0,
                0,
                SERVER_CRITICAL_ALLOC,
                SERVER_CRITICAL_FREE,
            ],
        );
        e.put_vtable(FALLBACK_VTABLE, &[0, FALLBACK_ROOM, FALLBACK_GIVE_UP]);
        e.register(SERVER_FLAG, |e, a| {
            let flag = e.mem.u32(SWITCHES);
            e.mem.set_u8(a[1], flag as u8);
            a[1].into_ret()
        });
        e.register(SERVER_ROUND, |_, a| ((a[1] + 0xfff) & !0xfff).into_ret());
        e.register(SERVER_ALLOC, |e, a| {
            if e.mem.u32(SWITCHES + 8) != 0 {
                return 0u32.into_ret();
            }
            // Room beyond the granted size lets a segment grow in place.
            let block = e.mem.alloc(a[1] + 0x2_0000);
            e.mem.set_u32(a[2], a[1]);
            block.into_ret()
        });
        e.register(SERVER_FREE, |e, a| {
            e.mem.free(a[1]);
            Ret::default()
        });
        e.register(SERVER_RESIZE, |e, a| {
            // (this, answer cell, segment, segment size, new size, granted cell)
            let ok = e.mem.u32(SWITCHES + 4);
            e.mem.set_u8(a[1], ok as u8);
            if ok != 0 {
                e.mem.set_u32(a[5], a[4]);
            }
            a[1].into_ret()
        });
        e.register(SERVER_CRITICAL_ALLOC, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(SERVER_CRITICAL_FREE, noop);
        e.register(FALLBACK_ROOM, noop);
        e.register(FALLBACK_GIVE_UP, noop);
        e.register(OPERATOR_DELETE, noop);
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

    /// A constructed allocator and its server.
    fn heap(e: &mut Engine) -> (u32, u32) {
        let allocator = e.mem.alloc(0xd0);
        let server = e.mem.alloc(8);
        e.mem.set_u32(server, SERVER_VTABLE);
        e.call(0x00dd_8d40, &args![allocator, server]);
        (allocator, server)
    }

    fn alloc(e: &mut Engine, allocator: u32, size: u32) -> u32 {
        e.call(0x00dd_84c0, &args![allocator, size]).u32()
    }

    fn set_switches(e: &mut Engine, flag: u32, resize_ok: u32, alloc_fails: u32) {
        e.mem.set_u32(SWITCHES, flag);
        e.mem.set_u32(SWITCHES + 4, resize_ok);
        e.mem.set_u32(SWITCHES + 8, alloc_fails);
    }

    /// A free chunk of `size` bytes inside a fresh block, with a "next chunk" header after it.
    fn free_chunk(e: &mut Engine, size: u32) -> u32 {
        let block = e.mem.alloc(size + 0x40);
        e.mem.set_u32(block + 4, size | 1);
        e.mem.set_u32(block + size + 4, 2);
        block
    }

    #[test]
    fn the_server_destructor_stores_the_base_table_and_deletes_on_request() {
        let mut e = engine();
        let object = e.mem.alloc(8);
        assert_eq!(e.call(0x00dd_7de0, &args![object, 0u32]).u32(), object);
        assert_eq!(e.mem.u32(object), BASE_VTABLE);
        assert!(calls(&e, OPERATOR_DELETE).is_empty());
        e.call(0x00dd_7de0, &args![object, 3u32]);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![object]]);
    }

    #[test]
    fn compute_tree_index_follows_the_bin_scheme() {
        let mut e = engine();
        let mut index = |size: u32| e.call(0x00dd_7e00, &args![size]).i32();
        assert_eq!(index(0), 0);
        assert_eq!(index(0xff), 0);
        assert_eq!(index(0x100), 0);
        assert_eq!(index(0x180), 1);
        assert_eq!(index(0x200), 2);
        assert_eq!(index(0x300), 3);
        assert_eq!(index(0x400), 4);
        assert_eq!(index(0x5000), 12);
        assert_eq!(index(0xff_ffff), 0x1f);
        assert_eq!(index(0x100_0000), 0x1f);
        assert_eq!(index(0xffff_ffff), 0x1f);
    }

    #[test]
    fn fn_00dd7e70_is_the_base_two_logarithm_of_a_power_of_two() {
        let mut e = engine();
        for shift in [0u32, 1, 2, 3, 7, 8, 16, 20, 31] {
            assert_eq!(e.call(0x00dd_7e70, &args![1u32 << shift]).u32(), shift);
        }
    }

    /// Two segments linked in the ring (sizes 0x1000 and 0x2000).
    fn two_segments(e: &mut Engine, allocator: u32) -> (u32, u32) {
        let (first, second) = (e.mem.alloc(0x40), e.mem.alloc(0x40));
        let sentinel = allocator + 0xa4;
        e.mem.set_u32(first + 0xc, 0x1000);
        e.mem.set_u32(second + 0xc, 0x2000);
        e.mem.set_u32(sentinel + 4, first);
        e.mem.set_u32(first, sentinel);
        e.mem.set_u32(first + 4, second);
        e.mem.set_u32(second, first);
        e.mem.set_u32(second + 4, sentinel);
        e.mem.set_u32(sentinel, second);
        (first, second)
    }

    #[test]
    fn the_destructor_gives_the_segments_back_and_stores_the_base_table() {
        let mut e = engine();
        let (allocator, _) = heap(&mut e);
        let (first, second) = two_segments(&mut e, allocator);
        e.call(0x00dd_7ec0, &args![allocator]);
        assert_eq!(
            calls(&e, SERVER_FREE)
                .iter()
                .map(|c| c[1..].to_vec())
                .collect::<Vec<_>>(),
            vec![vec![first, 0x1000], vec![second, 0x2000]]
        );
        assert_eq!(e.mem.u32(allocator), BASE_VTABLE);
    }

    #[test]
    fn fn_00dd7fc0_gives_the_segments_back_and_resets_the_state() {
        let mut e = engine();
        let (allocator, _) = heap(&mut e);
        let (_, second) = two_segments(&mut e, allocator);
        e.mem.set_u32(allocator + 0xbc, 77);
        e.call(0x00dd_7fc0, &args![allocator]);
        assert_eq!(calls(&e, SERVER_FREE).len(), 2);
        assert_eq!(calls(&e, SERVER_FREE)[1][1], second);
        assert_eq!(e.mem.u32(allocator + 0xa8), allocator + 0xa4);
        assert_eq!(e.mem.u32(allocator + 0xbc), 0);
        assert_eq!(e.mem.u32(allocator), VTABLE);
    }

    #[test]
    fn the_scalar_deleting_destructor_runs_the_destructor_and_answers_this() {
        let mut e = engine();
        let (allocator, _) = heap(&mut e);
        let (_, second) = two_segments(&mut e, allocator);
        assert_eq!(
            e.call(0x00dd_8df0, &args![allocator, 1u32]).u32(),
            allocator
        );
        assert_eq!(calls(&e, SERVER_FREE).len(), 2);
        assert_eq!(calls(&e, SERVER_FREE)[1][1], second);
        assert_eq!(e.mem.u32(allocator), BASE_VTABLE);
        // The flags word is not read: nothing is deleted.
        assert!(calls(&e, OPERATOR_DELETE).is_empty());
    }

    #[test]
    fn chunks_go_into_and_out_of_the_tree_bins() {
        let mut e = engine();
        let (allocator, _) = heap(&mut e);
        let bin2 = allocator + 0x18 + 2 * 4;
        let a = free_chunk(&mut e, 0x200);
        let b = free_chunk(&mut e, 0x200);
        let c = free_chunk(&mut e, 0x280);
        let d = free_chunk(&mut e, 0x300);
        // First into an empty bin: the root, its own ring, the bin's bit.
        e.call(0x00dd_8010, &args![allocator, a, 0x200u32]);
        assert_eq!(e.mem.u32(bin2), a);
        assert_eq!(e.mem.u32(allocator + 8), 1 << 2);
        assert_eq!(e.mem.u32(a + 0x18), bin2);
        assert_eq!((e.mem.u32(a + 8), e.mem.u32(a + 0xc)), (a, a));
        assert_eq!(e.mem.u32(a + 0x1c), 2);
        // The same size joins the root's ring.
        e.call(0x00dd_8010, &args![allocator, b, 0x200u32]);
        assert_eq!((e.mem.u32(a + 0xc), e.mem.u32(a + 8)), (b, b));
        assert_eq!((e.mem.u32(b + 0xc), e.mem.u32(b + 8)), (a, a));
        assert_eq!(e.mem.u32(b + 0x18), 0);
        // A larger size becomes the root's second child (its top key bit is 1).
        e.call(0x00dd_8010, &args![allocator, c, 0x280u32]);
        assert_eq!(e.mem.u32(a + 0x14), c);
        assert_eq!(e.mem.u32(c + 0x18), a);
        // Another bin.
        e.call(0x00dd_8010, &args![allocator, d, 0x300u32]);
        assert_eq!(e.mem.u32(allocator + 8), (1 << 2) | (1 << 3));
        assert_eq!(e.mem.u32(allocator + 0x18 + 3 * 4), d);
        // Unlinking a ring member only fixes the ring.
        e.call(0x00dd_80d0, &args![allocator, b]);
        assert_eq!((e.mem.u32(a + 0xc), e.mem.u32(a + 8)), (a, a));
        // The only chunk of its size: its deepest descendant takes its place in the bin.
        e.call(0x00dd_80d0, &args![allocator, a]);
        assert_eq!(e.mem.u32(bin2), c);
        assert_eq!(e.mem.u32(c + 0x18), bin2);
        assert_eq!(e.mem.u32(a + 0x14), 0);
        // The last chunk of the bin empties it and clears its bit.
        e.call(0x00dd_80d0, &args![allocator, c]);
        assert_eq!(e.mem.u32(bin2), 0);
        assert_eq!(e.mem.u32(allocator + 8), 1 << 3);
        e.call(0x00dd_80d0, &args![allocator, d]);
        assert_eq!(e.mem.u32(allocator + 8), 0);
    }

    #[test]
    fn unlinking_the_tree_node_of_a_ring_promotes_the_next_chunk_with_the_children() {
        let mut e = engine();
        let (allocator, _) = heap(&mut e);
        let bin2 = allocator + 0x18 + 2 * 4;
        let a = free_chunk(&mut e, 0x200);
        let b = free_chunk(&mut e, 0x200);
        let c = free_chunk(&mut e, 0x280);
        let d = free_chunk(&mut e, 0x180);
        for (chunk, size) in [(a, 0x200u32), (b, 0x200), (c, 0x280), (d, 0x180)] {
            e.call(0x00dd_8010, &args![allocator, chunk, size]);
        }
        // 0x180 is bin 1, not bin 2.
        assert_eq!(e.mem.u32(allocator + 0x18 + 4), d);
        e.call(0x00dd_80d0, &args![allocator, a]);
        // b is now the tree node: the bin's root, owner of c.
        assert_eq!(e.mem.u32(bin2), b);
        assert_eq!(e.mem.u32(b + 0x18), bin2);
        assert_eq!(e.mem.u32(b + 0x14), c);
        assert_eq!(e.mem.u32(c + 0x18), b);
        assert_eq!((e.mem.u32(b + 8), e.mem.u32(b + 0xc)), (b, b));
    }

    #[test]
    fn alloc_large_splits_a_big_surplus_and_takes_a_small_one_whole() {
        let mut e = engine();
        let (allocator, _) = heap(&mut e);
        let x = free_chunk(&mut e, 0x400);
        e.call(0x00dd_8010, &args![allocator, x, 0x400u32]);
        // Bin 3 is empty: the next used bin up (4) provides it. Surplus 0x100: split.
        assert_eq!(
            e.call(0x00dd_8180, &args![allocator, 0x300u32]).u32(),
            x + 0x10
        );
        assert_eq!(e.mem.u32(x + 4), 0x303);
        assert_eq!(e.mem.u32(allocator + 0xbc), 0x300);
        assert_eq!(e.mem.u32(allocator + 0xc0), 0x2f0);
        let rest = x + 0x300;
        assert_eq!(e.mem.u32(rest + 4), 0x101);
        assert_eq!(e.mem.u32(rest + 0x100), 0x100);
        assert_eq!(e.mem.u32(allocator + 0x18), rest);
        assert_eq!(e.mem.u32(allocator + 8), 1);
        // A surplus under 0x100 goes with the chunk, and the next chunk learns its neighbour
        // is in use.
        let y = free_chunk(&mut e, 0x400);
        e.call(0x00dd_8010, &args![allocator, y, 0x400u32]);
        assert_eq!(
            e.call(0x00dd_8180, &args![allocator, 0x380u32]).u32(),
            y + 0x10
        );
        assert_eq!(e.mem.u32(y + 4), 0x403);
        assert_eq!(e.mem.u32(y + 0x400 + 4), 3);
        assert_eq!(e.mem.u32(allocator + 0xbc), 0x300 + 0x400);
        assert_eq!(e.mem.u32(allocator + 0xc0), 0x2f0 + 0x3f0);
        assert_eq!(e.mem.u32(allocator + 8), 1);
    }

    #[test]
    fn alloc_large_finds_the_best_fit_in_the_bins_tree_and_answers_0_when_nothing_fits() {
        let mut e = engine();
        let (allocator, _) = heap(&mut e);
        // Bin 2 holds 0x280 (root) and 0x200 below it... insert the root first.
        let big = free_chunk(&mut e, 0x280);
        let small = free_chunk(&mut e, 0x200);
        e.call(0x00dd_8010, &args![allocator, big, 0x280u32]);
        e.call(0x00dd_8010, &args![allocator, small, 0x200u32]);
        // A request of 0x200 fits exactly the smaller chunk (a child of the root).
        assert_eq!(
            e.call(0x00dd_8180, &args![allocator, 0x200u32]).u32(),
            small + 0x10
        );
        assert_eq!(e.mem.u32(small + 4), 0x203);
        assert_eq!(e.mem.u32(allocator + 0x18 + 2 * 4), big);
        // Nothing is big enough for 0x1000.
        assert_eq!(e.call(0x00dd_8180, &args![allocator, 0x1000u32]).u32(), 0);
        // With the bin empty and no other bin in use there is nothing either.
        e.call(0x00dd_80d0, &args![allocator, big]);
        assert_eq!(e.call(0x00dd_8180, &args![allocator, 0x200u32]).u32(), 0);
    }

    #[test]
    fn largest_chunk_functions_walk_the_tree() {
        let mut e = engine();
        let (allocator, _) = heap(&mut e);
        let root = free_chunk(&mut e, 0x1000);
        let second = free_chunk(&mut e, 0x3000);
        let first_of_second = free_chunk(&mut e, 0x2000);
        e.mem.set_u32(root + 0x14, second);
        e.mem.set_u32(second + 0x10, first_of_second);
        assert_eq!(e.call(0x00dd_8310, &args![root, 0x500u32]).u32(), 0x3000);
        assert_eq!(e.call(0x00dd_8310, &args![root, 0x9000u32]).u32(), 0x9000);
        assert_eq!(e.call(0x00dd_8310, &args![0u32, 0x42u32]).u32(), 0x42);
        // The allocator: the top's size, or the largest chunk of the highest used bin.
        e.mem.set_u32(allocator + 0x14, 0x500);
        assert_eq!(e.call(0x00dd_8350, &args![allocator]).u32(), 0x500);
        e.mem.set_u32(allocator + 0x18 + 0x10 * 4, root);
        assert_eq!(e.call(0x00dd_8350, &args![allocator]).u32(), 0x3000);
    }

    #[test]
    fn the_top_chunk_helpers_write_its_header_and_carve_from_it() {
        let mut e = engine();
        let (allocator, _) = heap(&mut e);
        let region = e.mem.alloc(0x1000);
        e.mem.set_u32(allocator + 0x10, region);
        e.mem.set_u32(allocator + 0x14, 0x800);
        e.call(0x00dd_83a0, &args![allocator]);
        assert_eq!(e.mem.u32(region + 4), 0x801);
        assert_eq!(e.mem.u32(region + 0x800), 0x800);
        // Carving 0x100: the top moves on, its header and footer follow.
        assert_eq!(
            e.call(0x00dd_83c0, &args![allocator, 0x100u32]).u32(),
            region + 0x10
        );
        assert_eq!(e.mem.u32(allocator + 0x10), region + 0x100);
        assert_eq!(e.mem.u32(allocator + 0x14), 0x700);
        assert_eq!(e.mem.u32(region + 4), 0x103);
        assert_eq!(e.mem.u32(region + 0x100), 0x100);
        assert_eq!(e.mem.u32(region + 0x104), 0x701);
        assert_eq!(e.mem.u32(allocator + 0xbc), 0x100);
        assert_eq!(e.mem.u32(allocator + 0xc0), 0xf0);
    }

    #[test]
    fn underscore_alloc_uses_the_tree_then_the_top() {
        let mut e = engine();
        let (allocator, _) = heap(&mut e);
        let region = e.mem.alloc(0x2000);
        e.mem.set_u32(allocator + 0x10, region);
        e.mem.set_u32(allocator + 0x14, 0x1000);
        // No tree: carved from the top.
        assert_eq!(
            e.call(0x00dd_8480, &args![allocator, 0x200u32]).u32(),
            region + 0x10
        );
        // A top not more than 256 bytes larger than the request is not used.
        e.mem.set_u32(allocator + 0x14, 0x300);
        assert_eq!(e.call(0x00dd_8480, &args![allocator, 0x200u32]).u32(), 0);
        // A chunk in the tree comes first.
        let chunk = free_chunk(&mut e, 0x400);
        e.call(0x00dd_8010, &args![allocator, chunk, 0x400u32]);
        assert_eq!(
            e.call(0x00dd_8480, &args![allocator, 0x400u32]).u32(),
            chunk + 0x10
        );
    }

    #[test]
    fn the_first_allocation_gets_a_segment_from_the_server_and_carves_from_it() {
        let mut e = engine();
        let (allocator, _) = heap(&mut e);
        set_switches(&mut e, 0, 0, 0);
        let pointer = alloc(&mut e, allocator, 100);
        let segment = e.mem.u32(allocator + 0xa8);
        let sentinel = allocator + 0xa4;
        assert_ne!(segment, sentinel);
        // 100 rounds to the 256-byte minimum; the server was asked for it plus 0x138, rounded.
        assert_eq!(calls(&e, SERVER_ROUND)[0][1], 0x238);
        assert_eq!(calls(&e, SERVER_ALLOC)[0][1], 0x1000);
        assert_eq!(e.mem.u32(segment + 0xc), 0x1000);
        let first = (segment + 0x18 + 0xf) & !0xf;
        let end = (segment + 0x1000) & !0xf;
        assert_eq!(e.mem.u32(segment + 0x10), first);
        assert_eq!(e.mem.u32(segment + 0x14), end);
        assert_eq!(pointer, first + 0x10);
        assert_eq!(e.mem.u32(first + 4), 0x103);
        // The rest of the segment is the top; the fence closes it.
        assert_eq!(e.mem.u32(allocator + 0x10), first + 0x100);
        assert_eq!(e.mem.u32(allocator + 0x14), end - first - 0x10 - 0x100);
        assert_eq!(e.mem.u32(end - 0x10 + 4), 2);
        assert_eq!(e.mem.u32(allocator + 0xbc), 0x100);
        assert_eq!(e.mem.u32(allocator + 0xc0), 0xf0);
        // The ring: the segment links to the sentinel both ways.
        assert_eq!(
            (e.mem.u32(segment), e.mem.u32(segment + 4)),
            (sentinel, sentinel)
        );
        assert_eq!(e.mem.u32(sentinel), segment);
    }

    #[test]
    fn free_merges_into_the_top_and_garbage_collect_gives_the_segment_back() {
        let mut e = engine();
        let (allocator, _) = heap(&mut e);
        set_switches(&mut e, 0, 0, 0);
        let pointer = alloc(&mut e, allocator, 100);
        let segment = e.mem.u32(allocator + 0xa8);
        let first = e.mem.u32(segment + 0x10);
        let top_size = e.mem.u32(allocator + 0x14);
        e.call(0x00dd_89b0, &args![allocator, pointer]);
        // The chunk was next to the top: the top grows back over it.
        assert_eq!(e.mem.u32(allocator + 0x10), first);
        assert_eq!(e.mem.u32(allocator + 0x14), top_size + 0x100);
        assert_eq!(e.mem.u32(first + 4), (top_size + 0x100) | 1);
        assert_eq!(
            (e.mem.u32(allocator + 0xbc), e.mem.u32(allocator + 0xc0)),
            (0, 0)
        );
        // The whole segment is free: garbage collection returns it.
        e.call(0x00dd_8790, &args![allocator]);
        assert_eq!(
            calls(&e, SERVER_FREE)
                .iter()
                .map(|c| c[1..].to_vec())
                .collect::<Vec<_>>(),
            vec![vec![segment, 0x1000]]
        );
        assert_eq!(e.mem.u32(allocator + 0xa8), allocator + 0xa4);
        assert_eq!(e.mem.u32(allocator + 0x10), allocator + 0x9c);
        assert_eq!(e.mem.u32(allocator + 0x14), 0);
        // A null block does nothing.
        e.call(0x00dd_89b0, &args![allocator, 0u32]);
    }

    #[test]
    fn free_merges_with_free_neighbours_and_puts_the_result_in_the_tree() {
        let mut e = engine();
        let (allocator, _) = heap(&mut e);
        set_switches(&mut e, 0, 0, 0);
        let a = alloc(&mut e, allocator, 100);
        let b = alloc(&mut e, allocator, 100);
        let c = alloc(&mut e, allocator, 100);
        let _d = alloc(&mut e, allocator, 100);
        // Freeing a middle chunk puts it in the tree (bin 0) and clears the next one's bit 0.
        e.call(0x00dd_89b0, &args![allocator, b]);
        let chunk_b = b - 0x10;
        assert_eq!(e.mem.u32(chunk_b + 4), 0x101);
        assert_eq!(e.mem.u32(chunk_b + 0x100), 0x100);
        assert_eq!(e.mem.u32(c - 0xc) & 1, 0);
        assert_eq!(e.mem.u32(allocator + 0x18), chunk_b);
        // Freeing the one after it merges backwards (it unlinks b from the tree).
        e.call(0x00dd_89b0, &args![allocator, c]);
        assert_eq!(e.mem.u32(chunk_b + 4), 0x201);
        assert_eq!(e.mem.u32(chunk_b + 0x200), 0x200);
        // 0x200 is bin 2; bin 0 is empty again.
        assert_eq!(e.mem.u32(allocator + 0x18), 0);
        assert_eq!(e.mem.u32(allocator + 0x18 + 2 * 4), chunk_b);
        // Freeing the one before merges forwards.
        e.call(0x00dd_89b0, &args![allocator, a]);
        let chunk_a = a - 0x10;
        assert_eq!(e.mem.u32(chunk_a + 4), 0x301);
        assert_eq!(e.mem.u32(chunk_a + 0x300), 0x300);
        assert_eq!(e.mem.u32(allocator + 0x18 + 2 * 4), 0);
        assert_eq!(e.mem.u32(allocator + 0x18 + 3 * 4), chunk_a);
        assert_eq!(e.mem.u32(allocator + 8), 1 << 3);
    }

    #[test]
    fn alloc_grows_the_first_segment_when_the_server_can_resize() {
        let mut e = engine();
        let (allocator, _) = heap(&mut e);
        set_switches(&mut e, 1, 1, 0);
        // The first request has no segment yet: a new one (the flag does not matter).
        let first = alloc(&mut e, allocator, 100);
        assert_ne!(first, 0);
        let segment = e.mem.u32(allocator + 0xa8);
        let end = e.mem.u32(segment + 0x14);
        // A request bigger than what is left: the server resizes the segment.
        let pointer = alloc(&mut e, allocator, 0x1800);
        // 0x1800 rounds up to 0x1810 (size + 0x1f, down to 16).
        let request = 0x1810;
        assert_ne!(pointer, 0);
        let resizes = calls(&e, SERVER_RESIZE);
        assert_eq!(resizes.len(), 1);
        // (this, answer cell, segment, its size, new size, granted cell): size + the request.
        assert_eq!(resizes[0][2..5], [segment, 0x1000, 0x1000 + request]);
        assert_eq!(e.mem.u32(segment + 0xc), 0x1000 + request);
        assert_eq!(e.mem.u32(segment + 0x14), end + request);
        assert_eq!(calls(&e, SERVER_ALLOC).len(), 1);
    }

    #[test]
    fn alloc_asks_the_fallback_when_the_server_gives_nothing() {
        let mut e = engine();
        let (allocator, _) = heap(&mut e);
        set_switches(&mut e, 0, 0, 1);
        // No fallback: the answer is 0.
        assert_eq!(alloc(&mut e, allocator, 100), 0);
        assert!(calls(&e, FALLBACK_ROOM).is_empty());
        // With a fallback it is asked to make room, then to give up.
        let fallback = e.mem.alloc(8);
        e.mem.set_u32(fallback, FALLBACK_VTABLE);
        e.mem.set_u32(allocator + 4, fallback);
        assert_eq!(alloc(&mut e, allocator, 100), 0);
        assert_eq!(calls(&e, FALLBACK_ROOM), vec![vec![fallback, 0x100]]);
        assert_eq!(calls(&e, FALLBACK_GIVE_UP), vec![vec![fallback, 0x100]]);
        assert_eq!(calls(&e, SERVER_ALLOC).len(), 3);
        // A failed resize with a fallback asks it too.
        set_switches(&mut e, 1, 0, 0);
        let _ = alloc(&mut e, allocator, 100);
        assert_ne!(e.mem.u32(allocator + 0xa8), allocator + 0xa4);
        e.call_log = Some(vec![]);
        assert_eq!(alloc(&mut e, allocator, 0x4000), 0);
        assert_eq!(calls(&e, SERVER_RESIZE).len(), 2);
        assert_eq!(calls(&e, FALLBACK_ROOM).len(), 1);
        assert_eq!(calls(&e, FALLBACK_GIVE_UP).len(), 1);
    }

    #[test]
    fn resize_single_block_server_page_extends_the_segment_and_the_top() {
        let mut e = engine();
        let (allocator, _) = heap(&mut e);
        set_switches(&mut e, 0, 1, 0);
        alloc(&mut e, allocator, 100);
        let segment = e.mem.u32(allocator + 0xa8);
        let top_size = e.mem.u32(allocator + 0x14);
        let end = e.mem.u32(segment + 0x14);
        let out = e.mem.alloc(8);
        let answer = e.call(0x00dd_8400, &args![allocator, out, 0x3000u32]).u32();
        assert_eq!(answer, out);
        assert_eq!(e.mem.u8(out), 1);
        assert_eq!(e.mem.u32(segment + 0xc), 0x3000);
        assert_eq!(e.mem.u32(segment + 0x14), end + 0x2000);
        assert_eq!(e.mem.u32(allocator + 0x14), top_size + 0x2000);
        // The fence after the top now sits at the new end, with the old surplus taken off.
        let top = e.mem.u32(allocator + 0x10);
        assert_eq!(
            e.mem.u32(top + top_size + 0x2000 + 4),
            0u32.wrapping_sub(0x2000) | 2
        );
        // A server that refuses changes nothing.
        set_switches(&mut e, 0, 0, 0);
        e.mem.set_u8(out, 7);
        assert_eq!(
            e.call(0x00dd_8400, &args![allocator, out, 0x9000u32]).u32(),
            out
        );
        assert_eq!(e.mem.u8(out), 0);
        assert_eq!(e.mem.u32(segment + 0xc), 0x3000);
    }

    #[test]
    fn critical_alloc_and_free_tag_the_blocks_by_origin() {
        let mut e = engine();
        let (allocator, _) = heap(&mut e);
        set_switches(&mut e, 0, 0, 0);
        // From the allocator itself (virtual slot 4): tag 1.
        let block = e.call(0x00dd_8700, &args![allocator, 100u32]).u32();
        let chunk_user = block - 0x10;
        assert_eq!(e.mem.u32(chunk_user), 1);
        assert_eq!(e.mem.u32(allocator + 0xbc), 0x100);
        e.call(0x00dd_8750, &args![allocator, block, 100u32]);
        assert_eq!(e.mem.u32(allocator + 0xbc), 0);
        // When the allocator cannot, the server's critical slot provides it: tag 0, and it
        // takes it back with the rounded size.
        set_switches(&mut e, 0, 0, 1);
        e.mem.set_u32(allocator + 0xa8, allocator + 0xa4);
        e.mem.set_u32(allocator + 0x10, allocator + 0x9c);
        e.mem.set_u32(allocator + 0x14, 0);
        let from_server = e.call(0x00dd_8700, &args![allocator, 100u32]).u32();
        assert_eq!(e.mem.u32(from_server - 0x10), 0);
        e.call(0x00dd_8750, &args![allocator, from_server, 100u32]);
        let freed = calls(&e, SERVER_CRITICAL_FREE);
        assert_eq!(freed.len(), 1);
        assert_eq!(freed[0][1..], [from_server - 0x10, 0x80]);
        // A tag that is neither does nothing.
        e.mem.set_u32(from_server - 0x10, 9);
        e.call(0x00dd_8750, &args![allocator, from_server, 100u32]);
        assert_eq!(calls(&e, SERVER_CRITICAL_FREE).len(), 1);
        // No allocator, no server block: 0.
        e.mem.set_u32(SWITCHES + 8, 1);
        e.register(SERVER_CRITICAL_ALLOC, |_, _| Ret::default());
        assert_eq!(e.call(0x00dd_8700, &args![allocator, 100u32]).u32(), 0);
    }

    #[test]
    fn garbage_collect_shrinks_a_big_top_when_the_server_resizes() {
        let mut e = engine();
        let (allocator, _) = heap(&mut e);
        set_switches(&mut e, 1, 1, 0);
        let first = alloc(&mut e, allocator, 100);
        let _second = alloc(&mut e, allocator, 0xa000);
        e.call(0x00dd_89b0, &args![allocator, first]);
        let segment = e.mem.u32(allocator + 0xa8);
        // Something is still allocated, so the segment stays; the top is big.
        e.mem.set_u32(allocator + 0x14, 0x9000);
        let offset = e.mem.u32(allocator + 0x10) - e.mem.u32(segment + 0x10);
        e.call(0x00dd_8790, &args![allocator]);
        let resizes = calls(&e, SERVER_RESIZE);
        assert_eq!(resizes.last().unwrap()[4], offset + 0x120);
        // A top under 0x8000 is left alone.
        e.mem.set_u32(allocator + 0x14, 0x7fff);
        let before = calls(&e, SERVER_RESIZE).len();
        e.call(0x00dd_8790, &args![allocator]);
        assert_eq!(calls(&e, SERVER_RESIZE).len(), before);
    }

    #[test]
    fn garbage_collect_without_a_resizing_server_keeps_the_largest_free_end_chunk_as_top() {
        let mut e = engine();
        let (allocator, _) = heap(&mut e);
        set_switches(&mut e, 0, 0, 0);
        let a = alloc(&mut e, allocator, 100);
        let _b = alloc(&mut e, allocator, 100);
        let segment = e.mem.u32(allocator + 0xa8);
        let top = e.mem.u32(allocator + 0x10);
        let top_size = e.mem.u32(allocator + 0x14);
        e.call(0x00dd_8790, &args![allocator]);
        // The top went into the tree and came out again as the largest free end chunk.
        assert_eq!(e.mem.u32(allocator + 0x10), top);
        assert_eq!(e.mem.u32(allocator + 0x14), top_size);
        assert_eq!(e.mem.u32(allocator + 8), 0);
        assert_eq!(e.mem.u32(allocator + 0xa8), segment);
        let _ = a;
        // With the end chunk in use (fence bit 0 set) nothing is left to be the top.
        let fence = e.mem.u32(segment + 0x14) - 0x10;
        e.mem.set_u32(fence + 4, e.mem.u32(fence + 4) | 1);
        e.mem.set_u32(allocator + 0x10, allocator + 0x9c);
        e.mem.set_u32(allocator + 0x14, 0);
        e.call(0x00dd_8790, &args![allocator]);
        assert_eq!(e.mem.u32(allocator + 0x10), allocator + 0x9c);
        assert_eq!(e.mem.u32(allocator + 0x14), 0);
    }

    #[test]
    fn is_valid_alloc_accepts_only_live_chunk_boundaries() {
        let mut e = engine();
        let (allocator, _) = heap(&mut e);
        set_switches(&mut e, 0, 0, 0);
        let pointer = alloc(&mut e, allocator, 100);
        let out = e.mem.alloc(8);
        let check = |e: &mut Engine, block: u32| {
            e.mem.set_u8(out, 9);
            assert_eq!(
                e.call(0x00dd_8910, &args![allocator, out, block]).u32(),
                out
            );
            e.mem.u8(out)
        };
        assert_eq!(check(&mut e, pointer), 1);
        // Misaligned.
        assert_eq!(check(&mut e, pointer + 8), 0);
        // Aligned but not on a chunk boundary (its header word claims "in use").
        let inside = pointer + 0x40;
        e.mem.set_u32(inside - 0xc, 2);
        assert_eq!(check(&mut e, inside), 0);
        // In no segment.
        let outside = (e.mem.alloc(0x80) + 0x2f) & !0xf;
        e.mem.set_u32(outside - 0xc, 2);
        assert_eq!(check(&mut e, outside), 0);
        // Not in use.
        e.mem.set_u32(outside - 0xc, 1);
        assert_eq!(check(&mut e, outside), 0);
        // Freed: no longer valid.
        e.call(0x00dd_89b0, &args![allocator, pointer]);
        assert_eq!(check(&mut e, pointer), 0);
    }

    #[test]
    fn allocated_size_functions_read_the_chunk_header() {
        let mut e = engine();
        let block = e.mem.alloc(0x40) + 0x10;
        e.mem.set_u32(block - 0xc, 0x103);
        assert_eq!(e.call(0x00dd_8a80, &args![0u32, block, 0u32]).u32(), 0xf0);
        assert_eq!(e.call(0x00dd_8a90, &args![0u32, block, 0u32]).u32(), 0x100);
    }

    #[test]
    fn the_heap_walk_visits_every_chunk_and_the_statistics_add_up() {
        let mut e = engine();
        let (allocator, _) = heap(&mut e);
        set_switches(&mut e, 0, 0, 0);
        alloc(&mut e, allocator, 100);
        let segment = e.mem.u32(allocator + 0xa8);
        let first = e.mem.u32(segment + 0x10);
        let top = e.mem.u32(allocator + 0x10);
        let fence = e.mem.u32(segment + 0x14) - 0x10;
        let walk = e.mem.alloc(16);
        e.call(0x00dd_8b20, &args![allocator, walk]);
        assert_eq!(
            [e.mem.u32(walk), e.mem.u32(walk + 4), e.mem.u32(walk + 8)],
            [first, segment, fence]
        );
        let out = e.mem.alloc(8);
        // First chunk, then the top, then the end of the only segment.
        assert_eq!(e.call(0x00dd_8b60, &args![allocator, out, walk]).u32(), out);
        assert_eq!((e.mem.u8(out), e.mem.u32(walk)), (1, top));
        e.call(0x00dd_8b60, &args![allocator, out, walk]);
        assert_eq!((e.mem.u8(out), e.mem.u32(walk)), (0, 0));
        // A finished walk stays finished.
        e.mem.set_u8(out, 5);
        e.call(0x00dd_8b60, &args![allocator, out, walk]);
        assert_eq!(e.mem.u8(out), 0);
        // With no segment the walk start stores 0.
        let (empty, _) = heap(&mut e);
        e.mem.set_u32(walk, 77);
        e.call(0x00dd_8b20, &args![empty, walk]);
        assert_eq!(e.mem.u32(walk), 0);
        // Statistics: total, handed out, difference, largest free chunk.
        let stats = e.mem.alloc(0x18);
        e.call(0x00dd_8d70, &args![allocator, stats]);
        let words: Vec<u32> = (0..6).map(|i| e.mem.u32(stats + 4 * i)).collect();
        let top_size = e.mem.u32(allocator + 0x14);
        assert_eq!(words, vec![0x1000, 0xf0, 0, 0x1000 - 0xf0, 0, top_size]);
        assert_eq!(e.call(0x00dd_8c20, &args![allocator]).u32(), 0x1000);
    }
}

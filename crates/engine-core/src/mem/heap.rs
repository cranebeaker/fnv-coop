//! The heap's free space: which addresses below the heap's top are free, and where a new block
//! goes. A platform layer like [`super::Mem::alloc`] itself (the game's
//! `MemoryManager::Allocate`, `HeapAlloc` and `VirtualAlloc` reservations are all served from
//! it), so its addresses never had to match the exe's.
//!
//! The heap runs from [`HEAP_BASE`] up to its top ([`Heap::top`]). Below the top every byte is
//! in a live block, a free block in a bin, or a block waiting in a quick stack, except the
//! platform's addresses between the heap's two ranges ([`super::PLATFORM_AREA`]):
//!
//! - The top starts in the lower range ([`HEAP_BASE`]..[`HEAP_LOW_END`]) and stays there
//!   until a request finds no room in it after everything below has been tried (the bins, the
//!   whole of its own bin, every block waiting in the quick stacks). Only then does the top
//!   move to the upper range ([`HEAP_HIGH_BASE`]..[`HEAP_END`]): what was left of the lower
//!   range goes into the bins as one free block, and the request is served from the new top.
//!   So a run whose memory fits in the lower range places every block where it did before the
//!   upper range existed, and afterwards the free blocks of the lower range still serve
//!   requests first (the bins come before the top).
//! - When the upper range empties again (a freed block brings the top down to
//!   [`HEAP_HIGH_BASE`]), the top goes back to the lower range's end, taking in the free block
//!   that ends there.
//!
//! A block never spans both ranges. The policy is a two-level segregated fit (the scheme known
//! as TLSF):
//!
//! - Free blocks sit in bins by size: one bin per 8-byte size below [`EXACT_LIMIT`], then
//!   [`SUB_BINS`] bins per power of two, each holding sizes within about 3% of one another. A
//!   bin is a list with the newest block first, and a bitmap of the bins that hold a block finds
//!   the smallest such bin at or above a size in a few instructions.
//! - A request takes the newest block of its own bin that fits (one of exactly its size first,
//!   so in a bin of one size it is the newest freed block of that size), else the newest block
//!   of the next bin above that holds one (every block there fits), else fresh memory at the
//!   top. It looks at the first [`OWN_BIN_LOOK`] blocks of its own bin; only when nothing else
//!   is left does it search the whole bin, so the heap reports itself full only when no free
//!   block fits.
//! - A bigger block is split: the request takes its start and the rest stays free, so a block
//!   handed out always has exactly the size asked for.
//! - A freed block merges with its free neighbours on both sides; a free block that reaches the
//!   top lowers the top instead of going into a bin, so the top can come down again.
//! - In front of the bins, freed blocks of up to [`QUICK_MAX`] bytes wait in stacks of one class
//!   each, newest first, for the next request of their class, without any merging: the game makes
//!   and drops tens of thousands of small objects a frame, nearly all of sizes it has just
//!   freed, and a stack serves them as cheaply as the old exact-size lists did. A class is the
//!   exe's memory pool ([`pool_size`]): as in the exe, a freed block comes back only for a
//!   request of its own pool (a 12-byte object's block never for a 16-byte one, though both are
//!   16 bytes here), so a stale pointer to a freed object finds an object of its own pool or the
//!   old bytes, as in the original, never a different-sized object. At most
//!   [`QUICK_BYTES`] wait: a free that would put more there moves a few of the oldest waiting
//!   blocks on to the bins, where they merge (at most [`SPILL_STEP`] + 1, so no one free stalls
//!   the frame); only those, merged, can serve another pool's request. A request that the bins and the top cannot serve moves them on too,
//!   [`FLUSH_STEP`] at a time, trying again after each batch: it may then be served, and only
//!   when it cannot be once all have gone does the heap report itself full.
//!
//! All the bookkeeping is in host tables keyed by address. Nothing is written into guest memory
//! (a freed block keeps what the freed-block check filled it with, and the write watch sees
//! only the game's own stores). The tables that find a free block by its start and by its end
//! are split by the address's [`REGION_BITS`] region ([`Regions`]), so that no single free
//! rehashes every free block at once.

use super::{HEAP_BASE, HEAP_END, HEAP_HIGH_BASE, HEAP_LOW_END};
use crate::engine::AddrMap;
use std::collections::VecDeque;

/// Free blocks below this size have a bin of their own size each.
const EXACT_LIMIT: u32 = 512;
/// The bins of one size each (sizes 0 to [`EXACT_LIMIT`] - 8 by 8; size 0 is never used).
const EXACT_BINS: usize = (EXACT_LIMIT / 8) as usize;
/// The bins per power of two above [`EXACT_LIMIT`], as a shift (32 bins).
const SUB_SHIFT: u32 = 5;
const SUB_BINS: usize = 1 << SUB_SHIFT;
/// The power of two [`EXACT_LIMIT`] is.
const FIRST_POWER: u32 = EXACT_LIMIT.trailing_zeros();
/// Every bin: the exact ones, then the sub-bins of each power of two up to 2^31.
const BINS: usize = EXACT_BINS + (32 - FIRST_POWER as usize) * SUB_BINS;
/// The words of the bitmap of bins that hold a block.
const WORDS: usize = BINS.div_ceil(64);
const _: () = assert!(WORDS <= 32, "the bitmap of words must fit in a u32");
/// Freed blocks of up to this size wait in the quick stacks: the sizes the old exact-size lists
/// were indexed for. Through the bins alone, a free and an allocation of 264 bytes to 4 KB took
/// 67-116% longer than with those lists, and of 8 to 256 bytes more than twice as long
/// (`heap_speed` in `mem.rs`, the runs given at [`QUICK_BYTES`]).
pub(super) const QUICK_MAX: u32 = 4096;
/// The most bytes the quick stacks hold. With every multiple of 8 from 264 to 4096 in use, 1 MB
/// holds about one block of each size, so nearly every block went on and the stacks seldom
/// served a request; with 8 MB a free and an allocation of those sizes take 12-32% longer than
/// with the old lists, 23-32% in eight of the nine runs pinned to one processor (`heap_speed`,
/// release, 2026-11-01 and 11-02: twelve runs in four sets of three, of this heap and its
/// round-2 form, on a machine with other work running). Small sizes alone -11 to +27%, five
/// sizes +6 to +26%, the game-like mix -1 to +15%.
pub(super) const QUICK_BYTES: u64 = 8 << 20;
/// The quick stacks, one per class / 4 (stacks 0 to 1 unused; see [`Heap::take`]), and the words
/// of the bitmap of those that hold a block.
const STACKS: usize = QUICK_MAX as usize / 4 + 1;
const STACK_WORDS: usize = STACKS.div_ceil(64);
/// How many blocks of its own bin a request looks at for one that fits.
const OWN_BIN_LOOK: u32 = 8;
/// How many waiting blocks, the oldest of one size after another, a free that puts the quick
/// stacks over [`QUICK_BYTES`] moves on to the bins before it moves the oldest of its own size
/// (which brings them back under). Moving the older half of every stack at once, as before,
/// took 51-60 ms in one free with 8 MB of 8-byte blocks waiting (release, pinned to one core):
/// 6 or 7 frames at 120 fps.
const SPILL_STEP: usize = 16;
/// How many waiting blocks a request the bins and the top cannot serve moves on to the bins
/// before it tries them again ([`Heap::take_free`]): moving all of them at once took one request
/// as long as moving 8 MB of 8-byte blocks takes (about a million merges) even when the first
/// few would have served it.
const FLUSH_STEP: usize = 4096;
/// The size, as a power of two, of the address regions [`Regions`] splits its table by (256 KB).
/// Two free blocks in bins are never adjacent, so a region holds at most one free block per 16
/// bytes: 16K, which one table of a region rehashes in well under a millisecond.
const REGION_BITS: u32 = 18;
/// No node.
const NONE: u32 = u32::MAX;

/// The block sizes of the exe's memory pools, as `MemoryManager::SpecifyPools` (00866770) makes
/// them ("Pool 8" to "Pool 596").
const POOLS: [u32; 37] = [
    8, 12, 16, 20, 24, 28, 32, 36, 40, 44, 48, 52, 56, 64, 72, 76, 80, 92, 96, 100, 104, 120, 128,
    160, 192, 200, 208, 240, 256, 272, 288, 400, 448, 480, 508, 576, 596,
];
/// The largest request the exe's pool table by size (011f67b8, a pool per 4 bytes) is read for:
/// `MemoryManager::Allocate` (00aa3e40) and its fallback (00aa4960) look only below 0x3fd.
const POOL_TABLE_END: u32 = 0x3fc;

/// Whether the exe makes a pool of blocks of `size` bytes.
const fn is_pool(size: u32) -> bool {
    let mut i = 0;
    while i < POOLS.len() {
        if POOLS[i] == size {
            return true;
        }
        i += 1;
    }
    false
}

/// [`POOL_OF`]'s entries: for every size `s` rounded up to 4 (at least 8) the pool
/// `MemoryManager::Allocate` (00aa3e40, disassembly) serves it from: the pool of exactly `s`;
/// else 00aa4960's: below 300 that of `s` rounded up to 16, below 0x200 that one or else that of
/// `s` rounded up to 32, below 0x240 the 576-byte pool; else none (0: the general heap).
const fn pool_table() -> [u16; (POOL_TABLE_END / 4 + 1) as usize] {
    let mut table = [0u16; (POOL_TABLE_END / 4 + 1) as usize];
    let mut i = 2;
    while i < table.len() {
        let s = i as u32 * 4;
        let (by16, by32) = ((s + 15) & !15, (s + 31) & !31);
        let pool = if is_pool(s) {
            s
        } else if s < 300 {
            if is_pool(by16) {
                by16
            } else {
                0
            }
        } else if s < 0x200 {
            if is_pool(by16) {
                by16
            } else if is_pool(by32) {
                by32
            } else {
                0
            }
        } else if s < 0x240 {
            0x240
        } else {
            0
        };
        table[i] = pool as u16;
        i += 1;
    }
    table
}

/// The exe's pool for a request, by its size rounded up to 4 / 4 (see [`pool_table`]).
static POOL_OF: [u16; (POOL_TABLE_END / 4 + 1) as usize] = pool_table();

/// A request of `asked` bytes in the exe's pools: the block size of the pool the exe's
/// `MemoryManager::Allocate` (00aa3e40) serves it from (its size rounded up to 4, at least 8,
/// when the exe has that pool; else 00aa4960's pool of it rounded up to 16 or 32, or the
/// 576-byte one), or, outside the pools (the exe's general heap), `asked` rounded up to 8.
/// Always a multiple of 4; the heap holds it rounded up to 8.
#[inline]
pub(super) fn pool_size(asked: u32) -> u32 {
    if asked <= POOL_TABLE_END {
        let pool = POOL_OF[(asked.max(8) as usize + 3) >> 2];
        if pool != 0 {
            return u32::from(pool);
        }
    }
    asked.max(1).div_ceil(8) * 8
}

/// The bytes a block of class `class` holds ([`Heap::take`]): the class rounded up to 8.
#[inline]
pub(super) fn class_bytes(class: u32) -> u32 {
    (class + 7) & !7
}

/// The bin of a free block of `size` bytes (a multiple of 8, at least 8).
#[inline]
fn bin_of(size: u32) -> usize {
    if size < EXACT_LIMIT {
        (size >> 3) as usize
    } else {
        let power = 31 - size.leading_zeros();
        let sub = (size >> (power - SUB_SHIFT)) as usize & (SUB_BINS - 1);
        EXACT_BINS + (power - FIRST_POWER) as usize * SUB_BINS + sub
    }
}

/// A free block in a bin, linked into its bin's list by node number.
#[derive(Clone, Copy, Debug)]
struct Node {
    start: u32,
    size: u32,
    prev: u32,
    next: u32,
}

/// A table keyed by heap address (a free block's node number here, a live block's size in
/// [`super::Mem`]), split into one hash table per [`REGION_BITS`] region of the heap. A hash
/// table doubles its slots all at once, rehashing every entry: one table of every free block did
/// that inside a single free, 18-27 ms at about 0.9M free blocks and 37-51 ms at about 1.8M
/// (measured 2026-11-01, a burst of frees that did not merge). A region's table holds at most
/// 16K entries, so the most one free rehashes is that many: in the same burst of 3M frees no
/// free took more than 0.6 ms, and the burst took 90 ns a free against 230-260 ns. The table of
/// live blocks did the same inside one allocation (9-10 ms at 0.9M live blocks, 18-19 ms at
/// 1.8M); a region holds at most 32K of those.
#[derive(Default)]
pub(super) struct Regions {
    /// The table of each region, from [`HEAP_BASE`] up (as far as a region with an entry).
    tables: Vec<AddrMap<u32, u32>>,
    /// The entries in all of them.
    len: usize,
}

impl Regions {
    /// The region of heap address `addr` (an address below [`HEAP_BASE`] wraps round to a
    /// region far past the heap's, which holds nothing).
    #[inline]
    fn region(addr: u32) -> usize {
        (addr.wrapping_sub(HEAP_BASE) >> REGION_BITS) as usize
    }

    #[inline]
    pub(super) fn get(&self, addr: &u32) -> Option<&u32> {
        self.tables.get(Self::region(*addr))?.get(addr)
    }

    #[inline]
    fn get_mut(&mut self, addr: &u32) -> Option<&mut u32> {
        self.tables.get_mut(Self::region(*addr))?.get_mut(addr)
    }

    /// Puts `n` under heap address `addr` (at least [`HEAP_BASE`]).
    #[inline]
    pub(super) fn insert(&mut self, addr: u32, n: u32) {
        assert!(addr >= HEAP_BASE, "{addr:08x} is below the heap");
        let r = Self::region(addr);
        if r >= self.tables.len() {
            self.tables.resize_with(r + 1, AddrMap::default);
        }
        if self.tables[r].insert(addr, n).is_none() {
            self.len += 1;
        }
    }

    #[inline]
    pub(super) fn remove(&mut self, addr: &u32) -> Option<u32> {
        let n = self.tables.get_mut(Self::region(*addr))?.remove(addr);
        self.len -= usize::from(n.is_some());
        n
    }

    pub(super) fn len(&self) -> usize {
        self.len
    }

    pub(super) fn values(&self) -> impl Iterator<Item = &u32> + '_ {
        self.tables.iter().flat_map(|t| t.values())
    }

    /// Every address and its entry (the checks of the tests, the heap census).
    pub(super) fn iter(&self) -> impl Iterator<Item = (&u32, &u32)> + '_ {
        self.tables.iter().flat_map(|t| t.iter())
    }

    /// The most entries one region's table holds.
    #[cfg(test)]
    fn most_in_one(&self) -> usize {
        self.tables.iter().map(|t| t.len()).max().unwrap_or(0)
    }
}

/// The size, as a power of two, of the chunks [`Nodes`] keeps its nodes in (4096, 64 KB).
const NODE_CHUNK_BITS: u32 = 12;

/// The nodes of the free blocks in bins by number, in chunks that never move, and a list of the
/// numbers not in use (through their `next`). One vector of every node copied all of them each
/// time it doubled: 1.9 ms in one free at a million free blocks (measured 2026-11-01).
#[derive(Default)]
struct Nodes {
    chunks: Vec<Box<[Node]>>,
    /// The nodes made so far, and how many of them are on the list of those not in use.
    len: usize,
    spare_len: usize,
    /// The first node not in use (while `spare_len` is not 0).
    spare: u32,
}

impl Nodes {
    /// A node for `node`: the last one given back, else a new one.
    fn alloc(&mut self, node: Node) -> u32 {
        if self.spare_len != 0 {
            let n = self.spare;
            self.spare = self[n as usize].next;
            self.spare_len -= 1;
            self[n as usize] = node;
            return n;
        }
        let n = self.len;
        if n >> NODE_CHUNK_BITS == self.chunks.len() {
            self.chunks
                .push(vec![node; 1 << NODE_CHUNK_BITS].into_boxed_slice());
        }
        self.len += 1;
        self[n] = node;
        n as u32
    }

    /// Gives node `n` back.
    fn release(&mut self, n: u32) {
        let spare = self.spare;
        self[n as usize].next = spare;
        self.spare = n;
        self.spare_len += 1;
    }
}

impl std::ops::Index<usize> for Nodes {
    type Output = Node;
    #[inline]
    fn index(&self, n: usize) -> &Node {
        &self.chunks[n >> NODE_CHUNK_BITS][n & ((1 << NODE_CHUNK_BITS) - 1)]
    }
}

impl std::ops::IndexMut<usize> for Nodes {
    #[inline]
    fn index_mut(&mut self, n: usize) -> &mut Node {
        &mut self.chunks[n >> NODE_CHUNK_BITS][n & ((1 << NODE_CHUNK_BITS) - 1)]
    }
}

/// The heap's top and free space (see the module's description).
pub(super) struct Heap {
    /// The next fresh address: everything at or above it is unused. In the lower range
    /// (at most [`HEAP_LOW_END`]) or, once that is full, in the upper one (above
    /// [`HEAP_HIGH_BASE`]); never between them.
    top: u32,
    /// The highest the top has been.
    peak: u32,
    /// The free blocks in bins, by node number.
    nodes: Nodes,
    /// The node of each free block in a bin, by its start and by its end (the address after
    /// it), to find a freed block's neighbours.
    by_start: Regions,
    by_end: Regions,
    /// The newest block of each bin.
    heads: [u32; BINS],
    /// The bins that hold a block (bit `b % 64` of word `b / 64`), and the words that are not 0.
    words: [u64; WORDS],
    nonempty: u32,
    /// Bytes in the bins.
    free_bytes: u64,
    /// The quick stacks by class / 4, the newest block at the back (the one a request takes) and
    /// the oldest at the front (the first to go on to the bins), and the bytes in them.
    quick: Vec<VecDeque<u32>>,
    quick_bytes: u64,
    /// The quick stacks that hold a block (bit `i % 64` of word `i / 64` for stack `i`), so
    /// that a move to the bins finds the next one without looking at every empty stack.
    waiting: [u64; STACK_WORDS],
    /// The stack the last move to the bins took a block from: the next move looks at the stacks
    /// after it first, so every size ages out in turn.
    spill_from: usize,
    /// The bytes and blocks the quick stacks held when the last request that failed moved them
    /// all to the bins on its way to failing (for the heap's report).
    failed_flush: (u64, usize),
    /// The largest size the quick stacks take (0: none, for tests of the bins alone).
    quick_max: u32,
}

impl Heap {
    pub(super) fn new() -> Self {
        Heap {
            top: HEAP_BASE,
            peak: HEAP_BASE,
            nodes: Nodes::default(),
            by_start: Regions::default(),
            by_end: Regions::default(),
            heads: [NONE; BINS],
            words: [0; WORDS],
            nonempty: 0,
            free_bytes: 0,
            quick: vec![VecDeque::new(); STACKS],
            quick_bytes: 0,
            waiting: [0; STACK_WORDS],
            spill_from: 0,
            failed_flush: (0, 0),
            quick_max: QUICK_MAX,
        }
    }

    /// The next fresh address. It comes down when the block below it is freed.
    pub(super) fn top(&self) -> u32 {
        self.top
    }

    /// The highest address the heap has reached: no block was ever handed out above it.
    pub(super) fn peak(&self) -> u32 {
        self.peak
    }

    /// A block of class `class`, or `None` when the heap has no room for it. The class is the
    /// block's size (a multiple of 8, at least 8), less 4 for a block of one of the exe's pools
    /// whose size is 4 above a multiple of 8 ([`pool_size`]): the 12-byte pool's 16-byte blocks
    /// wait apart from the 16-byte pool's. A freed block of up to [`QUICK_MAX`] bytes comes back
    /// only for a request of its class (from its quick stack); a bigger one, or one the stacks
    /// moved on to the bins, for any request it fits.
    #[inline]
    pub(super) fn take(&mut self, class: u32) -> Option<u32> {
        let size = class_bytes(class);
        if class <= self.quick_max {
            let i = (class >> 2) as usize;
            if let Some(a) = self.quick[i].pop_back() {
                self.quick_bytes -= u64::from(size);
                if self.quick[i].is_empty() {
                    self.waiting[i / 64] &= !(1 << (i % 64));
                }
                return Some(a);
            }
        }
        self.take_free(size)
    }

    /// [`Heap::take`] past the quick stacks: the bins, then the top. When both fail, the whole
    /// of the request's own bin is searched (a fitting block can sit behind more than
    /// [`OWN_BIN_LOOK`] newer, smaller ones), then the waiting blocks go to the bins,
    /// [`FLUSH_STEP`] at a time, the bins and the top tried again after each batch. Nothing in
    /// the bins fitted before the batches, so a block that fits after one is one a release in
    /// the batches made (the largest is noted): when one did and the usual look does not find
    /// it (newer, smaller blocks of its bin in front of it), the whole bin is searched. So the
    /// first batch that makes a fitting block, or lowers the top far enough, serves the request
    /// and the rest still wait; once none wait and none was made, nothing fits. A request that
    /// fails notes what the quick stacks held ([`Heap::failed_flush`]). A request that fails
    /// with the top still in the lower range moves it to the upper range ([`Heap::cross`]).
    fn take_free(&mut self, size: u32) -> Option<u32> {
        self.take_in_range(size).or_else(|| self.cross(size))
    }

    /// [`Heap::take_free`] without moving the top to the upper range.
    fn take_in_range(&mut self, size: u32) -> Option<u32> {
        if let Some(n) = self.fit(size, OWN_BIN_LOOK) {
            return Some(self.carve(n, size));
        }
        if let Some(a) = self.grow(size) {
            return Some(a);
        }
        if let Some(n) = self.fit(size, u32::MAX) {
            return Some(self.carve(n, size));
        }
        let waiting = (self.quick_bytes, self.quick_blocks());
        if waiting.0 == 0 {
            self.failed_flush = waiting;
            return None;
        }
        loop {
            let (mut moved, mut made) = (0, 0);
            while moved < FLUSH_STEP {
                let Some(i) = self.next_waiting() else {
                    break;
                };
                self.spill_from = i;
                made = made.max(self.spill_oldest(i));
                moved += 1;
            }
            if let Some(n) = self.fit(size, OWN_BIN_LOOK) {
                return Some(self.carve(n, size));
            }
            if let Some(a) = self.grow(size) {
                return Some(a);
            }
            if made >= size {
                // It still fits: blocks in bins only grow by merging, and one merged into the
                // top would have let the top serve the request.
                let n = self
                    .fit(size, u32::MAX)
                    .expect("the block a batch made fits");
                return Some(self.carve(n, size));
            }
            if moved < FLUSH_STEP || self.quick_bytes == 0 {
                break;
            }
        }
        self.failed_flush = waiting;
        None
    }

    /// The free block a request of `size` bytes takes: in its own bin the newest of exactly
    /// that size or else the newest that fits (among the first `look` blocks of the bin), else
    /// the newest of the next bin above that holds one. Requests look at the first
    /// [`OWN_BIN_LOOK`], so that a bin of many smaller blocks does not slow every request down;
    /// only a request the heap would otherwise fail looks at the whole bin.
    #[inline]
    fn fit(&self, size: u32, look: u32) -> Option<u32> {
        let bin = bin_of(size);
        let mut n = self.heads[bin];
        let mut fits = NONE;
        let mut looked = 0;
        while n != NONE && looked < look {
            let node = &self.nodes[n as usize];
            if node.size == size {
                return Some(n);
            }
            if node.size > size && fits == NONE {
                fits = n;
            }
            n = node.next;
            looked += 1;
        }
        if fits != NONE {
            return Some(fits);
        }
        self.first_bin_from(bin + 1).map(|b| self.heads[b])
    }

    /// The first bin at or after `bin` that holds a block.
    #[inline]
    fn first_bin_from(&self, bin: usize) -> Option<usize> {
        let word = bin / 64;
        if word >= WORDS {
            return None;
        }
        let here = self.words[word] & (!0u64 << (bin % 64));
        if here != 0 {
            return Some(word * 64 + here.trailing_zeros() as usize);
        }
        let later = self.nonempty & (!0u32 << word << 1);
        if later == 0 {
            return None;
        }
        let word = later.trailing_zeros() as usize;
        Some(word * 64 + self.words[word].trailing_zeros() as usize)
    }

    /// Hands out the first `size` bytes of the free block `n`; the rest of it stays free.
    fn carve(&mut self, n: u32, size: u32) -> u32 {
        self.unlink(n);
        let Node {
            start, size: have, ..
        } = self.nodes[n as usize];
        self.by_start.remove(&start);
        self.free_bytes -= u64::from(size);
        if have == size {
            self.by_end.remove(&(start + have));
            self.nodes.release(n);
        } else {
            // The rest keeps its end, so only its start moves; its right neighbour is not free
            // (free blocks are merged), so there is nothing to merge.
            let rest = start + size;
            let node = &mut self.nodes[n as usize];
            node.start = rest;
            node.size = have - size;
            self.by_start.insert(rest, n);
            self.link(n);
        }
        start
    }

    /// `size` fresh bytes at the top, or `None` past the end of the top's range
    /// ([`HEAP_LOW_END`] or [`HEAP_END`]).
    #[inline]
    fn grow(&mut self, size: u32) -> Option<u32> {
        let a = self.top;
        let end = if a <= HEAP_LOW_END {
            HEAP_LOW_END
        } else {
            HEAP_END
        };
        if u64::from(a) + u64::from(size) > u64::from(end) {
            return None;
        }
        self.top = a + size;
        self.peak = self.peak.max(self.top);
        Some(a)
    }

    /// Moves the top from the lower range to the upper one for a request of `size` bytes that
    /// the lower range cannot serve ([`Heap::take_in_range`] failed there): the rest of the
    /// lower range, above the top, goes into the bins as a free block (merged with a free block
    /// below it, which by then cannot be one in a bin: none ends at the top), and the request
    /// is served at [`HEAP_HIGH_BASE`]. `None` when the top is already in the upper range or
    /// the request is bigger than that range. A vanilla preset (a plain 32-bit process, not
    /// built yet) would stop here: answer `None` so the heap ends at [`HEAP_LOW_END`].
    #[cold]
    #[inline(never)]
    fn cross(&mut self, size: u32) -> Option<u32> {
        if self.top > HEAP_LOW_END || size > HEAP_END - HEAP_HIGH_BASE {
            return None;
        }
        let rest = self.top;
        self.top = HEAP_HIGH_BASE;
        if rest < HEAP_LOW_END {
            self.release(rest, HEAP_LOW_END - rest);
        }
        self.grow(size)
    }

    /// Takes the free block in a bin that ends at `end` out of the bins, if there is one, and
    /// answers where the free space ending at `end` starts (its start, else `end`): the top
    /// comes down to it.
    fn merge_below_top(&mut self, end: u32) -> u32 {
        let Some(l) = self.by_end.remove(&end) else {
            return end;
        };
        self.unlink(l);
        let left = self.nodes[l as usize];
        self.by_start.remove(&left.start);
        self.free_bytes -= u64::from(left.size);
        self.nodes.release(l);
        left.start
    }

    /// Takes back the block at `addr` of class `class` ([`Heap::take`]): into its class's quick
    /// stack when it is small, else into the bins ([`Heap::release`]).
    #[inline]
    pub(super) fn give(&mut self, addr: u32, class: u32) {
        let size = class_bytes(class);
        if class <= self.quick_max {
            let i = (class >> 2) as usize;
            self.quick[i].push_back(addr);
            self.waiting[i / 64] |= 1 << (i % 64);
            self.quick_bytes += u64::from(size);
            if self.quick_bytes > QUICK_BYTES {
                self.spill(class);
            }
            return;
        }
        self.release(addr, size);
    }

    /// Brings the quick stacks back to at most [`QUICK_BYTES`] after a give of class `class` put
    /// them over: the oldest block of the next stack that holds one goes on to the bins, stack
    /// after stack, up to [`SPILL_STEP`] blocks; if the stacks are still over, the oldest block of
    /// `class` goes too. That one holds at least the bytes the give added (the stacks held
    /// at most [`QUICK_BYTES`] before it; were the block just given already gone, the stacks
    /// would be under), so a give moves at most [`SPILL_STEP`] + 1 blocks and leaves the stacks
    /// at most full.
    #[cold]
    #[inline(never)]
    fn spill(&mut self, class: u32) {
        for _ in 0..SPILL_STEP {
            let Some(i) = self.next_waiting() else {
                return;
            };
            self.spill_from = i;
            self.spill_oldest(i);
            if self.quick_bytes <= QUICK_BYTES {
                return;
            }
        }
        self.spill_oldest((class >> 2) as usize);
    }

    /// The first quick stack after the one [`Heap::spill`] took from last (round the sizes) that
    /// holds a block, `None` when none does: the first bit set in [`Heap::waiting`] after it,
    /// else the first one from the start. With one size waiting, looking at every stack in turn
    /// (as before) passed 511 empty ones a block, which made a burst of frees past the limit
    /// take 70-110% longer (8 to 1024 bytes, measured 2026-11-01).
    #[inline]
    fn next_waiting(&self) -> Option<usize> {
        let from = self.spill_from + 1;
        let word = from / 64;
        if word < STACK_WORDS {
            let here = self.waiting[word] & (!0u64 << (from % 64));
            if here != 0 {
                return Some(word * 64 + here.trailing_zeros() as usize);
            }
        }
        // The words after it, then round from the first (its own word again, for the bits
        // below `from`).
        (word + 1..STACK_WORDS)
            .chain(0..=word.min(STACK_WORDS - 1))
            .find(|&w| self.waiting[w] != 0)
            .map(|w| w * 64 + self.waiting[w].trailing_zeros() as usize)
    }

    /// Moves the oldest block of quick stack `i` (if it holds one) on to the bins; answers the
    /// size of the free block that made in a bin ([`Heap::release`]; 0 for none).
    fn spill_oldest(&mut self, i: usize) -> u32 {
        let Some(a) = self.quick[i].pop_front() else {
            return 0;
        };
        if self.quick[i].is_empty() {
            self.waiting[i / 64] &= !(1 << (i % 64));
        }
        let size = class_bytes((i as u32) << 2);
        self.quick_bytes -= u64::from(size);
        self.release(a, size)
    }

    /// Puts the block `[addr, addr + size)` into the bins, merged with the free blocks on
    /// either side, and answers the size of the free block that makes; a block that reaches the
    /// top lowers the top instead (answering 0). A top that comes down to the start of the
    /// upper range goes back to the lower range's end and comes down further over the free block
    /// that ends there.
    pub(super) fn release(&mut self, addr: u32, size: u32) -> u32 {
        let end = addr + size;
        if end == self.top {
            // No free block in a bin ends at the top, so only the left one can merge.
            self.top = self.merge_below_top(addr);
            if self.top == HEAP_HIGH_BASE {
                self.top = self.merge_below_top(HEAP_LOW_END);
            }
            return 0;
        }
        self.free_bytes += u64::from(size);
        let left = self.by_end.remove(&addr);
        let right = self.by_start.remove(&end);
        let n = match (left, right) {
            (None, None) => {
                let n = self.new_node(addr, size);
                self.by_start.insert(addr, n);
                self.by_end.insert(end, n);
                n
            }
            (Some(l), None) => {
                // The left block keeps its start and grows to the freed block's end.
                self.unlink(l);
                self.nodes[l as usize].size += size;
                self.by_end.insert(end, l);
                l
            }
            (None, Some(r)) => {
                // The right block keeps its end and grows down to the freed block's start.
                self.unlink(r);
                let node = &mut self.nodes[r as usize];
                node.start = addr;
                node.size += size;
                self.by_start.insert(addr, r);
                r
            }
            (Some(l), Some(r)) => {
                self.unlink(l);
                self.unlink(r);
                let right = self.nodes[r as usize];
                self.nodes[l as usize].size += size + right.size;
                *self
                    .by_end
                    .get_mut(&(right.start + right.size))
                    .expect("a free block's end is listed") = l;
                self.nodes.release(r);
                l
            }
        };
        self.link(n);
        self.nodes[n as usize].size
    }

    fn new_node(&mut self, start: u32, size: u32) -> u32 {
        let node = Node {
            start,
            size,
            prev: NONE,
            next: NONE,
        };
        self.nodes.alloc(node)
    }

    /// Puts node `n` at the head of its size's bin.
    #[inline]
    fn link(&mut self, n: u32) {
        let bin = bin_of(self.nodes[n as usize].size);
        let head = self.heads[bin];
        let node = &mut self.nodes[n as usize];
        node.prev = NONE;
        node.next = head;
        if head != NONE {
            self.nodes[head as usize].prev = n;
        }
        self.heads[bin] = n;
        self.words[bin / 64] |= 1 << (bin % 64);
        self.nonempty |= 1 << (bin / 64);
    }

    /// Takes node `n` out of its bin (its size must be the one it was linked with).
    #[inline]
    fn unlink(&mut self, n: u32) {
        let Node {
            size, prev, next, ..
        } = self.nodes[n as usize];
        if next != NONE {
            self.nodes[next as usize].prev = prev;
        }
        if prev != NONE {
            self.nodes[prev as usize].next = next;
            return;
        }
        let bin = bin_of(size);
        self.heads[bin] = next;
        if next == NONE {
            self.words[bin / 64] &= !(1 << (bin % 64));
            if self.words[bin / 64] == 0 {
                self.nonempty &= !(1 << (bin / 64));
            }
        }
    }

    /// Bytes and blocks in the bins.
    pub(super) fn free_bytes(&self) -> u64 {
        self.free_bytes
    }
    pub(super) fn free_blocks(&self) -> usize {
        self.by_start.len()
    }

    /// The bytes the heap can still hand out: its free blocks, the quick stacks and the fresh
    /// space above the top, in both ranges.
    pub(super) fn available(&self) -> u64 {
        let fresh = if self.top <= HEAP_LOW_END {
            u64::from(HEAP_LOW_END - self.top) + u64::from(HEAP_END - HEAP_HIGH_BASE)
        } else {
            u64::from(HEAP_END - self.top)
        };
        self.free_bytes + self.quick_bytes + fresh
    }

    /// Bytes waiting in the quick stacks (only the tests ask; the engine reads the count
    /// through take_free's failed flush).
    #[cfg(test)]
    pub(super) fn quick_bytes(&self) -> u64 {
        self.quick_bytes
    }
    /// Blocks waiting in the quick stacks.
    pub(super) fn quick_blocks(&self) -> usize {
        self.quick.iter().map(VecDeque::len).sum()
    }

    /// The bytes and blocks that waited in the quick stacks when the last request that failed
    /// moved them all to the bins before giving up ((0, 0) when none waited): what the stacks
    /// held when the request came, which the stacks themselves no longer show.
    pub(super) fn failed_flush(&self) -> (u64, usize) {
        self.failed_flush
    }

    /// The largest free block in the bins (0 when there is none).
    pub(super) fn largest_free(&self) -> u32 {
        let Some(top_bin) = (0..WORDS).rev().find(|&w| self.words[w] != 0) else {
            return 0;
        };
        let bin = top_bin * 64 + 63 - self.words[top_bin].leading_zeros() as usize;
        let mut largest = 0;
        let mut n = self.heads[bin];
        while n != NONE {
            largest = largest.max(self.nodes[n as usize].size);
            n = self.nodes[n as usize].next;
        }
        largest
    }

    /// Every free block's size, in the bins and in the quick stacks (for the heap's report).
    pub(super) fn free_sizes(&self) -> impl Iterator<Item = u32> + '_ {
        let binned = self.by_start.values().map(|&n| self.nodes[n as usize].size);
        let waiting = self
            .quick
            .iter()
            .enumerate()
            .flat_map(|(i, s)| std::iter::repeat(class_bytes((i as u32) << 2)).take(s.len()));
        binned.chain(waiting)
    }

    /// Takes everything above the top, in both ranges, as one block per range (no guest
    /// memory is touched: the heap is bookkeeping): the tests of a full heap. Answers the
    /// blocks.
    #[cfg(test)]
    pub(super) fn take_rest(&mut self) -> Vec<(u32, u32)> {
        let mut taken: Vec<_> = self.take_lower_rest().into_iter().collect();
        if self.top <= HEAP_LOW_END {
            let all = HEAP_END - HEAP_HIGH_BASE;
            taken.push((self.cross(all).expect("the whole upper range"), all));
        } else if self.top < HEAP_END {
            let rest = HEAP_END - self.top;
            taken.push((self.grow(rest).expect("the upper range's rest"), rest));
        }
        taken
    }

    /// Takes the rest of the lower range above the top as one block, when the top is there
    /// and below its end (as [`Heap::take_rest`]).
    #[cfg(test)]
    pub(super) fn take_lower_rest(&mut self) -> Option<(u32, u32)> {
        if self.top >= HEAP_LOW_END {
            return None;
        }
        let rest = HEAP_LOW_END - self.top;
        Some((self.grow(rest).expect("the lower range's rest"), rest))
    }

    /// Turns the quick stacks off (tests of the bins alone).
    #[cfg(test)]
    pub(super) fn without_quick(mut self) -> Self {
        self.quick_max = 0;
        self
    }

    /// The free block in a bin that starts at `start`, as (start, size).
    #[cfg(test)]
    pub(super) fn free_block_at(&self, start: u32) -> Option<(u32, u32)> {
        self.by_start
            .get(&start)
            .map(|&n| (self.nodes[n as usize].start, self.nodes[n as usize].size))
    }

    /// Checks the bookkeeping: the bins, their lists and bitmaps, both maps, the bitmap of the
    /// quick stacks and the counts agree;
    /// no two free blocks in bins touch and none ends at the top; and the free blocks, the quick
    /// stacks and `held` (the live blocks and any held back by the freed-block check) cover
    /// `[HEAP_BASE, top)` exactly, without overlap, but for the platform's addresses between the
    /// ranges (with the top in the upper range, the lower one is covered to its end). Panics on
    /// the first disagreement.
    #[cfg(test)]
    pub(super) fn check(&self, held: impl IntoIterator<Item = (u32, u32)>) {
        let mut listed = 0;
        let mut binned = 0u64;
        for bin in 0..BINS {
            let mut prev = NONE;
            let mut n = self.heads[bin];
            assert_eq!(
                n != NONE,
                (self.words[bin / 64] >> (bin % 64)) & 1 == 1,
                "bitmap of bin {bin}"
            );
            while n != NONE {
                let node = self.nodes[n as usize];
                assert_eq!(
                    bin_of(node.size),
                    bin,
                    "block {:08x} in bin {bin}",
                    node.start
                );
                assert_eq!(node.prev, prev, "back link of node {n}");
                assert_eq!(self.by_start.get(&node.start), Some(&n));
                assert_eq!(self.by_end.get(&(node.start + node.size)), Some(&n));
                assert!(node.size >= 8 && node.size % 8 == 0 && node.start % 8 == 0);
                listed += 1;
                binned += u64::from(node.size);
                prev = n;
                n = node.next;
            }
        }
        for w in 0..WORDS {
            assert_eq!(
                self.words[w] != 0,
                (self.nonempty >> w) & 1 == 1,
                "word {w}"
            );
        }
        assert_eq!(listed, self.by_start.len());
        assert_eq!(listed, self.by_end.len());
        assert_eq!(binned, self.free_bytes);
        assert_eq!(listed + self.nodes.spare_len, self.nodes.len);
        // 0 live, 1 free in a bin, 2 waiting in a quick stack.
        let mut all: Vec<(u32, u32, u8)> = held.into_iter().map(|(a, s)| (a, s, 0)).collect();
        all.extend(self.by_start.values().map(|&n| {
            let node = self.nodes[n as usize];
            (node.start, node.size, 1)
        }));
        let mut waiting = 0u64;
        for (i, stack) in self.quick.iter().enumerate() {
            assert_eq!(
                !stack.is_empty(),
                (self.waiting[i / 64] >> (i % 64)) & 1 == 1,
                "bitmap of quick stack {i}"
            );
            for &a in stack {
                let size = class_bytes((i as u32) << 2);
                all.push((a, size, 2));
                waiting += u64::from(size);
            }
        }
        assert_eq!(waiting, self.quick_bytes);
        all.sort_unstable();
        let mut at = HEAP_BASE;
        let mut last_kind = 0;
        for &(a, s, kind) in &all {
            if a >= HEAP_HIGH_BASE && at <= HEAP_LOW_END {
                // The platform's addresses between the ranges: the lower range is covered to
                // its end, and nothing touches across them.
                assert_eq!(
                    at, HEAP_LOW_END,
                    "the lower range is covered to {at:08x} only"
                );
                at = HEAP_HIGH_BASE;
                last_kind = 0;
            }
            assert_eq!(
                a, at,
                "a gap or an overlap at {at:08x} (block {a:08x}+{s:x})"
            );
            assert!(
                !(kind == 1 && last_kind == 1),
                "free blocks touch at {a:08x} without being merged"
            );
            at = a + s;
            last_kind = kind;
        }
        assert_eq!(
            at, self.top,
            "the blocks end at {at:08x}, the top is {:08x}",
            self.top
        );
        assert!(last_kind != 1, "a free block in a bin ends at the top");
        assert!(self.peak >= self.top);
        assert!(
            self.top <= HEAP_LOW_END || (self.top > HEAP_HIGH_BASE && self.top <= HEAP_END),
            "the top {:08x} is outside the heap's ranges",
            self.top
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The bins: one per size below 512, then 32 per power of two, in increasing order of size.
    #[test]
    fn bins_are_exact_below_512_then_32_per_power_of_two() {
        assert_eq!(bin_of(8), 1);
        assert_eq!(bin_of(504), 63);
        assert_eq!(bin_of(512), 64);
        assert_eq!(bin_of(520), 64);
        assert_eq!(bin_of(528), 65);
        assert_eq!(bin_of(1016), 95);
        assert_eq!(bin_of(1024), 96);
        assert_eq!(bin_of(1056), 97);
        assert_eq!(bin_of(0xffff_fff8), BINS - 1);
        let mut last = 0;
        for size in (8..1 << 20).step_by(8) {
            let b = bin_of(size);
            assert!(b == last || b == last + 1, "{size}: {b} after {last}");
            last = b;
        }
    }

    /// A bigger free block is split: the request takes its start, the rest stays free.
    #[test]
    fn a_bigger_free_block_is_split_from_its_start() {
        let mut h = Heap::new().without_quick();
        let a = h.take(0x1000).unwrap();
        let b = h.take(0x40).unwrap();
        h.release(a, 0x1000);
        assert_eq!(h.take(0x300), Some(a));
        assert_eq!(h.free_block_at(a + 0x300), Some((a + 0x300, 0xd00)));
        assert_eq!(h.take(0xd00), Some(a + 0x300));
        assert_eq!(h.free_blocks(), 0);
        h.check([(a, 0x300), (a + 0x300, 0xd00), (b, 0x40)]);
    }

    /// A freed block merges with a free block on its left, on its right, and on both sides.
    #[test]
    fn freed_blocks_merge_with_free_neighbours() {
        let mut h = Heap::new().without_quick();
        let blocks: Vec<u32> = (0..7).map(|_| h.take(0x100).unwrap()).collect();
        // Left: 0 then 1.
        h.release(blocks[0], 0x100);
        h.release(blocks[1], 0x100);
        assert_eq!(h.free_block_at(blocks[0]), Some((blocks[0], 0x200)));
        // Right: 4 then 3.
        h.release(blocks[4], 0x100);
        h.release(blocks[3], 0x100);
        assert_eq!(h.free_block_at(blocks[3]), Some((blocks[3], 0x200)));
        assert_eq!(h.free_block_at(blocks[4]), None);
        // Both: 2 joins [0, 2) and [3, 5).
        h.release(blocks[2], 0x100);
        assert_eq!(h.free_block_at(blocks[0]), Some((blocks[0], 0x500)));
        assert_eq!(h.free_blocks(), 1);
        h.check([(blocks[5], 0x100), (blocks[6], 0x100)]);
        // The merged block serves one request of its whole size.
        assert_eq!(h.take(0x500), Some(blocks[0]));
        assert_eq!(h.free_blocks(), 0);
    }

    /// A freed block at the top lowers the top, together with a free block below it.
    #[test]
    fn a_freed_block_at_the_top_lowers_it() {
        let mut h = Heap::new().without_quick();
        let a = h.take(0x800).unwrap();
        let b = h.take(0x800).unwrap();
        let c = h.take(0x800).unwrap();
        assert_eq!(h.top(), c + 0x800);
        h.release(b, 0x800);
        h.release(c, 0x800);
        assert_eq!(h.top(), b);
        assert_eq!(h.free_blocks(), 0);
        assert_eq!(h.peak(), c + 0x800);
        h.release(a, 0x800);
        assert_eq!(h.top(), HEAP_BASE);
        h.check([]);
        // Fresh blocks come from the lowered top again.
        assert_eq!(h.take(0x10), Some(HEAP_BASE));
    }

    /// The top stays in the lower range while anything there can serve a request, the quick
    /// stacks included; only a request nothing there serves moves it to the upper range, and
    /// the lower range's rest becomes a free block that later requests take first.
    #[test]
    fn the_top_moves_up_only_when_the_lower_range_cannot_serve() {
        let mut h = Heap::new();
        let a = h.take(0x40).unwrap();
        let b = h.take(0x40).unwrap();
        let room = HEAP_LOW_END - h.top() - 0x40;
        let big = h.take(room).unwrap();
        let mut held = vec![(big, room)];
        assert_eq!(h.top(), HEAP_LOW_END - 0x40);
        // 0x80 fits once the two waiting blocks merge: still the lower range.
        h.give(a, 0x40);
        h.give(b, 0x40);
        assert_eq!(h.take(0x80), Some(a));
        held.push((a, 0x80));
        assert_eq!(
            h.take(0x40),
            Some(HEAP_LOW_END - 0x40),
            "the top's last bytes"
        );
        held.push((HEAP_LOW_END - 0x40, 0x40));
        assert_eq!(h.top(), HEAP_LOW_END);
        h.check(held.iter().copied());
        // Nothing in the lower range is left: the next request goes up.
        assert_eq!(h.take(0x10), Some(HEAP_HIGH_BASE));
        held.push((HEAP_HIGH_BASE, 0x10));
        assert_eq!(h.top(), HEAP_HIGH_BASE + 0x10);
        h.check(held.iter().copied());
        // A request bigger than the upper range fails and leaves the heap as it was.
        assert_eq!(h.take(HEAP_END - HEAP_HIGH_BASE), None);
        assert_eq!(h.top(), HEAP_HIGH_BASE + 0x10);
        h.check(held);
    }

    /// When the top moves up, the lower range's rest above it goes into the bins (merged with
    /// nothing: the platform's addresses are not free space) and is taken before the top; the
    /// upper range emptying brings the top back down over it.
    #[test]
    fn the_lower_range_rest_is_a_free_block_and_the_top_comes_back() {
        let mut h = Heap::new().without_quick();
        let room = HEAP_LOW_END - HEAP_BASE - 0x1000;
        let big = h.take(room).unwrap();
        let up = h.take(0x2000).unwrap();
        assert_eq!(up, HEAP_HIGH_BASE);
        let rest = HEAP_LOW_END - 0x1000;
        assert_eq!(h.free_block_at(rest), Some((rest, 0x1000)));
        h.check([(big, room), (up, 0x2000)]);
        // The lower free block comes before the top.
        assert_eq!(h.take(0x800), Some(rest));
        let next = h.take(0x100).unwrap();
        assert_eq!(next, rest + 0x800);
        let high = h.take(0x1000).unwrap();
        assert_eq!(high, up + 0x2000, "the lower block had no room left for it");
        h.release(rest, 0x800);
        h.release(next, 0x100);
        h.check([(big, room), (up, 0x2000), (high, 0x1000)]);
        // The upper range empties: the top goes back to the lower range's end and over the
        // free block that ends there.
        h.release(up, 0x2000);
        assert_eq!(h.top(), HEAP_HIGH_BASE + 0x3000);
        h.release(high, 0x1000);
        assert_eq!(h.top(), rest);
        assert_eq!(h.free_blocks(), 0);
        assert_eq!(h.peak(), HEAP_HIGH_BASE + 0x3000);
        h.check([(big, room)]);
        h.release(big, room);
        assert_eq!(h.top(), HEAP_BASE);
        h.check([]);
        // And the lower range is used first again.
        assert_eq!(h.take(0x10), Some(HEAP_BASE));
    }

    /// Blocks of one size come back newest first, as the old exact-size lists gave them, both
    /// from the quick stacks and from exact bins (which a block reaches when the quick stacks
    /// let it go).
    #[test]
    fn same_size_reuse_is_newest_first() {
        for (size, quick) in [(0x20, true), (0x180, true), (0x180, false)] {
            let mut h = if quick {
                Heap::new()
            } else {
                Heap::new().without_quick()
            };
            let blocks: Vec<u32> = (0..6).map(|_| h.take(size).unwrap()).collect();
            // Every other block, so none merge.
            for a in [blocks[0], blocks[2], blocks[4]] {
                h.give(a, size);
            }
            assert_eq!(h.take(size), Some(blocks[4]));
            assert_eq!(h.take(size), Some(blocks[2]));
            assert_eq!(h.take(size), Some(blocks[0]));
            assert_eq!(h.take(size), Some(blocks[5] + size));
        }
    }

    /// In a bin of several sizes, a block of exactly the size asked for is taken before a newer
    /// one that is bigger.
    #[test]
    fn an_exact_block_is_taken_before_a_bigger_newer_one_in_the_bin() {
        let mut h = Heap::new().without_quick();
        let a = h.take(592).unwrap();
        let _ = h.take(8).unwrap();
        let b = h.take(600).unwrap();
        let _ = h.take(8).unwrap();
        assert_eq!(bin_of(592), bin_of(600));
        h.release(a, 592);
        h.release(b, 600);
        assert_eq!(h.take(592), Some(a));
        assert_eq!(h.take(592), Some(b));
        assert_eq!(h.free_block_at(b + 592), Some((b + 592, 8)));
    }

    /// Freed blocks of up to 4 KB (the old exact-size lists' sizes) wait in the quick stacks;
    /// bigger ones go to the bins.
    #[test]
    fn blocks_up_to_4k_wait_in_the_quick_stacks() {
        let mut h = Heap::new();
        let a = h.take(0x1000).unwrap();
        let b = h.take(0x1008).unwrap();
        let last = h.take(8).unwrap();
        h.give(a, 0x1000);
        h.give(b, 0x1008);
        assert_eq!((h.quick_blocks(), h.quick_bytes()), (1, 0x1000));
        assert_eq!(h.free_block_at(b), Some((b, 0x1008)));
        assert_eq!(h.take(0x1000), Some(a));
        h.check([(a, 0x1000), (last, 8)]);
    }

    /// A block of `size` bytes taken with a live 8-byte block after it (into `held`), so that
    /// freed blocks taken this way never merge with one another.
    fn take_apart(h: &mut Heap, size: u32, held: &mut Vec<(u32, u32)>) -> u32 {
        let a = h.take(size).unwrap();
        held.push((h.take(8).unwrap(), 8));
        a
    }

    /// Past [`QUICK_BYTES`] a give moves the oldest waiting block to the bins, not a share of
    /// every stack: with one size waiting, one block a give, the oldest first, and the stacks
    /// stay at the limit. The newest blocks still wait and come back first.
    #[test]
    fn a_give_past_the_limit_moves_the_oldest_waiting_block() {
        let mut h = Heap::new();
        let mut held = Vec::new();
        let n = (QUICK_BYTES / 0x100) as usize + 64;
        let blocks: Vec<u32> = (0..n)
            .map(|_| take_apart(&mut h, 0x100, &mut held))
            .collect();
        for (i, &a) in blocks.iter().enumerate() {
            let waiting = h.quick_blocks();
            h.give(a, 0x100);
            // The first QUICK_BYTES / 0x100 blocks fill the stacks to the limit exactly.
            let moved = waiting + 1 - h.quick_blocks();
            assert_eq!(moved, usize::from(i >= n - 64), "give {i}");
            assert!(h.quick_bytes() <= QUICK_BYTES, "give {i}");
        }
        assert_eq!(h.free_blocks(), 64);
        for &a in &blocks[..64] {
            assert_eq!(h.free_block_at(a), Some((a, 0x100)));
        }
        assert_eq!(h.free_block_at(blocks[64]), None);
        let newest = *blocks.last().unwrap();
        assert_eq!(h.take(0x100), Some(newest));
        held.push((newest, 0x100));
        h.check(held);
    }

    /// A give that puts the stacks over the limit moves at most [`SPILL_STEP`] + 1 blocks: the
    /// oldest of each size in turn, then, while the stacks are still over the limit, the oldest
    /// of its own size. Here a 4 KB block goes over by more than the 16 smallest waiting blocks
    /// hold.
    #[test]
    fn a_big_give_over_the_limit_also_moves_the_oldest_of_its_size() {
        let mut h = Heap::new();
        let mut held = Vec::new();
        // One block of each size from 8 to 136 bytes, then 4 KB blocks up to the limit.
        let small: Vec<(u32, u32)> = (1..=17)
            .map(|k| (take_apart(&mut h, k * 8, &mut held), k * 8))
            .collect();
        let small_bytes: u64 = small.iter().map(|&(_, s)| u64::from(s)).sum();
        let big_n = ((QUICK_BYTES - small_bytes) / 0x1000) as usize;
        let big: Vec<u32> = (0..=big_n)
            .map(|_| take_apart(&mut h, 0x1000, &mut held))
            .collect();
        for &(a, s) in &small {
            h.give(a, s);
        }
        for &a in &big[..big_n] {
            h.give(a, 0x1000);
        }
        assert_eq!(h.quick_blocks(), 17 + big_n, "nothing moved yet");
        let waiting = h.quick_blocks();
        h.give(big[big_n], 0x1000);
        assert_eq!(waiting + 1 - h.quick_blocks(), SPILL_STEP + 1);
        assert!(h.quick_bytes() <= QUICK_BYTES);
        for &(a, s) in &small[..16] {
            assert_eq!(h.free_block_at(a), Some((a, s)));
        }
        assert_eq!(h.free_block_at(small[16].0), None, "136 bytes still wait");
        assert_eq!(h.free_block_at(big[0]), Some((big[0], 0x1000)));
        assert_eq!(h.take(0x1000), Some(big[big_n]));
        held.push((big[big_n], 0x1000));
        h.check(held);
    }

    /// When nothing else fits, the quick stacks go to the bins, merge, and serve the request.
    #[test]
    fn a_full_heap_merges_the_quick_stacks_before_giving_up() {
        let mut h = Heap::new();
        let a = h.take(0x40).unwrap();
        let b = h.take(0x40).unwrap();
        let rest = h.take_rest();
        assert_eq!(h.take(8), None);
        h.give(a, 0x40);
        h.give(b, 0x40);
        assert_eq!(h.quick_bytes(), 0x80);
        assert_eq!(h.take(0x80), Some(a));
        assert_eq!(h.quick_bytes(), 0);
        h.check([(a, 0x80)].into_iter().chain(rest.iter().copied()));
    }

    /// A request that only the waiting blocks can serve moves them to the bins a batch at a
    /// time and stops at the first batch that serves it: the rest still wait.
    #[test]
    fn a_full_heap_moves_only_the_waiting_blocks_it_needs() {
        let mut h = Heap::new();
        let n = 2 * FLUSH_STEP + 2;
        let blocks: Vec<u32> = (0..n).map(|_| h.take(16).unwrap()).collect();
        let rest = h.take_rest();
        assert_eq!(h.take(8), None);
        // Oldest first: the first batch is the first FLUSH_STEP blocks, which merge into one.
        for &a in &blocks {
            h.give(a, 16);
        }
        assert_eq!(h.quick_blocks(), n);
        assert_eq!(h.take(32), Some(blocks[0]));
        assert_eq!(h.quick_blocks(), n - FLUSH_STEP, "one batch moved");
        assert_eq!(
            h.free_block_at(blocks[0] + 32),
            Some((blocks[0] + 32, FLUSH_STEP as u32 * 16 - 32))
        );
        h.check([(blocks[0], 32)].into_iter().chain(rest.iter().copied()));
        // A request the waiting blocks cannot serve moves all of them, then fails.
        assert_eq!(h.take(0x10_0000), None);
        assert_eq!(h.quick_blocks(), 0);
        assert_eq!(
            h.failed_flush(),
            ((n - FLUSH_STEP) as u64 * 16, n - FLUSH_STEP)
        );
        h.check([(blocks[0], 32)].into_iter().chain(rest.iter().copied()));
    }

    /// A full heap whose quick stacks hold a 592-byte block X and the 8-byte block right after
    /// it (the oldest of their sizes, so the first batch moves them first and they merge into a
    /// 600-byte free block), then `smaller` more 592-byte blocks (the bin of 600) and `tiny`
    /// more 8-byte ones, each apart. Answers how many blocks `take(600)` moved to the bins,
    /// where it was served, and X. No 600-byte block waits, so the request goes to the bins.
    fn buried_fit(smaller: usize, tiny: usize) -> (usize, Option<u32>, u32) {
        let mut h = Heap::new();
        let mut held = Vec::new();
        assert_eq!(bin_of(592), bin_of(600));
        let x = h.take(592).unwrap();
        let y = take_apart(&mut h, 8, &mut held);
        assert_eq!(y, x + 592);
        let small: Vec<u32> = (0..smaller)
            .map(|_| take_apart(&mut h, 592, &mut held))
            .collect();
        let little: Vec<u32> = (0..tiny)
            .map(|_| take_apart(&mut h, 8, &mut held))
            .collect();
        held.extend(h.take_rest());
        h.give(y, 8);
        h.give(x, 592);
        for &a in &small {
            h.give(a, 592);
        }
        for &a in &little {
            h.give(a, 8);
        }
        assert_eq!(h.quick_blocks(), 2 + smaller + tiny, "none moved yet");
        assert_eq!(h.free_blocks(), 0);
        let before = h.quick_blocks();
        let got = h.take(600);
        if got == Some(x) {
            held.push((x, 600));
        }
        h.check(held);
        (before - h.quick_blocks(), got, x)
    }

    /// The first batch that makes a block a request fits serves it, even when the batch also
    /// puts more than [`OWN_BIN_LOOK`] newer, smaller blocks of its bin in front of that block:
    /// a 4096-byte block moved from the quick stacks lands in the bin of a 4104-byte request
    /// the same way. (With only the usual look after each batch, every waiting block moved
    /// before a last look at the whole bin found it: in the review, with the stacks full of
    /// 8-byte blocks, 1,047,846 blocks in 66-76 ms.)
    #[test]
    fn a_fit_behind_newer_blocks_of_its_bin_is_served_by_the_first_batch() {
        let tiny = 3 * FLUSH_STEP;
        for smaller in [OWN_BIN_LOOK as usize - 1, OWN_BIN_LOOK as usize + 1, 100] {
            let (moved, got, fitting) = buried_fit(smaller, tiny);
            assert_eq!(got, Some(fitting), "{smaller} smaller");
            assert_eq!(moved, FLUSH_STEP, "{smaller} smaller: one batch");
        }
    }

    /// Free blocks that never merge (each with a live block after it) fill the tables of their
    /// start and end region by region: no region's table holds more than 16K, the most a
    /// 256 KB region can hold, however many there are in all.
    #[test]
    fn the_free_block_tables_stay_small_region_by_region() {
        let mut h = Heap::new().without_quick();
        let mut held = Vec::new();
        let per_region = 1usize << (REGION_BITS - 4);
        let blocks: Vec<u32> = (0..3 * per_region + 5)
            .map(|_| take_apart(&mut h, 8, &mut held))
            .collect();
        for &a in &blocks {
            h.release(a, 8);
        }
        assert_eq!(h.free_blocks(), blocks.len());
        assert_eq!(h.by_start.most_in_one(), per_region);
        assert_eq!(h.by_end.most_in_one(), per_region);
        h.check(held);
    }

    /// The quick stacks move to the bins round the sizes, the oldest block of each, starting
    /// after the size they last moved and wrapping round from the smallest.
    #[test]
    fn moves_to_the_bins_go_round_the_sizes() {
        let mut h = Heap::new();
        let mut held = Vec::new();
        // One block each of 8, 16 and 4096 bytes, then 4 KB blocks up to 24 bytes short of the
        // limit.
        let firsts: Vec<u32> = [8u32, 16, 0x1000]
            .iter()
            .map(|&s| take_apart(&mut h, s, &mut held))
            .collect();
        let big_n = ((QUICK_BYTES - 8 - 16 - 0x1000) / 0x1000) as usize;
        let big: Vec<u32> = (0..big_n + 3)
            .map(|_| take_apart(&mut h, 0x1000, &mut held))
            .collect();
        for (&a, s) in firsts.iter().zip([8, 16, 0x1000]) {
            h.give(a, s);
        }
        for &a in &big[..big_n] {
            h.give(a, 0x1000);
        }
        assert_eq!(QUICK_BYTES - h.quick_bytes(), 0x1000 - 24);
        assert_eq!(h.free_blocks(), 0);
        // 24 bytes over: the 8-byte block, then the 16-byte one.
        h.give(big[big_n], 0x1000);
        assert_eq!(h.free_block_at(firsts[0]), Some((firsts[0], 8)));
        assert_eq!(h.free_block_at(firsts[1]), Some((firsts[1], 16)));
        assert_eq!(h.free_blocks(), 2);
        // After 16 bytes, the next size that holds a block is 4096: its oldest.
        h.give(big[big_n + 1], 0x1000);
        assert_eq!(h.free_block_at(firsts[2]), Some((firsts[2], 0x1000)));
        // After 4096, round from the smallest: only 4096 holds blocks, its oldest next.
        h.give(big[big_n + 2], 0x1000);
        assert_eq!(h.free_block_at(big[0]), Some((big[0], 0x1000)));
        assert_eq!(h.free_blocks(), 4);
        assert_eq!(h.quick_bytes(), QUICK_BYTES);
        h.check(held);
    }

    /// On a full heap, a free block that fits is found even behind more than [`OWN_BIN_LOOK`]
    /// newer blocks of its bin that are too small.
    #[test]
    fn a_full_heap_finds_a_fitting_block_deep_in_its_bin() {
        let mut h = Heap::new().without_quick();
        let fitting = h.take(600).unwrap();
        let mut held = vec![(fitting, 600)];
        let mut small = Vec::new();
        held.push((h.take(8).unwrap(), 8));
        for _ in 0..OWN_BIN_LOOK {
            small.push(h.take(592).unwrap());
            held.push((h.take(8).unwrap(), 8));
        }
        assert_eq!(bin_of(592), bin_of(600));
        held.extend(h.take_rest());
        assert_eq!(h.take(8), None);
        // The 8-byte blocks between them keep the freed blocks apart.
        h.release(fitting, 600);
        for &a in &small {
            h.release(a, 592);
        }
        held.retain(|&(a, _)| a != fitting && !small.contains(&a));
        h.check(held.iter().copied());
        assert_eq!(h.take(600), Some(fitting));
        assert_eq!(h.take(600), None);
        held.push((fitting, 600));
        h.check(held.iter().copied());
    }
}

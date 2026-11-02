//! `Havok SDK/run_00db0060` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The three functions are unnamed in the engine map. They belong to a sweep over boxes whose
//! corners are kept as sortable integers: `fn_00db0060` builds such a box record from a float
//! box, `fn_00db00f0` pairs up the overlapping boxes of two sorted lists, and `fn_00db06c0`
//! sorts a list of 0x20-byte records by their first word.

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router getter (`00c85750`, no arguments): its word at `+0xc` is the allocator
/// whose virtual function at `+0xc` allocates a block (it takes a pointer to the requested size
/// and writes back the size it gave) and at `+0x10` frees one (block, size).
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// The sort of `(key, index)` pairs (`00da0010`, cdecl: the pairs, their count, a scratch
/// area of the same size).
const SORT_PAIRS: u32 = 0x00da_0010;
/// The allocator's virtual functions (byte offsets in its vtable).
const SLOT_ALLOCATE: u32 = 0x0c;
const SLOT_FREE: u32 = 0x10;
/// The sign bit the box tests look at.
const SIGN_BIT: u32 = 0x8000_0000;

/// The sortable integer the box record keeps for a float's bits: the code flips the other 31
/// bits for a negative number (an arithmetic shift gives the sign, OR `0x80000001`, XOR the
/// word) and shifts the result right by one, so that unsigned order is float order.
fn sortable(bits: u32) -> u32 {
    ((((bits as i32) >> 31) as u32 | 0x8000_0001) ^ bits) >> 1
}

// Translated from 00db0060 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`: `this` = the record written, then `source`, `value`).
/// Turns the float box at `source` into the record at `this`: words 0 to 2 of the source are
/// made sortable (shifted right by one), the record's word 3 is `value`, and words 4 to 6 of
/// the source are made sortable the same way and increased by one.
pub fn fn_00db0060(e: &mut Engine, this: u32, source: u32, value: u32) {
    let word = e.mem.u32(source);
    e.mem.set_u32(this, sortable(word));
    let word = e.mem.u32(source + 4);
    e.mem.set_u32(this + 4, sortable(word));
    let word = e.mem.u32(source + 8);
    e.mem.set_u32(this + 8, sortable(word));
    e.mem.set_u32(this + 0xc, value);
    let word = e.mem.u32(source + 0x10);
    e.mem.set_u32(this + 0x10, sortable(word).wrapping_add(1));
    let word = e.mem.u32(source + 0x14);
    e.mem.set_u32(this + 0x14, sortable(word).wrapping_add(1));
    let word = e.mem.u32(source + 0x18);
    e.mem.set_u32(this + 0x18, sortable(word).wrapping_add(1));
}

/// Where the pairs of one box against a group of four go: the output cursor, its end (a pair
/// is two words) and the address of the counter of pairs that did not fit.
struct PairSink {
    cursor: u32,
    end: u32,
    overflow: u32,
}

impl PairSink {
    /// Writes the pair `(first, second)` when there is room, else counts it as lost.
    fn push(&mut self, e: &mut Engine, first: u32, second: u32) {
        if self.cursor < self.end {
            e.mem.set_u32(self.cursor, first);
            e.mem.set_u32(self.cursor + 4, second);
            self.cursor = self.cursor.wrapping_add(8);
        } else {
            let lost = e.mem.u32(self.overflow);
            e.mem.set_u32(self.overflow, lost.wrapping_add(1));
        }
    }
}

/// Runs the box `query` (word 4 is its largest x, words 1 and 2 its y and z minimum, words 5
/// and 6 the y and z maximum, word 3 its tag) against the groups of four 0x20-byte records
/// starting at `group` for as long as the next group's first word is below the query's
/// largest x. A record overlaps when the y and z intervals meet (all four differences of a
/// record have a clear sign bit). The pair is `(record tag, query tag)`, or `(query tag, record
/// tag)` when `query_first` is set. Records 1 to 3 of a group also have to start (word 0) at
/// or before the query's largest x; record 0 always passes (the loop only continues while it
/// does).
fn sweep_groups(
    e: &mut Engine,
    mut group: u32,
    query: u32,
    query_first: bool,
    sink: &mut PairSink,
) {
    let min_y = e.mem.u32(query + 4);
    let min_z = e.mem.u32(query + 8);
    let max_x = e.mem.u32(query + 0x10);
    let max_y = e.mem.u32(query + 0x14);
    let max_z = e.mem.u32(query + 0x18);
    loop {
        let node = |index: u32, word: u32| group + 0x20 * index + 4 * word;
        let mut miss = [0u32; 4];
        for index in 0..4u32 {
            let low_y = e.mem.u32(node(index, 1));
            let low_z = e.mem.u32(node(index, 2));
            let high_y = e.mem.u32(node(index, 5));
            let high_z = e.mem.u32(node(index, 6));
            miss[index as usize] = (max_y.wrapping_sub(low_y)
                | max_z.wrapping_sub(low_z)
                | high_y.wrapping_sub(min_y)
                | high_z.wrapping_sub(min_z))
                & SIGN_BIT;
        }
        if miss[0] & miss[1] & miss[2] & miss[3] == 0 {
            for index in 0..4u32 {
                if miss[index as usize] != 0 {
                    continue;
                }
                if index != 0 && e.mem.u32(node(index, 0)) > max_x {
                    continue;
                }
                let tag = e.mem.u32(node(index, 3));
                let own = e.mem.u32(query + 0xc);
                if query_first {
                    sink.push(e, own, tag);
                } else {
                    sink.push(e, tag, own);
                }
            }
        }
        group = group.wrapping_add(0x80);
        if e.mem.u32(group) >= max_x {
            break;
        }
    }
}

// Translated from 00db00f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl: `first`, `first_count`, `second`, `second_count`, `out`,
/// `max_pairs`, `overflow`). Sweeps two lists of 0x20-byte box records, each sorted by its
/// first word (the smallest x), and writes the tags of the overlapping boxes as pairs `(tag of
/// the first list's box, tag of the second list's box)` at `out`, at most `max_pairs` of them;
/// the pairs that did not fit are counted in the word at `overflow`. Answers the number of pairs
/// written.
///
/// Each step compares the first words of the two current records: when the first list's is
/// larger the second list's record is the query and is run against the first list's groups
/// (see `sweep_groups`) if the first list's first word is below its largest x, and the second
/// list advances by one record; otherwise the first list's record is the query against the
/// second list's groups (if the second's first word is below its largest x) and the first list
/// advances. The loop ends when the count of the list that advanced is no longer positive.
pub fn fn_00db00f0(
    e: &mut Engine,
    mut first: u32,
    mut first_count: i32,
    mut second: u32,
    mut second_count: i32,
    out: u32,
    max_pairs: i32,
    overflow: u32,
) -> i32 {
    let mut sink = PairSink {
        cursor: out,
        end: out.wrapping_add((max_pairs as u32).wrapping_mul(8)),
        overflow,
    };
    e.mem.set_u32(overflow, 0);
    loop {
        let first_start = e.mem.u32(first);
        let second_start = e.mem.u32(second);
        let remaining;
        if first_start > second_start {
            let query_end = e.mem.u32(second + 0x10);
            if first_start < query_end {
                sweep_groups(e, first, second, false, &mut sink);
            }
            second = second.wrapping_add(0x20);
            second_count = second_count.wrapping_sub(1);
            remaining = second_count;
        } else {
            let query_end = e.mem.u32(first + 0x10);
            if second_start < query_end {
                sweep_groups(e, second, first, true, &mut sink);
            }
            first = first.wrapping_add(0x20);
            first_count = first_count.wrapping_sub(1);
            remaining = first_count;
        }
        if remaining <= 0 {
            break;
        }
    }
    (sink.cursor.wrapping_sub(out) as i32) >> 3
}

/// Asks the allocator behind the memory router for `bytes` bytes. The allocator takes a pointer
/// to the size and writes back the size it gave; answers the block and the granted size.
fn allocate_block(e: &mut Engine, bytes: u32) -> (u32, i32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0xc);
    let request = e.mem.alloc(4);
    e.mem.set_u32(request, bytes);
    let block = e.vcall(allocator, SLOT_ALLOCATE, &args![request]).u32();
    let granted = e.mem.i32(request);
    e.mem.free(request);
    (block, granted)
}

/// The capacity word the code keeps for a granted size: `granted` divided by the unit (a
/// power of two, rounded toward zero), or `0x80000000` ("owns nothing") when that is zero.
fn capacity_word(granted: i32, unit_shift: u32) -> u32 {
    let mask = (1i32 << unit_shift) - 1;
    let units = granted.wrapping_add((granted >> 31) & mask) >> unit_shift;
    if units == 0 {
        0x8000_0000
    } else {
        units as u32
    }
}

/// Frees a block through the router's allocator (the router is asked first, as in the code),
/// unless the capacity word has its sign bit set. The size is the capacity (low 30 bits) times
/// the unit.
fn free_block(e: &mut Engine, block: u32, capacity: u32, unit_shift: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0xc);
    if (capacity as i32) >= 0 {
        let size = (capacity & 0x3fff_ffff) << unit_shift;
        e.vcall(allocator, SLOT_FREE, &args![block, size]);
    }
}

// Translated from 00db06c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl: `blocks`, `count`). Sorts `count` 0x20-byte records at
/// `blocks` by their first word, through three temporary blocks from the router's allocator.
///
/// With `rounded = (count + 3) & !3` it builds `rounded` pairs `(first word of record i, i)`
/// (reading records past `count` when `count` is not a multiple of four, as the code does),
/// sorts them with `00da0010` and releases that scratch; then it copies the records in the
/// sorted order into a second temporary block (`count << 5` bytes) and back over `blocks` as
/// `(count & 0x7ffffff) * 2` units of 16 bytes, and releases both blocks. Nothing is allocated
/// for a size of zero, and a capacity word with its sign bit set (which is what the code keeps
/// for "nothing") is not freed.
pub fn fn_00db06c0(e: &mut Engine, blocks: u32, count: u32) {
    let rounded = count.wrapping_add(3) & 0xffff_fffc;
    // The pairs, `rounded * 8` bytes.
    let (pairs, pairs_capacity) = if rounded == 0 {
        e.call(MEMORY_ROUTER, &args![]);
        (0, 0x8000_0000)
    } else {
        let (block, granted) = allocate_block(e, rounded.wrapping_mul(8));
        (block, capacity_word(granted, 3))
    };
    let mut index = 0i32;
    while index < rounded as i32 {
        let key = e
            .mem
            .u32(blocks.wrapping_add((index as u32).wrapping_mul(0x20)));
        let at = pairs.wrapping_add((index as u32).wrapping_mul(8));
        e.mem.set_u32(at, key);
        e.mem.set_u32(at + 4, index as u32);
        index += 1;
    }
    // The sort's scratch, the same size.
    let (scratch, scratch_capacity) = if rounded == 0 {
        e.call(MEMORY_ROUTER, &args![]);
        (0, 0x8000_0000)
    } else {
        let (block, granted) = allocate_block(e, rounded.wrapping_mul(8));
        (block, capacity_word(granted, 3))
    };
    e.call(SORT_PAIRS, &args![pairs, rounded, scratch]);
    free_block(e, scratch, scratch_capacity, 3);
    // The records in sorted order.
    let (copy, copy_capacity) = if count == 0 {
        e.call(MEMORY_ROUTER, &args![]);
        (0, 0x8000_0000)
    } else {
        let (block, granted) = allocate_block(e, count << 5);
        (block, capacity_word(granted, 5))
    };
    if (count as i32) > 0 {
        for slot in 0..count {
            let from = e
                .mem
                .u32(pairs.wrapping_add(slot.wrapping_mul(8)).wrapping_add(4));
            let source = blocks.wrapping_add(from << 5);
            let target = copy.wrapping_add(slot.wrapping_mul(0x20));
            for word in 0..8 {
                let value = e.mem.u32(source + 4 * word);
                e.mem.set_u32(target + 4 * word, value);
            }
        }
    }
    let units = (count << 5) >> 4;
    let mut unit = 0u32;
    while (unit as i32) < units as i32 {
        for word in 0..4 {
            let at = unit * 0x10 + 4 * word;
            let value = e.mem.u32(copy.wrapping_add(at));
            e.mem.set_u32(blocks.wrapping_add(at), value);
        }
        unit += 1;
    }
    free_block(e, copy, copy_capacity, 5);
    free_block(e, pairs, pairs_capacity, 3);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00db0060, fn_00db0060(u32, u32, u32)),
        entry!(0x00db00f0, fn_00db00f0(u32, i32, u32, i32, u32, i32, u32) -> i32),
        entry!(0x00db06c0, fn_00db06c0(u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn box_corners_become_sortable_integers() {
        let mut e = Engine::new();
        let source = e.mem.alloc(0x20);
        let this = e.mem.alloc(0x20);
        let words = [
            1.0f32.to_bits(),
            (-1.0f32).to_bits(),
            0.0f32.to_bits(),
            0,
            2.0f32.to_bits(),
            (-2.0f32).to_bits(),
            3.0f32.to_bits(),
        ];
        for (i, w) in words.into_iter().enumerate() {
            e.mem.set_u32(source + 4 * i as u32, w);
        }
        e.call(0x00db_0060, &args![this, source, 0x1234u32]);
        let got: Vec<u32> = (0..7).map(|i| e.mem.u32(this + 4 * i)).collect();
        assert_eq!(
            got,
            vec![
                0x5fc0_0000,
                0x203f_ffff,
                0x4000_0000,
                0x1234,
                0x6000_0001,
                0x2000_0000,
                0x6020_0001
            ]
        );
    }

    /// Writes a 0x20-byte box record: minimum x, y, z, tag, maximum x, y, z.
    fn record(e: &mut Engine, at: u32, min: [u32; 3], tag: u32, max: [u32; 3]) {
        e.mem.set_u32(at, min[0]);
        e.mem.set_u32(at + 4, min[1]);
        e.mem.set_u32(at + 8, min[2]);
        e.mem.set_u32(at + 0xc, tag);
        e.mem.set_u32(at + 0x10, max[0]);
        e.mem.set_u32(at + 0x14, max[1]);
        e.mem.set_u32(at + 0x18, max[2]);
    }

    /// Eight records: four real boxes, then a group whose first word stops the sweep.
    fn list(e: &mut Engine, boxes: [([u32; 3], u32, [u32; 3]); 4]) -> u32 {
        let at = e.mem.alloc(0x100);
        for (i, (min, tag, max)) in boxes.into_iter().enumerate() {
            record(e, at + 0x20 * i as u32, min, tag, max);
        }
        e.mem.set_u32(at + 0x80, 0xffff_ffff);
        at
    }

    fn sweep(
        e: &mut Engine,
        first: u32,
        second: u32,
        max_pairs: i32,
    ) -> (i32, Vec<(u32, u32)>, u32) {
        let out = e.mem.alloc(0x40);
        let overflow = e.mem.alloc(4);
        e.mem.set_u32(overflow, 99);
        let n = e
            .call(
                0x00db_00f0,
                &args![first, 1i32, second, 1i32, out, max_pairs, overflow],
            )
            .i32();
        let pairs = (0..n.max(0) as u32)
            .map(|i| (e.mem.u32(out + 8 * i), e.mem.u32(out + 8 * i + 4)))
            .collect();
        (n, pairs, e.mem.u32(overflow))
    }

    #[test]
    fn a_second_list_query_meets_the_first_lists_groups() {
        let mut e = Engine::new();
        // The first list starts after the second list's box (10 > 5): the second box is the
        // query. Record 1 misses in y, record 3 starts after the query's largest x (60).
        let first = list(
            &mut e,
            [
                ([10, 0, 0], 1, [100, 50, 50]),
                ([20, 100, 0], 2, [100, 150, 50]),
                ([30, 10, 10], 3, [100, 20, 20]),
                ([200, 0, 0], 4, [300, 50, 50]),
            ],
        );
        let second = list(
            &mut e,
            [
                ([5, 5, 5], 100, [60, 15, 15]),
                ([0, 0, 0], 0, [0, 0, 0]),
                ([0, 0, 0], 0, [0, 0, 0]),
                ([0, 0, 0], 0, [0, 0, 0]),
            ],
        );
        let (n, pairs, overflow) = sweep(&mut e, first, second, 8);
        assert_eq!(n, 2);
        assert_eq!(pairs, vec![(1, 100), (3, 100)]);
        assert_eq!(overflow, 0);
    }

    #[test]
    fn a_first_list_query_meets_the_second_lists_groups_and_pairs_that_do_not_fit_are_counted() {
        let mut e = Engine::new();
        // The first list's box starts first (5 <= 10): it is the query against the second list.
        let first = list(
            &mut e,
            [
                ([5, 0, 0], 7, [60, 50, 50]),
                ([0, 0, 0], 0, [0, 0, 0]),
                ([0, 0, 0], 0, [0, 0, 0]),
                ([0, 0, 0], 0, [0, 0, 0]),
            ],
        );
        let second = list(
            &mut e,
            [
                ([10, 0, 0], 21, [100, 10, 10]),
                ([20, 20, 20], 22, [100, 30, 30]),
                ([30, 100, 100], 23, [100, 200, 200]),
                ([40, 5, 5], 24, [100, 6, 6]),
            ],
        );
        // Records 0, 1 and 3 overlap in y and z; record 2 does not.
        let (n, pairs, overflow) = sweep(&mut e, first, second, 8);
        assert_eq!(n, 3);
        assert_eq!(pairs, vec![(7, 21), (7, 22), (7, 24)]);
        assert_eq!(overflow, 0);
        let (n, pairs, overflow) = sweep(&mut e, first, second, 1);
        assert_eq!(n, 1);
        assert_eq!(pairs, vec![(7, 21)]);
        assert_eq!(overflow, 2);
    }

    #[test]
    fn a_query_that_ends_before_the_other_list_begins_pairs_nothing() {
        let mut e = Engine::new();
        // First list starts at 50, after the second box's largest x (40): no overlap is looked
        // for, the second list advances and the loop ends.
        let first = list(
            &mut e,
            [
                ([50, 0, 0], 1, [90, 9, 9]),
                ([0, 0, 0], 0, [0, 0, 0]),
                ([0, 0, 0], 0, [0, 0, 0]),
                ([0, 0, 0], 0, [0, 0, 0]),
            ],
        );
        let second = list(
            &mut e,
            [
                ([5, 0, 0], 2, [40, 9, 9]),
                ([0, 0, 0], 0, [0, 0, 0]),
                ([0, 0, 0], 0, [0, 0, 0]),
                ([0, 0, 0], 0, [0, 0, 0]),
            ],
        );
        let (n, pairs, overflow) = sweep(&mut e, first, second, 4);
        assert_eq!((n, pairs.len(), overflow), (0, 0, 0));
    }

    type Calls = Rc<RefCell<Vec<(&'static str, Vec<u32>)>>>;

    /// A memory router whose allocator hands out heap blocks (the granted size is the request)
    /// and records its calls; the pair sort is a stable sort on the key.
    fn sorting_environment(e: &mut Engine) -> Calls {
        let calls: Calls = Rc::default();
        let allocator = e.mem.alloc(8);
        let alloc_double = 0x0071_0010;
        let free_double = 0x0071_0020;
        e.put_vtable(0x0070_0000, &[0, 0, 0, alloc_double, free_double]);
        e.mem.set_u32(allocator, 0x0070_0000);
        let router = e.mem.alloc(0x14);
        e.mem.set_u32(router + 0xc, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let log = Rc::clone(&calls);
        e.register_double(alloc_double, move |e, a| {
            let size = e.mem.u32(a[1]);
            log.borrow_mut().push(("allocate", vec![size]));
            Ret {
                eax: e.mem.alloc(size),
                ..Ret::default()
            }
        });
        let log = Rc::clone(&calls);
        e.register_double(free_double, move |_, a| {
            log.borrow_mut().push(("free", vec![a[2]]));
            Ret::default()
        });
        let log = Rc::clone(&calls);
        e.register_double(SORT_PAIRS, move |e, a| {
            log.borrow_mut().push(("sort", vec![a[1]]));
            let mut pairs: Vec<(u32, u32)> = (0..a[1])
                .map(|i| (e.mem.u32(a[0] + 8 * i), e.mem.u32(a[0] + 8 * i + 4)))
                .collect();
            pairs.sort_by_key(|p| p.0);
            for (i, p) in pairs.into_iter().enumerate() {
                e.mem.set_u32(a[0] + 8 * i as u32, p.0);
                e.mem.set_u32(a[0] + 8 * i as u32 + 4, p.1);
            }
            Ret::default()
        });
        calls
    }

    #[test]
    fn records_are_sorted_by_their_first_word_through_temporary_blocks() {
        let mut e = Engine::new();
        let calls = sorting_environment(&mut e);
        let blocks = e.mem.alloc(0x80);
        // Three records to sort and a fourth the sort pads the count to (it must sort last).
        for (i, key) in [30u32, 10, 20, 0xffff_ffff].into_iter().enumerate() {
            let at = blocks + 0x20 * i as u32;
            e.mem.set_u32(at, key);
            e.mem.set_u32(at + 0x1c, 0xa0 + i as u32);
        }
        e.call(0x00db_06c0, &args![blocks, 3u32]);
        let keys: Vec<u32> = (0..4).map(|i| e.mem.u32(blocks + 0x20 * i)).collect();
        let tails: Vec<u32> = (0..4)
            .map(|i| e.mem.u32(blocks + 0x20 * i + 0x1c))
            .collect();
        assert_eq!(keys, vec![10, 20, 30, 0xffff_ffff]);
        assert_eq!(tails, vec![0xa1, 0xa2, 0xa0, 0xa3]);
        assert_eq!(
            *calls.borrow(),
            vec![
                ("allocate", vec![32]),
                ("allocate", vec![32]),
                ("sort", vec![4]),
                ("free", vec![32]),
                ("allocate", vec![96]),
                ("free", vec![96]),
                ("free", vec![32]),
            ]
        );
    }

    #[test]
    fn an_empty_list_allocates_and_frees_nothing() {
        let mut e = Engine::new();
        let calls = sorting_environment(&mut e);
        e.call(0x00db_06c0, &args![0x1000u32, 0u32]);
        assert_eq!(*calls.borrow(), vec![("sort", vec![0])]);
    }
}

//! `Havok SDK/run_00d1b1d0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkadeltacompressedanimation::profile_record;

/// The first word (the sort key) of element `index` (8 bytes each).
fn key_00d1b1d0(e: &Engine, array: u32, index: i32) -> u32 {
    e.mem
        .u32(array.wrapping_add((index as u32).wrapping_mul(8)))
}

// Translated from 00d1b1d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. cdecl, four words: quicksort of the elements `low..=high`
/// of the array of 8-byte pairs at `array`, by their first word as an
/// unsigned number. The fourth word is only passed on to the recursion. The
/// pivot is the key of the middle element; the partition scans, swaps and
/// recursion (left part recursed, right part looped) are those of the code.
pub fn fn_00d1b1d0(e: &mut Engine, array: Ptr, mut low: i32, high: i32, unused_4: u32) {
    let array = array.addr();
    loop {
        let pivot = key_00d1b1d0(e, array, low.wrapping_add(high) >> 1);
        let (mut i, mut j) = (low, high);
        loop {
            while key_00d1b1d0(e, array, i) < pivot {
                i = i.wrapping_add(1);
            }
            while pivot < key_00d1b1d0(e, array, j) {
                j = j.wrapping_sub(1);
            }
            if j < i {
                break;
            }
            if j != i {
                let at = |index: i32| array.wrapping_add((index as u32).wrapping_mul(8));
                let (from, to) = (at(i), at(j));
                let (a0, a1) = (e.mem.u32(to), e.mem.u32(to + 4));
                let (b0, b1) = (e.mem.u32(from), e.mem.u32(from + 4));
                e.mem.set_u32(to, b0);
                e.mem.set_u32(to + 4, b1);
                e.mem.set_u32(from, a0);
                e.mem.set_u32(from + 4, a1);
            }
            j = j.wrapping_sub(1);
            i = i.wrapping_add(1);
            if i > j {
                break;
            }
        }
        if low < j {
            fn_00d1b1d0(e, Ptr::new(array), low, j, unused_4);
        }
        if i < high {
            low = i;
        } else {
            return;
        }
    }
}

/// The memory router (`00c85750`): word `+0` is the cursor of its stack arena and word `+0xc` the
/// allocator word `00c90940` grows an array through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `00c90940` (cdecl `(allocator word, array, element size)`): grows an `hkArray`.
const ARRAY_GROW: u32 = 0x00c9_0940;
/// The double `65535.0` the 16-bit split bounds are divided by (`010217d0`).
const BOUND_DIVISOR: u32 = 0x0102_17d0;
/// The float (`010cd644`) multiplied with the zero lanes of a decoded 16-bit box.
const FILL_FLOAT: u32 = 0x010c_d644;
/// The four lanes `1 / 65535` (`011b3810`) the scale is multiplied with.
const LANE_FACTOR: u32 = 0x011b_3810;

/// The compare-and-push sequence of both tree queries: after `00c85750` has been called, the
/// array's size (`+4`) is compared with the capacity bits of `+8` and `00c90940` grows it with
/// `(router word +0xc, array, 4)` when they are equal; then `key` is stored at element `size` and
/// the size incremented.
fn push_key_00d1b290(e: &mut Engine, array: u32, key: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
    if e.mem.u32(array + 4) == capacity {
        let allocator = e.mem.u32(router + 0xc);
        e.call(ARRAY_GROW, &args![allocator, array, 4u32]);
    }
    let size = e.mem.u32(array + 4);
    let data = e.mem.u32(array);
    e.mem.set_u32(data.wrapping_add(size.wrapping_mul(4)), key);
    e.mem.set_u32(array + 4, size.wrapping_add(1));
}

/// A split bound of an inner node as the x87 code computes it: `value / 65535 * scale[axis] +
/// offset[axis]`, stored as a float and loaded back.
fn split_bound_00d1b290(e: &Engine, value: u32, scale: u32, offset: u32, axis: u32) -> f64 {
    let divisor = e.mem.f64(BOUND_DIVISOR);
    let scaled = x87_mul(
        x87_div(f64::from(value), divisor),
        x87_m32(e.mem.u32(scale.wrapping_add(axis * 4))),
    );
    let shifted = x87_add(scaled, x87_m32(e.mem.u32(offset.wrapping_add(axis * 4))));
    x87_load32(x87_store32(shifted))
}

/// Whether the 16-byte leaf record that ends at `end` (`end - 0xe` is its first word) overlaps
/// the query box (`query`: minimum lanes, maximum lanes at `+0x10`) on lanes 0 to 2: the eight
/// words are decoded as `0 * fill + word` lanes (the zero lanes of the unpacking multiplied by
/// `fill`), the first four as the box minimum and the last four as its maximum, each scaled by
/// `scale * 1/65535` and moved by `offset`, then compared with `CMPLEPS`.
fn leaf_overlaps_00d1b290(
    e: &Engine,
    query: u32,
    scale: u32,
    offset: u32,
    end: u32,
    fill: f32,
) -> bool {
    let word = |index: u32| e.mem.u16(end.wrapping_sub(0xe).wrapping_add(2 * index));
    let decoded = |index: u32| (0.0f32 * fill) + f32::from(word(index));
    let lanes = |at: u32| [0u32, 1, 2, 3].map(|lane| e.mem.f32(at.wrapping_add(4 * lane)));
    let factor = lanes(LANE_FACTOR);
    let scale = lanes(scale);
    let offset = lanes(offset);
    let query_minimum = lanes(query);
    let query_maximum = lanes(query.wrapping_add(0x10));
    (0..3).all(|lane| {
        let s = scale[lane] * factor[lane];
        let minimum = decoded(lane as u32) * s + offset[lane];
        let maximum = decoded(4 + lane as u32) * s + offset[lane];
        minimum <= query_maximum[lane] && query_minimum[lane] <= maximum
    })
}

/// The 32-bit key of the leaf record ending at `end`: the words at `end - 8` and `end`.
fn leaf_key_00d1b290(e: &Engine, end: u32) -> u32 {
    u32::from(e.mem.u16(end.wrapping_sub(8))) | (u32::from(e.mem.u16(end)) << 16)
}

// Translated from 00d1b290 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (cdecl, seven words `context`, `node`, `leaves`, `scale`, `offset`, `query`, `out`;
/// the names are descriptions, not confirmed): a query of a tree of 16-bit boxes. An inner node
/// (sign bit of its first word set) has its first child at `node + (word & 0x7ffffffc)`, the
/// second 8 bytes after it, an axis in the low two bits, and two 16-bit split bounds in its
/// second word (`value / 65535 * scale[axis] + offset[axis]`, as floats). With the low bound
/// above the query maximum on that axis only the first child is walked; with the high bound
/// below the query minimum only the second; otherwise the first is walked recursively and the
/// second by the loop. A leaf (first word not `0x7fffffff`, count in the second word above 0)
/// holds that many 16-byte records at `leaves + 16 * index`; each record whose box overlaps the
/// query box on the first three lanes pushes its 32-bit key (words 3 and 7) on the `hkArray`
/// at `out`. `context` is only passed on.
pub fn fn_00d1b290(
    e: &mut Engine,
    context: u32,
    node: u32,
    leaves: u32,
    scale: u32,
    offset: u32,
    query: u32,
    out: u32,
) {
    let mut node = node;
    let mut word = e.mem.u32(node);
    let mut second = e.mem.u32(node.wrapping_add(4));
    while (word as i32) < 0 {
        let first_child = node.wrapping_add(word & 0x7fff_fffc);
        let second_child = first_child.wrapping_add(8);
        let axis = word & 3;
        let maximum = x87_load32(e.mem.u32(query.wrapping_add(axis * 4).wrapping_add(0x10)));
        let low_bound = split_bound_00d1b290(e, second & 0xffff, scale, offset, axis);
        if low_bound > maximum {
            node = first_child;
        } else {
            let minimum = x87_load32(e.mem.u32(query.wrapping_add(axis * 4)));
            let high_bound = split_bound_00d1b290(e, second >> 16, scale, offset, axis);
            if high_bound < minimum {
                node = second_child;
            } else {
                fn_00d1b290(e, context, first_child, leaves, scale, offset, query, out);
                node = second_child;
            }
        }
        word = e.mem.u32(node);
        second = e.mem.u32(node.wrapping_add(4));
    }
    if word == 0x7fff_ffff || (second as i32) <= 0 {
        return;
    }
    let fill = f32::from_bits(x87_quiet_bits(e.mem.u32(FILL_FLOAT)));
    let mut end = leaves
        .wrapping_add((word & 0x7fff_ffff) << 4)
        .wrapping_add(0xe);
    for _ in 0..second {
        let key = leaf_key_00d1b290(e, end);
        if leaf_overlaps_00d1b290(e, query, scale, offset, end, fill) {
            push_key_00d1b290(e, out, key);
        }
        end = end.wrapping_add(0x10);
    }
}

// Translated from 00d1b4d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (cdecl, ten words `node`, `leaves`, `scale`, `offset`, `query`, `stack_words`, `out`,
/// `callback`, `cursor`, `limit`; the names are descriptions, not confirmed): the same query as
/// `00d1b290` with an explicit stack in the memory router's arena (`stack_words * 4` bytes rounded
/// up to 0x80, taken from the router's cursor at `+0` and given back at the end) instead of
/// recursion: for both children the second is pushed and the first walked; when a leaf is done
/// the last pushed node is popped. The inner node bounds use a private copy of the 32-byte query
/// box taken at the start; the leaf records are tested against `query` itself. Each hit key is
/// given, in this order of preference, to `callback` (virtual slot 4 with the address of a byte
/// and the key; the walk ends when the byte it answers is not zero), pushed on the `hkArray` at
/// `out`, or stored at the pointer cursor `*cursor` (advanced by 4; the walk ends when the number
/// of stored keys reaches `limit`, signed). Answers the number of keys stored at the cursor (the
/// counter at `[ESP+0x28]`, zeroed at `00d1b52f` and counted at `00d1b78a`..`00d1b792`, which
/// `00d1b7e4` reads into `EAX` after `POP EDI; POP ESI`): 0 when the keys go to the callback or
/// the array, or nowhere.
pub fn fn_00d1b4d0(
    e: &mut Engine,
    node: u32,
    leaves: u32,
    scale: u32,
    offset: u32,
    query: u32,
    stack_words: u32,
    out: u32,
    callback: u32,
    cursor: u32,
    limit: u32,
) -> u32 {
    let mut current = node;
    let mut word = e.mem.u32(node);
    let mut second = e.mem.u32(node.wrapping_add(4));
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let arena_start = e.mem.u32(router);
    let fill = f32::from_bits(x87_quiet_bits(e.mem.u32(FILL_FLOAT)));
    let bump = (stack_words.wrapping_mul(4).wrapping_add(0x7f)) & 0xffff_ff80;
    let mut stack_top = arena_start;
    e.mem.set_u32(router, bump.wrapping_add(arena_start));
    let local_query: Vec<u32> = (0..8)
        .map(|word_index| e.mem.u32(query.wrapping_add(word_index * 4)))
        .collect();
    let mut stored = 0u32;
    'walk: loop {
        while (word as i32) < 0 {
            let first_child = current.wrapping_add(word & 0x7fff_fffc);
            let axis = (word & 3) as usize;
            let second_child = first_child.wrapping_add(8);
            let maximum = x87_load32(local_query[4 + axis]);
            let low_bound = split_bound_00d1b290(e, second & 0xffff, scale, offset, axis as u32);
            if low_bound > maximum {
                current = first_child;
            } else {
                let minimum = x87_load32(local_query[axis]);
                let high_bound = split_bound_00d1b290(e, second >> 16, scale, offset, axis as u32);
                if high_bound < minimum {
                    current = second_child;
                } else {
                    e.mem.set_u32(stack_top, second_child);
                    stack_top = stack_top.wrapping_add(4);
                    current = first_child;
                }
            }
            word = e.mem.u32(current);
            second = e.mem.u32(current.wrapping_add(4));
        }
        if word != 0x7fff_ffff && (second as i32) > 0 {
            let mut end = leaves
                .wrapping_add((word & 0x7fff_ffff) << 4)
                .wrapping_add(0xe);
            let mut index = 0u32;
            while (index as i32) < (second as i32) {
                let last_key = leaf_key_00d1b290(e, end);
                if leaf_overlaps_00d1b290(e, query, scale, offset, end, fill) {
                    if callback != 0 {
                        let stop = e.with_stack(4, |e, flag| {
                            let answer = e.vcall(callback, 4, &args![flag, last_key]).u32();
                            e.mem.u8(answer) != 0
                        });
                        if stop {
                            break 'walk;
                        }
                    } else if out != 0 {
                        push_key_00d1b290(e, out, last_key);
                    } else if cursor != 0 {
                        let slot = e.mem.u32(cursor);
                        e.mem.set_u32(slot, last_key);
                        e.mem.set_u32(cursor, slot.wrapping_add(4));
                        stored = stored.wrapping_add(1);
                        if (stored as i32) >= (limit as i32) {
                            break 'walk;
                        }
                    }
                }
                index = index.wrapping_add(1);
                end = end.wrapping_add(0x10);
            }
        }
        if stack_top == arena_start {
            break;
        }
        stack_top = stack_top.wrapping_sub(4);
        current = e.mem.u32(stack_top);
        word = e.mem.u32(current);
        second = e.mem.u32(current.wrapping_add(4));
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.set_u32(router, arena_start);
    // 00d1b7e0..00d1b7e4: `POP EDI; POP ESI; MOV EAX,[ESP+0x20]` reads the body's [ESP+0x28],
    // the stored-key counter (00d1b52f, 00d1b78a..00d1b792), not the key at [ESP+0x20].
    stored
}

/// Copies the words at `tree + 0x30` to `frame` and stores `(tree + 0x40) - (tree + 0x30)` (per
/// lane, `SUBPS`) at `frame + 0x10`: the offset and scale vectors of the queries.
fn tree_vectors_00d1b7f0(e: &mut Engine, tree: u32, frame: u32) {
    for index in 0..4 {
        let bits = e.mem.u32(tree + 0x30 + index * 4);
        e.mem.set_u32(frame + index * 4, bits);
    }
    for index in 0..4 {
        let difference = e.mem.f32(tree + 0x40 + index * 4) - e.mem.f32(tree + 0x30 + index * 4);
        e.mem.set_f32(frame + 0x10 + index * 4, difference);
    }
}

// Translated from 00d1b7f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (cdecl, three words `tree`, `query`, `out`; descriptions, not confirmed): runs
/// `00d1b290` with the tree's root (the word at `tree + 0`, passed as `context` and `node`), its
/// leaves (`tree + 0x1c`), a scale vector `(tree + 0x40) - (tree + 0x30)` and an offset vector (the
/// words at `tree + 0x30`), both on the stack, then `query` and `out`.
pub fn fn_00d1b7f0(e: &mut Engine, tree: u32, query: u32, out: u32) {
    e.with_stack(0x20, |e, frame| {
        let f = frame.addr();
        tree_vectors_00d1b7f0(e, tree, f);
        let root = e.mem.u32(tree);
        let leaves = e.mem.u32(tree + 0x1c);
        fn_00d1b290(e, root, root, leaves, f + 0x10, f, query, out);
    });
}

/// The number of words of the traversal stack the wrappers ask `00d1b4d0` for: the word at
/// `tree + 0xc` when above 0, else 0x80.
fn stack_words_00d1b850(e: &Engine, tree: u32) -> u32 {
    let words = e.mem.i32(tree + 0xc);
    if words > 0 {
        words as u32
    } else {
        0x80
    }
}

// Translated from 00d1b850 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (cdecl, three words `tree`, `query`, `out`; descriptions, not confirmed): the
/// stacked query: a profiling record (`010cd648`), `00d1b4d0` with the tree's root, leaves
/// (`tree + 0x1c`), the scale and offset vectors as in `00d1b7f0`, `query`, the stack size
/// (word at `tree + 0xc` when above 0, else 0x80), `out` and no callback or cursor, then the
/// closing record (`010c41f8`). The time stamps are the host clock (the engine has no `RDTSC`).
pub fn fn_00d1b850(e: &mut Engine, tree: u32, query: u32, out: u32) {
    profile_record(e, 0x010c_d648);
    e.with_stack(0x20, |e, frame| {
        let f = frame.addr();
        tree_vectors_00d1b7f0(e, tree, f);
        let root = e.mem.u32(tree);
        let leaves = e.mem.u32(tree + 0x1c);
        let words = stack_words_00d1b850(e, tree);
        fn_00d1b4d0(e, root, leaves, f + 0x10, f, query, words, out, 0, 0, 0);
    });
    profile_record(e, 0x010c_41f8);
}

// Translated from 00d1b930 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (cdecl, four words `tree`, `query`, `cursor`, `limit`; descriptions, not
/// confirmed): `00d1b4d0` as in `00d1b850` but with no array and no callback and the pointer
/// cursor and limit given. No profiling records. Answers what `00d1b4d0` answers (the number of
/// keys stored at the cursor): nothing after the call writes `EAX`; `00df6850`/`00df6950` add it
/// to the entry's count and take it off the budget.
pub fn fn_00d1b930(e: &mut Engine, tree: u32, query: u32, cursor: u32, limit: u32) -> u32 {
    // 00d1b997..00d1b9a3: `CALL 00d1b4d0; ADD ESP,0x28; POP ESI; MOV ESP,EBP; POP EBP; RET`.
    e.with_stack(0x20, |e, frame| {
        let f = frame.addr();
        tree_vectors_00d1b7f0(e, tree, f);
        let root = e.mem.u32(tree);
        let leaves = e.mem.u32(tree + 0x1c);
        let words = stack_words_00d1b850(e, tree);
        fn_00d1b4d0(
            e,
            root,
            leaves,
            f + 0x10,
            f,
            query,
            words,
            0,
            0,
            cursor,
            limit,
        )
    })
}

// Translated from 00d1b9b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (cdecl, three words `tree`, `query`, `callback`; descriptions, not confirmed):
/// `00d1b4d0` as in `00d1b850` but with the callback given and no array or cursor, inside the
/// same two profiling records.
pub fn fn_00d1b9b0(e: &mut Engine, tree: u32, query: u32, callback: u32) {
    profile_record(e, 0x010c_d648);
    e.with_stack(0x20, |e, frame| {
        let f = frame.addr();
        tree_vectors_00d1b7f0(e, tree, f);
        let root = e.mem.u32(tree);
        let leaves = e.mem.u32(tree + 0x1c);
        let words = stack_words_00d1b850(e, tree);
        fn_00d1b4d0(
            e,
            root,
            leaves,
            f + 0x10,
            f,
            query,
            words,
            0,
            callback,
            0,
            0,
        );
    });
    profile_record(e, 0x010c_41f8);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d1b1d0, fn_00d1b1d0(Ptr, i32, i32, u32)),
        entry!(0x00d1_b290, fn_00d1b290(u32, u32, u32, u32, u32, u32, u32)),
        entry!(0x00d1_b4d0, fn_00d1b4d0(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00d1_b7f0, fn_00d1b7f0(u32, u32, u32)),
        entry!(0x00d1_b850, fn_00d1b850(u32, u32, u32)),
        entry!(0x00d1_b930, fn_00d1b930(u32, u32, u32, u32) -> u32),
        entry!(0x00d1_b9b0, fn_00d1b9b0(u32, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00d1b1d0_sorts_the_pairs_by_unsigned_first_word() {
        let mut e = Engine::new();
        let keys = [5u32, 0xffff_fff0, 9, 1, 7, 3, 8, 2, 3];
        let array = e.mem.alloc(8 * keys.len() as u32);
        for (i, key) in keys.iter().enumerate() {
            e.mem.set_u32(array + 8 * i as u32, *key);
            e.mem.set_u32(array + 8 * i as u32 + 4, 500 + i as u32);
        }
        e.call(
            0x00d1_b1d0,
            &args![Ptr::<()>::new(array), 0i32, keys.len() as i32 - 1, 0u32],
        );
        let sorted: Vec<u32> = (0..keys.len() as u32)
            .map(|i| e.mem.u32(array + 8 * i))
            .collect();
        assert_eq!(sorted, vec![1, 2, 3, 3, 5, 7, 8, 9, 0xffff_fff0]);
        // The second word travelled with its key (key 0xfffffff0 was index 1).
        assert_eq!(e.mem.u32(array + 8 * 8 + 4), 501);
    }
}

#[cfg(test)]
mod havok_a04_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// Keys of the three leaf records: record 0 is child A (key 0x10007), records 1 and 2 are
    /// child B (keys 0x20008 and 0x30009).
    const KEYS: [u32; 3] = [0x0001_0007, 0x0002_0008, 0x0003_0009];

    struct Fixture {
        root: u32,
        leaves: u32,
        scale: u32,
        offset: u32,
        query: u32,
        router: u32,
        arena: u32,
        /// The router cursor seen by each call of `00c85750`.
        cursors: Rc<RefCell<Vec<u32>>>,
    }

    fn write_record(
        e: &mut Engine,
        at: u32,
        minimum: u16,
        key_low: u16,
        maximum: u16,
        key_high: u16,
    ) {
        for lane in 0..3 {
            e.mem.set_u16(at + lane * 2, minimum);
            e.mem.set_u16(at + 8 + lane * 2, maximum);
        }
        e.mem.set_u16(at + 6, key_low);
        e.mem.set_u16(at + 14, key_high);
    }

    /// A root node splitting on axis 0 with bounds `low` and `high` over child A (leaf 0, one
    /// record) and child B (leaf 1, two records); the query box is `(3, 0, 0)..(10, 100, 100)`,
    /// scale 65535 and offset 0.
    fn fixture(e: &mut Engine, low: u32, high: u32) -> Fixture {
        e.map(0x0100_0000, 0x0030_0000);
        e.mem.set_f64(BOUND_DIVISOR, 65535.0);
        e.mem.set_u32(FILL_FLOAT, 0x5a0e_1bca);
        for lane in 0..4 {
            e.mem.set_u32(LANE_FACTOR + lane * 4, 0x3780_0080);
        }
        let nodes = e.mem.alloc(0x40);
        e.mem.set_u32(nodes, 0x8000_0010);
        e.mem.set_u32(nodes + 4, low | (high << 16));
        e.mem.set_u32(nodes + 0x10, 0);
        e.mem.set_u32(nodes + 0x14, 1);
        e.mem.set_u32(nodes + 0x18, 1);
        e.mem.set_u32(nodes + 0x1c, 2);
        let leaves = e.mem.alloc(0x30);
        write_record(e, leaves, 1, 7, 4, 1);
        write_record(e, leaves + 0x10, 50, 8, 60, 2);
        write_record(e, leaves + 0x20, 5, 9, 20, 3);
        let scale = e.mem.alloc(0x10);
        let offset = e.mem.alloc(0x10);
        for lane in 0..4 {
            e.mem.set_f32(scale + lane * 4, 65535.0);
            e.mem.set_f32(offset + lane * 4, 0.0);
        }
        let query = e.mem.alloc(0x20);
        for (lane, (minimum, maximum)) in
            [(3.0f32, 10.0f32), (0.0, 100.0), (0.0, 100.0), (0.0, 0.0)]
                .into_iter()
                .enumerate()
        {
            e.mem.set_f32(query + lane as u32 * 4, minimum);
            e.mem.set_f32(query + 0x10 + lane as u32 * 4, maximum);
        }
        let arena = e.mem.alloc(0x400);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router, arena);
        e.mem.set_u32(router + 0xc, 0x7777);
        let cursors = Rc::new(RefCell::new(Vec::new()));
        let seen = Rc::clone(&cursors);
        e.register_double(MEMORY_ROUTER, move |e, _| {
            seen.borrow_mut().push(e.mem.u32(router));
            Ret {
                eax: router,
                ..Ret::default()
            }
        });
        Fixture {
            root: nodes,
            leaves,
            scale,
            offset,
            query,
            router,
            arena,
            cursors,
        }
    }

    /// An `hkArray` of words with room for `capacity`.
    fn array(e: &mut Engine, capacity: u32) -> u32 {
        let array = e.mem.alloc(0xc);
        let data = e.mem.alloc(capacity * 4 + 4);
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 8, capacity | 0x8000_0000);
        array
    }

    fn keys(e: &Engine, array: u32) -> Vec<u32> {
        let data = e.mem.u32(array);
        (0..e.mem.u32(array + 4))
            .map(|index| e.mem.u32(data + index * 4))
            .collect()
    }

    /// (low bound, high bound, expected keys): both children, only the first, only the second.
    const CASES: [(u32, u32, [usize; 2]); 3] = [(5, 6, [0, 2]), (20, 6, [0, 9]), (5, 2, [2, 9])];

    fn expected(case: [usize; 2]) -> Vec<u32> {
        case.into_iter()
            .filter(|index| *index < 3)
            .map(|index| KEYS[index])
            .collect()
    }

    #[test]
    fn fn_00d1b290_walks_the_children_the_bounds_allow() {
        for (low, high, case) in CASES {
            let mut e = Engine::new();
            let f = fixture(&mut e, low, high);
            let out = array(&mut e, 8);
            e.call(
                0x00d1_b290,
                &args![f.root, f.root, f.leaves, f.scale, f.offset, f.query, out],
            );
            assert_eq!(keys(&e, out), expected(case), "bounds {low}, {high}");
        }
    }

    #[test]
    fn fn_00d1b290_grows_a_full_array() {
        let mut e = Engine::new();
        let f = fixture(&mut e, 5, 6);
        let out = array(&mut e, 1);
        let bigger = e.mem.alloc(0x40);
        e.register_double(ARRAY_GROW, move |e, a| {
            let (array, old) = (a[1], e.mem.u32(a[1]));
            let size = e.mem.u32(array + 4);
            for index in 0..size {
                let word = e.mem.u32(old + index * 4);
                e.mem.set_u32(bigger + index * 4, word);
            }
            e.mem.set_u32(array, bigger);
            e.mem.set_u32(array + 8, 8);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(
            0x00d1_b290,
            &args![f.root, f.root, f.leaves, f.scale, f.offset, f.query, out],
        );
        let log = e.call_log.take().unwrap();
        let grows: Vec<_> = log.iter().filter(|(a, _)| *a == ARRAY_GROW).collect();
        assert_eq!(grows.len(), 1);
        assert_eq!(grows[0].1, vec![0x7777, out, 4]);
        assert_eq!(keys(&e, out), vec![KEYS[0], KEYS[2]]);
    }

    #[test]
    fn fn_00d1b290_stops_at_empty_and_zero_count_leaves() {
        let mut e = Engine::new();
        let f = fixture(&mut e, 5, 6);
        let out = array(&mut e, 8);
        // A leaf marker word of 0x7fffffff, then a count of 0.
        let node = e.mem.alloc(0x10);
        e.mem.set_u32(node, 0x7fff_ffff);
        e.mem.set_u32(node + 4, 3);
        e.call(
            0x00d1_b290,
            &args![node, node, f.leaves, f.scale, f.offset, f.query, out],
        );
        assert!(keys(&e, out).is_empty());
        e.mem.set_u32(node, 0);
        e.mem.set_u32(node + 4, 0);
        e.call(
            0x00d1_b290,
            &args![node, node, f.leaves, f.scale, f.offset, f.query, out],
        );
        assert!(keys(&e, out).is_empty());
        assert!(f.cursors.borrow().is_empty());
    }

    #[test]
    fn fn_00d1b4d0_walks_with_its_own_stack_and_restores_the_arena() {
        for (low, high, case) in CASES {
            let mut e = Engine::new();
            let f = fixture(&mut e, low, high);
            let out = array(&mut e, 8);
            let last = e
                .call(
                    0x00d1_b4d0,
                    &args![
                        f.root, f.leaves, f.scale, f.offset, f.query, 8u32, out, 0u32, 0u32, 0u32
                    ],
                )
                .u32();
            assert_eq!(keys(&e, out), expected(case), "bounds {low}, {high}");
            // Keys pushed on the array are not counted: the answer (00d1b7e4) is 0.
            assert_eq!(last, 0);
            assert_eq!(e.mem.u32(f.router), f.arena);
            // 8 words of stack are 0x80 bytes: every later call of the router sees the bump.
            let cursors = f.cursors.borrow();
            assert_eq!(cursors[0], f.arena);
            assert!(cursors[1..].iter().all(|cursor| *cursor == f.arena + 0x80));
        }
    }

    #[test]
    fn fn_00d1b4d0_gives_hits_to_the_callback_until_it_answers_nonzero() {
        let mut e = Engine::new();
        let f = fixture(&mut e, 5, 6);
        let answers = e.mem.alloc(8);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(0x0900_0021, move |e, a| {
            log.borrow_mut().push((a[0], a[2]));
            // The first key is answered with a zero byte, the second with a non-zero one.
            let stop = u8::from(log.borrow().len() > 1);
            e.mem.set_u8(answers, stop);
            Ret {
                eax: answers,
                ..Ret::default()
            }
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 4, 0x0900_0021);
        let callback = e.mem.alloc(0x10);
        e.mem.set_u32(callback, vtable);
        let last = e
            .call(
                0x00d1_b4d0,
                &args![
                    f.root, f.leaves, f.scale, f.offset, f.query, 8u32, 0u32, callback, 0u32, 0u32
                ],
            )
            .u32();
        let seen = seen.borrow();
        assert_eq!(
            seen.iter().map(|(_, key)| *key).collect::<Vec<_>>(),
            vec![KEYS[0], KEYS[2]]
        );
        assert_ne!(seen[0].0, 0);
        // Keys given to the callback are not counted: the answer (00d1b7e4) is 0.
        assert_eq!(last, 0);
        assert_eq!(e.mem.u32(f.router), f.arena);
    }

    #[test]
    fn fn_00d1b4d0_stores_hits_at_the_cursor_up_to_the_limit() {
        let mut e = Engine::new();
        let f = fixture(&mut e, 5, 6);
        let buffer = e.mem.alloc(0x20);
        let cursor = e.mem.alloc(4);
        e.mem.set_u32(cursor, buffer);
        let stored = e
            .call(
                0x00d1_b4d0,
                &args![
                    f.root, f.leaves, f.scale, f.offset, f.query, 8u32, 0u32, 0u32, cursor, 5u32
                ],
            )
            .u32();
        assert_eq!(e.mem.u32(cursor), buffer + 8);
        assert_eq!(e.mem.u32(buffer), KEYS[0]);
        assert_eq!(e.mem.u32(buffer + 4), KEYS[2]);
        // The answer is the counter of stored keys (00d1b78a..00d1b792, read at 00d1b7e4).
        assert_eq!(stored, 2);
        // With a limit of one the walk ends after the first key.
        e.mem.set_u32(cursor, buffer);
        let stored = e
            .call(
                0x00d1_b4d0,
                &args![
                    f.root, f.leaves, f.scale, f.offset, f.query, 8u32, 0u32, 0u32, cursor, 1u32
                ],
            )
            .u32();
        assert_eq!(e.mem.u32(cursor), buffer + 4);
        assert_eq!(stored, 1);
        assert_eq!(e.mem.u32(f.router), f.arena);
    }

    #[test]
    fn fn_00d1b4d0_with_no_receiver_only_walks() {
        let mut e = Engine::new();
        let f = fixture(&mut e, 5, 6);
        let last = e
            .call(
                0x00d1_b4d0,
                &args![f.root, f.leaves, f.scale, f.offset, f.query, 8u32, 0u32, 0u32, 0u32, 0u32],
            )
            .u32();
        // Nothing stored at a cursor: the answer (00d1b7e4) is 0.
        assert_eq!(last, 0);
        assert_eq!(e.mem.u32(f.router), f.arena);
    }

    /// A tree record: root at `+0`, leaves at `+0x1c`, stack words at `+0xc`, the offset vector
    /// (zero) at `+0x30` and the vector at `+0x40` that makes the scale 65535.
    fn tree(e: &mut Engine, f: &Fixture, stack_words: i32) -> u32 {
        let tree = e.mem.alloc(0x50);
        e.mem.set_u32(tree, f.root);
        e.mem.set_i32(tree + 0xc, stack_words);
        e.mem.set_u32(tree + 0x1c, f.leaves);
        for lane in 0..4 {
            e.mem.set_f32(tree + 0x40 + lane * 4, 65535.0);
        }
        tree
    }

    #[test]
    fn fn_00d1b7f0_queries_with_the_trees_vectors() {
        let mut e = Engine::new();
        let f = fixture(&mut e, 5, 6);
        let tree = tree(&mut e, &f, 0);
        let out = array(&mut e, 8);
        e.call(0x00d1_b7f0, &args![tree, f.query, out]);
        assert_eq!(keys(&e, out), vec![KEYS[0], KEYS[2]]);
    }

    /// Profiling records: the stream the double of `TlsGetValue` answers.
    fn profile_stream(e: &mut Engine) -> u32 {
        e.set_global(0x0126_835c, 3u32);
        let stream = e.mem.alloc(0x40);
        let records = e.mem.alloc(0x40);
        e.mem.set_u32(stream + 4, records);
        e.mem.set_u32(stream + 0xc, records + 0x40);
        e.register(0x00fd_f044, |e, a| {
            assert_eq!(a[0], 3);
            Ret {
                eax: e.mem.u32(0x0126_8400),
                ..Ret::default()
            }
        });
        e.mem.set_u32(0x0126_8400, stream);
        records
    }

    #[test]
    fn fn_00d1b850_queries_between_two_profiling_records() {
        let mut e = Engine::new();
        let f = fixture(&mut e, 5, 6);
        let records = profile_stream(&mut e);
        // 0 stack words ask for 0x80 words: 0x200 bytes of arena.
        let tree = tree(&mut e, &f, 0);
        let out = array(&mut e, 8);
        e.call(0x00d1_b850, &args![tree, f.query, out]);
        assert_eq!(keys(&e, out), vec![KEYS[0], KEYS[2]]);
        assert_eq!(e.mem.u32(records), 0x010c_d648);
        assert_eq!(e.mem.u32(records + 0xc), 0x010c_41f8);
        let cursors = f.cursors.borrow();
        assert_eq!(cursors[0], f.arena);
        assert!(cursors[1..].iter().all(|cursor| *cursor == f.arena + 0x200));
    }

    #[test]
    fn fn_00d1b930_queries_into_the_cursor() {
        let mut e = Engine::new();
        let f = fixture(&mut e, 5, 6);
        let tree = tree(&mut e, &f, 8);
        let buffer = e.mem.alloc(0x20);
        let cursor = e.mem.alloc(4);
        e.mem.set_u32(cursor, buffer);
        e.call(0x00d1_b930, &args![tree, f.query, cursor, 5u32]);
        assert_eq!(e.mem.u32(cursor), buffer + 8);
        assert_eq!(e.mem.u32(buffer + 4), KEYS[2]);
        assert_eq!(*f.cursors.borrow(), vec![f.arena, f.arena + 0x80]);
    }

    /// 00d1b997..00d1b9a3: nothing after `CALL 00d1b4d0` writes `EAX`, so the answer is the
    /// number of keys `00d1b4d0` stored at the cursor (what `00df6850` and `00df6950` add to the
    /// entry and take off the budget).
    #[test]
    fn fn_00d1b930_answers_the_number_of_keys_stored() {
        let mut e = Engine::new();
        let f = fixture(&mut e, 5, 6);
        let tree = tree(&mut e, &f, 8);
        let buffer = e.mem.alloc(0x20);
        let cursor = e.mem.alloc(4);
        e.mem.set_u32(cursor, buffer);
        let stored = e
            .call(0x00d1_b930, &args![tree, f.query, cursor, 5u32])
            .u32();
        assert_eq!(stored, 2);
        e.mem.set_u32(cursor, buffer);
        let stored = e
            .call(0x00d1_b930, &args![tree, f.query, cursor, 1u32])
            .u32();
        assert_eq!(stored, 1);
        assert_eq!(e.mem.u32(cursor), buffer + 4);
    }

    #[test]
    fn fn_00d1b9b0_queries_through_the_callback_between_two_records() {
        let mut e = Engine::new();
        let f = fixture(&mut e, 5, 6);
        let records = profile_stream(&mut e);
        let tree = tree(&mut e, &f, 8);
        let flag = e.mem.alloc(4);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(0x0900_0021, move |_, a| {
            log.borrow_mut().push(a[2]);
            Ret {
                eax: flag,
                ..Ret::default()
            }
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 4, 0x0900_0021);
        let callback = e.mem.alloc(0x10);
        e.mem.set_u32(callback, vtable);
        e.call(0x00d1_b9b0, &args![tree, f.query, callback]);
        assert_eq!(*seen.borrow(), vec![KEYS[0], KEYS[2]]);
        assert_eq!(e.mem.u32(records), 0x010c_d648);
        assert_eq!(e.mem.u32(records + 0xc), 0x010c_41f8);
    }
}

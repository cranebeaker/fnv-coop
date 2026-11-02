//! `Havok SDK/hkkdtreebuilder.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkgeometryprocessing::{
    add4, float_to_x87, load_bits, load_vec4, mul4, store_vec4, x87_to_float, X87,
};

/// The `float` `00df3170` splats (`010cd644`).
const SCALE_FLOAT: u32 = 0x010c_d644;

/// Copies the 16-byte vector at `from` to `to` (four `MOV` dwords, or `MOVAPS`).
fn copy_vector(e: &mut Engine, from: u32, to: u32) {
    for lane in 0..4 {
        let word = e.mem.u32(from + 4 * lane);
        e.mem.set_u32(to + 4 * lane, word);
    }
}

// Translated from 00df3110 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkKdTreeBuilderSparseWriter::hkKdTreeBuilderSparseWriter` (Xbox PDB), `thiscall`: clears
/// the word at `+0x100`. Answers `this`.
pub fn hk_kd_tree_builder_sparse_writer_hk_kd_tree_builder_sparse_writer(
    e: &mut Engine,
    this: Ptr,
) -> Ptr {
    e.mem.set_u32(this.addr() + 0x100, 0);
    this
}

// Translated from 00df3120 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl: `array`, `index`, `vector`): stores the 16-byte vector at
/// `vector` into the 16-byte slot `index` of `array`.
pub fn fn_00df3120(e: &mut Engine, array: u32, index: u32, vector: u32) {
    let slot = array.wrapping_add(index << 4);
    copy_vector(e, vector, slot);
}

// Translated from 00df3160 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkKdTreeBuilderSparseWriter::flush` (Xbox PDB), `thiscall`: does nothing.
pub fn hk_kd_tree_builder_sparse_writer_flush(_e: &mut Engine, _this: Ptr) {}

// Translated from 00df3170 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. Inputs are in registers, so the register values come first:
/// `source` (`EAX`), `destination` (`ECX`). Reads eight 16-bit words `w0..w7` at `source`
/// and writes four `float` lanes at `destination`: lane `i` is
/// `(0 * k + w[i + 4]) - (0 * k + w[i])` where `k` is the `float` at `010cd644` (the code
/// splits each word into a high half, always 0 for these inputs, times `k`, and a low
/// half; `CVTDQ2PS`, `MULPS`, `ADDPS`, `SUBPS` as in the exe). The `FLD`/`FST` of `k` quiets
/// a signalling NaN.
pub fn fn_00df3170(e: &mut Engine, source: u32, destination: u32) {
    let scale = x87_quiet(e.mem.f32(SCALE_FLOAT));
    let word = |e: &Engine, index: u32| f32::from(e.mem.u16(source + 2 * index));
    // The high halves are all 0 (`CVTDQ2PS` gives +0.0), multiplied by `k`.
    let high_times_scale = 0.0f32 * scale;
    for lane in 0..4 {
        let upper = high_times_scale + word(e, lane + 4);
        let lower = high_times_scale + word(e, lane);
        e.mem.set_f32(destination + 4 * lane, upper - lower);
    }
}

// Translated from 00df3210 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. The input is in a register: `target` (`EAX`). Zeroes the eight
/// 16-byte vectors at `target`. `EAX` is left at `target + 0x60`, which is answered.
pub fn fn_00df3210(e: &mut Engine, target: u32) -> u32 {
    for vector in 0..8 {
        for lane in 0..4 {
            e.mem.set_u32(target + 0x10 * vector + 4 * lane, 0);
        }
    }
    target.wrapping_add(0x60)
}

// Translated from 00df3250 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, one word): the leading-zero-count trick on the low 16
/// bits of the argument. Each step subtracts a bound (0x100, 0x1000, 0x4000) from the low
/// word, takes an arithmetic shift right by 16 and keeps bit 3, 2 or 1 to get a shift of
/// 8, 4 or 2 bits that is applied to the whole register; the shifts add up; the last step
/// looks at the top two bits `c` of the low word and subtracts `!(c >> 1) & c` from the sum
/// before adding 2. Answers the sum.
pub fn fn_00df3250(_e: &mut Engine, value: u32) -> u32 {
    let step = |edx: u32, bound: i32, mask: u32| -> (u32, u32) {
        let shifted = ((((edx & 0xffff) as i32).wrapping_sub(bound)) >> 16) as u32 & mask;
        (edx << shifted, shifted)
    };
    let (edx, mut total) = step(value, 0x100, 8);
    let (edx, add) = step(edx, 0x1000, 4);
    total += add;
    let (edx, add) = step(edx, 0x4000, 2);
    total += add;
    let top = (edx & 0xffff) >> 14;
    let both = !((top as i32) >> 1) as u32 & top;
    total.wrapping_sub(both).wrapping_add(2)
}

// Translated from 00df32a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl: `array`, `index`): sets the 8-byte entry `index` of
/// `array` to `{0x7fffffff, 0}`.
pub fn fn_00df32a0(e: &mut Engine, array: u32, index: u32) {
    let entry = array.wrapping_add(index.wrapping_mul(8));
    e.mem.set_u32(entry, 0x7fff_ffff);
    e.mem.set_u32(entry + 4, 0);
}

// Translated from 00df32d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl: `entries`, `source`, `destination`, `slot`, `index`):
/// copies the 16-byte vector `index` of `source` to `destination` and sets the 8-byte entry
/// `slot` of `entries` to `{index & 0x7fffffff, 1}`.
pub fn fn_00df32d0(
    e: &mut Engine,
    entries: u32,
    source: u32,
    destination: u32,
    slot: u32,
    index: u32,
) {
    let offset = index << 4;
    copy_vector(
        e,
        source.wrapping_add(offset),
        destination.wrapping_add(offset),
    );
    let entry = entries.wrapping_add(slot.wrapping_mul(8));
    e.mem.set_u32(entry, index & 0x7fff_ffff);
    e.mem.set_u32(entry + 4, 1);
}

// Translated from 00df3330 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl: `entries`, `source`, `destination`, `slot`, `index`,
/// `count`): sets the 8-byte entry `slot` of `entries` to `{index & 0x7fffffff, count}`
/// and, when `count` is positive (signed), copies `count` consecutive 16-byte vectors
/// from `source[index]` to `destination[index]`. Answers `2 * count - 2`.
pub fn fn_00df3330(
    e: &mut Engine,
    entries: u32,
    source: u32,
    destination: u32,
    slot: u32,
    index: u32,
    count: i32,
) -> u32 {
    let offset = index << 4;
    let entry = entries.wrapping_add(slot.wrapping_mul(8));
    e.mem.set_u32(entry, index & 0x7fff_ffff);
    e.mem.set_u32(entry + 4, count as u32);
    if count > 0 {
        let mut from = source.wrapping_add(offset);
        let mut to = destination.wrapping_add(offset);
        for _ in 0..count {
            copy_vector(e, from, to);
            from = from.wrapping_add(0x10);
            to = to.wrapping_add(0x10);
        }
    }
    (count as u32).wrapping_add(count as u32).wrapping_sub(2)
}

/// Copies `entries` 16-byte entries from `from` to `to` (`REP MOVSD` with the count the code
/// forms: `(entries & 0x0fffffff) * 4` dwords).
fn copy_entries(e: &mut Engine, from: u32, to: u32, entries: u32) {
    let dwords = (entries & 0x0fff_ffff).wrapping_mul(4);
    for dword in 0..dwords {
        let value = e.mem.u32(from.wrapping_add(4 * dword));
        e.mem.set_u32(to.wrapping_add(4 * dword), value);
    }
}

// Translated from 00df33b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl: `tree`, `start`, `end`, `split`, `buffers`, `bounds`;
/// answers 0 or 1). Splits the entries `start` to `end` of the first buffer (`tree + 0`, 16-byte
/// entries of eight 16-bit words: the minimum of x, y, z at `+0`, `+2`, `+4` and the maximum
/// at `+8`, `+0xa`, `+0xc`) between two groups written to the same places of the second
/// buffer (`tree + 4`). `bounds` holds the node's box in the same form; the axis is the
/// longest side (`max - min` as 16-bit words; with x longer than y the axis is x unless z is
/// not shorter than x, in which case it is z; otherwise it is y unless z is not shorter than
/// y, in which case it is z). The pivot is the sum of the entries' middles `(min + max) >> 1`
/// along the axis divided by the count (signed). `totalL` counts the entries whose minimum is at
/// most the pivot and `totalR` those whose maximum is at least it. When the two are equal and
/// equal to the count nothing can be split and 1 is answered (no outputs are written).
/// When one of them is 0 the entries are copied in order and cut in the middle (boundary `start +
/// count / 2`). Otherwise the groups follow the smaller of the two counts: with `totalL <
/// totalR` the entries with minimum at most the pivot go first (boundary `start + totalL`),
/// else the entries whose maximum is below the pivot do (boundary `end - totalR`).
/// `split` receives the axis (`+0`), the boundary (`+4`), the smallest minimum of the second
/// group (`+8`, a word) and the largest maximum of the first (`+0xa`), `buffers` receives
/// the buffers swapped (`+0` the second, `+4` the first). Answers 0.
pub fn fn_00df33b0(
    e: &mut Engine,
    tree: u32,
    start: i32,
    end: i32,
    split: u32,
    buffers: u32,
    bounds: u32,
) -> u32 {
    let first = e.mem.u32(tree);
    let second = e.mem.u32(tree + 4);
    let start_bytes = (start as u32) << 4;
    let entries = first.wrapping_add(start_bytes);
    let word = |e: &Engine, at: u32| u32::from(e.mem.u16(at));
    let range_x = e.mem.u16(bounds + 8).wrapping_sub(e.mem.u16(bounds));
    let range_y = e.mem.u16(bounds + 0xa).wrapping_sub(e.mem.u16(bounds + 2));
    let range_z = e.mem.u16(bounds + 0xc).wrapping_sub(e.mem.u16(bounds + 4));
    let axis: u32 = if range_x > range_y {
        if range_z < range_x {
            0
        } else {
            2
        }
    } else if range_z < range_y {
        1
    } else {
        2
    };
    let count = end.wrapping_sub(start);
    let low = |e: &Engine, entry: u32| word(e, entry + 2 * axis);
    let high = |e: &Engine, entry: u32| word(e, entry + 2 * axis + 8);
    let mut sum_even = 0i32;
    let mut sum_odd = 0i32;
    let mut handled = 0i32;
    let mut cursor = entries;
    if count >= 2 {
        let pairs = (((count - 2) as u32) >> 1) + 1;
        handled = (pairs * 2) as i32;
        for _ in 0..pairs {
            let middle = ((low(e, cursor) + high(e, cursor)) as i32) >> 1;
            sum_even = sum_even.wrapping_add(middle);
            let middle = ((low(e, cursor + 0x10) + high(e, cursor + 0x10)) as i32) >> 1;
            sum_odd = sum_odd.wrapping_add(middle);
            cursor = cursor.wrapping_add(0x20);
        }
    }
    let mut last_middle = 0i32;
    if handled < count {
        last_middle = ((high(e, cursor) + low(e, cursor)) as i32) >> 1;
    }
    let total = sum_odd.wrapping_add(sum_even).wrapping_add(last_middle);
    let pivot = total / count;
    let (mut low_even, mut high_even, mut low_odd, mut high_odd) = (0i32, 0i32, 0i32, 0i32);
    let mut cursor = entries;
    if count >= 2 {
        let pairs = (((count - 2) as u32) >> 1) + 1;
        for _ in 0..pairs {
            low_even += i32::from(low(e, cursor) as i32 <= pivot);
            high_even += i32::from(high(e, cursor) as i32 >= pivot);
            low_odd += i32::from(low(e, cursor + 0x10) as i32 <= pivot);
            high_odd += i32::from(high(e, cursor + 0x10) as i32 >= pivot);
            cursor = cursor.wrapping_add(0x20);
        }
    }
    let (mut low_last, mut high_last) = (0i32, 0i32);
    if handled < count {
        low_last = i32::from(low(e, cursor) as i32 <= pivot);
        high_last = i32::from(high(e, cursor) as i32 >= pivot);
    }
    let total_high = high_last + (high_odd + high_even);
    let total_low = low_last + (low_odd + low_even);
    let degenerate = total_low == 0 || total_high == 0;
    if total_low == total_high && total_low == count {
        return 1;
    }
    let mut destination = second.wrapping_add(start_bytes);
    let mut low_maximum = word(e, bounds + 2 * axis);
    let mut high_minimum = word(e, bounds + 2 * axis + 8);
    let boundary: i32;
    let mut source = entries;
    if degenerate {
        let half = count >> 1;
        boundary = start.wrapping_add(half);
        let mut copied = 0i32;
        if half > 0 {
            copy_entries(e, source, destination, half as u32);
            destination = destination.wrapping_add((half as u32) << 4);
            for _ in 0..half {
                let maximum = high(e, source);
                if (low_maximum as u16) <= maximum as u16 {
                    low_maximum = maximum;
                }
                source = source.wrapping_add(0x10);
            }
            copied = half;
        }
        if copied < count {
            let remaining = count - copied;
            copy_entries(e, source, destination, remaining as u32);
            let mut cursor = source;
            for _ in 0..remaining {
                let minimum = low(e, cursor);
                if (high_minimum as u16) >= minimum as u16 {
                    high_minimum = minimum;
                }
                cursor = cursor.wrapping_add(0x10);
            }
        }
    } else if total_low < total_high {
        boundary = start.wrapping_add(total_low);
        let mut right = second.wrapping_add((boundary as u32) << 4);
        for _ in 0..count.max(0) {
            let minimum = low(e, source);
            let maximum = high(e, source);
            let entry = load_vec4(e, source);
            if minimum as i32 > pivot {
                store_vec4(e, right, entry);
                right = right.wrapping_add(0x10);
                if (high_minimum as u16) >= minimum as u16 {
                    high_minimum = minimum;
                }
            } else {
                store_vec4(e, destination, entry);
                destination = destination.wrapping_add(0x10);
                if (low_maximum as u16) <= maximum as u16 {
                    low_maximum = maximum;
                }
            }
            source = source.wrapping_add(0x10);
        }
    } else {
        boundary = count.wrapping_sub(total_high).wrapping_add(start);
        let mut right = second.wrapping_add((boundary as u32) << 4);
        for _ in 0..count.max(0) {
            let minimum = low(e, source);
            let maximum = high(e, source);
            let entry = load_vec4(e, source);
            if (maximum as i32) < pivot {
                store_vec4(e, destination, entry);
                destination = destination.wrapping_add(0x10);
                if (low_maximum as u16) <= maximum as u16 {
                    low_maximum = maximum;
                }
            } else {
                store_vec4(e, right, entry);
                right = right.wrapping_add(0x10);
                if (high_minimum as u16) >= minimum as u16 {
                    high_minimum = minimum;
                }
            }
            source = source.wrapping_add(0x10);
        }
    }
    e.mem.set_u16(split + 8, high_minimum as u16);
    e.mem.set_u16(split + 0xa, low_maximum as u16);
    e.mem.set_u32(split, axis);
    e.mem.set_i32(split + 4, boundary);
    e.mem.set_u32(buffers + 4, first);
    e.mem.set_u32(buffers, second);
    0
}

/// `0.125` as a `float` (`0105559c`), `8.0` as a `double` (`010240b0`) and the largest `float`
/// (`0104da48`) that `00df4ec0` uses to cost a split.
const ZERO_FLOAT: u32 = 0x0101_1d78;
const COST_FACTOR: u32 = 0x0105_559c;
const AREA_STEP: u32 = 0x0102_40b0;
const WORST_COST: u32 = 0x0104_da48;

/// Four dwords at `at` (a 16-byte entry copied with `MOV` dwords or `MOVAPS`).
fn load_words(e: &Engine, at: u32) -> [u32; 4] {
    [0, 1, 2, 3].map(|i| e.mem.u32(at + 4 * i))
}

fn store_words(e: &mut Engine, at: u32, words: [u32; 4]) {
    for (i, word) in words.into_iter().enumerate() {
        e.mem.set_u32(at + 4 * i as u32, word);
    }
}

// Translated from 00df4ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl: `tree`, `start`, `end`, `split`, `counter`, `buffers`,
/// `bounds`, `mask`; answers 0, or what `00df33b0` answers for fewer than 8 entries). Chooses
/// the cut of the entries `start` to `end` of the first buffer (`tree + 0`; entries as in
/// `00df33b0`), writing the two groups to the second buffer (`tree + 4`); `split` receives the
/// axis (`+0`), the boundary (`+4`), the smallest minimum of the second group (`+8`, a word)
/// and the largest maximum of the first (`+0xa`); `buffers` receives the buffers swapped.
///
/// Two entries are ordered by their minimum along the axis of `bounds` (as `00df33b0` picks it)
/// and the boundary is `start + 1`. Fewer than 8 entries go to `00df33b0`. Otherwise the code
/// takes the three extents of `bounds` as `float`s (`00df3170`), the face areas `(ey * ez,
/// ez * ex, ex * ey)` and the costs `0.125 * e_i * (e_j + e_k)` (all lanes of `SSE`
/// products), a shift per axis (`max(0, 13 - 00df3250(range))`) and builds, from every
/// `step`-th entry (1 up to 5 as the count passes 10, 100, 1000, 10000), two histograms of 8
/// bins per axis: the bins of the minimum (`00df3210` clears them) and of the maximum, in the
/// frame at `+0x100` and `+0x180`. For each axis and each of the 7 boundaries between bins
/// the cost is `left * leftArea + right * rightArea` (x87: the areas start at the face area and
/// at `8 * cost + area` and move by `cost` per bin; `left` counts minima and `right` the
/// maxima above the boundary); the cheapest wins when it is allowed: with `*counter > 1` when the
/// axis bit is set in the sign mask of the vector `mask`, or when both counts are above 0, and
/// without it when both counts are above 0. The entries are then distributed by the
/// chosen boundary `((bin + 1) << shift) + min` (minimum below it, or maximum below it when the
/// left count was the full sample) with the extent trackers kept as in `00df33b0`; when no split was
/// found the entries are copied in order and cut in the middle. `*counter` goes down by 2 when
/// the boundary is `start` or `end`.
pub fn fn_00df4ec0(
    e: &mut Engine,
    tree: u32,
    start: i32,
    end: i32,
    split: u32,
    counter: u32,
    buffers: u32,
    bounds: u32,
    mask: u32,
) -> u32 {
    let count = end.wrapping_sub(start);
    let finish = |e: &mut Engine| {
        let first = e.mem.u32(tree);
        let second = e.mem.u32(tree + 4);
        e.mem.set_u32(buffers + 4, first);
        e.mem.set_u32(buffers, second);
    };
    if count == 2 {
        let first = e.mem.u32(tree);
        let second = e.mem.u32(tree + 4);
        let offset = (start as u32) << 4;
        let entry0 = first.wrapping_add(offset);
        let destination = second.wrapping_add(offset);
        let words0 = load_words(e, entry0);
        let words1 = load_words(e, entry0.wrapping_add(0x10));
        let range_x = e.mem.u16(bounds + 8).wrapping_sub(e.mem.u16(bounds));
        let range_y = e.mem.u16(bounds + 0xa).wrapping_sub(e.mem.u16(bounds + 2));
        let range_z = e.mem.u16(bounds + 0xc).wrapping_sub(e.mem.u16(bounds + 4));
        let axis: u32 = if range_x > range_y {
            if range_z < range_x {
                0
            } else {
                2
            }
        } else if range_z < range_y {
            1
        } else {
            2
        };
        let half_word = |words: &[u32; 4], index: u32| -> u16 {
            let word = words[(index / 2) as usize];
            if index % 2 == 0 {
                word as u16
            } else {
                (word >> 16) as u16
            }
        };
        let minimum1 = half_word(&words1, axis);
        let minimum0 = half_word(&words0, axis);
        let boundary = start.wrapping_add(1);
        if minimum0 > minimum1 {
            store_words(e, destination, words1);
            store_words(e, destination + 0x10, words0);
            e.mem.set_u16(split + 0xa, half_word(&words1, axis + 4));
            e.mem.set_u16(split + 8, minimum0);
        } else {
            store_words(e, destination, words0);
            store_words(e, destination + 0x10, words1);
            e.mem.set_u16(split + 0xa, half_word(&words0, axis + 4));
            e.mem.set_u16(split + 8, minimum1);
        }
        e.mem.set_u32(split, axis);
        e.mem.set_i32(split + 4, boundary);
        finish(e);
        return 0;
    }
    if count < 8 {
        return fn_00df33b0(e, tree, start, end, split, buffers, bounds);
    }
    e.with_stack(0x200, |e, frame| {
        let s = frame.addr();
        let buffer1 = e.mem.u32(tree);
        fn_00df3170(e, bounds, s + 0x80);
        let extents = load_vec4(e, s + 0x80);
        let zero = e.mem.f32(ZERO_FLOAT);
        let [ex, ey, ez, _] = extents;
        let first = [ez, ex, ey, zero];
        let second = [ey, ez, ex, zero];
        let areas = mul4(first, second);
        store_vec4(e, s + 0xe0, areas);
        let factor = [e.mem.f32(COST_FACTOR); 4];
        let sums = add4(first, second);
        let costs = mul4(mul4(factor, load_vec4(e, s + 0x80)), sums);
        store_vec4(e, s + 0xf0, costs);
        fn_00df3210(e, s + 0x100);
        fn_00df3210(e, s + 0x180);
        // The shifts of the three axes.
        for axis in 0..3u32 {
            let range = e
                .mem
                .u16(bounds + 8 + 2 * axis)
                .wrapping_sub(e.mem.u16(bounds + 2 * axis));
            let leading = fn_00df3250(e, u32::from(range));
            let shift = 13i32.wrapping_sub(leading as i32);
            let shift = if shift <= 0 { 0 } else { shift };
            e.mem.set_i32(s + 0x80 + 4 * axis, shift);
        }
        let entries = buffer1.wrapping_add((start as u32) << 4);
        let step: i32 = if count > 10000 {
            5
        } else if count > 1000 {
            4
        } else if count > 100 {
            3
        } else if count > 10 {
            2
        } else {
            1
        };
        let stride_bytes = (step as u32) << 4;
        let minimum = [0u32, 1, 2].map(|axis| u32::from(e.mem.u16(bounds + 2 * axis)));
        if start < end {
            let mut index = start;
            let mut cursor = entries;
            while index < end {
                let dword0 = e.mem.u32(cursor);
                let dword2 = e.mem.u32(cursor + 8);
                let shift_x = e.mem.u32(s + 0x80);
                let shift_y = e.mem.u32(s + 0x84);
                let shift_z = e.mem.u32(s + 0x88);
                let bin = |value: u32, base: u32, shift: u32| -> u32 {
                    (((value.wrapping_sub(base)) as i32) >> (shift & 31)) as u32
                };
                let bump = |e: &mut Engine, at: u32| {
                    let value = e.mem.u32(at).wrapping_add(1);
                    e.mem.set_u32(at, value);
                };
                let bin_at = |base: u32, bin: u32, lane: u32| {
                    s.wrapping_add(base)
                        .wrapping_add(bin << 4)
                        .wrapping_add(lane)
                };
                let b = bin(dword0 & 0xffff, minimum[0], shift_x);
                bump(e, bin_at(0x100, b, 0));
                let b = bin(dword2 & 0xffff, minimum[0], shift_x);
                bump(e, bin_at(0x180, b, 0));
                let b = bin(dword0 >> 16, minimum[1], shift_y);
                bump(e, bin_at(0x100, b, 4));
                let b = bin(dword2 >> 16, minimum[1], shift_y);
                bump(e, bin_at(0x180, b, 4));
                let b = bin(u32::from(e.mem.u16(cursor + 4)), minimum[2], shift_z);
                bump(e, bin_at(0x100, b, 8));
                let b = bin(u32::from(e.mem.u16(cursor + 0xc)), minimum[2], shift_z);
                bump(e, bin_at(0x180, b, 8));
                cursor = cursor.wrapping_add(stride_bytes);
                index = index.wrapping_add(step);
            }
        }
        let sampled = (step.wrapping_add(count).wrapping_sub(1)) / step;
        e.mem.set_f32(s + 0xb8, x87_quiet(e.mem.f32(WORST_COST)));
        let wide = e.mem.i32(counter) > 1;
        let mask_bits = {
            let lanes = load_bits(e, mask);
            (0..4).fold(0u32, |bits, i| bits | ((lanes[i] >> 31) << i))
        };
        let axis_bits = [1u32, 2, 4];
        let mut best_axis = 0u32;
        let mut best_bin = 0u32;
        let mut best_left = sampled;
        let mut best_right = sampled;
        let mut x = X87::new();
        x.fld(e.mem.f64(AREA_STEP));
        for axis in 0..3u32 {
            let mut right = sampled;
            let mut left = 0i32;
            x.fld_f32(e.mem.f32(s + 0xf0 + 4 * axis));
            let value = x.fstp_f32();
            e.mem.set_f32(s + 0x6c, value);
            x.fld_f32(e.mem.f32(s + 0xe0 + 4 * axis));
            let value = x.fstp_f32();
            e.mem.set_f32(s + 0x74, value);
            x.fld_f32(e.mem.f32(s + 0x6c));
            x.fld_st(0);
            x.mul(0, 2);
            x.add_m(float_to_x87(e.mem.f32(s + 0xe0 + 4 * axis)));
            let value = x.fstp_f32();
            e.mem.set_f32(s + 0x6c, value);
            x.fld_f32(e.mem.f32(s + 0x74));
            x.fld_f32(e.mem.f32(s + 0x6c));
            for bin in 0..7u32 {
                let offset = 4 * axis + 0x10 * bin;
                right = right.wrapping_sub(e.mem.i32(s + 0x180 + offset));
                x.fld_st(2);
                left = left.wrapping_add(e.mem.i32(s + 0x100 + offset));
                x.addp(2);
                x.fxch(1);
                e.mem.set_i32(s + 0xdc, right);
                let value = x.fstp_f32();
                e.mem.set_f32(s + 0x74, value);
                e.mem.set_i32(s + 0xac, left);
                x.sub(0, 1);
                let value = x.fstp_f32();
                e.mem.set_f32(s + 0x6c, value);
                x.fild(e.mem.i32(s + 0xdc));
                x.fld_f32(e.mem.f32(s + 0x6c));
                x.fld_st(0);
                x.mulp(2);
                x.fild(e.mem.i32(s + 0xac));
                x.fld_f32(e.mem.f32(s + 0x74));
                x.fld_st(0);
                x.mulp(2);
                x.fxch(3);
                x.addp(1);
                let value = x.fstp_f32();
                e.mem.set_f32(s + 0xac, value);
                let both = left > 0 && right > 0;
                let allowed = if wide {
                    mask_bits & axis_bits[axis as usize] != 0 || both
                } else {
                    both
                };
                x.fld_f32(e.mem.f32(s + 0xac));
                x.fld_f32(e.mem.f32(s + 0xb8));
                let flags = x.fcom(1);
                x.pop();
                if flags & 0x41 == 0 && allowed {
                    let value = x.fstp_f32();
                    e.mem.set_f32(s + 0xb8, value);
                    best_axis = axis;
                    best_bin = bin;
                    best_left = left;
                    best_right = right;
                } else {
                    x.pop();
                }
            }
            x.fstp_st(2);
            x.fstp_st(1);
            x.pop();
        }
        x.pop();
        assert!(x.is_empty());
        let shift = e.mem.u32(s + 4 * best_axis + 0x80);
        let lower = u32::from(e.mem.u16(bounds + 2 * best_axis));
        let pivot = (1i32.wrapping_shl(shift & 31))
            .wrapping_mul(best_bin as i32 + 1)
            .wrapping_add(lower as i32);
        let mut right_minimum = u32::from(e.mem.u16(bounds + 2 * best_axis + 8));
        let mut left_maximum = lower;
        let second = e.mem.u32(tree + 4);
        let mut destination = second.wrapping_add((start as u32) << 4);
        let mut from_end = second.wrapping_add((end as u32) << 4).wrapping_sub(0x10);
        let source_end = buffer1.wrapping_add((end as u32) << 4);
        let mut source = entries;
        let low = |e: &Engine, entry: u32| u32::from(e.mem.u16(entry + 2 * best_axis));
        let high = |e: &Engine, entry: u32| u32::from(e.mem.u16(entry + 2 * best_axis + 8));
        let boundary: i32;
        if best_left < sampled {
            while source < source_end {
                let minimum = low(e, source);
                let entry = load_words(e, source);
                if (minimum as i32) < pivot {
                    let maximum = high(e, source);
                    store_words(e, destination, entry);
                    destination = destination.wrapping_add(0x10);
                    if (left_maximum as u16) <= maximum as u16 {
                        left_maximum = maximum;
                    }
                } else {
                    store_words(e, from_end, entry);
                    from_end = from_end.wrapping_sub(0x10);
                    if (right_minimum as u16) >= minimum as u16 {
                        right_minimum = minimum;
                    }
                }
                source = source.wrapping_add(0x10);
            }
            boundary = (destination.wrapping_sub(second) as i32) >> 4;
        } else if best_right < sampled {
            while source < source_end {
                let maximum = high(e, source);
                let entry = load_words(e, source);
                if (maximum as i32) < pivot {
                    store_words(e, destination, entry);
                    destination = destination.wrapping_add(0x10);
                    if (left_maximum as u16) <= maximum as u16 {
                        left_maximum = maximum;
                    }
                } else {
                    store_words(e, from_end, entry);
                    from_end = from_end.wrapping_sub(0x10);
                    let minimum = low(e, source);
                    if (right_minimum as u16) >= minimum as u16 {
                        right_minimum = minimum;
                    }
                }
                source = source.wrapping_add(0x10);
            }
            boundary = (destination.wrapping_sub(second) as i32) >> 4;
        } else {
            let middle = start.wrapping_add(count >> 1);
            boundary = middle;
            let mut reached = start;
            if start < middle {
                let first_part = middle - start;
                reached = start.wrapping_add(first_part);
                for _ in 0..first_part {
                    let maximum = high(e, source);
                    let entry = load_words(e, source);
                    store_words(e, destination, entry);
                    destination = destination.wrapping_add(0x10);
                    if (left_maximum as u16) <= maximum as u16 {
                        left_maximum = maximum;
                    }
                    source = source.wrapping_add(0x10);
                }
            }
            if reached < end {
                for _ in 0..end - reached {
                    let minimum = low(e, source);
                    let entry = load_words(e, source);
                    store_words(e, destination, entry);
                    destination = destination.wrapping_add(0x10);
                    if (right_minimum as u16) >= minimum as u16 {
                        right_minimum = minimum;
                    }
                    source = source.wrapping_add(0x10);
                }
            }
        }
        if boundary == start || boundary == end {
            let value = e.mem.i32(counter).wrapping_sub(2);
            e.mem.set_i32(counter, value);
        }
        e.mem.set_u16(split + 8, right_minimum as u16);
        e.mem.set_u16(split + 0xa, left_maximum as u16);
        e.mem.set_i32(split + 4, boundary);
        e.mem.set_u32(split, best_axis);
        finish(e);
        0
    })
}

/// The static vectors `buildTreeRecursive` gives the two halves (`01268460`, sixteen bytes per
/// entry, indexed by `6`, `5`, `3` for the axes 0, 1, 2; and the vector at `012684d0`).
const AXIS_MASK_TABLE: u32 = 0x0126_8460;
const DEFAULT_MASK: u32 = 0x0126_84d0;

// Translated from 00df56a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkKdTreeBuilder<hkKdTreeProjectedEntry const *, hkKdTreeProjectedEntry *
/// __restrict>::buildTreeRecursive` (Xbox PDB), cdecl (`range`, `entries`, `destination`,
/// `counter`, `total`; answers the depth). `range` is the node's description: `+0` the pointer
/// to the buffer pair `{first, second}`, `+4` the node's slot, `+8`, `+0xc` the first entry,
/// `+0x10` the end, `+0x20` its box (eight 16-bit words), `+0x30` a word and `+0x40` a
/// vector; `entries` points at the pointer to the table of 8-byte nodes written (the node
/// `+4` of the range). With no entry the node is made with `00df32a0` (slot `+4`) and 1 is
/// answered; with one entry `00df32d0` (`*entries`, the first buffer, `destination`, slot, first
/// entry) copies it; with up to 4 `00df3330` (the same, and the count) does it and adds what it
/// answers to `*total`; both answer 1. With more `00df4ec0` chooses the cut (its split record
/// is the local `{axis, boundary, right minimum word, left maximum word}`, the buffers it
/// swapped go to the children): if it answers 1 the entries are handled as for up to 4 (the
/// count is the whole range); otherwise the node `*entries + 8 * slot` becomes
/// `((counter + 1 - slot) * 8 | (axis & 3) | 0x80000000, {right minimum, left maximum})`,
/// `*counter` goes up by 2 and the two halves (`[start, boundary)` and `[boundary, end)`, slots
/// `counter + 1` and `counter + 2`, the buffers swapped, the box of the half cut at the left
/// maximum / right minimum on the axis) are built recursively. Each half's vector is the
/// parent's `+0x40` vector ANDed with the table entry of `{6, 5, 3}[axis]` at `01268460` when the
/// boundary is the start or the end of the range, else the vector at `012684d0`. The answer is
/// the larger depth of the two halves plus 1.
pub fn hk_kd_tree_builder_build_tree_recursive(
    e: &mut Engine,
    range: u32,
    entries: u32,
    destination: u32,
    counter: u32,
    total: u32,
) -> i32 {
    let start = e.mem.i32(range + 0xc);
    let end = e.mem.i32(range + 0x10);
    let count = end.wrapping_sub(start);
    let tree = e.mem.u32(range);
    let slot = e.mem.u32(range + 4);
    let table = e.mem.u32(entries);
    if count == 0 {
        fn_00df32a0(e, table, slot);
        return count.wrapping_add(1);
    }
    if count == 1 {
        let source = e.mem.u32(tree);
        fn_00df32d0(e, table, source, destination, slot, start as u32);
        return count;
    }
    if count <= 4 {
        let source = e.mem.u32(tree);
        let added = fn_00df3330(e, table, source, destination, slot, start as u32, count);
        let value = e.mem.u32(total).wrapping_add(added);
        e.mem.set_u32(total, value);
        return 1;
    }
    e.with_stack(0x110, |e, frame| {
        let base = frame.addr();
        let buffers = base;
        let split = base + 0x10;
        let data_first = base + 0x20;
        let data_second = base + 0x28;
        let child_first = base + 0x30;
        let child_second = base + 0x80;
        let result = fn_00df4ec0(
            e,
            tree,
            start,
            end,
            split,
            total,
            buffers,
            range + 0x20,
            range + 0x40,
        );
        if result == 1 {
            let source = e.mem.u32(tree);
            let added = fn_00df3330(e, table, source, destination, slot, start as u32, count);
            let value = e.mem.u32(total).wrapping_add(added);
            e.mem.set_u32(total, value);
            return 1;
        }
        let axis = e.mem.u32(split);
        let boundary = e.mem.i32(split + 4);
        let bounds_word = e.mem.u32(split + 8);
        let right_minimum = e.mem.u16(split + 8);
        let left_maximum = e.mem.u16(split + 0xa);
        let used = e.mem.i32(counter);
        e.mem.set_i32(counter, used.wrapping_add(2));
        let first_slot = used.wrapping_add(1);
        let node = table.wrapping_add(slot.wrapping_mul(8));
        let head = (first_slot.wrapping_sub(slot as i32) as u32).wrapping_mul(8)
            | (axis & 3)
            | 0x8000_0000;
        e.mem.set_u32(node, head);
        e.mem.set_u32(node + 4, bounds_word);
        let new_first = e.mem.u32(buffers);
        let new_second = e.mem.u32(buffers + 4);
        for data in [data_first, data_second] {
            e.mem.set_u32(data, new_first);
            e.mem.set_u32(data + 4, new_second);
        }
        let range_eight = e.mem.u32(range + 8);
        let range_word = e.mem.u32(range + 0x30);
        let bounds = load_words(e, range + 0x20);
        let mask_pointer = range + 0x40;
        let default_mask = load_words(e, DEFAULT_MASK);
        let mut mask = default_mask;
        if boundary == end || boundary == start {
            let parent = load_words(e, mask_pointer);
            let selected = [6u32, 5, 3][(axis as usize) % 3];
            let table_vector = load_words(e, AXIS_MASK_TABLE + (selected << 4));
            mask = [0, 1, 2, 3].map(|i| parent[i] & table_vector[i]);
        }
        for (child, data, from, to, second_slot) in [
            (child_first, data_first, start, boundary, first_slot),
            (
                child_second,
                data_second,
                boundary,
                end,
                first_slot.wrapping_add(1),
            ),
        ] {
            e.mem.set_u32(child, data);
            e.mem.set_i32(child + 4, second_slot);
            e.mem.set_u32(child + 8, range_eight);
            e.mem.set_i32(child + 0xc, from);
            e.mem.set_i32(child + 0x10, to);
            store_words(e, child + 0x20, bounds);
            e.mem.set_u32(child + 0x30, range_word);
            store_words(e, child + 0x40, mask);
        }
        e.mem.set_u16(child_first + 0x28 + 2 * axis, left_maximum);
        e.mem.set_u16(child_second + 0x20 + 2 * axis, right_minimum);
        let first_depth = hk_kd_tree_builder_build_tree_recursive(
            e,
            child_first,
            entries,
            destination,
            counter,
            total,
        );
        let second_depth = hk_kd_tree_builder_build_tree_recursive(
            e,
            child_second,
            entries,
            destination,
            counter,
            total,
        );
        let deeper = if first_depth > second_depth {
            first_depth
        } else {
            second_depth
        };
        deeper.wrapping_add(1)
    })
}
/// The Havok report sink's global (`01268360`, a pointer to the object whose slot `0xc` takes a
/// report), the `hkOstream` routines `splitTreeBuilding` writes its assertion with (`00cb1600`
/// `thiscall`: buffer, size; `00cc01c0` `thiscall`: text; `00cc00e0` the destructor), the text
/// (`010da240`) and file name (`010da21c`) strings it passes, and the two `unsigned short`
/// helpers it calls through (`00cfcb80`, `00d19d70`: the smaller and the larger of two words).
const REPORT_SINK: u32 = 0x0126_8360;
const OSTREAM_CONSTRUCT: u32 = 0x00cb_1600;
const OSTREAM_WRITE_TEXT: u32 = 0x00cc_01c0;
const OSTREAM_DESTRUCT: u32 = 0x00cc_00e0;
const FALLBACK_MESSAGE: u32 = 0x010d_a240;
const SOURCE_FILE_NAME: u32 = 0x010d_a21c;
const MIN_WORD: u32 = 0x00cf_cb80;
const MAX_WORD: u32 = 0x00d1_9d70;

/// `|left - right|` as the code forms it: the `int` difference, `FILD`, `FABS` and a store as a
/// `float`.
fn abs_difference(left: i32, right: i32) -> f32 {
    x87_to_float(f64::from(left.wrapping_sub(right)).abs())
}

// Translated from 00df37b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkKdTreeBuilder<hkKdTreeProjectedEntry const *, hkKdTreeProjectedEntry *
/// __restrict>::splitTreeBuilding` (Xbox PDB), cdecl (`tree`, `nodes`, `capacity`, `count`,
/// `ranges`, `counts`, `bounds`). Cuts the `count` entries of the first buffer (`*tree`;
/// entries as in `00df33b0`, `bounds` the box that holds them) into four groups along one
/// axis and writes the top two levels of the tree.
///
/// Two histograms of 64 bins per axis count the minimum and the maximum (`word >> 10`) of
/// every `step`-th entry (1 to 5 as `count` passes 10, 100, 1000 and 10000; `sampled =
/// count / step`). For each axis the bins give three cuts, each the bin after which the number
/// of entries on its left (minima counted so far) is nearest to the number on its right
/// (what remains once the maxima are taken away; the first one met wins, the difference is
/// compared as a `float` against the previous best, which starts as the largest `float`
/// `0104da48`): the median `k1` (bin 1 to 61), then the cut of the left part (bins 0 to
/// `k1 - 1`, `p_left`) and the one of the right (bins `k1 + 1` to 62, `p_right`). The four
/// counts `g` they leave are scored with the sum of `|g_i - g_j|` over the six pairs (summed
/// as `float`s in the code's order); the axis with the smaller score (the first on a tie)
/// wins.
///
/// The pivots `(p_left + 1) << 10`, `(k1 + 1) << 10` and `(p_right + 1) << 10` split the
/// entries by their minimum on that axis into four groups (compared as `int`s); the exact
/// counts and, per group, the smallest minimum and the largest maximum are collected. When one
/// of the four counts from the histograms is 0, or the entries do not fall into four non-empty
/// groups, the entries are instead cut in quarters in order (`count / 4` each, the last
/// group the rest); `capacity` must then be at least `count / 4 * 8 + 8` or an assertion is
/// reported (`hkOstream` over a 0x200-byte buffer: "Not enough memory allocated for balanced
/// fallback", file `.\KdTree\Build\hkKdTreeBuilder.cpp`, line 0x198, sent to the sink at
/// `01268360` with slot `0xc`; an answer other than 0 hits an `INT3`, which is not modelled:
/// the code goes on as after a resumed breakpoint). The groups are copied to the second buffer
/// (`tree + 4`) one after the other.
///
/// `nodes` gets the cut as a small tree: the node 0 and the nodes 2 and 3 as `(axis & 3) |
/// 0x80000010` / `0x80000018` heads with the payloads `(highest maximum of the groups 0 and 1
/// << 16) | lowest minimum of the groups 2 and 3`, `(maximum 0 << 16) | minimum 1` and
/// `(maximum 2 << 16) | minimum 3`. The four descriptors at `ranges` (0x50 bytes each, with
/// the pair of buffers their first word points at) get that pair swapped (`{second, first}`),
/// the slot 0, the start and end of their group (`+0xc`, `+0x10`), `capacity` at `+8`, a copy of
/// `bounds` at `+0x20` with the axis range replaced by the group's minimum and maximum, and
/// `counts[k]` is twice the group's count.
pub fn hk_kd_tree_builder_split_tree_building(
    e: &mut Engine,
    tree: u32,
    nodes: u32,
    capacity: i32,
    count: i32,
    ranges: u32,
    counts: u32,
    bounds: u32,
) {
    let first = e.mem.u32(tree);
    let mut minima = [[0i32; 4]; 64];
    let mut maxima = [[0i32; 4]; 64];
    let step: i32 = if count > 10000 {
        5
    } else if count > 1000 {
        4
    } else if count > 100 {
        3
    } else if count > 10 {
        2
    } else {
        1
    };
    if count > 0 {
        let mut cursor = first;
        let mut index = 0i32;
        loop {
            let dword0 = e.mem.u32(cursor);
            let dword1 = e.mem.u32(cursor + 4);
            let dword2 = e.mem.u32(cursor + 8);
            let maximum_z = u32::from(e.mem.u16(cursor + 0xc));
            index = index.wrapping_add(step);
            minima[((dword0 & 0xffff) >> 10) as usize][0] += 1;
            minima[((dword0 >> 16) >> 10) as usize][1] += 1;
            minima[((dword1 & 0xffff) >> 10) as usize][2] += 1;
            cursor = cursor.wrapping_add((step as u32) << 4);
            maxima[((dword2 & 0xffff) >> 10) as usize][0] += 1;
            maxima[((dword2 >> 16) >> 10) as usize][1] += 1;
            maxima[(maximum_z >> 10) as usize][2] += 1;
            if index >= count {
                break;
            }
        }
    }
    let sampled = count.wrapping_div(step);
    let largest = x87_quiet(e.mem.f32(WORST_COST));
    let mut best_score = largest;
    // The best cut so far: the axis, `k1`, `p_left`, `p_right` and the four counts (the
    // exe starts the indices at -1).
    let mut best_axis = -1i32;
    let mut best_median = -1i32;
    let mut best_left_cut = -1i32;
    let mut best_right_cut = -1i32;
    let mut best_counts = [0i32; 4];
    for axis in 0..3usize {
        let mut left = minima[0][axis];
        let mut right = sampled.wrapping_sub(maxima[0][axis]);
        let mut best = largest;
        let (mut median_left, mut median_right, mut median) = (0i32, 0i32, 0usize);
        for bin in 1..62usize {
            left = left.wrapping_add(minima[bin][axis]);
            right = right.wrapping_sub(maxima[bin][axis]);
            let value = abs_difference(left, right);
            if value < best {
                best = value;
                median_left = left;
                median_right = right;
                median = bin;
            }
        }
        // The left part: bins 0 to `median - 1`, starting from the entries left of the median.
        let mut left = 0i32;
        let mut right = median_left;
        let mut best = largest;
        let (mut group0, mut group1, mut left_cut) = (0i32, 0i32, 0usize);
        for bin in 0..median {
            left = left.wrapping_add(minima[bin][axis]);
            right = right.wrapping_sub(maxima[bin][axis]);
            let value = abs_difference(left, right);
            if value < best {
                best = value;
                group0 = left;
                group1 = right;
                left_cut = bin;
            }
        }
        // The right part: bins `median + 1` to 62, starting from the entries right of it.
        let mut left = 0i32;
        let mut right = median_right;
        let mut best = largest;
        let (mut group2, mut group3, mut right_cut) = (0i32, 0i32, 0usize);
        for bin in median + 1..63 {
            left = left.wrapping_add(minima[bin][axis]);
            right = right.wrapping_sub(maxima[bin][axis]);
            let value = abs_difference(left, right);
            if value < best {
                best = value;
                group2 = left;
                group3 = right;
                right_cut = bin;
            }
        }
        let groups = [group0, group1, group2, group3];
        let mut score = 0.0f32;
        for i in 0..3usize {
            for j in i..4usize {
                let term = abs_difference(groups[i], groups[j]);
                score = x87_to_float(float_to_x87(score) + float_to_x87(term));
            }
        }
        if score < best_score {
            best_score = score;
            best_left_cut = left_cut as i32;
            best_median = median as i32;
            best_right_cut = right_cut as i32;
            best_axis = axis as i32;
            best_counts = groups;
        }
    }
    let axis = best_axis as u32;
    let axis_offset = axis.wrapping_mul(2);
    let bounds_minimum = e.mem.u16(bounds.wrapping_add(axis_offset));
    let bounds_maximum = e.mem.u16(bounds.wrapping_add(axis_offset).wrapping_add(8));
    // The smallest minimum and the largest maximum of each group, and its count.
    let mut minimums = [bounds_maximum; 4];
    let mut maximums = [bounds_minimum; 4];
    let mut sizes = [0u32; 4];
    let pivot_left = ((best_left_cut as u32).wrapping_add(1) << 10) as i32;
    let pivot_median = ((best_median as u32).wrapping_add(1) << 10) as i32;
    let pivot_right = ((best_right_cut as u32).wrapping_add(1) << 10) as i32;
    let end = first.wrapping_add((count as u32) << 4);
    let mut fallback = best_counts.contains(&0);
    if !fallback {
        let mut cursor = first;
        while cursor < end {
            let minimum = e.mem.u16(cursor.wrapping_add(axis_offset));
            let maximum = e.mem.u16(cursor.wrapping_add(axis_offset).wrapping_add(8));
            let value = i32::from(minimum);
            let group = if value < pivot_left {
                0
            } else if value < pivot_median {
                1
            } else if value < pivot_right {
                2
            } else {
                3
            };
            sizes[group] += 1;
            if minimum <= minimums[group] {
                minimums[group] = minimum;
            }
            if maximum >= maximums[group] {
                maximums[group] = maximum;
            }
            cursor = cursor.wrapping_add(0x10);
        }
        if sizes.contains(&0) {
            fallback = true;
            minimums = [bounds_maximum; 4];
            maximums = [bounds_minimum; 4];
            sizes = [0; 4];
        }
    }
    let quarter = count.wrapping_add((count >> 31) & 3) >> 2;
    if fallback {
        if capacity < quarter.wrapping_mul(8).wrapping_add(8) {
            e.with_stack(0x40, |e, stream| {
                e.with_stack(0x200, |e, buffer| {
                    e.call(
                        OSTREAM_CONSTRUCT,
                        &args![stream.addr(), buffer.addr(), 0x200u32],
                    );
                    e.call(OSTREAM_WRITE_TEXT, &args![stream.addr(), FALLBACK_MESSAGE]);
                    let sink = e.global::<u32>(REPORT_SINK);
                    let answer = e.vcall(
                        sink,
                        0xc,
                        &args![
                            3u32,
                            0x5010_d60cu32,
                            buffer.addr(),
                            SOURCE_FILE_NAME,
                            0x198u32
                        ],
                    );
                    // A nonzero answer runs into an `INT3` (a debugger breakpoint).
                    let _breakpoint = answer.u32() != 0;
                    e.call(OSTREAM_DESTRUCT, &args![stream.addr()]);
                });
            });
        }
        let quarter_bytes = (quarter as u32) << 4;
        let ends = [
            first.wrapping_add(quarter_bytes),
            first.wrapping_add(quarter_bytes << 1),
            first.wrapping_add(quarter_bytes.wrapping_mul(3)),
            end,
        ];
        let mut cursor = first;
        for group in 0..4 {
            if cursor < ends[group] {
                let entries =
                    (ends[group].wrapping_sub(cursor).wrapping_sub(1) >> 4).wrapping_add(1);
                let mut word = cursor.wrapping_add(axis_offset).wrapping_add(8);
                let mut minimum = u32::from(minimums[group]);
                let mut maximum = u32::from(maximums[group]);
                sizes[group] = sizes[group].wrapping_add(entries);
                cursor = cursor.wrapping_add(entries << 4);
                for _ in 0..entries {
                    let low = u32::from(e.mem.u16(word.wrapping_sub(8)));
                    minimum = u32::from(e.call(MIN_WORD, &args![minimum, low]).u32() as u16);
                    let high = u32::from(e.mem.u16(word));
                    maximum = u32::from(e.call(MAX_WORD, &args![maximum, high]).u32() as u16);
                    word = word.wrapping_add(0x10);
                }
                minimums[group] = minimum as u16;
                maximums[group] = maximum as u16;
            }
        }
    }
    // The groups go to the second buffer one after the other.
    let second = e.mem.u32(tree + 4);
    let mut write = [
        second,
        second.wrapping_add(sizes[0] << 4),
        second.wrapping_add(sizes[0].wrapping_add(sizes[1]) << 4),
        second.wrapping_add(sizes[1].wrapping_add(sizes[2]).wrapping_add(sizes[0]) << 4),
    ];
    if !fallback {
        let mut cursor = first;
        while cursor < end {
            let value = i32::from(e.mem.u16(cursor.wrapping_add(axis_offset)));
            let group = if value < pivot_left {
                0
            } else if value < pivot_median {
                1
            } else if value < pivot_right {
                2
            } else {
                3
            };
            let entry = load_words(e, cursor);
            store_words(e, write[group], entry);
            write[group] = write[group].wrapping_add(0x10);
            cursor = cursor.wrapping_add(0x10);
        }
    } else {
        let quarter_bytes = (quarter as u32) << 4;
        let ends = [
            first.wrapping_add(quarter_bytes),
            first.wrapping_add(quarter_bytes << 1),
            first.wrapping_add(quarter_bytes.wrapping_mul(3)),
            end,
        ];
        let mut cursor = first;
        for group in 0..4 {
            if cursor < ends[group] {
                let entries =
                    (ends[group].wrapping_sub(cursor).wrapping_sub(1) >> 4).wrapping_add(1);
                copy_entries(e, cursor, write[group], entries);
                cursor = cursor.wrapping_add(entries << 4);
            }
        }
    }
    // The top of the tree: the node 0 and the nodes 2 and 3 below it.
    let left_maximum = if maximums[1] > maximums[0] {
        maximums[1]
    } else {
        maximums[0]
    };
    let right_minimum = if minimums[2] < minimums[3] {
        minimums[2]
    } else {
        minimums[3]
    };
    e.mem.set_u32(
        nodes + 4,
        (u32::from(left_maximum) << 16) | u32::from(right_minimum),
    );
    let head = axis & 3;
    e.mem.set_u32(nodes, head | 0x8000_0010);
    e.mem.set_u32(nodes + 0x10, head | 0x8000_0010);
    e.mem.set_u32(
        nodes + 0x14,
        (u32::from(maximums[0]) << 16) | u32::from(minimums[1]),
    );
    e.mem.set_u32(nodes + 0x18, head | 0x8000_0018);
    e.mem.set_u32(
        nodes + 0x1c,
        (u32::from(maximums[2]) << 16) | u32::from(minimums[3]),
    );
    // The four descriptors of the groups.
    let swapped_first = e.mem.u32(tree);
    let swapped_second = e.mem.u32(tree + 4);
    let starts = [
        0u32,
        sizes[0],
        sizes[0].wrapping_add(sizes[1]),
        sizes[0].wrapping_add(sizes[1]).wrapping_add(sizes[2]),
    ];
    for group in 0..4usize {
        let range = ranges.wrapping_add(0x50 * group as u32);
        let pair = e.mem.u32(range);
        e.mem.set_u32(pair, swapped_second);
        e.mem.set_u32(pair + 4, swapped_first);
        e.mem.set_u32(range + 0xc, starts[group]);
        e.mem.set_u32(range + 4, 0);
        e.mem
            .set_u32(range + 0x10, starts[group].wrapping_add(sizes[group]));
        let box_words = load_words(e, bounds);
        store_words(e, range + 0x20, box_words);
        e.mem.set_u16(
            range.wrapping_add(axis_offset).wrapping_add(0x20),
            minimums[group],
        );
        e.mem.set_u16(
            range.wrapping_add(axis_offset).wrapping_add(0x28),
            maximums[group],
        );
        e.mem.set_u32(range + 8, capacity as u32);
        e.mem.set_u32(
            counts + 4 * group as u32,
            sizes[group].wrapping_add(sizes[group]),
        );
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00df3110, hk_kd_tree_builder_sparse_writer_hk_kd_tree_builder_sparse_writer(Ptr) -> Ptr),
        entry!(0x00df3120, fn_00df3120(u32, u32, u32)),
        entry!(0x00df3160, hk_kd_tree_builder_sparse_writer_flush(Ptr)),
        entry!(0x00df3170, fn_00df3170(u32, u32)),
        entry!(0x00df3210, fn_00df3210(u32) -> u32),
        entry!(0x00df3250, fn_00df3250(u32) -> u32),
        entry!(0x00df32a0, fn_00df32a0(u32, u32)),
        entry!(0x00df32d0, fn_00df32d0(u32, u32, u32, u32, u32)),
        entry!(0x00df3330, fn_00df3330(u32, u32, u32, u32, u32, i32) -> u32),
        entry!(0x00df33b0, fn_00df33b0(u32, i32, i32, u32, u32, u32) -> u32),
        entry!(
            0x00df37b0,
            hk_kd_tree_builder_split_tree_building(u32, u32, i32, i32, u32, u32, u32)
        ),
        entry!(0x00df4ec0, fn_00df4ec0(u32, i32, i32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00df56a0, hk_kd_tree_builder_build_tree_recursive(u32, u32, u32, u32, u32) -> i32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_sparse_writer_constructor_and_flush() {
        let mut e = Engine::new();
        let writer = e.mem.alloc(0x110);
        e.mem.write(writer, &[0xcd; 0x110]);
        assert_eq!(e.call(0x00df_3110, &args![writer]).u32(), writer);
        assert_eq!(e.mem.u32(writer + 0x100), 0);
        assert_eq!(e.mem.u32(writer + 0xfc), 0xcdcd_cdcd);
        e.call_log = Some(vec![]);
        e.call(0x00df_3160, &args![writer]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn a_vector_is_stored_in_a_slot() {
        let mut e = Engine::new();
        let array = e.mem.alloc(0x40);
        let vector = e.mem.alloc(0x10);
        for lane in 0..4 {
            e.mem.set_u32(vector + 4 * lane, 0x11 + lane);
        }
        e.call(0x00df_3120, &args![array, 2u32, vector]);
        assert_eq!(e.mem.u32(array + 0x20), 0x11);
        assert_eq!(e.mem.u32(array + 0x2c), 0x14);
        assert_eq!(e.mem.u32(array + 0x1c), 0);
    }

    #[test]
    fn the_word_differences_become_floats() {
        let mut e = Engine::new();
        e.map(0x010c_d000, 0x1000);
        e.mem.set_f32(SCALE_FLOAT, 4.0);
        let source = e.mem.alloc(0x10);
        for (index, word) in [1u16, 2, 3, 4, 10, 20, 30, 40].into_iter().enumerate() {
            e.mem.set_u16(source + 2 * index as u32, word);
        }
        let destination = e.mem.alloc(0x10);
        e.call(0x00df_3170, &args![source, destination]);
        let lanes: Vec<f32> = (0..4).map(|i| e.mem.f32(destination + 4 * i)).collect();
        assert_eq!(lanes, vec![9.0, 18.0, 27.0, 36.0]);
        // A NaN scale poisons every lane (0 * NaN).
        e.mem.set_u32(SCALE_FLOAT, 0x7fc0_0000);
        e.call(0x00df_3170, &args![source, destination]);
        assert!(e.mem.f32(destination).is_nan());
    }

    #[test]
    fn eight_vectors_are_zeroed() {
        let mut e = Engine::new();
        let target = e.mem.alloc(0x90);
        e.mem.write(target, &[0xcd; 0x90]);
        assert_eq!(e.call(0x00df_3210, &args![target]).u32(), target + 0x60);
        assert_eq!(e.mem.bytes(target, 0x80), vec![0; 0x80]);
        assert_eq!(e.mem.u32(target + 0x80), 0xcdcd_cdcd);
    }

    #[test]
    fn the_leading_zero_trick_counts_sixteen_bit_zeros() {
        let mut e = Engine::new();
        let count = |e: &mut Engine, value: u32| e.call(0x00df_3250, &args![value]).u32();
        assert_eq!(count(&mut e, 0x8000), 0);
        assert_eq!(count(&mut e, 0x4000), 1);
        assert_eq!(count(&mut e, 0x00ff), 8);
        assert_eq!(count(&mut e, 1), 15);
        assert_eq!(count(&mut e, 0), 16);
        // Only the low 16 bits count.
        assert_eq!(count(&mut e, 0x1_0000), 16);
        assert_eq!(count(&mut e, 0xffff_8000), 0);
    }

    #[test]
    fn entries_are_set_and_vectors_copied() {
        let mut e = Engine::new();
        let entries = e.mem.alloc(0x40);
        let source = e.mem.alloc(0x80);
        let destination = e.mem.alloc(0x80);
        for byte in 0..0x80 {
            e.mem.set_u8(source + byte, byte as u8 + 1);
        }
        e.call(0x00df_32a0, &args![entries, 3u32]);
        assert_eq!(e.mem.u32(entries + 0x18), 0x7fff_ffff);
        assert_eq!(e.mem.u32(entries + 0x1c), 0);
        e.call(
            0x00df_32d0,
            &args![entries, source, destination, 1u32, 0x8000_0002u32],
        );
        assert_eq!(e.mem.u32(entries + 8), 2);
        assert_eq!(e.mem.u32(entries + 0xc), 1);
        assert_eq!(
            e.mem.bytes(destination + 0x20, 0x10),
            e.mem.bytes(source + 0x20, 0x10)
        );
        assert_eq!(e.mem.u32(destination), 0);
        // Three vectors from index 1.
        let result = e.call(
            0x00df_3330,
            &args![entries, source, destination, 0u32, 1u32, 3u32],
        );
        assert_eq!(result.u32(), 4);
        assert_eq!(e.mem.u32(entries), 1);
        assert_eq!(e.mem.u32(entries + 4), 3);
        assert_eq!(
            e.mem.bytes(destination + 0x10, 0x30),
            e.mem.bytes(source + 0x10, 0x30)
        );
        // Count 0 copies nothing and answers -2.
        let result = e.call(
            0x00df_3330,
            &args![entries, source, destination, 0u32, 5u32, 0u32],
        );
        assert_eq!(result.u32(), 0xffff_fffe);
        assert_eq!(e.mem.u32(entries + 4), 0);
    }

    fn builder_engine() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x0101_1000,
            0x0102_4000,
            0x0104_d000,
            0x0105_5000,
            0x010c_d000,
            0x0126_8000,
        ] {
            e.map(page, 0x1000);
        }
        e.mem.set_f32(SCALE_FLOAT, 1.0);
        e.mem.set_f32(COST_FACTOR, 0.125);
        e.mem.set_f64(AREA_STEP, 8.0);
        e.mem.set_f32(WORST_COST, f32::MAX);
        e
    }

    /// An entry of the buffers: the minimum x, y, z at `+0`, `+2`, `+4` and the maximum at
    /// `+8`, `+0xa`, `+0xc` (16-bit words).
    type Box16 = ([u16; 3], [u16; 3]);

    fn put_entry(e: &mut Engine, at: u32, entry: Box16) {
        for k in 0..3u32 {
            e.mem.set_u16(at + 2 * k, entry.0[k as usize]);
            e.mem.set_u16(at + 8 + 2 * k, entry.1[k as usize]);
        }
    }

    fn get_entry(e: &Engine, at: u32) -> Box16 {
        let word = |offset: u32| e.mem.u16(at + offset);
        ([word(0), word(2), word(4)], [word(8), word(0xa), word(0xc)])
    }

    /// The pair of buffers (`first` holding `entries` from index `start`), the split record,
    /// the answer of the buffers and a box that holds them all.
    struct Split {
        tree: u32,
        first: u32,
        second: u32,
        split: u32,
        buffers: u32,
        bounds: u32,
    }

    fn split_setup(e: &mut Engine, start: u32, entries: &[Box16], bounds: Box16) -> Split {
        let size = 0x10 * (start + entries.len() as u32);
        let first = e.mem.alloc(size);
        let second = e.mem.alloc(size);
        let tree = e.mem.alloc(8);
        e.mem.set_u32(tree, first);
        e.mem.set_u32(tree + 4, second);
        for (i, entry) in entries.iter().enumerate() {
            put_entry(e, first + 0x10 * (start + i as u32), *entry);
        }
        let split = e.mem.alloc(0x10);
        let buffers = e.mem.alloc(8);
        let at = e.mem.alloc(0x10);
        put_entry(e, at, bounds);
        Split {
            tree,
            first,
            second,
            split,
            buffers,
            bounds: at,
        }
    }

    fn wide(min_x: u16, max_x: u16) -> Box16 {
        ([min_x, 0, 0], [max_x, 4, 4])
    }

    #[test]
    fn entries_are_split_at_the_mean_of_their_middles() {
        let mut e = builder_engine();
        // Along x (the longest side): the middles 5, 7, 25 and 27 have the mean 16; two
        // minima are at most the pivot and two maxima are at least it: the entries with a
        // maximum below the pivot go first.
        let entries = [wide(0, 10), wide(2, 12), wide(20, 30), wide(22, 32)];
        let s = split_setup(&mut e, 0, &entries, wide(0, 32));
        assert_eq!(
            e.call(
                0x00df_33b0,
                &args![s.tree, 0i32, 4i32, s.split, s.buffers, s.bounds]
            )
            .u32(),
            0
        );
        assert_eq!(e.mem.u32(s.split), 0);
        assert_eq!(e.mem.i32(s.split + 4), 2);
        assert_eq!(e.mem.u16(s.split + 8), 20);
        assert_eq!(e.mem.u16(s.split + 0xa), 12);
        assert_eq!(e.mem.u32(s.buffers), s.second);
        assert_eq!(e.mem.u32(s.buffers + 4), s.first);
        for (i, entry) in entries.iter().enumerate() {
            assert_eq!(get_entry(&e, s.second + 0x10 * i as u32), *entry);
        }
        // The same call through `00df4ec0` (fewer than 8 entries are left to it).
        let counter = e.mem.alloc(4);
        let mask = e.mem.alloc(16);
        let t = split_setup(&mut e, 0, &entries, wide(0, 32));
        assert_eq!(
            e.call(
                0x00df_4ec0,
                &args![t.tree, 0i32, 4i32, t.split, counter, t.buffers, t.bounds, mask]
            )
            .u32(),
            0
        );
        assert_eq!(e.mem.i32(t.split + 4), 2);
        assert_eq!(e.mem.u16(t.split + 8), 20);
        assert_eq!(e.mem.u16(t.split + 0xa), 12);
    }

    #[test]
    fn the_smaller_side_goes_by_the_minimum() {
        let mut e = builder_engine();
        // Three minima are at most the pivot 62 but four maxima are at least it: the entry
        // whose minimum is above the pivot goes second. The entries start at index 2.
        let entries = [wide(0, 100), wide(60, 70), wide(62, 72), wide(64, 74)];
        let s = split_setup(&mut e, 2, &entries, wide(0, 100));
        e.call(
            0x00df_33b0,
            &args![s.tree, 2i32, 6i32, s.split, s.buffers, s.bounds],
        );
        assert_eq!(e.mem.i32(s.split + 4), 5);
        assert_eq!(e.mem.u16(s.split + 8), 64);
        assert_eq!(e.mem.u16(s.split + 0xa), 100);
        for (i, entry) in entries.iter().enumerate() {
            assert_eq!(get_entry(&e, s.second + 0x10 * (2 + i as u32)), *entry);
        }
    }

    #[test]
    fn identical_entries_cannot_be_split() {
        let mut e = builder_engine();
        let entries = [wide(0, 10); 4];
        let s = split_setup(&mut e, 0, &entries, wide(0, 10));
        e.mem.set_u32(s.split + 4, 0x1234);
        assert_eq!(
            e.call(
                0x00df_33b0,
                &args![s.tree, 0i32, 4i32, s.split, s.buffers, s.bounds]
            )
            .u32(),
            1
        );
        assert_eq!(e.mem.u32(s.split + 4), 0x1234);
        assert_eq!(e.mem.u32(s.buffers), 0);
    }

    #[test]
    fn two_entries_are_ordered_by_their_minimum() {
        let mut e = builder_engine();
        let entries = [wide(10, 20), wide(5, 15)];
        let s = split_setup(&mut e, 0, &entries, wide(0, 20));
        let counter = e.mem.alloc(4);
        let mask = e.mem.alloc(16);
        assert_eq!(
            e.call(
                0x00df_4ec0,
                &args![s.tree, 0i32, 2i32, s.split, counter, s.buffers, s.bounds, mask]
            )
            .u32(),
            0
        );
        assert_eq!(get_entry(&e, s.second), entries[1]);
        assert_eq!(get_entry(&e, s.second + 0x10), entries[0]);
        assert_eq!(e.mem.u32(s.split), 0);
        assert_eq!(e.mem.i32(s.split + 4), 1);
        // The minimum of the second group and the maximum of the first.
        assert_eq!(e.mem.u16(s.split + 8), 10);
        assert_eq!(e.mem.u16(s.split + 0xa), 15);
        assert_eq!(e.mem.u32(s.buffers), s.second);
        assert_eq!(e.mem.u32(s.buffers + 4), s.first);
        // Already in order: the first stays first.
        let entries = [wide(5, 15), wide(10, 20)];
        let t = split_setup(&mut e, 0, &entries, wide(0, 20));
        e.call(
            0x00df_4ec0,
            &args![t.tree, 0i32, 2i32, t.split, counter, t.buffers, t.bounds, mask],
        );
        assert_eq!(get_entry(&e, t.second), entries[0]);
        assert_eq!(e.mem.u16(t.split + 8), 10);
        assert_eq!(e.mem.u16(t.split + 0xa), 15);
    }

    /// Eight entries around x = 0 to 15 and eight around 100 to 110, y and z from 0 to 40.
    fn clusters() -> Vec<Box16> {
        let mut entries = Vec::new();
        for i in 0..8u16 {
            entries.push(([i, 0, 0], [8 + i, 40, 40]));
        }
        for i in 0..8u16 {
            entries.push(([100 + i, 0, 0], [103 + i, 40, 40]));
        }
        entries
    }

    #[test]
    fn the_histogram_cuts_between_two_clusters() {
        let mut e = builder_engine();
        let entries = clusters();
        let bounds: Box16 = ([0, 0, 0], [110, 40, 40]);
        let s = split_setup(&mut e, 0, &entries, bounds);
        let counter = e.mem.alloc(4);
        let mask = e.mem.alloc(16);
        assert_eq!(
            e.call(
                0x00df_4ec0,
                &args![s.tree, 0i32, 16i32, s.split, counter, s.buffers, s.bounds, mask]
            )
            .u32(),
            0
        );
        assert_eq!(e.mem.u32(s.split), 0);
        assert_eq!(e.mem.i32(s.split + 4), 8);
        assert_eq!(e.mem.u16(s.split + 8), 100);
        assert_eq!(e.mem.u16(s.split + 0xa), 15);
        assert_eq!(e.mem.u32(s.buffers), s.second);
        assert_eq!(e.mem.u32(s.buffers + 4), s.first);
        // The first cluster in order, the second from the end backwards.
        for i in 0..8u32 {
            assert_eq!(get_entry(&e, s.second + 0x10 * i), entries[i as usize]);
            assert_eq!(
                get_entry(&e, s.second + 0x10 * (15 - i)),
                entries[8 + i as usize]
            );
        }
        // The counter only goes down when the cut is at an end.
        assert_eq!(e.mem.i32(counter), 0);
    }

    /// A node description for `buildTreeRecursive`: the buffer pair, the slot, the entries
    /// `start` to `end`, the box, a word and the mask vector.
    fn range_of(e: &mut Engine, tree: u32, slot: u32, start: u32, end: u32, bounds: Box16) -> u32 {
        let range = e.mem.alloc(0x50);
        e.mem.set_u32(range, tree);
        e.mem.set_u32(range + 4, slot);
        e.mem.set_u32(range + 0xc, start);
        e.mem.set_u32(range + 0x10, end);
        put_entry(e, range + 0x20, bounds);
        range
    }

    struct Build {
        range: u32,
        entries: u32,
        table: u32,
        destination: u32,
        counter: u32,
        total: u32,
        source: Vec<Box16>,
    }

    fn build_setup(e: &mut Engine, source: Vec<Box16>, bounds: Box16) -> Build {
        let tree_state = split_setup(e, 0, &source, bounds);
        let range = range_of(e, tree_state.tree, 0, 0, source.len() as u32, bounds);
        let table = e.mem.alloc(0x200);
        let entries = e.mem.alloc(4);
        e.mem.set_u32(entries, table);
        let destination = e.mem.alloc(0x10 * source.len() as u32 + 0x10);
        let counter = e.mem.alloc(4);
        let total = e.mem.alloc(4);
        Build {
            range,
            entries,
            table,
            destination,
            counter,
            total,
            source,
        }
    }

    fn build(e: &mut Engine, b: &Build) -> i32 {
        e.call(
            0x00df_56a0,
            &args![b.range, b.entries, b.destination, b.counter, b.total],
        )
        .i32()
    }

    #[test]
    fn a_node_of_few_entries_is_a_leaf() {
        let mut e = builder_engine();
        let source: Vec<Box16> = (0..4u16).map(|i| wide(i, i + 10)).collect();
        // No entry: the empty marker.
        let b = build_setup(&mut e, source.clone(), wide(0, 14));
        e.mem.set_u32(b.range + 0xc, 3);
        e.mem.set_u32(b.range + 0x10, 3);
        e.mem.set_u32(b.range + 4, 2);
        assert_eq!(build(&mut e, &b), 1);
        assert_eq!(e.mem.u32(b.table + 0x10), 0x7fff_ffff);
        assert_eq!(e.mem.u32(b.table + 0x14), 0);
        // One entry: copied, with the node {index, 1}.
        e.mem.set_u32(b.range + 0xc, 3);
        e.mem.set_u32(b.range + 0x10, 4);
        e.mem.set_u32(b.range + 4, 1);
        assert_eq!(build(&mut e, &b), 1);
        assert_eq!(e.mem.u32(b.table + 8), 3);
        assert_eq!(e.mem.u32(b.table + 0xc), 1);
        assert_eq!(get_entry(&e, b.destination + 0x30), source[3]);
        // Up to four: the node {index, count}, the entries copied and 2 * count - 2 added
        // to the total.
        e.mem.set_u32(b.range + 0xc, 0);
        e.mem.set_u32(b.range + 0x10, 4);
        e.mem.set_u32(b.range + 4, 0);
        assert_eq!(build(&mut e, &b), 1);
        assert_eq!(e.mem.u32(b.table), 0);
        assert_eq!(e.mem.u32(b.table + 4), 4);
        assert_eq!(e.mem.u32(b.total), 6);
        for (i, entry) in source.iter().enumerate() {
            assert_eq!(get_entry(&e, b.destination + 0x10 * i as u32), *entry);
        }
    }

    /// The leaves reachable from `slot`: (index, count) pairs.
    fn leaves(e: &Engine, table: u32, slot: u32, out: &mut Vec<(u32, u32)>) {
        let head = e.mem.u32(table + 8 * slot);
        let tail = e.mem.u32(table + 8 * slot + 4);
        if head & 0x8000_0000 != 0 {
            let child = slot + ((head & 0x7fff_ffff) >> 3);
            leaves(e, table, child, out);
            leaves(e, table, child + 1, out);
        } else if head != 0x7fff_ffff {
            out.push((head, tail));
        }
    }

    #[test]
    fn the_tree_of_two_clusters_holds_every_entry_once() {
        let mut e = builder_engine();
        let source = clusters();
        let b = build_setup(&mut e, source.clone(), ([0, 0, 0], [110, 40, 40]));
        let depth = build(&mut e, &b);
        // The root cuts at x between the clusters: its payload is the right group's lowest
        // minimum (100) and the left group's highest maximum (15), its first child slot 1
        // (the children are 8 nodes after the root's slot: `(1 - 0) * 8`), the axis 0.
        assert_eq!(e.mem.u32(b.table), 0x8000_0008);
        assert_eq!(e.mem.u32(b.table + 4), (15 << 16) | 100);
        assert!(depth >= 2, "{depth}");
        assert!(e.mem.i32(b.counter) >= 2);
        let mut found = Vec::new();
        leaves(&e, b.table, 0, &mut found);
        found.sort_unstable();
        // The leaves cover the indices 0 to 16 without a gap, and their entries in the
        // destination are the sixteen entries in some order.
        let mut next = 0;
        for (index, count) in &found {
            assert_eq!(*index, next);
            next += count;
        }
        assert_eq!(next, 16);
        let mut placed: Vec<Box16> = (0..16u32)
            .map(|i| get_entry(&e, b.destination + 0x10 * i))
            .collect();
        let mut wanted = b.source.clone();
        placed.sort_unstable();
        wanted.sort_unstable();
        assert_eq!(placed, wanted);
        // The leaves of 2 or more entries added `2 * count - 2` each to the total.
        let expected: u32 = found.iter().map(|(_, count)| 2 * count - 2).sum();
        assert_eq!(e.mem.u32(b.total), expected);
    }

    /// The two word helpers `splitTreeBuilding` calls (`00cfcb80` the smaller, `00d19d70` the
    /// larger of two `unsigned short`s).
    fn word_doubles(e: &mut Engine) {
        e.register(MIN_WORD, |_, a| Ret {
            eax: u32::from((a[0] as u16).min(a[1] as u16)),
            ..Ret::default()
        });
        e.register(MAX_WORD, |_, a| Ret {
            eax: u32::from((a[0] as u16).max(a[1] as u16)),
            ..Ret::default()
        });
    }

    struct Quad {
        tree: Split,
        nodes: u32,
        ranges: u32,
        counts: u32,
    }

    fn quad_setup(e: &mut Engine, entries: &[Box16], bounds: Box16) -> Quad {
        let tree = split_setup(e, 0, entries, bounds);
        let nodes = e.mem.alloc(0x20);
        let ranges = e.mem.alloc(0x140);
        for group in 0..4u32 {
            let pair = e.mem.alloc(8);
            e.mem.set_u32(ranges + 0x50 * group, pair);
        }
        let counts = e.mem.alloc(0x10);
        Quad {
            tree,
            nodes,
            ranges,
            counts,
        }
    }

    fn split_four(e: &mut Engine, q: &Quad, capacity: i32, count: i32) {
        e.call(
            0x00df_37b0,
            &args![
                q.tree.tree,
                q.nodes,
                capacity,
                count,
                q.ranges,
                q.counts,
                q.tree.bounds
            ],
        );
    }

    /// The entries of four clusters at the bins 4, 20, 36 and 52 of the x axis (`min = bin *
    /// 1024 + 100 + 20 * i`, `max = bin * 1024 + 300 + 20 * i`, y and z from 0 to 10).
    fn cluster_entry(cluster: usize, i: u16) -> Box16 {
        let base = [4u16, 20, 36, 52][cluster] * 1024;
        ([base + 100 + 20 * i, 0, 0], [base + 300 + 20 * i, 10, 10])
    }

    const CLUSTER_BOUNDS: Box16 = ([0, 0, 0], [60000, 10, 10]);

    /// The facts the four-cluster tests check, for groups of `per` entries.
    fn check_groups(e: &Engine, q: &Quad, capacity: u32, per: u32, expected: &[Box16]) {
        let second = q.tree.second;
        for (i, entry) in expected.iter().enumerate() {
            assert_eq!(get_entry(e, second + 0x10 * i as u32), *entry, "entry {i}");
        }
        let minimum = |group: usize| u32::from(cluster_entry(group, 0).0[0]);
        let maximum = |group: usize| u32::from(cluster_entry(group, per as u16 - 1).1[0]);
        assert_eq!(
            e.mem.u32(q.nodes + 4),
            (maximum(1) << 16) | minimum(2),
            "node 0"
        );
        assert_eq!(e.mem.u32(q.nodes), 0x8000_0010);
        assert_eq!(e.mem.u32(q.nodes + 0x10), 0x8000_0010);
        assert_eq!(e.mem.u32(q.nodes + 0x14), (maximum(0) << 16) | minimum(1));
        assert_eq!(e.mem.u32(q.nodes + 0x18), 0x8000_0018);
        assert_eq!(e.mem.u32(q.nodes + 0x1c), (maximum(2) << 16) | minimum(3));
        for group in 0..4usize {
            let range = q.ranges + 0x50 * group as u32;
            let pair = e.mem.u32(range);
            // The pair of buffers is swapped for the children.
            assert_eq!(e.mem.u32(pair), q.tree.second);
            assert_eq!(e.mem.u32(pair + 4), q.tree.first);
            assert_eq!(e.mem.u32(range + 4), 0);
            assert_eq!(e.mem.u32(range + 8), capacity);
            assert_eq!(e.mem.u32(range + 0xc), per * group as u32);
            assert_eq!(e.mem.u32(range + 0x10), per * (group as u32 + 1));
            // The box is the parent's with the x range of the group.
            assert_eq!(
                get_entry(e, range + 0x20),
                (
                    [minimum(group) as u16, 0, 0],
                    [maximum(group) as u16, 10, 10]
                )
            );
            assert_eq!(e.mem.u32(q.counts + 4 * group as u32), 2 * per);
        }
    }

    #[test]
    fn four_groups_follow_the_histogram_cuts() {
        let mut e = builder_engine();
        word_doubles(&mut e);
        // Two entries in each cluster, in the order 3, 0, 2, 1 twice.
        let mut entries = Vec::new();
        for pass in 0..2u16 {
            for cluster in [3usize, 0, 2, 1] {
                entries.push(cluster_entry(cluster, pass));
            }
        }
        let q = quad_setup(&mut e, &entries, CLUSTER_BOUNDS);
        split_four(&mut e, &q, 100, 8);
        let mut expected = Vec::new();
        for cluster in 0..4usize {
            expected.push(cluster_entry(cluster, 0));
            expected.push(cluster_entry(cluster, 1));
        }
        check_groups(&e, &q, 100, 2, &expected);
        // The first buffer is still the source: nothing was written to it.
        assert_eq!(get_entry(&e, q.tree.first), entries[0]);
    }

    #[test]
    fn every_second_entry_is_sampled_above_ten() {
        let mut e = builder_engine();
        word_doubles(&mut e);
        // Six entries in each cluster, a cluster after another: the histograms see every second
        // one (three each), the groups still hold all six.
        let mut entries = Vec::new();
        for cluster in 0..4usize {
            for i in 0..6u16 {
                entries.push(cluster_entry(cluster, i));
            }
        }
        let q = quad_setup(&mut e, &entries, CLUSTER_BOUNDS);
        split_four(&mut e, &q, 100, 24);
        check_groups(&e, &q, 100, 6, &entries);
    }

    /// Eight equal entries cannot be cut by the histograms: the balanced fallback cuts them
    /// in quarters of two.
    fn equal_entries() -> (Vec<Box16>, Box16) {
        (
            vec![([100, 0, 0], [300, 10, 10]); 8],
            ([0, 0, 0], [1000, 10, 10]),
        )
    }

    #[test]
    fn entries_that_cannot_be_cut_go_in_quarters() {
        let mut e = builder_engine();
        word_doubles(&mut e);
        let (entries, bounds) = equal_entries();
        let q = quad_setup(&mut e, &entries, bounds);
        split_four(&mut e, &q, 100, 8);
        for i in 0..8u32 {
            assert_eq!(get_entry(&e, q.tree.second + 0x10 * i), entries[i as usize]);
        }
        // Every group ends with the equal entries' range: (300 << 16) | 100 for the node 0.
        assert_eq!(e.mem.u32(q.nodes + 4), (300 << 16) | 100);
        assert_eq!(e.mem.u32(q.nodes + 0x14), (300 << 16) | 100);
        assert_eq!(e.mem.u32(q.nodes + 0x1c), (300 << 16) | 100);
        for group in 0..4u32 {
            let range = q.ranges + 0x50 * group;
            assert_eq!(e.mem.u32(range + 0xc), 2 * group);
            assert_eq!(e.mem.u32(range + 0x10), 2 * group + 2);
            assert_eq!(get_entry(&e, range + 0x20), ([100, 0, 0], [300, 10, 10]));
            assert_eq!(e.mem.u32(q.counts + 4 * group), 4);
        }
    }

    #[test]
    fn the_fallback_reports_too_little_room() {
        use std::cell::RefCell;
        use std::rc::Rc;
        let mut e = builder_engine();
        word_doubles(&mut e);
        let log: Rc<RefCell<Vec<(u32, Vec<u32>)>>> = Rc::default();
        for address in [OSTREAM_CONSTRUCT, OSTREAM_WRITE_TEXT, OSTREAM_DESTRUCT] {
            let sink = Rc::clone(&log);
            e.register_double(address, move |_, a| {
                sink.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
        e.put_vtable(0x4400_9000, &[0, 0, 0, 0x4400_9001]);
        let sink = Rc::clone(&log);
        e.register_double(0x4400_9001, move |_, a| {
            sink.borrow_mut().push((0x4400_9001, a.to_vec()));
            Ret::default()
        });
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, 0x4400_9000);
        e.mem.set_u32(REPORT_SINK, object);
        let (entries, bounds) = equal_entries();
        // A quarter is 2 entries: the room must be at least 2 * 8 + 8 = 24.
        let q = quad_setup(&mut e, &entries, bounds);
        split_four(&mut e, &q, 24, 8);
        assert!(log.borrow().is_empty());
        let q = quad_setup(&mut e, &entries, bounds);
        split_four(&mut e, &q, 23, 8);
        let calls = log.borrow().clone();
        let addresses: Vec<u32> = calls.iter().map(|c| c.0).collect();
        assert_eq!(
            addresses,
            vec![
                OSTREAM_CONSTRUCT,
                OSTREAM_WRITE_TEXT,
                0x4400_9001,
                OSTREAM_DESTRUCT
            ]
        );
        let stream = calls[0].1[0];
        let buffer = calls[0].1[1];
        assert_eq!(calls[0].1[2], 0x200);
        assert_eq!(calls[1].1, vec![stream, 0x010d_a240]);
        assert_eq!(
            calls[2].1,
            vec![object, 3, 0x5010_d60c, buffer, 0x010d_a21c, 0x198]
        );
        assert_eq!(calls[3].1, vec![stream]);
        // The cut is made all the same.
        assert_eq!(e.mem.u32(q.nodes + 4), (300 << 16) | 100);
    }
}

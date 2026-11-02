//! `Havok SDK/run_00e4f910` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::havok_sdk::run_00d6bf60::{
    addps, addss, andnps, andps, cmpeqss, cmpleps, divss, fcom_status, load_vec, maxps, minps,
    movmskps, mulps, mulss, rsqrtss, shufps, store_vec, subps, subss, Vec4,
};

/// `00c85750` (hkMemoryRouter lookup): no arguments, the router in `EAX`. The router's first word
/// is a stack pointer the functions of this unit take scratch memory from (and put back), and its
/// `+0x10` is the heap allocator object.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkArrayUtil::_reserve` (Xbox PDB, `00c908c0`): cdecl, `(allocator, array, count, element size)`.
const RESERVE: u32 = 0x00c9_08c0;
/// `00e4ead0`: cdecl, `(u16 array, offset, stats)`; `00e4ebb0`: cdecl, `(u16 array, second array,
/// offset, count, stats)`; `00e4e660`: thiscall on the statistics block, no stack words (its
/// report; the exe's name for it is `hkaTrackAnalysis::AnalysisStats::displayReport`).
const ANALYSE_FLAGS: u32 = 0x00e4_ead0;
const ANALYSE_RANGES: u32 = 0x00e4_ebb0;
const DISPLAY_REPORT: u32 = 0x00e4_e660;
/// `_ftol2`-style helper (`00ec62c0`): the `double` in `ST0` as a leading argument, the truncated
/// integer in `EAX`.
const FLOAT_TO_INT: u32 = 0x00ec_62c0;
/// The `float`s the box minimum and maximum start from (`0104da48`, `0104e148`).
const MINIMUM_START: u32 = 0x0104_da48;
const MAXIMUM_START: u32 = 0x0104_e148;
/// The words 0x80000000 and flags: the "not owned" bit of an `hkArray`'s capacity word.
const NOT_OWNED: u32 = 0x8000_0000;

/// The heap allocator object of the memory router (`00c85750`, then `+0x10`).
fn heap_allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &[]).u32();
    e.mem.u32(router.wrapping_add(0x10))
}

/// Frees the block of the `hkArray` at `array` (`data`, `size`, `capacity | flags`) and empties
/// it, as `00e50ca0` does for each of its arrays: the router is asked first, the size is set to 0,
/// the block is freed through the allocator (virtual slot `0x10`, with the pointer and
/// `element_size * capacity`) unless the array is unowned (bit 31 of the capacity word), the data
/// pointer is cleared and the capacity word set to `0x80000000`.
fn release_array(e: &mut Engine, array: u32, element_size: u32) {
    let allocator = heap_allocator(e);
    let flags = e.mem.u32(array.wrapping_add(8));
    e.mem.set_u32(array.wrapping_add(4), 0);
    if flags & NOT_OWNED == 0 {
        let data = e.mem.u32(array);
        e.vcall(
            allocator,
            0x10,
            &args![data, (flags & 0x3fff_ffff).wrapping_mul(element_size)],
        );
    }
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array.wrapping_add(8), NOT_OWNED);
}

/// Makes room for `count` elements of `element_size` bytes in the `hkArray` at `array` the way the
/// functions of this unit do for their stack arrays: the router is asked, and when the capacity
/// bits are below `count` the array is reserved (`00c908c0`) for the larger of twice the capacity
/// and `count`; then the size is stored.
fn reserve_array(e: &mut Engine, array: u32, count: i32, element_size: u32) {
    let allocator = heap_allocator(e);
    let capacity = (e.mem.u32(array.wrapping_add(8)) & 0x3fff_ffff) as i32;
    if capacity < count {
        let doubled = capacity.wrapping_add(capacity);
        let wanted = if count < doubled { doubled } else { count };
        e.call(RESERVE, &args![allocator, array, wanted, element_size]);
    }
    e.mem.set_i32(array.wrapping_add(4), count);
}

/// An empty unowned `hkArray`: `data` 0, `size` 0, `capacity | flags` `0x80000000`.
fn empty_array(e: &mut Engine, array: u32) {
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array.wrapping_add(4), 0);
    e.mem.set_u32(array.wrapping_add(8), NOT_OWNED);
}

/// Takes `count * 4` bytes (rounded up to 0x80) off the router's scratch stack (its first word) and
/// builds an `hkArray` on them at `array` (`data` the scratch block, `size` 0, capacity `count`
/// with the unowned flag). Returns the scratch block's start, which is also what the router's
/// stack pointer must be set back to.
fn scratch_array(e: &mut Engine, array: u32, count: u32) -> u32 {
    empty_array(e, array);
    let router = e.call(MEMORY_ROUTER, &[]).u32();
    let block = e.mem.u32(router);
    let rounded = (count.wrapping_mul(4).wrapping_add(0x7f)) & 0xffff_ff80;
    e.mem.set_u32(router, rounded.wrapping_add(block));
    e.mem.set_u32(array.wrapping_add(8), count | NOT_OWNED);
    e.mem.set_u32(array, block);
    block
}

// Translated from 00e50ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fastcall: empties the five `hkArray`s of `this` (the byte array at `+0x3c` and the
/// word arrays at `+0x30`, `+0x24`, `+0x18` and `+0xc`, in that order) with the allocator of the
/// memory router: for each, see `release_array` (a block the array owns is freed through virtual
/// slot `0x10`).
pub fn fn_00e50ca0(e: &mut Engine, this: Ptr) {
    let this = this.addr();
    release_array(e, this.wrapping_add(0x3c), 1);
    for offset in [0x30u32, 0x24, 0x18, 0xc] {
        release_array(e, this.wrapping_add(offset), 4);
    }
}

// Translated from 00e50d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cdecl, `(factor, source, target, count)`, a lifting step of a wavelet transform: for
/// `k = 0 .. count - 2`, `target[k] += (source[k] + source[k + 1]) * factor`, and finally
/// `target[count - 1] += source[count - 1] * (factor + factor)`, all on the x87 stack (the exe
/// runs the first `4 * ((count - 5) / 4 + 1)` elements four at a time; the values are the same). The
/// operand that is loaded with `FLD` and the one added from memory follow the exe (`FLD
/// source[k]`, `FADD source[k + 1]` for the first and third element of each group of four,
/// the other way round for the second and fourth and in the remainder loop), which matters only
/// for NaN operands.
pub fn fn_00e50d90(e: &mut Engine, factor: f32, source: u32, target: u32, count: i32) {
    let factor = x87_load32(factor.to_bits());
    let last = count.wrapping_sub(1);
    let element = |e: &Engine, base: u32, index: i32| {
        e.mem.u32(base.wrapping_add((index as u32).wrapping_mul(4)))
    };
    let mut done = 0i32;
    if last >= 4 {
        done = ((((last - 4) as u32) >> 2) + 1) as i32 * 4;
    }
    for k in 0..done.min(last.max(0)) {
        // Groups of four: elements 0 and 2 load `source[k]` and add `source[k + 1]`, 1 and 3 load
        // `source[k + 1]` and add `source[k]`.
        let (loaded, added) = if k % 2 == 0 { (k, k + 1) } else { (k + 1, k) };
        let sum = x87_add(
            x87_load32(element(e, source, loaded)),
            x87_m32(element(e, source, added)),
        );
        let scaled = x87_mul(sum, factor);
        let total = x87_add(scaled, x87_m32(element(e, target, k)));
        e.mem.set_u32(
            target.wrapping_add((k as u32).wrapping_mul(4)),
            x87_store32(total),
        );
    }
    for k in done..last {
        let sum = x87_add(
            x87_load32(element(e, source, k + 1)),
            x87_m32(element(e, source, k)),
        );
        let scaled = x87_mul(sum, factor);
        let total = x87_add(scaled, x87_m32(element(e, target, k)));
        e.mem.set_u32(
            target.wrapping_add((k as u32).wrapping_mul(4)),
            x87_store32(total),
        );
    }
    let doubled = x87_add(factor, factor);
    let product = x87_mul(x87_load32(element(e, source, last)), doubled);
    let total = x87_add(product, x87_m32(element(e, target, last)));
    e.mem.set_u32(
        target.wrapping_add((last as u32).wrapping_mul(4)),
        x87_store32(total),
    );
}

// Translated from 00e50e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cdecl, `(factor, target, source, count)`, the mirror step of `00e50d90`: for `k = 1 .. count -
/// 1`, `target[k] += (source[k - 1] + source[k]) * factor`, and first
/// `target[0] += source[0] * (factor + factor)`. The exe's groups of four start at element 1
/// (`FLD source[k - 1]` and add `source[k]` for the first two elements of a group, `FLD
/// source[k]` and add `source[k - 1]` for the other two; the remainder loop loads `source[k - 1]`).
pub fn fn_00e50e50(e: &mut Engine, factor: f32, target: u32, source: u32, count: i32) {
    let factor = x87_load32(factor.to_bits());
    let last = count.wrapping_sub(1);
    let element = |e: &Engine, base: u32, index: i32| {
        e.mem.u32(base.wrapping_add((index as u32).wrapping_mul(4)))
    };
    let mut done = 1i32;
    if last >= 4 {
        done = ((((count - 5) as u32) >> 2) + 1) as i32 * 4 + 1;
    }
    for k in 1..done.min(count) {
        // In a group of four starting at 1 the first two elements load the lower neighbour.
        let in_group = (k - 1) % 4;
        let (loaded, added) = if in_group < 2 { (k - 1, k) } else { (k, k - 1) };
        let sum = x87_add(
            x87_load32(element(e, source, loaded)),
            x87_m32(element(e, source, added)),
        );
        let scaled = x87_mul(sum, factor);
        let total = x87_add(scaled, x87_m32(element(e, target, k)));
        e.mem.set_u32(
            target.wrapping_add((k as u32).wrapping_mul(4)),
            x87_store32(total),
        );
    }
    for k in done..count {
        let sum = x87_add(
            x87_load32(element(e, source, k - 1)),
            x87_m32(element(e, source, k)),
        );
        let scaled = x87_mul(sum, factor);
        let total = x87_add(scaled, x87_m32(element(e, target, k)));
        e.mem.set_u32(
            target.wrapping_add((k as u32).wrapping_mul(4)),
            x87_store32(total),
        );
    }
    let doubled = x87_add(factor, factor);
    let product = x87_mul(doubled, x87_m32(element(e, source, 0)));
    let total = x87_add(product, x87_m32(element(e, target, 0)));
    e.mem.set_u32(target, x87_store32(total));
}

// Translated from 00e50f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cdecl, `(array, low, high, extra)`: quicksort of the `float`s `array[low ..= high]` into
/// ascending order, recursing on the lower part and looping on the upper (`extra` is only passed
/// on). The pivot is the middle element, read through the x87 stack; elements are compared with
/// `FCOMP` (a lane below the pivot moves `low` on, a lane above moves `high` back, a NaN stops
/// both) and swapped as `FLD`/`FSTP` copies (a signalling NaN is quieted).
pub fn fn_00e50f10(e: &mut Engine, array: u32, low: i32, high: i32, extra: u32) {
    let element = |index: i32| array.wrapping_add((index as u32).wrapping_mul(4));
    let mut low = low;
    loop {
        let middle = low.wrapping_add(high) >> 1;
        let pivot = x87_load32(x87_quiet_bits(e.mem.u32(element(middle))));
        let mut j = high;
        let mut i = low;
        loop {
            while matches!(
                fcom_status(x87_load32(e.mem.u32(element(i))), pivot) & 0x05,
                0x01 | 0x04
            ) {
                i += 1;
            }
            while fcom_status(x87_load32(e.mem.u32(element(j))), pivot) & 0x41 == 0 {
                j -= 1;
            }
            if j < i {
                break;
            }
            if j != i {
                let held = x87_quiet_bits(e.mem.u32(element(j)));
                let lower = x87_quiet_bits(e.mem.u32(element(i)));
                e.mem.set_u32(element(j), lower);
                e.mem.set_u32(element(i), x87_quiet_bits(held));
            }
            j -= 1;
            i += 1;
            if i > j {
                break;
            }
        }
        if low < j {
            fn_00e50f10(e, array, low, j, extra);
        }
        if i < high {
            low = i;
        } else {
            return;
        }
    }
}

// Translated from 00e51070 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cdecl, `(source, target, count)`: interleaves the two halves of `source`. `count * 4` bytes
/// (rounded up to 0x80) are taken off the router's scratch stack (its first word) and the
/// `count` words of `source` are copied there; then for `i < count / 2` (signed) `target[2 * i]`
/// is the scratch word `i` and `target[2 * i + 1]` the scratch word `count / 2 + i` (each through
/// the x87 stack: a signalling NaN is quieted). The router's stack pointer is put back.
pub fn fn_00e51070(e: &mut Engine, source: u32, target: u32, count: i32) {
    let router = e.call(MEMORY_ROUTER, &[]).u32();
    let block = e.mem.u32(router);
    let rounded = ((count as u32).wrapping_mul(4).wrapping_add(0x7f)) & 0xffff_ff80;
    e.mem.set_u32(router, rounded.wrapping_add(block));
    if count > 0 {
        for word in 0..count as u32 {
            let value = e.mem.u32(source.wrapping_add(4 * word));
            e.mem.set_u32(block.wrapping_add(4 * word), value);
        }
    }
    let half = count >> 1;
    for i in 0..half.max(0) as u32 {
        let low = e.mem.u32(block.wrapping_add(4 * i));
        e.mem
            .set_u32(target.wrapping_add(8 * i), x87_quiet_bits(low));
        let high = e.mem.u32(block.wrapping_add(4 * (half as u32 + i)));
        e.mem
            .set_u32(target.wrapping_add(8 * i + 4), x87_quiet_bits(high));
    }
    let router = e.call(MEMORY_ROUTER, &[]).u32();
    e.mem.set_u32(router, block);
}

/// The static tables `00e51140` fills on first use (each row is 16 bytes): `0126ef00..0126ef80`
/// (flag bit 0 of `0126ef80`) from the rows of the executable's data at the addresses given, and
/// `0126ee80..0126ef00` (flag bit 1).
const FIRST_TABLE: [(u32, u32); 8] = [
    (0x0126_ef00, 0x010e_c7f0),
    (0x0126_ef10, 0x010e_c850),
    (0x0126_ef20, 0x010e_c840),
    (0x0126_ef30, 0x010e_c830),
    (0x0126_ef40, 0x010e_c820),
    (0x0126_ef50, 0x010e_c810),
    (0x0126_ef60, 0x010e_c800),
    (0x0126_ef70, 0x010e_c7b0),
];
const SECOND_TABLE: [(u32, u32); 8] = [
    (0x0126_ee80, 0x010e_c7f0),
    (0x0126_ee90, 0x010e_c7e0),
    (0x0126_eea0, 0x010e_c7d0),
    (0x0126_eeb0, 0x010e_c7c0),
    (0x0126_eec0, 0x010e_c7b0),
    (0x0126_eed0, 0x010e_c7a0),
    (0x0126_eee0, 0x010e_c790),
    (0x0126_eef0, 0x010e_c810),
];
/// The flags word of the two static tables (`0126ef80`).
const TABLE_FLAGS: u32 = 0x0126_ef80;
/// The `float`s the two lifting steps of `00e51140` are given (`0104fd68`, `01016248`).
const FIRST_LIFT_FACTOR: u32 = 0x0104_fd68;
const SECOND_LIFT_FACTOR: u32 = 0x0101_6248;

/// Copies the rows `rows` (static, source) as 16-byte vectors.
fn copy_rows(e: &mut Engine, rows: &[(u32, u32)]) {
    for (target, source) in rows {
        let row = load_vec(e, *source);
        store_vec(e, *target, row);
    }
}

// Translated from 00e51140 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cdecl, `(data, count)` (`count` unsigned): a forward transform of `count` floats in place. On
/// first use the two static tables (rows of 16 bytes at `0126ef00` and `0126ee80`, copied from
/// the executable's rows; bits 0 and 1 of `0126ef80` say which are made) are filled. The first
/// eight floats become two 4-lane sums: lanes of `data[0] + sum over k of data[k + 1] * row
/// k` for the rows at `0126ef10 + 16 * k` (into `data[0..4]`) and at `0126ee90 + 16 * k` (into
/// `data[4..8]`), `k` from 0 to 6, each product and sum a `MULPS` / `ADDPS` in that order (the
/// results go through the x87 stack: a signalling NaN is quieted). Then, while the length `n`
/// (starting at 8) is below `count` (unsigned), `00e50e50` runs with the `float` at `0104fd68`
/// on `data`, `data + 4 * n` and `n`, `00e50d90` with the `float` at `01016248` on `data`,
/// `data + 4 * n` and `n` (source and target as the exe passes them), `n` doubles and
/// `00e51070` interleaves `2 * n` floats of `data`.
pub fn fn_00e51140(e: &mut Engine, data: u32, count: u32) {
    let mut flags = e.mem.u32(TABLE_FLAGS);
    if flags & 1 == 0 {
        copy_rows(e, &FIRST_TABLE[..7]);
        flags |= 1;
        e.mem.set_u32(TABLE_FLAGS, flags);
        copy_rows(e, &FIRST_TABLE[7..]);
    }
    if flags & 2 == 0 {
        copy_rows(e, &SECOND_TABLE[..7]);
        flags |= 2;
        e.mem.set_u32(TABLE_FLAGS, flags);
        copy_rows(e, &SECOND_TABLE[7..]);
    }
    let first = [f32::from_bits(e.mem.u32(data)), 0.0, 0.0, 0.0];
    let broadcast = |v: Vec4| [v[0]; 4];
    let mut sum_first = broadcast(first);
    let mut sum_second = broadcast(first);
    for k in 0..7u32 {
        let value = [
            f32::from_bits(e.mem.u32(data.wrapping_add(4 + 4 * k))),
            0.0,
            0.0,
            0.0,
        ];
        let row_first = load_vec(e, 0x0126_ef10 + 16 * k);
        sum_first = addps(mulps(broadcast(value), row_first), sum_first);
        let row_second = load_vec(e, 0x0126_ee90 + 16 * k);
        sum_second = addps(mulps(broadcast(value), row_second), sum_second);
    }
    for (lane, value) in sum_first.iter().chain(sum_second.iter()).enumerate() {
        e.mem.set_u32(
            data.wrapping_add(4 * lane as u32),
            x87_quiet_bits(value.to_bits()),
        );
    }
    let mut length = 8u32;
    if count > length {
        loop {
            let second_half = data.wrapping_add(length.wrapping_mul(4));
            let factor = x87_quiet(f32::from_bits(e.mem.u32(FIRST_LIFT_FACTOR)));
            fn_00e50e50(e, factor, data, second_half, length as i32);
            let factor = x87_quiet(f32::from_bits(e.mem.u32(SECOND_LIFT_FACTOR)));
            fn_00e50d90(e, factor, data, second_half, length as i32);
            length = length.wrapping_add(length);
            fn_00e51070(e, data, data, length as i32);
            if length >= count {
                break;
            }
        }
    }
}

// Translated from 00e51390 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cdecl, `(data, count, threshold, mode)`, zeroing small values of `count` floats at `data`.
/// With `mode` 0: from the last element down, a value whose magnitude is below the running
/// threshold (`threshold` through the x87 stack) is zeroed and, if it is the largest such value
/// since the last level change, remembered; when the index reaches the level marker (`count / 2`,
/// then half of it each time) the threshold is reduced by the largest zeroed magnitude (as a
/// float) and the largest reset. With a nonzero `mode`: nothing happens when `1 / count` (the
/// count as a float, unsigned) is above `threshold`. Otherwise the magnitudes of the values are
/// copied to scratch memory taken off the router's stack (`count * 4` bytes rounded up to 0x80;
/// an unowned `hkArray` on the stack), the index `trunc(count * threshold - 0.5...)` (the
/// `double` at `010c9300`, through `00ec62c0`) minus 1, clamped to `0 ..= count - 1`, picks the
/// cutoff in the sorted copy (`00e50f10`), and every value of `data` whose magnitude is not above
/// the cutoff is set to 0. The scratch memory is given back (and freed through the allocator
/// when the array grew and owns a block). The exe passes `00e50f10` a stack word it never
/// reads; it is passed as 0 here.
pub fn fn_00e51390(e: &mut Engine, data: u32, count: u32, threshold: f32, mode: u8) {
    if mode == 0 {
        threshold_by_levels(e, data, count, threshold);
        return;
    }
    let float_count = {
        let mut value = f64::from(count as i32);
        if (count as i32) < 0 {
            value = x87_add(value, x87_m32(e.mem.u32(UNSIGNED_BIAS)));
        }
        x87_store32(value)
    };
    let threshold_value = x87_load32(threshold.to_bits());
    let reciprocal = x87_div(1.0, x87_load32(float_count));
    // FCOMPP with `TEST AH, 0x41`: a reciprocal above the threshold ends the function.
    if fcom_status(reciprocal, threshold_value) & 0x41 == 0 {
        return;
    }
    e.with_stack(0x10, |e, array| {
        let array = array.addr();
        let saved = scratch_array(e, array, count);
        let allocator = heap_allocator(e);
        let capacity = (e.mem.u32(array.wrapping_add(8)) & 0x3fff_ffff) as i32;
        if capacity < count as i32 {
            let doubled = capacity.wrapping_add(capacity);
            let wanted = if (count as i32) < doubled {
                doubled
            } else {
                count as i32
            };
            e.call(RESERVE, &args![allocator, array, wanted, 4u32]);
        }
        e.mem.set_u32(array.wrapping_add(4), count);
        for index in 0..count {
            let bits = e.mem.u32(data.wrapping_add(4 * index));
            let magnitude = x87_quiet_bits(bits) & 0x7fff_ffff;
            let buffer = e.mem.u32(array);
            e.mem.set_u32(buffer.wrapping_add(4 * index), magnitude);
        }
        let scaled = x87_mul(x87_load32(float_count), x87_m32(threshold.to_bits()));
        let adjusted = x87_sub(scaled, e.mem.f64(CUTOFF_OFFSET));
        let position = e.call(FLOAT_TO_INT, &args![adjusted]).i32();
        let mut position = position.wrapping_sub(1);
        if position >= count as i32 {
            position = (count as i32).wrapping_sub(1);
        }
        let clamped = if position < 0 { 0 } else { position };
        if (count as i32) > 1 {
            let buffer = e.mem.u32(array);
            fn_00e50f10(e, buffer, 0, (count as i32).wrapping_sub(1), 0);
        }
        let buffer = e.mem.u32(array);
        let cutoff_bits = x87_quiet_bits(
            e.mem
                .u32(buffer.wrapping_add((clamped as u32).wrapping_mul(4))),
        );
        let cutoff = x87_load32(cutoff_bits);
        for index in 0..count {
            let bits = e.mem.u32(data.wrapping_add(4 * index));
            let magnitude = x87_load32(x87_quiet_bits(bits) & 0x7fff_ffff);
            // FCOMP with the cutoff, `TEST AH, 0x41` and `JP`: equal or below only.
            if matches!(fcom_status(magnitude, cutoff) & 0x41, 0x01 | 0x40) {
                e.mem.set_u32(data.wrapping_add(4 * index), 0);
            }
        }
        let buffer = e.mem.u32(array);
        if saved == buffer {
            e.mem.set_u32(array.wrapping_add(4), 0);
        }
        let router = e.call(MEMORY_ROUTER, &[]).u32();
        e.mem.set_u32(router, saved);
        release_scratch(e, array);
    });
}

/// The `float` added to a count that is negative as a signed integer (`01099378`).
const UNSIGNED_BIAS: u32 = 0x0109_9378;
/// The `double` subtracted from `count * threshold` before truncation (`010c9300`).
const CUTOFF_OFFSET: u32 = 0x010c_9300;

/// The end of `00e51390`'s scratch use: the array's size is cleared and its block freed through
/// the allocator when it owns one (the capacity word's sign bit is clear).
fn release_scratch(e: &mut Engine, array: u32) {
    let allocator = heap_allocator(e);
    let flags = e.mem.u32(array.wrapping_add(8));
    e.mem.set_u32(array.wrapping_add(4), 0);
    if (flags as i32) >= 0 {
        let data = e.mem.u32(array);
        e.vcall(
            allocator,
            0x10,
            &args![data, (flags & 0x3fff_ffff).wrapping_mul(4)],
        );
    }
}

/// Mode 0 of `00e51390`.
fn threshold_by_levels(e: &mut Engine, data: u32, count: u32, threshold: f32) {
    let mut limit_bits = x87_quiet_bits(threshold.to_bits());
    let mut level = count >> 1;
    let mut largest_bits = 0u32;
    let mut remaining = count;
    if remaining == 0 {
        return;
    }
    loop {
        let element_bits = e
            .mem
            .u32(data.wrapping_add((remaining - 1).wrapping_mul(4)));
        remaining -= 1;
        let magnitude_bits = x87_quiet_bits(element_bits) & 0x7fff_ffff;
        let magnitude = x87_load32(magnitude_bits);
        let limit = x87_load32(limit_bits);
        // FCOM and `TEST AH, 5` / `JP`: only a magnitude strictly below the limit passes.
        if (fcom_status(magnitude, limit) & 0x05).count_ones() == 1 {
            e.mem
                .set_u32(data.wrapping_add(remaining.wrapping_mul(4)), 0);
            // FCOMP of the remembered maximum with the magnitude: only a larger magnitude is kept.
            if (fcom_status(x87_load32(largest_bits), magnitude) & 0x05).count_ones() == 1 {
                largest_bits = x87_store32(magnitude);
            }
        }
        if remaining == level {
            let reduced = x87_sub(limit, x87_m32(largest_bits));
            level >>= 1;
            limit_bits = x87_store32(reduced);
            largest_bits = 0;
        }
        if remaining == 0 {
            break;
        }
    }
}

/// A float of the image as `MOVSS xmm, [address]` loads it: lane 0, the other lanes zero.
pub(crate) fn scalar_vec(e: &Engine, address: u32) -> Vec4 {
    [f32::from_bits(e.mem.u32(address)), 0.0, 0.0, 0.0]
}

/// The registers of the three big functions of the track analysis (`00e4f910`, `00e59a40` and
/// `00e59700`). Their code keeps several values on the x87 stack across loops that jump between
/// dozens of places and mixes it with SSE, so they are translated instruction by instruction:
/// every register, flag and stack word of the exe is here (the frame is real memory, `esp` the
/// real address), and each instruction of the exe is one statement on this state. Only the flags
/// the functions test are kept (`zf`, `sf`, `of`, `pf`, `cf`).
#[derive(Default)]
pub(crate) struct Cpu {
    pub(crate) eax: u32,
    pub(crate) ebx: u32,
    pub(crate) ecx: u32,
    pub(crate) edx: u32,
    pub(crate) esi: u32,
    pub(crate) edi: u32,
    pub(crate) ebp: u32,
    pub(crate) esp: u32,
    /// `XMM0` to `XMM7`.
    pub(crate) x: [Vec4; 8],
    /// The x87 stack, `ST(0)` last.
    pub(crate) st: Vec<f64>,
    /// The x87 status word (the condition codes `FNSTSW` stores).
    pub(crate) sw: u16,
    pub(crate) zf: bool,
    pub(crate) sf: bool,
    pub(crate) of: bool,
    pub(crate) pf: bool,
    pub(crate) cf: bool,
}

impl Cpu {
    fn set_result(&mut self, result: u32) {
        self.zf = result == 0;
        self.sf = result >> 31 != 0;
        self.pf = (result as u8).count_ones() % 2 == 0;
    }

    fn logic(&mut self, result: u32) -> u32 {
        self.set_result(result);
        self.of = false;
        self.cf = false;
        result
    }

    pub(crate) fn and32(&mut self, a: u32, b: u32) -> u32 {
        self.logic(a & b)
    }

    pub(crate) fn or32(&mut self, a: u32, b: u32) -> u32 {
        self.logic(a | b)
    }

    pub(crate) fn xor32(&mut self, a: u32, b: u32) -> u32 {
        self.logic(a ^ b)
    }

    pub(crate) fn add32(&mut self, a: u32, b: u32) -> u32 {
        let (result, carry) = a.overflowing_add(b);
        self.set_result(result);
        self.of = ((a ^ result) & (b ^ result)) >> 31 != 0;
        self.cf = carry;
        result
    }

    pub(crate) fn sub32(&mut self, a: u32, b: u32) -> u32 {
        let (result, borrow) = a.overflowing_sub(b);
        self.set_result(result);
        self.of = ((a ^ b) & (a ^ result)) >> 31 != 0;
        self.cf = borrow;
        result
    }

    pub(crate) fn cmp32(&mut self, a: u32, b: u32) {
        self.sub32(a, b);
    }

    /// `CMP` of two bytes (the low bytes of the operands).
    pub(crate) fn cmp8(&mut self, a: u32, b: u32) {
        let (a, b) = (a as u8, b as u8);
        let (result, borrow) = a.overflowing_sub(b);
        self.zf = result == 0;
        self.sf = result >> 7 != 0;
        self.pf = result.count_ones() % 2 == 0;
        self.of = ((a ^ b) & (a ^ result)) >> 7 != 0;
        self.cf = borrow;
    }

    pub(crate) fn test32(&mut self, a: u32, b: u32) {
        self.logic(a & b);
    }

    /// `TEST` of two bytes (the low bytes of the operands).
    pub(crate) fn test8(&mut self, a: u32, b: u32) {
        let result = (a & b) as u8;
        self.zf = result == 0;
        self.sf = result >> 7 != 0;
        self.pf = result.count_ones() % 2 == 0;
        self.of = false;
        self.cf = false;
    }

    pub(crate) fn inc32(&mut self, a: u32) -> u32 {
        let result = a.wrapping_add(1);
        self.set_result(result);
        self.of = a == 0x7fff_ffff;
        result
    }

    /// `SHL` by an immediate count (the flags follow the result; an undefined `OF` is cleared).
    pub(crate) fn shl32(&mut self, a: u32, count: u32) -> u32 {
        let count = count & 31;
        if count == 0 {
            return a;
        }
        let result = a << count;
        self.set_result(result);
        self.of = false;
        self.cf = (a >> (32 - count)) & 1 != 0;
        result
    }

    /// `SHR` by an immediate count.
    pub(crate) fn shr32(&mut self, a: u32, count: u32) -> u32 {
        let count = count & 31;
        if count == 0 {
            return a;
        }
        let result = a >> count;
        self.set_result(result);
        self.of = false;
        self.cf = (a >> (count - 1)) & 1 != 0;
        result
    }

    /// Two- and three-operand `IMUL`: the low 32 bits of the product; `CF` and `OF` are set when
    /// the signed product does not fit.
    pub(crate) fn imul32(&mut self, a: u32, b: u32) -> u32 {
        let wide = i64::from(a as i32) * i64::from(b as i32);
        let result = wide as i32;
        self.cf = i64::from(result) != wide;
        self.of = self.cf;
        result as u32
    }

    pub(crate) fn push(&mut self, e: &mut Engine, value: u32) {
        self.esp = self.esp.wrapping_sub(4);
        e.mem.set_u32(self.esp, value);
    }

    pub(crate) fn pop(&mut self, e: &Engine) -> u32 {
        let value = e.mem.u32(self.esp);
        self.esp = self.esp.wrapping_add(4);
        value
    }

    pub(crate) fn st_push(&mut self, value: f64) {
        self.st.push(value);
    }

    pub(crate) fn st_pop(&mut self) {
        self.st
            .pop()
            .expect("the exe never pops an empty x87 stack");
    }

    pub(crate) fn st(&self, i: usize) -> f64 {
        self.st[self.st.len() - 1 - i]
    }

    pub(crate) fn set_st(&mut self, i: usize, value: f64) {
        let top = self.st.len() - 1;
        self.st[top - i] = value;
    }

    pub(crate) fn st_exchange(&mut self, i: usize) {
        let top = self.st.len() - 1;
        self.st.swap(top, top - i);
    }
}

// Translated from 00e50650 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cdecl, `(source, second, third)`: builds a block on the stack from the record `source` and
/// runs `00e4f910` on it with `(second, third, 1)`. The block has `*source` at `+0`, the count
/// `n = source[+4]` at `+4`, `source[+8]` at `+8` and five `hkArray`s: of `float`s at `+0xc`,
/// `+0x18`, `+0x24` and `+0x30` and of bytes at `+0x3c`, each made empty and unowned and then
/// reserved (`00c908c0`) and sized for `n` entries. For each of the `n` entries the floats
/// `source[+0xc]`, `[+0x10]`, `[+0x14]` and `[+0x18]` (through the x87 stack: a signalling NaN is
/// quieted) and the byte `source[+0x1c]` are stored in the five arrays. After the call the arrays
/// are released (`00e50ca0`). The exe's fourth word for `00e4f910` has the low byte 1 and nothing
/// else that is read.
pub fn fn_00e50650(e: &mut Engine, source: u32, second: u32, third: u32) {
    e.with_stack(0x48, |e, block| {
        let block = block.addr();
        let count = e.mem.i32(source.wrapping_add(4));
        let first_word = e.mem.u32(source);
        let third_word = e.mem.u32(source.wrapping_add(8));
        for offset in [0x0cu32, 0x18, 0x24, 0x30, 0x3c] {
            empty_array(e, block.wrapping_add(offset));
        }
        e.mem.set_u32(block, first_word);
        e.mem.set_i32(block.wrapping_add(4), count);
        e.mem.set_u32(block.wrapping_add(8), third_word);
        for (offset, element_size) in [(0x0cu32, 4u32), (0x18, 4), (0x24, 4), (0x30, 4), (0x3c, 1)]
        {
            reserve_array(e, block.wrapping_add(offset), count, element_size);
        }
        for index in 0..count.max(0) as u32 {
            for (array, field) in [(0x0cu32, 0xcu32), (0x18, 0x10), (0x24, 0x14), (0x30, 0x18)] {
                let data = e.mem.u32(block.wrapping_add(array));
                let bits = x87_quiet_bits(e.mem.u32(source.wrapping_add(field)));
                e.mem.set_u32(data.wrapping_add(4 * index), bits);
            }
            let data = e.mem.u32(block.wrapping_add(0x3c));
            let flag = e.mem.u8(source.wrapping_add(0x1c));
            e.mem.set_u8(data.wrapping_add(index), flag);
        }
        fn_00e4f910(e, block, second, third, 1);
        fn_00e50ca0(e, Ptr::new(block));
    });
}

/// The two lifting constants and the half `double` `00e50920` uses on its x87 stack: `01011588`
/// (multiplied with the sum of a column's minimum and maximum) and `01011590` (multiplied with the
/// column's threshold).
const MIDPOINT_FACTOR: u32 = 0x0101_1588;
const THRESHOLD_FACTOR: u32 = 0x0101_1590;

// Translated from 00e50920 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cdecl, `(info, matrix, flags, out, report)`: `00e4f910` runs first with `(info, flags, out,
/// 0)`. Then `00e4ead0` is run with `(flags, info[+4], stats)` on a zeroed statistics block of
/// fourteen words, and `out` is advanced by the sum of its first three words (floats). Two
/// scratch arrays of `matrix[+4]` floats (the column count; taken off the router's scratch stack,
/// `scratch_array`) then hold, per column `c` of the `info[+8]` rows of the matrix
/// (`matrix[0]`, `row * matrix[+4] + c` floats), the minimum and maximum (comparisons strict, on
/// the x87 flags: a NaN never replaces them; they start at the floats `0104da48` and
/// `0104e148`). With `threshold = matrix[+0xc][c]`: when `threshold * 2.0` (the `double` at
/// `01011590`) is above `|max - min|` the `u16` at `flags[info[+4] + c]` is 1 and the midpoint
/// `(min + max) * [01011588]` is stored at `out` (which advances by a `float`), otherwise the `u16`
/// is 0. With `report` nonzero a second statistics block is made by `00e4ebb0` (`flags`, `flags +
/// 2 * info[+4]`, `info[+4]`, `matrix[+4]`) and `00e4e660` runs on it. The scratch memory is given
/// back, the second array first (freed through the allocator when it owns a block).
pub fn fn_00e50920(e: &mut Engine, info: u32, matrix: u32, flags: u32, out: u32, report: u8) {
    fn_00e4f910(e, info, flags, out, 0);
    let mut cursor = e.with_stack(0x38, |e, stats| {
        let stats = stats.addr();
        for word in 0..14u32 {
            e.mem.set_u32(stats.wrapping_add(4 * word), 0);
        }
        let start = e.mem.u32(info.wrapping_add(4));
        e.call(ANALYSE_FLAGS, &args![flags, start, stats]);
        let advance = e
            .mem
            .u32(stats.wrapping_add(8))
            .wrapping_add(e.mem.u32(stats.wrapping_add(4)))
            .wrapping_add(e.mem.u32(stats));
        out.wrapping_add(advance.wrapping_mul(4))
    });
    let columns_for_minimum = e.mem.u32(matrix.wrapping_add(4));
    e.with_stack(0x20, |e, arrays| {
        let minimums = arrays.addr();
        let maximums = minimums.wrapping_add(0x10);
        let saved_minimums = scratch_array(e, minimums, columns_for_minimum);
        let columns = e.mem.u32(matrix.wrapping_add(4));
        reserve_array(e, minimums, columns as i32, 4);
        let columns_for_maximum = e.mem.u32(matrix.wrapping_add(4));
        let saved_maximums = scratch_array(e, maximums, columns_for_maximum);
        let columns = e.mem.u32(matrix.wrapping_add(4));
        reserve_array(e, maximums, columns as i32, 4);
        let midpoint_factor = e.mem.f64(MIDPOINT_FACTOR);
        let threshold_factor = e.mem.f64(THRESHOLD_FACTOR);
        let low_start = x87_quiet_bits(e.mem.u32(MINIMUM_START));
        let high_start = x87_quiet_bits(e.mem.u32(MAXIMUM_START));
        let mut column = 0u32;
        if e.mem.u32(matrix.wrapping_add(4)) != 0 {
            loop {
                let threshold_bits = x87_quiet_bits(
                    e.mem
                        .u32(e.mem.u32(matrix.wrapping_add(0xc)).wrapping_add(4 * column)),
                );
                let low_slot = e.mem.u32(minimums).wrapping_add(4 * column);
                e.mem.set_u32(low_slot, low_start);
                let high_slot = e.mem.u32(maximums).wrapping_add(4 * column);
                e.mem.set_u32(high_slot, high_start);
                let mut row = 0u32;
                if e.mem.u32(info.wrapping_add(8)) != 0 {
                    loop {
                        let stride = e.mem.u32(matrix.wrapping_add(4));
                        let cell = e.mem.u32(matrix).wrapping_add(
                            stride
                                .wrapping_mul(row)
                                .wrapping_add(column)
                                .wrapping_mul(4),
                        );
                        let value = x87_load32(x87_quiet_bits(e.mem.u32(cell)));
                        let low_slot = e.mem.u32(minimums).wrapping_add(4 * column);
                        let low = x87_load32(e.mem.u32(low_slot));
                        // FCOMP and `TEST AH, 0x41`: a smaller value replaces the minimum.
                        if fcom_status(low, value) & 0x41 == 0 {
                            e.mem.set_u32(low_slot, x87_store32(value));
                        }
                        let high_slot = e.mem.u32(maximums).wrapping_add(4 * column);
                        let high = x87_load32(e.mem.u32(high_slot));
                        // FCOMP, `TEST AH, 5` and `JP`: a larger value replaces the maximum.
                        if (fcom_status(high, value) & 0x05).count_ones() == 1 {
                            e.mem.set_u32(high_slot, x87_store32(value));
                        }
                        row += 1;
                        if row >= e.mem.u32(info.wrapping_add(8)) {
                            break;
                        }
                    }
                }
                let low_slot = e.mem.u32(minimums).wrapping_add(4 * column);
                let high_slot = e.mem.u32(maximums).wrapping_add(4 * column);
                let sum = x87_add(
                    x87_load32(e.mem.u32(low_slot)),
                    x87_m32(e.mem.u32(high_slot)),
                );
                let midpoint_bits = x87_store32(x87_mul(sum, midpoint_factor));
                let difference = x87_sub(
                    x87_load32(e.mem.u32(high_slot)),
                    x87_m32(e.mem.u32(low_slot)),
                );
                let magnitude = x87_load32(x87_quiet_bits(x87_store32(difference)) & 0x7fff_ffff);
                let limit = x87_mul(x87_load32(threshold_bits), threshold_factor);
                let limit = x87_load32(x87_store32(limit));
                let index = e.mem.u32(info.wrapping_add(4)).wrapping_add(column);
                // FCOMPP of the limit with the magnitude, `TEST AH, 0x41`: only a limit above it.
                if fcom_status(limit, magnitude) & 0x41 == 0 {
                    e.mem.set_u16(flags.wrapping_add(index.wrapping_mul(2)), 1);
                    e.mem.set_u32(cursor, x87_quiet_bits(midpoint_bits));
                    cursor = cursor.wrapping_add(4);
                } else {
                    e.mem.set_u16(flags.wrapping_add(index.wrapping_mul(2)), 0);
                }
                column += 1;
                if column >= e.mem.u32(matrix.wrapping_add(4)) {
                    break;
                }
            }
        }
        if report != 0 {
            let columns = e.mem.u32(matrix.wrapping_add(4));
            let start = e.mem.u32(info.wrapping_add(4));
            e.with_stack(0x38, |e, stats| {
                let stats = stats.addr();
                for word in 0..14u32 {
                    e.mem.set_u32(stats.wrapping_add(4 * word), 0);
                }
                e.call(
                    ANALYSE_RANGES,
                    &args![
                        flags,
                        flags.wrapping_add(start.wrapping_mul(2)),
                        start,
                        columns,
                        stats
                    ],
                );
                e.call(DISPLAY_REPORT, &args![stats]);
            });
        }
        if saved_maximums == e.mem.u32(maximums) {
            e.mem.set_u32(maximums.wrapping_add(4), 0);
        }
        let router = e.call(MEMORY_ROUTER, &[]).u32();
        e.mem.set_u32(router, saved_maximums);
        release_scratch(e, maximums);
        e.mem.set_u32(maximums, 0);
        e.mem.set_u32(maximums.wrapping_add(8), NOT_OWNED);
        if saved_minimums == e.mem.u32(minimums) {
            e.mem.set_u32(minimums.wrapping_add(4), 0);
        }
        let router = e.call(MEMORY_ROUTER, &[]).u32();
        e.mem.set_u32(router, saved_minimums);
        release_scratch(e, minimums);
    });
}

// Translated from 00e4f910 (decompiled, FalloutNV.exe 1.4.0.525)
/// The analysis of a table of `n` tracks of `m` samples (no PDB name for the function; Ghidra
/// names the report it ends with `hkaTrackAnalysis::AnalysisStats::displayReport`). Cdecl
/// `(block, flags, out, report)`, `report` a byte. `block` has the base pointer of the sample
/// records at `+0`, `n` at `+4`, `m` at `+8` and `hkArray`s at `+0xc`, `+0x18`, `+0x24` and
/// `+0x30` (one `float` per track each: tolerances the compares use) and `+0x3c` (one byte per
/// track). A record is 0x30 bytes, three vectors of four `float`s (the first and third are
/// summed, the second is summed with the sign that keeps its dot product with the running sum
/// non-negative), and sample `s` of track `t` is at `base + (n * s + t) * 0x30`. A scratch table
/// of `n` records is taken off the memory router's stack (`00c85750`, rounded up to 0x80 bytes)
/// and made an `hkArray` of 0x30-byte elements with room for `n` (the reserve `00c908c0` makes
/// through the router's allocator is only asked for when the capacity bits are below `n`, which
/// they are not for any `n` below 0x40000000; the exe's code for it is kept).
/// The minimum and maximum of every lane of the first vectors over all tracks and samples
/// (`MINPS` and `MAXPS`, started at the floats `0104da48` and `0104e148`) give the extent. Each track's samples are averaged into its record of the table (the second vector
/// normalised with `RSQRTSS`, a host estimate; the fourth lane of its samples is also ranged when
/// the track's byte is nonzero) and compared with every sample lane by lane
/// (`ABS(a - b) <= tolerance`, `CMPLEPS` and `MOVMSKPS`); the results build the `u16` at
/// `flags + 2 * track` (the byte table `011be718` maps a lane mask to a code). The averaged lanes
/// whose bits in that word are clear are then written to `out`, a `float` stream that advances.
/// With `report` nonzero a block of fourteen zero words is filled by `00e4ead0` from
/// `(flags, n)` and reported by `00e4e660`. The scratch memory is given back (the allocator
/// frees the table when it owns one). The function is the exe's code instruction by instruction
/// on the [`Cpu`] state (the exe keeps values on the x87 stack through loops that jump between
/// dozens of places, mixed with SSE).
pub fn fn_00e4f910(e: &mut Engine, block: u32, flags: u32, out: u32, report: u8) {
    run_00e4f910(e, [block, flags, out, u32::from(report)], 0);
}

/// `00e4f910` on a frame whose words start as `fill` (the exe's frame has whatever was on the
/// stack: the analysis must not depend on it).
fn run_00e4f910(e: &mut Engine, args: [u32; 4], fill: u32) {
    e.with_stack(0x400, |e, frame| {
        let top = frame.addr() + 0x3c0;
        for word in 0..0x100 {
            e.mem.set_u32(frame.addr() + 4 * word, fill);
        }
        let mut cpu = Cpu {
            esp: top,
            ..Cpu::default()
        };
        e.mem.set_u32(top, 0);
        for (i, arg) in args.iter().enumerate() {
            e.mem.set_u32(top + 4 + 4 * i as u32, *arg);
        }
        let mut pc: u32 = 0x00e4_f910;
        'run: loop {
            match pc {
                0x00e4f910 => {
                    // 00e4f910: PUSH EBP
                    cpu.push(e, cpu.ebp);
                    // 00e4f911: MOV EBP,ESP
                    cpu.ebp = cpu.esp;
                    // 00e4f913: AND ESP,0xfffffff0
                    cpu.esp = cpu.and32(cpu.esp, 0xfffffff0);
                    // 00e4f916: SUB ESP,0x124
                    cpu.esp = cpu.sub32(cpu.esp, 0x124);
                    // 00e4f91c: PUSH EBX
                    cpu.push(e, cpu.ebx);
                    // 00e4f91d: MOV EBX,dword ptr [EBP + 0x8]
                    cpu.ebx = e.mem.u32(cpu.ebp.wrapping_add(0x8));
                    // 00e4f920: PUSH ESI
                    cpu.push(e, cpu.esi);
                    // 00e4f921: MOV ESI,dword ptr [EBX + 0x4]
                    cpu.esi = e.mem.u32(cpu.ebx.wrapping_add(0x4));
                    // 00e4f924: PUSH EDI
                    cpu.push(e, cpu.edi);
                    // 00e4f925: XOR EDI,EDI
                    cpu.edi = cpu.xor32(cpu.edi, cpu.edi);
                    // 00e4f927: MOV dword ptr [ESP + 0x54],EDI
                    e.mem.set_u32(cpu.esp.wrapping_add(0x54), cpu.edi);
                    // 00e4f92b: MOV dword ptr [ESP + 0x58],EDI
                    e.mem.set_u32(cpu.esp.wrapping_add(0x58), cpu.edi);
                    // 00e4f92f: MOV dword ptr [ESP + 0x5c],0x80000000
                    e.mem.set_u32(cpu.esp.wrapping_add(0x5c), 0x80000000);
                    // 00e4f937: CALL 0x00c85750
                    cpu.eax = e.call(MEMORY_ROUTER, &args![]).u32();
                    // 00e4f93c: MOV ECX,dword ptr [EAX]
                    cpu.ecx = e.mem.u32(cpu.eax);
                    // 00e4f93e: LEA EDX,[ESI + ESI*0x2]
                    cpu.edx = cpu.esi.wrapping_add(cpu.esi.wrapping_mul(2));
                    // 00e4f941: SHL EDX,0x4
                    cpu.edx = cpu.shl32(cpu.edx, 0x4);
                    // 00e4f944: ADD EDX,0x7f
                    cpu.edx = cpu.add32(cpu.edx, 0x7f);
                    // 00e4f947: AND EDX,0xffffff80
                    cpu.edx = cpu.and32(cpu.edx, 0xffffff80);
                    // 00e4f94a: ADD EDX,ECX
                    cpu.edx = cpu.add32(cpu.edx, cpu.ecx);
                    // 00e4f94c: MOV dword ptr [EAX],EDX
                    e.mem.set_u32(cpu.eax, cpu.edx);
                    // 00e4f94e: OR ESI,0x80000000
                    cpu.esi = cpu.or32(cpu.esi, 0x80000000);
                    // 00e4f954: MOV dword ptr [ESP + 0x5c],ESI
                    e.mem.set_u32(cpu.esp.wrapping_add(0x5c), cpu.esi);
                    // 00e4f958: MOV ESI,dword ptr [EBX + 0x4]
                    cpu.esi = e.mem.u32(cpu.ebx.wrapping_add(0x4));
                    // 00e4f95b: MOV dword ptr [ESP + 0x54],ECX
                    e.mem.set_u32(cpu.esp.wrapping_add(0x54), cpu.ecx);
                    // 00e4f95f: MOV dword ptr [ESP + 0x60],ECX
                    e.mem.set_u32(cpu.esp.wrapping_add(0x60), cpu.ecx);
                    // 00e4f963: CALL 0x00c85750
                    cpu.eax = e.call(MEMORY_ROUTER, &args![]).u32();
                    // 00e4f968: MOV ECX,dword ptr [EAX + 0x10]
                    cpu.ecx = e.mem.u32(cpu.eax.wrapping_add(0x10));
                    // 00e4f96b: MOV EAX,dword ptr [ESP + 0x5c]
                    cpu.eax = e.mem.u32(cpu.esp.wrapping_add(0x5c));
                    // 00e4f96f: AND EAX,0x3fffffff
                    cpu.eax = cpu.and32(cpu.eax, 0x3fffffff);
                    // 00e4f974: CMP EAX,ESI
                    cpu.cmp32(cpu.eax, cpu.esi);
                    // 00e4f976: JGE 0x00e4f991
                    if cpu.sf == cpu.of {
                        pc = 0x00e4f991;
                        continue 'run;
                    }
                    // 00e4f978: ADD EAX,EAX
                    cpu.eax = cpu.add32(cpu.eax, cpu.eax);
                    // 00e4f97a: CMP ESI,EAX
                    cpu.cmp32(cpu.esi, cpu.eax);
                    // 00e4f97c: JL 0x00e4f980
                    if cpu.sf != cpu.of {
                        pc = 0x00e4f980;
                        continue 'run;
                    }
                    // 00e4f97e: MOV EAX,ESI
                    cpu.eax = cpu.esi;
                    pc = 0x00e4f980;
                    continue 'run;
                }
                0x00e4f980 => {
                    // 00e4f980: PUSH 0x30
                    cpu.push(e, 0x30);
                    // 00e4f982: PUSH EAX
                    cpu.push(e, cpu.eax);
                    // 00e4f983: LEA EAX,[ESP + 0x5c]
                    cpu.eax = cpu.esp.wrapping_add(0x5c);
                    // 00e4f987: PUSH EAX
                    cpu.push(e, cpu.eax);
                    // 00e4f988: PUSH ECX
                    cpu.push(e, cpu.ecx);
                    // 00e4f989: CALL 0x00c908c0
                    e.call(
                        RESERVE,
                        &args![
                            e.mem.u32(cpu.esp.wrapping_add(0)),
                            e.mem.u32(cpu.esp.wrapping_add(4)),
                            e.mem.u32(cpu.esp.wrapping_add(8)),
                            e.mem.u32(cpu.esp.wrapping_add(12))
                        ],
                    );
                    // 00e4f98e: ADD ESP,0x10
                    cpu.esp = cpu.add32(cpu.esp, 0x10);
                    pc = 0x00e4f991;
                    continue 'run;
                }
                0x00e4f991 => {
                    // 00e4f991: MOV EDX,dword ptr [EBX + 0x4]
                    cpu.edx = e.mem.u32(cpu.ebx.wrapping_add(0x4));
                    // 00e4f994: MOVSS XMM1,dword ptr [0x0104da48]
                    cpu.x[1] = scalar_vec(e, 0x0104da48);
                    // 00e4f99c: MOVSS XMM0,dword ptr [0x0104e148]
                    cpu.x[0] = scalar_vec(e, 0x0104e148);
                    // 00e4f9a4: MOV dword ptr [ESP + 0x58],ESI
                    e.mem.set_u32(cpu.esp.wrapping_add(0x58), cpu.esi);
                    // 00e4f9a8: SHUFPS XMM1,XMM1,0x0
                    cpu.x[1] = shufps(cpu.x[1], cpu.x[1], 0);
                    // 00e4f9ac: SHUFPS XMM0,XMM0,0x0
                    cpu.x[0] = shufps(cpu.x[0], cpu.x[0], 0);
                    // 00e4f9b0: CMP EDX,EDI
                    cpu.cmp32(cpu.edx, cpu.edi);
                    // 00e4f9b2: JBE 0x00e4fa0c
                    if cpu.cf || cpu.zf {
                        pc = 0x00e4fa0c;
                        continue 'run;
                    }
                    // 00e4f9b4: MOV dword ptr [ESP + 0x48],EDI
                    e.mem.set_u32(cpu.esp.wrapping_add(0x48), cpu.edi);
                    // 00e4f9b8: MOV dword ptr [ESP + 0x18],EDX
                    e.mem.set_u32(cpu.esp.wrapping_add(0x18), cpu.edx);
                    // 00e4f9bc: LEA ESP,[ESP]
                    // (no code: the exe pads with LEA ESP,[ESP])
                    pc = 0x00e4f9c0;
                    continue 'run;
                }
                0x00e4f9c0 => {
                    // 00e4f9c0: MOV ESI,dword ptr [EBX + 0x8]
                    cpu.esi = e.mem.u32(cpu.ebx.wrapping_add(0x8));
                    // 00e4f9c3: TEST ESI,ESI
                    cpu.test32(cpu.esi, cpu.esi);
                    // 00e4f9c5: JBE 0x00e4fa00
                    if cpu.cf || cpu.zf {
                        pc = 0x00e4fa00;
                        continue 'run;
                    }
                    // 00e4f9c7: MOV EAX,dword ptr [EBX]
                    cpu.eax = e.mem.u32(cpu.ebx);
                    // 00e4f9c9: LEA ECX,[EDX + EDX*0x2]
                    cpu.ecx = cpu.edx.wrapping_add(cpu.edx.wrapping_mul(2));
                    // 00e4f9cc: SHL ECX,0x4
                    cpu.ecx = cpu.shl32(cpu.ecx, 0x4);
                    // 00e4f9cf: ADD EAX,dword ptr [ESP + 0x48]
                    cpu.eax = cpu.add32(cpu.eax, e.mem.u32(cpu.esp.wrapping_add(0x48)));
                    pc = 0x00e4f9d3;
                    continue 'run;
                }
                0x00e4f9d3 => {
                    // 00e4f9d3: MOV EDI,dword ptr [EAX]
                    cpu.edi = e.mem.u32(cpu.eax);
                    // 00e4f9d5: MOV dword ptr [ESP + 0x70],EDI
                    e.mem.set_u32(cpu.esp.wrapping_add(0x70), cpu.edi);
                    // 00e4f9d9: MOV EDI,dword ptr [EAX + 0x4]
                    cpu.edi = e.mem.u32(cpu.eax.wrapping_add(0x4));
                    // 00e4f9dc: MOV dword ptr [ESP + 0x74],EDI
                    e.mem.set_u32(cpu.esp.wrapping_add(0x74), cpu.edi);
                    // 00e4f9e0: MOV EDI,dword ptr [EAX + 0x8]
                    cpu.edi = e.mem.u32(cpu.eax.wrapping_add(0x8));
                    // 00e4f9e3: MOV dword ptr [ESP + 0x78],EDI
                    e.mem.set_u32(cpu.esp.wrapping_add(0x78), cpu.edi);
                    // 00e4f9e7: MOV EDI,dword ptr [EAX + 0xc]
                    cpu.edi = e.mem.u32(cpu.eax.wrapping_add(0xc));
                    // 00e4f9ea: MOV dword ptr [ESP + 0x7c],EDI
                    e.mem.set_u32(cpu.esp.wrapping_add(0x7c), cpu.edi);
                    // 00e4f9ee: MOVAPS XMM2,xmmword ptr [ESP + 0x70]
                    cpu.x[2] = load_vec(e, cpu.esp.wrapping_add(0x70));
                    // 00e4f9f3: ADD EAX,ECX
                    cpu.eax = cpu.add32(cpu.eax, cpu.ecx);
                    // 00e4f9f5: SUB ESI,0x1
                    cpu.esi = cpu.sub32(cpu.esi, 0x1);
                    // 00e4f9f8: MINPS XMM1,XMM2
                    cpu.x[1] = minps(cpu.x[1], cpu.x[2]);
                    // 00e4f9fb: MAXPS XMM0,XMM2
                    cpu.x[0] = maxps(cpu.x[0], cpu.x[2]);
                    // 00e4f9fe: JNZ 0x00e4f9d3
                    if !cpu.zf {
                        pc = 0x00e4f9d3;
                        continue 'run;
                    }
                    pc = 0x00e4fa00;
                    continue 'run;
                }
                0x00e4fa00 => {
                    // 00e4fa00: ADD dword ptr [ESP + 0x48],0x30
                    {
                        let at = cpu.esp.wrapping_add(0x48);
                        let v = cpu.add32(e.mem.u32(at), 0x30);
                        e.mem.set_u32(at, v);
                    }
                    // 00e4fa05: SUB dword ptr [ESP + 0x18],0x1
                    {
                        let at = cpu.esp.wrapping_add(0x18);
                        let v = cpu.sub32(e.mem.u32(at), 0x1);
                        e.mem.set_u32(at, v);
                    }
                    // 00e4fa0a: JNZ 0x00e4f9c0
                    if !cpu.zf {
                        pc = 0x00e4f9c0;
                        continue 'run;
                    }
                    pc = 0x00e4fa0c;
                    continue 'run;
                }
                0x00e4fa0c => {
                    // 00e4fa0c: FLDZ
                    cpu.st_push(0.0);
                    // 00e4fa0e: SUBPS XMM0,XMM1
                    cpu.x[0] = subps(cpu.x[0], cpu.x[1]);
                    // 00e4fa11: MOVAPS xmmword ptr [ESP + 0xf0],XMM0
                    store_vec(e, cpu.esp.wrapping_add(0xf0), cpu.x[0]);
                    // 00e4fa19: MOV dword ptr [ESP + 0x4c],0x0
                    e.mem.set_u32(cpu.esp.wrapping_add(0x4c), 0x0);
                    // 00e4fa21: TEST EDX,EDX
                    cpu.test32(cpu.edx, cpu.edx);
                    // 00e4fa23: JBE 0x00e5046c
                    if cpu.cf || cpu.zf {
                        pc = 0x00e5046c;
                        continue 'run;
                    }
                    // 00e4fa29: FLD float ptr [0x0104da4c]
                    cpu.st_push(x87_load32(e.mem.u32(0x0104da4c)));
                    // 00e4fa2f: MOVSS XMM5,dword ptr [0x01012068]
                    cpu.x[5] = scalar_vec(e, 0x01012068);
                    // 00e4fa37: FLD double ptr [0x01011588]
                    cpu.st_push(e.mem.f64(0x01011588));
                    // 00e4fa3d: MOVAPS XMM6,XMM0
                    cpu.x[6] = cpu.x[0];
                    // 00e4fa40: MULPS XMM6,XMM0
                    cpu.x[6] = mulps(cpu.x[6], cpu.x[0]);
                    // 00e4fa43: XORPS XMM0,XMM0
                    cpu.x[0] = [0.0; 4];
                    // 00e4fa46: MOVAPS XMM4,XMM0
                    cpu.x[4] = cpu.x[0];
                    // 00e4fa49: MOVAPS xmmword ptr [ESP + 0x100],XMM4
                    store_vec(e, cpu.esp.wrapping_add(0x100), cpu.x[4]);
                    // 00e4fa51: JMP 0x00e4fa55
                    pc = 0x00e4fa55;
                    continue 'run;
                }
                0x00e4fa53 => {
                    // 00e4fa53: FXCH ST2
                    cpu.st_exchange(2);
                    pc = 0x00e4fa55;
                    continue 'run;
                }
                0x00e4fa55 => {
                    // 00e4fa55: MOV ECX,dword ptr [ESP + 0x4c]
                    cpu.ecx = e.mem.u32(cpu.esp.wrapping_add(0x4c));
                    // 00e4fa59: MOV EDX,dword ptr [EBX + 0xc]
                    cpu.edx = e.mem.u32(cpu.ebx.wrapping_add(0xc));
                    // 00e4fa5c: FLD float ptr [EDX + ECX*0x4]
                    cpu.st_push(x87_load32(
                        e.mem.u32(cpu.edx.wrapping_add(cpu.ecx.wrapping_mul(4))),
                    ));
                    // 00e4fa5f: MOV EAX,dword ptr [EBX + 0x18]
                    cpu.eax = e.mem.u32(cpu.ebx.wrapping_add(0x18));
                    // 00e4fa62: MOV EDX,dword ptr [EBX + 0x24]
                    cpu.edx = e.mem.u32(cpu.ebx.wrapping_add(0x24));
                    // 00e4fa65: FSTP float ptr [ESP + 0x50]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x50), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e4fa69: FLD float ptr [EAX + ECX*0x4]
                    cpu.st_push(x87_load32(
                        e.mem.u32(cpu.eax.wrapping_add(cpu.ecx.wrapping_mul(4))),
                    ));
                    // 00e4fa6c: MOV EAX,dword ptr [EBX + 0x30]
                    cpu.eax = e.mem.u32(cpu.ebx.wrapping_add(0x30));
                    // 00e4fa6f: FSTP float ptr [ESP + 0x48]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x48), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e4fa73: MOVAPS XMM0,XMM6
                    cpu.x[0] = cpu.x[6];
                    // 00e4fa76: FLD float ptr [EDX + ECX*0x4]
                    cpu.st_push(x87_load32(
                        e.mem.u32(cpu.edx.wrapping_add(cpu.ecx.wrapping_mul(4))),
                    ));
                    // 00e4fa79: MOV EDX,dword ptr [EBX + 0x3c]
                    cpu.edx = e.mem.u32(cpu.ebx.wrapping_add(0x3c));
                    // 00e4fa7c: FSTP float ptr [ESP + 0x64]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x64), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e4fa80: SHUFPS XMM0,XMM6,0x55
                    cpu.x[0] = shufps(cpu.x[0], cpu.x[6], 85);
                    // 00e4fa84: FLD float ptr [EAX + ECX*0x4]
                    cpu.st_push(x87_load32(
                        e.mem.u32(cpu.eax.wrapping_add(cpu.ecx.wrapping_mul(4))),
                    ));
                    // 00e4fa87: MOV AL,byte ptr [ECX + EDX*0x1]
                    cpu.eax = (cpu.eax & 0xffff_ff00)
                        | (u32::from(e.mem.u8(cpu.ecx.wrapping_add(cpu.edx))) & 0xff);
                    // 00e4fa8a: FSTP float ptr [ESP + 0x2c]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x2c), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e4fa8e: ADDSS XMM0,XMM6
                    cpu.x[0] = addss(cpu.x[0], cpu.x[6]);
                    // 00e4fa92: FLD float ptr [ESP + 0x48]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x48))));
                    // 00e4fa96: MOVAPS XMM1,XMM6
                    cpu.x[1] = cpu.x[6];
                    // 00e4fa99: FLD ST0
                    {
                        let v = cpu.st(0);
                        cpu.st_push(v);
                    }
                    // 00e4fa9b: SHUFPS XMM1,XMM6,0xaa
                    cpu.x[1] = shufps(cpu.x[1], cpu.x[6], 170);
                    // 00e4fa9f: FMUL ST0
                    {
                        let v = x87_mul(cpu.st(0), cpu.st(0));
                        cpu.set_st(0, v);
                    }
                    // 00e4faa1: ADDSS XMM1,XMM0
                    cpu.x[1] = addss(cpu.x[1], cpu.x[0]);
                    // 00e4faa5: MOV byte ptr [ESP + 0x17],AL
                    e.mem
                        .set_u8(cpu.esp.wrapping_add(0x17), (cpu.eax & 0xff) as u8);
                    // 00e4faa9: FSTP float ptr [ESP + 0x18]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x18), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e4faad: MOVSS XMM0,dword ptr [ESP + 0x18]
                    cpu.x[0] = scalar_vec(e, cpu.esp.wrapping_add(0x18));
                    // 00e4fab3: FXCH ST2
                    cpu.st_exchange(2);
                    // 00e4fab5: MULSS XMM1,XMM0
                    cpu.x[1] = mulss(cpu.x[1], cpu.x[0]);
                    // 00e4fab9: MOVSS dword ptr [ESP + 0x90],XMM1
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x90), cpu.x[1][0].to_bits());
                    // 00e4fac2: FCOM float ptr [ESP + 0x90]
                    cpu.sw = u16::from(fcom_status(
                        cpu.st(0),
                        x87_m32(e.mem.u32(cpu.esp.wrapping_add(0x90))),
                    )) << 8;
                    // 00e4fac9: FNSTSW AX
                    cpu.eax = (cpu.eax & 0xffff_0000) | u32::from(cpu.sw);
                    // 00e4facb: TEST AH,0x41
                    cpu.test8((cpu.eax >> 8) & 0xff, 0x41);
                    // 00e4face: JNZ 0x00e4fb15
                    if !cpu.zf {
                        pc = 0x00e4fb15;
                        continue 'run;
                    }
                    // 00e4fad0: MOVAPS XMM0,XMM6
                    cpu.x[0] = cpu.x[6];
                    // 00e4fad3: FXCH ST3
                    cpu.st_exchange(3);
                    // 00e4fad5: SHUFPS XMM0,XMM6,0x55
                    cpu.x[0] = shufps(cpu.x[0], cpu.x[6], 85);
                    // 00e4fad9: MOVAPS XMM1,XMM6
                    cpu.x[1] = cpu.x[6];
                    // 00e4fadc: ADDSS XMM0,XMM6
                    cpu.x[0] = addss(cpu.x[0], cpu.x[6]);
                    // 00e4fae0: SHUFPS XMM1,XMM6,0xaa
                    cpu.x[1] = shufps(cpu.x[1], cpu.x[6], 170);
                    // 00e4fae4: ADDSS XMM1,XMM0
                    cpu.x[1] = addss(cpu.x[1], cpu.x[0]);
                    // 00e4fae8: MOVSS dword ptr [ESP + 0x94],XMM1
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x94), cpu.x[1][0].to_bits());
                    // 00e4faf1: FCOM float ptr [ESP + 0x94]
                    cpu.sw = u16::from(fcom_status(
                        cpu.st(0),
                        x87_m32(e.mem.u32(cpu.esp.wrapping_add(0x94))),
                    )) << 8;
                    // 00e4faf8: FNSTSW AX
                    cpu.eax = (cpu.eax & 0xffff_0000) | u32::from(cpu.sw);
                    // 00e4fafa: TEST AH,0x5
                    cpu.test8((cpu.eax >> 8) & 0xff, 0x5);
                    // 00e4fafd: JP 0x00e4fb3e
                    if cpu.pf {
                        pc = 0x00e4fb3e;
                        continue 'run;
                    }
                    // 00e4faff: FSTP ST2
                    {
                        let v = cpu.st(0);
                        cpu.set_st(2, v);
                    }
                    cpu.st_pop();
                    // 00e4fb01: FXCH
                    cpu.st_exchange(1);
                    // 00e4fb03: FST float ptr [ESP + 0x48]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x48), x87_store32(cpu.st(0)));
                    // 00e4fb07: FLD float ptr [ESP + 0x48]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x48))));
                    // 00e4fb0b: FXCH
                    cpu.st_exchange(1);
                    // 00e4fb0d: FXCH ST3
                    cpu.st_exchange(3);
                    // 00e4fb0f: FXCH
                    cpu.st_exchange(1);
                    // 00e4fb11: FXCH ST2
                    cpu.st_exchange(2);
                    // 00e4fb13: FXCH
                    cpu.st_exchange(1);
                    pc = 0x00e4fb15;
                    continue 'run;
                }
                0x00e4fb15 => {
                    // 00e4fb15: MOV EDX,dword ptr [ESP + 0x54]
                    cpu.edx = e.mem.u32(cpu.esp.wrapping_add(0x54));
                    // 00e4fb19: LEA ESI,[ECX + ECX*0x2]
                    cpu.esi = cpu.ecx.wrapping_add(cpu.ecx.wrapping_mul(2));
                    // 00e4fb1c: SHL ESI,0x4
                    cpu.esi = cpu.shl32(cpu.esi, 0x4);
                    // 00e4fb1f: LEA EAX,[ESI + EDX*0x1]
                    cpu.eax = cpu.esi.wrapping_add(cpu.edx);
                    // 00e4fb22: XOR EDX,EDX
                    cpu.edx = cpu.xor32(cpu.edx, cpu.edx);
                    // 00e4fb24: MOVAPS xmmword ptr [EAX],XMM4
                    store_vec(e, cpu.eax, cpu.x[4]);
                    // 00e4fb27: MOVAPS xmmword ptr [EAX + 0x10],XMM4
                    store_vec(e, cpu.eax.wrapping_add(0x10), cpu.x[4]);
                    // 00e4fb2b: MOVAPS xmmword ptr [EAX + 0x20],XMM4
                    store_vec(e, cpu.eax.wrapping_add(0x20), cpu.x[4]);
                    // 00e4fb2f: MOV dword ptr [ESP + 0x30],ESI
                    e.mem.set_u32(cpu.esp.wrapping_add(0x30), cpu.esi);
                    // 00e4fb33: CMP dword ptr [EBX + 0x8],EDX
                    cpu.cmp32(e.mem.u32(cpu.ebx.wrapping_add(0x8)), cpu.edx);
                    // 00e4fb36: JBE 0x00e4fc32
                    if cpu.cf || cpu.zf {
                        pc = 0x00e4fc32;
                        continue 'run;
                    }
                    // 00e4fb3c: JMP 0x00e4fb46
                    pc = 0x00e4fb46;
                    continue 'run;
                }
                0x00e4fb3e => {
                    // 00e4fb3e: FXCH ST3
                    cpu.st_exchange(3);
                    // 00e4fb40: JMP 0x00e4fb15
                    pc = 0x00e4fb15;
                    continue 'run;
                }
                0x00e4fb42 => {
                    // 00e4fb42: MOV ECX,dword ptr [ESP + 0x4c]
                    cpu.ecx = e.mem.u32(cpu.esp.wrapping_add(0x4c));
                    pc = 0x00e4fb46;
                    continue 'run;
                }
                0x00e4fb46 => {
                    // 00e4fb46: MOV EAX,dword ptr [EBX + 0x4]
                    cpu.eax = e.mem.u32(cpu.ebx.wrapping_add(0x4));
                    // 00e4fb49: FXCH ST3
                    cpu.st_exchange(3);
                    // 00e4fb4b: IMUL EAX,EDX
                    cpu.eax = cpu.imul32(cpu.eax, cpu.edx);
                    // 00e4fb4e: ADD EAX,ECX
                    cpu.eax = cpu.add32(cpu.eax, cpu.ecx);
                    // 00e4fb50: MOV ECX,dword ptr [ESP + 0x54]
                    cpu.ecx = e.mem.u32(cpu.esp.wrapping_add(0x54));
                    // 00e4fb54: LEA EAX,[EAX + EAX*0x2]
                    cpu.eax = cpu.eax.wrapping_add(cpu.eax.wrapping_mul(2));
                    // 00e4fb57: SHL EAX,0x4
                    cpu.eax = cpu.shl32(cpu.eax, 0x4);
                    // 00e4fb5a: ADD EAX,dword ptr [EBX]
                    cpu.eax = cpu.add32(cpu.eax, e.mem.u32(cpu.ebx));
                    // 00e4fb5c: MOVSS XMM0,XMM5
                    {
                        let s = cpu.x[5][0];
                        cpu.x[0][0] = s;
                    }
                    // 00e4fb60: MOVAPS XMM2,XMM0
                    cpu.x[2] = cpu.x[0];
                    // 00e4fb63: SHUFPS XMM0,XMM2,0x0
                    cpu.x[0] = shufps(cpu.x[0], cpu.x[2], 0);
                    // 00e4fb67: MOVAPS XMM3,xmmword ptr [EAX]
                    cpu.x[3] = load_vec(e, cpu.eax);
                    // 00e4fb6a: ADD ECX,ESI
                    cpu.ecx = cpu.add32(cpu.ecx, cpu.esi);
                    // 00e4fb6c: MOVAPS XMM1,XMM0
                    cpu.x[1] = cpu.x[0];
                    // 00e4fb6f: MULPS XMM1,XMM3
                    cpu.x[1] = mulps(cpu.x[1], cpu.x[3]);
                    // 00e4fb72: MOVAPS XMM3,xmmword ptr [ECX]
                    cpu.x[3] = load_vec(e, cpu.ecx);
                    // 00e4fb75: ADDPS XMM1,XMM3
                    cpu.x[1] = addps(cpu.x[1], cpu.x[3]);
                    // 00e4fb78: MOVAPS xmmword ptr [ECX],XMM1
                    store_vec(e, cpu.ecx, cpu.x[1]);
                    // 00e4fb7b: MOVAPS XMM1,xmmword ptr [EAX + 0x20]
                    cpu.x[1] = load_vec(e, cpu.eax.wrapping_add(0x20));
                    // 00e4fb7f: MULPS XMM1,XMM0
                    cpu.x[1] = mulps(cpu.x[1], cpu.x[0]);
                    // 00e4fb82: MOVAPS XMM0,xmmword ptr [ECX + 0x20]
                    cpu.x[0] = load_vec(e, cpu.ecx.wrapping_add(0x20));
                    // 00e4fb86: ADDPS XMM1,XMM0
                    cpu.x[1] = addps(cpu.x[1], cpu.x[0]);
                    // 00e4fb89: MOVAPS xmmword ptr [ECX + 0x20],XMM1
                    store_vec(e, cpu.ecx.wrapping_add(0x20), cpu.x[1]);
                    // 00e4fb8d: MOVAPS XMM3,xmmword ptr [EAX + 0x10]
                    cpu.x[3] = load_vec(e, cpu.eax.wrapping_add(0x10));
                    // 00e4fb91: MOVAPS XMM0,xmmword ptr [ECX + 0x10]
                    cpu.x[0] = load_vec(e, cpu.ecx.wrapping_add(0x10));
                    // 00e4fb95: MOVAPS XMM1,XMM3
                    cpu.x[1] = cpu.x[3];
                    // 00e4fb98: MULPS XMM1,XMM0
                    cpu.x[1] = mulps(cpu.x[1], cpu.x[0]);
                    // 00e4fb9b: MOVAPS XMM0,XMM1
                    cpu.x[0] = cpu.x[1];
                    // 00e4fb9e: SHUFPS XMM0,XMM1,0x4e
                    cpu.x[0] = shufps(cpu.x[0], cpu.x[1], 78);
                    // 00e4fba2: ADDPS XMM0,XMM1
                    cpu.x[0] = addps(cpu.x[0], cpu.x[1]);
                    // 00e4fba5: MOVAPS XMM1,XMM0
                    cpu.x[1] = cpu.x[0];
                    // 00e4fba8: SHUFPS XMM1,XMM0,0xb1
                    cpu.x[1] = shufps(cpu.x[1], cpu.x[0], 177);
                    // 00e4fbac: ADDPS XMM1,XMM0
                    cpu.x[1] = addps(cpu.x[1], cpu.x[0]);
                    // 00e4fbaf: MOVSS dword ptr [ESP + 0x9c],XMM1
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x9c), cpu.x[1][0].to_bits());
                    // 00e4fbb8: FCOM float ptr [ESP + 0x9c]
                    cpu.sw = u16::from(fcom_status(
                        cpu.st(0),
                        x87_m32(e.mem.u32(cpu.esp.wrapping_add(0x9c))),
                    )) << 8;
                    // 00e4fbbf: MOVAPS xmmword ptr [ESP + 0x120],XMM2
                    store_vec(e, cpu.esp.wrapping_add(0x120), cpu.x[2]);
                    // 00e4fbc7: FNSTSW AX
                    cpu.eax = (cpu.eax & 0xffff_0000) | u32::from(cpu.sw);
                    // 00e4fbc9: TEST AH,0x41
                    cpu.test8((cpu.eax >> 8) & 0xff, 0x41);
                    // 00e4fbcc: JNZ 0x00e4fbea
                    if !cpu.zf {
                        pc = 0x00e4fbea;
                        continue 'run;
                    }
                    // 00e4fbce: MOVAPS XMM0,xmmword ptr [0x010c7160]
                    cpu.x[0] = load_vec(e, 0x010c7160);
                    // 00e4fbd5: SUBSS XMM0,XMM2
                    cpu.x[0] = subss(cpu.x[0], cpu.x[2]);
                    // 00e4fbd9: MOVAPS xmmword ptr [ESP + 0x110],XMM0
                    store_vec(e, cpu.esp.wrapping_add(0x110), cpu.x[0]);
                    // 00e4fbe1: LEA EAX,[ESP + 0x110]
                    cpu.eax = cpu.esp.wrapping_add(0x110);
                    // 00e4fbe8: JMP 0x00e4fbf1
                    pc = 0x00e4fbf1;
                    continue 'run;
                }
                0x00e4fbea => {
                    // 00e4fbea: LEA EAX,[ESP + 0x120]
                    cpu.eax = cpu.esp.wrapping_add(0x120);
                    pc = 0x00e4fbf1;
                    continue 'run;
                }
                0x00e4fbf1 => {
                    // 00e4fbf1: MOV EDI,dword ptr [EAX]
                    cpu.edi = e.mem.u32(cpu.eax);
                    // 00e4fbf3: FXCH ST3
                    cpu.st_exchange(3);
                    // 00e4fbf5: MOV dword ptr [ESP + 0x70],EDI
                    e.mem.set_u32(cpu.esp.wrapping_add(0x70), cpu.edi);
                    // 00e4fbf9: MOV EDI,dword ptr [EAX + 0x4]
                    cpu.edi = e.mem.u32(cpu.eax.wrapping_add(0x4));
                    // 00e4fbfc: MOV dword ptr [ESP + 0x74],EDI
                    e.mem.set_u32(cpu.esp.wrapping_add(0x74), cpu.edi);
                    // 00e4fc00: MOV EDI,dword ptr [EAX + 0x8]
                    cpu.edi = e.mem.u32(cpu.eax.wrapping_add(0x8));
                    // 00e4fc03: MOV EAX,dword ptr [EAX + 0xc]
                    cpu.eax = e.mem.u32(cpu.eax.wrapping_add(0xc));
                    // 00e4fc06: MOV dword ptr [ESP + 0x78],EDI
                    e.mem.set_u32(cpu.esp.wrapping_add(0x78), cpu.edi);
                    // 00e4fc0a: MOV dword ptr [ESP + 0x7c],EAX
                    e.mem.set_u32(cpu.esp.wrapping_add(0x7c), cpu.eax);
                    // 00e4fc0e: MOVAPS XMM0,xmmword ptr [ESP + 0x70]
                    cpu.x[0] = load_vec(e, cpu.esp.wrapping_add(0x70));
                    // 00e4fc13: MOVAPS XMM1,XMM0
                    cpu.x[1] = cpu.x[0];
                    // 00e4fc16: SHUFPS XMM1,XMM0,0x0
                    cpu.x[1] = shufps(cpu.x[1], cpu.x[0], 0);
                    // 00e4fc1a: MOVAPS XMM0,xmmword ptr [ECX + 0x10]
                    cpu.x[0] = load_vec(e, cpu.ecx.wrapping_add(0x10));
                    // 00e4fc1e: MULPS XMM1,XMM3
                    cpu.x[1] = mulps(cpu.x[1], cpu.x[3]);
                    // 00e4fc21: ADDPS XMM1,XMM0
                    cpu.x[1] = addps(cpu.x[1], cpu.x[0]);
                    // 00e4fc24: INC EDX
                    cpu.edx = cpu.inc32(cpu.edx);
                    // 00e4fc25: MOVAPS xmmword ptr [ECX + 0x10],XMM1
                    store_vec(e, cpu.ecx.wrapping_add(0x10), cpu.x[1]);
                    // 00e4fc29: CMP EDX,dword ptr [EBX + 0x8]
                    cpu.cmp32(cpu.edx, e.mem.u32(cpu.ebx.wrapping_add(0x8)));
                    // 00e4fc2c: JC 0x00e4fb42
                    if cpu.cf {
                        pc = 0x00e4fb42;
                        continue 'run;
                    }
                    pc = 0x00e4fc32;
                    continue 'run;
                }
                0x00e4fc32 => {
                    // 00e4fc32: MOV ECX,dword ptr [EBX + 0x8]
                    cpu.ecx = e.mem.u32(cpu.ebx.wrapping_add(0x8));
                    // 00e4fc35: FILD dword ptr [EBX + 0x8]
                    cpu.st_push(f64::from(e.mem.i32(cpu.ebx.wrapping_add(0x8))));
                    // 00e4fc38: TEST ECX,ECX
                    cpu.test32(cpu.ecx, cpu.ecx);
                    // 00e4fc3a: JGE 0x00e4fc42
                    if cpu.sf == cpu.of {
                        pc = 0x00e4fc42;
                        continue 'run;
                    }
                    // 00e4fc3c: FADD float ptr [0x01099378]
                    {
                        let v = x87_add(cpu.st(0), x87_m32(e.mem.u32(0x01099378)));
                        cpu.set_st(0, v);
                    }
                    pc = 0x00e4fc42;
                    continue 'run;
                }
                0x00e4fc42 => {
                    // 00e4fc42: MOV EDX,dword ptr [ESP + 0x54]
                    cpu.edx = e.mem.u32(cpu.esp.wrapping_add(0x54));
                    // 00e4fc46: FSTP float ptr [ESP + 0x18]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x18), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e4fc4a: MOVSS XMM1,dword ptr [ESP + 0x18]
                    cpu.x[1] = scalar_vec(e, cpu.esp.wrapping_add(0x18));
                    // 00e4fc50: MOVSS dword ptr [ESP + 0x88],XMM1
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x88), cpu.x[1][0].to_bits());
                    // 00e4fc59: FLD float ptr [ESP + 0x88]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x88))));
                    // 00e4fc60: FABS
                    {
                        let v = cpu.st(0).abs();
                        cpu.set_st(0, v);
                    }
                    // 00e4fc62: LEA ECX,[ESI + EDX*0x1]
                    cpu.ecx = cpu.esi.wrapping_add(cpu.edx);
                    // 00e4fc65: FSTP float ptr [ESP + 0x18]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x18), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e4fc69: FLD float ptr [ESP + 0x18]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x18))));
                    // 00e4fc6d: FCOMP
                    cpu.sw = u16::from(fcom_status(cpu.st(0), cpu.st(1))) << 8;
                    cpu.st_pop();
                    // 00e4fc6f: FNSTSW AX
                    cpu.eax = (cpu.eax & 0xffff_0000) | u32::from(cpu.sw);
                    // 00e4fc71: TEST AH,0x5
                    cpu.test8((cpu.eax >> 8) & 0xff, 0x5);
                    // 00e4fc74: JP 0x00e4fc89
                    if cpu.pf {
                        pc = 0x00e4fc89;
                        continue 'run;
                    }
                    // 00e4fc76: MOVAPS xmmword ptr [ECX],XMM4
                    store_vec(e, cpu.ecx, cpu.x[4]);
                    // 00e4fc79: MOVAPS XMM0,xmmword ptr [0x010c71b0]
                    cpu.x[0] = load_vec(e, 0x010c71b0);
                    // 00e4fc80: MOVAPS xmmword ptr [ECX + 0x10],XMM0
                    store_vec(e, cpu.ecx.wrapping_add(0x10), cpu.x[0]);
                    // 00e4fc84: JMP 0x00e4fd90
                    pc = 0x00e4fd90;
                    continue 'run;
                }
                0x00e4fc89 => {
                    // 00e4fc89: MOVSS XMM0,XMM5
                    {
                        let s = cpu.x[5][0];
                        cpu.x[0][0] = s;
                    }
                    // 00e4fc8d: DIVSS XMM0,XMM1
                    cpu.x[0] = divss(cpu.x[0], cpu.x[1]);
                    // 00e4fc91: MOVAPS XMM1,xmmword ptr [ECX]
                    cpu.x[1] = load_vec(e, cpu.ecx);
                    // 00e4fc94: SHUFPS XMM0,XMM0,0x0
                    cpu.x[0] = shufps(cpu.x[0], cpu.x[0], 0);
                    // 00e4fc98: MULPS XMM1,XMM0
                    cpu.x[1] = mulps(cpu.x[1], cpu.x[0]);
                    // 00e4fc9b: MOVAPS xmmword ptr [ECX],XMM1
                    store_vec(e, cpu.ecx, cpu.x[1]);
                    // 00e4fc9e: MOVAPS XMM1,xmmword ptr [ECX + 0x20]
                    cpu.x[1] = load_vec(e, cpu.ecx.wrapping_add(0x20));
                    // 00e4fca2: MULPS XMM1,XMM0
                    cpu.x[1] = mulps(cpu.x[1], cpu.x[0]);
                    // 00e4fca5: MOVAPS xmmword ptr [ECX + 0x20],XMM1
                    store_vec(e, cpu.ecx.wrapping_add(0x20), cpu.x[1]);
                    // 00e4fca9: MOVAPS XMM0,xmmword ptr [ECX + 0x10]
                    cpu.x[0] = load_vec(e, cpu.ecx.wrapping_add(0x10));
                    // 00e4fcad: MULPS XMM0,XMM0
                    cpu.x[0] = mulps(cpu.x[0], cpu.x[0]);
                    // 00e4fcb0: MOVAPS XMM1,XMM0
                    cpu.x[1] = cpu.x[0];
                    // 00e4fcb3: SHUFPS XMM0,XMM1,0x4e
                    cpu.x[0] = shufps(cpu.x[0], cpu.x[1], 78);
                    // 00e4fcb7: ADDPS XMM0,XMM1
                    cpu.x[0] = addps(cpu.x[0], cpu.x[1]);
                    // 00e4fcba: MOVAPS XMM1,XMM0
                    cpu.x[1] = cpu.x[0];
                    // 00e4fcbd: SHUFPS XMM1,XMM0,0xb1
                    cpu.x[1] = shufps(cpu.x[1], cpu.x[0], 177);
                    // 00e4fcc1: ADDPS XMM1,XMM0
                    cpu.x[1] = addps(cpu.x[1], cpu.x[0]);
                    // 00e4fcc4: MOVSS dword ptr [ESP + 0x98],XMM1
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x98), cpu.x[1][0].to_bits());
                    // 00e4fccd: FCOM float ptr [ESP + 0x98]
                    cpu.sw = u16::from(fcom_status(
                        cpu.st(0),
                        x87_m32(e.mem.u32(cpu.esp.wrapping_add(0x98))),
                    )) << 8;
                    // 00e4fcd4: FNSTSW AX
                    cpu.eax = (cpu.eax & 0xffff_0000) | u32::from(cpu.sw);
                    // 00e4fcd6: TEST AH,0x41
                    cpu.test8((cpu.eax >> 8) & 0xff, 0x41);
                    // 00e4fcd9: JNZ 0x00e4fce4
                    if !cpu.zf {
                        pc = 0x00e4fce4;
                        continue 'run;
                    }
                    // 00e4fcdb: MOVAPS XMM0,xmmword ptr [0x010c71b0]
                    cpu.x[0] = load_vec(e, 0x010c71b0);
                    // 00e4fce2: JMP 0x00e4fd58
                    pc = 0x00e4fd58;
                    continue 'run;
                }
                0x00e4fce4 => {
                    // 00e4fce4: MOVAPS XMM2,xmmword ptr [ECX + 0x10]
                    cpu.x[2] = load_vec(e, cpu.ecx.wrapping_add(0x10));
                    // 00e4fce8: MOVAPS XMM0,XMM2
                    cpu.x[0] = cpu.x[2];
                    // 00e4fceb: MULPS XMM0,XMM2
                    cpu.x[0] = mulps(cpu.x[0], cpu.x[2]);
                    // 00e4fcee: MOVAPS XMM1,XMM0
                    cpu.x[1] = cpu.x[0];
                    // 00e4fcf1: SHUFPS XMM1,XMM0,0x4e
                    cpu.x[1] = shufps(cpu.x[1], cpu.x[0], 78);
                    // 00e4fcf5: ADDPS XMM1,XMM0
                    cpu.x[1] = addps(cpu.x[1], cpu.x[0]);
                    // 00e4fcf8: MOVAPS XMM0,XMM1
                    cpu.x[0] = cpu.x[1];
                    // 00e4fcfb: SHUFPS XMM0,XMM1,0xb1
                    cpu.x[0] = shufps(cpu.x[0], cpu.x[1], 177);
                    // 00e4fcff: ADDPS XMM0,XMM1
                    cpu.x[0] = addps(cpu.x[0], cpu.x[1]);
                    // 00e4fd02: MOVAPS XMM1,XMM0
                    cpu.x[1] = cpu.x[0];
                    // 00e4fd05: RSQRTSS XMM1,XMM1
                    cpu.x[1] = rsqrtss(cpu.x[1], cpu.x[1]);
                    // 00e4fd09: MOVAPS XMM3,XMM1
                    cpu.x[3] = cpu.x[1];
                    // 00e4fd0c: MOVSS XMM1,dword ptr [0x01011d78]
                    cpu.x[1] = scalar_vec(e, 0x01011d78);
                    // 00e4fd14: MOVAPS XMM4,XMM1
                    cpu.x[4] = cpu.x[1];
                    // 00e4fd17: MOVAPS XMM1,XMM0
                    cpu.x[1] = cpu.x[0];
                    // 00e4fd1a: MULSS XMM0,XMM3
                    cpu.x[0] = mulss(cpu.x[0], cpu.x[3]);
                    // 00e4fd1e: MULSS XMM0,XMM3
                    cpu.x[0] = mulss(cpu.x[0], cpu.x[3]);
                    // 00e4fd22: CMPEQSS XMM1,XMM4
                    cpu.x[1] = cmpeqss(cpu.x[1], cpu.x[4]);
                    // 00e4fd27: MOVSS XMM4,dword ptr [0x01017718]
                    cpu.x[4] = scalar_vec(e, 0x01017718);
                    // 00e4fd2f: SUBSS XMM4,XMM0
                    cpu.x[4] = subss(cpu.x[4], cpu.x[0]);
                    // 00e4fd33: MOVSS XMM0,dword ptr [0x01016248]
                    cpu.x[0] = scalar_vec(e, 0x01016248);
                    // 00e4fd3b: MULSS XMM0,XMM3
                    cpu.x[0] = mulss(cpu.x[0], cpu.x[3]);
                    // 00e4fd3f: MULSS XMM0,XMM4
                    cpu.x[0] = mulss(cpu.x[0], cpu.x[4]);
                    // 00e4fd43: MOVAPS XMM4,xmmword ptr [ESP + 0x100]
                    cpu.x[4] = load_vec(e, cpu.esp.wrapping_add(0x100));
                    // 00e4fd4b: ANDNPS XMM1,XMM0
                    cpu.x[1] = andnps(cpu.x[1], cpu.x[0]);
                    // 00e4fd4e: MOVAPS XMM0,XMM1
                    cpu.x[0] = cpu.x[1];
                    // 00e4fd51: SHUFPS XMM0,XMM1,0x0
                    cpu.x[0] = shufps(cpu.x[0], cpu.x[1], 0);
                    // 00e4fd55: MULPS XMM0,XMM2
                    cpu.x[0] = mulps(cpu.x[0], cpu.x[2]);
                    pc = 0x00e4fd58;
                    continue 'run;
                }
                0x00e4fd58 => {
                    // 00e4fd58: MOVAPS xmmword ptr [ECX + 0x10],XMM0
                    store_vec(e, cpu.ecx.wrapping_add(0x10), cpu.x[0]);
                    // 00e4fd5c: MOVAPS XMM0,xmmword ptr [ECX + 0x20]
                    cpu.x[0] = load_vec(e, cpu.ecx.wrapping_add(0x20));
                    // 00e4fd60: MULPS XMM0,XMM0
                    cpu.x[0] = mulps(cpu.x[0], cpu.x[0]);
                    // 00e4fd63: MOVAPS XMM1,XMM0
                    cpu.x[1] = cpu.x[0];
                    // 00e4fd66: SHUFPS XMM1,XMM0,0x55
                    cpu.x[1] = shufps(cpu.x[1], cpu.x[0], 85);
                    // 00e4fd6a: MOVAPS XMM2,XMM0
                    cpu.x[2] = cpu.x[0];
                    // 00e4fd6d: ADDSS XMM1,XMM0
                    cpu.x[1] = addss(cpu.x[1], cpu.x[0]);
                    // 00e4fd71: SHUFPS XMM2,XMM0,0xaa
                    cpu.x[2] = shufps(cpu.x[2], cpu.x[0], 170);
                    // 00e4fd75: ADDSS XMM2,XMM1
                    cpu.x[2] = addss(cpu.x[2], cpu.x[1]);
                    // 00e4fd79: MOVSS dword ptr [ESP + 0x8c],XMM2
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x8c), cpu.x[2][0].to_bits());
                    // 00e4fd82: FCOM float ptr [ESP + 0x8c]
                    cpu.sw = u16::from(fcom_status(
                        cpu.st(0),
                        x87_m32(e.mem.u32(cpu.esp.wrapping_add(0x8c))),
                    )) << 8;
                    // 00e4fd89: FNSTSW AX
                    cpu.eax = (cpu.eax & 0xffff_0000) | u32::from(cpu.sw);
                    // 00e4fd8b: TEST AH,0x41
                    cpu.test8((cpu.eax >> 8) & 0xff, 0x41);
                    // 00e4fd8e: JNZ 0x00e4fd9c
                    if !cpu.zf {
                        pc = 0x00e4fd9c;
                        continue 'run;
                    }
                    pc = 0x00e4fd90;
                    continue 'run;
                }
                0x00e4fd90 => {
                    // 00e4fd90: MOVSS XMM0,XMM5
                    {
                        let s = cpu.x[5][0];
                        cpu.x[0][0] = s;
                    }
                    // 00e4fd94: SHUFPS XMM0,XMM0,0x0
                    cpu.x[0] = shufps(cpu.x[0], cpu.x[0], 0);
                    // 00e4fd98: MOVAPS xmmword ptr [ECX + 0x20],XMM0
                    store_vec(e, cpu.ecx.wrapping_add(0x20), cpu.x[0]);
                    pc = 0x00e4fd9c;
                    continue 'run;
                }
                0x00e4fd9c => {
                    // 00e4fd9c: CMP byte ptr [ESP + 0x17],0x0
                    cpu.cmp8(u32::from(e.mem.u8(cpu.esp.wrapping_add(0x17))), 0x0);
                    // 00e4fda1: MOV byte ptr [ESP + 0x23],0x0
                    e.mem.set_u8(cpu.esp.wrapping_add(0x23), 0x0);
                    // 00e4fda6: MOV byte ptr [ESP + 0x22],0x0
                    e.mem.set_u8(cpu.esp.wrapping_add(0x22), 0x0);
                    // 00e4fdab: JZ 0x00e4ffb0
                    if cpu.zf {
                        pc = 0x00e4ffb0;
                        continue 'run;
                    }
                    // 00e4fdb1: FLD float ptr [0x0104e148]
                    cpu.st_push(x87_load32(e.mem.u32(0x0104e148)));
                    // 00e4fdb7: MOV EDI,dword ptr [EBX + 0x8]
                    cpu.edi = e.mem.u32(cpu.ebx.wrapping_add(0x8));
                    // 00e4fdba: FSTP float ptr [ESP + 0x28]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x28), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e4fdbe: XOR ESI,ESI
                    cpu.esi = cpu.xor32(cpu.esi, cpu.esi);
                    // 00e4fdc0: CMP EDI,0x4
                    cpu.cmp32(cpu.edi, 0x4);
                    // 00e4fdc3: FLD float ptr [0x0104da48]
                    cpu.st_push(x87_load32(e.mem.u32(0x0104da48)));
                    // 00e4fdc9: FSTP float ptr [ESP + 0x24]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x24), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e4fdcd: JL 0x00e4ff1d
                    if cpu.sf != cpu.of {
                        pc = 0x00e4ff1d;
                        continue 'run;
                    }
                    // 00e4fdd3: MOV EAX,dword ptr [EBP + 0x8]
                    cpu.eax = e.mem.u32(cpu.ebp.wrapping_add(0x8));
                    // 00e4fdd6: MOV EDX,dword ptr [EAX]
                    cpu.edx = e.mem.u32(cpu.eax);
                    // 00e4fdd8: MOV EAX,dword ptr [EAX + 0x4]
                    cpu.eax = e.mem.u32(cpu.eax.wrapping_add(0x4));
                    // 00e4fddb: MOV ESI,dword ptr [ESP + 0x30]
                    cpu.esi = e.mem.u32(cpu.esp.wrapping_add(0x30));
                    // 00e4fddf: LEA EBX,[ESI + EDX*0x1 + 0x1c]
                    cpu.ebx = cpu.esi.wrapping_add(cpu.edx).wrapping_add(0x1c);
                    // 00e4fde3: MOV ESI,dword ptr [ESP + 0x4c]
                    cpu.esi = e.mem.u32(cpu.esp.wrapping_add(0x4c));
                    // 00e4fde7: LEA ESI,[ESI + EAX*0x2]
                    cpu.esi = cpu.esi.wrapping_add(cpu.eax.wrapping_mul(2));
                    // 00e4fdea: ADD ESI,EAX
                    cpu.esi = cpu.add32(cpu.esi, cpu.eax);
                    // 00e4fdec: LEA ESI,[ESI + ESI*0x2]
                    cpu.esi = cpu.esi.wrapping_add(cpu.esi.wrapping_mul(2));
                    // 00e4fdef: SHL ESI,0x4
                    cpu.esi = cpu.shl32(cpu.esi, 0x4);
                    // 00e4fdf2: LEA ESI,[ESI + EDX*0x1 + 0x1c]
                    cpu.esi = cpu.esi.wrapping_add(cpu.edx).wrapping_add(0x1c);
                    // 00e4fdf6: MOV dword ptr [ESP + 0x18],ESI
                    e.mem.set_u32(cpu.esp.wrapping_add(0x18), cpu.esi);
                    // 00e4fdfa: MOV ESI,dword ptr [ESP + 0x4c]
                    cpu.esi = e.mem.u32(cpu.esp.wrapping_add(0x4c));
                    // 00e4fdfe: LEA ESI,[ESI + EAX*0x2]
                    cpu.esi = cpu.esi.wrapping_add(cpu.eax.wrapping_mul(2));
                    // 00e4fe01: LEA ESI,[ESI + ESI*0x2]
                    cpu.esi = cpu.esi.wrapping_add(cpu.esi.wrapping_mul(2));
                    // 00e4fe04: SHL ESI,0x4
                    cpu.esi = cpu.shl32(cpu.esi, 0x4);
                    // 00e4fe07: LEA ESI,[ESI + EDX*0x1 + 0x1c]
                    cpu.esi = cpu.esi.wrapping_add(cpu.edx).wrapping_add(0x1c);
                    // 00e4fe0b: LEA ECX,[EAX + EAX*0x2]
                    cpu.ecx = cpu.eax.wrapping_add(cpu.eax.wrapping_mul(2));
                    // 00e4fe0e: MOV dword ptr [ESP + 0x68],ESI
                    e.mem.set_u32(cpu.esp.wrapping_add(0x68), cpu.esi);
                    // 00e4fe12: MOV ESI,dword ptr [ESP + 0x4c]
                    cpu.esi = e.mem.u32(cpu.esp.wrapping_add(0x4c));
                    // 00e4fe16: ADD EAX,ESI
                    cpu.eax = cpu.add32(cpu.eax, cpu.esi);
                    // 00e4fe18: LEA EAX,[EAX + EAX*0x2]
                    cpu.eax = cpu.eax.wrapping_add(cpu.eax.wrapping_mul(2));
                    // 00e4fe1b: SHL EAX,0x4
                    cpu.eax = cpu.shl32(cpu.eax, 0x4);
                    // 00e4fe1e: LEA EDX,[EAX + EDX*0x1 + 0x1c]
                    cpu.edx = cpu.eax.wrapping_add(cpu.edx).wrapping_add(0x1c);
                    // 00e4fe22: MOV dword ptr [ESP + 0x6c],EDX
                    e.mem.set_u32(cpu.esp.wrapping_add(0x6c), cpu.edx);
                    // 00e4fe26: LEA EDX,[EDI + -0x4]
                    cpu.edx = cpu.edi.wrapping_sub(0x4);
                    // 00e4fe29: SHR EDX,0x2
                    cpu.edx = cpu.shr32(cpu.edx, 0x2);
                    // 00e4fe2c: SHL ECX,0x6
                    cpu.ecx = cpu.shl32(cpu.ecx, 0x6);
                    // 00e4fe2f: INC EDX
                    cpu.edx = cpu.inc32(cpu.edx);
                    // 00e4fe30: LEA ESI,[EDX*0x4 + 0x0]
                    cpu.esi = cpu.edx.wrapping_mul(4);
                    pc = 0x00e4fe37;
                    continue 'run;
                }
                0x00e4fe37 => {
                    // 00e4fe37: FLD float ptr [EBX]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.ebx)));
                    // 00e4fe39: FSTP float ptr [ESP + 0x1c]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x1c), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e4fe3d: FLD float ptr [ESP + 0x1c]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x1c))));
                    // 00e4fe41: FLD float ptr [ESP + 0x28]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x28))));
                    // 00e4fe45: FCOMP
                    cpu.sw = u16::from(fcom_status(cpu.st(0), cpu.st(1))) << 8;
                    cpu.st_pop();
                    // 00e4fe47: FNSTSW AX
                    cpu.eax = (cpu.eax & 0xffff_0000) | u32::from(cpu.sw);
                    // 00e4fe49: TEST AH,0x5
                    cpu.test8((cpu.eax >> 8) & 0xff, 0x5);
                    // 00e4fe4c: JP 0x00e4fe52
                    if cpu.pf {
                        pc = 0x00e4fe52;
                        continue 'run;
                    }
                    // 00e4fe4e: FST float ptr [ESP + 0x28]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x28), x87_store32(cpu.st(0)));
                    pc = 0x00e4fe52;
                    continue 'run;
                }
                0x00e4fe52 => {
                    // 00e4fe52: FLD float ptr [ESP + 0x24]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x24))));
                    // 00e4fe56: FCOMP
                    cpu.sw = u16::from(fcom_status(cpu.st(0), cpu.st(1))) << 8;
                    cpu.st_pop();
                    // 00e4fe58: FNSTSW AX
                    cpu.eax = (cpu.eax & 0xffff_0000) | u32::from(cpu.sw);
                    // 00e4fe5a: TEST AH,0x41
                    cpu.test8((cpu.eax >> 8) & 0xff, 0x41);
                    // 00e4fe5d: JNZ 0x00e4fe65
                    if !cpu.zf {
                        pc = 0x00e4fe65;
                        continue 'run;
                    }
                    // 00e4fe5f: FSTP float ptr [ESP + 0x24]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x24), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e4fe63: JMP 0x00e4fe67
                    pc = 0x00e4fe67;
                    continue 'run;
                }
                0x00e4fe65 => {
                    // 00e4fe65: FSTP ST0
                    {
                        let v = cpu.st(0);
                        cpu.set_st(0, v);
                    }
                    cpu.st_pop();
                    pc = 0x00e4fe67;
                    continue 'run;
                }
                0x00e4fe67 => {
                    // 00e4fe67: MOV EAX,dword ptr [ESP + 0x6c]
                    cpu.eax = e.mem.u32(cpu.esp.wrapping_add(0x6c));
                    // 00e4fe6b: FLD float ptr [EAX]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.eax)));
                    // 00e4fe6d: FSTP float ptr [ESP + 0x1c]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x1c), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e4fe71: FLD float ptr [ESP + 0x1c]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x1c))));
                    // 00e4fe75: FLD float ptr [ESP + 0x28]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x28))));
                    // 00e4fe79: FCOMP
                    cpu.sw = u16::from(fcom_status(cpu.st(0), cpu.st(1))) << 8;
                    cpu.st_pop();
                    // 00e4fe7b: FNSTSW AX
                    cpu.eax = (cpu.eax & 0xffff_0000) | u32::from(cpu.sw);
                    // 00e4fe7d: TEST AH,0x5
                    cpu.test8((cpu.eax >> 8) & 0xff, 0x5);
                    // 00e4fe80: JP 0x00e4fe86
                    if cpu.pf {
                        pc = 0x00e4fe86;
                        continue 'run;
                    }
                    // 00e4fe82: FST float ptr [ESP + 0x28]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x28), x87_store32(cpu.st(0)));
                    pc = 0x00e4fe86;
                    continue 'run;
                }
                0x00e4fe86 => {
                    // 00e4fe86: FLD float ptr [ESP + 0x24]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x24))));
                    // 00e4fe8a: FCOMP
                    cpu.sw = u16::from(fcom_status(cpu.st(0), cpu.st(1))) << 8;
                    cpu.st_pop();
                    // 00e4fe8c: FNSTSW AX
                    cpu.eax = (cpu.eax & 0xffff_0000) | u32::from(cpu.sw);
                    // 00e4fe8e: TEST AH,0x41
                    cpu.test8((cpu.eax >> 8) & 0xff, 0x41);
                    // 00e4fe91: JNZ 0x00e4fe99
                    if !cpu.zf {
                        pc = 0x00e4fe99;
                        continue 'run;
                    }
                    // 00e4fe93: FSTP float ptr [ESP + 0x24]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x24), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e4fe97: JMP 0x00e4fe9b
                    pc = 0x00e4fe9b;
                    continue 'run;
                }
                0x00e4fe99 => {
                    // 00e4fe99: FSTP ST0
                    {
                        let v = cpu.st(0);
                        cpu.set_st(0, v);
                    }
                    cpu.st_pop();
                    pc = 0x00e4fe9b;
                    continue 'run;
                }
                0x00e4fe9b => {
                    // 00e4fe9b: MOV EAX,dword ptr [ESP + 0x68]
                    cpu.eax = e.mem.u32(cpu.esp.wrapping_add(0x68));
                    // 00e4fe9f: FLD float ptr [EAX]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.eax)));
                    // 00e4fea1: FSTP float ptr [ESP + 0x1c]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x1c), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e4fea5: FLD float ptr [ESP + 0x1c]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x1c))));
                    // 00e4fea9: FLD float ptr [ESP + 0x28]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x28))));
                    // 00e4fead: FCOMP
                    cpu.sw = u16::from(fcom_status(cpu.st(0), cpu.st(1))) << 8;
                    cpu.st_pop();
                    // 00e4feaf: FNSTSW AX
                    cpu.eax = (cpu.eax & 0xffff_0000) | u32::from(cpu.sw);
                    // 00e4feb1: TEST AH,0x5
                    cpu.test8((cpu.eax >> 8) & 0xff, 0x5);
                    // 00e4feb4: JP 0x00e4feba
                    if cpu.pf {
                        pc = 0x00e4feba;
                        continue 'run;
                    }
                    // 00e4feb6: FST float ptr [ESP + 0x28]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x28), x87_store32(cpu.st(0)));
                    pc = 0x00e4feba;
                    continue 'run;
                }
                0x00e4feba => {
                    // 00e4feba: FLD float ptr [ESP + 0x24]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x24))));
                    // 00e4febe: FCOMP
                    cpu.sw = u16::from(fcom_status(cpu.st(0), cpu.st(1))) << 8;
                    cpu.st_pop();
                    // 00e4fec0: FNSTSW AX
                    cpu.eax = (cpu.eax & 0xffff_0000) | u32::from(cpu.sw);
                    // 00e4fec2: TEST AH,0x41
                    cpu.test8((cpu.eax >> 8) & 0xff, 0x41);
                    // 00e4fec5: JNZ 0x00e4fecd
                    if !cpu.zf {
                        pc = 0x00e4fecd;
                        continue 'run;
                    }
                    // 00e4fec7: FSTP float ptr [ESP + 0x24]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x24), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e4fecb: JMP 0x00e4fecf
                    pc = 0x00e4fecf;
                    continue 'run;
                }
                0x00e4fecd => {
                    // 00e4fecd: FSTP ST0
                    {
                        let v = cpu.st(0);
                        cpu.set_st(0, v);
                    }
                    cpu.st_pop();
                    pc = 0x00e4fecf;
                    continue 'run;
                }
                0x00e4fecf => {
                    // 00e4fecf: MOV EAX,dword ptr [ESP + 0x18]
                    cpu.eax = e.mem.u32(cpu.esp.wrapping_add(0x18));
                    // 00e4fed3: FLD float ptr [EAX]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.eax)));
                    // 00e4fed5: FSTP float ptr [ESP + 0x1c]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x1c), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e4fed9: FLD float ptr [ESP + 0x1c]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x1c))));
                    // 00e4fedd: FLD float ptr [ESP + 0x28]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x28))));
                    // 00e4fee1: FCOMP
                    cpu.sw = u16::from(fcom_status(cpu.st(0), cpu.st(1))) << 8;
                    cpu.st_pop();
                    // 00e4fee3: FNSTSW AX
                    cpu.eax = (cpu.eax & 0xffff_0000) | u32::from(cpu.sw);
                    // 00e4fee5: TEST AH,0x5
                    cpu.test8((cpu.eax >> 8) & 0xff, 0x5);
                    // 00e4fee8: JP 0x00e4feee
                    if cpu.pf {
                        pc = 0x00e4feee;
                        continue 'run;
                    }
                    // 00e4feea: FST float ptr [ESP + 0x28]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x28), x87_store32(cpu.st(0)));
                    pc = 0x00e4feee;
                    continue 'run;
                }
                0x00e4feee => {
                    // 00e4feee: FLD float ptr [ESP + 0x24]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x24))));
                    // 00e4fef2: FCOMP
                    cpu.sw = u16::from(fcom_status(cpu.st(0), cpu.st(1))) << 8;
                    cpu.st_pop();
                    // 00e4fef4: FNSTSW AX
                    cpu.eax = (cpu.eax & 0xffff_0000) | u32::from(cpu.sw);
                    // 00e4fef6: TEST AH,0x41
                    cpu.test8((cpu.eax >> 8) & 0xff, 0x41);
                    // 00e4fef9: JNZ 0x00e4ff01
                    if !cpu.zf {
                        pc = 0x00e4ff01;
                        continue 'run;
                    }
                    // 00e4fefb: FSTP float ptr [ESP + 0x24]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x24), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e4feff: JMP 0x00e4ff03
                    pc = 0x00e4ff03;
                    continue 'run;
                }
                0x00e4ff01 => {
                    // 00e4ff01: FSTP ST0
                    {
                        let v = cpu.st(0);
                        cpu.set_st(0, v);
                    }
                    cpu.st_pop();
                    pc = 0x00e4ff03;
                    continue 'run;
                }
                0x00e4ff03 => {
                    // 00e4ff03: ADD dword ptr [ESP + 0x6c],ECX
                    {
                        let at = cpu.esp.wrapping_add(0x6c);
                        let v = cpu.add32(e.mem.u32(at), cpu.ecx);
                        e.mem.set_u32(at, v);
                    }
                    // 00e4ff07: ADD dword ptr [ESP + 0x68],ECX
                    {
                        let at = cpu.esp.wrapping_add(0x68);
                        let v = cpu.add32(e.mem.u32(at), cpu.ecx);
                        e.mem.set_u32(at, v);
                    }
                    // 00e4ff0b: ADD dword ptr [ESP + 0x18],ECX
                    {
                        let at = cpu.esp.wrapping_add(0x18);
                        let v = cpu.add32(e.mem.u32(at), cpu.ecx);
                        e.mem.set_u32(at, v);
                    }
                    // 00e4ff0f: ADD EBX,ECX
                    cpu.ebx = cpu.add32(cpu.ebx, cpu.ecx);
                    // 00e4ff11: SUB EDX,0x1
                    cpu.edx = cpu.sub32(cpu.edx, 0x1);
                    // 00e4ff14: JNZ 0x00e4fe37
                    if !cpu.zf {
                        pc = 0x00e4fe37;
                        continue 'run;
                    }
                    // 00e4ff1a: MOV EBX,dword ptr [EBP + 0x8]
                    cpu.ebx = e.mem.u32(cpu.ebp.wrapping_add(0x8));
                    pc = 0x00e4ff1d;
                    continue 'run;
                }
                0x00e4ff1d => {
                    // 00e4ff1d: CMP ESI,EDI
                    cpu.cmp32(cpu.esi, cpu.edi);
                    // 00e4ff1f: JNC 0x00e4ff76
                    if !cpu.cf {
                        pc = 0x00e4ff76;
                        continue 'run;
                    }
                    // 00e4ff21: MOV EAX,dword ptr [EBX + 0x4]
                    cpu.eax = e.mem.u32(cpu.ebx.wrapping_add(0x4));
                    // 00e4ff24: LEA ECX,[EAX + EAX*0x2]
                    cpu.ecx = cpu.eax.wrapping_add(cpu.eax.wrapping_mul(2));
                    // 00e4ff27: IMUL EAX,ESI
                    cpu.eax = cpu.imul32(cpu.eax, cpu.esi);
                    // 00e4ff2a: ADD EAX,dword ptr [ESP + 0x4c]
                    cpu.eax = cpu.add32(cpu.eax, e.mem.u32(cpu.esp.wrapping_add(0x4c)));
                    // 00e4ff2e: SHL ECX,0x4
                    cpu.ecx = cpu.shl32(cpu.ecx, 0x4);
                    // 00e4ff31: LEA EDX,[EAX + EAX*0x2]
                    cpu.edx = cpu.eax.wrapping_add(cpu.eax.wrapping_mul(2));
                    // 00e4ff34: MOV EAX,dword ptr [EBX]
                    cpu.eax = e.mem.u32(cpu.ebx);
                    // 00e4ff36: SHL EDX,0x4
                    cpu.edx = cpu.shl32(cpu.edx, 0x4);
                    // 00e4ff39: LEA EDX,[EDX + EAX*0x1 + 0x1c]
                    cpu.edx = cpu.edx.wrapping_add(cpu.eax).wrapping_add(0x1c);
                    // 00e4ff3d: SUB EDI,ESI
                    cpu.edi = cpu.sub32(cpu.edi, cpu.esi);
                    pc = 0x00e4ff3f;
                    continue 'run;
                }
                0x00e4ff3f => {
                    // 00e4ff3f: FLD float ptr [EDX]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.edx)));
                    // 00e4ff41: FSTP float ptr [ESP + 0x1c]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x1c), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e4ff45: FLD float ptr [ESP + 0x1c]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x1c))));
                    // 00e4ff49: FLD float ptr [ESP + 0x28]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x28))));
                    // 00e4ff4d: FCOMP
                    cpu.sw = u16::from(fcom_status(cpu.st(0), cpu.st(1))) << 8;
                    cpu.st_pop();
                    // 00e4ff4f: FNSTSW AX
                    cpu.eax = (cpu.eax & 0xffff_0000) | u32::from(cpu.sw);
                    // 00e4ff51: TEST AH,0x5
                    cpu.test8((cpu.eax >> 8) & 0xff, 0x5);
                    // 00e4ff54: JP 0x00e4ff5a
                    if cpu.pf {
                        pc = 0x00e4ff5a;
                        continue 'run;
                    }
                    // 00e4ff56: FST float ptr [ESP + 0x28]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x28), x87_store32(cpu.st(0)));
                    pc = 0x00e4ff5a;
                    continue 'run;
                }
                0x00e4ff5a => {
                    // 00e4ff5a: FLD float ptr [ESP + 0x24]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x24))));
                    // 00e4ff5e: FCOMP
                    cpu.sw = u16::from(fcom_status(cpu.st(0), cpu.st(1))) << 8;
                    cpu.st_pop();
                    // 00e4ff60: FNSTSW AX
                    cpu.eax = (cpu.eax & 0xffff_0000) | u32::from(cpu.sw);
                    // 00e4ff62: TEST AH,0x41
                    cpu.test8((cpu.eax >> 8) & 0xff, 0x41);
                    // 00e4ff65: JNZ 0x00e4ff6d
                    if !cpu.zf {
                        pc = 0x00e4ff6d;
                        continue 'run;
                    }
                    // 00e4ff67: FSTP float ptr [ESP + 0x24]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x24), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e4ff6b: JMP 0x00e4ff6f
                    pc = 0x00e4ff6f;
                    continue 'run;
                }
                0x00e4ff6d => {
                    // 00e4ff6d: FSTP ST0
                    {
                        let v = cpu.st(0);
                        cpu.set_st(0, v);
                    }
                    cpu.st_pop();
                    pc = 0x00e4ff6f;
                    continue 'run;
                }
                0x00e4ff6f => {
                    // 00e4ff6f: ADD EDX,ECX
                    cpu.edx = cpu.add32(cpu.edx, cpu.ecx);
                    // 00e4ff71: SUB EDI,0x1
                    cpu.edi = cpu.sub32(cpu.edi, 0x1);
                    // 00e4ff74: JNZ 0x00e4ff3f
                    if !cpu.zf {
                        pc = 0x00e4ff3f;
                        continue 'run;
                    }
                    pc = 0x00e4ff76;
                    continue 'run;
                }
                0x00e4ff76 => {
                    // 00e4ff76: FLD float ptr [ESP + 0x24]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x24))));
                    // 00e4ff7a: FCOMP double ptr [0x0101ffa0]
                    cpu.sw = u16::from(fcom_status(cpu.st(0), e.mem.f64(0x0101ffa0))) << 8;
                    cpu.st_pop();
                    // 00e4ff80: FNSTSW AX
                    cpu.eax = (cpu.eax & 0xffff_0000) | u32::from(cpu.sw);
                    // 00e4ff82: TEST AH,0x1
                    cpu.test8((cpu.eax >> 8) & 0xff, 0x1);
                    // 00e4ff85: JNZ 0x00e4ff8b
                    if !cpu.zf {
                        pc = 0x00e4ff8b;
                        continue 'run;
                    }
                    // 00e4ff87: MOV AL,0x1
                    cpu.eax = (cpu.eax & 0xffff_ff00) | (0x1 & 0xff);
                    // 00e4ff89: JMP 0x00e4ff8d
                    pc = 0x00e4ff8d;
                    continue 'run;
                }
                0x00e4ff8b => {
                    // 00e4ff8b: XOR AL,AL
                    cpu.eax = (cpu.eax & 0xffff_ff00)
                        | (cpu.xor32(cpu.eax & 0xff, cpu.eax & 0xff) & 0xff);
                    pc = 0x00e4ff8d;
                    continue 'run;
                }
                0x00e4ff8d => {
                    // 00e4ff8d: FLD float ptr [ESP + 0x28]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x28))));
                    // 00e4ff91: MOV byte ptr [ESP + 0x23],AL
                    e.mem
                        .set_u8(cpu.esp.wrapping_add(0x23), (cpu.eax & 0xff) as u8);
                    // 00e4ff95: FCOMP double ptr [0x0106b170]
                    cpu.sw = u16::from(fcom_status(cpu.st(0), e.mem.f64(0x0106b170))) << 8;
                    cpu.st_pop();
                    // 00e4ff9b: FNSTSW AX
                    cpu.eax = (cpu.eax & 0xffff_0000) | u32::from(cpu.sw);
                    // 00e4ff9d: TEST AH,0x41
                    cpu.test8((cpu.eax >> 8) & 0xff, 0x41);
                    // 00e4ffa0: JP 0x00e4ffa6
                    if cpu.pf {
                        pc = 0x00e4ffa6;
                        continue 'run;
                    }
                    // 00e4ffa2: MOV AL,0x1
                    cpu.eax = (cpu.eax & 0xffff_ff00) | (0x1 & 0xff);
                    // 00e4ffa4: JMP 0x00e4ffa8
                    pc = 0x00e4ffa8;
                    continue 'run;
                }
                0x00e4ffa6 => {
                    // 00e4ffa6: XOR AL,AL
                    cpu.eax = (cpu.eax & 0xffff_ff00)
                        | (cpu.xor32(cpu.eax & 0xff, cpu.eax & 0xff) & 0xff);
                    pc = 0x00e4ffa8;
                    continue 'run;
                }
                0x00e4ffa8 => {
                    // 00e4ffa8: MOV ESI,dword ptr [ESP + 0x30]
                    cpu.esi = e.mem.u32(cpu.esp.wrapping_add(0x30));
                    // 00e4ffac: MOV byte ptr [ESP + 0x22],AL
                    e.mem
                        .set_u8(cpu.esp.wrapping_add(0x22), (cpu.eax & 0xff) as u8);
                    pc = 0x00e4ffb0;
                    continue 'run;
                }
                0x00e4ffb0 => {
                    // 00e4ffb0: MOV ECX,dword ptr [ESP + 0x54]
                    cpu.ecx = e.mem.u32(cpu.esp.wrapping_add(0x54));
                    // 00e4ffb4: FXCH ST2
                    cpu.st_exchange(2);
                    // 00e4ffb6: MOVAPS XMM2,xmmword ptr [0x01017d10]
                    cpu.x[2] = load_vec(e, 0x01017d10);
                    // 00e4ffbd: FCOM ST3
                    cpu.sw = u16::from(fcom_status(cpu.st(0), cpu.st(3))) << 8;
                    // 00e4ffbf: ADD ESI,ECX
                    cpu.esi = cpu.add32(cpu.esi, cpu.ecx);
                    // 00e4ffc1: MOV dword ptr [ESP + 0x1c],ESI
                    e.mem.set_u32(cpu.esp.wrapping_add(0x1c), cpu.esi);
                    // 00e4ffc5: MOV EDX,0x15
                    cpu.edx = 0x15;
                    // 00e4ffca: FNSTSW AX
                    cpu.eax = (cpu.eax & 0xffff_0000) | u32::from(cpu.sw);
                    // 00e4ffcc: FLD float ptr [ESP + 0x50]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x50))));
                    // 00e4ffd0: TEST AH,0x41
                    cpu.test8((cpu.eax >> 8) & 0xff, 0x41);
                    // 00e4ffd3: JZ 0x00e4ffe8
                    if cpu.zf {
                        pc = 0x00e4ffe8;
                        continue 'run;
                    }
                    // 00e4ffd5: FCOM ST4
                    cpu.sw = u16::from(fcom_status(cpu.st(0), cpu.st(4))) << 8;
                    // 00e4ffd7: FNSTSW AX
                    cpu.eax = (cpu.eax & 0xffff_0000) | u32::from(cpu.sw);
                    // 00e4ffd9: TEST AH,0x41
                    cpu.test8((cpu.eax >> 8) & 0xff, 0x41);
                    // 00e4ffdc: JZ 0x00e4ffe8
                    if cpu.zf {
                        pc = 0x00e4ffe8;
                        continue 'run;
                    }
                    // 00e4ffde: MOV EDX,0x1d4
                    cpu.edx = 0x1d4;
                    // 00e4ffe3: JMP 0x00e500c1
                    pc = 0x00e500c1;
                    continue 'run;
                }
                0x00e4ffe8 => {
                    // 00e4ffe8: MOV ECX,dword ptr [EBX + 0x8]
                    cpu.ecx = e.mem.u32(cpu.ebx.wrapping_add(0x8));
                    // 00e4ffeb: TEST ECX,ECX
                    cpu.test32(cpu.ecx, cpu.ecx);
                    // 00e4ffed: JBE 0x00e500c1
                    if cpu.cf || cpu.zf {
                        pc = 0x00e500c1;
                        continue 'run;
                    }
                    // 00e4fff3: MOV EAX,dword ptr [ESP + 0x1c]
                    cpu.eax = e.mem.u32(cpu.esp.wrapping_add(0x1c));
                    // 00e4fff7: MOVAPS XMM4,xmmword ptr [EAX]
                    cpu.x[4] = load_vec(e, cpu.eax);
                    // 00e4fffa: MOV EAX,dword ptr [EBX + 0x4]
                    cpu.eax = e.mem.u32(cpu.ebx.wrapping_add(0x4));
                    // 00e4fffd: MOVSS XMM1,dword ptr [ESP + 0x48]
                    cpu.x[1] = scalar_vec(e, cpu.esp.wrapping_add(0x48));
                    // 00e50003: MOVSS XMM3,dword ptr [ESP + 0x50]
                    cpu.x[3] = scalar_vec(e, cpu.esp.wrapping_add(0x50));
                    // 00e50009: MOV EDI,dword ptr [EBX]
                    cpu.edi = e.mem.u32(cpu.ebx);
                    // 00e5000b: LEA EAX,[EAX + EAX*0x2]
                    cpu.eax = cpu.eax.wrapping_add(cpu.eax.wrapping_mul(2));
                    // 00e5000e: SHL EAX,0x4
                    cpu.eax = cpu.shl32(cpu.eax, 0x4);
                    // 00e50011: ADD EDI,dword ptr [ESP + 0x30]
                    cpu.edi = cpu.add32(cpu.edi, e.mem.u32(cpu.esp.wrapping_add(0x30)));
                    // 00e50015: SHUFPS XMM1,XMM1,0x0
                    cpu.x[1] = shufps(cpu.x[1], cpu.x[1], 0);
                    // 00e50019: MULPS XMM1,xmmword ptr [ESP + 0xf0]
                    cpu.x[1] = mulps(cpu.x[1], load_vec(e, cpu.esp.wrapping_add(0xf0)));
                    // 00e50021: SHUFPS XMM3,XMM3,0x0
                    cpu.x[3] = shufps(cpu.x[3], cpu.x[3], 0);
                    // 00e50025: MOV dword ptr [ESP + 0x18],EAX
                    e.mem.set_u32(cpu.esp.wrapping_add(0x18), cpu.eax);
                    // 00e50029: MOV dword ptr [ESP + 0x50],ECX
                    e.mem.set_u32(cpu.esp.wrapping_add(0x50), cpu.ecx);
                    pc = 0x00e5002d;
                    continue 'run;
                }
                0x00e5002d => {
                    // 00e5002d: MOVAPS XMM7,xmmword ptr [EDI]
                    cpu.x[7] = load_vec(e, cpu.edi);
                    // 00e50030: FCOM ST4
                    cpu.sw = u16::from(fcom_status(cpu.st(0), cpu.st(4))) << 8;
                    // 00e50032: MOVAPS XMM0,XMM4
                    cpu.x[0] = cpu.x[4];
                    // 00e50035: SUBPS XMM0,XMM7
                    cpu.x[0] = subps(cpu.x[0], cpu.x[7]);
                    // 00e50038: ANDPS XMM0,XMM2
                    cpu.x[0] = andps(cpu.x[0], cpu.x[2]);
                    // 00e5003b: MOVAPS XMM7,XMM0
                    cpu.x[7] = cpu.x[0];
                    // 00e5003e: FNSTSW AX
                    cpu.eax = (cpu.eax & 0xffff_0000) | u32::from(cpu.sw);
                    // 00e50040: CMPLEPS XMM7,XMM3
                    cpu.x[7] = cmpleps(cpu.x[7], cpu.x[3]);
                    // 00e50044: CMPLEPS XMM0,XMM1
                    cpu.x[0] = cmpleps(cpu.x[0], cpu.x[1]);
                    // 00e50048: MOVMSKPS ECX,XMM7
                    cpu.ecx = movmskps(cpu.x[7]);
                    // 00e5004b: MOVMSKPS ESI,XMM0
                    cpu.esi = movmskps(cpu.x[0]);
                    // 00e5004e: AND ECX,0x7
                    cpu.ecx = cpu.and32(cpu.ecx, 0x7);
                    // 00e50051: AND ESI,0x7
                    cpu.esi = cpu.and32(cpu.esi, 0x7);
                    // 00e50054: TEST AH,0x41
                    cpu.test8((cpu.eax >> 8) & 0xff, 0x41);
                    // 00e50057: JNZ 0x00e5007b
                    if !cpu.zf {
                        pc = 0x00e5007b;
                        continue 'run;
                    }
                    // 00e50059: CMP ECX,0x7
                    cpu.cmp32(cpu.ecx, 0x7);
                    // 00e5005c: JZ 0x00e5007b
                    if cpu.zf {
                        pc = 0x00e5007b;
                        continue 'run;
                    }
                    // 00e5005e: NOT ECX
                    cpu.ecx = !cpu.ecx;
                    // 00e50060: AND ECX,0x7
                    cpu.ecx = cpu.and32(cpu.ecx, 0x7);
                    // 00e50063: MOVZX CX,byte ptr [ECX + 0x11be718]
                    cpu.ecx = (cpu.ecx & 0xffff_0000)
                        | (u32::from(e.mem.u8(cpu.ecx.wrapping_add(0x011be718))) & 0xffff);
                    // 00e5006b: AND EDX,0xfffc
                    cpu.edx = cpu.and32(cpu.edx, 0xfffc);
                    // 00e50071: SHL CX,0x5
                    cpu.ecx = (cpu.ecx & 0xffff_0000) | (cpu.shl32(cpu.ecx & 0xffff, 0x5) & 0xffff);
                    // 00e50075: OR CX,DX
                    cpu.ecx = (cpu.ecx & 0xffff_0000)
                        | (cpu.or32(cpu.ecx & 0xffff, cpu.edx & 0xffff) & 0xffff);
                    // 00e50078: MOVZX EDX,CX
                    cpu.edx = cpu.ecx & 0xffff;
                    pc = 0x00e5007b;
                    continue 'run;
                }
                0x00e5007b => {
                    // 00e5007b: FXCH
                    cpu.st_exchange(1);
                    // 00e5007d: FCOM ST4
                    cpu.sw = u16::from(fcom_status(cpu.st(0), cpu.st(4))) << 8;
                    // 00e5007f: FNSTSW AX
                    cpu.eax = (cpu.eax & 0xffff_0000) | u32::from(cpu.sw);
                    // 00e50081: TEST AH,0x41
                    cpu.test8((cpu.eax >> 8) & 0xff, 0x41);
                    // 00e50084: JNZ 0x00e500a8
                    if !cpu.zf {
                        pc = 0x00e500a8;
                        continue 'run;
                    }
                    // 00e50086: CMP ESI,0x7
                    cpu.cmp32(cpu.esi, 0x7);
                    // 00e50089: JZ 0x00e500a8
                    if cpu.zf {
                        pc = 0x00e500a8;
                        continue 'run;
                    }
                    // 00e5008b: NOT ESI
                    cpu.esi = !cpu.esi;
                    // 00e5008d: AND ESI,0x7
                    cpu.esi = cpu.and32(cpu.esi, 0x7);
                    // 00e50090: MOVZX AX,byte ptr [ESI + 0x11be718]
                    cpu.eax = (cpu.eax & 0xffff_0000)
                        | (u32::from(e.mem.u8(cpu.esi.wrapping_add(0x011be718))) & 0xffff);
                    // 00e50098: AND EDX,0xfffc
                    cpu.edx = cpu.and32(cpu.edx, 0xfffc);
                    // 00e5009e: SHL AX,0x5
                    cpu.eax = (cpu.eax & 0xffff_0000) | (cpu.shl32(cpu.eax & 0xffff, 0x5) & 0xffff);
                    // 00e500a2: OR AX,DX
                    cpu.eax = (cpu.eax & 0xffff_0000)
                        | (cpu.or32(cpu.eax & 0xffff, cpu.edx & 0xffff) & 0xffff);
                    // 00e500a5: MOVZX EDX,AX
                    cpu.edx = cpu.eax & 0xffff;
                    pc = 0x00e500a8;
                    continue 'run;
                }
                0x00e500a8 => {
                    // 00e500a8: ADD EDI,dword ptr [ESP + 0x18]
                    cpu.edi = cpu.add32(cpu.edi, e.mem.u32(cpu.esp.wrapping_add(0x18)));
                    // 00e500ac: FXCH
                    cpu.st_exchange(1);
                    // 00e500ae: SUB dword ptr [ESP + 0x50],0x1
                    {
                        let at = cpu.esp.wrapping_add(0x50);
                        let v = cpu.sub32(e.mem.u32(at), 0x1);
                        e.mem.set_u32(at, v);
                    }
                    // 00e500b3: JNZ 0x00e5002d
                    if !cpu.zf {
                        pc = 0x00e5002d;
                        continue 'run;
                    }
                    // 00e500b9: MOVAPS XMM4,xmmword ptr [ESP + 0x100]
                    cpu.x[4] = load_vec(e, cpu.esp.wrapping_add(0x100));
                    pc = 0x00e500c1;
                    continue 'run;
                }
                0x00e500c1 => {
                    // 00e500c1: FXCH ST4
                    cpu.st_exchange(4);
                    // 00e500c3: FCOM float ptr [ESP + 0x64]
                    cpu.sw = u16::from(fcom_status(
                        cpu.st(0),
                        x87_m32(e.mem.u32(cpu.esp.wrapping_add(0x64))),
                    )) << 8;
                    // 00e500c7: FNSTSW AX
                    cpu.eax = (cpu.eax & 0xffff_0000) | u32::from(cpu.sw);
                    // 00e500c9: TEST AH,0x5
                    cpu.test8((cpu.eax >> 8) & 0xff, 0x5);
                    // 00e500cc: JP 0x00e501ce
                    if cpu.pf {
                        pc = 0x00e501ce;
                        continue 'run;
                    }
                    // 00e500d2: CMP byte ptr [ESP + 0x17],0x0
                    cpu.cmp8(u32::from(e.mem.u8(cpu.esp.wrapping_add(0x17))), 0x0);
                    // 00e500d7: JZ 0x00e500ee
                    if cpu.zf {
                        pc = 0x00e500ee;
                        continue 'run;
                    }
                    // 00e500d9: CMP byte ptr [ESP + 0x23],0x0
                    cpu.cmp8(u32::from(e.mem.u8(cpu.esp.wrapping_add(0x23))), 0x0);
                    // 00e500de: JNZ 0x00e500e7
                    if !cpu.zf {
                        pc = 0x00e500e7;
                        continue 'run;
                    }
                    // 00e500e0: CMP byte ptr [ESP + 0x22],0x0
                    cpu.cmp8(u32::from(e.mem.u8(cpu.esp.wrapping_add(0x22))), 0x0);
                    // 00e500e5: JZ 0x00e500ee
                    if cpu.zf {
                        pc = 0x00e500ee;
                        continue 'run;
                    }
                    pc = 0x00e500e7;
                    continue 'run;
                }
                0x00e500e7 => {
                    // 00e500e7: MOV byte ptr [ESP + 0x17],0x1
                    e.mem.set_u8(cpu.esp.wrapping_add(0x17), 0x1);
                    // 00e500ec: JMP 0x00e500f3
                    pc = 0x00e500f3;
                    continue 'run;
                }
                0x00e500ee => {
                    // 00e500ee: MOV byte ptr [ESP + 0x17],0x0
                    e.mem.set_u8(cpu.esp.wrapping_add(0x17), 0x0);
                    pc = 0x00e500f3;
                    continue 'run;
                }
                0x00e500f3 => {
                    // 00e500f3: MOV ECX,dword ptr [EBX + 0x8]
                    cpu.ecx = e.mem.u32(cpu.ebx.wrapping_add(0x8));
                    // 00e500f6: TEST ECX,ECX
                    cpu.test32(cpu.ecx, cpu.ecx);
                    // 00e500f8: JBE 0x00e5018a
                    if cpu.cf || cpu.zf {
                        pc = 0x00e5018a;
                        continue 'run;
                    }
                    // 00e500fe: MOV EAX,dword ptr [ESP + 0x1c]
                    cpu.eax = e.mem.u32(cpu.esp.wrapping_add(0x1c));
                    // 00e50102: MOVAPS XMM3,xmmword ptr [EAX + 0x10]
                    cpu.x[3] = load_vec(e, cpu.eax.wrapping_add(0x10));
                    // 00e50106: MOV EAX,dword ptr [EBX + 0x4]
                    cpu.eax = e.mem.u32(cpu.ebx.wrapping_add(0x4));
                    // 00e50109: MOVSS XMM1,dword ptr [ESP + 0x64]
                    cpu.x[1] = scalar_vec(e, cpu.esp.wrapping_add(0x64));
                    // 00e5010f: MOV EDI,dword ptr [ESP + 0x30]
                    cpu.edi = e.mem.u32(cpu.esp.wrapping_add(0x30));
                    // 00e50113: LEA ESI,[EAX + EAX*0x2]
                    cpu.esi = cpu.eax.wrapping_add(cpu.eax.wrapping_mul(2));
                    // 00e50116: MOV EAX,dword ptr [EBX]
                    cpu.eax = e.mem.u32(cpu.ebx);
                    // 00e50118: SHUFPS XMM1,XMM1,0x0
                    cpu.x[1] = shufps(cpu.x[1], cpu.x[1], 0);
                    // 00e5011c: SHL ESI,0x4
                    cpu.esi = cpu.shl32(cpu.esi, 0x4);
                    // 00e5011f: LEA EDI,[EDI + EAX*0x1 + 0x10]
                    cpu.edi = cpu.edi.wrapping_add(cpu.eax).wrapping_add(0x10);
                    // 00e50123: MOV dword ptr [ESP + 0x64],ECX
                    e.mem.set_u32(cpu.esp.wrapping_add(0x64), cpu.ecx);
                    pc = 0x00e50127;
                    continue 'run;
                }
                0x00e50127 => {
                    // 00e50127: MOVAPS XMM0,xmmword ptr [EDI]
                    cpu.x[0] = load_vec(e, cpu.edi);
                    // 00e5012a: MOVAPS XMM7,XMM0
                    cpu.x[7] = cpu.x[0];
                    // 00e5012d: SUBPS XMM7,XMM3
                    cpu.x[7] = subps(cpu.x[7], cpu.x[3]);
                    // 00e50130: ANDPS XMM7,XMM2
                    cpu.x[7] = andps(cpu.x[7], cpu.x[2]);
                    // 00e50133: CMPLEPS XMM7,XMM1
                    cpu.x[7] = cmpleps(cpu.x[7], cpu.x[1]);
                    // 00e50137: MOVMSKPS ECX,XMM7
                    cpu.ecx = movmskps(cpu.x[7]);
                    // 00e5013a: AND ECX,0xf
                    cpu.ecx = cpu.and32(cpu.ecx, 0xf);
                    // 00e5013d: CMP CL,0xf
                    cpu.cmp8(cpu.ecx & 0xff, 0xf);
                    // 00e50140: JZ 0x00e50181
                    if cpu.zf {
                        pc = 0x00e50181;
                        continue 'run;
                    }
                    // 00e50142: MOVAPS XMM7,XMM3
                    cpu.x[7] = cpu.x[3];
                    // 00e50145: SUBPS XMM7,XMM0
                    cpu.x[7] = subps(cpu.x[7], cpu.x[0]);
                    // 00e50148: ANDPS XMM7,XMM2
                    cpu.x[7] = andps(cpu.x[7], cpu.x[2]);
                    // 00e5014b: CMPLEPS XMM7,XMM1
                    cpu.x[7] = cmpleps(cpu.x[7], cpu.x[1]);
                    // 00e5014f: MOVMSKPS EAX,XMM7
                    cpu.eax = movmskps(cpu.x[7]);
                    // 00e50152: AND EAX,0xf
                    cpu.eax = cpu.and32(cpu.eax, 0xf);
                    // 00e50155: CMP byte ptr [ESP + 0x17],0x0
                    cpu.cmp8(u32::from(e.mem.u8(cpu.esp.wrapping_add(0x17))), 0x0);
                    // 00e5015a: JZ 0x00e5015f
                    if cpu.zf {
                        pc = 0x00e5015f;
                        continue 'run;
                    }
                    // 00e5015c: OR EAX,0x8
                    cpu.eax = cpu.or32(cpu.eax, 0x8);
                    pc = 0x00e5015f;
                    continue 'run;
                }
                0x00e5015f => {
                    // 00e5015f: CMP EAX,0xf
                    cpu.cmp32(cpu.eax, 0xf);
                    // 00e50162: JZ 0x00e50181
                    if cpu.zf {
                        pc = 0x00e50181;
                        continue 'run;
                    }
                    // 00e50164: NOT EAX
                    cpu.eax = !cpu.eax;
                    // 00e50166: AND EAX,0xf
                    cpu.eax = cpu.and32(cpu.eax, 0xf);
                    // 00e50169: MOVZX AX,byte ptr [EAX + 0x11be718]
                    cpu.eax = (cpu.eax & 0xffff_0000)
                        | (u32::from(e.mem.u8(cpu.eax.wrapping_add(0x011be718))) & 0xffff);
                    // 00e50171: AND EDX,0xfff3
                    cpu.edx = cpu.and32(cpu.edx, 0xfff3);
                    // 00e50177: SHL AX,0x9
                    cpu.eax = (cpu.eax & 0xffff_0000) | (cpu.shl32(cpu.eax & 0xffff, 0x9) & 0xffff);
                    // 00e5017b: OR AX,DX
                    cpu.eax = (cpu.eax & 0xffff_0000)
                        | (cpu.or32(cpu.eax & 0xffff, cpu.edx & 0xffff) & 0xffff);
                    // 00e5017e: MOVZX EDX,AX
                    cpu.edx = cpu.eax & 0xffff;
                    pc = 0x00e50181;
                    continue 'run;
                }
                0x00e50181 => {
                    // 00e50181: ADD EDI,ESI
                    cpu.edi = cpu.add32(cpu.edi, cpu.esi);
                    // 00e50183: SUB dword ptr [ESP + 0x64],0x1
                    {
                        let at = cpu.esp.wrapping_add(0x64);
                        let v = cpu.sub32(e.mem.u32(at), 0x1);
                        e.mem.set_u32(at, v);
                    }
                    // 00e50188: JNZ 0x00e50127
                    if !cpu.zf {
                        pc = 0x00e50127;
                        continue 'run;
                    }
                    pc = 0x00e5018a;
                    continue 'run;
                }
                0x00e5018a => {
                    // 00e5018a: CMP byte ptr [ESP + 0x17],0x0
                    cpu.cmp8(u32::from(e.mem.u8(cpu.esp.wrapping_add(0x17))), 0x0);
                    // 00e5018f: JZ 0x00e501da
                    if cpu.zf {
                        pc = 0x00e501da;
                        continue 'run;
                    }
                    // 00e50191: CMP byte ptr [ESP + 0x23],0x0
                    cpu.cmp8(u32::from(e.mem.u8(cpu.esp.wrapping_add(0x23))), 0x0);
                    // 00e50196: JZ 0x00e501ac
                    if cpu.zf {
                        pc = 0x00e501ac;
                        continue 'run;
                    }
                    // 00e50198: FLD float ptr [0x010ec620]
                    cpu.st_push(x87_load32(e.mem.u32(0x010ec620)));
                    // 00e5019e: MOV ECX,dword ptr [ESP + 0x1c]
                    cpu.ecx = e.mem.u32(cpu.esp.wrapping_add(0x1c));
                    // 00e501a2: FSTP float ptr [ECX + 0x1c]
                    e.mem
                        .set_u32(cpu.ecx.wrapping_add(0x1c), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e501a5: MOVAPS XMM2,xmmword ptr [0x01017d10]
                    cpu.x[2] = load_vec(e, 0x01017d10);
                    pc = 0x00e501ac;
                    continue 'run;
                }
                0x00e501ac => {
                    // 00e501ac: CMP byte ptr [ESP + 0x22],0x0
                    cpu.cmp8(u32::from(e.mem.u8(cpu.esp.wrapping_add(0x22))), 0x0);
                    // 00e501b1: JZ 0x00e501da
                    if cpu.zf {
                        pc = 0x00e501da;
                        continue 'run;
                    }
                    // 00e501b3: FLD float ptr [0x0104a494]
                    cpu.st_push(x87_load32(e.mem.u32(0x0104a494)));
                    // 00e501b9: MOV EAX,dword ptr [ESP + 0x54]
                    cpu.eax = e.mem.u32(cpu.esp.wrapping_add(0x54));
                    // 00e501bd: MOV ECX,dword ptr [ESP + 0x30]
                    cpu.ecx = e.mem.u32(cpu.esp.wrapping_add(0x30));
                    // 00e501c1: FSTP float ptr [ECX + EAX*0x1 + 0x1c]
                    e.mem.set_u32(
                        cpu.ecx.wrapping_add(cpu.eax).wrapping_add(0x1c),
                        x87_store32(cpu.st(0)),
                    );
                    cpu.st_pop();
                    // 00e501c5: MOVAPS XMM2,xmmword ptr [0x01017d10]
                    cpu.x[2] = load_vec(e, 0x01017d10);
                    // 00e501cc: JMP 0x00e501da
                    pc = 0x00e501da;
                    continue 'run;
                }
                0x00e501ce => {
                    // 00e501ce: AND EDX,0xfff3
                    cpu.edx = cpu.and32(cpu.edx, 0xfff3);
                    // 00e501d4: OR EDX,0x1e00
                    cpu.edx = cpu.or32(cpu.edx, 0x1e00);
                    pc = 0x00e501da;
                    continue 'run;
                }
                0x00e501da => {
                    // 00e501da: FLD float ptr [ESP + 0x2c]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x2c))));
                    // 00e501de: FCOM
                    cpu.sw = u16::from(fcom_status(cpu.st(0), cpu.st(1))) << 8;
                    // 00e501e0: FNSTSW AX
                    cpu.eax = (cpu.eax & 0xffff_0000) | u32::from(cpu.sw);
                    // 00e501e2: TEST AH,0x41
                    cpu.test8((cpu.eax >> 8) & 0xff, 0x41);
                    // 00e501e5: JNZ 0x00e50254
                    if !cpu.zf {
                        pc = 0x00e50254;
                        continue 'run;
                    }
                    // 00e501e7: MOV EDI,dword ptr [EBX + 0x8]
                    cpu.edi = e.mem.u32(cpu.ebx.wrapping_add(0x8));
                    // 00e501ea: TEST EDI,EDI
                    cpu.test32(cpu.edi, cpu.edi);
                    // 00e501ec: JBE 0x00e50260
                    if cpu.cf || cpu.zf {
                        pc = 0x00e50260;
                        continue 'run;
                    }
                    // 00e501ee: MOV EAX,dword ptr [ESP + 0x1c]
                    cpu.eax = e.mem.u32(cpu.esp.wrapping_add(0x1c));
                    // 00e501f2: MOVAPS XMM1,xmmword ptr [EAX + 0x20]
                    cpu.x[1] = load_vec(e, cpu.eax.wrapping_add(0x20));
                    // 00e501f6: MOV EAX,dword ptr [EBX + 0x4]
                    cpu.eax = e.mem.u32(cpu.ebx.wrapping_add(0x4));
                    // 00e501f9: MOV ESI,dword ptr [ESP + 0x30]
                    cpu.esi = e.mem.u32(cpu.esp.wrapping_add(0x30));
                    // 00e501fd: MOVSS XMM0,dword ptr [ESP + 0x2c]
                    cpu.x[0] = scalar_vec(e, cpu.esp.wrapping_add(0x2c));
                    // 00e50203: LEA ECX,[EAX + EAX*0x2]
                    cpu.ecx = cpu.eax.wrapping_add(cpu.eax.wrapping_mul(2));
                    // 00e50206: MOV EAX,dword ptr [EBX]
                    cpu.eax = e.mem.u32(cpu.ebx);
                    // 00e50208: SHUFPS XMM0,XMM0,0x0
                    cpu.x[0] = shufps(cpu.x[0], cpu.x[0], 0);
                    // 00e5020c: SHL ECX,0x4
                    cpu.ecx = cpu.shl32(cpu.ecx, 0x4);
                    // 00e5020f: LEA ESI,[ESI + EAX*0x1 + 0x20]
                    cpu.esi = cpu.esi.wrapping_add(cpu.eax).wrapping_add(0x20);
                    pc = 0x00e50213;
                    continue 'run;
                }
                0x00e50213 => {
                    // 00e50213: MOVAPS XMM3,xmmword ptr [ESI]
                    cpu.x[3] = load_vec(e, cpu.esi);
                    // 00e50216: MOVAPS XMM7,XMM1
                    cpu.x[7] = cpu.x[1];
                    // 00e50219: SUBPS XMM7,XMM3
                    cpu.x[7] = subps(cpu.x[7], cpu.x[3]);
                    // 00e5021c: ANDPS XMM7,XMM2
                    cpu.x[7] = andps(cpu.x[7], cpu.x[2]);
                    // 00e5021f: CMPLEPS XMM7,XMM0
                    cpu.x[7] = cmpleps(cpu.x[7], cpu.x[0]);
                    // 00e50223: MOVMSKPS EAX,XMM7
                    cpu.eax = movmskps(cpu.x[7]);
                    // 00e50226: AND EAX,0x7
                    cpu.eax = cpu.and32(cpu.eax, 0x7);
                    // 00e50229: CMP EAX,0x7
                    cpu.cmp32(cpu.eax, 0x7);
                    // 00e5022c: JZ 0x00e5024b
                    if cpu.zf {
                        pc = 0x00e5024b;
                        continue 'run;
                    }
                    // 00e5022e: NOT EAX
                    cpu.eax = !cpu.eax;
                    // 00e50230: AND EAX,0x7
                    cpu.eax = cpu.and32(cpu.eax, 0x7);
                    // 00e50233: MOVZX AX,byte ptr [EAX + 0x11be718]
                    cpu.eax = (cpu.eax & 0xffff_0000)
                        | (u32::from(e.mem.u8(cpu.eax.wrapping_add(0x011be718))) & 0xffff);
                    // 00e5023b: AND EDX,0xffcf
                    cpu.edx = cpu.and32(cpu.edx, 0xffcf);
                    // 00e50241: SHL AX,0xc
                    cpu.eax = (cpu.eax & 0xffff_0000) | (cpu.shl32(cpu.eax & 0xffff, 0xc) & 0xffff);
                    // 00e50245: OR AX,DX
                    cpu.eax = (cpu.eax & 0xffff_0000)
                        | (cpu.or32(cpu.eax & 0xffff, cpu.edx & 0xffff) & 0xffff);
                    // 00e50248: MOVZX EDX,AX
                    cpu.edx = cpu.eax & 0xffff;
                    pc = 0x00e5024b;
                    continue 'run;
                }
                0x00e5024b => {
                    // 00e5024b: ADD ESI,ECX
                    cpu.esi = cpu.add32(cpu.esi, cpu.ecx);
                    // 00e5024d: SUB EDI,0x1
                    cpu.edi = cpu.sub32(cpu.edi, 0x1);
                    // 00e50250: JNZ 0x00e50213
                    if !cpu.zf {
                        pc = 0x00e50213;
                        continue 'run;
                    }
                    // 00e50252: JMP 0x00e50260
                    pc = 0x00e50260;
                    continue 'run;
                }
                0x00e50254 => {
                    // 00e50254: AND EDX,0xffcf
                    cpu.edx = cpu.and32(cpu.edx, 0xffcf);
                    // 00e5025a: OR EDX,0xe000
                    cpu.edx = cpu.or32(cpu.edx, 0xe000);
                    pc = 0x00e50260;
                    continue 'run;
                }
                0x00e50260 => {
                    // 00e50260: MOV EDI,dword ptr [EBP + 0xc]
                    cpu.edi = e.mem.u32(cpu.ebp.wrapping_add(0xc));
                    // 00e50263: MOV ESI,dword ptr [ESP + 0x4c]
                    cpu.esi = e.mem.u32(cpu.esp.wrapping_add(0x4c));
                    // 00e50267: MOV CL,DL
                    cpu.ecx = (cpu.ecx & 0xffff_ff00) | (cpu.edx & 0xff & 0xff);
                    // 00e50269: AND CL,0x3
                    cpu.ecx = (cpu.ecx & 0xffff_ff00) | (cpu.and32(cpu.ecx & 0xff, 0x3) & 0xff);
                    // 00e5026c: MOV word ptr [EDI + ESI*0x2],DX
                    e.mem.set_u16(
                        cpu.edi.wrapping_add(cpu.esi.wrapping_mul(2)),
                        (cpu.edx & 0xffff) as u16,
                    );
                    // 00e50270: CMP CL,0x1
                    cpu.cmp8(cpu.ecx & 0xff, 0x1);
                    // 00e50273: JNZ 0x00e5039a
                    if !cpu.zf {
                        pc = 0x00e5039a;
                        continue 'run;
                    }
                    // 00e50279: MOVAPS XMM2,xmmword ptr [0x01017d10]
                    cpu.x[2] = load_vec(e, 0x01017d10);
                    // 00e50280: FXCH ST5
                    cpu.st_exchange(5);
                    // 00e50282: MOVAPS XMM1,xmmword ptr [0x01267e30]
                    cpu.x[1] = load_vec(e, 0x01267e30);
                    // 00e50289: FCOM
                    cpu.sw = u16::from(fcom_status(cpu.st(0), cpu.st(1))) << 8;
                    // 00e5028b: FNSTSW AX
                    cpu.eax = (cpu.eax & 0xffff_0000) | u32::from(cpu.sw);
                    // 00e5028d: TEST AH,0x41
                    cpu.test8((cpu.eax >> 8) & 0xff, 0x41);
                    // 00e50290: JNZ 0x00e502e0
                    if !cpu.zf {
                        pc = 0x00e502e0;
                        continue 'run;
                    }
                    // 00e50292: MOV EAX,dword ptr [ESP + 0x1c]
                    cpu.eax = e.mem.u32(cpu.esp.wrapping_add(0x1c));
                    // 00e50296: FMUL ST3
                    {
                        let v = x87_mul(cpu.st(0), cpu.st(3));
                        cpu.set_st(0, v);
                    }
                    // 00e50298: MOVAPS XMM3,xmmword ptr [EAX]
                    cpu.x[3] = load_vec(e, cpu.eax);
                    // 00e5029b: SUBPS XMM3,XMM1
                    cpu.x[3] = subps(cpu.x[3], cpu.x[1]);
                    // 00e5029e: FSTP float ptr [ESP + 0x2c]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x2c), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e502a2: ANDPS XMM3,XMM2
                    cpu.x[3] = andps(cpu.x[3], cpu.x[2]);
                    // 00e502a5: MOVSS XMM0,dword ptr [ESP + 0x2c]
                    cpu.x[0] = scalar_vec(e, cpu.esp.wrapping_add(0x2c));
                    // 00e502ab: SHUFPS XMM0,XMM0,0x0
                    cpu.x[0] = shufps(cpu.x[0], cpu.x[0], 0);
                    // 00e502af: CMPLEPS XMM3,XMM0
                    cpu.x[3] = cmpleps(cpu.x[3], cpu.x[0]);
                    // 00e502b3: MOVMSKPS ECX,XMM3
                    cpu.ecx = movmskps(cpu.x[3]);
                    // 00e502b6: AND ECX,0x7
                    cpu.ecx = cpu.and32(cpu.ecx, 0x7);
                    // 00e502b9: CMP CL,0x7
                    cpu.cmp8(cpu.ecx & 0xff, 0x7);
                    // 00e502bc: JNZ 0x00e502e2
                    if !cpu.zf {
                        pc = 0x00e502e2;
                        continue 'run;
                    }
                    // 00e502be: FSTP ST1
                    {
                        let v = cpu.st(0);
                        cpu.set_st(1, v);
                    }
                    cpu.st_pop();
                    // 00e502c0: FXCH
                    cpu.st_exchange(1);
                    pc = 0x00e502c2;
                    continue 'run;
                }
                0x00e502c2 => {
                    // 00e502c2: MOV EAX,dword ptr [ESP + 0x54]
                    cpu.eax = e.mem.u32(cpu.esp.wrapping_add(0x54));
                    // 00e502c6: AND EDX,0xfffe
                    cpu.edx = cpu.and32(cpu.edx, 0xfffe);
                    // 00e502cc: OR EDX,0x2
                    cpu.edx = cpu.or32(cpu.edx, 0x2);
                    // 00e502cf: MOV word ptr [EDI + ESI*0x2],DX
                    e.mem.set_u16(
                        cpu.edi.wrapping_add(cpu.esi.wrapping_mul(2)),
                        (cpu.edx & 0xffff) as u16,
                    );
                    // 00e502d3: MOV EDX,dword ptr [ESP + 0x30]
                    cpu.edx = e.mem.u32(cpu.esp.wrapping_add(0x30));
                    // 00e502d7: MOVAPS xmmword ptr [EDX + EAX*0x1],XMM4
                    store_vec(e, cpu.edx.wrapping_add(cpu.eax), cpu.x[4]);
                    // 00e502db: JMP 0x00e503a0
                    pc = 0x00e503a0;
                    continue 'run;
                }
                0x00e502e0 => {
                    // 00e502e0: FSTP ST0
                    {
                        let v = cpu.st(0);
                        cpu.set_st(0, v);
                    }
                    cpu.st_pop();
                    pc = 0x00e502e2;
                    continue 'run;
                }
                0x00e502e2 => {
                    // 00e502e2: FCOM
                    cpu.sw = u16::from(fcom_status(cpu.st(0), cpu.st(1))) << 8;
                    // 00e502e4: FNSTSW AX
                    cpu.eax = (cpu.eax & 0xffff_0000) | u32::from(cpu.sw);
                    // 00e502e6: TEST AH,0x5
                    cpu.test8((cpu.eax >> 8) & 0xff, 0x5);
                    // 00e502e9: JP 0x00e5039c
                    if cpu.pf {
                        pc = 0x00e5039c;
                        continue 'run;
                    }
                    // 00e502ef: MOVAPS XMM0,xmmword ptr [ESP + 0xf0]
                    cpu.x[0] = load_vec(e, cpu.esp.wrapping_add(0xf0));
                    // 00e502f7: FSTP ST2
                    {
                        let v = cpu.st(0);
                        cpu.set_st(2, v);
                    }
                    cpu.st_pop();
                    // 00e502f9: ANDPS XMM0,XMM2
                    cpu.x[0] = andps(cpu.x[0], cpu.x[2]);
                    // 00e502fc: MOVAPS xmmword ptr [ESP + 0xa0],XMM0
                    store_vec(e, cpu.esp.wrapping_add(0xa0), cpu.x[0]);
                    // 00e50304: FLD float ptr [ESP + 0xa0]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0xa0))));
                    // 00e5030b: FLD float ptr [ESP + 0xa4]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0xa4))));
                    // 00e50312: FCOM
                    cpu.sw = u16::from(fcom_status(cpu.st(0), cpu.st(1))) << 8;
                    // 00e50314: FNSTSW AX
                    cpu.eax = (cpu.eax & 0xffff_0000) | u32::from(cpu.sw);
                    // 00e50316: TEST AH,0x41
                    cpu.test8((cpu.eax >> 8) & 0xff, 0x41);
                    // 00e50319: JNZ 0x00e5033b
                    if !cpu.zf {
                        pc = 0x00e5033b;
                        continue 'run;
                    }
                    // 00e5031b: FSTP ST1
                    {
                        let v = cpu.st(0);
                        cpu.set_st(1, v);
                    }
                    cpu.st_pop();
                    // 00e5031d: FLD float ptr [ESP + 0xa8]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0xa8))));
                    // 00e50324: FCOMPP
                    cpu.sw = u16::from(fcom_status(cpu.st(0), cpu.st(1))) << 8;
                    cpu.st_pop();
                    cpu.st_pop();
                    // 00e50326: FNSTSW AX
                    cpu.eax = (cpu.eax & 0xffff_0000) | u32::from(cpu.sw);
                    // 00e50328: TEST AH,0x41
                    cpu.test8((cpu.eax >> 8) & 0xff, 0x41);
                    // 00e5032b: JNZ 0x00e50334
                    if !cpu.zf {
                        pc = 0x00e50334;
                        continue 'run;
                    }
                    // 00e5032d: MOV EAX,0x2
                    cpu.eax = 0x2;
                    // 00e50332: JMP 0x00e50356
                    pc = 0x00e50356;
                    continue 'run;
                }
                0x00e50334 => {
                    // 00e50334: MOV EAX,0x1
                    cpu.eax = 0x1;
                    // 00e50339: JMP 0x00e50356
                    pc = 0x00e50356;
                    continue 'run;
                }
                0x00e5033b => {
                    // 00e5033b: FSTP ST0
                    {
                        let v = cpu.st(0);
                        cpu.set_st(0, v);
                    }
                    cpu.st_pop();
                    // 00e5033d: FLD float ptr [ESP + 0xa8]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0xa8))));
                    // 00e50344: FCOMPP
                    cpu.sw = u16::from(fcom_status(cpu.st(0), cpu.st(1))) << 8;
                    cpu.st_pop();
                    cpu.st_pop();
                    // 00e50346: FNSTSW AX
                    cpu.eax = (cpu.eax & 0xffff_0000) | u32::from(cpu.sw);
                    // 00e50348: TEST AH,0x41
                    cpu.test8((cpu.eax >> 8) & 0xff, 0x41);
                    // 00e5034b: JNZ 0x00e50354
                    if !cpu.zf {
                        pc = 0x00e50354;
                        continue 'run;
                    }
                    // 00e5034d: MOV EAX,0x2
                    cpu.eax = 0x2;
                    // 00e50352: JMP 0x00e50356
                    pc = 0x00e50356;
                    continue 'run;
                }
                0x00e50354 => {
                    // 00e50354: XOR EAX,EAX
                    cpu.eax = cpu.xor32(cpu.eax, cpu.eax);
                    pc = 0x00e50356;
                    continue 'run;
                }
                0x00e50356 => {
                    // 00e50356: FLD float ptr [ESP + EAX*0x4 + 0xf0]
                    cpu.st_push(x87_load32(
                        e.mem.u32(
                            cpu.esp
                                .wrapping_add(cpu.eax.wrapping_mul(4))
                                .wrapping_add(0xf0),
                        ),
                    ));
                    // 00e5035d: MOV EAX,dword ptr [ESP + 0x1c]
                    cpu.eax = e.mem.u32(cpu.esp.wrapping_add(0x1c));
                    // 00e50361: FLD double ptr [0x01011588]
                    cpu.st_push(e.mem.f64(0x01011588));
                    // 00e50367: MOVAPS XMM3,xmmword ptr [EAX]
                    cpu.x[3] = load_vec(e, cpu.eax);
                    // 00e5036a: FMUL ST1
                    {
                        let v = x87_mul(cpu.st(1), cpu.st(0));
                        cpu.set_st(1, v);
                    }
                    // 00e5036c: SUBPS XMM3,XMM1
                    cpu.x[3] = subps(cpu.x[3], cpu.x[1]);
                    // 00e5036f: FXCH
                    cpu.st_exchange(1);
                    // 00e50371: ANDPS XMM3,XMM2
                    cpu.x[3] = andps(cpu.x[3], cpu.x[2]);
                    // 00e50374: FMULP ST2
                    {
                        let v = x87_mul(cpu.st(2), cpu.st(0));
                        cpu.set_st(2, v);
                    }
                    cpu.st_pop();
                    // 00e50376: FXCH
                    cpu.st_exchange(1);
                    // 00e50378: FSTP float ptr [ESP + 0x2c]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x2c), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5037c: MOVSS XMM0,dword ptr [ESP + 0x2c]
                    cpu.x[0] = scalar_vec(e, cpu.esp.wrapping_add(0x2c));
                    // 00e50382: SHUFPS XMM0,XMM0,0x0
                    cpu.x[0] = shufps(cpu.x[0], cpu.x[0], 0);
                    // 00e50386: CMPLEPS XMM3,XMM0
                    cpu.x[3] = cmpleps(cpu.x[3], cpu.x[0]);
                    // 00e5038a: MOVMSKPS ECX,XMM3
                    cpu.ecx = movmskps(cpu.x[3]);
                    // 00e5038d: AND ECX,0x7
                    cpu.ecx = cpu.and32(cpu.ecx, 0x7);
                    // 00e50390: CMP CL,0x7
                    cpu.cmp8(cpu.ecx & 0xff, 0x7);
                    // 00e50393: JNZ 0x00e503a0
                    if !cpu.zf {
                        pc = 0x00e503a0;
                        continue 'run;
                    }
                    // 00e50395: JMP 0x00e502c2
                    pc = 0x00e502c2;
                    continue 'run;
                }
                0x00e5039a => {
                    // 00e5039a: FSTP ST5
                    {
                        let v = cpu.st(0);
                        cpu.set_st(5, v);
                    }
                    cpu.st_pop();
                    pc = 0x00e5039c;
                    continue 'run;
                }
                0x00e5039c => {
                    // 00e5039c: FSTP ST1
                    {
                        let v = cpu.st(0);
                        cpu.set_st(1, v);
                    }
                    cpu.st_pop();
                    // 00e5039e: FXCH
                    cpu.st_exchange(1);
                    pc = 0x00e503a0;
                    continue 'run;
                }
                0x00e503a0 => {
                    // 00e503a0: MOVZX ECX,word ptr [EDI + ESI*0x2]
                    cpu.ecx = u32::from(e.mem.u16(cpu.edi.wrapping_add(cpu.esi.wrapping_mul(2))));
                    // 00e503a4: MOV DL,CL
                    cpu.edx = (cpu.edx & 0xffff_ff00) | (cpu.ecx & 0xff & 0xff);
                    // 00e503a6: AND DL,0xc
                    cpu.edx = (cpu.edx & 0xffff_ff00) | (cpu.and32(cpu.edx & 0xff, 0xc) & 0xff);
                    // 00e503a9: CMP DL,0x4
                    cpu.cmp8(cpu.edx & 0xff, 0x4);
                    // 00e503ac: JNZ 0x00e503ed
                    if !cpu.zf {
                        pc = 0x00e503ed;
                        continue 'run;
                    }
                    // 00e503ae: MOV EAX,dword ptr [ESP + 0x1c]
                    cpu.eax = e.mem.u32(cpu.esp.wrapping_add(0x1c));
                    // 00e503b2: FLD float ptr [EAX + 0x1c]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.eax.wrapping_add(0x1c))));
                    // 00e503b5: FABS
                    {
                        let v = cpu.st(0).abs();
                        cpu.set_st(0, v);
                    }
                    // 00e503b7: FSTP float ptr [ESP + 0x2c]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x2c), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e503bb: FLD float ptr [ESP + 0x2c]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x2c))));
                    // 00e503bf: FCOMP float ptr [0x01012068]
                    cpu.sw = u16::from(fcom_status(cpu.st(0), x87_m32(e.mem.u32(0x01012068)))) << 8;
                    cpu.st_pop();
                    // 00e503c5: FNSTSW AX
                    cpu.eax = (cpu.eax & 0xffff_0000) | u32::from(cpu.sw);
                    // 00e503c7: TEST AH,0x44
                    cpu.test8((cpu.eax >> 8) & 0xff, 0x44);
                    // 00e503ca: JP 0x00e503ed
                    if cpu.pf {
                        pc = 0x00e503ed;
                        continue 'run;
                    }
                    // 00e503cc: MOV EDX,dword ptr [ESP + 0x30]
                    cpu.edx = e.mem.u32(cpu.esp.wrapping_add(0x30));
                    // 00e503d0: AND ECX,0xfffb
                    cpu.ecx = cpu.and32(cpu.ecx, 0xfffb);
                    // 00e503d6: OR ECX,0x8
                    cpu.ecx = cpu.or32(cpu.ecx, 0x8);
                    // 00e503d9: MOV word ptr [EDI + ESI*0x2],CX
                    e.mem.set_u16(
                        cpu.edi.wrapping_add(cpu.esi.wrapping_mul(2)),
                        (cpu.ecx & 0xffff) as u16,
                    );
                    // 00e503dd: MOV ECX,dword ptr [ESP + 0x54]
                    cpu.ecx = e.mem.u32(cpu.esp.wrapping_add(0x54));
                    // 00e503e1: MOVAPS XMM0,xmmword ptr [0x010c71b0]
                    cpu.x[0] = load_vec(e, 0x010c71b0);
                    // 00e503e8: MOVAPS xmmword ptr [EDX + ECX*0x1 + 0x10],XMM0
                    store_vec(
                        e,
                        cpu.edx.wrapping_add(cpu.ecx).wrapping_add(0x10),
                        cpu.x[0],
                    );
                    pc = 0x00e503ed;
                    continue 'run;
                }
                0x00e503ed => {
                    // 00e503ed: MOVZX EAX,word ptr [EDI + ESI*0x2]
                    cpu.eax = u32::from(e.mem.u16(cpu.edi.wrapping_add(cpu.esi.wrapping_mul(2))));
                    // 00e503f1: MOV CL,AL
                    cpu.ecx = (cpu.ecx & 0xffff_ff00) | (cpu.eax & 0xff & 0xff);
                    // 00e503f3: AND CL,0x30
                    cpu.ecx = (cpu.ecx & 0xffff_ff00) | (cpu.and32(cpu.ecx & 0xff, 0x30) & 0xff);
                    // 00e503f6: CMP CL,0x10
                    cpu.cmp8(cpu.ecx & 0xff, 0x10);
                    // 00e503f9: JNZ 0x00e50458
                    if !cpu.zf {
                        pc = 0x00e50458;
                        continue 'run;
                    }
                    // 00e503fb: MOV EDX,dword ptr [ESP + 0x1c]
                    cpu.edx = e.mem.u32(cpu.esp.wrapping_add(0x1c));
                    // 00e503ff: FMUL ST3
                    {
                        let v = x87_mul(cpu.st(3), cpu.st(0));
                        cpu.set_st(3, v);
                    }
                    // 00e50401: MOVAPS XMM0,xmmword ptr [0x010c7170]
                    cpu.x[0] = load_vec(e, 0x010c7170);
                    // 00e50408: FXCH ST3
                    cpu.st_exchange(3);
                    // 00e5040a: MOVAPS XMM1,xmmword ptr [EDX + 0x20]
                    cpu.x[1] = load_vec(e, cpu.edx.wrapping_add(0x20));
                    // 00e5040e: SUBPS XMM1,XMM0
                    cpu.x[1] = subps(cpu.x[1], cpu.x[0]);
                    // 00e50411: FSTP float ptr [ESP + 0x2c]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x2c), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e50415: MOVSS XMM0,dword ptr [ESP + 0x2c]
                    cpu.x[0] = scalar_vec(e, cpu.esp.wrapping_add(0x2c));
                    // 00e5041b: ANDPS XMM1,xmmword ptr [0x01017d10]
                    cpu.x[1] = andps(cpu.x[1], load_vec(e, 0x01017d10));
                    // 00e50422: SHUFPS XMM0,XMM0,0x0
                    cpu.x[0] = shufps(cpu.x[0], cpu.x[0], 0);
                    // 00e50426: CMPLEPS XMM1,XMM0
                    cpu.x[1] = cmpleps(cpu.x[1], cpu.x[0]);
                    // 00e5042a: MOVMSKPS ECX,XMM1
                    cpu.ecx = movmskps(cpu.x[1]);
                    // 00e5042d: AND ECX,0x7
                    cpu.ecx = cpu.and32(cpu.ecx, 0x7);
                    // 00e50430: CMP CL,0x7
                    cpu.cmp8(cpu.ecx & 0xff, 0x7);
                    // 00e50433: JNZ 0x00e5045a
                    if !cpu.zf {
                        pc = 0x00e5045a;
                        continue 'run;
                    }
                    // 00e50435: MOV EDX,dword ptr [ESP + 0x54]
                    cpu.edx = e.mem.u32(cpu.esp.wrapping_add(0x54));
                    // 00e50439: AND EAX,0xffef
                    cpu.eax = cpu.and32(cpu.eax, 0xffef);
                    // 00e5043e: OR EAX,0x20
                    cpu.eax = cpu.or32(cpu.eax, 0x20);
                    // 00e50441: MOV word ptr [EDI + ESI*0x2],AX
                    e.mem.set_u16(
                        cpu.edi.wrapping_add(cpu.esi.wrapping_mul(2)),
                        (cpu.eax & 0xffff) as u16,
                    );
                    // 00e50445: MOV EAX,dword ptr [ESP + 0x30]
                    cpu.eax = e.mem.u32(cpu.esp.wrapping_add(0x30));
                    // 00e50449: MOVSS XMM0,XMM5
                    {
                        let s = cpu.x[5][0];
                        cpu.x[0][0] = s;
                    }
                    // 00e5044d: SHUFPS XMM0,XMM0,0x0
                    cpu.x[0] = shufps(cpu.x[0], cpu.x[0], 0);
                    // 00e50451: MOVAPS xmmword ptr [EAX + EDX*0x1 + 0x20],XMM0
                    store_vec(
                        e,
                        cpu.eax.wrapping_add(cpu.edx).wrapping_add(0x20),
                        cpu.x[0],
                    );
                    // 00e50456: JMP 0x00e5045a
                    pc = 0x00e5045a;
                    continue 'run;
                }
                0x00e50458 => {
                    // 00e50458: FSTP ST3
                    {
                        let v = cpu.st(0);
                        cpu.set_st(3, v);
                    }
                    cpu.st_pop();
                    pc = 0x00e5045a;
                    continue 'run;
                }
                0x00e5045a => {
                    // 00e5045a: INC ESI
                    cpu.esi = cpu.inc32(cpu.esi);
                    // 00e5045b: MOV dword ptr [ESP + 0x4c],ESI
                    e.mem.set_u32(cpu.esp.wrapping_add(0x4c), cpu.esi);
                    // 00e5045f: CMP ESI,dword ptr [EBX + 0x4]
                    cpu.cmp32(cpu.esi, e.mem.u32(cpu.ebx.wrapping_add(0x4)));
                    // 00e50462: JC 0x00e4fa53
                    if cpu.cf {
                        pc = 0x00e4fa53;
                        continue 'run;
                    }
                    // 00e50468: FSTP ST2
                    {
                        let v = cpu.st(0);
                        cpu.set_st(2, v);
                    }
                    cpu.st_pop();
                    // 00e5046a: FSTP ST0
                    {
                        let v = cpu.st(0);
                        cpu.set_st(0, v);
                    }
                    cpu.st_pop();
                    pc = 0x00e5046c;
                    continue 'run;
                }
                0x00e5046c => {
                    // 00e5046c: CMP dword ptr [EBX + 0x4],0x0
                    cpu.cmp32(e.mem.u32(cpu.ebx.wrapping_add(0x4)), 0x0);
                    // 00e50470: MOV EAX,dword ptr [EBP + 0x10]
                    cpu.eax = e.mem.u32(cpu.ebp.wrapping_add(0x10));
                    // 00e50473: MOV dword ptr [ESP + 0x18],0x0
                    e.mem.set_u32(cpu.esp.wrapping_add(0x18), 0x0);
                    // 00e5047b: JBE 0x00e50567
                    if cpu.cf || cpu.zf {
                        pc = 0x00e50567;
                        continue 'run;
                    }
                    // 00e50481: MOV ESI,dword ptr [ESP + 0x54]
                    cpu.esi = e.mem.u32(cpu.esp.wrapping_add(0x54));
                    // 00e50485: ADD ESI,0x8
                    cpu.esi = cpu.add32(cpu.esi, 0x8);
                    pc = 0x00e50488;
                    continue 'run;
                }
                0x00e50488 => {
                    // 00e50488: MOV EDX,dword ptr [ESP + 0x18]
                    cpu.edx = e.mem.u32(cpu.esp.wrapping_add(0x18));
                    // 00e5048c: MOV ECX,dword ptr [EBP + 0xc]
                    cpu.ecx = e.mem.u32(cpu.ebp.wrapping_add(0xc));
                    // 00e5048f: MOVZX ECX,word ptr [ECX + EDX*0x2]
                    cpu.ecx = u32::from(e.mem.u16(cpu.ecx.wrapping_add(cpu.edx.wrapping_mul(2))));
                    // 00e50493: MOV DX,CX
                    cpu.edx = (cpu.edx & 0xffff_0000) | (cpu.ecx & 0xffff & 0xffff);
                    // 00e50496: SHR DX,0x6
                    cpu.edx = (cpu.edx & 0xffff_0000) | (cpu.shr32(cpu.edx & 0xffff, 0x6) & 0xffff);
                    // 00e5049a: MOVZX EDI,DX
                    cpu.edi = cpu.edx & 0xffff;
                    // 00e5049d: MOV DL,CL
                    cpu.edx = (cpu.edx & 0xffff_ff00) | (cpu.ecx & 0xff & 0xff);
                    // 00e5049f: AND DL,0x3
                    cpu.edx = (cpu.edx & 0xffff_ff00) | (cpu.and32(cpu.edx & 0xff, 0x3) & 0xff);
                    // 00e504a2: CMP DL,0x2
                    cpu.cmp8(cpu.edx & 0xff, 0x2);
                    // 00e504a5: JZ 0x00e504d3
                    if cpu.zf {
                        pc = 0x00e504d3;
                        continue 'run;
                    }
                    // 00e504a7: MOVZX EDX,DI
                    cpu.edx = cpu.edi & 0xffff;
                    // 00e504aa: TEST DL,0x4
                    cpu.test8(cpu.edx & 0xff, 0x4);
                    // 00e504ad: JNZ 0x00e504b8
                    if !cpu.zf {
                        pc = 0x00e504b8;
                        continue 'run;
                    }
                    // 00e504af: FLD float ptr [ESI + -0x8]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esi.wrapping_sub(0x8))));
                    // 00e504b2: ADD EAX,0x4
                    cpu.eax = cpu.add32(cpu.eax, 0x4);
                    // 00e504b5: FSTP float ptr [EAX + -0x4]
                    e.mem
                        .set_u32(cpu.eax.wrapping_sub(0x4), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    pc = 0x00e504b8;
                    continue 'run;
                }
                0x00e504b8 => {
                    // 00e504b8: TEST DL,0x2
                    cpu.test8(cpu.edx & 0xff, 0x2);
                    // 00e504bb: JNZ 0x00e504c6
                    if !cpu.zf {
                        pc = 0x00e504c6;
                        continue 'run;
                    }
                    // 00e504bd: FLD float ptr [ESI + -0x4]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esi.wrapping_sub(0x4))));
                    // 00e504c0: ADD EAX,0x4
                    cpu.eax = cpu.add32(cpu.eax, 0x4);
                    // 00e504c3: FSTP float ptr [EAX + -0x4]
                    e.mem
                        .set_u32(cpu.eax.wrapping_sub(0x4), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    pc = 0x00e504c6;
                    continue 'run;
                }
                0x00e504c6 => {
                    // 00e504c6: TEST DL,0x1
                    cpu.test8(cpu.edx & 0xff, 0x1);
                    // 00e504c9: JNZ 0x00e504d3
                    if !cpu.zf {
                        pc = 0x00e504d3;
                        continue 'run;
                    }
                    // 00e504cb: FLD float ptr [ESI]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esi)));
                    // 00e504cd: ADD EAX,0x4
                    cpu.eax = cpu.add32(cpu.eax, 0x4);
                    // 00e504d0: FSTP float ptr [EAX + -0x4]
                    e.mem
                        .set_u32(cpu.eax.wrapping_sub(0x4), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    pc = 0x00e504d3;
                    continue 'run;
                }
                0x00e504d3 => {
                    // 00e504d3: MOV DL,CL
                    cpu.edx = (cpu.edx & 0xffff_ff00) | (cpu.ecx & 0xff & 0xff);
                    // 00e504d5: AND DL,0xc
                    cpu.edx = (cpu.edx & 0xffff_ff00) | (cpu.and32(cpu.edx & 0xff, 0xc) & 0xff);
                    // 00e504d8: CMP DL,0x8
                    cpu.cmp8(cpu.edx & 0xff, 0x8);
                    // 00e504db: JZ 0x00e50518
                    if cpu.zf {
                        pc = 0x00e50518;
                        continue 'run;
                    }
                    // 00e504dd: MOVZX EDX,DI
                    cpu.edx = cpu.edi & 0xffff;
                    // 00e504e0: TEST DL,0x40
                    cpu.test8(cpu.edx & 0xff, 0x40);
                    // 00e504e3: JNZ 0x00e504ee
                    if !cpu.zf {
                        pc = 0x00e504ee;
                        continue 'run;
                    }
                    // 00e504e5: FLD float ptr [ESI + 0x8]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esi.wrapping_add(0x8))));
                    // 00e504e8: ADD EAX,0x4
                    cpu.eax = cpu.add32(cpu.eax, 0x4);
                    // 00e504eb: FSTP float ptr [EAX + -0x4]
                    e.mem
                        .set_u32(cpu.eax.wrapping_sub(0x4), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    pc = 0x00e504ee;
                    continue 'run;
                }
                0x00e504ee => {
                    // 00e504ee: TEST DL,0x20
                    cpu.test8(cpu.edx & 0xff, 0x20);
                    // 00e504f1: JNZ 0x00e504fc
                    if !cpu.zf {
                        pc = 0x00e504fc;
                        continue 'run;
                    }
                    // 00e504f3: FLD float ptr [ESI + 0xc]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esi.wrapping_add(0xc))));
                    // 00e504f6: ADD EAX,0x4
                    cpu.eax = cpu.add32(cpu.eax, 0x4);
                    // 00e504f9: FSTP float ptr [EAX + -0x4]
                    e.mem
                        .set_u32(cpu.eax.wrapping_sub(0x4), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    pc = 0x00e504fc;
                    continue 'run;
                }
                0x00e504fc => {
                    // 00e504fc: TEST DL,0x10
                    cpu.test8(cpu.edx & 0xff, 0x10);
                    // 00e504ff: JNZ 0x00e5050a
                    if !cpu.zf {
                        pc = 0x00e5050a;
                        continue 'run;
                    }
                    // 00e50501: FLD float ptr [ESI + 0x10]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esi.wrapping_add(0x10))));
                    // 00e50504: ADD EAX,0x4
                    cpu.eax = cpu.add32(cpu.eax, 0x4);
                    // 00e50507: FSTP float ptr [EAX + -0x4]
                    e.mem
                        .set_u32(cpu.eax.wrapping_sub(0x4), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    pc = 0x00e5050a;
                    continue 'run;
                }
                0x00e5050a => {
                    // 00e5050a: TEST DL,0x8
                    cpu.test8(cpu.edx & 0xff, 0x8);
                    // 00e5050d: JNZ 0x00e50518
                    if !cpu.zf {
                        pc = 0x00e50518;
                        continue 'run;
                    }
                    // 00e5050f: FLD float ptr [ESI + 0x14]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esi.wrapping_add(0x14))));
                    // 00e50512: ADD EAX,0x4
                    cpu.eax = cpu.add32(cpu.eax, 0x4);
                    // 00e50515: FSTP float ptr [EAX + -0x4]
                    e.mem
                        .set_u32(cpu.eax.wrapping_sub(0x4), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    pc = 0x00e50518;
                    continue 'run;
                }
                0x00e50518 => {
                    // 00e50518: AND CL,0x30
                    cpu.ecx = (cpu.ecx & 0xffff_ff00) | (cpu.and32(cpu.ecx & 0xff, 0x30) & 0xff);
                    // 00e5051b: CMP CL,0x20
                    cpu.cmp8(cpu.ecx & 0xff, 0x20);
                    // 00e5051e: JZ 0x00e50552
                    if cpu.zf {
                        pc = 0x00e50552;
                        continue 'run;
                    }
                    // 00e50520: MOVZX ECX,DI
                    cpu.ecx = cpu.edi & 0xffff;
                    // 00e50523: TEST ECX,0x200
                    cpu.test32(cpu.ecx, 0x200);
                    // 00e50529: JNZ 0x00e50534
                    if !cpu.zf {
                        pc = 0x00e50534;
                        continue 'run;
                    }
                    // 00e5052b: FLD float ptr [ESI + 0x18]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esi.wrapping_add(0x18))));
                    // 00e5052e: ADD EAX,0x4
                    cpu.eax = cpu.add32(cpu.eax, 0x4);
                    // 00e50531: FSTP float ptr [EAX + -0x4]
                    e.mem
                        .set_u32(cpu.eax.wrapping_sub(0x4), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    pc = 0x00e50534;
                    continue 'run;
                }
                0x00e50534 => {
                    // 00e50534: TEST ECX,0x100
                    cpu.test32(cpu.ecx, 0x100);
                    // 00e5053a: JNZ 0x00e50545
                    if !cpu.zf {
                        pc = 0x00e50545;
                        continue 'run;
                    }
                    // 00e5053c: FLD float ptr [ESI + 0x1c]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esi.wrapping_add(0x1c))));
                    // 00e5053f: ADD EAX,0x4
                    cpu.eax = cpu.add32(cpu.eax, 0x4);
                    // 00e50542: FSTP float ptr [EAX + -0x4]
                    e.mem
                        .set_u32(cpu.eax.wrapping_sub(0x4), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    pc = 0x00e50545;
                    continue 'run;
                }
                0x00e50545 => {
                    // 00e50545: TEST CL,CL
                    cpu.test8(cpu.ecx & 0xff, cpu.ecx & 0xff);
                    // 00e50547: JS 0x00e50552
                    if cpu.sf {
                        pc = 0x00e50552;
                        continue 'run;
                    }
                    // 00e50549: FLD float ptr [ESI + 0x20]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esi.wrapping_add(0x20))));
                    // 00e5054c: ADD EAX,0x4
                    cpu.eax = cpu.add32(cpu.eax, 0x4);
                    // 00e5054f: FSTP float ptr [EAX + -0x4]
                    e.mem
                        .set_u32(cpu.eax.wrapping_sub(0x4), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    pc = 0x00e50552;
                    continue 'run;
                }
                0x00e50552 => {
                    // 00e50552: MOV ECX,dword ptr [ESP + 0x18]
                    cpu.ecx = e.mem.u32(cpu.esp.wrapping_add(0x18));
                    // 00e50556: INC ECX
                    cpu.ecx = cpu.inc32(cpu.ecx);
                    // 00e50557: ADD ESI,0x30
                    cpu.esi = cpu.add32(cpu.esi, 0x30);
                    // 00e5055a: MOV dword ptr [ESP + 0x18],ECX
                    e.mem.set_u32(cpu.esp.wrapping_add(0x18), cpu.ecx);
                    // 00e5055e: CMP ECX,dword ptr [EBX + 0x4]
                    cpu.cmp32(cpu.ecx, e.mem.u32(cpu.ebx.wrapping_add(0x4)));
                    // 00e50561: JC 0x00e50488
                    if cpu.cf {
                        pc = 0x00e50488;
                        continue 'run;
                    }
                    pc = 0x00e50567;
                    continue 'run;
                }
                0x00e50567 => {
                    // 00e50567: CMP byte ptr [EBP + 0x14],0x0
                    cpu.cmp8(u32::from(e.mem.u8(cpu.ebp.wrapping_add(0x14))), 0x0);
                    // 00e5056b: JZ 0x00e505fb
                    if cpu.zf {
                        pc = 0x00e505fb;
                        continue 'run;
                    }
                    // 00e50571: MOV ECX,dword ptr [EBX + 0x4]
                    cpu.ecx = e.mem.u32(cpu.ebx.wrapping_add(0x4));
                    // 00e50574: FSTP float ptr [ESP + 0xec]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0xec), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5057b: MOV EDX,dword ptr [EBP + 0xc]
                    cpu.edx = e.mem.u32(cpu.ebp.wrapping_add(0xc));
                    // 00e5057e: XOR EAX,EAX
                    cpu.eax = cpu.xor32(cpu.eax, cpu.eax);
                    // 00e50580: MOV dword ptr [ESP + 0xb8],EAX
                    e.mem.set_u32(cpu.esp.wrapping_add(0xb8), cpu.eax);
                    // 00e50587: MOV dword ptr [ESP + 0xbc],EAX
                    e.mem.set_u32(cpu.esp.wrapping_add(0xbc), cpu.eax);
                    // 00e5058e: MOV dword ptr [ESP + 0xc0],EAX
                    e.mem.set_u32(cpu.esp.wrapping_add(0xc0), cpu.eax);
                    // 00e50595: MOV dword ptr [ESP + 0xc4],EAX
                    e.mem.set_u32(cpu.esp.wrapping_add(0xc4), cpu.eax);
                    // 00e5059c: MOV dword ptr [ESP + 0xc8],EAX
                    e.mem.set_u32(cpu.esp.wrapping_add(0xc8), cpu.eax);
                    // 00e505a3: MOV dword ptr [ESP + 0xcc],EAX
                    e.mem.set_u32(cpu.esp.wrapping_add(0xcc), cpu.eax);
                    // 00e505aa: MOV dword ptr [ESP + 0xd0],EAX
                    e.mem.set_u32(cpu.esp.wrapping_add(0xd0), cpu.eax);
                    // 00e505b1: MOV dword ptr [ESP + 0xd4],EAX
                    e.mem.set_u32(cpu.esp.wrapping_add(0xd4), cpu.eax);
                    // 00e505b8: MOV dword ptr [ESP + 0xd8],EAX
                    e.mem.set_u32(cpu.esp.wrapping_add(0xd8), cpu.eax);
                    // 00e505bf: MOV dword ptr [ESP + 0xdc],EAX
                    e.mem.set_u32(cpu.esp.wrapping_add(0xdc), cpu.eax);
                    // 00e505c6: MOV dword ptr [ESP + 0xe0],EAX
                    e.mem.set_u32(cpu.esp.wrapping_add(0xe0), cpu.eax);
                    // 00e505cd: MOV dword ptr [ESP + 0xe4],EAX
                    e.mem.set_u32(cpu.esp.wrapping_add(0xe4), cpu.eax);
                    // 00e505d4: MOV dword ptr [ESP + 0xe8],EAX
                    e.mem.set_u32(cpu.esp.wrapping_add(0xe8), cpu.eax);
                    // 00e505db: LEA EAX,[ESP + 0xb8]
                    cpu.eax = cpu.esp.wrapping_add(0xb8);
                    // 00e505e2: PUSH EAX
                    cpu.push(e, cpu.eax);
                    // 00e505e3: PUSH ECX
                    cpu.push(e, cpu.ecx);
                    // 00e505e4: PUSH EDX
                    cpu.push(e, cpu.edx);
                    // 00e505e5: CALL 0x00e4ead0
                    e.call(
                        ANALYSE_FLAGS,
                        &args![
                            e.mem.u32(cpu.esp.wrapping_add(0)),
                            e.mem.u32(cpu.esp.wrapping_add(4)),
                            e.mem.u32(cpu.esp.wrapping_add(8))
                        ],
                    );
                    // 00e505ea: ADD ESP,0xc
                    cpu.esp = cpu.add32(cpu.esp, 0xc);
                    // 00e505ed: LEA ECX,[ESP + 0xb8]
                    cpu.ecx = cpu.esp.wrapping_add(0xb8);
                    // 00e505f4: CALL 0x00e4e660
                    e.call(DISPLAY_REPORT, &args![cpu.ecx]);
                    // 00e505f9: JMP 0x00e505fd
                    pc = 0x00e505fd;
                    continue 'run;
                }
                0x00e505fb => {
                    // 00e505fb: FSTP ST0
                    {
                        let v = cpu.st(0);
                        cpu.set_st(0, v);
                    }
                    cpu.st_pop();
                    pc = 0x00e505fd;
                    continue 'run;
                }
                0x00e505fd => {
                    // 00e505fd: MOV ESI,dword ptr [ESP + 0x60]
                    cpu.esi = e.mem.u32(cpu.esp.wrapping_add(0x60));
                    // 00e50601: CMP ESI,dword ptr [ESP + 0x54]
                    cpu.cmp32(cpu.esi, e.mem.u32(cpu.esp.wrapping_add(0x54)));
                    // 00e50605: JNZ 0x00e5060f
                    if !cpu.zf {
                        pc = 0x00e5060f;
                        continue 'run;
                    }
                    // 00e50607: MOV dword ptr [ESP + 0x58],0x0
                    e.mem.set_u32(cpu.esp.wrapping_add(0x58), 0x0);
                    pc = 0x00e5060f;
                    continue 'run;
                }
                0x00e5060f => {
                    // 00e5060f: CALL 0x00c85750
                    cpu.eax = e.call(MEMORY_ROUTER, &args![]).u32();
                    // 00e50614: MOV dword ptr [EAX],ESI
                    e.mem.set_u32(cpu.eax, cpu.esi);
                    // 00e50616: CALL 0x00c85750
                    cpu.eax = e.call(MEMORY_ROUTER, &args![]).u32();
                    // 00e5061b: MOV ECX,dword ptr [EAX + 0x10]
                    cpu.ecx = e.mem.u32(cpu.eax.wrapping_add(0x10));
                    // 00e5061e: MOV EAX,dword ptr [ESP + 0x5c]
                    cpu.eax = e.mem.u32(cpu.esp.wrapping_add(0x5c));
                    // 00e50622: MOV dword ptr [ESP + 0x58],0x0
                    e.mem.set_u32(cpu.esp.wrapping_add(0x58), 0x0);
                    // 00e5062a: TEST EAX,EAX
                    cpu.test32(cpu.eax, cpu.eax);
                    // 00e5062c: JS 0x00e50646
                    if cpu.sf {
                        pc = 0x00e50646;
                        continue 'run;
                    }
                    // 00e5062e: MOV EDX,dword ptr [ECX]
                    cpu.edx = e.mem.u32(cpu.ecx);
                    // 00e50630: MOV EDX,dword ptr [EDX + 0x10]
                    cpu.edx = e.mem.u32(cpu.edx.wrapping_add(0x10));
                    // 00e50633: AND EAX,0x3fffffff
                    cpu.eax = cpu.and32(cpu.eax, 0x3fffffff);
                    // 00e50638: LEA EAX,[EAX + EAX*0x2]
                    cpu.eax = cpu.eax.wrapping_add(cpu.eax.wrapping_mul(2));
                    // 00e5063b: SHL EAX,0x4
                    cpu.eax = cpu.shl32(cpu.eax, 0x4);
                    // 00e5063e: PUSH EAX
                    cpu.push(e, cpu.eax);
                    // 00e5063f: MOV EAX,dword ptr [ESP + 0x58]
                    cpu.eax = e.mem.u32(cpu.esp.wrapping_add(0x58));
                    // 00e50643: PUSH EAX
                    cpu.push(e, cpu.eax);
                    // 00e50644: CALL EDX
                    e.call(
                        cpu.edx,
                        &args![
                            cpu.ecx,
                            e.mem.u32(cpu.esp.wrapping_add(0)),
                            e.mem.u32(cpu.esp.wrapping_add(4))
                        ],
                    );
                    cpu.esp = cpu.esp.wrapping_add(8);
                    pc = 0x00e50646;
                    continue 'run;
                }
                0x00e50646 => {
                    // 00e50646: POP EDI
                    cpu.edi = cpu.pop(e);
                    // 00e50647: POP ESI
                    cpu.esi = cpu.pop(e);
                    // 00e50648: POP EBX
                    cpu.ebx = cpu.pop(e);
                    // 00e50649: MOV ESP,EBP
                    cpu.esp = cpu.ebp;
                    // 00e5064b: POP EBP
                    cpu.ebp = cpu.pop(e);
                    // 00e5064c: RET
                    break 'run;
                }
                other => panic!("jump to {other:#x} outside the recovered function"),
            }
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e4_f910, fn_00e4f910(u32, u32, u32, u8)),
        entry!(0x00e5_0650, fn_00e50650(u32, u32, u32)),
        entry!(0x00e5_0920, fn_00e50920(u32, u32, u32, u32, u8)),
        entry!(0x00e5_0ca0, fn_00e50ca0(Ptr)),
        entry!(0x00e5_0d90, fn_00e50d90(f32, u32, u32, i32)),
        entry!(0x00e5_0e50, fn_00e50e50(f32, u32, u32, i32)),
        entry!(0x00e5_0f10, fn_00e50f10(u32, i32, i32, u32)),
        entry!(0x00e5_1070, fn_00e51070(u32, u32, i32)),
        entry!(0x00e5_1140, fn_00e51140(u32, u32)),
        entry!(0x00e5_1390, fn_00e51390(u32, u32, f32, u8)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// The vtable slot of the allocator's free the doubles stand in for.
    const FREE_DOUBLE: u32 = 0x0ff0_0010;

    /// The words of the executable's data the functions read, at their addresses (floats,
    /// doubles and vectors; see the notes in the doc comments).
    const EXE_WORDS: [(u32, &[u32]); 18] = [
        // Doubles 0.5 and 2.0 (`00e50920`'s midpoint and threshold factors).
        (0x0101_1588, &[0, 0x3fe0_0000, 0, 0x4000_0000]),
        (0x0101_1d78, &[0]),
        // The float 1.0.
        (0x0101_2068, &[0x3f80_0000]),
        // The float 0.5.
        (0x0101_6248, &[0x3f00_0000]),
        // The float 3.0.
        (0x0101_7718, &[0x4040_0000]),
        // The magnitude mask.
        (0x0101_7d10, &[0x7fff_ffff; 4]),
        // The doubles 0.1 and -0.1.
        (0x0101_ffa0, &[0xa000_0000, 0x3fb9_9999]),
        (0x0106_b170, &[0xa000_0000, 0xbfb9_9999]),
        // The float -2.0.
        (0x0104_a494, &[0xc000_0000]),
        // The floats the minimum and maximum start from, and `1.1920929e-07`.
        (0x0104_da48, &[0x7f7f_ffee, 0x3400_0000]),
        (0x0104_e148, &[0xff7f_ffee]),
        // `2^32` as a float and `2^-23` as a double.
        (0x0109_9378, &[0x4f80_0000]),
        (0x010c_9300, &[0, 0x3e80_0000]),
        // The vectors `1, 1, 1, 1` and `0, 0, 0, 1`, the float 2.0 and the table of codes.
        (0x010c_7170, &[0x3f80_0000; 4]),
        (0x010c_71b0, &[0, 0, 0, 0x3f80_0000]),
        (0x010e_c620, &[0x4000_0000]),
        (0x011b_e718, &[0x0c04_0800, 0x0e06_0a02]),
        // The float `-0.25` the first lifting step is given.
        (0x0104_fd68, &[0xbe80_0000]),
    ];

    /// What the callee doubles saw.
    #[derive(Default)]
    struct Seen {
        /// `(this, block, size)` of the allocator's free (virtual slot `0x10`).
        frees: Vec<[u32; 3]>,
        /// `(allocator, array, count, element size)` of `00c908c0`.
        reserves: Vec<[u32; 4]>,
        /// The blocks `00c908c0` handed out, in order.
        blocks: Vec<u32>,
        /// The words of `00e4ead0` and `00e4ebb0`.
        analyses: Vec<Vec<u32>>,
        ranges: Vec<Vec<u32>>,
        /// The statistics blocks `00e4e660` was run on.
        reports: Vec<u32>,
    }

    struct Rig {
        router: u32,
        allocator: u32,
        scratch: u32,
        seen: Rc<RefCell<Seen>>,
    }

    /// An engine with the executable's data area mapped, the router with a scratch stack and an
    /// allocator, and doubles for the callees the unit calls by address.
    fn rig() -> (Engine, Rig) {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        for (address, words) in EXE_WORDS {
            for (index, word) in words.iter().enumerate() {
                e.mem.set_u32(address + 4 * index as u32, *word);
            }
        }
        let seen = Rc::new(RefCell::new(Seen::default()));
        let scratch = e.mem.alloc(0x4000);
        let allocator = e.mem.alloc(0x10);
        e.put_vtable(0x0126_f000, &[0, 0, 0, 0, FREE_DOUBLE]);
        e.mem.set_u32(allocator, 0x0126_f000);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router, scratch);
        e.mem.set_u32(router.wrapping_add(0x10), allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let log = Rc::clone(&seen);
        e.register_double(FREE_DOUBLE, move |_, a| {
            log.borrow_mut().frees.push([a[0], a[1], a[2]]);
            Ret::default()
        });
        // `_reserve`: a fresh block for `count` elements, the old elements copied, the capacity
        // word set to `count` (the array owns its block now).
        let log = Rc::clone(&seen);
        e.register_double(RESERVE, move |e, a| {
            log.borrow_mut().reserves.push([a[0], a[1], a[2], a[3]]);
            let block = e.mem.alloc(a[2] * a[3]);
            let old = e.mem.u32(a[1]);
            let size = e.mem.u32(a[1] + 4);
            for byte in 0..size * a[3] {
                let value = e.mem.u8(old + byte);
                e.mem.set_u8(block + byte, value);
            }
            log.borrow_mut().blocks.push(block);
            e.mem.set_u32(a[1], block);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
        let log = Rc::clone(&seen);
        e.register_double(ANALYSE_FLAGS, move |e, a| {
            log.borrow_mut().analyses.push(a.to_vec());
            for (word, value) in [1u32, 2, 3].into_iter().enumerate() {
                e.mem.set_u32(a[2] + 4 * word as u32, value);
            }
            Ret::default()
        });
        let log = Rc::clone(&seen);
        e.register_double(ANALYSE_RANGES, move |_, a| {
            log.borrow_mut().ranges.push(a.to_vec());
            Ret::default()
        });
        let log = Rc::clone(&seen);
        e.register_double(DISPLAY_REPORT, move |_, a| {
            log.borrow_mut().reports.push(a[0]);
            Ret::default()
        });
        // `_ftol2`: the leading double truncated.
        e.register_double(FLOAT_TO_INT, |_, a| {
            let value = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            Ret {
                eax: value as i32 as u32,
                ..Ret::default()
            }
        });
        let rig = Rig {
            router,
            allocator,
            scratch,
            seen,
        };
        (e, rig)
    }

    fn put_floats(e: &mut Engine, address: u32, values: &[f32]) {
        for (index, value) in values.iter().enumerate() {
            e.mem.set_f32(address + 4 * index as u32, *value);
        }
    }

    fn floats(e: &Engine, address: u32, count: u32) -> Vec<f32> {
        (0..count)
            .map(|index| e.mem.f32(address + 4 * index))
            .collect()
    }

    fn bits(e: &Engine, address: u32, count: u32) -> Vec<u32> {
        (0..count)
            .map(|index| e.mem.u32(address + 4 * index))
            .collect()
    }

    /// Sets up a buffer holding `values`.
    fn buffer(e: &mut Engine, values: &[f32]) -> u32 {
        let address = e.mem.alloc(4 * values.len() as u32 + 16);
        put_floats(e, address, values);
        address
    }

    /// An `hkArray` header at `array`: `data`, `size`, `capacity | flags`.
    fn put_array(e: &mut Engine, array: u32, data: u32, size: u32, capacity_word: u32) {
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 4, size);
        e.mem.set_u32(array + 8, capacity_word);
    }

    #[test]
    fn arrays_are_emptied_and_only_owned_blocks_are_freed() {
        let (mut e, rig) = rig();
        let this = e.mem.alloc(0x60);
        put_array(&mut e, this + 0x3c, 0x5000, 5, 0x10);
        put_array(&mut e, this + 0x30, 0x6000, 6, 0x8000_0008);
        put_array(&mut e, this + 0x24, 0x7000, 7, 0x0000_0009);
        put_array(&mut e, this + 0x18, 0x8000, 8, 0x4000_0004);
        put_array(&mut e, this + 0xc, 0x9000, 1, 0x8000_0000);
        e.call(0x00e5_0ca0, &args![this]);
        // The byte array first, then the word arrays from `+0x30` down; the unowned ones keep their
        // blocks. The capacity bits are masked with `0x3fffffff` (a bit 30 is not a size).
        assert_eq!(
            rig.seen.borrow().frees,
            vec![
                [rig.allocator, 0x5000, 0x10],
                [rig.allocator, 0x7000, 36],
                [rig.allocator, 0x8000, 16],
            ]
        );
        for offset in [0x3cu32, 0x30, 0x24, 0x18, 0xc] {
            assert_eq!(bits(&e, this + offset, 3), vec![0, 0, 0x8000_0000]);
        }
        // Nothing of the router's own state changed.
        assert_eq!(e.mem.u32(rig.router), rig.scratch);
    }

    #[test]
    fn first_lifting_step_adds_neighbour_sums_and_the_doubled_end() {
        let (mut e, _) = rig();
        // Seven elements: one group of four, then the remainder loop.
        let source = buffer(&mut e, &[1.0, 2.0, 4.0, 8.0, 16.0, 32.0, 64.0]);
        let target = buffer(&mut e, &[0.5; 7]);
        e.call(0x00e5_0d90, &args![0.25f32, source, target, 7i32]);
        assert_eq!(
            floats(&e, target, 7),
            vec![1.25, 2.0, 3.5, 6.5, 12.5, 24.5, 32.5]
        );
        // The source is not changed.
        assert_eq!(floats(&e, source, 2), vec![1.0, 2.0]);
    }

    #[test]
    fn first_lifting_step_of_one_element_only_doubles_the_factor() {
        let (mut e, _) = rig();
        let source = buffer(&mut e, &[3.0]);
        let target = buffer(&mut e, &[1.0]);
        e.call(0x00e5_0d90, &args![0.5f32, source, target, 1i32]);
        assert_eq!(floats(&e, target, 1), vec![4.0]);
    }

    #[test]
    fn first_lifting_step_follows_the_exe_for_nan_operands() {
        let (mut e, _) = rig();
        // The first element's sum has a signalling NaN operand and comes out quiet (the payload
        // stays); the last element is `1 * 2 * 1`.
        let source = e.mem.alloc(16);
        e.mem.set_u32(source, 0x7f80_0001);
        e.mem.set_u32(source + 4, 0x3f80_0000);
        let target = buffer(&mut e, &[0.0, 0.0]);
        e.call(0x00e5_0d90, &args![1.0f32, source, target, 2i32]);
        assert_eq!(e.mem.u32(target) & 0xffc0_0000, 0x7fc0_0000);
        assert_eq!(e.mem.f32(target + 4), 2.0);
    }

    #[test]
    fn second_lifting_step_mirrors_the_first() {
        let (mut e, _) = rig();
        let source = buffer(&mut e, &[1.0, 2.0, 4.0, 8.0, 16.0, 32.0, 64.0]);
        let target = buffer(&mut e, &[0.5; 7]);
        e.call(0x00e5_0e50, &args![0.25f32, target, source, 7i32]);
        // `target[0]` gets `source[0]` times twice the factor; the others the neighbour sums.
        assert_eq!(
            floats(&e, target, 7),
            vec![1.0, 1.25, 2.0, 3.5, 6.5, 12.5, 24.5]
        );
    }

    #[test]
    fn second_lifting_step_of_one_element_only_touches_the_first() {
        let (mut e, _) = rig();
        let source = buffer(&mut e, &[3.0]);
        let target = buffer(&mut e, &[1.0]);
        e.call(0x00e5_0e50, &args![0.5f32, target, source, 1i32]);
        assert_eq!(floats(&e, target, 1), vec![4.0]);
    }

    #[test]
    fn sort_orders_a_range_ascending() {
        let (mut e, _) = rig();
        let values = [5.0, -1.0, 3.5, 3.5, 0.0, 9.0, -7.0, 2.0, 2.0, 1.0, 8.0];
        let array = buffer(&mut e, &values);
        e.call(0x00e5_0f10, &args![array, 0i32, 10i32, 0u32]);
        let mut expected = values.to_vec();
        expected.sort_by(f32::total_cmp);
        assert_eq!(floats(&e, array, 11), expected);
    }

    #[test]
    fn sort_leaves_elements_outside_the_range_alone() {
        let (mut e, _) = rig();
        let array = buffer(&mut e, &[9.0, 4.0, 3.0, 2.0, 1.0, 0.0]);
        e.call(0x00e5_0f10, &args![array, 1i32, 4i32, 0u32]);
        assert_eq!(floats(&e, array, 6), vec![9.0, 1.0, 2.0, 3.0, 4.0, 0.0]);
    }

    #[test]
    fn sort_of_one_element_changes_nothing() {
        let (mut e, _) = rig();
        let array = buffer(&mut e, &[2.0, 1.0]);
        e.call(0x00e5_0f10, &args![array, 0i32, 0i32, 0u32]);
        assert_eq!(floats(&e, array, 2), vec![2.0, 1.0]);
    }

    #[test]
    fn interleave_takes_the_halves_alternately_and_puts_the_router_back() {
        let (mut e, rig) = rig();
        let source = buffer(&mut e, &[0.0, 1.0, 2.0, 3.0, 4.0, 5.0]);
        let target = buffer(&mut e, &[-1.0; 7]);
        e.call(0x00e5_1070, &args![source, target, 6i32]);
        assert_eq!(
            floats(&e, target, 7),
            vec![0.0, 3.0, 1.0, 4.0, 2.0, 5.0, -1.0]
        );
        assert_eq!(e.mem.u32(rig.router), rig.scratch);
    }

    #[test]
    fn interleave_of_an_odd_count_leaves_the_last_word() {
        let (mut e, _) = rig();
        let source = buffer(&mut e, &[0.0, 1.0, 2.0, 3.0, 4.0]);
        let target = buffer(&mut e, &[-1.0; 6]);
        e.call(0x00e5_1070, &args![source, target, 5i32]);
        // `count / 2` is 2: `target[0..4]` get words 0, 2, 1, 3.
        assert_eq!(floats(&e, target, 6), vec![0.0, 2.0, 1.0, 3.0, -1.0, -1.0]);
    }

    #[test]
    fn interleave_works_in_place_and_quiets_signalling_nans() {
        let (mut e, _) = rig();
        let data = e.mem.alloc(32);
        for (index, word) in [0x7f80_0001u32, 0x3f80_0000, 0x4000_0000, 0x7fa0_0000]
            .into_iter()
            .enumerate()
        {
            e.mem.set_u32(data + 4 * index as u32, word);
        }
        e.call(0x00e5_1070, &args![data, data, 4i32]);
        assert_eq!(
            bits(&e, data, 4),
            vec![0x7fc0_0001, 0x4000_0000, 0x3f80_0000, 0x7fe0_0000]
        );
    }

    #[test]
    fn interleave_of_nothing_still_moves_the_router_and_puts_it_back() {
        let (mut e, rig) = rig();
        let target = buffer(&mut e, &[7.0]);
        e.call(0x00e5_1070, &args![0u32, target, 0i32]);
        assert_eq!(floats(&e, target, 1), vec![7.0]);
        assert_eq!(e.mem.u32(rig.router), rig.scratch);
    }

    /// Writes the executable's rows the tables of `00e51140` are copied from: row `source`
    /// is four copies of `index + 1`.
    fn put_table_sources(e: &mut Engine) {
        let sources = [
            0x010e_c7f0u32,
            0x010e_c850,
            0x010e_c840,
            0x010e_c830,
            0x010e_c820,
            0x010e_c810,
            0x010e_c800,
            0x010e_c7b0,
            0x010e_c7e0,
            0x010e_c7d0,
            0x010e_c7c0,
            0x010e_c7a0,
            0x010e_c790,
        ];
        for (index, source) in sources.into_iter().enumerate() {
            store_vec(e, source, [index as f32 + 1.0; 4]);
        }
    }

    #[test]
    fn forward_transform_of_eight_sums_the_rows() {
        let (mut e, _) = rig();
        put_table_sources(&mut e);
        let data = buffer(&mut e, &[1.0, 2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
        e.call(0x00e5_1140, &args![data, 8u32]);
        // `data[1] = 2` multiplies the first row of each table: the rows at `0126ef10` (copied
        // from `010ec850`, four copies of 2) and `0126ee90` (from `010ec7e0`, four copies of 9).
        assert_eq!(
            floats(&e, data, 8),
            vec![5.0, 5.0, 5.0, 5.0, 19.0, 19.0, 19.0, 19.0]
        );
        // The tables are filled once and the flags say so.
        assert_eq!(e.mem.u32(TABLE_FLAGS), 3);
        assert_eq!(load_vec(&e, 0x0126_ef10), [2.0; 4]);
        assert_eq!(load_vec(&e, 0x0126_ee90), [9.0; 4]);
        assert_eq!(load_vec(&e, 0x0126_ef70), [8.0; 4]);
        assert_eq!(load_vec(&e, 0x0126_eef0), [6.0; 4]);
    }

    #[test]
    fn forward_transform_keeps_tables_that_exist() {
        let (mut e, _) = rig();
        put_table_sources(&mut e);
        e.mem.set_u32(TABLE_FLAGS, 1);
        store_vec(&mut e, 0x0126_ef10, [10.0; 4]);
        let data = buffer(&mut e, &[0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
        e.call(0x00e5_1140, &args![data, 8u32]);
        // The first table was not touched; the second was made.
        assert_eq!(load_vec(&e, 0x0126_ef10), [10.0; 4]);
        assert_eq!(e.mem.u32(TABLE_FLAGS), 3);
        assert_eq!(
            floats(&e, data, 8),
            vec![10.0, 10.0, 10.0, 10.0, 9.0, 9.0, 9.0, 9.0]
        );
    }

    #[test]
    fn forward_transform_of_sixteen_lifts_and_interleaves() {
        let (mut e, _) = rig();
        // Zero rows: the first eight words become `data[0]` in every lane.
        for source in [
            0x010e_c7f0u32,
            0x010e_c850,
            0x010e_c840,
            0x010e_c830,
            0x010e_c820,
            0x010e_c810,
            0x010e_c800,
            0x010e_c7b0,
            0x010e_c7e0,
            0x010e_c7d0,
            0x010e_c7c0,
            0x010e_c7a0,
            0x010e_c790,
        ] {
            store_vec(&mut e, source, [0.0; 4]);
        }
        // The lifting factors the exe's data holds: -0.25 and 0.5.
        e.mem.set_f32(FIRST_LIFT_FACTOR, -0.25);
        e.mem.set_f32(SECOND_LIFT_FACTOR, 0.5);
        let mut values = vec![4.0f32; 8];
        values.extend([8.0, 8.0, 8.0, 8.0, 8.0, 8.0, 8.0, 8.0]);
        let data = buffer(&mut e, &values);
        e.call(0x00e5_1140, &args![data, 16u32]);
        // With `l` the first eight words (all 4) and `h` the others (all 8): `00e50e50` adds to
        // `l` (with `source = h`): word 0 gets `h[0] * 2 * -0.25`, the others `(h[k - 1] + h[k]) *
        // -0.25`, giving `4 - 4 = 0` and `4 - 4 = 0`. `00e50d90` then adds to `h` the sums of `l`
        // times 0.5 (the new `l` is all 0): `8 + 0 = 8`. The interleave puts them alternately.
        let result = floats(&e, data, 16);
        let expected: Vec<f32> = (0..16)
            .map(|index| if index % 2 == 0 { 0.0 } else { 8.0 })
            .collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn small_values_are_zeroed_level_by_level() {
        let (mut e, _) = rig();
        let data = buffer(&mut e, &[0.5, 0.1, 0.2, 3.0]);
        e.call(0x00e5_1390, &args![data, 4u32, 0.25f32, 0u8]);
        // Index 2 (0.2) is below 0.25 and zeroed; at the level marker 2 the threshold is lowered
        // by 0.2 to 0.05, at which neither 0.1 nor 0.5 is zeroed.
        assert_eq!(floats(&e, data, 4), vec![0.5, 0.1, 0.0, 3.0]);
    }

    #[test]
    fn zeroing_a_negative_value_uses_its_magnitude() {
        let (mut e, _) = rig();
        let data = buffer(&mut e, &[-0.125, 5.0]);
        e.call(0x00e5_1390, &args![data, 2u32, 0.25f32, 0u8]);
        assert_eq!(floats(&e, data, 2), vec![0.0, 5.0]);
    }

    #[test]
    fn nothing_to_threshold_changes_nothing() {
        let (mut e, _) = rig();
        let data = buffer(&mut e, &[0.125]);
        e.call(0x00e5_1390, &args![data, 0u32, 0.25f32, 0u8]);
        assert_eq!(floats(&e, data, 1), vec![0.125]);
    }

    #[test]
    fn thresholding_by_rank_zeroes_values_up_to_the_cutoff() {
        let (mut e, rig) = rig();
        let data = buffer(&mut e, &[1.0, -2.0, 0.5, 4.0]);
        e.call(0x00e5_1390, &args![data, 4u32, 0.5f32, 1u8]);
        // `count * 0.5 - 2^-23` truncates to 1, the position is 0 and the smallest magnitude 0.5
        // is the cutoff: every value whose magnitude is not above it is zeroed.
        assert_eq!(floats(&e, data, 4), vec![1.0, -2.0, 0.0, 4.0]);
        // The scratch array was made on the router's stack and is given back; nothing is reserved
        // or freed because it already had room.
        assert_eq!(e.mem.u32(rig.router), rig.scratch);
        assert!(rig.seen.borrow().reserves.is_empty());
        assert!(rig.seen.borrow().frees.is_empty());
    }

    #[test]
    fn thresholding_by_rank_stops_when_one_over_the_count_is_above_the_threshold() {
        let (mut e, rig) = rig();
        let data = buffer(&mut e, &[1.0, 2.0, 0.5, 4.0]);
        e.call(0x00e5_1390, &args![data, 4u32, 0.125f32, 1u8]);
        assert_eq!(floats(&e, data, 4), vec![1.0, 2.0, 0.5, 4.0]);
        assert_eq!(e.mem.u32(rig.router), rig.scratch);
    }

    #[test]
    fn thresholding_by_rank_clamps_the_position() {
        let (mut e, _) = rig();
        // `count * threshold - 2^-23` is 8 - tiny: position 7 - 1 = 6 is not below `count`.
        let data = buffer(&mut e, &[3.0, 1.0]);
        e.call(0x00e5_1390, &args![data, 2u32, 4.0f32, 1u8]);
        // The position is clamped to the last element: the cutoff is the largest magnitude, 3.
        assert_eq!(floats(&e, data, 2), vec![0.0, 0.0]);
        // A threshold of 0.5 with two elements: position `trunc(1 - tiny) - 1 = -1`, clamped to 0.
        let data = buffer(&mut e, &[3.0, 1.0]);
        e.call(0x00e5_1390, &args![data, 2u32, 0.5f32, 1u8]);
        assert_eq!(floats(&e, data, 2), vec![3.0, 0.0]);
    }

    /// One 0x30-byte record: translation, rotation, scale (four lanes each).
    fn put_record(
        e: &mut Engine,
        address: u32,
        translation: [f32; 3],
        rotation: [f32; 4],
        scale: [f32; 3],
    ) {
        store_vec(
            e,
            address,
            [translation[0], translation[1], translation[2], 0.0],
        );
        store_vec(e, address + 0x10, rotation);
        store_vec(e, address + 0x20, [scale[0], scale[1], scale[2], 1.0]);
    }

    /// A block of `tracks` tracks of `samples` samples with the tolerances `tolerances` (the same
    /// for every track: translation, relative translation, rotation, scale) and the byte
    /// `range_byte`.
    fn block(
        e: &mut Engine,
        records: u32,
        tracks: u32,
        samples: u32,
        tolerances: [f32; 4],
        range_byte: u8,
    ) -> u32 {
        let block = e.mem.alloc(0x48);
        e.mem.set_u32(block, records);
        e.mem.set_u32(block + 4, tracks);
        e.mem.set_u32(block + 8, samples);
        for (index, tolerance) in tolerances.into_iter().enumerate() {
            let data = e.mem.alloc(4 * tracks + 4);
            for track in 0..tracks {
                e.mem.set_f32(data + 4 * track, tolerance);
            }
            e.mem.set_u32(block + 0xc + 0xc * index as u32, data);
        }
        let data = e.mem.alloc(tracks + 4);
        for track in 0..tracks {
            e.mem.set_u8(data + track, range_byte);
        }
        e.mem.set_u32(block + 0x3c, data);
        block
    }

    #[test]
    fn analysing_no_tracks_only_moves_and_restores_the_router() {
        let (mut e, rig) = rig();
        let records = e.mem.alloc(0x30);
        let block = block(&mut e, records, 0, 5, [0.1; 4], 0);
        let out = buffer(&mut e, &[-1.0; 4]);
        let flags = e.mem.alloc(8);
        e.call(0x00e4_f910, &args![block, flags, out, 0u8]);
        assert_eq!(floats(&e, out, 4), vec![-1.0; 4]);
        assert_eq!(e.mem.u32(rig.router), rig.scratch);
        let seen = rig.seen.borrow();
        assert!(seen.reserves.is_empty());
        assert!(seen.frees.is_empty());
        assert!(seen.analyses.is_empty() && seen.reports.is_empty());
    }

    #[test]
    fn a_track_of_one_sample_keeps_its_translation_and_is_flagged() {
        let (mut e, rig) = rig();
        let records = e.mem.alloc(0x30);
        // A zero quaternion: its length is below the limit, so the average rotation is the
        // identity `(0, 0, 0, 1)` without the host's `RSQRTSS`.
        put_record(&mut e, records, [1.0, 2.0, 3.0], [0.0; 4], [1.0; 3]);
        let block = block(&mut e, records, 1, 1, [0.1, 0.0, 2.0, 0.1], 0);
        let out = buffer(&mut e, &[-1.0; 5]);
        let flags = e.mem.alloc(8);
        e.call(0x00e4_f910, &args![block, flags, out, 0u8]);
        // The rotation (identity) and the scale (all ones) are within the tolerances of the
        // sample: bits `0x08` and `0x20` mark them, the translation is kept and written.
        assert_eq!(e.mem.u16(flags), 0x29);
        assert_eq!(floats(&e, out, 5), vec![1.0, 2.0, 3.0, -1.0, -1.0]);
        // The scratch router is back where it was and nothing was reserved or freed.
        assert_eq!(e.mem.u32(rig.router), rig.scratch);
        let seen = rig.seen.borrow();
        assert!(seen.reserves.is_empty() && seen.frees.is_empty());
    }

    #[test]
    fn a_report_is_made_only_when_asked_for() {
        let (mut e, rig) = rig();
        let records = e.mem.alloc(0x30);
        put_record(&mut e, records, [1.0, 2.0, 3.0], [0.0; 4], [1.0; 3]);
        let block = block(&mut e, records, 1, 1, [0.1, 0.0, 2.0, 0.1], 0);
        let out = buffer(&mut e, &[0.0; 4]);
        let flags = e.mem.alloc(8);
        e.call(0x00e4_f910, &args![block, flags, out, 1u8]);
        let seen = rig.seen.borrow();
        // `00e4ead0` gets the flags, the track count and the statistics block, which `00e4e660`
        // is then run on.
        assert_eq!(seen.analyses.len(), 1);
        assert_eq!(seen.analyses[0][..2], [flags, 1]);
        assert_eq!(seen.reports, vec![seen.analyses[0][2]]);
    }

    /// Three tracks of four samples with values from a small generator, run on frames filled
    /// with different words: the results and the scratch table must not depend on the fill.
    fn analysis_with_fill(seed: u32, fill: u32) -> (Vec<u16>, Vec<u32>, Vec<u32>) {
        let (mut e, rig) = rig();
        let records = e.mem.alloc(0x30 * 12);
        let mut seed = seed;
        let mut next = || {
            seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            ((seed >> 16) % 64) as f32 / 16.0 - 2.0
        };
        for index in 0..12u32 {
            let translation = [next(), next(), next()];
            let rotation = [next(), next(), next(), next()];
            let scale = [next(), next(), next()];
            put_record(&mut e, records + 0x30 * index, translation, rotation, scale);
        }
        let block = block(&mut e, records, 3, 4, [0.5, 0.25, 0.75, 0.5], 1);
        let out = buffer(&mut e, &[-1.0; 64]);
        let flags = e.mem.alloc(8);
        run_00e4f910(&mut e, [block, flags, out, 0], fill);
        let table = bits(&e, rig.scratch, 36);
        (
            (0..3).map(|track| e.mem.u16(flags + 2 * track)).collect(),
            bits(&e, out, 64),
            table,
        )
    }

    #[test]
    fn the_analysis_does_not_read_the_frame_it_did_not_write() {
        for seed in 1..40u32 {
            let first = analysis_with_fill(seed, 0);
            assert_eq!(analysis_with_fill(seed, 0xdead_beef), first);
            assert_eq!(analysis_with_fill(seed, 0x7fc0_0000), first);
        }
    }

    #[test]
    fn a_translation_close_to_the_origin_is_zeroed_and_flagged() {
        let (mut e, _) = rig();
        let records = e.mem.alloc(0x30);
        // The static vector the translation is compared with (`01267e30`) is zero; 0.01 is within
        // half the tolerance 0.1 of it: bit 1 is set, bit 0 cleared, the translation zeroed.
        put_record(&mut e, records, [0.01, -0.01, 0.0], [0.0; 4], [1.0; 3]);
        let block = block(&mut e, records, 1, 1, [0.1, 0.0, 2.0, 0.1], 0);
        let out = buffer(&mut e, &[-1.0; 5]);
        let flags = e.mem.alloc(8);
        e.call(0x00e4_f910, &args![block, flags, out, 0u8]);
        assert_eq!(e.mem.u16(flags), 0x2a);
        // Flags `0x2a`: the translation is dropped, the rotation and scale identity too.
        assert_eq!(floats(&e, out, 5), vec![-1.0; 5]);
    }

    /// The matrix `00e50920` reads: `rows` rows of `columns` floats at `matrix[0]`, the column
    /// count at `+4` and the thresholds at `+0xc`.
    fn matrix(e: &mut Engine, columns: u32, rows: &[&[f32]], thresholds: &[f32]) -> u32 {
        let matrix = e.mem.alloc(0x20);
        let cells = e.mem.alloc(4 * columns * rows.len() as u32 + 4);
        for (row, values) in rows.iter().enumerate() {
            put_floats(e, cells + 4 * columns * row as u32, values);
        }
        let limits = buffer(e, thresholds);
        e.mem.set_u32(matrix, cells);
        e.mem.set_u32(matrix + 4, columns);
        e.mem.set_u32(matrix + 0xc, limits);
        matrix
    }

    #[test]
    fn column_ranges_set_flags_and_write_midpoints() {
        let (mut e, rig) = rig();
        // Three rows, two columns: column 0 spans 1..3 (midpoint 2, range 2), column 1 spans
        // 10..14 (midpoint 12, range 4); the thresholds 1.5 allow up to 2 * 1.5 = 3.
        let matrix = matrix(
            &mut e,
            2,
            &[&[1.0, 10.0], &[3.0, 14.0], &[2.0, 12.0]],
            &[1.5, 1.5],
        );
        let records = e.mem.alloc(0x30);
        let info = block(&mut e, records, 0, 3, [0.0; 4], 0);
        let flags = e.mem.alloc(8);
        e.mem.set_u16(flags, 0x77);
        e.mem.set_u16(flags + 2, 0x77);
        let out = buffer(&mut e, &[-1.0; 12]);
        e.call(0x00e5_0920, &args![info, matrix, flags, out, 0u8]);
        // The track analysis found no tracks. `00e4ead0`'s statistics words (1, 2 and 3 in the
        // double) move the output by six floats; column 0 qualifies (its flag is 1 and its
        // midpoint is written), column 1 does not (its flag is 0).
        assert_eq!(e.mem.u16(flags), 1);
        assert_eq!(e.mem.u16(flags + 2), 0);
        assert_eq!(
            floats(&e, out, 8),
            vec![-1.0, -1.0, -1.0, -1.0, -1.0, -1.0, 2.0, -1.0]
        );
        // The router's scratch stack is back where it was.
        assert_eq!(e.mem.u32(rig.router), rig.scratch);
        let seen = rig.seen.borrow();
        assert!(seen.ranges.is_empty() && seen.reports.is_empty());
        assert_eq!(seen.analyses.len(), 1);
        assert_eq!(seen.analyses[0][..2], [flags, 0]);
        assert!(seen.reserves.is_empty() && seen.frees.is_empty());
    }

    #[test]
    fn column_flags_follow_the_track_flags_and_a_report_may_be_made() {
        let (mut e, rig) = rig();
        // One track of two samples, every tolerance 0: the track's own flags are `0xffc0` and
        // nothing is written for it. The matrix has one column over the two samples.
        let records = e.mem.alloc(0x60);
        for index in 0..2u32 {
            put_record(
                &mut e,
                records + 0x30 * index,
                [index as f32, 1.0, 2.0],
                [0.0; 4],
                [1.0; 3],
            );
        }
        let info = block(&mut e, records, 1, 2, [0.0; 4], 0);
        let matrix = matrix(&mut e, 1, &[&[0.0], &[1.0]], &[10.0]);
        let flags = e.mem.alloc(8);
        let out = buffer(&mut e, &[-1.0; 12]);
        e.call(0x00e5_0920, &args![info, matrix, flags, out, 1u8]);
        assert_eq!(e.mem.u16(flags), 0xffc0);
        // The column's flag comes after the track's: `flags[1]`; the range 1 is below `2 * 10`
        // and its midpoint 0.5 is written after the six floats of the statistics.
        assert_eq!(e.mem.u16(flags + 2), 1);
        assert_eq!(e.mem.f32(out + 24), 0.5);
        let seen = rig.seen.borrow();
        assert_eq!(seen.analyses.len(), 1);
        assert_eq!(seen.analyses[0][..2], [flags, 1]);
        // The second statistics block is made by `00e4ebb0` from `(flags, flags + 2 * offset,
        // offset, columns)` and reported.
        assert_eq!(seen.ranges.len(), 1);
        assert_eq!(seen.ranges[0][..4], [flags, flags + 2, 1, 1]);
        assert_eq!(seen.reports, vec![seen.ranges[0][4]]);
    }

    #[test]
    fn a_column_of_equal_values_has_a_zero_range() {
        let (mut e, _) = rig();
        // The range of a constant column is 0, below any positive limit: flag 1.
        let matrix_positive = matrix(&mut e, 1, &[&[4.0], &[4.0]], &[0.25]);
        let records = e.mem.alloc(0x30);
        let info = block(&mut e, records, 0, 2, [0.0; 4], 0);
        let flags = e.mem.alloc(8);
        let out = buffer(&mut e, &[-1.0; 8]);
        e.call(0x00e5_0920, &args![info, matrix_positive, flags, out, 0u8]);
        assert_eq!(e.mem.u16(flags), 1);
        assert_eq!(e.mem.f32(out + 24), 4.0);
        // With a zero threshold the limit is 0, which the range 0 does not pass.
        let matrix_zero = matrix(&mut e, 1, &[&[4.0], &[4.0]], &[0.0]);
        let flags = e.mem.alloc(8);
        e.mem.set_u16(flags, 9);
        e.call(0x00e5_0920, &args![info, matrix_zero, flags, out, 0u8]);
        assert_eq!(e.mem.u16(flags), 0);
    }

    #[test]
    fn the_block_for_a_report_run_is_built_from_the_record() {
        let (mut e, rig) = rig();
        let source = e.mem.alloc(0x40);
        let records = e.mem.alloc(0x30);
        put_record(&mut e, records, [1.0, 2.0, 3.0], [0.0; 4], [1.0; 3]);
        e.mem.set_u32(source, records);
        e.mem.set_u32(source + 4, 1);
        e.mem.set_u32(source + 8, 1);
        // The tolerances and the byte of the one track: 0.1, 0, 2 and 0.1.
        e.mem.set_f32(source + 0xc, 0.1);
        e.mem.set_f32(source + 0x10, 0.0);
        e.mem.set_f32(source + 0x14, 2.0);
        e.mem.set_f32(source + 0x18, 0.1);
        e.mem.set_u8(source + 0x1c, 0);
        let flags = e.mem.alloc(8);
        let out = buffer(&mut e, &[-1.0; 5]);
        e.call(0x00e5_0650, &args![source, flags, out]);
        // The same track as in `a_track_of_one_sample_keeps_its_translation_and_is_flagged`.
        assert_eq!(e.mem.u16(flags), 0x29);
        assert_eq!(floats(&e, out, 4), vec![1.0, 2.0, 3.0, -1.0]);
        let seen = rig.seen.borrow();
        // The five arrays are empty and unowned: each is reserved for the one entry (the byte
        // array last), and given back by `00e50ca0` (the byte array first). The run asks for the
        // report.
        let array_sizes = [4u32, 4, 4, 4, 1];
        assert_eq!(seen.reserves.len(), 5);
        for (index, size) in array_sizes.into_iter().enumerate() {
            assert_eq!(seen.reserves[index][0], rig.allocator);
            assert_eq!(seen.reserves[index][2..], [1, size]);
        }
        assert_eq!(seen.blocks.len(), 5);
        assert_eq!(
            seen.frees,
            vec![
                [rig.allocator, seen.blocks[4], 1],
                [rig.allocator, seen.blocks[3], 4],
                [rig.allocator, seen.blocks[2], 4],
                [rig.allocator, seen.blocks[1], 4],
                [rig.allocator, seen.blocks[0], 4],
            ]
        );
        assert_eq!(seen.analyses.len(), 1);
        assert_eq!(seen.reports.len(), 1);
        assert_eq!(e.mem.u32(rig.router), rig.scratch);
    }

    #[test]
    fn a_record_without_tracks_reserves_nothing() {
        let (mut e, rig) = rig();
        let source = e.mem.alloc(0x40);
        e.mem.set_u32(source, 0x1000);
        e.mem.set_u32(source + 8, 5);
        let flags = e.mem.alloc(8);
        let out = buffer(&mut e, &[-1.0; 2]);
        e.call(0x00e5_0650, &args![source, flags, out]);
        let seen = rig.seen.borrow();
        // Capacity 0 is not below count 0; nothing is owned, so nothing is freed.
        assert!(seen.reserves.is_empty() && seen.frees.is_empty());
        assert_eq!(floats(&e, out, 2), vec![-1.0; 2]);
        // The report is asked for: the statistics are made for zero tracks.
        assert_eq!(seen.analyses.len(), 1);
    }
}

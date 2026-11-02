//! `Havok SDK/run_00dd0ff0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `fn_00dd0ff0` and `fn_00dd1180` run the same recurrence over a chain of blocks (a block is
//! three 16-byte vectors; the code reads the vectors of the neighbouring blocks too): for each
//! block it sums a weighted square of the block's two vectors with the diagonal terms of two
//! 16-byte vectors found through an index array, subtracts the previous block's coupling,
//! keeps the result above a floor, and carries the coupling to the next block. The x87 steps
//! (each `FSTP` stores a `float`) and the SSE steps are translated instruction by instruction
//! in `factor_blocks`. Outputs go to three consecutive `float`s per block.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkgeometryprocessing::{
    add4, compare_flags, load_vec4, mul4, shufps, sub4, sum3, unpckhps, Vec4,
};

/// The `float` floor of the recurrence (`0104da4c`) and the `double` added to the sum
/// (`010c9300`).
const FLOOR_FLOAT: u32 = 0x0104_da4c;
const TINY_DOUBLE: u32 = 0x010c_9300;

/// The diagonal sum of one block: with `a` and `b` the block's two vectors and `va`, `vb` the
/// two index-selected vectors, `s = a * a * va + b * b * vb` (lane by lane; when `b_first` is
/// set the code forms `b * b * vb` first and adds `a * a * va` to it) and `w = va + vb`; the
/// answer is lane 0 of the pairwise sums of `[s0, s1, s2, w3]`, `(x3 + x1) + (x2 + x0)`
/// (`UNPCKHPS`, `SHUFPS` 0xc4, 0x4e and 0xb1 with `ADDPS`).
fn diagonal_sum(a: Vec4, b: Vec4, va: Vec4, vb: Vec4, b_first: bool) -> f32 {
    let a_squared = mul4(a, a);
    let b_squared = mul4(b, b);
    let s = if b_first {
        add4(mul4(b_squared, vb), mul4(a_squared, va))
    } else {
        add4(mul4(a_squared, va), mul4(b_squared, vb))
    };
    let w = add4(va, vb);
    let high = unpckhps(s, w);
    let x = shufps(s, high, 0xc4);
    let pairs = add4(shufps(x, x, 0x4e), x);
    add4(shufps(pairs, pairs, 0xb1), pairs)[0]
}

/// The coupling of a block to the next: with `m` the vector 0x10 before the block's `a`, `b`
/// at `+0x10`, `c` at `+0x20`, `d` at `+0x30` (the next block's `a`) and `vb` the second
/// index-selected vector, the lanes 0 to 2 summed (in the code's order) of
/// `d * b * vb - c * m * splat(vb.w)`.
fn coupling(e: &Engine, diagonal: u32, vb: Vec4) -> f32 {
    let m = load_vec4(e, diagonal - 0x10);
    let c = load_vec4(e, diagonal + 0x20);
    let b = load_vec4(e, diagonal + 0x10);
    let d = load_vec4(e, diagonal + 0x30);
    let cm = mul4(c, m);
    let cm = mul4(cm, [vb[3]; 4]);
    let db = mul4(mul4(d, b), vb);
    sum3(sub4(db, cm))
}

/// The recurrence of `00dd0ff0` and `00dd1180`. `out` is the address of the second of the three
/// `float`s of the first block, `diagonal` the address of the first block's first vector, `arr`
/// the address of the index array (entry `i` and `i + 1` select the vectors of block `i`, each
/// entry plus `offset` plus 0x30 being the address of a vector). The loop runs `count` times
/// (the callers guarantee `count > 0`). `floor` and `tiny` are read once.
///
/// State: `previous` and `ratio` are the two `float` locals the code keeps on its stack
/// (words, starting at 0.0). For each block `i`: `s` is the diagonal sum, `t1 = f32(s +
/// tiny)`, `w1 = f32(t1 + epsilon)`; the coupling `term` is 0.0, or the coupling sum (through
/// `FLD`/`FSTP`) when `i < count - 1`; then `out[-1] = previous`, `out[0] = f32(w1 - previous *
/// ratio)`, which is reloaded and kept when above the floor (else the floor), becomes the new
/// `previous` and is stored back at `out[0]`; `out[1] = ratio = f32(term / out[0])` and
/// `previous = f32(term)`.
fn factor_blocks(
    e: &mut Engine,
    mut out: u32,
    mut diagonal: u32,
    count: i32,
    epsilon: f32,
    mut arr: u32,
    offset: u32,
    b_first: bool,
) {
    let floor = x87_load32(e.mem.u32(FLOOR_FLOAT));
    let tiny = e.mem.f64(TINY_DOUBLE);
    let epsilon = x87_load32(epsilon.to_bits());
    let mut previous = 0u32;
    let mut ratio = 0u32;
    let mut block = 0i32;
    loop {
        let second_entry = e.mem.u32(arr + 4).wrapping_add(offset);
        let first_entry = e.mem.u32(arr).wrapping_add(offset);
        let a = load_vec4(e, diagonal);
        let b = load_vec4(e, diagonal + 0x10);
        let va = load_vec4(e, first_entry.wrapping_add(0x30));
        let vb = load_vec4(e, second_entry.wrapping_add(0x30));
        let sum = diagonal_sum(a, b, va, vb, b_first);
        let with_coupling = block < count.wrapping_sub(1);
        let t1 = x87_store32(x87_add(x87_load32(sum.to_bits()), tiny));
        let w1 = x87_store32(x87_add(x87_load32(t1), epsilon));
        let mut term = x87_store32(0.0);
        if with_coupling {
            let vb_again = load_vec4(e, second_entry.wrapping_add(0x30));
            term = x87_quiet_bits(coupling(e, diagonal, vb_again).to_bits());
        }
        e.mem.set_u32(out.wrapping_sub(4), x87_quiet_bits(previous));
        let product = x87_mul(x87_load32(previous), x87_load32(ratio));
        let result = x87_sub(x87_m32(w1), product);
        e.mem.set_u32(out, x87_store32(result));
        let reloaded = x87_load32(e.mem.u32(out));
        let flags = compare_flags(reloaded, floor);
        previous = if flags & 0x41 == 0 {
            x87_store32(reloaded)
        } else {
            x87_store32(floor)
        };
        e.mem.set_u32(out, x87_quiet_bits(previous));
        let divisor = x87_m32(e.mem.u32(out));
        let term_value = x87_load32(term);
        let quotient = x87_store32(x87_div(term_value, divisor));
        e.mem.set_u32(out.wrapping_add(4), x87_quiet_bits(quotient));
        ratio = x87_quiet_bits(quotient);
        previous = x87_store32(term_value);
        block += 1;
        out = out.wrapping_add(0xc);
        diagonal = diagonal.wrapping_add(0x30);
        arr = arr.wrapping_add(4);
        if block >= count {
            break;
        }
    }
}

// Translated from 00dd0ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. Inputs are in registers, so the register values come first: `out`
/// (`ECX`) and `blocks` (`EDX`); then the cdecl stack words `count`, `epsilon` (a `float`),
/// `arr` and `offset`. Does nothing when `count` is not positive. Otherwise runs the recurrence
/// described at `factor_blocks` over `count` blocks: the first block's vectors are at `blocks
/// + 0x10` (and the one before it at `blocks`), each later block 0x30 further; the outputs are
/// three `float`s per block from `out` (the code adds 4 to `ECX` first and writes `out[-1]`,
/// `out[0]`, `out[1]` around it); the selected vectors are at `arr[i] + offset + 0x30` and
/// `arr[i + 1] + offset + 0x30`.
pub fn fn_00dd0ff0(
    e: &mut Engine,
    out: u32,
    blocks: u32,
    count: i32,
    epsilon: f32,
    arr: u32,
    offset: u32,
) {
    if count <= 0 {
        return;
    }
    factor_blocks(
        e,
        out.wrapping_add(4),
        blocks.wrapping_add(0x10),
        count,
        epsilon,
        arr,
        offset,
        false,
    );
}

// Translated from 00dd1180 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, nine stack words: `count`, `value_a` and `value_b` (`float`s),
/// `epsilon` (a `float`), `source`, `offset`, `_unused_6`, `_unused_7`, `cursor`). Builds a block
/// record at the address held in the word `cursor` points at (`rec`) and advances that word by the
/// record's size.
///
/// The record's first word becomes `f32(float at rec + tiny)` (x87, `tiny` the `double` at
/// `010c9300`) with its low byte replaced by 0x1d and its upper half (`+2`) by the low 16 bits
/// of `count` (`n`); `+8` and `+0xc` hold `value_a` and `value_b` (through the x87 stack); `+4` holds
/// the size `(n * 0x44 + 0x27) & !0xf`. `count + 1` words from `source` are copied to `rec + 0x14
/// + n * 0x40`. If `count` is positive the recurrence of `factor_blocks` (`b * b * vb` formed
/// first) runs over `count` blocks whose first vector is at `rec + 0x20`, with the index array
/// being the words just copied and the outputs from `rec + 0x14 + n * 0x30`. The word at `cursor`
/// is advanced by the size at `rec + 4` in every case.
pub fn fn_00dd1180(
    e: &mut Engine,
    count: u32,
    value_a: f32,
    value_b: f32,
    epsilon: f32,
    source: u32,
    offset: u32,
    _unused_6: u32,
    _unused_7: u32,
    cursor: u32,
) {
    let record = e.mem.u32(cursor);
    let first = e.mem.u32(record);
    let tiny = e.mem.f64(TINY_DOUBLE);
    let word = x87_store32(x87_add(x87_load32(first), tiny));
    e.mem.set_u32(record, word);
    e.mem.set_u32(record + 8, x87_quiet_bits(value_a.to_bits()));
    e.mem
        .set_u32(record + 0xc, x87_quiet_bits(value_b.to_bits()));
    let blocks = count & 0xffff;
    let size = blocks.wrapping_mul(0x44).wrapping_add(0x27) & 0xffff_fff0;
    e.mem.set_u32(record + 4, size);
    e.mem.set_u8(record, 0x1d);
    e.mem.set_u16(record + 2, count as u16);
    let words = count.wrapping_add(1) as i32;
    let copy = record.wrapping_add(0x14).wrapping_add(blocks << 6);
    if words > 0 {
        for k in 0..words as u32 {
            let value = e.mem.u32(source.wrapping_add(4 * k));
            e.mem.set_u32(copy.wrapping_add(4 * k), value);
        }
    }
    let blocks = u32::from(e.mem.u16(record + 2));
    let out = record
        .wrapping_add(0x10)
        .wrapping_add(blocks.wrapping_mul(0x30));
    if count as i32 > 0 {
        let copy = record.wrapping_add(0x14).wrapping_add(blocks << 6);
        factor_blocks(
            e,
            out.wrapping_add(4),
            record.wrapping_add(0x20),
            count as i32,
            epsilon,
            copy,
            offset,
            true,
        );
    }
    let size = e.mem.u32(record + 4);
    let advanced = e.mem.u32(cursor).wrapping_add(size);
    e.mem.set_u32(cursor, advanced);
}

// Translated from 00dd13b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, one stack word): answers `base + 0x10`.
pub fn fn_00dd13b0(_e: &mut Engine, base: u32) -> u32 {
    base.wrapping_add(0x10)
}

// Translated from 00dd13c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, one stack word): answers `base + 0x10` (the same code as
/// `00dd13b0`).
pub fn fn_00dd13c0(_e: &mut Engine, base: u32) -> u32 {
    base.wrapping_add(0x10)
}

// Translated from 00dd13d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, one stack word): answers `base + 0x20`.
pub fn fn_00dd13d0(_e: &mut Engine, base: u32) -> u32 {
    base.wrapping_add(0x20)
}

// Translated from 00dd13e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl: `base`, `index`): answers `base + 0x20 + index * 0x90`.
pub fn fn_00dd13e0(_e: &mut Engine, base: u32, index: u32) -> u32 {
    index
        .wrapping_mul(9)
        .wrapping_shl(4)
        .wrapping_add(base)
        .wrapping_add(0x20)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00dd0ff0, fn_00dd0ff0(u32, u32, i32, f32, u32, u32)),
        entry!(
            0x00dd1180,
            fn_00dd1180(u32, f32, f32, f32, u32, u32, u32, u32, u32)
        ),
        entry!(0x00dd13b0, fn_00dd13b0(u32) -> u32),
        entry!(0x00dd13c0, fn_00dd13c0(u32) -> u32),
        entry!(0x00dd13d0, fn_00dd13d0(u32) -> u32),
        entry!(0x00dd13e0, fn_00dd13e0(u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkgeometryprocessing::store_vec4;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(FLOOR_FLOAT, 8);
        e.map(TINY_DOUBLE, 16);
        e.mem.set_f32(FLOOR_FLOAT, f32::EPSILON);
        e.mem.set_f64(TINY_DOUBLE, f64::from(f32::EPSILON));
        e
    }

    /// An index array of `entries` words, each pointing at memory whose vector at `+0x30` is
    /// all ones.
    fn ones_index(e: &mut Engine, entries: u32) -> u32 {
        let target = e.mem.alloc(0x40);
        store_vec4(e, target + 0x30, [1.0; 4]);
        let arr = e.mem.alloc(4 * entries);
        for i in 0..entries {
            e.mem.set_u32(arr + 4 * i, target);
        }
        arr
    }

    fn floats(e: &Engine, at: u32, n: u32) -> Vec<f32> {
        (0..n).map(|i| e.mem.f32(at + 4 * i)).collect()
    }

    /// Two blocks laid out from `blocks` (the register value): the vector before the first block
    /// `m`, then `a0`, `b0`, `c0`, `a1`, `b1`, `c1`.
    fn two_blocks(e: &mut Engine) -> u32 {
        let blocks = e.mem.alloc(0x80);
        let vectors = [
            [1.0, 1.0, 1.0, 1.0],
            [1.0, 2.0, 3.0, 4.0],
            [1.0, 1.0, 1.0, 1.0],
            [1.0, 1.0, 1.0, 4.0],
            [2.0, 2.0, 2.0, 2.0],
            [1.0, 1.0, 1.0, 1.0],
            [0.0; 4],
        ];
        for (i, v) in vectors.into_iter().enumerate() {
            store_vec4(e, blocks + 0x10 * i as u32, v);
        }
        blocks
    }

    #[test]
    fn two_blocks_carry_the_coupling_into_the_second() {
        let mut e = engine();
        let blocks = two_blocks(&mut e);
        let arr = ones_index(&mut e, 3);
        let out = e.mem.alloc(0x30);
        e.call(0x00dd_0ff0, &args![out, blocks, 2i32, 0.5f32, arr, 0u32]);
        // Block 0: the diagonal sum is 19 and the coupling 3; block 1: the diagonal sum is 17.
        let ratio = (3.0f64 / 19.5) as f32;
        let second = (17.5f64 - 3.0f64 * f64::from(ratio)) as f32;
        assert_eq!(floats(&e, out, 6), vec![0.0, 19.5, ratio, 3.0, second, 0.0]);
    }

    #[test]
    fn a_result_below_the_floor_is_replaced_by_the_floor() {
        let mut e = engine();
        let blocks = two_blocks(&mut e);
        let arr = ones_index(&mut e, 2);
        let out = e.mem.alloc(0x30);
        e.call(0x00dd_0ff0, &args![out, blocks, 1i32, -100.0f32, arr, 0u32]);
        assert_eq!(floats(&e, out, 3), vec![0.0, f32::EPSILON, 0.0]);
    }

    #[test]
    fn an_empty_chain_writes_nothing() {
        let mut e = engine();
        let out = e.mem.alloc(0x30);
        e.mem.set_u32(out, 0x1234);
        e.call(0x00dd_0ff0, &args![out, 0u32, 0i32, 0.5f32, 0u32, 0u32]);
        e.call(0x00dd_0ff0, &args![out, 0u32, -3i32, 0.5f32, 0u32, 0u32]);
        assert_eq!(e.mem.u32(out), 0x1234);
    }

    #[test]
    fn a_nan_result_is_not_above_the_floor() {
        let mut e = engine();
        let blocks = two_blocks(&mut e);
        let arr = ones_index(&mut e, 2);
        let out = e.mem.alloc(0x30);
        e.call(0x00dd_0ff0, &args![out, blocks, 1i32, f32::NAN, arr, 0u32]);
        // The sum is NaN, which compares unordered, so the floor is kept.
        assert_eq!(e.mem.f32(out + 4), f32::EPSILON);
    }

    /// A record area with the first block's vectors at `rec + 0x20` and `rec + 0x30` and the
    /// header float at `rec`, plus the pointer word the function advances.
    fn record_area(e: &mut Engine) -> (u32, u32, u32) {
        let record = e.mem.alloc(0x100);
        e.mem.set_f32(record, 1.0);
        store_vec4(e, record + 0x20, [1.0, 2.0, 3.0, 4.0]);
        store_vec4(e, record + 0x30, [1.0; 4]);
        let cursor = e.mem.alloc(4);
        e.mem.set_u32(cursor, record);
        let source = ones_index(e, 2);
        (record, cursor, source)
    }

    #[test]
    fn a_record_is_built_with_its_header_copied_words_and_first_block() {
        let mut e = engine();
        let (record, cursor, source) = record_area(&mut e);
        e.call(
            0x00dd_1180,
            &args![1u32, 1.5f32, 2.5f32, 0.5f32, source, 0u32, 0u32, 0u32, cursor],
        );
        // 1.0 + 2^-23 is 0x3f800001; the low byte becomes 0x1d and the upper half the count.
        assert_eq!(e.mem.u32(record), 0x0001_001d);
        assert_eq!(e.mem.u32(record + 4), 0x60);
        assert_eq!(e.mem.f32(record + 8), 1.5);
        assert_eq!(e.mem.f32(record + 0xc), 2.5);
        let target = e.mem.u32(source);
        assert_eq!(e.mem.u32(record + 0x54), target);
        assert_eq!(e.mem.u32(record + 0x58), target);
        assert_eq!(floats(&e, record + 0x40, 3), vec![0.0, 19.5, 0.0]);
        assert_eq!(e.mem.u32(cursor), record + 0x60);
    }

    #[test]
    fn a_record_without_blocks_only_gets_its_header() {
        let mut e = engine();
        let (record, cursor, source) = record_area(&mut e);
        e.call(
            0x00dd_1180,
            &args![0u32, 0.0f32, 0.0f32, 0.5f32, source, 0u32, 0u32, 0u32, cursor],
        );
        assert_eq!(e.mem.u32(record), 0x0000_001d);
        assert_eq!(e.mem.u32(record + 4), 0x20);
        assert_eq!(e.mem.u32(cursor), record + 0x20);
        // One word was copied from the source (count + 1) and no block was run.
        assert_eq!(e.mem.u32(record + 0x14), e.mem.u32(source));
        assert_eq!(e.mem.u32(record + 0x40), 0);
    }

    #[test]
    fn the_small_address_functions_add_their_constants() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00dd_13b0, &args![0x1000u32]).u32(), 0x1010);
        assert_eq!(e.call(0x00dd_13c0, &args![0x1000u32]).u32(), 0x1010);
        assert_eq!(e.call(0x00dd_13d0, &args![0x1000u32]).u32(), 0x1020);
        assert_eq!(
            e.call(0x00dd_13e0, &args![0x1000u32, 3u32]).u32(),
            0x1000 + 0x20 + 3 * 0x90
        );
    }
}

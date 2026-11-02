//! `Havok SDK/run_00e3c9c0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::havok_sdk::run_00d6bf60::{
    addps, load_vec, maxps, minps, mulps, shufps, store_vec, subps, Vec4,
};

/// `hkpShapeCollection::calcContentStatistics` (Xbox PDB, `00d213e0`): thiscall, `RET 8`.
const CALC_CONTENT_STATISTICS: u32 = 0x00d2_13e0;
/// `hkVector4::setTransformedPos` (Xbox PDB, `00c7f640`): thiscall, `this` is the result, then
/// the transform and the position, `RET 8`.
const SET_TRANSFORMED_POS: u32 = 0x00c7_f640;
/// `00d59710`: thiscall on the object to build (`this` returned), one `float`, `RET 4`.
const BUILD_SHAPE_OBJECT: u32 = 0x00d5_9710;
/// `hkAabbUtil::calcAabb_ov3` (Xbox PDB, `00d26230`): cdecl, five words.
const CALC_AABB: u32 = 0x00d2_6230;
/// The `float` that fills lane 3 of a quantised point (`01011d78`).
const LANE_THREE: u32 = 0x0101_1d78;
/// The `float`s every lane of the box minimum and maximum start from (`0104da48`, `0104e148`).
const BOX_MINIMUM_START: u32 = 0x0104_da48;
const BOX_MAXIMUM_START: u32 = 0x0104_e148;
/// The 16-byte vector the box sums and differences are scaled by (`010c71d0`).
const HALF_VECTOR: u32 = 0x010c_71d0;
/// The four 16-byte rows of the default transform (`01268370`).
const DEFAULT_TRANSFORM: u32 = 0x0126_8370;
/// The `float` the new object's radius field gets in the second form (`011b153c`).
const DEFAULT_RADIUS: u32 = 0x011b_153c;
/// The vtable written to the second form of the object (`010c749c`).
const SECOND_FORM_VTABLE: u32 = 0x010c_749c;
/// The static buffer the transposed points are written to (`0126ec10`); the loop's cursor
/// starts one vector in.
const POINT_BUFFER: u32 = 0x0126_ec10;

/// `(x, y, z, c) * scale + [base]` as the SSE code computes it: `c` is the `float` at
/// `01011d78`, the four are scaled by `scale` broadcast (the packed value is the destination of
/// the `MULPS`) and the vector at `base` is added.
fn scaled_offset(e: &Engine, parts: [f32; 3], scale: f32, base: u32) -> Vec4 {
    let lane_three = f32::from_bits(e.mem.u32(LANE_THREE));
    let packed = [parts[0], parts[1], parts[2], lane_three];
    let scaled = mulps(packed, [scale; 4]);
    addps(scaled, load_vec(e, base))
}

/// The three `u16` at `address` as floats (`FILD` of each word).
fn quantised_parts(e: &Engine, address: u32) -> [f32; 3] {
    [
        f32::from(e.mem.u16(address)),
        f32::from(e.mem.u16(address.wrapping_add(2))),
        f32::from(e.mem.u16(address.wrapping_add(4))),
    ]
}

/// The mask `(1 << (32 - shift)) - 1` of a key's high part (`SHL` takes the count modulo 32).
fn high_mask(shift: u32) -> u32 {
    1u32.wrapping_shl(0x20u32.wrapping_sub(shift))
        .wrapping_sub(1)
}

/// The four 16-byte rows at `source` (`MOVAPS` loads, bits unchanged).
fn load_rows(e: &Engine, source: u32) -> [Vec4; 4] {
    std::array::from_fn(|row| load_vec(e, source.wrapping_add(16 * row as u32)))
}

/// Four rows of 16 bytes to `target`.
fn store_rows(e: &mut Engine, target: u32, rows: &[Vec4; 4]) {
    for (row, vector) in rows.iter().enumerate() {
        store_vec(e, target.wrapping_add(16 * row as u32), *vector);
    }
}

// Translated from 00e3c9c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, `RET 8`: a tail call. It overwrites the second stack word with `01269198` and
/// jumps to `hkpShapeCollection::calcContentStatistics` (`00d213e0`) with the first word and
/// `this` unchanged.
pub fn fn_00e3c9c0(e: &mut Engine, this: Ptr, first: u32, _unused_2: u32) {
    e.call(CALC_CONTENT_STATISTICS, &args![this, first, 0x0126_9198u32]);
}

// Translated from 00e3c9d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Virtual, `RET 8`: returns `0xf0`; neither stack word is read.
pub fn fn_00e3c9d0(_e: &mut Engine, _this: Ptr, _unused_1: u32, _unused_2: u32) -> u32 {
    0xf0
}

// Translated from 00e3c9e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, `RET 4`: copies the 16-byte vector at `source` to `this + 0xe0`.
pub fn fn_00e3c9e0(e: &mut Engine, this: Ptr, source: u32) {
    let value = load_vec(e, source);
    store_vec(e, this.addr().wrapping_add(0xe0), value);
}

// Translated from 00e3ca00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Virtual, `RET 4`: returns 0; the stack word is not read.
pub fn fn_00e3ca00(_e: &mut Engine, _this: Ptr, _unused_1: u32) -> u32 {
    0
}

// Translated from 00e3ca10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, `RET 8`. Writes 0 to `out`. A negative `index` gives `-1` when the count at
/// `+0x20` is zero and 0 otherwise. Otherwise walks the `u16` run lengths at `*(this + 0x28)`
/// (`*(this + 0x2c)` of them, adding them up) to find the run that holds `index + 1`; past the
/// last run the position is moved up to the next multiple of 3 from the end of the runs. Inside
/// a run that is not within 2 of its end, `out` gets the parity of the position's offset from
/// the start of the run, and a position that is within 2 of the end moves to the end of the run.
/// Returns the position when it is below the count at `+0x20`, else `-1`.
pub fn fn_00e3ca10(e: &mut Engine, this: Ptr, index: i32, out: u32) -> u32 {
    let this = this.addr();
    e.mem.set_u32(out, 0);
    if index < 0 {
        let any = e.mem.u32(this.wrapping_add(0x20)) != 0;
        return u32::from(any).wrapping_sub(1);
    }
    let count = e.mem.i32(this.wrapping_add(0x2c));
    let mut position = index.wrapping_add(1);
    let mut run = 0i32;
    let mut end = 0i32;
    let mut previous_end = 0i32;
    if count != 0 {
        let mut cursor = e.mem.u32(this.wrapping_add(0x28));
        end = i32::from(e.mem.u16(cursor));
        while position >= end {
            run += 1;
            cursor = cursor.wrapping_add(2);
            if run >= count {
                break;
            }
            let length = i32::from(e.mem.u16(cursor));
            previous_end = end;
            end = end.wrapping_add(length);
        }
    }
    if run >= count {
        let mut distance = position.wrapping_sub(end);
        if distance % 3 != 0 {
            loop {
                distance = distance.wrapping_add(1);
                position = position.wrapping_add(1);
                if distance % 3 == 0 {
                    break;
                }
            }
        }
    } else if position >= end.wrapping_sub(2) {
        position = end;
    } else {
        let parity = position.wrapping_sub(previous_end) & 1;
        e.mem.set_u32(out, parity as u32);
    }
    if position < e.mem.i32(this.wrapping_add(0x20)) {
        position as u32
    } else {
        u32::MAX
    }
}

// Translated from 00e3cad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fastcall (`this` in `ECX`, no stack words). With the count at `+0x60` nonzero: 0. Else with
/// the count at `+0x6c` nonzero: `1 << shift` (shift is the word at `+0xc`, taken modulo 32 as
/// `SHL` does). Else with the count at `+0x78` nonzero: `((1 << (32 - shift)) - 1) << shift`.
/// Otherwise `-1`.
pub fn fn_00e3cad0(e: &mut Engine, this: Ptr) -> u32 {
    let this = this.addr();
    if e.mem.u32(this.wrapping_add(0x60)) != 0 {
        return 0;
    }
    let shift = e.mem.u32(this.wrapping_add(0xc));
    if e.mem.u32(this.wrapping_add(0x6c)) != 0 {
        return 1u32.wrapping_shl(shift);
    }
    if e.mem.u32(this.wrapping_add(0x78)) != 0 {
        let mask = high_mask(shift);
        return mask.wrapping_shl(shift);
    }
    u32::MAX
}

/// How the search of `00e3cb10` ended: `Ok` with the finished key, `Err` with the key's low part
/// after the last failed attempt.
type SearchResult = Result<u32, u32>;

/// The loop of `00e3cb10` over the entries of `0x50` bytes from `index` on, with `low` the low
/// part of the key, `shift` the word at `+0xc` and `temp` the word the helper writes its second
/// result to.
fn search_entries(
    e: &mut Engine,
    this: u32,
    mut index: i32,
    shift: u32,
    mut low: u32,
    temp: u32,
) -> SearchResult {
    let count = e.mem.i32(this.wrapping_add(0x6c));
    if index >= count {
        return Err(low);
    }
    let base = e.mem.u32(this.wrapping_add(0x68));
    let mut entry = base
        .wrapping_add((index as u32).wrapping_mul(0x50))
        .wrapping_add(0x44);
    loop {
        let tag = e.mem.u16(entry);
        e.mem.set_u32(temp, 0);
        let source = if tag == 0xffff {
            entry.wrapping_sub(0x44)
        } else {
            u32::from(tag)
                .wrapping_mul(0x50)
                .wrapping_add(e.mem.u32(this.wrapping_add(0x68)))
        };
        let found = fn_00e3ca10(e, Ptr::new(source), low as i32, temp);
        if found != u32::MAX && (found as i32) <= e.mem.i32(this.wrapping_add(0x14)) {
            let parity = (e.mem.u32(temp) & 1).wrapping_shl(e.mem.u32(this.wrapping_add(8)));
            let high = (index.wrapping_add(1) as u32).wrapping_shl(shift);
            let kept = e.mem.u32(this.wrapping_add(0x10)) & found;
            return Ok(parity | high | kept);
        }
        index = index.wrapping_add(1);
        low = u32::MAX;
        entry = entry.wrapping_add(0x50);
        if index >= count {
            return Err(low);
        }
    }
}

// Translated from 00e3cb10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, `RET 4`: the next key after `key`. With `shift` the word at `+0xc`, the key splits
/// into a high part (`key >> shift`) and a low part (`key & [+0x14]`). A zero high part with
/// low part + 1 below the count at `+0x60` gives low + 1. Otherwise (unless the high part is
/// already the all-ones mask `(1 << (32 - shift)) - 1`) the entries of `0x50` bytes at
/// `*(this + 0x68)` from entry `high - 1` on (their count is at `+0x6c`) are asked, through
/// `00e3ca10` on the entry itself or on the entry its `u16` tag at `+0x44` names (`0xffff`: the
/// entry itself), for the next position after the low part; the first answer that is not `-1` and
/// within `[+0x14]` makes the key `(parity << [+8]) | ((entry + 1) << shift) | ([+0x10] & answer)`.
/// With no answer the key is `(mask << shift) | (low + 1)` when low + 1 is below the count at
/// `+0x78`, else `-1`.
pub fn fn_00e3cb10(e: &mut Engine, this: Ptr, key: u32) -> u32 {
    let this = this.addr();
    let shift = e.mem.u32(this.wrapping_add(0xc));
    let mut low = e.mem.u32(this.wrapping_add(0x14)) & key;
    let mut high = key.wrapping_shr(shift);
    if high == 0 {
        low = low.wrapping_add(1);
        if (low as i32) < e.mem.i32(this.wrapping_add(0x60)) {
            return low;
        }
        high = 1;
        low = u32::MAX;
    }
    let mask = high_mask(shift);
    if high != mask {
        let first = (high as i32).wrapping_sub(1);
        let outcome = e.with_stack(4, |e, temp| {
            search_entries(e, this, first, shift, low, temp.addr())
        });
        match outcome {
            Ok(key) => return key,
            Err(last_low) => low = last_low,
        }
    }
    low = low.wrapping_add(1);
    if (low as i32) < e.mem.i32(this.wrapping_add(0x78)) {
        mask.wrapping_shl(shift) | low
    } else {
        u32::MAX
    }
}

// Translated from 00e3cc20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cdecl, five words. `table` points at `{ *u16 triples, count }`; `base` and `factor` at
/// 16-byte vectors, `out` at two (the minimum, then the maximum). The largest x, y and z of the
/// `count / 3` triples are found; `out + 0x10` is `(max_x, max_y, max_z, c) * scale + [base]`
/// (`c` is the `float` at `01011d78`), `out` is `[base]`, and both are then multiplied
/// lane by lane by `[factor]`.
pub fn fn_00e3cc20(e: &mut Engine, table: u32, base: u32, scale: f32, out: u32, factor: u32) {
    let data = e.mem.u32(table);
    let groups = e.mem.i32(table.wrapping_add(4)) / 3;
    let (mut max_x, mut max_y, mut max_z) = (0u16, 0u16, 0u16);
    if groups > 0 {
        let mut cursor = data;
        for _ in 0..groups {
            max_x = max_x.max(e.mem.u16(cursor));
            max_y = max_y.max(e.mem.u16(cursor.wrapping_add(2)));
            max_z = max_z.max(e.mem.u16(cursor.wrapping_add(4)));
            cursor = cursor.wrapping_add(6);
        }
    }
    let parts = [f32::from(max_x), f32::from(max_y), f32::from(max_z)];
    let upper = scaled_offset(e, parts, scale, base);
    store_vec(e, out.wrapping_add(0x10), upper);
    let lower = load_vec(e, base);
    store_vec(e, out, lower);
    let times = load_vec(e, factor);
    let upper = mulps(load_vec(e, out.wrapping_add(0x10)), times);
    store_vec(e, out.wrapping_add(0x10), upper);
    let lower = load_vec(e, out);
    let times = load_vec(e, factor);
    store_vec(e, out, mulps(lower, times));
}

// Translated from 00e3cd20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, `RET 0xc`: `00e3cc20` on the quantised table at `this + 0x10` with `base = this`
/// and `scale` (a `float` passed through the x87 stack, so a signalling NaN is quieted).
pub fn fn_00e3cd20(e: &mut Engine, this: Ptr, scale: f32, out: u32, factor: u32) {
    let table = this.addr().wrapping_add(0x10);
    fn_00e3cc20(e, table, this.addr(), x87_quiet(scale), out, factor);
}

/// One corner of `00e3cd50`: the three `u16` at `*(this + 0x10) + 2 * index` as
/// `(x, y, z, c) * scale + [this]`, stored at `out`.
fn quantised_corner(e: &mut Engine, this: u32, index: u32, scale: f32, out: u32) {
    let table = e.mem.u32(this.wrapping_add(0x10));
    let parts = quantised_parts(e, table.wrapping_add(index.wrapping_mul(2)));
    let corner = scaled_offset(e, parts, scale, this);
    store_vec(e, out, corner);
}

// Translated from 00e3cd50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, `RET 0x1c`: three corners of the quantised box at `this`. For each of the three
/// indices into the `u16` triples at `*(this + 0x10)` the corner `(x, y, z, c) * scale + [this]`
/// (`c` is the `float` at `01011d78`) is stored at the matching output vector.
#[allow(clippy::too_many_arguments)]
pub fn fn_00e3cd50(
    e: &mut Engine,
    this: Ptr,
    first: u32,
    second: u32,
    third: u32,
    scale: f32,
    out_first: u32,
    out_second: u32,
    out_third: u32,
) {
    let this = this.addr();
    quantised_corner(e, this, first, scale, out_first);
    quantised_corner(e, this, second, scale, out_second);
    quantised_corner(e, this, third, scale, out_third);
}

// Translated from 00e3cf40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, `RET 4`: the bounding box of the `[+0x7c]` entries of `0x50` bytes at `*(this +
/// 0x78)`. `out` holds two vectors, the minimum (every lane starting from the `float` at
/// `0104da48`) and the maximum (from `0104e148`); each entry's box (`00e3cd20` with the `float`
/// at `+0x90` and the vector at `+0xe0`) is merged in with `MINPS` and `MAXPS`.
pub fn fn_00e3cf40(e: &mut Engine, this: Ptr, out: u32) {
    let this = this.addr();
    let minimum = f32::from_bits(e.mem.u32(BOX_MINIMUM_START));
    store_vec(e, out, [minimum; 4]);
    let maximum = f32::from_bits(e.mem.u32(BOX_MAXIMUM_START));
    store_vec(e, out.wrapping_add(0x10), [maximum; 4]);
    if e.mem.i32(this.wrapping_add(0x7c)) <= 0 {
        return;
    }
    let mut index = 0i32;
    let mut offset = 0u32;
    e.with_stack(0x20, |e, local| {
        let local = local.addr();
        loop {
            let entry = e.mem.u32(this.wrapping_add(0x78)).wrapping_add(offset);
            let scale = x87_quiet(e.mem.f32(this.wrapping_add(0x90)));
            fn_00e3cd20(e, Ptr::new(entry), scale, local, this.wrapping_add(0xe0));
            let merged = minps(load_vec(e, out), load_vec(e, local));
            store_vec(e, out, merged);
            index += 1;
            let merged = maxps(
                load_vec(e, out.wrapping_add(0x10)),
                load_vec(e, local.wrapping_add(0x10)),
            );
            store_vec(e, out.wrapping_add(0x10), merged);
            offset = offset.wrapping_add(0x50);
            if index >= e.mem.i32(this.wrapping_add(0x7c)) {
                break;
            }
        }
    });
}

// Translated from 00e3d060 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, `RET 8`: stores the 16-bit `value` for `key`. With `shift` the word at `+0x1c`, a
/// zero high part (`key >> shift`) writes at `*(this + 0x6c) + 0x10 * (key & [+0x20]) + 0xc`.
/// Otherwise the entry of `0x50` bytes `high - 1` at `*(this + 0x78)` is used, or the entry its
/// `u16` tag at `+0x44` names (`0xffff`: itself), and `value` is written at
/// `*(entry + 0x34) + 2 * (key & [+0x24])`.
pub fn fn_00e3d060(e: &mut Engine, this: Ptr, key: u32, value: u16) {
    let this = this.addr();
    let shift = e.mem.u32(this.wrapping_add(0x1c));
    let high = key.wrapping_shr(shift);
    if high == 0 {
        let slot = (e.mem.u32(this.wrapping_add(0x20)) & key).wrapping_shl(4);
        let table = e.mem.u32(this.wrapping_add(0x6c));
        e.mem
            .set_u16(slot.wrapping_add(table).wrapping_add(0xc), value);
        return;
    }
    let low = e.mem.u32(this.wrapping_add(0x24)) & key;
    let entries = e.mem.u32(this.wrapping_add(0x78));
    let mut entry = high.wrapping_sub(1);
    let tag = e.mem.u16(
        entry
            .wrapping_mul(0x50)
            .wrapping_add(entries)
            .wrapping_add(0x44),
    );
    if tag != 0xffff {
        entry = u32::from(tag);
    }
    let target = e.mem.u32(
        entry
            .wrapping_mul(0x50)
            .wrapping_add(entries)
            .wrapping_add(0x34),
    );
    e.mem
        .set_u16(target.wrapping_add(low.wrapping_mul(2)), value);
}

// Translated from 00e3d0d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, `RET 4`: stores the low byte of the stack word at `this + 0x2c`.
pub fn fn_00e3d0d0(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr().wrapping_add(0x2c), value);
}

// Translated from 00e3d250 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, `RET 8`: the box of a chunk record (`record`, with `u16` tags at `+0x44` and
/// `+0x46`) transformed into `out` (two vectors). `00e3cd20` runs on the entry the first tag
/// names (`0xffff`: the record itself) with the `float` at `+0x90` and the vector at `+0xe0`.
/// Unless the second tag is `0xffff`, the 64-byte transform it names (at `*(this + 0x54)`, 64
/// bytes per tag) with its translation row multiplied by the vector at `+0xe0` then transforms
/// both result vectors in place (`hkVector4::setTransformedPos`, `00c7f640`).
pub fn fn_00e3d250(e: &mut Engine, this: Ptr, record: u32, out: u32) {
    let this = this.addr();
    let scale = x87_quiet(e.mem.f32(this.wrapping_add(0x90)));
    let first_tag = e.mem.u16(record.wrapping_add(0x44));
    let owner = if first_tag == 0xffff {
        record
    } else {
        u32::from(first_tag)
            .wrapping_mul(0x50)
            .wrapping_add(e.mem.u32(this.wrapping_add(0x78)))
    };
    fn_00e3cd20(e, Ptr::new(owner), scale, out, this.wrapping_add(0xe0));
    let second_tag = e.mem.u16(record.wrapping_add(0x46));
    if second_tag == 0xffff {
        return;
    }
    let source = (u32::from(second_tag) << 6).wrapping_add(e.mem.u32(this.wrapping_add(0x54)));
    let scale_vector = load_vec(e, this.wrapping_add(0xe0));
    let mut rows = load_rows(e, source);
    rows[3] = mulps(rows[3], scale_vector);
    e.with_stack(0x40, |e, transform| {
        let transform = transform.addr();
        store_rows(e, transform, &rows);
        e.call(
            SET_TRANSFORMED_POS,
            &args![out.wrapping_add(0x10), transform, out.wrapping_add(0x10)],
        );
        e.call(SET_TRANSFORMED_POS, &args![out, transform, out]);
    });
}

// Translated from 00e3d380 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, `RET 0xc`: calls `hkAabbUtil::calcAabb_ov3` (`00d26230`, cdecl) with `first`, the
/// half extent `(hi - lo) * [010c71d0]` and the centre `(hi + lo) * [010c71d0]` of the box whose
/// corners are the vectors at `this + 0xa0` (`lo`) and `this + 0xb0` (`hi`), then `scale` (a
/// `float` through the x87 stack, so quieted) and `last`.
pub fn fn_00e3d380(e: &mut Engine, this: Ptr, first: u32, scale: f32, last: u32) {
    let this = this.addr();
    let lower = load_vec(e, this.wrapping_add(0xa0));
    let upper = load_vec(e, this.wrapping_add(0xb0));
    let lower_again = load_vec(e, this.wrapping_add(0xa0));
    let half = load_vec(e, HALF_VECTOR);
    let centre = mulps(addps(upper, lower), half);
    let upper_again = load_vec(e, this.wrapping_add(0xb0));
    let extent = mulps(subps(upper_again, lower_again), half);
    e.with_stack(0x20, |e, local| {
        let local = local.addr();
        store_vec(e, local, centre);
        store_vec(e, local.wrapping_add(0x10), extent);
        e.call(
            CALC_AABB,
            &args![
                first,
                local.wrapping_add(0x10),
                local,
                x87_quiet(scale),
                last
            ],
        );
    });
}

/// The first form of `00e3d3f0` (the key's high part is the all-ones mask): builds the shape
/// object at `out` with `00d59710` and the box of the points of the record the key names.
fn build_whole_shape(e: &mut Engine, this: u32, out: u32, index: u32) -> u32 {
    let mut transform = load_rows(e, DEFAULT_TRANSFORM);
    let result = if out != 0 {
        let radius = x87_quiet(e.mem.f32(this.wrapping_add(0x18)));
        e.call(BUILD_SHAPE_OBJECT, &args![out, radius]).u32()
    } else {
        0
    };
    let records = e.mem.u32(this.wrapping_add(0x74));
    let record = (index << 6).wrapping_add(records);
    let transform_tag = e.mem.u16(record.wrapping_add(0x36));
    if transform_tag != 0xffff {
        let source =
            (u32::from(transform_tag) << 6).wrapping_add(e.mem.u32(this.wrapping_add(0x44)));
        transform = load_rows(e, source);
    }
    let record_tag = e.mem.u16(record.wrapping_add(0x34));
    let leaf_index = if record_tag != 0xffff {
        u32::from(record_tag)
    } else {
        index
    };
    let leaf = (leaf_index << 6).wrapping_add(records);
    let mut cursor = e.mem.u32(leaf.wrapping_add(0x10));
    let point_count = e.mem.i32(leaf.wrapping_add(0x14));
    let triples = point_count / 3;
    let batches = triples.wrapping_add(3) >> 2;
    let mut low = [f32::from_bits(e.mem.u32(BOX_MINIMUM_START)); 4];
    let mut high = [f32::from_bits(e.mem.u32(BOX_MAXIMUM_START)); 4];
    if batches > 0 {
        let scale = f32::from_bits(e.mem.u32(this.wrapping_add(0x80)));
        e.with_stack(0x90, |e, frame| {
            let frame = frame.addr();
            let position = frame.wrapping_add(0x40);
            let slots = frame.wrapping_add(0x50);
            store_rows(e, frame, &transform);
            let mut output = POINT_BUFFER + 0x10;
            let mut counter = 0i32;
            for _ in 0..batches {
                for slot in 0..4u32 {
                    let parts = quantised_parts(e, cursor);
                    let point = scaled_offset(e, parts, scale, leaf);
                    store_vec(e, position, point);
                    let slot_address = slots.wrapping_add(16 * slot);
                    e.call(SET_TRANSFORMED_POS, &args![slot_address, frame, position]);
                    let seen = load_vec(e, position);
                    low = minps(low, seen);
                    high = maxps(high, seen);
                    if counter < triples.wrapping_sub(1) {
                        counter += 1;
                        cursor = cursor.wrapping_add(6);
                    }
                }
                let [first, second, third, fourth] = load_rows(e, slots);
                let low_pair_a = shufps(first, second, 0x44);
                let high_pair_a = shufps(first, second, 0xee);
                let low_pair_b = shufps(third, fourth, 0x44);
                let high_pair_b = shufps(third, fourth, 0xee);
                let xs = shufps(low_pair_a, low_pair_b, 0x88);
                let ys = shufps(low_pair_a, low_pair_b, 0xdd);
                let zs = shufps(high_pair_a, high_pair_b, 0x88);
                store_vec(e, output.wrapping_sub(0x10), xs);
                store_vec(e, output, ys);
                store_vec(e, output.wrapping_add(0x10), zs);
                output = output.wrapping_add(0x30);
                let ws = shufps(high_pair_a, high_pair_b, 0xdd);
                store_rows(e, slots, &[xs, ys, zs, ws]);
            }
        });
    }
    e.mem.set_u32(result.wrapping_add(0x40), POINT_BUFFER);
    e.mem.set_u32(result.wrapping_add(0x44), batches as u32);
    e.mem.set_u32(result.wrapping_add(0x48), 8);
    e.mem.set_u32(result.wrapping_add(0x4c), triples as u32);
    e.mem.set_u32(result.wrapping_add(0x50), leaf);
    let half = load_vec(e, HALF_VECTOR);
    let extent = mulps(subps(high, low), half);
    store_vec(e, result.wrapping_add(0x20), extent);
    let half = load_vec(e, HALF_VECTOR);
    let centre = mulps(addps(high, low), half);
    store_vec(e, result.wrapping_add(0x30), centre);
    result
}

/// The common start of the second form of `00e3d3f0`: initialises the object at `out` (when
/// not null) and copies the radius at `this + 0x18` and the byte at `this + 0x1c` into it.
fn start_second_form(e: &mut Engine, this: u32, out: u32) {
    if out != 0 {
        let radius = x87_quiet(f32::from_bits(e.mem.u32(DEFAULT_RADIUS)));
        e.mem.set_f32(out.wrapping_add(0x10), radius);
        e.mem.set_u16(out.wrapping_add(6), 1);
        e.mem.set_u32(out.wrapping_add(8), 0);
        e.mem.set_u32(out.wrapping_add(0xc), 3);
        e.mem.set_u32(out, SECOND_FORM_VTABLE);
        e.mem.set_u16(out.wrapping_add(0x14), 0);
        e.mem.set_u8(out.wrapping_add(0x16), 6);
        store_vec(e, out.wrapping_add(0x50), [0.0; 4]);
        e.mem.set_u8(out.wrapping_add(0x17), 0);
    }
    let radius = x87_quiet(e.mem.f32(this.wrapping_add(0x18)));
    let flag = e.mem.u8(this.wrapping_add(0x1c));
    e.mem.set_f32(out.wrapping_add(0x10), radius);
    e.mem.set_u8(out.wrapping_add(0x16), flag);
}

/// The second form of `00e3d3f0` with a zero high part: three vertices of the triangle entry
/// `index` scaled by the vector at `this + 0xd0`, stored at `out + 0x20`, `0x30` and `0x40`.
fn build_triangle_from_table(e: &mut Engine, this: u32, out: u32, index: u32) -> u32 {
    let table = e.mem.u32(this.wrapping_add(0x5c));
    let entry = table.wrapping_add(index << 4);
    let tail = e.mem.u16(entry.wrapping_add(0xc));
    let first = e.mem.u16(entry);
    e.mem.set_u16(out.wrapping_add(0x14), tail);
    let vertices = e.mem.u32(this.wrapping_add(0x50));
    let vertex = load_vec(e, vertices.wrapping_add(u32::from(first) << 4));
    let second = e.mem.u16(entry.wrapping_add(2));
    let third = e.mem.u16(entry.wrapping_add(4));
    store_vec(e, out.wrapping_add(0x20), vertex);
    let vertex = load_vec(e, vertices.wrapping_add(u32::from(second) << 4));
    store_vec(e, out.wrapping_add(0x30), vertex);
    let vertex = load_vec(e, vertices.wrapping_add(u32::from(third) << 4));
    store_vec(e, out.wrapping_add(0x40), vertex);
    let factor = load_vec(e, this.wrapping_add(0xd0));
    let product = mulps(factor, load_vec(e, out.wrapping_add(0x20)));
    store_vec(e, out.wrapping_add(0x20), product);
    let product = mulps(load_vec(e, out.wrapping_add(0x30)), factor);
    store_vec(e, out.wrapping_add(0x30), product);
    let product = mulps(load_vec(e, out.wrapping_add(0x40)), factor);
    store_vec(e, out.wrapping_add(0x40), product);
    out
}

/// The second form of `00e3d3f0` with a nonzero high part: the corners of a triangle of the
/// entry `high - 1`, transformed and scaled into `out`.
fn build_triangle_from_entry(
    e: &mut Engine,
    this: u32,
    out: u32,
    high: u32,
    index: u32,
    sub_index: u32,
) -> u32 {
    let low = index & e.mem.u32(this.wrapping_add(0x14));
    let first_entry = high.wrapping_sub(1);
    let entries = e.mem.u32(this.wrapping_add(0x68));
    let record = entries.wrapping_add(first_entry.wrapping_mul(0x50));
    let mut transform: [Vec4; 4] = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0; 4],
    ];
    let transform_tag = e.mem.u16(record.wrapping_add(0x46));
    if transform_tag != 0xffff {
        let source =
            (u32::from(transform_tag) << 6).wrapping_add(e.mem.u32(this.wrapping_add(0x44)));
        transform = load_rows(e, source);
    }
    let entry_tag = e.mem.u16(record.wrapping_add(0x44));
    let entry_index = if entry_tag != 0xffff {
        u32::from(entry_tag)
    } else {
        first_entry
    };
    let scale = x87_quiet(e.mem.f32(this.wrapping_add(0x80)));
    let entry = entry_index
        .wrapping_mul(0x50)
        .wrapping_add(e.mem.u32(this.wrapping_add(0x68)));
    let words = e.mem.u32(entry.wrapping_add(0x34));
    let tail = e.mem.u16(words.wrapping_add(low.wrapping_mul(2)));
    let triangles = e.mem.u32(entry.wrapping_add(0x1c));
    e.mem.set_u16(out.wrapping_add(0x14), tail);
    let at = triangles.wrapping_add(low.wrapping_mul(2));
    let third = u32::from(e.mem.u16(at.wrapping_add(4))).wrapping_mul(3);
    let second = u32::from(e.mem.u16(at.wrapping_add(2))).wrapping_mul(3);
    let first = u32::from(e.mem.u16(at)).wrapping_mul(3);
    let (corners, factor) = e.with_stack(0x80, |e, frame| {
        let frame = frame.addr();
        let (matrix, third_corner, second_corner, first_corner) = (
            frame,
            frame.wrapping_add(0x40),
            frame.wrapping_add(0x50),
            frame.wrapping_add(0x60),
        );
        store_rows(e, matrix, &transform);
        fn_00e3cd50(
            e,
            Ptr::new(entry),
            first,
            second,
            third,
            scale,
            first_corner,
            second_corner,
            third_corner,
        );
        for corner in [first_corner, second_corner, third_corner] {
            e.call(SET_TRANSFORMED_POS, &args![corner, matrix, corner]);
        }
        let corners = [
            load_vec(e, first_corner),
            load_vec(e, second_corner),
            load_vec(e, third_corner),
        ];
        (corners, load_vec(e, this.wrapping_add(0xd0)))
    });
    let first_slot = sub_index.wrapping_add(1).wrapping_shl(5);
    let third_slot = (sub_index ^ 1).wrapping_add(1).wrapping_shl(5);
    store_vec(e, first_slot.wrapping_add(out), mulps(corners[0], factor));
    store_vec(e, out.wrapping_add(0x30), mulps(corners[1], factor));
    store_vec(e, third_slot.wrapping_add(out), mulps(corners[2], factor));
    out
}

// Translated from 00e3d3f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, `RET 8`: builds the shape of `key` into the object at `out` and returns it. With
/// `shift` the word at `+0xc`, the key is split into `high = key >> shift`, `index = key &
/// [+0x10]` and `sub_index = index >> [+8]` (arithmetic). When `high` is the all-ones mask
/// `(1 << (32 - shift)) - 1`, `00d59710` builds the object (with the `float` at `+0x18`) and the
/// box of the points of the record `index` (their `u16` triples, 4 points per batch, transformed
/// by the record's matrix and written transposed into the static buffer at `0126ec10`) is
/// stored into it. Otherwise the object is set up (vtable `010c749c`) and receives its three
/// vertices, either from the vertex table (zero `high`) or from the triangle entry `high - 1`.
/// A null `out` makes the exe store through address 0, which the memory model stops.
pub fn fn_00e3d3f0(e: &mut Engine, this: Ptr, key: u32, out: u32) -> u32 {
    let this = this.addr();
    let shift = e.mem.u32(this.wrapping_add(0xc));
    let high = key.wrapping_shr(shift);
    let index = e.mem.u32(this.wrapping_add(0x10)) & key;
    let sub_index = (index as i32).wrapping_shr(e.mem.u32(this.wrapping_add(8))) as u32;
    let mask = high_mask(shift);
    if high == mask {
        return build_whole_shape(e, this, out, index);
    }
    start_second_form(e, this, out);
    if high == 0 {
        build_triangle_from_table(e, this, out, index)
    } else {
        build_triangle_from_entry(e, this, out, high, index, sub_index)
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e3_c9c0, fn_00e3c9c0(Ptr, u32, u32)),
        entry!(0x00e3_c9d0, fn_00e3c9d0(Ptr, u32, u32) -> u32),
        entry!(0x00e3_c9e0, fn_00e3c9e0(Ptr, u32)),
        entry!(0x00e3_ca00, fn_00e3ca00(Ptr, u32) -> u32),
        entry!(0x00e3_ca10, fn_00e3ca10(Ptr, i32, u32) -> u32),
        entry!(0x00e3_cad0, fn_00e3cad0(Ptr) -> u32),
        entry!(0x00e3_cb10, fn_00e3cb10(Ptr, u32) -> u32),
        entry!(0x00e3_cc20, fn_00e3cc20(u32, u32, f32, u32, u32)),
        entry!(0x00e3_cd20, fn_00e3cd20(Ptr, f32, u32, u32)),
        entry!(
            0x00e3_cd50,
            fn_00e3cd50(Ptr, u32, u32, u32, f32, u32, u32, u32)
        ),
        entry!(0x00e3_cf40, fn_00e3cf40(Ptr, u32)),
        entry!(0x00e3_d060, fn_00e3d060(Ptr, u32, u16)),
        entry!(0x00e3_d0d0, fn_00e3d0d0(Ptr, u8)),
        entry!(0x00e3_d250, fn_00e3d250(Ptr, u32, u32)),
        entry!(0x00e3_d380, fn_00e3d380(Ptr, u32, f32, u32)),
        entry!(0x00e3_d3f0, fn_00e3d3f0(Ptr, u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// The argument words of each call a double saw.
    type Calls = Rc<RefCell<Vec<Vec<u32>>>>;
    /// What the transform double saw: the argument words and the four rows of the transform.
    type TransformCalls = Rc<RefCell<Vec<(Vec<u32>, [Vec4; 4])>>>;
    /// What the box double saw: the argument words, the extent and the centre.
    type BoxCalls = Rc<RefCell<Vec<(Vec<u32>, Vec4, Vec4)>>>;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e
    }

    fn put_words(e: &mut Engine, address: u32, values: &[u16]) {
        for (index, value) in values.iter().enumerate() {
            e.mem.set_u16(address + 2 * index as u32, *value);
        }
    }

    fn put_u32s(e: &mut Engine, address: u32, values: &[(u32, u32)]) {
        for (offset, value) in values {
            e.mem.set_u32(address + offset, *value);
        }
    }

    /// A table `{ pointer to the words, count }` at a fresh block, with the words.
    fn table(e: &mut Engine, values: &[u16], count: u32) -> u32 {
        let words = e.mem.alloc(2 * values.len() as u32 + 8);
        put_words(e, words, values);
        let table = e.mem.alloc(8);
        put_u32s(e, table, &[(0, words), (4, count)]);
        table
    }

    /// The double of `hkVector4::setTransformedPos`: writes `pos + row 3` into `this` and
    /// records the argument words and the four rows of the transform.
    fn transform_double(e: &mut Engine) -> TransformCalls {
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(SET_TRANSFORMED_POS, move |e, a| {
            let rows = load_rows(e, a[1]);
            let pos = load_vec(e, a[2]);
            let moved = std::array::from_fn(|lane| pos[lane] + rows[3][lane]);
            store_vec(e, a[0], moved);
            log.borrow_mut().push((a.to_vec(), rows));
            Ret::default()
        });
        seen
    }

    #[test]
    fn collection_statistics_tail_call_replaces_the_second_word() {
        let mut e = engine();
        let seen: Calls = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(CALC_CONTENT_STATISTICS, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        e.call(0x00e3_c9c0, &args![0x1000u32, 7u32, 9u32]);
        assert_eq!(*seen.borrow(), vec![vec![0x1000, 7, 0x0126_9198]]);
    }

    #[test]
    fn constant_virtuals_return_their_value() {
        let mut e = engine();
        assert_eq!(e.call(0x00e3_c9d0, &args![0u32, 1u32, 2u32]).u32(), 0xf0);
        assert_eq!(e.call(0x00e3_ca00, &args![0u32, 1u32]).u32(), 0);
    }

    #[test]
    fn vector_is_copied_to_offset_0xe0() {
        let mut e = engine();
        let this = e.mem.alloc(0x100);
        let source = e.mem.alloc(16);
        store_vec(&mut e, source, [1.0, f32::from_bits(0x7f80_0001), 3.0, 4.0]);
        e.call(0x00e3_c9e0, &args![this, source]);
        assert_eq!(
            load_vec(&e, this + 0xe0).map(f32::to_bits),
            [
                1.0f32.to_bits(),
                0x7f80_0001,
                3.0f32.to_bits(),
                4.0f32.to_bits()
            ]
        );
    }

    /// `00e3ca10` on a table of runs `[4, 6]` with the count 20: the result and the word
    /// written to `out`.
    fn runs(e: &mut Engine, this: u32, index: i32) -> (u32, u32) {
        let out = e.mem.alloc(4);
        e.mem.set_u32(out, 0xdead);
        let result = e.call(0x00e3_ca10, &args![this, index, out]).u32();
        (result, e.mem.u32(out))
    }

    fn run_table(e: &mut Engine) -> u32 {
        let this = e.mem.alloc(0x40);
        let words = e.mem.alloc(8);
        put_words(e, words, &[4, 6]);
        put_u32s(e, this, &[(0x20, 20), (0x28, words), (0x2c, 2)]);
        this
    }

    #[test]
    fn position_inside_a_run_gives_the_parity_or_the_run_end() {
        let mut e = engine();
        let this = run_table(&mut e);
        assert_eq!(runs(&mut e, this, 0), (1, 1));
        assert_eq!(runs(&mut e, this, 1), (4, 0));
        assert_eq!(runs(&mut e, this, 4), (5, 1));
        assert_eq!(runs(&mut e, this, 7), (10, 0));
    }

    #[test]
    fn position_past_the_runs_moves_to_a_multiple_of_three() {
        let mut e = engine();
        let this = run_table(&mut e);
        assert_eq!(runs(&mut e, this, 9), (10, 0));
        assert_eq!(runs(&mut e, this, 10), (13, 0));
        assert_eq!(runs(&mut e, this, 18), (19, 0));
        assert_eq!(runs(&mut e, this, 19), (u32::MAX, 0));
        e.mem.set_u32(this + 0x2c, 0);
        assert_eq!(runs(&mut e, this, 0), (3, 0));
    }

    #[test]
    fn negative_index_depends_on_the_count() {
        let mut e = engine();
        let this = run_table(&mut e);
        assert_eq!(runs(&mut e, this, -1), (0, 0));
        e.mem.set_u32(this + 0x20, 0);
        assert_eq!(runs(&mut e, this, -5), (u32::MAX, 0));
    }

    #[test]
    fn mask_function_follows_the_counts() {
        let mut e = engine();
        let this = e.mem.alloc(0x80);
        put_u32s(&mut e, this, &[(0x60, 1), (0x6c, 1), (0x78, 1), (0xc, 5)]);
        assert_eq!(e.call(0x00e3_cad0, &args![this]).u32(), 0);
        e.mem.set_u32(this + 0x60, 0);
        assert_eq!(e.call(0x00e3_cad0, &args![this]).u32(), 32);
        e.mem.set_u32(this + 0xc, 33);
        assert_eq!(e.call(0x00e3_cad0, &args![this]).u32(), 2);
        e.mem.set_u32(this + 0x6c, 0);
        e.mem.set_u32(this + 0xc, 28);
        assert_eq!(e.call(0x00e3_cad0, &args![this]).u32(), 0xf000_0000);
        e.mem.set_u32(this + 0x78, 0);
        assert_eq!(e.call(0x00e3_cad0, &args![this]).u32(), u32::MAX);
    }

    /// An object for `00e3cb10` with two entries (tag `0xffff`, so each asks itself), the first
    /// holding the runs `[4, 6]` with the count 100, the second a count of `second_count`.
    fn key_table(e: &mut Engine, first_count: u32, second_count: u32) -> u32 {
        let this = e.mem.alloc(0x80);
        let entries = e.mem.alloc(0xa0);
        let words = e.mem.alloc(8);
        put_words(e, words, &[4, 6]);
        put_u32s(
            e,
            this,
            &[
                (0xc, 4),
                (0x14, 0xf),
                (0x10, 0xf),
                (8, 20),
                (0x68, entries),
                (0x6c, 2),
                (0x60, 4),
                (0x78, 5),
            ],
        );
        put_u32s(
            e,
            entries,
            &[
                (0x20, first_count),
                (0x28, words),
                (0x2c, 2),
                (0x50 + 0x20, second_count),
                (0x50 + 0x28, words),
                (0x50 + 0x2c, 2),
            ],
        );
        put_words(e, entries + 0x44, &[0xffff]);
        put_words(e, entries + 0x50 + 0x44, &[0xffff]);
        this
    }

    #[test]
    fn next_key_with_a_zero_high_part_adds_one_below_the_limit() {
        let mut e = engine();
        let this = key_table(&mut e, 100, 100);
        e.mem.set_u32(this + 0x60, 10);
        assert_eq!(e.call(0x00e3_cb10, &args![this, 3u32]).u32(), 4);
    }

    #[test]
    fn next_key_asks_the_entry_for_the_next_position() {
        let mut e = engine();
        let this = key_table(&mut e, 100, 100);
        // High part 1 (entry 0), low part 0: the entry answers position 1 with parity 1.
        assert_eq!(
            e.call(0x00e3_cb10, &args![this, 0x10u32]).u32(),
            0x0010_0011
        );
    }

    #[test]
    fn next_key_goes_to_the_next_entry_after_a_failed_one() {
        let mut e = engine();
        let this = key_table(&mut e, 1, 100);
        // The first entry has a count of 1 and fails; the low part becomes -1, which the second
        // entry answers with 0 (its count is not zero).
        assert_eq!(e.call(0x00e3_cb10, &args![this, 0x10u32]).u32(), 0x20);
    }

    #[test]
    fn next_key_follows_an_entry_tag() {
        let mut e = engine();
        let this = key_table(&mut e, 1, 100);
        let entries = e.mem.u32(this + 0x68);
        put_words(&mut e, entries + 0x44, &[1]);
        assert_eq!(
            e.call(0x00e3_cb10, &args![this, 0x10u32]).u32(),
            0x0010_0011
        );
    }

    #[test]
    fn next_key_without_an_answer_uses_the_mask_and_the_third_count() {
        let mut e = engine();
        let this = key_table(&mut e, 1, 0);
        assert_eq!(
            e.call(0x00e3_cb10, &args![this, 0x10u32]).u32(),
            0xffff_fff0
        );
        // The high part is already the mask: no entry is asked.
        e.mem.set_u32(this + 0xc, 28);
        e.mem.set_u32(this + 0x14, 0xf);
        assert_eq!(
            e.call(0x00e3_cb10, &args![this, 0xf000_0002u32]).u32(),
            0xf000_0003
        );
        e.mem.set_u32(this + 0x78, 3);
        assert_eq!(
            e.call(0x00e3_cb10, &args![this, 0xf000_0002u32]).u32(),
            u32::MAX
        );
        // A zero high part below the first count that does not fit falls to the entries.
        e.mem.set_u32(this + 0xc, 4);
        e.mem.set_u32(this + 0x14, 0xff);
        e.mem.set_u32(this + 0x6c, 0);
        e.mem.set_u32(this + 0x78, 3);
        assert_eq!(e.call(0x00e3_cb10, &args![this, 4u32]).u32(), 0xffff_fff0);
    }

    #[test]
    fn quantised_box_takes_the_largest_triple_values() {
        let mut e = engine();
        e.mem.set_f32(LANE_THREE, 8.0);
        let table = table(&mut e, &[1, 5, 2, 3, 4, 9], 7);
        let base = e.mem.alloc(16);
        store_vec(&mut e, base, [10.0, 20.0, 30.0, 40.0]);
        let factor = e.mem.alloc(16);
        store_vec(&mut e, factor, [2.0, 3.0, 4.0, 5.0]);
        let out = e.mem.alloc(32);
        e.call(0x00e3_cc20, &args![table, base, 0.5f32, out, factor]);
        assert_eq!(load_vec(&e, out), [20.0, 60.0, 120.0, 200.0]);
        assert_eq!(load_vec(&e, out + 16), [23.0, 67.5, 138.0, 220.0]);
        // No complete triple: the maximum stays at zero.
        e.mem.set_u32(table + 4, 2);
        e.call(0x00e3_cc20, &args![table, base, 0.5f32, out, factor]);
        assert_eq!(load_vec(&e, out + 16), [20.0, 60.0, 120.0, 220.0]);
    }

    /// A quantised box at a fresh block: the base vector, the table (`+0x10`, `+0x14`).
    fn quantised_box(e: &mut Engine, base: Vec4, values: &[u16], count: u32) -> u32 {
        let words = e.mem.alloc(2 * values.len() as u32 + 8);
        put_words(e, words, values);
        let this = e.mem.alloc(0x50);
        store_vec(e, this, base);
        put_u32s(e, this, &[(0x10, words), (0x14, count)]);
        this
    }

    #[test]
    fn box_function_reads_the_table_inside_the_object() {
        let mut e = engine();
        e.mem.set_f32(LANE_THREE, 8.0);
        let this = quantised_box(&mut e, [1.0, 2.0, 3.0, 4.0], &[2, 6, 4], 3);
        let factor = e.mem.alloc(16);
        store_vec(&mut e, factor, [1.0, 1.0, 2.0, 2.0]);
        let out = e.mem.alloc(32);
        e.call(0x00e3_cd20, &args![this, 2.0f32, out, factor]);
        assert_eq!(load_vec(&e, out), [1.0, 2.0, 6.0, 8.0]);
        assert_eq!(load_vec(&e, out + 16), [5.0, 14.0, 22.0, 40.0]);
    }

    #[test]
    fn three_corners_are_scaled_and_offset() {
        let mut e = engine();
        e.mem.set_f32(LANE_THREE, 8.0);
        let values: Vec<u16> = (1..=12).collect();
        let this = quantised_box(&mut e, [100.0, 200.0, 300.0, 400.0], &values, 12);
        let outs = e.mem.alloc(0x30);
        e.call(
            0x00e3_cd50,
            &args![this, 3u32, 0u32, 6u32, 2.0f32, outs, outs + 16, outs + 32],
        );
        assert_eq!(load_vec(&e, outs), [108.0, 210.0, 312.0, 416.0]);
        assert_eq!(load_vec(&e, outs + 16), [102.0, 204.0, 306.0, 416.0]);
        assert_eq!(load_vec(&e, outs + 32), [114.0, 216.0, 318.0, 416.0]);
    }

    /// An object for `00e3cf40`/`00e3d250` with two entries of `0x50` bytes.
    fn box_owner(e: &mut Engine) -> u32 {
        let this = e.mem.alloc(0x100);
        let entries = e.mem.alloc(0xa0);
        let first = [(0u32, [0.0f32; 4]), (0x50, [10.0, 10.0, 10.0, 0.0])];
        for (offset, base) in first {
            store_vec(e, entries + offset, base);
        }
        let first_words = e.mem.alloc(8);
        put_words(e, first_words, &[1, 2, 3]);
        let second_words = e.mem.alloc(8);
        put_words(e, second_words, &[2, 1, 0]);
        put_u32s(
            e,
            entries,
            &[
                (0x10, first_words),
                (0x14, 3),
                (0x50 + 0x10, second_words),
                (0x50 + 0x14, 3),
            ],
        );
        put_u32s(
            e,
            this,
            &[(0x78, entries), (0x7c, 2), (0x90, 1.0f32.to_bits())],
        );
        store_vec(e, this + 0xe0, [1.0; 4]);
        this
    }

    #[test]
    fn owner_box_merges_every_entry() {
        let mut e = engine();
        e.mem.set_f32(BOX_MINIMUM_START, 1.0e30);
        e.mem.set_f32(BOX_MAXIMUM_START, -1.0e30);
        let this = box_owner(&mut e);
        let out = e.mem.alloc(32);
        e.call(0x00e3_cf40, &args![this, out]);
        assert_eq!(load_vec(&e, out), [0.0, 0.0, 0.0, 0.0]);
        assert_eq!(load_vec(&e, out + 16), [12.0, 11.0, 10.0, 0.0]);
        // No entries: the starting values stay.
        e.mem.set_u32(this + 0x7c, 0);
        e.call(0x00e3_cf40, &args![this, out]);
        assert_eq!(load_vec(&e, out), [1.0e30; 4]);
        assert_eq!(load_vec(&e, out + 16), [-1.0e30; 4]);
    }

    #[test]
    fn value_is_stored_in_the_table_or_an_entry() {
        let mut e = engine();
        let this = e.mem.alloc(0x100);
        let table = e.mem.alloc(0x80);
        let entries = e.mem.alloc(0xa0);
        let first_target = e.mem.alloc(0x20);
        let second_target = e.mem.alloc(0x20);
        put_u32s(
            &mut e,
            this,
            &[
                (0x1c, 4),
                (0x20, 0xf),
                (0x24, 0xf),
                (0x6c, table),
                (0x78, entries),
            ],
        );
        put_u32s(
            &mut e,
            entries,
            &[(0x34, first_target), (0x50 + 0x34, second_target)],
        );
        put_words(&mut e, entries + 0x44, &[0xffff]);
        put_words(&mut e, entries + 0x50 + 0x44, &[0xffff]);
        e.call(0x00e3_d060, &args![this, 3u32, 0x1234u16]);
        assert_eq!(e.mem.u16(table + 0x30 + 0xc), 0x1234);
        e.call(0x00e3_d060, &args![this, 0x25u32, 0x4321u16]);
        assert_eq!(e.mem.u16(second_target + 10), 0x4321);
        // A tag names another entry.
        put_words(&mut e, entries + 0x50 + 0x44, &[0]);
        e.call(0x00e3_d060, &args![this, 0x23u32, 0x1111u16]);
        assert_eq!(e.mem.u16(first_target + 6), 0x1111);
    }

    #[test]
    fn byte_is_stored_at_0x2c() {
        let mut e = engine();
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x2c, 0x1234_5678);
        e.call(0x00e3_d0d0, &args![this, 0x1_00ab_u32]);
        assert_eq!(e.mem.u32(this + 0x2c), 0x1234_56ab);
    }

    #[test]
    fn chunk_box_without_a_second_tag_only_builds_the_box() {
        let mut e = engine();
        e.mem.set_f32(LANE_THREE, 8.0);
        let seen = transform_double(&mut e);
        let this = box_owner(&mut e);
        e.mem.set_u32(this + 0x90, 2.0f32.to_bits());
        let record = quantised_box(&mut e, [1.0, 1.0, 1.0, 1.0], &[1, 2, 3], 3);
        put_words(&mut e, record + 0x44, &[0xffff, 0xffff]);
        let out = e.mem.alloc(32);
        e.call(0x00e3_d250, &args![this, record, out]);
        assert_eq!(load_vec(&e, out), [1.0, 1.0, 1.0, 1.0]);
        assert_eq!(load_vec(&e, out + 16), [3.0, 5.0, 7.0, 17.0]);
        assert!(seen.borrow().is_empty());
    }

    #[test]
    fn chunk_box_with_a_transform_tag_transforms_both_vectors() {
        let mut e = engine();
        let seen = transform_double(&mut e);
        let this = box_owner(&mut e);
        e.mem.set_u32(this + 0x90, 1.0f32.to_bits());
        store_vec(&mut e, this + 0xe0, [2.0, 2.0, 2.0, 2.0]);
        // The record names the second entry for the box and transform 1.
        let transforms = e.mem.alloc(0x80);
        let rows = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [5.0, 6.0, 7.0, 8.0],
        ];
        store_rows(&mut e, transforms + 0x40, &rows);
        e.mem.set_u32(this + 0x54, transforms);
        let record = e.mem.alloc(0x50);
        put_words(&mut e, record + 0x44, &[1, 1]);
        let out = e.mem.alloc(32);
        e.call(0x00e3_d250, &args![this, record, out]);
        let calls = seen.borrow();
        assert_eq!(calls.len(), 2);
        assert_eq!(calls[0].0[0], out + 16);
        assert_eq!(calls[1].0[0], out);
        // The translation row was multiplied by [this + 0xe0].
        assert_eq!(calls[0].1[3], [10.0, 12.0, 14.0, 16.0]);
        assert_eq!(calls[0].1[0], rows[0]);
        // The second entry has a base of (10, 10, 10, 0): minimum (20, 20, 20, 0) after the
        // factor, plus the translation.
        assert_eq!(load_vec(&e, out), [30.0, 32.0, 34.0, 16.0]);
    }

    #[test]
    fn aabb_call_gets_centre_and_extent() {
        let mut e = engine();
        store_vec(&mut e, HALF_VECTOR, [0.5; 4]);
        let this = e.mem.alloc(0x100);
        store_vec(&mut e, this + 0xa0, [1.0, 1.0, 1.0, 1.0]);
        store_vec(&mut e, this + 0xb0, [3.0, 5.0, 7.0, 9.0]);
        let seen: BoxCalls = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(CALC_AABB, move |e, a| {
            log.borrow_mut()
                .push((a.to_vec(), load_vec(e, a[1]), load_vec(e, a[2])));
            Ret::default()
        });
        e.call(0x00e3_d380, &args![this, 0x77u32, 1.5f32, 0x99u32]);
        let calls = seen.borrow();
        assert_eq!(calls.len(), 1);
        let (words, extent, centre) = &calls[0];
        assert_eq!(words[0], 0x77);
        assert_eq!(words[3], 1.5f32.to_bits());
        assert_eq!(words[4], 0x99);
        assert_eq!(*extent, [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(*centre, [2.0, 3.0, 4.0, 5.0]);
    }

    /// An object for `00e3d3f0` with a 4-bit high part (`shift` 28, so the mask is 15).
    fn shape_source(e: &mut Engine) -> u32 {
        let this = e.mem.alloc(0x100);
        put_u32s(
            e,
            this,
            &[
                (0xc, 28),
                (0x10, 0xff),
                (0x14, 0xff),
                (0x18, 0.25f32.to_bits()),
                (0x80, 1.0f32.to_bits()),
            ],
        );
        e.mem.set_u8(this + 0x1c, 3);
        this
    }

    #[test]
    fn whole_shape_writes_the_transposed_points_and_the_box() {
        let mut e = engine();
        store_vec(&mut e, HALF_VECTOR, [0.5; 4]);
        e.mem.set_f32(BOX_MINIMUM_START, 1.0e30);
        e.mem.set_f32(BOX_MAXIMUM_START, -1.0e30);
        let seen = transform_double(&mut e);
        let built: Calls = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&built);
        e.register_double(BUILD_SHAPE_OBJECT, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        let this = shape_source(&mut e);
        let records = e.mem.alloc(0x100);
        e.mem.set_u32(this + 0x74, records);
        // Record 1 (key 0xf0000001): no tags, the points are its own.
        let leaf = records + 0x40;
        store_vec(&mut e, leaf, [100.0, 100.0, 100.0, 0.0]);
        let words = e.mem.alloc(16);
        put_words(&mut e, words, &[1, 2, 3, 4, 5, 6]);
        put_u32s(&mut e, leaf, &[(0x10, words), (0x14, 6)]);
        put_words(&mut e, leaf + 0x34, &[0xffff, 0xffff]);
        let out = e.mem.alloc(0x80);
        let result = e.call(0x00e3_d3f0, &args![this, 0xf000_0001u32, out]).u32();
        assert_eq!(result, out);
        assert_eq!(*built.borrow(), vec![vec![out, 0.25f32.to_bits()]]);
        assert_eq!(seen.borrow().len(), 4);
        assert_eq!(load_vec(&e, POINT_BUFFER), [101.0, 104.0, 104.0, 104.0]);
        assert_eq!(
            load_vec(&e, POINT_BUFFER + 0x10),
            [102.0, 105.0, 105.0, 105.0]
        );
        assert_eq!(
            load_vec(&e, POINT_BUFFER + 0x20),
            [103.0, 106.0, 106.0, 106.0]
        );
        assert_eq!(e.mem.u32(out + 0x40), POINT_BUFFER);
        assert_eq!(e.mem.u32(out + 0x44), 1);
        assert_eq!(e.mem.u32(out + 0x48), 8);
        assert_eq!(e.mem.u32(out + 0x4c), 2);
        assert_eq!(e.mem.u32(out + 0x50), leaf);
        assert_eq!(load_vec(&e, out + 0x20), [1.5, 1.5, 1.5, 0.0]);
        assert_eq!(load_vec(&e, out + 0x30), [102.5, 103.5, 104.5, 0.0]);
    }

    #[test]
    fn whole_shape_with_no_points_keeps_the_starting_box() {
        let mut e = engine();
        store_vec(&mut e, HALF_VECTOR, [0.5; 4]);
        e.mem.set_f32(BOX_MINIMUM_START, 4.0);
        e.mem.set_f32(BOX_MAXIMUM_START, 2.0);
        e.register(BUILD_SHAPE_OBJECT, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        let this = shape_source(&mut e);
        let records = e.mem.alloc(0x100);
        e.mem.set_u32(this + 0x74, records);
        let leaf = records + 0x40;
        put_words(&mut e, leaf + 0x34, &[0xffff, 0xffff]);
        let out = e.mem.alloc(0x80);
        e.call(0x00e3_d3f0, &args![this, 0xf000_0001u32, out]);
        assert_eq!(e.mem.u32(out + 0x44), 0);
        assert_eq!(load_vec(&e, out + 0x20), [-1.0; 4]);
        assert_eq!(load_vec(&e, out + 0x30), [3.0; 4]);
    }

    #[test]
    fn triangle_from_the_table_scales_its_vertices() {
        let mut e = engine();
        let this = shape_source(&mut e);
        let table = e.mem.alloc(0x100);
        let vertices = e.mem.alloc(0x100);
        put_u32s(&mut e, this, &[(0x5c, table), (0x50, vertices)]);
        store_vec(&mut e, this + 0xd0, [2.0, 3.0, 4.0, 5.0]);
        put_words(&mut e, table + 0x50, &[2, 0, 1, 0, 0, 0, 0x7777]);
        store_vec(&mut e, vertices, [1.0, 1.0, 1.0, 1.0]);
        store_vec(&mut e, vertices + 16, [2.0, 2.0, 2.0, 2.0]);
        store_vec(&mut e, vertices + 32, [3.0, 3.0, 3.0, 3.0]);
        let out = e.mem.alloc(0x80);
        let result = e.call(0x00e3_d3f0, &args![this, 5u32, out]).u32();
        assert_eq!(result, out);
        assert_eq!(e.mem.u32(out), SECOND_FORM_VTABLE);
        assert_eq!(e.mem.u16(out + 6), 1);
        assert_eq!(e.mem.u32(out + 0xc), 3);
        assert_eq!(e.mem.f32(out + 0x10), 0.25);
        assert_eq!(e.mem.u8(out + 0x16), 3);
        assert_eq!(e.mem.u16(out + 0x14), 0x7777);
        assert_eq!(load_vec(&e, out + 0x20), [6.0, 9.0, 12.0, 15.0]);
        assert_eq!(load_vec(&e, out + 0x30), [2.0, 3.0, 4.0, 5.0]);
        assert_eq!(load_vec(&e, out + 0x40), [4.0, 6.0, 8.0, 10.0]);
    }

    #[test]
    fn triangle_from_an_entry_is_transformed_and_placed_by_the_sub_index() {
        let mut e = engine();
        e.mem.set_f32(DEFAULT_RADIUS, 0.5);
        let seen = transform_double(&mut e);
        let this = shape_source(&mut e);
        let entries = e.mem.alloc(0x100);
        e.mem.set_u32(this + 0x68, entries);
        e.mem.set_u32(this + 8, 1);
        store_vec(&mut e, this + 0xd0, [2.0; 4]);
        // High part 2 selects entry 1, whose tags are both 0xffff.
        let entry = entries + 0x50;
        put_words(&mut e, entry + 0x44, &[0xffff, 0xffff]);
        let values: Vec<u16> = (1..=30).collect();
        let quantised = e.mem.alloc(80);
        put_words(&mut e, quantised, &values);
        let triangles = e.mem.alloc(32);
        put_words(&mut e, triangles + 6, &[0, 1, 2]);
        let tails = e.mem.alloc(32);
        put_words(&mut e, tails + 6, &[0x4242]);
        put_u32s(
            &mut e,
            entry,
            &[(0x10, quantised), (0x1c, triangles), (0x34, tails)],
        );
        let out = e.mem.alloc(0x80);
        // Key: high 2, low 3; the sub-index is 3 >> 1 = 1.
        let result = e.call(0x00e3_d3f0, &args![this, 0x2000_0003u32, out]).u32();
        assert_eq!(result, out);
        assert_eq!(e.mem.f32(out + 0x10), 0.25);
        assert_eq!(e.mem.u16(out + 0x14), 0x4242);
        assert_eq!(seen.borrow().len(), 3);
        assert_eq!(seen.borrow()[0].1[0], [1.0, 0.0, 0.0, 0.0]);
        // The corners are (1,2,3), (4,5,6) and (7,8,9) with lane 3 zero, doubled.
        assert_eq!(load_vec(&e, out + 0x40), [2.0, 4.0, 6.0, 0.0]);
        assert_eq!(load_vec(&e, out + 0x30), [8.0, 10.0, 12.0, 0.0]);
        assert_eq!(load_vec(&e, out + 0x20), [14.0, 16.0, 18.0, 0.0]);
    }
}

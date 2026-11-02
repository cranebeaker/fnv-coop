//! `Havok SDK/run_00da0ba0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::havok_sdk::run_00d6bf60::{
    addps, addss, cvtdq2ps, divps, fcom_status, fistp_truncate, load_vec, maxps, minps, mulps,
    pslld, psrld, shufps, store_vec, subps,
};

/// The thread's memory router (`00c85750`); its `+0x10` is the allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkArrayUtil::_reserve` (`00c908c0`, cdecl `(allocator, array, capacity, element size)`) and
/// the grow-by-one `00c90940` (cdecl `(allocator, array, element size)`).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
const ARRAY_GROW: u32 = 0x00c9_0940;
/// `00db0060` (thiscall `(record, box, index)`, `RET 8`: fills a 0x20-byte record from a box of
/// two vectors and an index) and `00db06c0` (cdecl `(records, count)`: sorts them).
const RECORD_INIT: u32 = 0x00db_0060;
const RECORD_SORT: u32 = 0x00db_06c0;
/// The `double`s `0.5` (`01011588`), `6.0` (`01034208`) and `1.0` (`01012070`).
const HALF: u32 = 0x0101_1588;
const SIX: u32 = 0x0103_4208;
const ONE: u32 = 0x0101_2070;
/// The floats `0x7f7fffee` (`0104da48`) and `0xff7fffee` (`0104e148`) the bounds start from, the
/// `0.01f` (`01013ea4`) the box grows by and the 16-bit weight (`010cd644`); the zero vector
/// (`01267e30`).
const MAX_START: u32 = 0x0104_da48;
const MIN_START: u32 = 0x0104_e148;
const BOX_MARGIN: u32 = 0x0101_3ea4;
const HIGH_WEIGHT: u32 = 0x010c_d644;
const ZERO_VECTOR: u32 = 0x0126_7e30;

/// The allocator behind the memory router.
fn router_allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

/// The capacity test every `hkArray::reserve` call site of this file makes: when the capacity
/// (the low 30 bits of the word at `array + 8`) is below `wanted` (signed), `00c908c0` makes room
/// for the larger of twice the capacity and `wanted` elements of `element_size` bytes.
fn ensure_capacity(e: &mut Engine, allocator: u32, array: u32, wanted: i32, element_size: u32) {
    let capacity = (e.mem.u32(array + 8) & 0x3fff_ffff) as i32;
    if capacity < wanted {
        let doubled = capacity.wrapping_add(capacity);
        let target = if wanted < doubled { doubled } else { wanted };
        e.call(
            ARRAY_RESERVE,
            &args![allocator, array, target as u32, element_size],
        );
    }
}

/// Frees `bytes` at `data` through the allocator's slot `+0x10`.
fn free_block(e: &mut Engine, allocator: u32, data: u32, bytes: u32) {
    e.vcall(allocator, 0x10, &args![data, bytes]);
}

/// Sets an `hkArray` (data, size, capacity word) to the empty, non-owning state.
fn empty_array(e: &mut Engine, array: u32) {
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 4, 0);
    e.mem.set_u32(array + 8, 0x8000_0000);
}

// Translated from 00da0ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cdecl `(out, words)`: the byte at `out` becomes 1 when any two of the three words at `words`
/// are equal (the first with the second, the first with the third, the second with the third),
/// otherwise 0. Returns `out` (`EAX`). The exe's name for it is unknown (a degenerate-triangle
/// test).
pub fn fn_00da0ba0(e: &mut Engine, out: u32, words: u32) -> u32 {
    let first = e.mem.u32(words);
    let second = e.mem.u32(words + 4);
    let same = first == second || {
        let third = e.mem.u32(words + 8);
        first == third || second == third
    };
    e.mem.set_u8(out, u8::from(same));
    out
}

// Translated from 00da0bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Register inputs `eax` and `ecx` (vector pointers `p` and `q`), then the stack word `third` (a
/// vector pointer), cdecl, the result in `ST0`: the triple product of the three vectors as the
/// exe computes it, `(shuf(q, 0xd2) * shuf(p, 0xc9) - shuf(q, 0xc9) * shuf(p, 0xd2)) * [third]`,
/// summed over x, y and z as `z + (y + x)`.
pub fn fn_00da0bd0(e: &mut Engine, eax: u32, ecx: u32, third: u32) -> f32 {
    let p = load_vec(e, eax);
    let q = load_vec(e, ecx);
    let q_yzx = shufps(q, q, 0xc9);
    let p_zxy = shufps(p, p, 0xd2);
    let p_yzx = shufps(p, p, 0xc9);
    let first = mulps(q_yzx, p_zxy);
    let q_zxy = shufps(q, q, 0xd2);
    let second = mulps(q_zxy, p_yzx);
    let t = load_vec(e, third);
    let cross = mulps(subps(second, first), t);
    let sum = addss(shufps(cross, cross, 0x55), cross);
    let total = addss(shufps(cross, cross, 0xaa), sum);
    f32::from_bits(x87_quiet_bits(total[0].to_bits()))
}

// Translated from 00da0c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cdecl `(mesh)`, the result in `ST0`: a sixth of the sum of the triple products of the
/// triangles' corners (the signed volume): `mesh` holds the vertex data pointer at `+0`, the
/// triangle data pointer at `+0xc` (16-byte triangles of three vertex indices and a word) and
/// the triangle count at `+0x10`. Each triple product is the one of `00da0bd0` (corners 2, 1 and
/// 0), added to the running `float` sum by the x87 stack; the sum is divided by the double at
/// `01034208`. An empty mesh gives 0.
pub fn fn_00da0c30(e: &mut Engine, mesh: u32) -> f32 {
    let mut sum = 0u32;
    let mut triangles = e.mem.i32(mesh + 0x10);
    if triangles > 0 {
        let vertices = e.mem.u32(mesh);
        let mut triangle = e.mem.u32(mesh + 0xc);
        while triangles != 0 {
            let corner_two = e.mem.u32(triangle + 8);
            let corner_one = e.mem.u32(triangle + 4);
            let v2 = load_vec(e, vertices.wrapping_add(corner_two << 4));
            let v1 = load_vec(e, vertices.wrapping_add(corner_one << 4));
            let corner_zero = e.mem.u32(triangle);
            let first = mulps(shufps(v1, v1, 0xc9), shufps(v2, v2, 0xd2));
            let second = mulps(shufps(v1, v1, 0xd2), shufps(v2, v2, 0xc9));
            let v0 = load_vec(e, vertices.wrapping_add(corner_zero << 4));
            let cross = mulps(subps(first, second), v0);
            let part = addss(shufps(cross, cross, 0x55), cross);
            let total = addss(shufps(cross, cross, 0xaa), part);
            sum = x87_store32(x87_add(x87_load32(total[0].to_bits()), x87_m32(sum)));
            triangle = triangle.wrapping_add(0x10);
            triangles = triangles.wrapping_sub(1);
        }
    }
    let six = e.mem.f64(SIX);
    let result = x87_store32(x87_div(x87_load32(sum), six));
    f32::from_bits(x87_quiet_bits(result))
}

// Translated from 00da0ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cdecl `(transform, points)`: transforms in place every vector of the array `points` (data
/// pointer at `+0`, count at `+4`, both read again each time round): `(y * c1 + x * c0) + z *
/// c2) + translation`, with the columns at `transform + 0`, `+0x10`, `+0x20` and the
/// translation at `+0x30`.
pub fn fn_00da0ce0(e: &mut Engine, transform: u32, points: u32) {
    let mut index = 0i32;
    if e.mem.i32(points + 4) <= 0 {
        return;
    }
    let mut offset = 0u32;
    loop {
        let data = e.mem.u32(points);
        let address = data.wrapping_add(offset);
        let v = load_vec(e, address);
        let c1 = load_vec(e, transform + 0x10);
        let c0 = load_vec(e, transform);
        let by_y = mulps(shufps(v, v, 0x55), c1);
        let by_x = mulps(shufps(v, v, 0x00), c0);
        let sum = addps(by_y, by_x);
        let c2 = load_vec(e, transform + 0x20);
        let by_z = mulps(shufps(v, v, 0xaa), c2);
        let translation = load_vec(e, transform + 0x30);
        let moved = addps(addps(sum, by_z), translation);
        store_vec(e, address, moved);
        index += 1;
        offset = offset.wrapping_add(0x10);
        if index >= e.mem.i32(points + 4) {
            break;
        }
    }
}

// Translated from 00da0d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cdecl `(points, levels)`: snaps every vector of the array `points` (data pointer at `+0`,
/// count at `+4`) onto a grid of `levels` steps per axis inside its own bounding box, in place.
/// The box is the lane-wise minimum and maximum widened by `0.01` each way; the scale is `(levels
/// - 1) / extent`; each vector goes to `((v - min) * scale)` cut to an integer (`FISTP` with the
/// rounding control set to truncate, then the low and high 16-bit halves recombined as `high * w
/// + low` with the weight `w` at `010cd644` and clamped to `0` and `levels - 1`), times `extent
/// / (levels - 1)`, plus the box minimum. The control word is saved and put back each time, so
/// nothing of it outlives the call.
pub fn fn_00da0d50(e: &mut Engine, points: u32, levels: i32) {
    let count = e.mem.i32(points + 4);
    let mut minimum = [f32::from_bits(e.mem.u32(MAX_START)); 4];
    let mut maximum = [f32::from_bits(e.mem.u32(MIN_START)); 4];
    if count > 0 {
        let data = e.mem.u32(points);
        for index in 0..count as u32 {
            let v = load_vec(e, data.wrapping_add(16 * index));
            minimum = minps(minimum, v);
            maximum = maxps(maximum, v);
        }
    }
    let margin = [f32::from_bits(e.mem.u32(BOX_MARGIN)); 4];
    maximum = addps(maximum, margin);
    let steps = x87_store32(x87_sub(f64::from(levels), e.mem.f64(ONE)));
    minimum = subps(minimum, margin);
    let extent = subps(maximum, minimum);
    let step_count = [f32::from_bits(steps); 4];
    let step_size = divps(extent, step_count);
    let scale = divps(step_count, extent);
    if count <= 0 {
        return;
    }
    let weight = [f32::from_bits(x87_quiet_bits(e.mem.u32(HIGH_WEIGHT))); 4];
    let mut index = 0i32;
    let mut offset = 0u32;
    loop {
        let data = e.mem.u32(points);
        let address = data.wrapping_add(offset);
        let v = load_vec(e, address);
        let zero = load_vec(e, ZERO_VECTOR);
        let scaled = mulps(subps(v, minimum), scale);
        let integers: [u32; 4] =
            std::array::from_fn(|lane| fistp_truncate(x87_load32(scaled[lane].to_bits())) as u32);
        let low = cvtdq2ps(psrld(pslld(integers, 16), 16));
        let high = cvtdq2ps(psrld(integers, 16));
        let rebuilt = addps(mulps(high, weight), low);
        let clamped = minps(step_count, maxps(zero, rebuilt));
        let snapped = addps(mulps(clamped, step_size), minimum);
        store_vec(e, address, snapped);
        index += 1;
        offset = offset.wrapping_add(0x10);
        if index >= e.mem.i32(points + 4) {
            break;
        }
    }
}

// Translated from 00da0f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cdecl `(vertices, unique, remap, tolerance)`: welds vertices closer than `tolerance`
/// (`hkGeometryUtility::weldVertices`'s first stage, by its shape). `vertices` is an array of
/// 16-byte vectors (data pointer at `+0`, count `n` at `+4`); `remap` (an `hkArray<int>`) is
/// reserved to `n` words and sized `n`; `unique` (an `hkArray` of vectors) gets the surviving
/// vertices. It allocates `n + 4` 32-byte sweep records through the allocator's slot `+0xc`
/// (the number of records it can hold is the returned size over 32; 0 is flagged as not owned),
/// fills record `i` from the box `[v - tolerance / 2, v + tolerance / 2]` by `00db0060` (the box
/// is the 32 bytes at stack `+0x60`), makes four sentinel records after them from the last box
/// with the first lane of its lower corner set to `0xffffffff` (the exe copies the stack box
/// as it stands; with no vertices that is memory the function never set), sorts the first `n`
/// records with `00db06c0` and sweeps them: a record whose word at `+0xc` (the vertex index) is
/// not `-1` becomes the next unique vertex (`remap[index]` = its position), and every later
/// record whose first word does not pass the upper bound at `+0x10` of the current one and
/// whose vertex lies within `tolerance` (the squared distance `z + (y + x)` against the squared
/// tolerance) gets the same remap entry and its index set to `-1`. The records are freed at the
/// end unless flagged. The class is unconfirmed.
pub fn fn_00da0f10(e: &mut Engine, vertices: u32, unique: u32, remap: u32, tolerance: f32) {
    let count = e.mem.i32(vertices + 4);
    e.with_stack(0x80, |e, frame| {
        let frame = frame.addr();
        let allocator = router_allocator(e);
        ensure_capacity(e, allocator, remap, count, 4);
        e.mem.set_u32(remap + 4, count as u32);
        let records = count.wrapping_add(4);
        let allocator = router_allocator(e);
        let (block, capacity_word) = if records == 0 {
            (0, 0x8000_0000)
        } else {
            e.mem.set_u32(frame + 0x28, (records as u32) << 5);
            let block = e.vcall(allocator, 0xc, &args![frame + 0x28]).u32();
            let size = e.mem.i32(frame + 0x28);
            let slots = size.wrapping_add((size >> 31) & 0x1f) >> 5;
            (
                block,
                if slots == 0 {
                    0x8000_0000
                } else {
                    slots as u32
                },
            )
        };
        let half = x87_store32(x87_mul(x87_load32(tolerance.to_bits()), e.mem.f64(HALF)));
        let half_vector = [f32::from_bits(half); 4];
        let mut record = block;
        let mut offset = 0u32;
        for index in 0..count.max(0) as u32 {
            let data = e.mem.u32(vertices);
            let v = load_vec(e, data.wrapping_add(offset));
            store_vec(e, frame + 0x60, subps(v, half_vector));
            let again = load_vec(e, data.wrapping_add(offset));
            store_vec(e, frame + 0x70, addps(half_vector, again));
            e.call(RECORD_INIT, &args![record, frame + 0x60, index]);
            record = record.wrapping_add(0x20);
            offset = offset.wrapping_add(0x10);
        }
        // The four sentinel records after the `count` real ones.
        let sentinels = block.wrapping_add((count as u32) << 5);
        e.mem.set_u32(frame + 0x60, 0xffff_ffff);
        for sentinel in 0..4u32 {
            for word in 0..8u32 {
                let value = e.mem.u32(frame + 0x60 + 4 * word);
                e.mem
                    .set_u32(sentinels.wrapping_add(0x20 * sentinel + 4 * word), value);
            }
        }
        e.call(RECORD_SORT, &args![block, count as u32]);
        let tolerance_squared = x87_store32(x87_mul(
            x87_load32(tolerance.to_bits()),
            x87_load32(tolerance.to_bits()),
        ));
        e.mem.set_u32(frame + 0x60, tolerance_squared);
        for lane in 1..4u32 {
            e.mem.set_u32(frame + 0x60 + 4 * lane, 0);
        }
        if count > 0 {
            let mut pointer = block.wrapping_add(0xc);
            let mut remaining = count;
            let mut next_start = 1i32;
            loop {
                let index = e.mem.u32(pointer);
                if index != 0xffff_ffff {
                    let vertex_data = e.mem.u32(vertices);
                    let source = vertex_data.wrapping_add(index << 4);
                    for lane in 0..4u32 {
                        let word = e.mem.u32(source + 4 * lane);
                        e.mem.set_u32(frame + 0x40 + 4 * lane, word);
                    }
                    let remap_data = e.mem.u32(remap);
                    let position = e.mem.u32(unique + 4);
                    e.mem
                        .set_u32(remap_data.wrapping_add(index.wrapping_mul(4)), position);
                    let allocator = router_allocator(e);
                    let capacity = e.mem.u32(unique + 8) & 0x3fff_ffff;
                    if e.mem.u32(unique + 4) == capacity {
                        e.call(ARRAY_GROW, &args![allocator, unique, 0x10u32]);
                    }
                    let slot = (e.mem.u32(unique + 4) << 4).wrapping_add(e.mem.u32(unique));
                    let vertex = load_vec(e, frame + 0x40);
                    store_vec(e, slot, vertex);
                    let size = e.mem.u32(unique + 4).wrapping_add(1);
                    e.mem.set_u32(unique + 4, size);
                    let mut other = next_start;
                    if other < count {
                        let squared_limit = e.mem.u32(frame + 0x60);
                        let mut candidate = pointer.wrapping_add(0x20);
                        loop {
                            let key = e.mem.u32(candidate.wrapping_sub(0xc));
                            if key > e.mem.u32(pointer + 4) {
                                break;
                            }
                            let other_index = e.mem.u32(candidate);
                            if other_index != 0xffff_ffff {
                                let vertex_data = e.mem.u32(vertices);
                                let near = vertex_data.wrapping_add(other_index << 4);
                                for lane in 0..4u32 {
                                    let word = e.mem.u32(near + 4 * lane);
                                    e.mem.set_u32(frame + 0x50 + 4 * lane, word);
                                }
                                let mine = load_vec(e, frame + 0x40);
                                let theirs = load_vec(e, frame + 0x50);
                                let difference = subps(mine, theirs);
                                let squares = mulps(difference, difference);
                                let part = addss(shufps(squares, squares, 0x55), squares);
                                let total = addss(shufps(squares, squares, 0xaa), part);
                                let distance = total[0].to_bits();
                                // `FCOMPP` of the squared limit against the squared distance:
                                // not merged when the limit is below it (or unordered).
                                let limit = x87_load32(squared_limit);
                                if fcom_status(limit, x87_load32(distance)) & 1 == 0 {
                                    let remap_data = e.mem.u32(remap);
                                    let kept =
                                        e.mem.u32(remap_data.wrapping_add(index.wrapping_mul(4)));
                                    e.mem.set_u32(
                                        remap_data.wrapping_add(other_index.wrapping_mul(4)),
                                        kept,
                                    );
                                    e.mem.set_u32(candidate, 0xffff_ffff);
                                }
                            }
                            other += 1;
                            candidate = candidate.wrapping_add(0x20);
                            if other >= count {
                                break;
                            }
                        }
                    }
                }
                next_start += 1;
                pointer = pointer.wrapping_add(0x20);
                remaining -= 1;
                if remaining == 0 {
                    break;
                }
            }
        }
        let allocator = router_allocator(e);
        if (capacity_word as i32) >= 0 {
            let bytes = (capacity_word & 0x3fff_ffff) << 5;
            free_block(e, allocator, block, bytes);
        }
    });
}

/// The code that grows a vertex array's buffer to `needed` vectors when its capacity is smaller:
/// frees the old buffer when it is owned (flag bit clear), allocates `needed * 16` bytes through
/// slot `+0xc` (which returns the size it really gave) and records the buffer and the capacity
/// `size / 16`. `size_slot` is the stack word the size travels in.
fn regrow_vectors(e: &mut Engine, allocator: u32, array: u32, needed: i32, size_slot: u32) {
    let capacity_word = e.mem.u32(array + 8);
    let capacity = (capacity_word & 0x3fff_ffff) as i32;
    if capacity < needed {
        if (capacity_word as i32) >= 0 {
            let data = e.mem.u32(array);
            free_block(e, allocator, data, (capacity as u32) << 4);
        }
        e.mem.set_u32(size_slot, (needed as u32) << 4);
        let block = e.vcall(allocator, 0xc, &args![size_slot]).u32();
        e.mem.set_u32(array, block);
        let size = e.mem.i32(size_slot);
        let slots = size.wrapping_add((size >> 31) & 0xf) >> 4;
        e.mem.set_u32(array + 8, slots as u32);
    }
}

/// Copies `count` 16-byte vectors from `source` to the data pointer of `array`.
fn copy_vectors(e: &mut Engine, array: u32, source: u32, count: i32) {
    let mut to = e.mem.u32(array);
    let mut from = source;
    for _ in 0..count.max(0) {
        let vector = load_vec(e, from);
        store_vec(e, to, vector);
        from = from.wrapping_add(0x10);
        to = to.wrapping_add(0x10);
    }
}

// Translated from 00da12b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cdecl `(geometry, tolerance, compact, vertex_remap, triangle_remap)` (`hkGeometryUtility::
/// weldVertices` by its shape; `compact` is a byte): welds the geometry's vertices with
/// `00da0f10` and rewrites it. `geometry` holds the vertex array at `+0` (data, count at `+4`,
/// capacity word at `+8`) and the triangle array at `+0xc` (data, count at `+0x10`, capacity at
/// `+0x14`; 16-byte triangles of three vertex indices and a word). `vertex_remap` is reserved
/// and sized to the old vertex count and receives each old vertex's new index.
///
/// With `compact` zero the vertex array becomes the unique vertices in sweep order. With it set
/// they are renumbered in the order the old vertices first use them (two local arrays and a
/// table filled with `-1`; the owned buffers are freed afterwards). Then every triangle's three
/// indices go through `vertex_remap`; triangles with two equal indices (`00da0ba0`) are dropped
/// by copying the others down (`triangle_remap`, reserved and sized to the old triangle count,
/// gets each triangle's new index, or `-1` for the dropped), and the triangle array is resized.
/// The allocator calls are the exe's (the stack arrays that start empty and unowned are freed
/// only when their capacity word says they own a buffer). The class is unconfirmed.
pub fn fn_00da12b0(
    e: &mut Engine,
    geometry: u32,
    tolerance: f32,
    compact: u8,
    vertex_remap: u32,
    triangle_remap: u32,
) {
    let triangle_count = e.mem.i32(geometry + 0x10);
    let vertex_count = e.mem.i32(geometry + 4);
    let allocator = router_allocator(e);
    ensure_capacity(e, allocator, vertex_remap, vertex_count, 4);
    let tolerance_bits = x87_quiet_bits(tolerance.to_bits());
    e.mem.set_u32(vertex_remap + 4, vertex_count as u32);
    e.with_stack(0x60, |e, frame| {
        let frame = frame.addr();
        // `A1` (unique vertices) at +0x44, `A2` (compacted vertices) at +0x2c, `A3` (new indices)
        // at +0x38; `+0x24` is the size word the allocator calls travel in.
        let unique = frame + 0x44;
        empty_array(e, unique);
        fn_00da0f10(
            e,
            geometry,
            unique,
            vertex_remap,
            f32::from_bits(tolerance_bits),
        );
        let unique_count = e.mem.i32(unique + 4);
        if compact != 0 {
            let compacted = frame + 0x2c;
            let table = frame + 0x38;
            empty_array(e, compacted);
            empty_array(e, table);
            let allocator = router_allocator(e);
            ensure_capacity(e, allocator, table, unique_count, 4);
            let table_size = e.mem.i32(table + 4);
            let table_data = e.mem.u32(table);
            let fill = unique_count.wrapping_sub(table_size);
            if fill > 0 {
                for slot in 0..fill as u32 {
                    let at = table_data
                        .wrapping_add((table_size as u32).wrapping_mul(4))
                        .wrapping_add(4 * slot);
                    e.mem.set_u32(at, 0xffff_ffff);
                }
            }
            e.mem.set_u32(table + 4, unique_count as u32);
            let mut index = 0i32;
            if e.mem.i32(vertex_remap + 4) > 0 {
                loop {
                    let table_data = e.mem.u32(table);
                    let remap_data = e.mem.u32(vertex_remap);
                    let unique_index = e
                        .mem
                        .u32(remap_data.wrapping_add((index as u32).wrapping_mul(4)));
                    let slot = table_data.wrapping_add(unique_index.wrapping_mul(4));
                    let mut new_index = e.mem.u32(slot);
                    if new_index == 0xffff_ffff {
                        new_index = e.mem.u32(compacted + 4);
                        e.mem.set_u32(slot, new_index);
                        let source = (unique_index << 4).wrapping_add(e.mem.u32(unique));
                        let allocator = router_allocator(e);
                        let capacity = e.mem.u32(compacted + 8) & 0x3fff_ffff;
                        if e.mem.u32(compacted + 4) == capacity {
                            e.call(ARRAY_GROW, &args![allocator, compacted, 0x10u32]);
                        }
                        let vector = load_vec(e, source);
                        let target =
                            (e.mem.u32(compacted + 4) << 4).wrapping_add(e.mem.u32(compacted));
                        store_vec(e, target, vector);
                        let size = e.mem.u32(compacted + 4).wrapping_add(1);
                        e.mem.set_u32(compacted + 4, size);
                    }
                    let remap_data = e.mem.u32(vertex_remap);
                    e.mem.set_u32(
                        remap_data.wrapping_add((index as u32).wrapping_mul(4)),
                        new_index,
                    );
                    index += 1;
                    if index >= e.mem.i32(vertex_remap + 4) {
                        break;
                    }
                }
            }
            let allocator = router_allocator(e);
            let new_count = e.mem.i32(compacted + 4);
            regrow_vectors(e, allocator, geometry, new_count, frame + 0x24);
            e.mem.set_u32(geometry + 4, new_count as u32);
            let from = e.mem.u32(compacted);
            copy_vectors(e, geometry, from, new_count);
            let allocator = router_allocator(e);
            let table_capacity = e.mem.u32(table + 8);
            e.mem.set_u32(table + 4, 0);
            if (table_capacity as i32) >= 0 {
                let bytes = (table_capacity & 0x3fff_ffff).wrapping_mul(4);
                let data = e.mem.u32(table);
                free_block(e, allocator, data, bytes);
            }
            e.mem.set_u32(table, 0);
            e.mem.set_u32(table + 8, 0x8000_0000);
            let allocator = router_allocator(e);
            let compacted_capacity = e.mem.u32(compacted + 8);
            e.mem.set_u32(compacted + 4, 0);
            if (compacted_capacity as i32) >= 0 {
                let bytes = (compacted_capacity & 0x3fff_ffff) << 4;
                let data = e.mem.u32(compacted);
                free_block(e, allocator, data, bytes);
            }
        } else {
            let allocator = router_allocator(e);
            regrow_vectors(e, allocator, geometry, unique_count, frame + 0x24);
            e.mem.set_u32(geometry + 4, unique_count as u32);
            let from = e.mem.u32(unique);
            copy_vectors(e, geometry, from, unique_count);
        }
        let allocator = router_allocator(e);
        let unique_capacity = e.mem.u32(unique + 8);
        e.mem.set_u32(unique + 4, 0);
        if (unique_capacity as i32) >= 0 {
            let bytes = (unique_capacity & 0x3fff_ffff) << 4;
            let data = e.mem.u32(unique);
            free_block(e, allocator, data, bytes);
        }
        for index in 0..triangle_count.max(0) as u32 {
            let triangles = e.mem.u32(geometry + 0xc);
            let triangle = triangles.wrapping_add(16 * index);
            for word in 0..3u32 {
                let old = e.mem.u32(triangle + 4 * word);
                let remap_data = e.mem.u32(vertex_remap);
                let new = e.mem.u32(remap_data.wrapping_add(old.wrapping_mul(4)));
                e.mem.set_u32(triangle + 4 * word, new);
            }
        }
        let allocator = router_allocator(e);
        ensure_capacity(e, allocator, triangle_remap, triangle_count, 4);
        let triangle_array = geometry.wrapping_add(0xc);
        e.mem.set_u32(triangle_remap + 4, triangle_count as u32);
        let mut cursor = e.mem.u32(triangle_array);
        for index in 0..triangle_count.max(0) as u32 {
            let triangles = e.mem.u32(triangle_array);
            let triangle = triangles.wrapping_add(16 * index);
            fn_00da0ba0(e, frame + 0x1b, triangle);
            let mut new_index = 0xffff_ffff;
            if e.mem.u8(frame + 0x1b) == 0 {
                let start = e.mem.u32(triangle_array);
                for word in 0..4u32 {
                    let value = e.mem.u32(triangle + 4 * word);
                    e.mem.set_u32(cursor + 4 * word, value);
                }
                new_index = (cursor.wrapping_sub(start) as i32 >> 4) as u32;
                cursor = cursor.wrapping_add(0x10);
            }
            let remap_data = e.mem.u32(triangle_remap);
            e.mem.set_u32(remap_data.wrapping_add(4 * index), new_index);
        }
        let new_count = (cursor.wrapping_sub(e.mem.u32(triangle_array)) as i32) >> 4;
        let allocator = router_allocator(e);
        ensure_capacity(e, allocator, triangle_array, new_count, 0x10);
        e.mem.set_u32(triangle_array + 4, new_count as u32);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00da_0ba0, fn_00da0ba0(u32, u32) -> u32),
        entry!(0x00da_0bd0, fn_00da0bd0(u32, u32, u32) -> f32),
        entry!(0x00da_0c30, fn_00da0c30(u32) -> f32),
        entry!(0x00da_0ce0, fn_00da0ce0(u32, u32)),
        entry!(0x00da_0d50, fn_00da0d50(u32, i32)),
        entry!(0x00da_0f10, fn_00da0f10(u32, u32, u32, f32)),
        entry!(0x00da_12b0, fn_00da12b0(u32, f32, u8, u32, u32)),
    ]
}

#[cfg(test)]
mod a09_tests {
    use super::*;
    use crate::units::havok_sdk::run_00d6bf60::Vec4;
    use std::cell::RefCell;
    use std::rc::Rc;

    const ALLOC_SLOT: u32 = 0x0900_000c;
    const FREE_SLOT: u32 = 0x0900_0010;

    type Frees = Rc<RefCell<Vec<(u32, u32)>>>;

    fn put(e: &mut Engine, address: u32, vector: Vec4) {
        store_vec(e, address, vector);
    }

    /// The router with an allocator (slot `+0xc` allocates the size in its word, slot `+0x10`
    /// logs the `(block, size)` it is asked to free), the array reserve and grow doubles and the
    /// sweep record doubles: `00db0060` writes the lower corner's x bits at `+0`, the index at
    /// `+0xc` and the upper corner's x bits at `+0x10`; `00db06c0` sorts records by the float at
    /// `+0`.
    fn install(e: &mut Engine) -> Frees {
        e.map(0x0100_0000, 0x0030_0000);
        e.set_global(HALF, 0.5f64);
        e.put_vtable(0x0900_0000, &[0, 0, 0, ALLOC_SLOT, FREE_SLOT]);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, 0x0900_0000);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        e.register(ALLOC_SLOT, |e, a| {
            let size = e.mem.u32(a[1]);
            Ret {
                eax: e.mem.alloc(size),
                ..Ret::default()
            }
        });
        let frees: Frees = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&frees);
        e.register_double(FREE_SLOT, move |_, a| {
            log.borrow_mut().push((a[1], a[2]));
            Ret::default()
        });
        e.register(ARRAY_RESERVE, |e, a| {
            let (array, capacity, element) = (a[1], a[2], a[3]);
            let block = e.mem.alloc(capacity * element);
            let size = e.mem.u32(array + 4);
            let old = e.mem.u32(array);
            for byte in 0..size * element {
                let value = e.mem.u8(old + byte);
                e.mem.set_u8(block + byte, value);
            }
            e.mem.set_u32(array, block);
            e.mem.set_u32(array + 8, capacity);
            Ret::default()
        });
        e.register(ARRAY_GROW, |e, a| {
            let (array, element) = (a[1], a[2]);
            let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
            let wanted = (capacity * 2).max(4);
            let block = e.mem.alloc(wanted * element);
            let size = e.mem.u32(array + 4);
            let old = e.mem.u32(array);
            for byte in 0..size * element {
                let value = e.mem.u8(old + byte);
                e.mem.set_u8(block + byte, value);
            }
            e.mem.set_u32(array, block);
            e.mem.set_u32(array + 8, wanted);
            Ret::default()
        });
        e.register(RECORD_INIT, |e, a| {
            let low = e.mem.u32(a[1]);
            let high = e.mem.u32(a[1] + 0x10);
            e.mem.set_u32(a[0], low);
            e.mem.set_u32(a[0] + 0xc, a[2]);
            e.mem.set_u32(a[0] + 0x10, high);
            Ret::default()
        });
        e.register(RECORD_SORT, |e, a| {
            let mut records: Vec<Vec<u32>> = (0..a[1])
                .map(|i| (0..8).map(|w| e.mem.u32(a[0] + 0x20 * i + 4 * w)).collect())
                .collect();
            records.sort_by(|x, y| f32::from_bits(x[0]).total_cmp(&f32::from_bits(y[0])));
            for (i, record) in records.iter().enumerate() {
                for (w, word) in record.iter().enumerate() {
                    e.mem.set_u32(a[0] + 0x20 * i as u32 + 4 * w as u32, *word);
                }
            }
            Ret::default()
        });
        frees
    }

    fn vector_array(e: &mut Engine, vectors: &[Vec4]) -> u32 {
        let array = e.mem.alloc(0x10);
        let data = e.mem.alloc(16 * vectors.len() as u32 + 16);
        for (i, v) in vectors.iter().enumerate() {
            put(e, data + 16 * i as u32, *v);
        }
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 4, vectors.len() as u32);
        e.mem.set_u32(array + 8, vectors.len() as u32);
        array
    }

    fn read_vectors(e: &Engine, array: u32) -> Vec<Vec4> {
        let data = e.mem.u32(array);
        (0..e.mem.u32(array + 4))
            .map(|i| load_vec(e, data + 16 * i))
            .collect()
    }

    fn read_words(e: &Engine, array: u32) -> Vec<u32> {
        let data = e.mem.u32(array);
        (0..e.mem.u32(array + 4))
            .map(|i| e.mem.u32(data + 4 * i))
            .collect()
    }

    fn empty(e: &mut Engine) -> u32 {
        let array = e.mem.alloc(0x10);
        empty_array(e, array);
        array
    }

    #[test]
    fn two_equal_indices_make_a_triangle_degenerate() {
        let mut e = Engine::new();
        let words = e.mem.alloc(0x10);
        let out = e.mem.alloc(8);
        for (triple, want) in [
            ([1u32, 2, 3], 0u8),
            ([1, 1, 3], 1),
            ([1, 2, 1], 1),
            ([1, 2, 2], 1),
            ([5, 5, 5], 1),
        ] {
            for (i, word) in triple.iter().enumerate() {
                e.mem.set_u32(words + 4 * i as u32, *word);
            }
            e.mem.set_u8(out, 0xee);
            assert_eq!(e.call(0x00da_0ba0, &args![out, words]).u32(), out);
            assert_eq!(e.mem.u8(out), want, "{triple:?}");
        }
    }

    #[test]
    fn the_triple_product_of_three_vectors() {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x1000);
        let (p, q, t) = (0x0100_0100, 0x0100_0200, 0x0100_0300);
        put(&mut e, p, [1.0, 2.0, 3.0, 0.0]);
        put(&mut e, q, [4.0, 5.0, 6.0, 0.0]);
        put(&mut e, t, [1.0, 0.0, 0.0, 0.0]);
        // (q.z * p.y - q.y * p.z, ...) = (-3, 6, -3), times t, summed: -3.
        assert_eq!(e.call(0x00da_0bd0, &args![p, q, t]).f32(), -3.0);
        put(&mut e, t, [0.0, 1.0, 0.0, 0.0]);
        assert_eq!(e.call(0x00da_0bd0, &args![p, q, t]).f32(), 6.0);
        put(&mut e, t, [1.0, 1.0, 1.0, 1.0]);
        assert_eq!(e.call(0x00da_0bd0, &args![p, q, t]).f32(), 0.0);
    }

    #[test]
    fn the_signed_volume_of_a_tetrahedron_is_a_sixth_and_an_empty_mesh_is_zero() {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.set_global(SIX, 6.0f64);
        let mesh = e.mem.alloc(0x20);
        let vertices = vector_array(
            &mut e,
            &[
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
            ],
        );
        e.mem.set_u32(mesh, e.mem.u32(vertices));
        let triangles = e.mem.alloc(0x40);
        for (i, corner) in [0u32, 1, 2, 0].iter().enumerate() {
            e.mem.set_u32(triangles + 4 * i as u32, *corner);
        }
        e.mem.set_u32(mesh + 0xc, triangles);
        e.mem.set_u32(mesh + 0x10, 1);
        assert_eq!(e.call(0x00da_0c30, &args![mesh]).f32(), 1.0 / 6.0);
        // A second triangle with the corners reversed cancels the first.
        for (i, corner) in [0u32, 2, 1, 0].iter().enumerate() {
            e.mem.set_u32(triangles + 0x10 + 4 * i as u32, *corner);
        }
        e.mem.set_u32(mesh + 0x10, 2);
        assert_eq!(e.call(0x00da_0c30, &args![mesh]).f32(), 0.0);
        e.mem.set_u32(mesh + 0x10, 0);
        assert_eq!(e.call(0x00da_0c30, &args![mesh]).f32(), 0.0);
    }

    #[test]
    fn the_points_are_moved_by_the_transform_in_place() {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x1000);
        let transform = 0x0100_0100;
        put(&mut e, transform, [0.0, 1.0, 0.0, 0.0]);
        put(&mut e, transform + 0x10, [-1.0, 0.0, 0.0, 0.0]);
        put(&mut e, transform + 0x20, [0.0, 0.0, 2.0, 0.0]);
        put(&mut e, transform + 0x30, [10.0, 20.0, 30.0, 0.0]);
        let array = vector_array(&mut e, &[[1.0, 2.0, 3.0, 0.0], [0.0, 0.0, 0.0, 0.0]]);
        e.call(0x00da_0ce0, &args![transform, array]);
        // y * c1 + x * c0 + z * c2 + t: (-2, 0, 0) + (0, 1, 0) + (0, 0, 6) + (10, 20, 30).
        assert_eq!(
            read_vectors(&e, array),
            [[8.0, 21.0, 36.0, 0.0], [10.0, 20.0, 30.0, 0.0]]
        );
        let none = vector_array(&mut e, &[]);
        e.call(0x00da_0ce0, &args![transform, none]);
    }

    #[test]
    fn points_snap_to_a_grid_inside_their_bounding_box() {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.set_global(MAX_START, 0x7f7f_ffeeu32);
        e.set_global(MIN_START, 0xff7f_ffeeu32);
        e.set_global(BOX_MARGIN, 0.01f32);
        e.set_global(ONE, 1.0f64);
        e.set_global(HIGH_WEIGHT, 65536.0f32);
        let points = [
            [0.0, 0.0, 0.0, 0.0],
            [10.0, 20.0, 30.0, 0.0],
            [5.0, 5.0, 5.0, 0.0],
        ];
        let array = vector_array(&mut e, &points);
        e.call(0x00da_0d50, &args![array, 11u32]);
        // The same arithmetic worked out with plain floats.
        let margin = 0.01f32;
        let mut want = Vec::new();
        for p in points {
            let mut snapped = [0.0f32; 4];
            for lane in 0..4 {
                let low = points.iter().map(|q| q[lane]).fold(f32::MAX, f32::min) - margin;
                let high = points.iter().map(|q| q[lane]).fold(f32::MIN, f32::max) + margin;
                let extent = high - low;
                let steps = 10.0f32;
                let scaled = (p[lane] - low) * (steps / extent);
                let cell = (scaled as i64 as f32).max(0.0).min(steps);
                snapped[lane] = cell * (extent / steps) + low;
            }
            want.push(snapped);
        }
        assert_eq!(read_vectors(&e, array), want);
    }

    fn weld_fixture(e: &mut Engine, vertices: &[Vec4], tolerance: f32) -> (u32, u32, u32, Frees) {
        let frees = install(e);
        let array = vector_array(e, vertices);
        let unique = empty(e);
        let remap = empty(e);
        e.call(0x00da_0f10, &args![array, unique, remap, tolerance]);
        (array, unique, remap, frees)
    }

    #[test]
    fn close_vertices_are_welded_into_the_first_of_their_group() {
        let mut e = Engine::new();
        let vertices = [
            [1.0, 1.0, 1.0, 0.0],
            [1.05, 1.0, 1.0, 0.0],
            [5.0, 5.0, 5.0, 0.0],
            [1.02, 1.01, 1.0, 0.0],
            [5.05, 5.0, 5.0, 0.0],
            [9.0, 9.0, 9.0, 0.0],
        ];
        let (_, unique, remap, frees) = weld_fixture(&mut e, &vertices, 0.2);
        assert_eq!(
            read_vectors(&e, unique),
            [vertices[0], vertices[2], vertices[5]]
        );
        assert_eq!(read_words(&e, remap), [0, 0, 1, 0, 1, 2]);
        // The sweep records (6 + 4 of 32 bytes) were freed with their size.
        assert_eq!(frees.borrow().len(), 1);
        assert_eq!(frees.borrow()[0].1, 10 * 32);
    }

    #[test]
    fn vertices_just_beyond_the_tolerance_stay_apart() {
        let mut e = Engine::new();
        let vertices = [
            [1.0, 1.0, 1.0, 0.0],
            [1.3, 1.0, 1.0, 0.0],
            [1.0, 1.0, 1.3, 0.0],
        ];
        let (_, unique, remap, _) = weld_fixture(&mut e, &vertices, 0.2);
        assert_eq!(read_vectors(&e, unique).len(), 3);
        assert_eq!(read_words(&e, remap), [0, 2, 1]);
    }

    #[test]
    fn no_vertices_leave_empty_arrays() {
        let mut e = Engine::new();
        let (_, unique, remap, frees) = weld_fixture(&mut e, &[], 0.2);
        assert_eq!(e.mem.u32(unique + 4), 0);
        assert_eq!(e.mem.u32(remap + 4), 0);
        assert_eq!(frees.borrow().len(), 1);
    }

    /// A geometry of the given vertices and triangles (index triples with a payload word).
    fn geometry(e: &mut Engine, vertices: &[Vec4], triangles: &[[u32; 4]]) -> u32 {
        let geometry = e.mem.alloc(0x20);
        let data = e.mem.alloc(16 * vertices.len() as u32 + 16);
        for (i, v) in vertices.iter().enumerate() {
            put(e, data + 16 * i as u32, *v);
        }
        e.mem.set_u32(geometry, data);
        e.mem.set_u32(geometry + 4, vertices.len() as u32);
        e.mem.set_u32(geometry + 8, vertices.len() as u32);
        let tri = e.mem.alloc(16 * triangles.len() as u32 + 16);
        for (i, t) in triangles.iter().enumerate() {
            for (w, word) in t.iter().enumerate() {
                e.mem.set_u32(tri + 16 * i as u32 + 4 * w as u32, *word);
            }
        }
        e.mem.set_u32(geometry + 0xc, tri);
        e.mem.set_u32(geometry + 0x10, triangles.len() as u32);
        e.mem.set_u32(geometry + 0x14, triangles.len() as u32);
        geometry
    }

    fn read_triangles(e: &Engine, geometry: u32) -> Vec<[u32; 4]> {
        let data = e.mem.u32(geometry + 0xc);
        (0..e.mem.u32(geometry + 0x10))
            .map(|i| std::array::from_fn(|w| e.mem.u32(data + 16 * i + 4 * w as u32)))
            .collect()
    }

    #[test]
    fn weld_rewrites_the_vertices_and_drops_degenerate_triangles() {
        let mut e = Engine::new();
        let frees = install(&mut e);
        let vertices = [
            [1.0, 1.0, 1.0, 0.0],
            [1.05, 1.0, 1.0, 0.0],
            [5.0, 5.0, 5.0, 0.0],
            [1.02, 1.01, 1.0, 0.0],
            [5.05, 5.0, 5.0, 0.0],
            [9.0, 9.0, 9.0, 0.0],
        ];
        let triangles = [
            [0, 2, 5, 0xa0],
            [0, 1, 3, 0xa1],
            [2, 4, 5, 0xa2],
            [1, 2, 5, 0xa3],
        ];
        let geometry = geometry(&mut e, &vertices, &triangles);
        let vertex_remap = empty(&mut e);
        let triangle_remap = empty(&mut e);
        e.call(
            0x00da_12b0,
            &args![geometry, 0.2f32, 0u32, vertex_remap, triangle_remap],
        );
        assert_eq!(
            read_vectors(&e, geometry),
            [vertices[0], vertices[2], vertices[5]]
        );
        assert_eq!(read_words(&e, vertex_remap), [0, 0, 1, 0, 1, 2]);
        assert_eq!(
            read_triangles(&e, geometry),
            [[0, 1, 2, 0xa0], [0, 1, 2, 0xa3]]
        );
        assert_eq!(read_words(&e, triangle_remap), [0, u32::MAX, u32::MAX, 1]);
        // The local array of unique vertices was freed (it owned a grown buffer).
        assert!(!frees.borrow().is_empty());
    }

    #[test]
    fn weld_with_compaction_numbers_the_vertices_by_first_use() {
        let mut e = Engine::new();
        install(&mut e);
        let vertices = [
            [9.0, 9.0, 9.0, 0.0],
            [1.0, 1.0, 1.0, 0.0],
            [1.05, 1.0, 1.0, 0.0],
            [5.0, 5.0, 5.0, 0.0],
            [1.02, 1.01, 1.0, 0.0],
            [5.05, 5.0, 5.0, 0.0],
        ];
        let triangles = [[0, 1, 3, 7]];
        let geometry = geometry(&mut e, &vertices, &triangles);
        let vertex_remap = empty(&mut e);
        let triangle_remap = empty(&mut e);
        e.call(
            0x00da_12b0,
            &args![geometry, 0.2f32, 1u32, vertex_remap, triangle_remap],
        );
        // The unique vertices in sweep order are [(1,1,1), (5,5,5), (9,9,9)]; first use by the
        // old vertex order makes them (9,9,9), (1,1,1), (5,5,5).
        assert_eq!(
            read_vectors(&e, geometry),
            [vertices[0], vertices[1], vertices[3]]
        );
        assert_eq!(read_words(&e, vertex_remap), [0, 1, 1, 2, 1, 2]);
        assert_eq!(read_triangles(&e, geometry), [[0, 1, 2, 7]]);
        assert_eq!(read_words(&e, triangle_remap), [0]);
    }

    #[test]
    fn welding_nothing_keeps_an_empty_geometry_empty() {
        let mut e = Engine::new();
        install(&mut e);
        let geometry = geometry(&mut e, &[], &[]);
        let vertex_remap = empty(&mut e);
        let triangle_remap = empty(&mut e);
        e.call(
            0x00da_12b0,
            &args![geometry, 0.1f32, 1u32, vertex_remap, triangle_remap],
        );
        assert_eq!(e.mem.u32(geometry + 4), 0);
        assert_eq!(e.mem.u32(geometry + 0x10), 0);
    }
}

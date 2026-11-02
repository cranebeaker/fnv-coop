//! `Havok SDK/run_00dde4c0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The functions of the mesh welding utility's helpers (the assertion text the first one
//! reports names the file `.\Util\Welding\hkpMeshWeldingUtility.cpp`) and the 12-byte-element
//! array routines and sort the utility uses.

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`00c85750`): its word `+0` is the stack allocator's cursor and `+0x10`
/// the allocator blocks are freed through (vtable slot `0x10`, `(pointer, size)`).
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// The error handler singleton (`01268360`). Its vtable slot `0xc` reports an assertion
/// `(1, id, message, file, line)`; slot `0x14` writes a byte for an id (`(out, id)`); slot
/// `0x10` takes `(id, value)`.
const ERROR_HANDLER: u32 = 0x0126_8360;
/// `hkOstream` constructor over a buffer (`00cb1600`, thiscall `(buffer, size)`), its text
/// writer (`00cc01c0`, `(text)`, returns the stream), the integer writer (`00cc02d0`,
/// `hkOstream::operator<<(int)`, returns the stream) and its destructor (`00cc00e0`).
const STREAM_BEGIN: u32 = 0x00cb_1600;
const STREAM_TEXT: u32 = 0x00cc_01c0;
const STREAM_INT: u32 = 0x00cc_02d0;
const STREAM_END: u32 = 0x00cc_00e0;
/// `00ddd6f0` (cdecl `(triangle, other triangle, index, out vectors, out indices)`): matches the
/// vertices of two triangles; returns 3 plus the number of vertices it had to copy.
const MATCH_VERTICES: u32 = 0x00dd_d6f0;
/// `00dddc40` (cdecl, one word): the value stored into the matched flags.
const EDGE_VALUE: u32 = 0x00dd_dc40;
/// `00ddcdd0` (cdecl `(word, index, value)`): replaces the field `index` of `word`.
const SET_FIELD: u32 = 0x00dd_cdd0;
/// `00dddcd0` (cdecl `(table, key, word, records, word)`).
const WELDING_MULTI_SHAPE: u32 = 0x00dd_dcd0;
/// `hkArrayUtil::_reserve` (`00c90940`, cdecl `(allocator, array, element size)`).
const ARRAY_RESERVE_MORE: u32 = 0x00c9_0940;
/// `hkArrayUtil::_reserve` (`00c908c0`, cdecl `(allocator, array, capacity, element size)`).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
/// `hkMemUtil::memMove` (`00d2cc70`, cdecl `(destination, source, byte count)`).
const MEM_MOVE: u32 = 0x00d2_cc70;
/// `hkpTriangleUtil::isDegenerate` (`00ceb8d0`, cdecl `(out byte, a, b, c, tolerance)`; returns
/// a pointer to the byte).
const IS_DEGENERATE: u32 = 0x00ce_b8d0;
/// The comparison of two 12-byte records (`00ddcda0`, cdecl `(out byte, a, b)`; the byte is 1
/// when the third float of `a` is below the one of `b`; returns a pointer to the byte).
const RECORD_LESS: u32 = 0x00dd_cda0;

/// The float `1e-4` (`010141cc`, loaded through the x87 stack), the float `-0.0` (`010c4990`)
/// and the float `0.0` (`01011d78`) the box corners are built from.
const MARGIN: u32 = 0x0101_41cc;
const BOX_BASE: u32 = 0x010c_4990;
const BOX_W: u32 = 0x0101_1d78;
/// The tolerance of the degenerate-triangle test (`011b2e90`, loaded through the x87 stack).
const DEGENERATE_TOLERANCE: u32 = 0x011b_2e90;
/// The 64 bytes at `01268370` the record of `00dde760` starts with, also the first argument of
/// `00dddcd0`.
const RECORD_TEMPLATE: u32 = 0x0126_8370;
/// The id of the assertion `00dde930` switches off while it checks (`0xfe98751e`).
const SWITCHED_ASSERT_ID: u32 = 0xfe98_751e;

/// The assertion `00dde4c0` reports: id, file, line and the three text pieces.
const WINDING_ASSERT_ID: u32 = 0xabba_751e;
const WINDING_ASSERT_FILE: u32 = 0x010d_7f50;
const WINDING_ASSERT_LINE: u32 = 0x2c9;
const WINDING_TEXT_1: u32 = 0x010d_7fc8;
const WINDING_TEXT_2: u32 = 0x010d_8004;
const WINDING_TEXT_3: u32 = 0x010d_8014;

type Lanes = [f32; 4];

fn load(e: &Engine, at: u32) -> Lanes {
    std::array::from_fn(|lane| e.mem.f32(at.wrapping_add(4 * lane as u32)))
}

fn store(e: &mut Engine, at: u32, v: Lanes) {
    for (lane, value) in v.iter().enumerate() {
        e.mem.set_f32(at.wrapping_add(4 * lane as u32), *value);
    }
}

// Translated from 00dde4c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Welding step for one vertex of a triangle (the engine map has no name), cdecl with six
/// words `(object, triangle, skip_key, index, report_winding, flags)`. `object` is the
/// collection (vtable `0x10` gives the container whose slot `0x14` is `getChildShape(key,
/// buffer)`; slot `0x30` fills an array of keys overlapping a box). The box is the vertex
/// `index % 3` of `triangle` (vertices at `+0x20`, 16 bytes apart) grown by `1e-4` in x, y
/// and z. For every key except `skip_key` whose child shape has type 3 and for which
/// `00ddd6f0` returns 4 (two shared vertices): if `report_winding` is set and the shared
/// edge is not traversed the other way round, the assertion `"Inconsistant triangle winding
/// ..."` is reported once; then the 16-bit word at `flags` is updated with `00ddcdd0`.
/// Returns whether such a winding problem was found.
///
/// The two index words `00ddd6f0` writes live on the exe's stack and are read only after it
/// returned 4; they are zero here until it writes them.
pub fn fn_00dde4c0(
    e: &mut Engine,
    object: Ptr,
    triangle: u32,
    skip_key: u32,
    index: i32,
    report_winding: u32,
    flags: Ptr,
) -> bool {
    // The exe's frame, addressed as the exe does from `ESP`: the vertex indices at `+0x18`,
    // the box at `+0x20`/`+0x30` (the text stream reuses `+0x20`), 16-byte vectors at
    // `+0x40`, the key array at `+0xa0` (its 128 inline entries at `+0xac`) and the shape
    // buffer / message buffer at `+0x2b0`.
    e.with_stack(0x4c0, |e, frame| {
        let f = frame.addr();
        let margin = x87_quiet(e.global::<f32>(MARGIN));
        let base_low = e.global::<f32>(BOX_BASE);
        let w_bias = e.global::<f32>(BOX_W);
        let vertices = triangle.wrapping_add(0x20);
        let corner = ((index % 3) << 4) as u32;
        let vertex = load(e, vertices.wrapping_add(corner));
        let low = base_low - margin;
        store(
            e,
            f + 0x20,
            [
                vertex[0] + low,
                vertex[1] + low,
                vertex[2] + low,
                vertex[3] + w_bias,
            ],
        );
        store(
            e,
            f + 0x30,
            [
                vertex[0] + margin,
                vertex[1] + margin,
                vertex[2] + margin,
                vertex[3] + w_bias,
            ],
        );
        e.mem.set_u32(f + 0xa0, f + 0xac);
        e.mem.set_u32(f + 0xa4, 0);
        e.mem.set_u32(f + 0xa8, 0x8000_0080);
        e.vcall(object.addr(), 0x30, &args![f + 0x20, f + 0xa0]);
        let mut consistent = true;
        let mut position = e.mem.i32(f + 0xa4).wrapping_sub(1);
        while position >= 0 {
            let keys = e.mem.u32(f + 0xa0);
            let key = e
                .mem
                .u32(keys.wrapping_add((position as u32).wrapping_mul(4)));
            if key != skip_key {
                let container = e.vcall(object.addr(), 0x10, &args![]).u32();
                let shape = e.vcall(container, 0x14, &args![key, f + 0x2b0]).u32();
                if e.mem.u32(shape.wrapping_add(0xc)) == 3 {
                    let found = e
                        .call(
                            MATCH_VERTICES,
                            &args![
                                vertices,
                                shape.wrapping_add(0x20),
                                index,
                                f + 0x40,
                                f + 0x18
                            ],
                        )
                        .u32();
                    if found == 4 {
                        if report_winding != 0 {
                            let first = e.mem.i32(f + 0x18);
                            let second = e.mem.i32(f + 0x1c);
                            if first != second.wrapping_add(1) % 3 {
                                if consistent {
                                    report_winding_assert(e, f, skip_key, key);
                                }
                                consistent = false;
                            }
                        }
                        let value = e.call(EDGE_VALUE, &args![f + 0x40]).u32();
                        let word = u32::from(e.mem.u16(flags.addr()));
                        let word = e.call(SET_FIELD, &args![word, index, value]).u16();
                        e.mem.set_u16(flags.addr(), word);
                    }
                }
            }
            position = position.wrapping_sub(1);
        }
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router.wrapping_add(0x10));
        let capacity = e.mem.u32(f + 0xa8);
        e.mem.set_u32(f + 0xa4, 0);
        if (capacity as i32) >= 0 {
            let data = e.mem.u32(f + 0xa0);
            e.vcall(
                allocator,
                0x10,
                &args![data, (capacity & 0x3fff_ffff).wrapping_mul(4)],
            );
        }
        !consistent
    })
}

/// The assertion of `00dde4c0`: builds `"Inconsistant triangle winding between at least
/// triangle <skip_key> and triangle <key>. One sided welding will not work."` in the buffer at
/// `frame + 0x2b0` through an `hkOstream` at `frame + 0x20` and reports it.
fn report_winding_assert(e: &mut Engine, frame: u32, skip_key: u32, key: u32) {
    let stream = frame + 0x20;
    let buffer = frame + 0x2b0;
    e.call(STREAM_BEGIN, &args![stream, buffer, 0x200u32]);
    let s = e.call(STREAM_TEXT, &args![stream, WINDING_TEXT_1]).u32();
    let s = e.call(STREAM_INT, &args![s, skip_key]).u32();
    let s = e.call(STREAM_TEXT, &args![s, WINDING_TEXT_2]).u32();
    let s = e.call(STREAM_INT, &args![s, key]).u32();
    e.call(STREAM_TEXT, &args![s, WINDING_TEXT_3]);
    let handler = e.mem.u32(ERROR_HANDLER);
    e.vcall(
        handler,
        0xc,
        &args![
            1u32,
            WINDING_ASSERT_ID,
            buffer,
            WINDING_ASSERT_FILE,
            WINDING_ASSERT_LINE
        ],
    );
    e.call(STREAM_END, &args![stream]);
}

// Translated from 00dde760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Welding with one record (the engine map has no name), cdecl with four words. Takes 0x80
/// bytes from the memory router's stack allocator (cursor at router `+0`), makes a one-element
/// array of 0x50-byte records whose element is a copy (`00ddea00`, 0x44 bytes) of the 64
/// bytes at `01268370` followed by `arg_2`, calls `00dddcd0(01268370, arg_1, arg_3, array,
/// arg_4)`, gives the cursor back, and frees the array's memory through the router's
/// allocator when the callee moved it (the flag word lost its top bit).
///
/// The first `if` of the exe (the array being full, `size == capacity`, `00c90940` called with
/// `(allocator, array, 0x50)`) cannot be true for a fresh one-element array; it is translated.
pub fn fn_00dde760(e: &mut Engine, arg_1: u32, arg_2: u32, arg_3: u32, arg_4: u32) {
    // The exe's frame: the array `{data, size, capacity and flags, cursor}` at `+0x10` and the
    // record at `+0x20` (16 words from `01268370`, then `arg_2`).
    e.with_stack(0x80, |e, frame| {
        let f = frame.addr();
        for word in 0..16 {
            let value = e.mem.u32(RECORD_TEMPLATE + 4 * word);
            e.mem.set_u32(f + 0x20 + 4 * word, value);
        }
        e.mem.set_u32(f + 0x60, arg_2);
        e.mem.set_u32(f + 0x10, 0);
        e.mem.set_u32(f + 0x14, 0);
        e.mem.set_u32(f + 0x18, 0x8000_0000);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let cursor = e.mem.u32(router);
        e.mem.set_u32(router, cursor.wrapping_add(0x80));
        e.mem.set_u32(f + 0x10, cursor);
        e.mem.set_u32(f + 0x18, 0x8000_0001);
        e.mem.set_u32(f + 0x1c, cursor);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let capacity = e.mem.u32(f + 0x18) & 0x3fff_ffff;
        if e.mem.u32(f + 0x14) == capacity {
            let allocator = e.mem.u32(router.wrapping_add(0x10));
            e.call(ARRAY_RESERVE_MORE, &args![allocator, f + 0x10, 0x50u32]);
        }
        let slot = e
            .mem
            .u32(f + 0x14)
            .wrapping_mul(0x50)
            .wrapping_add(e.mem.u32(f + 0x10));
        if slot != 0 {
            fn_00ddea00(e, Ptr::new(slot), Ptr::new(f + 0x20));
        }
        let size = e.mem.u32(f + 0x14);
        e.mem.set_u32(f + 0x14, size.wrapping_add(1));
        e.call(
            WELDING_MULTI_SHAPE,
            &args![RECORD_TEMPLATE, arg_1, arg_3, f + 0x10, arg_4],
        );
        let saved = e.mem.u32(f + 0x1c);
        if saved == e.mem.u32(f + 0x10) {
            e.mem.set_u32(f + 0x14, 0);
        }
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        e.mem.set_u32(router, saved);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router.wrapping_add(0x10));
        let capacity = e.mem.u32(f + 0x18);
        e.mem.set_u32(f + 0x14, 0);
        if (capacity as i32) >= 0 {
            let data = e.mem.u32(f + 0x10);
            e.vcall(
                allocator,
                0x10,
                &args![data, (capacity & 0x3fff_ffff).wrapping_mul(0x50)],
            );
        }
    });
}

// Translated from 00dde880 (decompiled, FalloutNV.exe 1.4.0.525)
/// Welds one triangle (the engine map has no name), cdecl with four words `(key, object,
/// arg_3, flags)`. Gets the child shape of `key` through `object` (vtable `0x10`, then slot
/// `0x14`). A degenerate triangle (`00ceb8d0` on the vertices at `+0x20`, `+0x30`, `+0x40`
/// with the tolerance at `011b2e90`) stores 0 in the 16-bit word at `flags` and returns 1.
/// Otherwise the word starts at 0 and `00dde4c0(object, shape, key, i, arg_3, word)` runs for
/// the three vertices `i`; the result is 1 when any of them returned 1.
pub fn fn_00dde880(e: &mut Engine, key: u32, object: Ptr, arg_3: u32, flags: Ptr) -> bool {
    // The exe's frame: the degenerate byte at `+0x17`, the word at `+0x18`, the result at
    // `+0x1c` and the shape buffer at `+0x20`.
    e.with_stack(0x220, |e, frame| {
        let f = frame.addr();
        let container = e.vcall(object.addr(), 0x10, &args![]).u32();
        let shape = e.vcall(container, 0x14, &args![key, f + 0x20]).u32();
        let tolerance = x87_quiet_bits(e.mem.u32(DEGENERATE_TOLERANCE));
        let degenerate_byte = e
            .call(
                IS_DEGENERATE,
                &args![
                    f + 0x17,
                    shape.wrapping_add(0x20),
                    shape.wrapping_add(0x30),
                    shape.wrapping_add(0x40),
                    tolerance
                ],
            )
            .u32();
        if e.mem.u8(degenerate_byte) != 0 {
            e.mem.set_u16(flags.addr(), 0);
            return true;
        }
        e.mem.set_u32(f + 0x18, 0);
        e.mem.set_u32(f + 0x1c, 0);
        for vertex in 0..3 {
            let welded = fn_00dde4c0(e, object, shape, key, vertex, arg_3, Ptr::new(f + 0x18));
            if welded {
                e.mem.set_u32(f + 0x1c, 1);
            }
        }
        let word = e.mem.u16(f + 0x18);
        e.mem.set_u16(flags.addr(), word);
        e.mem.u32(f + 0x1c) != 0
    })
}

// Translated from 00dde930 (decompiled, FalloutNV.exe 1.4.0.525)
/// Checks one triangle's winding without reporting (the engine map has no name), cdecl with
/// three words `(out, key, object)`; returns `out`. Gets the child shape of `key` through
/// `object` as `00dde880` does, then asks the error handler (`01268360`, slot `0x14`, `(byte,
/// 0xfe98751e)`) for a byte and, when it is set, calls slot `0x10` with `(0xfe98751e, handler
/// & 0xffffff00)` to switch the assertion off. Writes 1 to the byte at `out`, then runs
/// `00dde4c0(object, shape, key, i, 0, word)` for `i` in 0..3 and writes 0 at the first one that
/// returns 1. When the byte was set, slot `0x10` is called again with `(0xfe98751e,
/// (handler & 0xffffff00) | 1)`.
pub fn fn_00dde930(e: &mut Engine, out: Ptr, key: u32, object: Ptr) -> Ptr {
    // The exe's frame: the byte at `+0x17`, the word at `+0x18`, the shape at `+0x1c` and the
    // shape buffer at `+0x20`.
    e.with_stack(0x220, |e, frame| {
        let f = frame.addr();
        let container = e.vcall(object.addr(), 0x10, &args![]).u32();
        let shape = e.vcall(container, 0x14, &args![key, f + 0x20]).u32();
        e.mem.set_u32(f + 0x1c, shape);
        let handler = e.mem.u32(ERROR_HANDLER);
        e.vcall(handler, 0x14, &args![f + 0x17, SWITCHED_ASSERT_ID]);
        let was_set = e.mem.u8(f + 0x17) != 0;
        if was_set {
            let handler = e.mem.u32(ERROR_HANDLER);
            e.vcall(
                handler,
                0x10,
                &args![SWITCHED_ASSERT_ID, handler & 0xffff_ff00],
            );
        }
        e.mem.set_u8(out.addr(), 1);
        e.mem.set_u32(f + 0x18, 0);
        for vertex in 0..3 {
            let shape = e.mem.u32(f + 0x1c);
            let found = fn_00dde4c0(e, object, shape, key, vertex, 0, Ptr::new(f + 0x18));
            if found {
                e.mem.set_u8(out.addr(), 0);
                break;
            }
        }
        if e.mem.u8(f + 0x17) != 0 {
            let handler = e.mem.u32(ERROR_HANDLER);
            e.vcall(
                handler,
                0x10,
                &args![SWITCHED_ASSERT_ID, (handler & 0xffff_ff00) | 1],
            );
        }
        out
    })
}

// Translated from 00ddea00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copy constructor of a 0x44-byte record (the engine map has no name), thiscall with one
/// word: copies the 17 words at `source` into `this` (all read first, then written). Returns
/// `this`.
pub fn fn_00ddea00(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    let words: Vec<u32> = (0..17)
        .map(|word| e.mem.u32(source.addr().wrapping_add(4 * word)))
        .collect();
    for (word, value) in words.iter().enumerate() {
        e.mem
            .set_u32(this.addr().wrapping_add(4 * word as u32), *value);
    }
    this
}

// Translated from 00ddeab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Splice into an array of 12-byte elements (the engine map has no name; the body has the
/// shape of `hkArrayBase<T>::_spliceInto`), thiscall with five words `(allocator, index,
/// delete_count, source, insert_count)`. Replaces `delete_count` elements at `index` with the
/// `insert_count` elements at `source`: grows the array first when the new size exceeds its
/// capacity (`00c908c0` with `max(2 * capacity, new size)`), moves the tail with `00d2cc70`,
/// then copies the elements three words at a time (nothing is written to a null slot). The
/// exe reuses the caller's stack word of `insert_count` for the tail length; `insert_count`
/// itself is kept in a register.
pub fn fn_00ddeab0(
    e: &mut Engine,
    this: Ptr,
    allocator: u32,
    index: i32,
    delete_count: i32,
    source: u32,
    insert_count: i32,
) {
    let array = this.addr();
    let size = e.mem.i32(array + 4);
    let new_size = size.wrapping_sub(delete_count).wrapping_add(insert_count);
    let tail = size.wrapping_sub(index).wrapping_sub(delete_count);
    let capacity = (e.mem.u32(array + 8) & 0x3fff_ffff) as i32;
    if capacity < new_size {
        let doubled = capacity.wrapping_add(capacity);
        let wanted = if new_size < doubled {
            doubled
        } else {
            new_size
        };
        e.call(ARRAY_RESERVE, &args![allocator, array, wanted, 0xcu32]);
    }
    let data = e.mem.u32(array);
    let bytes = (tail as u32).wrapping_mul(12);
    let destination = data.wrapping_add((index.wrapping_add(insert_count) as u32).wrapping_mul(12));
    let from = data.wrapping_add((index.wrapping_add(delete_count) as u32).wrapping_mul(12));
    e.call(MEM_MOVE, &args![destination, from, bytes]);
    let data = e.mem.u32(array);
    let mut slot = data.wrapping_add((index as u32).wrapping_mul(12));
    if insert_count > 0 {
        let distance = source.wrapping_sub(slot);
        for _ in 0..insert_count {
            if slot != 0 {
                for word in 0..3 {
                    let value = e
                        .mem
                        .u32(distance.wrapping_add(slot).wrapping_add(4 * word));
                    e.mem.set_u32(slot.wrapping_add(4 * word), value);
                }
            }
            slot = slot.wrapping_add(12);
        }
    }
    e.mem.set_i32(array + 4, new_size);
}

// Translated from 00ddeb70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Quicksort of 12-byte records by the order `00ddcda0` gives (the engine map has no name),
/// cdecl with four words `(data, low, high, _unused_4)`: the pivot is a copy of the middle
/// record `(low + high) >> 1`; `i` advances while `00ddcda0(data[i], pivot)` holds, `j` goes
/// down while `00ddcda0(pivot, data[j])` holds, records are swapped while `i <= j`, then the
/// lower part is sorted by recursion (when `low < j`) and the upper part by looping (while `i
/// < high`). The fourth word is passed on to the recursive call and never read.
pub fn fn_00ddeb70(e: &mut Engine, data: u32, low: i32, high: i32, _unused_4: u32) {
    // The exe's frame: the two result bytes at `+0xe` / `+0xf`, the pivot at `+0x10` and a
    // spare record at `+0x24`.
    e.with_stack(0x30, |e, frame| {
        let f = frame.addr();
        let record = |index: i32| data.wrapping_add((index as u32).wrapping_mul(12));
        let mut low = low;
        loop {
            let mut i = low;
            let mut j = high;
            let middle = record(low.wrapping_add(high) >> 1);
            for word in 0..3 {
                let value = e.mem.u32(middle + 4 * word);
                e.mem.set_u32(f + 0x10 + 4 * word, value);
            }
            loop {
                let mut at = record(i);
                let mut less = e.call(RECORD_LESS, &args![f + 0xe, at, f + 0x10]).u32();
                while e.mem.u8(less) != 0 {
                    at = at.wrapping_add(12);
                    i = i.wrapping_add(1);
                    less = e.call(RECORD_LESS, &args![f + 0xe, at, f + 0x10]).u32();
                }
                let mut at = record(j);
                let mut less = e.call(RECORD_LESS, &args![f + 0xf, f + 0x10, at]).u32();
                while e.mem.u8(less) != 0 {
                    at = at.wrapping_sub(12);
                    less = e.call(RECORD_LESS, &args![f + 0xf, f + 0x10, at]).u32();
                    j = j.wrapping_sub(1);
                }
                if j < i {
                    break;
                }
                if j != i {
                    let upper = record(j);
                    let lower = record(i);
                    let first = e.mem.u32(upper);
                    let second = e.mem.u32(upper + 4);
                    let third = e.mem.u32(upper + 8);
                    e.mem.set_u32(f + 0x24, second);
                    e.mem.set_u32(f + 0x28, third);
                    for word in 0..3 {
                        let value = e.mem.u32(lower + 4 * word);
                        e.mem.set_u32(upper + 4 * word, value);
                    }
                    let second = e.mem.u32(f + 0x24);
                    let third = e.mem.u32(f + 0x28);
                    e.mem.set_u32(lower, first);
                    e.mem.set_u32(lower + 4, second);
                    e.mem.set_u32(lower + 8, third);
                }
                j = j.wrapping_sub(1);
                i = i.wrapping_add(1);
                if i > j {
                    break;
                }
            }
            if low < j {
                fn_00ddeb70(e, data, low, j, _unused_4);
            }
            if i < high {
                low = i;
                continue;
            }
            break;
        }
    });
}

// Translated from 00ddecb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Insert elements into an array of 12-byte elements (the engine map has no name; the shape of
/// `hkArrayBase<T>::_insertAt`), thiscall with four words `(allocator, index, source,
/// count)`: `00ddeab0` with nothing deleted.
pub fn fn_00ddecb0(e: &mut Engine, this: Ptr, allocator: u32, index: i32, source: u32, count: i32) {
    fn_00ddeab0(e, this, allocator, index, 0, source, count);
}

// Translated from 00ddecd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Insert one 12-byte element (the engine map has no name; the shape of
/// `hkArrayBase<T>::_insertAt` for one element), thiscall with three words `(allocator, index,
/// source)`: `00ddecb0` with a count of 1.
pub fn fn_00ddecd0(e: &mut Engine, this: Ptr, allocator: u32, index: i32, source: u32) {
    fn_00ddecb0(e, this, allocator, index, source, 1);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00dd_e4c0, fn_00dde4c0(Ptr, u32, u32, i32, u32, Ptr) -> bool),
        entry!(0x00dd_e760, fn_00dde760(u32, u32, u32, u32)),
        entry!(0x00dd_e880, fn_00dde880(u32, Ptr, u32, Ptr) -> bool),
        entry!(0x00dd_e930, fn_00dde930(Ptr, u32, Ptr) -> Ptr),
        entry!(0x00dd_ea00, fn_00ddea00(Ptr, Ptr) -> Ptr),
        entry!(0x00dd_eab0, fn_00ddeab0(Ptr, u32, i32, i32, u32, i32)),
        entry!(0x00dd_eb70, fn_00ddeb70(u32, i32, i32, u32)),
        entry!(0x00dd_ecb0, fn_00ddecb0(Ptr, u32, i32, u32, i32)),
        entry!(0x00dd_ecd0, fn_00ddecd0(Ptr, u32, i32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<Vec<u32>>>>;

    const OBJECT_SLOT_BOX: u32 = 0x0900_0030;
    const OBJECT_SLOT_CONTAINER: u32 = 0x0900_0010;
    const CONTAINER_SLOT_SHAPE: u32 = 0x0900_0114;
    const ASSERT_SLOT: u32 = 0x0900_0900;
    const FREE_SLOT: u32 = 0x0900_0910;
    const ROUTER_CELL: u32 = 0x0126_8100;

    fn bits(value: f32) -> u32 {
        value.to_bits()
    }

    /// An engine with the constants and globals the functions read mapped.
    fn base_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0101_0000, 0x1_0000);
        e.map(0x010c_4000, 0x1000);
        e.map(0x011b_2000, 0x1000);
        e.map(0x0126_8000, 0x1000);
        e.mem.set_u32(MARGIN, bits(0.5));
        e.mem.set_u32(BOX_BASE, 0x8000_0000);
        e.mem.set_u32(BOX_W, bits(0.0));
        e.mem.set_u32(DEGENERATE_TOLERANCE, bits(0.25));
        e
    }

    /// Doubles that append `[tag, words..]` to the log and return `eax`.
    fn logging_double(e: &mut Engine, address: u32, log: &Log, tag: u32, eax: u32) {
        let log = log.clone();
        e.register_double(address, move |_, words| {
            let mut row = vec![tag];
            row.extend_from_slice(words);
            log.borrow_mut().push(row);
            Ret {
                eax,
                ..Ret::default()
            }
        });
    }

    /// The memory router with an allocator whose free slot logs `[8, allocator, data, size]`;
    /// returns the allocator.
    fn router(e: &mut Engine, log: &Log) -> u32 {
        let router = e.mem.alloc(0x20);
        let allocator = e.mem.alloc(8);
        let allocator_vtable = e.mem.alloc(0x20);
        e.mem.set_u32(allocator, allocator_vtable);
        e.mem.set_u32(allocator_vtable + 0x10, FREE_SLOT);
        e.mem.set_u32(router + 0x10, allocator);
        e.mem.set_u32(ROUTER_CELL, router);
        e.register(MEMORY_ROUTER, |e, _| Ret {
            eax: e.mem.u32(ROUTER_CELL),
            ..Ret::default()
        });
        logging_double(e, FREE_SLOT, log, 8, 0);
        allocator
    }

    /// A collection object whose container (vtable `0x10`) hands out `shape` for every key;
    /// slot `0x30` is left to the caller.
    fn collection(e: &mut Engine, shape: u32) -> u32 {
        let object = e.mem.alloc(8);
        let container = e.mem.alloc(8);
        let object_vtable = e.mem.alloc(0x40);
        let container_vtable = e.mem.alloc(0x40);
        e.mem.set_u32(object, object_vtable);
        e.mem.set_u32(container, container_vtable);
        e.mem.set_u32(object + 4, container);
        e.mem.set_u32(object_vtable + 0x10, OBJECT_SLOT_CONTAINER);
        e.mem.set_u32(object_vtable + 0x30, OBJECT_SLOT_BOX);
        e.mem.set_u32(container_vtable + 0x14, CONTAINER_SLOT_SHAPE);
        e.register(OBJECT_SLOT_CONTAINER, |e, words| Ret {
            eax: e.mem.u32(words[0] + 4),
            ..Ret::default()
        });
        e.register_double(CONTAINER_SLOT_SHAPE, move |_, _| Ret {
            eax: shape,
            ..Ret::default()
        });
        object
    }

    // ----- 00dde4c0 -----

    struct Rig {
        e: Engine,
        object: u32,
        triangle: u32,
        log: Log,
        flags: u32,
        allocator: u32,
    }

    /// A collection whose box query finds `keys`, whose child shape has type `shape_type`,
    /// with doubles for every callee. `00ddd6f0` returns `matches` and writes `pair`.
    fn rig(keys: Vec<u32>, shape_type: u32, matches: u32, pair: (u32, u32)) -> Rig {
        let mut e = base_engine();
        let log: Log = Rc::default();
        let shape = e.mem.alloc(0x60);
        e.mem.set_u32(shape + 0xc, shape_type);
        let object = collection(&mut e, shape);
        let triangle = e.mem.alloc(0x80);
        for (vertex, base) in [(0u32, 1.0f32), (1, 2.0), (2, 3.0)] {
            for lane in 0..4 {
                e.mem.set_f32(
                    triangle + 0x20 + 0x10 * vertex + 4 * lane,
                    base + lane as f32 * 0.25,
                );
            }
        }
        let flags = e.mem.alloc(8);
        e.mem.set_u16(flags, 0x1234);
        {
            let log = log.clone();
            e.register_double(OBJECT_SLOT_BOX, move |e, words| {
                let (aabb, array) = (words[1], words[2]);
                let mut row = vec![1];
                row.extend((0..8).map(|lane| e.mem.u32(aabb + 4 * lane)));
                log.borrow_mut().push(row);
                let data = e.mem.u32(array);
                for (i, key) in keys.iter().enumerate() {
                    e.mem.set_u32(data + 4 * i as u32, *key);
                }
                e.mem.set_u32(array + 4, keys.len() as u32);
                Ret::default()
            });
        }
        {
            let log = log.clone();
            e.register_double(MATCH_VERTICES, move |e, words| {
                log.borrow_mut().push(vec![3, words[0], words[1], words[2]]);
                e.mem.set_u32(words[4], pair.0);
                e.mem.set_u32(words[4] + 4, pair.1);
                Ret {
                    eax: matches,
                    ..Ret::default()
                }
            });
        }
        logging_double(&mut e, EDGE_VALUE, &log, 4, 0x55);
        e.register(SET_FIELD, |_, words| Ret {
            eax: (words[0] << 4 | words[2]) & 0xffff,
            ..Ret::default()
        });
        logging_double(&mut e, STREAM_BEGIN, &log, 5, 0);
        e.register(STREAM_TEXT, |_, words| Ret {
            eax: words[0],
            ..Ret::default()
        });
        e.register(STREAM_INT, |_, words| Ret {
            eax: words[0],
            ..Ret::default()
        });
        logging_double(&mut e, STREAM_END, &log, 6, 0);
        let handler = e.mem.alloc(8);
        let handler_vtable = e.mem.alloc(0x20);
        e.mem.set_u32(handler, handler_vtable);
        e.mem.set_u32(handler_vtable + 0xc, ASSERT_SLOT);
        e.mem.set_u32(ERROR_HANDLER, handler);
        logging_double(&mut e, ASSERT_SLOT, &log, 7, 0);
        let allocator = router(&mut e, &log);
        Rig {
            e,
            object,
            triangle,
            log,
            flags,
            allocator,
        }
    }

    fn run_weld(r: &mut Rig, skip: u32, index: i32, winding: u32) -> bool {
        r.e.call(
            0x00dd_e4c0,
            &args![r.object, r.triangle, skip, index, winding, r.flags],
        )
        .bool()
    }

    #[test]
    fn the_box_is_the_vertex_grown_by_the_margin() {
        let mut r = rig(vec![], 3, 4, (0, 0));
        // index 4 is vertex 1: (2.0, 2.25, 2.5, 2.75); margin 0.5, base -0.0, w bias 0.0
        assert!(!run_weld(&mut r, 9, 4, 0));
        let log = r.log.borrow();
        let want: Vec<u32> = [1.5, 1.75, 2.0, 2.75, 2.5, 2.75, 3.0, 2.75]
            .iter()
            .map(|v: &f32| v.to_bits())
            .collect();
        assert_eq!(log[0][0], 1);
        assert_eq!(log[0][1..], want[..]);
        // no keys: nothing else runs, and the inline array is not freed
        assert_eq!(log.len(), 1);
    }

    #[test]
    fn a_matching_triangle_updates_the_word() {
        let mut r = rig(vec![7], 3, 4, (0, 0));
        assert!(!run_weld(&mut r, 9, 1, 0));
        // the edge value 0x55 goes into the word through the set-field double
        assert_eq!(r.e.mem.u16(r.flags), 0x2355);
        let log = r.log.borrow();
        let match_call = log.iter().find(|row| row[0] == 3).unwrap();
        assert_eq!(match_call[1], r.triangle + 0x20);
        assert_eq!(match_call[3], 1);
    }

    #[test]
    fn the_skipped_key_and_other_shape_types_do_nothing() {
        let mut r = rig(vec![9], 3, 4, (0, 0));
        run_weld(&mut r, 9, 0, 0);
        assert!(r.log.borrow().iter().all(|row| row[0] != 3));
        let mut r = rig(vec![7], 2, 4, (0, 0));
        run_weld(&mut r, 9, 0, 0);
        assert!(r.log.borrow().iter().all(|row| row[0] != 3));
        assert_eq!(r.e.mem.u16(r.flags), 0x1234);
    }

    #[test]
    fn anything_but_four_matches_does_nothing() {
        let mut r = rig(vec![7], 3, 5, (0, 0));
        run_weld(&mut r, 9, 0, 0);
        assert_eq!(r.e.mem.u16(r.flags), 0x1234);
    }

    #[test]
    fn a_wrong_winding_is_reported_once_and_makes_the_result_true() {
        // pair (0, 2): (2 + 1) % 3 == 0 == first: consistent
        let mut r = rig(vec![7, 8], 3, 4, (0, 2));
        assert!(!run_weld(&mut r, 9, 0, 1));
        assert!(r.log.borrow().iter().all(|row| row[0] != 5));
        // pair (1, 2): 0 != 1: inconsistent, reported once for two keys
        let mut r = rig(vec![7, 8], 3, 4, (1, 2));
        assert!(run_weld(&mut r, 9, 0, 1));
        let log = r.log.borrow();
        let begins: Vec<_> = log.iter().filter(|row| row[0] == 5).collect();
        assert_eq!(begins.len(), 1);
        assert_eq!(begins[0][3], 0x200);
        let report: Vec<_> = log.iter().filter(|row| row[0] == 7).collect();
        assert_eq!(report.len(), 1);
        assert_eq!(
            report[0][2..],
            [1, 0xabba_751e, begins[0][2], 0x010d_7f50, 0x2c9]
        );
        assert_eq!(log.iter().filter(|row| row[0] == 6).count(), 1);
    }

    #[test]
    fn winding_is_not_checked_when_not_asked() {
        let mut r = rig(vec![7], 3, 4, (1, 2));
        assert!(!run_weld(&mut r, 9, 0, 0));
        assert!(r.log.borrow().iter().all(|row| row[0] != 5));
    }

    #[test]
    fn a_moved_key_array_is_freed_through_the_allocator() {
        let mut r = rig(vec![], 3, 4, (0, 0));
        // the box query clears the "not heap" flag and replaces the data pointer
        r.e.register(OBJECT_SLOT_BOX, |e, words| {
            let array = words[2];
            e.mem.set_u32(array, 0x0777_0000);
            e.mem.set_u32(array + 8, 0x20);
            Ret::default()
        });
        run_weld(&mut r, 9, 0, 0);
        let log = r.log.borrow();
        assert_eq!(log.iter().filter(|row| row[0] == 8).count(), 1);
        assert_eq!(log[0], vec![8, r.allocator, 0x0777_0000, 0x80]);
    }

    // ----- 00dde760 -----

    fn record_rig(log: &Log) -> (Engine, u32, u32) {
        let mut e = base_engine();
        let allocator = router(&mut e, log);
        let router_object = e.mem.u32(ROUTER_CELL);
        let block = e.mem.alloc(0x100);
        e.mem.set_u32(router_object, block);
        for word in 0..16u32 {
            e.mem.set_u32(RECORD_TEMPLATE + 4 * word, 0x100 + word);
        }
        (e, block, allocator)
    }

    #[test]
    fn welding_with_a_record_builds_the_array_and_gives_the_cursor_back() {
        let log: Log = Rc::default();
        let (mut e, block, _) = record_rig(&log);
        let router_object = e.mem.u32(ROUTER_CELL);
        {
            let log = log.clone();
            e.register_double(WELDING_MULTI_SHAPE, move |e, words| {
                let array = words[3];
                let data = e.mem.u32(array);
                let mut row = vec![
                    e.mem.u32(router_object),
                    words[0],
                    words[1],
                    words[2],
                    words[4],
                    data,
                    e.mem.u32(array + 4),
                    e.mem.u32(array + 8),
                ];
                row.extend((0..17).map(|w| e.mem.u32(data + 4 * w)));
                log.borrow_mut().push(row);
                Ret::default()
            });
        }
        e.call(0x00dd_e760, &args![11u32, 22u32, 33u32, 44u32]);
        let log = log.borrow();
        assert_eq!(log.len(), 1);
        let row = &log[0];
        assert_eq!(
            row[..8],
            [
                block + 0x80,
                RECORD_TEMPLATE,
                11,
                33,
                44,
                block,
                1,
                0x8000_0001
            ]
        );
        let want: Vec<u32> = (0..16).map(|w| 0x100 + w).chain([22]).collect();
        assert_eq!(row[8..], want[..]);
        assert_eq!(e.mem.u32(router_object), block, "cursor restored");
    }

    #[test]
    fn a_moved_record_array_is_freed_with_its_size() {
        let log: Log = Rc::default();
        let (mut e, _, allocator) = record_rig(&log);
        e.register(WELDING_MULTI_SHAPE, |e, words| {
            let array = words[3];
            e.mem.set_u32(array, 0x0888_0000);
            e.mem.set_u32(array + 8, 3);
            Ret::default()
        });
        e.call(0x00dd_e760, &args![0u32, 0u32, 0u32, 0u32]);
        assert_eq!(
            *log.borrow(),
            vec![vec![8, allocator, 0x0888_0000, 3 * 0x50]]
        );
    }

    // ----- 00dde880 -----

    #[test]
    fn a_degenerate_triangle_clears_the_word_and_answers_true() {
        let mut e = base_engine();
        let log: Log = Rc::default();
        let shape = e.mem.alloc(0x60);
        let object = collection(&mut e, shape);
        {
            let log = log.clone();
            e.register_double(IS_DEGENERATE, move |e, words| {
                log.borrow_mut().push(words.to_vec());
                e.mem.set_u8(words[0], 1);
                Ret {
                    eax: words[0],
                    ..Ret::default()
                }
            });
        }
        let flags = e.mem.alloc(8);
        e.mem.set_u16(flags, 0xffff);
        let result = e
            .call(0x00dd_e880, &args![5u32, object, 1u32, flags])
            .bool();
        assert!(result);
        assert_eq!(e.mem.u16(flags), 0);
        let log = log.borrow();
        assert_eq!(log.len(), 1);
        assert_eq!(
            log[0][1..],
            [shape + 0x20, shape + 0x30, shape + 0x40, bits(0.25)]
        );
    }

    #[test]
    fn a_proper_triangle_runs_the_three_vertex_steps() {
        let mut e = base_engine();
        let log: Log = Rc::default();
        let shape = e.mem.alloc(0x60);
        e.mem.set_u32(shape + 0xc, 3);
        let object = collection(&mut e, shape);
        e.register(IS_DEGENERATE, |e, words| {
            e.mem.set_u8(words[0], 0);
            Ret {
                eax: words[0],
                ..Ret::default()
            }
        });
        // every vertex step finds one neighbour with two shared vertices
        e.register_double(OBJECT_SLOT_BOX, |e, words| {
            let array = words[2];
            let data = e.mem.u32(array);
            e.mem.set_u32(data, 7);
            e.mem.set_u32(array + 4, 1);
            Ret::default()
        });
        logging_double(&mut e, MATCH_VERTICES, &log, 3, 4);
        logging_double(&mut e, EDGE_VALUE, &log, 4, 0x55);
        e.register(SET_FIELD, |_, words| Ret {
            eax: words[0] | 1 << words[1],
            ..Ret::default()
        });
        router(&mut e, &log);
        let flags = e.mem.alloc(8);
        e.mem.set_u16(flags, 0xffff);
        let result = e
            .call(0x00dd_e880, &args![5u32, object, 0u32, flags])
            .bool();
        assert!(!result);
        // the word started at 0 and gained bits 0, 1 and 2, one per vertex step
        assert_eq!(e.mem.u16(flags), 7);
        let log = log.borrow();
        let indexes: Vec<u32> = log
            .iter()
            .filter(|row| row[0] == 3)
            .map(|row| row[3])
            .collect();
        assert_eq!(indexes, vec![0, 1, 2]);
    }

    // ----- 00dde930 -----

    /// An error handler whose slot `0x14` writes `byte` for its id and logs `[10, id]`, and
    /// whose slot `0x10` logs `[11, handler, id, value]`; returns the handler.
    fn error_handler(r: &mut Rig, byte: u8) -> u32 {
        let handler = r.e.mem.u32(ERROR_HANDLER);
        let handler_vtable = r.e.mem.u32(handler);
        r.e.mem.set_u32(handler_vtable + 0x14, 0x0900_0914);
        r.e.mem.set_u32(handler_vtable + 0x10, 0x0900_0920);
        let log = r.log.clone();
        r.e.register_double(0x0900_0914, move |e, words| {
            log.borrow_mut().push(vec![10, words[2]]);
            e.mem.set_u8(words[1], byte);
            Ret::default()
        });
        logging_double(&mut r.e, 0x0900_0920, &r.log, 11, 0);
        handler
    }

    #[test]
    fn the_winding_check_switches_the_assertion_off_and_back_on() {
        let mut r = rig(vec![7], 3, 4, (0, 0));
        let handler = error_handler(&mut r, 1);
        let out = r.e.mem.alloc(8);
        let result = r.e.call(0x00dd_e930, &args![out, 5u32, r.object]).u32();
        assert_eq!(result, out);
        // winding is never reported with the flag 0 the function passes: the byte stays 1
        assert_eq!(r.e.mem.u8(out), 1);
        let log = r.log.borrow();
        let ask: Vec<_> = log.iter().filter(|row| row[0] == 10).collect();
        assert_eq!(ask, vec![&vec![10, 0xfe98_751e]]);
        let switch: Vec<_> = log.iter().filter(|row| row[0] == 11).collect();
        assert_eq!(switch.len(), 2);
        assert_eq!(switch[0][2..], [0xfe98_751e, handler & 0xffff_ff00]);
        assert_eq!(switch[1][2..], [0xfe98_751e, (handler & 0xffff_ff00) | 1]);
    }

    #[test]
    fn the_winding_check_leaves_the_assertion_alone_when_it_was_off() {
        let mut r = rig(vec![7], 3, 4, (0, 0));
        error_handler(&mut r, 0);
        let out = r.e.mem.alloc(8);
        r.e.call(0x00dd_e930, &args![out, 5u32, r.object]);
        assert_eq!(r.e.mem.u8(out), 1);
        assert!(r.log.borrow().iter().all(|row| row[0] != 11));
    }

    // ----- 00ddea00 -----

    #[test]
    fn the_record_copy_moves_seventeen_words_and_returns_this() {
        let mut e = Engine::new();
        let source = e.mem.alloc(0x50);
        let this = e.mem.alloc(0x50);
        for word in 0..20 {
            e.mem.set_u32(source + 4 * word, 0x10 + word);
        }
        let result = e.call(0x00dd_ea00, &args![this, source]);
        assert_eq!(result.u32(), this);
        for word in 0..17 {
            assert_eq!(e.mem.u32(this + 4 * word), 0x10 + word);
        }
        assert_eq!(e.mem.u32(this + 0x44), 0);
    }

    // ----- 00ddeab0, 00ddecb0, 00ddecd0 -----

    /// An array of 12-byte records with `values` (the first word of each) and the stated
    /// capacity; the block behind it always has room for 16 records.
    fn array_of(e: &mut Engine, values: &[u32], capacity: u32) -> u32 {
        let array = e.mem.alloc(12);
        let data = e.mem.alloc(12 * 16);
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 4, values.len() as u32);
        e.mem.set_u32(array + 8, capacity);
        for (i, v) in values.iter().enumerate() {
            e.mem.set_u32(data + 12 * i as u32, *v);
            e.mem.set_u32(data + 12 * i as u32 + 4, *v + 1);
            e.mem.set_u32(data + 12 * i as u32 + 8, *v + 2);
        }
        array
    }

    fn first_words(e: &Engine, array: u32) -> Vec<u32> {
        let data = e.mem.u32(array);
        (0..e.mem.u32(array + 4))
            .map(|i| e.mem.u32(data + 12 * i))
            .collect()
    }

    /// A move double that copies the bytes and logs `[0, words..]`.
    fn move_double(e: &mut Engine, log: &Log) {
        let log = log.clone();
        e.register_double(MEM_MOVE, move |e, words| {
            let mut row = vec![0];
            row.extend_from_slice(words);
            log.borrow_mut().push(row);
            let bytes = e.mem.bytes(words[1], words[2]);
            e.mem.write(words[0], &bytes);
            Ret::default()
        });
    }

    #[test]
    fn splicing_replaces_elements_and_moves_the_tail() {
        let mut e = Engine::new();
        let log: Log = Rc::default();
        move_double(&mut e, &log);
        let array = array_of(&mut e, &[10, 20, 30, 40], 8);
        let source = array_of(&mut e, &[1, 2], 2);
        let source_data = e.mem.u32(source);
        // at index 1 replace two elements by two new ones
        e.call(
            0x00dd_eab0,
            &args![array, 0u32, 1u32, 2u32, source_data, 2u32],
        );
        assert_eq!(first_words(&e, array), vec![10, 1, 2, 40]);
        let data = e.mem.u32(array);
        assert_eq!(*log.borrow(), vec![vec![0, data + 36, data + 36, 12]]);
    }

    #[test]
    fn splicing_grows_to_the_larger_of_double_and_new_size() {
        let mut e = Engine::new();
        let log: Log = Rc::default();
        move_double(&mut e, &Rc::default());
        logging_double(&mut e, ARRAY_RESERVE, &log, 1, 0);
        let source = array_of(&mut e, &[1, 2, 3, 4], 4);
        let source_data = e.mem.u32(source);
        // capacity 3, size 3: inserting 3 gives 6 == 2 * 3: wanted 6
        let array = array_of(&mut e, &[10, 20, 30], 3);
        e.call(
            0x00dd_eab0,
            &args![array, 0x77u32, 3u32, 0u32, source_data, 3u32],
        );
        // inserting 4 more into size 6 (the double leaves the capacity at 3): wanted 10
        e.call(
            0x00dd_eab0,
            &args![array, 0x77u32, 0u32, 0u32, source_data, 4u32],
        );
        // capacity 4, size 4: inserting 1 gives 5, below 2 * 4: wanted 8
        let other = array_of(&mut e, &[10, 20, 30, 40], 4);
        e.call(
            0x00dd_eab0,
            &args![other, 0x88u32, 0u32, 0u32, source_data, 1u32],
        );
        let log = log.borrow();
        assert_eq!(log.len(), 3);
        assert_eq!(log[0][1..], [0x77, array, 6, 12]);
        assert_eq!(log[1][1..], [0x77, array, 10, 12]);
        assert_eq!(log[2][1..], [0x88, other, 8, 12]);
    }

    #[test]
    fn a_zero_insert_count_only_deletes() {
        let mut e = Engine::new();
        let log: Log = Rc::default();
        move_double(&mut e, &log);
        let array = array_of(&mut e, &[10, 20, 30, 40], 8);
        e.call(0x00dd_eab0, &args![array, 0u32, 0u32, 1u32, 0u32, 0u32]);
        assert_eq!(first_words(&e, array), vec![20, 30, 40]);
    }

    #[test]
    fn insert_at_is_a_splice_without_deletion_and_insert_one_adds_a_record() {
        let mut e = Engine::new();
        let log: Log = Rc::default();
        move_double(&mut e, &log);
        let array = array_of(&mut e, &[10, 20, 30], 8);
        let source = array_of(&mut e, &[5, 6], 2);
        let source_data = e.mem.u32(source);
        e.call(0x00dd_ecb0, &args![array, 0u32, 1u32, source_data, 2u32]);
        assert_eq!(first_words(&e, array), vec![10, 5, 6, 20, 30]);
        e.call(0x00dd_ecd0, &args![array, 0u32, 0u32, source_data]);
        assert_eq!(first_words(&e, array), vec![5, 10, 5, 6, 20, 30]);
    }

    // ----- 00ddeb70 -----

    /// A comparison double: `a < b` by the third word taken as a float.
    fn less_double(e: &mut Engine) {
        e.register(RECORD_LESS, |e, words| {
            let a = e.mem.f32(words[1] + 8);
            let b = e.mem.f32(words[2] + 8);
            e.mem.set_u8(words[0], u8::from(a < b));
            Ret {
                eax: words[0],
                ..Ret::default()
            }
        });
    }

    fn records(e: &mut Engine, keys: &[f32]) -> u32 {
        let data = e.mem.alloc(12 * keys.len() as u32);
        for (i, key) in keys.iter().enumerate() {
            e.mem.set_u32(data + 12 * i as u32, i as u32);
            e.mem.set_u32(data + 12 * i as u32 + 4, 0xaa);
            e.mem.set_f32(data + 12 * i as u32 + 8, *key);
        }
        data
    }

    fn sorted_keys(e: &Engine, data: u32, n: u32) -> Vec<f32> {
        (0..n).map(|i| e.mem.f32(data + 12 * i + 8)).collect()
    }

    #[test]
    fn the_quicksort_orders_records_by_their_third_float() {
        let mut e = Engine::new();
        less_double(&mut e);
        let keys = [5.0, 1.0, 4.0, 2.0, 9.0, 0.5, 7.0, 3.0, 8.0, 6.0, 1.5];
        let data = records(&mut e, &keys);
        e.call(0x00dd_eb70, &args![data, 0u32, 10u32, 0u32]);
        let got = sorted_keys(&e, data, 11);
        let mut want = keys.to_vec();
        want.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(got, want);
        // the other words moved along with their key
        for i in 0..11 {
            let key = e.mem.f32(data + 12 * i + 8);
            let original = keys.iter().position(|k| *k == key).unwrap() as u32;
            assert_eq!(e.mem.u32(data + 12 * i), original);
            assert_eq!(e.mem.u32(data + 12 * i + 4), 0xaa);
        }
    }

    #[test]
    fn a_range_sorts_only_its_part_and_one_record_is_left_alone() {
        let mut e = Engine::new();
        less_double(&mut e);
        let data = records(&mut e, &[9.0, 3.0, 2.0, 1.0, 0.0]);
        e.call(0x00dd_eb70, &args![data, 1u32, 3u32, 0u32]);
        assert_eq!(sorted_keys(&e, data, 5), vec![9.0, 1.0, 2.0, 3.0, 0.0]);
        let data = records(&mut e, &[2.0, 1.0]);
        e.call(0x00dd_eb70, &args![data, 0u32, 0u32, 0u32]);
        assert_eq!(sorted_keys(&e, data, 2), vec![2.0, 1.0]);
    }

    #[test]
    fn records_with_equal_keys_all_survive() {
        let mut e = Engine::new();
        less_double(&mut e);
        let data = records(&mut e, &[2.0, 2.0, 1.0, 2.0, 1.0, 2.0]);
        e.call(0x00dd_eb70, &args![data, 0u32, 5u32, 0u32]);
        assert_eq!(sorted_keys(&e, data, 6), vec![1.0, 1.0, 2.0, 2.0, 2.0, 2.0]);
        let mut firsts: Vec<u32> = (0..6).map(|i| e.mem.u32(data + 12 * i)).collect();
        firsts.sort_unstable();
        assert_eq!(firsts, vec![0, 1, 2, 3, 4, 5]);
    }
}

//! `Havok SDK/run_00d4b1a0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00d4b1a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. Builds a temporary object in a 0x70-byte stack buffer by calling `00d4a170` on it (flag word 0; the callee pops it) and returns the first word of that buffer, which is what the code reads back after the call. The class is unconfirmed.
pub fn fn_00d4b1a0(e: &mut Engine) -> u32 {
    e.with_stack(0x70, |e, buffer| {
        e.call(0x00d4_a170, &args![buffer, 0u32]);
        e.mem.u32(buffer.addr())
    })
}

// Translated from 00d4b200 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. Builds a temporary object in a 0x30-byte stack buffer by calling `00d49c40` on it (flag word 0; the callee pops it) and returns the first word of that buffer, which is what the code reads back after the call. The class is unconfirmed.
pub fn fn_00d4b200(e: &mut Engine) -> u32 {
    e.with_stack(0x30, |e, buffer| {
        e.call(0x00d4_9c40, &args![buffer, 0u32]);
        e.mem.u32(buffer.addr())
    })
}

// Translated from 00d4b260 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. Builds a temporary object in a 0x110-byte stack buffer by calling `00d4a180` on it (flag word 0; the callee pops it) and returns the first word of that buffer, which is what the code reads back after the call. The class is unconfirmed.
pub fn fn_00d4b260(e: &mut Engine) -> u32 {
    e.with_stack(0x110, |e, buffer| {
        e.call(0x00d4_a180, &args![buffer, 0u32]);
        e.mem.u32(buffer.addr())
    })
}

// Translated from 00d4b330 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. Builds a temporary object in a 0xf0-byte stack buffer by calling `00d56f10` on it (flag word 0; the callee pops it) and returns the first word of that buffer, which is what the code reads back after the call. The class is unconfirmed.
pub fn fn_00d4b330(e: &mut Engine) -> u32 {
    e.with_stack(0xf0, |e, buffer| {
        e.call(0x00d5_6f10, &args![buffer, 0u32]);
        e.mem.u32(buffer.addr())
    })
}

/// The memory router (`00c85750`): its field `+0x10` is the allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkArrayUtil::_reserve` (`00c908c0`, cdecl `(allocator, array, count, element size)`) and the
/// one-element grow `00c90940` (cdecl `(allocator, array, element size)`).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
const ARRAY_GROW: u32 = 0x00c9_0940;

/// Room for one more element of an `hkArray`: after `00c85750`, when the size at `+4` equals the
/// capacity bits of `+8` the array grows through `00c90940` with the router's allocator and
/// `element_size`. The caller stores the element and increments the size.
fn reserve_one_00d4b480(e: &mut Engine, array: u32, element_size: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
    if e.mem.u32(array + 4) == capacity {
        let allocator = e.mem.u32(router + 0x10);
        e.call(ARRAY_GROW, &args![allocator, array, element_size]);
    }
}

/// Room for `extra` more elements: after `00c85750`, when the capacity bits of `+8` are below
/// `size + extra` (signed) the array is reserved through `00c908c0` for `max(size + extra,
/// 2 * capacity)` elements; the size becomes `size + extra`. Answers the address of the first new
/// element (`data + old size * element_size`, the data pointer read after the reserve).
fn grow_by_00d4b480(e: &mut Engine, array: u32, extra: u32, element_size: u32) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = (e.mem.u32(array + 8) & 0x3fff_ffff) as i32;
    let size = e.mem.u32(array + 4);
    let need = size.wrapping_add(extra) as i32;
    if capacity < need {
        let doubled = capacity.wrapping_add(capacity);
        let wanted = if need >= doubled { need } else { doubled };
        e.call(
            ARRAY_RESERVE,
            &args![allocator, array, wanted, element_size],
        );
    }
    let data = e.mem.u32(array);
    e.mem.set_u32(array + 4, size.wrapping_add(extra));
    data.wrapping_add(size.wrapping_mul(element_size))
}

// Translated from 00d4b390 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (the inputs are in registers: `EAX` = `kind`, `ECX` = `count`; no stack words;
/// descriptions, not confirmed): the number of indices of `count` triangles for an index layout:
/// `kind == 1` answers `count + 2` (a strip), `kind == 2` answers `2 * count + 1`, anything else
/// `3 * count` (a list).
pub fn fn_00d4b390(_e: &mut Engine, kind: u32, count: u32) -> u32 {
    match kind {
        1 => count.wrapping_add(2),
        2 => count.wrapping_add(count).wrapping_add(1),
        _ => count.wrapping_add(count.wrapping_mul(2)),
    }
}

// Translated from 00d4b3b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (thiscall, `RET 4`, the stack word is never read): writes the vtable word
/// `0x010d_052c` at `+0` and returns `this`.
pub fn fn_00d4b3b0(e: &mut Engine, this: Ptr, _unused_1: u32) -> Ptr {
    e.mem.set_u32(this.addr(), 0x010d_052c);
    this
}

// Translated from 00d4b400 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor (`__fastcall`, no stack words; no map name, class unconfirmed): writes the vtable
/// words `+0` = `0x010d_055c` and `+0x10` = `0x010d_053c`; calls `00c905b0` on each of the pointers
/// of the array at `+0x60` (size at `+0x64`, capacity word `+0x68`; each element is read again
/// after the previous call); clears the size, frees the array through slot `0x10` of the router's
/// allocator (`(capacity & 0x3fffffff) * 4` bytes, when the sign bit is clear), clears the data
/// pointer, makes the capacity word `0x80000000` and continues with `00d4bd60` on `this`.
pub fn fn_00d4b400(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    let initial_size = e.mem.i32(me + 0x64);
    e.mem.set_u32(me, 0x010d_055c);
    e.mem.set_u32(me + 0x10, 0x010d_053c);
    if initial_size > 0 {
        let mut index = 0i32;
        loop {
            let data = e.mem.u32(me + 0x60);
            let item = e.mem.u32(data.wrapping_add(index as u32 * 4));
            e.call(0x00c9_05b0, &args![item]);
            index += 1;
            if index >= e.mem.i32(me + 0x64) {
                break;
            }
        }
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.u32(me + 0x68);
    e.mem.set_u32(me + 0x64, 0);
    if (capacity as i32) >= 0 {
        let data = e.mem.u32(me + 0x60);
        let bytes = (capacity & 0x3fff_ffff).wrapping_mul(4);
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
    e.mem.set_u32(me + 0x60, 0);
    e.mem.set_u32(me + 0x68, 0x8000_0000);
    e.call(0x00d4_bd60, &args![this]);
}

// Translated from 00d4b480 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (thiscall, `RET 4`: one stack word `source`; the names of the fields are
/// descriptions, not confirmed; this looks like the code that adds a triangle sub-part to a mesh
/// shape): appends one 0x38-byte record to the array at `this + 0x34` (new record initialised:
/// byte `+0x11` = 1, words `+0x18`, `+0x20`, `+0x24`, `+0x28`, `+0x2c` = 0, `+0x30` = 1, `+0x34` =
/// `0xffffffff`) and fills it, with copies of the data `source` describes, into a new 0x50-byte
/// object (slot 4 of the router's allocator, its `u16` at `+4` = 0x50, built by `00d4bbb0`), whose
/// pointer is appended to the array at `this + 0x60`.
///
/// `source` describes: vertices (pointer `+0`, stride `+4`, count `+8`: three floats each, copied
/// through `FLD`/`FSTP` to the object's float array at `+8`, record `+0` = its data, `+4` = 12,
/// `+8` = count); triangle indices (pointer `+0xc`, 16-bit when the byte at `+0x10` is 1 else 32-bit,
/// stride `+0x14`, count `+0x1c`, the number of indices per triangle row of the copy being
/// `min(stride / 2 or 4, 3)`: the arrays at the object's `+0x14` (16-bit) or `+0x20` (32-bit) grow by
/// `00d4b390(stride / 2 or 4, count)` elements and `count` rows of three are copied; record `+0x14`
/// = the new row size in bytes, `+0x10`, `+0x18`, `+0x1c`, `+0x11` copied from `source`, `+0xc` =
/// the copy's address); materials (pointer `+0x20`, bytes when `+0x11` is 1 else 16-bit, stride
/// `+0x24`: a single value when the stride is 0, else `count` values; record `+0x20` = the
/// address, `+0x24` = the stride, or 1 / 2 for a non-zero stride and kind 1 / 2); and
/// a word array (pointer `+0x28`, stride `+0x2c`, count `+0x30`, copied to the object's `+0x38`
/// array: one word when the stride is 0, else `count`; record `+0x28` = that array's data, `+0x2c`
/// = 0 or 4, `+0x30` = 1 or the count), `+0x34` copied. Without materials (null pointer, record
/// `+0x20` stays 0) the words array is replaced by the constant vector at `01267e30` (`+0x28`,
/// `+0x20`, `+0x2c` = 0, `+0x30` = 1).
pub fn fn_00d4b480(e: &mut Engine, this: Ptr, source: u32) {
    let me = this.addr();
    // The record: placement of the new element, then the size grows.
    let records = me + 0x34;
    reserve_one_00d4b480(e, records, 0x38);
    let size = e.mem.u32(records + 4);
    let data = e.mem.u32(records);
    let new_record = data.wrapping_add(size.wrapping_mul(0x38));
    if new_record != 0 {
        e.mem.set_u8(new_record + 0x11, 1);
        e.mem.set_u32(new_record + 0x24, 0);
        e.mem.set_u32(new_record + 0x2c, 0);
        e.mem.set_u32(new_record + 0x30, 1);
        e.mem.set_u32(new_record + 0x28, 0);
        e.mem.set_u32(new_record + 0x20, 0);
        e.mem.set_u32(new_record + 0x34, 0xffff_ffff);
        e.mem.set_u32(new_record + 0x18, 0);
    }
    let size = e.mem.u32(records + 4);
    let data = e.mem.u32(records);
    let record = data.wrapping_add(size.wrapping_mul(0x38));
    e.mem.set_u32(records + 4, size.wrapping_add(1));
    // The new 0x50-byte object and its pointer in the array at +0x60.
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![0x50u32]).u32();
    e.mem.set_u16(block + 4, 0x50);
    let object = e.call(0x00d4_bbb0, &args![block]).u32();
    let pointers = me + 0x60;
    reserve_one_00d4b480(e, pointers, 4);
    let size = e.mem.u32(pointers + 4);
    let data = e.mem.u32(pointers);
    e.mem
        .set_u32(data.wrapping_add(size.wrapping_mul(4)), object);
    e.mem.set_u32(pointers + 4, size.wrapping_add(1));
    let size = e.mem.u32(pointers + 4);
    let data = e.mem.u32(pointers);
    let object = e
        .mem
        .u32(data.wrapping_add(size.wrapping_mul(4)).wrapping_sub(4));
    // The vertices: three floats each, copied through the x87 stack.
    let vertex_count = e.mem.u32(source + 8);
    let floats = object + 8;
    let extra = vertex_count.wrapping_add(vertex_count.wrapping_mul(2));
    let mut destination = grow_by_00d4b480(e, floats, extra, 4);
    let mut from = e.mem.u32(source);
    if e.mem.i32(source + 8) > 0 {
        let mut index = 0i32;
        loop {
            for lane in 0..3 {
                let bits = x87_quiet_bits(e.mem.u32(from + lane * 4));
                e.mem.set_u32(destination + lane * 4, bits);
            }
            destination = destination.wrapping_add(0xc);
            index += 1;
            from = from.wrapping_add(e.mem.u32(source + 4));
            if index >= e.mem.i32(source + 8) {
                break;
            }
        }
    }
    let float_data = e.mem.u32(floats);
    e.mem.set_u32(record, float_data);
    e.mem.set_u32(record + 4, 0xc);
    e.mem.set_u32(record + 8, vertex_count);
    // The triangle indices, 16-bit or 32-bit.
    let wide = e.mem.u8(source + 0x10) != 1;
    let stride = e.mem.u32(source + 0x14);
    let shift = if wide { 2 } else { 1 };
    let kind = stride >> shift;
    let per_row = if (kind as i32) > 2 { 3 } else { kind };
    let element = if wide { 4 } else { 2 };
    e.mem.set_u32(record + 0x14, per_row.wrapping_mul(element));
    let count = e.mem.u32(source + 0x1c);
    let index_count = fn_00d4b390(e, kind, count);
    let indices = object + if wide { 0x20 } else { 0x14 };
    let copy_start = grow_by_00d4b480(e, indices, index_count, element);
    let mut destination = copy_start;
    let mut from = e.mem.u32(source + 0xc);
    if e.mem.i32(source + 0x1c) > 0 {
        let mut index = 0i32;
        loop {
            for lane in 0..3 {
                if wide {
                    let value = e.mem.u32(from + lane * 4);
                    e.mem.set_u32(destination + lane * 4, value);
                } else {
                    let value = e.mem.u16(from + lane * 2);
                    e.mem.set_u16(destination + lane * 2, value);
                }
            }
            destination = destination.wrapping_add(e.mem.u32(record + 0x14));
            from = from.wrapping_add(e.mem.u32(source + 0x14));
            index += 1;
            if index >= e.mem.i32(source + 0x1c) {
                break;
            }
        }
    }
    let kind_byte = e.mem.u8(source + 0x10);
    e.mem.set_u8(record + 0x10, kind_byte);
    let triangles = e.mem.u32(source + 0x1c);
    e.mem.set_u32(record + 0x1c, triangles);
    let word = e.mem.u32(source + 0x18);
    e.mem.set_u32(record + 0x18, word);
    e.mem.set_u32(record + 0xc, copy_start);
    let material_kind = e.mem.u8(source + 0x11);
    e.mem.set_u8(record + 0x11, material_kind);
    // The materials: a byte or a 16-bit value per triangle (or one for all).
    let materials = e.mem.u32(source + 0x20);
    if materials == 0 {
        e.mem.set_u32(record + 0x20, 0);
        e.mem.set_u32(record + 0x24, 0);
    } else {
        let byte_sized = e.mem.u8(source + 0x11) == 1;
        let (array, element) = if byte_sized {
            (object + 0x2c, 1u32)
        } else {
            (object + 0x44, 2)
        };
        let single = e.mem.u32(source + 0x24) == 0;
        let first_material = if single {
            reserve_one_00d4b480(e, array, element);
            let size = e.mem.u32(array + 4);
            let data = e.mem.u32(array);
            let at = data.wrapping_add(size.wrapping_mul(element));
            if byte_sized {
                let value = e.mem.u8(materials);
                e.mem.set_u8(at, value);
            } else {
                let value = e.mem.u16(materials);
                e.mem.set_u16(at, value);
            }
            e.mem.set_u32(array + 4, size.wrapping_add(1));
            let size = e.mem.u32(array + 4);
            let data = e.mem.u32(array);
            data.wrapping_add(size.wrapping_mul(element))
                .wrapping_sub(element)
        } else {
            let wanted = e.mem.u32(source + 0x1c);
            let start = grow_by_00d4b480(e, array, wanted, element);
            let mut from = e.mem.u32(source + 0x20);
            if e.mem.i32(source + 0x1c) > 0 {
                let mut index = 0u32;
                loop {
                    if byte_sized {
                        let value = e.mem.u8(from);
                        e.mem.set_u8(start + index, value);
                    } else {
                        let value = e.mem.u16(from);
                        e.mem.set_u16(start + index * 2, value);
                    }
                    from = from.wrapping_add(e.mem.u32(source + 0x24));
                    index += 1;
                    if index as i32 >= e.mem.i32(source + 0x1c) {
                        break;
                    }
                }
            }
            start
        };
        e.mem.set_u32(record + 0x20, first_material);
        let stride = e.mem.u32(source + 0x24);
        e.mem.set_u32(record + 0x24, stride);
        if stride != 0 {
            match e.mem.i8(record + 0x11) {
                1 => e.mem.set_u32(record + 0x24, 1),
                2 => e.mem.set_u32(record + 0x24, 2),
                _ => {}
            }
        }
    }
    // The word array, or the constant vector when there are no materials.
    if e.mem.u32(record + 0x20) == 0 {
        e.mem.set_u32(record + 0x30, 1);
        e.mem.set_u32(record + 0x28, 0x0126_7e30);
        e.mem.set_u32(record + 0x2c, 0);
        e.mem.set_u32(record + 0x20, 0x0126_7e30);
        let last = e.mem.u32(source + 0x34);
        e.mem.set_u32(record + 0x34, last);
        return;
    }
    let words = object + 0x38;
    if e.mem.u32(source + 0x2c) == 0 {
        let word_pointer = e.mem.u32(source + 0x28);
        reserve_one_00d4b480(e, words, 4);
        let size = e.mem.u32(words + 4);
        let data = e.mem.u32(words);
        let value = e.mem.u32(word_pointer);
        e.mem
            .set_u32(data.wrapping_add(size.wrapping_mul(4)), value);
        e.mem.set_u32(words + 4, size.wrapping_add(1));
        e.mem.set_u32(record + 0x2c, 0);
        e.mem.set_u32(record + 0x30, 1);
    } else {
        let wanted = e.mem.u32(source + 0x30);
        let start = grow_by_00d4b480(e, words, wanted, 4);
        let mut from = e.mem.u32(source + 0x28);
        if e.mem.i32(source + 0x30) > 0 {
            let mut index = 0u32;
            loop {
                let value = e.mem.u32(from);
                e.mem.set_u32(start + index * 4, value);
                from = from.wrapping_add(e.mem.u32(source + 0x2c));
                index += 1;
                if index as i32 >= e.mem.i32(source + 0x30) {
                    break;
                }
            }
        }
        e.mem.set_u32(record + 0x2c, 4);
        let count = e.mem.u32(source + 0x30);
        e.mem.set_u32(record + 0x30, count);
    }
    let data = e.mem.u32(words);
    e.mem.set_u32(record + 0x28, data);
    let last = e.mem.u32(source + 0x34);
    e.mem.set_u32(record + 0x34, last);
}

// Translated from 00d4ba30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor (thiscall, `RET 4`: one stack word `flag`; no map name, class unconfirmed): runs
/// the base constructor `00dcd380` on `this` with `flag`, writes the vtable words `+0` =
/// `0x010d_055c` and `+0x10` = `0x010d_053c`, and when `flag` is not 0 fills the 0x38-byte records
/// at `+0x34` (count `+0x38`) from the objects pointed to by the array at `+0x60`: record `+0` =
/// `item + 8`, `+0xc` = `item + 0x14` when the record's byte at `+0x10` is 1 else `item + 0x20`,
/// `+0x20` = `item + 0x2c` when the byte at `+0x11` is 1 else `item + 0x44`, `+0x28` = `item + 0x38`
/// (words, not the data they point to). Returns `this`.
pub fn fn_00d4ba30(e: &mut Engine, this: Ptr, flag: u32) -> Ptr {
    let me = this.addr();
    e.call(0x00dc_d380, &args![this, flag]);
    e.mem.set_u32(me, 0x010d_055c);
    e.mem.set_u32(me + 0x10, 0x010d_053c);
    if flag != 0 && e.mem.i32(me + 0x38) > 0 {
        let mut index = 0i32;
        let mut offset = 0u32;
        loop {
            let items = e.mem.u32(me + 0x60);
            let record = e.mem.u32(me + 0x34).wrapping_add(offset);
            let item = e.mem.u32(items.wrapping_add(index as u32 * 4));
            let first = e.mem.u32(item + 8);
            e.mem.set_u32(record, first);
            let second = if e.mem.u8(record + 0x10) == 1 {
                e.mem.u32(item + 0x14)
            } else {
                e.mem.u32(item + 0x20)
            };
            e.mem.set_u32(record + 0xc, second);
            let third = if e.mem.u8(record + 0x11) == 1 {
                e.mem.u32(item + 0x2c)
            } else {
                e.mem.u32(item + 0x44)
            };
            e.mem.set_u32(record + 0x20, third);
            let fourth = e.mem.u32(item + 0x38);
            e.mem.set_u32(record + 0x28, fourth);
            offset = offset.wrapping_add(0x38);
            index += 1;
            if index >= e.mem.i32(me + 0x38) {
                break;
            }
        }
    }
    this
}

// Translated from 00d4bab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copy constructor (thiscall, `RET 4`: one stack word `source`; no map name, class unconfirmed):
/// runs the base constructor `00dcd310` on `this` with the float at `source + 0x50` (copied
/// through `FLD`/`FSTP`) and the word at `source + 0x30`; writes the vtable words `+0` =
/// `0x010d_055c`, `+0x10` = `0x010d_053c`; an empty pointer array at `+0x60` (size `+0x64`,
/// capacity word `+0x68` = `0x80000000`); copies the vector at `+0x20`, the word at `+8` and the
/// byte at `+0x14`; adds each of the records of `source` (0x38 bytes from `source + 0x34`, count
/// `+0x38`) with `00d4b480`; then makes the 16-bit array at `+0x40` the size of the one at
/// `source + 0x44` (when its capacity is below that size: frees the old data (when the sign bit is
/// clear, `2 * capacity` bytes) and allocates `2 * size` bytes through slot `0xc` of the router's
/// allocator (the byte count passed by address, the capacity becomes half of what is answered
/// back)), and copies those 16-bit values; and the byte at `+0x4c`. Returns `this`.
pub fn fn_00d4bab0(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    let me = this.addr();
    let src = source.addr();
    let word = e.mem.u32(src + 0x30);
    let float_bits = x87_quiet_bits(e.mem.u32(src + 0x50));
    e.call(0x00dc_d310, &args![this, float_bits, word]);
    e.mem.set_u32(me, 0x010d_055c);
    e.mem.set_u32(me + 0x10, 0x010d_053c);
    e.mem.set_u32(me + 0x68, 0x8000_0000);
    e.mem.set_u32(me + 0x60, 0);
    e.mem.set_u32(me + 0x64, 0);
    for lane in 0..4 {
        let bits = e.mem.u32(src + 0x20 + lane * 4);
        e.mem.set_u32(me + 0x20 + lane * 4, bits);
    }
    let value = e.mem.u32(src + 8);
    e.mem.set_u32(me + 8, value);
    let flag = e.mem.u8(src + 0x14);
    e.mem.set_u8(me + 0x14, flag);
    if e.mem.i32(src + 0x38) > 0 {
        let mut offset = 0u32;
        let mut index = 0i32;
        loop {
            let record = e.mem.u32(src + 0x34).wrapping_add(offset);
            e.call(0x00d4_b480, &args![this, record]);
            offset = offset.wrapping_add(0x38);
            index += 1;
            if index >= e.mem.i32(src + 0x38) {
                break;
            }
        }
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity_word = e.mem.u32(me + 0x48);
    let capacity = (capacity_word & 0x3fff_ffff) as i32;
    if capacity < e.mem.i32(src + 0x44) {
        if (capacity_word as i32) >= 0 {
            let old = e.mem.u32(me + 0x40);
            e.vcall(
                allocator,
                0x10,
                &args![old, (capacity as u32).wrapping_add(capacity as u32)],
            );
        }
        let bytes = e.mem.u32(src + 0x44).wrapping_add(e.mem.u32(src + 0x44));
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), bytes);
            let block = e.vcall(allocator, 0xc, &args![slot]).u32();
            e.mem.set_u32(me + 0x40, block);
            let granted = e.mem.i32(slot.addr());
            e.mem
                .set_i32(me + 0x48, granted.wrapping_sub(granted >> 31) >> 1);
        });
    }
    let count = e.mem.u32(src + 0x44);
    e.mem.set_u32(me + 0x44, count);
    let destination = e.mem.u32(me + 0x40);
    let from = e.mem.u32(src + 0x40);
    if (count as i32) > 0 {
        for index in 0..count {
            let value = e.mem.u16(from.wrapping_add(index * 2));
            e.mem.set_u16(destination.wrapping_add(index * 2), value);
        }
    }
    let last = e.mem.u8(src + 0x4c);
    e.mem.set_u8(me + 0x4c, last);
    this
}

// Translated from 00d4bbb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor (thiscall, no stack words; no map name, class unconfirmed): vtable word `+0` =
/// `0x010d_052c`, the `u16` at `+6` = 1, and six empty `hkArray`s: the capacity word `0x80000000` at
/// `+0x10` alone, then data/size/capacity triples at `+0x14`, `+0x20`, `+0x2c`, `+0x38` and
/// `+0x44` (data 0, size 0, capacity `0x80000000`), and the word at `+8` and `+0xc` = 0. Returns
/// `this`.
pub fn fn_00d4bbb0(e: &mut Engine, this: Ptr) -> Ptr {
    let me = this.addr();
    e.mem.set_u32(me, 0x010d_052c);
    e.mem.set_u16(me + 6, 1);
    e.mem.set_u32(me + 0x10, 0x8000_0000);
    e.mem.set_u32(me + 8, 0);
    e.mem.set_u32(me + 0xc, 0);
    for base in [0x14u32, 0x20, 0x2c, 0x38, 0x44] {
        e.mem.set_u32(me + base, 0);
        e.mem.set_u32(me + base + 4, 0);
        e.mem.set_u32(me + base + 8, 0x8000_0000);
    }
    this
}

// Translated from 00d4b150 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: data 011b5178). A bare `RET`: it reads no stack
/// word and leaves no result.
pub fn fn_00d4b150(_e: &mut Engine) {}

// Translated from 00d4b170 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: imm 00fbee3f). Cdecl `(object, flag)`: does nothing
/// for a null `object`; otherwise calls `00d4a170` (thiscall on `object` with `flag` as its one stack word).
pub fn fn_00d4b170(e: &mut Engine, object: Ptr, flag: u32) {
    if !object.is_null() {
        e.call(0x00d4_a170, &args![object, flag]);
    }
}

// Translated from 00d4b190 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: imm 00fbee49). Cdecl `(object)`: calls the first
/// virtual slot of `object` (`[[object]]`, the destructor entry in the Xbox PDB's class listings)
/// with the one stack word 0 and answers that call's result.
pub fn fn_00d4b190(e: &mut Engine, object: Ptr) -> u32 {
    e.vcall(object.addr(), 0, &args![0u32]).u32()
}

// Translated from 00d4b1d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: imm 00fbeeaf). Cdecl `(object, flag)`: does nothing
/// for a null `object`; otherwise calls `00d49c40` (thiscall on `object` with `flag` as its one stack word).
pub fn fn_00d4b1d0(e: &mut Engine, object: Ptr, flag: u32) {
    if !object.is_null() {
        e.call(0x00d4_9c40, &args![object, flag]);
    }
}

// Translated from 00d4b1f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: imm 00fbeeb9). Cdecl `(object)`: calls the first
/// virtual slot of `object` (`[[object]]`, the destructor entry in the Xbox PDB's class listings)
/// with the one stack word 0 and answers that call's result.
pub fn fn_00d4b1f0(e: &mut Engine, object: Ptr) -> u32 {
    e.vcall(object.addr(), 0, &args![0u32]).u32()
}

// Translated from 00d4b230 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: imm 00fbef2f). Cdecl `(object, flag)`: does nothing
/// for a null `object`; otherwise calls `00d4a180` (thiscall on `object` with `flag` as its one stack word).
pub fn fn_00d4b230(e: &mut Engine, object: Ptr, flag: u32) {
    if !object.is_null() {
        e.call(0x00d4_a180, &args![object, flag]);
    }
}

// Translated from 00d4b250 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: imm 00fbef39). Cdecl `(object)`: calls the first
/// virtual slot of `object` (`[[object]]`, the destructor entry in the Xbox PDB's class listings)
/// with the one stack word 0 and answers that call's result.
pub fn fn_00d4b250(e: &mut Engine, object: Ptr) -> u32 {
    e.vcall(object.addr(), 0, &args![0u32]).u32()
}

// Translated from 00d4b290 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: data 011b5210). A bare `RET`: it reads no stack
/// word and leaves no result.
pub fn fn_00d4b290(_e: &mut Engine) {}

// Translated from 00d4b2b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: data 011b5228). A bare `RET`: it reads no stack
/// word and leaves no result.
pub fn fn_00d4b2b0(_e: &mut Engine) {}

// Translated from 00d4b2d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: data 011b523c). Cdecl `(object, flag)`: does nothing
/// for a null `object`; otherwise calls `00d56470` (thiscall on `object` with `flag` as its one stack word).
pub fn fn_00d4b2d0(e: &mut Engine, object: Ptr, flag: u32) {
    if !object.is_null() {
        e.call(0x00d5_6470, &args![object, flag]);
    }
}

// Translated from 00d4b300 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: imm 00fbf08f). Cdecl `(object, flag)`: does nothing
/// for a null `object`; otherwise calls `00d56f10` (thiscall on `object` with `flag` as its one stack word).
pub fn fn_00d4b300(e: &mut Engine, object: Ptr, flag: u32) {
    if !object.is_null() {
        e.call(0x00d5_6f10, &args![object, flag]);
    }
}

// Translated from 00d4b320 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: imm 00fbf099). Cdecl `(object)`: calls the first
/// virtual slot of `object` (`[[object]]`, the destructor entry in the Xbox PDB's class listings)
/// with the one stack word 0 and answers that call's result.
pub fn fn_00d4b320(e: &mut Engine, object: Ptr) -> u32 {
    e.vcall(object.addr(), 0, &args![0u32]).u32()
}

// Translated from 00d4b360 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: data 011b520c). A bare `RET`: it reads no stack
/// word and leaves no result.
pub fn fn_00d4b360(_e: &mut Engine) {}

// Translated from 00d4b370 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: data 011b5224). A bare `RET`: it reads no stack
/// word and leaves no result.
pub fn fn_00d4b370(_e: &mut Engine) {}

// Translated from 00d4b380 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: data 011b5240). Cdecl
/// `(object)`: loads `ECX` from its one stack word and tail-jumps to `00d56800` (thiscall, no
/// stack words), so it answers that function's result.
pub fn fn_00d4b380(e: &mut Engine, object: Ptr) -> u32 {
    e.call(0x00d5_6800, &args![object]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d4_b1a0, fn_00d4b1a0() -> u32),
        entry!(0x00d4_b200, fn_00d4b200() -> u32),
        entry!(0x00d4_b260, fn_00d4b260() -> u32),
        entry!(0x00d4_b330, fn_00d4b330() -> u32),
        entry!(0x00d4_b390, fn_00d4b390(u32, u32) -> u32),
        entry!(0x00d4_b3b0, fn_00d4b3b0(Ptr, u32) -> Ptr),
        entry!(0x00d4_b400, fn_00d4b400(Ptr)),
        entry!(0x00d4_b480, fn_00d4b480(Ptr, u32)),
        entry!(0x00d4_ba30, fn_00d4ba30(Ptr, u32) -> Ptr),
        entry!(0x00d4_bab0, fn_00d4bab0(Ptr, Ptr) -> Ptr),
        entry!(0x00d4_bbb0, fn_00d4bbb0(Ptr) -> Ptr),
        entry!(0x00d4_b150, fn_00d4b150()),
        entry!(0x00d4_b170, fn_00d4b170(Ptr, u32)),
        entry!(0x00d4_b190, fn_00d4b190(Ptr) -> u32),
        entry!(0x00d4_b1d0, fn_00d4b1d0(Ptr, u32)),
        entry!(0x00d4_b1f0, fn_00d4b1f0(Ptr) -> u32),
        entry!(0x00d4_b230, fn_00d4b230(Ptr, u32)),
        entry!(0x00d4_b250, fn_00d4b250(Ptr) -> u32),
        entry!(0x00d4_b290, fn_00d4b290()),
        entry!(0x00d4_b2b0, fn_00d4b2b0()),
        entry!(0x00d4_b2d0, fn_00d4b2d0(Ptr, u32)),
        entry!(0x00d4_b300, fn_00d4b300(Ptr, u32)),
        entry!(0x00d4_b320, fn_00d4b320(Ptr) -> u32),
        entry!(0x00d4_b360, fn_00d4b360()),
        entry!(0x00d4_b370, fn_00d4b370()),
        entry!(0x00d4_b380, fn_00d4b380(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod a07_first_word_tests {
    use crate::prelude::*;

    const BUILT_WORD: u32 = 0x0abc_def0;

    /// A double over `callee` writes `BUILT_WORD` at the buffer it is given; `function`
    /// must call it once with that buffer and a zero flag word, and return the word.
    fn check(function: u32, callee: u32) {
        let mut e = Engine::new();
        e.register(callee, |e, a| {
            e.mem.set_u32(a[0], BUILT_WORD);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        let result = e.call(function, &args![]).u32();
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == callee).collect();
        assert_eq!(calls.len(), 1);
        assert_ne!(calls[0].1[0], 0);
        assert_eq!(calls[0].1[1], 0);
        assert_eq!(result, BUILT_WORD);
    }

    macro_rules! first_word_test {
        ($name:ident, $function:expr, $callee:expr) => {
            #[test]
            fn $name() {
                check($function, $callee);
            }
        };
    }

    first_word_test!(
        fn_00d4b1a0_returns_first_word_of_built_object,
        0x00d4_b1a0,
        0x00d4_a170
    );
    first_word_test!(
        fn_00d4b200_returns_first_word_of_built_object,
        0x00d4_b200,
        0x00d4_9c40
    );
    first_word_test!(
        fn_00d4b260_returns_first_word_of_built_object,
        0x00d4_b260,
        0x00d4_a180
    );
    first_word_test!(
        fn_00d4b330_returns_first_word_of_built_object,
        0x00d4_b330,
        0x00d5_6f10
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type ConstructorLog = Rc<RefCell<Vec<Vec<u32>>>>;

    /// Double for a constructor: logs its argument words and writes `vtable` at the start of `this`.
    fn constructor_double(e: &mut Engine, address: u32, vtable: u32) -> ConstructorLog {
        let log: ConstructorLog = Rc::new(RefCell::new(Vec::new()));
        let seen = Rc::clone(&log);
        e.register_double(address, move |e, a| {
            seen.borrow_mut().push(a.to_vec());
            e.mem.set_u32(a[0], vtable);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        log
    }

    #[test]
    fn fn_00d4b1a0_constructs_a_temporary_and_returns_its_vtable_word() {
        let mut e = Engine::new();
        let log = constructor_double(&mut e, 0x00d4_a170, 0x0a0b_0c0d);
        let result = e.call(0x00d4_b1a0, &args![]).u32();
        assert_eq!(result, 0x0a0b_0c0d);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].len(), 2);
        assert_ne!(calls[0][0], 0);
        assert_eq!(calls[0][1], 0);
    }

    #[test]
    fn fn_00d4b200_constructs_a_temporary_and_returns_its_vtable_word() {
        let mut e = Engine::new();
        let log = constructor_double(&mut e, 0x00d4_9c40, 0x0a0b_0c0d);
        let result = e.call(0x00d4_b200, &args![]).u32();
        assert_eq!(result, 0x0a0b_0c0d);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].len(), 2);
        assert_ne!(calls[0][0], 0);
        assert_eq!(calls[0][1], 0);
    }

    #[test]
    fn fn_00d4b260_constructs_a_temporary_and_returns_its_vtable_word() {
        let mut e = Engine::new();
        let log = constructor_double(&mut e, 0x00d4_a180, 0x0a0b_0c0d);
        let result = e.call(0x00d4_b260, &args![]).u32();
        assert_eq!(result, 0x0a0b_0c0d);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].len(), 2);
        assert_ne!(calls[0][0], 0);
        assert_eq!(calls[0][1], 0);
    }

    #[test]
    fn fn_00d4b330_constructs_a_temporary_and_returns_its_vtable_word() {
        let mut e = Engine::new();
        let log = constructor_double(&mut e, 0x00d5_6f10, 0x0a0b_0c0d);
        let result = e.call(0x00d4_b330, &args![]).u32();
        assert_eq!(result, 0x0a0b_0c0d);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].len(), 2);
        assert_ne!(calls[0][0], 0);
        assert_eq!(calls[0][1], 0);
    }
}

#[cfg(test)]
mod havok_a04_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn fn_00d4b390_counts_the_indices_of_the_layouts() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00d4_b390, &args![1u32, 10u32]).u32(), 12);
        assert_eq!(e.call(0x00d4_b390, &args![2u32, 10u32]).u32(), 21);
        assert_eq!(e.call(0x00d4_b390, &args![3u32, 10u32]).u32(), 30);
        assert_eq!(e.call(0x00d4_b390, &args![0u32, 10u32]).u32(), 30);
    }

    #[test]
    fn fn_00d4b3b0_writes_the_vtable_word() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        assert_eq!(e.call(0x00d4_b3b0, &args![this, 7u32]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x010d_052c);
    }

    #[test]
    fn fn_00d4bbb0_makes_six_empty_arrays() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x60);
        for word in 0..0x18 {
            e.mem.set_u32(this + word * 4, 0xffff_ffff);
        }
        assert_eq!(e.call(0x00d4_bbb0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x010d_052c);
        assert_eq!(e.mem.u16(this + 4), 0xffff);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x10), 0x8000_0000);
        for base in [0x14u32, 0x20, 0x2c, 0x38, 0x44] {
            assert_eq!(e.mem.u32(this + base), 0);
            assert_eq!(e.mem.u32(this + base + 4), 0);
            assert_eq!(e.mem.u32(this + base + 8), 0x8000_0000);
        }
        // Words outside the object's fields are untouched.
        assert_eq!(e.mem.u32(this + 0x50), 0xffff_ffff);
    }

    /// A router with an allocator whose slot 4 allocates and slot 0x10 frees (recording the
    /// call), and doubles for the array reserve and grow that give an array fresh storage of the
    /// wanted capacity (copying what it holds).
    fn install(e: &mut Engine) -> Rc<RefCell<Vec<(u32, Vec<u32>)>>> {
        let calls = Rc::new(RefCell::new(Vec::new()));
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 4, 0x0900_00f1);
        e.mem.set_u32(vtable + 0xc, 0x0900_00f2);
        e.mem.set_u32(vtable + 0x10, 0x0900_00f3);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        e.register(0x0900_00f1, |e, a| Ret {
            eax: e.mem.alloc(a[1]),
            ..Ret::default()
        });
        e.register(0x0900_00f2, |e, a| {
            let wanted = e.mem.u32(a[1]);
            let block = e.mem.alloc(wanted + 0x10);
            e.mem.set_u32(a[1], wanted + 0x10);
            Ret {
                eax: block,
                ..Ret::default()
            }
        });
        let log = Rc::clone(&calls);
        e.register_double(0x0900_00f3, move |_, a| {
            log.borrow_mut().push((0x0900_00f3, a.to_vec()));
            Ret::default()
        });
        // `00c908c0 (allocator, array, wanted, element size)` and `00c90940 (allocator, array,
        // element size)`.
        let relocate = |e: &mut Engine, array: u32, capacity: u32, element: u32| {
            let size = e.mem.u32(array + 4);
            let old = e.mem.u32(array);
            let block = e.mem.alloc(capacity * element + 8);
            for byte in 0..size * element {
                let value = e.mem.u8(old + byte);
                e.mem.set_u8(block + byte, value);
            }
            e.mem.set_u32(array, block);
            e.mem.set_u32(array + 8, capacity);
        };
        let log = Rc::clone(&calls);
        e.register_double(ARRAY_RESERVE, move |e, a| {
            log.borrow_mut().push((ARRAY_RESERVE, a.to_vec()));
            relocate(e, a[1], a[2], a[3]);
            Ret::default()
        });
        let log = Rc::clone(&calls);
        e.register_double(ARRAY_GROW, move |e, a| {
            log.borrow_mut().push((ARRAY_GROW, a.to_vec()));
            let capacity = (e.mem.u32(a[1] + 8) & 0x3fff_ffff) * 2 + 4;
            relocate(e, a[1], capacity, a[2]);
            Ret::default()
        });
        calls
    }

    /// An empty mesh-like object: record array at `+0x34`, pointer array at `+0x60`.
    fn mesh(e: &mut Engine) -> u32 {
        let this = e.mem.alloc(0x100);
        e.mem.set_u32(this + 0x3c, 0x8000_0000);
        e.mem.set_u32(this + 0x68, 0x8000_0000);
        this
    }

    /// A source with 2 vertices (stride 16), 2 triangles of indices (16-bit with stride 6, or
    /// 32-bit with stride 12), byte materials (when `with_materials`) and the word array.
    fn source(
        e: &mut Engine,
        wide: bool,
        with_materials: bool,
        material_stride: u32,
        word_stride: u32,
    ) -> u32 {
        let at = e.mem.alloc(0x40);
        let vertices = e.mem.alloc(0x40);
        for (index, value) in [1.0f32, 2.0, 3.0, 9.0, 4.0, 5.0, 6.0, 9.0]
            .into_iter()
            .enumerate()
        {
            e.mem.set_f32(vertices + index as u32 * 4, value);
        }
        let indices = e.mem.alloc(0x40);
        for index in 0..6u32 {
            if wide {
                e.mem.set_u32(indices + index * 4, 100 + index);
            } else {
                e.mem.set_u16(indices + index * 2, 100 + index as u16);
            }
        }
        let materials = e.mem.alloc(0x20);
        for index in 0..4u32 {
            e.mem.set_u32(materials + index * 4, 0x30 + index);
        }
        let words = e.mem.alloc(0x20);
        for index in 0..4u32 {
            e.mem.set_u32(words + index * 4, 0x700 + index);
        }
        e.mem.set_u32(at, vertices);
        e.mem.set_u32(at + 4, 16);
        e.mem.set_u32(at + 8, 2);
        e.mem.set_u32(at + 0xc, indices);
        e.mem.set_u8(at + 0x10, if wide { 2 } else { 1 });
        e.mem.set_u8(at + 0x11, 1);
        e.mem.set_u32(at + 0x14, if wide { 12 } else { 6 });
        e.mem.set_u32(at + 0x18, 0x5151);
        e.mem.set_u32(at + 0x1c, 2);
        e.mem
            .set_u32(at + 0x20, if with_materials { materials } else { 0 });
        e.mem.set_u32(at + 0x24, material_stride);
        e.mem.set_u32(at + 0x28, words);
        e.mem.set_u32(at + 0x2c, word_stride);
        e.mem.set_u32(at + 0x30, 2);
        e.mem.set_u32(at + 0x34, 0x6262);
        at
    }

    fn words_of(e: &Engine, array: u32, element: u32) -> Vec<u32> {
        let data = e.mem.u32(array);
        (0..e.mem.u32(array + 4))
            .map(|index| match element {
                1 => u32::from(e.mem.u8(data + index)),
                2 => u32::from(e.mem.u16(data + index * 2)),
                _ => e.mem.u32(data + index * 4),
            })
            .collect()
    }

    #[test]
    fn fn_00d4b480_adds_a_sub_part_with_16_bit_indices_and_byte_materials() {
        let mut e = Engine::new();
        install(&mut e);
        let this = mesh(&mut e);
        let s = source(&mut e, false, true, 4, 4);
        e.call(0x00d4_b480, &args![this, s]);
        // One record and one object.
        assert_eq!(e.mem.u32(this + 0x38), 1);
        assert_eq!(e.mem.u32(this + 0x64), 1);
        let record = e.mem.u32(this + 0x34);
        let object = e.mem.u32(e.mem.u32(this + 0x60));
        assert_eq!(e.mem.u16(object + 4), 0x50);
        assert_eq!(e.mem.u16(object + 6), 1);
        // Vertices: 2 x 3 floats, stride 12.
        assert_eq!(e.mem.u32(record), e.mem.u32(object + 8));
        assert_eq!(e.mem.u32(record + 4), 0xc);
        assert_eq!(e.mem.u32(record + 8), 2);
        assert_eq!(e.mem.u32(object + 0xc), 6);
        let floats: Vec<f32> = (0..6)
            .map(|i| e.mem.f32(e.mem.u32(object + 8) + i * 4))
            .collect();
        assert_eq!(floats, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        // 16-bit indices: stride 6 / 2 = 3 is a list, so 3 * 2 triangles = 6 indices.
        assert_eq!(
            words_of(&e, object + 0x14, 2),
            vec![100, 101, 102, 103, 104, 105]
        );
        assert_eq!(e.mem.u32(record + 0x14), 6);
        assert_eq!(e.mem.u8(record + 0x10), 1);
        assert_eq!(e.mem.u32(record + 0x1c), 2);
        assert_eq!(e.mem.u32(record + 0x18), 0x5151);
        assert_eq!(e.mem.u32(record + 0xc), e.mem.u32(object + 0x14));
        // Byte materials, stride 4: one byte per triangle.
        assert_eq!(words_of(&e, object + 0x2c, 1), vec![0x30, 0x31]);
        assert_eq!(e.mem.u32(record + 0x20), e.mem.u32(object + 0x2c));
        assert_eq!(e.mem.u32(record + 0x24), 1);
        // Words, stride 4: two words; record data is the array's.
        assert_eq!(words_of(&e, object + 0x38, 4), vec![0x700, 0x701]);
        assert_eq!(e.mem.u32(record + 0x2c), 4);
        assert_eq!(e.mem.u32(record + 0x30), 2);
        assert_eq!(e.mem.u32(record + 0x28), e.mem.u32(object + 0x38));
        assert_eq!(e.mem.u32(record + 0x34), 0x6262);
    }

    #[test]
    fn fn_00d4b480_adds_a_sub_part_with_32_bit_indices_and_single_values() {
        let mut e = Engine::new();
        install(&mut e);
        let this = mesh(&mut e);
        // 32-bit kind, byte materials with a 0 stride (one value), a 0 word stride (one word).
        let s = source(&mut e, true, true, 0, 0);
        e.mem.set_u8(s + 0x11, 2);
        e.call(0x00d4_b480, &args![this, s]);
        let record = e.mem.u32(this + 0x34);
        let object = e.mem.u32(e.mem.u32(this + 0x60));
        // Stride 12 / 4 = 3: three indices per row, six in all.
        assert_eq!(
            words_of(&e, object + 0x20, 4),
            vec![100, 101, 102, 103, 104, 105]
        );
        assert_eq!(e.mem.u32(record + 0x14), 12);
        assert_eq!(e.mem.u8(record + 0x10), 2);
        // 16-bit materials (kind 2) and a stride of 0: a single value, stride stays 0.
        assert_eq!(words_of(&e, object + 0x44, 2), vec![0x30]);
        assert_eq!(e.mem.u32(record + 0x24), 0);
        assert_eq!(e.mem.u32(record + 0x20), e.mem.u32(object + 0x44));
        // One word, stride 0, count 1.
        assert_eq!(words_of(&e, object + 0x38, 4), vec![0x700]);
        assert_eq!(e.mem.u32(record + 0x2c), 0);
        assert_eq!(e.mem.u32(record + 0x30), 1);
    }

    #[test]
    fn fn_00d4b480_without_materials_uses_the_constant_vector() {
        let mut e = Engine::new();
        install(&mut e);
        let this = mesh(&mut e);
        let s = source(&mut e, false, false, 4, 4);
        e.call(0x00d4_b480, &args![this, s]);
        let record = e.mem.u32(this + 0x34);
        assert_eq!(e.mem.u32(record + 0x20), 0x0126_7e30);
        assert_eq!(e.mem.u32(record + 0x24), 0);
        assert_eq!(e.mem.u32(record + 0x28), 0x0126_7e30);
        assert_eq!(e.mem.u32(record + 0x2c), 0);
        assert_eq!(e.mem.u32(record + 0x30), 1);
        assert_eq!(e.mem.u32(record + 0x34), 0x6262);
    }

    #[test]
    fn fn_00d4b480_grows_the_record_and_pointer_arrays_through_00c90940() {
        let mut e = Engine::new();
        let calls = install(&mut e);
        let this = mesh(&mut e);
        let s = source(&mut e, false, true, 4, 4);
        e.call(0x00d4_b480, &args![this, s]);
        e.call(0x00d4_b480, &args![this, s]);
        assert_eq!(e.mem.u32(this + 0x38), 2);
        assert_eq!(e.mem.u32(this + 0x64), 2);
        let grows: Vec<_> = calls
            .borrow()
            .iter()
            .filter(|(address, _)| *address == ARRAY_GROW)
            .cloned()
            .collect();
        // The first call grows the empty record and pointer arrays (capacity 0 = size 0) and the
        // new object's arrays are empty with room reserved by `00c908c0`; the second needs more
        // room only when the capacity ran out (4 after the first grow).
        assert!(grows.iter().any(|(_, a)| a[2] == 0x38));
        assert!(grows.iter().any(|(_, a)| a[2] == 4));
    }

    #[test]
    fn fn_00d4ba30_points_the_records_at_the_objects_data() {
        let mut e = Engine::new();
        e.register(0x00dc_d380, |_, _| Ret::default());
        let this = e.mem.alloc(0x100);
        let records = e.mem.alloc(0x38 * 2);
        let items = e.mem.alloc(8);
        e.mem.set_u32(this + 0x34, records);
        e.mem.set_u32(this + 0x38, 2);
        e.mem.set_u32(this + 0x60, items);
        for (index, (first, second)) in [(1u8, 1u8), (0, 2)].into_iter().enumerate() {
            let item = e.mem.alloc(0x60);
            for word in 0..0x18 {
                e.mem
                    .set_u32(item + word * 4, 0x1000 * (index as u32 + 1) + word);
            }
            e.mem.set_u32(items + index as u32 * 4, item);
            let record = records + index as u32 * 0x38;
            e.mem.set_u8(record + 0x10, first);
            e.mem.set_u8(record + 0x11, second);
        }
        e.call_log = Some(vec![]);
        let result = e.call(0x00d4_ba30, &args![this, 5u32]).u32();
        assert_eq!(result, this);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(0x00dc_d380, vec![this, 5])]);
        assert_eq!(e.mem.u32(this), 0x010d_055c);
        assert_eq!(e.mem.u32(this + 0x10), 0x010d_053c);
        let item0 = e.mem.u32(items);
        let item1 = e.mem.u32(items + 4);
        // Record 0 (bytes 1, 1): item + 8, + 0x14, + 0x2c, + 0x38; record 1 (0, 2): item + 8,
        // + 0x20, + 0x44, + 0x38.
        assert_eq!(e.mem.u32(records), e.mem.u32(item0 + 8));
        assert_eq!(e.mem.u32(records + 0xc), e.mem.u32(item0 + 0x14));
        assert_eq!(e.mem.u32(records + 0x20), e.mem.u32(item0 + 0x2c));
        assert_eq!(e.mem.u32(records + 0x28), e.mem.u32(item0 + 0x38));
        let second = records + 0x38;
        assert_eq!(e.mem.u32(second), e.mem.u32(item1 + 8));
        assert_eq!(e.mem.u32(second + 0xc), e.mem.u32(item1 + 0x20));
        assert_eq!(e.mem.u32(second + 0x20), e.mem.u32(item1 + 0x44));
        assert_eq!(e.mem.u32(second + 0x28), e.mem.u32(item1 + 0x38));
    }

    #[test]
    fn fn_00d4ba30_with_a_zero_flag_only_constructs() {
        let mut e = Engine::new();
        e.register(0x00dc_d380, |_, _| Ret::default());
        let this = e.mem.alloc(0x100);
        e.mem.set_u32(this + 0x38, 3);
        e.call(0x00d4_ba30, &args![this, 0u32]);
        assert_eq!(e.mem.u32(this), 0x010d_055c);
        assert_eq!(e.mem.u32(this + 0x34), 0);
    }

    #[test]
    fn fn_00d4b400_releases_the_pointers_and_the_array_then_the_base() {
        let mut e = Engine::new();
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0x10, 0x0900_00f3);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        for address in [0x0900_00f3, 0x00c9_05b0, 0x00d4_bd60] {
            e.register(address, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x80);
        let data = e.mem.alloc(0x10);
        e.mem.set_u32(data, 0x1111);
        e.mem.set_u32(data + 4, 0x2222);
        e.mem.set_u32(this + 0x60, data);
        e.mem.set_u32(this + 0x64, 2);
        e.mem.set_u32(this + 0x68, 0x4000_0004);
        e.call_log = Some(vec![]);
        e.call(0x00d4_b400, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (0x00c9_05b0, vec![0x1111]),
                (0x00c9_05b0, vec![0x2222]),
                (MEMORY_ROUTER, vec![]),
                (0x0900_00f3, vec![allocator, data, 16]),
                (0x00d4_bd60, vec![this]),
            ]
        );
        assert_eq!(e.mem.u32(this), 0x010d_055c);
        assert_eq!(e.mem.u32(this + 0x10), 0x010d_053c);
        assert_eq!(e.mem.u32(this + 0x60), 0);
        assert_eq!(e.mem.u32(this + 0x64), 0);
        assert_eq!(e.mem.u32(this + 0x68), 0x8000_0000);
        // An empty array with the sign bit set: nothing is freed.
        e.mem.set_u32(this + 0x60, 0x7000);
        e.mem.set_u32(this + 0x64, 0);
        e.mem.set_u32(this + 0x68, 0x8000_0004);
        e.call_log = Some(vec![]);
        e.call(0x00d4_b400, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [(MEMORY_ROUTER, vec![]), (0x00d4_bd60, vec![this])]
        );
    }

    #[test]
    fn fn_00d4bab0_copies_the_object_and_its_records() {
        let mut e = Engine::new();
        e.map(0x0104_d000, 0x1000);
        // The router's allocator: slot 0xc rounds the byte count up by 6, slot 0x10 frees.
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0xc, 0x0900_00f2);
        e.mem.set_u32(vtable + 0x10, 0x0900_00f3);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        e.register(0x0900_00f2, |e, a| {
            let bytes = e.mem.u32(a[1]) + 6;
            e.mem.set_u32(a[1], bytes);
            Ret {
                eax: e.mem.alloc(bytes),
                ..Ret::default()
            }
        });
        e.register(0x0900_00f3, |_, _| Ret::default());
        let seen = Rc::new(RefCell::new(Vec::<(u32, Vec<u32>)>::new()));
        let log = Rc::clone(&seen);
        e.register_double(0x00dc_d310, move |_, a| {
            log.borrow_mut().push((0x00dc_d310, a.to_vec()));
            Ret::default()
        });
        let log = Rc::clone(&seen);
        e.register_double(0x00d4_b480, move |_, a| {
            log.borrow_mut().push((0x00d4_b480, a.to_vec()));
            Ret::default()
        });
        let source = e.mem.alloc(0x100);
        e.mem.set_u32(source + 0x30, 0x4141);
        e.mem.set_f32(source + 0x50, 2.5);
        e.mem.set_u32(source + 8, 0x8888);
        e.mem.set_u8(source + 0x14, 9);
        e.mem.set_u8(source + 0x4c, 7);
        for lane in 0..4 {
            e.mem.set_f32(source + 0x20 + lane * 4, 1.0 + lane as f32);
        }
        let records = e.mem.alloc(0x38 * 2);
        e.mem.set_u32(source + 0x34, records);
        e.mem.set_u32(source + 0x38, 2);
        let values = e.mem.alloc(0x10);
        for index in 0..3u32 {
            e.mem.set_u16(values + index * 2, 0x40 + index as u16);
        }
        e.mem.set_u32(source + 0x40, values);
        e.mem.set_u32(source + 0x44, 3);
        let this = e.mem.alloc(0x100);
        let result = e.call(0x00d4_bab0, &args![this, source]).u32();
        assert_eq!(result, this);
        let seen = seen.borrow();
        assert_eq!(seen[0], (0x00dc_d310, vec![this, 2.5f32.to_bits(), 0x4141]));
        assert_eq!(seen[1], (0x00d4_b480, vec![this, records]));
        assert_eq!(seen[2], (0x00d4_b480, vec![this, records + 0x38]));
        assert_eq!(e.mem.u32(this), 0x010d_055c);
        assert_eq!(e.mem.u32(this + 0x10), 0x010d_053c);
        assert_eq!(e.mem.u32(this + 8), 0x8888);
        assert_eq!(e.mem.u8(this + 0x14), 9);
        assert_eq!(e.mem.u8(this + 0x4c), 7);
        assert_eq!(e.mem.f32(this + 0x2c), 4.0);
        assert_eq!(e.mem.u32(this + 0x60), 0);
        assert_eq!(e.mem.u32(this + 0x64), 0);
        assert_eq!(e.mem.u32(this + 0x68), 0x8000_0000);
        // The 16-bit array: size 3, 6 bytes asked for and 12 granted (6 added), capacity 6.
        assert_eq!(e.mem.u32(this + 0x44), 3);
        assert_eq!(e.mem.u32(this + 0x48), 6);
        let data = e.mem.u32(this + 0x40);
        let copy: Vec<u16> = (0..3).map(|index| e.mem.u16(data + index * 2)).collect();
        assert_eq!(copy, vec![0x40, 0x41, 0x42]);
    }
}

#[cfg(test)]
mod a05_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type CallLog = Rc<RefCell<Vec<Vec<u32>>>>;

    /// A double at `address` that records the argument words of each call and answers `answer`.
    fn recording_double(e: &mut Engine, address: u32, answer: u32) -> CallLog {
        let log: CallLog = Rc::new(RefCell::new(Vec::new()));
        let seen = Rc::clone(&log);
        e.register_double(address, move |_, a| {
            seen.borrow_mut().push(a.to_vec());
            Ret {
                eax: answer,
                ..Ret::default()
            }
        });
        log
    }

    /// A bare `RET` function runs, answers nothing and calls nothing.
    fn check_ret(function: u32) {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        assert_eq!(e.call(function, &args![]), Ret::default());
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    macro_rules! ret_test {
        ($name:ident, $function:expr) => {
            #[test]
            fn $name() {
                check_ret($function);
            }
        };
    }

    /// `function(object, flag)` calls `callee(object, flag)` for a non-null object and nothing for
    /// a null one.
    fn check_finish(function: u32, callee: u32) {
        let mut e = Engine::new();
        let log = recording_double(&mut e, callee, 0);
        e.call(function, &args![0u32, 1u32]);
        assert!(log.borrow().is_empty());
        e.call(function, &args![0x4000u32, 1u32]);
        e.call(function, &args![0x4000u32, 0u32]);
        assert_eq!(*log.borrow(), vec![vec![0x4000, 1], vec![0x4000, 0]]);
    }

    macro_rules! finish_test {
        ($name:ident, $function:expr, $callee:expr) => {
            #[test]
            fn $name() {
                check_finish($function, $callee);
            }
        };
    }

    /// `function(object)` calls slot 0 of the object's vtable with `(object, 0)` and answers
    /// what the slot answers.
    fn check_cleanup(function: u32) {
        let mut e = Engine::new();
        let log = recording_double(&mut e, 0x7000_0000, 0x1234_5678);
        let table = e.mem.alloc(0x10);
        e.mem.set_u32(table, 0x7000_0000);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, table);
        assert_eq!(e.call(function, &args![object]).u32(), 0x1234_5678);
        assert_eq!(*log.borrow(), vec![vec![object, 0]]);
    }

    macro_rules! cleanup_test {
        ($name:ident, $function:expr) => {
            #[test]
            fn $name() {
                check_cleanup($function);
            }
        };
    }

    ret_test!(fn_00d4b150_does_nothing, 0x00d4_b150);
    ret_test!(fn_00d4b290_does_nothing, 0x00d4_b290);
    ret_test!(fn_00d4b2b0_does_nothing, 0x00d4_b2b0);
    ret_test!(fn_00d4b360_does_nothing, 0x00d4_b360);
    ret_test!(fn_00d4b370_does_nothing, 0x00d4_b370);
    finish_test!(
        fn_00d4b170_calls_00d4a170_for_a_non_null_object,
        0x00d4_b170,
        0x00d4_a170
    );
    finish_test!(
        fn_00d4b1d0_calls_00d49c40_for_a_non_null_object,
        0x00d4_b1d0,
        0x00d4_9c40
    );
    finish_test!(
        fn_00d4b230_calls_00d4a180_for_a_non_null_object,
        0x00d4_b230,
        0x00d4_a180
    );
    finish_test!(
        fn_00d4b2d0_calls_00d56470_for_a_non_null_object,
        0x00d4_b2d0,
        0x00d5_6470
    );
    finish_test!(
        fn_00d4b300_calls_00d56f10_for_a_non_null_object,
        0x00d4_b300,
        0x00d5_6f10
    );
    cleanup_test!(fn_00d4b190_calls_virtual_slot_0_with_zero, 0x00d4_b190);
    cleanup_test!(fn_00d4b1f0_calls_virtual_slot_0_with_zero, 0x00d4_b1f0);
    cleanup_test!(fn_00d4b250_calls_virtual_slot_0_with_zero, 0x00d4_b250);
    cleanup_test!(fn_00d4b320_calls_virtual_slot_0_with_zero, 0x00d4_b320);

    #[test]
    fn fn_00d4b380_hands_the_object_to_00d56800_and_answers_its_result() {
        let mut e = Engine::new();
        let log = recording_double(&mut e, 0x00d5_6800, 0x66);
        assert_eq!(e.call(0x00d4_b380, &args![0x4000u32]).u32(), 0x66);
        assert_eq!(*log.borrow(), vec![vec![0x4000]]);
    }
}

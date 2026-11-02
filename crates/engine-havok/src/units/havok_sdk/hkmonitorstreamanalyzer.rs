//! `Havok SDK/hkmonitorstreamanalyzer.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its
//! functions in FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The Xbox PDB has no type for the analyzer's classes; the offsets below are read from the
//! code. A statistics node (`hkMonitorStreamAnalyzer::Node`, 0x58 bytes) is: eight `float`
//! values at `+0`, eight 16-bit counts at `+0x20`, the `hkArray` of child node pointers at
//! `+0x30` (data, size, capacity word), a `double` at `+0x40` (a start time), the parent at
//! `+0x48`, the name (a string pointer) at `+0x4c`, a flag word at `+0x50` and one more word at
//! `+0x54`. An `hkArray` is three words: the data, the size and the capacity word (the top bit
//! `0x80000000` marks memory that is not owned, the rest `0x3fffffff` is the capacity). The
//! memory router (`00c85750`) has the heap allocator at `+0x10` and the one used for these
//! objects at `+0x14`; an allocator's slot 4 allocates `(bytes)`, slot 8 frees `(block, bytes)`
//! and slot `0x10` frees `(pointer, bytes)`.

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`00c85750`).
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkArrayUtil::_reserve` (`00c908c0`, cdecl `(allocator, array, capacity, element size)`) and
/// the function at `00c90940` (cdecl `(allocator, array, element size)`) that makes room for one
/// more element.
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
const ARRAY_RESERVE_MORE: u32 = 0x00c9_0940;
/// `hkString`: `strLen` (`00cb10d0`), `strCmp` (`00cb0fa0`), `strCasecmp` (`00cb0fe0`), `snprintf`
/// (`00cb0f60`, cdecl `(buffer, size, format, ...)`), `memSet` (`00cb1280`) and `memCpy`
/// (`00cb1260`), all cdecl.
const STR_LEN: u32 = 0x00cb_10d0;
const STR_CMP: u32 = 0x00cb_0fa0;
const STR_CASE_CMP: u32 = 0x00cb_0fe0;
const SNPRINTF: u32 = 0x00cb_0f60;
const MEM_SET: u32 = 0x00cb_1280;
const MEM_COPY: u32 = 0x00cb_1260;
/// `hkOstream::printf` (`00cc0410`, cdecl `(stream, format, ...)`; a `double` is two words), the
/// stream's write (`00cc0470`, thiscall `(buffer, size)`), `operator<<(char)` (`00cc01a0`,
/// thiscall `(character)`) and `operator<<(const char *)` (`00cc01c0`, thiscall `(string)`).
const PRINTF: u32 = 0x00cc_0410;
const STREAM_WRITE: u32 = 0x00cc_0470;
const STREAM_PUT_CHAR: u32 = 0x00cc_01a0;
const STREAM_PUT_STRING: u32 = 0x00cc_01c0;
/// The map `00cb1830` family: `get` (`00cb1980`, thiscall `(key, &out)`, answers 0 when the key
/// was found), `insert` (`00cb1870`, thiscall `(allocator, key, value)`).
const MAP_GET: u32 = 0x00cb_1980;
const MAP_INSERT: u32 = 0x00cb_1870;
/// The constants the code loads: the double `100.0`, the float `2^32` (added for a `FILD` of a
/// word with its top bit set) and the doubles at `0101a6b8` and `01012070`.
const HUNDRED: u32 = 0x0101_7a40;
const TWO_POW_32: u32 = 0x0109_9378;
const TWO_POW_32_DOUBLE: u32 = 0x0101_a6b8;
const ONE_DOUBLE: u32 = 0x0101_2070;
/// The table of pointers to character glyph bitmaps `00e0c1c0` draws from.
const GLYPH_TABLE: u32 = 0x011b_dea0;
const NOT_OWNED: u32 = 0x8000_0000;
const CAPACITY_MASK: u32 = 0x3fff_ffff;
/// Node offsets.
const COUNTS: u32 = 0x20;
const CHILDREN: u32 = 0x30;
const START_TIME: u32 = 0x40;
const NODE_NAME: u32 = 0x4c;
const NODE_FLAGS: u32 = 0x50;
const NODE_KIND: u32 = 0x54;
const NODE_SIZE: u32 = 0x58;

/// `FILD dword` followed by the exe's correction for a set top bit (`TEST`, `JGE`, `FADD` of
/// the float `2^32`): the word as an unsigned number.
fn fild_unsigned(e: &Engine, value: u32) -> f64 {
    let loaded = f64::from(value as i32);
    if (value as i32) < 0 {
        x87_add(loaded, x87_m32(e.mem.u32(TWO_POW_32)))
    } else {
        loaded
    }
}

/// `FLD qword` followed by `FSTP qword`: a signalling NaN comes out quiet.
fn quiet_f64(value: f64) -> f64 {
    let bits = value.to_bits();
    if value.is_nan() {
        f64::from_bits(bits | 1 << 51)
    } else {
        value
    }
}

/// The memory router's allocator at `router + 0x10`.
fn heap_allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

/// The memory router's allocator at `router + 0x14`.
fn object_allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x14)
}

/// The inlined "append room for one more": after the router has been asked for its allocator
/// (`allocator`, at `+0x10`), makes room with `00c90940` when the array's size equals its
/// capacity.
fn reserve_one_more(e: &mut Engine, allocator: u32, array: u32, element: u32) {
    let capacity = e.mem.u32(array + 8) & CAPACITY_MASK;
    if e.mem.u32(array + 4) == capacity {
        e.call(ARRAY_RESERVE_MORE, &args![allocator, array, element]);
    }
}

/// The inlined "make room for `wanted`": when the array's capacity (signed) is below `wanted`,
/// `_reserve` (`00c908c0`) grows it to the larger of `wanted` and twice the capacity, element
/// size `element`.
fn reserve_at_least(e: &mut Engine, allocator: u32, array: u32, wanted: u32, element: u32) {
    let capacity = e.mem.u32(array + 8) & CAPACITY_MASK;
    if (capacity as i32) < wanted as i32 {
        let doubled = capacity.wrapping_add(capacity);
        let target = if (wanted as i32) < doubled as i32 {
            doubled
        } else {
            wanted
        };
        e.call(ARRAY_RESERVE, &args![allocator, array, target, element]);
    }
}

/// The inlined "free this array's memory": the size word is cleared, the data is freed through
/// slot `0x10` of `allocator` with `capacity * element` bytes unless the capacity word marks it
/// as not owned, the data pointer is cleared and the capacity word set to `0x80000000`.
fn release_array(e: &mut Engine, allocator: u32, array: u32, element: u32) {
    let capacity = e.mem.u32(array + 8);
    e.mem.set_u32(array + 4, 0);
    if capacity & NOT_OWNED == 0 {
        let data = e.mem.u32(array);
        let bytes = (capacity & CAPACITY_MASK).wrapping_mul(element);
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 8, NOT_OWNED);
}

/// The size of a node's children array and one child by index.
fn child_count(e: &Engine, node: u32) -> i32 {
    e.mem.i32(node + CHILDREN + 4)
}

fn child_at(e: &Engine, node: u32, index: u32) -> u32 {
    let data = e.mem.u32(node + CHILDREN);
    e.mem.u32(data.wrapping_add(index.wrapping_mul(4)))
}

// Translated from 00e0bf20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMonitorStreamStringMap::StringMap::StringMap` (Xbox PDB): runs the function at `00cb0a60`
/// (engine map `hkStringPtr::hkStringPtr`, thiscall, no stack words) on `this + 8`. Returns
/// `this`.
pub fn hk_monitor_stream_string_map_string_map(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x00cb_0a60, &args![this.addr() + 8]);
    this
}

// Translated from 00e0bfe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `printSingleNodeValues` (engine map name), cdecl `(stream, total, node)`: with `count` the
/// largest of the node's eight counts, builds the text `"%s (%i) %4.1f%%"` (`name`, `count`,
/// `value 1 * 100.0 / total`, the float at +4) in a 200-byte buffer with `snprintf` when `total` is above 0.0
/// (ordered), else `"%s (%i)"`; prints it with `"%-34s%s"` (and the string at `010ea674`); then
/// for each of the eight counts that is not 0 prints the matching value with `"% 12.3f: "` (as a
/// `double`); then the string at `01012648`.
pub fn print_single_node_values(e: &mut Engine, stream: u32, total: f32, node: Ptr) {
    let node = node.addr();
    let mut count = 0u32;
    for slot in 0..8 {
        let value = u32::from(e.mem.u16(node + COUNTS + 2 * slot));
        if value > count {
            count = value;
        }
    }
    let name = e.mem.u32(node + NODE_NAME);
    e.with_stack(0x134, |e, buffer| {
        let buffer = buffer.addr();
        if x87_load32(total.to_bits()) > 0.0 {
            let percent = x87_div(
                x87_mul(x87_load32(e.mem.u32(node + 4)), e.mem.f64(HUNDRED)),
                x87_load32(total.to_bits()),
            );
            e.call(
                SNPRINTF,
                &args![buffer, 0xc8u32, 0x010e_a680u32, name, count, percent],
            );
        } else {
            e.call(
                SNPRINTF,
                &args![buffer, 0xc8u32, 0x010e_a678u32, name, count],
            );
        }
        e.call(
            PRINTF,
            &args![stream, 0x010e_a66cu32, buffer, 0x010e_a674u32],
        );
        for slot in 0..8u32 {
            if e.mem.u16(node + COUNTS + 2 * slot) != 0 {
                let value = x87_load32(e.mem.u32(node + 4 * slot));
                e.call(PRINTF, &args![stream, 0x010e_a660u32, value]);
            }
        }
        e.call(PRINTF, &args![stream, 0x0101_2648u32]);
    });
}

// Translated from 00e0c150 (decompiled, FalloutNV.exe 1.4.0.525)
/// `saveToTGA` (engine map name), cdecl `(image, stream, width, height)`: writes a 32-bit
/// uncompressed TGA through the stream's write (`00cc0470`): the 18-byte header (zeroed with
/// `memSet`, then byte 2 = 2, the 16-bit width at +12, the 16-bit height at +14, byte 16 =
/// 0x20) and `width * height * 4` bytes of `image`. Returns true.
pub fn save_to_tga(e: &mut Engine, image: u32, stream: u32, width: u32, height: u32) -> bool {
    e.with_stack(0x14, |e, header| {
        let header = header.addr();
        e.call(MEM_SET, &args![header, 0u32, 0x12u32]);
        e.mem.set_u8(header + 2, 2);
        e.mem.set_u16(header + 0xe, height as u16);
        e.mem.set_u16(header + 0xc, width as u16);
        e.mem.set_u8(header + 0x10, 0x20);
        e.call(STREAM_WRITE, &args![stream, header, 0x12u32]);
        let bytes = width.wrapping_mul(height).wrapping_mul(4);
        e.call(STREAM_WRITE, &args![stream, image, bytes]);
        true
    })
}

// Translated from 00e0c1c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: draws one 8 by 7 glyph. Takes `glyph` in `EAX` and `width` in
/// `ECX`; then the stack word `image`. The glyph's rows are the words at `table[glyph] + 0x18`,
/// going down in memory (rows 0 to 6 of the picture are the words from `+0x18` down to `+0`).
/// For each of the eight columns (the shift 0, 4, ..., 28) and each row, when the row word shifted
/// left by the column's shift has any of its top four bits set, the pixel at column `x`, row `y`
/// of the 32-bit `image` (rows `width * 4` bytes apart) is set to `0xff000000`. `table` is the
/// array of pointers at `011bdea0`.
pub fn fn_00e0c1c0(e: &mut Engine, glyph: u32, width: u32, image: u32) {
    let rows = e.mem.u32(GLYPH_TABLE.wrapping_add(glyph.wrapping_mul(4)));
    let base = rows.wrapping_add(0x18);
    let stride = width.wrapping_mul(4);
    for shift in (0..0x20u32).step_by(4) {
        let mut pixel = image.wrapping_add(shift);
        let mut row = base;
        for _ in 0..7 {
            let word = e.mem.u32(row) << shift;
            if word & 0xf000_0000 != 0 {
                e.mem.set_u32(pixel, 0xff00_0000);
            }
            row = row.wrapping_sub(4);
            pixel = pixel.wrapping_add(stride);
        }
    }
}

// Translated from 00e0c210 (decompiled, FalloutNV.exe 1.4.0.525)
/// `drawNumber` (engine map name). Takes `image` in `EAX`; then the stack words `number` and
/// `width` (cdecl). Draws four glyphs with `00e0c1c0` (the glyph is the digit `(number / d) % 10`
/// with signed division, for `d` = 1000, 100, 10, 1; `width` in `ECX`), the picture pointer
/// moving by 0x1c bytes (7 pixels) for each.
pub fn draw_number(e: &mut Engine, image: u32, number: i32, width: u32) {
    let mut divisor = 1000i32;
    let mut at = image;
    loop {
        let digit = (number / divisor) % 10;
        fn_00e0c1c0(e, digit as u32, width, at);
        divisor /= 10;
        at = at.wrapping_add(0x1c);
        if divisor < 1 {
            break;
        }
    }
}

// Translated from 00e0c320 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMonitorStreamFrameInfo::hkMonitorStreamFrameInfo` (Xbox PDB): sets the vtable
/// (`010ea690`), `+4` to 0, `+8` to 1, `+0xc` to 0, the floats at `+0x10` and `+0x14` to 1.0,
/// and `+0x18`, `+0x1c`, `+0x20` to 0. Returns `this`.
pub fn hk_monitor_stream_frame_info_hk_monitor_stream_frame_info(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, 0x010e_a690);
    e.mem.set_u32(base + 4, 0);
    e.mem.set_u32(base + 8, 1);
    e.mem.set_u32(base + 0xc, 0);
    e.mem.set_u32(base + 0x10, 1.0f32.to_bits());
    e.mem.set_u32(base + 0x14, 1.0f32.to_bits());
    e.mem.set_u32(base + 0x18, 0);
    e.mem.set_u32(base + 0x1c, 0);
    e.mem.set_u32(base + 0x20, 0);
    this
}

/// `bswap` as the exe does it, byte by byte through the stack.
fn byte_swapped(word: u32) -> u32 {
    word.swap_bytes()
}

// Translated from 00e0c350 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map, cdecl `(begin, end, map, swap)`: walks the words of a monitor
/// stream from `begin` while below `end` (unsigned), turning the type tags it holds back into
/// pointers through `map` (`hkMapBase::get`, `00cb1980`; a nonzero answer ends the walk). A word
/// below 10 is a plain command: when `swap` (a byte) is set the word is first byte-swapped, and
/// if the swapped word is below 10 it is stored back; the walk moves on one word. Any other word
/// is looked up in `map` (as it was read, not swapped); the answer replaces it. The first
/// character `c` of the string it points to selects the command's layout: `E`, `S`, `T`, `l`:
/// 3 words (with `swap`, words 1 and 2 are byte-swapped); `F`, `N`, `P`, `p`: 1 word; `L`, `O`:
/// 4 words (word 3 is also looked up in `map` and replaced when found; with `swap`, words 1 and
/// 2 are swapped); `M`: 2 words (with `swap`, word 1 is byte-swapped through the x87 as a
/// float, so a signalling NaN comes out quiet); any other character, or a character below `E`
/// or above `p`, ends the walk.
pub fn fn_00e0c350(e: &mut Engine, begin: u32, end: u32, map: u32, swap: u8) {
    let swap = swap != 0;
    let mut at = begin;
    while at < end {
        let word = e.mem.u32(at);
        if !swap {
            if word < 10 {
                at = at.wrapping_add(4);
                continue;
            }
        } else {
            let swapped = byte_swapped(word);
            if swapped < 10 {
                e.mem.set_u32(at, swapped);
                at = at.wrapping_add(4);
                continue;
            }
        }
        let found = e.with_stack(4, |e, out| {
            let status = e.call(MAP_GET, &args![map, word, out]).u32();
            (status, e.mem.u32(out.addr()))
        });
        if found.0 != 0 {
            return;
        }
        e.mem.set_u32(at, found.1);
        let tag = e.mem.u8(found.1) as i8 as i32;
        let index = tag.wrapping_sub(0x45);
        if index as u32 > 0x2b {
            return;
        }
        let class = match tag as u8 {
            b'E' | b'S' | b'T' | b'l' => 0,
            b'F' | b'N' | b'P' | b'p' => 1,
            b'L' => 2,
            b'M' => 3,
            b'O' => 4,
            _ => 5,
        };
        match class {
            0 => {
                let base = at;
                at = at.wrapping_add(0xc);
                if swap {
                    let first = e.mem.u32(base + 4);
                    e.mem.set_u32(base + 4, byte_swapped(first));
                    let second = e.mem.u32(base + 8);
                    e.mem.set_u32(base + 8, byte_swapped(second));
                }
            }
            1 => at = at.wrapping_add(4),
            2 | 4 => {
                let base = at;
                at = at.wrapping_add(0x10);
                let key = e.mem.u32(base + 0xc);
                let found = e.with_stack(4, |e, out| {
                    let status = e.call(MAP_GET, &args![map, key, out]).u32();
                    (status, e.mem.u32(out.addr()))
                });
                if found.0 == 0 {
                    e.mem.set_u32(base + 0xc, found.1);
                }
                if swap {
                    let first = e.mem.u32(base + 4);
                    e.mem.set_u32(base + 4, byte_swapped(first));
                    let second = e.mem.u32(base + 8);
                    e.mem.set_u32(base + 8, byte_swapped(second));
                }
            }
            3 => {
                let base = at;
                at = at.wrapping_add(8);
                if swap {
                    let value = x87_quiet_bits(e.mem.u32(base + 4));
                    let swapped = x87_quiet_bits(byte_swapped(value));
                    e.mem.set_u32(base + 4, swapped);
                }
            }
            _ => return,
        }
    }
}

// Translated from 00e0c5e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMonitorStreamAnalyzer::Node::setTimers` (Xbox PDB), thiscall `(command, begin, end)`: for
/// the command's word at `+4` below 8 (unsigned): adds `(end[1] - begin[1]) as unsigned *
/// float at command + 0x10` to the node's float of that index (x87 arithmetic, rounded to a
/// float) and increments the node's 16-bit count of that index; the same for the word at `+8`
/// with the difference of the words at `+8` and the float at `command + 0x14`. Then, when the
/// command's word at `+0xc` is 0, the double at `node + 0x40` becomes `begin[1] as unsigned *
/// float at command + 0x10`; when it is 1, `begin[2] as unsigned * float at command + 0x14`
/// (the unsigned correction there is the double at `0101a6b8`); any other value leaves it.
pub fn hk_monitor_stream_analyzer_node_set_timers(
    e: &mut Engine,
    this: Ptr,
    command: Ptr,
    begin: Ptr,
    end: Ptr,
) {
    let (node, command, begin, end) = (this.addr(), command.addr(), begin.addr(), end.addr());
    for (word, rate) in [(4u32, 0x10u32), (8, 0x14)] {
        let index = e.mem.u32(command + word);
        if index <= 7 {
            let difference = e.mem.u32(end + word).wrapping_sub(e.mem.u32(begin + word));
            let elapsed = fild_unsigned(e, difference);
            let scaled = x87_mul(elapsed, x87_m32(e.mem.u32(command + rate)));
            let total = x87_add(scaled, x87_m32(e.mem.u32(node + 4 * index)));
            e.mem.set_u32(node + 4 * index, x87_store32(total));
            let count = e.mem.u16(node + COUNTS + 2 * index);
            e.mem
                .set_u16(node + COUNTS + 2 * index, count.wrapping_add(1));
        }
    }
    let (source, rate) = match e.mem.u32(command + 0xc) {
        0 => (begin + 4, 0x10),
        1 => (begin + 8, 0x14),
        _ => return,
    };
    let word = e.mem.u32(source);
    let mut value = f64::from(word as i32);
    if (word as i32) < 0 {
        value = x87_add(value, e.mem.f64(TWO_POW_32_DOUBLE));
    }
    let product = x87_mul(x87_load32(e.mem.u32(command + rate)), value);
    e.mem.set_f64(node + START_TIME, product);
}

// Translated from 00e0c690 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. Takes the node in `EDI`; then the stack words `name` and `index`
/// (the address of a word). Looks for the child called `name` (`hkString::strCmp` on the child's
/// name at `+0x4c`): first at the child the word `*index` points at, when that is below the
/// number of children (signed), then in turn from child 0. When found it stores the child's
/// index in `*index` (not after the first try, which leaves it) and returns true; otherwise
/// false.
pub fn fn_00e0c690(e: &mut Engine, node: u32, name: u32, index: Ptr) -> bool {
    let hint = e.mem.i32(index.addr());
    if e.mem.i32(node + CHILDREN + 4) > hint {
        let data = e.mem.u32(node + CHILDREN);
        let child = e.mem.u32(data.wrapping_add((hint as u32).wrapping_mul(4)));
        let child_name = e.mem.u32(child + NODE_NAME);
        if e.call(STR_CMP, &args![child_name, name]).u32() == 0 {
            return true;
        }
    }
    let mut at = 0u32;
    while (at as i32) < child_count(e, node) {
        let child = child_at(e, node, at);
        let child_name = e.mem.u32(child + NODE_NAME);
        if e.call(STR_CMP, &args![child_name, name]).u32() == 0 {
            e.mem.set_u32(index.addr(), at);
            return true;
        }
        at += 1;
    }
    false
}

// Translated from 00e0c700 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map, cdecl `(node, index, scale)`: multiplies the node's float `index`
/// by `scale` (a memory operand of the x87 multiplication), sets its 16-bit count `index` to 0
/// and does the same for each child, recursively (the scale passes through the x87 each time,
/// so a signalling NaN comes out quiet).
pub fn fn_00e0c700(e: &mut Engine, node: Ptr, index: u32, scale: f32) {
    let node = node.addr();
    let product = x87_mul(
        x87_load32(e.mem.u32(node + 4 * index)),
        x87_m32(scale.to_bits()),
    );
    e.mem.set_u32(node + 4 * index, x87_store32(product));
    e.mem.set_u16(node + COUNTS + 2 * index, 0);
    let mut at = 0u32;
    while (at as i32) < child_count(e, node) {
        let child = child_at(e, node, at);
        fn_00e0c700(e, Ptr::new(child), index, x87_quiet(scale));
        at += 1;
    }
}

// Translated from 00e0c860 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMonitorStreamAnalyzer::findMaxTimerNameIndent` (Xbox PDB), cdecl `(node, depth, width,
/// only_open)` (the last is a byte): `strLen` of the node's name `+ 8 + depth * width`; when
/// `only_open` is not 0 and bit 0 of the node's flag word (`+0x50`) is clear, that is the answer.
/// Otherwise the larger (signed) of it and the answer for every child with `depth + 1`.
pub fn hk_monitor_stream_analyzer_find_max_timer_name_indent(
    e: &mut Engine,
    node: Ptr,
    depth: u32,
    width: u32,
    only_open: u8,
) -> i32 {
    let node = node.addr();
    let name = e.mem.u32(node + NODE_NAME);
    let length = e.call(STR_LEN, &args![name]).u32();
    let mut best = length
        .wrapping_add(depth.wrapping_mul(width))
        .wrapping_add(8) as i32;
    if only_open != 0 && e.mem.u32(node + NODE_FLAGS) & 1 == 0 {
        return best;
    }
    let mut at = 0u32;
    while (at as i32) < child_count(e, node) {
        let child = child_at(e, node, at);
        let answer = hk_monitor_stream_analyzer_find_max_timer_name_indent(
            e,
            Ptr::new(child),
            depth.wrapping_add(1),
            width,
            only_open,
        );
        if answer > best {
            best = answer;
        }
        at += 1;
    }
    best
}

/// The largest of a node's eight counts and the number of them that are not 0.
fn count_summary(e: &Engine, node: u32) -> (u32, u32) {
    let (mut largest, mut used) = (0u32, 0u32);
    for slot in 0..8 {
        let value = u32::from(e.mem.u16(node + COUNTS + 2 * slot));
        if value != 0 {
            used += 1;
            if value > largest {
                largest = value;
            }
        }
    }
    (largest, used)
}

// Translated from 00e0c8e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkWriteRec` (engine map name), cdecl `(stream, node, depth, scale)`: for a `depth` that is
/// not 0 it prints `depth - 1` times the string at `0109e324`, then, with `count` the largest of
/// the node's counts and `used` how many are not 0, makes the text (a 200-byte buffer, `snprintf`):
/// first `"%s(%i)"` (unused), then `"%s"` (string `01019f08`) when `count < 2`, else `"%s (%i)"`
/// when `scale` equals 1.0 (ordered), else `"%s (%4.1f)"` with `scale * count` (x87, as a
/// `double`). It prints the text with `"%-32s"`, the string `0109e324` three times and then
/// the node's values whose counts are not 0: each value times `scale` (float), times
/// `count / its count` when its count is below `count` (all rounded to floats), with `"% 12.3f: "`
/// (as `double`), the last one with `"% 12.3f\n"`; when `used` is 0 it prints `0.0` with the
/// latter format. Then, for each child, `hkWriteRec(stream, child, depth + 1, scale)`; for a
/// `depth` of 0 only the children.
pub fn hk_write_rec(e: &mut Engine, stream: u32, node: Ptr, depth: u32, scale: f32) {
    let node = node.addr();
    if depth != 0 {
        if (depth as i32) > 1 {
            for _ in 0..depth - 1 {
                e.call(PRINTF, &args![stream, 0x0109_e324u32]);
            }
        }
        let (count, used) = count_summary(e, node);
        let name = e.mem.u32(node + NODE_NAME);
        e.with_stack(0x134, |e, buffer| {
            let buffer = buffer.addr();
            e.call(
                SNPRINTF,
                &args![buffer, 0xc8u32, 0x0108_03c0u32, name, count],
            );
            if (count as i32) <= 1 {
                e.call(SNPRINTF, &args![buffer, 0xc8u32, 0x0101_9f08u32, name]);
            } else if x87_load32(scale.to_bits()) == 1.0 {
                e.call(
                    SNPRINTF,
                    &args![buffer, 0xc8u32, 0x010e_a678u32, name, count],
                );
            } else {
                let shown = x87_mul(x87_load32(scale.to_bits()), f64::from(count as i32));
                e.call(
                    SNPRINTF,
                    &args![buffer, 0xc8u32, 0x010e_a6b4u32, name, shown],
                );
            }
            e.call(PRINTF, &args![stream, 0x010e_a6acu32, buffer]);
            for _ in 0..3 {
                e.call(PRINTF, &args![stream, 0x0109_e324u32]);
            }
            if (used as i32) <= 0 {
                e.call(PRINTF, &args![stream, 0x010e_a6a0u32, 0.0f64]);
            } else {
                let mut remaining = used as i32;
                for slot in 0..8u32 {
                    let slot_count = u32::from(e.mem.u16(node + COUNTS + 2 * slot));
                    if slot_count == 0 {
                        continue;
                    }
                    let mut value = x87_store32(x87_mul(
                        x87_load32(e.mem.u32(node + 4 * slot)),
                        x87_m32(scale.to_bits()),
                    ));
                    if (slot_count as i32) < (count as i32) {
                        let ratio = x87_div(f64::from(count as i32), f64::from(slot_count as i32));
                        value = x87_store32(x87_mul(ratio, x87_m32(value)));
                    }
                    remaining -= 1;
                    let format = if remaining == 0 {
                        0x010e_a6a0u32
                    } else {
                        0x010e_a660u32
                    };
                    e.call(PRINTF, &args![stream, format, x87_load32(value)]);
                }
            }
        });
    }
    let mut at = 0u32;
    while (at as i32) < child_count(e, node) {
        let child = child_at(e, node, at);
        hk_write_rec(
            e,
            stream,
            Ptr::new(child),
            depth.wrapping_add(1),
            x87_quiet(scale),
        );
        at += 1;
    }
}

// Translated from 00e0cb80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. Takes the node in `EDI`; then the stack words `name` and
/// `first_if_none` (a byte). Answers the first child whose name `hkString::strCmp`s equal to
/// `name`; else, when `first_if_none` is not 0 and there are children, the first child; else 0.
pub fn fn_00e0cb80(e: &mut Engine, node: u32, name: u32, first_if_none: u8) -> u32 {
    let mut at = 0u32;
    while (at as i32) < child_count(e, node) {
        let child = child_at(e, node, at);
        let child_name = e.mem.u32(child + NODE_NAME);
        if e.call(STR_CMP, &args![name, child_name]).u32() == 0 {
            return child;
        }
        at += 1;
    }
    if first_if_none != 0 && child_count(e, node) > 0 {
        return child_at(e, node, 0);
    }
    0
}

// Translated from 00e0cbe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. Takes the node in `EDI` and a child pointer `after` in `EDX`; then
/// the stack word `name`. Finds the index of the child that is `after` (the first index with that
/// pointer, else the number of children), then answers the first child after it whose name
/// `hkString::strCmp`s equal to `name`, or 0.
pub fn fn_00e0cbe0(e: &mut Engine, node: u32, after: u32, name: u32) -> u32 {
    let count = e.mem.i32(node + CHILDREN + 4);
    let mut index = 0i32;
    if count > 0 {
        let data = e.mem.u32(node + CHILDREN);
        while e.mem.u32(data.wrapping_add((index as u32).wrapping_mul(4))) != after {
            index += 1;
            if index >= count {
                break;
            }
        }
    }
    index += 1;
    while index < count {
        let child = child_at(e, node, index as u32);
        let child_name = e.mem.u32(child + NODE_NAME);
        if e.call(STR_CMP, &args![name, child_name]).u32() == 0 {
            return child;
        }
        index += 1;
        if index >= e.mem.i32(node + CHILDREN + 4) {
            break;
        }
    }
    0
}

// Translated from 00e0cc40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map, cdecl `(fallback, start, time, index)` (`time` is a `double`).
/// When `start` is not null and the node's float `index` plus its double at `+0x40` is not below
/// `time`, the candidate is `start`, else `fallback`. Then, while the candidate is not null,
/// asks `00e10820` (the candidate, `time`, `index`) for a deeper node and takes it when it
/// answers one. Returns the last candidate.
pub fn fn_00e0cc40(e: &mut Engine, fallback: u32, start: u32, time: f64, index: u32) -> u32 {
    let mut candidate = start;
    let mut keep = false;
    if start != 0 {
        let value = x87_add(
            x87_load32(e.mem.u32(start + 4 * index)),
            e.mem.f64(start + START_TIME),
        );
        keep = value >= time;
    }
    if !keep {
        candidate = fallback;
    }
    while candidate != 0 {
        let deeper = fn_00e10820(e, candidate, quiet_f64(time), index);
        if deeper == 0 {
            break;
        }
        candidate = deeper;
    }
    candidate
}

// Translated from 00e0cc90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMonitorStreamColorTable` lookup (unnamed in the engine map), thiscall `(name)`: the table
/// is the array of 8-byte entries `(string, color)` at `+8` (size `+0xc`); returns the color of
/// the first entry whose string (with bit 0 cleared) `hkString::strCasecmp`s equal to `name`,
/// else the default color at `+0x14`.
pub fn fn_00e0cc90(e: &mut Engine, this: Ptr, name: u32) -> u32 {
    let this = this.addr();
    let mut index = 0u32;
    while (index as i32) < e.mem.i32(this + 0xc) {
        let entries = e.mem.u32(this + 8);
        let string = e.mem.u32(entries.wrapping_add(index.wrapping_mul(8))) & 0xffff_fffe;
        if e.call(STR_CASE_CMP, &args![string, name]).u32() == 0 {
            let entries = e.mem.u32(this + 8);
            return e.mem.u32(entries.wrapping_add(index.wrapping_mul(8)) + 4);
        }
        index += 1;
    }
    e.mem.u32(this + 0x14)
}

// Translated from 00e0ccf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, no stack words): clears `this + 0x10` and the word at `+4`
/// of each 12-byte element of the array at `this + 0` (size at `+4`, read again for each).
pub fn fn_00e0ccf0(e: &mut Engine, this: Ptr) {
    let this = this.addr();
    e.mem.set_u32(this + 0x10, 0);
    let mut index = 0u32;
    while (index as i32) < e.mem.i32(this + 4) {
        let data = e.mem.u32(this);
        e.mem
            .set_u32(data.wrapping_add(index.wrapping_mul(12)) + 4, 0);
        index += 1;
    }
}

// Translated from 00e0cd20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map, cdecl `(begin, end, map)`: walks the words of a monitor stream from
/// `begin` while below `end` (unsigned) like `00e0c350` but only collects the tags: a word below
/// 10 moves on one word; any other word `w` is inserted into `map` as its own value (`00cb1870`
/// with the memory router's allocator at `+0x10`, key `w`, value `w`), and the first character
/// `c` of the string it points to gives the layout: `E`, `S`, `T`, `l`: 3 words; `N`, `P`, `p`:
/// 1 word; `L`, `O`: 4 words, word 3 inserted into `map` the same way; `M`: 2 words; any other
/// character (`F` included), or one outside `E..=p`, ends the walk.
pub fn fn_00e0cd20(e: &mut Engine, begin: u32, end: u32, map: u32) {
    let mut at = begin;
    while at < end {
        let word = e.mem.u32(at);
        if word < 10 {
            at = at.wrapping_add(4);
            continue;
        }
        let allocator = heap_allocator(e);
        e.call(MAP_INSERT, &args![map, allocator, word, word]);
        let tag = e.mem.u8(e.mem.u32(at)) as i8 as i32;
        if (tag.wrapping_sub(0x45) as u32) > 0x2b {
            return;
        }
        match tag as u8 {
            b'E' | b'S' | b'T' | b'l' => at = at.wrapping_add(0xc),
            b'L' | b'O' => {
                let key = e.mem.u32(at + 0xc);
                let allocator = heap_allocator(e);
                e.call(MAP_INSERT, &args![map, allocator, key, key]);
                at = at.wrapping_add(0x10);
            }
            b'M' => at = at.wrapping_add(8),
            b'N' | b'P' | b'p' => at = at.wrapping_add(4),
            _ => return,
        }
    }
}

// Translated from 00e0ce00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map, thiscall `(out, begin, end, frame)`: `this` holds an array of
/// `hkArray`s of 0x24-byte frame records at `+0` (12 bytes each), a byte buffer at `+0xc` with
/// its used size at `+0x10` and its capacity word at `+0x14`. Takes `size = end - begin`; when
/// the capacity (low bits) minus the used size is below it (signed) it writes the byte 0 to
/// `*out` and stops. Otherwise it appends to the array `frame[+0x18]` of `this`: room for one
/// more (`00c90940`, element size 0x24, when full), the frame record built by `00e0c320` (the
/// frame-info constructor) in the new slot, then its nine words overwritten with those of
/// `frame`, `+0x1c` set to the used size and `+0x20` to the used size plus `size`; copies `size`
/// bytes from `begin` into the buffer at the used size (`memCpy`), adds `size` to the used size
/// and writes the byte 1 to `*out`.
/// Answers `out`: the exe leaves it in `EAX` at every `RET` (the hkBool/hkResult result
/// returned through a pointer, which callers read through `EAX`).
pub fn fn_00e0ce00(e: &mut Engine, this: Ptr, out: Ptr, begin: u32, end: u32, frame: Ptr) -> Ptr {
    let this = this.addr();
    let size = end.wrapping_sub(begin);
    let room = (e.mem.u32(this + 0x14) & CAPACITY_MASK).wrapping_sub(e.mem.u32(this + 0x10));
    if (room as i32) < size as i32 {
        e.mem.set_u8(out.addr(), 0);
        return out;
    }
    let allocator = heap_allocator(e);
    let frames = e
        .mem
        .u32(this)
        .wrapping_add(e.mem.u32(frame.addr() + 0x18).wrapping_mul(12));
    reserve_one_more(e, allocator, frames, 0x24);
    let slot = e
        .mem
        .u32(frames)
        .wrapping_add(e.mem.u32(frames + 4).wrapping_mul(0x24));
    if slot != 0 {
        hk_monitor_stream_frame_info_hk_monitor_stream_frame_info(e, Ptr::new(slot));
    }
    let count = e.mem.u32(frames + 4);
    let slot = e.mem.u32(frames).wrapping_add(count.wrapping_mul(0x24));
    e.mem.set_u32(frames + 4, count.wrapping_add(1));
    for word in 0..9u32 {
        let value = e.mem.u32(frame.addr() + 4 * word);
        e.mem.set_u32(slot + 4 * word, value);
    }
    let used = e.mem.u32(this + 0x10);
    e.mem.set_u32(slot + 0x1c, used);
    e.mem.set_u32(slot + 0x20, used.wrapping_add(size));
    let used = e.mem.u32(this + 0x10);
    e.mem.set_u32(this + 0x10, used.wrapping_add(size));
    let buffer = e.mem.u32(this + 0xc);
    e.call(MEM_COPY, &args![buffer.wrapping_add(used), begin, size]);
    e.mem.set_u8(out.addr(), 1);
    out
}

/// The inlined "append a word": after the router has been asked for its allocator, makes room
/// for one more (`00c90940`, element size 4) when the array is full and stores `value` after the
/// last element.
fn push_word(e: &mut Engine, allocator: u32, array: u32, value: u32) {
    reserve_one_more(e, allocator, array, 4);
    let size = e.mem.u32(array + 4);
    let data = e.mem.u32(array);
    e.mem
        .set_u32(data.wrapping_add(size.wrapping_mul(4)), value);
    e.mem.set_u32(array + 4, size.wrapping_add(1));
}

/// A new node (0x58 bytes from the allocator at `router + 0x10`, slot 4) built by `00e0d000`
/// with the given parent, name and kind; 0 when the allocator gives none.
fn allocate_node(e: &mut Engine, parent: u32, name: u32, kind: u32) -> u32 {
    let allocator = heap_allocator(e);
    let block = e.vcall(allocator, 4, &args![NODE_SIZE]).u32();
    if block == 0 {
        return 0;
    }
    fn_00e0d000(e, Ptr::new(block), Ptr::new(parent), name, kind).addr()
}

// Translated from 00e0d000 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the constructor of `hkMonitorStreamAnalyzer::Node`, thiscall
/// `(parent, name, kind)`. The children array is emptied (null, 0, capacity word `0x80000000`);
/// `+0x48` is `parent`, `+0x4c` `name`, `+0x50` 0, `+0x54` `kind`. When `parent` is not null the
/// node is appended to the parent's children (the router's allocator at `+0x10`, room made with
/// `00c90940` when full). The eight floats and counts are cleared and the double at `+0x40` is
/// 0. Returns `this`.
pub fn fn_00e0d000(e: &mut Engine, this: Ptr, parent: Ptr, name: u32, kind: u32) -> Ptr {
    let node = this.addr();
    e.mem.set_u32(node + CHILDREN, 0);
    e.mem.set_u32(node + CHILDREN + 4, 0);
    e.mem.set_u32(node + CHILDREN + 8, NOT_OWNED);
    e.mem.set_u32(node + 0x48, parent.addr());
    e.mem.set_u32(node + NODE_NAME, name);
    e.mem.set_u32(node + NODE_FLAGS, 0);
    e.mem.set_u32(node + NODE_KIND, kind);
    if parent.addr() != 0 {
        let allocator = heap_allocator(e);
        push_word(e, allocator, parent.addr() + CHILDREN, node);
    }
    for slot in 0..8u32 {
        e.mem.set_u32(node + 4 * slot, 0);
        e.mem.set_u16(node + COUNTS + 2 * slot, 0);
    }
    e.mem.set_f64(node + START_TIME, 0.0);
    this
}

// Translated from 00e0d0a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMonitorStreamAnalyzer::Node::~Node` (Xbox PDB), thiscall: runs the deleting destructor
/// (`00ddeff0`, `(flags = 1)`) on every child that is not null, sets the children's size to 0
/// and frees the children array through slot `0x10` of the memory router's allocator at `+0x14`
/// (`capacity * 4` bytes) unless its capacity word marks it as not owned, then sets the capacity
/// word to `0x80000000` and the data pointer to 0.
pub fn hk_monitor_stream_analyzer_node_destructor(e: &mut Engine, this: Ptr) {
    let node = this.addr();
    let mut at = 0u32;
    while (at as i32) < child_count(e, node) {
        let child = child_at(e, node, at);
        if child != 0 {
            e.call(0x00dd_eff0, &args![child, 1u32]);
        }
        at += 1;
    }
    e.mem.set_u32(node + CHILDREN + 4, 0);
    let allocator = object_allocator(e);
    let capacity = e.mem.u32(node + CHILDREN + 8);
    e.mem.set_u32(node + CHILDREN + 4, 0);
    if capacity & NOT_OWNED == 0 {
        let data = e.mem.u32(node + CHILDREN);
        e.vcall(
            allocator,
            0x10,
            &args![data, (capacity & CAPACITY_MASK).wrapping_mul(4)],
        );
    }
    e.mem.set_u32(node + CHILDREN + 8, NOT_OWNED);
    e.mem.set_u32(node + CHILDREN, 0);
}

// Translated from 00e0d100 (decompiled, FalloutNV.exe 1.4.0.525)
/// `createNewNode` (engine map name). Takes the parent node in `EDI`; then the stack words
/// `name`, `kind` and `search` (a byte). When `search` is not 0 and neither the parent nor the
/// name is null, answers the first child whose name is not null and `hkString::strCmp`s equal to
/// `name`. Otherwise makes a node (`00e0d000`, from the router's allocator at `+0x10`, slot 4,
/// 0x58 bytes) with the parent, the name and the kind, or answers 0 when the allocator gives none.
pub fn create_new_node(e: &mut Engine, parent: u32, name: u32, kind: u32, search: u8) -> u32 {
    if search != 0 && parent != 0 && name != 0 {
        let mut at = 0u32;
        while (at as i32) < child_count(e, parent) {
            let child = child_at(e, parent, at);
            let child_name = e.mem.u32(child + NODE_NAME);
            if child_name != 0 && e.call(STR_CMP, &args![child_name, name]).u32() == 0 {
                return child_at(e, parent, at);
            }
            at += 1;
        }
    }
    allocate_node(e, parent, name, kind)
}

// Translated from 00e0d180 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map, cdecl `(dest, source, dest_index, source_index, scale)`: merges the
/// children of `source` into `dest`, recursively. With `dest_index` limited to 6 (signed), and a
/// hint index that starts at 0: for each child of `source` in turn, `00e0c690` looks for the
/// child of `dest` with its name starting at the hint; a missing one is made (`00e0d000` with
/// `dest`, the name and the word at `+0x54`; 0 when the allocator gives none). The destination
/// child's float `dest_index` becomes `source child float[source_index] * (1 - scale) + its old
/// value` (the `1 - scale` kept as a double) and its count `dest_index` is set to the source
/// child's count `source_index`; then the same for their children. After each child the hint is
/// the found index plus one, when that index is below the number of source children minus one.
pub fn fn_00e0d180(
    e: &mut Engine,
    dest: u32,
    source: u32,
    dest_index: i32,
    source_index: u32,
    scale: f32,
) {
    let dest_index = dest_index.min(6) as u32;
    if child_count(e, source) <= 0 {
        return;
    }
    let one_minus = x87_sub(1.0, x87_load32(scale.to_bits()));
    e.with_stack(4, |e, hint| {
        e.mem.set_u32(hint.addr(), 0);
        let mut at = 0u32;
        loop {
            let child = child_at(e, source, at);
            let name = e.mem.u32(child + NODE_NAME);
            let found = fn_00e0c690(e, dest, name, hint);
            let index = e.mem.u32(hint.addr());
            let target = if found {
                child_at(e, dest, index)
            } else {
                let kind = e.mem.u32(child + NODE_KIND);
                let name = e.mem.u32(child + NODE_NAME);
                allocate_node(e, dest, name, kind)
            };
            let value = x87_mul(x87_load32(e.mem.u32(child + 4 * source_index)), one_minus);
            let sum = x87_add(value, x87_m32(e.mem.u32(target + 4 * dest_index)));
            let count = e.mem.u16(child + COUNTS + 2 * source_index);
            e.mem.set_u32(target + 4 * dest_index, x87_store32(sum));
            e.mem.set_u16(target + COUNTS + 2 * dest_index, count);
            fn_00e0d180(
                e,
                target,
                child,
                dest_index as i32,
                source_index,
                x87_quiet(scale),
            );
            let children = child_count(e, source);
            if (index as i32) < children - 1 {
                e.mem.set_u32(hint.addr(), index.wrapping_add(1));
            }
            at += 1;
            if (at as i32) >= children {
                break;
            }
        }
    });
}

// Translated from 00e0d280 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map, cdecl `(a, b, dest_index, source_index, scale)`: `00e0c700(a,
/// dest_index, scale)` (scales one value of a tree), then `00e0d180(a, b, dest_index,
/// source_index, scale)` (merges `b`'s children into it); the scale goes through the x87 each time.
pub fn fn_00e0d280(e: &mut Engine, a: Ptr, b: u32, dest_index: i32, source_index: u32, scale: f32) {
    fn_00e0c700(e, a, dest_index as u32, x87_quiet(scale));
    fn_00e0d180(e, a.addr(), b, dest_index, source_index, x87_quiet(scale));
}

// Translated from 00e0d880 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map, cdecl `(dest, source)`: adds the children of `source` into `dest`,
/// recursively. For each child of `source` in turn: the child of `dest` with the same name
/// (`00e0cb80`, without the first-child fallback); a missing one is made (`00e0d000` from the
/// allocator at `+0x10` with `dest`, the name and the word at `+0x54`; 0 when it gives none).
/// Every float and count of the source child is added to the destination child's (floats through
/// the x87, counts as 16-bit words), then the same for the two children's children.
pub fn fn_00e0d880(e: &mut Engine, dest: u32, source: u32) {
    let mut at = 0u32;
    while (at as i32) < child_count(e, source) {
        let child = child_at(e, source, at);
        let name = e.mem.u32(child + NODE_NAME);
        let mut target = fn_00e0cb80(e, dest, name, 0);
        if target == 0 {
            let kind = e.mem.u32(child + NODE_KIND);
            let name = e.mem.u32(child + NODE_NAME);
            target = allocate_node(e, dest, name, kind);
        }
        for slot in 0..8u32 {
            let mine = e.mem.u32(target + 4 * slot);
            let theirs = e.mem.u32(child + 4 * slot);
            let sum = if slot == 0 {
                x87_add(x87_load32(mine), x87_m32(theirs))
            } else {
                x87_add(x87_load32(theirs), x87_m32(mine))
            };
            e.mem.set_u32(target + 4 * slot, x87_store32(sum));
            let count = e.mem.u16(target + COUNTS + 2 * slot);
            let added = e.mem.u16(child + COUNTS + 2 * slot);
            e.mem
                .set_u16(target + COUNTS + 2 * slot, count.wrapping_add(added));
        }
        fn_00e0d880(e, target, child);
        at += 1;
    }
}

// Translated from 00e0d980 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMonitorStreamColorTable::addColor` (Xbox PDB), thiscall `(name, color)`: builds a local
/// `hkStringPtr` from `name` (`00cb0b80`), then appends an 8-byte entry to the array at
/// `this + 8` (the router's allocator at `+0x10`, room made with `00c90940` when full, element
/// size 8): its string is built from the local one (`00cb0bd0`, copy) and its word at `+4` is
/// `color`; the size grows by one and the local string is destroyed (`00cb0c00`).
pub fn hk_monitor_stream_color_table_add_color(e: &mut Engine, this: Ptr, name: u32, color: u32) {
    e.with_stack(8, |e, locals| {
        let string = locals.addr();
        e.call(0x00cb_0b80, &args![string, name]);
        e.mem.set_u32(string + 4, color);
        let array = this.addr() + 8;
        let allocator = heap_allocator(e);
        reserve_one_more(e, allocator, array, 8);
        let size = e.mem.u32(array + 4);
        let slot = e.mem.u32(array).wrapping_add(size.wrapping_mul(8));
        if slot != 0 {
            e.call(0x00cb_0bd0, &args![slot, string]);
            let color = e.mem.u32(string + 4);
            e.mem.set_u32(slot + 4, color);
        }
        e.mem.set_u32(array + 4, size.wrapping_add(1));
        e.call(0x00cb_0c00, &args![string]);
    });
}

// Translated from 00e10820 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map, cdecl `(node, time, index)` (`time` is a `double`): finds the child
/// of `node` that is running at `time`. The children (their count at `+0x34`) are bisected while
/// more than 2 are left: the middle child is looked at; if its word at `+0x54` is 2 the bisection
/// stops; else its double at `+0x40` is compared with `time` (below: the upper end moves down,
/// otherwise, a NaN included, the lower end moves up). Then, from the lower end, each child whose
/// word at `+0x54` is not 2 is tested: when its float `index` plus its double at `+0x40` is not
/// below `time` it is the answer if `time` is not below its double, else the answer is 0. 0 when
/// none. The path that answers 0 for `time` below the child's start pops the x87 stack once
/// more than it holds (underflow with the exceptions masked), which is not observable here.
pub fn fn_00e10820(e: &mut Engine, node: u32, time: f64, index: u32) -> u32 {
    let time = quiet_f64(time);
    let mut high = e.mem.i32(node + CHILDREN + 4);
    let mut low = 0i32;
    if high > 2 {
        loop {
            let data = e.mem.u32(node + CHILDREN);
            let middle = high.wrapping_add(low) >> 1;
            let child = e
                .mem
                .u32(data.wrapping_add((middle as u32).wrapping_mul(4)));
            if e.mem.u32(child + NODE_KIND) == 2 {
                break;
            }
            if time < e.mem.f64(child + START_TIME) {
                high = middle;
            } else {
                low = middle;
            }
            if high.wrapping_sub(low) <= 2 {
                break;
            }
        }
    }
    if low >= high {
        return 0;
    }
    let data = e.mem.u32(node + CHILDREN);
    let mut at = low;
    loop {
        let child = e.mem.u32(data.wrapping_add((at as u32).wrapping_mul(4)));
        if e.mem.u32(child + NODE_KIND) != 2 {
            let end = x87_add(
                x87_load32(e.mem.u32(child + 4 * index)),
                e.mem.f64(child + START_TIME),
            );
            if end >= time {
                if time >= e.mem.f64(child + START_TIME) {
                    return child;
                }
                return 0;
            }
        }
        at += 1;
        if at >= high {
            break;
        }
    }
    0
}

// Translated from 00e108b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `(anonymous namespace) Indenter::Indenter` (Xbox PDB), thiscall: an `hkArray` of bytes
/// (data 0, size 0, capacity word `0x80000000`) that gets room for at least 0x40 bytes from
/// the memory router's allocator at `+0x14` (`_reserve`, the larger of 0x40 and twice the
/// capacity (signed), element size 1) and a terminating zero byte (room made with `00c90940` when
/// full). Returns `this`.
pub fn indenter_indenter(e: &mut Engine, this: Ptr) -> Ptr {
    let array = this.addr();
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 4, 0);
    e.mem.set_u32(array + 8, NOT_OWNED);
    let allocator = object_allocator(e);
    let capacity = e.mem.u32(array + 8) & CAPACITY_MASK;
    if (capacity as i32) < 0x40 {
        let doubled = capacity.wrapping_add(capacity);
        let target = if (doubled as i32) > 0x40 {
            doubled
        } else {
            0x40
        };
        e.call(ARRAY_RESERVE, &args![allocator, array, target, 1u32]);
    }
    let allocator = object_allocator(e);
    reserve_one_more(e, allocator, array, 1);
    let at = e.mem.u32(array + 4).wrapping_add(e.mem.u32(array));
    e.mem.set_u8(at, 0);
    let size = e.mem.u32(array + 4);
    e.mem.set_u32(array + 4, size.wrapping_add(1));
    this
}

/// Runs the node destructor on each of the `count` 0x58-byte nodes at `data`, then frees the
/// array through slot `0x10` of the allocator (`capacity * 0x58` bytes) unless its capacity word
/// marks it as not owned: the size word is cleared first, the data pointer and capacity word
/// last.
fn destroy_node_array(e: &mut Engine, allocator: u32, array: u32) {
    let count = e.mem.i32(array + 4);
    let mut at = e.mem.u32(array);
    for _ in 0..count.max(0) {
        hk_monitor_stream_analyzer_node_destructor(e, Ptr::new(at));
        at = at.wrapping_add(NODE_SIZE);
    }
    release_array(e, allocator, array, NODE_SIZE);
}

// Translated from 00e10930 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMonitorStreamAnalyzerPrintByTypeInfo::~hkMonitorStreamAnalyzerPrintByTypeInfo` (Xbox PDB),
/// thiscall: for each of the two arrays of 0x58-byte nodes (data/size/capacity at `+0x64`, then
/// at `+0x58`; the router's allocator at `+0x14` is asked for before each) runs the node
/// destructor (`00e0d0a0`) on every element, clears the size, frees the memory (slot `0x10`,
/// `capacity * 0x58` bytes) unless the capacity word marks it as not owned, and resets the data
/// pointer and the capacity word (`0x80000000`). It ends by running the node destructor on
/// `this` itself.
pub fn hk_monitor_stream_analyzer_print_by_type_info_destructor(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let allocator = object_allocator(e);
    destroy_node_array(e, allocator, base + 0x64);
    let allocator = object_allocator(e);
    destroy_node_array(e, allocator, base + 0x58);
    hk_monitor_stream_analyzer_node_destructor(e, this);
}

// Translated from 00e0cec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `outputStatsForFrame` (engine map name). Takes `count` in `EAX`, `root` in `EDI` and `array`
/// in `ESI`; then the stack words `time` and `step` (floats) and `index`. With `first` and `last`
/// the first and last child of `root`: `limit = first.double - time` (the x87 `FSUBR`); while
/// `count` is not 0 and `cursor < limit` (ordered; `cursor` starts at 0.0) the cursor grows by
/// `step` and a 0 is appended to `array` (the allocator at `router + 0x10`, `00c90940` when
/// full), `count` decreasing. Then `end = last.float[index] + last.double` and `limit2 = end - time`;
/// the double at `root + 0x40` becomes `first.double` and the float `root.float[index]` becomes
/// `((time + limit2) - first.double) + 1.0` (the double at `01012070`). Then while `count` is not
/// 0 and `limit2 > cursor` (ordered) it asks `00e0cc40` (fallback `root`, the previous answer
/// (0 at first), `cursor + time` as a double, `index`) and appends a nonzero answer to `array`;
/// `count` decreases and the cursor grows by `step`.
pub fn output_stats_for_frame(
    e: &mut Engine,
    count: u32,
    root: u32,
    array: u32,
    time: f32,
    step: f32,
    index: u32,
) {
    let first = child_at(e, root, 0);
    let last_at = (e.mem.i32(root + CHILDREN + 4).wrapping_sub(1)) as u32;
    let last = child_at(e, root, last_at);
    let time_loaded = x87_load32(time.to_bits());
    let limit = x87_sub(e.mem.f64(first + START_TIME), time_loaded);
    let mut count = count as i32;
    let mut cursor = 0.0f64;
    if count != 0 {
        while cursor < limit {
            cursor = x87_add(cursor, x87_m32(step.to_bits()));
            let allocator = heap_allocator(e);
            push_word(e, allocator, array, 0);
            count -= 1;
            if count == 0 {
                break;
            }
        }
    }
    let end = x87_add(
        x87_load32(e.mem.u32(last + 4 * index)),
        e.mem.f64(last + START_TIME),
    );
    let time_loaded = x87_load32(time.to_bits());
    let limit_end = x87_sub(end, time_loaded);
    let first_start = e.mem.f64(first + START_TIME);
    e.mem.set_f64(root + START_TIME, first_start);
    let shifted = x87_sub(x87_add(time_loaded, limit_end), first_start);
    let stored = x87_store32(x87_add(shifted, e.mem.f64(ONE_DOUBLE)));
    e.mem.set_u32(root + 4 * index, stored);
    if count == 0 {
        return;
    }
    let mut previous = 0u32;
    while limit_end > cursor {
        let query = x87_add(cursor, x87_m32(time.to_bits()));
        let node = fn_00e0cc40(e, root, previous, query, index);
        previous = node;
        if node != 0 {
            let allocator = heap_allocator(e);
            push_word(e, allocator, array, node);
        }
        count -= 1;
        cursor = x87_add(x87_load32(step.to_bits()), cursor);
        if count == 0 {
            break;
        }
    }
}

// Translated from 00e0da00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `getNodeSampledAtTick` (engine map name). Takes `selector` in `EAX`; then the stack words
/// `frame`, `sample`, `frames` (the address of a word holding the array of tree roots), `count`,
/// `total` (float) and `start` (float) (cdecl). Makes a local `hkArray` of words (room for
/// `count`: `_reserve` with element size 4 when below it), and when the root `frames[frame]` has
/// children runs `outputStatsForFrame` (`00e0cec0`) with `count` in `EAX`, that root in `EDI`, the
/// array in `ESI`, `start`, `total / count` (the x87 division by the integer, rounded to a float)
/// and the index `selector[+4]` (or `selector[+8]` when `selector[+0xc]` is not 0). The answer is
/// the array's word `sample` when `sample` is below the array's size, else 0. The local array is
/// then emptied (size 0, data freed through the router's allocator at `+0x10`, `capacity * 4`
/// bytes, unless its capacity word marks it as not owned).
pub fn get_node_sampled_at_tick(
    e: &mut Engine,
    selector: u32,
    frame: u32,
    sample: u32,
    frames: Ptr,
    count: u32,
    total: f32,
    start: f32,
) -> u32 {
    let step = x87_store32(x87_div(
        x87_load32(total.to_bits()),
        f64::from(count as i32),
    ));
    e.with_stack(0xc, |e, array| {
        let local = array.addr();
        e.mem.set_u32(local, 0);
        e.mem.set_u32(local + 4, 0);
        e.mem.set_u32(local + 8, NOT_OWNED);
        let allocator = heap_allocator(e);
        let capacity = e.mem.u32(local + 8) & CAPACITY_MASK;
        if (capacity as i32) < count as i32 {
            e.call(ARRAY_RESERVE, &args![allocator, local, count, 4u32]);
        }
        let roots = e.mem.u32(frames.addr());
        let root = e.mem.u32(roots.wrapping_add(frame.wrapping_mul(4)));
        let mut answer = 0u32;
        if e.mem.i32(root + CHILDREN + 4) > 0 {
            e.mem.set_u32(local + 4, 0);
            let index = if e.mem.u32(selector + 0xc) == 0 {
                e.mem.u32(selector + 4)
            } else {
                e.mem.u32(selector + 8)
            };
            output_stats_for_frame(
                e,
                count,
                root,
                local,
                x87_quiet(start),
                x87_quiet(f32::from_bits(step)),
                index,
            );
            if (sample as i32) < e.mem.i32(local + 4) {
                let data = e.mem.u32(local);
                answer = e.mem.u32(data.wrapping_add(sample.wrapping_mul(4)));
            }
        }
        let allocator = heap_allocator(e);
        let capacity = e.mem.u32(local + 8);
        e.mem.set_u32(local + 4, 0);
        if (capacity as i32) >= 0 {
            let data = e.mem.u32(local);
            e.vcall(
                allocator,
                0x10,
                &args![data, (capacity & CAPACITY_MASK).wrapping_mul(4)],
            );
        }
        answer
    })
}

// Translated from 00e0ee00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMonitorStreamAnalyzer::makeStatisticsTreeForMultipleFrames` (Xbox PDB), cdecl `(this,
/// frame, flag)`. `this` has an array of 12-byte `hkArray`s of 0x24-byte frame records at `+0`
/// and a byte buffer at `+0xc`. Makes a root node (`00e0d000`, no parent, the name at
/// `01063cbc`, kind 1; null when the allocator gives none) and makes room in its children for
/// the number of records of `this[frame]` (`_reserve`, element size 4); for each record builds a
/// tree with `makeStatisticsTreeForSingleFrame` (`00e0e630`, with the buffer's bytes from the
/// record's `+0x1c` and `+0x20` offsets, the record, its first word and the flag word) and
/// stores it as the root's child at that index. Returns the root.
pub fn hk_monitor_stream_analyzer_make_statistics_tree_for_multiple_frames(
    e: &mut Engine,
    this: Ptr,
    frame: u32,
    flag: u32,
) -> u32 {
    let this = this.addr();
    let allocator = heap_allocator(e);
    let block = e.vcall(allocator, 4, &args![NODE_SIZE]).u32();
    let root = if block == 0 {
        0
    } else {
        fn_00e0d000(e, Ptr::new(block), Ptr::new(0), 0x0106_3cbc, 1).addr()
    };
    let frames = e.mem.u32(this).wrapping_add(frame.wrapping_mul(12));
    let records = e.mem.u32(frames + 4);
    let allocator = heap_allocator(e);
    reserve_at_least(e, allocator, root + CHILDREN, records, 4);
    e.mem.set_u32(root + CHILDREN + 4, records);
    let mut index = 0u32;
    let mut offset = 0u32;
    let frames = e.mem.u32(this).wrapping_add(frame.wrapping_mul(12));
    if (e.mem.i32(frames + 4)) > 0 {
        loop {
            let frames = e.mem.u32(this).wrapping_add(frame.wrapping_mul(12));
            let record = e.mem.u32(frames).wrapping_add(offset);
            let buffer = e.mem.u32(this + 0xc);
            let begin = e.mem.u32(record + 0x1c).wrapping_add(buffer);
            let end = e.mem.u32(record + 0x20).wrapping_add(buffer);
            let slot = e
                .mem
                .u32(root + CHILDREN)
                .wrapping_add(index.wrapping_mul(4));
            let first_word = e.mem.u32(record);
            let tree = hk_monitor_stream_analyzer_make_statistics_tree_for_single_frame(
                e, begin, end, record, first_word, flag,
            );
            offset = offset.wrapping_add(0x24);
            e.mem.set_u32(slot, tree);
            index += 1;
            let frames_now = e.mem.u32(this).wrapping_add(frame.wrapping_mul(12));
            if (index as i32) >= e.mem.i32(frames_now + 4) {
                break;
            }
        }
    }
    root
}

// Translated from 00e0f5e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map, thiscall `(a, b)`: makes a local `hkArray` of words with room for 6
/// (an inline buffer of 24 bytes, capacity word `0x80000006`), room for `this[+4]` entries
/// (`_reserve`, element size 4, when below it), and fills it with
/// `makeStatisticsTreeForMultipleFrames` (`00e0ee00`, `this`, the index, a flag word with low byte
/// 0) of each index below `this[+4]`. Then `writeStatisticsDetailsToTga` (`00e0e470`, the array,
/// `a`, `this`, `b`, 0) and destroys every tree (`00e0d0a0`, then slot 8 of the router's allocator
/// at `+0x10` frees 0x58 bytes), clears the array's size and frees its data (slot `0x10`,
/// `capacity * 4` bytes) unless the capacity word is negative.
pub fn fn_00e0f5e0(e: &mut Engine, this: Ptr, a: u32, b: u32) {
    let analyzer = this.addr();
    let frames = e.mem.u32(analyzer + 4);
    e.with_stack(0x24, |e, local| {
        let array = local.addr();
        e.mem.set_u32(array, array + 0xc);
        e.mem.set_u32(array + 4, 0);
        e.mem.set_u32(array + 8, 0x8000_0006);
        let allocator = heap_allocator(e);
        reserve_at_least(e, allocator, array, frames, 4);
        e.mem.set_u32(array + 4, frames);
        for index in 0..frames as i32 {
            let data = hk_monitor_stream_analyzer_make_statistics_tree_for_multiple_frames(
                e,
                this,
                index as u32,
                0,
            );
            let slots = e.mem.u32(array);
            e.mem
                .set_u32(slots.wrapping_add((index as u32).wrapping_mul(4)), data);
        }
        hk_monitor_stream_analyzer_write_statistics_details_to_tga(
            e,
            Ptr::new(array),
            a,
            this,
            b,
            0,
        );
        for index in 0..frames as i32 {
            let slots = e.mem.u32(array);
            let tree = e
                .mem
                .u32(slots.wrapping_add((index as u32).wrapping_mul(4)));
            if tree != 0 {
                hk_monitor_stream_analyzer_node_destructor(e, Ptr::new(tree));
                let allocator = heap_allocator(e);
                e.vcall(allocator, 8, &args![tree, NODE_SIZE]);
            }
        }
        let allocator = heap_allocator(e);
        let capacity = e.mem.u32(array + 8);
        e.mem.set_u32(array + 4, 0);
        if (capacity as i32) >= 0 {
            let data = e.mem.u32(array);
            e.vcall(
                allocator,
                0x10,
                &args![data, (capacity & CAPACITY_MASK).wrapping_mul(4)],
            );
        }
    });
}

// Translated from 00e0f6e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: an `hkArray` constructor, thiscall: data 0, size 0, capacity word
/// `0x80000000`. Returns `this`.
pub fn fn_00e0f6e0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), 0);
    e.mem.set_u32(this.addr() + 4, 0);
    e.mem.set_u32(this.addr() + 8, NOT_OWNED);
    this
}

// Translated from 00e0f710 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map, thiscall `(a, b)`: makes the array of 12-byte `hkArray`s at `this`
/// hold `a + b` elements. Room is made first (the router's allocator at `+0x10`, `_reserve` with
/// element size 12 and the larger of `a + b` and twice the capacity (signed) when the capacity is
/// below it). Elements past `a + b` have their size cleared and their data freed through slot
/// `0x10` (`capacity * 0x24` bytes) unless not owned, then are emptied (data 0, capacity word
/// `0x80000000`); new elements below `a + b` past the old size are constructed as empty arrays.
/// `this + 0x18` becomes `a`, the size `a + b`, `this + 0x1c` `b`, and `00e0ccf0` is run on
/// `this`.
pub fn fn_00e0f710(e: &mut Engine, this: Ptr, a: u32, b: u32) {
    let array = this.addr();
    let wanted = a.wrapping_add(b);
    let allocator = heap_allocator(e);
    reserve_at_least(e, allocator, array, wanted, 12);
    let size = e.mem.u32(array + 4);
    let data = e.mem.u32(array);
    let excess = size.wrapping_sub(wanted) as i32;
    let mut element = data.wrapping_add(wanted.wrapping_mul(12));
    for _ in 0..excess.max(0) {
        let allocator = heap_allocator(e);
        let capacity = e.mem.u32(element + 8);
        e.mem.set_u32(element + 4, 0);
        if (capacity as i32) >= 0 {
            let inner = e.mem.u32(element);
            e.vcall(
                allocator,
                0x10,
                &args![inner, (capacity & CAPACITY_MASK).wrapping_mul(0x24)],
            );
        }
        e.mem.set_u32(element, 0);
        e.mem.set_u32(element + 8, NOT_OWNED);
        element = element.wrapping_add(12);
    }
    let size = e.mem.u32(array + 4);
    let data = e.mem.u32(array);
    let fresh = wanted.wrapping_sub(size) as i32;
    let mut element = data.wrapping_add(size.wrapping_mul(12));
    for _ in 0..fresh.max(0) {
        if element != 0 {
            e.mem.set_u32(element, 0);
            e.mem.set_u32(element + 4, 0);
            e.mem.set_u32(element + 8, NOT_OWNED);
        }
        element = element.wrapping_add(12);
    }
    e.mem.set_u32(array + 0x18, a);
    e.mem.set_u32(array + 4, wanted);
    e.mem.set_u32(array + 0x1c, b);
    fn_00e0ccf0(e, this);
}

// Translated from 00e10630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMonitorStreamAnalyzer::writeStatistics` (Xbox PDB), thiscall `(a, b)`: builds a local
/// `hkArray` of trees, one per index below `this[+4]` (`makeStatisticsTreeForMultipleFrames`,
/// `00e0ee00`, the flag word with low byte 0, room made with `00c90940` when full), runs
/// `writeStatisticsDetails` (`00e0fec0`, `a`, the array, `this[+0x18]`, `this[+0x1c]`, `b`,
/// `this[+0x20]`, 1), destroys each tree (`00e0d0a0`, then slot 8 of the router's allocator at
/// `+0x10` frees 0x58 bytes), clears the array's size and frees its data (slot `0x10`,
/// `capacity * 4` bytes) unless the capacity word is negative.
pub fn hk_monitor_stream_analyzer_write_statistics(e: &mut Engine, this: Ptr, a: u32, b: u32) {
    let analyzer = this.addr();
    e.with_stack(0xc, |e, local| {
        let array = local.addr();
        e.mem.set_u32(array, 0);
        e.mem.set_u32(array + 4, 0);
        e.mem.set_u32(array + 8, NOT_OWNED);
        let mut index = 0u32;
        while (index as i32) < e.mem.i32(analyzer + 4) {
            let tree = hk_monitor_stream_analyzer_make_statistics_tree_for_multiple_frames(
                e, this, index, 0,
            );
            let allocator = heap_allocator(e);
            push_word(e, allocator, array, tree);
            index += 1;
        }
        let (threads, spus, name) = (
            e.mem.u32(analyzer + 0x18),
            e.mem.u32(analyzer + 0x1c),
            e.mem.u32(analyzer + 0x20),
        );
        hk_monitor_stream_analyzer_write_statistics_details(
            e,
            a,
            Ptr::new(array),
            threads as i32,
            spus as i32,
            b,
            name,
            1,
        );
        let mut index = 0u32;
        while (index as i32) < e.mem.i32(array + 4) {
            let slots = e.mem.u32(array);
            let tree = e.mem.u32(slots.wrapping_add(index.wrapping_mul(4)));
            if tree != 0 {
                hk_monitor_stream_analyzer_node_destructor(e, Ptr::new(tree));
                let allocator = heap_allocator(e);
                e.vcall(allocator, 8, &args![tree, NODE_SIZE]);
            }
            index += 1;
        }
        let allocator = heap_allocator(e);
        let capacity = e.mem.u32(array + 8);
        e.mem.set_u32(array + 4, 0);
        if (capacity as i32) >= 0 {
            let data = e.mem.u32(array);
            e.vcall(
                allocator,
                0x10,
                &args![data, (capacity & CAPACITY_MASK).wrapping_mul(4)],
            );
        }
    });
}

/// The indenter string of `writeRawStatistics` (the local `hkArray` of bytes) gets one more
/// character: when it is full, `00c90940` makes room (the router's allocator at `+0x14`), then
/// `character` is stored at the end and the size grows.
fn indent_push(e: &mut Engine, array: u32, character: u8) {
    let allocator = object_allocator(e);
    reserve_one_more(e, allocator, array, 1);
    let at = e.mem.u32(array).wrapping_add(e.mem.u32(array + 4));
    e.mem.set_u8(at, character);
    let size = e.mem.u32(array + 4);
    e.mem.set_u32(array + 4, size.wrapping_add(1));
}

/// What `writeRawStatistics` does with its indenter when a timer opens: the last character
/// (the terminating zero) becomes a space, then a space and a zero are appended.
fn indent_deeper(e: &mut Engine, array: u32) {
    let data = e.mem.u32(array);
    let size = e.mem.u32(array + 4);
    e.mem.set_u8(data.wrapping_add(size).wrapping_sub(1), 0x20);
    indent_push(e, array, 0x20);
    indent_push(e, array, 0);
}

/// What `writeRawStatistics` does with its indenter when a timer closes: the size shrinks by two
/// and the byte before the new end becomes the terminating zero.
fn indent_shallower(e: &mut Engine, array: u32) {
    let data = e.mem.u32(array);
    let size = e.mem.u32(array + 4).wrapping_sub(2);
    e.mem.set_u32(array + 4, size);
    e.mem.set_u8(data.wrapping_add(size).wrapping_sub(1), 0);
}

// Translated from 00e0eef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMonitorStreamAnalyzer::writeRawStatistics` (Xbox PDB), thiscall `(stream)`: prints
/// `StatisticsDumpInfo(num_threads, num_spus, num_frames)` (`this[6]`, `this[7]` and the size
/// word of the first element of the array at `this + 0`; format `010ea838`), then for every
/// 12-byte `hkArray` at `this[0]` (`this[1]` of them) and every 0x24-byte frame record in it a
/// `FrameInfo(heading, frame, thread_id, time_counter)` line (`010ea7f0`; record words 0, 6, 3)
/// and the record's monitor stream commands, which lie in the byte buffer at `this[3]` between
/// the offsets in record words 7 and 8. An indenter string (`indenterIndenter`, reset to the
/// empty string for each record) is printed in front of each line. By tag character:
/// `T` timer begin (`010ea7d4`), 12 bytes, deeper; `E` timer end (`010ea7bc`), shallower;
/// `S` timer split (`010ea784`, shallower, then deeper again); `l` timer end (`010ea768`,
/// shallower); `L` timer begin (`010ea7a0`) and split (`010ea784`, the name in word 3), deeper,
/// 16 bytes; `M` add value (`010ea754`, the float as a `double`), 8 bytes; `N` 4 bytes;
/// `m` allocation or free (`010ea724`, `Alloc` at `010ea744` when the signed word 2 is not
/// negative, else `Free` at `010ea74c`, with the byte count masked to 31 bits), 12 bytes.
/// `F`, `O`, `P` and `p` do not move on, so the loop handles the same command again for ever
/// (as the exe's does); any other tag character ends the function early. At the end the
/// indenter's size is cleared and its memory freed (slot `0x10` of the allocator at `+0x14`)
/// unless its capacity word is negative.
pub fn hk_monitor_stream_analyzer_write_raw_statistics(e: &mut Engine, this: Ptr, stream: u32) {
    let analyzer = this.addr();
    e.with_stack(0xc, |e, array| {
        let array = array.addr();
        indenter_indenter(e, Ptr::new(array));
        let first = e.mem.u32(analyzer);
        let frames = e.mem.u32(first + 4);
        let (threads, spus) = (e.mem.u32(analyzer + 0x18), e.mem.u32(analyzer + 0x1c));
        e.call(
            PRINTF,
            &args![stream, 0x010e_a838u32, threads, spus, frames],
        );
        let mut thread = 0u32;
        let mut thread_offset = 0u32;
        'threads: while (thread as i32) < e.mem.i32(analyzer + 4) {
            let frames = e.mem.u32(analyzer).wrapping_add(thread_offset);
            let mut frame = 0u32;
            let mut record_offset = 0u32;
            if e.mem.i32(frames + 4) > 0 {
                loop {
                    let record = e.mem.u32(frames).wrapping_add(record_offset);
                    let (heading, thread_id, counter) = (
                        e.mem.u32(record),
                        e.mem.u32(record + 0x18),
                        e.mem.u32(record + 0xc),
                    );
                    e.call(
                        PRINTF,
                        &args![stream, 0x010e_a7f0u32, heading, frame, thread_id, counter],
                    );
                    let buffer = e.mem.u32(analyzer + 0xc);
                    let mut at = e.mem.u32(record + 0x1c).wrapping_add(buffer);
                    let end = e.mem.u32(record + 0x20).wrapping_add(buffer);
                    let allocator = object_allocator(e);
                    reserve_at_least(e, allocator, array, 1, 1);
                    e.mem.set_u32(array + 4, 1);
                    let data = e.mem.u32(array);
                    e.mem.set_u8(data, 0);
                    while at < end {
                        let name = e.mem.u32(at);
                        let tag = e.mem.u8(name) as i8 as i32;
                        match tag {
                            0x54 => {
                                let (word1, word2) = (e.mem.u32(at + 4), e.mem.u32(at + 8));
                                let indent = e.mem.u32(array);
                                e.call(
                                    PRINTF,
                                    &args![stream, 0x010e_a7d4u32, indent, name + 2, word1, word2],
                                );
                                indent_deeper(e, array);
                                at = at.wrapping_add(0xc);
                            }
                            0x45 => {
                                indent_shallower(e, array);
                                let (word1, word2) = (e.mem.u32(at + 4), e.mem.u32(at + 8));
                                let indent = e.mem.u32(array);
                                e.call(
                                    PRINTF,
                                    &args![stream, 0x010e_a7bcu32, indent, name + 2, word1, word2],
                                );
                                at = at.wrapping_add(0xc);
                            }
                            0x53 => {
                                indent_shallower(e, array);
                                let (word1, word2) = (e.mem.u32(at + 4), e.mem.u32(at + 8));
                                let indent = e.mem.u32(array);
                                e.call(
                                    PRINTF,
                                    &args![stream, 0x010e_a784u32, indent, name + 2, word1, word2],
                                );
                                indent_deeper(e, array);
                                at = at.wrapping_add(0xc);
                            }
                            0x6c => {
                                indent_shallower(e, array);
                                let (word1, word2) = (e.mem.u32(at + 4), e.mem.u32(at + 8));
                                let indent = e.mem.u32(array);
                                e.call(
                                    PRINTF,
                                    &args![stream, 0x010e_a768u32, indent, name + 2, word1, word2],
                                );
                                at = at.wrapping_add(0xc);
                            }
                            0x4c => {
                                let (word1, word2) = (e.mem.u32(at + 4), e.mem.u32(at + 8));
                                let indent = e.mem.u32(array);
                                e.call(
                                    PRINTF,
                                    &args![stream, 0x010e_a7a0u32, indent, name + 2, word1, word2],
                                );
                                let split = e.mem.u32(at + 0xc);
                                e.call(
                                    PRINTF,
                                    &args![stream, 0x010e_a784u32, indent, split + 2, word1, word2],
                                );
                                indent_deeper(e, array);
                                at = at.wrapping_add(0x10);
                            }
                            0x4d => {
                                let value = x87_load32(e.mem.u32(at + 4));
                                let indent = e.mem.u32(array);
                                e.call(
                                    PRINTF,
                                    &args![stream, 0x010e_a754u32, indent, name + 1, value],
                                );
                                at = at.wrapping_add(8);
                            }
                            0x4e => at = at.wrapping_add(4),
                            0x6d => {
                                let (word1, word2) = (e.mem.u32(at + 4), e.mem.u32(at + 8));
                                let kind = if (word2 as i32) < 0 {
                                    0x010e_a74cu32
                                } else {
                                    0x010e_a744u32
                                };
                                let indent = e.mem.u32(array);
                                e.call(
                                    PRINTF,
                                    &args![
                                        stream,
                                        0x010e_a724u32,
                                        indent,
                                        kind,
                                        name + 1,
                                        word1,
                                        word2 & 0x7fff_ffff
                                    ],
                                );
                                at = at.wrapping_add(0xc);
                            }
                            // `F`, `O`, `P` and `p`: the exe's jump goes straight back to the
                            // loop test without moving `at`.
                            0x46 | 0x4f | 0x50 | 0x70 => {}
                            _ => break 'threads,
                        }
                    }
                    record_offset = record_offset.wrapping_add(0x24);
                    frame += 1;
                    if (frame as i32) >= e.mem.i32(frames + 4) {
                        break;
                    }
                }
            }
            thread_offset = thread_offset.wrapping_add(0xc);
            thread += 1;
        }
        let allocator = object_allocator(e);
        e.mem.set_u32(array + 4, 0);
        let capacity = e.mem.u32(array + 8);
        if (capacity as i32) >= 0 {
            let data = e.mem.u32(array);
            e.vcall(allocator, 0x10, &args![data, capacity & CAPACITY_MASK]);
        }
    });
}

/// The string-keyed map (`hkCachedHashMap`) `hkBuildSizePerName` keeps its entries in: `getWithDefault`
/// (`00d87800`, thiscall `(key, default)`) and `insert` (`00d874d0`, thiscall `(key, value)`).
const SIZE_MAP_GET: u32 = 0x00d8_7800;
const SIZE_MAP_INSERT: u32 = 0x00d8_74d0;

/// Adds the eight floats and eight counts of `source` to those of `dest`, slot by slot. Each
/// float sum is a `FLD` of one operand and an `FADD` of the other from memory (so which one
/// is a signalling NaN matters); slot 0 loads `dest` first when `dest_first`, all the other
/// slots load `source` first.
fn merge_values(e: &mut Engine, dest: u32, source: u32, dest_first: bool) {
    for slot in 0..8u32 {
        let (d, s) = (e.mem.u32(dest + 4 * slot), e.mem.u32(source + 4 * slot));
        let sum = if slot == 0 && dest_first {
            x87_add(x87_load32(d), x87_m32(s))
        } else {
            x87_add(x87_load32(s), x87_m32(d))
        };
        e.mem.set_u32(dest + 4 * slot, x87_store32(sum));
        let count = e
            .mem
            .u16(dest + COUNTS + 2 * slot)
            .wrapping_add(e.mem.u16(source + COUNTS + 2 * slot));
        e.mem.set_u16(dest + COUNTS + 2 * slot, count);
    }
}

/// The search `hkBuildSizePerName` does in its arrays of 0x58-byte nodes: from the last element
/// down to the first, `strCmp(name, element's name)`; the first match is the answer.
fn find_node_by_name(e: &mut Engine, array: u32, name: u32) -> Option<u32> {
    let mut index = e.mem.i32(array + 4).wrapping_sub(1);
    while index >= 0 {
        let element = e
            .mem
            .u32(array)
            .wrapping_add((index as u32).wrapping_mul(NODE_SIZE));
        let element_name = e.mem.u32(element + NODE_NAME);
        if e.call(STR_CMP, &args![name, element_name]).u32() == 0 {
            return Some(element);
        }
        index -= 1;
    }
    None
}

/// What `hkBuildSizePerName` does when the search finds nothing: the array of 0x58-byte nodes
/// grows by one (room made with `_reserve` for the larger of size + 1 and twice the capacity),
/// the new element's children array is emptied (when its address is not 0), and a node
/// constructor (`00e0d000`, no parent, `name`, kind 0) runs on it. Returns the element, or 0.
fn append_named_node(e: &mut Engine, array: u32, name: u32) -> u32 {
    let allocator = object_allocator(e);
    let size = e.mem.u32(array + 4);
    reserve_at_least(e, allocator, array, size.wrapping_add(1), NODE_SIZE);
    let element = e
        .mem
        .u32(array + 4)
        .wrapping_mul(NODE_SIZE)
        .wrapping_add(e.mem.u32(array));
    if element != 0 {
        e.mem.set_u32(element + CHILDREN, 0);
        e.mem.set_u32(element + CHILDREN + 4, 0);
        e.mem.set_u32(element + CHILDREN + 8, NOT_OWNED);
    }
    let grown = e.mem.u32(array + 4);
    e.mem.set_u32(array + 4, grown.wrapping_add(1));
    let element = size.wrapping_mul(NODE_SIZE).wrapping_add(e.mem.u32(array));
    if element == 0 {
        0
    } else {
        fn_00e0d000(e, Ptr::new(element), Ptr::new(0), name, 0).addr()
    }
}

// Translated from 00e0f880 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkBuildSizePerName` (engine map name), cdecl `(node, map)`: finds or makes the 0x70-byte
/// entry for the node's name (`node[0x4c]`) in `map` (`getWithDefault`, key, default 0; a
/// new entry comes from the allocator at the router's `+0x14`, slot 4, 0x70 bytes, with a node
/// constructor (`00e0d000`, no parent, no name, kind 0), the arrays at `+0x58` and `+0x64`
/// emptied (data 0, size 0, capacity word `0x80000000`), the name stored at `+0x4c` and then
/// inserted under the name) and adds the node's values to it. Then for each child (recursing
/// first) it finds, in the entry's array at `+0x64`, the node with the child's name, searching
/// from the end (`strCmp`), or appends one, and adds the child's values to it. At the
/// end the node's parent (word `+0x48`), when there is one, is added the same way into the
/// entry's array at `+0x58`.
pub fn hk_build_size_per_name(e: &mut Engine, node: Ptr, map: u32) {
    let node = node.addr();
    let name = e.mem.u32(node + NODE_NAME);
    let mut entry = e.call(SIZE_MAP_GET, &args![map, name, 0u32]).u32();
    if entry == 0 {
        let allocator = object_allocator(e);
        let block = e.vcall(allocator, 4, &args![0x70u32]).u32();
        entry = 0;
        if block != 0 {
            fn_00e0d000(e, Ptr::new(block), Ptr::new(0), 0, 0);
            for (offset, value) in [
                (0x58u32, 0u32),
                (0x5c, 0),
                (0x60, NOT_OWNED),
                (0x64, 0),
                (0x68, 0),
                (0x6c, NOT_OWNED),
            ] {
                e.mem.set_u32(block + offset, value);
            }
            entry = block;
        }
        e.mem.set_u32(entry.wrapping_add(NODE_NAME), name);
        e.call(SIZE_MAP_INSERT, &args![map, name, entry]);
    }
    merge_values(e, entry, node, true);
    let mut index = 0u32;
    if child_count(e, node) > 0 {
        loop {
            let child = child_at(e, node, index);
            hk_build_size_per_name(e, Ptr::new(child), map);
            let child_name = e.mem.u32(child + NODE_NAME);
            let target = match find_node_by_name(e, entry + 0x64, child_name) {
                Some(found) => found,
                None => append_named_node(e, entry + 0x64, child_name),
            };
            merge_values(e, target, child, false);
            index += 1;
            if (index as i32) >= child_count(e, node) {
                break;
            }
        }
    }
    let parent = e.mem.u32(node + 0x48);
    if parent != 0 {
        let parent_name = e.mem.u32(parent + NODE_NAME);
        let target = match find_node_by_name(e, entry + 0x58, parent_name) {
            Some(found) => found,
            None => append_named_node(e, entry + 0x58, parent_name),
        };
        merge_values(e, target, parent, true);
    }
}

/// The string-keyed map's other functions, all thiscall on the map (`+0` the table, `+4` the
/// number of entries, `+8` the hash mask): the constructor (`00d87790`, one ignored stack word),
/// `getIterator` (`00d87410`), `isValid` (`00d874b0`, `(out byte, iterator)`), `getValue`
/// (`00d87440`, `(iterator)`), `getNext` (`00d87480`, `(iterator)`) and the destructor
/// (`00d877d0`).
const SIZE_MAP_NEW: u32 = 0x00d8_7790;
const SIZE_MAP_ITERATOR: u32 = 0x00d8_7410;
const SIZE_MAP_IS_VALID: u32 = 0x00d8_74b0;
const SIZE_MAP_VALUE: u32 = 0x00d8_7440;
const SIZE_MAP_NEXT: u32 = 0x00d8_7480;
const SIZE_MAP_DELETE: u32 = 0x00d8_77d0;
/// `hkAlgorithm::quickSortRecursive` (`00e10760`, cdecl `(array, low, high, compare)`) and the
/// comparison function `00e10740` the exe sorts the entries with.
const QUICK_SORT: u32 = 0x00e1_0760;
const SORT_COMPARE: u32 = 0x00e1_0740;

// Translated from 00e0fc50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `printStatisticsByType` (engine map name). Takes the stream in `EDI`; then the stack words
/// `node` and `threshold` (a float). Builds a string-keyed map of per-name entries
/// (`hkBuildSizePerName` on `node`) and gathers the entries in an array of words (room from the
/// allocator at the router's `+0xc`, `_reserve`, element size 4) by walking the map (the
/// iterator functions), sorts it with `00e10760` when it has more than one entry (compare
/// function `00e10740`) and destroys the map. It prints the four heading lines (`01095188`,
/// `010eaab0`, `010eaa88`, `010eaab0`), takes `limit = (float [last entry + 4] * threshold)` as
/// a `double`, and walks the entries from the last to the first: when the entry's float at `+4`
/// is above `limit` (ordered compare) it prints `01012648` and `010eaa60`, then the nodes of the
/// array at `+0x58` (each after `010eaa5c`, `printSingleNodeValues` with a total of 0.0), the
/// entry itself (total: the largest entry's float) and the nodes of the array at `+0x64` like
/// the first. Every entry is destroyed (`00e10930`) and freed (slot 8 of the router's allocator
/// at `+0x14`, 0x70 bytes) whether it was printed or not. At the end the entry array's memory is
/// freed (slot `0x10`, `capacity * 4` bytes) unless its capacity word is negative.
pub fn print_statistics_by_type(e: &mut Engine, stream: u32, node: Ptr, threshold: f32) {
    e.with_stack(0x28, |e, frame| {
        let frame = frame.addr();
        let (flag, top_slot, scratch, map, array) =
            (frame + 3, frame + 4, frame + 8, frame + 0xc, frame + 0x1c);
        e.mem.set_u8(scratch, 0);
        e.mem.set_u32(array, 0);
        e.mem.set_u32(array + 4, 0);
        e.mem.set_u32(array + 8, NOT_OWNED);
        let leftover = e.mem.u32(scratch);
        e.call(SIZE_MAP_NEW, &args![map, leftover]);
        hk_build_size_per_name(e, node, map);
        let entries = e.mem.u32(map + 4);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0xc);
        reserve_at_least(e, allocator, array, entries, 4);
        let mut iterator = e.call(SIZE_MAP_ITERATOR, &args![map]).u32();
        e.call(SIZE_MAP_IS_VALID, &args![map, flag, iterator]);
        while e.mem.u8(flag) != 0 {
            let value = e.call(SIZE_MAP_VALUE, &args![map, iterator]).u32();
            let (data, size) = (e.mem.u32(array), e.mem.u32(array + 4));
            e.mem
                .set_u32(data.wrapping_add(size.wrapping_mul(4)), value);
            e.mem.set_u32(array + 4, size.wrapping_add(1));
            iterator = e.call(SIZE_MAP_NEXT, &args![map, iterator]).u32();
            e.call(SIZE_MAP_IS_VALID, &args![map, flag, iterator]);
        }
        let size = e.mem.i32(array + 4);
        if size > 1 {
            let data = e.mem.u32(array);
            e.call(
                QUICK_SORT,
                &args![data, 0u32, (size - 1) as u32, SORT_COMPARE],
            );
        }
        e.call(SIZE_MAP_DELETE, &args![map]);
        let (data, size) = (e.mem.u32(array), e.mem.u32(array + 4));
        let last = e
            .mem
            .u32(data.wrapping_add(size.wrapping_mul(4)).wrapping_sub(4));
        let top = x87_quiet_bits(e.mem.u32(last + 4));
        e.mem.set_u32(top_slot, top);
        for format in [0x0109_5188u32, 0x010e_aab0, 0x010e_aa88, 0x010e_aab0] {
            e.call(PRINTF, &args![stream, format]);
        }
        let mut index = e.mem.i32(array + 4).wrapping_sub(1);
        e.mem.set_u32(scratch, index as u32);
        if index >= 0 {
            let limit = x87_mul(
                x87_load32(e.mem.u32(top_slot)),
                x87_m32(threshold.to_bits()),
            );
            e.mem.set_f64(map, limit);
            loop {
                let data = e.mem.u32(array);
                let entry = e.mem.u32(data.wrapping_add((index as u32).wrapping_mul(4)));
                if x87_load32(e.mem.u32(entry + 4)) > e.mem.f64(map) {
                    e.call(PRINTF, &args![stream, 0x0101_2648u32]);
                    e.call(PRINTF, &args![stream, 0x010e_aa60u32]);
                    for (list, count) in [(0x58u32, 0x5cu32), (0x64, 0x68)] {
                        if list == 0x64 {
                            let total = x87_quiet(f32::from_bits(e.mem.u32(top_slot)));
                            print_single_node_values(e, stream, total, Ptr::new(entry));
                        }
                        let mut position = 0u32;
                        let mut offset = 0u32;
                        if e.mem.i32(entry + count) > 0 {
                            loop {
                                e.call(PRINTF, &args![stream, 0x010e_aa5cu32]);
                                let nodes = e.mem.u32(entry + list).wrapping_add(offset);
                                print_single_node_values(e, stream, 0.0, Ptr::new(nodes));
                                position += 1;
                                offset = offset.wrapping_add(NODE_SIZE);
                                if (position as i32) >= e.mem.i32(entry + count) {
                                    break;
                                }
                            }
                        }
                    }
                }
                hk_monitor_stream_analyzer_print_by_type_info_destructor(e, Ptr::new(entry));
                let allocator = object_allocator(e);
                e.vcall(allocator, 8, &args![entry, 0x70u32]);
                index = index.wrapping_sub(1);
                e.mem.set_u32(scratch, index as u32);
                if index < 0 {
                    break;
                }
            }
        }
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0xc);
        let capacity = e.mem.u32(array + 8);
        e.mem.set_u32(array + 4, 0);
        if (capacity as i32) >= 0 {
            let data = e.mem.u32(array);
            e.vcall(
                allocator,
                0x10,
                &args![data, (capacity & CAPACITY_MASK).wrapping_mul(4)],
            );
        }
    });
}

/// The text stream the summaries are built in: `hkOstream::hkOstream_ov2` (`00cc0630`, thiscall
/// `(array)`, writing into an `hkArray` of bytes) and its destructor (`00cc00e0`).
const TEXT_STREAM_NEW: u32 = 0x00cc_0630;
const TEXT_STREAM_DELETE: u32 = 0x00cc_00e0;

/// Starts a text stream at `stream` (12 bytes) writing into the empty `hkArray` at `array`.
fn open_text(e: &mut Engine, stream: u32, array: u32) {
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 4, 0);
    e.mem.set_u32(array + 8, NOT_OWNED);
    e.call(TEXT_STREAM_NEW, &args![stream, array]);
}

/// Ends the text stream: its destructor runs, then the array's size is cleared and its data
/// freed (slot `0x10` of the router's allocator at `+0x10`, `capacity` bytes) unless the
/// capacity word is negative.
fn close_text(e: &mut Engine, stream: u32, array: u32) {
    e.call(TEXT_STREAM_DELETE, &args![stream]);
    let allocator = heap_allocator(e);
    let capacity = e.mem.u32(array + 8);
    e.mem.set_u32(array + 4, 0);
    if (capacity as i32) >= 0 {
        let data = e.mem.u32(array);
        e.vcall(allocator, 0x10, &args![data, capacity & CAPACITY_MASK]);
    }
}

/// `operator<<(char)` (`00cc01a0`) of a text stream.
fn put_char(e: &mut Engine, stream: u32, character: u32) {
    e.call(STREAM_PUT_CHAR, &args![stream, character]);
}

/// `count` spaces when `count` is above 0 (signed).
fn put_spaces(e: &mut Engine, stream: u32, count: i32) {
    for _ in 0..count.max(0) {
        put_char(e, stream, 0x20);
    }
}

/// The gap in front of a column of the summary: a tab when the config's byte at `+0x1a` is not
/// 0, else spaces up to `column + config[0]` characters, less what the text already holds (the
/// array's size word).
fn column_gap(e: &mut Engine, stream: u32, array: u32, config: u32, column: i32) {
    if e.mem.u8(config + 0x1a) != 0 {
        put_char(e, stream, 9);
    } else {
        let gap = column
            .wrapping_sub(e.mem.i32(array + 4))
            .wrapping_add(e.mem.i32(config));
        put_spaces(e, stream, gap);
    }
}

// Translated from 00e0d2c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMonitorStreamAnalyzer::showCombinedThreadSummaryRec` (Xbox PDB), cdecl `(stream, node,
/// depth, threads, spus, config)`. The config's words are: `+0` the column of the first value,
/// `+4` the width of a column, `+8` the indent per depth, `+0xc` the indent of the values per
/// depth; bytes `+0x10` (show open/closed marks), `+0x18` and `+0x19` (the marks), `+0x1a` (use
/// tabs), and `+0x14` a node to mark. Builds one line in a text stream: when `depth` is not 0
/// the mark of the config's node, the indent, the open/closed mark (only with the marks
/// switched on), then `"%s (%i) "` with the node's name and the sum of its first `threads +
/// spus` counts (the 16-bit words from `node + 0x20`, read as far as that number says), then
/// for every one of those columns the gap and `"%-6.3f (%i)"` of the float and count (the
/// first `threads` of them also add up, so do the rest), then the averages of the thread sums
/// and of the spu sums, each when there is more than one. The line goes to `stream` (the
/// string, then a newline). Then it does the same for each child with `depth + 1`, unless the
/// marks are on and the node's flag bit 0 is clear.
pub fn hk_monitor_stream_analyzer_show_combined_thread_summary_rec(
    e: &mut Engine,
    stream: u32,
    node: u32,
    depth: i32,
    threads: i32,
    spus: i32,
    config: u32,
) {
    e.with_stack(0x2c, |e, block| {
        let block = block.addr();
        let (text, array) = (block + 0x14, block + 0x20);
        open_text(e, text, array);
        let columns = threads.wrapping_add(spus);
        let marks = e.mem.u8(config + 0x10) != 0;
        if depth != 0 {
            if marks {
                let mark = if e.mem.u32(config + 0x14) == node {
                    u32::from(e.mem.u8(config + 0x18))
                } else {
                    0x20
                };
                put_char(e, text, mark);
            }
            let indent = e.mem.i32(config + 8).wrapping_mul(depth.wrapping_sub(1));
            put_spaces(e, text, indent);
            if marks {
                let mark = if e.mem.i32(node + CHILDREN + 4) < 1 {
                    0x20
                } else if e.mem.u8(node + NODE_FLAGS) & 1 == 0 {
                    u32::from(e.mem.u8(config + 0x18))
                } else {
                    u32::from(e.mem.u8(config + 0x19))
                };
                put_char(e, text, mark);
            }
            let mut total = 0u32;
            for slot in 0..columns.max(0) as u32 {
                total = total.wrapping_add(u32::from(e.mem.u16(node + COUNTS + 2 * slot)));
            }
            let name = e.mem.u32(node + NODE_NAME);
            e.call(PRINTF, &args![text, 0x010e_a6ccu32, name, total]);
            let (mut sum_threads, mut sum_spus) = (0u32, 0u32);
            let (mut count_threads, mut count_spus) = (0u32, 0u32);
            let base_column = e.mem.i32(config + 0xc).wrapping_mul(depth.wrapping_sub(1));
            let mut index = 0i32;
            while index < columns {
                let column = base_column.wrapping_add(e.mem.i32(config + 4).wrapping_mul(index));
                column_gap(e, text, array, config, column);
                let slot = index as u32;
                let count = u32::from(e.mem.u16(node + COUNTS + 2 * slot));
                let shown = x87_load32(e.mem.u32(node + 4 * slot));
                e.call(PRINTF, &args![text, 0x010e_a6c0u32, shown, count]);
                let value = x87_load32(e.mem.u32(node + 4 * slot));
                if index < threads {
                    sum_threads = x87_store32(x87_add(value, x87_m32(sum_threads)));
                    count_threads = count_threads.wrapping_add(count);
                } else {
                    sum_spus = x87_store32(x87_add(value, x87_m32(sum_spus)));
                    count_spus = count_spus.wrapping_add(count);
                }
                index += 1;
            }
            let mut next = columns;
            if threads > 1 {
                let column = base_column.wrapping_add(e.mem.i32(config + 4).wrapping_mul(next));
                column_gap(e, text, array, config, column);
                next = next.wrapping_add(1);
                let average = x87_div(x87_load32(sum_threads), f64::from(threads));
                e.call(PRINTF, &args![text, 0x010e_a6c0u32, average, count_threads]);
            }
            if spus > 1 {
                let column = base_column.wrapping_add(e.mem.i32(config + 4).wrapping_mul(next));
                column_gap(e, text, array, config, column);
                let average = x87_div(x87_load32(sum_spus), f64::from(spus));
                e.call(PRINTF, &args![text, 0x010e_a6c0u32, average, count_spus]);
            }
            let data = e.mem.u32(array);
            e.call(STREAM_PUT_STRING, &args![stream, data]);
            put_char(e, stream, 0xa);
        }
        if !marks || e.mem.u8(node + NODE_FLAGS) & 1 != 0 {
            let mut index = 0u32;
            while (index as i32) < child_count(e, node) {
                let child = child_at(e, node, index);
                hk_monitor_stream_analyzer_show_combined_thread_summary_rec(
                    e,
                    stream,
                    child,
                    depth.wrapping_add(1),
                    threads,
                    spus,
                    config,
                );
                index += 1;
            }
        }
        close_text(e, text, array);
    });
}

// Translated from 00e0d640 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMonitorStreamAnalyzer::showCombinedThreadSummaryForSingleFrame` (Xbox PDB), cdecl
/// `(node, threads, spus, stream, config)`: stores `findMaxTimerNameIndent` of the node (depth
/// 0, width `config[2]`, the byte `config[0x10]`) in `config[0]`; builds the heading line in a
/// text stream: `"Timer Name"`, then, when there is more than one thread or any spu, `"Thread
/// %d"` for each thread, `"Spu %d"` for each spu, `"Average Cpu"` when there is more than one
/// thread and `"Average Spu"` when there is more than one spu, each after the column gap
/// (a tab when `config[0x1a]` is set, else spaces up to the column `config[1] * index +
/// config[0]`, less what the line holds). The line goes to `stream` (the string, then
/// `01095188`), and `showCombinedThreadSummaryRec` runs for the node with depth 0.
pub fn hk_monitor_stream_analyzer_show_combined_thread_summary_for_single_frame(
    e: &mut Engine,
    node: u32,
    threads: i32,
    spus: i32,
    stream: u32,
    config: u32,
) {
    let width = e.mem.u32(config + 8);
    let only_open = e.mem.u8(config + 0x10);
    let indent = hk_monitor_stream_analyzer_find_max_timer_name_indent(
        e,
        Ptr::new(node),
        0,
        width,
        only_open,
    );
    e.mem.set_u32(config, indent as u32);
    e.with_stack(0x18, |e, block| {
        let block = block.addr();
        let (text, array) = (block, block + 0xc);
        open_text(e, text, array);
        e.call(PRINTF, &args![text, 0x010e_a704u32]);
        if threads > 1 || spus > 0 {
            let width = |e: &Engine| e.mem.i32(config + 4);
            let mut index = 0i32;
            while index < threads {
                let column = width(e).wrapping_mul(index);
                column_gap(e, text, array, config, column);
                e.call(PRINTF, &args![text, 0x010e_a6f8u32, index as u32]);
                index += 1;
            }
            let mut index = 0i32;
            while index < spus {
                let column = index.wrapping_add(threads).wrapping_mul(width(e));
                column_gap(e, text, array, config, column);
                e.call(PRINTF, &args![text, 0x010e_a6f0u32, index as u32]);
                index += 1;
            }
            let mut next = threads.wrapping_add(spus);
            if threads > 1 {
                let column = width(e).wrapping_mul(next);
                column_gap(e, text, array, config, column);
                next = next.wrapping_add(1);
                e.call(PRINTF, &args![text, 0x010e_a6e4u32]);
            }
            if spus > 1 {
                let column = width(e).wrapping_mul(next);
                column_gap(e, text, array, config, column);
                e.call(PRINTF, &args![text, 0x010e_a6d8u32]);
            }
        }
        let data = e.mem.u32(array);
        e.call(STREAM_PUT_STRING, &args![stream, data]);
        e.call(PRINTF, &args![stream, 0x0109_5188u32]);
        hk_monitor_stream_analyzer_show_combined_thread_summary_rec(
            e, stream, node, 0, threads, spus, config,
        );
        close_text(e, text, array);
    });
}

/// The constants of `getTimerLimits`: the float `1e30` (`010ea720`) every frame's start begins
/// as, the double `16666.0` (`010ea718`) and float `16666.0` (`010ea710`) the limit is kept
/// above, the double `1000.0` (`01017b70`), the double `0.25` (`010290b0`), the double `0.05`
/// widened from a float (`0103c7c8`), and the float global (`011bdd84`) that holds the smoothed
/// limit between calls.
const START_UNSET: u32 = 0x010e_a720;
const LIMIT_FLOOR_DOUBLE: u32 = 0x010e_a718;
const LIMIT_FLOOR_FLOAT: u32 = 0x010e_a710;
const THOUSAND: u32 = 0x0101_7b70;
const QUARTER: u32 = 0x0102_90b0;
const SMOOTHING: u32 = 0x0103_c7c8;
const SMOOTH_LIMIT: u32 = 0x011b_dd84;

/// The low word `FISTP qword` leaves with the truncating rounding mode: the truncated value, or
/// the "integer indefinite" (`0x8000000000000000`, low word 0) for a NaN and out-of-range values.
fn fistp_truncated_low_word(value: f64) -> u32 {
    if value.is_nan()
        || value >= 9_223_372_036_854_775_808.0
        || value < -9_223_372_036_854_775_808.0
    {
        0
    } else {
        value.trunc() as i64 as u32
    }
}

/// A float stored with `FSTP dword` and loaded again with `FLD dword`: rounded to a float, a
/// signalling NaN quiet.
fn through_float(value: f64) -> f64 {
    x87_load32(x87_store32(value))
}

// Translated from 00e0dd30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMonitorStreamAnalyzer::getTimerLimits` (Xbox PDB), cdecl `(trees, range, infos,
/// max_out, starts)`. `trees` is an `hkArray` of one tree per thread, `range` has the first
/// frame (word 0), the wanted frame count (word 1) and the floats at `+0x18` (a fixed limit)
/// and `+0x1c` (a padding); `infos` points at a pointer to the 0x24-byte frame records (the
/// record's words 1 to 3 are used: word 3 picks word 1 or 2 as the index of the value to look
/// at). The frame range is cut down to the number of frames of the first tree. With room made
/// for the wanted count in `starts` (an `hkArray` of floats: the router's allocator at `+0x10`,
/// `_reserve`, element size 4; new elements zeroed), `*max_out` is 0.0 and for each frame of the
/// range its start is set to `1e30`, lowered to the smallest start time (the double at `+0x40`,
/// rounded to a float) of the frame's nodes in every thread, and `*max_out` is raised to the
/// largest of `value + start time - start` over those nodes. A padding above 0.0 is added to
/// the starts. A fixed limit above 0.0 becomes `*max_out`. Otherwise, with a single frame,
/// the float global is moved a twentieth of the way to the next power of two times 1000 above
/// `*max_out / 1000` (but no more than a quarter of it), kept at 16666.0 or above, and `*max_out`
/// is set to it.
pub fn hk_monitor_stream_analyzer_get_timer_limits(
    e: &mut Engine,
    trees: u32,
    range: u32,
    infos: u32,
    max_out: u32,
    starts: u32,
) {
    let thread_count = e.mem.i32(trees + 4);
    let first_tree = e.mem.u32(e.mem.u32(trees));
    let frames = e.mem.i32(first_tree + 0x34);
    let mut count = e.mem.i32(range + 4);
    let mut end = e.mem.i32(range).wrapping_add(count);
    if end > frames {
        count = frames.wrapping_sub(e.mem.i32(range));
        end = frames;
    }
    if count <= 0 {
        return;
    }
    let wanted = e.mem.u32(range + 4);
    let allocator = heap_allocator(e);
    reserve_at_least(e, allocator, starts, wanted, 4);
    let size = e.mem.u32(starts + 4);
    let data = e.mem.u32(starts);
    let missing = wanted.wrapping_sub(size) as i32;
    let mut at = data.wrapping_add(size.wrapping_mul(4));
    for _ in 0..missing.max(0) {
        e.mem.set_u32(at, 0);
        at = at.wrapping_add(4);
    }
    e.mem.set_u32(starts + 4, wanted);
    e.mem.set_u32(max_out, 0);
    let mut frame = e.mem.i32(range);
    if frame < end {
        let records = e.mem.u32(e.mem.u32(infos));
        let mut record = records
            .wrapping_add((frame as u32).wrapping_mul(0x24))
            .wrapping_add(4);
        loop {
            let slot = frame.wrapping_sub(e.mem.i32(range)) as u32;
            let selector = if e.mem.u32(record + 8) == 0 {
                e.mem.u32(record)
            } else {
                e.mem.u32(record + 4)
            };
            let out = e.mem.u32(starts);
            e.mem.set_u32(
                out.wrapping_add(slot.wrapping_mul(4)),
                x87_store32(x87_load32(e.mem.u32(START_UNSET))),
            );
            let mut thread = 0i32;
            while thread < thread_count {
                let tree = e.mem.u32(
                    e.mem
                        .u32(trees)
                        .wrapping_add((thread as u32).wrapping_mul(4)),
                );
                let slots = e.mem.u32(tree + CHILDREN);
                let node = e
                    .mem
                    .u32(slots.wrapping_add((frame as u32).wrapping_mul(4)));
                let list = node + CHILDREN;
                if e.mem.i32(node + CHILDREN + 4) > 0 {
                    let mut index = 0u32;
                    loop {
                        let child = e
                            .mem
                            .u32(e.mem.u32(list).wrapping_add(index.wrapping_mul(4)));
                        let start = through_float(quiet_f64(e.mem.f64(child + START_TIME)));
                        let out = e.mem.u32(starts).wrapping_add(slot.wrapping_mul(4));
                        let current = through_float(x87_load32(e.mem.u32(out)));
                        let lowest = if current > start { start } else { current };
                        e.mem.set_u32(out, x87_store32(through_float(lowest)));
                        index += 1;
                        if (index as i32) >= e.mem.i32(list + 4) {
                            break;
                        }
                    }
                    let mut index = 0u32;
                    while (index as i32) < e.mem.i32(list + 4) {
                        let child = e
                            .mem
                            .u32(e.mem.u32(list).wrapping_add(index.wrapping_mul(4)));
                        let value =
                            x87_load32(e.mem.u32(child.wrapping_add(selector.wrapping_mul(4))));
                        let later = x87_add(value, e.mem.f64(child + START_TIME));
                        let out = e.mem.u32(starts).wrapping_add(slot.wrapping_mul(4));
                        let length = x87_sub(later, x87_m32(e.mem.u32(out)));
                        let length = through_float(length);
                        let longest = through_float(x87_load32(e.mem.u32(max_out)));
                        let kept = if longest < length { length } else { longest };
                        e.mem.set_u32(max_out, x87_store32(through_float(kept)));
                        index += 1;
                    }
                }
                thread += 1;
            }
            record = record.wrapping_add(0x24);
            frame = frame.wrapping_add(1);
            if frame >= end {
                break;
            }
        }
    }
    let padding = e.mem.u32(range + 0x1c);
    if 0.0 < x87_m32(padding) {
        let first = e.mem.i32(range);
        let mut frame = first;
        if end.wrapping_sub(first) >= 4 {
            let stop = end.wrapping_sub(3);
            loop {
                for lane in 0..4i32 {
                    let index = frame.wrapping_add(lane).wrapping_sub(e.mem.i32(range)) as u32;
                    let out = e.mem.u32(starts).wrapping_add(index.wrapping_mul(4));
                    let current = e.mem.u32(out);
                    let sum = if lane < 2 {
                        x87_add(x87_load32(padding), x87_m32(current))
                    } else {
                        x87_add(x87_load32(current), x87_m32(padding))
                    };
                    e.mem.set_u32(out, x87_store32(sum));
                }
                frame = frame.wrapping_add(4);
                if frame >= stop {
                    break;
                }
            }
        }
        while frame < end {
            let index = frame.wrapping_sub(e.mem.i32(range)) as u32;
            let out = e.mem.u32(starts).wrapping_add(index.wrapping_mul(4));
            let sum = x87_add(x87_load32(padding), x87_m32(e.mem.u32(out)));
            e.mem.set_u32(out, x87_store32(sum));
            frame = frame.wrapping_add(1);
        }
    }
    let fixed = e.mem.u32(range + 0x18);
    if 0.0 < x87_m32(fixed) {
        e.mem.set_u32(max_out, x87_store32(x87_load32(fixed)));
        return;
    }
    if count != 1 {
        return;
    }
    let quotient = x87_div(x87_load32(e.mem.u32(max_out)), e.mem.f64(THOUSAND));
    let mut bits = fistp_truncated_low_word(quotient);
    for shift in [1u32, 2, 4, 8, 16] {
        bits |= bits >> shift;
    }
    let power = bits.wrapping_add(1);
    let smooth = x87_load32(e.mem.u32(SMOOTH_LIMIT));
    let quarter = x87_store32(x87_mul(e.mem.f64(QUARTER), smooth));
    let scaled = x87_store32(x87_mul(e.mem.f64(THOUSAND), fild_unsigned(e, power)));
    let excess = x87_store32(x87_sub(x87_load32(scaled), smooth));
    let (excess_value, quarter_value) = (x87_load32(excess), x87_load32(quarter));
    let step = if excess_value > quarter_value {
        quarter_value
    } else {
        excess_value
    };
    let step = x87_store32(step);
    let moved = x87_store32(x87_add(
        smooth,
        x87_mul(x87_load32(step), e.mem.f64(SMOOTHING)),
    ));
    e.mem.set_u32(max_out, x87_store32(x87_load32(moved)));
    let moved = x87_load32(moved);
    let limit = if moved < e.mem.f64(LIMIT_FLOOR_DOUBLE) {
        x87_store32(x87_load32(e.mem.u32(LIMIT_FLOOR_FLOAT)))
    } else {
        x87_store32(moved)
    };
    e.mem.set_u32(max_out, x87_store32(x87_load32(limit)));
    e.mem.set_u32(SMOOTH_LIMIT, x87_store32(x87_load32(limit)));
}

/// The pointer map (`hkMapBase<unsigned long, unsigned long>`) functions `drawStatistics` uses
/// on a local map, all thiscall: `reserve` (`00cb1fe0`, `(allocator, count)`),
/// `clearAndDeallocate` (`00cb1a90`, `(allocator)`) and the destructor (`00a29680`).
const COLOR_MAP_RESERVE: u32 = 0x00cb_1fe0;
const COLOR_MAP_CLEAR: u32 = 0x00cb_1a90;
const COLOR_MAP_DELETE: u32 = 0x00a2_9680;

// Translated from 00e0daf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `drawStatistics` (engine map name). Takes `selector` in `EAX`; then the stack words
/// `frame`, `frames` (the address of a thread tree's array of frame nodes), `out`, `rows`, `names`, `stride`,
/// `width`, `total` and `start` (floats) and `colors` (a map, cdecl). Fills `rows` words of
/// colour for each of the `width` columns: the node `frames[frame]` sampled at each
/// column (`outputStatsForFrame`, as `getNodeSampledAtTick` does, with `start`, `total /
/// width` and the index `selector[+4]`, or `selector[+8]` when `selector[+0xc]` is not 0) is
/// walked up through its parents, colour -1 at first: a name found in the local map gives its
/// colour and ends the walk; a name found in `colors` gives the colour so far; otherwise the
/// name is looked up (`strCasecmp`, the string with bit 0 masked off) in the pairs of the array
/// at `names + 8` (`names[+0xc]` of them: string, colour), a match putting its colour in the
/// local map and ending the walk, and no match recording the colour so far under the name in
/// `colors` (the router's allocator at `+0x10`, insert). The colour is stored `rows` times, from
/// `out + column * 4`, `stride` words apart. Nothing is drawn when the tree has no children.
pub fn draw_statistics(
    e: &mut Engine,
    selector: u32,
    frame: u32,
    frames: u32,
    out: u32,
    rows: i32,
    names: u32,
    stride: u32,
    width: i32,
    total: f32,
    start: f32,
    colors: u32,
) {
    e.with_stack(0x20, |e, block| {
        let block = block.addr();
        let (map, array, found_in_map, found_in_colors) =
            (block, block + 0xc, block + 0x18, block + 0x1c);
        e.mem.set_u32(map, 0);
        e.mem.set_u32(map + 4, 0);
        e.mem.set_u32(map + 8, 0xffff_ffff);
        let allocator = heap_allocator(e);
        e.call(COLOR_MAP_RESERVE, &args![map, allocator, 0u32]);
        let step = x87_store32(x87_div(x87_load32(total.to_bits()), f64::from(width)));
        e.mem.set_u32(array, 0);
        e.mem.set_u32(array + 4, 0);
        e.mem.set_u32(array + 8, NOT_OWNED);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let capacity = e.mem.u32(array + 8) & CAPACITY_MASK;
        if (capacity as i32) < width {
            let allocator = e.mem.u32(router + 0x10);
            e.call(ARRAY_RESERVE, &args![allocator, array, width as u32, 4u32]);
        }
        let roots = e.mem.u32(frames);
        let tree = e.mem.u32(roots.wrapping_add(frame.wrapping_mul(4)));
        if e.mem.i32(tree + CHILDREN + 4) > 0 {
            e.mem.set_u32(array + 4, 0);
            let index = if e.mem.u32(selector + 0xc) == 0 {
                e.mem.u32(selector + 4)
            } else {
                e.mem.u32(selector + 8)
            };
            output_stats_for_frame(
                e,
                width as u32,
                tree,
                array,
                x87_quiet(start),
                x87_quiet(f32::from_bits(step)),
                index,
            );
            let sampled = e.mem.i32(array + 4);
            let mut column = 0i32;
            while column < sampled && column < width {
                let entry = e.mem.u32(
                    e.mem
                        .u32(array)
                        .wrapping_add((column as u32).wrapping_mul(4)),
                );
                let mut color = 0xffff_ffffu32;
                let mut node = entry;
                while node != 0 {
                    let name = e.mem.u32(node + NODE_NAME);
                    if e.call(MAP_GET, &args![map, name, found_in_map]).u32() == 0 {
                        color = e.mem.u32(found_in_map);
                        break;
                    }
                    if e.call(MAP_GET, &args![colors, name, found_in_colors]).u32() == 0 {
                        color = e.mem.u32(found_in_colors);
                    } else {
                        let mut pair = 0u32;
                        let mut matched = false;
                        while (pair as i32) < e.mem.i32(names + 0xc) {
                            let table = e.mem.u32(names + 8);
                            let candidate =
                                e.mem.u32(table.wrapping_add(pair.wrapping_mul(8))) & 0xffff_fffe;
                            if e.call(STR_CASE_CMP, &args![candidate, name]).u32() == 0 {
                                color = e.mem.u32(table.wrapping_add(pair.wrapping_mul(8)) + 4);
                                matched = true;
                                break;
                            }
                            pair += 1;
                        }
                        let allocator = heap_allocator(e);
                        if matched {
                            e.call(MAP_INSERT, &args![map, allocator, name, color]);
                            break;
                        }
                        e.call(MAP_INSERT, &args![colors, allocator, name, color]);
                    }
                    node = e.mem.u32(node + 0x48);
                }
                if rows > 0 {
                    let mut at = out.wrapping_add((column as u32).wrapping_mul(4));
                    for _ in 0..rows {
                        e.mem.set_u32(at, color);
                        at = at.wrapping_add(stride.wrapping_mul(4));
                    }
                }
                column += 1;
            }
        }
        let allocator = heap_allocator(e);
        let capacity = e.mem.u32(array + 8);
        e.mem.set_u32(array + 4, 0);
        if (capacity as i32) >= 0 {
            let data = e.mem.u32(array);
            e.vcall(
                allocator,
                0x10,
                &args![data, (capacity & CAPACITY_MASK).wrapping_mul(4)],
            );
        }
        e.mem.set_u32(array, 0);
        e.mem.set_u32(array + 8, NOT_OWNED);
        let allocator = heap_allocator(e);
        e.call(COLOR_MAP_CLEAR, &args![map, allocator]);
        e.call(COLOR_MAP_DELETE, &args![map]);
    });
}

/// The float `-1.0` (`01012054`) the limit starts as, the float `5.0` (`0101712c`) a column
/// spacing has to exceed for the red ticks, and the double `0.001` (`01016978`, a float
/// widened); `_ftol2` (`00ec62c0`, the float in `ST0` as a leading `f64`, the low word of the
/// truncation); `hkMemoryRouter::easyAlloc` (`00c85690`, cdecl `(allocator, bytes)`) and
/// `easyFree` (`00c856c0`, cdecl `(allocator, block)`).
const LIMIT_UNSET: u32 = 0x0101_2054;
const TICK_THRESHOLD: u32 = 0x0101_712c;
const THOUSANDTH: u32 = 0x0101_6978;
const FTOL: u32 = 0x00ec_62c0;
const EASY_ALLOC: u32 = 0x00c8_5690;
const EASY_FREE: u32 = 0x00c8_56c0;

/// `_ftol2` of an x87 value: the truncated integer's low word.
fn ftol(e: &mut Engine, value: f64) -> i32 {
    e.call(FTOL, &args![value]).u32() as i32
}

// Translated from 00e0e0c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMonitorStreamAnalyzer::writeStatisticsDetailsToTexture` (Xbox PDB), cdecl `(trees,
/// layout, analyzer, image_out, height_out, limit_out)`. Nothing happens when `trees` (an
/// `hkArray` of one tree per thread) holds no tree. `layout` is the range `getTimerLimits` takes
/// (first frame, wanted count, the floats at `+0x18` and `+0x1c`) with: `+8` and `+0xc` two
/// heights (a thread's strip and the gap under it), `+0x10` the gap under a frame, `+0x14` the
/// width in pixels, `+0x20` a byte, `+0x24` the list of named colours. It runs `getTimerLimits`
/// (limit starting at -1.0, an `hkArray` of frame starts), takes the frame count `c` (the range's,
/// at most the first thread's frames left after the first frame), the height `((strip + gap) *
/// threads + frame gap) * c` and, with a margin of 0x20 pixels when `c` is above 1, the inner
/// width `width - margin`. The image (`(width + 1) * height * 4` bytes from `easyAlloc` with
/// the allocator at the router's `+0x10`, zeroed) address goes to `*image_out`, the height to
/// `*height_out` and the limit to `*limit_out` (when not null). For each frame `drawNumber`
/// writes the frame number (when `c` is above 1) and for each thread `drawStatistics` draws
/// the thread's strip, with a colour map shared by all of them. Then it draws the time scale: red
/// ticks (`0xff0000ff`) every `inner * 1000 / limit` pixels (the spacing `p` as a float; only
/// when above 5.0) offset by the padding's remainder in milliseconds, and green two-pixel ticks
/// (`0xff00ff00`) every `inner * 16666 / limit` pixels (when that is above 0). The colour map and
/// the starts array are then freed.
pub fn hk_monitor_stream_analyzer_write_statistics_details_to_texture(
    e: &mut Engine,
    trees: Ptr,
    layout: u32,
    analyzer: Ptr,
    image_out: u32,
    height_out: u32,
    limit_out: u32,
) {
    let (trees, analyzer) = (trees.addr(), analyzer.addr());
    let threads = e.mem.i32(trees + 4);
    if threads < 1 {
        return;
    }
    e.with_stack(0x1c, |e, block| {
        let block = block.addr();
        let (limit, starts, map) = (block, block + 4, block + 0x10);
        e.mem
            .set_u32(limit, x87_store32(x87_load32(e.mem.u32(LIMIT_UNSET))));
        e.mem.set_u32(starts, 0);
        e.mem.set_u32(starts + 4, 0);
        e.mem.set_u32(starts + 8, NOT_OWNED);
        hk_monitor_stream_analyzer_get_timer_limits(e, trees, layout, analyzer, limit, starts);
        let available = e
            .mem
            .i32(e.mem.u32(analyzer) + 4)
            .wrapping_sub(e.mem.i32(layout));
        let wanted = e.mem.i32(layout + 4);
        let count = if wanted <= available {
            wanted
        } else {
            available
        };
        let strip = e.mem.i32(layout + 0xc).wrapping_add(e.mem.i32(layout + 8));
        let end = e.mem.i32(layout).wrapping_add(count);
        let height = strip
            .wrapping_mul(threads)
            .wrapping_add(e.mem.i32(layout + 0x10))
            .wrapping_mul(count);
        let width = e.mem.i32(layout + 0x14);
        let margin = if count <= 1 { 0 } else { 0x20 };
        let inner = width.wrapping_sub(margin);
        let bytes = width.wrapping_add(1).wrapping_mul(height).wrapping_mul(4) as u32;
        let allocator = heap_allocator(e);
        let image = e.call(EASY_ALLOC, &args![allocator, bytes]).u32();
        e.mem.set_u32(image_out, image);
        e.mem.set_u32(height_out, height as u32);
        let image = e.mem.u32(image_out);
        e.call(MEM_SET, &args![image, 0u32, bytes]);
        if limit_out != 0 {
            e.mem
                .set_u32(limit_out, x87_store32(x87_load32(e.mem.u32(limit))));
        }
        e.mem.set_u32(map, 0);
        e.mem.set_u32(map + 4, 0);
        e.mem.set_u32(map + 8, 0xffff_ffff);
        let allocator = heap_allocator(e);
        e.call(COLOR_MAP_RESERVE, &args![map, allocator, 0u32]);
        let mut frame = e.mem.i32(layout);
        let mut row = height;
        if frame < end {
            loop {
                row = row.wrapping_sub(e.mem.i32(layout + 0x10));
                if count > 1 {
                    let width = e.mem.i32(layout + 0x14);
                    let image = e.mem.u32(image_out);
                    let at = image.wrapping_add(
                        (row.wrapping_sub(strip).wrapping_mul(width) as u32).wrapping_mul(4),
                    );
                    draw_number(e, at, frame, width as u32);
                }
                let mut thread = 0i32;
                let mut offset = 0u32;
                while thread < threads {
                    let slot = frame.wrapping_sub(e.mem.i32(layout)) as u32;
                    let width = e.mem.i32(layout + 0x14);
                    row = row.wrapping_sub(strip);
                    let start = x87_store32(x87_load32(
                        e.mem
                            .u32(e.mem.u32(starts).wrapping_add(slot.wrapping_mul(4))),
                    ));
                    let names = e.mem.u32(layout + 0x24);
                    let rows = e.mem.i32(layout + 8);
                    let image = e.mem.u32(image_out);
                    let out = image.wrapping_add(
                        (width.wrapping_mul(row).wrapping_add(margin) as u32).wrapping_mul(4),
                    );
                    let total = x87_store32(x87_load32(e.mem.u32(limit)));
                    let tree = e.mem.u32(
                        e.mem
                            .u32(trees)
                            .wrapping_add((thread as u32).wrapping_mul(4)),
                    );
                    let records = e.mem.u32(e.mem.u32(analyzer).wrapping_add(offset));
                    draw_statistics(
                        e,
                        records,
                        frame as u32,
                        tree + CHILDREN,
                        out,
                        rows,
                        names,
                        width as u32,
                        inner,
                        f32::from_bits(total),
                        f32::from_bits(start),
                        map,
                    );
                    offset = offset.wrapping_add(0xc);
                    thread += 1;
                }
                frame = frame.wrapping_add(1);
                if frame >= end {
                    break;
                }
            }
        }
        let limit_value = x87_load32(e.mem.u32(limit));
        let spacing = x87_store32(x87_div(f64::from(inner.wrapping_mul(1000)), limit_value));
        let padding = ftol(e, x87_load32(e.mem.u32(layout + 0x1c)));
        let remainder = padding.wrapping_rem(1000);
        let scaled = x87_mul(f64::from(remainder), e.mem.f64(THOUSANDTH));
        let spacing_value = x87_load32(spacing);
        let shift = ftol(e, x87_mul(scaled, spacing_value));
        if x87_load32(e.mem.u32(TICK_THRESHOLD)) < spacing_value {
            let mut tick = 0i32;
            let mut at = margin.wrapping_sub(shift);
            if at < e.mem.i32(layout + 0x14) {
                loop {
                    if at >= 0 {
                        let mut line = 0i32;
                        while line < height {
                            let width = e.mem.i32(layout + 0x14);
                            let image = e.mem.u32(image_out);
                            let pixel = width.wrapping_mul(line).wrapping_add(at) as u32;
                            e.mem
                                .set_u32(image.wrapping_add(pixel.wrapping_mul(4)), 0xff00_00ff);
                            line += 1;
                        }
                    }
                    tick += 1;
                    let after = ftol(e, x87_mul(f64::from(tick), spacing_value));
                    at = after.wrapping_sub(shift).wrapping_add(margin);
                    if at >= e.mem.i32(layout + 0x14) {
                        break;
                    }
                }
            }
        }
        let frame_ticks = ftol(
            e,
            x87_div(f64::from(inner.wrapping_mul(0x411a)), limit_value),
        );
        if frame_ticks > 0 {
            let mut at = margin.wrapping_sub(shift);
            if at < e.mem.i32(layout + 0x14) {
                loop {
                    if at >= 0 {
                        let mut line = 0i32;
                        while line < height {
                            let width = e.mem.i32(layout + 0x14);
                            let image = e.mem.u32(image_out);
                            let pixel = width.wrapping_mul(line).wrapping_add(at) as u32;
                            let target = image.wrapping_add(pixel.wrapping_mul(4));
                            e.mem.set_u32(target, 0xff00_ff00);
                            e.mem.set_u32(target.wrapping_add(4), 0xff00_ff00);
                            line += 1;
                        }
                    }
                    at = at.wrapping_add(frame_ticks);
                    if at >= e.mem.i32(layout + 0x14) {
                        break;
                    }
                }
            }
        }
        if e.mem.u8(layout + 0x20) != 0 {
            // The exe walks the map's table here (skipping the unused entries) and does nothing
            // with what it finds.
            let table = e.mem.u32(map);
            let last = e.mem.i32(map + 8);
            let used =
                |e: &Engine, at: i32| e.mem.u32(table.wrapping_add((at as u32).wrapping_mul(8)));
            let mut at = 0i32;
            if last >= 0 {
                loop {
                    if used(e, at) != 0xffff_ffff {
                        break;
                    }
                    at += 1;
                    if at > last {
                        break;
                    }
                }
            }
            while at <= last {
                at += 1;
                if at > last {
                    break;
                }
                loop {
                    if used(e, at) != 0xffff_ffff {
                        break;
                    }
                    at += 1;
                    if at > last {
                        break;
                    }
                }
            }
        }
        let allocator = heap_allocator(e);
        e.call(COLOR_MAP_CLEAR, &args![map, allocator]);
        e.call(COLOR_MAP_DELETE, &args![map]);
        let allocator = heap_allocator(e);
        let capacity = e.mem.u32(starts + 8);
        e.mem.set_u32(starts + 4, 0);
        if (capacity as i32) >= 0 {
            let data = e.mem.u32(starts);
            e.vcall(
                allocator,
                0x10,
                &args![data, (capacity & CAPACITY_MASK).wrapping_mul(4)],
            );
        }
    });
}

// Translated from 00e0e470 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMonitorStreamAnalyzer::writeStatisticsDetailsToTga` (Xbox PDB), cdecl `(trees, layout,
/// analyzer, stream, limit_out)`: runs `writeStatisticsDetailsToTexture` with a local image
/// pointer and height (both 0 at first) and `limit_out`; when an image came out, `saveToTGA`
/// writes it to `stream` (`layout[+0x14]` wide, the height from the call) and the image is freed
/// (`easyFree`, the router's allocator at `+0x10`).
pub fn hk_monitor_stream_analyzer_write_statistics_details_to_tga(
    e: &mut Engine,
    trees: Ptr,
    layout: u32,
    analyzer: Ptr,
    stream: u32,
    limit_out: u32,
) {
    e.with_stack(8, |e, locals| {
        let (image_slot, height_slot) = (locals.addr(), locals.addr() + 4);
        e.mem.set_u32(image_slot, 0);
        e.mem.set_u32(height_slot, 0);
        hk_monitor_stream_analyzer_write_statistics_details_to_texture(
            e,
            trees,
            layout,
            analyzer,
            image_slot,
            height_slot,
            limit_out,
        );
        let image = e.mem.u32(image_slot);
        if image != 0 {
            let (width, height) = (e.mem.u32(layout + 0x14), e.mem.u32(height_slot));
            save_to_tga(e, image, stream, width, height);
            let allocator = heap_allocator(e);
            e.call(EASY_FREE, &args![allocator, image]);
        }
    });
}

/// The inlined "append a record": the router's allocator (`+0x10`) is asked for, room is made
/// with `00c90940` when the array's size equals its capacity (element size 12), then the three
/// words are stored at the end (not when that address is 0) and the size grows.
fn push_words(e: &mut Engine, array: u32, words: [u32; 3]) {
    let allocator = heap_allocator(e);
    reserve_one_more(e, allocator, array, 12);
    let element = e
        .mem
        .u32(array)
        .wrapping_add(e.mem.u32(array + 4).wrapping_mul(12));
    if element != 0 {
        for (slot, word) in words.into_iter().enumerate() {
            e.mem.set_u32(element + 4 * slot as u32, word);
        }
    }
    let size = e.mem.u32(array + 4);
    e.mem.set_u32(array + 4, size.wrapping_add(1));
}

/// [`push_words`] with the three words of the record at `source`, which are only read when the
/// element's address is not 0.
fn push_copy(e: &mut Engine, array: u32, source: u32) {
    let allocator = heap_allocator(e);
    reserve_one_more(e, allocator, array, 12);
    let element = e
        .mem
        .u32(array)
        .wrapping_add(e.mem.u32(array + 4).wrapping_mul(12));
    if element != 0 {
        for slot in 0..3u32 {
            let word = e.mem.u32(source + 4 * slot);
            e.mem.set_u32(element + 4 * slot, word);
        }
    }
    let size = e.mem.u32(array + 4);
    e.mem.set_u32(array + 4, size.wrapping_add(1));
}

/// The record on top of a stack of 12-byte records.
fn top_record(e: &Engine, array: u32) -> u32 {
    e.mem
        .u32(array)
        .wrapping_add(e.mem.u32(array + 4).wrapping_mul(12))
        .wrapping_sub(12)
}

/// Pops both stacks of `makeStatisticsTreeForSingleFrame`: the second one's size shrinks (and
/// `_reserve` asks for room for the new size when the capacity is below it, which it never
/// is), the first one's size shrinks and the new size of the second is stored.
fn pop_timer(e: &mut Engine, stack_a: u32, stack_b: u32) {
    let wanted = e.mem.i32(stack_a + 4).wrapping_sub(1) as u32;
    let size = e.mem.u32(stack_b + 4);
    e.mem.set_u32(stack_b + 4, size.wrapping_sub(1));
    let allocator = heap_allocator(e);
    reserve_at_least(e, allocator, stack_a, wanted, 12);
    e.mem.set_u32(stack_a + 4, wanted);
}

// Translated from 00e0e630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMonitorStreamAnalyzer::makeStatisticsTreeForSingleFrame` (Xbox PDB), cdecl `(begin, end,
/// record, first_word, flag)`: builds the statistics tree of one thread's frame from the
/// monitor stream commands in `[begin, end)`. The root is a node from the router's allocator
/// at `+0x10` (slot 4, 0x58 bytes; `00e0d000` with no parent, `first_word` as the name and
/// kind 1; null when none comes). Two stacks of 12-byte records (inline room for 16 each,
/// capacity word `0x80000010`) follow the open timers: the first has the records themselves,
/// the second only where the timers began (`E` and `l` pop both). Each command word is the
/// address of a tag string (a word below 10 is skipped alone): the tag character picks the
/// handler, any other one ends the function (as does a handler's failed check):
/// `T` makes a child (`createNewNode`, the name from byte 2 of the tag string, kind 0, searching
/// when the low byte of `flag` is not 0) and pushes the record on both stacks, 12 bytes;
/// `E` (needs a pushed record and, when the name is not empty, one whose name matches it with
/// `strCmp`) adds the timers between the top record and this one to the node (`00e0c5e0`),
/// goes up to the parent and pops, 12 bytes; `S` adds the timers, goes up and makes a sibling
/// (the top record becomes this record), 12 bytes; `l` (needs two records, the top one a `S`)
/// does what `E` does twice, 12 bytes; `L` pushes the record on both stacks, makes a child
/// and under it a second child named by word 3 (pushing that on both stacks as a record of
/// word 3, 1, 2), 16 bytes; `O` pushes the record, makes a child named by word 3 itself (no
/// skipping of the tag bytes), pushes it on the other stack, 16 bytes; `M` adds the float of
/// word 1 to the value 0 of a kind 2 child (and counts it), 8 bytes; `P` goes down into a kind
/// 1 child, 4 bytes; `p` goes up (ending the function without a parent), 4 bytes; `N` skips
/// 4 bytes; `m` skips 12. `F` does not move on, so the loop handles the same command for ever
/// (as the exe's does). The stacks are freed at the end and the root returned.
pub fn hk_monitor_stream_analyzer_make_statistics_tree_for_single_frame(
    e: &mut Engine,
    begin: u32,
    end: u32,
    record: u32,
    first_word: u32,
    flag: u32,
) -> u32 {
    let search = u8::from(flag as u8 != 0);
    e.with_stack(0x1c8, |e, frame| {
        let frame = frame.addr();
        let (stack_a, buffer_a, stack_b, buffer_b) =
            (frame + 0x24, frame + 0x30, frame + 0xf0, frame + 0xfc);
        let allocator = heap_allocator(e);
        let block = e.vcall(allocator, 4, &args![NODE_SIZE]).u32();
        let root = if block == 0 {
            0
        } else {
            fn_00e0d000(e, Ptr::new(block), Ptr::new(0), first_word, 1).addr()
        };
        let mut node = root;
        e.mem.set_u32(stack_b, buffer_b);
        e.mem.set_u32(stack_b + 4, 0);
        e.mem.set_u32(stack_b + 8, 0x8000_0010);
        e.mem.set_u32(stack_a, buffer_a);
        e.mem.set_u32(stack_a + 4, 0);
        e.mem.set_u32(stack_a + 8, 0x8000_0010);
        let mut at = begin;
        'commands: while at < end {
            let word = e.mem.u32(at);
            if word < 10 {
                at = at.wrapping_add(4);
                continue;
            }
            let tag = e.mem.u8(word);
            match tag {
                b'T' => {
                    node = create_new_node(e, node, word + 2, 0, search);
                    push_copy(e, stack_b, at);
                    push_copy(e, stack_a, at);
                    at = at.wrapping_add(0xc);
                }
                b'E' => {
                    if e.mem.u32(stack_b + 4) == 0 {
                        break 'commands;
                    }
                    let name = word + 2;
                    let top = top_record(e, stack_b);
                    if e.mem.u8(name) != 0 {
                        let top_name = e.mem.u32(top) + 2;
                        if e.call(STR_CMP, &args![top_name, name]).u32() != 0 {
                            break 'commands;
                        }
                    }
                    hk_monitor_stream_analyzer_node_set_timers(
                        e,
                        Ptr::new(node),
                        Ptr::new(record),
                        Ptr::new(top),
                        Ptr::new(at),
                    );
                    node = e.mem.u32(node + 0x48);
                    pop_timer(e, stack_a, stack_b);
                    at = at.wrapping_add(0xc);
                }
                b'L' => {
                    push_copy(e, stack_b, at);
                    push_copy(e, stack_a, at);
                    let first = create_new_node(e, node, word + 2, 0, search);
                    let (word1, word2, word3) =
                        (e.mem.u32(at + 4), e.mem.u32(at + 8), e.mem.u32(at + 0xc));
                    e.mem.set_u32(frame + 0x1bc, word);
                    push_words(e, stack_b, [word3, word1, word2]);
                    push_words(e, stack_a, [word3, word1, word2]);
                    node = create_new_node(e, first, word3 + 2, 0, search);
                    at = at.wrapping_add(0x10);
                }
                b'O' => {
                    push_copy(e, stack_b, at);
                    let name = e.mem.u32(at + 0xc);
                    node = create_new_node(e, node, name, 0, search);
                    push_copy(e, stack_a, at);
                    at = at.wrapping_add(0x10);
                }
                b'M' => {
                    let value = create_new_node(e, node, word + 2, 2, search);
                    let sum = x87_add(x87_load32(e.mem.u32(at + 4)), x87_m32(e.mem.u32(value)));
                    let count = e.mem.u16(value + COUNTS);
                    e.mem.set_u16(value + COUNTS, count.wrapping_add(1));
                    e.mem.set_u32(value, x87_store32(sum));
                    at = at.wrapping_add(8);
                }
                b'S' => {
                    if e.mem.u32(stack_b + 4) == 0 {
                        break 'commands;
                    }
                    let top = top_record(e, stack_b);
                    hk_monitor_stream_analyzer_node_set_timers(
                        e,
                        Ptr::new(node),
                        Ptr::new(record),
                        Ptr::new(top),
                        Ptr::new(at),
                    );
                    node = e.mem.u32(node + 0x48);
                    node = create_new_node(e, node, word + 2, 0, search);
                    for slot in 0..3u32 {
                        let copied = e.mem.u32(at + 4 * slot);
                        e.mem.set_u32(top + 4 * slot, copied);
                    }
                    at = at.wrapping_add(0xc);
                }
                b'l' => {
                    if e.mem.i32(stack_b + 4) <= 1 {
                        break 'commands;
                    }
                    let top = top_record(e, stack_b);
                    if e.mem.u8(e.mem.u32(top)) != b'S' {
                        break 'commands;
                    }
                    hk_monitor_stream_analyzer_node_set_timers(
                        e,
                        Ptr::new(node),
                        Ptr::new(record),
                        Ptr::new(top),
                        Ptr::new(at),
                    );
                    node = e.mem.u32(node + 0x48);
                    pop_timer(e, stack_a, stack_b);
                    let top = top_record(e, stack_b);
                    hk_monitor_stream_analyzer_node_set_timers(
                        e,
                        Ptr::new(node),
                        Ptr::new(record),
                        Ptr::new(top),
                        Ptr::new(at),
                    );
                    node = e.mem.u32(node + 0x48);
                    pop_timer(e, stack_a, stack_b);
                    at = at.wrapping_add(0xc);
                }
                b'P' => {
                    node = create_new_node(e, node, word + 2, 1, search);
                    at = at.wrapping_add(4);
                }
                b'p' => {
                    node = e.mem.u32(node + 0x48);
                    if node == 0 {
                        break 'commands;
                    }
                    at = at.wrapping_add(4);
                }
                b'N' => at = at.wrapping_add(4),
                b'm' => at = at.wrapping_add(0xc),
                // `F`: the exe's jump goes straight back to the loop test without moving on.
                b'F' => {}
                _ => break 'commands,
            }
        }
        let allocator = heap_allocator(e);
        let capacity = e.mem.u32(stack_a + 8);
        e.mem.set_u32(stack_a + 4, 0);
        if (capacity as i32) >= 0 {
            let data = e.mem.u32(stack_a);
            e.vcall(
                allocator,
                0x10,
                &args![data, (capacity & CAPACITY_MASK).wrapping_mul(12)],
            );
        }
        e.mem.set_u32(stack_a, 0);
        e.mem.set_u32(stack_a + 8, NOT_OWNED);
        let allocator = heap_allocator(e);
        e.mem.set_u32(stack_b + 4, 0);
        let capacity = e.mem.u32(stack_b + 8);
        if (capacity as i32) >= 0 {
            let data = e.mem.u32(stack_b);
            e.vcall(
                allocator,
                0x10,
                &args![data, (capacity & CAPACITY_MASK).wrapping_mul(12)],
            );
        }
        root
    })
}

/// The strings and constants of `writeStatisticsDetails`: the version string (`010cf830`), the
/// root's name `010eac88`, the name of the combined nodes (`01011584`), the name `Unknown`
/// (`0106f148`), the float `1000.0` (`01013974`), the double `20.0` (`0102fc70`), the float
/// `01013ea4` that is the by-type threshold, and the CRT's `_memset` (`00ec61c0`, cdecl).
const HAVOK_VERSION: u32 = 0x010c_f830;
const ROOT_NAME: u32 = 0x010e_ac88;
const COMBINED_NAME: u32 = 0x0101_1584;
const UNKNOWN_NAME: u32 = 0x0106_f148;
const DEFAULT_TOTAL: u32 = 0x0101_3974;
const BAR_SCALE: u32 = 0x0102_fc70;
const BY_TYPE_THRESHOLD: u32 = 0x0101_3ea4;
const CRT_MEMSET: u32 = 0x00ec_61c0;

// Translated from 00e0fec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMonitorStreamAnalyzer::writeStatisticsDetails` (Xbox PDB), cdecl `(stream, trees,
/// threads, spus, flags, name, combine)` (`combine` a byte). `trees` is the address of an `hkArray`
/// of one tree per thread. It prints `Havok version: %s` (`010eac8c`, version string `010cf830`),
/// adds up all of the first tree's frames in a node on the stack (`00e0d880`) and takes the
/// total of the values of its children called `name` (`00e0cb80`, `00e0cbe0`, each times 1 /
/// number of frames) as the total time, 1000.0 when that is not above 0.0. Then, by the bits of
/// `flags`: bit 0 prints the headings (`01095188`, `010eac64`, `010eac40`, `010eac64`,
/// `010eac24`) and `hkWriteRec` of the sum of the frames with the scale 1 / frames; bit 1 prints
/// an ascii-art line for each frame of the first tree (`01095188`, `010eac64`, `010eac00`,
/// `010eac64`, `010eabe0` first): the bar is built in 0x28 characters of the stack (`memSet`
/// with spaces, tabs every 4 columns, a `#`, then spaces and tabs) at the column `value * 20 /
/// total` clamped to 0..0x27, then `%i %-12s %f` of the frame, the last child's name (or
/// `Unknown`) and the frame's sum; bit 2 prints a summary of every frame and thread (headings
/// `01012648`, `010eaba4`, `Summary Frame:%i Thread:%i` `010eab58`, the frame name `010ce5cc`,
/// `hkWriteRec` with scale 1), and with bit 3 `printStatisticsByType` (`00e0fc50`, threshold
/// `01013ea4`); when there are several threads and `combine` is not 0 each frame instead gets
/// one combined tree (a bump-allocated array of nodes from the router's first word, `00e0d280`
/// merging the other threads into the first) shown with `showCombinedThreadSummaryForSingleFrame`
/// (config: column 0x10, indent 4 and 2, no marks, no tabs; `Summary Frame:%i` `010eab84`);
/// bit 4 prints `Details Frame-%i Thread:%i` (`010eab00`) or `Spu:%i` (`010eaad8`) and the
/// `hkWriteRec` of each frame of each thread.
pub fn hk_monitor_stream_analyzer_write_statistics_details(
    e: &mut Engine,
    stream: u32,
    trees: Ptr,
    threads: i32,
    spus: i32,
    flags: u32,
    name: u32,
    combine: u8,
) {
    let trees = trees.addr();
    e.with_stack(0x80, |e, frame| {
        let frame = frame.addr();
        let (scalar, local, saved, total_slot, scratch) =
            (frame, frame + 4, frame + 0x10, frame + 0x14, frame + 0x18);
        let (ratio_slot, root_slot, name_slot, node) =
            (frame + 0x1c, frame + 0x20, frame + 0x24, frame + 0x28);
        e.call(PRINTF, &args![stream, 0x010e_ac8cu32, HAVOK_VERSION]);
        e.mem.set_u32(total_slot, 0);
        let first_tree = e.mem.u32(e.mem.u32(trees));
        let root_children = first_tree + CHILDREN;
        e.mem.set_u32(root_slot, root_children);
        fn_00e0d000(e, Ptr::new(node), Ptr::new(0), ROOT_NAME, 1);
        let frames = e.mem.u32(root_children + 4);
        e.mem.set_u32(scratch, frames);
        let inverse = x87_store32(x87_div(1.0, f64::from(frames as i32)));
        e.mem.set_u32(scalar, inverse);
        if (frames as i32) > 0 {
            let mut index = 0u32;
            loop {
                let data = e.mem.u32(root_children);
                let frame_node = e.mem.u32(data.wrapping_add(index.wrapping_mul(4)));
                fn_00e0d880(e, node, frame_node);
                index += 1;
                if (index as i32) >= e.mem.i32(root_children + 4) {
                    break;
                }
            }
        }
        let mut positive = false;
        let mut child = fn_00e0cb80(e, node, name, 1);
        if child != 0 {
            loop {
                let product = x87_mul(x87_load32(e.mem.u32(child)), x87_m32(e.mem.u32(scalar)));
                let sum = x87_add(product, x87_m32(e.mem.u32(total_slot)));
                e.mem.set_u32(total_slot, x87_store32(sum));
                child = fn_00e0cbe0(e, node, child, name);
                if child == 0 {
                    break;
                }
            }
            positive = 0.0 < x87_m32(e.mem.u32(total_slot));
        }
        if !positive {
            let total = x87_store32(x87_load32(e.mem.u32(DEFAULT_TOTAL)));
            e.mem.set_u32(total_slot, total);
        }
        if flags & 1 != 0 {
            for format in [
                0x0109_5188u32,
                0x010e_ac64,
                0x010e_ac40,
                0x010e_ac64,
                0x010e_ac24,
            ] {
                e.call(PRINTF, &args![stream, format]);
            }
            let scale = x87_quiet(f32::from_bits(e.mem.u32(scalar)));
            hk_write_rec(e, stream, Ptr::new(node), 0, scale);
        }
        hk_monitor_stream_analyzer_node_destructor(e, Ptr::new(node));
        if flags & 2 != 0 {
            for format in [
                0x0109_5188u32,
                0x010e_ac64,
                0x010e_ac00,
                0x010e_ac64,
                0x010e_abe0,
            ] {
                e.call(PRINTF, &args![stream, format]);
            }
            e.call(MEM_SET, &args![node, 0x20u32, 0x28u32]);
            e.mem.set_u8(node + 0x28, 0);
            e.mem.set_u32(scratch, 0);
            if e.mem.i32(root_children + 4) > 0 {
                let mut index = 0u32;
                loop {
                    let data = e.mem.u32(root_children);
                    let frame_node = e.mem.u32(data.wrapping_add(index.wrapping_mul(4)));
                    if e.mem.u32(frame_node + CHILDREN + 4) != 0 {
                        e.mem.set_u32(scalar, 0);
                        e.mem.set_u32(name_slot, UNKNOWN_NAME);
                        let mut child = fn_00e0cb80(e, frame_node, name, 1);
                        while child != 0 {
                            let sum =
                                x87_add(x87_load32(e.mem.u32(child)), x87_m32(e.mem.u32(scalar)));
                            let child_name = e.mem.u32(child + NODE_NAME);
                            e.mem.set_u32(name_slot, child_name);
                            e.mem.set_u32(scalar, x87_store32(sum));
                            child = fn_00e0cbe0(e, frame_node, child, name);
                        }
                        let ratio = x87_div(
                            x87_mul(x87_load32(e.mem.u32(scalar)), e.mem.f64(BAR_SCALE)),
                            x87_m32(e.mem.u32(total_slot)),
                        );
                        e.mem.set_u32(ratio_slot, x87_store32(ratio));
                        let bar = ftol(e, x87_load32(e.mem.u32(ratio_slot)));
                        let bar = if bar < 0 {
                            0
                        } else if bar >= 0x28 {
                            0x27
                        } else {
                            bar
                        };
                        let mut position = node;
                        let mut written = 0i32;
                        if bar > 4 {
                            let mut stop = 4i32;
                            loop {
                                e.mem.set_u8(position, 9);
                                stop += 4;
                                position += 1;
                                written += 4;
                                if stop >= bar {
                                    break;
                                }
                            }
                        }
                        let mut column = written;
                        if written < bar {
                            let spaces = bar - written;
                            e.call(CRT_MEMSET, &args![position, 0x20u32, spaces as u32]);
                            position = position.wrapping_add(spaces as u32);
                            column = spaces + written;
                        }
                        e.mem.set_u8(position, 0x23);
                        column += 1;
                        written += 4;
                        position += 1;
                        if column < written {
                            let spaces = written - column;
                            e.call(CRT_MEMSET, &args![position, 0x20u32, spaces as u32]);
                            position = position.wrapping_add(spaces as u32);
                        }
                        if written < 0x28 {
                            let tabs = ((0x27 - written) as u32 >> 2) + 1;
                            e.call(CRT_MEMSET, &args![position, 9u32, tabs]);
                            position = position.wrapping_add(tabs);
                        }
                        e.mem.set_u8(position, 0);
                        e.call(PRINTF, &args![stream, node]);
                        let shown = x87_load32(e.mem.u32(scalar));
                        let last_name = e.mem.u32(name_slot);
                        e.call(
                            PRINTF,
                            &args![stream, 0x010e_abd0u32, index, last_name, shown],
                        );
                    }
                    index += 1;
                    e.mem.set_u32(scratch, index);
                    if (index as i32) >= e.mem.i32(root_children + 4) {
                        break;
                    }
                }
            }
        }
        if flags & 4 != 0 {
            let thread_count = e.mem.i32(trees + 4);
            if thread_count != 1 && combine != 0 {
                let tree_zero = e.mem.u32(e.mem.u32(trees));
                if e.mem.i32(tree_zero + CHILDREN + 4) > 0 {
                    e.mem.set_u32(ratio_slot, flags & 8);
                    let mut frame_index = 0u32;
                    e.mem.set_u32(scalar, 0);
                    loop {
                        e.call(PRINTF, &args![stream, 0x0101_2648u32]);
                        e.call(PRINTF, &args![stream, 0x010e_aba4u32]);
                        e.call(PRINTF, &args![stream, 0x010e_ab84u32, frame_index]);
                        e.call(PRINTF, &args![stream, 0x010e_aba4u32]);
                        let count = e.mem.u32(trees + 4);
                        e.mem.set_u32(local, 0);
                        e.mem.set_u32(local + 4, 0);
                        e.mem.set_u32(local + 8, NOT_OWNED);
                        let router = e.call(MEMORY_ROUTER, &args![]).u32();
                        let bump = e.mem.u32(router);
                        e.mem.set_u32(
                            router,
                            (count.wrapping_mul(4).wrapping_add(0x7f) & 0xffff_ff80)
                                .wrapping_add(bump),
                        );
                        e.mem.set_u32(local + 8, count | NOT_OWNED);
                        let wanted = e.mem.u32(trees + 4);
                        e.mem.set_u32(local, bump);
                        e.mem.set_u32(saved, bump);
                        let router = e.call(MEMORY_ROUTER, &args![]).u32();
                        let allocator = e.mem.u32(router + 0x10);
                        reserve_at_least(e, allocator, local, wanted, 4);
                        e.mem.set_u32(local + 4, wanted);
                        let mut thread = 0u32;
                        if e.mem.i32(trees + 4) > 0 {
                            loop {
                                let allocator = heap_allocator(e);
                                let block = e.vcall(allocator, 4, &args![NODE_SIZE]).u32();
                                let created = if block == 0 {
                                    0
                                } else {
                                    fn_00e0d000(e, Ptr::new(block), Ptr::new(0), COMBINED_NAME, 1)
                                        .addr()
                                };
                                let slots = e.mem.u32(local);
                                e.mem
                                    .set_u32(slots.wrapping_add(thread.wrapping_mul(4)), created);
                                let tree = e
                                    .mem
                                    .u32(e.mem.u32(trees).wrapping_add(thread.wrapping_mul(4)));
                                if e.mem.i32(tree + CHILDREN + 4) > frame_index as i32 {
                                    let source = e.mem.u32(
                                        e.mem
                                            .u32(tree + CHILDREN)
                                            .wrapping_add(frame_index.wrapping_mul(4)),
                                    );
                                    let slots = e.mem.u32(local);
                                    let dest =
                                        e.mem.u32(slots.wrapping_add(thread.wrapping_mul(4)));
                                    fn_00e0d880(e, dest, source);
                                }
                                thread += 1;
                                if (thread as i32) >= e.mem.i32(trees + 4) {
                                    break;
                                }
                            }
                        }
                        let config = node;
                        e.mem.set_u32(config + 8, 4);
                        e.mem.set_u32(config + 0xc, 2);
                        e.mem.set_u32(config + 4, 0x10);
                        e.mem.set_u8(config + 0x10, 0);
                        e.mem.set_u8(config + 0x1a, 0);
                        let mut other = 1u32;
                        if e.mem.i32(local + 4) > 1 {
                            loop {
                                let slots = e.mem.u32(local);
                                let first = e.mem.u32(slots);
                                let next = e.mem.u32(slots.wrapping_add(other.wrapping_mul(4)));
                                fn_00e0d280(e, Ptr::new(first), next, other as i32, 0, 0.0);
                                other += 1;
                                if (other as i32) >= e.mem.i32(local + 4) {
                                    break;
                                }
                            }
                        }
                        let first = e.mem.u32(e.mem.u32(local));
                        hk_monitor_stream_analyzer_show_combined_thread_summary_for_single_frame(
                            e, first, threads, spus, stream, config,
                        );
                        if e.mem.u32(ratio_slot) != 0 {
                            let first = e.mem.u32(e.mem.u32(local));
                            let threshold = x87_quiet(f32::from_bits(e.mem.u32(BY_TYPE_THRESHOLD)));
                            print_statistics_by_type(e, stream, Ptr::new(first), threshold);
                        }
                        let mut index = 0u32;
                        if e.mem.i32(trees + 4) > 0 {
                            loop {
                                let slots = e.mem.u32(local);
                                let combined = e.mem.u32(slots.wrapping_add(index.wrapping_mul(4)));
                                if combined != 0 {
                                    hk_monitor_stream_analyzer_node_destructor(
                                        e,
                                        Ptr::new(combined),
                                    );
                                    let allocator = heap_allocator(e);
                                    e.vcall(allocator, 8, &args![combined, NODE_SIZE]);
                                }
                                index += 1;
                                if (index as i32) >= e.mem.i32(trees + 4) {
                                    break;
                                }
                            }
                        }
                        let bump = e.mem.u32(saved);
                        if bump == e.mem.u32(local) {
                            e.mem.set_u32(local + 4, 0);
                        }
                        let router = e.call(MEMORY_ROUTER, &args![]).u32();
                        e.mem.set_u32(router, bump);
                        let allocator = heap_allocator(e);
                        let capacity = e.mem.u32(local + 8);
                        e.mem.set_u32(local + 4, 0);
                        if (capacity as i32) >= 0 {
                            let data = e.mem.u32(local);
                            e.vcall(
                                allocator,
                                0x10,
                                &args![data, (capacity & CAPACITY_MASK).wrapping_mul(4)],
                            );
                        }
                        frame_index += 1;
                        let tree_zero = e.mem.u32(e.mem.u32(trees));
                        e.mem.set_u32(local, 0);
                        e.mem.set_u32(local + 8, NOT_OWNED);
                        e.mem.set_u32(scalar, frame_index);
                        if (frame_index as i32) >= e.mem.i32(tree_zero + CHILDREN + 4) {
                            break;
                        }
                    }
                }
            } else {
                let tree_zero = e.mem.u32(e.mem.u32(trees));
                if e.mem.i32(tree_zero + CHILDREN + 4) > 0 {
                    let mut frame_index = 0u32;
                    loop {
                        let mut thread = 0u32;
                        if e.mem.i32(trees + 4) > 0 {
                            loop {
                                let tree = e
                                    .mem
                                    .u32(e.mem.u32(trees).wrapping_add(thread.wrapping_mul(4)));
                                if (frame_index as i32) < e.mem.i32(tree + CHILDREN + 4) {
                                    fn_00e0d000(e, Ptr::new(node), Ptr::new(0), ROOT_NAME, 1);
                                    let source = e.mem.u32(
                                        e.mem
                                            .u32(tree + CHILDREN)
                                            .wrapping_add(frame_index.wrapping_mul(4)),
                                    );
                                    fn_00e0d880(e, node, source);
                                    e.call(PRINTF, &args![stream, 0x0101_2648u32]);
                                    e.call(PRINTF, &args![stream, 0x010e_aba4u32]);
                                    e.call(
                                        PRINTF,
                                        &args![stream, 0x010e_ab58u32, frame_index, thread],
                                    );
                                    e.call(PRINTF, &args![stream, 0x010e_aba4u32]);
                                    let frame_name = e.mem.u32(
                                        e.mem.u32(
                                            e.mem
                                                .u32(tree + CHILDREN)
                                                .wrapping_add(frame_index.wrapping_mul(4)),
                                        ) + NODE_NAME,
                                    );
                                    e.call(PRINTF, &args![stream, 0x010c_e5ccu32, frame_name]);
                                    hk_write_rec(e, stream, Ptr::new(node), 0, 1.0);
                                    if flags & 8 != 0 {
                                        let threshold =
                                            x87_quiet(f32::from_bits(e.mem.u32(BY_TYPE_THRESHOLD)));
                                        print_statistics_by_type(
                                            e,
                                            stream,
                                            Ptr::new(node),
                                            threshold,
                                        );
                                    }
                                    hk_monitor_stream_analyzer_node_destructor(e, Ptr::new(node));
                                }
                                thread += 1;
                                if (thread as i32) >= e.mem.i32(trees + 4) {
                                    break;
                                }
                            }
                        }
                        frame_index += 1;
                        let tree_zero = e.mem.u32(e.mem.u32(trees));
                        if (frame_index as i32) >= e.mem.i32(tree_zero + CHILDREN + 4) {
                            break;
                        }
                    }
                }
            }
        }
        if flags & 0x10 != 0 {
            let tree_zero = e.mem.u32(e.mem.u32(trees));
            if e.mem.i32(tree_zero + CHILDREN + 4) > 0 {
                let mut frame_index = 0u32;
                loop {
                    if e.mem.i32(trees + 4) > 0 {
                        e.mem.set_u32(scalar, threads.wrapping_neg() as u32);
                        let mut thread = 0i32;
                        loop {
                            let tree = e.mem.u32(
                                e.mem
                                    .u32(trees)
                                    .wrapping_add((thread as u32).wrapping_mul(4)),
                            );
                            if (frame_index as i32) < e.mem.i32(tree + CHILDREN + 4) {
                                e.call(PRINTF, &args![stream, 0x0109_5188u32]);
                                e.call(PRINTF, &args![stream, 0x010e_ab2cu32]);
                                if thread < threads {
                                    e.call(
                                        PRINTF,
                                        &args![stream, 0x010e_ab00u32, frame_index, thread as u32],
                                    );
                                } else {
                                    let shifted = e.mem.u32(scalar);
                                    e.call(
                                        PRINTF,
                                        &args![stream, 0x010e_aad8u32, frame_index, shifted],
                                    );
                                }
                                e.call(PRINTF, &args![stream, 0x010e_ab2cu32]);
                                let frame_node = e.mem.u32(
                                    e.mem
                                        .u32(tree + CHILDREN)
                                        .wrapping_add(frame_index.wrapping_mul(4)),
                                );
                                let frame_name = e.mem.u32(frame_node + NODE_NAME);
                                e.call(PRINTF, &args![stream, 0x010c_e5ccu32, frame_name]);
                                let frame_node = e.mem.u32(
                                    e.mem
                                        .u32(tree + CHILDREN)
                                        .wrapping_add(frame_index.wrapping_mul(4)),
                                );
                                hk_write_rec(e, stream, Ptr::new(frame_node), 0, 1.0);
                            }
                            let shifted = e.mem.u32(scalar);
                            e.mem.set_u32(scalar, shifted.wrapping_add(1));
                            thread += 1;
                            if thread >= e.mem.i32(trees + 4) {
                                break;
                            }
                        }
                    }
                    frame_index += 1;
                    let tree_zero = e.mem.u32(e.mem.u32(trees));
                    if (frame_index as i32) >= e.mem.i32(tree_zero + CHILDREN + 4) {
                        break;
                    }
                }
            }
        }
    });
}

// Translated from 00e0bf30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one stack word `extra`, `RET 4`; a call at `00dfdecd`
/// reaches it): runs `00cb0a70` on the member at `this + 8` (`thiscall`, one stack word,
/// `extra`) and answers `this`.
pub fn fn_00e0bf30(e: &mut Engine, this: Ptr, extra: u32) -> Ptr {
    e.call(0x00cb_0a70, &args![this.addr().wrapping_add(8), extra]);
    this
}

// Translated from 00e0f700 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one stack word the function never reads, `RET 4`; a
/// call at `00dfdefd` reaches it): answers `this`.
pub fn fn_00e0f700(_e: &mut Engine, this: Ptr, _unused_1: u32) -> Ptr {
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e0bf30, fn_00e0bf30(Ptr, u32) -> Ptr),
        entry!(0x00e0f700, fn_00e0f700(Ptr, u32) -> Ptr),
        entry!(0x00e0bf20, hk_monitor_stream_string_map_string_map(Ptr) -> Ptr),
        entry!(0x00e0bfe0, print_single_node_values(u32, f32, Ptr)),
        entry!(0x00e0c150, save_to_tga(u32, u32, u32, u32) -> bool),
        entry!(0x00e0c1c0, fn_00e0c1c0(u32, u32, u32)),
        entry!(0x00e0c210, draw_number(u32, i32, u32)),
        entry!(0x00e0c320, hk_monitor_stream_frame_info_hk_monitor_stream_frame_info(Ptr) -> Ptr),
        entry!(0x00e0c350, fn_00e0c350(u32, u32, u32, u8)),
        entry!(
            0x00e0c5e0,
            hk_monitor_stream_analyzer_node_set_timers(Ptr, Ptr, Ptr, Ptr)
        ),
        entry!(0x00e0c690, fn_00e0c690(u32, u32, Ptr) -> bool),
        entry!(0x00e0c700, fn_00e0c700(Ptr, u32, f32)),
        entry!(0x00e0c860, hk_monitor_stream_analyzer_find_max_timer_name_indent(Ptr, u32, u32, u8) -> i32),
        entry!(0x00e0c8e0, hk_write_rec(u32, Ptr, u32, f32)),
        entry!(0x00e0cb80, fn_00e0cb80(u32, u32, u8) -> u32),
        entry!(0x00e0cbe0, fn_00e0cbe0(u32, u32, u32) -> u32),
        entry!(0x00e0cc40, fn_00e0cc40(u32, u32, f64, u32) -> u32),
        entry!(0x00e0cc90, fn_00e0cc90(Ptr, u32) -> u32),
        entry!(0x00e0ccf0, fn_00e0ccf0(Ptr)),
        entry!(0x00e0cd20, fn_00e0cd20(u32, u32, u32)),
        entry!(0x00e0ce00, fn_00e0ce00(Ptr, Ptr, u32, u32, Ptr) -> Ptr),
        entry!(
            0x00e0cec0,
            output_stats_for_frame(u32, u32, u32, f32, f32, u32)
        ),
        entry!(0x00e0d000, fn_00e0d000(Ptr, Ptr, u32, u32) -> Ptr),
        entry!(0x00e0d0a0, hk_monitor_stream_analyzer_node_destructor(Ptr)),
        entry!(0x00e0d100, create_new_node(u32, u32, u32, u8) -> u32),
        entry!(0x00e0d180, fn_00e0d180(u32, u32, i32, u32, f32)),
        entry!(0x00e0d280, fn_00e0d280(Ptr, u32, i32, u32, f32)),
        entry!(
            0x00e0d2c0,
            hk_monitor_stream_analyzer_show_combined_thread_summary_rec(
                u32, u32, i32, i32, i32, u32
            )
        ),
        entry!(
            0x00e0d640,
            hk_monitor_stream_analyzer_show_combined_thread_summary_for_single_frame(
                u32, i32, i32, u32, u32
            )
        ),
        entry!(0x00e0d880, fn_00e0d880(u32, u32)),
        entry!(
            0x00e0d980,
            hk_monitor_stream_color_table_add_color(Ptr, u32, u32)
        ),
        entry!(0x00e0da00, get_node_sampled_at_tick(u32, u32, u32, Ptr, u32, f32, f32) -> u32),
        entry!(
            0x00e0daf0,
            draw_statistics(u32, u32, u32, u32, i32, u32, u32, i32, f32, f32, u32)
        ),
        entry!(
            0x00e0dd30,
            hk_monitor_stream_analyzer_get_timer_limits(u32, u32, u32, u32, u32)
        ),
        entry!(
            0x00e0e0c0,
            hk_monitor_stream_analyzer_write_statistics_details_to_texture(
                Ptr, u32, Ptr, u32, u32, u32
            )
        ),
        entry!(
            0x00e0e470,
            hk_monitor_stream_analyzer_write_statistics_details_to_tga(Ptr, u32, Ptr, u32, u32)
        ),
        entry!(0x00e0e630, hk_monitor_stream_analyzer_make_statistics_tree_for_single_frame(u32, u32, u32, u32, u32) -> u32),
        entry!(0x00e0ee00, hk_monitor_stream_analyzer_make_statistics_tree_for_multiple_frames(Ptr, u32, u32) -> u32),
        entry!(
            0x00e0eef0,
            hk_monitor_stream_analyzer_write_raw_statistics(Ptr, u32)
        ),
        entry!(0x00e0f5e0, fn_00e0f5e0(Ptr, u32, u32)),
        entry!(0x00e0f6e0, fn_00e0f6e0(Ptr) -> Ptr),
        entry!(0x00e0f710, fn_00e0f710(Ptr, u32, u32)),
        entry!(0x00e0f880, hk_build_size_per_name(Ptr, u32)),
        entry!(0x00e0fc50, print_statistics_by_type(u32, Ptr, f32)),
        entry!(
            0x00e0fec0,
            hk_monitor_stream_analyzer_write_statistics_details(u32, Ptr, i32, i32, u32, u32, u8)
        ),
        entry!(
            0x00e10630,
            hk_monitor_stream_analyzer_write_statistics(Ptr, u32, u32)
        ),
        entry!(0x00e10820, fn_00e10820(u32, f64, u32) -> u32),
        entry!(0x00e108b0, indenter_indenter(Ptr) -> Ptr),
        entry!(
            0x00e10930,
            hk_monitor_stream_analyzer_print_by_type_info_destructor(Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn fn_00e0bf30_runs_00cb0a70_on_the_member_and_answers_this() {
        let mut e = Engine::new();
        e.register(0x00cb_0a70, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let answer = e.call(0x00e0_bf30, &args![0x1000u32, 0x55u32]).u32();
        assert_eq!(answer, 0x1000);
        let seen: Vec<_> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == 0x00cb_0a70)
            .cloned()
            .collect();
        assert_eq!(seen, vec![(0x00cb_0a70, vec![0x1008, 0x55])]);
    }

    #[test]
    fn fn_00e0f700_answers_this() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00e0_f700, &args![0x1000u32, 7u32]).u32(), 0x1000);
    }

    const ALLOCATE: u32 = 0x4500_0100;
    const FREE_BLOCK: u32 = 0x4500_0101;
    const FREE_DATA: u32 = 0x4500_0102;

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.clone().unwrap();
        log.into_iter()
            .filter(|call| call.0 == address)
            .map(|call| call.1)
            .collect()
    }

    fn double_words(value: f64) -> [u32; 2] {
        let bits = value.to_bits();
        [bits as u32, (bits >> 32) as u32]
    }

    /// Gives an `hkArray` a fresh, owned buffer of `capacity` elements (what `_reserve` does).
    fn grow(e: &mut Engine, array: u32, capacity: u32, element: u32) {
        let size = e.mem.u32(array + 4);
        let old = e.mem.u32(array);
        let new = e.mem.alloc(capacity.max(1) * element);
        if old != 0 && size != 0 {
            let bytes = e.mem.bytes(old, size * element);
            e.mem.write(new, &bytes);
        }
        e.mem.set_u32(array, new);
        e.mem.set_u32(array + 8, capacity);
    }

    /// An engine with the exe's data area mapped, a memory router whose three allocators are
    /// the same object (slot 4 allocates on the heap, slots 8 and `0x10` do nothing), and
    /// doubles for the array growth, the string functions and `printf`.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.put_vtable(0x4400_0000, &[0, ALLOCATE, FREE_BLOCK, 0, FREE_DATA]);
        let allocator = e.mem.alloc(8);
        e.mem.set_u32(allocator, 0x4400_0000);
        let router = e.mem.alloc(0x24);
        for offset in [0xc, 0x10, 0x14] {
            e.mem.set_u32(router + offset, allocator);
        }
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        e.register(ALLOCATE, |e, a| Ret {
            eax: e.mem.alloc(a[1]),
            ..Ret::default()
        });
        e.register(FREE_BLOCK, |_, _| Ret::default());
        e.register(FREE_DATA, |_, _| Ret::default());
        e.register(PRINTF, |_, _| Ret::default());
        e.register(SNPRINTF, |_, _| Ret::default());
        e.register(MEM_SET, |e, a| {
            for at in 0..a[2] {
                e.mem.set_u8(a[0] + at, a[1] as u8);
            }
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.register(ARRAY_RESERVE, |e, a| {
            grow(e, a[1], a[2], a[3]);
            Ret::default()
        });
        e.register(ARRAY_RESERVE_MORE, |e, a| {
            let capacity = e.mem.u32(a[1] + 8) & CAPACITY_MASK;
            grow(e, a[1], (capacity * 2).max(4), a[2]);
            Ret::default()
        });
        e.register(STR_CMP, |e, a| {
            let (first, second) = (e.mem.cstr(a[0]), e.mem.cstr(a[1]));
            Ret {
                eax: match first.cmp(&second) {
                    std::cmp::Ordering::Less => u32::MAX,
                    std::cmp::Ordering::Equal => 0,
                    std::cmp::Ordering::Greater => 1,
                },
                ..Ret::default()
            }
        });
        e.register(STR_CASE_CMP, |e, a| {
            let first = e.mem.cstr(a[0]).to_ascii_lowercase();
            let second = e.mem.cstr(a[1]).to_ascii_lowercase();
            Ret {
                eax: match first.cmp(&second) {
                    std::cmp::Ordering::Less => u32::MAX,
                    std::cmp::Ordering::Equal => 0,
                    std::cmp::Ordering::Greater => 1,
                },
                ..Ret::default()
            }
        });
        e.register(0x00dd_eff0, |_, _| Ret::default());
        e.register(STR_LEN, |e, a| Ret {
            eax: e.mem.cstr(a[0]).len() as u32,
            ..Ret::default()
        });
        e.register(CRT_MEMSET, |e, a| {
            for at in 0..a[2] {
                e.mem.set_u8(a[0] + at, a[1] as u8);
            }
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.register(FTOL, |_, a| {
            let value = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            Ret {
                eax: value.trunc() as i64 as u32,
                ..Ret::default()
            }
        });
        e.register(EASY_ALLOC, |e, a| Ret {
            eax: e.mem.alloc(a[1].max(8)),
            ..Ret::default()
        });
        for address in [
            EASY_FREE,
            STREAM_WRITE,
            STREAM_PUT_CHAR,
            STREAM_PUT_STRING,
            TEXT_STREAM_NEW,
            TEXT_STREAM_DELETE,
            COLOR_MAP_RESERVE,
            COLOR_MAP_CLEAR,
            COLOR_MAP_DELETE,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.mem.set_f64(HUNDRED, 100.0);
        e.mem.set_f32(TWO_POW_32, 4_294_967_296.0);
        e.mem.set_f64(TWO_POW_32_DOUBLE, 4_294_967_296.0);
        e.mem.set_f64(ONE_DOUBLE, 1.0);
        e.mem.set_f32(LIMIT_UNSET, -1.0);
        e.mem.set_f32(TICK_THRESHOLD, 5.0);
        e.mem.set_f64(THOUSANDTH, f64::from(0.001f32));
        e.mem.set_f64(THOUSAND, 1000.0);
        e.mem.set_f64(QUARTER, 0.25);
        e.mem.set_f64(SMOOTHING, f64::from(0.05f32));
        e.mem.set_f64(LIMIT_FLOOR_DOUBLE, 16666.0);
        e.mem.set_f32(LIMIT_FLOOR_FLOAT, 16666.0);
        e.mem.set_f32(START_UNSET, 1.0e30);
        e.mem.set_f32(SMOOTH_LIMIT, 20000.0);
        e.mem.set_f32(DEFAULT_TOTAL, 1000.0);
        e.mem.set_f64(BAR_SCALE, 20.0);
        e
    }

    fn router_of(e: &mut Engine) -> u32 {
        e.call(MEMORY_ROUTER, &args![]).u32()
    }

    fn text(e: &mut Engine, string: &str) -> u32 {
        let at = e.mem.alloc(string.len() as u32 + 1);
        e.mem.set_cstr(at, string.as_bytes());
        at
    }

    /// A statistics node with its values, counts and name and no children.
    fn node(e: &mut Engine, name: u32, values: [f32; 8], counts: [u16; 8]) -> u32 {
        let node = e.mem.alloc(NODE_SIZE);
        for (slot, value) in values.into_iter().enumerate() {
            e.mem.set_f32(node + 4 * slot as u32, value);
        }
        for (slot, count) in counts.into_iter().enumerate() {
            e.mem.set_u16(node + COUNTS + 2 * slot as u32, count);
        }
        e.mem.set_u32(node + NODE_NAME, name);
        e.mem.set_u32(node + CHILDREN + 8, NOT_OWNED);
        node
    }

    fn named(e: &mut Engine, name: &str) -> u32 {
        let name = text(e, name);
        node(e, name, [0.0; 8], [0; 8])
    }

    /// Appends `child` to `parent`'s children (in a buffer of 16) and sets its parent.
    fn adopt(e: &mut Engine, parent: u32, child: u32) {
        let mut data = e.mem.u32(parent + CHILDREN);
        if data == 0 {
            data = e.mem.alloc(64);
            e.mem.set_u32(parent + CHILDREN, data);
            e.mem.set_u32(parent + CHILDREN + 8, 16 | NOT_OWNED);
        }
        let size = e.mem.u32(parent + CHILDREN + 4);
        e.mem.set_u32(data + 4 * size, child);
        e.mem.set_u32(parent + CHILDREN + 4, size + 1);
        e.mem.set_u32(child + 0x48, parent);
    }

    #[test]
    fn the_string_map_constructor_runs_the_string_pointer_constructor_on_its_member() {
        let mut e = Engine::new();
        e.register(0x00cb_0a60, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e0_bf20, &args![this]).u32(), this);
        assert_eq!(calls_to(&e, 0x00cb_0a60), vec![vec![this + 8]]);
    }

    #[test]
    fn print_single_node_values_prints_the_percentage_and_each_counted_value() {
        let mut e = engine();
        let name = text(&mut e, "Physics");
        let node = node(
            &mut e,
            name,
            [2.0, 9.0, 4.5, 7.0, 0.0, 0.0, 0.0, 0.0],
            [3, 0, 5, 0, 0, 0, 0, 0],
        );
        e.call_log = Some(vec![]);
        e.call(0x00e0_bfe0, &args![0x1234u32, 8.0f32, node]);
        let formatted = calls_to(&e, SNPRINTF);
        assert_eq!(formatted.len(), 1);
        let buffer = formatted[0][0];
        let percent = double_words(112.5);
        assert_eq!(
            formatted[0][1..],
            [0xc8, 0x010e_a680, name, 5, percent[0], percent[1]]
        );
        let printed = calls_to(&e, PRINTF);
        let first = double_words(2.0);
        let second = double_words(4.5);
        assert_eq!(
            printed,
            vec![
                vec![0x1234, 0x010e_a66c, buffer, 0x010e_a674],
                vec![0x1234, 0x010e_a660, first[0], first[1]],
                vec![0x1234, 0x010e_a660, second[0], second[1]],
                vec![0x1234, 0x0101_2648],
            ]
        );
    }

    #[test]
    fn print_single_node_values_leaves_out_the_percentage_without_a_total() {
        let mut e = engine();
        let name = text(&mut e, "Physics");
        let node = node(&mut e, name, [2.0; 8], [0, 0, 0, 0, 0, 0, 0, 9]);
        e.call_log = Some(vec![]);
        // a NaN total is not above 0.0 either
        for total in [0.0f32, -1.0, f32::NAN] {
            e.call_log = Some(vec![]);
            e.call(0x00e0_bfe0, &args![0x1234u32, total, node]);
            let formatted = calls_to(&e, SNPRINTF);
            assert_eq!(formatted.len(), 1);
            assert_eq!(formatted[0][1..], [0xc8, 0x010e_a678, name, 9]);
            // only the eighth count is not 0
            assert_eq!(calls_to(&e, PRINTF).len(), 3);
        }
    }

    #[test]
    fn save_to_tga_writes_the_header_and_the_pixels() {
        let mut e = engine();
        let headers = Rc::new(RefCell::new(Vec::new()));
        let seen = headers.clone();
        e.register_double(STREAM_WRITE, move |e, a| {
            seen.borrow_mut()
                .push((a[1], a[2], e.mem.bytes(a[1], a[2].min(0x12))));
            Ret::default()
        });
        let image = e.mem.alloc(48);
        let stream = e.mem.alloc(8);
        assert!(e
            .call(0x00e0_c150, &args![image, stream, 4u32, 3u32])
            .bool());
        let writes = headers.borrow();
        assert_eq!(writes.len(), 2);
        let mut expected = [0u8; 0x12];
        expected[2] = 2;
        expected[0xc] = 4;
        expected[0xe] = 3;
        expected[0x10] = 0x20;
        assert_eq!(writes[0].1, 0x12);
        assert_eq!(writes[0].2, expected);
        assert_eq!((writes[1].0, writes[1].1), (image, 4 * 3 * 4));
    }

    /// Glyph `glyph` as a table entry: row `y` (picture row `y`) is `0xf000_0000 >> (4 * y)`.
    fn diagonal_glyph(e: &mut Engine, glyph: u32) {
        let rows = e.mem.alloc(0x20);
        for y in 0..7u32 {
            e.mem
                .set_u32(rows + 0x18 - 4 * y, 0xf000_0000u32 >> (4 * y));
        }
        e.mem.set_u32(GLYPH_TABLE + 4 * glyph, rows);
    }

    #[test]
    fn the_glyph_drawer_sets_the_pixels_whose_row_word_has_a_top_nibble() {
        let mut e = engine();
        diagonal_glyph(&mut e, 3);
        let image = e.mem.alloc(8 * 4 * 8);
        e.call(0x00e0_c1c0, &args![3u32, 8u32, image]);
        for y in 0..8u32 {
            for x in 0..8u32 {
                let pixel = e.mem.u32(image + (8 * y + x) * 4);
                let set = x == y && y < 7;
                assert_eq!(pixel, if set { 0xff00_0000 } else { 0 }, "({x}, {y})");
            }
        }
    }

    #[test]
    fn draw_number_draws_four_digits_seven_pixels_apart() {
        let mut e = engine();
        // digit d lights column d of picture row 0
        for digit in 0..8u32 {
            let rows = e.mem.alloc(0x20);
            e.mem.set_u32(rows + 0x18, 0xf000_0000u32 >> (4 * digit));
            e.mem.set_u32(GLYPH_TABLE + 4 * digit, rows);
        }
        let image = e.mem.alloc(32 * 7 * 4);
        e.call(0x00e0_c210, &args![image, 1234i32, 32u32]);
        let lit: Vec<u32> = (0..32u32)
            .filter(|x| e.mem.u32(image + 4 * x) != 0)
            .collect();
        assert_eq!(lit, vec![1, 9, 17, 25]);
    }

    #[test]
    fn the_frame_info_constructor_sets_its_defaults() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x24);
        for offset in (0..0x24).step_by(4) {
            e.mem.set_u32(this + offset, 0xdead_beef);
        }
        assert_eq!(e.call(0x00e0_c320, &args![this]).u32(), this);
        let words: Vec<u32> = (0..9).map(|i| e.mem.u32(this + 4 * i)).collect();
        assert_eq!(
            words,
            vec![
                0x010e_a690,
                0,
                1,
                0,
                1.0f32.to_bits(),
                1.0f32.to_bits(),
                0,
                0,
                0
            ]
        );
        assert_eq!(e.mem.u32(this + 0x20), 0);
    }

    /// A map double for `00cb1980`: the key's value from `pairs`, answering 0 when found.
    fn install_map(e: &mut Engine, pairs: Vec<(u32, u32)>) {
        e.register_double(MAP_GET, move |e, a| {
            match pairs.iter().find(|pair| pair.0 == a[1]) {
                Some(pair) => {
                    e.mem.set_u32(a[2], pair.1);
                    Ret::default()
                }
                None => Ret {
                    eax: 1,
                    ..Ret::default()
                },
            }
        });
    }

    #[test]
    fn the_stream_walker_turns_keys_into_strings_and_skips_each_commands_words() {
        let mut e = engine();
        let timer = text(&mut e, "Tphysics");
        let end = text(&mut e, "E");
        let value = text(&mut e, "Mvalue");
        let nested = text(&mut e, "Lsplit");
        let skip = text(&mut e, "N");
        let split = text(&mut e, "Ssplit name");
        install_map(
            &mut e,
            vec![
                (0x101, timer),
                (0x102, end),
                (0x103, value),
                (0x104, nested),
                (0x105, skip),
                (0x106, split),
            ],
        );
        let stream = e.mem.alloc(0x80);
        let words = [
            0x101u32, 11, 12, // T: 3 words
            0x102, 21, 22, // E: 3 words
            0x103, 31, // M: 2 words
            0x104, 41, 42, 0x106, // L: 4 words, the last one a key too
            0x105, // N: 1 word
            7,     // below 10: one word, left alone
        ];
        for (i, word) in words.into_iter().enumerate() {
            e.mem.set_u32(stream + 4 * i as u32, word);
        }
        e.call(0x00e0_c350, &args![stream, stream + 4 * 14, 0x77u32, 0u32]);
        let now: Vec<u32> = (0..14).map(|i| e.mem.u32(stream + 4 * i)).collect();
        assert_eq!(
            now,
            vec![timer, 11, 12, end, 21, 22, value, 31, nested, 41, 42, split, skip, 7]
        );
    }

    #[test]
    fn the_stream_walker_stops_at_an_unknown_key_or_tag() {
        let mut e = engine();
        let other = text(&mut e, "Zero");
        let timer = text(&mut e, "T");
        install_map(&mut e, vec![(0x101, timer), (0x102, other)]);
        let stream = e.mem.alloc(0x40);
        let words = [0x101u32, 1, 2, 0x103, 5, 6, 0x101, 3, 4];
        for (i, word) in words.into_iter().enumerate() {
            e.mem.set_u32(stream + 4 * i as u32, word);
        }
        // the key 0x103 is not in the map: the walk ends there
        e.call(0x00e0_c350, &args![stream, stream + 36, 0x77u32, 0u32]);
        assert_eq!(e.mem.u32(stream), timer);
        assert_eq!(e.mem.u32(stream + 12), 0x103);
        assert_eq!(e.mem.u32(stream + 24), 0x101);
        // a tag character that is not a command ends it too
        e.mem.set_u32(stream + 12, 0x102);
        e.call(0x00e0_c350, &args![stream + 12, stream + 36, 0x77u32, 0u32]);
        assert_eq!(e.mem.u32(stream + 12), other);
        assert_eq!(e.mem.u32(stream + 24), 0x101);
    }

    #[test]
    fn the_stream_walker_byte_swaps_what_it_walks_when_asked_to() {
        let mut e = engine();
        let timer = text(&mut e, "Tt");
        let value = text(&mut e, "Mv");
        let nested = text(&mut e, "Ln");
        install_map(
            &mut e,
            vec![
                (0x0a00_0000, timer),
                (0x0b00_0000, value),
                (0x0c00_0000, nested),
            ],
        );
        let stream = e.mem.alloc(0x80);
        let words = [
            0x0500_0000u32, // swapped it is 5: stored back swapped, one word
            0x0a00_0000,
            0x1122_3344,
            0x0102_0304, // T
            0x0b00_0000,
            0x0000_803f, // M: 1.0 swapped
            0x0c00_0000,
            0x5566_7788,
            0x090a_0b0c,
            0x0b00_0000, // L: word 3 is a key
        ];
        for (i, word) in words.into_iter().enumerate() {
            e.mem.set_u32(stream + 4 * i as u32, word);
        }
        e.call(0x00e0_c350, &args![stream, stream + 4 * 10, 0x77u32, 1u32]);
        let now: Vec<u32> = (0..10).map(|i| e.mem.u32(stream + 4 * i)).collect();
        assert_eq!(
            now,
            vec![
                5,
                timer,
                0x4433_2211,
                0x0403_0201,
                value,
                1.0f32.to_bits(),
                nested,
                0x8877_6655,
                0x0c0b_0a09,
                value
            ]
        );
    }

    #[test]
    fn set_timers_adds_the_elapsed_ticks_to_the_chosen_values() {
        let mut e = engine();
        let node = node(&mut e, 0, [10.0; 8], [0; 8]);
        let command = e.mem.alloc(0x18);
        let (begin, end) = (e.mem.alloc(12), e.mem.alloc(12));
        e.mem.set_u32(command + 4, 1);
        e.mem.set_u32(command + 8, 6);
        e.mem.set_u32(command + 0xc, 0);
        e.mem.set_f32(command + 0x10, 0.5);
        e.mem.set_f32(command + 0x14, 2.0);
        e.mem.set_u32(begin + 4, 100);
        e.mem.set_u32(end + 4, 140);
        e.mem.set_u32(begin + 8, 7);
        e.mem.set_u32(end + 8, 10);
        e.call(0x00e0_c5e0, &args![node, command, begin, end]);
        assert_eq!(e.mem.f32(node + 4), 30.0);
        assert_eq!(e.mem.f32(node + 24), 16.0);
        assert_eq!(e.mem.u16(node + COUNTS + 2), 1);
        assert_eq!(e.mem.u16(node + COUNTS + 12), 1);
        // the start time is begin[1] * the first rate
        assert_eq!(e.mem.f64(node + START_TIME), 50.0);
        // mode 1 uses begin[2] with the second rate, and an unsigned word
        e.mem.set_u32(command + 0xc, 1);
        e.mem.set_u32(begin + 8, 0x8000_0000);
        e.mem.set_u32(end + 8, 0x8000_0001);
        e.call(0x00e0_c5e0, &args![node, command, begin, end]);
        assert_eq!(e.mem.f64(node + START_TIME), 2_147_483_648.0 * 2.0);
        assert_eq!(e.mem.u16(node + COUNTS + 2), 2);
        // any other mode leaves the start time alone
        e.mem.set_u32(command + 0xc, 5);
        e.call(0x00e0_c5e0, &args![node, command, begin, end]);
        assert_eq!(e.mem.f64(node + START_TIME), 2_147_483_648.0 * 2.0);
    }

    #[test]
    fn set_timers_ignores_indexes_above_seven() {
        let mut e = engine();
        let node = node(&mut e, 0, [1.0; 8], [0; 8]);
        let command = e.mem.alloc(0x18);
        let (begin, end) = (e.mem.alloc(12), e.mem.alloc(12));
        e.mem.set_u32(command + 4, 8);
        e.mem.set_u32(command + 8, 0xffff_ffff);
        e.mem.set_u32(command + 0xc, 9);
        e.mem.set_u32(end + 4, 5);
        e.call(0x00e0_c5e0, &args![node, command, begin, end]);
        for slot in 0..8 {
            assert_eq!(e.mem.f32(node + 4 * slot), 1.0);
            assert_eq!(e.mem.u16(node + COUNTS + 2 * slot), 0);
        }
    }

    #[test]
    fn the_child_search_tries_the_hint_first_and_then_every_child() {
        let mut e = engine();
        let parent = named(&mut e, "root");
        for name in ["a", "b", "c"] {
            let child = named(&mut e, name);
            adopt(&mut e, parent, child);
        }
        let wanted = text(&mut e, "c");
        let index = e.mem.alloc(4);
        // hint 2 is the answer: true, the hint stays
        e.mem.set_u32(index, 2);
        assert!(e.call(0x00e0_c690, &args![parent, wanted, index]).bool());
        assert_eq!(e.mem.u32(index), 2);
        // hint 0 is wrong: found by the search, the index is stored
        e.mem.set_u32(index, 0);
        assert!(e.call(0x00e0_c690, &args![parent, wanted, index]).bool());
        assert_eq!(e.mem.u32(index), 2);
        // a hint past the children is not tried
        e.mem.set_u32(index, 7);
        assert!(e.call(0x00e0_c690, &args![parent, wanted, index]).bool());
        assert_eq!(e.mem.u32(index), 2);
        // a name nobody has
        let missing = text(&mut e, "z");
        e.mem.set_u32(index, 1);
        assert!(!e.call(0x00e0_c690, &args![parent, missing, index]).bool());
        assert_eq!(e.mem.u32(index), 1);
    }

    #[test]
    fn the_scaler_multiplies_one_value_of_every_node_and_clears_its_count() {
        let mut e = engine();
        let root = named(&mut e, "root");
        let child = named(&mut e, "child");
        adopt(&mut e, root, child);
        let grandchild = named(&mut e, "grandchild");
        adopt(&mut e, child, grandchild);
        for (at, value) in [(root, 2.0f32), (child, 3.0), (grandchild, 4.0)] {
            e.mem.set_f32(at + 8, value);
            e.mem.set_f32(at + 12, value);
            e.mem.set_u16(at + COUNTS + 4, 9);
            e.mem.set_u16(at + COUNTS + 6, 9);
        }
        e.call(0x00e0_c700, &args![root, 2u32, 0.5f32]);
        assert_eq!(e.mem.f32(root + 8), 1.0);
        assert_eq!(e.mem.f32(child + 8), 1.5);
        assert_eq!(e.mem.f32(grandchild + 8), 2.0);
        assert_eq!(e.mem.f32(grandchild + 12), 4.0);
        assert_eq!(e.mem.u16(grandchild + COUNTS + 4), 0);
        assert_eq!(e.mem.u16(grandchild + COUNTS + 6), 9);
    }

    #[test]
    fn the_longest_name_indent_looks_at_every_open_node() {
        let mut e = engine();
        let root = named(&mut e, "abcd");
        let child = named(&mut e, "efghijklmn");
        let grandchild = named(&mut e, "x");
        adopt(&mut e, root, child);
        adopt(&mut e, child, grandchild);
        // each name's length + depth * width + 8
        assert_eq!(
            e.call(0x00e0_c860, &args![root, 0u32, 3u32, 0u32]).i32(),
            10 + 3 + 8 // the longer of the two names (1 + 6 + 8 is shorter)
        );
        // with `only_open`, a node without bit 0 of its flags stops the search
        assert_eq!(
            e.call(0x00e0_c860, &args![root, 0u32, 3u32, 1u32]).i32(),
            12
        );
        e.mem.set_u32(root + NODE_FLAGS, 1);
        assert_eq!(
            e.call(0x00e0_c860, &args![root, 0u32, 3u32, 1u32]).i32(),
            21
        );
        e.mem.set_u32(child + NODE_FLAGS, 1);
        assert_eq!(
            e.call(0x00e0_c860, &args![root, 0u32, 3u32, 1u32]).i32(),
            21
        );
    }

    #[test]
    fn write_rec_prints_one_line_per_node_with_its_values() {
        let mut e = engine();
        let root = named(&mut e, "root");
        let child = node(
            &mut e,
            0,
            [3.0, 0.0, 8.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            [4, 0, 2, 0, 0, 0, 0, 0],
        );
        let name = text(&mut e, "child");
        e.mem.set_u32(child + NODE_NAME, name);
        adopt(&mut e, root, child);
        e.call_log = Some(vec![]);
        e.call(0x00e0_c8e0, &args![0x55u32, root, 0u32, 1.0f32]);
        // depth 0 prints only the children: the child at depth 1 has no indent
        let formatted = calls_to(&e, SNPRINTF);
        assert_eq!(formatted.len(), 2);
        assert_eq!(formatted[0][1..], [0xc8, 0x0108_03c0, name, 4]);
        // scale 1.0: the count in the text
        assert_eq!(formatted[1][1..], [0xc8, 0x010e_a678, name, 4]);
        let printed = calls_to(&e, PRINTF);
        let first = double_words(3.0);
        // value 2 has 2 of 4 counts: scaled up by 4 / 2
        let second = double_words(16.0);
        assert_eq!(
            printed,
            vec![
                vec![0x55, 0x010e_a6ac, formatted[0][0]],
                vec![0x55, 0x0109_e324],
                vec![0x55, 0x0109_e324],
                vec![0x55, 0x0109_e324],
                vec![0x55, 0x010e_a660, first[0], first[1]],
                vec![0x55, 0x010e_a6a0, second[0], second[1]],
            ]
        );
    }

    #[test]
    fn write_rec_indents_by_depth_and_shows_the_scaled_count() {
        let mut e = engine();
        let node = node(&mut e, 0, [0.0; 8], [3, 0, 0, 0, 0, 0, 0, 0]);
        let name = text(&mut e, "n");
        e.mem.set_u32(node + NODE_NAME, name);
        e.call_log = Some(vec![]);
        e.call(0x00e0_c8e0, &args![0x55u32, node, 3u32, 0.5f32]);
        let formatted = calls_to(&e, SNPRINTF);
        let shown = double_words(1.5);
        assert_eq!(
            formatted[1][1..],
            [0xc8, 0x010e_a6b4, name, shown[0], shown[1]]
        );
        let printed = calls_to(&e, PRINTF);
        // depth - 1 indents come first
        assert_eq!(printed[0], vec![0x55, 0x0109_e324]);
        assert_eq!(printed[1], vec![0x55, 0x0109_e324]);
        assert_eq!(printed[2][1], 0x010e_a6ac);
        // one counted value, printed with the last format
        let last = double_words(0.0);
        assert_eq!(
            printed.last().unwrap(),
            &vec![0x55, 0x010e_a6a0, last[0], last[1]]
        );
        // a node with no counts prints 0.0 with the last format; a count of 1 skips the scale
        let empty = e.mem.alloc(NODE_SIZE);
        e.mem.set_u32(empty + NODE_NAME, name);
        e.call_log = Some(vec![]);
        e.call(0x00e0_c8e0, &args![0x55u32, empty, 1u32, 1.0f32]);
        let formatted = calls_to(&e, SNPRINTF);
        assert_eq!(formatted[1][1..], [0xc8, 0x0101_9f08, name]);
        let printed = calls_to(&e, PRINTF);
        assert_eq!(
            printed.last().unwrap(),
            &vec![0x55, 0x010e_a6a0, last[0], last[1]]
        );
    }

    #[test]
    fn the_named_child_lookup_can_fall_back_to_the_first_child() {
        let mut e = engine();
        let parent = named(&mut e, "root");
        let first = named(&mut e, "a");
        let second = named(&mut e, "b");
        adopt(&mut e, parent, first);
        adopt(&mut e, parent, second);
        let wanted = text(&mut e, "b");
        let missing = text(&mut e, "z");
        assert_eq!(
            e.call(0x00e0_cb80, &args![parent, wanted, 0u32]).u32(),
            second
        );
        assert_eq!(
            e.call(0x00e0_cb80, &args![parent, missing, 1u32]).u32(),
            first
        );
        assert_eq!(e.call(0x00e0_cb80, &args![parent, missing, 0u32]).u32(), 0);
        let empty = named(&mut e, "empty");
        assert_eq!(e.call(0x00e0_cb80, &args![empty, wanted, 1u32]).u32(), 0);
        // only the low byte of the flag counts
        assert_eq!(
            e.call(0x00e0_cb80, &args![parent, missing, 0x100u32]).u32(),
            0
        );
    }

    #[test]
    fn the_next_named_child_lookup_starts_after_the_given_child() {
        let mut e = engine();
        let parent = named(&mut e, "root");
        let children: Vec<u32> = ["a", "b", "c", "b"]
            .into_iter()
            .map(|name| {
                let child = named(&mut e, name);
                adopt(&mut e, parent, child);
                child
            })
            .collect();
        let wanted = text(&mut e, "b");
        let call =
            |e: &mut Engine, after: u32| e.call(0x00e0_cbe0, &args![parent, after, wanted]).u32();
        assert_eq!(call(&mut e, children[0]), children[1]);
        assert_eq!(call(&mut e, children[1]), children[3]);
        assert_eq!(call(&mut e, children[3]), 0);
        // a child that is not in the list: the search starts past the end
        let stranger = named(&mut e, "b");
        assert_eq!(call(&mut e, stranger), 0);
    }

    #[test]
    fn the_running_node_search_starts_at_the_candidate_and_goes_down() {
        let mut e = engine();
        let fallback = named(&mut e, "fallback");
        let running = named(&mut e, "running");
        e.mem.set_f64(running + START_TIME, 1.0);
        e.mem.set_f32(running, 10.0);
        adopt(&mut e, fallback, running);
        let start = named(&mut e, "start");
        e.mem.set_f64(start + START_TIME, 2.0);
        e.mem.set_f32(start, 4.0);
        let call = |e: &mut Engine, start: u32, time: f64| {
            e.call(0x00e0_cc40, &args![fallback, start, time, 0u32])
                .u32()
        };
        // the start covers 2.0 to 6.0: it is the candidate (nothing is deeper)
        assert_eq!(call(&mut e, start, 5.0), start);
        // 7.0 is past its end: the fallback is the candidate and its child is deeper
        assert_eq!(call(&mut e, start, 7.0), running);
        // no start: the fallback and then its child
        assert_eq!(call(&mut e, 0, 5.0), running);
        // nothing runs at 20.0 below the fallback: the fallback itself
        assert_eq!(call(&mut e, 0, 20.0), fallback);
    }

    #[test]
    fn the_color_table_finds_a_color_by_name_ignoring_case() {
        let mut e = engine();
        let table = e.mem.alloc(0x18);
        let entries = e.mem.alloc(16);
        let alpha = text(&mut e, "Alpha");
        let beta = text(&mut e, "Beta");
        // bit 0 of the string pointer is a flag the lookup masks off
        e.mem.set_u32(entries, alpha | 1);
        e.mem.set_u32(entries + 4, 0x11);
        e.mem.set_u32(entries + 8, beta);
        e.mem.set_u32(entries + 12, 0x22);
        e.mem.set_u32(table + 8, entries);
        e.mem.set_u32(table + 0xc, 2);
        e.mem.set_u32(table + 0x14, 0x99);
        let wanted = text(&mut e, "beta");
        assert_eq!(e.call(0x00e0_cc90, &args![table, wanted]).u32(), 0x22);
        let wanted = text(&mut e, "ALPHA");
        assert_eq!(e.call(0x00e0_cc90, &args![table, wanted]).u32(), 0x11);
        let wanted = text(&mut e, "gamma");
        assert_eq!(e.call(0x00e0_cc90, &args![table, wanted]).u32(), 0x99);
    }

    #[test]
    fn the_hint_clearer_zeroes_one_word_of_each_element_and_the_word_at_0x10() {
        let mut e = engine();
        let this = e.mem.alloc(0x20);
        let elements = e.mem.alloc(36);
        for word in 0..9 {
            e.mem.set_u32(elements + 4 * word, 0x1000 + word);
        }
        e.mem.set_u32(this, elements);
        e.mem.set_u32(this + 4, 3);
        e.mem.set_u32(this + 0x10, 77);
        e.call(0x00e0_ccf0, &args![this]);
        assert_eq!(e.mem.u32(this + 0x10), 0);
        let words: Vec<u32> = (0..9).map(|i| e.mem.u32(elements + 4 * i)).collect();
        assert_eq!(
            words,
            vec![0x1000, 0, 0x1002, 0x1003, 0, 0x1005, 0x1006, 0, 0x1008]
        );
    }

    #[test]
    fn the_tag_collector_inserts_each_command_word_and_the_names_inside_commands() {
        let mut e = engine();
        let (timer, nested, split) = (text(&mut e, "Tt"), text(&mut e, "Ln"), text(&mut e, "Ss"));
        let (value, skip, stop) = (text(&mut e, "Mv"), text(&mut e, "N"), text(&mut e, "F"));
        e.register(MAP_INSERT, |_, _| Ret::default());
        let stream = e.mem.alloc(0x80);
        let words = [
            timer, 1, 2, // T
            nested, 3, 4, split, // L, word 3 is a name too
            value, 9,    // M
            skip, // N
            5,    // below 10
            stop, // F: inserted, then the walk ends
            timer, 1, 2,
        ];
        for (i, word) in words.into_iter().enumerate() {
            e.mem.set_u32(stream + 4 * i as u32, word);
        }
        let router = router_of(&mut e);
        let allocator = e.mem.u32(router + 0x10);
        e.call_log = Some(vec![]);
        e.call(0x00e0_cd20, &args![stream, stream + 4 * 15, 0x66u32]);
        let inserted = calls_to(&e, MAP_INSERT);
        assert_eq!(
            inserted,
            vec![
                vec![0x66, allocator, timer, timer],
                vec![0x66, allocator, nested, nested],
                vec![0x66, allocator, split, split],
                vec![0x66, allocator, value, value],
                vec![0x66, allocator, skip, skip],
                vec![0x66, allocator, stop, stop],
            ]
        );
    }

    #[test]
    fn a_recorded_frame_is_copied_into_the_buffer_and_the_thread_array() {
        let mut e = engine();
        e.register(MEM_COPY, |_, _| Ret::default());
        let this = e.mem.alloc(0x20);
        let threads = e.mem.alloc(24);
        e.mem.set_u32(threads + 8, NOT_OWNED);
        e.mem.set_u32(threads + 12 + 8, NOT_OWNED);
        let buffer = e.mem.alloc(0x40);
        e.mem.set_u32(this, threads);
        e.mem.set_u32(this + 0xc, buffer);
        e.mem.set_u32(this + 0x10, 8);
        e.mem.set_u32(this + 0x14, 0x30 | NOT_OWNED);
        let frame = e.mem.alloc(0x24);
        for word in 0..9 {
            e.mem.set_u32(frame + 4 * word, 0x500 + word);
        }
        e.mem.set_u32(frame + 0x18, 1);
        let out = e.mem.alloc(4);
        e.mem.set_u8(out, 0xaa);
        let source = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        e.call(0x00e0_ce00, &args![this, out, source, source + 0x10, frame]);
        assert_eq!(e.mem.u8(out), 1);
        assert_eq!(e.mem.u32(this + 0x10), 8 + 0x10);
        assert_eq!(calls_to(&e, MEM_COPY), vec![vec![buffer + 8, source, 0x10]]);
        // the second thread's array got one 0x24-byte record
        assert_eq!(e.mem.u32(threads + 12 + 4), 1);
        assert_eq!(e.mem.u32(threads + 4), 0);
        let record = e.mem.u32(threads + 12);
        let words: Vec<u32> = (0..9).map(|i| e.mem.u32(record + 4 * i)).collect();
        assert_eq!(
            words,
            vec![0x500, 0x501, 0x502, 0x503, 0x504, 0x505, 1, 8, 8 + 0x10]
        );
        // 0x30 bytes of room, 0x18 left: another 0x20 does not fit
        e.mem.set_u8(out, 0xaa);
        e.call(0x00e0_ce00, &args![this, out, source, source + 0x20, frame]);
        assert_eq!(e.mem.u8(out), 0);
        assert_eq!(e.mem.u32(this + 0x10), 8 + 0x10);
        assert_eq!(e.mem.u32(threads + 12 + 4), 1);
    }

    #[test]
    fn fn_00e0ce00_answers_its_result_pointer() {
        let mut e = engine();
        e.register(MEM_COPY, |_, _| Ret::default());
        let this = e.mem.alloc(0x20);
        let threads = e.mem.alloc(24);
        e.mem.set_u32(threads + 8, NOT_OWNED);
        e.mem.set_u32(threads + 12 + 8, NOT_OWNED);
        let buffer = e.mem.alloc(0x40);
        e.mem.set_u32(this, threads);
        e.mem.set_u32(this + 0xc, buffer);
        e.mem.set_u32(this + 0x10, 8);
        e.mem.set_u32(this + 0x14, 0x30 | NOT_OWNED);
        let frame = e.mem.alloc(0x24);
        e.mem.set_u32(frame + 0x18, 1);
        let out = e.mem.alloc(4);
        let source = e.mem.alloc(0x20);
        // EAX is `out` at both RETs (the hkBool result pointer the callers read through).
        let answered = e
            .call(0x00e0_ce00, &args![this, out, source, source + 0x10, frame])
            .u32();
        assert_eq!(answered, out);
    }

    #[test]
    fn the_node_constructor_clears_the_node_and_joins_its_parent() {
        let mut e = engine();
        let parent = named(&mut e, "parent");
        let node = e.mem.alloc(NODE_SIZE);
        for offset in (0..0x58).step_by(4) {
            e.mem.set_u32(node + offset, 0xdead_beef);
        }
        let name = text(&mut e, "node");
        assert_eq!(
            e.call(0x00e0_d000, &args![node, parent, name, 2u32]).u32(),
            node
        );
        for slot in 0..8 {
            assert_eq!(e.mem.u32(node + 4 * slot), 0);
            assert_eq!(e.mem.u16(node + COUNTS + 2 * slot), 0);
        }
        assert_eq!(e.mem.f64(node + START_TIME), 0.0);
        assert_eq!(e.mem.u32(node + 0x48), parent);
        assert_eq!(e.mem.u32(node + NODE_NAME), name);
        assert_eq!(e.mem.u32(node + NODE_FLAGS), 0);
        assert_eq!(e.mem.u32(node + NODE_KIND), 2);
        assert_eq!(e.mem.u32(node + CHILDREN), 0);
        assert_eq!(e.mem.u32(node + CHILDREN + 4), 0);
        assert_eq!(e.mem.u32(node + CHILDREN + 8), NOT_OWNED);
        assert_eq!(child_count(&e, parent), 1);
        assert_eq!(child_at(&e, parent, 0), node);
        // without a parent nothing is appended
        let alone = e.mem.alloc(NODE_SIZE);
        e.call(0x00e0_d000, &args![alone, 0u32, name, 1u32]);
        assert_eq!(e.mem.u32(alone + 0x48), 0);
        assert_eq!(child_count(&e, parent), 1);
    }

    #[test]
    fn the_node_destructor_destroys_the_children_and_frees_an_owned_array() {
        let mut e = engine();
        e.register(0x00dd_eff0, |_, _| Ret::default());
        let node = named(&mut e, "n");
        let (first, second) = (named(&mut e, "a"), named(&mut e, "b"));
        adopt(&mut e, node, first);
        adopt(&mut e, node, second);
        // a null child is skipped
        let data = e.mem.u32(node + CHILDREN);
        e.mem.set_u32(data + 8, 0);
        e.mem.set_u32(node + CHILDREN + 4, 3);
        e.mem.set_u32(node + CHILDREN + 8, 4);
        let router = router_of(&mut e);
        let allocator = e.mem.u32(router + 0x14);
        e.call_log = Some(vec![]);
        e.call(0x00e0_d0a0, &args![node]);
        assert_eq!(
            calls_to(&e, 0x00dd_eff0),
            vec![vec![first, 1], vec![second, 1]]
        );
        assert_eq!(calls_to(&e, FREE_DATA), vec![vec![allocator, data, 16]]);
        assert_eq!(e.mem.u32(node + CHILDREN), 0);
        assert_eq!(e.mem.u32(node + CHILDREN + 4), 0);
        assert_eq!(e.mem.u32(node + CHILDREN + 8), NOT_OWNED);
        // an array marked as not owned is not freed
        let other = named(&mut e, "o");
        let child = named(&mut e, "c");
        adopt(&mut e, other, child);
        e.call_log = Some(vec![]);
        e.call(0x00e0_d0a0, &args![other]);
        assert!(calls_to(&e, FREE_DATA).is_empty());
        assert_eq!(e.mem.u32(other + CHILDREN), 0);
    }

    /// The values of a node's eight floats and eight counts, for assertions.
    fn values_of(e: &Engine, node: u32) -> ([f32; 8], [u16; 8]) {
        (
            std::array::from_fn(|slot| e.mem.f32(node + 4 * slot as u32)),
            std::array::from_fn(|slot| e.mem.u16(node + COUNTS + 2 * slot as u32)),
        )
    }

    #[test]
    fn merging_children_creates_the_missing_ones_and_blends_one_value() {
        let mut e = engine();
        let dest = named(&mut e, "dst");
        let existing = named(&mut e, "A");
        e.mem.set_f32(existing + 24, 1.0);
        adopt(&mut e, dest, existing);
        let source = named(&mut e, "src");
        let (first, second, nested) = (named(&mut e, "A"), named(&mut e, "B"), named(&mut e, "G"));
        for (node, value, count) in [(first, 10.0f32, 3u16), (second, 20.0, 4), (nested, 4.0, 1)] {
            e.mem.set_f32(node + 8, value);
            e.mem.set_u16(node + COUNTS + 4, count);
        }
        e.mem.set_u32(second + NODE_KIND, 5);
        adopt(&mut e, source, first);
        adopt(&mut e, source, second);
        adopt(&mut e, first, nested);
        // the destination index is limited to 6; the blend is value * (1 - 0.25) + the old value
        e.call(0x00e0_d180, &args![dest, source, 9i32, 2u32, 0.25f32]);
        assert_eq!(child_count(&e, dest), 2);
        assert_eq!(child_at(&e, dest, 0), existing);
        assert_eq!(e.mem.f32(existing + 24), 8.5);
        assert_eq!(e.mem.u16(existing + COUNTS + 12), 3);
        let created = child_at(&e, dest, 1);
        assert_eq!(
            e.mem.u32(created + NODE_NAME),
            e.mem.u32(second + NODE_NAME)
        );
        assert_eq!(e.mem.u32(created + NODE_KIND), 5);
        assert_eq!(e.mem.u32(created + 0x48), dest);
        assert_eq!(e.mem.f32(created + 24), 15.0);
        assert_eq!(e.mem.u16(created + COUNTS + 12), 4);
        // the grandchild went under the existing child
        assert_eq!(child_count(&e, existing), 1);
        let grandchild = child_at(&e, existing, 0);
        assert_eq!(e.mem.f32(grandchild + 24), 3.0);
        assert_eq!(e.mem.u16(grandchild + COUNTS + 12), 1);
        // a source without children changes nothing
        let nothing = named(&mut e, "n");
        e.call(0x00e0_d180, &args![dest, nothing, 1i32, 0u32, 0.5f32]);
        assert_eq!(child_count(&e, dest), 2);
    }

    #[test]
    fn the_scaled_merge_scales_the_destination_first() {
        let mut e = engine();
        let target = named(&mut e, "t");
        e.mem.set_f32(target + 4, 8.0);
        e.mem.set_u16(target + COUNTS + 2, 5);
        let inner = named(&mut e, "x");
        e.mem.set_f32(inner + 4, 4.0);
        e.mem.set_u16(inner + COUNTS + 2, 6);
        adopt(&mut e, target, inner);
        let source = named(&mut e, "s");
        let other = named(&mut e, "x");
        e.mem.set_f32(other + 8, 10.0);
        e.mem.set_u16(other + COUNTS + 4, 7);
        adopt(&mut e, source, other);
        e.call(0x00e0_d280, &args![target, source, 1i32, 2u32, 0.5f32]);
        assert_eq!(e.mem.f32(target + 4), 4.0);
        assert_eq!(e.mem.u16(target + COUNTS + 2), 0);
        // 4.0 * 0.5 + 10.0 * (1 - 0.5)
        assert_eq!(e.mem.f32(inner + 4), 7.0);
        assert_eq!(e.mem.u16(inner + COUNTS + 2), 7);
    }

    #[test]
    fn adding_trees_sums_the_values_and_makes_missing_children() {
        let mut e = engine();
        let dest = named(&mut e, "d");
        let existing = node(
            &mut e,
            0,
            [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0],
            [1, 2, 3, 4, 5, 6, 7, 8],
        );
        let name = text(&mut e, "A");
        e.mem.set_u32(existing + NODE_NAME, name);
        adopt(&mut e, dest, existing);
        let source = named(&mut e, "s");
        let same = node(&mut e, 0, [10.0; 8], [10; 8]);
        let name = text(&mut e, "A");
        e.mem.set_u32(same + NODE_NAME, name);
        let extra = node(&mut e, 0, [5.0; 8], [2; 8]);
        let name = text(&mut e, "B");
        e.mem.set_u32(extra + NODE_NAME, name);
        e.mem.set_u32(extra + NODE_KIND, 7);
        let deeper = node(&mut e, 0, [0.5; 8], [1; 8]);
        let name = text(&mut e, "C");
        e.mem.set_u32(deeper + NODE_NAME, name);
        adopt(&mut e, source, same);
        adopt(&mut e, source, extra);
        adopt(&mut e, extra, deeper);
        e.call(0x00e0_d880, &args![dest, source]);
        assert_eq!(
            values_of(&e, existing),
            (
                [11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0, 18.0],
                [11, 12, 13, 14, 15, 16, 17, 18]
            )
        );
        assert_eq!(child_count(&e, dest), 2);
        let created = child_at(&e, dest, 1);
        assert_eq!(e.mem.u32(created + NODE_NAME), e.mem.u32(extra + NODE_NAME));
        assert_eq!(e.mem.u32(created + NODE_KIND), 7);
        assert_eq!(values_of(&e, created), ([5.0; 8], [2; 8]));
        let grandchild = child_at(&e, created, 0);
        assert_eq!(values_of(&e, grandchild), ([0.5; 8], [1; 8]));
    }

    #[test]
    fn add_color_appends_a_copy_of_the_name_and_its_color() {
        let mut e = engine();
        e.register(0x00cb_0b80, |_, _| Ret::default());
        e.register(0x00cb_0bd0, |_, _| Ret::default());
        e.register(0x00cb_0c00, |_, _| Ret::default());
        let this = e.mem.alloc(0x18);
        e.mem.set_u32(this + 8 + 8, NOT_OWNED);
        let name = text(&mut e, "Havok");
        e.call_log = Some(vec![]);
        e.call(0x00e0_d980, &args![this, name, 0xff12_3456u32]);
        let built = calls_to(&e, 0x00cb_0b80);
        let copied = calls_to(&e, 0x00cb_0bd0);
        let destroyed = calls_to(&e, 0x00cb_0c00);
        let string = built[0][0];
        assert_eq!(built, vec![vec![string, name]]);
        let data = e.mem.u32(this + 8);
        assert_eq!(copied, vec![vec![data, string]]);
        assert_eq!(destroyed, vec![vec![string]]);
        assert_eq!(e.mem.u32(this + 8 + 4), 1);
        assert_eq!(e.mem.u32(data + 4), 0xff12_3456);
    }

    /// A node whose children (kind 0) start at `starts` and last `duration`.
    fn timeline(e: &mut Engine, starts: &[f64], duration: f32) -> (u32, Vec<u32>) {
        let root = named(e, "timeline");
        let children = starts
            .iter()
            .map(|start| {
                let child = named(e, "slice");
                e.mem.set_f64(child + START_TIME, *start);
                e.mem.set_f32(child, duration);
                adopt(e, root, child);
                child
            })
            .collect();
        (root, children)
    }

    #[test]
    fn the_running_child_is_found_by_bisection_and_then_a_scan() {
        let mut e = engine();
        let (root, children) = timeline(&mut e, &[0.0, 10.0, 20.0, 30.0, 40.0, 50.0], 9.0);
        let at = |e: &mut Engine, time: f64| e.call(0x00e1_0820, &args![root, time, 0u32]).u32();
        assert_eq!(at(&mut e, 25.0), children[2]);
        assert_eq!(at(&mut e, 0.0), children[0]);
        assert_eq!(at(&mut e, 54.0), children[5]);
        // between two children: the scan stops at the first one that is still running
        assert_eq!(at(&mut e, 29.5), 0);
        // past the last one
        assert_eq!(at(&mut e, 100.0), 0);
        // two children are only scanned
        let (small, kids) = timeline(&mut e, &[5.0, 15.0], 4.0);
        assert_eq!(
            e.call(0x00e1_0820, &args![small, 16.0f64, 0u32]).u32(),
            kids[1]
        );
        // a node without children
        let empty = named(&mut e, "e");
        assert_eq!(e.call(0x00e1_0820, &args![empty, 1.0f64, 0u32]).u32(), 0);
        // the bisection stops at a child of kind 2, and kind 2 children are never the answer
        for child in &children {
            e.mem.set_u32(child + NODE_KIND, 2);
        }
        assert_eq!(at(&mut e, 25.0), 0);
    }

    #[test]
    fn the_indenter_starts_as_an_empty_string_with_room_for_64_characters() {
        let mut e = engine();
        let this = e.mem.alloc(12);
        for offset in [0, 4, 8] {
            e.mem.set_u32(this + offset, 0xdead_beef);
        }
        let router = router_of(&mut e);
        let allocator = e.mem.u32(router + 0x14);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e1_08b0, &args![this]).u32(), this);
        assert_eq!(
            calls_to(&e, ARRAY_RESERVE),
            vec![vec![allocator, this, 0x40, 1]]
        );
        assert_eq!(e.mem.u32(this + 4), 1);
        assert_eq!(e.mem.u32(this + 8) & CAPACITY_MASK, 0x40);
        assert_eq!(e.mem.u8(e.mem.u32(this)), 0);
    }

    #[test]
    fn the_by_type_info_destructor_destroys_both_node_arrays_and_itself() {
        let mut e = engine();
        e.register(0x00dd_eff0, |_, _| Ret::default());
        let owned = e.mem.alloc(NODE_SIZE * 2);
        let borrowed = e.mem.alloc(NODE_SIZE);
        for node in [owned, owned + NODE_SIZE, borrowed] {
            e.mem.set_u32(node + CHILDREN + 8, NOT_OWNED);
        }
        // the array at +0x64 holds two nodes it owns, the one at +0x58 one it does not
        let info = e.mem.alloc(0x80);
        e.mem.set_u32(info + 0x64, owned);
        e.mem.set_u32(info + 0x68, 2);
        e.mem.set_u32(info + 0x6c, 2);
        e.mem.set_u32(info + 0x58, borrowed);
        e.mem.set_u32(info + 0x5c, 1);
        e.mem.set_u32(info + 0x60, 1 | NOT_OWNED);
        e.mem.set_u32(info + CHILDREN + 8, NOT_OWNED);
        let router = router_of(&mut e);
        let allocator = e.mem.u32(router + 0x14);
        e.call_log = Some(vec![]);
        e.call(0x00e1_0930, &args![info]);
        assert_eq!(
            calls_to(&e, FREE_DATA),
            vec![vec![allocator, owned, 2 * NODE_SIZE]]
        );
        assert_eq!(e.mem.u32(info + 0x64), 0);
        assert_eq!(e.mem.u32(info + 0x68), 0);
        assert_eq!(e.mem.u32(info + 0x6c), NOT_OWNED);
        assert_eq!(e.mem.u32(info + 0x58), 0);
        assert_eq!(e.mem.u32(info + 0x5c), 0);
        assert_eq!(e.mem.u32(info + 0x60), NOT_OWNED);
        assert_eq!(e.mem.u32(info + CHILDREN + 8), NOT_OWNED);
    }

    /// A root whose two children start at 10.0 and 20.0 and have a first value of 3.0 and 4.0.
    fn sampled_tree(e: &mut Engine) -> (u32, u32) {
        let root = named(e, "root");
        let first = named(e, "first");
        let second = named(e, "second");
        e.mem.set_f64(first + START_TIME, 10.0);
        e.mem.set_f32(first, 3.0);
        e.mem.set_f64(second + START_TIME, 20.0);
        e.mem.set_f32(second, 4.0);
        adopt(e, root, first);
        adopt(e, root, second);
        (root, first)
    }

    fn words_of(e: &Engine, array: u32) -> Vec<u32> {
        let data = e.mem.u32(array);
        (0..e.mem.u32(array + 4))
            .map(|i| e.mem.u32(data + 4 * i))
            .collect()
    }

    #[test]
    fn frame_sampling_pads_with_zeros_and_then_looks_up_the_running_nodes() {
        let mut e = engine();
        let (root, first) = sampled_tree(&mut e);
        let array = e.mem.alloc(12);
        e.mem.set_u32(array + 8, NOT_OWNED);
        e.call(0x00e0_cec0, &args![5u32, root, array, 2.0f32, 4.0f32, 0u32]);
        // two zeros up to the first start less the time, then a node (or the root) for each step
        assert_eq!(words_of(&e, array), vec![0, 0, first, root, root]);
        assert_eq!(e.mem.f64(root + START_TIME), 10.0);
        // ((time + (last end - time)) - first start) + 1.0
        assert_eq!(e.mem.f32(root), 15.0);
    }

    #[test]
    fn frame_sampling_with_a_small_count_stops_early() {
        let mut e = engine();
        let (root, _) = sampled_tree(&mut e);
        let array = e.mem.alloc(12);
        e.mem.set_u32(array + 8, NOT_OWNED);
        // the count runs out in the zero-padding: the root's values are still set
        e.call(0x00e0_cec0, &args![2u32, root, array, 2.0f32, 4.0f32, 0u32]);
        assert_eq!(words_of(&e, array), vec![0, 0]);
        assert_eq!(e.mem.f32(root), 15.0);
        // a count of 0 pads nothing and samples nothing
        let other = e.mem.alloc(12);
        e.mem.set_u32(other + 8, NOT_OWNED);
        e.call(0x00e0_cec0, &args![0u32, root, other, 2.0f32, 4.0f32, 0u32]);
        assert_eq!(words_of(&e, other), Vec::<u32>::new());
    }

    #[test]
    fn the_sampled_node_is_the_array_word_for_the_sample() {
        let mut e = engine();
        let (root, first) = sampled_tree(&mut e);
        let roots = e.mem.alloc(8);
        let table = e.mem.alloc(8);
        e.mem.set_u32(roots, table);
        e.mem.set_u32(table + 4, root);
        let selector = e.mem.alloc(0x10);
        // selector[+0xc] is 0: the index is selector[+4]
        e.mem.set_u32(selector + 4, 0);
        e.mem.set_u32(selector + 8, 5);
        let call = |e: &mut Engine, sample: u32| {
            e.call(
                0x00e0_da00,
                &args![selector, 1u32, sample, roots, 5u32, 20.0f32, 2.0f32],
            )
            .u32()
        };
        assert_eq!(call(&mut e, 2), first);
        assert_eq!(call(&mut e, 0), 0);
        assert_eq!(call(&mut e, 4), root);
        // a sample past the array
        assert_eq!(call(&mut e, 5), 0);
        // a root without children answers 0
        let empty = named(&mut e, "empty");
        e.mem.set_u32(table, empty);
        let answer = e
            .call(
                0x00e0_da00,
                &args![selector, 0u32, 2u32, roots, 5u32, 20.0f32, 2.0f32],
            )
            .u32();
        assert_eq!(answer, 0);
    }

    #[test]
    fn the_array_constructor_makes_an_empty_array() {
        let mut e = Engine::new();
        let this = e.mem.alloc(12);
        for offset in [0, 4, 8] {
            e.mem.set_u32(this + offset, 0x1234_5678);
        }
        assert_eq!(e.call(0x00e0_f6e0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), 0);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u32(this + 8), NOT_OWNED);
    }

    #[test]
    fn resizing_the_array_of_arrays_constructs_new_ones_and_frees_the_extra_ones() {
        let mut e = engine();
        let this = e.mem.alloc(0x20);
        let elements = e.mem.alloc(12 * 3);
        // two elements: the first owns 4 elements of 0x24 bytes, the second does not own its data
        let owned = e.mem.alloc(0x24 * 4);
        e.mem.set_u32(elements, owned);
        e.mem.set_u32(elements + 4, 2);
        e.mem.set_u32(elements + 8, 4);
        let borrowed = e.mem.alloc(0x24);
        e.mem.set_u32(elements + 12, borrowed);
        e.mem.set_u32(elements + 16, 1);
        e.mem.set_u32(elements + 20, 1 | NOT_OWNED);
        e.mem.set_u32(this, elements);
        e.mem.set_u32(this + 4, 2);
        e.mem.set_u32(this + 8, 3);
        e.mem.set_u32(this + 0x10, 99);
        let router = router_of(&mut e);
        let allocator = e.mem.u32(router + 0x10);
        // two more elements than there are: room is made (the larger of 4 and twice 3 is 6)
        e.call_log = Some(vec![]);
        e.call(0x00e0_f710, &args![this, 1u32, 3u32]);
        assert_eq!(
            calls_to(&e, ARRAY_RESERVE),
            vec![vec![allocator, this, 6, 12]]
        );
        assert_eq!(e.mem.u32(this + 4), 4);
        assert_eq!(e.mem.u32(this + 0x18), 1);
        assert_eq!(e.mem.u32(this + 0x1c), 3);
        assert_eq!(e.mem.u32(this + 0x10), 0);
        let data = e.mem.u32(this);
        // the new elements are empty arrays; every element's size was cleared at the end
        for index in 0..4 {
            assert_eq!(e.mem.u32(data + 12 * index + 4), 0, "element {index}");
        }
        assert_eq!(e.mem.u32(data + 24), 0);
        assert_eq!(e.mem.u32(data + 24 + 8), NOT_OWNED);
        assert_eq!(e.mem.u32(data + 36 + 8), NOT_OWNED);
        // down to one element: the other three are emptied, the owned data is freed
        e.call_log = Some(vec![]);
        e.call(0x00e0_f710, &args![this, 1u32, 0u32]);
        assert_eq!(e.mem.u32(this + 4), 1);
        assert!(calls_to(&e, ARRAY_RESERVE).is_empty());
        // the first element survives with its data, the second one's data was not owned
        assert_eq!(e.mem.u32(data), owned);
        assert!(calls_to(&e, FREE_DATA).is_empty());
        e.mem.set_u32(this + 4, 4);
        e.mem.set_u32(data + 12, borrowed);
        e.mem.set_u32(data + 12 + 8, 3);
        e.call_log = Some(vec![]);
        e.call(0x00e0_f710, &args![this, 0u32, 1u32]);
        assert_eq!(
            calls_to(&e, FREE_DATA),
            vec![vec![allocator, borrowed, 3 * 0x24]]
        );
        assert_eq!(e.mem.u32(data + 12), 0);
        assert_eq!(e.mem.u32(data + 12 + 8), NOT_OWNED);
    }

    /// An analyzer with `threads` empty thread arrays (no frame records) and a frame buffer.
    fn analyzer(e: &mut Engine, threads: u32) -> u32 {
        let this = e.mem.alloc(0x30);
        let arrays = e.mem.alloc(12 * threads.max(1));
        for thread in 0..threads {
            e.mem.set_u32(arrays + 12 * thread + 8, NOT_OWNED);
        }
        e.mem.set_u32(this, arrays);
        e.mem.set_u32(this + 4, threads);
        let buffer = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0xc, buffer);
        this
    }

    #[test]
    fn the_tree_of_all_the_frames_of_a_thread_has_one_child_per_frame_record() {
        let mut e = engine();
        let this = analyzer(&mut e, 2);
        let arrays = e.mem.u32(this);
        // thread 1 has two frame records, each with an empty stream (begin = end)
        let records = e.mem.alloc(0x24 * 2);
        let (first, second) = (text(&mut e, "frame 0"), text(&mut e, "frame 1"));
        e.mem.set_u32(records, first);
        e.mem.set_u32(records + 0x24, second);
        e.mem.set_u32(arrays + 12, records);
        e.mem.set_u32(arrays + 12 + 4, 2);
        let root = e.call(0x00e0_ee00, &args![this, 1u32, 0u32]).u32();
        assert_eq!(e.mem.u32(root + NODE_NAME), 0x0106_3cbc);
        assert_eq!(e.mem.u32(root + NODE_KIND), 1);
        assert_eq!(child_count(&e, root), 2);
        for (index, heading) in [first, second].into_iter().enumerate() {
            let child = child_at(&e, root, index as u32);
            assert_eq!(e.mem.u32(child + NODE_NAME), heading);
            assert_eq!(e.mem.u32(child + NODE_KIND), 1);
        }
        // thread 0 has no records
        let empty = e.call(0x00e0_ee00, &args![this, 0u32, 0u32]).u32();
        assert_eq!(child_count(&e, empty), 0);
    }

    /// A tag string: the tag character, a flag byte and the name from byte 2.
    fn tag(e: &mut Engine, tag: u8, name: &str) -> u32 {
        let at = e.mem.alloc(name.len() as u32 + 3);
        e.mem.set_u8(at, tag);
        e.mem.set_u8(at + 1, b' ');
        e.mem.set_cstr(at + 2, name.as_bytes());
        at
    }

    fn put_words(e: &mut Engine, words: &[u32]) -> (u32, u32) {
        let begin = e.mem.alloc(4 * words.len() as u32 + 4);
        for (index, word) in words.iter().enumerate() {
            e.mem.set_u32(begin + 4 * index as u32, *word);
        }
        (begin, begin + 4 * words.len() as u32)
    }

    /// The command record `Node::setTimers` reads: the two value indexes, the mode and the rates.
    fn timer_command(e: &mut Engine, indexes: (u32, u32), mode: u32, rates: (f32, f32)) -> u32 {
        let command = e.mem.alloc(0x18);
        e.mem.set_u32(command + 4, indexes.0);
        e.mem.set_u32(command + 8, indexes.1);
        e.mem.set_u32(command + 0xc, mode);
        e.mem.set_f32(command + 0x10, rates.0);
        e.mem.set_f32(command + 0x14, rates.1);
        command
    }

    /// Runs `makeStatisticsTreeForSingleFrame` on `words` with timer indexes 0 and 1, rates 1.0.
    fn single_frame(e: &mut Engine, words: &[u32], flag: u32) -> u32 {
        let command = timer_command(e, (0, 1), 2, (1.0, 1.0));
        let (begin, end) = put_words(e, words);
        let heading = text(e, "Frame");
        e.call(0x00e0_e630, &args![begin, end, command, heading, flag])
            .u32()
    }

    fn first_two(e: &Engine, node: u32) -> ([f32; 2], [u16; 2]) {
        let (values, counts) = values_of(e, node);
        ([values[0], values[1]], [counts[0], counts[1]])
    }

    #[test]
    fn a_timer_begin_and_end_add_the_elapsed_ticks_to_a_child_of_the_root() {
        let mut e = engine();
        let (open, close) = (tag(&mut e, b'T', "Physics"), tag(&mut e, b'E', "Physics"));
        let command = timer_command(&mut e, (0, 1), 2, (1.0, 1.0));
        let (begin, end) = put_words(&mut e, &[open, 1000, 2000, close, 1500, 2600]);
        let heading = text(&mut e, "Frame 0");
        e.call_log = Some(vec![]);
        let root = e
            .call(0x00e0_e630, &args![begin, end, command, heading, 0u32])
            .u32();
        assert_eq!(e.mem.u32(root + NODE_NAME), heading);
        assert_eq!(e.mem.u32(root + NODE_KIND), 1);
        assert_eq!(child_count(&e, root), 1);
        let child = child_at(&e, root, 0);
        assert_eq!(e.mem.u32(child + NODE_NAME), open + 2);
        assert_eq!(e.mem.u32(child + 0x48), root);
        assert_eq!(first_two(&e, child), ([500.0, 600.0], [1, 1]));
        // the stacks live in the frame: nothing is freed
        assert!(calls_to(&e, FREE_DATA).is_empty());
    }

    #[test]
    fn nested_timers_make_nested_nodes_and_an_end_with_the_wrong_name_stops() {
        let mut e = engine();
        let outer_open = tag(&mut e, b'T', "A");
        let inner_open = tag(&mut e, b'T', "B");
        let inner_close = tag(&mut e, b'E', "B");
        let outer_close = tag(&mut e, b'E', "A");
        let root = single_frame(
            &mut e,
            &[
                outer_open,
                100,
                200,
                inner_open,
                110,
                210,
                inner_close,
                150,
                250,
                outer_close,
                180,
                300,
            ],
            0,
        );
        let outer = child_at(&e, root, 0);
        let inner = child_at(&e, outer, 0);
        assert_eq!(first_two(&e, outer), ([80.0, 100.0], [1, 1]));
        assert_eq!(first_two(&e, inner), ([40.0, 40.0], [1, 1]));
        assert_eq!(e.mem.u32(inner + 0x48), outer);
        // an end that does not name the open timer ends the build: nothing is added
        let wrong = tag(&mut e, b'E', "Other");
        let root = single_frame(&mut e, &[outer_open, 100, 200, wrong, 180, 300], 0);
        assert_eq!(first_two(&e, child_at(&e, root, 0)), ([0.0, 0.0], [0, 0]));
        // an end with an empty name matches any timer
        let blank = tag(&mut e, b'E', "");
        let root = single_frame(&mut e, &[outer_open, 100, 200, blank, 130, 260], 0);
        assert_eq!(first_two(&e, child_at(&e, root, 0)), ([30.0, 60.0], [1, 1]));
        // an end with nothing open ends the build
        let root = single_frame(&mut e, &[outer_close, 1, 2, outer_open, 1, 2], 0);
        assert_eq!(child_count(&e, root), 0);
    }

    #[test]
    fn a_split_ends_the_timer_and_starts_a_sibling() {
        let mut e = engine();
        let open = tag(&mut e, b'T', "A");
        let split = tag(&mut e, b'S', "B");
        let close = tag(&mut e, b'E', "B");
        let root = single_frame(
            &mut e,
            &[open, 100, 200, split, 130, 230, close, 160, 260],
            0,
        );
        assert_eq!(child_count(&e, root), 2);
        let (first, second) = (child_at(&e, root, 0), child_at(&e, root, 1));
        assert_eq!(e.mem.u32(second + NODE_NAME), split + 2);
        assert_eq!(first_two(&e, first), ([30.0, 30.0], [1, 1]));
        assert_eq!(first_two(&e, second), ([30.0, 30.0], [1, 1]));
        // a split with nothing open ends the build
        let root = single_frame(&mut e, &[split, 1, 2, open, 1, 2], 0);
        assert_eq!(child_count(&e, root), 0);
    }

    #[test]
    fn a_value_adds_to_a_child_of_kind_two_and_counts_it() {
        let mut e = engine();
        let value = tag(&mut e, b'M', "Hits");
        let words = [value, 1.5f32.to_bits(), value, 2.5f32.to_bits()];
        // searching: the second value finds the first one's node
        let root = single_frame(&mut e, &words, 1);
        assert_eq!(child_count(&e, root), 1);
        let child = child_at(&e, root, 0);
        assert_eq!(e.mem.u32(child + NODE_KIND), 2);
        assert_eq!(first_two(&e, child), ([4.0, 0.0], [2, 0]));
        // without searching each value makes a node
        let root = single_frame(&mut e, &words, 0);
        assert_eq!(child_count(&e, root), 2);
        assert_eq!(first_two(&e, child_at(&e, root, 0)), ([1.5, 0.0], [1, 0]));
        assert_eq!(first_two(&e, child_at(&e, root, 1)), ([2.5, 0.0], [1, 0]));
    }

    #[test]
    fn a_group_goes_down_a_kind_one_node_and_a_pop_goes_back_up() {
        let mut e = engine();
        let group = tag(&mut e, b'P', "group");
        let pop = tag(&mut e, b'p', "");
        let timer = tag(&mut e, b'T', "inside");
        let end_timer = tag(&mut e, b'E', "inside");
        let after = tag(&mut e, b'T', "after");
        // group, a timer inside it, pop to the root, a timer there; the second pop is
        // at the root, which has no parent: the build ends
        let root = single_frame(
            &mut e,
            &[
                group, timer, 10, 20, end_timer, 15, 26, pop, after, 1, 2, pop, pop, timer, 1, 2,
            ],
            0,
        );
        assert_eq!(child_count(&e, root), 2);
        let (first, second) = (child_at(&e, root, 0), child_at(&e, root, 1));
        assert_eq!(e.mem.u32(first + NODE_KIND), 1);
        assert_eq!(e.mem.u32(second + NODE_NAME), after + 2);
        assert_eq!(child_count(&e, first), 1);
        assert_eq!(first_two(&e, child_at(&e, first, 0)), ([5.0, 6.0], [1, 1]));
    }

    #[test]
    fn a_double_timer_makes_a_child_and_a_grandchild_that_two_ends_close() {
        let mut e = engine();
        let outer = tag(&mut e, b'L', "Outer");
        let inner = tag(&mut e, b'S', "Inner");
        let close = tag(&mut e, b'l', "");
        let root = single_frame(&mut e, &[outer, 100, 200, inner, close, 150, 250], 0);
        assert_eq!(child_count(&e, root), 1);
        let first = child_at(&e, root, 0);
        assert_eq!(e.mem.u32(first + NODE_NAME), outer + 2);
        let second = child_at(&e, first, 0);
        assert_eq!(e.mem.u32(second + NODE_NAME), inner + 2);
        // the inner one began at the same ticks as the outer one
        assert_eq!(first_two(&e, second), ([50.0, 50.0], [1, 1]));
        assert_eq!(first_two(&e, first), ([50.0, 50.0], [1, 1]));
        // `l` needs two records on the stack, the top one a split
        let root = single_frame(&mut e, &[close, 1, 2, outer, 1, 2, 1, 2], 0);
        assert_eq!(child_count(&e, root), 0);
        let (timer_a, timer_b) = (tag(&mut e, b'T', "a"), tag(&mut e, b'T', "b"));
        let root = single_frame(&mut e, &[timer_a, 1, 2, timer_b, 1, 2, close, 3, 4], 0);
        // the top record is a `T`, not a split: the build ends with every node empty
        let first = child_at(&e, root, 0);
        assert_eq!(first_two(&e, first), ([0.0, 0.0], [0, 0]));
        assert_eq!(first_two(&e, child_at(&e, first, 0)), ([0.0, 0.0], [0, 0]));
    }

    #[test]
    fn a_named_timer_uses_the_name_word_itself() {
        let mut e = engine();
        let wrapper = tag(&mut e, b'O', "Wrapper");
        let close = tag(&mut e, b'E', "Wrapper");
        let name = text(&mut e, "Named");
        let root = single_frame(&mut e, &[wrapper, 100, 200, name, close, 150, 250], 0);
        assert_eq!(child_count(&e, root), 1);
        let child = child_at(&e, root, 0);
        assert_eq!(e.mem.u32(child + NODE_NAME), name);
        assert_eq!(first_two(&e, child), ([50.0, 50.0], [1, 1]));
    }

    #[test]
    fn skipped_commands_move_on_by_their_size_and_an_unknown_tag_ends_the_build() {
        let mut e = engine();
        let skip = tag(&mut e, b'N', "");
        let memory = tag(&mut e, b'm', "");
        let timer = tag(&mut e, b'T', "z");
        let unknown = tag(&mut e, b'Z', "");
        let other = tag(&mut e, b'T', "y");
        // N: one word, a word below 10: one word, m: three words
        let root = single_frame(
            &mut e,
            &[
                skip, 5, memory, 0xaa, 0xbb, timer, 1, 2, unknown, other, 1, 2,
            ],
            0,
        );
        assert_eq!(child_count(&e, root), 1);
        assert_eq!(e.mem.u32(child_at(&e, root, 0) + NODE_NAME), timer + 2);
        // an empty stream gives just the root
        let root = single_frame(&mut e, &[], 0);
        assert_eq!(child_count(&e, root), 0);
    }

    #[test]
    fn the_search_flag_reuses_a_child_of_the_same_name() {
        let mut e = engine();
        let (open, close) = (tag(&mut e, b'T', "x"), tag(&mut e, b'E', "x"));
        let words = [open, 1, 2, close, 3, 4, open, 5, 6, close, 7, 8];
        let root = single_frame(&mut e, &words, 0x100);
        // only the low byte of the flag counts: 0, so two children
        assert_eq!(child_count(&e, root), 2);
        let root = single_frame(&mut e, &words, 0x01);
        assert_eq!(child_count(&e, root), 1);
        assert_eq!(first_two(&e, child_at(&e, root, 0)), ([4.0, 4.0], [2, 2]));
    }

    #[test]
    fn deep_nesting_grows_the_stacks_past_their_inline_room_and_frees_them() {
        let mut e = engine();
        let mut words = Vec::new();
        let opens: Vec<u32> = (0..20)
            .map(|depth| tag(&mut e, b'T', &format!("n{depth}")))
            .collect();
        let closes: Vec<u32> = (0..20)
            .map(|depth| tag(&mut e, b'E', &format!("n{depth}")))
            .collect();
        for depth in 0..20 {
            words.extend([opens[depth], 10, 20]);
        }
        for depth in (0..20).rev() {
            words.extend([closes[depth], 15, 26]);
        }
        let router = router_of(&mut e);
        let allocator = e.mem.u32(router + 0x10);
        e.call_log = Some(vec![]);
        let root = single_frame(&mut e, &words, 0);
        let mut node = root;
        for depth in 0..20 {
            assert_eq!(child_count(&e, node), 1, "depth {depth}");
            node = child_at(&e, node, 0);
            assert_eq!(first_two(&e, node), ([5.0, 6.0], [1, 1]));
        }
        // both stacks grew from 16 to 32 elements of 12 bytes: each is freed
        let freed = calls_to(&e, FREE_DATA);
        assert_eq!(freed.len(), 2);
        for call in &freed {
            assert_eq!(call[0], allocator);
            assert_eq!(call[2], 32 * 12);
        }
    }

    /// Records every block the allocator hands out, as `(size, address)`.
    fn record_allocations(e: &mut Engine) -> Rc<RefCell<Vec<(u32, u32)>>> {
        let blocks = Rc::new(RefCell::new(Vec::new()));
        let seen = blocks.clone();
        e.register_double(ALLOCATE, move |e, a| {
            let at = e.mem.alloc(a[1]);
            seen.borrow_mut().push((a[1], at));
            Ret {
                eax: at,
                ..Ret::default()
            }
        });
        blocks
    }

    fn nodes_allocated(blocks: &Rc<RefCell<Vec<(u32, u32)>>>) -> Vec<u32> {
        blocks
            .borrow()
            .iter()
            .filter(|block| block.0 == NODE_SIZE)
            .map(|block| block.1)
            .collect()
    }

    #[test]
    fn the_tga_dump_builds_the_trees_writes_the_image_and_destroys_the_trees() {
        let mut e = engine();
        let blocks = record_allocations(&mut e);
        let images = Rc::new(RefCell::new(Vec::new()));
        let seen = images.clone();
        e.register_double(EASY_ALLOC, move |e, a| {
            let image = e.mem.alloc(a[1].max(8));
            seen.borrow_mut().push(image);
            Ret {
                eax: image,
                ..Ret::default()
            }
        });
        let this = analyzer(&mut e, 1);
        let layout = e.mem.alloc(0x30);
        e.mem.set_u32(layout + 0x14, 16);
        let stream = e.mem.alloc(8);
        let router = router_of(&mut e);
        let allocator = e.mem.u32(router + 0x10);
        e.call_log = Some(vec![]);
        e.call(0x00e0_f5e0, &args![this, layout, stream]);
        let trees = nodes_allocated(&blocks);
        assert_eq!(trees.len(), 1);
        let image = images.borrow()[0];
        // the image has 16 columns and no rows: the header and no pixels
        let writes = calls_to(&e, STREAM_WRITE);
        assert_eq!(writes.len(), 2);
        assert_eq!(writes[0][0], stream);
        assert_eq!(writes[0][2], 0x12);
        assert_eq!(writes[1], vec![stream, image, 0]);
        assert_eq!(calls_to(&e, EASY_FREE), vec![vec![allocator, image]]);
        assert_eq!(
            calls_to(&e, FREE_BLOCK),
            vec![vec![allocator, trees[0], NODE_SIZE]]
        );
    }

    #[test]
    fn write_statistics_prints_the_version_and_destroys_one_tree_per_thread() {
        let mut e = engine();
        let blocks = record_allocations(&mut e);
        let this = analyzer(&mut e, 2);
        let name = text(&mut e, "Main");
        e.mem.set_u32(this + 0x18, 1);
        e.mem.set_u32(this + 0x20, name);
        let stream = e.mem.alloc(8);
        let router = router_of(&mut e);
        let allocator = e.mem.u32(router + 0x10);
        e.call_log = Some(vec![]);
        // no flags: only the heading
        e.call(0x00e1_0630, &args![this, stream, 0u32]);
        assert_eq!(
            calls_to(&e, PRINTF),
            vec![vec![stream, 0x010e_ac8c, 0x010c_f830]]
        );
        let trees = nodes_allocated(&blocks);
        assert_eq!(trees.len(), 2);
        assert_eq!(
            calls_to(&e, FREE_BLOCK),
            vec![
                vec![allocator, trees[0], NODE_SIZE],
                vec![allocator, trees[1], NODE_SIZE]
            ]
        );
        // the array of trees had grown to 4 elements: its data is freed
        let freed = calls_to(&e, FREE_DATA);
        assert_eq!(freed.len(), 1);
        assert_eq!(freed[0][2], 16);
    }

    /// `printf` as events: the format, the length of the indent string when it has one, and
    /// the arguments after it.
    type PrintEvent = (u32, usize, Vec<u32>);

    fn record_printf(e: &mut Engine) -> Rc<RefCell<Vec<PrintEvent>>> {
        let events = Rc::new(RefCell::new(Vec::new()));
        let seen = events.clone();
        e.register_double(PRINTF, move |e, a| {
            let indented = [
                0x010e_a7d4u32,
                0x010e_a7bc,
                0x010e_a784,
                0x010e_a768,
                0x010e_a7a0,
                0x010e_a754,
                0x010e_a724,
            ];
            if indented.contains(&a[1]) {
                let length = e.mem.cstr(a[2]).len();
                seen.borrow_mut().push((a[1], length, a[3..].to_vec()));
            } else {
                seen.borrow_mut().push((a[1], 0, a[2..].to_vec()));
            }
            Ret::default()
        });
        events
    }

    #[test]
    fn the_raw_dump_prints_every_command_with_its_indent() {
        let mut e = engine();
        let events = record_printf(&mut e);
        let tags = (
            tag(&mut e, b'T', "ta"),
            tag(&mut e, b'L', "lo"),
            tag(&mut e, b'M', "mv"),
            tag(&mut e, b'N', ""),
        );
        let moves = (tag(&mut e, b'm', "mm"), tag(&mut e, b'm', "mn"));
        let (split, plain, back, close) = (
            tag(&mut e, b'S', "sx"),
            tag(&mut e, b'S', "sp"),
            tag(&mut e, b'l', "le"),
            tag(&mut e, b'E', "ee"),
        );
        let words = [
            tags.0,
            1,
            2, // T
            tags.1,
            3,
            4,
            split, // L
            tags.2,
            1.5f32.to_bits(), // M
            tags.3,           // N
            moves.0,
            0xa000,
            0x8000_0010, // m: a free
            moves.1,
            0xb000,
            0x20, // m: an allocation
            plain,
            5,
            6, // S
            back,
            9,
            10, // l
            close,
            11,
            12, // E
        ];
        let (begin, end) = put_words(&mut e, &words);
        let buffer = begin;
        let this = analyzer(&mut e, 1);
        let records = e.mem.alloc(0x24);
        let heading = text(&mut e, "Thread 3");
        e.mem.set_u32(records, heading);
        e.mem.set_u32(records + 0xc, 77);
        e.mem.set_u32(records + 0x18, 3);
        e.mem.set_u32(records + 0x1c, 0);
        e.mem.set_u32(records + 0x20, end - begin);
        let arrays = e.mem.u32(this);
        e.mem.set_u32(arrays, records);
        e.mem.set_u32(arrays + 4, 1);
        e.mem.set_u32(this + 0xc, buffer);
        e.mem.set_u32(this + 0x18, 4);
        e.mem.set_u32(this + 0x1c, 2);
        let stream = e.mem.alloc(8);
        let router = router_of(&mut e);
        let allocator = e.mem.u32(router + 0x14);
        e.call_log = Some(vec![]);
        e.call(0x00e0_eef0, &args![this, stream]);
        let value = double_words(1.5);
        assert_eq!(
            *events.borrow(),
            vec![
                (0x010e_a838, 0, vec![4, 2, 1]),
                (0x010e_a7f0, 0, vec![heading, 0, 3, 77]),
                (0x010e_a7d4, 0, vec![tags.0 + 2, 1, 2]),
                (0x010e_a7a0, 2, vec![tags.1 + 2, 3, 4]),
                (0x010e_a784, 2, vec![split + 2, 3, 4]),
                (0x010e_a754, 4, vec![tags.2 + 1, value[0], value[1]]),
                (0x010e_a724, 4, vec![0x010e_a74c, moves.0 + 1, 0xa000, 0x10]),
                (0x010e_a724, 4, vec![0x010e_a744, moves.1 + 1, 0xb000, 0x20]),
                (0x010e_a784, 2, vec![plain + 2, 5, 6]),
                (0x010e_a768, 2, vec![back + 2, 9, 10]),
                (0x010e_a7bc, 0, vec![close + 2, 11, 12]),
            ]
        );
        // the indent string had room for 0x40 characters and is freed
        let freed = calls_to(&e, FREE_DATA);
        assert_eq!(freed.len(), 1);
        assert_eq!((freed[0][0], freed[0][2]), (allocator, 0x40));
    }

    #[test]
    fn the_raw_dump_ends_at_an_unknown_tag_and_still_frees_its_indent() {
        let mut e = engine();
        let events = record_printf(&mut e);
        let (open, unknown) = (tag(&mut e, b'T', "a"), tag(&mut e, b'Z', ""));
        let (begin, end) = put_words(&mut e, &[open, 1, 2, unknown, open, 3, 4]);
        let this = analyzer(&mut e, 1);
        let records = e.mem.alloc(0x24 * 2);
        let heading = text(&mut e, "h");
        for record in [records, records + 0x24] {
            e.mem.set_u32(record, heading);
            e.mem.set_u32(record + 0x20, end - begin);
        }
        let arrays = e.mem.u32(this);
        e.mem.set_u32(arrays, records);
        e.mem.set_u32(arrays + 4, 2);
        e.mem.set_u32(this + 0xc, begin);
        let stream = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        e.call(0x00e0_eef0, &args![this, stream]);
        // the dump line, the first record's heading, its timer, and then the unknown tag:
        // the second record is never reached
        let formats: Vec<u32> = events.borrow().iter().map(|event| event.0).collect();
        assert_eq!(formats, vec![0x010e_a838, 0x010e_a7f0, 0x010e_a7d4]);
        assert_eq!(calls_to(&e, FREE_DATA).len(), 1);
    }

    /// A store for the map `hkBuildSizePerName` uses: the entries by the content of their key.
    type SizeMapEntries = Rc<RefCell<Vec<(Vec<u8>, u32)>>>;

    fn install_size_map(e: &mut Engine) -> SizeMapEntries {
        let entries: SizeMapEntries = Rc::new(RefCell::new(Vec::new()));
        let found = entries.clone();
        e.register_double(SIZE_MAP_GET, move |e, a| {
            let key = e.mem.cstr(a[1]);
            let value = found
                .borrow()
                .iter()
                .find(|entry| entry.0 == key)
                .map_or(a[2], |entry| entry.1);
            Ret {
                eax: value,
                ..Ret::default()
            }
        });
        let inserted = entries.clone();
        e.register_double(SIZE_MAP_INSERT, move |e, a| {
            let key = e.mem.cstr(a[1]);
            let mut entries = inserted.borrow_mut();
            entries.push((key, a[2]));
            let count = entries.len() as u32;
            e.mem.set_u32(a[0] + 4, count);
            Ret::default()
        });
        entries
    }

    fn entry_named(entries: &SizeMapEntries, name: &str) -> u32 {
        entries
            .borrow()
            .iter()
            .find(|entry| entry.0 == name.as_bytes())
            .map(|entry| entry.1)
            .unwrap()
    }

    #[test]
    fn the_size_per_name_entry_sums_a_name_and_lists_its_children_and_parent() {
        let mut e = engine();
        let entries = install_size_map(&mut e);
        let up = named(&mut e, "up");
        let root = named(&mut e, "root");
        let kid = named(&mut e, "kid");
        for (node, value, count) in [(up, 3.0f32, 3u16), (root, 1.0, 1), (kid, 2.0, 2)] {
            e.mem.set_f32(node, value);
            e.mem.set_u16(node + COUNTS, count);
        }
        adopt(&mut e, up, root);
        adopt(&mut e, root, kid);
        let map = e.mem.alloc(16);
        e.call(0x00e0_f880, &args![root, map]);
        assert_eq!(entries.borrow().len(), 2);
        let root_entry = entry_named(&entries, "root");
        assert_eq!(
            e.mem.u32(root_entry + NODE_NAME),
            e.mem.u32(root + NODE_NAME)
        );
        assert_eq!(values_of(&e, root_entry).0[0], 1.0);
        assert_eq!(values_of(&e, root_entry).1[0], 1);
        // its children array (at +0x64) has the kid, its parent array (at +0x58) the parent
        assert_eq!(e.mem.u32(root_entry + 0x68), 1);
        let kid_node = e.mem.u32(root_entry + 0x64);
        assert_eq!(e.mem.u32(kid_node + NODE_NAME), e.mem.u32(kid + NODE_NAME));
        assert_eq!(values_of(&e, kid_node).0[0], 2.0);
        assert_eq!(values_of(&e, kid_node).1[0], 2);
        assert_eq!(e.mem.u32(root_entry + 0x5c), 1);
        let up_node = e.mem.u32(root_entry + 0x58);
        assert_eq!(e.mem.u32(up_node + NODE_NAME), e.mem.u32(up + NODE_NAME));
        assert_eq!(values_of(&e, up_node).0[0], 3.0);
        // the kid's own entry lists the root as its parent and nothing below it
        let kid_entry = entry_named(&entries, "kid");
        assert_eq!(e.mem.u32(kid_entry + 0x5c), 1);
        assert_eq!(values_of(&e, e.mem.u32(kid_entry + 0x58)).0[0], 1.0);
        assert_eq!(e.mem.u32(kid_entry + 0x68), 0);
        // doing it again finds the entries and adds to the same nodes
        e.call(0x00e0_f880, &args![root, map]);
        assert_eq!(entries.borrow().len(), 2);
        assert_eq!(values_of(&e, root_entry).0[0], 2.0);
        assert_eq!(e.mem.u32(root_entry + 0x68), 1);
        assert_eq!(values_of(&e, kid_node).0[0], 4.0);
        assert_eq!(values_of(&e, kid_node).1[0], 4);
    }

    /// The map iteration of `printStatisticsByType` over the entries of [`install_size_map`],
    /// and a sort of the entries by their float at `+4`.
    fn install_entry_walk(e: &mut Engine, entries: &SizeMapEntries) {
        let cleared = entries.clone();
        e.register_double(SIZE_MAP_NEW, move |e, a| {
            cleared.borrow_mut().clear();
            e.mem.set_u32(a[0] + 4, 0);
            Ret::default()
        });
        e.register(SIZE_MAP_ITERATOR, |_, _| Ret::default());
        let valid = entries.clone();
        e.register_double(SIZE_MAP_IS_VALID, move |e, a| {
            e.mem
                .set_u8(a[1], u8::from((a[2] as usize) < valid.borrow().len()));
            Ret::default()
        });
        let values = entries.clone();
        e.register_double(SIZE_MAP_VALUE, move |_, a| Ret {
            eax: values.borrow()[a[1] as usize].1,
            ..Ret::default()
        });
        e.register(SIZE_MAP_NEXT, |_, a| Ret {
            eax: a[1] + 1,
            ..Ret::default()
        });
        e.register(SIZE_MAP_DELETE, |_, _| Ret::default());
        e.register(QUICK_SORT, |e, a| {
            let mut sorted: Vec<u32> = (a[1]..=a[2])
                .map(|index| e.mem.u32(a[0] + 4 * index))
                .collect();
            sorted.sort_by(|x, y| e.mem.f32(x + 4).partial_cmp(&e.mem.f32(y + 4)).unwrap());
            for (index, entry) in sorted.into_iter().enumerate() {
                e.mem.set_u32(a[0] + 4 * (a[1] + index as u32), entry);
            }
            Ret::default()
        });
        e.register(0x00dd_eff0, |_, _| Ret::default());
    }

    #[test]
    fn the_by_type_summary_prints_the_big_entries_and_frees_every_entry() {
        let mut e = engine();
        let entries = install_size_map(&mut e);
        install_entry_walk(&mut e, &entries);
        let events = record_printf(&mut e);
        let root = named(&mut e, "a");
        let (big, small) = (named(&mut e, "b"), named(&mut e, "c"));
        e.mem.set_f32(root + 4, 100.0);
        e.mem.set_f32(big + 4, 40.0);
        e.mem.set_f32(small + 4, 2.0);
        adopt(&mut e, root, big);
        adopt(&mut e, root, small);
        let stream = e.mem.alloc(8);
        let router = router_of(&mut e);
        let allocator = e.mem.u32(router + 0x14);
        e.call_log = Some(vec![]);
        // a threshold of a half: only the 100.0 entry is above 50.0
        e.call(0x00e0_fc50, &args![stream, root, 0.5f32]);
        let formats: Vec<u32> = events.borrow().iter().map(|event| event.0).collect();
        assert_eq!(
            formats[..4],
            [0x0109_5188, 0x010e_aab0, 0x010e_aa88, 0x010e_aab0]
        );
        assert_eq!(formats.iter().filter(|f| **f == 0x010e_aa60).count(), 1);
        assert_eq!(formats.iter().filter(|f| **f == 0x010e_aa5c).count(), 2);
        // the entries are freed from the biggest down: a, b, c
        let order = [
            entry_named(&entries, "a"),
            entry_named(&entries, "b"),
            entry_named(&entries, "c"),
        ];
        assert_eq!(
            calls_to(&e, FREE_BLOCK),
            order
                .iter()
                .map(|entry| vec![allocator, *entry, 0x70])
                .collect::<Vec<_>>()
        );
    }

    /// How a test names the text stream the summaries are built in.
    const TEXT: u32 = 0xaaaa;

    type Event = (u32, Vec<u32>);

    /// The calls that build the text: character puts, `printf` and string puts, in order, with
    /// the address of every text stream that was made replaced by [`TEXT`].
    fn text_events(e: &Engine) -> Vec<Event> {
        let text_streams: Vec<u32> = calls_to(e, TEXT_STREAM_NEW)
            .iter()
            .map(|call| call[0])
            .collect();
        e.call_log
            .clone()
            .unwrap()
            .into_iter()
            .filter(|call| [STREAM_PUT_CHAR, PRINTF, STREAM_PUT_STRING].contains(&call.0))
            .map(|(address, mut words)| {
                if text_streams.contains(&words[0]) {
                    words[0] = TEXT;
                }
                (address, words)
            })
            .collect()
    }

    fn spaces(count: usize) -> Vec<Event> {
        vec![(STREAM_PUT_CHAR, vec![TEXT, 0x20]); count]
    }

    /// A summary config: first column, column width, indent, value indent; no marks, no tabs.
    fn summary_config(
        e: &mut Engine,
        first: u32,
        width: u32,
        indent: u32,
        value_indent: u32,
    ) -> u32 {
        let config = e.mem.alloc(0x30);
        e.mem.set_u32(config, first);
        e.mem.set_u32(config + 4, width);
        e.mem.set_u32(config + 8, indent);
        e.mem.set_u32(config + 0xc, value_indent);
        config
    }

    #[test]
    fn a_summary_line_has_the_name_the_values_and_the_thread_average() {
        let mut e = engine();
        let name = text(&mut e, "n");
        let node = node(
            &mut e,
            name,
            [1.5, 2.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            [3, 4, 0, 0, 0, 0, 0, 0],
        );
        let config = summary_config(&mut e, 4, 8, 2, 3);
        let stream = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        e.call(0x00e0_d2c0, &args![stream, node, 1i32, 2i32, 0i32, config]);
        let (first, second, average) = (double_words(1.5), double_words(2.5), double_words(2.0));
        let mut expected: Vec<Event> = vec![(PRINTF, vec![TEXT, 0x010e_a6cc, name, 7])];
        expected.extend(spaces(4));
        expected.push((PRINTF, vec![TEXT, 0x010e_a6c0, first[0], first[1], 3]));
        expected.extend(spaces(12));
        expected.push((PRINTF, vec![TEXT, 0x010e_a6c0, second[0], second[1], 4]));
        expected.extend(spaces(20));
        expected.push((PRINTF, vec![TEXT, 0x010e_a6c0, average[0], average[1], 7]));
        expected.push((STREAM_PUT_STRING, vec![stream, 0]));
        expected.push((STREAM_PUT_CHAR, vec![stream, 0xa]));
        assert_eq!(text_events(&e), expected);
        // the text stream is made and destroyed around it
        assert_eq!(calls_to(&e, TEXT_STREAM_DELETE).len(), 1);
    }

    #[test]
    fn a_summary_line_with_spus_adds_their_average_too() {
        let mut e = engine();
        let name = text(&mut e, "n");
        let node = node(
            &mut e,
            name,
            [1.0, 2.0, 3.0, 4.0, 0.0, 0.0, 0.0, 0.0],
            [1; 8],
        );
        let config = summary_config(&mut e, 0, 1, 0, 0);
        e.mem.set_u8(config + 0x1a, 1);
        let stream = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        // one thread and three spus: the threads' values are 1.0, the spus' 2.0, 3.0, 4.0
        e.call(0x00e0_d2c0, &args![stream, node, 1i32, 1i32, 3i32, config]);
        let events = text_events(&e);
        let tabs = events
            .iter()
            .filter(|event| *event == &(STREAM_PUT_CHAR, vec![TEXT, 9]))
            .count();
        // one tab in front of each of the 4 columns and one for the spu average
        assert_eq!(tabs, 5);
        let averages = double_words(3.0);
        let last_printf = events
            .iter()
            .rev()
            .find(|event| event.0 == PRINTF)
            .unwrap()
            .clone();
        // total count of the 4 columns is 4; the spu average is (2 + 3 + 4) / 3 with 3 counts
        assert_eq!(
            last_printf.1,
            vec![TEXT, 0x010e_a6c0, averages[0], averages[1], 3]
        );
    }

    #[test]
    fn marks_choose_the_characters_and_closed_nodes_are_not_entered() {
        let mut e = engine();
        let name = text(&mut e, "n");
        let parent = node(
            &mut e,
            name,
            [1.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            [3; 8],
        );
        let kid_name = text(&mut e, "kid");
        let child = node(&mut e, kid_name, [0.0; 8], [0; 8]);
        adopt(&mut e, parent, child);
        let config = summary_config(&mut e, 4, 8, 2, 3);
        e.mem.set_u8(config + 0x10, 1);
        e.mem.set_u32(config + 0x14, parent);
        e.mem.set_u8(config + 0x18, b'*');
        e.mem.set_u8(config + 0x19, b'+');
        let stream = e.mem.alloc(8);
        let value = double_words(1.5);
        let mut line: Vec<Event> = vec![(STREAM_PUT_CHAR, vec![TEXT, 0x2a])];
        line.extend(spaces(2));
        line.push((STREAM_PUT_CHAR, vec![TEXT, 0x2a]));
        line.push((PRINTF, vec![TEXT, 0x010e_a6cc, name, 3]));
        line.extend(spaces(7));
        line.push((PRINTF, vec![TEXT, 0x010e_a6c0, value[0], value[1], 3]));
        line.push((STREAM_PUT_STRING, vec![stream, 0]));
        line.push((STREAM_PUT_CHAR, vec![stream, 0xa]));
        // the node's flag bit 0 is clear: it is closed, its mark is the one at +0x18
        e.call_log = Some(vec![]);
        e.call(
            0x00e0_d2c0,
            &args![stream, parent, 2i32, 1i32, 0i32, config],
        );
        assert_eq!(text_events(&e), line);
        // open: the mark at +0x19, and the child's line follows
        e.mem.set_u32(parent + NODE_FLAGS, 1);
        e.call_log = Some(vec![]);
        e.call(
            0x00e0_d2c0,
            &args![stream, parent, 2i32, 1i32, 0i32, config],
        );
        let events = text_events(&e);
        let mut open_line = line.clone();
        open_line[3] = (STREAM_PUT_CHAR, vec![TEXT, 0x2b]);
        assert_eq!(events[..open_line.len()], open_line[..]);
        let rest = &events[open_line.len()..];
        // the child is one level deeper (indent 4, the value column at 6 + 4), it is not the
        // marked node and has no children
        let zero = double_words(0.0);
        let mut child_line: Vec<Event> = vec![(STREAM_PUT_CHAR, vec![TEXT, 0x20])];
        child_line.extend(spaces(4));
        child_line.push((STREAM_PUT_CHAR, vec![TEXT, 0x20]));
        child_line.push((PRINTF, vec![TEXT, 0x010e_a6cc, kid_name, 0]));
        child_line.extend(spaces(10));
        child_line.push((PRINTF, vec![TEXT, 0x010e_a6c0, zero[0], zero[1], 0]));
        child_line.push((STREAM_PUT_STRING, vec![stream, 0]));
        child_line.push((STREAM_PUT_CHAR, vec![stream, 0xa]));
        assert_eq!(rest, &child_line[..]);
    }

    #[test]
    fn depth_zero_prints_only_the_children() {
        let mut e = engine();
        let name = text(&mut e, "root");
        let root = node(&mut e, name, [0.0; 8], [0; 8]);
        let kid_name = text(&mut e, "kid");
        let child = node(&mut e, kid_name, [0.0; 8], [0; 8]);
        adopt(&mut e, root, child);
        let config = summary_config(&mut e, 0, 8, 2, 3);
        let stream = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        e.call(0x00e0_d2c0, &args![stream, root, 0i32, 1i32, 0i32, config]);
        let events = text_events(&e);
        let strings = events
            .iter()
            .filter(|event| event.0 == STREAM_PUT_STRING)
            .count();
        assert_eq!(strings, 1);
        assert_eq!(events[0].1[1..], [0x010e_a6cc, kid_name, 0]);
        // one text stream per node, the root's is made and never written to
        assert_eq!(calls_to(&e, TEXT_STREAM_NEW).len(), 2);
    }

    #[test]
    fn the_heading_lists_the_thread_and_spu_columns_and_the_averages() {
        let mut e = engine();
        let root = named(&mut e, "abc");
        let config = summary_config(&mut e, 0, 4, 2, 0);
        let stream = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        e.call(0x00e0_d640, &args![root, 2i32, 1i32, stream, config]);
        // the first column is after the longest name: 3 + 8
        assert_eq!(e.mem.u32(config), 11);
        let mut expected: Vec<Event> = vec![(PRINTF, vec![TEXT, 0x010e_a704])];
        expected.extend(spaces(11));
        expected.push((PRINTF, vec![TEXT, 0x010e_a6f8, 0]));
        expected.extend(spaces(15));
        expected.push((PRINTF, vec![TEXT, 0x010e_a6f8, 1]));
        expected.extend(spaces(19));
        expected.push((PRINTF, vec![TEXT, 0x010e_a6f0, 0]));
        expected.extend(spaces(23));
        expected.push((PRINTF, vec![TEXT, 0x010e_a6e4]));
        expected.push((STREAM_PUT_STRING, vec![stream, 0]));
        expected.push((PRINTF, vec![stream, 0x0109_5188]));
        assert_eq!(text_events(&e), expected);
        assert_eq!(calls_to(&e, TEXT_STREAM_DELETE).len(), 2);
    }

    #[test]
    fn the_heading_uses_tabs_when_asked_and_has_no_columns_for_one_thread() {
        let mut e = engine();
        let root = named(&mut e, "abc");
        let config = summary_config(&mut e, 0, 4, 2, 0);
        e.mem.set_u8(config + 0x1a, 1);
        let stream = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        e.call(0x00e0_d640, &args![root, 1i32, 2i32, stream, config]);
        let tab = (STREAM_PUT_CHAR, vec![TEXT, 9]);
        let expected: Vec<Event> = vec![
            (PRINTF, vec![TEXT, 0x010e_a704]),
            tab.clone(),
            (PRINTF, vec![TEXT, 0x010e_a6f8, 0]),
            tab.clone(),
            (PRINTF, vec![TEXT, 0x010e_a6f0, 0]),
            tab.clone(),
            (PRINTF, vec![TEXT, 0x010e_a6f0, 1]),
            tab,
            (PRINTF, vec![TEXT, 0x010e_a6d8]),
            (STREAM_PUT_STRING, vec![stream, 0]),
            (PRINTF, vec![stream, 0x0109_5188]),
        ];
        assert_eq!(text_events(&e), expected);
        // one thread and no spus: only the name heading
        e.call_log = Some(vec![]);
        e.call(0x00e0_d640, &args![root, 1i32, 0i32, stream, config]);
        let expected: Vec<Event> = vec![
            (PRINTF, vec![TEXT, 0x010e_a704]),
            (STREAM_PUT_STRING, vec![stream, 0]),
            (PRINTF, vec![stream, 0x0109_5188]),
        ];
        assert_eq!(text_events(&e), expected);
    }

    /// One thread whose tree has a frame node for each entry of `frames`, each with a timer
    /// child for each `(start, value)`, with what `getTimerLimits` reads around them.
    struct Limits {
        trees: u32,
        infos: u32,
        range: u32,
        max_out: u32,
        starts: u32,
    }

    fn limits_scene(e: &mut Engine, frames: &[&[(f64, f32)]], first: u32, wanted: u32) -> Limits {
        let tree = named(e, "tree");
        for timers in frames {
            let frame = named(e, "frame");
            adopt(e, tree, frame);
            for (start, value) in timers.iter() {
                let timer = named(e, "timer");
                e.mem.set_f64(timer + START_TIME, *start);
                e.mem.set_f32(timer, *value);
                adopt(e, frame, timer);
            }
        }
        let trees = e.mem.alloc(12);
        let data = e.mem.alloc(4);
        e.mem.set_u32(data, tree);
        e.mem.set_u32(trees, data);
        e.mem.set_u32(trees + 4, 1);
        let records = e.mem.alloc(0x24 * frames.len().max(1) as u32);
        let threads = e.mem.alloc(12);
        e.mem.set_u32(threads, records);
        e.mem.set_u32(threads + 4, frames.len() as u32);
        let infos = e.mem.alloc(4);
        e.mem.set_u32(infos, threads);
        let range = e.mem.alloc(0x30);
        e.mem.set_u32(range, first);
        e.mem.set_u32(range + 4, wanted);
        let max_out = e.mem.alloc(4);
        let starts = e.mem.alloc(12);
        e.mem.set_u32(starts + 8, NOT_OWNED);
        Limits {
            trees,
            infos,
            range,
            max_out,
            starts,
        }
    }

    fn run_limits(e: &mut Engine, limits: &Limits) {
        e.call(
            0x00e0_dd30,
            &args![
                limits.trees,
                limits.range,
                limits.infos,
                limits.max_out,
                limits.starts
            ],
        );
    }

    fn start_values(e: &Engine, limits: &Limits) -> Vec<f32> {
        let data = e.mem.u32(limits.starts);
        (0..e.mem.u32(limits.starts + 4))
            .map(|index| e.mem.f32(data + 4 * index))
            .collect()
    }

    #[test]
    fn the_limits_are_the_earliest_start_and_the_longest_run_of_each_frame() {
        let mut e = engine();
        let frames: [&[(f64, f32)]; 2] = [&[(100.0, 10.0), (105.0, 30.0)], &[(200.0, 5.0)]];
        let limits = limits_scene(&mut e, &frames, 0, 2);
        e.mem.set_f32(limits.max_out, 123.0);
        run_limits(&mut e, &limits);
        assert_eq!(start_values(&e, &limits), vec![100.0, 200.0]);
        // 30 + 105 - 100
        assert_eq!(e.mem.f32(limits.max_out), 35.0);
    }

    #[test]
    fn the_start_array_has_the_wanted_count_even_when_there_are_fewer_frames() {
        let mut e = engine();
        let frames: [&[(f64, f32)]; 2] = [&[(100.0, 10.0)], &[(200.0, 5.0)]];
        let limits = limits_scene(&mut e, &frames, 0, 5);
        run_limits(&mut e, &limits);
        assert_eq!(start_values(&e, &limits), vec![100.0, 200.0, 0.0, 0.0, 0.0]);
        assert_eq!(e.mem.f32(limits.max_out), 10.0);
        // no frames left from the first one asked for: nothing is touched
        let frames: [&[(f64, f32)]; 1] = [&[(1.0, 1.0)]];
        let limits = limits_scene(&mut e, &frames, 1, 3);
        e.mem.set_f32(limits.max_out, 123.0);
        run_limits(&mut e, &limits);
        assert!(start_values(&e, &limits).is_empty());
        assert_eq!(e.mem.f32(limits.max_out), 123.0);
    }

    #[test]
    fn a_fixed_limit_replaces_the_longest_run() {
        let mut e = engine();
        let frames: [&[(f64, f32)]; 2] = [&[(100.0, 10.0)], &[(200.0, 5.0)]];
        let limits = limits_scene(&mut e, &frames, 0, 2);
        e.mem.set_f32(limits.range + 0x18, 7.5);
        run_limits(&mut e, &limits);
        assert_eq!(e.mem.f32(limits.max_out), 7.5);
        assert_eq!(start_values(&e, &limits), vec![100.0, 200.0]);
    }

    #[test]
    fn a_padding_moves_every_start_four_at_a_time_and_then_one_at_a_time() {
        let mut e = engine();
        let frames: [&[(f64, f32)]; 5] = [
            &[(0.0, 1.0)],
            &[(10.0, 1.0)],
            &[(20.0, 1.0)],
            &[(30.0, 1.0)],
            &[(40.0, 1.0)],
        ];
        let limits = limits_scene(&mut e, &frames, 0, 5);
        e.mem.set_f32(limits.range + 0x1c, 2.5);
        run_limits(&mut e, &limits);
        assert_eq!(start_values(&e, &limits), vec![2.5, 12.5, 22.5, 32.5, 42.5]);
        assert_eq!(e.mem.f32(limits.max_out), 1.0);
        // a NaN padding is not above 0.0
        let limits = limits_scene(&mut e, &frames, 0, 5);
        e.mem.set_f32(limits.range + 0x1c, f32::NAN);
        run_limits(&mut e, &limits);
        assert_eq!(start_values(&e, &limits), vec![0.0, 10.0, 20.0, 30.0, 40.0]);
    }

    #[test]
    fn the_limit_of_a_single_frame_follows_the_float_global() {
        // 35 / 1000 is 0, the next power of two is 1: the target 1000 is far below the global
        let mut e = engine();
        let frames: [&[(f64, f32)]; 1] = [&[(0.0, 35.0)]];
        let limits = limits_scene(&mut e, &frames, 0, 1);
        run_limits(&mut e, &limits);
        assert_eq!(e.mem.f32(limits.max_out), 19050.0);
        assert_eq!(e.mem.f32(SMOOTH_LIMIT), 19050.0);
        // 3e6 / 1000 is 3000: the next power of two is 4096, the move is capped at a quarter
        let mut e = engine();
        let frames: [&[(f64, f32)]; 1] = [&[(0.0, 3.0e6)]];
        let limits = limits_scene(&mut e, &frames, 0, 1);
        run_limits(&mut e, &limits);
        assert_eq!(e.mem.f32(limits.max_out), 20250.0);
        assert_eq!(e.mem.f32(SMOOTH_LIMIT), 20250.0);
        // the limit is not allowed below 16666
        let mut e = engine();
        e.mem.set_f32(SMOOTH_LIMIT, 10000.0);
        let frames: [&[(f64, f32)]; 1] = [&[(0.0, 35.0)]];
        let limits = limits_scene(&mut e, &frames, 0, 1);
        run_limits(&mut e, &limits);
        assert_eq!(e.mem.f32(limits.max_out), 16666.0);
        assert_eq!(e.mem.f32(SMOOTH_LIMIT), 16666.0);
    }

    /// A map double for `00cb1980` and `00cb1870` that remembers `(map, key) -> value` and
    /// logs the inserts as `(map, key, value)`.
    fn install_color_maps(e: &mut Engine) -> Rc<RefCell<Vec<(u32, u32, u32)>>> {
        let inserted = Rc::new(RefCell::new(Vec::<(u32, u32, u32)>::new()));
        let entries = inserted.clone();
        e.register_double(MAP_GET, move |e, a| {
            match entries
                .borrow()
                .iter()
                .find(|entry| entry.0 == a[0] && entry.1 == a[1])
            {
                Some(entry) => {
                    e.mem.set_u32(a[2], entry.2);
                    Ret::default()
                }
                None => Ret {
                    eax: 1,
                    ..Ret::default()
                },
            }
        });
        let entries = inserted.clone();
        e.register_double(MAP_INSERT, move |_, a| {
            entries.borrow_mut().push((a[0], a[2], a[3]));
            Ret::default()
        });
        inserted
    }

    #[test]
    fn a_strip_gets_the_colour_of_the_node_sampled_at_each_column() {
        let mut e = engine();
        let inserted = install_color_maps(&mut e);
        // a frame node whose timers start at 10 and 20 (the sampled tree of the other tests)
        let (frame_node, first_timer) = sampled_tree(&mut e);
        let tree = named(&mut e, "tree");
        adopt(&mut e, tree, frame_node);
        let names = e.mem.alloc(0x14);
        let pairs = e.mem.alloc(8);
        let first_name = e.mem.u32(first_timer + NODE_NAME);
        let listed = text(&mut e, "FIRST");
        // the bit 0 of the string pointer is masked off
        e.mem.set_u32(pairs, listed | 1);
        e.mem.set_u32(pairs + 4, 0xff11_2233);
        e.mem.set_u32(names + 8, pairs);
        e.mem.set_u32(names + 0xc, 1);
        let selector = e.mem.alloc(0x10);
        let out = e.mem.alloc(40);
        let colors = e.mem.alloc(12);
        let router = router_of(&mut e);
        let allocator = e.mem.u32(router + 0x10);
        e.call_log = Some(vec![]);
        e.call(
            0x00e0_daf0,
            &args![
                selector,
                0u32,
                tree + CHILDREN,
                out,
                2i32,
                names,
                5u32,
                5i32,
                20.0f32,
                2.0f32,
                colors
            ],
        );
        // sampled: nothing, nothing, the first timer, then the frame node twice
        let expected = [
            0xffff_ffffu32,
            0xffff_ffff,
            0xff11_2233,
            0xffff_ffff,
            0xffff_ffff,
        ];
        for row in 0..2 {
            for (column, color) in expected.iter().enumerate() {
                let at = out + 4 * (row * 5 + column as u32);
                assert_eq!(e.mem.u32(at), *color, "row {row} column {column}");
            }
        }
        let local_map = calls_to(&e, COLOR_MAP_RESERVE)[0][0];
        assert_eq!(
            calls_to(&e, COLOR_MAP_RESERVE),
            vec![vec![local_map, allocator, 0]]
        );
        // a match goes in the local map; names without one are recorded in the shared map
        let frame_name = e.mem.u32(frame_node + NODE_NAME);
        let tree_name = e.mem.u32(tree + NODE_NAME);
        assert_eq!(
            *inserted.borrow(),
            vec![
                (local_map, first_name, 0xff11_2233),
                (colors, frame_name, 0xffff_ffff),
                (colors, tree_name, 0xffff_ffff),
            ]
        );
        assert_eq!(
            calls_to(&e, COLOR_MAP_CLEAR),
            vec![vec![local_map, allocator]]
        );
        assert_eq!(calls_to(&e, COLOR_MAP_DELETE), vec![vec![local_map]]);
        // the sample array (5 words) is freed
        let freed = calls_to(&e, FREE_DATA);
        assert_eq!(freed.len(), 1);
        assert_eq!(freed[0][2], 20);
    }

    #[test]
    fn a_strip_of_a_frame_without_timers_is_left_alone() {
        let mut e = engine();
        install_color_maps(&mut e);
        let frame_node = named(&mut e, "frame");
        let tree = named(&mut e, "tree");
        adopt(&mut e, tree, frame_node);
        let names = e.mem.alloc(0x14);
        let selector = e.mem.alloc(0x10);
        let out = e.mem.alloc(40);
        e.mem.set_u32(out, 0x1234);
        let colors = e.mem.alloc(12);
        e.call(
            0x00e0_daf0,
            &args![
                selector,
                0u32,
                tree + CHILDREN,
                out,
                2i32,
                names,
                5u32,
                5i32,
                20.0f32,
                2.0f32,
                colors
            ],
        );
        assert_eq!(e.mem.u32(out), 0x1234);
    }

    /// The scene of `writeStatisticsDetailsToTexture`: one thread with one frame holding one
    /// timer `slice` that starts at 0 and runs 50, a fixed limit of 100, a strip 4 high, a gap
    /// of 2 under the strip and 2 under the frame, 40 pixels wide.
    struct Scene {
        trees: u32,
        layout: u32,
        analyzer: u32,
    }

    fn texture_scene(e: &mut Engine) -> Scene {
        let tree = named(e, "tree");
        let frame = named(e, "frame");
        adopt(e, tree, frame);
        let timer = named(e, "slice");
        e.mem.set_f32(timer, 50.0);
        adopt(e, frame, timer);
        let trees = e.mem.alloc(12);
        let data = e.mem.alloc(4);
        e.mem.set_u32(data, tree);
        e.mem.set_u32(trees, data);
        e.mem.set_u32(trees + 4, 1);
        let records = e.mem.alloc(0x24);
        let arrays = e.mem.alloc(12);
        e.mem.set_u32(arrays, records);
        e.mem.set_u32(arrays + 4, 1);
        let analyzer = e.mem.alloc(4);
        e.mem.set_u32(analyzer, arrays);
        let layout = e.mem.alloc(0x30);
        e.mem.set_u32(layout + 4, 1);
        e.mem.set_u32(layout + 8, 4);
        e.mem.set_u32(layout + 0xc, 2);
        e.mem.set_u32(layout + 0x10, 2);
        e.mem.set_u32(layout + 0x14, 40);
        e.mem.set_f32(layout + 0x18, 100.0);
        let names = e.mem.alloc(0x14);
        let pairs = e.mem.alloc(8);
        let listed = text(e, "slice");
        e.mem.set_u32(pairs, listed);
        e.mem.set_u32(pairs + 4, 0xff11_2233);
        e.mem.set_u32(names + 8, pairs);
        e.mem.set_u32(names + 0xc, 1);
        e.mem.set_u32(layout + 0x24, names);
        install_color_maps(e);
        Scene {
            trees,
            layout,
            analyzer,
        }
    }

    #[test]
    fn the_texture_has_the_strip_the_red_tick_and_the_green_frame_ticks() {
        let mut e = engine();
        let scene = texture_scene(&mut e);
        let (image_out, height_out, limit_out) = (e.mem.alloc(4), e.mem.alloc(4), e.mem.alloc(4));
        let router = router_of(&mut e);
        let allocator = e.mem.u32(router + 0x10);
        e.call_log = Some(vec![]);
        e.call(
            0x00e0_e0c0,
            &args![
                scene.trees,
                scene.layout,
                scene.analyzer,
                image_out,
                height_out,
                limit_out
            ],
        );
        // (4 + 2) * 1 thread + 2, for one frame: 8 rows; (40 + 1) * 8 * 4 bytes
        assert_eq!(e.mem.u32(height_out), 8);
        assert_eq!(e.mem.f32(limit_out), 100.0);
        let image = e.mem.u32(image_out);
        assert_eq!(calls_to(&e, EASY_ALLOC), vec![vec![allocator, 41 * 8 * 4]]);
        assert_eq!(
            calls_to(&e, MEM_SET).last().unwrap().clone(),
            vec![image, 0, 41 * 8 * 4]
        );
        for row in 0..8u32 {
            for column in 0..40u32 {
                let pixel = e.mem.u32(image + 4 * (40 * row + column));
                let expected = if column < 2 {
                    0xff00_ff00
                } else if row < 4 && column < 20 {
                    0xff11_2233
                } else {
                    0
                };
                assert_eq!(pixel, expected, "row {row} column {column}");
            }
        }
        // the colour maps of the strip and of the whole texture are done with
        assert_eq!(calls_to(&e, COLOR_MAP_DELETE).len(), 2);
    }

    #[test]
    fn the_texture_of_no_threads_is_not_made() {
        let mut e = engine();
        let trees = e.mem.alloc(12);
        let (image_out, height_out) = (e.mem.alloc(4), e.mem.alloc(4));
        e.mem.set_u32(image_out, 0x77);
        e.call_log = Some(vec![]);
        e.call(
            0x00e0_e0c0,
            &args![trees, 0u32, 0u32, image_out, height_out, 0u32],
        );
        assert_eq!(e.mem.u32(image_out), 0x77);
        assert!(calls_to(&e, EASY_ALLOC).is_empty());
    }

    #[test]
    fn the_tga_gets_the_header_the_pixels_and_the_image_is_freed() {
        let mut e = engine();
        let scene = texture_scene(&mut e);
        let writes = Rc::new(RefCell::new(Vec::new()));
        let seen = writes.clone();
        e.register_double(STREAM_WRITE, move |e, a| {
            seen.borrow_mut()
                .push((a[1], a[2], e.mem.bytes(a[1], 0x12)));
            Ret::default()
        });
        let stream = e.mem.alloc(8);
        let router = router_of(&mut e);
        let allocator = e.mem.u32(router + 0x10);
        e.call_log = Some(vec![]);
        e.call(
            0x00e0_e470,
            &args![scene.trees, scene.layout, scene.analyzer, stream, 0u32],
        );
        let writes = writes.borrow();
        assert_eq!(writes.len(), 2);
        assert_eq!(writes[0].1, 0x12);
        assert_eq!(writes[0].2[0xc..0x10], [40, 0, 8, 0]);
        assert_eq!(writes[1].1, 40 * 8 * 4);
        assert_eq!(calls_to(&e, EASY_FREE), vec![vec![allocator, writes[1].0]]);
    }

    /// One thread's tree: a root with a frame node for each entry of `frames`, each frame with
    /// the timers `(name, value, count)`.
    fn thread_tree(e: &mut Engine, frames: &[&[(&str, f32, u16)]]) -> u32 {
        let tree = named(e, "thread");
        for (index, timers) in frames.iter().enumerate() {
            let frame = named(e, &format!("frame {index}"));
            adopt(e, tree, frame);
            for (name, value, count) in timers.iter() {
                let timer = named(e, name);
                e.mem.set_f32(timer, *value);
                e.mem.set_u16(timer + COUNTS, *count);
                adopt(e, frame, timer);
            }
        }
        tree
    }

    /// The `hkArray` of trees `writeStatisticsDetails` takes.
    fn tree_array(e: &mut Engine, trees: &[u32]) -> u32 {
        let array = e.mem.alloc(12);
        let data = e.mem.alloc(4 * trees.len().max(1) as u32);
        for (index, tree) in trees.iter().enumerate() {
            e.mem.set_u32(data + 4 * index as u32, *tree);
        }
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 4, trees.len() as u32);
        e.mem.set_u32(array + 8, trees.len() as u32 | NOT_OWNED);
        array
    }

    const FRAMES: [&[(&str, f32, u16)]; 2] = [
        &[("Main", 10.0, 1), ("Other", 2.0, 1)],
        &[("Main", 30.0, 1), ("Other", 4.0, 1)],
    ];

    /// `writeStatisticsDetails` of one thread with [`FRAMES`]: the events of the stream.
    fn run_details(
        e: &mut Engine,
        trees: &[u32],
        threads: i32,
        spus: i32,
        flags: u32,
        combine: u8,
    ) -> u32 {
        let array = tree_array(e, trees);
        let stream = e.mem.alloc(8);
        let name = text(e, "Main");
        e.call_log = Some(vec![]);
        e.call(
            0x00e0_fec0,
            &args![stream, array, threads, spus, flags, name, combine],
        );
        stream
    }

    fn formats_of(events: &Rc<RefCell<Vec<PrintEvent>>>) -> Vec<u32> {
        events.borrow().iter().map(|event| event.0).collect()
    }

    fn arguments_of(events: &Rc<RefCell<Vec<PrintEvent>>>, format: u32) -> Vec<Vec<u32>> {
        events
            .borrow()
            .iter()
            .filter(|event| event.0 == format)
            .map(|event| event.2.clone())
            .collect()
    }

    #[test]
    fn the_total_times_are_the_frames_added_up_and_printed_with_the_average_scale() {
        let mut e = engine();
        let events = record_printf(&mut e);
        let tree = thread_tree(&mut e, &FRAMES);
        let stream = run_details(&mut e, &[tree], 1, 0, 1, 1);
        assert_eq!(
            formats_of(&events)[..6],
            [
                0x010e_ac8c,
                0x0109_5188,
                0x010e_ac64,
                0x010e_ac40,
                0x010e_ac64,
                0x010e_ac24
            ]
        );
        assert_eq!(arguments_of(&events, 0x010e_ac8c), vec![vec![0x010c_f830]]);
        // the children are Main (10 + 30) and Other (2 + 4), each seen twice: the text
        // shows 0.5 * 2 and the values are halved
        let scaled: Vec<Vec<u32>> = calls_to(&e, SNPRINTF)
            .into_iter()
            .filter(|call| call[2] == 0x010e_a6b4)
            .collect();
        assert_eq!(scaled.len(), 2);
        let one = double_words(1.0);
        for call in &scaled {
            assert_eq!(call[4..], one);
        }
        let (main, other) = (double_words(20.0), double_words(3.0));
        assert_eq!(
            arguments_of(&events, 0x010e_a6a0),
            vec![main.to_vec(), other.to_vec()]
        );
        assert_eq!(calls_to(&e, PRINTF)[0][0], stream);
    }

    #[test]
    fn no_flags_print_only_the_version_line() {
        let mut e = engine();
        let events = record_printf(&mut e);
        let tree = thread_tree(&mut e, &FRAMES);
        run_details(&mut e, &[tree], 1, 0, 0, 1);
        assert_eq!(formats_of(&events), vec![0x010e_ac8c]);
    }

    /// `printf` calls with the text behind the first argument after the stream, when it is a
    /// string.
    fn record_texts(e: &mut Engine) -> Rc<RefCell<Vec<(Vec<u32>, String)>>> {
        let events = Rc::new(RefCell::new(Vec::new()));
        let seen = events.clone();
        e.register_double(PRINTF, move |e, a| {
            let shown = if a.len() == 2 && e.mem.is_mapped(a[1]) {
                String::from_utf8_lossy(&e.mem.cstr(a[1])).into_owned()
            } else {
                String::new()
            };
            seen.borrow_mut().push((a.to_vec(), shown));
            Ret::default()
        });
        events
    }

    #[test]
    fn the_ascii_art_has_a_bar_for_each_frame() {
        let mut e = engine();
        let events = record_texts(&mut e);
        let tree = thread_tree(&mut e, &FRAMES);
        let (first_frame, second_frame) = (child_at(&e, tree, 0), child_at(&e, tree, 1));
        let first_name = e.mem.u32(child_at(&e, first_frame, 0) + NODE_NAME);
        let second_name = e.mem.u32(child_at(&e, second_frame, 0) + NODE_NAME);
        let stream = run_details(&mut e, &[tree], 1, 0, 2, 1);
        let events = events.borrow();
        // the version line and the five headings, then a bar and a line for each frame
        assert_eq!(events.len(), 6 + 4);
        let ten = double_words(10.0);
        let thirty = double_words(30.0);
        // the total is the average of Main over the frames, 20: the bars are at 10 and 30
        assert_eq!(events[6].1, format!("\t\t  # {}", "\t".repeat(7)));
        assert_eq!(
            events[7].0,
            vec![stream, 0x010e_abd0, 0, first_name, ten[0], ten[1]]
        );
        assert_eq!(events[8].1, format!("{}  # \t\t", "\t".repeat(7)));
        assert_eq!(
            events[9].0,
            vec![stream, 0x010e_abd0, 1, second_name, thirty[0], thirty[1]]
        );
    }

    #[test]
    fn the_summaries_of_one_thread_list_each_frame_and_can_add_the_by_type_summary() {
        let mut e = engine();
        let events = record_printf(&mut e);
        let tree = thread_tree(&mut e, &FRAMES);
        let (first_frame, second_frame) = (child_at(&e, tree, 0), child_at(&e, tree, 1));
        let names = [
            e.mem.u32(first_frame + NODE_NAME),
            e.mem.u32(second_frame + NODE_NAME),
        ];
        run_details(&mut e, &[tree], 1, 0, 4, 1);
        assert_eq!(
            arguments_of(&events, 0x010e_ab58),
            vec![vec![0, 0], vec![1, 0]]
        );
        assert_eq!(
            arguments_of(&events, 0x010c_e5cc),
            vec![vec![names[0]], vec![names[1]]]
        );
        assert!(arguments_of(&events, 0x010e_aa88).is_empty());
        // with bit 3 each of them is followed by the by-type summary
        let entries = install_size_map(&mut e);
        install_entry_walk(&mut e, &entries);
        e.register(0x00dd_eff0, |_, _| Ret::default());
        let events = record_printf(&mut e);
        run_details(&mut e, &[tree], 1, 0, 4 | 8, 1);
        assert_eq!(arguments_of(&events, 0x010e_aa88).len(), 2);
    }

    #[test]
    fn the_summaries_skip_frames_a_thread_does_not_have() {
        let mut e = engine();
        let events = record_printf(&mut e);
        let long = thread_tree(&mut e, &FRAMES);
        let short = thread_tree(&mut e, &[&[("Main", 1.0, 1)]]);
        // several threads but no combining: one summary per frame and thread that exists
        run_details(&mut e, &[long, short], 2, 0, 4, 0);
        assert_eq!(
            arguments_of(&events, 0x010e_ab58),
            vec![vec![0, 0], vec![0, 1], vec![1, 0]]
        );
    }

    #[test]
    fn the_details_name_threads_and_spus() {
        let mut e = engine();
        let events = record_printf(&mut e);
        let first = thread_tree(&mut e, &FRAMES);
        let second = thread_tree(&mut e, &FRAMES);
        // the first thread is a thread, the second one a spu
        run_details(&mut e, &[first, second], 1, 1, 0x10, 1);
        assert_eq!(
            arguments_of(&events, 0x010e_ab00),
            vec![vec![0, 0], vec![1, 0]]
        );
        assert_eq!(
            arguments_of(&events, 0x010e_aad8),
            vec![vec![0, 0], vec![1, 0]]
        );
        // the order is frame by frame
        let formats = formats_of(&events);
        let named_lines: Vec<u32> = formats
            .into_iter()
            .filter(|format| [0x010e_ab00, 0x010e_aad8].contains(format))
            .collect();
        assert_eq!(
            named_lines,
            vec![0x010e_ab00, 0x010e_aad8, 0x010e_ab00, 0x010e_aad8]
        );
        // with no thread count every tree is a spu, counted from minus the number of threads
        let events = record_printf(&mut e);
        run_details(&mut e, &[first], 0, 1, 0x10, 1);
        assert_eq!(
            arguments_of(&events, 0x010e_aad8),
            vec![vec![0, 0], vec![1, 0]]
        );
        let events = record_printf(&mut e);
        run_details(&mut e, &[first, second], 3, 0, 0x10, 1);
        assert_eq!(arguments_of(&events, 0x010e_ab00).len(), 4);
    }

    #[test]
    fn several_threads_are_combined_in_nodes_from_the_bump_allocator() {
        let mut e = engine();
        let events = record_printf(&mut e);
        let blocks = record_allocations(&mut e);
        e.register(0x00dd_eff0, |_, _| Ret::default());
        let router = router_of(&mut e);
        let allocator = e.mem.u32(router + 0x10);
        let pool = e.mem.alloc(0x40);
        e.mem.set_u32(router, pool);
        let first = thread_tree(&mut e, &FRAMES[..1]);
        let second = thread_tree(&mut e, &FRAMES[..1]);
        run_details(&mut e, &[first, second], 2, 0, 4, 1);
        // one summary of the combined frame, built by the heading and the lines
        assert_eq!(arguments_of(&events, 0x010e_ab84), vec![vec![0]]);
        assert!(!arguments_of(&events, 0x010e_a704).is_empty());
        // the bump pointer is back where it was
        assert_eq!(e.mem.u32(router), pool);
        // the two nodes of the sum of the frames, then each combined node followed by the two
        // children made in it: the combined ones are destroyed
        let nodes = nodes_allocated(&blocks);
        assert_eq!(nodes.len(), 8);
        assert_eq!(
            calls_to(&e, FREE_BLOCK),
            vec![
                vec![allocator, nodes[2], NODE_SIZE],
                vec![allocator, nodes[5], NODE_SIZE]
            ]
        );
    }

    #[test]
    fn a_total_that_is_not_above_zero_becomes_a_thousand() {
        let mut e = engine();
        let events = record_texts(&mut e);
        // Main adds up to 0 over the frames: the total falls back to 1000.0
        let tree = thread_tree(&mut e, &[&[("Main", 500.0, 1)], &[("Main", -500.0, 1)]]);
        run_details(&mut e, &[tree], 1, 0, 2, 1);
        let events = events.borrow();
        assert_eq!(events.len(), 6 + 4);
        // 500 * 20 / 1000 is 10: the same bar as before; a negative value is cut to 0
        assert_eq!(events[6].1, format!("\t\t  # {}", "\t".repeat(7)));
        assert_eq!(events[8].1, format!("#   {}", "\t".repeat(9)));
    }
}

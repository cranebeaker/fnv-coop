//! `Havok SDK/hkstatisticsprocess.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00ddc840 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00dd_c0b0`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00ddc840(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00dd_c0b0, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00ddc830 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x8` (`SUB ECX,0x8`) and jumps to `00ddc840`, which (`RET 4`) has the shape of a scalar deleting destructor: it reads bit 0 of the one stack word. The caller gets its `EAX`; the class is unconfirmed.
pub fn fn_00ddc830(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00dd_c840, &args![this.addr().wrapping_sub(0x8), flags])
        .u32()
}

/// The vtables of `hkStatisticsProcess` (RTTI-confirmed, `.?AVhkStatisticsProcess@@`): the whole
/// object's and the `hkProcess` part's (at `+8`); `hkProcess`'s own vtable and `hkBaseObject`'s.
const STATISTICS_PROCESS_VTABLE: u32 = 0x010d_7e48;
const STATISTICS_PROCESS_PROCESS_VTABLE: u32 = 0x010d_7e28;
const PROCESS_VTABLE: u32 = 0x010d_1508;
const BASE_OBJECT_VTABLE: u32 = 0x0102_e388;
/// The `hkArrayStreamWriter` vtable the step builds its writer with (`010c9368`).
const ARRAY_STREAM_WRITER_VTABLE: u32 = 0x010c_9368;
/// `hkMonitorStreamFrameInfo`'s constructor (`thiscall`, no stack words; 0x24 bytes).
const FRAME_INFO_CONSTRUCTOR: u32 = 0x00e0_c320;
/// `hkStopwatch::getTicksPerSecond` (no arguments; answers `EDX:EAX`).
const GET_TICKS_PER_SECOND: u32 = 0x00d6_0580;
/// `1000.0`, a `double` in the exe's data (the numerator of the milliseconds-per-tick ratio).
const THOUSAND: u32 = 0x0101_7b70;
/// The code that the `hkArrayStreamWriter` is built with and torn down by (`00cc06f0`, named
/// `nullTerminate` by the map: `thiscall`, `(array, flag)`; stores the writer's fields, makes room
/// for one more byte and writes a zero after the array's last byte; `00cc0780` is its
/// destructor).
const ARRAY_STREAM_WRITER_CONSTRUCTOR: u32 = 0x00cc_06f0;
const ARRAY_STREAM_WRITER_DESTRUCTOR: u32 = 0x00cc_0780;
/// `hkStructureLayout::hkStructureLayout_ov2` (`thiscall`: copies the rules word at its argument)
/// and the rules it copies (`010d6ae8`).
const LAYOUT_RULES_COPY: u32 = 0x00dc_1680;
const LAYOUT_RULES: u32 = 0x010d_6ae8;
/// `hkObjectSerialize::writeObject` (cdecl, seven words: writer, layout, 1, 0, object, class, out
/// array) and the class records of the frame info (`0126c860`) and the string map (`0126c830`).
const WRITE_OBJECT: u32 = 0x00e0_bd40;
const FRAME_INFO_CLASS: u32 = 0x0126_c860;
const STRING_MAP_CLASS: u32 = 0x0126_c830;
/// `hkArrayUtil::_reserve` (cdecl: allocator, array, count, element size) and `00c90940` (cdecl:
/// allocator, array, element size; one grow step).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
const ARRAY_RESERVE_MORE: u32 = 0x00c9_0940;
/// `hkMapBase<uint64, uint64>::reserve` (`thiscall`: allocator, count) and `clearAndDeallocate`
/// (`thiscall`: allocator), and the cleanup run on the map afterwards (`00a29680`).
const MAP_RESERVE: u32 = 0x00cb_1fe0;
const MAP_CLEAR_AND_DEALLOCATE: u32 = 0x00cb_1a90;
const MAP_CLEANUP: u32 = 0x00a2_9680;
/// The string map array's constructor (`00e0f6e0`: `{0, 0, 0x80000000}`), its element constructor
/// (`00e0bf20`, a `hkStringPtr` at `+8`), and the monitor stream analyser's range reader
/// (`00e0cd20`, cdecl: start, end, map).
const STRING_MAP_ARRAY_CONSTRUCTOR: u32 = 0x00e0_f6e0;
const STRING_MAP_ELEMENT_CONSTRUCTOR: u32 = 0x00e0_bf20;
const READ_MONITOR_RANGE: u32 = 0x00e0_cd20;
/// `hkStringPtr::operator=` (`thiscall`: one string word).
const STRING_PTR_ASSIGN: u32 = 0x00cb_0c10;
/// `hkOArchive`: `write32u`, `write32` (one word each), `writeRaw` (pointer, length) and the byte
/// writer (`00d605d0`).
const OARCHIVE_WRITE_32U: u32 = 0x00d6_0960;
const OARCHIVE_WRITE_32: u32 = 0x00d6_0940;
const OARCHIVE_WRITE_RAW: u32 = 0x00d6_07b0;
const OARCHIVE_WRITE_BYTE: u32 = 0x00d6_05d0;
/// The process factory registry (`01269cd0`), its `registerProcess` (`00dd9380`, `thiscall`: name,
/// factory), the factory the statistics process registers (`00ddc160`), the name it registers
/// ("Statistics", `010d7e18`) and the word that keeps the answer (`01269cd8`).
const PROCESS_FACTORY_REGISTRY: u32 = 0x0126_9cd0;
const PROCESS_FACTORY_REGISTER: u32 = 0x00dd_9380;
const STATISTICS_PROCESS_FACTORY: u32 = 0x00dd_c160;
const STATISTICS_NAME: u32 = 0x010d_7e18;
const REGISTERED_PROCESS_TAG: u32 = 0x0126_9cd8;
/// The thread-local slot index (`0126835c`) and `TlsGetValue` (`kernel32` import slot, `stdcall`).
const TLS_INDEX: u32 = 0x0126_835c;
const TLS_GET_VALUE: u32 = 0x00fd_f044;
/// `hkStringPtr::~hkStringPtr` (`thiscall`).
const STRING_PTR_DESTRUCTOR: u32 = 0x00cb_0c00;
/// `hkStringMap::~hkStringMap` (`thiscall`).
const STRING_MAP_DESTRUCTOR: u32 = 0x00d8_77d0;

/// The allocator of the memory router (`router + 0x10`).
fn router_allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

/// Gives `array` the empty not-owning state `{0, 0, 0x80000000}` (data, size, capacity and flags).
fn init_array(e: &mut Engine, array: u32) {
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 4, 0);
    e.mem.set_u32(array + 8, 0x8000_0000);
}

/// Makes `dest` (an `hkArray` of words: data, size, capacity and flags) a copy of the words of
/// `source`, as both the constructor and `step` do: when the capacity is below the source's size
/// the old storage is freed (allocator slot `0x10`, if the array owns it) and new storage taken
/// (slot `0xc` with an in/out byte count; the capacity is the granted bytes divided by 4, rounded
/// toward zero), then the size is set and the words copied.
fn copy_words(e: &mut Engine, allocator: u32, dest: u32, source: u32) {
    let flags = e.mem.u32(dest + 8);
    let capacity = (flags & 0x3fff_ffff) as i32;
    let wanted = e.mem.i32(source + 4);
    if capacity < wanted {
        if (flags as i32) >= 0 {
            let data = e.mem.u32(dest);
            e.vcall(
                allocator,
                0x10,
                &args![data, (capacity as u32).wrapping_mul(4)],
            );
        }
        let cell = e.mem.alloc(4);
        e.mem.set_i32(cell, wanted.wrapping_mul(4));
        let block = e.vcall(allocator, 0xc, &args![cell]).u32();
        e.mem.set_u32(dest, block);
        let granted = e.mem.i32(cell);
        e.mem.free(cell);
        // `CDQ; AND EDX,3; ADD EAX,EDX; SAR EAX,2`: a signed division by 4.
        e.mem
            .set_i32(dest + 8, granted.wrapping_add((granted >> 31) & 3) >> 2);
    }
    let size = e.mem.i32(source + 4);
    let destination = e.mem.u32(dest);
    e.mem.set_i32(dest + 4, size);
    let words = e.mem.u32(source);
    if size > 0 {
        for i in 0..size as u32 {
            let word = e.mem.u32(words.wrapping_add(4 * i));
            e.mem.set_u32(destination.wrapping_add(4 * i), word);
        }
    }
}

/// Frees `array`'s storage through the allocator's slot `0x10` when it owns it (flag bit 31
/// clear), for `bytes` bytes.
fn free_if_owned(e: &mut Engine, allocator: u32, array: u32, flags: u32, bytes: u32) {
    if flags & 0x8000_0000 == 0 {
        let data = e.mem.u32(array);
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
}

/// The size of the constructor's frame (00ddbf00: `SUB ESP,0x4c` and four pushes).
const CONSTRUCTOR_FRAME_SIZE: u32 = 0x5c;

// Translated from 00ddbf00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStatisticsProcess::hkStatisticsProcess` (Xbox PDB), `thiscall`, `RET 4`: `source` is an
/// `hkArray` of words (data, size at `+4`). Stores the 16-bit word 1 at `+6`, the `hkProcess`
/// vtable at `+8` then the `hkStatisticsProcess` vtables at `+0` and `+8`, the byte 1 at `+0xc`,
/// and makes three empty arrays at `+0x20` (words), `+0x2c` and `+0x38` (bytes). Then it builds a
/// `hkMonitorStreamFrameInfo` on the stack (`00e0c320`, with its first word, `+4` and `+0xc` cleared and
/// `+8` set to 1), sets its milliseconds-per-tick field (`+0x10`) to `1000.0 / ticks per second`
/// (`hkStopwatch::getTicksPerSecond`, a 64-bit count converted as unsigned through the two
/// `FILD`s: the low 63 bits, and the top bit as `-2^63` negated; the division is x87 and the
/// result is stored as a `float`) and `+0x14` to 1.0, writes that object (class record `0126c860`)
/// through an `hkArrayStreamWriter` onto the array at `+0x2c` into a throw-away array
/// (`hkObjectSerialize::writeObject`), destroys the writer and frees the throw-away array
/// (24-byte elements, when it owns its storage). Finally the array at `+0x20` becomes a copy of
/// `source`'s words. Returns `this`.
pub fn hk_statistics_process_hk_statistics_process(e: &mut Engine, this: Ptr, source: u32) -> Ptr {
    let base = this.addr();
    e.mem.set_u16(base + 6, 1);
    e.mem.set_u32(base + 8, PROCESS_VTABLE);
    e.mem.set_u8(base + 0xc, 1);
    e.mem.set_u32(base, STATISTICS_PROCESS_VTABLE);
    e.mem.set_u32(base + 8, STATISTICS_PROCESS_PROCESS_VTABLE);
    init_array(e, base + 0x20);
    init_array(e, base + 0x2c);
    init_array(e, base + 0x38);
    // The exe's frame (00ddbf00: `SUB ESP,0x4c` and four pushes) holds the layout word at
    // +0x10, the throw-away array at +0x18, the writer at +0x24 and the frame info at +0x38.
    e.with_stack(CONSTRUCTOR_FRAME_SIZE, |e, frame| {
        let frame = frame.addr();
        let (layout, array, writer, frame_info) =
            (frame + 0x10, frame + 0x18, frame + 0x24, frame + 0x38);
        e.call(FRAME_INFO_CONSTRUCTOR, &args![frame_info]);
        e.mem.set_u32(frame_info, 0);
        e.mem.set_u32(frame_info + 0xc, 0);
        e.mem.set_u32(frame_info + 4, 0);
        e.mem.set_u32(frame_info + 8, 1);
        let ticks = e.call(GET_TICKS_PER_SECOND, &args![]).u64();
        let low = ticks as u32;
        let high = (ticks >> 32) as u32;
        // The two `FILD`s: the 63 low bits, and the top bit as an `i64` (`-2^63` or 0) negated.
        let low_part = (u64::from(high & 0x7fff_ffff) << 32 | u64::from(low)) as i64;
        let top_part = (u64::from(high & 0x8000_0000) << 32) as i64;
        let total = x87_add(low_part as f64, -(top_part as f64));
        let numerator = e.mem.f64(THOUSAND);
        let per_tick = x87_store32(x87_div(numerator, total));
        init_array(e, array);
        e.mem.set_u32(frame_info + 0x10, per_tick);
        e.mem.set_u32(frame_info + 0x14, 0x3f80_0000);
        e.call(
            ARRAY_STREAM_WRITER_CONSTRUCTOR,
            &args![writer, base + 0x2c, 1u32],
        );
        e.call(LAYOUT_RULES_COPY, &args![layout, LAYOUT_RULES]);
        e.call(
            WRITE_OBJECT,
            &args![
                writer,
                layout,
                1u32,
                0u32,
                frame_info,
                FRAME_INFO_CLASS,
                array
            ],
        );
        e.call(ARRAY_STREAM_WRITER_DESTRUCTOR, &args![writer]);
        let allocator = router_allocator(e);
        let flags = e.mem.u32(array + 8);
        e.mem.set_u32(array + 4, 0);
        free_if_owned(
            e,
            allocator,
            array,
            flags,
            (flags & 0x3fff_ffff).wrapping_mul(0x18),
        );
    });
    let allocator = router_allocator(e);
    copy_words(e, allocator, base + 0x20, source);
    this
}

// Translated from 00ddc0b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (the destructor body of `hkStatisticsProcess`; the vtables are RTTI-confirmed).
/// `ECX` = `this`, no stack words, plain `RET`. Stores the statistics vtables, then for each of the
/// three arrays (at `+0x38` and `+0x2c`: bytes, at `+0x20`: words) sets the size to 0, frees the
/// storage through the router's allocator (slot `0x10`) when the array owns it (bytes: the
/// capacity; words: four times it) and resets the data to 0 and the capacity word to
/// `0x80000000`. Last it stores the `hkProcess` vtable at `+8` and `hkBaseObject`'s at `+0`.
pub fn fn_00ddc0b0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, STATISTICS_PROCESS_VTABLE);
    e.mem.set_u32(base + 8, STATISTICS_PROCESS_PROCESS_VTABLE);
    for (array, element_size) in [(base + 0x38, 1u32), (base + 0x2c, 1), (base + 0x20, 4)] {
        let allocator = router_allocator(e);
        let flags = e.mem.u32(array + 8);
        e.mem.set_u32(array + 4, 0);
        free_if_owned(
            e,
            allocator,
            array,
            flags,
            (flags & 0x3fff_ffff).wrapping_mul(element_size),
        );
        e.mem.set_u32(array, 0);
        e.mem.set_u32(array + 8, 0x8000_0000);
    }
    e.mem.set_u32(base + 8, PROCESS_VTABLE);
    e.mem.set_u32(base, BASE_OBJECT_VTABLE);
}

// Translated from 00ddc1a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStatisticsProcess::registerProcess` (Xbox PDB), no arguments: registers the process factory
/// `00ddc160` under the name "Statistics" with the process factory registry (the object at
/// `01269cd0`, `hkProcessFactory::registerProcess`) and keeps the answer in the word at
/// `01269cd8`.
pub fn hk_statistics_process_register_process(e: &mut Engine) {
    let registry = e.mem.u32(PROCESS_FACTORY_REGISTRY);
    let tag = e
        .call(
            PROCESS_FACTORY_REGISTER,
            &args![registry, STATISTICS_NAME, STATISTICS_PROCESS_FACTORY],
        )
        .u32();
    e.mem.set_u32(REGISTERED_PROCESS_TAG, tag);
}

/// `hkArray` of words: appends `value` (growing it by one step through `00c90940`, with element
/// size 4, when size equals capacity).
fn push_word(e: &mut Engine, allocator: u32, array: u32, value: u32) {
    let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
    if e.mem.u32(array + 4) == capacity {
        e.call(ARRAY_RESERVE_MORE, &args![allocator, array, 4u32]);
    }
    let size = e.mem.u32(array + 4);
    let data = e.mem.u32(array);
    e.mem
        .set_u32(data.wrapping_add(size.wrapping_mul(4)), value);
    e.mem.set_u32(array + 4, size.wrapping_add(1));
}

/// The first occupied slot of the map `map` (8-byte entries, an entry whose first word is -1 is
/// empty) at or after `from`, looking no further than slot `last`; `last + 1` when there is none.
fn next_occupied_slot(e: &mut Engine, map: u32, from: i32, last: i32) -> i32 {
    let mut slot = from;
    while slot <= last {
        let data = e.mem.u32(map);
        if e.mem.u32(data.wrapping_add((slot as u32).wrapping_mul(8))) != 0xffff_ffff {
            break;
        }
        slot += 1;
    }
    slot
}

/// `step`'s second half: from the two word arrays `a` and `b` (start and end addresses of the
/// monitor stream ranges), fills the map and the string map array and writes the packet to the
/// output archive (`this + 0xc`) or drops the ranges, then frees the arrays. `frame` is the exe's
/// 0x6c-byte frame ([`STEP_FRAME_SIZE`], `ESP` after its `SUB` and four pushes): the layout word at
/// `+0x18`, array `a` at `+0x1c`, `b` at `+0x28`, the map (data, size, hash mask) at `+0x34`, the
/// string map array (16-byte elements) at `+0x40`, the array `c` (24-byte elements) at `+0x4c` and
/// the array stream writer at `+0x58`.
/// The size of `step`'s frame (00ddc1c0: `SUB ESP,0x5c` and four pushes).
const STEP_FRAME_SIZE: u32 = 0x6c;

fn step_write(e: &mut Engine, this: u32, frame: u32) {
    let a = frame + 0x1c;
    let b = frame + 0x28;
    let c = frame + 0x4c;
    let map = frame + 0x34;
    let strings = frame + 0x40;
    let layout = frame + 0x18;
    let writer = frame + 0x58;
    let out = this + 0x30;
    let allocator = router_allocator(e);
    let capacity = e.mem.u32(out + 8) & 0x3fff_ffff;
    // The compiler's test of the masked capacity for a negative number: it never is, so the
    // reserve below is never reached.
    if (capacity as i32) < 0 {
        let doubled = capacity.wrapping_add(capacity) as i32;
        let wanted = if doubled <= 0 { 0 } else { doubled };
        e.call(ARRAY_RESERVE, &args![allocator, out, wanted, 1u32]);
    }
    let mut total: i32 = 0;
    e.mem.set_u32(out + 4, 0);
    e.mem.set_u32(map, 0);
    e.mem.set_u32(map + 4, 0);
    e.mem.set_u32(map + 8, 0xffff_ffff);
    let allocator = router_allocator(e);
    e.call(MAP_RESERVE, &args![map, allocator, 0u32]);
    e.call(STRING_MAP_ARRAY_CONSTRUCTOR, &args![strings]);
    let mut index: u32 = 0;
    if e.mem.i32(a + 4) > 0 {
        loop {
            let start = e.mem.u32(e.mem.u32(a).wrapping_add(4 * index));
            let end = e.mem.u32(e.mem.u32(b).wrapping_add(4 * index));
            e.call(READ_MONITOR_RANGE, &args![start, end, map]);
            let end = e.mem.u32(e.mem.u32(b).wrapping_add(4 * index));
            let start = e.mem.u32(e.mem.u32(a).wrapping_add(4 * index));
            index += 1;
            total = total.wrapping_add(end.wrapping_sub(start) as i32);
            if (index as i32) >= e.mem.i32(a + 4) {
                break;
            }
        }
    }
    // Every occupied slot of the map becomes a 16-byte string map element: the key as a
    // sign-extended 64-bit number and the value as an `hkStringPtr`.
    let last = e.mem.i32(map + 8);
    let mut slot: i32 = 0;
    if last >= 0 {
        slot = next_occupied_slot(e, map, 0, last);
    }
    if slot <= last {
        loop {
            let allocator = router_allocator(e);
            let capacity = e.mem.u32(strings + 8) & 0x3fff_ffff;
            if e.mem.u32(strings + 4) == capacity {
                e.call(ARRAY_RESERVE_MORE, &args![allocator, strings, 0x10u32]);
            }
            let element = e
                .mem
                .u32(strings + 4)
                .wrapping_shl(4)
                .wrapping_add(e.mem.u32(strings));
            if element != 0 {
                e.call(STRING_MAP_ELEMENT_CONSTRUCTOR, &args![element]);
            }
            let count = e.mem.u32(strings + 4);
            e.mem.set_u32(strings + 4, count.wrapping_add(1));
            let key = e
                .mem
                .u32(e.mem.u32(map).wrapping_add((slot as u32).wrapping_mul(8)));
            let element = count.wrapping_shl(4).wrapping_add(e.mem.u32(strings));
            e.mem.set_u32(element, key);
            e.mem.set_u32(element + 4, ((key as i32) >> 31) as u32);
            let value = e.mem.u32(
                e.mem
                    .u32(map)
                    .wrapping_add((slot as u32).wrapping_mul(8))
                    .wrapping_add(4),
            );
            e.call(STRING_PTR_ASSIGN, &args![element + 8, value]);
            let last = e.mem.i32(map + 8);
            slot = next_occupied_slot(e, map, slot + 1, last);
            if slot > last {
                break;
            }
        }
    }
    // The packet's layout description goes through an array stream writer on `this + 0x30`.
    let size = e.mem.u32(out + 4);
    e.mem.set_u16(writer + 6, 1);
    e.mem.set_u32(writer, ARRAY_STREAM_WRITER_VTABLE);
    e.mem.set_u32(writer + 8, out);
    e.mem.set_u32(writer + 0xc, size);
    e.mem.set_u32(writer + 0x10, 1);
    let wanted = size.wrapping_add(1) as i32;
    let allocator = router_allocator(e);
    let capacity = (e.mem.u32(out + 8) & 0x3fff_ffff) as i32;
    if capacity < wanted {
        let doubled = capacity.wrapping_add(capacity);
        let grown = if wanted >= doubled { wanted } else { doubled };
        e.call(ARRAY_RESERVE, &args![allocator, out, grown, 1u32]);
    }
    let array_at = e.mem.u32(writer + 8);
    let end = e.mem.u32(array_at + 4);
    let data = e.mem.u32(array_at);
    e.mem.set_u8(end.wrapping_add(data), 0);
    e.call(LAYOUT_RULES_COPY, &args![layout, LAYOUT_RULES]);
    e.call(
        WRITE_OBJECT,
        &args![writer, layout, 1u32, 0u32, strings, STRING_MAP_CLASS, c],
    );
    e.mem.set_u32(writer, ARRAY_STREAM_WRITER_VTABLE);
    // The writer's destructor, inline: a writer that does not own its array (flag 0) and has one
    // empties and frees it. The flag is 1 here, so this never runs.
    if e.mem.u32(writer + 0x10) == 0 && e.mem.u32(writer + 8) != 0 {
        let owned = e.mem.u32(writer + 8);
        let allocator = router_allocator(e);
        let flags = e.mem.u32(owned + 8);
        e.mem.set_u32(owned + 4, 0);
        if (flags as i32) >= 0 {
            let data = e.mem.u32(owned);
            e.vcall(allocator, 0x10, &args![data, flags & 0x3fff_ffff]);
        }
        e.mem.set_u32(owned, 0);
        e.mem.set_u32(owned + 8, 0x8000_0000);
        let allocator = router_allocator(e);
        e.vcall(allocator, 8, &args![owned, 0xcu32]);
    }
    e.mem.set_u32(writer, BASE_OBJECT_VTABLE);
    e.call(STRING_MAP_DESTRUCTOR, &args![strings]);
    let allocator = router_allocator(e);
    e.call(MAP_CLEAR_AND_DEALLOCATE, &args![map, allocator]);
    e.call(MAP_CLEANUP, &args![map]);
    if total >= 1 {
        write_ranges(e, this, a, b, total);
    }
    let allocator = drop_ranges(e, a, b);
    let flags = e.mem.u32(c + 8);
    e.mem.set_u32(c + 4, 0);
    free_if_owned(
        e,
        allocator,
        c,
        flags,
        (flags & 0x3fff_ffff).wrapping_mul(0x18),
    );
}

/// Frees the two word arrays `a` and `b` as `step` does at its end and answers the allocator of
/// the router call made last (the one `step` frees its third array with).
fn drop_ranges(e: &mut Engine, a: u32, b: u32) -> u32 {
    for array in [b, a] {
        let allocator = router_allocator(e);
        let flags = e.mem.u32(array + 8);
        e.mem.set_u32(array + 4, 0);
        free_if_owned(
            e,
            allocator,
            array,
            flags,
            (flags & 0x3fff_ffff).wrapping_mul(4),
        );
        e.mem.set_u32(array, 0);
        e.mem.set_u32(array + 8, 0x8000_0000);
    }
    router_allocator(e)
}

/// The packet `step` writes when the ranges hold bytes: `write32u` of the total size, the byte
/// `0x0a`, the layout array (`this + 0x24` data, `this + 0x28` size) and the string array
/// (`this + 0x30` data, `this + 0x34` size), each as a 32-bit size and the bytes (when the size is
/// above 0), then the number of ranges and each range as its length and bytes.
fn write_ranges(e: &mut Engine, this: u32, a: u32, b: u32, total: i32) {
    let count = e.mem.i32(a + 4);
    let strings_size = e.mem.i32(this + 0x34);
    let layout_size = e.mem.i32(this + 0x28);
    let archive = e.mem.u32(this + 0xc);
    let packet = strings_size
        .wrapping_add(count.wrapping_mul(4))
        .wrapping_add(layout_size)
        .wrapping_add(total)
        .wrapping_add(0xd);
    e.call(OARCHIVE_WRITE_32U, &args![archive, packet]);
    let archive = e.mem.u32(this + 0xc);
    e.call(OARCHIVE_WRITE_BYTE, &args![archive, 0xau32]);
    let archive = e.mem.u32(this + 0xc);
    e.call(OARCHIVE_WRITE_32, &args![archive, layout_size]);
    if layout_size > 0 {
        let data = e.mem.u32(this + 0x24);
        let archive = e.mem.u32(this + 0xc);
        e.call(OARCHIVE_WRITE_RAW, &args![archive, data, layout_size]);
    }
    let archive = e.mem.u32(this + 0xc);
    e.call(OARCHIVE_WRITE_32, &args![archive, strings_size]);
    if strings_size > 0 {
        let data = e.mem.u32(this + 0x30);
        let archive = e.mem.u32(this + 0xc);
        e.call(OARCHIVE_WRITE_RAW, &args![archive, data, strings_size]);
    }
    let archive = e.mem.u32(this + 0xc);
    e.call(OARCHIVE_WRITE_32, &args![archive, count]);
    let mut index: u32 = 0;
    if count > 0 {
        loop {
            let end = e.mem.u32(e.mem.u32(b).wrapping_add(4 * index));
            let start = e.mem.u32(e.mem.u32(a).wrapping_add(4 * index));
            let length = end.wrapping_sub(start) as i32;
            let archive = e.mem.u32(this + 0xc);
            e.call(OARCHIVE_WRITE_32, &args![archive, length]);
            if length > 0 {
                let start = e.mem.u32(e.mem.u32(a).wrapping_add(4 * index));
                let archive = e.mem.u32(this + 0xc);
                e.call(OARCHIVE_WRITE_RAW, &args![archive, start, length]);
            }
            index += 1;
            if (index as i32) >= count {
                break;
            }
        }
    }
}

// Translated from 00ddc1c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStatisticsProcess::step` (Xbox PDB; slot 6 of the `hkProcess` vtable `010d7e28`),
/// `thiscall`, `RET 4` (the `float` step is not read: `_unused_1`). `this` is the `hkProcess` part
/// (`+8` of the whole object): `+0xc` the output archive, `+0x18`/`+0x1c` the array of items
/// (data, size), `+0x24`/`+0x28` the layout bytes and `+0x30`/`+0x34` the string bytes. Nothing
/// happens without an output archive. Otherwise it gathers two arrays of words, `a` and `b`, the
/// start and end addresses of monitor stream ranges: from the first item whose array at `+8` is
/// not empty, `a` is a copy of that array and `b` of the item's array at `+0x2c`; with no such
/// item, or when `a` stayed empty, `a` and `b` each get the two words of the thread-local block
/// (`TlsGetValue` of the slot at `0126835c`). Then the string bytes at `+0x30` are emptied, a
/// string map is built from the ranges (`00e0cd20` on every pair, the total of their lengths
/// kept), every entry of that map becomes a string map element, and the string map array is
/// written onto the bytes at `+0x30` through an `hkArrayStreamWriter`
/// (`hkObjectSerialize::writeObject`, class record `0126c830`). When the total is above 0 the
/// packet is written to the archive (`write32u` of its size, the byte `0x0a`, the layout bytes,
/// the string bytes and each range, each as its length and bytes); in every case the arrays are
/// freed.
pub fn hk_statistics_process_step(e: &mut Engine, this: Ptr, _unused_1: f32) {
    let base = this.addr();
    if e.mem.u32(base + 0xc) == 0 {
        return;
    }
    e.with_stack(STEP_FRAME_SIZE, |e, frame| {
        let frame = frame.addr();
        let (a, b, c) = (frame + 0x1c, frame + 0x28, frame + 0x4c);
        init_array(e, a);
        init_array(e, b);
        init_array(e, c);
        let count = e.mem.i32(base + 0x1c);
        let mut found = None;
        if count > 0 {
            let list = e.mem.u32(base + 0x18);
            let mut at = list;
            let mut index: u32 = 0;
            loop {
                let item = e.mem.u32(at);
                if e.mem.i32(item + 0xc) > 0 {
                    found = Some(index);
                    break;
                }
                index += 1;
                at = at.wrapping_add(4);
                if (index as i32) >= count {
                    break;
                }
            }
        }
        let mut skip_tls = false;
        if let Some(index) = found {
            let list = e.mem.u32(base + 0x18);
            let item = e.mem.u32(list.wrapping_add(4 * index));
            let allocator = router_allocator(e);
            copy_words(e, allocator, a, item + 8);
            let list = e.mem.u32(base + 0x18);
            let item = e.mem.u32(list.wrapping_add(4 * index));
            let allocator = router_allocator(e);
            copy_words(e, allocator, b, item + 0x2c);
            skip_tls = e.mem.u32(a + 4) != 0;
        }
        if !skip_tls {
            let slot = e.mem.u32(TLS_INDEX);
            let block = e.call(TLS_GET_VALUE, &args![slot]).u32();
            let first = e.mem.u32(block);
            let second = e.mem.u32(block + 4);
            let allocator = router_allocator(e);
            push_word(e, allocator, a, first);
            let allocator = router_allocator(e);
            push_word(e, allocator, b, second);
        }
        step_write(e, base, frame);
    });
}

// Translated from 00ddc820 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (slot 1 of the `hkProcess` vtable `010d7e28`). No arguments: the word at `01269cd8`
/// that `registerProcess` keeps (the process's tag).
pub fn fn_00ddc820(e: &mut Engine) -> u32 {
    e.mem.u32(REGISTERED_PROCESS_TAG)
}

// Translated from 00ddc870 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMonitorStreamStringMap::~hkMonitorStreamStringMap` (Xbox PDB), `ECX` = `this`, no stack
/// words, plain `RET`: `this` is an array of 16-byte elements (data, size at `+4`, capacity and
/// flags at `+8`) with an `hkStringPtr` at `+8` of each. Destroys every string pointer, sets the
/// size to 0, frees the storage (allocator slot `0x10`, capacity times 16 bytes) when the array
/// owns it, and resets the capacity word to `0x80000000` and the data to 0.
pub fn hk_monitor_stream_string_map_destructor(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let allocator = router_allocator(e);
    let size = e.mem.i32(base + 4);
    let data = e.mem.u32(base);
    if size > 0 {
        let mut at = data.wrapping_add(8);
        for _ in 0..size {
            e.call(STRING_PTR_DESTRUCTOR, &args![at]);
            at = at.wrapping_add(0x10);
        }
    }
    let flags = e.mem.u32(base + 8);
    e.mem.set_u32(base + 4, 0);
    free_if_owned(
        e,
        allocator,
        base,
        flags,
        (flags & 0x3fff_ffff).wrapping_shl(4),
    );
    e.mem.set_u32(base + 8, 0x8000_0000);
    e.mem.set_u32(base, 0);
}

// Translated from 00ddc160 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (the viewer or process class of this unit is unconfirmed): a
/// process creation function, the kind `hkProcessFactory::registerProcess` (`00dd9380`) is given.
/// It takes one stack word, the process contexts (cdecl, nothing popped). Allocates 0x44 bytes through the memory router's allocator (`00c85750`; the
/// virtual function at +4 of the object at router + 0x10), stores the size as a 16-bit word at +4,
/// runs the constructor `00ddbf00` on the block with the contexts, and answers that object + 8 when it is not null, else null.
/// Ghidra made no function here; the address is pushed at 00ddc1a6.
pub fn fn_00ddc160(e: &mut Engine, process_contexts: u32) -> u32 {
    let router = e.call(0x00c8_5750, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![0x44]).u32();
    e.mem.set_u16(block + 4, 0x44);
    let object = e.call(0x00dd_bf00, &args![block, process_contexts]).u32();
    if object != 0 {
        object.wrapping_add(8)
    } else {
        0
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ddc840, deleting_destructor_00ddc840(Ptr, u8) -> Ptr),
        entry!(0x00ddc830, fn_00ddc830(Ptr, u32) -> u32),
        entry!(0x00ddbf00, hk_statistics_process_hk_statistics_process(Ptr, u32) -> Ptr),
        entry!(0x00ddc0b0, fn_00ddc0b0(Ptr)),
        entry!(0x00ddc1a0, hk_statistics_process_register_process()),
        entry!(0x00ddc1c0, hk_statistics_process_step(Ptr, f32)),
        entry!(0x00ddc820, fn_00ddc820() -> u32),
        entry!(0x00ddc870, hk_monitor_stream_string_map_destructor(Ptr)),
        entry!(0x00dd_c160, fn_00ddc160(u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FREE_SLOT_DOUBLE: u32 = 0x0900_0011;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    /// Installs a base-destructor double at `base` and a memory router whose allocator
    /// frees through a double; both push their arguments (tagged 1 and 2) on the log.
    fn install(e: &mut Engine, base: u32) -> Rc<RefCell<Vec<u32>>> {
        let log = Rc::new(RefCell::new(Vec::new()));
        let base_log = Rc::clone(&log);
        e.register_double(base, move |_, a| {
            base_log.borrow_mut().extend([1, a[0]]);
            Ret::default()
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 8, FREE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let free_log = Rc::clone(&log);
        e.register_double(FREE_SLOT_DOUBLE, move |_, a| {
            free_log.borrow_mut().extend([2, a[1], a[2]]);
            Ret::default()
        });
        log
    }

    /// Without bit 0 only the base destructor runs; with it the block is also freed with
    /// its stored size. Either way `this` is returned.
    fn check(address: u32, base: u32) {
        let mut e = engine();
        let log = install(&mut e, base);
        let this = e.mem.alloc(0x40);
        e.mem.set_u16(this + 4, 0x1234);
        let got = e.call(address, &args![this, 0xffff_fffeu32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this]);
        log.borrow_mut().clear();
        let got = e.call(address, &args![this, 1u32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this, 2, this, 0x1234]);
    }

    #[test]
    fn deleting_destructor_00ddc840_frees_only_when_bit_zero_is_set() {
        check(0x00dd_c840, 0x00dd_c0b0);
    }
}

#[cfg(test)]
mod a07_thunk_tests {
    use crate::prelude::*;

    /// Calls `thunk` with `this = 0x2000` and one flags word, with a double
    /// over `target` that returns `0xdeadbeef`; checks that the double ran
    /// once, with `this` moved back by `back` and the flags passed on, and
    /// returns the thunk's `EAX`.
    fn check(thunk: u32, target: u32, back: u32) -> u32 {
        let mut e = Engine::new();
        e.register(target, |_, _| Ret {
            eax: 0xdead_beef,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let result = e.call(thunk, &[0x2000u32, 0x1]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == target).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, vec![0x2000 - back, 0x1]);
        result.u32()
    }

    #[test]
    fn thunk_00ddc830() {
        assert_eq!(check(0x00dd_c830, 0x00dd_c840, 0x8), 0xdead_beef);
    }
}

#[cfg(test)]
mod statistics_process_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    fn answer(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    /// Registers a double at `address` that logs `(address, arguments)` and answers `result`.
    fn logging(e: &mut Engine, log: &Log, address: u32, result: u32) {
        let l = Rc::clone(log);
        e.register_double(address, move |_, a| {
            l.borrow_mut().push((address, a.to_vec()));
            answer(result)
        });
    }

    /// Fake code addresses of the allocator's slots.
    const ALLOCATE_SLOT: u32 = 0x0f00_000c;
    const FREE_SLOT: u32 = 0x0f00_0010;

    /// An engine with the router, whose allocator's slot `0xc` hands out a fresh block granting 4
    /// bytes more than asked (logging the request) and slot `0x10` logs `(data, bytes)`; every
    /// other function the unit calls is a logging double answering 0.
    fn world() -> (Engine, Log) {
        let mut e = Engine::new();
        e.map(0x0101_7000, 0x1000);
        e.map(0x0126_8000, 0x2000);
        e.mem.set_f64(THOUSAND, 1000.0);
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let l = Rc::clone(&log);
        e.register_double(ALLOCATE_SLOT, move |e, a| {
            let requested = e.mem.u32(a[1]);
            l.borrow_mut().push((ALLOCATE_SLOT, vec![requested]));
            e.mem.set_u32(a[1], requested + 4);
            answer(e.mem.alloc(requested + 4))
        });
        let l = Rc::clone(&log);
        e.register_double(FREE_SLOT, move |_, a| {
            l.borrow_mut().push((FREE_SLOT, a[1..].to_vec()));
            Ret::default()
        });
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0xc, ALLOCATE_SLOT);
        e.mem.set_u32(vtable + 0x10, FREE_SLOT);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| answer(router));
        // `00c90940`: a new block of twice the elements (at least 1), the old ones copied.
        let l = Rc::clone(&log);
        e.register_double(ARRAY_RESERVE_MORE, move |e, a| {
            l.borrow_mut().push((ARRAY_RESERVE_MORE, a.to_vec()));
            let (array, element_size) = (a[1], a[2]);
            let size = e.mem.u32(array + 4);
            let capacity = (size * 2).max(1);
            let data = e.mem.alloc(capacity * element_size);
            if size > 0 {
                let old = e.mem.u32(array);
                let kept = e.mem.bytes(old, size * element_size);
                e.mem.write(data, &kept);
            }
            e.mem.set_u32(array, data);
            e.mem.set_u32(array + 8, capacity);
            Ret::default()
        });
        let l = Rc::clone(&log);
        e.register_double(STRING_MAP_ARRAY_CONSTRUCTOR, move |e, a| {
            l.borrow_mut()
                .push((STRING_MAP_ARRAY_CONSTRUCTOR, a.to_vec()));
            e.mem.set_u32(a[0], 0);
            e.mem.set_u32(a[0] + 4, 0);
            e.mem.set_u32(a[0] + 8, 0x8000_0000);
            answer(a[0])
        });
        for address in [
            FRAME_INFO_CONSTRUCTOR,
            GET_TICKS_PER_SECOND,
            ARRAY_STREAM_WRITER_CONSTRUCTOR,
            ARRAY_STREAM_WRITER_DESTRUCTOR,
            LAYOUT_RULES_COPY,
            WRITE_OBJECT,
            ARRAY_RESERVE,
            MAP_RESERVE,
            MAP_CLEAR_AND_DEALLOCATE,
            MAP_CLEANUP,
            STRING_MAP_ELEMENT_CONSTRUCTOR,
            READ_MONITOR_RANGE,
            STRING_PTR_ASSIGN,
            STRING_PTR_DESTRUCTOR,
            STRING_MAP_DESTRUCTOR,
            OARCHIVE_WRITE_32U,
            OARCHIVE_WRITE_32,
            OARCHIVE_WRITE_RAW,
            OARCHIVE_WRITE_BYTE,
            PROCESS_FACTORY_REGISTER,
            TLS_GET_VALUE,
        ] {
            logging(&mut e, &log, address, 0);
        }
        (e, log)
    }

    fn calls(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    fn array_of_words(e: &mut Engine, words: &[u32]) -> u32 {
        let array = e.mem.alloc(12);
        let data = e.mem.alloc(4 * words.len().max(1) as u32);
        for (i, word) in words.iter().enumerate() {
            e.mem.set_u32(data + 4 * i as u32, *word);
        }
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 4, words.len() as u32);
        e.mem.set_u32(array + 8, 0x8000_0000 | words.len() as u32);
        array
    }

    fn words_of(e: &Engine, data: u32, count: u32) -> Vec<u32> {
        (0..count).map(|i| e.mem.u32(data + 4 * i)).collect()
    }

    /// Runs the constructor with `ticks` as the tick rate; returns the engine, the object, the
    /// words of the frame info the written object had, and the log.
    fn construct(ticks: u64) -> (Engine, u32, Vec<u32>, Log) {
        let (mut e, log) = world();
        let seen: Rc<RefCell<Vec<u32>>> = Rc::new(RefCell::new(Vec::new()));
        let info = Rc::clone(&seen);
        let l = Rc::clone(&log);
        e.register_double(FRAME_INFO_CONSTRUCTOR, move |e, a| {
            l.borrow_mut().push((FRAME_INFO_CONSTRUCTOR, a.to_vec()));
            for offset in [0u32, 4, 8, 0xc, 0x10, 0x14] {
                e.mem.set_u32(a[0] + offset, 0x9999_0000 + offset);
            }
            answer(a[0])
        });
        let l = Rc::clone(&log);
        e.register_double(GET_TICKS_PER_SECOND, move |_, a| {
            l.borrow_mut().push((GET_TICKS_PER_SECOND, a.to_vec()));
            Ret {
                eax: ticks as u32,
                edx: (ticks >> 32) as u32,
                ..Ret::default()
            }
        });
        let l = Rc::clone(&log);
        e.register_double(WRITE_OBJECT, move |e, a| {
            l.borrow_mut().push((WRITE_OBJECT, a.to_vec()));
            for offset in [0u32, 4, 8, 0xc, 0x10, 0x14] {
                info.borrow_mut().push(e.mem.u32(a[4] + offset));
            }
            // The throw-away array: an owned block of capacity 3 holding 2 elements.
            e.mem.set_u32(a[6], 0x7777);
            e.mem.set_u32(a[6] + 4, 2);
            e.mem.set_u32(a[6] + 8, 3);
            Ret::default()
        });
        let this = e.mem.alloc(0x50);
        let source = array_of_words(&mut e, &[5, 6, 7]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00dd_bf00, &args![this, source]).u32(), this);
        e.call_log = None;
        let frame_info = seen.borrow().clone();
        (e, this, frame_info, log)
    }

    #[test]
    fn constructor_stores_the_vtables_and_empty_arrays_and_copies_the_source() {
        let (e, this, _, _) = construct(3_000_000_000);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this), STATISTICS_PROCESS_VTABLE);
        assert_eq!(e.mem.u32(this + 8), STATISTICS_PROCESS_PROCESS_VTABLE);
        assert_eq!(e.mem.u8(this + 0xc), 1);
        // The byte arrays stay empty and not owned; the word array is a copy of the source in
        // storage the allocator granted 4 bytes more of: capacity (12 + 4) / 4.
        for array in [this + 0x2c, this + 0x38] {
            assert_eq!(e.mem.u32(array), 0);
            assert_eq!(e.mem.u32(array + 4), 0);
            assert_eq!(e.mem.u32(array + 8), 0x8000_0000);
        }
        assert_eq!(e.mem.u32(this + 0x24), 3);
        assert_eq!(e.mem.u32(this + 0x28), 4);
        assert_eq!(words_of(&e, e.mem.u32(this + 0x20), 3), vec![5, 6, 7]);
    }

    #[test]
    fn constructor_builds_and_writes_the_frame_info_with_the_milliseconds_per_tick() {
        let (_, this, frame_info, log) = construct(3_000_000_000);
        // The first word (vtable), +4 and +0xc are cleared, +8 is 1, +0x10 is 1000 / ticks as a
        // float and +0x14 is 1.0.
        let per_tick = (1000.0f64 / 3.0e9f64) as f32;
        assert_eq!(
            frame_info,
            vec![0, 0, 1, 0, per_tick.to_bits(), 0x3f80_0000]
        );
        // The order of the calls and the arguments of the serialization.
        let order: Vec<u32> = log.borrow().iter().map(|c| c.0).collect();
        let position = |address: u32| order.iter().position(|a| *a == address).unwrap();
        assert!(position(FRAME_INFO_CONSTRUCTOR) < position(GET_TICKS_PER_SECOND));
        assert!(position(GET_TICKS_PER_SECOND) < position(ARRAY_STREAM_WRITER_CONSTRUCTOR));
        assert!(position(ARRAY_STREAM_WRITER_CONSTRUCTOR) < position(LAYOUT_RULES_COPY));
        assert!(position(LAYOUT_RULES_COPY) < position(WRITE_OBJECT));
        assert!(position(WRITE_OBJECT) < position(ARRAY_STREAM_WRITER_DESTRUCTOR));
        let writer = calls(&log, ARRAY_STREAM_WRITER_CONSTRUCTOR);
        assert_eq!(writer[0][1..], [this + 0x2c, 1]);
        let layout = calls(&log, LAYOUT_RULES_COPY);
        assert_eq!(layout[0][1], LAYOUT_RULES);
        let write = calls(&log, WRITE_OBJECT);
        assert_eq!(write[0][0], writer[0][0]);
        assert_eq!(write[0][1], layout[0][0]);
        assert_eq!(write[0][2..4], [1, 0]);
        assert_eq!(write[0][5], FRAME_INFO_CLASS);
        // The throw-away array (owned, capacity 3, 24-byte elements) is freed.
        assert_eq!(calls(&log, FREE_SLOT), vec![vec![0x7777, 0x48]]);
    }

    #[test]
    fn constructor_converts_the_tick_rate_as_an_unsigned_64_bit_number() {
        // The top bit set: the sum of the low part and 2^63, as the code's two `FILD`s make it.
        let (_, _, frame_info, _) = construct(0x8000_0000_0000_0001);
        let expected = (1000.0f64 / 9.223_372_036_854_776e18_f64) as f32;
        assert_eq!(frame_info[4], expected.to_bits());
        // A zero rate divides by zero: infinity.
        let (_, _, frame_info, _) = construct(0);
        assert_eq!(frame_info[4], f32::INFINITY.to_bits());
    }

    #[test]
    fn destructor_body_frees_owned_arrays_and_restores_the_base_vtables() {
        let (mut e, log) = world();
        let this = e.mem.alloc(0x50);
        // +0x38 (bytes) owned with capacity 6, +0x2c (bytes) not owned, +0x20 (words) capacity 4.
        e.mem.set_u32(this + 0x38, 0x1111);
        e.mem.set_u32(this + 0x3c, 2);
        e.mem.set_u32(this + 0x40, 6);
        e.mem.set_u32(this + 0x2c, 0x2222);
        e.mem.set_u32(this + 0x30, 5);
        e.mem.set_u32(this + 0x34, 0x8000_0009);
        e.mem.set_u32(this + 0x20, 0x3333);
        e.mem.set_u32(this + 0x24, 1);
        e.mem.set_u32(this + 0x28, 4);
        e.call(0x00dd_c0b0, &args![this]);
        assert_eq!(
            calls(&log, FREE_SLOT),
            vec![vec![0x1111, 6], vec![0x3333, 16]]
        );
        for array in [this + 0x38, this + 0x2c, this + 0x20] {
            assert_eq!(e.mem.u32(array), 0);
            assert_eq!(e.mem.u32(array + 4), 0);
            assert_eq!(e.mem.u32(array + 8), 0x8000_0000);
        }
        assert_eq!(e.mem.u32(this), BASE_OBJECT_VTABLE);
        assert_eq!(e.mem.u32(this + 8), PROCESS_VTABLE);
    }

    #[test]
    fn register_process_keeps_the_factory_answer_and_the_tag_getter_returns_it() {
        let (mut e, log) = world();
        e.mem.set_u32(PROCESS_FACTORY_REGISTRY, 0x4a4a);
        logging(&mut e, &log, PROCESS_FACTORY_REGISTER, 0x77);
        e.call(0x00dd_c1a0, &args![]);
        assert_eq!(
            calls(&log, PROCESS_FACTORY_REGISTER),
            vec![vec![0x4a4a, STATISTICS_NAME, STATISTICS_PROCESS_FACTORY]]
        );
        assert_eq!(e.mem.u32(REGISTERED_PROCESS_TAG), 0x77);
        assert_eq!(e.call(0x00dd_c820, &args![]).u32(), 0x77);
    }

    #[test]
    fn the_string_map_destructor_destroys_the_strings_and_frees_the_elements() {
        let (mut e, log) = world();
        let map = e.mem.alloc(12);
        let data = e.mem.alloc(0x40);
        e.mem.set_u32(map, data);
        e.mem.set_u32(map + 4, 3);
        e.mem.set_u32(map + 8, 4);
        e.call(0x00dd_c870, &args![map]);
        // Each element's string pointer (+8), 16 bytes apart, then the storage (capacity 4 times 16).
        assert_eq!(
            calls(&log, STRING_PTR_DESTRUCTOR),
            vec![vec![data + 8], vec![data + 0x18], vec![data + 0x28]]
        );
        assert_eq!(calls(&log, FREE_SLOT), vec![vec![data, 0x40]]);
        assert_eq!(e.mem.u32(map), 0);
        assert_eq!(e.mem.u32(map + 4), 0);
        assert_eq!(e.mem.u32(map + 8), 0x8000_0000);
        // An array that does not own its storage and has no elements frees nothing.
        let empty = e.mem.alloc(12);
        e.mem.set_u32(empty + 8, 0x8000_0004);
        let before = log.borrow().len();
        e.call(0x00dd_c870, &args![empty]);
        assert_eq!(log.borrow().len(), before);
    }

    /// A process `this` (the `hkProcess` part) with an output archive `0xa4c1`, layout bytes (3 at
    /// `0x4000`) and an empty string array; `items` are word-array pairs for the item list.
    fn process(e: &mut Engine, items: &[(&[u32], &[u32])]) -> u32 {
        let this = e.mem.alloc(0x60);
        e.mem.set_u32(this + 0xc, 0xa4c1);
        e.mem.set_u32(this + 0x24, 0x4000);
        e.mem.set_u32(this + 0x28, 3);
        let out = e.mem.alloc(16);
        e.mem.set_u32(this + 0x30, out);
        e.mem.set_u32(this + 0x38, 0x8000_0000);
        let list = e.mem.alloc(4 * items.len().max(1) as u32);
        for (i, (starts, ends)) in items.iter().enumerate() {
            let item = e.mem.alloc(0x40);
            let first = array_of_words(e, starts);
            let second = array_of_words(e, ends);
            // The item's arrays sit inline at +8 and +0x2c.
            for (at, source) in [(item + 8, first), (item + 0x2c, second)] {
                for offset in [0, 4, 8] {
                    let word = e.mem.u32(source + offset);
                    e.mem.set_u32(at + offset, word);
                }
            }
            e.mem.set_u32(list + 4 * i as u32, item);
        }
        e.mem.set_u32(this + 0x18, list);
        e.mem.set_u32(this + 0x1c, items.len() as u32);
        this
    }

    /// Doubles for a step that serializes: the monitor range reader fills the map with two
    /// entries on its first call, the serialization leaves 6 bytes in the string array at
    /// `this + 0x30` and a 2-element owned array `c` (capacity 2).
    fn serialize_doubles(e: &mut Engine, log: &Log, this: u32) {
        let l = Rc::clone(log);
        let table = e.mem.alloc(32);
        e.mem.set_u32(table, 0xffff_ffff);
        e.mem.set_u32(table + 8, 0x11);
        e.mem.set_u32(table + 12, 0x5001);
        e.mem.set_u32(table + 16, 0xffff_ffff);
        e.mem.set_u32(table + 24, 0xffff_fff0);
        e.mem.set_u32(table + 28, 0x5002);
        let filled = Rc::new(RefCell::new(false));
        e.register_double(READ_MONITOR_RANGE, move |e, a| {
            l.borrow_mut().push((READ_MONITOR_RANGE, a.to_vec()));
            if !*filled.borrow() {
                *filled.borrow_mut() = true;
                e.mem.set_u32(a[2], table);
                e.mem.set_u32(a[2] + 8, 3);
            }
            Ret::default()
        });
        let l = Rc::clone(log);
        e.register_double(WRITE_OBJECT, move |e, a| {
            // The string map array holds its elements at this point: log them as
            // (size, key low, key high, ...).
            let (strings, c) = (a[5 - 1], a[7 - 1]);
            let mut seen = vec![e.mem.u32(strings + 4)];
            for i in 0..e.mem.u32(strings + 4) {
                let element = e.mem.u32(strings) + 0x10 * i;
                seen.push(e.mem.u32(element));
                seen.push(e.mem.u32(element + 4));
            }
            l.borrow_mut().push((WRITE_OBJECT, seen));
            e.mem.set_u32(this + 0x34, 6);
            e.mem.set_u32(c, 0x8888);
            e.mem.set_u32(c + 4, 2);
            e.mem.set_u32(c + 8, 2);
            Ret::default()
        });
    }

    #[test]
    fn step_does_nothing_without_an_output_archive() {
        let (mut e, log) = world();
        let this = process(&mut e, &[]);
        e.mem.set_u32(this + 0xc, 0);
        e.call(0x00dd_c1c0, &args![this, 0.0f32]);
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn step_gathers_the_ranges_of_the_first_item_with_bytes_and_writes_the_packet() {
        let (mut e, log) = world();
        // The first item has no ranges; the second has one (0x1000 to 0x1010).
        let this = process(&mut e, &[(&[], &[]), (&[0x1000], &[0x1010])]);
        serialize_doubles(&mut e, &log, this);
        e.call(0x00dd_c1c0, &args![this, 0.0f32]);
        // The thread-local block was not needed.
        assert!(calls(&log, TLS_GET_VALUE).is_empty());
        // The range reader was given the pair and the map; the total length is 0x10.
        let reads = calls(&log, READ_MONITOR_RANGE);
        assert_eq!(reads.len(), 1);
        assert_eq!(reads[0][..2], [0x1000, 0x1010]);
        // Both map entries became string map elements (the key sign-extended), each value
        // assigned to its string pointer.
        let seen = calls(&log, WRITE_OBJECT);
        assert_eq!(seen[0], vec![2, 0x11, 0, 0xffff_fff0, 0xffff_ffff]);
        let assigned = calls(&log, STRING_PTR_ASSIGN);
        assert_eq!(assigned.len(), 2);
        assert_eq!(assigned[0][1], 0x5001);
        assert_eq!(assigned[1][1], 0x5002);
        assert_eq!(calls(&log, STRING_MAP_ELEMENT_CONSTRUCTOR).len(), 2);
        // packet size: 6 string bytes + 1 range * 4 + 3 layout bytes + 0x10 total + 0xd.
        assert_eq!(calls(&log, OARCHIVE_WRITE_32U), vec![vec![0xa4c1, 0x2a]]);
        assert_eq!(calls(&log, OARCHIVE_WRITE_BYTE), vec![vec![0xa4c1, 0xa]]);
        // The 32-bit writes: layout size, string size, range count, range length.
        assert_eq!(
            calls(&log, OARCHIVE_WRITE_32),
            vec![
                vec![0xa4c1, 3],
                vec![0xa4c1, 6],
                vec![0xa4c1, 1],
                vec![0xa4c1, 0x10]
            ]
        );
        // The raw writes: layout bytes, string bytes, the range.
        let raw = calls(&log, OARCHIVE_WRITE_RAW);
        assert_eq!(raw.len(), 3);
        assert_eq!(raw[0], vec![0xa4c1, 0x4000, 3]);
        assert_eq!(raw[1][2], 6);
        assert_eq!(raw[2], vec![0xa4c1, 0x1000, 0x10]);
        // The string map is destroyed by its destructor (a double here); then b and a (4 bytes
        // per word) and the throw-away c (24 bytes each) are freed, in that order.
        assert_eq!(calls(&log, STRING_MAP_DESTRUCTOR).len(), 1);
        let frees = calls(&log, FREE_SLOT);
        assert_eq!(frees.len(), 3);
        assert_eq!(frees[2], vec![0x8888, 0x30]);
    }

    #[test]
    fn step_uses_the_thread_block_when_no_item_has_bytes() {
        let (mut e, log) = world();
        let this = process(&mut e, &[(&[], &[])]);
        let block = e.mem.alloc(8);
        e.mem.set_u32(TLS_INDEX, 7);
        e.mem.set_u32(block, 0x2000);
        e.mem.set_u32(block + 4, 0x2040);
        logging(&mut e, &log, TLS_GET_VALUE, block);
        serialize_doubles(&mut e, &log, this);
        e.call(0x00dd_c1c0, &args![this, 0.0f32]);
        assert_eq!(calls(&log, TLS_GET_VALUE), vec![vec![7]]);
        // Growing the empty word arrays by one step each before the words are appended.
        let word_grows = calls(&log, ARRAY_RESERVE_MORE)
            .into_iter()
            .filter(|c| c[2] == 4)
            .count();
        assert_eq!(word_grows, 2);
        let reads = calls(&log, READ_MONITOR_RANGE);
        assert_eq!(reads[0][..2], [0x2000, 0x2040]);
        // The packet size: 6 + 4 + 3 + 0x40 + 0xd.
        assert_eq!(calls(&log, OARCHIVE_WRITE_32U), vec![vec![0xa4c1, 0x5a]]);
        assert_eq!(
            calls(&log, OARCHIVE_WRITE_RAW).last().unwrap()[1..],
            [0x2000, 0x40]
        );
    }

    #[test]
    fn step_writes_no_packet_when_the_ranges_total_nothing() {
        let (mut e, log) = world();
        let this = process(&mut e, &[]);
        let block = e.mem.alloc(8);
        e.mem.set_u32(block, 0x2000);
        e.mem.set_u32(block + 4, 0x2000);
        logging(&mut e, &log, TLS_GET_VALUE, block);
        serialize_doubles(&mut e, &log, this);
        e.call(0x00dd_c1c0, &args![this, 0.0f32]);
        assert!(calls(&log, OARCHIVE_WRITE_32U).is_empty());
        assert!(calls(&log, OARCHIVE_WRITE_32).is_empty());
        // The serialization still ran and the arrays were freed.
        assert_eq!(calls(&log, WRITE_OBJECT).len(), 1);
        assert_eq!(calls(&log, FREE_SLOT).len(), 3);
    }

    #[test]
    fn the_constructor_keeps_its_locals_at_the_exe_frame_offsets() {
        let (mut e, log) = world();
        let this = e.mem.alloc(0x50);
        let source = array_of_words(&mut e, &[5]);
        // The frame is the next stack local: the layout word at +0x10, the throw-away array at
        // +0x18, the writer at +0x24 and the frame info at +0x38.
        let frame = e.next_stack_local(CONSTRUCTOR_FRAME_SIZE);
        e.call(0x00dd_bf00, &args![this, source]);
        assert_eq!(
            calls(&log, FRAME_INFO_CONSTRUCTOR),
            vec![vec![frame + 0x38]]
        );
        assert_eq!(
            calls(&log, ARRAY_STREAM_WRITER_CONSTRUCTOR),
            vec![vec![frame + 0x24, this + 0x2c, 1]]
        );
        assert_eq!(
            calls(&log, WRITE_OBJECT),
            vec![vec![
                frame + 0x24,
                frame + 0x10,
                1,
                0,
                frame + 0x38,
                FRAME_INFO_CLASS,
                frame + 0x18
            ]]
        );
    }
    #[test]
    fn step_keeps_its_locals_at_the_exe_frame_offsets() {
        let (mut e, log) = world();
        let this = process(&mut e, &[(&[0x1000], &[0x1010])]);
        // The frame is the next stack local: the layout word sits at +0x18, the map at +0x34,
        // the string map array at +0x40, the array `c` at +0x4c and the array stream writer at
        // +0x58 of the exe's frame.
        let frame = e.next_stack_local(STEP_FRAME_SIZE);
        e.call(0x00dd_c1c0, &args![this, 0.0f32]);
        assert_eq!(calls(&log, READ_MONITOR_RANGE)[0][2], frame + 0x34);
        assert_eq!(
            calls(&log, WRITE_OBJECT),
            vec![vec![
                frame + 0x58,
                frame + 0x18,
                1,
                0,
                frame + 0x40,
                STRING_MAP_CLASS,
                frame + 0x4c
            ]]
        );
        assert_eq!(calls(&log, STRING_MAP_DESTRUCTOR), vec![vec![frame + 0x40]]);
    }
}

#[cfg(test)]
mod tests_a01 {
    use crate::prelude::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// What the constructor double saw: its argument words and the size word at +4 of the block.
    type ConstructorCall = (Vec<u32>, u16);

    /// The slot of the allocator's vtable (offset 4) that allocates, as a test double.
    const ALLOCATE_SLOT_DOUBLE: u32 = 0x0900_0001;

    /// A memory router (`00c85750`) whose allocator allocates through a double that logs the
    /// size it was asked for and answers a zeroed block.
    fn allocator_engine() -> (Engine, Rc<RefCell<Vec<u32>>>) {
        let mut e = Engine::new();
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 4, ALLOCATE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(0x00c8_5750, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let sizes = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&sizes);
        e.register_double(ALLOCATE_SLOT_DOUBLE, move |e, a| {
            log.borrow_mut().push(a[1]);
            Ret {
                eax: e.mem.alloc(a[1]),
                ..Ret::default()
            }
        });
        (e, sizes)
    }

    /// The constructor sees the block with its size already stored; the answer is the object + 8.
    #[test]
    fn fn_00ddc160_builds_the_object_and_answers_its_second_base() {
        let (mut e, sizes) = allocator_engine();
        let seen: Rc<RefCell<Vec<ConstructorCall>>> = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(0x00dd_bf00, move |e, a| {
            log.borrow_mut().push((a.to_vec(), e.mem.u16(a[0] + 4)));
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        let object = e.call(0x00dd_c160, &args![0x1234u32]).u32();
        assert_eq!(*sizes.borrow(), vec![0x44]);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        let block = seen[0].0[0];
        assert_eq!(seen[0].0, vec![block, 0x1234]);
        assert_eq!(seen[0].1, 0x44);
        assert_eq!(object, block + 8);
    }

    /// A constructor that answers null gives a null answer.
    #[test]
    fn fn_00ddc160_answers_null_when_the_constructor_does() {
        let (mut e, _) = allocator_engine();
        e.register_double(0x00dd_bf00, |_, _| Ret::default());
        assert_eq!(e.call(0x00dd_c160, &args![0x1234u32]).u32(), 0);
    }
}

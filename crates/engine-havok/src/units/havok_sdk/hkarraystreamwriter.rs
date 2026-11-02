//! `Havok SDK/hkarraystreamwriter.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00d46cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00d4_6c30`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00d46cc0(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00d4_6c30, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// `hkArrayUtil::_reserve` (`00c908c0`, cdecl `(allocator, array, capacity, element size)`) and
/// `_reserveMore` (`00c90940`, cdecl `(allocator, array, element size)`).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
const ARRAY_RESERVE_MORE: u32 = 0x00c9_0940;
/// `hkStatisticsCollector::getAllocatedSize` (`00d06f80`, cdecl `(pointer, bytes)`).
const GET_ALLOCATED_SIZE: u32 = 0x00d0_6f80;
/// `hkString::memCpy` (`00cb1260`, cdecl `(destination, source, size)`) and the C runtime's
/// `memset` (`00ec61c0`, cdecl `(destination, value, size)`).
const MEM_COPY: u32 = 0x00cb_1260;
const MEM_SET: u32 = 0x00ec_61c0;
/// The low 30 bits of an `hkArray` capacity word are the capacity, the top bit marks storage that
/// is not owned.
const CAPACITY_MASK: u32 = 0x3fff_ffff;
const NOT_OWNED: u32 = 0x8000_0000;
/// The vtable the stream writers share while they run (`010cf734`) and the one they leave behind
/// (`0102e388`).
const WRITER_VTABLE: u32 = 0x010c_f734;
const BASE_VTABLE: u32 = 0x0102_e388;
/// The class label `calcContentStatistics` reports its array under (`010cc460`).
const ARRAY_LABEL: u32 = 0x010c_c460;

/// The allocator of the thread's memory router (`router + 0x10`).
fn router_allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

/// Makes room for `needed` entries of one byte in the `hkArray` at `array` (data, size, capacity
/// word): when the capacity is smaller, reserves the larger of `needed` and twice the capacity
/// (signed compares) through `_reserve`. The router is asked for its allocator first either way.
fn reserve_at_least(e: &mut Engine, array: u32, needed: i32) {
    let allocator = router_allocator(e);
    let capacity = (e.mem.u32(array + 8) & CAPACITY_MASK) as i32;
    if capacity < needed {
        let doubled = capacity.wrapping_add(capacity);
        let wanted = if needed >= doubled { needed } else { doubled };
        e.call(ARRAY_RESERVE, &args![allocator, array, wanted as u32, 1u32]);
    }
}

/// Gives the block-pointer array of the `hkMemoryTrack` at `track` (data `+8`, size `+0xc`,
/// capacity word `+0x10`) back to the allocator (slot `0x10`, `(data, capacity * 4)`) when it owns
/// its storage, and zeroes the size.
fn release_block_array(e: &mut Engine, track: u32) {
    let allocator = router_allocator(e);
    let flags = e.mem.u32(track + 0x10);
    e.mem.set_u32(track + 0xc, 0);
    if (flags as i32) >= 0 {
        let data = e.mem.u32(track + 8);
        e.vcall(
            allocator,
            0x10,
            &args![data, (flags & CAPACITY_MASK).wrapping_mul(4)],
        );
    }
}

// Translated from 00d46660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMemoryTrack::read` (Xbox PDB), `this` = the track (block size `+0`, bytes in the last block
/// `+4`, block pointers `+8`, block count `+0xc`): copies `size` bytes starting at byte `offset`
/// of the track to `dest`, block by block (a block holds the block size, the last one only its
/// fill). The offset is divided by the block size with a signed `IDIV`.
pub fn memory_track_read(e: &mut Engine, this: Ptr, dest: u32, size: i32, offset: i32) {
    let t = this.addr();
    let block_size = e.mem.u32(t) as i32;
    let mut block = offset / block_size;
    let mut offset = offset.wrapping_sub(block_size.wrapping_mul(block));
    let mut dest = dest;
    let mut size = size;
    if size > 0 {
        loop {
            let held = if block < (e.mem.u32(t + 0xc) as i32).wrapping_sub(1) {
                e.mem.u32(t)
            } else {
                e.mem.u32(t + 4)
            };
            let available = held.wrapping_sub(offset as u32) as i32;
            let source = e
                .mem
                .u32(
                    e.mem
                        .u32(t + 8)
                        .wrapping_add((block as u32).wrapping_mul(4)),
                )
                .wrapping_add(offset as u32);
            if size <= available {
                e.call(MEM_COPY, &args![dest, source, size as u32]);
                return;
            }
            e.call(MEM_COPY, &args![dest, source, available as u32]);
            dest = dest.wrapping_add(available as u32);
            size = size.wrapping_sub(available);
            block = block.wrapping_add(1);
            offset = 0;
            if size <= 0 {
                return;
            }
        }
    }
}

// Translated from 00d466e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the memory-track stream writer (vtable `010cf734`; the class is unconfirmed):
/// `this` gets the vtable, the track pointer `track` at `+8`, the ownership word `owns` at
/// `+0x10`, position `0` at `+0xc` and the 16-bit value `1` at `+6`. Returns `this`.
pub fn constructor_00d466e0(e: &mut Engine, this: Ptr, track: u32, owns: u32) -> Ptr {
    let t = this.addr();
    e.mem.set_u16(t + 6, 1);
    e.mem.set_u32(t, WRITER_VTABLE);
    e.mem.set_u32(t + 0x10, owns);
    e.mem.set_u32(t + 8, track);
    e.mem.set_u32(t + 0xc, 0);
    this
}

// Translated from 00d46710 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMemoryTrackStreamReader::read` (Xbox PDB), `this` = the reader (track `+8`, position
/// `+0xc`): asks the stream (slot `0xc` with a pointer to a byte on the stack, the answer being
/// read through the pointer it returns) whether it is ok and, if so, reads
/// `min(size, bytes after the position)` bytes into `dest` with `hkMemoryTrack::read`. The
/// position advances by the requested `size`. Returns the bytes read, or `0` when not ok.
pub fn memory_track_stream_reader_read(e: &mut Engine, this: Ptr, dest: u32, size: i32) -> i32 {
    let r = this.addr();
    let answer = e.with_stack(4, |e, slot| {
        let result = e.vcall(r, 0xc, &args![slot]).u32();
        e.mem.u8(result)
    });
    if answer == 0 {
        return 0;
    }
    let track = e.mem.u32(r + 8);
    let position = e.mem.u32(r + 0xc);
    let end = e
        .mem
        .u32(track + 0xc)
        .wrapping_sub(1)
        .wrapping_mul(e.mem.u32(track))
        .wrapping_add(e.mem.u32(track + 4));
    let available = end.wrapping_sub(position) as i32;
    let count = if size < available { size } else { available };
    memory_track_read(e, Ptr::new(track), dest, count, position as i32);
    e.mem.set_u32(r + 0xc, position.wrapping_add(size as u32));
    count
}

// Translated from 00d46770 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkArrayStreamWriter::calcContentStatistics` (Xbox PDB), `this` = the writer (its `hkArray` of
/// bytes at `+8`), `RET 8`: when the array exists and either its capacity word is not negative or
/// the collector's byte at `+0x10` is set, reports it to the collector (slot `0x14`:
/// `(3, "array", data, size, allocated size)`, the allocated size from
/// `hkStatisticsCollector::getAllocatedSize` of `(data, capacity)`).
pub fn array_stream_writer_calc_content_statistics(
    e: &mut Engine,
    this: Ptr,
    collector: Ptr,
    _unused_2: u32,
) {
    let array = e.mem.u32(this.addr() + 8);
    if array == 0 {
        return;
    }
    let flags = e.mem.u32(array + 8);
    if (flags as i32) < 0 && e.mem.u8(collector.addr() + 0x10) == 0 {
        return;
    }
    let data = e.mem.u32(array);
    let allocated = e
        .call(GET_ALLOCATED_SIZE, &args![data, flags & CAPACITY_MASK])
        .u32();
    let data = e.mem.u32(array);
    let size = e.mem.u32(array + 4);
    e.vcall(
        collector.addr(),
        0x14,
        &args![3u32, ARRAY_LABEL, data, size, allocated],
    );
}

// Translated from 00d467c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMemoryTrack::write` (Xbox PDB), `this` = the track (layout as `00d46660`): appends `size`
/// bytes from `source`. When the last block is full a new block of the block size is taken from
/// the allocator (slot `4`) and its pointer added to the block array (`_reserveMore` when the
/// array is full) with an empty fill; bytes are copied into the last block up to its end, the
/// rest continues in new blocks. Nothing happens for a size of `0` or less.
pub fn memory_track_write(e: &mut Engine, this: Ptr, source: u32, size: i32) {
    if size <= 0 {
        return;
    }
    let t = this.addr();
    let array = t + 8;
    // Byte offset of the last block's pointer in the block array.
    let mut last = e.mem.u32(t + 0xc).wrapping_sub(1).wrapping_mul(4);
    let mut source = source;
    let mut size = size;
    loop {
        let block_size = e.mem.u32(t);
        let mut space = block_size.wrapping_sub(e.mem.u32(t + 4)) as i32;
        if space == 0 {
            let allocator = router_allocator(e);
            let fresh = e.vcall(allocator, 4, &args![block_size]).u32();
            let allocator = router_allocator(e);
            let capacity = (e.mem.u32(array + 8) & CAPACITY_MASK) as i32;
            if e.mem.u32(array + 4) as i32 == capacity {
                e.call(ARRAY_RESERVE_MORE, &args![allocator, array, 4u32]);
            }
            let count = e.mem.u32(array + 4);
            let data = e.mem.u32(array);
            e.mem
                .set_u32(data.wrapping_add(count.wrapping_mul(4)), fresh);
            e.mem.set_u32(array + 4, count.wrapping_add(1));
            last = last.wrapping_add(4);
            space = e.mem.u32(t) as i32;
            e.mem.set_u32(t + 4, 0);
        }
        let destination = e
            .mem
            .u32(e.mem.u32(array).wrapping_add(last))
            .wrapping_add(e.mem.u32(t + 4));
        if size <= space {
            e.call(MEM_COPY, &args![destination, source, size as u32]);
            e.mem
                .set_u32(t + 4, e.mem.u32(t + 4).wrapping_add(size as u32));
            return;
        }
        e.call(MEM_COPY, &args![destination, source, space as u32]);
        size = size.wrapping_sub(space);
        source = source.wrapping_add(space as u32);
        let block_size = e.mem.u32(t);
        e.mem.set_u32(t + 4, block_size);
        if size <= 0 {
            return;
        }
    }
}

// Translated from 00d468a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMemoryTrackStreamWriter::write` (Xbox PDB), `this` = the writer (track at `+8`): appends
/// `size` bytes from `source` to the track with `hkMemoryTrack::write` and returns `size`.
pub fn memory_track_stream_writer_write(e: &mut Engine, this: Ptr, source: u32, size: i32) -> i32 {
    let track = e.mem.u32(this.addr() + 8);
    memory_track_write(e, Ptr::new(track), source, size);
    size
}

// Translated from 00d468c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkArrayStreamWriter::clear` (Xbox PDB), `this` = the writer (byte array at `+8`, position
/// `+0xc`): empties the array and the position, makes room for one byte and stores a zero byte at
/// the start.
pub fn array_stream_writer_clear(e: &mut Engine, this: Ptr) {
    let w = this.addr();
    let array = e.mem.u32(w + 8);
    e.mem.set_u32(array + 4, 0);
    e.mem.set_u32(w + 0xc, 0);
    let needed = e.mem.u32(array + 4).wrapping_add(1) as i32;
    reserve_at_least(e, array, needed);
    let array = e.mem.u32(w + 8);
    let data = e.mem.u32(array);
    e.mem.set_u8(data.wrapping_add(e.mem.u32(array + 4)), 0);
}

// Translated from 00d46920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkArrayStreamWriter::write` (Xbox PDB), `this` = the writer (byte array at `+8`, position
/// `+0xc`): writes `count` bytes from `source` at the position. When that passes the array's size
/// the array grows to the new end plus a terminating zero byte (the size becomes the new end);
/// otherwise a zero byte is stored at the size when the capacity allows. The position advances and
/// `count` is returned.
pub fn array_stream_writer_write(e: &mut Engine, this: Ptr, source: u32, count: i32) -> i32 {
    let w = this.addr();
    let array = e.mem.u32(w + 8);
    let size = e.mem.u32(array + 4);
    let remaining = size.wrapping_sub(e.mem.u32(w + 0xc)) as i32;
    if count > remaining {
        let end = size
            .wrapping_sub(remaining as u32)
            .wrapping_add(count as u32);
        reserve_at_least(e, array, end.wrapping_add(1) as i32);
        let array = e.mem.u32(w + 8);
        e.mem.set_u32(array + 4, end);
        let data = e.mem.u32(e.mem.u32(w + 8));
        e.mem.set_u8(end.wrapping_add(data), 0);
    } else {
        let capacity = (e.mem.u32(array + 8) & CAPACITY_MASK) as i32;
        if capacity > size as i32 {
            let data = e.mem.u32(array);
            e.mem.set_u8(size.wrapping_add(data), 0);
        }
    }
    let destination = e.mem.u32(e.mem.u32(w + 8)).wrapping_add(e.mem.u32(w + 0xc));
    e.call(MEM_COPY, &args![destination, source, count as u32]);
    e.mem
        .set_u32(w + 0xc, e.mem.u32(w + 0xc).wrapping_add(count as u32));
    count
}

// Translated from 00d469e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMemoryTrack::clear` (Xbox PDB), `this` = the track: gives every block back to the allocator
/// (slot `8`, `(block, block size)`), then the block array if the track owns it (slot `0x10`),
/// leaves no blocks, no array (capacity word `0x80000000`) and the last block's fill at the block
/// size.
pub fn memory_track_clear(e: &mut Engine, this: Ptr) {
    let t = this.addr();
    let mut index = 0u32;
    if (e.mem.u32(t + 0xc) as i32) > 0 {
        loop {
            let block_size = e.mem.u32(t);
            let block = e
                .mem
                .u32(e.mem.u32(t + 8).wrapping_add(index.wrapping_mul(4)));
            let allocator = router_allocator(e);
            e.vcall(allocator, 8, &args![block, block_size]);
            index = index.wrapping_add(1);
            if (index as i32) >= (e.mem.u32(t + 0xc) as i32) {
                break;
            }
        }
    }
    release_block_array(e, t);
    e.mem.set_u32(t + 8, 0);
    e.mem.set_u32(t + 0x10, NOT_OWNED);
    e.mem.set_u32(t + 4, e.mem.u32(t));
}

// Translated from 00d46b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMemoryTrackStreamWriter::clear` (Xbox PDB), `this` = the writer: clears its track
/// (`hkMemoryTrack::clear` on the pointer at `+8`).
pub fn memory_track_stream_writer_clear(e: &mut Engine, this: Ptr) {
    let track = e.mem.u32(this.addr() + 8);
    memory_track_clear(e, Ptr::new(track));
}

// Translated from 00d46b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkArrayStreamWriter::seek` (Xbox PDB), `this` = the writer (byte array at `+8`, position
/// `+0xc`), `(offset, whence)`: `whence` `0` sets the position to `offset`, `1` adds it to the
/// position, `2` subtracts it from the array's size, anything else keeps the position. A negative
/// target returns `1` and changes nothing. A target past the array's size grows the array to the
/// target plus one byte, zero-fills the new bytes and sets the size to the target. The position
/// becomes the target and `0` is returned.
pub fn array_stream_writer_seek(e: &mut Engine, this: Ptr, offset: i32, whence: u32) -> u32 {
    let w = this.addr();
    let mut target = e.mem.u32(w + 0xc) as i32;
    match whence {
        0 => target = offset,
        1 => target = target.wrapping_add(offset),
        2 => {
            let array = e.mem.u32(w + 8);
            target = (e.mem.u32(array + 4) as i32).wrapping_sub(offset);
        }
        _ => {}
    }
    if target < 0 {
        return 1;
    }
    let array = e.mem.u32(w + 8);
    if target > e.mem.u32(array + 4) as i32 {
        let needed = target.wrapping_add(1);
        reserve_at_least(e, array, needed);
        let size = e.mem.u32(array + 4);
        let data = e.mem.u32(array);
        let extra = (needed as u32).wrapping_sub(size) as i32;
        if extra > 0 {
            e.call(MEM_SET, &args![data.wrapping_add(size), 0u32, extra as u32]);
        }
        e.mem.set_u32(array + 4, needed as u32);
        let array = e.mem.u32(w + 8);
        e.mem.set_u32(array + 4, target as u32);
    }
    e.mem.set_u32(w + 0xc, target as u32);
    0
}

// Translated from 00d46be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the memory track (the class is unconfirmed), `this` = the track: clears it
/// (`00d469e0`), then, as `clear` has left the array unowned, releases the block array again
/// when its capacity word says it is owned, and ends with no blocks, no array and the capacity
/// word `0x80000000`.
pub fn destructor_00d46be0(e: &mut Engine, this: Ptr) {
    let t = this.addr();
    memory_track_clear(e, this);
    release_block_array(e, t);
    e.mem.set_u32(t + 0x10, NOT_OWNED);
    e.mem.set_u32(t + 8, 0);
}

// Translated from 00d46c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the stream writers (vtable `010cf734`; the class is unconfirmed), `this` = the
/// writer: when the ownership word at `+0x10` is `1` and a track exists at `+8`, destroys the
/// track (`00d46be0`) and gives its `0x14` bytes back to the allocator (slot `8`). The vtable is
/// set to `0102e388` in every case.
pub fn destructor_00d46c30(e: &mut Engine, this: Ptr) {
    let w = this.addr();
    e.mem.set_u32(w, WRITER_VTABLE);
    if e.mem.u32(w + 0x10) == 1 {
        let track = e.mem.u32(w + 8);
        if track != 0 {
            destructor_00d46be0(e, Ptr::new(track));
            let allocator = router_allocator(e);
            e.vcall(allocator, 8, &args![track, 0x14u32]);
        }
    }
    e.mem.set_u32(w, BASE_VTABLE);
}

// Translated from 00d46c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMemoryTrackStreamReader::skip` (Xbox PDB), `this` = the reader: advances the position at
/// `+0xc` by `count`.
/// Answers `count` (`hkStreamReader::skip` returns `int` in the Xbox PDB; `EAX` is `count`, loaded
/// at 00d46c80, at the RET 00d46c87). No exe reader of the slot +0x14 through an
/// hkStreamReader was found.
pub fn memory_track_stream_reader_skip(e: &mut Engine, this: Ptr, count: u32) -> u32 {
    let at = this.addr() + 0xc;
    e.mem.set_u32(at, e.mem.u32(at).wrapping_add(count));
    count
}

// Translated from 00d46c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// A stream method (`RET 4`) that returns `1` whatever it is given (the slot is unconfirmed).
pub fn returns_true_00d46c90(_e: &mut Engine, _this: Ptr, _unused_1: u32) -> u32 {
    1
}

// Translated from 00d46ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMemoryTrackStreamReader::isOk` (Xbox PDB), `this` = the reader (track `+8`, position
/// `+0xc`), `(result)`: stores through `result` whether the position is at most the track's
/// length (`(blocks - 1) * block size + last fill`, signed) and returns `result`.
pub fn memory_track_stream_reader_is_ok(e: &mut Engine, this: Ptr, result: u32) -> u32 {
    let r = this.addr();
    let track = e.mem.u32(r + 8);
    let end = e
        .mem
        .u32(track + 0xc)
        .wrapping_sub(1)
        .wrapping_mul(e.mem.u32(track))
        .wrapping_add(e.mem.u32(track + 4));
    let ok = (e.mem.u32(r + 0xc) as i32) <= (end as i32);
    e.mem.set_u8(result, u8::from(ok));
    result
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d4_6660, memory_track_read(Ptr, u32, i32, i32)),
        entry!(0x00d4_66e0, constructor_00d466e0(Ptr, u32, u32) -> Ptr),
        entry!(0x00d4_6710, memory_track_stream_reader_read(Ptr, u32, i32) -> i32),
        entry!(
            0x00d4_6770,
            array_stream_writer_calc_content_statistics(Ptr, Ptr, u32)
        ),
        entry!(0x00d4_67c0, memory_track_write(Ptr, u32, i32)),
        entry!(0x00d4_68a0, memory_track_stream_writer_write(Ptr, u32, i32) -> i32),
        entry!(0x00d4_68c0, array_stream_writer_clear(Ptr)),
        entry!(0x00d4_6920, array_stream_writer_write(Ptr, u32, i32) -> i32),
        entry!(0x00d4_69e0, memory_track_clear(Ptr)),
        entry!(0x00d4_6b20, memory_track_stream_writer_clear(Ptr)),
        entry!(0x00d4_6b30, array_stream_writer_seek(Ptr, i32, u32) -> u32),
        entry!(0x00d4_6be0, destructor_00d46be0(Ptr)),
        entry!(0x00d4_6c30, destructor_00d46c30(Ptr)),
        entry!(0x00d4_6c80, memory_track_stream_reader_skip(Ptr, u32) -> u32),
        entry!(0x00d4_6c90, returns_true_00d46c90(Ptr, u32) -> u32),
        entry!(0x00d4_6ca0, memory_track_stream_reader_is_ok(Ptr, u32) -> u32),
        entry!(0x00d4_6cc0, deleting_destructor_00d46cc0(Ptr, u8) -> Ptr),
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
    fn deleting_destructor_00d46cc0_frees_only_when_bit_zero_is_set() {
        check(0x00d4_6cc0, 0x00d4_6c30);
    }

    use std::cell::Cell;

    type Words = Vec<u32>;
    type Log = Rc<RefCell<Vec<(u32, Words)>>>;

    const ALLOCATE_SLOT: u32 = 0x0900_0021;
    const FREE_BLOCK_SLOT: u32 = 0x0900_0022;
    const FREE_ARRAY_SLOT: u32 = 0x0900_0023;
    const STATISTICS_SLOT: u32 = 0x0900_0024;
    const IS_OK_SLOT: u32 = 0x0900_0025;
    const ALLOCATED_SIZE: u32 = 0x77;

    /// The words of every logged call with the tag `tag`.
    fn logged(log: &Log, tag: u32) -> Vec<Words> {
        log.borrow()
            .iter()
            .filter(|(at, _)| *at == tag)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// Replaces the storage of the `hkArray` at `array` with a fresh buffer of `capacity` entries
    /// of `element` bytes holding the old contents, and records the capacity.
    fn regrow(e: &mut Engine, array: u32, capacity: u32, element: u32) {
        let old = e.mem.u32(array);
        let used = e.mem.u32(array + 4) * element;
        let fresh = e.mem.alloc(capacity * element);
        let kept = e.mem.bytes(old, used);
        e.mem.write(fresh, &kept);
        e.mem.set_u32(array, fresh);
        e.mem.set_u32(array + 8, capacity);
    }

    /// An engine with the memory router and its allocator (slots `4`, `8` and `0x10` log their
    /// words, slot `4` hands out fresh blocks), a real `memCpy` and `memset`, the two array
    /// reserve routines (which log and move the array to a bigger buffer) and the statistics
    /// size routine (which answers [`ALLOCATED_SIZE`]). Returns the log and the allocator.
    fn track_engine() -> (Engine, Log, u32) {
        let mut e = engine();
        let log: Log = Rc::default();
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 4, ALLOCATE_SLOT);
        e.mem.set_u32(vtable + 8, FREE_BLOCK_SLOT);
        e.mem.set_u32(vtable + 0x10, FREE_ARRAY_SLOT);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let l = Rc::clone(&log);
        e.register_double(ALLOCATE_SLOT, move |e, a| {
            l.borrow_mut().push((4, a[1..].to_vec()));
            Ret {
                eax: e.mem.alloc(a[1]),
                ..Ret::default()
            }
        });
        let l = Rc::clone(&log);
        e.register_double(FREE_BLOCK_SLOT, move |_, a| {
            l.borrow_mut().push((8, a[1..].to_vec()));
            Ret::default()
        });
        let l = Rc::clone(&log);
        e.register_double(FREE_ARRAY_SLOT, move |_, a| {
            l.borrow_mut().push((0x10, a[1..].to_vec()));
            Ret::default()
        });
        let l = Rc::clone(&log);
        e.register_double(MEM_COPY, move |e, a| {
            l.borrow_mut().push((MEM_COPY, a.to_vec()));
            let data = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &data);
            Ret::default()
        });
        let l = Rc::clone(&log);
        e.register_double(MEM_SET, move |e, a| {
            l.borrow_mut().push((MEM_SET, a.to_vec()));
            for offset in 0..a[2] {
                e.mem.set_u8(a[0] + offset, a[1] as u8);
            }
            Ret::default()
        });
        let l = Rc::clone(&log);
        e.register_double(ARRAY_RESERVE, move |e, a| {
            l.borrow_mut().push((ARRAY_RESERVE, a.to_vec()));
            regrow(e, a[1], a[2], a[3]);
            Ret::default()
        });
        let l = Rc::clone(&log);
        e.register_double(ARRAY_RESERVE_MORE, move |e, a| {
            l.borrow_mut().push((ARRAY_RESERVE_MORE, a.to_vec()));
            let doubled = (e.mem.u32(a[1] + 8) & CAPACITY_MASK) * 2;
            regrow(e, a[1], doubled, a[2]);
            Ret::default()
        });
        let l = Rc::clone(&log);
        e.register_double(GET_ALLOCATED_SIZE, move |_, a| {
            l.borrow_mut().push((GET_ALLOCATED_SIZE, a.to_vec()));
            Ret {
                eax: ALLOCATED_SIZE,
                ..Ret::default()
            }
        });
        (e, log, allocator)
    }

    /// A memory track with the given block size and last fill, one block per entry of `blocks`
    /// (each filled from its bytes) and room for `capacity` block pointers.
    fn track(e: &mut Engine, block_size: u32, fill: u32, blocks: &[Vec<u8>], capacity: u32) -> u32 {
        let t = e.mem.alloc(0x14);
        let array = e.mem.alloc(4 * capacity);
        for (index, contents) in blocks.iter().enumerate() {
            let block = e.mem.alloc(block_size);
            e.mem.write(block, contents);
            e.mem.set_u32(array + 4 * index as u32, block);
        }
        e.mem.set_u32(t, block_size);
        e.mem.set_u32(t + 4, fill);
        e.mem.set_u32(t + 8, array);
        e.mem.set_u32(t + 0xc, blocks.len() as u32);
        e.mem.set_u32(t + 0x10, capacity);
        t
    }

    /// The track used by most tests: blocks of 8 bytes holding 0 to 20, the last one filled to 5.
    fn counting_track(e: &mut Engine) -> u32 {
        track(
            e,
            8,
            5,
            &[(0..8).collect(), (8..16).collect(), (16..21).collect()],
            4,
        )
    }

    /// The pointer of block `index` of `track`.
    fn block_of(e: &Engine, track: u32, index: u32) -> u32 {
        e.mem.u32(e.mem.u32(track + 8) + 4 * index)
    }

    /// A byte `hkArray` with `contents`, `size` used entries and the capacity word `flags`.
    fn byte_array(e: &mut Engine, contents: &[u8], size: u32, flags: u32) -> u32 {
        let array = e.mem.alloc(0x10);
        let data = e.mem.alloc(contents.len() as u32 + 1);
        e.mem.write(data, contents);
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 4, size);
        e.mem.set_u32(array + 8, flags);
        array
    }

    /// A stream object whose field `+8` is `target` and whose position (`+0xc`) is `position`.
    fn stream(e: &mut Engine, target: u32, position: u32) -> u32 {
        let object = e.mem.alloc(0x14);
        e.mem.set_u32(object + 8, target);
        e.mem.set_u32(object + 0xc, position);
        object
    }

    #[test]
    fn memory_track_read_copies_across_blocks() {
        let (mut e, log, _) = track_engine();
        let t = counting_track(&mut e);
        let dest = e.mem.alloc(32);
        e.call(0x00d4_6660, &args![t, dest, 10i32, 6i32]);
        assert_eq!(e.mem.bytes(dest, 10), (6..16).collect::<Vec<u8>>());
        let sizes: Vec<u32> = logged(&log, MEM_COPY).iter().map(|a| a[2]).collect();
        assert_eq!(sizes, [2, 8]);
        // A read that ends in the last block stops at its fill.
        log.borrow_mut().clear();
        e.call(0x00d4_6660, &args![t, dest, 7i32, 14i32]);
        assert_eq!(e.mem.bytes(dest, 7), (14..21).collect::<Vec<u8>>());
        let copies = logged(&log, MEM_COPY);
        assert_eq!(copies.len(), 2);
        assert_eq!(copies[0][..], [dest, block_of(&e, t, 1) + 6, 2]);
        assert_eq!(copies[1][..], [dest + 2, block_of(&e, t, 2), 5]);
        // A whole block, the last block's fill, and nothing at all.
        log.borrow_mut().clear();
        e.call(0x00d4_6660, &args![t, dest, 8i32, 8i32]);
        e.call(0x00d4_6660, &args![t, dest, 5i32, 16i32]);
        e.call(0x00d4_6660, &args![t, dest, 0i32, 3i32]);
        e.call(0x00d4_6660, &args![t, dest, -4i32, 3i32]);
        assert_eq!(logged(&log, MEM_COPY).len(), 2);
    }

    #[test]
    fn constructor_00d466e0_sets_the_fields() {
        let mut e = engine();
        let this = e.mem.alloc(0x14);
        let got = e.call(0x00d4_66e0, &args![this, 0x1234u32, 1u32]).u32();
        assert_eq!(got, this);
        assert_eq!(e.mem.u32(this), WRITER_VTABLE);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this + 8), 0x1234);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x10), 1);
    }

    /// A reader of `track` at `position` whose `isOk` slot (`0xc`) answers `answer`.
    fn reader(e: &mut Engine, track: u32, position: u32, answer: &Rc<Cell<u8>>) -> u32 {
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0xc, IS_OK_SLOT);
        let object = stream(e, track, position);
        e.mem.set_u32(object, vtable);
        let answer = Rc::clone(answer);
        e.register_double(IS_OK_SLOT, move |e, a| {
            e.mem.set_u8(a[1], answer.get());
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        object
    }

    #[test]
    fn memory_track_stream_reader_read_is_limited_by_the_track_length() {
        let (mut e, _log, _) = track_engine();
        let t = counting_track(&mut e);
        let answer = Rc::new(Cell::new(1));
        let r = reader(&mut e, t, 3, &answer);
        let dest = e.mem.alloc(32);
        // 21 bytes in all: 18 remain after the position, the position moves by the request.
        let got = e.call(0x00d4_6710, &args![r, dest, 100i32]).u32();
        assert_eq!(got, 18);
        assert_eq!(e.mem.bytes(dest, 18), (3..21).collect::<Vec<u8>>());
        assert_eq!(e.mem.u32(r + 0xc), 103);
        // A smaller request is read in full.
        e.mem.set_u32(r + 0xc, 3);
        let got = e.call(0x00d4_6710, &args![r, dest, 4i32]).u32();
        assert_eq!(got, 4);
        assert_eq!(e.mem.bytes(dest, 4), [3, 4, 5, 6]);
        assert_eq!(e.mem.u32(r + 0xc), 7);
        // Past the end the count is negative: nothing is copied but the request still counts.
        e.mem.set_u32(r + 0xc, 30);
        let got = e.call(0x00d4_6710, &args![r, dest, 5i32]).u32();
        assert_eq!(got as i32, -9);
        assert_eq!(e.mem.u32(r + 0xc), 35);
        // A stream that is not ok reads nothing and does not move.
        answer.set(0);
        e.mem.set_u32(r + 0xc, 3);
        let got = e.call(0x00d4_6710, &args![r, dest, 4i32]).u32();
        assert_eq!(got, 0);
        assert_eq!(e.mem.u32(r + 0xc), 3);
    }

    #[test]
    fn calc_content_statistics_reports_owned_or_requested_arrays() {
        let (mut e, log, _) = track_engine();
        let reports = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&reports);
        e.register_double(STATISTICS_SLOT, move |_, a| {
            sink.borrow_mut().push(a[1..].to_vec());
            Ret::default()
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0x14, STATISTICS_SLOT);
        let collector = e.mem.alloc(0x20);
        e.mem.set_u32(collector, vtable);
        let array = byte_array(&mut e, &[1, 2, 3, 4, 5], 5, 8);
        let data = e.mem.u32(array);
        let writer = stream(&mut e, array, 0);
        e.call(0x00d4_6770, &args![writer, collector, 0u32]);
        assert_eq!(
            reports.borrow()[..],
            [vec![3, ARRAY_LABEL, data, 5, ALLOCATED_SIZE]]
        );
        assert_eq!(logged(&log, GET_ALLOCATED_SIZE)[0][..], [data, 8]);
        // A negative capacity word (storage not owned) is only reported when the collector asks.
        reports.borrow_mut().clear();
        e.mem.set_u32(array + 8, NOT_OWNED | 8);
        e.call(0x00d4_6770, &args![writer, collector, 0u32]);
        assert!(reports.borrow().is_empty());
        e.mem.set_u8(collector + 0x10, 1);
        e.call(0x00d4_6770, &args![writer, collector, 0u32]);
        assert_eq!(reports.borrow().len(), 1);
        assert_eq!(logged(&log, GET_ALLOCATED_SIZE)[1][..], [data, 8]);
        // No array, no report.
        reports.borrow_mut().clear();
        e.mem.set_u32(writer + 8, 0);
        e.call(0x00d4_6770, &args![writer, collector, 0u32]);
        assert!(reports.borrow().is_empty());
    }

    #[test]
    fn memory_track_write_fills_the_last_block_then_takes_new_ones() {
        let (mut e, log, allocator) = track_engine();
        let t = track(&mut e, 8, 5, &[vec![0xa0; 8]], 1);
        let source = e.mem.alloc(32);
        let contents: Vec<u8> = (1..=20).collect();
        e.mem.write(source, &contents);
        e.call(0x00d4_67c0, &args![t, source, 20i32]);
        assert_eq!(e.mem.u32(t + 0xc), 4);
        assert_eq!(e.mem.u32(t + 4), 1);
        let first = e.mem.bytes(block_of(&e, t, 0), 8);
        assert_eq!(first, [0xa0, 0xa0, 0xa0, 0xa0, 0xa0, 1, 2, 3]);
        assert_eq!(
            e.mem.bytes(block_of(&e, t, 1), 8),
            (4..12).collect::<Vec<u8>>()
        );
        assert_eq!(
            e.mem.bytes(block_of(&e, t, 2), 8),
            (12..20).collect::<Vec<u8>>()
        );
        assert_eq!(e.mem.u8(block_of(&e, t, 3)), 20);
        let sizes: Vec<u32> = logged(&log, MEM_COPY).iter().map(|a| a[2]).collect();
        assert_eq!(sizes, [3, 8, 8, 1]);
        assert_eq!(logged(&log, 4), vec![vec![8], vec![8], vec![8]]);
        // The block array of capacity 1 grew when it was full: at 1 and at 2 entries.
        let growth = logged(&log, ARRAY_RESERVE_MORE);
        assert_eq!(growth.len(), 2);
        assert_eq!(growth[0][..], [allocator, t + 8, 4]);
        // A full last block takes a new one even for a short write; nothing for a size of 0.
        log.borrow_mut().clear();
        let full = track(&mut e, 4, 4, &[vec![9; 4]], 2);
        e.call(0x00d4_67c0, &args![full, source, 2i32]);
        assert_eq!(e.mem.u32(full + 0xc), 2);
        assert_eq!(e.mem.u32(full + 4), 2);
        assert_eq!(e.mem.bytes(block_of(&e, full, 1), 2), [1, 2]);
        assert_eq!(logged(&log, 4), vec![vec![4]]);
        log.borrow_mut().clear();
        e.call(0x00d4_67c0, &args![full, source, 0i32]);
        e.call(0x00d4_67c0, &args![full, source, -3i32]);
        assert!(log.borrow().is_empty());
        assert_eq!(e.mem.u32(full + 4), 2);
    }

    #[test]
    fn memory_track_stream_writer_write_appends_to_its_track() {
        let (mut e, _log, _) = track_engine();
        let t = track(&mut e, 8, 5, &[vec![0; 8]], 2);
        let writer = stream(&mut e, t, 0);
        let source = e.mem.alloc(8);
        e.mem.write(source, &[7, 8, 9]);
        let got = e.call(0x00d4_68a0, &args![writer, source, 3i32]).u32();
        assert_eq!(got, 3);
        assert_eq!(e.mem.u32(t + 4), 8);
        assert_eq!(e.mem.bytes(block_of(&e, t, 0) + 5, 3), [7, 8, 9]);
    }

    #[test]
    fn array_stream_writer_clear_keeps_room_for_the_terminator() {
        let (mut e, log, allocator) = track_engine();
        let array = byte_array(&mut e, &[9, 9, 9, 9], 4, 2);
        let writer = stream(&mut e, array, 3);
        e.call(0x00d4_68c0, &args![writer]);
        assert_eq!(e.mem.u32(array + 4), 0);
        assert_eq!(e.mem.u32(writer + 0xc), 0);
        assert_eq!(e.mem.u8(e.mem.u32(array)), 0);
        assert!(logged(&log, ARRAY_RESERVE).is_empty());
        // Storage that is not owned has no capacity: room for one byte is reserved.
        let bare = byte_array(&mut e, &[9], 1, NOT_OWNED);
        let writer = stream(&mut e, bare, 1);
        e.call(0x00d4_68c0, &args![writer]);
        assert_eq!(logged(&log, ARRAY_RESERVE)[0][..], [allocator, bare, 1, 1]);
        assert_eq!(e.mem.u32(bare + 4), 0);
        assert_eq!(e.mem.u8(e.mem.u32(bare)), 0);
    }

    #[test]
    fn array_stream_writer_write_grows_the_array_or_terminates_it() {
        let (mut e, log, allocator) = track_engine();
        let source = e.mem.alloc(16);
        e.mem.write(source, b"XYZUVW");
        // Inside the array and with spare capacity: a zero byte follows the size.
        let array = byte_array(&mut e, b"abcd\x55", 4, 8);
        let writer = stream(&mut e, array, 2);
        let got = e.call(0x00d4_6920, &args![writer, source, 2i32]).u32();
        assert_eq!(got, 2);
        let data = e.mem.u32(array);
        assert_eq!(e.mem.bytes(data, 5), b"abXY\0");
        assert_eq!(e.mem.u32(writer + 0xc), 4);
        // Past the end: twice the capacity is reserved when it is larger than the new end + 1.
        let got = e.call(0x00d4_6920, &args![writer, source, 6i32]).u32();
        assert_eq!(got, 6);
        assert_eq!(
            logged(&log, ARRAY_RESERVE)[0][..],
            [allocator, array, 16, 1]
        );
        assert_eq!(e.mem.u32(array + 4), 10);
        let data = e.mem.u32(array);
        assert_eq!(e.mem.bytes(data, 11), b"abXYXYZUVW\0");
        assert_eq!(e.mem.u32(writer + 0xc), 10);
        // A small capacity reserves exactly the new end + 1.
        let small = byte_array(&mut e, b"", 0, 2);
        let writer = stream(&mut e, small, 0);
        e.call(0x00d4_6920, &args![writer, source, 6i32]);
        assert_eq!(logged(&log, ARRAY_RESERVE)[1][..], [allocator, small, 7, 1]);
        // Capacity equal to the size: no terminator is stored when writing inside the array.
        let tight = byte_array(&mut e, b"abcd\x55", 4, 4);
        let writer = stream(&mut e, tight, 1);
        e.call(0x00d4_6920, &args![writer, source, 2i32]);
        let data = e.mem.u32(tight);
        assert_eq!(e.mem.bytes(data, 5), b"aXYd\x55");
    }

    #[test]
    fn memory_track_clear_gives_everything_back() {
        let (mut e, log, _) = track_engine();
        let t = counting_track(&mut e);
        let blocks: Vec<u32> = (0..3).map(|index| block_of(&e, t, index)).collect();
        let array = e.mem.u32(t + 8);
        e.call(0x00d4_69e0, &args![t]);
        assert_eq!(
            log.borrow()[..],
            [
                (8, vec![blocks[0], 8]),
                (8, vec![blocks[1], 8]),
                (8, vec![blocks[2], 8]),
                (0x10, vec![array, 16]),
            ]
        );
        assert_eq!(e.mem.u32(t + 0xc), 0);
        assert_eq!(e.mem.u32(t + 8), 0);
        assert_eq!(e.mem.u32(t + 0x10), NOT_OWNED);
        assert_eq!(e.mem.u32(t + 4), 8);
        // Without blocks and with storage that is not owned only the fields are reset.
        log.borrow_mut().clear();
        let empty = track(&mut e, 16, 3, &[], 1);
        e.mem.set_u32(empty + 0x10, NOT_OWNED | 1);
        e.call(0x00d4_69e0, &args![empty]);
        assert!(log.borrow().is_empty());
        assert_eq!(e.mem.u32(empty + 0x10), NOT_OWNED);
        assert_eq!(e.mem.u32(empty + 4), 16);
    }

    #[test]
    fn memory_track_stream_writer_clear_clears_its_track() {
        let (mut e, log, _) = track_engine();
        let t = track(&mut e, 8, 2, &[vec![1; 8]], 1);
        let block = block_of(&e, t, 0);
        let writer = stream(&mut e, t, 0);
        e.call(0x00d4_6b20, &args![writer]);
        assert_eq!(logged(&log, 8), vec![vec![block, 8]]);
        assert_eq!(e.mem.u32(t + 4), 8);
    }

    #[test]
    fn array_stream_writer_seek_moves_and_extends() {
        let (mut e, log, allocator) = track_engine();
        let array = byte_array(&mut e, b"abcd", 4, 8);
        let writer = stream(&mut e, array, 2);
        // Set, relative, from the end and an unknown mode that keeps the position.
        assert_eq!(e.call(0x00d4_6b30, &args![writer, 3i32, 0u32]).u32(), 0);
        assert_eq!(e.mem.u32(writer + 0xc), 3);
        assert_eq!(e.call(0x00d4_6b30, &args![writer, 1i32, 1u32]).u32(), 0);
        assert_eq!(e.mem.u32(writer + 0xc), 4);
        assert_eq!(e.call(0x00d4_6b30, &args![writer, 3i32, 2u32]).u32(), 0);
        assert_eq!(e.mem.u32(writer + 0xc), 1);
        assert_eq!(e.call(0x00d4_6b30, &args![writer, 9i32, 7u32]).u32(), 0);
        assert_eq!(e.mem.u32(writer + 0xc), 1);
        assert!(log.borrow().is_empty());
        // A negative target fails without a change.
        assert_eq!(e.call(0x00d4_6b30, &args![writer, -1i32, 0u32]).u32(), 1);
        assert_eq!(e.mem.u32(writer + 0xc), 1);
        // Past the size with enough capacity: the new bytes and the byte after are cleared.
        let data = e.mem.u32(array);
        e.mem.write(data + 4, &[0x55, 0x55, 0x55]);
        e.call(0x00d4_6b30, &args![writer, 5i32, 0u32]);
        assert_eq!(logged(&log, MEM_SET)[0][..], [data + 4, 0, 2]);
        assert_eq!(e.mem.u32(array + 4), 5);
        assert_eq!(e.mem.bytes(data + 4, 3), [0, 0, 0x55]);
        assert_eq!(e.mem.u32(writer + 0xc), 5);
        // Past the capacity: the larger of twice the capacity and the target + 1 is reserved.
        e.call(0x00d4_6b30, &args![writer, 20i32, 0u32]);
        assert_eq!(
            logged(&log, ARRAY_RESERVE)[0][..],
            [allocator, array, 21, 1]
        );
        let data = e.mem.u32(array);
        assert_eq!(logged(&log, MEM_SET)[1][..], [data + 5, 0, 16]);
        assert_eq!(e.mem.u32(array + 4), 20);
        assert_eq!(e.mem.u32(writer + 0xc), 20);
    }

    #[test]
    fn destructor_00d46be0_leaves_an_empty_track() {
        let (mut e, log, _) = track_engine();
        let t = track(&mut e, 8, 3, &[vec![1; 8], vec![2; 8]], 2);
        let blocks = [block_of(&e, t, 0), block_of(&e, t, 1)];
        let array = e.mem.u32(t + 8);
        e.call(0x00d4_6be0, &args![t]);
        // The clear gave everything back; the second release finds nothing owned.
        assert_eq!(
            log.borrow()[..],
            [
                (8, vec![blocks[0], 8]),
                (8, vec![blocks[1], 8]),
                (0x10, vec![array, 8]),
            ]
        );
        assert_eq!(e.mem.u32(t + 8), 0);
        assert_eq!(e.mem.u32(t + 0xc), 0);
        assert_eq!(e.mem.u32(t + 0x10), NOT_OWNED);
    }

    #[test]
    fn destructor_00d46c30_destroys_an_owned_track() {
        let (mut e, log, _) = track_engine();
        let t = track(&mut e, 8, 3, &[], 1);
        let array = e.mem.u32(t + 8);
        let writer = stream(&mut e, t, 0);
        e.mem.set_u32(writer + 0x10, 1);
        e.call(0x00d4_6c30, &args![writer]);
        assert_eq!(e.mem.u32(writer), BASE_VTABLE);
        assert_eq!(
            log.borrow()[..],
            [(0x10, vec![array, 4]), (8, vec![t, 0x14])]
        );
        // A track that is not owned, or no track at all, is left alone.
        log.borrow_mut().clear();
        let other = track(&mut e, 8, 3, &[], 1);
        let writer = stream(&mut e, other, 0);
        e.mem.set_u32(writer + 0x10, 2);
        e.call(0x00d4_6c30, &args![writer]);
        assert_eq!(e.mem.u32(writer), BASE_VTABLE);
        let writer = stream(&mut e, 0, 0);
        e.mem.set_u32(writer + 0x10, 1);
        e.call(0x00d4_6c30, &args![writer]);
        assert_eq!(e.mem.u32(writer), BASE_VTABLE);
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn reader_skip_answers_the_count() {
        let (mut e, _log, _) = track_engine();
        let t = counting_track(&mut e);
        let r = stream(&mut e, t, 10);
        // EAX is the count at the RET 00d46c87 (`int skip(int)`).
        assert_eq!(e.call(0x00d4_6c80, &args![r, 5u32]).u32(), 5);
    }

    #[test]
    fn reader_skip_and_status_functions() {
        let (mut e, _log, _) = track_engine();
        let t = counting_track(&mut e);
        let r = stream(&mut e, t, 10);
        e.call(0x00d4_6c80, &args![r, 5u32]);
        assert_eq!(e.mem.u32(r + 0xc), 15);
        assert_eq!(e.call(0x00d4_6c90, &args![r, 0u32]).u32(), 1);
        // The track holds 21 bytes: a position up to 21 is ok, a negative one too (signed).
        let result = e.mem.alloc(4);
        for (position, ok) in [(21u32, 1u8), (22, 0), (0xffff_fff0, 1), (0, 1)] {
            e.mem.set_u32(r + 0xc, position);
            e.mem.set_u8(result, 0xee);
            let got = e.call(0x00d4_6ca0, &args![r, result]).u32();
            assert_eq!(got, result);
            assert_eq!(e.mem.u8(result), ok, "position {position:#x}");
        }
    }
}

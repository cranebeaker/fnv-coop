//! `Havok SDK/hkadeltacompressedanimation.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its
//! functions in FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkaDeltaCompressedAnimation` (Xbox PDB name) keeps its tracks in blocks of `+0x2c` frames. The
//! class is not in the Xbox PDB's type list, so the fields are read off the code (PC offsets):
//! `+0x10` / `+0x14` the transform / float track counts, `+0x28` the frame count, `+0x2c` the frames
//! per block, `+0x30` / `+0x31` the two bytes of the default quantisation record, `+0x34` the
//! floats per frame, `+0x38` / `+0x3c` / `+0x40` the offsets of the two float arrays and the byte
//! array inside a block's static data, `+0x44` the size of that static data, `+0x4c` / `+0x54` /
//! `+0x5c` / `+0x60` the offsets of the track description, the static floats and the two
//! recomposition ranges, `+0x64` the bytes of a block, `+0x68` the bytes of the last block and
//! `+0x6c` the pointer to the compressed data.
//!
//! Functions that take registers are written with the register values first, then the stack
//! words; the doc comment of each says which. `RDTSC` is the host clock (the profiling records are
//! timings only). x87 arithmetic is done in `f64` and rounded to `f32` at each store, and a
//! `float` copied through `FLD`/`FSTP` goes through [`x87_quiet_bits`].
//!
//! The per-function arrays of `samplePartialTracks` are `hkLocalArray`s: four words (data,
//! size, capacity and flags, the local block) on the stack, backed by the thread's stack memory
//! router (`00c85750`). Not translated: nothing (none of these functions has an exception
//! frame).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpconvexradiusviewer::time_stamp_low;

/// `TlsGetValue` import slot and the TLS index of the profiling record stream (`0126835c`).
const TLS_GET_VALUE: u32 = 0x00fd_f044;
const PROFILE_SLOT: u32 = 0x0126_835c;
/// The thread's stack memory router (`00c85750`, no arguments): its field `+0` is the stack
/// cursor and `+0x10` the allocator.
const STACK_ROUTER: u32 = 0x00c8_5750;
/// `hkArrayUtil::_reserve` (`00c908c0`, cdecl): `(allocator, array, capacity, element size)`.
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
/// `hkExpandReal` (`00e51660`, cdecl): `(data, record, destination, count)`.
const EXPAND_REAL: u32 = 0x00e5_1660;
/// `hkInverseDeltaTransform` (`00e592a0`, cdecl): `(values, count)`.
const INVERSE_DELTA_TRANSFORM: u32 = 0x00e5_92a0;
/// `hkCalcQuantizedSize` (`00e51da0`, cdecl): `(count, record)`.
const CALC_QUANTIZED_SIZE: u32 = 0x00e5_1da0;
/// `hkaTrackAnalysis::calcDynamicStats` (`00e4e610`, cdecl): `(track data, end, transform count,
/// float count, statistics)`.
const CALC_DYNAMIC_STATS: u32 = 0x00e4_e610;
/// `hkaTrackAnalysis::recompose` (`00e4f640`, cdecl): `(description, transforms out, floats out)`.
const RECOMPOSE: u32 = 0x00e4_f640;
/// `hkaAnimation::getFrameAndDelta` (`00e2f7c0`, `RET 0xc`): `this`, `(time, frame out, delta out)`.
const GET_FRAME_AND_DELTA: u32 = 0x00e2_f7c0;

const NUM_TRANSFORM_TRACKS: u32 = 0x10;
const NUM_FLOAT_TRACKS: u32 = 0x14;
const NUM_FRAMES: u32 = 0x28;
const BLOCK_SIZE: u32 = 0x2c;
const QUANTISATION_BYTE_0: u32 = 0x30;
const QUANTISATION_BYTE_1: u32 = 0x31;
const FLOATS_PER_FRAME: u32 = 0x34;
const FIRST_FLOAT_ARRAY: u32 = 0x38;
const SECOND_FLOAT_ARRAY: u32 = 0x3c;
const BYTE_ARRAY: u32 = 0x40;
const STATIC_DATA_SIZE: u32 = 0x44;
const TRACK_DESCRIPTION: u32 = 0x4c;
const SWAP_FLAG: u32 = 0x4c;
const STATIC_FLOATS: u32 = 0x54;
const RECOMPOSE_STATIC_RANGE: u32 = 0x5c;
const RECOMPOSE_DYNAMIC_RANGE: u32 = 0x60;
const BLOCK_BYTES: u32 = 0x64;
const LAST_BLOCK_BYTES: u32 = 0x68;
const DATA: u32 = 0x6c;

/// Slots of the animation's own vtable: the full cache key (`getFullCacheKey`, `00e34b40`) and
/// the frame count getter.
const SLOT_FULL_CACHE_KEY: u32 = 0x40;
const SLOT_NUM_FRAMES: u32 = 0x20;
/// Slots of the animation's `sampleTracks` target (`samplePartialTracks`).
const SLOT_SAMPLE_PARTIAL_TRACKS: u32 = 0x10;

/// Profiling labels, in the order they appear: the three begin labels of the cache, block
/// decompression and sampling paths, the pair that opens `samplePartialTracks`, and the others.
const LABEL_END: u32 = 0x010c_41f8;
const LABEL_DECOMPRESS_BLOCK_CACHE: u32 = 0x010e_b77c;
const LABEL_CACHE: u32 = 0x010e_b794;
const LABEL_RECOMPOSE_CHUNK: u32 = 0x010e_b7a0;
const LABEL_DECOMPRESS_CHUNK: u32 = 0x010e_b7b4;
const LABEL_STATIC: u32 = 0x010e_b7c8;
const LABEL_SAMPLE_CHUNK: u32 = 0x010e_b7d4;
const LABEL_RECOMPOSE: u32 = 0x010e_b7e8;
const LABEL_INTERPOLATE: u32 = 0x010e_b7f8;
const LABEL_DECOMPRESS: u32 = 0x010e_b804;
const LABEL_SAMPLE: u32 = 0x010e_b814;
const LABEL_SAMPLE_END: u32 = 0x0107_6f38;

/// The identity rotation (`010c71b0`, 16 bytes) and the `float` one (`01012068`).
const IDENTITY_ROTATION: u32 = 0x010c_71b0;
const ONE: u32 = 0x0101_2068;

/// The capacity word of an empty `hkArray` that owns nothing, and the mask of its capacity bits.
const EMPTY_CAPACITY: u32 = 0x8000_0000;
const CAPACITY_MASK: u32 = 0x3fff_ffff;

/// Appends a profiling record (`label`, time stamp) to the thread's record stream while its
/// cursor (`+4`) is below its limit (`+0xC`); a record is 12 bytes. Answers the time stamp that
/// was stored (`RDTSC` writes it to a stack slot), or `None` when the stream is full.
pub(crate) fn profile_record(e: &mut Engine, label: u32) -> Option<u32> {
    let slot: u32 = e.global(PROFILE_SLOT);
    let stream = e.call(TLS_GET_VALUE, &args![slot]).u32();
    let cursor = e.mem.u32(stream + 4);
    if cursor >= e.mem.u32(stream + 0xc) {
        return None;
    }
    let stamp = time_stamp_low();
    e.mem.set_u32(cursor, label);
    e.mem.set_u32(cursor + 4, stamp);
    e.mem.set_u32(stream + 4, cursor + 12);
    Some(stamp)
}

/// The pair of records the two sampling functions open with: `first` at the cursor and `second`
/// at `+0xC`, the time stamp in the first, the cursor moving on by 16 bytes.
fn profile_record_pair(e: &mut Engine, first: u32, second: u32) {
    let slot: u32 = e.global(PROFILE_SLOT);
    let stream = e.call(TLS_GET_VALUE, &args![slot]).u32();
    let cursor = e.mem.u32(stream + 4);
    if cursor >= e.mem.u32(stream + 0xc) {
        return;
    }
    e.mem.set_u32(cursor, first);
    e.mem.set_u32(cursor + 0xc, second);
    e.mem.set_u32(cursor + 4, time_stamp_low());
    e.mem.set_u32(stream + 4, cursor + 0x10);
}

/// Calls virtual slot `slot` of the object the cache holder points to (the holder is read again
/// for every call, as the code does).
fn cache_call(e: &mut Engine, holder: u32, slot: u32, args: &[u32]) -> Ret {
    let object = e.mem.u32(holder);
    e.vcall(object, slot, args)
}

/// The `hkLocalArray` set-up of `samplePartialTracks`: an empty array (data 0, size 0,
/// capacity `0x80000000`) that takes `count` floats from the stack router (rounded up to 128
/// bytes), grows through `_reserve` if the capacity does not cover `count`, and ends with
/// size `count`.
fn local_array_init(e: &mut Engine, array: u32, count: u32) {
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 4, 0);
    e.mem.set_u32(array + 8, EMPTY_CAPACITY);
    let router = e.call(STACK_ROUTER, &args![]).u32();
    let cursor = e.mem.u32(router);
    let block = count.wrapping_mul(4).wrapping_add(0x7f) & 0xffff_ff80;
    e.mem.set_u32(router, block.wrapping_add(cursor));
    e.mem.set_u32(array, cursor);
    e.mem.set_u32(array + 8, count | EMPTY_CAPACITY);
    e.mem.set_u32(array + 0xc, cursor);
    let router = e.call(STACK_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.u32(array + 8) & CAPACITY_MASK;
    if (capacity as i32) < (count as i32) {
        let doubled = capacity.wrapping_add(capacity);
        let wanted = if (count as i32) < (doubled as i32) {
            doubled
        } else {
            count
        };
        e.call(ARRAY_RESERVE, &args![allocator, array, wanted, 4u32]);
    }
    e.mem.set_u32(array + 4, count);
}

/// The end of the lifetime of a `samplePartialTracks` `hkLocalArray`: when its data is still its
/// local block the size is cleared, the router's cursor goes back to the local block, and a
/// heap-owned buffer (capacity flag clear) is freed through the allocator's slot `0x10`.
/// `reset_after` is the data / capacity reset the first array gets (the second is dead).
fn local_array_release(e: &mut Engine, array: u32, reset_after: bool) {
    let local = e.mem.u32(array + 0xc);
    if local == e.mem.u32(array) {
        e.mem.set_u32(array + 4, 0);
    }
    let router = e.call(STACK_ROUTER, &args![]).u32();
    e.mem.set_u32(router, local);
    let router = e.call(STACK_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.u32(array + 8);
    e.mem.set_u32(array + 4, 0);
    if (capacity as i32) >= 0 {
        let data = e.mem.u32(array);
        let bytes = (capacity & CAPACITY_MASK).wrapping_mul(4);
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
    if reset_after {
        e.mem.set_u32(array, 0);
        e.mem.set_u32(array + 8, EMPTY_CAPACITY);
    }
}

// Translated from 00e34990 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaDeltaCompressedAnimation::getNumOriginalFrames` (Xbox PDB): the frame count (`+0x28`).
pub fn get_num_original_frames(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + NUM_FRAMES)
}

// Translated from 00e349a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaDeltaCompressedAnimation::getNumDataChunks` (Xbox PDB): 4 chunks for the last frame of a
/// block (its interpolation partner is in the next block), otherwise 3. `RET 8`: the second stack
/// word is never read.
pub fn get_num_data_chunks(e: &mut Engine, this: Ptr, frame: u32, _unused_2: u32) -> u32 {
    let block_size = e.mem.u32(this.addr() + BLOCK_SIZE);
    let position = frame % block_size;
    if position + 1 != block_size {
        3
    } else {
        4
    }
}

// Translated from 00e349c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The block decompressor `_decompress` (overload 2, Xbox PDB map name). Register `EAX` is the
/// destination (`dest`); the eight stack words are a by-value record: `count` tracks, the float
/// `stride` between two tracks' values, the number of `frames` in the block, `flags` (its low
/// byte and the next one are the two bytes of the default record), the two float arrays `first`
/// and `second` (one entry per track), the optional byte array `widths` (the first byte of each
/// track's record; null uses the default) and the `data` the tracks are packed in.
///
/// For each track a 12-byte record `{byte, byte, float second[i], float first[i]}` goes through
/// `hkExpandReal` into the destination, `hkInverseDeltaTransform` is run over it, the
/// destination moves on by `stride` floats and the data offset by `hkCalcQuantizedSize`.
#[allow(clippy::too_many_arguments)]
pub fn decompress_ov2(
    e: &mut Engine,
    dest: Ptr,
    count: u32,
    stride: u32,
    frames: u32,
    flags: u32,
    first: Ptr,
    second: Ptr,
    widths: Ptr,
    data: Ptr,
) {
    let default_byte = (flags >> 8) as u8;
    let second_byte = flags as u8;
    let mut dest = dest.addr();
    let mut offset = 0u32;
    let mut track = 0u32;
    if count == 0 {
        return;
    }
    loop {
        let from_first = x87_quiet_bits(e.mem.u32(first.addr() + 4 * track));
        let from_second = x87_quiet_bits(e.mem.u32(second.addr() + 4 * track));
        let first_byte = if widths.is_null() {
            default_byte
        } else {
            e.mem.u8(widths.addr() + track)
        };
        // The exe's frame (`SUB ESP,0xc`, four pushes): 0x1c bytes, the quantization record at
        // +0x10 (00e349d5, 00e34a15).
        let size = e.with_stack(0x1c, |e, frame| {
            let record = frame.addr() + 0x10;
            e.mem.set_u8(record, first_byte);
            e.mem.set_u8(record + 1, second_byte);
            e.mem.set_u32(record + 4, from_second);
            e.mem.set_u32(record + 8, from_first);
            e.call(
                EXPAND_REAL,
                &args![data.addr().wrapping_add(offset), record, dest, frames],
            );
            e.call(INVERSE_DELTA_TRANSFORM, &args![dest, frames]);
            dest = dest.wrapping_add(stride.wrapping_mul(4));
            e.call(CALC_QUANTIZED_SIZE, &args![frames, record]).u32()
        });
        track += 1;
        offset = offset.wrapping_add(size);
        if track >= count {
            break;
        }
    }
}

// Translated from 00e34a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaDeltaCompressedAnimation::sampleTracks` (Xbox PDB): calls the animation's own slot `0x10`
/// (`samplePartialTracks`) with the time (through `FLD`/`FSTP`), the transform track count (`+0x10`),
/// the transforms, the float track count (`+0x14`), the floats and the cache.
pub fn sample_tracks(
    e: &mut Engine,
    this: Ptr,
    time: f32,
    transforms: u32,
    floats: u32,
    cache: u32,
) {
    let transform_tracks = e.mem.u32(this.addr() + NUM_TRANSFORM_TRACKS);
    let float_tracks = e.mem.u32(this.addr() + NUM_FLOAT_TRACKS);
    e.vcall(
        this.addr(),
        SLOT_SAMPLE_PARTIAL_TRACKS,
        &args![
            f32::from_bits(x87_quiet_bits(time.to_bits())),
            transform_tracks,
            transforms,
            float_tracks,
            floats,
            cache
        ],
    );
}

// Translated from 00e34a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaDeltaCompressedAnimation::sampleIndividualFloatTracks` (Xbox PDB): zeroes `count` words at
/// `dest`; the other stack words and `this` are not read.
pub fn sample_individual_float_tracks(
    e: &mut Engine,
    _this: Ptr,
    _unused_1: u32,
    _unused_2: u32,
    count: u32,
    dest: Ptr,
) {
    for index in 0..count {
        e.mem.set_u32(dest.addr() + 4 * index, 0);
    }
}

// Translated from 00e34ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Hands a decompressed block back to the cache. Registers first: `EAX` the animation (`anim`),
/// `ECX` the frame (`frame`), `EDI` the address of the cache holder (`holder`); then the stack
/// word `floats` (the floats per frame). It asks the animation for the full cache key of
/// `frame` (slot `0x40`), takes the cache object's slot `0x2c`, calls its slot `0x1c` with the
/// address of a stack handle (the handle lives in the caller's argument words; the first holds
/// `floats` on entry), the key and `floats * block size * 4`, and ends by tail-calling slot `0x30`,
/// whose answer is returned. The class of the cache object is unconfirmed.
pub fn fn_00e34ab0(e: &mut Engine, anim: Ptr, frame: u32, holder: u32, floats: u32) -> u32 {
    let key = e
        .vcall(anim.addr(), SLOT_FULL_CACHE_KEY, &args![frame])
        .u32();
    let size = e
        .mem
        .u32(anim.addr() + BLOCK_SIZE)
        .wrapping_mul(floats)
        .wrapping_mul(4);
    cache_call(e, holder, 0x2c, &args![]);
    e.with_stack(0x20, |e, handle| {
        e.mem.set_u32(handle.addr(), floats);
        cache_call(e, holder, 0x1c, &args![handle.addr(), key, size]);
    });
    cache_call(e, holder, 0x30, &args![]).u32()
}

// Translated from 00e34b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaDeltaCompressedAnimation::getFullCacheKey` (Xbox PDB): `frame / block size + [+0x6c]`.
pub fn get_full_cache_key(e: &mut Engine, this: Ptr, frame: u32) -> u32 {
    let block_size = e.mem.u32(this.addr() + BLOCK_SIZE);
    (frame / block_size).wrapping_add(e.mem.u32(this.addr() + DATA))
}

// Translated from 00e34b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaDeltaCompressedAnimation::clearAllCacheKeys` (Xbox PDB): for every block (the frame count,
/// from the animation's slot `0x20`, minus one over the block size, plus one) calls the cache
/// object's slot `0x14` with the address of a scratch byte, the block's key (`[+0x6c]` plus the
/// block number) and `floats per frame * block size * 4`.
pub fn clear_all_cache_keys(e: &mut Engine, this: Ptr, cache: Ptr) {
    let block_size = e.mem.u32(this.addr() + BLOCK_SIZE);
    let frames = e.vcall(this.addr(), SLOT_NUM_FRAMES, &args![]).u32();
    let size = e
        .mem
        .u32(this.addr() + FLOATS_PER_FRAME)
        .wrapping_mul(block_size)
        .wrapping_mul(4);
    let blocks = ((frames.wrapping_sub(1) as i32) / (block_size as i32)).wrapping_add(1);
    let mut block = 0i32;
    // The scratch byte is the top byte of a saved register slot on the stack.
    e.with_stack(8, |e, scratch| {
        while block < blocks {
            let key = e.mem.u32(this.addr() + DATA).wrapping_add(block as u32);
            e.vcall(cache.addr(), 0x14, &args![scratch.addr() + 3, key, size]);
            block += 1;
        }
    });
}

// Translated from 00e34bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Describes one block's data chunk: `out` gets the 16-byte aligned start (`block bytes * block +
/// [+0x6c] + [+0x44]`, low nibble cleared) at `+0`, the aligned size at `+4` (the last block uses
/// its own byte count) and the low nibble of the start at `+8` (a byte).
pub fn fn_00e34bb0(e: &mut Engine, this: Ptr, block: u32, block_count: u32, out: Ptr) {
    let a = this.addr();
    let start = e
        .mem
        .u32(a + BLOCK_BYTES)
        .wrapping_mul(block)
        .wrapping_add(e.mem.u32(a + DATA))
        .wrapping_add(e.mem.u32(a + STATIC_DATA_SIZE));
    let low = (start & 0xf) as u8;
    e.mem.set_u32(out.addr(), start & 0xffff_fff0);
    e.mem.set_u8(out.addr() + 8, low);
    let bytes = if block == block_count.wrapping_sub(1) {
        e.mem.u32(a + LAST_BLOCK_BYTES)
    } else {
        e.mem.u32(a + BLOCK_BYTES)
    };
    e.mem
        .set_u32(out.addr() + 4, (u32::from(low) + bytes + 0xf) & 0xffff_fff0);
}

// Translated from 00e34c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaDeltaCompressedAnimation::getDataChunks` (Xbox PDB): fills the 12-byte chunk records at
/// `out` (pointer, size, misalignment byte): the object itself (0x80 bytes), the static data
/// (`[+0x6c]`, size `[+0x44]` rounded up to 16) and the block holding `frame` (and, with `count`
/// 4, the following block). The second stack word is never read.
pub fn get_data_chunks(
    e: &mut Engine,
    this: Ptr,
    frame: u32,
    _unused_2: u32,
    out: Ptr,
    count: u32,
) {
    let a = this.addr();
    let o = out.addr();
    e.mem.set_u8(o + 8, (a & 0xf) as u8);
    e.mem.set_u32(o, a);
    e.mem.set_u32(o + 4, 0x80);
    let data = e.mem.u32(a + DATA);
    e.mem.set_u32(o + 0xc, data);
    let static_size = e.mem.u32(a + STATIC_DATA_SIZE).wrapping_add(0xf) & 0xffff_fff0;
    e.mem.set_u8(o + 0x14, (data & 0xf) as u8);
    e.mem.set_u32(o + 0x10, static_size);
    let block_size = e.mem.u32(a + BLOCK_SIZE);
    let block = frame / block_size;
    let frames = e.mem.u32(a + NUM_FRAMES);
    let blocks = (frames.wrapping_add(block_size).wrapping_sub(1) as i32) / (block_size as i32);
    let blocks = blocks as u32;
    fn_00e34bb0(e, this, block, blocks, Ptr::new(o + 0x18));
    if count == 4 {
        fn_00e34bb0(
            e,
            this,
            block.wrapping_add(1) % blocks,
            blocks,
            Ptr::new(o + 0x24),
        );
    }
}

// Translated from 00e34c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaDeltaCompressedAnimation::getMaxSizeOfCombinedDataChunks` (Xbox PDB): the largest, over the
/// blocks, of `0x80 + the static data rounded up to 16 + the block's chunk size (+ the next block's
/// when there is one)`. Zero when there are no blocks.
pub fn get_max_size_of_combined_data_chunks(e: &mut Engine, this: Ptr) -> i32 {
    let a = this.addr();
    let frames = e.mem.u32(a + NUM_FRAMES);
    let block_size = e.mem.u32(a + BLOCK_SIZE);
    let blocks = (frames.wrapping_add(block_size).wrapping_sub(1) as i32) / (block_size as i32);
    let blocks = blocks as u32;
    let mut maximum = 0i32;
    if blocks == 0 {
        return 0;
    }
    let static_size = e.mem.u32(a + STATIC_DATA_SIZE).wrapping_add(0xf) & 0xffff_fff0;
    // The exe's frame (`SUB ESP,0x14`, four pushes): 0x24 bytes, the chunk record at +0x18
    // (00e34cc9..00e34cd1).
    e.with_stack(0x24, |e, frame| {
        let chunk = frame.addr() + 0x18;
        let mut block = 0u32;
        loop {
            e.mem.set_u32(chunk, 0);
            e.mem.set_u32(chunk + 4, 0);
            e.mem.set_u8(chunk + 8, 0);
            fn_00e34bb0(e, this, block, blocks, Ptr::new(chunk));
            let mut total = e
                .mem
                .u32(chunk + 4)
                .wrapping_add(static_size)
                .wrapping_add(0x80);
            if block < blocks.wrapping_sub(1) {
                fn_00e34bb0(e, this, block + 1, blocks, Ptr::new(chunk));
                total = total.wrapping_add(e.mem.u32(chunk + 4));
            }
            if total as i32 > maximum {
                maximum = total as i32;
            }
            block += 1;
            if block >= blocks {
                break;
            }
        }
    });
    maximum
}

// Translated from 00e34d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaDeltaCompressedAnimation::sampleIndividualTransformTracks` (Xbox PDB): writes `count`
/// reference transforms of 48 bytes at `dest`: translation zero, the identity rotation (16 bytes
/// from `010c71b0`) and scale 1.0 in all four lanes (from `01012068`). `this` and the other
/// stack words are not read.
pub fn sample_individual_transform_tracks(
    e: &mut Engine,
    _this: Ptr,
    _unused_1: u32,
    _unused_2: u32,
    count: u32,
    dest: Ptr,
) {
    let one = e.mem.u32(ONE);
    for track in 0..count {
        let at = dest.addr().wrapping_add(track.wrapping_mul(0x30));
        for lane in 0..4 {
            e.mem.set_u32(at + 4 * lane, 0);
            let rotation = e.mem.u32(IDENTITY_ROTATION + 4 * lane);
            e.mem.set_u32(at + 0x10 + 4 * lane, rotation);
            e.mem.set_u32(at + 0x20 + 4 * lane, one);
        }
    }
}

// Translated from 00e34d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `decompressBlockToCache` (overload 2, Xbox PDB map name). Register `EAX` is the animation
/// (`anim`); stack words `frame`, the destination `dest` and `count`, the track values per frame.
/// Decompresses the block holding `frame` (the frames of the block clipped to the animation's
/// frame count) with `_decompress`, between two profiling records.
pub fn decompress_block_to_cache_ov2(e: &mut Engine, anim: Ptr, frame: u32, dest: Ptr, count: u32) {
    let a = anim.addr();
    profile_record(e, LABEL_DECOMPRESS_BLOCK_CACHE);
    let block_size = e.mem.u32(a + BLOCK_SIZE);
    let num_frames = e.mem.u32(a + NUM_FRAMES);
    let mut block = frame / block_size;
    let mut in_block = block_size;
    if (block_size as i32) > (num_frames as i32) {
        in_block = num_frames;
        block = 0;
    }
    let first_frame = block_size.wrapping_mul(block);
    if (first_frame.wrapping_add(in_block) as i32) > (num_frames as i32) {
        in_block = num_frames.wrapping_sub(first_frame);
    }
    let data = e.mem.u32(a + DATA);
    // Only the low two bytes of this word are written; the other two are never read.
    let flags = u32::from(e.mem.u8(a + QUANTISATION_BYTE_0)) << 8
        | u32::from(e.mem.u8(a + QUANTISATION_BYTE_1));
    let first = e.mem.u32(a + FIRST_FLOAT_ARRAY).wrapping_add(data);
    let second = e.mem.u32(a + SECOND_FLOAT_ARRAY).wrapping_add(data);
    let widths = e.mem.u32(a + BYTE_ARRAY).wrapping_add(data);
    let block_data = e
        .mem
        .u32(a + BLOCK_BYTES)
        .wrapping_mul(block)
        .wrapping_add(e.mem.u32(a + STATIC_DATA_SIZE))
        .wrapping_add(data);
    decompress_ov2(
        e,
        dest,
        count,
        block_size,
        in_block,
        flags,
        Ptr::new(first),
        Ptr::new(second),
        Ptr::new(widths),
        Ptr::new(block_data),
    );
    profile_record(e, LABEL_END);
}

// Translated from 00e34e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `retrieveFromCache` (overload 2, Xbox PDB map name). Register `EAX` is the cache holder
/// (`holder`, the address of a word holding the cache object); stack words `anim`, `frame` and
/// `count` (floats per frame). Under a profiling record it locks the cache object (slot `0x2c`),
/// looks the block's key up (slot `0x28`, answering a pointer to a byte; a nonzero byte means the
/// entry is pending and the answer is 0) and otherwise either takes an existing entry (slot
/// `0xc`, handing back the cached data with slot `0x18`) or allocates one (slot `0x10`) and
/// fills it with `decompressBlockToCache` before publishing it (slots `0x18`, `0x20`, `0x2c`,
/// `0x24`). Slot `0x30` unlocks. Answers the data pointer or 0. The cache object's handle
/// lives in the caller's argument words; those words are modelled as a block here and the
/// class of the cache object is unconfirmed.
pub fn retrieve_from_cache_ov2(
    e: &mut Engine,
    holder: u32,
    anim: Ptr,
    frame: u32,
    count: u32,
) -> u32 {
    e.with_stack(0x20, |e, slot| {
        let slot = slot.addr();
        e.mem.set_u32(slot, anim.addr());
        e.mem.set_u32(slot + 4, frame);
        e.mem.set_u32(slot + 8, count);
        if let Some(stamp) = profile_record(e, LABEL_CACHE) {
            e.mem.set_u32(slot, stamp);
        }
        let key = e
            .vcall(anim.addr(), SLOT_FULL_CACHE_KEY, &args![frame])
            .u32();
        let size = e
            .mem
            .u32(anim.addr() + BLOCK_SIZE)
            .wrapping_mul(count)
            .wrapping_mul(4);
        cache_call(e, holder, 0x2c, &args![]);
        let pending = cache_call(e, holder, 0x28, &args![slot, key, size]).u32();
        if e.mem.u8(pending) != 0 {
            cache_call(e, holder, 0x30, &args![]);
            profile_record(e, LABEL_END);
            return 0;
        }
        let existing = cache_call(e, holder, 0xc, &args![key, size]).u32();
        if existing != 0 {
            cache_call(e, holder, 0x18, &args![slot + 8, key, size]);
            cache_call(e, holder, 0x30, &args![]);
            profile_record(e, LABEL_END);
            return existing;
        }
        let fresh = cache_call(e, holder, 0x10, &args![key, size]).u32();
        if fresh == 0 {
            cache_call(e, holder, 0x30, &args![]);
            profile_record(e, LABEL_END);
            return 0;
        }
        cache_call(e, holder, 0x18, &args![slot, key, size]);
        cache_call(e, holder, 0x20, &args![slot, key, size]);
        cache_call(e, holder, 0x30, &args![]);
        let block_frame = e.mem.u32(slot + 4);
        let block_count = e.mem.u32(slot + 8);
        decompress_block_to_cache_ov2(e, anim, block_frame, Ptr::new(fresh), block_count);
        cache_call(e, holder, 0x2c, &args![]);
        cache_call(e, holder, 0x24, &args![slot + 8, key, size]);
        cache_call(e, holder, 0x30, &args![]);
        profile_record(e, LABEL_END);
        fresh
    })
}

/// Swaps the byte order of a 32-bit word that goes through `FLD`/`FSTP` before and after
/// (`FLD float [src]; FSTP float [tmp]`, four byte moves, `FLD float [tmp]; FSTP float [dst]`).
fn swap_float_word(e: &mut Engine, address: u32) {
    let word = x87_quiet_bits(e.mem.u32(address));
    e.mem.set_u32(address, x87_quiet_bits(word.swap_bytes()));
}

// Translated from 00e35050 (decompiled, FalloutNV.exe 1.4.0.525)
/// Byte-swaps the animation's data in place (the Xbox 360 stores it big-endian). `this` is in
/// `ECX`. Only when `[+0x4c]` is 4 and the first byte of the data is nonzero: the `u16` values
/// that follow the 4-byte header (one per track), then, once `00e4ebb0` has measured the track
/// description, `FLD`/`FSTP` swaps of the dwords of the static floats (`[+0x54]`) and of the two
/// float arrays (`[+0x38]`, `[+0x3c]`) with the counts it returned.
pub fn fn_00e35050(e: &mut Engine, this: Ptr) {
    let a = this.addr();
    if e.mem.u32(a + SWAP_FLAG) != 4 {
        return;
    }
    let data = e.mem.u32(a + DATA);
    if e.mem.u8(data) == 0 {
        return;
    }
    let tracks = e.mem.u32(a + NUM_TRANSFORM_TRACKS);
    let floats = e.mem.u32(a + NUM_FLOAT_TRACKS);
    let values = data.wrapping_add(4);
    let mut index = 0i32;
    if (tracks.wrapping_add(floats) as i32) > 0 {
        loop {
            let at = values.wrapping_add((index as u32).wrapping_mul(2));
            let word = e.mem.u16(at);
            e.mem.set_u16(at, word.swap_bytes());
            let total = e
                .mem
                .u32(a + NUM_TRANSFORM_TRACKS)
                .wrapping_add(e.mem.u32(a + NUM_FLOAT_TRACKS));
            index += 1;
            if index >= total as i32 {
                break;
            }
        }
    }
    // The exe's frame (`SUB ESP,0x48`, four pushes): 0x58 bytes, the dynamic statistics at +0x20
    // (00e350c9..00e35105).
    e.with_stack(0x58, |e, frame| {
        let stats = frame.addr() + 0x20;
        e.call(
            CALC_DYNAMIC_STATS,
            &args![
                values,
                values.wrapping_add(tracks.wrapping_mul(2)),
                tracks,
                floats,
                stats
            ],
        );
        let stat = |e: &Engine, index: u32| e.mem.u32(stats + 4 * index);
        let static_total = stat(e, 2)
            .wrapping_add(stat(e, 3))
            .wrapping_add(stat(e, 1))
            .wrapping_add(stat(e, 0));
        let mut at = e.mem.u32(a + STATIC_FLOATS).wrapping_add(data);
        let mut done = 0u32;
        if (static_total as i32) >= 4 {
            let groups = (static_total.wrapping_sub(4) >> 2) + 1;
            done = groups * 4;
            for _ in 0..groups {
                for lane in 0..4 {
                    swap_float_word(e, at.wrapping_add(4 * lane));
                }
                at = at.wrapping_add(0x10);
            }
        }
        if (done as i32) < (static_total as i32) {
            for _ in 0..static_total.wrapping_sub(done) {
                swap_float_word(e, at);
                at = at.wrapping_add(4);
            }
        }
        let array_total = stat(e, 10)
            .wrapping_add(stat(e, 9))
            .wrapping_add(stat(e, 8))
            .wrapping_add(stat(e, 7));
        let mut first = e.mem.u32(a + FIRST_FLOAT_ARRAY).wrapping_add(data);
        let mut second = e.mem.u32(a + SECOND_FLOAT_ARRAY).wrapping_add(data);
        let mut done = 0u32;
        if (array_total as i32) >= 4 {
            let groups = (array_total.wrapping_sub(4) >> 2) + 1;
            done = groups * 4;
            for _ in 0..groups {
                for lane in 0..4 {
                    swap_float_word(e, first.wrapping_add(4 * lane));
                    swap_float_word(e, second.wrapping_add(4 * lane));
                }
                first = first.wrapping_add(0x10);
                second = second.wrapping_add(0x10);
            }
        }
        if done < array_total {
            for _ in 0..array_total - done {
                swap_float_word(e, first);
                swap_float_word(e, second);
                first = first.wrapping_add(4);
                second = second.wrapping_add(4);
            }
        }
    });
}

/// One interpolated value: `second * second_weight + first * first_weight`, computed in double
/// precision (the `FMUL`/`FMUL`/`FADDP` sequence) and rounded to `float` at the store.
fn blend(first: f32, second: f32, first_weight: f32, second_weight: f32) -> f32 {
    (f64::from(second) * f64::from(second_weight) + f64::from(first) * f64::from(first_weight))
        as f32
}

/// The track statistics both sampling functions take when the caller's track counts differ from
/// the animation's: `00e4e610` fills a 14-word record (zeroed first) over the track description
/// and the number of dynamic floats is the sum of its words 7 to 9, plus word 10 when the caller
/// asked for float tracks. Without float tracks the caller's transform count is passed, with them
/// the animation's own count and the caller's float count.
fn recount_floats(
    e: &mut Engine,
    track_data: u32,
    own_transforms: u32,
    transforms: u32,
    floats: u32,
) -> u32 {
    let stats = e.mem.alloc(0x38);
    let end = track_data.wrapping_add(own_transforms.wrapping_mul(2));
    let total = if floats == 0 {
        e.call(
            CALC_DYNAMIC_STATS,
            &args![track_data, end, transforms, 0u32, stats],
        );
        e.mem
            .u32(stats + 0x24)
            .wrapping_add(e.mem.u32(stats + 0x20))
            .wrapping_add(e.mem.u32(stats + 0x1c))
    } else {
        e.call(
            CALC_DYNAMIC_STATS,
            &args![track_data, end, own_transforms, floats, stats],
        );
        e.mem
            .u32(stats + 0x28)
            .wrapping_add(e.mem.u32(stats + 0x24))
            .wrapping_add(e.mem.u32(stats + 0x20))
            .wrapping_add(e.mem.u32(stats + 0x1c))
    };
    e.mem.free(stats);
    total
}

/// The track ranges `recompose` takes (by address), in the eight words the code builds: the two
/// caller counts, the track description and its end, the static floats and their second range,
/// the dynamic floats and their second range.
struct RecomposeInput {
    transforms: u32,
    floats: u32,
    track_data: u32,
    own_transforms: u32,
    static_floats: u32,
    static_range: u32,
    dynamic: u32,
    dynamic_range: u32,
}

/// Writes `input` as the eight-word description `recompose` is given a pointer to.
fn recompose_description(e: &mut Engine, input: &RecomposeInput) -> u32 {
    let description = e.mem.alloc(0x20);
    let words = [
        input.transforms,
        input.floats,
        input.track_data,
        input
            .track_data
            .wrapping_add(input.own_transforms.wrapping_mul(2)),
        input.static_floats,
        input
            .static_floats
            .wrapping_add(input.static_range.wrapping_mul(4)),
        input.dynamic,
        input
            .dynamic
            .wrapping_add(input.dynamic_range.wrapping_mul(4)),
    ];
    for (index, word) in words.into_iter().enumerate() {
        e.mem.set_u32(description + 4 * index as u32, word);
    }
    description
}

// Translated from 00e354b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaDeltaCompressedAnimation::samplePartialWithDataChunks` (Xbox PDB), a cdecl function: the
/// `frame` and the fractional `delta` toward the next one, the track counts and outputs, the
/// chunk records `chunks` that `getDataChunks` filled (chunk 0 is the animation, chunk 1 the
/// static data, chunk 2 the block of `frame`, chunk 3 the next block) and `num_chunks` (3, or 4
/// when the next frame is in the next block).
///
/// It takes the animation from chunk 0 (pointer plus misalignment byte), recounts the dynamic
/// floats when the caller's counts differ, takes two scratch buffers from the stack router,
/// decompresses chunk 2 and blends the frame with the next one (inside the block with 3 chunks;
/// with 4 chunks the frame weighted by `1 - delta` and then the first frame of chunk 3 weighted by
/// `delta` added on) and recomposes the tracks.
#[allow(clippy::too_many_arguments)]
pub fn sample_partial_with_data_chunks(
    e: &mut Engine,
    frame: u32,
    delta: f32,
    num_transforms: u32,
    transforms_out: u32,
    num_floats: u32,
    floats_out: u32,
    chunks: Ptr,
    num_chunks: u32,
) {
    let c = chunks.addr();
    let anim = e.mem.u32(c).wrapping_add(u32::from(e.mem.u8(c + 8)));
    profile_record_pair(e, LABEL_SAMPLE_CHUNK, LABEL_STATIC);
    let base = u32::from(e.mem.u8(c + 0x14)).wrapping_add(e.mem.u32(c + 0xc));
    let track_data = e.mem.u32(anim + TRACK_DESCRIPTION).wrapping_add(base);
    let static_floats = e.mem.u32(anim + STATIC_FLOATS).wrapping_add(base);
    let mut float_count = e.mem.u32(anim + FLOATS_PER_FRAME);
    let own_transforms = e.mem.u32(anim + NUM_TRANSFORM_TRACKS);
    if num_transforms != own_transforms || num_floats != e.mem.u32(anim + NUM_FLOAT_TRACKS) {
        float_count = recount_floats(e, track_data, own_transforms, num_transforms, num_floats);
    }
    profile_record(e, LABEL_DECOMPRESS_CHUNK);
    let block_size = e.mem.u32(anim + BLOCK_SIZE);
    let omt = (1.0 - f64::from(delta)) as f32;
    let block_floats = block_size.wrapping_mul(float_count);
    let router = e.call(STACK_ROUTER, &args![]).u32();
    let block_buffer = e.mem.u32(router);
    let rounded = block_floats.wrapping_mul(4).wrapping_add(0x7f) & 0xffff_ff80;
    e.mem.set_u32(router, rounded.wrapping_add(block_buffer));
    let router = e.call(STACK_ROUTER, &args![]).u32();
    let out = e.mem.u32(router);
    let rounded = float_count.wrapping_mul(4).wrapping_add(0x7f) & 0xffff_ff80;
    e.mem.set_u32(router, rounded.wrapping_add(out));
    let num_frames = e.mem.u32(anim + NUM_FRAMES);
    let mut in_block = block_size;
    if (block_size as i32) > (num_frames as i32) {
        in_block = num_frames;
    } else {
        let block = frame / block_size;
        if (block.wrapping_add(1).wrapping_mul(block_size) as i32) > (num_frames as i32) {
            in_block = num_frames.wrapping_sub(block.wrapping_mul(block_size));
        }
    }
    // Only the low two bytes of the fourth word are written; the other two are never read.
    let flags = u32::from(e.mem.u8(anim + QUANTISATION_BYTE_0)) << 8
        | u32::from(e.mem.u8(anim + QUANTISATION_BYTE_1));
    let first = e.mem.u32(anim + FIRST_FLOAT_ARRAY).wrapping_add(base);
    let second = e.mem.u32(anim + SECOND_FLOAT_ARRAY).wrapping_add(base);
    let widths = e.mem.u32(anim + BYTE_ARRAY).wrapping_add(base);
    // The data pointer of chunk `at` (a chunk record is a pointer, a size and a misalignment byte).
    let chunk_data =
        |e: &Engine, at: u32| u32::from(e.mem.u8(c + at + 8)).wrapping_add(e.mem.u32(c + at));
    let own_block = chunk_data(e, 0x18);
    decompress_ov2(
        e,
        Ptr::new(block_buffer),
        float_count,
        block_size,
        in_block,
        flags,
        Ptr::new(first),
        Ptr::new(second),
        Ptr::new(widths),
        Ptr::new(own_block),
    );
    let stride = e.mem.u32(anim + BLOCK_SIZE);
    let position = frame % stride;
    if num_chunks == 3 {
        let mut index = position;
        for value in 0..float_count {
            let at = block_buffer.wrapping_add(index.wrapping_mul(4));
            let result = blend(e.mem.f32(at), e.mem.f32(at.wrapping_add(4)), omt, delta);
            e.mem.set_f32(out.wrapping_add(4 * value), result);
            index = index.wrapping_add(stride);
        }
    } else {
        let mut index = position;
        for value in 0..float_count {
            let at = block_buffer.wrapping_add(index.wrapping_mul(4));
            let scaled = (f64::from(e.mem.f32(at)) * f64::from(omt)) as f32;
            e.mem.set_f32(out.wrapping_add(4 * value), scaled);
            index = index.wrapping_add(stride);
        }
        let next_block = chunk_data(e, 0x24);
        let next = frame.wrapping_add(1) / stride;
        let own_frames = e.mem.u32(anim + NUM_FRAMES);
        let end = next
            .wrapping_add(1)
            .wrapping_mul(e.mem.u32(anim + BLOCK_SIZE));
        if (end as i32) > (own_frames as i32) {
            in_block = own_frames.wrapping_sub(next.wrapping_mul(stride));
        }
        decompress_ov2(
            e,
            Ptr::new(block_buffer),
            float_count,
            block_size,
            in_block,
            flags,
            Ptr::new(first),
            Ptr::new(second),
            Ptr::new(widths),
            Ptr::new(next_block),
        );
        let step = e.mem.u32(anim + BLOCK_SIZE);
        let mut index = 0u32;
        for value in 0..float_count {
            let at = block_buffer.wrapping_add(index.wrapping_mul(4));
            let target = out.wrapping_add(4 * value);
            let added = f64::from(e.mem.f32(at)) * f64::from(delta) + f64::from(e.mem.f32(target));
            e.mem.set_f32(target, added as f32);
            index = index.wrapping_add(step);
        }
    }
    profile_record(e, LABEL_RECOMPOSE_CHUNK);
    let input = RecomposeInput {
        transforms: num_transforms,
        floats: num_floats,
        track_data,
        own_transforms: e.mem.u32(anim + NUM_TRANSFORM_TRACKS),
        static_floats,
        static_range: e.mem.u32(anim + RECOMPOSE_STATIC_RANGE),
        dynamic: out,
        dynamic_range: e.mem.u32(anim + RECOMPOSE_DYNAMIC_RANGE),
    };
    let description = recompose_description(e, &input);
    e.call(RECOMPOSE, &args![description, transforms_out, floats_out]);
    e.mem.free(description);
    let router = e.call(STACK_ROUTER, &args![]).u32();
    e.mem.set_u32(router, out);
    let router = e.call(STACK_ROUTER, &args![]).u32();
    e.mem.set_u32(router, block_buffer);
    profile_record(e, LABEL_SAMPLE_END);
}

/// Finds the block data of `frame` for `samplePartialTracks`: the cache's data when the cache
/// (the word at `holder`) is present and has it, otherwise the block decompressed into the
/// scratch buffer `buffer`. `floats` is the animation's own floats per frame, `count` the
/// recounted one the block is decompressed for.
fn fetch_block(
    e: &mut Engine,
    anim: Ptr,
    holder: u32,
    frame: u32,
    floats: u32,
    count: u32,
    buffer: u32,
) -> u32 {
    if e.mem.u32(holder) != 0 {
        let cached = retrieve_from_cache_ov2(e, holder, anim, frame, floats);
        if cached != 0 {
            return cached;
        }
    }
    decompress_block_to_cache_ov2(e, anim, frame, Ptr::new(buffer), count);
    buffer
}

// Translated from 00e359f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaDeltaCompressedAnimation::samplePartialTracks` (Xbox PDB), `RET 0x18`: samples the
/// animation at `time` into `transforms_out` (`num_transforms` tracks) and `floats_out`
/// (`num_floats` tracks); `cache` is the optional cache object.
///
/// It recounts the dynamic floats when the track counts differ from the animation's, splits the
/// time into a frame and a fraction with `getFrameAndDelta` (the frame is written over the
/// time's argument word), takes the two `hkLocalArray`s (the output floats and one decompressed
/// block, `block size * floats` entries) from the stack router, and gets the block of the frame
/// from the cache or decompresses it. The output is the frame's values blended with the next
/// frame's; on the last frame of a block the frame's values are scaled by `1 - delta` and the
/// next block (from the cache, or decompressed over the same scratch) is added scaled by
/// `delta`. A cached block is handed back with `00e34ab0`. The tracks are then recomposed and
/// the arrays given back to the router (and the allocator if they grew).
///
/// The argument words are a real block here (the cache holder passed on is the address of the
/// `cache` word, and `getFrameAndDelta` writes the frame over the time's word).
#[allow(clippy::too_many_arguments)]
pub fn sample_partial_tracks(
    e: &mut Engine,
    this: Ptr,
    time: f32,
    num_transforms: u32,
    transforms_out: u32,
    num_floats: u32,
    floats_out: u32,
    cache: u32,
) {
    let a = this.addr();
    profile_record_pair(e, LABEL_SAMPLE, LABEL_STATIC);
    let track_data = e
        .mem
        .u32(a + TRACK_DESCRIPTION)
        .wrapping_add(e.mem.u32(a + DATA));
    let own_transforms = e.mem.u32(a + NUM_TRANSFORM_TRACKS);
    let own_floats = e.mem.u32(a + FLOATS_PER_FRAME);
    let mut float_count = own_floats;
    if num_transforms != own_transforms || num_floats != e.mem.u32(a + NUM_FLOAT_TRACKS) {
        float_count = recount_floats(e, track_data, own_transforms, num_transforms, num_floats);
    }
    profile_record(e, LABEL_DECOMPRESS);
    // The time's word (later the frame), the delta, and the cache word.
    let words = e.mem.alloc(0x20);
    let holder = words + 0x14;
    let delta_word = words + 0x18;
    let time_bits = x87_quiet_bits(time.to_bits());
    e.mem.set_u32(words, time_bits);
    e.mem.set_u32(holder, cache);
    e.call(
        GET_FRAME_AND_DELTA,
        &args![this, time_bits, words, delta_word],
    );
    let delta = e.mem.f32(delta_word);
    let omt = (1.0 - f64::from(delta)) as f32;
    let arrays = e.mem.alloc(0x20);
    let out_array = arrays;
    let block_array = arrays + 0x10;
    local_array_init(e, out_array, float_count);
    let block_floats = e.mem.u32(a + BLOCK_SIZE).wrapping_mul(float_count);
    local_array_init(e, block_array, block_floats);
    let frame = e.mem.u32(words);
    let buffer = e.mem.u32(block_array);
    let block = fetch_block(e, this, holder, frame, own_floats, float_count, buffer);
    profile_record(e, LABEL_INTERPOLATE);
    let out = e.mem.u32(out_array);
    let block_size = e.mem.u32(a + BLOCK_SIZE);
    let position = frame % block_size;
    if position + 1 != block_size {
        let mut index = position;
        for value in 0..float_count {
            let at = block.wrapping_add(index.wrapping_mul(4));
            let result = blend(e.mem.f32(at), e.mem.f32(at.wrapping_add(4)), omt, delta);
            e.mem.set_f32(out.wrapping_add(4 * value), result);
            index = index.wrapping_add(e.mem.u32(a + BLOCK_SIZE));
        }
        if block != e.mem.u32(block_array) {
            fn_00e34ab0(e, this, frame, holder, own_floats);
        }
    } else {
        let mut index = position;
        for value in 0..float_count {
            let at = block.wrapping_add(index.wrapping_mul(4));
            let scaled = (f64::from(e.mem.f32(at)) * f64::from(omt)) as f32;
            e.mem.set_f32(out.wrapping_add(4 * value), scaled);
            index = index.wrapping_add(e.mem.u32(a + BLOCK_SIZE));
        }
        if block != e.mem.u32(block_array) {
            fn_00e34ab0(e, this, frame, holder, own_floats);
        }
        profile_record(e, LABEL_DECOMPRESS);
        let next = frame.wrapping_add(1);
        e.mem.set_u32(words, next);
        let buffer = e.mem.u32(block_array);
        let following = fetch_block(e, this, holder, next, own_floats, float_count, buffer);
        profile_record(e, LABEL_INTERPOLATE);
        let mut index = 0u32;
        for value in 0..float_count {
            let at = following.wrapping_add(index.wrapping_mul(4));
            let target = out.wrapping_add(4 * value);
            let added = f64::from(e.mem.f32(at)) * f64::from(delta) + f64::from(e.mem.f32(target));
            e.mem.set_f32(target, added as f32);
            index = index.wrapping_add(e.mem.u32(a + BLOCK_SIZE));
        }
        if following != e.mem.u32(block_array) {
            let held = e.mem.u32(words);
            fn_00e34ab0(e, this, held, holder, own_floats);
        }
    }
    // The next record's time stamp is stored over the cache word, which nothing reads again.
    profile_record(e, LABEL_RECOMPOSE);
    let input = RecomposeInput {
        transforms: num_transforms,
        floats: num_floats,
        track_data,
        own_transforms: e.mem.u32(a + NUM_TRANSFORM_TRACKS),
        static_floats: e
            .mem
            .u32(a + STATIC_FLOATS)
            .wrapping_add(e.mem.u32(a + DATA)),
        static_range: e.mem.u32(a + RECOMPOSE_STATIC_RANGE),
        dynamic: out,
        dynamic_range: e.mem.u32(a + RECOMPOSE_DYNAMIC_RANGE),
    };
    let description = recompose_description(e, &input);
    e.call(RECOMPOSE, &args![description, transforms_out, floats_out]);
    e.mem.free(description);
    profile_record(e, LABEL_SAMPLE_END);
    local_array_release(e, block_array, true);
    local_array_release(e, out_array, false);
    e.mem.free(arrays);
    e.mem.free(words);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e3_4990, get_num_original_frames(Ptr) -> u32),
        entry!(0x00e3_49a0, get_num_data_chunks(Ptr, u32, u32) -> u32),
        entry!(
            0x00e3_49c0,
            decompress_ov2(Ptr, u32, u32, u32, u32, Ptr, Ptr, Ptr, Ptr)
        ),
        entry!(0x00e3_4a60, sample_tracks(Ptr, f32, u32, u32, u32)),
        entry!(
            0x00e3_4a90,
            sample_individual_float_tracks(Ptr, u32, u32, u32, Ptr)
        ),
        entry!(0x00e3_4ab0, fn_00e34ab0(Ptr, u32, u32, u32) -> u32),
        entry!(0x00e3_4b40, get_full_cache_key(Ptr, u32) -> u32),
        entry!(0x00e3_4b50, clear_all_cache_keys(Ptr, Ptr)),
        entry!(0x00e3_4bb0, fn_00e34bb0(Ptr, u32, u32, Ptr)),
        entry!(0x00e3_4c10, get_data_chunks(Ptr, u32, u32, Ptr, u32)),
        entry!(0x00e3_4c90, get_max_size_of_combined_data_chunks(Ptr) -> i32),
        entry!(
            0x00e3_4d30,
            sample_individual_transform_tracks(Ptr, u32, u32, u32, Ptr)
        ),
        entry!(
            0x00e3_4d80,
            decompress_block_to_cache_ov2(Ptr, u32, Ptr, u32)
        ),
        entry!(0x00e3_4e80, retrieve_from_cache_ov2(u32, Ptr, u32, u32) -> u32),
        entry!(0x00e3_5050, fn_00e35050(Ptr)),
        entry!(
            0x00e3_54b0,
            sample_partial_with_data_chunks(u32, f32, u32, u32, u32, u32, Ptr, u32)
        ),
        entry!(
            0x00e3_59f0,
            sample_partial_tracks(Ptr, f32, u32, u32, u32, u32, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Calls = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// Base addresses of the recording doubles of the animation and of the cache object: the
    /// double for vtable offset `o` of an object sits at `base + o`.
    const ANIM_BASE: u32 = 0x0900_0000;
    const CACHE_BASE: u32 = 0x0901_0000;
    /// The key the animation's slot `0x40` answers.
    const KEY: u32 = 0x1234;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    /// A double at `address` that logs its argument words and answers `answer`.
    fn recorder(e: &mut Engine, calls: &Calls, address: u32, answer: u32) {
        let log = Rc::clone(calls);
        e.register_double(address, move |_, a| {
            log.borrow_mut().push((address, a.to_vec()));
            Ret {
                eax: answer,
                ..Ret::default()
            }
        });
    }

    fn calls_to(calls: &Calls, address: u32) -> Vec<Vec<u32>> {
        calls
            .borrow()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// The vtable offsets, in call order, of the calls made to the object at `base`.
    fn slots(calls: &Calls, base: u32) -> Vec<u32> {
        calls
            .borrow()
            .iter()
            .filter(|(a, _)| a & 0xffff_0000 == base)
            .map(|(a, _)| a & 0xffff)
            .collect()
    }

    /// An object (0x100 bytes) whose vtable has a recording double at each of the byte
    /// `offsets`, answering the paired value.
    fn object(e: &mut Engine, calls: &Calls, base: u32, offsets: &[(u32, u32)]) -> u32 {
        let table = e.mem.alloc(0x80);
        let this = e.mem.alloc(0x100);
        e.mem.set_u32(this, table);
        for &(offset, answer) in offsets {
            e.mem.set_u32(table + offset, base + offset);
            recorder(e, calls, base + offset, answer);
        }
        this
    }

    /// A profiling stream with room for 85 records; the TLS lookup answers it. Returns the stream
    /// and the address of its first record.
    fn profile(e: &mut Engine) -> (u32, u32) {
        let stream = e.mem.alloc(0x20);
        let records = e.mem.alloc(0x400);
        e.mem.set_u32(stream + 4, records);
        e.mem.set_u32(stream + 0xc, records + 0x400);
        e.set_global(PROFILE_SLOT, 3u32);
        e.register_double(TLS_GET_VALUE, move |_, a| {
            assert_eq!(a[0], 3);
            Ret {
                eax: stream,
                ..Ret::default()
            }
        });
        (stream, records)
    }

    /// The labels of the single-label records written so far.
    fn labels(e: &Engine, stream: u32, records: u32) -> Vec<u32> {
        let count = (e.mem.u32(stream + 4) - records) / 12;
        (0..count).map(|i| e.mem.u32(records + 12 * i)).collect()
    }

    /// An animation with a vtable (slot `0x40` answers [`KEY`], `0x20` the frame count 10) and
    /// fields for 2 transform tracks, 1 float track, 10 frames in blocks of 4 and 2 dynamic
    /// floats per frame; the data (returned) holds the first float array (10, 20) at `+0x100`.
    fn animation(e: &mut Engine, calls: &Calls) -> (u32, u32) {
        let anim = object(e, calls, ANIM_BASE, &[(0x40, KEY), (0x20, 10), (0x10, 0)]);
        let data = e.mem.alloc(0x800);
        for (offset, value) in [
            (NUM_TRANSFORM_TRACKS, 2),
            (NUM_FLOAT_TRACKS, 1),
            (NUM_FRAMES, 10),
            (BLOCK_SIZE, 4),
            (FLOATS_PER_FRAME, 2),
            (FIRST_FLOAT_ARRAY, 0x100),
            (SECOND_FLOAT_ARRAY, 0x180),
            (BYTE_ARRAY, 0x200),
            (STATIC_DATA_SIZE, 0x20),
            (TRACK_DESCRIPTION, 0x40),
            (STATIC_FLOATS, 0x60),
            (RECOMPOSE_STATIC_RANGE, 3),
            (RECOMPOSE_DYNAMIC_RANGE, 5),
            (BLOCK_BYTES, 0x100),
            (LAST_BLOCK_BYTES, 0x80),
            (DATA, data),
        ] {
            e.mem.set_u32(anim + offset, value);
        }
        e.mem.set_u8(anim + QUANTISATION_BYTE_0, 0x0a);
        e.mem.set_u8(anim + QUANTISATION_BYTE_1, 0x0b);
        e.mem.set_f32(data + 0x100, 10.0);
        e.mem.set_f32(data + 0x104, 20.0);
        e.mem.set_f32(data + 0x180, 2000.0);
        e.mem.set_f32(data + 0x184, 2001.0);
        e.mem.set_u8(data + 0x200, 5);
        e.mem.set_u8(data + 0x201, 6);
        (anim, data)
    }

    /// Doubles for the expansion helpers: `hkExpandReal` logs `(data, first word of the record,
    /// second float, first float, destination, frames)` and writes `first + k` (plus 100 when the
    /// data pointer is at or past `later`) as value `k` of the destination;
    /// `hkInverseDeltaTransform` is logged; `hkCalcQuantizedSize` answers 100.
    fn expansion(e: &mut Engine, calls: &Calls, later: u32) {
        let log = Rc::clone(calls);
        e.register_double(EXPAND_REAL, move |e, a| {
            let record = a[1];
            let first = e.mem.f32(record + 8);
            log.borrow_mut().push((
                EXPAND_REAL,
                vec![
                    a[0],
                    e.mem.u32(record) & 0xffff,
                    e.mem.u32(record + 4),
                    e.mem.u32(record + 8),
                    a[2],
                    a[3],
                ],
            ));
            let shift = if a[0] >= later { 100.0 } else { 0.0 };
            for k in 0..a[3] {
                e.mem.set_f32(a[2] + 4 * k, first + k as f32 + shift);
            }
            Ret::default()
        });
        recorder(e, calls, INVERSE_DELTA_TRANSFORM, 0);
        recorder(e, calls, CALC_QUANTIZED_SIZE, 100);
    }

    /// A stack router double with a 0x1000-byte arena; returns the router and the arena.
    fn install_router(e: &mut Engine) -> (u32, u32) {
        let arena = e.mem.alloc(0x1000);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router, arena);
        e.mem.set_u32(router + 0x10, 0x0abc_0000);
        e.register_double(STACK_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        (router, arena)
    }

    type Words = Rc<RefCell<Vec<u32>>>;
    type Floats = Rc<RefCell<Vec<f32>>>;

    /// A `recompose` double that keeps the description words and the two dynamic floats.
    fn recomposer(e: &mut Engine) -> (Words, Floats) {
        let words = Rc::new(RefCell::new(Vec::new()));
        let floats = Rc::new(RefCell::new(Vec::new()));
        let (kept_words, kept_floats) = (Rc::clone(&words), Rc::clone(&floats));
        e.register_double(RECOMPOSE, move |e, a| {
            let description: Vec<u32> = (0..8).map(|i| e.mem.u32(a[0] + 4 * i)).collect();
            let dynamic = description[6];
            kept_floats
                .borrow_mut()
                .extend((0..2).map(|i| e.mem.f32(dynamic + 4 * i)));
            kept_words.borrow_mut().extend(description);
            kept_words.borrow_mut().extend([a[1], a[2]]);
            Ret::default()
        });
        (words, floats)
    }

    #[test]
    fn num_original_frames_is_the_frame_count() {
        let mut e = engine();
        let this = e.mem.alloc(0x80);
        e.mem.set_u32(this + 0x28, 77);
        assert_eq!(e.call(0x00e3_4990, &args![this]).u32(), 77);
    }

    #[test]
    fn data_chunks_are_four_for_the_last_frame_of_a_block() {
        let mut e = engine();
        let this = e.mem.alloc(0x80);
        e.mem.set_u32(this + 0x2c, 8);
        for (frame, expected) in [(0u32, 3u32), (6, 3), (7, 4), (8, 3), (15, 4), (16, 3)] {
            let got = e.call(0x00e3_49a0, &args![this, frame, 0xdead_u32]).u32();
            assert_eq!(got, expected, "frame {frame}");
        }
    }

    #[test]
    fn decompress_expands_every_track_into_its_own_stride() {
        let mut e = engine();
        let calls: Calls = Rc::default();
        expansion(&mut e, &calls, u32::MAX);
        let first = e.mem.alloc(8);
        let second = e.mem.alloc(8);
        let widths = e.mem.alloc(8);
        e.mem.set_f32(first, 1.0);
        e.mem.set_f32(first + 4, 2.0);
        e.mem.set_f32(second, 10.0);
        e.mem.set_f32(second + 4, 20.0);
        e.mem.set_u8(widths, 5);
        e.mem.set_u8(widths + 1, 6);
        let dest = e.mem.alloc(0x100);
        let data = 0x0777_0000u32;
        // The low flag byte is the record's second byte, the next one the default first byte.
        let words =
            |widths: u32| args![dest, 2u32, 3u32, 4u32, 0x0907u32, first, second, widths, data];
        e.call(0x00e3_49c0, &words(widths));
        assert_eq!(
            calls_to(&calls, EXPAND_REAL),
            vec![
                vec![data, 0x0705, 10.0f32.to_bits(), 1.0f32.to_bits(), dest, 4],
                vec![
                    data + 100,
                    0x0706,
                    20.0f32.to_bits(),
                    2.0f32.to_bits(),
                    dest + 12,
                    4
                ],
            ]
        );
        assert_eq!(
            calls_to(&calls, INVERSE_DELTA_TRANSFORM),
            vec![vec![dest, 4], vec![dest + 12, 4]]
        );
        let sizes = calls_to(&calls, CALC_QUANTIZED_SIZE);
        assert_eq!(sizes.len(), 2);
        assert_eq!(sizes[0][0], 4);
        // Without a width array every record starts with the default byte; a count of zero
        // expands nothing.
        calls.borrow_mut().clear();
        e.call(0x00e3_49c0, &words(0));
        let expanded = calls_to(&calls, EXPAND_REAL);
        assert_eq!(expanded[0][1], 0x0709);
        assert_eq!(expanded[1][1], 0x0709);
        calls.borrow_mut().clear();
        e.call(
            0x00e3_49c0,
            &args![dest, 0u32, 3u32, 4u32, 0x0907u32, first, second, widths, data],
        );
        assert!(calls.borrow().is_empty());
    }

    #[test]
    fn sample_tracks_calls_slot_0x10_with_the_track_counts() {
        let mut e = engine();
        let calls: Calls = Rc::default();
        let this = object(&mut e, &calls, ANIM_BASE, &[(0x10, 0)]);
        e.mem.set_u32(this + 0x10, 3);
        e.mem.set_u32(this + 0x14, 5);
        e.call(
            0x00e3_4a60,
            &args![
                this,
                f32::from_bits(0x7f80_0001),
                0x111u32,
                0x222u32,
                0x333u32
            ],
        );
        // The time went through FLD/FSTP, which quiets a signalling NaN.
        assert_eq!(
            calls_to(&calls, ANIM_BASE + 0x10),
            vec![vec![this, 0x7fc0_0001, 3, 0x111, 5, 0x222, 0x333]]
        );
        calls.borrow_mut().clear();
        e.call(0x00e3_4a60, &args![this, 1.5f32, 1u32, 2u32, 3u32]);
        assert_eq!(
            calls_to(&calls, ANIM_BASE + 0x10),
            vec![vec![this, 1.5f32.to_bits(), 3, 1, 5, 2, 3]]
        );
    }

    #[test]
    fn individual_float_tracks_are_zeroed() {
        let mut e = engine();
        let dest = e.mem.alloc(0x20);
        for i in 0..4 {
            e.mem.set_u32(dest + 4 * i, 0xffff_ffff);
        }
        e.call(0x00e3_4a90, &args![0u32, 1u32, 2u32, 3u32, dest]);
        assert_eq!(e.mem.u32(dest), 0);
        assert_eq!(e.mem.u32(dest + 8), 0);
        assert_eq!(e.mem.u32(dest + 12), 0xffff_ffff);
        e.mem.set_u32(dest, 7);
        e.call(0x00e3_4a90, &args![0u32, 1u32, 2u32, 0u32, dest]);
        assert_eq!(e.mem.u32(dest), 7);
    }

    #[test]
    fn handing_a_block_back_asks_for_the_key_and_unlocks() {
        let mut e = engine();
        let calls: Calls = Rc::default();
        let anim = object(&mut e, &calls, ANIM_BASE, &[(0x40, KEY)]);
        e.mem.set_u32(anim + 0x2c, 4);
        let cache = object(
            &mut e,
            &calls,
            CACHE_BASE,
            &[(0x2c, 0), (0x1c, 0), (0x30, 0x77)],
        );
        let holder = e.mem.alloc(8);
        e.mem.set_u32(holder, cache);
        let got = e.call(0x00e3_4ab0, &args![anim, 9u32, holder, 3u32]).u32();
        assert_eq!(got, 0x77);
        assert_eq!(calls_to(&calls, ANIM_BASE + 0x40), vec![vec![anim, 9]]);
        assert_eq!(slots(&calls, CACHE_BASE), vec![0x2c, 0x1c, 0x30]);
        let handed = calls_to(&calls, CACHE_BASE + 0x1c);
        assert_eq!(handed.len(), 1);
        assert_eq!(handed[0][0], cache);
        assert_ne!(handed[0][1], 0);
        // key, then floats * block size * 4
        assert_eq!(handed[0][2..], [KEY, 48]);
    }

    #[test]
    fn full_cache_key_is_the_block_number_plus_the_base() {
        let mut e = engine();
        let this = e.mem.alloc(0x80);
        e.mem.set_u32(this + 0x2c, 8);
        e.mem.set_u32(this + 0x6c, 100);
        assert_eq!(e.call(0x00e3_4b40, &args![this, 21u32]).u32(), 102);
        assert_eq!(e.call(0x00e3_4b40, &args![this, 7u32]).u32(), 100);
    }

    #[test]
    fn clearing_the_cache_keys_visits_every_block() {
        let mut e = engine();
        let calls: Calls = Rc::default();
        let anim = object(&mut e, &calls, ANIM_BASE, &[(0x20, 20)]);
        e.mem.set_u32(anim + 0x2c, 8);
        e.mem.set_u32(anim + 0x34, 2);
        e.mem.set_u32(anim + 0x6c, 1000);
        let cache = object(&mut e, &calls, CACHE_BASE, &[(0x14, 0)]);
        e.call(0x00e3_4b50, &args![anim, cache]);
        let cleared = calls_to(&calls, CACHE_BASE + 0x14);
        assert_eq!(cleared.len(), 3);
        for (block, words) in cleared.iter().enumerate() {
            assert_eq!(words[0], cache);
            assert_eq!(words[2..], [1000 + block as u32, 64]);
        }
        assert_eq!(cleared[0][1], cleared[2][1]);
        assert_ne!(cleared[0][1], 0);
        // With no frames the first block is still visited.
        calls.borrow_mut().clear();
        e.register_double(ANIM_BASE + 0x20, |_, _| Ret {
            eax: 0,
            ..Ret::default()
        });
        e.call(0x00e3_4b50, &args![anim, cache]);
        assert_eq!(calls_to(&calls, CACHE_BASE + 0x14).len(), 1);
    }

    #[test]
    fn a_block_chunk_is_aligned_and_the_last_one_is_shorter() {
        let mut e = engine();
        let this = e.mem.alloc(0x80);
        e.mem.set_u32(this + 0x64, 0x30);
        e.mem.set_u32(this + 0x68, 0x20);
        e.mem.set_u32(this + 0x6c, 0x1005);
        e.mem.set_u32(this + 0x44, 0x13);
        let out = e.mem.alloc(0x20);
        e.call(0x00e3_4bb0, &args![this, 1u32, 3u32, out]);
        assert_eq!(e.mem.u32(out), 0x1040);
        assert_eq!(e.mem.u8(out + 8), 8);
        assert_eq!(e.mem.u32(out + 4), 0x40);
        e.call(0x00e3_4bb0, &args![this, 2u32, 3u32, out]);
        assert_eq!(e.mem.u32(out), 0x1070);
        assert_eq!(e.mem.u8(out + 8), 8);
        assert_eq!(e.mem.u32(out + 4), 0x30);
    }

    /// An animation with 10 frames in blocks of 4, block sizes 0x40 / 0x28, data at `0x2005`
    /// and static size 0x21 (no vtable).
    fn chunked(e: &mut Engine) -> u32 {
        let this = e.mem.alloc(0x80);
        for (offset, value) in [
            (NUM_FRAMES, 10),
            (BLOCK_SIZE, 4),
            (BLOCK_BYTES, 0x40),
            (LAST_BLOCK_BYTES, 0x28),
            (DATA, 0x2005),
            (STATIC_DATA_SIZE, 0x21),
        ] {
            e.mem.set_u32(this + offset, value);
        }
        this
    }

    #[test]
    fn data_chunks_describe_the_object_the_static_data_and_the_blocks() {
        let mut e = engine();
        let this = chunked(&mut e);
        let out = e.mem.alloc(0x40);
        e.call(0x00e3_4c10, &args![this, 5u32, 0xdead_u32, out, 3u32]);
        assert_eq!(e.mem.u32(out), this);
        assert_eq!(e.mem.u32(out + 4), 0x80);
        assert_eq!(u32::from(e.mem.u8(out + 8)), this & 0xf);
        assert_eq!(e.mem.u32(out + 0xc), 0x2005);
        assert_eq!(e.mem.u32(out + 0x10), 0x30);
        assert_eq!(e.mem.u8(out + 0x14), 5);
        assert_eq!(e.mem.u32(out + 0x18), 0x2060);
        assert_eq!(e.mem.u32(out + 0x1c), 0x50);
        assert_eq!(e.mem.u8(out + 0x20), 6);
        assert_eq!(e.mem.u32(out + 0x24), 0);
        // Four chunks add the following block, here the last one.
        e.call(0x00e3_4c10, &args![this, 5u32, 0xdead_u32, out, 4u32]);
        assert_eq!(e.mem.u32(out + 0x24), 0x20a0);
        assert_eq!(e.mem.u32(out + 0x28), 0x30);
        assert_eq!(e.mem.u8(out + 0x2c), 6);
        // After the last block the following block wraps to the first.
        e.call(0x00e3_4c10, &args![this, 9u32, 0xdead_u32, out, 4u32]);
        assert_eq!(e.mem.u32(out + 0x18), 0x20a0);
        assert_eq!(e.mem.u32(out + 0x24), 0x2020);
    }

    #[test]
    fn the_largest_combined_chunks_are_found_per_block() {
        let mut e = engine();
        let this = chunked(&mut e);
        assert_eq!(e.call(0x00e3_4c90, &args![this]).i32(), 0x150);
        e.mem.set_u32(this + NUM_FRAMES, 0);
        assert_eq!(e.call(0x00e3_4c90, &args![this]).i32(), 0);
        // One block: its own chunk, the static data and 0x80, nothing following.
        e.mem.set_u32(this + NUM_FRAMES, 3);
        assert_eq!(e.call(0x00e3_4c90, &args![this]).i32(), 0x30 + 0x30 + 0x80);
    }

    #[test]
    fn reference_transforms_are_filled_in() {
        let mut e = engine();
        for lane in 0..4 {
            e.mem.set_u32(IDENTITY_ROTATION + 4 * lane, 0x1000 + lane);
        }
        e.set_global(ONE, 1.0f32);
        let dest = e.mem.alloc(0x60);
        for i in 0..24 {
            e.mem.set_u32(dest + 4 * i, 0xffff_ffff);
        }
        e.call(0x00e3_4d30, &args![0u32, 1u32, 2u32, 1u32, dest]);
        for lane in 0..4 {
            assert_eq!(e.mem.u32(dest + 4 * lane), 0);
            assert_eq!(e.mem.u32(dest + 0x10 + 4 * lane), 0x1000 + lane);
            assert_eq!(e.mem.f32(dest + 0x20 + 4 * lane), 1.0);
        }
        assert_eq!(e.mem.u32(dest + 0x30), 0xffff_ffff);
        e.call(0x00e3_4d30, &args![0u32, 1u32, 2u32, 0u32, dest]);
        assert_eq!(e.mem.f32(dest + 0x20), 1.0);
    }

    #[test]
    fn a_block_is_decompressed_with_the_frames_it_holds() {
        let mut e = engine();
        let calls: Calls = Rc::default();
        let (anim, data) = animation(&mut e, &calls);
        let (stream, records) = profile(&mut e);
        expansion(&mut e, &calls, u32::MAX);
        let dest = e.mem.alloc(0x100);
        e.call(0x00e3_4d80, &args![anim, 9u32, dest, 2u32]);
        // Frame 9 is in block 2, which holds the last 2 frames; the stride is the block size.
        let block = data + 0x220;
        assert_eq!(
            calls_to(&calls, EXPAND_REAL),
            vec![
                vec![
                    block,
                    0x0b05,
                    2000.0f32.to_bits(),
                    10.0f32.to_bits(),
                    dest,
                    2
                ],
                vec![
                    block + 100,
                    0x0b06,
                    2001.0f32.to_bits(),
                    20.0f32.to_bits(),
                    dest + 16,
                    2
                ],
            ]
        );
        assert_eq!(e.mem.f32(dest), 10.0);
        assert_eq!(e.mem.f32(dest + 4), 11.0);
        assert_eq!(e.mem.f32(dest + 16), 20.0);
        assert_eq!(
            labels(&e, stream, records),
            vec![LABEL_DECOMPRESS_BLOCK_CACHE, LABEL_END]
        );
        // A block larger than the animation holds all its frames, from block 0.
        calls.borrow_mut().clear();
        e.mem.set_u32(anim + NUM_FRAMES, 3);
        e.call(0x00e3_4d80, &args![anim, 2u32, dest, 1u32]);
        let expanded = calls_to(&calls, EXPAND_REAL);
        assert_eq!(expanded.len(), 1);
        assert_eq!(expanded[0][0], data + 0x20);
        assert_eq!(expanded[0][5], 3);
    }

    /// The cache object's slots, answering: lock, the entry lookup (a pointer to `pending`), the
    /// existing entry, the new entry, and the handle operations and unlock (`0x99`).
    fn cache(e: &mut Engine, calls: &Calls, pending: u32, existing: u32, fresh: u32) -> u32 {
        object(
            e,
            calls,
            CACHE_BASE,
            &[
                (0x2c, 0),
                (0x28, pending),
                (0xc, existing),
                (0x10, fresh),
                (0x18, 0x99),
                (0x1c, 0x99),
                (0x20, 0x99),
                (0x24, 0x99),
                (0x30, 0x99),
            ],
        )
    }

    #[test]
    fn the_cache_answers_a_pending_an_existing_or_a_new_block() {
        let mut e = engine();
        let calls: Calls = Rc::default();
        let (anim, _data) = animation(&mut e, &calls);
        let (stream, records) = profile(&mut e);
        expansion(&mut e, &calls, u32::MAX);
        let pending = e.mem.alloc(8);
        let free = e.mem.alloc(8);
        let block = e.mem.alloc(0x100);
        let holder = e.mem.alloc(8);
        // Entry marked pending: nothing else is asked.
        e.mem.set_u8(pending, 1);
        let cache_object = cache(&mut e, &calls, pending, 0, 0);
        e.mem.set_u32(holder, cache_object);
        let got = e.call(0x00e3_4e80, &args![holder, anim, 9u32, 3u32]).u32();
        assert_eq!(got, 0);
        assert_eq!(slots(&calls, CACHE_BASE), vec![0x2c, 0x28, 0x30]);
        let asked = calls_to(&calls, CACHE_BASE + 0x28);
        assert_eq!(asked[0][0], cache_object);
        assert_eq!(asked[0][2..], [KEY, 48]);
        // An existing entry is handed back with slot 0x18.
        calls.borrow_mut().clear();
        e.mem.set_u8(pending, 0);
        let cache_object = cache(&mut e, &calls, pending, 0xaaa0, 0);
        e.mem.set_u32(holder, cache_object);
        let got = e.call(0x00e3_4e80, &args![holder, anim, 9u32, 3u32]).u32();
        assert_eq!(got, 0xaaa0);
        assert_eq!(slots(&calls, CACHE_BASE), vec![0x2c, 0x28, 0xc, 0x18, 0x30]);
        // No entry and no room for a new one.
        calls.borrow_mut().clear();
        let cache_object = cache(&mut e, &calls, free, 0, 0);
        e.mem.set_u32(holder, cache_object);
        let got = e.call(0x00e3_4e80, &args![holder, anim, 9u32, 3u32]).u32();
        assert_eq!(got, 0);
        assert_eq!(slots(&calls, CACHE_BASE), vec![0x2c, 0x28, 0xc, 0x10, 0x30]);
        // A new entry is decompressed (3 tracks) and published.
        calls.borrow_mut().clear();
        let cache_object = cache(&mut e, &calls, free, 0, block);
        e.mem.set_u32(holder, cache_object);
        let got = e.call(0x00e3_4e80, &args![holder, anim, 9u32, 3u32]).u32();
        assert_eq!(got, block);
        assert_eq!(
            slots(&calls, CACHE_BASE),
            vec![0x2c, 0x28, 0xc, 0x10, 0x18, 0x20, 0x30, 0x2c, 0x24, 0x30]
        );
        let expanded = calls_to(&calls, EXPAND_REAL);
        assert_eq!(expanded.len(), 3);
        assert_eq!(expanded[0][4], block);
        let begins = labels(&e, stream, records);
        assert_eq!(begins.iter().filter(|l| **l == LABEL_CACHE).count(), 4);
        assert_eq!(begins.last(), Some(&LABEL_END));
    }

    /// Words laid out for `fn_00e35050`: header byte at the data's start, 3 `u16` values after
    /// the 4-byte header, and the arrays the statistics count.
    #[test]
    fn the_animation_data_is_byte_swapped_in_place() {
        let mut e = engine();
        let calls: Calls = Rc::default();
        let this = e.mem.alloc(0x80);
        let data = e.mem.alloc(0x400);
        for (offset, value) in [
            (SWAP_FLAG, 4),
            (NUM_TRANSFORM_TRACKS, 2),
            (NUM_FLOAT_TRACKS, 1),
            (STATIC_FLOATS, 0x40),
            (FIRST_FLOAT_ARRAY, 0x100),
            (SECOND_FLOAT_ARRAY, 0x200),
            (DATA, data),
        ] {
            e.mem.set_u32(this + offset, value);
        }
        e.mem.set_u8(data, 1);
        for (i, value) in [0x1122u16, 0x3344, 0x5566].into_iter().enumerate() {
            e.mem.set_u16(data + 4 + 2 * i as u32, value);
        }
        // Statistics: 2 + 1 + 1 + 1 = 5 static words, 3 + 2 + 1 + 0 = 6 array words.
        let log = Rc::clone(&calls);
        e.register_double(CALC_DYNAMIC_STATS, move |e, a| {
            log.borrow_mut().push((CALC_DYNAMIC_STATS, a.to_vec()));
            for (index, value) in [2u32, 1, 1, 1, 0, 0, 0, 3, 2, 1, 0].into_iter().enumerate() {
                e.mem.set_u32(a[4] + 4 * index as u32, value);
            }
            Ret::default()
        });
        for index in 0..5 {
            e.mem.set_u32(data + 0x40 + 4 * index, 0x0102_0304 + index);
        }
        for index in 0..6 {
            e.mem.set_u32(data + 0x100 + 4 * index, 0x1112_1314 + index);
            e.mem.set_u32(data + 0x200 + 4 * index, 0x2122_2324 + index);
        }
        // A signalling NaN is quieted before and after the swap.
        e.mem.set_u32(data + 0x40 + 16, 0x7f80_0001);
        e.mem.set_u32(data + 0x100 + 20, 0x0100_807f);
        e.call(0x00e3_5050, &args![this]);
        let stats = calls_to(&calls, CALC_DYNAMIC_STATS);
        assert_eq!(stats.len(), 1);
        assert_eq!(stats[0][..4], [data + 4, data + 4 + 4, 2, 1]);
        assert_eq!(e.mem.u16(data + 4), 0x2211);
        assert_eq!(e.mem.u16(data + 6), 0x4433);
        assert_eq!(e.mem.u16(data + 8), 0x6655);
        for index in 0..4 {
            assert_eq!(
                e.mem.u32(data + 0x40 + 4 * index),
                (0x0102_0304 + index).swap_bytes()
            );
        }
        assert_eq!(e.mem.u32(data + 0x40 + 16), 0x0100_c07f);
        for index in 0..5 {
            assert_eq!(
                e.mem.u32(data + 0x100 + 4 * index),
                (0x1112_1314 + index).swap_bytes()
            );
        }
        assert_eq!(e.mem.u32(data + 0x100 + 20), 0x7fc0_0001);
        for index in 0..6 {
            assert_eq!(
                e.mem.u32(data + 0x200 + 4 * index),
                (0x2122_2324 + index).swap_bytes()
            );
        }
        // Nothing happens when the flag is not 4, or when the header byte is clear.
        let before = e.mem.u32(data + 0x40);
        e.mem.set_u32(this + SWAP_FLAG, 3);
        e.call(0x00e3_5050, &args![this]);
        assert_eq!(e.mem.u32(data + 0x40), before);
        e.mem.set_u32(this + SWAP_FLAG, 4);
        e.mem.set_u8(data, 0);
        e.call(0x00e3_5050, &args![this]);
        assert_eq!(e.mem.u32(data + 0x40), before);
        assert_eq!(calls_to(&calls, CALC_DYNAMIC_STATS).len(), 1);
    }

    /// The pieces `samplePartialWithDataChunks` reads: chunk records for an animation (chunk 0,
    /// misaligned by 3), static data (chunk 1, by 2) and two blocks.
    fn chunk_records(e: &mut Engine, anim: u32, data: u32, block: u32, next: u32) -> u32 {
        let chunks = e.mem.alloc(0x40);
        for (chunk, pointer, misaligned) in [
            (0u32, anim - 3, 3u8),
            (0xc, data - 2, 2),
            (0x18, block, 0),
            (0x24, next, 0),
        ] {
            e.mem.set_u32(chunks + chunk, pointer);
            e.mem.set_u8(chunks + chunk + 8, misaligned);
        }
        chunks
    }

    #[test]
    fn chunk_sampling_blends_inside_a_block_or_adds_the_next_block() {
        let mut e = engine();
        let calls: Calls = Rc::default();
        let (anim, data) = animation(&mut e, &calls);
        let block = e.mem.alloc(0x400);
        let next = e.mem.alloc(0x400);
        let chunks = chunk_records(&mut e, anim, data, block, next);
        profile(&mut e);
        expansion(&mut e, &calls, next);
        let (router, arena) = install_router(&mut e);
        let (words, floats) = recomposer(&mut e);
        // Three chunks: frame 1 of block 0 with delta 0.25 blends frames 1 and 2.
        e.call(
            0x00e3_54b0,
            &args![1u32, 0.25f32, 2u32, 0x1111u32, 1u32, 0x2222u32, chunks, 3u32],
        );
        assert_eq!(*floats.borrow(), vec![11.25, 21.25]);
        let kept = words.borrow().clone();
        let track_data = data + 0x40;
        let static_floats = data + 0x60;
        let out = arena + 0x80;
        assert_eq!(
            kept,
            vec![
                2,
                1,
                track_data,
                track_data + 4,
                static_floats,
                static_floats + 12,
                out,
                out + 20,
                0x1111,
                0x2222
            ]
        );
        assert_eq!(e.mem.u32(router), arena);
        let expanded = calls_to(&calls, EXPAND_REAL);
        assert_eq!(expanded.len(), 2);
        assert_eq!(expanded[0][0], block);
        assert_eq!(expanded[0][5], 4);
        // Four chunks: the last frame of block 0 is scaled by 1 - delta and the first frame of
        // the next block, scaled by delta, is added.
        calls.borrow_mut().clear();
        words.borrow_mut().clear();
        floats.borrow_mut().clear();
        e.call(
            0x00e3_54b0,
            &args![3u32, 0.5f32, 2u32, 0x1111u32, 1u32, 0x2222u32, chunks, 4u32],
        );
        assert_eq!(*floats.borrow(), vec![61.5, 71.5]);
        let expanded = calls_to(&calls, EXPAND_REAL);
        assert_eq!(expanded.len(), 4);
        assert_eq!(expanded[2][0], next);
        assert_eq!(e.mem.u32(router), arena);
    }

    /// A `getFrameAndDelta` double that logs its words and writes `frame` and `delta` through the
    /// two pointers it is given.
    fn frame_and_delta(e: &mut Engine, calls: &Calls, frame: u32, delta: f32) {
        let log = Rc::clone(calls);
        e.register_double(GET_FRAME_AND_DELTA, move |e, a| {
            log.borrow_mut().push((GET_FRAME_AND_DELTA, a.to_vec()));
            e.mem.set_u32(a[2], frame);
            e.mem.set_f32(a[3], delta);
            Ret::default()
        });
    }

    #[test]
    fn track_sampling_blends_frames_and_reads_the_next_block_after_a_block_end() {
        let mut e = engine();
        let calls: Calls = Rc::default();
        let (anim, data) = animation(&mut e, &calls);
        profile(&mut e);
        expansion(&mut e, &calls, u32::MAX);
        let (router, arena) = install_router(&mut e);
        let (words, floats) = recomposer(&mut e);
        // Frame 1 with delta 0.25 blends frames 1 and 2 of block 0.
        frame_and_delta(&mut e, &calls, 1, 0.25);
        e.call(
            0x00e3_59f0,
            &args![anim, 0.5f32, 2u32, 0x1111u32, 1u32, 0x2222u32, 0u32],
        );
        assert_eq!(*floats.borrow(), vec![11.25, 21.25]);
        let asked = calls_to(&calls, GET_FRAME_AND_DELTA);
        assert_eq!(asked.len(), 1);
        assert_eq!(asked[0][..2], [anim, 0.5f32.to_bits()]);
        let track_data = data + 0x40;
        let static_floats = data + 0x60;
        assert_eq!(
            *words.borrow(),
            vec![
                2,
                1,
                track_data,
                track_data + 4,
                static_floats,
                static_floats + 12,
                arena,
                arena + 20,
                0x1111,
                0x2222
            ]
        );
        assert_eq!(e.mem.u32(router), arena);
        let expanded = calls_to(&calls, EXPAND_REAL);
        assert_eq!(expanded.len(), 2);
        assert_eq!(expanded[0][0], data + 0x20);
        assert_eq!(expanded[0][4], arena + 0x80);
        // Frame 3 is the last of block 0: scaled by 0.5, then the first frame of block 1 (frame
        // 4) is added, also scaled by 0.5.
        calls.borrow_mut().clear();
        floats.borrow_mut().clear();
        frame_and_delta(&mut e, &calls, 3, 0.5);
        e.call(
            0x00e3_59f0,
            &args![anim, 0.5f32, 2u32, 0x1111u32, 1u32, 0x2222u32, 0u32],
        );
        assert_eq!(*floats.borrow(), vec![11.5, 21.5]);
        let expanded = calls_to(&calls, EXPAND_REAL);
        assert_eq!(expanded.len(), 4);
        assert_eq!(expanded[0][0], data + 0x20);
        assert_eq!(expanded[2][0], data + 0x120);
        assert_eq!(e.mem.u32(router), arena);
    }

    #[test]
    fn track_sampling_reads_cached_blocks_and_hands_them_back() {
        let mut e = engine();
        let calls: Calls = Rc::default();
        let (anim, _data) = animation(&mut e, &calls);
        profile(&mut e);
        expansion(&mut e, &calls, u32::MAX);
        let (router, arena) = install_router(&mut e);
        let (_words, floats) = recomposer(&mut e);
        let cached = e.mem.alloc(0x100);
        for track in 0..2u32 {
            for k in 0..4u32 {
                e.mem
                    .set_f32(cached + 16 * track + 4 * k, (50 + 10 * track + k) as f32);
            }
        }
        let pending = e.mem.alloc(8);
        let cache_object = cache(&mut e, &calls, pending, cached, 0);
        frame_and_delta(&mut e, &calls, 1, 0.25);
        e.call(
            0x00e3_59f0,
            &args![anim, 0.5f32, 2u32, 0x1111u32, 1u32, 0x2222u32, cache_object],
        );
        assert_eq!(*floats.borrow(), vec![51.25, 61.25]);
        assert!(calls_to(&calls, EXPAND_REAL).is_empty());
        assert_eq!(
            slots(&calls, CACHE_BASE),
            vec![0x2c, 0x28, 0xc, 0x18, 0x30, 0x2c, 0x1c, 0x30]
        );
        let asked = calls_to(&calls, CACHE_BASE + 0x28);
        assert_eq!(asked[0][2..], [KEY, 32]);
        let handed = calls_to(&calls, CACHE_BASE + 0x1c);
        assert_eq!(handed[0][2..], [KEY, 32]);
        assert_eq!(e.mem.u32(router), arena);
    }

    #[test]
    fn track_counts_that_differ_are_recounted() {
        let mut e = engine();
        let calls: Calls = Rc::default();
        let (anim, data) = animation(&mut e, &calls);
        profile(&mut e);
        expansion(&mut e, &calls, u32::MAX);
        let (_router, _arena) = install_router(&mut e);
        let (words, floats) = recomposer(&mut e);
        let log = Rc::clone(&calls);
        e.register_double(CALC_DYNAMIC_STATS, move |e, a| {
            log.borrow_mut().push((CALC_DYNAMIC_STATS, a.to_vec()));
            // Words 7 to 9 are the dynamic float counts: 1 in all.
            e.mem.set_u32(a[4] + 0x1c, 1);
            Ret::default()
        });
        frame_and_delta(&mut e, &calls, 1, 0.25);
        e.call(
            0x00e3_59f0,
            &args![anim, 0.5f32, 3u32, 0x1111u32, 0u32, 0x2222u32, 0u32],
        );
        // Without float tracks the caller's transform count is passed.
        let stats = calls_to(&calls, CALC_DYNAMIC_STATS);
        assert_eq!(stats.len(), 1);
        assert_eq!(stats[0][..4], [data + 0x40, data + 0x44, 3, 0]);
        // One float per frame: one value out, one track expanded.
        assert_eq!(floats.borrow()[0], 11.25);
        assert_eq!(calls_to(&calls, EXPAND_REAL).len(), 1);
        assert_eq!(words.borrow()[..2], [3, 0]);
        // With float tracks the animation's own transform count and the caller's float count.
        calls.borrow_mut().clear();
        e.call(
            0x00e3_59f0,
            &args![anim, 0.5f32, 2u32, 0x1111u32, 4u32, 0x2222u32, 0u32],
        );
        let stats = calls_to(&calls, CALC_DYNAMIC_STATS);
        assert_eq!(stats[0][..4], [data + 0x40, data + 0x44, 2, 4]);
    }

    #[test]
    fn the_decompression_record_sits_at_0x10_of_the_exe_frame() {
        let mut e = engine();
        let calls: Calls = Rc::default();
        expansion(&mut e, &calls, u32::MAX);
        let records = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&records);
        e.register_double(CALC_QUANTIZED_SIZE, move |_, a| {
            sink.borrow_mut().push(a[1]);
            100u32.into_ret()
        });
        let first = e.mem.alloc(8);
        let second = e.mem.alloc(8);
        let dest = e.mem.alloc(0x100);
        let frame = e.next_stack_local(0x1c);
        e.call(
            0x00e3_49c0,
            &args![
                dest,
                1u32,
                3u32,
                4u32,
                0x0907u32,
                first,
                second,
                0u32,
                0x0777_0000u32
            ],
        );
        assert_eq!(*records.borrow(), vec![frame + 0x10]);
    }

    #[test]
    fn the_combined_chunk_record_sits_at_0x18_of_the_exe_frame() {
        let mut e = engine();
        let this = chunked(&mut e);
        e.mem.set_u32(this + NUM_FRAMES, 3);
        // The frame is the next stack local; its bytes stay on the thread's stack after the call
        // (nothing takes that place again before they are read).
        let frame = e.next_stack_local(0x24);
        e.call(0x00e3_4c90, &args![this]);
        // One block: its chunk record (data, size, flag byte) is what `00e34bb0` fills there.
        let expected = e.mem.alloc(0x10);
        e.call(0x00e3_4bb0, &args![this, 0u32, 1u32, expected]);
        assert_eq!(e.mem.u32(frame + 0x18), e.mem.u32(expected));
        assert_eq!(e.mem.u32(frame + 0x1c), e.mem.u32(expected + 4));
        assert_eq!(e.mem.u8(frame + 0x20), e.mem.u8(expected + 8));
    }

    #[test]
    fn the_swap_statistics_sit_at_0x20_of_the_exe_frame() {
        let mut e = engine();
        let this = e.mem.alloc(0x80);
        let data = e.mem.alloc(0x400);
        e.mem.set_u32(this + SWAP_FLAG, 4);
        e.mem.set_u32(this + DATA, data);
        e.mem.set_u8(data, 1);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        e.register_double(CALC_DYNAMIC_STATS, move |_, a| {
            sink.borrow_mut().push(a[4]);
            Ret::default()
        });
        let frame = e.next_stack_local(0x58);
        e.call(0x00e3_5050, &args![this]);
        assert_eq!(*seen.borrow(), vec![frame + 0x20]);
    }
}

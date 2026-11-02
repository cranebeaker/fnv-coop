//! `Havok SDK/hkawaveletcompressedanimation.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkaWaveletCompressedAnimation` (Xbox PDB name) keeps its data in blocks. The fields the
//! functions here read are described by offset (unconfirmed): the number of transform tracks
//! (`+0x10`) and of float tracks (`+0x14`), the block size in frames (`+0x2c`), two bytes
//! (`+0x30`, `+0x31`) that make up the word handed to the decompressor, the number of values in
//! a block (`+0x34`), offsets in the data of the minimums (`+0x38`), the ranges (`+0x3c`) and
//! the bit widths (`+0x40`), of the track words (`+0x44`), of the static values (`+0x48`) with
//! the counts of the transform and the float part (`+0x4c`, `+0x50`), of the block table
//! (`+0x54`) and of the quantised data (`+0x5c`), and the data pointer (`+0x64`).
//!
//! The x87 code computes in `f64` and rounds when a `float` is stored. The callees outside this
//! unit (`_decompress`, `hkaTrackAnalysis::calcDynamicStats` and `recompose`) are called by
//! address.
//!
//! The functions of this unit are translated; `funcs()` lists them.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkp3axissweep::MEMORY_ROUTER;
use crate::units::havok_sdk::hkpbvagent::profile_start_record;
use crate::units::havok_sdk::hkpsphereshape::{profile_record, PROFILE_END};

/// `_decompress` (`00e300b0`, usercall: `EAX` = destination, cdecl stack arguments).
const DECOMPRESS: u32 = 0x00e3_00b0;
/// `hkaTrackAnalysis::calcDynamicStats` (`00e4e610`) and `recompose` (`00e4f640`).
const CALC_DYNAMIC_STATS: u32 = 0x00e4_e610;
const RECOMPOSE: u32 = 0x00e4_f640;
/// The profiling labels of `samplePartialWithDataChunks`: `"LtSampleWaveChunk"`,
/// `"StStaticW"`, `"StDecompressWChunk"`, `"StRecomposeWChunk"` and the end label.
const LABEL_SAMPLE_CHUNK: u32 = 0x010e_b344;
const LABEL_STATIC: u32 = 0x010e_b338;
const LABEL_DECOMPRESS_CHUNK: u32 = 0x010e_b324;
const LABEL_RECOMPOSE_CHUNK: u32 = 0x010e_b310;
const LABEL_END: u32 = 0x0107_6f38;

/// Reserves `bytes` (rounded up to 128) from the memory router's bump pointer and returns the
/// start of the block.
fn reserve(e: &mut Engine, bytes: u32) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let start = e.mem.u32(router);
    e.mem.set_u32(
        router,
        start.wrapping_add(bytes.wrapping_add(0x7f) & 0xffff_ff80),
    );
    start
}

/// Sets the memory router's bump pointer back to `start`.
fn release(e: &mut Engine, start: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.set_u32(router, start);
}

/// The address of the float of row `row` at column `column` in a table of rows `block_size`
/// floats long.
fn cell(table: u32, row: u32, block_size: u32, column: u32) -> u32 {
    table.wrapping_add(
        row.wrapping_mul(block_size)
            .wrapping_add(column)
            .wrapping_mul(4),
    )
}

/// Decompresses one block of `count` rows into `table` (`_decompress`): the word, the three
/// offsets and the stream are read from the animation `anim` and the data base `data`.
fn decompress_block(e: &mut Engine, table: u32, anim: u32, data: u32, count: u32, stream: u32) {
    let block_size = e.mem.u32(anim + 0x2c);
    let word = (u32::from(e.mem.u8(anim + 0x30)) << 8) | u32::from(e.mem.u8(anim + 0x31));
    let minimums = e.mem.u32(anim + 0x38).wrapping_add(data);
    let ranges = e.mem.u32(anim + 0x3c).wrapping_add(data);
    let widths = e.mem.u32(anim + 0x40).wrapping_add(data);
    e.call(
        DECOMPRESS,
        &args![table, count, block_size, word, minimums, ranges, widths, stream],
    );
}

/// `samplePartialWithDataChunks`' frame: `SUB ESP,0x6c` and four pushes.
const SAMPLE_CHUNKS_FRAME_SIZE: u32 = 0x7c;

// Translated from 00e30200 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaWaveletCompressedAnimation::samplePartialWithDataChunks` (Xbox PDB name; cdecl, plain
/// `RET`): samples the tracks from a block of data chunks as `getDataChunks` (`00e2fac0`)
/// laid them out. `chunks` holds the animation (`+0`, plus the byte at `+8`), the data base
/// (`+0xc`, plus the byte at `+0x14`) and the streams of the first (`+0x18`, byte `+0x20`) and
/// the second (`+0x24`, byte `+0x2c`) block; `chunk_count` is 3 or 4.
///
/// Between profile records the function counts the values to decode (the animation's own
/// count, or the dynamic count `calcDynamicStats` (`00e4e610`) finds when the requested track
/// counts differ), reserves two buffers from the memory router, decompresses the first block
/// and, for `frame % blockSize`, blends two neighbouring columns with weights `1 - t` and `t`
/// (three chunks). With four chunks the first block is scaled by `1 - t`, the second block is
/// decompressed and its first column, scaled by `t`, is added. The result goes through
/// `recompose` (`00e4f640`) and the router is reset.
pub fn sample_partial_with_data_chunks(
    e: &mut Engine,
    frame: u32,
    time: f32,
    transform_count: u32,
    transforms: u32,
    float_count: u32,
    floats: u32,
    chunks: u32,
    chunk_count: u32,
) {
    // The exe's frame (`SUB ESP,0x6c` and four pushes) holds the statistics block at +0x24
    // (00e302d0) and the analysis block at +0x5c (00e30544).
    e.with_stack(SAMPLE_CHUNKS_FRAME_SIZE, |e, locals| {
        let locals = locals.addr();
        let anim = e
            .mem
            .u32(chunks)
            .wrapping_add(u32::from(e.mem.u8(chunks + 8)));
        profile_start_record(e, LABEL_SAMPLE_CHUNK, LABEL_STATIC);
        let data = e
            .mem
            .u32(chunks + 0xc)
            .wrapping_add(u32::from(e.mem.u8(chunks + 0x14)));
        let mut count = e.mem.u32(anim + 0x34);
        let static_values = e.mem.u32(anim + 0x48).wrapping_add(data);
        let anim_tracks = e.mem.u32(anim + 0x10);
        let words = e.mem.u32(anim + 0x44).wrapping_add(data);
        if transform_count != anim_tracks || float_count != e.mem.u32(anim + 0x14) {
            let float_words = words.wrapping_add(anim_tracks.wrapping_mul(2));
            let stats = locals + 0x24;
            let sum = if float_count == 0 {
                e.call(
                    CALC_DYNAMIC_STATS,
                    &args![words, float_words, transform_count, 0u32, stats],
                );
                e.mem
                    .u32(stats + 0x20)
                    .wrapping_add(e.mem.u32(stats + 0x24))
            } else {
                e.call(
                    CALC_DYNAMIC_STATS,
                    &args![words, float_words, anim_tracks, float_count, stats],
                );
                e.mem
                    .u32(stats + 0x24)
                    .wrapping_add(e.mem.u32(stats + 0x28))
                    .wrapping_add(e.mem.u32(stats + 0x20))
            };
            count = sum.wrapping_add(e.mem.u32(stats + 0x1c));
        }
        profile_record(e, LABEL_DECOMPRESS_CHUNK);
        let block_size = e.mem.u32(anim + 0x2c);
        let rest = (1.0 - f64::from(time)) as f32;
        let table = reserve(e, block_size.wrapping_mul(count).wrapping_mul(4));
        let result = reserve(e, count.wrapping_mul(4));
        let first_stream =
            u32::from(e.mem.u8(chunks + 0x20)).wrapping_add(e.mem.u32(chunks + 0x18));
        decompress_block(e, table, anim, data, count, first_stream);
        let block_size = e.mem.u32(anim + 0x2c);
        if block_size == 0 {
            panic!("integer divide fault at 0x00e30404");
        }
        let column = frame % block_size;
        if chunk_count == 3 {
            for row in 0..count {
                let low = f64::from(e.mem.f32(cell(table, row, block_size, column)));
                let high = f64::from(e.mem.f32(cell(table, row, block_size, column) + 4));
                let value = (high * f64::from(time) + low * f64::from(rest)) as f32;
                e.mem
                    .set_f32(result.wrapping_add(row.wrapping_mul(4)), value);
            }
        } else {
            for row in 0..count {
                let low = f64::from(e.mem.f32(cell(table, row, block_size, column)));
                let value = (low * f64::from(rest)) as f32;
                e.mem
                    .set_f32(result.wrapping_add(row.wrapping_mul(4)), value);
            }
            let second_stream =
                u32::from(e.mem.u8(chunks + 0x2c)).wrapping_add(e.mem.u32(chunks + 0x24));
            decompress_block(e, table, anim, data, count, second_stream);
            let block_size = e.mem.u32(anim + 0x2c);
            for row in 0..count {
                let high = f64::from(e.mem.f32(cell(table, row, block_size, 0)));
                let at = result.wrapping_add(row.wrapping_mul(4));
                let value = (high * f64::from(time) + f64::from(e.mem.f32(at))) as f32;
                e.mem.set_f32(at, value);
            }
        }
        profile_record(e, LABEL_RECOMPOSE_CHUNK);
        let analysis = locals + 0x5c;
        e.mem.set_u32(analysis, transform_count);
        e.mem.set_u32(analysis + 4, float_count);
        e.mem.set_u32(analysis + 8, words);
        let tracks = e.mem.u32(anim + 0x10);
        e.mem
            .set_u32(analysis + 0xc, words.wrapping_add(tracks.wrapping_mul(2)));
        e.mem.set_u32(analysis + 0x10, static_values);
        let static_count = e.mem.u32(anim + 0x4c);
        e.mem.set_u32(
            analysis + 0x14,
            static_values.wrapping_add(static_count.wrapping_mul(4)),
        );
        e.mem.set_u32(analysis + 0x18, result);
        let float_static = e.mem.u32(anim + 0x50);
        e.mem.set_u32(
            analysis + 0x1c,
            result.wrapping_add(float_static.wrapping_mul(4)),
        );
        e.call(RECOMPOSE, &args![analysis, transforms, floats]);
        release(e, result);
        release(e, table);
        profile_record(e, LABEL_END);
    })
}

/// The profiling labels of the unit's other functions: `"LtSampleWave"`, `"StDecompressW"`,
/// `"StInterpW"`, `"StRecomposeW"` (`samplePartialTracks`, the start record also carries
/// `LABEL_STATIC`), `"TtdecompressBlockCacheW"` and `"TtcacheW"`.
const LABEL_SAMPLE: u32 = 0x010e_b3a8;
const LABEL_DECOMPRESS: u32 = 0x010e_b398;
const LABEL_INTERPOLATE: u32 = 0x010e_b38c;
const LABEL_RECOMPOSE: u32 = 0x010e_b37c;
const LABEL_DECOMPRESS_BLOCK: u32 = 0x010e_b358;
const LABEL_CACHE: u32 = 0x010e_b370;
/// The `float` `4294967296.0` the code adds to a signed value it treats as unsigned (`FILD`).
const UNSIGNED_BIAS: u32 = 0x0109_9378;
/// The `float` `1.0` and the identity quaternion the track samplers store.
const ONE: u32 = 0x0101_2068;
const IDENTITY_ROTATION: u32 = 0x010c_71b0;
/// The endian analysis (`00e4ebb0`, cdecl `(words, float words, transform count, float count,
/// stats)`); the same statistics block as `calcDynamicStats`.
const ANALYSE_ENDIAN: u32 = 0x00e4_ebb0;
/// `hkCalcQuantizedSize` (`00e51da0`, cdecl `(values, bytes)`), `hkBlockDecode` (`00e51a60`, cdecl
/// `(stream, parameters, buffer, size)`, answers the bytes used), `hkExpandReal` (`00e51660`, cdecl
/// `(buffer, bytes, destination, values)`) and the pass over the expanded row (`00e51140`, cdecl
/// `(destination, values)`).
const CALC_QUANTIZED_SIZE: u32 = 0x00e5_1da0;
const BLOCK_DECODE: u32 = 0x00e5_1a60;
const EXPAND_REAL: u32 = 0x00e5_1660;
const ROW_PASS: u32 = 0x00e5_1140;

/// `x87 FISTP qword` with truncation (the control word rounds toward zero): the low dword of the
/// result, which is all the code reads; NaN and values that do not fit give the "integer
/// indefinite" `0x8000000000000000`, whose low dword is 0.
fn fistp_truncate_low(value: f64) -> u32 {
    if (-9_223_372_036_854_775_808.0..9_223_372_036_854_775_808.0).contains(&value) {
        (value.trunc() as i64) as u32
    } else {
        0
    }
}

/// `FILD dword` of a value the code treats as unsigned: a negative signed value gets the float
/// at `01099378` (2^32) added.
fn fild_unsigned(e: &Engine, value: u32) -> f64 {
    let mut result = f64::from(value as i32);
    if (value as i32) < 0 {
        result += f64::from(e.global::<f32>(UNSIGNED_BIAS));
    }
    result
}

// Translated from 00e2f7c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaAnimation::getFrameAndDelta` (Xbox PDB), stack `(time, frame_out, delta_out)`, `RET 0xC`:
/// asks virtual slot `+0x20` for the frame count `n`, divides `time` by the duration (`+0xC`), and
/// multiplies by `n - 1` (taken as unsigned: `2^32` is added when it is negative as a signed word).
/// The product goes through a `float` store, then truncates to a 64-bit integer (the exe sets the
/// control word to round toward zero) whose low dword is the frame, stored at `frame_out`. When the
/// frame is above `n - 2` (unsigned) it is clamped to `n - 2` and the delta is `1.0`; otherwise the
/// delta is the product minus the frame (also unsigned) as a `float`. The x87 code computes in
/// extended precision, here in `f64`.
pub fn hka_animation_get_frame_and_delta(
    e: &mut Engine,
    this: Ptr,
    time: f32,
    frame_out: u32,
    delta_out: u32,
) {
    let frames = e.vcall(this.addr(), 0x20, &args![]).u32();
    let duration = e.mem.f32(this.addr() + 0xc);
    let quotient = f64::from(x87_quiet(time)) / f64::from(x87_quiet(duration));
    let last = frames.wrapping_sub(1);
    let scaled = fild_unsigned(e, last) * quotient;
    let position = f64::from(scaled as f32);
    let frame = fistp_truncate_low(position);
    e.mem.set_u32(frame_out, frame);
    let limit = frames.wrapping_sub(2);
    if frame > limit {
        e.mem.set_u32(frame_out, limit);
        e.mem.set_f32(delta_out, 1.0);
    } else {
        let delta = (position - fild_unsigned(e, frame)) as f32;
        e.mem.set_f32(delta_out, delta);
    }
}

// Translated from 00e2f8b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaWaveletCompressedAnimation::getNumOriginalFrames` (Xbox PDB): the word at `+0x28`.
pub fn hka_wavelet_compressed_animation_get_num_original_frames(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x28)
}

// Translated from 00e2f8c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaWaveletCompressedAnimation::getNumDataChunks` (Xbox PDB), stack `(frame, unused)`,
/// `RET 8`: 3 data chunks, or 4 when the frame is the last of its block (`frame % blockSize + 1`
/// equals the block size `+0x2C`). A zero block size is a divide fault.
pub fn hka_wavelet_compressed_animation_get_num_data_chunks(
    e: &mut Engine,
    this: Ptr,
    frame: u32,
    _unused_2: u32,
) -> u32 {
    let block_size = e.mem.u32(this.addr() + 0x2c);
    if block_size == 0 {
        panic!("integer divide fault at 0x00e2f8c9");
    }
    let remainder = frame % block_size;
    if remainder.wrapping_add(1).wrapping_sub(block_size) != 0 {
        3
    } else {
        4
    }
}

// Translated from 00e2f8e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaWaveletCompressedAnimation::sampleTracks` (Xbox PDB), stack `(time, transforms, floats,
/// cache)`, `RET 0x10`: forwards to virtual slot `+0x10` (`samplePartialTracks`) with `(time,
/// transform track count (+0x10), transforms, float track count (+0x14), floats, cache)`; the time
/// goes through the x87 stack (a signalling NaN is quieted). The callee's `EAX` is passed through.
pub fn hka_wavelet_compressed_animation_sample_tracks(
    e: &mut Engine,
    this: Ptr,
    time: f32,
    transforms: u32,
    floats: u32,
    cache: u32,
) -> u32 {
    let transform_count = e.mem.u32(this.addr() + 0x10);
    let float_count = e.mem.u32(this.addr() + 0x14);
    e.vcall(
        this.addr(),
        0x10,
        &args![
            x87_quiet(time),
            transform_count,
            transforms,
            float_count,
            floats,
            cache
        ],
    )
    .u32()
}

// Translated from 00e2f910 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. It takes its inputs in registers, so they come first, in this order:
/// `EAX` = `animation`, `ECX` = `frame`, `EDI` = `holder` (the address of a word that holds the
/// cache object); then the stack word `count` (the exe never reads it back: `RET` is a jump). It
/// asks `animation` (virtual slot `+0x40`, `getFullCacheKey`) for the key of `frame`, computes
/// `bytes = blockSize (+0x2C) * count * 4`, calls the cache object (`[holder]`): slot `+0x2C` (no
/// arguments), slot `+0x1C` with `(address of the stack word, key, bytes)`, and ends with a jump to
/// its slot `+0x30`, whose `EAX` is the result. The cache is read from `holder` again for each
/// call.
pub fn fn_00e2f910(e: &mut Engine, animation: u32, frame: u32, holder: u32, count: u32) -> u32 {
    let key = e.vcall(animation, 0x40, &args![frame]).u32();
    let bytes = e
        .mem
        .u32(animation + 0x2c)
        .wrapping_mul(count)
        .wrapping_mul(4);
    let cache = e.mem.u32(holder);
    e.vcall(cache, 0x2c, &args![]);
    let cache = e.mem.u32(holder);
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), count);
        e.vcall(cache, 0x1c, &args![slot, key, bytes]);
    });
    let cache = e.mem.u32(holder);
    e.vcall(cache, 0x30, &args![]).u32()
}

// Translated from 00e2f950 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaWaveletCompressedAnimation::sampleIndividualFloatTracks` (Xbox PDB), stack `(unused,
/// unused, count, out)`, `RET 0x10`: stores `count` zero dwords at `out` (`REP STOSD`).
pub fn hka_wavelet_compressed_animation_sample_individual_float_tracks(
    e: &mut Engine,
    _this: Ptr,
    _unused_1: u32,
    _unused_2: u32,
    count: u32,
    out: u32,
) {
    for index in 0..count {
        e.mem.set_u32(out.wrapping_add(index.wrapping_mul(4)), 0);
    }
}

// Translated from 00e2f9f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaWaveletCompressedAnimation::getFullCacheKey` (Xbox PDB), stack `(frame)`, `RET 4`:
/// `frame / blockSize (+0x2C) + the word at +0x64`; a zero block size is a divide fault.
pub fn hka_wavelet_compressed_animation_get_full_cache_key(
    e: &mut Engine,
    this: Ptr,
    frame: u32,
) -> u32 {
    let block_size = e.mem.u32(this.addr() + 0x2c);
    if block_size == 0 {
        panic!("integer divide fault at 0x00e2f9f6");
    }
    (frame / block_size).wrapping_add(e.mem.u32(this.addr() + 0x64))
}

// Translated from 00e2fa00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaWaveletCompressedAnimation::clearAllCacheKeys` (Xbox PDB), stack `(cache)`, `RET 4`: asks
/// virtual slot `+0x20` for the frame count `n`, takes `blocks = (n - 1) / blockSize (+0x2C) + 1`
/// (signed division; a zero block size, or `i32::MIN / -1`, is a divide fault) and, for each block
/// `i` while `i < blocks` (signed), calls the cache's virtual slot `+0x14` with `(address of a
/// scratch byte, the word at +0x64 plus i, bytes)`, where `bytes` is the word at `+0x34` times the
/// block size times 4.
pub fn hka_wavelet_compressed_animation_clear_all_cache_keys(
    e: &mut Engine,
    this: Ptr,
    cache: u32,
) {
    let t = this.addr();
    let block_size = e.mem.u32(t + 0x2c);
    let frames = e.vcall(t, 0x20, &args![]).u32();
    let last = frames.wrapping_sub(1) as i32;
    let divisor = block_size as i32;
    if divisor == 0 || (last == i32::MIN && divisor == -1) {
        panic!("integer divide fault at 0x00e2fa13");
    }
    let bytes = e.mem.u32(t + 0x34).wrapping_mul(block_size).wrapping_mul(4);
    let blocks = (last / divisor).wrapping_add(1);
    if blocks > 0 {
        // The scratch byte is the high byte of a saved word in the exe's frame.
        e.with_stack(4, |e, slot| {
            let mut index: i32 = 0;
            loop {
                let key = e.mem.u32(t + 0x64).wrapping_add(index as u32);
                e.vcall(cache, 0x14, &args![slot.addr() + 3, key, bytes]);
                index = index.wrapping_add(1);
                if index >= blocks {
                    break;
                }
            }
        });
    }
}

// Translated from 00e2fa60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map, stack `(index, out)`, `RET 8`: describes block `index` of the data.
/// The block's address is the word at `+0x5C` plus the data base (`+0x64`) plus entry `index` of the
/// block table (`+0x54` plus the data base). `out` gets that address rounded down to 16 (`+0`), the
/// misalignment (the low nibble, a byte at `+8`) and, at `+4`, the block's size (the next entry
/// minus this one, or for the last block (`index == count (+0x58) - 1`) the word at `+0x60` minus
/// this one) plus the misalignment plus 15, rounded down to 16.
pub fn fn_00e2fa60(e: &mut Engine, this: Ptr, index: u32, out: u32) {
    let t = this.addr();
    let data = e.mem.u32(t + 0x64);
    let table = e.mem.u32(t + 0x54).wrapping_add(data);
    let entry = table.wrapping_add(index.wrapping_mul(4));
    let address = e
        .mem
        .u32(t + 0x5c)
        .wrapping_add(data)
        .wrapping_add(e.mem.u32(entry));
    let misalignment = (address as u8) & 0xf;
    e.mem.set_u32(out, address & 0xffff_fff0);
    e.mem.set_u8(out + 8, misalignment);
    let last = e.mem.u32(t + 0x58).wrapping_sub(1);
    if index == last {
        let size = e.mem.u32(t + 0x60).wrapping_sub(e.mem.u32(entry));
        e.mem.set_u32(out + 4, size);
    } else {
        let size = e.mem.u32(entry + 4).wrapping_sub(e.mem.u32(entry));
        e.mem.set_u32(out + 4, size);
    }
    let size = e.mem.u32(out + 4);
    e.mem.set_u32(
        out + 4,
        u32::from(misalignment).wrapping_add(size).wrapping_add(0xf) & 0xffff_fff0,
    );
}

// Translated from 00e2fac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaWaveletCompressedAnimation::getDataChunks` (Xbox PDB), stack `(frame, unused, out,
/// count)`, `RET 0x10`: lays out the chunks `samplePartialWithDataChunks` (`00e30200`) reads:
/// the animation object itself (address at `+0`, size `0x70` at `+4`, the address's low nibble as a
/// byte at `+8`), the data (the word at `+0x64` at `+0xC`, the word at `+0x5C` plus 15 rounded down
/// to 16 at `+0x10`, its low nibble at `+0x14`), the chunk of the frame's block (`00e2fa60` at
/// `+0x18`) and, when `count` is 4, the chunk of the next block, wrapping at the block count
/// (`+0x58`), at `+0x24`. A zero block size or block count is a divide fault.
pub fn hka_wavelet_compressed_animation_get_data_chunks(
    e: &mut Engine,
    this: Ptr,
    frame: u32,
    _unused_2: u32,
    out: u32,
    count: u32,
) {
    let t = this.addr();
    e.mem.set_u8(out + 8, (t as u8) & 0xf);
    e.mem.set_u32(out, t);
    e.mem.set_u32(out + 4, 0x70);
    let data = e.mem.u32(t + 0x64);
    e.mem.set_u32(out + 0xc, data);
    let size = e.mem.u32(t + 0x5c).wrapping_add(0xf) & 0xffff_fff0;
    e.mem.set_u32(out + 0x10, size);
    e.mem.set_u8(out + 0x14, (data as u8) & 0xf);
    let block_size = e.mem.u32(t + 0x2c);
    if block_size == 0 {
        panic!("integer divide fault at 0x00e2fafa");
    }
    let block = frame / block_size;
    fn_00e2fa60(e, this, block, out + 0x18);
    if count == 4 {
        let blocks = e.mem.u32(t + 0x58);
        if blocks == 0 {
            panic!("integer divide fault at 0x00e2fb17");
        }
        fn_00e2fa60(e, this, block.wrapping_add(1) % blocks, out + 0x24);
    }
}

// Translated from 00e2fb30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaWaveletCompressedAnimation::getMaxSizeOfCombinedDataChunks` (Xbox PDB): the largest, over
/// the blocks (count at `+0x58`), of `(the word at +0x5C + 15 rounded down to 16) + 0x70 + the
/// size of the block's chunk + the size of the next block's chunk (not for the last block)`, the
/// sizes from `00e2fa60` (a 12-byte description, zeroed for each block); compared signed, 0 when
/// there are no blocks.
pub fn hka_wavelet_compressed_animation_get_max_size_of_combined_data_chunks(
    e: &mut Engine,
    this: Ptr,
) -> u32 {
    let t = this.addr();
    let blocks = e.mem.u32(t + 0x58);
    let mut maximum: i32 = 0;
    if blocks != 0 {
        let fixed = e.mem.u32(t + 0x5c).wrapping_add(0xf) & 0xffff_fff0;
        let last = blocks.wrapping_sub(1);
        let mut index: u32 = 0;
        loop {
            let total = e.with_stack(12, |e, chunk| {
                let chunk = chunk.addr();
                fn_00e2fa60(e, this, index, chunk);
                let mut total = fixed.wrapping_add(e.mem.u32(chunk + 4)).wrapping_add(0x70);
                if index < last {
                    fn_00e2fa60(e, this, index.wrapping_add(1), chunk);
                    total = total.wrapping_add(e.mem.u32(chunk + 4));
                }
                total
            });
            if total as i32 > maximum {
                maximum = total as i32;
            }
            index = index.wrapping_add(1);
            if index >= e.mem.u32(t + 0x58) {
                break;
            }
        }
    }
    maximum as u32
}

// Translated from 00e2fbc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaWaveletCompressedAnimation::sampleIndividualTransformTracks` (Xbox PDB), stack `(unused,
/// unused, count, out)`, `RET 0x10`: writes `count` identity transforms (0x30 bytes each) at `out`:
/// translation zero, rotation the vector at `010c71b0` and scale four copies of the float at
/// `01012068`.
pub fn hka_wavelet_compressed_animation_sample_individual_transform_tracks(
    e: &mut Engine,
    _this: Ptr,
    _unused_1: u32,
    _unused_2: u32,
    count: u32,
    out: u32,
) {
    let mut at = out;
    for _ in 0..count {
        for lane in 0..4 {
            e.mem.set_u32(at + 4 * lane, 0);
        }
        for lane in 0..4 {
            let word = e.mem.u32(IDENTITY_ROTATION + 4 * lane);
            e.mem.set_u32(at + 0x10 + 4 * lane, word);
        }
        let one = e.mem.u32(ONE);
        for lane in 0..4 {
            e.mem.set_u32(at + 0x20 + 4 * lane, one);
        }
        at = at.wrapping_add(0x30);
    }
}

/// One dword of the endian pass over the float-typed arrays: `FLD`/`FSTP` copies it (quieting a
/// signalling NaN), the bytes are reversed in integer registers, and `FLD`/`FSTP` copies the result
/// back (quieting again).
fn swap_dword_through_x87(e: &mut Engine, at: u32) {
    let loaded = x87_quiet_bits(e.mem.u32(at));
    e.mem.set_u32(at, x87_quiet_bits(loaded.swap_bytes()));
}

/// The endian conversion's frame: `SUB ESP,0x4c` and four pushes.
const ENDIAN_FRAME_SIZE: u32 = 0x5c;

// Translated from 00e2fc10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map, `this` = the animation: the endian conversion of the stored data.
/// Does nothing unless the word at `+0x44` is 4 and the first byte of the data (`+0x64`) is not
/// zero. Reverses the bytes of the `count = (+0x10) + (+0x14)` halfwords from `data + 4` (while
/// `count` is positive, as signed), has `00e4ebb0` analyse them into a 14-word statistics block
/// (zeroed first), then reverses the dwords of three arrays through the x87 stack (each load and
/// store quiets a signalling NaN): `n1` dwords from `+0x48` (the sum of statistics 0..3, signed:
/// nothing when it is not positive) and `n2` dwords from `+0x38` and from `+0x3C` (statistics 7..10,
/// element by element, first one array then the other; the count is taken as unsigned, so a
/// negative one runs off the data). Last the `count (+0x58)` dwords of the block table (`+0x54`) are
/// reversed. The compiler's four-way unrolling of the loops is folded into one loop each.
pub fn fn_00e2fc10(e: &mut Engine, this: Ptr) {
    let t = this.addr();
    if e.mem.u32(t + 0x44) != 4 {
        return;
    }
    let data = e.mem.u32(t + 0x64);
    if e.mem.u8(data) == 0 {
        return;
    }
    let halfwords = data.wrapping_add(4);
    let count = e.mem.i32(t + 0x10).wrapping_add(e.mem.i32(t + 0x14));
    if count > 0 {
        let mut index: i32 = 0;
        let mut at = halfwords;
        loop {
            let value = e.mem.u16(at);
            e.mem.set_u16(at, value.swap_bytes());
            index += 1;
            at = at.wrapping_add(2);
            let count = e.mem.i32(t + 0x10).wrapping_add(e.mem.i32(t + 0x14));
            if index >= count {
                break;
            }
        }
    }
    let transforms = e.mem.u32(t + 0x10);
    let floats = e.mem.u32(t + 0x14);
    // The statistics block sits at +0x24 of the exe's frame (`SUB ESP,0x4c` and four pushes,
    // 00e2fc89).
    e.with_stack(ENDIAN_FRAME_SIZE, |e, frame| {
        let stats = frame.addr() + 0x24;
        e.call(
            ANALYSE_ENDIAN,
            &args![
                halfwords,
                halfwords.wrapping_add(transforms.wrapping_mul(2)),
                transforms,
                floats,
                stats
            ],
        );
        let stat = |e: &Engine, index: u32| e.mem.i32(stats + 4 * index);
        let first_count = stat(e, 2)
            .wrapping_add(stat(e, 3))
            .wrapping_add(stat(e, 1))
            .wrapping_add(stat(e, 0));
        let first = e.mem.u32(t + 0x48).wrapping_add(e.mem.u32(t + 0x64));
        if first_count > 0 {
            for index in 0..first_count as u32 {
                swap_dword_through_x87(e, first.wrapping_add(index.wrapping_mul(4)));
            }
        }
        let pair_count = stat(e, 9)
            .wrapping_add(stat(e, 10))
            .wrapping_add(stat(e, 8))
            .wrapping_add(stat(e, 7));
        let data = e.mem.u32(t + 0x64);
        let second = e.mem.u32(t + 0x38).wrapping_add(data);
        let third = e.mem.u32(t + 0x3c).wrapping_add(data);
        for index in 0..pair_count as u32 {
            swap_dword_through_x87(e, second.wrapping_add(index.wrapping_mul(4)));
            swap_dword_through_x87(e, third.wrapping_add(index.wrapping_mul(4)));
        }
        let table = e.mem.u32(t + 0x54).wrapping_add(e.mem.u32(t + 0x64));
        let mut index: u32 = 0;
        while index < e.mem.u32(t + 0x58) {
            let at = table.wrapping_add(index.wrapping_mul(4));
            let value = e.mem.u32(at);
            e.mem.set_u32(at, value.swap_bytes());
            index = index.wrapping_add(1);
        }
    });
}

// Translated from 00e300b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_decompress` (Xbox PDB): decompresses `count` rows of `block_size` values into `dest` (`EAX`,
/// so first; then the stack words). `word` carries the default bit width in its second byte and
/// another byte in its first (only the low 16 bits are read). `minimums` and `ranges` are float
/// arrays by row, `widths` an optional byte array by row (0: every row uses the default width), and
/// `stream` the bit stream. A scratch buffer for `hkCalcQuantizedSize` bytes (rounded up to 128) is
/// taken from the memory router's bump pointer and given back at the end. For each row: the
/// minimum and range are copied through the x87 stack, the quantised size of the row is asked
/// (`00e51da0`), the quantised zero point `q = trunc((-min * (1 << width)) / range)` (a width of 32
/// or more shifts by its low 5 bits; `FIMUL` takes `1 << width` as signed; `q` is the low dword of
/// the 64-bit truncation, and `q == 1 << width` becomes `q - 1`) goes with the width into the
/// 8-byte parameter block of `hkBlockDecode` (`00e51a60`), which reads the row from the stream
/// (advancing it by the bytes it answers) into the scratch buffer; `hkExpandReal` (`00e51660`) and
/// `00e51140` then fill and finish the row at `dest`, which advances by `block_size` floats.
pub fn decompress(
    e: &mut Engine,
    dest: u32,
    count: u32,
    block_size: u32,
    word: u32,
    minimums: u32,
    ranges: u32,
    widths: u32,
    stream: u32,
) {
    e.with_stack(0x30, |e, frame| {
        // The exe's frame: the two width bytes (`E - 0x18`), the float copies of the minimum
        // (`E - 0x10`) and the range (`E - 0x14`), and the parameter block (`E - 0xc`, `E - 8`).
        let f = frame.addr();
        let bytes = f + 0x18;
        let minimum_slot = f + 0x20;
        let range_slot = f + 0x1c;
        let parameters = f + 0x24;
        e.mem.set_u8(bytes, (word >> 8) as u8);
        e.mem.set_u8(bytes + 1, word as u8);
        let size = e.call(CALC_QUANTIZED_SIZE, &args![block_size, bytes]).u32();
        let scratch = reserve(e, size);
        let mut stream = stream;
        let mut row_at = dest;
        let mut row: u32 = 0;
        while row < count {
            let minimum = x87_quiet_bits(e.mem.u32(minimums.wrapping_add(row.wrapping_mul(4))));
            e.mem.set_u32(minimum_slot, minimum);
            let range = x87_quiet_bits(e.mem.u32(ranges.wrapping_add(row.wrapping_mul(4))));
            e.mem.set_u32(range_slot, range);
            if widths != 0 {
                let width = e.mem.u8(widths.wrapping_add(row));
                e.mem.set_u8(bytes, width);
            } else {
                e.mem.set_u8(bytes, (word >> 8) as u8);
            }
            let step = e.call(CALC_QUANTIZED_SIZE, &args![block_size, bytes]).u32();
            let width = u32::from(e.mem.u8(bytes));
            let unit = 1u32 << (width & 31);
            e.mem.set_u32(parameters, width);
            let negated = -f64::from(f32::from_bits(minimum));
            let value = (negated * f64::from(unit as i32)) / f64::from(f32::from_bits(range));
            let mut quantised = fistp_truncate_low(value);
            e.mem.set_u32(parameters + 4, quantised);
            if quantised == unit {
                quantised = quantised.wrapping_sub(1);
                e.mem.set_u32(parameters + 4, quantised);
            }
            let used = e
                .call(BLOCK_DECODE, &args![stream, parameters, scratch, step])
                .u32();
            stream = stream.wrapping_add(used);
            e.call(EXPAND_REAL, &args![scratch, bytes, row_at, block_size]);
            e.call(ROW_PASS, &args![row_at, block_size]);
            row = row.wrapping_add(1);
            row_at = row_at.wrapping_add(block_size.wrapping_mul(4));
        }
        release(e, scratch);
    });
}

// Translated from 00e306d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `decompressBlockToCache` (Xbox PDB): `EAX` = `animation` (so first), then the stack words
/// `(frame, dest, count)`. Profiled (`"TtdecompressBlockCacheW"`, then the end label). Decompresses
/// the block of `frame` (`frame / blockSize (+0x2C)`; a zero block size is a divide fault) into
/// `dest` with `_decompress`: `count` rows, the block size, the word made of the bytes at `+0x30`
/// (high) and `+0x31` (low), the minimums (`+0x38`), ranges (`+0x3C`) and widths (`+0x40`) in the
/// data (`+0x64`), and the stream: the block table (`+0x54`) entry of the block plus the word at
/// `+0x5C`, in the data.
pub fn decompress_block_to_cache(
    e: &mut Engine,
    animation: u32,
    frame: u32,
    dest: u32,
    count: u32,
) {
    profile_record(e, LABEL_DECOMPRESS_BLOCK);
    let data = e.mem.u32(animation + 0x64);
    let low = u32::from(e.mem.u8(animation + 0x31));
    let high = u32::from(e.mem.u8(animation + 0x30));
    let block_size = e.mem.u32(animation + 0x2c);
    let minimums = e.mem.u32(animation + 0x38).wrapping_add(data);
    let ranges = e.mem.u32(animation + 0x3c).wrapping_add(data);
    let widths = e.mem.u32(animation + 0x40).wrapping_add(data);
    if block_size == 0 {
        panic!("integer divide fault at 0x00e30746");
    }
    let block = frame / block_size;
    let table = e.mem.u32(animation + 0x54);
    let entry = e
        .mem
        .u32(table.wrapping_add(block.wrapping_mul(4)).wrapping_add(data));
    let stream = entry
        .wrapping_add(e.mem.u32(animation + 0x5c))
        .wrapping_add(data);
    decompress(
        e,
        dest,
        count,
        block_size,
        (high << 8) | low,
        minimums,
        ranges,
        widths,
        stream,
    );
    profile_record(e, PROFILE_END);
}

// Translated from 00e307b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `retrieveFromCache` (Xbox PDB): `EAX` = `holder` (so first; the address of a word that holds the
/// cache object), then the stack words `(animation, frame, count)`. Profiled (`"TtcacheW"`, then the
/// end label). Asks `animation` (slot `+0x40`) for the key of `frame` and takes `bytes = blockSize
/// (+0x2C) * count * 4`. With the cache object (read from `holder` each time): slot `+0x2C`, then
/// slot `+0x28` `(address of a scratch word, key, bytes)`, whose answer is the address of a byte.
/// When that byte is not zero, slot `+0x30` runs and the answer is 0. Otherwise slot `+0xC` `(key,
/// bytes)` is asked for a cached block: when it answers an address, slot `+0x18` `(scratch word
/// address, key, bytes)` and slot `+0x30` run and the answer is that address. When it answers 0,
/// slot `+0x10` `(key, bytes)` is asked for a block to fill: a 0 answer runs slot `+0x30` and
/// answers 0; otherwise slots `+0x18` and `+0x20` `(scratch word address, key, bytes)` and `+0x30`
/// run, the block is filled by `decompressBlockToCache` (`00e306d0`: `frame`, the block, `count`),
/// slots `+0x2C`, `+0x24` `(address of the scratch count word, key, bytes)` and `+0x30` run, and the
/// answer is the block. The scratch words are the exe's stack slots of the three stack arguments,
/// here a 12-byte frame holding `animation`, `frame` and `count`.
pub fn retrieve_from_cache(
    e: &mut Engine,
    holder: u32,
    animation: u32,
    frame: u32,
    count: u32,
) -> u32 {
    profile_record(e, LABEL_CACHE);
    e.with_stack(12, |e, slots| {
        let slot_a = slots.addr();
        let slot_b = slot_a + 4;
        let slot_c = slot_a + 8;
        e.mem.set_u32(slot_a, animation);
        e.mem.set_u32(slot_b, frame);
        e.mem.set_u32(slot_c, count);
        let key = e.vcall(animation, 0x40, &args![e.mem.u32(slot_b)]).u32();
        let bytes = e
            .mem
            .u32(animation + 0x2c)
            .wrapping_mul(e.mem.u32(slot_c))
            .wrapping_mul(4);
        let cache = e.mem.u32(holder);
        e.vcall(cache, 0x2c, &args![]);
        let cache = e.mem.u32(holder);
        let answer = e.vcall(cache, 0x28, &args![slot_a, key, bytes]).u32();
        let cache = e.mem.u32(holder);
        if e.mem.u8(answer) != 0 {
            e.vcall(cache, 0x30, &args![]);
            profile_record(e, PROFILE_END);
            return 0;
        }
        let cached = e.vcall(cache, 0xc, &args![key, bytes]).u32();
        let cache = e.mem.u32(holder);
        let result;
        if cached == 0 {
            let fresh = e.vcall(cache, 0x10, &args![key, bytes]).u32();
            let cache = e.mem.u32(holder);
            if fresh == 0 {
                e.vcall(cache, 0x30, &args![]);
                profile_record(e, PROFILE_END);
                return 0;
            }
            e.vcall(cache, 0x18, &args![slot_a, key, bytes]);
            let cache = e.mem.u32(holder);
            e.vcall(cache, 0x20, &args![slot_a, key, bytes]);
            let cache = e.mem.u32(holder);
            e.vcall(cache, 0x30, &args![]);
            let frame = e.mem.u32(slot_b);
            let count = e.mem.u32(slot_c);
            decompress_block_to_cache(e, animation, frame, fresh, count);
            let cache = e.mem.u32(holder);
            e.vcall(cache, 0x2c, &args![]);
            let cache = e.mem.u32(holder);
            e.vcall(cache, 0x24, &args![slot_c, key, bytes]);
            let cache = e.mem.u32(holder);
            e.vcall(cache, 0x30, &args![]);
            result = fresh;
        } else {
            e.vcall(cache, 0x18, &args![slot_c, key, bytes]);
            let cache = e.mem.u32(holder);
            e.vcall(cache, 0x30, &args![]);
            result = cached;
        }
        profile_record(e, PROFILE_END);
        result
    })
}

/// `samplePartialTracks`' frame: `SUB ESP,0x58`, four pushes, the return address and six
/// argument words.
const SAMPLE_TRACKS_FRAME_SIZE: u32 = 0x84;

// Translated from 00e30980 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaWaveletCompressedAnimation::samplePartialTracks` (Xbox PDB), stack `(time, transform_count,
/// transforms, float_count, floats, cache)`, `RET 0x18`: the sampler that works from the cache (or
/// decompresses the blocks itself), like `samplePartialWithDataChunks` (`00e30200`) does from chunks.
/// Profiled (`"LtSampleWave"` with `"StStaticW"`, `"StDecompressW"`, `"StInterpW"`,
/// `"StRecomposeW"`, the end label). The value count is the animation's (`+0x34`) unless the
/// requested track counts differ from `+0x10` and `+0x14`, when `calcDynamicStats` (`00e4e610`) is
/// asked and the sum of its statistics 7, 8, 9 (and 10 with float tracks) is used. The frame and
/// delta come from `getFrameAndDelta` (the frame goes into the slot that held `time`); `1 - delta` is
/// kept as a `float`. Two buffers (`blockSize * count * 4` and `count * 4` bytes, each rounded up to
/// 128) are taken from the memory router's bump pointer and given back at the end. The first block
/// is taken from the cache (`retrieveFromCache`, `00e307b0`, when `cache` is not null and it
/// answers an address) or decompressed into the first buffer (`decompressBlockToCache`,
/// `00e306d0`); a block that came from the cache is given back with `00e2f910`. When the frame is
/// not the last column of its block each value is `next * delta + this * (1 - delta)`; for the last
/// column the values are `this * (1 - delta)`, then the next block (frame + 1, column 0) is fetched
/// the same way and each value gets `value * delta` added. The result goes through `recompose`
/// (`00e4f640`) with the analysis block `(transform count, float count, words, float words, static
/// values, static float values, result, result floats)`. The x87 arithmetic runs in `f64`, rounded at
/// each `float` store. A zero block size is a divide fault. `cache` is the address of the
/// cache object; the helpers take the address of the stack slot that holds it, its own argument
/// slot.
pub fn hka_wavelet_compressed_animation_sample_partial_tracks(
    e: &mut Engine,
    this: Ptr,
    time: f32,
    transform_count: u32,
    transforms: u32,
    float_count: u32,
    floats: u32,
    cache: u32,
) {
    // The exe's frame (`SUB ESP,0x58` and four pushes, the return address, then the six
    // argument words as the caller pushed them): the statistics and later the analysis block at
    // +0x30.
    e.with_stack(SAMPLE_TRACKS_FRAME_SIZE, |e, locals| {
        let locals = locals.addr();
        for (slot, word) in [
            time.to_bits(),
            transform_count,
            transforms,
            float_count,
            floats,
            cache,
        ]
        .into_iter()
        .enumerate()
        {
            e.mem.set_u32(locals + 0x6c + 4 * slot as u32, word);
        }
        let t = this.addr();
        profile_start_record(e, LABEL_SAMPLE, LABEL_STATIC);
        let words = e.mem.u32(t + 0x44).wrapping_add(e.mem.u32(t + 0x64));
        let tracks = e.mem.u32(t + 0x10);
        let default_count = e.mem.u32(t + 0x34);
        let mut count = default_count;
        if transform_count != tracks || float_count != e.mem.u32(t + 0x14) {
            let float_words = words.wrapping_add(tracks.wrapping_mul(2));
            let stats = locals + 0x30;
            let sum = if float_count == 0 {
                e.call(
                    CALC_DYNAMIC_STATS,
                    &args![words, float_words, transform_count, 0u32, stats],
                );
                e.mem
                    .u32(stats + 0x24)
                    .wrapping_add(e.mem.u32(stats + 0x20))
            } else {
                e.call(
                    CALC_DYNAMIC_STATS,
                    &args![words, float_words, tracks, float_count, stats],
                );
                e.mem
                    .u32(stats + 0x24)
                    .wrapping_add(e.mem.u32(stats + 0x28))
                    .wrapping_add(e.mem.u32(stats + 0x20))
            };
            count = sum.wrapping_add(e.mem.u32(stats + 0x1c));
        }
        profile_record(e, LABEL_DECOMPRESS);
        // The exe's frame slots the callees see: the argument slot that held `time` (+0x6c; the frame
        // goes there), the delta (+0x18) and the argument slot of the cache pointer (+0x80).
        let (frame_slot, delta_slot, holder) = (locals + 0x6c, locals + 0x18, locals + 0x80);
        hka_animation_get_frame_and_delta(e, this, x87_quiet(time), frame_slot, delta_slot);
        let delta = f64::from(x87_quiet(e.mem.f32(delta_slot)));
        let rest_value = (1.0 - delta) as f32;
        let block_size = e.mem.u32(t + 0x2c);
        let table = reserve(e, block_size.wrapping_mul(count).wrapping_mul(4));
        let result = reserve(e, count.wrapping_mul(4));
        let first_frame = e.mem.u32(frame_slot);
        let mut source = table;
        let mut cached = 0;
        if e.mem.u32(holder) != 0 {
            cached = retrieve_from_cache(e, holder, t, first_frame, default_count);
        }
        if cached != 0 {
            source = cached;
        } else {
            decompress_block_to_cache(e, t, first_frame, table, count);
        }
        profile_record(e, LABEL_INTERPOLATE);
        let block_size = e.mem.u32(t + 0x2c);
        let rest = f64::from(x87_quiet(rest_value));
        let frame = e.mem.u32(frame_slot);
        if block_size == 0 {
            panic!("integer divide fault at 0x00e30b7c");
        }
        let column = frame % block_size;
        if column.wrapping_add(1) != block_size {
            let delta = f64::from(x87_quiet(e.mem.f32(delta_slot)));
            let mut index = column;
            for row in 0..count {
                let next = f64::from(x87_quiet(
                    e.mem
                        .f32(source.wrapping_add(index.wrapping_mul(4)).wrapping_add(4)),
                ));
                let this_value = f64::from(x87_quiet(
                    e.mem.f32(source.wrapping_add(index.wrapping_mul(4))),
                ));
                let value = (next * delta + this_value * rest) as f32;
                e.mem
                    .set_f32(result.wrapping_add(row.wrapping_mul(4)), value);
                index = index.wrapping_add(e.mem.u32(t + 0x2c));
            }
            if source != table {
                let frame = e.mem.u32(frame_slot);
                fn_00e2f910(e, t, frame, holder, default_count);
            }
        } else {
            let mut index = column;
            for row in 0..count {
                let value = f64::from(x87_quiet(
                    e.mem.f32(source.wrapping_add(index.wrapping_mul(4))),
                ));
                e.mem.set_f32(
                    result.wrapping_add(row.wrapping_mul(4)),
                    (value * rest) as f32,
                );
                index = index.wrapping_add(e.mem.u32(t + 0x2c));
            }
            if source != table {
                let frame = e.mem.u32(frame_slot);
                fn_00e2f910(e, t, frame, holder, default_count);
            }
            profile_record(e, LABEL_DECOMPRESS);
            let next_frame = e.mem.u32(frame_slot).wrapping_add(1);
            e.mem.set_u32(frame_slot, next_frame);
            let mut second = table;
            let mut cached = 0;
            if e.mem.u32(holder) != 0 {
                cached = retrieve_from_cache(e, holder, t, next_frame, default_count);
            }
            if cached != 0 {
                second = cached;
            } else {
                decompress_block_to_cache(e, t, next_frame, table, count);
            }
            profile_record(e, LABEL_INTERPOLATE);
            let delta = f64::from(x87_quiet(e.mem.f32(delta_slot)));
            let mut index: u32 = 0;
            for row in 0..count {
                let value = f64::from(x87_quiet(
                    e.mem.f32(second.wrapping_add(index.wrapping_mul(4))),
                ));
                let at = result.wrapping_add(row.wrapping_mul(4));
                let sum = f64::from(x87_quiet(e.mem.f32(at)));
                e.mem.set_f32(at, (value * delta + sum) as f32);
                index = index.wrapping_add(e.mem.u32(t + 0x2c));
            }
            if second != table {
                let frame = e.mem.u32(frame_slot);
                fn_00e2f910(e, t, frame, holder, default_count);
            }
        }
        profile_record(e, LABEL_RECOMPOSE);
        let static_values = e.mem.u32(t + 0x48).wrapping_add(e.mem.u32(t + 0x64));
        let analysis = locals + 0x30;
        e.mem.set_u32(analysis, transform_count);
        e.mem.set_u32(analysis + 4, float_count);
        e.mem.set_u32(analysis + 8, words);
        let float_words = words.wrapping_add(e.mem.u32(t + 0x10).wrapping_mul(2));
        e.mem.set_u32(analysis + 0xc, float_words);
        e.mem.set_u32(analysis + 0x10, static_values);
        e.mem.set_u32(
            analysis + 0x14,
            static_values.wrapping_add(e.mem.u32(t + 0x4c).wrapping_mul(4)),
        );
        e.mem.set_u32(analysis + 0x18, result);
        e.mem.set_u32(
            analysis + 0x1c,
            result.wrapping_add(e.mem.u32(t + 0x50).wrapping_mul(4)),
        );
        e.call(RECOMPOSE, &args![analysis, transforms, floats]);
        profile_record(e, LABEL_END);
        release(e, result);
        release(e, table);
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00e2_f7c0,
            hka_animation_get_frame_and_delta(Ptr, f32, u32, u32)
        ),
        entry!(
            0x00e2_f8b0,
            hka_wavelet_compressed_animation_get_num_original_frames(Ptr) -> u32
        ),
        entry!(
            0x00e2_f8c0,
            hka_wavelet_compressed_animation_get_num_data_chunks(Ptr, u32, u32) -> u32
        ),
        entry!(
            0x00e2_f8e0,
            hka_wavelet_compressed_animation_sample_tracks(Ptr, f32, u32, u32, u32) -> u32
        ),
        entry!(0x00e2_f910, fn_00e2f910(u32, u32, u32, u32) -> u32),
        entry!(
            0x00e2_f950,
            hka_wavelet_compressed_animation_sample_individual_float_tracks(
                Ptr, u32, u32, u32, u32
            )
        ),
        entry!(
            0x00e2_f9f0,
            hka_wavelet_compressed_animation_get_full_cache_key(Ptr, u32) -> u32
        ),
        entry!(
            0x00e2_fa00,
            hka_wavelet_compressed_animation_clear_all_cache_keys(Ptr, u32)
        ),
        entry!(0x00e2_fa60, fn_00e2fa60(Ptr, u32, u32)),
        entry!(
            0x00e2_fac0,
            hka_wavelet_compressed_animation_get_data_chunks(Ptr, u32, u32, u32, u32)
        ),
        entry!(
            0x00e2_fb30,
            hka_wavelet_compressed_animation_get_max_size_of_combined_data_chunks(Ptr) -> u32
        ),
        entry!(
            0x00e2_fbc0,
            hka_wavelet_compressed_animation_sample_individual_transform_tracks(
                Ptr, u32, u32, u32, u32
            )
        ),
        entry!(0x00e2_fc10, fn_00e2fc10(Ptr)),
        entry!(
            0x00e3_00b0,
            decompress(u32, u32, u32, u32, u32, u32, u32, u32)
        ),
        entry!(
            0x00e3_0200,
            sample_partial_with_data_chunks(u32, f32, u32, u32, u32, u32, u32, u32)
        ),
        entry!(0x00e3_06d0, decompress_block_to_cache(u32, u32, u32, u32)),
        entry!(0x00e3_07b0, retrieve_from_cache(u32, u32, u32, u32) -> u32),
        entry!(
            0x00e3_0980,
            hka_wavelet_compressed_animation_sample_partial_tracks(
                Ptr, f32, u32, u32, u32, u32, u32
            )
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpsphereshape::{PROFILE_SLOT, TLS_GET_VALUE};
    use std::cell::RefCell;
    use std::rc::Rc;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.call_log = Some(vec![]);
        e
    }

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(called, _)| *called == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// A router double whose bump pointer starts at a fresh block; returns the address of the
    /// pointer.
    fn install_router(e: &mut Engine) -> u32 {
        let router = e.mem.alloc(0x20);
        let base = e.mem.alloc(0x400);
        e.mem.set_u32(router, base);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        router
    }

    /// A profiling stream with room for 4 sixteen-byte records.
    fn install_stream(e: &mut Engine) -> u32 {
        let records = e.mem.alloc(0x40);
        let stream = e.mem.alloc(0x10);
        e.mem.set_u32(stream + 4, records);
        e.mem.set_u32(stream + 0xc, records + 0x40);
        e.set_global(PROFILE_SLOT, 5u32);
        e.register_double(TLS_GET_VALUE, move |_, _| Ret {
            eax: stream,
            ..Ret::default()
        });
        stream
    }

    /// An animation with block size 4, 2 transform tracks and 1 float track, data base `0x6000`
    /// (plus a unit offset of 3 in the chunk struct) and the given dynamic value count.
    fn setup(e: &mut Engine) -> (u32, u32) {
        let anim = e.mem.alloc(0x80);
        for (offset, value) in [
            (0x10, 2),
            (0x14, 1),
            (0x2c, 4),
            (0x34, 3),
            (0x38, 0x100),
            (0x3c, 0x200),
            (0x40, 0x300),
            (0x44, 0x400),
            (0x48, 0x500),
            (0x4c, 5),
            (0x50, 6),
        ] {
            e.mem.set_u32(anim + offset, value);
        }
        e.mem.set_u8(anim + 0x30, 0x12);
        e.mem.set_u8(anim + 0x31, 0x34);
        let chunks = e.mem.alloc(0x40);
        e.mem.set_u32(chunks, anim - 2);
        e.mem.set_u8(chunks + 8, 2);
        e.mem.set_u32(chunks + 0xc, 0x6000 - 3);
        e.mem.set_u8(chunks + 0x14, 3);
        e.mem.set_u32(chunks + 0x18, 0x7000 - 5);
        e.mem.set_u8(chunks + 0x20, 5);
        e.mem.set_u32(chunks + 0x24, 0x8000 - 7);
        e.mem.set_u8(chunks + 0x2c, 7);
        (anim, chunks)
    }

    /// Doubles for the callees: `_decompress` fills the table with `row * 10 + column + pass`
    /// where `pass` counts its calls, and `recompose` and the stats function only log.
    fn install_callees(e: &mut Engine) -> Rc<RefCell<Vec<Vec<u32>>>> {
        let passes = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&passes);
        e.register_double(DECOMPRESS, move |e, a| {
            let pass = log.borrow().len() as f32;
            log.borrow_mut().push(a.to_vec());
            let (table, count, block_size) = (a[0], a[1], a[2]);
            for row in 0..count {
                for column in 0..block_size {
                    e.mem.set_f32(
                        table + 4 * (row * block_size + column),
                        (row * 10 + column) as f32 + 100.0 * pass,
                    );
                }
            }
            Ret::default()
        });
        e.register_double(CALC_DYNAMIC_STATS, |e, a| {
            for (offset, value) in [(0x1c, 1u32), (0x20, 2), (0x24, 3), (0x28, 4)] {
                e.mem.set_u32(a[4] + offset, value);
            }
            Ret::default()
        });
        e.register_double(RECOMPOSE, |_, _| Ret::default());
        passes
    }

    /// The words of the analysis block and the 6 result floats `recompose` was given.
    type Recomposed = Rc<RefCell<Vec<(Vec<u32>, Vec<f32>)>>>;

    fn record_recompose(e: &mut Engine) -> Recomposed {
        let seen: Recomposed = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        e.register_double(RECOMPOSE, move |e, a| {
            let words = (0..8).map(|i| e.mem.u32(a[0] + 4 * i)).collect();
            let result = e.mem.u32(a[0] + 0x18);
            let floats = (0..6).map(|i| e.mem.f32(result + 4 * i)).collect();
            sink.borrow_mut().push((words, floats));
            Ret::default()
        });
        seen
    }

    /// The labels of the records in a stream whose first record is 16 bytes long.
    fn labels(e: &Engine, stream: u32) -> Vec<u32> {
        let first = e.mem.u32(stream + 0xc) - 0x40;
        let cursor = e.mem.u32(stream + 4);
        let mut labels = vec![e.mem.u32(first), e.mem.u32(first + 0xc)];
        let mut at = first + 0x10;
        while at < cursor {
            labels.push(e.mem.u32(at));
            at += 12;
        }
        labels
    }

    #[test]
    fn three_chunks_blend_two_neighbouring_columns() {
        let mut e = engine();
        let router = install_router(&mut e);
        let base = e.mem.u32(router);
        let stream = install_stream(&mut e);
        let passes = install_callees(&mut e);
        let seen = record_recompose(&mut e);
        let (_, chunks) = setup(&mut e);
        let transforms = e.mem.alloc(0x20);
        let floats = e.mem.alloc(0x20);
        // Matching track counts keep the animation's value count (3). Column 6 % 4 = 2: row r
        // blends r * 10 + 2 and r * 10 + 3 with t = 0.25.
        e.call(
            0x00e3_0200,
            &args![6u32, 0.25f32, 2u32, transforms, 1u32, floats, chunks, 3u32],
        );
        assert!(calls(&e, CALC_DYNAMIC_STATS).is_empty());
        assert_eq!(
            *passes.borrow(),
            vec![vec![base, 3, 4, 0x1234, 0x6100, 0x6200, 0x6300, 0x7000]]
        );
        assert_eq!(e.mem.u32(router), base);
        let (words, result) = seen.borrow()[0].clone();
        assert_eq!(
            words,
            vec![
                2,
                1,
                0x6400,
                0x6404,
                0x6500,
                0x6514,
                base + 0x80,
                base + 0x98
            ]
        );
        assert_eq!(result[..3], [2.25, 12.25, 22.25]);
        assert_eq!(calls(&e, RECOMPOSE)[0][1..], [transforms, floats]);
        assert_eq!(
            labels(&e, stream),
            vec![
                LABEL_SAMPLE_CHUNK,
                LABEL_STATIC,
                LABEL_DECOMPRESS_CHUNK,
                LABEL_RECOMPOSE_CHUNK,
                LABEL_END
            ]
        );
    }

    #[test]
    fn four_chunks_add_the_second_block_and_other_counts_use_the_stats() {
        let mut e = engine();
        let router = install_router(&mut e);
        let base = e.mem.u32(router);
        install_stream(&mut e);
        let passes = install_callees(&mut e);
        let seen = record_recompose(&mut e);
        let (_, chunks) = setup(&mut e);
        let transforms = e.mem.alloc(0x20);
        let floats = e.mem.alloc(0x20);
        // Three transform tracks and no float tracks differ from the animation's 2 and 1: the
        // stats double gives 1 + 2 + 3 = 6 values. Column 5 % 4 = 1.
        e.call(
            0x00e3_0200,
            &args![5u32, 0.25f32, 3u32, transforms, 0u32, floats, chunks, 4u32],
        );
        assert_eq!(calls(&e, CALC_DYNAMIC_STATS)[0][1..4], [0x6400 + 4, 3, 0]);
        assert_eq!(passes.borrow().len(), 2);
        assert_eq!(passes.borrow()[0][1], 6);
        assert_eq!(passes.borrow()[0][7], 0x7000);
        assert_eq!(passes.borrow()[1][7], 0x8000);
        assert_eq!(e.mem.u32(router), base);
        let (_, result) = seen.borrow()[0].clone();
        for (row, got) in result.iter().enumerate() {
            let row = row as f32;
            let want = (row * 10.0 + 1.0) * 0.75 + (row * 10.0 + 100.0) * 0.25;
            assert_eq!(*got, want);
        }
    }

    #[test]
    #[should_panic(expected = "integer divide fault at 0x00e30404")]
    fn a_zero_block_size_faults() {
        let mut e = engine();
        install_router(&mut e);
        install_stream(&mut e);
        install_callees(&mut e);
        let (anim, chunks) = setup(&mut e);
        e.mem.set_u32(anim + 0x2c, 0);
        e.call(
            0x00e3_0200,
            &args![6u32, 0.25f32, 2u32, 0u32, 1u32, 0u32, chunks, 3u32],
        );
    }

    /// A double that answers `value` in `EAX`.
    fn answer(e: &mut Engine, address: u32, value: u32) {
        e.register_double(address, move |_, _| Ret {
            eax: value,
            ..Ret::default()
        });
    }

    /// An object whose vtable holds `(slot offset, target)` pairs.
    fn fake_object(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x80);
        for (offset, target) in slots {
            e.mem.set_u32(vtable + offset, *target);
        }
        let object = e.mem.alloc(0x80);
        e.mem.set_u32(object, vtable);
        object
    }

    /// The addresses of the logged calls, in order, that are in `wanted`.
    fn sequence(e: &Engine, wanted: &[u32]) -> Vec<u32> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|(address, _)| *address)
            .filter(|address| wanted.contains(address))
            .collect()
    }

    const FRAME_COUNT_SLOT: u32 = 0x0900_0020;
    const CACHE_KEY_SLOT: u32 = 0x0900_0040;
    const SAMPLE_SLOT: u32 = 0x0900_0010;

    #[test]
    fn the_frame_and_delta_split_the_scaled_time() {
        let mut e = engine();
        let animation = fake_object(&mut e, &[(0x20, FRAME_COUNT_SLOT)]);
        answer(&mut e, FRAME_COUNT_SLOT, 11);
        e.mem.set_f32(animation + 0xc, 2.0);
        let out = e.mem.alloc(8);
        // 0.5 / 2 * 10 = 2.5.
        e.call(0x00e2_f7c0, &args![animation, 0.5f32, out, out + 4]);
        assert_eq!(e.mem.u32(out), 2);
        assert_eq!(e.mem.f32(out + 4), 0.5);
        // 1.0 / 2 * 10 = 5 exactly: no delta.
        e.call(0x00e2_f7c0, &args![animation, 1.0f32, out, out + 4]);
        assert_eq!(e.mem.u32(out), 5);
        assert_eq!(e.mem.f32(out + 4), 0.0);
        // A negative time truncates toward zero (-2) and wraps as a dword, which is above the
        // limit: the frame is clamped to 9 and the delta is 1.
        e.call(0x00e2_f7c0, &args![animation, -0.5f32, out, out + 4]);
        assert_eq!(e.mem.u32(out), 9);
        assert_eq!(e.mem.f32(out + 4), 1.0);
    }

    #[test]
    fn a_time_beyond_the_last_frame_clamps_to_the_frame_before_it() {
        let mut e = engine();
        let animation = fake_object(&mut e, &[(0x20, FRAME_COUNT_SLOT)]);
        answer(&mut e, FRAME_COUNT_SLOT, 11);
        e.mem.set_f32(animation + 0xc, 2.0);
        let out = e.mem.alloc(8);
        // 4.0 / 2 * 10 = 20 > 9.
        e.call(0x00e2_f7c0, &args![animation, 4.0f32, out, out + 4]);
        assert_eq!(e.mem.u32(out), 9);
        assert_eq!(e.mem.f32(out + 4), 1.0);
        // 1.8 / 2 * 10 rounds to 9.0 as a float: frame 9 is the limit, so it is not clamped and
        // its delta is 0.
        e.call(0x00e2_f7c0, &args![animation, 1.8f32, out, out + 4]);
        assert_eq!(e.mem.u32(out), 9);
        assert_eq!(e.mem.f32(out + 4), 0.0);
    }

    #[test]
    fn a_nan_time_gives_frame_zero_and_a_nan_delta() {
        let mut e = engine();
        let animation = fake_object(&mut e, &[(0x20, FRAME_COUNT_SLOT)]);
        answer(&mut e, FRAME_COUNT_SLOT, 11);
        e.mem.set_f32(animation + 0xc, 2.0);
        let out = e.mem.alloc(8);
        e.call(
            0x00e2_f7c0,
            &args![animation, f32::from_bits(0x7fc0_0000), out, out + 4],
        );
        assert_eq!(e.mem.u32(out), 0);
        assert!(e.mem.f32(out + 4).is_nan());
    }

    #[test]
    fn an_unsigned_frame_count_below_two_wraps_the_clamp_limit() {
        let mut e = engine();
        let animation = fake_object(&mut e, &[(0x20, FRAME_COUNT_SLOT)]);
        answer(&mut e, FRAME_COUNT_SLOT, 1);
        e.mem.set_f32(animation + 0xc, 1.0);
        let out = e.mem.alloc(8);
        // One frame: n - 1 = 0, so the position is 0; the limit n - 2 wraps to 0xffffffff.
        e.call(0x00e2_f7c0, &args![animation, 0.5f32, out, out + 4]);
        assert_eq!(e.mem.u32(out), 0);
        assert_eq!(e.mem.f32(out + 4), 0.0);
        // No frames: n - 1 = 0xffffffff counts as 2^32 - 1, but time 0 still gives position 0.
        answer(&mut e, FRAME_COUNT_SLOT, 0);
        e.call(0x00e2_f7c0, &args![animation, 0.0f32, out, out + 4]);
        assert_eq!(e.mem.u32(out), 0);
        assert_eq!(e.mem.f32(out + 4), 0.0);
    }

    #[test]
    fn the_original_frame_count_is_the_word_at_0x28() {
        let mut e = engine();
        let animation = e.mem.alloc(0x80);
        e.mem.set_u32(animation + 0x28, 77);
        assert_eq!(e.call(0x00e2_f8b0, &args![animation]).u32(), 77);
    }

    #[test]
    fn the_last_frame_of_a_block_needs_four_data_chunks() {
        let mut e = engine();
        let animation = e.mem.alloc(0x80);
        e.mem.set_u32(animation + 0x2c, 4);
        assert_eq!(e.call(0x00e2_f8c0, &args![animation, 3u32, 0u32]).u32(), 4);
        assert_eq!(e.call(0x00e2_f8c0, &args![animation, 7u32, 0u32]).u32(), 4);
        assert_eq!(e.call(0x00e2_f8c0, &args![animation, 2u32, 0u32]).u32(), 3);
        assert_eq!(e.call(0x00e2_f8c0, &args![animation, 4u32, 9u32]).u32(), 3);
        e.mem.set_u32(animation + 0x2c, 1);
        assert_eq!(e.call(0x00e2_f8c0, &args![animation, 5u32, 0u32]).u32(), 4);
    }

    #[test]
    #[should_panic(expected = "integer divide fault at 0x00e2f8c9")]
    fn a_zero_block_size_faults_when_counting_chunks() {
        let mut e = engine();
        let animation = e.mem.alloc(0x80);
        e.call(0x00e2_f8c0, &args![animation, 3u32, 0u32]);
    }

    #[test]
    fn sample_tracks_forwards_to_sample_partial_tracks_with_the_track_counts() {
        let mut e = engine();
        let animation = fake_object(&mut e, &[(0x10, SAMPLE_SLOT)]);
        e.mem.set_u32(animation + 0x10, 2);
        e.mem.set_u32(animation + 0x14, 1);
        answer(&mut e, SAMPLE_SLOT, 0x4242);
        let got = e
            .call(
                0x00e2_f8e0,
                &args![
                    animation,
                    f32::from_bits(0x7f80_0001),
                    0x100u32,
                    0x200u32,
                    0x300u32
                ],
            )
            .u32();
        assert_eq!(got, 0x4242);
        assert_eq!(
            calls(&e, SAMPLE_SLOT),
            vec![vec![animation, 0x7fc0_0001, 2, 0x100, 1, 0x200, 0x300]]
        );
    }

    /// An animation whose key getter is `CACHE_KEY_SLOT` and a cache object with the slots
    /// `0x14` to `0x30` as fake targets `0x0900_0500 + slot`.
    fn cache_setup(e: &mut Engine) -> (u32, u32, u32) {
        let animation = fake_object(e, &[(0x20, FRAME_COUNT_SLOT), (0x40, CACHE_KEY_SLOT)]);
        e.mem.set_u32(animation + 0x2c, 4);
        let slots: Vec<(u32, u32)> = (0x0c..=0x30)
            .step_by(4)
            .map(|slot| (slot, 0x0900_0500 + slot))
            .collect();
        let cache = fake_object(e, &slots);
        let holder = e.mem.alloc(4);
        e.mem.set_u32(holder, cache);
        for (_, target) in slots {
            answer(e, target, 0);
        }
        (animation, cache, holder)
    }

    #[test]
    fn the_cache_block_is_released_through_the_cache_objects_slots() {
        let mut e = engine();
        let (animation, cache, holder) = cache_setup(&mut e);
        answer(&mut e, CACHE_KEY_SLOT, 0x77);
        e.register_double(0x0900_0500 + 0x30, |_, _| Ret {
            eax: 0x99,
            ..Ret::default()
        });
        let seen = Rc::new(RefCell::new(0u32));
        let slot_value = Rc::clone(&seen);
        e.register_double(0x0900_0500 + 0x1c, move |e, a| {
            *slot_value.borrow_mut() = e.mem.u32(a[1]);
            Ret::default()
        });
        // frame 9, count 5: bytes = 4 * 5 * 4.
        let got = e
            .call(0x00e2_f910, &args![animation, 9u32, holder, 5u32])
            .u32();
        assert_eq!(got, 0x99);
        assert_eq!(calls(&e, CACHE_KEY_SLOT), vec![vec![animation, 9]]);
        assert_eq!(
            sequence(&e, &[0x0900_052c, 0x0900_051c, 0x0900_0530]),
            vec![0x0900_052c, 0x0900_051c, 0x0900_0530]
        );
        let release = calls(&e, 0x0900_051c);
        assert_eq!(release[0][0], cache);
        assert_eq!(release[0][2..], [0x77, 80]);
        // The scratch word the cache was given holds the count.
        assert_eq!(*seen.borrow(), 5);
    }

    #[test]
    fn individual_float_tracks_are_zeroed() {
        let mut e = engine();
        let out = e.mem.alloc(32);
        for word in 0..8 {
            e.mem.set_u32(out + 4 * word, 0xdead_beef);
        }
        e.call(0x00e2_f950, &args![0u32, 1u32, 2u32, 3u32, out]);
        for word in 0..3 {
            assert_eq!(e.mem.u32(out + 4 * word), 0);
        }
        assert_eq!(e.mem.u32(out + 12), 0xdead_beef);
        // A count of zero stores nothing.
        e.call(0x00e2_f950, &args![0u32, 1u32, 2u32, 0u32, out + 12]);
        assert_eq!(e.mem.u32(out + 12), 0xdead_beef);
    }

    #[test]
    fn the_full_cache_key_is_the_block_number_plus_the_data_word() {
        let mut e = engine();
        let animation = e.mem.alloc(0x80);
        e.mem.set_u32(animation + 0x2c, 4);
        e.mem.set_u32(animation + 0x64, 100);
        assert_eq!(e.call(0x00e2_f9f0, &args![animation, 10u32]).u32(), 102);
        assert_eq!(e.call(0x00e2_f9f0, &args![animation, 3u32]).u32(), 100);
    }

    #[test]
    #[should_panic(expected = "integer divide fault at 0x00e2f9f6")]
    fn a_zero_block_size_faults_when_asking_the_cache_key() {
        let mut e = engine();
        let animation = e.mem.alloc(0x80);
        e.call(0x00e2_f9f0, &args![animation, 10u32]);
    }

    #[test]
    fn clearing_the_cache_keys_visits_each_block_once() {
        let mut e = engine();
        let (animation, cache, _) = cache_setup(&mut e);
        answer(&mut e, FRAME_COUNT_SLOT, 10);
        e.mem.set_u32(animation + 0x34, 3);
        e.mem.set_u32(animation + 0x64, 50);
        e.call(0x00e2_fa00, &args![animation, cache]);
        // (10 - 1) / 4 + 1 = 3 blocks of 3 * 4 * 4 bytes; the scratch byte is the same each time.
        let clears = calls(&e, 0x0900_0514);
        assert_eq!(clears.len(), 3);
        assert_eq!(
            clears
                .iter()
                .map(|c| (c[0], c[2], c[3]))
                .collect::<Vec<_>>(),
            vec![(cache, 50, 48), (cache, 51, 48), (cache, 52, 48)]
        );
        assert_eq!(clears[0][1], clears[2][1]);
        // A single frame is one block, no frames also one (0 / 4 + 1).
        answer(&mut e, FRAME_COUNT_SLOT, 1);
        e.call_log = Some(vec![]);
        e.call(0x00e2_fa00, &args![animation, cache]);
        assert_eq!(calls(&e, 0x0900_0514).len(), 1);
        answer(&mut e, FRAME_COUNT_SLOT, 0);
        e.call_log = Some(vec![]);
        e.call(0x00e2_fa00, &args![animation, cache]);
        assert_eq!(calls(&e, 0x0900_0514).len(), 1);
        // A negative block size gives a non-positive block count: nothing is cleared.
        answer(&mut e, FRAME_COUNT_SLOT, 10);
        e.mem.set_u32(animation + 0x2c, 0xffff_ffff);
        e.call_log = Some(vec![]);
        e.call(0x00e2_fa00, &args![animation, cache]);
        assert!(calls(&e, 0x0900_0514).is_empty());
    }

    #[test]
    #[should_panic(expected = "integer divide fault at 0x00e2fa13")]
    fn clearing_the_cache_keys_of_a_zero_block_size_faults() {
        let mut e = engine();
        let (animation, cache, _) = cache_setup(&mut e);
        answer(&mut e, FRAME_COUNT_SLOT, 10);
        e.mem.set_u32(animation + 0x2c, 0);
        e.call(0x00e2_fa00, &args![animation, cache]);
    }

    /// An animation with three blocks: the quantised data at `+0x5c` (offset 0x20 into the data
    /// at `+0x64` = `data`), the block table at `+0x54` (offset 0x100: entries 0x10, 0x35, 0x62),
    /// the block count at `+0x58` and the end of the data at `+0x60` = 0x90.
    fn block_animation(e: &mut Engine) -> (u32, u32) {
        let animation = e.mem.alloc(0x80);
        let data = e.mem.alloc(0x400);
        e.mem.set_u32(animation + 0x64, data);
        e.mem.set_u32(animation + 0x5c, 0x20);
        e.mem.set_u32(animation + 0x54, 0x100);
        e.mem.set_u32(animation + 0x58, 3);
        e.mem.set_u32(animation + 0x60, 0x90);
        e.mem.set_u32(animation + 0x2c, 4);
        for (index, entry) in [0x10u32, 0x35, 0x62].into_iter().enumerate() {
            e.mem.set_u32(data + 0x100 + 4 * index as u32, entry);
        }
        (animation, data)
    }

    #[test]
    fn a_block_is_described_by_its_aligned_address_misalignment_and_rounded_size() {
        let mut e = engine();
        let (animation, data) = block_animation(&mut e);
        let out = e.mem.alloc(16);
        // Block 1: address data + 0x20 + 0x35, size 0x62 - 0x35 = 0x2d.
        e.call(0x00e2_fa60, &args![animation, 1u32, out]);
        let address = data + 0x20 + 0x35;
        assert_eq!(e.mem.u32(out), address & !0xf);
        assert_eq!(e.mem.u8(out + 8), (address & 0xf) as u8);
        assert_eq!(e.mem.u32(out + 4), ((address & 0xf) + 0x2d + 0xf) & !0xf);
        // The last block ends at the word at +0x60: 0x90 - 0x62 = 0x2e.
        e.call(0x00e2_fa60, &args![animation, 2u32, out]);
        let address = data + 0x20 + 0x62;
        assert_eq!(e.mem.u32(out), address & !0xf);
        assert_eq!(e.mem.u8(out + 8), (address & 0xf) as u8);
        assert_eq!(e.mem.u32(out + 4), ((address & 0xf) + 0x2e + 0xf) & !0xf);
    }

    #[test]
    fn the_data_chunks_describe_the_animation_the_data_and_one_or_two_blocks() {
        let mut e = engine();
        let (animation, data) = block_animation(&mut e);
        let out = e.mem.alloc(0x40);
        // Frame 9 is in block 2; with four chunks the next block wraps to block 0.
        e.call(0x00e2_fac0, &args![animation, 9u32, 0u32, out, 4u32]);
        assert_eq!(e.mem.u32(out), animation);
        assert_eq!(e.mem.u32(out + 4), 0x70);
        assert_eq!(e.mem.u8(out + 8), (animation & 0xf) as u8);
        assert_eq!(e.mem.u32(out + 0xc), data);
        assert_eq!(e.mem.u32(out + 0x10), (0x20 + 0xf) & !0xf);
        assert_eq!(e.mem.u8(out + 0x14), (data & 0xf) as u8);
        let block_two = data + 0x20 + 0x62;
        assert_eq!(e.mem.u32(out + 0x18), block_two & !0xf);
        let block_zero = data + 0x20 + 0x10;
        assert_eq!(e.mem.u32(out + 0x24), block_zero & !0xf);
        assert_eq!(e.mem.u8(out + 0x2c), (block_zero & 0xf) as u8);
        // Three chunks leave the fourth description alone.
        let other = e.mem.alloc(0x40);
        e.call(0x00e2_fac0, &args![animation, 5u32, 0u32, other, 3u32]);
        assert_eq!(e.mem.u32(other + 0x18), (data + 0x20 + 0x35) & !0xf);
        assert_eq!(e.mem.u32(other + 0x24), 0);
    }

    #[test]
    #[should_panic(expected = "integer divide fault at 0x00e2fafa")]
    fn data_chunks_of_a_zero_block_size_fault() {
        let mut e = engine();
        let (animation, _) = block_animation(&mut e);
        e.mem.set_u32(animation + 0x2c, 0);
        let out = e.mem.alloc(0x40);
        e.call(0x00e2_fac0, &args![animation, 9u32, 0u32, out, 3u32]);
    }

    #[test]
    #[should_panic(expected = "integer divide fault at 0x00e2fb17")]
    fn the_next_data_chunk_of_a_zero_block_count_faults() {
        let mut e = engine();
        let (animation, _) = block_animation(&mut e);
        e.mem.set_u32(animation + 0x58, 0);
        let out = e.mem.alloc(0x40);
        e.call(0x00e2_fac0, &args![animation, 9u32, 0u32, out, 4u32]);
    }

    #[test]
    fn the_largest_combined_size_covers_each_block_with_its_successor() {
        let mut e = engine();
        let (animation, data) = block_animation(&mut e);
        let size = |address: u32, length: u32| ((address & 0xf) + length + 0xf) & !0xf;
        let blocks = [
            size(data + 0x20 + 0x10, 0x35 - 0x10),
            size(data + 0x20 + 0x35, 0x62 - 0x35),
            size(data + 0x20 + 0x62, 0x90 - 0x62),
        ];
        let fixed = ((0x20 + 0xf) & !0xf) + 0x70;
        let want = [
            fixed + blocks[0] + blocks[1],
            fixed + blocks[1] + blocks[2],
            fixed + blocks[2],
        ]
        .into_iter()
        .max()
        .unwrap();
        assert_eq!(e.call(0x00e2_fb30, &args![animation]).u32(), want);
        e.mem.set_u32(animation + 0x58, 0);
        assert_eq!(e.call(0x00e2_fb30, &args![animation]).u32(), 0);
    }

    #[test]
    fn individual_transform_tracks_are_identity_transforms() {
        let mut e = engine();
        for (lane, bits) in [0x3f80_0000u32, 0, 0x4000_0000, 0x3f00_0000]
            .into_iter()
            .enumerate()
        {
            e.mem.set_u32(IDENTITY_ROTATION + 4 * lane as u32, bits);
        }
        e.mem.set_u32(ONE, 0x3f80_0000);
        let out = e.mem.alloc(0x80);
        for word in 0..32 {
            e.mem.set_u32(out + 4 * word, 0xdead_beef);
        }
        e.call(0x00e2_fbc0, &args![0u32, 1u32, 2u32, 2u32, out]);
        for transform in 0..2 {
            let at = out + 0x30 * transform;
            assert_eq!(
                (0..4)
                    .map(|lane| e.mem.u32(at + 4 * lane))
                    .collect::<Vec<_>>(),
                vec![0, 0, 0, 0]
            );
            assert_eq!(
                (0..4)
                    .map(|lane| e.mem.u32(at + 0x10 + 4 * lane))
                    .collect::<Vec<_>>(),
                vec![0x3f80_0000, 0, 0x4000_0000, 0x3f00_0000]
            );
            assert_eq!(
                (0..4)
                    .map(|lane| e.mem.u32(at + 0x20 + 4 * lane))
                    .collect::<Vec<_>>(),
                vec![0x3f80_0000; 4]
            );
        }
        assert_eq!(e.mem.u32(out + 0x60), 0xdead_beef);
        e.call(0x00e2_fbc0, &args![0u32, 1u32, 2u32, 0u32, out + 0x60]);
        assert_eq!(e.mem.u32(out + 0x60), 0xdead_beef);
    }

    /// An animation for the endian pass: type 4, data starting with a non-zero byte, three
    /// halfwords (2 + 1) at `data + 4`, the float-typed arrays at offsets 0x40 (`+0x48`), 0x60
    /// (`+0x38`) and 0x80 (`+0x3C`) and a two entry block table at 0xa0 (`+0x54`).
    fn endian_animation(e: &mut Engine) -> (u32, u32) {
        let animation = e.mem.alloc(0x80);
        let data = e.mem.alloc(0x200);
        e.mem.set_u32(animation + 0x44, 4);
        e.mem.set_u32(animation + 0x64, data);
        e.mem.set_u8(data, 1);
        e.mem.set_u32(animation + 0x10, 2);
        e.mem.set_u32(animation + 0x14, 1);
        e.mem.set_u32(animation + 0x48, 0x40);
        e.mem.set_u32(animation + 0x38, 0x60);
        e.mem.set_u32(animation + 0x3c, 0x80);
        e.mem.set_u32(animation + 0x54, 0xa0);
        e.mem.set_u32(animation + 0x58, 2);
        for (at, value) in [(4u32, 0x1234u16), (6, 0xabcd), (8, 0x0102)] {
            e.mem.set_u16(data + at, value);
        }
        for (at, value) in [
            (0x40u32, 0x1122_3344u32),
            (0x44, 0x7f80_0001),
            (0x48, 0x5555_5555),
            (0x60, 0x0102_0304),
            (0x64, 0x0a0b_0c0d),
            (0x68, 0x7777_7777),
            (0x80, 0x1111_1111),
            (0x84, 0x2233_4455),
            (0x88, 0x7777_7777),
            (0xa0, 0xaabb_ccdd),
            (0xa4, 0x1020_3040),
            (0xa8, 0x7777_7777),
        ] {
            e.mem.set_u32(data + at, value);
        }
        (animation, data)
    }

    /// Statistics as the analysis double writes them: the words at `a[4]`, indexes 0 to 10.
    fn stats_double(e: &mut Engine, stats: [u32; 11]) {
        e.register_double(ANALYSE_ENDIAN, move |e, a| {
            for (index, value) in stats.into_iter().enumerate() {
                e.mem.set_u32(a[4] + 4 * index as u32, value);
            }
            Ret::default()
        });
    }

    #[test]
    fn the_endian_pass_reverses_halfwords_and_the_counted_dwords() {
        let mut e = engine();
        let (animation, data) = endian_animation(&mut e);
        // n1 = 1 + 1 (statistics 0..3), n2 = 1 + 1 (statistics 7..10).
        stats_double(&mut e, [1, 1, 0, 0, 0, 0, 0, 1, 0, 1, 0]);
        e.call(0x00e2_fc10, &args![animation]);
        assert_eq!(e.mem.u16(data + 4), 0x3412);
        assert_eq!(e.mem.u16(data + 6), 0xcdab);
        assert_eq!(e.mem.u16(data + 8), 0x0201);
        let analysis = calls(&e, ANALYSE_ENDIAN);
        assert_eq!(analysis.len(), 1);
        assert_eq!(analysis[0][..4], [data + 4, data + 4 + 4, 2, 1]);
        // The first array: two dwords; the signalling NaN is quieted, then swapped.
        assert_eq!(e.mem.u32(data + 0x40), 0x4433_2211);
        assert_eq!(e.mem.u32(data + 0x44), 0x0100_c07f);
        assert_eq!(e.mem.u32(data + 0x48), 0x5555_5555);
        // The other two: two dwords each.
        assert_eq!(e.mem.u32(data + 0x60), 0x0403_0201);
        assert_eq!(e.mem.u32(data + 0x64), 0x0d0c_0b0a);
        assert_eq!(e.mem.u32(data + 0x68), 0x7777_7777);
        assert_eq!(e.mem.u32(data + 0x80), 0x1111_1111);
        assert_eq!(e.mem.u32(data + 0x84), 0x5544_3322);
        assert_eq!(e.mem.u32(data + 0x88), 0x7777_7777);
        // The block table: two dwords, plain byte swaps.
        assert_eq!(e.mem.u32(data + 0xa0), 0xddcc_bbaa);
        assert_eq!(e.mem.u32(data + 0xa4), 0x4030_2010);
        assert_eq!(e.mem.u32(data + 0xa8), 0x7777_7777);
    }

    #[test]
    fn the_endian_pass_swaps_nothing_for_other_types_or_empty_data() {
        let mut e = engine();
        let (animation, data) = endian_animation(&mut e);
        stats_double(&mut e, [1, 1, 0, 0, 0, 0, 0, 1, 0, 1, 0]);
        e.mem.set_u32(animation + 0x44, 3);
        e.call(0x00e2_fc10, &args![animation]);
        assert_eq!(e.mem.u16(data + 4), 0x1234);
        assert!(calls(&e, ANALYSE_ENDIAN).is_empty());
        e.mem.set_u32(animation + 0x44, 4);
        e.mem.set_u8(data, 0);
        e.call(0x00e2_fc10, &args![animation]);
        assert_eq!(e.mem.u16(data + 4), 0x1234);
        assert!(calls(&e, ANALYSE_ENDIAN).is_empty());
    }

    #[test]
    fn a_non_positive_first_count_swaps_no_dwords_and_zero_halfwords_none() {
        let mut e = engine();
        let (animation, data) = endian_animation(&mut e);
        e.mem.set_u32(animation + 0x10, 0);
        e.mem.set_u32(animation + 0x14, 0);
        e.mem.set_u32(animation + 0x58, 0);
        // Statistics 0..3 add up to -1: nothing at the first array; 7..10 add up to 0.
        stats_double(&mut e, [0xffff_ffff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        e.call(0x00e2_fc10, &args![animation]);
        assert_eq!(e.mem.u16(data + 4), 0x1234);
        assert_eq!(e.mem.u32(data + 0x40), 0x1122_3344);
        assert_eq!(e.mem.u32(data + 0x60), 0x0102_0304);
        assert_eq!(e.mem.u32(data + 0xa0), 0xaabb_ccdd);
    }

    /// Doubles for the callees of `_decompress` that record their arguments (and the parameter
    /// block and width bytes as they were at the call). Returns the log: tag, words.
    fn decompress_doubles(e: &mut Engine) -> Rc<RefCell<Vec<(&'static str, Vec<u32>)>>> {
        let log: Rc<RefCell<Vec<(&'static str, Vec<u32>)>>> = Rc::new(RefCell::new(Vec::new()));
        let sizes = Rc::clone(&log);
        e.register_double(CALC_QUANTIZED_SIZE, move |e, a| {
            let width = u32::from(e.mem.u8(a[1]));
            let other = u32::from(e.mem.u8(a[1] + 1));
            sizes.borrow_mut().push(("size", vec![a[0], width, other]));
            Ret {
                eax: 20,
                ..Ret::default()
            }
        });
        let decodes = Rc::clone(&log);
        e.register_double(BLOCK_DECODE, move |e, a| {
            let (width, quantised) = (e.mem.u32(a[1]), e.mem.u32(a[1] + 4));
            decodes
                .borrow_mut()
                .push(("decode", vec![a[0], width, quantised, a[3]]));
            Ret {
                eax: 10,
                ..Ret::default()
            }
        });
        let expands = Rc::clone(&log);
        e.register_double(EXPAND_REAL, move |e, a| {
            let width = u32::from(e.mem.u8(a[1]));
            expands
                .borrow_mut()
                .push(("expand", vec![a[0], width, a[2], a[3]]));
            Ret::default()
        });
        let passes = Rc::clone(&log);
        e.register_double(ROW_PASS, move |_, a| {
            passes.borrow_mut().push(("pass", a.to_vec()));
            Ret::default()
        });
        log
    }

    #[test]
    fn decompress_quantises_each_row_with_the_default_width() {
        let mut e = engine();
        let router = install_router(&mut e);
        let base = e.mem.u32(router);
        let log = decompress_doubles(&mut e);
        let minimums = e.mem.alloc(8);
        let ranges = e.mem.alloc(8);
        e.mem.set_f32(minimums, -0.5);
        e.mem.set_f32(minimums + 4, 2.0);
        e.mem.set_f32(ranges, 1.0);
        e.mem.set_f32(ranges + 4, 4.0);
        let dest = 0x0a00_0000u32;
        // Word 0x0805: default width 8 (second byte), first byte 5.
        e.call(
            0x00e3_00b0,
            &args![dest, 2u32, 4u32, 0x0805u32, minimums, ranges, 0u32, 0x7000u32],
        );
        let log = log.borrow();
        assert_eq!(log[0], ("size", vec![4, 8, 5]));
        // Row 0: -(-0.5) * 256 / 1 = 128; row 1: -2 * 256 / 4 = -128 (low dword of the 64-bit
        // result). Each row reads 10 bytes of the stream (the double's answer).
        assert_eq!(log[1], ("size", vec![4, 8, 5]));
        assert_eq!(log[2], ("decode", vec![0x7000, 8, 128, 20]));
        assert_eq!(log[3].0, "expand");
        assert_eq!(log[3].1[1..], [8, dest, 4]);
        assert_eq!(log[3].1[0], base);
        assert_eq!(log[4], ("pass", vec![dest, 4]));
        assert_eq!(log[5], ("size", vec![4, 8, 5]));
        assert_eq!(log[6], ("decode", vec![0x700a, 8, 0xffff_ff80, 20]));
        assert_eq!(log[7].1[1..], [8, dest + 16, 4]);
        assert_eq!(log[8], ("pass", vec![dest + 16, 4]));
        // The scratch buffer (20 bytes rounded up to 128) is given back.
        assert_eq!(e.mem.u32(router), base);
    }

    #[test]
    fn decompress_takes_each_rows_width_from_the_width_array() {
        let mut e = engine();
        let router = install_router(&mut e);
        let base = e.mem.u32(router);
        let log = decompress_doubles(&mut e);
        let minimums = e.mem.alloc(8);
        let ranges = e.mem.alloc(8);
        let widths = e.mem.alloc(4);
        e.mem.set_f32(minimums, -1.0);
        e.mem.set_f32(minimums + 4, -0.5);
        e.mem.set_f32(ranges, 1.0);
        e.mem.set_f32(ranges + 4, 1.0);
        // Row 0: width 3, q = 8 = 1 << 3, which becomes 7. Row 1: width 32 shifts by 0: unit 1.
        e.mem.set_u8(widths, 3);
        e.mem.set_u8(widths + 1, 32);
        e.call(
            0x00e3_00b0,
            &args![
                0x0a00_0000u32,
                2u32,
                2u32,
                0x0805u32,
                minimums,
                ranges,
                widths,
                0x7000u32
            ],
        );
        let log = log.borrow();
        let decodes: Vec<&Vec<u32>> = log
            .iter()
            .filter(|(tag, _)| *tag == "decode")
            .map(|(_, words)| words)
            .collect();
        assert_eq!(*decodes[0], vec![0x7000, 3, 7, 20]);
        // 0.5 / 1 truncates to 0, and 0 is not the unit 1.
        assert_eq!(*decodes[1], vec![0x700a, 32, 0, 20]);
        let expands: Vec<u32> = log
            .iter()
            .filter(|(tag, _)| *tag == "expand")
            .map(|(_, words)| words[1])
            .collect();
        assert_eq!(expands, vec![3, 32]);
        assert_eq!(e.mem.u32(router), base);
    }

    #[test]
    fn decompress_of_a_nan_range_quantises_to_zero_and_zero_rows_decode_nothing() {
        let mut e = engine();
        let router = install_router(&mut e);
        let base = e.mem.u32(router);
        let log = decompress_doubles(&mut e);
        let minimums = e.mem.alloc(4);
        let ranges = e.mem.alloc(4);
        e.mem.set_f32(minimums, -1.0);
        e.mem.set_u32(ranges, 0x7fc0_0000);
        e.call(
            0x00e3_00b0,
            &args![
                0x0a00_0000u32,
                1u32,
                2u32,
                0x0402u32,
                minimums,
                ranges,
                0u32,
                0x7000u32
            ],
        );
        assert_eq!(
            log.borrow()
                .iter()
                .find(|(tag, _)| *tag == "decode")
                .unwrap()
                .1[2],
            0
        );
        log.borrow_mut().clear();
        e.call(
            0x00e3_00b0,
            &args![
                0x0a00_0000u32,
                0u32,
                2u32,
                0x0402u32,
                minimums,
                ranges,
                0u32,
                0x7000u32
            ],
        );
        // Only the size of the buffer is asked for.
        assert_eq!(log.borrow().len(), 1);
        assert_eq!(e.mem.u32(router), base);
    }

    /// A profiling stream with room for 16 twelve-byte records: the labels of its records.
    fn install_big_stream(e: &mut Engine) -> u32 {
        let records = e.mem.alloc(0xc0);
        let stream = e.mem.alloc(0x10);
        e.mem.set_u32(stream + 4, records);
        e.mem.set_u32(stream + 0xc, records + 0xc0);
        e.set_global(PROFILE_SLOT, 5u32);
        e.register_double(TLS_GET_VALUE, move |_, _| Ret {
            eax: stream,
            ..Ret::default()
        });
        stream
    }

    /// The labels of the records of a stream from `install_big_stream`; a record is 12 bytes.
    fn big_labels(e: &Engine, stream: u32) -> Vec<u32> {
        let first = e.mem.u32(stream + 0xc) - 0xc0;
        let cursor = e.mem.u32(stream + 4);
        (0..(cursor - first) / 12)
            .map(|record| e.mem.u32(first + 12 * record))
            .collect()
    }

    /// Like `big_labels` for a stream whose first record is the 16-byte start record (two labels).
    fn big_labels_with_start(e: &Engine, stream: u32) -> Vec<u32> {
        let first = e.mem.u32(stream + 0xc) - 0xc0;
        let cursor = e.mem.u32(stream + 4);
        let mut labels = vec![e.mem.u32(first), e.mem.u32(first + 0xc)];
        let mut at = first + 0x10;
        while at < cursor {
            labels.push(e.mem.u32(at));
            at += 12;
        }
        labels
    }

    #[test]
    fn a_block_is_decompressed_from_its_table_entry_with_the_animations_bytes() {
        let mut e = engine();
        install_router(&mut e);
        let stream = install_big_stream(&mut e);
        let log = decompress_doubles(&mut e);
        let (animation, data) = block_animation(&mut e);
        e.mem.set_u8(animation + 0x30, 0x08);
        e.mem.set_u8(animation + 0x31, 0x05);
        e.mem.set_u32(animation + 0x38, 0x180);
        e.mem.set_u32(animation + 0x3c, 0x190);
        e.mem.set_u32(animation + 0x40, 0x1a0);
        e.mem.set_f32(data + 0x180, -0.25);
        e.mem.set_f32(data + 0x190, 1.0);
        e.mem.set_u8(data + 0x1a0, 6);
        let dest = 0x0a00_0000u32;
        // Frame 6 is in block 1 (6 / 4): its stream is the table entry (0x35) plus the word at
        // +0x5c (0x20), in the data. Row 0 has width 6: q = 0.25 * 64 / 1 = 16.
        e.call(0x00e3_06d0, &args![animation, 6u32, dest, 1u32]);
        let log = log.borrow();
        assert_eq!(log[0], ("size", vec![4, 8, 5]));
        assert_eq!(log[1], ("size", vec![4, 6, 5]));
        assert_eq!(log[2], ("decode", vec![data + 0x35 + 0x20, 6, 16, 20]));
        assert_eq!(
            big_labels(&e, stream),
            vec![LABEL_DECOMPRESS_BLOCK, PROFILE_END]
        );
    }

    #[test]
    #[should_panic(expected = "integer divide fault at 0x00e30746")]
    fn decompressing_a_block_of_a_zero_block_size_faults() {
        let mut e = engine();
        install_router(&mut e);
        install_big_stream(&mut e);
        let (animation, _) = block_animation(&mut e);
        e.mem.set_u32(animation + 0x2c, 0);
        e.call(0x00e3_06d0, &args![animation, 6u32, 0x0a00_0000u32, 1u32]);
    }

    /// A cache object whose slots `0x0c` to `0x30` log their calls and answer what `answers`
    /// says (by slot, 0 for the others); returns the animation and the holder word.
    fn cache_doubles(e: &mut Engine, answers: &[(u32, u32)]) -> (u32, u32) {
        let (animation, _cache, holder) = cache_setup(e);
        for (slot, value) in answers {
            answer(e, 0x0900_0500 + slot, *value);
        }
        (animation, holder)
    }

    #[test]
    fn a_cache_entry_that_is_not_ready_answers_zero() {
        let mut e = engine();
        let stream = install_big_stream(&mut e);
        let flag = e.mem.alloc(4);
        e.mem.set_u8(flag, 1);
        let (animation, holder) = cache_doubles(&mut e, &[(0x28, flag)]);
        answer(&mut e, CACHE_KEY_SLOT, 9);
        let got = e
            .call(0x00e3_07b0, &args![holder, animation, 5u32, 3u32])
            .u32();
        assert_eq!(got, 0);
        assert_eq!(
            sequence(&e, &[0x0900_052c, 0x0900_0528, 0x0900_050c, 0x0900_0530]),
            vec![0x0900_052c, 0x0900_0528, 0x0900_0530]
        );
        // The bytes asked for: block size 4 * count 3 * 4, with the key of the frame.
        assert_eq!(calls(&e, CACHE_KEY_SLOT), vec![vec![animation, 5]]);
        assert_eq!(calls(&e, 0x0900_0528)[0][2..], [9, 48]);
        assert_eq!(big_labels(&e, stream), vec![LABEL_CACHE, PROFILE_END]);
    }

    #[test]
    fn a_cached_block_is_answered_after_its_slot_is_read() {
        let mut e = engine();
        let stream = install_big_stream(&mut e);
        let flag = e.mem.alloc(4);
        let (animation, holder) = cache_doubles(&mut e, &[(0x28, flag), (0x0c, 0x0a00_1000)]);
        answer(&mut e, CACHE_KEY_SLOT, 9);
        let got = e
            .call(0x00e3_07b0, &args![holder, animation, 5u32, 3u32])
            .u32();
        assert_eq!(got, 0x0a00_1000);
        assert_eq!(
            sequence(
                &e,
                &[
                    0x0900_052c,
                    0x0900_0528,
                    0x0900_050c,
                    0x0900_0510,
                    0x0900_0518,
                    0x0900_0530
                ]
            ),
            vec![
                0x0900_052c,
                0x0900_0528,
                0x0900_050c,
                0x0900_0518,
                0x0900_0530
            ]
        );
        assert_eq!(big_labels(&e, stream), vec![LABEL_CACHE, PROFILE_END]);
    }

    #[test]
    fn a_missing_block_with_no_room_answers_zero() {
        let mut e = engine();
        install_big_stream(&mut e);
        let flag = e.mem.alloc(4);
        let (animation, holder) = cache_doubles(&mut e, &[(0x28, flag)]);
        answer(&mut e, CACHE_KEY_SLOT, 9);
        let got = e
            .call(0x00e3_07b0, &args![holder, animation, 5u32, 3u32])
            .u32();
        assert_eq!(got, 0);
        assert_eq!(
            sequence(
                &e,
                &[
                    0x0900_050c,
                    0x0900_0510,
                    0x0900_0518,
                    0x0900_0520,
                    0x0900_0530
                ]
            ),
            vec![0x0900_050c, 0x0900_0510, 0x0900_0530]
        );
    }

    #[test]
    fn a_missing_block_is_decompressed_into_the_entry_the_cache_gave() {
        let mut e = engine();
        install_router(&mut e);
        install_big_stream(&mut e);
        let log = decompress_doubles(&mut e);
        let flag = e.mem.alloc(4);
        let (animation, holder) = cache_doubles(&mut e, &[(0x28, flag), (0x10, 0x0a00_2000)]);
        // The animation needs the data of `block_animation` for the decompression.
        let (blocks, data) = block_animation(&mut e);
        for offset in [0x2c, 0x54, 0x58, 0x5c, 0x60, 0x64] {
            let value = e.mem.u32(blocks + offset);
            e.mem.set_u32(animation + offset, value);
        }
        e.mem.set_u32(animation + 0x38, 0x180);
        e.mem.set_u32(animation + 0x3c, 0x190);
        e.mem.set_u32(animation + 0x40, 0x1a0);
        e.mem.set_f32(data + 0x180, -0.25);
        e.mem.set_f32(data + 0x190, 1.0);
        answer(&mut e, CACHE_KEY_SLOT, 9);
        let got = e
            .call(0x00e3_07b0, &args![holder, animation, 5u32, 1u32])
            .u32();
        assert_eq!(got, 0x0a00_2000);
        let wanted = [
            0x0900_052c,
            0x0900_0528,
            0x0900_050c,
            0x0900_0510,
            0x0900_0518,
            0x0900_0520,
            0x0900_0524,
            0x0900_0530,
        ];
        assert_eq!(
            sequence(&e, &wanted),
            vec![
                0x0900_052c,
                0x0900_0528,
                0x0900_050c,
                0x0900_0510,
                0x0900_0518,
                0x0900_0520,
                0x0900_0530,
                0x0900_052c,
                0x0900_0524,
                0x0900_0530
            ]
        );
        // The block was decompressed into the entry: its row was expanded at 0x0a002000.
        assert!(log
            .borrow()
            .iter()
            .any(|(tag, words)| *tag == "expand" && words[2] == 0x0a00_2000));
    }

    /// The animation of the sampling tests: 2 transform tracks, 1 float track, 3 values, a
    /// frame count of 9 and a duration of 1.0 behind a fake vtable (frame count and cache key
    /// slots), and the three blocks of `block_animation` (block size 4).
    fn sampling_animation(e: &mut Engine) -> (u32, u32) {
        let (animation, data) = block_animation(e);
        let object = fake_object(e, &[(0x20, FRAME_COUNT_SLOT), (0x40, CACHE_KEY_SLOT)]);
        let vtable = e.mem.u32(object);
        e.mem.set_u32(animation, vtable);
        answer(e, FRAME_COUNT_SLOT, 9);
        answer(e, CACHE_KEY_SLOT, 9);
        e.mem.set_f32(animation + 0xc, 1.0);
        for (offset, value) in [
            (0x10, 2),
            (0x14, 1),
            (0x34, 3),
            (0x38, 0x180),
            (0x3c, 0x190),
            (0x40, 0x1a0),
            (0x44, 0x1c0),
            (0x48, 0x1d0),
            (0x4c, 5),
            (0x50, 6),
        ] {
            e.mem.set_u32(animation + offset, value);
        }
        (animation, data)
    }

    /// The decompression callees of the sampling tests: `BLOCK_DECODE` notes which block the
    /// stream is in (its table entry), and `EXPAND_REAL` fills row `r` of the table (a row is
    /// 16 bytes from `table`) with `r * 10 + column + entry`.
    fn table_doubles(e: &mut Engine, data: u32, table: u32) {
        let entry = Rc::new(RefCell::new(0u32));
        let noted = Rc::clone(&entry);
        e.register_double(BLOCK_DECODE, move |_, a| {
            *noted.borrow_mut() = a[0] - (data + 0x20);
            Ret {
                eax: 0,
                ..Ret::default()
            }
        });
        e.register_double(EXPAND_REAL, move |e, a| {
            let row = (a[2] - table) / (4 * a[3]);
            let entry = *entry.borrow();
            for column in 0..a[3] {
                e.mem
                    .set_f32(a[2] + 4 * column, (row * 10 + column + entry) as f32);
            }
            Ret::default()
        });
    }

    #[test]
    fn a_frame_inside_a_block_interpolates_between_two_columns() {
        let mut e = engine();
        let router = install_router(&mut e);
        let base = e.mem.u32(router);
        let stream = install_big_stream(&mut e);
        decompress_doubles(&mut e);
        let (animation, data) = sampling_animation(&mut e);
        table_doubles(&mut e, data, base);
        let seen = record_recompose(&mut e);
        let transforms = e.mem.alloc(0x20);
        let floats = e.mem.alloc(0x20);
        // 0.3125 / 1 * (9 - 1) = 2.5: frame 2, delta 0.5, column 2 of block 0 (entry 0x10 = 16).
        // Row r blends columns 2 and 3: (r * 10 + 18) * 0.5 + (r * 10 + 19) * 0.5.
        e.call(
            0x00e3_0980,
            &args![animation, 0.3125f32, 2u32, transforms, 1u32, floats, 0u32],
        );
        let (words, result) = seen.borrow()[0].clone();
        assert_eq!(result[..3], [18.5, 28.5, 38.5]);
        let result_at = base + 128;
        assert_eq!(
            words,
            vec![
                2,
                1,
                data + 0x1c0,
                data + 0x1c0 + 4,
                data + 0x1d0,
                data + 0x1d0 + 20,
                result_at,
                result_at + 24
            ]
        );
        assert_eq!(calls(&e, RECOMPOSE)[0][1..], [transforms, floats]);
        assert_eq!(e.mem.u32(router), base);
        // No cache: the one block was decompressed, then nothing was given back.
        assert_eq!(calls(&e, BLOCK_DECODE).len(), 3);
        assert!(calls(&e, 0x0900_052c).is_empty());
        assert_eq!(
            big_labels_with_start(&e, stream),
            vec![
                LABEL_SAMPLE,
                LABEL_STATIC,
                LABEL_DECOMPRESS,
                LABEL_DECOMPRESS_BLOCK,
                PROFILE_END,
                LABEL_INTERPOLATE,
                LABEL_RECOMPOSE,
                LABEL_END
            ]
        );
    }

    #[test]
    fn the_last_column_of_a_block_blends_in_column_zero_of_the_next_block() {
        let mut e = engine();
        let router = install_router(&mut e);
        let base = e.mem.u32(router);
        let stream = install_big_stream(&mut e);
        decompress_doubles(&mut e);
        let (animation, data) = sampling_animation(&mut e);
        table_doubles(&mut e, data, base);
        let seen = record_recompose(&mut e);
        let transforms = e.mem.alloc(0x20);
        let floats = e.mem.alloc(0x20);
        // 0.4375 * 8 = 3.5: frame 3 (the last column of block 0, entry 16), delta 0.5. Column 3 of
        // block 0 is r * 10 + 19, column 0 of block 1 (entry 0x35 = 53) is r * 10 + 53.
        e.call(
            0x00e3_0980,
            &args![animation, 0.4375f32, 2u32, transforms, 1u32, floats, 0u32],
        );
        let (_, result) = seen.borrow()[0].clone();
        assert_eq!(result[..3], [36.0, 46.0, 56.0]);
        assert_eq!(calls(&e, BLOCK_DECODE).len(), 6);
        assert_eq!(e.mem.u32(router), base);
        assert_eq!(
            big_labels_with_start(&e, stream),
            vec![
                LABEL_SAMPLE,
                LABEL_STATIC,
                LABEL_DECOMPRESS,
                LABEL_DECOMPRESS_BLOCK,
                PROFILE_END,
                LABEL_INTERPOLATE,
                LABEL_DECOMPRESS,
                LABEL_DECOMPRESS_BLOCK,
                PROFILE_END,
                LABEL_INTERPOLATE,
                LABEL_RECOMPOSE,
                LABEL_END
            ]
        );
    }

    #[test]
    fn a_block_from_the_cache_is_used_in_place_and_given_back() {
        let mut e = engine();
        let router = install_router(&mut e);
        let base = e.mem.u32(router);
        install_big_stream(&mut e);
        decompress_doubles(&mut e);
        let (animation, _data) = sampling_animation(&mut e);
        let seen = record_recompose(&mut e);
        // The cache answers a table of its own (r * 10 + column); the frame is not ready-checked
        // by the byte, which stays 0.
        let cached = e.mem.alloc(0x80);
        for row in 0..3u32 {
            for column in 0..4u32 {
                e.mem
                    .set_f32(cached + 16 * row + 4 * column, (row * 10 + column) as f32);
            }
        }
        let flag = e.mem.alloc(4);
        let (_, cache, holder) = cache_setup(&mut e);
        e.mem.set_u32(holder, cache);
        answer(&mut e, 0x0900_0528, flag);
        answer(&mut e, 0x0900_050c, cached);
        let transforms = e.mem.alloc(0x20);
        let floats = e.mem.alloc(0x20);
        e.call(
            0x00e3_0980,
            &args![animation, 0.3125f32, 2u32, transforms, 1u32, floats, cache],
        );
        let (_, result) = seen.borrow()[0].clone();
        assert_eq!(result[..3], [2.5, 12.5, 22.5]);
        assert!(calls(&e, BLOCK_DECODE).is_empty());
        // The lookup went through the cache holder slot and the block was released with the
        // default count (3).
        let lookups = calls(&e, 0x0900_0528);
        assert_eq!(lookups.len(), 1);
        let release = calls(&e, 0x0900_051c);
        assert_eq!(release.len(), 1);
        assert_eq!(release[0][2], 9);
        assert_eq!(release[0][3], 4 * 3 * 4);
        assert_eq!(e.mem.u32(router), base);
    }

    #[test]
    fn other_track_counts_ask_for_the_dynamic_statistics() {
        let mut e = engine();
        let router = install_router(&mut e);
        let base = e.mem.u32(router);
        install_big_stream(&mut e);
        decompress_doubles(&mut e);
        let (animation, data) = sampling_animation(&mut e);
        // Statistics 7 to 10 are 1, 2, 3, 4: 6 values without float tracks, 10 with them.
        e.register_double(CALC_DYNAMIC_STATS, |e, a| {
            for (offset, value) in [(0x1c, 1u32), (0x20, 2), (0x24, 3), (0x28, 4)] {
                e.mem.set_u32(a[4] + offset, value);
            }
            Ret::default()
        });
        table_doubles(&mut e, data, base);
        let seen = record_recompose(&mut e);
        let transforms = e.mem.alloc(0x20);
        let floats = e.mem.alloc(0x20);
        e.call(
            0x00e3_0980,
            &args![animation, 0.3125f32, 3u32, transforms, 0u32, floats, 0u32],
        );
        let stats = calls(&e, CALC_DYNAMIC_STATS);
        assert_eq!(stats[0][..4], [data + 0x1c0, data + 0x1c0 + 4, 3, 0]);
        // Six values: the table is 4 * 6 * 4 = 96 bytes (128 reserved), the result starts after.
        let (words, _) = seen.borrow()[0].clone();
        assert_eq!(words[0], 3);
        assert_eq!(words[1], 0);
        assert_eq!(words[6], base + 128);
        assert_eq!(words[7], base + 128 + 6 * 4);
        assert_eq!(e.mem.u32(router), base);
        // Two float tracks differ from the animation's one: tracks (2), float count (2).
        e.call(
            0x00e3_0980,
            &args![animation, 0.3125f32, 2u32, transforms, 2u32, floats, 0u32],
        );
        let stats = calls(&e, CALC_DYNAMIC_STATS);
        assert_eq!(stats[1][..4], [data + 0x1c0, data + 0x1c0 + 4, 2, 2]);
        let (words, _) = seen.borrow()[1].clone();
        // Ten values: the table is 4 * 10 * 4 = 160 bytes, 256 reserved.
        assert_eq!(words[6], base + 256);
        assert_eq!(words[7], base + 256 + 6 * 4);
    }

    #[test]
    #[should_panic(expected = "integer divide fault at 0x00e30b7c")]
    fn sampling_from_the_cache_with_a_zero_block_size_faults() {
        let mut e = engine();
        install_router(&mut e);
        install_big_stream(&mut e);
        decompress_doubles(&mut e);
        let (animation, _) = sampling_animation(&mut e);
        let flag = e.mem.alloc(4);
        let (_, cache, _) = cache_setup(&mut e);
        answer(&mut e, 0x0900_0528, flag);
        answer(&mut e, 0x0900_050c, 0x0a00_1000);
        e.mem.set_u32(animation + 0x2c, 0);
        let transforms = e.mem.alloc(0x20);
        let floats = e.mem.alloc(0x20);
        e.call(
            0x00e3_0980,
            &args![animation, 0.3125f32, 2u32, transforms, 1u32, floats, cache],
        );
    }

    #[test]
    fn sampling_from_chunks_keeps_its_blocks_at_the_exe_frame_offsets() {
        let mut e = engine();
        install_router(&mut e);
        install_stream(&mut e);
        install_callees(&mut e);
        let (_, chunks) = setup(&mut e);
        let transforms = e.mem.alloc(0x20);
        let floats = e.mem.alloc(0x20);
        // The frame block is the next stack local: the statistics at +0x24, the analysis block at
        // +0x5c.
        let frame = e.next_stack_local(SAMPLE_CHUNKS_FRAME_SIZE);
        e.call(
            0x00e3_0200,
            &args![5u32, 0.25f32, 3u32, transforms, 0u32, floats, chunks, 4u32],
        );
        assert_eq!(calls(&e, CALC_DYNAMIC_STATS)[0][4], frame + 0x24);
        assert_eq!(calls(&e, RECOMPOSE)[0][0], frame + 0x5c);
    }

    #[test]
    fn the_endian_pass_keeps_its_statistics_at_0x24_of_the_exe_frame() {
        let mut e = engine();
        let (animation, _) = endian_animation(&mut e);
        stats_double(&mut e, [1, 1, 0, 0, 0, 0, 0, 1, 0, 1, 0]);
        let frame = e.next_stack_local(ENDIAN_FRAME_SIZE);
        e.call(0x00e2_fc10, &args![animation]);
        assert_eq!(calls(&e, ANALYSE_ENDIAN)[0][4], frame + 0x24);
    }

    #[test]
    fn sampling_tracks_keeps_its_slots_at_the_exe_frame_offsets() {
        let mut e = engine();
        let router = install_router(&mut e);
        let base = e.mem.u32(router);
        install_big_stream(&mut e);
        decompress_doubles(&mut e);
        let (animation, data) = sampling_animation(&mut e);
        table_doubles(&mut e, data, base);
        record_recompose(&mut e);
        let transforms = e.mem.alloc(0x20);
        let floats = e.mem.alloc(0x20);
        // The frame block is the next stack local: the analysis block at +0x30, the delta at
        // +0x18, and the argument words from +0x6c, the frame (2) in the slot of `time`. Its
        // bytes stay on the thread's stack after the call (nothing takes that place again before
        // they are read).
        let frame = e.next_stack_local(SAMPLE_TRACKS_FRAME_SIZE);
        e.call(
            0x00e3_0980,
            &args![animation, 0.3125f32, 2u32, transforms, 1u32, floats, 0u32],
        );
        assert_eq!(calls(&e, RECOMPOSE)[0][0], frame + 0x30);
        assert_eq!(e.mem.f32(frame + 0x18), 0.5);
        let arguments: Vec<u32> = (0..6).map(|i| e.mem.u32(frame + 0x6c + 4 * i)).collect();
        assert_eq!(arguments, vec![2, 2, transforms, 1, floats, 0]);
    }
}

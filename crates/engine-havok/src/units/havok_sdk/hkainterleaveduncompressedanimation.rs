//! `Havok SDK/hkainterleaveduncompressedanimation.cpp` (Xbox PDB source unit), subsystem
//! `Havok SDK`: its functions in FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkaInterleavedUncompressedAnimation` (Xbox PDB name) keeps its samples frame by frame. The
//! class is not in the Xbox PDB's type list, so the fields are read off the code (PC offsets):
//! `+0xc` the duration, `+0x10` / `+0x14` the transform / float track counts, `+0x28` the
//! pointer to the transforms (48 bytes each: translation, rotation, scale, as three 16-byte
//! vectors, the tracks of one frame next to each other), `+0x2c` the total number of
//! transforms, `+0x34` the pointer to the floats, `+0x38` their total number.
//!
//! The transform blending is SSE code on `f32` lanes; the float blending is x87 code, done in
//! `f64` and rounded to `f32` at each store. A `float` copied through `FLD`/`FSTP` goes through
//! [`x87_quiet`]. Functions that take registers are written with the register values first.
//! Nothing here has an exception frame.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkadeltacompressedanimation::profile_record;
use crate::units::havok_sdk::hkpsphereshape::{load_vec, store_vec, zip_with, Vec4};

/// Runs `f` on an exe-shaped frame: a function that aligns `ESP` (`AND ESP,0xfffffff0`) and
/// then reserves `below` bytes (its `SUB ESP` and pushes) has its frame `below` bytes under a
/// 16-byte boundary; `f` gets the frame's lowest address.
fn with_aligned_frame<R>(e: &mut Engine, below: u32, f: impl FnOnce(&mut Engine, u32) -> R) -> R {
    e.with_stack(below + 0x10, |e, raw| {
        let boundary = (raw.addr() + below + 0xf) & !0xf;
        f(e, boundary - below)
    })
}

/// The frame base [`with_aligned_frame`] picks for a block at `raw` (tests).
#[cfg(test)]
fn aligned_frame_at(raw: u32, below: u32) -> u32 {
    ((raw + below + 0xf) & !0xf) - below
}

/// `hkaAnimation::getFrameAndDelta` (`00e2f7c0`, `this`, `(time, frame pointer, delta pointer)`).
const GET_FRAME_AND_DELTA: u32 = 0x00e2_f7c0;
/// The quaternion step both blending functions end with (`005611c0`, `this` = the quaternion).
const QUATERNION_STEP: u32 = 0x0056_11c0;
/// The vector of ones (`010ec7f0`) and the vector of zeros (`010c7160`) the blending reads.
const ONES: u32 = 0x010e_c7f0;
const ZEROS: u32 = 0x010c_7160;
/// The profiling labels: the opening one of `samplePartialTracks` and its closing one.
const LABEL_SAMPLE: u32 = 0x010e_bd50;
const LABEL_END: u32 = 0x010c_41f8;

const DURATION: u32 = 0xc;
const NUM_TRANSFORM_TRACKS: u32 = 0x10;
const NUM_FLOAT_TRACKS: u32 = 0x14;
const TRANSFORMS: u32 = 0x28;
const TOTAL_TRANSFORMS: u32 = 0x2c;
const FLOATS: u32 = 0x34;
const TOTAL_FLOATS: u32 = 0x38;
/// The vtable offset of `samplePartialTracks` and of `getDataChunks`.
const SLOT_SAMPLE_PARTIAL_TRACKS: u32 = 0x10;
const SLOT_GET_DATA_CHUNKS: u32 = 0x28;

/// Splits `time` into a frame and the fraction toward the next one with
/// `hkaAnimation::getFrameAndDelta` (the time goes through `FLD`/`FSTP`).
fn frame_and_delta(e: &mut Engine, this: u32, time: f32) -> (u32, f32) {
    e.with_stack(8, |e, results| {
        let delta_at = results.addr() + 4;
        e.call(
            GET_FRAME_AND_DELTA,
            &args![this, x87_quiet(time), results, delta_at],
        );
        (e.mem.u32(results.addr()), e.mem.f32(delta_at))
    })
}

/// Blends the transform at `from` into the one at `to` by `delta` and stores the result at `out`
/// (three 16-byte vectors each: translation, rotation, scale). Translation and scale are
/// `(1 - delta) * from + to * delta` per lane. The rotation is
/// `(1 - delta) * from + sign * to` with the `delta` taken negative when the dot product of the
/// two rotations (summed as `(x + z)` and `(y + w)`, then together) is below zero (not for NaN),
/// where `1 - delta` is computed on the x87 stack and the sign goes through `FLD`/`FSTP`; the
/// quaternion step `005611c0` runs on the result before it is stored. Scale, translation and
/// rotation are stored in that order.
fn blend_transform(e: &mut Engine, from: u32, to: u32, delta: f32, out: u32) {
    let ones = load_vec(e, ONES);
    let mul = |a: Vec4, b: Vec4| zip_with(a, b, |x, y| x * y);
    let add = |a: Vec4, b: Vec4| zip_with(a, b, |x, y| x + y);
    let translation_from = load_vec(e, from);
    let rotation_from = load_vec(e, from + 0x10);
    let scale_from = load_vec(e, from + 0x20);
    let translation_to = load_vec(e, to);
    let rotation_to = load_vec(e, to + 0x10);
    let scale_to = load_vec(e, to + 0x20);
    let weight = [delta; 4];
    let rest = zip_with(ones, weight, |x, y| x - y);
    store_vec(
        e,
        out + 0x20,
        add(mul(rest, scale_from), mul(scale_to, weight)),
    );
    store_vec(
        e,
        out,
        add(mul(rest, translation_from), mul(translation_to, weight)),
    );
    let products = mul(rotation_to, rotation_from);
    let outer = products[0] + products[2];
    let inner = products[1] + products[3];
    let dot = inner + outer;
    let rest_x87 = (1.0 - f64::from(x87_quiet(delta))) as f32;
    let sign = if 0.0 > dot {
        e.mem.f32(ZEROS) - delta
    } else {
        delta
    };
    let sign = x87_quiet(sign);
    let rotation = add(
        mul([rest_x87; 4], rotation_from),
        mul([sign; 4], rotation_to),
    );
    let rotation = e.with_stack(16, |e, scratch| {
        store_vec(e, scratch.addr(), rotation);
        e.call(QUATERNION_STEP, &args![scratch]);
        load_vec(e, scratch.addr())
    });
    store_vec(e, out + 0x10, rotation);
}

// Translated from 00e3f920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaInterleavedUncompressedAnimation::sampleTracks` (Xbox PDB): calls the animation's own slot
/// `0x10` (`samplePartialTracks`) with the time (through `FLD`/`FSTP`), the transform track count
/// (`+0x10`), the transforms, the float track count (`+0x14`), the floats and the cache.
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
            x87_quiet(time),
            transform_tracks,
            transforms,
            float_tracks,
            floats,
            cache
        ],
    );
}

// Translated from 00e3f950 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaInterleavedUncompressedAnimation::getNumDataChunks` (Xbox PDB), `RET 8`: one chunk for the
/// object, one more when there are transform tracks and one more when there are float tracks
/// (signed counts). The stack words are not read.
pub fn get_num_data_chunks(e: &mut Engine, this: Ptr, _unused_1: u32, _unused_2: u32) -> u32 {
    let a = this.addr();
    let mut chunks = if (e.mem.u32(a + NUM_TRANSFORM_TRACKS) as i32) <= 0 {
        1
    } else {
        2
    };
    if (e.mem.u32(a + NUM_FLOAT_TRACKS) as i32) > 0 {
        chunks += 1;
    }
    chunks
}

// Translated from 00e3f970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaInterleavedUncompressedAnimation::getMaxSizeOfCombinedDataChunks` (Xbox PDB): `0x40` for
/// the object, `96` bytes per transform track and the float track count times 8 plus 15 rounded
/// down to a multiple of 16.
pub fn get_max_size_of_combined_data_chunks(e: &mut Engine, this: Ptr) -> u32 {
    let floats = e.mem.u32(this.addr() + NUM_FLOAT_TRACKS);
    let transforms = e.mem.u32(this.addr() + NUM_TRANSFORM_TRACKS);
    let float_bytes = floats.wrapping_mul(8).wrapping_add(0x1f) & 0xffff_fff0;
    let transform_bytes = transforms.wrapping_mul(3) << 5;
    float_bytes.wrapping_add(transform_bytes).wrapping_add(0x40)
}

// Translated from 00e3f990 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaInterleavedUncompressedAnimation::getNumOriginalFrames` (Xbox PDB): the total number of
/// transforms divided by the transform track count (signed `IDIV`), or, with no transform
/// tracks, the total number of floats divided by the float track count.
pub fn get_num_original_frames(e: &mut Engine, this: Ptr) -> i32 {
    let a = this.addr();
    let transforms = e.mem.u32(a + NUM_TRANSFORM_TRACKS) as i32;
    if transforms != 0 {
        (e.mem.u32(a + TOTAL_TRANSFORMS) as i32) / transforms
    } else {
        (e.mem.u32(a + TOTAL_FLOATS) as i32) / (e.mem.u32(a + NUM_FLOAT_TRACKS) as i32)
    }
}

// Translated from 00e3f9b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaInterleavedUncompressedAnimation::sampleIndividualTransformTracks` (Xbox PDB), `this`, then
/// `(time, tracks, count, out)`: for each of the `count` entries of the 16-bit list `tracks`
/// whose value is not negative and below the transform track count, blends that track's
/// transform of the frame `time` falls in with the one of the next frame (`blend_transform`)
/// into the 48-byte slot of `out` at the entry's index. Other slots are left alone.
pub fn sample_individual_transform_tracks(
    e: &mut Engine,
    this: Ptr,
    time: f32,
    tracks: Ptr,
    count: u32,
    out: Ptr,
) {
    let a = this.addr();
    let (frame, delta) = frame_and_delta(e, a, time);
    for index in 0..count {
        let track = e.mem.u16(tracks.addr().wrapping_add(index.wrapping_mul(2))) as i16;
        if track < 0 {
            continue;
        }
        let total = e.mem.u32(a + NUM_TRANSFORM_TRACKS);
        let track = i32::from(track);
        if track >= total as i32 {
            continue;
        }
        let first = total
            .wrapping_mul(frame)
            .wrapping_mul(3)
            .wrapping_shl(4)
            .wrapping_add(e.mem.u32(a + TRANSFORMS));
        let from = first.wrapping_add((track as u32).wrapping_mul(3).wrapping_shl(4));
        let to = first.wrapping_add(
            total
                .wrapping_add(track as u32)
                .wrapping_mul(3)
                .wrapping_shl(4),
        );
        blend_transform(
            e,
            from,
            to,
            delta,
            out.addr().wrapping_add(index.wrapping_mul(0x30)),
        );
    }
}

// Translated from 00e3fb60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaInterleavedUncompressedAnimation::sampleIndividualFloatTracks` (Xbox PDB), `this`, then
/// `(time, tracks, count, out)`: for each of the `count` entries of the 16-bit list `tracks`
/// whose value is not negative and below the float track count, stores
/// `float(frame) * (1 - delta) + float(next frame) * delta` (x87, `1 - delta` rounded to `float`
/// first) at the entry's index in `out`. Other slots are left alone.
pub fn sample_individual_float_tracks(
    e: &mut Engine,
    this: Ptr,
    time: f32,
    tracks: Ptr,
    count: u32,
    out: Ptr,
) {
    let a = this.addr();
    let (frame, delta) = frame_and_delta(e, a, time);
    let rest = (1.0 - f64::from(delta)) as f32;
    for index in 0..count {
        let track = e.mem.u16(tracks.addr().wrapping_add(index.wrapping_mul(2))) as i16;
        if track < 0 {
            continue;
        }
        let total = e.mem.u32(a + NUM_FLOAT_TRACKS);
        let track = i32::from(track);
        if track >= total as i32 {
            continue;
        }
        let base = e
            .mem
            .u32(a + FLOATS)
            .wrapping_add(total.wrapping_mul(frame).wrapping_mul(4));
        let current = e.mem.f32(base.wrapping_add((track as u32).wrapping_mul(4)));
        let next = e
            .mem
            .f32(base.wrapping_add(total.wrapping_add(track as u32).wrapping_mul(4)));
        let value = f64::from(next) * f64::from(delta) + f64::from(current) * f64::from(rest);
        e.mem
            .set_f32(out.addr().wrapping_add(index.wrapping_mul(4)), value as f32);
    }
}

// Translated from 00e3fd20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaInterleavedUncompressedAnimation::getDataChunks` (Xbox PDB), `this`, then
/// `(frame, _, out, _)`: fills the 12-byte chunk records at `out`. Record 0 is the object itself
/// (size `0x40`). With transform tracks the next record is `frame`'s transforms in the transform
/// array (pointer and size `tracks * 96`; the third word is not written). With float tracks the
/// next record is `frame`'s floats (the pointer rounded down to 16 bytes, size `(low nibble +
/// floats * 8 + 15)` rounded down to 16, and the low nibble in the byte at `+8`).
pub fn get_data_chunks(
    e: &mut Engine,
    this: Ptr,
    frame: u32,
    _unused_2: u32,
    out: Ptr,
    _unused_4: u32,
) {
    let a = this.addr();
    let o = out.addr();
    e.mem.set_u32(o, a);
    e.mem.set_u32(o + 4, 0x40);
    let mut index = 1u32;
    let transforms = e.mem.u32(a + NUM_TRANSFORM_TRACKS);
    if (transforms as i32) > 0 {
        let start = transforms
            .wrapping_mul(frame)
            .wrapping_mul(3)
            .wrapping_shl(4)
            .wrapping_add(e.mem.u32(a + TRANSFORMS));
        e.mem.set_u32(o + 0xc, start);
        e.mem
            .set_u32(o + 0x10, transforms.wrapping_mul(3).wrapping_shl(5));
        index = 2;
    }
    let floats = e.mem.u32(a + NUM_FLOAT_TRACKS);
    if (floats as i32) > 0 {
        let record = o.wrapping_add(index * 12);
        let start = e
            .mem
            .u32(a + FLOATS)
            .wrapping_add(floats.wrapping_mul(frame).wrapping_mul(4));
        let low = start & 0xf;
        e.mem.set_u32(record, start & 0xffff_fff0);
        e.mem.set_u8(record + 8, low as u8);
        e.mem.set_u32(
            record + 4,
            low.wrapping_add(floats.wrapping_mul(8)).wrapping_add(0xf) & 0xffff_fff0,
        );
    }
}

// Translated from 00e3fda0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaInterleavedUncompressedAnimation::samplePartialWithDataChunks` (Xbox PDB), a cdecl
/// function of eight stack words that has the same signature as the delta-compressed one
/// (`frame`, `delta`, the track counts and outputs, the chunk records `chunks` that
/// `getDataChunks` filled, the chunk count; the frame and the count are not read). The animation
/// is chunk 0. With transform tracks, chunk 1 holds the frame's transforms followed by the next
/// frame's (one transform track count further on), and the first `num_transforms` are blended
/// into `transforms_out` (`blend_transform`). With float tracks, the next chunk holds the frame's
/// floats (pointer plus the misalignment byte) followed by the next frame's, and
/// `float(frame) * (1 - delta) + float(next frame) * delta` (x87, `1 - delta` rounded to `float`
/// first) is stored for the first `num_floats` into `floats_out`.
#[allow(clippy::too_many_arguments)]
pub fn sample_partial_with_data_chunks(
    e: &mut Engine,
    _unused_1: u32,
    delta: f32,
    num_transforms: u32,
    transforms_out: u32,
    num_floats: u32,
    floats_out: u32,
    chunks: Ptr,
    _unused_8: u32,
) {
    let c = chunks.addr();
    let animation = e.mem.u32(c);
    let transforms = e.mem.u32(animation + NUM_TRANSFORM_TRACKS);
    let mut index = 1u32;
    if (transforms as i32) > 0 {
        let from = e.mem.u32(c + 0xc);
        let to = transforms
            .wrapping_mul(3)
            .wrapping_shl(4)
            .wrapping_add(from);
        for track in 0..num_transforms {
            let step = track.wrapping_mul(0x30);
            blend_transform(
                e,
                from.wrapping_add(step),
                to.wrapping_add(step),
                delta,
                transforms_out.wrapping_add(step),
            );
        }
        index = 2;
    }
    let floats = e.mem.u32(animation + NUM_FLOAT_TRACKS);
    if (floats as i32) > 0 {
        let record = c.wrapping_add(index * 12);
        let current = u32::from(e.mem.u8(record + 8)).wrapping_add(e.mem.u32(record));
        let next = current.wrapping_add(floats.wrapping_mul(4));
        let rest = (1.0 - f64::from(delta)) as f32;
        for value in 0..num_floats {
            let step = value.wrapping_mul(4);
            let blended = f64::from(e.mem.f32(current.wrapping_add(step))) * f64::from(rest)
                + f64::from(delta) * f64::from(e.mem.f32(next.wrapping_add(step)));
            e.mem.set_f32(floats_out.wrapping_add(step), blended as f32);
        }
    }
}

// Translated from 00e3ffc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaInterleavedUncompressedAnimation::samplePartialTracks` (Xbox PDB), `RET 0x18`, `this`, then
/// `(time, num_transforms, transforms_out, num_floats, floats_out, cache)` (the cache is not
/// used), between a profiling record of label `010ebd50` and one of `010c41f8`. When the
/// duration (`+0xc`) is not above `time` (an ordered compare, so not for NaN) the last frame is
/// copied out: the last frame's first `num_transforms` transforms (48 bytes each, no track count
/// check) and its first `num_floats` floats (through `FLD`/`FSTP`). Otherwise `time` is split by
/// `getFrameAndDelta`, `getDataChunks` (slot `0x28`) fills three zeroed chunk records for the frame
/// and `samplePartialWithDataChunks` blends.
#[allow(clippy::too_many_arguments)]
pub fn sample_partial_tracks(
    e: &mut Engine,
    this: Ptr,
    time: f32,
    num_transforms: u32,
    transforms_out: u32,
    num_floats: u32,
    floats_out: u32,
    _unused_6: u32,
) {
    let a = this.addr();
    profile_record(e, LABEL_SAMPLE);
    if e.mem.f32(a + DURATION) <= time {
        for track in 0..num_transforms {
            let source = e
                .mem
                .u32(a + TOTAL_TRANSFORMS)
                .wrapping_sub(e.mem.u32(a + NUM_TRANSFORM_TRACKS))
                .wrapping_add(track)
                .wrapping_mul(3)
                .wrapping_shl(4)
                .wrapping_add(e.mem.u32(a + TRANSFORMS));
            let target = transforms_out.wrapping_add(track.wrapping_mul(0x30));
            for part in [0u32, 0x10, 0x20] {
                let vector = load_vec(e, source.wrapping_add(part));
                store_vec(e, target.wrapping_add(part), vector);
            }
        }
        for value in 0..num_floats {
            let index = e
                .mem
                .u32(a + TOTAL_FLOATS)
                .wrapping_sub(e.mem.u32(a + NUM_FLOAT_TRACKS))
                .wrapping_add(value);
            let bits = e
                .mem
                .u32(e.mem.u32(a + FLOATS).wrapping_add(index.wrapping_mul(4)));
            e.mem.set_u32(
                floats_out.wrapping_add(value.wrapping_mul(4)),
                x87_quiet_bits(bits),
            );
        }
    } else {
        let (frame, delta) = frame_and_delta(e, a, time);
        let delta = x87_quiet(delta);
        // The exe aligns ESP, then `SUB ESP,0x34` and three pushes: a 0x40-byte frame, the three
        // chunk records at +0x1c (00e40138..00e4015e).
        with_aligned_frame(e, 0x40, |e, stack| {
            let chunks = stack + 0x1c;
            for offset in [8u32, 0x14, 0x20] {
                e.mem.set_u8(chunks + offset, 0);
            }
            e.vcall(a, SLOT_GET_DATA_CHUNKS, &args![frame, delta, chunks, 3u32]);
            sample_partial_with_data_chunks(
                e,
                frame,
                delta,
                num_transforms,
                transforms_out,
                num_floats,
                floats_out,
                Ptr::new(chunks),
                3,
            );
        });
    }
    profile_record(e, LABEL_END);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e3_f920, sample_tracks(Ptr, f32, u32, u32, u32)),
        entry!(0x00e3_f950, get_num_data_chunks(Ptr, u32, u32) -> u32),
        entry!(0x00e3_f970, get_max_size_of_combined_data_chunks(Ptr) -> u32),
        entry!(0x00e3_f990, get_num_original_frames(Ptr) -> i32),
        entry!(
            0x00e3_f9b0,
            sample_individual_transform_tracks(Ptr, f32, Ptr, u32, Ptr)
        ),
        entry!(
            0x00e3_fb60,
            sample_individual_float_tracks(Ptr, f32, Ptr, u32, Ptr)
        ),
        entry!(0x00e3_fd20, get_data_chunks(Ptr, u32, u32, Ptr, u32)),
        entry!(
            0x00e3_fda0,
            sample_partial_with_data_chunks(u32, f32, u32, u32, u32, u32, Ptr, u32)
        ),
        entry!(
            0x00e3_ffc0,
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

    const TLS_GET_VALUE: u32 = 0x00fd_f044;
    const PROFILE_SLOT: u32 = 0x0126_835c;
    /// The recording double behind the animation's slot `0x10`.
    const SAMPLE_SLOT: u32 = 0x0900_0010;
    /// A sentinel the tests fill output buffers with.
    const UNTOUCHED: u32 = 0x4242_4242;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        for lane in 0..4 {
            e.mem.set_f32(ONES + 4 * lane, 1.0);
        }
        e
    }

    fn calls_to(calls: &Calls, address: u32) -> Vec<Vec<u32>> {
        calls
            .borrow()
            .iter()
            .filter(|(at, _)| *at == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// An animation (0x80 bytes) with a vtable whose slot `0x10` is the recording double and
    /// slot `0x28` the real `getDataChunks`, and the given track counts.
    fn animation(e: &mut Engine, transforms: u32, floats: u32) -> u32 {
        let table = e.mem.alloc(0x80);
        e.mem.set_u32(table + 0x10, SAMPLE_SLOT);
        e.mem.set_u32(table + 0x28, 0x00e3_fd20);
        let this = e.mem.alloc(0x80);
        e.mem.set_u32(this, table);
        e.mem.set_u32(this + NUM_TRANSFORM_TRACKS, transforms);
        e.mem.set_u32(this + NUM_FLOAT_TRACKS, floats);
        this
    }

    /// Gives `anim` three frames of `values`: transforms (`+0x28`, 48 bytes each, `+0x2c` total
    /// count) laid out frame by frame, and the floats (`+0x34`, `+0x38`). Returns the transform
    /// and float arrays.
    fn samples(e: &mut Engine, anim: u32, transforms: &[[Vec4; 3]], floats: &[f32]) -> (u32, u32) {
        let transform_array = e.mem.alloc(48 * transforms.len() as u32 + 16);
        for (index, parts) in transforms.iter().enumerate() {
            for (part, vector) in parts.iter().enumerate() {
                store_vec(
                    e,
                    transform_array + 48 * index as u32 + 16 * part as u32,
                    *vector,
                );
            }
        }
        let float_array = e.mem.alloc(4 * floats.len() as u32 + 64);
        for (index, value) in floats.iter().enumerate() {
            e.mem.set_f32(float_array + 4 * index as u32, *value);
        }
        e.mem.set_u32(anim + TRANSFORMS, transform_array);
        e.mem
            .set_u32(anim + TOTAL_TRANSFORMS, transforms.len() as u32);
        e.mem.set_u32(anim + FLOATS, float_array);
        e.mem.set_u32(anim + TOTAL_FLOATS, floats.len() as u32);
        (transform_array, float_array)
    }

    /// Two tracks over three frames (frame `f`, track `t` is entry `2 * f + t`): the entries
    /// 2 to 5 are the ones the tests blend. Every rotation is `(0, 0, 0, 1)` except entry 5's,
    /// `(0, 0, 1, 0)` (orthogonal to it).
    fn transforms() -> Vec<[Vec4; 3]> {
        let upright: Vec4 = [0.0, 0.0, 0.0, 1.0];
        let sideways: Vec4 = [0.0, 0.0, 1.0, 0.0];
        let one: Vec4 = [1.0; 4];
        let two: Vec4 = [2.0; 4];
        vec![
            [[0.0; 4], upright, one],
            [[0.0; 4], upright, one],
            [[4.0, 8.0, 12.0, 16.0], upright, one],
            [[1.0, 2.0, 3.0, 4.0], upright, one],
            [[8.0, 16.0, 24.0, 32.0], upright, two],
            [[5.0, 6.0, 7.0, 8.0], sideways, two],
        ]
    }

    /// The frame and fraction the `getFrameAndDelta` double answers; it logs its words.
    fn answer_frame(e: &mut Engine, calls: &Calls, frame: u32, delta: f32) {
        let log = Rc::clone(calls);
        e.register_double(GET_FRAME_AND_DELTA, move |e, a| {
            log.borrow_mut().push((GET_FRAME_AND_DELTA, a.to_vec()));
            e.mem.set_u32(a[2], frame);
            e.mem.set_f32(a[3], delta);
            Ret::default()
        });
    }

    /// The quaternion step double: logs the bits of the vector it is given and adds 100 to its
    /// first lane.
    fn quaternion_step(e: &mut Engine, calls: &Calls) {
        let log = Rc::clone(calls);
        e.register_double(QUATERNION_STEP, move |e, a| {
            let seen = load_vec(e, a[0]);
            log.borrow_mut()
                .push((QUATERNION_STEP, seen.map(f32::to_bits).to_vec()));
            e.mem.set_f32(a[0], seen[0] + 100.0);
            Ret::default()
        });
    }

    /// A profiling stream with room for 21 records; the TLS lookup answers it. Returns the
    /// stream and the address of its first record.
    fn profile(e: &mut Engine) -> (u32, u32) {
        let stream = e.mem.alloc(0x20);
        let records = e.mem.alloc(0x100);
        e.mem.set_u32(stream + 4, records);
        e.mem.set_u32(stream + 0xc, records + 0x100);
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

    /// The labels of the records written so far.
    fn labels(e: &Engine, stream: u32, records: u32) -> Vec<u32> {
        let count = (e.mem.u32(stream + 4) - records) / 12;
        (0..count).map(|i| e.mem.u32(records + 12 * i)).collect()
    }

    /// An output buffer of `words` words filled with [`UNTOUCHED`].
    fn output(e: &mut Engine, words: u32) -> u32 {
        let at = e.mem.alloc(4 * words);
        for word in 0..words {
            e.mem.set_u32(at + 4 * word, UNTOUCHED);
        }
        at
    }

    /// A 16-bit track list.
    fn track_list(e: &mut Engine, tracks: &[u16]) -> u32 {
        let at = e.mem.alloc(2 * tracks.len() as u32);
        for (index, track) in tracks.iter().enumerate() {
            e.mem.set_u16(at + 2 * index as u32, *track);
        }
        at
    }

    #[test]
    fn sample_tracks_calls_the_partial_sampler_with_the_track_counts() {
        let mut e = engine();
        let calls: Calls = Rc::default();
        let log = Rc::clone(&calls);
        e.register_double(SAMPLE_SLOT, move |_, a| {
            log.borrow_mut().push((SAMPLE_SLOT, a.to_vec()));
            Ret::default()
        });
        let anim = animation(&mut e, 2, 3);
        e.call(
            0x00e3_f920,
            &args![
                anim,
                f32::from_bits(0x7f80_0001),
                0x1000u32,
                0x2000u32,
                0x3000u32
            ],
        );
        assert_eq!(
            calls_to(&calls, SAMPLE_SLOT),
            [vec![anim, 0x7fc0_0001, 2, 0x1000, 3, 0x2000, 0x3000]]
        );
    }

    #[test]
    fn chunk_counts_and_sizes_follow_the_track_counts() {
        let mut e = engine();
        for (transforms, floats, chunks) in [
            (0u32, 0u32, 1u32),
            (2, 0, 2),
            (0, 3, 2),
            (2, 3, 3),
            (u32::MAX, u32::MAX, 1),
        ] {
            let anim = animation(&mut e, transforms, floats);
            let got = e.call(0x00e3_f950, &args![anim, 0u32, 0u32]).u32();
            assert_eq!(got, chunks, "{transforms} {floats}");
        }
        let anim = animation(&mut e, 2, 3);
        // 3 floats: (24 + 31) & !15 = 48; 2 transforms: 192; plus the object's 0x40.
        assert_eq!(e.call(0x00e3_f970, &args![anim]).u32(), 48 + 192 + 0x40);
        e.mem.set_u32(anim + NUM_FLOAT_TRACKS, 1);
        assert_eq!(e.call(0x00e3_f970, &args![anim]).u32(), 32 + 192 + 0x40);
    }

    #[test]
    fn the_frame_count_divides_by_the_transform_or_the_float_tracks() {
        let mut e = engine();
        let anim = animation(&mut e, 4, 3);
        e.mem.set_u32(anim + TOTAL_TRANSFORMS, 42);
        e.mem.set_u32(anim + TOTAL_FLOATS, 100);
        assert_eq!(e.call(0x00e3_f990, &args![anim]).u32(), 10);
        e.mem.set_u32(anim + NUM_TRANSFORM_TRACKS, 0);
        assert_eq!(e.call(0x00e3_f990, &args![anim]).u32(), 33);
    }

    #[test]
    fn get_data_chunks_describes_the_frame_in_the_animation() {
        let mut e = engine();
        let anim = animation(&mut e, 2, 3);
        e.mem.set_u32(anim + TRANSFORMS, 0x0123_4560);
        e.mem.set_u32(anim + FLOATS, 0x0123_4568);
        let out = e.mem.alloc(0x30);
        for offset in [8, 0x14, 0x20] {
            e.mem.set_u8(out + offset, 0xaa);
        }
        e.call(0x00e3_fd20, &args![anim, 3u32, 0u32, out, 3u32]);
        assert_eq!(e.mem.u32(out), anim);
        assert_eq!(e.mem.u32(out + 4), 0x40);
        // Transforms of frame 3: 2 tracks * 3 * 48 bytes in, 96 bytes per track.
        assert_eq!(e.mem.u32(out + 0xc), 0x0123_4560 + 288);
        assert_eq!(e.mem.u32(out + 0x10), 192);
        // Floats of frame 3 start at 0x0123_4568 + 36 = 0x0123_458c: rounded down, nibble 0xc.
        assert_eq!(e.mem.u32(out + 0x18), 0x0123_4580);
        assert_eq!(e.mem.u32(out + 0x1c), (0xc + 24 + 15) & !15);
        assert_eq!(e.mem.u8(out + 0x20), 0xc);
        // The bytes the code never writes keep their value.
        assert_eq!(e.mem.u8(out + 8), 0xaa);
        assert_eq!(e.mem.u8(out + 0x14), 0xaa);
        // Floats only: their record is the second one.
        let anim = animation(&mut e, 0, 3);
        e.mem.set_u32(anim + FLOATS, 0x0123_4568);
        let out = e.mem.alloc(0x30);
        e.call(0x00e3_fd20, &args![anim, 1u32, 0u32, out, 3u32]);
        assert_eq!(e.mem.u32(out + 0xc), 0x0123_4570);
        assert_eq!(e.mem.u8(out + 0x14), 4);
        assert_eq!(e.mem.u32(out + 0x10), (4 + 24 + 15) & !15);
        // Neither: only the object.
        let anim = animation(&mut e, 0, 0);
        let out = e.mem.alloc(0x30);
        e.call(0x00e3_fd20, &args![anim, 1u32, 0u32, out, 3u32]);
        assert_eq!(e.mem.u32(out + 0xc), 0);
    }

    #[test]
    fn transform_tracks_are_blended_with_the_next_frame() {
        let mut e = engine();
        let calls: Calls = Rc::default();
        quaternion_step(&mut e, &calls);
        answer_frame(&mut e, &calls, 1, 0.25);
        let anim = animation(&mut e, 2, 0);
        samples(&mut e, anim, &transforms(), &[]);
        let tracks = track_list(&mut e, &[1, 0xffff, 5, 0]);
        let out = output(&mut e, 4 * 12);
        e.call(0x00e3_f9b0, &args![anim, 1.5f32, tracks, 4u32, out]);
        let asked = calls_to(&calls, GET_FRAME_AND_DELTA);
        assert_eq!(asked.len(), 1);
        assert_eq!(asked[0][..2], [anim, 1.5f32.to_bits()]);
        // Slot 0: track 1, entries 3 and 5: 0.75 * a + 0.25 * b; the rotations are orthogonal.
        assert_eq!(load_vec(&e, out), [2.0, 3.0, 4.0, 5.0]);
        assert_eq!(load_vec(&e, out + 0x10), [100.0, 0.0, 0.25, 0.75]);
        assert_eq!(load_vec(&e, out + 0x20), [1.25; 4]);
        // Slot 3: track 0, entries 2 and 4.
        assert_eq!(load_vec(&e, out + 0x90), [5.0, 10.0, 15.0, 20.0]);
        assert_eq!(load_vec(&e, out + 0xa0), [100.0, 0.0, 0.0, 1.0]);
        assert_eq!(load_vec(&e, out + 0xb0), [1.25; 4]);
        // Slots 1 and 2 (negative and too large track numbers) are untouched.
        for word in 12..36 {
            assert_eq!(e.mem.u32(out + 4 * word), UNTOUCHED);
        }
        // The step saw the blended rotations before it changed them.
        let seen = calls_to(&calls, QUATERNION_STEP);
        assert_eq!(seen.len(), 2);
        assert_eq!(
            seen[0],
            [0.0f32, 0.0, 0.25, 0.75].map(f32::to_bits).to_vec()
        );
    }

    #[test]
    fn opposite_rotations_blend_with_a_negated_weight() {
        let mut e = engine();
        let calls: Calls = Rc::default();
        quaternion_step(&mut e, &calls);
        answer_frame(&mut e, &calls, 0, 0.25);
        let anim = animation(&mut e, 1, 0);
        let upright: Vec4 = [0.0, 0.0, 0.0, 1.0];
        let upside_down: Vec4 = [0.0, 0.0, 0.0, -1.0];
        let zero: Vec4 = [0.0; 4];
        samples(
            &mut e,
            anim,
            &[[zero, upright, zero], [zero, upside_down, zero]],
            &[],
        );
        let tracks = track_list(&mut e, &[0]);
        let out = output(&mut e, 12);
        e.call(0x00e3_f9b0, &args![anim, 0.0f32, tracks, 1u32, out]);
        // 0.75 * (0, 0, 0, 1) - 0.25 * (0, 0, 0, -1).
        assert_eq!(load_vec(&e, out + 0x10), [100.0, 0.0, 0.0, 1.0]);
        // A NaN in the dot product is not below zero: the weight stays positive (0.75 - 0.25).
        e.mem
            .set_f32(e.mem.u32(anim + TRANSFORMS) + 0x30 + 0x10, f32::NAN);
        e.call(0x00e3_f9b0, &args![anim, 0.0f32, tracks, 1u32, out]);
        let rotation = load_vec(&e, out + 0x10);
        assert!(rotation[0].is_nan());
        assert_eq!(rotation[3], 0.5);
    }

    #[test]
    fn float_tracks_are_blended_with_the_next_frame() {
        let mut e = engine();
        let calls: Calls = Rc::default();
        answer_frame(&mut e, &calls, 1, 0.25);
        let anim = animation(&mut e, 0, 3);
        samples(
            &mut e,
            anim,
            &[],
            &[1.0, 2.0, 3.0, 10.0, 20.0, 30.0, 14.0, 24.0, 34.0],
        );
        // Five entries: the first four take the unrolled path, the fifth the tail.
        let tracks = track_list(&mut e, &[1, 0, 0xffff, 7, 2]);
        let out = output(&mut e, 5);
        e.call(0x00e3_fb60, &args![anim, 1.5f32, tracks, 5u32, out]);
        assert_eq!(e.mem.f32(out), 21.0);
        assert_eq!(e.mem.f32(out + 4), 11.0);
        assert_eq!(e.mem.u32(out + 8), UNTOUCHED);
        assert_eq!(e.mem.u32(out + 12), UNTOUCHED);
        assert_eq!(e.mem.f32(out + 16), 31.0);
        assert_eq!(calls_to(&calls, GET_FRAME_AND_DELTA).len(), 1);
    }

    /// Chunk records for [`animation`] data: the object, the transforms of frame 1 (entries 2
    /// and 3, the next frame's following) and the floats of frame 1 at a misaligned address.
    fn chunk_records(e: &mut Engine, anim: u32, transform_array: u32, floats: &[f32]) -> u32 {
        let chunks = e.mem.alloc(0x30);
        e.mem.set_u32(chunks, anim);
        e.mem.set_u32(chunks + 0xc, transform_array + 2 * 48);
        let block = e.mem.alloc(4 * floats.len() as u32 + 16);
        let data = block + 8;
        for (index, value) in floats.iter().enumerate() {
            e.mem.set_f32(data + 4 * index as u32, *value);
        }
        e.mem.set_u32(chunks + 0x18, data & !0xf);
        e.mem.set_u8(chunks + 0x20, (data & 0xf) as u8);
        chunks
    }

    #[test]
    fn partial_sampling_with_chunks_blends_transforms_and_floats() {
        let mut e = engine();
        let calls: Calls = Rc::default();
        quaternion_step(&mut e, &calls);
        let anim = animation(&mut e, 2, 3);
        let (transform_array, _) = samples(&mut e, anim, &transforms(), &[]);
        let chunks = chunk_records(
            &mut e,
            anim,
            transform_array,
            &[10.0, 20.0, 30.0, 14.0, 24.0, 34.0, 40.0, 50.0],
        );
        let transforms_out = output(&mut e, 24);
        let floats_out = output(&mut e, 6);
        e.call(
            0x00e3_fda0,
            &args![
                7u32,
                0.25f32,
                2u32,
                transforms_out,
                5u32,
                floats_out,
                chunks,
                3u32
            ],
        );
        // Transforms 2 and 3 against 4 and 5.
        assert_eq!(load_vec(&e, transforms_out), [5.0, 10.0, 15.0, 20.0]);
        assert_eq!(load_vec(&e, transforms_out + 0x10), [100.0, 0.0, 0.0, 1.0]);
        assert_eq!(load_vec(&e, transforms_out + 0x20), [1.25; 4]);
        assert_eq!(load_vec(&e, transforms_out + 0x30), [2.0, 3.0, 4.0, 5.0]);
        assert_eq!(
            load_vec(&e, transforms_out + 0x40),
            [100.0, 0.0, 0.25, 0.75]
        );
        // Floats: the frame's values weighted 0.75, the ones three further on 0.25.
        let blended: Vec<f32> = (0..5).map(|i| e.mem.f32(floats_out + 4 * i)).collect();
        assert_eq!(blended, [11.0, 21.0, 31.0, 20.5, 30.5]);
        assert_eq!(e.mem.u32(floats_out + 20), UNTOUCHED);
    }

    #[test]
    fn partial_sampling_with_chunks_skips_missing_track_kinds() {
        let mut e = engine();
        let calls: Calls = Rc::default();
        quaternion_step(&mut e, &calls);
        // Floats only: their record is chunk 1.
        let anim = animation(&mut e, 0, 3);
        let chunks = e.mem.alloc(0x30);
        e.mem.set_u32(chunks, anim);
        let block = e.mem.alloc(64);
        for (index, value) in [10.0f32, 20.0, 30.0, 14.0, 24.0, 34.0].iter().enumerate() {
            e.mem.set_f32(block + 4 * index as u32, *value);
        }
        e.mem.set_u32(chunks + 0xc, block);
        let transforms_out = output(&mut e, 12);
        let floats_out = output(&mut e, 4);
        e.call(
            0x00e3_fda0,
            &args![
                0u32,
                0.5f32,
                4u32,
                transforms_out,
                3u32,
                floats_out,
                chunks,
                2u32
            ],
        );
        assert_eq!(e.mem.f32(floats_out), 12.0);
        assert_eq!(e.mem.f32(floats_out + 8), 32.0);
        assert_eq!(e.mem.u32(transforms_out), UNTOUCHED);
        assert!(calls_to(&calls, QUATERNION_STEP).is_empty());
        // Transforms only: no floats are written.
        let anim = animation(&mut e, 1, 0);
        let (transform_array, _) = samples(&mut e, anim, &transforms(), &[]);
        let chunks = e.mem.alloc(0x30);
        e.mem.set_u32(chunks, anim);
        e.mem.set_u32(chunks + 0xc, transform_array);
        let floats_out = output(&mut e, 4);
        let transforms_out = output(&mut e, 12);
        e.call(
            0x00e3_fda0,
            &args![
                0u32,
                0.0f32,
                0u32,
                transforms_out,
                3u32,
                floats_out,
                chunks,
                2u32
            ],
        );
        assert_eq!(e.mem.u32(floats_out), UNTOUCHED);
        assert_eq!(e.mem.u32(transforms_out), UNTOUCHED);
    }

    /// The scene of the `samplePartialTracks` tests: two transform tracks and three float
    /// tracks over three frames, duration 2.
    fn partial_scene(e: &mut Engine) -> u32 {
        let anim = animation(e, 2, 3);
        e.mem.set_f32(anim + DURATION, 2.0);
        let floats = [
            1.0,
            2.0,
            3.0,
            10.0,
            20.0,
            30.0,
            f32::from_bits(0x7f80_0001),
            24.0,
            34.0,
        ];
        samples(e, anim, &transforms(), &floats);
        anim
    }

    #[test]
    fn partial_tracks_after_the_duration_copy_the_last_frame() {
        let mut e = engine();
        let calls: Calls = Rc::default();
        answer_frame(&mut e, &calls, 0, 0.0);
        let (stream, records) = profile(&mut e);
        let anim = partial_scene(&mut e);
        let transform_array = e.mem.u32(anim + TRANSFORMS);
        e.mem.set_u32(transform_array + 4 * 48 + 0x10, 0x7f80_0001);
        for time in [3.0f32, 2.0] {
            let transforms_out = output(&mut e, 24);
            let floats_out = output(&mut e, 5);
            e.call(
                0x00e3_ffc0,
                &args![anim, time, 2u32, transforms_out, 4u32, floats_out, 0u32],
            );
            // Entries 4 and 5; the signalling NaN is copied unchanged by the vector move.
            assert_eq!(load_vec(&e, transforms_out), [8.0, 16.0, 24.0, 32.0]);
            assert_eq!(e.mem.u32(transforms_out + 0x10), 0x7f80_0001);
            assert_eq!(load_vec(&e, transforms_out + 0x30), [5.0, 6.0, 7.0, 8.0]);
            // Floats 6 to 9: the signalling NaN is quieted, the last one is past the data.
            assert_eq!(e.mem.u32(floats_out), 0x7fc0_0001);
            assert_eq!(e.mem.f32(floats_out + 4), 24.0);
            assert_eq!(e.mem.f32(floats_out + 8), 34.0);
            assert_eq!(e.mem.f32(floats_out + 12), 0.0);
            assert_eq!(e.mem.u32(floats_out + 16), UNTOUCHED);
        }
        assert!(calls_to(&calls, GET_FRAME_AND_DELTA).is_empty());
        assert_eq!(
            labels(&e, stream, records),
            [LABEL_SAMPLE, LABEL_END, LABEL_SAMPLE, LABEL_END]
        );
        // Without tracks of either kind nothing is copied but the records are written.
        e.call(
            0x00e3_ffc0,
            &args![anim, 9.0f32, 0u32, 0u32, 0u32, 0u32, 0u32],
        );
        assert_eq!(labels(&e, stream, records).len(), 6);
    }

    #[test]
    fn partial_tracks_before_the_duration_blend_through_the_chunks() {
        let mut e = engine();
        let calls: Calls = Rc::default();
        let (stream, records) = profile(&mut e);
        quaternion_step(&mut e, &calls);
        answer_frame(&mut e, &calls, 1, 0.25);
        let anim = partial_scene(&mut e);
        // Frame 1 of the floats is (10, 20, 30); frame 2 starts with 14 instead of the NaN.
        let float_array = e.mem.u32(anim + FLOATS);
        e.mem.set_f32(float_array + 24, 14.0);
        // A time that is not a number is not above the duration either: it takes this path.
        for time in [1.0f32, f32::NAN] {
            let transforms_out = output(&mut e, 24);
            let floats_out = output(&mut e, 4);
            e.call(
                0x00e3_ffc0,
                &args![anim, time, 2u32, transforms_out, 3u32, floats_out, 0u32],
            );
            assert_eq!(load_vec(&e, transforms_out), [5.0, 10.0, 15.0, 20.0]);
            assert_eq!(load_vec(&e, transforms_out + 0x10), [100.0, 0.0, 0.0, 1.0]);
            assert_eq!(load_vec(&e, transforms_out + 0x30), [2.0, 3.0, 4.0, 5.0]);
            assert_eq!(e.mem.f32(floats_out), 11.0);
            assert_eq!(e.mem.f32(floats_out + 4), 21.0);
            assert_eq!(e.mem.f32(floats_out + 8), 31.0);
            assert_eq!(e.mem.u32(floats_out + 12), UNTOUCHED);
        }
        let asked = calls_to(&calls, GET_FRAME_AND_DELTA);
        assert_eq!(asked[0][..2], [anim, 1.0f32.to_bits()]);
        assert_eq!(
            labels(&e, stream, records),
            [LABEL_SAMPLE, LABEL_END, LABEL_SAMPLE, LABEL_END]
        );
    }

    #[test]
    fn the_partial_chunks_sit_at_0x1c_of_the_aligned_frame() {
        let mut e = engine();
        let calls: Calls = Rc::default();
        profile(&mut e);
        quaternion_step(&mut e, &calls);
        answer_frame(&mut e, &calls, 1, 0.25);
        let anim = partial_scene(&mut e);
        let transforms_out = output(&mut e, 24);
        let floats_out = output(&mut e, 4);
        // The frame block is the next stack local; its bytes stay on the thread's stack after
        // the call (nothing takes that place again before they are read).
        let raw = e.next_stack_local(0x50);
        e.call(
            0x00e3_ffc0,
            &args![anim, 1.0f32, 2u32, transforms_out, 3u32, floats_out, 0u32],
        );
        // The first chunk record's data pointer is the animation's frame data.
        let frame = aligned_frame_at(raw, 0x40);
        assert_ne!(e.mem.u32(frame + 0x1c), 0);
        assert_eq!(e.mem.u8(frame + 0x1c + 8), 0);
    }
}

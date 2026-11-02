//! `Havok SDK/hkasplinecompressedanimationctor.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The constructor of `hkaSplineCompressedAnimation` (Xbox PDB name) builds the compressed data
//! of an uncompressed animation, block by block. The fields it writes are described by offset
//! (unconfirmed, see `hkasplinecompressedanimation`): the type word (`+8`), the frame count of a
//! block (`+0x30`), the frame count of the animation (`+0x28`), the number of blocks (`+0x2c`),
//! the offset where the transform data begins (`+0x34`), the block time and its reciprocal
//! (`+0x38`, `+0x3c`), the time of one frame (`+0x40`), the arrays of block offsets (`+0x44`),
//! float-track offsets (`+0x50`), transform offsets (`+0x5c`) and float offsets (`+0x68`), and the
//! byte array of data (`+0x74`; `+0x78` is its size).
//!
//! The per-track settings are kept in a `PerTrackCompressionParams` (Xbox PDB name, 0x24 bytes):
//! three `hkArray`s, the settings records (0x1c bytes each: four floats, three words of degrees,
//! a float and then bytes the compressor reads at `+0x18` to `+0x1b`), the index of the record of
//! every transform track and the index of the record of every float track.
//!
//! `initialize` keeps its work in a stack frame of the exe; the translation keeps the same
//! layout for the parts the callees see through pointers (the average, minimum and maximum of a
//! lane at `+0x90`, `+0xa0` and `+0xb0`, the per-lane arrays at `+0xc0` and `+0xe8`, the fitted
//! points and knots at `+0x20` and `+0x40`, the text stream at `+0x5c` and its buffer at
//! `+0x190`). SSE code computes lane by lane in `f32`; the x87 code computes in `f64` and rounds
//! when a `float` is stored. A float copied through the x87 stack is quieted
//! (`x87_quiet_bits`). `RSQRTSS` is the host CPU's estimate.
//!
//! The final report of `initialize` goes to the log object (the global at `01268360`) through
//! `hkOstream` (Xbox PDB name) calls; the stream functions are called by address.
//!
//! Not translated: none of these functions has a C++ exception frame.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkp3axissweep::{ARRAY_GROW, ARRAY_RESERVE, MEMORY_ROUTER};
use crate::units::havok_sdk::hkpboxmotion::{add, mul};
use crate::units::havok_sdk::hkpconvexradiusbuilder::normalising_scale;
use crate::units::havok_sdk::hkpsphereshape::{
    load_bits, load_vec, maxps, minps, store_bits, store_vec, zip_with, HALF_F32, THREE_F32,
    ZERO_F32,
};

/// The capacity bits of the capacity-and-flags word of an `hkArray`, and the word of an array
/// that owns nothing.
const CAPACITY_MASK: u32 = 0x3fff_ffff;
const NOT_OWNED: u32 = 0x8000_0000;
/// The size of an element of each kind (table in the exe's data).
const ELEMENT_SIZES: u32 = 0x010e_b3d0;
/// The `float` 1.0 (`01012068`), the `double` 0.5 and the `float` 4294967296.0.
const ONE_F32: u32 = 0x0101_2068;
const HALF_F64: u32 = 0x0101_1588;
const TWO_POW_32_F32: u32 = 0x0109_9378;
/// The vtable stored by the constructor.
const VTABLE: u32 = 0x010d_e0ec;
/// The log object (a global pointer) and the strings of the report.
const LOGGER: u32 = 0x0126_8360;
const TEXT_COMPRESSION: u32 = 0x010e_b4c8;
const TEXT_BLOCKS: u32 = 0x010e_b4b4;
const TEXT_SOURCE_PATH: u32 = 0x010e_b470;
const TEXT_BYTES: u32 = 0x010e_b468;
const TEXT_LARGEST: u32 = 0x010e_b444;
const TEXT_ORIGINAL: u32 = 0x010e_b418;
const TEXT_COMPRESSED: u32 = 0x010e_b428;
const TEXT_RATIO: u32 = 0x010e_b400;
const TEXT_RATIO_END: u32 = 0x010e_b414;
/// The id the log's first call gets with the category text.
const LOG_CATEGORY_ID: u32 = 0x3611_8e94;

/// Functions of other units, called by address (all cdecl unless said otherwise):
/// writes one element of a kind (`kind`, vector, destination); the 8-bit and the 16-bit
/// quantiser (`low`, `high`, `value` floats); the zero padding of an `hkArray<u8>`
/// (`count`, array); the alignment of the array to a size (`alignment`, array) and to the size
/// of a kind (`kind`, array); the pushes of a byte, a word and a float (`value`, array); the
/// curve fit (nine arguments); the quaternion normaliser (`this` = the quaternion);
/// `hkaAnimation::hkaAnimation` (thiscall, `this` and the source animation).
const WRITE_ELEMENT: u32 = 0x00e3_0f50;
const QUANTISE_BYTE: u32 = 0x00e3_0f70;
const QUANTISE_WORD: u32 = 0x00e3_0fb0;
const PAD_ZEROS: u32 = 0x00e3_1800;
const ALIGN: u32 = 0x00e3_1850;
const ALIGN_FOR_KIND: u32 = 0x00e3_18a0;
const PUSH_BYTE: u32 = 0x00e3_17c0;
const PUSH_WORD: u32 = 0x00e3_1ad0;
const PUSH_FLOAT: u32 = 0x00e3_1b00;
const FIT_CURVE: u32 = 0x00e5_8740;
const NORMALISE_QUATERNION: u32 = 0x0056_11c0;
const ANIMATION_CONSTRUCTOR: u32 = 0x00e2_f500;
/// `hkOstream` (Xbox PDB name) functions: the constructor over a buffer (thiscall, `RET 8`:
/// buffer, size), `operator<<` for a string, an `int`, an `unsigned` and a `float` (thiscall,
/// `RET 4`, each answers the stream) and the destructor.
const STREAM_INIT: u32 = 0x00cb_1600;
const STREAM_TEXT: u32 = 0x00cc_01c0;
const STREAM_INT: u32 = 0x00cc_0280;
const STREAM_UNSIGNED: u32 = 0x00cc_02d0;
const STREAM_FLOAT: u32 = 0x00cc_0320;
const STREAM_DROP: u32 = 0x00cc_00e0;
/// `getMaxSizeOfCombinedDataChunks` is the vtable slot `0x2c` of the animation, the size of the
/// compressed data is `00e33330`.
const SLOT_LARGEST_BLOCK: u32 = 0x2c;
const COMPRESSED_SIZE: u32 = 0x00e3_3330;

/// The functions of this file the others call.
const AVERAGE: u32 = 0x00e3_1410;
const LOWEST: u32 = 0x00e3_1470;
const HIGHEST: u32 = 0x00e3_14c0;
const CLASSIFY: u32 = 0x00e3_11f0;
const PACK_THREE: u32 = 0x00e3_0f20;
const PACK_TWO: u32 = 0x00e3_0f40;
const SAMPLE_TRANSFORMS: u32 = 0x00e3_10c0;
const SAMPLE_FLOATS: u32 = 0x00e3_1180;
const RAMP: u32 = 0x00e3_1650;
const STORE_ROTATION: u32 = 0x00e3_19e0;
const STORE_VECTOR: u32 = 0x00e3_1b20;
const STORE_KNOTS: u32 = 0x00e3_1ca0;
const INITIALIZE: u32 = 0x00e3_1d50;
const PARAMS_DESTRUCTOR: u32 = 0x00e3_2cf0;

/// The layout of the stack frame of `initialize` (offsets from the lowest local the exe's
/// `ESP` addresses).
const FRAME_SIZE: u32 = 0x400;
const FITTED_POINTS: u32 = 0x20;
const FITTED_KNOTS: u32 = 0x40;
const SIZE_SLOT: u32 = 0x54;
const STREAM: u32 = 0x5c;
const KNOT_COUNT_SLOT: u32 = 0x74;
const POINT_COUNT_SLOT: u32 = 0x78;
const AVERAGE_SLOT: u32 = 0x90;
const LOWEST_SLOT: u32 = 0xa0;
const HIGHEST_SLOT: u32 = 0xb0;
const POINT_ARRAYS: u32 = 0xc0;
const KNOT_ARRAYS: u32 = 0xe8;
const REFERENCES: u32 = 0x140;
const LOWEST_COPY: u32 = 0x170;
const HIGHEST_COPY: u32 = 0x180;
const MESSAGE_BUFFER: u32 = 0x190;
const MESSAGE_SIZE: u32 = 0x200;

/// The allocator of the memory router (`router + 0x10`).
fn memory_allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

/// An `hkArray` that owns nothing: no data, size 0, capacity word `0x80000000`.
fn init_array(e: &mut Engine, array: u32) {
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 4, 0);
    e.mem.set_u32(array + 8, NOT_OWNED);
}

/// `hkArray<u32>::pushBack`: the router, room for one more through `00c90940` when the size
/// equals the capacity, then the word.
fn push_back_word(e: &mut Engine, array: u32, value: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let capacity = e.mem.u32(array + 8) & CAPACITY_MASK;
    if e.mem.u32(array + 4) == capacity {
        let allocator = e.mem.u32(router + 0x10);
        e.call(ARRAY_GROW, &args![allocator, array, 4u32]);
    }
    let size = e.mem.u32(array + 4);
    let data = e.mem.u32(array);
    e.mem
        .set_u32(data.wrapping_add(size.wrapping_mul(4)), value);
    e.mem.set_u32(array + 4, size.wrapping_add(1));
}

/// `hkArray<u8>::pushBack`.
fn push_back_byte(e: &mut Engine, array: u32, value: u8) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let capacity = e.mem.u32(array + 8) & CAPACITY_MASK;
    if e.mem.u32(array + 4) == capacity {
        let allocator = e.mem.u32(router + 0x10);
        e.call(ARRAY_GROW, &args![allocator, array, 1u32]);
    }
    let at = e.mem.u32(array + 4).wrapping_add(e.mem.u32(array));
    e.mem.set_u8(at, value);
    let size = e.mem.u32(array + 4);
    e.mem.set_u32(array + 4, size.wrapping_add(1));
}

/// Makes room for `wanted` elements of `element_size` bytes (`hkArrayUtil::_reserve`, `00c908c0`)
/// when the capacity is smaller: the new capacity is twice the old one, or `wanted` when that is
/// not smaller than twice the old one (signed compares).
fn reserve_for(e: &mut Engine, array: u32, wanted: i32, element_size: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = (e.mem.u32(array + 8) & CAPACITY_MASK) as i32;
    if capacity < wanted {
        let doubled = capacity.wrapping_add(capacity);
        let target = if wanted < doubled { doubled } else { wanted };
        e.call(
            ARRAY_RESERVE,
            &args![allocator, array, target as u32, element_size],
        );
    }
}

/// Empties an array: the size is cleared, the memory is given back through the allocator's slot
/// `0x10` with `(data, capacity * element_size)` when the array owns it (the top bit of the
/// capacity word clear), then the data pointer is cleared and the capacity word set to
/// `0x80000000`.
fn release_array(e: &mut Engine, array: u32, element_size: u32) {
    let allocator = memory_allocator(e);
    let capacity = e.mem.u32(array + 8);
    e.mem.set_u32(array + 4, 0);
    if capacity & NOT_OWNED == 0 {
        let data = e.mem.u32(array);
        let bytes = (capacity & CAPACITY_MASK).wrapping_mul(element_size);
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 8, NOT_OWNED);
}

/// An array filled from a chunk the allocator's slot `0xc` gives: the size slot holds the bytes
/// wanted (`count * element_size`) and then the bytes given. The array gets the chunk as data,
/// `given / element_size` as size and capacity, and the capacity word `0x80000000` when that is
/// zero. With `count == 0` the empty array stays as it is.
fn allocate_chunk_array(e: &mut Engine, slot: u32, array: u32, count: u32, element_size: u32) {
    let allocator = memory_allocator(e);
    if count == 0 {
        return;
    }
    e.mem.set_u32(slot, count.wrapping_mul(element_size));
    let data = e.vcall(allocator, 0xc, &args![slot]).u32();
    let given = e.mem.u32(slot) as i32;
    let capacity = given / element_size as i32;
    e.mem.set_u32(array, data);
    e.mem.set_u32(array + 4, capacity as u32);
    e.mem.set_u32(
        array + 8,
        if capacity == 0 {
            NOT_OWNED
        } else {
            capacity as u32
        },
    );
}

/// A float as `FLD`/`FSTP` copies it: the bits with a signalling NaN quieted.
fn quiet_word(e: &Engine, at: u32) -> u32 {
    x87_quiet_bits(e.mem.u32(at))
}

/// The value of `FISTP` with the truncating rounding mode: a value out of the range of an `int`
/// (and a NaN) gives `0x80000000`.
fn truncate_to_int(value: f64) -> u32 {
    if value.is_nan() || value >= 2147483648.0 || value <= -2147483649.0 {
        0x8000_0000
    } else {
        value.trunc() as i32 as u32
    }
}

// Translated from 00e30f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Packs three numbers (no name in the engine map; cdecl): `(((c << 4) | b) * 4) | a`.
pub fn fn_00e30f20(_e: &mut Engine, a: u32, b: u32, c: u32) -> u32 {
    ((c << 4) | b).wrapping_mul(4) | a
}

// Translated from 00e30f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Packs two bytes (no name in the engine map; cdecl): `b * 2 | a`, both zero-extended from
/// their low byte.
pub fn fn_00e30f40(_e: &mut Engine, a: u8, b: u8) -> u32 {
    (u32::from(b) * 2) | u32::from(a)
}

// Translated from 00e310c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the transforms of one track into three arrays of four-wide vectors (no name in the
/// engine map; cdecl). The transform of frame `first_frame + i` of the track is the 0x30-byte
/// record number `(first_frame + i) * tracks + track` behind `transforms`; for each of the
/// `frames` frames (nothing when `frames` is not positive) its translation (`+0`) goes to
/// `(x, y, z, 0.0)` in the array whose data pointer is stored behind `translations`, the
/// rotation (`+0x10`, four floats) to `rotations` and the scale (`+0x20`) to `(x, y, z, 0.0)` in
/// `scales`. The destination pointers are read again for every frame. The floats are copied bit
/// for bit; the fourth lane is the `float` at `01011d78`.
pub fn fn_00e310c0(
    e: &mut Engine,
    transforms: u32,
    tracks: i32,
    frames: i32,
    first_frame: i32,
    track: i32,
    translations: u32,
    rotations: u32,
    scales: u32,
) {
    if frames <= 0 {
        return;
    }
    let zero = e.mem.u32(ZERO_F32);
    let stride = (tracks as u32).wrapping_mul(0x30);
    let first = (tracks.wrapping_mul(first_frame).wrapping_add(track) as u32).wrapping_mul(0x30);
    let mut element = transforms.wrapping_add(first);
    for frame in 0..frames as u32 {
        let at = frame.wrapping_mul(16);
        let translation = [
            e.mem.u32(element),
            e.mem.u32(element.wrapping_add(4)),
            e.mem.u32(element.wrapping_add(8)),
            zero,
        ];
        let out = e.mem.u32(translations);
        store_bits(e, out.wrapping_add(at), translation);
        let rotation = load_bits(e, element.wrapping_add(0x10));
        let out = e.mem.u32(rotations);
        store_bits(e, out.wrapping_add(at), rotation);
        let scale = [
            e.mem.u32(element.wrapping_add(0x20)),
            e.mem.u32(element.wrapping_add(0x24)),
            e.mem.u32(element.wrapping_add(0x28)),
            zero,
        ];
        let out = e.mem.u32(scales);
        store_bits(e, out.wrapping_add(at), scale);
        element = element.wrapping_add(stride);
    }
}

// Translated from 00e31180 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the values of one float track into an array of four-wide vectors (no name in the
/// engine map; cdecl). The value of frame `first_frame + i` is the `float` number `(first_frame
/// + i) * tracks + track` behind `values`; for each of the `frames` frames (nothing when
/// `frames` is not positive) the vector `(value, 0.0, 0.0, 0.0)` goes to the array whose data
/// pointer is stored behind `out` (read again for every frame). The value is copied bit for bit;
/// the zeros are the `float` at `01011d78`.
pub fn fn_00e31180(
    e: &mut Engine,
    values: u32,
    tracks: i32,
    frames: i32,
    first_frame: i32,
    track: i32,
    out: u32,
) {
    if frames <= 0 {
        return;
    }
    let zero = e.mem.u32(ZERO_F32);
    let stride = (tracks as u32).wrapping_mul(4);
    let first = (tracks.wrapping_mul(first_frame).wrapping_add(track) as u32).wrapping_mul(4);
    let mut at = values.wrapping_add(first);
    for frame in 0..frames as u32 {
        let value = e.mem.u32(at);
        let data = e.mem.u32(out);
        store_bits(
            e,
            data.wrapping_add(frame.wrapping_mul(16)),
            [value, zero, zero, zero],
        );
        at = at.wrapping_add(stride);
    }
}

/// `|a - b|` as the x87 sequence computes it: the difference is stored as a `float`, then
/// `FABS`.
fn distance(a: f32, b: f32) -> f32 {
    ((f64::from(a) - f64::from(b)) as f32).abs()
}

// Translated from 00e311f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tells which lanes of a vector differ from the bounds and from a reference by more than a
/// tolerance (no name in the engine map; cdecl, the tolerance is a `float` argument): for each
/// of the four lanes `i`, with `d(p, q) = |p - q|`, the lane sets bit `0x10 << i` when
/// `!(d(value, low) <= tolerance)` or `!(d(value, high) <= tolerance)` (a NaN counts as
/// exceeding), else bit `1 << i` when `!(d(value, reference) <= tolerance)`. The answer is the
/// OR of the lanes' bits.
pub fn fn_00e311f0(
    e: &mut Engine,
    value: u32,
    low: u32,
    high: u32,
    tolerance: f32,
    reference: u32,
) -> u8 {
    let mut found = 0u8;
    for lane in 0..4u32 {
        let at = 4 * lane;
        let v = e.mem.f32(value + at);
        let outside = !(distance(v, e.mem.f32(low + at)) <= tolerance)
            || !(distance(v, e.mem.f32(high + at)) <= tolerance);
        if outside {
            found |= 0x10 << lane;
        } else if !(distance(v, e.mem.f32(reference + at)) <= tolerance) {
            found |= 1 << lane;
        }
    }
    found
}

// Translated from 00e31410 (decompiled, FalloutNV.exe 1.4.0.525)
/// The average of the vectors of an `hkArray` (no name in the engine map; cdecl, answers `out`):
/// `out` is cleared, the sum of the elements is added up (`element + sum` per lane, in order)
/// and stored when the size is positive, then `out` is multiplied by the `float`
/// `(1.0 / size)` (computed on the x87 stack, also for a size of 0 or less).
pub fn fn_00e31410(e: &mut Engine, out: u32, array: u32) -> u32 {
    let size = e.mem.u32(array + 4) as i32;
    store_vec(e, out, [0.0; 4]);
    if size > 0 {
        let data = e.mem.u32(array);
        let mut sum = [0.0f32; 4];
        for i in 0..size as u32 {
            sum = add(load_vec(e, data.wrapping_add(i.wrapping_mul(16))), sum);
        }
        store_vec(e, out, sum);
    }
    let reciprocal = (1.0f64 / f64::from(size)) as f32;
    let sum = load_vec(e, out);
    store_vec(e, out, mul([reciprocal; 4], sum));
    out
}

// Translated from 00e31470 (decompiled, FalloutNV.exe 1.4.0.525)
/// The lane-wise minimum of the vectors of an `hkArray` (no name in the engine map; cdecl,
/// answers `out`): `out` gets the first element (four dword copies, also for an empty array),
/// and when the size is greater than 1 the minimum with every later element (`MINPS`: the
/// running value when it is smaller, else the element).
pub fn fn_00e31470(e: &mut Engine, out: u32, array: u32) -> u32 {
    let data = e.mem.u32(array);
    let size = e.mem.u32(array + 4) as i32;
    for lane in 0..4u32 {
        let word = e.mem.u32(data.wrapping_add(4 * lane));
        e.mem.set_u32(out + 4 * lane, word);
    }
    if size > 1 {
        let mut lowest = load_vec(e, out);
        for i in 1..size as u32 {
            lowest = zip_with(
                lowest,
                load_vec(e, data.wrapping_add(i.wrapping_mul(16))),
                minps,
            );
        }
        store_vec(e, out, lowest);
    }
    out
}

// Translated from 00e314c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The lane-wise maximum of the vectors of an `hkArray` (no name in the engine map; cdecl,
/// answers `out`): like `00e31470` with `MAXPS` (the running value when it is greater, else the
/// element).
pub fn fn_00e314c0(e: &mut Engine, out: u32, array: u32) -> u32 {
    let data = e.mem.u32(array);
    let size = e.mem.u32(array + 4) as i32;
    for lane in 0..4u32 {
        let word = e.mem.u32(data.wrapping_add(4 * lane));
        e.mem.set_u32(out + 4 * lane, word);
    }
    if size > 1 {
        let mut highest = load_vec(e, out);
        for i in 1..size as u32 {
            highest = zip_with(
                highest,
                load_vec(e, data.wrapping_add(i.wrapping_mul(16))),
                maxps,
            );
        }
        store_vec(e, out, highest);
    }
    out
}

// Translated from 00e31650 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fills a `float` array with a ramp (no name in the engine map; cdecl): the array whose data
/// pointer is stored behind `array` gets `start` at index 0, `end` at index `count` and, for `k`
/// from 1 to `count - 1`, `(k / (float)count) * (end - start) + start` (computed in `f64` on the
/// x87 stack, the count rounded to a `float`, rounded to a `float` when stored).
pub fn fn_00e31650(e: &mut Engine, count: i32, array: u32, start: f32, end: f32) {
    let data = e.mem.u32(array);
    let start = x87_quiet(start);
    let end = x87_quiet(end);
    e.mem.set_f32(data, start);
    e.mem
        .set_f32(data.wrapping_add((count as u32).wrapping_mul(4)), end);
    let divisor = f64::from(count as f32);
    let difference = f64::from(end) - f64::from(start);
    for k in 1..count {
        let value = ((f64::from(k) / divisor) * difference + f64::from(start)) as f32;
        e.mem
            .set_f32(data.wrapping_add((k as u32).wrapping_mul(4)), value);
    }
}

// Translated from 00e319e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Normalises quaternions and writes them as elements of `kind` (no name in the engine map;
/// thiscall, `RET 0x14`). The data array is `this + 0x74` (`hkArray<u8>`: data, size) and is
/// first aligned for `kind` by `00e318a0`. With a bit of `0xf0` set in `mask`, the vectors
/// number `0` to `last` (nothing when `last` is negative) of the array of vectors whose data
/// pointer is stored behind `vectors` are handled; else with a bit of `0xf` set, the single
/// vector `single`; else nothing. Each vector is copied to a local, normalised by `005611c0`,
/// `size` zero bytes are appended (`00e31800`, `size` the entry of the table `010eb3d0` for the
/// kind) and the vector is written (`00e30f50`) to the data at `(size field of this + 0x78) - size`
/// from the data pointer.
pub fn fn_00e319e0(
    e: &mut Engine,
    this: Ptr,
    mask: u8,
    kind: u32,
    single: u32,
    last: i32,
    vectors: u32,
) {
    let this = this.addr();
    let array = this + 0x74;
    e.with_stack(0x10, |e, local| {
        let local = local.addr();
        if mask & 0xf0 != 0 {
            e.call(ALIGN_FOR_KIND, &args![kind, array]);
            let size = e.mem.u32(ELEMENT_SIZES + 4 * kind);
            if last >= 0 {
                for index in 0..=last as u32 {
                    let data = e.mem.u32(vectors);
                    let words = load_bits(e, data.wrapping_add(index.wrapping_mul(16)));
                    store_bits(e, local, words);
                    e.call(NORMALISE_QUATERNION, &args![local]);
                    e.call(PAD_ZEROS, &args![size, array]);
                    let destination = e
                        .mem
                        .u32(this + 0x78)
                        .wrapping_sub(size)
                        .wrapping_add(e.mem.u32(array));
                    e.call(WRITE_ELEMENT, &args![kind, local, destination]);
                }
            }
        } else if mask & 0xf != 0 {
            e.call(ALIGN_FOR_KIND, &args![kind, array]);
            let words = load_bits(e, single);
            let size = e.mem.u32(ELEMENT_SIZES + 4 * kind);
            store_bits(e, local, words);
            e.call(NORMALISE_QUATERNION, &args![local]);
            e.call(PAD_ZEROS, &args![size, array]);
            let destination = e
                .mem
                .u32(this + 0x78)
                .wrapping_sub(size)
                .wrapping_add(e.mem.u32(array));
            e.call(WRITE_ELEMENT, &args![kind, local, destination]);
        }
    });
}

// Translated from 00e31b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes the bounds and the quantised samples of a vector (no name in the engine map; thiscall,
/// `RET 0x1c`). The data array is `this + 0x74`; it is first aligned to 4 (`00e31850`). For each
/// of the lanes 0 to 2, with `bit = 1 << lane`: when `mask & (bit << 4)` is set the lane's
/// `low` and `high` floats are pushed (`00e31b00`), else when `mask & bit` is set the lane's
/// `mean` float. Then, when `mask & 0xf0` is set and `last` is not negative, for every sample
/// `k` from 0 to `last` and every lane with its animated bit set, the float number `lane + 4 k`
/// of the array of vectors whose data pointer is stored behind `samples` is quantised between
/// `low` and `high` into a byte (`00e30f70`, pushed with `00e317c0`) when `wide` is 0, else into
/// a word (`00e30fb0`, pushed with `00e31ad0`).
pub fn fn_00e31b20(
    e: &mut Engine,
    this: Ptr,
    mask: u8,
    wide: u32,
    mean: u32,
    low: u32,
    high: u32,
    last: i32,
    samples: u32,
) {
    let array = this.addr() + 0x74;
    let bits = u32::from(mask);
    e.call(ALIGN, &args![4u32, array]);
    for lane in 0..3u32 {
        let bit = 1u32 << lane;
        if bits & (bit << 4) != 0 {
            let value = quiet_word(e, low + 4 * lane);
            e.call(PUSH_FLOAT, &args![value, array]);
            let value = quiet_word(e, high + 4 * lane);
            e.call(PUSH_FLOAT, &args![value, array]);
        } else if bits & bit != 0 {
            let value = quiet_word(e, mean + 4 * lane);
            e.call(PUSH_FLOAT, &args![value, array]);
        }
    }
    if mask & 0xf0 != 0 && last >= 0 {
        for sample in 0..=last as u32 {
            for lane in 0..3u32 {
                if bits & ((1 << lane) << 4) == 0 {
                    continue;
                }
                let low_word = quiet_word(e, low + 4 * lane);
                let high_word = quiet_word(e, high + 4 * lane);
                let data = e.mem.u32(samples);
                let value = quiet_word(e, data.wrapping_add((lane + 4 * sample).wrapping_mul(4)));
                if wide == 0 {
                    let byte = e
                        .call(QUANTISE_BYTE, &args![low_word, high_word, value])
                        .u32()
                        & 0xff;
                    e.call(PUSH_BYTE, &args![byte, array]);
                } else {
                    let word = e
                        .call(QUANTISE_WORD, &args![low_word, high_word, value])
                        .u32()
                        & 0xffff;
                    e.call(PUSH_WORD, &args![word, array]);
                }
            }
        }
    }
}

// Translated from 00e31ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes a header word, a header byte and quantised knots (no name in the engine map; thiscall,
/// `RET 0xc`). The data array is `this + 0x74`. The word `first` is pushed (`00e31ad0`), then
/// the byte `second` (`00e317c0`), then for `i` from 0 to `first + second + 1` (nothing when that
/// is negative) the float number `i` of the array whose data pointer is stored behind `values`
/// gives the byte `trunc(value / [this + 0xc] * ([this + 0x28] - 1) + 0.5)` (the x87 stack, the
/// double 0.5 at `01011588`, an out-of-range result gives 0), pushed into the array.
pub fn fn_00e31ca0(e: &mut Engine, this: Ptr, first: u32, second: u32, values: u32) {
    let this = this.addr();
    let array = this + 0x74;
    let last = first.wrapping_add(second).wrapping_add(1) as i32;
    e.call(PUSH_WORD, &args![first, array]);
    e.call(PUSH_BYTE, &args![second, array]);
    if last < 0 {
        return;
    }
    for i in 0..=last as u32 {
        let data = e.mem.u32(values);
        let value = f64::from(e.mem.f32(data.wrapping_add(i.wrapping_mul(4))));
        let scale = f64::from(e.mem.f32(this + 0xc));
        let frames = e.mem.u32(this + 0x28).wrapping_sub(1) as i32;
        let half = e.mem.f64(HALF_F64);
        let key = truncate_to_int((value / scale) * f64::from(frames) + half);
        push_back_byte(e, array, key as u8);
    }
}

/// The values the lane loop of `initialize` shares.
struct TrackContext {
    frame: u32,
    this: u32,
    entry: u32,
    kind: u32,
    frames_in_block: i32,
}

/// The fit of a lane's points (`00e58740`, nine arguments): frames, the knot array's data, the
/// point array's data, the degree, the tolerance, where the knot count goes, the fitted knots'
/// data, where the point count goes and the fitted points' data.
fn fit_curve(e: &mut Engine, ctx: &TrackContext, lane: u32, degree: u32, tolerance: u32) {
    let frame = ctx.frame;
    let knots = e.mem.u32(frame + KNOT_ARRAYS + 12 * lane);
    let points = e.mem.u32(frame + POINT_ARRAYS + 12 * lane);
    let fitted_knots = e.mem.u32(frame + FITTED_KNOTS);
    let fitted_points = e.mem.u32(frame + FITTED_POINTS);
    e.call(
        FIT_CURVE,
        &args![
            ctx.frames_in_block as u32,
            knots,
            points,
            degree,
            tolerance,
            frame + KNOT_COUNT_SLOT,
            fitted_knots,
            frame + POINT_COUNT_SLOT,
            fitted_points
        ],
    );
}

/// Takes the counts the fit wrote: the fitted knots and points are given room for them and
/// get them as sizes. Answers the number of points.
fn take_fit_counts(e: &mut Engine, frame: u32) -> i32 {
    let knot_count = e.mem.u32(frame + KNOT_COUNT_SLOT) as i32;
    reserve_for(e, frame + FITTED_KNOTS, knot_count, 4);
    e.mem.set_u32(frame + FITTED_KNOTS + 4, knot_count as u32);
    let point_count = e.mem.u32(frame + POINT_COUNT_SLOT) as i32;
    reserve_for(e, frame + FITTED_POINTS, point_count, 16);
    e.mem.set_u32(frame + FITTED_POINTS + 4, point_count as u32);
    point_count
}

/// One lane of one track of one block: the statistics of the lane's points, the classification
/// against the tolerance, the bytes of the descriptor, the fit of the animated lanes and the
/// bytes of the lane's data.
fn compress_lane(
    e: &mut Engine,
    ctx: &TrackContext,
    lane: u32,
    degrees: &[u32; 3],
    tolerances: &[u32; 3],
    cursor: &mut u32,
) {
    let frame = ctx.frame;
    let this = ctx.this;
    let entry = ctx.entry;
    let lane_points = frame + POINT_ARRAYS + 12 * lane;
    e.call(AVERAGE, &args![frame + AVERAGE_SLOT, lane_points]);
    e.call(LOWEST, &args![frame + LOWEST_SLOT, lane_points]);
    e.call(HIGHEST, &args![frame + HIGHEST_SLOT, lane_points]);
    let zero_word = e.mem.u32(ZERO_F32);
    let one_word = e.mem.u32(ONE_F32);
    if lane == 1 {
        // The rotation's average is normalised: the four-lane sum of squares is
        // `(w + y) + (z + x)`.
        let average = load_vec(e, frame + AVERAGE_SLOT);
        let squares = mul(average, average);
        let sum = (squares[3] + squares[1]) + (squares[2] + squares[0]);
        let zero: f32 = e.global(ZERO_F32);
        let half: f32 = e.global(HALF_F32);
        let three: f32 = e.global(THREE_F32);
        let scale = normalising_scale(sum, zero, half, three);
        store_vec(e, frame + AVERAGE_SLOT, mul([scale; 4], average));
    }
    // The reference of each lane: translation 0, the identity rotation, scale 1.
    let references = frame + REFERENCES;
    store_bits(e, references, [zero_word; 4]);
    store_bits(
        e,
        references + 0x10,
        [zero_word, zero_word, zero_word, one_word],
    );
    store_bits(
        e,
        references + 0x20,
        [one_word, one_word, one_word, zero_word],
    );
    let tolerance = tolerances[lane as usize];
    let mut descriptor = e
        .call(
            CLASSIFY,
            &args![
                frame + AVERAGE_SLOT,
                frame + LOWEST_SLOT,
                frame + HIGHEST_SLOT,
                tolerance,
                references + 16 * lane
            ],
        )
        .u32() as u8;
    // A float track is always described as at least static.
    if ctx.kind == 1 && descriptor == 0 {
        descriptor = 1;
    }
    if ctx.kind == 0 && lane == 0 {
        let a = u32::from(e.mem.u8(entry + 0x19));
        let b = u32::from(e.mem.u8(entry + 0x18));
        let c = u32::from(e.mem.u8(entry + 0x1a));
        let packed = e.call(PACK_THREE, &args![a, b, c]).u32();
        let data = e.mem.u32(this + 0x74);
        e.mem.set_u8(data.wrapping_add(*cursor), packed as u8);
        *cursor = cursor.wrapping_add(1);
        let data = e.mem.u32(this + 0x74);
        e.mem.set_u8(data.wrapping_add(*cursor), descriptor);
    } else if ctx.kind == 0 {
        let data = e.mem.u32(this + 0x74);
        e.mem.set_u8(data.wrapping_add(*cursor), descriptor);
    } else {
        let flags = u32::from(e.mem.u8(entry + 0x1b));
        let packed = e.call(PACK_TWO, &args![u32::from(descriptor), flags]).u32();
        let data = e.mem.u32(this + 0x74);
        e.mem.set_u8(data.wrapping_add(*cursor), packed as u8);
    }
    *cursor = cursor.wrapping_add(1);

    // The fitted points (four-wide, three per frame) and knots (two more than that) come from
    // chunks of the allocator.
    let total = ctx.frames_in_block.wrapping_mul(3) as u32;
    let points = frame + FITTED_POINTS;
    init_array(e, points);
    allocate_chunk_array(e, frame + SIZE_SLOT, points, total, 16);
    let knots = frame + FITTED_KNOTS;
    init_array(e, knots);
    allocate_chunk_array(e, frame + SIZE_SLOT, knots, total.wrapping_add(2), 4);

    let animated = descriptor & 0xf0;
    if animated != 0 {
        fit_curve(e, ctx, lane, degrees[lane as usize], tolerance);
        let mut point_count = take_fit_counts(e, frame);
        if point_count > ctx.frames_in_block {
            // Too many points: fit again with degree 1.
            fit_curve(e, ctx, lane, 1, tolerance);
            point_count = take_fit_counts(e, frame);
        }
        let knot_count = e.mem.u32(knots + 4) as i32;
        e.call(
            STORE_KNOTS,
            &args![
                this,
                point_count.wrapping_sub(1) as u32,
                knot_count.wrapping_sub(point_count).wrapping_sub(1) as u32,
                knots
            ],
        );
    }
    let point_count = e.mem.u32(points + 4);
    if lane == 1 {
        let kind_byte = u32::from(e.mem.u8(entry + 0x18));
        e.call(
            STORE_ROTATION,
            &args![
                this,
                u32::from(descriptor),
                kind_byte,
                frame + AVERAGE_SLOT,
                point_count.wrapping_sub(1),
                points
            ],
        );
    } else {
        if animated != 0 {
            // The bounds are those of the fitted points.
            let lowest = e.call(LOWEST, &args![frame + LOWEST_COPY, points]).u32();
            let words = load_bits(e, lowest);
            store_bits(e, frame + LOWEST_SLOT, words);
            let highest = e.call(HIGHEST, &args![frame + HIGHEST_COPY, points]).u32();
            let words = load_bits(e, highest);
            store_bits(e, frame + HIGHEST_SLOT, words);
        }
        let wide = if ctx.kind == 0 {
            if lane == 0 {
                e.mem.u8(entry + 0x19)
            } else {
                e.mem.u8(entry + 0x1a)
            }
        } else {
            e.mem.u8(entry + 0x1b)
        };
        e.call(
            STORE_VECTOR,
            &args![
                this,
                u32::from(descriptor),
                u32::from(wide),
                frame + AVERAGE_SLOT,
                frame + LOWEST_SLOT,
                frame + HIGHEST_SLOT,
                point_count.wrapping_sub(1),
                points
            ],
        );
    }
    e.call(ALIGN, &args![4u32, this + 0x74]);
    release_array(e, knots, 4);
    release_array(e, points, 16);
}

// Translated from 00e31d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaSplineCompressedAnimation::initialize` (Xbox PDB name; thiscall, `RET 0xc`): compresses
/// the animation `source` (its vtable slot `0x20` answers the number of frames; `[source +
/// 0x28]` points to the transforms, 0x30 bytes per frame and track, `[source + 0x34]` to the
/// float tracks' values) with the per-track settings `params`; `settings` holds the frame count
/// of a block (`u16`) and a flag byte (when set, the offsets of the tracks inside the block are
/// recorded). It divides the frames into blocks of `[this + 0x30] - 1` steps (a divide error when
/// the block frame count is 1, which the exe raises as a fault), and for each block, track kind
/// (transforms, then floats), track and lane it copies the frames (`00e310c0`, `00e31180`),
/// builds the knot ramp (`00e31650`), classifies the lane (`00e311f0`), writes the descriptor
/// bytes, fits the animated lanes (`00e58740`, again with degree 1 when more points than frames
/// come back) and writes the bounds and samples (`00e319e0`, `00e31b20`, `00e31ca0`). Finally it
/// reports the number of blocks, the size of the largest block, the original and compressed sizes
/// and the ratio to the log. The frame is memory (see the module documentation) because the
/// callees receive pointers into it.
pub fn hka_spline_compressed_animation_initialize(
    e: &mut Engine,
    this: Ptr,
    source: u32,
    params: u32,
    settings: u32,
) {
    let this = this.addr();
    e.with_stack(FRAME_SIZE, |e, frame| {
        initialize_in_frame(e, frame.addr(), this, source, params, settings);
    });
}

fn initialize_in_frame(
    e: &mut Engine,
    frame: u32,
    this: u32,
    source: u32,
    params: u32,
    settings: u32,
) {
    e.mem.set_u32(this + 8, 5);
    e.mem.set_u32(this + 0x80, 0);
    let block_frames = u32::from(e.mem.u16(settings));
    e.mem.set_u32(this + 0x30, block_frames);
    let record_offsets = e.mem.u8(settings + 2) != 0;
    let frames = e.vcall(source, 0x20, &args![]).u32() as i32;
    let steps = (block_frames as i32).wrapping_sub(1);
    let numerator = (block_frames as i32).wrapping_add(frames).wrapping_sub(3);
    if steps == 0 || (numerator == i32::MIN && steps == -1) {
        panic!("integer divide fault at 0x00e31d9b");
    }
    let mut blocks = numerator / steps;
    e.mem.set_u32(this + 0x28, frames as u32);
    if blocks < 1 {
        blocks = 1;
    }
    let duration = f64::from(e.mem.f32(this + 0xc));
    let frame_time = duration / f64::from(frames.wrapping_sub(1));
    let tracks = e.mem.u32(this + 0x10);
    let float_tracks = e.mem.u32(this + 0x14);
    e.mem.set_u32(this + 0x2c, blocks as u32);
    e.mem.set_u32(
        this + 0x34,
        float_tracks
            .wrapping_add(tracks.wrapping_mul(4))
            .wrapping_add(3)
            & 0xffff_fffc,
    );
    let block_time = (f64::from(steps) * frame_time) as f32;
    e.mem.set_f32(this + 0x38, block_time);
    e.mem
        .set_f32(this + 0x3c, (1.0f64 / f64::from(block_time)) as f32);
    e.mem.set_f32(this + 0x40, frame_time as f32);

    let mut frame_offset = 0i32;
    let mut block = 0i32;
    if blocks > 0 {
        loop {
            let used = e.mem.u32(this + 0x78);
            push_back_word(e, this + 0x44, used);
            let remaining = (e.mem.u32(this + 0x28) as i32).wrapping_sub(frame_offset);
            let block_size = e.mem.u32(this + 0x30) as i32;
            let frames_in_block = if block_size < remaining {
                block_size
            } else {
                remaining
            };
            let last_frame = frames_in_block.wrapping_sub(1);
            let mut cursor = e.mem.u32(this + 0x78);
            let frame_count = e.mem.u32(this + 0x28).wrapping_sub(1) as i32;
            let step = f64::from(e.mem.f32(this + 0xc)) / f64::from(frame_count);
            let header = e.mem.u32(this + 0x34);
            let ramp_end = (step * f64::from(last_frame)) as f32;
            e.call(PAD_ZEROS, &args![header, this + 0x74]);

            for kind in 0..2u32 {
                let count = e.mem.u32(this + if kind == 0 { 0x10 } else { 0x14 }) as i32;
                let lanes = if kind == 0 { 3u32 } else { 1 };
                if kind == 1 {
                    let offsets = e.mem.u32(this + 0x44);
                    let start = e
                        .mem
                        .u32(offsets.wrapping_add((block as u32).wrapping_mul(4)));
                    let value = e.mem.u32(this + 0x78).wrapping_sub(start);
                    push_back_word(e, this + 0x50, value);
                }
                let mut track = 0i32;
                if count > 0 {
                    loop {
                        if record_offsets {
                            let offsets = e.mem.u32(this + 0x44);
                            let start = e
                                .mem
                                .u32(offsets.wrapping_add((block as u32).wrapping_mul(4)));
                            let value = e.mem.u32(this + 0x78).wrapping_sub(start);
                            push_back_word(e, this + if kind == 0 { 0x5c } else { 0x68 }, value);
                        }
                        let indices = e.mem.u32(params + if kind == 0 { 0xc } else { 0x18 });
                        let index = e
                            .mem
                            .u32(indices.wrapping_add((track as u32).wrapping_mul(4)));
                        let entry = e
                            .mem
                            .u32(params)
                            .wrapping_add(index.wrapping_mul(7).wrapping_mul(4));
                        for lane in 0..3 {
                            init_array(e, frame + POINT_ARRAYS + 12 * lane);
                            init_array(e, frame + KNOT_ARRAYS + 12 * lane);
                        }
                        for lane in 0..lanes {
                            let points = frame + POINT_ARRAYS + 12 * lane;
                            reserve_for(e, points, frames_in_block, 16);
                            e.mem.set_u32(points + 4, frames_in_block as u32);
                            let knots = frame + KNOT_ARRAYS + 12 * lane;
                            reserve_for(e, knots, frames_in_block, 4);
                            e.mem.set_u32(knots + 4, frames_in_block as u32);
                            e.call(RAMP, &args![last_frame as u32, knots, 0.0f32, ramp_end]);
                        }
                        let mut degrees = [0u32; 3];
                        let mut tolerances = [0u32; 3];
                        if kind == 0 {
                            degrees[0] = u32::from(e.mem.u16(entry + 0x12));
                            tolerances[0] = quiet_word(e, entry + 4);
                            degrees[1] = u32::from(e.mem.u16(entry + 0x10));
                            tolerances[1] = quiet_word(e, entry);
                            tolerances[2] = quiet_word(e, entry + 8);
                            degrees[2] = u32::from(e.mem.u16(entry + 0x14));
                            let transforms = e.mem.u32(source + 0x28);
                            let track_count = e.mem.u32(this + 0x10);
                            e.call(
                                SAMPLE_TRANSFORMS,
                                &args![
                                    transforms,
                                    track_count,
                                    frames_in_block as u32,
                                    frame_offset as u32,
                                    track as u32,
                                    frame + POINT_ARRAYS,
                                    frame + POINT_ARRAYS + 12,
                                    frame + POINT_ARRAYS + 24
                                ],
                            );
                        } else {
                            degrees[0] = u32::from(e.mem.u16(entry + 0x16));
                            tolerances[0] = quiet_word(e, entry + 0xc);
                            let values = e.mem.u32(source + 0x34);
                            let track_count = e.mem.u32(this + 0x14);
                            e.call(
                                SAMPLE_FLOATS,
                                &args![
                                    values,
                                    track_count,
                                    frames_in_block as u32,
                                    frame_offset as u32,
                                    track as u32,
                                    frame + POINT_ARRAYS
                                ],
                            );
                        }
                        let ctx = TrackContext {
                            frame,
                            this,
                            entry,
                            kind,
                            frames_in_block,
                        };
                        for lane in 0..lanes {
                            compress_lane(e, &ctx, lane, &degrees, &tolerances, &mut cursor);
                        }
                        for lane in (0..3).rev() {
                            release_array(e, frame + KNOT_ARRAYS + 12 * lane, 4);
                        }
                        for lane in (0..3).rev() {
                            release_array(e, frame + POINT_ARRAYS + 12 * lane, 16);
                        }
                        track = track.wrapping_add(1);
                        if track >= count {
                            break;
                        }
                    }
                }
            }
            frame_offset = frame_offset.wrapping_add(frames_in_block).wrapping_sub(1);
            e.call(ALIGN, &args![0x10u32, this + 0x74]);
            block = block.wrapping_add(1);
            if block >= e.mem.u32(this + 0x2c) as i32 {
                break;
            }
        }
    }
    report_compression(e, frame, this, source);
}

/// The text the end of `initialize` logs: the category, the number of blocks, the size of the
/// largest block, the original and the compressed size and the ratio, each message through an
/// `hkOstream` over the frame's buffer and then the log's slot `0xc` with `(0, -1, buffer,
/// source path, line)`; the log's slot `0x20` is called at the very end.
fn report_compression(e: &mut Engine, frame: u32, this: u32, source: u32) {
    let stream = frame + STREAM;
    let buffer = frame + MESSAGE_BUFFER;
    let logger = e.mem.u32(LOGGER);
    e.vcall(logger, 0x1c, &args![LOG_CATEGORY_ID, TEXT_COMPRESSION]);

    e.call(STREAM_INIT, &args![stream, buffer, MESSAGE_SIZE]);
    let blocks = e.mem.u32(this + 0x2c);
    let text = e.call(STREAM_TEXT, &args![stream, TEXT_BLOCKS]).u32();
    e.call(STREAM_INT, &args![text, blocks]);
    log_message(e, buffer, 0x16e);
    e.call(STREAM_DROP, &args![stream]);

    e.call(STREAM_INIT, &args![stream, buffer, MESSAGE_SIZE]);
    let largest = e.vcall(this, SLOT_LARGEST_BLOCK, &args![]).u32();
    let text = e.call(STREAM_TEXT, &args![stream, TEXT_LARGEST]).u32();
    let text = e.call(STREAM_INT, &args![text, largest]).u32();
    e.call(STREAM_TEXT, &args![text, TEXT_BYTES]);
    log_message(e, buffer, 0x16f);
    e.call(STREAM_DROP, &args![stream]);

    let original = e
        .mem
        .u32(source + 0x2c)
        .wrapping_mul(16)
        .wrapping_add(e.mem.u32(source + 0x38))
        .wrapping_mul(4);
    e.call(STREAM_INIT, &args![stream, buffer, MESSAGE_SIZE]);
    let compressed = e.call(COMPRESSED_SIZE, &args![this]).u32();
    let text = e.call(STREAM_TEXT, &args![stream, TEXT_ORIGINAL]).u32();
    let text = e.call(STREAM_UNSIGNED, &args![text, original]).u32();
    let text = e.call(STREAM_TEXT, &args![text, TEXT_COMPRESSED]).u32();
    let text = e.call(STREAM_INT, &args![text, compressed]).u32();
    e.call(STREAM_TEXT, &args![text, TEXT_BYTES]);
    log_message(e, buffer, 0x174);
    e.call(STREAM_DROP, &args![stream]);

    e.call(STREAM_INIT, &args![stream, buffer, MESSAGE_SIZE]);
    let mut numerator = f64::from(original as i32);
    if (original as i32) < 0 {
        numerator += f64::from(e.mem.f32(TWO_POW_32_F32));
    }
    let numerator = numerator as f32;
    let compressed = e.call(COMPRESSED_SIZE, &args![this]).u32();
    let ratio = (f64::from(numerator) / f64::from(compressed as i32)) as f32;
    let text = e.call(STREAM_TEXT, &args![stream, TEXT_RATIO]).u32();
    let text = e.call(STREAM_FLOAT, &args![text, x87_quiet(ratio)]).u32();
    e.call(STREAM_TEXT, &args![text, TEXT_RATIO_END]);
    log_message(e, buffer, 0x175);
    e.call(STREAM_DROP, &args![stream]);

    let logger = e.mem.u32(LOGGER);
    e.vcall(logger, 0x20, &args![]);
}

/// The log's slot `0xc` with `(0, -1, text, source path, line)`.
fn log_message(e: &mut Engine, buffer: u32, line: u32) {
    let logger = e.mem.u32(LOGGER);
    e.vcall(
        logger,
        0xc,
        &args![0u32, 0xffff_ffffu32, buffer, TEXT_SOURCE_PATH, line],
    );
}

// Translated from 00e32af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaSplineCompressedAnimation::hkaSplineCompressedAnimation` (Xbox PDB name; thiscall, `RET
/// 0xc`; answers `this`): runs the base constructor `00e2f500` on `source`, stores the vtable
/// and empties the five arrays at `+0x44`, `+0x50`, `+0x5c`, `+0x68` and `+0x74`. Then it builds a
/// `PerTrackCompressionParams` on the stack holding one copy of the 0x1c-byte record `record`,
/// with one index (0) for every transform track and every float track of `source` (the counts at
/// `+0x10` and `+0x14`), calls `initialize` (`00e31d50`) with `(source, params, settings)` and
/// destroys the params (`00e32cf0`).
pub fn hka_spline_compressed_animation_constructor(
    e: &mut Engine,
    this: Ptr,
    source: u32,
    record: u32,
    settings: u32,
) -> u32 {
    let this = this.addr();
    e.call(ANIMATION_CONSTRUCTOR, &args![this, source]);
    e.mem.set_u32(this, VTABLE);
    for at in [0x44, 0x50, 0x5c, 0x68, 0x74] {
        init_array(e, this + at);
    }
    e.with_stack(0x24, |e, local| {
        let params = local.addr();
        for at in [0, 0xc, 0x18] {
            init_array(e, params + at);
        }
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let capacity = e.mem.u32(params + 8) & CAPACITY_MASK;
        if e.mem.u32(params + 4) == capacity {
            let allocator = e.mem.u32(router + 0x10);
            e.call(ARRAY_GROW, &args![allocator, params, 0x1cu32]);
        }
        let size = e.mem.u32(params + 4);
        let slot = e.mem.u32(params).wrapping_add(size.wrapping_mul(0x1c));
        if slot != 0 {
            for word in 0..7u32 {
                let value = e.mem.u32(record + 4 * word);
                e.mem.set_u32(slot + 4 * word, value);
            }
        }
        let size = e.mem.u32(params + 4);
        e.mem.set_u32(params + 4, size.wrapping_add(1));

        let transform_tracks = e.mem.u32(source + 0x10) as i32;
        reserve_for(e, params + 0xc, transform_tracks, 4);
        e.mem.set_u32(params + 0x10, transform_tracks as u32);
        let float_tracks = e.mem.u32(source + 0x14) as i32;
        reserve_for(e, params + 0x18, float_tracks, 4);
        e.mem.set_u32(params + 0x1c, float_tracks as u32);
        clear_entries(e, params + 0xc, source + 0x10);
        clear_entries(e, params + 0x18, source + 0x14);
        e.call(INITIALIZE, &args![this, source, params, settings]);
        e.call(PARAMS_DESTRUCTOR, &args![params]);
    });
    this
}

/// Zeroes the words of an array, as many as the count stored at `count_at` (read again after
/// every word; nothing when it is not positive).
fn clear_entries(e: &mut Engine, array: u32, count_at: u32) {
    if (e.mem.u32(count_at) as i32) <= 0 {
        return;
    }
    let mut index = 0u32;
    loop {
        let data = e.mem.u32(array);
        e.mem.set_u32(data.wrapping_add(index.wrapping_mul(4)), 0);
        index = index.wrapping_add(1);
        if (index as i32) >= (e.mem.u32(count_at) as i32) {
            break;
        }
    }
}

// Translated from 00e32cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaSplineCompressedAnimation::PerTrackCompressionParams::~PerTrackCompressionParams` (Xbox
/// PDB name; thiscall): empties the three arrays at `+0x18`, `+0xc` and `+0` (words, words and
/// 0x1c-byte records): the size is cleared, the memory given back through the allocator's slot
/// `0x10` when the array owns it, the data pointer cleared and the capacity word set to
/// `0x80000000`.
pub fn per_track_compression_params_destructor(e: &mut Engine, this: Ptr) {
    let this = this.addr();
    release_array(e, this + 0x18, 4);
    release_array(e, this + 0xc, 4);
    release_array(e, this, 0x1c);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e3_0f20, fn_00e30f20(u32, u32, u32) -> u32),
        entry!(0x00e3_0f40, fn_00e30f40(u8, u8) -> u32),
        entry!(
            0x00e3_10c0,
            fn_00e310c0(u32, i32, i32, i32, i32, u32, u32, u32)
        ),
        entry!(0x00e3_1180, fn_00e31180(u32, i32, i32, i32, i32, u32)),
        entry!(0x00e3_11f0, fn_00e311f0(u32, u32, u32, f32, u32) -> u8),
        entry!(0x00e3_1410, fn_00e31410(u32, u32) -> u32),
        entry!(0x00e3_1470, fn_00e31470(u32, u32) -> u32),
        entry!(0x00e3_14c0, fn_00e314c0(u32, u32) -> u32),
        entry!(0x00e3_1650, fn_00e31650(i32, u32, f32, f32)),
        entry!(0x00e3_19e0, fn_00e319e0(Ptr, u8, u32, u32, i32, u32)),
        entry!(
            0x00e3_1b20,
            fn_00e31b20(Ptr, u8, u32, u32, u32, u32, i32, u32)
        ),
        entry!(0x00e3_1ca0, fn_00e31ca0(Ptr, u32, u32, u32)),
        entry!(
            0x00e3_1d50,
            hka_spline_compressed_animation_initialize(Ptr, u32, u32, u32)
        ),
        entry!(
            0x00e3_2af0,
            hka_spline_compressed_animation_constructor(Ptr, u32, u32, u32) -> u32
        ),
        entry!(0x00e3_2cf0, per_track_compression_params_destructor(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpsphereshape::Vec4;
    use std::cell::RefCell;
    use std::rc::Rc;

    type FreeLog = Rc<RefCell<Vec<(u32, u32)>>>;
    type Written = Rc<RefCell<Vec<(u32, Vec4, u32)>>>;
    type Vectors = Rc<RefCell<Vec<[Vec4; 3]>>>;

    const FREE_DOUBLE: u32 = 0x0900_0010;
    const CHUNK_DOUBLE: u32 = 0x0900_000c;
    const FRAMES_DOUBLE: u32 = 0x0900_0101;
    const LARGEST_DOUBLE: u32 = 0x0900_0102;
    const LOG_CATEGORY_DOUBLE: u32 = 0x0900_0103;
    const LOG_MESSAGE_DOUBLE: u32 = 0x0900_0104;
    const LOG_FLUSH_DOUBLE: u32 = 0x0900_0105;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.set_global(ZERO_F32, 0.0f32);
        e.set_global(ONE_F32, 1.0f32);
        e.set_global(THREE_F32, 3.0f32);
        e.set_global(HALF_F32, 0.5f32);
        e.mem.set_f64(HALF_F64, 0.5);
        e.set_global(TWO_POW_32_F32, 4294967296.0f32);
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

    /// An `hkArray` of four-wide vectors (size and capacity as many as there are).
    fn vector_array(e: &mut Engine, vectors: &[Vec4]) -> u32 {
        let array = e.mem.alloc(12);
        let data = e.mem.alloc(16 * vectors.len().max(1) as u32);
        for (i, vector) in vectors.iter().enumerate() {
            store_vec(e, data + 16 * i as u32, *vector);
        }
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 4, vectors.len() as u32);
        e.mem.set_u32(array + 8, vectors.len() as u32);
        array
    }

    /// The memory router (`00c85750`) as a double. Its allocator's vtable has the free slot
    /// (`0x10`, which records `(data, bytes)`) and the chunk slot (`0xc`, which gives the bytes
    /// asked for). Answers the allocator and the record of the frees.
    fn install_allocator(e: &mut Engine) -> (u32, FreeLog) {
        let freed: FreeLog = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&freed);
        e.register_double(FREE_DOUBLE, move |_, a| {
            sink.borrow_mut().push((a[1], a[2]));
            Ret::default()
        });
        e.register_double(CHUNK_DOUBLE, |e, a| {
            let bytes = e.mem.u32(a[1]);
            Ret {
                eax: e.mem.alloc(bytes),
                ..Ret::default()
            }
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0xc, CHUNK_DOUBLE);
        e.mem.set_u32(vtable + 0x10, FREE_DOUBLE);
        let allocator = e.mem.alloc(8);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        (allocator, freed)
    }

    /// Moves an array to a fresh block of `capacity` elements (filled with `0xa5` bytes, the
    /// old elements copied) with the capacity flag cleared.
    fn move_array(e: &mut Engine, array: u32, capacity: u32, element: u32) {
        let old = e.mem.u32(array);
        let size = e.mem.u32(array + 4);
        let data = e.mem.alloc(capacity * element);
        for byte in 0..capacity * element {
            e.mem.set_u8(data + byte, 0xa5);
        }
        for byte in 0..size * element {
            let value = e.mem.u8(old + byte);
            e.mem.set_u8(data + byte, value);
        }
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 8, capacity);
    }

    /// `00c90940` (room for one more) and `00c908c0` (room for `wanted`) as doubles that move
    /// the array with `move_array`.
    fn install_array_doubles(e: &mut Engine) {
        e.register_double(ARRAY_GROW, |e, a| {
            let capacity = ((e.mem.u32(a[1] + 8) & CAPACITY_MASK) * 2).max(4);
            move_array(e, a[1], capacity, a[2]);
            Ret::default()
        });
        e.register_double(ARRAY_RESERVE, |e, a| {
            move_array(e, a[1], a[2], a[3]);
            Ret::default()
        });
    }

    #[test]
    fn fn_00e30f20_packs_three_numbers() {
        let mut e = engine();
        assert_eq!(e.call(0x00e3_0f20, &args![1u32, 2u32, 3u32]).u32(), 0xc9);
        assert_eq!(
            e.call(0x00e3_0f20, &args![0x100u32, 0u32, 0u32]).u32(),
            0x100
        );
    }

    #[test]
    fn fn_00e30f40_packs_two_bytes() {
        let mut e = engine();
        assert_eq!(e.call(0x00e3_0f40, &args![0x105u32, 0x203u32]).u32(), 7);
    }

    #[test]
    fn fn_00e310c0_copies_the_transforms_of_a_track_into_three_vector_arrays() {
        let mut e = engine();
        let transforms = e.mem.alloc(6 * 0x30);
        for word in 0..6 * 12u32 {
            e.mem.set_u32(transforms + 4 * word, 0x4000_0000 + word);
        }
        // A signalling NaN is copied unchanged.
        e.mem.set_u32(transforms + 3 * 0x30, 0x7f80_0001);
        let mut pointers = Vec::new();
        for _ in 0..3 {
            let pointer = e.mem.alloc(4);
            let buffer = e.mem.alloc(32);
            e.mem.set_u32(pointer, buffer);
            pointers.push(pointer);
        }
        e.call(
            0x00e3_10c0,
            &args![
                transforms,
                2i32,
                2i32,
                1i32,
                1i32,
                pointers[0],
                pointers[1],
                pointers[2]
            ],
        );
        for (frame, element) in [3u32, 5].into_iter().enumerate() {
            let word = |n: u32| {
                if element * 12 + n == 36 {
                    0x7f80_0001
                } else {
                    0x4000_0000 + element * 12 + n
                }
            };
            let at = 16 * frame as u32;
            let translation = e.mem.u32(pointers[0]) + at;
            let rotation = e.mem.u32(pointers[1]) + at;
            let scale = e.mem.u32(pointers[2]) + at;
            assert_eq!(
                load_bits(&e, translation),
                [word(0), word(1), word(2), 0],
                "translation of frame {frame}"
            );
            assert_eq!(
                load_bits(&e, rotation),
                [word(4), word(5), word(6), word(7)],
                "rotation of frame {frame}"
            );
            assert_eq!(
                load_bits(&e, scale),
                [word(8), word(9), word(10), 0],
                "scale of frame {frame}"
            );
        }
        // Nothing is copied for a frame count that is not positive.
        let first = e.mem.u32(pointers[0]);
        e.mem.set_u32(first, 0xdead);
        e.call(
            0x00e3_10c0,
            &args![
                transforms,
                2i32,
                0i32,
                1i32,
                1i32,
                pointers[0],
                pointers[1],
                pointers[2]
            ],
        );
        assert_eq!(e.mem.u32(first), 0xdead);
    }

    #[test]
    fn fn_00e31180_copies_the_values_of_a_float_track() {
        let mut e = engine();
        let values = e.mem.alloc(40);
        for i in 0..10u32 {
            e.mem.set_u32(values + 4 * i, 0x4100_0000 + i);
        }
        let out = e.mem.alloc(4);
        let buffer = e.mem.alloc(32);
        e.mem.set_u32(out, buffer);
        e.call(0x00e3_1180, &args![values, 3i32, 2i32, 1i32, 2i32, out]);
        // The value of frame 1 + i is the float number (1 + i) * 3 + 2: 5 and 8.
        assert_eq!(load_bits(&e, buffer), [0x4100_0005, 0, 0, 0]);
        assert_eq!(load_bits(&e, buffer + 16), [0x4100_0008, 0, 0, 0]);
    }

    #[test]
    fn fn_00e311f0_flags_the_lanes_outside_the_bounds_or_the_reference() {
        let mut e = engine();
        let mut vectors = Vec::new();
        for lanes in [
            [1.0, 2.0, 3.0, f32::NAN],
            [1.0, 1.0, 3.0, 1.0],
            [1.0, 2.0, 3.0, 1.0],
            [1.0, 2.0, 9.0, 1.0],
        ] {
            let at = e.mem.alloc(16);
            store_vec(&mut e, at, lanes);
            vectors.push(at);
        }
        // Lane 0 is inside, lane 1 is more than 0.5 above the lower bound (0x20), lane 2 is
        // inside the bounds but far from the reference (4) and lane 3 is a NaN (0x80).
        let found = e
            .call(
                0x00e3_11f0,
                &args![vectors[0], vectors[1], vectors[2], 0.5f32, vectors[3]],
            )
            .u32();
        assert_eq!(found, 0xa4);
    }

    #[test]
    fn fn_00e31410_averages_the_vectors() {
        let mut e = engine();
        let array = vector_array(
            &mut e,
            &[
                [1.0, 2.0, 3.0, 4.0],
                [2.0, 4.0, 6.0, 8.0],
                [3.0, 6.0, 9.0, 12.0],
            ],
        );
        let out = e.mem.alloc(16);
        assert_eq!(e.call(0x00e3_1410, &args![out, array]).u32(), out);
        let reciprocal = (1.0f64 / 3.0) as f32;
        assert_eq!(
            load_vec(&e, out),
            [
                reciprocal * 6.0,
                reciprocal * 12.0,
                reciprocal * 18.0,
                reciprocal * 24.0
            ]
        );
        // An empty array: the reciprocal is infinite and the sum zero.
        let empty = vector_array(&mut e, &[]);
        e.call(0x00e3_1410, &args![out, empty]);
        assert!(load_vec(&e, out).iter().all(|lane| lane.is_nan()));
    }

    #[test]
    fn fn_00e31470_takes_the_lane_wise_minimum() {
        let mut e = engine();
        let array = vector_array(
            &mut e,
            &[
                [1.0, 5.0, -2.0, f32::NAN],
                [2.0, 3.0, -1.0, 1.0],
                [0.5, 9.0, -3.0, 2.0],
            ],
        );
        let out = e.mem.alloc(16);
        assert_eq!(e.call(0x00e3_1470, &args![out, array]).u32(), out);
        // A NaN running value gives the element (MINPS answers the second operand).
        assert_eq!(load_vec(&e, out), [0.5, 3.0, -3.0, 1.0]);
        // A single element is copied.
        let single = vector_array(&mut e, &[[1.0, 2.0, 3.0, 4.0]]);
        e.call(0x00e3_1470, &args![out, single]);
        assert_eq!(load_vec(&e, out), [1.0, 2.0, 3.0, 4.0]);
    }

    #[test]
    fn fn_00e314c0_takes_the_lane_wise_maximum() {
        let mut e = engine();
        let array = vector_array(
            &mut e,
            &[
                [1.0, 5.0, -2.0, f32::NAN],
                [2.0, 3.0, -1.0, 1.0],
                [0.5, 9.0, -3.0, 2.0],
            ],
        );
        let out = e.mem.alloc(16);
        assert_eq!(e.call(0x00e3_14c0, &args![out, array]).u32(), out);
        assert_eq!(load_vec(&e, out), [2.0, 9.0, -1.0, 2.0]);
        let single = vector_array(&mut e, &[[1.0, 2.0, 3.0, 4.0]]);
        e.call(0x00e3_14c0, &args![out, single]);
        assert_eq!(load_vec(&e, out), [1.0, 2.0, 3.0, 4.0]);
    }

    #[test]
    fn fn_00e31650_fills_the_ramp_between_start_and_end() {
        let mut e = engine();
        let data = e.mem.alloc(4 * 12);
        let array = e.mem.alloc(4);
        e.mem.set_u32(array, data);
        e.call(0x00e3_1650, &args![9i32, array, 1.0f32, 3.0f32]);
        assert_eq!(e.mem.f32(data), 1.0);
        assert_eq!(e.mem.f32(data + 36), 3.0);
        for k in 1..9u32 {
            let want = ((f64::from(k) / 9.0) * 2.0 + 1.0) as f32;
            assert_eq!(e.mem.f32(data + 4 * k), want, "index {k}");
        }
        // The start goes through the x87 stack: a signalling NaN is quieted.
        e.call(
            0x00e3_1650,
            &args![1i32, array, f32::from_bits(0x7f80_0001), 2.0f32],
        );
        assert_eq!(e.mem.u32(data), 0x7fc0_0001);
        assert_eq!(e.mem.f32(data + 4), 2.0);
    }

    #[test]
    fn fn_00e319e0_normalises_and_writes_the_vectors_for_the_mask() {
        let mut e = engine();
        e.mem.set_u32(ELEMENT_SIZES + 8, 16);
        let written: Written = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&written);
        e.register_double(WRITE_ELEMENT, move |e, a| {
            sink.borrow_mut().push((a[0], load_vec(e, a[1]), a[2]));
            Ret::default()
        });
        e.register_double(NORMALISE_QUATERNION, |e, a| {
            let v = load_vec(e, a[0]);
            store_vec(e, a[0], mul(v, [2.0; 4]));
            Ret::default()
        });
        e.register_double(PAD_ZEROS, |e, a| {
            let size = e.mem.u32(a[1] + 4);
            e.mem.set_u32(a[1] + 4, size + a[0]);
            Ret::default()
        });
        e.register_double(ALIGN_FOR_KIND, |_, _| Ret::default());
        let this = e.mem.alloc(0x100);
        let data = e.mem.alloc(0x100);
        e.mem.set_u32(this + 0x74, data);
        e.mem.set_u32(this + 0x78, 4);
        let vectors = vector_array(&mut e, &[[1.0, 2.0, 3.0, 4.0], [5.0, 6.0, 7.0, 8.0]]);
        let single = e.mem.alloc(16);
        store_vec(&mut e, single, [9.0; 4]);

        // A bit of 0xf0: the vectors 0 to 1 of the array.
        e.call(
            0x00e3_19e0,
            &args![this, 0x10u8, 2u32, single, 1i32, vectors],
        );
        assert_eq!(
            *written.borrow(),
            vec![
                (2, [2.0, 4.0, 6.0, 8.0], data + 4),
                (2, [10.0, 12.0, 14.0, 16.0], data + 20)
            ]
        );
        assert_eq!(calls(&e, PAD_ZEROS), vec![vec![16, this + 0x74]; 2]);

        // Only a bit of 0x0f: the single vector.
        written.borrow_mut().clear();
        e.mem.set_u32(this + 0x78, 4);
        e.call(
            0x00e3_19e0,
            &args![this, 0x0fu8, 2u32, single, 1i32, vectors],
        );
        assert_eq!(*written.borrow(), vec![(2, [18.0; 4], data + 4)]);

        // No bit: nothing happens. A negative last index: only the alignment.
        written.borrow_mut().clear();
        e.call(0x00e3_19e0, &args![this, 0u8, 2u32, single, 1i32, vectors]);
        e.call(
            0x00e3_19e0,
            &args![this, 0xf0u8, 2u32, single, -1i32, vectors],
        );
        assert!(written.borrow().is_empty());
        assert_eq!(calls(&e, ALIGN_FOR_KIND).len(), 3);
        assert_eq!(calls(&e, ALIGN_FOR_KIND)[0], vec![2, this + 0x74]);
    }

    #[test]
    fn fn_00e31b20_writes_the_bounds_and_the_quantised_samples() {
        let mut e = engine();
        e.register_double(QUANTISE_BYTE, |_, a| Ret {
            eax: 0x1234_0000 | (a[2] >> 23),
            ..Ret::default()
        });
        e.register_double(QUANTISE_WORD, |_, a| Ret {
            eax: 0x5678_0000 | (a[2] >> 16),
            ..Ret::default()
        });
        for address in [ALIGN, PUSH_FLOAT, PUSH_BYTE, PUSH_WORD] {
            e.register_double(address, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x100);
        let array = this + 0x74;
        let mean = e.mem.alloc(16);
        store_vec(&mut e, mean, [9.0, 10.0, 11.0, 12.0]);
        let low = e.mem.alloc(16);
        store_vec(&mut e, low, [1.0, 2.0, 3.0, 4.0]);
        let high = e.mem.alloc(16);
        store_vec(&mut e, high, [5.0, 6.0, 7.0, 8.0]);
        let samples = vector_array(&mut e, &[[2.0, 0.0, 0.0, 0.0], [4.0, 0.0, 0.0, 0.0]]);
        let bits = |value: f32| value.to_bits();
        let others = |e: &Engine| -> Vec<(u32, Vec<u32>)> {
            e.call_log
                .clone()
                .unwrap()
                .into_iter()
                .filter(|(address, _)| *address != 0x00e3_1b20)
                .collect()
        };

        // Lane 0 animated (0x10), lane 1 static (0x02), lane 2 neither; bytes.
        e.call(
            0x00e3_1b20,
            &args![this, 0x12u8, 0u32, mean, low, high, 1i32, samples],
        );
        assert_eq!(
            others(&e),
            vec![
                (ALIGN, vec![4, array]),
                (PUSH_FLOAT, vec![bits(1.0), array]),
                (PUSH_FLOAT, vec![bits(5.0), array]),
                (PUSH_FLOAT, vec![bits(10.0), array]),
                (QUANTISE_BYTE, vec![bits(1.0), bits(5.0), bits(2.0)]),
                (PUSH_BYTE, vec![0x80, array]),
                (QUANTISE_BYTE, vec![bits(1.0), bits(5.0), bits(4.0)]),
                (PUSH_BYTE, vec![0x81, array]),
            ]
        );

        // The same with words.
        e.call_log = Some(vec![]);
        e.call(
            0x00e3_1b20,
            &args![this, 0x12u8, 1u32, mean, low, high, 1i32, samples],
        );
        assert_eq!(
            others(&e)[4..],
            vec![
                (QUANTISE_WORD, vec![bits(1.0), bits(5.0), bits(2.0)]),
                (PUSH_WORD, vec![0x4000, array]),
                (QUANTISE_WORD, vec![bits(1.0), bits(5.0), bits(4.0)]),
                (PUSH_WORD, vec![0x4080, array]),
            ]
        );
    }

    #[test]
    fn fn_00e31ca0_writes_the_header_and_the_quantised_knots() {
        let mut e = engine();
        let (allocator, _) = install_allocator(&mut e);
        install_array_doubles(&mut e);
        e.register_double(PUSH_WORD, |_, _| Ret::default());
        e.register_double(PUSH_BYTE, |_, _| Ret::default());
        let this = e.mem.alloc(0x100);
        e.mem.set_f32(this + 0xc, 2.0);
        e.mem.set_u32(this + 0x28, 5);
        let array = this + 0x74;
        let first = e.mem.alloc(1);
        e.mem.set_u32(array, first);
        e.mem.set_u32(array + 8, 1);
        let values = e.mem.alloc(4);
        let data = e.mem.alloc(12);
        for (i, value) in [0.0f32, 0.5, 1e10].into_iter().enumerate() {
            e.mem.set_f32(data + 4 * i as u32, value);
        }
        e.mem.set_u32(values, data);

        // The words 1 + 0 + 1 give the knots 0 to 2: trunc(v / 2 * 4 + 0.5) is 0, 1 and (out of
        // range) 0. The second push finds the array full.
        e.call(0x00e3_1ca0, &args![this, 1u32, 0u32, values]);
        let bytes = e.mem.u32(array);
        assert_eq!(e.mem.u32(array + 4), 3);
        assert_eq!(
            [e.mem.u8(bytes), e.mem.u8(bytes + 1), e.mem.u8(bytes + 2)],
            [0, 1, 0]
        );
        assert_eq!(calls(&e, ARRAY_GROW), vec![vec![allocator, array, 1]]);
        assert_eq!(calls(&e, PUSH_WORD), vec![vec![1, array]]);
        assert_eq!(calls(&e, PUSH_BYTE), vec![vec![0, array]]);

        // A negative last index: only the header.
        e.call(0x00e3_1ca0, &args![this, 0xffff_fffeu32, 0u32, values]);
        assert_eq!(e.mem.u32(array + 4), 3);
        assert_eq!(calls(&e, PUSH_WORD).len(), 2);
    }

    #[test]
    fn initialize_compresses_one_block_of_one_track_and_reports() {
        let mut e = engine();
        let (_, freed) = install_allocator(&mut e);
        install_array_doubles(&mut e);
        e.register_double(PAD_ZEROS, |e, a| {
            let size = e.mem.u32(a[1] + 4);
            e.mem.set_u32(a[1] + 4, size + a[0]);
            Ret::default()
        });
        e.register_double(ALIGN, |_, _| Ret::default());
        e.register_double(FIT_CURVE, |e, a| {
            e.mem.set_u32(a[5], 5);
            e.mem.set_u32(a[7], 3);
            Ret::default()
        });
        e.register_double(STORE_KNOTS, |_, _| Ret::default());
        e.register_double(STORE_ROTATION, |_, _| Ret::default());
        let vectors: Vectors = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&vectors);
        e.register_double(STORE_VECTOR, move |e, a| {
            sink.borrow_mut()
                .push([load_vec(e, a[3]), load_vec(e, a[4]), load_vec(e, a[5])]);
            Ret::default()
        });
        for address in [STREAM_INIT, STREAM_DROP] {
            e.register_double(address, |_, _| Ret::default());
        }
        for address in [STREAM_TEXT, STREAM_INT, STREAM_UNSIGNED, STREAM_FLOAT] {
            e.register_double(address, |_, a| Ret {
                eax: a[0],
                ..Ret::default()
            });
        }

        // The source animation: 4 frames of one transform track.
        let source_vtable = e.mem.alloc(0x40);
        e.mem.set_u32(source_vtable + 0x20, FRAMES_DOUBLE);
        e.register_double(FRAMES_DOUBLE, |_, _| Ret {
            eax: 4,
            ..Ret::default()
        });
        let source = e.mem.alloc(0x40);
        e.mem.set_u32(source, source_vtable);
        let transforms = e.mem.alloc(4 * 0x30);
        for frame in 0..4u32 {
            let at = transforms + 0x30 * frame;
            store_vec(&mut e, at, [frame as f32, 2.0, 3.0, 7.0]);
            store_vec(&mut e, at + 0x10, [0.0, 0.0, 0.0, 1.0]);
            store_vec(&mut e, at + 0x20, [2.0, 1.0, 1.0, 9.0]);
        }
        e.mem.set_u32(source + 0x28, transforms);
        e.mem.set_u32(source + 0x2c, 2);
        e.mem.set_u32(source + 0x38, 3);

        // The animation being built: its vtable answers the largest block size.
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0x2c, LARGEST_DOUBLE);
        e.register_double(LARGEST_DOUBLE, |_, _| Ret {
            eax: 0x40,
            ..Ret::default()
        });
        let this = e.mem.alloc(0x100);
        e.mem.set_u32(this, vtable);
        e.mem.set_f32(this + 0xc, 3.0);
        e.mem.set_u32(this + 0x10, 1);
        let bytes = e.mem.alloc(0x400);
        e.mem.set_u32(this + 0x74, bytes);
        e.mem.set_u32(this + 0x7c, 0x400);
        for at in [0x44, 0x50, 0x5c, 0x68] {
            init_array(&mut e, this + at);
        }

        // One settings record for all tracks: tolerance 0.1, degree 3, bytes 7, 1, 2, 3.
        let record = e.mem.alloc(0x1c);
        for at in [0, 4, 8, 0xc] {
            e.mem.set_f32(record + at, 0.1);
        }
        for at in [0x10, 0x12, 0x14, 0x16] {
            e.mem.set_u16(record + at, 3);
        }
        for (i, byte) in [7u8, 1, 2, 3].into_iter().enumerate() {
            e.mem.set_u8(record + 0x18 + i as u32, byte);
        }
        let indices = e.mem.alloc(4);
        let params = e.mem.alloc(0x24);
        e.mem.set_u32(params, record);
        e.mem.set_u32(params + 0xc, indices);
        let settings = e.mem.alloc(4);
        e.mem.set_u16(settings, 4);
        e.mem.set_u8(settings + 2, 1);

        // The report goes to a log object whose slots are doubles.
        let logger_vtable = e.mem.alloc(0x40);
        e.mem.set_u32(logger_vtable + 0x1c, LOG_CATEGORY_DOUBLE);
        e.mem.set_u32(logger_vtable + 0xc, LOG_MESSAGE_DOUBLE);
        e.mem.set_u32(logger_vtable + 0x20, LOG_FLUSH_DOUBLE);
        for address in [LOG_CATEGORY_DOUBLE, LOG_MESSAGE_DOUBLE, LOG_FLUSH_DOUBLE] {
            e.register_double(address, |_, _| Ret::default());
        }
        let logger = e.mem.alloc(8);
        e.mem.set_u32(logger, logger_vtable);
        e.mem.set_u32(LOGGER, logger);

        e.call(0x00e3_1d50, &args![this, source, params, settings]);

        // The header: type 5, 4 frames in a block, 4 frames, 1 block, the data begins at 4,
        // block time 3 (3 steps of 1), its reciprocal and the frame time.
        assert_eq!(e.mem.u32(this + 8), 5);
        assert_eq!(e.mem.u32(this + 0x30), 4);
        assert_eq!(e.mem.u32(this + 0x28), 4);
        assert_eq!(e.mem.u32(this + 0x2c), 1);
        assert_eq!(e.mem.u32(this + 0x34), 4);
        assert_eq!(e.mem.f32(this + 0x38), 3.0);
        assert_eq!(e.mem.f32(this + 0x3c), (1.0f64 / 3.0) as f32);
        assert_eq!(e.mem.f32(this + 0x40), 1.0);
        // The block starts at 0; the offsets of the transform track within it: 4.
        let block_offsets = e.mem.u32(this + 0x44);
        assert_eq!((e.mem.u32(this + 0x48), e.mem.u32(block_offsets)), (1, 0));
        let track_offsets = e.mem.u32(this + 0x5c);
        assert_eq!((e.mem.u32(this + 0x60), e.mem.u32(track_offsets)), (1, 4));
        let float_offsets = e.mem.u32(this + 0x50);
        assert_eq!((e.mem.u32(this + 0x54), e.mem.u32(float_offsets)), (1, 4));
        // The descriptor bytes: the packed bytes and 0x16 for the translation (x animated, y
        // and z static), 0 for the rotation, 1 for the scale (x differs from 1).
        assert_eq!(
            [
                e.mem.u8(bytes),
                e.mem.u8(bytes + 1),
                e.mem.u8(bytes + 2),
                e.mem.u8(bytes + 3)
            ],
            [0x9d, 0x16, 0, 1]
        );

        // The fit for the animated translation: 4 frames, degree 3, tolerance 0.1.
        let fits = calls(&e, FIT_CURVE);
        assert_eq!(fits.len(), 1);
        assert_eq!(
            (fits[0][0], fits[0][3], fits[0][4]),
            (4, 3, 0.1f32.to_bits())
        );
        // Its 3 points are fewer than the 4 frames; 5 knots: 2, 5 - 3 - 1.
        let knots = calls(&e, STORE_KNOTS);
        assert_eq!(knots.len(), 1);
        assert_eq!(knots[0][..3], [this, 2, 1]);
        let rotation = calls(&e, STORE_ROTATION);
        assert_eq!(rotation.len(), 1);
        assert_eq!(
            (rotation[0][0], rotation[0][1], rotation[0][2]),
            (this, 0, 7)
        );
        assert_eq!(rotation[0][4], 11);
        let stored = calls(&e, STORE_VECTOR);
        assert_eq!(stored.len(), 2);
        assert_eq!(stored[0][..3], [this, 0x16, 1]);
        assert_eq!(stored[0][6], 2);
        assert_eq!(stored[1][..3], [this, 1, 2]);
        assert_eq!(stored[1][6], 11);
        for call in &stored {
            assert_eq!((call[4], call[5]), (call[3] + 0x10, call[3] + 0x20));
        }
        let seen = vectors.borrow();
        assert_eq!(
            seen[0],
            [[1.5, 2.0, 3.0, 0.0], [0.0; 4], [0.0; 4]],
            "animated lane: the bounds are those of the fitted points"
        );
        assert_eq!(
            seen[1],
            [
                [2.0, 1.0, 1.0, 0.0],
                [2.0, 1.0, 1.0, 0.0],
                [2.0, 1.0, 1.0, 0.0]
            ]
        );

        // Memory given back: per lane the knots (14 floats) and the points (12 vectors), then
        // the three knot ramps (4 floats) and the three point arrays (4 vectors).
        let sizes: Vec<u32> = freed.borrow().iter().map(|(_, bytes)| *bytes).collect();
        assert_eq!(
            sizes,
            vec![56, 192, 56, 192, 56, 192, 16, 16, 16, 64, 64, 64]
        );
        assert_eq!(calls(&e, PAD_ZEROS), vec![vec![4, this + 0x74]]);
        assert_eq!(
            calls(&e, ALIGN),
            vec![
                vec![4, this + 0x74],
                vec![4, this + 0x74],
                vec![4, this + 0x74],
                vec![0x10, this + 0x74]
            ]
        );

        // The report: the category, then four messages (blocks, largest, sizes, ratio).
        assert_eq!(
            calls(&e, LOG_CATEGORY_DOUBLE),
            vec![vec![logger, LOG_CATEGORY_ID, TEXT_COMPRESSION]]
        );
        let lines: Vec<(u32, u32, u32)> = calls(&e, LOG_MESSAGE_DOUBLE)
            .iter()
            .map(|message| (message[1], message[2], message[5]))
            .collect();
        assert_eq!(
            lines,
            vec![
                (0, 0xffff_ffff, 0x16e),
                (0, 0xffff_ffff, 0x16f),
                (0, 0xffff_ffff, 0x174),
                (0, 0xffff_ffff, 0x175)
            ]
        );
        assert!(calls(&e, LOG_MESSAGE_DOUBLE)
            .iter()
            .all(|message| message[4] == TEXT_SOURCE_PATH));
        assert_eq!(calls(&e, LOG_FLUSH_DOUBLE), vec![vec![logger]]);
        let stream = calls(&e, STREAM_INIT)[0][0];
        assert_eq!(calls(&e, STREAM_INIT).len(), 4);
        assert_eq!(calls(&e, STREAM_DROP), vec![vec![stream]; 4]);
        let texts: Vec<u32> = calls(&e, STREAM_TEXT).iter().map(|call| call[1]).collect();
        assert_eq!(
            texts,
            vec![
                TEXT_BLOCKS,
                TEXT_LARGEST,
                TEXT_BYTES,
                TEXT_ORIGINAL,
                TEXT_COMPRESSED,
                TEXT_BYTES,
                TEXT_RATIO,
                TEXT_RATIO_END
            ]
        );
        // 1 block, a largest block of 0x40, the original size (2 * 16 + 3) * 4 = 140 and the
        // compressed size 4 + (0 + 1 + 1) * 4 + 0x84 = 144; the ratio is 140 / 144.
        assert_eq!(
            calls(&e, STREAM_INT),
            vec![vec![stream, 1], vec![stream, 0x40], vec![stream, 144]]
        );
        assert_eq!(calls(&e, STREAM_UNSIGNED), vec![vec![stream, 140]]);
        assert_eq!(
            calls(&e, STREAM_FLOAT),
            vec![vec![stream, ((140.0f64 / 144.0) as f32).to_bits()]]
        );
        assert_eq!(calls(&e, COMPRESSED_SIZE).len(), 2);
    }

    #[test]
    fn constructor_builds_the_params_runs_initialize_and_destroys_them() {
        let mut e = engine();
        let (_, freed) = install_allocator(&mut e);
        install_array_doubles(&mut e);
        e.register_double(ANIMATION_CONSTRUCTOR, |_, _| Ret::default());
        let snapshot: Rc<RefCell<Vec<u32>>> = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&snapshot);
        e.register_double(INITIALIZE, move |e, a| {
            let params = a[2];
            let mut seen = Vec::new();
            for array in [params, params + 0xc, params + 0x18] {
                for word in 0..3 {
                    seen.push(e.mem.u32(array + 4 * word));
                }
            }
            let records = e.mem.u32(params);
            for word in 0..7 {
                seen.push(e.mem.u32(records + 4 * word));
            }
            for (array, count) in [(params + 0xc, 3), (params + 0x18, 2)] {
                let data = e.mem.u32(array);
                for index in 0..count {
                    seen.push(e.mem.u32(data + 4 * index));
                }
            }
            *sink.borrow_mut() = seen;
            Ret::default()
        });
        let source = e.mem.alloc(0x20);
        e.mem.set_u32(source + 0x10, 3);
        e.mem.set_u32(source + 0x14, 2);
        let record = e.mem.alloc(0x1c);
        for word in 0..7u32 {
            e.mem.set_u32(record + 4 * word, 0x100 + word);
        }
        let settings = e.mem.alloc(4);
        let this = e.mem.alloc(0x100);
        let result = e
            .call(0x00e3_2af0, &args![this, source, record, settings])
            .u32();
        assert_eq!(result, this);
        assert_eq!(calls(&e, ANIMATION_CONSTRUCTOR), vec![vec![this, source]]);
        assert_eq!(e.mem.u32(this), VTABLE);
        for at in [0x44, 0x50, 0x5c, 0x68, 0x74] {
            assert_eq!(
                [
                    e.mem.u32(this + at),
                    e.mem.u32(this + at + 4),
                    e.mem.u32(this + at + 8)
                ],
                [0, 0, NOT_OWNED]
            );
        }
        // The params as `initialize` saw them: one 0x1c-byte record (room for 4), 3 indices
        // and 2 indices, all zero.
        let seen = snapshot.borrow().clone();
        assert_eq!(seen[1..3], [1, 4]);
        assert_eq!(seen[4..6], [3, 3]);
        assert_eq!(seen[7..9], [2, 2]);
        assert_eq!(
            seen[9..16],
            [0x100, 0x101, 0x102, 0x103, 0x104, 0x105, 0x106]
        );
        assert_eq!(seen[16..21], [0; 5]);
        let initialize = calls(&e, INITIALIZE);
        assert_eq!(initialize.len(), 1);
        assert_eq!(
            (initialize[0][0], initialize[0][1], initialize[0][3]),
            (this, source, settings)
        );
        assert_eq!(calls(&e, PARAMS_DESTRUCTOR), vec![vec![initialize[0][2]]]);
        assert_eq!(
            *freed.borrow(),
            vec![(seen[6], 8), (seen[3], 12), (seen[0], 4 * 0x1c)]
        );
    }

    #[test]
    fn per_track_compression_params_destructor_empties_the_three_arrays() {
        let mut e = engine();
        let (_, freed) = install_allocator(&mut e);
        let this = e.mem.alloc(0x24);
        // (offset, data, size, capacity word): owned, not owned, owned.
        for (at, data, size, capacity) in [
            (0x18, 0x1111, 5, 4),
            (0xc, 0x2222, 6, 0x8000_0003),
            (0, 0x3333, 7, 2),
        ] {
            e.mem.set_u32(this + at, data);
            e.mem.set_u32(this + at + 4, size);
            e.mem.set_u32(this + at + 8, capacity);
        }
        e.call(0x00e3_2cf0, &args![this]);
        assert_eq!(*freed.borrow(), vec![(0x1111, 16), (0x3333, 2 * 0x1c)]);
        for at in [0, 0xc, 0x18] {
            assert_eq!(
                [
                    e.mem.u32(this + at),
                    e.mem.u32(this + at + 4),
                    e.mem.u32(this + at + 8)
                ],
                [0, 0, NOT_OWNED]
            );
        }
    }
}

//! `Havok SDK/hkasplinecompressedanimation.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkaSplineCompressedAnimation` (Xbox PDB name) stores an animation in blocks. The fields the
//! functions read are described by offset (unconfirmed): the number of transform tracks (`+0x10`)
//! and of float tracks (`+0x14`), the number of original frames (`+0x28`), the number of blocks
//! (`+0x2c`), the frame count of a block (`+0x30`, the block holds one frame less than that), a
//! byte offset added to the cursor of the transform data (`+0x34`), two `float`s that turn a block
//! time into a fraction and a key (`+0x3c`, `+0x40`), the arrays of block offsets (`+0x44`),
//! float-track offsets in a block (`+0x50`), transform offsets (`+0x5c`) and float offsets
//! (`+0x68`) (`hkArray`: data, size, capacity and flags), and the data (`+0x74` pointer, `+0x78`
//! size, `+0x7c` capacity and flags).
//!
//! A track is stored as a header of four bytes (the flags of the translation, rotation and scale
//! with their kinds) followed by data a cursor walks over. A vector is either static, or
//! animated between a minimum and a maximum by quantised bytes or 16-bit words; a rotation is
//! read by a decoder chosen by its kind. The curve is evaluated by the degree function in the
//! table at `011be4d0` (the table at `011be4b8` holds the decoders of one element).
//!
//! The x87 code computes in `f64` and rounds when a `float` is stored. Calls to the
//! per-track samplers go through `e.call` so the functions of this file call each other as
//! the exe does.
//!
//! Not translated completely: `00e33a30` (a read of uninitialised stack when the mask names the
//! fourth lane) and `00e33440` (basis arrays that outgrow the exe's frame above degree 3).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkaskeletonmapperutils::IDENTITY_QUATERNION;
use crate::units::havok_sdk::hkp3axissweep::MEMORY_ROUTER;
use crate::units::havok_sdk::hkpboxmotion::{add, mul, sub};
use crate::units::havok_sdk::hkpsphereshape::{
    load_bits, load_vec, profile_record, store_bits, store_vec, Vec4, PROFILE_END,
};

/// Tables in the exe's data: the alignment and the size of an element of each kind, the
/// decoder of one element of each kind (function pointers), the degree functions, the width
/// in bytes of a quantised component of each kind, and the number of set bits of a three-bit
/// value.
const ALIGNMENTS: u32 = 0x010e_b3b8;
const ELEMENT_SIZES: u32 = 0x010e_b3d0;
const DECODERS: u32 = 0x011b_e4b8;
const DEGREE_FUNCTIONS: u32 = 0x011b_e4d0;
const BYTES_PER_COMPONENT: u32 = 0x010e_b4d4;
const COMPONENT_COUNTS: u32 = 0x010e_b73c;
/// The profiling label `"TtSampleSpline"` of `samplePartialTracks`.
const PROFILE_LABEL: u32 = 0x010e_b75c;
/// The identity transform global: its translation, and its scale (`+0x20`).
const IDENTITY_TRANSFORM: u32 = 0x0126_8700;
const IDENTITY_SCALE: u32 = 0x0126_8720;
/// The vtable stored by the destructor and the base destructor it ends with.
const VTABLE: u32 = 0x010d_e0ec;
const ANIMATION_DESTRUCTOR: u32 = 0x00df_b720;
/// `hkaAnimation::getFrameAndDelta` (`00e2f7c0`, thiscall, `RET 0xc`): time, the frame and
/// the delta it writes.
const GET_FRAME_AND_DELTA: u32 = 0x00e2_f7c0;
/// `computePackedNurbsOffsets` (`00e52fa0`, cdecl): data, offsets array, block, value.
const PACKED_OFFSETS: u32 = 0x00e5_2fa0;
/// The functions of this file the others call.
const KNOT_SEARCH: u32 = 0x00e3_2d90;
const KNOT_READ: u32 = 0x00e3_2f10;
const POINT_READ: u32 = 0x00e3_3030;
const VECTOR_READ: u32 = 0x00e3_3960;
const STATIC_OR_ANIMATED: u32 = 0x00e3_3a30;
const TRANSLATION_SAMPLE: u32 = 0x00e3_3e10;
const ROTATION_SAMPLE: u32 = 0x00e3_3dc0;
const SCALE_SAMPLE: u32 = 0x00e3_3e90;
const FLOAT_SAMPLE: u32 = 0x00e3_3f10;
const BLOCK_AND_TIME: u32 = 0x00e3_4720;
const STATIC_FILL: u32 = 0x00e3_46a0;

/// `FISTP` with the truncating control word: values out of range and NaN give the integer
/// indefinite `0x80000000`.
fn truncate(value: f64) -> i32 {
    let whole = value.trunc();
    if !(-2_147_483_648.0..=2_147_483_647.0).contains(&whole) {
        i32::MIN
    } else {
        whole as i32
    }
}

/// A cursor rounded up to a multiple of four (`+ 3 & ~3`).
fn align_four(cursor: u32) -> u32 {
    cursor.wrapping_add(3) & 0xffff_fffc
}

/// The word `index` of a table in the exe's data.
fn table_word(e: &Engine, table: u32, index: u32) -> u32 {
    e.mem.u32(table.wrapping_add(index.wrapping_mul(4)))
}

/// Copies a vector word for word (`MOVAPS`).
fn copy_vector(e: &mut Engine, from: u32, to: u32) {
    let bits = load_bits(e, from);
    store_bits(e, to, bits);
}

// Translated from 00e32d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Finds the knot span (no name in the engine map; cdecl): `knots` is an array of bytes and
/// only the low byte of `key` is compared. The result is `last_index` when
/// `key >= knots[last_index + 1]`, `first_index` when `key <= knots[0]`, else the index `m`
/// found by bisecting between `first_index` and `last_index + 1` with `knots[m] <= key <
/// knots[m + 1]` (the middle is the signed sum halved towards zero).
pub fn fn_00e32d90(e: &mut Engine, last_index: u32, first_index: u32, key: u32, knots: u32) -> u32 {
    let key = key as u8;
    let knot = |e: &Engine, index: u32| e.mem.u8(knots.wrapping_add(index));
    if key >= knot(e, last_index.wrapping_add(1)) {
        return last_index;
    }
    if key <= knot(e, 0) {
        return first_index;
    }
    let mut low = first_index;
    let mut high = last_index.wrapping_add(1);
    loop {
        let sum = high.wrapping_add(low) as i32;
        let middle = (sum.wrapping_sub(sum >> 31) >> 1) as u32;
        if key < knot(e, middle) {
            high = middle;
        } else if key < knot(e, middle.wrapping_add(1)) {
            return middle;
        } else {
            low = middle;
        }
    }
}

// Translated from 00e32ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaSplineCompressedAnimation::getNumOriginalFrames` (Xbox PDB name; thiscall): the word at
/// `this + 0x28`.
pub fn hka_spline_compressed_animation_get_num_original_frames(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x28)
}

// Translated from 00e32ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaSplineCompressedAnimation::getNumDataChunks` (Xbox PDB name; thiscall, `RET 8`): always
/// 3. The two stack words are not read.
pub fn hka_spline_compressed_animation_get_num_data_chunks(
    _e: &mut Engine,
    _this: Ptr,
    _unused_1: u32,
    _unused_2: u32,
) -> u32 {
    3
}

// Translated from 00e32ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaSplineCompressedAnimation::sampleTracks` (Xbox PDB name; thiscall, `RET 0x10`): calls
/// the virtual function at `+0x10` (`samplePartialTracks`) with the time (through the x87
/// stack: a signalling NaN comes out quiet), the number of transform tracks (`this + 0x10`),
/// `transforms`, the number of float tracks (`this + 0x14`), `floats` and `extra`.
pub fn hka_spline_compressed_animation_sample_tracks(
    e: &mut Engine,
    this: Ptr,
    time: f32,
    transforms: u32,
    floats: u32,
    extra: u32,
) {
    let this = this.addr();
    let transform_count = e.mem.u32(this + 0x10);
    let float_count = e.mem.u32(this + 0x14);
    e.vcall(
        this,
        0x10,
        &args![
            x87_quiet(time),
            transform_count,
            transforms,
            float_count,
            floats,
            extra
        ],
    );
}

// Translated from 00e32f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads the knots of a curve (no name in the engine map; cdecl): `*cursor` points at a 16-bit
/// count `N`, a degree byte `b` and `N + b + 2` knot bytes. The span is
/// `fn_00e32d90(N, b, key, knots)`; the `2 * b` floats `knots[span - b + 1 + k] * scale` are
/// written to `floats_out`; `*cursor` moves past the knots (to `knots + N + b + 2`), `*count_out`
/// gets `N`, `*degree_out` gets `b`. Returns the span.
pub fn fn_00e32f10(
    e: &mut Engine,
    cursor: u32,
    count_out: u32,
    degree_out: u32,
    key: u32,
    scale: f32,
    floats_out: u32,
) -> u32 {
    let at = e.mem.u32(cursor);
    let count = u32::from(e.mem.u16(at));
    let degree_at = at.wrapping_add(2);
    let degree = u32::from(e.mem.u8(degree_at));
    let knots = degree_at.wrapping_add(1);
    e.mem.set_u32(cursor, knots);
    let span = e.call(KNOT_SEARCH, &args![count, degree, key, knots]).u32();
    let scale = f64::from(x87_quiet(scale));
    for k in 0..degree.wrapping_mul(2) {
        let index = knots
            .wrapping_sub(degree)
            .wrapping_add(k)
            .wrapping_add(span)
            .wrapping_add(1);
        let knot = f64::from(e.mem.u8(index));
        e.mem.set_f32(
            floats_out.wrapping_add(k.wrapping_mul(4)),
            (knot * scale) as f32,
        );
    }
    let end = degree
        .wrapping_add(count)
        .wrapping_add(1)
        .wrapping_add(knots)
        .wrapping_add(1);
    e.mem.set_u32(cursor, end);
    e.mem.set_u32(count_out, count);
    e.mem.set_u32(degree_out, degree);
    span
}

// Translated from 00e33030 (decompiled, FalloutNV.exe 1.4.0.525)
/// Decodes the control points of a curve (no name in the engine map; cdecl): `*cursor` is
/// rounded up to the alignment of `kind` (table `010eb3b8`); for `k` in `0..=degree` the decoder
/// of the kind (table `011be4b8`) is called with the element at
/// `*cursor + (span - degree) * size + k * size` (size from table `010eb3d0`) and
/// `out + 16 * k`; at the end `*cursor` moves on by `(count + 1) * size`. Nothing is decoded
/// when `degree` is negative.
pub fn fn_00e33030(
    e: &mut Engine,
    cursor: u32,
    kind: u32,
    count: u32,
    degree: i32,
    span: u32,
    out: u32,
) {
    let alignment = table_word(e, ALIGNMENTS, kind);
    let at = e.mem.u32(cursor);
    e.mem.set_u32(
        cursor,
        at.wrapping_add(alignment).wrapping_sub(1) & !alignment.wrapping_sub(1),
    );
    let size = table_word(e, ELEMENT_SIZES, kind);
    if degree >= 0 {
        let mut offset = span.wrapping_sub(degree as u32).wrapping_mul(size);
        let mut target = out;
        for _ in 0..=degree {
            let source = e.mem.u32(cursor).wrapping_add(offset);
            let decoder = table_word(e, DECODERS, kind);
            e.call(decoder, &args![source, target]);
            offset = offset.wrapping_add(size);
            target = target.wrapping_add(16);
        }
    }
    let advance = count.wrapping_add(1).wrapping_mul(size);
    let at = e.mem.u32(cursor);
    e.mem.set_u32(cursor, at.wrapping_add(advance));
}

// Translated from 00e33330 (decompiled, FalloutNV.exe 1.4.0.525)
/// A size computed from the animation (no name in the engine map; thiscall):
/// `[this + 0x78] + ([this + 0x6c] + [this + 0x60] + [this + 0x48]) * 4 + 0x84`.
pub fn fn_00e33330(e: &mut Engine, this: Ptr) -> u32 {
    let this = this.addr();
    let sizes = e
        .mem
        .u32(this + 0x6c)
        .wrapping_add(e.mem.u32(this + 0x60))
        .wrapping_add(e.mem.u32(this + 0x48));
    e.mem
        .u32(this + 0x78)
        .wrapping_add(sizes.wrapping_mul(4))
        .wrapping_add(0x84)
}

// Translated from 00e33350 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaSplineCompressedAnimation::getDataChunks` (Xbox PDB name; thiscall, `RET 0x10`; the
/// last stack word is not read): fills the chunk description at `chunk` for the block of
/// `frame` and the time. `getBlockAndTime` gives the block; its start is `[this + 0x44][block]`
/// and its end the next entry (the data size `[this + 0x78]` for the last block); the data
/// address `[this + 0x74] + start` is split into a 16-byte aligned address (`+0xc`) and the
/// remainder (byte at `+0x14`); `+0x10` is the remainder plus the size rounded up to 16, `+0`
/// is `this` and `+4` is `0x90`. The address `[this + 0x50] + block * 4` is split the same way
/// (aligned address at `+0x18`, remainder byte at `+0x20`, and `+0x1c` is the remainder plus
/// `0x13` rounded down to 16).
pub fn hka_spline_compressed_animation_get_data_chunks(
    e: &mut Engine,
    this: Ptr,
    frame: u32,
    time: f32,
    chunk: u32,
    _unused_3: u32,
) {
    let this = this.addr();
    e.mem.set_u32(chunk, this);
    e.mem.set_u32(chunk + 4, 0x90);
    let block = e.with_stack(0xc, |e, slots| {
        let slots = slots.addr();
        e.call(
            BLOCK_AND_TIME,
            &args![this, frame, x87_quiet(time), slots, slots + 4, slots + 8],
        );
        e.mem.u32(slots)
    });
    let offsets = e.mem.u32(this + 0x44);
    let start_at = offsets.wrapping_add(block.wrapping_mul(4));
    let end = if block == e.mem.u32(this + 0x2c).wrapping_sub(1) {
        e.mem.u32(this + 0x78)
    } else {
        e.mem.u32(start_at.wrapping_add(4))
    };
    let start = e.mem.u32(start_at);
    let size = end.wrapping_sub(start).wrapping_add(0xf) & 0xffff_fff0;
    let address = e.mem.u32(this + 0x74).wrapping_add(start);
    let remainder = (address & 0xf) as u8;
    e.mem.set_u32(chunk + 0xc, address & 0xffff_fff0);
    e.mem.set_u8(chunk + 0x14, remainder);
    e.mem.set_u32(
        chunk + 0x10,
        u32::from(remainder).wrapping_add(size).wrapping_add(0xf) & 0xffff_fff0,
    );
    let float_address = e.mem.u32(this + 0x50).wrapping_add(block.wrapping_mul(4));
    let float_remainder = (float_address & 0xf) as u8;
    e.mem.set_u8(chunk + 0x20, float_remainder);
    e.mem.set_u32(chunk + 0x18, float_address & 0xffff_fff0);
    e.mem.set_u32(
        chunk + 0x1c,
        u32::from(float_remainder).wrapping_add(0x13) & 0xffff_fff0,
    );
}

// Translated from 00e33400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaSplineCompressedAnimation::getMaxSizeOfCombinedDataChunks` (Xbox PDB name; thiscall): the
/// largest block size, as the difference of neighbouring entries of the block offsets
/// (`[this + 0x44]`, with the data size `[this + 0x78]` after the last), walked from the last
/// block down and compared as signed numbers starting from 0; the result is that maximum rounded
/// up to 16, plus `0xa0`.
pub fn hka_spline_compressed_animation_get_max_size_of_combined_data_chunks(
    e: &mut Engine,
    this: Ptr,
) -> u32 {
    let this = this.addr();
    let mut index = (e.mem.u32(this + 0x48) as i32).wrapping_sub(1);
    let mut previous = e.mem.u32(this + 0x78);
    let offsets = e.mem.u32(this + 0x44);
    let mut largest = 0i32;
    while index >= 0 {
        let current = e
            .mem
            .u32(offsets.wrapping_add((index as u32).wrapping_mul(4)));
        let size = previous.wrapping_sub(current) as i32;
        largest = largest.max(size);
        previous = current;
        index -= 1;
    }
    ((largest as u32).wrapping_add(0xf) & 0xffff_fff0).wrapping_add(0xa0)
}

// Translated from 00e33440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Evaluates a B-spline of any degree (no name in the engine map; cdecl; no caller is known):
/// the basis functions are built the usual way from `left[j] = u - knots[degree - j]` and
/// `right[j] = knots[degree - 1 + j] - u` (each stored as a `float`, the sums and quotients
/// done in `f64` and stored as `float` again), and the result is
/// `sum(N[r] * points[r])` over the 16-byte points, each product added to the sum so far
/// (`ADDPS`). A negative degree gives zeros; degree 0 gives `points[0]`. The arrays are kept
/// in one frame the way the exe lays them out (`saved` at word 6, the temporary at 7,
/// `N` from 8, `right` from 12, `left` from 16), so they overlap for degrees above 3 as they do
/// there.
// Translation incomplete: 00e33440 - the exe keeps the basis arrays in a fixed 0x44-byte frame; from degree 4 on they run over the end of it into the saved registers, which is not modelled.
pub fn fn_00e33440(e: &mut Engine, time: f32, degree: i32, knots: u32, points: u32, out: u32) {
    const SAVED: usize = 6;
    const TEMPORARY: usize = 7;
    const BASIS: usize = 8;
    const RIGHT: usize = 12;
    const LEFT: usize = 16;
    let mut frame = vec![0.0f32; LEFT + degree.max(0) as usize + 2];
    frame[BASIS] = 1.0;
    if degree >= 1 {
        let time = f64::from(x87_quiet(time));
        let knot = |e: &Engine, index: i32| {
            f64::from(
                e.mem
                    .f32(knots.wrapping_add((index as u32).wrapping_mul(4))),
            )
        };
        for j in 1..=degree {
            let step = j as usize;
            frame[LEFT + step] = (time - knot(e, degree - j)) as f32;
            frame[RIGHT + step] = (knot(e, degree - 1 + j) - time) as f32;
            frame[SAVED] = 0.0;
            for r in 0..step {
                let sum = f64::from(frame[RIGHT + r + 1]) + f64::from(frame[LEFT + step - r]);
                frame[TEMPORARY] = (f64::from(frame[BASIS + r]) / sum) as f32;
                let temporary = f64::from(frame[TEMPORARY]);
                frame[BASIS + r] =
                    (f64::from(frame[RIGHT + r + 1]) * temporary + f64::from(frame[SAVED])) as f32;
                frame[SAVED] = (temporary * f64::from(frame[LEFT + step - r])) as f32;
            }
            frame[BASIS + step] = frame[SAVED];
        }
    }
    let mut sum: Vec4 = [0.0; 4];
    if degree >= 0 {
        for r in 0..=degree as u32 {
            let weight = [frame[BASIS + r as usize]; 4];
            let point = load_vec(e, points.wrapping_add(r.wrapping_mul(16)));
            sum = add(mul(weight, point), sum);
        }
    }
    store_vec(e, out, sum);
}

// Translated from 00e33610 (decompiled, FalloutNV.exe 1.4.0.525)
/// Evaluates a linear curve (no name in the engine map; cdecl; entry 1 of the degree table):
/// `d1 = u - knots[0]` and `d2 = knots[1] - u` (stored as `float`), `t = d1 / (d2 + d1)` and the
/// result is `(1 - t) * points[0] + points[1] * t` lane by lane. The degree word is not read.
pub fn fn_00e33610(e: &mut Engine, time: f32, _unused_1: u32, knots: u32, points: u32, out: u32) {
    let time = f64::from(x87_quiet(time));
    let first = f64::from(e.mem.f32(knots));
    let second = f64::from(e.mem.f32(knots + 4));
    let d1 = (time - first) as f32;
    let d2 = (second - time) as f32;
    let t = [d1 / (d2 + d1); 4];
    let first_point = load_vec(e, points);
    let second_point = load_vec(e, points + 0x10);
    store_vec(
        e,
        out,
        add(mul(sub([1.0; 4], t), first_point), mul(second_point, t)),
    );
}

// Translated from 00e33670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Evaluates a quadratic curve (no name in the engine map; cdecl; entry 2 of the degree table):
/// with `l0 = u - k0`, `l1 = u - k1`, `r2 = k2 - u`, `r3 = k3 - u` (each stored as `float`) the
/// `float` weights are `d = 1 / (r2 + l1)`, `a = (0 + r2 * d) / (r2 + l0)`, `b0 = 0 + r2 * a`,
/// `c = (l1 * d) / (r3 + l1)`, `b2 = l1 * c`, `b1 = l0 * a + r3 * c`, and the result is
/// `(b1 * p1 + b0 * p0) + b2 * p2` lane by lane. The degree word is not read.
pub fn fn_00e33670(e: &mut Engine, time: f32, _unused_1: u32, knots: u32, points: u32, out: u32) {
    let time = f64::from(x87_quiet(time));
    let knot = |e: &Engine, index: u32| f64::from(e.mem.f32(knots + 4 * index));
    let l1 = (time - knot(e, 1)) as f32;
    let r2 = (knot(e, 2) - time) as f32;
    let d = 1.0f32 / (r2 + l1);
    let l0 = (time - knot(e, 0)) as f32;
    let r3 = (knot(e, 3) - time) as f32;
    let a = (0.0f32 + r2 * d) / (r2 + l0);
    let b0 = 0.0f32 + r2 * a;
    let c = (l1 * d) / (r3 + l1);
    let b2 = l1 * c;
    let b1 = l0 * a + r3 * c;
    let p0 = load_vec(e, points);
    let p1 = load_vec(e, points + 0x10);
    let p2 = load_vec(e, points + 0x20);
    let sum = add(mul([b1; 4], p1), mul([b0; 4], p0));
    store_vec(e, out, add(sum, mul([b2; 4], p2)));
}

// Translated from 00e33760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Evaluates a cubic curve (no name in the engine map; cdecl; entry 3 of the degree table):
/// with `a0 = u - k2`, `a1 = u - k1`, `a2 = u - k0` and `r3 = k3 - u`, `r4 = k4 - u`,
/// `r5 = k5 - u` (each stored as `float`) the `float` weights are `d = 1 / (r3 + a0)`,
/// `f1 = (0 + r3 * d) / (r3 + a1)`, `f2 = (a0 * d) / (r4 + a0)`, `f3 = (0 + r3 * f1) / (r3 + a2)`,
/// `b0 = 0 + r3 * f3`, `f4 = (a1 * f1 + r4 * f2) / (r4 + a1)`, `b1 = a2 * f3 + r4 * f4`,
/// `f5 = (a0 * f2) / (r5 + a0)`, `b2 = a1 * f4 + r5 * f5`, `b3 = a0 * f5`, and the result is
/// `((b1 * p1 + b0 * p0) + b2 * p2) + b3 * p3` lane by lane. The degree word is not read.
pub fn fn_00e33760(e: &mut Engine, time: f32, _unused_1: u32, knots: u32, points: u32, out: u32) {
    let time = f64::from(x87_quiet(time));
    let knot = |e: &Engine, index: u32| f64::from(e.mem.f32(knots + 4 * index));
    let a0 = (time - knot(e, 2)) as f32;
    let r3 = (knot(e, 3) - time) as f32;
    let d = 1.0f32 / (r3 + a0);
    let a1 = (time - knot(e, 1)) as f32;
    let r4 = (knot(e, 4) - time) as f32;
    let f1 = (0.0f32 + r3 * d) / (r3 + a1);
    let a2 = (time - knot(e, 0)) as f32;
    let f2 = (a0 * d) / (r4 + a0);
    let r5 = (knot(e, 5) - time) as f32;
    let f3 = (0.0f32 + r3 * f1) / (r3 + a2);
    let b0 = 0.0f32 + r3 * f3;
    let f4 = (a1 * f1 + r4 * f2) / (r4 + a1);
    let b1 = a2 * f3 + r4 * f4;
    let f5 = (a0 * f2) / (r5 + a0);
    let b2 = a1 * f4 + r5 * f5;
    let b3 = a0 * f5;
    let p0 = load_vec(e, points);
    let p1 = load_vec(e, points + 0x10);
    let p2 = load_vec(e, points + 0x20);
    let p3 = load_vec(e, points + 0x30);
    let sum = add(mul([b1; 4], p1), mul([b0; 4], p0));
    let sum = add(sum, mul([b2; 4], p2));
    store_vec(e, out, add(sum, mul([b3; 4], p3)));
}

// Translated from 00e33960 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads one rotation or vector of a track (no name in the engine map; cdecl): `mask` is a
/// byte. With any of its high four bits set the curve is animated: the knots are read by
/// `fn_00e32f10(cursor, &count, &degree, key, scale, &knots)` (the scale through the x87 stack),
/// the points by `fn_00e33030(cursor, kind, count, degree, span, &points)` and the degree
/// function of the table at `011be4d0` is called with the time, the degree, the knots, the
/// points and `out`. With only the low four bits set `*cursor` is rounded up to the alignment of
/// `kind` and the decoder of the kind is called with that address and `out`, then `*cursor`
/// moves by the element size. With neither, `out` gets the identity quaternion (`010c71b0`).
pub fn fn_00e33960(
    e: &mut Engine,
    cursor: u32,
    key: u32,
    scale: f32,
    time: f32,
    mask: u8,
    kind: u32,
    out: u32,
) {
    if mask & 0xf0 != 0 {
        // Locals: the degree, the count, the knots (24 bytes) and the points (64 bytes).
        e.with_stack(0x60, |e, frame| {
            let frame = frame.addr();
            let (degree_at, count_at, knots_at, points_at) =
                (frame, frame + 4, frame + 8, frame + 0x20);
            let span = e
                .call(
                    KNOT_READ,
                    &args![cursor, count_at, degree_at, key, x87_quiet(scale), knots_at],
                )
                .u32();
            let degree = e.mem.u32(degree_at);
            let count = e.mem.u32(count_at);
            e.call(
                POINT_READ,
                &args![cursor, kind, count, degree, span, points_at],
            );
            let function = table_word(e, DEGREE_FUNCTIONS, degree);
            e.call(
                function,
                &args![x87_quiet(time), degree, knots_at, points_at, out],
            );
        });
    } else if mask & 0xf != 0 {
        let alignment = table_word(e, ALIGNMENTS, kind);
        let at = e.mem.u32(cursor);
        let aligned = at.wrapping_add(alignment).wrapping_sub(1) & !alignment.wrapping_sub(1);
        e.mem.set_u32(cursor, aligned);
        let decoder = table_word(e, DECODERS, kind);
        e.call(decoder, &args![aligned, out]);
        let size = table_word(e, ELEMENT_SIZES, kind);
        let at = e.mem.u32(cursor);
        e.mem.set_u32(cursor, at.wrapping_add(size));
    } else {
        copy_vector(e, IDENTITY_QUATERNION, out);
    }
}

// Translated from 00e33a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads one three-component vector of a track (no name in the engine map; cdecl): `mask`
/// is the low byte of the word (bits 0 to 2: static x, y, z; bits 4 to 6: animated x, y, z
/// between a minimum and a maximum). With an animated bit set the knots are read as in
/// `fn_00e33960`; `*cursor` is rounded up to 4, then for each of x, y, z the static value (one
/// `float`) or the minimum and maximum (two `float`s, only when the static bit is clear) are
/// read, each through the x87 stack. Without an animated bit `fn_00e346a0(mask, statics, defaults,
/// out)` finishes it. Otherwise `*cursor` is rounded up to 2 and the quantised data (one byte per
/// component for kind 0, else 16-bit words, table `010eb4d4`; the number of animated components
/// from table `010eb73c`) is read from `(span - degree) * width * components` on: for each of the
/// `degree + 1` points the animated component is `((max - min) * n / 255) + min` (`/ 65535` for
/// kinds other than 0; computed in `f64`, stored as `float`), the static lanes copy the static
/// value, the lanes with neither bit copy `defaults`; then the degree function is called with the
/// time (through the x87 stack), the degree, the knots, the points and `out`, and `*cursor` moves
/// by `(count + 1) * width * components`.
// Translation incomplete: 00e33a30 - the exe leaves the fourth lane of its static vector and of its points unwritten (uninitialised stack) when the mask names that lane (bit 3 or bit 7); the model reads zero there.
pub fn fn_00e33a30(
    e: &mut Engine,
    cursor: u32,
    key: u32,
    scale: f32,
    time: f32,
    mask: u32,
    kind: u32,
    defaults: u32,
    out: u32,
) {
    let flags = mask as u8;
    // Locals: the degree, the count, the static vector, the knots and the points.
    e.with_stack(0x80, |e, frame| {
        let frame = frame.addr();
        let (degree_at, count_at, statics_at) = (frame, frame + 4, frame + 0x10);
        let (knots_at, points_at) = (frame + 0x20, frame + 0x40);
        let mut span = 0;
        if flags & 0xf0 != 0 {
            span = e
                .call(
                    KNOT_READ,
                    &args![cursor, count_at, degree_at, key, x87_quiet(scale), knots_at],
                )
                .u32();
        }
        let degree = e.mem.i32(degree_at);
        let count = e.mem.u32(count_at);
        let at = e.mem.u32(cursor);
        e.mem.set_u32(cursor, align_four(at));
        let mut bounds = [[0.0f32; 2]; 3];
        for lane in 0..3u32 {
            let at = e.mem.u32(cursor);
            if flags & (1 << lane) != 0 {
                let value = x87_quiet_bits(e.mem.u32(at));
                e.mem.set_u32(statics_at + 4 * lane, value);
                e.mem.set_u32(cursor, at.wrapping_add(4));
            } else if flags & (0x10 << lane) != 0 {
                let minimum = x87_quiet(e.mem.f32(at));
                let maximum = x87_quiet(e.mem.f32(at.wrapping_add(4)));
                bounds[lane as usize] = [minimum, maximum];
                e.mem.set_u32(cursor, at.wrapping_add(8));
            }
        }
        if flags & 0xf0 == 0 {
            e.call(STATIC_FILL, &args![mask, statics_at, defaults, out]);
            return;
        }
        let width = table_word(e, BYTES_PER_COMPONENT, kind);
        let aligned = e.mem.u32(cursor).wrapping_add(1) & 0xffff_fffe;
        e.mem.set_u32(cursor, aligned);
        let components = table_word(e, COMPONENT_COUNTS, u32::from((flags >> 4) & 7));
        let mut data = span
            .wrapping_sub(degree as u32)
            .wrapping_mul(width)
            .wrapping_mul(components)
            .wrapping_add(aligned);
        if degree >= 0 {
            let inverted = !u32::from(flags);
            let default_lanes = ((inverted as i32 >> 4) as u32) & inverted & 0xf;
            let static_lanes = u32::from(flags & 0xf);
            for point in 0..=degree as u32 {
                let target = points_at + 16 * point;
                for lane in 0..3u32 {
                    if flags & (0x10 << lane) == 0 {
                        continue;
                    }
                    let fraction = if kind == 0 {
                        let step = f64::from(e.mem.u8(data));
                        data = data.wrapping_add(1);
                        step / 255.0
                    } else {
                        let step = f64::from(e.mem.u16(data));
                        data = data.wrapping_add(2);
                        step / 65535.0
                    };
                    let [minimum, maximum] = bounds[lane as usize];
                    let value = ((f64::from(maximum) - f64::from(minimum)) * fraction
                        + f64::from(minimum)) as f32;
                    e.mem.set_f32(target + 4 * lane, value);
                }
                for lane in 0..4u32 {
                    if static_lanes & (1 << lane) != 0 {
                        let value = x87_quiet_bits(e.mem.u32(statics_at + 4 * lane));
                        e.mem.set_u32(target + 4 * lane, value);
                    } else if default_lanes & (1 << lane) != 0 {
                        let value = x87_quiet_bits(e.mem.u32(defaults.wrapping_add(4 * lane)));
                        e.mem.set_u32(target + 4 * lane, value);
                    }
                }
            }
        }
        let function = table_word(e, DEGREE_FUNCTIONS, degree as u32);
        e.call(
            function,
            &args![x87_quiet(time), degree, knots_at, points_at, out],
        );
        let advance = count
            .wrapping_add(1)
            .wrapping_mul(width)
            .wrapping_mul(components);
        let at = e.mem.u32(cursor);
        e.mem.set_u32(cursor, at.wrapping_add(advance));
    });
}

// Translated from 00e33dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaSplineCompressedAnimation::readNURBSQuaternion` (Xbox PDB name; thiscall, `RET 0x18`):
/// reads a rotation with `fn_00e33960(cursor, key, [this + 0x40], time, mask, kind, out)` (the
/// scale and the time through the x87 stack) and rounds `*cursor` up to 4.
pub fn hka_spline_compressed_animation_read_nurbs_quaternion(
    e: &mut Engine,
    this: Ptr,
    time: f32,
    key: u32,
    mask: u32,
    kind: u32,
    cursor: u32,
    out: u32,
) {
    let scale = x87_quiet(e.mem.f32(this.addr() + 0x40));
    e.call(
        VECTOR_READ,
        &args![cursor, key, scale, x87_quiet(time), mask, kind, out],
    );
    let at = e.mem.u32(cursor);
    e.mem.set_u32(cursor, align_four(at));
}

/// Reads a vector track whose defaults are the four words at `defaults_at` (`this` is the
/// animation, the scale `this + 0x40`): with a zero mask byte the defaults are copied to `out`,
/// else `fn_00e33a30` reads the vector with a copy of the defaults; then `*cursor` is rounded
/// up to 4.
fn read_vector_track(
    e: &mut Engine,
    this: u32,
    time: f32,
    key: u32,
    mask: u32,
    kind: u32,
    cursor: u32,
    out: u32,
    defaults_at: u32,
) {
    if mask as u8 == 0 {
        copy_vector(e, defaults_at, out);
    } else {
        let scale = x87_quiet(e.mem.f32(this + 0x40));
        e.with_stack(0x10, |e, copy| {
            let copy = copy.addr();
            copy_vector(e, defaults_at, copy);
            e.call(
                STATIC_OR_ANIMATED,
                &args![cursor, key, scale, x87_quiet(time), mask, kind, copy, out],
            );
        });
    }
    let at = e.mem.u32(cursor);
    e.mem.set_u32(cursor, align_four(at));
}

// Translated from 00e33e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads the translation of a track (no name in the engine map; thiscall, `RET 0x18`): with a
/// zero mask byte `out` gets the four words at `01268700` (the identity transform's
/// translation); otherwise `fn_00e33a30(cursor, key, [this + 0x40], time, mask, kind, defaults,
/// out)` reads it, `defaults` being a copy of those words (the scale and the time go through the
/// x87 stack). `*cursor` is rounded up to 4 afterwards.
pub fn fn_00e33e10(
    e: &mut Engine,
    this: Ptr,
    time: f32,
    key: u32,
    mask: u32,
    kind: u32,
    cursor: u32,
    out: u32,
) {
    read_vector_track(
        e,
        this.addr(),
        time,
        key,
        mask,
        kind,
        cursor,
        out,
        IDENTITY_TRANSFORM,
    );
}

// Translated from 00e33e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaSplineCompressedAnimation::sampleScale` (Xbox PDB name; thiscall, `RET 0x18`): as
/// `fn_00e33e10` for the scale of a track, with the four words at `01268720` (the identity
/// transform's scale) as defaults.
pub fn hka_spline_compressed_animation_sample_scale(
    e: &mut Engine,
    this: Ptr,
    time: f32,
    key: u32,
    mask: u32,
    kind: u32,
    cursor: u32,
    out: u32,
) {
    read_vector_track(
        e,
        this.addr(),
        time,
        key,
        mask,
        kind,
        cursor,
        out,
        IDENTITY_SCALE,
    );
}

// Translated from 00e33f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads one float track (no name in the engine map; thiscall, `RET 0x18`): with a zero mask
/// byte `*out` is `0.0`; otherwise `fn_00e33a30(cursor, key, [this + 0x40], time, mask, kind,
/// zeros, local)` reads a vector into a local and its first lane (through the x87 stack) is
/// stored to `*out`. `*cursor` is rounded up to 4 afterwards.
pub fn fn_00e33f10(
    e: &mut Engine,
    this: Ptr,
    time: f32,
    key: u32,
    mask: u32,
    kind: u32,
    cursor: u32,
    out: u32,
) {
    if mask as u8 == 0 {
        e.mem.set_f32(out, 0.0);
    } else {
        let scale = x87_quiet(e.mem.f32(this.addr() + 0x40));
        // Locals: the vector of zero defaults and the vector read.
        e.with_stack(0x20, |e, frame| {
            let frame = frame.addr();
            e.call(
                STATIC_OR_ANIMATED,
                &args![
                    cursor,
                    key,
                    scale,
                    x87_quiet(time),
                    mask,
                    kind,
                    frame,
                    frame + 0x10
                ],
            );
            let value = x87_quiet_bits(e.mem.u32(frame + 0x10));
            e.mem.set_u32(out, value);
        });
    }
    let at = e.mem.u32(cursor);
    e.mem.set_u32(cursor, align_four(at));
}

/// Samples one transform track from its four header bytes `[kinds, translation mask, rotation
/// mask, scale mask]`: the translation (kind `kinds & 3`) into `out`, the rotation (kind
/// `kinds >> 2 & 0xf`) into `out + 0x10` and the scale (kind `kinds >> 6`) into `out + 0x20`,
/// all through the cursor in the word at `cursor_at`.
fn sample_transform_track(
    e: &mut Engine,
    this: u32,
    fraction: f32,
    key: u32,
    header: [u8; 4],
    cursor_at: u32,
    out: u32,
) {
    let [kinds, translation_mask, rotation_mask, scale_mask] = header;
    e.call(
        TRANSLATION_SAMPLE,
        &args![
            this,
            fraction,
            key,
            u32::from(translation_mask),
            u32::from(kinds & 3),
            cursor_at,
            out
        ],
    );
    e.call(
        ROTATION_SAMPLE,
        &args![
            this,
            fraction,
            key,
            u32::from(rotation_mask),
            u32::from((kinds >> 2) & 0xf),
            cursor_at,
            out + 0x10
        ],
    );
    e.call(
        SCALE_SAMPLE,
        &args![
            this,
            fraction,
            key,
            u32::from(scale_mask),
            u32::from(kinds >> 6),
            cursor_at,
            out + 0x20
        ],
    );
}

/// Samples one float track from its flag byte: the mask is `byte & 0xf9` and the kind
/// `byte >> 1 & 3`.
fn sample_float_track(
    e: &mut Engine,
    this: u32,
    fraction: f32,
    key: u32,
    byte: u8,
    cursor_at: u32,
    out: u32,
) {
    e.call(
        FLOAT_SAMPLE,
        &args![
            this,
            fraction,
            key,
            u32::from(byte & 0xf9),
            u32::from((byte >> 1) & 3),
            cursor_at,
            out
        ],
    );
}

/// The four header bytes at `at`.
fn header_bytes(e: &Engine, at: u32) -> [u8; 4] {
    [
        e.mem.u8(at),
        e.mem.u8(at.wrapping_add(1)),
        e.mem.u8(at.wrapping_add(2)),
        e.mem.u8(at.wrapping_add(3)),
    ]
}

// Translated from 00e33f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Samples the tracks of one block (no name in the engine map; cdecl): `chunk` is a chunk
/// description made by `getDataChunks` (`+0` the animation, `+0xc` the aligned data address,
/// `+0x14` the remainder byte, `+0x18` the aligned address of the float offset with `+0x20` its
/// remainder). `getBlockAndTime(frame, time)` gives the fraction and the key byte. The cursor
/// starts at `data + remainder + [this + 0x34]` and the track headers (four bytes each) at
/// `data + remainder`: every transform track writes 0x30 bytes to `transforms` (translation,
/// rotation, scale, see `sample_transform_track`). The float tracks then continue with the cursor
/// `[chunk[0x18] + chunk[0x20]] + data + remainder` and their flag bytes at
/// `data + remainder + [this + 0x10] * 4`, one `float` each to `floats`.
pub fn fn_00e33f90(
    e: &mut Engine,
    frame: u32,
    time: f32,
    transform_count: u32,
    transforms: u32,
    float_count: u32,
    floats: u32,
    chunk: u32,
) {
    let this = e.mem.u32(chunk);
    // Locals: the fraction, the key word and the cursor (first the block).
    e.with_stack(0x10, |e, slots| {
        let slots = slots.addr();
        let (fraction_at, key_at, cursor_at) = (slots, slots + 4, slots + 8);
        e.call(
            BLOCK_AND_TIME,
            &args![this, frame, x87_quiet(time), cursor_at, fraction_at, key_at],
        );
        let data = e.mem.u32(chunk + 0xc);
        let remainder = u32::from(e.mem.u8(chunk + 0x14));
        let headers = remainder.wrapping_add(data);
        let cursor = e.mem.u32(this + 0x34).wrapping_add(headers);
        e.mem.set_u32(cursor_at, cursor);
        let mut header_at = headers;
        let mut out = transforms;
        for _ in 0..transform_count {
            let header = header_bytes(e, header_at);
            let fraction = x87_quiet(e.mem.f32(fraction_at));
            let key = e.mem.u32(key_at);
            sample_transform_track(e, this, fraction, key, header, cursor_at, out);
            header_at = header_at.wrapping_add(4);
            out = out.wrapping_add(0x30);
        }
        let float_offsets = e
            .mem
            .u32(chunk + 0x18)
            .wrapping_add(u32::from(e.mem.u8(chunk + 0x20)));
        let float_cursor = e
            .mem
            .u32(float_offsets)
            .wrapping_add(data)
            .wrapping_add(remainder);
        e.mem.set_u32(cursor_at, float_cursor);
        let transform_tracks = e.mem.u32(this + 0x10);
        let mut mask_at = data
            .wrapping_add(transform_tracks.wrapping_mul(4))
            .wrapping_add(remainder);
        let mut out = floats;
        for _ in 0..float_count {
            let byte = e.mem.u8(mask_at);
            mask_at = mask_at.wrapping_add(1);
            let fraction = x87_quiet(e.mem.f32(fraction_at));
            let key = e.mem.u32(key_at);
            sample_float_track(e, this, fraction, key, byte, cursor_at, out);
            out = out.wrapping_add(4);
        }
    });
}

// Translated from 00e340e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaSplineCompressedAnimation::samplePartialTracks` (Xbox PDB name; thiscall, `RET 0x18`;
/// the last stack word is not read): a profiling record `"TtSampleSpline"` is appended to the
/// thread's record stream and the end record (`010c41f8`) is appended before every return.
/// `getFrameAndDelta(time)` (`00e2f7c0`) gives the frame and the delta, `getBlockAndTime(frame,
/// delta)` the block, the fraction and the key byte (the delta's bits keep the other three
/// bytes of the key word). `computePackedNurbsOffsets(data, block offsets, block, value)` is
/// asked for the cursor (`value` = `[this + 0x34]`) and for the track headers (`value` =
/// `0x80000000`); when either is zero nothing is sampled. Otherwise the transform tracks
/// (`transform_count`, four header bytes each) write to `transforms` and the float tracks
/// (`float_count`, flag bytes at `data + [this + 0x10] * 4 + block start`) write to `floats`
/// exactly as `fn_00e33f90` does, the float cursor being `[this + 0x50][block] + data +
/// block start`.
pub fn hka_spline_compressed_animation_sample_partial_tracks(
    e: &mut Engine,
    this: Ptr,
    time: f32,
    transform_count: u32,
    transforms: u32,
    float_count: u32,
    floats: u32,
    _unused_5: u32,
) {
    let this = this.addr();
    profile_record(e, PROFILE_LABEL);
    // Locals: the key word (first the time), the cursor, the fraction and the block (first
    // the frame).
    e.with_stack(0x10, |e, slots| {
        let slots = slots.addr();
        let (key_at, cursor_at, fraction_at, block_at) = (slots, slots + 4, slots + 8, slots + 12);
        e.mem.set_u32(key_at, time.to_bits());
        e.call(
            GET_FRAME_AND_DELTA,
            &args![this, x87_quiet(time), block_at, key_at],
        );
        let frame = e.mem.u32(block_at);
        let delta = x87_quiet(e.mem.f32(key_at));
        e.call(
            BLOCK_AND_TIME,
            &args![this, frame, delta, block_at, fraction_at, key_at],
        );
        let block = e.mem.u32(block_at);
        let data = e.mem.u32(this + 0x74);
        let offsets = e.mem.u32(this + 0x44);
        let transform_offset = e.mem.u32(this + 0x34);
        let cursor = e
            .call(
                PACKED_OFFSETS,
                &args![data, offsets, block, transform_offset],
            )
            .u32();
        e.mem.set_u32(cursor_at, cursor);
        let headers = e
            .call(PACKED_OFFSETS, &args![data, offsets, block, 0x8000_0000u32])
            .u32();
        if cursor == 0 || headers == 0 {
            profile_record(e, PROFILE_END);
            return;
        }
        let mut header_at = headers;
        let mut out = transforms;
        for _ in 0..transform_count {
            let header = header_bytes(e, header_at);
            let fraction = x87_quiet(e.mem.f32(fraction_at));
            let key = e.mem.u32(key_at);
            sample_transform_track(e, this, fraction, key, header, cursor_at, out);
            header_at = header_at.wrapping_add(4);
            out = out.wrapping_add(0x30);
        }
        let block = e.mem.u32(block_at);
        let block_start = e.mem.u32(offsets.wrapping_add(block.wrapping_mul(4)));
        let float_offset = e
            .mem
            .u32(e.mem.u32(this + 0x50).wrapping_add(block.wrapping_mul(4)));
        let float_cursor = float_offset.wrapping_add(data).wrapping_add(block_start);
        e.mem.set_u32(cursor_at, float_cursor);
        let transform_tracks = e.mem.u32(this + 0x10);
        let mut mask_at = data
            .wrapping_add(transform_tracks.wrapping_mul(4))
            .wrapping_add(block_start);
        let mut out = floats;
        for _ in 0..float_count {
            let byte = e.mem.u8(mask_at);
            mask_at = mask_at.wrapping_add(1);
            let fraction = x87_quiet(e.mem.f32(fraction_at));
            let key = e.mem.u32(key_at);
            sample_float_track(e, this, fraction, key, byte, cursor_at, out);
            out = out.wrapping_add(4);
        }
        profile_record(e, PROFILE_END);
    });
}

/// Calls `getFrameAndDelta` and `getBlockAndTime` for `time`, leaving the block, the fraction
/// and the key word in the three words at `slots` (block at `+0`, fraction at `+4`, key at `+8`)
/// and the delta slot at `+12` used on the way.
fn locate_block(e: &mut Engine, this: u32, time: f32, slots: u32) {
    let (block_at, fraction_at, key_at, delta_at) = (slots, slots + 4, slots + 8, slots + 12);
    e.call(
        GET_FRAME_AND_DELTA,
        &args![this, x87_quiet(time), block_at, delta_at],
    );
    let frame = e.mem.u32(block_at);
    let delta = x87_quiet(e.mem.f32(delta_at));
    e.call(
        BLOCK_AND_TIME,
        &args![this, frame, delta, block_at, fraction_at, key_at],
    );
}

// Translated from 00e34310 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaSplineCompressedAnimation::sampleIndividualTransformTracks` (Xbox PDB name; thiscall,
/// `RET 0x10`): `time` gives the frame and delta (`00e2f7c0`) and the block, fraction and key
/// (`getBlockAndTime`). For each of the `count` signed 16-bit track numbers in `indices`: a
/// negative number or one not below the number of transform tracks (`this + 0x10`) zeroes the
/// 0x30 bytes at the output; otherwise the cursor is `computePackedNurbsOffsets(data, block
/// offsets, block, [this + 0x5c][tracks * block + number])` and the header
/// `computePackedNurbsOffsets(.., number * 4)`, and the track is sampled as in `fn_00e33f90`.
pub fn hka_spline_compressed_animation_sample_individual_transform_tracks(
    e: &mut Engine,
    this: Ptr,
    time: f32,
    indices: u32,
    count: u32,
    transforms: u32,
) {
    let this = this.addr();
    // Locals: the block, fraction, key, delta words and the cursor.
    e.with_stack(0x14, |e, slots| {
        let slots = slots.addr();
        let (fraction_at, key_at, cursor_at) = (slots + 4, slots + 8, slots + 0x10);
        locate_block(e, this, time, slots);
        let mut out = transforms;
        for i in 0..count {
            let number = e.mem.u16(indices.wrapping_add(i.wrapping_mul(2))) as i16;
            let tracks = e.mem.i32(this + 0x10);
            if number < 0 || i32::from(number) >= tracks {
                for lane in 0..3 {
                    store_vec(e, out + 16 * lane, [0.0; 4]);
                }
            } else {
                let block = e.mem.u32(slots);
                let data = e.mem.u32(this + 0x74);
                let offsets = e.mem.u32(this + 0x44);
                let entry = (tracks as u32)
                    .wrapping_mul(block)
                    .wrapping_add(i32::from(number) as u32);
                let table = e.mem.u32(this + 0x5c);
                let value = e.mem.u32(table.wrapping_add(entry.wrapping_mul(4)));
                let cursor = e
                    .call(PACKED_OFFSETS, &args![data, offsets, block, value])
                    .u32();
                e.mem.set_u32(cursor_at, cursor);
                let header_value = (i32::from(number) as u32).wrapping_mul(4);
                let header_at = e
                    .call(PACKED_OFFSETS, &args![data, offsets, block, header_value])
                    .u32();
                let header = header_bytes(e, header_at);
                let fraction = x87_quiet(e.mem.f32(fraction_at));
                let key = e.mem.u32(key_at);
                sample_transform_track(e, this, fraction, key, header, cursor_at, out);
            }
            out = out.wrapping_add(0x30);
        }
    });
}

// Translated from 00e34490 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaSplineCompressedAnimation::sampleIndividualFloatTracks` (Xbox PDB name; thiscall,
/// `RET 0x10`): as the transform version for the float tracks. A track number that is negative
/// or not below the number of float tracks (`this + 0x14`) stores `0.0` to its output `float`;
/// otherwise the cursor is `computePackedNurbsOffsets(data, block offsets, block, [this +
/// 0x68][float tracks * block + number])`, the flag byte is at
/// `computePackedNurbsOffsets(.., number + [this + 0x10] * 4)` and the track is sampled as in
/// `fn_00e33f90`.
pub fn hka_spline_compressed_animation_sample_individual_float_tracks(
    e: &mut Engine,
    this: Ptr,
    time: f32,
    indices: u32,
    count: u32,
    floats: u32,
) {
    let this = this.addr();
    // Locals: the block, fraction, key, delta words and the cursor.
    e.with_stack(0x14, |e, slots| {
        let slots = slots.addr();
        let (fraction_at, key_at, cursor_at) = (slots + 4, slots + 8, slots + 0x10);
        locate_block(e, this, time, slots);
        let mut out = floats;
        for i in 0..count {
            let number = e.mem.u16(indices.wrapping_add(i.wrapping_mul(2))) as i16;
            let tracks = e.mem.i32(this + 0x14);
            if number < 0 || i32::from(number) >= tracks {
                e.mem.set_f32(out, 0.0);
            } else {
                let block = e.mem.u32(slots);
                let data = e.mem.u32(this + 0x74);
                let offsets = e.mem.u32(this + 0x44);
                let entry = (tracks as u32)
                    .wrapping_mul(block)
                    .wrapping_add(i32::from(number) as u32);
                let table = e.mem.u32(this + 0x68);
                let value = e.mem.u32(table.wrapping_add(entry.wrapping_mul(4)));
                let cursor = e
                    .call(PACKED_OFFSETS, &args![data, offsets, block, value])
                    .u32();
                e.mem.set_u32(cursor_at, cursor);
                let transform_tracks = e.mem.u32(this + 0x10);
                let flag_value =
                    (i32::from(number) as u32).wrapping_add(transform_tracks.wrapping_mul(4));
                let flag_at = e
                    .call(PACKED_OFFSETS, &args![data, offsets, block, flag_value])
                    .u32();
                let byte = e.mem.u8(flag_at);
                let fraction = x87_quiet(e.mem.f32(fraction_at));
                let key = e.mem.u32(key_at);
                sample_float_track(e, this, fraction, key, byte, cursor_at, out);
            }
            out = out.wrapping_add(4);
        }
    });
}

// Translated from 00e345a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaSplineCompressedAnimation::~hkaSplineCompressedAnimation` (Xbox PDB name; `this` in
/// `ECX`): stores the vtable `010de0ec`, then empties five arrays in turn (the data array at
/// `+0x74` with size `+0x78` and capacity word `+0x7c`, then `+0x68`, `+0x5c`, `+0x50`, `+0x44`
/// with sizes and capacity words after them): the size is cleared, the memory is given back
/// through the allocator (field `+0x10` of the memory router `00c85750`, slot `0x10`, with
/// `(data, bytes)`; bytes is the capacity, times 4 for all but the first array) unless the
/// capacity word's top bit marks it as not owned, then the data pointer is cleared and the
/// capacity word set to `0x80000000`. It ends by running the base destructor `00dfb720`.
pub fn hka_spline_compressed_animation_destructor(e: &mut Engine, this: Ptr) {
    let this = this.addr();
    e.mem.set_u32(this, VTABLE);
    for (data_at, size_at, flags_at, multiplier) in [
        (0x74, 0x78, 0x7c, 1u32),
        (0x68, 0x6c, 0x70, 4),
        (0x5c, 0x60, 0x64, 4),
        (0x50, 0x54, 0x58, 4),
        (0x44, 0x48, 0x4c, 4),
    ] {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let flags = e.mem.u32(this + flags_at);
        e.mem.set_u32(this + size_at, 0);
        if flags & 0x8000_0000 == 0 {
            let data = e.mem.u32(this + data_at);
            let bytes = (flags & 0x3fff_ffff).wrapping_mul(multiplier);
            e.vcall(allocator, 0x10, &args![data, bytes]);
        }
        e.mem.set_u32(this + data_at, 0);
        e.mem.set_u32(this + flags_at, 0x8000_0000);
    }
    e.call(ANIMATION_DESTRUCTOR, &args![this]);
}

// Translated from 00e346a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fills the lanes of a vector for a mask without animated bits (no name in the engine map;
/// cdecl): with `mask` a byte, `s = mask & 0xf` and `d = (!mask >> 4 & !mask) & 0xf` (the
/// arithmetic shift of the inverted zero-extended byte), lane `i` of `out` gets the static
/// value `statics[i]` when bit `i` of `s` is set, else `defaults[i]` when bit `i` of `d` is
/// set, else it is left alone (each copy goes through the x87 stack).
pub fn fn_00e346a0(e: &mut Engine, mask: u8, statics: u32, defaults: u32, out: u32) {
    let inverted = !u32::from(mask);
    let default_lanes = ((inverted as i32 >> 4) as u32) & inverted & 0xf;
    let static_lanes = u32::from(mask & 0xf);
    for lane in 0..4u32 {
        let source = if static_lanes & (1 << lane) != 0 {
            statics
        } else if default_lanes & (1 << lane) != 0 {
            defaults
        } else {
            continue;
        };
        let value = x87_quiet_bits(e.mem.u32(source.wrapping_add(4 * lane)));
        e.mem.set_u32(out.wrapping_add(4 * lane), value);
    }
}

// Translated from 00e34720 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaSplineCompressedAnimation::getBlockAndTime` (Xbox PDB name; thiscall, `RET 0x14`): with
/// `n = [this + 0x30] - 1` frames per block (an unsigned division by zero faults), the block is
/// `frame / n` (0 when that is not positive as a signed number), stored to `*block_out`, then
/// limited (signed) to `[this + 0x2c] - 1` and stored again. The frame in the block is
/// `frame - n * block` (signed; a negative one has `2^32` added after conversion) plus the time,
/// stored as a `float`; times `[this + 0x40]` (stored as a `float`) it is the fraction stored to
/// `*fraction_out`; the key is the fraction times `[this + 0x3c]` times `n`, truncated towards
/// zero (out of range gives `0x80000000`), and its low byte is stored to `*byte_out`.
/// Answers `byte_out`: the exe leaves it in `EAX` at every `RET` (the hkBool/hkResult result
/// returned through a pointer, which callers read through `EAX`).
pub fn hka_spline_compressed_animation_get_block_and_time(
    e: &mut Engine,
    this: Ptr,
    frame: u32,
    time: f32,
    block_out: u32,
    fraction_out: u32,
    byte_out: u32,
) -> u32 {
    let this = this.addr();
    let frames = e.mem.u32(this + 0x30).wrapping_sub(1);
    if frames == 0 {
        panic!("00e34720: division by zero (DIV by the frame count of a block)");
    }
    let quotient = frame / frames;
    let block = if (quotient as i32) <= 0 { 0 } else { quotient };
    e.mem.set_u32(block_out, block);
    let last = e.mem.u32(this + 0x2c).wrapping_sub(1);
    let block = if (block as i32) < (last as i32) {
        block
    } else {
        last
    };
    e.mem.set_u32(block_out, block);
    let frames = e.mem.u32(this + 0x30).wrapping_sub(1);
    let inside = frame.wrapping_sub(frames.wrapping_mul(block)) as i32;
    let mut sum = f64::from(inside);
    if inside < 0 {
        sum += 4_294_967_296.0;
    }
    let sum = (sum + f64::from(time)) as f32;
    let fraction = (f64::from(sum) * f64::from(e.mem.f32(this + 0x40))) as f32;
    e.mem.set_f32(fraction_out, fraction);
    let frames = e.mem.u32(this + 0x30).wrapping_sub(1);
    let scaled = f64::from(fraction) * f64::from(e.mem.f32(this + 0x3c)) * f64::from(frames as i32);
    e.mem.set_u8(byte_out, truncate(scaled) as u8);
    byte_out
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e3_2d90, fn_00e32d90(u32, u32, u32, u32) -> u32),
        entry!(
            0x00e3_2ec0,
            hka_spline_compressed_animation_get_num_original_frames(Ptr) -> u32
        ),
        entry!(
            0x00e3_2ed0,
            hka_spline_compressed_animation_get_num_data_chunks(Ptr, u32, u32) -> u32
        ),
        entry!(
            0x00e3_2ee0,
            hka_spline_compressed_animation_sample_tracks(Ptr, f32, u32, u32, u32)
        ),
        entry!(
            0x00e3_2f10,
            fn_00e32f10(u32, u32, u32, u32, f32, u32) -> u32
        ),
        entry!(0x00e3_3030, fn_00e33030(u32, u32, u32, i32, u32, u32)),
        entry!(0x00e3_3330, fn_00e33330(Ptr) -> u32),
        entry!(
            0x00e3_3350,
            hka_spline_compressed_animation_get_data_chunks(Ptr, u32, f32, u32, u32)
        ),
        entry!(
            0x00e3_3400,
            hka_spline_compressed_animation_get_max_size_of_combined_data_chunks(Ptr) -> u32
        ),
        entry!(0x00e3_3440, fn_00e33440(f32, i32, u32, u32, u32)),
        entry!(0x00e3_3610, fn_00e33610(f32, u32, u32, u32, u32)),
        entry!(0x00e3_3670, fn_00e33670(f32, u32, u32, u32, u32)),
        entry!(0x00e3_3760, fn_00e33760(f32, u32, u32, u32, u32)),
        entry!(0x00e3_3960, fn_00e33960(u32, u32, f32, f32, u8, u32, u32)),
        entry!(
            0x00e3_3a30,
            fn_00e33a30(u32, u32, f32, f32, u32, u32, u32, u32)
        ),
        entry!(
            0x00e3_3dc0,
            hka_spline_compressed_animation_read_nurbs_quaternion(
                Ptr, f32, u32, u32, u32, u32, u32
            )
        ),
        entry!(0x00e3_3e10, fn_00e33e10(Ptr, f32, u32, u32, u32, u32, u32)),
        entry!(
            0x00e3_3e90,
            hka_spline_compressed_animation_sample_scale(Ptr, f32, u32, u32, u32, u32, u32)
        ),
        entry!(0x00e3_3f10, fn_00e33f10(Ptr, f32, u32, u32, u32, u32, u32)),
        entry!(0x00e3_3f90, fn_00e33f90(u32, f32, u32, u32, u32, u32, u32)),
        entry!(
            0x00e3_40e0,
            hka_spline_compressed_animation_sample_partial_tracks(
                Ptr, f32, u32, u32, u32, u32, u32
            )
        ),
        entry!(
            0x00e3_4310,
            hka_spline_compressed_animation_sample_individual_transform_tracks(
                Ptr, f32, u32, u32, u32
            )
        ),
        entry!(
            0x00e3_4490,
            hka_spline_compressed_animation_sample_individual_float_tracks(Ptr, f32, u32, u32, u32)
        ),
        entry!(0x00e3_45a0, hka_spline_compressed_animation_destructor(Ptr)),
        entry!(0x00e3_46a0, fn_00e346a0(u8, u32, u32, u32)),
        entry!(
            0x00e3_4720,
            hka_spline_compressed_animation_get_block_and_time(Ptr, u32, f32, u32, u32, u32) -> u32
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpsphereshape::{PROFILE_SLOT, TLS_GET_VALUE};
    use std::cell::RefCell;
    use std::rc::Rc;

    /// The calls a sampler double saw: its address, the argument words without the cursor
    /// slot (index 5) and the value in that slot.
    type Samples = Rc<RefCell<Vec<(u32, Vec<u32>, u32)>>>;

    const DEGREE_DOUBLE: u32 = 0x0900_0002;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        put_words(&mut e, ALIGNMENTS, &[4, 1, 2, 1, 2, 4]);
        put_words(&mut e, ELEMENT_SIZES, &[4, 5, 6, 3, 2, 16]);
        put_words(&mut e, BYTES_PER_COMPONENT, &[1, 2]);
        put_words(&mut e, COMPONENT_COUNTS, &[0, 1, 1, 2, 1, 2, 2, 3]);
        put_words(
            &mut e,
            DEGREE_FUNCTIONS,
            &[0, 0x00e3_3610, 0x00e3_3670, 0x00e3_3760],
        );
        store_vec(&mut e, IDENTITY_TRANSFORM, [1.5, 2.5, 3.5, 4.5]);
        store_vec(&mut e, IDENTITY_SCALE, [1.0, 1.0, 1.0, 0.5]);
        store_bits(&mut e, IDENTITY_QUATERNION, [0, 0, 0, 0x3f80_0000]);
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

    fn put_bytes(e: &mut Engine, at: u32, bytes: &[u8]) {
        for (i, byte) in bytes.iter().enumerate() {
            e.mem.set_u8(at + i as u32, *byte);
        }
    }

    fn put_words(e: &mut Engine, at: u32, words: &[u32]) {
        for (i, word) in words.iter().enumerate() {
            e.mem.set_u32(at + 4 * i as u32, *word);
        }
    }

    fn put_floats(e: &mut Engine, at: u32, values: &[f32]) {
        for (i, value) in values.iter().enumerate() {
            e.mem.set_f32(at + 4 * i as u32, *value);
        }
    }

    /// A zero-filled block at a 16-byte boundary with room for `size` bytes.
    fn aligned(e: &mut Engine, size: u32) -> u32 {
        (e.mem.alloc(size + 0x10) + 0xf) & !0xf
    }

    fn assert_close(got: Vec4, want: Vec4, tolerance: f32) {
        for lane in 0..4 {
            assert!(
                (got[lane] - want[lane]).abs() <= tolerance,
                "lane {lane}: {got:?} != {want:?}"
            );
        }
    }

    /// Doubles for the four per-track samplers that log every call.
    fn record_samplers(e: &mut Engine) -> Samples {
        let log: Samples = Rc::new(RefCell::new(Vec::new()));
        for address in [
            TRANSLATION_SAMPLE,
            ROTATION_SAMPLE,
            SCALE_SAMPLE,
            FLOAT_SAMPLE,
        ] {
            let sink = Rc::clone(&log);
            e.register_double(address, move |e, a| {
                let mut shown = a.to_vec();
                let cursor_at = shown.remove(5);
                let cursor = e.mem.u32(cursor_at);
                sink.borrow_mut().push((address, shown, cursor));
                Ret::default()
            });
        }
        log
    }

    /// A profiling record stream with room for 4 records (cursor at `+4`, limit at `+0xc`).
    fn profile_stream(e: &mut Engine) -> u32 {
        let records = e.mem.alloc(0x30);
        let stream = e.mem.alloc(0x10);
        e.mem.set_u32(stream + 4, records);
        e.mem.set_u32(stream + 0xc, records + 0x30);
        e.set_global(PROFILE_SLOT, 5u32);
        e.register_double(TLS_GET_VALUE, move |_, _| Ret {
            eax: stream,
            ..Ret::default()
        });
        stream
    }

    /// The labels of the profiling records written so far.
    fn labels(e: &Engine, stream: u32) -> Vec<u32> {
        let first = e.mem.u32(stream + 0xc) - 0x30;
        let cursor = e.mem.u32(stream + 4);
        (first..cursor).step_by(12).map(|a| e.mem.u32(a)).collect()
    }

    #[test]
    fn fn_00e32d90_bisects_the_knot_bytes() {
        let mut e = engine();
        let knots = e.mem.alloc(8);
        put_bytes(&mut e, knots, &[10, 20, 30, 40, 50, 60]);
        let mut search = |key: u32| e.call(0x00e3_2d90, &args![4u32, 1u32, key, knots]).u32();
        assert_eq!(search(25), 1);
        assert_eq!(search(35), 2);
        assert_eq!(search(55), 4);
        assert_eq!(search(60), 4);
        assert_eq!(search(200), 4);
        assert_eq!(search(10), 1);
        assert_eq!(search(3), 1);
        // Only the low byte of the key is compared: 0x19 = 25.
        assert_eq!(search(0x1_0019), 1);
    }

    #[test]
    fn get_num_original_frames_reads_the_frame_count() {
        let mut e = engine();
        let this = e.mem.alloc(0x100);
        e.mem.set_u32(this + 0x28, 77);
        assert_eq!(e.call(0x00e3_2ec0, &args![this]).u32(), 77);
    }

    #[test]
    fn get_num_data_chunks_is_three() {
        let mut e = engine();
        let this = e.mem.alloc(0x100);
        assert_eq!(e.call(0x00e3_2ed0, &args![this, 5u32, 6u32]).u32(), 3);
    }

    #[test]
    fn sample_tracks_calls_the_virtual_sampler_with_the_track_counts() {
        let mut e = engine();
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0x10, 0x0900_0001);
        let this = e.mem.alloc(0x80);
        e.mem.set_u32(this, vtable);
        e.mem.set_u32(this + 0x10, 7);
        e.mem.set_u32(this + 0x14, 3);
        e.register_double(0x0900_0001, |_, _| Ret::default());
        e.call(
            0x00e3_2ee0,
            &args![this, f32::from_bits(0x7f80_0001), 0x11u32, 0x22u32, 0x33u32],
        );
        assert_eq!(
            calls(&e, 0x0900_0001),
            vec![vec![this, 0x7fc0_0001, 7, 0x11, 3, 0x22, 0x33]]
        );
    }

    #[test]
    fn fn_00e32f10_reads_the_knots_and_scales_the_ones_around_the_span() {
        let mut e = engine();
        let stream = e.mem.alloc(0x20);
        put_bytes(&mut e, stream, &[3, 0, 1, 10, 20, 30, 40, 50, 60]);
        let words = e.mem.alloc(0x10);
        let floats = e.mem.alloc(0x20);
        let (cursor, count, degree) = (words, words + 4, words + 8);
        e.mem.set_u32(cursor, stream);
        let span = e
            .call(
                0x00e3_2f10,
                &args![cursor, count, degree, 25u32, 0.5f32, floats],
            )
            .u32();
        assert_eq!(span, 1);
        assert_eq!(e.mem.u32(count), 3);
        assert_eq!(e.mem.u32(degree), 1);
        assert_eq!(e.mem.f32(floats), 10.0);
        assert_eq!(e.mem.f32(floats + 4), 15.0);
        assert_eq!(e.mem.u32(cursor), stream + 9);
        assert_eq!(calls(&e, KNOT_SEARCH), vec![vec![3, 1, 25, stream + 3]]);
        // Degree 2 writes four floats, k = 0..4 from knots[span - 2 + 1].
        let stream = e.mem.alloc(0x20);
        put_bytes(&mut e, stream, &[4, 0, 2, 5, 10, 15, 20, 25, 30, 35, 40]);
        e.mem.set_u32(cursor, stream);
        let span = e
            .call(
                0x00e3_2f10,
                &args![cursor, count, degree, 17u32, 2.0f32, floats],
            )
            .u32();
        assert_eq!(span, 2);
        assert_eq!(e.mem.u32(count), 4);
        assert_eq!(e.mem.u32(degree), 2);
        let written: Vec<f32> = (0..4).map(|k| e.mem.f32(floats + 4 * k)).collect();
        assert_eq!(written, vec![20.0, 30.0, 40.0, 50.0]);
        assert_eq!(e.mem.u32(cursor), stream + 3 + 4 + 2 + 2);
    }

    #[test]
    fn fn_00e33030_decodes_the_control_points_around_the_span() {
        let mut e = engine();
        e.register_double(0x0900_0001, |e, a| {
            e.mem.set_u32(a[1], a[0]);
            Ret::default()
        });
        put_words(&mut e, DECODERS, &[0x0900_0001]);
        let data = e.mem.alloc(0x40);
        let cursor = e.mem.alloc(4);
        let out = e.mem.alloc(0x40);
        e.mem.set_u32(cursor, data + 1);
        e.call(0x00e3_3030, &args![cursor, 0u32, 5u32, 2i32, 4u32, out]);
        // Kind 0: alignment 4, size 4. The sources start at (4 - 2) * 4 from the aligned cursor.
        assert_eq!(e.mem.u32(out), data + 12);
        assert_eq!(e.mem.u32(out + 16), data + 16);
        assert_eq!(e.mem.u32(out + 32), data + 20);
        assert_eq!(e.mem.u32(cursor), data + 4 + 6 * 4);
        assert_eq!(calls(&e, 0x0900_0001).len(), 3);
        // A negative degree decodes nothing but still moves the cursor.
        e.mem.set_u32(cursor, data);
        e.call(0x00e3_3030, &args![cursor, 0u32, 1u32, -1i32, 0u32, out]);
        assert_eq!(calls(&e, 0x0900_0001).len(), 3);
        assert_eq!(e.mem.u32(cursor), data + 8);
    }

    #[test]
    fn fn_00e33330_sums_the_array_sizes() {
        let mut e = engine();
        let this = e.mem.alloc(0x100);
        e.mem.set_u32(this + 0x78, 0x1000);
        e.mem.set_u32(this + 0x6c, 1);
        e.mem.set_u32(this + 0x60, 2);
        e.mem.set_u32(this + 0x48, 3);
        assert_eq!(
            e.call(0x00e3_3330, &args![this]).u32(),
            0x1000 + 6 * 4 + 0x84
        );
    }

    #[test]
    fn get_data_chunks_splits_the_block_addresses_into_aligned_part_and_remainder() {
        let mut e = engine();
        let this = e.mem.alloc(0x100);
        let data = aligned(&mut e, 0x100);
        let float_offsets = aligned(&mut e, 0x20) + 8;
        let offsets = e.mem.alloc(0x10);
        put_words(&mut e, offsets, &[0, 0x30, 0x70]);
        e.mem.set_u32(this + 0x2c, 3);
        e.mem.set_u32(this + 0x30, 5);
        e.mem.set_f32(this + 0x3c, 1.0);
        e.mem.set_f32(this + 0x40, 0.25);
        e.mem.set_u32(this + 0x44, offsets);
        e.mem.set_u32(this + 0x50, float_offsets);
        e.mem.set_u32(this + 0x74, data + 5);
        e.mem.set_u32(this + 0x78, 0x95);
        let chunk = e.mem.alloc(0x30);
        e.call(0x00e3_3350, &args![this, 5u32, 0.5f32, chunk, 0u32]);
        assert_eq!(e.mem.u32(chunk), this);
        assert_eq!(e.mem.u32(chunk + 4), 0x90);
        // Block 1 starts at 0x30 and ends at 0x70.
        assert_eq!(e.mem.u32(chunk + 0xc), data + 0x30);
        assert_eq!(e.mem.u8(chunk + 0x14), 5);
        assert_eq!(e.mem.u32(chunk + 0x10), 0x50);
        assert_eq!(e.mem.u32(chunk + 0x18), float_offsets - 8);
        assert_eq!(e.mem.u8(chunk + 0x20), 12);
        assert_eq!(e.mem.u32(chunk + 0x1c), 0x10);
        // The last block ends at the data size.
        e.call(0x00e3_3350, &args![this, 10u32, 0.5f32, chunk, 0u32]);
        assert_eq!(e.mem.u32(chunk + 0xc), data + 0x70);
        assert_eq!(e.mem.u8(chunk + 0x14), 5);
        assert_eq!(e.mem.u32(chunk + 0x10), 0x40);
        assert_eq!(e.mem.u32(chunk + 0x18), float_offsets + 8);
        assert_eq!(e.mem.u8(chunk + 0x20), 0);
        assert_eq!(e.mem.u32(chunk + 0x1c), 0x10);
    }

    #[test]
    fn get_max_size_of_combined_data_chunks_finds_the_largest_block() {
        let mut e = engine();
        let this = e.mem.alloc(0x100);
        let offsets = e.mem.alloc(0x10);
        put_words(&mut e, offsets, &[0, 0x30, 0x7c]);
        e.mem.set_u32(this + 0x44, offsets);
        e.mem.set_u32(this + 0x48, 3);
        e.mem.set_u32(this + 0x78, 0xa5);
        assert_eq!(e.call(0x00e3_3400, &args![this]).u32(), 0xf0);
        e.mem.set_u32(this + 0x48, 0);
        assert_eq!(e.call(0x00e3_3400, &args![this]).u32(), 0xa0);
    }

    #[test]
    fn fn_00e33440_builds_the_basis_functions_and_blends_the_points() {
        let mut e = engine();
        let knots = e.mem.alloc(0x20);
        let points = aligned(&mut e, 0x40);
        let out = aligned(&mut e, 0x10);
        store_vec(&mut e, points, [1.0, 2.0, 3.0, 4.0]);
        store_vec(&mut e, points + 0x10, [5.0, 6.0, 7.0, 8.0]);
        put_floats(&mut e, knots, &[0.0, 1.0]);
        e.call(0x00e3_3440, &args![0.25f32, 1i32, knots, points, out]);
        assert_eq!(load_vec(&e, out), [2.0, 3.0, 4.0, 5.0]);
        // Degree 2 on the knots 0 0 1 1 at 0.5 has the weights 1/4, 1/2, 1/4.
        put_floats(&mut e, knots, &[0.0, 0.0, 1.0, 1.0]);
        store_vec(&mut e, points, [4.0, 0.0, 0.0, 0.0]);
        store_vec(&mut e, points + 0x10, [0.0, 4.0, 0.0, 0.0]);
        store_vec(&mut e, points + 0x20, [0.0, 0.0, 4.0, 0.0]);
        e.call(0x00e3_3440, &args![0.5f32, 2i32, knots, points, out]);
        assert_eq!(load_vec(&e, out), [1.0, 2.0, 1.0, 0.0]);
        // Degree 0 copies the first point; a negative degree gives zeros.
        e.call(0x00e3_3440, &args![0.5f32, 0i32, knots, points, out]);
        assert_eq!(load_vec(&e, out), [4.0, 0.0, 0.0, 0.0]);
        e.call(0x00e3_3440, &args![0.5f32, -1i32, knots, points, out]);
        assert_eq!(load_vec(&e, out), [0.0; 4]);
    }

    #[test]
    fn fn_00e33610_blends_two_points_by_the_knot_fraction() {
        let mut e = engine();
        let knots = e.mem.alloc(0x10);
        let points = aligned(&mut e, 0x20);
        let out = aligned(&mut e, 0x10);
        put_floats(&mut e, knots, &[0.0, 2.0]);
        store_vec(&mut e, points, [1.0, 2.0, 3.0, 4.0]);
        store_vec(&mut e, points + 0x10, [5.0, 6.0, 7.0, 8.0]);
        e.call(0x00e3_3610, &args![0.5f32, 1u32, knots, points, out]);
        assert_eq!(load_vec(&e, out), [2.0, 3.0, 4.0, 5.0]);
    }

    #[test]
    fn fn_00e33670_weights_three_points_with_the_quadratic_basis() {
        let mut e = engine();
        let knots = e.mem.alloc(0x10);
        let points = aligned(&mut e, 0x30);
        let out = aligned(&mut e, 0x10);
        put_floats(&mut e, knots, &[0.0, 1.0, 2.0, 3.0]);
        store_vec(&mut e, points, [8.0, 0.0, 0.0, 0.0]);
        store_vec(&mut e, points + 0x10, [0.0, 8.0, 0.0, 0.0]);
        store_vec(&mut e, points + 0x20, [0.0, 0.0, 8.0, 0.0]);
        e.call(0x00e3_3670, &args![1.5f32, 2u32, knots, points, out]);
        assert_eq!(load_vec(&e, out), [1.0, 6.0, 1.0, 0.0]);
    }

    #[test]
    fn fn_00e33760_weights_four_points_with_the_cubic_basis() {
        let mut e = engine();
        let knots = e.mem.alloc(0x20);
        let points = aligned(&mut e, 0x40);
        let out = aligned(&mut e, 0x10);
        put_floats(&mut e, knots, &[0.0, 1.0, 2.0, 3.0, 4.0, 5.0]);
        store_vec(&mut e, points, [48.0, 0.0, 0.0, 0.0]);
        store_vec(&mut e, points + 0x10, [0.0, 48.0, 0.0, 0.0]);
        store_vec(&mut e, points + 0x20, [0.0, 0.0, 48.0, 0.0]);
        store_vec(&mut e, points + 0x30, [0.0, 0.0, 0.0, 48.0]);
        e.call(0x00e3_3760, &args![2.5f32, 3u32, knots, points, out]);
        assert_close(load_vec(&e, out), [1.0, 23.0, 23.0, 1.0], 1e-4);
    }

    #[test]
    fn fn_00e33960_reads_an_animated_curve_a_static_element_or_the_identity() {
        let mut e = engine();
        put_words(&mut e, DEGREE_FUNCTIONS + 4, &[DEGREE_DOUBLE]);
        e.register_double(0x0900_0001, |e, a| {
            e.mem.set_u32(a[1], a[0]);
            Ret::default()
        });
        e.register_double(0x0900_0003, |e, a| {
            e.mem.set_u32(a[1], a[0]);
            e.mem.set_u32(a[1] + 4, 0xabcd);
            Ret::default()
        });
        put_words(&mut e, DECODERS, &[0x0900_0001, 0, 0x0900_0003]);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        e.register_double(DEGREE_DOUBLE, move |e, a| {
            let knots = (e.mem.f32(a[2]), e.mem.f32(a[2] + 4));
            let points = (e.mem.u32(a[3]), e.mem.u32(a[3] + 16));
            sink.borrow_mut().push((a.to_vec(), knots, points));
            Ret::default()
        });
        let stream = aligned(&mut e, 0x40);
        put_bytes(&mut e, stream, &[1, 0, 1, 0, 0, 5, 5]);
        let cursor = e.mem.alloc(4);
        let out = aligned(&mut e, 0x10);
        e.mem.set_u32(cursor, stream);
        e.call(
            0x00e3_3960,
            &args![cursor, 7u32, 0.5f32, 0.75f32, 0x10u32, 0u32, out],
        );
        // Kind 0: aligned data at stream + 8, two elements of four bytes.
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        let (args, knots, points) = &seen[0];
        assert_eq!(args[0], 0.75f32.to_bits());
        assert_eq!(args[1], 1);
        assert_eq!(args[4], out);
        assert_eq!(*knots, (0.0, 2.5));
        assert_eq!(*points, (stream + 8, stream + 12));
        assert_eq!(e.mem.u32(cursor), stream + 16);
        // Only the low flags: the cursor is aligned for kind 2 (alignment 2, size 6) and the
        // decoder gets the aligned address.
        e.mem.set_u32(cursor, stream + 1);
        e.call(
            0x00e3_3960,
            &args![cursor, 0u32, 0.5f32, 0.75f32, 0x01u32, 2u32, out],
        );
        assert_eq!(calls(&e, 0x0900_0003), vec![vec![stream + 2, out]]);
        assert_eq!(e.mem.u32(cursor), stream + 8);
        // Neither: the identity quaternion.
        store_vec(&mut e, out, [9.0; 4]);
        e.call(
            0x00e3_3960,
            &args![cursor, 0u32, 0.5f32, 0.75f32, 0u32, 0u32, out],
        );
        assert_eq!(load_vec(&e, out), [0.0, 0.0, 0.0, 1.0]);
        assert_eq!(e.mem.u32(cursor), stream + 8);
    }

    #[test]
    fn fn_00e33a30_fills_static_lanes_without_animated_bits() {
        let mut e = engine();
        let stream = aligned(&mut e, 0x20);
        put_floats(&mut e, stream + 4, &[1.5, 2.5]);
        let cursor = e.mem.alloc(4);
        e.mem.set_u32(cursor, stream + 1);
        let defaults = aligned(&mut e, 0x10);
        store_vec(&mut e, defaults, [7.0, 8.0, 9.0, 10.0]);
        let out = aligned(&mut e, 0x10);
        e.call(
            0x00e3_3a30,
            &args![cursor, 0u32, 1.0f32, 0.0f32, 0x03u32, 0u32, defaults, out],
        );
        assert_eq!(load_vec(&e, out), [1.5, 2.5, 9.0, 10.0]);
        assert_eq!(e.mem.u32(cursor), stream + 12);
        let fill = calls(&e, STATIC_FILL);
        assert_eq!(fill.len(), 1);
        assert_eq!((fill[0][0], fill[0][2], fill[0][3]), (0x03, defaults, out));
    }

    #[test]
    fn fn_00e33a30_dequantises_bytes_between_the_minimum_and_the_maximum() {
        let mut e = engine();
        put_words(&mut e, DEGREE_FUNCTIONS + 4, &[DEGREE_DOUBLE]);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        e.register_double(DEGREE_DOUBLE, move |e, a| {
            let knots = (e.mem.f32(a[2]), e.mem.f32(a[2] + 4));
            let points: Vec<Vec4> = (0..2).map(|k| load_vec(e, a[3] + 16 * k)).collect();
            sink.borrow_mut().push((a.to_vec(), knots, points));
            Ret::default()
        });
        let stream = aligned(&mut e, 0x40);
        put_bytes(&mut e, stream, &[1, 0, 1, 0, 0, 5, 5]);
        // Minimum and maximum of x and y, then the static z.
        put_floats(&mut e, stream + 8, &[10.0, 20.0, -1.0, 1.0, 7.0]);
        put_bytes(&mut e, stream + 28, &[0, 51, 255, 255]);
        let cursor = e.mem.alloc(4);
        e.mem.set_u32(cursor, stream);
        let defaults = aligned(&mut e, 0x10);
        store_vec(&mut e, defaults, [1.0, 2.0, 3.0, 9.0]);
        let out = aligned(&mut e, 0x10);
        e.call(
            0x00e3_3a30,
            &args![cursor, 7u32, 2.0f32, 0.75f32, 0x34u32, 0u32, defaults, out],
        );
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        let (args, knots, points) = &seen[0];
        assert_eq!((args[0], args[1], args[4]), (0.75f32.to_bits(), 1, out));
        assert_eq!(*knots, (0.0, 10.0));
        assert_close(points[0], [10.0, -0.6, 7.0, 9.0], 1e-6);
        assert_close(points[1], [20.0, 1.0, 7.0, 9.0], 1e-6);
        assert_eq!(e.mem.u32(cursor), stream + 32);
    }

    #[test]
    fn fn_00e33a30_dequantises_words_for_the_other_kinds() {
        let mut e = engine();
        put_words(&mut e, DEGREE_FUNCTIONS + 4, &[DEGREE_DOUBLE]);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        e.register_double(DEGREE_DOUBLE, move |e, a| {
            let points: Vec<Vec4> = (0..2).map(|k| load_vec(e, a[3] + 16 * k)).collect();
            sink.borrow_mut().push(points);
            Ret::default()
        });
        let stream = aligned(&mut e, 0x40);
        put_bytes(&mut e, stream, &[1, 0, 1, 0, 0, 5, 5]);
        put_floats(&mut e, stream + 8, &[100.0, 200.0]);
        put_bytes(&mut e, stream + 16, &[0, 0, 0xff, 0xff]);
        let cursor = e.mem.alloc(4);
        e.mem.set_u32(cursor, stream);
        let defaults = aligned(&mut e, 0x10);
        store_vec(&mut e, defaults, [1.0, 2.0, 3.0, 9.0]);
        let out = aligned(&mut e, 0x10);
        e.call(
            0x00e3_3a30,
            &args![cursor, 7u32, 1.0f32, 0.0f32, 0x10u32, 1u32, defaults, out],
        );
        let seen = seen.borrow();
        assert_eq!(seen[0][0], [100.0, 2.0, 3.0, 9.0]);
        assert_eq!(seen[0][1], [200.0, 2.0, 3.0, 9.0]);
        assert_eq!(e.mem.u32(cursor), stream + 20);
    }

    #[test]
    fn read_nurbs_quaternion_reads_with_the_scale_and_aligns_the_cursor() {
        let mut e = engine();
        e.register_double(VECTOR_READ, |_, _| Ret::default());
        let this = e.mem.alloc(0x80);
        e.mem.set_f32(this + 0x40, 2.0);
        let cursor = e.mem.alloc(4);
        let base = e.mem.alloc(0x10);
        e.mem.set_u32(cursor, base + 1);
        e.call(
            0x00e3_3dc0,
            &args![
                this,
                f32::from_bits(0x7f80_0001),
                9u32,
                0x21u32,
                3u32,
                cursor,
                0x77u32
            ],
        );
        assert_eq!(
            calls(&e, VECTOR_READ),
            vec![vec![
                cursor,
                9,
                2.0f32.to_bits(),
                0x7fc0_0001,
                0x21,
                3,
                0x77
            ]]
        );
        assert_eq!(e.mem.u32(cursor), base + 4);
    }

    #[test]
    fn fn_00e33e10_reads_the_translation_with_the_identity_translation_as_default() {
        let mut e = engine();
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        e.register_double(STATIC_OR_ANIMATED, move |e, a| {
            sink.borrow_mut().push((a.to_vec(), load_vec(e, a[6])));
            Ret::default()
        });
        let this = e.mem.alloc(0x80);
        e.mem.set_f32(this + 0x40, 2.0);
        let cursor = e.mem.alloc(4);
        let base = e.mem.alloc(0x10);
        let out = aligned(&mut e, 0x10);
        e.mem.set_u32(cursor, base + 1);
        e.call(
            0x00e3_3e10,
            &args![this, 0.5f32, 9u32, 0x1234_5601u32, 1u32, cursor, out],
        );
        {
            let seen = seen.borrow();
            let (args, defaults) = &seen[0];
            assert_eq!(
                (args[0], args[1], args[2], args[3]),
                (cursor, 9, 2.0f32.to_bits(), 0.5f32.to_bits())
            );
            assert_eq!((args[4], args[5], args[7]), (0x1234_5601, 1, out));
            assert_ne!(args[6], IDENTITY_TRANSFORM);
            assert_eq!(*defaults, [1.5, 2.5, 3.5, 4.5]);
        }
        assert_eq!(e.mem.u32(cursor), base + 4);
        // A zero mask byte copies the identity translation without reading.
        e.mem.set_u32(cursor, base + 5);
        e.call(
            0x00e3_3e10,
            &args![this, 0.5f32, 9u32, 0x1234_5600u32, 1u32, cursor, out],
        );
        assert_eq!(seen.borrow().len(), 1);
        assert_eq!(load_vec(&e, out), [1.5, 2.5, 3.5, 4.5]);
        assert_eq!(e.mem.u32(cursor), base + 8);
    }

    #[test]
    fn sample_scale_reads_the_scale_with_the_identity_scale_as_default() {
        let mut e = engine();
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        e.register_double(STATIC_OR_ANIMATED, move |e, a| {
            sink.borrow_mut().push((a.to_vec(), load_vec(e, a[6])));
            Ret::default()
        });
        let this = e.mem.alloc(0x80);
        e.mem.set_f32(this + 0x40, 4.0);
        let cursor = e.mem.alloc(4);
        let base = e.mem.alloc(0x10);
        let out = aligned(&mut e, 0x10);
        e.mem.set_u32(cursor, base + 2);
        e.call(
            0x00e3_3e90,
            &args![this, 0.25f32, 3u32, 0x40u32, 2u32, cursor, out],
        );
        {
            let seen = seen.borrow();
            let (args, defaults) = &seen[0];
            assert_eq!(args[2], 4.0f32.to_bits());
            assert_eq!((args[4], args[5], args[7]), (0x40, 2, out));
            assert_eq!(*defaults, [1.0, 1.0, 1.0, 0.5]);
        }
        assert_eq!(e.mem.u32(cursor), base + 4);
        e.call(
            0x00e3_3e90,
            &args![this, 0.25f32, 3u32, 0u32, 2u32, cursor, out],
        );
        assert_eq!(load_vec(&e, out), [1.0, 1.0, 1.0, 0.5]);
        assert_eq!(seen.borrow().len(), 1);
    }

    #[test]
    fn fn_00e33f10_stores_the_first_lane_or_zero() {
        let mut e = engine();
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        e.register_double(STATIC_OR_ANIMATED, move |e, a| {
            sink.borrow_mut().push((a.to_vec(), load_vec(e, a[6])));
            e.mem.set_u32(a[7], 0x7f80_0001);
            Ret::default()
        });
        let this = e.mem.alloc(0x80);
        e.mem.set_f32(this + 0x40, 2.0);
        let cursor = e.mem.alloc(4);
        let base = e.mem.alloc(0x10);
        let out = e.mem.alloc(4);
        e.mem.set_u32(cursor, base + 1);
        e.call(
            0x00e3_3f10,
            &args![this, 0.5f32, 9u32, 0x11u32, 1u32, cursor, out],
        );
        {
            let seen = seen.borrow();
            let (args, defaults) = &seen[0];
            assert_eq!(*defaults, [0.0; 4]);
            assert_eq!((args[4], args[5]), (0x11, 1));
        }
        assert_eq!(e.mem.u32(out), 0x7fc0_0001);
        assert_eq!(e.mem.u32(cursor), base + 4);
        e.mem.set_f32(out, 5.0);
        e.call(
            0x00e3_3f10,
            &args![this, 0.5f32, 9u32, 0x100u32, 1u32, cursor, out],
        );
        assert_eq!(e.mem.f32(out), 0.0);
        assert_eq!(seen.borrow().len(), 1);
    }

    #[test]
    fn fn_00e33f90_samples_the_transform_and_float_tracks_of_a_chunk() {
        let mut e = engine();
        let samples = record_samplers(&mut e);
        e.register_double(BLOCK_AND_TIME, |e, a| {
            e.mem.set_u32(a[3], 1);
            e.mem.set_f32(a[4], 0.25);
            e.mem.set_u8(a[5], 0x42);
            Ret::default()
        });
        let this = e.mem.alloc(0x100);
        e.mem.set_u32(this + 0x34, 0x100);
        e.mem.set_u32(this + 0x10, 2);
        let data = aligned(&mut e, 0x40);
        put_bytes(
            &mut e,
            data + 2,
            &[0x8d, 0x11, 0x22, 0x33, 0x00, 0x44, 0x55, 0x66, 0x13, 0x05],
        );
        let table = aligned(&mut e, 0x20);
        e.mem.set_u32(table + 4, 0x50);
        let chunk = e.mem.alloc(0x30);
        e.mem.set_u32(chunk, this);
        e.mem.set_u32(chunk + 0xc, data);
        e.mem.set_u8(chunk + 0x14, 2);
        e.mem.set_u32(chunk + 0x18, table);
        e.mem.set_u8(chunk + 0x20, 4);
        let transforms = e.mem.alloc(0x60);
        let floats = e.mem.alloc(8);
        e.call(
            0x00e3_3f90,
            &args![9u32, 0.5f32, 2u32, transforms, 2u32, floats, chunk],
        );
        let (fraction, key) = (0.25f32.to_bits(), 0x42);
        let first = 0x100 + data + 2;
        let second = 0x50 + data + 2;
        let want = vec![
            (
                TRANSLATION_SAMPLE,
                vec![this, fraction, key, 0x11, 1, transforms],
                first,
            ),
            (
                ROTATION_SAMPLE,
                vec![this, fraction, key, 0x22, 3, transforms + 0x10],
                first,
            ),
            (
                SCALE_SAMPLE,
                vec![this, fraction, key, 0x33, 2, transforms + 0x20],
                first,
            ),
            (
                TRANSLATION_SAMPLE,
                vec![this, fraction, key, 0x44, 0, transforms + 0x30],
                first,
            ),
            (
                ROTATION_SAMPLE,
                vec![this, fraction, key, 0x55, 0, transforms + 0x40],
                first,
            ),
            (
                SCALE_SAMPLE,
                vec![this, fraction, key, 0x66, 0, transforms + 0x50],
                first,
            ),
            (
                FLOAT_SAMPLE,
                vec![this, fraction, key, 0x11, 1, floats],
                second,
            ),
            (
                FLOAT_SAMPLE,
                vec![this, fraction, key, 0x01, 2, floats + 4],
                second,
            ),
        ];
        assert_eq!(*samples.borrow(), want);
        let block_calls = calls(&e, BLOCK_AND_TIME);
        assert_eq!(block_calls[0][..3], [this, 9, 0.5f32.to_bits()]);
    }

    /// Doubles for `getFrameAndDelta` (frame 9, delta 0.5) and `getBlockAndTime` (block 1,
    /// fraction 0.25, key byte 0x42) as `samplePartialTracks` calls them.
    fn install_block_doubles(e: &mut Engine) {
        e.register_double(GET_FRAME_AND_DELTA, |e, a| {
            e.mem.set_u32(a[2], 9);
            e.mem.set_f32(a[3], 0.5);
            Ret::default()
        });
        e.register_double(BLOCK_AND_TIME, |e, a| {
            e.mem.set_u32(a[3], 1);
            e.mem.set_f32(a[4], 0.25);
            e.mem.set_u8(a[5], 0x42);
            Ret::default()
        });
    }

    /// An animation with the arrays `samplePartialTracks` reads, a data block and the track
    /// headers; returns `(this, data, headers)`.
    fn partial_animation(e: &mut Engine) -> (u32, u32, u32) {
        let this = e.mem.alloc(0x100);
        let data = aligned(e, 0x200);
        let offsets = e.mem.alloc(8);
        put_words(e, offsets, &[0x10, 0x40]);
        let float_offsets = e.mem.alloc(8);
        put_words(e, float_offsets, &[0x08, 0x20]);
        e.mem.set_u32(this + 0x10, 2);
        e.mem.set_u32(this + 0x34, 0x77);
        e.mem.set_u32(this + 0x44, offsets);
        e.mem.set_u32(this + 0x50, float_offsets);
        e.mem.set_u32(this + 0x74, data);
        let headers = e.mem.alloc(0x10);
        put_bytes(
            e,
            headers,
            &[0x8d, 0x11, 0x22, 0x33, 0x00, 0x44, 0x55, 0x66],
        );
        put_bytes(e, data + 8 + 0x40, &[0x13, 0x05]);
        (this, data, headers)
    }

    #[test]
    fn sample_partial_tracks_samples_the_tracks_between_two_profile_records() {
        let mut e = engine();
        let stream = profile_stream(&mut e);
        install_block_doubles(&mut e);
        let samples = record_samplers(&mut e);
        let (this, data, headers) = partial_animation(&mut e);
        e.register_double(PACKED_OFFSETS, move |_, a| Ret {
            eax: if a[3] == 0x8000_0000 { headers } else { 0x5000 },
            ..Ret::default()
        });
        let transforms = e.mem.alloc(0x60);
        let floats = e.mem.alloc(8);
        e.call(
            0x00e3_40e0,
            &args![this, 1.5f32, 2u32, transforms, 2u32, floats, 0u32],
        );
        let offsets = e.mem.u32(this + 0x44);
        assert_eq!(
            calls(&e, PACKED_OFFSETS),
            vec![
                vec![data, offsets, 1, 0x77],
                vec![data, offsets, 1, 0x8000_0000]
            ]
        );
        assert_eq!(calls(&e, GET_FRAME_AND_DELTA)[0][1], 1.5f32.to_bits());
        let block = calls(&e, BLOCK_AND_TIME);
        assert_eq!(block[0][..3], [this, 9, 0.5f32.to_bits()]);
        let (fraction, key) = (0.25f32.to_bits(), 0x3f00_0042);
        let float_cursor = 0x20 + data + 0x40;
        let want = vec![
            (
                TRANSLATION_SAMPLE,
                vec![this, fraction, key, 0x11, 1, transforms],
                0x5000,
            ),
            (
                ROTATION_SAMPLE,
                vec![this, fraction, key, 0x22, 3, transforms + 0x10],
                0x5000,
            ),
            (
                SCALE_SAMPLE,
                vec![this, fraction, key, 0x33, 2, transforms + 0x20],
                0x5000,
            ),
            (
                TRANSLATION_SAMPLE,
                vec![this, fraction, key, 0x44, 0, transforms + 0x30],
                0x5000,
            ),
            (
                ROTATION_SAMPLE,
                vec![this, fraction, key, 0x55, 0, transforms + 0x40],
                0x5000,
            ),
            (
                SCALE_SAMPLE,
                vec![this, fraction, key, 0x66, 0, transforms + 0x50],
                0x5000,
            ),
            (
                FLOAT_SAMPLE,
                vec![this, fraction, key, 0x11, 1, floats],
                float_cursor,
            ),
            (
                FLOAT_SAMPLE,
                vec![this, fraction, key, 0x01, 2, floats + 4],
                float_cursor,
            ),
        ];
        assert_eq!(*samples.borrow(), want);
        assert_eq!(labels(&e, stream), vec![PROFILE_LABEL, PROFILE_END]);
    }

    #[test]
    fn sample_partial_tracks_stops_when_the_headers_are_missing() {
        let mut e = engine();
        let stream = profile_stream(&mut e);
        install_block_doubles(&mut e);
        let samples = record_samplers(&mut e);
        let (this, _, _) = partial_animation(&mut e);
        e.register_double(PACKED_OFFSETS, |_, a| Ret {
            eax: if a[3] == 0x8000_0000 { 0 } else { 0x5000 },
            ..Ret::default()
        });
        let transforms = e.mem.alloc(0x60);
        let floats = e.mem.alloc(8);
        e.call(
            0x00e3_40e0,
            &args![this, 1.5f32, 2u32, transforms, 2u32, floats, 0u32],
        );
        assert!(samples.borrow().is_empty());
        assert_eq!(labels(&e, stream), vec![PROFILE_LABEL, PROFILE_END]);
    }

    /// An animation for the individual-track samplers: `tracks` transform tracks, `floats`
    /// float tracks, the tables at `+0x5c`/`+0x68` holding `0x70000000 + 0x10 * (n + 1)` at
    /// `tracks * 1 + n` (block 1).
    fn individual_animation(e: &mut Engine, transform_tracks: u32, float_tracks: u32) -> u32 {
        let this = e.mem.alloc(0x100);
        e.mem.set_u32(this + 0x10, transform_tracks);
        e.mem.set_u32(this + 0x14, float_tracks);
        e.mem.set_u32(this + 0x74, 0x1000);
        e.mem.set_u32(this + 0x44, 0x2000);
        let transform_table = e.mem.alloc(0x40);
        let float_table = e.mem.alloc(0x40);
        for n in 0..4u32 {
            let value = 0x7000_0000 + 0x10 * (n + 1);
            e.mem
                .set_u32(transform_table + 4 * (transform_tracks + n), value);
            e.mem.set_u32(float_table + 4 * (float_tracks + n), value);
        }
        e.mem.set_u32(this + 0x5c, transform_table);
        e.mem.set_u32(this + 0x68, float_table);
        this
    }

    fn install_individual_block_doubles(e: &mut Engine) {
        e.register_double(GET_FRAME_AND_DELTA, |e, a| {
            e.mem.set_u32(a[2], 5);
            e.mem.set_f32(a[3], 0.25);
            Ret::default()
        });
        e.register_double(BLOCK_AND_TIME, |e, a| {
            e.mem.set_u32(a[3], 1);
            e.mem.set_f32(a[4], 0.5);
            e.mem.set_u8(a[5], 0x77);
            Ret::default()
        });
    }

    #[test]
    fn sample_individual_transform_tracks_samples_the_requested_tracks_and_zeroes_the_rest() {
        let mut e = engine();
        install_individual_block_doubles(&mut e);
        let samples = record_samplers(&mut e);
        let this = individual_animation(&mut e, 3, 0);
        let headers = e.mem.alloc(0x20);
        put_bytes(
            &mut e,
            headers,
            &[
                0x01, 0x10, 0x20, 0x30, 0x8d, 0x11, 0x22, 0x33, 0x42, 0x01, 0x02, 0x03,
            ],
        );
        e.register_double(PACKED_OFFSETS, move |_, a| Ret {
            eax: if a[3] >= 0x7000_0000 {
                a[3] - 0x7000_0000 + 0x100
            } else {
                headers + a[3]
            },
            ..Ret::default()
        });
        let indices = e.mem.alloc(0x10);
        for (i, number) in [1u16, 0xffff, 3, 0, 2].iter().enumerate() {
            e.mem.set_u16(indices + 2 * i as u32, *number);
        }
        let out = e.mem.alloc(5 * 0x30);
        for word in 0..5 * 12 {
            e.mem.set_u32(out + 4 * word, 0xaaaa_aaaa);
        }
        e.call(0x00e3_4310, &args![this, 1.5f32, indices, 5u32, out]);
        let (fraction, key) = (0.5f32.to_bits(), 0x77);
        let track = |kind: u32, masks: [u32; 3], kinds: [u32; 3], at: u32, cursor: u32| {
            vec![
                (
                    kind,
                    vec![this, fraction, key, masks[0], kinds[0], at],
                    cursor,
                ),
                (
                    ROTATION_SAMPLE,
                    vec![this, fraction, key, masks[1], kinds[1], at + 0x10],
                    cursor,
                ),
                (
                    SCALE_SAMPLE,
                    vec![this, fraction, key, masks[2], kinds[2], at + 0x20],
                    cursor,
                ),
            ]
        };
        let mut want = track(
            TRANSLATION_SAMPLE,
            [0x11, 0x22, 0x33],
            [1, 3, 2],
            out,
            0x120,
        );
        want.extend(track(
            TRANSLATION_SAMPLE,
            [0x10, 0x20, 0x30],
            [1, 0, 0],
            out + 0x90,
            0x110,
        ));
        want.extend(track(
            TRANSLATION_SAMPLE,
            [0x01, 0x02, 0x03],
            [2, 0, 1],
            out + 0xc0,
            0x130,
        ));
        assert_eq!(*samples.borrow(), want);
        for word in 12..36 {
            assert_eq!(e.mem.u32(out + 4 * word), 0);
        }
        assert_eq!(calls(&e, GET_FRAME_AND_DELTA)[0][1], 1.5f32.to_bits());
    }

    #[test]
    fn sample_individual_float_tracks_samples_the_requested_tracks_and_zeroes_the_rest() {
        let mut e = engine();
        install_individual_block_doubles(&mut e);
        let samples = record_samplers(&mut e);
        let this = individual_animation(&mut e, 2, 3);
        let flags = e.mem.alloc(0x10);
        put_bytes(&mut e, flags, &[0x13, 0x05, 0x07]);
        e.register_double(PACKED_OFFSETS, move |_, a| Ret {
            eax: if a[3] >= 0x7000_0000 {
                a[3] - 0x7000_0000 + 0x100
            } else {
                flags + (a[3] - 8)
            },
            ..Ret::default()
        });
        let indices = e.mem.alloc(0x10);
        for (i, number) in [2u16, 0xffff, 0, 5].iter().enumerate() {
            e.mem.set_u16(indices + 2 * i as u32, *number);
        }
        let out = e.mem.alloc(0x10);
        for word in 0..4 {
            e.mem.set_f32(out + 4 * word, 9.0);
        }
        e.call(0x00e3_4490, &args![this, 1.5f32, indices, 4u32, out]);
        let (fraction, key) = (0.5f32.to_bits(), 0x77);
        let want = vec![
            (FLOAT_SAMPLE, vec![this, fraction, key, 0x01, 3, out], 0x130),
            (
                FLOAT_SAMPLE,
                vec![this, fraction, key, 0x11, 1, out + 8],
                0x110,
            ),
        ];
        assert_eq!(*samples.borrow(), want);
        assert_eq!(e.mem.f32(out + 4), 0.0);
        assert_eq!(e.mem.f32(out + 12), 0.0);
    }

    #[test]
    fn destructor_empties_the_five_arrays_and_runs_the_base_destructor() {
        let mut e = engine();
        e.register_double(ANIMATION_DESTRUCTOR, |_, _| Ret::default());
        let freed = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&freed);
        e.register_double(0x0900_0011, move |_, a| {
            sink.borrow_mut().push((a[1], a[2]));
            Ret::default()
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0x10, 0x0900_0011);
        let allocator = e.mem.alloc(8);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let this = e.mem.alloc(0x100);
        // (data, size, flags) of the arrays at +0x74, +0x68, +0x5c, +0x50, +0x44.
        let arrays = [
            (0x74, 0x1111, 5, 0x10),
            (0x68, 0x2222, 6, 0x8000_0008),
            (0x5c, 0x3333, 7, 3),
            (0x50, 0x4444, 8, 5),
            (0x44, 0x5555, 9, 0x8000_0000),
        ];
        for (at, data, size, flags) in arrays {
            e.mem.set_u32(this + at, data);
            e.mem.set_u32(this + at + 4, size);
            e.mem.set_u32(this + at + 8, flags);
        }
        e.call(0x00e3_45a0, &args![this]);
        assert_eq!(e.mem.u32(this), VTABLE);
        assert_eq!(
            *freed.borrow(),
            vec![(0x1111, 0x10), (0x3333, 12), (0x4444, 20)]
        );
        for (at, _, _, _) in arrays {
            assert_eq!(e.mem.u32(this + at), 0);
            assert_eq!(e.mem.u32(this + at + 4), 0);
            assert_eq!(e.mem.u32(this + at + 8), 0x8000_0000);
        }
        assert_eq!(calls(&e, ANIMATION_DESTRUCTOR), vec![vec![this]]);
    }

    #[test]
    fn fn_00e346a0_copies_static_lanes_then_default_lanes() {
        let mut e = engine();
        let statics = aligned(&mut e, 0x10);
        let defaults = aligned(&mut e, 0x10);
        let out = aligned(&mut e, 0x10);
        store_vec(&mut e, statics, [1.0, 2.0, 3.0, 4.0]);
        store_vec(&mut e, defaults, [5.0, 6.0, 7.0, 8.0]);
        store_vec(&mut e, out, [9.0; 4]);
        e.call(0x00e3_46a0, &args![0x05u32, statics, defaults, out]);
        assert_eq!(load_vec(&e, out), [1.0, 6.0, 3.0, 8.0]);
        // An animated bit leaves its lane alone.
        store_vec(&mut e, out, [9.0; 4]);
        store_bits(&mut e, statics, [0x7f80_0001, 0, 0, 0]);
        e.call(0x00e3_46a0, &args![0x31u32, statics, defaults, out]);
        assert_eq!(
            load_bits(&e, out),
            [
                0x7fc0_0001,
                9.0f32.to_bits(),
                7.0f32.to_bits(),
                8.0f32.to_bits()
            ]
        );
    }

    fn block_animation(e: &mut Engine) -> u32 {
        let this = e.mem.alloc(0x100);
        e.mem.set_u32(this + 0x2c, 4);
        e.mem.set_u32(this + 0x30, 5);
        e.mem.set_f32(this + 0x3c, 100.0);
        e.mem.set_f32(this + 0x40, 0.25);
        this
    }

    #[test]
    fn get_block_and_time_splits_the_frame_into_block_fraction_and_key() {
        let mut e = engine();
        let this = block_animation(&mut e);
        let outs = e.mem.alloc(0x10);
        let (block, fraction, byte) = (outs, outs + 4, outs + 8);
        e.call(
            0x00e3_4720,
            &args![this, 9u32, 0.5f32, block, fraction, byte],
        );
        assert_eq!(e.mem.u32(block), 2);
        assert_eq!(e.mem.f32(fraction), 0.375);
        assert_eq!(e.mem.u8(byte), 150);
        // The block is limited to the last one: 8850 = 0x2292.
        e.call(
            0x00e3_4720,
            &args![this, 100u32, 0.5f32, block, fraction, byte],
        );
        assert_eq!(e.mem.u32(block), 3);
        assert_eq!(e.mem.f32(fraction), 22.125);
        assert_eq!(e.mem.u8(byte), 0x92);
        // Frame 0 is block 0.
        e.call(
            0x00e3_4720,
            &args![this, 0u32, 0.0f32, block, fraction, byte],
        );
        assert_eq!(e.mem.u32(block), 0);
        assert_eq!(e.mem.f32(fraction), 0.0);
        assert_eq!(e.mem.u8(byte), 0);
        // A frame far past the last block: the frame in the block is negative, 2^32 is
        // added, and the key is out of range (the integer indefinite, low byte 0).
        e.call(
            0x00e3_4720,
            &args![this, 0xffff_fff0u32, 0.5f32, block, fraction, byte],
        );
        assert_eq!(e.mem.u32(block), 3);
        assert_eq!(e.mem.f32(fraction), 1_073_741_824.0);
        assert_eq!(e.mem.u8(byte), 0);
    }

    #[test]
    fn get_block_and_time_splits_the_frame_into_block_fraction_and_key_answers_its_result_pointer()
    {
        let mut e = engine();
        let this = block_animation(&mut e);
        let outs = e.mem.alloc(0x10);
        let (block, fraction, byte) = (outs, outs + 4, outs + 8);
        // The exe leaves the result pointer in EAX at every RET; callers read through it.
        assert_eq!(
            e.call(
                0x00e3_4720,
                &args![this, 9u32, 0.5f32, block, fraction, byte],
            )
            .u32(),
            args![byte][0]
        );
    }

    #[test]
    #[should_panic(expected = "00e34720: division by zero")]
    fn get_block_and_time_faults_on_a_one_frame_block() {
        let mut e = engine();
        let this = block_animation(&mut e);
        e.mem.set_u32(this + 0x30, 1);
        let outs = e.mem.alloc(0x10);
        e.call(
            0x00e3_4720,
            &args![this, 9u32, 0.5f32, outs, outs + 4, outs + 8],
        );
    }
}

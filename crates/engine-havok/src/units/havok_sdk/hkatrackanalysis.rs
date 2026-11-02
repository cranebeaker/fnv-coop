//! `Havok SDK/hkatrackanalysis.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The track analysis counts the degrees of freedom (DOFs) of an animation's tracks. Every
//! transform track has a 16-bit mask word: bits 0-1 describe the translation (value 2 = clear,
//! the identity), bits 2-3 the rotation (8 = clear), bits 4-5 the scale (0x20 = clear) and bits
//! 6-15 tell which of the ten channels (translation x, y, z; rotation x, y, z, w; scale x, y, z)
//! are dynamic. A float track has a mask word of its own (0 = static). The statistics record
//! (`AnalysisStats`, Xbox PDB name, 0x38 bytes) holds, by offset: the static position, rotation,
//! scale and float DOFs (`+0`, `+4`, `+8`, `+0xc`), the clear position, rotation and scale
//! DOFs (`+0x10`, `+0x14`, `+0x18`), the dynamic position, rotation, scale and float DOFs
//! (`+0x1c`, `+0x20`, `+0x24`, `+0x28`), the total transform DOFs (`+0x2c`), the number of float
//! tracks (`+0x30`) and the redundancy (a float, `+0x34`). The names follow the strings
//! `displayReport` prints for them.
//!
//! Unnamed in the Xbox PDB (the names below are descriptions): `00e4ead0` and `00e4ebb0`
//! compute the statistics, `00e4ecb0` packs the dynamic channels of every track into rows.
//! x87 code computes in `f64` and rounds when a `float` is stored; a float copied through the
//! x87 stack is quieted (`x87_quiet_bits`).
//!
//! Not translated: none of these functions has a C++ exception frame.

use crate::prelude::*;

/// The log object (a global pointer), the id of its category call and the strings of the report.
const LOGGER: u32 = 0x0126_8360;
const LOG_CATEGORY_ID: u32 = 0x4324_34a4;
const TEXT_CATEGORY: u32 = 0x010e_c758;
const TEXT_SOURCE_PATH: u32 = 0x010e_c724;
const TEXT_TOTAL: u32 = 0x010e_c74c;
const TEXT_STATIC_POSITION: u32 = 0x010e_c70c;
const TEXT_STATIC_ROTATION: u32 = 0x010e_c6f4;
const TEXT_STATIC_SCALE: u32 = 0x010e_c6e0;
const TEXT_STATIC_FLOAT: u32 = 0x010e_c6cc;
const TEXT_CLEAR_POSITION: u32 = 0x010e_c6b4;
const TEXT_CLEAR_ROTATION: u32 = 0x010e_c69c;
const TEXT_CLEAR_SCALE: u32 = 0x010e_c688;
const TEXT_DYNAMIC_POSITION: u32 = 0x010e_c670;
const TEXT_DYNAMIC_ROTATION: u32 = 0x010e_c658;
const TEXT_DYNAMIC_SCALE: u32 = 0x010e_c644;
const TEXT_DYNAMIC_FLOAT: u32 = 0x010e_c630;
const TEXT_REDUNDANCY: u32 = 0x010e_c624;

/// `hkOstream` (Xbox PDB name) functions: the constructor over a buffer (thiscall, `RET 8`:
/// buffer, size), `operator<<` for a string, an `unsigned` and a `float` (thiscall, `RET 4`,
/// each answers the stream) and the destructor.
const STREAM_INIT: u32 = 0x00cb_1600;
const STREAM_TEXT: u32 = 0x00cc_01c0;
const STREAM_UNSIGNED: u32 = 0x00cc_02d0;
const STREAM_FLOAT: u32 = 0x00cc_0320;
const STREAM_DROP: u32 = 0x00cc_00e0;

/// The frame of `displayReport`: the stream (0xc bytes) at its start, then the 0x200-byte buffer.
const REPORT_FRAME_SIZE: u32 = 0x20c;
const REPORT_BUFFER_AT: u32 = 0xc;
const REPORT_BUFFER_SIZE: u32 = 0x200;

/// The CRT square root (`00ec6040`, argument in ST0, answers in ST0).
const X87_SQRT: u32 = 0x00ec_6040;
/// The float 2.0 `recompose` compares the fourth rotation lane with, the vector (0, 0, 0, 1) it
/// stores for a clear rotation and the float 1.0 it stores for a clear scale.
const TWO_F32: u32 = 0x010e_c620;
const IDENTITY_ROTATION: u32 = 0x010c_71b0;
const ONE_F32: u32 = 0x0101_2068;

// Translated from 00e4e570 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaTrackAnalysis::calcDynamicStatsQsTransform` (Xbox PDB name; cdecl): clears the dynamic
/// position, rotation and scale DOFs (`+0x1c`, `+0x20`, `+0x24`) of `stats`, then adds for each
/// of the first `count` mask words the number of dynamic channels it flags: bits 6 to 8 to the
/// position count, bits 9 to 12 to the rotation count and bits 13 to 15 to the scale count.
pub fn calc_dynamic_stats_qs_transform(e: &mut Engine, words: u32, count: u32, stats: u32) {
    e.mem.set_u32(stats + 0x1c, 0);
    e.mem.set_u32(stats + 0x20, 0);
    e.mem.set_u32(stats + 0x24, 0);
    for i in 0..count {
        let flags = u32::from(e.mem.u16(words.wrapping_add(i.wrapping_mul(2)))) >> 6;
        let bit = |n: u32| (flags >> n) & 1;
        let position = bit(2) + bit(1) + bit(0);
        let rotation = bit(6) + bit(5) + bit(4) + bit(3);
        let scale = bit(9) + bit(8) + bit(7);
        for (at, added) in [(0x1c, position), (0x20, rotation), (0x24, scale)] {
            let total = e.mem.u32(stats + at).wrapping_add(added);
            e.mem.set_u32(stats + at, total);
        }
    }
}

// Translated from 00e4e610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaTrackAnalysis::calcDynamicStats` (Xbox PDB name; cdecl): runs
/// `calcDynamicStatsQsTransform` on `(words, count, stats)`, then sets the dynamic float DOFs
/// (`+0x28`) to the number of zero words among the first `float_count` words of `float_words`.
pub fn calc_dynamic_stats(
    e: &mut Engine,
    words: u32,
    float_words: u32,
    count: u32,
    float_count: u32,
    stats: u32,
) {
    calc_dynamic_stats_qs_transform(e, words, count, stats);
    e.mem.set_u32(stats + 0x28, 0);
    for i in 0..float_count {
        if e.mem.u16(float_words.wrapping_add(i.wrapping_mul(2))) == 0 {
            let zeros = e.mem.u32(stats + 0x28).wrapping_add(1);
            e.mem.set_u32(stats + 0x28, zeros);
        }
    }
}

// Translated from 00e4e660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaTrackAnalysis::AnalysisStats::displayReport` (Xbox PDB name; thiscall, plain `RET`): sends
/// the report to the log object (the global at `01268360`): its slot `0x1c` with the category,
/// then for each of 13 lines an `hkOstream` over the frame's buffer takes the label and the
/// value (an `unsigned`, the redundancy a `float`) and the log's slot `0xc` gets `(0, -1,
/// buffer, source path, line)`, lines `0x2f` to `0x3b`; the stream is destroyed after each line.
/// The values are the total (`+0x30` plus `+0x2c`) and the fields `+0`, `+4`, `+8`, `+0xc`,
/// `+0x10`, `+0x14`, `+0x18`, `+0x1c`, `+0x20`, `+0x24`, `+0x28` and the float `+0x34`. At the end
/// the log's slot `0x20` is entered by a tail jump with the caller's return address.
pub fn analysis_stats_display_report(e: &mut Engine, this: Ptr) {
    let this = this.addr();
    e.with_stack(REPORT_FRAME_SIZE, |e, frame| {
        let stream = frame.addr();
        let buffer = stream + REPORT_BUFFER_AT;
        let logger = e.mem.u32(LOGGER);
        e.vcall(logger, 0x1c, &args![LOG_CATEGORY_ID, TEXT_CATEGORY]);

        let total = e.mem.u32(this + 0x30).wrapping_add(e.mem.u32(this + 0x2c));
        let mut lines: Vec<(u32, u32, u32)> = vec![(TEXT_TOTAL, total, 0x2f)];
        for (text, at, line) in [
            (TEXT_STATIC_POSITION, 0x0, 0x30),
            (TEXT_STATIC_ROTATION, 0x4, 0x31),
            (TEXT_STATIC_SCALE, 0x8, 0x32),
            (TEXT_STATIC_FLOAT, 0xc, 0x33),
            (TEXT_CLEAR_POSITION, 0x10, 0x34),
            (TEXT_CLEAR_ROTATION, 0x14, 0x35),
            (TEXT_CLEAR_SCALE, 0x18, 0x36),
            (TEXT_DYNAMIC_POSITION, 0x1c, 0x37),
            (TEXT_DYNAMIC_ROTATION, 0x20, 0x38),
            (TEXT_DYNAMIC_SCALE, 0x24, 0x39),
            (TEXT_DYNAMIC_FLOAT, 0x28, 0x3a),
        ] {
            lines.push((text, e.mem.u32(this + at), line));
        }
        for (text, value, line) in lines {
            e.call(STREAM_INIT, &args![stream, buffer, REPORT_BUFFER_SIZE]);
            let label = e.call(STREAM_TEXT, &args![stream, text]).u32();
            e.call(STREAM_UNSIGNED, &args![label, value]);
            report_line(e, buffer, line);
            e.call(STREAM_DROP, &args![stream]);
        }

        e.call(STREAM_INIT, &args![stream, buffer, REPORT_BUFFER_SIZE]);
        let redundancy = x87_quiet(e.mem.f32(this + 0x34));
        let label = e.call(STREAM_TEXT, &args![stream, TEXT_REDUNDANCY]).u32();
        e.call(STREAM_FLOAT, &args![label, redundancy]);
        report_line(e, buffer, 0x3b);
        e.call(STREAM_DROP, &args![stream]);

        let logger = e.mem.u32(LOGGER);
        e.vcall(logger, 0x20, &args![]);
    });
}

/// The log's slot `0xc` with `(0, -1, text, source path, line)`.
fn report_line(e: &mut Engine, buffer: u32, line: u32) {
    let logger = e.mem.u32(LOGGER);
    e.vcall(
        logger,
        0xc,
        &args![0u32, 0xffff_ffffu32, buffer, TEXT_SOURCE_PATH, line],
    );
}

/// Counts the clear channels of the first `count` mask words into `stats`: `+0x10` gets 3 for a
/// word whose bits 0-1 are 2, `+0x14` gets 4 when bits 2-3 are 8 and `+0x18` gets 3 when bits
/// 4-5 are 0x20. Only the low byte of a word is read.
fn count_clear_channels(e: &mut Engine, words: u32, count: u32, stats: u32) {
    for i in 0..count {
        let low = e.mem.u8(words.wrapping_add(i.wrapping_mul(2)));
        for (mask, wanted, at, added) in [
            (0x03u8, 0x02u8, 0x10u32, 3u32),
            (0x0c, 0x08, 0x14, 4),
            (0x30, 0x20, 0x18, 3),
        ] {
            if low & mask == wanted {
                let total = e.mem.u32(stats + at).wrapping_add(added);
                e.mem.set_u32(stats + at, total);
            }
        }
    }
}

/// The redundancy: `numerator / denominator` of two unsigned values, rounded to a float.
fn redundancy(numerator: u32, denominator: u32) -> f32 {
    (f64::from(numerator) / f64::from(denominator)) as f32
}

// Translated from 00e4ead0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The statistics of `count` transform tracks with mask words `words` (cdecl, `(words, count,
/// stats)`; unnamed in the Xbox PDB). Sets the redundancy to 0, the total (`+0x2c`) to
/// `count * 10` and the fields `+0x30`, `+0` to `+0xc`, `+0x10` to `+0x18` and `+0x28` to 0,
/// counts the clear channels, runs `calcDynamicStatsQsTransform` and derives the static DOFs:
/// position `count * 3 - dynamic position - clear position`, rotation `count * 4 - clear
/// rotation - dynamic rotation`, scale `count * 3 - dynamic scale - clear scale`. The
/// redundancy (`+0x34`) is the total divided by the dynamic DOFs (unsigned), when they are not
/// 0.
pub fn fn_00e4ead0(e: &mut Engine, words: u32, count: u32, stats: u32) {
    e.mem.set_f32(stats + 0x34, 0.0);
    e.mem.set_u32(stats + 0x2c, count.wrapping_mul(10));
    for at in [0x30, 0x0, 0x4, 0x8, 0xc, 0x10, 0x14, 0x18, 0x28] {
        e.mem.set_u32(stats + at, 0);
    }
    count_clear_channels(e, words, count, stats);
    calc_dynamic_stats_qs_transform(e, words, count, stats);

    let dynamic_position = e.mem.u32(stats + 0x1c);
    let dynamic_rotation = e.mem.u32(stats + 0x20);
    let dynamic_scale = e.mem.u32(stats + 0x24);
    let static_position = count
        .wrapping_mul(3)
        .wrapping_sub(dynamic_position)
        .wrapping_sub(e.mem.u32(stats + 0x10));
    e.mem.set_u32(stats, static_position);
    let static_rotation = count
        .wrapping_mul(4)
        .wrapping_sub(e.mem.u32(stats + 0x14))
        .wrapping_sub(dynamic_rotation);
    e.mem.set_u32(stats + 4, static_rotation);
    let static_scale = count
        .wrapping_mul(3)
        .wrapping_sub(dynamic_scale)
        .wrapping_sub(e.mem.u32(stats + 0x18));
    e.mem.set_u32(stats + 8, static_scale);

    let dynamic = dynamic_scale
        .wrapping_add(dynamic_rotation)
        .wrapping_add(dynamic_position);
    if dynamic != 0 {
        let total = e.mem.u32(stats + 0x2c);
        e.mem.set_f32(stats + 0x34, redundancy(total, dynamic));
    }
}

// Translated from 00e4ebb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The statistics of `count` transform tracks and `float_count` float tracks (cdecl, `(words,
/// float_words, count, float_count, stats)`; unnamed in the Xbox PDB). Like `00e4ead0` with the
/// float tracks: the float track count is stored at `+0x30`, `calcDynamicStats` runs instead of
/// the transform-only count, the static float DOFs (`+0xc`) are `float_count - dynamic float DOFs`
/// and the redundancy is `(float_count + count * 10)` divided by the dynamic DOFs including the
/// float ones, when they are not 0.
pub fn fn_00e4ebb0(
    e: &mut Engine,
    words: u32,
    float_words: u32,
    count: u32,
    float_count: u32,
    stats: u32,
) {
    e.mem.set_f32(stats + 0x34, 0.0);
    e.mem.set_u32(stats + 0x2c, count.wrapping_mul(10));
    e.mem.set_u32(stats + 0x30, float_count);
    for at in [0x0, 0x4, 0x8, 0x10, 0x14, 0x18, 0x28] {
        e.mem.set_u32(stats + at, 0);
    }
    count_clear_channels(e, words, count, stats);
    calc_dynamic_stats(e, words, float_words, count, float_count, stats);

    let dynamic_position = e.mem.u32(stats + 0x1c);
    let dynamic_rotation = e.mem.u32(stats + 0x20);
    let dynamic_scale = e.mem.u32(stats + 0x24);
    let dynamic_float = e.mem.u32(stats + 0x28);
    let static_position = count
        .wrapping_mul(3)
        .wrapping_sub(e.mem.u32(stats + 0x10))
        .wrapping_sub(dynamic_position);
    e.mem.set_u32(stats, static_position);
    let static_rotation = count
        .wrapping_mul(4)
        .wrapping_sub(dynamic_rotation)
        .wrapping_sub(e.mem.u32(stats + 0x14));
    e.mem.set_u32(stats + 4, static_rotation);
    let static_scale = count
        .wrapping_mul(3)
        .wrapping_sub(e.mem.u32(stats + 0x18))
        .wrapping_sub(dynamic_scale);
    e.mem.set_u32(stats + 8, static_scale);
    e.mem
        .set_u32(stats + 0xc, float_count.wrapping_sub(dynamic_float));

    let dynamic = dynamic_float
        .wrapping_add(dynamic_scale)
        .wrapping_add(dynamic_rotation)
        .wrapping_add(dynamic_position);
    if dynamic != 0 {
        let total = e
            .mem
            .u32(stats + 0x30)
            .wrapping_add(e.mem.u32(stats + 0x2c));
        e.mem.set_f32(stats + 0x34, redundancy(total, dynamic));
    }
}

// Translated from 00e4ecb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Packs the dynamic channels of every track into rows of floats (cdecl, `(analysis, out)`;
/// unnamed in the Xbox PDB; the field names are unconfirmed). `analysis` holds: the pointer
/// `+0` to the transform samples (frame by frame, each track 12 floats: translation, rotation,
/// scale), the transform track count `+4`, the pointer `+8` to the float samples (frame by
/// frame, one float per track), the float track count `+0xc`, the frame count `+0x10`, a second
/// count `+0x14` and the pointer `+0x18` to the mask words (the transform tracks, then the
/// float tracks). The row length is the larger of the two counts. For each transform track and
/// each group whose mask bits are clear (position bits 0-1, rotation bits 2-3, scale bits 4-5),
/// every channel flagged dynamic gets a row, in the order translation x, y, z, rotation x, y, z,
/// w, scale x, y, z: the frame count values of the channel, then the last one repeated up to
/// the row length (the flag bits, of the word shifted right by 6, are 2, 1, 0 for the
/// translation, 6, 5, 4, 3 for the rotation and 9, 8, 7 for the scale). The float tracks whose
/// mask word is 0 follow, each a row the same way. Every value passes the x87 stack
/// (`x87_quiet_bits`). The exe unrolls the copies by four; the result is the same.
pub fn fn_00e4ecb0(e: &mut Engine, analysis: Ptr, out: u32) {
    let analysis = analysis.addr();
    let samples = e.mem.u32(analysis);
    let tracks = e.mem.u32(analysis + 4);
    let float_samples = e.mem.u32(analysis + 8);
    let float_tracks = e.mem.u32(analysis + 0xc);
    let frames = e.mem.u32(analysis + 0x10);
    let second_count = e.mem.u32(analysis + 0x14);
    let masks = e.mem.u32(analysis + 0x18);
    let row_length = if frames <= second_count {
        second_count
    } else {
        frames
    };

    let mut row = 0u32;
    for track in 0..tracks {
        let word = u32::from(e.mem.u16(masks.wrapping_add(track.wrapping_mul(2))));
        let flags = word >> 6;
        // (mask, first float of the group, flag bit of its first channel, channel count)
        for (mask, group, first_bit, channels) in
            [(0x3u32, 0u32, 2u32, 3u32), (0xc, 4, 6, 4), (0x30, 8, 9, 3)]
        {
            if word & mask != 0 {
                continue;
            }
            for channel in 0..channels {
                if flags & (1 << (first_bit - channel)) == 0 {
                    continue;
                }
                let first = group
                    .wrapping_add(channel)
                    .wrapping_add(track.wrapping_mul(12));
                let row_at = out.wrapping_add(row.wrapping_mul(row_length).wrapping_mul(4));
                copy_row(
                    e,
                    samples,
                    row_at,
                    (first, tracks.wrapping_mul(12)),
                    (frames, row_length),
                );
                row = row.wrapping_add(1);
            }
        }
    }
    for track in 0..float_tracks {
        let at = tracks.wrapping_add(track).wrapping_mul(2);
        if e.mem.u16(masks.wrapping_add(at)) == 0 {
            let row_at = out.wrapping_add(row.wrapping_mul(row_length).wrapping_mul(4));
            copy_row(
                e,
                float_samples,
                row_at,
                (track, float_tracks),
                (frames, row_length),
            );
            row = row.wrapping_add(1);
        }
    }
}

/// One row of `00e4ecb0`: `frames` floats read from `source` at float index `first + frame *
/// stride`, then the last of them repeated up to `row_length` floats, written at `row`.
fn copy_row(
    e: &mut Engine,
    source: u32,
    row: u32,
    (first, stride): (u32, u32),
    (frames, row_length): (u32, u32),
) {
    for frame in 0..frames {
        let index = first.wrapping_add(frame.wrapping_mul(stride));
        let bits = e.mem.u32(source.wrapping_add(index.wrapping_mul(4)));
        e.mem.set_u32(
            row.wrapping_add(frame.wrapping_mul(4)),
            x87_quiet_bits(bits),
        );
    }
    let last = first.wrapping_add(frames.wrapping_sub(1).wrapping_mul(stride));
    let mut frame = frames;
    while frame < row_length {
        let bits = e.mem.u32(source.wrapping_add(last.wrapping_mul(4)));
        e.mem.set_u32(
            row.wrapping_add(frame.wrapping_mul(4)),
            x87_quiet_bits(bits),
        );
        frame += 1;
    }
}

/// A float read through the x87 stack from `*cursor`, which advances by 4.
fn take_float(e: &mut Engine, cursor: &mut u32) -> u32 {
    let bits = x87_quiet_bits(e.mem.u32(*cursor));
    *cursor = cursor.wrapping_add(4);
    bits
}

// Translated from 00e4f640 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaTrackAnalysis::recompose` (Xbox PDB name; cdecl, `(analysis, transforms, floats)`): the
/// inverse of the packing, rebuilding the transforms of every track from streams of floats.
/// `analysis` holds the transform track count `+0`, the float track count `+4`, the mask words
/// `+8` and the float mask words `+0xc`, the stream `+0x10` read for a channel whose flag bit
/// is clear and the stream `+0x18` read for a set flag bit, and for the float tracks the stream
/// `+0x14` (read for a mask word that is not 0) and `+0x1c` (for 0). Each transform is 12
/// floats at `transforms` (0x30 bytes). Per track, with `flags` the mask word shifted right by
/// 6:
/// * translation: when mask bits 0-1 are 2 the four floats are 0; otherwise x, y, z are read
///   by flag bits 4, 2, 1 (the fourth float is not written);
/// * rotation: when bits 2-3 are 8 the four floats are the vector (0, 0, 0, 1) at `010c71b0`;
///   otherwise x, y, z, w are read by flag bits 0x40, 0x20, 0x10, 0x8. When `|w|` equals the
///   float 2.0 at `010ec620` (a marker for a rebuilt fourth component) `w` becomes
///   `sqrt(1 - min(x*x + y*y + z*z, 1))` with the CRT square root `00ec6040` (its argument
///   rounded to a float), negated unless `w` was greater than 0;
/// * scale: when bits 4-5 are 0x20 the four floats are 1.0; otherwise x, y, z are read by flag
///   bits 0x200, 0x100, 0x80 (the fourth float is not written).
///
/// Element `i` of `floats` then comes from the `+0x14` stream when the mask word of float
/// track `i` is not 0, else from the `+0x1c` stream. Every float read passes the x87 stack
/// (`x87_quiet_bits`).
pub fn track_analysis_recompose(e: &mut Engine, analysis: Ptr, transforms: u32, floats: u32) {
    let analysis = analysis.addr();
    let track_count = e.mem.u32(analysis);
    let float_count = e.mem.u32(analysis + 4);
    let masks = e.mem.u32(analysis + 8);
    let float_masks = e.mem.u32(analysis + 0xc);
    let mut clear_cursor = e.mem.u32(analysis + 0x10);
    let mut set_cursor = e.mem.u32(analysis + 0x18);
    let mut nonzero_cursor = e.mem.u32(analysis + 0x14);
    let mut zero_cursor = e.mem.u32(analysis + 0x1c);

    for track in 0..track_count {
        let word = u32::from(e.mem.u16(masks.wrapping_add(track.wrapping_mul(2))));
        let flags = word >> 6;
        let out = transforms.wrapping_add(track.wrapping_mul(0x30));

        if word & 3 == 2 {
            for lane in 0..4 {
                e.mem.set_u32(out + lane * 4, 0);
            }
        } else {
            for (lane, bit) in [(0u32, 4u32), (1, 2), (2, 1)] {
                let cursor = if flags & bit != 0 {
                    &mut set_cursor
                } else {
                    &mut clear_cursor
                };
                let bits = take_float(e, cursor);
                e.mem.set_u32(out + lane * 4, bits);
            }
        }

        if word & 0xc == 8 {
            for lane in 0..4 {
                let bits = e.mem.u32(IDENTITY_ROTATION + lane * 4);
                e.mem.set_u32(out + 0x10 + lane * 4, bits);
            }
        } else {
            for (lane, bit) in [(0u32, 0x40u32), (1, 0x20), (2, 0x10), (3, 0x8)] {
                let cursor = if flags & bit != 0 {
                    &mut set_cursor
                } else {
                    &mut clear_cursor
                };
                let bits = take_float(e, cursor);
                e.mem.set_u32(out + 0x10 + lane * 4, bits);
            }
            let w = e.mem.f32(out + 0x1c);
            if w.abs() == e.mem.f32(TWO_F32) {
                let x = f64::from(e.mem.f32(out + 0x10));
                let y = f64::from(e.mem.f32(out + 0x14));
                let z = f64::from(e.mem.f32(out + 0x18));
                let squared = (x * x + y * y + z * z) as f32;
                let remainder = if squared > 1.0 {
                    0.0
                } else {
                    1.0 - f64::from(squared)
                };
                let root = e.call(X87_SQRT, &args![f64::from(remainder as f32)]).f64() as f32;
                let rebuilt = if w > 0.0 { root } else { -root };
                e.mem.set_f32(out + 0x1c, rebuilt);
            }
        }

        if word & 0x30 == 0x20 {
            let one = e.mem.u32(ONE_F32);
            for lane in 0..4 {
                e.mem.set_u32(out + 0x20 + lane * 4, one);
            }
        } else {
            for (lane, bit) in [(0u32, 0x200u32), (1, 0x100), (2, 0x80)] {
                let cursor = if flags & bit != 0 {
                    &mut set_cursor
                } else {
                    &mut clear_cursor
                };
                let bits = take_float(e, cursor);
                e.mem.set_u32(out + 0x20 + lane * 4, bits);
            }
        }
    }

    for index in 0..float_count {
        let mask = e.mem.u16(float_masks.wrapping_add(index.wrapping_mul(2)));
        let cursor = if mask == 0 {
            &mut zero_cursor
        } else {
            &mut nonzero_cursor
        };
        let bits = take_float(e, cursor);
        e.mem
            .set_u32(floats.wrapping_add(index.wrapping_mul(4)), bits);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e4_e570, calc_dynamic_stats_qs_transform(u32, u32, u32)),
        entry!(0x00e4_e610, calc_dynamic_stats(u32, u32, u32, u32, u32)),
        entry!(0x00e4_e660, analysis_stats_display_report(Ptr)),
        entry!(0x00e4_ead0, fn_00e4ead0(u32, u32, u32)),
        entry!(0x00e4_ebb0, fn_00e4ebb0(u32, u32, u32, u32, u32)),
        entry!(0x00e4_ecb0, fn_00e4ecb0(Ptr, u32)),
        entry!(0x00e4_f640, track_analysis_recompose(Ptr, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const LOG_CATEGORY_DOUBLE: u32 = 0x0900_0201;
    const LOG_MESSAGE_DOUBLE: u32 = 0x0900_0202;
    const LOG_FLUSH_DOUBLE: u32 = 0x0900_0203;

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

    fn word_array(e: &mut Engine, values: &[u16]) -> u32 {
        let at = e.mem.alloc(values.len() as u32 * 2 + 2);
        for (i, value) in values.iter().enumerate() {
            e.mem.set_u16(at + 2 * i as u32, *value);
        }
        at
    }

    fn float_array(e: &mut Engine, values: &[f32]) -> u32 {
        let at = e.mem.alloc(values.len() as u32 * 4 + 4);
        for (i, value) in values.iter().enumerate() {
            e.mem.set_f32(at + 4 * i as u32, *value);
        }
        at
    }

    /// A statistics record whose fields all read 0xaaaaaaaa.
    fn dirty_stats(e: &mut Engine) -> u32 {
        let stats = e.mem.alloc(0x38);
        for at in (0..0x38).step_by(4) {
            e.mem.set_u32(stats + at, 0xaaaa_aaaa);
        }
        stats
    }

    fn fields(e: &Engine, stats: u32) -> Vec<u32> {
        (0..0x38)
            .step_by(4)
            .map(|at| e.mem.u32(stats + at))
            .collect()
    }

    #[test]
    fn dynamic_channels_are_counted_per_group() {
        let mut e = engine();
        // 0x01c2: position bits 6-8 (3). 0x3e28: rotation bits 9-12 (4) and scale bit 13 (1).
        // 0xffff: 3, 4 and 3.
        let words = word_array(&mut e, &[0x01c2, 0x3e28, 0, 0xffff]);
        let stats = dirty_stats(&mut e);
        e.call(0x00e4_e570, &args![words, 4u32, stats]);
        assert_eq!(e.mem.u32(stats + 0x1c), 6);
        assert_eq!(e.mem.u32(stats + 0x20), 8);
        assert_eq!(e.mem.u32(stats + 0x24), 4);
        // The other fields are untouched.
        assert_eq!(e.mem.u32(stats), 0xaaaa_aaaa);
        assert_eq!(e.mem.u32(stats + 0x28), 0xaaaa_aaaa);

        e.call(0x00e4_e570, &args![words, 0u32, stats]);
        assert_eq!(
            [0x1c, 0x20, 0x24].map(|at| e.mem.u32(stats + at)),
            [0, 0, 0]
        );
    }

    #[test]
    fn zero_float_words_are_the_dynamic_floats() {
        let mut e = engine();
        let words = word_array(&mut e, &[0x01c0]);
        // Only the first four of the five words count.
        let float_words = word_array(&mut e, &[0, 5, 0, 0, 0]);
        let stats = dirty_stats(&mut e);
        e.call(0x00e4_e610, &args![words, float_words, 1u32, 4u32, stats]);
        assert_eq!(e.mem.u32(stats + 0x1c), 3);
        assert_eq!(e.mem.u32(stats + 0x28), 3);

        e.call(0x00e4_e610, &args![words, float_words, 1u32, 0u32, stats]);
        assert_eq!(e.mem.u32(stats + 0x28), 0);
    }

    #[test]
    fn transform_statistics_derive_the_static_dofs() {
        let mut e = engine();
        // Track 0: clear position, 3 dynamic position DOFs. Track 1: clear rotation and scale,
        // 4 dynamic rotation DOFs and 1 dynamic scale DOF. Track 2: all static.
        let words = word_array(&mut e, &[0x01c2, 0x3e28, 0]);
        let stats = dirty_stats(&mut e);
        e.call(0x00e4_ead0, &args![words, 3u32, stats]);
        let expected = [
            3,                         // static position: 9 - 3 - 3
            4,                         // static rotation: 12 - 4 - 4
            5,                         // static scale: 9 - 1 - 3
            0,                         // static float
            3,                         // clear position
            4,                         // clear rotation
            3,                         // clear scale
            3,                         // dynamic position
            4,                         // dynamic rotation
            1,                         // dynamic scale
            0,                         // dynamic float
            30,                        // total
            0,                         // float tracks
            (30.0f32 / 8.0).to_bits(), // redundancy
        ];
        assert_eq!(fields(&e, stats), expected);

        // Nothing dynamic: the redundancy stays 0.
        let stats = dirty_stats(&mut e);
        e.call(0x00e4_ead0, &args![words, 0u32, stats]);
        assert_eq!(fields(&e, stats), vec![0; 14]);
    }

    #[test]
    fn statistics_with_float_tracks() {
        let mut e = engine();
        let words = word_array(&mut e, &[0x01c2, 0x3e28, 0]);
        let float_words = word_array(&mut e, &[0, 1, 0]);
        let stats = dirty_stats(&mut e);
        e.call(0x00e4_ebb0, &args![words, float_words, 3u32, 3u32, stats]);
        let expected = [
            3,                          // static position
            4,                          // static rotation
            5,                          // static scale
            1,                          // static float: 3 - 2
            3,                          // clear position
            4,                          // clear rotation
            3,                          // clear scale
            3,                          // dynamic position
            4,                          // dynamic rotation
            1,                          // dynamic scale
            2,                          // dynamic float
            30,                         // total
            3,                          // float tracks
            (33.0f32 / 10.0).to_bits(), // redundancy: (3 + 30) / (2 + 1 + 4 + 3)
        ];
        assert_eq!(fields(&e, stats), expected);

        // Float tracks only, none dynamic: the redundancy stays 0.
        let float_words = word_array(&mut e, &[1, 1]);
        let stats = dirty_stats(&mut e);
        e.call(0x00e4_ebb0, &args![words, float_words, 0u32, 2u32, stats]);
        let mut expected = vec![0u32; 14];
        expected[3] = 2;
        expected[12] = 2;
        assert_eq!(fields(&e, stats), expected);
    }

    #[test]
    fn report_goes_line_by_line_to_the_log() {
        let mut e = engine();
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
        for address in [STREAM_INIT, STREAM_DROP] {
            e.register_double(address, |_, _| Ret::default());
        }
        for address in [STREAM_TEXT, STREAM_UNSIGNED, STREAM_FLOAT] {
            e.register_double(address, |_, a| Ret {
                eax: a[0],
                ..Ret::default()
            });
        }
        let stats = e.mem.alloc(0x38);
        for i in 0..13u32 {
            e.mem.set_u32(stats + 4 * i, 100 + i);
        }
        e.mem.set_f32(stats + 0x34, 2.5);

        e.call(0x00e4_e660, &args![stats]);

        assert_eq!(
            calls(&e, LOG_CATEGORY_DOUBLE),
            vec![vec![logger, LOG_CATEGORY_ID, TEXT_CATEGORY]]
        );
        let inits = calls(&e, STREAM_INIT);
        assert_eq!(inits.len(), 13);
        let (stream, buffer) = (inits[0][0], inits[0][1]);
        assert_eq!(buffer, stream + 0xc);
        assert!(inits
            .iter()
            .all(|call| call == &vec![stream, buffer, 0x200]));
        assert_eq!(calls(&e, STREAM_DROP), vec![vec![stream]; 13]);
        let texts: Vec<u32> = calls(&e, STREAM_TEXT).iter().map(|call| call[1]).collect();
        assert_eq!(
            texts,
            vec![
                TEXT_TOTAL,
                TEXT_STATIC_POSITION,
                TEXT_STATIC_ROTATION,
                TEXT_STATIC_SCALE,
                TEXT_STATIC_FLOAT,
                TEXT_CLEAR_POSITION,
                TEXT_CLEAR_ROTATION,
                TEXT_CLEAR_SCALE,
                TEXT_DYNAMIC_POSITION,
                TEXT_DYNAMIC_ROTATION,
                TEXT_DYNAMIC_SCALE,
                TEXT_DYNAMIC_FLOAT,
                TEXT_REDUNDANCY
            ]
        );
        // The total is +0x30 (112) plus +0x2c (111), then +0 to +0x28.
        let values: Vec<u32> = [223u32]
            .into_iter()
            .chain((0..11).map(|i| 100 + i))
            .collect();
        assert_eq!(
            calls(&e, STREAM_UNSIGNED),
            values
                .iter()
                .map(|value| vec![stream, *value])
                .collect::<Vec<_>>()
        );
        assert_eq!(
            calls(&e, STREAM_FLOAT),
            vec![vec![stream, 2.5f32.to_bits()]]
        );
        let messages = calls(&e, LOG_MESSAGE_DOUBLE);
        let expected: Vec<Vec<u32>> = (0x2f..=0x3b)
            .map(|line| vec![logger, 0, 0xffff_ffff, buffer, TEXT_SOURCE_PATH, line])
            .collect();
        assert_eq!(messages, expected);
        assert_eq!(calls(&e, LOG_FLUSH_DOUBLE), vec![vec![logger]]);
    }

    /// The analysis record of `00e4ecb0`.
    #[allow(clippy::too_many_arguments)]
    fn pack_analysis(
        e: &mut Engine,
        samples: u32,
        tracks: u32,
        float_samples: u32,
        float_tracks: u32,
        frames: u32,
        second_count: u32,
        masks: u32,
    ) -> u32 {
        let analysis = e.mem.alloc(0x20);
        for (at, value) in [
            (0, samples),
            (4, tracks),
            (8, float_samples),
            (0xc, float_tracks),
            (0x10, frames),
            (0x14, second_count),
            (0x18, masks),
        ] {
            e.mem.set_u32(analysis + at, value);
        }
        analysis
    }

    #[test]
    fn dynamic_channels_become_padded_rows() {
        let mut e = engine();
        // 2 tracks of 12 floats, 2 frames; sample i is 100 + i except frame 1 of track 0's
        // translation x, a signalling NaN.
        let samples = e.mem.alloc(48 * 4);
        for i in 0..48u32 {
            e.mem.set_f32(samples + 4 * i, 100.0 + i as f32);
        }
        e.mem.set_u32(samples + 24 * 4, 0x7f80_0001);
        let float_samples = float_array(&mut e, &[7.0, 9.0]);
        // Track 0 (flags 0x14d): translation x and z, rotation x and w, scale y. Track 1:
        // translation y only (rotation and scale are clear). The float track is dynamic.
        let masks = word_array(&mut e, &[0x5340, 0x00a8, 0]);
        let analysis = pack_analysis(&mut e, samples, 2, float_samples, 1, 2, 3, masks);
        let out = e.mem.alloc(7 * 3 * 4 + 4);
        for i in 0..22u32 {
            e.mem.set_u32(out + 4 * i, 0xdddd_dddd);
        }

        e.call(0x00e4_ecb0, &args![analysis, out]);

        let sample = |e: &Engine, index: u32| x87_quiet_bits(e.mem.u32(samples + 4 * index));
        // (first sample of the channel, track stride 24 floats between the frames)
        let channels = [0u32, 2, 4, 7, 9, 13];
        let mut expected: Vec<u32> = vec![];
        for first in channels {
            let frame0 = sample(&e, first);
            let frame1 = sample(&e, first + 24);
            expected.extend([frame0, frame1, frame1]);
        }
        expected.extend([7.0f32.to_bits(), 9.0f32.to_bits(), 9.0f32.to_bits()]);
        let produced: Vec<u32> = (0..21).map(|i| e.mem.u32(out + 4 * i)).collect();
        assert_eq!(produced, expected);
        assert_eq!(expected[1], 0x7fc0_0001);
        assert_eq!(e.mem.u32(out + 84), 0xdddd_dddd);
    }

    #[test]
    fn rows_without_padding_cover_all_frames() {
        let mut e = engine();
        // One track, 5 frames (more than the unrolled step of four) and a second count of 2:
        // the rows are 5 floats long. Translation y is dynamic.
        let samples = e.mem.alloc(5 * 12 * 4);
        for i in 0..60u32 {
            e.mem.set_f32(samples + 4 * i, i as f32);
        }
        let masks = word_array(&mut e, &[0x00a8]);
        let analysis = pack_analysis(&mut e, samples, 1, 0, 0, 5, 2, masks);
        let out = e.mem.alloc(5 * 4);
        e.call(0x00e4_ecb0, &args![analysis, out]);
        let produced: Vec<f32> = (0..5).map(|i| e.mem.f32(out + 4 * i)).collect();
        assert_eq!(produced, vec![1.0, 13.0, 25.0, 37.0, 49.0]);
    }

    /// An engine with the constants `recompose` reads and a square root double.
    fn recompose_engine() -> Engine {
        let mut e = engine();
        e.mem.set_f32(TWO_F32, 2.0);
        e.mem.set_f32(ONE_F32, 1.0);
        for (lane, value) in [0.0f32, 0.0, 0.0, 1.0].into_iter().enumerate() {
            e.mem.set_f32(IDENTITY_ROTATION + 4 * lane as u32, value);
        }
        e.register_double(X87_SQRT, |_, a| {
            let value = f64::from_bits((u64::from(a[1]) << 32) | u64::from(a[0]));
            Ret {
                st0: value.sqrt(),
                ..Ret::default()
            }
        });
        e
    }

    /// The analysis record of `recompose` for `tracks` mask words and `float_tracks` float mask
    /// words, with the four streams.
    fn recompose_analysis(
        e: &mut Engine,
        masks: &[u16],
        float_masks: &[u16],
        streams: [&[f32]; 4],
    ) -> u32 {
        let track_count = masks.len() as u32;
        let masks = word_array(e, masks);
        let float_masks_at = word_array(e, float_masks);
        let analysis = e.mem.alloc(0x20);
        e.mem.set_u32(analysis, track_count);
        e.mem.set_u32(analysis + 8, masks);
        e.mem.set_u32(analysis + 0xc, float_masks_at);
        e.mem.set_u32(analysis + 4, float_masks.len() as u32);
        // Streams: clear (+0x10), nonzero float (+0x14), set (+0x18), zero float (+0x1c).
        for (at, values) in [
            (0x10, streams[0]),
            (0x14, streams[1]),
            (0x18, streams[2]),
            (0x1c, streams[3]),
        ] {
            let stream = float_array(e, values);
            e.mem.set_u32(analysis + at, stream);
        }
        analysis
    }

    fn filled_transforms(e: &mut Engine, tracks: u32) -> u32 {
        let out = e.mem.alloc(tracks * 0x30);
        for i in 0..tracks * 12 {
            e.mem.set_u32(out + 4 * i, 0xdddd_dddd);
        }
        out
    }

    #[test]
    fn transforms_and_floats_are_rebuilt_from_the_streams() {
        let mut e = recompose_engine();
        // Track 0 (0xb002, flags 0x2c0): clear translation; rotation x from the set stream,
        // y, z, w from the clear stream (w = 2.0 is rebuilt); scale x and z from the set stream,
        // y from the clear stream. Track 1 (0x1a8, flags 6): translation x and y from the set
        // stream, z from the clear stream; clear rotation and scale.
        let analysis = recompose_analysis(
            &mut e,
            &[0xb002, 0x01a8],
            &[0, 1, 0],
            [
                &[0.5, 0.5, 2.0, 4.0, 6.0],
                &[11.0],
                &[0.5, 3.0, 5.0, 8.0, 9.0],
                &[21.0, 22.0],
            ],
        );
        let transforms = filled_transforms(&mut e, 2);
        let floats = e.mem.alloc(12);

        e.call(0x00e4_f640, &args![analysis, transforms, floats]);

        let read: Vec<f32> = (0..24).map(|i| e.mem.f32(transforms + 4 * i)).collect();
        let untouched = f32::from_bits(0xdddd_dddd);
        assert_eq!(
            &read[..12],
            &[0.0, 0.0, 0.0, 0.0, 0.5, 0.5, 0.5, 0.5, 3.0, 4.0, 5.0, untouched][..]
        );
        assert_eq!(
            &read[12..],
            &[8.0, 9.0, 6.0, untouched, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0][..]
        );
        let produced: Vec<f32> = (0..3).map(|i| e.mem.f32(floats + 4 * i)).collect();
        assert_eq!(produced, vec![21.0, 11.0, 22.0]);
        // 1 - (0.25 + 0.25 + 0.25) = 0.25 went to the square root.
        assert_eq!(
            calls(&e, X87_SQRT),
            vec![vec![
                0.25f64.to_bits() as u32,
                (0.25f64.to_bits() >> 32) as u32
            ]]
        );
    }

    /// The rotation `recompose` makes of one track whose rotation is read from the clear stream.
    fn rotation_of(e: &mut Engine, values: [u32; 4]) -> [u32; 4] {
        let stream = e.mem.alloc(16);
        for (lane, bits) in values.into_iter().enumerate() {
            e.mem.set_u32(stream + 4 * lane as u32, bits);
        }
        let masks = word_array(e, &[0x0022]);
        let analysis = e.mem.alloc(0x20);
        e.mem.set_u32(analysis, 1);
        e.mem.set_u32(analysis + 8, masks);
        e.mem.set_u32(analysis + 0x10, stream);
        let transforms = filled_transforms(e, 1);
        e.call(0x00e4_f640, &args![analysis, transforms, 0u32]);
        [0, 1, 2, 3].map(|lane| e.mem.u32(transforms + 0x10 + 4 * lane))
    }

    #[test]
    fn a_marker_w_of_minus_two_gives_a_negative_root_and_clips_the_sum() {
        let mut e = recompose_engine();
        // 1 + 1 + 1 is above 1: the root of 0 is taken and negated.
        let rotation = rotation_of(
            &mut e,
            [
                1.0f32.to_bits(),
                1.0f32.to_bits(),
                1.0f32.to_bits(),
                (-2.0f32).to_bits(),
            ],
        );
        assert_eq!(rotation[..3], [1.0f32.to_bits(); 3]);
        assert_eq!(rotation[3], (-0.0f32).to_bits());
        assert_eq!(calls(&e, X87_SQRT), vec![vec![0, 0]]);
    }

    #[test]
    fn other_w_values_are_copied_and_signalling_nans_are_quieted() {
        let mut e = recompose_engine();
        let rotation = rotation_of(
            &mut e,
            [
                0x7f80_0001,
                0.25f32.to_bits(),
                0.0f32.to_bits(),
                0.5f32.to_bits(),
            ],
        );
        assert_eq!(
            rotation,
            [0x7fc0_0001, 0.25f32.to_bits(), 0, 0.5f32.to_bits()]
        );
        assert!(calls(&e, X87_SQRT).is_empty());
    }
}

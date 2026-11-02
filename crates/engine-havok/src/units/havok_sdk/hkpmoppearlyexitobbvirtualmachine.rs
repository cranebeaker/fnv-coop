//! `Havok SDK/hkpmoppearlyexitobbvirtualmachine.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkpMoppEarlyExitObbVirtualMachine` (Xbox PDB) asks whether any leaf of a MOPP tree touches
//! an oriented box, stopping at the first one. The machine object (`this`) holds the integer
//! bounds of the box in the code's grid: the upper bounds at `+0x10`, `+0x14` and `+0x18`
//! (plus one), the lower bounds at `+0x20`, `+0x24` and `+0x28` (minus one), their centers at
//! `+0x30`, `+0x34` and `+0x38`, the bound of the radius at `+0x3C`, the "hit" byte at `+0x40`
//! and the MOPP code object at `+0x44` (origin vector at `+0x10`, scale at `+0x1C`, code bytes
//! at `+0x20`). The walk keeps a "query" of 18 integers (see `fn_00e645b0`): the upper bounds
//! at `[0]` to `[2]`, the lower ones at `[4]` to `[6]`, the centers at `[8]` to `[10]`, the
//! radius bound at `[11]`, the offsets of the frame at `[12]` to `[14]`, an id word at `[15]`,
//! the shift at `[16]` and a kept word at `[17]`. The member names are guesses and not
//! confirmed in the exe, except the ones of the Xbox PDB.

#[allow(unused_imports)]
use crate::prelude::*;

/// A byte in the exe image (`0126efba`): non-zero once the product key check has passed.
const LICENSE_FLAG: u32 = 0x0126_efba;
/// `hkpProcessFlyingColors` (Xbox PDB, `00df22f0`, cdecl, one byte pointer), translated in
/// another unit and called by address.
const LICENSE_CHECK: u32 = 0x00df_22f0;
/// Constructor of `hkErrStream` (`00cb1600`, `(this, buffer, size)`).
const ERR_STREAM_CONSTRUCT: u32 = 0x00cb_1600;
/// Writes text into the stream (`00cc01c0`, `(this, text)`) and destroys it (`00cc00e0`).
const OSTREAM_WRITE_TEXT: u32 = 0x00cc_01c0;
const OSTREAM_DESTRUCT: u32 = 0x00cc_00e0;
/// The global pointer to the Havok error reporter (`01268360`); its vtable offset 0xc reports
/// a message `(level, id, text, file, line)` and returns non-zero to request a break.
const ERROR_REPORTER: u32 = 0x0126_8360;
/// The message `Unknown command - This mopp data has been corrupted (check for m...` (`010d5100`)
/// and the source file string (`010ecfbc`).
const CORRUPTED_TEXT: u32 = 0x010d_5100;
const ERROR_FILE: u32 = 0x010e_cfbc;
/// The four floats of a 16-byte vector.
fn vec4(e: &Engine, at: u32) -> [f32; 4] {
    [
        e.mem.f32(at),
        e.mem.f32(at + 4),
        e.mem.f32(at + 8),
        e.mem.f32(at + 12),
    ]
}

fn byte(e: &Engine, at: u32) -> u32 {
    u32::from(e.mem.u8(at))
}

/// The byte at `at` as a signed number.
fn sbyte(e: &Engine, at: u32) -> i32 {
    i32::from(e.mem.u8(at))
}

/// `count` bytes at `at` as a big-endian number.
fn big_endian(e: &Engine, at: u32, count: u32) -> u32 {
    let mut value = 0u32;
    for i in 0..count {
        value = (value << 8) | byte(e, at + i);
    }
    value
}

/// The signed 16-bit word at `at`.
fn short(e: &Engine, at: u32) -> i32 {
    i32::from(e.mem.u16(at) as i16)
}

/// `FISTP` of a single-precision value with the default rounding mode (nearest, ties to even);
/// the integer indefinite `0x80000000` for NaN and out-of-range values.
fn fistp(value: f32) -> i32 {
    // Ties to even without `round_ties_even` (newer than the MSRV).
    let wide = f64::from(value);
    let mut rounded = wide.round();
    if (wide - wide.trunc()).abs() == 0.5 && rounded % 2.0 != 0.0 {
        rounded -= wide.signum();
    }
    if rounded.is_nan() || rounded < f64::from(i32::MIN) || rounded > f64::from(i32::MAX) {
        i32::MIN
    } else {
        rounded as i32
    }
}

/// Copies the 18 words of the query at `from` to `to` (`REP MOVSD`).
fn copy_query(e: &mut Engine, to: u32, from: u32) {
    for i in 0..18 {
        let value = e.mem.u32(from + 4 * i);
        e.mem.set_u32(to + 4 * i, value);
    }
}

/// The report of a command byte no case knows (`hkErrStream`, the reporter's virtual
/// function at 0xc, then `INT3`, which this translation panics on: it is a breakpoint trap
/// in the exe, not a call).
fn report_corrupted(e: &mut Engine, stream: u32, buffer: u32) {
    e.call(ERR_STREAM_CONSTRUCT, &args![stream, buffer, 0x200u32]);
    e.call(OSTREAM_WRITE_TEXT, &args![stream, CORRUPTED_TEXT]);
    let reporter: u32 = e.global(ERROR_REPORTER);
    let wanted_break = e
        .vcall(
            reporter,
            0xc,
            &args![3u32, 0x1298_feddu32, buffer, ERROR_FILE, 0x156u32],
        )
        .u32();
    if wanted_break != 0 {
        panic!("00e64494: the exe's INT3 trap (the error reporter asked for a break)");
    }
    e.call(OSTREAM_DESTRUCT, &args![stream]);
}

// Translated from 00e63f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMoppEarlyExitObbVirtualMachine::queryOBBOnTree` (Xbox PDB), `RET 8` with `(query,
/// code)`: interprets the MOPP bytecode at `code` against the box, with `query` the 18-word
/// state (see the unit comment) of the current frame. Commands 1 to 4 make a new frame
/// (shifted by the command number, relative to the box bounds of `this`), 5 to 7 skip, 9 to
/// 0xB and 0x60 to 0x6B change the frame's words, 0x10 to 0x25 test an interval on an axis or a
/// sum of axes and walk the child blocks (the first recursively), 0x26 to 0x2B end the walk
/// without a hit when the box lies outside the interval and 0x30 to 0x53 are leaves: they set
/// the hit byte `this + 0x40`, which ends every walk.
pub fn fn_00e63f10(e: &mut Engine, this: Ptr, query: Ptr, code: Ptr) {
    let me = this.addr();
    if e.mem.u8(me + 0x40) != 0 {
        return;
    }
    // The frame is laid out like the exe's: the local query at +0x18, the kept word of the
    // 0x60 to 0x6B commands at +0x5C, the error stream at +0x6C and its buffer at +0x78.
    e.with_stack(0x280, |e, frame| {
        let f = frame.addr();
        let local = f + 0x18;
        let mut q = query.addr();
        let mut pc = code.addr();
        loop {
            let op = byte(e, pc);
            let word = |e: &Engine, index: u32| e.mem.u32(q + 4 * index) as i32;
            let this_word = |e: &Engine, offset: u32| e.mem.u32(me + offset) as i32;
            match op {
                0 => return,
                1..=4 => {
                    let shift = op;
                    let x = (byte(e, pc + 1) as i32).wrapping_add(word(e, 12)) << shift;
                    let y = (byte(e, pc + 2) as i32).wrapping_add(word(e, 13)) << shift;
                    let z = (byte(e, pc + 3) as i32).wrapping_add(word(e, 14)) << shift;
                    let total = word(e, 16).wrapping_add(op as i32);
                    let down = (0x10i32.wrapping_sub(total) & 0x1f) as u32;
                    let sar = |value: i32| value >> down;
                    let values = [
                        sar(this_word(e, 0x20)).wrapping_sub(x),
                        sar(this_word(e, 0x24)).wrapping_sub(y),
                        sar(this_word(e, 0x28)).wrapping_sub(z),
                        0,
                        sar(this_word(e, 0x10)).wrapping_add(1).wrapping_sub(x),
                        sar(this_word(e, 0x14)).wrapping_add(1).wrapping_sub(y),
                        sar(this_word(e, 0x18)).wrapping_add(1).wrapping_sub(z),
                    ];
                    let centers = [
                        sar(this_word(e, 0x30)).wrapping_sub(x),
                        sar(this_word(e, 0x34)).wrapping_sub(y),
                        sar(this_word(e, 0x38)).wrapping_sub(z),
                        sar(this_word(e, 0x3c)).wrapping_add(1),
                    ];
                    let kept_id = word(e, 15);
                    let kept_word = word(e, 17);
                    // The upper bounds [0] to [2], the lower ones [4] to [6].
                    e.mem.set_u32(local + 0x10, values[0] as u32);
                    e.mem.set_u32(local + 0x14, values[1] as u32);
                    e.mem.set_u32(local + 0x18, values[2] as u32);
                    e.mem.set_u32(local, values[4] as u32);
                    e.mem.set_u32(local + 4, values[5] as u32);
                    e.mem.set_u32(local + 8, values[6] as u32);
                    for (i, center) in centers.iter().enumerate() {
                        e.mem.set_u32(local + 0x20 + 4 * i as u32, *center as u32);
                    }
                    e.mem.set_u32(local + 0x30, x as u32);
                    e.mem.set_u32(local + 0x34, y as u32);
                    e.mem.set_u32(local + 0x38, z as u32);
                    e.mem.set_u32(local + 0x40, total as u32);
                    e.mem.set_u32(local + 0x44, kept_word as u32);
                    e.mem.set_u32(local + 0x3c, kept_id as u32);
                    q = local;
                    pc = pc.wrapping_add(4);
                }
                5 => pc = pc.wrapping_add(byte(e, pc + 1)).wrapping_add(2),
                6 => pc = pc.wrapping_add(big_endian(e, pc + 1, 2)).wrapping_add(3),
                7 => pc = pc.wrapping_add(big_endian(e, pc + 1, 3)).wrapping_add(4),
                9..=0xb => {
                    let (value, length) = match op {
                        9 => (byte(e, pc + 1), 2),
                        0xa => (big_endian(e, pc + 1, 2), 3),
                        _ => (big_endian(e, pc + 1, 4), 5),
                    };
                    if q != local {
                        copy_query(e, local, q);
                        q = local;
                    }
                    let id = if op == 0xb {
                        value
                    } else {
                        e.mem.u32(local + 0x3c).wrapping_add(value)
                    };
                    e.mem.set_u32(local + 0x3c, id);
                    pc = pc.wrapping_add(length);
                }
                0x10..=0x12 => {
                    let a = op - 0x10;
                    let b1 = sbyte(e, pc + 1);
                    let b2 = sbyte(e, pc + 2);
                    let near = byte(e, pc + 3);
                    let first = pc.wrapping_add(4);
                    pc = first;
                    if b2 < word(e, a) {
                        pc = pc.wrapping_add(near);
                        if word(e, a + 4) < b1 {
                            fn_00e63f10(e, this, Ptr::new(q), Ptr::new(first));
                        }
                    }
                }
                0x13..=0x1c => {
                    let b1 = sbyte(e, pc + 1);
                    let b2 = sbyte(e, pc + 2);
                    let (first_edge, second_edge, edge, scale) = match op {
                        0x13 | 0x15 | 0x17 => {
                            let sum = match op {
                                0x13 => word(e, 10).wrapping_add(word(e, 9)),
                                0x15 => word(e, 10).wrapping_add(word(e, 8)),
                                _ => word(e, 9).wrapping_add(word(e, 8)),
                            };
                            let half = word(e, 11);
                            (
                                b1 * 2,
                                b2 * 2,
                                sum,
                                (half >> 1).wrapping_add(1).wrapping_add(half),
                            )
                        }
                        0x14 | 0x16 | 0x18 => {
                            let diff = match op {
                                0x14 => word(e, 9).wrapping_sub(word(e, 10)),
                                0x16 => word(e, 8).wrapping_sub(word(e, 10)),
                                _ => word(e, 8).wrapping_sub(word(e, 9)),
                            };
                            let half = word(e, 11);
                            (
                                b1 * 2,
                                b2 * 2,
                                diff.wrapping_add(0xff),
                                (half >> 1).wrapping_add(1).wrapping_add(half),
                            )
                        }
                        _ => {
                            let edge = match op {
                                0x19 => word(e, 10)
                                    .wrapping_add(word(e, 9))
                                    .wrapping_add(word(e, 8)),
                                0x1a => word(e, 9)
                                    .wrapping_sub(word(e, 10))
                                    .wrapping_add(word(e, 8))
                                    .wrapping_add(0xff),
                                0x1b => word(e, 10)
                                    .wrapping_sub(word(e, 9))
                                    .wrapping_add(word(e, 8))
                                    .wrapping_add(0xff),
                                _ => word(e, 8)
                                    .wrapping_sub(word(e, 10))
                                    .wrapping_sub(word(e, 9))
                                    .wrapping_add(0x1fe),
                            };
                            (b1 * 3, b2 * 3, edge, word(e, 11).wrapping_mul(4))
                        }
                    };
                    let near = byte(e, pc + 3);
                    let first = pc.wrapping_add(4);
                    pc = first;
                    if second_edge < scale.wrapping_add(edge) {
                        pc = pc.wrapping_add(near);
                        if edge < scale.wrapping_add(first_edge) {
                            fn_00e63f10(e, this, Ptr::new(q), Ptr::new(first));
                        }
                    }
                }
                0x20..=0x22 => {
                    let a = op - 0x20;
                    let b1 = sbyte(e, pc + 1);
                    let near = byte(e, pc + 2);
                    let first = pc.wrapping_add(3);
                    pc = first;
                    if b1 < word(e, a) {
                        pc = pc.wrapping_add(near);
                        if word(e, a + 4) <= b1 {
                            fn_00e63f10(e, this, Ptr::new(q), Ptr::new(first));
                        }
                    }
                }
                0x23..=0x25 => {
                    let a = op - 0x23;
                    let b1 = sbyte(e, pc + 1);
                    let b2 = sbyte(e, pc + 2);
                    let far = big_endian(e, pc + 3, 2);
                    let near = big_endian(e, pc + 5, 2);
                    let start = pc.wrapping_add(7);
                    if b2 < word(e, a) {
                        pc = start.wrapping_add(near);
                        if word(e, a + 4) < b1 {
                            fn_00e63f10(e, this, Ptr::new(q), Ptr::new(start.wrapping_add(far)));
                        }
                    } else {
                        pc = start.wrapping_add(far);
                    }
                }
                0x26..=0x28 => {
                    let a = op - 0x26;
                    if word(e, a) < sbyte(e, pc + 1) {
                        return;
                    }
                    if word(e, a + 4) >= sbyte(e, pc + 2) {
                        return;
                    }
                    pc = pc.wrapping_add(3);
                }
                0x29..=0x2b => {
                    let a = op - 0x29;
                    let first = big_endian(e, pc + 1, 3) as i32;
                    if this_word(e, 0x10 + 4 * a) < first {
                        return;
                    }
                    let second = big_endian(e, pc + 4, 3) as i32;
                    if this_word(e, 0x20 + 4 * a) > second {
                        return;
                    }
                    pc = pc.wrapping_add(7);
                }
                0x30..=0x53 => {
                    e.mem.set_u8(me + 0x40, 1);
                    return;
                }
                0x60..=0x6b => {
                    let (value, length) = match op {
                        0x60..=0x63 => (byte(e, pc + 1), 2),
                        0x64..=0x67 => (big_endian(e, pc + 1, 2), 3),
                        _ => (big_endian(e, pc + 1, 4), 5),
                    };
                    pc = pc.wrapping_add(length);
                    e.mem.set_u32(f + 0x5c + 4 * (op & 3), value);
                    let kept = e.mem.u32(local + 0x44);
                    if q != local {
                        copy_query(e, local, q);
                        q = local;
                    }
                    e.mem.set_u32(local + 0x44, kept);
                }
                _ => report_corrupted(e, f + 0x6c, f + 0x78),
            }
            if e.mem.u8(me + 0x40) != 0 {
                return;
            }
        }
    });
}

// Translated from 00e645b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMoppEarlyExitObbVirtualMachine::generateQueryFromNode` (Xbox PDB), `RET 0x10` with
/// `(half_extents, transform, radius, query)`: the extent of the box on each axis is
/// `|h1 * row1| + |h0 * row0| + |h2 * row2| + radius` per component (single-precision SSE),
/// the center the row 3 of `transform` (the translation at `+0x30`). The lower and upper
/// corners are converted to the code's grid, `((corner - origin) * scale)` rounded, minus or
/// plus one, and stored as the bounds in `this` (`+0x20`, `+0x24`, `+0x28` and
/// `+0x10`, `+0x14`, `+0x18`), with the centers (`+0x30` to `+0x38`) and the radius bound
/// `round((radius + |half_extents|) * scale) + 1` (`+0x3C`). `query` gets the upper 16 bits of
/// these bounds as its integers (see the unit comment) and its words 12 to 17 cleared.
pub fn fn_00e645b0(
    e: &mut Engine,
    this: Ptr,
    half_extents: Ptr,
    transform: Ptr,
    radius: Ptr,
    query: Ptr,
) {
    let me = this.addr();
    let half = vec4(e, half_extents.addr());
    let m = transform.addr();
    let rows = [vec4(e, m), vec4(e, m + 0x10), vec4(e, m + 0x20)];
    let translation = vec4(e, m + 0x30);
    let radius_value = e.mem.f32(radius.addr());
    let mut extent = [0.0f32; 4];
    for i in 0..4 {
        let along_y = (half[1] * rows[1][i]).abs();
        let along_x = (half[0] * rows[0][i]).abs();
        let along_z = (half[2] * rows[2][i]).abs();
        extent[i] = ((along_y + along_x) + along_z) + radius_value;
    }
    let mut lower = [0.0f32; 4];
    let mut upper = [0.0f32; 4];
    for i in 0..4 {
        lower[i] = translation[i] + -extent[i];
        upper[i] = translation[i] + extent[i];
    }
    let object = e.mem.u32(me + 0x44);
    let scale = e.mem.f32(object + 0x1c);
    // One bound converted to the grid: `((value - origin) * scale)` through a single store,
    // then rounded.
    let grid = |e: &Engine, value: f32, axis: u32| -> i32 {
        let origin = e.mem.f32(object + 0x10 + 4 * axis);
        let scaled = ((f64::from(value) - f64::from(origin)) * f64::from(scale)) as f32;
        fistp(scaled)
    };
    let low_x = grid(e, lower[0], 0).wrapping_sub(1);
    e.mem.set_u32(me + 0x20, low_x as u32);
    let high_x = grid(e, upper[0], 0).wrapping_add(1);
    e.mem.set_u32(me + 0x10, high_x as u32);
    let low_y = grid(e, lower[1], 1).wrapping_sub(1);
    e.mem.set_u32(me + 0x24, low_y as u32);
    let high_y = grid(e, upper[1], 1).wrapping_add(1);
    e.mem.set_u32(me + 0x14, high_y as u32);
    let low_z = grid(e, lower[2], 2).wrapping_sub(1);
    e.mem.set_u32(me + 0x28, low_z as u32);
    let high_z = grid(e, upper[2], 2).wrapping_add(1);
    e.mem
        .set_u32(me + 0x30, (low_x.wrapping_add(high_x) >> 1) as u32);
    e.mem
        .set_u32(me + 0x34, (low_y.wrapping_add(high_y) >> 1) as u32);
    e.mem.set_u32(me + 0x18, high_z as u32);
    e.mem
        .set_u32(me + 0x38, (low_z.wrapping_add(high_z) >> 1) as u32);
    let squares = [half[0] * half[0], half[1] * half[1], half[2] * half[2]];
    let length = ((squares[2]) + (squares[1] + squares[0])).sqrt();
    let reach = (f64::from(x87_quiet(radius_value)) + f64::from(length)) as f32;
    let scaled = (f64::from(reach) * f64::from(scale)) as f32;
    e.mem
        .set_u32(me + 0x3c, fistp(scaled).wrapping_add(1) as u32);
    let out = query.addr();
    let words = [
        (4, short(e, me + 0x22)),
        (0, short(e, me + 0x12).wrapping_add(1)),
        (5, short(e, me + 0x26)),
        (1, short(e, me + 0x16).wrapping_add(1)),
        (6, short(e, me + 0x2a)),
        (2, short(e, me + 0x1a).wrapping_add(1)),
        (8, short(e, me + 0x32)),
        (9, short(e, me + 0x36)),
        (10, short(e, me + 0x3a)),
        (11, short(e, me + 0x3e).wrapping_add(1)),
    ];
    for (index, value) in words {
        e.mem.set_u32(out + 4 * index, value as u32);
    }
    for index in 12..18 {
        e.mem.set_u32(out + 4 * index, 0);
    }
}

// Translated from 00e647e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMoppEarlyExitObbVirtualMachine::queryObb` (Xbox PDB), `RET 0x14` with `(result, mopp,
/// transform, half_extents, radius)`: after the one-time product check (`00df22f0` on the
/// byte `0126efba`; if it still reads 0 the result byte is 0), stores the MOPP object at
/// `+0x44`, builds the query of the box (`generateQueryFromNode`), clears the hit byte at
/// `+0x40`, walks the code at `mopp + 0x20` (`queryOBBOnTree`) and writes the hit byte to
/// `result`.
/// Answers `result` (`EAX` at both `RET`s; the caller tests the flag through it, 00e5cf29).
pub fn fn_00e647e0(
    e: &mut Engine,
    this: Ptr,
    result: Ptr,
    mopp: Ptr,
    transform: Ptr,
    half_extents: Ptr,
    radius: Ptr,
) -> Ptr {
    let me = this.addr();
    if e.mem.u8(LICENSE_FLAG) == 0 {
        e.call(LICENSE_CHECK, &args![LICENSE_FLAG]);
        if e.mem.u8(LICENSE_FLAG) == 0 {
            e.mem.set_u8(result.addr(), 0);
            return result;
        }
    }
    e.mem.set_u32(me + 0x44, mopp.addr());
    let code = e.mem.u32(mopp.addr() + 0x20);
    e.with_stack(0x48, |e, query| {
        fn_00e645b0(e, this, half_extents, transform, radius, query);
        e.mem.set_u32(query.addr() + 0x44, 0);
        e.mem.set_u8(me + 0x40, 0);
        fn_00e63f10(e, this, query, Ptr::new(code));
    });
    let hit = e.mem.u8(me + 0x40);
    e.mem.set_u8(result.addr(), hit);
    result
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e6_3f10, fn_00e63f10(Ptr, Ptr, Ptr)),
        entry!(0x00e6_45b0, fn_00e645b0(Ptr, Ptr, Ptr, Ptr, Ptr)),
        entry!(0x00e6_47e0, fn_00e647e0(Ptr, Ptr, Ptr, Ptr, Ptr, Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod a06_tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    const REPORT_SLOT: u32 = 0x0900_0204;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// A machine (`this`), a query of 18 words and the code to walk.
    struct Rig {
        e: Engine,
        this: u32,
        query: u32,
        code: u32,
    }

    fn rig(code: &[u8]) -> Rig {
        let mut e = Engine::new();
        // The tests run without the exe, so its data range is mapped.
        e.map(0x0100_0000, 0x0030_0000);
        let this = e.mem.alloc(0x80);
        let query = e.mem.alloc(0x48);
        let at = e.mem.alloc(0x100);
        e.mem.write(at, code);
        Rig {
            e,
            this,
            query,
            code: at,
        }
    }

    #[test]
    fn the_obb_query_answers_its_result_pointer() {
        let mut r = rig(&[]);
        // Without the licence flag (the check leaves it clear) nothing is walked.
        r.e.register_double(LICENSE_CHECK, |_, _| Ret::default());
        let result = r.e.mem.alloc(4);
        r.e.mem.set_u8(result, 9);
        // EAX is `result` at both RETs; the caller tests the flag through it (00e5cf29).
        let answered =
            r.e.call(0x00e6_47e0, &args![r.this, result, 0u32, 0u32, 0u32, 0u32])
                .u32();
        assert_eq!(answered, result);
        assert_eq!(r.e.mem.u8(result), 0);
    }

    impl Rig {
        /// Sets query words by index.
        fn set_query(&mut self, words: &[(u32, i32)]) {
            for &(index, value) in words {
                self.e.mem.set_u32(self.query + 4 * index, value as u32);
            }
        }

        /// Sets words of the machine by offset.
        fn set_this(&mut self, words: &[(u32, i32)]) {
            for &(offset, value) in words {
                self.e.mem.set_u32(self.this + offset, value as u32);
            }
        }

        /// Walks the code and tells whether a leaf was reached.
        fn run(&mut self) -> bool {
            let (this, query, code) = (self.this, self.query, self.code);
            self.e.call(0x00e63f10, &args![this, query, code]);
            self.e.mem.u8(self.this + 0x40) != 0
        }
    }

    fn hits(code: &[u8], words: &[(u32, i32)]) -> bool {
        let mut r = rig(code);
        r.set_query(words);
        r.run()
    }

    /// A command followed by the leaf `0x30` as its first block and a return as its second.
    fn first_leaf(head: &[u8]) -> Vec<u8> {
        [head, &[0x30, 0x00]].concat()
    }

    /// A command followed by a return as its first block and the leaf `0x30` as its second.
    fn second_leaf(head: &[u8]) -> Vec<u8> {
        [head, &[0x00, 0x30]].concat()
    }

    #[test]
    fn leaves_set_the_hit_byte_and_end_the_walk() {
        for op in [0x30u8, 0x3f, 0x4f, 0x50, 0x51, 0x52, 0x53] {
            assert!(hits(&[op, 0xee, 0xee], &[]), "{op:#x}");
        }
        assert!(!hits(&[0x00, 0x30], &[]));
    }

    #[test]
    fn a_walk_that_has_a_hit_already_does_nothing() {
        let mut r = rig(&[0xee]);
        r.e.mem.set_u8(r.this + 0x40, 1);
        assert!(r.run());
    }

    #[test]
    fn skip_commands_move_over_their_operands() {
        for code in [
            vec![5u8, 2, 0xee, 0xee, 0x30],
            vec![6, 0, 1, 0xee, 0x30],
            vec![7, 0, 0, 1, 0xee, 0x30],
        ] {
            assert!(hits(&code, &[]), "{code:?}");
        }
    }

    #[test]
    fn the_id_and_scratch_commands_have_their_lengths() {
        for code in [
            vec![9u8, 0xee, 0x30],
            vec![0xa, 0xee, 0xee, 0x30],
            vec![0xb, 0xee, 0xee, 0xee, 0xee, 0x30],
            vec![0x60, 0xee, 0x30],
            vec![0x63, 0xee, 0x30],
            vec![0x64, 0xee, 0xee, 0x30],
            vec![0x67, 0xee, 0xee, 0x30],
            vec![0x68, 0xee, 0xee, 0xee, 0xee, 0x30],
            vec![0x6b, 0xee, 0xee, 0xee, 0xee, 0x30],
        ] {
            assert!(hits(&code, &[]), "{code:?}");
        }
    }

    #[test]
    fn the_commands_that_change_the_frame_copy_the_query_first() {
        // The upper bound 6 and the lower bound 6 of axis 0 must survive the copy.
        for head in [
            vec![9u8, 1],
            vec![0xa, 0, 1],
            vec![0x60, 1],
            vec![0x65, 0, 1],
        ] {
            let code = [&head[..], &[0x26, 6, 7, 0x30]].concat();
            assert!(hits(&code, &[(0, 6), (4, 6)]), "{head:?}");
            assert!(!hits(&code, &[(0, 5), (4, 6)]), "{head:?}");
        }
    }

    #[test]
    fn the_two_byte_axis_commands_walk_the_blocks_the_box_reaches() {
        for axis in 0..3u32 {
            let head = [0x10 + axis as u8, 10, 5, 1];
            let (first, second) = (first_leaf(&head), second_leaf(&head));
            // (upper, lower, first block reached, second block reached): the second block
            // needs the low byte (5) below the upper bound, the first one the lower bound
            // below the high byte (10) as well.
            for (upper, lower, in_first, in_second) in [
                (3, 0, true, false),
                (5, 0, true, false),
                (8, 7, true, true),
                (8, 12, false, true),
                (6, 9, true, true),
                (6, 10, false, true),
            ] {
                let words = [(axis, upper), (axis + 4, lower)];
                assert_eq!(hits(&first, &words), in_first, "{axis} {upper} {lower}");
                assert_eq!(hits(&second, &words), in_second, "{axis} {upper} {lower}");
            }
        }
    }

    #[test]
    fn the_one_byte_axis_commands_walk_the_blocks_the_box_reaches() {
        for axis in 0..3u32 {
            let head = [0x20 + axis as u8, 5, 1];
            let (first, second) = (first_leaf(&head), second_leaf(&head));
            for (upper, lower, in_first, in_second) in
                [(5, 0, true, false), (6, 5, true, true), (6, 6, false, true)]
            {
                let words = [(axis, upper), (axis + 4, lower)];
                assert_eq!(hits(&first, &words), in_first, "{axis} {upper} {lower}");
                assert_eq!(hits(&second, &words), in_second, "{axis} {upper} {lower}");
            }
        }
    }

    #[test]
    fn the_long_axis_commands_have_two_block_lengths() {
        for axis in 0..3u32 {
            // The blocks are at +0 (never walked), +1 (the near one, walked in place) and +2
            // (the far one, walked by the recursive call, or in place when the box is below
            // the low byte).
            let head = [0x23 + axis as u8, 10, 5, 0, 2, 0, 1];
            let far_leaf = [&head[..], &[0x00, 0x00, 0x30]].concat();
            let near_leaf = [&head[..], &[0x00, 0x30, 0x00]].concat();
            for (upper, lower, in_far, in_near) in [
                (3, 0, true, false),
                (8, 7, true, true),
                (8, 12, false, true),
                (6, 10, false, true),
            ] {
                let words = [(axis, upper), (axis + 4, lower)];
                assert_eq!(hits(&far_leaf, &words), in_far, "{axis} {upper} {lower}");
                assert_eq!(hits(&near_leaf, &words), in_near, "{axis} {upper} {lower}");
            }
        }
    }

    #[test]
    fn the_sum_commands_compare_a_combination_of_the_centers() {
        // The centers are 11, 20 and 30 and the radius bound 4: the one-sided scale of the
        // two-axis commands is 4 / 2 + 1 + 4 = 7, the one of the three-axis commands 16.
        let words = [(8, 11), (9, 20), (10, 30), (11, 4)];
        // (command, edge, factor of the bytes, scale)
        for (op, edge, factor, scale) in [
            (0x13u8, 50i32, 2i32, 7i32),
            (0x14, 245, 2, 7),
            (0x15, 41, 2, 7),
            (0x16, 236, 2, 7),
            (0x17, 31, 2, 7),
            (0x18, 246, 2, 7),
            (0x19, 61, 3, 16),
            (0x1a, 256, 3, 16),
            (0x1b, 276, 3, 16),
            (0x1c, 471, 3, 16),
        ] {
            let total: i32 = scale + edge;
            // The second block needs factor * b2 < total, the first one also
            // edge < scale + factor * b1.
            let b2_out = (total + factor - 1) / factor;
            let b2_in = (total - 1) / factor;
            let b1_yes = (edge - scale).div_euclid(factor) + 1;
            let b1_no = b1_yes - 1;
            let code = |b1: i32, b2: i32, first: bool| {
                let head = [op, b1 as u8, b2 as u8, 1];
                if first {
                    first_leaf(&head)
                } else {
                    second_leaf(&head)
                }
            };
            assert!(hits(&code(b1_no, b2_out, true), &words), "{op:#x}");
            assert!(!hits(&code(b1_no, b2_out, false), &words), "{op:#x}");
            assert!(hits(&code(b1_yes, b2_in, true), &words), "{op:#x}");
            assert!(hits(&code(b1_yes, b2_in, false), &words), "{op:#x}");
            assert!(!hits(&code(b1_no, b2_in, true), &words), "{op:#x}");
            assert!(hits(&code(b1_no, b2_in, false), &words), "{op:#x}");
        }
    }

    #[test]
    fn the_range_commands_end_the_walk_when_the_box_is_outside() {
        for axis in 0..3u32 {
            let code = [0x26 + axis as u8, 6, 7, 0x30];
            // Needs upper >= 6 and lower < 7.
            assert!(hits(&code, &[(axis, 6), (axis + 4, 6)]));
            assert!(!hits(&code, &[(axis, 5), (axis + 4, 6)]));
            assert!(!hits(&code, &[(axis, 6), (axis + 4, 7)]));
        }
    }

    #[test]
    fn the_wide_range_commands_compare_the_grid_bounds_of_the_machine() {
        for axis in 0..3u32 {
            // 24-bit bounds 256 and 512 against the upper (+0x10) and lower (+0x20) bounds.
            let code = [0x29 + axis as u8, 0, 1, 0, 0, 2, 0, 0x30];
            let (upper, lower) = (0x10 + 4 * axis, 0x20 + 4 * axis);
            let attempt = |u: i32, l: i32| {
                let mut r = rig(&code);
                r.set_this(&[(upper, u), (lower, l)]);
                r.run()
            };
            assert!(!attempt(255, 0));
            assert!(attempt(256, 512));
            assert!(!attempt(256, 513));
            // The compares are signed.
            assert!(!attempt(-1, 0));
            assert!(attempt(300, -5));
        }
    }

    /// The upper and lower bounds `(axis, axis + 4)` the new frame leaves, probed with the
    /// range command: it takes the walk to the leaf when upper >= b1 and lower < b2.
    fn frame_bounds(prefix: &[u8], this: &[(u32, i32)], query: &[(u32, i32)]) -> [(i32, i32); 3] {
        let mut found = [(0, 0); 3];
        let mut r = rig(&[]);
        r.set_this(this);
        r.set_query(query);
        for axis in 0..3u32 {
            let mut reach = |b1: i32, b2: i32| {
                let code = [prefix, &[0x26 + axis as u8, b1 as u8, b2 as u8, 0x30]].concat();
                r.e.mem.write(r.code, &code);
                r.e.mem.set_u8(r.this + 0x40, 0);
                r.run()
            };
            // The largest b1 and the smallest b2 that still let the walk through.
            let upper = (0..=255).rev().find(|&b1| reach(b1, 255)).unwrap();
            let lower = (0..=255).find(|&b2| reach(0, b2)).unwrap() - 1;
            found[axis as usize] = (upper, lower);
        }
        found
    }

    /// The upper bounds (+0x10), lower bounds (+0x20), centers (+0x30) and radius bound (+0x3c)
    /// of the machine for the frame tests.
    const THIS_WORDS: [(u32, i32); 10] = [
        (0x10, 1600),
        (0x14, 2112),
        (0x18, 2624),
        (0x20, 1280),
        (0x24, 1792),
        (0x28, 2304),
        (0x30, 768),
        (0x34, 1024),
        (0x38, 1280),
        (0x3c, 64),
    ];

    #[test]
    fn a_frame_command_rebases_the_box_bounds() {
        // Command 2: the shift 2, the bytes (3, 4, 5) and the offsets (1, 2, 3) of the old
        // frame give (16, 24, 32); the total shift 10 + 2 makes the grid bounds of the machine
        // shift right by 4.
        let this = THIS_WORDS;
        let query = [(12, 1), (13, 2), (14, 3), (16, 10)];
        let found = frame_bounds(&[2, 3, 4, 5], &this, &query);
        // upper = (bound >> 4) + 1 - offset, lower = (bound >> 4) - offset.
        assert_eq!(found, [(85, 64), (109, 88), (133, 112)]);
        // The centers and the radius bound, probed through the sum commands: the centers are
        // 32, 40 and 48 and the radius bound 5 (so the two-axis scale is 2 + 1 + 5 = 8 and the
        // three-axis one 20).
        let sums = [
            (0x13u8, 47, 48),
            (0x15, 43, 44),
            (0x17, 39, 40),
            (0x19, 46, 47),
        ];
        for (op, yes, no) in sums {
            let reach = |b2: u8| {
                let code = [2, 3, 4, 5, op, 0, b2, 1, 0x00, 0x30];
                let mut r = rig(&code);
                r.set_this(&this);
                r.set_query(&query);
                r.run()
            };
            assert!(reach(yes), "{op:#x}");
            assert!(!reach(no), "{op:#x}");
        }
    }

    #[test]
    fn a_second_frame_command_adds_to_the_first_one() {
        let this = [
            (0x10, 800),
            (0x14, 1056),
            (0x18, 1312),
            (0x20, 640),
            (0x24, 896),
            (0x28, 1152),
        ];
        let query = [(12, 1), (13, 2), (14, 3), (16, 10)];
        // The second frame starts at the offsets (16, 24, 32), shifts by one more and the
        // bounds shift right by 3: x = 32, y = 48, z = 64.
        let found = frame_bounds(&[2, 3, 4, 5, 1, 0, 0, 0], &this, &query);
        assert_eq!(
            found,
            [
                ((800 >> 3) + 1 - 32, (640 >> 3) - 32),
                ((1056 >> 3) + 1 - 48, (896 >> 3) - 48),
                ((1312 >> 3) + 1 - 64, (1152 >> 3) - 64),
            ]
        );
    }

    #[test]
    fn every_frame_command_shifts_by_its_number() {
        for op in 1..=4u8 {
            let down = 16 - u32::from(op);
            let mut this = Vec::new();
            for axis in 0..3 {
                this.push((0x10 + 4 * axis, 100 << down));
                this.push((0x20 + 4 * axis, 50 << down));
            }
            let found = frame_bounds(&[op, 1, 1, 1], &this, &[(16, 0)]);
            let offset = 1 << op;
            assert_eq!(found, [(101 - offset, 50 - offset); 3], "{op}");
        }
    }

    #[test]
    fn a_command_byte_nobody_knows_is_reported_and_read_again() {
        unknown_command(0);
    }

    #[test]
    #[should_panic(expected = "00e64494: the exe's INT3 trap")]
    fn an_unknown_command_traps_with_int3_when_the_reporter_asks() {
        unknown_command(1);
    }

    fn unknown_command(wanted: u32) {
        let mut r = rig(&[0xff]);
        let log: Log = Rc::default();
        for target in [ERR_STREAM_CONSTRUCT, OSTREAM_WRITE_TEXT, OSTREAM_DESTRUCT] {
            let sink = Rc::clone(&log);
            r.e.register_double(target, move |_, a| {
                sink.borrow_mut().push((target, a.to_vec()));
                Ret::default()
            });
        }
        let sink = Rc::clone(&log);
        let code = r.code;
        r.e.register_double(REPORT_SLOT, move |e, a| {
            sink.borrow_mut().push((REPORT_SLOT, a.to_vec()));
            // The handler "repairs" the byte so the next pass ends the query.
            e.mem.set_u8(code, 0);
            Ret {
                eax: wanted,
                ..Ret::default()
            }
        });
        let vtable = r.e.mem.alloc(0x20);
        r.e.mem.set_u32(vtable + 0xc, REPORT_SLOT);
        let reporter = r.e.mem.alloc(0x10);
        r.e.mem.set_u32(reporter, vtable);
        r.e.set_global(ERROR_REPORTER, reporter);
        assert!(!r.run());
        let log = log.borrow();
        let stream = log[0].1[0];
        assert_eq!(
            log[0],
            (ERR_STREAM_CONSTRUCT, vec![stream, stream + 0xc, 0x200])
        );
        assert_eq!(log[1], (OSTREAM_WRITE_TEXT, vec![stream, CORRUPTED_TEXT]));
        assert_eq!(
            log[2],
            (
                REPORT_SLOT,
                vec![reporter, 3, 0x1298_fedd, stream + 0xc, ERROR_FILE, 0x156]
            )
        );
        assert_eq!(log[3], (OSTREAM_DESTRUCT, vec![stream]));
        assert_eq!(log.len(), 4);
    }

    fn put(e: &mut Engine, at: u32, v: [f32; 4]) {
        for (i, lane) in v.iter().enumerate() {
            e.mem.set_f32(at + 4 * i as u32, *lane);
        }
    }

    /// Builds the box query of a box (half extents, the three rows and the translation of the
    /// transform, the radius) for a code object with the given scale and origin. Returns the
    /// engine, the machine and the query (every word preset to a marker).
    fn generate(
        half: [f32; 4],
        rows: [[f32; 4]; 3],
        translation: [f32; 4],
        radius: f32,
        scale: f32,
        origin: [f32; 3],
    ) -> (Engine, u32, u32) {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x80);
        let object = e.mem.alloc(0x40);
        e.mem.set_f32(object + 0x1c, scale);
        for (i, lane) in origin.iter().enumerate() {
            e.mem.set_f32(object + 0x10 + 4 * i as u32, *lane);
        }
        e.mem.set_u32(this + 0x44, object);
        let half_at = e.mem.alloc(0x10);
        put(&mut e, half_at, half);
        let transform = e.mem.alloc(0x40);
        for (i, row) in rows.iter().enumerate() {
            put(&mut e, transform + 0x10 * i as u32, *row);
        }
        put(&mut e, transform + 0x30, translation);
        let radius_at = e.mem.alloc(4);
        e.mem.set_f32(radius_at, radius);
        let query = e.mem.alloc(0x48);
        for i in 0..18 {
            e.mem.set_u32(query + 4 * i, 0xdead_beef);
        }
        e.call(
            0x00e645b0,
            &args![this, half_at, transform, radius_at, query],
        );
        (e, this, query)
    }

    const IDENTITY: [[f32; 4]; 3] = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
    ];

    #[test]
    fn the_box_query_holds_the_grid_bounds_and_their_high_halves() {
        // The extents are (1.5, 2.5, 3.5) around (10, 20, 30); with the origin (1, 2, 3) and
        // the scale 2^14 the grid bounds are 16384 * (corner - origin), rounded, less or plus 1.
        let (e, this, query) = generate(
            [1.0, 2.0, 3.0, 0.0],
            IDENTITY,
            [10.0, 20.0, 30.0, 0.0],
            0.5,
            16384.0,
            [1.0, 2.0, 3.0],
        );
        let field = |offset: u32| e.mem.u32(this + offset);
        assert_eq!(field(0x20), 122_879);
        assert_eq!(field(0x10), 172_033);
        assert_eq!(field(0x24), 253_951);
        assert_eq!(field(0x14), 335_873);
        assert_eq!(field(0x28), 385_023);
        assert_eq!(field(0x18), 499_713);
        assert_eq!(field(0x30), 147_456);
        assert_eq!(field(0x34), 294_912);
        assert_eq!(field(0x38), 442_368);
        // (0.5 + sqrt(14)) * 16384 = 69495.3, rounded, plus one.
        assert_eq!(field(0x3c), 69_496);
        let word = |index: u32| e.mem.u32(query + 4 * index);
        let marker = 0xdead_beef;
        let expected = [
            3, 6, 8, marker, 1, 3, 5, marker, 2, 4, 6, 2, 0, 0, 0, 0, 0, 0,
        ];
        for (index, want) in expected.into_iter().enumerate() {
            assert_eq!(word(index as u32), want, "word {index}");
        }
    }

    #[test]
    fn the_extent_follows_the_rows_of_the_transform_by_absolute_value() {
        // Row 0 maps the half extent 1 to the y axis (negated), row 1 the 2 to x, row 2 the
        // 3 to z (negated): the extents are (2, 1, 3) around (1, 0, 0).
        let (e, this, _) = generate(
            [1.0, 2.0, 3.0, 0.0],
            [
                [0.0, -1.0, 0.0, 0.0],
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, -1.0, 0.0],
            ],
            [1.0, 0.0, 0.0, 0.0],
            0.0,
            1.0,
            [0.0, 0.0, 0.0],
        );
        let field = |offset: u32| e.mem.u32(this + offset) as i32;
        assert_eq!((field(0x20), field(0x10)), (-2, 4));
        assert_eq!((field(0x24), field(0x14)), (-2, 2));
        assert_eq!((field(0x28), field(0x18)), (-4, 4));
        assert_eq!((field(0x30), field(0x34), field(0x38)), (1, 0, 0));
        // sqrt(14) rounds to 4.
        assert_eq!(field(0x3c), 5);
    }

    #[test]
    fn the_grid_conversion_rounds_ties_to_even() {
        for (translation, grid) in [
            ([0.5, 1.5, 2.5, 0.0], [0, 2, 2]),
            ([-0.5, -1.5, -2.5, 0.0], [0, -2, -2]),
        ] {
            let (e, this, _) = generate([0.0; 4], [[0.0; 4]; 3], translation, 0.0, 1.0, [0.0; 3]);
            let field = |offset: u32| e.mem.u32(this + offset) as i32;
            assert_eq!(field(0x20), grid[0] - 1);
            assert_eq!(field(0x10), grid[0] + 1);
            assert_eq!(field(0x24), grid[1] - 1);
            assert_eq!(field(0x14), grid[1] + 1);
            assert_eq!(field(0x28), grid[2] - 1);
            assert_eq!(field(0x18), grid[2] + 1);
            assert_eq!(field(0x3c), 1);
        }
    }

    /// The setup of a call of `queryObb`: a box of size 0 at the origin, a code object of scale
    /// 1 whose code is `code`, and the result byte preset to 0xaa.
    struct Obb {
        e: Engine,
        this: u32,
        result: u32,
        mopp: u32,
        inputs: [u32; 3],
    }

    fn obb(code: &[u8]) -> Obb {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        let this = e.mem.alloc(0x80);
        e.mem.set_u8(this + 0x40, 1);
        let result = e.mem.alloc(4);
        e.mem.set_u8(result, 0xaa);
        let mopp = e.mem.alloc(0x40);
        e.mem.set_f32(mopp + 0x1c, 1.0);
        let at = e.mem.alloc(0x100);
        e.mem.write(at, code);
        e.mem.set_u32(mopp + 0x20, at);
        let transform = e.mem.alloc(0x40);
        let half = e.mem.alloc(0x10);
        let radius = e.mem.alloc(4);
        Obb {
            e,
            this,
            result,
            mopp,
            inputs: [transform, half, radius],
        }
    }

    impl Obb {
        fn query_obb(&mut self) {
            let (this, result, mopp) = (self.this, self.result, self.mopp);
            let [transform, half, radius] = self.inputs;
            self.e.call(
                0x00e647e0,
                &args![this, result, mopp, transform, half, radius],
            );
        }
    }

    #[test]
    fn the_box_test_does_nothing_until_the_product_check_passes() {
        let mut o = obb(&[0x30]);
        o.e.mem.set_u8(LICENSE_FLAG, 0);
        let asked = Rc::new(Cell::new(0u32));
        let seen = Rc::clone(&asked);
        o.e.register_double(LICENSE_CHECK, move |_, a| {
            seen.set(a[0]);
            Ret::default()
        });
        o.query_obb();
        assert_eq!(asked.get(), LICENSE_FLAG);
        assert_eq!(o.e.mem.u8(o.result), 0);
        assert_eq!(o.e.mem.u32(o.this + 0x44), 0);
        // A check that sets the flag lets the test go on.
        let mut o = obb(&[0x30]);
        o.e.mem.set_u8(LICENSE_FLAG, 0);
        o.e.register_double(LICENSE_CHECK, |e, a| {
            e.mem.set_u8(a[0], 1);
            Ret::default()
        });
        o.query_obb();
        assert_eq!(o.e.mem.u8(o.result), 1);
        assert_eq!(o.e.mem.u32(o.this + 0x44), o.mopp);
    }

    #[test]
    fn the_box_test_clears_the_hit_and_reports_what_the_walk_found() {
        let mut o = obb(&[0x00, 0x30]);
        o.e.mem.set_u8(LICENSE_FLAG, 1);
        o.query_obb();
        assert_eq!(o.e.mem.u8(o.result), 0);
        assert_eq!(o.e.mem.u8(o.this + 0x40), 0);
        let mut o = obb(&[0x30]);
        o.e.mem.set_u8(LICENSE_FLAG, 1);
        o.query_obb();
        assert_eq!(o.e.mem.u8(o.result), 1);
    }

    #[test]
    fn the_walk_runs_on_the_query_of_the_box() {
        // The box is a point at the origin: the grid bounds are -1 and 1, so the query has the
        // upper word 1 and the lower word -1 on every axis.
        for (code, expected) in [
            ([0x26, 1, 1, 0x30], 1),
            ([0x26, 2, 1, 0x30], 0),
            ([0x27, 1, 1, 0x30], 1),
            ([0x28, 2, 1, 0x30], 0),
        ] {
            let mut o = obb(&code);
            o.e.mem.set_u8(LICENSE_FLAG, 1);
            o.query_obb();
            assert_eq!(o.e.mem.u8(o.result), expected, "{code:?}");
        }
    }
}

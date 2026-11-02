//! `Havok SDK/hkpmoppmodifyvirtualmachine.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkpMoppModifyVirtualMachine` (field names describe what the code does and are
//! unconfirmed): the box as integers at +0x10/+0x14/+0x18 (upper, one added) and
//! +0x20/+0x24/+0x28 (lower, one taken off); the MOPP code object at +0x30 (its pointer at
//! +0x20 is the code base); the byte result of the last report at +0x40; the object that
//! gets the reports at +0x44 (virtual function at +4 reports a hit, at +8 a range of the
//! code) and a mask at +0x48.

#[allow(unused_imports)]
use crate::prelude::*;

/// A byte in the exe image (`0126efbc`): non-zero once the product key check has passed.
const LICENSE_FLAG: u32 = 0x0126_efbc;
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
/// and the source file string `.\Collide\Mopp\Machine\hkpMoppModifyVirtualMachine.cpp`
/// (`010ed0ac`).
const CORRUPTED_TEXT: u32 = 0x010d_5100;
const ERROR_FILE: u32 = 0x010e_d0ac;

/// `FISTP` of a single-precision value with the default rounding mode (nearest, ties to even);
/// the integer indefinite `0x80000000` for NaN and out-of-range values.
fn fistp(value: f32) -> i32 {
    let v = f64::from(value);
    let mut r = v.round();
    if (v - v.trunc()).abs() == 0.5 && r % 2.0 != 0.0 {
        r -= v.signum();
    }
    if r.is_nan() || r < f64::from(i32::MIN) || r > f64::from(i32::MAX) {
        i32::MIN
    } else {
        r as i32
    }
}

fn byte(e: &Engine, at: u32) -> i32 {
    i32::from(e.mem.u8(at))
}

fn int(e: &Engine, at: u32) -> i32 {
    e.mem.u32(at) as i32
}

/// A big-endian value of `count` bytes starting at `at`.
fn big_endian(e: &Engine, at: u32, count: u32) -> i32 {
    (0..count).fold(0i32, |acc, i| (acc << 8).wrapping_add(byte(e, at + i)))
}

/// Copies the 0x40-byte record at `from` over the one at `to` (`REP MOVSD`, sixteen words).
fn copy_record(e: &mut Engine, from: u32, to: u32) {
    for i in 0..16 {
        let word = e.mem.u32(from + 4 * i);
        e.mem.set_u32(to + 4 * i, word);
    }
}

/// The code base: the pointer at +0x20 of the code object at `this + 0x30`.
fn code_base(e: &Engine, this: u32) -> u32 {
    e.mem.u32(e.mem.u32(this + 0x30) + 0x20)
}

/// Queries the code at `code` and returns the result byte (the exe passes a byte of its
/// frame as the result pointer and reads it back).
fn query_at(
    e: &mut Engine,
    this: u32,
    scratch: u32,
    state: u32,
    code: u32,
    jump_offset: i32,
) -> u8 {
    hkp_mopp_modify_virtual_machine_query_modication_points_recursive(
        e,
        Ptr::new(this),
        Ptr::new(scratch),
        Ptr::new(state),
        Ptr::new(code),
        jump_offset,
    );
    e.mem.u8(scratch)
}

/// The end of the plane commands 0x10 to 0x25 with the two sides of the test `(a, b, c, d)`:
/// when `a <= b` and `c >= d` neither half is hit; with `a <= b` only the first half
/// (at `code + near`) is queried, with `a > b` and `c >= d` only the second (at
/// `code + second`) and with `a > b` and `c < d` both. A half that was hit while the other
/// was not or not queried is reported to the collector (virtual function at +8) as an
/// offset into the code. The result is non-zero when both halves were hit.
fn query_halves(
    e: &mut Engine,
    this: u32,
    scratch: u32,
    state: u32,
    code: u32,
    jump_offset: i32,
    (a, b, c, d): (i32, i32, i32, i32),
    (near, second): (u32, u32),
) -> u8 {
    let mut first_hit = 0u8;
    let mut second_hit = 0u8;
    if a <= b {
        if c >= d {
            return 0;
        }
        first_hit = query_at(
            e,
            this,
            scratch,
            state,
            code.wrapping_add(near),
            jump_offset,
        );
    } else if c >= d {
        second_hit = query_at(
            e,
            this,
            scratch,
            state,
            code.wrapping_add(second),
            jump_offset,
        );
    } else {
        first_hit = query_at(
            e,
            this,
            scratch,
            state,
            code.wrapping_add(near),
            jump_offset,
        );
        second_hit = query_at(
            e,
            this,
            scratch,
            state,
            code.wrapping_add(second),
            jump_offset,
        );
    }
    let collector = e.mem.u32(this + 0x44);
    if first_hit != 0 && second_hit == 0 {
        let offset = near.wrapping_sub(code_base(e, this)).wrapping_add(code);
        e.vcall(collector, 8, &args![offset]);
    }
    if second_hit != 0 && first_hit == 0 {
        let offset = second.wrapping_sub(code_base(e, this)).wrapping_add(code);
        e.vcall(collector, 8, &args![offset]);
    }
    u8::from(first_hit != 0 && second_hit != 0)
}

// Translated from 00e66950 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMoppModifyVirtualMachine::queryModicationPointsRecursive` (Xbox PDB; the name is
/// spelled so in the source), thiscall `(this, out, state, code, jump_offset)`, `RET 0x10`:
/// interprets the command bytes of a MOPP tree at `code` and stores a result byte in `out`
/// (which it returns): non-zero when a hit was reported or both halves of a plane were hit.
///
/// `state` is a record of sixteen words: `+0/+4/+8` and `+0x10/+0x14/+0x18` the upper and
/// lower bounds of the box relative to the current cell (words +0xC and +0x1C are not used),
/// `+0x20/+0x24/+0x28` the cell origin, `+0x2C` an id offset, `+0x30` the number of bits the
/// cell was narrowed by, `+0x34` a word the hit report gets a pointer to and `+0x38/+0x3C`
/// two more nothing reads. The interpreter keeps a local record of its own; a command that
/// changes the state copies the record there first (and recursive calls get a pointer to
/// either record). `jump_offset` is the code offset of the last jump (commands 0xC and 0x70),
/// which goes into the high bits of each reported hit. Commands (the byte tables `00e671b0`
/// and `00e67124`): 0 stores the byte at `this + 0x40` and returns; 1 to 4 build the local
/// record narrowed by that many bits from the next three bytes; 5 to 7 skip forward by a
/// 1, 2 or 3 byte offset and 0xD by five bytes; 9 to 0xB add to or set the id offset; 0xC
/// and 0x70 jump to a code offset (9 bits left of a 16-bit value, or a 32-bit value);
/// 0x10 to 0x25 are plane commands (see `query_halves`); 0x26 to 0x2B return 0 when the box
/// lies outside a range; 0x30 to 0x53 report a hit (the id plus the id offset, or-ed with
/// the masked jump offset, through the collector's virtual function at +4, which returns a
/// pointer to the result byte) and return it; 0x60 to 0x6B store a value in a slot of the
/// frame beside the local record and make the local record the current one; any other
/// command reports the "Unknown command" error (`hkErrStream`, `00cb1600`, the reporter's
/// virtual function at 0xC, then `INT3`, which this translation panics on: it is a
/// breakpoint trap in the exe, not a call) and continues with the same byte. The command
/// meanings beyond this are unconfirmed.
///
/// Uninitialised stack words (checked against the disassembly): the local record is built
/// whole by command 1 to 4 or copied whole before any read of it, except for the words
/// nothing reads. Commands 0x60 to 0x6B write slots of the frame; slot 0 is the record's
/// word +0x34, which the hit report gets a pointer to. A store to another slot as the first
/// command that makes the local record the current one keeps the local record's never
/// written word +0x34 over the copy; the exe reads stack garbage there, it reads as zero
/// here.
// Translation incomplete: 00e66950 - a slot store other than slot 0 as the first record-copying command leaves the local record's word +0x34 (passed to the hit report by pointer) as stack garbage in the exe; zero here.
pub fn hkp_mopp_modify_virtual_machine_query_modication_points_recursive(
    e: &mut Engine,
    this: Ptr,
    out: Ptr,
    state: Ptr,
    code: Ptr,
    jump_offset: i32,
) -> Ptr {
    let t = this.addr();
    e.with_stack(0x260, |e, frame| {
        // The frame: the local record at +0, the slots of commands 0x60 to 0x6B from +0x34
        // (the fourth at +0x40), the error stream at +0x44, its 0x200-byte buffer behind it
        // and a result byte at +0x250.
        let local = frame.addr();
        let scratch = local + 0x250;
        let mut s = state.addr();
        let mut c = code.addr();
        let mut offset = jump_offset;
        loop {
            let op = e.mem.u8(c) as u32;
            match op {
                0 => {
                    let last = e.mem.u8(t + 0x40);
                    e.mem.set_u8(out.addr(), last);
                    return;
                }
                1..=4 => {
                    let words = |e: &Engine, first: u32| [0, 4, 8].map(|i| int(e, s + first + i));
                    let cell = words(e, 0x20);
                    let id_offset = e.mem.u32(s + 0x2c);
                    let total = int(e, s + 0x30).wrapping_add(op as i32);
                    let extra = e.mem.u32(s + 0x34);
                    let count = (0x10u32.wrapping_sub(total as u32)) & 31;
                    for axis in 0..3u32 {
                        let origin = byte(e, c + 1 + axis)
                            .wrapping_add(cell[axis as usize])
                            .wrapping_shl(op);
                        e.mem.set_u32(local + 0x20 + 4 * axis, origin as u32);
                        let lower = (int(e, t + 0x20 + 4 * axis) >> count).wrapping_sub(origin);
                        e.mem.set_u32(local + 0x10 + 4 * axis, lower as u32);
                        let upper = (int(e, t + 0x10 + 4 * axis) >> count)
                            .wrapping_sub(origin)
                            .wrapping_add(1);
                        e.mem.set_u32(local + 4 * axis, upper as u32);
                    }
                    e.mem.set_u32(local + 0x30, total as u32);
                    e.mem.set_u32(local + 0x34, extra);
                    e.mem.set_u32(local + 0x2c, id_offset);
                    s = local;
                    c += 4;
                }
                5 => c = c.wrapping_add(byte(e, c + 1) as u32 + 2),
                6 => c = c.wrapping_add(big_endian(e, c + 1, 2) as u32 + 3),
                7 => c = c.wrapping_add(big_endian(e, c + 1, 3) as u32 + 4),
                9..=11 => {
                    let (amount, length) = match op {
                        9 => (byte(e, c + 1), 2),
                        10 => (big_endian(e, c + 1, 2), 3),
                        _ => (big_endian(e, c + 1, 4), 5),
                    };
                    if s != local {
                        copy_record(e, s, local);
                        s = local;
                    }
                    let id_offset = if op == 11 {
                        amount
                    } else {
                        int(e, local + 0x2c).wrapping_add(amount)
                    };
                    e.mem.set_u32(local + 0x2c, id_offset as u32);
                    c += length;
                }
                0xd => c += 5,
                0xc | 0x70 => {
                    offset = if op == 0xc {
                        big_endian(e, c + 1, 2).wrapping_shl(9)
                    } else {
                        big_endian(e, c + 1, 4)
                    };
                    c = code_base(e, t).wrapping_add(offset as u32);
                }
                0x10..=0x1c | 0x20..=0x25 => {
                    let r = |e: &Engine, word: u32| int(e, s + 4 * word);
                    let (r0, r1, r2) = (r(e, 0), r(e, 1), r(e, 2));
                    let (r4, r5, r6) = (r(e, 4), r(e, 5), r(e, 6));
                    let (b1, b2, b3) = (byte(e, c + 1), byte(e, c + 2), byte(e, c + 3));
                    let (test, near, second, length) = match op {
                        0x10..=0x12 => {
                            let k = op - 0x10;
                            ((r(e, k), b2, r(e, 4 + k), b1), 0, b3 as u32, 4)
                        }
                        0x13 => (
                            (r2.wrapping_add(r1), 2 * b2, r6.wrapping_add(r5), 2 * b1),
                            0,
                            b3 as u32,
                            4,
                        ),
                        0x14 => (
                            (
                                r1.wrapping_sub(r6),
                                (2 * b2).wrapping_sub(255),
                                r5.wrapping_sub(r2),
                                (2 * b1).wrapping_sub(255),
                            ),
                            0,
                            b3 as u32,
                            4,
                        ),
                        0x15 => (
                            (r2.wrapping_add(r0), 2 * b2, r6.wrapping_add(r4), 2 * b1),
                            0,
                            b3 as u32,
                            4,
                        ),
                        0x16 => (
                            (
                                r0.wrapping_sub(r6),
                                (2 * b2).wrapping_sub(255),
                                r4.wrapping_sub(r2),
                                (2 * b1).wrapping_sub(255),
                            ),
                            0,
                            b3 as u32,
                            4,
                        ),
                        0x17 => (
                            (r1.wrapping_add(r0), 2 * b2, r5.wrapping_add(r4), 2 * b1),
                            0,
                            b3 as u32,
                            4,
                        ),
                        0x18 => (
                            (
                                r0.wrapping_sub(r5),
                                (2 * b2).wrapping_sub(255),
                                r4.wrapping_sub(r1),
                                (2 * b1).wrapping_sub(255),
                            ),
                            0,
                            b3 as u32,
                            4,
                        ),
                        0x19 => (
                            (
                                r2.wrapping_add(r1).wrapping_add(r0),
                                3 * b2,
                                r6.wrapping_add(r5).wrapping_add(r4),
                                3 * b1,
                            ),
                            0,
                            b3 as u32,
                            4,
                        ),
                        0x1a => (
                            (
                                r1.wrapping_sub(r6).wrapping_add(r0),
                                (b2 - 0x55) * 3,
                                r5.wrapping_sub(r2).wrapping_add(r4),
                                (b1 - 0x55) * 3,
                            ),
                            0,
                            b3 as u32,
                            4,
                        ),
                        0x1b => (
                            (
                                r2.wrapping_sub(r5).wrapping_add(r0),
                                (b2 - 0x55) * 3,
                                r6.wrapping_sub(r1).wrapping_add(r4),
                                (b1 - 0x55) * 3,
                            ),
                            0,
                            b3 as u32,
                            4,
                        ),
                        0x1c => (
                            (
                                r0.wrapping_sub(r6).wrapping_sub(r5),
                                (b2 - 0xaa) * 3,
                                r4.wrapping_sub(r2).wrapping_sub(r1),
                                (b1 - 0xaa) * 3,
                            ),
                            0,
                            b3 as u32,
                            4,
                        ),
                        0x20..=0x22 => {
                            let k = op - 0x20;
                            ((r(e, k), b1, r(e, 4 + k), b1 + 1), 0, b2 as u32, 3)
                        }
                        _ => {
                            let k = op - 0x23;
                            (
                                (r(e, k), b2, r(e, 4 + k), b1),
                                big_endian(e, c + 3, 2) as u32,
                                big_endian(e, c + 5, 2) as u32,
                                7,
                            )
                        }
                    };
                    let result =
                        query_halves(e, t, scratch, s, c + length, offset, test, (near, second));
                    e.mem.set_u8(out.addr(), result);
                    return;
                }
                0x26..=0x28 => {
                    let k = op - 0x26;
                    if int(e, s + 4 * k) < byte(e, c + 1)
                        || byte(e, c + 2) <= int(e, s + 0x10 + 4 * k)
                    {
                        e.mem.set_u8(out.addr(), 0);
                        return;
                    }
                    c += 3;
                }
                0x29..=0x2b => {
                    let k = op - 0x29;
                    if int(e, t + 0x10 + 4 * k) < big_endian(e, c + 1, 3)
                        || big_endian(e, c + 4, 3) < int(e, t + 0x20 + 4 * k)
                    {
                        e.mem.set_u8(out.addr(), 0);
                        return;
                    }
                    c += 7;
                }
                0x30..=0x53 => {
                    let id = match op {
                        0x30..=0x4f => op as i32 - 0x30,
                        0x50 => byte(e, c + 1),
                        0x51 => big_endian(e, c + 1, 2),
                        0x52 => big_endian(e, c + 1, 3),
                        _ => big_endian(e, c + 1, 4),
                    };
                    let masked = ((offset >> 9) << 8) as u32 & e.mem.u32(t + 0x48);
                    let value = masked | int(e, s + 0x2c).wrapping_add(id) as u32;
                    let collector = e.mem.u32(t + 0x44);
                    let result = e
                        .vcall(collector, 4, &args![scratch, value, s + 0x34])
                        .u32();
                    let hit = e.mem.u8(result);
                    e.mem.set_u8(t + 0x40, hit);
                    e.mem.set_u8(out.addr(), hit);
                    return;
                }
                0x60..=0x6b => {
                    let (value, length) = match op {
                        0x60..=0x63 => (byte(e, c + 1), 2),
                        0x64..=0x67 => (big_endian(e, c + 1, 2), 3),
                        _ => (big_endian(e, c + 1, 4), 5),
                    };
                    e.mem.set_u32(local + 0x34 + 4 * (op & 3), value as u32);
                    c += length;
                    // The exe keeps the record's word +0x34 in a register across the copy.
                    let kept = e.mem.u32(local + 0x34);
                    if s != local {
                        copy_record(e, s, local);
                        s = local;
                    }
                    e.mem.set_u32(local + 0x34, kept);
                }
                _ => {
                    let stream = local + 0x44;
                    let buffer = stream + 0xc;
                    e.call(ERR_STREAM_CONSTRUCT, &args![stream, buffer, 0x200u32]);
                    e.call(OSTREAM_WRITE_TEXT, &args![stream, CORRUPTED_TEXT]);
                    let reporter: u32 = e.global(ERROR_REPORTER);
                    let wanted_break = e
                        .vcall(
                            reporter,
                            0xc,
                            &args![3u32, 0x1298_feddu32, buffer, ERROR_FILE, 0x125u32],
                        )
                        .u32();
                    if wanted_break != 0 {
                        panic!(
                            "00e66950: the exe's INT3 trap (the error reporter asked for a break)"
                        );
                    }
                    e.call(OSTREAM_DESTRUCT, &args![stream]);
                }
            }
        }
    });
    out
}

// Translated from 00e67230 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMoppModifyVirtualMachine::queryAabb` (Xbox PDB), thiscall `(this, mopp, aabb,
/// collector)`, `RET 0xC`: does nothing until the product key check (`00df22f0`) has set the
/// byte at `0126efbc`. Stores the code object `mopp` at +0x30 and the `collector` at +0x44,
/// clears the word at +0, turns the box (six floats at `aabb`: the lower corner, then the
/// upper one at +0x10) into integers relative to the code object's origin (floats at +0x10)
/// and scale (+0x1C): `FISTP` of `(coordinate - origin) * scale`, one less for the lower
/// corner and one more for the upper one, stored at +0x20/+0x24/+0x28 and +0x10/+0x14/+0x18.
/// It builds the starting record from them (the high halves of the stored words, the
/// upper ones plus one) with the mask at +0x48 set to all ones when the first code byte is
/// 0xD (else 0), queries the code (`queryModicationPointsRecursive`) and, when that hit
/// something, tells the collector (virtual function at +8) 5 for a first byte of 0xD and 0
/// for another.
pub fn hkp_mopp_modify_virtual_machine_query_aabb(
    e: &mut Engine,
    this: Ptr,
    mopp: Ptr,
    aabb: Ptr,
    collector: Ptr,
) {
    if e.mem.u8(LICENSE_FLAG) == 0 {
        e.call(LICENSE_CHECK, &args![LICENSE_FLAG]);
        if e.mem.u8(LICENSE_FLAG) == 0 {
            return;
        }
    }
    let t = this.addr();
    let (m, b) = (mopp.addr(), aabb.addr());
    e.mem.set_u32(t + 0x30, m);
    e.mem.set_u32(t + 0x44, collector.addr());
    e.mem.set_u32(t, 0);
    let scale = f64::from(x87_quiet(e.mem.f32(m + 0x1c)));
    let converted = |e: &Engine, at: u32, origin_at: u32| {
        let origin = f64::from(x87_quiet(e.mem.f32(origin_at)));
        let value = f64::from(x87_quiet(e.mem.f32(at)));
        fistp(((value - origin) * scale) as f32)
    };
    let lower_x = converted(e, b, m + 0x10).wrapping_sub(1);
    e.mem.set_u32(t + 0x20, lower_x as u32);
    let upper_x = converted(e, b + 0x10, m + 0x10).wrapping_add(1);
    e.mem.set_u32(t + 0x10, upper_x as u32);
    let lower_y = converted(e, b + 4, m + 0x14).wrapping_sub(1);
    e.mem.set_u32(t + 0x24, lower_y as u32);
    let upper_y = converted(e, b + 0x14, m + 0x14).wrapping_add(1);
    e.mem.set_u32(t + 0x14, upper_y as u32);
    let lower_z = converted(e, b + 8, m + 0x18).wrapping_sub(1);
    e.mem.set_u32(t + 0x28, lower_z as u32);
    let upper_z = converted(e, b + 0x18, m + 0x18).wrapping_add(1);
    e.mem.set_u32(t + 0x18, upper_z as u32);
    e.with_stack(0x50, |e, frame| {
        // The record is at +0 and the result byte at +0x40.
        let record = frame.addr();
        let high_half = |e: &Engine, at: u32| i32::from(e.mem.u16(at) as i16);
        let words = [
            (0, high_half(e, t + 0x12).wrapping_add(1)),
            (4, high_half(e, t + 0x16).wrapping_add(1)),
            (8, (upper_z >> 16).wrapping_add(1)),
            (0x10, high_half(e, t + 0x22)),
            (0x14, high_half(e, t + 0x26)),
            (0x18, high_half(e, t + 0x2a)),
        ];
        for (offset, word) in words {
            e.mem.set_u32(record + offset, word as u32);
        }
        for offset in [0x20, 0x24, 0x28, 0x2c, 0x30, 0x34] {
            e.mem.set_u32(record + offset, 0);
        }
        let code = e.mem.u32(m + 0x20);
        let mask = if e.mem.u8(code) != 0xd {
            0
        } else {
            0xffff_ffff
        };
        e.mem.set_u32(t + 0x48, mask);
        let result = query_at(e, t, record + 0x40, record, code, 0);
        if result != 0 {
            let code = e.mem.u32(e.mem.u32(t + 0x30) + 0x20);
            let kind = if e.mem.u8(code) != 0xd { 0u32 } else { 5 };
            e.vcall(collector.addr(), 8, &args![kind]);
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00e6_6950,
            hkp_mopp_modify_virtual_machine_query_modication_points_recursive(Ptr, Ptr, Ptr, Ptr, i32) -> Ptr
        ),
        entry!(
            0x00e6_7230,
            hkp_mopp_modify_virtual_machine_query_aabb(Ptr, Ptr, Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const HIT_DOUBLE: u32 = 0x0900_0010;
    const RANGE_DOUBLE: u32 = 0x0900_0020;

    /// One hit report: the value, the pointer to the extra word and the sixteen record words
    /// around it (the record starts 0x34 bytes before the pointer).
    type Hit = (u32, u32, Vec<u32>);

    struct Rig {
        e: Engine,
        hits: Rc<RefCell<Vec<Hit>>>,
        ranges: Rc<RefCell<Vec<u32>>>,
        this: u32,
        state: u32,
        code: u32,
        out: u32,
    }

    /// A machine whose collector answers a hit with 1 for an odd value and 0 for an even one
    /// and logs its reports, a state record of zeros, the key check passed and every
    /// mask bit set.
    fn rig() -> Rig {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.mem.set_u8(LICENSE_FLAG, 1);
        let hits: Rc<RefCell<Vec<Hit>>> = Rc::new(RefCell::new(Vec::new()));
        let ranges = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&hits);
        e.register_double(HIT_DOUBLE, move |e, a| {
            let record = (0..16).map(|i| e.mem.u32(a[3] - 0x34 + 4 * i)).collect();
            sink.borrow_mut().push((a[2], a[3], record));
            e.mem.set_u8(a[1], u8::from(a[2] & 1 != 0));
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        let sink = Rc::clone(&ranges);
        e.register_double(RANGE_DOUBLE, move |_, a| {
            sink.borrow_mut().push(a[1]);
            Ret::default()
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 4, HIT_DOUBLE);
        e.mem.set_u32(vtable + 8, RANGE_DOUBLE);
        let collector = e.mem.alloc(0x10);
        e.mem.set_u32(collector, vtable);
        let code = e.mem.alloc(0x1000);
        let mopp = e.mem.alloc(0x40);
        e.mem.set_u32(mopp + 0x20, code);
        let this = e.mem.alloc(0x80);
        e.mem.set_u32(this + 0x30, mopp);
        e.mem.set_u32(this + 0x44, collector);
        e.mem.set_u32(this + 0x48, 0xffff_ffff);
        let state = e.mem.alloc(0x40);
        let out = e.mem.alloc(4);
        Rig {
            e,
            hits,
            ranges,
            this,
            state,
            code,
            out,
        }
    }

    /// A record without the words nothing writes (+0xC, +0x1C, +0x38 and +0x3C).
    fn defined_words(record: &[u32]) -> Vec<u32> {
        (0..16)
            .filter(|i| ![3, 7, 14, 15].contains(i))
            .map(|i| record[i])
            .collect()
    }

    impl Rig {
        fn set_state(&mut self, words: &[(u32, u32)]) {
            for &(index, word) in words {
                self.e.mem.set_u32(self.state + 4 * index, word);
            }
        }

        /// Runs `bytes` as the command stream and returns the result byte.
        fn run_with(&mut self, bytes: &[u8], jump_offset: i32) -> u8 {
            self.e.mem.write(self.code, bytes);
            self.hits.borrow_mut().clear();
            self.ranges.borrow_mut().clear();
            self.e.mem.set_u8(self.out, 0xee);
            let (t, o, s, c) = (self.this, self.out, self.state, self.code);
            let returned = self.e.call(0x00e6_6950, &args![t, o, s, c, jump_offset]);
            assert_eq!(returned.u32(), o);
            self.e.mem.u8(self.out)
        }

        fn run(&mut self, bytes: &[u8]) -> u8 {
            self.run_with(bytes, 0)
        }

        fn hit_values(&self) -> Vec<u32> {
            self.hits.borrow().iter().map(|hit| hit.0).collect()
        }
    }

    #[test]
    fn a_hit_command_reports_the_id_with_the_offset_and_returns_the_answer() {
        let mut r = rig();
        r.set_state(&[(11, 100)]);
        // 0x31 is id 1: the collector answers 1 for the odd value 101.
        assert_eq!(r.run(&[0x31]), 1);
        assert_eq!(r.hit_values(), [101]);
        assert_eq!(r.e.mem.u8(r.this + 0x40), 1);
        assert_eq!(r.hits.borrow()[0].1, r.state + 0x34);
        for (code, value) in [
            (&[0x4fu8][..], 131),
            (&[0x50, 0xfe], 354),
            (&[0x51, 0x01, 0x02], 100 + 0x102),
            (&[0x52, 0x01, 0x02, 0x03], 100 + 0x01_0203),
            (&[0x53, 0x01, 0x02, 0x03, 0x04], 100 + 0x0102_0304),
        ] {
            r.run(code);
            assert_eq!(r.hit_values(), [value]);
        }
        // An even value is answered with 0.
        assert_eq!(r.run(&[0x30]), 0);
        assert_eq!(r.e.mem.u8(r.this + 0x40), 0);
    }

    #[test]
    fn the_jump_offset_and_the_mask_go_into_the_high_bits() {
        let mut r = rig();
        r.set_state(&[(11, 10)]);
        // (0x600 >> 9) << 8 = 0x300.
        r.run_with(&[0x31], 0x600);
        assert_eq!(r.hit_values(), [0x300 | 11]);
        r.e.mem.set_u32(r.this + 0x48, 0xff);
        r.run_with(&[0x31], 0x600);
        assert_eq!(r.hit_values(), [11]);
    }

    #[test]
    fn command_zero_returns_the_last_answer() {
        let mut r = rig();
        r.e.mem.set_u8(r.this + 0x40, 7);
        assert_eq!(r.run(&[0x00, 0x31]), 7);
        assert!(r.hit_values().is_empty());
    }

    #[test]
    fn skip_commands_move_the_pointer() {
        let mut r = rig();
        r.run(&[0x05, 0x01, 0xaa, 0x32]);
        assert_eq!(r.hit_values(), [2]);
        r.run(&[0x06, 0x00, 0x02, 0xaa, 0xaa, 0x33]);
        assert_eq!(r.hit_values(), [3]);
        r.run(&[0x07, 0x00, 0x00, 0x01, 0xaa, 0x34]);
        assert_eq!(r.hit_values(), [4]);
        r.run(&[0x0d, 0, 0, 0, 0, 0x35]);
        assert_eq!(r.hit_values(), [5]);
    }

    #[test]
    fn id_offset_commands_work_on_a_copy_of_the_state() {
        let mut r = rig();
        r.set_state(&[(11, 10)]);
        r.run(&[0x09, 0x05, 0x30]);
        assert_eq!(r.hit_values(), [15]);
        r.run(&[0x0a, 0x01, 0x00, 0x30]);
        assert_eq!(r.hit_values(), [266]);
        r.run(&[0x0b, 0x00, 0x01, 0x00, 0x00, 0x30]);
        assert_eq!(r.hit_values(), [0x1_0000]);
        r.run(&[0x09, 0x05, 0x09, 0x07, 0x30]);
        assert_eq!(r.hit_values(), [22]);
        assert_eq!(r.e.mem.u32(r.state + 0x2c), 10);
    }

    #[test]
    fn jump_commands_move_to_an_offset_in_the_code() {
        let mut r = rig();
        r.e.mem.write(r.code + 0x200, &[0x30]);
        // 0x0c: 1 << 9 = 0x200 and the offset goes into the hit value.
        r.run(&[0x0c, 0x00, 0x01]);
        assert_eq!(r.hit_values(), [0x100]);
        r.e.mem.write(r.code + 0x400, &[0x31]);
        r.run(&[0x70, 0x00, 0x00, 0x04, 0x00]);
        assert_eq!(r.hit_values(), [((0x400 >> 9) << 8) | 1]);
    }

    #[test]
    fn narrowing_builds_the_local_record() {
        // Bits narrowed 14 + 2 = 16, so the shift is 0. Origins (3+5)<<2 = 32, (4+6)<<2 = 40,
        // (5+7)<<2 = 48; lower words 60-32, 70-40, 80-48; upper words 100-32+1, 200-40+1,
        // 300-48+1.
        let mut r = rig();
        r.set_state(&[(8, 5), (9, 6), (10, 7), (11, 0x40), (12, 14), (13, 0x77)]);
        for (i, (upper, lower)) in [(100u32, 60u32), (200, 70), (300, 80)]
            .into_iter()
            .enumerate()
        {
            r.e.mem.set_u32(r.this + 0x10 + 4 * i as u32, upper);
            r.e.mem.set_u32(r.this + 0x20 + 4 * i as u32, lower);
        }
        r.run(&[0x02, 3, 4, 5, 0x31]);
        let hits = r.hits.borrow();
        assert_eq!(hits[0].0, 0x41);
        assert_eq!(
            defined_words(&hits[0].2),
            [69, 161, 253, 28, 30, 32, 32, 40, 48, 0x40, 16, 0x77]
        );
    }

    #[test]
    fn narrowing_shifts_the_box_arithmetically() {
        // Bits narrowed 11 + 1 = 12, so the shift is 4: -64 >> 4 = -4; origin (1+2)<<1 = 6.
        let mut r = rig();
        r.set_state(&[(8, 2), (12, 11)]);
        r.e.mem.set_u32(r.this + 0x10, 80);
        r.e.mem.set_u32(r.this + 0x20, (-64i32) as u32);
        r.run(&[0x01, 1, 0, 0, 0x31]);
        let hits = r.hits.borrow();
        let words = &hits[0].2;
        assert_eq!(words[4] as i32, -4 - 6);
        assert_eq!(words[0] as i32, (80 >> 4) - 6 + 1);
        assert_eq!(words[8], 6);
        assert_eq!(words[12], 12);
    }

    /// The two bytes (upper test, lower test) that give each side of the plane test, for the
    /// state words R0..R2 = (90, 100, 110) and R4..R6 = (120, 135, 140): (command,
    /// `b2` with a <= b, `b2` with a > b, `b1` with c < d, `b1` with c >= d).
    const PLANE_BYTES: [(u8, u8, u8, u8, u8); 10] = [
        (0x13, 105, 104, 138, 137),
        (0x14, 108, 107, 141, 140),
        (0x15, 100, 99, 131, 130),
        (0x16, 103, 102, 133, 132),
        (0x17, 95, 94, 128, 127),
        (0x18, 105, 104, 138, 137),
        (0x19, 100, 99, 132, 131),
        (0x1a, 102, 101, 134, 133),
        (0x1b, 107, 106, 139, 138),
        (0x1c, 109, 108, 141, 140),
    ];

    fn plane_state(r: &mut Rig) {
        r.set_state(&[(0, 90), (1, 100), (2, 110), (4, 120), (5, 135), (6, 140)]);
    }

    /// The answer, the hit values and the reported ranges of a plane command whose first half
    /// is the hit command `0x31` and the second `0x33`.
    fn plane_outcome(r: &mut Rig, op: u8, b1: u8, b2: u8) -> (u8, Vec<u32>, Vec<u32>) {
        let result = r.run(&[op, b1, b2, 1, 0x31, 0x33]);
        (result, r.hit_values(), r.ranges.borrow().clone())
    }

    #[test]
    fn combined_axis_plane_commands_choose_between_four_outcomes() {
        let mut r = rig();
        plane_state(&mut r);
        for (op, le, gt, lt, ge) in PLANE_BYTES {
            let name = format!("op {op:#x}");
            // a <= b, c >= d: neither half.
            assert_eq!(
                plane_outcome(&mut r, op, ge, le),
                (0, vec![], vec![]),
                "{name}"
            );
            // a <= b, c < d: the first half (one hit, reported as a range at offset 4).
            assert_eq!(
                plane_outcome(&mut r, op, lt, le),
                (0, vec![1], vec![4]),
                "{name}"
            );
            // a > b, c >= d: the second half (offset 5).
            assert_eq!(
                plane_outcome(&mut r, op, ge, gt),
                (0, vec![3], vec![5]),
                "{name}"
            );
            // a > b, c < d: both halves, both hit, nothing reported.
            assert_eq!(
                plane_outcome(&mut r, op, lt, gt),
                (1, vec![1, 3], vec![]),
                "{name}"
            );
        }
    }

    #[test]
    fn plane_commands_report_the_half_that_was_hit_alone() {
        let mut r = rig();
        plane_state(&mut r);
        // Both halves are queried but only the second (id 3) answers 1.
        let result = r.run(&[0x13, 138, 104, 1, 0x30, 0x33]);
        assert_eq!(result, 0);
        assert_eq!(*r.ranges.borrow(), [5]);
        // Only the first (id 1) answers 1.
        let result = r.run(&[0x13, 138, 104, 1, 0x31, 0x32]);
        assert_eq!(result, 0);
        assert_eq!(*r.ranges.borrow(), [4]);
        // Neither answers 1.
        let result = r.run(&[0x13, 138, 104, 1, 0x30, 0x32]);
        assert_eq!(result, 0);
        assert!(r.ranges.borrow().is_empty());
    }

    #[test]
    fn single_axis_plane_commands_choose_between_four_outcomes() {
        let mut r = rig();
        plane_state(&mut r);
        // 0x11: a = 100, c = 135, b = b2, d = b1.
        assert_eq!(plane_outcome(&mut r, 0x11, 135, 100), (0, vec![], vec![]));
        assert_eq!(plane_outcome(&mut r, 0x11, 136, 100), (0, vec![1], vec![4]));
        assert_eq!(plane_outcome(&mut r, 0x11, 135, 99), (0, vec![3], vec![5]));
        assert_eq!(
            plane_outcome(&mut r, 0x11, 136, 99),
            (1, vec![1, 3], vec![])
        );
    }

    #[test]
    fn one_byte_plane_commands_compare_with_the_byte_and_one_more() {
        // 0x20: a = R0 = 90, c = R4 = 120, b = b1, d = b1 + 1; the second byte is the skip.
        let mut r = rig();
        plane_state(&mut r);
        let outcome = |r: &mut Rig, b1: u8| {
            let result = r.run(&[0x20, b1, 1, 0x31, 0x33]);
            (result, r.hit_values(), r.ranges.borrow().clone())
        };
        assert_eq!(outcome(&mut r, 100), (0, vec![], vec![]));
        assert_eq!(outcome(&mut r, 120), (0, vec![1], vec![3]));
        assert_eq!(outcome(&mut r, 80), (0, vec![3], vec![4]));
        // Both need a box above the lower bound.
        r.set_state(&[(0, 130)]);
        assert_eq!(outcome(&mut r, 125), (1, vec![1, 3], vec![]));
    }

    #[test]
    fn two_jump_plane_commands_use_their_jumps() {
        // 0x25: a = R2 = 110, c = R6 = 140, b = b2, d = b1; the jumps are 0 and 1 behind the
        // seven byte header.
        let mut r = rig();
        plane_state(&mut r);
        let mut outcome = |b1: u8, b2: u8| {
            let result = r.run(&[0x25, b1, b2, 0, 0, 0, 1, 0x31, 0x33]);
            (result, r.hit_values(), r.ranges.borrow().clone())
        };
        assert_eq!(outcome(140, 110), (0, vec![], vec![]));
        assert_eq!(outcome(141, 110), (0, vec![1], vec![7]));
        assert_eq!(outcome(140, 109), (0, vec![3], vec![8]));
        assert_eq!(outcome(141, 109), (1, vec![1, 3], vec![]));
    }

    #[test]
    fn range_commands_return_zero_outside_the_range() {
        let mut r = rig();
        plane_state(&mut r);
        // 0x26: R0 = 90 must be >= b1 and R4 = 120 < b2.
        assert_eq!(r.run(&[0x26, 90, 121, 0x31]), 1);
        assert_eq!(r.run(&[0x26, 91, 121, 0x31]), 0);
        assert!(r.hit_values().is_empty());
        assert_eq!(r.run(&[0x26, 90, 120, 0x31]), 0);
        // 0x28 works on R2 = 110 and R6 = 140.
        assert_eq!(r.run(&[0x28, 110, 141, 0x31]), 1);
        assert_eq!(r.run(&[0x28, 111, 141, 0x31]), 0);
    }

    #[test]
    fn box_range_commands_test_the_machine_s_own_box() {
        let mut r = rig();
        r.e.mem.set_u32(r.this + 0x10, 300);
        r.e.mem.set_u32(r.this + 0x20, 100);
        // 0x29: the upper word 300 must be >= the first 24-bit value and the lower word 100
        // <= the second.
        assert_eq!(r.run(&[0x29, 0, 1, 0x2c, 0, 0, 100, 0x31]), 1);
        assert_eq!(r.run(&[0x29, 0, 1, 0x2d, 0, 0, 100, 0x31]), 0);
        assert_eq!(r.run(&[0x29, 0, 1, 0x2c, 0, 0, 99, 0x31]), 0);
    }

    #[test]
    fn slot_commands_keep_or_drop_the_extra_word() {
        let mut r = rig();
        r.set_state(&[(13, 0x55)]);
        let extra = |r: &Rig| r.hits.borrow()[0].2[13];
        // Slot 0 is the extra word itself and survives the copy of the record.
        r.run(&[0x60, 0x77, 0x31]);
        assert_eq!(extra(&r), 0x77);
        r.run(&[0x64, 0x01, 0x02, 0x31]);
        assert_eq!(extra(&r), 0x0102);
        r.run(&[0x68, 0x01, 0x02, 0x03, 0x04, 0x31]);
        assert_eq!(extra(&r), 0x0102_0304);
        // Another slot keeps the local record's own (never written) word over the copy.
        r.run(&[0x61, 0x09, 0x31]);
        assert_eq!(extra(&r), 0);
        r.run(&[0x67, 0x01, 0x02, 0x31]);
        assert_eq!(extra(&r), 0);
        r.run(&[0x6b, 0x01, 0x02, 0x03, 0x04, 0x31]);
        assert_eq!(extra(&r), 0);
        // Once the local record is the current one, a slot 0 store changes it in place.
        r.run(&[0x61, 0x09, 0x60, 0x66, 0x31]);
        assert_eq!(extra(&r), 0x66);
        assert_eq!(r.e.mem.u32(r.state + 0x34), 0x55);
    }

    #[test]
    fn an_unknown_command_reports_and_continues_on_the_same_byte() {
        unknown_command(0);
    }

    #[test]
    #[should_panic(expected = "00e66950: the exe's INT3 trap")]
    fn an_unknown_command_traps_with_int3_when_the_reporter_asks() {
        unknown_command(1);
    }

    fn unknown_command(wanted: u32) {
        let mut r = rig();
        r.e.mem.write(r.code, &[0x08]);
        let code = r.code;
        let log: Rc<RefCell<Vec<(u32, Vec<u32>)>>> = Rc::new(RefCell::new(Vec::new()));
        for address in [ERR_STREAM_CONSTRUCT, OSTREAM_WRITE_TEXT, OSTREAM_DESTRUCT] {
            let sink = Rc::clone(&log);
            r.e.register_double(address, move |_, a| {
                sink.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
        let sink = Rc::clone(&log);
        r.e.register_double(0x0900_000c, move |e, a| {
            sink.borrow_mut().push((0x0900_000c, a.to_vec()));
            // The handler "repairs" the byte so the next pass ends the query.
            e.mem.set_u8(code, 0x00);
            Ret {
                eax: wanted,
                ..Ret::default()
            }
        });
        let vtable = r.e.mem.alloc(0x20);
        r.e.mem.set_u32(vtable + 0xc, 0x0900_000c);
        let reporter = r.e.mem.alloc(0x10);
        r.e.mem.set_u32(reporter, vtable);
        r.e.set_global(ERROR_REPORTER, reporter);
        r.e.mem.set_u8(r.this + 0x40, 3);
        let (t, o, s, c) = (r.this, r.out, r.state, r.code);
        r.e.call(0x00e6_6950, &args![t, o, s, c, 0i32]);
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
                0x0900_000c,
                vec![reporter, 3, 0x1298_fedd, stream + 0xc, ERROR_FILE, 0x125]
            )
        );
        assert_eq!(log[3], (OSTREAM_DESTRUCT, vec![stream]));
        assert_eq!(r.e.mem.u8(r.out), 3);
    }

    /// A code object with origin (10, 20, 30) and scale 2, and a box (8.5, 21.25, 30.5) to
    /// (12.25, 22.75, 33.25).
    fn aabb_setup(r: &mut Rig, first_code_bytes: &[u8]) -> (u32, u32) {
        let mopp = r.e.mem.u32(r.this + 0x30);
        for (i, origin) in [10.0f32, 20.0, 30.0].into_iter().enumerate() {
            r.e.mem.set_f32(mopp + 0x10 + 4 * i as u32, origin);
        }
        r.e.mem.set_f32(mopp + 0x1c, 2.0);
        r.e.mem.write(r.code, first_code_bytes);
        let aabb = r.e.mem.alloc(0x20);
        for (i, value) in [8.5f32, 21.25, 30.5, 0.0, 12.25, 22.75, 33.25]
            .into_iter()
            .enumerate()
        {
            r.e.mem.set_f32(aabb + 4 * i as u32, value);
        }
        (mopp, aabb)
    }

    #[test]
    fn query_aabb_turns_the_box_into_integers_and_builds_the_start_record() {
        let mut r = rig();
        // The code is one hit command so the record can be looked at.
        let (mopp, aabb) = aabb_setup(&mut r, &[0x31]);
        let t = r.this;
        let collector = r.e.mem.u32(t + 0x44);
        // The box: x (8.5 - 10) * 2 = -3 -> -4; (12.25 - 10) * 2 = 4.5 -> 4 (ties to even) + 1;
        // y (21.25 - 20) * 2 = 2.5 -> 2 - 1; (22.75 - 20) * 2 = 5.5 -> 6 + 1; z
        // (30.5 - 30) * 2 = 1 - 1; (33.25 - 30) * 2 = 6.5 -> 6 + 1.
        r.e.call(0x00e6_7230, &args![t, mopp, aabb, collector]);
        assert_eq!(r.e.mem.u32(t + 0x30), mopp);
        assert_eq!(r.e.mem.u32(t + 0x44), collector);
        assert_eq!(r.e.mem.u32(t + 0x20) as i32, -4);
        assert_eq!(r.e.mem.u32(t + 0x10), 5);
        assert_eq!(r.e.mem.u32(t + 0x24), 1);
        assert_eq!(r.e.mem.u32(t + 0x14), 7);
        assert_eq!(r.e.mem.u32(t + 0x28), 0);
        assert_eq!(r.e.mem.u32(t + 0x18), 7);
        assert_eq!(r.e.mem.u32(t + 0x48), 0);
        // The record: upper high halves + 1 = (1, 1, 1), lower high halves sign-extended
        // (-1 for the first word -4, 0 for the others), all else zero. The hit command
        // returns 1 (an odd value) so the collector is told 0.
        let hits = r.hits.borrow();
        assert_eq!(hits.len(), 1);
        assert_eq!(
            defined_words(&hits[0].2),
            [1, 1, 1, 0xffff_ffff, 0, 0, 0, 0, 0, 0, 0, 0]
        );
        assert_eq!(*r.ranges.borrow(), [0]);
    }

    #[test]
    fn query_aabb_with_a_first_byte_of_0d_sets_the_mask_and_the_range_kind() {
        let mut r = rig();
        let (mopp, aabb) = aabb_setup(&mut r, &[0x0d, 0, 0, 0, 0, 0x31]);
        let t = r.this;
        let collector = r.e.mem.u32(t + 0x44);
        r.e.call(0x00e6_7230, &args![t, mopp, aabb, collector]);
        assert_eq!(r.e.mem.u32(t + 0x48), 0xffff_ffff);
        assert_eq!(*r.ranges.borrow(), [5]);
        // A second query with a code that finds nothing tells the collector nothing.
        r.ranges.borrow_mut().clear();
        r.e.mem.write(r.code, &[0x30]);
        r.e.call(0x00e6_7230, &args![t, mopp, aabb, collector]);
        assert!(r.ranges.borrow().is_empty());
        assert_eq!(r.e.mem.u32(t + 0x48), 0);
    }

    #[test]
    fn query_aabb_is_gated_by_the_key_check() {
        let mut r = rig();
        let (mopp, aabb) = aabb_setup(&mut r, &[0x31]);
        r.e.mem.set_u8(LICENSE_FLAG, 0);
        let calls = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&calls);
        r.e.register_double(LICENSE_CHECK, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        let t = r.this;
        let collector = r.e.mem.u32(t + 0x44);
        r.e.mem.set_u32(t + 0x30, 0);
        r.e.call(0x00e6_7230, &args![t, mopp, aabb, collector]);
        assert_eq!(*calls.borrow(), [vec![LICENSE_FLAG]]);
        assert_eq!(r.e.mem.u32(t + 0x30), 0);
        assert!(r.hits.borrow().is_empty());
    }
}

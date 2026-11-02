//! `Havok SDK/hkpmoppobbvirtualmachine.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// A byte in the exe image (`01269cbc`): non-zero once the product key check has passed.
const LICENSE_FLAG: u32 = 0x0126_9cbc;
/// `hkpProcessFlyingColors` (Xbox PDB, `00df22f0`, cdecl, one byte pointer), translated in
/// another unit and called by address.
const LICENSE_CHECK: u32 = 0x00df_22f0;
/// The memory router (`00c85750`); its word at +0x10 is the allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkArrayUtil::_reserveMore` (Xbox PDB, `00c90940`, cdecl `(allocator, array, elementSize)`).
const ARRAY_RESERVE_MORE: u32 = 0x00c9_0940;
/// Constructor of `hkErrStream` (`00cb1600`, `(this, buffer, size)`).
const ERR_STREAM_CONSTRUCT: u32 = 0x00cb_1600;
/// Writes text into the stream (`00cc01c0`, `(this, text)`) and destroys it (`00cc00e0`).
const OSTREAM_WRITE_TEXT: u32 = 0x00cc_01c0;
const OSTREAM_DESTRUCT: u32 = 0x00cc_00e0;
/// The global pointer to the Havok error reporter (`01268360`); its vtable offset 0xc reports
/// a message `(level, id, text, file, line)` and returns non-zero to request a break.
const ERROR_REPORTER: u32 = 0x0126_8360;
/// The message `Unknown command - This mopp data has been corrupted (check for m...` (`010d5100`)
/// and the source file string `.\Collide\Mopp\Machine\hkpMoppObbVirtualMachine.cpp`
/// (`010d7af8`).
const CORRUPTED_TEXT: u32 = 0x010d_5100;
const ERROR_FILE: u32 = 0x010d_7af8;
/// A vector of four absolute-value masks (`01017d10`) and one of four sign bits (`010c44d0`).
const ABS_MASK: u32 = 0x0101_7d10;
const SIGN_MASK: u32 = 0x010c_44d0;

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

fn short(e: &Engine, at: u32) -> i32 {
    i32::from(e.mem.u16(at) as i16)
}

/// A big-endian value of `count` bytes starting at `at`.
fn big_endian(e: &Engine, at: u32, count: u32) -> i32 {
    (0..count).fold(0i32, |acc, i| (acc << 8).wrapping_add(byte(e, at + i)))
}

/// `(value - origin) * scale` in x87 arithmetic with the product stored as a float.
fn scaled(e: &Engine, at: u32, origin: f32, scale: f32) -> f32 {
    let value = f64::from(x87_quiet(e.mem.f32(at)));
    ((value - f64::from(origin)) * f64::from(scale)) as f32
}

fn lanes(e: &Engine, at: u32) -> [f32; 4] {
    [0, 1, 2, 3].map(|i| e.mem.f32(at + 4 * i))
}

fn zip(a: [f32; 4], b: [f32; 4], op: impl Fn(f32, f32) -> f32) -> [f32; 4] {
    [0, 1, 2, 3].map(|i| op(a[i], b[i]))
}

/// Copies the 0x40-byte record at `from` over the one at `to` (`REP MOVSD`, 0x10 words).
fn copy_record(e: &mut Engine, from: u32, to: u32) {
    for i in 0..0x10 {
        let word = e.mem.u32(from + 4 * i);
        e.mem.set_u32(to + 4 * i, word);
    }
}

// Translated from 00dd1400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMoppObbVirtualMachine::queryAabbOnTree` (Xbox PDB), thiscall `(this, state, code,
/// offset)`: interprets the command bytes of a MOPP tree at `code`, appending the hit ids to
/// the array at `this + 0`. Does nothing until the product key check (`00df22f0`) has set the
/// byte at `01269cbc`. `state` is a record of integers: `+0/+4/+8` and `+0x10/+0x14/+0x18` the
/// lower and upper bounds, `+0x20/+0x24/+0x28`, `+0x2c` an id offset, `+0x30` a shift and
/// `+0x34` a word the one at `this + 0x34` masks; `this + 0x10..0x18` and `this + 0x20..0x28`
/// hold bounds, `this + 0x30` the code object (its pointer at +0x20 is the code base) and
/// `this + 0x34` a mask. Commands (the byte table `00dd1b74`): 0 returns; 1 to 4 build a
/// narrowed local copy of the record by the shift; 5 to 7 and 9 to 0xd skip or adjust ids (copying
/// the record to a local one first when it is not already); 0x10 to 0x2b test bounds and skip
/// or recurse into sub-trees; 0x13 to 0x1c compare combined bounds; 0x30 to 0x53 append an id
/// to the array and return; 0x60 to 0x6b store a value into a local slot; 0x70 and 0xc jump
/// relative to the code base; any other command reports the "Unknown command" error
/// (`hkErrStream`, `00cb1600`, the reporter's virtual function at 0xc, then `INT3`, which
/// this translation panics on: it is a breakpoint trap in the exe, not a call) and continues
/// with the same byte. The command meanings beyond this are unconfirmed.
///
/// Uninitialised stack words (checked against the disassembly): the exe's frame words
/// `ESP+0x10/0x14/0x18` are written before every read; the local 0x40-byte record at
/// `ESP+0x20` is built by command 1 or copied whole from the state record (`REP MOVSD`)
/// before any read of its fields, and the words the exe never writes in it (+0xc, +0x1c,
/// +0x38, +0x3c, and the frame word at `ESP+0x60`) are only ever copied on (into the local
/// record of a recursive call), never compared, stored to the array or returned. The one
/// read of an unwritten word is the `ESP+0x54` (+0x34, the mask word) kept across the copy by
/// commands 0x61 to 0x6b; it lands in the local record's +0x34, which is only copied by
/// command 1 (the mask the results use is `this + 0x34`). They read as zero here, with no
/// effect on the results; the callers' own state records (`00dd1bf0`, `00dd1da0`) write every
/// word the interpreter reads.
pub fn hkp_mopp_obb_virtual_machine_query_aabb_on_tree(
    e: &mut Engine,
    this: Ptr,
    state: Ptr,
    code: Ptr,
    offset: i32,
) {
    if e.mem.u8(LICENSE_FLAG) == 0 {
        e.call(LICENSE_CHECK, &args![LICENSE_FLAG]);
        if e.mem.u8(LICENSE_FLAG) == 0 {
            return;
        }
    }
    let t = this.addr();
    e.with_stack(0x280, |e, frame| {
        let f = frame.addr();
        for i in 0..0x1c {
            e.mem.set_u32(f + 4 * i, 0);
        }
        let local = f + 0x20;
        let mut s = state.addr();
        let mut c = code.addr();
        let mut a3 = offset;
        loop {
            let op = u32::from(e.mem.u8(c));
            // The compiler's switch on the command byte: the case of each byte from 0 to 0x70
            // (case 34 is the corrupted-data error, also the case of a byte past 0x70).
            let case = match op {
                0 => 0,
                1..=4 => 1,
                5 => 2,
                6 => 3,
                7 => 4,
                9 => 5,
                10 => 6,
                11 => 7,
                12 => 8,
                13 => 9,
                16..=18 => 10,
                19 => 11,
                20 => 12,
                21 => 13,
                22 => 14,
                23 => 15,
                24 => 16,
                25 => 17,
                26 => 18,
                27 => 19,
                28 => 20,
                32..=34 => 21,
                35..=37 => 22,
                38..=40 => 23,
                41..=43 => 24,
                48..=79 => 25,
                80 => 26,
                81 => 27,
                82 => 28,
                83 => 29,
                96..=99 => 30,
                100..=103 => 31,
                104..=107 => 32,
                112 => 33,
                _ => 34,
            };
            match case {
                0 => return,
                1 => {
                    let shift = op;
                    let x = (byte(e, c + 1).wrapping_add(int(e, s + 0x20))) << shift;
                    let y = (byte(e, c + 2).wrapping_add(int(e, s + 0x24))) << shift;
                    let z = (byte(e, c + 3).wrapping_add(int(e, s + 0x28))) << shift;
                    e.mem.set_u32(local + 0x20, x as u32);
                    e.mem.set_u32(local + 0x24, y as u32);
                    e.mem.set_u32(local + 0x28, z as u32);
                    let total = int(e, s + 0x30).wrapping_add(shift as i32);
                    e.mem.set_u32(local + 0x30, total as u32);
                    let count = (0x10i32.wrapping_sub(total) as u32) & 31;
                    for (i, origin) in [x, y, z].into_iter().enumerate() {
                        let i = i as u32;
                        let low = (int(e, t + 0x20 + 4 * i) >> count).wrapping_sub(origin);
                        e.mem.set_u32(local + 0x10 + 4 * i, low as u32);
                        let high = (int(e, t + 0x10 + 4 * i) >> count)
                            .wrapping_sub(origin)
                            .wrapping_add(1);
                        e.mem.set_u32(local + 4 * i, high as u32);
                    }
                    let mask_word = e.mem.u32(s + 0x34);
                    e.mem.set_u32(local + 0x34, mask_word);
                    let id_offset = e.mem.u32(s + 0x2c);
                    e.mem.set_u32(local + 0x2c, id_offset);
                    s = local;
                    c += 4;
                }
                2 => c += 2 + byte(e, c + 1) as u32,
                3 => c += 3 + big_endian(e, c + 1, 2) as u32,
                4 => c += 4 + big_endian(e, c + 1, 3) as u32,
                5..=7 => {
                    let value = big_endian(e, c + 1, [1, 2, 4][case as usize - 5]);
                    if s != local {
                        copy_record(e, s, local);
                        s = local;
                    }
                    let id_offset = int(e, local + 0x2c);
                    let updated = if case == 7 {
                        value
                    } else {
                        id_offset.wrapping_add(value)
                    };
                    e.mem.set_u32(local + 0x2c, updated as u32);
                    c += [2, 3, 5][case as usize - 5];
                }
                8 => {
                    let base = e.mem.u32(e.mem.u32(t + 0x30) + 0x20);
                    a3 = big_endian(e, c + 1, 2) << 9;
                    c = base.wrapping_add(a3 as u32);
                }
                9 => c += 5,
                10 => {
                    let k = (op - 0x10) * 4;
                    let b1 = byte(e, c + 1);
                    let b2 = byte(e, c + 2);
                    c += 4;
                    let low = int(e, s + k);
                    let high = int(e, s + 0x10 + k);
                    if low > b2 {
                        let skip = u32::from(e.mem.u8(c - 1));
                        c += skip;
                        if high < b1 {
                            hkp_mopp_obb_virtual_machine_query_aabb_on_tree(
                                e,
                                this,
                                Ptr::new(s),
                                Ptr::new(c - skip),
                                a3,
                            );
                        }
                    } else if high >= b1 {
                        return;
                    }
                }
                11..=20 => {
                    let b1 = byte(e, c + 1);
                    let b2 = byte(e, c + 2);
                    let (s0, s4, s8) = (int(e, s), int(e, s + 4), int(e, s + 8));
                    let (s10, s14, s18) = (int(e, s + 0x10), int(e, s + 0x14), int(e, s + 0x18));
                    let (a, b, cc, d) = match case {
                        11 => (s8.wrapping_add(s4), 2 * b2, s18.wrapping_add(s14), 2 * b1),
                        12 => (
                            s4.wrapping_sub(s18),
                            2 * b2 - 255,
                            s14.wrapping_sub(s8),
                            2 * b1 - 255,
                        ),
                        13 => (s8.wrapping_add(s0), 2 * b2, s18.wrapping_add(s10), 2 * b1),
                        14 => (
                            s0.wrapping_sub(s18),
                            2 * b2 - 255,
                            s10.wrapping_sub(s8),
                            2 * b1 - 255,
                        ),
                        15 => (s4.wrapping_add(s0), 2 * b2, s14.wrapping_add(s10), 2 * b1),
                        16 => (
                            s0.wrapping_sub(s14),
                            2 * b2 - 255,
                            s10.wrapping_sub(s4),
                            2 * b1 - 255,
                        ),
                        17 => (
                            s8.wrapping_add(s4).wrapping_add(s0),
                            3 * b2,
                            s18.wrapping_add(s14).wrapping_add(s10),
                            3 * b1,
                        ),
                        18 => (
                            s4.wrapping_sub(s18).wrapping_add(s0),
                            3 * (b2 - 0x55),
                            s14.wrapping_sub(s8).wrapping_add(s10),
                            3 * (b1 - 0x55),
                        ),
                        19 => (
                            s8.wrapping_sub(s14).wrapping_add(s0),
                            3 * (b2 - 0x55),
                            s18.wrapping_sub(s4).wrapping_add(s10),
                            3 * (b1 - 0x55),
                        ),
                        _ => (
                            s0.wrapping_sub(s18).wrapping_sub(s14),
                            3 * (b2 - 0xaa),
                            s10.wrapping_sub(s8).wrapping_sub(s4),
                            3 * (b1 - 0xaa),
                        ),
                    };
                    c += 4;
                    if a > b {
                        let skip = u32::from(e.mem.u8(c - 1));
                        if cc < d {
                            hkp_mopp_obb_virtual_machine_query_aabb_on_tree(
                                e,
                                this,
                                Ptr::new(s),
                                Ptr::new(c),
                                a3,
                            );
                        }
                        c += skip;
                    } else if cc >= d {
                        return;
                    }
                }
                21 => {
                    let k = (op - 0x20) * 4;
                    let b1 = byte(e, c + 1);
                    c += 3;
                    let low = int(e, s + k);
                    let high = int(e, s + 0x10 + k);
                    if low > b1 {
                        let skip = u32::from(e.mem.u8(c - 1));
                        c += skip;
                        if high <= b1 {
                            hkp_mopp_obb_virtual_machine_query_aabb_on_tree(
                                e,
                                this,
                                Ptr::new(s),
                                Ptr::new(c - skip),
                                a3,
                            );
                        }
                    }
                }
                22 => {
                    let k = (op - 0x23) * 4;
                    let b1 = byte(e, c + 1);
                    let b2 = byte(e, c + 2);
                    let high = int(e, s + 0x10 + k);
                    let long_skip = big_endian(e, c + 3, 2) as u32;
                    c += 7;
                    if int(e, s + k) > b2 {
                        let short_skip = big_endian(e, c - 2, 2) as u32;
                        c += short_skip;
                        if high < b1 {
                            hkp_mopp_obb_virtual_machine_query_aabb_on_tree(
                                e,
                                this,
                                Ptr::new(s),
                                Ptr::new(c.wrapping_add(long_skip).wrapping_sub(short_skip)),
                                a3,
                            );
                        }
                    } else if high >= b1 {
                        return;
                    } else {
                        c += long_skip;
                    }
                }
                23 => {
                    let k = (op - 0x26) * 4;
                    if int(e, s + k) < byte(e, c + 1) {
                        return;
                    }
                    if int(e, s + 0x10 + k) >= byte(e, c + 2) {
                        return;
                    }
                    c += 3;
                }
                24 => {
                    let k = (op - 0x29) * 4;
                    if int(e, t + 0x10 + k) < big_endian(e, c + 1, 3) {
                        return;
                    }
                    if int(e, t + 0x20 + k) > big_endian(e, c + 4, 3) {
                        return;
                    }
                    c += 7;
                }
                25..=29 => {
                    let amount = match case {
                        25 => op as i32 - 0x30,
                        26 => big_endian(e, c + 1, 1),
                        27 => big_endian(e, c + 1, 2),
                        28 => big_endian(e, c + 1, 3),
                        _ => big_endian(e, c + 1, 4),
                    };
                    let masked = ((a3 >> 9) << 8) as u32 & e.mem.u32(t + 0x34);
                    let value = masked | int(e, s + 0x2c).wrapping_add(amount) as u32;
                    let array = e.mem.u32(t);
                    let router = e.call(MEMORY_ROUTER, &args![]).u32();
                    let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
                    if e.mem.u32(array + 4) == capacity {
                        let allocator = e.mem.u32(router + 0x10);
                        e.call(ARRAY_RESERVE_MORE, &args![allocator, array, 4u32]);
                    }
                    let size = e.mem.u32(array + 4);
                    let data = e.mem.u32(array);
                    e.mem.set_u32(array + 4, size.wrapping_add(1));
                    e.mem
                        .set_u32(data.wrapping_add(size.wrapping_mul(4)), value);
                    return;
                }
                30..=32 => {
                    let (slot, value, length) = match case {
                        30 => (f + op * 4 - 0x12c, big_endian(e, c + 1, 1), 2),
                        31 => (f + op * 4 - 0x13c, big_endian(e, c + 1, 2), 3),
                        _ => (f + op * 4 - 0x14c, big_endian(e, c + 1, 4), 5),
                    };
                    e.mem.set_u32(slot, value as u32);
                    c += length;
                    // The exe keeps the word at +0x54 in a register across the copy, so only
                    // that slot survives it.
                    let kept = e.mem.u32(f + 0x54);
                    if s != local {
                        copy_record(e, s, local);
                        s = local;
                    }
                    e.mem.set_u32(f + 0x54, kept);
                }
                33 => {
                    let base = e.mem.u32(e.mem.u32(t + 0x30) + 0x20);
                    a3 = big_endian(e, c + 1, 4);
                    c = base.wrapping_add(a3 as u32);
                }
                _ => {
                    let stream = f + 0x64;
                    let buffer = f + 0x70;
                    e.call(ERR_STREAM_CONSTRUCT, &args![stream, buffer, 0x200u32]);
                    e.call(OSTREAM_WRITE_TEXT, &args![stream, CORRUPTED_TEXT]);
                    let reporter: u32 = e.global(ERROR_REPORTER);
                    let wanted_break = e
                        .vcall(
                            reporter,
                            0xc,
                            &args![3u32, 0x1298_feddu32, buffer, ERROR_FILE, 0x1cbu32],
                        )
                        .u32();
                    if wanted_break != 0 {
                        panic!(
                            "00dd1400: the exe's INT3 trap (the error reporter asked for a break)"
                        );
                    }
                    e.call(OSTREAM_DESTRUCT, &args![stream]);
                }
            }
        }
    });
}

/// The shared tail of `queryObb` and `queryAabb` (the exe repeats it inline in both): with
/// the six bounds already stored at `this + 0x10..0x28`, builds the 0x40-byte state record
/// (the 16-bit halves at `this + 0x12/0x16` plus one and the upper half of the word at
/// `this + 0x18` plus one as the first three words, the 16-bit words at `this + 0x22/0x26/
/// 0x2a` at +0x10, the rest zero), stores the mask at `this + 0x34` (all ones when the first
/// command byte is 0x0d, else zero) and runs `queryAabbOnTree` on the code at the mopp
/// object's +0x20.
fn run_tree_query(e: &mut Engine, this: u32, mopp: u32) {
    e.with_stack(0x40, |e, record| {
        let r = record.addr();
        for i in 0..0x10 {
            e.mem.set_u32(r + 4 * i, 0);
        }
        e.mem.set_u32(r + 0x10, short(e, this + 0x22) as u32);
        e.mem
            .set_u32(r, short(e, this + 0x12).wrapping_add(1) as u32);
        e.mem.set_u32(r + 0x14, short(e, this + 0x26) as u32);
        e.mem
            .set_u32(r + 4, short(e, this + 0x16).wrapping_add(1) as u32);
        e.mem.set_u32(r + 0x18, short(e, this + 0x2a) as u32);
        let top = (int(e, this + 0x18) >> 16).wrapping_add(1);
        e.mem.set_u32(r + 8, top as u32);
        let code = e.mem.u32(mopp + 0x20);
        let not_marker = u32::from(e.mem.u8(code) != 0x0d);
        e.mem.set_u32(this + 0x34, not_marker.wrapping_sub(1));
        hkp_mopp_obb_virtual_machine_query_aabb_on_tree(
            e,
            Ptr::new(this),
            Ptr::new(r),
            Ptr::new(code),
            0,
        );
    });
}

// Translated from 00dd1bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMoppObbVirtualMachine::queryObb` (Xbox PDB), thiscall `(this, mopp, matrix, vector,
/// extra, output)`; callee pops 5 words after `this`. Stores `output` at `this + 0` and
/// `mopp` at `this + 0x30`, then bounds the oriented box: the sum of the absolute values of
/// the vector's three lanes times the matrix rows 0 to 2, plus `extra` in every lane, is the
/// half-extent `h`; with the translation row (`matrix + 0x30`) minus the vector at
/// `mopp + 0x10` as `p`, the lower bounds `fistp(scale * (p - h))` minus one go to
/// `this + 0x20..0x28` and the upper bounds `fistp(scale * (p + h))` plus one to
/// `this + 0x10..0x18`, with `scale` the float at `mopp + 0x1c`. Then runs the tree query
/// (`queryAabbOnTree`, same unit) on the code. The `FISTP` rounding is assumed to be the
/// default mode (nearest, ties to even).
pub fn hkp_mopp_obb_virtual_machine_query_obb(
    e: &mut Engine,
    this: Ptr,
    mopp: Ptr,
    matrix: Ptr,
    vector: Ptr,
    extra: f32,
    output: Ptr,
) {
    let t = this.addr();
    let m = mopp.addr();
    e.mem.set_u32(t, output.addr());
    e.mem.set_u32(t + 0x30, m);
    let scale = x87_quiet(e.mem.f32(m + 0x1c));
    let v = lanes(e, vector.addr());
    let rows = [0, 0x10, 0x20, 0x30].map(|o| lanes(e, matrix.addr() + o));
    let abs_mask = [0, 4, 8, 12].map(|o| e.mem.u32(ABS_MASK + o));
    let sign_mask = [0, 4, 8, 12].map(|o| e.mem.u32(SIGN_MASK + o));
    let masked = |p: [f32; 4]| [0, 1, 2, 3].map(|i| f32::from_bits(p[i].to_bits() & abs_mask[i]));
    let product = |lane: f32, row: [f32; 4]| masked(zip([lane; 4], row, |a, b| a * b));
    let y_term = product(v[1], rows[1]);
    let x_term = product(v[0], rows[0]);
    let mut half = zip(y_term, x_term, |a, b| a + b);
    let z_term = product(v[2], rows[2]);
    let origin = lanes(e, m + 0x10);
    half = zip(half, z_term, |a, b| a + b);
    half = zip(half, [extra; 4], |a, b| a + b);
    let centre = zip(rows[3], origin, |a, b| a - b);
    let negated = [0, 1, 2, 3].map(|i| f32::from_bits(half[i].to_bits() ^ sign_mask[i]));
    let lower = zip(zip(negated, centre, |a, b| a + b), [scale; 4], |a, b| b * a);
    let upper = zip(zip(centre, half, |a, b| a + b), [scale; 4], |a, b| a * b);
    for i in 0..3u32 {
        let low = fistp(x87_quiet(lower[i as usize]));
        e.mem.set_u32(t + 0x20 + 4 * i, low.wrapping_sub(1) as u32);
        let high = fistp(x87_quiet(upper[i as usize]));
        e.mem.set_u32(t + 0x10 + 4 * i, high.wrapping_add(1) as u32);
    }
    run_tree_query(e, t, m);
}

// Translated from 00dd1da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMoppObbVirtualMachine::queryAabb` (Xbox PDB), thiscall `(this, mopp, box, output)`;
/// callee pops 3 words after `this`. Stores `output` at `this + 0` and `mopp` at
/// `this + 0x30`. With `origin` the vector at `mopp + 0x10` (the three floats) and `scale`
/// the float at `mopp + 0x1c`, the box's lower corner (`box + 0..8`) gives
/// `fistp((lower - origin) * scale) - 1` at `this + 0x20..0x28` and its upper corner
/// (`box + 0x10..0x18`) `fistp((upper - origin) * scale) + 1` at `this + 0x10..0x18`
/// (x87 arithmetic, the product stored as a float before the conversion). Then runs the tree
/// query (`queryAabbOnTree`, same unit) on the code. The `FISTP` rounding is assumed to be
/// the default mode.
pub fn hkp_mopp_obb_virtual_machine_query_aabb(
    e: &mut Engine,
    this: Ptr,
    mopp: Ptr,
    bounds: Ptr,
    output: Ptr,
) {
    let t = this.addr();
    let m = mopp.addr();
    let b = bounds.addr();
    e.mem.set_u32(t, output.addr());
    e.mem.set_u32(t + 0x30, m);
    let scale = x87_quiet(e.mem.f32(m + 0x1c));
    for i in 0..3u32 {
        let origin = x87_quiet(e.mem.f32(m + 0x10 + 4 * i));
        let low = fistp(scaled(e, b + 4 * i, origin, scale));
        e.mem.set_u32(t + 0x20 + 4 * i, low.wrapping_sub(1) as u32);
        let high = fistp(scaled(e, b + 0x10 + 4 * i, origin, scale));
        e.mem.set_u32(t + 0x10 + 4 * i, high.wrapping_add(1) as u32);
    }
    run_tree_query(e, t, m);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00dd_1400,
            hkp_mopp_obb_virtual_machine_query_aabb_on_tree(Ptr, Ptr, Ptr, i32)
        ),
        entry!(
            0x00dd_1bf0,
            hkp_mopp_obb_virtual_machine_query_obb(Ptr, Ptr, Ptr, Ptr, f32, Ptr)
        ),
        entry!(
            0x00dd_1da0,
            hkp_mopp_obb_virtual_machine_query_aabb(Ptr, Ptr, Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod movement_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// An engine with the exe's data range and the import page mapped, as the tests run without the exe.
    fn test_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    struct Rig {
        e: Engine,
        log: Log,
        this: u32,
        state: u32,
        code: u32,
        array: u32,
    }

    fn rig(code_bytes: &[u8]) -> Rig {
        let mut e = test_engine();
        e.mem.set_u8(LICENSE_FLAG, 1);
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, 0xa110c);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let sink = Rc::clone(&log);
        e.register_double(ARRAY_RESERVE_MORE, move |e, a| {
            sink.borrow_mut().push((ARRAY_RESERVE_MORE, a.to_vec()));
            let cap = e.mem.u32(a[1] + 8);
            e.mem.set_u32(a[1] + 8, cap + 4);
            Ret::default()
        });
        let code = e.mem.alloc(0x400);
        e.mem.write(code, code_bytes);
        let mopp = e.mem.alloc(0x40);
        e.mem.set_u32(mopp + 0x20, code);
        let array = e.mem.alloc(0x10);
        let data = e.mem.alloc(0x40);
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 8, 4);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this, array);
        e.mem.set_u32(this + 0x30, mopp);
        e.mem.set_u32(this + 0x34, 0xffff_ffff);
        let state = e.mem.alloc(0x40);
        Rig {
            e,
            log,
            this,
            state,
            code,
            array,
        }
    }

    impl Rig {
        fn run(&mut self) {
            let (t, s, c) = (self.this, self.state, self.code);
            self.e.call(0x00dd_1400, &args![t, s, c, 0i32]);
        }

        fn ids(&self) -> Vec<u32> {
            let data = self.e.mem.u32(self.array);
            (0..self.e.mem.u32(self.array + 4))
                .map(|i| self.e.mem.u32(data + 4 * i))
                .collect()
        }
    }

    #[test]
    fn the_key_check_gates_the_interpreter() {
        let mut r = rig(&[0x31]);
        r.e.mem.set_u8(LICENSE_FLAG, 0);
        let sink = Rc::clone(&r.log);
        let passes = Rc::new(std::cell::Cell::new(false));
        let switch = Rc::clone(&passes);
        r.e.register_double(LICENSE_CHECK, move |e, a| {
            sink.borrow_mut().push((LICENSE_CHECK, a.to_vec()));
            if switch.get() {
                e.mem.set_u8(a[0], 1);
            }
            Ret::default()
        });
        r.run();
        assert!(r.ids().is_empty());
        assert_eq!(r.log.borrow()[0], (LICENSE_CHECK, vec![LICENSE_FLAG]));
        // Once the check sets the byte, the command runs.
        passes.set(true);
        r.run();
        assert_eq!(r.ids(), vec![1]);
    }

    #[test]
    fn id_commands_append_and_skips_move_the_pointer() {
        // 0x31 emits 1; the id offset in the state is added, the masked offset word is or-ed.
        let mut r = rig(&[0x31]);
        r.e.mem.set_u32(r.state + 0x2c, 10);
        r.run();
        assert_eq!(r.ids(), vec![11]);
        // A one-byte skip over a filler byte reaches the second id command.
        let mut r = rig(&[0x05, 0x01, 0xaa, 0x32]);
        r.run();
        assert_eq!(r.ids(), vec![2]);
        // Command 0x0d skips five bytes; command 0 returns without an id.
        let mut r = rig(&[0x0d, 0, 0, 0, 0, 0x33]);
        r.run();
        assert_eq!(r.ids(), vec![3]);
        let mut r = rig(&[0x00, 0x33]);
        r.run();
        assert!(r.ids().is_empty());
        // The 16-bit id command 0x51 appends the value 0x0102.
        let mut r = rig(&[0x51, 0x01, 0x02]);
        r.run();
        assert_eq!(r.ids(), vec![0x102]);
    }

    #[test]
    fn a_full_array_is_grown_before_the_append() {
        let mut r = rig(&[0x31]);
        r.e.mem.set_u32(r.array + 4, 4);
        r.run();
        let log = r.log.borrow();
        assert_eq!(log[0].0, ARRAY_RESERVE_MORE);
        assert_eq!(log[0].1, vec![0xa110c, r.array, 4]);
        drop(log);
        assert_eq!(r.e.mem.u32(r.array + 4), 5);
    }

    #[test]
    fn the_masked_offset_is_or_ed_into_the_id() {
        // Command 8 jumps to the code base plus (0x0001 << 9) = 0x200, where an id is emitted.
        let mut r = rig(&[0x0c, 0x00, 0x01]);
        let target = r.code + 0x200;
        r.e.mem.write(target, &[0x30]);
        r.run();
        // a3 = 0x200: (0x200 >> 9) << 8 = 0x100, masked by all ones, or-ed with the id 0.
        assert_eq!(r.ids(), vec![0x100]);
    }

    #[test]
    fn combined_bound_commands_skip_or_descend() {
        // Command 0x13 with state sums 0 and a second byte 0: continue only when the second
        // sum is below twice the first byte.
        let mut r = rig(&[0x13, 1, 0, 0, 0x31]);
        r.run();
        assert_eq!(r.ids(), vec![1]);
        let mut r = rig(&[0x13, 0, 0, 0, 0x31]);
        r.run();
        assert!(r.ids().is_empty());
        // With the first sum above the bound, the sub-tree at the next byte runs when the
        // second sum is below, and then the skip byte (2) is taken: both ids appear.
        let mut r = rig(&[0x13, 1, 1, 2, 0x31, 0x00, 0x32]);
        r.e.mem.set_u32(r.state + 4, 10);
        r.run();
        assert_eq!(r.ids(), vec![1, 2]);
        // With the second sum not below, the sub-tree is skipped.
        let mut r = rig(&[0x13, 0, 1, 2, 0x31, 0x00, 0x32]);
        r.e.mem.set_u32(r.state + 4, 10);
        r.run();
        assert_eq!(r.ids(), vec![2]);
    }

    #[test]
    fn range_commands_test_one_axis() {
        // 0x26: return when the lower bound is below the first byte.
        let mut r = rig(&[0x26, 5, 9, 0x31]);
        r.e.mem.set_u32(r.state, 4);
        r.run();
        assert!(r.ids().is_empty());
        let mut r = rig(&[0x26, 5, 9, 0x31]);
        r.e.mem.set_u32(r.state, 6);
        r.e.mem.set_u32(r.state + 0x10, 8);
        r.run();
        assert_eq!(r.ids(), vec![1]);
        // The upper bound at or above the second byte ends the query.
        let mut r = rig(&[0x26, 5, 9, 0x31]);
        r.e.mem.set_u32(r.state, 6);
        r.e.mem.set_u32(r.state + 0x10, 9);
        r.run();
        assert!(r.ids().is_empty());
        // 0x29 compares the query's own bounds: upper below the 24-bit value ends it.
        let mut r = rig(&[0x29, 0, 0, 5, 0, 0, 9, 0x31]);
        r.e.mem.set_u32(r.this + 0x10, 4);
        r.run();
        assert!(r.ids().is_empty());
        let mut r = rig(&[0x29, 0, 0, 5, 0, 0, 9, 0x31]);
        r.e.mem.set_u32(r.this + 0x10, 5);
        r.e.mem.set_u32(r.this + 0x20, 9);
        r.run();
        assert_eq!(r.ids(), vec![1]);
        let mut r = rig(&[0x29, 0, 0, 5, 0, 0, 9, 0x31]);
        r.e.mem.set_u32(r.this + 0x10, 5);
        r.e.mem.set_u32(r.this + 0x20, 10);
        r.run();
        assert!(r.ids().is_empty());
    }

    #[test]
    fn the_id_offset_commands_copy_the_state_record_first() {
        // Command 9 adds 3 to the id offset of a local copy; the caller's record is unchanged.
        let mut r = rig(&[0x09, 3, 0x30]);
        r.e.mem.set_u32(r.state + 0x2c, 4);
        r.run();
        assert_eq!(r.ids(), vec![7]);
        assert_eq!(r.e.mem.u32(r.state + 0x2c), 4);
        // Command 0x0b sets it from four bytes.
        let mut r = rig(&[0x0b, 0, 0, 1, 0, 0x30]);
        r.e.mem.set_u32(r.state + 0x2c, 4);
        r.run();
        assert_eq!(r.ids(), vec![0x100]);
    }

    #[test]
    fn narrowing_commands_shift_the_bounds_into_a_local_record() {
        // Command 1 builds the local record with a shift of 1 from the bytes (1, 0, 0): origin 2
        // on the first axis. 0x20000 >> 15 is 4, so the record holds 4 - 2 + 1 = 3 (word 0) and
        // 4 - 2 = 2 (word 4). Command 0x26 then tests them.
        let build = [0x01, 1, 0, 0];
        let mut code = build.to_vec();
        code.extend_from_slice(&[0x26, 2, 3, 0x31]);
        let mut r = rig(&code);
        r.e.mem.set_u32(r.this + 0x10, 0x2_0000);
        r.e.mem.set_u32(r.this + 0x20, 0x2_0000);
        r.run();
        assert_eq!(r.ids(), vec![1]);
        // A first-byte bound of 4 is above the local lower bound of 3: nothing is appended.
        let mut code = build.to_vec();
        code.extend_from_slice(&[0x26, 4, 3, 0x31]);
        let mut r = rig(&code);
        r.e.mem.set_u32(r.this + 0x10, 0x2_0000);
        r.e.mem.set_u32(r.this + 0x20, 0x2_0000);
        r.run();
        assert!(r.ids().is_empty());
    }

    #[test]
    fn an_unknown_command_reports_and_continues_on_the_same_byte() {
        unknown_command(0);
    }

    #[test]
    #[should_panic(expected = "00dd1400: the exe's INT3 trap")]
    fn an_unknown_command_traps_with_int3_when_the_reporter_asks() {
        unknown_command(1);
    }

    fn unknown_command(wanted: u32) {
        let mut r = rig(&[0x08]);
        let code = r.code;
        let sink = Rc::clone(&r.log);
        r.e.register_double(ERR_STREAM_CONSTRUCT, move |_, a| {
            sink.borrow_mut().push((ERR_STREAM_CONSTRUCT, a.to_vec()));
            Ret::default()
        });
        let sink = Rc::clone(&r.log);
        r.e.register_double(OSTREAM_WRITE_TEXT, move |_, a| {
            sink.borrow_mut().push((OSTREAM_WRITE_TEXT, a.to_vec()));
            Ret::default()
        });
        let sink = Rc::clone(&r.log);
        r.e.register_double(OSTREAM_DESTRUCT, move |_, a| {
            sink.borrow_mut().push((OSTREAM_DESTRUCT, a.to_vec()));
            Ret::default()
        });
        let sink = Rc::clone(&r.log);
        r.e.register_double(0x0900_000c, move |e, a| {
            sink.borrow_mut().push((0x0900_000c, a.to_vec()));
            // The handler "repairs" the byte so the next pass ends the query.
            e.mem.set_u8(code, 0);
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
        r.run();
        let log = r.log.borrow();
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
                vec![reporter, 3, 0x1298_fedd, stream + 0xc, ERROR_FILE, 0x1cb]
            )
        );
        assert_eq!(log[3], (OSTREAM_DESTRUCT, vec![stream]));
    }

    fn query_rig(first_code_byte: u8) -> (Engine, u32, u32) {
        let mut e = test_engine();
        e.mem.set_u8(LICENSE_FLAG, 1);
        let code = e.mem.alloc(0x40);
        e.mem.write(code, &[first_code_byte, 0, 0, 0, 0, 0]);
        let mopp = e.mem.alloc(0x40);
        e.mem.set_u32(mopp + 0x20, code);
        e.mem.set_f32(mopp + 0x1c, 2.0);
        let this = e.mem.alloc(0x40);
        (e, this, mopp)
    }

    #[test]
    fn query_aabb_scales_the_box_into_integer_bounds() {
        let (mut e, this, mopp) = query_rig(0x0d);
        e.mem.set_f32(mopp + 0x1c, 0.5);
        for (i, v) in [1.0f32, 2.0, 3.0].into_iter().enumerate() {
            e.mem.set_f32(mopp + 0x10 + 4 * i as u32, v);
        }
        let bounds = e.mem.alloc(0x20);
        for (i, v) in [3.0f32, 4.0, 5.0].into_iter().enumerate() {
            e.mem.set_f32(bounds + 4 * i as u32, v);
        }
        for (i, v) in [7.0f32, 8.0, 9.0].into_iter().enumerate() {
            e.mem.set_f32(bounds + 0x10 + 4 * i as u32, v);
        }
        let output = e.mem.alloc(0x20);
        e.call(0x00dd_1da0, &args![this, mopp, bounds, output]);
        assert_eq!(e.mem.u32(this), output);
        assert_eq!(e.mem.u32(this + 0x30), mopp);
        for i in 0..3 {
            assert_eq!(e.mem.u32(this + 0x20 + 4 * i), 0);
            assert_eq!(e.mem.u32(this + 0x10 + 4 * i), 4);
        }
        // The first command byte is 0x0d, so the mask is all ones.
        assert_eq!(e.mem.u32(this + 0x34), 0xffff_ffff);
    }

    #[test]
    fn query_obb_bounds_the_oriented_box() {
        let (mut e, this, mopp) = query_rig(0x00);
        e.mem.set_u32(ABS_MASK, 0x7fff_ffff);
        e.mem.set_u32(ABS_MASK + 4, 0x7fff_ffff);
        e.mem.set_u32(ABS_MASK + 8, 0x7fff_ffff);
        e.mem.set_u32(ABS_MASK + 12, 0x7fff_ffff);
        for i in 0..4 {
            e.mem.set_u32(SIGN_MASK + 4 * i, 0x8000_0000);
        }
        for (i, v) in [1.0f32, 2.0, 3.0].into_iter().enumerate() {
            e.mem.set_f32(mopp + 0x10 + 4 * i as u32, v);
        }
        let matrix = e.mem.alloc(0x40);
        for row in 0..3u32 {
            e.mem.set_f32(matrix + 0x10 * row + 4 * row, 1.0);
        }
        for (i, v) in [10.0f32, 20.0, 30.0].into_iter().enumerate() {
            e.mem.set_f32(matrix + 0x30 + 4 * i as u32, v);
        }
        let vector = e.mem.alloc(0x10);
        for i in 0..3 {
            e.mem.set_f32(vector + 4 * i, 1.0);
        }
        let output = e.mem.alloc(0x20);
        e.call(
            0x00dd_1bf0,
            &args![this, mopp, matrix, vector, 0.5f32, output],
        );
        assert_eq!(e.mem.u32(this), output);
        assert_eq!(e.mem.u32(this + 0x34), 0);
        // half = (1.5, 1.5, 1.5); centre = (9, 18, 27); scale 2.
        for (i, (low, high)) in [(14, 22), (32, 40), (50, 58)].into_iter().enumerate() {
            assert_eq!(e.mem.u32(this + 0x20 + 4 * i as u32), low);
            assert_eq!(e.mem.u32(this + 0x10 + 4 * i as u32), high);
        }
    }

    #[test]
    fn fistp_rounds_to_nearest_even() {
        assert_eq!(fistp(2.5), 2);
        assert_eq!(fistp(3.5), 4);
        assert_eq!(fistp(-2.5), -2);
        assert_eq!(fistp(f32::NAN), i32::MIN);
        assert_eq!(fistp(3.0e10), i32::MIN);
    }
}

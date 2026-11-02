//! `Havok SDK/hkpmoppkdopgeometriesvirtualmachine.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkpMoppKDopGeometriesVirtualMachine` (field names describe what the code does and are
//! unconfirmed): a float scale at +0x10; the array of hit ids at +0x34 (data, size, capacity
//! and flags); the current id at +0x30 (-1 when none); the write pointer into the stack of
//! saved k-dops at +0x40; the current k-dop at +0x44, thirteen pairs of floats (lower bound
//! at +0x44 + 8 * axis, upper bound at +0x48 + 8 * axis, the last ten axes being sums and
//! differences of the first three); the recursion depth at +0xAC, a flag at +0xB0, a limit at
//! +0xB4, a flag at +0xB8 with the id it applies to at +0xBC and the stop flag at +0xC0.

#[allow(unused_imports)]
use crate::prelude::*;

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
/// and the source file string `.\Collide\Mopp\Machine\hkpMoppKDopGeometriesVirtualMachine.cpp`
/// (`010ed034`).
const CORRUPTED_TEXT: u32 = 0x010d_5100;
const ERROR_FILE: u32 = 0x010e_d034;
/// Size of one saved k-dop: thirteen pairs of floats, a word, two bytes and padding.
const SAVED_K_DOP_SIZE: u32 = 0x70;

fn byte(e: &Engine, at: u32) -> u32 {
    u32::from(e.mem.u8(at))
}

/// A big-endian value of `count` bytes starting at `at`.
fn big_endian(e: &Engine, at: u32, count: u32) -> u32 {
    (0..count).fold(0u32, |acc, i| (acc << 8).wrapping_add(byte(e, at + i)))
}

/// Copies the 0x18-byte record at `from` over the one at `to` (`REP MOVSD`, six words).
fn copy_record(e: &mut Engine, from: u32, to: u32) {
    for i in 0..6 {
        let word = e.mem.u32(from + 4 * i);
        e.mem.set_u32(to + 4 * i, word);
    }
}

/// The scale at `this + 0x10` as the x87 multiplies by it.
fn scale(e: &Engine, this: u32) -> f64 {
    f64::from(x87_quiet(e.mem.f32(this + 0x10)))
}

// Translated from 00e65040 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMoppKDopGeometriesVirtualMachine::pushKDop` (Xbox PDB), thiscall `(this, has_value,
/// value)`, `RET 8`: copies the current k-dop (0x68 bytes at `this + 0x44`) to the stack of
/// saved k-dops at the pointer in `this + 0x40`, followed by `value` (only when the low byte
/// of `has_value` is not zero), the recursion depth byte and `has_value`'s low byte, and
/// advances the pointer by 0x70.
pub fn hkp_mopp_k_dop_geometries_virtual_machine_push_k_dop(
    e: &mut Engine,
    this: Ptr,
    has_value: u8,
    value: u32,
) {
    let t = this.addr();
    let saved = e.mem.u32(t + 0x40);
    e.mem.set_u8(saved + 0x6d, has_value);
    if has_value != 0 {
        e.mem.set_u32(saved + 0x68, value);
    }
    let depth = e.mem.u8(t + 0xac);
    e.mem.set_u8(saved + 0x6c, depth);
    for word in 0..0x1a {
        let bits = e.mem.u32(t + 0x44 + 4 * word);
        e.mem.set_u32(saved + 4 * word, bits);
    }
    e.mem
        .set_u32(t + 0x40, saved.wrapping_add(SAVED_K_DOP_SIZE));
}

// Translated from 00e65080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMoppKDopGeometriesVirtualMachine::addHit` (Xbox PDB), thiscall `(this, hit, _)`, `RET 8`
/// (the second word is not used): when `hit` is not in the array at `this + 0x34` yet and
/// either the byte at +0xB8 is zero or `hit` is the id at +0xBC, saves the current k-dop
/// (`pushKDop`) and sets the byte at +0xB0 if it is set or `hit` is the id at +0xBC; a new
/// hit is then appended (growing the array when full). Whenever the byte at +0xB0 is set at
/// the end (found or new), the stop flag at +0xC0 is set.
pub fn hkp_mopp_k_dop_geometries_virtual_machine_add_hit(
    e: &mut Engine,
    this: Ptr,
    hit: u32,
    _unused_0: u32,
) {
    let t = this.addr();
    let array = t + 0x34;
    let count = e.mem.u32(t + 0x38) as i32;
    let data = e.mem.u32(array);
    let found = (0..count).any(|i| e.mem.u32(data.wrapping_add(4 * i as u32)) == hit);
    if !found {
        let restricted = e.mem.u8(t + 0xb8) != 0;
        let is_the_one = restricted && hit == e.mem.u32(t + 0xbc);
        if !restricted || is_the_one {
            hkp_mopp_k_dop_geometries_virtual_machine_push_k_dop(e, this, 1, hit);
            let set = e.mem.u8(t + 0xb0) != 0 || is_the_one;
            e.mem.set_u8(t + 0xb0, u8::from(set));
        }
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
        if e.mem.u32(array + 4) == capacity {
            let allocator = e.mem.u32(router + 0x10);
            e.call(ARRAY_RESERVE_MORE, &args![allocator, array, 4u32]);
        }
        let size = e.mem.u32(array + 4);
        let data = e.mem.u32(array);
        e.mem.set_u32(data.wrapping_add(size.wrapping_mul(4)), hit);
        e.mem.set_u32(array + 4, size.wrapping_add(1));
    }
    if e.mem.u8(t + 0xb0) != 0 {
        e.mem.set_u8(t + 0xc0, 1);
    }
}

/// The second operand of the combined-axis commands 0x13 to 0x1c: the command's byte
/// doubled (0x13, 0x15, 0x17), doubled less 255 (0x14, 0x16, 0x18), tripled (0x19) or
/// tripled after subtracting 0x55 (0x1a, 0x1b) or 0xAA (0x1c), as the exe computes them.
fn combined_axis_operand(op: u32, value: i32) -> i32 {
    match op {
        0x13 | 0x15 | 0x17 => value.wrapping_mul(2),
        0x14 | 0x16 | 0x18 => value.wrapping_mul(2).wrapping_sub(255),
        0x19 => value.wrapping_mul(3),
        0x1a | 0x1b => value.wrapping_sub(0x55).wrapping_mul(3),
        _ => value.wrapping_sub(0xaa).wrapping_mul(3),
    }
}

/// The end of the plane commands 0x10 to 0x25: queries the two halves of the tree. The first
/// (at `code + near`) runs with the upper bound of `axis` set to `upper`, the second (at
/// `code + far`) with the lower bound set to `lower`; the bounds are put back after each.
fn query_both_halves(
    e: &mut Engine,
    this: Ptr,
    state: u32,
    code: u32,
    axis: u32,
    (near, far): (u32, u32),
    (upper, lower): (f32, f32),
) {
    let t = this.addr();
    let upper_slot = t + 0x48 + axis * 8;
    let lower_slot = t + 0x44 + axis * 8;
    let first = code.wrapping_add(near);
    let saved_upper = x87_quiet_bits(e.mem.u32(upper_slot));
    e.mem.set_u32(upper_slot, upper.to_bits());
    hkp_mopp_k_dop_geometries_virtual_machine_query_mopp_k_dop_geometries_recurse(
        e,
        this,
        Ptr::new(state),
        Ptr::new(first),
    );
    e.mem.set_u32(upper_slot, x87_quiet_bits(saved_upper));
    let second = first.wrapping_add(far.wrapping_sub(near));
    let saved_lower = x87_quiet_bits(e.mem.u32(lower_slot));
    e.mem.set_u32(lower_slot, lower.to_bits());
    hkp_mopp_k_dop_geometries_virtual_machine_query_mopp_k_dop_geometries_recurse(
        e,
        this,
        Ptr::new(state),
        Ptr::new(second),
    );
    e.mem.set_u32(lower_slot, x87_quiet_bits(saved_lower));
}

// Translated from 00e65140 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMoppKDopGeometriesVirtualMachine::queryMoppKDopGeometriesRecurse` (Xbox PDB), thiscall
/// `(this, state, code)`, `RET 8`: interprets the command bytes of a MOPP tree at `code`,
/// narrowing the current k-dop at `this + 0x44` as it goes and reporting the ids it reaches
/// through `addHit`. Returns at once, and stops, when the stop flag at `this + 0xC0` is set.
///
/// `state` is a record of six words: `+0/+4/+8` the cell origin, `+0xC` the number of
/// bits the cell has been narrowed by, `+0x10` an id offset and `+0x14` a word nothing
/// reads (it is passed on to `addHit`, which ignores it). The interpreter keeps a local
/// record of its own; a command that changes the state copies the record there first (and
/// recursive calls get a pointer to either record). Commands (the byte tables `00e65d24`
/// and `00e65cac`): 0 to 4 build the local record narrowed by that many bits from the next
/// three bytes; 5 to 7 skip forward by a 1, 2 or 3 byte offset; 9 to 0xB add to or set the
/// id offset; 0x10 to 0x25 are plane commands: bounds are computed from two bytes, the
/// state and the scale and the two halves of the tree behind the command are queried with
/// the upper bound and then the lower bound of the plane's axis replaced; 0x26 to 0x2B
/// replace both bounds of an axis for one query of the code behind them (saving the k-dop
/// first, unless the depth limit at +0xB4 is reached); 0x30 to 0x52 report an id (plus the
/// id offset) and return; 0x60 to 0x6B store a value in a slot of the frame beside the
/// local record and make the local record the current one; any other command reports the
/// "Unknown command" error (`hkErrStream`, `00cb1600`, the reporter's virtual function at
/// 0xC, then `INT3`, which this translation panics on: it is a breakpoint trap in the exe,
/// not a call) and continues with the same byte. The command meanings beyond this are
/// unconfirmed.
///
/// x87 arithmetic follows the exe: integers are converted exactly, sums and products are
/// computed in double precision and rounded to float only where the exe stores a float.
///
/// Uninitialised stack words (checked against the disassembly): the local record is built
/// whole by command 0 to 4 or copied whole before any read of it. Commands 0x60 to 0x6B write
/// slots of the frame; slot 0 is the record's last word, which is only ever copied on
/// (and `addHit` ignores it); a store to another slot keeps the local record's (possibly
/// never written) last word over the copy, which reads as zero here and has no effect on
/// any result. The other frame words the exe reads (the bounds, skip and axis it computes
/// before the plane commands' common tail) are always written first.
pub fn hkp_mopp_k_dop_geometries_virtual_machine_query_mopp_k_dop_geometries_recurse(
    e: &mut Engine,
    this: Ptr,
    state: Ptr,
    code: Ptr,
) {
    let t = this.addr();
    e.with_stack(0x230, |e, frame| {
        // The frame: the local record at +0, the slots of commands 0x60 to 0x6B from +0x14,
        // the error stream at +0x18 and its 0x200-byte buffer behind it.
        let local = frame.addr();
        let mut s = state.addr();
        let mut c = code.addr();
        loop {
            if e.mem.u8(t + 0xc0) != 0 {
                return;
            }
            let op = byte(e, c);
            match op {
                0..=4 => {
                    let shift = 0x10u32.wrapping_sub(e.mem.u32(s + 0xc)) & 31;
                    let words = [0, 4, 8, 0xc, 0x10, 0x14].map(|offset| e.mem.u32(s + offset));
                    for axis in 0..3u32 {
                        let value =
                            (byte(e, c + 1 + axis) << shift).wrapping_add(words[axis as usize]);
                        e.mem.set_u32(local + 4 * axis, value);
                    }
                    e.mem.set_u32(local + 0xc, words[3].wrapping_add(op));
                    e.mem.set_u32(local + 0x10, words[4]);
                    e.mem.set_u32(local + 0x14, words[5]);
                    s = local;
                    c += 4;
                }
                5 => c = c.wrapping_add(byte(e, c + 1) + 2),
                6 => c = c.wrapping_add(big_endian(e, c + 1, 2) + 3),
                7 => c = c.wrapping_add(big_endian(e, c + 1, 3) + 4),
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
                        e.mem.u32(local + 0x10).wrapping_add(amount)
                    };
                    e.mem.set_u32(local + 0x10, id_offset);
                    c += length;
                }
                0x10..=0x12 => {
                    let axis = op - 0x10;
                    let shift = 0x10u32.wrapping_sub(e.mem.u32(s + 0xc)) & 31;
                    let base = e.mem.u32(s + 4 * axis);
                    let factor = scale(e, t);
                    let upper = (byte(e, c + 1) << shift).wrapping_add(base);
                    let lower = (byte(e, c + 2) << shift).wrapping_add(base);
                    let upper = (f64::from(upper as i32) * factor) as f32;
                    let lower = (f64::from(lower as i32) * factor) as f32;
                    let skip = byte(e, c + 3);
                    query_both_halves(e, this, s, c + 4, axis, (0, skip), (upper, lower));
                    return;
                }
                0x13..=0x1c => {
                    let axis = op - 0x10;
                    let shift = 0x10u32.wrapping_sub(e.mem.u32(s + 0xc)) & 31;
                    let [x, y, z] = [0, 4, 8].map(|offset| e.mem.u32(s + offset) as i32);
                    let sum = match op {
                        0x13 => z.wrapping_add(y),
                        0x14 => y.wrapping_sub(z),
                        0x15 => z.wrapping_add(x),
                        0x16 => x.wrapping_sub(z),
                        0x17 => y.wrapping_add(x),
                        0x18 => x.wrapping_sub(y),
                        0x19 => z.wrapping_add(y).wrapping_add(x),
                        0x1a => y.wrapping_sub(z).wrapping_add(x),
                        0x1b => z.wrapping_sub(y).wrapping_add(x),
                        _ => x.wrapping_sub(z).wrapping_sub(y),
                    };
                    let upper = combined_axis_operand(op, byte(e, c + 1) as i32);
                    let lower = combined_axis_operand(op, byte(e, c + 2) as i32);
                    let upper = f64::from(upper.wrapping_shl(shift));
                    let lower = f64::from(lower.wrapping_shl(shift));
                    let factor = scale(e, t);
                    let sum = sum as f32;
                    let upper = (upper + f64::from(sum)) as f32;
                    let lower = (f64::from(sum) + lower) as f32;
                    let upper = (f64::from(upper) * factor) as f32;
                    let lower = (f64::from(lower) * factor) as f32;
                    let skip = byte(e, c + 3);
                    query_both_halves(e, this, s, c + 4, axis, (0, skip), (upper, lower));
                    return;
                }
                0x20..=0x22 => {
                    let axis = op - 0x20;
                    let shift = 0x10u32.wrapping_sub(e.mem.u32(s + 0xc)) & 31;
                    let base = e.mem.u32(s + 4 * axis);
                    let position = ((byte(e, c + 1) << shift).wrapping_add(base) as i32) as f32;
                    let cell = f64::from(1u32.wrapping_shl(shift) as i32);
                    let factor = scale(e, t);
                    let cell_end = (cell + f64::from(position)) as f32;
                    let lower = (f64::from(position) * factor) as f32;
                    let upper = (factor * f64::from(cell_end)) as f32;
                    let skip = byte(e, c + 2);
                    query_both_halves(e, this, s, c + 3, axis, (0, skip), (upper, lower));
                    return;
                }
                0x23..=0x25 => {
                    let axis = op - 0x23;
                    let shift = 0x10u32.wrapping_sub(e.mem.u32(s + 0xc)) & 31;
                    let base = e.mem.u32(s + 4 * axis);
                    let factor = scale(e, t);
                    let upper = (byte(e, c + 1) << shift).wrapping_add(base);
                    let lower = (byte(e, c + 2) << shift).wrapping_add(base);
                    let upper = (f64::from(upper as i32) * factor) as f32;
                    let lower = (f64::from(lower as i32) * factor) as f32;
                    let near = big_endian(e, c + 3, 2);
                    let far = big_endian(e, c + 5, 2);
                    query_both_halves(e, this, s, c + 7, axis, (near, far), (upper, lower));
                    return;
                }
                0x26..=0x2b => {
                    let (axis, lower, upper, length) = if op <= 0x28 {
                        let axis = op - 0x26;
                        let shift = 0x10u32.wrapping_sub(e.mem.u32(s + 0xc)) & 31;
                        let base = e.mem.u32(s + 4 * axis);
                        let factor = scale(e, t);
                        let lower = (byte(e, c + 1) << shift).wrapping_add(base);
                        let upper = (byte(e, c + 2) << shift).wrapping_add(base);
                        let lower = (f64::from(lower as i32) * factor) as f32;
                        let upper = (f64::from(upper as i32) * factor) as f32;
                        (axis, lower, upper, 3)
                    } else {
                        let factor = scale(e, t);
                        let lower = (f64::from(big_endian(e, c + 1, 3) as i32) * factor) as f32;
                        let upper = (f64::from(big_endian(e, c + 4, 3) as i32) * factor) as f32;
                        (op - 0x29, lower, upper, 7)
                    };
                    let limit = e.mem.u32(t + 0xb4) as i32;
                    let pushed = limit == 0 || (e.mem.u32(t + 0xac) as i32) < limit;
                    if pushed {
                        hkp_mopp_k_dop_geometries_virtual_machine_push_k_dop(
                            e,
                            this,
                            0,
                            0xffff_ffff,
                        );
                    }
                    let lower_slot = t + 0x44 + axis * 8;
                    let upper_slot = t + 0x48 + axis * 8;
                    let saved_lower = x87_quiet_bits(e.mem.u32(lower_slot));
                    let saved_upper = x87_quiet_bits(e.mem.u32(upper_slot));
                    e.mem.set_u32(lower_slot, lower.to_bits());
                    e.mem.set_u32(upper_slot, upper.to_bits());
                    let depth = e.mem.u32(t + 0xac);
                    e.mem.set_u32(t + 0xac, depth.wrapping_add(1));
                    hkp_mopp_k_dop_geometries_virtual_machine_query_mopp_k_dop_geometries_recurse(
                        e,
                        this,
                        Ptr::new(s),
                        Ptr::new(c + length),
                    );
                    let depth = e.mem.u32(t + 0xac);
                    e.mem.set_u32(t + 0xac, depth.wrapping_sub(1));
                    // The byte ranges pop the saved k-dop unless the query was stopped; the
                    // 24-bit ranges pop it whenever the flag at +0xB8 is set.
                    let popped =
                        pushed && e.mem.u8(t + 0xb8) != 0 && (op > 0x28 || e.mem.u8(t + 0xc0) == 0);
                    if popped {
                        let top = e.mem.u32(t + 0x40);
                        e.mem.set_u32(t + 0x40, top.wrapping_sub(SAVED_K_DOP_SIZE));
                    }
                    e.mem.set_u32(lower_slot, saved_lower);
                    e.mem.set_u32(upper_slot, saved_upper);
                    return;
                }
                0x30..=0x52 => {
                    let id = match op {
                        0x30..=0x4f => op - 0x30,
                        0x50 => byte(e, c + 1),
                        0x51 => big_endian(e, c + 1, 2),
                        _ => big_endian(e, c + 1, 3),
                    };
                    e.mem.set_u32(t + 0x30, id);
                    let id_offset = e.mem.u32(s + 0x10);
                    let hit = id.wrapping_add(id_offset);
                    e.mem.set_u32(t + 0x30, hit);
                    hkp_mopp_k_dop_geometries_virtual_machine_add_hit(e, this, hit, s + 0x14);
                    e.mem.set_u32(t + 0x30, 0xffff_ffff);
                    return;
                }
                0x60..=0x6b => {
                    let (value, length) = match op {
                        0x60..=0x63 => (byte(e, c + 1), 2),
                        0x64..=0x67 => (big_endian(e, c + 1, 2), 3),
                        _ => (big_endian(e, c + 1, 4), 5),
                    };
                    e.mem.set_u32(local + 0x14 + 4 * (op & 3), value);
                    c += length;
                    // The exe keeps the record's last word in a register across the copy.
                    let kept = e.mem.u32(local + 0x14);
                    if s != local {
                        copy_record(e, s, local);
                        s = local;
                    }
                    e.mem.set_u32(local + 0x14, kept);
                }
                _ => {
                    let stream = local + 0x18;
                    let buffer = stream + 0xc;
                    e.call(ERR_STREAM_CONSTRUCT, &args![stream, buffer, 0x200u32]);
                    e.call(OSTREAM_WRITE_TEXT, &args![stream, CORRUPTED_TEXT]);
                    let reporter: u32 = e.global(ERROR_REPORTER);
                    let wanted_break = e
                        .vcall(
                            reporter,
                            0xc,
                            &args![3u32, 0x1298_feddu32, buffer, ERROR_FILE, 0x251u32],
                        )
                        .u32();
                    if wanted_break != 0 {
                        panic!(
                            "00e65140: the exe's INT3 trap (the error reporter asked for a break)"
                        );
                    }
                    e.call(OSTREAM_DESTRUCT, &args![stream]);
                }
            }
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00e6_5040,
            hkp_mopp_k_dop_geometries_virtual_machine_push_k_dop(Ptr, u8, u32)
        ),
        entry!(
            0x00e6_5080,
            hkp_mopp_k_dop_geometries_virtual_machine_add_hit(Ptr, u32, u32)
        ),
        entry!(
            0x00e6_5140,
            hkp_mopp_k_dop_geometries_virtual_machine_query_mopp_k_dop_geometries_recurse(
                Ptr, Ptr, Ptr
            )
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    struct Rig {
        e: Engine,
        log: Log,
        this: u32,
        state: u32,
        code: u32,
        kdops: u32,
        array: u32,
    }

    /// The scale of the machine and the value every bound starts with.
    const SCALE: f32 = 0.5;
    const OPEN_LOWER: f32 = -1000.0;
    const OPEN_UPPER: f32 = 1000.0;

    /// A machine with the scale 0.5, every pair of bounds open (-1000, 1000), a stack of
    /// saved k-dops, a hit array (four words of capacity) and a state record of zeros.
    fn rig() -> Rig {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
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
            let capacity = e.mem.u32(a[1] + 8);
            e.mem.set_u32(a[1] + 8, capacity + 4);
            Ret::default()
        });
        let code = e.mem.alloc(0x400);
        let kdops = e.mem.alloc(SAVED_K_DOP_SIZE * 16);
        let data = e.mem.alloc(0x400);
        let this = e.mem.alloc(0x100);
        let state = e.mem.alloc(0x20);
        e.mem.set_f32(this + 0x10, SCALE);
        e.mem.set_u32(this + 0x34, data);
        e.mem.set_u32(this + 0x3c, 4);
        let array = this + 0x34;
        let mut rig = Rig {
            e,
            log,
            this,
            state,
            code,
            kdops,
            array,
        };
        rig.reset();
        rig
    }

    impl Rig {
        /// Opens every bound and empties the hit array and the saved k-dops.
        fn reset(&mut self) {
            let t = self.this;
            for axis in 0..13 {
                self.e.mem.set_f32(t + 0x44 + 8 * axis, OPEN_LOWER);
                self.e.mem.set_f32(t + 0x48 + 8 * axis, OPEN_UPPER);
            }
            self.e.mem.set_u32(t + 0x40, self.kdops);
            self.e.mem.set_u32(t + 0x38, 0);
            self.e.mem.set_u32(t + 0x30, 0xffff_ffff);
            for offset in [0xac, 0xb4, 0xb8, 0xbc, 0xc0] {
                self.e.mem.set_u32(t + offset, 0);
            }
            self.e.mem.set_u8(t + 0xb0, 0);
        }

        /// The state words: the origin, the shift (16 minus the narrowing bits), the id
        /// offset.
        fn set_state(&mut self, words: &[u32]) {
            for (i, &word) in words.iter().enumerate() {
                self.e.mem.set_u32(self.state + 4 * i as u32, word);
            }
        }

        /// Runs `bytes` as the command stream; returns the hit ids.
        fn run(&mut self, bytes: &[u8]) -> Vec<u32> {
            self.e.mem.write(self.code, bytes);
            let (t, s, c) = (self.this, self.state, self.code);
            self.e.call(0x00e6_5140, &args![t, s, c]);
            self.hits()
        }

        fn hits(&self) -> Vec<u32> {
            let data = self.e.mem.u32(self.array);
            (0..self.e.mem.u32(self.array + 4))
                .map(|i| self.e.mem.u32(data + 4 * i))
                .collect()
        }

        /// The pair of bounds of `axis` in the `index`th saved k-dop.
        fn saved_pair(&self, index: u32, axis: u32) -> (f32, f32) {
            let at = self.kdops + SAVED_K_DOP_SIZE * index + 8 * axis;
            (self.e.mem.f32(at), self.e.mem.f32(at + 4))
        }

        /// The number of saved k-dops.
        fn saved_count(&self) -> u32 {
            (self.e.mem.u32(self.this + 0x40) - self.kdops) / SAVED_K_DOP_SIZE
        }
    }

    #[test]
    fn push_k_dop_copies_the_current_k_dop_and_advances() {
        let mut r = rig();
        let t = r.this;
        for word in 0..0x1a {
            r.e.mem.set_u32(t + 0x44 + 4 * word, 0x1000 + word);
        }
        r.e.mem.set_u8(t + 0xac, 9);
        r.e.mem.set_u32(r.kdops + 0x70 + 0x68, 0x7777_7777);
        r.e.call(0x00e6_5040, &args![t, 1u8, 0xabcdu32]);
        for word in 0..0x1a {
            assert_eq!(r.e.mem.u32(r.kdops + 4 * word), 0x1000 + word);
        }
        assert_eq!(r.e.mem.u32(r.kdops + 0x68), 0xabcd);
        assert_eq!(r.e.mem.u8(r.kdops + 0x6c), 9);
        assert_eq!(r.e.mem.u8(r.kdops + 0x6d), 1);
        assert_eq!(r.e.mem.u32(t + 0x40), r.kdops + 0x70);
        // Without the flag the value word stays as it was.
        r.e.call(0x00e6_5040, &args![t, 0u8, 0x5555u32]);
        assert_eq!(r.e.mem.u32(r.kdops + 0x70 + 0x68), 0x7777_7777);
        assert_eq!(r.e.mem.u8(r.kdops + 0x70 + 0x6d), 0);
        assert_eq!(r.e.mem.u32(t + 0x40), r.kdops + 0xe0);
    }

    #[test]
    fn add_hit_appends_new_hits_and_saves_the_k_dop() {
        let mut r = rig();
        let t = r.this;
        r.e.call(0x00e6_5080, &args![t, 7u32, 0u32]);
        r.e.call(0x00e6_5080, &args![t, 9u32, 0u32]);
        r.e.call(0x00e6_5080, &args![t, 7u32, 0u32]);
        assert_eq!(r.hits(), [7, 9]);
        assert_eq!(r.saved_count(), 2);
        assert_eq!(r.e.mem.u32(r.kdops + 0x68), 7);
        assert_eq!(r.e.mem.u32(r.kdops + 0x70 + 0x68), 9);
        assert_eq!(r.e.mem.u8(t + 0xb0), 0);
        assert_eq!(r.e.mem.u8(t + 0xc0), 0);
    }

    #[test]
    fn add_hit_with_a_restriction_saves_only_the_wanted_hit_and_stops() {
        let mut r = rig();
        let t = r.this;
        r.e.mem.set_u8(t + 0xb8, 1);
        r.e.mem.set_u32(t + 0xbc, 9);
        r.e.call(0x00e6_5080, &args![t, 7u32, 0u32]);
        assert_eq!(r.hits(), [7]);
        assert_eq!(r.saved_count(), 0);
        assert_eq!(r.e.mem.u8(t + 0xc0), 0);
        r.e.call(0x00e6_5080, &args![t, 9u32, 0u32]);
        assert_eq!(r.hits(), [7, 9]);
        assert_eq!(r.saved_count(), 1);
        assert_eq!(r.e.mem.u8(t + 0xb0), 1);
        assert_eq!(r.e.mem.u8(t + 0xc0), 1);
    }

    #[test]
    fn add_hit_of_a_known_hit_still_stops_when_the_flag_is_set() {
        let mut r = rig();
        let t = r.this;
        r.e.call(0x00e6_5080, &args![t, 7u32, 0u32]);
        r.e.mem.set_u8(t + 0xb0, 1);
        r.e.call(0x00e6_5080, &args![t, 7u32, 0u32]);
        assert_eq!(r.hits(), [7]);
        assert_eq!(r.saved_count(), 1);
        assert_eq!(r.e.mem.u8(t + 0xc0), 1);
    }

    #[test]
    fn add_hit_grows_a_full_array_first() {
        let mut r = rig();
        let t = r.this;
        for hit in 1..=5u32 {
            r.e.call(0x00e6_5080, &args![t, hit, 0u32]);
        }
        assert_eq!(r.hits(), [1, 2, 3, 4, 5]);
        let log = r.log.borrow();
        assert_eq!(log.len(), 1);
        assert_eq!(log[0], (ARRAY_RESERVE_MORE, vec![0xa110c, r.array, 4]));
    }

    #[test]
    fn id_commands_report_the_id_plus_the_offset() {
        let mut r = rig();
        r.set_state(&[0, 0, 0, 16, 100]);
        assert_eq!(r.run(&[0x30]), [100]);
        r.reset();
        assert_eq!(r.run(&[0x4f]), [131]);
        r.reset();
        assert_eq!(r.run(&[0x50, 0xfe]), [354]);
        r.reset();
        assert_eq!(r.run(&[0x51, 0x01, 0x02]), [100 + 0x102]);
        r.reset();
        assert_eq!(r.run(&[0x52, 0x01, 0x02, 0x03]), [100 + 0x01_0203]);
        assert_eq!(r.e.mem.u32(r.this + 0x30), 0xffff_ffff);
    }

    #[test]
    fn skip_commands_move_the_pointer() {
        let mut r = rig();
        assert_eq!(r.run(&[0x05, 0x01, 0xaa, 0x32]), [2]);
        r.reset();
        assert_eq!(r.run(&[0x06, 0x00, 0x02, 0xaa, 0xaa, 0x33]), [3]);
        r.reset();
        assert_eq!(r.run(&[0x07, 0x00, 0x00, 0x01, 0xaa, 0x34]), [4]);
    }

    #[test]
    fn the_stop_flag_ends_the_query_at_once() {
        let mut r = rig();
        r.e.mem.set_u8(r.this + 0xc0, 1);
        assert!(r.run(&[0x31]).is_empty());
    }

    #[test]
    fn id_offset_commands_work_on_a_copy_of_the_state() {
        let mut r = rig();
        r.set_state(&[0, 0, 0, 16, 10]);
        assert_eq!(r.run(&[0x09, 0x05, 0x30]), [15]);
        r.reset();
        assert_eq!(r.run(&[0x0a, 0x01, 0x00, 0x30]), [266]);
        r.reset();
        assert_eq!(r.run(&[0x0b, 0x00, 0x01, 0x00, 0x00, 0x30]), [0x1_0000]);
        r.reset();
        assert_eq!(r.run(&[0x09, 0x05, 0x09, 0x07, 0x30]), [22]);
        assert_eq!(r.e.mem.u32(r.state + 0x10), 10);
    }

    #[test]
    fn slot_commands_have_their_lengths_and_keep_the_state() {
        let mut r = rig();
        r.set_state(&[0, 0, 0, 16, 7]);
        for (code, hit) in [
            (&[0x60u8, 0x09, 0x30][..], 7),
            (&[0x63, 0x09, 0x31], 8),
            (&[0x64, 0x01, 0x02, 0x30], 7),
            (&[0x67, 0x01, 0x02, 0x30], 7),
            (&[0x68, 0x01, 0x02, 0x03, 0x04, 0x30], 7),
            (&[0x6b, 0x01, 0x02, 0x03, 0x04, 0x31], 8),
            (&[0x60, 0x00, 0x09, 0x01, 0x30], 8),
        ] {
            r.reset();
            assert_eq!(r.run(code), [hit]);
        }
        assert_eq!(r.e.mem.u32(r.state + 0x10), 7);
    }

    #[test]
    fn narrowing_builds_the_local_cell_for_the_next_commands() {
        // Origin (10, 20, 30) with no bits narrowed (shift 16); narrowing by 2 with bytes
        // (1, 2, 3) gives the origin ((1<<16)+10, (2<<16)+20, (3<<16)+30) and the shift 14. The range command
        // after it with bytes (1, 2) on axis 0 then has the bounds
        // (((1<<14) + 65546) * 0.5, ((2<<14) + 65546) * 0.5) = (40965, 49157).
        let mut r = rig();
        r.set_state(&[10, 20, 30, 0, 5]);
        let hits = r.run(&[0x02, 1, 2, 3, 0x26, 1, 2, 0x30]);
        assert_eq!(hits, [5]);
        assert_eq!(r.saved_pair(1, 0), (40965.0, 49157.0));
        // Axis 1 and axis 2 with the origin words 2<<16 + 20 and 3<<16 + 30.
        r.reset();
        r.run(&[0x02, 1, 2, 3, 0x27, 1, 2, 0x30]);
        assert_eq!(
            r.saved_pair(1, 1),
            (
                ((1 << 14) + (2 << 16) + 20) as f32 * 0.5,
                ((2 << 14) + (2 << 16) + 20) as f32 * 0.5
            )
        );
        r.reset();
        r.run(&[0x02, 1, 2, 3, 0x28, 1, 2, 0x30]);
        assert_eq!(
            r.saved_pair(1, 2),
            (
                ((1 << 14) + (3 << 16) + 30) as f32 * 0.5,
                ((2 << 14) + (3 << 16) + 30) as f32 * 0.5
            )
        );
        // Narrowing again by 1 (bytes 0) makes the shift 13 and keeps the origin x 65546.
        r.reset();
        r.run(&[0x02, 1, 2, 3, 0x01, 0, 0, 0, 0x26, 1, 2, 0x30]);
        assert_eq!(r.saved_pair(1, 0), (36869.0, 40965.0));
    }

    #[test]
    fn plane_commands_on_one_axis_query_both_halves_with_one_bound_replaced() {
        // Origin (100, 230, 300), shift 0; bytes (10, 20); the first half is the one-byte
        // sub-tree `0x31`, the second the one behind it (`0x32`).
        let mut r = rig();
        r.set_state(&[100, 230, 300, 16, 0]);
        let hits = r.run(&[0x11, 10, 20, 1, 0x31, 0x32]);
        assert_eq!(hits, [1, 2]);
        // (10 + 230) * 0.5 and (20 + 230) * 0.5.
        assert_eq!(r.saved_pair(0, 1), (OPEN_LOWER, 120.0));
        assert_eq!(r.saved_pair(1, 1), (125.0, OPEN_UPPER));
        // Every other axis stays open and the bounds are put back afterwards.
        assert_eq!(r.saved_pair(0, 0), (OPEN_LOWER, OPEN_UPPER));
        assert_eq!(r.e.mem.f32(r.this + 0x4c), OPEN_LOWER);
        assert_eq!(r.e.mem.f32(r.this + 0x50), OPEN_UPPER);
        // A narrower cell shifts the bytes: shift 2 for 14 bits left.
        r.reset();
        r.set_state(&[100, 230, 300, 14, 0]);
        r.run(&[0x10, 3, 5, 1, 0x31, 0x32]);
        assert_eq!(r.saved_pair(0, 0), (OPEN_LOWER, 56.0));
        assert_eq!(r.saved_pair(1, 0), (60.0, OPEN_UPPER));
    }

    #[test]
    fn plane_commands_on_combined_axes_use_the_combination_as_a_float() {
        // Origin (100, 230, 300), shift 0, bytes (10, 20); the bounds (scale 0.5) of the
        // first half are (sum + upper operand) * 0.5 and of the second (sum + lower
        // operand) * 0.5.
        let cases: [(u8, f32, f32); 10] = [
            (0x13, 550.0, 570.0),
            (0x14, -305.0, -285.0),
            (0x15, 420.0, 440.0),
            (0x16, -435.0, -415.0),
            (0x17, 350.0, 370.0),
            (0x18, -365.0, -345.0),
            (0x19, 660.0, 690.0),
            (0x1a, -195.0, -165.0),
            (0x1b, -55.0, -25.0),
            (0x1c, -910.0, -880.0),
        ];
        for (op, upper, lower) in cases {
            let mut r = rig();
            r.set_state(&[100, 230, 300, 16, 0]);
            let hits = r.run(&[op, 10, 20, 1, 0x31, 0x32]);
            let axis = u32::from(op) - 0x10;
            assert_eq!(hits, [1, 2], "op {op:#x}");
            assert_eq!(
                r.saved_pair(0, axis),
                (OPEN_LOWER, upper * 0.5),
                "op {op:#x}"
            );
            assert_eq!(
                r.saved_pair(1, axis),
                (lower * 0.5, OPEN_UPPER),
                "op {op:#x}"
            );
        }
    }

    #[test]
    fn plane_commands_on_one_axis_with_one_byte_use_the_cell_width() {
        // Shift 2: position (3 << 2) + 100 = 112; the cell is 4 wide.
        let mut r = rig();
        r.set_state(&[100, 230, 300, 14, 0]);
        let hits = r.run(&[0x20, 3, 1, 0x31, 0x32]);
        assert_eq!(hits, [1, 2]);
        assert_eq!(r.saved_pair(0, 0), (OPEN_LOWER, 58.0));
        assert_eq!(r.saved_pair(1, 0), (56.0, OPEN_UPPER));
    }

    #[test]
    fn plane_commands_with_two_jumps_run_the_halves_they_point_to() {
        // `[op, upper, lower, near (2 bytes), far (2 bytes)]` then three one-byte sub-trees:
        // the first half is the one `near` bytes in (`0x32`), the second the one `far`
        // bytes in (`0x33`); the first sub-tree is never run.
        let mut r = rig();
        r.set_state(&[100, 230, 300, 14, 0]);
        let hits = r.run(&[0x23, 3, 5, 0, 1, 0, 2, 0x31, 0x32, 0x33]);
        assert_eq!(hits, [2, 3]);
        assert_eq!(r.saved_pair(0, 0), (OPEN_LOWER, 56.0));
        assert_eq!(r.saved_pair(1, 0), (60.0, OPEN_UPPER));
    }

    #[test]
    fn a_stop_in_the_first_half_skips_the_second() {
        let mut r = rig();
        r.set_state(&[100, 230, 300, 16, 0]);
        r.e.mem.set_u8(r.this + 0xb8, 1);
        r.e.mem.set_u32(r.this + 0xbc, 1);
        let hits = r.run(&[0x10, 10, 20, 1, 0x31, 0x32]);
        assert_eq!(hits, [1]);
        assert_eq!(r.e.mem.u8(r.this + 0xc0), 1);
        // The bound is still put back.
        assert_eq!(r.e.mem.f32(r.this + 0x48), OPEN_UPPER);
    }

    #[test]
    fn range_commands_replace_both_bounds_around_one_query() {
        // Shift 2; the bounds ((3 << 2) + 100) * 0.5 = 56 and ((5 << 2) + 100) * 0.5 = 60.
        let mut r = rig();
        r.set_state(&[100, 230, 300, 14, 0]);
        let hits = r.run(&[0x26, 3, 5, 0x31]);
        assert_eq!(hits, [1]);
        // The first saved k-dop is the one the command pushed before changing the bounds,
        // the second the one the hit pushed.
        assert_eq!(r.saved_count(), 2);
        assert_eq!(r.saved_pair(0, 0), (OPEN_LOWER, OPEN_UPPER));
        assert_eq!(r.e.mem.u32(r.kdops + 0x68), 0);
        assert_eq!(r.e.mem.u8(r.kdops + 0x6d), 0);
        assert_eq!(r.saved_pair(1, 0), (56.0, 60.0));
        assert_eq!(r.e.mem.u8(r.kdops + 0x70 + 0x6c), 1);
        assert_eq!(r.e.mem.f32(r.this + 0x44), OPEN_LOWER);
        assert_eq!(r.e.mem.f32(r.this + 0x48), OPEN_UPPER);
        assert_eq!(r.e.mem.u32(r.this + 0xac), 0);
        // 24-bit bounds: (100 * 0.5, 200 * 0.5).
        r.reset();
        let hits = r.run(&[0x2a, 0, 0, 100, 0, 0, 200, 0x31]);
        assert_eq!(hits, [1]);
        assert_eq!(r.saved_pair(1, 1), (50.0, 100.0));
    }

    #[test]
    fn range_commands_only_save_below_the_depth_limit() {
        let mut r = rig();
        r.set_state(&[100, 230, 300, 16, 0]);
        r.e.mem.set_u32(r.this + 0xb4, 2);
        r.e.mem.set_u32(r.this + 0xac, 2);
        r.run(&[0x26, 3, 5, 0x31]);
        // Only the hit saved a k-dop; the depth went to 3 and back.
        assert_eq!(r.saved_count(), 1);
        assert_eq!(r.e.mem.u8(r.kdops + 0x6c), 3);
        assert_eq!(r.e.mem.u32(r.this + 0xac), 2);
        r.reset();
        r.e.mem.set_u32(r.this + 0xb4, 2);
        r.e.mem.set_u32(r.this + 0xac, 1);
        r.run(&[0x26, 3, 5, 0x31]);
        assert_eq!(r.saved_count(), 2);
    }

    #[test]
    fn range_commands_pop_the_saved_k_dop_when_the_flag_is_set() {
        // With the flag set and a hit that is not the wanted one nothing else is saved, so
        // both kinds of range command pop theirs.
        for code in [&[0x26u8, 3, 5, 0x31][..], &[0x29, 0, 0, 3, 0, 0, 5, 0x31]] {
            let mut r = rig();
            r.set_state(&[100, 230, 300, 16, 0]);
            r.e.mem.set_u8(r.this + 0xb8, 1);
            r.e.mem.set_u32(r.this + 0xbc, 99);
            assert_eq!(r.run(code), [1]);
            assert_eq!(r.saved_count(), 0);
        }
        // With the wanted hit the query is stopped: the byte ranges keep their k-dop, the
        // 24-bit ones pop it anyway.
        let mut r = rig();
        r.e.mem.set_u8(r.this + 0xb8, 1);
        r.e.mem.set_u32(r.this + 0xbc, 1);
        r.set_state(&[100, 230, 300, 16, 0]);
        assert_eq!(r.run(&[0x26, 3, 5, 0x31]), [1]);
        assert_eq!(r.saved_count(), 2);
        let mut r = rig();
        r.e.mem.set_u8(r.this + 0xb8, 1);
        r.e.mem.set_u32(r.this + 0xbc, 1);
        r.set_state(&[100, 230, 300, 16, 0]);
        assert_eq!(r.run(&[0x29, 0, 0, 3, 0, 0, 5, 0x31]), [1]);
        assert_eq!(r.saved_count(), 1);
    }

    #[test]
    fn saved_bounds_are_put_back_as_quiet_nans() {
        let mut r = rig();
        r.set_state(&[100, 230, 300, 16, 0]);
        r.e.mem.set_u32(r.this + 0x44, 0x7f80_0001);
        r.e.mem.set_u32(r.this + 0x48, 0x7f80_0002);
        r.run(&[0x26, 3, 5, 0x31]);
        assert_eq!(r.e.mem.u32(r.this + 0x44), 0x7fc0_0001);
        assert_eq!(r.e.mem.u32(r.this + 0x48), 0x7fc0_0002);
        // The saved k-dop is a plain copy.
        assert_eq!(r.e.mem.u32(r.kdops), 0x7f80_0001);
    }

    #[test]
    fn an_unknown_command_reports_and_continues_on_the_same_byte() {
        unknown_command(0);
    }

    #[test]
    #[should_panic(expected = "00e65140: the exe's INT3 trap")]
    fn an_unknown_command_traps_with_int3_when_the_reporter_asks() {
        unknown_command(1);
    }

    fn unknown_command(wanted: u32) {
        let mut r = rig();
        r.e.mem.write(r.code, &[0x08]);
        let code = r.code;
        for address in [ERR_STREAM_CONSTRUCT, OSTREAM_WRITE_TEXT, OSTREAM_DESTRUCT] {
            let sink = Rc::clone(&r.log);
            r.e.register_double(address, move |_, a| {
                sink.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
        let sink = Rc::clone(&r.log);
        r.e.register_double(0x0900_000c, move |e, a| {
            sink.borrow_mut().push((0x0900_000c, a.to_vec()));
            // The handler "repairs" the byte so the next pass ends the query.
            e.mem.set_u8(code, 0x30);
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
        let (t, s, c) = (r.this, r.state, r.code);
        r.e.call(0x00e6_5140, &args![t, s, c]);
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
                vec![reporter, 3, 0x1298_fedd, stream + 0xc, ERROR_FILE, 0x251]
            )
        );
        assert_eq!(log[3], (OSTREAM_DESTRUCT, vec![stream]));
        drop(log);
        assert_eq!(r.hits(), [0]);
    }
}

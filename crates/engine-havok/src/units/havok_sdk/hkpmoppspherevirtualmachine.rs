//! `Havok SDK/hkpmoppspherevirtualmachine.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// A byte in the exe image (`0126efbb`): non-zero once the product key check has passed.
const LICENSE_FLAG: u32 = 0x0126_efbb;
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
/// and the source file string `.\Collide\Mopp\Machine\hkpMoppSphereVirtualMachine.cpp`
/// (`010ecffc`).
const CORRUPTED_TEXT: u32 = 0x010d_5100;
const ERROR_FILE: u32 = 0x010e_cffc;

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

/// Copies the 0x28-byte record at `from` over the one at `to` (`REP MOVSD`, ten words).
fn copy_record(e: &mut Engine, from: u32, to: u32) {
    for i in 0..10 {
        let word = e.mem.u32(from + 4 * i);
        e.mem.set_u32(to + 4 * i, word);
    }
}

/// `r3 + r3 / 2 + 1`, the sphere radius of the commands that work on doubled bounds.
fn radius_one_and_a_half(radius: i32) -> i32 {
    radius.wrapping_add(radius >> 1).wrapping_add(1)
}

// Translated from 00e64870 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMoppSphereVirtualMachine::querySphereOnTree` (Xbox PDB), thiscall `(this, state, code)`:
/// interprets the command bytes of a MOPP tree at `code`, appending the hit ids to the array
/// the word at `this + 0` points to. Does nothing until the product key check (`00df22f0`)
/// has set the byte at `0126efbb`.
///
/// `state` is a record of ten words: `+0/+4/+8` the sphere centre relative to the current
/// cell, `+0xC` the radius, `+0x10/+0x14/+0x18` the cell origin, `+0x1C` an id offset, `+0x20`
/// a shift and `+0x24` a word nothing reads. `this + 0x10..0x1C` are the sphere's three
/// centre coordinates and its radius. The interpreter keeps a local record of its own; a
/// command that changes the state copies the record there first (and recursive calls get a
/// pointer to either record). Commands: 0 returns; 1 to 4 build the local record narrowed by
/// that many bits from the next three bytes; 5 to 7 skip forward by a 1, 2 or 3 byte
/// offset; 9 to 0xB add to or set the id offset; 0x10 to 0x1C test the sphere against a
/// plane (one axis or a combination of axes) and skip or recurse into a sub-tree; 0x20 to
/// 0x25 do the same against one axis; 0x26 to 0x2B leave when the sphere is outside a
/// range; 0x30 to 0x53 append an id (plus the id offset) and return; 0x60 to 0x6B store a
/// value in a slot of the frame beside the local record and make the local record the
/// current one; any other command reports the "Unknown command" error (`hkErrStream`,
/// `00cb1600`, the reporter's virtual function at 0xC, then `INT3`, which this translation
/// panics on: it is a breakpoint trap in the exe, not a call) and continues with the same
/// byte. The command meanings beyond this are unconfirmed.
///
/// Uninitialised stack words (checked against the disassembly): the local record is built
/// whole by command 1 to 4 or copied whole (`REP MOVSD`) before any read of it. Commands
/// 0x61 to 0x6B write slots beyond the record, which nothing reads; command 0x60/0x64/0x68
/// writes the record's last word, which is only ever copied on. A store to a slot other than
/// the record's last word keeps the local record's (possibly never written) last word over
/// the copy; it reads as zero here and has no effect on any result, as nothing compares it.
pub fn hkp_mopp_sphere_virtual_machine_query_sphere_on_tree(
    e: &mut Engine,
    this: Ptr,
    state: Ptr,
    code: Ptr,
) {
    if e.mem.u8(LICENSE_FLAG) == 0 {
        e.call(LICENSE_CHECK, &args![LICENSE_FLAG]);
        if e.mem.u8(LICENSE_FLAG) == 0 {
            return;
        }
    }
    let t = this.addr();
    e.with_stack(0x240, |e, frame| {
        // The frame: the local record at +0, the slots of commands 0x60 to 0x6B at +0x24
        // (the record's last word) to +0x30, the error stream at +0x30 and its 0x200-byte
        // buffer behind it.
        let local = frame.addr();
        let mut s = state.addr();
        let mut c = code.addr();
        loop {
            let op = u32::from(e.mem.u8(c));
            match op {
                0 => return,
                1..=4 => {
                    let shift = int(e, s + 0x20).wrapping_sub(op as i32);
                    let count = (shift as u32) & 31;
                    let origin_words = [int(e, s + 0x10), int(e, s + 0x14), int(e, s + 0x18)];
                    let id_offset = e.mem.u32(s + 0x1c);
                    let last_word = e.mem.u32(s + 0x24);
                    for (i, origin_word) in origin_words.into_iter().enumerate() {
                        let i = i as u32;
                        let origin = byte(e, c + 1 + i)
                            .wrapping_add(origin_word)
                            .wrapping_shl(op);
                        e.mem.set_u32(local + 0x10 + 4 * i, origin as u32);
                        let relative = (int(e, t + 0x10 + 4 * i) >> count).wrapping_sub(origin);
                        e.mem.set_u32(local + 4 * i, relative as u32);
                    }
                    let radius = (int(e, t + 0x1c) >> count).wrapping_add(1);
                    e.mem.set_u32(local + 0xc, radius as u32);
                    e.mem.set_u32(local + 0x1c, id_offset);
                    e.mem.set_u32(local + 0x20, shift as u32);
                    e.mem.set_u32(local + 0x24, last_word);
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
                        int(e, local + 0x1c).wrapping_add(amount)
                    };
                    e.mem.set_u32(local + 0x1c, id_offset as u32);
                    c += length;
                }
                0x10..=0x1c => {
                    let words = [int(e, s), int(e, s + 4), int(e, s + 8)];
                    let [x, y, z] = words;
                    let radius = int(e, s + 0xc);
                    let (value, radius, factor) = match op {
                        0x10..=0x12 => (words[(op - 0x10) as usize], radius, 1),
                        0x13 => (z.wrapping_add(y), radius_one_and_a_half(radius), 2),
                        0x14 => (
                            y.wrapping_sub(z).wrapping_add(0xff),
                            radius_one_and_a_half(radius),
                            2,
                        ),
                        0x15 => (z.wrapping_add(x), radius_one_and_a_half(radius), 2),
                        0x16 => (
                            x.wrapping_sub(z).wrapping_add(0xff),
                            radius_one_and_a_half(radius),
                            2,
                        ),
                        0x17 => (y.wrapping_add(x), radius_one_and_a_half(radius), 2),
                        0x18 => (
                            x.wrapping_sub(y).wrapping_add(0xff),
                            radius_one_and_a_half(radius),
                            2,
                        ),
                        0x19 => (z.wrapping_add(y).wrapping_add(x), radius.wrapping_mul(4), 3),
                        0x1a => (
                            y.wrapping_sub(z).wrapping_add(x).wrapping_add(0xff),
                            radius.wrapping_mul(4),
                            3,
                        ),
                        0x1b => (
                            z.wrapping_sub(y).wrapping_add(x).wrapping_add(0xff),
                            radius.wrapping_mul(4),
                            3,
                        ),
                        _ => (
                            x.wrapping_sub(z).wrapping_sub(y).wrapping_add(0x1fe),
                            radius.wrapping_mul(4),
                            3,
                        ),
                    };
                    let upper = byte(e, c + 1) * factor;
                    let lower = byte(e, c + 2) * factor;
                    c += 4;
                    if radius.wrapping_add(value) >= lower {
                        let skip = u32::from(e.mem.u8(c - 1));
                        c += skip;
                        if value <= radius.wrapping_add(upper) {
                            hkp_mopp_sphere_virtual_machine_query_sphere_on_tree(
                                e,
                                this,
                                Ptr::new(s),
                                Ptr::new(c - skip),
                            );
                        }
                    }
                }
                0x20..=0x22 => {
                    let value = int(e, s + 4 * (op - 0x20));
                    let radius = int(e, s + 0xc);
                    let lower = byte(e, c + 1);
                    c += 3;
                    if radius.wrapping_add(value) >= lower {
                        let skip = u32::from(e.mem.u8(c - 1));
                        c += skip;
                        if value <= radius.wrapping_add(lower).wrapping_add(1) {
                            hkp_mopp_sphere_virtual_machine_query_sphere_on_tree(
                                e,
                                this,
                                Ptr::new(s),
                                Ptr::new(c - skip),
                            );
                        }
                    }
                }
                0x23..=0x25 => {
                    let value = int(e, s + 4 * (op - 0x23));
                    let upper = byte(e, c + 1);
                    let lower = byte(e, c + 2);
                    let near = big_endian(e, c + 3, 2) as u32;
                    let radius = int(e, s + 0xc);
                    // The exe keeps these two in frame slots (`ESP+0x3c` and `ESP+0x40`).
                    e.mem.set_u32(local + 0x28, value as u32);
                    e.mem.set_u32(local + 0x2c, upper as u32);
                    c += 7;
                    if value.wrapping_add(radius) < lower {
                        c = c.wrapping_add(near);
                    } else {
                        let far = big_endian(e, c - 2, 2) as u32;
                        let near_target = c.wrapping_add(near);
                        c = c.wrapping_add(far);
                        if value <= radius.wrapping_add(upper) {
                            hkp_mopp_sphere_virtual_machine_query_sphere_on_tree(
                                e,
                                this,
                                Ptr::new(s),
                                Ptr::new(near_target),
                            );
                        }
                    }
                }
                0x26..=0x28 => {
                    let value = int(e, s + 4 * (op - 0x26));
                    let radius = int(e, s + 0xc);
                    if radius.wrapping_add(value) < byte(e, c + 1) {
                        return;
                    }
                    if value > byte(e, c + 2).wrapping_add(radius) {
                        return;
                    }
                    c += 3;
                }
                0x29..=0x2b => {
                    let value = int(e, t + 0x10 + 4 * (op - 0x29));
                    let base = int(e, t + 0x1c);
                    if base.wrapping_add(value) < big_endian(e, c + 1, 3) {
                        return;
                    }
                    if value > big_endian(e, c + 4, 3).wrapping_add(base) {
                        return;
                    }
                    c += 7;
                }
                0x30..=0x53 => {
                    let amount = match op {
                        0x30..=0x4f => op as i32 - 0x30,
                        0x50 => byte(e, c + 1),
                        0x51 => big_endian(e, c + 1, 2),
                        0x52 => big_endian(e, c + 1, 3),
                        _ => big_endian(e, c + 1, 4),
                    };
                    let value = amount.wrapping_add(int(e, s + 0x1c)) as u32;
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
                0x60..=0x6b => {
                    let (value, length) = match op {
                        0x60..=0x63 => (byte(e, c + 1), 2),
                        0x64..=0x67 => (big_endian(e, c + 1, 2), 3),
                        _ => (big_endian(e, c + 1, 4), 5),
                    };
                    e.mem.set_u32(local + 0x24 + 4 * (op & 3), value as u32);
                    c += length;
                    // The exe keeps the record's last word in a register across the copy.
                    let kept = e.mem.u32(local + 0x24);
                    if s != local {
                        copy_record(e, s, local);
                        s = local;
                    }
                    e.mem.set_u32(local + 0x24, kept);
                }
                _ => {
                    let stream = local + 0x30;
                    let buffer = stream + 0xc;
                    e.call(ERR_STREAM_CONSTRUCT, &args![stream, buffer, 0x200u32]);
                    e.call(OSTREAM_WRITE_TEXT, &args![stream, CORRUPTED_TEXT]);
                    let reporter: u32 = e.global(ERROR_REPORTER);
                    let wanted_break = e
                        .vcall(
                            reporter,
                            0xc,
                            &args![3u32, 0x1298_feddu32, buffer, ERROR_FILE, 0x124u32],
                        )
                        .u32();
                    if wanted_break != 0 {
                        panic!(
                            "00e64870: the exe's INT3 trap (the error reporter asked for a break)"
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
    vec![entry!(
        0x00e6_4870,
        hkp_mopp_sphere_virtual_machine_query_sphere_on_tree(Ptr, Ptr, Ptr)
    )]
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
        array: u32,
    }

    /// An engine with a sphere machine whose result array holds 0x100 words (four at first),
    /// a state record of zeros and the key check passed.
    fn rig() -> Rig {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
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
            let capacity = e.mem.u32(a[1] + 8);
            e.mem.set_u32(a[1] + 8, capacity + 4);
            Ret::default()
        });
        let code = e.mem.alloc(0x400);
        let array = e.mem.alloc(0x10);
        let data = e.mem.alloc(0x400);
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 8, 4);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this, array);
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
        fn set_state(&mut self, words: &[u32]) {
            for (i, &word) in words.iter().enumerate() {
                self.e.mem.set_u32(self.state + 4 * i as u32, word);
            }
        }

        /// The sphere centre and radius words at `this + 0x10`.
        fn set_sphere(&mut self, words: [u32; 4]) {
            for (i, word) in words.into_iter().enumerate() {
                self.e.mem.set_u32(self.this + 0x10 + 4 * i as u32, word);
            }
        }

        /// Runs `bytes` as the command stream with an empty result array; returns the ids.
        fn run(&mut self, bytes: &[u8]) -> Vec<u32> {
            self.e.mem.write(self.code, bytes);
            self.e.mem.set_u32(self.array + 4, 0);
            let (t, s, c) = (self.this, self.state, self.code);
            self.e.call(0x00e6_4870, &args![t, s, c]);
            let data = self.e.mem.u32(self.array);
            (0..self.e.mem.u32(self.array + 4))
                .map(|i| self.e.mem.u32(data + 4 * i))
                .collect()
        }
    }

    #[test]
    fn the_key_check_gates_the_interpreter() {
        let mut r = rig();
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
        assert!(r.run(&[0x31]).is_empty());
        assert_eq!(r.log.borrow()[0], (LICENSE_CHECK, vec![LICENSE_FLAG]));
        passes.set(true);
        assert_eq!(r.run(&[0x31]), [1]);
    }

    #[test]
    fn id_commands_append_the_id_plus_the_offset_and_return() {
        let mut r = rig();
        r.set_state(&[0, 0, 0, 0, 0, 0, 0, 100]);
        assert_eq!(r.run(&[0x30, 0x31]), [100]);
        assert_eq!(r.run(&[0x4f]), [131]);
        assert_eq!(r.run(&[0x50, 0xfe]), [354]);
        assert_eq!(r.run(&[0x51, 0x01, 0x02]), [100 + 0x102]);
        assert_eq!(r.run(&[0x52, 0x01, 0x02, 0x03]), [100 + 0x01_0203]);
        assert_eq!(r.run(&[0x53, 0x01, 0x02, 0x03, 0x04]), [100 + 0x0102_0304]);
        assert!(r.run(&[0x00, 0x31]).is_empty());
    }

    #[test]
    fn a_full_array_is_grown_before_the_append() {
        let mut r = rig();
        r.e.mem.set_u32(r.array + 4, 4);
        r.e.mem.write(r.code, &[0x31]);
        let (t, s, c) = (r.this, r.state, r.code);
        r.e.call(0x00e6_4870, &args![t, s, c]);
        assert_eq!(
            r.log.borrow()[0],
            (ARRAY_RESERVE_MORE, vec![0xa110c, r.array, 4])
        );
        assert_eq!(r.e.mem.u32(r.array + 4), 5);
    }

    #[test]
    fn skip_commands_move_the_pointer() {
        let mut r = rig();
        assert_eq!(r.run(&[0x05, 0x01, 0xaa, 0x32]), [2]);
        assert_eq!(r.run(&[0x06, 0x00, 0x02, 0xaa, 0xaa, 0x33]), [3]);
        assert_eq!(r.run(&[0x07, 0x00, 0x00, 0x01, 0xaa, 0x34]), [4]);
    }

    #[test]
    fn id_offset_commands_work_on_a_copy_of_the_state() {
        let mut r = rig();
        r.set_state(&[0, 0, 0, 0, 0, 0, 0, 10]);
        assert_eq!(r.run(&[0x09, 0x05, 0x30]), [15]);
        assert_eq!(r.run(&[0x0a, 0x01, 0x00, 0x30]), [266]);
        assert_eq!(r.run(&[0x0b, 0x00, 0x01, 0x00, 0x00, 0x30]), [0x1_0000]);
        // Two in a row add up (the second works on the local record already).
        assert_eq!(r.run(&[0x09, 0x05, 0x09, 0x07, 0x30]), [22]);
        assert_eq!(r.e.mem.u32(r.state + 0x1c), 10);
    }

    #[test]
    fn slot_commands_have_their_lengths_and_keep_the_state() {
        let mut r = rig();
        r.set_state(&[0, 0, 0, 0, 0, 0, 0, 7]);
        assert_eq!(r.run(&[0x60, 0x09, 0x30]), [7]);
        assert_eq!(r.run(&[0x63, 0x09, 0x31]), [8]);
        assert_eq!(r.run(&[0x64, 0x01, 0x02, 0x30]), [7]);
        assert_eq!(r.run(&[0x67, 0x01, 0x02, 0x30]), [7]);
        assert_eq!(r.run(&[0x68, 0x01, 0x02, 0x03, 0x04, 0x30]), [7]);
        assert_eq!(r.run(&[0x6b, 0x01, 0x02, 0x03, 0x04, 0x31]), [8]);
        // After a slot command the id offset is edited on the local copy.
        assert_eq!(r.run(&[0x60, 0x00, 0x09, 0x01, 0x30]), [8]);
        assert_eq!(r.e.mem.u32(r.state + 0x1c), 7);
    }

    /// Runs a narrowing command (`op`, bytes 1, 2, 3 after it) on a state with origin
    /// (3, 4, 5), shift 8 and the sphere (2000, 3000, 4000, 640), followed by the range check
    /// of `axis` with `lower` and `upper` and an id command; true when the id came out.
    fn survives_narrowing(r: &mut Rig, axis: u8, lower: u8, upper: u8) -> bool {
        r.set_state(&[0, 0, 0, 0, 3, 4, 5, 77, 8]);
        r.set_sphere([2000, 3000, 4000, 640]);
        let ids = r.run(&[0x02, 1, 2, 3, 0x26 + axis, lower, upper, 0x30]);
        ids == [77]
    }

    #[test]
    fn narrowing_builds_the_relative_centre_and_radius() {
        // Shift 8 - 2 = 6; origins (1+3)<<2 = 16, (2+4)<<2 = 24, (3+5)<<2 = 32; the centre
        // words are 2000>>6 - 16 = 15, 3000>>6 - 24 = 22 and 4000>>6 - 32 = 30; the radius is
        // 640>>6 + 1 = 11. The range check leaves unless radius + centre >= lower and
        // centre <= upper + radius, so the boundaries give 26/4, 33/11 and 41/19.
        let mut r = rig();
        for (axis, sum, difference) in [(0u8, 26u8, 4u8), (1, 33, 11), (2, 41, 19)] {
            assert!(survives_narrowing(&mut r, axis, sum, 255));
            assert!(!survives_narrowing(&mut r, axis, sum + 1, 255));
            assert!(survives_narrowing(&mut r, axis, 0, difference));
            assert!(!survives_narrowing(&mut r, axis, 0, difference - 1));
        }
    }

    #[test]
    fn narrowing_twice_uses_the_shift_left_over() {
        // The first command leaves shift 6 and origin 16; the second one (1 bit, zero bytes)
        // makes the shift 5 and the origin (0+16)<<1 = 32: the centre is 2000>>5 - 32 = 30,
        // the radius 640>>5 + 1 = 21; the boundaries are 51 and 9.
        let mut r = rig();
        r.set_state(&[0, 0, 0, 0, 3, 4, 5, 77, 8]);
        r.set_sphere([2000, 3000, 4000, 640]);
        let run = |r: &mut Rig, lower: u8, upper: u8| {
            r.run(&[0x02, 1, 2, 3, 0x01, 0, 0, 0, 0x26, lower, upper, 0x30]) == [77]
        };
        assert!(run(&mut r, 51, 255));
        assert!(!run(&mut r, 52, 255));
        assert!(run(&mut r, 0, 9));
        assert!(!run(&mut r, 0, 8));
        // The caller's record is untouched.
        assert_eq!(r.e.mem.u32(r.state + 0x20), 8);
        assert_eq!(r.e.mem.u32(r.state + 0x10), 3);
    }

    /// The outcome of a plane command `[op, upper, lower, 1, 0x31, 0x32]` (its sub-tree is
    /// the one id command, the rest of the stream another): the sub-tree runs as the main
    /// stream (`[1]`), only the rest (`[2]`) or the sub-tree by recursion and then the rest
    /// (`[1, 2]`).
    fn plane_outcome(r: &mut Rig, op: u8, upper: u8, lower: u8) -> Vec<u32> {
        r.run(&[op, upper, lower, 1, 0x31, 0x32])
    }

    #[test]
    fn plane_commands_choose_between_three_outcomes() {
        // State (3, 6, 10) with radius 2: (op, value, radius, factor) as the commands combine
        // them.
        let mut r = rig();
        r.set_state(&[3, 6, 10, 2]);
        let cases = [
            (0x10u8, 3i32, 2i32, 1i32),
            (0x11, 6, 2, 1),
            (0x12, 10, 2, 1),
            (0x13, 16, 4, 2),
            (0x14, 251, 4, 2),
            (0x15, 13, 4, 2),
            (0x16, 248, 4, 2),
            (0x17, 9, 4, 2),
            (0x18, 252, 4, 2),
            (0x19, 19, 8, 3),
            (0x1a, 254, 8, 3),
            (0x1b, 262, 8, 3),
            (0x1c, 497, 8, 3),
        ];
        for (op, value, radius, factor) in cases {
            // Left of the plane: value + radius < lower * factor.
            let lower = ((value + radius) / factor + 1) as u8;
            assert_eq!(plane_outcome(&mut r, op, 255, lower), [1], "op {op:#x} low");
            // Right of the plane: value > radius + upper * factor.
            let upper = ((value - radius - 1) / factor) as u8;
            assert_eq!(plane_outcome(&mut r, op, upper, 0), [2], "op {op:#x} high");
            // On the plane: both.
            assert_eq!(
                plane_outcome(&mut r, op, upper + 1, 0),
                [1, 2],
                "op {op:#x} both"
            );
        }
    }

    #[test]
    fn single_axis_plane_commands_choose_between_three_outcomes() {
        let mut r = rig();
        r.set_state(&[3, 6, 10, 2]);
        for (axis, value) in [(0u8, 3u8), (1, 6), (2, 10)] {
            let op = 0x20 + axis;
            // value + radius < lower: the sub-tree runs as the main stream.
            assert_eq!(plane_outcome_short(&mut r, op, value + 3), [1]);
            // value > radius + lower + 1: only the rest.
            if value >= 4 {
                assert_eq!(plane_outcome_short(&mut r, op, value - 4), [2]);
            }
            // value <= radius + lower + 1: both.
            assert_eq!(plane_outcome_short(&mut r, op, value - 3), [1, 2]);
        }
    }

    /// `[op, lower, 1, 0x31, 0x32]` for the single-axis plane commands.
    fn plane_outcome_short(r: &mut Rig, op: u8, lower: u8) -> Vec<u32> {
        r.run(&[op, lower, 1, 0x31, 0x32])
    }

    #[test]
    fn two_way_plane_commands_choose_between_three_outcomes() {
        // `[op, upper, lower, 0, 1, 0, 2, 0x31, 0x32, 0x33]`: jump 1 over the first id
        // command to the second one, jump 2 over both to the third.
        let mut r = rig();
        r.set_state(&[3, 6, 10, 2]);
        for (axis, value) in [(0u8, 3u8), (1, 6), (2, 10)] {
            let op = 0x23 + axis;
            let mut outcome =
                |upper: u8, lower: u8| r.run(&[op, upper, lower, 0, 1, 0, 2, 0x31, 0x32, 0x33]);
            // value + radius < lower: the second id command runs as the main stream.
            assert_eq!(outcome(255, value + 3), [2]);
            // value > radius + upper: only the third.
            if value >= 3 {
                assert_eq!(outcome(value - 3, 0), [3]);
            }
            // value <= radius + upper: the second by recursion, then the third.
            assert_eq!(outcome(value - 2, 0), [2, 3]);
        }
    }

    #[test]
    fn range_commands_leave_outside_the_range() {
        let mut r = rig();
        r.set_state(&[3, 6, 10, 2]);
        for (axis, value) in [(0u8, 3u8), (1, 6), (2, 10)] {
            let op = 0x26 + axis;
            // Stays when value + radius >= lower and value <= upper + radius.
            assert_eq!(r.run(&[op, value + 2, 255, 0x31]), [1]);
            assert!(r.run(&[op, value + 3, 255, 0x31]).is_empty());
            assert_eq!(r.run(&[op, 0, value - 2, 0x31]), [1]);
            assert!(r.run(&[op, 0, value - 3, 0x31]).is_empty());
        }
    }

    #[test]
    fn sphere_range_commands_use_the_machine_s_own_bounds() {
        let mut r = rig();
        r.set_sphere([100, 200, 300, 5]);
        for (axis, centre) in [(0u8, 100u32), (1, 200), (2, 300)] {
            let op = 0x29 + axis;
            // Stays when radius + centre >= lower and centre <= upper + radius (24-bit values).
            let lower = (centre + 5).to_be_bytes();
            let upper = (centre - 5).to_be_bytes();
            let stays = [
                op, lower[1], lower[2], lower[3], upper[1], upper[2], upper[3], 0x31,
            ];
            assert_eq!(r.run(&stays), [1]);
            let too_high = [
                op,
                lower[1],
                lower[2],
                lower[3] + 1,
                upper[1],
                upper[2],
                upper[3],
                0x31,
            ];
            assert!(r.run(&too_high).is_empty());
            let too_low = [
                op,
                lower[1],
                lower[2],
                lower[3],
                upper[1],
                upper[2],
                upper[3] - 1,
                0x31,
            ];
            assert!(r.run(&too_low).is_empty());
        }
    }

    #[test]
    fn an_unknown_command_reports_and_continues_on_the_same_byte() {
        unknown_command(0);
    }

    #[test]
    #[should_panic(expected = "00e64870: the exe's INT3 trap")]
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
        let (t, s, c) = (r.this, r.state, r.code);
        r.e.call(0x00e6_4870, &args![t, s, c]);
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
                vec![reporter, 3, 0x1298_fedd, stream + 0xc, ERROR_FILE, 0x124]
            )
        );
        assert_eq!(log[3], (OSTREAM_DESTRUCT, vec![stream]));
    }
}

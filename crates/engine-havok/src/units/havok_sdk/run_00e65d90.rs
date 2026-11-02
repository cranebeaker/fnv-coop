//! `Havok SDK/run_00e65d90` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The functions of this range belong to the k-dop geometry machine of
//! `hkpMoppKDopGeometriesVirtualMachine` (`units/havok_sdk/hkpmoppkdopgeometriesvirtualmachine.rs`:
//! the object `00e65140` takes as `this`): the scale at +0x10, the hit array at
//! +0x34 (data, size, capacity with flag bits at +0x3c), the saved-k-dop write pointer at
//! +0x40, the thirteen bound pairs from +0x44 and the flags from +0xac (field names
//! describe what the code does and are unconfirmed), and to three walkers of the MOPP
//! command stream that collect terminal ids. Walkers are described by what they do; the
//! engine map has no names for them.

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`00c85750`); its word at +0x10 is the allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkArrayUtil::_reserveMore` (Xbox PDB, `00c90940`, cdecl `(allocator, array, elementSize)`).
const ARRAY_RESERVE_MORE: u32 = 0x00c9_0940;
/// Constructor of `hkErrStream` (`00cb1600`, thiscall `(this, buffer, size)`).
const ERR_STREAM_CONSTRUCT: u32 = 0x00cb_1600;
/// Writes text into the stream (`00cc01c0`, `(this, text)`) and destroys it (`00cc00e0`).
const OSTREAM_WRITE_TEXT: u32 = 0x00cc_01c0;
const OSTREAM_DESTRUCT: u32 = 0x00cc_00e0;
/// The global pointer to the Havok error reporter (`01268360`); its vtable offset 0xc reports
/// a message `(level, id, text, file, line)` and returns non-zero to request a break.
const ERROR_REPORTER: u32 = 0x0126_8360;
/// The message `Unknown command - This mopp data has been corrupted (check for m...` (`010d5100`)
/// and the source file string of the walkers (`010ed074`).
const CORRUPTED_TEXT: u32 = 0x010d_5100;
const ERROR_FILE: u32 = 0x010e_d074;
/// `hkpMoppKDopGeometriesVirtualMachine::queryMoppKDopGeometriesRecurse` (`00e65140`, thiscall
/// `(this, state, code)`).
const QUERY_RECURSE: u32 = 0x00e6_5140;
/// The double 2.0 and the double the scale is multiplied by (`01091c00`).
const TWO_DOUBLE: u32 = 0x0101_1590;
const BOUND_SCALE_DOUBLE: u32 = 0x0109_1c00;
/// Report id of the "Unknown command" message.
const CORRUPTED_ID: u32 = 0x1298_fedd;

// Translated from 00e65d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (thiscall on the machine of `00e65140`, three words, `RET 0xc`): sets the
/// machine up for one query and runs it. `shape` is the object queried: `shape + 0x1c`
/// the float the scale is made from, `shape + 0x10` four words copied to `this + 0x20`,
/// `shape + 0x20` the pointer to the command bytes. `settings` is four words copied to
/// `this + 0xb0`. `stack` becomes `this + 0x40`. The scale at `this + 0x10` is
/// `1 / shape.float`; the stop byte at +0xc0, the depth at +0xac are cleared, the current
/// id at +0x30 is -1; the thirteen bound pairs from +0x44 are set to `(-2 * k, 2 * k)` with
/// `k = float(scale * c)` (the double at `01091c00`, times the double 2.0 of `01011590`).
/// Then `00e65140` runs on the command bytes with a zeroed six-word state record.
pub fn fn_00e65d90(e: &mut Engine, this: Ptr, shape: Ptr, settings: Ptr, stack: u32) {
    let t = this.addr();
    let shape = shape.addr();
    e.mem.set_u32(t + 0x40, stack);
    let divisor = x87_load32(e.mem.u32(shape + 0x1c));
    let scale_bits = x87_store32(x87_div(1.0, divisor));
    e.mem.set_u32(t + 0x10, scale_bits);
    for word in 0..4 {
        let bits = e.mem.u32(shape + 0x10 + 4 * word);
        e.mem.set_u32(t + 0x20 + 4 * word, bits);
    }
    e.mem.set_u8(t + 0xc0, 0);
    let scale = x87_load32(e.mem.u32(t + 0x10));
    let product = x87_mul(scale, e.mem.f64(BOUND_SCALE_DOUBLE));
    for word in 0..4 {
        let bits = e.mem.u32(settings.addr() + 4 * word);
        e.mem.set_u32(t + 0xb0 + 4 * word, bits);
    }
    let half_range = x87_load32(x87_store32(product));
    let two = e.mem.f64(TWO_DOUBLE);
    e.mem.set_u32(t + 0x30, 0xffff_ffff);
    e.mem.set_u32(t + 0xac, 0);
    let lower = x87_store32(x87_mul(-half_range, two));
    let upper = x87_store32(x87_mul(half_range, two));
    for pair in 0..13 {
        e.mem.set_u32(t + 0x44 + 8 * pair, x87_quiet_bits(lower));
        e.mem.set_u32(t + 0x48 + 8 * pair, x87_quiet_bits(upper));
    }
    let code = e.mem.u32(shape + 0x20);
    e.with_stack(0x18, |e, state| {
        e.call(QUERY_RECURSE, &args![this, state, code]);
    });
}

// Translated from 00e65e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (fastcall on the machine): empties the hit array (data pointer +0x34 and
/// size +0x38 to 0, capacity word +0x3c to 0x80000000, the "not owned" flag) and
/// resets the settings: byte +0xb0 to 0, limit +0xb4 to -1, byte +0xb8 to 0, id +0xbc to 0.
pub fn fn_00e65e70(e: &mut Engine, this: Ptr) {
    let t = this.addr();
    e.mem.set_u32(t + 0x34, 0);
    e.mem.set_u32(t + 0x38, 0);
    e.mem.set_u32(t + 0x3c, 0x8000_0000);
    e.mem.set_u8(t + 0xb0, 0);
    e.mem.set_u32(t + 0xb4, 0xffff_ffff);
    e.mem.set_u8(t + 0xb8, 0);
    e.mem.set_u32(t + 0xbc, 0);
}

// Translated from 00e65ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (fastcall on the machine): releases the hit array. The size at +0x38
/// becomes 0; when the capacity word at +0x3c is not negative (the block is owned) the
/// allocator (`[00c85750() + 0x10]`) frees the data pointer at +0x34 through its virtual
/// function at +0x10 with `(data, 4 * (capacity & 0x3fffffff))`; then the capacity word
/// is 0x80000000 and the data pointer 0.
pub fn fn_00e65ea0(e: &mut Engine, this: Ptr) {
    let t = this.addr();
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.u32(t + 0x3c);
    e.mem.set_u32(t + 0x38, 0);
    if (capacity as i32) >= 0 {
        let data = e.mem.u32(t + 0x34);
        e.vcall(
            allocator,
            0x10,
            &args![data, (capacity & 0x3fff_ffff).wrapping_mul(4)],
        );
    }
    e.mem.set_u32(t + 0x3c, 0x8000_0000);
    e.mem.set_u32(t + 0x34, 0);
}

/// What the plane commands (0x10 to 0x25) of a walker do. All three walkers read the same
/// command bytes and add the id (plus the id offset of the state) of every terminal they
/// reach to the hit array at `*this`.
#[derive(Clone, Copy, PartialEq, Debug)]
enum PlaneCommands {
    /// `00e65ee0`: walk the near subtree (after the command), then continue with the far
    /// subtree.
    Both,
    /// `00e66290`: skip the near subtree and continue with the far one.
    Far,
    /// `00e66600`: continue with the near subtree (the command's operand bytes are skipped).
    Near,
}

/// Offsets inside the walkers' stack frame, measured from the first byte of the frame the
/// code addresses as `ESP + 0xc` after its prologue: the two words of the local state
/// record, the saved `this` word, and the error stream with its buffer. Four word slots at
/// +0x8 (`properties`, indexed by the low two bits of commands 0x60 to 0x6b) overlap the
/// second word of the record, the saved `this` and the stream: the exe stores there, and
/// the translation does too.
const FRAME_RECORD: u32 = 4;
const FRAME_THIS: u32 = 0xc;
const FRAME_STREAM: u32 = 0x10;
const FRAME_BUFFER: u32 = 0x1c;
const FRAME_SIZE: u32 = 0x21c;

fn byte(e: &Engine, at: u32) -> u32 {
    u32::from(e.mem.u8(at))
}

/// A big-endian value of `count` bytes starting at `at`.
fn big_endian(e: &Engine, at: u32, count: u32) -> u32 {
    (0..count).fold(0u32, |acc, i| (acc << 8).wrapping_add(byte(e, at + i)))
}

/// The walkers' shared body. `this` points to a word that points to the hit array
/// (`hkArray<unsigned>`: data, size, capacity with flags); `state` to a record whose first
/// word is the id offset; `code` to the command bytes. Commands (the byte tables of each
/// walker are the same up to the plane commands): 0 returns; 1 to 4 skip 4 bytes; 5 to 7
/// skip a 1, 2 or 3 byte distance; 9 and 10 add a 1 or 2 byte value, 0xb sets a 4 byte
/// value, as the first word of a local copy of the state (made first when the state is not
/// the local one); 0x10 to 0x25 are plane commands (see [`PlaneCommands`]); 0x26 to 0x28 skip
/// 3 bytes and 0x29 to 0x2b skip 7; 0x30 to 0x53 are terminals (the id is `op - 0x30`, or a 1,
/// 2, 3 or 4 byte value; with the id offset it is appended to the hit array, which grows
/// through `00c90940` when full) and return; 0x60 to 0x6b store a 1, 2 or 4 byte value in a
/// slot of the frame and make the local record current; every other command reports the
/// "Unknown command" error through the error reporter (`INT3` when it asks for a break,
/// which this translation panics on) and goes on with the same byte.
fn walk(e: &mut Engine, plane: PlaneCommands, error_line: u32, this: u32, state: u32, code: u32) {
    e.with_stack(FRAME_SIZE + 4, |e, frame| {
        let frame = frame.addr();
        let record = frame + FRAME_RECORD;
        e.mem.set_u32(frame + FRAME_THIS, this);
        let mut s = state;
        let mut c = code;
        loop {
            let op = byte(e, c);
            match op {
                0 => return,
                1..=4 => c = c.wrapping_add(4),
                5 => c = c.wrapping_add(byte(e, c + 1) + 2),
                6 => c = c.wrapping_add(big_endian(e, c + 1, 2) + 3),
                7 => c = c.wrapping_add(big_endian(e, c + 1, 3) + 4),
                9..=11 => {
                    let (value, length) = match op {
                        9 => (byte(e, c + 1), 2),
                        10 => (big_endian(e, c + 1, 2), 3),
                        _ => (big_endian(e, c + 1, 4), 5),
                    };
                    if s != record {
                        copy_state(e, s, record);
                        s = record;
                    }
                    let first = if op == 11 {
                        value
                    } else {
                        e.mem.u32(record).wrapping_add(value)
                    };
                    e.mem.set_u32(record, first);
                    c = c.wrapping_add(length);
                }
                0x10..=0x1c => match plane {
                    PlaneCommands::Both => {
                        let skip = byte(e, c + 3);
                        let this = e.mem.u32(frame + FRAME_THIS);
                        walk(e, plane, error_line, this, s, c + 4);
                        c = c.wrapping_add(4).wrapping_add(skip);
                    }
                    PlaneCommands::Far => c = c.wrapping_add(byte(e, c + 3) + 4),
                    PlaneCommands::Near => c = c.wrapping_add(4),
                },
                0x20..=0x22 => match plane {
                    PlaneCommands::Both => {
                        let skip = byte(e, c + 2);
                        let this = e.mem.u32(frame + FRAME_THIS);
                        walk(e, plane, error_line, this, s, c + 3);
                        c = c.wrapping_add(3).wrapping_add(skip);
                    }
                    PlaneCommands::Far => c = c.wrapping_add(byte(e, c + 2) + 3),
                    PlaneCommands::Near => c = c.wrapping_add(3),
                },
                0x23..=0x25 => match plane {
                    PlaneCommands::Both => {
                        let near = big_endian(e, c + 3, 2);
                        let far = big_endian(e, c + 5, 2);
                        let this = e.mem.u32(frame + FRAME_THIS);
                        walk(e, plane, error_line, this, s, c.wrapping_add(7).wrapping_add(near));
                        c = c.wrapping_add(7).wrapping_add(far);
                    }
                    PlaneCommands::Far => {
                        c = c.wrapping_add(big_endian(e, c + 5, 2)).wrapping_add(7);
                    }
                    PlaneCommands::Near => {
                        c = c.wrapping_add(big_endian(e, c + 3, 2)).wrapping_add(7);
                    }
                },
                0x26..=0x28 => c = c.wrapping_add(3),
                0x29..=0x2b => c = c.wrapping_add(7),
                0x30..=0x53 => {
                    let id = match op {
                        0x30..=0x4f => op - 0x30,
                        0x50 => byte(e, c + 1),
                        0x51 => big_endian(e, c + 1, 2),
                        0x52 => big_endian(e, c + 1, 3),
                        _ => {
                            // 0x53: bytes 1 and 2 shifted 16, byte 3 shifted 8, byte 4.
                            (byte(e, c + 1) << 8)
                                .wrapping_add(byte(e, c + 2))
                                .wrapping_shl(16)
                                .wrapping_add(byte(e, c + 4))
                                .wrapping_add(byte(e, c + 3) << 8)
                        }
                    };
                    let this = e.mem.u32(frame + FRAME_THIS);
                    let id = id.wrapping_add(e.mem.u32(s));
                    let array = e.mem.u32(this);
                    append_hit(e, array, id);
                    return;
                }
                0x60..=0x6b => {
                    let (value, length) = match op {
                        0x60..=0x63 => (byte(e, c + 1), 2),
                        0x64..=0x67 => (big_endian(e, c + 1, 2), 3),
                        _ => (big_endian(e, c + 1, 4), 5),
                    };
                    e.mem.set_u32(frame + 8 + 4 * (op & 3), value);
                    c = c.wrapping_add(length);
                    let kept = e.mem.u32(frame + 8);
                    if s != record {
                        copy_state(e, s, record);
                        s = record;
                    }
                    e.mem.set_u32(frame + 8, kept);
                }
                _ => {
                    let stream = frame + FRAME_STREAM;
                    let buffer = frame + FRAME_BUFFER;
                    e.call(ERR_STREAM_CONSTRUCT, &args![stream, buffer, 0x200u32]);
                    e.call(OSTREAM_WRITE_TEXT, &args![stream, CORRUPTED_TEXT]);
                    let reporter: u32 = e.global(ERROR_REPORTER);
                    let wanted_break = e
                        .vcall(
                            reporter,
                            0xc,
                            &args![3u32, CORRUPTED_ID, buffer, ERROR_FILE, error_line],
                        )
                        .u32();
                    if wanted_break != 0 {
                        panic!(
                            "the exe's INT3 trap (the error reporter asked for a break), command {op:#x}"
                        );
                    }
                    e.call(OSTREAM_DESTRUCT, &args![stream]);
                }
            }
        }
    });
}

/// The two words of a state record copied to the local one.
fn copy_state(e: &mut Engine, from: u32, to: u32) {
    let first = e.mem.u32(from);
    let second = e.mem.u32(from + 4);
    e.mem.set_u32(to, first);
    e.mem.set_u32(to + 4, second);
}

/// Appends `id` to the `hkArray<unsigned>` at `array`, growing it through `00c90940`
/// (allocator from the memory router) when its size equals its capacity.
fn append_hit(e: &mut Engine, array: u32, id: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
    if e.mem.u32(array + 4) == capacity {
        let allocator = e.mem.u32(router + 0x10);
        e.call(ARRAY_RESERVE_MORE, &args![allocator, array, 4u32]);
    }
    let size = e.mem.u32(array + 4);
    let data = e.mem.u32(array);
    e.mem.set_u32(array + 4, size.wrapping_add(1));
    e.mem.set_u32(data.wrapping_add(size.wrapping_mul(4)), id);
}

// Translated from 00e65ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (thiscall, `RET 8`: `(this, state, code)`): a walker of the MOPP command
/// bytes at `code` that collects the id of every terminal into the array that
/// `*this` points to. It walks both subtrees of every plane command (the near subtree
/// recursively, then the far one), see `walk` for the commands and the frame it keeps.
pub fn fn_00e65ee0(e: &mut Engine, this: Ptr, state: Ptr, code: Ptr) {
    walk(
        e,
        PlaneCommands::Both,
        0x71,
        this.addr(),
        state.addr(),
        code.addr(),
    );
}

// Translated from 00e66290 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (thiscall, `RET 8`: `(this, state, code)`): like `fn_00e65ee0`, but at every
/// plane command it skips the near subtree and continues with the far one, so it reaches
/// the terminal at the end of the far branches. Its error report carries source line 0xd3.
pub fn fn_00e66290(e: &mut Engine, this: Ptr, state: Ptr, code: Ptr) {
    walk(
        e,
        PlaneCommands::Far,
        0xd3,
        this.addr(),
        state.addr(),
        code.addr(),
    );
}

// Translated from 00e66600 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (thiscall, `RET 8`: `(this, state, code)`): like `fn_00e65ee0`, but at every
/// plane command it continues with the near subtree (skipping only the command's operand
/// bytes), so it reaches the first terminal. Its error report carries source line 0x131.
pub fn fn_00e66600(e: &mut Engine, this: Ptr, state: Ptr, code: Ptr) {
    walk(
        e,
        PlaneCommands::Near,
        0x131,
        this.addr(),
        state.addr(),
        code.addr(),
    );
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e65d90, fn_00e65d90(Ptr, Ptr, Ptr, u32)),
        entry!(0x00e65e70, fn_00e65e70(Ptr)),
        entry!(0x00e65ea0, fn_00e65ea0(Ptr)),
        entry!(0x00e65ee0, fn_00e65ee0(Ptr, Ptr, Ptr)),
        entry!(0x00e66290, fn_00e66290(Ptr, Ptr, Ptr)),
        entry!(0x00e66600, fn_00e66600(Ptr, Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    const STATE_OFFSET: u32 = 0x100;
    const REPORT_DOUBLE: u32 = 0x00aa_0100;
    const ALLOCATOR_FREE: u32 = 0x00aa_0200;

    struct Rig {
        e: Engine,
        log: Log,
        this: u32,
        array: u32,
        state: u32,
        code: u32,
    }

    /// A walker's world: the object whose first word points at an `hkArray<unsigned>` (room
    /// for four ids), a two-word state record (id offset 0x100), the command buffer and
    /// doubles for the memory router, the array growth and the error stream.
    fn rig() -> Rig {
        let mut e = Engine::new();
        e.map(0x0, 0x1000);
        e.map(0x0126_8000, 0x1000);
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
            let (array, size, capacity) = (a[1], e.mem.u32(a[1] + 4), e.mem.u32(a[1] + 8));
            let data = e.mem.alloc((capacity + 4) * 4);
            for i in 0..size {
                let word = e.mem.u32(e.mem.u32(array) + 4 * i);
                e.mem.set_u32(data + 4 * i, word);
            }
            e.mem.set_u32(array, data);
            e.mem.set_u32(array + 8, capacity + 4);
            Ret::default()
        });
        for address in [ERR_STREAM_CONSTRUCT, OSTREAM_WRITE_TEXT, OSTREAM_DESTRUCT] {
            let sink = Rc::clone(&log);
            e.register_double(address, move |_, a| {
                sink.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
        let array = e.mem.alloc(0x10);
        let data = e.mem.alloc(0x10);
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 8, 4);
        let this = e.mem.alloc(8);
        e.mem.set_u32(this, array);
        let state = e.mem.alloc(8);
        e.mem.set_u32(state, STATE_OFFSET);
        e.mem.set_u32(state + 4, 0xaa);
        let code = e.mem.alloc(0x100);
        Rig {
            e,
            log,
            this,
            array,
            state,
            code,
        }
    }

    impl Rig {
        /// Runs the walker at `address` on `bytes`; returns the hit ids.
        fn run(&mut self, address: u32, bytes: &[u8]) -> Vec<u32> {
            self.e.mem.write(self.code, bytes);
            self.e.mem.set_u32(self.array + 4, 0);
            self.e.call(
                address,
                &args![
                    Ptr::<()>::new(self.this),
                    Ptr::<()>::new(self.state),
                    Ptr::<()>::new(self.code)
                ],
            );
            self.hits(self.array)
        }

        fn hits(&self, array: u32) -> Vec<u32> {
            let data = self.e.mem.u32(array);
            (0..self.e.mem.u32(array + 4))
                .map(|i| self.e.mem.u32(data + 4 * i))
                .collect()
        }
    }

    const WALKERS: [u32; 3] = [0x00e6_5ee0, 0x00e6_6290, 0x00e6_6600];

    /// A reporter object whose vtable slot 3 is `REPORT_DOUBLE`.
    fn install_reporter(rig: &mut Rig, sink: Log, answer: u32, patch: Option<(u32, u8)>) {
        let reporter = rig.e.mem.alloc(8);
        rig.e.put_vtable(0x0126_8400, &[0, 0, 0, REPORT_DOUBLE]);
        rig.e.mem.set_u32(reporter, 0x0126_8400);
        rig.e.mem.set_u32(ERROR_REPORTER, reporter);
        rig.e.register_double(REPORT_DOUBLE, move |e, a| {
            sink.borrow_mut().push((REPORT_DOUBLE, a.to_vec()));
            if let Some((at, value)) = patch {
                e.mem.set_u8(at, value);
            }
            Ret {
                eax: answer,
                ..Ret::default()
            }
        });
    }

    #[test]
    fn terminals_add_the_id_offset_in_every_walker() {
        for walker in WALKERS {
            let mut rig = rig();
            assert_eq!(rig.run(walker, &[0x31]), vec![0x101], "{walker:08x}");
            assert_eq!(rig.run(walker, &[0x4f]), vec![0x100 + 0x1f]);
            assert_eq!(rig.run(walker, &[0x50, 0xee]), vec![0x100 + 0xee]);
            assert_eq!(rig.run(walker, &[0x51, 0x12, 0x34]), vec![0x100 + 0x1234]);
            assert_eq!(
                rig.run(walker, &[0x52, 0x12, 0x34, 0x56]),
                vec![0x100 + 0x12_3456]
            );
            assert_eq!(
                rig.run(walker, &[0x53, 0x01, 0x02, 0x03, 0x04]),
                vec![0x100 + 0x0102_0304]
            );
        }
    }

    #[test]
    fn skips_move_the_code_pointer_by_the_commands_distances() {
        for walker in WALKERS {
            let mut rig = rig();
            for bytes in [
                vec![0x00, 0x31],
                vec![0x01, 9, 9, 9, 0x31],
                vec![0x05, 1, 0xff, 0x31],
                vec![0x06, 0, 1, 0xff, 0x31],
                vec![0x07, 0, 0, 1, 0xff, 0x31],
                vec![0x26, 0xff, 0xff, 0x31],
                vec![0x29, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x31],
            ] {
                let expected = if bytes[0] == 0 { vec![] } else { vec![0x101] };
                assert_eq!(rig.run(walker, &bytes), expected, "{bytes:?}");
            }
        }
    }

    #[test]
    fn id_offset_commands_work_on_a_local_copy_of_the_state() {
        for walker in WALKERS {
            let mut rig = rig();
            assert_eq!(rig.run(walker, &[0x09, 5, 0x30]), vec![0x105]);
            assert_eq!(rig.run(walker, &[0x0a, 1, 2, 0x30]), vec![0x102 + 0x100]);
            assert_eq!(rig.run(walker, &[0x0b, 0, 0, 1, 0, 0x30]), vec![0x100]);
            // Two commands in a row use the same local copy.
            assert_eq!(rig.run(walker, &[0x09, 1, 0x09, 2, 0x30]), vec![0x103]);
            // The caller's record is untouched.
            assert_eq!(rig.e.mem.u32(rig.state), 0x100);
            assert_eq!(rig.e.mem.u32(rig.state + 4), 0xaa);
        }
    }

    #[test]
    fn plane_commands_differ_by_walker() {
        let mut rig = rig();
        // Command 0x10 (operand: skip 1 at byte 3), 0x20 (skip at byte 2), 0x23 (near 0,
        // far 1 at bytes 3..7): near subtree is the leaf 1, far subtree the leaf 5.
        for bytes in [
            vec![0x10, 0, 0, 1, 0x31, 0x35],
            vec![0x1c, 0, 0, 1, 0x31, 0x35],
            vec![0x20, 0, 1, 0x31, 0x35],
            vec![0x22, 0, 1, 0x31, 0x35],
            vec![0x23, 0, 0, 0, 0, 0, 1, 0x31, 0x35],
            vec![0x25, 0, 0, 0, 0, 0, 1, 0x31, 0x35],
        ] {
            assert_eq!(
                rig.run(0x00e6_5ee0, &bytes),
                vec![0x101, 0x105],
                "{bytes:?}"
            );
            assert_eq!(rig.run(0x00e6_6290, &bytes), vec![0x105], "{bytes:?}");
            assert_eq!(rig.run(0x00e6_6600, &bytes), vec![0x101], "{bytes:?}");
        }
        // A near offset of 2 skips one leaf before the near subtree: the first walker
        // enters it first, the third only follows it.
        let bytes = [0x24, 0, 0, 0, 1, 0, 3, 0x32, 0x31, 0x35, 0x36];
        assert_eq!(rig.run(0x00e6_5ee0, &bytes), vec![0x101, 0x106]);
        assert_eq!(rig.run(0x00e6_6290, &bytes), vec![0x106]);
        assert_eq!(rig.run(0x00e6_6600, &bytes), vec![0x101]);
    }

    #[test]
    fn nested_planes_pass_the_local_state_on() {
        let mut rig = rig();
        // 0x09 sets the offset, then a plane whose near subtree adds to it again.
        let bytes = [0x09, 1, 0x10, 0, 0, 3, 0x09, 2, 0x30, 0x30];
        assert_eq!(rig.run(0x00e6_5ee0, &bytes), vec![0x103, 0x101]);
    }

    #[test]
    fn the_hit_array_grows_when_full() {
        let mut rig = rig();
        // A chain of 0x20 commands, each with one leaf as its near subtree (skip 1) and the
        // next command as its far subtree, ending in a fifth leaf.
        let mut bytes = Vec::new();
        for leaf in [0x30u8, 0x31, 0x32, 0x33] {
            bytes.extend_from_slice(&[0x20, 0, 1, leaf]);
        }
        bytes.push(0x34);
        let hits = rig.run(0x00e6_5ee0, &bytes);
        assert_eq!(hits.len(), 5);
        let reserve: Vec<_> = rig
            .log
            .borrow()
            .iter()
            .filter(|c| c.0 == ARRAY_RESERVE_MORE)
            .cloned()
            .collect();
        assert_eq!(reserve.len(), 1);
        assert_eq!(reserve[0].1, vec![0xa110c, rig.array, 4]);
    }

    #[test]
    fn properties_store_into_frame_slots_including_the_saved_this() {
        for walker in WALKERS {
            let mut rig = rig();
            // Slot 0 and 2 harm nothing.
            assert_eq!(rig.run(walker, &[0x60, 5, 0x31]), vec![0x101]);
            assert_eq!(rig.run(walker, &[0x66, 1, 2, 0x31]), vec![0x101]);
            assert_eq!(rig.run(walker, &[0x6b, 1, 2, 3, 4, 0x31]), vec![0x101]);
            // Slot 1 is the frame's word holding `this`: a terminal after it appends to
            // the array that the stored word (0x80) points at.
            let other = rig.e.mem.alloc(0x10);
            let other_data = rig.e.mem.alloc(0x10);
            rig.e.mem.set_u32(other, other_data);
            rig.e.mem.set_u32(other + 8, 4);
            rig.e.mem.set_u32(0x80, other);
            for bytes in [
                vec![0x61u8, 0x80, 0x31],
                vec![0x65, 0, 0x80, 0x31],
                vec![0x69, 0, 0, 0, 0x80, 0x31],
            ] {
                rig.e.mem.set_u32(other + 4, 0);
                assert_eq!(rig.run(walker, &bytes), vec![], "{bytes:?}");
                assert_eq!(rig.hits(other), vec![0x101], "{bytes:?}");
            }
        }
    }

    #[test]
    fn a_property_makes_the_local_record_current() {
        let mut rig = rig();
        // After a property command the state record is the local copy: a following
        // add command works on the copy (the caller's record stays).
        assert_eq!(rig.run(0x00e6_5ee0, &[0x60, 5, 0x09, 1, 0x30]), vec![0x101]);
        assert_eq!(rig.e.mem.u32(rig.state), 0x100);
    }

    #[test]
    fn unknown_commands_report_the_error_and_go_on() {
        let expected_lines = [
            (0x00e6_5ee0, 0x71),
            (0x00e6_6290, 0xd3),
            (0x00e6_6600, 0x131),
        ];
        for (walker, line) in expected_lines {
            let mut rig = rig();
            let log = Rc::clone(&rig.log);
            // The reporter patches the bad command into a terminal so the loop ends.
            let code = rig.code;
            install_reporter(&mut rig, log, 0, Some((code, 0x31)));
            assert_eq!(rig.run(walker, &[0x08]), vec![0x101]);
            let calls = rig.log.borrow().clone();
            let at = |address: u32| calls.iter().filter(|c| c.0 == address).count();
            assert_eq!(at(ERR_STREAM_CONSTRUCT), 1);
            assert_eq!(at(OSTREAM_WRITE_TEXT), 1);
            assert_eq!(at(OSTREAM_DESTRUCT), 1);
            let construct = calls.iter().find(|c| c.0 == ERR_STREAM_CONSTRUCT).unwrap();
            // The stream and its buffer sit in the walker's frame, 0xc apart.
            assert_eq!(construct.1[1], construct.1[0] + 0xc);
            assert_eq!(construct.1[2], 0x200);
            let write = calls.iter().find(|c| c.0 == OSTREAM_WRITE_TEXT).unwrap();
            assert_eq!(write.1, vec![construct.1[0], CORRUPTED_TEXT]);
            let report = calls.iter().find(|c| c.0 == REPORT_DOUBLE).unwrap();
            assert_eq!(
                report.1,
                vec![
                    rig.e.mem.u32(ERROR_REPORTER),
                    3,
                    CORRUPTED_ID,
                    construct.1[1],
                    ERROR_FILE,
                    line
                ]
            );
        }
    }

    #[test]
    #[should_panic(expected = "INT3")]
    fn the_error_reporter_can_ask_for_a_break() {
        let mut rig = rig();
        let log = Rc::clone(&rig.log);
        install_reporter(&mut rig, log, 1, None);
        rig.run(0x00e6_5ee0, &[0x2c]);
    }

    #[test]
    fn the_setup_makes_the_scale_and_the_bounds_and_runs_the_query() {
        let mut e = Engine::new();
        e.map(0x0101_0000, 0x2_0000);
        e.map(0x0109_1000, 0x1000);
        e.mem.set_f64(TWO_DOUBLE, 2.0);
        e.mem.set_f64(BOUND_SCALE_DOUBLE, 16_777_216.0);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        e.register_double(QUERY_RECURSE, move |e, a| {
            let state: Vec<u32> = (0..6).map(|i| e.mem.u32(a[1] + 4 * i)).collect();
            sink.borrow_mut().push((a.to_vec(), state));
            Ret::default()
        });
        let this = e.mem.alloc(0x100);
        let shape = e.mem.alloc(0x40);
        let lanes = [0.5f32, 1.5, 2.5, 4.0];
        for (lane, value) in lanes.iter().enumerate() {
            e.mem.set_f32(shape + 0x10 + 4 * lane as u32, *value);
        }
        e.mem.set_u32(shape + 0x20, 0x1234_5678);
        let settings = e.mem.alloc(0x10);
        for lane in 0..4 {
            e.mem.set_u32(settings + 4 * lane, 0x1111_1111 * (lane + 1));
        }
        e.mem.set_u8(this + 0xc0, 7);
        e.call(
            0x00e6_5d90,
            &args![
                Ptr::<()>::new(this),
                Ptr::<()>::new(shape),
                Ptr::<()>::new(settings),
                0xcafe_0000u32
            ],
        );
        assert_eq!(e.mem.u32(this + 0x40), 0xcafe_0000);
        assert_eq!(e.mem.f32(this + 0x10), 0.25);
        for lane in 0..4 {
            assert_eq!(e.mem.f32(this + 0x20 + 4 * lane), lanes[lane as usize]);
            assert_eq!(e.mem.u32(this + 0xb0 + 4 * lane), 0x1111_1111 * (lane + 1));
        }
        assert_eq!(e.mem.u8(this + 0xc0), 0);
        assert_eq!(e.mem.u32(this + 0x30), 0xffff_ffff);
        assert_eq!(e.mem.u32(this + 0xac), 0);
        let range = 0.25f32 * 16_777_216.0;
        for pair in 0..13 {
            assert_eq!(e.mem.f32(this + 0x44 + 8 * pair), -range * 2.0);
            assert_eq!(e.mem.f32(this + 0x48 + 8 * pair), range * 2.0);
        }
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        assert_eq!(seen[0].0[0], this);
        assert_eq!(seen[0].0[2], 0x1234_5678);
        assert_eq!(seen[0].1, vec![0; 6]);
    }

    #[test]
    fn reset_clears_the_array_and_the_settings() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x100);
        for offset in [0x34, 0x38, 0x3c, 0xb4, 0xbc] {
            e.mem.set_u32(this + offset, 0x5555_5555);
        }
        e.mem.set_u8(this + 0xb0, 3);
        e.mem.set_u8(this + 0xb8, 4);
        e.call(0x00e6_5e70, &args![Ptr::<()>::new(this)]);
        assert_eq!(e.mem.u32(this + 0x34), 0);
        assert_eq!(e.mem.u32(this + 0x38), 0);
        assert_eq!(e.mem.u32(this + 0x3c), 0x8000_0000);
        assert_eq!(e.mem.u8(this + 0xb0), 0);
        assert_eq!(e.mem.u32(this + 0xb4), 0xffff_ffff);
        assert_eq!(e.mem.u8(this + 0xb8), 0);
        assert_eq!(e.mem.u32(this + 0xbc), 0);
    }

    #[test]
    fn release_frees_an_owned_array_only() {
        let mut e = Engine::new();
        let router = e.mem.alloc(0x20);
        let allocator = e.mem.alloc(8);
        e.mem.set_u32(router + 0x10, allocator);
        e.put_vtable(0x0126_8500, &[0, 0, 0, 0, ALLOCATOR_FREE]);
        e.mem.set_u32(allocator, 0x0126_8500);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let freed = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&freed);
        e.register_double(ALLOCATOR_FREE, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        let this = e.mem.alloc(0x100);
        // Owned: capacity 5 with a flag bit in bit 30.
        e.mem.set_u32(this + 0x34, 0x7000);
        e.mem.set_u32(this + 0x38, 3);
        e.mem.set_u32(this + 0x3c, 0x4000_0005);
        e.call(0x00e6_5ea0, &args![Ptr::<()>::new(this)]);
        assert_eq!(*freed.borrow(), vec![vec![allocator, 0x7000, 20]]);
        assert_eq!(e.mem.u32(this + 0x34), 0);
        assert_eq!(e.mem.u32(this + 0x38), 0);
        assert_eq!(e.mem.u32(this + 0x3c), 0x8000_0000);
        // Not owned (sign bit set): nothing is freed, but the fields are reset.
        e.mem.set_u32(this + 0x34, 0x7000);
        e.mem.set_u32(this + 0x38, 3);
        e.mem.set_u32(this + 0x3c, 0x8000_0005);
        e.call(0x00e6_5ea0, &args![Ptr::<()>::new(this)]);
        assert_eq!(freed.borrow().len(), 1);
        assert_eq!(e.mem.u32(this + 0x34), 0);
        assert_eq!(e.mem.u32(this + 0x3c), 0x8000_0000);
    }
}

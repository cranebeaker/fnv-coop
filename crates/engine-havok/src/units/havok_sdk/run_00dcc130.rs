//! `Havok SDK/run_00dcc130` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The class is `hkIstream` (the vtable at `010d7540` has that RTTI name): a referenced object
//! (`u16` reference count at `+6`) whose word at `+8` is a stream reader. The reader's virtual
//! functions the code uses are named by the Xbox PDB's `hkStreamReader`: `read` (`+0x10`),
//! `skip` (`+0x14`), `setMark` (`+0x1c`) and `rewindToMark` (`+0x20`). Several functions here take
//! the reader in a register (`EDI` or `EAX`), noted in each doc comment.

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable of `hkIstream` (RTTI `.?AVhkIstream@@`) and of `hkBaseObject` (`0102e388`).
const ISTREAM_VTABLE: u32 = 0x010d_7540;
const BASE_OBJECT_VTABLE: u32 = 0x0102_e388;
/// The reference-count functions of `hkReferencedObject` (`hkreferencedobject.cpp`), each
/// `thiscall` on the object with no stack words: add (`00c90510`) and remove (`00c905b0`).
const ADD_REFERENCE: u32 = 0x00c9_0510;
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// `hkStreamReader` virtual functions (Xbox PDB `hkStreamReader`).
const SLOT_READ: u32 = 0x10;
const SLOT_SKIP: u32 = 0x14;
const SLOT_SET_MARK: u32 = 0x1c;
const SLOT_REWIND_TO_MARK: u32 = 0x20;
/// `00cb1550` (`hkstreamreader.cpp`, unnamed): `thiscall` on the reader, answers the next
/// character as an `int`, or -1.
const READ_CHARACTER: u32 = 0x00cb_1550;
/// `strncmp` (`00cb0fd0`, `hkstring.cpp`, cdecl: first, second, count) and the two texts the
/// boolean reader compares with (`0102fd2c` and `0102fd24`; the code compares four bytes of
/// each).
const STRING_COMPARE: u32 = 0x00cb_0fd0;
const TRUE_TEXT: u32 = 0x0102_fd2c;
const FALSE_TEXT: u32 = 0x0102_fd24;
/// The C library routine that turns text into a floating point number (`00ecdf2d`, cdecl:
/// the text and 0), answering in `ST0`.
const TEXT_TO_FLOAT: u32 = 0x00ec_df2d;
/// The `float` the number reader answers when there is no number (`01012054`, -1.0).
const NO_NUMBER: u32 = 0x0101_2054;
/// A global object (`01268364`) whose virtual function at `+0xc` takes a name and answers a
/// stream reader.
const READER_FACTORY: u32 = 0x0126_8364;

// Translated from 00dcc130 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`: `this`, `reader`): the `hkIstream`
/// constructor over an existing reader. Sets the reference count (`+6`) to 1, the vtable, stores
/// the reader at `+8` and adds a reference to it (`00c90510`). Answers `this`.
pub fn fn_00dcc130(e: &mut Engine, this: u32, reader: u32) -> u32 {
    e.mem.set_u16(this + 6, 1);
    e.mem.set_u32(this, ISTREAM_VTABLE);
    e.mem.set_u32(this + 8, reader);
    e.call(ADD_REFERENCE, &args![reader]);
    this
}

// Translated from 00dcc160 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`fastcall` on `this`): the `hkIstream` destructor body. Sets the
/// vtable, removes the reference to the reader when there is one (`00c905b0`), and leaves the
/// `hkBaseObject` vtable.
pub fn fn_00dcc160(e: &mut Engine, this: u32) {
    let reader = e.mem.u32(this + 8);
    e.mem.set_u32(this, ISTREAM_VTABLE);
    if reader != 0 {
        e.call(REMOVE_REFERENCE, &args![reader]);
    }
    e.mem.set_u32(this, BASE_OBJECT_VTABLE);
}

fn is_white_space(byte: u8) -> bool {
    byte == b' ' || byte == b'\t' || byte == b'\r' || byte == b'\n'
}

// Translated from 00dcc1b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. The reader is in a register: `reader` (`EDI`, a stream reader),
/// no stack words. Skips white space (space, tab, carriage return, line feed): sets a mark with
/// a read-ahead of 0x40, reads up to 0x40 bytes into a local buffer, and when one is not white
/// space rewinds to the mark and skips up to that byte; when all are, sets a mark and reads the
/// next 0x40 and looks again. Ends when a read answers 0.
pub fn fn_00dcc1b0(e: &mut Engine, reader: u32) {
    e.with_stack(0x40, |e, buffer| {
        let buffer = buffer.addr();
        e.vcall(reader, SLOT_SET_MARK, &args![0x40u32]);
        let mut count = e.vcall(reader, SLOT_READ, &args![buffer, 0x40u32]).i32();
        if count == 0 {
            return;
        }
        loop {
            if count > 0 {
                let mut index = 0;
                while index < count {
                    if !is_white_space(e.mem.u8(buffer + index as u32)) {
                        e.vcall(reader, SLOT_REWIND_TO_MARK, &args![]);
                        e.vcall(reader, SLOT_SKIP, &args![index as u32]);
                        return;
                    }
                    index += 1;
                }
            }
            e.vcall(reader, SLOT_SET_MARK, &args![0x40u32]);
            count = e.vcall(reader, SLOT_READ, &args![buffer, 0x40u32]).i32();
            if count == 0 {
                return;
            }
        }
    });
}

// Translated from 00dcc3b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. The reader is in a register: `reader` (`EAX`, a stream reader),
/// no stack words; answers a `float` in `ST0`. Skips white space (`00dcc1b0`), sets a mark with
/// a read-ahead of 0xff and reads up to 0xff bytes into a local buffer. The text is taken as far
/// as it looks like a number: the first byte has to be a digit, `+`, `-`, `.` or `,`; each
/// later byte a digit, `+`, `-`, `E` or `e`, or `.` or `,` (both are replaced by `.` in the
/// buffer). The code also replaces `.` or `,` after the digit/sign test; that step cannot be
/// reached for a byte that passed it (it is translated anyway). Then the reader rewinds to the
/// mark and skips the bytes taken, the text is ended with a zero byte and, when at least one
/// byte was taken, converted with the C library routine `00ecdf2d` (text, 0) and stored as a
/// `float` (`FSTP`/`FLD`); otherwise the answer is the `float` at `01012054` (-1.0).
pub fn fn_00dcc3b0(e: &mut Engine, reader: u32) -> f32 {
    fn_00dcc1b0(e, reader);
    e.with_stack(0x100, |e, buffer| {
        let buffer = buffer.addr();
        e.vcall(reader, SLOT_SET_MARK, &args![0xffu32]);
        let count = e.vcall(reader, SLOT_READ, &args![buffer, 0xffu32]).i32();
        let mut taken = 0i32;
        if count != 0 {
            let first = e.mem.i8(buffer);
            let starts_number = (0x30..=0x39).contains(&first)
                || first == 0x2b
                || first == 0x2d
                || first == 0x2e
                || first == 0x2c;
            if starts_number {
                taken = 1;
                if count > taken {
                    loop {
                        let at = buffer + taken as u32;
                        let byte = e.mem.i8(at);
                        let continues = (0x30..=0x39).contains(&byte)
                            || byte == 0x2b
                            || byte == 0x2d
                            || byte == 0x45
                            || byte == 0x65;
                        if continues {
                            // Unreachable: a byte that passed the test above is never `.` or `,`.
                            if byte == 0x2e || byte == 0x2c {
                                e.mem.set_u8(at, 0x2e);
                            }
                        } else if byte == 0x2e || byte == 0x2c {
                            e.mem.set_u8(at, 0x2e);
                        } else {
                            break;
                        }
                        taken += 1;
                        if taken >= count {
                            break;
                        }
                    }
                }
            }
        }
        e.vcall(reader, SLOT_REWIND_TO_MARK, &args![]);
        e.vcall(reader, SLOT_SKIP, &args![taken as u32]);
        e.mem.set_u8(buffer.wrapping_add(taken as u32), 0);
        if taken > 0 {
            let value = e.call(TEXT_TO_FLOAT, &args![buffer, 0u32]).f64();
            x87_quiet(f32::from_bits(x87_store32(value)))
        } else {
            f32::from_bits(x87_quiet_bits(e.mem.u32(NO_NUMBER)))
        }
    })
}

// Translated from 00dcc4a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`: `this` (an `hkIstream`), `out` (a pointer to
/// a byte)). Reads a boolean word: skips white space (`00dcc1b0`), sets a mark with a read-ahead
/// of 6 and reads up to 6 bytes. When four or more were read and the first four equal `true`
/// (the text at `0102fd2c`) and the read stopped there or the next byte is white space, `out`
/// receives 1 and the reader rewinds and skips 4; otherwise when five or more were read and the
/// first four equal the text at `0102fd24` (`false`; only four bytes are compared) and the read
/// stopped at five or the byte after is white space, `out` receives 0 and the reader rewinds and
/// skips 5. In every other case the reader just rewinds and `out` is left alone. Answers `this`.
pub fn fn_00dcc4a0(e: &mut Engine, this: u32, out: u32) -> u32 {
    let reader = e.mem.u32(this + 8);
    fn_00dcc1b0(e, reader);
    e.with_stack(8, |e, buffer| {
        let buffer = buffer.addr();
        let reader = e.mem.u32(this + 8);
        e.vcall(reader, SLOT_SET_MARK, &args![6u32]);
        let reader = e.mem.u32(this + 8);
        let count = e.vcall(reader, SLOT_READ, &args![buffer, 6u32]).i32();
        if count >= 4 {
            let same = e
                .call(STRING_COMPARE, &args![buffer, TRUE_TEXT, 4u32])
                .i32();
            if same == 0 && (count == 4 || is_white_space(e.mem.u8(buffer + 4))) {
                e.mem.set_u8(out, 1);
                let reader = e.mem.u32(this + 8);
                e.vcall(reader, SLOT_REWIND_TO_MARK, &args![]);
                let reader = e.mem.u32(this + 8);
                e.vcall(reader, SLOT_SKIP, &args![4u32]);
                return this;
            }
        }
        if count >= 5 {
            let same = e
                .call(STRING_COMPARE, &args![buffer, FALSE_TEXT, 4u32])
                .i32();
            if same == 0 && (count == 5 || is_white_space(e.mem.u8(buffer + 5))) {
                e.mem.set_u8(out, 0);
                let reader = e.mem.u32(this + 8);
                e.vcall(reader, SLOT_REWIND_TO_MARK, &args![]);
                let reader = e.mem.u32(this + 8);
                e.vcall(reader, SLOT_SKIP, &args![5u32]);
                return this;
            }
        }
        let reader = e.mem.u32(this + 8);
        e.vcall(reader, SLOT_REWIND_TO_MARK, &args![]);
        this
    })
}

// Translated from 00dcc5c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`: `this` (an `hkIstream`), `out` (a pointer to
/// a `float`)). Reads a number from the reader at `this + 8` with `00dcc3b0` and stores it as
/// a `float` at `out`. Answers `this`.
pub fn fn_00dcc5c0(e: &mut Engine, this: u32, out: u32) -> u32 {
    let reader = e.mem.u32(this + 8);
    let value = fn_00dcc3b0(e, reader);
    e.mem.set_u32(out, value.to_bits());
    this
}

// Translated from 00dcc5e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`: `this` (an `hkIstream`), `out` (a pointer to
/// a `double`)). Reads a number from the reader at `this + 8` with `00dcc3b0` and stores the
/// `float` it answers widened to a `double` at `out`. Answers `this`.
pub fn fn_00dcc5e0(e: &mut Engine, this: u32, out: u32) -> u32 {
    let reader = e.mem.u32(this + 8);
    let value = fn_00dcc3b0(e, reader);
    e.mem.set_f64(out, x87_load32(value.to_bits()));
    this
}

// Translated from 00dcc600 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0xc`: `this` (an `hkIstream`), `buffer`,
/// `capacity`, `delimiter` (a `char` passed as a word; its low byte is sign-extended)). Skips
/// white space on the reader at `this + 8` (`00dcc1b0`), then reads characters with `00cb1550`
/// into `buffer` until one is -1 or equals the delimiter: that one is replaced by a zero byte
/// and the number of characters stored is answered. When `capacity` characters were stored
/// without such an end (or `capacity` is not positive) the answer is -1 and no zero byte is
/// written.
pub fn fn_00dcc600(e: &mut Engine, this: u32, buffer: u32, capacity: i32, delimiter: u32) -> i32 {
    let reader = e.mem.u32(this + 8);
    fn_00dcc1b0(e, reader);
    let mut stored = 0i32;
    if capacity > 0 {
        loop {
            let reader = e.mem.u32(this + 8);
            let character = e.call(READ_CHARACTER, &args![reader]).i32();
            if character == -1 || character == i32::from(delimiter as u8 as i8) {
                e.mem.set_u8(buffer.wrapping_add(stored as u32), 0);
                return stored;
            }
            e.mem
                .set_u8(buffer.wrapping_add(stored as u32), character as u8);
            stored += 1;
            if stored >= capacity {
                break;
            }
        }
    }
    -1
}

// Translated from 00dcc670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`: `this`, `name`): the other `hkIstream`
/// constructor. Sets the reference count (`+6`) to 1 and the vtable, then asks the global object
/// at `01268364` (virtual function at `+0xc`) for a reader for `name` and stores it at `+8`.
/// Answers `this`.
pub fn fn_00dcc670(e: &mut Engine, this: u32, name: u32) -> u32 {
    e.mem.set_u16(this + 6, 1);
    e.mem.set_u32(this, ISTREAM_VTABLE);
    let factory: u32 = e.global(READER_FACTORY);
    let reader = e.vcall(factory, 0xc, &args![name]).u32();
    e.mem.set_u32(this + 8, reader);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00dcc130, fn_00dcc130(u32, u32) -> u32),
        entry!(0x00dcc160, fn_00dcc160(u32)),
        entry!(0x00dcc1b0, fn_00dcc1b0(u32)),
        entry!(0x00dcc3b0, fn_00dcc3b0(u32) -> f32),
        entry!(0x00dcc4a0, fn_00dcc4a0(u32, u32) -> u32),
        entry!(0x00dcc5c0, fn_00dcc5c0(u32, u32) -> u32),
        entry!(0x00dcc5e0, fn_00dcc5e0(u32, u32) -> u32),
        entry!(0x00dcc600, fn_00dcc600(u32, u32, i32, u32) -> i32),
        entry!(0x00dcc670, fn_00dcc670(u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// A stream reader over a byte string: `read` copies and advances, `skip` advances,
    /// `setMark` remembers the position and `rewindToMark` goes back to it. Every virtual call
    /// is logged as text.
    struct Stream {
        data: Vec<u8>,
        position: usize,
        mark: usize,
        log: Vec<String>,
    }

    type Shared = Rc<RefCell<Stream>>;

    const READ_DOUBLE: u32 = 0x0072_0010;
    const SKIP_DOUBLE: u32 = 0x0072_0014;
    const MARK_DOUBLE: u32 = 0x0072_001c;
    const REWIND_DOUBLE: u32 = 0x0072_0020;

    fn engine_with_text(text: &[u8]) -> (Engine, u32, Shared) {
        let mut e = Engine::new();
        let stream: Shared = Rc::new(RefCell::new(Stream {
            data: text.to_vec(),
            position: 0,
            mark: 0,
            log: Vec::new(),
        }));
        e.put_vtable(
            0x0072_1000,
            &[
                0,
                0,
                0,
                0,
                READ_DOUBLE,
                SKIP_DOUBLE,
                0,
                MARK_DOUBLE,
                REWIND_DOUBLE,
            ],
        );
        let reader = e.mem.alloc(8);
        e.mem.set_u32(reader, 0x0072_1000);
        let s = Rc::clone(&stream);
        e.register_double(READ_DOUBLE, move |e, a| {
            let mut s = s.borrow_mut();
            let wanted = a[2] as usize;
            let available = s.data.len() - s.position;
            let n = wanted.min(available);
            for i in 0..n {
                let byte = s.data[s.position + i];
                e.mem.set_u8(a[1] + i as u32, byte);
            }
            s.position += n;
            s.log.push(format!("read {wanted} -> {n}"));
            Ret {
                eax: n as u32,
                ..Ret::default()
            }
        });
        let s = Rc::clone(&stream);
        e.register_double(SKIP_DOUBLE, move |_, a| {
            let mut s = s.borrow_mut();
            s.position += a[1] as usize;
            s.log.push(format!("skip {}", a[1]));
            Ret::default()
        });
        let s = Rc::clone(&stream);
        e.register_double(MARK_DOUBLE, move |_, a| {
            let mut s = s.borrow_mut();
            s.mark = s.position;
            s.log.push(format!("mark {}", a[1]));
            Ret::default()
        });
        let s = Rc::clone(&stream);
        e.register_double(REWIND_DOUBLE, move |_, _| {
            let mut s = s.borrow_mut();
            s.position = s.mark;
            s.log.push("rewind".to_string());
            Ret::default()
        });
        let s = Rc::clone(&stream);
        e.register_double(READ_CHARACTER, move |_, _| {
            let mut s = s.borrow_mut();
            let value = match s.data.get(s.position) {
                Some(&byte) => i32::from(byte as i8) as u32,
                None => 0xffff_ffff,
            };
            s.position += 1;
            Ret {
                eax: value,
                ..Ret::default()
            }
        });
        (e, reader, stream)
    }

    fn istream(e: &mut Engine, reader: u32) -> u32 {
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 8, reader);
        this
    }

    #[test]
    fn leading_white_space_is_skipped_up_to_the_first_other_byte() {
        let (mut e, reader, stream) = engine_with_text(b"  \t\r\n abc");
        e.call(0x00dc_c1b0, &args![reader]);
        let s = stream.borrow();
        assert_eq!(s.position, 6);
        assert_eq!(s.log, vec!["mark 64", "read 64 -> 9", "rewind", "skip 6"]);
    }

    #[test]
    fn white_space_longer_than_the_buffer_is_read_in_chunks() {
        let mut text = vec![b' '; 70];
        text.push(b'x');
        let (mut e, reader, stream) = engine_with_text(&text);
        e.call(0x00dc_c1b0, &args![reader]);
        let s = stream.borrow();
        assert_eq!(s.position, 70);
        assert_eq!(
            s.log,
            vec![
                "mark 64",
                "read 64 -> 64",
                "mark 64",
                "read 64 -> 7",
                "rewind",
                "skip 6"
            ]
        );
    }

    #[test]
    fn an_empty_stream_just_reads_once() {
        let (mut e, reader, stream) = engine_with_text(b"");
        e.call(0x00dc_c1b0, &args![reader]);
        assert_eq!(stream.borrow().log, vec!["mark 64", "read 64 -> 0"]);
    }

    fn set_up_number_reading(e: &mut Engine) {
        e.map(NO_NUMBER, 8);
        e.mem.set_f32(NO_NUMBER, -1.0);
        // The C library routine: parses the zero-terminated text with Rust.
        e.register_double(TEXT_TO_FLOAT, |e, a| {
            let text = String::from_utf8(e.mem.cstr(a[0])).unwrap();
            Ret {
                st0: text.parse::<f64>().unwrap(),
                ..Ret::default()
            }
        });
    }

    #[test]
    fn a_number_with_a_comma_and_an_exponent_is_taken_and_converted() {
        let (mut e, reader, stream) = engine_with_text(b"  3,5e2 rest");
        set_up_number_reading(&mut e);
        let value = e.call(0x00dc_c3b0, &args![reader]).f32();
        assert_eq!(value, 350.0);
        // Blanks skipped (2), then the five bytes of the number taken.
        assert_eq!(stream.borrow().position, 7);
        assert_eq!(
            stream.borrow().log[4..],
            ["mark 255", "read 255 -> 10", "rewind", "skip 5"]
        );
    }

    #[test]
    fn a_leading_sign_and_point_are_a_number() {
        let (mut e, reader, _) = engine_with_text(b"-.5");
        set_up_number_reading(&mut e);
        assert_eq!(e.call(0x00dc_c3b0, &args![reader]).f32(), -0.5);
    }

    #[test]
    fn text_that_does_not_start_like_a_number_answers_minus_one() {
        let (mut e, reader, stream) = engine_with_text(b"x1");
        set_up_number_reading(&mut e);
        assert_eq!(e.call(0x00dc_c3b0, &args![reader]).f32(), -1.0);
        assert_eq!(stream.borrow().position, 0);
        let (mut e, reader, _) = engine_with_text(b"");
        set_up_number_reading(&mut e);
        assert_eq!(e.call(0x00dc_c3b0, &args![reader]).f32(), -1.0);
    }

    #[test]
    fn a_number_that_ends_the_stream_is_converted_whole() {
        let (mut e, reader, stream) = engine_with_text(b"1.5");
        set_up_number_reading(&mut e);
        assert_eq!(e.call(0x00dc_c3b0, &args![reader]).f32(), 1.5);
        assert_eq!(stream.borrow().position, 3);
    }

    #[test]
    fn the_float_and_double_readers_store_what_the_number_reader_answers() {
        let (mut e, reader, _) = engine_with_text(b"2.25");
        set_up_number_reading(&mut e);
        let this = istream(&mut e, reader);
        let out = e.mem.alloc(8);
        assert_eq!(e.call(0x00dc_c5c0, &args![this, out]).u32(), this);
        assert_eq!(e.mem.f32(out), 2.25);
        let (mut e, reader, _) = engine_with_text(b"2.25");
        set_up_number_reading(&mut e);
        let this = istream(&mut e, reader);
        let out = e.mem.alloc(8);
        assert_eq!(e.call(0x00dc_c5e0, &args![this, out]).u32(), this);
        assert_eq!(e.mem.f64(out), 2.25);
    }

    /// Maps the two texts the boolean reader compares with and a `strncmp` double.
    fn set_up_booleans(e: &mut Engine) {
        e.map(0x0102_f000, 0x1000);
        e.mem.set_cstr(TRUE_TEXT, b"true");
        e.mem.set_cstr(FALSE_TEXT, b"false");
        e.register(STRING_COMPARE, |e, a| {
            for i in 0..a[2] {
                let (x, y) = (e.mem.u8(a[0] + i), e.mem.u8(a[1] + i));
                if x != y {
                    return Ret {
                        eax: i32::from(x).wrapping_sub(i32::from(y)) as u32,
                        ..Ret::default()
                    };
                }
                if x == 0 {
                    break;
                }
            }
            Ret::default()
        });
    }

    fn read_boolean(text: &[u8]) -> (Option<u8>, usize, Vec<String>) {
        let (mut e, reader, stream) = engine_with_text(text);
        set_up_booleans(&mut e);
        let this = istream(&mut e, reader);
        let out = e.mem.alloc(8);
        e.mem.set_u8(out, 7);
        assert_eq!(e.call(0x00dc_c4a0, &args![this, out]).u32(), this);
        let value = Some(e.mem.u8(out)).filter(|v| *v != 7);
        let s = stream.borrow();
        (value, s.position, s.log.clone())
    }

    #[test]
    fn true_and_false_are_read_when_they_end_the_stream_or_a_word() {
        let (value, position, _) = read_boolean(b"true");
        assert_eq!((value, position), (Some(1), 4));
        let (value, position, log) = read_boolean(b"  true next");
        assert_eq!((value, position), (Some(1), 6));
        assert_eq!(log[log.len() - 3..], ["read 6 -> 6", "rewind", "skip 4"]);
        let (value, position, _) = read_boolean(b"false\nx");
        assert_eq!((value, position), (Some(0), 5));
        let (value, position, _) = read_boolean(b"false");
        assert_eq!((value, position), (Some(0), 5));
    }

    #[test]
    fn other_words_just_rewind() {
        // "truex": four bytes match but the next byte is not white space.
        let (value, position, log) = read_boolean(b"truex");
        assert_eq!((value, position), (None, 0));
        assert_eq!(log.last().unwrap(), "rewind");
        // Fewer than four bytes: no comparison at all.
        let (value, position, _) = read_boolean(b"tru");
        assert_eq!((value, position), (None, 0));
        // "falsy ": only four bytes of `false` are compared ("fals"), so it is read as false.
        let (value, position, _) = read_boolean(b"falsy ");
        assert_eq!((value, position), (Some(0), 5));
    }

    fn read_until(text: &[u8], capacity: i32, delimiter: u32) -> (i32, Vec<u8>, usize) {
        let (mut e, reader, stream) = engine_with_text(text);
        let this = istream(&mut e, reader);
        let buffer = e.mem.alloc(16);
        for i in 0..16 {
            e.mem.set_u8(buffer + i, 0x55);
        }
        let n = e
            .call(0x00dc_c600, &args![this, buffer, capacity, delimiter])
            .i32();
        let bytes = e.mem.bytes(buffer, 8);
        let position = stream.borrow().position;
        (n, bytes, position)
    }

    #[test]
    fn characters_are_read_up_to_the_delimiter() {
        let (n, bytes, _) = read_until(b"abc,def", 8, u32::from(b','));
        assert_eq!(n, 3);
        assert_eq!(bytes[..5], [b'a', b'b', b'c', 0, 0x55]);
        // The end of the stream also ends the text.
        let (n, bytes, _) = read_until(b"ab", 8, u32::from(b','));
        assert_eq!(n, 2);
        assert_eq!(bytes[..4], [b'a', b'b', 0, 0x55]);
    }

    #[test]
    fn a_full_buffer_answers_minus_one_and_writes_no_end() {
        let (n, bytes, _) = read_until(b"abc,", 2, u32::from(b','));
        assert_eq!(n, -1);
        assert_eq!(bytes[..3], [b'a', b'b', 0x55]);
        let (n, bytes, position) = read_until(b"abc,", 0, u32::from(b','));
        assert_eq!((n, bytes[0]), (-1, 0x55));
        assert_eq!(position, 0);
        // The delimiter is a signed byte.
        let (n, _, _) = read_until(&[b'a', 0xe9, b'b'], 8, 0xffff_ffe9);
        assert_eq!(n, 1);
    }

    #[test]
    fn the_constructors_and_the_destructor_keep_the_reader_referenced() {
        let mut e = Engine::new();
        let calls: Rc<RefCell<Vec<(u32, u32)>>> = Rc::default();
        for address in [ADD_REFERENCE, REMOVE_REFERENCE] {
            let seen = Rc::clone(&calls);
            e.register_double(address, move |_, a| {
                seen.borrow_mut().push((address, a[0]));
                Ret::default()
            });
        }
        let this = e.mem.alloc(0x10);
        assert_eq!(e.call(0x00dc_c130, &args![this, 0x4242u32]).u32(), this);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this), ISTREAM_VTABLE);
        assert_eq!(e.mem.u32(this + 8), 0x4242);
        e.call(0x00dc_c160, &args![this]);
        assert_eq!(e.mem.u32(this), BASE_OBJECT_VTABLE);
        e.mem.set_u32(this + 8, 0);
        e.call(0x00dc_c160, &args![this]);
        assert_eq!(
            *calls.borrow(),
            vec![(ADD_REFERENCE, 0x4242), (REMOVE_REFERENCE, 0x4242)]
        );
    }

    #[test]
    fn the_second_constructor_asks_the_global_for_a_reader() {
        let mut e = Engine::new();
        e.map(READER_FACTORY, 4);
        let factory = e.mem.alloc(8);
        e.put_vtable(0x0072_2000, &[0, 0, 0, 0x0072_2100]);
        e.mem.set_u32(factory, 0x0072_2000);
        e.set_global(READER_FACTORY, factory);
        e.register_double(0x0072_2100, |_, a| Ret {
            eax: a[1] + 1,
            ..Ret::default()
        });
        let this = e.mem.alloc(0x10);
        assert_eq!(e.call(0x00dc_c670, &args![this, 0x5000u32]).u32(), this);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this), ISTREAM_VTABLE);
        assert_eq!(e.mem.u32(this + 8), 0x5001);
    }
}

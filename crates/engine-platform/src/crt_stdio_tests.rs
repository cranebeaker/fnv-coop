//! The tests of the C runtime's stdio (`engine-core`'s `units/crt/stdio.rs`) that run on the
//! platform layer's files (`CreateFileA`, `ReadFile`, ... and the run's disk, [`crate::files`]):
//! moved here with the files, since the core's own tests have no platform layer.

use crate::prelude::*;
use engine_core::units::crt::stdio::test_support::*;

fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
    let log = e.call_log.as_ref().unwrap();
    log.iter()
        .filter(|c| c.0 == addr)
        .map(|c| c.1.clone())
        .collect()
}

/// The invalid-parameter handler the doubled `__decode_pointer` answers.
const HANDLER: u32 = 0x0e30_0000;

/// An engine with the CRT's data pages mapped, a thread block (`errno` +8, `_doserrno`
/// +0xc), the locks 1, 0xa and 0xb set up, one `ioinfo` block of 32 closed entries, three
/// busy streams and a free `__piob` slot, `_get_fmode` (in another crate) doubled to the
/// text default, and the platform's own file, error and critical-section calls at their
/// import slots. Answers the engine and the thread block.
fn engine() -> (Engine, u32) {
    let mut e = Engine::new();
    e.map(0x0126_f000, 0x2000);
    e.map(0x011c_1000, 0x2000);
    e.map(0x0127_0000, 0x2000);
    let block = e.mem.alloc(0x40);
    e.register_double(GETPTD_NOEXIT, move |_, _| block.into_ret());
    e.register(DECODE_POINTER, |_, _| HANDLER.into_ret());
    e.register(HANDLER, |_, _| Ret::default());
    e.mem.set_u32(LOCKS_READY, 1);
    for number in [1u32, 0xa, 0xb] {
        let section = e.mem.alloc(0x18);
        e.mem.set_u32(LOCK_TABLE + 8 * number, section);
    }
    let entries = e.mem.alloc(0x800);
    for index in 0..0x20 {
        e.mem.set_u32(entries + 0x40 * index, 0xffff_ffff);
    }
    e.mem.set_u32(PIOINFO, entries);
    e.mem.set_u32(NHANDLE, 0x20);
    let piob = e.mem.alloc(0x10);
    for index in 0..3 {
        let busy = e.mem.alloc(0x38);
        e.mem.set_u32(busy + 0xc, 1);
        e.mem.set_u32(piob + 4 * index, busy);
    }
    e.mem.set_u32(PIOB, piob);
    e.mem.set_u32(NSTREAM, 4);
    e.register(GET_FMODE, |e, a| {
        e.mem.set_u32(a[0], 0);
        0u32.into_ret()
    });
    let slots = [
        (IMPORT_CREATE_FILE_A, "CreateFileA"),
        (IMPORT_GET_FILE_TYPE, "GetFileType"),
        (IMPORT_SET_FILE_POINTER, "SetFilePointer"),
        (IMPORT_READ_FILE, "ReadFile"),
        (IMPORT_CLOSE_HANDLE, "CloseHandle"),
        (IMPORT_GET_LAST_ERROR, "GetLastError"),
        (IMPORT_ENTER_CRITICAL_SECTION, "EnterCriticalSection"),
        (IMPORT_LEAVE_CRITICAL_SECTION, "LeaveCriticalSection"),
        (
            IMPORT_INIT_CRITICAL_SECTION_SPIN,
            "InitializeCriticalSectionAndSpinCount",
        ),
    ];
    let imports: Vec<crate::exe::Import> = slots
        .iter()
        .map(|&(slot, name)| crate::exe::Import {
            slot,
            dll: "KERNEL32.dll".into(),
            name: name.into(),
        })
        .collect();
    assert_eq!(e.register_imports(&imports), slots.len());
    e.call_log = Some(vec![]);
    (e, block)
}

/// `C:\Test\<name>` holding `data` on the run's disk; answers the path's address.
fn file(e: &mut Engine, name: &str, data: &[u8]) -> u32 {
    let text = format!("C:\\Test\\{name}");
    let path = e.mem.alloc(0x40);
    e.mem.set_cstr(path, text.as_bytes());
    state!(e)
        .disk
        .files
        .insert(text.to_ascii_lowercase().into_bytes(), data.to_vec());
    path
}

/// The file opened read-only through `_sopen_s` with the flags (`_O_TEXT` 0x4000 or
/// `_O_BINARY` 0x8000), `_SH_DENYNO`; answers the CRT handle.
fn open(e: &mut Engine, name: &str, data: &[u8], flags: u32) -> u32 {
    let path = file(e, name, data);
    let handle = e.mem.alloc(4);
    let r = e
        .call(0x00ee_3908, &args![handle, path, flags, 0x40u32, 0x180u32])
        .u32();
    assert_eq!(r, 0);
    e.mem.u32(handle)
}

#[test]
fn plugins_txt_reads_line_by_line_until_feof() {
    let (mut e, _) = engine();
    let path = file(
        &mut e,
        "Plugins.txt",
        b"# Comment\r\nFalloutNV.esm\r\n\r\nDeadMoney.esm\r\n",
    );
    let mode = e.mem.alloc(4);
    e.mem.set_cstr(mode, b"r");
    let out = e.mem.alloc(4);
    assert_eq!(e.call(0x00ec_9a5e, &args![out, path, mode]).u32(), 0);
    let stream = e.mem.u32(out);
    assert_ne!(stream, 0);
    // _IOREAD, the handle's ioinfo in text mode (FTEXT | FOPEN).
    assert_eq!(e.mem.u32(stream + 0xc), 1);
    let handle = e.mem.u32(stream + 0x10);
    assert_eq!(e.mem.u8(ioinfo(&e, handle) + 4), 0x81);
    let buffer = e.mem.alloc(0x104);
    let mut lines = vec![];
    // As 00af4550 reads it: while (!feof(f)) fgets(buf, 0x104, f).
    while e.call(0x00ec_dacc, &args![stream]).u32() == 0 {
        let r = e.call(0x00ec_c104, &args![buffer, 0x104u32, stream]).u32();
        lines.push((r, e.mem.cstr(buffer)));
    }
    let expected: Vec<(u32, Vec<u8>)> = vec![
        (buffer, b"# Comment\n".to_vec()),
        (buffer, b"FalloutNV.esm\n".to_vec()),
        (buffer, b"\n".to_vec()),
        (buffer, b"DeadMoney.esm\n".to_vec()),
        // The end comes with the fifth fgets: null, the buffer keeps the last line.
        (0, b"DeadMoney.esm\n".to_vec()),
    ];
    assert_eq!(lines, expected);
    assert_eq!(e.call(0x00ec_dacc, &args![stream]).u32(), 0x10);
    // One read of the whole file into the 0x1000-byte buffer, one that finds the end.
    let reads = calls(&e, IMPORT_READ_FILE);
    assert_eq!(reads.len(), 2);
    assert_eq!(reads[0][2], 0x1000);
    assert_eq!(e.mem.u32(stream + 0x18), 0x1000);
}

#[test]
fn a_last_line_without_newline_sets_feof_with_it() {
    let (mut e, _) = engine();
    let path = file(&mut e, "Short.txt", b"a\r\nlast");
    let mode = e.mem.alloc(4);
    e.mem.set_cstr(mode, b"r");
    let out = e.mem.alloc(4);
    e.call(0x00ec_9a5e, &args![out, path, mode]);
    let stream = e.mem.u32(out);
    let buffer = e.mem.alloc(0x104);
    assert_eq!(
        e.call(0x00ec_c104, &args![buffer, 0x104u32, stream]).u32(),
        buffer
    );
    assert_eq!(e.mem.cstr(buffer), b"a\n".to_vec());
    assert_eq!(e.call(0x00ec_dacc, &args![stream]).u32(), 0);
    assert_eq!(
        e.call(0x00ec_c104, &args![buffer, 0x104u32, stream]).u32(),
        buffer
    );
    assert_eq!(e.mem.cstr(buffer), b"last".to_vec());
    assert_eq!(e.call(0x00ec_dacc, &args![stream]).u32(), 0x10);
}

#[test]
fn fopen_s_and_feof_check_their_arguments() {
    let (mut e, block) = engine();
    assert_eq!(e.call(0x00ec_9a5e, &args![0u32, 1u32, 2u32]).u32(), 0x16);
    assert_eq!(e.mem.u32(block + 8), 0x16);
    assert_eq!(calls(&e, HANDLER).len(), 1);
    // _fsopen failing: its errno is the answer, the out pointer null.
    e.register(FSOPEN, |e, _| {
        let slot = errno(e);
        e.mem.set_u32(slot.addr(), 0x18);
        0u32.into_ret()
    });
    let out = e.mem.alloc(4);
    e.mem.set_u32(out, 0x1234);
    assert_eq!(e.call(0x00ec_9a5e, &args![out, 5u32, 6u32]).u32(), 0x18);
    assert_eq!(e.mem.u32(out), 0);
    assert_eq!(calls(&e, FSOPEN)[0], vec![5, 6, 0x80]);
    assert_eq!(e.call(0x00ec_dacc, &args![0u32]).u32(), 0);
    assert_eq!(calls(&e, HANDLER).len(), 2);
    let stream = e.mem.alloc(0x38);
    e.mem.set_u32(stream + 0xc, 0x19);
    assert_eq!(e.call(0x00ec_dacc, &args![stream]).u32(), 0x10);
}

/// A stream on handle 3 (text, ANSI) reading `data` from its buffer; `_filbuf` doubled to
/// the end (-1).
fn buffered_stream(e: &mut Engine, data: &[u8]) -> u32 {
    let entries = e.mem.u32(PIOINFO);
    e.mem.set_u32(entries + 0xc0, 0x600);
    e.mem.set_u8(entries + 0xc4, 0x81);
    let stream = e.mem.alloc(0x38);
    let base = e.mem.alloc(0x40);
    e.mem.write(base, data);
    e.mem.set_u32(stream, base);
    e.mem.set_u32(stream + 4, data.len() as u32);
    e.mem.set_u32(stream + 8, base);
    e.mem.set_u32(stream + 0xc, 9);
    e.mem.set_u32(stream + 0x10, 3);
    e.register(FILBUF, |_, _| 0xffff_ffffu32.into_ret());
    stream
}

#[test]
fn fgets_stops_at_newline_count_and_end() {
    let (mut e, _) = engine();
    let stream = buffered_stream(&mut e, b"ab\ncdefg");
    let buffer = e.mem.alloc(0x20);
    assert_eq!(
        e.call(0x00ec_c104, &args![buffer, 0x20u32, stream]).u32(),
        buffer
    );
    assert_eq!(e.mem.cstr(buffer), b"ab\n".to_vec());
    // Count 3: two characters and the zero.
    assert_eq!(
        e.call(0x00ec_c104, &args![buffer, 3u32, stream]).u32(),
        buffer
    );
    assert_eq!(e.mem.cstr(buffer), b"cd".to_vec());
    assert_eq!(e.mem.u32(stream + 4), 3);
    // The end after some characters ends the line.
    assert_eq!(
        e.call(0x00ec_c104, &args![buffer, 0x20u32, stream]).u32(),
        buffer
    );
    assert_eq!(e.mem.cstr(buffer), b"efg".to_vec());
    assert_eq!(calls(&e, FILBUF), vec![vec![stream]]);
    // The end first: null, the buffer untouched.
    assert_eq!(
        e.call(0x00ec_c104, &args![buffer, 0x20u32, stream]).u32(),
        0
    );
    assert_eq!(e.mem.cstr(buffer), b"efg".to_vec());
    // Count 1: only the zero.
    assert_eq!(
        e.call(0x00ec_c104, &args![buffer, 1u32, stream]).u32(),
        buffer
    );
    assert_eq!(e.mem.u8(buffer), 0);
    // Each call locked and unlocked the stream (a heap stream: its critical section).
    assert_eq!(
        calls(&e, IMPORT_ENTER_CRITICAL_SECTION),
        vec![vec![stream + 0x20]; 5]
    );
    assert_eq!(
        calls(&e, IMPORT_LEAVE_CRITICAL_SECTION),
        vec![vec![stream + 0x20]; 5]
    );
}

#[test]
fn fgets_refuses_bad_arguments_and_unicode_handles() {
    let (mut e, block) = engine();
    let stream = buffered_stream(&mut e, b"ab\n");
    let buffer = e.mem.alloc(0x20);
    assert_eq!(e.call(0x00ec_c104, &args![buffer, 0u32, stream]).u32(), 0);
    assert!(calls(&e, IMPORT_ENTER_CRITICAL_SECTION).is_empty());
    assert!(calls(&e, HANDLER).is_empty());
    for a in [
        args![buffer, 0xffff_ffffu32, stream],
        args![0u32, 4u32, stream],
        args![buffer, 4u32, 0u32],
    ] {
        assert_eq!(e.call(0x00ec_c104, &a).u32(), 0);
    }
    assert_eq!(calls(&e, HANDLER).len(), 3);
    assert_eq!(e.mem.u32(block + 8), 0x16);
    // A null buffer with count 0 is just null.
    assert_eq!(e.call(0x00ec_c104, &args![0u32, 0u32, stream]).u32(), 0);
    assert_eq!(calls(&e, HANDLER).len(), 3);
    // A UTF-8 or Unicode handle: EINVAL, nothing read, unlocked.
    let entries = e.mem.u32(PIOINFO);
    for modes in [1u8, 0x80] {
        e.mem.set_u8(entries + 0xc0 + 0x24, modes);
        assert_eq!(e.call(0x00ec_c104, &args![buffer, 4u32, stream]).u32(), 0);
        assert_eq!(e.mem.u32(stream + 4), 3);
    }
    assert_eq!(calls(&e, HANDLER).len(), 5);
    // A string stream (_IOSTRG) is not checked.
    e.mem.set_u32(stream + 0xc, 0x49);
    assert_eq!(
        e.call(0x00ec_c104, &args![buffer, 4u32, stream]).u32(),
        buffer
    );
    assert_eq!(e.mem.cstr(buffer), b"ab\n".to_vec());
    assert_eq!(
        calls(&e, IMPORT_ENTER_CRITICAL_SECTION).len(),
        calls(&e, IMPORT_LEAVE_CRITICAL_SECTION).len()
    );
}

#[test]
fn strtok_s_splits_on_delimiters() {
    let (mut e, block) = engine();
    let text = e.mem.alloc(0x20);
    e.mem.set_cstr(text, b"a.bsa, b.bsa,,c");
    let comma = e.mem.alloc(4);
    e.mem.set_cstr(comma, b",");
    let context = e.mem.alloc(4);
    let mut tokens = vec![];
    let mut r = e.call(0x00ec_c5f8, &args![text, comma, context]).u32();
    while r != 0 {
        tokens.push(e.mem.cstr(r));
        r = e.call(0x00ec_c5f8, &args![0u32, comma, context]).u32();
    }
    assert_eq!(
        tokens,
        vec![b"a.bsa".to_vec(), b" b.bsa".to_vec(), b"c".to_vec()]
    );
    // The context is left at the end; the separators became zeros.
    assert_eq!(e.mem.u32(context), text + 15);
    assert_eq!(e.mem.u8(text + 5), 0);
    assert_eq!(e.mem.u8(text + 12), 0);
    // Only delimiters: null, the context past them.
    e.mem.set_cstr(text, b",,");
    assert_eq!(e.call(0x00ec_c5f8, &args![text, comma, context]).u32(), 0);
    assert_eq!(e.mem.u32(context), text + 2);
    // A null context, delimiters, or string with a null *context: EINVAL.
    e.mem.set_u32(context, 0);
    for a in [
        args![text, comma, 0u32],
        args![text, 0u32, context],
        args![0u32, comma, context],
    ] {
        assert_eq!(e.call(0x00ec_c5f8, &a).u32(), 0);
    }
    assert_eq!(calls(&e, HANDLER).len(), 3);
    assert_eq!(e.mem.u32(block + 8), 0x16);
}

#[test]
fn read_joins_cr_lf_across_the_end_of_a_read() {
    let (mut e, _) = engine();
    let h = open(&mut e, "a.txt", b"ab\r\ncd", 0x4000);
    let buffer = e.mem.alloc(0x10);
    // The CR is the last byte read: the look-ahead finds LF, the file moves back one and
    // the CR is dropped.
    assert_eq!(e.call(0x00ed_cf0c, &args![h, buffer, 3u32]).i32(), 2);
    assert_eq!(e.mem.bytes(buffer, 2), b"ab".to_vec());
    let seeks = calls(&e, IMPORT_SET_FILE_POINTER);
    assert_eq!(seeks.last().unwrap()[1], 0xffff_ffff);
    assert_eq!(seeks.last().unwrap()[3], 1);
    // The LF starts the next read: FCRLF.
    assert_eq!(e.call(0x00ed_cf0c, &args![h, buffer, 8u32]).i32(), 3);
    assert_eq!(e.mem.bytes(buffer, 3), b"\ncd".to_vec());
    assert_eq!(e.mem.u8(ioinfo(&e, h) + 4) & 4, 4);
    assert_eq!(e.call(0x00ed_cf0c, &args![h, buffer, 8u32]).i32(), 0);
    assert_eq!(e.mem.u8(ioinfo(&e, h) + 4) & 4, 0);
}

#[test]
fn read_keeps_lone_crs_and_stops_at_ctrl_z() {
    let (mut e, _) = engine();
    let buffer = e.mem.alloc(0x10);
    // A CR alone at the start with an LF after it: LF, nothing moved back.
    let h = open(&mut e, "b.txt", b"\r\nx", 0x4000);
    let seeks = calls(&e, IMPORT_SET_FILE_POINTER).len();
    assert_eq!(e.call(0x00ed_cf0c, &args![h, buffer, 1u32]).i32(), 1);
    assert_eq!(e.mem.u8(buffer), b'\n');
    assert_eq!(calls(&e, IMPORT_SET_FILE_POINTER).len(), seeks);
    assert_eq!(e.call(0x00ed_cf0c, &args![h, buffer, 4u32]).i32(), 1);
    assert_eq!(e.mem.u8(buffer), b'x');
    // A CR before another character, and a CR at the very end of the file, stay.
    let h = open(&mut e, "c.txt", b"a\rb\r", 0x4000);
    assert_eq!(e.call(0x00ed_cf0c, &args![h, buffer, 8u32]).i32(), 4);
    assert_eq!(e.mem.bytes(buffer, 4), b"a\rb\r".to_vec());
    // Ctrl-Z ends a text file for good.
    let h = open(&mut e, "d.txt", b"ab\x1acd", 0x4000);
    assert_eq!(e.call(0x00ed_cf0c, &args![h, buffer, 8u32]).i32(), 2);
    assert_eq!(e.mem.u8(ioinfo(&e, h) + 4) & 2, 2);
    let reads = calls(&e, IMPORT_READ_FILE).len();
    assert_eq!(e.call(0x00ed_cf0c, &args![h, buffer, 8u32]).i32(), 0);
    assert_eq!(calls(&e, IMPORT_READ_FILE).len(), reads);
    // Binary: as read.
    let h = open(&mut e, "e.bin", b"a\r\n\x1a", 0x8000);
    assert_eq!(e.call(0x00ed_cf0c, &args![h, buffer, 8u32]).i32(), 4);
    assert_eq!(e.mem.bytes(buffer, 4), b"a\r\n\x1a".to_vec());
}

#[test]
fn read_on_a_pipe_keeps_the_look_ahead_byte() {
    let (mut e, _) = engine();
    let buffer = e.mem.alloc(0x10);
    let h = open(&mut e, "pipe.txt", b"ab\rx", 0x4000);
    // FPIPE.
    let flags = e.mem.u8(ioinfo(&e, h) + 4) | 8;
    e.mem.set_u8(ioinfo(&e, h) + 4, flags);
    e.mem.set_u8(ioinfo(&e, h) + 5, 0xa);
    assert_eq!(e.call(0x00ed_cf0c, &args![h, buffer, 3u32]).i32(), 3);
    assert_eq!(e.mem.bytes(buffer, 3), b"ab\r".to_vec());
    assert_eq!(e.mem.u8(ioinfo(&e, h) + 5), b'x');
    // The kept byte comes first in the next read.
    assert_eq!(e.call(0x00ed_cf0c, &args![h, buffer, 4u32]).i32(), 1);
    assert_eq!(e.mem.u8(buffer), b'x');
    assert_eq!(e.mem.u8(ioinfo(&e, h) + 5), 0xa);
}

#[test]
fn read_utf16_joins_cr_lf_units() {
    let (mut e, _) = engine();
    let buffer = e.mem.alloc(0x10);
    let h = open(&mut e, "w.txt", b"a\0\r\0\n\0b\0c\0\r\0\n\0", 0x4000);
    e.mem.set_u8(ioinfo(&e, h) + 0x24, 2);
    // Odd counts are refused.
    assert_eq!(e.call(0x00ed_cf0c, &args![h, buffer, 7u32]).i32(), -1);
    assert_eq!(e.call(0x00ed_cf0c, &args![h, buffer, 12u32]).i32(), 8);
    assert_eq!(e.mem.bytes(buffer, 8), b"a\0\n\0b\0c\0".to_vec());
    let seeks = calls(&e, IMPORT_SET_FILE_POINTER);
    assert_eq!(seeks.last().unwrap()[1], 0xffff_fffe);
    assert_eq!(e.call(0x00ed_cf0c, &args![h, buffer, 12u32]).i32(), 2);
    assert_eq!(e.mem.bytes(buffer, 2), b"\n\0".to_vec());
}

#[test]
fn read_utf8_moves_back_over_a_split_sequence() {
    let (mut e, _) = engine();
    for lead in 0xc0u32..0xf8 {
        let trail = if lead < 0xe0 {
            1
        } else if lead < 0xf0 {
            2
        } else {
            3
        };
        e.mem.set_u8(UTF8_TRAIL_BYTES + lead, trail);
    }
    e.register(IMPORT_MULTI_BYTE_TO_WIDE_CHAR, |_, a| {
        (if a[3] == 3 { 3u32 } else { 1 }).into_ret()
    });
    let buffer = e.mem.alloc(0x10);
    let h = open(&mut e, "u.txt", b"abc\xc3\xa9", 0x4000);
    e.mem.set_u8(ioinfo(&e, h) + 0x24, 1);
    // Count 8: a buffer of 4 bytes, "abc" and the lead of a two-byte sequence.
    assert_eq!(e.call(0x00ed_cf0c, &args![h, buffer, 8u32]).i32(), 6);
    assert_eq!(calls(&e, IMPORT_READ_FILE)[0][2], 4);
    let converted = calls(&e, IMPORT_MULTI_BYTE_TO_WIDE_CHAR);
    assert_eq!(converted[0][..4], [0xfde9, 0, converted[0][2], 3]);
    assert_eq!(converted[0][4..], [buffer, 4]);
    assert_eq!(e.mem.u32(ioinfo(&e, h) + 0x30), 0);
    assert_eq!(e.mem.u32(ioinfo(&e, h) + 0x28), 0);
    // The file moved back one: the whole sequence comes next.
    assert_eq!(e.call(0x00ed_cf0c, &args![h, buffer, 8u32]).i32(), 2);
    assert_eq!(calls(&e, IMPORT_MULTI_BYTE_TO_WIDE_CHAR)[1][3], 2);
    assert_eq!(e.mem.u32(ioinfo(&e, h) + 0x30), 1);
    assert_eq!(e.mem.u32(ioinfo(&e, h) + 0x28), 3);
    // Odd counts are refused.
    assert_eq!(e.call(0x00ed_cf0c, &args![h, buffer, 7u32]).i32(), -1);
}

#[test]
fn read_reports_bad_handles_and_failed_reads() {
    let (mut e, block) = engine();
    let buffer = e.mem.alloc(0x10);
    assert_eq!(
        e.call(0x00ed_cf0c, &args![0xffff_fffeu32, buffer, 1u32])
            .i32(),
        -1
    );
    assert_eq!(e.mem.u32(block + 8), 9);
    assert!(calls(&e, HANDLER).is_empty());
    // Out of range, not open, too large a count.
    assert_eq!(e.call(0x00ed_cf0c, &args![0x20u32, buffer, 1u32]).i32(), -1);
    assert_eq!(e.call(0x00ed_cf0c, &args![2u32, buffer, 1u32]).i32(), -1);
    assert_eq!(e.mem.u32(block + 8), 9);
    let h = open(&mut e, "f.txt", b"abc", 0x4000);
    assert_eq!(
        e.call(0x00ed_cf0c, &args![h, buffer, 0x8000_0000u32]).i32(),
        -1
    );
    assert_eq!(e.mem.u32(block + 8), 0x16);
    assert_eq!(calls(&e, HANDLER).len(), 3);
    // A count of 0 answers 0; a null buffer is EINVAL.
    assert_eq!(e.call(0x00ed_cf0c, &args![h, buffer, 0u32]).i32(), 0);
    assert_eq!(e.call(0x00ed_cf0c, &args![h, 0u32, 4u32]).i32(), -1);
    assert_eq!(calls(&e, HANDLER).len(), 4);
    // ReadFile failing: a broken pipe reads as the end, access denied is EBADF, the
    // rest is mapped.
    e.register(IMPORT_READ_FILE, |_, _| 0u32.into_ret());
    e.register(IMPORT_GET_LAST_ERROR, |_, _| 0x6du32.into_ret());
    assert_eq!(e.call(0x00ed_cf0c, &args![h, buffer, 4u32]).i32(), 0);
    e.register(IMPORT_GET_LAST_ERROR, |_, _| 5u32.into_ret());
    assert_eq!(e.call(0x00ed_cf0c, &args![h, buffer, 4u32]).i32(), -1);
    assert_eq!(e.mem.u32(block + 8), 9);
    assert_eq!(e.mem.u32(block + 0xc), 5);
    e.register(IMPORT_GET_LAST_ERROR, |_, _| 2u32.into_ret());
    assert_eq!(e.call(0x00ed_cf0c, &args![h, buffer, 4u32]).i32(), -1);
    assert_eq!(e.mem.u32(block + 0xc), 2);
    // A count read above the count asked is a failure too.
    e.register(IMPORT_READ_FILE, |e, a| {
        e.mem.set_u32(a[3], a[2] + 1);
        1u32.into_ret()
    });
    assert_eq!(e.call(0x00ed_cf0c, &args![h, buffer, 4u32]).i32(), -1);
    // Every read took and left the handle's lock.
    assert_eq!(
        calls(&e, IMPORT_ENTER_CRITICAL_SECTION).len(),
        calls(&e, IMPORT_LEAVE_CRITICAL_SECTION).len()
    );
}

/// A binary stream (`rb`) over `data`: answers the stream.
fn open_binary(e: &mut Engine, data: &[u8]) -> u32 {
    let path = file(e, "Archive.bsa", data);
    let mode = e.mem.alloc(4);
    e.mem.set_cstr(mode, b"rb");
    let out = e.mem.alloc(4);
    assert_eq!(e.call(0x00ec_9a5e, &args![out, path, mode]).u32(), 0);
    e.mem.u32(out)
}

#[test]
fn fread_fseek_and_ftell_walk_a_binary_file() {
    let (mut e, _) = engine();
    let data: Vec<u8> = (0..0x3000u32).map(|i| (i * 7) as u8).collect();
    let stream = open_binary(&mut e, &data);
    let buffer = e.mem.alloc(0x2000);
    // The size as BSFile asks for it: SEEK_END then ftell, and back.
    assert_eq!(e.call(0x00ec_b562, &args![stream]).u32(), 0);
    assert_eq!(e.call(0x00ec_b65a, &args![stream, 0u32, 2u32]).u32(), 0);
    assert_eq!(e.call(0x00ec_b562, &args![stream]).u32(), 0x3000);
    assert_eq!(e.call(0x00ec_b65a, &args![stream, 0u32, 0u32]).u32(), 0);
    // A small read goes through the stream's buffer (0x1000 bytes).
    assert_eq!(
        e.call(0x00ec_b3a8, &args![buffer, 1u32, 0x24u32, stream])
            .u32(),
        0x24
    );
    assert_eq!(e.mem.bytes(buffer, 0x24), data[..0x24].to_vec());
    assert_eq!(e.call(0x00ec_b562, &args![stream]).u32(), 0x24);
    // SEEK_CUR from there, then a read larger than the buffer goes to the handle.
    assert_eq!(e.call(0x00ec_b65a, &args![stream, 0x10u32, 1u32]).u32(), 0);
    assert_eq!(e.call(0x00ec_b562, &args![stream]).u32(), 0x34);
    assert_eq!(
        e.call(0x00ec_b3a8, &args![buffer, 0x10u32, 0x180u32, stream])
            .u32(),
        0x180
    );
    assert_eq!(e.mem.bytes(buffer, 0x1800), data[0x34..0x1834].to_vec());
    assert_eq!(e.call(0x00ec_b562, &args![stream]).u32(), 0x1834);
    // Past the end: the whole elements read, and _IOEOF.
    assert_eq!(
        e.call(0x00ec_b3a8, &args![buffer, 0x100u32, 0x20u32, stream])
            .u32(),
        0x17
    );
    assert_ne!(e.mem.u32(stream + 0xc) & 0x10, 0);
    // A bad origin and a null stream are refused.
    assert_eq!(
        e.call(0x00ec_b65a, &args![stream, 0u32, 3u32]).u32(),
        0xffff_ffff
    );
    assert_eq!(e.call(0x00ec_b562, &args![0u32]).u32(), 0xffff_ffff);
    assert_eq!(
        calls(&e, IMPORT_ENTER_CRITICAL_SECTION).len(),
        calls(&e, IMPORT_LEAVE_CRITICAL_SECTION).len()
    );
}

#[test]
fn fread_s_refuses_a_destination_too_small() {
    let (mut e, block) = engine();
    let stream = open_binary(&mut e, &[1, 2, 3, 4, 5, 6, 7, 8]);
    let buffer = e.mem.alloc(0x10);
    e.mem.write(buffer, &[0xaa; 8]);
    assert_eq!(
        e.call(0x00ec_b312, &args![buffer, 4u32, 1u32, 8u32, stream])
            .u32(),
        0
    );
    assert_eq!(e.mem.u32(block + 8), 0x22);
    assert_eq!(e.mem.bytes(buffer, 8), [0, 0, 0, 0, 0xaa, 0xaa, 0xaa, 0xaa]);
    // Zero elements read nothing; a null stream zeroes the destination.
    assert_eq!(
        e.call(0x00ec_b312, &args![buffer, 8u32, 0u32, 8u32, stream])
            .u32(),
        0
    );
    e.mem.write(buffer, &[0xaa; 8]);
    assert_eq!(
        e.call(0x00ec_b312, &args![buffer, 8u32, 1u32, 8u32, 0u32])
            .u32(),
        0
    );
    assert_eq!(e.mem.bytes(buffer, 8), [0; 8]);
    assert_eq!(e.mem.u32(block + 8), 0x16);
}

#[test]
fn memcpy_s_copies_or_zeroes_with_the_error() {
    let (mut e, block) = engine();
    let to = e.mem.alloc(8);
    let from = e.mem.alloc(8);
    e.mem.write(from, b"abcdefgh");
    assert_eq!(e.call(0x00ec_7c66, &args![to, 8u32, from, 0u32]).u32(), 0);
    assert_eq!(e.call(0x00ec_7c66, &args![to, 8u32, from, 4u32]).u32(), 0);
    assert_eq!(e.mem.bytes(to, 4), b"abcd".to_vec());
    assert_eq!(
        e.call(0x00ec_7c66, &args![to, 2u32, from, 4u32]).u32(),
        0x22
    );
    assert_eq!(e.mem.bytes(to, 4), [0, 0, b'c', b'd']);
    assert_eq!(e.mem.u32(block + 8), 0x22);
    assert_eq!(
        e.call(0x00ec_7c66, &args![to, 8u32, 0u32, 4u32]).u32(),
        0x16
    );
    assert_eq!(e.mem.bytes(to, 4), [0; 4]);
    assert_eq!(
        e.call(0x00ec_7c66, &args![0u32, 8u32, from, 4u32]).u32(),
        0x16
    );
}

#[test]
fn printf_writes_stdout_through_output_l_with_its_arguments() {
    let (mut e, block) = engine();
    let section = e.mem.alloc(0x18);
    e.mem.set_u32(LOCK_TABLE + 8 * 0x11, section);
    let iob = e.mem.alloc(0x60);
    e.register_double(IOB_FUNC, move |_, _| iob.into_ret());
    e.register(STBUF, |_, _| 1u32.into_ret());
    e.register(FTBUF, |_, _| Ret::default());
    e.register(OUTPUT_L, |e, a| {
        // The va_list holds the arguments: answer the first one.
        e.mem.u32(a[3]).into_ret()
    });
    let format = e.mem.alloc(8);
    e.mem.set_cstr(format, b"%i\n");
    assert_eq!(e.call(0x00ec_9d4a, &args![format, 0x2au32]).i32(), 0x2a);
    assert_eq!(calls(&e, OUTPUT_L)[0][..3], [iob + 0x20, format, 0]);
    assert_eq!(calls(&e, FTBUF), [vec![1, iob + 0x20]]);
    // The stream was unlocked again.
    assert_eq!(e.mem.u32(iob + 0x20 + 0xc) & 0x8000, 0);
    assert_eq!(e.call(0x00ec_9d4a, &args![0u32]).i32(), -1);
    assert_eq!(e.mem.u32(block + 8), 0x16);
}

#[test]
fn rename_moves_the_file_or_maps_the_windows_error() {
    let mut e = Engine::new();
    e.call_log = Some(vec![]);
    let moved = std::rc::Rc::new(std::cell::Cell::new(1u32));
    let answer = moved.clone();
    e.register_double(IMPORT_MOVE_FILE_A, move |_, _| answer.get().into_ret());
    e.register(IMPORT_GET_LAST_ERROR, |_, _| 5u32.into_ret());
    e.register(DOSMAPERR, |_, _| Ret::default());
    let (old, new) = (e.mem.alloc(8), e.mem.alloc(8));
    assert_eq!(e.call(0x00ec_862c, &args![old, new]).i32(), 0);
    assert_eq!(calls(&e, IMPORT_MOVE_FILE_A), vec![vec![old, new]]);
    assert!(calls(&e, IMPORT_GET_LAST_ERROR).is_empty());
    assert!(calls(&e, DOSMAPERR).is_empty());
    moved.set(0);
    assert_eq!(e.call(0x00ec_862c, &args![old, new]).i32(), -1);
    assert_eq!(calls(&e, DOSMAPERR), vec![vec![5]]);
    // A failure with no error code answers 0 without `_dosmaperr`.
    e.register(IMPORT_GET_LAST_ERROR, |_, _| 0u32.into_ret());
    assert_eq!(e.call(0x00ec_862c, &args![old, new]).i32(), 0);
    assert_eq!(calls(&e, DOSMAPERR).len(), 1);
}

#[test]
fn the_fgets_and_fclose_finally_blocks_unlock_the_stream() {
    let mut e = Engine::new();
    e.call_log = Some(vec![]);
    e.register(UNLOCK_FILE, |_, _| Ret::default());
    e.call(0x00ec_c244, &args![0x0123_0000u32]);
    e.call(0x00ec_997b, &args![0x0456_0000u32]);
    assert_eq!(
        calls(&e, UNLOCK_FILE),
        vec![vec![0x0123_0000], vec![0x0456_0000]]
    );
}

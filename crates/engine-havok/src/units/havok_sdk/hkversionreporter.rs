//! `Havok SDK/hkversionreporter.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions
//! in FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The archive writes. `hkOArchive` over an `hkArray<char>` (`00dda690`, thiscall, one stack
/// word: the array), the byte write (`00d605d0`, thiscall, one stack word), `write32`
/// (`00d60940`), `write16u` (`00d60920`) and `writeRaw` (`00d607b0`, `(data, size)`), the
/// archive destructor (`00d60590`, thiscall) and the `hkDisplaySerializeOStream` constructor
/// over a stream writer (`00dda6e0`, thiscall, one stack word: the writer).
const ARRAY_ARCHIVE_CONSTRUCTOR: u32 = 0x00dd_a690;
const WRITE_BYTE: u32 = 0x00d6_05d0;
const WRITE32: u32 = 0x00d6_0940;
const WRITE16_UNSIGNED: u32 = 0x00d6_0920;
const WRITE_RAW: u32 = 0x00d6_07b0;
const ARCHIVE_DESTRUCTOR: u32 = 0x00d6_0590;
const DISPLAY_STREAM_CONSTRUCTOR: u32 = 0x00dd_a6e0;
/// `hkString::strLen` (`00cb10d0`, cdecl `(text)`).
const STR_LEN: u32 = 0x00cb_10d0;
/// The `hkStringBuf` default constructor (`00cbbd80`, thiscall, no stack words) and its
/// `printf` (`00d61d00`, cdecl, variable arguments `(buffer, format, ...)`).
const STRING_BUF_CONSTRUCTOR: u32 = 0x00cb_bd80;
const STRING_BUF_PRINTF: u32 = 0x00d6_1d00;
/// The calling thread's memory router (`00c85750`): its word at `+0xc` is the temporary
/// allocator and the one at `+0x10` the heap allocator; their vtable slot `+0x10` frees
/// `(storage, bytes)`.
const GET_MEMORY_ROUTER: u32 = 0x00c8_5750;
/// The two version words the report writes after the marker byte (globals at `011b5dc0` and
/// `011b5dbc`).
const VERSION_WORD_FIRST: u32 = 0x011b_5dc0;
const VERSION_WORD_SECOND: u32 = 0x011b_5dbc;
/// The marker byte that starts the report.
const REPORT_MARKER: u32 = 0x90;
/// The strings: `"PC_32"`, the format `"%s:%i"`, `"MSVC"`, and the number printed with them.
const PLATFORM_STRING: u32 = 0x010d_15b4;
const FORMAT_STRING: u32 = 0x010d_15a4;
const COMPILER_STRING: u32 = 0x010d_15ac;
const COMPILER_NUMBER: u32 = 0x5dc;
/// A string length is written as an unsigned 16-bit number: it is cut to this.
const MAX_STRING_LENGTH: i32 = 0xffff;
/// An `hkArray`'s capacity bits and the empty array's capacity word.
const CAPACITY_MASK: u32 = 0x3fff_ffff;
const EMPTY_ARRAY_CAPACITY: u32 = 0x8000_0000;
/// Sizes of the locals: the `hkArray` (three words), an archive object (`0x10` bytes between
/// the exe's slots) and the `hkStringBuf` (`0x8c` bytes: array words, 128 inline bytes).
const ARRAY_SIZE: u32 = 0xc;
const ARCHIVE_SIZE: u32 = 0x10;
const STRING_BUF_SIZE: u32 = 0x8c;

/// A string length as the report writes it: cut to `0xffff` (a signed compare).
fn cut_length(length: i32) -> u32 {
    if length > MAX_STRING_LENGTH {
        MAX_STRING_LENGTH as u32
    } else {
        length as u32
    }
}

/// Frees `array`'s storage through `allocator`'s slot `+0x10` `(data, bytes)` when it owns
/// it (capacity word non-negative), after setting its size to 0 (the order of the exe).
fn release_array(e: &mut Engine, allocator: u32, array: u32) {
    let capacity_word = e.mem.u32(array + 8);
    e.mem.set_u32(array + 4, 0);
    if (capacity_word as i32) >= 0 {
        let data = e.mem.u32(array);
        e.vcall(allocator, 0x10, &args![data, capacity_word & CAPACITY_MASK]);
    }
}

// Translated from 00d61290 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkVersionReporter::sendVersionInformation` (engine map name), cdecl, one stack word
/// `writer` (an `hkStreamWriter`; the decompiler lists no parameter, the code reads it): builds
/// a report in an `hkArray<char>` through an `hkOArchive` over it: the marker byte `0x90`, the
/// two version words (32-bit), `"PC_32"` as a 16-bit length (cut to `0xffff`) and its bytes,
/// then the text `"MSVC:1500"` (`hkStringBuf::printf` of `"%s:%i"` with `"MSVC"` and `0x5dc`)
/// the same way (length is the buffer size minus 1). Then an `hkDisplaySerializeOStream` over
/// `writer` writes the report's size (32-bit) and its bytes. The archives are destroyed, the
/// string buffer's storage is freed through the temporary allocator when it owns it and the
/// array's through the heap allocator. Returns 0.
pub fn hk_version_reporter_send_version_information(e: &mut Engine, writer: Ptr) -> u32 {
    e.with_stack(ARRAY_SIZE, |e, array| {
        let array_address = array.addr();
        e.mem.set_u32(array_address, 0);
        e.mem.set_u32(array_address + 4, 0);
        e.mem.set_u32(array_address + 8, EMPTY_ARRAY_CAPACITY);
        e.with_stack(ARCHIVE_SIZE, |e, report| {
            e.call(ARRAY_ARCHIVE_CONSTRUCTOR, &args![report, array]);
            e.call(WRITE_BYTE, &args![report, REPORT_MARKER]);
            let first: u32 = e.global(VERSION_WORD_FIRST);
            e.call(WRITE32, &args![report, first]);
            let second: u32 = e.global(VERSION_WORD_SECOND);
            e.call(WRITE32, &args![report, second]);
            let length = e.call(STR_LEN, &args![PLATFORM_STRING]).i32();
            let length = cut_length(length);
            e.call(WRITE16_UNSIGNED, &args![report, length]);
            e.call(WRITE_RAW, &args![report, PLATFORM_STRING, length]);
            e.with_stack(STRING_BUF_SIZE, |e, text| {
                e.call(STRING_BUF_CONSTRUCTOR, &args![text]);
                e.call(
                    STRING_BUF_PRINTF,
                    &args![text, FORMAT_STRING, COMPILER_STRING, COMPILER_NUMBER],
                );
                let size = e.mem.i32(text.addr() + 4);
                let length = cut_length(size.wrapping_sub(1));
                e.call(WRITE16_UNSIGNED, &args![report, length]);
                let data = e.mem.u32(text.addr());
                e.call(WRITE_RAW, &args![report, data, length]);
                e.with_stack(ARCHIVE_SIZE, |e, stream| {
                    e.call(DISPLAY_STREAM_CONSTRUCTOR, &args![stream, writer]);
                    let size = e.mem.u32(array_address + 4);
                    e.call(WRITE32, &args![stream, size]);
                    let data = e.mem.u32(array_address);
                    e.call(WRITE_RAW, &args![stream, data, size]);
                    e.call(ARCHIVE_DESTRUCTOR, &args![stream]);
                });
                let router = e.call(GET_MEMORY_ROUTER, &args![]).u32();
                let temporary = e.mem.u32(router + 0xc);
                release_array(e, temporary, text.addr());
                e.mem.set_u32(text.addr(), 0);
                e.mem.set_u32(text.addr() + 8, EMPTY_ARRAY_CAPACITY);
            });
            e.call(ARCHIVE_DESTRUCTOR, &args![report]);
        });
        let router = e.call(GET_MEMORY_ROUTER, &args![]).u32();
        let heap = e.mem.u32(router + 0x10);
        release_array(e, heap, array_address);
    });
    0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00d6_1290,
        hk_version_reporter_send_version_information(Ptr) -> u32
    )]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    const FREE_TEMPORARY_SLOT: u32 = 0x0900_00a0;
    const FREE_HEAP_SLOT: u32 = 0x0900_00a1;

    type Bytes = Rc<RefCell<Vec<u8>>>;
    type Frees = Rc<RefCell<Vec<(u32, u32, u32)>>>;

    struct Rig {
        e: Engine,
        /// The bytes the real stream (the writer) received.
        sink: Bytes,
        frees: Frees,
        /// The archives the destructor ran on, in order.
        destroyed: Rc<RefCell<Vec<u32>>>,
        /// The writer the display stream was built over.
        display_writer: Rc<Cell<u32>>,
    }

    /// Doubles for the archive: writes to the report archive append to the array it was built
    /// over (a heap block of 64 bytes the array then owns), writes to the display stream go to
    /// the sink; the string buffer's constructor/printf produce `"MSVC:1500"`.
    fn rig() -> Rig {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.mem.set_cstr(PLATFORM_STRING, b"PC_32");
        e.mem.set_cstr(FORMAT_STRING, b"%s:%i");
        e.mem.set_cstr(COMPILER_STRING, b"MSVC");
        e.set_global(VERSION_WORD_FIRST, 0x1122_3344u32);
        e.set_global(VERSION_WORD_SECOND, 0x5566_7788u32);
        e.register(STR_LEN, |e, a| Ret {
            eax: e.mem.cstr(a[0]).len() as u32,
            ..Ret::default()
        });
        // Router: temporary allocator (+0xc) and heap allocator (+0x10), each an object whose
        // slot 0x10 logs (slot, storage, bytes).
        let frees: Frees = Rc::new(RefCell::new(Vec::new()));
        let mut allocators = [0u32; 2];
        for (index, slot) in [FREE_TEMPORARY_SLOT, FREE_HEAP_SLOT]
            .into_iter()
            .enumerate()
        {
            let vtable = e.mem.alloc(0x20);
            e.mem.set_u32(vtable + 0x10, slot);
            let object = e.mem.alloc(8);
            e.mem.set_u32(object, vtable);
            allocators[index] = object;
            let log = Rc::clone(&frees);
            e.register_double(slot, move |_, a| {
                log.borrow_mut().push((slot, a[1], a[2]));
                Ret::default()
            });
        }
        let router = e.mem.alloc(0x24);
        e.mem.set_u32(router + 0xc, allocators[0]);
        e.mem.set_u32(router + 0x10, allocators[1]);
        e.register_double(GET_MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        // The report archive: remember it and its array.
        let report = Rc::new(Cell::new(0u32));
        let array = Rc::new(Cell::new(0u32));
        let (r, a) = (Rc::clone(&report), Rc::clone(&array));
        e.register_double(ARRAY_ARCHIVE_CONSTRUCTOR, move |e, args| {
            r.set(args[0]);
            a.set(args[1]);
            // The array gets 64 bytes it owns (capacity word without flags).
            let block = e.mem.alloc(64);
            e.mem.set_u32(args[1], block);
            e.mem.set_u32(args[1] + 8, 64);
            Ret::default()
        });
        let display_writer = Rc::new(Cell::new(0u32));
        let dw = Rc::clone(&display_writer);
        e.register_double(DISPLAY_STREAM_CONSTRUCTOR, move |_, args| {
            dw.set(args[1]);
            Ret::default()
        });
        let sink: Bytes = Rc::new(RefCell::new(Vec::new()));
        // Every write: to the array when `this` is the report archive, otherwise the sink.
        let write = {
            let (report, array, sink) = (Rc::clone(&report), Rc::clone(&array), Rc::clone(&sink));
            move |e: &mut Engine, this: u32, bytes: &[u8]| {
                if this == report.get() {
                    let at = array.get();
                    let size = e.mem.u32(at + 4);
                    let data = e.mem.u32(at);
                    e.mem.write(data + size, bytes);
                    e.mem.set_u32(at + 4, size + bytes.len() as u32);
                } else {
                    sink.borrow_mut().extend_from_slice(bytes);
                }
            }
        };
        let w = write.clone();
        e.register_double(WRITE_BYTE, move |e, a| {
            w(e, a[0], &[a[1] as u8]);
            Ret::default()
        });
        let w = write.clone();
        e.register_double(WRITE32, move |e, a| {
            w(e, a[0], &a[1].to_le_bytes());
            Ret::default()
        });
        let w = write.clone();
        e.register_double(WRITE16_UNSIGNED, move |e, a| {
            w(e, a[0], &(a[1] as u16).to_le_bytes());
            Ret::default()
        });
        let w = write;
        e.register_double(WRITE_RAW, move |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            w(e, a[0], &bytes);
            Ret::default()
        });
        let destroyed = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&destroyed);
        e.register_double(ARCHIVE_DESTRUCTOR, move |_, a| {
            log.borrow_mut().push(a[0]);
            Ret::default()
        });
        // hkStringBuf: the default constructor leaves the inline state (empty string); printf
        // writes "MSVC:1500" and sets the size.
        e.register(STRING_BUF_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0], a[0] + 0xc);
            e.mem.set_u32(a[0] + 4, 1);
            e.mem.set_u32(a[0] + 8, 0x8000_0080);
            Ret::default()
        });
        e.register(STRING_BUF_PRINTF, |e, a| {
            let mut text = e.mem.cstr(a[2]);
            text.extend_from_slice(format!(":{}", a[3]).as_bytes());
            let data = e.mem.u32(a[0]);
            e.mem.write(data, &text);
            e.mem.set_u8(data + text.len() as u32, 0);
            e.mem.set_u32(a[0] + 4, text.len() as u32 + 1);
            Ret::default()
        });
        Rig {
            e,
            sink,
            frees,
            destroyed,
            display_writer,
        }
    }

    #[test]
    fn the_report_goes_through_an_archive_into_the_writer() {
        let mut rig = rig();
        rig.e.call_log = Some(vec![]);
        let writer = rig.e.mem.alloc(0x10);
        assert_eq!(rig.e.call(0x00d6_1290, &args![writer]).u32(), 0);
        // The report: marker, the two version words, "PC_32" and "MSVC:1500", each string
        // with a 16-bit length.
        let mut report = vec![0x90u8];
        report.extend_from_slice(&0x1122_3344u32.to_le_bytes());
        report.extend_from_slice(&0x5566_7788u32.to_le_bytes());
        report.extend_from_slice(&5u16.to_le_bytes());
        report.extend_from_slice(b"PC_32");
        report.extend_from_slice(&9u16.to_le_bytes());
        report.extend_from_slice(b"MSVC:1500");
        // The writer receives the report's size, then the report.
        let mut want = (report.len() as u32).to_le_bytes().to_vec();
        want.extend_from_slice(&report);
        assert_eq!(*rig.sink.borrow(), want);
        assert_eq!(rig.display_writer.get(), writer);
    }

    #[test]
    fn the_archives_are_destroyed_stream_first_and_the_array_is_freed() {
        let mut rig = rig();
        let writer = rig.e.mem.alloc(0x10);
        rig.e.call(0x00d6_1290, &args![writer]);
        // Two archives were destroyed: the display stream, then the report archive.
        let destroyed = rig.destroyed.borrow().clone();
        assert_eq!(destroyed.len(), 2);
        assert_ne!(destroyed[0], destroyed[1]);
        // The string buffer lived in its inline bytes (not freed); the array owned 64
        // bytes, freed through the heap allocator's slot (not the temporary one).
        let frees = rig.frees.borrow();
        assert_eq!(frees.len(), 1);
        assert_eq!(frees[0].0, FREE_HEAP_SLOT);
        assert_eq!(frees[0].2, 64);
    }

    #[test]
    fn a_string_buffer_that_grew_is_freed_through_the_temporary_allocator() {
        let mut rig = rig();
        // printf moves the buffer to a block it owns (capacity word without the flag).
        rig.e.register(STRING_BUF_PRINTF, |e, a| {
            let block = e.mem.alloc(32);
            let text = b"MSVC:1500";
            e.mem.write(block, text);
            e.mem.set_u8(block + text.len() as u32, 0);
            e.mem.set_u32(a[0], block);
            e.mem.set_u32(a[0] + 4, text.len() as u32 + 1);
            e.mem.set_u32(a[0] + 8, 32);
            Ret::default()
        });
        let writer = rig.e.mem.alloc(0x10);
        rig.e.call(0x00d6_1290, &args![writer]);
        let frees = rig.frees.borrow();
        assert_eq!(frees.len(), 2);
        assert_eq!(frees[0].0, FREE_TEMPORARY_SLOT);
        assert_eq!(frees[0].2, 32);
        assert_eq!(frees[1].0, FREE_HEAP_SLOT);
        let sink = rig.sink.borrow();
        assert!(sink.ends_with(b"MSVC:1500"));
    }

    #[test]
    fn long_strings_are_cut_to_sixteen_bits() {
        assert_eq!(cut_length(5), 5);
        assert_eq!(cut_length(0xffff), 0xffff);
        assert_eq!(cut_length(0x10000), 0xffff);
        assert_eq!(cut_length(i32::MAX), 0xffff);
    }
}

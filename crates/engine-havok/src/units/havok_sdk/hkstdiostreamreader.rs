//! `Havok SDK/hkstdiostreamreader.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00d2be00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00d2_bda0`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00d2be00(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00d2_bda0, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// The C runtime functions, called through their addresses: `fread` (cdecl: buffer, element size,
/// count, file), `fseek` (file, offset, origin), `ftell` (file), `_fclose` (file) and `_fopen`
/// (name, mode; the engine map names this address `std::basic_string::c_str`, which it is
/// not: the body is `_fsopen(name, mode, 0x40)`), with the mode string `"rb"` (`0106e79c`).
const FREAD: u32 = 0x00ec_b3a8;
const FSEEK: u32 = 0x00ec_b65a;
const FTELL: u32 = 0x00ec_b562;
const FCLOSE: u32 = 0x00ec_9907;
const FOPEN: u32 = 0x00ec_9a47;
const MODE_READ_BINARY: u32 = 0x0106_e79c;
/// `hkStdioStreamReader`'s vtable (`010ce6c0`) and `hkBaseObject`'s (`0102e388`).
const STDIO_READER_VTABLE: u32 = 0x010c_e6c0;
const BASE_OBJECT_VTABLE: u32 = 0x0102_e388;

// Translated from 00d2bcd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStdioStreamReader::read` (Xbox PDB), `thiscall`, `RET 8`: `fread(buffer, 1, count, file)`
/// with the file at `+0xc`. When it reads nothing (0 or less) the ok byte at `+0x10` is cleared.
/// Answers (in `EAX`; the decompiler shows no result) the number of bytes read.
pub fn hk_stdio_stream_reader_read(e: &mut Engine, this: Ptr, buffer: u32, count: u32) -> i32 {
    let file = e.mem.u32(this.addr() + 0xc);
    let read = e.call(FREAD, &args![buffer, 1u32, count, file]).i32();
    if read < 1 {
        e.mem.set_u8(this.addr() + 0x10, 0);
    }
    read
}

// Translated from 00d2bd00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStdioStreamReader::isOk` (Xbox PDB), `thiscall`, `RET 4`: copies the ok byte (`+0x10`) to the
/// byte `out`; answers `out`.
pub fn hk_stdio_stream_reader_is_ok(e: &mut Engine, this: Ptr, out: u32) -> u32 {
    let ok = e.mem.u8(this.addr() + 0x10);
    e.mem.set_u8(out, ok);
    out
}

// Translated from 00d2bd10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStdioStreamReader::markSupported` (Xbox PDB), virtual, `RET 4` (`ECX`, the reader, is not
/// read): writes 1 at the byte `out`; answers `out`.
pub fn hk_stdio_stream_reader_mark_supported(e: &mut Engine, _this: Ptr, out: u32) -> u32 {
    e.mem.set_u8(out, 1);
    out
}

// Translated from 00d2bd20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStdioStreamReader::seekTellSupported` (Xbox PDB), virtual, `RET 4` (`ECX` not read): writes
/// 1 at the byte `out`; answers `out`.
pub fn hk_stdio_stream_reader_seek_tell_supported(e: &mut Engine, _this: Ptr, out: u32) -> u32 {
    e.mem.set_u8(out, 1);
    out
}

// Translated from 00d2bd30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStdioStreamReader::seek` (Xbox PDB), `thiscall`, `RET 8`: `fseek(file, offset, origin)` with
/// the file at `+0xc`; answers 1 when it fails (a non-zero result), else 0.
pub fn hk_stdio_stream_reader_seek(e: &mut Engine, this: Ptr, offset: u32, origin: u32) -> u32 {
    let file = e.mem.u32(this.addr() + 0xc);
    let result = e.call(FSEEK, &args![file, offset, origin]).u32();
    u32::from(result != 0)
}

// Translated from 00d2bd50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStdioStreamReader::tell` (Xbox PDB), `ECX` = `this`, no stack words: `ftell(file)` with the
/// file at `+0xc`; answers the position.
pub fn hk_stdio_stream_reader_tell(e: &mut Engine, this: Ptr) -> i32 {
    let file = e.mem.u32(this.addr() + 0xc);
    e.call(FTELL, &args![file]).i32()
}

// Translated from 00d2bd60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStdioStreamReader`'s constructor from a file name (no map name; the vtable it stores is
/// `hkStdioStreamReader`'s, `010ce6c0`), `thiscall`, `RET 4`: stores the 16-bit word 1 at `+6`, the
/// word -1 at `+8` and the vtable, opens the file (`_fopen(name, "rb")`), keeps the handle at
/// `+0xc` and sets the ok byte at `+0x10` to whether it opened. Returns `this`.
pub fn fn_00d2bd60(e: &mut Engine, this: Ptr, name: u32) -> Ptr {
    let base = this.addr();
    e.mem.set_u16(base + 6, 1);
    e.mem.set_u32(base + 8, 0xffff_ffff);
    e.mem.set_u32(base, STDIO_READER_VTABLE);
    e.mem.set_u8(base + 0x10, 1);
    let file = e.call(FOPEN, &args![name, MODE_READ_BINARY]).u32();
    e.mem.set_u32(base + 0xc, file);
    e.mem.set_u8(base + 0x10, u8::from(file != 0));
    this
}

// Translated from 00d2bda0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (the destructor body of `hkStdioStreamReader`, which `00d2be00` calls). `ECX` =
/// `this`, no stack words: stores the reader's vtable, closes the file at `+0xc` when there is one
/// (`_fclose`) and stores the `hkBaseObject` vtable.
pub fn fn_00d2bda0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let file = e.mem.u32(base + 0xc);
    e.mem.set_u32(base, STDIO_READER_VTABLE);
    if file != 0 {
        e.call(FCLOSE, &args![file]);
    }
    e.mem.set_u32(base, BASE_OBJECT_VTABLE);
}

// Translated from 00d2bdd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSeekableStreamReader::setMark` (Xbox PDB), virtual, `RET 4` (one word, not read): asks the
/// stream where it is (slot `0x2c`, `tell`), keeps the answer as the mark at `+8` and answers whether
/// it was -1 (the stream could not tell).
pub fn hk_seekable_stream_reader_set_mark(e: &mut Engine, this: Ptr, _unused_1: u32) -> bool {
    let position = e.vcall(this.addr(), 0x2c, &args![]).u32();
    e.mem.set_u32(this.addr() + 8, position);
    position == 0xffff_ffff
}

// Translated from 00d2bdf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSeekableStreamReader::rewindToMark` (Xbox PDB), `ECX` = `this`, no stack words: seeks the
/// stream (slot `0x28`) to the mark at `+8` with origin 0 and answers the seek's `EAX` (the exe
/// ends on that call; `rewindToMark` answers an `hkResult` in the Xbox PDB).
pub fn hk_seekable_stream_reader_rewind_to_mark(e: &mut Engine, this: Ptr) -> u32 {
    let mark = e.mem.u32(this.addr() + 8);
    e.vcall(this.addr(), 0x28, &args![mark, 0u32]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d2_be00, deleting_destructor_00d2be00(Ptr, u8) -> Ptr),
        entry!(0x00d2bcd0, hk_stdio_stream_reader_read(Ptr, u32, u32) -> i32),
        entry!(0x00d2bd00, hk_stdio_stream_reader_is_ok(Ptr, u32) -> u32),
        entry!(0x00d2bd10, hk_stdio_stream_reader_mark_supported(Ptr, u32) -> u32),
        entry!(0x00d2bd20, hk_stdio_stream_reader_seek_tell_supported(Ptr, u32) -> u32),
        entry!(0x00d2bd30, hk_stdio_stream_reader_seek(Ptr, u32, u32) -> u32),
        entry!(0x00d2bd50, hk_stdio_stream_reader_tell(Ptr) -> i32),
        entry!(0x00d2bd60, fn_00d2bd60(Ptr, u32) -> Ptr),
        entry!(0x00d2bda0, fn_00d2bda0(Ptr)),
        entry!(0x00d2bdd0, hk_seekable_stream_reader_set_mark(Ptr, u32) -> bool),
        entry!(0x00d2bdf0, hk_seekable_stream_reader_rewind_to_mark(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FREE_SLOT_DOUBLE: u32 = 0x0900_0011;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    /// Installs a base-destructor double at `base` and a memory router whose allocator
    /// frees through a double; both push their arguments (tagged 1 and 2) on the log.
    fn install(e: &mut Engine, base: u32) -> Rc<RefCell<Vec<u32>>> {
        let log = Rc::new(RefCell::new(Vec::new()));
        let base_log = Rc::clone(&log);
        e.register_double(base, move |_, a| {
            base_log.borrow_mut().extend([1, a[0]]);
            Ret::default()
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 8, FREE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let free_log = Rc::clone(&log);
        e.register_double(FREE_SLOT_DOUBLE, move |_, a| {
            free_log.borrow_mut().extend([2, a[1], a[2]]);
            Ret::default()
        });
        log
    }

    /// Without bit 0 only the base destructor runs; with it the block is also freed with
    /// its stored size. Either way `this` is returned.
    fn check(address: u32, base: u32) {
        let mut e = engine();
        let log = install(&mut e, base);
        let this = e.mem.alloc(0x40);
        e.mem.set_u16(this + 4, 0x1234);
        let got = e.call(address, &args![this, 0xffff_fffeu32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this]);
        log.borrow_mut().clear();
        let got = e.call(address, &args![this, 1u32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this, 2, this, 0x1234]);
    }

    #[test]
    fn deleting_destructor_00d2be00_frees_only_when_bit_zero_is_set() {
        check(0x00d2_be00, 0x00d2_bda0);
    }
}

#[cfg(test)]
mod stdio_reader_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Calls = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// A reader whose file handle is `0xf11e`; the C runtime functions are doubles that log their
    /// arguments and answer `results[address]`.
    fn reader(results: &[(u32, u32)]) -> (Engine, u32, Calls) {
        let mut e = Engine::new();
        let calls: Calls = Rc::new(RefCell::new(Vec::new()));
        for (address, result) in results {
            let (address, result) = (*address, *result);
            let log = Rc::clone(&calls);
            e.register_double(address, move |_, a| {
                log.borrow_mut().push((address, a.to_vec()));
                Ret {
                    eax: result,
                    ..Ret::default()
                }
            });
        }
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0xc, 0xf11e);
        e.mem.set_u8(this + 0x10, 1);
        (e, this, calls)
    }

    #[test]
    fn read_reads_through_fread_and_clears_the_ok_byte_when_nothing_comes() {
        let (mut e, this, calls) = reader(&[(FREAD, 5)]);
        assert_eq!(e.call(0x00d2_bcd0, &args![this, 0x1000u32, 8u32]).i32(), 5);
        assert_eq!(*calls.borrow(), vec![(FREAD, vec![0x1000, 1, 8, 0xf11e])]);
        assert_eq!(e.mem.u8(this + 0x10), 1);
        for result in [0u32, 0xffff_ffff] {
            let (mut e, this, _) = reader(&[(FREAD, result)]);
            assert_eq!(
                e.call(0x00d2_bcd0, &args![this, 0x1000u32, 8u32]).u32(),
                result
            );
            assert_eq!(e.mem.u8(this + 0x10), 0);
        }
    }

    #[test]
    fn is_ok_and_the_support_queries_write_their_bytes() {
        let (mut e, this, _) = reader(&[]);
        let out = e.mem.alloc(4);
        e.mem.set_u8(this + 0x10, 7);
        assert_eq!(e.call(0x00d2_bd00, &args![this, out]).u32(), out);
        assert_eq!(e.mem.u8(out), 7);
        for address in [0x00d2_bd10u32, 0x00d2_bd20] {
            e.mem.set_u8(out, 0);
            assert_eq!(e.call(address, &args![this, out]).u32(), out);
            assert_eq!(e.mem.u8(out), 1);
        }
    }

    #[test]
    fn seek_answers_1_for_a_failed_fseek_and_tell_answers_the_position() {
        let (mut e, this, calls) = reader(&[(FSEEK, 0), (FTELL, 0x1234)]);
        assert_eq!(e.call(0x00d2_bd30, &args![this, 0x40u32, 2u32]).u32(), 0);
        assert_eq!(*calls.borrow(), vec![(FSEEK, vec![0xf11e, 0x40, 2])]);
        assert_eq!(e.call(0x00d2_bd50, &args![this]).i32(), 0x1234);
        assert_eq!(calls.borrow()[1], (FTELL, vec![0xf11e]));
        for failure in [1u32, 0xffff_ffff] {
            let (mut e, this, _) = reader(&[(FSEEK, failure)]);
            assert_eq!(e.call(0x00d2_bd30, &args![this, 0u32, 0u32]).u32(), 1);
        }
    }

    #[test]
    fn the_constructor_opens_the_file_and_records_whether_it_worked() {
        let (mut e, _, calls) = reader(&[(FOPEN, 0xf11e)]);
        let this = e.mem.alloc(0x20);
        assert_eq!(e.call(0x00d2_bd60, &args![this, 0x7000u32]).u32(), this);
        assert_eq!(
            *calls.borrow(),
            vec![(FOPEN, vec![0x7000, MODE_READ_BINARY])]
        );
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this + 8), 0xffff_ffff);
        assert_eq!(e.mem.u32(this), STDIO_READER_VTABLE);
        assert_eq!(e.mem.u32(this + 0xc), 0xf11e);
        assert_eq!(e.mem.u8(this + 0x10), 1);
        // A null handle clears the ok byte.
        let (mut e, _, _) = reader(&[(FOPEN, 0)]);
        let this = e.mem.alloc(0x20);
        e.call(0x00d2_bd60, &args![this, 0x7000u32]);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u8(this + 0x10), 0);
    }

    #[test]
    fn the_destructor_body_closes_an_open_file() {
        let (mut e, this, calls) = reader(&[(FCLOSE, 0)]);
        e.call(0x00d2_bda0, &args![this]);
        assert_eq!(*calls.borrow(), vec![(FCLOSE, vec![0xf11e])]);
        assert_eq!(e.mem.u32(this), BASE_OBJECT_VTABLE);
        // Without a file nothing is closed.
        let (mut e, this, calls) = reader(&[(FCLOSE, 0)]);
        e.mem.set_u32(this + 0xc, 0);
        e.call(0x00d2_bda0, &args![this]);
        assert!(calls.borrow().is_empty());
        assert_eq!(e.mem.u32(this), BASE_OBJECT_VTABLE);
    }

    /// An object whose slots `0x28` (seek) and `0x2c` (tell) log and answer `position`.
    fn seekable(e: &mut Engine, position: u32, calls: &Calls) -> u32 {
        let log = Rc::clone(calls);
        e.register_double(0x0c50_002c, move |_, a| {
            log.borrow_mut().push((0x2c, a[1..].to_vec()));
            Ret {
                eax: position,
                ..Ret::default()
            }
        });
        let log = Rc::clone(calls);
        e.register_double(0x0c50_0028, move |_, a| {
            log.borrow_mut().push((0x28, a[1..].to_vec()));
            Ret::default()
        });
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0x28, 0x0c50_0028);
        e.mem.set_u32(vtable + 0x2c, 0x0c50_002c);
        let object = e.mem.alloc(0x20);
        e.mem.set_u32(object, vtable);
        object
    }

    #[test]
    fn set_mark_keeps_the_position_and_reports_a_stream_that_cannot_tell() {
        let calls: Calls = Rc::new(RefCell::new(Vec::new()));
        let mut e = Engine::new();
        let object = seekable(&mut e, 0x77, &calls);
        assert_eq!(e.call(0x00d2_bdd0, &args![object, 0x100u32]).u32(), 0);
        assert_eq!(e.mem.u32(object + 8), 0x77);
        let mut e = Engine::new();
        let object = seekable(&mut e, 0xffff_ffff, &calls);
        assert_eq!(e.call(0x00d2_bdd0, &args![object, 0x100u32]).u32(), 1);
        assert_eq!(e.mem.u32(object + 8), 0xffff_ffff);
    }

    #[test]
    fn rewind_to_mark_seeks_to_the_mark_from_the_start() {
        let calls: Calls = Rc::new(RefCell::new(Vec::new()));
        let mut e = Engine::new();
        let object = seekable(&mut e, 0, &calls);
        e.mem.set_u32(object + 8, 0x55);
        e.call(0x00d2_bdf0, &args![object]);
        assert_eq!(*calls.borrow(), vec![(0x28, vec![0x55, 0])]);
    }

    #[test]
    fn rewind_to_mark_answers_the_seek_result() {
        let calls: Calls = Rc::new(RefCell::new(Vec::new()));
        let mut e = Engine::new();
        let object = seekable(&mut e, 0, &calls);
        e.register(0x0c50_0028, |_, _| 1u32.into_ret());
        assert_eq!(e.call(0x00d2_bdf0, &args![object]).u32(), 1);
    }
}

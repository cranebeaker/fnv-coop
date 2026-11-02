//! `Havok SDK/hkstdiostreamwriter.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

/// The C runtime's `fopen` (`00ec9a47`, cdecl `(name, mode)`), `fclose` (`00ec9907`, cdecl
/// `(file)`), `fwrite` (`00ecb086`, cdecl `(buffer, size, count, file)`), `fflush`
/// (`00ecda70`, cdecl `(file)`), `fseek` (`00ecb65a`, cdecl `(file, offset, origin)`), `ftell`
/// (`00ecb562`, cdecl `(file)`) and `__iob_func` (`00ecdf44`, no arguments: the address of
/// the table of standard streams, `0x20` bytes per `FILE`).
const CRT_FOPEN: u32 = 0x00ec_9a47;
const CRT_FCLOSE: u32 = 0x00ec_9907;
const CRT_FWRITE: u32 = 0x00ec_b086;
const CRT_FFLUSH: u32 = 0x00ec_da70;
const CRT_FSEEK: u32 = 0x00ec_b65a;
const CRT_FTELL: u32 = 0x00ec_b562;
const CRT_IOB_FUNC: u32 = 0x00ec_df44;
/// The class's vtable (`.?AVhkStdioStreamWriter@@`) and the base object's vtable
/// (`.?AVhkBaseObject@@`) the destructor leaves.
const STDIO_STREAM_WRITER_VTABLE: u32 = 0x010c_e6f4;
const BASE_OBJECT_VTABLE: u32 = 0x0102_e388;
/// The mode string the file constructor opens with (`"wb"`).
const MODE_WRITE_BINARY: u32 = 0x0106_e798;

layout! {
    /// `hkStdioStreamWriter` (engine map name; not in the Xbox PDB), 0x10 bytes: a
    /// `hkStreamWriter` (vtable, the 16-bit size-and-flags at +4 and the 16-bit reference
    /// count at +6, as `hkReferencedObject`) followed by the stream. The names of the last two
    /// fields are descriptive and unconfirmed.
    pub struct HkStdioStreamWriter: 0x10 {
        /// `m_memSizeAndFlags` (Xbox PDB `hkReferencedObject`).
        0x04 m_memSizeAndFlags: u16,
        /// `m_referenceCount` (Xbox PDB `hkReferencedObject`).
        0x06 m_referenceCount: i16,
        /// The C `FILE*` (zero when closed or never opened).
        0x08 file: Ptr,
        /// Whether the writer closes the file itself (the constructors say so: zero for a
        /// standard stream, one for a file it opened).
        0x0C closes_file: bool,
    }
}

// Translated from 00d2bfd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00d2_bfb0`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00d2bfd0(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00d2_bfb0, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00d2be30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (opens the stream's file): `fopen(name, mode)` and the result
/// stored at `+8`; thiscall, two stack words. Returns the `FILE*` (left in EAX).
pub fn fn_00d2be30(e: &mut Engine, this: Ptr<HkStdioStreamWriter>, name: Ptr, mode: Ptr) -> Ptr {
    let file = e.call(CRT_FOPEN, &args![name, mode]).ptr::<()>();
    e.set(this, HkStdioStreamWriter::file, file);
    file
}

// Translated from 00d2be50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (closes the stream's file): when a file is open and the writer
/// closes its own file, `fclose`s it; the file word is cleared either way. The register
/// leftovers in EAX are not meaningful.
pub fn fn_00d2be50(e: &mut Engine, this: Ptr<HkStdioStreamWriter>) {
    let file = e.get(this, HkStdioStreamWriter::file);
    if !file.is_null() && e.get(this, HkStdioStreamWriter::closes_file) {
        e.call(CRT_FCLOSE, &args![file]);
    }
    e.set(this, HkStdioStreamWriter::file, Ptr::NULL);
}

// Translated from 00d2be80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStdioStreamWriter::write` (engine map name), thiscall, two stack words `(buffer,
/// size)`: `fwrite(buffer, 1, size, file)`; when it wrote fewer than one item (signed
/// compare) the stream is closed (`00d2be50`). Returns what `fwrite` returned, or 0 without a
/// file.
pub fn hk_stdio_stream_writer_write(
    e: &mut Engine,
    this: Ptr<HkStdioStreamWriter>,
    buffer: Ptr,
    size: u32,
) -> i32 {
    let file = e.get(this, HkStdioStreamWriter::file);
    if file.is_null() {
        return 0;
    }
    let written = e.call(CRT_FWRITE, &args![buffer, 1u32, size, file]).i32();
    if written < 1 {
        fn_00d2be50(e, this);
    }
    written
}

// Translated from 00d2bec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStdioStreamWriter::flush` (engine map name), thiscall: `fflush(file)` when a file is
/// open. Returns `fflush`'s result, or 0 without a file (the null file word is left in EAX).
pub fn hk_stdio_stream_writer_flush(e: &mut Engine, this: Ptr<HkStdioStreamWriter>) -> i32 {
    let file = e.get(this, HkStdioStreamWriter::file);
    if file.is_null() {
        return 0;
    }
    e.call(CRT_FFLUSH, &args![file]).i32()
}

// Translated from 00d2bed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStdioStreamWriter::isOk` (engine map name), thiscall, one stack word (the `hkBool`
/// result): stores whether a file is open. Returns the `hkBool` pointer.
pub fn hk_stdio_stream_writer_is_ok(
    e: &mut Engine,
    this: Ptr<HkStdioStreamWriter>,
    out: Ptr,
) -> Ptr {
    let open = !e.get(this, HkStdioStreamWriter::file).is_null();
    e.mem.set_u8(out.addr(), u8::from(open));
    out
}

// Translated from 00d2bee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStdioStreamWriter::markSupported` (engine map name), thiscall, one stack word (the
/// `hkBool` result; `this` is not read): stores true. Returns the `hkBool` pointer.
pub fn hk_stdio_stream_writer_mark_supported(
    e: &mut Engine,
    _this: Ptr<HkStdioStreamWriter>,
    out: Ptr,
) -> Ptr {
    e.mem.set_u8(out.addr(), 1);
    out
}

// Translated from 00d2bef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStdioStreamWriter::seekTellSupported` (engine map name), thiscall, one stack word (the
/// `hkBool` result; `this` is not read): stores true. Returns the `hkBool` pointer.
pub fn hk_stdio_stream_writer_seek_tell_supported(
    e: &mut Engine,
    _this: Ptr<HkStdioStreamWriter>,
    out: Ptr,
) -> Ptr {
    e.mem.set_u8(out.addr(), 1);
    out
}

// Translated from 00d2bf00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStdioStreamWriter::seek` (engine map name), thiscall, two stack words `(offset,
/// origin)`: `fseek(file, offset, origin)`; returns whether it failed (a non-zero result),
/// as 0 or 1.
pub fn hk_stdio_stream_writer_seek(
    e: &mut Engine,
    this: Ptr<HkStdioStreamWriter>,
    offset: i32,
    origin: i32,
) -> u32 {
    let file = e.get(this, HkStdioStreamWriter::file);
    let result = e.call(CRT_FSEEK, &args![file, offset, origin]).u32();
    u32::from(result != 0)
}

// Translated from 00d2bf20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStdioStreamWriter::tell` (engine map name), thiscall: `ftell(file)`.
pub fn hk_stdio_stream_writer_tell(e: &mut Engine, this: Ptr<HkStdioStreamWriter>) -> i32 {
    let file = e.get(this, HkStdioStreamWriter::file);
    e.call(CRT_FTELL, &args![file]).i32()
}

// Translated from 00d2bf30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (constructor over a standard stream), thiscall, one stack word
/// `which`: reference count 1, the class's vtable, the writer does not close the file; the
/// file is the standard stream table's entry 2 (`__iob_func() + 0x40`) when `which` is 2,
/// otherwise entry 1 (`__iob_func() + 0x20`). Returns `this`.
pub fn fn_00d2bf30(e: &mut Engine, this: Ptr<HkStdioStreamWriter>, which: i32) -> Ptr {
    e.set(this, HkStdioStreamWriter::m_referenceCount, 1);
    e.mem.set_u32(this.addr(), STDIO_STREAM_WRITER_VTABLE);
    e.set(this, HkStdioStreamWriter::closes_file, false);
    let table = e.call(CRT_IOB_FUNC, &args![]).u32();
    let file = if which == 2 {
        table.wrapping_add(0x40)
    } else {
        table.wrapping_add(0x20)
    };
    e.set(this, HkStdioStreamWriter::file, Ptr::new(file));
    this.cast()
}

// Translated from 00d2bf70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (constructor over a file name), thiscall, one stack word
/// `name`: reference count 1, the class's vtable, no file yet, the writer closes its own
/// file; a non-null `name` is opened with mode `"wb"` (`00d2be30`). Returns `this`.
pub fn fn_00d2bf70(e: &mut Engine, this: Ptr<HkStdioStreamWriter>, name: Ptr) -> Ptr {
    e.set(this, HkStdioStreamWriter::m_referenceCount, 1);
    e.mem.set_u32(this.addr(), STDIO_STREAM_WRITER_VTABLE);
    e.set(this, HkStdioStreamWriter::file, Ptr::NULL);
    e.set(this, HkStdioStreamWriter::closes_file, true);
    if !name.is_null() {
        fn_00d2be30(e, this, name, Ptr::new(MODE_WRITE_BINARY));
    }
    this.cast()
}

// Translated from 00d2bfb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (the destructor the deleting destructor `00d2bfd0` runs):
/// sets the class's vtable, closes the file (`00d2be50`), then sets the base object's
/// vtable (`.?AVhkBaseObject@@`).
pub fn fn_00d2bfb0(e: &mut Engine, this: Ptr<HkStdioStreamWriter>) {
    e.mem.set_u32(this.addr(), STDIO_STREAM_WRITER_VTABLE);
    fn_00d2be50(e, this);
    e.mem.set_u32(this.addr(), BASE_OBJECT_VTABLE);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d2_bfd0, deleting_destructor_00d2bfd0(Ptr, u8) -> Ptr),
        entry!(
            0x00d2_be30,
            fn_00d2be30(Ptr<HkStdioStreamWriter>, Ptr, Ptr) -> Ptr
        ),
        entry!(0x00d2_be50, fn_00d2be50(Ptr<HkStdioStreamWriter>)),
        entry!(
            0x00d2_be80,
            hk_stdio_stream_writer_write(Ptr<HkStdioStreamWriter>, Ptr, u32) -> i32
        ),
        entry!(
            0x00d2_bec0,
            hk_stdio_stream_writer_flush(Ptr<HkStdioStreamWriter>) -> i32
        ),
        entry!(
            0x00d2_bed0,
            hk_stdio_stream_writer_is_ok(Ptr<HkStdioStreamWriter>, Ptr) -> Ptr
        ),
        entry!(
            0x00d2_bee0,
            hk_stdio_stream_writer_mark_supported(Ptr<HkStdioStreamWriter>, Ptr) -> Ptr
        ),
        entry!(
            0x00d2_bef0,
            hk_stdio_stream_writer_seek_tell_supported(Ptr<HkStdioStreamWriter>, Ptr) -> Ptr
        ),
        entry!(
            0x00d2_bf00,
            hk_stdio_stream_writer_seek(Ptr<HkStdioStreamWriter>, i32, i32) -> u32
        ),
        entry!(
            0x00d2_bf20,
            hk_stdio_stream_writer_tell(Ptr<HkStdioStreamWriter>) -> i32
        ),
        entry!(
            0x00d2_bf30,
            fn_00d2bf30(Ptr<HkStdioStreamWriter>, i32) -> Ptr
        ),
        entry!(
            0x00d2_bf70,
            fn_00d2bf70(Ptr<HkStdioStreamWriter>, Ptr) -> Ptr
        ),
        entry!(0x00d2_bfb0, fn_00d2bfb0(Ptr<HkStdioStreamWriter>)),
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
    fn deleting_destructor_00d2bfd0_frees_only_when_bit_zero_is_set() {
        check(0x00d2_bfd0, 0x00d2_bfb0);
    }
}

#[cfg(test)]
mod tests_stdio {
    use super::*;

    type Calls = Vec<(u32, Vec<u32>)>;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e
    }

    /// A writer record with the given file word and close flag.
    fn writer(e: &mut Engine, file: u32, closes: bool) -> u32 {
        let record = e.mem.alloc(0x10);
        e.mem.set_u32(record + 8, file);
        e.mem.set_u8(record + 0xc, u8::from(closes));
        record
    }

    /// Makes the function at `address` answer `value` in EAX.
    fn answer(e: &mut Engine, address: u32, value: u32) {
        e.register_double(address, move |_, _| Ret {
            eax: value,
            ..Ret::default()
        });
    }

    /// The logged calls to `address`.
    fn calls_to(e: &Engine, address: u32) -> Calls {
        let log = e.call_log.as_ref().unwrap();
        log.iter().filter(|c| c.0 == address).cloned().collect()
    }

    #[test]
    fn fn_00d2be30_stores_the_file_fopen_returns() {
        let mut e = engine();
        answer(&mut e, CRT_FOPEN, 0x7000);
        let this = writer(&mut e, 0, true);
        let got = e
            .call(0x00d2_be30, &args![this, 0x1111u32, 0x2222u32])
            .u32();
        assert_eq!(got, 0x7000);
        assert_eq!(e.mem.u32(this + 8), 0x7000);
        assert_eq!(
            calls_to(&e, CRT_FOPEN),
            vec![(CRT_FOPEN, vec![0x1111, 0x2222])]
        );
    }

    #[test]
    fn fn_00d2be50_closes_only_an_owned_open_file_and_always_clears_it() {
        let mut e = engine();
        answer(&mut e, CRT_FCLOSE, 0);
        let owned = writer(&mut e, 0x7000, true);
        e.call(0x00d2_be50, &args![owned]);
        assert_eq!(calls_to(&e, CRT_FCLOSE), vec![(CRT_FCLOSE, vec![0x7000])]);
        assert_eq!(e.mem.u32(owned + 8), 0);
        e.call_log = Some(vec![]);
        let borrowed = writer(&mut e, 0x7100, false);
        e.call(0x00d2_be50, &args![borrowed]);
        assert!(calls_to(&e, CRT_FCLOSE).is_empty());
        assert_eq!(e.mem.u32(borrowed + 8), 0);
        let closed = writer(&mut e, 0, true);
        e.call(0x00d2_be50, &args![closed]);
        assert!(calls_to(&e, CRT_FCLOSE).is_empty());
        assert_eq!(e.mem.u32(closed + 8), 0);
    }

    #[test]
    fn write_returns_the_count_and_closes_the_stream_when_it_wrote_nothing() {
        let mut e = engine();
        answer(&mut e, CRT_FCLOSE, 0);
        // A positive count keeps the file open.
        answer(&mut e, CRT_FWRITE, 5);
        let this = writer(&mut e, 0x7000, true);
        let got = e.call(0x00d2_be80, &args![this, 0x3000u32, 5u32]).i32();
        assert_eq!(got, 5);
        assert_eq!(e.mem.u32(this + 8), 0x7000);
        assert_eq!(
            calls_to(&e, CRT_FWRITE),
            vec![(CRT_FWRITE, vec![0x3000, 1, 5, 0x7000])]
        );
        assert!(calls_to(&e, CRT_FCLOSE).is_empty());
        // Zero and negative counts close it (the compare is signed).
        for result in [0u32, 0xffff_ffff] {
            answer(&mut e, CRT_FWRITE, result);
            e.mem.set_u32(this + 8, 0x7000);
            e.call_log = Some(vec![]);
            let got = e.call(0x00d2_be80, &args![this, 0x3000u32, 5u32]).u32();
            assert_eq!(got, result);
            assert_eq!(e.mem.u32(this + 8), 0);
            assert_eq!(calls_to(&e, CRT_FCLOSE).len(), 1);
        }
        // Without a file nothing is written.
        e.call_log = Some(vec![]);
        let got = e.call(0x00d2_be80, &args![this, 0x3000u32, 5u32]).u32();
        assert_eq!(got, 0);
        assert!(calls_to(&e, CRT_FWRITE).is_empty());
    }

    #[test]
    fn flush_flushes_an_open_file_and_answers_zero_without_one() {
        let mut e = engine();
        answer(&mut e, CRT_FFLUSH, 0xffff_fffe);
        let this = writer(&mut e, 0x7000, true);
        assert_eq!(e.call(0x00d2_bec0, &args![this]).i32(), -2);
        assert_eq!(calls_to(&e, CRT_FFLUSH), vec![(CRT_FFLUSH, vec![0x7000])]);
        e.call_log = Some(vec![]);
        let none = writer(&mut e, 0, true);
        assert_eq!(e.call(0x00d2_bec0, &args![none]).i32(), 0);
        assert!(calls_to(&e, CRT_FFLUSH).is_empty());
    }

    #[test]
    fn is_ok_stores_whether_a_file_is_open_and_returns_the_result_pointer() {
        let mut e = engine();
        let out = e.mem.alloc(4);
        let open = writer(&mut e, 0x7000, true);
        assert_eq!(e.call(0x00d2_bed0, &args![open, out]).u32(), out);
        assert_eq!(e.mem.u8(out), 1);
        let closed = writer(&mut e, 0, true);
        assert_eq!(e.call(0x00d2_bed0, &args![closed, out]).u32(), out);
        assert_eq!(e.mem.u8(out), 0);
    }

    #[test]
    fn mark_and_seek_tell_support_store_true_without_reading_the_writer() {
        let mut e = engine();
        let out = e.mem.alloc(4);
        for address in [0x00d2_bee0u32, 0x00d2_bef0] {
            e.mem.set_u8(out, 0);
            // `this` is not read: an unmapped address is fine.
            assert_eq!(e.call(address, &args![0xdead_0000u32, out]).u32(), out);
            assert_eq!(e.mem.u8(out), 1);
        }
    }

    #[test]
    fn seek_answers_one_for_any_failure_and_passes_offset_and_origin() {
        let mut e = engine();
        let this = writer(&mut e, 0x7000, true);
        for (result, expected) in [(0u32, 0u32), (0xffff_ffff, 1), (5, 1)] {
            answer(&mut e, CRT_FSEEK, result);
            e.call_log = Some(vec![]);
            let got = e.call(0x00d2_bf00, &args![this, -4i32, 2i32]).u32();
            assert_eq!(got, expected);
            assert_eq!(
                calls_to(&e, CRT_FSEEK),
                vec![(CRT_FSEEK, vec![0x7000, 0xffff_fffc, 2])]
            );
        }
    }

    #[test]
    fn tell_returns_ftell_of_the_file() {
        let mut e = engine();
        answer(&mut e, CRT_FTELL, 1234);
        let this = writer(&mut e, 0x7000, true);
        assert_eq!(e.call(0x00d2_bf20, &args![this]).i32(), 1234);
        assert_eq!(calls_to(&e, CRT_FTELL), vec![(CRT_FTELL, vec![0x7000])]);
    }

    #[test]
    fn standard_stream_constructor_picks_the_table_entry_by_which() {
        let mut e = engine();
        answer(&mut e, CRT_IOB_FUNC, 0x9000);
        for (which, file) in [(2i32, 0x9040u32), (1, 0x9020), (0, 0x9020), (3, 0x9020)] {
            let this = e.mem.alloc(0x10);
            e.mem.set_u8(this + 0xc, 1);
            assert_eq!(e.call(0x00d2_bf30, &args![this, which]).u32(), this);
            assert_eq!(e.mem.u32(this), STDIO_STREAM_WRITER_VTABLE);
            assert_eq!(e.mem.i16(this + 6), 1);
            assert_eq!(e.mem.u8(this + 0xc), 0);
            assert_eq!(e.mem.u32(this + 8), file);
        }
    }

    #[test]
    fn file_constructor_opens_a_given_name_for_writing() {
        let mut e = engine();
        answer(&mut e, CRT_FOPEN, 0x7000);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 8, 0x1234);
        assert_eq!(e.call(0x00d2_bf70, &args![this, 0x4000u32]).u32(), this);
        assert_eq!(e.mem.u32(this), STDIO_STREAM_WRITER_VTABLE);
        assert_eq!(e.mem.i16(this + 6), 1);
        assert_eq!(e.mem.u8(this + 0xc), 1);
        assert_eq!(e.mem.u32(this + 8), 0x7000);
        assert_eq!(
            calls_to(&e, CRT_FOPEN),
            vec![(CRT_FOPEN, vec![0x4000, MODE_WRITE_BINARY])]
        );
    }

    #[test]
    fn file_constructor_without_a_name_opens_nothing() {
        let mut e = engine();
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 8, 0x1234);
        assert_eq!(e.call(0x00d2_bf70, &args![this, 0u32]).u32(), this);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u8(this + 0xc), 1);
        assert!(calls_to(&e, CRT_FOPEN).is_empty());
    }

    #[test]
    fn destructor_closes_the_file_and_leaves_the_base_vtable() {
        let mut e = engine();
        answer(&mut e, CRT_FCLOSE, 0);
        let this = writer(&mut e, 0x7000, true);
        e.mem.set_u32(this, 0x1111);
        e.call(0x00d2_bfb0, &args![this]);
        assert_eq!(calls_to(&e, CRT_FCLOSE), vec![(CRT_FCLOSE, vec![0x7000])]);
        assert_eq!(e.mem.u32(this), BASE_OBJECT_VTABLE);
        assert_eq!(e.mem.u32(this + 8), 0);
    }
}

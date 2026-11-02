//! `Havok SDK/hkbufferedstreamwriter.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00d27570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00d2_7520`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00d27570(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00d2_7520, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// `memset` of the CRT behind the jump at `00cb1280` (cdecl `(destination, value, bytes)`).
const MEM_SET: u32 = 0x00cb_1280;

// Translated from 00d274c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkBufferedStreamWriter::hkBufferedStreamWriter_ov2` (Xbox PDB name from the exe's symbol
/// table; the vtable `010ce448` it stores has the RTTI name `hkBufferedStreamWriter`).
/// `this` is ECX; `buffer` and `size` describe the byte buffer; `clear` is the low byte of
/// the third stack word (the upper bytes are not read). Sets the 16-bit word at +6 to 1, the
/// vtable, the buffer at +0xc, zeros at +8 and +0x10 and the byte at +0x18, and the limit at
/// +0x14 to `size - 1` when `clear` is non-zero (the buffer is zeroed with `memset`) or `size`
/// when it is zero. Returns `this`.
pub fn hk_buffered_stream_writer_hk_buffered_stream_writer_ov2(
    e: &mut Engine,
    this: Ptr,
    buffer: u32,
    size: u32,
    clear: u8,
) -> Ptr {
    let t = this.addr();
    e.mem.set_u16(t + 6, 1);
    e.mem.set_u32(t, 0x010c_e448);
    e.mem.set_u32(t + 8, 0);
    e.mem.set_u32(t + 0xc, buffer);
    e.mem.set_u32(t + 0x10, 0);
    let limit = if clear != 0 {
        size.wrapping_sub(1)
    } else {
        size
    };
    e.mem.set_u32(t + 0x14, limit);
    e.mem.set_u8(t + 0x18, 0);
    if clear != 0 {
        e.call(MEM_SET, &args![buffer, 0u32, size]);
    }
    this
}

// Translated from 00d27560 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkBufferedStreamWriter::getClassType` (Xbox PDB): takes no arguments and returns the constant 0x0126_89e4.
pub fn hk_buffered_stream_writer_get_class_type(_e: &mut Engine) -> u32 {
    0x0126_89e4
}

/// `memcpy` (`00cb1260`, cdecl `(destination, source, bytes)`).
const MEM_COPY: u32 = 0x00cb_1260;
/// The reference-count functions the constructor and destructor call on the inner stream
/// (`ECX` = the object): add (`00c90510`) and remove (`00c905b0`).
const ADD_REFERENCE: u32 = 0x00c9_0510;
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// `alignedAlloc` (`00c85620`, `(allocator, size, alignment)`) and `alignedFree` (`00c85660`,
/// `(allocator, block)`).
const ALIGNED_ALLOC: u32 = 0x00c8_5620;
const ALIGNED_FREE: u32 = 0x00c8_5660;
/// The base vtable the destructor leaves in the object.
const BASE_VTABLE: u32 = 0x0102_e388;
/// The vtable of `hkBufferedStreamWriter`.
const VTABLE: u32 = 0x010c_e448;

// Translated from 00d27210 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkBufferedStreamWriter::flushBuffer` (Xbox PDB; `ECX` = `this`): without an inner stream
/// (`+8`) it answers 0. Otherwise the buffered bytes (`+0xc`, count at `+0x10`) are written
/// through slot `0x10` of the inner stream, `(pointer, remaining)` each time, until all are
/// written (then the count is reset to 0) or the stream writes nothing (the count is left).
/// Answers the number of bytes written.
pub fn hk_buffered_stream_writer_flush_buffer(e: &mut Engine, this: Ptr) -> u32 {
    let t = this.addr();
    let stream = e.mem.u32(t + 8);
    if stream == 0 {
        return 0;
    }
    let total = e.mem.u32(t + 0x10) as i32;
    let mut written: u32 = 0;
    if total > 0 {
        loop {
            let stream = e.mem.u32(t + 8);
            let buffer = e.mem.u32(t + 0xc).wrapping_add(written);
            let remaining = (total as u32).wrapping_sub(written);
            let count = e.vcall(stream, 0x10, &args![buffer, remaining]).u32();
            written = written.wrapping_add(count);
            if count == 0 {
                return written;
            }
            if written as i32 >= total {
                break;
            }
        }
    }
    e.mem.set_u32(t + 0x10, 0);
    written
}

// Translated from 00d27260 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkBufferedStreamWriter::write` (Xbox PDB): copies `count` bytes from `buffer` into the
/// buffer at `+0xc` (position `+0x10`, limit `+0x14`). While more than the free space is
/// left the free space is filled, the buffer is flushed ([`hk_buffered_stream_writer_flush_buffer`])
/// and, when that did not write exactly the filled position, the answer is the count taken so
/// far. Answers `count` once everything is copied.
pub fn hk_buffered_stream_writer_write(e: &mut Engine, this: Ptr, buffer: u32, count: u32) -> u32 {
    let t = this.addr();
    let mut source = buffer;
    let mut remaining = count as i32;
    let mut space = (e.mem.u32(t + 0x14) as i32).wrapping_sub(e.mem.u32(t + 0x10) as i32);
    while remaining > space {
        let destination = e.mem.u32(t + 0x10).wrapping_add(e.mem.u32(t + 0xc));
        e.call(MEM_COPY, &args![destination, source, space as u32]);
        let position = e.mem.u32(t + 0x10).wrapping_add(space as u32);
        e.mem.set_u32(t + 0x10, position);
        source = source.wrapping_add(space as u32);
        remaining = remaining.wrapping_sub(space);
        if hk_buffered_stream_writer_flush_buffer(e, this) != position {
            return count.wrapping_sub(remaining as u32);
        }
        space = (e.mem.u32(t + 0x14) as i32).wrapping_sub(e.mem.u32(t + 0x10) as i32);
    }
    let destination = e.mem.u32(t + 0xc).wrapping_add(e.mem.u32(t + 0x10));
    e.call(MEM_COPY, &args![destination, source, remaining as u32]);
    let position = e.mem.u32(t + 0x10).wrapping_add(remaining as u32);
    e.mem.set_u32(t + 0x10, position);
    count
}

// Translated from 00d272e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkBufferedStreamWriter::flush` (Xbox PDB; `ECX` = `this`): flushes the buffer
/// ([`hk_buffered_stream_writer_flush_buffer`]) and then jumps to slot `0x14` of the inner
/// stream (`+8`), whose result it returns; without an inner stream it returns the result of
/// the buffer flush (0).
pub fn hk_buffered_stream_writer_flush(e: &mut Engine, this: Ptr) -> u32 {
    let flushed = hk_buffered_stream_writer_flush_buffer(e, this);
    let stream = e.mem.u32(this.addr() + 8);
    if stream == 0 {
        return flushed;
    }
    e.vcall(stream, 0x14, &args![]).u32()
}

// Translated from 00d27300 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkBufferedStreamWriter::isOk` (Xbox PDB): writes the answer to the byte at `out` and
/// returns `out`. With an inner stream (`+8`) it is the byte slot `0xc` of the stream stores
/// through the pointer it returns (called with a local byte); without one it is whether the
/// position (`+0x10`) differs from the limit (`+0x14`).
pub fn hk_buffered_stream_writer_is_ok(e: &mut Engine, this: Ptr, out: u32) -> u32 {
    let t = this.addr();
    let stream = e.mem.u32(t + 8);
    if stream != 0 {
        // The local byte is at +3 of the exe's one-word frame.
        let byte = e.with_stack(4, |e, local| {
            let result = e.vcall(stream, 0xc, &args![local.addr() + 3]).u32();
            e.mem.u8(result)
        });
        e.mem.set_u8(out, byte);
        return out;
    }
    let differs = e.mem.u32(t + 0x10) != e.mem.u32(t + 0x14);
    e.mem.set_u8(out, u8::from(differs));
    out
}

// Translated from 00d27340 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkBufferedStreamWriter::seekTellSupported` (Xbox PDB): writes the answer to the byte at
/// `out` and returns `out`. With an inner stream (`+8`) it is the byte slot `0x18` of the
/// stream stores through the pointer it returns (called with a local byte); without one it
/// is 1.
pub fn hk_buffered_stream_writer_seek_tell_supported(e: &mut Engine, this: Ptr, out: u32) -> u32 {
    let stream = e.mem.u32(this.addr() + 8);
    if stream != 0 {
        // The local byte is at +3 of the exe's one-word frame.
        let byte = e.with_stack(4, |e, local| {
            let result = e.vcall(stream, 0x18, &args![local.addr() + 3]).u32();
            e.mem.u8(result)
        });
        e.mem.set_u8(out, byte);
        return out;
    }
    e.mem.set_u8(out, 1);
    out
}

// Translated from 00d27380 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkBufferedStreamWriter::seek` (Xbox PDB): with an inner stream (`+8`) the buffer is
/// flushed and slot `0x1c` of the stream is jumped to with the same `(offset, mode)`; its
/// result is returned. Without one the new position is `offset` for mode 0, `position +
/// offset` for mode 1, `position - offset` for mode 2 and -1 for any other mode; a negative
/// position sets the position to 0 and answers 1, a position above the limit (`+0x14`; signed
/// compare) is clamped to it and answers 1, otherwise the answer is 0.
pub fn hk_buffered_stream_writer_seek(e: &mut Engine, this: Ptr, offset: u32, mode: u32) -> u32 {
    let t = this.addr();
    let stream = e.mem.u32(t + 8);
    if stream != 0 {
        hk_buffered_stream_writer_flush_buffer(e, this);
        let stream = e.mem.u32(t + 8);
        return e.vcall(stream, 0x1c, &args![offset, mode]).u32();
    }
    let position = e.mem.u32(t + 0x10);
    let mut target: i32 = match mode {
        0 => offset as i32,
        1 => position.wrapping_add(offset) as i32,
        2 => position.wrapping_sub(offset) as i32,
        _ => -1,
    };
    if target < 0 {
        e.mem.set_u32(t + 0x10, 0);
        return 1;
    }
    let limit = e.mem.u32(t + 0x14) as i32;
    let mut answer = 0;
    if target > limit {
        target = limit;
        answer = 1;
    }
    e.mem.set_u32(t + 0x10, target as u32);
    answer
}

// Translated from 00d273f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkBufferedStreamWriter::tell` (Xbox PDB; `ECX` = `this`): the position (`+0x10`) plus the
/// inner stream's own position (slot `0x20`) when there is an inner stream (`+8`); -1 when
/// that position is negative.
pub fn hk_buffered_stream_writer_tell(e: &mut Engine, this: Ptr) -> i32 {
    let t = this.addr();
    let position = e.mem.u32(t + 0x10);
    let stream = e.mem.u32(t + 8);
    if stream == 0 {
        return position as i32;
    }
    let inner = e.vcall(stream, 0x20, &args![]).i32();
    if inner < 0 {
        return -1;
    }
    position.wrapping_add(inner as u32) as i32
}

// Translated from 00d27420 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkBufferedStreamWriter::calcContentStatistics` (Xbox PDB): reports to the visitor,
/// through its slot `0x14`, the buffer as `(1, 0x01080668, buffer (+0xc), limit (+0x14), 0)`
/// and through its slot `0xc` the inner stream as `(0x010ce43c, stream (+8), 0)`.
/// The second stack word (`RET 8`; callers pass the class record) is not read.
pub fn hk_buffered_stream_writer_calc_content_statistics(
    e: &mut Engine,
    this: Ptr,
    visitor: u32,
    _unused_2: u32,
) {
    let t = this.addr();
    let buffer = e.mem.u32(t + 0xc);
    let limit = e.mem.u32(t + 0x14);
    e.vcall(
        visitor,
        0x14,
        &args![1u32, 0x0108_0668u32, buffer, limit, 0u32],
    );
    let stream = e.mem.u32(t + 8);
    e.vcall(visitor, 0xc, &args![0x010c_e43cu32, stream, 0u32]);
}

// Translated from 00d27460 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkBufferedStreamWriter::hkBufferedStreamWriter` (Xbox PDB): stores the inner `stream`
/// (`+8`), the 16-bit word 1 at `+6`, the vtable (`010ce448`) and the byte 1 at `+0x18`; a
/// non-null stream gets a reference added (`00c90510`). The buffer (`+0xc`) is `size` bytes
/// from the memory router's allocator (`+0x10`) aligned to 0x40, the limit (`+0x14`) is
/// `size` and the position (`+0x10`) 0. Returns `this`.
pub fn hk_buffered_stream_writer_hk_buffered_stream_writer(
    e: &mut Engine,
    this: Ptr,
    stream: u32,
    size: u32,
) -> Ptr {
    let t = this.addr();
    e.mem.set_u32(t + 8, stream);
    e.mem.set_u16(t + 6, 1);
    e.mem.set_u32(t, VTABLE);
    e.mem.set_u8(t + 0x18, 1);
    let stream = e.mem.u32(t + 8);
    if stream != 0 {
        e.call(ADD_REFERENCE, &args![stream]);
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let buffer = e
        .call(ALIGNED_ALLOC, &args![allocator, size, 0x40u32])
        .u32();
    e.mem.set_u32(t + 0x14, size);
    e.mem.set_u32(t + 0xc, buffer);
    e.mem.set_u32(t + 0x10, 0);
    this
}

// Translated from 00d27520 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkBufferedStreamWriter::~hkBufferedStreamWriter` (Xbox PDB): stores the vtable, flushes
/// ([`hk_buffered_stream_writer_flush`]), removes the reference on a non-null inner stream
/// (`00c905b0`), frees the buffer through the router's allocator (`alignedFree`) when the
/// byte at `+0x18` is set, and stores the base vtable (`0102e388`).
pub fn hk_buffered_stream_writer_destructor(e: &mut Engine, this: Ptr) {
    let t = this.addr();
    e.mem.set_u32(t, VTABLE);
    hk_buffered_stream_writer_flush(e, this);
    let stream = e.mem.u32(t + 8);
    if stream != 0 {
        e.call(REMOVE_REFERENCE, &args![stream]);
    }
    if e.mem.u8(t + 0x18) != 0 {
        let buffer = e.mem.u32(t + 0xc);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        e.call(ALIGNED_FREE, &args![allocator, buffer]);
    }
    e.mem.set_u32(t, BASE_VTABLE);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d2_7210, hk_buffered_stream_writer_flush_buffer(Ptr) -> u32),
        entry!(0x00d2_7260, hk_buffered_stream_writer_write(Ptr, u32, u32) -> u32),
        entry!(0x00d2_72e0, hk_buffered_stream_writer_flush(Ptr) -> u32),
        entry!(0x00d2_7300, hk_buffered_stream_writer_is_ok(Ptr, u32) -> u32),
        entry!(0x00d2_7340, hk_buffered_stream_writer_seek_tell_supported(Ptr, u32) -> u32),
        entry!(0x00d2_7380, hk_buffered_stream_writer_seek(Ptr, u32, u32) -> u32),
        entry!(0x00d2_73f0, hk_buffered_stream_writer_tell(Ptr) -> i32),
        entry!(
            0x00d2_7420,
            hk_buffered_stream_writer_calc_content_statistics(Ptr, u32, u32)
        ),
        entry!(0x00d2_7460, hk_buffered_stream_writer_hk_buffered_stream_writer(Ptr, u32, u32) -> Ptr),
        entry!(0x00d2_7520, hk_buffered_stream_writer_destructor(Ptr)),
        entry!(0x00d2_7570, deleting_destructor_00d27570(Ptr, u8) -> Ptr),
        entry!(0x00d2_74c0, hk_buffered_stream_writer_hk_buffered_stream_writer_ov2(Ptr, u32, u32, u8) -> Ptr),
        entry!(0x00d27560, hk_buffered_stream_writer_get_class_type() -> u32),
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
    fn deleting_destructor_00d27570_frees_only_when_bit_zero_is_set() {
        check(0x00d2_7570, 0x00d2_7520);
    }

    #[test]
    fn hk_buffered_stream_writer_get_class_type_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00d2_7560, &args![]).u32(), 0x0126_89e4);
    }
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

    #[test]
    fn construct_sets_the_fields_and_clears_the_buffer_when_asked() {
        let mut e = test_engine();
        let log: Rc<RefCell<Vec<Vec<u32>>>> = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&log);
        e.register_double(MEM_SET, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        let this = e.mem.alloc(0x20);
        e.mem.set_u8(this + 0x18, 9);
        let got = e.call(0x00d2_74c0, &args![this, 0x4000u32, 0x100u32, 1u32]);
        assert_eq!(got.u32(), this);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this), 0x010c_e448);
        assert_eq!(e.mem.u32(this + 0xc), 0x4000);
        assert_eq!(e.mem.u32(this + 0x14), 0xff);
        assert_eq!(e.mem.u8(this + 0x18), 0);
        assert_eq!(*log.borrow(), vec![vec![0x4000, 0, 0x100]]);
        // Without the flag the limit is the whole size and nothing is cleared; only the low
        // byte of the word counts.
        log.borrow_mut().clear();
        e.call(
            0x00d2_74c0,
            &args![this, 0x4000u32, 0x100u32, 0x1234_5600u32],
        );
        assert_eq!(e.mem.u32(this + 0x14), 0x100);
        assert!(log.borrow().is_empty());
    }
}

#[cfg(test)]
mod translated_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// Doubles behind the slots of the inner stream (`0xc` isOk, `0x10` write, `0x14` flush,
    /// `0x18` seekTellSupported, `0x1c` seek, `0x20` tell), the visitor and the allocator.
    const SLOT_IS_OK: u32 = 0x0900_000c;
    const SLOT_WRITE: u32 = 0x0900_0010;
    const SLOT_FLUSH: u32 = 0x0900_0014;
    const SLOT_SUPPORTED: u32 = 0x0900_0018;
    const SLOT_SEEK: u32 = 0x0900_001c;
    const SLOT_TELL: u32 = 0x0900_0020;

    type Calls = Rc<RefCell<Vec<Vec<u32>>>>;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e.call_log = Some(Vec::new());
        e
    }

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn answer(e: &mut Engine, address: u32, value: u32) {
        e.register_double(address, move |_, _| Ret {
            eax: value,
            ..Ret::default()
        });
    }

    /// An inner stream object whose vtable holds the slot doubles.
    fn inner_stream(e: &mut Engine) -> u32 {
        let vtable = e.mem.alloc(0x40);
        for slot in [
            SLOT_IS_OK,
            SLOT_WRITE,
            SLOT_FLUSH,
            SLOT_SUPPORTED,
            SLOT_SEEK,
            SLOT_TELL,
        ] {
            e.mem.set_u32(vtable + (slot & 0xff), slot);
        }
        let stream = e.mem.alloc(0x10);
        e.mem.set_u32(stream, vtable);
        stream
    }

    /// A writer with a 16-byte buffer at `buffer`, position `position`, limit `limit` and the
    /// given inner stream.
    fn writer(e: &mut Engine, stream: u32, position: u32, limit: u32) -> (u32, u32) {
        let this = e.mem.alloc(0x20);
        let buffer = e.mem.alloc(16);
        e.mem.set_u32(this + 8, stream);
        e.mem.set_u32(this + 0xc, buffer);
        e.mem.set_u32(this + 0x10, position);
        e.mem.set_u32(this + 0x14, limit);
        (this, buffer)
    }

    /// A `memcpy` double that copies for real.
    fn real_mem_copy(e: &mut Engine) {
        e.register_double(MEM_COPY, |e, a| {
            for i in 0..a[2] {
                let byte = e.mem.u8(a[1] + i);
                e.mem.set_u8(a[0] + i, byte);
            }
            Ret::default()
        });
    }

    /// A write double that answers the given counts in turn (then 0).
    fn write_counts(e: &mut Engine, counts: &[u32]) -> Calls {
        let calls: Calls = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&calls);
        let mut counts = counts.to_vec();
        counts.reverse();
        e.register_double(SLOT_WRITE, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            Ret {
                eax: counts.pop().unwrap_or(0),
                ..Ret::default()
            }
        });
        calls
    }

    #[test]
    fn flush_buffer_writes_in_pieces_until_done() {
        let mut e = engine();
        let stream = inner_stream(&mut e);
        let (this, buffer) = writer(&mut e, stream, 10, 16);
        let calls = write_counts(&mut e, &[4, 6]);
        assert_eq!(e.call(0x00d2_7210, &args![this]).u32(), 10);
        assert_eq!(
            *calls.borrow(),
            vec![vec![stream, buffer, 10], vec![stream, buffer + 4, 6],]
        );
        assert_eq!(e.mem.u32(this + 0x10), 0);
        // A stream that writes nothing leaves the count.
        let (this, _) = writer(&mut e, stream, 10, 16);
        write_counts(&mut e, &[3]);
        assert_eq!(e.call(0x00d2_7210, &args![this]).u32(), 3);
        assert_eq!(e.mem.u32(this + 0x10), 10);
        // An empty buffer calls nothing and keeps the count 0.
        let (this, _) = writer(&mut e, stream, 0, 16);
        let calls = write_counts(&mut e, &[9]);
        assert_eq!(e.call(0x00d2_7210, &args![this]).u32(), 0);
        assert!(calls.borrow().is_empty());
        // Without a stream the answer is 0 and the count is left.
        let (this, _) = writer(&mut e, 0, 5, 16);
        assert_eq!(e.call(0x00d2_7210, &args![this]).u32(), 0);
        assert_eq!(e.mem.u32(this + 0x10), 5);
    }

    #[test]
    fn write_fills_flushes_and_copies_the_rest() {
        let mut e = engine();
        real_mem_copy(&mut e);
        let stream = inner_stream(&mut e);
        let (this, buffer) = writer(&mut e, stream, 5, 8);
        // The inner stream accepts everything it is given.
        e.register_double(SLOT_WRITE, |_, a| Ret {
            eax: a[2],
            ..Ret::default()
        });
        let data = e.mem.alloc(16);
        e.mem.write(data, b"ABCDEFGHIJKL");
        assert_eq!(e.call(0x00d2_7260, &args![this, data, 12u32]).u32(), 12);
        assert_eq!(e.mem.u32(this + 0x10), 1);
        assert_eq!(e.mem.bytes(buffer, 1), b"L".to_vec());
        assert_eq!(calls_to(&e, SLOT_WRITE).len(), 2);
        assert_eq!(calls_to(&e, SLOT_WRITE)[0], vec![stream, buffer, 8]);
        // Data that fits is only copied.
        let (this, buffer) = writer(&mut e, stream, 2, 8);
        assert_eq!(e.call(0x00d2_7260, &args![this, data, 4u32]).u32(), 4);
        assert_eq!(e.mem.u32(this + 0x10), 6);
        assert_eq!(e.mem.bytes(buffer + 2, 4), b"ABCD".to_vec());
        assert_eq!(calls_to(&e, SLOT_WRITE).len(), 2);
        // A flush that writes nothing stops with the count taken so far (12 - 9).
        let (this, _) = writer(&mut e, stream, 5, 8);
        answer(&mut e, SLOT_WRITE, 0);
        assert_eq!(e.call(0x00d2_7260, &args![this, data, 12u32]).u32(), 3);
        assert_eq!(e.mem.u32(this + 0x10), 8);
    }

    #[test]
    fn flush_writes_the_buffer_then_flushes_the_stream() {
        let mut e = engine();
        let stream = inner_stream(&mut e);
        let (this, buffer) = writer(&mut e, stream, 3, 16);
        e.register_double(SLOT_WRITE, |_, a| Ret {
            eax: a[2],
            ..Ret::default()
        });
        answer(&mut e, SLOT_FLUSH, 0x55);
        assert_eq!(e.call(0x00d2_72e0, &args![this]).u32(), 0x55);
        assert_eq!(calls_to(&e, SLOT_WRITE), vec![vec![stream, buffer, 3]]);
        assert_eq!(calls_to(&e, SLOT_FLUSH), vec![vec![stream]]);
        // Without a stream only the (empty) buffer flush answers.
        let (this, _) = writer(&mut e, 0, 3, 16);
        assert_eq!(e.call(0x00d2_72e0, &args![this]).u32(), 0);
        assert_eq!(calls_to(&e, SLOT_FLUSH).len(), 1);
    }

    /// A double storing `value` into the byte whose address it gets and answering the address.
    fn store_byte(e: &mut Engine, address: u32, value: u8) {
        e.register_double(address, move |e, a| {
            e.mem.set_u8(a[1], value);
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
    }

    #[test]
    fn is_ok_asks_the_stream_or_compares_the_position() {
        let mut e = engine();
        let stream = inner_stream(&mut e);
        let out = e.mem.alloc(4);
        store_byte(&mut e, SLOT_IS_OK, 1);
        let (this, _) = writer(&mut e, stream, 3, 16);
        assert_eq!(e.call(0x00d2_7300, &args![this, out]).u32(), out);
        assert_eq!(e.mem.u8(out), 1);
        store_byte(&mut e, SLOT_IS_OK, 0);
        e.call(0x00d2_7300, &args![this, out]);
        assert_eq!(e.mem.u8(out), 0);
        // Without a stream: position differs from the limit.
        let (this, _) = writer(&mut e, 0, 3, 16);
        e.call(0x00d2_7300, &args![this, out]);
        assert_eq!(e.mem.u8(out), 1);
        e.mem.set_u32(this + 0x10, 16);
        assert_eq!(e.call(0x00d2_7300, &args![this, out]).u32(), out);
        assert_eq!(e.mem.u8(out), 0);
    }

    #[test]
    fn seek_tell_supported_asks_the_stream_or_says_yes() {
        let mut e = engine();
        let stream = inner_stream(&mut e);
        let out = e.mem.alloc(4);
        store_byte(&mut e, SLOT_SUPPORTED, 0);
        let (this, _) = writer(&mut e, stream, 3, 16);
        e.mem.set_u8(out, 7);
        assert_eq!(e.call(0x00d2_7340, &args![this, out]).u32(), out);
        assert_eq!(e.mem.u8(out), 0);
        store_byte(&mut e, SLOT_SUPPORTED, 1);
        e.call(0x00d2_7340, &args![this, out]);
        assert_eq!(e.mem.u8(out), 1);
        let (this, _) = writer(&mut e, 0, 3, 16);
        e.mem.set_u8(out, 0);
        e.call(0x00d2_7340, &args![this, out]);
        assert_eq!(e.mem.u8(out), 1);
    }

    #[test]
    fn seek_flushes_and_forwards_or_moves_the_position() {
        let mut e = engine();
        let stream = inner_stream(&mut e);
        let (this, buffer) = writer(&mut e, stream, 3, 16);
        e.register_double(SLOT_WRITE, |_, a| Ret {
            eax: a[2],
            ..Ret::default()
        });
        answer(&mut e, SLOT_SEEK, 0x66);
        assert_eq!(e.call(0x00d2_7380, &args![this, 40u32, 1u32]).u32(), 0x66);
        assert_eq!(calls_to(&e, SLOT_WRITE), vec![vec![stream, buffer, 3]]);
        assert_eq!(calls_to(&e, SLOT_SEEK), vec![vec![stream, 40, 1]]);
        // Without a stream: absolute, forward, backward, unknown mode, below 0, above the limit.
        let (this, _) = writer(&mut e, 0, 10, 16);
        let cases = [
            (4u32, 0u32, 4u32, 0u32),
            (3, 1, 13, 0),
            (3, 2, 7, 0),
            (3, 3, 0, 1),
            (11, 2, 0, 1),
            (20, 0, 16, 1),
            (6, 1, 16, 0),
            (7, 1, 16, 1),
        ];
        for (offset, mode, position, result) in cases {
            e.mem.set_u32(this + 0x10, 10);
            let got = e.call(0x00d2_7380, &args![this, offset, mode]).u32();
            assert_eq!((e.mem.u32(this + 0x10), got), (position, result));
        }
    }

    #[test]
    fn tell_adds_the_stream_position() {
        let mut e = engine();
        let stream = inner_stream(&mut e);
        let (this, _) = writer(&mut e, stream, 10, 16);
        answer(&mut e, SLOT_TELL, 100);
        assert_eq!(e.call(0x00d2_73f0, &args![this]).i32(), 110);
        answer(&mut e, SLOT_TELL, 0xffff_fff0);
        assert_eq!(e.call(0x00d2_73f0, &args![this]).i32(), -1);
        let (this, _) = writer(&mut e, 0, 10, 16);
        assert_eq!(e.call(0x00d2_73f0, &args![this]).i32(), 10);
    }

    #[test]
    fn calc_content_statistics_reports_the_buffer_and_the_stream() {
        let mut e = engine();
        let (this, buffer) = writer(&mut e, 0x4444, 3, 16);
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0xc, 0x0900_0101);
        e.mem.set_u32(vtable + 0x14, 0x0900_0102);
        let visitor = e.mem.alloc(8);
        e.mem.set_u32(visitor, vtable);
        answer(&mut e, 0x0900_0101, 0);
        answer(&mut e, 0x0900_0102, 0);
        e.call(0x00d2_7420, &args![this, visitor, 0u32]);
        assert_eq!(
            calls_to(&e, 0x0900_0102),
            vec![vec![visitor, 1, 0x0108_0668, buffer, 16, 0]]
        );
        assert_eq!(
            calls_to(&e, 0x0900_0101),
            vec![vec![visitor, 0x010c_e43c, 0x4444, 0]]
        );
    }

    /// A memory router whose allocator slots `4`/`8` are not used; the aligned functions are doubles.
    fn router(e: &mut Engine) -> u32 {
        let allocator = e.mem.alloc(8);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        answer(e, MEMORY_ROUTER, router);
        allocator
    }

    #[test]
    fn constructor_allocates_the_buffer() {
        let mut e = engine();
        let allocator = router(&mut e);
        answer(&mut e, ADD_REFERENCE, 0);
        answer(&mut e, ALIGNED_ALLOC, 0x7000);
        let this = e.mem.alloc(0x20);
        let got = e.call(0x00d2_7460, &args![this, 0x4444u32, 0x100u32]);
        assert_eq!(got.u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u8(this + 0x18), 1);
        assert_eq!(e.mem.u32(this + 8), 0x4444);
        assert_eq!(e.mem.u32(this + 0xc), 0x7000);
        assert_eq!(e.mem.u32(this + 0x10), 0);
        assert_eq!(e.mem.u32(this + 0x14), 0x100);
        assert_eq!(calls_to(&e, ADD_REFERENCE), vec![vec![0x4444]]);
        assert_eq!(
            calls_to(&e, ALIGNED_ALLOC),
            vec![vec![allocator, 0x100, 0x40]]
        );
        // Without a stream no reference is added.
        e.call(0x00d2_7460, &args![this, 0u32, 0x80u32]);
        assert_eq!(calls_to(&e, ADD_REFERENCE).len(), 1);
        assert_eq!(e.mem.u32(this + 0x14), 0x80);
    }

    #[test]
    fn destructor_flushes_releases_and_frees_the_buffer() {
        let mut e = engine();
        let allocator = router(&mut e);
        let stream = inner_stream(&mut e);
        answer(&mut e, REMOVE_REFERENCE, 0);
        answer(&mut e, ALIGNED_FREE, 0);
        answer(&mut e, SLOT_FLUSH, 0);
        e.register_double(SLOT_WRITE, |_, a| Ret {
            eax: a[2],
            ..Ret::default()
        });
        let (this, buffer) = writer(&mut e, stream, 2, 16);
        e.mem.set_u8(this + 0x18, 1);
        e.call(0x00d2_7520, &args![this]);
        assert_eq!(calls_to(&e, SLOT_WRITE), vec![vec![stream, buffer, 2]]);
        assert_eq!(calls_to(&e, SLOT_FLUSH), vec![vec![stream]]);
        assert_eq!(calls_to(&e, REMOVE_REFERENCE), vec![vec![stream]]);
        assert_eq!(calls_to(&e, ALIGNED_FREE), vec![vec![allocator, buffer]]);
        assert_eq!(e.mem.u32(this), BASE_VTABLE);
        // A buffer the object does not own is kept, and no stream means no release.
        let (this, _) = writer(&mut e, 0, 0, 16);
        e.call(0x00d2_7520, &args![this]);
        assert_eq!(calls_to(&e, REMOVE_REFERENCE).len(), 1);
        assert_eq!(calls_to(&e, ALIGNED_FREE).len(), 1);
    }
}

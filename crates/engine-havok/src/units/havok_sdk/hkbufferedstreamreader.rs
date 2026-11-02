//! `Havok SDK/hkbufferedstreamreader.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00d2bca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00d2_bc70`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00d2bca0(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00d2_bc70, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// `memcpy` (`00cb1260`, cdecl `(destination, source, bytes)`) and `memmove` (`00cb1270`,
/// cdecl `(destination, source, bytes)`).
const MEM_COPY: u32 = 0x00cb_1260;
const MEM_MOVE: u32 = 0x00cb_1270;
/// The reference-count functions run on the inner stream (`ECX` = the object): add
/// (`00c90510`) and remove (`00c905b0`).
const ADD_REFERENCE: u32 = 0x00c9_0510;
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// `alignedAlloc` (`00c85620`, `(allocator, size, alignment)`) and `alignedFree` (`00c85660`,
/// `(allocator, block)`).
const ALIGNED_ALLOC: u32 = 0x00c8_5620;
const ALIGNED_FREE: u32 = 0x00c8_5660;
/// The vtable of `hkBufferedStreamReader` and the base vtable the destructor leaves.
const VTABLE: u32 = 0x010c_e688;
const BASE_VTABLE: u32 = 0x0102_e388;
/// The reader's own virtual slot `0x30`: `refillBuffer` (the vtable entry is `00d2b990`).
const SLOT_REFILL: u32 = 0x30;

// Translated from 00d2b900 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkBufferedStreamReader::prepareBufferForRefill` (Xbox PDB; `ECX` = `this`). The fields
/// are the buffer (`+0xc`), position (`+0x10`), end (`+0x14`), capacity (`+0x18`), mark
/// (`+0x1c`) and mark limit (`+0x20`). Without a mark (negative) the position and end are
/// reset to 0. A mark further back than the limit (signed compare of `position - mark`) drops
/// the mark (both set to -1) and resets position and end. A mark above 0 moves the
/// `position - mark` bytes since the mark (`memmove`) to the start of the buffer plus the
/// padding that makes their count a multiple of 0x200 (`0x200 - count % 0x200`, 0 when it is
/// a multiple); the mark becomes that padding and position and end become the count rounded
/// up to a multiple of 0x200. A mark of 0 changes nothing.
pub fn hk_buffered_stream_reader_prepare_buffer_for_refill(e: &mut Engine, this: Ptr) {
    let t = this.addr();
    let mark = e.mem.u32(t + 0x1c) as i32;
    if mark < 0 {
        e.mem.set_u32(t + 0x10, 0);
        e.mem.set_u32(t + 0x14, 0);
        return;
    }
    let since_mark = (e.mem.u32(t + 0x10) as i32).wrapping_sub(mark);
    if since_mark > e.mem.u32(t + 0x20) as i32 {
        e.mem.set_u32(t + 0x10, 0);
        e.mem.set_u32(t + 0x14, 0);
        e.mem.set_u32(t + 0x1c, 0xffff_ffff);
        e.mem.set_u32(t + 0x20, 0xffff_ffff);
        return;
    }
    if mark > 0 {
        let remainder = since_mark % 0x200;
        let padding = if remainder != 0 { 0x200 - remainder } else { 0 };
        let buffer = e.mem.u32(t + 0xc);
        e.call(
            MEM_MOVE,
            &args![
                buffer.wrapping_add(padding as u32),
                (mark as u32).wrapping_add(buffer),
                since_mark as u32
            ],
        );
        let blocks = since_mark / 0x200 + i32::from(remainder != 0);
        let end = (blocks << 9) as u32;
        e.mem.set_u32(t + 0x1c, padding as u32);
        e.mem.set_u32(t + 0x10, end);
        e.mem.set_u32(t + 0x14, end);
    }
}

// Translated from 00d2b990 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkBufferedStreamReader::refillBuffer` (Xbox PDB; `ECX` = `this`): when the inner stream
/// (`+8`; slot `0xc` stores its answer through a local byte) is not ok the answer is true.
/// Otherwise the buffer is prepared ([`hk_buffered_stream_reader_prepare_buffer_for_refill`])
/// and the free space (`capacity - end`), when positive, is read from the stream (slot `0x10`,
/// `(buffer + position, space)`) and added to the end. A short read answers whether nothing
/// at all was read; a full read answers false (the exe's loop for further reads cannot
/// repeat: a full read ends it).
pub fn hk_buffered_stream_reader_refill_buffer(e: &mut Engine, this: Ptr) -> bool {
    let t = this.addr();
    let stream = e.mem.u32(t + 8);
    // The local byte is at +3 of the exe's one-word frame.
    let ok = e.with_stack(4, |e, local| {
        let result = e.vcall(stream, 0xc, &args![local.addr() + 3]).u32();
        e.mem.u8(result)
    });
    if ok == 0 {
        return true;
    }
    hk_buffered_stream_reader_prepare_buffer_for_refill(e, this);
    let space = (e.mem.u32(t + 0x18) as i32).wrapping_sub(e.mem.u32(t + 0x14) as i32);
    let mut total: i32 = 0;
    if space > 0 {
        loop {
            let destination = e.mem.u32(t + 0xc).wrapping_add(e.mem.u32(t + 0x10));
            let stream = e.mem.u32(t + 8);
            let count = e
                .vcall(stream, 0x10, &args![destination, space as u32])
                .u32();
            let end = e.mem.u32(t + 0x14).wrapping_add(count);
            e.mem.set_u32(t + 0x14, end);
            total = total.wrapping_add(count as i32);
            if count != space as u32 {
                return total == 0;
            }
            if total >= space {
                break;
            }
        }
    }
    false
}

// Translated from 00d2ba00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkBufferedStreamReader::read` (Xbox PDB): copies `count` bytes into `buffer`. While more
/// is wanted than is buffered (`end - position`) the buffered bytes are copied, the position
/// moves to the end and the buffer is refilled through the reader's virtual slot `0x30`; a
/// non-zero answer of that stops with the number of bytes copied so far. The rest is copied
/// last and `count` is answered.
pub fn hk_buffered_stream_reader_read(e: &mut Engine, this: Ptr, buffer: u32, count: u32) -> u32 {
    let t = this.addr();
    let mut destination = buffer;
    let mut remaining = count as i32;
    let mut available = (e.mem.u32(t + 0x14) as i32).wrapping_sub(e.mem.u32(t + 0x10) as i32);
    while remaining > available {
        let source = e.mem.u32(t + 0xc).wrapping_add(e.mem.u32(t + 0x10));
        e.call(MEM_COPY, &args![destination, source, available as u32]);
        let position = e.mem.u32(t + 0x10).wrapping_add(available as u32);
        e.mem.set_u32(t + 0x10, position);
        destination = destination.wrapping_add(available as u32);
        remaining = remaining.wrapping_sub(available);
        if e.vcall(t, SLOT_REFILL, &args![]).u32() != 0 {
            return count.wrapping_sub(remaining as u32);
        }
        available = (e.mem.u32(t + 0x14) as i32).wrapping_sub(e.mem.u32(t + 0x10) as i32);
    }
    let source = e.mem.u32(t + 0xc).wrapping_add(e.mem.u32(t + 0x10));
    e.call(MEM_COPY, &args![destination, source, remaining as u32]);
    let position = e.mem.u32(t + 0x10).wrapping_add(remaining as u32);
    e.mem.set_u32(t + 0x10, position);
    count
}

// Translated from 00d2ba80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkBufferedStreamReader::skip` (Xbox PDB): skips `count` bytes. While more than is buffered
/// (`end - position`) is left, the buffered amount is taken off and the buffer refilled
/// through the reader's virtual slot `0x30`; a non-zero answer of that stops with the number
/// skipped so far. The rest moves the position and `count` is answered.
pub fn hk_buffered_stream_reader_skip(e: &mut Engine, this: Ptr, count: u32) -> u32 {
    let t = this.addr();
    let mut left = count as i32;
    let mut available = (e.mem.u32(t + 0x14) as i32).wrapping_sub(e.mem.u32(t + 0x10) as i32);
    if left > available {
        loop {
            left = left.wrapping_sub(available);
            if e.vcall(t, SLOT_REFILL, &args![]).u32() != 0 {
                return count.wrapping_sub(left as u32);
            }
            available = (e.mem.u32(t + 0x14) as i32).wrapping_sub(e.mem.u32(t + 0x10) as i32);
            if left <= available {
                break;
            }
        }
    }
    let position = e.mem.u32(t + 0x10).wrapping_add(left as u32);
    e.mem.set_u32(t + 0x10, position);
    count
}

// Translated from 00d2bad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkBufferedStreamReader::isOk` (Xbox PDB): writes to the byte at `out` and returns `out`.
/// The answer is 1 when data is buffered (position differs from the end) and otherwise
/// whatever the inner stream's slot `0xc` stores through a local byte (non-zero gives 1).
pub fn hk_buffered_stream_reader_is_ok(e: &mut Engine, this: Ptr, out: u32) -> u32 {
    let t = this.addr();
    let buffered = e.mem.u32(t + 0x10) != e.mem.u32(t + 0x14);
    let ok = buffered || {
        let stream = e.mem.u32(t + 8);
        // The local byte is at +3 of the exe's one-word frame.
        let byte = e.with_stack(4, |e, local| {
            let result = e.vcall(stream, 0xc, &args![local.addr() + 3]).u32();
            e.mem.u8(result)
        });
        byte != 0
    };
    e.mem.set_u8(out, u8::from(ok));
    out
}

// Translated from 00d2bb10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkBufferedStreamReader::markSupported` (Xbox PDB): writes to the byte at `out` whether
/// the capacity (`+0x18`) is non-zero and returns `out`.
pub fn hk_buffered_stream_reader_mark_supported(e: &mut Engine, this: Ptr, out: u32) -> u32 {
    let supported = e.mem.u32(this.addr() + 0x18) != 0;
    e.mem.set_u8(out, u8::from(supported));
    out
}

// Translated from 00d2bb20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkBufferedStreamReader::setMark` (Xbox PDB): the mark (`+0x1c`) becomes the position
/// (`+0x10`), the mark limit (`+0x20`) `limit`; answers whether `limit` is above the capacity
/// (`+0x18`; signed compare).
pub fn hk_buffered_stream_reader_set_mark(e: &mut Engine, this: Ptr, limit: u32) -> bool {
    let t = this.addr();
    let position = e.mem.u32(t + 0x10);
    e.mem.set_u32(t + 0x1c, position);
    e.mem.set_u32(t + 0x20, limit);
    limit as i32 > e.mem.u32(t + 0x18) as i32
}

// Translated from 00d2bb40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkBufferedStreamReader::rewindToMark` (Xbox PDB; `ECX` = `this`): with a negative mark
/// (`+0x1c`; signed) the answer is 1; otherwise the position (`+0x10`) becomes the mark and
/// the answer is 0.
pub fn hk_buffered_stream_reader_rewind_to_mark(e: &mut Engine, this: Ptr) -> u32 {
    let t = this.addr();
    let mark = e.mem.u32(t + 0x1c);
    if (mark as i32) < 0 {
        return 1;
    }
    e.mem.set_u32(t + 0x10, mark);
    0
}

// Translated from 00d2bb60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkBufferedStreamReader::seekTellSupported` (Xbox PDB): calls slot `0x24` of the inner
/// stream (`+8`) with `out` and returns `out` (the stream's own answer is not used).
pub fn hk_buffered_stream_reader_seek_tell_supported(e: &mut Engine, this: Ptr, out: u32) -> u32 {
    let stream = e.mem.u32(this.addr() + 8);
    e.vcall(stream, 0x24, &args![out]);
    out
}

// Translated from 00d2bb80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkBufferedStreamReader::seek` (Xbox PDB; `ECX` = `this`): sets the mark and mark limit
/// (`+0x1c`, `+0x20`) to -1 and the position and end (`+0x10`, `+0x14`) to 0, then jumps to
/// slot `0x28` of the inner stream (`+8`) with the caller's stack words unchanged. The
/// decompiler shows no arguments; the callee seen behind that slot (`00d2bd30`) returns with
/// `RET 8`, so they are `(offset, whence)`. The stream's result is returned.
pub fn hk_buffered_stream_reader_seek(e: &mut Engine, this: Ptr, offset: u32, whence: u32) -> u32 {
    let t = this.addr();
    e.mem.set_u32(t + 0x1c, 0xffff_ffff);
    e.mem.set_u32(t + 0x20, 0xffff_ffff);
    e.mem.set_u32(t + 0x10, 0);
    e.mem.set_u32(t + 0x14, 0);
    let stream = e.mem.u32(t + 8);
    e.vcall(stream, 0x28, &args![offset, whence]).u32()
}

// Translated from 00d2bba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkBufferedStreamReader::tell` (Xbox PDB; `ECX` = `this`): the inner stream's position
/// (slot `0x2c` of `+8`); -1 when it is negative, else `position - end` added to it.
pub fn hk_buffered_stream_reader_tell(e: &mut Engine, this: Ptr) -> i32 {
    let t = this.addr();
    let stream = e.mem.u32(t + 8);
    let inner = e.vcall(stream, 0x2c, &args![]).i32();
    if inner < 0 {
        return -1;
    }
    (e.mem.u32(t + 0x10).wrapping_sub(e.mem.u32(t + 0x14)) as i32).wrapping_add(inner)
}

// Translated from 00d2bbd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the buffer part of the reader (unnamed in the exe; `ECX` = the part, which
/// is the reader's fields from `+0xc`): `size` bytes aligned to 0x40 from the memory router's
/// allocator (`+0x10`) go to word 0, words 1 and 2 (position, end) are 0, word 3 (capacity) is
/// `size` and words 4 and 5 (mark, mark limit) are -1. Returns `this`.
pub fn construct_buffer_00d2bbd0(e: &mut Engine, this: Ptr, size: u32) -> Ptr {
    let t = this.addr();
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e
        .call(ALIGNED_ALLOC, &args![allocator, size, 0x40u32])
        .u32();
    e.mem.set_u32(t, block);
    e.mem.set_u32(t + 4, 0);
    e.mem.set_u32(t + 8, 0);
    e.mem.set_u32(t + 0xc, size);
    e.mem.set_u32(t + 0x10, 0xffff_ffff);
    e.mem.set_u32(t + 0x14, 0xffff_ffff);
    this
}

// Translated from 00d2bc10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the buffer part of the reader (unnamed in the exe; `ECX` = the part): frees
/// the block in word 0 through the memory router's allocator (`alignedFree`).
pub fn destruct_buffer_00d2bc10(e: &mut Engine, this: Ptr) {
    let block = e.mem.u32(this.addr());
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    e.call(ALIGNED_FREE, &args![allocator, block]);
}

// Translated from 00d2bc30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the reader (unnamed in the exe): stores the inner `stream` (`+8`), the
/// 16-bit word 1 (`+6`) and the vtable (`010ce688`), builds the buffer part at `+0xc` for
/// `size` bytes ([`construct_buffer_00d2bbd0`]) and adds a reference to the stream
/// (`00c90510`; there is no null test). Returns `this`.
pub fn construct_00d2bc30(e: &mut Engine, this: Ptr, stream: u32, size: u32) -> Ptr {
    let t = this.addr();
    e.mem.set_u32(t + 8, stream);
    e.mem.set_u16(t + 6, 1);
    e.mem.set_u32(t, VTABLE);
    construct_buffer_00d2bbd0(e, Ptr::new(t + 0xc), size);
    let stream = e.mem.u32(t + 8);
    e.call(ADD_REFERENCE, &args![stream]);
    this
}

// Translated from 00d2bc70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the reader (unnamed in the exe): stores the vtable (`010ce688`), removes the
/// reference on the inner stream (`+8`; `00c905b0`, no null test), destroys the buffer part
/// at `+0xc` ([`destruct_buffer_00d2bc10`]) and stores the base vtable (`0102e388`).
pub fn destructor_00d2bc70(e: &mut Engine, this: Ptr) {
    let t = this.addr();
    let stream = e.mem.u32(t + 8);
    e.mem.set_u32(t, VTABLE);
    e.call(REMOVE_REFERENCE, &args![stream]);
    destruct_buffer_00d2bc10(e, Ptr::new(t + 0xc));
    e.mem.set_u32(t, BASE_VTABLE);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d2_bca0, deleting_destructor_00d2bca0(Ptr, u8) -> Ptr),
        entry!(
            0x00d2_b900,
            hk_buffered_stream_reader_prepare_buffer_for_refill(Ptr)
        ),
        entry!(0x00d2_b990, hk_buffered_stream_reader_refill_buffer(Ptr) -> bool),
        entry!(0x00d2_ba00, hk_buffered_stream_reader_read(Ptr, u32, u32) -> u32),
        entry!(0x00d2_ba80, hk_buffered_stream_reader_skip(Ptr, u32) -> u32),
        entry!(0x00d2_bad0, hk_buffered_stream_reader_is_ok(Ptr, u32) -> u32),
        entry!(0x00d2_bb10, hk_buffered_stream_reader_mark_supported(Ptr, u32) -> u32),
        entry!(0x00d2_bb20, hk_buffered_stream_reader_set_mark(Ptr, u32) -> bool),
        entry!(0x00d2_bb40, hk_buffered_stream_reader_rewind_to_mark(Ptr) -> u32),
        entry!(0x00d2_bb60, hk_buffered_stream_reader_seek_tell_supported(Ptr, u32) -> u32),
        entry!(0x00d2_bb80, hk_buffered_stream_reader_seek(Ptr, u32, u32) -> u32),
        entry!(0x00d2_bba0, hk_buffered_stream_reader_tell(Ptr) -> i32),
        entry!(0x00d2_bbd0, construct_buffer_00d2bbd0(Ptr, u32) -> Ptr),
        entry!(0x00d2_bc10, destruct_buffer_00d2bc10(Ptr)),
        entry!(0x00d2_bc30, construct_00d2bc30(Ptr, u32, u32) -> Ptr),
        entry!(0x00d2_bc70, destructor_00d2bc70(Ptr)),
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
    fn deleting_destructor_00d2bca0_frees_only_when_bit_zero_is_set() {
        check(0x00d2_bca0, 0x00d2_bc70);
    }
}

#[cfg(test)]
mod translated_tests {
    use super::*;

    /// Doubles behind the slots of the inner stream (`0xc` isOk, `0x10` read, `0x24`
    /// seekTellSupported, `0x28` seek, `0x2c` tell) and the reader's own slot `0x30`.
    const SLOT_IS_OK: u32 = 0x0900_000c;
    const SLOT_READ: u32 = 0x0900_0010;
    const SLOT_SUPPORTED: u32 = 0x0900_0024;
    const SLOT_SEEK: u32 = 0x0900_0028;
    const SLOT_TELL: u32 = 0x0900_002c;
    const SLOT_REFILL_DOUBLE: u32 = 0x0900_0030;

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

    /// An inner stream object whose vtable holds the slot doubles.
    fn inner_stream(e: &mut Engine) -> u32 {
        let vtable = e.mem.alloc(0x40);
        for slot in [SLOT_IS_OK, SLOT_READ, SLOT_SUPPORTED, SLOT_SEEK, SLOT_TELL] {
            e.mem.set_u32(vtable + (slot & 0xff), slot);
        }
        let stream = e.mem.alloc(0x10);
        e.mem.set_u32(stream, vtable);
        stream
    }

    /// A reader with an 8-byte buffer holding "ABCDEFGH", the given position and end, mark
    /// -1, capacity 8, the inner stream and the vtable whose slot `0x30` is a double.
    fn reader(e: &mut Engine, stream: u32, position: u32, end: u32) -> (u32, u32) {
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0x30, SLOT_REFILL_DOUBLE);
        let this = e.mem.alloc(0x30);
        let buffer = e.mem.alloc(0x800);
        e.mem.write(buffer, b"ABCDEFGH");
        e.mem.set_u32(this, vtable);
        e.mem.set_u32(this + 8, stream);
        e.mem.set_u32(this + 0xc, buffer);
        e.mem.set_u32(this + 0x10, position);
        e.mem.set_u32(this + 0x14, end);
        e.mem.set_u32(this + 0x18, 8);
        e.mem.set_u32(this + 0x1c, 0xffff_ffff);
        e.mem.set_u32(this + 0x20, 0xffff_ffff);
        (this, buffer)
    }

    /// `memcpy`/`memmove` doubles that copy for real.
    fn real_copies(e: &mut Engine) {
        for address in [MEM_COPY, MEM_MOVE] {
            e.register_double(address, |e, a| {
                let bytes = e.mem.bytes(a[1], a[2]);
                e.mem.write(a[0], &bytes);
                Ret::default()
            });
        }
    }

    /// A refill double that puts "IJKLMNOP" in `buffer`, sets position 0 and end 8, and answers `status`.
    fn refill_with(e: &mut Engine, buffer: u32, status: u32) {
        e.register_double(SLOT_REFILL_DOUBLE, move |e, a| {
            e.mem.write(buffer, b"IJKLMNOP");
            e.mem.set_u32(a[0] + 0x10, 0);
            e.mem.set_u32(a[0] + 0x14, 8);
            Ret {
                eax: status,
                ..Ret::default()
            }
        });
    }

    #[test]
    fn prepare_buffer_for_refill_resets_or_moves_the_marked_bytes() {
        // No mark: position and end are reset.
        let mut e = engine();
        let (this, _) = reader(&mut e, 0, 5, 9);
        e.call(0x00d2_b900, &args![this]);
        assert_eq!((e.mem.u32(this + 0x10), e.mem.u32(this + 0x14)), (0, 0));
        // A mark further back than its limit is dropped.
        let (this, _) = reader(&mut e, 0, 100, 120);
        e.mem.set_u32(this + 0x1c, 10);
        e.mem.set_u32(this + 0x20, 50);
        e.call(0x00d2_b900, &args![this]);
        let fields: Vec<u32> = [0x10, 0x14, 0x1c, 0x20]
            .iter()
            .map(|o| e.mem.u32(this + o))
            .collect();
        assert_eq!(fields, vec![0, 0, 0xffff_ffff, 0xffff_ffff]);
        assert!(calls_to(&e, MEM_MOVE).is_empty());
        // A mark of 0 changes nothing.
        let (this, _) = reader(&mut e, 0, 0x20, 0x40);
        e.mem.set_u32(this + 0x1c, 0);
        e.mem.set_u32(this + 0x20, 0x100);
        e.call(0x00d2_b900, &args![this]);
        assert_eq!(
            (e.mem.u32(this + 0x10), e.mem.u32(this + 0x14)),
            (0x20, 0x40)
        );
        assert!(calls_to(&e, MEM_MOVE).is_empty());
        // A mark above 0: 0x250 bytes since the mark are moved behind 0x1b0 bytes of padding.
        real_copies(&mut e);
        let (this, buffer) = reader(&mut e, 0, 0x280, 0x300);
        e.mem.set_u32(this + 0x1c, 0x30);
        e.mem.set_u32(this + 0x20, 0x1000);
        e.call(0x00d2_b900, &args![this]);
        assert_eq!(
            calls_to(&e, MEM_MOVE),
            vec![vec![buffer + 0x1b0, buffer + 0x30, 0x250]]
        );
        let fields: Vec<u32> = [0x10, 0x14, 0x1c]
            .iter()
            .map(|o| e.mem.u32(this + o))
            .collect();
        assert_eq!(fields, vec![0x400, 0x400, 0x1b0]);
        // A multiple of 0x200 needs no padding.
        let (this, buffer) = reader(&mut e, 0, 0x410, 0x500);
        e.mem.set_u32(this + 0x1c, 0x10);
        e.mem.set_u32(this + 0x20, 0x1000);
        e.call(0x00d2_b900, &args![this]);
        assert_eq!(
            calls_to(&e, MEM_MOVE)[1],
            vec![buffer, buffer + 0x10, 0x400]
        );
        let fields: Vec<u32> = [0x10, 0x14, 0x1c]
            .iter()
            .map(|o| e.mem.u32(this + o))
            .collect();
        assert_eq!(fields, vec![0x400, 0x400, 0]);
    }

    #[test]
    fn refill_buffer_reads_the_free_space_from_the_stream() {
        let mut e = engine();
        let stream = inner_stream(&mut e);
        // A stream that is not ok: true, nothing read.
        store_byte(&mut e, SLOT_IS_OK, 0);
        answer(&mut e, SLOT_READ, 0);
        let (this, _) = reader(&mut e, stream, 0, 0);
        assert!(e.call(0x00d2_b990, &args![this]).bool());
        assert!(calls_to(&e, SLOT_READ).is_empty());
        // A full read: false, the end moves.
        store_byte(&mut e, SLOT_IS_OK, 1);
        answer(&mut e, SLOT_READ, 8);
        let (this, buffer) = reader(&mut e, stream, 0, 0);
        assert!(!e.call(0x00d2_b990, &args![this]).bool());
        assert_eq!(calls_to(&e, SLOT_READ), vec![vec![stream, buffer, 8]]);
        assert_eq!(e.mem.u32(this + 0x14), 8);
        // A short read is true only when nothing at all came.
        answer(&mut e, SLOT_READ, 3);
        let (this, _) = reader(&mut e, stream, 0, 0);
        assert!(!e.call(0x00d2_b990, &args![this]).bool());
        assert_eq!(e.mem.u32(this + 0x14), 3);
        answer(&mut e, SLOT_READ, 0);
        let (this, _) = reader(&mut e, stream, 0, 0);
        assert!(e.call(0x00d2_b990, &args![this]).bool());
        // No free space: nothing is read.
        let (this, _) = reader(&mut e, stream, 0, 0);
        e.mem.set_u32(this + 0x18, 0);
        let before = calls_to(&e, SLOT_READ).len();
        assert!(!e.call(0x00d2_b990, &args![this]).bool());
        assert_eq!(calls_to(&e, SLOT_READ).len(), before);
    }

    #[test]
    fn read_copies_buffered_bytes_and_refills() {
        let mut e = engine();
        real_copies(&mut e);
        // Everything is buffered.
        let (this, _) = reader(&mut e, 0, 2, 6);
        let out = e.mem.alloc(16);
        assert_eq!(e.call(0x00d2_ba00, &args![this, out, 3u32]).u32(), 3);
        assert_eq!(e.mem.bytes(out, 3), b"CDE".to_vec());
        assert_eq!(e.mem.u32(this + 0x10), 5);
        // More than is buffered: the rest comes after a refill.
        let (this, buffer2) = reader(&mut e, 0, 2, 6);
        refill_with(&mut e, buffer2, 0);
        assert_eq!(e.call(0x00d2_ba00, &args![this, out, 10u32]).u32(), 10);
        assert_eq!(e.mem.bytes(out, 10), b"CDEFIJKLMN".to_vec());
        assert_eq!(e.mem.u32(this + 0x10), 6);
        // A refill that fails stops with the bytes copied so far.
        let (this, buffer3) = reader(&mut e, 0, 2, 6);
        refill_with(&mut e, buffer3, 1);
        assert_eq!(e.call(0x00d2_ba00, &args![this, out, 10u32]).u32(), 4);
    }

    #[test]
    fn skip_moves_the_position_and_refills() {
        let mut e = engine();
        let (this, _) = reader(&mut e, 0, 2, 6);
        assert_eq!(e.call(0x00d2_ba80, &args![this, 3u32]).u32(), 3);
        assert_eq!(e.mem.u32(this + 0x10), 5);
        let (this, buffer) = reader(&mut e, 0, 2, 6);
        refill_with(&mut e, buffer, 0);
        assert_eq!(e.call(0x00d2_ba80, &args![this, 10u32]).u32(), 10);
        assert_eq!(e.mem.u32(this + 0x10), 6);
        assert_eq!(calls_to(&e, SLOT_REFILL_DOUBLE), vec![vec![this]]);
        let (this, buffer) = reader(&mut e, 0, 2, 6);
        refill_with(&mut e, buffer, 1);
        assert_eq!(e.call(0x00d2_ba80, &args![this, 10u32]).u32(), 4);
    }

    #[test]
    fn is_ok_prefers_buffered_data() {
        let mut e = engine();
        let stream = inner_stream(&mut e);
        let out = e.mem.alloc(4);
        store_byte(&mut e, SLOT_IS_OK, 0);
        let (this, _) = reader(&mut e, stream, 2, 6);
        assert_eq!(e.call(0x00d2_bad0, &args![this, out]).u32(), out);
        assert_eq!(e.mem.u8(out), 1);
        assert!(calls_to(&e, SLOT_IS_OK).is_empty());
        let (this, _) = reader(&mut e, stream, 6, 6);
        e.call(0x00d2_bad0, &args![this, out]);
        assert_eq!(e.mem.u8(out), 0);
        store_byte(&mut e, SLOT_IS_OK, 1);
        e.call(0x00d2_bad0, &args![this, out]);
        assert_eq!(e.mem.u8(out), 1);
    }

    #[test]
    fn mark_supported_tests_the_capacity() {
        let mut e = engine();
        let out = e.mem.alloc(4);
        let (this, _) = reader(&mut e, 0, 0, 0);
        assert_eq!(e.call(0x00d2_bb10, &args![this, out]).u32(), out);
        assert_eq!(e.mem.u8(out), 1);
        e.mem.set_u32(this + 0x18, 0);
        e.call(0x00d2_bb10, &args![this, out]);
        assert_eq!(e.mem.u8(out), 0);
    }

    #[test]
    fn set_mark_records_the_position_and_limit() {
        let mut e = engine();
        let (this, _) = reader(&mut e, 0, 7, 8);
        assert!(!e.call(0x00d2_bb20, &args![this, 4u32]).bool());
        assert_eq!((e.mem.u32(this + 0x1c), e.mem.u32(this + 0x20)), (7, 4));
        assert!(e.call(0x00d2_bb20, &args![this, 9u32]).bool());
        // The compare is signed: -1 is below the capacity.
        assert!(!e.call(0x00d2_bb20, &args![this, 0xffff_ffffu32]).bool());
    }

    #[test]
    fn rewind_to_mark_needs_a_mark() {
        let mut e = engine();
        let (this, _) = reader(&mut e, 0, 7, 8);
        assert_eq!(e.call(0x00d2_bb40, &args![this]).u32(), 1);
        assert_eq!(e.mem.u32(this + 0x10), 7);
        e.mem.set_u32(this + 0x1c, 5);
        assert_eq!(e.call(0x00d2_bb40, &args![this]).u32(), 0);
        assert_eq!(e.mem.u32(this + 0x10), 5);
    }

    #[test]
    fn seek_tell_supported_forwards_the_out_pointer() {
        let mut e = engine();
        let stream = inner_stream(&mut e);
        answer(&mut e, SLOT_SUPPORTED, 0x1234);
        let (this, _) = reader(&mut e, stream, 0, 0);
        assert_eq!(e.call(0x00d2_bb60, &args![this, 0x5000u32]).u32(), 0x5000);
        assert_eq!(calls_to(&e, SLOT_SUPPORTED), vec![vec![stream, 0x5000]]);
    }

    #[test]
    fn seek_clears_the_buffer_and_forwards() {
        let mut e = engine();
        let stream = inner_stream(&mut e);
        answer(&mut e, SLOT_SEEK, 0x77);
        let (this, _) = reader(&mut e, stream, 3, 6);
        e.mem.set_u32(this + 0x1c, 1);
        assert_eq!(e.call(0x00d2_bb80, &args![this, 40u32, 2u32]).u32(), 0x77);
        assert_eq!(calls_to(&e, SLOT_SEEK), vec![vec![stream, 40, 2]]);
        let fields: Vec<u32> = [0x10, 0x14, 0x1c, 0x20]
            .iter()
            .map(|o| e.mem.u32(this + o))
            .collect();
        assert_eq!(fields, vec![0, 0, 0xffff_ffff, 0xffff_ffff]);
    }

    #[test]
    fn tell_subtracts_the_buffered_bytes() {
        let mut e = engine();
        let stream = inner_stream(&mut e);
        let (this, _) = reader(&mut e, stream, 2, 6);
        answer(&mut e, SLOT_TELL, 100);
        assert_eq!(e.call(0x00d2_bba0, &args![this]).i32(), 96);
        answer(&mut e, SLOT_TELL, 0xffff_ffff);
        assert_eq!(e.call(0x00d2_bba0, &args![this]).i32(), -1);
    }

    /// A memory router with an allocator object; returns the allocator.
    fn router(e: &mut Engine) -> u32 {
        let allocator = e.mem.alloc(8);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        answer(e, MEMORY_ROUTER, router);
        allocator
    }

    #[test]
    fn buffer_constructor_fills_the_six_words() {
        let mut e = engine();
        let allocator = router(&mut e);
        answer(&mut e, ALIGNED_ALLOC, 0x7000);
        let this = e.mem.alloc(0x20);
        assert_eq!(e.call(0x00d2_bbd0, &args![this, 0x400u32]).u32(), this);
        let words: Vec<u32> = (0..6).map(|i| e.mem.u32(this + 4 * i)).collect();
        assert_eq!(words, vec![0x7000, 0, 0, 0x400, 0xffff_ffff, 0xffff_ffff]);
        assert_eq!(
            calls_to(&e, ALIGNED_ALLOC),
            vec![vec![allocator, 0x400, 0x40]]
        );
    }

    #[test]
    fn buffer_destructor_frees_the_block() {
        let mut e = engine();
        let allocator = router(&mut e);
        answer(&mut e, ALIGNED_FREE, 0);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this, 0x7000);
        e.call(0x00d2_bc10, &args![this]);
        assert_eq!(calls_to(&e, ALIGNED_FREE), vec![vec![allocator, 0x7000]]);
    }

    #[test]
    fn constructor_builds_the_buffer_and_references_the_stream() {
        let mut e = engine();
        router(&mut e);
        answer(&mut e, ALIGNED_ALLOC, 0x7000);
        answer(&mut e, ADD_REFERENCE, 0);
        let this = e.mem.alloc(0x30);
        assert_eq!(
            e.call(0x00d2_bc30, &args![this, 0x4444u32, 0x200u32]).u32(),
            this
        );
        assert_eq!(e.mem.u32(this), VTABLE);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this + 8), 0x4444);
        assert_eq!(e.mem.u32(this + 0xc), 0x7000);
        assert_eq!(e.mem.u32(this + 0x18), 0x200);
        assert_eq!(calls_to(&e, ADD_REFERENCE), vec![vec![0x4444]]);
    }

    #[test]
    fn destructor_releases_the_stream_and_the_buffer() {
        let mut e = engine();
        let allocator = router(&mut e);
        answer(&mut e, REMOVE_REFERENCE, 0);
        answer(&mut e, ALIGNED_FREE, 0);
        let this = e.mem.alloc(0x30);
        e.mem.set_u32(this + 8, 0x4444);
        e.mem.set_u32(this + 0xc, 0x7000);
        e.call(0x00d2_bc70, &args![this]);
        assert_eq!(calls_to(&e, REMOVE_REFERENCE), vec![vec![0x4444]]);
        assert_eq!(calls_to(&e, ALIGNED_FREE), vec![vec![allocator, 0x7000]]);
        assert_eq!(e.mem.u32(this), BASE_VTABLE);
    }
}

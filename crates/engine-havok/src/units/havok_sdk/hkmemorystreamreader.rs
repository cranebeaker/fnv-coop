//! `Havok SDK/hkmemorystreamreader.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00e05bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00e0_5bb0`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00e05bf0(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00e0_5bb0, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// The reader's fields on the PC (the Xbox PDB has no `hkMemoryStreamReader`; the code shows
// these): the vtable at +0, the 16-bit size at +4 and the 16-bit reference count at +6 (set to
// 1), the buffer at +8, the read position at +0xc, the size at +0x10, the mark at +0x14 (-1:
// none) and the memory type word at +0x18.
const BUFFER: u32 = 0x08;
const POSITION: u32 = 0x0c;
const SIZE: u32 = 0x10;
const MARK: u32 = 0x14;
const MEMORY_TYPE: u32 = 0x18;
/// The reader's vtable (`010ea540`) and the base class's (`0102e388`, written last by the
/// destructor).
const READER_VTABLE: u32 = 0x010e_a540;
const BASE_VTABLE: u32 = 0x0102_e388;
/// `hkString::memCpy` (`00cb1260`, cdecl `(destination, source, size)`), `hkMemoryRouter::easyAlloc`
/// (`00c85690`, cdecl `(allocator, size)`) and `hkMemoryRouter::easyFree` (`00c856c0`, cdecl
/// `(allocator, block)`).
const MEM_COPY: u32 = 0x00cb_1260;
const EASY_ALLOC: u32 = 0x00c8_5690;
const EASY_FREE: u32 = 0x00c8_56c0;

// Translated from 00e059a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMemoryStreamReader::isOk` (Xbox PDB): writes to the byte `out` whether the read position
/// differs from the size plus one (the position the reader gives to a read past the end).
/// Returns `out`.
pub fn hk_memory_stream_reader_is_ok(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    let position = e.mem.u32(this.addr() + POSITION);
    let past_the_end = e.mem.u32(this.addr() + SIZE).wrapping_add(1);
    e.mem.set_u8(out.addr(), u8::from(position != past_the_end));
    out
}

// Translated from 00e059c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMemoryStreamReader::markSupported` (Xbox PDB): writes to the byte `out` whether the size
/// field (`+0x10`) is not zero. Returns `out`.
pub fn hk_memory_stream_reader_mark_supported(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    let size = e.mem.u32(this.addr() + SIZE);
    e.mem.set_u8(out.addr(), u8::from(size != 0));
    out
}

// Translated from 00e059d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMemoryStreamReader::setMark` (Xbox PDB): makes the mark (`+0x14`) the read position
/// (`+0xc`). Takes one stack word it never reads. Returns 0.
pub fn hk_memory_stream_reader_set_mark(e: &mut Engine, this: Ptr, _unused_1: u32) -> u32 {
    let position = e.mem.u32(this.addr() + POSITION);
    e.mem.set_u32(this.addr() + MARK, position);
    0
}

// Translated from 00e059e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMemoryStreamReader::rewindToMark` (Xbox PDB): returns 1, changing nothing, when the mark
/// is negative (signed); otherwise makes the read position the mark and returns 0.
pub fn hk_memory_stream_reader_rewind_to_mark(e: &mut Engine, this: Ptr) -> u32 {
    let mark = e.mem.i32(this.addr() + MARK);
    if mark < 0 {
        return 1;
    }
    e.mem.set_i32(this.addr() + POSITION, mark);
    0
}

// Translated from 00e05a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMemoryStreamReader::seekTellSupported` (Xbox PDB): writes 1 to the byte `out` (`this`,
/// in `ECX`, is not read). Returns `out`.
pub fn hk_memory_stream_reader_seek_tell_supported(e: &mut Engine, _this: Ptr, out: Ptr) -> Ptr {
    e.mem.set_u8(out.addr(), 1);
    out
}

// Translated from 00e05a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMemoryStreamReader::seek` (Xbox PDB): the target is `offset` for `whence` 0, the read
/// position plus `offset` for 1, the size minus `offset` for 2, and -1 for any other value. A
/// negative target (signed) makes the position 0 and a target above the size makes it the
/// size, both returning 1; otherwise the position becomes the target and the result is 0.
pub fn hk_memory_stream_reader_seek(e: &mut Engine, this: Ptr, offset: i32, whence: i32) -> u32 {
    let target = match whence {
        0 => offset,
        1 => e.mem.i32(this.addr() + POSITION).wrapping_add(offset),
        2 => e.mem.i32(this.addr() + SIZE).wrapping_sub(offset),
        _ => -1,
    };
    if target < 0 {
        e.mem.set_u32(this.addr() + POSITION, 0);
        return 1;
    }
    let size = e.mem.i32(this.addr() + SIZE);
    if target > size {
        e.mem.set_i32(this.addr() + POSITION, size);
        return 1;
    }
    e.mem.set_i32(this.addr() + POSITION, target);
    0
}

// Translated from 00e05a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMemoryStreamReader::tell` (Xbox PDB): the read position (`+0xc`).
pub fn hk_memory_stream_reader_tell(e: &mut Engine, this: Ptr) -> i32 {
    e.mem.i32(this.addr() + POSITION)
}

/// What the readers do first: ask the object's own `isOk` (slot `0xc`, given the address of a
/// local byte) and read the byte it answers with.
fn stream_is_ok(e: &mut Engine, this: Ptr) -> bool {
    e.with_stack(4, |e, local| {
        let answer = e.vcall(this.addr(), 0xc, &args![local.addr() + 3]).u32();
        e.mem.u8(answer) != 0
    })
}

// Translated from 00e05a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMemoryStreamReader::read` (Xbox PDB), `(buffer, count)`: returns 0 when the reader's own
/// `isOk` (slot `0xc`) says no. Otherwise the amount is `count`, or the bytes left (size minus
/// position) when `count` is not below them (signed); it copies that many bytes from the buffer
/// at the position to `buffer` (`hkString::memCpy`), advances the position by it, and when none
/// were copied for a nonzero `count` makes the position the size plus one. Returns the amount.
pub fn hk_memory_stream_reader_read(e: &mut Engine, this: Ptr, buffer: Ptr, count: i32) -> i32 {
    if !stream_is_ok(e, this) {
        return 0;
    }
    let position = e.mem.u32(this.addr() + POSITION);
    let left = e.mem.i32(this.addr() + SIZE).wrapping_sub(position as i32);
    let amount = count.min(left);
    let source = e.mem.u32(this.addr() + BUFFER).wrapping_add(position);
    e.call(MEM_COPY, &args![buffer, source, amount]);
    let advanced = e
        .mem
        .u32(this.addr() + POSITION)
        .wrapping_add(amount as u32);
    e.mem.set_u32(this.addr() + POSITION, advanced);
    if amount == 0 && count != 0 {
        let past_the_end = e.mem.u32(this.addr() + SIZE).wrapping_add(1);
        e.mem.set_u32(this.addr() + POSITION, past_the_end);
    }
    amount
}

// Translated from 00e05af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMemoryStreamReader::skip` (Xbox PDB), `(count)`: like `read` without the copy. Returns 0
/// when the reader's own `isOk` (slot `0xc`) says no; otherwise the amount is `count` or the
/// bytes left (size minus position) when `count` is not below them (signed). The position
/// advances by it; when it is 0 for a nonzero `count` the position becomes the size plus one.
/// Returns the amount.
pub fn hk_memory_stream_reader_skip(e: &mut Engine, this: Ptr, count: i32) -> i32 {
    if !stream_is_ok(e, this) {
        return 0;
    }
    let size = e.mem.u32(this.addr() + SIZE);
    let position = e.mem.u32(this.addr() + POSITION);
    let left = size.wrapping_sub(position) as i32;
    let amount = count.min(left);
    e.mem
        .set_u32(this.addr() + POSITION, position.wrapping_add(amount as u32));
    if amount == 0 && count != 0 {
        e.mem.set_u32(this.addr() + POSITION, size.wrapping_add(1));
    }
    amount
}

// Translated from 00e05b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMemoryStreamReader::hkMemoryStreamReader` (Xbox PDB), `(buffer, size, memory_type)`:
/// sets the reference count (`+6`) to 1, the vtable, position 0, the size, mark -1 and the
/// memory type. For type 0 it makes its own copy: the memory router's heap allocator
/// (`00c85750`, field `+0x10`) allocates `size` bytes with `easyAlloc` (`00c85690`), the block
/// becomes the buffer and `hkString::memCpy` fills it from `buffer`. Any other type uses
/// `buffer` itself. Returns `this`. The size word at `+4` is not written.
pub fn hk_memory_stream_reader_hk_memory_stream_reader(
    e: &mut Engine,
    this: Ptr,
    buffer: u32,
    size: u32,
    memory_type: u32,
) -> Ptr {
    e.mem.set_u16(this.addr() + 6, 1);
    e.mem.set_u32(this.addr(), READER_VTABLE);
    e.mem.set_u32(this.addr() + POSITION, 0);
    e.mem.set_u32(this.addr() + SIZE, size);
    e.mem.set_u32(this.addr() + MARK, 0xffff_ffff);
    e.mem.set_u32(this.addr() + MEMORY_TYPE, memory_type);
    if memory_type == 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let block = e.call(EASY_ALLOC, &args![allocator, size]).u32();
        e.mem.set_u32(this.addr() + BUFFER, block);
        e.call(MEM_COPY, &args![block, buffer, size]);
    } else {
        e.mem.set_u32(this.addr() + BUFFER, buffer);
    }
    this
}

// Translated from 00e05bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMemoryStreamReader::~hkMemoryStreamReader` (Xbox PDB): sets the reader's vtable; when the
/// memory type (`+0x18`) is 0 or 1 it frees the buffer (`+8`) with `easyFree` (`00c856c0`)
/// through the memory router's heap allocator (`00c85750`, field `+0x10`); then sets the base
/// class's vtable (`0102e388`).
pub fn hk_memory_stream_reader_destructor(e: &mut Engine, this: Ptr) {
    let memory_type = e.mem.u32(this.addr() + MEMORY_TYPE);
    e.mem.set_u32(this.addr(), READER_VTABLE);
    if memory_type == 0 || memory_type == 1 {
        let block = e.mem.u32(this.addr() + BUFFER);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        e.call(EASY_FREE, &args![allocator, block]);
    }
    e.mem.set_u32(this.addr(), BASE_VTABLE);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e059a0, hk_memory_stream_reader_is_ok(Ptr, Ptr) -> Ptr),
        entry!(0x00e059c0, hk_memory_stream_reader_mark_supported(Ptr, Ptr) -> Ptr),
        entry!(0x00e059d0, hk_memory_stream_reader_set_mark(Ptr, u32) -> u32),
        entry!(0x00e059e0, hk_memory_stream_reader_rewind_to_mark(Ptr) -> u32),
        entry!(0x00e05a00, hk_memory_stream_reader_seek_tell_supported(Ptr, Ptr) -> Ptr),
        entry!(0x00e05a10, hk_memory_stream_reader_seek(Ptr, i32, i32) -> u32),
        entry!(0x00e05a70, hk_memory_stream_reader_tell(Ptr) -> i32),
        entry!(0x00e05a80, hk_memory_stream_reader_read(Ptr, Ptr, i32) -> i32),
        entry!(0x00e05af0, hk_memory_stream_reader_skip(Ptr, i32) -> i32),
        entry!(0x00e05b40, hk_memory_stream_reader_hk_memory_stream_reader(Ptr, u32, u32, u32) -> Ptr),
        entry!(0x00e05bb0, hk_memory_stream_reader_destructor(Ptr)),
        entry!(0x00e0_5bf0, deleting_destructor_00e05bf0(Ptr, u8) -> Ptr),
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
    fn deleting_destructor_00e05bf0_frees_only_when_bit_zero_is_set() {
        check(0x00e0_5bf0, 0x00e0_5bb0);
    }

    /// A reader over `bytes` (the buffer at +8, the size, no mark) whose vtable has the real
    /// `isOk` (`00e059a0`) in slot `0xc`.
    fn reader(e: &mut Engine, bytes: &[u8]) -> u32 {
        e.put_vtable(0x4400_0000, &[0, 0, 0, 0x00e0_59a0]);
        let buffer = e.mem.alloc(bytes.len() as u32);
        e.mem.write(buffer, bytes);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this, 0x4400_0000);
        e.mem.set_u32(this + 8, buffer);
        e.mem.set_u32(this + 0x10, bytes.len() as u32);
        e.mem.set_u32(this + 0x14, 0xffff_ffff);
        this
    }

    fn copy_double(e: &mut Engine, a: &[u32]) -> Ret {
        let bytes = e.mem.bytes(a[1], a[2]);
        e.mem.write(a[0], &bytes);
        Ret {
            eax: a[0],
            ..Ret::default()
        }
    }

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.clone().unwrap();
        log.into_iter()
            .filter(|call| call.0 == address)
            .map(|call| call.1)
            .collect()
    }

    #[test]
    fn is_ok_is_false_only_one_past_the_size() {
        let mut e = Engine::new();
        let this = reader(&mut e, &[0; 10]);
        let out = e.mem.alloc(4);
        e.mem.set_u32(this + 0xc, 5);
        assert_eq!(e.call(0x00e0_59a0, &args![this, out]).u32(), out);
        assert_eq!(e.mem.u8(out), 1);
        e.mem.set_u32(this + 0xc, 10);
        e.call(0x00e0_59a0, &args![this, out]);
        assert_eq!(e.mem.u8(out), 1);
        e.mem.set_u32(this + 0xc, 11);
        e.call(0x00e0_59a0, &args![this, out]);
        assert_eq!(e.mem.u8(out), 0);
    }

    #[test]
    fn mark_support_follows_the_size_field() {
        let mut e = Engine::new();
        let this = reader(&mut e, &[0; 10]);
        let out = e.mem.alloc(4);
        assert_eq!(e.call(0x00e0_59c0, &args![this, out]).u32(), out);
        assert_eq!(e.mem.u8(out), 1);
        e.mem.set_u32(this + 0x10, 0);
        e.call(0x00e0_59c0, &args![this, out]);
        assert_eq!(e.mem.u8(out), 0);
    }

    #[test]
    fn marks_remember_and_rewind() {
        let mut e = Engine::new();
        let this = reader(&mut e, &[0; 10]);
        // no mark yet: the rewind fails and the position stays
        e.mem.set_u32(this + 0xc, 4);
        assert_eq!(e.call(0x00e0_59e0, &args![this]).u32(), 1);
        assert_eq!(e.mem.u32(this + 0xc), 4);
        assert_eq!(e.call(0x00e0_59d0, &args![this, 0u32]).u32(), 0);
        assert_eq!(e.mem.u32(this + 0x14), 4);
        e.mem.set_u32(this + 0xc, 9);
        assert_eq!(e.call(0x00e0_59e0, &args![this]).u32(), 0);
        assert_eq!(e.mem.u32(this + 0xc), 4);
    }

    #[test]
    fn seek_and_tell_are_always_supported_and_tell_reads_the_position() {
        let mut e = Engine::new();
        let this = reader(&mut e, &[0; 10]);
        let out = e.mem.alloc(4);
        assert_eq!(e.call(0x00e0_5a00, &args![this, out]).u32(), out);
        assert_eq!(e.mem.u8(out), 1);
        e.mem.set_u32(this + 0xc, 7);
        assert_eq!(e.call(0x00e0_5a70, &args![this]).u32(), 7);
    }

    #[test]
    fn seek_clamps_to_the_buffer() {
        let mut e = Engine::new();
        let this = reader(&mut e, &[0; 10]);
        let seek = |e: &mut Engine, offset: i32, whence: i32| {
            let result = e.call(0x00e0_5a10, &args![this, offset, whence]).u32();
            (result, e.mem.u32(this + 0xc))
        };
        assert_eq!(seek(&mut e, 4, 0), (0, 4));
        assert_eq!(seek(&mut e, 3, 1), (0, 7));
        assert_eq!(seek(&mut e, 2, 2), (0, 8));
        // the end of the buffer itself is allowed, one past it is clamped
        assert_eq!(seek(&mut e, 10, 0), (0, 10));
        assert_eq!(seek(&mut e, 11, 0), (1, 10));
        assert_eq!(seek(&mut e, -1, 0), (1, 0));
        assert_eq!(seek(&mut e, 5, 0), (0, 5));
        assert_eq!(seek(&mut e, -6, 1), (1, 0));
        assert_eq!(seek(&mut e, 11, 2), (1, 0));
        // any other whence gives -1
        assert_eq!(seek(&mut e, 5, 3), (1, 0));
        assert_eq!(seek(&mut e, 5, -1), (1, 0));
    }

    #[test]
    fn read_copies_what_is_left_and_marks_the_end() {
        let mut e = Engine::new();
        e.register(MEM_COPY, copy_double);
        let this = reader(&mut e, &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        let out = e.mem.alloc(16);
        assert_eq!(e.call(0x00e0_5a80, &args![this, out, 4i32]).i32(), 4);
        assert_eq!(e.mem.bytes(out, 4), vec![0, 1, 2, 3]);
        assert_eq!(e.mem.u32(this + 0xc), 4);
        // more than is left: only the rest
        assert_eq!(e.call(0x00e0_5a80, &args![this, out, 100i32]).i32(), 6);
        assert_eq!(e.mem.bytes(out, 6), vec![4, 5, 6, 7, 8, 9]);
        assert_eq!(e.mem.u32(this + 0xc), 10);
        // nothing left: the position goes one past the size
        assert_eq!(e.call(0x00e0_5a80, &args![this, out, 3i32]).i32(), 0);
        assert_eq!(e.mem.u32(this + 0xc), 11);
        // not ok any more: no copy at all
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e0_5a80, &args![this, out, 3i32]).i32(), 0);
        assert!(calls_to(&e, MEM_COPY).is_empty());
        // a zero count at the end is not the end marker
        e.mem.set_u32(this + 0xc, 10);
        assert_eq!(e.call(0x00e0_5a80, &args![this, out, 0i32]).i32(), 0);
        assert_eq!(e.mem.u32(this + 0xc), 10);
    }

    #[test]
    fn skip_moves_the_position_without_copying() {
        let mut e = Engine::new();
        let this = reader(&mut e, &[0; 10]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e0_5af0, &args![this, 4i32]).i32(), 4);
        assert_eq!(e.mem.u32(this + 0xc), 4);
        assert_eq!(e.call(0x00e0_5af0, &args![this, 100i32]).i32(), 6);
        assert_eq!(e.mem.u32(this + 0xc), 10);
        assert_eq!(e.call(0x00e0_5af0, &args![this, 0i32]).i32(), 0);
        assert_eq!(e.mem.u32(this + 0xc), 10);
        assert_eq!(e.call(0x00e0_5af0, &args![this, 3i32]).i32(), 0);
        assert_eq!(e.mem.u32(this + 0xc), 11);
        assert_eq!(e.call(0x00e0_5af0, &args![this, 3i32]).i32(), 0);
        assert_eq!(e.mem.u32(this + 0xc), 11);
        assert!(calls_to(&e, MEM_COPY).is_empty());
    }

    fn install_router(e: &mut Engine) -> (u32, u32) {
        let router = e.mem.alloc(0x24);
        let allocator = e.mem.alloc(8);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        e.register(EASY_ALLOC, |e, a| Ret {
            eax: e.mem.alloc(a[1]),
            ..Ret::default()
        });
        e.register(EASY_FREE, |_, _| Ret::default());
        e.register(MEM_COPY, copy_double);
        (router, allocator)
    }

    #[test]
    fn the_constructor_copies_for_type_zero_and_borrows_otherwise() {
        let mut e = Engine::new();
        let (_, allocator) = install_router(&mut e);
        let source = e.mem.alloc(8);
        e.mem.write(source, &[9, 8, 7, 6, 5, 4, 3, 2]);
        let this = e.mem.alloc(0x20);
        e.mem.write(this, &[0xcd; 0x20]);
        e.call_log = Some(vec![]);
        let got = e.call(0x00e0_5b40, &args![this, source, 8u32, 0u32]).u32();
        assert_eq!(got, this);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this), 0x010e_a540);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x10), 8);
        assert_eq!(e.mem.u32(this + 0x14), 0xffff_ffff);
        assert_eq!(e.mem.u32(this + 0x18), 0);
        // the +4 word is not written
        assert_eq!(e.mem.u16(this + 4), 0xcdcd);
        let buffer = e.mem.u32(this + 8);
        assert_ne!(buffer, source);
        assert_eq!(e.mem.bytes(buffer, 8), vec![9, 8, 7, 6, 5, 4, 3, 2]);
        assert_eq!(calls_to(&e, EASY_ALLOC), vec![vec![allocator, 8]]);
        assert_eq!(calls_to(&e, MEM_COPY), vec![vec![buffer, source, 8]]);
        // any other type uses the caller's buffer
        e.call_log = Some(vec![]);
        e.call(0x00e0_5b40, &args![this, source, 8u32, 2u32]);
        assert_eq!(e.mem.u32(this + 8), source);
        assert_eq!(e.mem.u32(this + 0x18), 2);
        assert!(calls_to(&e, EASY_ALLOC).is_empty());
        assert!(calls_to(&e, MEM_COPY).is_empty());
    }

    #[test]
    fn the_destructor_frees_the_buffer_for_types_zero_and_one_only() {
        let mut e = Engine::new();
        let (_, allocator) = install_router(&mut e);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 8, 0x1234_5678);
        for (memory_type, frees) in [(0u32, true), (1, true), (2, false), (0xffff_ffff, false)] {
            e.mem.set_u32(this + 0x18, memory_type);
            e.call_log = Some(vec![]);
            e.call(0x00e0_5bb0, &args![this]);
            let expected = if frees {
                vec![vec![allocator, 0x1234_5678]]
            } else {
                vec![]
            };
            assert_eq!(calls_to(&e, EASY_FREE), expected, "type {memory_type:#x}");
            assert_eq!(e.mem.u32(this), 0x0102_e388);
        }
    }
}

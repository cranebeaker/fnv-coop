//! `Havok SDK/hkoarchive.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The class is not in the Xbox PDB; its layout is read from the PC code: vtable at `+0`, the
//! reference-counted base's size word at `+4` and count at `+6` (both `u16`), the stream
//! writer it writes through at `+8` and a byte at `+0xC` that selects byte swapping in
//! `writeArrayGeneric`. The writer's vtable slot `0x10` is `write(buffer, length)` and slot
//! `0xC` is the one `isOk` forwards to.

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkString::memCpy(dest, source, size)` (cdecl).
const MEMORY_COPY: u32 = 0x00cb_1260;
/// `hkReferencedObject::removeReference` (locked variant).
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// `hkReferencedObject::addReference` (locked variant).
const ADD_REFERENCE: u32 = 0x00c9_0510;
/// The buffered stream writer's constructor (`this`, stream, buffer size, flag byte).
const BUFFERED_STREAM_WRITER_CONSTRUCTOR: u32 = 0x00d2_74c0;
/// Vtable the class installs while it is alive (`0x010d14f8`) and the one its destructor
/// leaves on the object (`0x0102e388`).
const OARCHIVE_VTABLE: u32 = 0x010d_14f8;
const BASE_VTABLE: u32 = 0x0102_e388;
/// The global holding the file-system object whose slot `0x10` opens a writer.
const FILE_SYSTEM_GLOBAL: u32 = 0x0126_8364;
/// The mask `writeArrayGeneric` applies to the byte count to get the length of its last chunk.
const TAIL_MASK: i32 = 0x8000_01ff_u32 as i32;

// Translated from 00d60590 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkOArchive::~hkOArchive` (Xbox PDB): installs the archive's vtable, releases the writer
/// held at `+8` (`00c905b0`) and leaves the base vtable `0x0102e388` on the object.
pub fn hk_oarchive_destructor(e: &mut Engine, this: Ptr) {
    let writer = e.mem.u32(this.addr() + 8);
    e.mem.set_u32(this.addr(), OARCHIVE_VTABLE);
    e.call(REMOVE_REFERENCE, &args![writer]);
    e.mem.set_u32(this.addr(), BASE_VTABLE);
}

/// Writes the low byte of `value`, kept in a stack word whose address is passed, through the
/// writer at `+8`: slot `0x10` with the address and length 1. Returns the writer's EAX.
fn write_byte(e: &mut Engine, this: Ptr, value: u8) -> u32 {
    let writer = e.mem.u32(this.addr() + 8);
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), u32::from(value));
        e.vcall(writer, 0x10, &args![slot, 1u32]).u32()
    })
}

// Translated from 00d605b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkOArchive::write8` (Xbox PDB): writes the byte through the writer's slot `0x10`. The
/// argument word is a stack slot whose address is passed on; the result is the writer's EAX.
pub fn hk_oarchive_write8(e: &mut Engine, this: Ptr, value: u8) -> u32 {
    write_byte(e, this, value)
}

// Translated from 00d605d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map; the body is identical to `write8` (`00d605b0`): writes one byte
/// through the writer at `+8`.
pub fn fn_00d605d0(e: &mut Engine, this: Ptr, value: u8) -> u32 {
    write_byte(e, this, value)
}

/// Reverses the bytes of each of `count` elements of `size` bytes at `buffer`, for the sizes
/// 2, 4 and 8 the exe swaps (any other size leaves the buffer alone).
fn swap_elements(e: &mut Engine, buffer: u32, size: i32, count: i32) {
    if !matches!(size, 2 | 4 | 8) {
        return;
    }
    let size = size as u32;
    for element in 0..count.max(0) as u32 {
        let base = buffer + element * size;
        for lane in 0..size / 2 {
            let low = e.mem.u8(base + lane);
            let high = e.mem.u8(base + size - 1 - lane);
            e.mem.set_u8(base + lane, high);
            e.mem.set_u8(base + size - 1 - lane, low);
        }
    }
}

// Translated from 00d60600 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkOArchive::writeArrayGeneric` (Xbox PDB): writes `count` elements of `size` bytes.
/// Without the swap byte (`+0xC`) the block goes to the writer's slot `0x10` in one call. With
/// it, the data is copied (`hkString::memCpy`, `00cb1260`) 0x200 bytes at a time into a stack
/// buffer, each element's bytes are reversed when `size` is 2, 4 or 8, and the buffer is
/// written. A `size` of 0 divides by zero in the exe (it faults); here it panics likewise.
pub fn hk_oarchive_write_array_generic(
    e: &mut Engine,
    this: Ptr,
    data: Ptr,
    size: i32,
    count: i32,
) {
    let writer = e.mem.u32(this.addr() + 8);
    if e.mem.u8(this.addr() + 0xc) == 0 {
        e.vcall(writer, 0x10, &args![data, size.wrapping_mul(count)]);
        return;
    }
    let mut chunk_bytes: i32 = 0x200;
    let mut chunk_count: i32 = 0x200 / size;
    let mut remaining = size.wrapping_mul(count);
    let mut tail_bytes = remaining & TAIL_MASK;
    if tail_bytes < 0 {
        tail_bytes = ((tail_bytes - 1) | 0xffff_fe00_u32 as i32).wrapping_add(1);
    }
    let tail_count = tail_bytes / size;
    let mut source = data.addr();
    while remaining > 0 {
        if remaining < 0x200 {
            chunk_count = tail_count;
            chunk_bytes = tail_bytes;
        }
        e.with_stack(0x200, |e, buffer| {
            e.call(MEMORY_COPY, &args![buffer, source, chunk_bytes]);
            swap_elements(e, buffer.addr(), size, chunk_count);
            e.vcall(writer, 0x10, &args![buffer, chunk_bytes]);
        });
        source = source.wrapping_add(chunk_bytes as u32);
        remaining = remaining.wrapping_sub(chunk_bytes);
    }
}

// Translated from 00d607b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkOArchive::writeRaw` (Xbox PDB): a tail jump to the writer's slot `0x10` with the same
/// `(buffer, length)` words; returns the writer's EAX.
pub fn hk_oarchive_write_raw(e: &mut Engine, this: Ptr, buffer: Ptr, length: u32) -> u32 {
    let writer = e.mem.u32(this.addr() + 8);
    e.vcall(writer, 0x10, &args![buffer, length]).u32()
}

// Translated from 00d607e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkOArchive::isOk` (Xbox PDB, the name is the map's): calls the writer's slot `0xC` with the
/// one argument and returns that argument.
pub fn hk_oarchive_is_ok(e: &mut Engine, this: Ptr, value: u32) -> u32 {
    let writer = e.mem.u32(this.addr() + 8);
    e.vcall(writer, 0xc, &args![value]);
    value
}

// Translated from 00d60800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkOArchive::getStreamWriter` (Xbox PDB): the writer held at `+8`.
pub fn hk_oarchive_get_stream_writer(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 8)
}

// Translated from 00d60830 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkOArchive::hkOArchive` (Xbox PDB): reference count 1 (`+6`), the archive's vtable, the
/// writer at `+8`, the swap byte at `+0xC`; then `addReference` (`00c90510`) on the writer.
/// Returns `this`.
pub fn hk_oarchive_constructor(e: &mut Engine, this: Ptr, writer: Ptr, swap: u8) -> Ptr {
    e.mem.set_u16(this.addr() + 6, 1);
    e.mem.set_u32(this.addr(), OARCHIVE_VTABLE);
    e.mem.set_u32(this.addr() + 8, writer.addr());
    e.mem.set_u8(this.addr() + 0xc, swap);
    e.call(ADD_REFERENCE, &args![writer]);
    this
}

// Translated from 00d60860 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: the constructor that opens its writer through the file-system
/// object held in the global `0x01268364` (its vtable slot `0x10`, one argument). Returns `this`.
pub fn fn_00d60860(e: &mut Engine, this: Ptr, name: u32, swap: u8) -> Ptr {
    e.mem.set_u16(this.addr() + 6, 1);
    e.mem.set_u32(this.addr(), OARCHIVE_VTABLE);
    e.mem.set_u8(this.addr() + 0xc, swap);
    let file_system = e.mem.u32(FILE_SYSTEM_GLOBAL);
    let writer = e.vcall(file_system, 0x10, &args![name]).u32();
    e.mem.set_u32(this.addr() + 8, writer);
    this
}

// Translated from 00d608a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: the constructor that makes its own writer, a 0x1c-byte block from
/// the memory router's allocator (slot 4) with its size word set, built by the buffered stream
/// writer constructor (`00d274c0`) with the two arguments and flag 0. Returns `this`.
pub fn fn_00d608a0(e: &mut Engine, this: Ptr, first: u32, second: u32, swap: u8) -> Ptr {
    e.mem.set_u16(this.addr() + 6, 1);
    e.mem.set_u32(this.addr(), OARCHIVE_VTABLE);
    e.mem.set_u8(this.addr() + 0xc, swap);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![0x1cu32]).u32();
    e.mem.set_u16(block + 4, 0x1c);
    let writer = e
        .call(
            BUFFERED_STREAM_WRITER_CONSTRUCTOR,
            &args![block, first, second, 0u32],
        )
        .u32();
    e.mem.set_u32(this.addr() + 8, writer);
    this
}

/// Writes the stack word(s) `value` through `write_array_generic` as the fixed-size writers
/// do: the address of the argument, element size `size`, count 1.
fn write_one(e: &mut Engine, this: Ptr, value: u64, size: i32) {
    e.with_stack(8, |e, slot| {
        e.mem.set_u64(slot.addr(), value);
        hk_oarchive_write_array_generic(e, this, slot, size, 1);
    });
}

// Translated from 00d60900 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkOArchive::write16` (Xbox PDB): `writeArrayGeneric(&value, 2, 1)`.
pub fn hk_oarchive_write16(e: &mut Engine, this: Ptr, value: u32) {
    write_one(e, this, u64::from(value), 2);
}

// Translated from 00d60920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkOArchive::write16u` (Xbox PDB): `writeArrayGeneric(&value, 2, 1)`.
pub fn hk_oarchive_write16u(e: &mut Engine, this: Ptr, value: u32) {
    write_one(e, this, u64::from(value), 2);
}

// Translated from 00d60940 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkOArchive::write32` (Xbox PDB): `writeArrayGeneric(&value, 4, 1)`.
pub fn hk_oarchive_write32(e: &mut Engine, this: Ptr, value: u32) {
    write_one(e, this, u64::from(value), 4);
}

// Translated from 00d60960 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkOArchive::write32u` (Xbox PDB): `writeArrayGeneric(&value, 4, 1)`.
pub fn hk_oarchive_write32u(e: &mut Engine, this: Ptr, value: u32) {
    write_one(e, this, u64::from(value), 4);
}

// Translated from 00d609a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkOArchive::write64u` (Xbox PDB): `writeArrayGeneric(&value, 8, 1)` on the two argument
/// words.
pub fn hk_oarchive_write64u(e: &mut Engine, this: Ptr, value: u64) {
    write_one(e, this, value, 8);
}

// Translated from 00d609c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkOArchive::writeFloat32` (Xbox PDB): `writeArrayGeneric(&value, 4, 1)` on the float's
/// bits, which are written as they are.
pub fn hk_oarchive_write_float32(e: &mut Engine, this: Ptr, value: u32) {
    write_one(e, this, u64::from(value), 4);
}

// Translated from 00d60aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkOArchive::writeArray32u` (Xbox PDB): `writeArrayGeneric(data, 4, count)`.
pub fn hk_oarchive_write_array32u(e: &mut Engine, this: Ptr, data: Ptr, count: i32) {
    hk_oarchive_write_array_generic(e, this, data, 4, count);
}

// Translated from 00d60b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkOArchive::writeArrayFloat32` (Xbox PDB): `writeArrayGeneric(data, 4, count)`.
pub fn hk_oarchive_write_array_float32(e: &mut Engine, this: Ptr, data: Ptr, count: i32) {
    hk_oarchive_write_array_generic(e, this, data, 4, count);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d6_0590, hk_oarchive_destructor(Ptr)),
        entry!(0x00d6_05b0, hk_oarchive_write8(Ptr, u8) -> u32),
        entry!(0x00d6_05d0, fn_00d605d0(Ptr, u8) -> u32),
        entry!(
            0x00d6_0600,
            hk_oarchive_write_array_generic(Ptr, Ptr, i32, i32)
        ),
        entry!(0x00d6_07b0, hk_oarchive_write_raw(Ptr, Ptr, u32) -> u32),
        entry!(0x00d6_07e0, hk_oarchive_is_ok(Ptr, u32) -> u32),
        entry!(0x00d6_0800, hk_oarchive_get_stream_writer(Ptr) -> u32),
        entry!(0x00d6_0830, hk_oarchive_constructor(Ptr, Ptr, u8) -> Ptr),
        entry!(0x00d6_0860, fn_00d60860(Ptr, u32, u8) -> Ptr),
        entry!(0x00d6_08a0, fn_00d608a0(Ptr, u32, u32, u8) -> Ptr),
        entry!(0x00d6_0900, hk_oarchive_write16(Ptr, u32)),
        entry!(0x00d6_0920, hk_oarchive_write16u(Ptr, u32)),
        entry!(0x00d6_0940, hk_oarchive_write32(Ptr, u32)),
        entry!(0x00d6_0960, hk_oarchive_write32u(Ptr, u32)),
        entry!(0x00d6_09a0, hk_oarchive_write64u(Ptr, u64)),
        entry!(0x00d6_09c0, hk_oarchive_write_float32(Ptr, u32)),
        entry!(0x00d6_0aa0, hk_oarchive_write_array32u(Ptr, Ptr, i32)),
        entry!(0x00d6_0b00, hk_oarchive_write_array_float32(Ptr, Ptr, i32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const WRITE_DOUBLE: u32 = 0x0900_0010;
    const OK_DOUBLE: u32 = 0x0900_0020;
    const ALLOCATE_DOUBLE: u32 = 0x0900_0030;
    const OPEN_DOUBLE: u32 = 0x0900_0040;

    /// What the writer double saw: (buffer address, length, the bytes read there).
    type Writes = Rc<RefCell<Vec<(u32, u32, Vec<u8>)>>>;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0126_8000, 0x1000);
        e.register(MEMORY_COPY, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        e
    }

    /// A writer whose slot `0x10` records the bytes it is given and returns 7, and whose
    /// slot `0xC` returns 9.
    fn writer(e: &mut Engine) -> (u32, Writes) {
        let writes: Writes = Rc::new(RefCell::new(Vec::new()));
        let seen = Rc::clone(&writes);
        e.register_double(WRITE_DOUBLE, move |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            seen.borrow_mut().push((a[1], a[2], bytes));
            Ret {
                eax: 7,
                ..Ret::default()
            }
        });
        e.register(OK_DOUBLE, |_, _| Ret {
            eax: 9,
            ..Ret::default()
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0xc, OK_DOUBLE);
        e.mem.set_u32(vtable + 0x10, WRITE_DOUBLE);
        let object = e.mem.alloc(0x20);
        e.mem.set_u32(object, vtable);
        (object, writes)
    }

    fn archive(e: &mut Engine, writer: u32, swap: u8) -> u32 {
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 8, writer);
        e.mem.set_u8(this + 0xc, swap);
        this
    }

    fn data(e: &mut Engine, bytes: &[u8]) -> u32 {
        let block = e.mem.alloc(bytes.len().max(1) as u32);
        e.mem.write(block, bytes);
        block
    }

    #[test]
    fn destructor_releases_the_writer_and_leaves_the_base_vtable() {
        let mut e = engine();
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 8, 0x1234_5678);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(REMOVE_REFERENCE, move |e, a| {
            log.borrow_mut().push((a[0], e.mem.u32(this)));
            Ret::default()
        });
        e.call(0x00d6_0590, &args![this]);
        assert_eq!(*seen.borrow(), vec![(0x1234_5678, OARCHIVE_VTABLE)]);
        assert_eq!(e.mem.u32(this), BASE_VTABLE);
    }

    fn check_byte(address: u32) {
        let mut e = engine();
        let (w, writes) = writer(&mut e);
        let this = archive(&mut e, w, 0);
        let got = e.call(address, &args![this, 0x1234_56abu32]).u32();
        assert_eq!(got, 7);
        let writes = writes.borrow();
        assert_eq!(writes.len(), 1);
        assert_eq!((writes[0].1, writes[0].2.clone()), (1, vec![0xab]));
    }

    #[test]
    fn write8_writes_the_low_byte() {
        check_byte(0x00d6_05b0);
    }

    #[test]
    fn fn_00d605d0_writes_the_low_byte() {
        check_byte(0x00d6_05d0);
    }

    #[test]
    fn write_array_generic_without_swap_writes_the_block_in_one_call() {
        let mut e = engine();
        let (w, writes) = writer(&mut e);
        let this = archive(&mut e, w, 0);
        let block = data(&mut e, &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
        e.call(0x00d6_0600, &args![this, block, 4u32, 3u32]);
        assert_eq!(
            *writes.borrow(),
            vec![(block, 12, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12])]
        );
    }

    #[test]
    fn write_array_generic_with_swap_reverses_elements_of_two_four_and_eight_bytes() {
        let bytes: Vec<u8> = (1..=24).collect();
        let cases: [(u32, u32, Vec<u8>); 4] = [
            (2, 6, vec![2, 1, 4, 3, 6, 5, 8, 7, 10, 9, 12, 11]),
            (4, 3, vec![4, 3, 2, 1, 8, 7, 6, 5, 12, 11, 10, 9]),
            (
                8,
                2,
                vec![8, 7, 6, 5, 4, 3, 2, 1, 16, 15, 14, 13, 12, 11, 10, 9],
            ),
            (3, 3, (1..=9).collect()),
        ];
        for (size, count, expected) in cases {
            let mut e = engine();
            let (w, writes) = writer(&mut e);
            let this = archive(&mut e, w, 1);
            let block = data(&mut e, &bytes);
            e.call(0x00d6_0600, &args![this, block, size, count]);
            let writes = writes.borrow();
            assert_eq!(writes.len(), 1, "size {size}");
            assert_eq!(writes[0].1 as usize, expected.len());
            assert_eq!(writes[0].2, expected, "size {size}");
        }
    }

    #[test]
    fn write_array_generic_writes_long_arrays_in_chunks_of_0x200_bytes() {
        let mut e = engine();
        let (w, writes) = writer(&mut e);
        let this = archive(&mut e, w, 1);
        // 200 four-byte elements: 800 bytes, written as 512 and then 288.
        let bytes: Vec<u8> = (0..800u32).map(|i| (i % 251) as u8).collect();
        let block = data(&mut e, &bytes);
        e.call(0x00d6_0600, &args![this, block, 4u32, 200u32]);
        let writes = writes.borrow();
        assert_eq!(
            writes.iter().map(|w| w.1).collect::<Vec<_>>(),
            vec![0x200, 288]
        );
        let swapped: Vec<u8> = bytes
            .chunks(4)
            .flat_map(|c| c.iter().rev().copied().collect::<Vec<_>>())
            .collect();
        assert_eq!(writes[0].2, swapped[..0x200].to_vec());
        assert_eq!(writes[1].2, swapped[0x200..].to_vec());
    }

    #[test]
    fn write_array_generic_with_swap_and_nothing_to_write_writes_nothing() {
        let mut e = engine();
        let (w, writes) = writer(&mut e);
        let this = archive(&mut e, w, 1);
        let block = data(&mut e, &[1, 2, 3, 4]);
        e.call(0x00d6_0600, &args![this, block, 4u32, 0u32]);
        assert!(writes.borrow().is_empty());
    }

    #[test]
    fn write_raw_forwards_the_buffer_and_length() {
        let mut e = engine();
        let (w, writes) = writer(&mut e);
        let this = archive(&mut e, w, 0);
        let block = data(&mut e, &[9, 8, 7]);
        let got = e.call(0x00d6_07b0, &args![this, block, 3u32]).u32();
        assert_eq!(got, 7);
        assert_eq!(*writes.borrow(), vec![(block, 3, vec![9, 8, 7])]);
    }

    #[test]
    fn is_ok_forwards_to_slot_0xc_and_returns_its_argument() {
        let mut e = engine();
        let (w, _) = writer(&mut e);
        let this = archive(&mut e, w, 0);
        e.call_log = Some(vec![]);
        let got = e.call(0x00d6_07e0, &args![this, 0x55u32]).u32();
        assert_eq!(got, 0x55);
        let log = e.call_log.clone().unwrap();
        assert!(log.contains(&(OK_DOUBLE, vec![w, 0x55])));
    }

    #[test]
    fn get_stream_writer_returns_the_writer() {
        let mut e = engine();
        let this = archive(&mut e, 0xabcd_0123, 0);
        assert_eq!(e.call(0x00d6_0800, &args![this]).u32(), 0xabcd_0123);
    }

    #[test]
    fn constructor_sets_the_fields_and_adds_a_reference_to_the_writer() {
        let mut e = engine();
        let this = e.mem.alloc(0x20);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(ADD_REFERENCE, move |_, a| {
            log.borrow_mut().push(a[0]);
            Ret::default()
        });
        let got = e
            .call(0x00d6_0830, &args![this, 0x4444u32, 0x1_0001u32])
            .u32();
        assert_eq!(got, this);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this), OARCHIVE_VTABLE);
        assert_eq!(e.mem.u32(this + 8), 0x4444);
        assert_eq!(e.mem.u8(this + 0xc), 1);
        assert_eq!(*seen.borrow(), vec![0x4444]);
    }

    #[test]
    fn fn_00d60860_opens_its_writer_through_the_file_system_object() {
        let mut e = engine();
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0x10, OPEN_DOUBLE);
        let file_system = e.mem.alloc(0x10);
        e.mem.set_u32(file_system, vtable);
        e.mem.set_u32(FILE_SYSTEM_GLOBAL, file_system);
        e.register_double(OPEN_DOUBLE, move |_, a| {
            assert_eq!(a, [file_system, 0x7700]);
            Ret {
                eax: 0x5555,
                ..Ret::default()
            }
        });
        let this = e.mem.alloc(0x20);
        let got = e.call(0x00d6_0860, &args![this, 0x7700u32, 1u32]).u32();
        assert_eq!(got, this);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this), OARCHIVE_VTABLE);
        assert_eq!(e.mem.u32(this + 8), 0x5555);
        assert_eq!(e.mem.u8(this + 0xc), 1);
    }

    #[test]
    fn fn_00d608a0_builds_its_own_buffered_writer() {
        let mut e = engine();
        let block = e.mem.alloc(0x40);
        e.register_double(ALLOCATE_DOUBLE, move |_, a| {
            assert_eq!(a[1], 0x1c);
            Ret {
                eax: block,
                ..Ret::default()
            }
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 4, ALLOCATE_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let built = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&built);
        e.register_double(BUFFERED_STREAM_WRITER_CONSTRUCTOR, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        let this = e.mem.alloc(0x20);
        let got = e
            .call(0x00d6_08a0, &args![this, 0xaaaau32, 0xbbbbu32, 1u32])
            .u32();
        assert_eq!(got, this);
        assert_eq!(*built.borrow(), vec![vec![block, 0xaaaa, 0xbbbb, 0]]);
        assert_eq!(e.mem.u16(block + 4), 0x1c);
        assert_eq!(e.mem.u32(this + 8), block);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this), OARCHIVE_VTABLE);
        assert_eq!(e.mem.u8(this + 0xc), 1);
    }

    /// A swapping archive writing `args` through `address` writes `expected` once.
    fn check_swapped(address: u32, extra: Vec<u32>, expected: &[u8]) {
        let mut e = engine();
        let (w, writes) = writer(&mut e);
        let this = archive(&mut e, w, 1);
        let mut words = vec![this];
        words.extend(extra);
        e.call(address, &words);
        let writes = writes.borrow();
        assert_eq!(writes.len(), 1);
        assert_eq!(writes[0].2, expected);
    }

    #[test]
    fn write16_swaps_two_bytes() {
        check_swapped(0x00d6_0900, vec![0x1122], &[0x11, 0x22]);
    }

    #[test]
    fn write16u_swaps_two_bytes() {
        check_swapped(0x00d6_0920, vec![0x3344], &[0x33, 0x44]);
    }

    #[test]
    fn write32_swaps_four_bytes() {
        check_swapped(0x00d6_0940, vec![0x1122_3344], &[0x11, 0x22, 0x33, 0x44]);
    }

    #[test]
    fn write32u_swaps_four_bytes() {
        check_swapped(0x00d6_0960, vec![0x5566_7788], &[0x55, 0x66, 0x77, 0x88]);
    }

    #[test]
    fn write64u_swaps_eight_bytes() {
        check_swapped(
            0x00d6_09a0,
            vec![0x5566_7788, 0x1122_3344],
            &[0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88],
        );
    }

    #[test]
    fn write_float32_swaps_the_float_bits() {
        check_swapped(
            0x00d6_09c0,
            vec![1.0f32.to_bits()],
            &[0x3f, 0x80, 0x00, 0x00],
        );
    }

    #[test]
    fn write_array32u_and_float32_write_swapped_four_byte_elements() {
        for address in [0x00d6_0aa0, 0x00d6_0b00] {
            let mut e = engine();
            let (w, writes) = writer(&mut e);
            let this = archive(&mut e, w, 1);
            let block = data(&mut e, &[1, 2, 3, 4, 5, 6, 7, 8]);
            e.call(address, &args![this, block, 2u32]);
            let writes = writes.borrow();
            assert_eq!(writes.len(), 1);
            assert_eq!(writes[0].2, vec![4, 3, 2, 1, 8, 7, 6, 5]);
        }
    }
}

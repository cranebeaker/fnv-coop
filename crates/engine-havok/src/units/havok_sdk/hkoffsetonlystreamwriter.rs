//! `Havok SDK/hkoffsetonlystreamwriter.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its
//! functions in FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkOffsetOnlyStreamWriter` writes nothing: it keeps a position at `+8` and the largest
//! position reached at `+0xC` (both signed words). Layout from the PC code (the class is not in
//! the Xbox PDB): vtable at `+0`, reference-counted base's count at `+6` (`u16`).

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable the constructor installs.
const OFFSET_ONLY_WRITER_VTABLE: u32 = 0x010e_a388;

/// Raises the largest position (`+0xC`) to the current one (`+8`) when that is greater
/// (signed). The exe stores the larger of the two back either way.
fn raise_end(e: &mut Engine, this: Ptr) {
    let position = e.mem.u32(this.addr() + 8) as i32;
    let end = e.mem.u32(this.addr() + 0xc) as i32;
    let larger = if position > end { position } else { end };
    e.mem.set_u32(this.addr() + 0xc, larger as u32);
}

// Translated from 00e02e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkOffsetOnlyStreamWriter::write` (Xbox PDB), `(buffer, length)`: advances the position
/// by `length` and raises the largest position when needed. The buffer is not read; the result
/// is `length` (`EAX`).
pub fn hk_offset_only_stream_writer_write(
    e: &mut Engine,
    this: Ptr,
    _unused_1: Ptr,
    length: u32,
) -> u32 {
    let position = e.mem.u32(this.addr() + 8).wrapping_add(length);
    e.mem.set_u32(this.addr() + 8, position);
    raise_end(e, this);
    length
}

// Translated from 00e02e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkOffsetOnlyStreamWriter::seek` (Xbox PDB), `(offset, mode)`: mode 0 sets the position to
/// `offset`, mode 1 adds `offset` to it, mode 2 sets it to the largest position minus
/// `offset`, any other mode leaves it; then the largest position is raised as in `write`.
/// Returns 0.
pub fn hk_offset_only_stream_writer_seek(e: &mut Engine, this: Ptr, offset: u32, mode: u32) -> u32 {
    match mode {
        0 => e.mem.set_u32(this.addr() + 8, offset),
        1 => {
            let position = e.mem.u32(this.addr() + 8);
            e.mem
                .set_u32(this.addr() + 8, position.wrapping_add(offset));
        }
        2 => {
            let end = e.mem.u32(this.addr() + 0xc);
            e.mem.set_u32(this.addr() + 8, end.wrapping_sub(offset));
        }
        _ => {}
    }
    raise_end(e, this);
    0
}

// Translated from 00e02ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkOffsetOnlyStreamWriter::tell` (Xbox PDB): the position (`+8`).
pub fn hk_offset_only_stream_writer_tell(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 8)
}

// Translated from 00e02ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkOffsetOnlyStreamWriter::hkOffsetOnlyStreamWriter` (Xbox PDB): reference count 1 (`+6`),
/// vtable `0x010ea388`, position and largest position 0. Returns `this`.
pub fn hk_offset_only_stream_writer_constructor(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u16(this.addr() + 6, 1);
    e.mem.set_u32(this.addr(), OFFSET_ONLY_WRITER_VTABLE);
    e.mem.set_u32(this.addr() + 8, 0);
    e.mem.set_u32(this.addr() + 0xc, 0);
    this
}

// Translated from 00e02f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkOffsetOnlyStreamWriter::isOk` (Xbox PDB): the result byte goes through a pointer
/// argument (a hidden return slot): stores 1 there and returns the pointer. `this` is not
/// read.
pub fn hk_offset_only_stream_writer_is_ok(e: &mut Engine, _unused_0: Ptr, out: Ptr) -> Ptr {
    e.mem.set_u8(out.addr(), 1);
    out
}

// Translated from 00e02f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkOffsetOnlyStreamWriter::seekTellSupported` (Xbox PDB): stores 1 through the pointer
/// argument (a hidden return slot) and returns the pointer. `this` is not read.
pub fn hk_offset_only_stream_writer_seek_tell_supported(
    e: &mut Engine,
    _unused_0: Ptr,
    out: Ptr,
) -> Ptr {
    e.mem.set_u8(out.addr(), 1);
    out
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e0_2e70, hk_offset_only_stream_writer_write(Ptr, Ptr, u32) -> u32),
        entry!(0x00e0_2e90, hk_offset_only_stream_writer_seek(Ptr, u32, u32) -> u32),
        entry!(0x00e0_2ee0, hk_offset_only_stream_writer_tell(Ptr) -> u32),
        entry!(0x00e0_2ef0, hk_offset_only_stream_writer_constructor(Ptr) -> Ptr),
        entry!(0x00e0_2f10, hk_offset_only_stream_writer_is_ok(Ptr, Ptr) -> Ptr),
        entry!(0x00e0_2f20, hk_offset_only_stream_writer_seek_tell_supported(Ptr, Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn writer(e: &mut Engine, position: i32, end: i32) -> u32 {
        let this = e.mem.alloc(0x10);
        e.mem.set_i32(this + 8, position);
        e.mem.set_i32(this + 0xc, end);
        this
    }

    #[test]
    fn write_advances_and_raises_the_end() {
        let mut e = Engine::new();
        let this = writer(&mut e, 10, 12);
        let got = e.call(0x00e0_2e70, &args![this, 0x5555u32, 5u32]).u32();
        assert_eq!(got, 5);
        assert_eq!((e.mem.i32(this + 8), e.mem.i32(this + 0xc)), (15, 15));
        // A write that stays below the end leaves the end.
        let this = writer(&mut e, 1, 12);
        e.call(0x00e0_2e70, &args![this, 0u32, 2u32]);
        assert_eq!((e.mem.i32(this + 8), e.mem.i32(this + 0xc)), (3, 12));
    }

    #[test]
    fn seek_modes() {
        let mut e = Engine::new();
        // Mode 0: absolute; the end follows when passed.
        let this = writer(&mut e, 3, 5);
        assert_eq!(e.call(0x00e0_2e90, &args![this, 9u32, 0u32]).u32(), 0);
        assert_eq!((e.mem.i32(this + 8), e.mem.i32(this + 0xc)), (9, 9));
        // Mode 1: relative to the position (a negative offset moves back).
        let this = writer(&mut e, 8, 20);
        e.call(0x00e0_2e90, &args![this, -3i32, 1u32]);
        assert_eq!((e.mem.i32(this + 8), e.mem.i32(this + 0xc)), (5, 20));
        // Mode 2: from the end.
        let this = writer(&mut e, 8, 20);
        e.call(0x00e0_2e90, &args![this, 4u32, 2u32]);
        assert_eq!((e.mem.i32(this + 8), e.mem.i32(this + 0xc)), (16, 20));
        // Any other mode leaves the position.
        let this = writer(&mut e, 8, 20);
        e.call(0x00e0_2e90, &args![this, 4u32, 7u32]);
        assert_eq!((e.mem.i32(this + 8), e.mem.i32(this + 0xc)), (8, 20));
    }

    #[test]
    fn tell_is_the_position() {
        let mut e = Engine::new();
        let this = writer(&mut e, 42, 50);
        assert_eq!(e.call(0x00e0_2ee0, &args![this]).u32(), 42);
    }

    #[test]
    fn constructor_resets_the_positions() {
        let mut e = Engine::new();
        let this = writer(&mut e, 42, 50);
        let got = e.call(0x00e0_2ef0, &args![this]).u32();
        assert_eq!(got, this);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this), OFFSET_ONLY_WRITER_VTABLE);
        assert_eq!((e.mem.u32(this + 8), e.mem.u32(this + 0xc)), (0, 0));
    }

    #[test]
    fn is_ok_and_seek_tell_supported_store_one() {
        for address in [0x00e0_2f10, 0x00e0_2f20] {
            let mut e = Engine::new();
            let out = e.mem.alloc(4);
            let got = e.call(address, &args![0x1234u32, out]).u32();
            assert_eq!(got, out);
            assert_eq!(e.mem.u8(out), 1);
        }
    }
}

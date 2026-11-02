//! `Havok SDK/hkstreamwriter.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The defaults of `hkStreamWriter`'s virtual functions for a stream that cannot seek.

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00cb15d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStreamWriter::seek` (Xbox PDB), virtual, `RET 8` (two words, not read; `ECX` is the writer,
/// not read): answers 1 (the seek failed).
pub fn hk_stream_writer_seek(_e: &mut Engine, _this: Ptr, _unused_1: u32, _unused_2: u32) -> u32 {
    1
}

// Translated from 00cb15e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStreamWriter::tell` (Xbox PDB), virtual, no stack words (`ECX` not read): answers -1.
pub fn hk_stream_writer_tell(_e: &mut Engine, _this: Ptr) -> i32 {
    -1
}

// Translated from 00cb15f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStreamWriter::seekTellSupported` (Xbox PDB), virtual, `RET 4` (`ECX` not read): writes 0 at
/// the byte `out` and answers `out`.
pub fn hk_stream_writer_seek_tell_supported(e: &mut Engine, _this: Ptr, out: u32) -> u32 {
    e.mem.set_u8(out, 0);
    out
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cb15d0, hk_stream_writer_seek(Ptr, u32, u32) -> u32),
        entry!(0x00cb15e0, hk_stream_writer_tell(Ptr) -> i32),
        entry!(0x00cb15f0, hk_stream_writer_seek_tell_supported(Ptr, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_writer_that_cannot_seek_answers_the_defaults() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00cb_15d0, &args![0x10u32, 5u32, 0u32]).u32(), 1);
        assert_eq!(e.call(0x00cb_15e0, &args![0x10u32]).i32(), -1);
        let out = e.mem.alloc(4);
        e.mem.set_u8(out, 9);
        assert_eq!(e.call(0x00cb_15f0, &args![0x10u32, out]).u32(), out);
        assert_eq!(e.mem.u8(out), 0);
    }
}

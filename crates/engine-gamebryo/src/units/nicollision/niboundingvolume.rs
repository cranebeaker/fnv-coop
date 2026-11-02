//! `NiCollision/niboundingvolume.cpp` (Xbox PDB source unit), subsystem `NiCollision`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::nicollision::nicollisiondata::stream_transfer;

/// `NiStream::m_pkIstr` / `m_pkOstr` (Xbox PDB) +0x24C / +0x250, and the
/// binary stream's read (+8) and write (+0xc) functions.
const STREAM_INPUT: u32 = 0x24c;
const STREAM_OUTPUT: u32 = 0x250;
const BINARY_STREAM_READ: u32 = 0x8;
const BINARY_STREAM_WRITE: u32 = 0xc;
/// The six loader functions by volume type (`0126fa7c`, `.data`; zero in
/// the image, filled at start-up by the code at `00e99045` and on).
const LOADERS: u32 = 0x0126_fa7c;

// Translated from 00e9cab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoundingVolume::CreateFromStream` (Xbox PDB; cdecl, the stream):
/// reads the volume type (4 bytes, through the input stream's read
/// function) and, for a type from 0 to 5 (signed), answers the loader of
/// that type from the table at `0126fa7c` run on the stream; 0 otherwise.
pub fn ni_bounding_volume_create_from_stream(e: &mut Engine, stream: Ptr) -> u32 {
    let kind = e.with_stack(4, |e, local| {
        let input = e.mem.u32(stream.addr().wrapping_add(STREAM_INPUT));
        stream_transfer(e, input, BINARY_STREAM_READ, local.addr(), 4);
        e.mem.i32(local.addr())
    });
    if (0..6).contains(&kind) {
        let loader = e.global::<u32>(LOADERS + 4 * kind as u32);
        e.call(loader, &args![stream]).u32()
    } else {
        0
    }
}

// Translated from 00e9cb10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoundingVolume::SaveBinary` (Xbox PDB; `thiscall`; the stream;
/// `RET 4`): writes the volume's `Type` (virtual slot +0xc) as 4 bytes
/// through the output stream's write function.
pub fn ni_bounding_volume_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    let kind = e.vcall(this.addr(), 0xc, &args![]).u32();
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), kind);
        let output = e.mem.u32(stream.addr().wrapping_add(STREAM_OUTPUT));
        stream_transfer(e, output, BINARY_STREAM_WRITE, local.addr(), 4);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e9_cab0, ni_bounding_volume_create_from_stream(Ptr) -> u32),
        entry!(0x00e9_cb10, ni_bounding_volume_save_binary(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A stream whose input and output (+0x24c, +0x250) are one binary
    /// stream with read (+8) and write (+0xc) doubles: a read fills the
    /// buffer from `0x7000`, a write copies it there.
    fn stream(e: &mut Engine) -> u32 {
        e.map(0x7000, 0x100);
        e.register(0x0f00_0008, |e, a| {
            let bytes = e.mem.bytes(0x7000, a[2]);
            e.mem.write(a[1], &bytes);
            assert_eq!(e.mem.u32(a[3]), a[2]);
            a[2].into_ret()
        });
        e.register(0x0f00_000c, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(0x7000, &bytes);
            a[2].into_ret()
        });
        let binary = e.mem.alloc(0x10);
        e.mem.set_u32(binary + 4, 0x10);
        e.mem.set_u32(binary + 8, 0x0f00_0008);
        e.mem.set_u32(binary + 0xc, 0x0f00_000c);
        let stream = e.mem.alloc(0x254);
        e.mem.set_u32(stream + STREAM_INPUT, binary);
        e.mem.set_u32(stream + STREAM_OUTPUT, binary);
        stream
    }

    #[test]
    fn create_from_stream_dispatches_on_the_type() {
        let mut e = Engine::new();
        let stream = stream(&mut e);
        e.map(LOADERS & !0xfff, 0x1000);
        e.set_global(LOADERS + 4 * 5, 0x0f00_0100u32);
        e.register(0x0f00_0100, |_, a| (a[0] + 1).into_ret());
        e.mem.set_u32(0x7000, 5);
        assert_eq!(e.call(0x00e9_cab0, &args![stream]).u32(), stream + 1);
        // The read advanced the binary stream's position.
        let binary = e.mem.u32(stream + STREAM_INPUT);
        assert_eq!(e.mem.u32(binary + 4), 0x14);
        // Types 6 and -1 have no loader.
        for kind in [6u32, 0xffff_ffff] {
            e.mem.set_u32(0x7000, kind);
            assert_eq!(e.call(0x00e9_cab0, &args![stream]).u32(), 0);
        }
    }

    #[test]
    fn save_binary_writes_the_type() {
        let mut e = Engine::new();
        let stream = stream(&mut e);
        e.register(0x0f00_0200, |_, _| 3u32.into_ret());
        e.put_vtable(0x0f00_1000, &[0, 0, 0, 0x0f00_0200]);
        let volume = e.mem.alloc(4);
        e.mem.set_u32(volume, 0x0f00_1000);
        e.call(0x00e9_cb10, &args![volume, stream]);
        assert_eq!(e.mem.u32(0x7000), 3);
        let binary = e.mem.u32(stream + STREAM_OUTPUT);
        assert_eq!(e.mem.u32(binary + 4), 0x14);
    }
}

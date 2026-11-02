//! `NiMain/nipixeldata_xenon.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiPixelData` fields (Xbox PDB names, the PC offsets; the layout is in
/// `nipixeldata.rs`): `m_pucPixels` +0x50, `m_puiWidth` +0x54, `m_puiHeight` +0x58,
/// `m_puiOffsetInBytes` +0x5c.
const PIXEL_DATA_PIXELS: u32 = 0x50;
const PIXEL_DATA_WIDTHS: u32 = 0x54;
const PIXEL_DATA_HEIGHTS: u32 = 0x58;
const PIXEL_DATA_OFFSETS: u32 = 0x5c;
/// `00aa1070`: the cdecl allocation (one word).
const ALLOCATE: u32 = 0x00aa_1070;

// Translated from 00a8f010 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPixelData::AllocateData` (Xbox PDB; `__thiscall`, three words: levels, faces, bytes per
/// face): one allocation (`00aa1070`) holds the pixels (`faces` times the face size rounded up
/// to 4 bytes), then the width, height and offset tables (one word per level, the offsets one
/// more); the four pointers are stored.
pub fn ni_pixel_data_allocate_data(
    e: &mut Engine,
    this: Ptr,
    levels: u32,
    faces: u32,
    face_bytes: u32,
) {
    let pixels = faces.wrapping_mul(face_bytes.wrapping_add(3) & !3);
    let size = pixels.wrapping_add(levels.wrapping_mul(12)).wrapping_add(4);
    let buffer = e.call(ALLOCATE, &args![size]).u32();
    let base = this.addr();
    e.mem.set_u32(base + PIXEL_DATA_PIXELS, buffer);
    let widths = buffer.wrapping_add(pixels);
    e.mem.set_u32(base + PIXEL_DATA_WIDTHS, widths);
    let heights = widths.wrapping_add(levels.wrapping_mul(4));
    let offsets = heights.wrapping_add(levels.wrapping_mul(4));
    e.mem.set_u32(base + PIXEL_DATA_HEIGHTS, heights);
    e.mem.set_u32(base + PIXEL_DATA_OFFSETS, offsets);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00a8f010,
        ni_pixel_data_allocate_data(Ptr, u32, u32, u32)
    )]
}

#[cfg(test)]
mod b007_tests {
    use super::*;

    #[test]
    fn allocate_data_carves_one_block() {
        let mut e = Engine::new();
        e.register(ALLOCATE, |_, a| {
            // 6 faces of 10 bytes rounded to 12, then 3 levels of three tables and a word.
            assert_eq!(a[0], 72 + 36 + 4);
            Ret {
                eax: 0x4000,
                ..Ret::default()
            }
        });
        let data = e.mem.alloc(0x74);
        e.call(0x00a8f010, &args![data, 3u32, 6u32, 10u32]);
        assert_eq!(e.mem.u32(data + 0x50), 0x4000);
        assert_eq!(e.mem.u32(data + 0x54), 0x4000 + 72);
        assert_eq!(e.mem.u32(data + 0x58), 0x4000 + 84);
        assert_eq!(e.mem.u32(data + 0x5c), 0x4000 + 96);
    }
}

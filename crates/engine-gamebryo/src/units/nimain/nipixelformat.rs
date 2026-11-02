//! `NiMain/nipixelformat.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `NiPixelFormat` (Xbox PDB), 0x44 bytes. `m_akComponents` (+0x14) is four
    /// `NiComponentSpec`s of 0xC bytes: `m_eComponent` (+0), `m_eRepresentation` (+4),
    /// `m_ucBitsPerComponent` (+8), `m_bSigned` (+9).
    pub struct NiPixelFormat: 0x44 {
        /// `m_uFlags` (Xbox PDB).
        0x00 m_uFlags: u8,
        /// `m_ucBitsPerPixel` (Xbox PDB).
        0x01 m_ucBitsPerPixel: u8,
        /// `m_eFormat` (Xbox PDB).
        0x04 m_eFormat: u32,
        /// `m_eTiling` (Xbox PDB).
        0x08 m_eTiling: u32,
        /// `m_uiRendererHint` (Xbox PDB).
        0x0C m_uiRendererHint: u32,
        /// `m_uiExtraData` (Xbox PDB).
        0x10 m_uiExtraData: u32,
    }
}

/// Offset of `m_akComponents` (Xbox PDB) and the size of one `NiComponentSpec`.
const COMPONENTS: u32 = 0x14;
const COMPONENT_SIZE: u32 = 0xc;

// Translated from 00a7cca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPixelFormat::NiPixelFormat` (engine map; `thiscall`, no stack word): the default
/// format. Flags 1, 8 bits per pixel, format 2, tiling 0, renderer hint `0xFFFFFFFF`, extra
/// data 0; the first component spec (3, 0x10, 8 bits, signed byte 1), the other three
/// (5, 0x13, 0 bits, signed byte 1). The numbers are stored as the exe has them; their enum
/// names are not confirmed here. The stores follow the exe's order. Returns `this`.
pub fn ni_pixel_format_ni_pixel_format(
    e: &mut Engine,
    this: Ptr<NiPixelFormat>,
) -> Ptr<NiPixelFormat> {
    e.set(this, NiPixelFormat::m_ucBitsPerPixel, 8);
    e.set(this, NiPixelFormat::m_eFormat, 2);
    e.set(this, NiPixelFormat::m_uiRendererHint, 0xffff_ffff);
    e.set(this, NiPixelFormat::m_uiExtraData, 0);
    e.set(this, NiPixelFormat::m_uFlags, 1);
    e.set(this, NiPixelFormat::m_eTiling, 0);
    let first = this.addr() + COMPONENTS;
    e.mem.set_u32(first, 0x10);
    e.mem.set_u32(first + 4, 3);
    e.mem.set_u8(first + 8, 8);
    e.mem.set_u8(first + 9, 1);
    for index in 1..4u32 {
        let spec = first + COMPONENT_SIZE * index;
        e.mem.set_u32(spec, 0x13);
        e.mem.set_u32(spec + 4, 5);
        e.mem.set_u8(spec + 8, 0);
        e.mem.set_u8(spec + 9, 1);
    }
    this
}

// Translated from 00a7cd20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPixelFormat::CreateFromRGBAMasks` (engine map; cdecl, the format returned through
/// `out` and five words): the default format ([`ni_pixel_format_ni_pixel_format`]) with
/// flag bit 0 set, `bits_per_pixel`, the renderer hint 0xFFFFFFFF, tiling 0 and the format
/// 1 when there is an alpha mask, else 0. Each mask's bit count is taken; the red, green
/// and blue masks (with their component numbers 0, 1, 2 and counts) are put in ascending
/// order by exchanges (the first against the later two, then the second against the third,
/// each only when its own mask is not 0); then each non-zero mask of the four gives the
/// next component spec (its component number, representation 0, its bit count, signed byte
/// 1), and the remaining specs are (0x13, 5, 0 bits, 1). Returns `out`.
pub fn ni_pixel_format_create_from_rgba_masks(
    e: &mut Engine,
    out: Ptr<NiPixelFormat>,
    bits_per_pixel: u8,
    red: u32,
    green: u32,
    blue: u32,
    alpha: u32,
) -> Ptr<NiPixelFormat> {
    ni_pixel_format_ni_pixel_format(e, out);
    let flags = e.get(out, NiPixelFormat::m_uFlags) | 1;
    e.set(out, NiPixelFormat::m_uFlags, flags);
    e.set(out, NiPixelFormat::m_ucBitsPerPixel, bits_per_pixel);
    e.set(out, NiPixelFormat::m_uiRendererHint, 0xffff_ffff);
    e.set(out, NiPixelFormat::m_eTiling, 0);
    e.set(out, NiPixelFormat::m_eFormat, u32::from(alpha != 0));
    let mut masks = [red, green, blue, alpha];
    let mut components = [0u32, 1, 2, 3];
    let mut bits = masks.map(|mask| {
        let mut count = 0u32;
        let mut rest = mask;
        while rest != 0 {
            count = count.wrapping_add(rest & 1);
            rest >>= 1;
        }
        count as u8
    });
    for i in 0..3usize {
        if masks[i] != 0 && i < 2 {
            for j in i + 1..3 {
                if masks[i] > masks[j] {
                    masks.swap(i, j);
                    bits.swap(i, j);
                    components.swap(i, j);
                }
            }
        }
    }
    let mut used = 0u32;
    for c in 0..4usize {
        if masks[c] != 0 {
            let spec = out.addr() + COMPONENTS + COMPONENT_SIZE * used;
            e.mem.set_u8(spec + 8, bits[c]);
            e.mem.set_u8(spec + 9, 1);
            e.mem.set_u32(spec + 4, 0);
            e.mem.set_u32(spec, components[c]);
            used += 1;
        }
    }
    while used < 4 {
        let spec = out.addr() + COMPONENTS + COMPONENT_SIZE * used;
        e.mem.set_u8(spec + 9, 1);
        e.mem.set_u8(spec + 8, 0);
        e.mem.set_u32(spec + 4, 5);
        e.mem.set_u32(spec, 0x13);
        used += 1;
    }
    out
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00a7_cd20,
            ni_pixel_format_create_from_rgba_masks(
                Ptr<NiPixelFormat>,
                u8,
                u32,
                u32,
                u32,
                u32,
            ) -> Ptr<NiPixelFormat>
        ),
        entry!(
            0x00a7_cca0,
            ni_pixel_format_ni_pixel_format(Ptr<NiPixelFormat>) -> Ptr<NiPixelFormat>
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_pixel_format_gets_its_default_fields() {
        let mut e = Engine::new();
        e.map(0x0000_1000, 0x100);
        for i in 0..0x50u32 {
            e.mem.set_u8(0x1000 + i, 0xaa);
        }
        assert_eq!(e.call(0x00a7_cca0, &args![0x1000u32]).u32(), 0x1000);
        let word = |e: &Engine, o: u32| e.mem.u32(0x1000 + o);
        assert_eq!(e.mem.u8(0x1000), 1);
        assert_eq!(e.mem.u8(0x1001), 8);
        assert_eq!(e.mem.u8(0x1002), 0xaa, "untouched");
        assert_eq!(
            [word(&e, 4), word(&e, 8), word(&e, 0xc), word(&e, 0x10)],
            [2, 0, 0xffff_ffff, 0]
        );
        assert_eq!([word(&e, 0x14), word(&e, 0x18)], [0x10, 3]);
        assert_eq!([e.mem.u8(0x101c), e.mem.u8(0x101d)], [8, 1]);
        assert_eq!(e.mem.u8(0x101e), 0xaa, "untouched");
        for entry in [0x20u32, 0x2c, 0x38] {
            assert_eq!([word(&e, entry), word(&e, entry + 4)], [0x13, 5]);
            assert_eq!(
                [e.mem.u8(0x1000 + entry + 8), e.mem.u8(0x1000 + entry + 9)],
                [0, 1]
            );
        }
        assert_eq!(e.mem.u8(0x1042), 0xaa, "untouched");
    }

    #[test]
    fn constructor_writes_the_default_format() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x44);
        for offset in (0..0x44).step_by(4) {
            e.mem.set_u32(this + offset, 0xaaaa_aaaa);
        }
        let r = e.call(0x00a7_cca0, &[this]);
        assert_eq!(r.u32(), this);
        assert_eq!(e.mem.u8(this), 1);
        assert_eq!(e.mem.u8(this + 1), 8);
        // The two padding bytes after m_ucBitsPerPixel are left as they were.
        assert_eq!(e.mem.u8(this + 2), 0xaa);
        let words: Vec<u32> = [4u32, 8, 0xc, 0x10, 0x14, 0x18]
            .iter()
            .map(|&o| e.mem.u32(this + o))
            .collect();
        assert_eq!(words, vec![2, 0, 0xffff_ffff, 0, 0x10, 3]);
        assert_eq!([e.mem.u8(this + 0x1c), e.mem.u8(this + 0x1d)], [8, 1]);
        for spec in [0x20u32, 0x2c, 0x38] {
            assert_eq!(
                [e.mem.u32(this + spec), e.mem.u32(this + spec + 4)],
                [0x13, 5]
            );
            assert_eq!(
                [e.mem.u8(this + spec + 8), e.mem.u8(this + spec + 9)],
                [0, 1]
            );
            assert_eq!(e.mem.u8(this + spec + 0xa), 0xaa);
        }
    }
}

#[cfg(test)]
mod n083_tests {
    use super::*;

    fn spec(e: &Engine, out: u32, index: u32) -> (u32, u32, u8, u8) {
        let at = out + COMPONENTS + COMPONENT_SIZE * index;
        (
            e.mem.u32(at),
            e.mem.u32(at + 4),
            e.mem.u8(at + 8),
            e.mem.u8(at + 9),
        )
    }

    #[test]
    fn masks_become_sorted_component_specs() {
        let mut e = Engine::new();
        let out = e.mem.alloc(0x44);
        // A8R8G8B8: red is the largest mask, so blue, green, red come out in that order.
        let r = e.call(
            0x00a7_cd20,
            &args![
                out,
                32u8,
                0x00ff_0000u32,
                0x0000_ff00u32,
                0x0000_00ffu32,
                0xff00_0000u32
            ],
        );
        assert_eq!(r.u32(), out);
        assert_eq!(e.mem.u8(out), 1);
        assert_eq!(e.mem.u8(out + 1), 32);
        assert_eq!(e.mem.u32(out + 4), 1);
        assert_eq!(e.mem.u32(out + 8), 0);
        assert_eq!(e.mem.u32(out + 0xc), 0xffff_ffff);
        assert_eq!(spec(&e, out, 0), (2, 0, 8, 1));
        assert_eq!(spec(&e, out, 1), (1, 0, 8, 1));
        assert_eq!(spec(&e, out, 2), (0, 0, 8, 1));
        assert_eq!(spec(&e, out, 3), (3, 0, 8, 1));
    }

    #[test]
    fn missing_masks_leave_default_specs() {
        let mut e = Engine::new();
        let out = e.mem.alloc(0x44);
        // R5G6B5 without alpha: already ascending from blue; a zero red is skipped.
        e.call(
            0x00a7_cd20,
            &args![out, 16u8, 0u32, 0x07e0u32, 0x001fu32, 0u32],
        );
        assert_eq!(e.mem.u32(out + 4), 0);
        assert_eq!(spec(&e, out, 0), (2, 0, 5, 1));
        assert_eq!(spec(&e, out, 1), (1, 0, 6, 1));
        assert_eq!(spec(&e, out, 2), (0x13, 5, 0, 1));
        assert_eq!(spec(&e, out, 3), (0x13, 5, 0, 1));
    }
}

//! `NiMain/run_00a7c9f0` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiPixelFormat` (Xbox PDB, 0x44 bytes): `m_uFlags` +0 (u8), `m_ucBitsPerPixel` +1 (u8),
/// `m_eFormat` +4, `m_eTiling` +8, `m_uiRendererHint` +0xc, `m_uiExtraData` +0x10, then the
/// component specifications. The PC code writes the same offsets.
const FLAGS: u32 = 0;
const BITS_PER_PIXEL: u32 = 1;
const FORMAT: u32 = 4;
const TILING: u32 = 8;
const RENDERER_HINT: u32 = 0xc;
const EXTRA_DATA: u32 = 0x10;
/// The size of an `NiPixelFormat` in words (the `REP MOVSD` count).
const PIXEL_FORMAT_WORDS: u32 = 0x11;

/// The format value the function stores when nothing matches (0x10).
const UNKNOWN_FORMAT: u32 = 0x10;

/// Whether the four channel masks at `masks` are the ones given (compared in order, the
/// exe's short-circuit `CMP`s).
fn masks_are(e: &Engine, masks: u32, expected: [u32; 4]) -> bool {
    expected
        .iter()
        .enumerate()
        .all(|(i, mask)| e.mem.u32(masks + 4 * i as u32) == *mask)
}

/// Copies the 0x44-byte pixel format at `preset` over `this` (`REP MOVSD`).
fn copy_preset(e: &mut Engine, this: u32, preset: u32) {
    for i in 0..PIXEL_FORMAT_WORDS {
        let word = e.mem.u32(preset + 4 * i);
        e.mem.set_u32(this + 4 * i, word);
    }
}

// Translated from 00a7c9f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on an `NiPixelFormat`, `RET 0xc`: a format value,
/// the bits per pixel (the low byte of its word) and the address of four channel masks).
/// Stores the bits per pixel, the format, renderer hint 0xffffffff, extra data 0, flags 1
/// and tiling 0, then, when the format, bits and masks are one of the exe's known
/// combinations, copies a whole static pixel format from `.data` over `this`; otherwise the
/// format becomes 0x10. The combinations (the compiler's jump table on the format is the
/// `match`):
///
/// - format 0: 16 bits with masks `f800 7e0 1f 0` (`011aa8d0`); 24 bits with `ff ff00
///   ff0000 0` (`011aa2e8`) or `ff0000 ff00 ff 0` (`011aaa80`);
/// - format 1: 16 bits with `1f 3e0 7c00 8000` (`011aa330`), `7c00 3e0 1f 8000`
///   (`011aa3c0`) or `f00 f0 f f000` (`011aa9f0`); 32 bits with `ff0000 ff00 ff ff000000`
///   (`011aaac8`), any other 32-bit masks (`011aa2a0`);
/// - format 2 or 3 with 8 bits (`011aa210`, `011aa258`);
/// - formats 4, 5, 6, 8 and 9 whatever the bits and masks (`011aa498`, `011aa4e0`,
///   `011aa528`, `011aa408`, `011aa450`).
///
/// The masks are only read where the exe compares them.
pub fn fn_00a7c9f0(e: &mut Engine, this: Ptr, format: u32, bits: u8, masks: Ptr) {
    let base = this.addr();
    let masks = masks.addr();
    e.mem.set_u8(base + BITS_PER_PIXEL, bits);
    e.mem.set_u32(base + FORMAT, format);
    e.mem.set_u32(base + RENDERER_HINT, 0xffff_ffff);
    e.mem.set_u32(base + EXTRA_DATA, 0);
    e.mem.set_u8(base + FLAGS, 1);
    e.mem.set_u32(base + TILING, 0);
    let preset = match format {
        0 => match bits {
            0x10 if masks_are(e, masks, [0xf800, 0x7e0, 0x1f, 0]) => Some(0x011a_a8d0),
            0x18 if masks_are(e, masks, [0xff, 0xff00, 0x00ff_0000, 0]) => Some(0x011a_a2e8),
            0x18 if masks_are(e, masks, [0x00ff_0000, 0xff00, 0xff, 0]) => Some(0x011a_aa80),
            _ => None,
        },
        1 => match bits {
            0x10 if masks_are(e, masks, [0x1f, 0x3e0, 0x7c00, 0x8000]) => Some(0x011a_a330),
            0x10 if masks_are(e, masks, [0x7c00, 0x3e0, 0x1f, 0x8000]) => Some(0x011a_a3c0),
            0x10 if masks_are(e, masks, [0xf00, 0xf0, 0xf, 0xf000]) => Some(0x011a_a9f0),
            0x20 if masks_are(e, masks, [0x00ff_0000, 0xff00, 0xff, 0xff00_0000]) => {
                Some(0x011a_aac8)
            }
            0x20 => Some(0x011a_a2a0),
            _ => None,
        },
        2 if bits == 8 => Some(0x011a_a210),
        3 if bits == 8 => Some(0x011a_a258),
        4 => Some(0x011a_a498),
        5 => Some(0x011a_a4e0),
        6 => Some(0x011a_a528),
        8 => Some(0x011a_a408),
        9 => Some(0x011a_a450),
        _ => None,
    };
    match preset {
        Some(preset) => copy_preset(e, base, preset),
        None => e.mem.set_u32(base + FORMAT, UNKNOWN_FORMAT),
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a7_c9f0, fn_00a7c9f0(Ptr, u32, u8, Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// An engine with each preset's first word set to its own address.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011a_a000, 0x1000);
        for preset in [
            0x011a_a210u32,
            0x011a_a258,
            0x011a_a2a0,
            0x011a_a2e8,
            0x011a_a330,
            0x011a_a3c0,
            0x011a_a408,
            0x011a_a450,
            0x011a_a498,
            0x011a_a4e0,
            0x011a_a528,
            0x011a_a8d0,
            0x011a_a9f0,
            0x011a_aa80,
            0x011a_aac8,
        ] {
            e.mem.set_u32(preset, preset);
            e.mem.set_u32(preset + 0x40, !preset);
        }
        e
    }

    fn run(e: &mut Engine, format: u32, bits: u32, masks: [u32; 4]) -> (u32, u32) {
        let this = e.mem.alloc(0x44);
        let words = e.mem.alloc(0x10);
        for (i, mask) in masks.iter().enumerate() {
            e.mem.set_u32(words + 4 * i as u32, *mask);
        }
        e.call(0x00a7_c9f0, &args![this, format, bits, words]);
        (this, e.mem.u32(this))
    }

    #[test]
    fn known_combinations_copy_their_preset() {
        let mut e = engine();
        for (format, bits, masks, preset) in [
            (0u32, 0x10u32, [0xf800u32, 0x7e0, 0x1f, 0], 0x011a_a8d0u32),
            (0, 0x18, [0xff, 0xff00, 0xff_0000, 0], 0x011a_a2e8),
            (0, 0x18, [0xff_0000, 0xff00, 0xff, 0], 0x011a_aa80),
            (1, 0x10, [0x1f, 0x3e0, 0x7c00, 0x8000], 0x011a_a330),
            (1, 0x10, [0x7c00, 0x3e0, 0x1f, 0x8000], 0x011a_a3c0),
            (1, 0x10, [0xf00, 0xf0, 0xf, 0xf000], 0x011a_a9f0),
            (1, 0x20, [0xff_0000, 0xff00, 0xff, 0xff00_0000], 0x011a_aac8),
            (1, 0x20, [1, 2, 3, 4], 0x011a_a2a0),
            (2, 0x108, [0; 4], 0x011a_a210),
            (3, 8, [0; 4], 0x011a_a258),
            (4, 0, [0; 4], 0x011a_a498),
            (5, 0, [0; 4], 0x011a_a4e0),
            (6, 0, [0; 4], 0x011a_a528),
            (8, 0, [0; 4], 0x011a_a408),
            (9, 0, [0; 4], 0x011a_a450),
        ] {
            let (this, first) = run(&mut e, format, bits, masks);
            assert_eq!(first, preset, "format {format} bits {bits:#x}");
            assert_eq!(e.mem.u32(this + 0x40), !preset);
        }
    }

    #[test]
    fn anything_else_keeps_the_fields_and_sets_format_0x10() {
        let mut e = engine();
        for (format, bits, masks) in [
            (0u32, 0x10u32, [0xf800u32, 0x7e0, 0x1f, 1]),
            (0, 0x20, [0; 4]),
            (1, 0x18, [0; 4]),
            (1, 0x10, [0; 4]),
            (2, 0x10, [0; 4]),
            (3, 0x10, [0; 4]),
            (7, 8, [0; 4]),
            (10, 8, [0; 4]),
        ] {
            let this = e.mem.alloc(0x44);
            e.mem.write(this, &[0xaa; 0x44]);
            let words = e.mem.alloc(0x10);
            for (i, mask) in masks.iter().enumerate() {
                e.mem.set_u32(words + 4 * i as u32, *mask);
            }
            e.call(0x00a7_c9f0, &args![this, format, bits, words]);
            assert_eq!(e.mem.u8(this), 1);
            assert_eq!(e.mem.u8(this + 1), bits as u8);
            assert_eq!(e.mem.u16(this + 2), 0xaaaa);
            assert_eq!(e.mem.u32(this + 4), 0x10);
            assert_eq!(e.mem.u32(this + 8), 0);
            assert_eq!(e.mem.u32(this + 0xc), 0xffff_ffff);
            assert_eq!(e.mem.u32(this + 0x10), 0);
            assert_eq!(e.mem.u32(this + 0x14), 0xaaaa_aaaa);
        }
    }

    #[test]
    fn masks_are_not_read_when_the_bits_do_not_ask_for_them() {
        let mut e = engine();
        let this = e.mem.alloc(0x44);
        // An unmapped mask address: reading it would stop the test.
        e.call(0x00a7_c9f0, &args![this, 4u32, 0u32, 0x0f00_0000u32]);
        assert_eq!(e.mem.u32(this), 0x011a_a498);
        e.call(0x00a7_c9f0, &args![this, 0u32, 8u32, 0x0f00_0000u32]);
        assert_eq!(e.mem.u32(this + 4), 0x10);
    }
}

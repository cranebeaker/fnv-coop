//! `NiMain/nitexturecodec.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Every function here keeps a /GS stack cookie (`011c16bc`, checked by
//! `__security_check_cookie` `00ec408c` on return); the engine does not model it, so each
//! is marked incomplete for that alone.

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `NiTextureCodec` (Xbox PDB), 0x210 bytes: the fields these functions use, at the
    /// PC offsets (the same).
    pub struct NiTextureCodec: 0x210 {
        /// `m_pfnAlphaBlockDecoder` (Xbox PDB): thiscall `(this, out)`, or null.
        0x00 m_pfnAlphaBlockDecoder: u32,
        /// `m_pfnColorBlockDecoder` (Xbox PDB): thiscall `(this, out)`.
        0x04 m_pfnColorBlockDecoder: u32,
        /// `m_pucSrcPtr` (Xbox PDB): the next compressed byte.
        0x08 m_pucSrcPtr: u32,
        /// `m_uiDestRowStride` (Xbox PDB): bytes from one output row to the next.
        0x0C m_uiDestRowStride: u32,
    }
}

/// `NiPixelData` fields (Xbox PDB, PC offsets): `m_kPixelFormat.m_eFormat` +0x0C,
/// `m_pucPixels` +0x50, `m_puiWidth` +0x54, `m_puiHeight` +0x58, `m_puiOffsetInBytes` +0x5C
/// (one more entry than levels: the face size), `m_uiMipmapLevels` +0x60, `m_uiFaces` +0x6C.
const PIXEL_FORMAT: u32 = 0x0c;
const PIXELS: u32 = 0x50;
const WIDTHS: u32 = 0x54;
const HEIGHTS: u32 = 0x58;
const OFFSETS: u32 = 0x5c;
const MIPMAP_LEVELS: u32 = 0x60;
const FACES: u32 = 0x6c;
/// The decoders `ExpandCompressedData` installs: for format 4 none and `00a8bab0`, for 5
/// `00a8b4b0` and [`fn_00a8b8c0`], for 6 [`fn_00a8b5e0`] and [`fn_00a8b8c0`] (alpha,
/// colour; `00a8b4b0` and `00a8bab0` are unnamed and outside this unit list).
const DECODER_00A8BAB0: u32 = 0x00a8_bab0;
const DECODER_00A8B4B0: u32 = 0x00a8_b4b0;
const DECODER_00A8B5E0: u32 = 0x00a8_b5e0;
const DECODER_00A8B8C0: u32 = 0x00a8_b8c0;
/// `NiNew` allocator (`00aa13e0`, cdecl: size), `NiPixelData::NiPixelData_ov2` (`00a7c190`,
/// thiscall: width, height, the format, mipmap levels, faces) and the format it is given
/// (`011aa2a0`); `_memcpy` (`00ec44d0`, cdecl: destination, source, size).
const NI_NEW: u32 = 0x00aa_13e0;
const PIXEL_DATA_CONSTRUCT: u32 = 0x00a7_c190;
const EXPANDED_FORMAT: u32 = 0x011a_a2a0;
const MEMCPY: u32 = 0x00ec_44d0;

/// The decoder calls: the alpha decoder (when set) into `block + 3`, then the colour
/// decoder into `block`.
fn decode_block(e: &mut Engine, this: Ptr<NiTextureCodec>, block: u32) {
    let alpha = e.get(this, NiTextureCodec::m_pfnAlphaBlockDecoder);
    if alpha != 0 {
        e.call(alpha, &args![this, block + 3]);
    }
    let colour = e.get(this, NiTextureCodec::m_pfnColorBlockDecoder);
    e.call(colour, &args![this, block]);
}

// Translated from 00a8b5e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00a8b5e0 - the /GS stack cookie (011c16bc, __security_check_cookie 00ec408c) is not translated
/// Unnamed in the engine map; the body is an interpolated alpha block decoder (thiscall,
/// `RET 4`: the output, one byte every four): two alpha bytes `a`, `b` from the source,
/// then a table of eight: `a`, `b` and, when `a > b`, the six sevenths `(6a+b)/7` ...
/// `(a+6b)/7`; otherwise the four fifths `(4a+b)/5` ... `(a+4b)/5`, 0 and 255. Then two
/// groups of three source bytes give sixteen 3-bit indices into the table, written to every
/// fourth output byte. The source pointer moves on by 8.
pub fn fn_00a8b5e0(e: &mut Engine, this: Ptr<NiTextureCodec>, out: u32) {
    let source = e.get(this, NiTextureCodec::m_pucSrcPtr);
    let a = u32::from(e.mem.u8(source));
    e.set(this, NiTextureCodec::m_pucSrcPtr, source + 1);
    let b = u32::from(e.mem.u8(source + 1));
    e.set(this, NiTextureCodec::m_pucSrcPtr, source + 2);
    let mut table = [a as u8, b as u8, 0, 0, 0, 0, 0, 0];
    if a > b {
        let weights = [(6, 1), (5, 2), (4, 3), (3, 4), (2, 5), (1, 6)];
        for (i, (wa, wb)) in weights.into_iter().enumerate() {
            table[i + 2] = ((a * wa + b * wb) / 7) as u8;
        }
    } else {
        let weights = [(4, 1), (3, 2), (2, 3), (1, 4)];
        for (i, (wa, wb)) in weights.into_iter().enumerate() {
            table[i + 2] = ((a * wa + b * wb) / 5) as u8;
        }
        table[6] = 0;
        table[7] = 0xff;
    }
    for group in 0..2u32 {
        let at = e.get(this, NiTextureCodec::m_pucSrcPtr);
        let s0 = u32::from(e.mem.u8(at));
        let s1 = u32::from(e.mem.u8(at + 1));
        let s2 = u32::from(e.mem.u8(at + 2));
        let indices = [
            s0 & 7,
            (s0 >> 3) & 7,
            (s0 >> 6) + (s1 & 1) * 4,
            (s1 >> 1) & 7,
            (s1 >> 4) & 7,
            (s1 >> 7) + (s2 & 3) * 2,
            (s2 >> 2) & 7,
            s2 >> 5,
        ];
        for (i, index) in indices.into_iter().enumerate() {
            let byte = out.wrapping_add(group * 0x20 + i as u32 * 4);
            e.mem.set_u8(byte, table[index as usize]);
        }
        let at = e.get(this, NiTextureCodec::m_pucSrcPtr);
        e.set(this, NiTextureCodec::m_pucSrcPtr, at + 3);
    }
}

// Translated from 00a8b8c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00a8b8c0 - the /GS stack cookie (011c16bc, __security_check_cookie 00ec408c) is not translated
/// Unnamed in the engine map; the body is a four-colour block decoder (thiscall, `RET 4`:
/// the output, four bytes per pixel of which the first three are written): two 16-bit
/// colours from the source give the bytes `(c >> 8) & 0xF8`, `(c >> 3) & 0xFE` (of the low
/// byte) and `c << 3` (of the low byte); the third and fourth colours are `(2 c0 + c1) / 3`
/// and `(c0 + 2 c1) / 3` byte by byte. Then four source bytes, one per row, give four 2-bit
/// indices each. The source pointer moves on by 8.
pub fn fn_00a8b8c0(e: &mut Engine, this: Ptr<NiTextureCodec>, out: u32) {
    let source = e.get(this, NiTextureCodec::m_pucSrcPtr);
    let word = |e: &Engine, at: u32| {
        u16::from(e.mem.u8(at + 1))
            .wrapping_mul(0x100)
            .wrapping_add(u16::from(e.mem.u8(at)))
    };
    let first = word(e, source);
    e.set(this, NiTextureCodec::m_pucSrcPtr, source + 2);
    let second = word(e, source + 2);
    let channels = |c: u16| -> [u8; 3] {
        [
            ((c >> 8) as u8) & 0xf8,
            ((c >> 3) as u8) & 0xfe,
            (c as u8).wrapping_mul(8),
        ]
    };
    let c0 = channels(first);
    let c1 = channels(second);
    e.set(this, NiTextureCodec::m_pucSrcPtr, source + 4);
    let mut palette = [0u8; 12];
    for k in 0..3 {
        let (p, q) = (u32::from(c0[k]), u32::from(c1[k]));
        palette[k] = c0[k];
        palette[3 + k] = c1[k];
        palette[6 + k] = ((q + p * 2) / 3) as u8;
        palette[9 + k] = ((p + q * 2) / 3) as u8;
    }
    let mut row_out = out;
    for _ in 0..4 {
        let at = e.get(this, NiTextureCodec::m_pucSrcPtr);
        let bits = e.mem.u8(at);
        for pixel in 0..4u32 {
            let index = usize::from((bits >> (pixel * 2)) & 3);
            for k in 0..3u32 {
                e.mem.set_u8(
                    row_out.wrapping_add(pixel * 4 + k),
                    palette[index * 3 + k as usize],
                );
            }
        }
        e.set(this, NiTextureCodec::m_pucSrcPtr, at + 1);
        row_out = row_out.wrapping_add(0x10);
    }
}

// Translated from 00a8bdf0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00a8bdf0 - the /GS stack cookie (011c16bc, __security_check_cookie 00ec408c) is not translated
/// Unnamed in the engine map; the body decodes one block into a 64-byte local and copies a
/// part of it (thiscall, `RET 0xc`: the destination, the width in pixels, the rows): the
/// decoders as [`ni_texture_codec_expand_compressed_data`] calls them, then for each row
/// `_memcpy` of `width * 4` bytes, the destination moving on by the row stride and the
/// local by 16 bytes.
pub fn fn_00a8bdf0(e: &mut Engine, this: Ptr<NiTextureCodec>, dest: u32, width: u32, rows: u32) {
    e.with_stack(0x48, |e, frame| {
        let mut block = frame.addr() + 4;
        decode_block(e, this, block);
        let bytes = width.wrapping_mul(4);
        let mut to = dest;
        let mut remaining = rows;
        while remaining != 0 {
            e.call(MEMCPY, &args![to, block, bytes]);
            to = to.wrapping_add(e.get(this, NiTextureCodec::m_uiDestRowStride));
            block = block.wrapping_add(0x10);
            remaining -= 1;
        }
    });
}

// Translated from 00a8be70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00a8be70 - the /GS stack cookie (011c16bc, __security_check_cookie 00ec408c) is not translated
/// `NiTextureCodec::ExpandCompressedData` (Xbox PDB; thiscall, `RET 4`: the source pixel
/// data). Formats 4, 5 and 6 install their decoders (see the constants); any other format
/// gives 0. A new 0x74-byte `NiPixelData` (`00aa13e0`, `NiPixelData_ov2` with the first
/// level's width and height, the format at `011aa2a0`, the levels and the faces; 0 when the
/// allocation fails, and the code then reads through it as it is). For each face and level
/// the source pointer and the destination start at `offsets[levels] * face +
/// offsets[level]` past each one's pixels, the row stride is `width * 4`; each row of whole
/// 4x4 blocks is decoded into a 64-byte local and copied 16 bytes per row, the leftover
/// columns through [`fn_00a8bdf0`]; then the leftover rows the same way. Returns the new
/// pixel data.
pub fn ni_texture_codec_expand_compressed_data(
    e: &mut Engine,
    this: Ptr<NiTextureCodec>,
    source: Ptr,
) -> u32 {
    let src = source.addr();
    let (alpha, colour) = match e.mem.u32(src + PIXEL_FORMAT) {
        4 => (0, DECODER_00A8BAB0),
        5 => (DECODER_00A8B4B0, DECODER_00A8B8C0),
        6 => (DECODER_00A8B5E0, DECODER_00A8B8C0),
        _ => return 0,
    };
    e.set(this, NiTextureCodec::m_pfnAlphaBlockDecoder, alpha);
    e.set(this, NiTextureCodec::m_pfnColorBlockDecoder, colour);
    let memory = e.call(NI_NEW, &args![0x74u32]).u32();
    let data = if memory == 0 {
        0
    } else {
        let faces = e.mem.u32(src + FACES);
        let levels = e.mem.u32(src + MIPMAP_LEVELS);
        let height = e.mem.u32(e.mem.u32(src + HEIGHTS));
        let width = e.mem.u32(e.mem.u32(src + WIDTHS));
        e.call(
            PIXEL_DATA_CONSTRUCT,
            &args![memory, width, height, EXPANDED_FORMAT, levels, faces],
        )
        .u32()
    };
    let faces = e.mem.u32(src + FACES);
    let levels = e.mem.u32(src + MIPMAP_LEVELS);
    let level_start = |e: &Engine, pixels: u32, face: u32, level: u32| {
        let offsets = e.mem.u32(pixels + OFFSETS);
        let face_size = e
            .mem
            .u32(offsets.wrapping_add(e.mem.u32(pixels + MIPMAP_LEVELS).wrapping_mul(4)));
        face_size
            .wrapping_mul(face)
            .wrapping_add(e.mem.u32(offsets.wrapping_add(level.wrapping_mul(4))))
            .wrapping_add(e.mem.u32(pixels + PIXELS))
    };
    // The exe's frame (`SUB ESP,0x74`, four pushes): 0x84 bytes, the decoded 64-byte block at
    // +0x40 (00a8bfd9), just under the cookie word.
    e.with_stack(0x84, |e, frame| {
        let block = frame.addr() + 0x40;
        let mut face = 0u32;
        while face < faces {
            let mut level = 0u32;
            while level < levels {
                let from = level_start(e, src, face, level);
                e.set(this, NiTextureCodec::m_pucSrcPtr, from);
                let mut dest = level_start(e, data, face, level);
                let width = e
                    .mem
                    .u32(e.mem.u32(src + WIDTHS).wrapping_add(level.wrapping_mul(4)));
                let height = e
                    .mem
                    .u32(e.mem.u32(src + HEIGHTS).wrapping_add(level.wrapping_mul(4)));
                let blocks_across = width >> 2;
                let block_rows = height >> 2;
                let rows_left = height & 3;
                let columns_left = width & 3;
                e.set(
                    this,
                    NiTextureCodec::m_uiDestRowStride,
                    width.wrapping_mul(4),
                );
                let row_step = width.wrapping_mul(12);
                for _ in 0..block_rows {
                    for _ in 0..blocks_across {
                        decode_block(e, this, block);
                        let mut to = dest;
                        for row in 0..4u32 {
                            for word in 0..4u32 {
                                let value = e.mem.u32(block + row * 0x10 + word * 4);
                                e.mem.set_u32(to.wrapping_add(word * 4), value);
                            }
                            to = to.wrapping_add(e.get(this, NiTextureCodec::m_uiDestRowStride));
                        }
                        dest = dest.wrapping_add(0x10);
                    }
                    if columns_left != 0 {
                        fn_00a8bdf0(e, this, dest, columns_left, 4);
                        dest = dest.wrapping_add(columns_left * 4);
                    }
                    dest = dest.wrapping_add(row_step);
                }
                if rows_left != 0 {
                    for _ in 0..blocks_across {
                        fn_00a8bdf0(e, this, dest, 4, rows_left);
                        dest = dest.wrapping_add(0x10);
                    }
                    if columns_left != 0 {
                        fn_00a8bdf0(e, this, dest, columns_left, rows_left);
                    }
                }
                level += 1;
            }
            face += 1;
        }
    });
    data
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a8_b5e0, fn_00a8b5e0(Ptr<NiTextureCodec>, u32)),
        entry!(0x00a8_b8c0, fn_00a8b8c0(Ptr<NiTextureCodec>, u32)),
        entry!(0x00a8_bdf0, fn_00a8bdf0(Ptr<NiTextureCodec>, u32, u32, u32)),
        entry!(
            0x00a8_be70,
            ni_texture_codec_expand_compressed_data(Ptr<NiTextureCodec>, Ptr) -> u32
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn codec(e: &mut Engine, source: &[u8]) -> Ptr<NiTextureCodec> {
        let this = e.new_object::<NiTextureCodec>();
        let bytes = e.mem.alloc(source.len() as u32 + 4);
        e.mem.write(bytes, source);
        e.set(this, NiTextureCodec::m_pucSrcPtr, bytes);
        this
    }

    fn every_fourth(e: &Engine, out: u32, first: u32) -> Vec<u8> {
        (0..16).map(|i| e.mem.u8(out + first + 4 * i)).collect()
    }

    #[test]
    fn alpha_block_uses_sevenths_or_fifths() {
        let mut e = Engine::new();
        // a = 0xff > b = 0: indices 0..7 then 7..0 (3 bits each, little-endian).
        let mut source = vec![0xff, 0x00];
        let indices: Vec<u32> = (0..8).chain((0..8).rev()).collect();
        for group in indices.chunks(8) {
            let bits = group
                .iter()
                .enumerate()
                .fold(0u32, |bits, (i, v)| bits | v << (3 * i));
            source.extend_from_slice(&bits.to_le_bytes()[..3]);
        }
        let this = codec(&mut e, &source);
        let start = e.get(this, NiTextureCodec::m_pucSrcPtr);
        let out = e.mem.alloc(0x40);
        e.call(0x00a8_b5e0, &args![this, out]);
        let table = [255u8, 0, 218, 182, 145, 109, 72, 36];
        let expected: Vec<u8> = indices.iter().map(|i| table[*i as usize]).collect();
        assert_eq!(every_fourth(&e, out, 0), expected);
        assert_eq!(e.get(this, NiTextureCodec::m_pucSrcPtr), start + 8);
        // a <= b: fifths, then 0 and 255.
        let mut source = vec![10, 60];
        source.extend_from_slice(&[0x88, 0xc6, 0xfa, 0, 0, 0]);
        let this = codec(&mut e, &source);
        e.call(0x00a8_b5e0, &args![this, out]);
        let table = [10u8, 60, 20, 30, 40, 50, 0, 255];
        let got = every_fourth(&e, out, 0);
        // The first eight indices are 0, 1, ..., 7 (0xfac688 in 3-bit fields).
        assert_eq!(got[..8], table);
        assert_eq!(got[8..], [10u8; 8]);
    }

    #[test]
    fn colour_block_expands_two_colours_to_four() {
        let mut e = Engine::new();
        // c0 = 0xffff (white), c1 = 0x0000; rows of indices 0,1,2,3 / 3,2,1,0 / 0... .
        let source = [0xff, 0xff, 0x00, 0x00, 0xe4, 0x1b, 0x00, 0xff];
        let this = codec(&mut e, &source);
        let start = e.get(this, NiTextureCodec::m_pucSrcPtr);
        let out = e.mem.alloc(0x40);
        for i in 0..16 {
            e.mem.set_u8(out + 4 * i + 3, 0x77);
        }
        e.call(0x00a8_b8c0, &args![this, out]);
        let white = [0xf8u8, 0xfe, 0xf8];
        let third = [0xa5u8, 0xa9, 0xa5];
        let fourth = [0x52u8, 0x54, 0x52];
        let pixel =
            |e: &Engine, i: u32| -> [u8; 4] { [0, 1, 2, 3].map(|k| e.mem.u8(out + 4 * i + k)) };
        let with_alpha = |c: [u8; 3]| [c[0], c[1], c[2], 0x77];
        assert_eq!(pixel(&e, 0), with_alpha(white));
        assert_eq!(pixel(&e, 1), with_alpha([0, 0, 0]));
        assert_eq!(pixel(&e, 2), with_alpha(third));
        assert_eq!(pixel(&e, 3), with_alpha(fourth));
        assert_eq!(pixel(&e, 4), with_alpha(fourth));
        assert_eq!(pixel(&e, 7), with_alpha(white));
        assert_eq!(pixel(&e, 15), with_alpha(fourth));
        assert_eq!(e.get(this, NiTextureCodec::m_pucSrcPtr), start + 8);
    }

    const ALPHA_DOUBLE: u32 = 0x0710_0000;
    const COLOUR_DOUBLE: u32 = 0x0710_0010;

    /// Decoder doubles: the colour one fills each pixel with its index and the call count,
    /// the alpha one marks byte 3.
    fn decoder_doubles(e: &mut Engine) {
        let mut calls = 0u8;
        e.register_double(COLOUR_DOUBLE, move |e, a| {
            calls = calls.wrapping_add(1);
            for i in 0..16u32 {
                e.mem.set_u8(a[1] + 4 * i, i as u8);
                e.mem.set_u8(a[1] + 4 * i + 1, calls);
            }
            Ret::default()
        });
        e.register(ALPHA_DOUBLE, |e, a| {
            for i in 0..16u32 {
                e.mem.set_u8(a[1] + 4 * i, 0xaa);
            }
            Ret::default()
        });
        e.register(MEMCPY, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
    }

    #[test]
    fn partial_block_copies_the_requested_corner() {
        let mut e = Engine::new();
        decoder_doubles(&mut e);
        let this = codec(&mut e, &[]);
        e.set(this, NiTextureCodec::m_pfnAlphaBlockDecoder, ALPHA_DOUBLE);
        e.set(this, NiTextureCodec::m_pfnColorBlockDecoder, COLOUR_DOUBLE);
        e.set(this, NiTextureCodec::m_uiDestRowStride, 0x20);
        let dest = e.mem.alloc(0x80);
        e.call(0x00a8_bdf0, &args![this, dest, 3u32, 2u32]);
        // Rows 0 and 1, three pixels each.
        assert_eq!(e.mem.bytes(dest, 4), vec![0, 1, 0, 0xaa]);
        assert_eq!(e.mem.u8(dest + 8), 2);
        assert_eq!(e.mem.u8(dest + 12), 0);
        assert_eq!(e.mem.u8(dest + 0x20), 4);
        assert_eq!(e.mem.u8(dest + 0x40), 0);
    }

    #[test]
    fn expand_installs_decoders_and_fills_every_level() {
        let mut e = Engine::new();
        decoder_doubles(&mut e);
        // Source: one face, one level of 6 x 5 pixels (one whole block, a leftover column
        // pair and a leftover row).
        let source = e.mem.alloc(0x74);
        e.mem.set_u32(source + PIXEL_FORMAT, 6);
        let widths = e.mem.alloc(4);
        e.mem.set_u32(widths, 6);
        let heights = e.mem.alloc(4);
        e.mem.set_u32(heights, 5);
        let offsets = e.mem.alloc(8);
        e.mem.set_u32(offsets + 4, 0x40);
        let pixels = e.mem.alloc(0x40);
        for (field, value) in [
            (WIDTHS, widths),
            (HEIGHTS, heights),
            (OFFSETS, offsets),
            (PIXELS, pixels),
            (MIPMAP_LEVELS, 1),
            (FACES, 1),
        ] {
            e.mem.set_u32(source + field, value);
        }
        // The new pixel data: 6 x 5 x 4 bytes.
        let out = e.mem.alloc(0x200);
        let new_offsets = e.mem.alloc(8);
        e.mem.set_u32(new_offsets + 4, 6 * 5 * 4);
        e.register(NI_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register_double(PIXEL_DATA_CONSTRUCT, move |e, a| {
            assert_eq!(a[1..5], [6, 5, EXPANDED_FORMAT, 1]);
            e.mem.set_u32(a[0] + PIXELS, out);
            e.mem.set_u32(a[0] + OFFSETS, new_offsets);
            e.mem.set_u32(a[0] + MIPMAP_LEVELS, 1);
            a[0].into_ret()
        });
        e.register(DECODER_00A8B5E0, |e, a| {
            for i in 0..16u32 {
                e.mem.set_u8(a[1] + 4 * i, 0xaa);
            }
            Ret::default()
        });
        e.register_double(DECODER_00A8B8C0, move |e, a| {
            for i in 0..16u32 {
                e.mem.set_u8(a[1] + 4 * i, i as u8);
                e.mem.set_u8(a[1] + 4 * i + 1, 1);
            }
            Ret::default()
        });
        let this = codec(&mut e, &[]);
        e.call_log = Some(vec![]);
        let data = e.call(0x00a8_be70, &args![this, source]).u32();
        assert_ne!(data, 0);
        assert_eq!(
            e.get(this, NiTextureCodec::m_pfnAlphaBlockDecoder),
            DECODER_00A8B5E0
        );
        assert_eq!(e.get(this, NiTextureCodec::m_uiDestRowStride), 24);
        // The whole block lands at the start, rows 24 bytes apart.
        assert_eq!(e.mem.bytes(out, 4), vec![0, 1, 0, 0xaa]);
        assert_eq!(e.mem.u8(out + 24 + 4), 5);
        // The leftover columns (2) of the block row, then the leftover row (1) of the
        // block and of the two columns.
        let copies: Vec<(u32, u32)> = e
            .call_log
            .iter()
            .flatten()
            .filter(|(a, _)| *a == MEMCPY)
            .map(|(_, w)| (w[0] - out, w[2]))
            .collect();
        assert_eq!(
            copies,
            vec![
                (16, 8),
                (16 + 24, 8),
                (16 + 48, 8),
                (16 + 72, 8),
                (96, 16),
                (112, 8)
            ]
        );
        // Unknown formats give 0.
        e.mem.set_u32(source + PIXEL_FORMAT, 7);
        assert_eq!(e.call(0x00a8_be70, &args![this, source]).u32(), 0);
        e.mem.set_u32(source + PIXEL_FORMAT, 4);
        e.mem.set_u32(source + FACES, 0);
        e.call(0x00a8_be70, &args![this, source]);
        assert_eq!(e.get(this, NiTextureCodec::m_pfnAlphaBlockDecoder), 0);
        assert_eq!(
            e.get(this, NiTextureCodec::m_pfnColorBlockDecoder),
            DECODER_00A8BAB0
        );
    }

    #[test]
    fn the_decoded_block_sits_at_0x40_of_the_exe_frame() {
        let mut e = Engine::new();
        decoder_doubles(&mut e);
        // Source: one face, one level of 6 x 5 pixels (one whole block, a leftover column
        // pair and a leftover row).
        let source = e.mem.alloc(0x74);
        e.mem.set_u32(source + PIXEL_FORMAT, 6);
        let widths = e.mem.alloc(4);
        e.mem.set_u32(widths, 6);
        let heights = e.mem.alloc(4);
        e.mem.set_u32(heights, 5);
        let offsets = e.mem.alloc(8);
        e.mem.set_u32(offsets + 4, 0x40);
        let pixels = e.mem.alloc(0x40);
        for (field, value) in [
            (WIDTHS, widths),
            (HEIGHTS, heights),
            (OFFSETS, offsets),
            (PIXELS, pixels),
            (MIPMAP_LEVELS, 1),
            (FACES, 1),
        ] {
            e.mem.set_u32(source + field, value);
        }
        // The new pixel data: 6 x 5 x 4 bytes.
        let out = e.mem.alloc(0x200);
        let new_offsets = e.mem.alloc(8);
        e.mem.set_u32(new_offsets + 4, 6 * 5 * 4);
        e.register(NI_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register_double(PIXEL_DATA_CONSTRUCT, move |e, a| {
            assert_eq!(a[1..5], [6, 5, EXPANDED_FORMAT, 1]);
            e.mem.set_u32(a[0] + PIXELS, out);
            e.mem.set_u32(a[0] + OFFSETS, new_offsets);
            e.mem.set_u32(a[0] + MIPMAP_LEVELS, 1);
            a[0].into_ret()
        });
        let blocks = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let sink = std::rc::Rc::clone(&blocks);
        e.register_double(DECODER_00A8B5E0, move |_, a| {
            sink.borrow_mut().push(a[1]);
            Ret::default()
        });
        e.register(DECODER_00A8B8C0, |_, _| Ret::default());
        let this = codec(&mut e, &[]);
        // The frame is the next local the game thread's stack gives out.
        let frame = e.next_stack_local(0x84);
        e.call(0x00a8_be70, &args![this, source]);
        // The alpha decoder writes the fourth byte of each pixel of the block (block + 3).
        assert_eq!(blocks.borrow()[0], frame + 0x40 + 3);
    }
}

//! Rows of the non-block formats to and from D3DX's working colour (`d3dx9_38.dll` codecs:
//! R8G8B8 0x61b1c5, A8R8G8B8 0x61b288 / 0x6148a1, X8R8G8B8 0x61b330 / 0x614c75, A4R4G4B4
//! 0x61b69c / 0x6159bc).

use crate::dxt::Color;
use crate::format;
use crate::num::{fistp, Rz, INV15, INV255, Z};

/// The formats this crate reads and writes row by row.
pub fn supported(f: u32) -> bool {
    matches!(
        f,
        format::R8G8B8 | format::A8R8G8B8 | format::X8R8G8B8 | format::A4R4G4B4
    )
}

/// One row of `f` (`w` pixels) to the working colour.
pub fn decode_row(f: u32, row: &[u8], w: usize, out: &mut [Color]) {
    match f {
        format::A8R8G8B8 => {
            for (o, p) in out[..w].iter_mut().zip(row.chunks_exact(4)) {
                *o = [
                    f32::from(p[2]) * INV255,
                    f32::from(p[1]) * INV255,
                    f32::from(p[0]) * INV255,
                    f32::from(p[3]) * INV255,
                ];
            }
        }
        format::X8R8G8B8 => {
            for (o, p) in out[..w].iter_mut().zip(row.chunks_exact(4)) {
                *o = [
                    f32::from(p[2]) * INV255,
                    f32::from(p[1]) * INV255,
                    f32::from(p[0]) * INV255,
                    1.0,
                ];
            }
        }
        format::R8G8B8 => {
            for (o, p) in out[..w].iter_mut().zip(row.chunks_exact(3)) {
                *o = [
                    f32::from(p[2]) * INV255,
                    f32::from(p[1]) * INV255,
                    f32::from(p[0]) * INV255,
                    1.0,
                ];
            }
        }
        format::A4R4G4B4 => {
            for (o, p) in out[..w].iter_mut().zip(row.chunks_exact(2)) {
                let v = u32::from(u16::from_le_bytes([p[0], p[1]]));
                *o = [
                    ((v >> 8) & 0xf) as f32 * INV15,
                    ((v >> 4) & 0xf) as f32 * INV15,
                    (v & 0xf) as f32 * INV15,
                    (v >> 12) as f32 * INV15,
                ];
            }
        }
        _ => unreachable!("decode_row of an unsupported format"),
    }
}

/// D3DX's ordered dither (0x787590): four rows of eight thresholds, used as
/// `[row & 3][(slice & 3) + (column & 3)]`.
const BAYER: [[f32; 8]; 4] = [
    [
        0.96875, 0.46875, 0.84375, 0.34375, 0.96875, 0.46875, 0.84375, 0.34375,
    ],
    [
        0.21875, 0.71875, 0.09375, 0.59375, 0.21875, 0.71875, 0.09375, 0.59375,
    ],
    [
        0.78125, 0.28125, 0.90625, 0.40625, 0.78125, 0.28125, 0.90625, 0.40625,
    ],
    [
        0.03125, 0.53125, 0.15625, 0.65625, 0.03125, 0.53125, 0.15625, 0.65625,
    ],
];

/// The quantization of one channel: `(c * scale) + threshold` in truncating arithmetic (the
/// encoders set the rounding control to truncate around their loops), truncated, clamped to
/// `0..=max`.
#[inline(always)]
fn quantize(c: f32, scale: f32, t: f32, max: i32) -> u32 {
    let v = fistp((Z(c) * Z(scale) + Z(t)).0);
    v.clamp(0, max) as u32
}

/// One row of the working colour to `f`: row `y` of the surface (the encoders go right to
/// left on odd rows, which picks the threshold of each pixel: the n-th pixel written takes
/// column `n & 3`). `dither`: D3DX's ordered dither, else every threshold is 0.5 (rounding).
#[inline(never)]
pub fn encode_row(f: u32, colors: &[Color], y: u32, dither: bool, out: &mut [u8]) {
    // The encoders truncate in their pixel loops (0x61493f to 0x614c66 and the like).
    let _rz = Rz::new();
    let w = colors.len();
    let bayer_row = &BAYER[(y & 3) as usize];
    for n in 0..w {
        let x = if y & 1 == 0 { n } else { w - 1 - n };
        let t = if dither { bayer_row[n & 3] } else { 0.5 };
        let c = &colors[x];
        match f {
            format::A8R8G8B8 => {
                let v = quantize(c[3], 255.0, t, 255) << 24
                    | quantize(c[0], 255.0, t, 255) << 16
                    | quantize(c[1], 255.0, t, 255) << 8
                    | quantize(c[2], 255.0, t, 255);
                out[x * 4..x * 4 + 4].copy_from_slice(&v.to_le_bytes());
            }
            format::X8R8G8B8 => {
                let v = quantize(c[0], 255.0, t, 255) << 16
                    | quantize(c[1], 255.0, t, 255) << 8
                    | quantize(c[2], 255.0, t, 255);
                out[x * 4..x * 4 + 4].copy_from_slice(&v.to_le_bytes());
            }
            format::A4R4G4B4 => {
                let v = quantize(c[3], 15.0, t, 15) << 12
                    | quantize(c[0], 15.0, t, 15) << 8
                    | quantize(c[1], 15.0, t, 15) << 4
                    | quantize(c[2], 15.0, t, 15);
                out[x * 2..x * 2 + 2].copy_from_slice(&(v as u16).to_le_bytes());
            }
            _ => unreachable!("encode_row of an unsupported format"),
        }
    }
}

/// D3DX's integer box filter for a level half the size of `src` in both directions (8888 and
/// X888: 0x623b32 with MMX, 0x6238f6 without; 4444: 0x62403f): each channel the sum of the four
/// source pixels plus 2, shifted right by 2. `src` is `w` x `h` (both at least 2), rows packed.
pub fn box_integer(f: u32, src: &[u8], w: usize, h: usize) -> Vec<u8> {
    let (w2, h2) = (w / 2, h / 2);
    match f {
        format::A8R8G8B8 | format::X8R8G8B8 => {
            let mut out = vec![0u8; w2 * h2 * 4];
            for y in 0..h2 {
                let r0 = &src[(2 * y) * w * 4..];
                let r1 = &src[(2 * y + 1) * w * 4..];
                for x in 0..w2 {
                    for c in 0..4 {
                        let s = u32::from(r0[8 * x + c])
                            + u32::from(r0[8 * x + 4 + c])
                            + u32::from(r1[8 * x + c])
                            + u32::from(r1[8 * x + 4 + c]);
                        out[(y * w2 + x) * 4 + c] = ((s + 2) >> 2) as u8;
                    }
                }
            }
            out
        }
        format::A4R4G4B4 => {
            let mut out = vec![0u8; w2 * h2 * 2];
            let px = |row: usize, x: usize| -> u32 {
                u32::from(u16::from_le_bytes([
                    src[(row * w + x) * 2],
                    src[(row * w + x) * 2 + 1],
                ]))
            };
            for y in 0..h2 {
                for x in 0..w2 {
                    let p = [
                        px(2 * y, 2 * x),
                        px(2 * y, 2 * x + 1),
                        px(2 * y + 1, 2 * x),
                        px(2 * y + 1, 2 * x + 1),
                    ];
                    let mut v = 0u32;
                    for shift in [0, 4, 8, 12] {
                        let s: u32 = p.iter().map(|q| (q >> shift) & 0xf).sum();
                        v |= ((s + 2) >> 2) << shift;
                    }
                    out[(y * w2 + x) * 2..(y * w2 + x) * 2 + 2]
                        .copy_from_slice(&(v as u16).to_le_bytes());
                }
            }
            out
        }
        _ => unreachable!("box_integer of an unsupported format"),
    }
}

/// Whether D3DX's box filter has an integer path for `f` (and the source is at least 2x2).
pub fn has_integer_box(f: u32) -> bool {
    matches!(f, format::A8R8G8B8 | format::X8R8G8B8 | format::A4R4G4B4)
}

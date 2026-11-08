//! D3DX's DXT block decoder and encoder (`d3dx9_38.dll`: decoders 0x62990d DXT1 colour,
//! 0x629a3d DXT3, 0x629ac7 DXT5; encoders 0x629c16 DXT1, 0x629d52 DXT3, 0x629e94 DXT5, with the
//! colour block encoder 0x628f03, its endpoint search 0x6288d0, the alpha endpoint search
//! 0x62858a and the 5:6:5 conversions 0x628493 / 0x627a50).
//!
//! A block is 16 pixels, row by row, each D3DX's working colour: red, green, blue, alpha as
//! floats. The arithmetic is the DLL's at the thread's precision `P` ([`crate::num::Fpu`]): what
//! it keeps in x87 registers is a [`Reg`] value, what it stores to a float is `.st()` (each one
//! a store in the DLL, at the address noted), in the rounding of the region: to nearest, or
//! truncating in the loops where the DLL sets its rounding control to truncate (`T`). Outside
//! those loops a single operation on stored floats followed by a store rounds the same at
//! either precision, so those stay plain `f32` ([`crate::num`]); inside them every operation is
//! a `T` one.
// Everything here is translated from the DLL. DirectXTex's BC.cpp (D3DX's encoder's
// descendant) was used as a map while reading it, and some names follow it (pC/pD for the
// step weights, the endpoint search's shape); its notice:
//
//   The MIT License (MIT). Copyright (c) Microsoft Corporation.
//   Permission is hereby granted, free of charge, to any person obtaining a copy of this
//   software and associated documentation files (the "Software"), to deal in the Software
//   without restriction, including without limitation the rights to use, copy, modify, merge,
//   publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons
//   to whom the Software is furnished to do so, subject to the following conditions:
//   The above copyright notice and this permission notice shall be included in all copies or
//   substantial portions of the Software.
//   THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED,
//   INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR
//   PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE
//   FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR
//   OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
//   DEALINGS IN THE SOFTWARE.

// The comparisons keep the DLL's NaN behaviour: `!(a < b)` is not `a >= b` for a NaN.
#![allow(clippy::neg_cmp_op_on_partial_ord)]

use crate::num::{
    fistp, Fpu, Precision, Reg, Rz, INV15, INV255, INV31, INV5, INV63, INV7, P24, P53, THIRD,
    TWO_THIRDS, W1, W3, W5, W7,
};

/// D3DX's working colour: red, green, blue, alpha.
pub type Color = [f32; 4];

/// A register value rounding to nearest at precision `P`.
type N<P> = <P as Fpu>::N;
/// A register value truncating at precision `P`.
type T<P> = <P as Fpu>::T;

/// The weights D3DX gives red and blue against green while it fits a colour block (0.2125 and
/// 0.0721 over 0.7154; the DLL's floats at 0x787e44..) and their inverses (0x787e54..).
#[inline(always)]
fn lum() -> [f32; 3] {
    [
        f32::from_bits(0x3e98_1530),
        1.0,
        f32::from_bits(0x3dce_6734),
    ]
}
#[inline(always)]
fn lum_inv() -> [f32; 3] {
    [
        f32::from_bits(0x4057_762e),
        1.0,
        f32::from_bits(0x411e_c1dd),
    ]
}

/// 0x627a50: a 5:6:5 colour to the working colour (one multiply per channel: the same at
/// either precision).
#[inline]
pub fn decode565(w: u16) -> Color {
    let w = u32::from(w);
    [
        ((w >> 11) & 31) as f32 * INV31,
        ((w >> 5) & 63) as f32 * INV63,
        (w & 31) as f32 * INV31,
        1.0,
    ]
}

/// The clamp of 0x628493 and 0x62858a: below 0 gives 0, above 1 gives 1, anything else (a NaN
/// too) is kept.
#[inline(always)]
fn clamp01(x: f32) -> f32 {
    x.clamp(0.0, 1.0)
}

/// `trunc(x * scale + 0.5)`, the encoders' quantization (inside an [`Rz`] region): the sum is
/// stored to a float, which `FISTP` converts (0x628534, 0x6290b6, 0x629dc3, 0x629f0c, ...).
#[inline(always)]
fn quant<R: Reg>(x: f32, scale: f32) -> i32 {
    fistp((R::ld(x) * R::ld(scale) + R::ld(0.5)).st())
}

/// [`quant`] of three values in their own truncating region (0x62851c to 0x628576, and the
/// DXT5 endpoints' 0x62a06e to 0x62a0ad).
#[inline(never)]
fn quant3<P: Fpu>(v: [f32; 3], scale: [f32; 3]) -> [i32; 3] {
    let _rz = Rz::new();
    [
        quant::<T<P>>(v[0], scale[0]),
        quant::<T<P>>(v[1], scale[1]),
        quant::<T<P>>(v[2], scale[2]),
    ]
}

/// 0x628493: the working colour to 5:6:5.
#[inline]
fn encode565_p<P: Fpu>(c: &[f32; 3]) -> u16 {
    let [r, g, b] = quant3::<P>(
        [clamp01(c[0]), clamp01(c[1]), clamp01(c[2])],
        [31.0, 63.0, 31.0],
    );
    (((r << 6 | g) << 5) | b) as u16
}

/// 0x628493 at single precision.
pub fn encode565(c: &[f32; 3]) -> u16 {
    encode565_p::<P24>(c)
}

/// `c0 + (c1 - c0) * t` for each channel, the difference and product in registers, as the
/// decoder (0x629956..0x629a08) and encoder (0x6294a5..0x629590) make their palettes.
#[inline(always)]
fn lerp<P: Fpu>(c0: &[f32], c1: &[f32], t: f32) -> [f32; 4] {
    let mut o = [0.0f32; 4];
    for (k, v) in o.iter_mut().enumerate().take(c0.len()) {
        let (a, b) = (N::<P>::ld(c0[k]), N::<P>::ld(c1[k]));
        *v = (a + (b - a) * N::<P>::ld(t)).st();
    }
    o
}

/// 0x62990d: a DXT1 colour block (also the colour half of DXT2..5: D3DX decodes those with the
/// DXT1 rule, three colours and transparent black when the first endpoint is not the larger).
pub(crate) fn decode_color_p<P: Fpu>(b: &[u8]) -> [Color; 16] {
    let w0 = u16::from_le_bytes([b[0], b[1]]);
    let w1 = u16::from_le_bytes([b[2], b[3]]);
    let c0 = decode565(w0);
    let c1 = decode565(w1);
    let mut pal = [c0, c1, [0.0; 4], [0.0; 4]];
    if w0 > w1 {
        pal[2] = lerp::<P>(&c0, &c1, THIRD);
        pal[3] = lerp::<P>(&c0, &c1, TWO_THIRDS);
    } else {
        pal[2] = lerp::<P>(&c0, &c1, 0.5);
    }
    let idx = u32::from_le_bytes([b[4], b[5], b[6], b[7]]);
    std::array::from_fn(|i| pal[((idx >> (2 * i)) & 3) as usize])
}

/// 0x629a3d: a DXT2/DXT3 block (explicit 4-bit alpha, then the colour block).
pub(crate) fn decode_dxt3_p<P: Fpu>(b: &[u8]) -> [Color; 16] {
    let mut px = decode_color_p::<P>(&b[8..16]);
    for half in 0..2 {
        let mut a = u32::from_le_bytes(b[half * 4..half * 4 + 4].try_into().unwrap());
        for i in 0..8 {
            px[half * 8 + i][3] = (a & 0xf) as f32 * INV15;
            a >>= 4;
        }
    }
    px
}

/// The eight alpha values of a DXT4/DXT5 alpha block (0x629b4d, 0x629b91), also the encoder's
/// steps (0x62a136, 0x62a19c): `s[i + 1] = (i s1 + (n - 1 - i) s0) / (n - 1)` (times the DLL's
/// 1/7 or 1/5), in registers until the store; then for six steps 0 and 1.
fn alpha_steps<P: Fpu>(s0: f32, s1: f32, n: u32) -> [f32; 8] {
    let mut s = [0.0f32; 8];
    s[0] = s0;
    s[1] = s1;
    let inv = N::<P>::ld(if n == 8 { INV7 } else { INV5 });
    for i in 1..n - 1 {
        let v = N::<P>::int(i64::from(i)) * N::<P>::ld(s1)
            + N::<P>::int(i64::from(n - 1 - i)) * N::<P>::ld(s0);
        s[i as usize + 1] = (v * inv).st();
    }
    if n == 6 {
        s[6] = 0.0;
        s[7] = 1.0;
    }
    s
}

/// 0x629ac7: a DXT4/DXT5 block (interpolated alpha, then the colour block).
pub(crate) fn decode_dxt5_p<P: Fpu>(b: &[u8]) -> [Color; 16] {
    let mut px = decode_color_p::<P>(&b[8..16]);
    let a0 = f32::from(b[0]) * INV255;
    let a1 = INV255 * f32::from(b[1]);
    let pal = alpha_steps::<P>(a0, a1, if b[1] < b[0] { 8 } else { 6 });
    for half in 0..2 {
        let o = 2 + half * 3;
        let mut bits = u32::from(b[o]) | u32::from(b[o + 1]) << 8 | u32::from(b[o + 2]) << 16;
        for i in 0..8 {
            px[half * 8 + i][3] = pal[(bits & 7) as usize];
            bits >>= 3;
        }
    }
    px
}

/// A DXT1 colour block decoded at `precision` ([`decode_color_p`]).
pub fn decode_color(b: &[u8], precision: Precision) -> [Color; 16] {
    match precision {
        Precision::Single => decode_color_p::<P24>(b),
        Precision::Double => decode_color_p::<P53>(b),
    }
}

/// A DXT2/DXT3 block decoded at `precision`.
pub fn decode_dxt3(b: &[u8], precision: Precision) -> [Color; 16] {
    match precision {
        Precision::Single => decode_dxt3_p::<P24>(b),
        Precision::Double => decode_dxt3_p::<P53>(b),
    }
}

/// A DXT4/DXT5 block decoded at `precision`.
pub fn decode_dxt5(b: &[u8], precision: Precision) -> [Color; 16] {
    match precision {
        Precision::Single => decode_dxt5_p::<P24>(b),
        Precision::Double => decode_dxt5_p::<P53>(b),
    }
}

/// The colour lookup tables of the endpoint search (0x4080d4.., 0x4080f4..): for three steps
/// `pC3`, `pD3`; for four `pC4`, `pD4`.
const PC3: [f32; 3] = [1.0, 0.5, 0.0];
const PD3: [f32; 3] = [0.0, 0.5, 1.0];
const PC4: [f32; 4] = [1.0, 2.0 / 3.0, 1.0 / 3.0, 0.0];
const PD4: [f32; 4] = [0.0, 1.0 / 3.0, 2.0 / 3.0, 1.0];

/// 0x6288d0: the colour endpoints of a block of quantized, weighted colours (D3DX's search: the
/// box's corners, the diagonal that fits best, then up to eight Newton steps with truncating
/// arithmetic). `steps` is 3 or 4.
fn optimize_rgb_p<P: Fpu>(points: &[Color; 16], steps: usize) -> ([f32; 3], [f32; 3]) {
    let ld = N::<P>::ld;
    let sq = |v: f32| ld(v) * ld(v);
    let mut x = lum();
    let mut y = [0.0f32; 3];
    for p in points {
        for c in 0..3 {
            if p[c] < x[c] {
                x[c] = p[c];
            }
            if y[c] < p[c] {
                y[c] = p[c];
            }
        }
    }
    // 0x6289b2..0x6289ea.
    let ab: [f32; 3] = std::array::from_fn(|c| y[c] - x[c]);
    let f_ab = (sq(ab[2]) + (sq(ab[0]) + sq(ab[1]))).st();
    if f_ab < f32::MIN_POSITIVE {
        return (x, y);
    }
    // 0x628a27..0x628a6a.
    let inv = 1.0 / f_ab;
    let dir = [inv * ab[0], inv * ab[1], inv * ab[2]];
    let mid: [f32; 3] = std::array::from_fn(|c| ((ld(y[c]) + ld(x[c])) * ld(0.5)).st());
    let mut f_dir = [0.0f32; 4];
    for p in points {
        // 0x628a84..0x628afd: each projection stored, the sums in registers.
        let pt: [f32; 3] = std::array::from_fn(|c| ((ld(p[c]) - ld(mid[c])) * ld(dir[c])).st());
        let gr = ld(pt[1]) + ld(pt[0]);
        let rg = ld(pt[0]) - ld(pt[1]);
        let p2 = ld(pt[2]);
        let f = [
            (p2 + gr).st(),
            (gr - p2).st(),
            (p2 + rg).st(),
            (rg - p2).st(),
        ];
        for k in 0..4 {
            f_dir[k] = (sq(f[k]) + ld(f_dir[k])).st();
        }
    }
    let mut best = f_dir[0];
    let mut i_best = 0;
    for (i, &v) in f_dir.iter().enumerate().skip(1) {
        if best < v {
            best = v;
            i_best = i;
        }
    }
    if i_best & 2 != 0 {
        std::mem::swap(&mut x[1], &mut y[1]);
    }
    if i_best & 1 != 0 {
        std::mem::swap(&mut x[2], &mut y[2]);
    }
    if f_ab < 1.0 / 4096.0 {
        return (x, y);
    }
    newton_rgb::<P>(points, x, y, steps)
}

/// The endpoint search at `precision` ([`optimize_rgb_p`]).
pub fn optimize_rgb(
    points: &[Color; 16],
    steps: usize,
    precision: Precision,
) -> ([f32; 3], [f32; 3]) {
    match precision {
        Precision::Single => optimize_rgb_p::<P24>(points, steps),
        Precision::Double => optimize_rgb_p::<P53>(points, steps),
    }
}

/// The Newton steps of the endpoint search (truncating, 0x628bb8 to 0x628ec9). What the DLL
/// stores to floats is kept here in registers rounded as the store rounds ([`Reg::round`]).
#[inline(never)]
fn newton_rgb<P: Fpu>(
    points: &[Color; 16],
    x: [f32; 3],
    y: [f32; 3],
    steps: usize,
) -> ([f32; 3], [f32; 3]) {
    let _rz = Rz::new();
    let ld = T::<P>::ld;
    let (pc, pd): (&[f32], &[f32]) = if steps == 3 {
        (&PC3, &PD3)
    } else {
        (&PC4, &PD4)
    };
    let pc: [T<P>; 4] = std::array::from_fn(|i| ld(pc.get(i).copied().unwrap_or(0.0)));
    let pd: [T<P>; 4] = std::array::from_fn(|i| ld(pd.get(i).copied().unwrap_or(0.0)));
    let pts: [[T<P>; 3]; 16] = std::array::from_fn(|i| std::array::from_fn(|c| ld(points[i][c])));
    let (zero, half, eighth, minus_one) = (ld(0.0), ld(0.5), ld(0.125), ld(-1.0));
    let f_steps = ld((steps - 1) as f32);
    let mut x = x.map(ld);
    let mut y = y.map(ld);
    for _ in 0..8 {
        // 0x628bf1..0x628c13.
        let mut s = [[zero; 3]; 4];
        for i in 0..steps {
            for c in 0..3 {
                s[i][c] = (y[c] * pd[i] + pc[i] * x[c]).round();
            }
        }
        // 0x628c21..0x628cac.
        let dv: [T<P>; 3] = std::array::from_fn(|c| (y[c] - x[c]).round());
        let len = (dv[2] * dv[2] + (dv[0] * dv[0] + dv[1] * dv[1])).round();
        if len < ld(1.0 / 4096.0) {
            break;
        }
        let scale = (f_steps / len).round();
        let dir: [T<P>; 3] = std::array::from_fn(|c| (scale * dv[c]).round());
        let (mut d2x, mut d2y) = (zero, zero);
        let (mut dx, mut dy) = ([zero; 3], [zero; 3]);
        for p in &pts {
            // 0x628ceb: the projection stored, then compared and converted (0x628d12).
            let d = |c: usize| (p[c] - x[c]) * dir[c];
            let dot = (d(2) + (d(0) + d(1))).round();
            let i = if f_steps > dot {
                (dot + half).round().trunc()
            } else {
                steps as i32 - 1
            };
            let i = step_index(i);
            // 0x628d40..0x628ddb.
            let diff: [T<P>; 3] = std::array::from_fn(|c| (s[i][c] - p[c]).round());
            let fc = (pc[i] * eighth).round();
            let fd = (eighth * pd[i]).round();
            d2x = (d2x + fc * pc[i]).round();
            for c in 0..3 {
                dx[c] = (fc * diff[c] + dx[c]).round();
            }
            d2y = (fd * pd[i] + d2y).round();
            for c in 0..3 {
                dy[c] = (fd * diff[c] + dy[c]).round();
            }
        }
        // 0x628df7..0x628e55.
        if zero < d2x {
            let f = (minus_one / d2x).round();
            for c in 0..3 {
                x[c] = (f * dx[c] + x[c]).round();
            }
        }
        if zero < d2y {
            let f = (minus_one / d2y).round();
            for c in 0..3 {
                y[c] = (f * dy[c] + y[c]).round();
            }
        }
        // 0x628e5f..0x628eaf: the squares compared in registers.
        let eps = ld((0.25 / 64.0) * (0.25 / 64.0));
        if dx.iter().chain(dy.iter()).all(|&v| v * v < eps) {
            break;
        }
    }
    (x.map(|v| v.st()), y.map(|v| v.st()))
}

/// Counts the step indices the endpoint search's Newton loop takes below 0 (a point projecting
/// more than half a step before the first endpoint). The DLL then reads memory before its tables;
/// none of the game's textures does this (the verification tool checks the count stays 0).
pub static NEGATIVE_STEPS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

#[inline(always)]
fn step_index(i: i32) -> usize {
    if i < 0 {
        NEGATIVE_STEPS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        0
    } else {
        i as usize
    }
}

/// Floyd-Steinberg error diffusion within a 4x4 block (the DLL's encoders, pixel by pixel in
/// row order, truncating): `err[i + k] = err[i + k] + e * w`, each stored (0x6291ad.., 0x629cd0..,
/// 0x629e25.., 0x629f80.., 0x62a314..).
#[inline(always)]
fn diffuse<P: Fpu, const N: usize>(err: &mut [[f32; N]; 16], i: usize, e: [f32; N]) {
    let ld = T::<P>::ld;
    let mut add = |k: usize, w: f32| {
        for c in 0..N {
            err[k][c] = (ld(err[k][c]) + ld(e[c]) * ld(w)).st();
        }
    };
    if i & 3 != 3 {
        add(i + 1, W7);
    }
    if i < 12 {
        if i & 3 != 0 {
            add(i + 3, W3);
        }
        add(i + 4, W5);
        if i & 3 != 3 {
            add(i + 5, W1);
        }
    }
}

/// The quantization of a colour block to 5:6:5 with error diffusion when dithering
/// (truncating, 0x628f94 to 0x6292c8): each pixel's quantized colour weighted by `lum`, alpha 1.
#[inline(never)]
fn quantize_colors<P: Fpu>(c: &[Color; 16], dither: bool, lum: [f32; 3]) -> [Color; 16] {
    let _rz = Rz::new();
    let ld = T::<P>::ld;
    let mut err = [[0.0f32; 3]; 16];
    let mut color = [[0.0f32; 4]; 16];
    for i in 0..16 {
        let mut v = [c[i][0], c[i][1], c[i][2]];
        if dither {
            // 0x62908e..0x6290a4.
            for k in 0..3 {
                v[k] = (ld(err[i][k]) + ld(v[k])).st();
            }
        }
        // 0x6290b6..0x62914f.
        let q = [
            (T::<P>::int(quant::<T<P>>(v[0], 31.0).into()) * ld(INV31)).st(),
            (T::<P>::int(quant::<T<P>>(v[1], 63.0).into()) * ld(INV63)).st(),
            (T::<P>::int(quant::<T<P>>(v[2], 31.0).into()) * ld(INV31)).st(),
        ];
        if dither {
            // 0x629175..0x629187.
            let e = [
                (ld(v[0]) - ld(q[0])).st(),
                (ld(v[1]) - ld(q[1])).st(),
                (ld(v[2]) - ld(q[2])).st(),
            ];
            diffuse::<P, 3>(&mut err, i, e);
        }
        // 0x629296..0x6292b6.
        color[i] = [
            (ld(q[0]) * ld(lum[0])).st(),
            (ld(q[1]) * ld(lum[1])).st(),
            (ld(q[2]) * ld(lum[2])).st(),
            1.0,
        ];
    }
    color
}

/// What the colour block's index loop needs: the palette (`s`, weighted), its first colour,
/// the direction scaled to steps, the step count and its index table.
struct Palette<'a> {
    s: [[f32; 4]; 4],
    dir: [f32; 3],
    steps: usize,
    p_steps: &'a [u32],
}

/// The colour block's indices (truncating, 0x629636 to 0x6298f3).
#[inline(never)]
fn color_indices<P: Fpu>(c: &[Color; 16], pal: &Palette, dither: bool, lum: [f32; 3]) -> u32 {
    let _rz = Rz::new();
    let ld = T::<P>::ld;
    let s0 = pal.s[0];
    let dir = pal.dir;
    let f_steps = (pal.steps - 1) as f32;
    let mut err = [[0.0f32; 3]; 16];
    let mut bitmap = 0u32;
    for i in 0..16 {
        if pal.steps == 3 && c[i][3] < 0.5 {
            bitmap = (bitmap >> 2) | (3 << 30);
            continue;
        }
        // 0x6296cb..0x629707.
        let mut cl = [
            (ld(c[i][0]) * ld(lum[0])).st(),
            (ld(lum[1]) * ld(c[i][1])).st(),
            (ld(c[i][2]) * ld(lum[2])).st(),
        ];
        if dither {
            cl = [
                (ld(cl[0]) + ld(err[i][0])).st(),
                (ld(err[i][1]) + ld(cl[1])).st(),
                (ld(err[i][2]) + ld(cl[2])).st(),
            ];
        }
        // 0x629732, then compared and converted (0x629762).
        let d = |k: usize| (ld(cl[k]) - ld(s0[k])) * ld(dir[k]);
        let dot = (d(2) + (d(0) + d(1))).st();
        let step = if !(0.0 < dot) {
            0
        } else if !(f_steps > dot) {
            1
        } else {
            pal.p_steps[fistp((ld(dot) + ld(0.5)).st()) as usize]
        };
        bitmap = (step << 30) | (bitmap >> 2);
        if dither {
            // 0x6297aa..0x6297d0: the error weighted by the quantized colour's alpha, 1.
            let t = &pal.s[step as usize];
            let e: [f32; 3] = std::array::from_fn(|k| ((ld(cl[k]) - ld(t[k])) * ld(1.0)).st());
            diffuse::<P, 3>(&mut err, i, e);
        }
    }
    bitmap
}

/// 0x628f03: a colour block. `dxt1`: a DXT1 block, which may use three colours and transparent
/// black for pixels with alpha below 0.5; `dither`: D3DX's error diffusion (filters with
/// `D3DX_FILTER_DITHER`).
fn encode_color<P: Fpu>(c: &[Color; 16], dxt1: bool, dither: bool) -> [u8; 8] {
    let mut steps = 4usize;
    if dxt1 {
        let n = c.iter().filter(|p| p[3] < 0.5).count();
        if n == 16 {
            return [0, 0, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff];
        }
        if n > 0 {
            steps = 3;
        }
    }
    let lum = lum();
    let color = quantize_colors::<P>(c, dither, lum);
    let (a, b) = optimize_rgb_p::<P>(&color, steps);
    // 0x6292fc..0x629354.
    let inv = lum_inv();
    let wa = encode565_p::<P>(&[inv[0] * a[0], inv[1] * a[1], inv[2] * a[2]]);
    let wb = encode565_p::<P>(&[b[0] * inv[0], inv[1] * b[1], inv[2] * b[2]]);
    if steps == 4 && wa == wb {
        let mut out = [0u8; 8];
        out[0..2].copy_from_slice(&wa.to_le_bytes());
        out[2..4].copy_from_slice(&wb.to_le_bytes());
        return out;
    }
    let ca = decode565(wa);
    let cb = decode565(wb);
    let a = [lum[0] * ca[0], lum[1] * ca[1], lum[2] * ca[2]];
    let b = [lum[0] * cb[0], lum[1] * cb[1], lum[2] * cb[2]];
    let (w0, w1, s0, s1) = if (steps == 3) == (wa <= wb) {
        (wa, wb, a, b)
    } else {
        (wb, wa, b, a)
    };
    let mut s = [[0.0f32; 4]; 4];
    s[0] = [s0[0], s0[1], s0[2], 0.0];
    s[1] = [s1[0], s1[1], s1[2], 0.0];
    let p_steps: &[u32] = if steps == 3 {
        s[2] = lerp::<P>(&s0, &s1, 0.5);
        &[0, 2, 1]
    } else {
        s[2] = lerp::<P>(&s0, &s1, THIRD);
        s[3] = lerp::<P>(&s0, &s1, TWO_THIRDS);
        &[0, 2, 3, 1]
    };
    // 0x629596..0x6295e2: the differences stored, the scale's sum in registers.
    let dv = [s1[0] - s0[0], s1[1] - s0[1], s1[2] - s0[2]];
    let f_steps = (steps - 1) as f32;
    let scale = if wa == wb {
        0.0
    } else {
        let ld = N::<P>::ld;
        let sq = |v: f32| ld(v) * ld(v);
        (ld(f_steps) / ((sq(dv[0]) + sq(dv[1])) + sq(dv[2]))).st()
    };
    let pal = Palette {
        s,
        dir: [scale * dv[0], scale * dv[1], scale * dv[2]],
        steps,
        p_steps,
    };
    let bitmap = color_indices::<P>(c, &pal, dither, lum);
    let mut out = [0u8; 8];
    out[0..2].copy_from_slice(&w0.to_le_bytes());
    out[2..4].copy_from_slice(&w1.to_le_bytes());
    out[4..8].copy_from_slice(&bitmap.to_le_bytes());
    out
}

/// The DXT1 encoder's alpha dither: each alpha to 0 or 1 with error diffusion (truncating,
/// 0x629c4c to 0x629d25). The sum is stored (0x629c74) but rounded from its register (0x629c88).
#[inline(never)]
fn dither_alpha_dxt1<P: Fpu>(c: &[Color; 16]) -> [Color; 16] {
    let _rz = Rz::new();
    let ld = T::<P>::ld;
    let mut q = *c;
    let mut err = [[0.0f32; 1]; 16];
    for i in 0..16 {
        let sum = ld(err[i][0]) + ld(c[i][3]);
        let a = sum.st();
        let v = T::<P>::int(fistp((sum + ld(0.5)).st()).into()).st();
        q[i][3] = v;
        diffuse::<P, 1>(&mut err, i, [(ld(a) - ld(v)).st()]);
    }
    q
}

/// 0x629c16: a DXT1 block. With dither the alpha is first quantized to 0 or 1 with error
/// diffusion.
pub(crate) fn encode_dxt1_p<P: Fpu>(c: &[Color; 16], dither: bool) -> [u8; 8] {
    if !dither {
        return encode_color::<P>(c, true, false);
    }
    encode_color::<P>(&dither_alpha_dxt1::<P>(c), true, true)
}

/// The DXT3 encoder's 4-bit alphas (truncating, 0x629d89 to 0x629e75).
#[inline(never)]
fn alpha_dxt3<P: Fpu>(c: &[Color; 16], dither: bool) -> [u32; 2] {
    let _rz = Rz::new();
    let ld = T::<P>::ld;
    let mut dw = [0u32; 2];
    let mut err = [[0.0f32; 1]; 16];
    for i in 0..16 {
        let mut a = c[i][3];
        if dither {
            a = (ld(err[i][0]) + ld(a)).st();
        }
        let q = quant::<T<P>>(a, 15.0);
        dw[i >> 3] = (q as u32).wrapping_shl(28) | (dw[i >> 3] >> 4);
        if dither {
            // 0x629e0c: FILD of the unsigned value (a negative one gets 2^32 added), times
            // 1/15, subtracted, all in registers.
            let qf = if q < 0 {
                T::<P>::int(q.into()) + ld(4_294_967_296.0)
            } else {
                T::<P>::int(q.into())
            };
            diffuse::<P, 1>(&mut err, i, [(ld(a) - qf * ld(INV15)).st()]);
        }
    }
    dw
}

/// 0x629d52: a DXT2/DXT3 block: explicit 4-bit alpha (with error diffusion when dithering), then
/// the colour block.
pub(crate) fn encode_dxt3_p<P: Fpu>(c: &[Color; 16], dither: bool) -> [u8; 16] {
    let dw = alpha_dxt3::<P>(c, dither);
    let mut out = [0u8; 16];
    out[0..4].copy_from_slice(&dw[0].to_le_bytes());
    out[4..8].copy_from_slice(&dw[1].to_le_bytes());
    out[8..16].copy_from_slice(&encode_color::<P>(c, false, dither));
    out
}

/// The alpha lookup tables of 0x62858a: `pC6`/`pD6` (0x4080b8, 0x4080a0) and `pC8`/`pD8`
/// (0x408080, 0x408060). `pD8`'s last value is 8/7 in the DLL, not 1.
fn alpha_tables(steps: usize) -> ([f32; 8], [f32; 8]) {
    if steps == 6 {
        (
            [1.0, 0.8, 0.6, 0.4, 0.2, 0.0, 0.0, 0.0],
            [0.0, 0.2, 0.4, 0.6, 0.8, 1.0, 0.0, 0.0],
        )
    } else {
        (
            [
                1.0,
                6.0 / 7.0,
                5.0 / 7.0,
                4.0 / 7.0,
                3.0 / 7.0,
                2.0 / 7.0,
                1.0 / 7.0,
                0.0,
            ],
            [
                0.0,
                1.0 / 7.0,
                2.0 / 7.0,
                3.0 / 7.0,
                4.0 / 7.0,
                5.0 / 7.0,
                6.0 / 7.0,
                8.0 / 7.0,
            ],
        )
    }
}

/// 0x62858a: the alpha endpoints of a block of quantized alphas, `steps` 6 or 8.
fn optimize_alpha<P: Fpu>(points: &[f32; 16], steps: usize) -> (f32, f32) {
    let mut x = 1.0f32;
    let mut y = 0.0f32;
    if steps == 8 {
        for &p in points {
            if p < x {
                x = p;
            }
            if y < p {
                y = p;
            }
        }
    } else {
        for &p in points {
            if p < x && 0.0 < p {
                x = p;
            }
            if y < p && p < 1.0 {
                y = p;
            }
        }
        if x == y {
            y = 1.0;
        }
    }
    let (x, y) = newton_alpha::<P>(points, x, y, steps);
    (clamp01(x), clamp01(y))
}

/// The Newton steps of the alpha endpoint search (truncating, 0x628680 to 0x628864), the stored
/// values kept in registers rounded as the stores round ([`Reg::round`]).
#[inline(never)]
fn newton_alpha<P: Fpu>(points: &[f32; 16], x: f32, y: f32, steps: usize) -> (f32, f32) {
    let _rz = Rz::new();
    let ld = T::<P>::ld;
    let (pc, pd) = alpha_tables(steps);
    let (pc, pd) = (pc.map(ld), pd.map(ld));
    let pts = points.map(ld);
    let (zero, half, one) = (ld(0.0), ld(0.5), ld(1.0));
    let f_steps = ld((steps - 1) as f32);
    let (mut x, mut y) = (ld(x), ld(y));
    for _ in 0..8 {
        // 0x62869b..0x6286ab: the difference in a register.
        let span = y - x;
        if span < ld(1.0 / 256.0) {
            break;
        }
        let scale = (f_steps / span).round();
        let mut s = [zero; 8];
        for i in 0..steps {
            s[i] = (pc[i] * x + pd[i] * y).round();
        }
        if steps == 6 {
            s[6] = zero;
            s[7] = one;
        }
        let (mut dx, mut dy, mut d2x, mut d2y) = (zero, zero, zero, zero);
        // 0x62871c, 0x628758.
        let lo = (x * half).round();
        let hi = ((y + one) * half).round();
        for &p in &pts {
            // 0x6286ff, then compared and converted (0x628777).
            let dot = ((p - x) * scale).round();
            let i = if !(zero < dot) {
                if steps == 6 && !(lo < p) {
                    continue;
                }
                0
            } else if f_steps > dot {
                (dot + half).round().trunc() as usize
            } else {
                if steps == 6 && hi <= p {
                    continue;
                }
                steps - 1
            };
            if i < steps {
                // 0x628795..0x6287c7.
                let diff = (p - s[i]).round();
                dx = (diff * pc[i] + dx).round();
                d2x = (pc[i] * pc[i] + d2x).round();
                dy = (pd[i] * diff + dy).round();
                d2y = (pd[i] * pd[i] + d2y).round();
            }
        }
        // 0x6287e9, 0x628801: the quotient in a register.
        if zero < d2x {
            x = (x - dx / d2x).round();
        }
        if zero < d2y {
            y = (y - dy / d2y).round();
        }
        if y < x {
            std::mem::swap(&mut x, &mut y);
        }
        // 0x628832, 0x628844: the squares in registers.
        if dx * dx < ld(1.0 / 64.0) && dy * dy < ld(1.0 / 64.0) {
            break;
        }
    }
    (x.st(), y.st())
}

/// The DXT5 encoder's alphas to 8 bits (truncating, 0x629edc to 0x629fd8): the quantized
/// alphas and their smallest and largest (both starting from the first pixel's alpha as given).
#[inline(never)]
fn alpha_dxt5<P: Fpu>(c: &[Color; 16], dither: bool) -> ([f32; 16], f32, f32) {
    let _rz = Rz::new();
    let ld = T::<P>::ld;
    let mut lo = c[0][3];
    let mut hi = c[0][3];
    let mut q = [0.0f32; 16];
    let mut err = [[0.0f32; 1]; 16];
    for i in 0..16 {
        let mut a = c[i][3];
        if dither {
            a = (ld(err[i][0]) + ld(a)).st();
        }
        let v = (T::<P>::int(quant::<T<P>>(a, 255.0).into()) * ld(INV255)).st();
        q[i] = v;
        if v < lo {
            lo = v;
        } else if hi < v {
            hi = v;
        }
        if dither {
            diffuse::<P, 1>(&mut err, i, [(ld(a) - ld(v)).st()]);
        }
    }
    (q, lo, hi)
}

/// The DXT5 encoder's alpha indices (truncating, 0x62a208 to 0x62a38e): two 24-bit words.
#[inline(never)]
fn alpha_indices_dxt5<P: Fpu>(
    c: &[Color; 16],
    s: &[f32; 8],
    scale: f32,
    steps: u32,
    p_steps: &[u32],
    dither: bool,
) -> [u32; 2] {
    let _rz = Rz::new();
    let ld = T::<P>::ld;
    let f_steps = (steps - 1) as f32;
    // 0x62a276, 0x62a2af: both thresholds in registers.
    let (lo_t, hi_t) = (ld(s[0]) * ld(0.5), (ld(s[1]) + ld(1.0)) * ld(0.5));
    let mut err = [[0.0f32; 1]; 16];
    let mut out = [0u32; 2];
    for (set, word) in out.iter_mut().enumerate() {
        let mut dw = 0u32;
        for i in set * 8..set * 8 + 8 {
            let mut a = c[i][3];
            if dither {
                a = (ld(err[i][0]) + ld(a)).st();
            }
            // 0x62a255, then compared and converted (0x62a2ca).
            let dot = ((ld(a) - ld(s[0])) * ld(scale)).st();
            let step = if !(0.0 < dot) {
                if steps == 6 && !(lo_t < ld(a)) {
                    6
                } else {
                    0
                }
            } else if f_steps > dot {
                p_steps[fistp((ld(dot) + ld(0.5)).st()) as usize]
            } else if steps == 6 && hi_t <= ld(a) {
                7
            } else {
                1
            };
            dw = (dw >> 3) | (step << 21);
            if dither {
                diffuse::<P, 1>(&mut err, i, [(ld(a) - ld(s[step as usize])).st()]);
            }
        }
        *word = dw;
    }
    out
}

/// 0x629e94: a DXT4/DXT5 block: interpolated alpha, then the colour block.
pub(crate) fn encode_dxt5_p<P: Fpu>(c: &[Color; 16], dither: bool) -> [u8; 16] {
    let mut out = [0u8; 16];
    let (q, lo, hi) = alpha_dxt5::<P>(c, dither);
    out[8..16].copy_from_slice(&encode_color::<P>(c, false, dither));
    if lo == 1.0 {
        out[0] = 0xff;
        out[1] = 0xff;
        return out;
    }
    let steps: u32 = if lo == 0.0 || hi == 1.0 { 6 } else { 8 };
    let (fa, fb) = optimize_alpha::<P>(&q, steps as usize);
    let [ba, bb, _] = quant3::<P>([fa, fb, 0.0], [255.0, 255.0, 0.0]);
    let fa = (ba & 0xff) as f32 * INV255;
    let fb = (bb & 0xff) as f32 * INV255;
    let (ca, cb) = (ba as u8, bb as u8);
    let (s, p_steps): ([f32; 8], &[u32]) = if steps == 8 {
        if ca == cb {
            out[0] = ca;
            out[1] = cb;
            return out;
        }
        out[1] = ca;
        out[0] = cb;
        (alpha_steps::<P>(fb, fa, 8), &[0, 2, 3, 4, 5, 6, 7, 1])
    } else {
        out[0] = ca;
        out[1] = cb;
        (alpha_steps::<P>(fa, fb, 6), &[0, 2, 3, 4, 5, 1])
    };
    // 0x62a1cf..0x62a1e2: the difference and quotient in registers.
    let f_steps = (steps - 1) as f32;
    let scale = if s[1] == s[0] {
        0.0
    } else {
        let ld = N::<P>::ld;
        (ld(f_steps) / (ld(s[1]) - ld(s[0]))).st()
    };
    let words = alpha_indices_dxt5::<P>(c, &s, scale, steps, p_steps, dither);
    for (set, dw) in words.iter().enumerate() {
        out[2 + set * 3..5 + set * 3].copy_from_slice(&dw.to_le_bytes()[..3]);
    }
    out
}

/// A DXT1 block at `precision` ([`encode_dxt1_p`]).
pub fn encode_dxt1(c: &[Color; 16], dither: bool, precision: Precision) -> [u8; 8] {
    match precision {
        Precision::Single => encode_dxt1_p::<P24>(c, dither),
        Precision::Double => encode_dxt1_p::<P53>(c, dither),
    }
}

/// A DXT2/DXT3 block at `precision`.
pub fn encode_dxt3(c: &[Color; 16], dither: bool, precision: Precision) -> [u8; 16] {
    match precision {
        Precision::Single => encode_dxt3_p::<P24>(c, dither),
        Precision::Double => encode_dxt3_p::<P53>(c, dither),
    }
}

/// A DXT4/DXT5 block at `precision`.
pub fn encode_dxt5(c: &[Color; 16], dither: bool, precision: Precision) -> [u8; 16] {
    match precision {
        Precision::Single => encode_dxt5_p::<P24>(c, dither),
        Precision::Double => encode_dxt5_p::<P53>(c, dither),
    }
}

/// The generic forms, for the per-level code ([`crate::filter`]).
pub(crate) mod generic {
    pub(crate) use super::{
        decode_color_p as decode_color, decode_dxt3_p as decode_dxt3, decode_dxt5_p as decode_dxt5,
        encode_dxt1_p as encode_dxt1, encode_dxt3_p as encode_dxt3, encode_dxt5_p as encode_dxt5,
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode565_and_decode565_round_trip() {
        for w in [0u16, 0xffff, 0x1234, 0xf81f, 0x07e0] {
            let c = decode565(w);
            assert_eq!(encode565(&[c[0], c[1], c[2]]), w);
        }
    }

    #[test]
    fn a_transparent_dxt1_block_is_the_dlls_constant() {
        let px = [[0.5, 0.5, 0.5, 0.0]; 16];
        assert_eq!(
            encode_dxt1(&px, false, Precision::Single),
            [0, 0, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]
        );
    }

    /// The lookup tables are the DLL's floats (0x4080d4.., 0x408060..).
    #[test]
    fn tables_are_the_dlls() {
        let bits = |v: &[f32]| v.iter().map(|x| x.to_bits()).collect::<Vec<_>>();
        assert_eq!(bits(&PD4), [0, 0x3eaa_aaab, 0x3f2a_aaab, 0x3f80_0000]);
        assert_eq!(bits(&PC4), [0x3f80_0000, 0x3f2a_aaab, 0x3eaa_aaab, 0]);
        let (pc8, pd8) = alpha_tables(8);
        assert_eq!(
            bits(&pd8),
            [
                0,
                0x3e12_4925,
                0x3e92_4925,
                0x3edb_6db7,
                0x3f12_4925,
                0x3f36_db6e,
                0x3f5b_6db7,
                0x3f92_4925
            ]
        );
        assert_eq!(
            bits(&pc8),
            [
                0x3f80_0000,
                0x3f5b_6db7,
                0x3f36_db6e,
                0x3f12_4925,
                0x3edb_6db7,
                0x3e92_4925,
                0x3e12_4925,
                0
            ]
        );
        let (pc6, pd6) = alpha_tables(6);
        assert_eq!(
            bits(&pc6[..6]),
            [
                0x3f80_0000,
                0x3f4c_cccd,
                0x3f19_999a,
                0x3ecc_cccd,
                0x3e4c_cccd,
                0
            ]
        );
        assert_eq!(
            bits(&pd6[..6]),
            [
                0,
                0x3e4c_cccd,
                0x3ecc_cccd,
                0x3f19_999a,
                0x3f4c_cccd,
                0x3f80_0000
            ]
        );
    }
}

//! Levels through D3DX's working colour, a few rows (a band of blocks for DXT) at a time:
//! decoding, encoding, the same-size conversion D3DX does when it cannot copy (0x624879)
//! and its box filter for the mip levels it makes (0x624c78).
//!
//! The DXT blocks of a level are independent of each other in D3DX's encoder, so a large level
//! is encoded by several threads, each a band of block rows; every block's bytes are what one
//! thread would make.

use crate::dxt::{generic as dxt, Color};
use crate::format;
use crate::num::{Fpu, Precision, P24, P53};
use crate::pixel;
use std::sync::atomic::{AtomicUsize, Ordering};

/// One level: format, size and bytes (rows of pixels, or of blocks for DXT, packed).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Level {
    pub format: u32,
    pub width: u32,
    pub height: u32,
    pub bytes: Vec<u8>,
}

/// Whether this crate reads and writes `f`.
pub(crate) fn supported(f: u32) -> bool {
    format::is_dxt(f) && f != format::DXT2 && f != format::DXT4 || pixel::supported(f)
}

/// The bytes of one block of the DXT format `f`.
fn block_bytes(f: u32) -> usize {
    if f == format::DXT1 {
        8
    } else {
        16
    }
}

/// One DXT block of `f` to the working colour.
fn decode_block<P: Fpu>(f: u32, b: &[u8]) -> [Color; 16] {
    match f {
        format::DXT1 => dxt::decode_color::<P>(b),
        format::DXT2 | format::DXT3 => dxt::decode_dxt3::<P>(b),
        _ => dxt::decode_dxt5::<P>(b),
    }
}

/// Block rows `first..first + n` of a DXT level (`w` x `h`) to the working colour: rows
/// `4 first..` of the level, `w` pixels each, into `out` (rows past the level's bottom are
/// left as they are).
fn decode_block_rows<P: Fpu>(
    f: u32,
    w: usize,
    h: usize,
    bytes: &[u8],
    first: usize,
    n: usize,
    out: &mut [Color],
) {
    let bs = block_bytes(f);
    let bw = w.div_ceil(4);
    for by in first..(first + n).min(h.div_ceil(4)) {
        for bx in 0..bw {
            let o = (by * bw + bx) * bs;
            let px = decode_block::<P>(f, &bytes[o..o + bs]);
            for j in 0..4 {
                let y = by * 4 + j;
                if y >= h {
                    break;
                }
                let row = (y - first * 4) * w;
                for i in 0..4 {
                    let x = bx * 4 + i;
                    if x < w {
                        out[row + x] = px[j * 4 + i];
                    }
                }
            }
        }
    }
}

/// A level to the working colour, `width * height` pixels row by row, at `precision`.
pub fn decode(f: u32, w: usize, h: usize, bytes: &[u8], precision: Precision) -> Vec<Color> {
    match precision {
        Precision::Single => decode_p::<P24>(f, w, h, bytes),
        Precision::Double => decode_p::<P53>(f, w, h, bytes),
    }
}

fn decode_p<P: Fpu>(f: u32, w: usize, h: usize, bytes: &[u8]) -> Vec<Color> {
    let mut out = vec![[0.0f32; 4]; w * h];
    if format::is_dxt(f) {
        decode_block_rows::<P>(f, w, h, bytes, 0, h.div_ceil(4), &mut out);
    } else {
        let row = format::level_size(f, w as u32, 1);
        for y in 0..h {
            pixel::decode_row(
                f,
                &bytes[y * row..(y + 1) * row],
                w,
                &mut out[y * w..(y + 1) * w],
            );
        }
    }
    out
}

/// The 16 pixels of the block at column `bx` of a band of up to 4 rows (`rows`, `w` pixels
/// each, `vh` of them valid), as D3DX's DXT encoder gathers them: a block reaching past the
/// level's right or bottom edge repeats its pixels (columns, then rows, from the pattern 0, 0,
/// 0, 1).
fn band_block(rows: &[Color], w: usize, vh: usize, bx: usize) -> [Color; 16] {
    const REP: [usize; 4] = [0, 0, 0, 1];
    let mut px = [[0.0f32; 4]; 16];
    let vw = w - bx * 4;
    for j in 0..vh.min(4) {
        for i in 0..vw.min(4) {
            px[j * 4 + i] = rows[j * w + bx * 4 + i];
        }
    }
    if vw < 4 {
        for j in 0..vh.min(4) {
            for i in vw..4 {
                px[j * 4 + i] = px[j * 4 + REP[i]];
            }
        }
    }
    for j in vh.min(4)..4 {
        for i in 0..4 {
            px[j * 4 + i] = px[REP[j] * 4 + i];
        }
    }
    px
}

/// The 16 pixels of block (`bx`, `by`) of a `w` x `h` level ([`band_block`]).
pub fn block_pixels(colors: &[Color], w: usize, h: usize, bx: usize, by: usize) -> [Color; 16] {
    band_block(&colors[by * 4 * w..], w, h - by * 4, bx)
}

/// One DXT block of `f`, into `out`.
fn encode_block_into<P: Fpu>(f: u32, px: &[Color; 16], dither: bool, out: &mut [u8]) {
    match f {
        format::DXT1 => out.copy_from_slice(&dxt::encode_dxt1::<P>(px, dither)),
        format::DXT2 | format::DXT3 => out.copy_from_slice(&dxt::encode_dxt3::<P>(px, dither)),
        _ => out.copy_from_slice(&dxt::encode_dxt5::<P>(px, dither)),
    }
}

/// One DXT block of `f` at `precision`.
pub fn encode_block(f: u32, px: &[Color; 16], dither: bool, precision: Precision) -> Vec<u8> {
    let mut out = vec![0u8; block_bytes(f)];
    match precision {
        Precision::Single => encode_block_into::<P24>(f, px, dither, &mut out),
        Precision::Double => encode_block_into::<P53>(f, px, dither, &mut out),
    }
    out
}

/// One pixel of the box filter's float path: `(((a + b) + c) + d) * 0.25` per channel, the
/// two pixels of the upper row first (0x624ef2), each step stored (0x624f0a..): the same at
/// either precision.
#[inline(always)]
fn box4(a: &Color, b: &Color, c: &Color, d: &Color) -> Color {
    std::array::from_fn(|k| (((a[k] + b[k]) + c[k]) + d[k]) * 0.25)
}

/// The box filter's float path for row `y` of the next level: rows `r0`, `r1` of the source
/// (`w` pixels each; the same row for a source 1 high), into `out` (`w2` pixels).
#[inline(always)]
fn box_row(r0: &[Color], r1: &[Color], w: usize, out: &mut [Color]) {
    for (x, o) in out.iter_mut().enumerate() {
        let (x0, x1) = if w == 1 { (0, 0) } else { (2 * x, 2 * x + 1) };
        *o = box4(&r0[x0], &r0[x1], &r1[x0], &r1[x1]);
    }
}

/// Blocks per thread below which a level is not split.
const BLOCKS_PER_THREAD: usize = 256;

/// The most threads, the loading threads' own included, that levels are encoded by at once
/// across the whole process (0: the machine's hardware threads).
static MAX_THREADS: AtomicUsize = AtomicUsize::new(0);

/// Helper threads in use now, by every level being split.
static HELPERS: AtomicUsize = AtomicUsize::new(0);

/// Caps the threads levels are encoded by at once, across the process (1: each loading thread
/// alone; 0: the machine's hardware threads, the default). Several loading threads share the
/// helpers, so they never start more threads than this together.
pub fn set_max_threads(n: usize) {
    MAX_THREADS.store(n, Ordering::Relaxed);
}

/// Helper threads taken from the process-wide budget, given back when dropped.
struct Helpers(usize);

impl Helpers {
    /// Up to `want` helpers, as many as the budget has left (possibly none).
    fn take(want: usize) -> Helpers {
        let cap = match MAX_THREADS.load(Ordering::Relaxed) {
            0 => std::thread::available_parallelism().map_or(1, |n| n.get()),
            n => n,
        }
        .saturating_sub(1);
        let mut used = HELPERS.load(Ordering::Acquire);
        loop {
            let got = want.min(cap.saturating_sub(used));
            match HELPERS.compare_exchange_weak(
                used,
                used + got,
                Ordering::AcqRel,
                Ordering::Acquire,
            ) {
                Ok(_) => return Helpers(got),
                Err(now) => used = now,
            }
        }
    }
}

impl Drop for Helpers {
    fn drop(&mut self) {
        HELPERS.fetch_sub(self.0, Ordering::AcqRel);
    }
}

/// The box filter for a DXT level: band by band of four destination rows, each band from the
/// eight source rows under it, decoded, filtered and encoded (only those rows are ever in the
/// working colour); a large level's bands split between this thread and helpers.
fn box_dxt<P: Fpu>(src: &Level) -> Level {
    let f = src.format;
    let (w, h) = (src.width as usize, src.height as usize);
    let (w2, h2) = ((w / 2).max(1), (h / 2).max(1));
    let bs = block_bytes(f);
    let (bw2, bh2) = (w2.div_ceil(4), h2.div_ceil(4));
    let mut out = vec![0u8; bw2 * bh2 * bs];
    let band_bytes = bw2 * bs;
    let work = |first: usize, out: &mut [u8]| {
        let mut srows = vec![[0.0f32; 4]; 8 * w];
        let mut drows = vec![[0.0f32; 4]; 4 * w2];
        for (k, band) in out.chunks_mut(band_bytes).enumerate() {
            let by = first + k;
            // Source block rows 2 by and 2 by + 1: source rows 8 by .. 8 by + 8.
            decode_block_rows::<P>(f, w, h, &src.bytes, 2 * by, 2, &mut srows);
            let vh = (h2 - by * 4).min(4);
            for j in 0..vh {
                let (r0, r1) = if h == 1 { (0, 0) } else { (2 * j, 2 * j + 1) };
                box_row(
                    &srows[r0 * w..(r0 + 1) * w],
                    &srows[r1 * w..(r1 + 1) * w],
                    w,
                    &mut drows[j * w2..(j + 1) * w2],
                );
            }
            for bx in 0..bw2 {
                let px = band_block(&drows, w2, vh, bx);
                encode_block_into::<P>(f, &px, false, &mut band[bx * bs..(bx + 1) * bs]);
            }
        }
    };
    let blocks = bw2 * bh2;
    let want = if blocks < 2 * BLOCKS_PER_THREAD {
        0
    } else {
        blocks / BLOCKS_PER_THREAD - 1
    };
    let helpers = Helpers::take(want);
    if helpers.0 == 0 {
        work(0, &mut out);
    } else {
        let per = bh2.div_ceil(helpers.0 + 1);
        std::thread::scope(|s| {
            let mut chunks = out.chunks_mut(per * band_bytes).enumerate();
            let own = chunks.next();
            for (i, chunk) in chunks {
                let work = &work;
                s.spawn(move || work(i * per, chunk));
            }
            if let Some((_, chunk)) = own {
                work(0, chunk);
            }
        });
    }
    drop(helpers);
    Level {
        format: f,
        width: w2 as u32,
        height: h2 as u32,
        bytes: out,
    }
}

/// D3DX's box filter (0x624c78) for the next mip level of `src` (no dither: the mip filter is
/// `D3DX_FILTER_BOX` alone): the integer path for the formats that have one when `src` is at
/// least 2x2, else through the working colour: each pixel
/// `(((r0[2x] + r0[2x+1]) + r1[2x]) + r1[2x+1]) * 0.25`, a side of 1 using its one pixel twice
/// (such a level is one row or one column).
pub(crate) fn box_next<P: Fpu>(src: &Level) -> Level {
    let (w, h) = (src.width as usize, src.height as usize);
    let (w2, h2) = ((w / 2).max(1), (h / 2).max(1));
    let f = src.format;
    if format::is_dxt(f) {
        return box_dxt::<P>(src);
    }
    if pixel::has_integer_box(f) && w >= 2 && h >= 2 {
        return Level {
            format: f,
            width: w2 as u32,
            height: h2 as u32,
            bytes: pixel::box_integer(f, &src.bytes, w, h),
        };
    }
    // Row by row: two source rows and one destination row in the working colour at a time.
    let (srow, drow) = (
        format::level_size(f, w as u32, 1),
        format::level_size(f, w2 as u32, 1),
    );
    let mut bytes = vec![0u8; drow * h2];
    let (mut c0, mut c1, mut out) = (
        vec![[0.0f32; 4]; w],
        vec![[0.0f32; 4]; w],
        vec![[0.0f32; 4]; w2],
    );
    for y in 0..h2 {
        let (r0, r1) = if h == 1 { (0, 0) } else { (2 * y, 2 * y + 1) };
        pixel::decode_row(f, &src.bytes[r0 * srow..(r0 + 1) * srow], w, &mut c0);
        pixel::decode_row(f, &src.bytes[r1 * srow..(r1 + 1) * srow], w, &mut c1);
        box_row(&c0, &c1, w, &mut out);
        pixel::encode_row(
            f,
            &out,
            y as u32,
            false,
            &mut bytes[y * drow..(y + 1) * drow],
        );
    }
    Level {
        format: f,
        width: w2 as u32,
        height: h2 as u32,
        bytes,
    }
}

/// The working colours of the box filter's float path for the next level of `src`, whole (for
/// the verification tools; [`box_next`] goes row by row), the source decoded at `precision`.
pub fn box_colors(src: &Level, precision: Precision) -> Vec<Color> {
    let (w, h) = (src.width as usize, src.height as usize);
    let (w2, h2) = ((w / 2).max(1), (h / 2).max(1));
    let c = decode(src.format, w, h, &src.bytes, precision);
    let mut out = vec![[0.0f32; 4]; w2 * h2];
    for y in 0..h2 {
        let (r0, r1) = if h == 1 { (0, 0) } else { (2 * y, 2 * y + 1) };
        box_row(
            &c[r0 * w..(r0 + 1) * w],
            &c[r1 * w..(r1 + 1) * w],
            w,
            &mut out[y * w2..(y + 1) * w2],
        );
    }
    out
}

/// What D3DX does with a level of the file it loads into a level of the same size
/// (`D3DXLoadSurfaceFromMemory` with `D3DX_FILTER_TRIANGLE | D3DX_FILTER_DITHER`): the bytes as
/// they are when the formats match and, for DXT, the level is whole blocks (0x624779, 0x6237ae);
/// otherwise through the working colour with dither (0x624879), a row at a time (a block row for
/// DXT), so a large level never is in the working colour whole.
pub(crate) fn load_same_size<P: Fpu>(
    src_format: u32,
    dst_format: u32,
    w: u32,
    h: u32,
    bytes: &[u8],
) -> Vec<u8> {
    if src_format == dst_format && (!format::is_dxt(dst_format) || (w % 4 == 0 && h % 4 == 0)) {
        return bytes.to_vec();
    }
    let (w, h) = (w as usize, h as usize);
    let mut out = vec![0u8; format::level_size(dst_format, w as u32, h as u32)];
    // Rows `y0..y0 + n` of the source (whole block rows for DXT) into `rows`.
    let decode_rows = |y0: usize, n: usize, rows: &mut [Color]| {
        if format::is_dxt(src_format) {
            decode_block_rows::<P>(src_format, w, h, bytes, y0 / 4, n.div_ceil(4), rows);
        } else {
            let row = format::level_size(src_format, w as u32, 1);
            for y in y0..(y0 + n).min(h) {
                let r = &mut rows[(y - y0) * w..(y - y0 + 1) * w];
                pixel::decode_row(src_format, &bytes[y * row..(y + 1) * row], w, r);
            }
        }
    };
    if format::is_dxt(dst_format) {
        let bs = block_bytes(dst_format);
        let bw = w.div_ceil(4);
        let mut rows = vec![[0.0f32; 4]; 4 * w];
        for by in 0..h.div_ceil(4) {
            decode_rows(4 * by, 4, &mut rows);
            let vh = (h - 4 * by).min(4);
            for bx in 0..bw {
                let px = band_block(&rows, w, vh, bx);
                let o = (by * bw + bx) * bs;
                encode_block_into::<P>(dst_format, &px, true, &mut out[o..o + bs]);
            }
        }
    } else {
        let row = format::level_size(dst_format, w as u32, 1);
        let mut c = vec![[0.0f32; 4]; if format::is_dxt(src_format) { 4 * w } else { w }];
        let step = if format::is_dxt(src_format) { 4 } else { 1 };
        for y0 in (0..h).step_by(step) {
            decode_rows(y0, step, &mut c);
            for y in y0..(y0 + step).min(h) {
                let colors = &c[(y - y0) * w..(y - y0 + 1) * w];
                pixel::encode_row(
                    dst_format,
                    colors,
                    y as u32,
                    true,
                    &mut out[y * row..(y + 1) * row],
                );
            }
        }
    }
    out
}

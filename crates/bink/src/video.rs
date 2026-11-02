//! Bink 1 video frames.
//!
//! A frame holds up to four planes (alpha, luma, and two chroma planes at
//! half size), each coded as rows of 8x8 blocks. Every block row starts by
//! topping up nine "bundles", streams of values (block types, colours,
//! motion offsets, DC values and so on) that the blocks of that row and
//! later rows then take from. Blocks either copy from the previous frame,
//! fill, draw patterns and runs, or run an 8x8 inverse DCT, and some code a
//! 16x16 area at half resolution.
//!
//! Function addresses in comments are in the game's `binkw32.dll` (image
//! base 0x18000000), whose output this decoder matches; see the crate
//! documentation for how that was checked.

use crate::bits::BitReader;
use crate::container::{FLAG_ALPHA, FLAG_GRAY};
use crate::tables::{INTER_QUANT, INTRA_QUANT, PATTERNS, SCAN, TREE_CODES, TREE_LENGTHS};

/// Block types (8x8 unless noted).
const SKIP: u8 = 0;
const SCALED: u8 = 1;
const MOTION: u8 = 2;
const RUN: u8 = 3;
const RESIDUE: u8 = 4;
const INTRA: u8 = 5;
const FILL: u8 = 6;
const INTER: u8 = 7;
const PATTERN: u8 = 8;
const RAW: u8 = 9;

/// Run lengths for block-type symbols 12 to 15 (binkw32.dll 0x180487c8,
/// indexed from 0x180487bc by the block-type reader at 0x18019860).
const BLOCK_TYPE_RUNS: [usize; 4] = [4, 8, 12, 32];

/// Bits of the first DC value in a DC bundle.
const DC_START_BITS: u32 = 11;

/// The bundles, in the order a block row reads them.
const BLOCK_TYPES: usize = 0;
const SUB_BLOCK_TYPES: usize = 1;
const COLORS: usize = 2;
const PATTERN_BITS: usize = 3;
const X_OFF: usize = 4;
const Y_OFF: usize = 5;
const INTRA_DC: usize = 6;
const INTER_DC: usize = 7;
const RUNS: usize = 8;
const BUNDLES: usize = 9;

/// Why a frame could not be decoded completely. The planes keep what was
/// decoded before the problem.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecodeError {
    /// A bundle was given more values than it can hold.
    BundleOverflow,
    /// A block type outside the known ones.
    BadBlockType(u8),
    /// A run went past the end of its block.
    BadRun,
    /// A motion vector points outside the previous frame.
    MotionOutOfRange,
    /// The frame's data ended early.
    Truncated,
}

impl std::fmt::Display for DecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DecodeError::BundleOverflow => write!(f, "a bundle overflowed"),
            DecodeError::BadBlockType(t) => write!(f, "unknown block type {t}"),
            DecodeError::BadRun => write!(f, "a run went past its block"),
            DecodeError::MotionOutOfRange => write!(f, "a motion vector left the frame"),
            DecodeError::Truncated => write!(f, "the frame's data ended early"),
        }
    }
}

impl std::error::Error for DecodeError {}

/// One of the 16 fixed Huffman code sets, as a lookup on the next 7 bits
/// (the longest code): (leaf, code length).
struct Vlc {
    table: [(u8, u8); 128],
}

fn build_vlcs() -> Vec<Vlc> {
    (0..16)
        .map(|t| {
            let mut table = [(0u8, 0u8); 128];
            for leaf in 0..16 {
                let len = TREE_LENGTHS[t][leaf] as usize;
                let code = TREE_CODES[t][leaf] as usize;
                for fill in 0..(1usize << (7 - len)) {
                    table[code | (fill << len)] = (leaf as u8, len as u8);
                }
            }
            Vlc { table }
        })
        .collect()
}

/// A tree as a bundle uses it: which code set, and which symbol each leaf
/// stands for.
#[derive(Clone, Copy)]
struct Tree {
    vlc: usize,
    syms: [u8; 16],
}

impl Default for Tree {
    fn default() -> Self {
        let mut syms = [0u8; 16];
        for (i, s) in syms.iter_mut().enumerate() {
            *s = i as u8;
        }
        Tree { vlc: 0, syms }
    }
}

/// Merges two sorted halves of `src` into `dst` as the stream says: a 0 bit
/// takes the next symbol of the first half, a 1 bit the second.
fn merge(br: &mut BitReader, dst: &mut [u8], src: &[u8], size: usize) {
    let (mut a, mut b) = (0usize, size);
    let (mut left_a, mut left_b) = (size, size);
    let mut o = 0;
    while left_a > 0 && left_b > 0 {
        if !br.bit() {
            dst[o] = src[a];
            a += 1;
            left_a -= 1;
        } else {
            dst[o] = src[b];
            b += 1;
            left_b -= 1;
        }
        o += 1;
    }
    while left_a > 0 {
        dst[o] = src[a];
        a += 1;
        left_a -= 1;
        o += 1;
    }
    while left_b > 0 {
        dst[o] = src[b];
        b += 1;
        left_b -= 1;
        o += 1;
    }
}

fn read_tree(br: &mut BitReader) -> Tree {
    let mut tree = Tree {
        vlc: br.bits(4) as usize,
        ..Tree::default()
    };
    if tree.vlc == 0 {
        return tree;
    }
    if br.bit() {
        // An explicit list of the first symbols; the rest follow in order.
        let mut seen = [false; 16];
        let mut len = br.bits(3) as usize;
        for i in 0..=len {
            let s = br.bits(4) as u8;
            tree.syms[i] = s;
            seen[s as usize] = true;
        }
        for (i, &was) in seen.iter().enumerate() {
            if len >= 15 {
                break;
            }
            if !was {
                len += 1;
                tree.syms[len] = i as u8;
            }
        }
    } else {
        // A shuffle: up to four rounds of merges of runs of 1, 2, 4, 8.
        let rounds = br.bits(2) as usize;
        let mut cur = [0u8; 16];
        for (i, c) in cur.iter_mut().enumerate() {
            *c = i as u8;
        }
        let mut next = [0u8; 16];
        for i in 0..=rounds {
            let size = 1usize << i;
            let mut t = 0;
            while t < 16 {
                merge(br, &mut next[t..t + 2 * size], &cur[t..t + 2 * size], size);
                t += size << 1;
            }
            std::mem::swap(&mut cur, &mut next);
        }
        tree.syms = cur;
    }
    tree
}

/// A stream of values for one kind of data.
struct Bundle {
    /// Bits of the "how many values follow" field.
    len_bits: u32,
    tree: Tree,
    data: Vec<i16>,
    /// Where the next values read from the stream go, or `None` once the
    /// stream said there are no more for this plane.
    dec: Option<usize>,
    /// The next value a block takes.
    ptr: usize,
}

impl Bundle {
    fn new(capacity: usize) -> Self {
        Bundle {
            len_bits: 0,
            tree: Tree::default(),
            data: vec![0; capacity],
            dec: Some(0),
            ptr: 0,
        }
    }

    fn take(&mut self) -> i32 {
        let v = self.data.get(self.ptr).copied().unwrap_or(0);
        self.ptr += 1;
        v as i32
    }

    /// The count of values to read now, or `None` when nothing is read: the
    /// values already read are not all used yet, or the stream ended.
    fn count(&mut self, br: &mut BitReader) -> Option<usize> {
        let dec = self.dec?;
        if dec > self.ptr {
            return None;
        }
        let t = br.bits(self.len_bits) as usize;
        if t == 0 {
            self.dec = None;
            return None;
        }
        Some(t)
    }
}

fn log2(v: usize) -> u32 {
    usize::BITS - 1 - v.leading_zeros()
}

/// A plane's pixels, `stride` bytes per row, with spare rows below so that
/// blocks along the edges never write outside it.
#[derive(Clone)]
struct Plane {
    stride: usize,
    /// Blocks across and down.
    bw: usize,
    bh: usize,
    /// Visible size.
    width: usize,
    height: usize,
    pixels: Vec<u8>,
}

impl Plane {
    fn new(width: usize, height: usize, bw: usize, bh: usize) -> Self {
        let stride = bw * 8;
        Plane {
            stride,
            bw,
            bh,
            width,
            height,
            pixels: vec![0; stride * (bh * 8 + 16)],
        }
    }
}

/// Decodes a movie's frames in order.
pub struct Decoder {
    revision: u8,
    flags: u32,
    width: usize,
    height: usize,
    vlcs: Vec<Vlc>,
    bundles: Vec<Bundle>,
    col_high: [Tree; 16],
    col_last: usize,
    /// The frame being decoded and the one before it, each Y, U, V, A.
    cur: Vec<Plane>,
    prev: Vec<Plane>,
}

impl Decoder {
    pub fn new(width: u32, height: u32, revision: u8, video_flags: u32) -> Self {
        let (w, h) = (width as usize, height as usize);
        let luma = Plane::new(w, h, (w + 7) >> 3, (h + 7) >> 3);
        let chroma = Plane::new(w >> 1, h >> 1, (w + 15) >> 4, (h + 15) >> 4);
        let planes = vec![luma.clone(), chroma.clone(), chroma, luma];
        let blocks = ((w + 7) >> 3) * ((h + 7) >> 3);
        let bundles = (0..BUNDLES)
            .map(|i| {
                // DC bundles hold 16-bit values in the same space.
                let cap = if i == INTRA_DC || i == INTER_DC {
                    blocks * 32
                } else {
                    blocks * 64
                };
                Bundle::new(cap)
            })
            .collect();
        Decoder {
            revision,
            flags: video_flags,
            width: w,
            height: h,
            vlcs: build_vlcs(),
            bundles,
            col_high: [Tree::default(); 16],
            col_last: 0,
            cur: planes.clone(),
            prev: planes,
        }
    }

    pub fn width(&self) -> u32 {
        self.width as u32
    }

    pub fn height(&self) -> u32 {
        self.height as u32
    }

    /// Decodes the next frame from its video data (the part of the packet
    /// after the audio).
    pub fn decode(&mut self, data: &[u8]) -> Result<(), DecodeError> {
        std::mem::swap(&mut self.cur, &mut self.prev);
        let mut br = BitReader::new(data);
        let skip_size_words = self.revision >= b'i';
        if self.flags & FLAG_ALPHA != 0 {
            if skip_size_words {
                br.skip(32);
            }
            self.decode_plane(&mut br, 3)?;
        }
        if skip_size_words {
            br.skip(32);
        }
        let planes = if self.flags & FLAG_GRAY != 0 { 1 } else { 3 };
        let swap_chroma = self.revision >= b'h';
        for p in 0..planes {
            let idx = if p == 0 || !swap_chroma { p } else { p ^ 3 };
            self.decode_plane(&mut br, idx)?;
            if br.position() >= br.len() {
                break;
            }
        }
        if br.overrun() {
            return Err(DecodeError::Truncated);
        }
        Ok(())
    }

    /// The current frame's planes: luma, then U (Cb) and V (Cr) at half
    /// size, each as (pixels, stride). Only `width x height` (halved for
    /// chroma) of each is picture.
    pub fn planes(&self) -> [(&[u8], usize); 3] {
        [
            (&self.cur[0].pixels, self.cur[0].stride),
            (&self.cur[1].pixels, self.cur[1].stride),
            (&self.cur[2].pixels, self.cur[2].stride),
        ]
    }

    /// The alpha plane, for movies that have one.
    pub fn alpha(&self) -> Option<(&[u8], usize)> {
        (self.flags & FLAG_ALPHA != 0).then(|| (&self.cur[3].pixels[..], self.cur[3].stride))
    }

    /// The current frame as 32-bit pixels, 4 bytes each in the order blue,
    /// green, red, 0, rows of `width` pixels: what the game's library gives
    /// for its 32-bit surface type, which is what the game draws (see
    /// `color`).
    pub fn to_bgrx(&self, out: &mut Vec<u8>) {
        out.clear();
        out.reserve(self.width * self.height * 4);
        let (y_plane, u_plane, v_plane) = (&self.cur[0], &self.cur[1], &self.cur[2]);
        for row in 0..self.height {
            let ys = &y_plane.pixels[row * y_plane.stride..];
            let us = &u_plane.pixels[(row >> 1) * u_plane.stride..];
            let vs = &v_plane.pixels[(row >> 1) * v_plane.stride..];
            for x in 0..self.width {
                let [b, g, r] = crate::color::bgr(ys[x], us[x >> 1], vs[x >> 1]);
                out.extend_from_slice(&[b, g, r, 0]);
            }
        }
    }

    /// The current frame as YV12: the luma plane, then V, then U, each row
    /// exactly as wide as the picture. This is the layout the game's library
    /// gives for its YV12 surface type.
    pub fn to_yv12(&self, out: &mut Vec<u8>) {
        out.clear();
        for idx in [0usize, 2, 1] {
            let p = &self.cur[idx];
            let (w, h) = if idx == 0 {
                (self.width, self.height)
            } else {
                (self.width >> 1, self.height >> 1)
            };
            for y in 0..h {
                out.extend_from_slice(&p.pixels[y * p.stride..y * p.stride + w]);
            }
        }
    }

    fn huff(&self, br: &mut BitReader, tree: &Tree) -> u8 {
        let (leaf, len) = self.vlcs[tree.vlc].table[br.peek(7) as usize];
        br.skip(len as usize);
        tree.syms[leaf as usize]
    }

    fn read_bundle_trees(&mut self, br: &mut BitReader, plane_width: usize, bw: usize) {
        let width = (plane_width.max(8) + 7) & !7;
        let lens = [
            log2((width >> 3) + 511) + 1, // block types
            log2((width >> 4) + 511) + 1, // sub-block types
            log2(bw * 64 + 511) + 1,      // colours
            log2((bw << 3) + 511) + 1,    // pattern bits
            log2((width >> 3) + 511) + 1, // x offsets
            log2((width >> 3) + 511) + 1, // y offsets
            log2((width >> 3) + 511) + 1, // intra DC
            log2((width >> 3) + 511) + 1, // inter DC
            log2(bw * 48 + 511) + 1,      // runs
        ];
        for (i, len) in lens.into_iter().enumerate() {
            if i == COLORS {
                for t in 0..16 {
                    self.col_high[t] = read_tree(br);
                }
                self.col_last = 0;
            }
            let b = &mut self.bundles[i];
            b.len_bits = len;
            if i != INTRA_DC && i != INTER_DC {
                b.tree = read_tree(br);
            }
            b.dec = Some(0);
            b.ptr = 0;
        }
    }

    fn read_block_types(&mut self, br: &mut BitReader, which: usize) -> Result<(), DecodeError> {
        let Some(t) = self.bundles[which].count(br) else {
            return Ok(());
        };
        let start = self.bundles[which].dec.unwrap_or(0);
        let end = start + t;
        if end > self.bundles[which].data.len() {
            return Err(DecodeError::BundleOverflow);
        }
        if br.bit() {
            let v = br.bits(4) as i16;
            self.bundles[which].data[start..end].fill(v);
        } else {
            let tree = self.bundles[which].tree;
            let mut o = start;
            let mut last = 0i16;
            while o < end {
                let v = self.huff(br, &tree);
                if v < 12 {
                    last = v as i16;
                    self.bundles[which].data[o] = last;
                    o += 1;
                } else {
                    let run = BLOCK_TYPE_RUNS[(v - 12) as usize];
                    if end - o < run {
                        return Err(DecodeError::BundleOverflow);
                    }
                    self.bundles[which].data[o..o + run].fill(last);
                    o += run;
                }
            }
        }
        self.bundles[which].dec = Some(end);
        Ok(())
    }

    fn read_colors(&mut self, br: &mut BitReader) -> Result<(), DecodeError> {
        let Some(t) = self.bundles[COLORS].count(br) else {
            return Ok(());
        };
        let start = self.bundles[COLORS].dec.unwrap_or(0);
        let end = start + t;
        if end > self.bundles[COLORS].data.len() {
            return Err(DecodeError::BundleOverflow);
        }
        let tree = self.bundles[COLORS].tree;
        let old = self.revision < b'i';
        let one = |d: &mut Decoder, br: &mut BitReader| -> i16 {
            let high_tree = d.col_high[d.col_last];
            d.col_last = d.huff(br, &high_tree) as usize;
            let low = d.huff(br, &tree) as i32;
            let mut v = ((d.col_last as i32) << 4) | low;
            if old {
                // Earlier revisions store a sign and magnitude around 128.
                let sign = ((v as i8) >> 7) as i32;
                v = ((v & 0x7f) ^ sign) - sign;
                v += 0x80;
            }
            v as u8 as i16
        };
        if br.bit() {
            let v = one(self, br);
            self.bundles[COLORS].data[start..end].fill(v);
        } else {
            for o in start..end {
                let v = one(self, br);
                self.bundles[COLORS].data[o] = v;
            }
        }
        self.bundles[COLORS].dec = Some(end);
        Ok(())
    }

    fn read_patterns(&mut self, br: &mut BitReader) -> Result<(), DecodeError> {
        let Some(t) = self.bundles[PATTERN_BITS].count(br) else {
            return Ok(());
        };
        let start = self.bundles[PATTERN_BITS].dec.unwrap_or(0);
        let end = start + t;
        if end > self.bundles[PATTERN_BITS].data.len() {
            return Err(DecodeError::BundleOverflow);
        }
        let tree = self.bundles[PATTERN_BITS].tree;
        for o in start..end {
            let lo = self.huff(br, &tree) as i16;
            let hi = self.huff(br, &tree) as i16;
            self.bundles[PATTERN_BITS].data[o] = lo | (hi << 4);
        }
        self.bundles[PATTERN_BITS].dec = Some(end);
        Ok(())
    }

    fn read_motion(&mut self, br: &mut BitReader, which: usize) -> Result<(), DecodeError> {
        let Some(t) = self.bundles[which].count(br) else {
            return Ok(());
        };
        let start = self.bundles[which].dec.unwrap_or(0);
        let end = start + t;
        if end > self.bundles[which].data.len() {
            return Err(DecodeError::BundleOverflow);
        }
        if br.bit() {
            let mut v = br.bits(4) as i16;
            if v != 0 && br.bit() {
                v = -v;
            }
            self.bundles[which].data[start..end].fill(v);
        } else {
            let tree = self.bundles[which].tree;
            for o in start..end {
                let mut v = self.huff(br, &tree) as i16;
                if v != 0 && br.bit() {
                    v = -v;
                }
                self.bundles[which].data[o] = v;
            }
        }
        self.bundles[which].dec = Some(end);
        Ok(())
    }

    fn read_dcs(
        &mut self,
        br: &mut BitReader,
        which: usize,
        signed: bool,
    ) -> Result<(), DecodeError> {
        let Some(count) = self.bundles[which].count(br) else {
            return Ok(());
        };
        let start = self.bundles[which].dec.unwrap_or(0);
        if start + count > self.bundles[which].data.len() {
            return Err(DecodeError::BundleOverflow);
        }
        let mut v = br.bits(DC_START_BITS - signed as u32) as i32;
        if v != 0 && signed && br.bit() {
            v = -v;
        }
        let data = &mut self.bundles[which].data;
        let mut o = start;
        data[o] = v as i16;
        o += 1;
        let rest = count - 1;
        let mut i = 0;
        while i < rest {
            let group = (rest - i).min(8);
            let bsize = br.bits(4);
            if bsize != 0 {
                for _ in 0..group {
                    let mut d = br.bits(bsize) as i32;
                    if d != 0 && br.bit() {
                        d = -d;
                    }
                    v += d;
                    data[o] = v as i16;
                    o += 1;
                }
            } else {
                for _ in 0..group {
                    data[o] = v as i16;
                    o += 1;
                }
            }
            i += 8;
        }
        self.bundles[which].dec = Some(o);
        Ok(())
    }

    fn read_runs(&mut self, br: &mut BitReader) -> Result<(), DecodeError> {
        let Some(t) = self.bundles[RUNS].count(br) else {
            return Ok(());
        };
        let start = self.bundles[RUNS].dec.unwrap_or(0);
        let end = start + t;
        if end > self.bundles[RUNS].data.len() {
            return Err(DecodeError::BundleOverflow);
        }
        if br.bit() {
            let v = br.bits(4) as i16;
            self.bundles[RUNS].data[start..end].fill(v);
        } else {
            let tree = self.bundles[RUNS].tree;
            for o in start..end {
                let v = self.huff(br, &tree) as i16;
                self.bundles[RUNS].data[o] = v;
            }
        }
        self.bundles[RUNS].dec = Some(end);
        Ok(())
    }

    fn value(&mut self, which: usize) -> i32 {
        self.bundles[which].take()
    }

    fn decode_plane(&mut self, br: &mut BitReader, idx: usize) -> Result<(), DecodeError> {
        let (bw, bh, plane_width, plane_height, stride) = {
            let p = &self.cur[idx];
            (p.bw, p.bh, p.width, p.height, p.stride)
        };
        if self.revision == b'k' && br.bit() {
            // Revision 'k' can fill a whole plane with one value.
            let fill = br.bits(8) as u8;
            let p = &mut self.cur[idx];
            for y in 0..plane_height {
                p.pixels[y * stride..y * stride + plane_width].fill(fill);
            }
            br.align32();
            return Ok(());
        }
        self.read_bundle_trees(br, plane_width, bw);

        // Motion vectors may point anywhere from the frame's first block to
        // its last block's top-left corner.
        let ref_end = ((bw - 1) + stride * (bh - 1)) * 8;

        for by in 0..bh {
            self.read_block_types(br, BLOCK_TYPES)?;
            self.read_block_types(br, SUB_BLOCK_TYPES)?;
            self.read_colors(br)?;
            self.read_patterns(br)?;
            self.read_motion(br, X_OFF)?;
            self.read_motion(br, Y_OFF)?;
            self.read_dcs(br, INTRA_DC, false)?;
            self.read_dcs(br, INTER_DC, true)?;
            self.read_runs(br)?;

            let mut bx = 0;
            while bx < bw {
                let at = by * 8 * stride + bx * 8;
                let blk = self.value(BLOCK_TYPES) as u8;
                // A 16x16 block's type also appears at the places it covers
                // after its first; those are already drawn.
                if ((by & 1) != 0 || (bx & 1) != 0) && blk == SCALED {
                    bx += 2;
                    continue;
                }
                match blk {
                    SKIP => self.copy_block(idx, at, at),
                    SCALED => {
                        self.scaled_block(br, idx, at, stride)?;
                        bx += 1;
                    }
                    MOTION => {
                        let r = self.motion_ref(at, stride, ref_end)?;
                        self.copy_block(idx, at, r);
                    }
                    RUN => {
                        let mut out = [0u8; 64];
                        self.run_block(br, &mut out)?;
                        let p = &mut self.cur[idx].pixels;
                        for y in 0..8 {
                            p[at + y * stride..at + y * stride + 8]
                                .copy_from_slice(&out[y * 8..y * 8 + 8]);
                        }
                    }
                    RESIDUE => {
                        let r = self.motion_ref(at, stride, ref_end)?;
                        self.copy_block(idx, at, r);
                        let masks = br.bits(7) as i32;
                        let mut block = [0i16; 64];
                        read_residue(br, &mut block, masks);
                        let p = &mut self.cur[idx].pixels;
                        for y in 0..8 {
                            for x in 0..8 {
                                let o = at + y * stride + x;
                                p[o] = (p[o] as i32 + block[y * 8 + x] as i32) as u8;
                            }
                        }
                    }
                    INTRA => {
                        let mut coef = [0i32; 64];
                        coef[0] = self.value(INTRA_DC);
                        let q = read_dct_coeffs(br, &mut coef);
                        let out = idct(&coef, &INTRA_QUANT[q]);
                        let p = &mut self.cur[idx].pixels;
                        for y in 0..8 {
                            for x in 0..8 {
                                p[at + y * stride + x] = out[y * 8 + x];
                            }
                        }
                    }
                    FILL => {
                        let v = self.value(COLORS) as u8;
                        let p = &mut self.cur[idx].pixels;
                        for y in 0..8 {
                            p[at + y * stride..at + y * stride + 8].fill(v);
                        }
                    }
                    INTER => {
                        let r = self.motion_ref(at, stride, ref_end)?;
                        self.copy_block(idx, at, r);
                        let mut coef = [0i32; 64];
                        coef[0] = self.value(INTER_DC);
                        let q = read_dct_coeffs(br, &mut coef);
                        let out = idct(&coef, &INTER_QUANT[q]);
                        let p = &mut self.cur[idx].pixels;
                        for y in 0..8 {
                            for x in 0..8 {
                                let o = at + y * stride + x;
                                p[o] = p[o].wrapping_add(out[y * 8 + x]);
                            }
                        }
                    }
                    PATTERN => {
                        let c = [self.value(COLORS) as u8, self.value(COLORS) as u8];
                        for y in 0..8 {
                            let mut v = self.value(PATTERN_BITS);
                            let p = &mut self.cur[idx].pixels;
                            for x in 0..8 {
                                p[at + y * stride + x] = c[(v & 1) as usize];
                                v >>= 1;
                            }
                        }
                    }
                    RAW => {
                        for y in 0..8 {
                            for x in 0..8 {
                                let v = self.value(COLORS) as u8;
                                self.cur[idx].pixels[at + y * stride + x] = v;
                            }
                        }
                    }
                    other => return Err(DecodeError::BadBlockType(other)),
                }
                bx += 1;
            }
        }
        br.align32();
        Ok(())
    }

    /// Copies the previous frame's 8x8 block at `src` to `dst` in this one.
    fn copy_block(&mut self, idx: usize, dst: usize, src: usize) {
        let stride = self.cur[idx].stride;
        for y in 0..8 {
            let s = src + y * stride;
            let d = dst + y * stride;
            self.cur[idx].pixels[d..d + 8].copy_from_slice(&self.prev[idx].pixels[s..s + 8]);
        }
    }

    /// The previous frame's block a motion vector points at.
    fn motion_ref(
        &mut self,
        at: usize,
        stride: usize,
        ref_end: usize,
    ) -> Result<usize, DecodeError> {
        let x = self.value(X_OFF) as isize;
        let y = self.value(Y_OFF) as isize;
        let r = at as isize + x + y * stride as isize;
        if r < 0 || r as usize > ref_end {
            return Err(DecodeError::MotionOutOfRange);
        }
        Ok(r as usize)
    }

    /// A run block into `out` (block positions): one of the 16 fill orders,
    /// then runs that either repeat one colour or take a colour per pixel.
    fn run_block(&mut self, br: &mut BitReader, out: &mut [u8; 64]) -> Result<(), DecodeError> {
        let scan = &PATTERNS[br.bits(4) as usize];
        let mut i = 0usize;
        let mut s = 0usize;
        loop {
            let run = self.value(RUNS) as usize + 1;
            i += run;
            if i > 64 {
                return Err(DecodeError::BadRun);
            }
            if br.bit() {
                let v = self.value(COLORS) as u8;
                for _ in 0..run {
                    out[scan[s] as usize] = v;
                    s += 1;
                }
            } else {
                for _ in 0..run {
                    out[scan[s] as usize] = self.value(COLORS) as u8;
                    s += 1;
                }
            }
            if i >= 63 {
                break;
            }
        }
        if i == 63 {
            out[scan[s] as usize] = self.value(COLORS) as u8;
        }
        Ok(())
    }

    /// A 16x16 block coded at half resolution: an 8x8 block of one of the
    /// sub-block types, each pixel then drawn as 2x2.
    fn scaled_block(
        &mut self,
        br: &mut BitReader,
        idx: usize,
        at: usize,
        stride: usize,
    ) -> Result<(), DecodeError> {
        let sub = self.value(SUB_BLOCK_TYPES) as u8;
        let mut u = [0u8; 64];
        match sub {
            RUN => self.run_block(br, &mut u)?,
            INTRA => {
                let mut coef = [0i32; 64];
                coef[0] = self.value(INTRA_DC);
                let q = read_dct_coeffs(br, &mut coef);
                u = idct(&coef, &INTRA_QUANT[q]);
            }
            FILL => {
                let v = self.value(COLORS) as u8;
                let p = &mut self.cur[idx].pixels;
                for y in 0..16 {
                    p[at + y * stride..at + y * stride + 16].fill(v);
                }
                return Ok(());
            }
            PATTERN => {
                let c = [self.value(COLORS) as u8, self.value(COLORS) as u8];
                for y in 0..8 {
                    let mut v = self.value(PATTERN_BITS);
                    for x in 0..8 {
                        u[y * 8 + x] = c[(v & 1) as usize];
                        v >>= 1;
                    }
                }
            }
            RAW => {
                for v in u.iter_mut() {
                    *v = self.value(COLORS) as u8;
                }
            }
            other => return Err(DecodeError::BadBlockType(other)),
        }
        let p = &mut self.cur[idx].pixels;
        for y in 0..16 {
            for x in 0..16 {
                p[at + y * stride + x] = u[(y >> 1) * 8 + (x >> 1)];
            }
        }
        Ok(())
    }
}

/// Reads a block's DCT coefficients (all but the DC, which comes from a
/// bundle) into `coef` at their block positions, and returns the
/// quantizer index that follows them.
///
/// Coefficients are coded by bit planes, from the highest magnitude bit
/// down. A work list holds what is still to be visited: mode 0 is a group
/// of four that may split into more groups, mode 1 its second visit, mode 2
/// a group of four coefficients, and mode 3 a single coefficient. An entry
/// with mode 0 and coefficient 0 is spent.
fn read_dct_coeffs(br: &mut BitReader, coef: &mut [i32; 64]) -> usize {
    let mut list = [(0usize, 0u8); 128];
    let (mut start, mut end) = (64usize, 64usize);
    for (c, m) in [(4, 0), (24, 0), (44, 0), (1, 3), (2, 3), (3, 3)] {
        list[end] = (c, m);
        end += 1;
    }
    let mut bits = br.bits(4) as i32 - 1;
    while bits >= 0 {
        let mut pos = start;
        while pos < end {
            let (ccoef, mode) = list[pos];
            if (mode == 0 && ccoef == 0) || !br.bit() {
                pos += 1;
                continue;
            }
            match mode {
                0 | 2 => {
                    if mode == 0 {
                        list[pos] = (ccoef + 4, 1);
                    } else {
                        list[pos] = (0, 0);
                        pos += 1;
                    }
                    for k in 0..4 {
                        let c = ccoef + k;
                        if br.bit() {
                            start -= 1;
                            list[start] = (c, 3);
                        } else {
                            coef[SCAN[c] as usize] = read_coef(br, bits as u32);
                        }
                    }
                }
                1 => {
                    list[pos].1 = 2;
                    let mut c = ccoef;
                    for _ in 0..3 {
                        c += 4;
                        list[end] = (c, 2);
                        end += 1;
                    }
                }
                _ => {
                    coef[SCAN[ccoef] as usize] = read_coef(br, bits as u32);
                    list[pos] = (0, 0);
                    pos += 1;
                }
            }
        }
        bits -= 1;
    }
    br.bits(4) as usize
}

/// One coefficient whose top bit is `bits`: the lower bits and a sign.
fn read_coef(br: &mut BitReader, bits: u32) -> i32 {
    if bits == 0 {
        if br.bit() {
            -1
        } else {
            1
        }
    } else {
        let t = (br.bits(bits) | (1 << bits)) as i32;
        if br.bit() {
            -t
        } else {
            t
        }
    }
}

/// Reads a residue block: values added to the copied pixels, coded by bit
/// planes like DCT coefficients but without a transform. `masks` limits how
/// many bit-plane updates the block may contain.
fn read_residue(br: &mut BitReader, block: &mut [i16; 64], mut masks: i32) {
    let mut list = [(0usize, 0u8); 128];
    let (mut start, mut end) = (64usize, 64usize);
    for (c, m) in [(4, 0), (24, 0), (44, 0), (0, 2)] {
        list[end] = (c, m);
        end += 1;
    }
    let mut nz = [0usize; 64];
    let mut nz_count = 0usize;
    let mut mask: i32 = 1 << br.bits(3);
    while mask != 0 {
        for &n in &nz[..nz_count] {
            if !br.bit() {
                continue;
            }
            if block[n] < 0 {
                block[n] = block[n].wrapping_sub(mask as i16);
            } else {
                block[n] = block[n].wrapping_add(mask as i16);
            }
            masks -= 1;
            if masks < 0 {
                return;
            }
        }
        let mut pos = start;
        while pos < end {
            let (ccoef, mode) = list[pos];
            if (mode == 0 && ccoef == 0) || !br.bit() {
                pos += 1;
                continue;
            }
            match mode {
                0 | 2 => {
                    if mode == 0 {
                        list[pos] = (ccoef + 4, 1);
                    } else {
                        list[pos] = (0, 0);
                        pos += 1;
                    }
                    for k in 0..4 {
                        let c = ccoef + k;
                        if br.bit() {
                            start -= 1;
                            list[start] = (c, 3);
                        } else {
                            let at = SCAN[c] as usize;
                            nz[nz_count] = at;
                            nz_count += 1;
                            block[at] = if br.bit() { -mask } else { mask } as i16;
                            masks -= 1;
                            if masks < 0 {
                                return;
                            }
                        }
                    }
                }
                1 => {
                    list[pos].1 = 2;
                    let mut c = ccoef;
                    for _ in 0..3 {
                        c += 4;
                        list[end] = (c, 2);
                        end += 1;
                    }
                }
                _ => {
                    let at = SCAN[ccoef] as usize;
                    nz[nz_count] = at;
                    nz_count += 1;
                    block[at] = if br.bit() { -mask } else { mask } as i16;
                    list[pos] = (0, 0);
                    pos += 1;
                    masks -= 1;
                    if masks < 0 {
                        return;
                    }
                }
            }
        }
        mask >>= 1;
    }
}

/// `a * k`, shifted right by 11, with the 32-bit wrap-around of the
/// library's `imul`.
fn mul(a: i32, k: i32) -> i32 {
    a.wrapping_mul(k) >> 11
}

/// The 8-point transform the inverse DCT applies to each column and then
/// each row (binkw32.dll 0x1801cb30; the same constants appear in 0x1801ce60
/// and 0x1801d230).
fn transform(s: [i32; 8]) -> [i32; 8] {
    let a0 = s[0].wrapping_add(s[4]);
    let a1 = s[0].wrapping_sub(s[4]);
    let a2 = s[2].wrapping_add(s[6]);
    let a3 = mul(s[2].wrapping_sub(s[6]), 2896).wrapping_sub(a2);
    let e0 = a0.wrapping_add(a2);
    let e3 = a0.wrapping_sub(a2);
    let e1 = a1.wrapping_add(a3);
    let e2 = a1.wrapping_sub(a3);
    let a4 = s[5].wrapping_add(s[3]);
    let a5 = s[5].wrapping_sub(s[3]);
    let a6 = s[1].wrapping_add(s[7]);
    let a7 = s[1].wrapping_sub(s[7]);
    let b0 = a4.wrapping_add(a6);
    let b1 = mul(a5.wrapping_add(a7), 3784);
    let b2 = mul(a5, -5352).wrapping_sub(b0).wrapping_add(b1);
    let b3 = mul(a6.wrapping_sub(a4), 2896).wrapping_sub(b2);
    let b4 = mul(a7, 2217).wrapping_sub(b1).wrapping_add(b3);
    [
        e0.wrapping_add(b0),
        e1.wrapping_add(b2),
        e2.wrapping_add(b3),
        e3.wrapping_sub(b4),
        e3.wrapping_add(b4),
        e2.wrapping_sub(b3),
        e1.wrapping_sub(b2),
        e0.wrapping_sub(b0),
    ]
}

/// Dequantizes and inverse-transforms a block. Each coefficient is
/// multiplied by its table entry and shifted right by 11 as the columns are
/// read; a column whose coefficients below the first are all zero is
/// filled with its first value. Results are `(x + 127) >> 8`, kept to their
/// low 8 bits (the library stores bytes without clamping).
fn idct(coef: &[i32; 64], quant: &[i32; 64]) -> [u8; 64] {
    let mut tmp = [0i32; 64];
    for c in 0..8 {
        if (1..8).all(|r| coef[r * 8 + c] == 0) {
            let v = mul(coef[c], quant[c]);
            for r in 0..8 {
                tmp[r * 8 + c] = v;
            }
        } else {
            let s: [i32; 8] = std::array::from_fn(|r| mul(coef[r * 8 + c], quant[r * 8 + c]));
            let t = transform(s);
            for r in 0..8 {
                tmp[r * 8 + c] = t[r];
            }
        }
    }
    let mut out = [0u8; 64];
    for r in 0..8 {
        let s: [i32; 8] = std::array::from_fn(|c| tmp[r * 8 + c]);
        let t = transform(s);
        for c in 0..8 {
            out[r * 8 + c] = (t[c].wrapping_add(0x7f) >> 8) as u8;
        }
    }
    out
}

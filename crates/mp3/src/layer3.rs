//! Layer III decoding of one frame (ISO 11172-3 section 2.4.3.4, with ISO
//! 13818-3's changes for the lower sample rates): side information, the
//! bit reservoir, scale factors, the Huffman-coded spectrum,
//! requantization, stereo processing, reordering of short blocks, alias
//! reduction and the hybrid filterbank (IMDCT, windows, overlap-add). The
//! polyphase synthesis is in `synth`.

use std::f64::consts::{FRAC_1_SQRT_2, PI};

use crate::bits::BitReader;
use crate::decoder::Problem;
use crate::header::{frame_crc, ChannelMode, FrameHeader};
use crate::huffman;
use crate::synth::Synth;
use crate::tables::{self, computed, pow2_quarter, Bands, Computed, NR_OF_SFB, PRETAB, SLEN};

/// Spectral lines in a granule (32 subbands x 18).
pub(crate) const LINES: usize = 576;

/// How much of earlier frames' data to keep for the bit reservoir. A frame
/// can reach back at most 511 bytes (MPEG-1) or 255 (MPEG-2).
const MAX_RESERVOIR: usize = 4096;

/// One channel's side information for one granule.
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct Granule {
    /// Bits of scale factors and Huffman data.
    pub part2_3_length: usize,
    /// Pairs of values coded with the big tables.
    pub big_values: usize,
    pub global_gain: i32,
    pub scalefac_compress: usize,
    pub window_switching: bool,
    /// 0 normal, 1 start (long to short), 2 short (three windows), 3 stop.
    pub block_type: u8,
    /// Short block whose lowest two subbands (four at 8 kHz) are long.
    pub mixed_block: bool,
    pub table_select: [u8; 3],
    pub subblock_gain: [i32; 3],
    pub region0_count: usize,
    pub region1_count: usize,
    pub preflag: bool,
    pub scalefac_scale: bool,
    pub count1table_select: bool,
}

impl Granule {
    fn is_short(&self) -> bool {
        self.window_switching && self.block_type == 2
    }
}

#[derive(Clone, Debug, Default)]
pub(crate) struct SideInfo {
    /// How many bytes before this frame's own data its data starts.
    pub main_data_begin: usize,
    /// MPEG-1: granule 1 reuses granule 0's scale factors for these groups
    /// of bands.
    pub scfsi: [[bool; 4]; 2],
    /// By granule, then channel.
    pub granules: [[Granule; 2]; 2],
}

pub(crate) fn read_side_info(header: &FrameHeader, bytes: &[u8]) -> Result<SideInfo, &'static str> {
    let mut r = BitReader::new(bytes);
    let channels = header.channels();
    let lsf = header.is_lsf();
    let mut si = SideInfo::default();
    let granules = if lsf {
        si.main_data_begin = r.read(8) as usize;
        r.read(if channels == 1 { 1 } else { 2 });
        1
    } else {
        si.main_data_begin = r.read(9) as usize;
        r.read(if channels == 1 { 5 } else { 3 });
        for scfsi in si.scfsi.iter_mut().take(channels) {
            for band in scfsi.iter_mut() {
                *band = r.flag();
            }
        }
        2
    };
    for gr in 0..granules {
        for ch in 0..channels {
            let g = &mut si.granules[gr][ch];
            g.part2_3_length = r.read(12) as usize;
            g.big_values = r.read(9) as usize;
            if g.big_values > LINES / 2 {
                return Err("more than 288 pairs of big values");
            }
            g.global_gain = r.read(8) as i32;
            g.scalefac_compress = r.read(if lsf { 9 } else { 4 }) as usize;
            g.window_switching = r.flag();
            if g.window_switching {
                g.block_type = r.read(2) as u8;
                g.mixed_block = r.flag();
                g.table_select[0] = r.read(5) as u8;
                g.table_select[1] = r.read(5) as u8;
                for gain in g.subblock_gain.iter_mut() {
                    *gain = r.read(3) as i32;
                }
                if g.block_type == 0 {
                    return Err("window switching with block type 0");
                }
            } else {
                for table in g.table_select.iter_mut() {
                    *table = r.read(5) as u8;
                }
                g.region0_count = r.read(4) as usize;
                g.region1_count = r.read(3) as usize;
            }
            if !lsf {
                g.preflag = r.flag();
            }
            g.scalefac_scale = r.flag();
            g.count1table_select = r.flag();
        }
    }
    Ok(si)
}

/// One channel's scale factors for one granule.
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct Scalefactors {
    pub long: [u8; 22],
    /// By band, then window.
    pub short: [[u8; 3]; 13],
    /// MPEG-2 intensity stereo, right channel: bands holding the largest
    /// value their size allows, which means "not intensity coded" (0 for
    /// a group sent with no bits; as Windows' Fraunhofer decoder reads it).
    pub illegal_long: [bool; 22],
    pub illegal_short: [[bool; 3]; 13],
}

fn read_scalefactors_mpeg1(
    r: &mut BitReader,
    g: &Granule,
    scfsi: &[bool; 4],
    gr: usize,
    previous: &Scalefactors,
) -> Scalefactors {
    let mut sf = Scalefactors::default();
    let slen1 = SLEN[0][g.scalefac_compress];
    let slen2 = SLEN[1][g.scalefac_compress];
    if g.is_short() {
        if g.mixed_block {
            for v in sf.long.iter_mut().take(8) {
                *v = r.read(slen1) as u8;
            }
            for band in &mut sf.short[3..6] {
                for v in band.iter_mut() {
                    *v = r.read(slen1) as u8;
                }
            }
        } else {
            for band in &mut sf.short[0..6] {
                for v in band.iter_mut() {
                    *v = r.read(slen1) as u8;
                }
            }
        }
        for band in &mut sf.short[6..12] {
            for v in band.iter_mut() {
                *v = r.read(slen2) as u8;
            }
        }
    } else {
        const GROUPS: [(usize, usize); 4] = [(0, 6), (6, 11), (11, 16), (16, 21)];
        for (group, &(from, to)) in GROUPS.iter().enumerate() {
            if gr == 1 && scfsi[group] {
                sf.long[from..to].copy_from_slice(&previous.long[from..to]);
            } else {
                let len = if group < 2 { slen1 } else { slen2 };
                for v in &mut sf.long[from..to] {
                    *v = r.read(len) as u8;
                }
            }
        }
    }
    sf
}

/// MPEG-2 scale factors (ISO 13818-3 section 2.4.3.2). Sets the granule's
/// `preflag`, which MPEG-2 doesn't transmit.
fn read_scalefactors_lsf(
    r: &mut BitReader,
    g: &mut Granule,
    intensity_right: bool,
) -> Scalefactors {
    let c = g.scalefac_compress;
    let (slen, table, preflag) = if intensity_right {
        let c = c >> 1;
        if c < 180 {
            ([c / 36, (c % 36) / 6, (c % 36) % 6, 0], 3, false)
        } else if c < 244 {
            let c = c - 180;
            ([(c % 64) >> 4, (c % 16) >> 2, c % 4, 0], 4, false)
        } else {
            let c = c - 244;
            ([c / 3, c % 3, 0, 0], 5, false)
        }
    } else if c < 400 {
        ([(c >> 4) / 5, (c >> 4) % 5, (c % 16) >> 2, c % 4], 0, false)
    } else if c < 500 {
        let c = c - 400;
        ([(c >> 2) / 5, (c >> 2) % 5, c % 4, 0], 1, false)
    } else {
        let c = c - 500;
        ([c / 3, c % 3, 0, 0], 2, true)
    };
    g.preflag = preflag;
    let kind = match (g.is_short(), g.mixed_block) {
        (false, _) => 0,
        (true, false) => 1,
        (true, true) => 2,
    };
    let mut values = [0u8; 39];
    let mut illegal = [false; 39];
    let mut n = 0;
    for (&count, &len) in NR_OF_SFB[table][kind].iter().zip(&slen) {
        let len = len as u32;
        for _ in 0..count {
            let v = r.read(len);
            values[n] = v as u8;
            illegal[n] = v == (1 << len) - 1;
            n += 1;
        }
    }
    let mut sf = Scalefactors::default();
    let mut k = 0;
    let first_short = match kind {
        0 => {
            sf.long[..21].copy_from_slice(&values[..21]);
            sf.illegal_long[..21].copy_from_slice(&illegal[..21]);
            return sf;
        }
        1 => 0,
        _ => {
            sf.long[..6].copy_from_slice(&values[..6]);
            sf.illegal_long[..6].copy_from_slice(&illegal[..6]);
            k = 6;
            3
        }
    };
    for sfb in first_short..12 {
        for w in 0..3 {
            sf.short[sfb][w] = values[k];
            sf.illegal_short[sfb][w] = illegal[k];
            k += 1;
        }
    }
    sf
}

/// Where the second Huffman region starts in a window-switching granule
/// (there is no third). The standard fixes `region0_count` at 8 for short
/// blocks and 7 otherwise, so the first region is 9 or 8 scale factor
/// bands, counted in the order the values are sent: long bands, then for
/// the short part every window of every band. That's line 36 for short
/// blocks (72 at 8 kHz) and for MPEG-1 mixed blocks; after 8 long bands
/// for start and stop blocks; and for MPEG-2 mixed blocks, 6 long bands
/// plus 2 windows of short band 3: line 48 at 22.05 kHz (96 at 8 kHz).
/// Found by matching Windows' Fraunhofer decoder on test streams (ffmpeg
/// and mpg123 use 36 for every short block).
fn window_switching_region1(g: &Granule, bands: &Bands, lsf: bool) -> usize {
    if g.block_type != 2 {
        return bands.long[8];
    }
    let (mut remaining, long, first_short) = if g.mixed_block {
        (8, tables::mixed_long_bands(lsf), 3)
    } else {
        (9, 0, 0)
    };
    if remaining <= long {
        return bands.long[remaining];
    }
    remaining -= long;
    let mut line = 3 * bands.short[first_short];
    for sfb in first_short..13 {
        let width = bands.short[sfb + 1] - bands.short[sfb];
        for _ in 0..3 {
            if remaining == 0 {
                return line;
            }
            line += width;
            remaining -= 1;
        }
    }
    line
}

/// Decodes the Huffman-coded values of one granule and channel: pairs in
/// up to three regions, each with its own table, then quadruples of -1, 0
/// and 1 until the granule's bits run out.
fn read_spectrum(
    r: &mut BitReader,
    end: usize,
    g: &Granule,
    bands: &Bands,
    lsf: bool,
    out: &mut [i32; LINES],
) -> Result<(), &'static str> {
    let big = (g.big_values * 2).min(LINES);
    let (region1, region2) = if g.window_switching {
        (window_switching_region1(g, bands, lsf), LINES)
    } else {
        (
            bands.long[(g.region0_count + 1).min(22)],
            bands.long[(g.region0_count + g.region1_count + 2).min(22)],
        )
    };
    let mut i = 0;
    for (region, stop) in [region1.min(big), region2.min(big), big]
        .into_iter()
        .enumerate()
    {
        let table = g.table_select[region];
        while i < stop {
            let (x, y) = huffman::read_pair(r, table)?;
            out[i] = x;
            out[i + 1] = y;
            i += 2;
        }
    }
    if r.position() > end {
        return Err("Huffman data runs past the granule's length");
    }
    while i + 4 <= LINES && r.position() < end {
        let quad = huffman::read_quad(r, g.count1table_select);
        if r.position() > end {
            // The last quadruple ran past the end: it isn't part of the
            // granule (some encoders leave stray bits).
            break;
        }
        out[i..i + 4].copy_from_slice(&quad);
        i += 4;
    }
    Ok(())
}

/// Requantization: `sign(v) |v|^(4/3) 2^((global_gain - 210) / 4)`, less
/// the scale factor (in steps of 2^-0.5 or 2^-1 with `scalefac_scale`),
/// `preflag`'s fixed boost, and for short windows 2^-2 per subblock gain
/// step.
fn requantize(
    g: &Granule,
    sf: &Scalefactors,
    bands: &Bands,
    lsf: bool,
    values: &[i32; LINES],
    xr: &mut [f64; LINES],
) {
    let pow43 = &computed().pow43;
    let step = if g.scalefac_scale { 4 } else { 2 };
    let base = g.global_gain - 210;
    let mut band = |from: usize, to: usize, quarters: i32| {
        let gain = pow2_quarter(quarters);
        for i in from..to {
            let v = values[i];
            if v != 0 {
                let magnitude = pow43[v.unsigned_abs() as usize] * gain;
                xr[i] = if v < 0 { -magnitude } else { magnitude };
            }
        }
    };
    let long_band = |sfb: usize| {
        let pre = if g.preflag { PRETAB[sfb] } else { 0 };
        base - step * (i32::from(sf.long[sfb]) + pre)
    };
    let first_short = if !g.is_short() {
        for sfb in 0..22 {
            band(bands.long[sfb], bands.long[sfb + 1], long_band(sfb));
        }
        return;
    } else if g.mixed_block {
        for sfb in 0..tables::mixed_long_bands(lsf) {
            band(bands.long[sfb], bands.long[sfb + 1], long_band(sfb));
        }
        3
    } else {
        0
    };
    for sfb in first_short..13 {
        let width = bands.short[sfb + 1] - bands.short[sfb];
        let start = 3 * bands.short[sfb];
        for w in 0..3 {
            let quarters = base - 8 * g.subblock_gain[w] - step * i32::from(sf.short[sfb][w]);
            band(start + w * width, start + (w + 1) * width, quarters);
        }
    }
}

/// The left and right gains for an intensity-coded band.
fn intensity_factors(lsf: bool, position: u8, scale: usize) -> (f64, f64) {
    if lsf {
        // MPEG-2: powers of 2^-1/4 (or 2^-1/2 with the right channel's
        // `scalefac_compress` odd) applied to one side.
        let io: f64 = if scale == 1 {
            FRAC_1_SQRT_2
        } else {
            2f64.powf(-0.25)
        };
        let p = i32::from(position);
        if p == 0 {
            (1.0, 1.0)
        } else if p % 2 == 1 {
            (io.powi((p + 1) / 2), 1.0)
        } else {
            (1.0, io.powi(p / 2))
        }
    } else {
        // MPEG-1: ratio tan(position * pi / 12) between left and right.
        let (s, c) = (f64::from(position) * PI / 12.0).sin_cos();
        (s / (s + c), c / (s + c))
    }
}

/// Intensity stereo for one granule: which bands are coded as the left
/// channel with a balance, and those bands' new left and right values.
struct Intensity<'a> {
    sf: &'a Scalefactors,
    lsf: bool,
    scale: usize,
    /// Lines already decoded this way (mid/side leaves them alone).
    done: [bool; LINES],
}

impl Intensity<'_> {
    fn band(
        &mut self,
        xr: &mut [[f64; LINES]; 2],
        from: usize,
        to: usize,
        position: u8,
        illegal: bool,
    ) {
        // MPEG-1 positions run 0 to 6; 7 and up mean "not intensity coded".
        let illegal = if self.lsf { illegal } else { position >= 7 };
        if illegal {
            return;
        }
        let (left, right) = intensity_factors(self.lsf, position, self.scale);
        let [l, r] = xr;
        let lines = l[from..to].iter_mut().zip(&mut r[from..to]);
        for ((l, r), done) in lines.zip(&mut self.done[from..to]) {
            let v = *l;
            *l = v * left;
            *r = v * right;
            *done = true;
        }
    }

    /// Long bands `0..count`: those above the right channel's last nonzero
    /// line.
    ///
    /// The top band (21, and short band 12) has no scale factor, so its
    /// position is 0: all of the left channel's value goes right in MPEG-1,
    /// both sides keep it in MPEG-2. That's how Windows' Fraunhofer decoder
    /// reads it (and libmad); the standard's sample decoder and mpg123
    /// reuse band 20's position instead, which disagreed with Fraunhofer's
    /// on test streams by up to 28 steps of 16-bit output.
    fn long_bands(&mut self, xr: &mut [[f64; LINES]; 2], bands: &Bands, count: usize) {
        let end = bands.long[count];
        let start = match (0..end).rev().find(|&i| xr[1][i] != 0.0) {
            None => 0,
            Some(last) => (0..count)
                .find(|&sfb| bands.long[sfb + 1] > last)
                .map_or(count, |sfb| sfb + 1),
        };
        for sfb in start..count {
            let (position, illegal) = (self.sf.long[sfb], self.sf.illegal_long[sfb]);
            self.band(xr, bands.long[sfb], bands.long[sfb + 1], position, illegal);
        }
    }
}

/// Joint stereo: intensity-coded bands first (from the left channel's
/// values), then mid/side for every other line: L = (M + S) / sqrt 2,
/// R = (M - S) / sqrt 2. The right channel's block type decides the bands.
fn stereo(
    header: &FrameHeader,
    right: &Granule,
    sf: &Scalefactors,
    bands: &Bands,
    lsf: bool,
    xr: &mut [[f64; LINES]; 2],
) {
    let mut intensity = Intensity {
        sf,
        lsf,
        scale: right.scalefac_compress & 1,
        done: [false; LINES],
    };
    if header.intensity_stereo() {
        if right.is_short() {
            let first = if right.mixed_block { 3 } else { 0 };
            let mut highest_start = first;
            for w in 0..3 {
                let window_band = |sfb: usize| {
                    let width = bands.short[sfb + 1] - bands.short[sfb];
                    let from = 3 * bands.short[sfb] + w * width;
                    (from, from + width)
                };
                let start = (first..13)
                    .rev()
                    .find(|&sfb| {
                        let (from, to) = window_band(sfb);
                        xr[1][from..to].iter().any(|&v| v != 0.0)
                    })
                    .map_or(first, |sfb| sfb + 1);
                highest_start = highest_start.max(start);
                for sfb in start..13 {
                    // Band 12 has no scale factor: position 0 (see
                    // `Intensity::long_bands`).
                    let (from, to) = window_band(sfb);
                    let (position, illegal) = (sf.short[sfb][w], sf.illegal_short[sfb][w]);
                    intensity.band(xr, from, to, position, illegal);
                }
            }
            // A mixed block's long part is intensity coded only when the
            // right channel's short part is silent.
            if right.mixed_block && highest_start <= 3 {
                intensity.long_bands(xr, bands, tables::mixed_long_bands(lsf));
            }
        } else {
            intensity.long_bands(xr, bands, 22);
        }
    }
    if header.ms_stereo() {
        let [l, r] = xr;
        for ((l, r), done) in l.iter_mut().zip(r.iter_mut()).zip(&intensity.done) {
            if !done {
                let (m, s) = (*l, *r);
                *l = (m + s) * FRAC_1_SQRT_2;
                *r = (m - s) * FRAC_1_SQRT_2;
            }
        }
    }
}

/// Short blocks arrive band by band, window by window; the IMDCT wants each
/// subband's 18 lines as the three windows' values interleaved
/// (frequency f of window w at 3f + w).
fn reorder(g: &Granule, bands: &Bands, xr: &mut [f64; LINES]) {
    if !g.is_short() {
        return;
    }
    let first = if g.mixed_block { 3 } else { 0 };
    let start = 3 * bands.short[first];
    let mut out = [0.0f64; LINES];
    for sfb in first..13 {
        let from = bands.short[sfb];
        let width = bands.short[sfb + 1] - from;
        for w in 0..3 {
            for k in 0..width {
                out[3 * (from + k) + w] = xr[3 * from + w * width + k];
            }
        }
    }
    xr[start..].copy_from_slice(&out[start..]);
}

/// How many subbands a mixed block's long part covers: 2 (36 lines), or 4
/// at 8 kHz, where MPEG-2.5's six long bands reach line 72.
fn mixed_long_subbands(bands: &Bands, lsf: bool) -> usize {
    bands.long[tables::mixed_long_bands(lsf)] / 18
}

/// Alias reduction: eight butterflies across each boundary between long
/// subbands: all 31, none for short blocks, and in a mixed block those
/// inside its long part (one; three at 8 kHz, as Windows' Fraunhofer
/// decoder does it, where ffmpeg does only the first).
fn antialias(g: &Granule, bands: &Bands, lsf: bool, c: &Computed, xr: &mut [f64; LINES]) {
    let boundaries = match (g.is_short(), g.mixed_block) {
        (false, _) => 31,
        (true, true) => mixed_long_subbands(bands, lsf) - 1,
        (true, false) => return,
    };
    for sb in 1..=boundaries {
        for i in 0..8 {
            let (lower, upper) = (18 * sb - 1 - i, 18 * sb + i);
            let (a, b) = (xr[lower], xr[upper]);
            xr[lower] = a * c.alias_cs[i] - b * c.alias_ca[i];
            xr[upper] = b * c.alias_cs[i] + a * c.alias_ca[i];
        }
    }
}

/// 36-point IMDCT, `x[i] = sum X[k] cos(pi/72 (2i + 19)(2k + 1))`, windowed.
/// Outputs 9..18 mirror 0..9 negated and 27..36 mirror 18..27.
fn imdct_long(c: &Computed, input: &[f64], block_type: u8, z: &mut [f64; 36]) {
    let w = &c.long_windows[usize::from(block_type)];
    for i in 0..9 {
        let a: f64 = c.imdct36[i].iter().zip(input).map(|(c, x)| c * x).sum();
        let b: f64 = c.imdct36[9 + i].iter().zip(input).map(|(c, x)| c * x).sum();
        z[i] = a * w[i];
        z[17 - i] = -a * w[17 - i];
        z[18 + i] = b * w[18 + i];
        z[35 - i] = b * w[35 - i];
    }
}

/// Three 12-point IMDCTs (`cos(pi/24 (2i + 7)(2k + 1))`), each windowed and
/// overlapped at 6, 12 and 18 of the 36 outputs.
fn imdct_short(c: &Computed, input: &[f64], z: &mut [f64; 36]) {
    for w in 0..3 {
        let x: [f64; 6] = std::array::from_fn(|k| input[w + 3 * k]);
        let mut y = [0.0f64; 12];
        for i in 0..3 {
            let a: f64 = c.imdct12[i].iter().zip(&x).map(|(c, x)| c * x).sum();
            let b: f64 = c.imdct12[3 + i].iter().zip(&x).map(|(c, x)| c * x).sum();
            y[i] = a;
            y[5 - i] = -a;
            y[6 + i] = b;
            y[11 - i] = b;
        }
        for (i, v) in y.iter().enumerate() {
            z[6 + 6 * w + i] += v * c.short_window[i];
        }
    }
}

/// The hybrid filterbank's second half: each subband's 18 lines through the
/// IMDCT and window, overlapped with the previous granule's second half,
/// then every odd sample of every odd subband negated (frequency
/// inversion). Gives 18 time slots of 32 subband samples.
fn hybrid(
    g: &Granule,
    bands: &Bands,
    lsf: bool,
    xr: &[f64; LINES],
    overlap: &mut [[f64; 18]; 32],
    slots: &mut [[f64; 32]; 18],
) {
    let c = computed();
    let long_subbands = if g.is_short() && g.mixed_block {
        mixed_long_subbands(bands, lsf)
    } else {
        0
    };
    for (sb, previous) in overlap.iter_mut().enumerate() {
        let input = &xr[18 * sb..18 * sb + 18];
        let mut z = [0.0f64; 36];
        if input.iter().any(|&v| v != 0.0) {
            if g.is_short() && sb >= long_subbands {
                imdct_short(c, input, &mut z);
            } else {
                // A mixed block's long subbands use the normal window.
                let block_type = if g.is_short() { 0 } else { g.block_type };
                imdct_long(c, input, block_type, &mut z);
            }
        }
        for t in 0..18 {
            let mut v = z[t] + previous[t];
            previous[t] = z[t + 18];
            if sb % 2 == 1 && t % 2 == 1 {
                v = -v;
            }
            slots[t][sb] = v;
        }
    }
}

/// Everything Layer III carries from frame to frame.
pub(crate) struct Layer3 {
    reservoir: Vec<u8>,
    main: Vec<u8>,
    overlap: [[[f64; 18]; 32]; 2],
    synth: [Synth; 2],
}

impl Layer3 {
    pub fn new() -> Box<Layer3> {
        Box::new(Layer3 {
            reservoir: Vec::with_capacity(MAX_RESERVOIR + 2048),
            main: Vec::with_capacity(MAX_RESERVOIR + 2048),
            overlap: [[[0.0; 18]; 32]; 2],
            synth: [Synth::default(), Synth::default()],
        })
    }

    /// Decodes one frame (`frame` is all of it, header first) into `pcm`:
    /// per channel, the frame's samples. Returns the first problem met;
    /// damaged parts decode as silence and the filterbank carries on.
    pub fn decode(
        &mut self,
        header: &FrameHeader,
        frame: &[u8],
        pcm: &mut [Vec<f32>; 2],
    ) -> Option<Problem> {
        let channels = header.channels();
        let lsf = header.is_lsf();
        let granules = if lsf { 1 } else { 2 };
        for out in pcm.iter_mut() {
            out.clear();
            out.resize(LINES * granules, 0.0);
        }
        let mut problem = None;
        let mut note = |p: Problem| {
            if problem.is_none() {
                problem = Some(p);
            }
        };

        let side_start = header.side_info_start();
        let side_end = side_start + header.side_info_length();
        let side = match frame.get(side_start..side_end) {
            Some(bytes) => {
                if header.protected {
                    let stored = u16::from_be_bytes([frame[4], frame[5]]);
                    if frame_crc(&frame[..4], bytes) != stored {
                        note(Problem::Checksum);
                    }
                }
                match read_side_info(header, bytes) {
                    Ok(si) => Some(si),
                    Err(e) => {
                        note(Problem::Damaged(e));
                        None
                    }
                }
            }
            None => {
                note(Problem::Damaged(
                    "the frame is shorter than its side information",
                ));
                None
            }
        };

        // The frame's data starts `main_data_begin` bytes back in the data
        // of earlier frames (the bit reservoir).
        let own = frame.get(side_end..).unwrap_or(&[]);
        self.main.clear();
        let mut usable = false;
        if let Some(si) = &side {
            if si.main_data_begin <= self.reservoir.len() {
                let from = self.reservoir.len() - si.main_data_begin;
                self.main.extend_from_slice(&self.reservoir[from..]);
                self.main.extend_from_slice(own);
                usable = true;
            } else {
                note(Problem::MissingReservoir);
            }
        }
        self.reservoir.extend_from_slice(own);
        if self.reservoir.len() > MAX_RESERVOIR {
            let excess = self.reservoir.len() - MAX_RESERVOIR;
            self.reservoir.drain(..excess);
        }

        let bands = tables::bands_for(header);
        let c = computed();
        let available = self.main.len() * 8;
        let mut scalefactors = [[Scalefactors::default(); 2]; 2];
        let mut bit = 0;
        for gr in 0..granules {
            let mut xr = [[0.0f64; LINES]; 2];
            let mut g = [Granule::default(); 2];
            if let (true, Some(si)) = (usable, &side) {
                for ch in 0..channels {
                    g[ch] = si.granules[gr][ch];
                    let start = bit;
                    let end = start + g[ch].part2_3_length;
                    bit = end;
                    if end > available {
                        note(Problem::Damaged(
                            "the side information claims more data than the frame has",
                        ));
                        continue;
                    }
                    let mut r = BitReader::at(&self.main, start);
                    let sf = if lsf {
                        let intensity_right = ch == 1 && header.intensity_stereo();
                        read_scalefactors_lsf(&mut r, &mut g[ch], intensity_right)
                    } else {
                        let previous = scalefactors[0][ch];
                        read_scalefactors_mpeg1(&mut r, &g[ch], &si.scfsi[ch], gr, &previous)
                    };
                    scalefactors[gr][ch] = sf;
                    if r.position() > end {
                        note(Problem::Damaged(
                            "scale factors run past the granule's data",
                        ));
                        continue;
                    }
                    let mut values = [0i32; LINES];
                    match read_spectrum(&mut r, end, &g[ch], bands, lsf, &mut values) {
                        Ok(()) => requantize(&g[ch], &sf, bands, lsf, &values, &mut xr[ch]),
                        Err(e) => note(Problem::Damaged(e)),
                    }
                }
                if channels == 2 && header.mode == ChannelMode::JointStereo {
                    stereo(header, &g[1], &scalefactors[gr][1], bands, lsf, &mut xr);
                }
            }
            for ch in 0..channels {
                reorder(&g[ch], bands, &mut xr[ch]);
                antialias(&g[ch], bands, lsf, c, &mut xr[ch]);
                let mut slots = [[0.0f64; 32]; 18];
                hybrid(
                    &g[ch],
                    bands,
                    lsf,
                    &xr[ch],
                    &mut self.overlap[ch],
                    &mut slots,
                );
                let out = &mut pcm[ch][gr * LINES..(gr + 1) * LINES];
                for (slot, samples) in slots.iter().zip(out.chunks_exact_mut(32)) {
                    self.synth[ch].run(slot, samples);
                }
            }
        }
        problem
    }
}

#[cfg(test)]
pub(crate) mod reference {
    //! The hybrid filterbank written straight from the formulas, for tests.

    use std::f64::consts::PI;

    /// `x[i] = sum over k < n/2 of X[k] cos(pi/(2n) (2i + 1 + n/2)(2k + 1))`.
    pub fn imdct(input: &[f64], n: usize) -> Vec<f64> {
        (0..n)
            .map(|i| {
                (0..n / 2)
                    .map(|k| {
                        input[k]
                            * (PI / (2 * n) as f64
                                * (2 * i + 1 + n / 2) as f64
                                * (2 * k + 1) as f64)
                                .cos()
                    })
                    .sum()
            })
            .collect()
    }

    /// The standard's window for block type `bt` at sample `i` of 36.
    pub fn window(bt: u8, i: usize) -> f64 {
        let long = (PI / 36.0 * (i as f64 + 0.5)).sin();
        match bt {
            0 => long,
            1 => match i {
                0..=17 => long,
                18..=23 => 1.0,
                24..=29 => (PI / 12.0 * (i as f64 - 18.0 + 0.5)).sin(),
                _ => 0.0,
            },
            3 => match i {
                0..=5 => 0.0,
                6..=11 => (PI / 12.0 * (i as f64 - 6.0 + 0.5)).sin(),
                12..=17 => 1.0,
                _ => long,
            },
            _ => (PI / 12.0 * (i as f64 + 0.5)).sin(),
        }
    }

    /// One subband's 36 windowed outputs for a block type.
    pub fn subband(input: &[f64], bt: u8) -> [f64; 36] {
        let mut z = [0.0; 36];
        if bt == 2 {
            for w in 0..3 {
                let x: Vec<f64> = (0..6).map(|k| input[w + 3 * k]).collect();
                let y = imdct(&x, 12);
                for i in 0..12 {
                    z[6 + 6 * w + i] += y[i] * window(2, i);
                }
            }
        } else {
            let y = imdct(input, 36);
            for i in 0..36 {
                z[i] = y[i] * window(bt, i);
            }
        }
        z
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn imdct_matches_the_formula() {
        let c = computed();
        let input: Vec<f64> = (0..18).map(|k| ((k * 7 + 3) % 11) as f64 - 5.0).collect();
        for bt in [0u8, 1, 2, 3] {
            let mut z = [0.0; 36];
            if bt == 2 {
                imdct_short(c, &input, &mut z);
            } else {
                imdct_long(c, &input, bt, &mut z);
            }
            let expected = reference::subband(&input, bt);
            for i in 0..36 {
                assert!((z[i] - expected[i]).abs() < 1e-9, "type {bt} [{i}]");
            }
        }
    }

    /// The boundaries Windows' Fraunhofer decoder uses (each confirmed on
    /// a test stream decoded by both).
    #[test]
    fn window_switching_regions() {
        let header = |bytes: [u8; 4]| FrameHeader::parse(bytes).unwrap();
        let mpeg1 = header([0xFF, 0xFB, 0x90, 0x00]); // 44.1 kHz
        let mpeg2 = header([0xFF, 0xF3, 0x80, 0x00]); // 22.05 kHz
        let mpeg25 = header([0xFF, 0xE3, 0x88, 0x00]); // 8 kHz
        let region = |h: &FrameHeader, block_type: u8, mixed_block: bool| {
            let g = Granule {
                window_switching: true,
                block_type,
                mixed_block,
                ..Granule::default()
            };
            window_switching_region1(&g, tables::bands_for(h), h.is_lsf())
        };
        assert_eq!(region(&mpeg1, 2, false), 36);
        assert_eq!(region(&mpeg1, 2, true), 36);
        assert_eq!(region(&mpeg1, 1, false), 36);
        assert_eq!(region(&mpeg2, 2, false), 36);
        assert_eq!(region(&mpeg2, 2, true), 48);
        assert_eq!(region(&mpeg2, 3, false), 54);
        assert_eq!(region(&mpeg25, 2, false), 72);
        assert_eq!(region(&mpeg25, 2, true), 96);
        assert_eq!(region(&mpeg25, 1, false), 108);
    }

    #[test]
    fn mpeg2_scale_factor_sizes() {
        // scalefac_compress 0x1F3 = 499 -> table 1: (99 >> 2) = 24 -> 4, 4;
        // 99 % 4 = 3.
        let bytes = [0u8; 16];
        let mut g = Granule {
            scalefac_compress: 499,
            ..Granule::default()
        };
        let mut r = BitReader::new(&bytes);
        read_scalefactors_lsf(&mut r, &mut g, false);
        // 6 x 4 + 5 x 4 + 7 x 3 + 3 x 0 bits.
        assert_eq!(r.position(), 24 + 20 + 21);
        assert!(!g.preflag);
        // 511: table 2, slen 3 and 2, preflag on.
        g.scalefac_compress = 511;
        let mut r = BitReader::new(&bytes);
        read_scalefactors_lsf(&mut r, &mut g, false);
        assert_eq!(r.position(), 11 * 3 + 10 * 2);
        assert!(g.preflag);
        // Intensity stereo's right channel, short blocks: 2 * 179 + 1 ->
        // table 3: 179 / 36 = 4, 35 / 6 = 5, 35 % 6 = 5.
        g.scalefac_compress = 359;
        g.window_switching = true;
        g.block_type = 2;
        let mut r = BitReader::new(&bytes);
        let sf = read_scalefactors_lsf(&mut r, &mut g, true);
        assert_eq!(r.position(), 12 * 4 + 12 * 5 + 12 * 5);
        // All zeros: not the largest value, so intensity coded.
        assert!(!sf.illegal_short[0][0]);
        // Table 5 (2 * 250 + 1): no preflag for the right channel (mpg123
        // sets one; Windows' Fraunhofer decoder doesn't, checked on a test
        // stream).
        g.scalefac_compress = 501;
        g.preflag = true;
        let mut r = BitReader::new(&bytes);
        read_scalefactors_lsf(&mut r, &mut g, true);
        assert!(!g.preflag);
    }
}

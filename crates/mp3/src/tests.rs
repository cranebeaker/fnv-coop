//! Frames assembled bit by bit (side information, scale factors, Huffman
//! codes), decoded by the crate and by a slow decoder written straight from
//! the standard's formulas: requantization, mid/side and intensity stereo,
//! reordering, alias reduction, the IMDCT and its windows, overlap-add and
//! the synthesis flowchart. The two must agree sample for sample.

use std::f64::consts::PI;

use crate::header::{frame_crc, ChannelMode, FrameHeader, Version};
use crate::huffman;
use crate::layer3::reference as hybrid;
use crate::synth::reference::Flowchart;
use crate::tables::{self, Bands, NR_OF_SFB, PRETAB, SLEN};
use crate::{decode_all, Decoder, Error, Problem, DECODER_DELAY};

// ---- Writing bits ----

#[derive(Default, Clone)]
struct BitWriter {
    bytes: Vec<u8>,
    bits: usize,
}

impl BitWriter {
    fn put(&mut self, value: u32, n: u32) {
        for i in (0..n).rev() {
            if self.bits % 8 == 0 {
                self.bytes.push(0);
            }
            if (value >> i) & 1 == 1 {
                *self.bytes.last_mut().unwrap() |= 0x80 >> (self.bits % 8);
            }
            self.bits += 1;
        }
    }

    fn append(&mut self, other: &BitWriter) {
        for i in 0..other.bits {
            self.put(u32::from((other.bytes[i / 8] >> (7 - i % 8)) & 1), 1);
        }
    }
}

// ---- What a frame holds ----

/// One channel of one granule, as an encoder chose it.
#[derive(Clone)]
struct Channel {
    block_type: u8,
    mixed: bool,
    global_gain: i32,
    scalefac_compress: usize,
    scalefac_scale: bool,
    /// MPEG-1 only (MPEG-2 derives it from `scalefac_compress`).
    preflag: bool,
    subblock_gain: [i32; 3],
    long_sf: [u8; 22],
    short_sf: [[u8; 3]; 13],
    /// Quantized values in the order they're transmitted.
    values: Vec<i32>,
    tables: [u8; 3],
    region0_count: usize,
    region1_count: usize,
    count1_b: bool,
}

impl Channel {
    fn silent() -> Channel {
        Channel {
            block_type: 0,
            mixed: false,
            global_gain: 0,
            scalefac_compress: 0,
            scalefac_scale: false,
            preflag: false,
            subblock_gain: [0; 3],
            long_sf: [0; 22],
            short_sf: [[0; 3]; 13],
            values: vec![0; 576],
            tables: [0; 3],
            region0_count: 0,
            region1_count: 0,
            count1_b: false,
        }
    }
}

struct Spec {
    header: FrameHeader,
    scfsi: [[bool; 4]; 2],
    /// By granule, then channel.
    granules: Vec<Vec<Channel>>,
}

fn header(version: Version, rate: u8, mode: ChannelMode, extension: u8) -> FrameHeader {
    FrameHeader {
        version,
        layer: 3,
        protected: false,
        bitrate_index: 1,
        sample_rate_index: rate,
        padding: false,
        private: false,
        mode,
        mode_extension: extension,
        copyright: false,
        original: true,
        emphasis: 0,
    }
}

struct Rng(u32);

impl Rng {
    fn below(&mut self, n: u32) -> u32 {
        self.0 = self.0.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        (self.0 >> 8) % n
    }

    fn signed(&mut self, max: i32) -> i32 {
        self.below(2 * max as u32 + 1) as i32 - max
    }
}

/// The largest value a table can code.
fn largest(table: u8) -> i32 {
    match table {
        0 => 0,
        16..=31 => 14 + (1 << huffman::linbits(table)),
        t => huffman::code_table(t).unwrap().size as i32 - 1,
    }
}

/// Where the second and third Huffman regions start. With window switching
/// the first region is 9 bands for short blocks, 8 otherwise, counted as
/// they're sent: long bands (a mixed block's end where short band 3
/// starts), then each window of each short band.
fn regions(bands: &Bands, c: &Channel) -> (usize, usize) {
    if c.block_type == 0 {
        return (
            bands.long[(c.region0_count + 1).min(22)],
            bands.long[(c.region0_count + c.region1_count + 2).min(22)],
        );
    }
    let mut ends: Vec<usize> = Vec::new();
    if c.block_type == 2 {
        let switch = 3 * bands.short[3];
        let first = if c.mixed {
            ends.extend(bands.long[1..].iter().take_while(|&&e| e <= switch));
            3
        } else {
            0
        };
        for sfb in first..13 {
            let width = bands.short[sfb + 1] - bands.short[sfb];
            ends.extend((1..=3).map(|w| 3 * bands.short[sfb] + w * width));
        }
    } else {
        ends.extend(&bands.long[1..]);
    }
    let count = if c.block_type == 2 && !c.mixed { 9 } else { 8 };
    (ends[count - 1], 576)
}

/// Random values: up to `max` (as far as each region's table allows)
/// below line `big`, then -1, 0 or 1 up to line `top`.
fn fill(rng: &mut Rng, bands: &Bands, c: &mut Channel, big: usize, top: usize, max: i32) {
    let (r1, r2) = regions(bands, c);
    for i in 0..576 {
        let table = c.tables[if i < r1 {
            0
        } else if i < r2 {
            1
        } else {
            2
        }];
        let limit = if i < big {
            max.min(largest(table))
        } else if i < top {
            1.min(largest(table))
        } else {
            0
        };
        c.values[i] = rng.signed(limit);
    }
}

// ---- Encoding ----

fn put_pair(w: &mut BitWriter, table: u8, x: i32, y: i32) {
    assert!(
        x.abs().max(y.abs()) <= largest(table),
        "table {table}: {x} {y}"
    );
    if table == 0 {
        return;
    }
    let t = huffman::code_table(table).unwrap();
    let linbits = huffman::linbits(table);
    let coded = |v: i32| {
        if linbits > 0 {
            v.unsigned_abs().min(15) as usize
        } else {
            v.unsigned_abs() as usize
        }
    };
    let i = coded(x) * t.size + coded(y);
    w.put(u32::from(t.codes[i]), u32::from(t.lens[i]));
    for v in [x, y] {
        if linbits > 0 && coded(v) == 15 {
            w.put((v.abs() - 15) as u32, linbits);
        }
        if v != 0 {
            w.put(u32::from(v < 0), 1);
        }
    }
}

fn put_quad(w: &mut BitWriter, table_b: bool, q: &[i32]) {
    assert!(q.iter().all(|v| v.abs() <= 1));
    let index = q
        .iter()
        .fold(0usize, |n, &v| (n << 1) | usize::from(v != 0));
    if table_b {
        w.put(15 - index as u32, 4);
    } else {
        let t = huffman::quad_a();
        w.put(u32::from(t.codes[index]), u32::from(t.lens[index]));
    }
    for &v in q {
        if v != 0 {
            w.put(u32::from(v < 0), 1);
        }
    }
}

/// The Huffman part; returns `big_values`.
fn put_spectrum(w: &mut BitWriter, bands: &Bands, c: &Channel) -> usize {
    let v = &c.values;
    let big = v
        .iter()
        .rposition(|x| x.abs() > 1)
        .map_or(0, |i| (i / 2 + 1) * 2);
    let last = v.iter().rposition(|&x| x != 0).map_or(0, |i| i + 1);
    let (r1, r2) = regions(bands, c);
    for i in (0..big).step_by(2) {
        let region = if i < r1 {
            0
        } else if i < r2 {
            1
        } else {
            2
        };
        put_pair(w, c.tables[region], v[i], v[i + 1]);
    }
    let mut i = big;
    while i < last {
        put_quad(w, c.count1_b, &v[i..i + 4]);
        i += 4;
    }
    big / 2
}

fn put_checked(w: &mut BitWriter, value: u8, len: u32) {
    assert!(
        u32::from(value) < 1 << len,
        "scale factor {value} in {len} bits"
    );
    w.put(u32::from(value), len);
}

fn put_scalefactors_mpeg1(w: &mut BitWriter, c: &Channel, scfsi: &[bool; 4], gr: usize) {
    let (s1, s2) = (SLEN[0][c.scalefac_compress], SLEN[1][c.scalefac_compress]);
    if c.block_type == 2 {
        if c.mixed {
            for sfb in 0..8 {
                put_checked(w, c.long_sf[sfb], s1);
            }
        }
        let first = if c.mixed { 3 } else { 0 };
        for sfb in first..12 {
            for win in 0..3 {
                put_checked(w, c.short_sf[sfb][win], if sfb < 6 { s1 } else { s2 });
            }
        }
    } else {
        for (group, (from, to)) in [(0, 6), (6, 11), (11, 16), (16, 21)]
            .into_iter()
            .enumerate()
        {
            if gr == 1 && scfsi[group] {
                continue;
            }
            for sfb in from..to {
                put_checked(w, c.long_sf[sfb], if group < 2 { s1 } else { s2 });
            }
        }
    }
}

/// MPEG-2 scale factor sizes, group counts and preflag (ISO 13818-3).
fn lsf_layout(c: &Channel, intensity_right: bool) -> ([u32; 4], [usize; 4], bool) {
    let s = c.scalefac_compress;
    let (slen, table, preflag) = if intensity_right {
        let s = s / 2;
        if s < 180 {
            ([s / 36, s % 36 / 6, s % 36 % 6, 0], 3, false)
        } else if s < 244 {
            let s = s - 180;
            ([s % 64 / 16, s % 16 / 4, s % 4, 0], 4, false)
        } else {
            let s = s - 244;
            ([s / 3, s % 3, 0, 0], 5, false)
        }
    } else if s < 400 {
        ([s / 16 / 5, s / 16 % 5, s % 16 / 4, s % 4], 0, false)
    } else if s < 500 {
        let s = s - 400;
        ([s / 4 / 5, s / 4 % 5, s % 4, 0], 1, false)
    } else {
        let s = s - 500;
        ([s / 3, s % 3, 0, 0], 2, true)
    };
    let kind = match (c.block_type == 2, c.mixed) {
        (false, _) => 0,
        (true, false) => 1,
        (true, true) => 2,
    };
    (slen.map(|v| v as u32), NR_OF_SFB[table][kind], preflag)
}

/// MPEG-2 scale factors in transmission order, as (long?, band, window),
/// each with its size in bits.
fn lsf_order(c: &Channel, intensity_right: bool) -> Vec<((bool, usize, usize), u32)> {
    let mut order = Vec::new();
    match (c.block_type == 2, c.mixed) {
        (false, _) => order.extend((0..21).map(|sfb| (true, sfb, 0))),
        (true, mixed) => {
            let first = if mixed {
                order.extend((0..6).map(|sfb| (true, sfb, 0)));
                3
            } else {
                0
            };
            for sfb in first..12 {
                order.extend((0..3).map(|w| (false, sfb, w)));
            }
        }
    }
    let (slen, counts, _) = lsf_layout(c, intensity_right);
    let sizes = counts
        .iter()
        .zip(slen)
        .flat_map(|(&n, len)| std::iter::repeat(len).take(n));
    order.into_iter().zip(sizes).collect()
}

fn put_scalefactors_lsf(w: &mut BitWriter, c: &Channel, intensity_right: bool) {
    for ((long, sfb, win), len) in lsf_order(c, intensity_right) {
        let v = if long {
            c.long_sf[sfb]
        } else {
            c.short_sf[sfb][win]
        };
        put_checked(w, v, len);
    }
}

/// The main data of a frame, and each granule and channel's
/// (part2_3_length, big_values).
fn encode_main(spec: &Spec) -> (BitWriter, Vec<Vec<(usize, usize)>>) {
    let h = &spec.header;
    let bands = tables::bands_for(h);
    let mut main = BitWriter::default();
    let mut fields = Vec::new();
    for (gr, granule) in spec.granules.iter().enumerate() {
        let mut row = Vec::new();
        for (ch, c) in granule.iter().enumerate().take(h.channels()) {
            let mut w = BitWriter::default();
            if h.is_lsf() {
                put_scalefactors_lsf(&mut w, c, ch == 1 && h.intensity_stereo());
            } else {
                put_scalefactors_mpeg1(&mut w, c, &spec.scfsi[ch], gr);
            }
            let big = put_spectrum(&mut w, bands, c);
            row.push((w.bits, big));
            main.append(&w);
        }
        fields.push(row);
    }
    (main, fields)
}

fn side_info(spec: &Spec, fields: &[Vec<(usize, usize)>], main_data_begin: usize) -> Vec<u8> {
    let h = &spec.header;
    let lsf = h.is_lsf();
    let channels = h.channels();
    let mut w = BitWriter::default();
    if lsf {
        w.put(main_data_begin as u32, 8);
        w.put(0, if channels == 1 { 1 } else { 2 });
    } else {
        w.put(main_data_begin as u32, 9);
        w.put(0, if channels == 1 { 5 } else { 3 });
        for scfsi in spec.scfsi.iter().take(channels) {
            for &band in scfsi {
                w.put(u32::from(band), 1);
            }
        }
    }
    for (gr, granule) in spec.granules.iter().enumerate() {
        for (ch, c) in granule.iter().enumerate().take(channels) {
            let (length, big) = fields[gr][ch];
            w.put(length as u32, 12);
            w.put(big as u32, 9);
            w.put(c.global_gain as u32, 8);
            w.put(c.scalefac_compress as u32, if lsf { 9 } else { 4 });
            if c.block_type != 0 {
                w.put(1, 1);
                w.put(u32::from(c.block_type), 2);
                w.put(u32::from(c.mixed), 1);
                w.put(u32::from(c.tables[0]), 5);
                w.put(u32::from(c.tables[1]), 5);
                for &g in &c.subblock_gain {
                    w.put(g as u32, 3);
                }
            } else {
                w.put(0, 1);
                for &t in &c.tables {
                    w.put(u32::from(t), 5);
                }
                w.put(c.region0_count as u32, 4);
                w.put(c.region1_count as u32, 3);
            }
            if !lsf {
                w.put(u32::from(c.preflag), 1);
            }
            w.put(u32::from(c.scalefac_scale), 1);
            w.put(u32::from(c.count1_b), 1);
        }
    }
    assert_eq!(w.bits, h.side_info_length() * 8);
    w.bytes
}

/// Header (at the smallest bitrate that holds everything, unless it's free
/// format), checksum, side information and data, padded to the frame size.
fn assemble(mut h: FrameHeader, side: &[u8], own: &[u8], free_size: Option<usize>) -> Vec<u8> {
    let need = h.side_info_start() + side.len() + own.len();
    let len = match free_size {
        Some(size) => {
            h.bitrate_index = 0;
            assert!(need <= size);
            size
        }
        None => {
            h.bitrate_index = (1..15)
                .find(|&i| {
                    let mut t = h;
                    t.bitrate_index = i;
                    t.frame_length().unwrap() >= need
                })
                .expect("too much data for one frame");
            h.frame_length().unwrap()
        }
    };
    let mut frame = h.to_bytes().to_vec();
    if h.protected {
        let crc = frame_crc(&frame, side);
        frame.extend(crc.to_be_bytes());
    }
    frame.extend(side);
    frame.extend(own);
    frame.resize(len, 0);
    frame
}

/// A frame whose data is all its own (`main_data_begin` 0).
fn frame(spec: &Spec) -> Vec<u8> {
    let (main, fields) = encode_main(spec);
    assemble(spec.header, &side_info(spec, &fields, 0), &main.bytes, None)
}

// ---- The slow decoder ----

fn dequantize(bands: &Bands, lsf: bool, c: &Channel, preflag: bool) -> Vec<f64> {
    let multiplier = if c.scalefac_scale { 1.0 } else { 0.5 };
    let long_end = match (c.block_type == 2, c.mixed) {
        (false, _) => 576,
        (true, false) => 0,
        (true, true) => bands.long[if lsf { 6 } else { 8 }],
    };
    (0..576)
        .map(|i| {
            let v = c.values[i];
            if v == 0 {
                return 0.0;
            }
            let exponent = if i < long_end {
                let sfb = (0..22).find(|&b| bands.long[b + 1] > i).unwrap();
                let pre = if preflag { PRETAB[sfb] } else { 0 };
                0.25 * f64::from(c.global_gain - 210)
                    - multiplier * f64::from(i32::from(c.long_sf[sfb]) + pre)
            } else {
                let sfb = (0..13).find(|&b| 3 * bands.short[b + 1] > i).unwrap();
                let width = bands.short[sfb + 1] - bands.short[sfb];
                let w = (i - 3 * bands.short[sfb]) / width;
                0.25 * f64::from(c.global_gain - 210 - 8 * c.subblock_gain[w])
                    - multiplier * f64::from(c.short_sf[sfb][w])
            };
            f64::from(v.signum()) * f64::from(v.abs()).powf(4.0 / 3.0) * 2f64.powf(exponent)
        })
        .collect()
}

/// Intensity stereo's (left, right) gains, or `None` for "not coded".
fn intensity_gains(lsf: bool, right: &Channel, position: u8, illegal: bool) -> Option<(f64, f64)> {
    if lsf {
        if illegal {
            return None;
        }
        let i0 = if right.scalefac_compress % 2 == 1 {
            2f64.powf(-0.5)
        } else {
            2f64.powf(-0.25)
        };
        let p = f64::from(position);
        Some(match position {
            0 => (1.0, 1.0),
            _ if position % 2 == 1 => (i0.powf((p + 1.0) / 2.0), 1.0),
            _ => (1.0, i0.powf(p / 2.0)),
        })
    } else if position >= 7 {
        None
    } else {
        let ratio = (f64::from(position) * PI / 12.0).tan();
        Some((ratio / (1.0 + ratio), 1.0 / (1.0 + ratio)))
    }
}

fn reference_stereo(h: &FrameHeader, bands: &Bands, right: &Channel, xr: &mut [Vec<f64>]) {
    let lsf = h.is_lsf();
    let mut gains: Vec<Option<(f64, f64)>> = vec![None; 576];
    if h.intensity_stereo() {
        // MPEG-2: a position is "illegal" when it's the largest its size
        // allows.
        let sizes = lsf_order(right, true);
        let illegal = |long: bool, sfb: usize, w: usize, v: u8| {
            lsf && sizes
                .iter()
                .find(|((l, b, win), _)| *l == long && *b == sfb && (long || *win == w))
                .is_some_and(|(_, len)| u32::from(v) == (1 << len) - 1)
        };
        if right.block_type == 2 {
            assert!(!right.mixed, "not modelled here");
            for w in 0..3 {
                let window_lines = |sfb: usize| {
                    let width = bands.short[sfb + 1] - bands.short[sfb];
                    let from = 3 * bands.short[sfb] + w * width;
                    from..from + width
                };
                let start = (0..13)
                    .rev()
                    .find(|&sfb| window_lines(sfb).any(|i| xr[1][i] != 0.0))
                    .map_or(0, |sfb| sfb + 1);
                for sfb in start..13 {
                    // The top band has no scale factor: position 0.
                    let v = if sfb == 12 { 0 } else { right.short_sf[sfb][w] };
                    let g = intensity_gains(lsf, right, v, sfb < 12 && illegal(false, sfb, w, v));
                    for i in window_lines(sfb) {
                        gains[i] = g;
                    }
                }
            }
        } else {
            let start = match (0..576).rev().find(|&i| xr[1][i] != 0.0) {
                None => 0,
                Some(last) => (0..22).find(|&b| bands.long[b + 1] > last).unwrap() + 1,
            };
            for sfb in start..22 {
                let v = if sfb == 21 { 0 } else { right.long_sf[sfb] };
                let g = intensity_gains(lsf, right, v, sfb < 21 && illegal(true, sfb, 0, v));
                for gain in &mut gains[bands.long[sfb]..bands.long[sfb + 1]] {
                    *gain = g;
                }
            }
        }
    }
    for (i, gain) in gains.iter().enumerate() {
        let (l, r) = (xr[0][i], xr[1][i]);
        match gain {
            Some((gl, gr)) => {
                xr[0][i] = l * gl;
                xr[1][i] = l * gr;
            }
            None if h.ms_stereo() => {
                xr[0][i] = (l + r) / 2f64.sqrt();
                xr[1][i] = (l - r) / 2f64.sqrt();
            }
            None => {}
        }
    }
}

fn reference_reorder(bands: &Bands, c: &Channel, xr: &mut [f64]) {
    if c.block_type != 2 {
        return;
    }
    let copy = xr.to_vec();
    for sfb in if c.mixed { 3 } else { 0 }..13 {
        let start = bands.short[sfb];
        let lines = bands.short[sfb + 1] - start;
        for w in 0..3 {
            for f in 0..lines {
                xr[3 * start + w + 3 * f] = copy[3 * start + w * lines + f];
            }
        }
    }
}

/// Butterflies across the boundaries between long subbands: in a mixed
/// block, those inside its long part (`long_subbands` - 1).
fn reference_antialias(c: &Channel, long_subbands: usize, xr: &mut [f64]) {
    const C: [f64; 8] = [
        -0.6, -0.535, -0.33, -0.185, -0.095, -0.041, -0.0142, -0.0037,
    ];
    let limit = match (c.block_type == 2, c.mixed) {
        (false, _) => 31,
        (true, true) => long_subbands - 1,
        (true, false) => 0,
    };
    for sb in 0..limit {
        for (i, ci) in C.iter().enumerate() {
            let cs = 1.0 / (1.0 + ci * ci).sqrt();
            let ca = ci * cs;
            let (up, down) = (sb * 18 + 17 - i, (sb + 1) * 18 + i);
            let (bu, bd) = (xr[up], xr[down]);
            xr[up] = bu * cs - bd * ca;
            xr[down] = bd * cs + bu * ca;
        }
    }
}

struct Reference {
    overlap: Vec<[[f64; 18]; 32]>,
    synth: Vec<Flowchart>,
}

impl Reference {
    fn new() -> Reference {
        Reference {
            overlap: vec![[[0.0; 18]; 32]; 2],
            synth: vec![Flowchart::default(), Flowchart::default()],
        }
    }

    /// One frame's samples, interleaved.
    fn frame(&mut self, spec: &Spec) -> Vec<f64> {
        let h = &spec.header;
        let lsf = h.is_lsf();
        let channels = h.channels();
        let bands = tables::bands_for(h);
        let mut out = [Vec::new(), Vec::new()];
        for granule in &spec.granules {
            let mut xr: Vec<Vec<f64>> = (0..channels)
                .map(|ch| {
                    let c = &granule[ch];
                    let preflag = if lsf {
                        lsf_layout(c, ch == 1 && h.intensity_stereo()).2
                    } else {
                        c.preflag
                    };
                    dequantize(bands, lsf, c, preflag)
                })
                .collect();
            if channels == 2 && h.mode == ChannelMode::JointStereo {
                reference_stereo(h, bands, &granule[1], &mut xr);
            }
            for ch in 0..channels {
                let c = &granule[ch];
                reference_reorder(bands, c, &mut xr[ch]);
                let long_subbands = if c.block_type == 2 && c.mixed {
                    bands.long[if lsf { 6 } else { 8 }] / 18
                } else {
                    0
                };
                reference_antialias(c, long_subbands, &mut xr[ch]);
                let mut slots = [[0.0; 32]; 18];
                for sb in 0..32 {
                    let bt = if sb < long_subbands { 0 } else { c.block_type };
                    let z = hybrid::subband(&xr[ch][18 * sb..18 * sb + 18], bt);
                    for t in 0..18 {
                        let mut v = z[t] + self.overlap[ch][sb][t];
                        self.overlap[ch][sb][t] = z[t + 18];
                        if sb % 2 == 1 && t % 2 == 1 {
                            v = -v;
                        }
                        slots[t][sb] = v;
                    }
                }
                for slot in &slots {
                    out[ch].extend(self.synth[ch].run(slot));
                }
            }
        }
        (0..out[0].len())
            .flat_map(|i| (0..channels).map(move |ch| (ch, i)))
            .map(|(ch, i)| out[ch][i])
            .collect()
    }
}

fn decoded(bytes: &[u8]) -> (Vec<f32>, Vec<Option<Problem>>) {
    let mut samples = Vec::new();
    let mut problems = Vec::new();
    for frame in Decoder::new(bytes).unwrap() {
        samples.extend(frame.samples);
        problems.push(frame.problem);
    }
    (samples, problems)
}

/// Encodes the frames, decodes them, and compares with the slow decoder.
fn check(specs: &[Spec]) {
    let bytes: Vec<u8> = specs.iter().flat_map(frame).collect();
    let (got, problems) = decoded(&bytes);
    assert!(problems.iter().all(Option::is_none), "{problems:?}");
    let mut reference = Reference::new();
    let expected: Vec<f64> = specs.iter().flat_map(|s| reference.frame(s)).collect();
    assert_eq!(got.len(), expected.len());
    let loudest = expected.iter().fold(0.0f64, |m, v| m.max(v.abs()));
    assert!(loudest > 0.01, "the test signal is too quiet: {loudest}");
    for (i, (g, e)) in got.iter().zip(&expected).enumerate() {
        assert!(
            (f64::from(*g) - e).abs() < 2e-6 * loudest.max(1.0),
            "sample {i}: decoded {g}, formulas {e}"
        );
    }
}

fn random_long_sf(rng: &mut Rng, c: &mut Channel, lsf: bool, intensity_right: bool) {
    if lsf {
        for ((long, sfb, w), len) in lsf_order(c, intensity_right) {
            let v = rng.below(1 << len) as u8;
            if long {
                c.long_sf[sfb] = v;
            } else {
                c.short_sf[sfb][w] = v;
            }
        }
        return;
    }
    let (s1, s2) = (SLEN[0][c.scalefac_compress], SLEN[1][c.scalefac_compress]);
    for sfb in 0..21 {
        c.long_sf[sfb] = rng.below(1 << if sfb < 11 { s1 } else { s2 }) as u8;
    }
    for sfb in 0..12 {
        for w in 0..3 {
            c.short_sf[sfb][w] = rng.below(1 << if sfb < 6 { s1 } else { s2 }) as u8;
        }
    }
}

// ---- The tests ----

#[test]
fn silent_frames_decode_to_silence() {
    for (version, mode) in [
        (Version::Mpeg1, ChannelMode::Stereo),
        (Version::Mpeg2, ChannelMode::Mono),
        (Version::Mpeg25, ChannelMode::JointStereo),
    ] {
        let h = header(version, 0, mode, 0);
        let granules = if h.is_lsf() { 1 } else { 2 };
        let spec = Spec {
            header: h,
            scfsi: [[false; 4]; 2],
            granules: vec![vec![Channel::silent(), Channel::silent()]; granules],
        };
        let bytes: Vec<u8> = (0..3).flat_map(|_| frame(&spec)).collect();
        let (samples, problems) = decoded(&bytes);
        assert_eq!(problems, vec![None; 3]);
        assert_eq!(samples.len(), 3 * 576 * granules * h.channels());
        assert!(samples.iter().all(|&s| s == 0.0));
    }
}

#[test]
fn one_known_value() {
    // One value: 1 in line 0 at gain 2^0, so the requantized spectrum is
    // exactly 1.0 there (coded as a quadruple); then with a pair (2, -3)
    // after it, coded with table 5.
    let mut c = Channel::silent();
    c.global_gain = 210;
    c.values[0] = 1;
    let mut spec = Spec {
        header: header(Version::Mpeg1, 0, ChannelMode::Mono, 0),
        scfsi: [[false; 4]; 2],
        granules: vec![vec![c.clone()], vec![Channel::silent()]],
    };
    let bytes = frame(&spec);
    // Side info: part2_3_length is just the quadruple's code (table A,
    // "0111" for 1 0 0 0) and its sign bit.
    // (After 9 bits of main_data_begin, 5 private and 4 scfsi.)
    let word = u32::from_be_bytes(bytes[6..10].try_into().unwrap());
    assert_eq!((word >> 18) & 0xFFF, 5);
    check(std::slice::from_ref(&spec));
    c.values[2] = 2;
    c.values[3] = -3;
    c.tables = [5, 5, 5];
    spec.granules[0][0] = c;
    check(&[spec]);
}

#[test]
fn long_blocks_match_the_formulas() {
    let mut rng = Rng(7);
    let h = header(Version::Mpeg1, 0, ChannelMode::Mono, 0);
    let bands = tables::bands_for(&h);
    let mut specs = Vec::new();
    for n in 0..3 {
        let mut g0 = Channel::silent();
        g0.global_gain = 170 + n;
        g0.scalefac_compress = 13;
        g0.preflag = true;
        g0.tables = [24, 15, 5];
        g0.region0_count = 5;
        g0.region1_count = 4;
        random_long_sf(&mut rng, &mut g0, false, false);
        fill(&mut rng, bands, &mut g0, 120, 400, 40);
        let mut g1 = Channel::silent();
        g1.global_gain = 160;
        g1.scalefac_compress = 6;
        g1.scalefac_scale = true;
        g1.tables = [16, 13, 1];
        g1.region0_count = 3;
        g1.region1_count = 2;
        g1.count1_b = true;
        random_long_sf(&mut rng, &mut g1, false, false);
        // Bands sharing granule 0's scale factors (scfsi groups 0 and 2).
        g1.long_sf[0..6].copy_from_slice(&g0.long_sf[0..6]);
        g1.long_sf[11..16].copy_from_slice(&g0.long_sf[11..16]);
        fill(&mut rng, bands, &mut g1, 200, 300, 16);
        specs.push(Spec {
            header: h,
            scfsi: [[true, false, true, false], [false; 4]],
            granules: vec![vec![g0], vec![g1]],
        });
    }
    check(&specs);
}

#[test]
fn window_switching_matches_the_formulas() {
    let mut rng = Rng(99);
    let h = header(Version::Mpeg1, 1, ChannelMode::Mono, 0);
    let bands = tables::bands_for(&h);
    let make = |rng: &mut Rng, block_type: u8, mixed: bool| {
        let mut c = Channel::silent();
        c.block_type = block_type;
        c.mixed = mixed;
        c.global_gain = 175;
        c.scalefac_compress = 11;
        c.subblock_gain = [0, 2, 5];
        c.tables = [15, 24, 0];
        random_long_sf(rng, &mut c, false, false);
        fill(rng, bands, &mut c, 150, 380, 25);
        c
    };
    let mut specs = Vec::new();
    for [a, b] in [
        [(1, false), (2, false)],
        [(2, true), (3, false)],
        [(2, false), (2, true)],
    ] {
        specs.push(Spec {
            header: h,
            scfsi: [[false; 4]; 2],
            granules: vec![
                vec![make(&mut rng, a.0, a.1)],
                vec![make(&mut rng, b.0, b.1)],
            ],
        });
    }
    check(&specs);
}

#[test]
fn joint_stereo_matches_the_formulas() {
    let mut rng = Rng(2024);
    let mut specs = Vec::new();
    for (extension, short) in [(2u8, false), (3, false), (1, true), (3, true)] {
        let h = header(Version::Mpeg1, 2, ChannelMode::JointStereo, extension);
        let bands = tables::bands_for(&h);
        let mut granules = Vec::new();
        for _ in 0..2 {
            let mut left = Channel::silent();
            left.global_gain = 172;
            left.scalefac_compress = 15;
            left.tables = [24, 13, 7];
            left.region0_count = 7;
            left.region1_count = 3;
            if short {
                left.block_type = 2;
            }
            random_long_sf(&mut rng, &mut left, false, false);
            // Up to the top band, which has no scale factor of its own.
            fill(&mut rng, bands, &mut left, 160, 568, 30);
            let mut right = left.clone();
            right.global_gain = 168;
            random_long_sf(&mut rng, &mut right, false, false);
            // The right channel stops early; above, its scale factors are
            // intensity positions (0-6, 7 meaning "not intensity coded").
            let top = if short { 90 } else { 100 };
            fill(&mut rng, bands, &mut right, 60, top, 12);
            for sf in right.long_sf.iter_mut() {
                *sf = (*sf).min(7);
            }
            right.long_sf[15] = 7;
            for band in right.short_sf.iter_mut() {
                for sf in band.iter_mut() {
                    *sf = (*sf).min(7);
                }
            }
            right.short_sf[9][1] = 7;
            granules.push(vec![left, right]);
        }
        specs.push(Spec {
            header: h,
            scfsi: [[false; 4]; 2],
            granules,
        });
    }
    check(&specs);
}

#[test]
fn mpeg2_and_2_5_match_the_formulas() {
    let mut rng = Rng(5);
    // MPEG-2, 22.05 kHz, mid/side and intensity stereo on long blocks.
    let h = header(Version::Mpeg2, 0, ChannelMode::JointStereo, 3);
    let bands = tables::bands_for(&h);
    let mut specs = Vec::new();
    for sfc in [
        (4 * 5 + 3) * 16 + (2 << 2) + 1,
        400 + (3 * 5 + 2) * 4 + 3,
        511,
    ] {
        let mut left = Channel::silent();
        left.global_gain = 165;
        left.scalefac_compress = sfc;
        left.tables = [24, 15, 9];
        left.region0_count = 6;
        left.region1_count = 5;
        random_long_sf(&mut rng, &mut left, true, false);
        fill(&mut rng, bands, &mut left, 140, 568, 22);
        let mut right = left.clone();
        right.scalefac_compress = 2 * 100 + 1; // table 3: slen 2, 4, 4
        random_long_sf(&mut rng, &mut right, true, true);
        fill(&mut rng, bands, &mut right, 30, 70, 9);
        specs.push(Spec {
            header: h,
            scfsi: [[false; 4]; 2],
            granules: vec![vec![left, right]],
        });
    }
    check(&specs);

    // MPEG-2.5, 8 kHz mono: short and mixed blocks (four long subbands).
    let h = header(Version::Mpeg25, 2, ChannelMode::Mono, 0);
    let bands = tables::bands_for(&h);
    let mut specs = Vec::new();
    for (block_type, mixed, sfc) in [
        (2, false, 87),
        (2, true, 300),
        (3, false, 505),
        (0, false, 45),
    ] {
        let mut c = Channel::silent();
        c.block_type = block_type;
        c.mixed = mixed;
        c.global_gain = 180;
        c.scalefac_compress = sfc;
        c.subblock_gain = [3, 0, 1];
        c.tables = [13, 16, 2];
        c.region0_count = 4;
        c.region1_count = 4;
        random_long_sf(&mut rng, &mut c, true, false);
        fill(&mut rng, bands, &mut c, 100, 300, 15);
        specs.push(Spec {
            header: h,
            scfsi: [[false; 4]; 2],
            granules: vec![vec![c]],
        });
    }
    check(&specs);
}

/// A random mono MPEG-1 frame (for tests about the stream around frames).
fn random_frame_spec(rng: &mut Rng) -> Spec {
    let h = header(Version::Mpeg1, 0, ChannelMode::Mono, 0);
    let bands = tables::bands_for(&h);
    let mut granules = Vec::new();
    for _ in 0..2 {
        let mut c = Channel::silent();
        c.global_gain = 175;
        c.tables = [15, 15, 15];
        c.region0_count = 8;
        c.region1_count = 4;
        fill(rng, bands, &mut c, 80, 200, 12);
        granules.push(vec![c]);
    }
    Spec {
        header: h,
        scfsi: [[false; 4]; 2],
        granules,
    }
}

#[test]
fn the_bit_reservoir_reaches_into_earlier_frames() {
    let mut rng = Rng(31);
    let specs: Vec<Spec> = (0..3).map(|_| random_frame_spec(&mut rng)).collect();
    // Frame 1's data starts 100 bytes back, in the last 100 bytes of frame
    // 0 (after frame 0's own data and its unused space).
    let (main0, fields0) = encode_main(&specs[0]);
    let (main1, fields1) = encode_main(&specs[1]);
    let side0 = side_info(&specs[0], &fields0, 0);
    let borrowed = &main1.bytes[..100];
    let size = assemble(
        specs[0].header,
        &side0,
        &[&main0.bytes[..], borrowed].concat(),
        None,
    )
    .len();
    let mut own0 = main0.bytes.clone();
    own0.resize(size - 4 - side0.len() - 100, 0);
    own0.extend(borrowed);
    let f0 = assemble(specs[0].header, &side0, &own0, None);
    assert_eq!(f0.len(), size);
    let side1 = side_info(&specs[1], &fields1, 100);
    let f1 = assemble(specs[1].header, &side1, &main1.bytes[100..], None);
    let f2 = frame(&specs[2]);
    let bytes: Vec<u8> = [f0, f1, f2].concat();
    let (got, problems) = decoded(&bytes);
    assert_eq!(problems, vec![None; 3]);
    let mut reference = Reference::new();
    let expected: Vec<f64> = specs.iter().flat_map(|s| reference.frame(s)).collect();
    assert_eq!(got.len(), expected.len());
    for (g, e) in got.iter().zip(&expected) {
        assert!((f64::from(*g) - e).abs() < 1e-5);
    }
}

#[test]
fn checksums_are_checked() {
    let mut rng = Rng(8);
    let mut spec = random_frame_spec(&mut rng);
    spec.header.protected = true;
    let good = frame(&spec);
    let mut bad = good.clone();
    bad[5] ^= 0x40;
    let bytes: Vec<u8> = [good.clone(), good.clone(), bad, good].concat();
    let (_, problems) = decoded(&bytes);
    assert_eq!(problems, vec![None, None, Some(Problem::Checksum), None]);
}

/// An Info frame with a LAME tag, as LAME writes before the audio.
fn info_frame(frames: u32, delay: u32, padding: u32) -> Vec<u8> {
    let h = FrameHeader {
        bitrate_index: 9,
        ..header(Version::Mpeg1, 0, ChannelMode::Mono, 0)
    };
    info_frame_for(h, frames, delay, padding)
}

/// [`info_frame`] for a stream of frames like `h`.
fn info_frame_for(h: FrameHeader, frames: u32, delay: u32, padding: u32) -> Vec<u8> {
    let mut f = h.to_bytes().to_vec();
    f.resize(4 + h.side_info_length(), 0);
    f.extend(b"Info");
    f.extend(0x0Fu32.to_be_bytes());
    f.extend(frames.to_be_bytes());
    f.extend(12345u32.to_be_bytes());
    f.extend((0..100).map(|i| i as u8));
    f.extend(57u32.to_be_bytes());
    let tag_start = f.len();
    f.extend(b"LAME3.99r");
    f.extend([0x01, 0xA0, 0, 0, 0, 0, 0, 0, 0, 0, 0x02, 0x80]);
    f.push((delay >> 4) as u8);
    f.push((((delay & 15) << 4) | (padding >> 8)) as u8);
    f.push(padding as u8);
    f.extend([0x45, 0, 0, 0, 0, 0, 0x12, 0x34, 0, 0]);
    assert_eq!(f.len() - tag_start, 34);
    let crc = {
        let mut crc = 0u16;
        for &b in &f {
            crc ^= u16::from(b);
            for _ in 0..8 {
                crc = if crc & 1 != 0 {
                    (crc >> 1) ^ 0xA001
                } else {
                    crc >> 1
                };
            }
        }
        crc
    };
    f.extend(crc.to_be_bytes());
    f.resize(h.frame_length().unwrap(), 0);
    f
}

#[test]
fn gapless_playback_trims_the_lame_delay_and_padding() {
    let mut rng = Rng(77);
    let specs: Vec<Spec> = (0..5).map(|_| random_frame_spec(&mut rng)).collect();
    let (delay, padding) = (576, 1000);
    let mut bytes = info_frame(5, delay, padding);
    for s in &specs {
        bytes.extend(frame(s));
    }
    let mut reference = Reference::new();
    let all: Vec<f64> = specs.iter().flat_map(|s| reference.frame(s)).collect();
    let start = (delay + DECODER_DELAY) as usize;
    let total = 5 * 1152 - (delay + padding) as usize;

    let decoder = Decoder::new(&bytes[..]).unwrap();
    let info = decoder.info().clone();
    assert_eq!(info.xing.as_ref().unwrap().frames, Some(5));
    let lame = info.lame.clone().unwrap();
    assert_eq!(
        (lame.delay, lame.padding, lame.crc_ok),
        (delay, padding, true)
    );
    assert_eq!(lame.encoder, "LAME3.99r");
    assert_eq!(info.gapless_samples, Some(total as u64));
    let got: Vec<f32> = decoder.flat_map(|f| f.samples).collect();
    assert_eq!(got.len(), total);
    for (g, e) in got.iter().zip(&all[start..start + total]) {
        assert!((f64::from(*g) - e).abs() < 1e-5);
    }

    // Without trimming: every frame whole (the Info frame isn't audio).
    let mut decoder = Decoder::new(&bytes[..]).unwrap();
    decoder.set_gapless(false);
    let got: Vec<f32> = decoder.flat_map(|f| f.samples).collect();
    assert_eq!(got.len(), 5 * 1152);
}

#[test]
fn finds_frames_between_tags_and_junk() {
    let mut rng = Rng(3);
    let specs: Vec<Spec> = (0..5).map(|_| random_frame_spec(&mut rng)).collect();
    let frames: Vec<Vec<u8>> = specs.iter().map(frame).collect();
    // An ID3v2 tag whose contents look like a frame header.
    let mut bytes = b"ID3\x04\x00\x00\x00\x00\x00\x08\xFF\xFB\x90\x00\xFF\xFB\x90\x00".to_vec();
    bytes.extend(frames[..3].concat());
    bytes.extend([0x55u8; 50]);
    bytes.extend(frames[3..].concat());
    let mut v1 = b"TAG".to_vec();
    v1.resize(128, 0x20);
    bytes.extend(v1);

    let mut decoder = Decoder::new(&bytes[..]).unwrap();
    assert_eq!(decoder.info().tags.id3v2, 18);
    assert!(decoder.info().tags.id3v1);
    let got: Vec<f32> = decoder.by_ref().flat_map(|f| f.samples).collect();
    assert_eq!(decoder.skipped_bytes(), 50);
    let mut reference = Reference::new();
    let expected: Vec<f64> = specs.iter().flat_map(|s| reference.frame(s)).collect();
    assert_eq!(got.len(), expected.len());
    for (g, e) in got.iter().zip(&expected) {
        assert!((f64::from(*g) - e).abs() < 1e-5);
    }

    let scan = crate::scan(&bytes).unwrap();
    assert_eq!((scan.frames, scan.skipped), (5, 50));
}

#[test]
fn free_format_frames() {
    let mut rng = Rng(12);
    let specs: Vec<Spec> = (0..4).map(|_| random_frame_spec(&mut rng)).collect();
    let mut bytes = Vec::new();
    for s in &specs {
        let (main, fields) = encode_main(s);
        bytes.extend(assemble(
            s.header,
            &side_info(s, &fields, 0),
            &main.bytes,
            Some(700),
        ));
    }
    let (got, problems) = decoded(&bytes);
    assert_eq!(problems, vec![None; 4]);
    let mut reference = Reference::new();
    let expected: Vec<f64> = specs.iter().flat_map(|s| reference.frame(s)).collect();
    assert_eq!(got.len(), expected.len());
    for (g, e) in got.iter().zip(&expected) {
        assert!((f64::from(*g) - e).abs() < 1e-5);
    }
}

#[test]
fn damaged_frames_are_reported_and_silent() {
    let mut rng = Rng(4);
    let spec = random_frame_spec(&mut rng);
    let (main, mut fields) = encode_main(&spec);
    let good = frame(&spec);
    // Claims far more data than there is.
    fields[1][0].0 = 4000;
    let too_long = assemble(
        spec.header,
        &side_info(&spec, &fields, 0),
        &main.bytes,
        None,
    );
    // Needs 300 bytes of reservoir at the very start.
    let (_, fields) = encode_main(&spec);
    let missing = assemble(
        spec.header,
        &side_info(&spec, &fields, 300),
        &main.bytes,
        None,
    );
    // A table that doesn't exist.
    let mut bad_table = spec.granules.clone();
    bad_table[0][0].tables = [4, 4, 4];
    let mut w = BitWriter::default();
    w.put(0, 8);
    let fields = vec![vec![(8, 10)], vec![(0, 0)]];
    let table4 = assemble(
        spec.header,
        &side_info(
            &Spec {
                header: spec.header,
                scfsi: spec.scfsi,
                granules: bad_table,
            },
            &fields,
            0,
        ),
        &w.bytes,
        None,
    );
    let bytes: Vec<u8> = [missing, good.clone(), too_long, table4, good].concat();
    let (samples, problems) = decoded(&bytes);
    assert_eq!(problems[0], Some(Problem::MissingReservoir));
    assert_eq!(problems[1], None);
    assert!(matches!(problems[2], Some(Problem::Damaged(_))));
    assert!(matches!(problems[3], Some(Problem::Damaged(_))));
    assert_eq!(problems[4], None);
    assert_eq!(samples.len(), 5 * 1152);
    assert!(samples[..1152].iter().all(|&s| s == 0.0));
    assert!(samples.iter().all(|s| s.is_finite()));
}

#[test]
fn reading_samples_and_rewinding() {
    let mut rng = Rng(21);
    let bytes: Vec<u8> = (0..4)
        .flat_map(|_| frame(&random_frame_spec(&mut rng)))
        .collect();
    let all = decode_all(&bytes).unwrap();
    assert_eq!((all.sample_rate, all.channels, all.frames), (44_100, 1, 4));
    assert_eq!(all.samples.len(), 4 * 1152);
    assert_eq!(all.problem_frames, 0);
    let mut decoder = Decoder::new(bytes.clone()).unwrap();
    for _ in 0..2 {
        let mut read = Vec::new();
        let mut buffer = [0i16; 333];
        loop {
            let n = decoder.read_i16(&mut buffer);
            read.extend_from_slice(&buffer[..n]);
            if n < buffer.len() {
                break;
            }
        }
        assert_eq!(read, all.samples);
        decoder.rewind();
    }
}

#[test]
fn a_decoder_can_move_to_an_audio_thread() {
    fn movable<T: Send + Sync + 'static>() {}
    movable::<Decoder<Vec<u8>>>();
    movable::<Decoder<std::sync::Arc<[u8]>>>();
}

#[test]
fn refuses_what_it_cannot_decode() {
    // Layer II frames: 192 kbit/s, 48 kHz.
    let h = FrameHeader {
        layer: 2,
        bitrate_index: 10,
        ..header(Version::Mpeg1, 1, ChannelMode::Stereo, 0)
    };
    let mut f = h.to_bytes().to_vec();
    f.resize(h.frame_length().unwrap(), 0);
    let bytes = [f.clone(), f].concat();
    assert!(matches!(
        Decoder::new(&bytes[..]),
        Err(Error::Unsupported(_))
    ));
    assert!(matches!(
        Decoder::new(&b"not an mp3 at all"[..]),
        Err(Error::NoFrames)
    ));
}

// ---- Seeking ----

/// The block type after `last` as an encoder switches them: long, start, short, stop, long.
fn next_block(rng: &mut Rng, last: u8) -> u8 {
    match last {
        0 | 3 => {
            if rng.below(3) == 0 {
                1
            } else {
                0
            }
        }
        1 => 2,
        _ => {
            if rng.below(2) == 0 {
                2
            } else {
                3
            }
        }
    }
}

/// One granule's channel of block type `block_type` (`mixed`: a short block whose lowest
/// subbands are long), with random scale factors and values, more or fewer of them.
fn switching_channel(rng: &mut Rng, h: &FrameHeader, block_type: u8, mixed: bool) -> Channel {
    let lsf = h.is_lsf();
    let bands = tables::bands_for(h);
    let mut c = Channel::silent();
    c.block_type = block_type;
    c.mixed = mixed;
    c.global_gain = 160 + rng.below(20) as i32;
    c.scalefac_compress = rng.below(if lsf { 512 } else { 16 }) as usize;
    c.subblock_gain = [0, 1, 2].map(|_| rng.below(4) as i32);
    c.tables = if block_type == 0 {
        [15, 24, 13]
    } else {
        [15, 24, 0]
    };
    c.region0_count = 7;
    c.region1_count = 3;
    c.count1_b = rng.below(2) == 1;
    random_long_sf(rng, &mut c, lsf, false);
    let big = 2 * rng.below(60) as usize;
    let top = big + 4 * rng.below(60) as usize;
    fill(rng, bands, &mut c, big, top, 15);
    c
}

/// `frames` frames like `h` whose granules switch blocks, each channel on its own (both the
/// same in joint stereo, as mid/side wants), the frames `mono` mono.
fn switching_specs(
    rng: &mut Rng,
    h: FrameHeader,
    frames: usize,
    mono: std::ops::Range<usize>,
) -> Vec<Spec> {
    let mut last = [(0u8, false); 2];
    let mut specs = Vec::new();
    for i in 0..frames {
        let h = if mono.contains(&i) {
            FrameHeader {
                mode: ChannelMode::Mono,
                mode_extension: 0,
                ..h
            }
        } else {
            h
        };
        let mut granules = Vec::new();
        for _ in 0..if h.is_lsf() { 1 } else { 2 } {
            let mut channels = Vec::new();
            for ch in 0..h.channels() {
                if ch == 0 || h.mode != ChannelMode::JointStereo {
                    let block_type = next_block(rng, last[ch].0);
                    last[ch] = (block_type, block_type == 2 && rng.below(3) == 0);
                } else {
                    last[1] = last[0];
                }
                let (block_type, mixed) = last[ch];
                channels.push(switching_channel(rng, &h, block_type, mixed));
            }
            granules.push(channels);
        }
        specs.push(Spec {
            header: h,
            scfsi: [[false; 4]; 2],
            granules,
        });
    }
    specs
}

/// Frames whose data run on from one to the next (the bit reservoir): each frame's data starts
/// as far back in the room earlier frames left as `main_data_begin` reaches (511 bytes, 255 in
/// MPEG-2), in a frame of the smallest bitrate that holds the rest; every fifth frame takes the
/// largest, which leaves room for those after it. Answers the stream and each frame's
/// `main_data_begin`.
fn reservoir_stream(specs: &[Spec]) -> (Vec<u8>, Vec<usize>) {
    // The frames' own data areas one after another, and where the data so far ends in them.
    let mut run: Vec<u8> = Vec::new();
    let mut end = 0;
    let mut frames = Vec::new();
    for (i, s) in specs.iter().enumerate() {
        let (main, fields) = encode_main(s);
        let area = run.len();
        let most = if s.header.is_lsf() { 255 } else { 511 };
        let start = end.max(area.saturating_sub(most));
        let with = |index: u8| FrameHeader {
            bitrate_index: index,
            ..s.header
        };
        let room = |index: u8| {
            let h = with(index);
            h.frame_length().unwrap() - h.side_info_start() - h.side_info_length()
        };
        let fits = |index: u8| start + main.bytes.len() <= area + room(index);
        let index = if i % 5 == 2 && fits(14) {
            14
        } else {
            (1..15)
                .find(|&x| fits(x))
                .expect("a bitrate that holds the frame's data")
        };
        run.resize(area + room(index), 0);
        run[start..start + main.bytes.len()].copy_from_slice(&main.bytes);
        end = start + main.bytes.len();
        let begin = area - start;
        frames.push((with(index), side_info(s, &fields, begin), area, begin));
    }
    let mut bytes = Vec::new();
    for (h, side, area, _) in &frames {
        let room = h.frame_length().unwrap() - h.side_info_start() - h.side_info_length();
        let f = assemble(*h, side, &run[*area..*area + room], None);
        assert_eq!(f.len(), h.frame_length().unwrap());
        bytes.extend(f);
    }
    (bytes, frames.iter().map(|f| f.3).collect())
}

/// Every frame decodes the same after [`Decoder::seek`] to it as from the start, bit for bit,
/// gapless trimming and all: through block switching (long, start, short, mixed and stop
/// blocks), data reaching back into earlier frames (511 bytes; 255 in MPEG-2, one granule a
/// frame), mono frames among stereo ones, and an Info frame with LAME's delay and padding. A
/// decoder started at the frame itself, with nothing before it, gives other samples.
#[test]
fn seeking_decodes_every_frame_as_from_the_start() {
    let mut rng = Rng(1031);
    let stereo = header(Version::Mpeg1, 1, ChannelMode::Stereo, 0);
    let specs = switching_specs(&mut rng, stereo, 30, 13..16);
    let (audio, begins) = reservoir_stream(&specs);
    let info = info_frame_for(
        FrameHeader {
            bitrate_index: 9,
            ..stereo
        },
        30,
        576,
        1000,
    );
    let joint = header(Version::Mpeg1, 0, ChannelMode::JointStereo, 2);
    let joint_specs = switching_specs(&mut rng, joint, 24, 0..0);
    let (joint_audio, joint_begins) = reservoir_stream(&joint_specs);
    let lsf = header(Version::Mpeg2, 0, ChannelMode::Stereo, 0);
    let lsf_specs = switching_specs(&mut rng, lsf, 30, 0..0);
    let (lsf_audio, lsf_begins) = reservoir_stream(&lsf_specs);
    // What the streams hold: every block type and mixed blocks, and data far back.
    let kinds: std::collections::BTreeSet<(u8, bool)> = [&specs, &joint_specs, &lsf_specs]
        .into_iter()
        .flat_map(|s| s.iter())
        .flat_map(|s| s.granules.iter().flatten())
        .map(|c| (c.block_type, c.mixed))
        .collect();
    assert_eq!(
        kinds.into_iter().collect::<Vec<_>>(),
        [(0, false), (1, false), (2, false), (2, true), (3, false)]
    );
    assert!(begins.iter().chain(&joint_begins).any(|&b| b > 400));
    assert!(lsf_begins.iter().any(|&b| b > 200));
    let bits = |f: &crate::Frame| f.samples.iter().map(|s| s.to_bits()).collect::<Vec<_>>();
    let mut fresh_differs = 0;
    for bytes in [[info, audio].concat(), joint_audio, lsf_audio] {
        for gapless in [true, false] {
            let mut decoder = Decoder::new(&bytes[..]).unwrap();
            decoder.set_gapless(gapless);
            let all: Vec<_> = decoder.by_ref().collect();
            assert!(all.iter().all(|f| f.problem.is_none()));
            assert_eq!(decoder.frame_count(), all.len());
            for k in 0..=all.len() + 1 {
                decoder.seek(k);
                let rest: Vec<_> = decoder.by_ref().collect();
                assert_eq!(rest.len(), all.len().saturating_sub(k), "from frame {k}");
                for (i, (got, want)) in rest.iter().zip(&all[k.min(all.len())..]).enumerate() {
                    let at = k + i;
                    assert_eq!((got.offset, got.problem), (want.offset, want.problem));
                    assert_eq!(bits(got), bits(want), "frame {at} after seeking to {k}");
                }
                if !gapless && k > 0 && k < all.len() {
                    let mut fresh = Decoder::new(&bytes[all[k].offset..]).unwrap();
                    let first = fresh.next_frame().unwrap();
                    if bits(&first) != bits(&all[k]) {
                        fresh_differs += 1;
                    }
                }
            }
        }
    }
    // Nearly every frame of the 84 depends on the frames before it.
    assert!(fresh_differs > 70, "{fresh_differs}");
}

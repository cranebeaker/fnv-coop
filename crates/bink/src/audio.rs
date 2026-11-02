//! Bink audio, the DCT variant (the one the game's intro uses).
//!
//! Each frame's audio packet starts with the number of bytes of 16-bit PCM
//! it decodes to, then holds blocks. A block codes each channel's
//! frequency coefficients: two leading values as small floats, a level per
//! frequency band, then the rest as runs of fixed-width integers scaled by
//! their band's level. An inverse DCT turns each channel into samples, which
//! are scaled, rounded and saturated to 16 bits and interleaved. Each block
//! overlaps the next by a sixteenth of its length, crossfaded in 16-bit
//! integers.
//!
//! Everything except the transform follows the game's `binkw32.dll`
//! (image base 0x18000000): the band and level tables and the bit layout
//! (block decoder 0x18018210, coefficient reader 0x18017f20, set-up
//! 0x180185d0), the scale `2 / sqrt(N)` applied only when converting to
//! integers (0x18017e40, round to nearest even and saturate), and the
//! integer crossfade (0x18018800). The DLL's inverse DCT is single-precision
//! x87 code (Takuya Ooura's split-radix FFT package), whose last bit
//! depends on the thread's x87 precision setting; this one is computed in
//! double precision. The crate documentation gives the measured agreement.

use std::f64::consts::PI;
use std::fmt;

/// Why an audio packet could not be decoded.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AudioError {
    /// Only the DCT variant is implemented.
    Rdft,
    /// More than two channels.
    Channels(u16),
    /// The packet ended inside a block.
    Truncated,
}

impl fmt::Display for AudioError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AudioError::Rdft => write!(f, "RDFT Bink audio is not supported"),
            AudioError::Channels(c) => write!(f, "{c} audio channels"),
            AudioError::Truncated => write!(f, "the audio packet ended inside a block"),
        }
    }
}

impl std::error::Error for AudioError {}

/// Band edge frequencies in Hz (binkw32.dll 0x1805e710; the list starts at
/// 0 and its 25 entries end at 15500).
const CRITICAL_FREQS: [u32; 25] = [
    0, 100, 200, 300, 400, 510, 630, 770, 920, 1080, 1270, 1480, 1720, 2000, 2320, 2700, 3150,
    3700, 4400, 5300, 6400, 7700, 9500, 12000, 15500,
];

/// Run lengths in units of 8 coefficients (binkw32.dll 0x1805e700).
const RUN_LENGTHS: [u32; 16] = [2, 3, 4, 5, 6, 8, 9, 10, 11, 12, 13, 14, 15, 16, 32, 64];

/// Scale for a leading coefficient's 5-bit exponent, as the 32 floats at
/// binkw32.dll 0x1805eb78: 2^(e - 23) for e below 24, then what the DLL
/// happens to hold after them: two zeros, then six pointers into the DLL's
/// read-only data (0x180483e0 and so on), which as floats are about 1e-24.
/// Valid streams do not use exponents above 23.
const EXPONENT_SCALE: [u32; 32] = [
    0x3400_0000,
    0x3480_0000,
    0x3500_0000,
    0x3580_0000,
    0x3600_0000,
    0x3680_0000,
    0x3700_0000,
    0x3780_0000,
    0x3800_0000,
    0x3880_0000,
    0x3900_0000,
    0x3980_0000,
    0x3a00_0000,
    0x3a80_0000,
    0x3b00_0000,
    0x3b80_0000,
    0x3c00_0000,
    0x3c80_0000,
    0x3d00_0000,
    0x3d80_0000,
    0x3e00_0000,
    0x3e80_0000,
    0x3f00_0000,
    0x3f80_0000,
    0x0000_0000,
    0x0000_0000,
    0x1804_83e0,
    0x1804_83f0,
    0x1804_8410,
    0x1804_8430,
    0x1804_8450,
    0x1804_8470,
];

/// A band's level for each 8-bit code: binkw32.dll 0x1805e778, 256 floats
/// that grow by a factor of about 1.1652 per step from 1.0. Generated from
/// the DLL and checked bit for bit (see `level`).
const LEVEL_BITS: [u32; 256] = include!("audio_levels.in");

fn level(code: u32) -> f32 {
    f32::from_bits(LEVEL_BITS[code as usize])
}

/// The bit reader for audio: 32-bit little-endian words, low bits first,
/// as the block decoder reads them.
struct Bits<'a> {
    data: &'a [u8],
    pos: usize,
}

impl Bits<'_> {
    fn bits(&mut self, n: u32) -> Result<u32, AudioError> {
        if n == 0 {
            return Ok(0);
        }
        if self.pos + n as usize > self.data.len() * 8 {
            return Err(AudioError::Truncated);
        }
        let mut v = 0u32;
        for i in 0..n as usize {
            let p = self.pos + i;
            v |= (((self.data[p / 8] >> (p % 8)) & 1) as u32) << i;
        }
        self.pos += n as usize;
        Ok(v)
    }

    /// Bytes of whole 32-bit words touched so far.
    fn words_used(&self) -> usize {
        self.pos.div_ceil(32) * 4
    }
}

/// An inverse DCT, `y[k] = sum_n x[n] cos(pi n (k + 1/2) / N)`, through a
/// complex FFT of twice the length.
struct Dct {
    n: usize,
    /// e^(-i pi n / 2N) for the pre-twiddle (conjugated input).
    pre: Vec<(f64, f64)>,
    /// FFT twiddles e^(-2 pi i k / 2N).
    roots: Vec<(f64, f64)>,
    buf: Vec<(f64, f64)>,
}

impl Dct {
    fn new(n: usize) -> Self {
        let m = 2 * n;
        let pre = (0..n)
            .map(|j| {
                let a = -PI * j as f64 / m as f64;
                (a.cos(), a.sin())
            })
            .collect();
        let roots = (0..m / 2)
            .map(|k| {
                let a = -2.0 * PI * k as f64 / m as f64;
                (a.cos(), a.sin())
            })
            .collect();
        Dct {
            n,
            pre,
            roots,
            buf: vec![(0.0, 0.0); m],
        }
    }

    fn run(&mut self, data: &mut [f32]) {
        let (n, m) = (self.n, 2 * self.n);
        for (j, b) in self.buf.iter_mut().enumerate() {
            *b = if j < n {
                let x = data[j] as f64;
                (x * self.pre[j].0, x * self.pre[j].1)
            } else {
                (0.0, 0.0)
            };
        }
        // Iterative radix-2 FFT.
        let bits = m.trailing_zeros();
        for i in 0..m {
            let r = i.reverse_bits() >> (usize::BITS - bits);
            if r > i {
                self.buf.swap(i, r);
            }
        }
        let mut len = 2;
        while len <= m {
            let step = m / len;
            for start in (0..m).step_by(len) {
                for k in 0..len / 2 {
                    let w = self.roots[k * step];
                    let a = self.buf[start + k];
                    let b = self.buf[start + k + len / 2];
                    let t = (b.0 * w.0 - b.1 * w.1, b.0 * w.1 + b.1 * w.0);
                    self.buf[start + k] = (a.0 + t.0, a.1 + t.1);
                    self.buf[start + k + len / 2] = (a.0 - t.0, a.1 - t.1);
                }
            }
            len <<= 1;
        }
        for (k, d) in data.iter_mut().enumerate().take(n) {
            *d = self.buf[k].0 as f32;
        }
    }
}

/// Rounds to the nearest integer, halves to even (the x87 default).
fn round_ties_even(x: f32) -> f32 {
    let r = x.round();
    if (r - x).abs() == 0.5 {
        2.0 * (x / 2.0).round()
    } else {
        r
    }
}

/// Decodes one audio track's packets in order.
pub struct AudioDecoder {
    channels: usize,
    frame_len: usize,
    /// Band edges in units of two coefficients, `bands + 1` of them.
    bands: Vec<u32>,
    scale: f32,
    /// Interleaved samples of overlap at the end of each block.
    overlap: usize,
    prev: Vec<i16>,
    first: bool,
    dct: Dct,
    coeffs: Vec<f32>,
    block: Vec<i16>,
}

impl AudioDecoder {
    /// A decoder for a track with this rate, channel count and flags (from
    /// the movie header).
    pub fn new(sample_rate: u32, channels: u16, flags: u16) -> Result<Self, AudioError> {
        if flags & crate::AUDIO_DCT == 0 {
            return Err(AudioError::Rdft);
        }
        if !(1..=2).contains(&channels) {
            return Err(AudioError::Channels(channels));
        }
        let frame_len: usize = if sample_rate < 22050 {
            512
        } else if sample_rate < 44100 {
            1024
        } else {
            2048
        };
        let half_len = (frame_len / 2) as u32;
        let half_rate = (sample_rate + 1) >> 1;
        let count = CRITICAL_FREQS
            .iter()
            .position(|&f| half_rate <= f)
            .unwrap_or(CRITICAL_FREQS.len());
        let mut bands: Vec<u32> = CRITICAL_FREQS[..count]
            .iter()
            .map(|&f| ((f * half_len) / half_rate).max(1))
            .collect();
        bands.push(half_len);
        let channels = channels as usize;
        let scale = (2.0f64 / (frame_len as f32 as f64).sqrt()) as f32;
        let overlap = frame_len * channels / 16;
        Ok(AudioDecoder {
            channels,
            frame_len,
            bands,
            scale,
            overlap,
            prev: vec![0; overlap],
            first: true,
            dct: Dct::new(frame_len),
            coeffs: vec![0.0; frame_len * channels],
            block: vec![0; frame_len * channels],
        })
    }

    /// Interleaved samples each block adds.
    pub fn block_samples(&self) -> usize {
        self.frame_len * self.channels - self.overlap
    }

    /// Decodes one frame's packet for this track, appending interleaved
    /// 16-bit samples to `out`.
    pub fn decode_packet(&mut self, packet: &[u8], out: &mut Vec<i16>) -> Result<(), AudioError> {
        if packet.len() < 4 {
            return Ok(());
        }
        let mut wanted =
            u32::from_le_bytes([packet[0], packet[1], packet[2], packet[3]]) as usize / 2;
        let mut pos = 4;
        while wanted > 0 {
            let used = self.decode_block(&packet[pos.min(packet.len())..])?;
            pos += used;
            let n = self.block_samples().min(wanted);
            out.extend_from_slice(&self.block[..n]);
            wanted -= n;
        }
        Ok(())
    }

    /// Decodes one block into `self.block` (crossfaded, ready to output) and
    /// returns how many bytes of the packet it used.
    fn decode_block(&mut self, data: &[u8]) -> Result<usize, AudioError> {
        let mut br = Bits { data, pos: 0 };
        // The DCT variant starts with two unused bits.
        br.bits(2)?;
        let n = self.frame_len;
        for ch in 0..self.channels {
            let c = &mut self.coeffs[ch * n..(ch + 1) * n];
            for v in c.iter_mut().take(2) {
                let raw = br.bits(29)?;
                let mantissa = (raw >> 5) & 0x7f_ffff;
                let mut f = mantissa as f32 * f32::from_bits(EXPONENT_SCALE[(raw & 31) as usize]);
                if raw & 0x1000_0000 != 0 {
                    f = -f;
                }
                *v = f;
            }
            let mut levels = [0f32; 25];
            let band_count = self.bands.len() - 1;
            for l in levels.iter_mut().take(band_count) {
                *l = level(br.bits(8)?);
            }
            // Coefficients 2 onwards.
            let edge = |k: usize, bands: &[u32]| bands[k] * 2;
            let mut k = 0usize;
            let mut q = 0f32;
            while (edge(k, &self.bands) as usize) < 2 {
                q = levels[k];
                k += 1;
            }
            let mut i = 2usize;
            while i < n {
                let mut j = if br.bits(1)? != 0 {
                    i + RUN_LENGTHS[br.bits(4)? as usize] as usize * 8
                } else {
                    i + 8
                };
                if j > n {
                    j = n;
                }
                let width = br.bits(4)?;
                if width == 0 {
                    c[i..j].fill(0.0);
                    i = j;
                    while (edge(k, &self.bands) as usize) < i {
                        q = levels[k];
                        k += 1;
                    }
                } else {
                    while i < j {
                        if i == edge(k, &self.bands) as usize {
                            q = levels[k];
                            k += 1;
                        }
                        let v = br.bits(width)?;
                        c[i] = if v == 0 {
                            0.0
                        } else if br.bits(1)? != 0 {
                            v as i32 as f32 * -q
                        } else {
                            v as i32 as f32 * q
                        };
                        i += 1;
                    }
                }
            }
            self.dct.run(c);
        }

        // Scale, round to nearest even, saturate and interleave.
        for s in 0..n {
            for ch in 0..self.channels {
                let x = self.coeffs[ch * n + s] * self.scale;
                // FISTP: out of the 32-bit range (or NaN) it stores the
                // "integer indefinite" 0x80000000, which then saturates low.
                let r = round_ties_even(x);
                let i = if r.is_nan() || !(-2147483648.0..2147483648.0).contains(&r) {
                    i32::MIN
                } else {
                    r as i32
                };
                self.block[s * self.channels + ch] = i.clamp(-32768, 32767) as i16;
            }
        }

        // Crossfade the start with the previous block's end, in integers:
        // the DLL divides the weighted sum as an unsigned number.
        if !self.first {
            let o = self.overlap as u32;
            for i in 0..self.overlap {
                let a = self.prev[i] as i32 * (o - i as u32) as i32;
                let b = self.block[i] as i32 * i as i32;
                self.block[i] = ((a.wrapping_add(b) as u32) / o) as u16 as i16;
            }
        }
        self.first = false;
        let total = n * self.channels;
        self.prev
            .copy_from_slice(&self.block[total - self.overlap..total]);
        Ok(br.words_used())
    }
}

//! DirectSound's two sample-rate converters, as the player's 32-bit `dsound.dll` uses them for a
//! secondary buffer (all of the game's buffers have `DSBCAPS_CTRLFREQUENCY`). Translated from the
//! instructions of the DLLs the game loads (`SysWOW64`, Windows 11 10.0.26200: `dsound.dll` and
//! `RESAMPLEDMO.DLL`; read in a private analysis with the maintainer's leave, 2026-10-28;
//! addresses below are those DLLs', image base `0x10000000`), and checked sample for sample
//! against what `dsound.dll` hands the Windows audio engine (`scripts/dsmix-measure`, docs/SOUND.md).
//!
//! Which converter a buffer gets is the device's state, not the buffer's:
//! - At first every buffer goes through the Windows resampler, `RESAMPLEDMO.DLL` (ordinal 1,
//!   created in the source-rate APO's `LockForProcess`, `dsound` `10064090`, with a half filter
//!   length of 30 and its default cut-off): [`Dmo`]. At the device's rate it passes the samples
//!   through unchanged.
//! - A frequency change of more than 1/500 of the buffer's rate (`SetFrequency`, or Doppler; the
//!   check is in `dsound` `100581e0`, `rate / 500 < |rate - new|` in whole numbers) turns on the
//!   device's "APL high-performance SRC" (`100527c0`) for good: every buffer of the device,
//!   playing ones included (`1005713d`, a new converter from where the buffer is), and every
//!   buffer played later, goes through a 4-point Lagrange interpolator in `dsound.dll` itself
//!   ([`Apl`]), the device's own rate included.
//!
//! Both run in 10 ms periods (480 frames at 48 kHz) from the buffer's start. A non-looping buffer
//! ends after the period in which the frames handed to its converter reach its end (measured:
//! [`Converter::fed`]). The converter reads the buffer's own samples (16-bit `x / 32768`, 8-bit
//! `(b - 128) / 128`, exact in `f32`); the volume and pan multiply what it gives.
//!
//! `RESAMPLEDMO` is driven as `dsound.dll` drives it ([`Dmo`]): each period it is handed what
//! `GetRequiredInputFrames` asks for and makes the period's frames; a change of 1/500 of the rate
//! or less while it plays is its `SetRate` at the next period, which keeps the output and input
//! positions in time and re-designs the filter. Its filter is either a polyphase table (whole
//! rates, at most 1280 phases, made once per converter) or, otherwise, a curve of the filter
//! sampled 16384 times and interpolated for each output frame, built once from `sin` (the CRT's
//! and Rust's agree on all of its 16385 points: the curve matches the DLL's in memory) and the
//! window table. Both read two piecewise-linear tables (a windowed sinc and a Kaiser window)
//! that are constant data of the DLL: [`Tables`] reads them from the player's own
//! `RESAMPLEDMO.DLL` at run time, so no byte of them is in the repository.

use std::sync::{Arc, OnceLock};

/// The half filter length `dsound.dll` asks for (`10064090`: `0x1e`).
const HALF_LENGTH: i32 = 30;

/// `RESAMPLEDMO.DLL`'s two piecewise-linear tables: value `offset[i] + slope[i] * f` for the
/// fixed-point argument's index `i` and fraction `f` (`1002c5f0`).
pub struct Tables {
    sinc_offset: Vec<f32>,
    sinc_slope: Vec<f32>,
    window_offset: Vec<f32>,
    window_slope: Vec<f32>,
}

/// The player's `RESAMPLEDMO.DLL` (the 32-bit one the game's `dsound.dll` loads), read once.
pub fn tables() -> Option<&'static Tables> {
    static TABLES: OnceLock<Option<Tables>> = OnceLock::new();
    TABLES
        .get_or_init(|| {
            let root = std::env::var_os("SystemRoot").unwrap_or_else(|| "C:\\Windows".into());
            let root = std::path::PathBuf::from(root);
            ["SysWOW64", "System32"].iter().find_map(|d| {
                Tables::read(&std::fs::read(root.join(d).join("RESAMPLEDMO.DLL")).ok()?)
            })
        })
        .as_ref()
}

impl Tables {
    /// The tables of a `RESAMPLEDMO.DLL` file: the four pointers at `1009760c`, `10097610`,
    /// `10097628` and `1009762c` (32-bit build of Windows 11 10.0.26200), checked by the masks
    /// and shifts beside them; `None` for another build.
    pub fn read(file: &[u8]) -> Option<Tables> {
        let pe = Pe::new(file)?;
        let word = |va: u32| pe.word(va);
        let layout = [
            (0x1009_7618, 0x3fff_0000),
            (0x1009_761c, 16),
            (0x1009_7620, 0xffff),
            (0x1009_7634, 0x3ff0_0000),
            (0x1009_7638, 20),
            (0x1009_763c, 0xf_ffff),
        ];
        if layout.iter().any(|&(va, v)| word(va) != Some(v)) {
            return None;
        }
        let floats = |va: u32, n: usize| pe.floats(word(va)?, n);
        Some(Tables {
            sinc_offset: floats(0x1009_760c, 16384)?,
            sinc_slope: floats(0x1009_7610, 16384)?,
            window_offset: floats(0x1009_7628, 1024)?,
            window_slope: floats(0x1009_762c, 1024)?,
        })
    }

    /// The Kaiser window at `t` (`|t| < 1`), else 0.
    fn window(&self, t: f64) -> f32 {
        let mut v = t as f32;
        if v <= -1.0 {
            return 0.0;
        }
        if v < 0.0 {
            v *= -1.073_741_8e9;
        } else if v < 1.0 {
            v *= 1.073_741_8e9;
        } else {
            return 0.0;
        }
        let i = v as i64 as u32;
        let at = ((i & 0x3ff0_0000) >> 20) as usize;
        (i & 0xf_ffff) as f32 * self.window_slope[at] + self.window_offset[at]
    }

    /// The filter's value at `x` zero crossings from its centre (`1002c5f0`, `x * hl` scaled by
    /// `1 / hl` for the window).
    fn coefficient(&self, x: f64, per_half: f64) -> f32 {
        let w = x * per_half;
        if x > -35.0 && x < 35.0 {
            let mut v = x as f32;
            let s = if v <= -35.0 || v >= 35.0 {
                0.0
            } else {
                if v < 0.0 {
                    v *= -30_678_338.0;
                } else {
                    v *= 30_678_338.0;
                }
                let i = v as i64 as u32;
                let at = ((i & 0x3fff_0000) >> 16) as usize;
                (i & 0xffff) as f32 * self.sinc_slope[at] + self.sinc_offset[at]
            };
            self.window(w) * s
        } else {
            let s = if x > -1.000_000_011_686_097_4e-7 && x < 1.000_000_011_686_097_4e-7 {
                1.0
            } else {
                // The DLL's pi, cut after ten decimals (`10027480`).
                #[allow(clippy::approx_constant)]
                let a = x * 3.141_592_653_5;
                a.sin() / a
            };
            (f64::from(self.window(w)) * s) as f32
        }
    }
}

/// Enough of a PE file to read words and floats at virtual addresses.
struct Pe<'a> {
    file: &'a [u8],
    base: u32,
    sections: Vec<(u32, u32, u32, u32)>,
}

impl<'a> Pe<'a> {
    fn new(file: &'a [u8]) -> Option<Pe<'a>> {
        let u16_at =
            |o: usize| Some(u16::from_le_bytes(file.get(o..o + 2)?.try_into().ok()?) as usize);
        let u32_at = |o: usize| Some(u32::from_le_bytes(file.get(o..o + 4)?.try_into().ok()?));
        let pe = u32_at(0x3c)? as usize;
        if file.get(pe..pe + 4)? != b"PE\0\0" || u16_at(pe + 4)? != 0x14c {
            return None;
        }
        let count = u16_at(pe + 6)?;
        let optional = u16_at(pe + 20)?;
        let base = u32_at(pe + 24 + 28)?;
        let first = pe + 24 + optional;
        let sections = (0..count)
            .map(|i| {
                let o = first + 40 * i;
                Some((
                    u32_at(o + 12)?,
                    u32_at(o + 8)?,
                    u32_at(o + 20)?,
                    u32_at(o + 16)?,
                ))
            })
            .collect::<Option<Vec<_>>>()?;
        Some(Pe {
            file,
            base,
            sections,
        })
    }

    fn offset(&self, va: u32, len: usize) -> Option<usize> {
        let rva = va.checked_sub(self.base)?;
        self.sections.iter().find_map(|&(at, size, raw, raw_size)| {
            let end = at + size.max(raw_size);
            (rva >= at && rva < end && (rva - at) as usize + len <= raw_size as usize)
                .then(|| (raw + rva - at) as usize)
        })
    }

    fn word(&self, va: u32) -> Option<u32> {
        let o = self.offset(va, 4)?;
        Some(u32::from_le_bytes(
            self.file.get(o..o + 4)?.try_into().ok()?,
        ))
    }

    fn floats(&self, va: u32, n: usize) -> Option<Vec<f32>> {
        let o = self.offset(va, 4 * n)?;
        let bytes = self.file.get(o..o + 4 * n)?;
        Some(
            bytes
                .chunks(4)
                .map(|c| f32::from_le_bytes([c[0], c[1], c[2], c[3]]))
                .collect(),
        )
    }
}

/// `round` as the x87 unit's `FISTP` does it (to nearest, halves to even).
fn rint(x: f64) -> i64 {
    let f = x.floor();
    let d = x - f;
    let f = f as i64;
    if d > 0.5 || (d == 0.5 && f % 2 != 0) {
        f + 1
    } else {
        f
    }
}

/// `CVTTSD2SI`: towards zero.
fn trunc32(x: f64) -> i64 {
    x as i32 as i64
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

/// What `RESAMPLEDMO`'s design (`1002bfe0`) leaves for one pair of rates and a phase.
#[derive(Debug)]
pub struct Design {
    /// The rates as given (`f32`, `+0x58`, `+0x5c`) and over their gcd after rounding each to a
    /// whole number (`1002e065`): `m` input frames per `l` output frames.
    rin: f32,
    rout: f32,
    m: i64,
    l: i64,
    /// Both rates whole numbers (`+0x60` clear).
    whole: bool,
    /// Converting at all (`+0x74`): not equal rates on a whole phase.
    on: bool,
    /// The cut-off's scale (`+0x130`), the filter's half width in input frames (`+0xb0`), its
    /// span (`+0x78`), and for the table: taps before the centre (`+0xfc`) and per phase
    /// (`+0xf8`).
    s130: f64,
    half: f64,
    span: i64,
    fc: i64,
    taps: i64,
    /// The polyphase table (`+0xd8`, `l` rows of `taps`, the last input first), when the DLL
    /// makes one (`+0xd4`): whole rates, at most 1280 phases, fewer than 100000 coefficients.
    table: Option<Vec<f32>>,
}

/// The half filter's other form (`100905c3`, built once, `+0xb8`): 16385 values and slopes of
/// the filter over `[0, hl + 5)` zero crossings in steps of `(hl + 5) / 16384`, from the DLL's
/// `sin` and window table, scaled by `10090554`.
struct Curve {
    values: Vec<f32>,
    slopes: Vec<f32>,
}

/// `(hl + 5) / 2^30` and its inverse (`+0x98`, `+0x90`).
fn fixed_scale() -> (f64, f64) {
    let per = 1.0 / (f64::from(HALF_LENGTH) + 5.0);
    let d90 = per * 1_073_741_824.0;
    (1.0 / d90, d90)
}

/// `100942b0`: the filter at fixed-point argument `p` (2^30 for `hl + 5`).
fn curve_value(t: &Tables, p: i64) -> f32 {
    let (d98, _) = fixed_scale();
    let x = p as f64 * d98;
    let w = x * (1.0 / f64::from(HALF_LENGTH));
    if x <= -35.0 || (p as f32) >= 35.0 {
        let s = if x <= -1.000_000_011_686_097_4e-7 || x >= 1.000_000_011_686_097_4e-7 {
            // The DLL's pi, cut after ten decimals (`10027480`).
            #[allow(clippy::approx_constant)]
            let a = x * 3.141_592_653_5;
            a.sin() / a
        } else {
            1.0
        };
        (f64::from(t.window(w)) * s) as f32
    } else {
        t.coefficient(x, 1.0 / f64::from(HALF_LENGTH))
    }
}

fn curve() -> Option<&'static Curve> {
    static CURVE: OnceLock<Option<Curve>> = OnceLock::new();
    CURVE
        .get_or_init(|| {
            let t = tables()?;
            let n = 16384;
            let step = (1.0f64 / 65536.0) as f32;
            let mut values = vec![0f32; n + 1];
            let mut slopes = vec![0f32; n + 1];
            values[0] = curve_value(t, 0);
            let mut p = 0i64;
            for i in 0..n {
                p += 65536;
                values[i + 1] = curve_value(t, p);
                slopes[i] = (values[i + 1] - values[i]) * step;
            }
            slopes[n] = (curve_value(t, p + 65536) - values[n]) * step;
            // Scaled so that the whole curve, mirrored, adds up to 16384 / (hl + 5) (`10090554`).
            let mut lanes = [0f32; 4];
            for (i, v) in values[..n].iter().enumerate() {
                lanes[i % 4] += v;
            }
            let sum = (lanes[2] + lanes[0]) + (lanes[3] + lanes[1]);
            let target = ((1.0 / (f64::from(HALF_LENGTH) + 5.0)) * 16384.0) as f32;
            let k = target / ((sum + sum) - values[0]);
            for i in 0..n {
                values[i] *= k;
                slopes[i] *= k;
            }
            Some(Curve { values, slopes })
        })
        .as_ref()
}

/// The curve's values and slopes (16385 each), to check them against the DLL's.
pub fn curve_points() -> Option<(&'static [f32], &'static [f32])> {
    curve().map(|c| (c.values.as_slice(), c.slopes.as_slice()))
}

impl Curve {
    fn at(&self, p: u32) -> f32 {
        let i = ((p & 0x3fff_0000) >> 16) as usize;
        (p & 0xffff) as f32 * self.slopes[i] + self.values[i]
    }
}

/// The numbers of the table a converter made (`+0x104`..`+0x120`).
type Built = (i64, i64, u64, u64, i64);

/// The design for `rin` Hz to `rout` Hz at `phase` (`1002bfe0`; `on` from `+0x74`).
fn design(
    t: &Tables,
    rin: f32,
    rout: f32,
    phase: f64,
    on: bool,
    built: &mut Option<Built>,
) -> Design {
    let (ri, ro) = (rint(f64::from(rin)), rint(f64::from(rout)));
    let whole = ri as f32 == rin && ro as f32 == rout;
    let g = gcd(ri, ro).max(1);
    let (m, l) = (ri / g, ro / g);
    if !on {
        return Design {
            rin,
            rout,
            m,
            l,
            whole,
            on,
            s130: 1.0,
            half: 0.0,
            span: 0,
            fc: 0,
            taps: 0,
            table: None,
        };
    }
    let d = f64::from((HALF_LENGTH as f32).ln());
    let fit = ((d * 0.067_308_02 - d * 0.005_794_964 * d) + 0.804_445_578) as f32;
    let cut = (rin / rout).min(1.0) * fit;
    let mut half = f64::from(HALF_LENGTH);
    if rout < rin {
        half *= f64::from(rin) / f64::from(rout);
    }
    let s130 = f64::from(rin.max(rout) / (rin / cut).max(rout));
    half /= s130;
    let span = (half + half + 1.0).ceil() as i64 + 5;
    let fc = half.ceil() as i64;
    let taps = (2 * fc + 4) & !3;
    // The table is made once (`+0x101`): a later design with other numbers takes the curve.
    let key = (l, m, phase.to_bits(), s130.to_bits(), fc);
    let eligible = whole && (fc + 1) * l < 100_000 && l.max(m) <= 0x500;
    let table = match (eligible, *built) {
        (false, _) => None,
        (true, None) => {
            *built = Some(key);
            Some(make_table(t, l, m, fc, taps, s130, phase))
        }
        (true, Some(k)) if k == key => Some(make_table(t, l, m, fc, taps, s130, phase)),
        (true, Some(_)) => None,
    };
    Design {
        rin,
        rout,
        m,
        l,
        whole,
        on,
        s130,
        half,
        span,
        fc,
        taps,
        table,
    }
}

/// The polyphase table (`1002bfe0`'s loops): coefficient `i` of the upsampled filter at
/// `i * s130 / max(l, m) + s130 * phase`; at phase 0 the negative half is the positive one's
/// mirror image. Normalised to sum to `l` (`1002b919`).
fn make_table(t: &Tables, l: i64, m: i64, fc: i64, taps: i64, s130: f64, phase: f64) -> Vec<f32> {
    let step = s130 / l.max(m) as f64;
    let offset = s130 * phase;
    let per_half = 1.0 / f64::from(HALF_LENGTH);
    let (tu, last) = (taps as usize, taps - 2 * fc - 1);
    let mut table = vec![0f32; tu * l as usize];
    for pass in 0..2 {
        for r in 0..l {
            let mut at = (r as usize + 1) * tu;
            for j in -fc..=fc + last {
                at -= 1;
                let i = l * j + r;
                let mirrored = phase == 0.0 && i < 0 && i > -(fc + 1) * l;
                if pass == 0 && !mirrored {
                    table[at] = t.coefficient(i as f64 * step + offset, per_half);
                } else if pass == 1 && mirrored {
                    let q = i / l;
                    let from = (1 + l * q - i) * taps - fc + q - 1;
                    table[at] = table[from as usize];
                }
            }
        }
    }
    let sum = table.iter().fold(0f32, |s, &v| s + v);
    if sum != 0.0 {
        let k = l as f32 / sum;
        for v in &mut table {
            *v *= k;
        }
    }
    table
}

/// The sum of `c[i] * x[i]` in the kernels' order (`1002c760`, `100916a8`): one at a time until
/// the input's place in the ring is 16-byte aligned, then eight at a time in two groups of four
/// lanes, the groups added, four more in the lanes, the rest one at a time, and the lanes added
/// in order to the single sum. `start` is the running sum carried over a wrap of the ring.
fn dot(c: &[f32], x: &[f32], place: usize, mut start: f32) -> f32 {
    let n = c.len();
    let mut k = 0;
    while k < n && (place + k) % 4 != 0 {
        start += c[k] * x[k];
        k += 1;
    }
    let (mut low, mut high) = ([0f32; 4], [0f32; 4]);
    while n - k >= 8 {
        for i in 0..4 {
            low[i] += c[k + i] * x[k + i];
            high[i] += c[k + 4 + i] * x[k + 4 + i];
        }
        k += 8;
    }
    for i in 0..4 {
        high[i] += low[i];
    }
    if n - k >= 4 {
        for i in 0..4 {
            high[i] += c[k + i] * x[k + i];
        }
        k += 4;
    }
    while k < n {
        start += c[k] * x[k];
        k += 1;
    }
    ((high[0] + high[1]) + high[2]) + high[3] + start
}

/// The converter's ring of input frames (`+0xe0`): `oldest` and `last` are the first and last
/// frame indices it holds (`+8`, `+0xc`), `next` the slot the next frame goes to (`+0x10`).
#[derive(Debug, Clone)]
struct Ring {
    data: Vec<Vec<f32>>,
    size: i64,
    oldest: i64,
    last: i64,
    next: i64,
}

impl Ring {
    fn slot(&self, k: i64) -> usize {
        (k - self.oldest + self.next).rem_euclid(self.size) as usize
    }

    /// Takes frame `i` of `frames` (a `Vec` per channel) as the newest.
    fn push(&mut self, frames: &[Vec<f32>], i: usize) {
        for (c, d) in self.data.iter_mut().enumerate() {
            d[self.next as usize] = frames[c][i];
        }
        self.next += 1;
        if self.next == self.size {
            self.next = 0;
        }
        self.oldest += 1;
        self.last += 1;
    }

    /// `1002d94a`: emptied, holding frames `oldest..=last`, the next going to slot `next`.
    fn reset(&mut self, oldest: i64, last: i64, next: i64) {
        for d in &mut self.data {
            d.iter_mut().for_each(|v| *v = 0.0);
        }
        (self.oldest, self.last, self.next) = (oldest, last, next);
    }

    /// `1002b3ca`: at least `size` slots, the frames kept (`1008f73c`).
    fn grow(&mut self, size: i64) {
        if self.size >= size {
            return;
        }
        let mut new = Ring {
            data: vec![vec![0f32; size as usize]; self.data.len()],
            size,
            oldest: self.last - size + 1,
            last: self.last,
            next: 0,
        };
        for k in self.oldest.max(new.oldest)..=self.last.min(new.last) {
            let (from, to) = (self.slot(k), new.slot(k));
            for c in 0..self.data.len() {
                new.data[c][to] = self.data[c][from];
            }
        }
        *self = new;
    }
}

/// The input of one `Process` call: `frames` frames, read in order.
struct Block<'a> {
    frames: &'a [Vec<f32>],
    read: usize,
}

impl Block<'_> {
    fn len(&self) -> i64 {
        self.frames.first().map_or(0, |c| c.len() as i64)
    }

    /// The reader (`1002d760`): up to `limit` more frames into the ring.
    fn read(&mut self, ring: &mut Ring, limit: i64) -> i64 {
        let n = limit.min(self.len() - self.read as i64).max(0);
        for _ in 0..n {
            ring.push(self.frames, self.read);
            self.read += 1;
        }
        n
    }
}

/// Buffers a converter keeps from one period to the next so that a period allocates nothing once
/// they have grown. They hold no state: a period writes what it reads of them first.
#[derive(Debug, Clone, Default)]
struct Scratch {
    /// The period's input frames, a `Vec` per channel.
    block: Vec<Vec<f32>>,
    /// One input frame, all its channels.
    frame: Vec<f32>,
    /// The first converter's frames and coefficients of one output frame ([`Dmo::convert`]).
    xs: Vec<f32>,
    cs: Vec<f32>,
}

impl Scratch {
    /// The period's input: `need` frames from frame `from` on, each read with `input` into a
    /// frame of `channels` zeros at first (as a new one), kept by channel in `block` (`need`
    /// long each).
    fn gather(
        &mut self,
        input: &mut dyn FnMut(u64, &mut [f32]),
        from: u64,
        need: usize,
        channels: usize,
    ) {
        self.block.resize_with(channels, Vec::new);
        for b in &mut self.block {
            b.clear();
            b.resize(need, 0.0);
        }
        self.frame.clear();
        self.frame.resize(channels, 0.0);
        for i in 0..need {
            input(from + i as u64, &mut self.frame);
            for c in 0..channels {
                self.block[c][i] = self.frame[c];
            }
        }
    }
}

/// One buffer's `RESAMPLEDMO` converter: the DLL's channel object (`+0x84`... below are its
/// offsets) for the buffer's channels, driven as `dsound.dll`'s APO drives it: each 10 ms
/// period `GetRequiredInputFrames` frames in, `Process` of the period's frames out, the frames
/// it does not use dropped.
#[derive(Debug, Clone)]
pub struct Dmo {
    design: Arc<Design>,
    channels: usize,
    /// The device's rate and the ratio last given (`dsound` `10063d20`, `+0x98`).
    device: f32,
    ratio: f32,
    /// The last input frame taken and the last output frame made (`+0x84`, `+0x88`).
    f84: i64,
    f88: i64,
    f118: f64,
    f128: f64,
    f130: f64,
    f138: i64,
    f140: f64,
    f180: i64,
    f184: i64,
    f188: f32,
    f190: f64,
    f1a0: f64,
    f1b0: f64,
    f58: f64,
    f60: f64,
    ring: Ring,
    /// Frames handed to it so far.
    fed: u64,
    built: Option<Built>,
    scratch: Scratch,
}

impl Dmo {
    /// For a buffer at `rate` Hz on a `device` Hz device (`LockForProcess`, `Init` with half
    /// length 30): `None` without the player's `RESAMPLEDMO.DLL` tables when it must convert.
    pub fn new(rate: u32, device: u32, channels: usize) -> Option<Dmo> {
        let mut built = None;
        let (rin, rout) = (rate as f32, device as f32);
        let on = rin != rout;
        let t = if on { Some(tables()?) } else { None };
        let design = Arc::new(match t {
            Some(t) => design(t, rin, rout, 0.0, true, &mut built),
            None => design_off(rin, rout),
        });
        if design.table.is_none() && design.on {
            curve()?;
        }
        let ring = Ring {
            data: vec![vec![0f32; (2 * design.span + 500) as usize]; channels],
            size: 2 * design.span + 500,
            oldest: 0,
            last: 0,
            next: 0,
        };
        let mut d = Dmo {
            design,
            channels,
            device: rout,
            ratio: rout / rin,
            f84: -1,
            f88: -1,
            f118: 0.0,
            f128: 0.0,
            f130: 0.0,
            f138: 0,
            f140: 0.0,
            f180: 0,
            f184: 0,
            f188: 1.0,
            f190: 0.0,
            f1a0: 0.0,
            f1b0: 0.0,
            f58: 0.0,
            f60: 0.0,
            ring,
            fed: 0,
            built,
            scratch: Scratch::default(),
        };
        d.apply();
        let size = d.ring.size;
        d.ring.reset(-size, -1, 0);
        Some(d)
    }

    /// `1002b750` (with `1002b6b0`): the channel's numbers from the design.
    fn apply(&mut self) {
        let d = self.design.clone();
        let (rin, rout) = (d.rin, d.rout);
        self.f58 = 1.0 / f64::from(rin);
        self.f60 = 1.0 / f64::from(rout);
        let per_out = 1.0 / f64::from(rout);
        let per_in = 1.0 / f64::from(rin);
        self.f140 = d.half;
        self.f118 = per_out * f64::from(rin);
        let low = f64::from(rin.min(rout)) * d.s130;
        self.f130 = low * per_out;
        self.f128 = low * per_in;
        self.f138 = trunc32(self.f128 * fixed_scale().1 + 0.5);
        self.f180 = d.m / d.l.max(1);
        self.f184 = d.m - self.f180 * d.l;
        self.f188 = if d.table.is_some() || !d.on {
            1.0
        } else {
            let k = if rout > rin { 1.0 } else { rout / rin };
            d.s130 as f32 * k
        };
    }

    /// Frames handed to it so far.
    pub fn fed(&self) -> u64 {
        self.fed
    }

    /// A new frequency while it plays (`SetFrequency` of 1/500 of the rate or less): `dsound`
    /// gives the ratio `(f32)device / (f32)frequency` and, at the next period's
    /// `CalcInputFrames`, calls `SetRate(device / ratio, device)` when it changed (`10063d20`).
    pub fn retune(&mut self, frequency: u32) {
        let ratio = self.device / frequency as f32;
        if ratio == self.ratio {
            return;
        }
        self.ratio = ratio;
        self.set_rate(self.device / ratio, self.device);
    }

    /// `SetRate` (`10093ae0`): new rates, the output and input positions carried over in time.
    fn set_rate(&mut self, a: f32, b: f32) {
        let old = self.design.clone();
        let (old_in, old_out) = (old.rin, old.rout);
        if a == old_in && b == old_out {
            return;
        }
        let (old84, old88) = (self.f84, self.f88);
        let (mut n, mut x4) = (old88 + 1, 0.0);
        if b != old_out {
            x4 = (old88 + 1) as f64 / f64::from(old_out) + self.f190;
            let k = rint(f64::from(b) * x4);
            x4 -= k as f64 / f64::from(b);
            n = k;
        }
        let mut k = old84 + 1;
        if a != old_in {
            let (oi, oo) = (f64::from(old_in), f64::from(old_out));
            let mut x1 = (old88 + 1) as f64 / oo + self.f190;
            let mut x3 = old84 as f64 + 1.0;
            x1 *= oi;
            x3 -= x1;
            x1 /= oi;
            x3 /= f64::from(a);
            x3 += x1;
            k = rint(f64::from(a) * x3);
            x3 -= k as f64 / f64::from(a);
            x4 -= x3;
        }
        let mut phase = 0.0;
        if x4.abs() > 1.0 {
            let e = x4.floor();
            n += e as i64;
            x4 -= e;
            phase = f64::from(b.min(a)) * x4;
        }
        let on = !(a == b && x4 == 0.0);
        let d = match (on, tables()) {
            (true, Some(t)) => design(t, a, b, phase, true, &mut self.built),
            _ => design_off(a, b),
        };
        self.design = Arc::new(d);
        if on {
            self.ring.grow(2 * self.design.span + 500);
        }
        self.apply();
        let d = self.design.clone();
        self.f190 = x4;
        self.f1a0 = f64::from(d.rin) * x4;
        let f1a8 = f64::from(d.rout.min(d.rin)) * x4;
        self.f1b0 = f1a8 * d.s130;
        self.f88 = n - 1;
        self.f84 = k - 1;
        let shift = old84 - k + 1;
        self.ring.oldest -= shift;
        self.ring.last -= shift;
    }

    /// `GetRequiredInputFrames(frames)` (`10093f80`).
    fn need(&self, frames: i64) -> i64 {
        let d = &self.design;
        let n = if !d.on {
            self.f88 - self.f84 + frames
        } else {
            let x = (self.f88 + frames) as f64 * self.f118 + self.f1a0;
            let last = if d.table.is_none() {
                (x + self.f140).ceil() as i64
            } else {
                x.floor() as i64 + d.fc
            };
            last - self.f84 + 1
        };
        n.max(0)
    }

    /// One period: `frames` output frames into `out` (a `Vec` per channel), reading the next
    /// input frames with `input` (frame `k` counted from the start).
    pub fn period(
        &mut self,
        frames: usize,
        input: &mut dyn FnMut(u64, &mut [f32]),
        out: &mut [Vec<f32>],
    ) {
        let need = self.need(frames as i64) as usize;
        let mut scratch = std::mem::take(&mut self.scratch);
        scratch.gather(input, self.fed, need, self.channels);
        self.fed += need as u64;
        let mut block = Block {
            frames: &scratch.block,
            read: 0,
        };
        let before = out[0].len();
        if !self.design.on {
            self.pass(&mut block, frames as i64, out);
        } else {
            self.convert(
                &mut block,
                frames as i64,
                out,
                &mut scratch.xs,
                &mut scratch.cs,
            );
        }
        self.scratch = scratch;
        // A short period (not enough input) is padded, as the APO's output is.
        for o in out.iter_mut() {
            o.resize(before + frames, 0.0);
        }
    }

    /// The pass-through (`1009305d`): what the ring still holds, then the input itself.
    fn pass(&mut self, block: &mut Block, frames: i64, out: &mut [Vec<f32>]) {
        let held = frames.min(self.ring.last - self.f88);
        if self.ring.size != 0 && held > 0 {
            for k in self.f88..self.f88 + held {
                let s = self.ring.slot(k);
                for (c, o) in out.iter_mut().enumerate() {
                    o.push(self.ring.data[c][s]);
                }
            }
            self.f88 += held;
        }
        let rest = (frames - held.max(0)).min(block.len());
        if rest > 0 {
            for i in 0..rest as usize {
                for (c, o) in out.iter_mut().enumerate() {
                    o.push(block.frames[c][i]);
                }
            }
            self.f88 += rest;
            self.f84 += rest;
        }
        let size = self.ring.size;
        self.ring.reset(self.f84 - size + 1, self.f84, 0);
        self.rebase();
    }

    /// The table kernel (`1002c760`) or the curve kernel (`100916a8`); `xs` and `cs` are
    /// [`Scratch`]'s buffers (at least 4096 long here, each frame writes what it reads).
    fn convert(
        &mut self,
        block: &mut Block,
        frames: i64,
        out: &mut [Vec<f32>],
        xs: &mut Vec<f32>,
        cs: &mut Vec<f32>,
    ) {
        let d = self.design.clone();
        let n_in = block.len();
        self.f84 += n_in;
        let first_n = self.f88 + 1;
        let end_n = self.f88 + frames;
        let mut local44 = self.ring.last + 1;
        let mut split_at = local44 - self.ring.next;
        let limit_read = d.span + 500;
        for scratch in [&mut *xs, &mut *cs] {
            if scratch.len() < 4096 {
                scratch.resize(4096, 0.0);
            }
        }
        self.f88 = first_n;
        if let Some(table) = &d.table {
            let (l, m) = (d.l, d.m);
            let mut q = (m * first_n).div_euclid(l);
            let mut r = m * first_n - l * q;
            while self.f88 <= end_n {
                let newest = d.fc + q;
                let first = newest - d.taps + 1;
                let needed = newest - self.ring.last;
                if needed > 0 {
                    let e = self.ring.size - self.ring.last + first;
                    let lim = if e <= limit_read { e - 1 } else { limit_read };
                    let got = block.read(&mut self.ring, lim);
                    if got < needed {
                        break;
                    }
                    local44 += got;
                    split_at = local44 - self.ring.next;
                }
                let row = &table[(r * d.taps) as usize..((r + 1) * d.taps) as usize];
                self.sum_out(row, first, newest, split_at, xs, out);
                r += self.f184;
                q += self.f180;
                if r >= l {
                    r -= l;
                    q += 1;
                }
                self.f88 += 1;
            }
        } else {
            let curve = curve().expect("RESAMPLEDMO's curve");
            let d90 = fixed_scale().1;
            let (mut x2, mut x3) = (0.0f64, 0.0f64);
            let mut counter = 0;
            while self.f88 <= end_n {
                if counter == 0 {
                    x2 = self.f88 as f64 * self.f118 + self.f1a0;
                    x3 = self.f88 as f64 * self.f130 + self.f1b0;
                }
                let lo = rint(x2 - self.f140);
                let hi = rint(x2 + self.f140);
                let count = ((hi - lo) + 4) & !3;
                let first = hi - count + 1;
                let needed = hi - self.ring.last;
                if needed > 0 {
                    let e = (first - 4) + (self.ring.size - self.ring.last);
                    let lim = limit_read.min(e);
                    let got = block.read(&mut self.ring, lim);
                    if got < needed {
                        break;
                    }
                    local44 += got;
                    split_at = local44 - self.ring.next;
                }
                // The coefficients from the curve, from the first input on (`100919a2`).
                let mut pos = trunc32((x3 - first as f64 * self.f128) * d90);
                let taps = (hi - first + 1) as usize;
                if cs.len() < taps {
                    cs.resize(taps, 0.0);
                }
                for c in cs.iter_mut().take(taps) {
                    let at = if pos > 0 { pos } else { -pos };
                    *c = curve.at(at as u32);
                    pos = (pos as i32).wrapping_sub(self.f138 as i32) as i64;
                }
                let row = &cs[..taps];
                self.sum_out(row, first, hi, split_at, xs, out);
                counter = if counter != 199 { counter + 1 } else { 0 };
                x3 += self.f130;
                x2 += self.f118;
                self.f88 += 1;
            }
        }
        self.f88 -= 1;
        // The input it did not use: read on as far as the next frame needs (`1002d3fb`).
        let unused = n_in - block.read as i64;
        self.f84 -= unused;
        let x = (self.f88 + 2) as f64 * self.f118 + self.f1a0 + self.f140;
        let lim = x.floor() as i64 - self.f84 - 1;
        let more = unused.min(lim);
        if more > 0 {
            let got = block.read(&mut self.ring, more);
            self.f84 += got;
        }
        self.rebase();
    }

    /// One output frame of each channel: the sum of the ring's frames `first..=newest` times
    /// `row`, split where the ring wraps (`split_at`), times the gain.
    fn sum_out(
        &self,
        row: &[f32],
        first: i64,
        newest: i64,
        split_at: i64,
        xs: &mut Vec<f32>,
        out: &mut [Vec<f32>],
    ) {
        let n = (newest - first + 1) as usize;
        if xs.len() < n {
            xs.resize(n, 0.0);
        }
        let sp = if first <= split_at {
            split_at.min(newest + 1)
        } else {
            first
        };
        let place = self.ring.slot(first);
        for (c, o) in out.iter_mut().enumerate() {
            for (e, x) in xs.iter_mut().take(n).enumerate() {
                *x = self.ring.data[c][self.ring.slot(first + e as i64)];
            }
            let v = if sp == newest + 1 || sp == first {
                dot(row, &xs[..n], place, 0.0)
            } else {
                let k = (sp - first) as usize;
                let s = dot(&row[..k], &xs[..k], place, 0.0);
                dot(&row[k..], &xs[k..n], 0, s)
            };
            o.push(v * self.f188);
        }
    }

    /// Keeps the counters small (`1002d4ea`, `10091490`): past 100000 frames (or, with a
    /// table, past 10^7 in their product with the larger of `l` and `m`), whole rates take off
    /// whole cycles; other rates move the start of time to the input frame now.
    fn rebase(&mut self) {
        let d = self.design.clone();
        let big = self.f84 > 100_000 || self.f88 > 100_000;
        let product = d.table.is_some() || !d.on;
        if !big && !(product && self.f84.max(self.f88) * d.m.max(d.l) > 10_000_000) {
            return;
        }
        let shift;
        if d.whole {
            let k = (self.f84 + 1) / d.m;
            shift = k * d.m;
            self.f84 -= shift;
            self.f88 -= d.l * k;
        } else {
            let mut x3 = self.f88 as f64 * self.f60 + self.f190 - self.f84 as f64 * self.f58;
            let e = (x3 / self.f58).floor() as i64;
            x3 -= e as f64 * self.f58;
            self.f190 = x3;
            self.f1a0 = f64::from(d.rin) * x3;
            let f1a8 = f64::from(d.rout.min(d.rin)) * x3;
            self.f1b0 = f1a8 * d.s130;
            shift = e + self.f84;
            self.f84 -= shift;
            self.f88 = 0;
        }
        self.ring.oldest -= shift;
        self.ring.last -= shift;
    }
}

/// The design when not converting (`1002bfe0` with `+0x74` clear).
fn design_off(rin: f32, rout: f32) -> Design {
    let (ri, ro) = (rint(f64::from(rin)), rint(f64::from(rout)));
    let g = gcd(ri, ro).max(1);
    Design {
        rin,
        rout,
        m: ri / g,
        l: ro / g,
        whole: ri as f32 == rin && ro as f32 == rout,
        on: false,
        s130: 1.0,
        half: 0.0,
        span: 0,
        fc: 0,
        taps: 0,
        table: None,
    }
}

/// `dsound.dll`'s own converter (the "APL" source-rate APO, class at `10002d30`): 4-point
/// Lagrange interpolation between the two middle frames at the fraction `t`.
#[derive(Debug, Clone)]
pub struct Apl {
    /// Input frames per output frame: `1 / ratio` with `ratio = (f32)device / (f32)frequency`
    /// (`100636a0`).
    step: f32,
    /// The fraction and the next frame's place against the next period's input (0 or -1).
    t: f32,
    u: i32,
    /// The last four input frames of each channel.
    history: Vec<[f32; 4]>,
    /// The kernel with the factors kept in four lanes (`100212e0`, 16-bit mono): else the
    /// plain formula (`10066870`, `10066fb0`).
    lanes: bool,
    fed: u64,
    scratch: Scratch,
}

/// The lanes' constants (`100133e0`): the cube roots of -1/6, 1/2, -1/2, 1/6 as `f32`.
#[allow(clippy::excessive_precision)]
const LA: f32 = -0.550_321_221;
#[allow(clippy::excessive_precision)]
const LB: f32 = 0.793_700_516;
#[allow(clippy::excessive_precision)]
const LC: f32 = -0.793_700_516;
#[allow(clippy::excessive_precision)]
const LD: f32 = 0.550_321_221;
/// `1 / LA` (`100133b4`), turning the first lane back into the fraction after a period.
#[allow(clippy::excessive_precision)]
const LA_INVERSE: f32 = -1.817_120_55;
const SIXTH: f32 = 1.0 / 6.0;
const THIRD: f32 = 1.0 / 3.0;

/// The plain formula at fraction `t` between `b` and `c`, from `a b c d`.
fn lagrange(a: f32, b: f32, c: f32, d: f32, t: f32) -> f32 {
    let cubic = (d - a) * SIXTH + (b - c) * 0.5;
    let square = (c + a) * 0.5 - b;
    let linear = ((c - a * THIRD) - b * 0.5) - d * SIXTH;
    ((t * cubic + square) * t + linear) * t + b
}

impl Apl {
    /// For a buffer heard at `frequency` on a `device` Hz device, `lanes` for 16-bit mono.
    pub fn new(frequency: u32, device: u32, channels: usize, lanes: bool) -> Apl {
        Apl {
            step: Apl::step_for(frequency, device),
            t: 0.0,
            u: 0,
            history: vec![[0.0; 4]; channels],
            lanes,
            fed: 0,
            scratch: Scratch::default(),
        }
    }

    fn step_for(frequency: u32, device: u32) -> f32 {
        1.0 / (device as f32 / frequency as f32)
    }

    /// A new frequency (`SetFrequency`, Doppler): from the next period.
    pub fn retune(&mut self, frequency: u32, device: u32) {
        self.step = Apl::step_for(frequency, device);
    }

    /// Frames handed to it so far.
    pub fn fed(&self) -> u64 {
        self.fed
    }

    /// One period: `frames` output frames into `out`, reading the next input frames with
    /// `input` (frame `k` counted from the start).
    pub fn period(
        &mut self,
        frames: usize,
        input: &mut dyn FnMut(u64, &mut [f32]),
        out: &mut [Vec<f32>],
    ) {
        // `CalcInputFrames` (`100636a0`).
        let need = ((frames as f32) * self.step + self.t + self.u as f32) as i32 + 1;
        let need = need.max(0) as usize;
        let channels = self.history.len();
        let mut scratch = std::mem::take(&mut self.scratch);
        scratch.gather(input, self.fed, need, channels);
        let block = &scratch.block;
        self.fed += need as u64;
        let whole = self.step as i32;
        let part = self.step - whole as f32;
        let (mut u_end, mut t_end) = (self.u, self.t);
        for c in 0..channels {
            let (u, t) = self.run(&block[c], c, frames, whole, part, &mut out[c]);
            u_end = u;
            t_end = t;
        }
        let last = i32::from(u_end == need as i32);
        for c in 0..channels {
            // The last four frames of the history followed by the block.
            let (h, b) = (&mut self.history[c], &block[c]);
            let k = b.len();
            if k >= 4 {
                h.copy_from_slice(&b[k - 4..]);
            } else {
                h.copy_within(k.., 0);
                h[4 - k..].copy_from_slice(b);
            }
        }
        self.scratch = scratch;
        self.u = last - 1;
        self.t = t_end;
    }

    /// One channel's period: where it ends (the next frame's place, the fraction).
    fn run(
        &self,
        block: &[f32],
        c: usize,
        frames: usize,
        whole: i32,
        part: f32,
        out: &mut Vec<f32>,
    ) -> (i32, f32) {
        let history = &self.history[c];
        let at = |k: i32| -> f32 {
            if k < 0 {
                history[(4 + k) as usize]
            } else {
                block.get(k as usize).copied().unwrap_or(0.0)
            }
        };
        let (mut u, mut t) = (self.u, self.t);
        let mut done = 0;
        while (u < 3 || !self.lanes) && done < frames {
            out.push(lagrange(at(u - 3), at(u - 2), at(u - 1), at(u), t));
            done += 1;
            u += whole;
            t += part;
            while t >= 1.0 {
                t -= 1.0;
                u += 1;
            }
        }
        if !self.lanes {
            return (u, t);
        }
        // The four lanes hold the three factors of each frame's weight (`100210a7`).
        let (t2, t1) = (t - 2.0, t - 1.0);
        let tb = t * LB;
        let mut fa = [t2 * LA, t2 * LB, t2 * LC, t1 * LD];
        let mut fb = [t1 * LA, t1 * LB, t * LC, t * LD];
        let mut fc = [t * LA, tb + LB, LC - tb, t * LD + LD];
        let fd = [part * LA, part * LB, part * LC, part * LD];
        let fe = [LA, LB, LC, LD];
        let mut e = u - 3;
        while done < frames {
            let mut p = [0f32; 4];
            for i in 0..4 {
                p[i] = ((fa[i] * fc[i]) * fb[i]) * at(e + i as i32);
                fa[i] += fd[i];
                fc[i] += fd[i];
                fb[i] += fd[i];
            }
            let v = ((p[2] + p[0]) + p[1]) + p[3];
            if fc[0] <= fe[0] {
                e += 1;
                for i in 0..4 {
                    fc[i] -= fe[i];
                    fb[i] -= fe[i];
                    fa[i] -= fe[i];
                }
            }
            e += whole;
            out.push(v);
            done += 1;
        }
        (e + 3, fc[0] * LA_INVERSE)
    }
}

/// The frames [`Apl`] takes period by period, without the samples: its `CalcInputFrames`
/// (`100636a0`) and where each period ends (the plain formula's steps, or the lanes' first
/// factor), the same `f32` arithmetic. DirectSound's write cursor follows them with the second
/// converter on (measured, `super::dsound`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AplFrames {
    step: f32,
    t: f32,
    u: i32,
    lanes: bool,
    /// Output frames per period (the device's rate / 100).
    frames: usize,
}

impl AplFrames {
    /// As [`Apl::new`].
    pub fn new(frequency: u32, device: u32, lanes: bool) -> AplFrames {
        AplFrames {
            step: Apl::step_for(frequency, device),
            t: 0.0,
            u: 0,
            lanes,
            frames: (device / 100) as usize,
        }
    }

    /// As [`Apl::retune`]: from the next period.
    pub fn retune(&mut self, frequency: u32, device: u32) {
        self.step = Apl::step_for(frequency, device);
    }

    /// One period: the input frames taken.
    pub fn period(&mut self) -> u64 {
        let need = ((self.frames as f32) * self.step + self.t + self.u as f32) as i32 + 1;
        let need = need.max(0);
        let (whole, part) = (self.step as i32, self.step - (self.step as i32) as f32);
        let (mut u, mut t, mut done) = (self.u, self.t, 0);
        while (u < 3 || !self.lanes) && done < self.frames {
            done += 1;
            u += whole;
            t += part;
            while t >= 1.0 {
                t -= 1.0;
                u += 1;
            }
        }
        if self.lanes {
            // [`Apl::run`]'s first lane alone decides where the frames step.
            let mut fc = t * LA;
            let fd = part * LA;
            let mut e = u - 3;
            while done < self.frames {
                fc += fd;
                if fc <= LA {
                    e += 1;
                    fc -= LA;
                }
                e += whole;
                done += 1;
            }
            (u, t) = (e + 3, fc * LA_INVERSE);
        }
        self.u = i32::from(u == need) - 1;
        self.t = t;
        need as u64
    }
}

/// A buffer's converter.
#[derive(Debug, Clone)]
pub enum Converter {
    Dmo(Box<Dmo>),
    Apl(Apl),
}

impl Converter {
    /// Frames handed to it so far.
    pub fn fed(&self) -> u64 {
        match self {
            Converter::Dmo(d) => d.fed(),
            Converter::Apl(a) => a.fed(),
        }
    }

    pub fn period(
        &mut self,
        frames: usize,
        input: &mut dyn FnMut(u64, &mut [f32]),
        out: &mut [Vec<f32>],
    ) {
        match self {
            Converter::Dmo(d) => d.period(frames, input, out),
            Converter::Apl(a) => a.period(frames, input, out),
        }
    }

    /// A new frequency while it plays (the second converter: from the next period; the first:
    /// its `SetRate` at the next period).
    pub fn retune(&mut self, frequency: u32, device: u32) {
        match self {
            Converter::Dmo(d) => d.retune(frequency),
            Converter::Apl(a) => a.retune(frequency, device),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The test signal of `scripts/dsmix-measure` (16-bit, mono).
    fn signal(frames: usize) -> Vec<f32> {
        let mut x = 0x1234_5678u32;
        (0..frames)
            .map(|_| {
                x ^= x << 13;
                x ^= x >> 17;
                x ^= x << 5;
                ((x % 2001) as i32 - 1000) as f32 / 32768.0
            })
            .collect()
    }

    fn run(mut c: Converter, x: &[f32], periods: usize) -> Vec<f32> {
        let mut out = vec![Vec::new()];
        for _ in 0..periods {
            c.period(
                480,
                &mut |k, f| f[0] = x.get(k as usize).copied().unwrap_or(0.0),
                &mut out,
            );
        }
        out.remove(0)
    }

    /// What `dsound.dll` handed the audio engine (`DSMIX_HOOK`, 2026-10-28) for the test signal:
    /// output frames 0..8 and 20000..20004, `f32` bits.
    const APL_44100: [(usize, u32); 6] = [
        (0, 0),
        (1, 0xba35_cf0b),
        (2, 0x3cce_4849),
        (3, 0x3cc7_92f4),
        (20000, 0xbc0b_a97f),
        (20001, 0xbaba_986d),
    ];
    /// The same for the first converter, 22100 frames at 44100 Hz (`src_end_22100`), up to its
    /// last sound (24087).
    const DMO_44100: [(usize, u32); 7] = [
        (0, 0x3ce9_dd04),
        (1, 0x3cb7_2578),
        (2, 0xbc0d_bbed),
        (3, 0xbc82_d357),
        (20000, 0xbc07_77c1),
        (20001, 0xb8f9_0a20),
        (24087, 0xb283_e296),
    ];

    /// [`AplFrames`] takes what [`Apl`] takes, period by period: both kernels, rates up, down and
    /// the device's own, retuned while it runs.
    #[test]
    fn the_frame_counter_takes_what_the_converter_takes() {
        let x = signal(4000);
        for lanes in [true, false] {
            for (frequency, retuned) in [
                (22050, 16537),
                (11025, 11040),
                (22071, 22050),
                (48000, 44100),
                (44100, 60000),
                (16537, 8000),
            ] {
                let mut apl = Apl::new(frequency, 48000, 1, lanes);
                let mut counter = AplFrames::new(frequency, 48000, lanes);
                let mut out = vec![Vec::new()];
                for period in 0..60 {
                    if period == 25 {
                        apl.retune(retuned, 48000);
                        counter.retune(retuned, 48000);
                    }
                    let before = apl.fed();
                    apl.period(
                        480,
                        &mut |k, f| f[0] = x.get(k as usize).copied().unwrap_or(0.0),
                        &mut out,
                    );
                    assert_eq!(
                        counter.period(),
                        apl.fed() - before,
                        "{frequency} then {retuned}, lanes {lanes}, period {period}"
                    );
                }
            }
        }
    }

    #[test]
    fn the_constants_are_the_dlls() {
        let bits = [LA, LB, LC, LD, LA_INVERSE, SIXTH, THIRD].map(f32::to_bits);
        assert_eq!(
            bits,
            [
                0xbf0c_e1da,
                0x3f4b_2ff5,
                0xbf4b_2ff5,
                0x3f0c_e1da,
                0xbfe8_9768,
                0x3e2a_aaab,
                0x3eaa_aaab
            ]
        );
        assert_eq!(1.073_741_8e9_f32, (1u32 << 30) as f32);
    }

    #[test]
    fn the_first_converter_designs_as_the_dll_does() {
        let Some(t) = tables() else {
            return; // no RESAMPLEDMO.DLL on this machine
        };
        let d = design(t, 44100.0, 48000.0, 0.0, true, &mut None);
        assert_eq!(
            (d.l, d.m, d.fc, d.taps, 2 * d.span + 500),
            (160, 147, 32, 68, 638)
        );
        assert!(d.table.is_some());
        let d = design(t, 60000.0, 48000.0, 0.0, true, &mut None);
        assert_eq!(
            (d.l, d.m, d.fc, d.taps, 2 * d.span + 500),
            (4, 5, 39, 80, 668)
        );
        // 22071 on 48000 needs 16000 phases: the DLL's curve path.
        let d = design(t, 22071.0, 48000.0, 0.0, true, &mut None);
        assert!(d.table.is_none() && (d.l, d.m) == (16000, 7357));
    }

    #[test]
    fn the_first_converter_asks_for_what_the_dll_asks_for() {
        let Some(mut c) = Dmo::new(44100, 48000, 1).filter(|_| tables().is_some()) else {
            return;
        };
        let x = signal(4000);
        let mut out = vec![Vec::new()];
        let mut fed = Vec::new();
        for _ in 0..3 {
            c.period(480, &mut |k, f| f[0] = x[k as usize], &mut out);
            fed.push(c.fed());
        }
        // `GetRequiredInputFrames(480)` from the DLL: 474, 441, 441.
        assert_eq!(fed, [474, 915, 1356]);
    }

    #[test]
    fn the_first_converter_gives_the_measured_samples() {
        let Some(c) = Dmo::new(44100, 48000, 1) else {
            return;
        };
        let x = signal(22100);
        let out = run(Converter::Dmo(Box::new(c)), &x, 51);
        for (at, bits) in DMO_44100 {
            assert_eq!(out[at].to_bits(), bits, "frame {at}");
        }
    }

    #[test]
    fn the_second_converter_gives_the_measured_samples() {
        let x = signal(22050);
        let out = run(Converter::Apl(Apl::new(44100, 48000, 1, true)), &x, 42);
        for (at, bits) in APL_44100 {
            assert_eq!(out[at].to_bits(), bits, "frame {at}");
        }
    }
}

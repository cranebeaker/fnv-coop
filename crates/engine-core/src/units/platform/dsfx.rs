//! DirectSound's standard distortion effect (`GUID_DSFX_STANDARD_DISTORTION`), as the player's
//! `dsdmo.dll` computes it, and the effect chain `dsound.dll` runs it in. Translated from the
//! instructions of the 32-bit DLLs the game loads (`SysWOW64`, Windows 11 10.0.26200; read in a
//! private analysis with the maintainer's leave, 2026-10-28; addresses are those DLLs', image
//! base `0x10000000`) and checked against what reaches the device (`scripts/dsmix-measure`,
//! docs/SOUND.md).
//!
//! The game sets it on the sounds it marks muffled (`00aec530`, flag `0x80000`): gain
//! `fFilterDistortionGain:Audio` (default -18.2 dB), edge 0, post-EQ centre 5000 Hz and
//! bandwidth 1135 Hz, pre-lowpass cutoff 5000 Hz.
//!
//! Per sample, on the buffer's own samples (`1000d080`), in `f32`:
//! - a one-pole lowpass `lp = x * b + lp * a`;
//! - a waveshaper on `lp * 32768` (`100183c0`): the value's integer part as a float with an
//!   exponent of 2 to 5 bits, chosen by the edge;
//! - a two-pole band-pass (a lattice of two states) after the gain;
//! - the result back to the buffer's format: rounded to nearest even for mono, truncated for
//!   stereo (the DLL's two loops differ), clamped to 16 bits (8-bit buffers divide by 256
//!   toward zero).
//!
//! The coefficients (`1000d8d0`) come from the parameters with the DLL's `f64` and `f32`
//! roundings. One part is a table: the gain correction for each edge setting (32 values, two
//! decimals each, not a formula). It is not in this file: [`edge_table`] measures it from the
//! player's own `dsdmo.dll` at run time, through the effect's public interface.
//!
//! The chain (`dsound.dll`, `CEffectChain`): the effect writes into a copy of the buffer
//! (`PostFxBuffer`) that the mixer plays. When the buffer starts (`Play`, `10034ae0`) the
//! effect's state is cleared (`Discontinuity`) and it runs from 32 frames before the play
//! position (`10046fb8`), so the start hears the end of a looping sound; from then on it keeps
//! ahead of the play cursor. Here the chain runs just ahead of what the mixer reads, so data
//! the game writes into the buffer while it plays is processed as soon as it is due; the DLL
//! runs about 100 ms ahead (its write-ahead), so data written into those 100 ms is heard
//! unprocessed there and processed here (open, docs/SOUND.md).

use std::sync::OnceLock;

/// `DSFXDistortion` in its order: gain (dB), edge (%), post-EQ centre frequency (Hz), post-EQ
/// bandwidth (Hz), pre-lowpass cutoff (Hz).
pub type Params = [f32; 5];

/// The coefficients for `rate` (Hz) and `p`, with the edge correction `table`
/// (`1000d8d0`, called for each parameter in turn by `SetAllParameters`, `1000dea0`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coefficients {
    /// The waveshaper's shift count (`+0x1ec`): `trunc(edge / 100 * 29 + 2)`.
    pub shift: u32,
    /// Gain times the band-pass's normalisation (`+0x1f0`).
    pub gain: f32,
    /// The lowpass's feedback and input weights (`+0x1f4`, `+0x1f8`).
    pub feedback: f32,
    pub input: f32,
    /// The band-pass's `-cos(centre)` and `(1 - tan(bw)) / (1 + tan(bw))` (`+0x1fc`, `+0x200`).
    pub centre: f32,
    pub width: f32,
}

/// `2 pi` as the DLL writes it (`6.283185307`, a decimal literal, not `f64` pi).
#[allow(clippy::approx_constant)] // The DLL's literal, deliberately not `TAU`.
const TWO_PI: f64 = 6.283185307;

impl Coefficients {
    pub fn new(rate: u32, p: Params, table: &[f64; 32]) -> Coefficients {
        let [gain, edge, centre, bandwidth, cutoff] = p;
        let w = f64::from(rate);
        let limit = (rate / 3) as f32;
        let held = |v: f32| if v > limit { limit } else { v };
        // Gain: 10^(gain / 20).
        let level = 10f64.powf(f64::from(gain / 20.0f32)) as f32;
        // Edge.
        let edge = (edge / 100.0f32) * 29.0f32 + 2.0f32;
        let shift = edge as i32;
        // The lowpass: a = sqrt((2 cos(wc) + 3) / 5), b = sqrt(1 - a^2).
        let c = ((f64::from(held(cutoff)) * TWO_PI) / w).cos();
        let a = ((c + c + 3.0) / 5.0).sqrt() as f32;
        let b = (f64::from(1.0f32 - a * a)).sqrt() as f32;
        // The band-pass: its centre (the last `SetParameter(3)`) and its width with the
        // normalisation (the last `SetParameter(4)`, which reads the centre as an `f32`).
        let centre = held(centre);
        let k = -((f64::from(centre) * TWO_PI) / w).cos();
        let k32 = k as f32;
        let arg = (f64::from(held(bandwidth)) * TWO_PI) / w * 0.5;
        let r = ((1.0 - arg.tan()) / (arg.tan() + 1.0)) as f32;
        let (r64, k64) = (f64::from(r), f64::from(k32));
        let norm = ((1.0 - r64 * r64).sqrt() * (1.0 - k64 * k64).sqrt()) as f32;
        let norm = (f64::from(norm) * table[shift as usize]) as f32;
        Coefficients {
            shift: shift as u32,
            gain: norm * level,
            feedback: a,
            input: b,
            centre: k32,
            width: r,
        }
    }
}

/// `cvttss2si`: truncation, `i32::MIN` when out of range or NaN.
fn truncate(v: f32) -> i32 {
    if v.is_nan() || !(-2_147_483_648.0..2_147_483_648.0).contains(&v) {
        i32::MIN
    } else {
        v as i32
    }
}

/// `v` rounded to the nearest whole number, halves to even.
fn ties_even(v: f32) -> f32 {
    let r = v.round();
    if (v - v.trunc()).abs() == 0.5 {
        2.0 * (v / 2.0).round()
    } else {
        r
    }
}

/// `fistp` with the default rounding: nearest, ties to even; `i32::MIN` when out of range.
fn round_even(v: f32) -> i32 {
    let r = ties_even(v);
    if r.is_nan() || !(-2_147_483_648.0..2_147_483_648.0).contains(&r) {
        i32::MIN
    } else {
        r as i32
    }
}

/// The waveshaper (`100183c0`): the integer part of `v`, shifted left until its top bit is set
/// or `shift` steps are used, then packed as `(steps left + 1) << (31 - e) | rest >> e` with an
/// exponent field of `e` bits (2 for shifts to 3, 3 to 7, 4 to 15, 5 above), sign restored by
/// complement, back to `f32`.
pub fn shape(v: f32, shift: u32) -> f32 {
    let mut x = truncate(v) as u32;
    let sign = x & 0x8000_0000;
    if sign != 0 {
        x = x.wrapping_neg();
    }
    let mut left = shift;
    if x < 0x8000_0000 {
        while left != 0 {
            x = x.wrapping_add(x);
            left -= 1;
            if x >= 0x8000_0000 {
                break;
            }
        }
    }
    let (rest, exponent) = if x < 0x8000_0000 {
        (x, left)
    } else {
        (x & 0x7fff_ffff, left + 1)
    };
    let e = match shift {
        16.. => 5,
        8.. => 4,
        4.. => 3,
        _ => 2,
    };
    let packed = (exponent << (31 - e)) | (rest >> e);
    let out = if sign == 0 { packed } else { !packed | sign };
    out as i32 as f32
}

/// One channel's state (`+0x218` lowpass; `+0x220`, `+0x224` band-pass).
#[derive(Debug, Clone, Copy, Default, PartialEq)]
struct Channel {
    lowpass: f32,
    s1: f32,
    s2: f32,
}

/// The effect for one buffer: its coefficients and state.
#[derive(Debug, Clone, PartialEq)]
pub struct Distortion {
    pub c: Coefficients,
    state: [Channel; 2],
}

impl Distortion {
    pub fn new(c: Coefficients) -> Distortion {
        Distortion {
            c,
            state: Default::default(),
        }
    }

    /// `Discontinuity` (`1000d030`): the state cleared.
    pub fn clear(&mut self) {
        self.state = Default::default();
    }

    /// One sample of channel `ch`, in the 16-bit scale.
    fn step(&mut self, ch: usize, x: f32) -> f32 {
        let c = &self.c;
        let s = &mut self.state[ch];
        let lp = x * c.input + s.lowpass * c.feedback;
        s.lowpass = lp;
        let w = shape(lp * 32768.0, c.shift);
        let old = s.s2;
        let y = ((w * (1.0 / 32768.0)) * c.gain - c.width * s.s1) - c.centre * old;
        s.s2 = y;
        s.s1 = c.centre * y + old;
        y
    }

    /// Processes one frame of `bits`-bit PCM (`channels` 1 or 2) in place.
    pub fn frame(&mut self, frame: &mut [u8], channels: u16, bits: u16) {
        for ch in 0..usize::from(channels) {
            let x = if bits == 16 {
                i32::from(i16::from_le_bytes([frame[2 * ch], frame[2 * ch + 1]]))
            } else {
                (i32::from(frame[ch]) - 128) * 256
            };
            let y = self.step(ch, x as f32);
            let v = if channels == 1 {
                round_even(y)
            } else {
                truncate(y)
            }
            .clamp(-32768, 32767);
            if bits == 16 {
                frame[2 * ch..2 * ch + 2].copy_from_slice(&(v as i16).to_le_bytes());
            } else {
                frame[ch] = ((v + if v < 0 { 255 } else { 0 }) >> 8).wrapping_add(128) as u8;
            }
        }
    }
}

/// Frames the chain processes before the play position when the buffer starts (`10046fb8`:
/// 32 block-aligns).
pub const PREROLL: usize = 32;

/// The effect chain on one buffer: the buffer's own samples (the game's, before the effect),
/// the effect, and the frame it processes next. The mixer plays the processed copy (the voice's
/// samples).
#[derive(Debug, Clone, PartialEq)]
pub struct Chain {
    pub source: Vec<u8>,
    pub fx: Distortion,
    pub channels: u16,
    pub bits: u16,
    /// The next frame to process.
    cursor: usize,
}

impl Chain {
    pub fn new(source: Vec<u8>, fx: Distortion, channels: u16, bits: u16) -> Chain {
        Chain {
            source,
            fx,
            channels,
            bits,
            cursor: 0,
        }
    }

    fn align(&self) -> usize {
        usize::from(self.channels) * usize::from(self.bits) / 8
    }

    fn frames(&self) -> usize {
        self.source.len() / self.align()
    }

    /// The buffer starts at frame `at`: the state cleared and the 32 frames before `at`
    /// (wrapping to the end) processed into `out`.
    pub fn start(&mut self, out: &mut [u8], at: usize) {
        self.fx.clear();
        let n = self.frames();
        if n == 0 {
            return;
        }
        self.cursor = (at % n + n - PREROLL % n) % n;
        self.run(out, PREROLL.min(n));
    }

    /// Processes `count` frames from the cursor into `out`, wrapping.
    fn run(&mut self, out: &mut [u8], count: usize) {
        let (align, n) = (self.align(), self.frames());
        for _ in 0..count {
            let at = self.cursor * align;
            let mut frame = [0u8; 4];
            frame[..align].copy_from_slice(&self.source[at..at + align]);
            self.fx.frame(&mut frame[..align], self.channels, self.bits);
            out[at..at + align].copy_from_slice(&frame[..align]);
            self.cursor = (self.cursor + 1) % n;
        }
    }

    /// Processes into `out` far enough that the `ahead` frames from the play position
    /// `position` are done: a non-looping buffer up to its end, a looping one at most one lap.
    pub fn keep_ahead(&mut self, out: &mut [u8], position: usize, ahead: usize, looping: bool) {
        let n = self.frames();
        if n == 0 {
            return;
        }
        let position = position % n;
        // Frames already processed from the play position on.
        let lead = (self.cursor + n - position) % n;
        let want = ahead.min(if looping {
            n - PREROLL.min(n)
        } else {
            n - position
        });
        if want > lead {
            let todo = want - lead;
            // A non-looping buffer is not processed past its end.
            let todo = if looping {
                todo
            } else {
                todo.min(n - self.cursor)
            };
            self.run(out, todo);
        }
    }
}

/// The edge correction table of the player's `dsdmo.dll`, measured once through the effect's
/// own interface ([`measure_edge_table`]); `None` when it cannot be measured (not Windows, no
/// effect).
pub fn edge_table() -> Option<&'static [f64; 32]> {
    static T: OnceLock<Option<[f64; 32]>> = OnceLock::new();
    T.get_or_init(|| std::thread::spawn(measure_edge_table).join().ok().flatten())
        .as_ref()
}

/// Measures the edge correction: for each shift `n` (2 to 31) the effect, on a float stream at
/// 48000 Hz with gain 0 dB, processes one impulse; its first output is
/// `shape(x) / 32768 * norm * table[n]` with the normalisation computed here, so the table entry
/// is the ratio, rounded to the two decimals the DLL's values have. Entries 0 and 1 cannot be
/// reached (the shift is at least 2) and are 1.
pub fn measure_edge_table() -> Option<[f64; 32]> {
    let mut table = [1.0f64; 32];
    for (n, slot) in table.iter_mut().enumerate().skip(2) {
        // An edge whose shift is n: (n - 1.5) / 29 * 100 percent (100 for the last).
        let edge = (((n as f32 - 1.5) / 29.0) * 100.0).min(100.0);
        let p: Params = [0.0, edge, 4000.0, 4000.0, 8000.0];
        let unit = Coefficients::new(48000, p, &[1.0; 32]);
        if unit.shift as usize != n {
            return None;
        }
        let x = 0.5f32;
        let w = shape((x * unit.input) * 32768.0, unit.shift);
        let expected = (w * (1.0 / 32768.0)) * unit.gain;
        let y = effect::first_output(p, x)?;
        let ratio = f64::from(y) / f64::from(expected);
        let v = (ratio * 100.0).round() / 100.0;
        if !(1.0..=2.0).contains(&v) || (ratio - v).abs() > 0.002 {
            return None;
        }
        *slot = v;
    }
    Some(table)
}

/// The player's distortion effect, run once through COM (`dsdmo.dll`).
mod effect {
    #[cfg(windows)]
    pub fn first_output(p: super::Params, x: f32) -> Option<f32> {
        windows::first_output(p, x)
    }

    #[cfg(not(windows))]
    pub fn first_output(_p: super::Params, _x: f32) -> Option<f32> {
        None
    }

    #[cfg(windows)]
    mod windows {
        use std::ffi::c_void;

        #[link(name = "ole32")]
        extern "system" {
            fn CoInitializeEx(reserved: *mut c_void, flags: u32) -> u32;
            fn CoCreateInstance(
                clsid: *const [u8; 16],
                outer: *mut c_void,
                context: u32,
                iid: *const [u8; 16],
                out: *mut *mut Obj,
            ) -> u32;
        }

        #[repr(C)]
        struct Obj {
            vtable: *const usize,
        }

        #[repr(C)]
        struct MediaType {
            major: [u8; 16],
            sub: [u8; 16],
            fixed: i32,
            temporal: i32,
            sample_size: u32,
            format_type: [u8; 16],
            unk: *mut c_void,
            format_size: u32,
            format: *const u8,
        }

        fn guid(s: &str) -> [u8; 16] {
            let h: String = s.chars().filter(|c| c.is_ascii_hexdigit()).collect();
            let b = |i: usize| u8::from_str_radix(&h[2 * i..2 * i + 2], 16).expect("hex");
            [
                b(3),
                b(2),
                b(1),
                b(0),
                b(5),
                b(4),
                b(7),
                b(6),
                b(8),
                b(9),
                b(10),
                b(11),
                b(12),
                b(13),
                b(14),
                b(15),
            ]
        }

        unsafe fn slot(o: *mut Obj, index: usize) -> usize {
            *(*o).vtable.add(index)
        }

        unsafe fn release(o: *mut Obj) {
            if !o.is_null() {
                let f: unsafe extern "system" fn(*mut Obj) -> u32 = std::mem::transmute(slot(o, 2));
                f(o);
            }
        }

        unsafe fn query(o: *mut Obj, iid: &[u8; 16]) -> *mut Obj {
            let f: unsafe extern "system" fn(*mut Obj, *const [u8; 16], *mut *mut Obj) -> i32 =
                std::mem::transmute(slot(o, 0));
            let mut out = std::ptr::null_mut();
            if f(o, iid, &mut out) < 0 {
                std::ptr::null_mut()
            } else {
                out
            }
        }

        pub fn first_output(p: super::super::Params, x: f32) -> Option<f32> {
            // SAFETY: COM calls on interfaces this function creates and releases, with the
            // documented layouts of `DMO_MEDIA_TYPE` and `WAVEFORMATEX`.
            unsafe {
                CoInitializeEx(std::ptr::null_mut(), 0);
                let mut dmo: *mut Obj = std::ptr::null_mut();
                let hr = CoCreateInstance(
                    &guid("ef114c90-cd1d-484e-96e5-09cfaf912a21"),
                    std::ptr::null_mut(),
                    1,
                    &guid("d8ad0f58-5494-4102-97c5-ec798e59bcf4"),
                    &mut dmo,
                );
                if hr != 0 || dmo.is_null() {
                    return None;
                }
                // WAVEFORMATEX: IEEE float, mono, 48000 Hz, 32 bits.
                let mut wf = [0u8; 18];
                wf[0..2].copy_from_slice(&3u16.to_le_bytes());
                wf[2..4].copy_from_slice(&1u16.to_le_bytes());
                wf[4..8].copy_from_slice(&48000u32.to_le_bytes());
                wf[8..12].copy_from_slice(&192000u32.to_le_bytes());
                wf[12..14].copy_from_slice(&4u16.to_le_bytes());
                wf[14..16].copy_from_slice(&32u16.to_le_bytes());
                let mt = MediaType {
                    major: guid("73647561-0000-0010-8000-00aa00389b71"),
                    sub: guid("00000003-0000-0010-8000-00aa00389b71"),
                    fixed: 1,
                    temporal: 0,
                    sample_size: 4,
                    format_type: guid("05589f81-c356-11ce-bf01-00aa0055595a"),
                    unk: std::ptr::null_mut(),
                    format_size: 18,
                    format: wf.as_ptr(),
                };
                type SetType =
                    unsafe extern "system" fn(*mut Obj, u32, *const MediaType, u32) -> i32;
                type Call = unsafe extern "system" fn(*mut Obj) -> i32;
                let set_input: SetType = std::mem::transmute(slot(dmo, 8));
                let set_output: SetType = std::mem::transmute(slot(dmo, 9));
                let allocate: Call = std::mem::transmute(slot(dmo, 18));
                let mut result = None;
                if set_input(dmo, 0, &mt, 0) >= 0
                    && set_output(dmo, 0, &mt, 0) >= 0
                    && allocate(dmo) >= 0
                {
                    let fx = query(dmo, &guid("8ecf4326-455f-4d8b-bda9-8d5d3e9e3e0b"));
                    let inplace = query(dmo, &guid("651b9ad0-0fc7-4aa9-9538-d89931010741"));
                    if !fx.is_null() && !inplace.is_null() {
                        let set_all: unsafe extern "system" fn(*mut Obj, *const f32) -> i32 =
                            std::mem::transmute(slot(fx, 3));
                        let process: unsafe extern "system" fn(
                            *mut Obj,
                            u32,
                            *mut u8,
                            i64,
                            u32,
                        ) -> i32 = std::mem::transmute(slot(inplace, 3));
                        let mut data = [0f32; 16];
                        data[0] = x;
                        if set_all(fx, p.as_ptr()) >= 0
                            && process(inplace, 64, data.as_mut_ptr().cast(), 0, 0) >= 0
                        {
                            result = Some(data[0]);
                        }
                    }
                    release(fx);
                    release(inplace);
                }
                release(dmo);
                result
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A made-up correction table (not the DLL's): the tests check the arithmetic only.
    const TABLE: [f64; 32] = [1.25; 32];

    #[test]
    fn the_waveshaper_packs_the_value() {
        // 3 with shift 2: two doublings (12) use the shift up, exponent 0: 12 >> 2.
        assert_eq!(shape(3.0, 2), 3.0);
        // A negative value is the complement, sign kept: !3 = -4.
        assert_eq!(shape(-3.0, 2), -4.0);
        // 2^30 reaches the top bit after one doubling: exponent (1 left + 1) << 29, rest 0.
        assert_eq!(shape(1_073_741_824.0, 2), (2u32 << 29) as f32);
        // Out of range reads as the integer indefinite value (sign set, magnitude 2^31).
        assert_eq!(shape(5e9, 2), shape(-5e9, 2));
        assert_eq!(shape(0.0, 7), 0.0);
    }

    #[test]
    fn the_coefficients_follow_the_parameters() {
        let c = Coefficients::new(48000, [-18.2, 0.0, 5000.0, 1135.0, 5000.0], &TABLE);
        assert_eq!(c.shift, 2);
        assert!(c.feedback > 0.0 && c.feedback < 1.0);
        assert!((c.input * c.input + c.feedback * c.feedback - 1.0).abs() < 1e-6);
        // Frequencies above a third of the rate are held there: at 11025 Hz the 5000 Hz cutoff
        // and centre are 3675 Hz.
        let low = Coefficients::new(11025, [-18.2, 0.0, 5000.0, 1135.0, 5000.0], &TABLE);
        let held = Coefficients::new(11025, [-18.2, 0.0, 3675.0, 1135.0, 3675.0], &TABLE);
        assert_eq!(low, held);
    }

    #[test]
    fn mono_rounds_and_stereo_truncates() {
        let c = Coefficients::new(48000, [0.0, 0.0, 5000.0, 1135.0, 5000.0], &TABLE);
        let mut mono = Distortion::new(c);
        let mut stereo = Distortion::new(c);
        for x in [1000i16, -2000, 3000, 0, 7] {
            let mut m = x.to_le_bytes();
            let mut s = [x.to_le_bytes(), x.to_le_bytes()].concat();
            let y = {
                let mut d = mono.clone();
                d.step(0, f32::from(x))
            };
            mono.frame(&mut m, 1, 16);
            stereo.frame(&mut s, 2, 16);
            assert_eq!(i16::from_le_bytes(m), ties_even(y) as i16);
            assert_eq!(i16::from_le_bytes([s[0], s[1]]), y as i16);
            assert_eq!(s[0..2], s[2..4]);
        }
    }

    #[test]
    fn the_chain_starts_32_frames_early_and_keeps_ahead() {
        let c = Coefficients::new(48000, [0.0, 0.0, 5000.0, 1135.0, 5000.0], &TABLE);
        let source: Vec<u8> = (0..1000i32)
            .flat_map(|i| (((i * 37) % 2001 - 1000) as i16).to_le_bytes())
            .collect();
        let mut chain = Chain::new(source.clone(), Distortion::new(c), 1, 16);
        let mut out = vec![0u8; source.len()];
        chain.start(&mut out, 0);
        chain.keep_ahead(&mut out, 0, 100, true);
        // By hand: the last 32 frames, then the first 100.
        let mut d = Distortion::new(c);
        let mut expect = source.clone();
        for f in (968..1000).chain(0..100) {
            d.frame(&mut expect[2 * f..2 * f + 2], 1, 16);
        }
        assert_eq!(out[..200], expect[..200]);
        // Asking again for less does nothing more.
        chain.keep_ahead(&mut out, 50, 10, true);
        assert_eq!(out[..200], expect[..200]);
    }
}

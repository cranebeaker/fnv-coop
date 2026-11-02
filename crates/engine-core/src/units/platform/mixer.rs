//! The platform's sound mixer: what DirectSound sends to the device for the game's buffers, as
//! measured on the player's machine (`scripts/dsmix-measure`: DirectSound 8 played known buffers
//! with known settings and WASAPI loopback recorded what reached a device without audio
//! enhancements, "Virtual Desktop Audio", 48000 Hz 32-bit float stereo; 2026-10-24,
//! `build43-scratch/measure/dsmix-*`).
//!
//! Exact (every sample bit for bit):
//! - A sample becomes `x / 32768` (16-bit) or `(b - 128) * 256 / 32768` (8-bit) as `f32`; a mono
//!   buffer goes unchanged to both channels; a stereo buffer's channels to their own.
//! - Volume and pan are one `f32` multiply per channel by [`gain`] of the channel's attenuation
//!   in hundredths of a decibel: the volume on both, a negative pan on the right only, a positive
//!   pan on the left only. Volume and pan 0 multiply by exactly 1. All 429 volumes and the 25
//!   pans measured.
//!
//! - A 3D buffer's channels are multiplied by the 3D stage's two multipliers
//!   ([`Voice::spatial`], `super::ds3d`) in place of the volume and pan, one `f32` multiply each.
//! - A buffer with the distortion effect is mixed from its processed copy ([`Voice::fx`],
//!   `super::dsfx`), processed just ahead of what is read.
//!
//! - Every buffer goes through DirectSound's converter (`super::resampler`, [`Converted`]) in 10
//!   ms periods (the first one passes the samples through at the device's rate), and the volume
//!   and pan multiply what it gives (measured, `DSMIX_HOOK` captures of 2026-10-28, every
//!   sample).
//!
//! - Several buffers playing at once are added in `f32`, the buffer started last first (each
//!   one's gained sample added to the sum of those started after it): 47996 of 48000 samples
//!   for 3 and 4 voices at mixed volumes and pans, and near full scale, where play order or an
//!   `f64` sum differ (`dsmix-multi-2026-10-24`; buffers made and started in the same order, so
//!   whether "last" means made or started is not separated). The 4 other samples: voices
//!   started 2 frames into a device period lost their last 2 frames (they stop on the period
//!   grid); voices started on a period boundary, as [`render`]'s callers start them, are heard
//!   whole.

/// The amplitude factor for an attenuation of `millibels` (hundredths of a decibel, 0 down to
/// -10000): exactly 1 for 0, else `floor(65536 * 10^(d / 2000))` as `f32` times `f32(1 / 65535)`
/// with `d = min(10 * round((millibels + 10) / 10), -10)` (halves away from zero), 0 at -10000.
pub fn gain(millibels: i32) -> f32 {
    if millibels >= 0 {
        return 1.0;
    }
    if millibels <= -10000 {
        return 0.0;
    }
    let tenth = ((millibels + 10) as f64 / 10.0).round() * 10.0;
    let d = tenth.min(-10.0);
    let k = (65536.0 * 10f64.powf(d / 2000.0)).floor() as f32;
    k * (1.0f32 / 65535.0)
}

/// A sample as the mixer reads it.
pub fn sample(bytes: &[u8], bits: u16, at: usize) -> f32 {
    if bits == 16 {
        let v = i16::from_le_bytes([bytes[2 * at], bytes[2 * at + 1]]);
        f32::from(v) / 32768.0
    } else {
        ((i32::from(bytes[at]) - 128) * 256) as f32 / 32768.0
    }
}

/// The left and right gains for a buffer's volume and pan.
pub fn channel_gains(volume: i32, pan: i32) -> (f32, f32) {
    let left = volume + if pan > 0 { -pan } else { 0 };
    let right = volume + if pan < 0 { pan } else { 0 };
    (gain(left.max(-10000)), gain(right.max(-10000)))
}

/// One buffer as the mixer plays it.
#[derive(Debug, Clone)]
pub struct Voice {
    /// The buffer's samples (a copy kept up to date at each `Unlock`).
    pub data: Vec<u8>,
    pub channels: u16,
    pub bits: u16,
    pub volume: i32,
    pub pan: i32,
    pub looping: bool,
    /// The frame the next output frame reads (for a converted voice, the next frame its
    /// converter is handed); `None` when not playing. Set it with [`Voice::start`].
    pub position: Option<usize>,
    /// The frequency it is heard at and the device's rate (0 and 0: the device's rate).
    pub rate: u32,
    pub device: u32,
    /// The device has turned on DirectSound's second converter (`super::resampler::Apl`).
    pub apl: bool,
    /// The converter, made when the voice starts (`super::resampler`); `None` before and on the
    /// exact path.
    pub converter: Option<Box<Converted>>,
    /// A 3D buffer's left and right multipliers from the 3D stage (`super::ds3d::hear`), in
    /// place of the volume and pan.
    pub spatial: Option<(f32, f32)>,
    /// The distortion effect's chain, for a buffer with it: `data` is then its processed copy.
    pub fx: Option<Box<super::dsfx::Chain>>,
    /// A stream whose samples another thread decodes ([`Feed`], `super::dshow`'s music): `data`
    /// has the stream's length, and holds its samples in `fed` (frames taken from the feed);
    /// what is read elsewhere is taken first, and waited for if it is not decoded yet.
    pub feed: Option<std::sync::Arc<Feed>>,
    pub fed: std::ops::Range<usize>,
}

/// The most frames a fed voice takes from its [`Feed`] at a time (about 1.4 s at 48000 Hz).
const TAKE: usize = 1 << 16;

/// A stream's samples while another thread decodes them (`super::dshow`'s music, which Windows'
/// MP3 Decoder DMO decodes while its graph runs): room for all of its frames (`frame_bytes`
/// each), the runs of them decoded so far, and the frame a reader is waiting for. A reader that
/// needs frames not decoded yet asks for them and waits until they are ([`Feed::take`]); the
/// decoder decodes them next, so what is heard is always the stream's own samples.
#[derive(Debug, Default)]
pub struct Feed {
    frames: usize,
    frame_bytes: usize,
    state: std::sync::Mutex<Fed>,
    changed: std::sync::Condvar,
}

#[derive(Debug, Default)]
struct Fed {
    data: Vec<u8>,
    /// The runs of frames decoded, in order and apart.
    runs: Vec<std::ops::Range<usize>>,
    /// The frame to decode next: one a reader waits for, or where the stream was sent.
    wanted: Option<usize>,
    /// The decoder waits before it starts (the tests' switch, [`Feed::hold`]).
    held: bool,
    /// Nothing holds the samples any more: the decoder stops.
    let_go: bool,
    /// The decoder stopped (all of it decoded, let go, or failed).
    ended: bool,
}

impl Feed {
    /// Room for `frames` frames of `frame_bytes` bytes, none of them decoded.
    pub fn new(frames: usize, frame_bytes: usize) -> Feed {
        Feed {
            frames,
            frame_bytes,
            state: std::sync::Mutex::new(Fed {
                data: vec![0; frames * frame_bytes],
                ..Fed::default()
            }),
            changed: std::sync::Condvar::new(),
        }
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, Fed> {
        // A decoder that failed passes its panic on elsewhere; the samples stay usable.
        self.state.lock().unwrap_or_else(|e| e.into_inner())
    }

    fn wait<'a>(&self, f: std::sync::MutexGuard<'a, Fed>) -> std::sync::MutexGuard<'a, Fed> {
        self.changed.wait(f).unwrap_or_else(|e| e.into_inner())
    }

    /// The stream's frames.
    pub fn frames(&self) -> usize {
        self.frames
    }

    /// Bytes a frame (all its channels).
    pub fn frame_bytes(&self) -> usize {
        self.frame_bytes
    }

    /// Holds the decoder at its start ([`Feed::started`]) until [`Feed::start`].
    pub fn hold(&self) {
        self.lock().held = true;
    }

    /// Lets a held decoder start.
    pub fn start(&self) {
        self.lock().held = false;
        self.changed.notify_all();
    }

    /// The decoder's start: waits while it is held; false when the samples were let go.
    pub fn started(&self) -> bool {
        let mut f = self.lock();
        while f.held && !f.let_go {
            f = self.wait(f);
        }
        !f.let_go
    }

    /// Nothing holds the samples any more (the graph was let go): the decoder stops.
    pub fn let_go(&self) {
        self.lock().let_go = true;
        self.changed.notify_all();
    }

    /// Whether the samples were let go.
    pub fn is_let_go(&self) -> bool {
        self.lock().let_go
    }

    /// The decoder gives the samples of frames `at..` (`bytes`, whole frames).
    pub fn put(&self, at: usize, bytes: &[u8]) {
        let n = (bytes.len() / self.frame_bytes).min(self.frames.saturating_sub(at));
        if n == 0 {
            return;
        }
        let mut f = self.lock();
        let from = at * self.frame_bytes;
        f.data[from..from + n * self.frame_bytes].copy_from_slice(&bytes[..n * self.frame_bytes]);
        let runs = &mut f.runs;
        // The runs that touch the new one become one with it.
        let mut new = at..at + n;
        let first = runs.partition_point(|r| r.end < new.start);
        let mut last = first;
        while last < runs.len() && runs[last].start <= new.end {
            new = new.start.min(runs[last].start)..new.end.max(runs[last].end);
            last += 1;
        }
        runs.splice(first..last, [new]);
        drop(f);
        self.changed.notify_all();
    }

    /// The decoder takes the frame to decode next, if one was asked for.
    pub fn wanted(&self) -> Option<usize> {
        self.lock().wanted.take()
    }

    /// The decoder stopped: readers no longer wait.
    pub fn end(&self) {
        self.lock().ended = true;
        self.changed.notify_all();
    }

    /// The stream goes on from frame `at` (a `Run` or a seek there): the decoder decodes there
    /// next if it has not yet.
    pub fn want(&self, at: usize) {
        self.lock().wanted = Some(at);
    }

    /// The runs of frames decoded so far.
    pub fn runs(&self) -> Vec<std::ops::Range<usize>> {
        self.lock().runs.clone()
    }

    /// Whether frame `at` is decoded.
    pub fn decoded(&self, at: usize) -> bool {
        self.lock().runs.iter().any(|r| r.contains(&at))
    }

    /// Frames `at..` as far as they are decoded (at most `most` of them) copied to the same
    /// place in `to`, which has the stream's length; if frame `at` is not decoded yet, asked for
    /// and waited for first. Answers the frames copied: none when the decoder stopped without
    /// it (it failed; its panic goes on to the game's next call on the graph, `super::dshow`).
    pub fn take(&self, at: usize, to: &mut [u8], most: usize) -> std::ops::Range<usize> {
        if at >= self.frames {
            return at..at;
        }
        let mut f = self.lock();
        loop {
            let i = f.runs.partition_point(|r| r.end <= at);
            if let Some(run) = f.runs.get(i).filter(|r| r.start <= at) {
                let end = run.end.min(at.saturating_add(most));
                let bytes = at * self.frame_bytes..end * self.frame_bytes;
                to[bytes.clone()].copy_from_slice(&f.data[bytes]);
                return at..end;
            }
            if f.ended {
                return at..at;
            }
            f.wanted = Some(at);
            f = self.wait(f);
        }
    }
}

impl Voice {
    /// A voice of `data` that is not playing, with volume, pan and frequency at their defaults.
    pub fn new(data: Vec<u8>, channels: u16, bits: u16) -> Voice {
        Voice {
            data,
            channels,
            bits,
            volume: 0,
            pan: 0,
            looping: false,
            position: None,
            rate: 0,
            device: 0,
            apl: false,
            converter: None,
            spatial: None,
            fx: None,
            feed: None,
            fed: 0..0,
        }
    }

    /// A voice of the 16-bit stream `feed` decodes, not playing.
    pub fn fed_by(feed: std::sync::Arc<Feed>, channels: u16) -> Voice {
        let mut v = Voice::new(vec![0; feed.frames() * feed.frame_bytes()], channels, 16);
        v.feed = Some(feed);
        v
    }

    fn frames(&self) -> usize {
        self.data.len() / (usize::from(self.channels) * usize::from(self.bits) / 8)
    }

    /// Plays from frame `at`: a new converter (DirectSound connects the buffer anew).
    pub fn start(&mut self, at: usize) {
        self.position = Some(at);
        self.converter = None;
    }

    /// A playing voice goes on from frame `at` (`SetCurrentPosition`, measured with
    /// `DSMIX_SETPOS`: every capture sample for sample): what was converted already is heard
    /// first, then the converter, kept with all it holds, is handed `at` and the frames after it.
    pub fn move_to(&mut self, at: usize) {
        if let Some(c) = self.converter.as_mut() {
            // The next frame handed is `origin + base + fed`: `at`.
            c.origin = at;
            c.base = 0u64.wrapping_sub(c.converter.fed());
            c.ending = false;
        }
        self.position = Some(at);
    }

    /// Source frames per output frame.
    pub fn step(&self) -> f64 {
        if self.rate == 0 || self.device == 0 {
            1.0
        } else {
            f64::from(self.rate) / f64::from(self.device)
        }
    }

    /// Whether the voice is on the exact path: no device rate given (the tests' voices).
    pub fn exact(&self) -> bool {
        self.device == 0
    }

    /// The left and right multipliers: the 3D stage's for a 3D buffer, else the volume and pan's.
    pub fn gains(&self) -> (f32, f32) {
        self.spatial
            .unwrap_or_else(|| channel_gains(self.volume, self.pan))
    }
}

/// For a fed voice ([`Voice::feed`], its parts): makes frame `at` of `data` the stream's own,
/// taking it from the feed with those after it, and waiting for it if it is not decoded yet.
fn take_fed(
    feed: &Option<std::sync::Arc<Feed>>,
    fed: &mut std::ops::Range<usize>,
    data: &mut [u8],
    at: usize,
) {
    if let Some(feed) = feed {
        if !fed.contains(&at) {
            *fed = feed.take(at, data, TAKE);
        }
    }
}

/// Adds `voices` into `out` (interleaved stereo, `out.len() / 2` frames). A voice with no device
/// rate (the tests' voices) is on the exact path, one frame per output frame; every other goes
/// through DirectSound's converter (`super::resampler`, [`Converted`]) in 10 ms periods, the
/// first one passing the samples through at the device's rate. Each voice's channels are
/// multiplied by [`Voice::gains`] (after the converter, measured). A voice with the distortion
/// effect has its processed copy brought ahead of what this call reads first
/// ([`super::dsfx::Chain::keep_ahead`]). A non-looping voice stops at its end; a voice whose
/// converter cannot be made (another build of `RESAMPLEDMO.DLL`) is not heard. `voices` come
/// newest first (the measured order of the sum, module documentation).
///
/// A fed voice ([`Voice::feed`]) reads only frames its feed has decoded: one it reads that is
/// not decoded yet is asked for and waited for ([`Feed::take`]), the decoder going there next
/// (`super::dshow`, which decodes it bit for bit as from the stream's start). Host code: on
/// Windows the graph's renderer plays what its decoder has given (its position moves from 36 to
/// 58 ms after `Run`, `super::dshow`); the wait here, only where a stream plays from a place
/// not decoded yet, lasts while the decoder decodes that place and the few frames before it.
pub fn render(voices: &mut [&mut Voice], out: &mut [f32]) {
    for v in voices.iter_mut() {
        render_voice(v, out);
    }
}

/// One voice of [`render`] added into `out` (each voice's sum is its own: rendering the voices
/// one after another in the same order is the same mix).
pub fn render_voice(v: &mut Voice, out: &mut [f32]) {
    let Some(at) = v.position else {
        return;
    };
    if v.fx.is_some() {
        // What this call reads: its frames at the voice's step, a converter's period ahead,
        // and the filter's span.
        let period = (v.device / 100) as usize;
        let ahead = ((out.len() / 2 + 2 * period) as f64 * v.step()).ceil() as usize + 160;
        if let Some(fx) = v.fx.as_mut() {
            fx.keep_ahead(&mut v.data, at, ahead, v.looping);
        }
    }
    if v.exact() {
        render_exact(v, out);
    } else if converted(v) {
        render_converted(v, out);
    } else {
        v.position = None;
    }
}

/// A voice's converter and what it gave that is not mixed yet.
#[derive(Debug, Clone)]
pub struct Converted {
    converter: super::resampler::Converter,
    /// The frame it started from, and the frames earlier converters were handed (DirectSound
    /// replaces the first converter by the second where the buffer is).
    origin: usize,
    base: u64,
    /// The frequency it converts.
    rate: u32,
    held: std::collections::VecDeque<(f32, f32)>,
    /// The buffer's end was reached: no period after the held frames.
    ending: bool,
    /// A period's output by channel, kept from one period to the next (emptied before each,
    /// so a period allocates nothing once it has grown).
    got: Vec<Vec<f32>>,
}

/// Makes the voice's converter if it has none: the second one when the device has turned it on,
/// else the first one. False when it cannot be made.
fn converted(v: &mut Voice) -> bool {
    use super::resampler::{Apl, Converter, Dmo};
    let Some(at) = v.position else { return false };
    let (rate, device, channels, lanes) = (v.rate, v.device, usize::from(v.channels), v.lanes());
    if let Some(c) = v.converter.as_mut() {
        if v.apl && matches!(c.converter, Converter::Dmo(_)) {
            // The device turned on its second converter: a new one where the buffer is.
            c.base = c.base.wrapping_add(c.converter.fed());
            c.converter = Converter::Apl(Apl::new(rate, device, channels, lanes));
            c.rate = rate;
        }
        return true;
    }
    let converter = if v.apl {
        Converter::Apl(Apl::new(rate, device, channels, lanes))
    } else {
        match Dmo::new(rate, device, channels) {
            Some(d) => Converter::Dmo(Box::new(d)),
            None => return false,
        }
    };
    v.converter = Some(Box::new(Converted {
        converter,
        origin: at,
        base: 0,
        rate,
        held: Default::default(),
        ending: false,
        got: Vec::new(),
    }));
    true
}

impl Voice {
    /// The second converter's four-lane kernel is the 16-bit mono one (`dsound` `10066219`).
    fn lanes(&self) -> bool {
        self.channels == 1 && self.bits == 16
    }
}

fn render_converted(v: &mut Voice, out: &mut [f32]) {
    let (gl, gr) = v.gains();
    let frames = v.frames();
    let period = (v.device / 100) as usize;
    let (channels, looping) = (usize::from(v.channels), v.looping);
    let Some(mut c) = v.converter.take() else {
        return;
    };
    let mut ended = false;
    for frame in out.chunks_mut(2) {
        if c.held.is_empty() {
            if c.ending || frames == 0 || period == 0 {
                ended = true;
                break;
            }
            if c.rate != v.rate {
                c.converter.retune(v.rate, v.device);
                c.rate = v.rate;
            }
            let (bits, origin, base) = (v.bits, c.origin, c.base);
            let (data, feed, fed) = (&mut v.data, &v.feed, &mut v.fed);
            let mut input = |k: u64, f: &mut [f32]| {
                let at = (origin as u64).wrapping_add(base).wrapping_add(k);
                let at = if looping {
                    Some((at % frames as u64) as usize)
                } else {
                    (at < frames as u64).then_some(at as usize)
                };
                if let Some(at) = at {
                    take_fed(feed, fed, data, at);
                }
                for (ch, x) in f.iter_mut().enumerate() {
                    *x = at.map_or(0.0, |at| sample(data, bits, at * channels + ch));
                }
            };
            let mut got = std::mem::take(&mut c.got);
            got.resize_with(channels, Vec::new);
            for g in &mut got {
                g.clear();
            }
            c.converter.period(period, &mut input, &mut got);
            for i in 0..period {
                let l = got[0][i];
                c.held
                    .push_back((l, if channels == 2 { got[1][i] } else { l }));
            }
            c.got = got;
            let fed = c.base.wrapping_add(c.converter.fed());
            let next = (origin as u64).wrapping_add(fed);
            if !looping && next >= frames as u64 {
                c.ending = true;
            }
            v.position = Some(if looping {
                (next % frames as u64) as usize
            } else {
                next.min(frames as u64) as usize
            });
        }
        let (l, r) = c.held.pop_front().expect("a converted frame");
        frame[0] += if gl == 1.0 { l } else { l * gl };
        frame[1] += if gr == 1.0 { r } else { r * gr };
    }
    if ended || (c.ending && c.held.is_empty()) {
        v.position = None;
    } else {
        v.converter = Some(c);
    }
}

fn render_exact(v: &mut Voice, out: &mut [f32]) {
    let Some(mut at) = v.position else { return };
    let (gl, gr) = v.gains();
    let frames = v.frames();
    for frame in out.chunks_mut(2) {
        if at >= frames {
            if v.looping && frames > 0 {
                at = 0;
            } else {
                break;
            }
        }
        take_fed(&v.feed, &mut v.fed, &mut v.data, at);
        let (l, r) = if v.channels == 1 {
            let s = sample(&v.data, v.bits, at);
            (s, s)
        } else {
            (
                sample(&v.data, v.bits, 2 * at),
                sample(&v.data, v.bits, 2 * at + 1),
            )
        };
        frame[0] += if gl == 1.0 { l } else { l * gl };
        frame[1] += if gr == 1.0 { r } else { r * gr };
        at += 1;
    }
    v.position = match (at >= frames, v.looping) {
        (false, _) => Some(at),
        (true, true) => Some(0),
        (true, false) => None,
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Measured: the attenuation asked for (millibels) and the gain DirectSound applied (its
    /// `f32` bits), from `dsmix-measure`'s volume sweeps.
    const MEASURED: &[(i32, u32)] = &[
        (-9000, 0x38000080),
        (-8000, 0x38c000c0),
        (-6000, 0x3a840084),
        (-5000, 0x3b5100d1),
        (-3000, 0x3d030083),
        (-2997, 0x3d030083),
        (-2990, 0x3d048084),
        (-2983, 0x3d061086),
        (-2976, 0x3d061086),
        (-2969, 0x3d07a088),
        (-2962, 0x3d093089),
        (-2955, 0x3d093089),
        (-2948, 0x3d0ac08b),
        (-2941, 0x3d0c608c),
        (-2934, 0x3d0e008e),
        (-2927, 0x3d0e008e),
        (-2920, 0x3d0fa090),
        (-2913, 0x3d115091),
        (-2906, 0x3d115091),
        (-2899, 0x3d130093),
        (-2892, 0x3d14b095),
        (-2885, 0x3d14b095),
        (-2878, 0x3d167096),
        (-2871, 0x3d182098),
        (-2864, 0x3d19f09a),
        (-2857, 0x3d19f09a),
        (-2850, 0x3d1bb09c),
        (-2843, 0x3d1d809e),
        (-2836, 0x3d1d809e),
        (-2829, 0x3d1f509f),
        (-2822, 0x3d2130a1),
        (-2815, 0x3d2130a1),
        (-2808, 0x3d2310a3),
        (-2801, 0x3d24f0a5),
        (-2794, 0x3d26d0a7),
        (-2787, 0x3d26d0a7),
        (-2780, 0x3d28c0a9),
        (-2773, 0x3d2ab0ab),
        (-2766, 0x3d2ab0ab),
        (-2759, 0x3d2cb0ad),
        (-2752, 0x3d2eb0af),
        (-2745, 0x3d2eb0af),
        (-2738, 0x3d30c0b1),
        (-2731, 0x3d32c0b3),
        (-2724, 0x3d34d0b5),
        (-2717, 0x3d34d0b5),
        (-2710, 0x3d36f0b7),
        (-2703, 0x3d3910b9),
        (-2696, 0x3d3910b9),
        (-2689, 0x3d3b30bb),
        (-2682, 0x3d3d60bd),
        (-2675, 0x3d3d60bd),
        (-2668, 0x3d3f90c0),
        (-2661, 0x3d41c0c2),
        (-2654, 0x3d4400c4),
        (-2647, 0x3d4400c4),
        (-2640, 0x3d4650c6),
        (-2633, 0x3d4890c9),
        (-2626, 0x3d4890c9),
        (-2619, 0x3d4ae0cb),
        (-2612, 0x3d4d40cd),
        (-2605, 0x3d4d40cd),
        (-2598, 0x3d4fa0d0),
        (-2591, 0x3d5210d2),
        (-2584, 0x3d5480d4),
        (-2577, 0x3d5480d4),
        (-2570, 0x3d56f0d7),
        (-2563, 0x3d5970d9),
        (-2556, 0x3d5970d9),
        (-2549, 0x3d5bf0dc),
        (-2542, 0x3d5e80de),
        (-2535, 0x3d5e80de),
        (-2528, 0x3d6110e1),
        (-2521, 0x3d63b0e4),
        (-2514, 0x3d6650e6),
        (-2507, 0x3d6650e6),
        (-2500, 0x3d6900e9),
        (-2493, 0x3d6bb0ec),
        (-2486, 0x3d6bb0ec),
        (-2479, 0x3d6e60ee),
        (-2472, 0x3d7130f1),
        (-2465, 0x3d7130f1),
        (-2458, 0x3d73f0f4),
        (-2451, 0x3d76c0f7),
        (-2444, 0x3d79a0fa),
        (-2437, 0x3d79a0fa),
        (-2430, 0x3d7c80fc),
        (-2423, 0x3d7f70ff),
        (-2416, 0x3d7f70ff),
        (-2409, 0x3d813881),
        (-2402, 0x3d82b083),
        (-2395, 0x3d82b083),
        (-2388, 0x3d843884),
        (-2381, 0x3d85c086),
        (-2374, 0x3d874887),
        (-2367, 0x3d874887),
        (-2360, 0x3d88e089),
        (-2353, 0x3d8a708a),
        (-2346, 0x3d8a708a),
        (-2339, 0x3d8c108c),
        (-2332, 0x3d8da88e),
        (-2325, 0x3d8da88e),
        (-2318, 0x3d8f508f),
        (-2311, 0x3d90f891),
        (-2304, 0x3d92a893),
        (-2297, 0x3d92a893),
        (-2290, 0x3d945894),
        (-2283, 0x3d961096),
        (-2276, 0x3d961096),
        (-2269, 0x3d97d098),
        (-2262, 0x3d99909a),
        (-2255, 0x3d99909a),
        (-2248, 0x3d9b589b),
        (-2241, 0x3d9d209d),
        (-2234, 0x3d9ef89f),
        (-2227, 0x3d9ef89f),
        (-2220, 0x3da0d0a1),
        (-2213, 0x3da2a8a3),
        (-2206, 0x3da2a8a3),
        (-2199, 0x3da488a5),
        (-2192, 0x3da670a6),
        (-2185, 0x3da670a6),
        (-2178, 0x3da860a8),
        (-2171, 0x3daa58aa),
        (-2164, 0x3dac50ac),
        (-2157, 0x3dac50ac),
        (-2150, 0x3dae50ae),
        (-2143, 0x3db050b0),
        (-2136, 0x3db050b0),
        (-2129, 0x3db258b2),
        (-2122, 0x3db470b4),
        (-2115, 0x3db470b4),
        (-2108, 0x3db680b6),
        (-2101, 0x3db8a0b9),
        (-2094, 0x3dbac0bb),
        (-2087, 0x3dbac0bb),
        (-2080, 0x3dbcf0bd),
        (-2073, 0x3dbf20bf),
        (-2066, 0x3dbf20bf),
        (-2059, 0x3dc150c1),
        (-2052, 0x3dc390c4),
        (-2045, 0x3dc390c4),
        (-2038, 0x3dc5d8c6),
        (-2031, 0x3dc820c8),
        (-2024, 0x3dca70ca),
        (-2017, 0x3dca70ca),
        (-2010, 0x3dccc8cd),
        (-2003, 0x3dcf28cf),
        (-2000, 0x3dcf28cf),
        (-1996, 0x3dcf28cf),
        (-1989, 0x3dd190d2),
        (-1982, 0x3dd3f8d4),
        (-1975, 0x3dd3f8d4),
        (-1968, 0x3dd670d6),
        (-1961, 0x3dd8e8d9),
        (-1954, 0x3ddb70db),
        (-1947, 0x3ddb70db),
        (-1940, 0x3dddf8de),
        (-1933, 0x3de088e1),
        (-1926, 0x3de088e1),
        (-1919, 0x3de328e3),
        (-1912, 0x3de5c8e6),
        (-1905, 0x3de5c8e6),
        (-1898, 0x3de870e8),
        (-1891, 0x3deb20eb),
        (-1884, 0x3dedd8ee),
        (-1877, 0x3dedd8ee),
        (-1870, 0x3df098f1),
        (-1863, 0x3df360f3),
        (-1856, 0x3df360f3),
        (-1849, 0x3df638f6),
        (-1842, 0x3df910f9),
        (-1835, 0x3df910f9),
        (-1828, 0x3dfbf0fc),
        (-1821, 0x3dfee0ff),
        (-1814, 0x3e00e881),
        (-1807, 0x3e00e881),
        (-1800, 0x3e026882),
        (-1793, 0x3e03e884),
        (-1786, 0x3e03e884),
        (-1779, 0x3e057085),
        (-1772, 0x3e06fc87),
        (-1765, 0x3e06fc87),
        (-1758, 0x3e088c89),
        (-1751, 0x3e0a208a),
        (-1744, 0x3e0bb88c),
        (-1737, 0x3e0bb88c),
        (-1730, 0x3e0d588d),
        (-1723, 0x3e0efc8f),
        (-1716, 0x3e0efc8f),
        (-1709, 0x3e10a491),
        (-1702, 0x3e125092),
        (-1695, 0x3e125092),
        (-1688, 0x3e140094),
        (-1681, 0x3e15b896),
        (-1674, 0x3e177497),
        (-1667, 0x3e177497),
        (-1660, 0x3e193499),
        (-1653, 0x3e1afc9b),
        (-1646, 0x3e1afc9b),
        (-1639, 0x3e1cc89d),
        (-1632, 0x3e1e989f),
        (-1625, 0x3e1e989f),
        (-1618, 0x3e206ca0),
        (-1611, 0x3e2248a2),
        (-1604, 0x3e242ca4),
        (-1597, 0x3e242ca4),
        (-1590, 0x3e2610a6),
        (-1583, 0x3e27fca8),
        (-1576, 0x3e27fca8),
        (-1569, 0x3e29f0aa),
        (-1562, 0x3e2be8ac),
        (-1555, 0x3e2be8ac),
        (-1548, 0x3e2de4ae),
        (-1541, 0x3e2fe8b0),
        (-1534, 0x3e31f0b2),
        (-1527, 0x3e31f0b2),
        (-1520, 0x3e3400b4),
        (-1513, 0x3e3618b6),
        (-1506, 0x3e3618b6),
        (-1499, 0x3e3834b8),
        (-1492, 0x3e3a54ba),
        (-1485, 0x3e3a54ba),
        (-1478, 0x3e3c7cbc),
        (-1471, 0x3e3eacbf),
        (-1464, 0x3e40e0c1),
        (-1457, 0x3e40e0c1),
        (-1450, 0x3e431cc3),
        (-1443, 0x3e4560c5),
        (-1436, 0x3e4560c5),
        (-1429, 0x3e47a8c8),
        (-1422, 0x3e49f8ca),
        (-1415, 0x3e49f8ca),
        (-1408, 0x3e4c50cc),
        (-1401, 0x3e4eaccf),
        (-1394, 0x3e5110d1),
        (-1387, 0x3e5110d1),
        (-1380, 0x3e537cd3),
        (-1373, 0x3e55f0d6),
        (-1366, 0x3e55f0d6),
        (-1359, 0x3e5868d8),
        (-1352, 0x3e5aecdb),
        (-1345, 0x3e5aecdb),
        (-1338, 0x3e5d74dd),
        (-1331, 0x3e6004e0),
        (-1324, 0x3e629ce3),
        (-1317, 0x3e629ce3),
        (-1310, 0x3e653ce5),
        (-1303, 0x3e67e4e8),
        (-1296, 0x3e67e4e8),
        (-1289, 0x3e6a94eb),
        (-1282, 0x3e6d4ced),
        (-1275, 0x3e6d4ced),
        (-1268, 0x3e700cf0),
        (-1261, 0x3e72d4f3),
        (-1254, 0x3e75a0f6),
        (-1247, 0x3e75a0f6),
        (-1240, 0x3e787cf8),
        (-1234, 0x3e7b5cfb),
        (-1233, 0x3e7b5cfb),
        (-1226, 0x3e7b5cfb),
        (-1219, 0x3e7e44fe),
        (-1212, 0x3e809a81),
        (-1205, 0x3e809a81),
        (-1198, 0x3e821882),
        (-1191, 0x3e839a84),
        (-1184, 0x3e852085),
        (-1177, 0x3e852085),
        (-1170, 0x3e86aa87),
        (-1163, 0x3e883a88),
        (-1156, 0x3e883a88),
        (-1149, 0x3e89ce8a),
        (-1142, 0x3e8b668b),
        (-1135, 0x3e8b668b),
        (-1128, 0x3e8d048d),
        (-1121, 0x3e8ea68f),
        (-1114, 0x3e904c90),
        (-1107, 0x3e904c90),
        (-1100, 0x3e91f892),
        (-1093, 0x3e93a894),
        (-1086, 0x3e93a894),
        (-1079, 0x3e955e95),
        (-1072, 0x3e971a97),
        (-1065, 0x3e971a97),
        (-1058, 0x3e98da99),
        (-1051, 0x3e9a9e9b),
        (-1044, 0x3e9c689c),
        (-1037, 0x3e9c689c),
        (-1030, 0x3e9e389e),
        (-1023, 0x3ea00ea0),
        (-1016, 0x3ea00ea0),
        (-1009, 0x3ea1e8a2),
        (-1002, 0x3ea3c8a4),
        (-1000, 0x3ea3c8a4),
        (-995, 0x3ea3c8a4),
        (-988, 0x3ea5aea6),
        (-981, 0x3ea798a8),
        (-974, 0x3ea98aaa),
        (-967, 0x3ea98aaa),
        (-960, 0x3eab80ac),
        (-953, 0x3ead7cad),
        (-946, 0x3ead7cad),
        (-939, 0x3eaf7eaf),
        (-932, 0x3eb186b2),
        (-925, 0x3eb186b2),
        (-918, 0x3eb394b4),
        (-911, 0x3eb5aab6),
        (-904, 0x3eb7c4b8),
        (-897, 0x3eb7c4b8),
        (-890, 0x3eb9e4ba),
        (-883, 0x3ebc0cbc),
        (-876, 0x3ebc0cbc),
        (-869, 0x3ebe38be),
        (-862, 0x3ec06cc0),
        (-855, 0x3ec06cc0),
        (-848, 0x3ec2a8c3),
        (-841, 0x3ec4e8c5),
        (-834, 0x3ec730c7),
        (-827, 0x3ec730c7),
        (-820, 0x3ec97ec9),
        (-813, 0x3ecbd4cc),
        (-806, 0x3ecbd4cc),
        (-799, 0x3ece30ce),
        (-792, 0x3ed094d1),
        (-785, 0x3ed094d1),
        (-778, 0x3ed2fed3),
        (-771, 0x3ed56ed5),
        (-764, 0x3ed7e8d8),
        (-757, 0x3ed7e8d8),
        (-750, 0x3eda68da),
        (-743, 0x3edcf0dd),
        (-736, 0x3edcf0dd),
        (-729, 0x3edf7edf),
        (-722, 0x3ee214e2),
        (-715, 0x3ee214e2),
        (-708, 0x3ee4b2e5),
        (-701, 0x3ee758e7),
        (-694, 0x3eea06ea),
        (-687, 0x3eea06ea),
        (-680, 0x3eecbced),
        (-673, 0x3eef7aef),
        (-666, 0x3eef7aef),
        (-659, 0x3ef240f2),
        (-652, 0x3ef50ef5),
        (-645, 0x3ef50ef5),
        (-638, 0x3ef7e4f8),
        (-631, 0x3efac4fb),
        (-624, 0x3efdaafe),
        (-617, 0x3efdaafe),
        (-610, 0x3f004d80),
        (-603, 0x3f01ca82),
        (-600, 0x3f01ca82),
        (-596, 0x3f01ca82),
        (-589, 0x3f034a83),
        (-582, 0x3f04d085),
        (-575, 0x3f04d085),
        (-568, 0x3f065986),
        (-561, 0x3f07e888),
        (-555, 0x3f07e888),
        (-554, 0x3f097a89),
        (-547, 0x3f097a89),
        (-540, 0x3f0b128b),
        (-533, 0x3f0cae8d),
        (-526, 0x3f0cae8d),
        (-519, 0x3f0e4f8e),
        (-512, 0x3f0ff590),
        (-505, 0x3f0ff590),
        (-498, 0x3f11a092),
        (-491, 0x3f135093),
        (-484, 0x3f150495),
        (-477, 0x3f150495),
        (-470, 0x3f16be97),
        (-463, 0x3f187d98),
        (-456, 0x3f187d98),
        (-449, 0x3f1a419a),
        (-442, 0x3f1c0a9c),
        (-435, 0x3f1c0a9c),
        (-428, 0x3f1dd99e),
        (-421, 0x3f1fada0),
        (-414, 0x3f2186a2),
        (-407, 0x3f2186a2),
        (-400, 0x3f2365a3),
        (-393, 0x3f2549a5),
        (-386, 0x3f2549a5),
        (-379, 0x3f2733a7),
        (-372, 0x3f2923a9),
        (-365, 0x3f2923a9),
        (-358, 0x3f2b18ab),
        (-351, 0x3f2d13ad),
        (-344, 0x3f2f14af),
        (-337, 0x3f2f14af),
        (-330, 0x3f311bb1),
        (-323, 0x3f3328b3),
        (-316, 0x3f3328b3),
        (-309, 0x3f353bb5),
        (-302, 0x3f3755b7),
        (-295, 0x3f3755b7),
        (-288, 0x3f3974b9),
        (-281, 0x3f3b9abc),
        (-274, 0x3f3dc6be),
        (-267, 0x3f3dc6be),
        (-260, 0x3f3ff9c0),
        (-253, 0x3f4232c2),
        (-246, 0x3f4232c2),
        (-239, 0x3f4471c4),
        (-232, 0x3f46b8c7),
        (-225, 0x3f46b8c7),
        (-218, 0x3f4905c9),
        (-211, 0x3f4b59cb),
        (-204, 0x3f4db3ce),
        (-200, 0x3f4db3ce),
        (-199, 0x3f4db3ce),
        (-197, 0x3f4db3ce),
        (-190, 0x3f5015d0),
        (-183, 0x3f527ed2),
        (-176, 0x3f527ed2),
        (-169, 0x3f54eed5),
        (-162, 0x3f5765d7),
        (-155, 0x3f5765d7),
        (-150, 0x3f59e4da),
        (-148, 0x3f59e4da),
        (-141, 0x3f5c6adc),
        (-134, 0x3f5ef7df),
        (-127, 0x3f5ef7df),
        (-120, 0x3f618ce2),
        (-113, 0x3f6429e4),
        (-106, 0x3f6429e4),
        (-101, 0x3f66cde7),
        (-100, 0x3f66cde7),
        (-99, 0x3f66cde7),
        (-92, 0x3f6979e9),
        (-85, 0x3f6979e9),
        (-78, 0x3f6c2dec),
        (-71, 0x3f6ee9ef),
        (-64, 0x3f71adf2),
        (-57, 0x3f71adf2),
        (-51, 0x3f747af4),
        (-50, 0x3f747af4),
        (-49, 0x3f747af4),
        (-43, 0x3f774ff7),
        (-36, 0x3f774ff7),
        (-29, 0x3f7a2cfa),
        (-22, 0x3f7d11fd),
        (-15, 0x3f7d11fd),
        (-11, 0x3f7d11fd),
        (-10, 0x3f7d11fd),
        (-9, 0x3f7d11fd),
        (-8, 0x3f7d11fd),
        (-5, 0x3f7d11fd),
        (-1, 0x3f7d11fd),
    ];

    #[test]
    fn the_gain_law_gives_the_measured_factors() {
        for &(mb, bits) in MEASURED {
            assert_eq!(gain(mb).to_bits(), bits, "{mb} mB");
        }
        assert_eq!(gain(0), 1.0);
        assert_eq!(gain(-10000), 0.0);
    }

    #[test]
    fn pan_attenuates_the_far_channel_only() {
        let (l, r) = channel_gains(0, -1000);
        assert_eq!((l, r.to_bits()), (1.0, 0x3ea3c8a4));
        let (l, r) = channel_gains(-300, -700);
        assert_eq!(
            (l.to_bits(), r.to_bits()),
            (gain(-300).to_bits(), 0x3ea3c8a4)
        );
    }

    #[test]
    fn samples_and_voices_mix_as_measured() {
        let data: Vec<u8> = [1000i16, -1000, 32767, -32768]
            .iter()
            .flat_map(|v| v.to_le_bytes())
            .collect();
        let mut a = Voice::new(data, 1, 16);
        a.volume = -600;
        a.start(0);
        let mut b = Voice::new(vec![128, 255], 1, 8);
        b.looping = true;
        b.start(0);
        let mut out = vec![0f32; 2 * 6];
        render(&mut [&mut a, &mut b], &mut out);
        let g = gain(-600);
        let x = |v: f32| v / 32768.0;
        assert_eq!(out[0], x(1000.0) * g + 0.0);
        assert_eq!(out[2], x(-1000.0) * g + x(127.0 * 256.0));
        assert_eq!(out[1], out[0]);
        // The 16-bit voice ended after 4 frames; the looping one goes round.
        assert_eq!(out[8], x(0.0) + 0.0);
        assert_eq!(out[10], x(127.0 * 256.0));
        assert_eq!(a.position, None);
        assert_eq!(b.position, Some(0));
    }

    /// A resampled voice keeps a steady level, moves on by its step and ends at its end.
    #[test]
    fn resampled_voices_keep_their_level_and_end() {
        if super::super::resampler::tables().is_none() {
            return; // no RESAMPLEDMO.DLL on this machine
        }
        let data: Vec<u8> = std::iter::repeat_n(8192i16, 441)
            .flat_map(|v| v.to_le_bytes())
            .collect();
        let mut v = Voice::new(data, 1, 16);
        v.start(0);
        v.rate = 44100;
        v.device = 48000;
        let mut out = vec![0f32; 2 * 600];
        render(&mut [&mut v], &mut out);
        // In the middle the kernel sums to about 1 (0.25 in, about 0.25 out).
        assert!((out[2 * 200] - 0.25).abs() < 0.01, "{}", out[2 * 200]);
        assert_eq!(out[2 * 200], out[2 * 200 + 1]);
        // 441 frames at 0.91875 a frame: 480 output frames, then silence.
        assert_eq!(v.position, None);
        assert_eq!(out[2 * 500], 0.0);
    }

    /// A feed keeps the runs of frames decoded apart and in order, joining those that touch;
    /// a reader takes from a run as much as it holds (or as it asks); a frame never decoded,
    /// once the decoder stopped, is none.
    #[test]
    fn a_feed_keeps_the_runs_decoded() {
        let feed = Feed::new(100, 2);
        let bytes: Vec<u8> = (0..=255).cycle().take(200).collect();
        feed.put(40, &bytes[80..120]);
        feed.put(0, &bytes[..20]);
        feed.put(60, &bytes[120..140]);
        feed.put(95, &bytes[190..200]);
        feed.put(99, &bytes[198..200]);
        assert_eq!(feed.runs(), [0..10, 40..70, 95..100]);
        assert!(feed.decoded(69) && !feed.decoded(70));
        let mut to = vec![0u8; 200];
        assert_eq!(feed.take(45, &mut to, 1000), 45..70);
        assert_eq!(to[90..140], bytes[90..140]);
        assert!(to[..90].iter().chain(&to[140..]).all(|&b| b == 0));
        assert_eq!(feed.take(45, &mut to, 10), 45..55);
        feed.put(10, &bytes[20..80]);
        assert_eq!(feed.runs(), [0..70, 95..100]);
        feed.end();
        assert_eq!(feed.take(80, &mut to, 10), 80..80, "the decoder stopped");
        assert_eq!(feed.take(100, &mut to, 10), 100..100, "past the end");
        assert_eq!(feed.wanted(), None);
    }

    /// A fed voice plays the stream's own samples bit for bit, the same as a voice of all of
    /// them, on the exact path and through a converter: where it reads frames not decoded yet
    /// it asks for them and waits until they are. Here the decoder (on a thread of its own)
    /// gives only what it is asked for, so every frame from where the voice starts is waited
    /// for.
    #[test]
    fn a_fed_voice_waits_for_the_samples_it_reads() {
        let all = signal(22050);
        for (rate, device) in [(0, 0), (44100, 48000)] {
            let mut plain = Voice::new(all.clone(), 1, 16);
            (plain.rate, plain.device, plain.apl) = (rate, device, true);
            plain.start(5000);
            let feed = std::sync::Arc::new(Feed::new(22050, 2));
            feed.put(0, &all[..2 * 3000]);
            let mut fed = Voice::fed_by(feed.clone(), 1);
            (fed.rate, fed.device, fed.apl) = (rate, device, true);
            fed.start(5000);
            let decoder = {
                let (feed, all) = (feed.clone(), all.clone());
                std::thread::spawn(move || {
                    let mut asked = 0;
                    while !feed.is_let_go() {
                        match feed.wanted() {
                            Some(at) if !feed.decoded(at) => {
                                let end = (at + 1152).min(22050);
                                feed.put(at, &all[2 * at..2 * end]);
                                asked += 1;
                            }
                            _ => std::thread::yield_now(),
                        }
                    }
                    asked
                })
            };
            let mut a = vec![0f32; 2 * 24480];
            let mut b = vec![0f32; 2 * 24480];
            for (pa, pb) in a.chunks_mut(2 * 1000).zip(b.chunks_mut(2 * 1000)) {
                render(&mut [&mut plain], pa);
                render(&mut [&mut fed], pb);
            }
            feed.let_go();
            let asked = decoder.join().expect("the decoder");
            assert_eq!(
                asked,
                (22050 - 5000 + 1151) / 1152,
                "frames 5000.. asked for"
            );
            let bits = |v: &[f32]| v.iter().map(|s| s.to_bits()).collect::<Vec<_>>();
            assert!(bits(&a) == bits(&b), "{rate} on {device}");
            assert!(a.iter().any(|&s| s != 0.0));
            assert_eq!((plain.position, fed.position), (None, None));
        }
    }

    /// `scripts/dsmix-measure`'s test signal: `frames` 16-bit mono frames.
    fn signal(frames: usize) -> Vec<u8> {
        let mut x = 0x1234_5678u32;
        (0..frames)
            .flat_map(|_| {
                x ^= x << 13;
                x ^= x >> 17;
                x ^= x << 5;
                (((x % 2001) as i32 - 1000) as i16).to_le_bytes()
            })
            .collect()
    }

    /// Half a second at 44100 Hz on 48000, rendered in pieces that are not periods: `f32` bits
    /// of the left channel at some frames, against what `dsound.dll` handed the audio engine
    /// (`DSMIX_HOOK` captures, 2026-10-28), and where the voice ends.
    fn converted(mut v: Voice, expect: &[(usize, u32)]) {
        let mut out = vec![0f32; 2 * 24480];
        for piece in out.chunks_mut(2 * 1000) {
            let mut voices = [&mut v];
            render(&mut voices, piece);
        }
        for &(at, bits) in expect {
            assert_eq!(out[2 * at].to_bits(), bits, "frame {at}");
            assert_eq!(out[2 * at + 1], out[2 * at]);
        }
        // Its 22050 frames were all handed over in the period that ends at 24000.
        assert_eq!(v.position, None);
        assert!(out[2 * 24000..].iter().all(|&s| s == 0.0));
    }

    /// FNV-1a of `f32` bits.
    fn bits_hash(hash: &mut u64, samples: &[f32]) {
        for s in samples {
            for b in s.to_bits().to_le_bytes() {
                *hash = (*hash ^ u64::from(b)).wrapping_mul(0x0100_0000_01b3);
            }
        }
    }

    /// Voices on every converter path (the first converter's table and curve kernels and its
    /// pass-through, the second converter's lanes and plain kernel), mono and stereo, 8 and
    /// 16 bits, looping and ending, retuned while playing, mixed newest first in pieces that
    /// are not periods.
    fn every_path() -> Vec<Voice> {
        let stereo = |frames: usize| -> Vec<u8> {
            let s = signal(2 * frames);
            s.chunks(4).flat_map(|c| [c[0], c[1], c[3], c[2]]).collect()
        };
        let mut voices = Vec::new();
        for (data, channels, bits, rate, apl, looping, volume, pan) in [
            (signal(22050), 1u16, 16u16, 44100u32, false, true, -600, 0),
            (stereo(11000), 2, 16, 22071, false, true, 0, 2500),
            (
                signal(4800)[..4800].to_vec(),
                1,
                8,
                48000,
                false,
                true,
                -100,
                -900,
            ),
            (signal(30000), 1, 16, 32000, false, false, 0, 0),
            (signal(22050), 1, 16, 22050, true, true, -300, 0),
            (stereo(20000), 2, 16, 44100, true, true, 0, -4000),
            (signal(9000), 1, 16, 11025, true, false, 0, 0),
        ] {
            let mut v = Voice::new(data, channels, bits);
            (v.rate, v.device, v.apl, v.looping) = (rate, 48000, apl, looping);
            (v.volume, v.pan) = (volume, pan);
            v.start(0);
            voices.push(v);
        }
        voices
    }

    /// The bits of 3 s of [`every_path`] mixed in pieces of 700 frames, retuning two voices
    /// halfway (one of each converter): one hash.
    fn mix_hash() -> u64 {
        let mut voices = every_path();
        let mut hash = 0xcbf2_9ce4_8422_2325u64;
        let mut out = vec![0f32; 2 * 700];
        for piece in 0..206 {
            if piece == 103 {
                voices[1].rate = 22100;
                voices[4].rate = 23000;
            }
            out.fill(0.0);
            let mut refs: Vec<&mut Voice> = voices.iter_mut().collect();
            render(&mut refs, &mut out);
            bits_hash(&mut hash, &out);
        }
        for v in &voices {
            hash = hash
                .wrapping_mul(31)
                .wrapping_add(v.position.map_or(1, |p| p as u64 + 2));
        }
        hash
    }

    /// The mix's bits are those of the build before the allocation work (captured there with this
    /// machine's `RESAMPLEDMO.DLL`): `cargo test --release -p engine-core --lib mix_bits --
    /// --ignored --nocapture`.
    #[test]
    #[ignore]
    fn mix_bits_are_unchanged() {
        if super::super::resampler::tables().is_none() {
            return; // no RESAMPLEDMO.DLL on this machine
        }
        let hash = mix_hash();
        println!("mix hash {hash:016x}");
        assert_eq!(hash, 0xf434_97bb_b7ed_dc91);
    }

    /// Not a check: the mixer's host heap allocations per 10 ms period of [`every_path`] (the
    /// output's thread), for the frame-rate work (`cargo test --release -p engine-core --lib
    /// allocations_per -- --ignored --nocapture`).
    #[test]
    #[ignore]
    fn allocations_per_operation_of_the_mixer() {
        use crate::alloc_count::{per_run, row};
        if super::super::resampler::tables().is_none() {
            return; // no RESAMPLEDMO.DLL on this machine
        }
        let mut voices = every_path();
        for v in &mut voices {
            v.looping = true;
        }
        let mut out = vec![0f32; 2 * 480];
        for (what, keep) in [
            ("first converter, table kernel (44100 on 48000)", 0),
            ("first converter, curve kernel (22071 stereo on 48000)", 1),
            ("first converter, pass-through (48000 on 48000)", 2),
            ("second converter, lanes (22050 mono on 48000)", 4),
            ("second converter, plain (44100 stereo on 48000)", 5),
        ] {
            let v = &mut voices[keep];
            row(
                &format!("mixer: 10 ms of one voice, {what}"),
                per_run(100, || {
                    out.fill(0.0);
                    render(&mut [&mut *v], &mut out);
                }),
            );
        }
    }

    #[test]
    fn the_first_converter_is_heard_as_measured() {
        if super::super::resampler::tables().is_none() {
            return; // no RESAMPLEDMO.DLL on this machine
        }
        let mut v = Voice::new(signal(22050), 1, 16);
        v.volume = -600;
        (v.rate, v.device) = (44100, 48000);
        v.start(0);
        let expect = [
            (0, 0x3c6d_22bc),
            (1, 0x3c39_b584),
            (2, 0xbb8f_b7a1),
            (3, 0xbc04_a7f8),
            (20000, 0xbb89_5d03),
            (20001, 0xb87c_8635),
        ];
        converted(v, &expect);
    }

    #[test]
    fn the_second_converter_is_heard_as_measured() {
        let mut v = Voice::new(signal(22050), 1, 16);
        (v.rate, v.device, v.apl) = (44100, 48000, true);
        v.start(0);
        let expect = [
            (0, 0),
            (1, 0xba35_cf0b),
            (2, 0x3cce_4849),
            (3, 0x3cc7_92f4),
            (20000, 0xbc0b_a97f),
            (20001, 0xbaba_986d),
        ];
        converted(v, &expect);
    }

    /// Every `DSMIX_SRC` capture (`scripts/dsmix-measure` with `DSMIX_HOOK`, the folder in
    /// `NV_DSMIX_SRC`) against the mixer, sample for sample until the voice ends and the silence
    /// after it: `cargo test -p engine-core --lib -- --ignored dsmix_src`. The cases after the
    /// first large frequency change are on the second converter; the changes while playing (their
    /// period is the capture's timing) are left out.
    #[test]
    #[ignore]
    fn dsmix_src_captures() {
        let Ok(dir) = std::env::var("NV_DSMIX_SRC") else {
            return;
        };
        let dir = std::path::PathBuf::from(dir);
        let cases = std::fs::read_to_string(dir.join("cases.tsv")).unwrap();
        let (mut apl, mut checked) = (false, 0);
        for line in cases.lines().filter(|l| l.starts_with("src_")) {
            let mut fields = line.split('\t');
            let name = fields.next().unwrap();
            let w: Vec<&str> = fields.next().unwrap().split(' ').collect();
            let after = |k: &str| w[w.iter().position(|&x| x == k).unwrap() + 1];
            let (ch, rate, bits): (u16, u32, u16) = (
                w[0].parse().unwrap(),
                w[1].parse().unwrap(),
                w[2].parse().unwrap(),
            );
            let freq = match after("freq").parse::<u32>().unwrap() {
                0 => rate,
                f => f,
            };
            let frames = after("length")
                .strip_prefix("Some(")
                .map_or(rate as usize / 2, |s| {
                    s.trim_end_matches(')').parse().unwrap()
                });
            if freq.abs_diff(rate) > rate / 500 {
                apl = true;
            }
            if name.contains("swap") {
                apl = true; // another buffer's large change while it played
            }
            if name.contains("later") || name.contains("swap") {
                continue;
            }
            let mut x = 0x1234_5678u32;
            let data: Vec<u8> = (0..frames * usize::from(ch))
                .flat_map(|_| {
                    x ^= x << 13;
                    x ^= x >> 17;
                    x ^= x << 5;
                    let s = (x % 2001) as i32 - 1000;
                    if bits == 16 {
                        (s as i16).to_le_bytes().to_vec()
                    } else {
                        vec![(128 + s / 250) as u8]
                    }
                })
                .collect();
            let mut v = Voice::new(data, ch, bits);
            v.volume = after("vol").parse().unwrap();
            v.pan = after("pan").parse().unwrap();
            (v.rate, v.device, v.apl) = (freq, 48000, apl);
            v.start(0);
            let cap: Vec<f32> = std::fs::read(dir.join(format!("{name}.f32")))
                .unwrap()
                .chunks(4)
                .map(|c| f32::from_le_bytes([c[0], c[1], c[2], c[3]]))
                .collect();
            // It starts on a period boundary (a run's first case can start a period late).
            let first = cap.iter().position(|&s| s != 0.0).unwrap() / 2;
            let cap = &cap[2 * (first - first % 480)..];
            let mut out = vec![0f32; cap.len()];
            for piece in out.chunks_mut(2 * 777) {
                render(&mut [&mut v], piece);
            }
            assert_eq!(v.position, None, "{name}: still playing");
            let diff = out
                .iter()
                .zip(cap.iter())
                .position(|(a, b)| a.to_bits() != b.to_bits());
            assert_eq!(diff, None, "{name}: the first sample that differs");
            checked += 1;
        }
        eprintln!("{checked} captures match");
    }

    /// Every `DSMIX_SETPOS` capture (`scripts/dsmix-measure` with `DSMIX_HOOK`, the folder in
    /// `NV_DSMIX_SETPOS`: a buffer moved by `SetCurrentPosition` while it plays, some with the
    /// distortion) against the mixer: the voice played from its start and moved
    /// ([`Voice::move_to`]; a distortion restarted there, `super::dsfx::Chain::start`) at each
    /// period boundary in turn; the boundary that gives the capture sample for sample is named,
    /// and so is any other way of moving that would give it (`cargo test -p engine-core --lib --
    /// --ignored dsmix_setpos --nocapture`).
    #[test]
    #[ignore]
    fn dsmix_setpos_captures() {
        use super::super::dsfx::{edge_table, Chain, Coefficients, Distortion};
        let Ok(dir) = std::env::var("NV_DSMIX_SETPOS") else {
            return;
        };
        let dir = std::path::PathBuf::from(dir);
        let cases = std::fs::read_to_string(dir.join("cases.tsv")).unwrap();
        let (mut failed, mut checked) = (Vec::new(), 0);
        for line in cases.lines().filter(|l| l.starts_with('s')) {
            let mut fields = line.split('\t');
            let name = fields.next().unwrap();
            let w: Vec<&str> = fields.next().unwrap().split(' ').collect();
            let after = |k: &str| w[w.iter().position(|&x| x == k).unwrap() + 1];
            let (ch, rate): (u16, u32) = (w[0].parse().unwrap(), w[1].parse().unwrap());
            let looping = after("looping") == "true";
            let to: usize = after("frame").parse().unwrap();
            let fx = name.contains("fx");
            let table = edge_table();
            if fx && table.is_none() {
                continue; // the player's dsdmo.dll cannot be measured here
            }
            checked += 1;
            let frames = rate as usize;
            let mut x = 0x1234_5678u32;
            let data: Vec<u8> = (0..frames * usize::from(ch))
                .flat_map(|_| {
                    x ^= x << 13;
                    x ^= x >> 17;
                    x ^= x << 5;
                    (((x % 2001) as i32 - 1000) as i16).to_le_bytes()
                })
                .collect();
            let cap: Vec<f32> = std::fs::read(dir.join(format!("{name}.f32")))
                .unwrap()
                .chunks(4)
                .map(|c| f32::from_le_bytes([c[0], c[1], c[2], c[3]]))
                .collect();
            let Some(first) = cap.iter().position(|&s| s != 0.0) else {
                failed.push(format!("{name}: silent"));
                continue;
            };
            let first = first / 2;
            let last = cap.iter().rposition(|&s| s != 0.0).unwrap() / 2 + 1;
            let cap = &cap[2 * (first - first % 480)..2 * last];
            let n = cap.len() / 2;
            let mut found = Vec::new();
            let ways: &[&str] = if fx {
                &["moved, effect restarted", "moved, effect kept"]
            } else {
                &["moved", "new converter"]
            };
            for &how in ways {
                for j in (480..n).step_by(480) {
                    let mut v = Voice::new(data.clone(), ch, 16);
                    (v.rate, v.device, v.looping) = (rate, 48000, looping);
                    if let Some(table) = table.filter(|_| fx) {
                        let p = [-18.2, 0.0, 5000.0, 1135.0, 5000.0];
                        let d = Distortion::new(Coefficients::new(rate, p, table));
                        let mut chain = Chain::new(v.data.clone(), d, ch, 16);
                        chain.start(&mut v.data, 0);
                        v.fx = Some(Box::new(chain));
                    }
                    v.start(0);
                    let mut out = vec![0f32; 2 * n];
                    for piece in out[..2 * j].chunks_mut(2 * 777) {
                        render(&mut [&mut v], piece);
                    }
                    match how {
                        "new converter" => v.start(to),
                        "moved, effect restarted" => {
                            if let Some(c) = v.fx.as_mut() {
                                c.start(&mut v.data, to);
                            }
                            v.move_to(to);
                        }
                        _ => v.move_to(to),
                    }
                    for piece in out[2 * j..].chunks_mut(2 * 777) {
                        render(&mut [&mut v], piece);
                    }
                    let diff = out
                        .iter()
                        .zip(cap.iter())
                        .position(|(a, b)| a.to_bits() != b.to_bits());
                    if diff.is_none() {
                        found.push(format!("{how} at output frame {j}"));
                    }
                }
            }
            eprintln!("{name}: {n} frames from the first period; {found:?}");
            if !found.iter().any(|f| f.starts_with(ways[0])) {
                failed.push(name.to_string());
            }
        }
        assert!(failed.is_empty(), "no match: {failed:?}");
        assert!(checked > 0, "no captures");
    }

    /// Every `DSMIX_SETPOS_ORDER` capture (`scripts/dsmix-measure` with `DSMIX_HOOK`, the folder
    /// in `NV_DSMIX_SETPOS`): three looping 48 kHz voices played A, B, C (they start together,
    /// two frames into a device period) and one of them moved to its frame 24000 while they play.
    /// The capture against the sum as the mixer makes it (each voice's sample times its gains,
    /// added in `f32` in the list's order, newest first: C, B, A), the moved voice reading from
    /// its new frame from each frame in turn before the first that differs without the move,
    /// in each order of the three; the orders that give the capture sample for sample are named
    /// (`cargo test -p engine-core --lib -- --ignored dsmix_setpos_order --nocapture`).
    #[test]
    #[ignore]
    fn dsmix_setpos_order_captures() {
        let Ok(dir) = std::env::var("NV_DSMIX_SETPOS") else {
            return;
        };
        let dir = std::path::PathBuf::from(dir);
        let cases = std::fs::read_to_string(dir.join("cases.tsv")).unwrap();
        let (mut failed, mut checked) = (Vec::new(), 0);
        for line in cases.lines().filter(|l| l.starts_with("order_")) {
            checked += 1;
            let mut fields = line.split('\t');
            let name = fields.next().unwrap();
            let numbers: Vec<i32> = fields
                .next()
                .unwrap()
                .split(|c: char| !(c.is_ascii_digit() || c == '-'))
                .filter_map(|s| s.parse().ok())
                .collect();
            // [volume, pan] for A, B, C, then the voice moved.
            let moved = numbers[6] as usize;
            let samples: Vec<Vec<f32>> = (0..3u32)
                .map(|k| {
                    let mut x = k + 1;
                    (0..48000)
                        .map(|_| {
                            x ^= x << 13;
                            x ^= x >> 17;
                            x ^= x << 5;
                            f32::from(((x % 2001) as i32 - 1000) as i16) / 32768.0
                        })
                        .collect()
                })
                .collect();
            let gains: Vec<(f32, f32)> = (0..3)
                .map(|k| channel_gains(numbers[2 * k], numbers[2 * k + 1]))
                .collect();
            let cap: Vec<f32> = std::fs::read(dir.join(format!("{name}.f32")))
                .unwrap()
                .chunks(4)
                .map(|c| f32::from_le_bytes([c[0], c[1], c[2], c[3]]))
                .collect();
            let first = cap.iter().position(|&s| s != 0.0).unwrap() / 2;
            let last = cap.iter().rposition(|&s| s != 0.0).unwrap() / 2 + 1;
            let cap = &cap[2 * first..2 * last];
            let n = cap.len() / 2;
            // The first frame that differs when the moved voice reads its frame 24000 from frame
            // `j` on (none: not moved) and the sum is in `order` from then (indices, newest first).
            let differs = |j: usize, order: [usize; 3]| {
                (0..n)
                    .find(|&i| {
                        let mut out = [0f32; 2];
                        let list = if i < j { [2, 1, 0] } else { order };
                        for k in list {
                            let at = if k == moved && i >= j {
                                24000 + i - j
                            } else {
                                i
                            };
                            let s = samples[k][at % 48000];
                            let (gl, gr) = gains[k];
                            out[0] += if gl == 1.0 { s } else { s * gl };
                            out[1] += if gr == 1.0 { s } else { s * gr };
                        }
                        out[0].to_bits() != cap[2 * i].to_bits()
                            || out[1].to_bits() != cap[2 * i + 1].to_bits()
                    })
                    .unwrap_or(n)
            };
            let d = differs(n, [2, 1, 0]);
            let mut found = Vec::new();
            for order in [
                [0, 1, 2],
                [0, 2, 1],
                [1, 0, 2],
                [1, 2, 0],
                [2, 0, 1],
                [2, 1, 0],
            ] {
                for j in d.saturating_sub(960)..=d {
                    if differs(j, order) == n {
                        let names = order.map(|k| ["A", "B", "C"][k]);
                        found.push(format!("{names:?} from frame {}", first + j));
                    }
                }
            }
            eprintln!(
                "{name}: moved {}, held unmoved to frame {} of {}; {found:?}",
                ["A", "B", "C"][moved],
                first + d,
                first + n
            );
            if found.is_empty() {
                failed.push(name.to_string());
            }
        }
        assert!(failed.is_empty(), "no match: {failed:?}");
        assert!(checked > 0, "no captures");
    }
}

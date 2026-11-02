//! The decoder: frame by frame for streaming, or a whole file at once.

use crate::error::{Error, Result};
use crate::header::{ChannelMode, FrameHeader, Version};
use crate::layer3::Layer3;
use crate::stream::{
    audio_range, parse_vbri, parse_xing, FrameWalker, LameTag, Tags, VbriHeader, XingHeader,
};

/// What LAME and every decoder modelled on the standard's lose to the
/// filterbanks at the start: 528 samples, plus one (LAME's own count).
pub const DECODER_DELAY: u32 = 529;

/// How far back in earlier frames' data a frame's own data can start: `main_data_begin`'s 9
/// bits (MPEG-1; 8 bits in MPEG-2 and 2.5).
const MAX_MAIN_DATA_BEGIN: usize = 511;

/// Something wrong with a frame. Its damaged parts play as silence; the
/// decoder carries on with the next frame.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Problem {
    /// The frame's checksum doesn't match its side information (it's
    /// decoded anyway, as most decoders do).
    Checksum,
    /// The frame's data starts in earlier frames that aren't there: the
    /// start of a cut stream. Silent.
    MissingReservoir,
    /// Inconsistent data, e.g. Huffman codes running past their length.
    Damaged(&'static str),
}

impl std::fmt::Display for Problem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Problem::Checksum => write!(f, "checksum mismatch"),
            Problem::MissingReservoir => write!(f, "needs data from frames before the stream"),
            Problem::Damaged(what) => write!(f, "damaged: {what}"),
        }
    }
}

/// What the start of a stream says about it.
#[derive(Clone, Debug)]
pub struct StreamInfo {
    pub version: Version,
    pub sample_rate: u32,
    /// Channels the decoder gives (the first frame's; frames with another
    /// count are mixed down or duplicated to match).
    pub channels: u16,
    pub mode: ChannelMode,
    /// The first audio frame's bitrate in bits per second (0: free format).
    pub bitrate: u32,
    /// Frames carry checksums.
    pub protected: bool,
    pub samples_per_frame: usize,
    pub tags: Tags,
    /// Byte offset of the first audio frame (after any Xing/VBRI frame).
    pub audio_start: usize,
    /// Byte offset where the audio ends (before end tags).
    pub audio_end: usize,
    pub xing: Option<XingHeader>,
    pub lame: Option<LameTag>,
    pub vbri: Option<VbriHeader>,
    /// Samples per channel the decoder gives in all, when the LAME tag
    /// says how many were encoded (gapless playback); `None` otherwise.
    pub gapless_samples: Option<u64>,
}

/// One decoded frame.
#[derive(Clone, Debug)]
pub struct Frame {
    /// Where the frame starts in the data.
    pub offset: usize,
    pub header: FrameHeader,
    /// Interleaved samples, nominally -1.0 to 1.0 (a loud encoding can go
    /// slightly past). Fewer than a whole frame's where gapless playback
    /// trims the encoder's delay and padding; possibly none.
    pub samples: Vec<f32>,
    pub channels: u16,
    pub sample_rate: u32,
    pub problem: Option<Problem>,
}

impl Frame {
    pub fn to_i16(&self) -> Vec<i16> {
        self.samples.iter().map(|&s| to_i16(s)).collect()
    }
}

/// A sample in -1.0..1.0 as 16-bit, rounded and clipped.
pub fn to_i16(sample: f32) -> i16 {
    (sample * 32768.0).round().clamp(-32768.0, 32767.0) as i16
}

/// Decodes an MP3 frame by frame. Works on anything holding the file's
/// bytes (`Vec<u8>`, `&[u8]`, `Arc<[u8]>`).
///
/// With a LAME tag, the encoder's delay (plus the decoder's 529 samples)
/// is cut from the start and its padding from the end, so a track loops
/// without a gap; [`Decoder::set_gapless`] turns that off.
pub struct Decoder<B: AsRef<[u8]> = Vec<u8>> {
    data: B,
    info: StreamInfo,
    walker: FrameWalker,
    start_walker: FrameWalker,
    layer3: Box<Layer3>,
    pcm: [Vec<f32>; 2],
    gapless: bool,
    /// Samples per channel still to drop at the start.
    skip: u64,
    /// Samples per channel still to give, when known.
    remaining: Option<u64>,
    pending: Vec<f32>,
    pending_pos: usize,
}

impl<B: AsRef<[u8]>> Decoder<B> {
    /// Finds the first frame and reads the stream's information.
    pub fn new(data: B) -> Result<Self> {
        let bytes = data.as_ref();
        let (start, end, tags) = audio_range(bytes);
        let mut walker = FrameWalker::find(bytes, start, end).ok_or(Error::NoFrames)?;
        let first = walker.first;
        if first.layer != 3 {
            return Err(Error::Unsupported(format!(
                "{} Layer {} audio (only Layer III, MP3, is decoded)",
                first.version.name(),
                ["I", "II", "III"][usize::from(first.layer - 1)]
            )));
        }
        // An information frame isn't audio: step over it.
        let mut start_walker = walker.clone();
        let (pos, header, len) = walker.next(bytes).ok_or(Error::NoFrames)?;
        let frame = &bytes[pos..pos + len];
        let (xing, lame) = match parse_xing(&header, frame) {
            Some((x, l)) => (Some(x), l),
            None => (None, None),
        };
        let vbri = parse_vbri(frame);
        if xing.is_some() || vbri.is_some() {
            start_walker = walker.clone();
        }
        let audio_start = start_walker.pos;
        // Gapless playback: how many samples the encoder took in.
        let gapless_samples = match (&lame, &xing) {
            (Some(lame), Some(xing)) if lame.gapless() => {
                let frames = match xing.frames {
                    Some(f) => u64::from(f),
                    None => {
                        let mut w = start_walker.clone();
                        std::iter::from_fn(|| w.next(bytes)).count() as u64
                    }
                };
                let total = frames * first.samples_per_frame() as u64;
                Some(total.saturating_sub(u64::from(lame.delay) + u64::from(lame.padding)))
            }
            _ => None,
        };
        let info = StreamInfo {
            version: first.version,
            sample_rate: first.sample_rate(),
            channels: first.channels() as u16,
            mode: first.mode,
            bitrate: if xing.is_some() || vbri.is_some() {
                start_walker
                    .clone()
                    .next(bytes)
                    .map_or(header.bitrate(), |(_, h, _)| h.bitrate())
            } else {
                header.bitrate()
            },
            protected: first.protected,
            samples_per_frame: first.samples_per_frame(),
            tags,
            audio_start,
            audio_end: end,
            xing,
            lame,
            vbri,
            gapless_samples,
        };
        let mut decoder = Decoder {
            data,
            info,
            walker: start_walker.clone(),
            start_walker,
            layer3: Layer3::new(),
            pcm: [Vec::new(), Vec::new()],
            gapless: true,
            skip: 0,
            remaining: None,
            pending: Vec::new(),
            pending_pos: 0,
        };
        decoder.rewind();
        Ok(decoder)
    }

    pub fn info(&self) -> &StreamInfo {
        &self.info
    }

    pub fn sample_rate(&self) -> u32 {
        self.info.sample_rate
    }

    pub fn channels(&self) -> u16 {
        self.info.channels
    }

    /// Whether to trim the encoder's delay and padding when the LAME tag
    /// gives them (on by default). Takes effect from the start: rewinds.
    pub fn set_gapless(&mut self, on: bool) {
        self.gapless = on;
        self.rewind();
    }

    /// Goes back to the first frame, as for looping a track.
    pub fn rewind(&mut self) {
        self.walker = self.start_walker.clone();
        self.layer3 = Layer3::new();
        self.pending.clear();
        self.pending_pos = 0;
        match (self.gapless, &self.info.lame, self.info.gapless_samples) {
            (true, Some(lame), Some(total)) => {
                self.skip = u64::from(lame.delay) + u64::from(DECODER_DELAY);
                self.remaining = Some(total);
            }
            _ => {
                self.skip = 0;
                self.remaining = None;
            }
        }
    }

    /// Bytes skipped between frames so far (junk or damage).
    pub fn skipped_bytes(&self) -> usize {
        self.walker.skipped - self.start_walker.skipped
    }

    /// The audio frames [`Decoder::next_frame`] gives from the start (a Xing/VBRI frame is not
    /// one), counted by stepping through their headers without decoding them.
    pub fn frame_count(&self) -> usize {
        let bytes = self.data.as_ref();
        let mut walker = self.start_walker.clone();
        std::iter::from_fn(|| walker.next(bytes)).count()
    }

    /// Goes to audio frame `frame` (counted as [`Decoder::frame_count`] counts them; past the
    /// last one, to the end) so that it and every frame after it decode exactly as they do from
    /// the start: the same samples bit for bit, the same problems, the same gapless trimming.
    ///
    /// What a frame decodes to depends on earlier frames in three ways, each of them bounded:
    /// its data can start up to 511 bytes back in the data of the frames before it (the bit
    /// reservoir: `main_data_begin` has 9 bits, 8 in MPEG-2 and 2.5), and each channel's IMDCT
    /// overlap and synthesis filterbank carry what that channel's last granule gave (the overlap
    /// is that granule's second half, a function of its own data alone; the filterbank's 1,024
    /// values are its last 16 time slots, all from that granule, which has 18). So the decoder
    /// starts afresh (as [`Decoder::rewind`]) a few frames back and decodes, without giving
    /// them, the frames from there: from the frame holding each channel's second-to-last
    /// granule before `frame` (the last granule's time slots add that one's overlap), and
    /// before that, frames holding at least 511 bytes of data of their own, so that every frame
    /// from there on finds in the reservoir all the data it finds there from the start.
    pub fn seek(&mut self, frame: usize) {
        self.rewind();
        let bytes = self.data.as_ref();
        // The frames before `frame`: where each starts, its samples per channel and how many
        // bytes of its own data it adds to the reservoir; and by channel, the frames that hold
        // its last two granules.
        let mut walker = self.start_walker.clone();
        let mut before: Vec<(usize, u64, usize)> = Vec::new();
        let mut granules: [[Option<usize>; 2]; 2] = [[None; 2]; 2];
        while before.len() < frame {
            let Some((pos, header, len)) = walker.next(bytes) else {
                // Past the last frame: at the end.
                self.walker = walker;
                return;
            };
            let side_end = header.side_info_start() + header.side_info_length();
            let j = before.len();
            for _ in 0..if header.is_lsf() { 1 } else { 2 } {
                for last in granules.iter_mut().take(header.channels()) {
                    *last = [last[1], Some(j)];
                }
            }
            let samples = header.samples_per_frame() as u64;
            before.push((pos, samples, len.saturating_sub(side_end)));
        }
        for &(_, samples, _) in &before {
            trim(&mut self.skip, &mut self.remaining, samples);
        }
        let needed = granules
            .iter()
            .filter_map(|last| last[0].or(last[1]))
            .min()
            .unwrap_or(frame);
        let mut from = needed;
        let mut reach = 0;
        while from > 0 && reach < MAX_MAIN_DATA_BEGIN {
            from -= 1;
            reach += before[from].2;
        }
        if from == frame {
            return;
        }
        self.walker.pos = before[from].0;
        for _ in from..frame {
            let (pos, header, len) = self.walker.next(bytes).expect("a frame walked above");
            let _ = self
                .layer3
                .decode(&header, &bytes[pos..pos + len], &mut self.pcm);
        }
    }

    /// Decodes the next frame; `None` at the end.
    pub fn next_frame(&mut self) -> Option<Frame> {
        let bytes = self.data.as_ref();
        let (pos, header, len) = self.walker.next(bytes)?;
        let problem = self
            .layer3
            .decode(&header, &bytes[pos..pos + len], &mut self.pcm);

        // Gapless trimming.
        let n = header.samples_per_frame() as u64;
        let (from, to) = trim(&mut self.skip, &mut self.remaining, n);

        let channels = usize::from(self.info.channels);
        let mut samples = Vec::with_capacity((to - from) * channels);
        let [left, right] = &self.pcm;
        match (channels, header.channels()) {
            (2, 2) => {
                for i in from..to {
                    samples.push(left[i]);
                    samples.push(right[i]);
                }
            }
            (2, _) => {
                for &v in &left[from..to] {
                    samples.push(v);
                    samples.push(v);
                }
            }
            (_, 2) => samples.extend((from..to).map(|i| 0.5 * (left[i] + right[i]))),
            _ => samples.extend_from_slice(&left[from..to]),
        }
        Some(Frame {
            offset: pos,
            header,
            samples,
            channels: self.info.channels,
            sample_rate: self.info.sample_rate,
            problem,
        })
    }

    /// Fills `out` with the next interleaved samples, decoding as needed.
    /// Returns how many were written: fewer than `out.len()` only at the end.
    pub fn read_f32(&mut self, out: &mut [f32]) -> usize {
        let mut written = 0;
        while written < out.len() {
            if self.pending_pos == self.pending.len() {
                match self.next_frame() {
                    Some(frame) => {
                        self.pending = frame.samples;
                        self.pending_pos = 0;
                        continue;
                    }
                    None => break,
                }
            }
            let n = (self.pending.len() - self.pending_pos).min(out.len() - written);
            out[written..written + n]
                .copy_from_slice(&self.pending[self.pending_pos..self.pending_pos + n]);
            self.pending_pos += n;
            written += n;
        }
        written
    }

    /// As [`Decoder::read_f32`], as 16-bit samples.
    pub fn read_i16(&mut self, out: &mut [i16]) -> usize {
        let mut buffer = [0.0f32; 1152];
        let mut written = 0;
        while written < out.len() {
            let want = (out.len() - written).min(buffer.len());
            let got = self.read_f32(&mut buffer[..want]);
            for (o, &s) in out[written..written + got].iter_mut().zip(&buffer[..got]) {
                *o = to_i16(s);
            }
            written += got;
            if got < want {
                break;
            }
        }
        written
    }
}

/// Gapless trimming of a frame of `n` samples per channel ([`Decoder`]'s `skip` and
/// `remaining`): the part of them given.
fn trim(skip: &mut u64, remaining: &mut Option<u64>, n: u64) -> (usize, usize) {
    let dropped = (*skip).min(n);
    *skip -= dropped;
    let mut keep = n - dropped;
    if let Some(remaining) = remaining {
        keep = keep.min(*remaining);
        *remaining -= keep;
    }
    (dropped as usize, (dropped + keep) as usize)
}

impl<B: AsRef<[u8]>> Iterator for Decoder<B> {
    type Item = Frame;

    fn next(&mut self) -> Option<Frame> {
        self.next_frame()
    }
}

/// A whole decoded file.
#[derive(Clone, Debug)]
pub struct Pcm {
    pub sample_rate: u32,
    pub channels: u16,
    /// Interleaved 16-bit samples.
    pub samples: Vec<i16>,
    /// Frames decoded.
    pub frames: usize,
    /// Frames with a [`Problem`], and the first one met.
    pub problem_frames: usize,
    pub first_problem: Option<Problem>,
}

/// Decodes a whole file to 16-bit samples (gapless when the file says how).
pub fn decode_all(bytes: &[u8]) -> Result<Pcm> {
    let mut decoder = Decoder::new(bytes)?;
    let mut pcm = Pcm {
        sample_rate: decoder.sample_rate(),
        channels: decoder.channels(),
        samples: Vec::new(),
        frames: 0,
        problem_frames: 0,
        first_problem: None,
    };
    for frame in &mut decoder {
        pcm.frames += 1;
        if let Some(p) = frame.problem {
            pcm.problem_frames += 1;
            pcm.first_problem.get_or_insert(p);
        }
        pcm.samples.extend(frame.samples.iter().map(|&s| to_i16(s)));
    }
    Ok(pcm)
}

//! The `.bik` file: header, audio track list and frame index.

use std::fmt;

/// Why a file could not be read as a Bink 1 movie.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// Not a Bink file, or a Bink 2 file (`KB2`).
    NotBink,
    /// A Bink 1 revision this decoder does not handle.
    Revision(u8),
    /// The file ends inside its header or frame index.
    Truncated,
    /// The frame index points outside the file or goes backwards.
    BadIndex(u32),
    /// A frame's packet is shorter than its audio sizes say.
    BadFrame(u32),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::NotBink => write!(f, "not a Bink 1 movie"),
            Error::Revision(r) => write!(f, "unsupported Bink revision '{}'", *r as char),
            Error::Truncated => write!(f, "the file ends inside its header"),
            Error::BadIndex(i) => write!(f, "frame index entry {i} is out of range"),
            Error::BadFrame(i) => write!(f, "frame {i} is shorter than its audio data"),
        }
    }
}

impl std::error::Error for Error {}

/// Video flag: the movie has an alpha plane.
pub const FLAG_ALPHA: u32 = 0x0010_0000;
/// Video flag: the movie is grey (no chroma planes).
pub const FLAG_GRAY: u32 = 0x0002_0000;

/// Audio track flag: 16-bit samples.
pub const AUDIO_16BIT: u16 = 0x4000;
/// Audio track flag: two channels.
pub const AUDIO_STEREO: u16 = 0x2000;
/// Audio track flag: the track is coded with the DCT variant (otherwise the
/// RDFT one).
pub const AUDIO_DCT: u16 = 0x1000;

/// One audio track as the header describes it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AudioTrack {
    pub id: u32,
    pub sample_rate: u16,
    pub flags: u16,
    /// The header's per-track size field (the largest decoded packet).
    pub max_size: u32,
}

impl AudioTrack {
    pub fn channels(&self) -> u16 {
        if self.flags & AUDIO_STEREO != 0 {
            2
        } else {
            1
        }
    }
}

/// A parsed `.bik` header and frame index, borrowing the file's bytes.
#[derive(Debug, Clone)]
pub struct Movie<'a> {
    data: &'a [u8],
    /// The revision letter after `BIK` (for example `b'i'`).
    pub revision: u8,
    pub width: u32,
    pub height: u32,
    /// Frame rate as a fraction: `fps_num / fps_den` frames per second.
    pub fps_num: u32,
    pub fps_den: u32,
    pub video_flags: u32,
    pub largest_frame: u32,
    pub audio: Vec<AudioTrack>,
    /// Start offset of each frame, with one extra entry for the end of the
    /// last frame.
    offsets: Vec<u32>,
    keyframes: Vec<bool>,
}

/// One frame's data: an audio packet per track (possibly empty), then the
/// video data.
#[derive(Debug, Clone)]
pub struct Packet<'a> {
    pub keyframe: bool,
    pub audio: Vec<&'a [u8]>,
    pub video: &'a [u8],
}

fn u32_at(d: &[u8], o: usize) -> Result<u32, Error> {
    d.get(o..o + 4)
        .map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
        .ok_or(Error::Truncated)
}

fn u16_at(d: &[u8], o: usize) -> Result<u16, Error> {
    d.get(o..o + 2)
        .map(|b| u16::from_le_bytes([b[0], b[1]]))
        .ok_or(Error::Truncated)
}

impl<'a> Movie<'a> {
    /// Reads the header and the frame index.
    ///
    /// Layout (all little-endian u32 unless noted): `BIK` and a revision
    /// letter; file size minus 8; frame count; largest frame; frame count
    /// again; width; height; frame rate numerator and denominator; video
    /// flags; audio track count. Then, per track, its largest decoded
    /// size; then, per track, sample rate (u16) and flags (u16); then, per
    /// track, its ID. Then the frame index: one offset per frame plus one,
    /// where bit 0 marks a keyframe.
    pub fn parse(data: &'a [u8]) -> Result<Movie<'a>, Error> {
        if data.len() < 4 || &data[0..3] != b"BIK" {
            return Err(Error::NotBink);
        }
        let revision = data[3];
        // Bink 1 revisions. Only 'i' (the game's movies) is checked against
        // the game's library.
        if !matches!(revision, b'b' | b'd' | b'f' | b'g' | b'h' | b'i' | b'k') {
            return Err(Error::Revision(revision));
        }
        let frames = u32_at(data, 8)?;
        let largest_frame = u32_at(data, 12)?;
        let width = u32_at(data, 20)?;
        let height = u32_at(data, 24)?;
        let fps_num = u32_at(data, 28)?;
        let fps_den = u32_at(data, 32)?;
        let video_flags = u32_at(data, 36)?;
        let tracks = u32_at(data, 40)? as usize;
        if tracks > 256 {
            return Err(Error::Truncated);
        }
        let mut o = 44;
        let mut audio = Vec::with_capacity(tracks);
        for t in 0..tracks {
            audio.push(AudioTrack {
                id: 0,
                sample_rate: 0,
                flags: 0,
                max_size: u32_at(data, o + 4 * t)?,
            });
        }
        o += 4 * tracks;
        for (t, track) in audio.iter_mut().enumerate() {
            track.sample_rate = u16_at(data, o + 4 * t)?;
            track.flags = u16_at(data, o + 4 * t + 2)?;
        }
        o += 4 * tracks;
        for (t, track) in audio.iter_mut().enumerate() {
            track.id = u32_at(data, o + 4 * t)?;
        }
        o += 4 * tracks;

        let count = frames as usize;
        if data.len() < o + 4 * (count + 1) {
            return Err(Error::Truncated);
        }
        let mut offsets = Vec::with_capacity(count + 1);
        let mut keyframes = Vec::with_capacity(count);
        for i in 0..=count {
            let raw = u32_at(data, o + 4 * i)?;
            if i < count {
                keyframes.push(raw & 1 != 0);
            }
            offsets.push(raw & !1);
        }
        for i in 0..count {
            let (a, b) = (offsets[i], offsets[i + 1]);
            if b < a || b as usize > data.len() {
                return Err(Error::BadIndex(i as u32));
            }
        }
        Ok(Movie {
            data,
            revision,
            width,
            height,
            fps_num,
            fps_den,
            video_flags,
            largest_frame,
            audio,
            offsets,
            keyframes,
        })
    }

    pub fn frame_count(&self) -> u32 {
        self.keyframes.len() as u32
    }

    /// Frames per second.
    pub fn fps(&self) -> f64 {
        if self.fps_den == 0 {
            0.0
        } else {
            self.fps_num as f64 / self.fps_den as f64
        }
    }

    /// Frame `i`'s audio packets and video data. Each audio packet is
    /// preceded in the file by its size in bytes; a size of 0 means the
    /// track has nothing in this frame.
    pub fn packet(&self, i: u32) -> Result<Packet<'a>, Error> {
        let i = i as usize;
        if i >= self.keyframes.len() {
            return Err(Error::BadIndex(i as u32));
        }
        let start = self.offsets[i] as usize;
        let end = self.offsets[i + 1] as usize;
        let mut o = start;
        let mut audio = Vec::with_capacity(self.audio.len());
        for _ in 0..self.audio.len() {
            let size = u32_at(self.data, o).map_err(|_| Error::BadFrame(i as u32))? as usize;
            o += 4;
            if o + size > end {
                return Err(Error::BadFrame(i as u32));
            }
            audio.push(&self.data[o..o + size]);
            o += size;
        }
        Ok(Packet {
            keyframe: self.keyframes[i],
            audio,
            video: &self.data[o..end],
        })
    }
}

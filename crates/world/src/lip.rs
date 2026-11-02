//! Lip sync files (`.lip`, beside each voice file): how a face moves while
//! a line is said, 30 frames a second. Read from the game's loader
//! (`004d5350`, decoder `004d5880`, frame parser `004d5570`) and checked on
//! all 370 of Doc Mitchell's lines.
//!
//! - A 12-byte header: 1 (a header is present), the decoded size (always
//!   16 more than the data really decodes to; not needed), flags (bit 0:
//!   compressed, set in every file). A file not starting with 1 is a
//!   compressed stream from its first byte (an older layout no New Vegas
//!   file uses).
//! - Compression: runs of zero bytes. A byte other than 0 stands for
//!   itself; a 0 is followed by a 16-bit count of zero bytes.
//! - Decoded: the frame count, the lead-in as a (zero or negative) number
//!   of frames, then per frame 16 phoneme weights and 17 modifier weights
//!   (f32, in the order of [`crate::face::PHONEMES`] and
//!   [`crate::face::MODIFIERS`]).
//!
//! Frame `k` belongs to the voice's time `(k + offset) / 30` seconds
//! (`004d50e0`), so the first `-offset` frames come before the voice
//! starts.

use crate::face::{MODIFIER_COUNT, PHONEME_COUNT};

/// Frames a second (each key lasts 1/30 s: the constant at `010209a0`).
pub const FRAMES_PER_SECOND: f32 = 30.0;

/// The most frames the game's parser accepts.
const MAX_FRAMES: u32 = 1_000_000;

/// One line's lip sync.
#[derive(Debug, Clone, PartialEq)]
pub struct Lip {
    /// Frames before the voice starts, as a zero or negative number (Doc
    /// Mitchell's: 0 to −33, mostly −3 or −4).
    pub offset: i32,
    pub frames: Vec<LipFrame>,
}

/// One frame's weights, as stored (the head turns, modifiers 14–16, not
/// yet multiplied by the INI's exaggerations).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LipFrame {
    pub phonemes: [f32; PHONEME_COUNT],
    pub modifiers: [f32; MODIFIER_COUNT],
}

const FRAME_BYTES: usize = 4 * (PHONEME_COUNT + MODIFIER_COUNT);

impl Lip {
    /// Reads a `.lip` file.
    pub fn parse(bytes: &[u8]) -> Result<Lip, String> {
        let u32_at = |b: &[u8], at: usize| -> Option<u32> {
            Some(u32::from_le_bytes(b.get(at..at + 4)?.try_into().ok()?))
        };
        let decoded = match u32_at(bytes, 0) {
            Some(1) => {
                let flags = u32_at(bytes, 8).ok_or("the header is cut short")?;
                if flags & 1 != 0 {
                    expand_zero_runs(&bytes[12..])?
                } else {
                    bytes[12..].to_vec()
                }
            }
            _ => expand_zero_runs(bytes)?,
        };
        let count = u32_at(&decoded, 0).ok_or("no frame count")?;
        if count > MAX_FRAMES {
            return Err(format!("{count} frames is more than the game reads"));
        }
        let offset = u32_at(&decoded, 4).ok_or("no lead-in")? as i32;
        let count = count as usize;
        let data = decoded
            .get(8..8 + count * FRAME_BYTES)
            .ok_or_else(|| format!("{count} frames claimed, fewer stored"))?;
        let frames = data
            .chunks_exact(FRAME_BYTES)
            .map(|frame| {
                let f = |i: usize| f32::from_le_bytes(frame[i * 4..i * 4 + 4].try_into().unwrap());
                LipFrame {
                    phonemes: std::array::from_fn(f),
                    modifiers: std::array::from_fn(|i| f(PHONEME_COUNT + i)),
                }
            })
            .collect();
        Ok(Lip { offset, frames })
    }

    /// Seconds of lead-in before the voice starts (`004d5a90`):
    /// `−offset / 30`.
    pub fn lead_in(&self) -> f32 {
        -(self.offset as f32) / FRAMES_PER_SECOND
    }

    /// How long its frames last (`N / 30`).
    pub fn length(&self) -> f32 {
        self.frames.len() as f32 / FRAMES_PER_SECOND
    }
}

/// A voice file's lip sync file: the same path with `.lip` for `.ogg`
/// (`004d5ad0`).
pub fn lip_path(voice: &str) -> Option<String> {
    let (stem, _) = voice.rsplit_once('.')?;
    Some(format!("{stem}.lip"))
}

/// Undoes the zero-run compression: bytes other than 0 as they are, a 0
/// followed by a 16-bit count as that many zero bytes.
fn expand_zero_runs(packed: &[u8]) -> Result<Vec<u8>, String> {
    let mut out = Vec::with_capacity(packed.len() * 3);
    let mut i = 0;
    while i < packed.len() {
        let b = packed[i];
        if b != 0 {
            out.push(b);
            i += 1;
            continue;
        }
        let run = packed
            .get(i + 1..i + 3)
            .ok_or("a run of zeros is cut short")?;
        out.resize(
            out.len() + usize::from(u16::from_le_bytes([run[0], run[1]])),
            0,
        );
        i += 3;
    }
    Ok(out)
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    /// Packs bytes the way the game's files are: zero runs as 0 + count.
    pub(crate) fn pack(raw: &[u8]) -> Vec<u8> {
        let mut out = Vec::new();
        let mut i = 0;
        while i < raw.len() {
            if raw[i] != 0 {
                out.push(raw[i]);
                i += 1;
                continue;
            }
            let start = i;
            while i < raw.len() && raw[i] == 0 && i - start < 0xFFFF {
                i += 1;
            }
            out.push(0);
            out.extend(((i - start) as u16).to_le_bytes());
        }
        out
    }

    /// A `.lip` file with the given lead-in and frames, compressed, with
    /// the header's size 16 too large as in every game file.
    pub(crate) fn lip_file(offset: i32, frames: &[LipFrame]) -> Vec<u8> {
        let mut raw = (frames.len() as u32).to_le_bytes().to_vec();
        raw.extend(offset.to_le_bytes());
        for f in frames {
            for v in f.phonemes.iter().chain(&f.modifiers) {
                raw.extend(v.to_le_bytes());
            }
        }
        let mut out = 1u32.to_le_bytes().to_vec();
        out.extend((raw.len() as u32 + 16).to_le_bytes());
        out.extend(1u32.to_le_bytes());
        out.extend(pack(&raw));
        out
    }

    pub(crate) fn frame(phoneme: (usize, f32), modifier: (usize, f32)) -> LipFrame {
        let mut f = LipFrame {
            phonemes: [0.0; PHONEME_COUNT],
            modifiers: [0.0; MODIFIER_COUNT],
        };
        f.phonemes[phoneme.0] = phoneme.1;
        f.modifiers[modifier.0] = modifier.1;
        f
    }

    #[test]
    fn reads_compressed_frames() {
        let frames = [
            frame((15, 0.0187), (8, 0.0005)),
            frame((5, 0.975), (0, 0.95)),
        ];
        let bytes = lip_file(-5, &frames);
        // Mostly zeros: the file is far smaller than the frames.
        assert!(bytes.len() < 8 + 2 * FRAME_BYTES / 2, "{}", bytes.len());
        let lip = Lip::parse(&bytes).unwrap();
        assert_eq!(lip.offset, -5);
        assert_eq!(lip.frames, frames);
        assert!((lip.lead_in() - 5.0 / 30.0).abs() < 1e-6);
        assert!((lip.length() - 2.0 / 30.0).abs() < 1e-6);
    }

    #[test]
    fn the_bytes_of_doc_mitchells_greeting_decode() {
        // The start of `vcg01_greeting_00107222_1.lip`: 0xA3 and three
        // zeros give 163 frames, then a lead-in of −5, then 96 zero bytes
        // and LookDown 0.000496.
        let mut stream = vec![
            0xA3, 0x00, 0x03, 0x00, 0xFB, 0xFF, 0xFF, 0xFF, 0x00, 0x60, 0x00,
        ];
        stream.extend([0x6C, 0xF7, 0x01, 0x3A]);
        let raw = expand_zero_runs(&stream).unwrap();
        assert_eq!(&raw[..4], &163u32.to_le_bytes());
        assert_eq!(&raw[4..8], &(-5i32).to_le_bytes());
        assert_eq!(raw.len(), 8 + 96 + 4);
        let look_down = f32::from_le_bytes(raw[104..108].try_into().unwrap());
        assert!((look_down - 0.000496).abs() < 1e-6, "{look_down}");
    }

    #[test]
    fn uncompressed_and_headerless_files_read_too() {
        let frames = [frame((0, 0.5), (1, 1.0))];
        let packed = lip_file(0, &frames);
        // Without the header: the compressed stream from byte 0.
        assert_eq!(Lip::parse(&packed[12..]).unwrap().frames, frames);
        // With the header but not compressed.
        let mut raw = 1u32.to_le_bytes().to_vec();
        raw.extend(0u32.to_le_bytes());
        raw.extend(0u32.to_le_bytes());
        raw.extend(expand_zero_runs(&packed[12..]).unwrap());
        assert_eq!(Lip::parse(&raw).unwrap().frames, frames);
    }

    #[test]
    fn short_files_are_refused() {
        let bytes = lip_file(-3, &[frame((0, 0.5), (1, 1.0))]);
        assert!(Lip::parse(&bytes[..bytes.len() - 3]).is_err());
    }

    #[test]
    fn the_lip_file_sits_beside_the_voice() {
        assert_eq!(
            lip_path(
                "sound\\voice\\falloutnv.esm\\maleuniquedocmitchell\\vcg01_greeting_00107222_1.ogg"
            )
            .as_deref(),
            Some(
                "sound\\voice\\falloutnv.esm\\maleuniquedocmitchell\\vcg01_greeting_00107222_1.lip"
            )
        );
    }
}

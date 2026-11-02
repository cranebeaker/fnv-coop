//! Finding the audio in a file: tags before and after it, the frames
//! themselves (skipping anything between them), and the information frame
//! some encoders put first (Xing or Info, with LAME's extension holding
//! the encoder delay and padding; or Fraunhofer's VBRI).

use crate::header::FrameHeader;

/// The tags around the frames.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Tags {
    /// Bytes of ID3v2 tags before the audio (the game's files have one
    /// each, of 256, 1,024 or about 10,300 bytes).
    pub id3v2: usize,
    /// A 128-byte ID3v1 tag ends the file.
    pub id3v1: bool,
    /// Bytes of APEv2 tag before the end (or before ID3v1).
    pub ape: usize,
    /// Bytes of Lyrics3 (version 2) tag before the end.
    pub lyrics3: usize,
}

/// Where the audio is: from after the ID3v2 tags to before the end tags.
pub(crate) fn audio_range(bytes: &[u8]) -> (usize, usize, Tags) {
    let mut tags = Tags::default();
    let mut start = 0;
    while let Some(tag) = bytes.get(start..start + 10) {
        let syncsafe = tag[6..10].iter().all(|&b| b < 0x80);
        if &tag[..3] != b"ID3" || tag[3] == 0xFF || tag[4] == 0xFF || !syncsafe {
            break;
        }
        let size = tag[6..10]
            .iter()
            .fold(0usize, |n, &b| (n << 7) | usize::from(b));
        let footer = if tag[5] & 0x10 != 0 { 10 } else { 0 };
        start = (start + 10 + size + footer).min(bytes.len());
    }
    tags.id3v2 = start;

    let mut end = bytes.len();
    loop {
        let room = end - start;
        if !tags.id3v1 && room >= 128 && &bytes[end - 128..end - 125] == b"TAG" {
            end -= 128;
            tags.id3v1 = true;
            continue;
        }
        if tags.ape == 0 && room >= 32 && &bytes[end - 32..end - 24] == b"APETAGEX" {
            let le = |at: usize| {
                u32::from_le_bytes([bytes[at], bytes[at + 1], bytes[at + 2], bytes[at + 3]])
            };
            let size = le(end - 20) as usize;
            let header = if le(end - 12) & (1 << 31) != 0 { 32 } else { 0 };
            if size >= 32 && size + header <= room {
                end -= size + header;
                tags.ape = size + header;
                continue;
            }
        }
        if tags.lyrics3 == 0 && room >= 15 && &bytes[end - 9..end] == b"LYRICS200" {
            let digits = &bytes[end - 15..end - 9];
            if digits.iter().all(u8::is_ascii_digit) {
                let size = digits
                    .iter()
                    .fold(0usize, |n, &d| n * 10 + usize::from(d - b'0'));
                if size + 15 <= room {
                    end -= size + 15;
                    tags.lyrics3 = size + 15;
                    continue;
                }
            }
        }
        break;
    }
    (start, end, tags)
}

pub(crate) fn header_at(bytes: &[u8], pos: usize) -> Option<FrameHeader> {
    let b = bytes.get(pos..pos + 4)?;
    FrameHeader::parse([b[0], b[1], b[2], b[3]])
}

/// Steps from frame to frame. A frame is taken where the previous one
/// ends if a header of the same stream is there; otherwise the walker
/// searches on for a header that's followed by another (or by the end of
/// the audio), counting the bytes it skips.
#[derive(Clone)]
pub(crate) struct FrameWalker {
    pub pos: usize,
    pub end: usize,
    pub first: FrameHeader,
    /// Free format: the frame size without padding, measured once.
    pub free_size: Option<usize>,
    /// Bytes skipped looking for frames.
    pub skipped: usize,
}

impl FrameWalker {
    /// Finds the first frame from `start`: a header followed by another of
    /// the same stream (or by the end).
    pub fn find(bytes: &[u8], start: usize, end: usize) -> Option<FrameWalker> {
        let mut pos = start;
        while pos + 4 <= end {
            if bytes[pos] == 0xFF {
                if let Some(h) = header_at(bytes, pos) {
                    let free_size = if h.bitrate_index == 0 {
                        measure_free_format(bytes, pos, &h, end)
                    } else {
                        None
                    };
                    let walker = FrameWalker {
                        pos,
                        end,
                        first: h,
                        free_size,
                        skipped: pos - start,
                    };
                    if walker.confirmed(bytes, pos) {
                        return Some(walker);
                    }
                }
            }
            pos += 1;
        }
        None
    }

    fn length(&self, h: &FrameHeader) -> Option<usize> {
        h.frame_length()
            .or_else(|| self.free_size.map(|size| size + usize::from(h.padding)))
    }

    /// Whether a frame of this stream starts at `pos` and is followed by
    /// another, or by the end of the audio.
    fn confirmed(&self, bytes: &[u8], pos: usize) -> bool {
        let Some(h) = header_at(bytes, pos).filter(|h| h.same_stream(&self.first)) else {
            return false;
        };
        let Some(len) = self.length(&h) else {
            return false;
        };
        let next = pos + len;
        next == self.end
            || (next + 4 <= self.end
                && header_at(bytes, next).is_some_and(|n| n.same_stream(&self.first)))
    }

    /// The next frame: where it starts, its header and its length.
    pub fn next(&mut self, bytes: &[u8]) -> Option<(usize, FrameHeader, usize)> {
        let here = if self.pos + 4 <= self.end {
            header_at(bytes, self.pos).filter(|h| h.same_stream(&self.first))
        } else {
            None
        };
        if let Some(h) = here {
            if let Some(len) = self.length(&h) {
                if self.pos + len <= self.end {
                    let at = self.pos;
                    self.pos += len;
                    return Some((at, h, len));
                }
            }
        }
        // Lost (junk between frames, or the end): search on.
        let from = self.pos;
        let mut pos = from + 1;
        while pos + 4 <= self.end {
            if bytes[pos] == 0xFF && self.confirmed(bytes, pos) {
                self.skipped += pos - from;
                self.pos = pos;
                return self.next(bytes);
            }
            pos += 1;
        }
        self.skipped += self.end.saturating_sub(from);
        self.pos = self.end;
        None
    }
}

/// A free-format frame's size without padding: the distance to the next
/// header of the same stream that's also free format.
fn measure_free_format(bytes: &[u8], pos: usize, h: &FrameHeader, end: usize) -> Option<usize> {
    let min = pos + 4 + h.side_info_length();
    // 640 kbit/s at 8 kHz is the largest free frame anyone writes.
    let max = (pos + 6000).min(end.saturating_sub(4));
    (min..=max)
        .find(|&p| {
            bytes[p] == 0xFF
                && header_at(bytes, p).is_some_and(|n| n.same_stream(h) && n.bitrate_index == 0)
        })
        .map(|p| p - pos - usize::from(h.padding))
}

/// The Xing (variable bitrate) or Info (constant bitrate) header that
/// LAME and others write into a silent first frame.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct XingHeader {
    /// `"Xing"` or `"Info"`.
    pub tag: String,
    /// Audio frames in the file, not counting this one.
    pub frames: Option<u32>,
    /// Bytes of audio, this frame included.
    pub bytes: Option<u32>,
    /// A 100-entry seek table is present.
    pub toc: bool,
    pub quality: Option<u32>,
}

/// LAME's extension of the Xing header.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LameTag {
    /// The encoder's name and version, e.g. `"LAME3.92 "`.
    pub encoder: String,
    /// Silent samples the encoder added at the start (the decoder's own
    /// 529-sample delay comes on top).
    pub delay: u32,
    /// Samples added at the end to fill the last frame.
    pub padding: u32,
    /// The tag's own checksum matches (CRC-16 of the frame up to it).
    pub crc_ok: bool,
}

impl LameTag {
    /// Whether the delay and padding can be trusted for gapless playback.
    pub fn gapless(&self) -> bool {
        self.crc_ok
            || ["LAME", "Lavf", "Lavc", "L3.9"]
                .iter()
                .any(|n| self.encoder.starts_with(n))
    }
}

/// Fraunhofer's VBRI header.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VbriHeader {
    pub version: u16,
    pub delay: u16,
    pub quality: u16,
    pub bytes: u32,
    pub frames: u32,
}

fn be32(b: &[u8], at: usize) -> Option<u32> {
    Some(u32::from_be_bytes(b.get(at..at + 4)?.try_into().ok()?))
}

/// Reads a Xing/Info header (and a LAME tag after it) from a first frame.
pub(crate) fn parse_xing(h: &FrameHeader, frame: &[u8]) -> Option<(XingHeader, Option<LameTag>)> {
    let after_side = h.side_info_start() + h.side_info_length();
    // Some writers leave out the checksum's two bytes when placing it.
    let at = [after_side, 4 + h.side_info_length()]
        .into_iter()
        .find(|&at| matches!(frame.get(at..at + 4), Some(b"Xing") | Some(b"Info")))?;
    let tag = String::from_utf8_lossy(&frame[at..at + 4]).into_owned();
    let flags = be32(frame, at + 4)?;
    let mut p = at + 8;
    let mut field = |present: bool, size: usize| {
        if !present {
            return None;
        }
        let v = be32(frame, p);
        p += size;
        v
    };
    let frames = field(flags & 1 != 0, 4);
    let bytes = field(flags & 2 != 0, 4);
    let toc = flags & 4 != 0 && field(true, 100).is_some();
    let quality = field(flags & 8 != 0, 4);
    let xing = XingHeader {
        tag,
        frames,
        bytes,
        toc,
        quality,
    };
    let lame = frame.get(p..p + 36).and_then(|t| {
        let name = &t[..9];
        if !name[..4].iter().all(u8::is_ascii_alphanumeric) {
            return None;
        }
        let stored = u16::from_be_bytes([t[34], t[35]]);
        Some(LameTag {
            encoder: String::from_utf8_lossy(name).into_owned(),
            delay: (u32::from(t[21]) << 4) | (u32::from(t[22]) >> 4),
            padding: (u32::from(t[22] & 0x0F) << 8) | u32::from(t[23]),
            crc_ok: lame_crc(&frame[..p + 34]) == stored,
        })
    });
    Some((xing, lame))
}

/// The CRC-16 LAME puts at the end of its tag (polynomial 0x8005 taken
/// least significant bit first, starting from 0).
fn lame_crc(bytes: &[u8]) -> u16 {
    let mut crc = 0u16;
    for &b in bytes {
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
}

/// Reads a VBRI header, which always sits 32 bytes after the header.
pub(crate) fn parse_vbri(frame: &[u8]) -> Option<VbriHeader> {
    let v = frame.get(36..54)?;
    if &v[..4] != b"VBRI" {
        return None;
    }
    let be16 = |at: usize| u16::from_be_bytes([v[at], v[at + 1]]);
    Some(VbriHeader {
        version: be16(4),
        delay: be16(6),
        quality: be16(8),
        bytes: be32(v, 10)?,
        frames: be32(v, 14)?,
    })
}

/// What a file holds, found by stepping through its frame headers without
/// decoding them.
#[derive(Clone, Debug)]
pub struct Scan {
    pub first: FrameHeader,
    pub tags: Tags,
    /// Audio frames, not counting a Xing/Info/VBRI frame.
    pub frames: usize,
    /// Bytes of audio frames.
    pub audio_bytes: usize,
    /// Bytes between frames that weren't frames (junk, damage).
    pub skipped: usize,
    pub min_bitrate: u32,
    pub max_bitrate: u32,
    /// Frames whose channel mode differs from the first's.
    pub mode_changes: usize,
}

/// Steps through a file's frames (see [`Scan`]).
pub fn scan(bytes: &[u8]) -> crate::Result<Scan> {
    let (start, end, tags) = audio_range(bytes);
    let mut walker = FrameWalker::find(bytes, start, end).ok_or(crate::Error::NoFrames)?;
    let first = walker.first;
    let first_pos = walker.pos;
    let mut scan = Scan {
        first,
        tags,
        frames: 0,
        audio_bytes: 0,
        skipped: 0,
        min_bitrate: u32::MAX,
        max_bitrate: 0,
        mode_changes: 0,
    };
    while let Some((pos, h, len)) = walker.next(bytes) {
        if pos == first_pos {
            let frame = &bytes[pos..pos + len];
            if parse_xing(&h, frame).is_some() || parse_vbri(frame).is_some() {
                continue;
            }
        }
        scan.frames += 1;
        scan.audio_bytes += len;
        let rate = if h.bitrate_index == 0 {
            (len * 8 * h.sample_rate() as usize / h.samples_per_frame()) as u32
        } else {
            h.bitrate()
        };
        scan.min_bitrate = scan.min_bitrate.min(rate);
        scan.max_bitrate = scan.max_bitrate.max(rate);
        if h.mode != first.mode {
            scan.mode_changes += 1;
        }
    }
    scan.skipped = walker.skipped;
    if scan.frames == 0 {
        scan.min_bitrate = 0;
    }
    Ok(scan)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tags_at_both_ends() {
        let mut file = b"ID3\x03\x00\x00\x00\x00\x00\x05hello".to_vec();
        file.extend([0xAB; 20]);
        // APEv2 with header: 32-byte header, 8 bytes of items, footer.
        let mut ape = b"APETAGEX".to_vec();
        ape.extend(2000u32.to_le_bytes());
        ape.extend(40u32.to_le_bytes()); // items + footer
        ape.extend(1u32.to_le_bytes());
        ape.extend((1u32 << 31 | 1 << 29).to_le_bytes());
        ape.extend([0; 8]);
        let header = ape.clone();
        file.extend(&header);
        file.extend([7; 8]);
        ape[20..24].copy_from_slice(&(1u32 << 31).to_le_bytes());
        file.extend(&ape);
        let mut v1 = b"TAG".to_vec();
        v1.resize(128, 0);
        file.extend(v1);
        let (start, end, tags) = audio_range(&file);
        assert_eq!((start, end), (15, 35));
        assert_eq!(tags.id3v2, 15);
        assert!(tags.id3v1);
        assert_eq!(tags.ape, 72);
        assert!(file[start..end].iter().all(|&b| b == 0xAB));
    }

    #[test]
    fn lame_checksum() {
        // CRC-16 (ARC) of "123456789" is 0xBB3D.
        assert_eq!(lame_crc(b"123456789"), 0xBB3D);
    }
}

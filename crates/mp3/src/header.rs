//! The four-byte header at the start of every MPEG audio frame (ISO
//! 11172-3 section 2.4.1.3, extended by ISO 13818-3 and MPEG-2.5).

/// Which MPEG audio standard a frame follows.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Version {
    /// MPEG-1 (ISO 11172-3): 32, 44.1 and 48 kHz. All of the game's music.
    Mpeg1,
    /// MPEG-2's lower sample rates (ISO 13818-3): 16, 22.05 and 24 kHz.
    Mpeg2,
    /// MPEG-2.5, Fraunhofer's extension to 8, 11.025 and 12 kHz.
    Mpeg25,
}

impl Version {
    pub fn name(self) -> &'static str {
        match self {
            Version::Mpeg1 => "MPEG-1",
            Version::Mpeg2 => "MPEG-2",
            Version::Mpeg25 => "MPEG-2.5",
        }
    }
}

/// How the channels are coded.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChannelMode {
    Stereo,
    /// Stereo where some of the spectrum is coded as mid/side or as one
    /// channel with a left/right balance (intensity stereo).
    JointStereo,
    /// Two independent mono channels.
    DualChannel,
    Mono,
}

impl ChannelMode {
    pub fn name(self) -> &'static str {
        match self {
            ChannelMode::Stereo => "stereo",
            ChannelMode::JointStereo => "joint stereo",
            ChannelMode::DualChannel => "dual channel",
            ChannelMode::Mono => "mono",
        }
    }
}

/// One frame's header.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FrameHeader {
    pub version: Version,
    /// 1, 2 or 3. Only Layer III (MP3) is decoded.
    pub layer: u8,
    /// A 16-bit checksum follows the header.
    pub protected: bool,
    /// 1 to 14; 0 is "free format" (the frame size has to be measured).
    pub bitrate_index: u8,
    /// 0 to 2 within the version's three sample rates.
    pub sample_rate_index: u8,
    /// The frame is one byte (Layer I: one 4-byte slot) longer.
    pub padding: bool,
    pub private: bool,
    pub mode: ChannelMode,
    /// Joint stereo: bit 1 mid/side stereo on, bit 0 intensity stereo on.
    pub mode_extension: u8,
    pub copyright: bool,
    pub original: bool,
    /// De-emphasis the encoder asked for (0 none, 1 50/15 µs, 3 CCITT
    /// J.17); not applied, as no common decoder does.
    pub emphasis: u8,
}

/// Bitrates in kbit/s by [MPEG-1, MPEG-2/2.5][layer - 1][index].
const BITRATES: [[[u16; 15]; 3]; 2] = [
    [
        [
            0, 32, 64, 96, 128, 160, 192, 224, 256, 288, 320, 352, 384, 416, 448,
        ],
        [
            0, 32, 48, 56, 64, 80, 96, 112, 128, 160, 192, 224, 256, 320, 384,
        ],
        [
            0, 32, 40, 48, 56, 64, 80, 96, 112, 128, 160, 192, 224, 256, 320,
        ],
    ],
    [
        [
            0, 32, 48, 56, 64, 80, 96, 112, 128, 144, 160, 176, 192, 224, 256,
        ],
        [0, 8, 16, 24, 32, 40, 48, 56, 64, 80, 96, 112, 128, 144, 160],
        [0, 8, 16, 24, 32, 40, 48, 56, 64, 80, 96, 112, 128, 144, 160],
    ],
];

/// Sample rates by version (MPEG-1, MPEG-2, MPEG-2.5) and index.
const SAMPLE_RATES: [[u32; 3]; 3] = [
    [44100, 48000, 32000],
    [22050, 24000, 16000],
    [11025, 12000, 8000],
];

impl FrameHeader {
    /// Reads a header, or `None` if these bytes can't be one (no sync
    /// word, or a reserved value in any field).
    pub fn parse(bytes: [u8; 4]) -> Option<FrameHeader> {
        let h = u32::from_be_bytes(bytes);
        if h >> 21 != 0x7FF {
            return None;
        }
        let version = match (h >> 19) & 3 {
            0 => Version::Mpeg25,
            2 => Version::Mpeg2,
            3 => Version::Mpeg1,
            _ => return None,
        };
        let layer = match (h >> 17) & 3 {
            1 => 3,
            2 => 2,
            3 => 1,
            _ => return None,
        };
        let bitrate_index = ((h >> 12) & 15) as u8;
        let sample_rate_index = ((h >> 10) & 3) as u8;
        let emphasis = (h & 3) as u8;
        if bitrate_index == 15 || sample_rate_index == 3 || emphasis == 2 {
            return None;
        }
        Some(FrameHeader {
            version,
            layer,
            protected: (h >> 16) & 1 == 0,
            bitrate_index,
            sample_rate_index,
            padding: (h >> 9) & 1 == 1,
            private: (h >> 8) & 1 == 1,
            mode: match (h >> 6) & 3 {
                0 => ChannelMode::Stereo,
                1 => ChannelMode::JointStereo,
                2 => ChannelMode::DualChannel,
                _ => ChannelMode::Mono,
            },
            mode_extension: ((h >> 4) & 3) as u8,
            copyright: (h >> 3) & 1 == 1,
            original: (h >> 2) & 1 == 1,
            emphasis,
        })
    }

    /// The header's four bytes.
    pub fn to_bytes(&self) -> [u8; 4] {
        let version = match self.version {
            Version::Mpeg25 => 0,
            Version::Mpeg2 => 2,
            Version::Mpeg1 => 3,
        };
        let mode = match self.mode {
            ChannelMode::Stereo => 0,
            ChannelMode::JointStereo => 1,
            ChannelMode::DualChannel => 2,
            ChannelMode::Mono => 3,
        };
        let h = (0x7FFu32 << 21)
            | (version << 19)
            | (u32::from(4 - self.layer) << 17)
            | (u32::from(!self.protected) << 16)
            | (u32::from(self.bitrate_index) << 12)
            | (u32::from(self.sample_rate_index) << 10)
            | (u32::from(self.padding) << 9)
            | (u32::from(self.private) << 8)
            | (mode << 6)
            | (u32::from(self.mode_extension & 3) << 4)
            | (u32::from(self.copyright) << 3)
            | (u32::from(self.original) << 2)
            | u32::from(self.emphasis & 3);
        h.to_be_bytes()
    }

    /// MPEG-2 or 2.5: one granule per frame, the "low sampling frequency"
    /// rules for scale factors and side information.
    pub fn is_lsf(&self) -> bool {
        self.version != Version::Mpeg1
    }

    /// Bits per second; 0 for free format.
    pub fn bitrate(&self) -> u32 {
        let v = usize::from(self.is_lsf());
        u32::from(BITRATES[v][usize::from(self.layer - 1)][usize::from(self.bitrate_index)]) * 1000
    }

    pub fn sample_rate(&self) -> u32 {
        let v = match self.version {
            Version::Mpeg1 => 0,
            Version::Mpeg2 => 1,
            Version::Mpeg25 => 2,
        };
        SAMPLE_RATES[v][usize::from(self.sample_rate_index)]
    }

    pub fn channels(&self) -> usize {
        if self.mode == ChannelMode::Mono {
            1
        } else {
            2
        }
    }

    /// Samples per channel in one frame.
    pub fn samples_per_frame(&self) -> usize {
        match self.layer {
            1 => 384,
            2 => 1152,
            _ if self.is_lsf() => 576,
            _ => 1152,
        }
    }

    /// The frame's size in bytes, header included; `None` for free format,
    /// whose size has to be found from where the next frame starts.
    pub fn frame_length(&self) -> Option<usize> {
        let bitrate = self.bitrate() as usize;
        if bitrate == 0 {
            return None;
        }
        Some(self.length_for(bitrate))
    }

    /// The size a frame at `bitrate` bits per second would have.
    pub(crate) fn length_for(&self, bitrate: usize) -> usize {
        let rate = self.sample_rate() as usize;
        let pad = usize::from(self.padding);
        match self.layer {
            1 => (12 * bitrate / rate + pad) * 4,
            3 if self.is_lsf() => 72 * bitrate / rate + pad,
            _ => 144 * bitrate / rate + pad,
        }
    }

    /// Bytes of Layer III side information after the header (and checksum).
    pub fn side_info_length(&self) -> usize {
        match (self.is_lsf(), self.channels()) {
            (false, 1) => 17,
            (false, _) => 32,
            (true, 1) => 9,
            (true, _) => 17,
        }
    }

    /// Where the side information starts: after the header and checksum.
    pub(crate) fn side_info_start(&self) -> usize {
        if self.protected {
            6
        } else {
            4
        }
    }

    pub fn ms_stereo(&self) -> bool {
        self.mode == ChannelMode::JointStereo && self.mode_extension & 2 != 0
    }

    pub fn intensity_stereo(&self) -> bool {
        self.mode == ChannelMode::JointStereo && self.mode_extension & 1 != 0
    }

    /// Whether another frame can belong to the same stream: the same
    /// version, layer and sample rate (the bitrate and the channel mode may
    /// change from frame to frame).
    pub fn same_stream(&self, other: &FrameHeader) -> bool {
        self.version == other.version
            && self.layer == other.layer
            && self.sample_rate_index == other.sample_rate_index
    }
}

/// The CRC-16 that protects a frame (polynomial 0x8005, starting from all
/// ones, most significant bit first), over the header's last two bytes and
/// the side information.
pub(crate) fn frame_crc(header_bytes: &[u8], side_info: &[u8]) -> u16 {
    let mut crc = 0xFFFFu16;
    for &byte in header_bytes[2..4].iter().chain(side_info) {
        for bit in (0..8).rev() {
            let input = u16::from((byte >> bit) & 1);
            let top = crc >> 15;
            crc <<= 1;
            if top ^ input != 0 {
                crc ^= 0x8005;
            }
        }
    }
    crc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_games_header() {
        // Doc Mitchell's theme: FF FB B4 00.
        let h = FrameHeader::parse([0xFF, 0xFB, 0xB4, 0x00]).unwrap();
        assert_eq!(h.version, Version::Mpeg1);
        assert_eq!(h.layer, 3);
        assert!(!h.protected);
        assert_eq!(h.bitrate(), 192_000);
        assert_eq!(h.sample_rate(), 48_000);
        assert_eq!(h.mode, ChannelMode::Stereo);
        assert_eq!(h.frame_length(), Some(576));
        assert_eq!(h.samples_per_frame(), 1152);
        assert_eq!(h.to_bytes(), [0xFF, 0xFB, 0xB4, 0x00]);
    }

    #[test]
    fn frame_lengths_for_every_version() {
        // 128 kbit/s, 44.1 kHz, padded: 144 * 128000 / 44100 = 417 (+1).
        let h = FrameHeader::parse([0xFF, 0xFB, 0x92, 0x00]).unwrap();
        assert_eq!(
            (h.bitrate(), h.sample_rate(), h.padding),
            (128_000, 44_100, true)
        );
        assert_eq!(h.frame_length(), Some(418));
        // MPEG-2, 64 kbit/s, 22.05 kHz, mono: 72 * 64000 / 22050 = 208.
        let h = FrameHeader::parse([0xFF, 0xF3, 0x80, 0xC0]).unwrap();
        assert_eq!(h.version, Version::Mpeg2);
        assert_eq!((h.bitrate(), h.sample_rate()), (64_000, 22_050));
        assert_eq!(h.frame_length(), Some(208));
        assert_eq!((h.samples_per_frame(), h.side_info_length()), (576, 9));
        // MPEG-2.5, 8 kbit/s, 8 kHz: 72 * 8000 / 8000 = 72.
        let h = FrameHeader::parse([0xFF, 0xE3, 0x18, 0xC4]).unwrap();
        assert_eq!(h.version, Version::Mpeg25);
        assert_eq!((h.bitrate(), h.sample_rate()), (8_000, 8_000));
        assert_eq!(h.frame_length(), Some(72));
        // Free format has no length of its own.
        let h = FrameHeader::parse([0xFF, 0xFB, 0x04, 0x00]).unwrap();
        assert_eq!(h.frame_length(), None);
        // Reserved values aren't headers.
        assert!(FrameHeader::parse([0xFF, 0xFB, 0xF0, 0x00]).is_none()); // bitrate 15
        assert!(FrameHeader::parse([0xFF, 0xFB, 0x0C, 0x00]).is_none()); // rate 3
        assert!(FrameHeader::parse([0xFF, 0xF9, 0x90, 0x00]).is_none()); // layer 0
        assert!(FrameHeader::parse([0xFF, 0xEB, 0x90, 0x00]).is_none()); // version 1
        assert!(FrameHeader::parse([0xFF, 0xFB, 0x90, 0x02]).is_none()); // emphasis 2
        assert!(FrameHeader::parse([0xFE, 0xFB, 0x90, 0x00]).is_none());
    }
}

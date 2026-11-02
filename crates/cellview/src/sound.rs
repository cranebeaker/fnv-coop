//! The game's sound files: WAV, as the game stores its effects (16-bit
//! PCM, stereo or mono: a Vegas door's opening sound is 44,100 Hz stereo),
//! decoded to samples; OGG files are left to the player (the viewer's
//! audio decodes them itself).

/// Decoded sound: interleaved 16-bit samples.
#[derive(Debug, Clone, PartialEq)]
pub struct Pcm {
    pub channels: u16,
    pub rate: u32,
    pub samples: Vec<i16>,
}

/// Reads a WAV file: a RIFF `WAVE` with a `fmt ` chunk (format 1, PCM, 8
/// or 16 bits) and a `data` chunk.
pub fn read_wav(bytes: &[u8]) -> Result<Pcm, String> {
    if bytes.len() < 12 || &bytes[..4] != b"RIFF" || &bytes[8..12] != b"WAVE" {
        return Err("not a WAV file".into());
    }
    let u16_at = |i: usize| u16::from_le_bytes([bytes[i], bytes[i + 1]]);
    let u32_at =
        |i: usize| u32::from_le_bytes([bytes[i], bytes[i + 1], bytes[i + 2], bytes[i + 3]]);
    let mut format = None;
    let mut at = 12;
    while at + 8 <= bytes.len() {
        let id = &bytes[at..at + 4];
        let size = u32_at(at + 4) as usize;
        let body = at + 8;
        let end = (body + size).min(bytes.len());
        if id == b"fmt " && size >= 16 {
            format = Some((
                u16_at(body),
                u16_at(body + 2),
                u32_at(body + 4),
                u16_at(body + 14),
            ));
        } else if id == b"data" {
            let Some((tag, channels, rate, bits)) = format else {
                return Err("the samples come before their format".into());
            };
            if tag != 1 {
                return Err(format!("format {tag} isn't plain PCM"));
            }
            let data = &bytes[body..end];
            let samples = match bits {
                16 => data
                    .chunks_exact(2)
                    .map(|c| i16::from_le_bytes([c[0], c[1]]))
                    .collect(),
                8 => data.iter().map(|&b| (i16::from(b) - 128) << 8).collect(),
                other => return Err(format!("{other}-bit samples")),
            };
            return Ok(Pcm {
                channels: channels.max(1),
                rate,
                samples,
            });
        }
        // Chunks are padded to an even size.
        at = body + size + (size & 1);
    }
    Err("no samples".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn wav(bits: u16, channels: u16, data: &[u8]) -> Vec<u8> {
        let mut v = b"RIFF".to_vec();
        v.extend(((36 + data.len()) as u32).to_le_bytes());
        v.extend(b"WAVEfmt ");
        v.extend(16u32.to_le_bytes());
        v.extend(1u16.to_le_bytes());
        v.extend(channels.to_le_bytes());
        v.extend(22050u32.to_le_bytes());
        v.extend((22050 * u32::from(channels) * u32::from(bits / 8)).to_le_bytes());
        v.extend((channels * bits / 8).to_le_bytes());
        v.extend(bits.to_le_bytes());
        // A chunk to skip, odd-sized, then the samples.
        v.extend(b"LIST");
        v.extend(3u32.to_le_bytes());
        v.extend([1, 2, 3, 0]);
        v.extend(b"data");
        v.extend((data.len() as u32).to_le_bytes());
        v.extend(data);
        v
    }

    #[test]
    fn reads_sixteen_and_eight_bit_samples() {
        let p = read_wav(&wav(16, 2, &[0x34, 0x12, 0x00, 0x80])).unwrap();
        assert_eq!((p.channels, p.rate), (2, 22050));
        assert_eq!(p.samples, vec![0x1234, i16::MIN]);
        let p = read_wav(&wav(8, 1, &[128, 255, 0])).unwrap();
        assert_eq!(p.samples, vec![0, 127 << 8, -128 << 8]);
        assert!(read_wav(b"nothing").is_err());
    }
}

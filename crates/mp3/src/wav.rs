//! A WAV writer for decoded audio: 16-bit PCM, the format the game's own
//! sound effects use (and `cellview::sound::read_wav` reads).

use std::io::{self, Write};

/// Writes interleaved 16-bit samples as a WAV file.
pub fn write(
    out: &mut impl Write,
    sample_rate: u32,
    channels: u16,
    samples: &[i16],
) -> io::Result<()> {
    let data_bytes = u32::try_from(samples.len() * 2)
        .ok()
        .filter(|&n| n <= u32::MAX - 36)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "too long for a WAV file"))?;
    let block_align = channels * 2;
    out.write_all(b"RIFF")?;
    out.write_all(&(36 + data_bytes).to_le_bytes())?;
    out.write_all(b"WAVEfmt ")?;
    out.write_all(&16u32.to_le_bytes())?;
    out.write_all(&1u16.to_le_bytes())?; // PCM
    out.write_all(&channels.to_le_bytes())?;
    out.write_all(&sample_rate.to_le_bytes())?;
    out.write_all(&(sample_rate * u32::from(block_align)).to_le_bytes())?;
    out.write_all(&block_align.to_le_bytes())?;
    out.write_all(&16u16.to_le_bytes())?;
    out.write_all(b"data")?;
    out.write_all(&data_bytes.to_le_bytes())?;
    let mut bytes = Vec::with_capacity(samples.len() * 2);
    for s in samples {
        bytes.extend_from_slice(&s.to_le_bytes());
    }
    out.write_all(&bytes)
}

#[cfg(test)]
mod tests {
    #[test]
    fn writes_a_canonical_header() {
        let mut out = Vec::new();
        super::write(&mut out, 48_000, 2, &[1, -2, 0x1234, -32768]).unwrap();
        assert_eq!(out.len(), 44 + 8);
        assert_eq!(&out[..4], b"RIFF");
        assert_eq!(u32::from_le_bytes(out[4..8].try_into().unwrap()), 44);
        assert_eq!(&out[8..16], b"WAVEfmt ");
        assert_eq!(u16::from_le_bytes([out[22], out[23]]), 2);
        assert_eq!(u32::from_le_bytes(out[24..28].try_into().unwrap()), 48_000);
        assert_eq!(u32::from_le_bytes(out[28..32].try_into().unwrap()), 192_000);
        assert_eq!(u16::from_le_bytes([out[32], out[33]]), 4);
        assert_eq!(u16::from_le_bytes([out[34], out[35]]), 16);
        assert_eq!(&out[36..40], b"data");
        assert_eq!(u32::from_le_bytes(out[40..44].try_into().unwrap()), 8);
        assert_eq!(&out[44..], &[1, 0, 0xFE, 0xFF, 0x34, 0x12, 0x00, 0x80]);
    }
}

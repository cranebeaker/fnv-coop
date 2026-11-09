//! Opus codec behind the `opus` feature. Not compiled by default or in tests;
//! NOT yet built or verified. Mono, 48 kHz, 20 ms frames (960 samples).

use crate::codec::{Codec, CodecError, CODEC_OPUS};

pub struct OpusCodec {
    enc: opus::Encoder,
    dec: opus::Decoder,
}

impl OpusCodec {
    pub const RATE: u32 = 48_000;
    pub const FRAME: usize = 960;

    pub fn new() -> Result<Self, opus::Error> {
        let mut enc =
            opus::Encoder::new(Self::RATE, opus::Channels::Mono, opus::Application::Voip)?;
        enc.set_bitrate(opus::Bitrate::Bits(24_000))?;
        let dec = opus::Decoder::new(Self::RATE, opus::Channels::Mono)?;
        Ok(Self { enc, dec })
    }
}

impl Codec for OpusCodec {
    fn id(&self) -> u8 {
        CODEC_OPUS
    }
    fn sample_rate(&self) -> u32 {
        Self::RATE
    }
    fn frame_samples(&self) -> usize {
        Self::FRAME
    }

    fn encode(&mut self, pcm: &[i16], out: &mut Vec<u8>) {
        let mut buf = [0u8; 1200];
        if let Ok(n) = self.enc.encode(pcm, &mut buf) {
            out.extend_from_slice(&buf[..n]);
        }
    }

    fn decode(&mut self, data: Option<&[u8]>, out: &mut Vec<i16>) -> Result<(), CodecError> {
        let mut buf = [0i16; Self::FRAME];
        // An empty input slice asks libopus for packet loss concealment.
        let n = self
            .dec
            .decode(data.unwrap_or(&[]), &mut buf, false)
            .map_err(|_| CodecError::BadFrame)?;
        if n != Self::FRAME {
            return Err(CodecError::BadFrame);
        }
        out.extend_from_slice(&buf);
        Ok(())
    }
}

//! Codec trait plus built-in test codecs (raw i16 and IMA ADPCM).

pub const CODEC_RAW: u8 = 0;
pub const CODEC_ADPCM: u8 = 1;
pub const CODEC_OPUS: u8 = 2;

#[derive(Debug, PartialEq, Eq)]
pub enum CodecError {
    BadFrame,
}

/// A mono voice codec working on fixed-size frames.
pub trait Codec: Send {
    fn id(&self) -> u8;
    fn sample_rate(&self) -> u32;
    /// Samples per frame (mono).
    fn frame_samples(&self) -> usize;
    /// Encode exactly `frame_samples()` samples, appending bytes to `out`.
    fn encode(&mut self, pcm: &[i16], out: &mut Vec<u8>);
    /// Decode a frame (`Some`) or conceal a lost one (`None`). Appends
    /// exactly `frame_samples()` samples to `out`.
    fn decode(&mut self, data: Option<&[u8]>, out: &mut Vec<i16>) -> Result<(), CodecError>;
}

/// Raw little-endian i16. Useful as the identity codec in tests.
pub struct RawCodec {
    rate: u32,
    frame: usize,
    last: Vec<i16>,
}

impl RawCodec {
    pub fn new(rate: u32, frame: usize) -> Self {
        Self {
            rate,
            frame,
            last: vec![0; frame],
        }
    }
}

impl Codec for RawCodec {
    fn id(&self) -> u8 {
        CODEC_RAW
    }
    fn sample_rate(&self) -> u32 {
        self.rate
    }
    fn frame_samples(&self) -> usize {
        self.frame
    }
    fn encode(&mut self, pcm: &[i16], out: &mut Vec<u8>) {
        for s in pcm.iter().take(self.frame) {
            out.extend_from_slice(&s.to_le_bytes());
        }
    }
    fn decode(&mut self, data: Option<&[u8]>, out: &mut Vec<i16>) -> Result<(), CodecError> {
        match data {
            Some(d) => {
                if d.len() != self.frame * 2 {
                    return Err(CodecError::BadFrame);
                }
                self.last = d
                    .chunks_exact(2)
                    .map(|c| i16::from_le_bytes([c[0], c[1]]))
                    .collect();
            }
            None => conceal(&mut self.last),
        }
        out.extend_from_slice(&self.last);
        Ok(())
    }
}

/// Repeat-and-fade concealment shared by the test codecs.
fn conceal(last: &mut [i16]) {
    for s in last.iter_mut() {
        *s = (*s as i32 * 3 / 4) as i16;
    }
}

const STEP: [i32; 89] = [
    7, 8, 9, 10, 11, 12, 13, 14, 16, 17, 19, 21, 23, 25, 28, 31, 34, 37, 41, 45, 50, 55, 60, 66,
    73, 80, 88, 97, 107, 118, 130, 143, 157, 173, 190, 209, 230, 253, 279, 307, 337, 371, 408, 449,
    494, 544, 598, 658, 724, 796, 876, 963, 1060, 1166, 1282, 1411, 1552, 1707, 1878, 2066, 2272,
    2499, 2749, 3024, 3327, 3660, 4026, 4428, 4871, 5358, 5894, 6484, 7132, 7845, 8630, 9493,
    10442, 11487, 12635, 13899, 15289, 16818, 18500, 20350, 22385, 24623, 27086, 29794, 32767,
];
const INDEX_ADJ: [i32; 8] = [-1, -1, -1, -1, 2, 4, 6, 8];

#[derive(Clone, Copy, Default)]
struct AdpcmState {
    pred: i32,
    index: i32,
}

impl AdpcmState {
    fn encode_sample(&mut self, s: i16) -> u8 {
        let step = STEP[self.index as usize];
        let mut diff = s as i32 - self.pred;
        let mut code = 0u8;
        if diff < 0 {
            code = 8;
            diff = -diff;
        }
        let mut temp = step;
        if diff >= temp {
            code |= 4;
            diff -= temp;
        }
        temp >>= 1;
        if diff >= temp {
            code |= 2;
            diff -= temp;
        }
        temp >>= 1;
        if diff >= temp {
            code |= 1;
        }
        self.apply(code);
        code
    }

    fn apply(&mut self, code: u8) -> i16 {
        let step = STEP[self.index as usize];
        let mut diff = step >> 3;
        if code & 4 != 0 {
            diff += step;
        }
        if code & 2 != 0 {
            diff += step >> 1;
        }
        if code & 1 != 0 {
            diff += step >> 2;
        }
        if code & 8 != 0 {
            self.pred -= diff
        } else {
            self.pred += diff
        }
        self.pred = self.pred.clamp(-32768, 32767);
        self.index = (self.index + INDEX_ADJ[(code & 7) as usize]).clamp(0, 88);
        self.pred as i16
    }
}

/// IMA ADPCM, 4 bits per sample. Every frame carries its own predictor and
/// step index (3 bytes), so frames decode independently and loss never
/// corrupts later frames. Frame size must be even.
pub struct AdpcmCodec {
    rate: u32,
    frame: usize,
    enc: AdpcmState,
    last: Vec<i16>,
}

impl AdpcmCodec {
    pub fn new(rate: u32, frame: usize) -> Self {
        assert!(frame % 2 == 0, "frame size must be even");
        Self {
            rate,
            frame,
            enc: AdpcmState::default(),
            last: vec![0; frame],
        }
    }
}

impl Codec for AdpcmCodec {
    fn id(&self) -> u8 {
        CODEC_ADPCM
    }
    fn sample_rate(&self) -> u32 {
        self.rate
    }
    fn frame_samples(&self) -> usize {
        self.frame
    }

    fn encode(&mut self, pcm: &[i16], out: &mut Vec<u8>) {
        let first = pcm.first().copied().unwrap_or(0);
        self.enc.pred = first as i32;
        out.extend_from_slice(&first.to_le_bytes());
        out.push(self.enc.index as u8);
        let mut st = self.enc;
        for pair in pcm.chunks(2).take(self.frame / 2) {
            let lo = st.encode_sample(pair[0]);
            let hi = st.encode_sample(*pair.get(1).unwrap_or(&pair[0]));
            out.push(lo | (hi << 4));
        }
        self.enc = st;
    }

    fn decode(&mut self, data: Option<&[u8]>, out: &mut Vec<i16>) -> Result<(), CodecError> {
        match data {
            Some(d) => {
                if d.len() != 3 + self.frame / 2 || d[2] > 88 {
                    return Err(CodecError::BadFrame);
                }
                let mut st = AdpcmState {
                    pred: i16::from_le_bytes([d[0], d[1]]) as i32,
                    index: d[2] as i32,
                };
                self.last.clear();
                for b in &d[3..] {
                    self.last.push(st.apply(b & 15));
                    self.last.push(st.apply(b >> 4));
                }
            }
            None => conceal(&mut self.last),
        }
        out.extend_from_slice(&self.last);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sine(n: usize, ph: usize) -> Vec<i16> {
        (0..n)
            .map(|i| (((i + ph) as f32 * 0.1).sin() * 8000.0) as i16)
            .collect()
    }

    #[test]
    fn raw_is_identity() {
        let mut c = RawCodec::new(16000, 320);
        let pcm = sine(320, 0);
        let mut b = vec![];
        c.encode(&pcm, &mut b);
        let mut o = vec![];
        c.decode(Some(&b), &mut o).unwrap();
        assert_eq!(pcm, o);
    }

    #[test]
    fn adpcm_close_and_small() {
        let mut c = AdpcmCodec::new(16000, 320);
        let pcm = sine(320, 0);
        let mut b = vec![];
        c.encode(&pcm, &mut b);
        assert_eq!(b.len(), 163);
        let mut o = vec![];
        c.decode(Some(&b), &mut o).unwrap();
        let err: i64 = pcm
            .iter()
            .zip(&o)
            .map(|(a, b)| (*a as i64 - *b as i64).abs())
            .sum::<i64>()
            / 320;
        assert!(err < 600, "mean abs err {err}");
    }

    #[test]
    fn conceal_fades() {
        let mut c = AdpcmCodec::new(16000, 320);
        let mut b = vec![];
        c.encode(&sine(320, 0), &mut b);
        let mut o = vec![];
        c.decode(Some(&b), &mut o).unwrap();
        let mut o2 = vec![];
        c.decode(None, &mut o2).unwrap();
        assert_eq!(o2.len(), 320);
        let e1: i64 = o.iter().map(|x| (*x as i64).abs()).sum();
        let e2: i64 = o2.iter().map(|x| (*x as i64).abs()).sum();
        assert!(e2 < e1);
    }

    #[test]
    fn bad_frame_rejected() {
        let mut c = AdpcmCodec::new(16000, 320);
        assert_eq!(
            c.decode(Some(&[1, 2]), &mut vec![]),
            Err(CodecError::BadFrame)
        );
    }
}

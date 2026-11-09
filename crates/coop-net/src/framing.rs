//! Fragmentation and reassembly, shared by every backend.
//!
//! Reliable messages above [`MAX_FRAGMENT`] bytes are split into pieces with a
//! 1-byte header (whole / part / last). The reliable lanes are ordered, so
//! reassembly is a plain append. Unreliable frames carry a 4-byte sequence so
//! stale and duplicate datagrams can be discarded.

use thiserror::Error;

/// Largest payload of one reliable fragment.
pub const MAX_FRAGMENT: usize = 60_000;
/// Largest unreliable payload.
pub const MAX_UNRELIABLE: usize = 1_100;
/// Default cap on one reassembled reliable message.
pub const DEFAULT_MAX_MESSAGE: usize = 32 * 1024 * 1024;

const H_WHOLE: u8 = 0;
const H_PART: u8 = 1;
const H_LAST: u8 = 2;

/// Framing failures.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum FrameError {
    /// Empty frame or unknown header byte.
    #[error("malformed frame")]
    Malformed,
    /// A whole frame arrived in the middle of a fragmented message, or a last
    /// frame arrived with nothing before it that was expected.
    #[error("fragment out of sequence")]
    OutOfSequence,
    /// Reassembled message would exceed the cap.
    #[error("message exceeds {0} bytes")]
    TooBig(usize),
    /// Unreliable payload over the limit.
    #[error("unreliable payload of {len} bytes exceeds {max}")]
    UnreliableTooLarge {
        /// Payload size.
        len: usize,
        /// Limit.
        max: usize,
    },
}

/// Splits `msg` into framed reliable fragments of at most `max_fragment`
/// payload bytes each (`0 < max_fragment`).
pub fn fragment(msg: &[u8], max_fragment: usize) -> Vec<Vec<u8>> {
    let max_fragment = max_fragment.max(1);
    if msg.len() <= max_fragment {
        let mut f = Vec::with_capacity(msg.len() + 1);
        f.push(H_WHOLE);
        f.extend_from_slice(msg);
        return vec![f];
    }
    let n = msg.len().div_ceil(max_fragment);
    msg.chunks(max_fragment)
        .enumerate()
        .map(|(i, c)| {
            let mut f = Vec::with_capacity(c.len() + 1);
            f.push(if i + 1 == n { H_LAST } else { H_PART });
            f.extend_from_slice(c);
            f
        })
        .collect()
}

/// Per-peer reassembly state for one reliable lane.
#[derive(Debug)]
pub struct Reassembler {
    buf: Vec<u8>,
    active: bool,
    max_message: usize,
}

impl Default for Reassembler {
    fn default() -> Self {
        Self::new(DEFAULT_MAX_MESSAGE)
    }
}

impl Reassembler {
    /// A reassembler capping messages at `max_message` bytes.
    pub fn new(max_message: usize) -> Self {
        Reassembler {
            buf: Vec::new(),
            active: false,
            max_message,
        }
    }

    /// True while a fragmented message is half received.
    pub fn in_progress(&self) -> bool {
        self.active
    }

    fn reset(&mut self) {
        self.buf = Vec::new();
        self.active = false;
    }

    /// Feeds one frame. Returns the finished message, if this frame completed
    /// one. On error the partial message is discarded.
    pub fn push(&mut self, frame: &[u8]) -> Result<Option<Vec<u8>>, FrameError> {
        let (&h, body) = frame.split_first().ok_or(FrameError::Malformed)?;
        let r = match h {
            H_WHOLE => {
                if self.active {
                    Err(FrameError::OutOfSequence)
                } else if body.len() > self.max_message {
                    Err(FrameError::TooBig(self.max_message))
                } else {
                    return Ok(Some(body.to_vec()));
                }
            }
            H_PART | H_LAST => {
                if self.buf.len() + body.len() > self.max_message {
                    Err(FrameError::TooBig(self.max_message))
                } else {
                    self.buf.extend_from_slice(body);
                    if h == H_LAST {
                        if !self.active {
                            // A "last" with no parts is a lone fragment: reject.
                            self.reset();
                            return Err(FrameError::OutOfSequence);
                        }
                        let out = std::mem::take(&mut self.buf);
                        self.reset();
                        return Ok(Some(out));
                    }
                    self.active = true;
                    return Ok(None);
                }
            }
            _ => Err(FrameError::Malformed),
        };
        self.reset();
        r.map(|()| None)
    }
}

/// Frames an unreliable payload: 4-byte little endian sequence then data.
pub fn encode_unreliable(seq: u32, data: &[u8]) -> Result<Vec<u8>, FrameError> {
    if data.len() > MAX_UNRELIABLE {
        return Err(FrameError::UnreliableTooLarge {
            len: data.len(),
            max: MAX_UNRELIABLE,
        });
    }
    let mut f = Vec::with_capacity(data.len() + 4);
    f.extend_from_slice(&seq.to_le_bytes());
    f.extend_from_slice(data);
    Ok(f)
}

/// Splits an unreliable frame into sequence and payload.
pub fn decode_unreliable(frame: &[u8]) -> Result<(u32, &[u8]), FrameError> {
    if frame.len() < 4 {
        return Err(FrameError::Malformed);
    }
    Ok((
        u32::from_le_bytes([frame[0], frame[1], frame[2], frame[3]]),
        &frame[4..],
    ))
}

/// Serial-number comparison: true if `a` is newer than `b` (wrapping).
pub fn seq_newer(a: u32, b: u32) -> bool {
    (a.wrapping_sub(b) as i32) > 0
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn small_is_whole() {
        let f = fragment(b"hi", MAX_FRAGMENT);
        assert_eq!(f, vec![vec![0, b'h', b'i']]);
        let mut r = Reassembler::default();
        assert_eq!(r.push(&f[0]).unwrap().unwrap(), b"hi");
    }

    #[test]
    fn empty_message() {
        let f = fragment(b"", MAX_FRAGMENT);
        let mut r = Reassembler::default();
        assert_eq!(r.push(&f[0]).unwrap().unwrap(), Vec::<u8>::new());
    }

    #[test]
    fn big_blob_uses_60000_pieces() {
        let msg = vec![7u8; 5 * 1024 * 1024];
        let f = fragment(&msg, MAX_FRAGMENT);
        assert_eq!(f.len(), msg.len().div_ceil(MAX_FRAGMENT));
        assert!(f.iter().all(|x| x.len() <= MAX_FRAGMENT + 1));
        assert_eq!(f[0][0], H_PART);
        assert_eq!(f.last().unwrap()[0], H_LAST);
        let mut r = Reassembler::default();
        let mut out = None;
        for x in &f {
            out = r.push(x).unwrap();
        }
        assert_eq!(out.unwrap(), msg);
    }

    #[test]
    fn exact_multiple_boundaries() {
        for len in [
            MAX_FRAGMENT - 1,
            MAX_FRAGMENT,
            MAX_FRAGMENT + 1,
            2 * MAX_FRAGMENT,
        ] {
            let msg: Vec<u8> = (0..len).map(|i| i as u8).collect();
            let mut r = Reassembler::default();
            let mut out = None;
            for x in fragment(&msg, MAX_FRAGMENT) {
                out = r.push(&x).unwrap();
            }
            assert_eq!(out.unwrap(), msg, "len {len}");
        }
    }

    #[test]
    fn cap_enforced_and_reset() {
        let mut r = Reassembler::new(100);
        let f = fragment(&vec![0u8; 250], 60);
        let mut err = None;
        for x in &f {
            if let Err(e) = r.push(x) {
                err = Some(e);
                break;
            }
        }
        assert_eq!(err, Some(FrameError::TooBig(100)));
        assert!(!r.in_progress());
        // Usable afterwards.
        assert!(r.push(&fragment(b"ok", 60)[0]).unwrap().is_some());
    }

    #[test]
    fn whole_inside_fragmented_is_rejected() {
        let mut r = Reassembler::default();
        let f = fragment(&[1u8; 10], 4);
        r.push(&f[0]).unwrap();
        assert_eq!(r.push(&[H_WHOLE, 1]), Err(FrameError::OutOfSequence));
        assert!(!r.in_progress());
    }

    #[test]
    fn lone_last_rejected() {
        let mut r = Reassembler::default();
        assert_eq!(r.push(&[H_LAST, 1, 2]), Err(FrameError::OutOfSequence));
    }

    #[test]
    fn unreliable_limit() {
        assert!(encode_unreliable(1, &[0; MAX_UNRELIABLE]).is_ok());
        assert!(matches!(
            encode_unreliable(1, &[0; MAX_UNRELIABLE + 1]),
            Err(FrameError::UnreliableTooLarge { .. })
        ));
        let f = encode_unreliable(0xdead_beef, b"xyz").unwrap();
        assert_eq!(decode_unreliable(&f).unwrap(), (0xdead_beef, &b"xyz"[..]));
        assert!(decode_unreliable(&[1, 2]).is_err());
    }

    #[test]
    fn serial_compare_wraps() {
        assert!(seq_newer(1, 0));
        assert!(seq_newer(0, u32::MAX));
        assert!(!seq_newer(0, 1));
        assert!(!seq_newer(5, 5));
    }

    proptest! {
        #[test]
        fn roundtrip(msg in proptest::collection::vec(any::<u8>(), 0..5000), frag in 1usize..700) {
            let mut r = Reassembler::default();
            let mut done = Vec::new();
            for x in fragment(&msg, frag) {
                if let Some(m) = r.push(&x).unwrap() { done.push(m); }
            }
            prop_assert_eq!(done.len(), 1);
            prop_assert_eq!(&done[0], &msg);
        }

        #[test]
        fn back_to_back_messages(a in proptest::collection::vec(any::<u8>(), 0..900),
                                  b in proptest::collection::vec(any::<u8>(), 0..900), frag in 1usize..200) {
            let mut r = Reassembler::default();
            let mut done = Vec::new();
            for x in fragment(&a, frag).into_iter().chain(fragment(&b, frag)) {
                if let Some(m) = r.push(&x).unwrap() { done.push(m); }
            }
            prop_assert_eq!(done, vec![a, b]);
        }

        #[test]
        fn swapped_fragments_never_reproduce_original(msg in proptest::collection::vec(any::<u8>(), 50..800), frag in 1usize..40,
                                                       i in 0usize..1000, j in 0usize..1000) {
            let mut f = fragment(&msg, frag);
            let (i, j) = (i % f.len(), j % f.len());
            prop_assume!(f[i] != f[j]);
            f.swap(i, j);
            let mut r = Reassembler::default();
            let mut out = None;
            for x in &f {
                match r.push(x) {
                    Ok(Some(m)) => out = Some(m),
                    Ok(None) => {}
                    Err(_) => { out = None; break; }
                }
            }
            prop_assert_ne!(out, Some(msg));
        }

        #[test]
        fn garbage_never_panics(frames in proptest::collection::vec(proptest::collection::vec(any::<u8>(), 0..300), 0..40)) {
            let mut r = Reassembler::new(1000);
            for x in &frames { let _ = r.push(x); }
            for x in &frames { let _ = decode_unreliable(x); }
        }
    }
}

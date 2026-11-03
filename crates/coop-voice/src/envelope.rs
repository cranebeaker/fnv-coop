//! Voice packet envelope and the adapter contract.
//!
//! Wire layout (little endian, 12 byte header):
//!
//! ```text
//! 0  u8   version (1)
//! 1  u8   flags   (bit0 END_OF_TALK, bit1 PTT_ACTIVE)
//! 2  u8   codec id
//! 3  u8   speaker slot (player slot assigned by the host)
//! 4  u16  sequence (wraps)
//! 6  u32  timestamp in samples at the codec sample rate
//! 10 u16  payload length
//! 12 ..   payload
//! ```
//!
//! # Adapter contract
//!
//! `coop-core` owns the real network message enum. To carry voice, its
//! `social` module should add one unreliable, unordered message variant that
//! wraps `Vec<u8>` and call [`VoicePacket::encode`] / [`VoicePacket::decode`]
//! at the edges. Implement [`VoiceTransport`] for the session type to plug it
//! in. The host should overwrite `speaker` with the sender's authoritative
//! slot before relaying, so clients cannot impersonate each other.

pub const VERSION: u8 = 1;
pub const HEADER_LEN: usize = 12;
/// Largest payload accepted (keeps datagrams under a typical MTU).
pub const MAX_PAYLOAD: usize = 1200;

pub const FLAG_END_OF_TALK: u8 = 1;
pub const FLAG_PTT_ACTIVE: u8 = 2;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VoicePacket {
    pub flags: u8,
    pub codec: u8,
    pub speaker: u8,
    pub seq: u16,
    pub timestamp: u32,
    pub payload: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DecodeError {
    TooShort,
    BadVersion(u8),
    BadLength,
}

impl VoicePacket {
    pub fn is_end_of_talk(&self) -> bool {
        self.flags & FLAG_END_OF_TALK != 0
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(HEADER_LEN + self.payload.len());
        out.push(VERSION);
        out.push(self.flags);
        out.push(self.codec);
        out.push(self.speaker);
        out.extend_from_slice(&self.seq.to_le_bytes());
        out.extend_from_slice(&self.timestamp.to_le_bytes());
        out.extend_from_slice(&(self.payload.len() as u16).to_le_bytes());
        out.extend_from_slice(&self.payload);
        out
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        if bytes.len() < HEADER_LEN {
            return Err(DecodeError::TooShort);
        }
        if bytes[0] != VERSION {
            return Err(DecodeError::BadVersion(bytes[0]));
        }
        let len = u16::from_le_bytes([bytes[10], bytes[11]]) as usize;
        if len > MAX_PAYLOAD || bytes.len() != HEADER_LEN + len {
            return Err(DecodeError::BadLength);
        }
        Ok(VoicePacket {
            flags: bytes[1],
            codec: bytes[2],
            speaker: bytes[3],
            seq: u16::from_le_bytes([bytes[4], bytes[5]]),
            timestamp: u32::from_le_bytes([bytes[6], bytes[7], bytes[8], bytes[9]]),
            payload: bytes[HEADER_LEN..].to_vec(),
        })
    }
}

/// Implemented by the session type that owns the real connection.
pub trait VoiceTransport {
    /// Send a datagram unreliably and unordered. Dropping it is fine.
    fn send_unreliable(&mut self, datagram: &[u8]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_and_errors() {
        let p = VoicePacket {
            flags: 3,
            codec: 1,
            speaker: 2,
            seq: 65535,
            timestamp: 99,
            payload: vec![1, 2, 3],
        };
        let b = p.encode();
        assert_eq!(VoicePacket::decode(&b), Ok(p));
        assert_eq!(VoicePacket::decode(&b[..5]), Err(DecodeError::TooShort));
        assert_eq!(
            VoicePacket::decode(&b[..b.len() - 1]),
            Err(DecodeError::BadLength)
        );
        let mut bad = b.clone();
        bad[0] = 9;
        assert_eq!(VoicePacket::decode(&bad), Err(DecodeError::BadVersion(9)));
    }
}

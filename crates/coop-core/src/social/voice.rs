//! Push-to-talk voice envelope: validation and relay on the host, ordering on
//! the receiver. The codec payload is opaque here.

use std::collections::{BTreeMap, VecDeque};

use super::{VoicePacket, MAX_VOICE_PAYLOAD};
use crate::ids::PlayerId;
use crate::interest::Audience;

/// Packets per second one speaker may send.
pub const MAX_PACKETS_PER_SEC: usize = 80;
/// Payload bytes per second one speaker may send.
pub const MAX_BYTES_PER_SEC: usize = 24 * 1024;

/// Host-side voice validation.
#[derive(Clone, Debug, Default)]
pub struct VoiceHub {
    window: BTreeMap<PlayerId, VecDeque<(u64, usize)>>,
    /// Packets dropped for a forged speaker byte.
    pub forged: u32,
    /// Packets dropped by the rate limit.
    pub limited: u32,
    /// Packets relayed.
    pub relayed: u32,
}

impl VoiceHub {
    /// Empty hub.
    pub fn new() -> Self {
        Self::default()
    }

    /// Validates a packet received from transport peer `from`. Returns who to
    /// relay it to (everyone but the sender) or `None` if dropped.
    pub fn on_packet(
        &mut self,
        from: PlayerId,
        pkt: &VoicePacket,
        now_ms: u64,
    ) -> Option<Audience> {
        if pkt.speaker != from {
            self.forged += 1;
            return None;
        }
        if pkt.payload.is_empty() || pkt.payload.len() > MAX_VOICE_PAYLOAD {
            return None;
        }
        let w = self.window.entry(from).or_default();
        while w
            .front()
            .map_or(false, |(t, _)| now_ms.saturating_sub(*t) >= 1000)
        {
            w.pop_front();
        }
        let bytes: usize = w.iter().map(|(_, b)| *b).sum();
        if w.len() >= MAX_PACKETS_PER_SEC || bytes + pkt.payload.len() > MAX_BYTES_PER_SEC {
            self.limited += 1;
            return None;
        }
        w.push_back((now_ms, pkt.payload.len()));
        self.relayed += 1;
        Some(Audience::AllExcept(from))
    }

    /// Forgets a speaker.
    pub fn remove(&mut self, p: PlayerId) {
        self.window.remove(&p);
    }
}

/// Receiver-side ordering per speaker: drops duplicates and late frames and
/// counts gaps so the decoder can conceal them.
#[derive(Clone, Debug, Default)]
pub struct VoiceRx {
    last: BTreeMap<PlayerId, u16>,
    /// Frames missing between accepted frames (decoder conceals these).
    pub gaps: u32,
}

impl VoiceRx {
    /// Empty receiver.
    pub fn new() -> Self {
        Self::default()
    }

    /// Accepts a frame in order; returns how many frames were skipped before
    /// it, or `None` for a duplicate or late frame.
    pub fn push(&mut self, pkt: &VoicePacket) -> Option<u16> {
        let Some(last) = self.last.get(&pkt.speaker).copied() else {
            self.last.insert(pkt.speaker, pkt.seq);
            return Some(0);
        };
        let ahead = pkt.seq.wrapping_sub(last);
        if ahead == 0 || ahead >= 0x8000 {
            // a very large backwards jump is a restarted speaker, not a late frame
            if last.wrapping_sub(pkt.seq) > 1000 {
                self.last.insert(pkt.speaker, pkt.seq);
                return Some(0);
            }
            return None;
        }
        self.last.insert(pkt.speaker, pkt.seq);
        let skipped = ahead - 1;
        self.gaps += skipped as u32;
        Some(skipped)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const A: PlayerId = PlayerId(1);

    fn pk(speaker: PlayerId, seq: u16, n: usize) -> VoicePacket {
        VoicePacket {
            speaker,
            seq,
            payload: vec![7; n],
        }
    }

    #[test]
    fn relays_to_all_but_sender_and_rejects_forgery() {
        let mut h = VoiceHub::new();
        assert_eq!(
            h.on_packet(A, &pk(A, 1, 60), 0),
            Some(Audience::AllExcept(A))
        );
        assert_eq!(h.on_packet(A, &pk(PlayerId(2), 2, 60), 0), None);
        assert_eq!((h.forged, h.relayed), (1, 1));
    }

    #[test]
    fn empty_and_oversized_are_dropped() {
        let mut h = VoiceHub::new();
        assert!(h.on_packet(A, &pk(A, 1, 0), 0).is_none());
        assert!(h
            .on_packet(A, &pk(A, 1, MAX_VOICE_PAYLOAD + 1), 0)
            .is_none());
    }

    #[test]
    fn packet_rate_limit() {
        let mut h = VoiceHub::new();
        let ok = (0..200u16)
            .filter(|i| h.on_packet(A, &pk(A, *i, 10), 500).is_some())
            .count();
        assert_eq!(ok, MAX_PACKETS_PER_SEC);
        assert!(
            h.on_packet(A, &pk(A, 999, 10), 1500).is_some(),
            "window slides"
        );
        assert_eq!(h.limited, 120);
    }

    #[test]
    fn byte_rate_limit() {
        let mut h = VoiceHub::new();
        let ok = (0..70u16)
            .filter(|i| h.on_packet(A, &pk(A, *i, 480), 0).is_some())
            .count();
        assert_eq!(ok, MAX_BYTES_PER_SEC / 480);
    }

    #[test]
    fn normal_opus_stream_is_never_limited() {
        // 20 ms frames of ~60 bytes: 50 packets and 3 KB per second
        let mut h = VoiceHub::new();
        for i in 0..500u16 {
            assert!(h.on_packet(A, &pk(A, i, 60), i as u64 * 20).is_some());
        }
    }

    #[test]
    fn receiver_orders_and_counts_gaps() {
        let mut r = VoiceRx::new();
        assert_eq!(r.push(&pk(A, 10, 1)), Some(0));
        assert_eq!(r.push(&pk(A, 11, 1)), Some(0));
        assert_eq!(r.push(&pk(A, 11, 1)), None, "duplicate");
        assert_eq!(r.push(&pk(A, 14, 1)), Some(2));
        assert_eq!(r.push(&pk(A, 12, 1)), None, "late");
        assert_eq!(r.gaps, 2);
    }

    #[test]
    fn receiver_handles_wrap_and_restart() {
        let mut r = VoiceRx::new();
        r.push(&pk(A, 65534, 1));
        assert_eq!(r.push(&pk(A, 0, 1)), Some(1));
        r.push(&pk(A, 30_000, 1));
        assert_eq!(r.push(&pk(A, 5, 1)), Some(0), "speaker restarted");
        assert_eq!(r.push(&pk(A, 6, 1)), Some(0));
    }
}

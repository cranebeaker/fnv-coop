//! Transport abstraction, an in-memory loopback, and a seeded chaos wrapper.

mod chaos;
mod loopback;
mod profile;

use std::cell::Cell;
use std::rc::Rc;

pub use chaos::{ChaosStats, ChaosTransport};
pub use loopback::{LoopbackEndpoint, LoopbackNet};
pub use profile::{ChaosProfile, PROFILE_NAMES};

use thiserror::Error;

use crate::ids::PlayerId;
use crate::proto::Channel;

/// Transport failures.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum TransportError {
    /// No such peer on this network.
    #[error("unknown peer {0:?}")]
    UnknownPeer(PlayerId),
    /// Payload over the channel's size limit.
    #[error("payload of {len} bytes exceeds {max}")]
    TooLarge {
        /// Payload size.
        len: usize,
        /// Channel limit.
        max: usize,
    },
}

/// A received datagram or reliable message.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Packet {
    /// Sender.
    pub from: PlayerId,
    /// Channel it arrived on.
    pub channel: Channel,
    /// Payload.
    pub data: Vec<u8>,
}

/// Per-peer message transport. Reliable-ordered messages must arrive exactly
/// once and in order per sender; unreliable ones may not.
pub trait Transport {
    /// This endpoint's own peer id.
    fn local(&self) -> PlayerId;
    /// Queues a message to `to`.
    fn send(&mut self, to: PlayerId, channel: Channel, data: Vec<u8>)
        -> Result<(), TransportError>;
    /// Next received message, if any.
    fn recv(&mut self) -> Option<Packet>;
}

/// Shared virtual clock in milliseconds. Cheap to clone; all clones agree.
#[derive(Clone, Debug, Default)]
pub struct VirtualClock(Rc<Cell<u64>>);

impl VirtualClock {
    /// A clock at t = 0.
    pub fn new() -> Self {
        Self::default()
    }

    /// Current time in ms.
    pub fn now(&self) -> u64 {
        self.0.get()
    }

    /// Moves time forward.
    pub fn advance(&self, ms: u64) {
        self.0.set(self.0.get() + ms);
    }
}

/// Small deterministic generator (SplitMix64) so chaos needs no dependency
/// and a seed reproduces a run exactly.
#[derive(Clone, Debug)]
pub struct SeededRng(u64);

impl SeededRng {
    /// Seeds the generator.
    pub fn new(seed: u64) -> Self {
        SeededRng(seed)
    }

    /// Next 64 random bits.
    pub fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// Uniform in `0..n` (`n > 0`).
    pub fn below(&mut self, n: u64) -> u64 {
        self.next_u64() % n.max(1)
    }

    /// Uniform float in `[0, 1)`.
    pub fn unit(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }

    /// True with probability `p`.
    pub fn chance(&mut self, p: f64) -> bool {
        p > 0.0 && self.unit() < p
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rng_is_deterministic_and_spread() {
        let mut a = SeededRng::new(42);
        let mut b = SeededRng::new(42);
        let xs: Vec<u64> = (0..100).map(|_| a.next_u64()).collect();
        let ys: Vec<u64> = (0..100).map(|_| b.next_u64()).collect();
        assert_eq!(xs, ys);
        let mut c = SeededRng::new(43);
        assert_ne!(xs[0], c.next_u64());
        let mut r = SeededRng::new(1);
        let hits = (0..10_000).filter(|_| r.chance(0.1)).count();
        assert!((800..1200).contains(&hits), "hits {hits}");
        assert!((0..1000).all(|_| r.below(7) < 7));
    }

    #[test]
    fn clock_is_shared() {
        let c = VirtualClock::new();
        let d = c.clone();
        c.advance(5);
        assert_eq!(d.now(), 5);
    }
}

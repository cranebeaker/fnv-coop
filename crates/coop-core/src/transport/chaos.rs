//! Seeded fault injection around any [`Transport`], driven by a virtual clock.
//!
//! Outgoing messages are scheduled for release at a virtual time. The owner
//! advances the [`VirtualClock`] and calls [`ChaosTransport::pump`]; nothing
//! depends on wall time, so a seed reproduces a run exactly.
//!
//! Unreliable messages can be lost, duplicated, delayed and reordered, and
//! are dropped during a partition. Reliable-ordered messages are never lost
//! or reordered (as a TCP/QUIC stream would behave): loss shows up as extra
//! retransmit latency, and a partition holds them until the link heals.

use std::cmp::Ordering;
use std::collections::{BTreeMap, BinaryHeap};

use super::{ChaosProfile, Packet, SeededRng, Transport, TransportError, VirtualClock};
use crate::ids::PlayerId;
use crate::proto::Channel;

/// Counters of what the chaos layer did.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ChaosStats {
    /// Messages handed to `send`.
    pub sent: u64,
    /// Unreliable messages dropped (loss or partition).
    pub dropped: u64,
    /// Extra copies injected.
    pub duplicated: u64,
    /// Messages delayed by the reorder rule.
    pub reordered: u64,
    /// Reliable retransmissions simulated.
    pub retransmits: u64,
    /// Messages released to the inner transport.
    pub released: u64,
}

struct Pending {
    at: u64,
    order: u64,
    to: PlayerId,
    channel: Channel,
    data: Vec<u8>,
}

impl PartialEq for Pending {
    fn eq(&self, o: &Self) -> bool {
        self.at == o.at && self.order == o.order
    }
}
impl Eq for Pending {}
impl PartialOrd for Pending {
    fn partial_cmp(&self, o: &Self) -> Option<Ordering> {
        Some(self.cmp(o))
    }
}
impl Ord for Pending {
    // reversed so BinaryHeap pops the earliest (at, order)
    fn cmp(&self, o: &Self) -> Ordering {
        (o.at, o.order).cmp(&(self.at, self.order))
    }
}

/// Fault-injecting wrapper.
pub struct ChaosTransport<T: Transport> {
    inner: T,
    clock: VirtualClock,
    profile: ChaosProfile,
    rng: SeededRng,
    queue: BinaryHeap<Pending>,
    order: u64,
    last_reliable: BTreeMap<PlayerId, u64>,
    stats: ChaosStats,
}

impl<T: Transport> ChaosTransport<T> {
    /// Wraps `inner` with `profile`, seeded by `seed`.
    pub fn new(inner: T, clock: VirtualClock, profile: ChaosProfile, seed: u64) -> Self {
        ChaosTransport {
            inner,
            clock,
            profile,
            rng: SeededRng::new(seed),
            queue: BinaryHeap::new(),
            order: 0,
            last_reliable: BTreeMap::new(),
            stats: ChaosStats::default(),
        }
    }

    /// Counters so far.
    pub fn stats(&self) -> ChaosStats {
        self.stats
    }

    /// The active profile.
    pub fn profile(&self) -> &ChaosProfile {
        &self.profile
    }

    /// Changes the profile on the fly (e.g. to heal a link).
    pub fn set_profile(&mut self, p: ChaosProfile) {
        self.profile = p;
    }

    /// Messages still waiting for their release time.
    pub fn in_flight(&self) -> usize {
        self.queue.len()
    }

    /// Releases every message whose time has come into the inner transport.
    pub fn pump(&mut self) {
        let now = self.clock.now();
        while self.queue.peek().map_or(false, |p| p.at <= now) {
            let p = self.queue.pop().expect("peeked");
            let _ = self.inner.send(p.to, p.channel, p.data);
            self.stats.released += 1;
        }
    }

    fn schedule(&mut self, at: u64, to: PlayerId, channel: Channel, data: Vec<u8>) {
        self.order += 1;
        self.queue.push(Pending {
            at,
            order: self.order,
            to,
            channel,
            data,
        });
    }

    fn delay(&mut self) -> u64 {
        self.profile.latency_ms + self.rng.below(self.profile.jitter_ms + 1)
    }
}

impl<T: Transport> Transport for ChaosTransport<T> {
    fn local(&self) -> PlayerId {
        self.inner.local()
    }

    fn send(
        &mut self,
        to: PlayerId,
        channel: Channel,
        data: Vec<u8>,
    ) -> Result<(), TransportError> {
        if data.len() > channel.max_len() {
            return Err(TransportError::TooLarge {
                len: data.len(),
                max: channel.max_len(),
            });
        }
        self.pump();
        self.stats.sent += 1;
        let now = self.clock.now();
        let down_until = self.profile.partition_end(now);
        match channel {
            Channel::Unreliable => {
                if down_until.is_some() || self.rng.chance(self.profile.loss) {
                    self.stats.dropped += 1;
                    return Ok(());
                }
                let copies = if self.rng.chance(self.profile.dup) {
                    self.stats.duplicated += 1;
                    2
                } else {
                    1
                };
                for _ in 0..copies {
                    let mut d = self.delay();
                    if self.rng.chance(self.profile.reorder) {
                        d += self.profile.reorder_extra_ms;
                        self.stats.reordered += 1;
                    }
                    self.schedule(now + d, to, channel, data.clone());
                }
            }
            Channel::ReliableOrdered => {
                let mut d = self.delay();
                let mut tries = 0;
                while tries < 8 && self.rng.chance(self.profile.loss) {
                    d += 2 * self.profile.latency_ms + 20;
                    tries += 1;
                    self.stats.retransmits += 1;
                }
                let mut at = now + d;
                if let Some(end) = down_until {
                    at = at.max(end + self.profile.latency_ms);
                }
                // keep per-destination order
                let last = self.last_reliable.entry(to).or_insert(0);
                at = at.max(*last);
                *last = at;
                self.schedule(at, to, channel, data);
            }
        }
        Ok(())
    }

    fn recv(&mut self) -> Option<Packet> {
        self.pump();
        self.inner.recv()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transport::{LoopbackNet, PROFILE_NAMES};

    const A: PlayerId = PlayerId(0);
    const B: PlayerId = PlayerId(1);

    fn pair(
        profile: ChaosProfile,
        seed: u64,
    ) -> (
        ChaosTransport<crate::transport::LoopbackEndpoint>,
        crate::transport::LoopbackEndpoint,
        VirtualClock,
    ) {
        let net = LoopbackNet::new();
        let clock = VirtualClock::new();
        let a = ChaosTransport::new(net.endpoint(A), clock.clone(), profile, seed);
        (a, net.endpoint(B), clock)
    }

    fn drain(b: &mut crate::transport::LoopbackEndpoint) -> Vec<Packet> {
        std::iter::from_fn(|| b.recv()).collect()
    }

    #[test]
    fn perfect_profile_is_transparent() {
        let (mut a, mut b, _c) = pair(ChaosProfile::PERFECT, 1);
        for i in 0..10u8 {
            a.send(B, Channel::Unreliable, vec![i]).unwrap();
        }
        let got: Vec<u8> = drain(&mut b).iter().map(|p| p.data[0]).collect();
        // delivered once a pump runs (zero latency)
        a.pump();
        let got2: Vec<u8> = drain(&mut b).iter().map(|p| p.data[0]).collect();
        assert_eq!(got.len() + got2.len(), 10);
    }

    #[test]
    fn latency_holds_messages_until_clock_advances() {
        let (mut a, mut b, c) = pair(
            ChaosProfile {
                latency_ms: 100,
                ..ChaosProfile::PERFECT
            },
            1,
        );
        a.send(B, Channel::ReliableOrdered, vec![1]).unwrap();
        a.pump();
        assert!(drain(&mut b).is_empty());
        c.advance(99);
        a.pump();
        assert!(drain(&mut b).is_empty());
        c.advance(1);
        a.pump();
        assert_eq!(drain(&mut b).len(), 1);
    }

    #[test]
    fn unreliable_loss_dup_reorder_happen() {
        let (mut a, mut b, c) = pair(ChaosProfile::FLAKY, 7);
        for i in 0..2000u32 {
            a.send(B, Channel::Unreliable, i.to_le_bytes().to_vec())
                .unwrap();
            c.advance(5);
            a.pump();
        }
        c.advance(10_000);
        a.pump();
        let got: Vec<u32> = drain(&mut b)
            .iter()
            .map(|p| u32::from_le_bytes(p.data[..4].try_into().unwrap()))
            .collect();
        let s = a.stats();
        assert!(s.dropped > 200, "{s:?}");
        assert!(s.duplicated > 20, "{s:?}");
        assert!(got.len() < 2000 + s.duplicated as usize);
        assert!(got.windows(2).any(|w| w[1] < w[0]), "expected reordering");
        assert!(got.windows(2).any(|w| w[1] == w[0]) || s.duplicated > 0);
    }

    #[test]
    fn reliable_is_complete_and_ordered_under_every_profile() {
        for name in PROFILE_NAMES {
            let (mut a, mut b, c) = pair(ChaosProfile::by_name(name).unwrap(), 99);
            for i in 0..500u32 {
                a.send(B, Channel::ReliableOrdered, i.to_le_bytes().to_vec())
                    .unwrap();
                c.advance(7);
                a.pump();
            }
            c.advance(60_000);
            a.pump();
            let got: Vec<u32> = drain(&mut b)
                .iter()
                .map(|p| u32::from_le_bytes(p.data[..4].try_into().unwrap()))
                .collect();
            assert_eq!(got, (0..500).collect::<Vec<_>>(), "profile {name}");
        }
    }

    #[test]
    fn partition_drops_unreliable_and_holds_reliable() {
        let (mut a, mut b, c) = pair(ChaosProfile::PARTITION, 3);
        c.advance(4_600); // inside the outage (4500..6000)
        a.send(B, Channel::Unreliable, vec![1]).unwrap();
        a.send(B, Channel::ReliableOrdered, vec![2]).unwrap();
        c.advance(1_000); // 5600, still down
        a.pump();
        assert!(drain(&mut b).is_empty());
        c.advance(1_000); // 6600, healed
        a.pump();
        let got = drain(&mut b);
        assert_eq!(got.len(), 1);
        assert_eq!(got[0].data, vec![2]);
        assert_eq!(a.stats().dropped, 1);
    }

    #[test]
    fn same_seed_same_outcome() {
        let run = |seed| {
            let (mut a, mut b, c) = pair(ChaosProfile::BAD_WIFI, seed);
            for i in 0..300u32 {
                a.send(B, Channel::Unreliable, i.to_le_bytes().to_vec())
                    .unwrap();
                c.advance(3);
                a.pump();
            }
            c.advance(5_000);
            a.pump();
            drain(&mut b)
                .into_iter()
                .map(|p| p.data)
                .collect::<Vec<_>>()
        };
        assert_eq!(run(5), run(5));
        assert_ne!(run(5), run(6));
    }
}

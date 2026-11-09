//! UDP backend for LAN, direct IP and CI: `std::net::UdpSocket` only, no
//! async runtime, and a small reliable-ordered channel of its own.
//!
//! Datagram kinds (first byte):
//!
//! | kind | name | body |
//! |---|---|---|
//! | 1 | OPEN | token\[16\], session nonce u32 (guest to host, repeated until acked) |
//! | 2 | OPENACK | nothing |
//! | 3 | REL | seq u32, flags u8 (1 = last segment of a message), payload |
//! | 4 | ACK | next\_expected u32, bitmap u32 (bit i = seq next\_expected+1+i held) |
//! | 5 | UNREL | frame |
//! | 6 | CLOSE | nothing |
//!
//! Reliable messages (`[lane][frame]`) are cut into segments of at most
//! [`UdpConfig::seg_payload`] bytes, numbered, sent inside a sliding window and
//! retransmitted on a per-segment RTO with exponential backoff. The receiver
//! delivers strictly in order. The host ignores every datagram from an address
//! that has not sent a valid OPEN with the session token.
//!
//! [`ChaosCfg`] injects seeded loss, duplication and reordering on the send
//! path of the socket, so the same code path is exercised in tests.

use std::collections::{HashMap, VecDeque};
use std::io::ErrorKind;
use std::net::{SocketAddr, UdpSocket};

use coop_core::transport::SeededRng;

use crate::backend::{Backend, BackendError, BackendEvent, Lane, PeerKey};
use crate::endpoint::Token;
use crate::liveness::TOKEN_LEN;

const K_OPEN: u8 = 1;
const K_OPENACK: u8 = 2;
const K_REL: u8 = 3;
const K_ACK: u8 = 4;
const K_UNREL: u8 = 5;
const K_CLOSE: u8 = 6;

const RECV_WINDOW: u32 = 256;
const MAX_QUEUED_SEGS: usize = 32_768;
const MAX_ASSEMBLED: usize = 64 * 1024 * 1024;

/// Seeded fault injection on the socket send path.
#[derive(Clone, Copy, Debug)]
pub struct ChaosCfg {
    /// RNG seed; the same seed gives the same fault sequence.
    pub seed: u64,
    /// Probability a datagram is dropped.
    pub drop: f64,
    /// Probability a datagram is sent twice.
    pub dup: f64,
    /// Probability a datagram is held back and sent after the next one.
    pub reorder: f64,
}

/// Counters for tests and debugging.
#[derive(Clone, Copy, Debug, Default)]
pub struct UdpStats {
    /// Datagrams the backend tried to send.
    pub sent: u64,
    /// Dropped by chaos.
    pub dropped: u64,
    /// Duplicated by chaos.
    pub duplicated: u64,
    /// Reordered by chaos.
    pub reordered: u64,
    /// Reliable segments sent again after an RTO.
    pub retransmits: u64,
    /// Datagrams received.
    pub received: u64,
}

/// UDP backend settings.
#[derive(Clone, Debug)]
pub struct UdpConfig {
    /// Socket-level fault injection.
    pub chaos: Option<ChaosCfg>,
    /// Lower bound of the retransmission timeout.
    pub min_rto_ms: u64,
    /// Most simultaneous peers (host).
    pub max_peers: usize,
    /// Segments allowed in flight per peer.
    pub window: usize,
    /// Payload bytes per reliable segment (keeps datagrams under typical MTU).
    pub seg_payload: usize,
    /// How long a disconnecting peer may keep flushing queued data.
    pub close_grace_ms: u64,
}

impl Default for UdpConfig {
    fn default() -> Self {
        UdpConfig {
            chaos: None,
            min_rto_ms: 50,
            max_peers: 16,
            window: 64,
            seg_payload: 1000,
            close_grace_ms: 2000,
        }
    }
}

struct Seg {
    seq: u32,
    last: bool,
    payload: Vec<u8>,
    sent_at: u64,
    tries: u32,
    acked: bool,
}

struct Conn {
    addr: SocketAddr,
    nonce: u32,
    open: bool,
    last_open: Option<u64>,
    // sender
    base: u32,
    next_seq: u32,
    segs: VecDeque<Seg>,
    // receiver
    next_expected: u32,
    held: HashMap<u32, (bool, Vec<u8>)>,
    partial: Vec<u8>,
    // rtt
    srtt: Option<f64>,
    rttvar: f64,
    closing: Option<u64>,
}

impl Conn {
    fn new(_key: PeerKey, addr: SocketAddr, nonce: u32, open: bool) -> Self {
        Conn {
            addr,
            nonce,
            open,
            last_open: None,
            base: 0,
            next_seq: 0,
            segs: VecDeque::new(),
            next_expected: 0,
            held: HashMap::new(),
            partial: Vec::new(),
            srtt: None,
            rttvar: 0.0,
            closing: None,
        }
    }

    fn rto(&self, min: u64, tries: u32) -> u64 {
        let base = match self.srtt {
            Some(s) => (s + 4.0 * self.rttvar) as u64,
            None => 200,
        };
        let base = base.clamp(min, 2000);
        (base << tries.saturating_sub(1).min(4)).min(4000)
    }

    fn sample_rtt(&mut self, r: f64) {
        match self.srtt {
            None => {
                self.srtt = Some(r);
                self.rttvar = r / 2.0;
            }
            Some(s) => {
                self.rttvar = 0.75 * self.rttvar + 0.25 * (s - r).abs();
                self.srtt = Some(0.875 * s + 0.125 * r);
            }
        }
    }
}

enum Role {
    Host,
    Guest {
        host_key: PeerKey,
        host_addr: SocketAddr,
    },
}

/// UDP [`Backend`].
pub struct UdpBackend {
    sock: UdpSocket,
    role: Role,
    token: Token,
    cfg: UdpConfig,
    conns: HashMap<PeerKey, Conn>,
    addr_key: HashMap<SocketAddr, PeerKey>,
    next_key: PeerKey,
    rng: SeededRng,
    held_out: Option<(SocketAddr, Vec<u8>)>,
    events: Vec<BackendEvent>,
    now: u64,
    /// Fault and traffic counters.
    pub stats: UdpStats,
}

impl UdpBackend {
    /// Host: binds `bind` (use port 0 for an ephemeral port) and accepts
    /// guests that present `token`.
    pub fn host(bind: SocketAddr, token: Token, cfg: UdpConfig) -> std::io::Result<Self> {
        Self::new(bind, Role::Host, token, cfg)
    }

    /// Guest: binds an ephemeral port and targets the host at `host`.
    pub fn guest(host: SocketAddr, token: Token, cfg: UdpConfig) -> std::io::Result<Self> {
        let bind: SocketAddr = if host.is_ipv4() {
            "0.0.0.0:0".parse().unwrap()
        } else {
            "[::]:0".parse().unwrap()
        };
        let mut b = Self::new(
            bind,
            Role::Guest {
                host_key: 1,
                host_addr: host,
            },
            token,
            cfg,
        )?;
        let nonce = b.fresh_nonce();
        b.conns.insert(1, Conn::new(1, host, nonce, false));
        b.addr_key.insert(host, 1);
        b.next_key = 2;
        Ok(b)
    }

    fn new(bind: SocketAddr, role: Role, token: Token, cfg: UdpConfig) -> std::io::Result<Self> {
        let sock = UdpSocket::bind(bind)?;
        sock.set_nonblocking(true)?;
        let seed = cfg.chaos.map_or(0x5EED, |c| c.seed);
        Ok(UdpBackend {
            sock,
            role,
            token,
            cfg,
            conns: HashMap::new(),
            addr_key: HashMap::new(),
            next_key: 1,
            rng: SeededRng::new(seed),
            held_out: None,
            events: Vec::new(),
            now: 0,
            stats: UdpStats::default(),
        })
    }

    /// The bound local address (read the port after binding port 0).
    pub fn local_addr(&self) -> std::io::Result<SocketAddr> {
        self.sock.local_addr()
    }

    fn fresh_nonce(&self) -> u32 {
        use std::hash::{BuildHasher, Hasher};
        let mut h = std::collections::hash_map::RandomState::new().build_hasher();
        h.write_u64(self.now);
        h.finish() as u32
    }

    fn raw_send(&mut self, addr: SocketAddr, bytes: Vec<u8>) {
        self.stats.sent += 1;
        if let Some(c) = self.cfg.chaos {
            if self.rng.chance(c.drop) {
                self.stats.dropped += 1;
                return;
            }
            if self.rng.chance(c.reorder) {
                self.stats.reordered += 1;
                if let Some((a, b)) = self.held_out.replace((addr, bytes)) {
                    let _ = self.sock.send_to(&b, a);
                }
                return;
            }
            if self.rng.chance(c.dup) {
                self.stats.duplicated += 1;
                let _ = self.sock.send_to(&bytes, addr);
            }
        }
        let _ = self.sock.send_to(&bytes, addr);
        if let Some((a, b)) = self.held_out.take() {
            let _ = self.sock.send_to(&b, a);
        }
    }

    fn send_ack(&mut self, key: PeerKey) {
        let Some(c) = self.conns.get(&key) else {
            return;
        };
        let mut bm = 0u32;
        for i in 0..32u32 {
            if c.held
                .contains_key(&c.next_expected.wrapping_add(1).wrapping_add(i))
            {
                bm |= 1 << i;
            }
        }
        let mut p = vec![K_ACK];
        p.extend_from_slice(&c.next_expected.to_le_bytes());
        p.extend_from_slice(&bm.to_le_bytes());
        let addr = c.addr;
        self.raw_send(addr, p);
    }

    fn remove_conn(&mut self, key: PeerKey, notify: bool) {
        if let Some(c) = self.conns.remove(&key) {
            self.addr_key.remove(&c.addr);
            if notify {
                self.events.push(BackendEvent::PeerGone { peer: key });
            }
        }
    }

    fn on_datagram(&mut self, addr: SocketAddr, d: &[u8]) {
        self.stats.received += 1;
        let Some((&kind, body)) = d.split_first() else {
            return;
        };
        let is_host = matches!(self.role, Role::Host);
        if kind == K_OPEN {
            if !is_host || body.len() < TOKEN_LEN + 4 || body[..TOKEN_LEN] != self.token {
                return;
            }
            let nonce = u32::from_le_bytes(body[TOKEN_LEN..TOKEN_LEN + 4].try_into().unwrap());
            if let Some(&k) = self.addr_key.get(&addr) {
                if self.conns.get(&k).map(|c| c.nonce) == Some(nonce) {
                    self.raw_send(addr, vec![K_OPENACK]);
                    return;
                }
                // Same address, new session: the old one is dead.
                self.remove_conn(k, true);
            }
            if self.conns.len() >= self.cfg.max_peers {
                return;
            }
            let key = self.next_key;
            self.next_key += 1;
            self.conns.insert(key, Conn::new(key, addr, nonce, true));
            self.addr_key.insert(addr, key);
            self.raw_send(addr, vec![K_OPENACK]);
            return;
        }
        let Some(&key) = self.addr_key.get(&addr) else {
            return;
        };
        match kind {
            K_OPENACK => {
                if let Some(c) = self.conns.get_mut(&key) {
                    c.open = true;
                }
            }
            K_CLOSE => {
                let closing = self.conns.get(&key).is_some_and(|c| c.closing.is_some());
                self.remove_conn(key, !closing);
            }
            K_UNREL => {
                if let Some(c) = self.conns.get_mut(&key) {
                    c.open = true;
                    if c.closing.is_none() {
                        self.events.push(BackendEvent::Frame {
                            peer: key,
                            lane: Lane::Unreliable,
                            data: body.to_vec(),
                        });
                    }
                }
            }
            K_ACK => self.on_ack(key, body),
            K_REL => self.on_rel(key, body),
            _ => {}
        }
    }

    fn on_ack(&mut self, key: PeerKey, body: &[u8]) {
        if body.len() < 8 {
            return;
        }
        let ne = u32::from_le_bytes(body[0..4].try_into().unwrap());
        let bm = u32::from_le_bytes(body[4..8].try_into().unwrap());
        let now = self.now;
        let Some(c) = self.conns.get_mut(&key) else {
            return;
        };
        c.open = true;
        let n = ne.wrapping_sub(c.base) as usize;
        if n > c.segs.len() {
            return;
        }
        for _ in 0..n {
            if let Some(s) = c.segs.pop_front() {
                if s.tries == 1 && !s.acked {
                    c.sample_rtt(now.saturating_sub(s.sent_at) as f64);
                }
            }
        }
        c.base = ne;
        for i in 0..32usize {
            if bm & (1 << i) != 0 {
                if let Some(s) = c.segs.get_mut(1 + i) {
                    s.acked = true;
                }
            }
        }
    }

    fn on_rel(&mut self, key: PeerKey, body: &[u8]) {
        if body.len() < 5 {
            return;
        }
        let seq = u32::from_le_bytes(body[0..4].try_into().unwrap());
        let last = body[4] & 1 != 0;
        let Some(c) = self.conns.get_mut(&key) else {
            return;
        };
        c.open = true;
        let d = seq.wrapping_sub(c.next_expected);
        if d < RECV_WINDOW {
            c.held
                .entry(seq)
                .or_insert_with(|| (last, body[5..].to_vec()));
            let mut overflow = false;
            let mut out = Vec::new();
            while let Some((l, p)) = c.held.remove(&c.next_expected) {
                c.next_expected = c.next_expected.wrapping_add(1);
                c.partial.extend_from_slice(&p);
                if c.partial.len() > MAX_ASSEMBLED {
                    overflow = true;
                    break;
                }
                if l {
                    out.push(std::mem::take(&mut c.partial));
                }
            }
            let closing = c.closing.is_some();
            if overflow {
                self.remove_conn(key, true);
                return;
            }
            if !closing {
                for m in out {
                    if let Some((&lane, rest)) = m.split_first() {
                        if let Some(lane @ (Lane::Reliable | Lane::Control)) = Lane::from_u8(lane) {
                            self.events.push(BackendEvent::Frame {
                                peer: key,
                                lane,
                                data: rest.to_vec(),
                            });
                        }
                    }
                }
            }
        } else if d >= 0x8000_0000 {
            // Old duplicate: fall through to re-ack.
        } else {
            return; // too far ahead
        }
        self.send_ack(key);
    }

    fn transmit(&mut self, key: PeerKey, now: u64) {
        let (window, min_rto) = (self.cfg.window, self.cfg.min_rto_ms);
        let mut out: Vec<(SocketAddr, Vec<u8>)> = Vec::new();
        let mut retrans = 0;
        if let Some(c) = self.conns.get_mut(&key) {
            if !c.open {
                if matches!(self.role, Role::Guest { .. })
                    && c.last_open.map_or(true, |t| now.saturating_sub(t) >= 200)
                {
                    c.last_open = Some(now);
                    let mut p = vec![K_OPEN];
                    p.extend_from_slice(&self.token);
                    p.extend_from_slice(&c.nonce.to_le_bytes());
                    out.push((c.addr, p));
                }
            } else {
                let rtos: Vec<u64> = (0..=8).map(|t| c.rto(min_rto, t)).collect();
                let addr = c.addr;
                for s in c.segs.iter_mut().take(window) {
                    if s.acked {
                        continue;
                    }
                    let due = s.tries == 0
                        || now.saturating_sub(s.sent_at) >= rtos[(s.tries as usize).min(8)];
                    if due {
                        if s.tries > 0 {
                            retrans += 1;
                        }
                        s.tries += 1;
                        s.sent_at = now;
                        let mut p = Vec::with_capacity(s.payload.len() + 6);
                        p.push(K_REL);
                        p.extend_from_slice(&s.seq.to_le_bytes());
                        p.push(s.last as u8);
                        p.extend_from_slice(&s.payload);
                        out.push((addr, p));
                    }
                }
            }
        }
        self.stats.retransmits += retrans;
        for (a, p) in out {
            self.raw_send(a, p);
        }
    }
}

impl Backend for UdpBackend {
    fn send(&mut self, peer: PeerKey, lane: Lane, data: &[u8]) -> Result<(), BackendError> {
        let seg_payload = self.cfg.seg_payload;
        let now = self.now;
        if let Role::Guest {
            host_key,
            host_addr,
        } = self.role
        {
            // A guest that left (or was closed on) reopens lazily on its next send.
            if peer == host_key && !self.conns.contains_key(&peer) {
                let nonce = self.fresh_nonce();
                self.conns
                    .insert(peer, Conn::new(peer, host_addr, nonce, false));
                self.addr_key.insert(host_addr, peer);
            }
        }
        let Some(c) = self.conns.get_mut(&peer) else {
            return Err(BackendError::UnknownPeer(peer));
        };
        if c.closing.is_some() {
            return Err(BackendError::UnknownPeer(peer));
        }
        match lane {
            Lane::Unreliable => {
                if data.len() > 1200 {
                    return Err(BackendError::TooLarge {
                        len: data.len(),
                        max: 1200,
                    });
                }
                if c.open {
                    let addr = c.addr;
                    let mut p = vec![K_UNREL];
                    p.extend_from_slice(data);
                    self.raw_send(addr, p);
                }
                Ok(())
            }
            Lane::Reliable | Lane::Control => {
                let mut msg = Vec::with_capacity(data.len() + 1);
                msg.push(lane as u8);
                msg.extend_from_slice(data);
                let nsegs = msg.len().div_ceil(seg_payload);
                if c.segs.len() + nsegs > MAX_QUEUED_SEGS {
                    return Err(BackendError::Failed("udp send queue full".into()));
                }
                for (i, chunk) in msg.chunks(seg_payload).enumerate() {
                    c.segs.push_back(Seg {
                        seq: c.next_seq,
                        last: i + 1 == nsegs,
                        payload: chunk.to_vec(),
                        sent_at: 0,
                        tries: 0,
                        acked: false,
                    });
                    c.next_seq = c.next_seq.wrapping_add(1);
                }
                self.transmit(peer, now);
                Ok(())
            }
        }
    }

    fn poll(&mut self, now: u64) -> Vec<BackendEvent> {
        self.now = now;
        let mut buf = vec![0u8; 2048];
        for _ in 0..8192 {
            match self.sock.recv_from(&mut buf) {
                Ok((n, addr)) => {
                    let d = buf[..n].to_vec();
                    self.on_datagram(addr, &d);
                }
                Err(e) if e.kind() == ErrorKind::WouldBlock => break,
                // Windows reports ICMP port-unreachable as a recv error on UDP.
                Err(e) if e.kind() == ErrorKind::ConnectionReset => continue,
                Err(_) => break,
            }
        }
        let keys: Vec<PeerKey> = self.conns.keys().copied().collect();
        for k in keys {
            self.transmit(k, now);
            let done = self
                .conns
                .get(&k)
                .and_then(|c| c.closing.map(|dl| (c.segs.is_empty(), dl)));
            if let Some((empty, deadline)) = done {
                if empty || now >= deadline {
                    let addr = self.conns[&k].addr;
                    self.raw_send(addr, vec![K_CLOSE]);
                    self.remove_conn(k, false);
                }
            }
        }
        if let Some((a, b)) = self.held_out.take() {
            let _ = self.sock.send_to(&b, a);
        }
        std::mem::take(&mut self.events)
    }

    fn host_peer(&self) -> Option<PeerKey> {
        match self.role {
            Role::Guest { host_key, .. } => Some(host_key),
            Role::Host => None,
        }
    }

    fn disconnect(&mut self, peer: PeerKey) {
        let grace = self.cfg.close_grace_ms;
        let now = self.now;
        let Some(c) = self.conns.get_mut(&peer) else {
            return;
        };
        if c.open && c.segs.iter().any(|s| !s.acked) {
            c.closing = Some(now + grace);
        } else {
            let addr = c.addr;
            self.raw_send(addr, vec![K_CLOSE]);
            self.remove_conn(peer, false);
        }
    }

    /// The smoothed round trip, rounded up to a whole millisecond and at
    /// least 1. The status line uses 0 for "no sample yet", so a measured
    /// sub-millisecond trip on a local link must not read as 0.
    fn rtt_ms(&self, peer: PeerKey) -> Option<u32> {
        self.conns
            .get(&peer)
            .and_then(|c| c.srtt)
            .map(|s| (s.ceil() as u32).max(1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pair(chaos: Option<ChaosCfg>) -> (UdpBackend, UdpBackend) {
        let token = [3u8; 16];
        let cfg = UdpConfig {
            chaos,
            min_rto_ms: 10,
            ..Default::default()
        };
        let h = UdpBackend::host("127.0.0.1:0".parse().unwrap(), token, cfg.clone()).unwrap();
        let addr = h.local_addr().unwrap();
        let g = UdpBackend::guest(addr, token, cfg).unwrap();
        (h, g)
    }

    fn run_until(
        h: &mut UdpBackend,
        g: &mut UdpBackend,
        ms: u64,
        mut f: impl FnMut(&mut UdpBackend, &mut UdpBackend, &[BackendEvent], &[BackendEvent]) -> bool,
    ) -> bool {
        let t0 = std::time::Instant::now();
        loop {
            let now = t0.elapsed().as_millis() as u64;
            let he = h.poll(now);
            let ge = g.poll(now);
            if f(h, g, &he, &ge) {
                return true;
            }
            if now > ms {
                return false;
            }
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
    }

    #[test]
    fn open_and_exchange() {
        let (mut h, mut g) = pair(None);
        g.send(1, Lane::Control, b"hello").unwrap();
        let mut got = Vec::new();
        let mut hkey = 0;
        assert!(run_until(&mut h, &mut g, 2000, |_, _, he, _| {
            for e in he {
                if let BackendEvent::Frame { peer, lane, data } = e {
                    got.push((*lane, data.clone()));
                    hkey = *peer;
                }
            }
            !got.is_empty()
        }));
        assert_eq!(got[0], (Lane::Control, b"hello".to_vec()));
        h.send(hkey, Lane::Reliable, &vec![9u8; 5000]).unwrap();
        let mut back = None;
        assert!(run_until(&mut h, &mut g, 2000, |_, _, _, ge| {
            for e in ge {
                if let BackendEvent::Frame {
                    lane: Lane::Reliable,
                    data,
                    ..
                } = e
                {
                    back = Some(data.clone());
                }
            }
            back.is_some()
        }));
        assert_eq!(back.unwrap(), vec![9u8; 5000]);
    }

    #[test]
    fn a_measured_round_trip_never_reads_as_zero() {
        let (mut h, mut g) = pair(None);
        g.send(1, Lane::Control, b"hello").unwrap();
        assert!(run_until(&mut h, &mut g, 2000, |_, g, _, _| !g
            .conns
            .is_empty()));
        let k = *g.conns.keys().next().unwrap();
        g.conns.get_mut(&k).unwrap().srtt = None;
        assert_eq!(g.rtt_ms(k), None, "no sample yet");
        // a local link: the smoothed trip is below one millisecond
        g.conns.get_mut(&k).unwrap().srtt = Some(0.3);
        assert_eq!(g.rtt_ms(k), Some(1));
        g.conns.get_mut(&k).unwrap().srtt = Some(16.2);
        assert_eq!(g.rtt_ms(k), Some(17));
    }

    #[test]
    fn wrong_token_is_ignored() {
        let cfg = UdpConfig::default();
        let mut h = UdpBackend::host("127.0.0.1:0".parse().unwrap(), [1; 16], cfg.clone()).unwrap();
        let mut g = UdpBackend::guest(h.local_addr().unwrap(), [2; 16], cfg).unwrap();
        g.send(1, Lane::Control, b"x").unwrap();
        let ok = run_until(&mut h, &mut g, 300, |_, _, he, _| !he.is_empty());
        assert!(!ok);
        assert!(h.conns.is_empty());
    }

    #[test]
    fn chaos_is_seeded() {
        let c = ChaosCfg {
            seed: 5,
            drop: 0.5,
            dup: 0.0,
            reorder: 0.0,
        };
        let run = || {
            let (mut h, _g) = pair(Some(c));
            let a: SocketAddr = "127.0.0.1:9".parse().unwrap();
            for _ in 0..200 {
                h.raw_send(a, vec![1]);
            }
            h.stats.dropped
        };
        assert_eq!(run(), run());
        assert!(run() > 50);
    }

    #[test]
    fn unreliable_size_cap() {
        let (mut h, mut g) = pair(None);
        g.send(1, Lane::Control, b"x").unwrap();
        run_until(&mut h, &mut g, 1000, |_, _, he, _| !he.is_empty());
        assert!(matches!(
            g.send(1, Lane::Unreliable, &[0; 1201]),
            Err(BackendError::TooLarge { .. })
        ));
    }
}

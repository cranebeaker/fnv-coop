//! [`Endpoint`]: the shared layer that turns any [`Backend`] into a coop-core
//! [`Transport`]. Star topology: the host is `PlayerId(0)`, guests get slots
//! 1.. at the handshake, and guests can only talk to the host.

use std::collections::{HashMap, VecDeque};

use coop_core::ids::PlayerId;
use coop_core::proto::Channel;
use coop_core::transport::{Packet, Transport, TransportError};

use crate::backend::{Backend, BackendError, BackendEvent, Lane, PeerKey, Readiness};
use crate::clock::Clock;
use crate::framing::{self, FrameError, Reassembler, MAX_FRAGMENT, MAX_UNRELIABLE};
use crate::liveness::{Ctl, Liveness, RejectCode, NET_PROTO, TIMEOUT_MS, TOKEN_LEN};

/// 16-byte session token carried by the join code.
pub type Token = [u8; TOKEN_LEN];

/// Why a peer left.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LeaveReason {
    /// The peer said goodbye.
    Goodbye,
    /// Silent for 20 seconds.
    Timeout,
    /// Removed by the host (we are the host and called `kick`, or we were kicked).
    Kicked(String),
    /// The backend reported the peer gone (close packet, lobby leave, session failure).
    BackendGone,
    /// The peer sent frames that violate the framing rules.
    ProtocolError,
}

/// Connection-level events, drained with [`Endpoint::events`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NetEvent {
    /// Guest: the host accepted us; we are `local`.
    Connected {
        /// Our slot.
        local: PlayerId,
    },
    /// Host: a guest joined.
    PeerJoined {
        /// Its slot.
        player: PlayerId,
        /// Its display name.
        name: String,
    },
    /// A peer is gone (host sees guests, a guest sees `PlayerId(0)`).
    PeerLeft {
        /// The slot.
        player: PlayerId,
        /// Why.
        reason: LeaveReason,
    },
    /// Guest: the host refused the join.
    Rejected(RejectCode),
    /// Guest: could not join at all (backend failure, no answer within 20 s).
    JoinFailed(String),
    /// A send failed below the transport (reliable data may be lost; the
    /// liveness timeout will end the session).
    SendError(String),
}

/// Endpoint settings.
#[derive(Clone, Debug)]
pub struct EndpointConfig {
    /// Display name sent in the join request.
    pub name: String,
    /// Most guests the host accepts.
    pub max_guests: u8,
    /// Cap on one reassembled reliable message.
    pub max_message: usize,
    /// Time source.
    pub clock: Clock,
}

impl Default for EndpointConfig {
    fn default() -> Self {
        EndpointConfig {
            name: "player".into(),
            max_guests: 3,
            max_message: framing::DEFAULT_MAX_MESSAGE,
            clock: Clock::system(),
        }
    }
}

#[derive(Debug)]
struct Peer {
    key: PeerKey,
    name: String,
    live: Liveness,
    rel: Reassembler,
    unrel_rx: Option<u32>,
    unrel_tx: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum GuestState {
    Joining {
        started: u64,
        last_sent: Option<u64>,
    },
    Joined,
    Closed,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Role {
    Host,
    Guest,
}

/// A coop-core [`Transport`] over any [`Backend`].
pub struct Endpoint<B: Backend> {
    backend: B,
    role: Role,
    token: Token,
    cfg: EndpointConfig,
    local: Option<PlayerId>,
    peers: HashMap<PlayerId, Peer>,
    by_key: HashMap<PeerKey, PlayerId>,
    inbox: VecDeque<Packet>,
    events: VecDeque<NetEvent>,
    guest: GuestState,
    /// Frames dropped for violating the framing rules.
    pub frame_errors: u64,
}

impl<B: Backend> Endpoint<B> {
    /// A host endpoint. Guests must present `token`.
    pub fn host(backend: B, token: Token, cfg: EndpointConfig) -> Self {
        let mut e = Self::new(backend, Role::Host, token, cfg);
        e.local = Some(PlayerId::HOST);
        e
    }

    /// A guest endpoint. It joins the backend's host peer as soon as the
    /// backend is ready.
    pub fn guest(backend: B, token: Token, cfg: EndpointConfig) -> Self {
        let now = cfg.clock.now();
        let mut e = Self::new(backend, Role::Guest, token, cfg);
        e.guest = GuestState::Joining {
            started: now,
            last_sent: None,
        };
        e
    }

    fn new(backend: B, role: Role, token: Token, cfg: EndpointConfig) -> Self {
        Endpoint {
            backend,
            role,
            token,
            cfg,
            local: None,
            peers: HashMap::new(),
            by_key: HashMap::new(),
            inbox: VecDeque::new(),
            events: VecDeque::new(),
            guest: GuestState::Closed,
            frame_errors: 0,
        }
    }

    /// The underlying backend.
    pub fn backend(&self) -> &B {
        &self.backend
    }
    /// The underlying backend, mutable.
    pub fn backend_mut(&mut self) -> &mut B {
        &mut self.backend
    }

    /// True once this endpoint can exchange game data (host: always; guest: after Welcome).
    pub fn is_connected(&self) -> bool {
        match self.role {
            Role::Host => true,
            Role::Guest => self.guest == GuestState::Joined,
        }
    }

    /// Connected players other than ourselves, sorted.
    pub fn players(&self) -> Vec<PlayerId> {
        let mut v: Vec<_> = self.peers.keys().copied().collect();
        v.sort();
        v
    }

    /// Display name of a peer.
    pub fn name_of(&self, p: PlayerId) -> Option<&str> {
        self.peers.get(&p).map(|x| x.name.as_str())
    }

    /// Round trip time to a peer, if the backend measures it.
    pub fn rtt_ms(&self, p: PlayerId) -> Option<u32> {
        self.peers.get(&p).and_then(|x| self.backend.rtt_ms(x.key))
    }

    /// Drains connection events.
    pub fn events(&mut self) -> Vec<NetEvent> {
        self.pump();
        self.events.drain(..).collect()
    }

    /// Host: removes a guest with a reason the guest will see.
    pub fn kick(&mut self, player: PlayerId, reason: &str) {
        if self.role != Role::Host {
            return;
        }
        self.send_ctl_to(
            player,
            &Ctl::Kick {
                reason: reason.to_string(),
            },
        );
        self.drop_peer(player, LeaveReason::Kicked(reason.to_string()));
    }

    /// Leaves cleanly: tells every peer goodbye and forgets them.
    pub fn leave(&mut self) {
        let all: Vec<_> = self.peers.keys().copied().collect();
        for p in all {
            self.send_ctl_to(p, &Ctl::Goodbye);
            self.remove_quiet(p);
        }
        self.guest = GuestState::Closed;
    }

    /// Guest: starts a fresh join (after a timeout or loss of the host).
    pub fn rejoin(&mut self) {
        if self.role != Role::Guest {
            return;
        }
        let all: Vec<_> = self.peers.keys().copied().collect();
        for p in all {
            self.remove_quiet(p);
        }
        self.local = None;
        self.guest = GuestState::Joining {
            started: self.cfg.clock.now(),
            last_sent: None,
        };
    }

    /// Runs the endpoint: backend poll, frame handling, keep-alives, timeouts.
    pub fn pump(&mut self) {
        let now = self.cfg.clock.now();
        let evs = self.backend.poll(now);
        for ev in evs {
            match ev {
                BackendEvent::Frame { peer, lane, data } => self.on_frame(now, peer, lane, data),
                BackendEvent::PeerGone { peer } => {
                    if let Some(&p) = self.by_key.get(&peer) {
                        self.drop_peer(p, LeaveReason::BackendGone);
                    }
                }
            }
        }
        self.tick(now);
    }

    fn tick(&mut self, now: u64) {
        // Guest join attempts.
        if let GuestState::Joining { started, last_sent } = self.guest {
            match self.backend.readiness() {
                Readiness::Failed(msg) => {
                    self.guest = GuestState::Closed;
                    self.events.push_back(NetEvent::JoinFailed(msg));
                }
                Readiness::Pending => {
                    if now.saturating_sub(started) >= TIMEOUT_MS * 3 {
                        self.guest = GuestState::Closed;
                        self.events
                            .push_back(NetEvent::JoinFailed("setup timed out".into()));
                    }
                }
                Readiness::Ready => {
                    if now.saturating_sub(started) >= TIMEOUT_MS * 3 {
                        self.guest = GuestState::Closed;
                        self.events
                            .push_back(NetEvent::JoinFailed("host did not answer".into()));
                    } else if last_sent.map_or(true, |t| now.saturating_sub(t) >= 1_000) {
                        if let Some(host) = self.backend.host_peer() {
                            let join = Ctl::Join {
                                proto: NET_PROTO,
                                token: self.token,
                                name: self.cfg.name.clone(),
                            };
                            let _ = self.backend.send(host, Lane::Control, &enc(&join));
                            self.guest = GuestState::Joining {
                                started,
                                last_sent: Some(now),
                            };
                        }
                    }
                }
            }
        }
        // Keep-alives and timeouts.
        let mut ka = Vec::new();
        let mut dead = Vec::new();
        for (&p, peer) in self.peers.iter_mut() {
            if peer.live.timed_out(now) {
                dead.push(p);
            } else if peer.live.keepalive_due(now) {
                ka.push(p);
            }
        }
        for p in ka {
            self.send_ctl_to(p, &Ctl::KeepAlive);
        }
        for p in dead {
            self.drop_peer(p, LeaveReason::Timeout);
        }
    }

    fn on_frame(&mut self, now: u64, key: PeerKey, lane: Lane, data: Vec<u8>) {
        let known = self.by_key.get(&key).copied();
        if let Some(p) = known {
            if let Some(peer) = self.peers.get_mut(&p) {
                peer.live.on_rx(now);
            }
        }
        match lane {
            Lane::Control => {
                if let Ok(ctl) = postcard::from_bytes::<Ctl>(&data) {
                    self.on_ctl(now, key, known, ctl);
                }
            }
            Lane::Reliable => {
                let Some(p) = known else { return };
                let res = match self.peers.get_mut(&p) {
                    Some(peer) => peer.rel.push(&data),
                    None => return,
                };
                match res {
                    Ok(Some(msg)) => self.inbox.push_back(Packet {
                        from: p,
                        channel: Channel::ReliableOrdered,
                        data: msg,
                    }),
                    Ok(None) => {}
                    Err(_) => {
                        self.frame_errors += 1;
                        if self.role == Role::Host {
                            self.send_ctl_to(
                                p,
                                &Ctl::Kick {
                                    reason: "protocol error".into(),
                                },
                            );
                        }
                        self.drop_peer(p, LeaveReason::ProtocolError);
                    }
                }
            }
            Lane::Unreliable => {
                let Some(p) = known else { return };
                let Ok((seq, body)) = framing::decode_unreliable(&data) else {
                    self.frame_errors += 1;
                    return;
                };
                if body.len() > MAX_UNRELIABLE {
                    self.frame_errors += 1;
                    return;
                }
                let Some(peer) = self.peers.get_mut(&p) else {
                    return;
                };
                if peer
                    .unrel_rx
                    .map_or(true, |last| framing::seq_newer(seq, last))
                {
                    peer.unrel_rx = Some(seq);
                    self.inbox.push_back(Packet {
                        from: p,
                        channel: Channel::Unreliable,
                        data: body.to_vec(),
                    });
                }
            }
        }
    }

    fn on_ctl(&mut self, now: u64, key: PeerKey, known: Option<PlayerId>, ctl: Ctl) {
        match (self.role, ctl) {
            (Role::Host, Ctl::Join { proto, token, name }) => {
                if let Some(p) = known {
                    // Retransmitted Join: our Welcome was lost.
                    self.send_ctl_to(p, &Ctl::Welcome { slot: p.0 });
                    return;
                }
                let reject = if proto != NET_PROTO {
                    Some(RejectCode::ProtoMismatch)
                } else if token != self.token {
                    Some(RejectCode::BadToken)
                } else if self.peers.len() >= self.cfg.max_guests as usize {
                    Some(RejectCode::Full)
                } else {
                    None
                };
                if let Some(code) = reject {
                    let _ = self
                        .backend
                        .send(key, Lane::Control, &enc(&Ctl::Reject(code)));
                    self.backend.disconnect(key);
                    return;
                }
                let slot = (1..=u8::MAX)
                    .map(PlayerId)
                    .find(|s| !self.peers.contains_key(s));
                let Some(slot) = slot else { return };
                self.peers.insert(
                    slot,
                    Peer {
                        key,
                        name: name.clone(),
                        live: Liveness::new(now),
                        rel: Reassembler::new(self.cfg.max_message),
                        unrel_rx: None,
                        unrel_tx: 0,
                    },
                );
                self.by_key.insert(key, slot);
                self.send_ctl_to(slot, &Ctl::Welcome { slot: slot.0 });
                self.events
                    .push_back(NetEvent::PeerJoined { player: slot, name });
            }
            (Role::Host, Ctl::Goodbye) => {
                if let Some(p) = known {
                    self.drop_peer(p, LeaveReason::Goodbye);
                }
            }
            (Role::Guest, ctl) => {
                if self.backend.host_peer() != Some(key) {
                    return;
                }
                match ctl {
                    Ctl::Welcome { slot } => {
                        if matches!(self.guest, GuestState::Joining { .. }) {
                            self.guest = GuestState::Joined;
                            let me = PlayerId(slot);
                            self.local = Some(me);
                            self.peers.insert(
                                PlayerId::HOST,
                                Peer {
                                    key,
                                    name: "host".into(),
                                    live: Liveness::new(now),
                                    rel: Reassembler::new(self.cfg.max_message),
                                    unrel_rx: None,
                                    unrel_tx: 0,
                                },
                            );
                            self.by_key.insert(key, PlayerId::HOST);
                            self.events.push_back(NetEvent::Connected { local: me });
                        }
                    }
                    Ctl::Reject(code) => {
                        self.guest = GuestState::Closed;
                        self.events.push_back(NetEvent::Rejected(code));
                    }
                    Ctl::Goodbye => self.drop_peer(PlayerId::HOST, LeaveReason::Goodbye),
                    Ctl::Kick { reason } => {
                        self.drop_peer(PlayerId::HOST, LeaveReason::Kicked(reason))
                    }
                    _ => {}
                }
            }
            // KeepAlive only refreshes liveness (done in on_frame).
            _ => {}
        }
    }

    fn send_ctl_to(&mut self, p: PlayerId, ctl: &Ctl) {
        if let Some(peer) = self.peers.get(&p) {
            let key = peer.key;
            let _ = self.backend.send(key, Lane::Control, &enc(ctl));
        }
    }

    fn remove_quiet(&mut self, p: PlayerId) {
        if let Some(peer) = self.peers.remove(&p) {
            self.by_key.remove(&peer.key);
            self.backend.disconnect(peer.key);
        }
    }

    fn drop_peer(&mut self, p: PlayerId, reason: LeaveReason) {
        if self.peers.contains_key(&p) {
            self.remove_quiet(p);
            if self.role == Role::Guest {
                self.guest = GuestState::Closed;
            }
            self.events
                .push_back(NetEvent::PeerLeft { player: p, reason });
        }
    }
}

fn enc(c: &Ctl) -> Vec<u8> {
    postcard::to_stdvec(c).expect("ctl encodes")
}

impl<B: Backend> Transport for Endpoint<B> {
    fn local(&self) -> PlayerId {
        self.local.unwrap_or(PlayerId(u8::MAX))
    }

    fn send(
        &mut self,
        to: PlayerId,
        channel: Channel,
        data: Vec<u8>,
    ) -> Result<(), TransportError> {
        let (key, seq) = match self.peers.get_mut(&to) {
            Some(p) => {
                let s = p.unrel_tx;
                if channel == Channel::Unreliable {
                    p.unrel_tx = p.unrel_tx.wrapping_add(1);
                }
                (p.key, s)
            }
            None => return Err(TransportError::UnknownPeer(to)),
        };
        let res: Result<(), BackendError> = match channel {
            Channel::Unreliable => {
                let frame = framing::encode_unreliable(seq, &data).map_err(|e| match e {
                    FrameError::UnreliableTooLarge { len, max } => {
                        TransportError::TooLarge { len, max }
                    }
                    _ => TransportError::TooLarge {
                        len: data.len(),
                        max: MAX_UNRELIABLE,
                    },
                })?;
                self.backend.send(key, Lane::Unreliable, &frame)
            }
            Channel::ReliableOrdered => {
                if data.len() > self.cfg.max_message {
                    return Err(TransportError::TooLarge {
                        len: data.len(),
                        max: self.cfg.max_message,
                    });
                }
                let mut r = Ok(());
                for f in framing::fragment(&data, MAX_FRAGMENT) {
                    r = self.backend.send(key, Lane::Reliable, &f);
                    if r.is_err() {
                        break;
                    }
                }
                r
            }
        };
        match res {
            Ok(()) => Ok(()),
            Err(BackendError::UnknownPeer(_)) => Err(TransportError::UnknownPeer(to)),
            Err(BackendError::TooLarge { len, max }) => Err(TransportError::TooLarge { len, max }),
            Err(BackendError::Failed(m)) => {
                self.events.push_back(NetEvent::SendError(m));
                Ok(())
            }
        }
    }

    fn recv(&mut self) -> Option<Packet> {
        if self.inbox.is_empty() {
            self.pump();
        }
        self.inbox.pop_front()
    }
}

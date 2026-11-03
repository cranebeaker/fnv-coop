//! The session's link: any [`Transport`], optionally wrapped in the seeded
//! chaos transport (for `--coop-net PROFILE`) on a clock the session moves.

use std::cell::RefCell;
use std::net::{SocketAddr, ToSocketAddrs};
use std::rc::Rc;

use coop_core::ids::PlayerId;
use coop_core::proto::Channel;
use coop_core::transport::{
    ChaosProfile, ChaosTransport, Packet, Transport, TransportError, VirtualClock,
};
use coop_net::{
    Backend, Endpoint, EndpointConfig, JoinCode, JoinTarget as NetTarget, NetEvent, UdpBackend,
    UdpConfig,
};

use crate::{Error, Role};

/// A boxed transport that is itself a [`Transport`].
pub struct DynLink(pub Box<dyn Transport>);

impl Transport for DynLink {
    fn local(&self) -> PlayerId {
        self.0.local()
    }
    fn send(
        &mut self,
        to: PlayerId,
        channel: Channel,
        data: Vec<u8>,
    ) -> Result<(), TransportError> {
        self.0.send(to, channel, data)
    }
    fn recv(&mut self) -> Option<Packet> {
        self.0.recv()
    }
}

/// Plain or chaos wrapped.
pub enum Net {
    /// Straight through.
    Plain(DynLink),
    /// Delayed, lost and reordered as a profile says.
    Chaos(ChaosTransport<DynLink>, VirtualClock),
}

impl Net {
    /// Wraps `link`.
    pub fn new(link: Box<dyn Transport>, chaos: Option<(ChaosProfile, u64)>) -> Net {
        match chaos {
            None => Net::Plain(DynLink(link)),
            Some((profile, seed)) => {
                let clock = VirtualClock::new();
                Net::Chaos(
                    ChaosTransport::new(DynLink(link), clock.clone(), profile, seed),
                    clock,
                )
            }
        }
    }

    /// Moves the chaos clock to `now_ms` and releases what is due.
    pub fn advance(&mut self, now_ms: u64) {
        if let Net::Chaos(t, clock) = self {
            let behind = now_ms.saturating_sub(clock.now());
            clock.advance(behind);
            t.pump();
        }
    }

    /// Releases what is due without moving time.
    pub fn pump(&mut self) {
        if let Net::Chaos(t, _) = self {
            t.pump();
        }
    }

    /// This endpoint's peer id.
    pub fn local(&self) -> PlayerId {
        match self {
            Net::Plain(t) => t.local(),
            Net::Chaos(t, _) => t.local(),
        }
    }

    /// Sends, ignoring a peer that is not there.
    pub fn send(&mut self, to: PlayerId, channel: Channel, data: Vec<u8>) {
        let _ = match self {
            Net::Plain(t) => t.send(to, channel, data),
            Net::Chaos(t, _) => t.send(to, channel, data),
        };
    }

    /// Next received packet.
    pub fn recv(&mut self) -> Option<Packet> {
        match self {
            Net::Plain(t) => t.recv(),
            Net::Chaos(t, _) => t.recv(),
        }
    }
}

/// Errors the real transport reports (join failed, send failed), shared
/// with the session so the viewer can log them.
pub type LinkErrors = Rc<RefCell<Vec<String>>>;

/// The fixed token of test runs. The viewer uses it only with `--coop-test-token`
/// (see [`crate::link`]); real hosts make a random token per session and put it in
/// their join code. [`open`] below (the library default when no transport is given,
/// used by tests that go through `HOST:PORT`) still uses it.
pub const DIRECT_TOKEN: [u8; 16] = *b"nvrs-direct-0001";

/// coop-net's endpoint as a plain transport that also rejoins by itself
/// when a guest loses its host at the connection level.
pub(crate) struct Link<B: Backend> {
    pub(crate) ep: Endpoint<B>,
    pub(crate) guest: bool,
    pub(crate) errors: LinkErrors,
    pub(crate) retry_at: Option<std::time::Instant>,
}

impl<B: Backend> Link<B> {
    pub(crate) fn drain_events(&mut self) {
        for ev in self.ep.events() {
            match ev {
                NetEvent::JoinFailed(m) => {
                    self.errors.borrow_mut().push(format!("join failed: {m}"));
                    self.retry_at =
                        Some(std::time::Instant::now() + std::time::Duration::from_secs(3));
                }
                NetEvent::PeerLeft { player, .. } if self.guest && player.is_host() => {
                    self.retry_at =
                        Some(std::time::Instant::now() + std::time::Duration::from_secs(1));
                }
                NetEvent::Rejected(c) => self
                    .errors
                    .borrow_mut()
                    .push(format!("host refused the join: {c:?}")),
                NetEvent::SendError(m) => self.errors.borrow_mut().push(format!("send error: {m}")),
                _ => {}
            }
        }
        if let Some(t) = self.retry_at {
            if std::time::Instant::now() >= t {
                self.retry_at = None;
                self.ep.rejoin();
            }
        }
    }
}

impl<B: Backend> Transport for Link<B> {
    fn local(&self) -> PlayerId {
        self.ep.local()
    }
    fn send(
        &mut self,
        to: PlayerId,
        channel: Channel,
        data: Vec<u8>,
    ) -> Result<(), TransportError> {
        self.ep.send(to, channel, data)
    }
    fn recv(&mut self) -> Option<Packet> {
        self.drain_events();
        self.ep.recv()
    }
}

fn resolve(addr: &str) -> Result<SocketAddr, Error> {
    addr.to_socket_addrs()
        .map_err(|e| Error::Target(format!("{addr}: {e}")))?
        .next()
        .ok_or_else(|| Error::Target(format!("{addr}: no address")))
}

/// Opens the real UDP transport (coop-net) for `role`: `addr` is a bind
/// port (host: `PORT`, `:PORT`, `ADDR:PORT`) or what a guest was given
/// (`HOST:PORT` or an `NVRS-` join code).
pub fn open(role: Role, name: &str, addr: &str) -> Result<(Box<dyn Transport>, LinkErrors), Error> {
    let errors: LinkErrors = Rc::default();
    let cfg = EndpointConfig {
        name: name.to_string(),
        ..EndpointConfig::default()
    };
    let io = |e: std::io::Error| Error::Io(format!("{addr}: {e}"));
    let ep = match role {
        Role::Host => {
            let port = crate::target::parse_host_port(addr).map_err(Error::Target)?;
            let bind: SocketAddr = SocketAddr::from(([0, 0, 0, 0], port));
            let b = UdpBackend::host(bind, DIRECT_TOKEN, UdpConfig::default()).map_err(io)?;
            Endpoint::host(b, DIRECT_TOKEN, cfg)
        }
        Role::Guest => {
            let (target, token) =
                match crate::target::JoinTarget::parse(addr).map_err(Error::Target)? {
                    crate::target::JoinTarget::Addr(a) => (resolve(&a)?, DIRECT_TOKEN),
                    crate::target::JoinTarget::Packed(c) => {
                        let code: JoinCode =
                            c.parse().map_err(|e| Error::Target(format!("{e}")))?;
                        match code.target {
                            NetTarget::Direct(a) => (a, code.token),
                            _ => return Err(Error::Target(
                                "this join code names a Steam lobby; this build joins by address"
                                    .into(),
                            )),
                        }
                    }
                    crate::target::JoinTarget::Code(c) => {
                        // the join service the environment names resolves it to an NVRS- code
                        let req = crate::link::LinkRequest::new(role, name, addr);
                        let code = crate::link::resolve_short_code(&req, &c)?;
                        match code.target {
                            NetTarget::Direct(a) => (a, code.token),
                            _ => return Err(Error::Target(
                                "this join code names a Steam lobby; this build joins by address"
                                    .into(),
                            )),
                        }
                    }
                };
            let b = UdpBackend::guest(target, token, UdpConfig::default()).map_err(io)?;
            Endpoint::guest(b, token, cfg)
        }
    };
    let link = Link {
        ep,
        guest: role == Role::Guest,
        errors: errors.clone(),
        retry_at: None,
    };
    Ok((Box::new(link), errors))
}

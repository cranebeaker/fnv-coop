//! `coop-net`: real transports behind coop-core's [`Transport`] trait.
//!
//! ```text
//!  coop-core sync  --Transport-->  Endpoint<B>  --Backend-->  UdpBackend | SteamBackend | (future)
//!                                  handshake, slots,          moves frames on 3 lanes
//!                                  fragmentation,
//!                                  keep-alive, timeout
//! ```
//!
//! * [`framing`]: reliable fragmentation (60000 B pieces, 1-byte header) and
//!   unreliable framing (at most 1100 B, sequence numbered).
//! * [`liveness`]: control messages, 1 s keep-alive, 20 s timeout.
//! * [`Endpoint`]: the host/guest handshake (session token, slot assignment,
//!   goodbye, kick) and the coop-core `Transport` impl. Star topology: the
//!   host is `PlayerId(0)`, guests only talk to the host.
//! * [`udp`]: LAN, direct IP and CI backend with its own reliable-ordered channel.
//! * `steam` (cargo feature `steam`, off by default): Steam Spacewar lobbies
//!   and ISteamNetworkingMessages through a runtime-loaded `steam_api64.dll`.
//!   The feature `steam-mock` compiles only the DLL-free part (the `SteamApi`
//!   trait, `MockSteam` and `SteamBackend`) for test rigs.
//! * [`join`]: copy/paste join codes.
//! * [`joinsvc`]: the join service client (six character short codes for
//!   `NVRS-` codes) over `curl.exe` (HTTPS) or plain HTTP.
//!
//! # Extension point: iroh / QUIC
//!
//! Not included on purpose (Windows build size and RAM). A QUIC backend is one
//! more [`Backend`] impl: map `Lane::Reliable` and `Lane::Control` to a single
//! bidirectional stream with length prefixed messages (keep the lane tag),
//! `Lane::Unreliable` to QUIC datagrams (stay under the connection's max
//! datagram size), `PeerKey` to a table index of endpoint ids, and add a
//! `JoinTarget` variant (kind 4 is reserved in [`join`]). `Endpoint` needs no
//! change: it already provides fragmentation, liveness and the handshake.
//!
//! # Fallback from Steam
//!
//! `steam::try_init` returns a typed error when `steam_api64.dll` is missing,
//! Steam is not running, or an export is absent. Callers then build a
//! [`UdpBackend`] instead; both plug into the same [`Endpoint`].

#![warn(missing_docs)]

pub mod backend;
pub mod clock;
pub mod endpoint;
pub mod framing;
pub mod join;
pub mod joinsvc;
pub mod liveness;
pub mod mem;
pub mod udp;

#[cfg(feature = "steam-mock")]
pub mod steam;

pub use backend::{Backend, BackendError, BackendEvent, Lane, PeerKey, Readiness};
pub use clock::{Clock, ManualClock};
pub use endpoint::{Endpoint, EndpointConfig, LeaveReason, NetEvent, Token};
pub use join::{random_token, JoinCode, JoinError, JoinTarget};
pub use liveness::{Ctl, RejectCode};
pub use udp::{ChaosCfg, UdpBackend, UdpConfig};

pub use coop_core::transport::Transport;

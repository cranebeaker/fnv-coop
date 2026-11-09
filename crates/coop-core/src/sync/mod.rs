//! The replication protocol state machines (pure: messages in, messages out).
//!
//! [`HostSync`] owns the authoritative store; [`GuestSync`] holds a replica.
//! Neither touches a socket: a driver encodes the returned messages and
//! feeds received ones back in, which is what makes them testable under the
//! chaos transport.

mod glive;
mod guest;
mod host;
mod live;

pub use glive::{ChatLine, GuestLive, RemoteAvatar, CHAT_LOG_CAP};
pub use guest::{GuestPhase, GuestSync};
pub use host::{CompatNote, HostConfig, HostSync};
pub use live::HostLive;

/// Ticks of silence before the guest restarts a handshake. Hello and Join
/// travel on the reliable channel, so this only covers a dead connection.
pub const RETRY_TICKS: u32 = 400;

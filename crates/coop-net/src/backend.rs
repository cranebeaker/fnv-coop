//! The byte-mover under an [`crate::Endpoint`]. A backend delivers frames to
//! numbered peers on three lanes; everything above (fragmentation, liveness,
//! handshake, the coop-core `Transport` impl) is shared and backend-agnostic.

use thiserror::Error;

/// Opaque per-backend peer handle (SteamID64 for Steam, a table index for UDP).
pub type PeerKey = u64;

/// Message lanes. The numbers are the Steam message channels.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Lane {
    /// Game data, reliable and ordered (fragmented above 60000 bytes).
    Reliable = 0,
    /// Handshake, keep-alive, goodbye, kick. Reliable and ordered.
    Control = 1,
    /// Positions and other disposable data, at most 1100 bytes.
    Unreliable = 2,
}

impl Lane {
    /// Lane for a channel number.
    pub fn from_u8(v: u8) -> Option<Lane> {
        match v {
            0 => Some(Lane::Reliable),
            1 => Some(Lane::Control),
            2 => Some(Lane::Unreliable),
            _ => None,
        }
    }
}

/// Something a backend saw.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BackendEvent {
    /// A frame arrived from `peer`.
    Frame {
        /// Sender.
        peer: PeerKey,
        /// Lane it arrived on.
        lane: Lane,
        /// Payload.
        data: Vec<u8>,
    },
    /// The backend knows `peer` is gone (close packet, lobby leave, session failure).
    PeerGone {
        /// The departed peer.
        peer: PeerKey,
    },
}

/// Whether the backend can carry traffic yet.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Readiness {
    /// Still setting up (creating or joining a lobby, ...).
    Pending,
    /// Ready to send.
    Ready,
    /// Setup failed; the text is user-facing.
    Failed(String),
}

/// Backend send failures.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum BackendError {
    /// Peer unknown to the backend.
    #[error("unknown peer {0}")]
    UnknownPeer(PeerKey),
    /// Payload over the lane limit.
    #[error("payload of {len} bytes exceeds {max}")]
    TooLarge {
        /// Payload size.
        len: usize,
        /// Limit.
        max: usize,
    },
    /// Send failed for another reason.
    #[error("send failed: {0}")]
    Failed(String),
}

/// A frame mover.
pub trait Backend {
    /// Sends one frame. Reliable lanes never drop silently; they queue or error.
    fn send(&mut self, peer: PeerKey, lane: Lane, data: &[u8]) -> Result<(), BackendError>;
    /// Drives the backend and returns what happened since the last call.
    fn poll(&mut self, now_ms: u64) -> Vec<BackendEvent>;
    /// Setup state. Defaults to ready.
    fn readiness(&self) -> Readiness {
        Readiness::Ready
    }
    /// The peer a guest must talk to, once known.
    fn host_peer(&self) -> Option<PeerKey>;
    /// Forgets a peer (and tells it, best effort).
    fn disconnect(&mut self, peer: PeerKey);
    /// Smoothed round trip time, if the backend measures it.
    fn rtt_ms(&self, _peer: PeerKey) -> Option<u32> {
        None
    }
}

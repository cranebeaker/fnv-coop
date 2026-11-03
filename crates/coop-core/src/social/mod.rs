//! Social channels: chat, map markers and pings, push-to-talk voice.

mod chat;
pub mod compass;
mod markers;
mod voice;

use serde::{Deserialize, Serialize};

use crate::ids::{CellId, PlayerId};

pub use chat::{sanitize, ChatHost, ChatOut};
pub use markers::{MarkerOut, MarkerTable};
pub use voice::{VoiceHub, VoiceRx};

/// Longest chat message in bytes.
pub const MAX_CHAT_BYTES: usize = 160;
/// Longest marker label in bytes.
pub const MAX_LABEL_BYTES: usize = 32;
/// Largest opaque voice payload.
pub const MAX_VOICE_PAYLOAD: usize = 480;

/// Where a chat line goes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChatChannel {
    /// Everyone.
    All,
    /// One player (and the sender).
    Whisper(PlayerId),
    /// Host announcements; guests cannot send on it.
    System,
}

/// Why a chat line was refused.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChatReject {
    /// Too many lines recently.
    RateLimited,
    /// Nothing left after sanitizing, or a bad channel.
    Invalid,
}

/// A map marker or ping.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Marker {
    /// Owner-local id.
    pub id: u16,
    /// Game-defined kind (ping, waypoint, danger ...).
    pub kind: u8,
    /// Cell.
    pub cell: CellId,
    /// Position, 1/16 unit fixed point.
    pub pos: [i32; 3],
    /// Short label.
    pub label: String,
    /// Lifetime in ms (0 = until cleared). Pings use a short one.
    pub ttl_ms: u32,
}

/// One voice packet: an opaque codec frame with its envelope.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct VoicePacket {
    /// Who is speaking; must equal the transport sender.
    pub speaker: PlayerId,
    /// Frame sequence number (wraps).
    pub seq: u16,
    /// Codec payload (opaque to this crate).
    pub payload: Vec<u8>,
}

/// Social wire messages.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SocialMsg {
    /// Guest says a line.
    Say {
        /// Per-sender id, deduplicated.
        msg_id: u32,
        /// Channel.
        channel: ChatChannel,
        /// Text.
        text: String,
    },
    /// Host: a line to show.
    Line {
        /// Speaker (`None` = system).
        from: Option<PlayerId>,
        /// Display name chosen by the host.
        name: String,
        /// Channel.
        channel: ChatChannel,
        /// Sanitized text.
        text: String,
    },
    /// Host: your line was refused.
    Rejected {
        /// The refused `msg_id`.
        msg_id: u32,
        /// Why.
        reason: ChatReject,
    },
    /// Player sets or replaces one of their markers.
    MarkerSet {
        /// The marker.
        marker: Marker,
    },
    /// Player clears one of their markers.
    MarkerClear {
        /// Owner-local id.
        id: u16,
    },
    /// Host: a player's marker (new, changed, or in a join snapshot).
    MarkerShown {
        /// Owner.
        owner: PlayerId,
        /// The marker.
        marker: Marker,
    },
    /// Host: a marker went away (cleared or expired).
    MarkerGone {
        /// Owner.
        owner: PlayerId,
        /// Id.
        id: u16,
    },
    /// Voice frame (unreliable): guest to host, host to the others.
    Voice(VoicePacket),
}

impl SocialMsg {
    /// True for the unreliable voice message.
    pub fn is_stream(&self) -> bool {
        matches!(self, SocialMsg::Voice(_))
    }

    /// Structural bounds.
    pub fn is_sane(&self) -> bool {
        match self {
            SocialMsg::Say { text, .. } | SocialMsg::Line { text, .. } => {
                text.len() <= MAX_CHAT_BYTES * 4
            }
            SocialMsg::MarkerSet { marker } | SocialMsg::MarkerShown { marker, .. } => {
                marker.label.len() <= MAX_LABEL_BYTES
            }
            SocialMsg::Voice(v) => v.payload.len() <= MAX_VOICE_PAYLOAD,
            _ => true,
        }
    }
}

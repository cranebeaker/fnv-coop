//! `coop-session`: the headless driver that makes a `GameState` a co-op peer.
//!
//! No Bevy. A viewer (or a test) owns one [`Session`] and calls, once per
//! frame:
//!
//! 1. [`Session::pre_frame`] at the start (network receive; on a guest,
//!    apply the replicated state to the `GameState`);
//! 2. [`Session::post_frame`] at the end (on the host, mirror the frame's
//!    changes into the replicated store; run the protocol clock; send).
//!
//! The session owns the transport, coop-core's `HostSync` / `GuestSync` and
//! the world crate's `HostBridge` / `GuestBridge`. It also carries a few
//! session-level messages that are not replicated state (see [`ext`]): the
//! host's place (so a guest loads the same cell), script commands from
//! guests, names and ping figures.
//!
//! Output for tests and tools: a JSONL event log and a state dump file
//! (see [`SessionConfig::log`], [`SessionConfig::fields_dump`]).

#![warn(missing_docs)]

pub mod control;
pub mod dump;
pub mod ext;
pub mod json;
pub mod link;
pub mod live;
mod live_loot;
mod live_roam;
mod live_combat;
pub mod net;
pub mod out;
pub mod policy_ids;
pub mod party;
#[cfg(feature = "steam")]
pub mod steam_transport;
mod session;
pub mod start;
pub mod live_extra;
pub mod target;

pub use coop_core::transport::ChaosProfile;
pub use coop_core::avatar::{ActionKind, AvatarAction, FLAG_DRAWN, FLAG_SNEAK};
pub use coop_core::ids::PlayerId;
pub use coop_core::interact;
pub use coop_core::social::compass;
pub use coop_core::compat;
pub use coop_core::rules::{Prompt, VoteKind};
pub use coop_core::scene::SceneKind;
pub use ext::{Command, PeerInfo, Place};
pub use dump::LiveCounters;
pub use live_extra::VoiceFrame;
pub use live_roam::RoamView;
pub use party::PartyEvent;
pub use live::{ActorRow, ActorView, SpawnedView, ChatView, DialogueView, Intent, LiveFlags, LiveView, MarkerView, RemoteAvatarView};
pub use session::{CompatConfig, Event, Session, SessionConfig, State, Status};
pub use start::{Action as StartAction, OwnCharacter, Opening, Phase, Sheet, StartConfig, StartFlow, StartInfo, Step, Venue};
pub use target::{JoinTarget, DEFAULT_PORT};

/// Which side of the session this process is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Role {
    /// Runs the rules, owns the story.
    Host,
    /// Follows the host.
    Guest,
}

/// Session errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// No transport was supplied and none is linked into this build.
    NoTransport,
    /// The address or code was not understood.
    Target(String),
    /// A file could not be opened.
    Io(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NoTransport => write!(f, "this build has no network transport"),
            Error::Target(s) => write!(f, "{s}"),
            Error::Io(s) => write!(f, "{s}"),
        }
    }
}

impl std::error::Error for Error {}

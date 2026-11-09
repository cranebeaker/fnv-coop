//! Dialogue session protocol. The initiator picks, the host runs each result
//! script once, everyone else spectates behind a per-step barrier.
//!
//! Host and client are pure state machines. Result scripts are not run here:
//! the host emits [`DialogueEvent`]s for the game to act on.

mod client;
mod host;

use serde::{Deserialize, Serialize};

use crate::ids::{FormId, PlayerId};

pub use client::{DialogueClient, View};
pub use host::{DOut, DialogueHost};

/// Initiator heartbeat period.
pub const HEARTBEAT_MS: u64 = 15_000;
/// Silence after which the initiator counts as gone.
pub const HEARTBEAT_MISS_MS: u64 = 45_000;
/// A session with no pick for this long is closed.
pub const LEASE_MS: u64 = 300_000;
/// Longest a step waits for spectators to finish their lines.
pub const LINE_TIMEOUT_MS: u64 = 30_000;

/// Why an open was refused.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Deny {
    /// Someone else is talking to this NPC.
    Busy,
    /// The requester already has a session.
    AlreadyTalking,
    /// The NPC is dead.
    NpcDead,
    /// The NPC is in combat.
    NpcInCombat,
}

/// Why a session ended.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CloseReason {
    /// The initiator ended it.
    Finished,
    /// The initiator disconnected.
    InitiatorLeft,
    /// No heartbeat from the initiator.
    HeartbeatLost,
    /// No pick within the lease.
    LeaseExpired,
    /// The NPC died.
    NpcDied,
}

/// Dialogue wire messages (all reliable).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DialogueMsg {
    /// Initiator asks to talk to an NPC.
    Open {
        /// Request id.
        req: u32,
        /// The NPC.
        npc: FormId,
    },
    /// Host refuses an open.
    Denied {
        /// Request id.
        req: u32,
        /// Reason.
        reason: Deny,
    },
    /// Host: a session began (initiator and spectators).
    Started {
        /// Session id.
        session: u32,
        /// The NPC.
        npc: FormId,
        /// Who picks.
        initiator: PlayerId,
        /// Request id this answers (meaningful to the initiator).
        req: u32,
    },
    /// Initiator picks a choice for the next step.
    Pick {
        /// Session.
        session: u32,
        /// Step number, strictly increasing from 1.
        step: u32,
        /// Choice id (topic info).
        choice: u32,
    },
    /// Host: step accepted, everyone plays the lines.
    Picked {
        /// Session.
        session: u32,
        /// Step.
        step: u32,
        /// Choice.
        choice: u32,
    },
    /// A peer finished playing a step's lines.
    LineDone {
        /// Session.
        session: u32,
        /// Step.
        step: u32,
    },
    /// Host: barrier passed and the end script ran.
    StepEnd {
        /// Session.
        session: u32,
        /// Step.
        step: u32,
    },
    /// Initiator keep-alive.
    Heartbeat {
        /// Session.
        session: u32,
    },
    /// Initiator ends the conversation.
    Close {
        /// Session.
        session: u32,
    },
    /// Host: the session is over.
    Closed {
        /// Session.
        session: u32,
        /// Why.
        reason: CloseReason,
    },
    /// Host to the picker: the game refused the pick (the line's conditions
    /// fail for the picker). The step stays open.
    Refused {
        /// Session.
        session: u32,
        /// The step the pick asked for.
        step: u32,
        /// The refused choice.
        choice: u32,
    },
}

/// What the game must do on the host.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DialogueEvent {
    /// Run the begin script of `choice` (once).
    RunBegin {
        /// Session.
        session: u32,
        /// NPC.
        npc: FormId,
        /// Step.
        step: u32,
        /// Choice.
        choice: u32,
        /// Who picked.
        initiator: PlayerId,
    },
    /// Run the end script of the step's choice (once).
    RunEnd {
        /// Session.
        session: u32,
        /// NPC.
        npc: FormId,
        /// Step.
        step: u32,
        /// Choice.
        choice: u32,
    },
    /// Release the NPC and the control locks.
    Closed {
        /// Session.
        session: u32,
        /// NPC.
        npc: FormId,
        /// Why.
        reason: CloseReason,
    },
}

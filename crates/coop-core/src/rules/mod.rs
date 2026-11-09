//! Session rules: story tether, unanimous barrier, world-time consent,
//! downed and revive, difficulty scaling, and shared-stat policy.

pub mod barrier;
mod client;
pub mod difficulty;
mod host;
pub mod policy;
pub mod revive;
pub mod tether;
pub mod vote;

use serde::{Deserialize, Serialize};

use crate::ids::PlayerId;

pub use barrier::{Barrier, BarrierState, CancelReason};
pub use client::{Notice, Prompt, RulesClient};
pub use difficulty::{Mode, Ramp, Scaling};
pub use host::{ROut, RulesEvent, RulesHost};
pub use policy::{PolicyTable, RewardLedger, StatKind, StatPolicy};
pub use revive::{ReviveDeny, ReviveEvent, Reviver};
pub use tether::{Tether, TetherEvent};
pub use vote::{Proposal, VoteHost, VoteKind, VoteOutcome};

/// Rules wire messages (all reliable).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RulesMsg {
    /// Host: you will be warped to the tether anchor soon.
    TetherWarn {
        /// Milliseconds until the warp.
        in_ms: u32,
    },
    /// Host: warp to the tether anchor now.
    TetherWarp {
        /// Anchor id.
        anchor: u32,
    },
    /// Host: a barrier opened; every listed player must agree.
    BarrierOpen {
        /// Barrier id.
        id: u32,
        /// Game-defined transition tag.
        tag: u32,
        /// Required players.
        required: Vec<PlayerId>,
    },
    /// Player agrees to the barrier.
    BarrierReady {
        /// Barrier id.
        id: u32,
    },
    /// Player refuses the barrier.
    BarrierDecline {
        /// Barrier id.
        id: u32,
    },
    /// Host: barrier outcome.
    BarrierResult {
        /// Barrier id.
        id: u32,
        /// Tag.
        tag: u32,
        /// True when everyone agreed (warp together).
        released: bool,
        /// Why not, when cancelled.
        reason: Option<CancelReason>,
    },
    /// Player proposes a vote (sleep, wait, fast travel, fork).
    VoteRequest {
        /// What.
        kind: VoteKind,
        /// Game-defined parameter.
        param: u32,
    },
    /// Host: a vote opened.
    VoteOpen {
        /// Vote id.
        id: u32,
        /// What.
        kind: VoteKind,
        /// Parameter.
        param: u32,
        /// Proposer.
        proposer: PlayerId,
    },
    /// Player casts a vote.
    VoteCast {
        /// Vote id.
        id: u32,
        /// Yes or no.
        yes: bool,
    },
    /// Host: vote outcome (id 0 = no vote was needed).
    VoteResult {
        /// Vote id.
        id: u32,
        /// What.
        kind: VoteKind,
        /// Parameter.
        param: u32,
        /// Approved or not.
        approved: bool,
    },
    /// Player reports their health hit zero.
    ReportDown {
        /// False for deaths that cannot be revived.
        revivable: bool,
    },
    /// Helper starts holding the revive key next to a downed player.
    ReviveBegin {
        /// Downed player.
        target: PlayerId,
    },
    /// Helper let go.
    ReviveCancel,
    /// Host: a revive attempt was refused.
    ReviveDenied {
        /// Target.
        target: PlayerId,
        /// Why.
        reason: ReviveDeny,
    },
    /// Host: a revive attempt started or ended (UI progress ring).
    ReviveProgress {
        /// Target.
        target: PlayerId,
        /// Helper, or none when the attempt ended.
        helper: Option<PlayerId>,
    },
    /// Host: a player is down.
    Downed {
        /// Player.
        player: PlayerId,
    },
    /// Host: a player was revived in place.
    Revived {
        /// Player.
        player: PlayerId,
        /// Helper.
        helper: PlayerId,
        /// Health to restore, percent.
        health_pct: u8,
    },
    /// Host: respawn this player at the party checkpoint.
    Respawn {
        /// Player.
        player: PlayerId,
    },
    /// Host: everyone is down.
    Wipe,
    /// Host: difficulty scaling for the current party size.
    ScalingSet {
        /// Party size.
        players: u8,
        /// Campaign mode; clients derive the factors with
        /// [`Scaling::for_party`].
        mode: Mode,
        /// The host's friendly-fire choice (the combat vitals apply it).
        friendly_fire: bool,
    },
}

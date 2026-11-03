//! NPC and actor replication: one owner per actor with an epoch, a state
//! stream from the owner, host-resolved damage and death, and spawn
//! arbitration.

mod client;
mod host;
mod ownership;
mod spawn;

use serde::{Deserialize, Serialize};

use crate::ids::{CellId, Epoch, FormId, PlayerId, Seq};

pub use client::{ActorClient, RemoteActor};
pub use host::{ActorHost, Out};
pub use ownership::{OwnerChange, Ownership, PlayerLoc, StateVerdict};
pub use spawn::{DenyReason, SpawnArbiter, SpawnOutcome, SpawnRec, DYN_ID_BASE};

/// Largest damage in one hit request.
pub const MAX_HIT: u16 = 5000;
/// Most actors in one `Track` report.
pub const MAX_TRACK: usize = 128;
/// Most hit requests per attacker per second.
pub const MAX_HITS_PER_SEC: usize = 30;

/// What the owner publishes about an actor.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActorState {
    /// Cell the actor stands in.
    pub cell: CellId,
    /// Position, 1/16 unit fixed point.
    pub pos: [i32; 3],
    /// Heading, full circle in 65536 steps.
    pub heading: u16,
    /// Animation state id.
    pub anim: u16,
    /// Health points. On the host the damage ledger overrides this.
    pub health: u16,
    /// Current target form (0 = none).
    pub target: u32,
    /// Flag bits.
    pub flags: u16,
}

/// Why a spawn or other request failed, on the wire.
pub type Reason = DenyReason;

/// Actor wire messages.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActorMsg {
    /// Owner stream (unreliable): guest to host, host to interested peers.
    State {
        /// Actor.
        actor: FormId,
        /// Ownership epoch the sender believes it holds.
        epoch: Epoch,
        /// Per-epoch sample number.
        seq: Seq,
        /// The state.
        state: ActorState,
    },
    /// Host announces an ownership change (snapshot first: carries the last
    /// state so the new owner starts exactly where the old one stopped).
    Assign {
        /// Actor.
        actor: FormId,
        /// New owner (`None` = dormant, host holds it).
        owner: Option<PlayerId>,
        /// New epoch.
        epoch: Epoch,
        /// Last accepted state.
        seed: Option<ActorState>,
    },
    /// Attacker asks the host to apply damage.
    Hit {
        /// Victim.
        actor: FormId,
        /// Per-attacker hit id (deduplicated).
        hit: u32,
        /// Damage points.
        amount: u16,
        /// An NPC's blow (Doc Mitchell on a gecko): no player and no follower
        /// struck it. The host applies it but credits no player.
        npc: bool,
    },
    /// Host: damage applied.
    Damaged {
        /// Victim.
        actor: FormId,
        /// The player who struck. `None`: an NPC's blow, which pays no player
        /// and adds no threat to anyone.
        by: Option<PlayerId>,
        /// Points applied.
        amount: u16,
        /// Health afterwards.
        health: u16,
    },
    /// Host: the actor died (sent exactly once).
    Died {
        /// Victim.
        actor: FormId,
        /// Killer if any.
        killer: Option<PlayerId>,
    },
    /// Ask for permission to spawn at a site.
    SpawnReq {
        /// Per-requester id (deduplicated).
        req: u32,
        /// Spawn site form.
        site: FormId,
        /// Base form to create.
        base: FormId,
        /// Cell of the site.
        cell: CellId,
        /// Where to create it, 1/16 unit fixed point.
        pos: [i32; 3],
        /// Starting health.
        health: u16,
    },
    /// Host verdict on a spawn request.
    SpawnReply {
        /// Request id.
        req: u32,
        /// Result.
        outcome: SpawnOutcome,
    },
    /// Host: a dynamic ref now exists.
    Spawned {
        /// Host-allocated id.
        id: FormId,
        /// Base form.
        base: FormId,
        /// Site.
        site: FormId,
        /// Cell.
        cell: CellId,
        /// Position, 1/16 unit fixed point.
        pos: [i32; 3],
    },
    /// A peer reports actors it has loaded, so the host tracks them too
    /// (a creature found in a place only that peer has loaded).
    Track {
        /// Actor and its state as that peer sees it.
        actors: Vec<(FormId, ActorState)>,
    },
    /// Host: a dynamic ref is gone.
    Despawn {
        /// Id.
        id: FormId,
    },
    /// A peer proposes a leveled pick; the host keeps the first per key.
    LeveledReq {
        /// Request id.
        req: u32,
        /// Leveled list form.
        list: FormId,
        /// Roll key (for example the spawn site or container).
        key: u32,
        /// The requester's own pick.
        pick: FormId,
    },
    /// Host: the one true pick.
    LeveledReply {
        /// Request id.
        req: u32,
        /// List.
        list: FormId,
        /// Roll key.
        key: u32,
        /// Winning pick.
        pick: FormId,
    },
    /// Host: something on the host's world raised the actor's health (a
    /// script's `ForceActorValue Health`, a heal). Only the host sends it.
    Healed {
        /// Actor.
        actor: FormId,
        /// Health afterwards.
        health: u16,
    },
}

impl ActorMsg {
    /// True for the unreliable stream message.
    pub fn is_stream(&self) -> bool {
        matches!(self, ActorMsg::State { .. })
    }

    /// Structural bounds.
    pub fn is_sane(&self) -> bool {
        match self {
            ActorMsg::Hit { amount, .. } => *amount <= MAX_HIT,
            ActorMsg::Track { actors } => actors.len() <= MAX_TRACK,
            _ => true,
        }
    }
}

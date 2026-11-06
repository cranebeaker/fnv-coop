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
        /// Damage points. 0: a player's blow under one point (the sender
        /// keeps the part points). The host's friendly-fire guard counts it,
        /// and the ledger takes no health for it.
        amount: u16,
        /// An NPC's blow (Doc Mitchell on a gecko): the NPC that struck. No
        /// player and no follower struck it. The host applies it but credits
        /// no player. A guest's NPC blow counts only when the host's ledger
        /// tracks that NPC alive near the victim, and the guest owns that NPC
        /// or the victim. `None`: a blow of the sender's player or follower.
        npc: Option<FormId>,
        /// What the sender's player or followers did in each blow of this
        /// hit. The host computes the damage of these blows from its own copy
        /// of the sender and does not use `amount` for them. Empty: damage
        /// with no blow (an explosion, an effect), or an NPC's blow.
        blows: Vec<Blow>,
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
    /// Host to the striker only: the friendly-fire guard dropped its blow
    /// on `actor`. The striker forgets the part point it kept for `actor`,
    /// so the dropped blow adds nothing to a later blow.
    HitBlocked {
        /// Victim.
        actor: FormId,
        /// The dropped blow was an NPC's blow (see [`ActorMsg::Hit`]).
        npc: bool,
    },
}

/// The most blows one hit message names (a frame's pellets on one victim).
pub const MAX_BLOWS: usize = 64;

/// What a player or a follower did in one blow. The host computes the blow's
/// damage from this and from its own copy of the striking player.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Blow {
    /// The follower that struck. `None`: the player.
    pub by: Option<FormId>,
    /// The weapon in hand. `None`: fists, or the creature's own attack.
    pub weapon: Option<FormId>,
    /// The body part type hit. `None`: no part found.
    pub part: Option<u8>,
    /// [`Blow::POWER`], [`Blow::SNEAK`] and [`Blow::CRITICAL`].
    pub flags: u8,
    /// The V.A.T.S. attack kind, when a queued V.A.T.S. hit made the blow.
    pub vats: Option<u8>,
    /// The explosion, when the weapon's explosion made the blow.
    pub blast: Option<BlastBlow>,
}

/// The explosion of a [`Blow`]: which explosion went off, and where.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlastBlow {
    /// The explosion (`EXPL`).
    pub explosion: FormId,
    /// Its centre, quantized ([`crate::avatar::quant_pos`]).
    pub at: [i32; 3],
}

impl Blow {
    /// A power attack.
    pub const POWER: u8 = 1;
    /// A sneak attack (the target did not detect the striker).
    pub const SNEAK: u8 = 2;
    /// A critical hit (the striker's roll).
    pub const CRITICAL: u8 = 4;

    /// Whether the blow has `flag`.
    pub fn has(&self, flag: u8) -> bool {
        self.flags & flag != 0
    }
}

impl ActorMsg {
    /// True for the unreliable stream message.
    pub fn is_stream(&self) -> bool {
        matches!(self, ActorMsg::State { .. })
    }

    /// Structural bounds.
    pub fn is_sane(&self) -> bool {
        match self {
            ActorMsg::Hit { amount, blows, .. } => *amount <= MAX_HIT && blows.len() <= MAX_BLOWS,
            ActorMsg::Track { actors } => actors.len() <= MAX_TRACK,
            _ => true,
        }
    }
}

//! Combat depth for co-op: NPC damage to remote players, per-player vitals,
//! threat splitting, VATS in real time, area blasts, weapon-fire relay and
//! encounter extras.
//!
//! Pure: messages in, addressed messages out. The host is the only judge:
//!
//! * [`VitalsHost`]: a shadow of every player's health and ailments (health,
//!   downed, radiation, poison, bleeding, crippled limbs). Damage that
//!   reaches a player is computed here, once, and sent to the player's own
//!   machine as [`CombatMsg::PlayerHurt`] / [`CombatMsg::PlayerEffect`].
//! * [`ThreatDirector`]: which player an enemy attends to.
//! * [`FireHub`]: validates and relays shots and explosions.
//! * [`VatsHost`]: resolves real-time VATS shots with the actor's own AP.
//! * [`ExtrasPlanner`]: deterministic encounter extras.
//!
//! [`CombatHost`] glues them behind one `on_msg`; [`CombatClient`] is what
//! every other machine keeps.

mod client;
mod extras;
mod fire;
mod host;
mod threat;
mod vats;
mod vitals;

#[cfg(test)]
mod tests;

use serde::{Deserialize, Serialize};

use crate::ids::{CellId, FormId, PlayerId};

pub use client::{CombatClient, EffectNote, HurtNote};
pub use extras::{extra_site, ExtraPlan, ExtrasPlanner, CLEAR_DISTANCE};
pub use fire::{FireHub, MAX_SHOT_RANGE};
pub use host::{Attribution, COut, CombatHost, VatsHit};
pub use threat::{Threat, ThreatDirector, ThreatInput};
pub use vats::{vats_scales, TimeScales, VatsHost, VatsLimits};
pub use vitals::{FriendlyFire, Vitals, VitalsHost, VitalsStats, LIMB_MAX_DAMAGE};

/// Largest damage in one request.
pub const MAX_PLAYER_HIT: u16 = 5000;
/// Most requests a peer may send per second (each kind).
pub const MAX_REQ_PER_SEC: usize = 40;
/// Most shots in one VATS queue.
pub const MAX_VATS_SHOTS: usize = 8;
/// Limbs: head, torso, left arm, right arm, left leg, right leg, weapon.
pub const LIMB_COUNT: u8 = 7;

/// What kind of harm.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DamageKind {
    /// Bullet.
    Bullet,
    /// Blade, fist or club.
    Melee,
    /// Energy weapon.
    Energy,
    /// Fire.
    Fire,
    /// Explosion.
    Explosion,
    /// Mine.
    Mine,
    /// Thrown weapon.
    Thrown,
    /// Poison over time.
    Poison,
    /// Bleeding over time.
    Bleed,
    /// Fall.
    Fall,
}

impl DamageKind {
    /// Damage that arrives in one blow (armor and limbs matter) as opposed
    /// to over time.
    pub fn is_direct(self) -> bool {
        !matches!(self, DamageKind::Poison | DamageKind::Bleed)
    }
}

/// A lasting effect on a player.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EffectKind {
    /// Radiation dose (amount is rads).
    Radiation,
    /// Poison for `ms` at `amount` points a second.
    Poison,
    /// Bleeding on a limb for `ms`.
    Bleed,
    /// A limb is crippled (`ms` 0: until healed or revived).
    Cripple,
    /// Knocked down for `ms`.
    Knockdown,
    /// All ailments cleared (doctor, stimpak, revive).
    Cure,
}

/// A shot, as relayed so other machines show the impact and the sound.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FireEvent {
    /// Shooter (filled by the host on relay).
    pub shooter: PlayerId,
    /// Per-shooter id (deduplicated).
    pub id: u32,
    /// Weapon form.
    pub weapon: FormId,
    /// Cell.
    pub cell: CellId,
    /// Muzzle, 1/16 unit fixed point.
    pub origin: [i32; 3],
    /// Where the shot landed, 1/16 unit fixed point.
    pub hit: [i32; 3],
    /// Who was hit (0: the world).
    pub target: FormId,
    /// Limb hit (`LIMB_COUNT` or more: none).
    pub limb: u8,
    /// Surface material id for decals and sounds.
    pub surface: u8,
}

/// An explosion, grenade, mine or thrown weapon going off.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlastEvent {
    /// Who set it off (filled by the host on relay).
    pub source: PlayerId,
    /// Per-source id (deduplicated).
    pub id: u32,
    /// The `EXPL` record.
    pub explosion: FormId,
    /// The projectile, weapon or mine that made it (0 unknown).
    pub origin: FormId,
    /// Cell.
    pub cell: CellId,
    /// Centre, 1/16 unit fixed point.
    pub pos: [i32; 3],
    /// Damage at the centre.
    pub damage: u16,
    /// Radius in game units.
    pub radius: u16,
    /// What it was.
    pub kind: DamageKind,
}

/// One VATS shot of a queue.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct VatsShot {
    /// Target actor.
    pub target: FormId,
    /// Limb aimed at.
    pub limb: u8,
    /// AP this shot costs.
    pub cost: u16,
    /// Chance to hit, per mille (the shooter's own number: skill, perks,
    /// distance). The host clamps it and rolls.
    pub chance: u16,
    /// Damage if it hits (before enemy toughness).
    pub damage: u16,
}

/// What came of one VATS shot.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct VatsResult {
    /// The shot hit.
    pub hit: bool,
    /// The shot was paid for and fired (false: no AP left).
    pub fired: bool,
    /// Damage dealt (after toughness).
    pub damage: u16,
    /// Limb.
    pub limb: u8,
    /// Critical.
    pub critical: bool,
}

/// Combat wire messages (all reliable).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CombatMsg {
    /// A peer whose NPC hurt a player asks the host to apply it.
    NpcHit {
        /// Request id (per sender).
        id: u32,
        /// The NPC.
        attacker: FormId,
        /// The player hurt.
        victim: PlayerId,
        /// Points before the party scaling.
        amount: u16,
        /// Kind.
        kind: DamageKind,
        /// Limb.
        limb: u8,
    },
    /// A player's shot hit another player (friendly-fire check on the host).
    PlayerHit {
        /// Request id (per sender).
        id: u32,
        /// The victim.
        victim: PlayerId,
        /// Points.
        amount: u16,
        /// Kind.
        kind: DamageKind,
        /// Limb.
        limb: u8,
    },
    /// Host to the victim's machine: apply this to your health.
    PlayerHurt {
        /// Victim.
        victim: PlayerId,
        /// Host sequence (deduplicated by the receiver).
        seq: u32,
        /// Points after the party scaling.
        amount: u16,
        /// Kind.
        kind: DamageKind,
        /// Limb.
        limb: u8,
        /// Who did it (an NPC, or a puppet form of a player; 0 unknown).
        attacker: FormId,
        /// Host's health for the victim afterwards.
        health: u16,
    },
    /// Host to the victim's machine: a lasting effect.
    PlayerEffect {
        /// Victim.
        victim: PlayerId,
        /// Host sequence.
        seq: u32,
        /// What.
        effect: EffectKind,
        /// Amount (rads, points a second).
        amount: u16,
        /// Limb.
        limb: u8,
        /// Duration in ms (0: until cured).
        ms: u32,
    },
    /// A shot to show on the other machines.
    Fire(FireEvent),
    /// An explosion to resolve on the host and show everywhere.
    Blast(BlastEvent),
    /// A player queued VATS shots.
    VatsQueue {
        /// Queue id (per sender).
        id: u32,
        /// AP the player says they have.
        ap: u16,
        /// The shots.
        shots: Vec<VatsShot>,
    },
    /// Host to the shooter: what the queue did.
    VatsDone {
        /// Queue id.
        id: u32,
        /// One per shot.
        results: Vec<VatsResult>,
        /// AP spent.
        spent: u16,
    },
    /// Host to everyone: a player's stealth numbers, so every machine's AI
    /// reads the same sneak skill and level for them.
    Stealth {
        /// The player.
        player: PlayerId,
        /// Sneak skill.
        sneak: u8,
        /// Level.
        level: u16,
    },
    /// A player entered or left VATS (presentation only; time never stops).
    VatsMark {
        /// Who (filled by the host on relay).
        player: PlayerId,
        /// In VATS now.
        active: bool,
    },
}

impl CombatMsg {
    /// Structural bounds.
    pub fn is_sane(&self) -> bool {
        match self {
            CombatMsg::NpcHit { amount, limb, .. } | CombatMsg::PlayerHit { amount, limb, .. } => {
                *amount > 0 && *amount <= MAX_PLAYER_HIT && *limb <= LIMB_COUNT
            }
            CombatMsg::PlayerHurt { amount, .. } => *amount <= MAX_PLAYER_HIT,
            CombatMsg::VatsQueue { shots, .. } => {
                !shots.is_empty()
                    && shots.len() <= MAX_VATS_SHOTS
                    && shots
                        .iter()
                        .all(|s| s.damage <= MAX_PLAYER_HIT && s.limb <= LIMB_COUNT)
            }
            CombatMsg::VatsDone { results, .. } => results.len() <= MAX_VATS_SHOTS,
            CombatMsg::Blast(b) => b.damage <= MAX_PLAYER_HIT && b.radius <= 4096,
            _ => true,
        }
    }
}

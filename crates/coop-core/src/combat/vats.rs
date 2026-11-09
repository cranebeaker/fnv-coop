//! Real-time VATS in co-op.
//!
//! Single player freezes the world while the player plans. Co-op never
//! does: the clock the world runs on is shared, so [`vats_scales`] gives the
//! world speed 1.0 whatever VATS wants, and the slow motion (and the VATS
//! camera) become a presentation speed for the acting player's view only.
//!
//! Shots resolve on the host: [`VatsHost::resolve`] takes the queue, pays
//! each shot from the shooter's AP (the lower of what they claim and what the
//! host knows), rolls the hit chance deterministically from the queue id and
//! shot index, and applies the party toughness to the damage.

use std::collections::{BTreeMap, BTreeSet};

use super::{VatsResult, VatsShot, LIMB_COUNT, MAX_VATS_SHOTS};
use crate::ids::PlayerId;
use crate::rules::Scaling;

/// The two clocks VATS drives.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TimeScales {
    /// Speed of the simulation everyone shares (1.0 = normal; 0 = frozen).
    pub world: f32,
    /// Speed of the acting player's own presentation (camera, animation,
    /// sound of their view).
    pub presentation: f32,
}

/// The clocks for a VATS that wants `wanted` speed (0 while planning).
/// `shared` is true in a co-op session (host or guest).
pub fn vats_scales(shared: bool, wanted: f32) -> TimeScales {
    if shared {
        TimeScales {
            world: 1.0,
            presentation: wanted.clamp(0.0, 1.0),
        }
    } else {
        TimeScales {
            world: wanted.clamp(0.0, 1.0),
            presentation: wanted.clamp(0.0, 1.0),
        }
    }
}

/// Limits the host holds shooters to.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VatsLimits {
    /// Most AP any player can have.
    pub max_ap: u16,
    /// Chance cap, per mille.
    pub max_chance: u16,
    /// Per mille of a hit that is a critical (the shooter's crit chance).
    pub crit: u16,
}

impl Default for VatsLimits {
    fn default() -> Self {
        VatsLimits {
            max_ap: 200,
            max_chance: 950,
            crit: 50,
        }
    }
}

fn mix(mut x: u64) -> u64 {
    x ^= x >> 33;
    x = x.wrapping_mul(0xff51_afd7_ed55_8ccd);
    x ^= x >> 33;
    x = x.wrapping_mul(0xc4ce_b9fe_1a85_ec53);
    x ^ (x >> 33)
}

/// Host-side VATS bookkeeping.
#[derive(Clone, Debug, Default)]
pub struct VatsHost {
    /// Limits.
    pub limits: VatsLimits,
    /// AP the host knows for a player (from their shadow), when known.
    pub known_ap: BTreeMap<PlayerId, u16>,
    active: BTreeSet<PlayerId>,
    done: BTreeMap<(PlayerId, u32), (Vec<VatsResult>, u16)>,
    /// Queues resolved.
    pub queues: u32,
    /// Shots that hit.
    pub hits: u32,
    /// Shots refused for lack of AP.
    pub broke: u32,
}

impl VatsHost {
    /// Empty.
    pub fn new() -> Self {
        Self::default()
    }

    /// A player entered or left VATS.
    pub fn mark(&mut self, p: PlayerId, active: bool) {
        if active {
            self.active.insert(p);
        } else {
            self.active.remove(&p);
        }
    }

    /// Players currently in VATS (presentation).
    pub fn active(&self) -> impl Iterator<Item = &PlayerId> {
        self.active.iter()
    }

    /// Resolves a queue. A repeated `(from, id)` returns the first answer.
    pub fn resolve(
        &mut self,
        from: PlayerId,
        id: u32,
        claimed_ap: u16,
        shots: &[VatsShot],
        scaling: &Scaling,
    ) -> (Vec<VatsResult>, u16) {
        if let Some(r) = self.done.get(&(from, id)) {
            return r.clone();
        }
        let have = claimed_ap
            .min(self.limits.max_ap)
            .min(self.known_ap.get(&from).copied().unwrap_or(u16::MAX));
        let mut left = have;
        let mut spent = 0u16;
        let mut results = Vec::new();
        for (i, s) in shots.iter().take(MAX_VATS_SHOTS).enumerate() {
            if s.cost > left {
                self.broke += 1;
                results.push(VatsResult {
                    hit: false,
                    fired: false,
                    damage: 0,
                    limb: s.limb,
                    critical: false,
                });
                continue;
            }
            left -= s.cost;
            spent = spent.saturating_add(s.cost);
            let roll = mix(u64::from(from.0) << 40 ^ u64::from(id) << 8 ^ i as u64) % 1000;
            let chance = u64::from(s.chance.min(self.limits.max_chance));
            let hit = roll < chance && s.limb <= LIMB_COUNT;
            let critical = hit && mix(roll ^ 0xC417) % 1000 < u64::from(self.limits.crit);
            let base = if critical {
                f32::from(s.damage) * 2.0
            } else {
                f32::from(s.damage)
            };
            let damage = if hit {
                scaling.damage_dealt(base).round().max(1.0) as u16
            } else {
                0
            };
            if hit {
                self.hits += 1;
            }
            results.push(VatsResult {
                hit,
                fired: true,
                damage,
                limb: s.limb,
                critical,
            });
        }
        if let Some(k) = self.known_ap.get_mut(&from) {
            *k = k.saturating_sub(spent);
        }
        self.queues += 1;
        self.done.insert((from, id), (results.clone(), spent));
        if self.done.len() > 256 {
            if let Some(k) = self.done.keys().next().copied() {
                self.done.remove(&k);
            }
        }
        (results, spent)
    }
}

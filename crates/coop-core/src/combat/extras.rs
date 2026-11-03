//! Encounter extras: when the party is bigger, some enemies bring a copy.
//!
//! The decision is a hash of the enemy's net id ([`Scaling::spawns_extra`]),
//! so it is the same on every machine and runs once per enemy; the host alone
//! creates the extra (through the spawn arbiter, so the id is host-issued and
//! replicated), capped at `max_extras_alive`, and clears the ones left far
//! behind.

use std::collections::BTreeMap;

use crate::ids::{CellId, FormId, PlayerId};
use crate::rules::Scaling;

/// Extras this far from every player are cleared (game units).
pub const CLEAR_DISTANCE: f32 = 6_144.0;
/// Where the extra stands relative to its origin (game units).
const OFFSET: f32 = 96.0;

/// The spawn site key of the extra of `enemy` (one extra per enemy).
pub fn extra_site(enemy: FormId) -> FormId {
    FormId(0xFD00_0000 | (enemy.0 & 0x00FF_FFFF))
}

/// What to create.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ExtraPlan {
    /// The enemy the extra copies.
    pub origin: FormId,
    /// Spawn site key.
    pub site: FormId,
    /// Position, game units.
    pub pos: [f32; 3],
}

/// Host-side bookkeeping of extras.
#[derive(Clone, Debug, Default)]
pub struct ExtrasPlanner {
    decided: BTreeMap<FormId, bool>,
    /// Extra id (host issued) to its origin and position.
    alive: BTreeMap<FormId, (FormId, [f32; 3], CellId)>,
    planned: BTreeMap<FormId, ()>,
    /// Extras created.
    pub created: u32,
    /// Extras cleared.
    pub cleared: u32,
}

impl ExtrasPlanner {
    /// Empty planner.
    pub fn new() -> Self {
        Self::default()
    }

    /// Live extras.
    pub fn alive(&self) -> usize {
        self.alive.len() + self.planned.len()
    }

    /// An enemy appears (a dynamic encounter, not a placed ref the party
    /// already fought). Returns a plan when it brings an extra. `is_extra`
    /// keeps extras from breeding.
    pub fn consider(
        &mut self,
        s: &Scaling,
        enemy: FormId,
        pos: [f32; 3],
        is_extra: bool,
    ) -> Option<ExtraPlan> {
        if is_extra || self.decided.contains_key(&enemy) {
            return None;
        }
        let wants = s.spawns_extra(crate::ids::FormId(enemy.0));
        self.decided.insert(enemy, wants);
        if !wants || self.alive() as u32 >= s.max_extras_alive {
            return None;
        }
        self.planned.insert(enemy, ());
        // deterministic side of the origin from the id
        let sgn = if enemy.0 & 1 == 0 { 1.0 } else { -1.0 };
        Some(ExtraPlan {
            origin: enemy,
            site: extra_site(enemy),
            pos: [pos[0] + OFFSET * sgn, pos[1] + OFFSET, pos[2]],
        })
    }

    /// The host granted the spawn: `id` is the extra.
    pub fn created_as(&mut self, plan: &ExtraPlan, id: FormId, cell: CellId) {
        self.planned.remove(&plan.origin);
        self.alive.insert(id, (plan.origin, plan.pos, cell));
        self.created += 1;
    }

    /// The arbiter refused (site taken or cooling): forget the plan.
    pub fn refused(&mut self, plan: &ExtraPlan) {
        self.planned.remove(&plan.origin);
    }

    /// An extra died or was removed.
    pub fn gone(&mut self, id: FormId) {
        self.alive.remove(&id);
    }

    /// Extras that should be cleared: far from every player in `players`.
    /// `pos_of` gives an extra's current position (falls back to where it
    /// was created).
    pub fn to_clear(
        &mut self,
        players: &[(PlayerId, [f32; 3])],
        pos_of: impl Fn(FormId) -> Option<[f32; 3]>,
    ) -> Vec<FormId> {
        let far: Vec<FormId> = self
            .alive
            .iter()
            .filter(|(id, (_, p0, _))| {
                let p = pos_of(**id).unwrap_or(*p0);
                players.iter().all(|(_, q)| {
                    let d = ((q[0] - p[0]).powi(2) + (q[1] - p[1]).powi(2) + (q[2] - p[2]).powi(2))
                        .sqrt();
                    d > CLEAR_DISTANCE
                })
            })
            .map(|(id, _)| *id)
            .collect();
        for id in &far {
            self.alive.remove(id);
            self.cleared += 1;
        }
        far
    }

    /// True when `id` is a live extra.
    pub fn is_extra(&self, id: FormId) -> bool {
        self.alive.contains_key(&id)
    }
}

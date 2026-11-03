//! Friendly fire guard (G40).
//!
//! A row's `friendly_fire` names protected actors (an ally the quest needs
//! alive) and how many deliberate hits within a window a player may land
//! before the guard drops further hits. [`FfTracker`] remembers hits per
//! (slot, victim); the verdict is the policy's (`EventKind::Hit`).

use std::collections::{BTreeMap, VecDeque};

use coop_policy::Decision;
use esm::FormId;

use crate::scripting::GameState;

/// Recent hits per (attacker slot, victim), as session seconds.
#[derive(Clone, Debug, Default)]
pub struct FfTracker {
    hits: BTreeMap<(u8, FormId), VecDeque<f32>>,
    clock: f32,
    /// Hits dropped, by (slot, victim).
    pub blocked: BTreeMap<(u8, FormId), u32>,
}

impl FfTracker {
    /// Advances the clock.
    pub fn tick(&mut self, dt: f32) {
        self.clock += dt;
    }

    fn recent(&mut self, slot: u8, victim: FormId, window: f32) -> u32 {
        let now = self.clock;
        let q = self.hits.entry((slot, victim)).or_default();
        while q.front().is_some_and(|t| now - *t > window) {
            q.pop_front();
        }
        q.len() as u32
    }

    fn record(&mut self, slot: u8, victim: FormId) {
        let now = self.clock;
        self.hits.entry((slot, victim)).or_default().push_back(now);
    }
}

/// A player's hit on `victim` is about to land. Returns whether it may.
/// Hits on actors no row protects always may.
pub fn allow_hit(order: &esm::LoadOrder, gs: &mut GameState, slot: u8, victim: FormId) -> bool {
    let Some(table) = gs.coop.gate.as_ref().map(|g| g.table_arc()) else {
        return true;
    };
    if table.rows().iter().all(|r| r.friendly_fire.is_none()) {
        return true;
    }
    // rows name actors by editor id or `plugin:id`: send both aliases
    let edid = order
        .get(victim)
        .and_then(|r| r.editor_id().ok().flatten())
        .unwrap_or_default();
    let aliases = format!("{edid}|{:08X}", victim.0);
    let guards = table.guards_for(&aliases);
    if guards.is_empty() {
        return true;
    }
    let mut ok = true;
    for row in guards {
        let window = row
            .friendly_fire
            .as_ref()
            .map_or(10.0, |f| f.window_s as f32);
        let recent = gs.coop.campaign.ff.recent(slot, victim, window);
        let d = gs.with_gate(Decision::Apply, |g, gs| {
            g.hit_check(gs, g.lid(row.id), slot, &aliases, recent)
        });
        if matches!(d, Decision::Reject { .. }) {
            ok = false;
        }
    }
    if ok {
        gs.coop.campaign.ff.record(slot, victim);
    } else {
        *gs.coop
            .campaign
            .ff
            .blocked
            .entry((slot, victim))
            .or_insert(0) += 1;
        gs.coop.campaign.stats.ff_blocked += 1;
    }
    ok
}

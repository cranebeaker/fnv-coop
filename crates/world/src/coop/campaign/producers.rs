//! Producers for the policy gate: the facts `PolicyGate` only had setters
//! for. Zone membership from each player's cell and position, barrier
//! passes, interaction starts, ready checks, vote results.

use coop_policy::{Decision, PlayerMask};
use esm::{FormId, LoadOrder};

use super::owners::resolve;
use crate::coop::apply::Ctx;
use crate::dialogue::Info;
use crate::scripting::GameState;

/// Runs `f` as player `slot` (shadow rows and attribution), after telling the
/// gate that `slot` started an interaction on `quest`.
pub fn as_initiator<R>(
    gs: &mut GameState,
    slot: u8,
    quest: Option<FormId>,
    f: impl FnOnce(&mut GameState) -> R,
) -> R {
    if let (Some(q), Some(g)) = (quest, gs.coop.gate.as_mut()) {
        g.begin_interaction(q, slot);
    }
    gs.with_player(slot, f)
}

/// Where a player is, as the zone test needs it.
#[derive(Clone, Copy, Debug)]
struct Where {
    slot: u8,
    cell: Option<FormId>,
    world: Option<FormId>,
    pos: Option<[f32; 3]>,
}

fn wheres(gs: &GameState) -> Vec<Where> {
    let Some(g) = gs.coop.gate.as_ref() else {
        return Vec::new();
    };
    (0..8u8)
        .filter(|s| g.present.contains(*s))
        .filter_map(|slot| {
            let sh = gs.shadow_of(slot)?;
            let pos = sh.position.map(|p| {
                [
                    f32::from_bits(p[0]),
                    f32::from_bits(p[1]),
                    f32::from_bits(p[2]),
                ]
            });
            Some(Where {
                slot,
                cell: sh.cell,
                world: sh.world,
                pos,
            })
        })
        .collect()
}

/// True when the player stands in the tether zone of the row.
fn inside(order: &LoadOrder, gs: &GameState, t: &coop_policy::Tether, w: &Where) -> bool {
    for c in &t.cells {
        if let Some(f) = resolve(order, c) {
            if w.cell == Some(f) || w.world == Some(f) {
                return true;
            }
        }
    }
    if let (Some(a), Some(r), Some(p)) = (&t.anchor, t.radius, w.pos) {
        if let Some(anchor) = resolve(order, a) {
            if let Some((_, cell, at, _)) = gs.place(order, anchor) {
                // same place (cell) or the outdoors
                let same = w.cell == Some(cell) || w.world.is_some();
                let d = ((p[0] - at[0]).powi(2) + (p[1] - at[1]).powi(2) + (p[2] - at[2]).powi(2))
                    .sqrt();
                return same && d <= r as f32;
            }
        }
    }
    false
}

/// Recomputes `in_zone` of every tethered quest and the barrier passes that
/// follow from it. Call each frame (cheap: only rows with a tether).
pub fn update_zones(order: &LoadOrder, gs: &mut GameState) {
    let Some(table) = gs.coop.gate.as_ref().map(|g| g.table_arc()) else {
        return;
    };
    let ws = wheres(gs);
    if ws.is_empty() {
        return;
    }
    let mut updates: Vec<(FormId, u8, bool)> = Vec::new();
    let mut passes: Vec<(FormId, u8)> = Vec::new();
    let lid = |id: u32| gs.coop.gate.as_ref().map_or(FormId(id), |g| g.lid(id));
    for row in table.rows() {
        let q = lid(row.id);
        if let Some(t) = &row.tether {
            for w in &ws {
                let inz = inside(order, gs, t, w);
                updates.push((q, w.slot, inz));
                // a player in the zone has passed the gate of its quest
                if inz && row.barrier.is_some() {
                    passes.push((q, w.slot));
                }
            }
        }
        // per-player quests: reaching a barrier stage on one's own copy passes
        if row.scope == coop_policy::Scope::PerPlayer {
            if let Some(b) = &row.barrier {
                for w in &ws {
                    if let Some(sh) = gs.shadow_of(w.slot) {
                        let at = sh.quest_stage.get(&q).copied().unwrap_or(0);
                        if b.stages.iter().any(|s| at as u32 >= *s) {
                            passes.push((q, w.slot));
                        }
                    }
                }
            }
        }
    }
    if let Some(g) = gs.coop.gate.as_mut() {
        for (q, s, inz) in updates {
            g.set_in_zone(q, s, inz);
        }
        for (q, s) in passes {
            g.barrier_pass(q, s);
        }
    }
}

/// An INFO ended: when a row lists it for a ready check, starts one (the
/// player who finished the line has said yes).
pub fn ready_if_listed(cx: Ctx, gs: &mut GameState, info: &Info) {
    let Some(table) = gs.coop.gate.as_ref().map(|g| g.table_arc()) else {
        return;
    };
    for row in table.rows() {
        let Some(r) = &row.ready else { continue };
        let hit = r
            .infos
            .iter()
            .any(|s| resolve(cx.order, s) == Some(info.form_id));
        if !hit {
            continue;
        }
        let q = gs
            .coop
            .gate
            .as_ref()
            .map_or(FormId(row.id), |g| g.lid(row.id));
        let who = gs.coop.gate.as_ref().and_then(|g| g.initiator());
        gs.coop.campaign.ready.insert(q, info.form_id.0);
        if let Some(g) = gs.coop.gate.as_mut() {
            g.begin_ready(q);
            if let Some(w) = who {
                g.ready_answer(q, w, true);
            }
        }
    }
}

/// Ready checks in flight: (quest, info).
pub fn ready_open(gs: &GameState) -> Vec<(FormId, u32)> {
    gs.coop
        .campaign
        .ready
        .iter()
        .map(|(q, i)| (*q, *i))
        .collect()
}

/// A player answers the ready check of `quest`.
pub fn ready_answer(gs: &mut GameState, quest: FormId, slot: u8, yes: bool) {
    if let Some(g) = gs.coop.gate.as_mut() {
        g.ready_answer(quest, slot, yes);
    }
}

/// The current outcome of a ready check; a finished check is forgotten.
pub fn ready_outcome(gs: &mut GameState, quest: FormId) -> Option<Decision> {
    let info = *gs.coop.campaign.ready.get(&quest)?;
    let d = gs.with_gate(None, |g, gs| Some(g.ready_request(gs, quest, info)))?;
    if !matches!(d, Decision::DeferToBarrier { .. }) {
        gs.coop.campaign.ready.remove(&quest);
        if let Some(g) = gs.coop.gate.as_mut() {
            g.begin_ready(quest);
        }
    }
    Some(d)
}

/// Frame step: refreshes zones and barrier passes (the session ticks the
/// gate clocks).
pub fn frame(cx: Ctx, gs: &mut GameState) {
    update_zones(cx.order, gs);
}

/// Everyone present (for tests and diagnostics).
pub fn present(gs: &GameState) -> PlayerMask {
    gs.coop
        .gate
        .as_ref()
        .map_or(PlayerMask::NONE, |g| g.present)
}

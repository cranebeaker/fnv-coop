//! Companion ownership: whose follower is it, who may command it, and
//! whether it counts against the humanoid or the critter cap.
//!
//! The owner of a companion is the player whose shadow lists it as a
//! teammate (`PlayerShadow::teammates`). A command (wait, follow, trade,
//! dismiss) from anyone else is refused, except for plot companions of a
//! `owner = party` row. Humanoid and critter followers are counted apart
//! (`PolicyGate::critters`), classified from the base record.

use std::sync::Arc;

use coop_policy::{CompanionOp, Decision, PolicyTable};
use esm::{FormId, LoadOrder};

use super::Campaign;
use crate::scripting::GameState;

/// A form named in a row: an editor id, or `plugin:hex`.
pub fn resolve(order: &LoadOrder, s: &str) -> Option<FormId> {
    if let Some((plugin, hex)) = s.rsplit_once(':') {
        let local = u32::from_str_radix(hex, 16).ok()?;
        let p = order
            .plugins()
            .iter()
            .find(|p| p.name.eq_ignore_ascii_case(plugin))?;
        return Some(p.to_global(FormId(local)));
    }
    order.form_by_editor_id(s)
}

/// Fills the plot companion map from the table.
pub fn configure(
    camp: &mut Campaign,
    order: &LoadOrder,
    table: &Arc<PolicyTable>,
    lid: &dyn Fn(u32) -> FormId,
) {
    for row in table.rows() {
        let Some(c) = &row.companion else { continue };
        for n in &c.npcs {
            if let Some(f) = resolve(order, n) {
                camp.companion_quest.insert(f, lid(row.id));
            }
        }
    }
}

/// The slot that owns `npc` as a follower (live rows or a parked shadow).
pub fn owner_of(gs: &GameState, npc: FormId) -> Option<u8> {
    if gs.teammates.contains(&npc) {
        return Some(gs.active_slot());
    }
    gs.coop
        .bank
        .parked
        .iter()
        .find(|(_, s)| s.teammates.contains(&npc))
        .map(|(slot, _)| *slot)
}

/// Marks every follower that is a creature as a critter for the caps.
pub fn sync_critters(order: &LoadOrder, gs: &mut GameState) {
    let mut all: Vec<FormId> = gs.teammates.iter().copied().collect();
    for s in gs.coop.bank.parked.values() {
        all.extend(s.teammates.iter().copied());
    }
    let Some(g) = gs.coop.gate.as_mut() else {
        return;
    };
    for f in all {
        if crate::combat::is_creature(order, f) {
            g.critters.insert(f);
        } else {
            g.critters.remove(&f);
        }
    }
}

/// May `slot` give `op` to `npc`? Refusals are counted.
pub fn command_ok(gs: &mut GameState, slot: u8, npc: FormId, op: CompanionOp) -> bool {
    let quest = gs
        .coop
        .campaign
        .companion_quest
        .get(&npc)
        .copied()
        .unwrap_or(FormId(0));
    let owner = owner_of(gs, npc);
    let ok = gs.with_gate(true, |g, gs| {
        let owner = owner.unwrap_or(slot);
        !matches!(
            g.companion_command(gs, quest, slot, owner, op),
            Decision::Reject { .. }
        )
    });
    if !ok {
        gs.coop.campaign.stats.commands_refused += 1;
    }
    ok
}

/// Recruits `npc` for `slot` when the caps allow; the follower is added to
/// that slot's shadow only. Returns whether it happened.
pub fn recruit(order: &LoadOrder, gs: &mut GameState, slot: u8, npc: FormId) -> bool {
    if let Some(g) = gs.coop.gate.as_mut() {
        if crate::combat::is_creature(order, npc) {
            g.critters.insert(npc);
        }
    }
    if owner_of(gs, npc).is_some_and(|o| o != slot) {
        return false;
    }
    let quest = gs
        .coop
        .campaign
        .companion_quest
        .get(&npc)
        .copied()
        .unwrap_or(FormId(0));
    gs.with_player(slot, |g| {
        let ok = g.with_gate(true, |gate, gs| {
            let critter = gate.critters.contains(&npc);
            !matches!(
                gate.companion(gs, CompanionOp::Recruit, quest, critter),
                Decision::Reject { .. }
            )
        });
        if ok {
            g.teammates.insert(npc);
        }
        ok
    })
}

/// Host, every frame: a follower comes along when its owner's player stands
/// in another place. Each game runs `companions::come_along` for its own
/// player only, and the host's parked shadows hold the other players'
/// followers, so the host moves those. `place` turns an owner's filed
/// place (an interior cell, or outdoors the worldspace) and position into
/// (space, cell). Returns the followers moved.
pub fn follow_owners(
    order: &LoadOrder,
    scripts: &crate::scripting::ScriptCache,
    gs: &mut GameState,
    place: &mut dyn FnMut(FormId, [f32; 3]) -> Option<(FormId, FormId)>,
) -> Vec<FormId> {
    let mut want = Vec::new();
    for sh in gs.coop.bank.parked.values() {
        let (Some(space), Some(pos)) = (sh.cell, sh.position) else {
            continue;
        };
        let (pos, heading) = (pos.map(f32::from_bits), f32::from_bits(sh.heading));
        want.extend(sh.teammates.iter().map(|t| (*t, space, pos, heading)));
    }
    let mut moved = Vec::new();
    for (who, space, pos, heading) in want {
        // only a change of place: in the same place the follower's AI follows
        match gs.place(order, who) {
            Some(p) if p.0 != space => {}
            _ => continue,
        }
        let Some((space, cell)) = place(space, pos) else {
            continue;
        };
        if crate::companions::bring_teammate(order, scripts, gs, who, (space, cell, pos, heading)) {
            moved.push(who);
        }
    }
    moved
}

/// Dismisses `npc` from its owner when `slot` may.
pub fn dismiss(gs: &mut GameState, slot: u8, npc: FormId) -> bool {
    if !command_ok(gs, slot, npc, CompanionOp::Dismiss) {
        return false;
    }
    let Some(owner) = owner_of(gs, npc) else {
        return false;
    };
    gs.with_player(owner, |g| g.teammates.remove(&npc))
}

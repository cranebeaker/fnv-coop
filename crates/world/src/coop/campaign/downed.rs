//! Downed players, respawn at the zone checkpoint, late join (G41).
//!
//! The policy says what a down means (`policy.death`, a row's
//! `shadow.down` / `checkpoint`) and what a wipe does; this keeps who is down,
//! for how long, and carries the outcome out: a player left down too long
//! respawns at the checkpoint, a player down until a checkpoint is revived on
//! reaching it, a wipe reloads the host save or respawns everyone, a joiner
//! is held out of the gates while a scene plays and warped into the zone
//! after it.

use std::collections::BTreeMap;

use coop_policy::lifecycle::{
    down_outcome, is_wipe, join_plan, wipe_outcome, DownOutcome, JoinPlan,
};
use coop_policy::{AllDown, Decision, Event, EventKind, Origin, PlayerMask};
use esm::{FormId, LoadOrder};

use super::owners::resolve;
use crate::scripting::GameState;

/// One downed player.
#[derive(Clone, Debug, PartialEq)]
pub struct DownInfo {
    /// Seconds down.
    pub since_s: f32,
    /// What the policy said about this down.
    pub outcome: DownOutcome,
    /// The quest whose row decided it.
    pub quest: Option<FormId>,
}

/// A place to come back to.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Checkpoint {
    /// Cell.
    pub cell: FormId,
    /// Worldspace when outdoors.
    pub world: Option<FormId>,
    /// Position.
    pub pos: [f32; 3],
}

/// Who is down, the checkpoint and what happened.
#[derive(Clone, Debug, Default)]
pub struct DownTracker {
    /// Downed players by slot.
    pub down: BTreeMap<u8, DownInfo>,
    /// The zone checkpoint (where the whole party last stood together).
    pub checkpoint: Option<Checkpoint>,
    /// Respawns done, in order.
    pub respawned: Vec<u8>,
    /// Revives done, in order.
    pub revived: Vec<u8>,
    /// A wipe the host must resolve by reloading its save.
    pub reload_requested: bool,
    /// Joiners warped into the zone.
    pub warped: Vec<u8>,
    /// Joiners refused while a scene played.
    pub refused: Vec<u8>,
    /// Downs of other players the host decided (a script run as that player):
    /// (slot, revivable). The host adapter reports them to the live rules.
    pub to_live: Vec<(u8, bool)>,
    /// How many of the live rules' events [`mirror_rules`] has read.
    pub rules_seen: usize,
}

/// The host's own slot.
const HOST_SLOT: u8 = 0;

fn table(gs: &GameState) -> Option<std::sync::Arc<coop_policy::PolicyTable>> {
    gs.coop.gate.as_ref().map(|g| g.table_arc())
}

/// A player's health reached zero. `quest` is the quest whose row applies
/// (for example the Dead Money collar quest), if any.
pub fn player_down(gs: &mut GameState, slot: u8, quest: Option<FormId>) -> Option<DownOutcome> {
    let t = table(gs)?;
    let row = quest.and_then(|q| gs.coop.gate.as_ref().and_then(|g| g.row(q)));
    let out = down_outcome(&t, row);
    // the gate counts the event and tells which class the down belongs to
    if let Some(g) = gs.coop.gate.take() {
        let ev = Event {
            quest: quest.map_or(0, |q| q.0),
            origin: Origin::Host,
            kind: EventKind::PlayerDown { slot },
        };
        let d = g.decide(gs, &ev);
        let _ = matches!(d, Decision::PerPlayerShadow { .. });
        gs.coop.gate = Some(g);
    }
    gs.coop.campaign.down.down.insert(
        slot,
        DownInfo {
            since_s: 0.0,
            outcome: out.clone(),
            quest,
        },
    );
    if out == DownOutcome::Respawn {
        respawn(gs, slot);
    }
    Some(out)
}

/// A teammate revived `slot`. A down the policy holds until a checkpoint (a
/// Dead Money collar death) cannot be revived: false, and nothing changes.
pub fn revive(gs: &mut GameState, slot: u8) -> bool {
    if held(gs, slot) {
        return false;
    }
    stand_up(gs, slot)
}

/// True when `slot` is down until the party reaches a checkpoint.
pub fn held(gs: &GameState, slot: u8) -> bool {
    matches!(
        gs.coop.campaign.down.down.get(&slot).map(|d| &d.outcome),
        Some(DownOutcome::UntilCheckpoint { .. })
    )
}

/// False when `slot` is down and no teammate may revive it.
pub fn revivable(gs: &GameState, slot: u8) -> bool {
    !held(gs, slot)
}

/// `slot` stands up where it lies (a revive, or a checkpoint reached).
fn stand_up(gs: &mut GameState, slot: u8) -> bool {
    let was = gs.coop.campaign.down.down.remove(&slot).is_some();
    if was {
        gs.coop.campaign.down.revived.push(slot);
        gs.with_player(slot, |g| {
            g.damage.remove(&crate::dialogue::PLAYER_REF);
            g.dead.remove(&crate::dialogue::PLAYER_REF);
        });
        if slot == HOST_SLOT && gs.coop.authority == crate::coop::Authority::Host {
            gs.coop.live.downed = false;
        }
    }
    was
}

/// The local player's health ran out (a hit, a script's `KillActor`). On the
/// host inside another player's scope (a script run as that player) it is
/// that player's down, decided here. Otherwise the local player goes down and
/// the quest whose script did it decides how (the rules bridge reports it).
pub fn health_out(gs: &mut GameState) {
    let quest = gs.coop.campaign.cond_quest;
    let slot = gs.active_slot();
    if gs.coop.authority == crate::coop::Authority::Host
        && slot != HOST_SLOT
        && gs.coop.bank.enabled()
    {
        script_down(gs, slot, quest);
        return;
    }
    gs.coop.live.down_pending = true;
    gs.coop.campaign.down_quest = quest;
}

/// Host: a script run as `slot` (a collar's explosion, a `KillActor` on that
/// player) put it down, under `quest`'s row if any. The campaign rules decide
/// the outcome, and the live rules hear of the down (revivable or not).
pub fn script_down(gs: &mut GameState, slot: u8, quest: Option<FormId>) -> Option<DownOutcome> {
    if slot == HOST_SLOT {
        gs.coop.live.down_pending = true;
        gs.coop.campaign.down_quest = quest;
        return None;
    }
    if gs.coop.campaign.down.down.contains_key(&slot) {
        return None;
    }
    let out = player_down(gs, slot, quest)?;
    match out {
        DownOutcome::Downed { .. } => gs.coop.campaign.down.to_live.push((slot, true)),
        DownOutcome::UntilCheckpoint { .. } => gs.coop.campaign.down.to_live.push((slot, false)),
        DownOutcome::Die | DownOutcome::Respawn => {}
    }
    Some(out)
}

/// Host: the live rules' revive events (a teammate's hold to revive, the
/// window's respawn, a guest's own report of a down) reach the campaign
/// tracker for every slot. `events` is the rules bridge's list. The host's
/// own downs are mirrored from its down flags in [`super::host_frame`].
pub fn mirror_rules(gs: &mut GameState, events: &[coop_core::rules::RulesEvent]) {
    use coop_core::rules::{ReviveEvent, RulesEvent};
    let seen = gs.coop.campaign.down.rules_seen;
    let from = if seen > events.len() { 0 } else { seen };
    gs.coop.campaign.down.rules_seen = events.len();
    for e in &events[from..] {
        let RulesEvent::Revive(r) = e else {
            continue;
        };
        match *r {
            ReviveEvent::Downed(p)
                if p.0 != HOST_SLOT && !gs.coop.campaign.down.down.contains_key(&p.0) =>
            {
                player_down(gs, p.0, None);
            }
            ReviveEvent::Revived { player, .. } => {
                revive(gs, player.0);
            }
            // the live window ran out; a down held until a checkpoint stays down
            ReviveEvent::Respawn(p)
                if gs.coop.campaign.down.down.contains_key(&p.0) && !held(gs, p.0) =>
            {
                respawn(gs, p.0);
            }
            _ => {}
        }
    }
}

/// Puts `slot` back at the checkpoint with its health.
pub fn respawn(gs: &mut GameState, slot: u8) {
    let cp = gs.coop.campaign.down.checkpoint;
    gs.coop.campaign.down.down.remove(&slot);
    gs.with_player(slot, |g| {
        g.damage.remove(&crate::dialogue::PLAYER_REF);
        g.dead.remove(&crate::dialogue::PLAYER_REF);
        if let Some(c) = cp {
            g.player_cell = Some(c.cell);
            g.player_world = c.world;
            g.player_position = Some(c.pos);
        }
    });
    gs.coop.campaign.down.respawned.push(slot);
}

fn downed_mask(gs: &GameState) -> PlayerMask {
    gs.coop
        .campaign
        .down
        .down
        .keys()
        .fold(PlayerMask::NONE, |m, s| m.or(PlayerMask::of(*s)))
}

/// Records the checkpoint whenever the whole party stands in one cell.
fn note_checkpoint(gs: &mut GameState) {
    let Some(g) = gs.coop.gate.as_ref() else {
        return;
    };
    let present = g.present;
    let mut cell: Option<FormId> = None;
    for s in 0..8u8 {
        if !present.contains(s) {
            continue;
        }
        let Some(sh) = gs.shadow_of(s) else { return };
        match (cell, sh.cell) {
            (None, Some(c)) => cell = Some(c),
            (Some(a), Some(b)) if a == b => {}
            _ => return,
        }
    }
    let (Some(cell), Some(host)) = (cell, gs.shadow_of(0)) else {
        return;
    };
    if let Some(p) = host.position {
        gs.coop.campaign.down.checkpoint = Some(Checkpoint {
            cell,
            world: host.world,
            pos: [
                f32::from_bits(p[0]),
                f32::from_bits(p[1]),
                f32::from_bits(p[2]),
            ],
        });
    }
}

/// One frame: clocks, checkpoint revives, expired downs, wipes, late joiners.
pub fn frame(order: &LoadOrder, gs: &mut GameState, dt: f32) {
    let Some(t) = table(gs) else { return };
    note_checkpoint(gs);
    // clocks
    for d in gs.coop.campaign.down.down.values_mut() {
        d.since_s += dt;
    }
    // down until a checkpoint cell is reached
    let waiting: Vec<(u8, Vec<String>)> = gs
        .coop
        .campaign
        .down
        .down
        .iter()
        .filter_map(|(s, d)| match &d.outcome {
            DownOutcome::UntilCheckpoint { checkpoints } => Some((*s, checkpoints.clone())),
            _ => None,
        })
        .collect();
    for (slot, cps) in waiting {
        let here = gs.shadow_of(slot).and_then(|s| s.cell);
        if cps
            .iter()
            .filter_map(|c| resolve(order, c))
            .any(|c| Some(c) == here)
        {
            stand_up(gs, slot);
        }
    }
    // revive window ran out: back at the checkpoint
    let expired: Vec<u8> = gs
        .coop
        .campaign
        .down
        .down
        .iter()
        .filter(|(_, d)| matches!(d.outcome, DownOutcome::Downed { revive_s } if d.since_s >= revive_s as f32))
        .map(|(s, _)| *s)
        .collect();
    // a wipe is decided before individual expiries
    let present = gs
        .coop
        .gate
        .as_ref()
        .map_or(PlayerMask::NONE, |g| g.present);
    if is_wipe(present, downed_mask(gs)) {
        match wipe_outcome(&t) {
            AllDown::ReloadHostSave => {
                gs.coop.campaign.down.reload_requested = true;
                gs.coop.live.wiped = true;
            }
            AllDown::RespawnAtCheckpoint => {
                for s in 0..8u8 {
                    if present.contains(s) {
                        respawn(gs, s);
                    }
                }
            }
        }
        return;
    }
    for s in expired {
        respawn(gs, s);
    }
    late_joiners(gs, &t);
}

/// A player joined: apply the late join policy.
pub fn on_join(gs: &mut GameState, slot: u8) -> Option<JoinPlan> {
    let t = table(gs)?;
    let scene = gs.coop.gate.as_ref().is_some_and(|g| g.scenes_active());
    let plan = join_plan(&t, scene);
    match plan {
        JoinPlan::WaitForScene => {
            if let Some(g) = gs.coop.gate.as_mut() {
                g.hold_joiner(slot);
            }
        }
        JoinPlan::Refuse => gs.coop.campaign.down.refused.push(slot),
        JoinPlan::WarpToZone => warp_to_checkpoint(gs, slot),
        JoinPlan::StayPut => {}
    }
    Some(plan)
}

fn warp_to_checkpoint(gs: &mut GameState, slot: u8) {
    if let Some(c) = gs.coop.campaign.down.checkpoint {
        gs.with_player(slot, |g| {
            g.player_cell = Some(c.cell);
            g.player_world = c.world;
            g.player_position = Some(c.pos);
        });
        gs.coop.campaign.down.warped.push(slot);
    }
}

fn late_joiners(gs: &mut GameState, t: &coop_policy::PolicyTable) {
    let Some(g) = gs.coop.gate.as_ref() else {
        return;
    };
    if g.held().is_empty() || g.scenes_active() {
        return;
    }
    let warp = join_plan(t, false) == JoinPlan::WarpToZone;
    let released = gs
        .coop
        .gate
        .as_mut()
        .map(|g| g.release_joiners())
        .unwrap_or(PlayerMask::NONE);
    if warp {
        for s in 0..8u8 {
            if released.contains(s) {
                warp_to_checkpoint(gs, s);
            }
        }
    }
}

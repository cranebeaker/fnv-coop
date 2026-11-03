//! Per-player state the player shadow does not carry.
//!
//! * object variables of scripts on references (Dead Money collar scripts:
//!   `NVDLC01VaultCollarBeepScript.bCollarOn` ...), listed in a row's
//!   `shadow.objvars`;
//! * every variable of the follower bookkeeping quests (per owner);
//! * hardcore needs and the rest of `Living`;
//! * the player's imported `.fos` inventory stacks;
//! * the contents of personal storage containers.
//!
//! [`swap`] is called from `GameState::swap_to`, so every
//! `as_player` / `with_player` scope sees its own copy.

use std::collections::BTreeMap;
use std::sync::Arc;

use coop_policy::PolicyTable;
use esm::{FormId, LoadOrder};

use super::Campaign;
use crate::dialogue::PLAYER_REF;
use crate::inventory_stacks::SavedInventory;
use crate::scripting::GameState;

/// One slot's extra rows.
#[derive(Clone, Debug, Default)]
pub struct ExtraShadow {
    /// (owner reference, variable) to value.
    pub objvars: BTreeMap<(FormId, String), f64>,
    /// (follower quest, variable) to value.
    pub follower_vars: BTreeMap<(FormId, String), f64>,
    /// Hardcore needs and friends.
    pub living: Option<crate::living::Living>,
    /// The player's imported inventory stacks.
    pub stacks: Option<SavedInventory>,
    /// (personal container, item) to count.
    pub storage: BTreeMap<(FormId, FormId), i32>,
}

/// Reads the wanted object variables and the follower quests from the table.
pub fn configure(
    camp: &mut Campaign,
    _order: &LoadOrder,
    table: &Arc<PolicyTable>,
    lid: &dyn Fn(u32) -> FormId,
) {
    for row in table.rows() {
        if let Some(sh) = &row.shadow {
            for ov in &sh.objvars {
                if let Some((script, var)) = ov.split_once('.') {
                    let e = camp
                        .objvar_wanted
                        .entry(script.to_ascii_lowercase())
                        .or_default();
                    let v = var.to_ascii_lowercase();
                    if !e.contains(&v) {
                        e.push(v);
                    }
                }
            }
        }
        if row.companion.is_some() && row.edid.to_ascii_lowercase().contains("follower") {
            camp.follower_quests.insert(lid(row.id));
        }
    }
}

impl Campaign {
    /// A reference was seen running `script` (editor id). Only wanted
    /// scripts are remembered.
    pub fn note_script_owner(&mut self, owner: FormId, script_edid: &str) {
        let k = script_edid.to_ascii_lowercase();
        if self.objvar_wanted.contains_key(&k) {
            self.objvar_owners.insert(owner, k);
        }
    }
}

impl GameState {
    /// Hook for the script runner: remembers owners of wanted scripts.
    pub fn campaign_note_script(&mut self, order: &LoadOrder, owner: FormId, script: FormId) {
        if self.coop.campaign.objvar_wanted.is_empty() {
            return;
        }
        if let Some(e) = order.get(script).and_then(|r| r.editor_id().ok().flatten()) {
            self.coop.campaign.note_script_owner(owner, &e);
        }
    }
}

fn capture(gs: &GameState) -> ExtraShadow {
    let c = &gs.coop.campaign;
    let mut x = ExtraShadow::default();
    for (owner, script) in &c.objvar_owners {
        let (Some(names), Some(l)) = (c.objvar_wanted.get(script), gs.variables.get(owner)) else {
            continue;
        };
        for n in names {
            if let Some(v) = l.get(n) {
                x.objvars.insert((*owner, n.clone()), v);
            }
        }
    }
    for q in &c.follower_quests {
        if let Some(l) = gs.variables.get(q) {
            for (n, _, v) in l.iter() {
                x.follower_vars.insert((*q, n.to_string()), v);
            }
        }
    }
    x.living = Some(gs.living.clone());
    x.stacks = gs.inventory_stacks.get(&PLAYER_REF).cloned();
    for ((h, i), n) in &gs.items {
        if c.personal.contains(h) {
            x.storage.insert((*h, *i), *n);
        }
    }
    x
}

fn clear(gs: &mut GameState) {
    let owners: Vec<(FormId, Vec<String>)> = gs
        .coop
        .campaign
        .objvar_owners
        .iter()
        .filter_map(|(o, s)| {
            gs.coop
                .campaign
                .objvar_wanted
                .get(s)
                .map(|n| (*o, n.clone()))
        })
        .collect();
    for (o, names) in owners {
        if let Some(l) = gs.variables.get_mut(&o) {
            for n in names {
                l.set(&n, 0.0);
            }
        }
    }
    for q in gs.coop.campaign.follower_quests.clone() {
        if let Some(l) = gs.variables.get_mut(&q) {
            let names: Vec<String> = l.iter().map(|(n, _, _)| n.to_string()).collect();
            for n in names {
                l.set(&n, 0.0);
            }
        }
    }
    let (hc, ahc) = (gs.living.hardcore, gs.living.always_hardcore);
    gs.living = crate::living::Living::default();
    gs.living.hardcore = hc;
    gs.living.always_hardcore = ahc;
    gs.inventory_stacks.remove(&PLAYER_REF);
    let personal = gs.coop.campaign.personal.clone();
    gs.items.retain(|(h, _), _| !personal.contains(h));
}

fn install(gs: &mut GameState, x: ExtraShadow) {
    for ((o, n), v) in x.objvars {
        if let Some(l) = gs.variables.get_mut(&o) {
            l.set(&n, v);
        }
    }
    for ((q, n), v) in x.follower_vars {
        if let Some(l) = gs.variables.get_mut(&q) {
            l.set(&n, v);
        }
    }
    if let Some(mut l) = x.living {
        l.hardcore = gs.living.hardcore;
        l.always_hardcore = gs.living.always_hardcore;
        gs.living = l;
    }
    if let Some(s) = x.stacks {
        gs.inventory_stacks.insert(PLAYER_REF, s);
    }
    for ((h, i), n) in x.storage {
        gs.items.insert((h, i), n);
    }
}

/// Parks `prev`'s extra rows and installs `next`'s. Called by the shadow
/// swap after the player rows moved.
pub fn swap(gs: &mut GameState, prev: u8, next: u8) {
    if gs.coop.campaign.is_idle() {
        return;
    }
    let parked = capture(gs);
    clear(gs);
    let incoming = gs.coop.campaign.parked.remove(&next).unwrap_or_default();
    install(gs, incoming);
    gs.coop.campaign.parked.insert(prev, parked);
}

/// A copy of `slot`'s extra rows wherever they live (live rows for the
/// active slot, the parked copy otherwise).
pub fn extra_of(gs: &GameState, slot: u8) -> ExtraShadow {
    if slot == gs.active_slot() {
        capture(gs)
    } else {
        gs.coop
            .campaign
            .parked
            .get(&slot)
            .cloned()
            .unwrap_or_default()
    }
}

impl Campaign {
    /// Nothing configured that needs a swap.
    pub fn is_idle(&self) -> bool {
        self.objvar_owners.is_empty()
            && self.follower_quests.is_empty()
            && self.personal.is_empty()
            && !self.swap_living
    }
}

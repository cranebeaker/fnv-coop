//! Campaign gaps closed in the world (see `crates/coop-policy/CAMPAIGN.md`).
//!
//! Everything here is new logic behind one field, `GameState::coop.campaign`,
//! and a handful of one-line hooks in shared files:
//!
//! * [`attribution`]: kills, XP and karma by real player slot.
//! * [`owners`]: companion ownership, owner-only commands, critter counting.
//! * [`extra`]: state the player shadow does not carry (script object
//!   variables such as Dead Money collars, follower quest variables, hardcore
//!   needs, imported `.fos` stacks, personal storage containers), swapped
//!   with the shadow.
//! * [`storage`]: personal storage redirect for the loot ledger.
//! * [`party`]: party aware condition reducers.
//! * [`producers`]: feeds the policy gate (zone membership, barrier passes,
//!   interaction start, ready checks, votes).
//! * [`consent`]: one-way fork votes.
//! * [`downed`]: downed state, checkpoint respawn, late join.
//! * [`ffguard`]: friendly fire guard.
//! * [`names`]: real script variable names on guests.

use std::collections::{BTreeMap, BTreeSet};

use esm::{FormId, LoadOrder};

use crate::scripting::GameState;

pub mod attribution;
pub mod bot;
pub mod consent;
pub mod downed;
pub mod extra;
pub mod ffguard;
pub mod names;
pub mod needs;
pub mod owners;
pub mod party;
pub mod producers;
pub mod progress;
pub mod storage;

/// All campaign bookkeeping of one machine.
#[derive(Clone, Debug, Default)]
pub struct Campaign {
    /// Resolved from the policy table and the load order.
    pub configured: bool,
    /// Hardcore needs and imported stacks are swapped per player.
    pub swap_living: bool,
    /// The live layer pays kills by slot (set by the host adapter).
    pub attribution_on: bool,
    /// Damage per victim and slot.
    pub tally: BTreeMap<FormId, BTreeMap<u8, f64>>,
    /// Victims whose last ledger hit came from an NPC: their death has no killer.
    pub npc_blow: BTreeSet<FormId>,
    /// Kills already settled.
    pub settled: BTreeSet<FormId>,
    /// What kills paid, for tests and diagnostics: (victim, slot, xp).
    pub paid: Vec<(FormId, u8, f64)>,
    /// Containers that are personal (one inventory per player).
    pub personal: BTreeSet<FormId>,
    /// Wanted object variables: script editor id (lower) to variable names (lower).
    pub objvar_wanted: BTreeMap<String, Vec<String>>,
    /// Owners (references) seen running a wanted script: owner to script (lower).
    pub objvar_owners: BTreeMap<FormId, String>,
    /// Quests whose every variable is per owner (follower bookkeeping).
    pub follower_quests: BTreeSet<FormId>,
    /// Parked extra state of every slot but the active one.
    pub parked: BTreeMap<u8, extra::ExtraShadow>,
    /// The quest whose script or dialogue is being evaluated (party reducers).
    pub cond_quest: Option<FormId>,
    /// The local player was down at the last host frame.
    pub local_down: bool,
    /// The quest whose script took the local player's health (a collar).
    pub down_quest: Option<FormId>,
    /// Skill points the open level-up menu has given (skill actor value to
    /// points), sent with the perk when it closes (`progress::level_up`).
    pub levelup_draft: BTreeMap<u16, i32>,
    /// Session clock (ms) of the next growth of the other players' needs.
    pub needs_ms: u64,
    /// The slot whose blow killed each settled victim (`party` `IsKiller`).
    pub killers: BTreeMap<FormId, u8>,
    /// Session clock of the last host frame (ms).
    pub last_ms: u64,
    /// Members seen by the last host frame (slot bit set).
    pub known: u8,
    /// Ready checks in flight: quest to the INFO that raised it.
    pub ready: BTreeMap<FormId, u32>,
    /// Companion NPC to the quest row that owns it (plot companions).
    pub companion_quest: BTreeMap<FormId, FormId>,
    /// Friendly fire tracker.
    pub ff: ffguard::FfTracker,
    /// Downed players and the checkpoint rules.
    pub down: downed::DownTracker,
    /// Open fork votes.
    pub forks: consent::Forks,
    /// Named results of party condition probes (test bot, diagnostics).
    pub probes: BTreeMap<String, f64>,
    /// Variable writes that arrived before game data was available.
    pub unnamed_vars: bool,
    /// Counters for tests.
    pub stats: CampaignStats,
}

/// Counters for tests and diagnostics.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CampaignStats {
    /// Kills settled by slot.
    pub kills_settled: u32,
    /// Hits dropped by the friendly fire guard.
    pub ff_blocked: u32,
    /// Companion commands refused.
    pub commands_refused: u32,
}

impl Campaign {
    /// True when the live layer settles kills and `experience::on_kill`
    /// must stay out of it.
    pub fn attributes(&self) -> bool {
        self.attribution_on
    }
}

/// Host adapter frame: configures once, then keeps the gate fed (zones,
/// barrier passes), classifies followers, runs the down and late join rules
/// and the open forks. `now_ms` is the session clock.
pub fn host_frame(cx: crate::coop::apply::Ctx, gs: &mut GameState, now_ms: u64) {
    gs.configure_campaign(cx.order);
    gs.coop.campaign.attribution_on = true;
    if gs.coop.gate.is_none() {
        return;
    }
    let dt = if gs.coop.campaign.last_ms == 0 {
        0.0
    } else {
        now_ms.saturating_sub(gs.coop.campaign.last_ms) as f32 / 1000.0
    };
    gs.coop.campaign.last_ms = now_ms.max(1);
    // new members: late join policy
    let members = gs.coop.gate.as_ref().map_or(0, |g| g.members().0);
    let fresh = members & !gs.coop.campaign.known & !1;
    gs.coop.campaign.known = members;
    for s in 1..8u8 {
        if fresh & (1 << s) != 0 {
            downed::on_join(gs, s);
        }
    }
    // the local player went down (the rules bridge consumes the flag later)
    let down_now = gs.coop.live.down_pending || gs.coop.live.downed;
    if down_now && !gs.coop.campaign.local_down {
        let slot = gs.active_slot();
        let quest = gs.coop.campaign.down_quest.take();
        downed::player_down(gs, slot, quest);
    } else if !down_now && gs.coop.campaign.local_down {
        let slot = gs.active_slot();
        downed::revive(gs, slot);
    }
    gs.coop.campaign.local_down = down_now;
    gs.coop.campaign.ff.tick(dt);
    // a respawned actor can pay again
    let dead = gs.dead.clone();
    gs.coop.campaign.settled.retain(|v| dead.contains(v));
    producers::frame(cx, gs);
    owners::sync_critters(cx.order, gs);
    downed::frame(cx.order, gs, dt);
    consent::poll(gs);
    needs::frame(cx.order, gs, now_ms);
}

impl GameState {
    /// Resolves the table's names against the load order (once). Call from
    /// the live adapters' frame; harmless without a policy gate.
    pub fn configure_campaign(&mut self, order: &LoadOrder) {
        if self.coop.campaign.configured {
            return;
        }
        let Some(gate) = self.coop.gate.as_mut() else {
            return;
        };
        let table = gate.table_arc();
        // a table still keyed by the plugins' own index bytes (one the session did not
        // resolve with `policy_ids`): map the gate's lookups to the running load order. A
        // resolved table gives no pairs. The shadow plan is left alone: only the host has a
        // gate, and both peers must keep the plan they were given or their shadows split
        let pairs = table_to_load_order(order, &table);
        gate.set_remap(pairs.iter().copied());
        let gate = self.coop.gate.as_ref().expect("gate");
        let lid = |id: u32| gate.lid(id);
        let mut camp = std::mem::take(&mut self.coop.campaign);
        camp.configured = true;
        camp.swap_living = true;
        extra::configure(&mut camp, order, &table, &lid);
        storage::configure(&mut camp, order, &table);
        owners::configure(&mut camp, order, &table, &lid);
        self.coop.campaign = camp;
    }
}

/// (load-order id, table id) for every row whose plugin is in `order`.
pub fn table_to_load_order(order: &LoadOrder, table: &coop_policy::PolicyTable) -> Vec<(u32, u32)> {
    let mut out = Vec::new();
    for row in table.rows() {
        let Some(p) = order
            .plugins()
            .iter()
            .find(|p| p.name.eq_ignore_ascii_case(&row.plugin))
        else {
            continue;
        };
        let load = p.to_global(FormId(row.id)).0;
        if load != row.id {
            out.push((load, row.id));
        }
    }
    out
}

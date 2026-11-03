//! Live-layer facts carried by `GameState` and the world-side hooks.
//!
//! The hooks are called from a few world sites (`combat::hurt`,
//! `experience::on_kill`, `map::travel`, `dialogue::line_begins`). Every one
//! starts with an authority test, so single player pays one integer compare.

use std::collections::HashMap;

use coop_core::ids::PlayerId;
use coop_core::rules::Scaling;
use esm::{FormId, LoadOrder};

use super::Authority;
use crate::dialogue::PLAYER_REF;
use crate::scripting::GameState;

/// First form id of the synthetic puppets that stand for remote players.
/// The block is far from the placed-reference allocator (`0xFF000001`).
pub const PUPPET_BASE: u32 = 0xFE00_0000;

/// The puppet form of a player slot.
pub fn puppet_of(p: PlayerId) -> FormId {
    FormId(PUPPET_BASE + p.0 as u32)
}

/// The player a puppet form stands for.
pub fn player_of(f: FormId) -> Option<PlayerId> {
    (f.0 >= PUPPET_BASE && f.0 < PUPPET_BASE + 256).then(|| PlayerId((f.0 - PUPPET_BASE) as u8))
}

/// A guest's damage on someone else's actor, waiting to go to the host.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PendingHit {
    /// Victim.
    pub who: FormId,
    /// Points (after enemy toughness).
    pub amount: f64,
    /// Attacker.
    pub by: FormId,
}

/// Damage the host just applied itself; the adapter feeds it to the ledger.
pub type HurtRecord = PendingHit;

/// A dynamic reference the host announced.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpawnNote {
    /// Allocated id.
    pub id: FormId,
    /// Base form (`None` when the reference was removed).
    pub base: Option<FormId>,
    /// Where it appeared.
    pub pos: [f32; 3],
}

/// A container action of the local player, for the live layer to carry.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LootOp {
    /// The container menu opened.
    Open(FormId),
    /// Items moved from the container to the player (guests: not yet moved).
    Take {
        /// Container.
        container: FormId,
        /// Item.
        item: FormId,
        /// Count.
        n: u32,
    },
    /// Items moved from the player to the container (guests: not yet moved).
    Put {
        /// Container.
        container: FormId,
        /// Item.
        item: FormId,
        /// Count.
        n: u32,
    },
    /// The container menu closed.
    Close(FormId),
}

/// Per-machine live-layer facts. Never replicated, never saved.
#[derive(Clone, Debug, Default)]
pub struct LiveFacts {
    /// Guest damage to resolve on the host.
    pub pending_hits: Vec<PendingHit>,
    /// Host damage to feed to the host ledger.
    pub hurts: Vec<HurtRecord>,
    /// The scaling in force (copied from the rules each frame).
    pub scaling: Option<Scaling>,
    /// The host's campaign rules in force: the difficulty mode and friendly
    /// fire (copied from the rules each frame; a guest learns them from the
    /// scaling announcement).
    pub rules_choice: Option<(coop_core::rules::Mode, bool)>,
    /// Animation ids of actors this machine owns (the viewer writes them).
    pub anim: HashMap<FormId, u16>,
    /// Animation ids of actors owned elsewhere (the viewer reads them).
    pub remote_anim: HashMap<FormId, u16>,
    /// While true, `hurt` applies a host verdict without hooks.
    pub replaying: bool,
    /// Fast travel the player asked for and the party has not yet allowed.
    pub travel_requests: Vec<FormId>,
    /// Marker the party allowed; the next `map::travel` to it goes through.
    pub travel_allowed: Option<FormId>,
    /// The local player's health reached zero (they are downed, not dead).
    pub down_pending: bool,
    /// The local player is downed.
    pub downed: bool,
    /// Set while a tether holds the story stage.
    pub stage_held: bool,
    /// Host: each party member's tether zone membership (slot, inside),
    /// empty while no tether runs.
    pub tether_zone: Vec<(u8, bool)>,
    /// Cells the host has loaded (`None` until the session reports them).
    /// Script commands that need a live entity wait for these
    /// ([`super::roam_ops`]).
    pub host_loaded: Option<std::collections::BTreeSet<u32>>,
    /// Script commands deferred for an unloaded cell: (reference, cell,
    /// function, arguments). The session drains them into the host queue.
    pub deferred_out: Vec<(FormId, FormId, String, Vec<coop_core::roam::DArg>)>,
    /// Dynamic references to create or remove (viewer drains).
    pub spawns: Vec<SpawnNote>,
    /// Sleep or wait approved by the party, in hours.
    pub time_skip_ok: Option<u32>,
    /// Fork vote result (param, approved).
    pub fork_result: Option<(u32, bool)>,
    /// The whole party was downed.
    pub wiped: bool,
    /// Dialogue line plays the viewer must show: (step, info form).
    pub lines: Vec<(u32, FormId)>,
    /// Container actions the viewer's menus queued (the live layer drains).
    pub loot_ops: Vec<LootOp>,
    /// The local player's animation id (the viewer writes it each frame).
    pub local_anim: u16,
    /// The local player's drawn weapon form (0 none; the viewer writes it).
    pub local_weapon: u32,
    /// Extra avatar flags (`FLAG_SNEAK`, `FLAG_DRAWN`; the viewer writes them).
    pub local_flags: u16,
    /// Combat actions the local player did this frame (the viewer pushes,
    /// the live layer sends them to the others and drains).
    pub local_actions: Vec<(coop_core::avatar::ActionKind, u32)>,
    /// Scenes, movies, control locks and time (see [`super::scene`]).
    pub scene: super::scene::SceneFacts,
    /// Combat depth: queued notes, remote players' stealth, counters.
    pub combat: super::combat_live::CombatFacts,
}

fn player_side(f: FormId) -> bool {
    f == PLAYER_REF || player_of(f).is_some()
}

/// Damage after the party scaling: players hit enemies for less, enemies hit
/// players for more.
pub fn scaled_damage(state: &GameState, who: FormId, by: FormId, amount: f64) -> f64 {
    let Some(s) = state.coop.live.scaling else {
        return amount;
    };
    if who != PLAYER_REF && player_side(by) {
        amount / f64::from(s.toughness)
    } else if who == PLAYER_REF && !player_side(by) {
        amount * f64::from(s.damage_to_players)
    } else {
        amount
    }
}

/// `combat::hurt` in a co-op session. Returns whether `who` died now.
pub(crate) fn hurt_hooked(
    order: &LoadOrder,
    state: &mut GameState,
    who: FormId,
    amount: f64,
    by: FormId,
) -> bool {
    if let Some(p) = player_of(who) {
        // a remote player's health is theirs; the host judges the blow
        return super::combat_live::puppet_hurt(state, p, amount, by);
    }
    if state.coop.live.replaying {
        return crate::combat::hurt_core(order, state, who, amount, by);
    }
    // a player's deliberate hits on a protected ally stop (friendly fire guard)
    if who != PLAYER_REF {
        if let Some(slot) = super::campaign::attribution::slot_of(by, state.active_slot()) {
            if !super::campaign::ffguard::allow_hit(order, state, slot, who) {
                return false;
            }
        }
    }
    let mut amount = scaled_damage(state, who, by, amount);
    if who == PLAYER_REF {
        // The local player goes down instead of dying; the party revives them.
        let left = crate::combat::health(order, state, who).unwrap_or(f64::MAX);
        if amount >= left {
            amount = (left - 1.0).max(0.0);
            // whose down, and under which quest's row (a collar's script)
            super::campaign::downed::health_out(state);
        }
        return amount > 0.0 && crate::combat::hurt_core(order, state, who, amount, by);
    }
    if state.coop.authority == Authority::Guest {
        state
            .coop
            .live
            .pending_hits
            .push(PendingHit { who, amount, by });
        return false;
    }
    let before = state.damage.get(&who).copied().unwrap_or(0.0);
    let killed = crate::combat::hurt_core(order, state, who, amount, by);
    let dealt = state.damage.get(&who).copied().unwrap_or(0.0) - before;
    if dealt > 0.0 {
        state.coop.live.hurts.push(HurtRecord {
            who,
            amount: dealt,
            by,
        });
    }
    killed
}

/// Experience for a kill, thinned by the party size.
pub(crate) fn xp_scale(state: &GameState, amount: f64) -> f64 {
    state.coop.live.scaling.map_or(amount, |s| {
        let thinned = (amount * f64::from(s.reward)).floor();
        // a kill that was worth something never thins to nothing
        if thinned < 1.0 && amount >= 1.0 {
            1.0
        } else {
            thinned
        }
    })
}

/// Fast travel in a co-op session: unless the party allowed this marker the
/// trip becomes a request. `Some(reason)` refuses the trip for now.
pub(crate) fn travel_gate(state: &mut GameState, marker: FormId) -> Option<String> {
    if state.coop.live.travel_allowed == Some(marker) {
        state.coop.live.travel_allowed = None;
        return None;
    }
    if !state.coop.live.travel_requests.contains(&marker) {
        state.coop.live.travel_requests.push(marker);
    }
    Some("Waiting for the party.".to_string())
}

impl GameState {
    /// The live facts (zero cost accessor).
    pub fn live(&self) -> &LiveFacts {
        &self.coop.live
    }

    /// Queues a container action of the local player (nothing in single
    /// player).
    pub fn coop_loot(&mut self, op: LootOp) {
        if self.coop.authority != Authority::Single {
            self.coop.live.loot_ops.push(op);
        }
    }

    /// Mutable live facts.
    pub fn live_mut(&mut self) -> &mut LiveFacts {
        &mut self.coop.live
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn puppet_block_round_trips() {
        for p in [0u8, 1, 7, 255] {
            assert_eq!(player_of(puppet_of(PlayerId(p))), Some(PlayerId(p)));
        }
        assert_eq!(player_of(FormId(0x14)), None);
        assert_eq!(player_of(FormId(0xFF00_0001)), None);
    }

    #[test]
    fn scaling_applies_by_side() {
        let mut s = GameState::default();
        s.coop.live.scaling = Some(Scaling::for_party(3, coop_core::rules::Mode::CoopBalanced));
        let enemy = FormId(0x500);
        assert!(scaled_damage(&s, enemy, PLAYER_REF, 20.0) < 20.0);
        assert!(scaled_damage(&s, PLAYER_REF, enemy, 20.0) > 20.0);
        assert_eq!(scaled_damage(&s, enemy, FormId(0x501), 20.0), 20.0);
        assert!(xp_scale(&s, 100.0) < 100.0);
    }
}

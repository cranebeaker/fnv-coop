//! Kill and hit attribution by real player slot.
//!
//! The host ledger already knows who dealt every point (`ActorMsg::Damaged`
//! carries the attacker, `Died` the killer). A guest's hit is applied on the
//! host world as an anonymous verdict, so `kill_share` never saw it and a
//! guest earned nothing. The live host adapter feeds those two messages to
//! [`observe`]; [`settle`] then pays experience to every contributor by slot
//! through the policy gate, karma once for a guest's kill, and the kill
//! statistics to the killer.

use coop_core::actors::ActorMsg;
use coop_core::proto::Msg;
use coop_policy::{Decision, Event, EventKind, Origin, RewardKind};
use esm::{FormId, LoadOrder};

use crate::coop::apply::Ctx;
use crate::dialogue::PLAYER_REF;
use crate::scripting::GameState;

/// The slot a combat form stands for: the live player row owner, or a puppet.
pub fn slot_of(f: FormId, live_slot: u8) -> Option<u8> {
    if f == PLAYER_REF {
        Some(live_slot)
    } else {
        crate::coop::facts::player_of(f).map(|p| p.0)
    }
}

/// A stable id for the xp of one kill.
pub fn kill_reward_id(victim: FormId) -> u32 {
    0x4B49_4C4C ^ victim.0.rotate_left(7)
}

/// Feeds one host-side actor message to the tally; settles on `Died`.
pub fn observe(cx: Ctx, gs: &mut GameState, msg: &Msg) {
    gs.coop.campaign.attribution_on = true;
    let Msg::Actor(a) = msg else { return };
    match a {
        ActorMsg::Damaged {
            actor, by, amount, ..
        } => {
            let victim = FormId(actor.0);
            let camp = &mut gs.coop.campaign;
            // an NPC's blow (Doc Mitchell on a gecko) pays nobody
            let Some(by) = by else {
                camp.npc_blow.insert(victim);
                return;
            };
            camp.npc_blow.remove(&victim);
            let e = camp.tally.entry(victim).or_default();
            *e.entry(by.0).or_insert(0.0) += f64::from(*amount);
        }
        ActorMsg::Died { actor, killer } => {
            let victim = FormId(actor.0);
            let npc = gs.coop.campaign.npc_blow.remove(&victim);
            let killer = killer.filter(|k| !(npc && k.is_host()));
            settle(cx.order, gs, victim, killer.map(|k| k.0));
        }
        _ => {}
    }
}

/// Pays a kill. Returns the experience given per slot.
pub fn settle(
    order: &LoadOrder,
    gs: &mut GameState,
    victim: FormId,
    killer: Option<u8>,
) -> Vec<(u8, f64)> {
    if !gs.coop.campaign.settled.insert(victim) {
        return Vec::new();
    }
    let dmg = gs.coop.campaign.tally.remove(&victim).unwrap_or_default();
    gs.coop.campaign.stats.kills_settled += 1;
    let live = gs.active_slot();
    if victim == PLAYER_REF || crate::coop::facts::player_of(victim).is_some() {
        return Vec::new();
    }
    let total: f64 = dmg.values().sum();
    // the killer counts even when the tally missed their last hit
    let mut who: Vec<u8> = dmg
        .iter()
        .filter(|(_, d)| **d > 0.0)
        .map(|(s, _)| *s)
        .collect();
    if let Some(k) = killer {
        // who killed whom, by slot (the party's `IsKiller`)
        gs.coop.campaign.killers.insert(victim, k);
        if !who.contains(&k) {
            who.push(k);
        }
    }
    who.sort_unstable();
    let amount = crate::experience::kill_award(order, gs, victim, total);
    let mut out = Vec::new();
    if amount > 0.0 {
        let allowed = match gs.coop.gate.take() {
            Some(g) => {
                let ev = Event {
                    quest: 0,
                    origin: Origin::Host,
                    kind: EventKind::Reward {
                        reward_id: kill_reward_id(victim),
                        what: RewardKind::Xp,
                    },
                };
                let d = g.decide(gs, &ev);
                let r = match d {
                    Decision::Replicate { recipients, .. } => Some(recipients),
                    _ => None,
                };
                gs.coop.gate = Some(g);
                r
            }
            None => None,
        };
        for slot in &who {
            if let Some(m) = allowed {
                if !m.contains(*slot) {
                    continue;
                }
            }
            if !gs.coop.bank.enabled() && *slot != live {
                continue;
            }
            let id = kill_reward_id(victim);
            let fresh = match gs.coop.gate.as_mut() {
                Some(g) => g.claim(id, *slot),
                None => true,
            };
            if !fresh {
                continue;
            }
            gs.with_player(*slot, |g| crate::experience::reward(order, g, amount));
            gs.coop.campaign.paid.push((victim, *slot, amount));
            out.push((*slot, amount));
        }
    }
    if let Some(k) = killer {
        // the host's own kills already count in `combat::hurt_core`
        if k != live && (gs.coop.bank.enabled()) {
            let creature = crate::combat::is_creature(order, victim);
            let karma = crate::reputation::kill_karma(order, gs, victim);
            gs.with_player(k, |g| {
                let kind = if creature {
                    crate::stats::CREATURES_KILLED
                } else {
                    crate::stats::PEOPLE_KILLED
                };
                crate::stats::bump(g, kind, 1);
                crate::stats::bump(g, crate::stats::TOTAL_THINGS_KILLED, 1);
            });
            if karma != 0 {
                crate::reputation::reward_karma(order, gs, karma);
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slots_of_combat_forms() {
        assert_eq!(slot_of(PLAYER_REF, 3), Some(3));
        assert_eq!(
            slot_of(
                crate::coop::facts::puppet_of(coop_core::ids::PlayerId(2)),
                0
            ),
            Some(2)
        );
        assert_eq!(slot_of(FormId(0x1234), 0), None);
    }

    #[test]
    fn reward_ids_differ_per_victim() {
        assert_ne!(kill_reward_id(FormId(1)), kill_reward_id(FormId(2)));
    }
}

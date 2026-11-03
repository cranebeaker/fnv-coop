//! Party aware conditions (G23, G37).
//!
//! A condition on "the player" (`GetItemCount`, `GetLevel`, `GetDistance`,
//! `GetInCell` ...) asks one player in single player. In a party the row of
//! the quest being evaluated says how to read it
//! (`party_cond: function -> reducer`): the initiator, any player, all of
//! them, the sum, the minimum or maximum, the host. [`facts_hook`] is called
//! from `Facts::value` while a quest context is set and answers from the
//! players' shadows without swapping anything.

use coop_policy::{Phase, PlayerMask, Reducer};
use esm::FormId;

use crate::coop::shadow::PlayerShadow;
use crate::scripting::{Facts, GameState, Value};

/// One player's value of a condition function, from their shadow. `slot` is
/// that player's slot (`IsKiller` asks who killed).
pub fn shadow_value(
    gs: &GameState,
    slot: u8,
    sh: &PlayerShadow,
    name: &str,
    args: &[Value],
) -> Option<f64> {
    let arg = |i: usize| args.get(i).cloned().unwrap_or(Value::Number(0.0));
    let b = |x: bool| if x { 1.0 } else { 0.0 };
    Some(match name {
        // the victim (the parameter) is dead and this player's blow killed it
        "IsKiller" => {
            let victim = arg(0).form();
            b(gs.dead.contains(&victim) && gs.coop.campaign.killers.get(&victim) == Some(&slot))
        }
        "GetItemCount" => f64::from(sh.items.get(&arg(0).form()).copied().unwrap_or(0)),
        "GetLevel" => f64::from(sh.level),
        "GetAV" | "GetActorValue" | "GetBaseActorValue" => sh
            .actor_values
            .get(&(arg(0).number() as u16))
            .copied()
            .unwrap_or(0.0),
        "HasPerk" => b(sh.perks.contains(&arg(0).form())),
        "GetPlayerTeammate" => b(sh.teammates.contains(&arg(0).form())),
        "GetInCell" => b(sh.cell == Some(arg(0).form())),
        "GetInWorldspace" => b(sh.world == Some(arg(0).form())),
        "GetDistance" => {
            let p = sh.position?;
            let at = [
                f32::from_bits(p[0]),
                f32::from_bits(p[1]),
                f32::from_bits(p[2]),
            ];
            let (o, _) = gs.positions.get(&arg(0).form())?;
            let d =
                ((at[0] - o[0]).powi(2) + (at[1] - o[1]).powi(2) + (at[2] - o[2]).powi(2)).sqrt();
            f64::from(d)
        }
        _ => return None,
    })
}

/// Host: copies every other player's newest cell, position and heading from
/// the avatar table into their parked shadow, so that `GetDistance` and
/// `GetInCell` read where they are now, not where they joined.
pub fn refresh_positions(gs: &mut GameState, players: &crate::coop::players::PlayerRegistry) {
    let active = gs.coop.bank.active;
    for (p, r) in players.iter() {
        if p.0 == active || !r.seen {
            continue;
        }
        let Some(sh) = gs.coop.bank.parked.get_mut(&p.0) else {
            continue;
        };
        if r.cell != 0 {
            sh.cell = Some(FormId(r.cell));
        }
        sh.position = Some(r.position.map(f32::to_bits));
        sh.heading = r.heading.to_bits();
    }
}

/// The reducer a quest's row names for a function (default: the initiator).
pub fn reducer_for(gs: &GameState, quest: FormId, name: &str) -> Reducer {
    gs.coop
        .gate
        .as_ref()
        .and_then(|g| g.row(quest))
        .and_then(|r| r.party_cond.get(name))
        .and_then(|s| Reducer::parse(s))
        .unwrap_or(Reducer::Initiator)
}

/// Values of `name` for every present player.
pub fn values(gs: &GameState, name: &str, args: &[Value]) -> Vec<(u8, f64)> {
    let Some(g) = gs.coop.gate.as_ref() else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for slot in 0..8u8 {
        if !g.present.contains(slot) {
            continue;
        }
        if let Some(sh) = gs.shadow_of(slot) {
            if let Some(v) = shadow_value(gs, slot, &sh, name, args) {
                out.push((slot, v));
            }
        }
    }
    out
}

/// The party answer for a function in a quest's context, or `None` when the
/// row leaves it to the initiator (then the live rows answer).
pub fn party_value(
    gs: &GameState,
    quest: FormId,
    name: &str,
    args: &[Value],
    phase: Phase,
) -> Option<f64> {
    let red = reducer_for(gs, quest, name);
    if red == Reducer::Initiator {
        return None;
    }
    let g = gs.coop.gate.as_ref()?;
    let initiator = g.session(quest).and_then(|s| s.initiator).or(g.initiator());
    red.reduce(g.present, initiator, phase, &values(gs, name, args))
}

/// `Facts::value` hook: only while a quest context is set.
pub fn facts_hook(f: &Facts, name: &str, args: &[Value]) -> Option<f64> {
    let quest = f.state.coop.campaign.cond_quest?;
    party_value(f.state, quest, name, args, Phase::Entry)
}

/// Players that satisfy `name` (non-zero), for scenes that need to know who.
pub fn who(gs: &GameState, name: &str, args: &[Value]) -> PlayerMask {
    values(gs, name, args)
        .into_iter()
        .filter(|(_, v)| *v != 0.0)
        .fold(PlayerMask::NONE, |m, (s, _)| m.or(PlayerMask::of(s)))
}

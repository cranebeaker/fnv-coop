//! Consent for one-way forks (G08).
//!
//! A row with `consent` lists the topics and message boxes where the story
//! forks for good. The script that would take the fork asks [`check`]: the
//! answer is `Proceed` (the votes came out as the row's mode wants),
//! `Wait` (hold the script, the party is voting) or `Denied`. Votes arrive
//! through [`vote`]; the gate's clocks time the vote out.

use std::collections::BTreeMap;

use coop_policy::{Decision, PlayerMask, Reject};
use esm::{FormId, LoadOrder};

use super::owners::resolve;
use crate::scripting::GameState;

/// Answer to a script asking to take a fork.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fork {
    /// Take it.
    Proceed,
    /// Hold the script; players are voting.
    Wait,
    /// The party said no (or the time ran out).
    Denied(Reject),
}

/// Forks being voted on.
#[derive(Clone, Debug, Default)]
pub struct Forks {
    /// Quest to the choice number put to the party.
    pub open: BTreeMap<FormId, u32>,
    /// Finished forks: (quest, choice, approved).
    pub done: Vec<(FormId, u32, bool)>,
}

/// The quest whose consent row lists this topic (`plugin:id` or editor id).
pub fn quest_for_topic(order: &LoadOrder, gs: &GameState, topic: FormId) -> Option<FormId> {
    let g = gs.coop.gate.as_ref()?;
    g.table().rows().iter().find_map(|r| {
        let c = r.consent.as_ref()?;
        c.topics.iter().any(|t| resolve(order, t) == Some(topic)).then_some(g.lid(r.id))
    })
}

/// The quest whose consent row lists this message box (editor id).
pub fn quest_for_box(order: &LoadOrder, gs: &GameState, message: FormId) -> Option<FormId> {
    let g = gs.coop.gate.as_ref()?;
    g.table().rows().iter().find_map(|r| {
        let c = r.consent.as_ref()?;
        c.boxes.iter().any(|b| resolve(order, b) == Some(message)).then_some(g.lid(r.id))
    })
}

/// A script asks to take fork `choice` of `quest`.
pub fn check(gs: &mut GameState, quest: FormId, choice: u32) -> Fork {
    let Some(has) = gs.coop.gate.as_ref().map(|g| g.row(quest).is_some_and(|r| r.consent.is_some())) else {
        return Fork::Proceed;
    };
    if !has {
        return Fork::Proceed;
    }
    // a fork already decided stays decided (one-way)
    if let Some((_, _, ok)) = gs.coop.campaign.forks.done.iter().rev().find(|(q, c, _)| *q == quest && *c == choice) {
        return if *ok { Fork::Proceed } else { Fork::Denied(Reject::Declined) };
    }
    if !gs.coop.campaign.forks.open.contains_key(&quest) {
        gs.coop.campaign.forks.open.insert(quest, choice);
        if let Some(g) = gs.coop.gate.as_mut() {
            g.begin_vote(quest);
        }
    }
    let d = gs.with_gate(Decision::Apply, |g, gs| g.choice(gs, quest, choice));
    match d {
        Decision::DeferToBarrier { .. } => Fork::Wait,
        Decision::Reject { reason } => {
            gs.coop.campaign.forks.open.remove(&quest);
            gs.coop.campaign.forks.done.push((quest, choice, false));
            gs.coop.live.fork_result = Some((choice, false));
            Fork::Denied(reason)
        }
        _ => {
            gs.coop.campaign.forks.open.remove(&quest);
            gs.coop.campaign.forks.done.push((quest, choice, true));
            gs.coop.live.fork_result = Some((choice, true));
            Fork::Proceed
        }
    }
}

/// A player votes on the open fork of `quest`.
pub fn vote(gs: &mut GameState, quest: FormId, slot: u8, yes: bool) {
    if !gs.coop.campaign.forks.open.contains_key(&quest) {
        return;
    }
    if let Some(g) = gs.coop.gate.as_mut() {
        if g.present.contains(slot) {
            g.vote(quest, slot, yes);
        }
    }
}

/// Players that still have to vote on `quest`.
pub fn waiting_for(gs: &GameState, quest: FormId) -> PlayerMask {
    let Some(g) = gs.coop.gate.as_ref() else { return PlayerMask::NONE };
    let s = g.session(quest);
    let answered = s.map_or(PlayerMask::NONE, |s| s.votes_yes.or(s.votes_no));
    g.present.minus(answered)
}

/// Re-asks every open fork (the clocks may have run it out); returns those
/// that finished: (quest, choice, approved).
pub fn poll(gs: &mut GameState) -> Vec<(FormId, u32, bool)> {
    let open: Vec<(FormId, u32)> = gs.coop.campaign.forks.open.iter().map(|(q, c)| (*q, *c)).collect();
    let mut out = Vec::new();
    for (q, c) in open {
        match check(gs, q, c) {
            Fork::Wait => {}
            Fork::Proceed => out.push((q, c, true)),
            Fork::Denied(_) => out.push((q, c, false)),
        }
    }
    out
}

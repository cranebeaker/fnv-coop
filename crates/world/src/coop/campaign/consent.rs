//! Consent for one-way forks (G08).
//!
//! A row with `consent` lists the topics and message boxes where the story
//! forks for good. The script that would take the fork asks [`check`]: the
//! answer is `Proceed` (the votes came out as the row's mode wants),
//! `Wait` (hold the script, the party is voting) or `Denied`. Votes arrive
//! through [`vote`]; the gate's clocks time the vote out.

use std::collections::BTreeMap;

use coop_core::authority::AuthorityTable;
use coop_core::ids::PlayerId;
use coop_core::proto::Msg;
use coop_core::rules::{Prompt, RulesClient, RulesEvent, RulesMsg, VoteKind};
use coop_core::sync::HostLive;
use coop_policy::{Decision, PlayerMask, Reject};
use esm::{FormId, LoadOrder};

use super::owners::resolve;
use crate::coop::Authority;
use crate::scripting::{Event, GameState};

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
    /// Party message boxes (`ShowMessage` of a box a consent row lists).
    pub boxes: Boxes,
}

/// The quest whose consent row lists this topic (`plugin:id` or editor id).
pub fn quest_for_topic(order: &LoadOrder, gs: &GameState, topic: FormId) -> Option<FormId> {
    let g = gs.coop.gate.as_ref()?;
    g.table().rows().iter().find_map(|r| {
        let c = r.consent.as_ref()?;
        c.topics
            .iter()
            .any(|t| resolve(order, t) == Some(topic))
            .then_some(g.lid(r.id))
    })
}

/// The quest whose consent row lists this message box (editor id).
pub fn quest_for_box(order: &LoadOrder, gs: &GameState, message: FormId) -> Option<FormId> {
    let g = gs.coop.gate.as_ref()?;
    g.table().rows().iter().find_map(|r| {
        let c = r.consent.as_ref()?;
        c.boxes
            .iter()
            .any(|b| resolve(order, b) == Some(message))
            .then_some(g.lid(r.id))
    })
}

/// A script asks to take fork `choice` of `quest`.
pub fn check(gs: &mut GameState, quest: FormId, choice: u32) -> Fork {
    let Some(has) = gs
        .coop
        .gate
        .as_ref()
        .map(|g| g.row(quest).is_some_and(|r| r.consent.is_some()))
    else {
        return Fork::Proceed;
    };
    if !has {
        return Fork::Proceed;
    }
    // a fork already decided stays decided (one-way)
    if let Some((_, _, ok)) = gs
        .coop
        .campaign
        .forks
        .done
        .iter()
        .rev()
        .find(|(q, c, _)| *q == quest && *c == choice)
    {
        return if *ok {
            Fork::Proceed
        } else {
            Fork::Denied(Reject::Declined)
        };
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
    let Some(g) = gs.coop.gate.as_ref() else {
        return PlayerMask::NONE;
    };
    let s = g.session(quest);
    let answered = s.map_or(PlayerMask::NONE, |s| s.votes_yes.or(s.votes_no));
    g.present.minus(answered)
}

/// Re-asks every open fork (the clocks may have run it out); returns those
/// that finished: (quest, choice, approved).
pub fn poll(gs: &mut GameState) -> Vec<(FormId, u32, bool)> {
    let open: Vec<(FormId, u32)> = gs
        .coop
        .campaign
        .forks
        .open
        .iter()
        .map(|(q, c)| (*q, *c))
        .collect();
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

// ---- party message boxes (G06) -------------------------------------------
//
// A script's `ShowMessage` of a box a consent row lists makes it a party box
// ([`box_shown`]). The host's player presses a button as always; the
// script's `GetButtonPressed` then asks [`box_button`]: the press is the
// host player's yes on a fork of the row's quest (the button is the
// choice), the question goes to the other players as a consent vote of the
// rules ([`box_frame`]: their prompt, `coop vote yes|no`), their casts are
// mirrored into the gate, and the button stays held (`GetButtonPressed`
// gives -1) until the gate decides. A refused button is dropped and the box
// comes back. Single player and guests pay one authority test.

/// A box a consent row lists, shown on the host.
#[derive(Clone, Debug)]
pub struct PartyBox {
    /// The consent row's quest.
    pub quest: FormId,
    /// The message.
    pub message: FormId,
    /// Whose box it is (the script that showed it).
    pub owner: FormId,
    /// The box as shown, shown again when the party refuses a button.
    pub shown: Option<Event>,
    /// The button the party votes on now.
    pub choice: Option<u32>,
}

/// The rules vote that carries a party box's question to the other players.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BoxVote {
    /// The consent row's quest.
    pub quest: FormId,
    /// The button put to the party.
    pub choice: u32,
    /// The rules vote, once it opened.
    pub id: Option<u32>,
}

/// Counters for the state dump (`campaign.boxes`).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct BoxStats {
    /// Messages scripts showed here (host).
    pub shown: u32,
    /// Of those, party boxes.
    pub party: u32,
    /// Party votes this machine saw open (host: buttons put to the party;
    /// guest: vote prompts for a party box).
    pub votes: u32,
    /// `GetButtonPressed` asks held while the party voted (host).
    pub held: u32,
    /// Buttons given to the script after the party agreed (host).
    pub released: u32,
    /// Buttons the party refused (host).
    pub refused: u32,
    /// Other players' casts mirrored into the gate (host).
    pub casts: u32,
}

/// Party boxes of one machine.
#[derive(Clone, Debug, Default)]
pub struct Boxes {
    /// The party box on screen (host).
    pub open: Option<PartyBox>,
    /// Questions put to the party and not settled yet (host).
    pub votes: Vec<BoxVote>,
    /// Counters.
    pub stats: BoxStats,
    cursor: usize,
    seen: Option<u32>,
}

/// What `GetButtonPressed` does with a pressed button.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Press {
    /// Give it to the script.
    Give,
    /// Keep it; the script gets -1 (the party is voting).
    Hold,
    /// Drop it; the script gets -1 (the party refused it).
    Drop,
}

/// `ShowMessage` hook: on the host a box a consent row lists becomes the
/// party box; any other message replaces it (`ShowMessage` takes the
/// button over, `005b4630`).
pub fn box_shown(order: &LoadOrder, gs: &mut GameState, message: FormId, owner: FormId) {
    if gs.coop.authority != Authority::Host {
        return;
    }
    let quest = quest_for_box(order, gs, message);
    let shown = match gs.events.last() {
        Some(e @ Event::Message { .. }) => Some(e.clone()),
        _ => None,
    };
    let b = &mut gs.coop.campaign.forks.boxes;
    b.stats.shown += 1;
    b.open = quest.map(|quest| PartyBox {
        quest,
        message,
        owner,
        shown,
        choice: None,
    });
    if b.open.is_some() {
        b.stats.party += 1;
    }
}

/// `GetButtonPressed` hook: the owner of the box asks for the pressed
/// button `b`. A party box's button waits for the party's decision.
pub fn box_button(gs: &mut GameState, b: i32) -> Press {
    if gs.coop.authority != Authority::Host {
        return Press::Give;
    }
    let Some(pb) = gs.coop.campaign.forks.boxes.open.clone() else {
        return Press::Give;
    };
    let (quest, choice) = (pb.quest, b.max(0) as u32);
    let mut fork = check(gs, quest, choice);
    if pb.choice != Some(choice) {
        if let Some(p) = gs.coop.campaign.forks.boxes.open.as_mut() {
            p.choice = Some(choice);
        }
        if fork == Fork::Wait {
            // the host's player pressed it: their yes; the others are asked
            vote(gs, quest, PlayerId::HOST.0, true);
            let bx = &mut gs.coop.campaign.forks.boxes;
            bx.votes.push(BoxVote {
                quest,
                choice,
                id: None,
            });
            bx.stats.votes += 1;
            fork = check(gs, quest, choice);
        }
    }
    let bx = &mut gs.coop.campaign.forks.boxes;
    match fork {
        Fork::Wait => {
            bx.stats.held += 1;
            Press::Hold
        }
        Fork::Proceed => {
            bx.stats.released += 1;
            bx.open = None;
            Press::Give
        }
        Fork::Denied(_) => {
            bx.stats.refused += 1;
            // the box comes back: the player picks again
            let again = bx.open.as_mut().and_then(|p| {
                p.choice = None;
                p.shown.clone()
            });
            if let Some(e) = again {
                gs.events.push(e);
            }
            Press::Drop
        }
    }
}

/// Host, every frame after the rules bridge: each party box question goes to
/// the other players as a consent vote, their casts reach the gate, and the
/// vote closes for everyone once the gate decided. Returns sends.
pub fn box_frame(
    gs: &mut GameState,
    live: &mut HostLive,
    auth: &mut AuthorityTable,
    events: &[RulesEvent],
    now_ms: u64,
) -> Vec<(PlayerId, Msg)> {
    let mut outs = Vec::new();
    let from = gs.coop.campaign.forks.boxes.cursor.min(events.len());
    gs.coop.campaign.forks.boxes.cursor = events.len();
    if gs.coop.campaign.forks.boxes.votes.is_empty() {
        return outs;
    }
    for e in &events[from..] {
        match e {
            RulesEvent::Cast { id, player, yes } => {
                let hit = gs
                    .coop
                    .campaign
                    .forks
                    .boxes
                    .votes
                    .iter()
                    .find(|v| v.id == Some(*id))
                    .map(|v| v.quest);
                if let (Some(q), false) = (hit, player.is_host()) {
                    vote(gs, q, player.0, *yes);
                    gs.coop.campaign.forks.boxes.stats.casts += 1;
                }
            }
            // the rules closed it themselves (a timeout, the proposer left)
            RulesEvent::Vote(o) if o.kind == VoteKind::Consent => {
                for v in gs.coop.campaign.forks.boxes.votes.iter_mut() {
                    if v.id == Some(o.id) {
                        v.id = None;
                    }
                }
            }
            _ => {}
        }
    }
    let mut keep = Vec::new();
    for mut v in std::mem::take(&mut gs.coop.campaign.forks.boxes.votes) {
        let f = &gs.coop.campaign.forks;
        if f.open.get(&v.quest) != Some(&v.choice) {
            let ok = f
                .done
                .iter()
                .rev()
                .find(|(q, c, _)| *q == v.quest && *c == v.choice)
                .is_some_and(|d| d.2);
            if let Some(id) = v.id {
                for o in live.rules.settle_vote(id, ok) {
                    outs.extend(live.route_extern(o.aud, Msg::Rules(o.msg)));
                }
            }
            continue;
        }
        if v.id.is_none() && live.rules.votes.open_id().is_none() {
            let ask = Msg::Rules(RulesMsg::VoteRequest {
                kind: VoteKind::Consent,
                param: v.choice,
            });
            outs.extend(live.on_msg(PlayerId::HOST, ask, now_ms, auth));
            v.id = live.rules.votes.open_id();
        }
        keep.push(v);
    }
    gs.coop.campaign.forks.boxes.votes = keep;
    outs
}

/// Guest, every frame: counts the party box votes put to this player.
pub fn box_prompts(gs: &mut GameState, client: &RulesClient) {
    let Some(Prompt::Vote {
        id,
        kind: VoteKind::Consent,
        ..
    }) = client.prompt
    else {
        return;
    };
    let b = &mut gs.coop.campaign.forks.boxes;
    if b.seen != Some(id) {
        b.seen = Some(id);
        b.stats.votes += 1;
    }
}

/// The `campaign.boxes` object of the state dump.
pub fn boxes_json(gs: &GameState) -> String {
    let s = &gs.coop.campaign.forks.boxes.stats;
    format!(
        "{{\"shown\":{},\"party\":{},\"votes\":{},\"held\":{},\"released\":{},\"refused\":{},\"casts\":{}}}",
        s.shown, s.party, s.votes, s.held, s.released, s.refused, s.casts
    )
}

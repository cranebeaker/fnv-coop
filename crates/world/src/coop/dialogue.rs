//! Dialogue sessions through `DialogueHost` / `DialogueClient`.
//!
//! The initiator's machine picks lines (`pick` with an INFO form id); the
//! host runs each picked line's result scripts exactly once with the real
//! `Runner` (`RunBegin` / `RunEnd`); everyone, the initiator included, plays
//! the picked lines from the `lines` queue in `GameState.coop.live` and
//! reports `line_done`. Guests never run a line's effects: `line_begins` is a
//! no-op on a guest and the `Runner` refuses scripts there.

use std::collections::BTreeSet;

use coop_core::dialogue::{DialogueClient, DialogueEvent};
use coop_core::ids::{FormId as CForm, PlayerId};
use coop_core::proto::Msg;
use coop_core::sync::HostLive;
use esm::FormId;

use super::apply::Ctx;
use crate::dialogue::{line_begins, menu_runs_begin_script, menu_runs_end_script, Info};
use crate::scripting::{GameState, Runner};

fn cf(f: FormId) -> CForm {
    CForm(f.0)
}

/// Dialogue bookkeeping for one machine.
#[derive(Default)]
pub struct DialogueBridge {
    shown: BTreeSet<(u32, u32)>,
    in_combat: BTreeSet<FormId>,
    died: BTreeSet<FormId>,
    /// Begin scripts run (host).
    pub begin_runs: u32,
    /// End scripts run (host).
    pub end_runs: u32,
    /// Sessions closed, with the NPC (host).
    pub closed: Vec<(u32, FormId)>,
    /// The host's own game runs the host player's lines itself (the
    /// viewer's dialogue menu): their result scripts must not run a second
    /// time from the session.
    pub native_host: bool,
    native_sessions: BTreeSet<u32>,
    /// Who picked in each open session (the end script runs as them).
    initiators: std::collections::BTreeMap<u32, PlayerId>,
}

impl DialogueBridge {
    /// A fresh bridge.
    pub fn new() -> Self {
        Self::default()
    }

    /// Initiator: asks to talk to `npc`.
    pub fn open(&self, client: &mut DialogueClient, npc: FormId) -> Msg {
        Msg::Dialogue(client.open(cf(npc)).1)
    }

    /// Initiator: picks a line by its INFO form.
    pub fn pick(&self, client: &mut DialogueClient, info: FormId) -> Option<Msg> {
        client.pick(info.0).map(Msg::Dialogue)
    }

    /// Everyone: the lines of the step are played.
    pub fn line_done(&self, client: &mut DialogueClient) -> Option<Msg> {
        client.line_done().map(Msg::Dialogue)
    }

    /// Initiator: leaves the conversation.
    pub fn close(&self, client: &mut DialogueClient) -> Option<Msg> {
        client.close().map(Msg::Dialogue)
    }

    /// Every machine, every frame: queues the picked line for the viewer to
    /// play (once per step).
    pub fn poll_lines(&mut self, gs: &mut GameState, client: &DialogueClient) {
        let Some(v) = client.view() else { return };
        let Some(choice) = v.playing else { return };
        if self.shown.insert((v.session, v.step)) {
            gs.coop.live.lines.push((v.step, FormId(choice)));
        }
        if self.shown.len() > 256 {
            let first = *self.shown.iter().next().expect("non-empty");
            self.shown.remove(&first);
        }
    }

    /// Host, every frame: feeds combat and deaths to the session table, then
    /// runs the scripts the sessions asked for. Returns sends.
    pub fn host_frame(&mut self, cx: Ctx, gs: &mut GameState, live: &mut HostLive) -> Vec<(PlayerId, Msg)> {
        let mut outs = Vec::new();
        let fighting: BTreeSet<FormId> = gs.combat.iter().flat_map(|(a, b)| [*a, *b]).collect();
        for n in fighting.difference(&self.in_combat.clone()) {
            live.dialogue.set_in_combat(cf(*n), true);
        }
        for n in self.in_combat.difference(&fighting.clone()) {
            live.dialogue.set_in_combat(cf(*n), false);
        }
        self.in_combat = fighting;
        let dead: Vec<FormId> = gs.dead.iter().copied().filter(|d| !self.died.contains(d)).collect();
        for d in dead {
            self.died.insert(d);
            for o in live.dialogue.npc_died(cf(d)) {
                outs.extend(live.route_extern(o.aud, Msg::Dialogue(o.msg)));
            }
        }
        for e in live.dialogue.take_events() {
            match e {
                DialogueEvent::RunBegin { session, npc, choice, initiator, .. } => {
                    let npc = FormId(npc.0);
                    if self.native_host && initiator.is_host() {
                        self.native_sessions.insert(session);
                        continue;
                    }
                    self.initiators.insert(session, initiator);
                    if let Some(info) = Info::load(cx.order, FormId(choice)) {
                        // the host runs the line as the player who picked it
                        super::campaign::producers::as_initiator(gs, initiator.0, info.quest, |gs| {
                            line_begins(gs, &info, npc);
                            if menu_runs_begin_script(&info) {
                                run(cx, gs, info.begin_script.as_deref(), npc);
                            }
                        });
                    }
                    self.begin_runs += 1;
                }
                DialogueEvent::RunEnd { session, npc, choice, .. } => {
                    let npc = FormId(npc.0);
                    if self.native_sessions.contains(&session) {
                        continue;
                    }
                    if let Some(info) = Info::load(cx.order, FormId(choice)) {
                        let who = self.initiators.get(&session).map_or(0, |p| p.0);
                        super::campaign::producers::as_initiator(gs, who, info.quest, |gs| {
                            if menu_runs_end_script(&info) {
                                run(cx, gs, info.end_script.as_deref(), npc);
                            }
                            super::campaign::producers::ready_if_listed(cx, gs, &info);
                        });
                    }
                    self.end_runs += 1;
                }
                DialogueEvent::Closed { session, npc, .. } => {
                    self.initiators.remove(&session);
                    self.closed.push((session, FormId(npc.0)))
                }
            }
        }
        outs
    }
}

/// Runs one result script on the speaker with the real runner.
fn run(cx: Ctx, gs: &mut GameState, source: Option<&str>, speaker: FormId) {
    if let Some(src) = source.filter(|s| !s.trim().is_empty()) {
        Runner::new(cx.order, cx.scripts, gs).run_source(src, Some(speaker), Some(speaker));
    }
}

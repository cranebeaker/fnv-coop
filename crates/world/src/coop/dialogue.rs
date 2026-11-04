//! Dialogue sessions through `DialogueHost` / `DialogueClient`.
//!
//! The initiator's machine picks lines (`pick` with an INFO form id); the
//! host runs each picked line's result scripts exactly once with the real
//! `Runner` (`RunBegin` / `RunEnd`); everyone, the initiator included, plays
//! the picked lines from the `lines` queue in `GameState.coop.live` and
//! reports `line_done`. Guests never run a line's effects: `line_begins` is a
//! no-op on a guest and the `Runner` refuses scripts there.

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use coop_core::authority::AuthorityTable;
use coop_core::dialogue::{DialogueClient, DialogueEvent, DialogueMsg};
use coop_core::ids::{FormId as CForm, PlayerId};
use coop_core::proto::Msg;
use coop_core::scene::{
    BarrierRule, CameraMode, EndReason, Locks, Mask, SceneKind, SceneSpec, SkipRule,
};
use coop_core::sync::HostLive;
use esm::FormId;

use super::apply::Ctx;
use crate::dialogue::{line_begins, menu_runs_begin_script, menu_runs_end_script, Info};
use crate::scripting::{GameState, Runner};

fn cf(f: FormId) -> CForm {
    CForm(f.0)
}

/// Longest a script's conversation waits for the host's menu to open its
/// session (the speaker walks up first) before its dialogue scene ends.
pub const FORCED_OPEN_MS: u64 = 60_000;
/// Longest a forced line waits for its session and its step (ms). A line
/// that waits in the queue behind another line waits this long at most too.
pub const FORCED_LINE_MS: u64 = 40_000;
/// Most forced lines that wait behind the line the host says now.
pub const FORCED_QUEUE_MAX: usize = 8;
/// Longest a dialogue scene lasts when nothing ends it (ms).
pub const FORCED_SCENE_LIMIT_MS: u32 = coop_core::dialogue::LEASE_MS as u32;
/// Longest the host's conversation stops everyone's time after its last
/// pick (one dialogue heartbeat). Then the world runs on under the menu.
pub const CONVERSATION_HOLD_MS: u64 = coop_core::dialogue::HEARTBEAT_MS;

/// What a machine's frames and clock did, for the pause counters.
#[derive(Clone, Copy, Debug, Default)]
struct PauseTrack {
    last_ms: u64,
    days: Option<f32>,
    lag_ms: f64,
}

/// A line a host script made someone say, while the host says it to everyone.
#[derive(Clone, Copy, Debug)]
struct ForcedLine {
    npc: FormId,
    info: FormId,
    req: u32,
    picked: bool,
    since_ms: u64,
}

/// A dialogue scene for a script's conversation.
#[derive(Clone, Copy, Debug)]
struct ForcedScene {
    speaker: FormId,
    /// The host's menu opened its session.
    seen: bool,
    since_ms: u64,
}

/// The first line `speaker` says on `topic` (its greeting when `None`) now.
pub fn line_of(cx: Ctx, gs: &GameState, speaker: FormId, topic: Option<FormId>) -> Option<FormId> {
    let base = crate::scripting::base_of(cx.order, speaker)
        .or_else(|| crate::more_functions::placed::base(gs, speaker))?;
    let sp = crate::dialogue::Speaker::load(cx.order, speaker, base)?;
    let topic = topic.or_else(|| cx.order.form_by_editor_id("GREETING"))?;
    crate::dialogue::pick(cx.order, topic, &sp, gs).map(|i| i.form_id)
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
    /// Host: dialogue scenes opened for a script's conversation, by scene id.
    forced_scenes: BTreeMap<u32, ForcedScene>,
    /// Host: the line a script made someone say, in progress.
    forced_line: Option<ForcedLine>,
    /// Host: one-line talk notes that came while the host talked, with the
    /// time each came (ms). The host says them in turn when it is free.
    forced_queue: VecDeque<(super::scene::Talk, u64)>,
    /// Host: guest picks that wait for the line's conditions check.
    held_picks: Vec<(PlayerId, Msg)>,
    /// Host: its own conversation's (session, step) and since when (ms).
    hold: Option<((u32, u32), u64)>,
    pause: PauseTrack,
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

    /// Every machine, every frame: how many of this machine's picks the
    /// host refused (the menu shows the line as not taken), and what the
    /// frames and the world clock did while another player talked.
    pub fn note_refusals(
        &mut self,
        cx: Ctx,
        gs: &mut GameState,
        client: &DialogueClient,
        now_ms: u64,
    ) {
        let t = &mut gs.coop.live.scene.talk;
        t.refusals = client.refused;
        let days = gs.global(cx.order, "GameDaysPassed");
        let p = &mut self.pause;
        let dt = if p.last_ms == 0 {
            0
        } else {
            now_ms.saturating_sub(p.last_ms).min(1000) as u32
        };
        p.last_ms = now_ms.max(1);
        // the clock's step this frame in real milliseconds at the time scale
        let scale = f64::from(gs.global(cx.order, "TimeScale").unwrap_or(30.0)).max(1e-3);
        let step_ms = match (p.days, days) {
            (Some(a), Some(b)) => (f64::from(b) - f64::from(a)).max(0.0) * 86_400_000.0 / scale,
            _ => 0.0,
        };
        let moved = step_ms > 0.0;
        p.days = days;
        p.lag_ms = (p.lag_ms + f64::from(dt) - step_ms).max(0.0);
        let t = &mut gs.coop.live.scene.talk;
        t.clock_lag_ms = p.lag_ms.min(f64::from(u32::MAX)) as u32;
        if moved {
            t.clock_ms += dt;
        }
        if client.is_spectator() {
            t.watched_frames += 1;
            t.watched_ms += dt;
            if moved {
                t.watched_clock_ms += dt;
            }
        }
    }

    /// Host: takes a guest's pick off the network before the session table
    /// sees it. The next [`check_picks`](Self::check_picks) tests the line's
    /// conditions as the picker first. Every other message comes back.
    pub fn hold_pick(&mut self, from: PlayerId, msg: Msg) -> Option<Msg> {
        match msg {
            Msg::Dialogue(DialogueMsg::Pick { .. }) if !from.is_host() => {
                self.held_picks.push((from, msg));
                None
            }
            m => Some(m),
        }
    }

    /// Host, every frame: the held guest picks. A line whose conditions
    /// pass for the picker (their skills, items and quests, said by the
    /// session's NPC) goes to the session table. Any other pick is refused:
    /// no script runs, and only the picker is told. Returns sends.
    pub fn check_picks(
        &mut self,
        cx: Ctx,
        gs: &mut GameState,
        live: &mut HostLive,
        auth: &mut AuthorityTable,
        now_ms: u64,
    ) -> Vec<(PlayerId, Msg)> {
        let mut outs = Vec::new();
        for (from, m) in std::mem::take(&mut self.held_picks) {
            let Msg::Dialogue(DialogueMsg::Pick {
                session,
                step,
                choice,
            }) = m
            else {
                continue;
            };
            let npc = live.dialogue.npc_of(session).map(|n| FormId(n.0));
            if pick_allowed(cx, gs, from, npc, FormId(choice)) {
                gs.coop.live.scene.talk.checked += 1;
                outs.extend(live.on_msg(from, m, now_ms, auth));
            } else {
                gs.coop.live.scene.talk.refused += 1;
                for o in live.dialogue.refuse(from, session, step, choice) {
                    outs.extend(live.route_extern(o.aud, Msg::Dialogue(o.msg)));
                }
            }
        }
        outs
    }

    /// Host, every frame after the host player's messages were delivered:
    /// the people host scripts made talk ([`GameState::coop_talk`]).
    ///
    /// * A conversation with the player (`StartConversation`) opens a
    ///   dialogue scene for the players where the host stands. The host's
    ///   own menu then runs the session (`intents::tap_dialogue`), so
    ///   every player gets its lines. The scene ends when that session
    ///   ends.
    /// * A line without a menu (`SayTo`, `Say`) is said to everyone: the
    ///   host opens a session of its own on the speaker, picks the line,
    ///   reports its own play done (its game says the line itself) and
    ///   closes the session when the step ends.
    ///
    /// Returns sends.
    pub fn forced_frame(
        &mut self,
        cx: Ctx,
        gs: &mut GameState,
        live: &mut HostLive,
        auth: &mut AuthorityTable,
        own: &mut DialogueClient,
        now_ms: u64,
    ) -> Vec<(PlayerId, Msg)> {
        let mut outs = Vec::new();
        // lines that waited too long go first: they do not count against the cap
        while self
            .forced_queue
            .front()
            .is_some_and(|(_, came)| now_ms.saturating_sub(*came) >= FORCED_LINE_MS)
        {
            self.forced_queue.pop_front();
            gs.coop.live.scene.talk.dropped += 1;
        }
        for t in std::mem::take(&mut gs.coop.live.scene.talk_notes) {
            gs.coop.live.scene.talk.notes += 1;
            if t.conversation {
                if t.to != crate::dialogue::PLAYER_REF {
                    // two people talk to each other: nothing to share
                    gs.coop.live.scene.talk.dropped += 1;
                    continue;
                }
                if self.forced_scenes.values().any(|s| s.speaker == t.speaker) {
                    continue;
                }
                let spec = dialogue_spec(gs, live, t.speaker);
                match live.scene.open(spec, now_ms) {
                    Some((id, o)) => {
                        outs.extend(live.route_scene(o));
                        self.forced_scenes.insert(
                            id,
                            ForcedScene {
                                speaker: t.speaker,
                                seen: false,
                                since_ms: now_ms,
                            },
                        );
                        gs.coop.live.scene.talk.scenes += 1;
                    }
                    None => gs.coop.live.scene.talk.dropped += 1,
                }
                continue;
            }
            // one line: it waits in turn while the host talks
            if self.forced_queue.len() >= FORCED_QUEUE_MAX {
                gs.coop.live.scene.talk.dropped += 1;
                continue;
            }
            self.forced_queue.push_back((t, now_ms));
        }
        // the host says the next waiting line only while it talks to no one
        while self.forced_line.is_none() && own.view().is_none() {
            let Some((t, came_ms)) = self.forced_queue.pop_front() else {
                break;
            };
            // the line is picked now: a say-once line said before is skipped
            let line = line_of(cx, gs, t.speaker, t.topic);
            let (Some(info), false) = (line, now_ms.saturating_sub(came_ms) >= FORCED_LINE_MS)
            else {
                gs.coop.live.scene.talk.dropped += 1;
                continue;
            };
            let (req, m) = own.open(cf(t.speaker));
            outs.extend(live.on_msg(PlayerId::HOST, Msg::Dialogue(m), now_ms, auth));
            self.forced_line = Some(ForcedLine {
                npc: t.speaker,
                info,
                req,
                picked: false,
                since_ms: now_ms,
            });
        }
        outs.extend(self.forced_line_step(gs, live, auth, own, now_ms));
        // the host's own conversation stops everyone's time until a heartbeat
        // passes with no pick; then the world runs on under the menu
        let key = own
            .view()
            .filter(|v| v.initiator.is_host())
            .map(|v| (v.session, v.step));
        self.hold = match (key, self.hold) {
            (Some(k), Some((held, since))) if held == k => Some((k, since)),
            (Some(k), _) => Some((k, now_ms)),
            (None, _) => None,
        };
        gs.coop.live.scene.conversation_runs_clock = self
            .hold
            .is_some_and(|(_, since)| now_ms.saturating_sub(since) >= CONVERSATION_HOLD_MS);
        // a conversation's scene lasts as long as the host's session with the speaker
        let talking = own
            .view()
            .filter(|v| v.initiator.is_host())
            .map(|v| FormId(v.npc.0));
        let ids: Vec<u32> = self.forced_scenes.keys().copied().collect();
        for id in ids {
            let Some(s) = self.forced_scenes.get_mut(&id) else {
                continue;
            };
            if live.scene.scene(id, now_ms).is_none() {
                self.forced_scenes.remove(&id);
                continue;
            }
            if talking == Some(s.speaker) {
                s.seen = true;
                continue;
            }
            let reason = if s.seen {
                EndReason::Finished
            } else if now_ms.saturating_sub(s.since_ms) >= FORCED_OPEN_MS {
                EndReason::TimedOut
            } else {
                continue;
            };
            self.forced_scenes.remove(&id);
            let o = live.scene.end(id, reason, now_ms);
            outs.extend(live.route_scene(o));
        }
        outs
    }

    /// The forced line's session, one step at a time.
    fn forced_line_step(
        &mut self,
        gs: &mut GameState,
        live: &mut HostLive,
        auth: &mut AuthorityTable,
        own: &mut DialogueClient,
        now_ms: u64,
    ) -> Vec<(PlayerId, Msg)> {
        let mut outs = Vec::new();
        let Some(mut l) = self.forced_line.take() else {
            return outs;
        };
        let mut send = |m: Option<DialogueMsg>, live: &mut HostLive, auth: &mut AuthorityTable| {
            if let Some(m) = m {
                outs.extend(live.on_msg(PlayerId::HOST, Msg::Dialogue(m), now_ms, auth));
            }
        };
        let view = own
            .view()
            .copied()
            .filter(|v| v.initiator.is_host() && v.npc == cf(l.npc));
        let late = now_ms.saturating_sub(l.since_ms) >= FORCED_LINE_MS;
        let keep = match view {
            Some(v) if !l.picked => {
                if v.playing.is_none() {
                    send(own.pick(l.info.0), live, auth);
                    l.picked = true;
                    gs.coop.live.scene.talk.lines += 1;
                }
                true
            }
            // the pick is on its way
            Some(v) if v.step == 0 => !late,
            // the host's game says the line itself: its play is done at once
            Some(v) if v.playing.is_some() => {
                send(own.line_done(), live, auth);
                true
            }
            Some(_) => {
                send(own.close(), live, auth);
                false
            }
            None => {
                let denied = own.denied.is_some_and(|(r, _)| r == l.req);
                if denied {
                    gs.coop.live.scene.talk.dropped += 1;
                }
                !(denied || l.picked || late)
            }
        };
        if keep {
            self.forced_line = Some(l);
        }
        outs
    }

    /// Host, every frame: feeds combat and deaths to the session table, then
    /// runs the scripts the sessions asked for. Returns sends.
    pub fn host_frame(
        &mut self,
        cx: Ctx,
        gs: &mut GameState,
        live: &mut HostLive,
    ) -> Vec<(PlayerId, Msg)> {
        let mut outs = Vec::new();
        let fighting: BTreeSet<FormId> = gs.combat.iter().flat_map(|(a, b)| [*a, *b]).collect();
        for n in fighting.difference(&self.in_combat.clone()) {
            live.dialogue.set_in_combat(cf(*n), true);
        }
        for n in self.in_combat.difference(&fighting.clone()) {
            live.dialogue.set_in_combat(cf(*n), false);
        }
        self.in_combat = fighting;
        let dead: Vec<FormId> = gs
            .dead
            .iter()
            .copied()
            .filter(|d| !self.died.contains(d))
            .collect();
        for d in dead {
            self.died.insert(d);
            for o in live.dialogue.npc_died(cf(d)) {
                outs.extend(live.route_extern(o.aud, Msg::Dialogue(o.msg)));
            }
        }
        for e in live.dialogue.take_events() {
            match e {
                DialogueEvent::RunBegin {
                    session,
                    npc,
                    choice,
                    initiator,
                    ..
                } => {
                    let npc = FormId(npc.0);
                    if self.native_host && initiator.is_host() {
                        self.native_sessions.insert(session);
                        continue;
                    }
                    self.initiators.insert(session, initiator);
                    if let Some(info) = Info::load(cx.order, FormId(choice)) {
                        // the host runs the line as the player who picked it
                        super::campaign::producers::as_initiator(
                            gs,
                            initiator.0,
                            info.quest,
                            |gs| {
                                line_begins(gs, &info, npc);
                                if menu_runs_begin_script(&info) {
                                    run(cx, gs, info.begin_script.as_deref(), npc);
                                }
                            },
                        );
                    }
                    self.begin_runs += 1;
                }
                DialogueEvent::RunEnd {
                    session,
                    npc,
                    choice,
                    ..
                } => {
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

/// The dialogue scene of a script's conversation: the dialogue camera for the
/// players where the host stands (roamers who arrive merge in). Nobody is
/// locked out and nobody votes to skip: the host's menu ends it.
fn dialogue_spec(gs: &GameState, live: &HostLive, speaker: FormId) -> SceneSpec {
    let mut spec = SceneSpec::new(SceneKind::Dialogue, speaker.0);
    spec.locks = Locks::NONE;
    spec.camera = CameraMode::Dialogue;
    spec.barrier = BarrierRule::Roam;
    spec.skip = SkipRule::Never;
    spec.timeout_ms = FORCED_SCENE_LIMIT_MS;
    let cell = gs.player_cell.map_or(0, |c| c.0);
    spec.cell = cell;
    let here: Vec<PlayerId> = live
        .members()
        .iter()
        .copied()
        .filter(|p| p.is_host() || live.avatars.state(*p).is_some_and(|s| s.cell.0 == cell))
        .collect();
    spec.participants = Mask::from_players(&here);
    spec
}

/// Whether player `from` may pick `choice` with `npc`: the line exists and
/// its conditions pass with the picker's own rows (skills, items, quests).
fn pick_allowed(
    cx: Ctx,
    gs: &mut GameState,
    from: PlayerId,
    npc: Option<FormId>,
    choice: FormId,
) -> bool {
    let Some(info) = Info::load(cx.order, choice) else {
        return false;
    };
    let Some(npc) = npc else {
        // no session: the table ignores the pick anyway
        return true;
    };
    let base = crate::scripting::base_of(cx.order, npc)
        .or_else(|| crate::more_functions::placed::base(gs, npc));
    let Some(sp) = base.and_then(|b| crate::dialogue::Speaker::load(cx.order, npc, b)) else {
        return true;
    };
    gs.with_player(from.0, |g| crate::dialogue::passes(cx.order, &info, &sp, g))
}

/// Runs one result script on the speaker with the real runner.
fn run(cx: Ctx, gs: &mut GameState, source: Option<&str>, speaker: FormId) {
    if let Some(src) = source.filter(|s| !s.trim().is_empty()) {
        Runner::new(cx.order, cx.scripts, gs).run_source(src, Some(speaker), Some(speaker));
    }
}

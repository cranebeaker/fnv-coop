//! Host side of scenes and time.

use std::collections::BTreeMap;

use super::*;
use crate::interest::Audience;

/// One addressed output message.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SOut {
    /// Recipients.
    pub aud: Audience,
    /// Message.
    pub msg: SceneMsg,
}

fn out(aud: Audience, msg: SceneMsg) -> SOut {
    SOut { aud, msg }
}

/// What the host's game may want to react to.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SceneEvent {
    /// A scene opened.
    Opened(u32),
    /// The barrier released: the scene is running.
    Started(u32),
    /// A player merged into a running scene.
    Merged(u32, PlayerId),
    /// A scene ended.
    Closed(u32, EndReason),
}

/// Knobs from the campaign policy.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ScenePolicy {
    /// A player who joins during a scene is held until it ends (the
    /// alternative is that the session refuses the join, see
    /// [`SceneHost::joins_allowed`]).
    pub late_join_waits: bool,
    /// Barrier timeout.
    pub gather_timeout_ms: u64,
    /// Lead between the release and the start.
    pub lead_ms: u32,
}

impl Default for ScenePolicy {
    fn default() -> Self {
        ScenePolicy {
            late_join_waits: true,
            gather_timeout_ms: GATHER_TIMEOUT_MS,
            lead_ms: GO_LEAD_MS,
        }
    }
}

struct HScene {
    info: SceneInfo,
    ready: Mask,
    done: Mask,
    yes: Mask,
    opened_ms: u64,
    /// Host time scene time zero (set when released).
    zero_ms: Option<u64>,
}

/// Authority for scenes, endings and world time.
pub struct SceneHost {
    scenes: BTreeMap<u32, HScene>,
    next_id: u32,
    members: Mask,
    committed: Vec<u32>,
    policy: ScenePolicy,
    events: Vec<SceneEvent>,
    next_time_event: u32,
    /// Time skips announced.
    pub skips: u32,
    last_clock: Option<(ClockSnap, u64)>,
    next_present: u32,
    /// Image space modifiers a host script left on (for late joiners).
    imods: Vec<u32>,
}

impl Default for SceneHost {
    fn default() -> Self {
        Self::new()
    }
}

impl SceneHost {
    /// A host with only the host player present.
    pub fn new() -> Self {
        SceneHost {
            scenes: BTreeMap::new(),
            next_id: 1,
            members: Mask::of(PlayerId::HOST),
            committed: Vec::new(),
            policy: ScenePolicy::default(),
            events: Vec::new(),
            next_time_event: 1,
            skips: 0,
            last_clock: None,
            next_present: 1,
            imods: Vec::new(),
        }
    }

    /// A host script's presentation for every other player (G39). A
    /// modifier that stays on is replayed to a late joiner.
    pub fn present(&mut self, cue: Cue) -> Vec<SOut> {
        if let Cue::ImageSpace { form, on } = cue {
            self.imods.retain(|f| *f != form);
            if on && self.imods.len() < MAX_IMODS {
                self.imods.push(form);
            }
        }
        let event = self.next_present;
        self.next_present += 1;
        vec![out(
            Audience::AllExcept(PlayerId::HOST),
            SceneMsg::Present { event, cue },
        )]
    }

    /// The image space modifiers a host script left on.
    pub fn imods(&self) -> &[u32] {
        &self.imods
    }

    fn replay_imods(&self, p: PlayerId, outs: &mut Vec<SOut>) {
        for &form in &self.imods {
            outs.push(out(
                Audience::To(p),
                SceneMsg::Present {
                    event: 0,
                    cue: Cue::ImageSpace { form, on: true },
                },
            ));
        }
    }

    /// Sets the campaign knobs.
    pub fn set_policy(&mut self, p: ScenePolicy) {
        self.policy = p;
    }

    /// The knobs.
    pub fn policy(&self) -> ScenePolicy {
        self.policy
    }

    /// Events for the host's game.
    pub fn take_events(&mut self) -> Vec<SceneEvent> {
        std::mem::take(&mut self.events)
    }

    /// Ids of the open scenes, ascending.
    pub fn ids(&self) -> Vec<u32> {
        self.scenes.keys().copied().collect()
    }

    /// A scene as it stands (elapsed time as of `now_ms`).
    pub fn scene(&self, id: u32, now_ms: u64) -> Option<SceneInfo> {
        self.scenes.get(&id).map(|s| self.snapshot(s, now_ms))
    }

    /// The open scene of this kind and tag, if any.
    pub fn find(&self, kind: SceneKind, tag: u32) -> Option<u32> {
        self.scenes
            .values()
            .find(|s| s.info.spec.kind == kind && s.info.spec.tag == tag)
            .map(|s| s.info.id)
    }

    /// True while any scene is open.
    pub fn active(&self) -> bool {
        !self.scenes.is_empty()
    }

    /// Ending tags the party committed to (saved in the party save).
    pub fn committed(&self) -> &[u32] {
        &self.committed
    }

    /// Restores the commitments from a save.
    pub fn restore_committed(&mut self, tags: &[u32]) {
        for t in tags {
            if !self.committed.contains(t) {
                self.committed.push(*t);
            }
        }
    }

    /// Whether the session should take a joiner now: false only when the
    /// policy refuses late joins and a scene is running.
    pub fn joins_allowed(&self) -> bool {
        self.policy.late_join_waits || self.scenes.is_empty()
    }

    /// The locks the player lives under now (participants of any scene, and
    /// joiners held until it ends).
    pub fn locks_for(&self, p: PlayerId) -> Locks {
        let mut l = Locks::NONE;
        for s in self.scenes.values() {
            if s.info.waiting.contains(p) {
                l = l.union(Locks::ALL);
            } else if s.info.participants.contains(p) {
                l = l.union(s.info.locks);
            }
        }
        l
    }

    fn snapshot(&self, s: &HScene, now_ms: u64) -> SceneInfo {
        let mut i = s.info.clone();
        i.elapsed_ms = match (i.phase, s.zero_ms) {
            (Phase::Playing, Some(z)) => now_ms.saturating_sub(z).min(u64::from(u32::MAX)) as u32,
            _ => 0,
        };
        i
    }

    /// Opens a scene. Asking again for the same kind and tag returns the open
    /// one (a script that runs twice does not stack scenes). `None` when there
    /// are too many scenes or nobody would be in it.
    pub fn open(&mut self, mut spec: SceneSpec, now_ms: u64) -> Option<(u32, Vec<SOut>)> {
        if let Some(id) = self.find(spec.kind, spec.tag) {
            return Some((id, Vec::new()));
        }
        if self.scenes.len() >= MAX_SCENES || !spec.is_sane() {
            return None;
        }
        let everyone = self.members;
        let participants = if spec.participants.is_empty() {
            everyone
        } else {
            Mask(spec.participants.0 & everyone.0)
        };
        if participants.is_empty() {
            return None;
        }
        spec.participants = participants;
        let id = self.next_id;
        self.next_id += 1;
        let together =
            spec.barrier == BarrierRule::Together && participants.minus_host().count() > 0;
        let info = SceneInfo {
            id,
            participants,
            waiting: Mask::NONE,
            locks: spec.locks,
            phase: if together {
                Phase::Gather
            } else {
                Phase::Playing
            },
            elapsed_ms: 0,
            slide: 0,
            spec: spec.clone(),
        };
        if spec.kind == SceneKind::Ending && !self.committed.contains(&spec.tag) {
            self.committed.push(spec.tag);
        }
        let mut sc = HScene {
            info,
            ready: Mask::of(PlayerId::HOST),
            done: Mask::NONE,
            yes: Mask::NONE,
            opened_ms: now_ms,
            zero_ms: None,
        };
        let mut outs = vec![out(
            Audience::All,
            SceneMsg::Open {
                scene: sc.info.clone(),
            },
        )];
        self.events.push(SceneEvent::Opened(id));
        if !together {
            outs.extend(self.release(&mut sc, now_ms));
        }
        self.scenes.insert(id, sc);
        Some((id, outs))
    }

    fn release(&mut self, sc: &mut HScene, now_ms: u64) -> Vec<SOut> {
        sc.info.phase = Phase::Playing;
        sc.zero_ms = Some(now_ms + u64::from(self.policy.lead_ms));
        self.events.push(SceneEvent::Started(sc.info.id));
        vec![out(
            Audience::All,
            SceneMsg::Go {
                id: sc.info.id,
                start_in_ms: self.policy.lead_ms,
            },
        )]
    }

    /// A participant is ready for the barrier.
    pub fn ready(&mut self, from: PlayerId, id: u32, now_ms: u64) -> Vec<SOut> {
        let Some(mut sc) = self.scenes.remove(&id) else {
            return vec![];
        };
        let mut outs = Vec::new();
        if sc.info.participants.contains(from) {
            sc.ready = sc.ready.with(from);
            if sc.info.phase == Phase::Gather && sc.info.participants.is_subset_of(sc.ready) {
                outs = self.release(&mut sc, now_ms);
            }
        }
        self.scenes.insert(id, sc);
        outs
    }

    /// A player finished playing the scene on their machine. The host's own
    /// end closes the scene for everyone; so does everyone being done.
    pub fn done(&mut self, from: PlayerId, id: u32, now_ms: u64) -> Vec<SOut> {
        let Some(sc) = self.scenes.get_mut(&id) else {
            return vec![];
        };
        if !sc.info.participants.contains(from) || sc.info.phase != Phase::Playing {
            return vec![];
        }
        sc.done = sc.done.with(from);
        if from.is_host() || sc.info.participants.is_subset_of(sc.done) {
            return self.end(id, EndReason::Finished, now_ms);
        }
        vec![]
    }

    /// A skip vote (or the host's skip).
    pub fn skip_vote(&mut self, from: PlayerId, id: u32, yes: bool, now_ms: u64) -> Vec<SOut> {
        let Some(sc) = self.scenes.get_mut(&id) else {
            return vec![];
        };
        if !sc.info.participants.contains(from) {
            return vec![];
        }
        match sc.info.spec.skip {
            SkipRule::Never => vec![],
            SkipRule::HostOnly => {
                if from.is_host() && yes {
                    self.end(id, EndReason::Skipped, now_ms)
                } else {
                    vec![]
                }
            }
            SkipRule::Unanimous => {
                sc.yes = if yes {
                    sc.yes.with(from)
                } else {
                    sc.yes.without(from)
                };
                if sc.info.participants.is_subset_of(sc.yes) {
                    self.end(id, EndReason::Skipped, now_ms)
                } else {
                    vec![]
                }
            }
        }
    }

    /// Adds a player to a running scene (they arrived in its cell, or asked
    /// to join). They are sent the scene with its elapsed time.
    pub fn merge(&mut self, id: u32, p: PlayerId, now_ms: u64) -> Vec<SOut> {
        let Some(sc) = self.scenes.get(&id) else {
            return vec![];
        };
        if !self.members.contains(p)
            || sc.info.participants.contains(p)
            || sc.info.phase != Phase::Playing
        {
            return vec![];
        }
        let sc = self.scenes.get_mut(&id).expect("checked");
        sc.info.participants = sc.info.participants.with(p);
        sc.info.waiting = sc.info.waiting.without(p);
        sc.ready = sc.ready.with(p);
        let sc = &self.scenes[&id];
        self.events.push(SceneEvent::Merged(id, p));
        let info = self.snapshot(sc, now_ms);
        vec![
            out(
                Audience::To(p),
                SceneMsg::Sync {
                    scenes: vec![info],
                    committed: self.committed.clone(),
                },
            ),
            out(Audience::All, SceneMsg::Merged { id, player: p }),
        ]
    }

    /// A player asks to merge in. Only roaming scenes take them.
    pub fn join_request(&mut self, from: PlayerId, id: u32, now_ms: u64) -> Vec<SOut> {
        match self.scenes.get(&id) {
            Some(sc) if sc.info.spec.barrier == BarrierRule::Roam => self.merge(id, from, now_ms),
            _ => vec![],
        }
    }

    /// Changes a scene's lock set (the story turned some controls back on).
    pub fn set_locks(&mut self, id: u32, locks: Locks) -> Vec<SOut> {
        let Some(sc) = self.scenes.get_mut(&id) else {
            return vec![];
        };
        if sc.info.locks == locks {
            return vec![];
        }
        sc.info.locks = locks;
        vec![out(Audience::All, SceneMsg::Lock { id, locks })]
    }

    /// Shows the next ending slide to everyone, in order. Stale or repeated
    /// indices are dropped.
    pub fn slide(&mut self, id: u32, index: u16) -> Vec<SOut> {
        let Some(sc) = self.scenes.get_mut(&id) else {
            return vec![];
        };
        // slide 0 is the one an ending opens on
        if sc.info.spec.kind != SceneKind::Ending
            || index <= sc.info.slide
            || index >= sc.info.spec.slides
        {
            return vec![];
        }
        sc.info.slide = index;
        vec![out(Audience::All, SceneMsg::Slide { id, index })]
    }

    /// Ends a scene.
    pub fn end(&mut self, id: u32, reason: EndReason, _now_ms: u64) -> Vec<SOut> {
        if self.scenes.remove(&id).is_none() {
            return vec![];
        }
        self.events.push(SceneEvent::Closed(id, reason));
        vec![out(Audience::All, SceneMsg::Close { id, reason })]
    }

    /// Ends every scene (the story reloaded, a save loaded).
    pub fn abort_all(&mut self, now_ms: u64) -> Vec<SOut> {
        let mut outs = Vec::new();
        for id in self.ids() {
            outs.extend(self.end(id, EndReason::Aborted, now_ms));
        }
        outs
    }

    /// A player joined the session: they are told the scenes in progress and
    /// held until those end (or merged in by the story).
    pub fn on_join(&mut self, p: PlayerId, now_ms: u64, clock: Option<ClockSnap>) -> Vec<SOut> {
        if self.members.contains(p) {
            return vec![];
        }
        self.members = self.members.with(p);
        if self.policy.late_join_waits {
            for sc in self.scenes.values_mut() {
                if !sc.info.participants.contains(p) {
                    sc.info.waiting = sc.info.waiting.with(p);
                }
            }
        }
        let scenes: Vec<SceneInfo> = self
            .scenes
            .values()
            .map(|s| self.snapshot(s, now_ms))
            .collect();
        let mut outs = vec![out(
            Audience::To(p),
            SceneMsg::Sync {
                scenes,
                committed: self.committed.clone(),
            },
        )];
        if let Some(c) = clock {
            outs.push(out(Audience::To(p), SceneMsg::Clock { clock: c }));
        }
        self.replay_imods(p, &mut outs);
        outs
    }

    /// A known player rejoined (their link dropped and came back): the scenes
    /// in progress are replayed to them as they stand now.
    pub fn resync(&mut self, p: PlayerId, now_ms: u64, clock: Option<ClockSnap>) -> Vec<SOut> {
        if !self.members.contains(p) {
            return vec![];
        }
        let scenes: Vec<SceneInfo> = self
            .scenes
            .values()
            .map(|s| self.snapshot(s, now_ms))
            .collect();
        let mut outs = vec![out(
            Audience::To(p),
            SceneMsg::Sync {
                scenes,
                committed: self.committed.clone(),
            },
        )];
        if let Some(c) = clock {
            outs.push(out(Audience::To(p), SceneMsg::Clock { clock: c }));
        }
        self.replay_imods(p, &mut outs);
        outs
    }

    /// A player left: scenes stop waiting for them.
    pub fn on_leave(&mut self, p: PlayerId, now_ms: u64) -> Vec<SOut> {
        self.members = self.members.without(p);
        let mut outs = Vec::new();
        for id in self.ids() {
            let sc = self.scenes.get_mut(&id).expect("listed");
            sc.info.participants = sc.info.participants.without(p);
            sc.info.waiting = sc.info.waiting.without(p);
            sc.ready = sc.ready.without(p);
            sc.done = sc.done.without(p);
            sc.yes = sc.yes.without(p);
            if sc.info.participants.is_empty() {
                outs.extend(self.end(id, EndReason::Empty, now_ms));
            } else if sc.info.phase == Phase::Gather && sc.info.participants.is_subset_of(sc.ready)
            {
                let mut sc = self.scenes.remove(&id).expect("listed");
                outs.extend(self.release(&mut sc, now_ms));
                self.scenes.insert(id, sc);
            } else if sc.info.phase == Phase::Playing && sc.info.participants.is_subset_of(sc.done)
            {
                outs.extend(self.end(id, EndReason::Finished, now_ms));
            }
        }
        outs
    }

    /// Timers: barrier timeouts and scene time limits.
    pub fn tick(&mut self, now_ms: u64) -> Vec<SOut> {
        let mut outs = Vec::new();
        for id in self.ids() {
            let Some(sc) = self.scenes.get(&id) else {
                continue;
            };
            match sc.info.phase {
                Phase::Gather
                    if now_ms.saturating_sub(sc.opened_ms) >= self.policy.gather_timeout_ms =>
                {
                    let mut sc = self.scenes.remove(&id).expect("listed");
                    outs.extend(self.release(&mut sc, now_ms));
                    self.scenes.insert(id, sc);
                }
                Phase::Playing => {
                    let limit = u64::from(sc.info.spec.timeout_ms);
                    if limit > 0
                        && sc
                            .zero_ms
                            .is_some_and(|z| now_ms.saturating_sub(z) >= limit)
                    {
                        outs.extend(self.end(id, EndReason::TimedOut, now_ms));
                    }
                }
                _ => {}
            }
        }
        outs
    }

    /// A time skip happened on the host (a vote passed and the hours went by):
    /// announced once, with the clock after it.
    pub fn time_skip(
        &mut self,
        hours: u16,
        sleeping: bool,
        clock: ClockSnap,
        now_ms: u64,
    ) -> Vec<SOut> {
        let event = self.next_time_event;
        self.next_time_event += 1;
        self.skips += 1;
        self.last_clock = Some((clock, now_ms));
        vec![out(
            Audience::All,
            SceneMsg::TimeSkip {
                event,
                hours,
                sleeping,
                clock,
            },
        )]
    }

    /// The clock, repeated about once a second when it changed.
    pub fn clock(&mut self, clock: ClockSnap, now_ms: u64) -> Vec<SOut> {
        if let Some((_, at)) = self.last_clock {
            if now_ms.saturating_sub(at) < CLOCK_EVERY_MS {
                return vec![];
            }
        }
        self.last_clock = Some((clock, now_ms));
        vec![out(
            Audience::AllExcept(PlayerId::HOST),
            SceneMsg::Clock { clock },
        )]
    }

    /// The newest clock the host announced.
    pub fn last_clock(&self) -> Option<ClockSnap> {
        self.last_clock.map(|c| c.0)
    }

    /// Handles a player message.
    pub fn on_msg(&mut self, from: PlayerId, msg: SceneMsg, now_ms: u64) -> Vec<SOut> {
        if !self.members.contains(from) {
            return vec![];
        }
        match msg {
            SceneMsg::Ready { id } => self.ready(from, id, now_ms),
            SceneMsg::Done { id } => self.done(from, id, now_ms),
            SceneMsg::SkipVote { id, yes } => self.skip_vote(from, id, yes, now_ms),
            SceneMsg::Join { id } => self.join_request(from, id, now_ms),
            _ => vec![],
        }
    }
}

impl Mask {
    fn minus_host(self) -> Mask {
        self.without(PlayerId::HOST)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(n: u8) -> PlayerId {
        PlayerId(n)
    }

    fn host3() -> SceneHost {
        let mut h = SceneHost::new();
        h.on_join(p(1), 0, None);
        h.on_join(p(2), 0, None);
        h
    }

    fn movie_spec() -> SceneSpec {
        let mut s = SceneSpec::new(SceneKind::Movie, 77);
        s.movie = Some(Movie {
            file: "intro.bik".into(),
            interruptable: true,
            mute_audio: true,
            pause_music: true,
            letterbox: true,
        });
        s.camera = CameraMode::Movie;
        s.locks = Locks::ALL;
        s.skip = SkipRule::Unanimous;
        s
    }

    #[test]
    fn together_waits_for_every_participant_then_goes_once() {
        let mut h = host3();
        let (id, o) = h.open(movie_spec(), 100).unwrap();
        assert!(
            matches!(&o[0].msg, SceneMsg::Open { scene } if scene.phase == Phase::Gather && scene.participants == Mask(7))
        );
        assert_eq!(o.len(), 1, "no go yet");
        assert!(h.ready(p(1), id, 150).is_empty());
        let go = h.ready(p(2), id, 200);
        assert!(matches!(
            go[0].msg,
            SceneMsg::Go {
                start_in_ms: GO_LEAD_MS,
                ..
            }
        ));
        assert!(
            h.ready(p(2), id, 210).is_empty(),
            "a second ready does not release twice"
        );
        assert_eq!(
            h.take_events(),
            vec![SceneEvent::Opened(id), SceneEvent::Started(id)]
        );
    }

    #[test]
    fn the_gather_times_out_and_goes_without_the_slow_player() {
        let mut h = host3();
        let (id, _) = h.open(movie_spec(), 0).unwrap();
        h.ready(p(1), id, 10);
        assert!(h.tick(GATHER_TIMEOUT_MS - 1).is_empty());
        let o = h.tick(GATHER_TIMEOUT_MS);
        assert!(matches!(o[0].msg, SceneMsg::Go { .. }));
    }

    #[test]
    fn roam_starts_at_once_and_a_roamer_merges_in_with_the_elapsed_time() {
        let mut h = host3();
        let mut s = SceneSpec::new(SceneKind::Cinematic, 5);
        s.barrier = BarrierRule::Roam;
        s.participants = Mask(0b011);
        s.locks = Locks(Locks::MOVEMENT | Locks::LOOKING);
        let (id, o) = h.open(s, 1_000).unwrap();
        assert!(o.iter().any(|x| matches!(x.msg, SceneMsg::Go { .. })));
        assert_eq!(h.locks_for(p(2)), Locks::NONE, "the roamer is free");
        assert_eq!(h.locks_for(p(1)), Locks(Locks::MOVEMENT | Locks::LOOKING));
        let o = h.merge(id, p(2), 6_300);
        let SceneMsg::Sync { scenes, .. } = &o[0].msg else {
            panic!()
        };
        assert_eq!(
            scenes[0].elapsed_ms, 5_000,
            "the roamer starts where the scene is"
        );
        assert_eq!(o[0].aud, Audience::To(p(2)));
        assert!(matches!(o[1].msg, SceneMsg::Merged { .. }));
        assert_eq!(h.locks_for(p(2)), Locks(Locks::MOVEMENT | Locks::LOOKING));
        assert!(
            h.merge(id, p(2), 6_400).is_empty(),
            "merging twice does nothing"
        );
    }

    #[test]
    fn a_scene_is_not_stacked_by_a_script_that_runs_twice() {
        let mut h = host3();
        let (a, _) = h.open(movie_spec(), 0).unwrap();
        let (b, o) = h.open(movie_spec(), 5).unwrap();
        assert_eq!(a, b);
        assert!(o.is_empty());
    }

    #[test]
    fn unanimous_skip_needs_every_participant_and_a_no_withdraws() {
        let mut h = host3();
        let (id, _) = h.open(movie_spec(), 0).unwrap();
        h.ready(p(1), id, 1);
        h.ready(p(2), id, 1);
        assert!(h.skip_vote(p(0), id, true, 5).is_empty());
        assert!(h.skip_vote(p(1), id, true, 6).is_empty());
        assert!(h.skip_vote(p(1), id, false, 7).is_empty());
        assert!(h.skip_vote(p(2), id, true, 8).is_empty());
        let o = h.skip_vote(p(1), id, true, 9);
        assert!(matches!(
            o[0].msg,
            SceneMsg::Close {
                reason: EndReason::Skipped,
                ..
            }
        ));
        assert!(!h.active());
    }

    #[test]
    fn host_only_skip_ignores_guests() {
        let mut h = host3();
        let mut s = movie_spec();
        s.skip = SkipRule::HostOnly;
        s.barrier = BarrierRule::Roam;
        let (id, _) = h.open(s, 0).unwrap();
        assert!(h.skip_vote(p(1), id, true, 1).is_empty());
        assert!(matches!(
            h.skip_vote(p(0), id, true, 2)[0].msg,
            SceneMsg::Close {
                reason: EndReason::Skipped,
                ..
            }
        ));
    }

    #[test]
    fn never_skip_cannot_be_skipped() {
        let mut h = host3();
        let mut s = movie_spec();
        s.skip = SkipRule::Never;
        s.barrier = BarrierRule::Roam;
        let (id, _) = h.open(s, 0).unwrap();
        for who in 0..3 {
            assert!(h.skip_vote(p(who), id, true, 1).is_empty());
        }
        assert!(h.active());
    }

    #[test]
    fn the_hosts_done_ends_the_scene_and_a_guests_done_does_not() {
        let mut h = host3();
        let mut s = movie_spec();
        s.barrier = BarrierRule::Roam;
        let (id, _) = h.open(s, 0).unwrap();
        assert!(h.done(p(1), id, 10).is_empty());
        assert!(matches!(
            h.done(p(0), id, 11)[0].msg,
            SceneMsg::Close {
                reason: EndReason::Finished,
                ..
            }
        ));
    }

    #[test]
    fn a_disconnect_in_the_gather_releases_it_and_the_last_one_leaving_ends_it() {
        let mut h = host3();
        let (id, _) = h.open(movie_spec(), 0).unwrap();
        h.ready(p(1), id, 1);
        let o = h.on_leave(p(2), 2);
        assert!(
            matches!(o[0].msg, SceneMsg::Go { .. }),
            "nobody waits for a player who left"
        );
        let o = h.on_leave(p(1), 3);
        assert!(o.is_empty(), "the host is still in it");
        assert_eq!(h.ids(), vec![id]);
        // only the host is left: the host's done closes it
        assert!(matches!(h.done(p(0), id, 4)[0].msg, SceneMsg::Close { .. }));
    }

    #[test]
    fn everyone_leaving_a_non_host_scene_ends_it_empty() {
        let mut h = host3();
        let mut s = movie_spec();
        s.participants = Mask(0b110);
        let (id, _) = h.open(s, 0).unwrap();
        h.on_leave(p(1), 1);
        let o = h.on_leave(p(2), 2);
        assert!(matches!(o[0].msg, SceneMsg::Close { id: i, reason: EndReason::Empty } if i == id));
    }

    #[test]
    fn a_late_joiner_is_replayed_the_scene_and_held_until_it_ends() {
        let mut h = SceneHost::new();
        let mut s = movie_spec();
        s.barrier = BarrierRule::Roam;
        let (id, _) = h.open(s, 0).unwrap();
        let o = h.on_join(p(1), 4_300, None);
        let SceneMsg::Sync { scenes, .. } = &o[0].msg else {
            panic!()
        };
        assert_eq!(scenes[0].elapsed_ms, 4_000);
        assert_eq!(h.locks_for(p(1)), Locks::ALL, "held");
        h.end(id, EndReason::Finished, 9);
        assert_eq!(h.locks_for(p(1)), Locks::NONE);
    }

    #[test]
    fn refusing_policy_blocks_joins_during_a_scene() {
        let mut h = SceneHost::new();
        h.set_policy(ScenePolicy {
            late_join_waits: false,
            ..Default::default()
        });
        assert!(h.joins_allowed());
        let mut s = movie_spec();
        s.barrier = BarrierRule::Roam;
        h.open(s, 0).unwrap();
        assert!(!h.joins_allowed());
    }

    #[test]
    fn scene_time_limit_and_abort() {
        let mut h = host3();
        let mut s = SceneSpec::new(SceneKind::Cinematic, 1);
        s.barrier = BarrierRule::Roam;
        s.timeout_ms = 1_000;
        let (id, _) = h.open(s, 0).unwrap();
        assert!(h.tick(GO_LEAD_MS as u64 + 999).is_empty());
        assert!(matches!(
            h.tick(GO_LEAD_MS as u64 + 1_000)[0].msg,
            SceneMsg::Close {
                reason: EndReason::TimedOut,
                ..
            }
        ));
        let _ = id;
        let mut s = SceneSpec::new(SceneKind::Cinematic, 2);
        s.barrier = BarrierRule::Roam;
        h.open(s, 0).unwrap();
        let o = h.abort_all(5);
        assert!(matches!(
            o[0].msg,
            SceneMsg::Close {
                reason: EndReason::Aborted,
                ..
            }
        ));
    }

    #[test]
    fn ending_slides_are_ordered_and_the_commitment_is_kept() {
        let mut h = host3();
        let mut s = SceneSpec::new(SceneKind::Ending, 0xE1);
        s.slides = 3;
        s.barrier = BarrierRule::Roam;
        let (id, _) = h.open(s, 0).unwrap();
        assert_eq!(h.committed(), &[0xE1]);
        assert_eq!(h.slide(id, 1).len(), 1);
        assert!(h.slide(id, 1).is_empty(), "repeat");
        assert!(h.slide(id, 0).is_empty(), "stale");
        assert!(h.slide(id, 3).is_empty(), "past the end");
        assert_eq!(h.slide(id, 2).len(), 1);
        h.end(id, EndReason::Finished, 1);
        let o = h.on_join(p(3), 2, None);
        assert!(
            matches!(&o[0].msg, SceneMsg::Sync { committed, scenes } if committed == &[0xE1] && scenes.is_empty())
        );
    }

    #[test]
    fn time_skips_have_increasing_event_ids_and_the_clock_is_rate_limited() {
        let mut h = host3();
        let c = ClockSnap {
            hour: 8.0f32.to_bits(),
            ..Default::default()
        };
        let a = h.time_skip(8, true, c, 0);
        let b = h.time_skip(1, false, c, 10);
        assert!(matches!(a[0].msg, SceneMsg::TimeSkip { event: 1, .. }));
        assert!(matches!(b[0].msg, SceneMsg::TimeSkip { event: 2, .. }));
        assert_eq!(h.skips, 2);
        assert!(h.clock(c, 500).is_empty());
        assert_eq!(h.clock(c, 1_010).len(), 1);
    }

    #[test]
    fn strangers_are_ignored() {
        let mut h = host3();
        let (id, _) = h.open(movie_spec(), 0).unwrap();
        assert!(h.on_msg(p(7), SceneMsg::Ready { id }, 1).is_empty());
    }
}

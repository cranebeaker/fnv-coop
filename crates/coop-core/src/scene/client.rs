//! One machine's view of scenes and the clock.

use std::collections::BTreeMap;

use super::*;

/// What happened that the game should react to (drained once).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Notice {
    /// A scene opened here.
    Opened(u32),
    /// The scene's time zero arrived (or a late joiner entered it
    /// `offset_ms` into it): start its camera and movie now.
    Started {
        /// Scene id.
        id: u32,
        /// Scene time already played.
        offset_ms: u32,
    },
    /// The scene ended.
    Closed(u32, EndReason),
    /// The scene's locks changed.
    Locked(u32),
    /// An ending slide to show.
    Slide {
        /// Scene id.
        id: u32,
        /// Slide index.
        index: u16,
    },
    /// A time skip to show (fade) and apply.
    TimeSkip {
        /// Event id.
        event: u32,
        /// Hours.
        hours: u16,
        /// Sleep rather than wait.
        sleeping: bool,
        /// The clock after it.
        clock: ClockSnap,
    },
    /// A host script's presentation to show or play here.
    Present(Cue),
}

struct CScene {
    info: SceneInfo,
    /// Local time of scene time zero (may be in the future).
    zero_local: Option<u64>,
    started: bool,
    done_sent: bool,
}

/// A scene as this machine should show it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SceneView {
    /// Scene id.
    pub id: u32,
    /// Kind.
    pub kind: SceneKind,
    /// Tag.
    pub tag: u32,
    /// Phase.
    pub phase: Phase,
    /// This player is in the scene.
    pub participant: bool,
    /// This player joined late and is held until it ends.
    pub waiting: bool,
    /// What this player cannot do.
    pub locks: Locks,
    /// Camera owner.
    pub camera: CameraMode,
    /// The movie.
    pub movie: Option<Movie>,
    /// Scene time played (0 before the start).
    pub elapsed_ms: u32,
    /// The scene's time zero has arrived.
    pub started: bool,
    /// Ending slide.
    pub slide: u16,
    /// Slides in the ending.
    pub slides: u16,
    /// Who may skip.
    pub skip: SkipRule,
}

/// Scene state of one machine.
pub struct SceneClient {
    me: PlayerId,
    scenes: BTreeMap<u32, CScene>,
    out: Vec<SceneMsg>,
    notices: Vec<Notice>,
    committed: Vec<u32>,
    last_event: u32,
    /// Time skips applied here.
    pub skips: u32,
    /// Scenes opened here so far.
    pub opened_total: u32,
    /// Scenes closed here so far.
    pub closed_total: u32,
    /// Newest clock from the host with the local time it arrived.
    pub clock: Option<(ClockSnap, u64)>,
    rtt_ms: u32,
    last_present: u32,
    /// Host presentations received here (each event once).
    pub presented: u32,
}

impl SceneClient {
    /// A client for player `me`.
    pub fn new(me: PlayerId) -> Self {
        SceneClient {
            me,
            scenes: BTreeMap::new(),
            out: Vec::new(),
            notices: Vec::new(),
            committed: Vec::new(),
            last_event: 0,
            skips: 0,
            opened_total: 0,
            closed_total: 0,
            clock: None,
            rtt_ms: 0,
            last_present: 0,
            presented: 0,
        }
    }

    /// The measured round trip, to line starts up.
    pub fn set_rtt(&mut self, ms: u32) {
        self.rtt_ms = ms;
    }

    /// Messages for the host.
    pub fn take_out(&mut self) -> Vec<SceneMsg> {
        std::mem::take(&mut self.out)
    }

    /// Things to react to.
    pub fn take_notices(&mut self) -> Vec<Notice> {
        std::mem::take(&mut self.notices)
    }

    /// Ending tags the party committed to.
    pub fn committed(&self) -> &[u32] {
        &self.committed
    }

    /// Open scene count.
    pub fn count(&self) -> usize {
        self.scenes.len()
    }

    /// The controls this player cannot use now: the locks of every scene they
    /// are in, everything while held as a late joiner.
    pub fn locks(&self) -> Locks {
        let mut l = Locks::NONE;
        for s in self.scenes.values() {
            if s.info.waiting.contains(self.me) {
                l = l.union(Locks::ALL);
            } else if s.info.participants.contains(self.me) {
                l = l.union(s.info.locks);
            }
        }
        l
    }

    /// True when any scene with a camera of its own includes this player.
    pub fn camera_taken(&self) -> bool {
        self.scenes.values().any(|s| {
            s.info.participants.contains(self.me) && s.info.spec.camera != CameraMode::Free
        })
    }

    /// The scenes as this machine shows them, lowest id first.
    pub fn views(&self, now_ms: u64) -> Vec<SceneView> {
        self.scenes
            .values()
            .map(|s| self.view_of(s, now_ms))
            .collect()
    }

    /// The scene one machine is mainly in (a participant's, playing first).
    pub fn primary(&self, now_ms: u64) -> Option<SceneView> {
        let v = self.views(now_ms);
        v.iter()
            .find(|s| s.participant && s.phase == Phase::Playing)
            .or_else(|| v.iter().find(|s| s.participant || s.waiting))
            .cloned()
    }

    fn view_of(&self, s: &CScene, now_ms: u64) -> SceneView {
        let elapsed = match s.zero_local {
            Some(z) if s.started => now_ms.saturating_sub(z).min(u64::from(u32::MAX)) as u32,
            _ => 0,
        };
        SceneView {
            id: s.info.id,
            kind: s.info.spec.kind,
            tag: s.info.spec.tag,
            phase: s.info.phase,
            participant: s.info.participants.contains(self.me),
            waiting: s.info.waiting.contains(self.me),
            locks: if s.info.waiting.contains(self.me) {
                Locks::ALL
            } else if s.info.participants.contains(self.me) {
                s.info.locks
            } else {
                Locks::NONE
            },
            camera: s.info.spec.camera,
            movie: s.info.spec.movie.clone(),
            elapsed_ms: elapsed,
            started: s.started,
            slide: s.info.slide,
            slides: s.info.spec.slides,
            skip: s.info.spec.skip,
        }
    }

    /// The game finished showing a scene here.
    pub fn done(&mut self, id: u32) {
        if let Some(s) = self.scenes.get_mut(&id) {
            if !s.done_sent {
                s.done_sent = true;
                self.out.push(SceneMsg::Done { id });
            }
        }
    }

    /// The player asks to skip (or withdraws).
    pub fn skip(&mut self, id: u32, yes: bool) {
        if self.scenes.contains_key(&id) {
            self.out.push(SceneMsg::SkipVote { id, yes });
        }
    }

    /// The player entered a running scene's cell and asks to merge in.
    pub fn join(&mut self, id: u32) {
        if self
            .scenes
            .get(&id)
            .is_some_and(|s| !s.info.participants.contains(self.me))
        {
            self.out.push(SceneMsg::Join { id });
        }
    }

    /// The link to the host is gone: every scene stops.
    pub fn abort_all(&mut self) {
        for id in self.scenes.keys().copied().collect::<Vec<_>>() {
            self.drop_scene(id, EndReason::Aborted);
        }
    }

    fn drop_scene(&mut self, id: u32, reason: EndReason) {
        if self.scenes.remove(&id).is_some() {
            self.closed_total += 1;
            self.notices.push(Notice::Closed(id, reason));
        }
    }

    fn upsert(&mut self, info: SceneInfo, now_ms: u64, replay: bool) {
        let id = info.id;
        let zero = (replay && info.phase == Phase::Playing)
            .then(|| now_ms.saturating_sub(u64::from(info.elapsed_ms)));
        // a replay (late join, merge, rejoin) of an ending shows the slide it is on
        let shown = self.scenes.get(&id).map_or(0, |s| s.info.slide);
        if info.spec.kind == SceneKind::Ending && info.slide > shown {
            self.notices.push(Notice::Slide {
                id,
                index: info.slide,
            });
        }
        match self.scenes.get_mut(&id) {
            Some(s) => {
                let was = s.info.phase;
                s.info.participants = info.participants;
                s.info.waiting = info.waiting;
                s.info.locks = info.locks;
                s.info.slide = s.info.slide.max(info.slide);
                if info.phase == Phase::Playing && was == Phase::Gather {
                    s.info.phase = Phase::Playing;
                    s.zero_local = zero;
                }
                if info.phase == Phase::Playing && s.zero_local.is_none() {
                    s.zero_local = zero;
                    s.info.phase = Phase::Playing;
                }
            }
            None => {
                self.opened_total += 1;
                self.notices.push(Notice::Opened(id));
                let participant = info.participants.contains(self.me);
                if participant && info.phase == Phase::Gather {
                    self.out.push(SceneMsg::Ready { id });
                }
                self.scenes.insert(
                    id,
                    CScene {
                        info,
                        zero_local: zero,
                        started: false,
                        done_sent: false,
                    },
                );
            }
        }
    }

    /// Applies one host message.
    pub fn on_msg(&mut self, msg: SceneMsg, now_ms: u64) {
        match msg {
            SceneMsg::Open { scene } => self.upsert(scene, now_ms, false),
            SceneMsg::Sync { scenes, committed } => {
                for t in committed {
                    if !self.committed.contains(&t) {
                        self.committed.push(t);
                    }
                }
                for mut info in scenes {
                    // a replay starts where the scene is, not at a lead
                    if info.phase == Phase::Playing {
                        info.elapsed_ms = info.elapsed_ms.saturating_add(self.rtt_ms / 2);
                    }
                    let id = info.id;
                    let had = self.scenes.contains_key(&id);
                    self.upsert(info.clone(), now_ms, true);
                    if had {
                        if let Some(s) = self.scenes.get_mut(&id) {
                            // merging in: restart on the host's elapsed time
                            s.zero_local = Some(now_ms.saturating_sub(u64::from(info.elapsed_ms)));
                            s.info.phase = info.phase;
                            s.info.participants = info.participants;
                            s.info.waiting = info.waiting;
                            s.info.locks = info.locks;
                        }
                    }
                }
            }
            SceneMsg::Go { id, start_in_ms } => {
                if let Some(s) = self.scenes.get_mut(&id) {
                    s.info.phase = Phase::Playing;
                    let lead = u64::from(start_in_ms).saturating_sub(u64::from(self.rtt_ms / 2));
                    s.zero_local = Some(now_ms + lead);
                }
            }
            SceneMsg::Merged { id, player } => {
                if let Some(s) = self.scenes.get_mut(&id) {
                    s.info.participants = s.info.participants.with(player);
                    s.info.waiting = s.info.waiting.without(player);
                }
            }
            SceneMsg::Lock { id, locks } => {
                if let Some(s) = self.scenes.get_mut(&id) {
                    s.info.locks = locks;
                    self.notices.push(Notice::Locked(id));
                }
            }
            SceneMsg::Slide { id, index } => {
                if let Some(s) = self.scenes.get_mut(&id) {
                    if index > s.info.slide {
                        s.info.slide = index;
                        self.notices.push(Notice::Slide { id, index });
                    }
                }
            }
            SceneMsg::Close { id, reason } => self.drop_scene(id, reason),
            SceneMsg::TimeSkip {
                event,
                hours,
                sleeping,
                clock,
            } => {
                if event > self.last_event {
                    self.last_event = event;
                    self.skips += 1;
                    self.clock = Some((clock, now_ms));
                    self.notices.push(Notice::TimeSkip {
                        event,
                        hours,
                        sleeping,
                        clock,
                    });
                }
            }
            SceneMsg::Clock { clock } => self.clock = Some((clock, now_ms)),
            SceneMsg::Present { event, cue } => {
                // event 0 replays a modifier that stays on: showing it again is harmless
                if event == 0 || event > self.last_present {
                    self.last_present = self.last_present.max(event);
                    self.presented += 1;
                    self.notices.push(Notice::Present(cue));
                }
            }
            SceneMsg::Ready { .. }
            | SceneMsg::Done { .. }
            | SceneMsg::SkipVote { .. }
            | SceneMsg::Join { .. } => {}
        }
    }

    /// Per-frame: scenes whose time zero arrived start now.
    pub fn tick(&mut self, now_ms: u64) {
        for s in self.scenes.values_mut() {
            if !s.started
                && s.info.phase == Phase::Playing
                && s.zero_local.is_some_and(|z| now_ms >= z)
            {
                s.started = true;
                let z = s.zero_local.unwrap_or(now_ms);
                self.notices.push(Notice::Started {
                    id: s.info.id,
                    offset_ms: now_ms.saturating_sub(z).min(u64::from(u32::MAX)) as u32,
                });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interest::Audience;
    use crate::scene::host::SceneHost;

    fn p(n: u8) -> PlayerId {
        PlayerId(n)
    }

    /// Delivers host output to the clients (index = player slot).
    fn deliver(outs: Vec<SOut>, cs: &mut [SceneClient], now: u64) {
        for o in outs {
            for c in cs.iter_mut() {
                let hit = match o.aud {
                    Audience::All => true,
                    Audience::To(t) => t == c.me,
                    Audience::AllExcept(x) => x != c.me,
                    _ => false,
                };
                if hit {
                    c.on_msg(o.msg.clone(), now);
                }
            }
        }
    }

    fn pump(h: &mut SceneHost, cs: &mut [SceneClient], now: u64) {
        for i in 0..cs.len() {
            for m in cs[i].take_out() {
                let outs = h.on_msg(cs[i].me, m, now);
                deliver(outs, cs, now);
            }
        }
    }

    fn setup() -> (SceneHost, Vec<SceneClient>) {
        let mut h = SceneHost::new();
        h.on_join(p(1), 0, None);
        h.on_join(p(2), 0, None);
        (
            h,
            vec![
                SceneClient::new(p(0)),
                SceneClient::new(p(1)),
                SceneClient::new(p(2)),
            ],
        )
    }

    fn movie() -> SceneSpec {
        let mut s = SceneSpec::new(SceneKind::Movie, 9);
        s.movie = Some(Movie {
            file: "a.bik".into(),
            interruptable: true,
            mute_audio: true,
            pause_music: true,
            letterbox: true,
        });
        s.locks = Locks::ALL;
        s.camera = CameraMode::Movie;
        s.skip = SkipRule::Unanimous;
        s
    }

    #[test]
    fn everyone_starts_together_and_locks_follow_the_participants() {
        let (mut h, mut cs) = setup();
        let (id, o) = h.open(movie(), 1_000).unwrap();
        deliver(o, &mut cs, 1_010);
        assert!(
            cs.iter().all(|c| c.locks() == Locks::ALL),
            "locked during the gather"
        );
        pump(&mut h, &mut cs, 1_030);
        for c in &mut cs {
            c.tick(1_100);
            assert!(
                c.take_notices()
                    .iter()
                    .all(|n| !matches!(n, Notice::Started { .. })),
                "not before time zero"
            );
        }
        let o = h.tick(1_040);
        assert!(o.is_empty());
        // host released at 1_030 (all ready): zero at 1_330
        for c in &mut cs {
            c.tick(1_330);
            let n = c.take_notices();
            assert!(
                n.iter()
                    .any(|n| matches!(n, Notice::Started { id: i, offset_ms: 0 } if *i == id)),
                "{n:?}"
            );
        }
        let v = cs[1].primary(1_830).unwrap();
        assert_eq!(v.elapsed_ms, 500);
        assert_eq!(v.movie.as_ref().unwrap().file, "a.bik");
    }

    #[test]
    fn a_unanimous_skip_ends_it_for_everyone_and_frees_the_controls() {
        let (mut h, mut cs) = setup();
        let (id, o) = h.open(movie(), 0).unwrap();
        deliver(o, &mut cs, 0);
        pump(&mut h, &mut cs, 1);
        for who in 0..3 {
            cs[who].skip(id, true);
            pump(&mut h, &mut cs, 10 + who as u64);
        }
        assert!(cs
            .iter()
            .all(|c| c.count() == 0 && c.locks() == Locks::NONE));
        assert!(cs[2]
            .take_notices()
            .iter()
            .any(|n| matches!(n, Notice::Closed(_, EndReason::Skipped))));
    }

    #[test]
    fn a_late_joiner_replays_the_remaining_scene_then_is_freed() {
        let (mut h, mut cs) = setup();
        let mut s = movie();
        s.barrier = BarrierRule::Roam;
        let (id, o) = h.open(s, 0).unwrap();
        deliver(o, &mut cs[..3], 0);
        let mut late = SceneClient::new(p(3));
        let o = h.on_join(p(3), 5_300, None);
        deliver(o, std::slice::from_mut(&mut late), 5_300);
        late.tick(5_300);
        let n = late.take_notices();
        assert!(n.iter().any(|n| matches!(n, Notice::Started { id: i, offset_ms } if *i == id && *offset_ms >= 4_999)), "{n:?}");
        assert_eq!(late.locks(), Locks::ALL);
        let o = h.end(id, EndReason::Finished, 6_000);
        deliver(o, std::slice::from_mut(&mut late), 6_000);
        assert_eq!(late.locks(), Locks::NONE);
    }

    #[test]
    fn time_skips_apply_once_even_when_the_message_repeats_or_reorders() {
        let mut c = SceneClient::new(p(1));
        let clock = ClockSnap {
            hour: 9.0f32.to_bits(),
            ..Default::default()
        };
        let m = |event| SceneMsg::TimeSkip {
            event,
            hours: 8,
            sleeping: true,
            clock,
        };
        c.on_msg(m(2), 0);
        c.on_msg(m(2), 1);
        c.on_msg(m(1), 2);
        assert_eq!(c.skips, 1);
        assert_eq!(c.take_notices().len(), 1);
        c.on_msg(m(3), 3);
        assert_eq!(c.skips, 2);
    }

    #[test]
    fn slides_arrive_in_order_only() {
        let (mut h, mut cs) = setup();
        let mut s = SceneSpec::new(SceneKind::Ending, 3);
        s.slides = 4;
        s.barrier = BarrierRule::Roam;
        let (id, o) = h.open(s, 0).unwrap();
        deliver(o, &mut cs, 0);
        cs[1].take_notices();
        cs[1].on_msg(SceneMsg::Slide { id, index: 2 }, 1);
        cs[1].on_msg(SceneMsg::Slide { id, index: 1 }, 2);
        let n = cs[1].take_notices();
        assert_eq!(n, vec![Notice::Slide { id, index: 2 }]);
        assert_eq!(cs[1].primary(3).unwrap().slide, 2);
    }

    #[test]
    fn a_late_joiner_to_an_ending_is_shown_the_slide_it_is_on() {
        let (mut h, mut cs) = setup();
        let mut s = SceneSpec::new(SceneKind::Ending, 3);
        s.slides = 8;
        s.barrier = BarrierRule::Roam;
        let (id, o) = h.open(s, 0).unwrap();
        deliver(o, &mut cs, 0);
        deliver(h.slide(id, 1), &mut cs, 1);
        deliver(h.slide(id, 2), &mut cs, 2);
        let mut late = SceneClient::new(p(3));
        let o = h.on_join(p(3), 5, None);
        deliver(o, std::slice::from_mut(&mut late), 5);
        let n = late.take_notices();
        assert!(n.contains(&Notice::Slide { id, index: 2 }), "{n:?}");
        // a repeat of the same replay shows nothing new
        let o = h.resync(p(3), 6, None);
        deliver(o, std::slice::from_mut(&mut late), 6);
        assert!(!late
            .take_notices()
            .iter()
            .any(|n| matches!(n, Notice::Slide { .. })));
    }

    #[test]
    fn a_host_presentation_reaches_every_guest_once_and_a_joiner_sees_the_fade_left_on() {
        let (mut h, mut cs) = setup();
        let fade = Cue::ImageSpace {
            form: 0x174411,
            on: true,
        };
        let o = h.present(fade);
        deliver(o.clone(), &mut cs, 1);
        // a duplicate of the same event is shown once
        deliver(o, &mut cs, 2);
        deliver(h.present(Cue::Sound { form: 9 }), &mut cs, 3);
        assert!(
            cs[0].take_notices().is_empty(),
            "the host showed it already"
        );
        for c in &mut cs[1..] {
            assert_eq!(
                c.take_notices(),
                vec![
                    Notice::Present(fade),
                    Notice::Present(Cue::Sound { form: 9 })
                ]
            );
            assert_eq!(c.presented, 2);
        }
        let mut late = SceneClient::new(p(3));
        deliver(h.on_join(p(3), 5, None), std::slice::from_mut(&mut late), 5);
        assert!(late.take_notices().contains(&Notice::Present(fade)));
        // the fade removed: no replay for the next joiner
        h.present(Cue::ImageSpace {
            form: 0x174411,
            on: false,
        });
        assert!(h.imods().is_empty());
    }

    #[test]
    fn the_host_link_dropping_aborts_every_scene() {
        let (mut h, mut cs) = setup();
        let (_, o) = h.open(movie(), 0).unwrap();
        deliver(o, &mut cs, 0);
        cs[1].take_notices();
        cs[1].abort_all();
        assert_eq!(cs[1].count(), 0);
        assert!(cs[1]
            .take_notices()
            .iter()
            .any(|n| matches!(n, Notice::Closed(_, EndReason::Aborted))));
        assert_eq!(cs[1].locks(), Locks::NONE);
    }

    #[test]
    fn a_roamer_asks_to_join_and_merges_in() {
        let (mut h, mut cs) = setup();
        let mut s = SceneSpec::new(SceneKind::Cinematic, 4);
        s.barrier = BarrierRule::Roam;
        s.participants = Mask(0b001);
        s.locks = Locks(Locks::MOVEMENT);
        s.camera = CameraMode::Scripted;
        let (id, o) = h.open(s, 0).unwrap();
        deliver(o, &mut cs, 0);
        assert_eq!(cs[2].locks(), Locks::NONE);
        assert!(!cs[2].camera_taken());
        cs[2].join(id);
        pump(&mut h, &mut cs, 2_000);
        assert_eq!(cs[2].locks(), Locks(Locks::MOVEMENT));
        assert!(cs[2].camera_taken());
        cs[2].tick(2_000);
        assert!(cs[2]
            .take_notices()
            .iter()
            .any(|n| matches!(n, Notice::Started { offset_ms, .. } if *offset_ms >= 1_600)));
    }

    #[test]
    fn rtt_pulls_the_guests_start_earlier() {
        let mut c = SceneClient::new(p(1));
        c.set_rtt(200);
        let mut h = SceneHost::new();
        h.on_join(p(1), 0, None);
        let mut s = SceneSpec::new(SceneKind::Cinematic, 1);
        s.barrier = BarrierRule::Roam;
        let (_, o) = h.open(s, 0).unwrap();
        for x in o {
            c.on_msg(x.msg, 1_000);
        }
        c.tick(1_000 + u64::from(GO_LEAD_MS) - 100);
        assert!(c
            .take_notices()
            .iter()
            .any(|n| matches!(n, Notice::Started { .. })));
    }
}

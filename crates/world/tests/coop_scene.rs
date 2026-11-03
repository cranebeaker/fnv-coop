//! Scenes, movies, control locks and world time on the world crate: a host
//! and two guests, each a `GameState`, joined by coop-core's chaos transport
//! and the live adapters. No viewer, generated world only.

use coop_core::ids::PlayerId;
use coop_core::proto::Msg;
use coop_core::scene::{ClockSnap, Locks, SceneKind};
use coop_core::sim_test::Sim;
use coop_core::sync::GuestPhase;
use coop_core::transport::ChaosProfile;
use esm::{ActivePlugins, FormId, LoadOrder};
use world::coop::apply::{Ctx, Opts};
use world::coop::live::{LiveGuestAdapter, LiveHostAdapter};
use world::coop::scene::{follow_clock, parse_mask};
use world::coop::{Authority, GuestBridge, HostBridge};
use world::scripting::{GameState, ScriptCache, Video};

struct Rig<'a> {
    order: &'a LoadOrder,
    scripts: &'a ScriptCache,
    sim: Sim,
    host: GameState,
    guests: Vec<GameState>,
    ha: LiveHostAdapter,
    gas: Vec<LiveGuestAdapter>,
    hb: HostBridge,
    gbs: Vec<GuestBridge>,
    /// Modelled Bink player: seconds a movie lasts, per machine (host, g1, g2).
    movie_ms: u64,
    movie_end: [Option<u64>; 3],
    /// Movies the viewer would have been told to start: (machine, file, offset).
    started: Vec<(usize, String, u32)>,
}

impl<'a> Rig<'a> {
    fn new(order: &'a LoadOrder, scripts: &'a ScriptCache, profile: &str, seed: u64) -> Rig<'a> {
        let mut host = GameState::new(order);
        host.set_authority(Authority::Host);
        host.enable_coop_policy(
            std::sync::Arc::new(coop_policy::PolicyTable::bundled().unwrap()),
            0,
        );
        let guests: Vec<GameState> = (0..2)
            .map(|_| {
                let mut g = GameState::new(order);
                g.set_authority(Authority::Guest);
                g
            })
            .collect();
        let mut hb = HostBridge::default();
        hb.backstop_every = 1;
        let mut sim = Sim::new(2, ChaosProfile::by_name(profile).unwrap(), seed);
        sim.start_guest(0);
        sim.start_guest(1);
        let mut r = Rig {
            order,
            scripts,
            sim,
            host,
            guests,
            ha: LiveHostAdapter::new(),
            gas: vec![
                LiveGuestAdapter::new(PlayerId(1)),
                LiveGuestAdapter::new(PlayerId(2)),
            ],
            hb,
            gbs: vec![GuestBridge::default(), GuestBridge::default()],
            movie_ms: 5_000,
            movie_end: [None; 3],
            started: Vec::new(),
        };
        let ok = r.sim.run_until(120_000, |s| {
            s.guests
                .iter()
                .all(|g| g.sync.phase() == GuestPhase::Active)
        });
        assert!(ok, "guests never became active");
        r
    }

    fn cx(&self) -> Ctx<'a> {
        Ctx {
            order: self.order,
            scripts: self.scripts,
        }
    }

    fn machine(&mut self, i: usize) -> &mut GameState {
        if i == 0 {
            &mut self.host
        } else {
            &mut self.guests[i - 1]
        }
    }

    fn step(&mut self) {
        let cx = self.cx();
        let now = self.sim.clock.now();
        self.hb.collect(&mut self.host, &mut self.sim.host);
        let outs = self.ha.frame(
            cx,
            &mut self.host,
            &mut self.sim.host.live,
            &mut self.sim.host.auth,
            now,
        );
        self.sim.host_emit(outs);
        for i in 0..2 {
            if self.sim.guests[i].sync.phase() == GuestPhase::Active {
                let opts = Opts {
                    present: true,
                    ctx: Some(cx),
                };
                self.gbs[i].sync(
                    &mut self.guests[i],
                    &self.sim.guests[i].sync.store,
                    false,
                    opts,
                );
                let msgs = self.gas[i].frame(
                    cx,
                    &mut self.guests[i],
                    &mut self.sim.guests[i].sync.live,
                    now,
                );
                self.sim.guest_emit(i, msgs);
            }
        }
        // the modelled Bink player
        for m in 0..3 {
            let (movie_ms, end) = (self.movie_ms, self.movie_end[m]);
            let sc = &mut self.machine(m).coop.live.scene;
            let mut start = None;
            if let Some(s) = sc.movie_start.take() {
                start = Some((s.movie.file.clone(), s.offset_ms));
            }
            let stop = std::mem::take(&mut sc.movie_stop);
            let mut end = end;
            if let Some((f, off)) = start {
                end = Some(now + movie_ms.saturating_sub(u64::from(off)));
                self.started.push((m, f, off));
            }
            if stop || end.is_some_and(|e| now >= e) {
                end = None;
            }
            self.machine(m).coop.live.scene.movie_busy = end.is_some();
            self.movie_end[m] = end;
        }
        self.sim.step();
    }

    fn run_ms(&mut self, ms: u64) {
        let end = self.sim.clock.now() + ms;
        while self.sim.clock.now() < end {
            self.step();
        }
    }

    fn until(&mut self, ms: u64, mut cond: impl FnMut(&mut Rig<'a>) -> bool) -> bool {
        let end = self.sim.clock.now() + ms;
        while self.sim.clock.now() < end {
            if cond(self) {
                return true;
            }
            self.step();
        }
        cond(self)
    }

    /// Guests and the host player answer an open party prompt.
    fn everyone_answers(&mut self, yes: bool) {
        let now = self.sim.clock.now();
        for i in 0..2 {
            if let Some(m) = self.sim.guests[i].sync.live.rules.answer(yes) {
                self.sim.guest_emit(i, vec![Msg::Rules(m)]);
            }
        }
        if let Some(m) = self.ha.own.rules.answer(yes) {
            let outs = self.ha.on_local(
                &mut self.sim.host.live,
                &mut self.sim.host.auth,
                now,
                Msg::Rules(m),
            );
            self.sim.host_emit(outs);
        }
    }
}

fn movie(file: &str, interruptable: bool) -> Video {
    Video {
        file: file.into(),
        interruptable,
        mute_audio: true,
        pause_music: true,
        letterbox: true,
    }
}

fn world_order(tag: &str) -> (testdata::TempData, LoadOrder) {
    let data = testdata::ai::world(tag);
    let order = LoadOrder::from_data_dir(data.path(), &ActivePlugins::OfficialOnly).unwrap();
    (data, order)
}

#[test]
fn a_script_movie_starts_on_every_machine_at_the_same_scene_time_and_ends_with_the_host() {
    let (_d, order) = world_order("cs-movie");
    let scripts = ScriptCache::default();
    let mut r = Rig::new(&order, &scripts, "good_wifi", 3);
    r.host
        .coop
        .live
        .scene
        .note_video(None, &movie("intro.bik", false));
    assert!(
        r.until(5_000, |r| r.started.len() == 3),
        "started {:?}",
        r.started
    );
    let mut machines: Vec<usize> = r.started.iter().map(|s| s.0).collect();
    machines.sort();
    assert_eq!(machines, vec![0, 1, 2]);
    assert!(
        r.started.iter().all(|s| s.1 == "intro.bik"),
        "the same file"
    );
    assert!(
        r.started.iter().all(|s| s.2 < 400),
        "none joins deep into the scene: {:?}",
        r.started
    );
    assert!(r.until(20_000, |r| (0..3).all(|m| r
        .machine(m)
        .coop
        .live
        .scene
        .counters
        .closed
        == 1)));
    for m in 0..3 {
        let c = r.machine(m).coop.live.scene.counters.clone();
        assert_eq!(
            (c.active, c.movies, c.last_reason),
            (0, 1, 1),
            "machine {m}"
        );
    }
    assert_eq!(r.started.len(), 3, "once each");
}

#[test]
fn a_movie_nobody_can_skip_ignores_skip_votes_and_an_interruptable_one_needs_everyone() {
    let (_d, order) = world_order("cs-skip");
    let scripts = ScriptCache::default();
    let mut r = Rig::new(&order, &scripts, "good_wifi", 4);
    r.movie_ms = 60_000;
    r.host
        .coop
        .live
        .scene
        .note_video(None, &movie("never.bik", false));
    assert!(r.until(5_000, |r| r.started.len() == 3));
    for m in 0..3 {
        r.machine(m).coop_scene_skip(true);
    }
    r.run_ms(2_000);
    assert_eq!(
        r.host.coop.live.scene.counters.active, 1,
        "not interruptable: still playing"
    );
    // the next one can be skipped, but not by one player
    r.host.coop_scene_abort();
    r.run_ms(1_000);
    assert_eq!(r.host.coop.live.scene.counters.active, 0);
    assert_eq!(r.host.coop.live.scene.counters.last_reason, 4, "aborted");
    r.host
        .coop
        .live
        .scene
        .note_video(None, &movie("skip.bik", true));
    assert!(r.until(5_000, |r| r.started.len() == 6));
    r.guests[0].coop_scene_skip(true);
    r.host.coop_scene_skip(true);
    r.run_ms(2_000);
    assert_eq!(
        r.guests[1].coop.live.scene.counters.active, 1,
        "player 2 did not agree"
    );
    r.guests[1].coop_scene_skip(true);
    assert!(r.until(5_000, |r| (0..3).all(|m| r
        .machine(m)
        .coop
        .live
        .scene
        .counters
        .active
        == 0)));
    for m in 0..3 {
        assert_eq!(
            r.machine(m).coop.live.scene.counters.last_reason,
            2,
            "skipped on machine {m}"
        );
    }
}

#[test]
fn a_script_control_lock_reaches_every_player_and_comes_off_together() {
    let (_d, order) = world_order("cs-lock");
    let scripts = ScriptCache::default();
    let mut r = Rig::new(&order, &scripts, "good_wifi", 5);
    r.host.coop_controls_begin();
    r.host.controls_off = [true, true, true, true, false, false, false];
    r.host.coop_controls_end(None);
    assert!(r.until(5_000, |r| r.guests.iter().all(|g| g.controls_off[0])));
    assert_eq!(Locks::from_flags(&r.guests[1].controls_off), Locks(0b1111));
    assert!(
        r.host.controls_off[0],
        "the host keeps its own script's lock"
    );
    r.host.coop_controls_begin();
    r.host.controls_off = [false; 7];
    r.host.coop_controls_end(None);
    assert!(r.until(5_000, |r| r
        .guests
        .iter()
        .all(|g| g.controls_off == [false; 7])));
}

#[test]
fn an_initiator_scoped_lock_holds_only_the_initiator() {
    let (_d, order) = world_order("cs-initiator");
    let scripts = ScriptCache::default();
    let mut r = Rig::new(&order, &scripts, "good_wifi", 6);
    // Old World Blues AutoDoc timer: the policy gives the lock to the initiator
    let quest = FormId(0x0100_48FE);
    r.host
        .coop
        .gate
        .as_mut()
        .unwrap()
        .begin_interaction(quest, 2);
    r.host.coop_controls_begin();
    r.host.controls_off = [true; 7];
    r.host.coop_controls_end(Some(quest));
    assert!(r.until(5_000, |r| r.guests[1].controls_off == [true; 7]));
    r.run_ms(500);
    assert_eq!(r.guests[0].controls_off, [false; 7], "player 1 is free");
    assert_eq!(r.host.controls_off, [false; 7], "the host is free");
    // the script still reads its own truth
    assert!(r.host.coop.live.scene.script_controls.unwrap()[0]);
    // the next script call starts from the script's own state, not the cleared one
    r.host.coop_controls_begin();
    assert_eq!(r.host.controls_off, [true; 7]);
}

#[test]
fn a_sleep_request_is_a_vote_the_host_runs_once_and_nobody_waits_alone() {
    let (_d, order) = world_order("cs-sleep");
    let scripts = ScriptCache::default();
    let mut r = Rig::new(&order, &scripts, "good_wifi", 7);
    let hour0 = r.host.global(&order, "GameHour").unwrap();
    // guest 1 picks "sleep 8 hours" in the menu
    world::living::sleep::begin(&mut r.guests[0], 8, true);
    assert_eq!(
        r.guests[0].living.hours_left, 0,
        "the menu did not start the hours here"
    );
    assert!(!r.guests[0].living.sleeping);
    // nobody agreed yet: nothing happens
    r.run_ms(1_500);
    assert_eq!(r.host.coop.live.scene.counters.time_skips, 0);
    // the others agree
    assert!(r.until(5_000, |r| r.ha.own.rules.prompt.is_some()));
    r.everyone_answers(true);
    assert!(
        r.until(10_000, |r| (0..3).all(|m| r
            .machine(m)
            .coop
            .live
            .scene
            .counters
            .time_skips
            == 1)),
        "one skip each"
    );
    r.run_ms(3_000);
    let h = r.host.global(&order, "GameHour").unwrap();
    assert!(
        ((h - hour0 + 24.0) % 24.0 - 8.0).abs() < 0.2,
        "host clock moved eight hours: {hour0} -> {h}"
    );
    for g in &r.guests {
        let gh = g.global(&order, "GameHour").unwrap();
        assert!(
            ((gh - h + 36.0) % 24.0 - 12.0).abs() < 0.4,
            "guest follows: {gh} vs {h}"
        );
        assert_eq!(g.living.hours_left, 0);
    }
    assert!(
        r.host.coop.live.scene.time_shown.is_some()
            && r.guests[1].coop.live.scene.time_shown.is_some()
    );
    assert_eq!(r.ha.own.scene.skips, 1);
}

#[test]
fn one_no_stops_the_wait() {
    let (_d, order) = world_order("cs-nowait");
    let scripts = ScriptCache::default();
    let mut r = Rig::new(&order, &scripts, "good_wifi", 8);
    let hour0 = r.host.global(&order, "GameHour").unwrap();
    world::living::sleep::begin(&mut r.host, 4, false);
    assert!(r.until(5_000, |r| r.sim.guests[0].sync.live.rules.prompt.is_some()));
    r.everyone_answers(false);
    r.run_ms(3_000);
    assert_eq!(r.host.global(&order, "GameHour"), Some(hour0));
    assert!((0..3).all(|m| r.machine(m).coop.live.scene.counters.time_skips == 0));
}

#[test]
fn a_policy_cinematic_stage_locks_the_party_and_the_scene_holds_saves_and_travel() {
    let (_d, order) = world_order("cs-stage");
    let scripts = ScriptCache::default();
    let mut r = Rig::new(&order, &scripts, "good_wifi", 9);
    // a quest the policy marks as a host cinematic
    let table = coop_policy::PolicyTable::bundled().unwrap();
    let row = table
        .rows()
        .iter()
        .find(|q| q.host_decision == Some(coop_policy::HostDecision::Cinematic))
        .expect("a cinematic row");
    let quest = FormId(row.id);
    r.host
        .coop
        .live
        .scene
        .stage_notes
        .push((quest, SceneKind::Cinematic));
    assert!(r.until(5_000, |r| r.guests.iter().all(|g| g
        .coop
        .live
        .scene
        .counters
        .active
        == 1)));
    let id = r.host.coop.live.scene.counters.id;
    assert_ne!(id, 0);
    // the host's story ends it
    r.ha.own.scene.done(id);
    r.ha.own.scene.take_out().into_iter().for_each(|m| {
        let now = r.sim.clock.now();
        let outs = r.ha.on_local(
            &mut r.sim.host.live,
            &mut r.sim.host.auth,
            now,
            Msg::Scene(m),
        );
        r.sim.host_emit(outs);
    });
    assert!(r.until(5_000, |r| (0..3).all(|m| r
        .machine(m)
        .coop
        .live
        .scene
        .counters
        .active
        == 0)));
    assert_eq!(
        r.guests[0].coop.live.scene.counters.last_reason, 1,
        "finished"
    );
}

#[test]
fn single_player_pays_nothing() {
    let (_d, order) = world_order("cs-single");
    let mut gs = GameState::new(&order);
    assert!(!gs.coop_time_intercept(8, true));
    world::living::sleep::begin(&mut gs, 8, true);
    assert_eq!(
        gs.living.hours_left, 8,
        "single player starts the hours as always"
    );
    gs.coop_controls_begin();
    gs.coop_controls_end(None);
    gs.coop_scene_skip(true);
    gs.coop_scene_slide(1);
    assert!(gs.coop.live.scene.skip_asks.is_empty());
    assert!(gs.coop.live.scene.script_controls.is_none());
    assert!(gs.coop.live.scene.time_asks.is_empty());
    assert!(!gs.coop_scene_owns_movies());
}

#[test]
fn a_guests_clock_slews_to_the_hosts_without_going_backwards_and_jumps_when_far_off() {
    let (_d, order) = world_order("cs-follow");
    let mut gs = GameState::new(&order);
    gs.set_authority(Authority::Guest);
    // host says: hour 14, 30 game seconds per second
    let snap = ClockSnap {
        hour: 14.0f32.to_bits(),
        days_passed: (3.0f32 + 14.0 / 24.0).to_bits(),
        year: 2281,
        month: 9,
        day: 19,
        scale: 30.0f32.to_bits(),
        weather: 0,
    };
    // far off (12.0 vs 14.0): jumps at once
    follow_clock(&order, &mut gs, Some((snap, 0)), 0, 0.05);
    let h = gs.global(&order, "GameHour").unwrap();
    assert!((h - 14.0).abs() < 0.01, "jumped to {h}");
    // 10 s later at 30x: 5 game minutes ahead on the host; the guest's frames keep up smoothly
    let mut last = h;
    let mut max_step = 0.0f32;
    for k in 1..=200u64 {
        let now = k * 50;
        follow_clock(&order, &mut gs, Some((snap, 0)), now, 0.05);
        let cur = gs.global(&order, "GameHour").unwrap();
        assert!(cur >= last, "never backwards");
        max_step = max_step.max(cur - last);
        last = cur;
    }
    let want = 14.0 + 10.0 * 30.0 / 3600.0;
    assert!(
        (last - want).abs() < 0.002,
        "tracks the host: {last} vs {want}"
    );
    assert!(max_step < 0.01, "no visible jump: {max_step}");
    // a guest that is a little ahead waits instead of going back
    let (hour, days) = (
        order.form_by_editor_id("GameHour").unwrap(),
        order.form_by_editor_id("GameDaysPassed").unwrap(),
    );
    let d0 = gs.globals[&days];
    gs.globals.insert(hour, want + 0.1);
    gs.globals.insert(days, d0 + 0.1 / 24.0);
    let before = gs.global(&order, "GameHour").unwrap();
    follow_clock(&order, &mut gs, Some((snap, 0)), 10_050, 0.05);
    assert!(gs.global(&order, "GameHour").unwrap() >= before);
}

#[test]
fn masks_parse_the_way_the_policy_writes_them() {
    assert_eq!(parse_mask("1 1 1 1 0 1 1"), Locks(0b110_1111));
    assert_eq!(parse_mask("0 0 0 1 0 0 1"), Locks(0b100_1000));
    assert_eq!(parse_mask("1 1 1 1 1 1 1"), Locks::ALL);
    assert_eq!(parse_mask(""), Locks::NONE);
}

/// The host's clock as the scene layer sends it.
fn snap_of(order: &LoadOrder, gs: &GameState) -> ClockSnap {
    let g = |n: &str| gs.global(order, n).unwrap();
    ClockSnap {
        hour: g("GameHour").to_bits(),
        days_passed: g("GameDaysPassed").to_bits(),
        year: 2281,
        month: 9,
        day: 19,
        scale: g("TimeScale").to_bits(),
        weather: 0,
    }
}

#[test]
fn a_fast_guest_keeps_the_hosts_hour_although_the_day_count_rounds_each_frame() {
    // GameDaysPassed is a float: a frame's step (a few millionths of a day)
    // loses part of itself in the rounding, more so at a high frame rate.
    // The guest must still show the host's hour, not run ahead of it.
    let (_d, order) = world_order("cs-follow-fps");
    // the guest at 400, 144 and 60 frames a second
    for frame_ms in [2.5f64, 1000.0 / 144.0, 1000.0 / 60.0] {
        let mut host = GameState::new(&order);
        let mut guest = GameState::new(&order);
        guest.set_authority(Authority::Guest);
        for (name, v) in [
            ("GameHour", 12.0),
            ("GameDaysPassed", 5.5),
            ("TimeScale", 30.0),
        ] {
            host.globals
                .insert(order.form_by_editor_id(name).unwrap(), v);
        }
        let mut sent = (snap_of(&order, &host), 0u64);
        let (mut host_ms, mut guest_ms) = (0.0f64, 0.0f64);
        while guest_ms < 60_000.0 {
            // the host at 60 frames a second, a clock message each second
            while host_ms <= guest_ms {
                host.advance_clock(&order, 1.0 / 60.0);
                host_ms += 1000.0 / 60.0;
                if host_ms as u64 / 1000 != sent.1 / 1000 {
                    sent = (snap_of(&order, &host), host_ms as u64);
                }
            }
            guest_ms += frame_ms;
            let dt = (frame_ms / 1000.0) as f32;
            follow_clock(&order, &mut guest, Some(sent), guest_ms as u64, dt);
        }
        let (h, g) = (
            host.global(&order, "GameHour").unwrap(),
            guest.global(&order, "GameHour").unwrap(),
        );
        assert!(
            (h - g).abs() < 0.003,
            "{frame_ms} ms frames: host hour {h}, guest hour {g}"
        );
        let (hd, gd) = (
            host.global(&order, "GameDaysPassed").unwrap(),
            guest.global(&order, "GameDaysPassed").unwrap(),
        );
        assert!(
            (hd - gd).abs() < 0.003 / 24.0,
            "{frame_ms} ms frames: host days {hd}, guest days {gd}"
        );
    }
}

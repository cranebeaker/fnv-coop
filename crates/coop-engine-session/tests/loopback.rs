//! Two (or three) games in one process, joined over coop-core's loopback and chaos transports: the
//! host plays a scripted opening, guests follow (docs/coop/DECISIONS.md D2's binding conditions).
//!
//! After every step each guest's *game* is read back (not its replica): every quest's stage, flags
//! and done stages must be a record the host's game really had at some point, never a mix; and no
//! story script may ever have run on a guest. At the end every guest's game equals the host's.

use std::collections::{BTreeMap, BTreeSet};

use coop_core::compat::plugin_list_hash;
use coop_core::ids::PlayerId;
use coop_core::proto::{Hello, PROTOCOL_VERSION};
use coop_core::sync::HostConfig;
use coop_core::transport::{ChaosProfile, ChaosTransport, LoopbackNet, Transport, VirtualClock};
use coop_engine::forms;
use coop_engine::quest::{self, QuestRecord};
use coop_engine::testkit::{Story, StorySpec};
use coop_engine_session::{echo, gate_guest, record, GuestDriver, HostDriver};

engine_core::assemble!();

const MQ: u32 = 0x0010_4c0c;
const SIDE: u32 = 0x0010_4c0d;
const LONG: u32 = 0x0010_4c0e;
const GLOBALS: [u32; 3] = [0x39, 0x3a, 0x3b];
/// The game clock's hour: shared as the clock, not as a story global.
const HOUR: u32 = 0x38;

fn spec() -> StorySpec {
    StorySpec {
        quests: vec![
            (MQ, vec![5, 10, 20, 30, 40, 100]),
            (SIDE, vec![0, 10, 50, 200]),
            (LONG, (0..60).map(|i| i * 4).collect()),
        ],
        globals: vec![(0x39, 0.0), (0x3a, 1.0), (0x3b, 0.0), (HOUR, 8.0)],
        objectives: vec![(MQ, vec![10, 20, 30]), (SIDE, vec![1])],
        vars: vec![(MQ, vec![1, 2, 3]), (SIDE, vec![4])],
        ..StorySpec::default()
    }
}

fn quest_ids() -> Vec<u32> {
    spec().quests.iter().map(|q| q.0).collect()
}

fn hello() -> Hello {
    let plugins = vec!["FalloutNV.esm".to_string()];
    Hello {
        proto: PROTOCOL_VERSION,
        build_hash: 1,
        rules_hash: 2,
        mods_hash: plugin_list_hash(&plugins),
        exe: "1.4.0.525".into(),
        volatile_hash: 3,
        plugins,
    }
}

/// One step of the host's scripted opening.
#[derive(Clone, Copy, Debug)]
enum Act {
    Stage(u32, u8),
    Flags(u32, u8),
    Global(u32, f32),
    Objective(u32, u32, u32),
    Var(u32, u32, f64),
    QuestScript,
}

/// An opening with stage runs, a burst of many stages in one frame (more keys than one delta
/// carries), quest flags, globals changing and changing back, and the story's own scripts.
fn opening() -> Vec<Vec<Act>> {
    use Act::*;
    let mut burst: Vec<Act> = (0..60).map(|i| Stage(LONG, i * 4)).collect();
    burst.push(Global(0x3b, 7.0));
    vec![
        vec![
            Stage(MQ, 5),
            QuestScript,
            Global(0x39, 1.0),
            Var(MQ, 1, 1.0),
        ],
        vec![Stage(MQ, 10), Objective(MQ, 10, 1), Stage(SIDE, 0)],
        vec![QuestScript, Global(0x3a, 2.5), Var(MQ, 2, -3.25)],
        burst,
        vec![
            Stage(MQ, 20),
            Objective(MQ, 10, 3),
            Objective(MQ, 20, 1),
            Stage(MQ, 30),
            Flags(SIDE, 1),
        ],
        vec![
            Global(0x39, 0.0),
            Stage(SIDE, 50),
            Var(SIDE, 4, 7.0),
            Objective(SIDE, 1, 1),
        ],
        vec![
            Stage(MQ, 40),
            Global(0x39, 1.0),
            QuestScript,
            Var(MQ, 1, 2.0),
        ],
        vec![
            Flags(MQ, 3),
            Stage(MQ, 100),
            Objective(MQ, 30, 3),
            Flags(SIDE, 0x41),
        ],
    ]
}

fn act(host: &mut Story, a: Act) {
    match a {
        Act::Stage(q, s) => assert!(host.set_stage(q, s)),
        Act::Flags(q, f) => host.set_quest_flags(q, f),
        Act::Global(g, v) => host.set_global(g, v),
        Act::Objective(q, o, st) => host.set_objective(q, o, st),
        Act::Var(q, v, x) => host.set_quest_var(q, v, x),
        Act::QuestScript => host.run_quest_script(),
    }
}

/// Every record each quest of the host's game has had, normalized.
type History = BTreeMap<u32, BTreeSet<Vec<u8>>>;

fn note_history(host: &mut Story, hist: &mut History) {
    for (q, _) in spec().quests {
        let r = quest::record(&mut host.e, q).unwrap();
        hist.entry(q).or_default().insert(encoded(&r));
    }
}

fn encoded(r: &QuestRecord) -> Vec<u8> {
    match record::encode(r) {
        Some(coop_core::schema::Field::Custom { data, .. }) => data,
        _ => unreachable!(),
    }
}

/// The guest's game holds, for every quest, a record the host really had.
fn assert_never_mixed(guest: &mut Story, hist: &History, when: &str) {
    for (q, seen) in hist {
        let r = quest::record(&mut guest.e, *q).unwrap();
        assert!(
            seen.contains(&encoded(&r)),
            "{when}: guest quest {q:08x} holds {r:?}, a state the host never had"
        );
    }
}

/// The guest's game equals the host's: every quest and every global.
fn assert_same_game(host: &mut Story, guest: &mut Story) {
    for (q, _) in spec().quests {
        assert_eq!(
            record::normalized(&quest::record(&mut guest.e, q).unwrap()),
            record::normalized(&quest::record(&mut host.e, q).unwrap()),
            "quest {q:08x}"
        );
    }
    for g in GLOBALS {
        assert_eq!(
            forms::global_value(&mut guest.e, g).map(f32::to_bits),
            forms::global_value(&mut host.e, g).map(f32::to_bits),
            "global {g:x}"
        );
    }
    for q in quest_ids() {
        assert_eq!(
            coop_engine::vars::quest_vars(&mut guest.e, q),
            coop_engine::vars::quest_vars(&mut host.e, q),
            "variables of quest {q:08x}"
        );
    }
}

/// A machine that sets out to join: gated from its first frame.
fn guest_story() -> Story {
    let mut g = Story::new(&spec());
    gate_guest(&mut g.e);
    g
}

struct Run {
    host: Story,
    hd: HostDriver,
    guests: Vec<(Story, GuestDriver)>,
    hist: History,
}

impl Run {
    fn new(guests: u8, delta_cap: usize) -> Run {
        let mut host = Story::new(&spec());
        let mut cfg = HostConfig::new(hello(), 0x5eed);
        cfg.delta_cap = delta_cap;
        let mut hd = HostDriver::new(cfg, quest_ids(), GLOBALS);
        hd.start(&mut host.e);
        let mut hist = History::new();
        note_history(&mut host, &mut hist);
        let guests = (1..=guests)
            .map(|p| (guest_story(), GuestDriver::new(PlayerId(p), hello())))
            .collect();
        Run {
            host,
            hd,
            guests,
            hist,
        }
    }

    /// One frame on every machine; each guest's game checked after its step.
    fn frame(&mut self, nets: &mut [Box<dyn Transport>], hostnet: &mut dyn Transport, now: u64) {
        self.hd.step(&mut self.host.e, hostnet, now);
        for (i, (g, d)) in self.guests.iter_mut().enumerate() {
            // The guest's game tries its quest scripts every frame, as the game does.
            g.run_quest_script();
            d.step(&mut g.e, nets[i].as_mut(), now);
            assert_never_mixed(g, &self.hist, &format!("t={now} guest {}", i + 1));
            assert_eq!(g.scripts_run(), 0, "a story script ran on guest {}", i + 1);
            assert!(
                echo(&mut g.e, &d.sync.store).is_empty(),
                "guest {} game differs from its replica",
                i + 1
            );
        }
    }
}

/// Plays the opening over plain loopback; guest `late` joins after `late_at` frames of it.
fn play(delta_cap: usize, late_at: usize) -> Run {
    let net = LoopbackNet::new();
    let mut hostnet = net.endpoint(PlayerId::HOST);
    let mut nets: Vec<Box<dyn Transport>> = vec![
        Box::new(net.endpoint(PlayerId(1))),
        Box::new(net.endpoint(PlayerId(2))),
    ];
    let mut run = Run::new(2, delta_cap);
    let mut late = Some(run.guests.pop().unwrap());
    let mut now = 0;
    for (i, frame) in opening().into_iter().enumerate() {
        if i == late_at {
            run.guests.push(late.take().unwrap());
        }
        for a in frame {
            act(&mut run.host, a);
            note_history(&mut run.host, &mut run.hist);
        }
        // A few frames between the host's actions, as a game runs.
        for _ in 0..3 {
            now += 50;
            run.frame(&mut nets, &mut hostnet, now);
        }
    }
    if let Some(g) = late.take() {
        run.guests.push(g);
    }
    // Settle: every delta sent, one key each when the cap is 1.
    for _ in 0..200 {
        now += 50;
        run.frame(&mut nets, &mut hostnet, now);
    }
    run
}

fn finish(run: &mut Run) {
    assert!(
        run.host.scripts_run() > 0,
        "the host's story ran its scripts"
    );
    for (g, d) in run.guests.iter_mut() {
        assert_same_game(&mut run.host, g);
        assert_eq!(d.refused, 0);
        assert_eq!(d.sync.desyncs_detected, 0);
        assert!(d.sync.hash_checks > 0, "hashes were compared");
        let (applying, story) = coop_engine::gate::skipped(&mut g.e);
        assert!(
            applying > 0,
            "result scripts reached the guest's runner and were skipped"
        );
        assert!(
            story > 0,
            "the guest's own quest scripts were tried and skipped"
        );
        assert_eq!(d.heal_refused, 0);
    }
}

#[test]
fn guests_follow_the_opening_one_key_per_delta() {
    let mut run = play(1, 2);
    finish(&mut run);
}

#[test]
fn guests_follow_the_opening_with_the_default_delta_size() {
    let mut run = play(24, 4);
    finish(&mut run);
}

#[test]
fn a_guest_that_joins_after_the_opening_starts_from_the_snapshot() {
    let mut run = play(24, usize::MAX);
    finish(&mut run);
    assert!(run.guests[1].1.sync.snapshots_loaded >= 1);
}

#[test]
fn guests_follow_over_a_bad_network() {
    for profile in ["bad_wifi", "flaky"] {
        let net = LoopbackNet::new();
        let clock = VirtualClock::new();
        let chaos = |id: u8, seed: u64| {
            ChaosTransport::new(
                net.endpoint(PlayerId(id)),
                clock.clone(),
                ChaosProfile::by_name(profile).unwrap(),
                seed,
            )
        };
        let mut hostnet = chaos(0, 1);
        let mut g1 = chaos(1, 2);
        let mut g2 = chaos(2, 3);
        let mut run = Run::new(2, 1);
        let mut now = 0;
        let step = |run: &mut Run,
                    hostnet: &mut ChaosTransport<_>,
                    g1: &mut ChaosTransport<_>,
                    g2: &mut ChaosTransport<_>,
                    now: &mut u64| {
            *now += 50;
            clock.advance(50);
            hostnet.pump();
            g1.pump();
            g2.pump();
            run.hd.step(&mut run.host.e, hostnet, *now);
            for (i, (g, d)) in run.guests.iter_mut().enumerate() {
                let net: &mut dyn Transport = if i == 0 { &mut *g1 } else { &mut *g2 };
                g.run_quest_script();
                d.step(&mut g.e, net, *now);
                assert_never_mixed(g, &run.hist, &format!("{profile} t={now}"));
                assert_eq!(g.scripts_run(), 0);
            }
        };
        for frame in opening() {
            for a in frame {
                act(&mut run.host, a);
                note_history(&mut run.host, &mut run.hist);
            }
            for _ in 0..3 {
                step(&mut run, &mut hostnet, &mut g1, &mut g2, &mut now);
            }
        }
        for _ in 0..1200 {
            step(&mut run, &mut hostnet, &mut g1, &mut g2, &mut now);
        }
        for (g, d) in run.guests.iter_mut() {
            assert_same_game(&mut run.host, g);
            assert_eq!(d.refused, 0, "{profile}");
            assert!(echo(&mut g.e, &d.sync.store).is_empty(), "{profile}");
        }
    }
}

#[test]
fn a_guest_whose_game_lacks_a_stage_refuses_the_record_and_keeps_its_own() {
    let net = LoopbackNet::new();
    let mut hostnet = net.endpoint(PlayerId::HOST);
    let mut gnet = net.endpoint(PlayerId(1));
    let mut host = Story::new(&spec());
    let mut hd = HostDriver::new(HostConfig::new(hello(), 1), quest_ids(), GLOBALS);
    hd.start(&mut host.e);
    // The guest's data is different: its main quest has no stage 100.
    let mut other = spec();
    other.quests[0].1.pop();
    let mut guest = Story::new(&other);
    let mut gd = GuestDriver::new(PlayerId(1), hello());
    assert!(host.set_stage(MQ, 100));
    for t in 1..20 {
        hd.step(&mut host.e, &mut hostnet, t * 50);
        gd.step(&mut guest.e, &mut gnet, t * 50);
    }
    assert!(gd.refused > 0);
    let r = quest::record(&mut guest.e, MQ).unwrap();
    assert_eq!((r.stage, r.done.len()), (0, 0));
    // The echo shows the difference instead of hiding it.
    assert!(!echo(&mut guest.e, &gd.sync.store).is_empty());
}

#[test]
fn a_guest_game_that_drifts_is_put_back_on_the_host_state() {
    let net = LoopbackNet::new();
    let mut hostnet = net.endpoint(PlayerId::HOST);
    let mut gnet = net.endpoint(PlayerId(1));
    let mut host = Story::new(&spec());
    let mut hd = HostDriver::new(HostConfig::new(hello(), 1), quest_ids(), GLOBALS);
    hd.start(&mut host.e);
    let mut guest = guest_story();
    let mut gd = GuestDriver::new(PlayerId(1), hello());
    assert!(host.set_stage(MQ, 10));
    host.set_global(0x39, 3.0);
    let mut t = 0;
    let mut frames =
        |n: u32, host: &mut Story, guest: &mut Story, hd: &mut HostDriver, gd: &mut GuestDriver| {
            for _ in 0..n {
                t += 50;
                hd.step(&mut host.e, &mut hostnet, t);
                gd.step(&mut guest.e, &mut gnet, t);
            }
        };
    frames(30, &mut host, &mut guest, &mut hd, &mut gd);
    assert_same_game(&mut host, &mut guest);
    // The guest's own game changes story state behind the replica's back: a global its own script
    // writes, and a stage done byte.
    guest.set_global(0x39, 99.0);
    let q = forms::quest(&mut guest.e, MQ).unwrap();
    guest
        .e
        .mem
        .set_u8(q + coop_engine::addr::QUEST_CURRENT_STAGE_OFFSET, 5);
    assert!(!echo(&mut guest.e, &gd.sync.store).is_empty());
    // Nothing changes on the host, so no delta comes: only the guest's own read-back can see it.
    frames(
        gd.heal_every as u32 + 1,
        &mut host,
        &mut guest,
        &mut hd,
        &mut gd,
    );
    assert!(gd.healed >= 2);
    assert_same_game(&mut host, &mut guest);
    assert_eq!(guest.scripts_run(), 0);
}

/// Steps the host and one guest `n` frames over loopback endpoints; the guest's game tries its
/// quest script every frame.
#[allow(clippy::too_many_arguments)]
fn frames(
    n: u32,
    t: &mut u64,
    host: &mut Story,
    hd: &mut HostDriver,
    hostnet: &mut dyn Transport,
    guest: &mut Story,
    gd: &mut GuestDriver,
    gnet: &mut dyn Transport,
) {
    for _ in 0..n {
        *t += 50;
        hd.step(&mut host.e, hostnet, *t);
        guest.run_quest_script();
        gd.step(&mut guest.e, gnet, *t);
    }
}

#[test]
fn a_guest_that_lost_the_host_rejoins_and_catches_up() {
    let net = LoopbackNet::new();
    let mut hostnet = net.endpoint(PlayerId::HOST);
    let mut gnet = net.endpoint(PlayerId(1));
    let mut host = Story::new(&spec());
    let mut hd = HostDriver::new(HostConfig::new(hello(), 1), quest_ids(), GLOBALS);
    hd.start(&mut host.e);
    let mut guest = guest_story();
    let mut gd = GuestDriver::new(PlayerId(1), hello());
    let mut t = 0;
    assert!(host.set_stage(MQ, 10));
    frames(
        30,
        &mut t,
        &mut host,
        &mut hd,
        &mut hostnet,
        &mut guest,
        &mut gd,
        &mut gnet,
    );
    assert_same_game(&mut host, &mut guest);
    // The connection drops: the host forgets the guest, everything in flight is lost, and the host
    // plays on alone for a while.
    hd.sync.disconnect(PlayerId(1));
    net.flush(PlayerId(1));
    net.flush(PlayerId::HOST);
    assert!(host.set_stage(MQ, 20));
    host.set_quest_var(MQ, 3, 9.0);
    host.set_global(0x3a, 5.0);
    for _ in 0..10 {
        t += 50;
        hd.step(&mut host.e, &mut hostnet, t);
    }
    net.flush(PlayerId(1));
    // The transport is back: the guest resumes from its replica.
    gd.rejoin(PlayerId(1), &mut gnet);
    frames(
        60,
        &mut t,
        &mut host,
        &mut hd,
        &mut hostnet,
        &mut guest,
        &mut gd,
        &mut gnet,
    );
    assert_same_game(&mut host, &mut guest);
    assert!(echo(&mut guest.e, &gd.sync.store).is_empty());
    assert_eq!(guest.scripts_run(), 0);
}

#[test]
fn a_host_that_starts_a_new_session_gives_a_rejoining_guest_the_whole_story() {
    let net = LoopbackNet::new();
    let mut hostnet = net.endpoint(PlayerId::HOST);
    let mut gnet = net.endpoint(PlayerId(1));
    let mut host = Story::new(&spec());
    let mut hd = HostDriver::new(HostConfig::new(hello(), 1), quest_ids(), GLOBALS);
    hd.start(&mut host.e);
    let mut guest = guest_story();
    let mut gd = GuestDriver::new(PlayerId(1), hello());
    let mut t = 0;
    assert!(host.set_stage(MQ, 10));
    host.set_objective(MQ, 10, 1);
    frames(
        30,
        &mut t,
        &mut host,
        &mut hd,
        &mut hostnet,
        &mut guest,
        &mut gd,
        &mut gnet,
    );
    // The session ends; the host's game goes on without co-op recording (say, it loads a later
    // save) before a new session starts. The new session's revisions start from zero again, below
    // what the guest had applied, and its journal never saw these changes.
    coop_engine::arm(&mut host.e, false);
    assert!(host.set_stage(SIDE, 50));
    host.set_quest_var(SIDE, 4, 1.5);
    let _ = coop_engine::drain(&mut host.e);
    net.flush(PlayerId(1));
    net.flush(PlayerId::HOST);
    let mut hd = HostDriver::new(HostConfig::new(hello(), 2), quest_ids(), GLOBALS);
    hd.start(&mut host.e);
    gd.rejoin(PlayerId(1), &mut gnet);
    frames(
        60,
        &mut t,
        &mut host,
        &mut hd,
        &mut hostnet,
        &mut guest,
        &mut gd,
        &mut gnet,
    );
    assert_same_game(&mut host, &mut guest);
    assert!(gd.sync.snapshots_loaded >= 2);
}

#[test]
fn a_quest_of_the_player_s_own_is_not_shared_and_its_script_runs_on_the_guest() {
    let net = LoopbackNet::new();
    let mut hostnet = net.endpoint(PlayerId::HOST);
    let mut gnet = net.endpoint(PlayerId(1));
    let mut host = Story::new(&spec());
    // SIDE is each player's own (as the opening and character creation are): not replicated.
    let shared: Vec<u32> = quest_ids().into_iter().filter(|q| *q != SIDE).collect();
    let mut hd = HostDriver::new(HostConfig::new(hello(), 1), shared, GLOBALS);
    hd.start(&mut host.e);
    let mut guest = guest_story();
    coop_engine::gate::set_own_quests(&mut guest.e, [SIDE]);
    let mut gd = GuestDriver::new(PlayerId(1), hello());
    let mut t = 0;
    // The host goes through its own SIDE and the shared MQ.
    assert!(host.set_stage(SIDE, 50));
    host.set_quest_var(SIDE, 4, 3.0);
    assert!(host.set_stage(MQ, 10));
    // The guest goes through its own SIDE differently, its quest script running.
    assert!(guest.set_stage(SIDE, 10));
    let before = guest.scripts_run();
    for _ in 0..40 {
        t += 50;
        hd.step(&mut host.e, &mut hostnet, t);
        guest.run_script_of(SIDE);
        guest.run_script_of(MQ);
        gd.step(&mut guest.e, &mut gnet, t);
    }
    // The shared quest arrived; the player's own did not, in either direction.
    assert_eq!(quest::record(&mut guest.e, MQ).unwrap().stage, 10);
    let own = quest::record(&mut guest.e, SIDE).unwrap();
    assert_eq!((own.stage, own.done), (10, vec![10]));
    assert_eq!(
        coop_engine::vars::quest_vars(&mut guest.e, SIDE).unwrap()[0].1,
        0
    );
    assert!(gd.sync.store.iter().all(|(k, _)| k.form.0 != SIDE));
    // The guest's own quest script ran every frame; the shared quest's never.
    assert_eq!(guest.scripts_run() - before, 40);
    assert_eq!(coop_engine::gate::ran_own(&mut guest.e), 40);
    assert!(echo(&mut guest.e, &gd.sync.store).is_empty());
    assert_eq!((gd.healed, gd.refused), (0, 0));
}

/// One host and one guest on a loopback network, the host's session started.
fn pair() -> (
    Story,
    HostDriver,
    Box<dyn Transport>,
    Story,
    GuestDriver,
    Box<dyn Transport>,
) {
    let net = LoopbackNet::new();
    let hostnet = Box::new(net.endpoint(PlayerId::HOST));
    let gnet = Box::new(net.endpoint(PlayerId(1)));
    let mut host = Story::new(&spec());
    let mut hd = HostDriver::new(HostConfig::new(hello(), 1), quest_ids(), GLOBALS);
    hd.set_clock([HOUR]);
    hd.start(&mut host.e);
    (
        host,
        hd,
        hostnet,
        guest_story(),
        GuestDriver::new(PlayerId(1), hello()),
        gnet,
    )
}

/// PC run 2026-11-10: the guest's Pip-Boy listed almost none of the host's quests. An objective
/// whose state is only written is not in the player's objective list, which is what the Pip-Boy
/// shows; the game's own function puts it there.
#[test]
fn the_hosts_objectives_are_in_the_guests_pip_boy() {
    let (mut host, mut hd, mut hostnet, mut guest, mut gd, mut gnet) = pair();
    let mut t = 0;
    // Before the guest joins: shown; shown then completed; completed without ever being shown.
    host.set_objective(MQ, 10, 1);
    host.set_objective(MQ, 20, 1);
    host.set_objective(MQ, 20, 3);
    host.set_objective(SIDE, 1, 2);
    frames(
        30,
        &mut t,
        &mut host,
        &mut hd,
        &mut *hostnet,
        &mut guest,
        &mut gd,
        &mut *gnet,
    );
    assert_same_game(&mut host, &mut guest);
    assert_eq!(guest.pipboy(), host.pipboy());
    assert_eq!(guest.pipboy(), [(MQ, 10), (MQ, 20), (SIDE, 1)]);
    // During play: one more shown, one completed.
    host.set_objective(MQ, 30, 1);
    host.set_objective(MQ, 10, 3);
    frames(
        30,
        &mut t,
        &mut host,
        &mut hd,
        &mut *hostnet,
        &mut guest,
        &mut gd,
        &mut *gnet,
    );
    assert_same_game(&mut host, &mut guest);
    assert_eq!(guest.pipboy(), [(MQ, 10), (MQ, 20), (SIDE, 1), (MQ, 30)]);
    assert_eq!((gd.refused, gd.healed, guest.scripts_run()), (0, 0, 0));
    assert_eq!(gd.relisted, 0);
    // The guest's game empties the player's list and keeps the objectives' states: put back.
    guest.empty_pipboy();
    frames(
        25,
        &mut t,
        &mut host,
        &mut hd,
        &mut *hostnet,
        &mut guest,
        &mut gd,
        &mut *gnet,
    );
    let mut listed = guest.pipboy();
    listed.sort_unstable();
    assert_eq!(listed, [(MQ, 10), (MQ, 20), (MQ, 30), (SIDE, 1)]);
    assert_eq!(gd.relisted, 4);
}

/// PC run 2026-11-10: night on the guest, day on the host. The clock is not story state (it moves
/// by itself on each machine), so it is shared on its own: the host's about once a second.
#[test]
fn the_guests_clock_is_kept_on_the_hosts() {
    let (mut host, mut hd, mut hostnet, mut guest, mut gd, mut gnet) = pair();
    let mut t = 0;
    let hour = |s: &mut Story| forms::global_value(&mut s.e, HOUR).unwrap();
    host.set_global(HOUR, 13.5);
    guest.set_global(HOUR, 2.0);
    frames(
        40,
        &mut t,
        &mut host,
        &mut hd,
        &mut *hostnet,
        &mut guest,
        &mut gd,
        &mut *gnet,
    );
    assert_eq!(hour(&mut guest), 13.5);
    // The host's clock stands still (a menu); the guest's runs on, and is set back.
    guest.set_global(HOUR, 14.25);
    frames(
        25,
        &mut t,
        &mut host,
        &mut hd,
        &mut *hostnet,
        &mut guest,
        &mut gd,
        &mut *gnet,
    );
    assert_eq!(hour(&mut guest), 13.5);
    // The clock moving is not a story difference: nothing is healed or reported for it.
    host.set_global(HOUR, 13.75);
    guest.set_global(HOUR, 13.6);
    assert!(echo(&mut guest.e, &gd.sync.store).is_empty());
    frames(
        40,
        &mut t,
        &mut host,
        &mut hd,
        &mut *hostnet,
        &mut guest,
        &mut gd,
        &mut *gnet,
    );
    assert_eq!(hour(&mut guest), 13.75);
    assert_eq!((gd.healed, gd.refused, gd.sync.desyncs_detected), (0, 0, 0));
}

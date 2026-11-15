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
/// Two placed NPCs.
const CHET: u32 = 0x0010_4f25;
const SUNNY: u32 = 0x0010_4e85;

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
        actors: vec![CHET, SUNNY],
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

/// PC run 4, 2026-11-10: an NPC the host had killed was alive, and trading, in the guest's game.
/// A death in the host's game is the guest's too, through the game's own kill, and the death's
/// scripts do not run a second time there.
#[test]
fn an_actor_the_host_kills_dies_in_the_guests_game() {
    let (mut host, mut hd, mut hostnet, mut guest, mut gd, mut gnet) = pair();
    let mut t = 0;
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
    // Sunny is not in the guest's game now (her cell is not loaded there).
    guest.set_present(SUNNY, false);
    let before = host.scripts_run();
    host.kill(CHET);
    host.kill(SUNNY);
    assert_eq!(
        host.scripts_run(),
        before + 2,
        "the deaths' scripts on the host"
    );
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
    assert_eq!(guest.dead(CHET), Some(true));
    assert_eq!(
        guest.scripts_run(),
        0,
        "no death script runs again on the guest"
    );
    // Nothing is refused or counted as a difference for the actor that is not there, and the
    // death is not sent back to the host as the guest's own.
    assert_eq!((gd.refused, gd.heal_refused, gd.claims_sent), (0, 0, 0));
    assert!(echo(&mut guest.e, &gd.sync.store).is_empty());
    assert_eq!((hd.claims_enacted, hd.claims_dropped), (0, 0));
    // Her cell loads on the guest: she is alive there until the next check, then dead.
    guest.set_present(SUNNY, true);
    assert_eq!(guest.dead(SUNNY), Some(false));
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
    assert_eq!(guest.dead(SUNNY), Some(true));
    assert_eq!(guest.scripts_run(), 0);
    assert_eq!(gd.sync.desyncs_detected, 0);
}

/// The other way: what dies in a guest's game dies in the host's, where the death's scripts run
/// once, as for any death there.
#[test]
fn an_actor_a_guest_kills_dies_in_the_hosts_game() {
    let (mut host, mut hd, mut hostnet, mut guest, mut gd, mut gnet) = pair();
    let mut t = 0;
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
    let before = host.scripts_run();
    guest.kill(CHET);
    assert_eq!(host.dead(CHET), Some(false));
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
    assert_eq!(host.dead(CHET), Some(true));
    assert_eq!(
        host.scripts_run(),
        before + 1,
        "the death's script on the host, once"
    );
    assert_eq!(
        (gd.claims_sent, hd.claims_enacted, hd.claims_dropped),
        (1, 1, 0)
    );
    assert_eq!(gd.sync.unacked_intents(), 0);
    assert_eq!(hd.claim_notes.len(), 1);
    // The host's story now holds the death; the guest's game agrees and claims nothing more.
    assert!(echo(&mut guest.e, &gd.sync.store).is_empty());
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
    assert_eq!((gd.claims_sent, hd.claims_enacted), (1, 1));
    // An actor that is not in the host's game now cannot be killed there: said, not enacted.
    host.set_present(SUNNY, false);
    guest.kill(SUNNY);
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
    assert_eq!((hd.claims_enacted, hd.claims_dropped), (1, 1));
    assert!(
        hd.claim_notes[1].contains("not in this game now"),
        "{:?}",
        hd.claim_notes
    );
}

/// PC run 5, 2026-11-10: Trudy was not in her bar on the guest. She and Joe Cobb start disabled
/// and a trigger's script enables them, which happened in the host's game only.
#[test]
fn a_reference_the_hosts_scripts_enable_or_disable_is_the_same_on_the_guest() {
    let (mut host, mut hd, mut hostnet, mut guest, mut gd, mut gnet) = pair();
    let mut t = 0;
    // Both games start with Chet disabled, as the plugin has some references.
    coop_engine::arm(&mut host.e, false);
    host.set_disabled(CHET, true);
    coop_engine::arm(&mut host.e, true);
    guest.set_disabled(CHET, true);
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
    assert_eq!(guest.disabled(CHET), Some(true));
    // Sunny is not in the guest's game now.
    guest.set_present(SUNNY, false);
    host.set_disabled(CHET, false);
    host.set_disabled(SUNNY, true);
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
    assert_eq!(guest.disabled(CHET), Some(false));
    assert_eq!((gd.refused, gd.healed), (0, 0));
    guest.set_present(SUNNY, true);
    // A script of the guest's own game disables him again: the host's state comes back.
    guest.set_disabled(CHET, true);
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
    assert_eq!(guest.disabled(CHET), Some(false));
    assert_eq!(guest.disabled(SUNNY), Some(true));
    assert!(echo(&mut guest.e, &gd.sync.store).is_empty());
    assert_eq!((gd.refused, gd.sync.desyncs_detected), (0, 0));
    assert_eq!(guest.scripts_run(), 0);
}

/// PC run 13, 2026-11-11: the guest could draw no weapon. The trigger outside Doc Mitchell's
/// house gives a player their weapons back and spends itself; the host's player spent it, the
/// story carried that to the guest, and the guest's player walked through a dead trigger.
#[test]
fn a_trigger_a_player_spends_is_spent_for_that_player_only() {
    let (mut host, mut hd, mut hostnet, mut guest, mut gd, mut gnet) = pair();
    let mut t = 0;
    host.make_trigger(CHET);
    guest.make_trigger(CHET);
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
    // The host's player walks into it: the guest's is still there for the guest's player.
    host.enter_trigger(CHET);
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
    assert_eq!(host.disabled(CHET), Some(true));
    assert_eq!(guest.disabled(CHET), Some(false));
    assert_eq!(hd.triggers_kept, 1);
    // The guest's player walks into theirs: it stays spent, and the host's story is not told.
    guest.enter_trigger(CHET);
    frames(
        45,
        &mut t,
        &mut host,
        &mut hd,
        &mut *hostnet,
        &mut guest,
        &mut gd,
        &mut *gnet,
    );
    assert_eq!(guest.disabled(CHET), Some(true));
    assert_eq!(gd.healed, 0);
    assert_eq!((gd.refused, gd.sync.desyncs_detected), (0, 0));
}

/// A trigger the host's story holds as enabled (a quest's script switched it on) and the guest's
/// player then spends is not switched on again by the heal; a later word of the story is taken.
#[test]
fn a_spent_trigger_is_not_put_back_until_the_story_says_something_new() {
    let (mut host, mut hd, mut hostnet, mut guest, mut gd, mut gnet) = pair();
    let mut t = 0;
    host.make_trigger(CHET);
    guest.make_trigger(CHET);
    // Both start with it off; the host's story switches it on (no script of its own).
    coop_engine::arm(&mut host.e, false);
    host.set_disabled(CHET, true);
    coop_engine::arm(&mut host.e, true);
    guest.set_disabled(CHET, true);
    host.set_disabled(CHET, false);
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
    assert_eq!(guest.disabled(CHET), Some(false));
    guest.enter_trigger(CHET);
    frames(
        45,
        &mut t,
        &mut host,
        &mut hd,
        &mut *hostnet,
        &mut guest,
        &mut gd,
        &mut *gnet,
    );
    assert_eq!(guest.disabled(CHET), Some(true), "the heal put it back");
    assert_eq!(gd.healed, 0);
    assert!(
        gd.healed_notes
            .iter()
            .any(|n| n.contains("spent by this player")),
        "{:?}",
        gd.healed_notes
    );
    // The story switches it off and on again: the new word reaches the guest's.
    host.set_disabled(CHET, true);
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
    host.set_disabled(CHET, false);
    frames(
        45,
        &mut t,
        &mut host,
        &mut hd,
        &mut *hostnet,
        &mut guest,
        &mut gd,
        &mut *gnet,
    );
    assert_eq!(guest.disabled(CHET), Some(false));
}

/// Only a trigger volume is each player's own: a thing anyone can see that its own script
/// switches off (an activator that is used up) is gone for everyone, as before.
#[test]
fn a_reference_that_is_no_trigger_and_spends_itself_is_still_shared() {
    let (mut host, mut hd, mut hostnet, mut guest, mut gd, mut gnet) = pair();
    let mut t = 0;
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
    host.enter_trigger(CHET);
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
    assert_eq!(guest.disabled(CHET), Some(true));
    assert_eq!(hd.triggers_kept, 0);
    // And the guest's own, spent by its player, is proposed and gone for everyone (S26).
    host.set_disabled(CHET, false);
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
    guest.enter_trigger(CHET);
    frames(
        45,
        &mut t,
        &mut host,
        &mut hd,
        &mut *hostnet,
        &mut guest,
        &mut gd,
        &mut *gnet,
    );
    assert_eq!(guest.disabled(CHET), Some(true));
    assert_eq!(host.disabled(CHET), Some(true));
}

/// PC run 16, 2026-11-11: the guest shot Sunny's bottles and the count stayed the host's; only
/// the host's shots counted. What a guest's own object script does to the story (D5, rung 2) is
/// proposed to the host, whose game does it for everyone: the stage (with its result script, on
/// the host only), the variable, the reference switched off. The guest's game is not put back
/// meanwhile.
#[test]
fn what_a_guests_own_object_script_does_is_done_by_the_host_for_everyone() {
    let (mut host, mut hd, mut hostnet, mut guest, mut gd, mut gnet) = pair();
    let mut t = 0;
    // The game's data: these object scripts name the main quest.
    for script in [
        coop_engine::testkit::SETTER_SCRIPT_ID,
        coop_engine::testkit::BOTTLE_SCRIPT_ID,
    ] {
        gd.object_script_quests.insert(script, vec![MQ]);
    }
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
    let ran = host.scripts_run();
    let var = |s: &mut Story| coop_engine::vars::quest_vars(&mut s.e, MQ).unwrap()[0].1;
    let before = f64::from_bits(var(&mut host));
    guest.shoot_target(CHET, MQ, 20);
    frames(
        60,
        &mut t,
        &mut host,
        &mut hd,
        &mut *hostnet,
        &mut guest,
        &mut gd,
        &mut *gnet,
    );
    assert_eq!(f64::from_bits(var(&mut host)), before + 1.0);
    assert_eq!(host.disabled(CHET), Some(true));
    assert!(coop_engine::quest::record(&mut host.e, MQ)
        .unwrap()
        .done
        .contains(&20));
    assert!(
        host.scripts_run() > ran,
        "the stage's result script ran on the host"
    );
    assert_same_game(&mut host, &mut guest);
    assert_eq!(gd.healed, 0, "{:?}", gd.healed_notes);
    assert!(hd.proposals_enacted >= 3 && gd.proposals_sent >= 3);
    // The host's player shoots one, then the guest's another: both count.
    host.set_disabled(CHET, false);
    host.shoot_target(SUNNY, MQ, 20);
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
    guest.shoot_target(CHET, MQ, 20);
    frames(
        80,
        &mut t,
        &mut host,
        &mut hd,
        &mut *hostnet,
        &mut guest,
        &mut gd,
        &mut *gnet,
    );
    assert_eq!(f64::from_bits(var(&mut host)), before + 3.0);
    // A bottle that only counts (no stage, no objective): nothing in the journal names a cause
    // for the quest; the script that ran names the quest, which is enough.
    guest.set_disabled(CHET, false);
    host.set_disabled(CHET, false);
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
    gd.forget_causes();
    guest.shoot_target(CHET, MQ, 0);
    frames(
        80,
        &mut t,
        &mut host,
        &mut hd,
        &mut *hostnet,
        &mut guest,
        &mut gd,
        &mut *gnet,
    );
    assert_eq!(f64::from_bits(var(&mut host)), before + 4.0);
    assert_same_game(&mut host, &mut guest);
    assert_eq!((gd.refused, gd.sync.desyncs_detected), (0, 0));
}

/// D5, rungs 1 and 2: a shared quest's stage that something in a guest's game set (a line of
/// dialogue that also hands this player an item runs there: the Varmint Rifle, PC run 17) is
/// proposed and done by the host; a change the host cannot do for everyone (a global) is put
/// back, as before, and logged.
#[test]
fn a_guests_stage_is_done_by_the_host_and_another_drift_is_put_back() {
    let (mut host, mut hd, mut hostnet, mut guest, mut gd, mut gnet) = pair();
    let mut t = 0;
    // The game's data: these object scripts name the main quest.
    for script in [
        coop_engine::testkit::SETTER_SCRIPT_ID,
        coop_engine::testkit::BOTTLE_SCRIPT_ID,
    ] {
        gd.object_script_quests.insert(script, vec![MQ]);
    }
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
    guest.run_object_script_setting_stage(CHET, MQ, 20);
    guest.set_global(0x39, 7.0);
    frames(
        60,
        &mut t,
        &mut host,
        &mut hd,
        &mut *hostnet,
        &mut guest,
        &mut gd,
        &mut *gnet,
    );
    assert!(coop_engine::quest::record(&mut host.e, MQ)
        .unwrap()
        .done
        .contains(&20));
    assert_same_game(&mut host, &mut guest);
    assert_eq!((gd.drift_named, gd.drift_unnamed), (0, 1));
    let notes = gd.healed_notes.join("\n");
    assert!(notes.contains("cause unknown"), "{notes}");
    assert!(notes.contains("proposed to the host"), "{notes}");
}

/// PC run 28, 2026-11-13: a dialogue line's result set a quest variable in the host's game
/// (`bMetSunny`), which leaves nothing in the journal; the story kept the old value, and the
/// guest's game was put back to it. A variable the host's game changes reaches the guests however
/// it was changed.
#[test]
fn a_quest_variable_the_hosts_game_sets_without_the_journal_reaches_the_guest() {
    let (mut host, mut hd, mut hostnet, mut guest, mut gd, mut gnet) = pair();
    let mut t = 0;
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
    let index = coop_engine::vars::quest_vars(&mut host.e, MQ).unwrap()[0].0;
    assert!(coop_engine::vars::set_quest_var(
        &mut host.e,
        MQ,
        index,
        1.0f64.to_bits()
    ));
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
    let var = |s: &mut Story| coop_engine::vars::quest_vars(&mut s.e, MQ).unwrap()[0].1;
    assert_eq!(var(&mut guest), 1.0f64.to_bits());
    assert_same_game(&mut host, &mut guest);
}

/// PC run 19, 2026-11-12: quests of the pre-order packs changed in the guest's game on their own
/// (timers, references), with no cause in the journal, and were proposed every few seconds. A
/// change with no cause is proposed only when an object script that names that quest ran in
/// this game just before; otherwise it is put back as before.
#[test]
fn a_quest_change_no_script_of_this_game_explains_is_put_back() {
    let (mut host, mut hd, mut hostnet, mut guest, mut gd, mut gnet) = pair();
    let mut t = 0;
    // The data says the bottle's script names some other quest, not this one.
    gd.object_script_quests
        .insert(coop_engine::testkit::BOTTLE_SCRIPT_ID, vec![0x0099_0001]);
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
    let var = |s: &mut Story| coop_engine::vars::quest_vars(&mut s.e, MQ).unwrap()[0].1;
    let before = var(&mut host);
    // The quest's variable moves in the guest's game with nothing in the journal, while an
    // object script that names another quest runs.
    guest.shoot_target(CHET, 0x0099_0001, 20);
    let index = coop_engine::vars::quest_vars(&mut guest.e, MQ).unwrap()[0].0;
    coop_engine::vars::set_quest_var(&mut guest.e, MQ, index, 2.5f64.to_bits());
    frames(
        80,
        &mut t,
        &mut host,
        &mut hd,
        &mut *hostnet,
        &mut guest,
        &mut gd,
        &mut *gnet,
    );
    assert_eq!(var(&mut host), before);
    assert_eq!(var(&mut guest), before, "put back");
}

/// PC run 20, 2026-11-12: Sunny's rifle line ran in the guest's game (objective 5 completed,
/// 10 shown) and was put back, so the guest got no instructions. A quest change the journal puts
/// down to a line of this player's dialogue is proposed like an object script's.
#[test]
fn a_quest_change_made_by_the_guests_dialogue_is_done_by_the_host() {
    let (mut host, mut hd, mut hostnet, mut guest, mut gd, mut gnet) = pair();
    let mut t = 0;
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
    // A dialogue result (script id 0, run on the speaker) sets the stage in the guest's game.
    guest.run_dialogue_result_setting_stage(SUNNY, MQ, 20);
    frames(
        60,
        &mut t,
        &mut host,
        &mut hd,
        &mut *hostnet,
        &mut guest,
        &mut gd,
        &mut *gnet,
    );
    assert!(coop_engine::quest::record(&mut host.e, MQ)
        .unwrap()
        .done
        .contains(&20));
    assert_same_game(&mut host, &mut guest);
}

/// PC run 25, 2026-11-12: the third well's geckos were counted in both games (their death script
/// in the guest's game, the death of each copy in the host's), the count passed 3 without being
/// 3, and the well was never cleared. What a script does to a quest after the actor it ran on
/// has died is that death's, and the host runs it.
#[test]
fn a_quest_change_from_an_actors_death_is_the_hosts() {
    let (mut host, mut hd, mut hostnet, mut guest, mut gd, mut gnet) = pair();
    let mut t = 0;
    gd.object_script_quests
        .insert(coop_engine::testkit::BOTTLE_SCRIPT_ID, vec![MQ]);
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
    let var = |s: &mut Story| coop_engine::vars::quest_vars(&mut s.e, MQ).unwrap()[0].1;
    let before = f64::from_bits(var(&mut host));
    // Its script runs while it lives (shot): that counts, as the bottles do.
    guest.shoot_target(CHET, MQ, 0);
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
    assert_eq!(f64::from_bits(var(&mut host)), before + 1.0);
    // It dies in the guest's game and its death script counts it there: not proposed.
    guest.kill(CHET);
    let index = coop_engine::vars::quest_vars(&mut guest.e, MQ).unwrap()[0].0;
    let now = f64::from_bits(var(&mut guest));
    coop_engine::vars::set_quest_var(&mut guest.e, MQ, index, (now + 1.0).to_bits());
    frames(
        80,
        &mut t,
        &mut host,
        &mut hd,
        &mut *hostnet,
        &mut guest,
        &mut gd,
        &mut *gnet,
    );
    assert_eq!(f64::from_bits(var(&mut host)), before + 1.0);
    assert_eq!(host.dead(CHET), Some(true), "the death reached the host");
    assert_same_game(&mut host, &mut guest);
}

/// PC run 25, 2026-11-12: the guest's player died, its game loaded the last save, and the guest
/// proposed the whole of that save to the host. After a load nothing is proposed for a while:
/// the guest's game is put back to the story.
#[test]
fn a_guest_that_loads_a_save_proposes_nothing() {
    let (mut host, mut hd, mut hostnet, mut guest, mut gd, mut gnet) = pair();
    let mut t = 0;
    for script in [
        coop_engine::testkit::SETTER_SCRIPT_ID,
        coop_engine::testkit::BOTTLE_SCRIPT_ID,
    ] {
        gd.object_script_quests.insert(script, vec![MQ]);
    }
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
    let var = |s: &mut Story| coop_engine::vars::quest_vars(&mut s.e, MQ).unwrap()[0].1;
    let before = var(&mut host);
    // The load: the game reads a save, and its own scripts then run as after any load.
    guest.load("autosave");
    guest.shoot_target(CHET, MQ, 20);
    frames(
        80,
        &mut t,
        &mut host,
        &mut hd,
        &mut *hostnet,
        &mut guest,
        &mut gd,
        &mut *gnet,
    );
    assert_eq!(var(&mut host), before);
    assert!(!coop_engine::quest::record(&mut host.e, MQ)
        .unwrap()
        .done
        .contains(&20));
    assert_eq!(hd.proposals_enacted, 0);
    assert_same_game(&mut host, &mut guest);
}

/// Harness, 2026-11-13: the host's story was read once, at the main menu; after the host loaded
/// its save, Sunny's quest was at stage 5 in its game and 0 in the story, and the guests were put
/// back to 0. After a load the story is read again from the host's game.
#[test]
fn the_story_is_read_again_after_the_host_loads_a_save() {
    let (mut host, mut hd, mut hostnet, mut guest, mut gd, mut gnet) = pair();
    let mut t = 0;
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
    // The save's state comes into the host's game with nothing in the journal.
    let mut rec = coop_engine::quest::record(&mut host.e, MQ).unwrap();
    assert!(!rec.done.contains(&20));
    rec.stage = 20;
    rec.done.push(20);
    assert!(coop_engine::gate::applying(&mut host.e, |e| {
        let ok = coop_engine::quest::apply(e, &rec);
        let _ = coop_engine::drain(e);
        ok
    }));
    host.load("Save 2");
    frames(
        60,
        &mut t,
        &mut host,
        &mut hd,
        &mut *hostnet,
        &mut guest,
        &mut gd,
        &mut *gnet,
    );
    assert!(coop_engine::quest::record(&mut guest.e, MQ)
        .unwrap()
        .done
        .contains(&20));
    assert_same_game(&mut host, &mut guest);
}

/// A guest whose game is merely behind the story (it joins with a game that already did things
/// of its own) proposes nothing: the story is put into its game, not its game into the story.
#[test]
fn a_guest_that_joins_with_a_game_of_its_own_proposes_nothing() {
    let (mut host, mut hd, mut hostnet, mut guest, mut gd, mut gnet) = pair();
    let mut t = 0;
    guest.run_object_script_setting_stage(CHET, MQ, 20);
    guest.shoot_target(SUNNY, MQ, 20);
    frames(
        80,
        &mut t,
        &mut host,
        &mut hd,
        &mut *hostnet,
        &mut guest,
        &mut gd,
        &mut *gnet,
    );
    assert!(!coop_engine::quest::record(&mut host.e, MQ)
        .unwrap()
        .done
        .contains(&20));
    assert_eq!((hd.proposals_enacted, gd.proposals_sent), (0, 0));
    assert_same_game(&mut host, &mut guest);
}

/// PC run 6, 2026-11-10: after her dialogue Sunny stood still. The stage's result script tells
/// her `evp` so she acts on the new stage at once; it runs on the host only. The host sends each
/// evaluation a script asks for, and the guest's copy is told too. The AI's own evaluations are
/// not sent.
#[test]
fn an_actor_a_hosts_script_tells_to_evaluate_its_package_is_told_on_the_guest() {
    let (mut host, mut hd, mut hostnet, mut guest, mut gd, mut gnet) = pair();
    let mut t = 0;
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
    host.ai_evp(CHET);
    host.ai_evp(CHET);
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
    assert_eq!(guest.evaluations(CHET), 0);
    host.set_stage(MQ, 10);
    host.script_evp(CHET);
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
    assert_eq!((guest.evaluations(CHET), guest.evaluations(SUNNY)), (1, 0));
    // A second command is a second evaluation; the heal and the checks do not repeat one.
    host.script_evp(CHET);
    frames(
        60,
        &mut t,
        &mut host,
        &mut hd,
        &mut *hostnet,
        &mut guest,
        &mut gd,
        &mut *gnet,
    );
    assert_eq!(guest.evaluations(CHET), 2);
    assert_eq!((gd.refused, gd.healed, gd.sync.desyncs_detected), (0, 0, 0));
    // The guest's own scripts' evaluations stay in its game.
    guest.script_evp(SUNNY);
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
    assert_eq!(host.evaluations(SUNNY), 0);
    // The host's log names the stage change.
    assert!(
        hd.stage_notes
            .iter()
            .any(|l| l.contains(&format!("quest {MQ:08x}: stage 10"))),
        "{:?}",
        hd.stage_notes
    );
}

/// S30 (harness, 2026-11-13): Trudy's line that ends Sunny's last tutorial gives `RewardXP 50`;
/// said to the guest, it ran in the host's game, and the host's player alone got the 50. The
/// story's experience on the host goes to each guest's player once; the guest's own run of a
/// story script stays held back by the gate.
#[test]
fn the_storys_experience_goes_to_every_player() {
    let net = LoopbackNet::new();
    let mut hostnet = net.endpoint(PlayerId::HOST);
    let mut gnet = net.endpoint(PlayerId(1));
    let mut host = Story::new(&spec());
    let mut hd = HostDriver::new(HostConfig::new(hello(), 1), quest_ids(), GLOBALS);
    hd.start(&mut host.e);
    let mut guest = guest_story();
    let mut gd = GuestDriver::new(PlayerId(1), hello());
    let mut t = 0;
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
    host.run_script_rewarding_xp(MQ, 50);
    guest.run_script_rewarding_xp(MQ, 50);
    frames(
        10,
        &mut t,
        &mut host,
        &mut hd,
        &mut hostnet,
        &mut guest,
        &mut gd,
        &mut gnet,
    );
    assert_eq!((host.xp(), guest.xp()), (50, 50));
    assert_eq!((hd.rewards, gd.xp_given), (1, 50));
    // Once: nothing more arrives later.
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
    assert_eq!(guest.xp(), 50);
}

/// S30 (the user, 2026-11-13: both players get the caps and items a quest gives): caps and items
/// the story gives the host's player go to each guest's player once, as `AddItem` gives them.
#[test]
fn the_storys_caps_and_items_go_to_every_player() {
    const CAPS: u32 = 0x0000_000f;
    const STIMPAK: u32 = 0x0001_5169;
    let net = LoopbackNet::new();
    let mut hostnet = net.endpoint(PlayerId::HOST);
    let mut gnet = net.endpoint(PlayerId(1));
    let mut host = Story::new(&spec());
    let mut hd = HostDriver::new(HostConfig::new(hello(), 1), quest_ids(), GLOBALS);
    hd.start(&mut host.e);
    let mut guest = guest_story();
    let mut gd = GuestDriver::new(PlayerId(1), hello());
    for s in [&mut host, &mut guest] {
        s.add_item_form(CAPS);
        s.add_item_form(STIMPAK);
    }
    let mut t = 0;
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
    host.run_script_giving_item(MQ, CAPS, 100);
    host.run_script_giving_item(MQ, STIMPAK, 2);
    guest.run_script_giving_item(MQ, CAPS, 100);
    frames(
        10,
        &mut t,
        &mut host,
        &mut hd,
        &mut hostnet,
        &mut guest,
        &mut gd,
        &mut gnet,
    );
    assert_eq!((host.items(CAPS), guest.items(CAPS)), (100, 100));
    assert_eq!((host.items(STIMPAK), guest.items(STIMPAK)), (2, 2));
    assert_eq!((hd.rewards, gd.items_given), (2, 2));
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
    assert_eq!(guest.items(CAPS), 100);
}

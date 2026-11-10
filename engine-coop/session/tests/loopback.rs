//! The host and guest drivers over coop-core's loopback and chaos transports, each driving a
//! fake game program through the adapter. No engine and no game files: what these tests show
//! holds for the fake program, and a run on two real games is still needed for each.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use coop_core::ids::{FormId, PlayerId};
use coop_core::schema::Field;
use coop_core::sync::HostConfig;
use coop_core::transport::{
    ChaosProfile, ChaosTransport, LoopbackEndpoint, LoopbackNet, Transport, VirtualClock,
};
use coop_engine::fake::FakeProgram;
use coop_engine::game::{resolve_policy, EngineCoop, EngineState, NameTable};
use coop_engine::{addr, life, quest, Program};
use coop_engine_session::record::QuestRecord;
use coop_engine_session::{hello, GuestDriver, HostDriver, CLOCK_TAG};
use coop_game::Shared;
use coop_policy::PolicyTable;

/// The main quest of these tests: four stages, three objectives, two script variables.
const MQ: u32 = 0x0010_4c0c;
/// A side quest with many stages, for a burst of changes in one frame.
const SIDE: u32 = 0x0015_d912;
/// `VCG01`: a quest that each player runs itself (a per-player row of the policy).
const OWN: u32 = 0x0010_4c1c;
/// A quest with 108 objectives, as a quest of an add-on has.
const WIDE: u32 = 0x0010_f3f7;
/// A story global.
const STORY: u32 = 0x0010_0001;
/// The clock: shared as the clock, not as a story global.
const HOUR: u32 = 0x38;
const DAYS: u32 = 0x39;
/// Two placed actors.
const SUNNY: u32 = 0x0010_4e84;
const GECKO: u32 = 0x0010_9f3a;
const HOUSE: u32 = 0x0010_3df9;

const SIDE_STAGES: [u8; 30] = [
    5, 10, 15, 20, 25, 30, 35, 40, 45, 50, 55, 60, 65, 70, 75, 80, 85, 90, 95, 100, 105, 110, 115,
    120, 125, 130, 135, 140, 145, 150,
];

fn names() -> NameTable {
    NameTable::parse(
        "P 00 FalloutNV.esm\n\
         F QUST 00104C0C MQ\nF QUST 0015D912 VCG03\nF QUST 00104C1C VCG01\n\
         F QUST 0010F3F7 Wide\n\
         F GLOB 00100001 StoryGlobal\nF GLOB 00000038 GameHour\nF GLOB 00000039 GameDaysPassed\n",
    )
    .unwrap()
}

fn policy() -> Option<Arc<PolicyTable>> {
    let bundled = PolicyTable::bundled().unwrap();
    Some(Arc::new(resolve_policy(&bundled, &names()).unwrap()))
}

/// One machine's game: the same data on each machine.
fn game() -> FakeProgram {
    let mut p = FakeProgram::new();
    let mq = p.add_quest(MQ, &[5, 10, 20, 30], &[10, 20, 30]);
    p.add_variables(mq, &[1, 2]);
    p.add_quest(SIDE, &SIDE_STAGES, &[]);
    p.add_quest(OWN, &[5, 10, 105], &[]);
    let wide: Vec<u32> = (0..108).map(|n| n * 5).collect();
    p.add_quest(WIDE, &[10], &wide);
    p.add_global(STORY, 0.0);
    p.add_global(HOUR, 8.0);
    p.add_global(DAYS, 0.0);
    p.add_change_map(64);
    p.add_actor(SUNNY, addr::FORM_TYPE_NPC_REF);
    p.add_actor(GECKO, addr::FORM_TYPE_CREATURE_REF);
    let player = p.add_player([0.0; 3]);
    let house = p.add_cell(HOUSE, addr::cell_state::ATTACHED);
    p.set_u32(player + addr::REF_CELL, house);
    p
}

fn plugins() -> Vec<String> {
    names().plugin_files()
}

fn host_driver(delta_cap: usize) -> HostDriver {
    let mut cfg = HostConfig::new(hello(plugins()), 0x5e55);
    cfg.delta_cap = delta_cap;
    cfg.max_guests = 3;
    HostDriver::new(cfg, EngineState::new(names()), policy())
}

fn guest_driver(slot: u8) -> GuestDriver {
    GuestDriver::new(
        PlayerId(slot),
        hello(plugins()),
        EngineState::new(names()),
        policy(),
    )
}

fn at(p: &mut FakeProgram, id: u32) -> u32 {
    p.call(addr::FORM_BY_ID, &[id]).eax
}

fn set_stage(p: &mut FakeProgram, id: u32, stage: u8) {
    let q = at(p, id);
    p.call(addr::QUEST_SET_STAGE, &[q, u32::from(stage)]);
}

fn show_objective(p: &mut FakeProgram, id: u32, n: usize, state: u32) {
    let q = at(p, id);
    let item = quest::objective_items(p, q)[n].0;
    p.call(addr::OBJECTIVE_SET_STATE, &[item, state]);
}

fn set_var(p: &mut FakeProgram, id: u32, n: usize, v: f64) {
    let q = at(p, id);
    let item = quest::variable_items(p, q)[n].0;
    p.write(item + addr::VAR_VALUE, &v.to_le_bytes());
}

fn set_global(p: &mut FakeProgram, id: u32, v: f32) {
    let g = at(p, id);
    p.set_f32(g + addr::GLOBAL_VALUE, v);
}

fn global(p: &mut FakeProgram, id: u32) -> f32 {
    let g = at(p, id);
    p.f32(g + addr::GLOBAL_VALUE)
}

/// The shared state of a game as its adapter exports it, the clock left out.
fn story(p: &mut FakeProgram, state: &mut EngineState) -> Vec<(FormId, Field)> {
    let mut coop = EngineCoop::over(p, state);
    coop.export_shared(true)
        .unwrap()
        .into_iter()
        .filter(|(f, _)| f.0 != HOUR && f.0 != DAYS)
        .collect()
}

/// The record that a quest of this game has now.
fn record_of(p: &mut FakeProgram, state: &mut EngineState, id: u32) -> QuestRecord {
    let coop = EngineCoop::over(p, state);
    QuestRecord::from_fields(&coop.fields_of(FormId(id)).unwrap_or_default())
}

/// One step of the host's story.
#[derive(Clone, Copy)]
enum Act {
    Stage(u32, u8),
    Objective(u32, usize, u32),
    Var(u32, usize, f64),
    Global(u32, f32),
    Kill(u32),
}

fn act(p: &mut FakeProgram, a: Act) {
    match a {
        Act::Stage(q, s) => set_stage(p, q, s),
        Act::Objective(q, n, s) => show_objective(p, q, n, s),
        Act::Var(q, n, v) => set_var(p, q, n, v),
        Act::Global(g, v) => set_global(p, g, v),
        Act::Kill(id) => {
            life::kill(p, id);
        }
    }
}

/// The host's story, frame by frame: stages, a burst of thirty stages in one frame (more keys
/// than one delta carries), objectives, variables, a global that changes and changes back, the
/// player's own quest, and a death.
fn opening() -> Vec<Vec<Act>> {
    let mut frames = vec![
        vec![Act::Stage(MQ, 5), Act::Objective(MQ, 0, 1)],
        vec![Act::Var(MQ, 0, 1.0), Act::Global(STORY, 1.0)],
        SIDE_STAGES.iter().map(|s| Act::Stage(SIDE, *s)).collect(),
        vec![
            Act::Stage(MQ, 10),
            Act::Objective(MQ, 0, 3),
            Act::Objective(MQ, 1, 1),
        ],
        vec![Act::Global(STORY, 0.0), Act::Var(MQ, 1, 2.5)],
        vec![Act::Stage(OWN, 5), Act::Stage(OWN, 10)],
        vec![Act::Kill(GECKO)],
        vec![
            Act::Stage(MQ, 20),
            Act::Objective(MQ, 2, 1),
            Act::Var(MQ, 0, 0.0),
        ],
        (0..108)
            .map(|n| Act::Objective(WIDE, n, 1 + (n % 3) as u32))
            .collect(),
        vec![Act::Stage(MQ, 30), Act::Global(STORY, 2.0)],
    ];
    // Empty frames between the acts, so that the guest catches up between bursts.
    let mut out = Vec::new();
    for f in frames.drain(..) {
        out.push(f);
        out.extend(std::iter::repeat_with(Vec::new).take(3));
    }
    out
}

/// One host and its guests, each with its own game and transport.
struct Run<T: Transport> {
    hp: FakeProgram,
    host: HostDriver,
    hnet: T,
    guests: Vec<(FakeProgram, GuestDriver, T)>,
    now: u64,
}

impl<T: Transport> Run<T> {
    /// One frame on each machine.
    fn frame(&mut self) {
        self.now += 16;
        self.host
            .step(&mut self.hp, &mut self.hnet, self.now)
            .unwrap();
        for (p, g, net) in &mut self.guests {
            g.step(p, net, self.now).unwrap();
        }
    }

    fn frames(&mut self, n: usize) {
        for _ in 0..n {
            self.frame();
        }
    }
}

fn loopback(guests: u8, delta_cap: usize) -> Run<LoopbackEndpoint> {
    let net = LoopbackNet::new();
    Run {
        hp: game(),
        host: host_driver(delta_cap),
        hnet: net.endpoint(PlayerId::HOST),
        guests: (1..=guests)
            .map(|i| (game(), guest_driver(i), net.endpoint(PlayerId(i))))
            .collect(),
        now: 0,
    }
}

/// The guest's game equals the host's: every shared quest, variable and global.
fn assert_same_story<T: Transport>(run: &mut Run<T>) {
    let want = story(&mut run.hp, &mut run.host.game);
    for (p, g, _) in &mut run.guests {
        assert_eq!(story(p, &mut g.game), want, "guest {}", g.sync.player.0);
    }
}

/// Plays the opening; after each frame each guest's game holds, for each quest, a record that
/// the host's game really had. Guest `late` (if any) joins after `late_at` frames.
fn play_opening<T: Transport>(
    run: &mut Run<T>,
    late: Option<(FakeProgram, GuestDriver, T)>,
    late_at: usize,
) {
    let mut late = late;
    let quests = [MQ, SIDE, WIDE];
    let mut history: BTreeMap<u32, BTreeSet<Vec<u8>>> = BTreeMap::new();
    let note = |run: &mut Run<T>, history: &mut BTreeMap<u32, BTreeSet<Vec<u8>>>| {
        for q in quests {
            let r = record_of(&mut run.hp, &mut run.host.game, q);
            history.entry(q).or_default().insert(encoded(&r));
        }
    };
    note(run, &mut history);
    for (i, acts) in opening().into_iter().enumerate() {
        if i == late_at {
            if let Some(g) = late.take() {
                run.guests.push(g);
            }
        }
        for a in acts {
            act(&mut run.hp, a);
        }
        note(run, &mut history);
        run.frame();
        for (p, g, _) in &mut run.guests {
            for q in quests {
                let r = record_of(p, &mut g.game, q);
                assert!(
                    history[&q].contains(&encoded(&r)),
                    "frame {i}: guest {} holds a record of {q:08x} that the host never had: {r:?}",
                    g.sync.player.0
                );
            }
        }
    }
    run.frames(60);
    assert_same_story(run);
    for (p, g, _) in &run.guests {
        assert!(p.stages_set.is_empty(), "no stage function ran on a guest");
        assert_eq!(g.refused, 0);
        assert_eq!(g.sync.desyncs_detected, 0);
        assert_eq!(p.gates.len(), 1, "the guest's game got its gate");
    }
    assert!(run.hp.gates.is_empty(), "a host's game has no gate");
}

fn encoded(r: &QuestRecord) -> Vec<u8> {
    match r.encode() {
        Some(Field::Custom { data, .. }) => data,
        _ => Vec::new(),
    }
}

/// With one key in each delta, a quest still crosses whole: its record is one key.
#[test]
fn guests_follow_the_story_one_key_per_delta() {
    let mut run = loopback(2, 1);
    play_opening(&mut run, None, 0);
}

#[test]
fn guests_follow_the_story_with_the_default_delta_size() {
    let mut run = loopback(1, 24);
    play_opening(&mut run, None, 0);
}

/// A guest that joins in the middle of the story gets it from its join snapshot. Mutation
/// twin: before its first step it holds none of the host's state.
#[test]
fn a_guest_that_joins_late_starts_from_the_snapshot() {
    let net = LoopbackNet::new();
    let mut run = Run {
        hp: game(),
        host: host_driver(24),
        hnet: net.endpoint(PlayerId::HOST),
        guests: vec![(game(), guest_driver(1), net.endpoint(PlayerId(1)))],
        now: 0,
    };
    let (mut lp, mut lg, ln) = (game(), guest_driver(2), net.endpoint(PlayerId(2)));
    assert!(
        story(&mut lp, &mut lg.game).is_empty(),
        "the twin: nothing yet"
    );
    play_opening(&mut run, Some((lp, lg, ln)), 20);
    assert!(run.guests[1].1.sync.snapshots_loaded >= 1);
}

/// Over a bad network (loss, duplicates, reordering, latency) the guests still end with the
/// host's story, and no record is ever mixed.
#[test]
fn guests_follow_over_a_bad_network() {
    let clock = VirtualClock::new();
    let net = LoopbackNet::new();
    let profile = ChaosProfile::by_name("bad_wifi").unwrap();
    let chaos = |id: u8, seed: u64| {
        ChaosTransport::new(net.endpoint(PlayerId(id)), clock.clone(), profile, seed)
    };
    let mut run = Run {
        hp: game(),
        host: host_driver(4),
        hnet: chaos(0, 7),
        guests: vec![(game(), guest_driver(1), chaos(1, 8))],
        now: 0,
    };
    // The chaos transports read the shared clock: it moves with each frame.
    let mut frames = 0;
    for acts in opening() {
        for a in acts {
            act(&mut run.hp, a);
        }
        run.frame();
        clock.advance(16);
        frames += 1;
    }
    for _ in 0..3000 {
        run.frame();
        clock.advance(16);
        frames += 1;
        let want = story(&mut run.hp, &mut run.host.game);
        let (p, g, _) = &mut run.guests[0];
        if story(p, &mut g.game) == want {
            break;
        }
    }
    assert!(
        frames < 3000 + opening().len(),
        "the guest did not catch up"
    );
    assert_same_story(&mut run);
}

/// A guest's game that drifts from the host's story (a write that the gate did not stop) is
/// put back by the heal. Mutation twin: with no heal the drift stays.
#[test]
fn a_guest_game_that_drifts_is_put_back_on_the_host_state() {
    for heal in [true, false] {
        let mut run = loopback(1, 24);
        if !heal {
            run.guests[0].1.heal_every = u64::MAX;
        }
        set_stage(&mut run.hp, MQ, 10);
        set_global(&mut run.hp, STORY, 3.0);
        run.frames(30);
        assert_same_story(&mut run);
        let (p, _, _) = &mut run.guests[0];
        set_stage(p, SIDE, 50);
        set_global(p, STORY, 9.0);
        run.frames(40);
        let want = story(&mut run.hp, &mut run.host.game);
        let (p, g, _) = &mut run.guests[0];
        if heal {
            assert_eq!(story(p, &mut g.game), want);
            assert!(g.healed >= 2, "{}", g.healed);
            assert!(!g.healed_notes.is_empty());
        } else {
            assert_ne!(story(p, &mut g.game), want, "the twin: no heal");
        }
    }
}

/// A guest that lost the host and joined again resumes and catches up with what it missed.
#[test]
fn a_guest_that_lost_the_host_rejoins_and_catches_up() {
    let net = LoopbackNet::new();
    let mut run = Run {
        hp: game(),
        host: host_driver(24),
        hnet: net.endpoint(PlayerId::HOST),
        guests: vec![(game(), guest_driver(1), net.endpoint(PlayerId(1)))],
        now: 0,
    };
    set_stage(&mut run.hp, MQ, 5);
    run.frames(30);
    assert_same_story(&mut run);
    // The link drops: the host forgets the guest, and the guest's messages are lost.
    run.host.sync.disconnect(PlayerId(1));
    net.flush(PlayerId::HOST);
    set_stage(&mut run.hp, MQ, 10);
    show_objective(&mut run.hp, MQ, 0, 1);
    for _ in 0..10 {
        run.now += 16;
        run.host.step(&mut run.hp, &mut run.hnet, run.now).unwrap();
        net.flush(PlayerId(1));
    }
    let (p, g, n) = &mut run.guests[0];
    assert_eq!(
        record_of(p, &mut g.game, MQ).stage,
        Some(5),
        "missed while away"
    );
    g.rejoin(PlayerId(1), n);
    run.frames(60);
    assert_same_story(&mut run);
}

/// A quest that each player runs itself (the opening's `VCG01`) is not shared: the host's
/// stages stay on the host, and the guest's own stages stay on the guest.
#[test]
fn a_quest_of_the_players_own_is_not_shared() {
    let mut run = loopback(1, 24);
    set_stage(&mut run.hp, OWN, 10);
    let (p, _, _) = &mut run.guests[0];
    set_stage(p, OWN, 5);
    run.frames(60);
    let host_quest = at(&mut run.hp, OWN);
    assert_eq!(run.hp.u8(host_quest + addr::QUEST_STAGE), 10);
    let (p, g, _) = &mut run.guests[0];
    let q = at(p, OWN);
    assert_eq!(
        p.u8(q + addr::QUEST_STAGE),
        5,
        "the guest's own stage stays"
    );
    assert!(g.sync.store.iter().all(|(k, _)| k.form.0 != OWN));
    // Not in the adapter's catalog either.
    assert!(EngineCoop::over(p, &mut g.game)
        .fields_of(FormId(OWN))
        .is_none());
}

/// The host's objectives are in the guest's Pip-Boy: the game's own setter put them in the
/// player's objective list. Mutation twin: an objective whose status the guest's game lost
/// from the list is put back by the heal's relist.
#[test]
fn the_hosts_objectives_are_in_the_guests_pip_boy() {
    let mut run = loopback(1, 24);
    show_objective(&mut run.hp, MQ, 0, 1);
    show_objective(&mut run.hp, MQ, 1, 3);
    run.frames(30);
    let (p, g, _) = &mut run.guests[0];
    let q = at(p, MQ);
    let items: Vec<u32> = quest::objective_items(p, q)
        .into_iter()
        .map(|(i, _)| i)
        .collect();
    assert_eq!(quest::player_objectives(p), items[..2]);
    // The guest's game loses its list (a load empties it): the relist puts the two back.
    let player = p.u32(addr::PLAYER);
    p.set_u32(player + addr::PLAYER_OBJECTIVES, 0);
    p.set_u32(player + addr::PLAYER_OBJECTIVES + 4, 0);
    assert!(quest::player_objectives(p).is_empty(), "the twin: lost");
    let before = g.relisted;
    run.frames(30);
    let (p, g, _) = &mut run.guests[0];
    assert_eq!(g.relisted - before, 2);
    assert_eq!(quest::player_objectives(p).len(), 2);
}

/// The clock is the host's: about once a second the host writes it, and the guest sets its
/// own clock to it, also when the guest's clock ran on alone. It is not a story global, so
/// the heal leaves it. Mutation twin: before the first second the guest has its own hour.
#[test]
fn the_guests_clock_is_kept_on_the_hosts() {
    let mut run = loopback(1, 24);
    set_global(&mut run.hp, HOUR, 20.0);
    set_global(&mut run.hp, DAYS, 3.0);
    let (p, _, _) = &mut run.guests[0];
    assert_eq!(global(p, HOUR), 8.0, "the twin: the guest's own hour");
    run.frames(80);
    let (p, g, _) = &mut run.guests[0];
    assert_eq!(global(p, HOUR), 20.0);
    assert_eq!(global(p, DAYS), 3.0);
    assert!(g.sync.store.iter().any(|(k, en)| k.form.0 == HOUR
        && matches!(en.field, Field::Custom { tag, .. } if tag == CLOCK_TAG)));
    assert!(
        g.sync
            .store
            .iter()
            .all(|(k, en)| k.form.0 != HOUR || !matches!(en.field, Field::Global { .. })),
        "the hour is no story global"
    );
    // The guest's clock runs on alone; within a second it is the host's again, while the
    // host's clock moves on.
    set_global(p, HOUR, 2.0);
    set_global(&mut run.hp, HOUR, 20.5);
    run.frames(80);
    let (p, g, _) = &mut run.guests[0];
    assert_eq!(global(p, HOUR), 20.5);
    assert!(g.healed_notes.iter().all(|n| !n.contains("00000038")));
}

/// A death in the host's game is the guest's too, by the game's own kill inside the apply
/// bracket (the death's scripts do not run a second time there). Mutation twin: before the
/// step the guest's actor lives.
#[test]
fn an_actor_the_host_kills_dies_in_the_guests_game() {
    let mut run = loopback(1, 24);
    run.frames(10);
    life::kill(&mut run.hp, GECKO);
    let (p, _, _) = &mut run.guests[0];
    assert_eq!(life::dead(p, GECKO), Some(false), "the twin: not yet");
    run.frames(20);
    let (p, g, _) = &mut run.guests[0];
    assert_eq!(life::dead(p, GECKO), Some(true));
    assert_eq!(life::dead(p, SUNNY), Some(false));
    assert!(p.applied_calls.contains(&addr::ACTOR_KILL));
    assert_eq!(
        g.claims_sent, 0,
        "a death from the host is not claimed back"
    );
}

/// The other way: what dies in a guest's game dies in the host's, where the death's scripts
/// run once, and from there it is every guest's. Mutation twin: a dynamic actor (made while
/// playing) is not claimed.
#[test]
fn an_actor_a_guest_kills_dies_in_the_hosts_game() {
    let mut run = loopback(2, 24);
    let dynamic = 0xff00_0900;
    for (p, _, _) in &mut run.guests {
        p.add_actor(dynamic, addr::FORM_TYPE_NPC_REF);
    }
    run.hp.add_actor(dynamic, addr::FORM_TYPE_NPC_REF);
    run.frames(10);
    let (p, _, _) = &mut run.guests[0];
    life::kill(p, SUNNY);
    life::kill(p, dynamic);
    run.frames(40);
    assert_eq!(life::dead(&mut run.hp, SUNNY), Some(true));
    assert_eq!(run.host.claims_enacted, 1);
    assert_eq!(life::dead(&mut run.hp, dynamic), Some(false), "the twin");
    assert_eq!(run.guests[0].1.claims_sent, 1);
    let (p2, g2, _) = &mut run.guests[1];
    assert_eq!(life::dead(p2, SUNNY), Some(true), "the other guest too");
    assert_eq!(g2.claims_sent, 0);
    assert!(run.host.claim_notes[0].contains("player 1"));
}

/// A quest with a record that cannot be carried is named once in the host's log, and the
/// rest of the story goes on. Mutation twin: the quest of 108 objectives does fit.
#[test]
fn a_record_that_cannot_be_sent_is_named_once() {
    const HUGE: u32 = 0x0010_aaaa;
    let huge_names = || {
        let mut text = String::from(
            "P 00 FalloutNV.esm\nF QUST 00104C0C MQ\nF QUST 0010AAAA Huge\nF QUST 0010F3F7 Wide\n",
        );
        text.push_str("F GLOB 00000038 GameHour\n");
        NameTable::parse(&text).unwrap()
    };
    let build = || {
        let mut p = game();
        let objectives: Vec<u32> = (0..300).collect();
        p.add_quest(HUGE, &[10], &objectives);
        p
    };
    let net = LoopbackNet::new();
    let cfg = HostConfig::new(hello(plugins()), 9);
    let mut host = HostDriver::new(cfg, EngineState::new(huge_names()), None);
    let mut guest = GuestDriver::new(
        PlayerId(1),
        hello(plugins()),
        EngineState::new(huge_names()),
        None,
    );
    let (mut hp, mut gp) = (build(), build());
    let (mut hn, mut gn) = (net.endpoint(PlayerId::HOST), net.endpoint(PlayerId(1)));
    for n in 0..300 {
        show_objective(&mut hp, HUGE, n, 1);
    }
    for n in 0..108 {
        show_objective(&mut hp, WIDE, n, 1);
    }
    set_stage(&mut hp, MQ, 10);
    for t in 0..60 {
        host.step(&mut hp, &mut hn, t * 16).unwrap();
        guest.step(&mut gp, &mut gn, t * 16).unwrap();
    }
    let named: Vec<&String> = host
        .unencodable_notes
        .iter()
        .filter(|n| n.contains("0010aaaa"))
        .collect();
    assert_eq!(named.len(), 1, "{:?}", host.unencodable_notes);
    assert!(host.unencodable >= 60, "tried each export");
    assert_eq!(record_of(&mut gp, &mut guest.game, MQ).stage, Some(10));
    let wide = record_of(&mut gp, &mut guest.game, WIDE);
    assert_eq!(wide.objectives.len(), 108, "the twin: 108 objectives fit");
    assert!(record_of(&mut gp, &mut guest.game, HUGE)
        .objectives
        .is_empty());
}

/// A guest whose game cannot take its gate does not go on: its step answers the reason and
/// says no hello.
#[test]
fn a_guest_with_no_gate_does_not_join() {
    struct NoGate(FakeProgram);
    impl Program for NoGate {
        fn read(&self, a: u32, out: &mut [u8]) {
            self.0.read(a, out)
        }
        fn write(&mut self, a: u32, b: &[u8]) {
            self.0.write(a, b)
        }
        fn call(&mut self, a: u32, args: &[u32]) -> coop_engine::Ret {
            self.0.call(a, args)
        }
        fn alloc(&mut self, len: u32) -> u32 {
            self.0.alloc(len)
        }
        fn free(&mut self, a: u32) {
            self.0.free(a)
        }
    }
    let net = LoopbackNet::new();
    let mut host_net = net.endpoint(PlayerId::HOST);
    let mut p = NoGate(game());
    let mut g = guest_driver(1);
    let mut n = net.endpoint(PlayerId(1));
    assert!(g.step(&mut p, &mut n, 0).is_err());
    assert!(g.step(&mut p, &mut n, 16).is_err(), "and stays out");
    assert!(host_net.recv().is_none(), "no hello");
}

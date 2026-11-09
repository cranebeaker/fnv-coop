use proptest::prelude::*;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};

use super::*;
use crate::delta::StateStore;
use crate::ids::{Epoch, Rev, Tick};
use crate::proto::RejectReason;
use crate::transport::PROFILE_NAMES;

fn profile(name: &str) -> ChaosProfile {
    ChaosProfile::by_name(name).unwrap()
}

fn converge_scenario(name: &str, seed: u64) {
    let mut sim = Sim::new(2, profile(name), seed);
    sim.start_guest(0);
    sim.start_guest(1);
    sim.run(400, 2); // 10 s of constant writes
    assert!(
        sim.run_until_converged(180_000),
        "profile {name} seed {seed} did not converge"
    );
    for g in &sim.guests {
        assert_eq!(g.sync.store.entries(), sim.host.store.entries());
        assert_eq!(g.sync.desyncs_detected, 0, "profile {name}");
    }
    assert_eq!(sim.decode_errors, 0);
    assert!(sim.host.store.len() > 50);
}

#[test]
fn converges_under_perfect_link() {
    converge_scenario("perfect", 1);
}

#[test]
fn converges_good_wifi() {
    converge_scenario("good_wifi", 2);
}

#[test]
fn converges_bad_wifi() {
    converge_scenario("bad_wifi", 3);
}

#[test]
fn converges_mobile() {
    converge_scenario("mobile", 4);
}

#[test]
fn converges_congested() {
    converge_scenario("congested", 5);
}

#[test]
fn converges_flaky() {
    converge_scenario("flaky", 6);
}

#[test]
fn converges_partition() {
    converge_scenario("partition", 7);
}

#[test]
fn hash_checks_run_and_agree_when_healthy() {
    let mut sim = Sim::new(2, profile("good_wifi"), 11);
    sim.start_guest(0);
    sim.start_guest(1);
    sim.run(200, 1);
    assert!(sim.run_until_converged(30_000));
    // let a few hash rounds happen at the settled revision
    for _ in 0..200 {
        sim.step();
    }
    for g in &sim.guests {
        assert!(g.sync.hash_checks > 0);
        assert_eq!(g.sync.desyncs_detected, 0);
    }
    assert_eq!(sim.host.repairs_sent, 0);
}

#[test]
fn late_join_gets_snapshot_then_follows() {
    let mut sim = Sim::new(2, profile("bad_wifi"), 21);
    sim.start_guest(0);
    sim.run(600, 3); // plenty of history before guest 2 exists
    assert!(sim.host.store.rev().0 > 500);
    sim.start_guest(1);
    sim.run(200, 1); // keeps mutating while it joins
    assert!(sim.run_until_converged(120_000));
    assert_eq!(sim.guests[1].sync.snapshots_loaded, 1);
    assert_eq!(sim.guests[1].sync.store.entries(), sim.host.store.entries());
    assert_eq!(sim.guests[0].sync.store.entries(), sim.host.store.entries());
    assert!(sim.host.snapshots_sent >= 2);
}

#[test]
fn rejoin_resumes_with_deltas_not_a_snapshot() {
    let mut sim = Sim::new(2, profile("mobile"), 31);
    sim.start_guest(0);
    sim.start_guest(1);
    sim.run(200, 2);
    assert!(sim.run_until_converged(60_000));
    let snaps_before = sim.host.snapshots_sent;
    sim.drop_guest(1);
    sim.run(300, 3); // guest 1 misses a lot (even ages out the ring)
    sim.reconnect_guest(1);
    sim.run(100, 1);
    assert!(sim.run_until_converged(120_000));
    assert_eq!(
        sim.guests[1].sync.snapshots_loaded, 1,
        "no second snapshot on resume"
    );
    assert!(sim.host.snapshots_sent >= snaps_before); // ring may age out while it is away
    assert_eq!(sim.guests[1].sync.store.entries(), sim.host.store.entries());
}

#[test]
fn rejoin_after_host_restart_falls_back_to_full_join() {
    let mut sim = Sim::new(1, profile("good_wifi"), 41);
    sim.start_guest(0);
    sim.run(100, 2);
    assert!(sim.run_until_converged(30_000));
    // brand-new host world with a different session id
    let mut cfg = HostConfig::new(test_hello(), 0xDEAD);
    cfg.max_guests = 1;
    sim.host = HostSync::new(cfg);
    sim.drop_guest(0);
    for _ in 0..50 {
        sim.mutate();
    }
    sim.reconnect_guest(0);
    assert!(sim.run_until_converged(60_000));
    assert_eq!(sim.guests[0].sync.snapshots_loaded, 2);
    assert_eq!(sim.guests[0].sync.store.entries(), sim.host.store.entries());
}

#[test]
fn cursor_from_the_future_gets_full_snapshot() {
    let mut sim = Sim::new(1, profile("perfect"), 51);
    sim.start_guest(0);
    sim.run(100, 3);
    assert!(sim.run_until_converged(10_000));
    // host "restores an older world": same session, lower revisions
    let old_len = sim.host.store.len();
    let mut cfg = HostConfig::new(test_hello(), 0x5E55);
    cfg.max_guests = 1;
    sim.host = HostSync::new(cfg);
    for _ in 0..10 {
        sim.mutate();
    }
    assert!(sim.host.store.rev() < sim.guests[0].sync.store.rev());
    sim.drop_guest(0);
    sim.reconnect_guest(0);
    assert!(sim.run_until_converged(30_000));
    assert_eq!(sim.guests[0].sync.snapshots_loaded, 2);
    assert!(sim.guests[0].sync.store.len() <= old_len);
    assert_eq!(sim.guests[0].sync.store.entries(), sim.host.store.entries());
}

#[test]
fn desync_is_detected_and_repaired_partially() {
    let mut sim = Sim::new(2, profile("good_wifi"), 61);
    sim.start_guest(0);
    sim.start_guest(1);
    sim.run(300, 2);
    assert!(sim.run_until_converged(30_000));
    // silently corrupt one record on guest 0 (same revision, wrong value)
    let victim = sim.universe[5];
    let entries = sim.guests[0].sync.store.entries();
    let target = entries
        .iter()
        .find(|e| e.form == victim)
        .cloned()
        .expect("victim present");
    let mut bad = target.clone();
    bad.field = match &target.field {
        Field::QuestStage { stage } => Field::QuestStage {
            stage: stage.wrapping_add(1),
        },
        Field::QuestFlags { flags } => Field::QuestFlags { flags: flags ^ 1 },
        Field::Objective { index, state } => Field::Objective {
            index: *index,
            state: state ^ 1,
        },
        Field::Global { bits } => Field::Global { bits: bits ^ 1 },
        Field::RefDisabled { disabled } => Field::RefDisabled {
            disabled: !disabled,
        },
        Field::ItemCount { item, count } => Field::ItemCount {
            item: *item,
            count: count + 1,
        },
        Field::ScriptVar { index, bits } => Field::ScriptVar {
            index: *index,
            bits: bits ^ 1,
        },
        other => other.clone(),
    };
    assert_ne!(bad.field, target.field);
    let shard = crate::ids::shard_of(victim);
    let keep: Vec<_> = sim.guests[0]
        .sync
        .store
        .entries_in_shards(&[shard])
        .into_iter()
        .filter(|e| e != &target)
        .collect();
    let mut repl = keep;
    repl.push(bad);
    sim.guests[0].sync.store.replace_shards(&[shard], repl);
    assert!(!sim.converged());
    assert!(sim.run_until_converged(60_000));
    assert!(sim.guests[0].sync.desyncs_detected >= 1);
    assert!(sim.guests[0].sync.repairs_applied >= 1);
    assert!(sim.host.repairs_sent >= 1);
    assert_eq!(sim.guests[1].sync.repairs_applied, 0);
    assert_eq!(
        sim.host.snapshots_sent, 2,
        "repair must not need a snapshot"
    );
}

#[test]
fn incompatible_peer_is_rejected() {
    let mut sim = Sim::new(1, profile("perfect"), 71);
    let mut bad = test_hello();
    bad.mods_hash ^= 1;
    sim.guests[0].sync = GuestSync::new(PlayerId(1), bad);
    sim.start_guest(0);
    sim.run(40, 0);
    assert_eq!(
        sim.guests[0].sync.phase(),
        GuestPhase::Rejected(RejectReason::ModsMismatch)
    );
    assert!(sim.host.players().is_empty());
}

#[test]
fn full_session_rejects_extra_guest() {
    let mut sim = Sim::new(2, profile("perfect"), 72);
    sim.host = {
        let mut cfg = HostConfig::new(test_hello(), 0x5E55);
        cfg.max_guests = 1;
        HostSync::new(cfg)
    };
    sim.start_guest(0);
    sim.start_guest(1);
    sim.run(40, 0);
    assert_eq!(sim.guests[0].sync.phase(), GuestPhase::Active);
    assert_eq!(
        sim.guests[1].sync.phase(),
        GuestPhase::Rejected(RejectReason::Full)
    );
    // the host counts the refusal once, however often the hello came
    assert_eq!(sim.host.full_refusals, 1);
}

#[test]
fn guest_intents_obey_authority_and_apply_once() {
    let mut sim = Sim::new(2, profile("bad_wifi"), 81);
    sim.start_guest(0);
    sim.start_guest(1);
    let avatar1 = FormId(0x0000_0014);
    let avatar2 = FormId(0x0000_0015);
    let npc = FormId(0x0001_5000);
    sim.host.auth.register_avatar(PlayerId(1), avatar1);
    sim.host.auth.register_avatar(PlayerId(2), avatar2);
    let epoch = sim.host.auth.claim_actor(npc, PlayerId(1));
    sim.run(60, 1);

    let pos = |n| Field::Position { x: n, y: n, z: n };
    let msgs = sim.guests[0].sync.send_intent(
        epoch,
        vec![
            (avatar1, pos(1)),                                      // own avatar: ok
            (npc, pos(2)),                                          // owned actor, right epoch: ok
            (avatar2, pos(3)), // someone else's avatar: refused
            (FormId(0x0100_0001), Field::QuestStage { stage: 99 }), // story: refused
        ],
    );
    for m in msgs {
        let b = proto::encode(&m).unwrap();
        sim.guests[0]
            .net
            .send(PlayerId::HOST, m.channel(), b)
            .unwrap();
    }
    // let the intent cross the link before judging convergence
    sim.run(100, 0);
    assert!(sim.run_until_converged(30_000));
    let host = &sim.host.store;
    assert!(host.get(avatar1, &pos(0)).is_some());
    assert!(host.get(npc, &pos(0)).is_some());
    assert!(host.get(avatar2, &pos(0)).is_none());
    assert!(host
        .get(FormId(0x0100_0001), &Field::QuestStage { stage: 0 })
        .map_or(true, |e| e.field != Field::QuestStage { stage: 99 }));
    assert_eq!(sim.guests[0].sync.unacked_intents(), 0);
    // the other guest sees guest 0's avatar through ordinary replication
    assert!(sim.guests[1].sync.store.get(avatar1, &pos(0)).is_some());

    // handoff to guest 2: guest 1's old epoch is now dead
    sim.host.auth.claim_actor(npc, PlayerId(2));
    let before = sim.host.store.rev();
    let msgs = sim.guests[0].sync.send_intent(epoch, vec![(npc, pos(500))]);
    for m in msgs {
        let b = proto::encode(&m).unwrap();
        sim.guests[0]
            .net
            .send(PlayerId::HOST, m.channel(), b)
            .unwrap();
    }
    sim.run(100, 0);
    assert!(sim.run_until_converged(30_000));
    assert_eq!(sim.host.store.rev(), before);
}

#[test]
fn duplicate_intent_is_applied_once() {
    let mut sim = Sim::new(1, profile("perfect"), 82);
    sim.start_guest(0);
    sim.host.auth.register_avatar(PlayerId(1), FormId(7));
    sim.run(20, 0);
    let m = Msg::Intent {
        id: 1,
        epoch: Epoch(0),
        changes: vec![(FormId(7), Field::ActorValue { av: 1, value: 5 })],
    };
    let b = proto::encode(&m).unwrap();
    for _ in 0..3 {
        sim.guests[0]
            .net
            .send(PlayerId::HOST, m.channel(), b.clone())
            .unwrap();
    }
    sim.run(20, 0);
    assert_eq!(sim.host.store.rev(), Rev(1));
    // a later write of a different value with a NEW id is applied
    let m2 = Msg::Intent {
        id: 2,
        epoch: Epoch(0),
        changes: vec![(FormId(7), Field::ActorValue { av: 1, value: 6 })],
    };
    let b2 = proto::encode(&m2).unwrap();
    sim.guests[0]
        .net
        .send(PlayerId::HOST, m2.channel(), b2)
        .unwrap();
    sim.run(20, 0);
    assert_eq!(sim.host.store.rev(), Rev(2));
    // replaying the old id does not roll the value back
    sim.guests[0]
        .net
        .send(PlayerId::HOST, m.channel(), b)
        .unwrap();
    sim.run(20, 0);
    assert_eq!(sim.host.store.rev(), Rev(2));
}

#[test]
fn same_seed_reproduces_exactly() {
    let run = |seed| {
        let mut sim = Sim::new(2, profile("congested"), seed);
        sim.start_guest(0);
        sim.start_guest(1);
        sim.run(300, 2);
        sim.run_until_converged(120_000);
        (
            sim.host_root(),
            sim.clock.now(),
            sim.host_net.stats(),
            sim.guests[0].net.stats(),
        )
    };
    assert_eq!(run(9), run(9));
    assert_ne!(run(9).0, run(10).0);
}

#[test]
fn bandwidth_stays_bounded_with_acks() {
    // with a good link, the host must not resend history forever
    let mut sim = Sim::new(1, profile("good_wifi"), 91);
    sim.start_guest(0);
    sim.run(100, 2);
    assert!(sim.run_until_converged(10_000));
    let sent_before = sim.host_net.stats().sent;
    for _ in 0..200 {
        sim.step(); // idle: no writes
    }
    let idle_sent = sim.host_net.stats().sent - sent_before;
    // only periodic hashes (and nothing else) once acked and quiet
    assert!(idle_sent <= 10, "idle sent {idle_sent}");
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(10))]

    #[test]
    fn any_seed_any_profile_converges(seed in any::<u64>(), pi in 0usize..6, guests in 1u8..=3) {
        let mut sim = Sim::new(guests, profile(PROFILE_NAMES[pi]), seed);
        for i in 0..guests as usize { sim.start_guest(i); }
        sim.run(160, 2);
        prop_assert!(sim.run_until_converged(240_000));
        prop_assert_eq!(sim.decode_errors, 0);
    }

    #[test]
    fn deltas_in_any_order_with_duplicates_converge(seed in any::<u64>()) {
        let mut rng = StdRng::seed_from_u64(seed);
        let mut host = StateStore::new();
        let mut deltas = Vec::new();
        let mut base = Rev::ZERO;
        for round in 0..12u32 {
            for _ in 0..rng.gen_range(1..8) {
                let f = FormId(rng.gen_range(0..20));
                host.write_local(f, Field::QuestStage { stage: rng.gen_range(0..1000) });
            }
            deltas.push(host.delta_since(base, Tick(round), 100));
            // sometimes the sender's baseline lags (ack lost)
            if rng.gen_bool(0.6) { base = host.rev(); }
        }
        let mut sched = deltas.clone();
        sched.extend(deltas.iter().cloned());
        sched.shuffle(&mut rng);
        // make sure the freshest delta from rev 0 is there so a lagging
        // baseline cannot leave holes (that is the sender's guarantee)
        sched.push(host.delta_since(Rev::ZERO, Tick(99), usize::MAX));
        let mut g = StateStore::new();
        for d in &sched { g.apply_delta(d); }
        prop_assert_eq!(g.entries(), host.entries());
        prop_assert_eq!(g.base(), host.rev());
        // idempotent: replaying everything changes nothing
        let snap = g.entries();
        for d in &sched { g.apply_delta(d); }
        prop_assert_eq!(g.entries(), snap);
    }
}

#[test]
fn live_messages_that_beat_the_join_snapshot_are_kept_not_dropped() {
    use crate::actors::{ActorMsg, ActorState};
    let mut sim = Sim::new(1, profile("perfect"), 3);
    sim.start_guest(0);
    // the guest is joining: the host accepted it but its snapshot has not been applied yet
    assert!(sim.run_until(60_000, |s| s.guests[0].sync.phase() == GuestPhase::Joining));
    let a = FormId(0x77);
    let assign = ActorMsg::Assign {
        actor: a,
        owner: None,
        epoch: Epoch(1),
        seed: Some(ActorState::default()),
    };
    sim.guests[0].sync.on_msg(Msg::Actor(assign));
    assert!(sim.run_until(60_000, |s| s.guests[0].sync.phase() == GuestPhase::Active));
    assert_eq!(
        sim.guests[0].sync.live.actors.iter().count(),
        1,
        "the early assignment was applied once the guest was active"
    );
}

#[test]
fn a_stale_rejoin_after_the_host_dropped_the_guest_restarts_the_handshake_not_kills_the_guest() {
    // The host drops a guest after peer_timeout (8 s) of silence; a Rejoin/Join the guest sent
    // from an older Welcome can reach the host after that. It used to answer BadMessage, which
    // leaves the guest in GuestPhase::Rejected for good (logged "host refused the connection").
    for stale_join in [false, true] {
        let mut sim = Sim::new(1, profile("perfect"), 11);
        sim.start_guest(0);
        assert!(sim.run_until_converged(60_000));
        sim.host.disconnect(PlayerId(1));
        let msg = if stale_join {
            Msg::Join
        } else {
            Msg::Rejoin {
                session: 0,
                last_seq: crate::ids::Seq(1),
                have_rev: Rev(1),
            }
        };
        sim.guest_emit(0, vec![msg]);
        sim.run(40, 0);
        assert!(
            !matches!(sim.guests[0].sync.phase(), GuestPhase::Rejected(_)),
            "join={stale_join}: guest was rejected"
        );
        sim.reconnect_guest(0);
        assert!(
            sim.run_until_converged(120_000),
            "join={stale_join}: guest never came back"
        );
    }
}

fn listed(plugins: &[&str]) -> crate::proto::Hello {
    let mut h = test_hello();
    h.plugins = plugins.iter().map(|s| s.to_string()).collect();
    h.mods_hash = crate::compat::plugin_list_hash(&h.plugins);
    h
}

#[test]
fn missing_plugin_is_refused_and_the_guest_learns_which() {
    let mut sim = Sim::new(1, profile("perfect"), 73);
    sim.host = HostSync::new(HostConfig::new(
        listed(&["FalloutNV.esm", "DeadMoney.esm"]),
        0x5E55,
    ));
    sim.guests[0].sync = GuestSync::new(PlayerId(1), listed(&["FalloutNV.esm"]));
    sim.start_guest(0);
    sim.run(40, 0);
    assert_eq!(
        sim.guests[0].sync.phase(),
        GuestPhase::Rejected(RejectReason::PluginsMismatch)
    );
    let found = sim.guests[0].sync.compat_issues();
    assert_eq!(
        found,
        vec![crate::compat::Issue::MissingPlugins(vec![
            "DeadMoney.esm".into()
        ])]
    );
    assert_eq!(sim.host.compat_log.len(), 1);
    assert!(sim.host.compat_log[0].refused);
    assert!(sim.host.players().is_empty());
}

#[test]
fn override_admits_a_mismatched_guest_with_a_warning() {
    let mut sim = Sim::new(1, profile("perfect"), 74);
    let mut cfg = HostConfig::new(listed(&["FalloutNV.esm", "A.esp"]), 0x5E55);
    cfg.compat.allow_mismatch = true;
    sim.host = HostSync::new(cfg);
    sim.guests[0].sync = GuestSync::new(PlayerId(1), listed(&["FalloutNV.esm", "B.esp"]));
    sim.start_guest(0);
    sim.run(60, 0);
    assert_eq!(sim.guests[0].sync.phase(), GuestPhase::Active);
    assert!(
        !sim.guests[0].sync.compat_issues().is_empty(),
        "guest is told it joined with differences"
    );
    assert!(!sim.host.compat_log[0].refused);
}

#[test]
fn override_does_not_admit_a_protocol_mismatch() {
    let mut sim = Sim::new(1, profile("perfect"), 75);
    let mut cfg = HostConfig::new(test_hello(), 0x5E55);
    cfg.compat.allow_mismatch = true;
    sim.host = HostSync::new(cfg);
    let mut bad = test_hello();
    bad.proto += 1;
    sim.guests[0].sync = GuestSync::new(PlayerId(1), bad);
    sim.start_guest(0);
    sim.run(40, 0);
    assert_eq!(
        sim.guests[0].sync.phase(),
        GuestPhase::Rejected(RejectReason::ProtocolMismatch)
    );
}

#[test]
fn recent_roots_match_a_lagging_guest_at_its_own_revision() {
    use crate::hash::ShardHashes;
    let mut sim = Sim::new(1, profile("bad_wifi"), 77);
    sim.start_guest(0);
    sim.run(100, 2);
    assert!(sim.run_until_converged(60_000));
    let (mut behind, mut checked) = (0, 0);
    for _ in 0..300 {
        sim.run(1, 3);
        let g = &sim.guests[0].sync.store;
        if g.base() != g.rev() {
            continue;
        }
        let points = sim.host.recent_roots(u64::MAX, &|_| true);
        assert_eq!(points[0].0, sim.host.store.rev());
        let ours = ShardHashes::compute(g).root();
        let at = points.iter().find(|(r, _)| *r == g.rev());
        assert_eq!(at.map(|p| p.1), Some(ours), "guest rev {:?}", g.rev());
        checked += 1;
        if g.rev() < sim.host.store.rev() {
            behind += 1;
        }
    }
    assert!(
        checked > 50 && behind > 10,
        "checked {checked}, behind {behind}"
    );
    // a field the filter drops does not count at any revision
    let skip = |f: &Field| !matches!(f, Field::QuestStage { .. });
    let points = sim.host.recent_roots(u64::MAX, &skip);
    let mut kept = StateStore::new();
    for e in sim
        .host
        .store
        .entries()
        .into_iter()
        .filter(|e| skip(&e.field))
    {
        kept.write_local(e.form, e.field);
    }
    assert_eq!(points[0].1, ShardHashes::compute(&kept).root());
}

#[test]
fn recent_roots_stop_at_a_write_the_host_did_not_log() {
    let mut sim = Sim::new(1, profile("perfect"), 78);
    sim.run(10, 2);
    assert!(sim.host.recent_roots(u64::MAX, &|_| true).len() > 10);
    sim.host
        .store
        .write_local(FormId(0x0700_0001), Field::QuestStage { stage: 5 });
    assert_eq!(sim.host.recent_roots(u64::MAX, &|_| true).len(), 1);
    sim.host
        .write(FormId(0x0700_0001), Field::QuestStage { stage: 6 });
    assert_eq!(sim.host.recent_roots(u64::MAX, &|_| true).len(), 2);
}

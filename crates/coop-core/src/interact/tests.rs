use super::*;
use crate::interest::InterestMap;
use crate::proto::{self, Channel, Msg};
use crate::sim_test::Sim;
use crate::sync::GuestPhase;
use crate::transport::{ChaosProfile, PROFILE_NAMES};

const H: PlayerId = PlayerId(0);
const G1: PlayerId = PlayerId(1);
const G2: PlayerId = PlayerId(2);

fn f(n: u32) -> FormId {
    FormId(n)
}

fn sample_acts() -> Vec<Act> {
    vec![
        Act::World(Shared::Door {
            door: f(1),
            open: true,
        }),
        Act::Activate { obj: f(2) },
        Act::Door { door: f(3) },
        Act::Pickup {
            obj: f(4),
            item: f(5),
            count: 3,
        },
        Act::Drop {
            item: f(5),
            count: 2,
            cell: CellId(9),
            pos: [1, -2, 3],
        },
        Act::Barter {
            vendor: f(6),
            buys: vec![(f(7), 2)],
            sells: vec![(f(8), 1)],
            total: -40,
        },
        Act::Steal {
            target: f(9),
            item: f(10),
            count: 1,
            mode: StealMode::Pickpocket,
        },
        Act::Craft {
            bench: f(11),
            recipe: f(12),
            times: 2,
        },
        Act::Repair {
            item: f(13),
            part: f(14),
            vendor: f(0),
            cost: 0,
        },
        Act::Use { item: f(15) },
        Act::Equip {
            item: f(16),
            on: true,
        },
        Act::Ammo { ammo: f(17) },
        Act::Pip(PipOp::Hotkey {
            slot: 3,
            item: f(18),
        }),
        Act::Companion {
            who: f(19),
            cmd: CompanionCmd::Wait,
        },
        Act::Lockpick {
            obj: f(20),
            won: true,
            pins: 2,
            broke: false,
        },
        Act::Hack {
            obj: f(21),
            won: false,
        },
        Act::Key {
            obj: f(22),
            key: f(23),
        },
        Act::Trap {
            obj: f(24),
            what: TrapOutcome::Detonated,
        },
        Act::Discover { marker: f(25) },
        Act::Transition {
            door: f(26),
            from: CellId(1),
            to: CellId(2),
            one_way: false,
        },
    ]
}

#[test]
fn every_act_round_trips_on_the_reliable_channel() {
    for (i, act) in sample_acts().into_iter().enumerate() {
        let m = Msg::Interact(InteractMsg::Request {
            txn: i as u32 + 1,
            act,
        });
        assert_eq!(m.channel(), Channel::ReliableOrdered);
        let bytes = proto::encode(&m).unwrap();
        assert_eq!(proto::decode(Channel::ReliableOrdered, &bytes).unwrap(), m);
    }
    for m in [
        InteractMsg::Verdict {
            txn: 1,
            ok: false,
            why: Why::Locked,
        },
        InteractMsg::Shared(Shared::Lock {
            obj: f(1),
            state: LockState {
                level: Some(2),
                broken: true,
            },
        }),
        InteractMsg::Sync(vec![Shared::Mine {
            obj: f(1),
            phase: MinePhase::Gone,
        }]),
        InteractMsg::Follow(Follow {
            door: f(1),
            to: CellId(5),
            leader: H,
        }),
    ] {
        let m = Msg::Interact(m);
        let bytes = proto::encode(&m).unwrap();
        assert_eq!(proto::decode(Channel::ReliableOrdered, &bytes).unwrap(), m);
    }
}

#[test]
fn oversized_messages_are_insane() {
    let big = Act::Barter {
        vendor: f(1),
        buys: (0..MAX_LINES as u32 + 1).map(|i| (f(i), 1)).collect(),
        sells: vec![],
        total: 0,
    };
    assert!(!InteractMsg::Request { txn: 1, act: big }.is_sane());
    assert!(!InteractMsg::Sync(vec![
        Shared::Door {
            door: f(1),
            open: true
        };
        SYNC_CHUNK + 1
    ])
    .is_sane());
}

#[test]
fn host_applies_each_request_once_and_replays_the_verdict() {
    let mut h = InteractHost::new();
    let req = InteractMsg::Request {
        txn: 1,
        act: Act::Use { item: f(1) },
    };
    assert!(h.on_msg(G1, req.clone()).is_empty());
    let q = h.take_queue();
    assert_eq!(q.len(), 1);
    let outs = h.resolve(G1, 1, Outcome::done());
    assert_eq!(outs.len(), 1);
    // a duplicate does not queue again and gets the same verdict
    let again = h.on_msg(G1, req.clone());
    assert!(h.take_queue().is_empty());
    assert_eq!(again.len(), 1);
    assert_eq!(
        again[0].msg,
        InteractMsg::Verdict {
            txn: 1,
            ok: true,
            why: Why::None
        }
    );
    assert_eq!(h.stats.requests, 1);
    assert_eq!(h.stats.replays, 1);
    // another player has its own txn space
    h.on_msg(G2, req.clone());
    assert_eq!(h.take_queue().len(), 1);
    // txn 0 is not a request
    assert!(h
        .on_msg(
            G1,
            InteractMsg::Request {
                txn: 0,
                act: Act::Use { item: f(1) }
            }
        )
        .is_empty());
    assert!(h.take_queue().is_empty());
    // a leaver starts over
    h.forget(G1);
    h.on_msg(G1, req);
    assert_eq!(h.take_queue().len(), 1);
}

#[test]
fn shared_log_keeps_the_newest_per_key_and_skips_no_ops() {
    let mut h = InteractHost::new();
    assert_eq!(
        h.publish(vec![Shared::Door {
            door: f(1),
            open: true
        }])
        .len(),
        1
    );
    assert!(
        h.publish(vec![Shared::Door {
            door: f(1),
            open: true
        }])
        .is_empty(),
        "no change, no message"
    );
    assert_eq!(
        h.publish(vec![Shared::Door {
            door: f(1),
            open: false
        }])
        .len(),
        1
    );
    assert_eq!(h.log_len(), 1);
    let sync = h.sync_for(G1);
    assert_eq!(sync.len(), 1);
    assert_eq!(
        sync[0].msg,
        InteractMsg::Sync(vec![Shared::Door {
            door: f(1),
            open: false
        }])
    );
    for i in 0..(SYNC_CHUNK as u32 * 2 + 1) {
        h.publish(vec![Shared::Door {
            door: f(100 + i),
            open: true,
        }]);
    }
    assert_eq!(h.sync_for(G1).len(), 3, "chunked");
}

#[test]
fn dropped_item_forms_are_unique_and_in_the_reserved_block() {
    let mut h = InteractHost::new();
    let a = h.mint_drop();
    let b = h.mint_drop();
    assert_ne!(a, b);
    assert!(a.0 >= DROP_BASE && b.0 >= DROP_BASE);
}

fn interest(cells: &[(PlayerId, u32)], links: &[(u32, u32)]) -> InterestMap {
    let mut i = InterestMap::new();
    for (p, c) in cells {
        i.set_peer_cell(*p, CellId(*c));
    }
    for (a, b) in links {
        i.link(CellId(*a), CellId(*b));
    }
    i
}

#[test]
fn leader_goes_anywhere_and_the_party_in_the_cell_group_follows() {
    let t = Transit::default();
    let m = [H, G1, G2];
    // G1 stands with the leader, G2 is far away
    let i = interest(&[(H, 1), (G1, 1), (G2, 50)], &[]);
    assert_eq!(
        t.decide(H, CellId(1), CellId(2), false, &m, &i),
        Decision::Go {
            followers: vec![G1]
        }
    );
    // a linked neighbour cell counts as the same group
    let i = interest(&[(H, 1), (G1, 3), (G2, 50)], &[(1, 3)]);
    assert_eq!(
        t.decide(H, CellId(1), CellId(2), false, &m, &i),
        Decision::Go {
            followers: vec![G1]
        }
    );
}

#[test]
fn non_leaders_stay_in_the_leaders_cell_group() {
    let t = Transit::default();
    let m = [H, G1];
    let i = interest(&[(H, 1), (G1, 1)], &[(1, 2)]);
    // into a cell linked to the leader's: fine, alone
    assert_eq!(
        t.decide(G1, CellId(1), CellId(2), false, &m, &i),
        Decision::Go { followers: vec![] }
    );
    // into the leader's own cell: fine
    assert_eq!(
        t.decide(G1, CellId(7), CellId(1), false, &m, &i),
        Decision::Go { followers: vec![] }
    );
    // away from the leader: refused
    assert_eq!(
        t.decide(G1, CellId(1), CellId(9), false, &m, &i),
        Decision::Deny(Why::Policy)
    );
}

#[test]
fn one_way_transitions_need_the_barrier_unless_alone() {
    let t = Transit::default();
    let i = interest(&[(H, 1), (G1, 1)], &[]);
    assert_eq!(
        t.decide(H, CellId(1), CellId(2), true, &[H, G1], &i),
        Decision::Barrier
    );
    assert_eq!(
        t.decide(G1, CellId(1), CellId(2), true, &[H, G1], &i),
        Decision::Barrier
    );
    assert_eq!(
        t.decide(H, CellId(1), CellId(2), true, &[H], &i),
        Decision::Go { followers: vec![] }
    );
    assert_eq!(t.everyone(G1, &[H, G1, G2]), vec![H, G2]);
}

fn route(sim: &mut Sim, outs: Vec<IOut>) {
    let mut sends = Vec::new();
    for o in outs {
        sends.extend(sim.host.live.route_extern(o.aud, Msg::Interact(o.msg)));
    }
    sim.host_emit(sends);
}

/// The host game's part: execute what is queued (here: every request
/// succeeds and a `Use` publishes a door change), counting executions.
fn serve(sim: &mut Sim, executed: &mut Vec<(PlayerId, u32)>) {
    let q = sim.host.live.interact.take_queue();
    for (p, txn, act) in q {
        executed.push((p, txn));
        let mut o = Outcome::done();
        if let Act::Door { door } = act {
            o = o.with_shared(vec![Shared::Door { door, open: true }]);
        }
        let outs = sim.host.live.interact.resolve(p, txn, o);
        route(sim, outs);
    }
}

fn active(sim: &Sim) -> bool {
    sim.guests
        .iter()
        .all(|g| g.sync.phase() == GuestPhase::Active)
}

#[test]
fn requests_apply_once_and_shared_state_reaches_everyone_under_every_profile() {
    let mut profiles = vec![ChaosProfile::PERFECT];
    profiles.extend(
        PROFILE_NAMES
            .iter()
            .map(|n| ChaosProfile::by_name(n).unwrap()),
    );
    for (n, profile) in profiles.into_iter().enumerate() {
        let name = profile.name;
        let mut sim = Sim::new(2, profile, 700 + n as u64);
        sim.start_guest(0);
        sim.start_guest(1);
        assert!(sim.run_until(120_000, |s| active(s)), "{name}: join");
        let mut executed = Vec::new();
        // guest 1 asks for a door twice (a resend) and a use; guest 2 asks for a use
        let m = sim.guests[0]
            .sync
            .live
            .interact
            .request(Act::Door { door: f(0x500) });
        sim.guest_emit(0, vec![m.clone(), m]);
        let m = sim.guests[0]
            .sync
            .live
            .interact
            .request(Act::Use { item: f(0x600) });
        sim.guest_emit(0, vec![m]);
        let m = sim.guests[1]
            .sync
            .live
            .interact
            .request(Act::Use { item: f(0x600) });
        sim.guest_emit(1, vec![m]);
        let end = sim.clock.now() + 60_000;
        let mut got = false;
        while sim.clock.now() < end && !got {
            sim.step();
            serve(&mut sim, &mut executed);
            got = sim.guests[0].sync.live.interact.verdicts == 2
                && sim.guests[1].sync.live.interact.verdicts == 1
                && sim.guests.iter().all(|g| {
                    g.sync
                        .live
                        .interact
                        .shared
                        .iter()
                        .any(|s| matches!(s, Shared::Door { open: true, .. }))
                });
        }
        assert!(got, "{name}: verdicts and the shared door arrived");
        assert_eq!(
            executed.len(),
            3,
            "{name}: each distinct request ran exactly once: {executed:?}"
        );
        let mut u = executed.clone();
        u.sort();
        u.dedup();
        assert_eq!(u.len(), 3, "{name}: no repeats");
        assert!(sim.guests[0]
            .sync
            .live
            .interact
            .settled
            .iter()
            .all(|s| s.ok));
    }
}

#[test]
fn a_late_joiner_receives_the_shared_log() {
    let mut sim = Sim::new(
        2,
        ChaosProfile::by_name("bad_wifi").unwrap_or(ChaosProfile::PERFECT),
        91,
    );
    sim.start_guest(0);
    assert!(sim.run_until(120_000, |s| s.guests[0].sync.phase() == GuestPhase::Active));
    let outs = sim.host.live.interact.publish(vec![
        Shared::Door {
            door: f(1),
            open: true,
        },
        Shared::Lock {
            obj: f(2),
            state: LockState {
                level: None,
                broken: false,
            },
        },
        Shared::Terminal {
            obj: f(3),
            hacked: true,
            lockouts: 1,
        },
    ]);
    route(&mut sim, outs);
    sim.start_guest(1);
    assert!(
        sim.run_until(120_000, |s| s.guests[1].sync.phase() == GuestPhase::Active
            && s.guests[1].sync.live.interact.shared.len() >= 3)
    );
    assert_eq!(sim.guests[1].sync.live.interact.shared.len(), 3);
}

#[test]
fn follow_orders_reach_only_the_named_players() {
    let mut sim = Sim::new(2, ChaosProfile::PERFECT, 5);
    sim.start_guest(0);
    sim.start_guest(1);
    assert!(sim.run_until(120_000, |s| active(s)));
    let fo = Follow {
        door: f(9),
        to: CellId(4),
        leader: H,
    };
    let outs = sim.host.live.interact.follow(fo, &[G2]);
    route(&mut sim, outs);
    assert!(sim.run_until(30_000, |s| !s.guests[1]
        .sync
        .live
        .interact
        .follows
        .is_empty()));
    sim.run_until(2_000, |_| false);
    assert!(sim.guests[0].sync.live.interact.follows.is_empty());
    assert_eq!(sim.guests[1].sync.live.interact.follows, vec![fo]);
}

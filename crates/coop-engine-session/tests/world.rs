//! The rest of the world's state between games in one process (docs/coop/SPECS.md S12): placed
//! objects, containers, factions, reputations, notes and dialogue responses as the host's records,
//! and doors and locks both ways. The games are test stories with the world kit's stand-ins for
//! the game functions; the transport is coop-core's loopback.

use coop_core::compat::plugin_list_hash;
use coop_core::ids::PlayerId;
use coop_core::proto::{Hello, PROTOCOL_VERSION};
use coop_core::schema::Field;
use coop_core::sync::HostConfig;
use coop_core::transport::{LoopbackNet, Transport};
use coop_engine::changes::Class;
use coop_engine::testkit::{Story, StorySpec};
use coop_engine::world::Lock;
use coop_engine::worldkit::{self as kit, RefSpec, WorldSpec};
use coop_engine::{addr, world};
use coop_engine_session::world::{GuestWorld, HostWorld, LOCAL_GRACE_STEPS, RECORD_TAG};
use coop_engine_session::{echo, gate_guest, GuestDriver, HostDriver};

engine_core::assemble!();

const MQ: u32 = 0x0010_4c0c;
const CELL: u32 = 0x0010_5a00;
const DOOR: u32 = 0x0010_5a10;
const CHEST: u32 = 0x0010_5a11;
const CRATE: u32 = 0x0010_5a12;
const SAFE: u32 = 0x0010_5a13;
const FACTION: u32 = 0x0001_b2a4;
const OTHER_FACTION: u32 = 0x000e_e68b;
const REP: u32 = 0x0010_4c22;
const NOTE: u32 = 0x0010_2a01;
const INFO: u32 = 0x0010_2b01;
const CAPS: u32 = 0x0000_000f;
const STIMPAK: u32 = 0x0100_15e9;
const DYNAMITE: u32 = 0x0200_109a;
const CHET: u32 = 0x0010_4f25;

fn machine() -> Story {
    let mut s = Story::new(&StorySpec {
        quests: vec![(MQ, vec![10, 20])],
        cells: vec![CELL],
        actors: vec![CHET],
        ..StorySpec::default()
    });
    kit::install(
        &mut s,
        &WorldSpec {
            refs: vec![
                RefSpec {
                    id: DOOR,
                    cell: CELL,
                    opens: true,
                    lock: Some(50),
                    items: vec![],
                },
                RefSpec {
                    id: CHEST,
                    cell: CELL,
                    opens: true,
                    lock: None,
                    items: vec![(CAPS, 20), (STIMPAK, 2)],
                },
                RefSpec {
                    id: CRATE,
                    cell: CELL,
                    ..RefSpec::default()
                },
                RefSpec {
                    id: SAFE,
                    cell: CELL,
                    opens: true,
                    lock: Some(100),
                    items: vec![(CAPS, 500)],
                },
            ],
            factions: vec![FACTION, OTHER_FACTION],
            reputations: vec![REP],
            notes: vec![NOTE],
            topics: vec![INFO],
        },
    );
    s
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

struct Run {
    host: Story,
    hd: HostDriver,
    hostnet: Box<dyn Transport>,
    guests: Vec<(Story, GuestDriver, Box<dyn Transport>)>,
    net: LoopbackNet,
    now: u64,
}

impl Run {
    /// A host sharing the world and `guests` guests, joined and settled.
    fn new(guests: u8, delta_cap: usize) -> Run {
        let net = LoopbackNet::new();
        let mut host = machine();
        let mut cfg = HostConfig::new(hello(), 0x5eed);
        cfg.delta_cap = delta_cap;
        let mut hd = HostDriver::new(cfg, [MQ], []);
        hd.share_world(HostWorld::new());
        hd.start(&mut host.e);
        let hostnet = Box::new(net.endpoint(PlayerId::HOST));
        let mut run = Run {
            host,
            hd,
            hostnet,
            guests: Vec::new(),
            net,
            now: 0,
        };
        for p in 1..=guests {
            run.join(p);
        }
        run.frames(20);
        run
    }

    fn join(&mut self, p: u8) {
        let mut g = machine();
        gate_guest(&mut g.e);
        let mut d = GuestDriver::new(PlayerId(p), hello());
        d.world = Some(GuestWorld::new());
        self.guests
            .push((g, d, Box::new(self.net.endpoint(PlayerId(p)))));
    }

    fn frames(&mut self, n: usize) {
        for _ in 0..n {
            self.now += 50;
            self.hd
                .step(&mut self.host.e, self.hostnet.as_mut(), self.now);
            for (g, d, net) in self.guests.iter_mut() {
                d.step(&mut g.e, net.as_mut(), self.now);
                assert!(echo(&mut g.e, &d.sync.store).is_empty());
            }
        }
    }

    fn hw(&self) -> &HostWorld {
        self.hd.world.as_ref().unwrap()
    }

    fn gw(&self, i: usize) -> &GuestWorld {
        self.guests[i].1.world.as_ref().unwrap()
    }

    /// Every machine's kit forms equal the host's, and nothing was refused or drifted.
    fn assert_same_world(&mut self) {
        for (i, (g, d, _)) in self.guests.iter_mut().enumerate() {
            for id in [DOOR, CHEST, CRATE, SAFE] {
                assert_eq!(
                    kit::ref_state(g, id),
                    kit::ref_state(&mut self.host, id),
                    "guest {} reference {id:08x}",
                    i + 1
                );
            }
            for id in [FACTION, OTHER_FACTION] {
                assert_eq!(kit::faction(g, id), kit::faction(&mut self.host, id));
            }
            assert_eq!(
                kit::reputation(g, REP),
                kit::reputation(&mut self.host, REP)
            );
            for id in [NOTE, INFO] {
                assert_eq!(kit::marked(g, id), kit::marked(&mut self.host, id));
            }
            // Loading a record starts no script on a guest (the kit's load would run one).
            assert_eq!(g.scripts_run(), 0, "a script ran on guest {}", i + 1);
            let w = d.world.as_ref().unwrap();
            assert_eq!(w.refused, 0, "{:?}", w.notes);
            assert_eq!(w.records_differ, 0, "{:?}", w.notes);
            assert_eq!(d.refused, 0);
            assert_eq!(d.sync.desyncs_detected, 0);
            // Neither game's own save numbering was used for a record.
            assert_eq!(kit::own_numbering(g), 1);
            assert!(kit::own_map_in_place(g));
        }
        assert_eq!(kit::own_numbering(&self.host), 1);
        assert!(kit::own_map_in_place(&self.host));
    }
}

/// What the host's game does in the test: every kind of change the stream carries.
fn host_plays(run: &mut Run) {
    let h = &mut run.host;
    kit::disable(h, CRATE, true);
    kit::move_to(h, CRATE, [1.0, 2.0, 3.0]);
    kit::set_item(h, CHEST, STIMPAK, 0);
    kit::set_item(h, CHEST, DYNAMITE, 3);
    run.frames(3);
    let h = &mut run.host;
    kit::set_faction(
        h,
        FACTION,
        &(1, [2, 0], vec![(OTHER_FACTION, -50, 1)]),
        2 | 4 | 0x8000_0000,
    );
    kit::set_reputation(h, REP, 5.0, 1.0);
    kit::mark(h, NOTE);
    kit::mark(h, INFO);
    kit::set_locked_as_played(h, DOOR, false);
    kit::open(h, DOOR, true);
    assert!(h.set_stage(MQ, 10));
    run.frames(3);
}

#[test]
fn guests_get_the_hosts_world_one_key_per_delta() {
    let mut run = Run::new(2, 1);
    host_plays(&mut run);
    run.frames(120);
    run.assert_same_world();
    assert!(run.hw().records >= 7);
    assert_eq!(run.hw().too_large, 0);
    for i in 0..2 {
        assert!(run.gw(i).records_applied >= 7);
    }
    // The story channel beside it is as it was: the quest arrived too.
    let (g, _, _) = &mut run.guests[0];
    assert_eq!(coop_engine::forms::quest_stage(&mut g.e, MQ), Some(10));
}

#[test]
fn a_guest_that_joins_later_gets_the_world_from_the_snapshot() {
    let mut run = Run::new(1, 24);
    host_plays(&mut run);
    run.frames(20);
    run.join(2);
    run.frames(60);
    run.assert_same_world();
    assert!(run.guests[1].1.sync.snapshots_loaded >= 1);
    assert!(run.gw(1).records_applied >= 7);
}

#[test]
fn a_door_shut_again_on_the_host_shuts_on_the_guests_though_the_journal_is_silent() {
    let mut run = Run::new(1, 24);
    kit::open(&mut run.host, DOOR, true);
    run.frames(10);
    assert!(kit::ref_state(&mut run.guests[0].0, DOOR).unwrap().open);
    // Shut: the game removes the change flag, nothing passes the journal. The host reads its
    // doors every step.
    kit::open(&mut run.host, DOOR, false);
    run.frames(10);
    assert!(!kit::ref_state(&mut run.guests[0].0, DOOR).unwrap().open);
    run.assert_same_world();
    // Twin: with doors and locks not shared the same shut door stays open on the guest.
    let mut run = Run::new(1, 24);
    kit::open(&mut run.host, DOOR, true);
    run.frames(10);
    run.hd.world.as_mut().unwrap().doors_and_locks = false;
    kit::open(&mut run.host, DOOR, false);
    run.frames(10);
    assert!(kit::ref_state(&mut run.guests[0].0, DOOR).unwrap().open);
}

#[test]
fn a_door_a_guest_opens_is_open_for_everyone_through_the_host() {
    let mut run = Run::new(2, 24);
    // Guest 1's player opens the door in its own game.
    kit::open(&mut run.guests[0].0, DOOR, true);
    run.frames(10);
    assert!(kit::ref_state(&mut run.host, DOOR).unwrap().open);
    assert!(kit::ref_state(&mut run.guests[1].0, DOOR).unwrap().open);
    assert_eq!(run.hw().enacted, 1);
    assert_eq!(run.gw(0).told_host, 1);
    // The other guest got its door from the host's state (the door's record, which the host's
    // game flagged when it opened it) and told nobody.
    assert_eq!(run.gw(1).told_host, 0);
    // Guest 2 shuts it: shut everywhere, the host's game having done it.
    kit::open(&mut run.guests[1].0, DOOR, false);
    run.frames(10);
    assert!(!kit::ref_state(&mut run.host, DOOR).unwrap().open);
    assert!(!kit::ref_state(&mut run.guests[0].0, DOOR).unwrap().open);
    assert_eq!(run.hw().enacted, 2);
    run.frames(40);
    run.assert_same_world();
    // Nothing went round again: each own change was told once.
    assert_eq!((run.gw(0).told_host, run.gw(1).told_host), (1, 1));
    assert_eq!((run.gw(0).put_back, run.gw(1).put_back), (0, 0));
}

#[test]
fn a_lock_a_guest_picks_or_breaks_is_so_for_everyone() {
    let mut run = Run::new(2, 24);
    // Guest 2 fails twice on the safe, then guest 1 picks the door.
    kit::fail_lock(&mut run.guests[1].0, SAFE);
    kit::fail_lock(&mut run.guests[1].0, SAFE);
    kit::set_locked_as_played(&mut run.guests[0].0, DOOR, false);
    run.frames(10);
    let safe = Lock {
        level: 100,
        flags: addr::LOCK_FLAG_LOCKED,
        tries: 2,
    };
    let door = Lock {
        level: 50,
        flags: 0,
        tries: 0,
    };
    let (g1, g2) = run.guests.split_at_mut(1);
    for s in [&mut run.host, &mut g1[0].0, &mut g2[0].0] {
        assert_eq!(world::lock(&mut s.e, SAFE), Some(safe));
        assert_eq!(world::lock(&mut s.e, DOOR), Some(door));
    }
    assert_eq!(run.hw().enacted, 2);
    // A lock the host's script locks again is locked for the guests.
    kit::set_locked_as_played(&mut run.host, DOOR, true);
    run.frames(10);
    assert!(world::lock(&mut run.guests[1].0.e, DOOR).unwrap().locked());
    run.frames(40);
    run.assert_same_world();
    assert_eq!((run.gw(0).told_host, run.gw(1).told_host), (1, 1));
}

#[test]
fn a_guests_own_change_the_host_does_not_take_up_is_put_back() {
    let mut run = Run::new(1, 24);
    kit::open(&mut run.host, DOOR, true);
    run.frames(10);
    // The door is not in the host's game when the guest shuts it (its cell is not loaded there).
    run.host.set_present(DOOR, false);
    kit::open(&mut run.guests[0].0, DOOR, false);
    run.frames(10);
    assert_eq!(run.hw().refused, 1);
    // Left alone for the grace, then put back to the host's state.
    assert!(!kit::ref_state(&mut run.guests[0].0, DOOR).unwrap().open);
    run.frames(LOCAL_GRACE_STEPS as usize + 40);
    assert!(kit::ref_state(&mut run.guests[0].0, DOOR).unwrap().open);
    assert_eq!(run.gw(0).put_back, 1);
}

#[test]
fn a_record_waits_for_its_form_and_for_a_game_that_can_take_it() {
    let mut run = Run::new(1, 24);
    // The crate's cell is not loaded on the guest, and the guest's game is loading.
    run.guests[0].0.set_present(CRATE, false);
    kit::set_loading(&mut run.guests[0].0, true);
    kit::disable(&mut run.host, CRATE, true);
    kit::mark(&mut run.host, NOTE);
    run.frames(10);
    assert_eq!(run.gw(0).records_applied, 0);
    assert!(!kit::marked(&mut run.guests[0].0, NOTE));
    kit::set_loading(&mut run.guests[0].0, false);
    run.frames(3);
    assert!(kit::marked(&mut run.guests[0].0, NOTE));
    assert_eq!(run.gw(0).records_applied, 1);
    run.guests[0].0.set_present(CRATE, true);
    run.frames(3);
    assert!(
        kit::ref_state(&mut run.guests[0].0, CRATE)
            .unwrap()
            .disabled
    );
    run.assert_same_world();
}

#[test]
fn a_class_taken_out_does_not_travel() {
    let mut run = Run::new(1, 24);
    run.hd
        .world
        .as_mut()
        .unwrap()
        .classes
        .remove(&Class::Container);
    kit::set_item(&mut run.host, CHEST, DYNAMITE, 3);
    kit::disable(&mut run.host, CRATE, true);
    run.frames(10);
    assert_eq!(
        kit::ref_state(&mut run.guests[0].0, CHEST).unwrap().items,
        vec![(CAPS, 20), (STIMPAK, 2)]
    );
    assert!(
        kit::ref_state(&mut run.guests[0].0, CRATE)
            .unwrap()
            .disabled
    );
    // No record of the chest is in the replica at all.
    let chest_records = run
        .hd
        .sync
        .store
        .iter()
        .filter(|(k, en)| {
            k.form.0 == CHEST
                && matches!(&en.field, Field::Custom { tag, .. } if *tag == RECORD_TAG)
        })
        .count();
    assert_eq!(chest_records, 0);
}

#[test]
fn deaths_and_the_world_share_the_claims_without_taking_each_others() {
    let mut run = Run::new(1, 24);
    // In one step a guest's game kills an actor and picks a lock: the lock goes to the world,
    // the death to the driver's own claims.
    run.guests[0].0.kill(CHET);
    kit::set_locked_as_played(&mut run.guests[0].0, DOOR, false);
    run.frames(10);
    assert_eq!(run.host.dead(CHET), Some(true));
    assert!(!world::lock(&mut run.host.e, DOOR).unwrap().locked());
    assert_eq!((run.hd.claims_enacted, run.hd.claims_dropped), (1, 0));
    assert_eq!(run.hw().enacted, 1);
}

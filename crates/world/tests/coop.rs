//! Co-op on the world crate: a host and guests, each a `GameState`, joined by
//! coop-core's chaos transport. The host plays through the real `Runner`
//! paths; guests only receive. No viewer, no game data (testdata quests).

use coop_core::sim_test::Sim;
use coop_core::sync::GuestPhase;
use coop_core::transport::ChaosProfile;
use esm::{ActivePlugins, FormId, LoadOrder};
use testdata::quest_ids::*;
use world::coop::apply::{Ctx, Opts};
use world::coop::{Authority, Domain, GuestBridge, HostBridge, WorldBackend};
use world::dialogue::PLAYER_REF;
use world::scripting::{Event, GameState, Runner, ScriptCache};

struct Rig<'a> {
    order: &'a LoadOrder,
    scripts: &'a ScriptCache,
    sim: Sim,
    host: GameState,
    guests: Vec<GameState>,
    hb: HostBridge,
    gbs: Vec<GuestBridge>,
}

impl<'a> Rig<'a> {
    fn new(
        order: &'a LoadOrder,
        scripts: &'a ScriptCache,
        n: u8,
        profile: &str,
        seed: u64,
    ) -> Rig<'a> {
        let mut host = GameState::new(order);
        host.set_authority(Authority::Host);
        let guests = (0..n)
            .map(|_| {
                let mut g = GameState::new(order);
                g.set_authority(Authority::Guest);
                g
            })
            .collect();
        let mut hb = HostBridge::default();
        hb.backstop_every = 1;
        Rig {
            order,
            scripts,
            sim: Sim::new(n, ChaosProfile::by_name(profile).unwrap(), seed),
            host,
            guests,
            hb,
            gbs: (0..n).map(|_| GuestBridge::default()).collect(),
        }
    }

    fn opts(&self) -> Opts<'a> {
        Opts {
            present: true,
            ctx: Some(Ctx {
                order: self.order,
                scripts: self.scripts,
            }),
        }
    }

    fn tick(&mut self) {
        self.hb.collect(&mut self.host, &mut self.sim.host);
        self.sim.step();
        for i in 0..self.guests.len() {
            if self.sim.guests[i].sync.phase() == GuestPhase::Active {
                let opts = self.opts();
                let store = &self.sim.guests[i].sync.store;
                self.gbs[i].sync(&mut self.guests[i], store, false, opts);
            }
        }
    }

    fn ticks(&mut self, n: u32) {
        for _ in 0..n {
            self.tick();
        }
    }

    fn same_hashes(&self, only: &[usize]) -> bool {
        let h = self.host.shard_hashes();
        only.iter().all(|&i| {
            self.sim.guests[i].sync.phase() == GuestPhase::Active
                && self.guests[i].shard_hashes() == h
                && self.sim.guests[i].sync.store.rev() == self.sim.host.store.rev()
        })
    }

    fn settle(&mut self, only: &[usize]) -> bool {
        for _ in 0..8000 {
            self.tick();
            if self.same_hashes(only) {
                // one more round proves it is stable
                self.ticks(10);
                if self.same_hashes(only) {
                    return true;
                }
            }
        }
        false
    }

    /// The host plays: a run of the quest through the real runner.
    fn host_plays_quest(&mut self) {
        let mut r = Runner::new(self.order, self.scripts, &mut self.host);
        assert!(r.set_stage(FormId(QUEST), 10));
        for _ in 0..4 {
            r.update(1.0);
        }
    }
}

fn order(tag: &str) -> (testdata::TempData, LoadOrder) {
    let data = testdata::quests(tag);
    let order = LoadOrder::from_data_dir(data.path(), &ActivePlugins::OfficialOnly).unwrap();
    (data, order)
}

#[test]
fn coop_single_player_is_unchanged_and_records_nothing() {
    let (_d, order) = order("coop-single");
    let scripts = ScriptCache::default();
    let mut state = GameState::new(&order);
    assert_eq!(state.authority(), Authority::Single);
    let mut r = Runner::new(&order, &scripts, &mut state);
    assert!(r.set_stage(FormId(QUEST), 10));
    for _ in 0..4 {
        r.update(1.0);
    }
    assert_eq!(state.stages.get(&FormId(QUEST)), Some(&20));
    assert!(state.coop.out.is_empty());
    assert_eq!(state.coop.stats.effects, 0);
}

#[test]
fn coop_host_and_guests_converge_under_every_profile() {
    for (profile, seed) in [
        ("perfect", 1u64),
        ("bad_wifi", 2),
        ("flaky", 3),
        ("partition", 4),
    ] {
        let (_d, order) = order(&format!("coop-conv-{profile}"));
        let scripts = ScriptCache::default();
        let mut rig = Rig::new(&order, &scripts, 3, profile, seed);
        for i in 0..3 {
            rig.sim.start_guest(i);
        }
        rig.ticks(20);
        rig.host_plays_quest();
        // other host-side changes through game paths
        rig.host
            .move_item(&order, PLAYER_REF, FormId(DOC_REF), FormId(CAPS), 5);
        rig.host.topics.insert(FormId(TOPIC_SECRET));
        rig.host.emit(Domain::Topic, FormId(TOPIC_SECRET));
        assert!(rig.settle(&[0, 1, 2]), "profile {profile} did not settle");
        let h = &rig.host;
        for g in &rig.guests {
            assert_eq!(g.stages.get(&FormId(QUEST)), Some(&20));
            assert_eq!(
                g.globals.get(&FormId(GLOBAL)),
                h.globals.get(&FormId(GLOBAL))
            );
            assert!(g.completed.contains(&FormId(QUEST)));
            assert_eq!(g.item_count(&order, FormId(DOC_REF), FormId(CAPS)), 5);
            assert!(g.topics.contains(&FormId(TOPIC_SECRET)));
            assert_eq!(
                g.disabled.get(&FormId(DOOR_REF)),
                h.disabled.get(&FormId(DOOR_REF))
            );
            assert_eq!(g.objectives, h.objectives);
            assert_eq!(g.stages_done, h.stages_done);
        }
    }
}

#[test]
fn coop_clock_ticks_on_host_only_and_hashes_still_match() {
    let (_d, order) = order("coop-clock");
    let scripts = ScriptCache::default();
    let mut rig = Rig::new(&order, &scripts, 1, "good_wifi", 9);
    rig.sim.start_guest(0);
    for _ in 0..30 {
        Runner::new(&order, &scripts, &mut rig.host).update(0.37);
        Runner::new(&order, &scripts, &mut rig.guests[0]).update(0.37);
        rig.tick();
    }
    assert!(rig.host.global(&order, "GameHour").unwrap() > 10.0);
    assert!(rig.settle(&[0]));
    // the guest clock only follows the host's coarse beacon
    let (gh, hh) = (
        rig.guests[0].global(&order, "GameHour").unwrap(),
        rig.host.global(&order, "GameHour").unwrap(),
    );
    assert!(gh <= hh && hh - gh < 1.0 / 64.0 + 1e-6);
}

#[test]
fn coop_guest_scripts_are_suppressed() {
    let (_d, order) = order("coop-suppress");
    let scripts = ScriptCache::default();
    let mut guest = GameState::new(&order);
    guest.set_authority(Authority::Guest);
    let before = guest.shard_hashes();
    let dice = guest.dice;
    {
        let mut r = Runner::new(&order, &scripts, &mut guest);
        assert!(!r.set_stage(FormId(QUEST), 10));
        for _ in 0..10 {
            r.update(1.0);
        }
        r.run_source("set TestDoorOpen to 9", None, None);
        r.run_source("SetStage TestQuest 20", None, None);
        r.run_event(FormId(DOC_REF), "onactivate", PLAYER_REF);
        r.run_item_scripts();
        r.menu_mode(1);
    }
    assert!(guest.stages.is_empty());
    assert_eq!(guest.shard_hashes(), before);
    assert_eq!(guest.dice, dice, "a guest never advances the dice");
    assert!(guest.coop.stats.suppressed_runs >= 5);
    assert!(guest.events.is_empty());
}

#[test]
fn coop_guest_host_trait_refuses_script_writes() {
    use script::interp::{Host, Locals};
    let (_d, order) = order("coop-hosttrait");
    let scripts = ScriptCache::default();
    let mut guest = GameState::new(&order);
    guest.set_authority(Authority::Guest);
    let mut r = Runner::new(&order, &scripts, &mut guest);
    let mut locals = Locals::default();
    assert!(r.set_var("", "TestGlobal", 99.0, &mut locals));
    let (i, _) = script::function("setstage").unwrap();
    let call = script::Call {
        function: i,
        on: None,
        args: vec![
            script::Arg::Word("TestQuest".into()),
            script::Arg::Number(10.0),
        ],
    };
    assert_eq!(r.call(&call, None, &mut locals), Some(0.0));
    assert!(guest.stages.is_empty());
    assert!(guest.coop.stats.suppressed_calls >= 2);
}

#[test]
fn coop_late_join_gets_a_snapshot_without_announcements() {
    let (_d, order) = order("coop-late");
    let scripts = ScriptCache::default();
    let mut rig = Rig::new(&order, &scripts, 2, "bad_wifi", 11);
    rig.sim.start_guest(0);
    rig.ticks(10);
    rig.host_plays_quest();
    assert!(rig.settle(&[0]));
    rig.guests[0].events.clear();
    rig.sim.start_guest(1);
    assert!(rig.settle(&[0, 1]));
    assert_eq!(rig.sim.guests[1].sync.snapshots_loaded, 1);
    assert_eq!(rig.guests[1].stages.get(&FormId(QUEST)), Some(&20));
    assert!(
        rig.guests[1].events.is_empty(),
        "join snapshot must not replay the journal"
    );
    assert!(rig.guests[0].events.is_empty());
}

#[test]
fn coop_live_changes_show_local_presentation_on_the_guest() {
    let (_d, order) = order("coop-present");
    let scripts = ScriptCache::default();
    let mut rig = Rig::new(&order, &scripts, 1, "perfect", 5);
    rig.sim.start_guest(0);
    rig.ticks(30);
    assert!(rig.settle(&[0]));
    rig.host_plays_quest();
    assert!(rig.settle(&[0]));
    let ev = &rig.guests[0].events;
    assert!(ev.contains(&Event::Journal {
        quest: FormId(QUEST),
        text: "Done.".into()
    }));
    assert!(ev.iter().any(|e| matches!(e, Event::QuestText(_))));
    assert!(ev.iter().any(|e| matches!(e, Event::Objective { .. })));
    // and the guest ran nothing itself
    assert!(rig.guests[0].coop.stats.suppressed_runs == 0);
}

#[test]
fn coop_guest_state_drift_is_detected_and_repaired() {
    let (_d, order) = order("coop-drift");
    let scripts = ScriptCache::default();
    let mut rig = Rig::new(&order, &scripts, 1, "good_wifi", 17);
    rig.sim.start_guest(0);
    rig.host_plays_quest();
    assert!(rig.settle(&[0]));
    // a stray local writer changes shared state on the guest
    rig.guests[0].stages.insert(FormId(QUEST), 99);
    rig.guests[0].topics.insert(FormId(TOPIC_ABOUT));
    assert!(!rig.same_hashes(&[0]));
    let opts = rig.opts();
    let store = &rig.sim.guests[0].sync.store;
    let rep = rig.gbs[0].sync(&mut rig.guests[0], store, true, opts);
    assert!(rep.is_some());
    assert_eq!(rig.gbs[0].drift_repairs, 1);
    assert!(rig.same_hashes(&[0]));
    assert_eq!(rig.guests[0].stages.get(&FormId(QUEST)), Some(&20));
}

#[test]
fn coop_store_desync_is_detected_and_repaired_through_to_the_state() {
    let (_d, order) = order("coop-store-desync");
    let scripts = ScriptCache::default();
    let mut rig = Rig::new(&order, &scripts, 2, "good_wifi", 23);
    rig.sim.start_guest(0);
    rig.sim.start_guest(1);
    rig.host_plays_quest();
    assert!(rig.settle(&[0, 1]));
    // corrupt guest 0's replica (same revision, wrong value)
    let quest = coop_core::ids::FormId(QUEST);
    let shard = coop_core::ids::shard_of(quest);
    let entries = rig.sim.guests[0].sync.store.entries_in_shards(&[shard]);
    let fixed: Vec<_> = entries
        .into_iter()
        .map(|mut e| {
            if e.form == quest {
                if let coop_core::schema::Field::QuestStage { stage } = e.field {
                    e.field = coop_core::schema::Field::QuestStage { stage: stage + 1 };
                }
            }
            e
        })
        .collect();
    rig.sim.guests[0].sync.store.replace_shards(&[shard], fixed);
    assert!(rig.settle(&[0, 1]));
    assert!(rig.sim.guests[0].sync.desyncs_detected >= 1);
    assert!(rig.sim.guests[0].sync.repairs_applied >= 1);
    assert_eq!(rig.guests[0].stages.get(&FormId(QUEST)), Some(&20));
}

#[test]
fn coop_backstop_reports_writers_that_emit_no_effect() {
    let (_d, order) = order("coop-backstop");
    let scripts = ScriptCache::default();
    let mut rig = Rig::new(&order, &scripts, 1, "perfect", 3);
    rig.sim.start_guest(0);
    rig.ticks(5);
    rig.host_plays_quest();
    let covered = rig.hb.collect(&mut rig.host, &mut rig.sim.host);
    assert!(covered.written > 0);
    assert!(
        covered.uncovered.is_empty(),
        "semantic sites cover quest play: {:?}",
        covered.uncovered
    );
    // a raw writer that forgot to emit
    rig.host.reputations.insert(FormId(0x77), (3.0, 1.0));
    rig.host.map_markers.insert(FormId(0x78));
    let r = rig.hb.collect(&mut rig.host, &mut rig.sim.host);
    assert_eq!(r.uncovered.get(&Domain::Reputation), Some(&1));
    assert_eq!(r.uncovered.get(&Domain::MapMarker), Some(&1));
    // it still replicates (the backstop catches it)
    assert!(rig.settle(&[0]));
    assert!(rig.guests[0].map_markers.contains(&FormId(0x78)));
    assert_eq!(
        rig.guests[0].reputations.get(&FormId(0x77)),
        Some(&(3.0, 1.0))
    );
}

#[test]
fn coop_removals_replicate_as_tombstones() {
    let (_d, order) = order("coop-tombstone");
    let scripts = ScriptCache::default();
    let mut rig = Rig::new(&order, &scripts, 1, "perfect", 8);
    rig.sim.start_guest(0);
    rig.host.items.insert((FormId(0x50), FormId(CAPS)), 4);
    rig.host.destroyed.insert(FormId(0x51));
    assert!(rig.settle(&[0]));
    assert_eq!(
        rig.guests[0].items.get(&(FormId(0x50), FormId(CAPS))),
        Some(&4)
    );
    rig.host.items.remove(&(FormId(0x50), FormId(CAPS)));
    rig.host.destroyed.clear();
    assert!(rig.settle(&[0]));
    assert!(!rig.guests[0]
        .items
        .contains_key(&(FormId(0x50), FormId(CAPS))));
    assert!(rig.guests[0].destroyed.is_empty());
}

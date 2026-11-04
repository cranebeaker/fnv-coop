//! Per-player shadows and campaign policy enforcement: a host and two guests
//! over coop-core's simulated transport (testdata quests, no game data).

use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::Arc;

use coop_core::ids::PlayerId;
use coop_core::sim_test::Sim;
use coop_core::sync::GuestPhase;
use coop_core::transport::ChaosProfile;
use coop_policy::{Decision, PolicyTable, Reject};
use esm::{ActivePlugins, FormId, LoadOrder};
use testdata::quest_ids::*;
use world::coop::apply::{Ctx, Opts};
use world::coop::policy_gate::{reward_id, Grant};
use world::coop::{Authority, GuestBridge, HostBridge, ShadowPlan, WorldBackend};
use world::dialogue::PLAYER_REF;
use world::scripting::{GameState, Runner, ScriptCache};

fn table(row_extra: &str) -> Arc<PolicyTable> {
    let json = format!(
        r#"{{"schema":2,
"policy":{{"death":{{"mode":"downed","revive_s":60,"all_down":"reload-host-save"}},"late_join":{{"during_scene":"wait","after_scene":"warp-to-zone"}},"barrier_timeout_s":120,"tether_timeout_s":120,"follower_cap":{{"humanoid":1,"critter":1}},"party_cap":4}},
"shadow_classes":{{"skills":"per_player","perks":"per_player","followers":"per_player","quest_var":"per_player","karma":"shared","reputation":"shared"}},
"quests":[{{"edid":"TQ","id":"0x{QUEST:08X}","local":"0x{:08X}","plugin":"t.esm","tier":"T1","line":"t","status":"confirmed","advance":"host"{row_extra}}}]}}"#,
        QUEST & 0x00FF_FFFF
    );
    Arc::new(PolicyTable::from_json(&json).expect("table"))
}

fn order(tag: &str) -> (testdata::TempData, LoadOrder) {
    let data = testdata::quests(tag);
    let order = LoadOrder::from_data_dir(data.path(), &ActivePlugins::OfficialOnly).unwrap();
    (data, order)
}

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
    fn new(order: &'a LoadOrder, scripts: &'a ScriptCache, t: Arc<PolicyTable>) -> Rig<'a> {
        let n = 2u8;
        let mut host = GameState::new(order);
        host.set_authority(Authority::Host);
        host.enable_coop_policy(t.clone(), 0);
        for s in 1..=n {
            host.register_player(s);
            host.with_player(s, |g| g.stock(order, PLAYER_REF));
        }
        host.coop
            .gate
            .as_mut()
            .unwrap()
            .set_present((0..=n).map(PlayerId));
        let guests = (1..=n)
            .map(|s| {
                let mut g = GameState::new(order);
                g.set_authority(Authority::Guest);
                g.enable_shadows(ShadowPlan::from_policy(&t), s);
                g
            })
            .collect();
        let mut hb = HostBridge::default();
        hb.backstop_every = 1;
        let mut rig = Rig {
            order,
            scripts,
            sim: Sim::new(n, ChaosProfile::by_name("perfect").unwrap(), 7),
            host,
            guests,
            hb,
            gbs: (0..n).map(|_| GuestBridge::default()).collect(),
        };
        for i in 0..n as usize {
            rig.sim.start_guest(i);
        }
        rig.ticks(20);
        rig
    }

    fn tick(&mut self) {
        self.hb.collect(&mut self.host, &mut self.sim.host);
        self.sim.step();
        for i in 0..self.guests.len() {
            if self.sim.guests[i].sync.phase() == GuestPhase::Active {
                let opts = Opts {
                    present: true,
                    ctx: Some(Ctx {
                        order: self.order,
                        scripts: self.scripts,
                    }),
                };
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

    fn settle(&mut self) -> bool {
        for _ in 0..4000 {
            self.tick();
            let h = self.host.shard_hashes();
            let ok = (0..self.guests.len()).all(|i| {
                self.sim.guests[i].sync.phase() == GuestPhase::Active
                    && self.guests[i].shard_hashes() == h
                    && self.sim.guests[i].sync.store.rev() == self.sim.host.store.rev()
            });
            if ok {
                return true;
            }
        }
        false
    }

    fn gate<R>(&mut self, f: impl FnOnce(&mut world::coop::PolicyGate, &mut GameState) -> R) -> R {
        let mut g = self.host.coop.gate.take().unwrap();
        let r = f(&mut g, &mut self.host);
        self.host.coop.gate = Some(g);
        r
    }
}

fn held(gs: &GameState, slot: u8, item: u32) -> i32 {
    gs.shadow_of(slot)
        .unwrap()
        .items
        .get(&FormId(item))
        .copied()
        .unwrap_or(0)
}

#[test]
fn coop_shadow_inventories_diverge_while_shared_state_converges() {
    let (_d, order) = order("coop-shadow-inv");
    let scripts = ScriptCache::default();
    let mut rig = Rig::new(&order, &scripts, table(""));
    // Player 1 picks up caps through the shadow swap; player 2 does not.
    rig.host.with_player(1, |g| {
        g.stock(&order, PLAYER_REF);
        *g.items.entry((PLAYER_REF, FormId(CAPS))).or_insert(0) += 7;
        g.skill_points.insert(32, 5);
    });
    // Shared story state changes on the host.
    {
        let mut r = Runner::new(&order, &scripts, &mut rig.host);
        assert!(r.set_stage(FormId(QUEST), 10));
        for _ in 0..4 {
            r.update(1.0);
        }
    }
    rig.host.topics.insert(FormId(TOPIC_SECRET));
    assert!(rig.settle(), "did not converge");
    let base = held(&rig.host, 2, CAPS);
    assert_eq!(held(&rig.host, 1, CAPS), base + 7);
    assert!(
        held(&rig.host, 0, CAPS) > base,
        "the quest script paid the host player only"
    );
    // Each guest holds its own rows live and mirrors the others.
    assert_eq!(
        rig.guests[0].items.get(&(PLAYER_REF, FormId(CAPS))),
        Some(&(base + 7))
    );
    assert_eq!(rig.guests[0].skill_points.get(&32), Some(&5));
    assert_eq!(
        rig.guests[1]
            .items
            .get(&(PLAYER_REF, FormId(CAPS)))
            .copied()
            .unwrap_or(0),
        base
    );
    assert!(rig.guests[1].skill_points.is_empty());
    assert_eq!(
        held(&rig.guests[1], 1, CAPS),
        base + 7,
        "guest 2 mirrors guest 1"
    );
    for g in &rig.guests {
        assert_eq!(g.stages.get(&FormId(QUEST)), Some(&20));
        assert!(g.topics.contains(&FormId(TOPIC_SECRET)));
    }
    assert!(
        rig.host.skill_points.is_empty(),
        "host player's skills are untouched"
    );
}

#[test]
fn coop_policy_reward_reaches_every_player_exactly_once() {
    let (_d, order) = order("coop-shadow-reward");
    let scripts = ScriptCache::default();
    let mut rig = Rig::new(&order, &scripts, table(""));
    let before: Vec<i32> = (0..3).map(|s| held(&rig.host, s, CAPS)).collect();
    let id = reward_id(FormId(QUEST), 20, 0);
    let grant = Grant::Item {
        item: FormId(CAPS),
        count: 10,
    };
    let out = rig.gate(|g, gs| g.route_reward(gs, &order, FormId(QUEST), id, grant.clone()));
    assert_eq!(out.applied.len(), 3);
    let again = rig.gate(|g, gs| g.route_reward(gs, &order, FormId(QUEST), id, grant.clone()));
    assert!(matches!(
        again.decision,
        Decision::Reject {
            reason: Reject::AlreadyGranted
        }
    ));
    assert!(again.applied.is_empty());
    assert!(rig.settle());
    for s in 0..3u8 {
        assert_eq!(
            held(&rig.host, s, CAPS),
            before[s as usize] + 10,
            "slot {s}"
        );
    }
    assert_eq!(
        rig.guests[1].items.get(&(PLAYER_REF, FormId(CAPS))),
        Some(&(before[2] + 10))
    );
}

#[test]
fn coop_policy_split_reward_divides_between_players() {
    let (_d, order) = order("coop-shadow-split");
    let scripts = ScriptCache::default();
    let mut rig = Rig::new(&order, &scripts, table(r#","reward":"split""#));
    let before: Vec<i32> = (0..3).map(|s| held(&rig.host, s, CAPS)).collect();
    let id = reward_id(FormId(QUEST), 20, 1);
    rig.gate(|g, gs| {
        g.route_reward(
            gs,
            &order,
            FormId(QUEST),
            id,
            Grant::Item {
                item: FormId(CAPS),
                count: 10,
            },
        )
    });
    let got: Vec<i32> = (0..3)
        .map(|s| held(&rig.host, s, CAPS) - before[s as usize])
        .collect();
    assert_eq!(got, vec![4, 3, 3]);
}

#[test]
fn coop_policy_barrier_stage_waits_for_every_present_player() {
    let (_d, order) = order("coop-shadow-barrier");
    let scripts = ScriptCache::default();
    let mut rig = Rig::new(&order, &scripts, table(r#","barrier":{"stages":[10]}"#));
    {
        let mut r = Runner::new(&order, &scripts, &mut rig.host);
        assert!(!r.set_stage(FormId(QUEST), 10), "held at the barrier");
    }
    assert!(!rig.host.stages.contains_key(&FormId(QUEST)));
    for slot in [0u8, 1] {
        let released = rig.gate(|g, gs| {
            g.barrier_pass(FormId(QUEST), slot);
            g.poll(gs)
        });
        assert!(released.is_empty(), "slot {slot} passed, one still missing");
    }
    let released = rig.gate(|g, gs| {
        g.barrier_pass(FormId(QUEST), 2);
        g.poll(gs)
    });
    assert_eq!(released.len(), 1);
    {
        let mut r = Runner::new(&order, &scripts, &mut rig.host);
        assert!(r.set_stage(FormId(QUEST), released[0].stage.stage));
        for _ in 0..4 {
            r.update(1.0);
        }
    }
    assert_eq!(rig.host.stages.get(&FormId(QUEST)), Some(&20));
    assert!(rig.settle());
    assert_eq!(rig.guests[0].stages.get(&FormId(QUEST)), Some(&20));
}

#[test]
fn coop_policy_rejects_host_only_events_from_a_guest() {
    let (_d, order) = order("coop-shadow-reject");
    let scripts = ScriptCache::default();
    let mut rig = Rig::new(&order, &scripts, table(""));
    let d = rig.gate(|g, gs| g.intent_set_stage(gs, 1, FormId(QUEST), 10));
    assert_eq!(
        d,
        Decision::Reject {
            reason: Reject::NotHost
        }
    );
    let ok = rig.gate(|g, gs| g.guest_script_allowed(gs, FormId(QUEST), FormId(QUEST_SCRIPT), 1));
    assert!(!ok);
    // A player who is not counted cannot act at all.
    rig.gate(|g, _| g.set_present([PlayerId(0)]));
    let d = rig.gate(|g, gs| g.intent_set_stage(gs, 2, FormId(QUEST), 10));
    assert_eq!(
        d,
        Decision::Reject {
            reason: Reject::NotPresent
        }
    );
}

#[test]
fn coop_shadow_swap_leaves_hash_unchanged_and_is_panic_safe() {
    let (_d, order) = order("coop-shadow-swap");
    let mut gs = GameState::new(&order);
    gs.set_authority(Authority::Host);
    gs.enable_shadows(ShadowPlan::all(), 0);
    gs.register_player(1);
    let before = gs.shard_hashes();
    let items = gs.items.clone();
    {
        let g = gs.as_player(1);
        assert_eq!(g.active_slot(), 1);
    }
    assert_eq!(gs.active_slot(), 0);
    assert_eq!(gs.shard_hashes(), before);
    assert_eq!(gs.items, items);

    // Karma is shared: not swapped.
    gs.actor_values
        .insert((PLAYER_REF, world::reputation::KARMA), 40.0);
    gs.with_player(1, |g| {
        assert_eq!(
            g.actor_values.get(&(PLAYER_REF, world::reputation::KARMA)),
            Some(&40.0)
        );
        g.actor_values
            .insert((PLAYER_REF, world::reputation::KARMA), 45.0);
    });
    assert_eq!(
        gs.actor_values.get(&(PLAYER_REF, world::reputation::KARMA)),
        Some(&45.0)
    );

    // A panic inside the scope still restores the previous owner and keeps
    // the work done for the other player.
    gs.items.insert((PLAYER_REF, FormId(CAPS)), 11);
    let r = catch_unwind(AssertUnwindSafe(|| {
        let mut g = gs.as_player(1);
        g.items.insert((PLAYER_REF, FormId(CAPS)), 99);
        panic!("script blew up");
    }));
    assert!(r.is_err());
    assert_eq!(gs.active_slot(), 0);
    assert_eq!(gs.items.get(&(PLAYER_REF, FormId(CAPS))), Some(&11));
    assert_eq!(held(&gs, 1, CAPS), 99);
}

/// A script variable the host sets for player 1 shows up in player 1's own live rows on its
/// machine and in the host's copy of slot 1, and nowhere else (not the host's own rows, not
/// player 2's, not the shared hash).
#[test]
fn a_per_player_quest_variable_reaches_its_owner_and_nobody_else() {
    let (_d, order) = order("coop-shadow-var");
    let scripts = ScriptCache::default();
    let t = table(r#","shadow":{"scope":"test","vars":["bRunTimer"]}"#);
    let mut rig = Rig::new(&order, &scripts, t);
    let base = {
        let s = world::scripting::script_of(&order, FormId(QUEST)).expect("quest script");
        script::interp::Locals::new(&scripts.script(&order, s).expect("script"))
    };
    // the host's script run for player 1 (the host's quest variables exist; the guests' do not)
    rig.host.variables.insert(FormId(QUEST), base);
    rig.host.with_player(1, |g| {
        assert!(g
            .variables
            .get_mut(&FormId(QUEST))
            .unwrap()
            .set("bRunTimer", 1.0));
    });
    assert!(rig.settle(), "shared state converges");
    let var = |gs: &GameState| {
        gs.variables
            .get(&FormId(QUEST))
            .and_then(|l| l.get("bRunTimer"))
            .unwrap_or(0.0)
    };
    assert_eq!(
        var(&rig.guests[0]),
        1.0,
        "player 1's own rows hold the variable"
    );
    assert_eq!(var(&rig.guests[1]), 0.0, "player 2 does not");
    assert_eq!(var(&rig.host), 0.0, "the host's own rows do not");
    assert_eq!(
        rig.host.shadow_of(1).unwrap().quest_vars.len(),
        1,
        "the host keeps a copy for slot 1"
    );
    assert_eq!(rig.guests[0].shadow_of(1), rig.host.shadow_of(1));
    assert!(
        rig.guests[1].shadow_of(1).unwrap().quest_vars.len() == 1,
        "everyone holds every player's copy"
    );
    // the variable is not in the shared rows
    assert!(!world::coop::export::export(&rig.host)
        .values()
        .any(|f| matches!(f, coop_core::schema::Field::ScriptVar { bits, .. } if *bits != 0)));
}

/// A player's health changes on that player's own machine (a hurt from the host, a blow of an
/// enemy it owns). Before, the guest's own shadow carried the damage into the shared hash (a
/// desync), and the host's next write to that shadow reset it (the guest healed).
#[test]
fn a_guest_keeps_its_own_health_damage_and_the_hash_agrees() {
    let (_d, order) = order("coop-own-damage");
    let scripts = ScriptCache::default();
    let mut rig = Rig::new(&order, &scripts, table(""));
    assert!(rig.settle(), "shared state converges");
    // a gecko the guest owns bites the guest's player on the guest's machine
    rig.guests[0].damage.insert(PLAYER_REF, 5.5);
    assert!(rig.settle(), "the guest's own damage is not shared state");
    // the host writes to the guest's shadow (an item for player 1)
    rig.host
        .with_player(1, |g| g.items.insert((PLAYER_REF, FormId(CAPS)), 7));
    assert!(rig.settle(), "converges after the host's write");
    assert_eq!(
        held(&rig.guests[0], 1, CAPS),
        7,
        "the item reached the guest"
    );
    assert_eq!(
        rig.guests[0].damage.get(&PLAYER_REF),
        Some(&5.5),
        "the host's write does not heal the guest"
    );
}

/// Each machine runs its own copy of a per-player quest's script, so the script's variables
/// stay on that machine. Before, the host shared them: the guest's copy of a timer was
/// overwritten with the host's, and the next guest frame wrote its own value again (a desync).
#[test]
fn a_per_player_quest_keeps_its_script_variables_on_each_machine() {
    let (_d, order) = order("coop-own-quest-var");
    let scripts = ScriptCache::default();
    let t = table(r#","scope":"per_player""#);
    assert!(ShadowPlan::from_policy(&t).quests.contains(&FormId(QUEST)));
    let mut rig = Rig::new(&order, &scripts, t);
    let base = {
        let s = world::scripting::script_of(&order, FormId(QUEST)).expect("quest script");
        script::interp::Locals::new(&scripts.script(&order, s).expect("script"))
    };
    let timer = |gs: &mut GameState, v: f64| {
        gs.variables.entry(FormId(QUEST)).or_insert(base.clone());
        assert!(gs
            .variables
            .get_mut(&FormId(QUEST))
            .unwrap()
            .set("fTimer", v));
    };
    timer(&mut rig.host, 10.93);
    timer(&mut rig.guests[0], 9.93);
    assert!(rig.settle(), "shared state converges");
    let var = |gs: &GameState| {
        gs.variables
            .get(&FormId(QUEST))
            .and_then(|l| l.get("fTimer"))
            .unwrap_or(0.0)
    };
    assert_eq!(var(&rig.host), 10.93);
    assert_eq!(
        var(&rig.guests[0]),
        9.93,
        "the host's timer does not reach the guest"
    );
    assert_eq!(var(&rig.guests[1]), 0.0);
    let q = world::coop::export::cf(FormId(QUEST));
    assert!(
        !world::coop::export::export(&rig.host)
            .iter()
            .any(|(k, f)| k.form == q && matches!(f, coop_core::schema::Field::ScriptVar { .. })),
        "the quest's variables are not in the shared rows"
    );
}

#[test]
fn coop_shadow_blob_round_trips() {
    let (_d, order) = order("coop-shadow-codec");
    let mut gs = GameState::new(&order);
    gs.enable_shadows(ShadowPlan::all(), 0);
    gs.actor_values.insert((PLAYER_REF, 24), 123.5);
    gs.perks.insert(FormId(0x77));
    gs.perk_ranks.insert(FormId(0x77), 2);
    gs.player_name = Some("Courier".into());
    gs.player_position = Some([1.5, -2.0, 3.25]);
    gs.hotkeys[3] = Some(FormId(9));
    let s = gs.shadow_of(0).unwrap();
    let back = world::coop::shadow::decode(&world::coop::shadow::encode(&s)).unwrap();
    assert_eq!(back.actor_values, s.actor_values);
    assert_eq!(back.perks, s.perks);
    assert_eq!(back.name, s.name);
    assert_eq!(back.position, s.position);
    assert_eq!(back.hotkeys, s.hotkeys);
    assert_eq!(back.items, s.items);
    assert!(world::coop::shadow::decode(&[1, 2, 3]).is_none());
}

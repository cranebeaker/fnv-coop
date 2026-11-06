//! Campaign gaps in the running world: a host and two guests over coop-core's
//! simulated transport (generated test worlds, the policy table built per
//! test). One test per gap, each with a mutation twin inside it (the same
//! steps without the cause must not give the effect).

use std::sync::Arc;

use coop_core::ids::PlayerId;
use coop_core::sim_test::Sim;
use coop_core::sync::GuestPhase;
use coop_core::transport::ChaosProfile;
use coop_policy::{CompanionOp, PolicyTable};
use esm::{ActivePlugins, FormId, LoadOrder};
use testdata::fighting::ids::*;
use world::coop::apply::{Ctx, Opts};
use world::coop::campaign::{
    attribution, consent, downed, ffguard, owners, party, producers, storage,
};
use world::coop::live::{LiveGuestAdapter, LiveHostAdapter};
use world::coop::{Authority, GuestBridge, HostBridge, ShadowPlan};
use world::dialogue::PLAYER_REF;
use world::scripting::{GameState, ScriptCache, Value};

fn table(rows: &str) -> Arc<PolicyTable> {
    let json = format!(
        r#"{{"schema":2,
"policy":{{"death":{{"mode":"downed","revive_s":30,"all_down":"respawn-at-checkpoint"}},"late_join":{{"during_scene":"wait","after_scene":"warp-to-zone"}},"barrier_timeout_s":120,"tether_timeout_s":120,"follower_cap":{{"humanoid":1,"critter":1}},"party_cap":4}},
"shadow_classes":{{"skills":"per_player","perks":"per_player","followers":"per_player","karma":"shared","reputation":"shared","collar":"per_player","quest_var":"per_player"}},
"quests":[{rows}]}}"#
    );
    Arc::new(PolicyTable::from_json(&json).expect("table"))
}

fn row(edid: &str, id: u32, extra: &str) -> String {
    let local = id & 0x00FF_FFFF;
    format!(
        r#"{{"edid":"{edid}","id":"0x{id:08X}","local":"0x{local:08X}","plugin":"t.esm","tier":"T1","line":"t","status":"confirmed","advance":"initiator"{extra}}}"#
    )
}

fn order(tag: &str) -> (testdata::TempData, LoadOrder) {
    let data = testdata::fighting::fighting(tag);
    let order = LoadOrder::from_data_dir(data.path(), &ActivePlugins::OfficialOnly).unwrap();
    (data, order)
}

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
    /// Guest hit requests wait for the friendly-fire guard on the host, as
    /// the session does (`LiveHostAdapter::hold_guest_hit`).
    hold_hits: bool,
}

impl<'a> Rig<'a> {
    fn new(
        order: &'a LoadOrder,
        scripts: &'a ScriptCache,
        t: Arc<PolicyTable>,
        profile: &str,
        seed: u64,
    ) -> Rig<'a> {
        let mut host = GameState::new(order);
        host.set_authority(Authority::Host);
        host.enable_coop_policy(t.clone(), 0);
        for s in 1..=2u8 {
            host.register_player(s);
            host.with_player(s, |g| g.stock(order, PLAYER_REF));
        }
        host.coop
            .gate
            .as_mut()
            .unwrap()
            .set_present((0..=2).map(PlayerId));
        let guests: Vec<GameState> = (1..=2u8)
            .map(|s| {
                let mut g = GameState::new(order);
                g.set_authority(Authority::Guest);
                g.enable_shadows(ShadowPlan::from_policy(&t), s);
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
            hold_hits: false,
        };
        let ok = r.sim.run_until(120_000, |s| {
            s.guests
                .iter()
                .all(|g| g.sync.phase() == GuestPhase::Active)
        });
        assert!(ok, "guests never became active");
        for g in std::iter::once(&mut r.host).chain(r.guests.iter_mut()) {
            g.player_cell = Some(FormId(CELL));
            g.player_position = Some([900.0, 1000.0, 5000.0]);
        }
        let cx = r.cx();
        r.ha.track(
            cx,
            &r.host,
            &mut r.sim.host.live,
            &[FormId(GECKO_REF), FormId(GUARD_REF)],
        );
        r
    }

    fn cx(&self) -> Ctx<'a> {
        Ctx {
            order: self.order,
            scripts: self.scripts,
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
                let msgs = if self.hold_hits {
                    let from = PlayerId(i as u8 + 1);
                    msgs.into_iter()
                        .filter_map(|m| self.ha.hold_guest_hit(from, m))
                        .collect()
                } else {
                    msgs
                };
                self.sim.guest_emit(i, msgs);
            }
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
}

fn xp_of(gs: &GameState, slot: u8) -> f64 {
    gs.shadow_of(slot)
        .and_then(|s| s.actor_values.get(&world::experience::XP).copied())
        .unwrap_or(0.0)
}

mod kills {
    use super::*;

    fn run(tag: &str, profile: &str, seed: u64, who: &[usize]) -> (f64, f64, f64, usize) {
        let (_d, order) = order(tag);
        let scripts = ScriptCache::default();
        let mut r = Rig::new(&order, &scripts, table(""), profile, seed);
        let victim = FormId(GECKO_REF);
        r.run_ms(2_000);
        // everyone but the last chips in, then the last one finishes it
        for (n, &i) in who.iter().enumerate() {
            let last = n + 1 == who.len();
            world::combat::hurt(
                &order,
                &mut r.guests[i],
                victim,
                if last { 100000.0 } else { 3.0 },
                PLAYER_REF,
            );
            if !last {
                r.run_ms(2_000);
            }
        }
        let killed = r.until(60_000, |r| !who.is_empty() && r.host.dead.contains(&victim));
        r.run_ms(3_000);
        assert_eq!(killed, !who.is_empty());
        (
            xp_of(&r.host, 0),
            xp_of(&r.host, 1),
            xp_of(&r.host, 2),
            r.host.coop.campaign.paid.len(),
        )
    }

    #[test]
    fn a_guest_kill_pays_xp_to_that_guests_slot_only() {
        for (profile, seed) in [("perfect", 5u64), ("bad_wifi", 6)] {
            let (host, g1, g2, paid) = run(&format!("camp-kill1-{profile}"), profile, seed, &[0]);
            assert!(g1 > 0.0, "{profile}: guest 1 earned xp ({g1})");
            assert_eq!(host, 0.0, "{profile}: the host earned nothing");
            assert_eq!(g2, 0.0, "{profile}: guest 2 earned nothing");
            assert_eq!(paid, 1, "{profile}: paid exactly once");
        }
    }

    #[test]
    fn mutation_twin_nobody_hits_nobody_is_paid() {
        let (host, g1, g2, paid) = run("camp-kill-twin", "perfect", 7, &[]);
        assert_eq!((host, g1, g2, paid), (0.0, 0.0, 0.0, 0));
    }

    #[test]
    fn every_contributor_is_paid_once() {
        let (_h, g1, g2, paid) = run("camp-kill-both", "perfect", 8, &[0, 1]);
        assert!(g1 > 0.0 && g2 > 0.0, "both contributors paid ({g1}, {g2})");
        assert_eq!(paid, 2);
    }

    #[test]
    fn a_host_kill_is_paid_by_slot_not_twice() {
        let (_d, order) = order("camp-kill-host");
        let scripts = ScriptCache::default();
        let mut r = Rig::new(&order, &scripts, table(""), "perfect", 9);
        r.run_ms(2_000);
        let before = xp_of(&r.host, 0);
        world::combat::hurt(&order, &mut r.host, FormId(GECKO_REF), 100000.0, PLAYER_REF);
        assert!(r.until(30_000, |r| r.host.coop.campaign.paid.len() == 1));
        r.run_ms(2_000);
        assert_eq!(r.host.coop.campaign.paid.len(), 1, "one payment");
        assert!(xp_of(&r.host, 0) > before);
        assert_eq!(xp_of(&r.host, 1), 0.0);
    }

    #[test]
    fn a_guest_kill_moves_karma_once_and_counts_the_kill() {
        let (_d, order) = order("camp-kill-stats");
        let scripts = ScriptCache::default();
        let mut r = Rig::new(&order, &scripts, table(""), "perfect", 10);
        r.run_ms(2_000);
        world::combat::hurt(
            &order,
            &mut r.guests[0],
            FormId(GECKO_REF),
            100000.0,
            PLAYER_REF,
        );
        assert!(r.until(60_000, |r| r.host.dead.contains(&FormId(GECKO_REF))));
        r.run_ms(2_000);
        let killed = r
            .host
            .shadow_of(1)
            .unwrap()
            .misc_stats
            .get(&world::stats::TOTAL_THINGS_KILLED)
            .copied()
            .unwrap_or(0);
        assert_eq!(killed, 1, "the kill counts for the guest");
        let host_killed = r
            .host
            .shadow_of(0)
            .unwrap()
            .misc_stats
            .get(&world::stats::TOTAL_THINGS_KILLED)
            .copied()
            .unwrap_or(0);
        assert_eq!(host_killed, 0);
    }
}

mod friendly_fire {
    use super::*;

    #[test]
    fn the_guard_drops_deliberate_hits_after_the_allowed_number() {
        let (_d, order) = order("camp-ff");
        let scripts = ScriptCache::default();
        let ff = format!(
            r#","friendly_fire":{{"actors":["t.esm:{GUARD_REF:X}"],"deliberate_hits":3,"window_s":10}}"#
        );
        let mut r = Rig::new(
            &order,
            &scripts,
            table(&row("FF", 0x777, &ff)),
            "perfect",
            12,
        );
        let guard = FormId(GUARD_REF);
        for _ in 0..6 {
            world::combat::hurt(&order, &mut r.host, guard, 1.0, PLAYER_REF);
        }
        assert_eq!(
            r.host.damage.get(&guard).copied().unwrap_or(0.0),
            3.0,
            "three landed"
        );
        assert_eq!(r.host.coop.campaign.stats.ff_blocked, 3);
        // others are never guarded
        world::combat::hurt(&order, &mut r.host, FormId(GECKO_REF), 1.0, PLAYER_REF);
        assert_eq!(
            r.host
                .damage
                .get(&FormId(GECKO_REF))
                .copied()
                .unwrap_or(0.0),
            1.0
        );
    }

    /// A guest's blows under one point each (its hit carry keeps the part
    /// points) still count as deliberate hits on the host.
    #[test]
    fn a_guests_blows_under_a_point_each_count_for_the_guard() {
        let (_d, order) = order("camp-ff-part");
        let scripts = ScriptCache::default();
        let ff = format!(
            r#","friendly_fire":{{"actors":["t.esm:{GUARD_REF:X}"],"deliberate_hits":3,"window_s":10}}"#
        );
        let mut r = Rig::new(
            &order,
            &scripts,
            table(&row("FF", 0x777, &ff)),
            "perfect",
            14,
        );
        r.hold_hits = true;
        r.run_ms(1_000);
        let guard = FormId(GUARD_REF);
        for _ in 0..6 {
            world::combat::hurt(&order, &mut r.guests[0], guard, 0.4, PLAYER_REF);
            r.run_ms(200);
        }
        assert!(
            r.until(5_000, |r| r.host.coop.campaign.stats.ff_blocked == 3),
            "three of six blows blocked: {} (guest hits sent {})",
            r.host.coop.campaign.stats.ff_blocked,
            r.gas[0].stats.hits_sent
        );
    }

    /// A blow that the guard drops carries nothing into a later blow: the
    /// guest forgets the part point it kept for that victim.
    #[test]
    fn a_blocked_blow_carries_no_part_point_into_a_later_blow() {
        let (_d, order) = order("camp-ff-carry");
        let scripts = ScriptCache::default();
        let ff = format!(
            r#","friendly_fire":{{"actors":["t.esm:{GUARD_REF:X}"],"deliberate_hits":3,"window_s":2}}"#
        );
        let mut r = Rig::new(
            &order,
            &scripts,
            table(&row("FF", 0x777, &ff)),
            "perfect",
            15,
        );
        r.hold_hits = true;
        r.run_ms(1_000);
        let guard = FormId(GUARD_REF);
        let dealt = |r: &Rig| r.host.damage.get(&guard).copied().unwrap_or(0.0);
        // the guest divides a player's blow by the party's toughness: a blow
        // of `pts` points after that scaling
        let blow = |r: &mut Rig, pts: f64| {
            let t = r.guests[0]
                .coop
                .live
                .scaling
                .map_or(1.0, |s| f64::from(s.toughness));
            world::combat::hurt(&order, &mut r.guests[0], guard, pts * t, PLAYER_REF);
        };
        // three whole blows land (a hair over a point each, for the rounding)
        for _ in 0..3 {
            blow(&mut r, 1.000_001);
            r.run_ms(100);
        }
        assert!(r.until(2_000, |r| dealt(r) == 3.0), "dealt {}", dealt(&r));
        // a fourth blow in the window is dropped: the guest kept its 0.6
        blow(&mut r, 0.6);
        assert!(r.until(1_000, |r| r.host.coop.campaign.stats.ff_blocked == 1));
        // the window passes, and a blow of 0.6 lands: 0 points, not 1 (0.6 + 0.6)
        r.run_ms(3_000);
        blow(&mut r, 0.6);
        r.run_ms(1_000);
        assert_eq!(r.host.coop.campaign.stats.ff_blocked, 1);
        assert_eq!(
            dealt(&r),
            3.0,
            "the dropped blow's part point went into the later blow"
        );
    }

    #[test]
    fn mutation_twin_without_a_row_every_hit_lands() {
        let (_d, order) = order("camp-ff-twin");
        let scripts = ScriptCache::default();
        let mut r = Rig::new(&order, &scripts, table(""), "perfect", 13);
        for _ in 0..6 {
            world::combat::hurt(&order, &mut r.host, FormId(GUARD_REF), 1.0, PLAYER_REF);
        }
        assert_eq!(
            r.host
                .damage
                .get(&FormId(GUARD_REF))
                .copied()
                .unwrap_or(0.0),
            6.0
        );
        assert_eq!(r.host.coop.campaign.stats.ff_blocked, 0);
    }

    #[test]
    fn the_window_forgets_old_hits() {
        let (_d, order) = order("camp-ff-window");
        let scripts = ScriptCache::default();
        let ff = format!(
            r#","friendly_fire":{{"actors":["t.esm:{GUARD_REF:X}"],"deliberate_hits":1,"window_s":5}}"#
        );
        let mut r = Rig::new(
            &order,
            &scripts,
            table(&row("FF", 0x777, &ff)),
            "perfect",
            14,
        );
        let guard = FormId(GUARD_REF);
        assert!(ffguard::allow_hit(&order, &mut r.host, 0, guard));
        assert!(!ffguard::allow_hit(&order, &mut r.host, 0, guard));
        r.host.coop.campaign.ff.tick(6.0);
        assert!(ffguard::allow_hit(&order, &mut r.host, 0, guard));
    }
}

mod companions {
    use super::*;

    #[test]
    fn commands_are_owner_only_and_caps_count_critters_apart() {
        let (_d, order) = order("camp-owner");
        let scripts = ScriptCache::default();
        let mut r = Rig::new(&order, &scripts, table(""), "perfect", 15);
        let (guard, gecko, townsperson) = (FormId(GUARD_REF), FormId(GECKO_REF), FormId(TOWN_REF));
        assert!(
            owners::recruit(&order, &mut r.host, 1, guard),
            "slot 1 recruits a humanoid"
        );
        assert_eq!(owners::owner_of(&r.host, guard), Some(1));
        // the humanoid cap is one per player; a critter has its own
        assert!(
            !owners::recruit(&order, &mut r.host, 1, townsperson),
            "second humanoid refused"
        );
        assert!(
            owners::recruit(&order, &mut r.host, 1, gecko),
            "a critter fits beside it"
        );
        // someone else's follower cannot be taken or commanded
        assert!(!owners::recruit(&order, &mut r.host, 2, guard));
        assert!(!owners::command_ok(
            &mut r.host,
            2,
            guard,
            CompanionOp::Wait
        ));
        assert!(owners::command_ok(&mut r.host, 1, guard, CompanionOp::Wait));
        assert!(!owners::dismiss(&mut r.host, 2, guard));
        assert_eq!(r.host.coop.campaign.stats.commands_refused, 2);
        assert!(owners::dismiss(&mut r.host, 1, guard));
        assert_eq!(owners::owner_of(&r.host, guard), None);
        // the host's live follower rows stayed the host's
        assert!(r.host.teammates.is_empty());
    }

    #[test]
    fn a_script_run_for_a_player_recruits_for_that_player() {
        let (_d, order) = order("camp-owner-script");
        let scripts = ScriptCache::default();
        let mut r = Rig::new(&order, &scripts, table(""), "perfect", 16);
        // the hook the recruit script uses runs as the speaker of the line
        assert!(r
            .host
            .with_player(2, |g| g.coop_recruit_ok(FormId(GUARD_REF))));
        let gate = r.host.coop.gate.as_mut().unwrap();
        assert_eq!(gate.initiator(), None, "the scope ended with the call");
    }
}

mod storage_and_vars {
    use super::*;

    #[test]
    fn a_personal_container_holds_one_inventory_per_player() {
        let (_d, order) = order("camp-storage");
        let scripts = ScriptCache::default();
        let mut r = Rig::new(&order, &scripts, table(""), "perfect", 17);
        let chest = FormId(0x4242);
        r.host.coop.campaign.personal.insert(chest);
        let item = FormId(0xF);
        r.host.with_player(1, |g| {
            g.items.insert((chest, item), 5);
        });
        assert_eq!(
            r.host.items.get(&(chest, item)),
            None,
            "the host sees an empty chest"
        );
        r.host
            .with_player(2, |g| assert_eq!(g.items.get(&(chest, item)), None));
        r.host.with_player(1, |g| {
            assert_eq!(g.items.get(&(chest, item)).copied(), Some(5))
        });
        // the ledger path: each player has a holder of their own
        assert_ne!(
            storage::redirect(&r.host, chest, 1),
            storage::redirect(&r.host, chest, 2)
        );
        assert_eq!(
            storage::redirect(&r.host, FormId(0x1111), 1),
            FormId(0x1111)
        );
    }

    #[test]
    fn mutation_twin_a_shared_container_is_shared() {
        let (_d, order) = order("camp-storage-twin");
        let scripts = ScriptCache::default();
        let mut r = Rig::new(&order, &scripts, table(""), "perfect", 18);
        let chest = FormId(0x4242);
        r.host.with_player(1, |g| {
            g.items.insert((chest, FormId(0xF)), 5);
        });
        assert_eq!(r.host.items.get(&(chest, FormId(0xF))).copied(), Some(5));
    }

    fn collar_script() -> script::Script {
        script::parse("scn NVCollarScript\nshort bCollarOn\nfloat fTimer\nbegin gamemode\nend\n")
            .expect("script")
    }

    #[test]
    fn collar_script_variables_are_per_player() {
        let (_d, order) = order("camp-collar");
        let scripts = ScriptCache::default();
        let mut r = Rig::new(&order, &scripts, table(""), "perfect", 19);
        let owner = FormId(0x5151);
        r.host
            .variables
            .insert(owner, script::interp::Locals::new(&collar_script()));
        r.host.coop.campaign.swap_living = true;
        r.host.coop.campaign.objvar_wanted.insert(
            "nvcollarscript".into(),
            vec!["bcollaron".into(), "ftimer".into()],
        );
        r.host
            .coop
            .campaign
            .note_script_owner(owner, "NVCollarScript");
        // player 1's collar ticks; the others do not see it
        r.host.with_player(1, |g| {
            let l = g.variables.get_mut(&owner).unwrap();
            l.set("bcollaron", 1.0);
            l.set("ftimer", 42.5);
        });
        let read = |gs: &GameState| {
            gs.variables
                .get(&owner)
                .map(|l| (l.get("bcollaron"), l.get("ftimer")))
        };
        assert_eq!(
            read(&r.host),
            Some((Some(0.0), Some(0.0))),
            "host collar untouched"
        );
        r.host
            .with_player(2, |g| assert_eq!(read(g), Some((Some(0.0), Some(0.0)))));
        r.host
            .with_player(1, |g| assert_eq!(read(g), Some((Some(1.0), Some(42.5)))));
    }

    #[test]
    fn hardcore_needs_and_imported_stacks_are_per_player() {
        let (_d, order) = order("camp-living");
        let scripts = ScriptCache::default();
        let mut r = Rig::new(&order, &scripts, table(""), "perfect", 20);
        r.host.coop.campaign.swap_living = true;
        r.host.with_player(1, |g| {
            g.living.clocks = [10.0, 20.0, 30.0];
            g.inventory_stacks.insert(
                PLAYER_REF,
                world::inventory_stacks::SavedInventory {
                    version: 1,
                    items: vec![],
                },
            );
        });
        assert_eq!(r.host.living.clocks, [0.0, 0.0, 0.0]);
        assert!(!r.host.inventory_stacks.contains_key(&PLAYER_REF));
        r.host.with_player(1, |g| {
            assert_eq!(g.living.clocks, [10.0, 20.0, 30.0]);
            assert!(g.inventory_stacks.contains_key(&PLAYER_REF));
        });
    }
}

mod conditions {
    use super::*;

    #[test]
    fn party_reducers_read_every_present_player() {
        let (_d, order) = order("camp-party");
        let scripts = ScriptCache::default();
        let rows = row(
            "PQ",
            0x700,
            r#","party_cond":{"GetItemCount":"sum-over-party","GetLevel":"max-over-party","HasPerk":"any-party-member","GetAV":"initiator"}"#,
        );
        let mut r = Rig::new(&order, &scripts, table(&rows), "perfect", 21);
        let (q, item, perk) = (FormId(0x700), FormId(0xF), FormId(0x9999));
        r.host.with_player(0, |g| {
            g.items.insert((PLAYER_REF, item), 2);
            g.player_level = 3;
        });
        r.host.with_player(1, |g| {
            g.items.insert((PLAYER_REF, item), 5);
            g.player_level = 9;
            g.perks.insert(perk);
        });
        r.host.with_player(2, |g| {
            g.items.insert((PLAYER_REF, item), 1);
        });
        let args = [Value::Form(item)];
        let get = |name: &str, a: &[Value]| {
            party::party_value(&r.host, q, name, a, coop_policy::Phase::Entry)
        };
        assert_eq!(get("GetItemCount", &args), Some(8.0), "sum over the party");
        assert_eq!(get("GetLevel", &[]), Some(9.0), "strongest member");
        assert_eq!(
            get("HasPerk", &[Value::Form(perk)]),
            Some(1.0),
            "any player has it"
        );
        assert_eq!(get("HasPerk", &[Value::Form(FormId(0x1))]), Some(0.0));
        // initiator rows are answered by the live rows
        assert_eq!(get("GetAV", &[Value::Number(5.0)]), None);
        // a quest without a row keeps the single player meaning
        assert_eq!(
            party::party_value(
                &r.host,
                FormId(0x701),
                "GetItemCount",
                &args,
                coop_policy::Phase::Entry
            ),
            None
        );
    }

    #[test]
    fn is_killer_and_the_cell_read_each_player() {
        let (_d, order) = order("camp-party-killer");
        let scripts = ScriptCache::default();
        let rows = row(
            "PQ",
            0x700,
            r#","party_cond":{"IsKiller":"any-party-member","GetInCell":"any"}"#,
        );
        let mut r = Rig::new(&order, &scripts, table(&rows), "perfect", 23);
        let (q, victim, cell) = (FormId(0x700), FormId(0x4321), FormId(0x5555));
        let get = |gs: &GameState, name: &str, a: &[Value]| {
            party::party_value(gs, q, name, a, coop_policy::Phase::Entry)
        };
        assert_eq!(get(&r.host, "IsKiller", &[Value::Form(victim)]), Some(0.0));
        // guest 2's blow killed it (the attribution's settle records the killer)
        r.host.dead.insert(victim);
        r.host.coop.campaign.killers.insert(victim, 2);
        assert_eq!(
            get(&r.host, "IsKiller", &[Value::Form(victim)]),
            Some(1.0),
            "any player killed it"
        );
        assert_eq!(get(&r.host, "GetInCell", &[Value::Form(cell)]), Some(0.0));
        // guest 1 walked into the cell: the avatar table refreshes its parked shadow
        let mut players = world::coop::players::PlayerRegistry::new();
        let rec = players.join(coop_core::ids::PlayerId(1), "Guest");
        rec.cell = cell.0;
        rec.position = [10.0, 20.0, 30.0];
        rec.seen = true;
        party::refresh_positions(&mut r.host, &players);
        assert_eq!(
            get(&r.host, "GetInCell", &[Value::Form(cell)]),
            Some(1.0),
            "where the guest is now, not where it joined"
        );
        let p = r.host.shadow_of(1).unwrap().position.unwrap();
        assert_eq!(f32::from_bits(p[2]), 30.0);
    }

    #[test]
    fn the_quest_context_routes_facts_through_the_reducers() {
        let (_d, order) = order("camp-party-facts");
        let scripts = ScriptCache::default();
        let rows = row(
            "PQ",
            0x700,
            r#","party_cond":{"GetLevel":"max-over-party"}"#,
        );
        let mut r = Rig::new(&order, &scripts, table(&rows), "perfect", 22);
        r.host.player_level = 2;
        r.host.with_player(2, |g| g.player_level = 12);
        let level = script::functions::FUNCTIONS
            .iter()
            .position(|f| f.name == "GetLevel")
            .unwrap() as u16;
        fn facts<'a>(order: &'a LoadOrder, gs: &'a GameState) -> world::scripting::Facts<'a> {
            world::scripting::Facts {
                order,
                state: gs,
                speaker: None,
            }
        }
        assert_eq!(
            facts(&order, &r.host).value(level, Some(PLAYER_REF), &[]),
            Some(2.0),
            "no quest context: the live player"
        );
        r.host.coop.campaign.cond_quest = Some(FormId(0x700));
        assert_eq!(
            facts(&order, &r.host).value(level, Some(PLAYER_REF), &[]),
            Some(12.0),
            "quest context: the party maximum"
        );
    }
}

mod forks {
    use super::*;

    fn consent_row(mode: &str) -> String {
        row(
            "Fork",
            0x800,
            &format!(r#","consent":{{"mode":"{mode}","timeout_s":30,"topics":["t.esm:1234"]}}"#),
        )
    }

    #[test]
    fn a_vote_fork_waits_then_follows_the_majority() {
        let (_d, order) = order("camp-fork");
        let scripts = ScriptCache::default();
        let mut r = Rig::new(&order, &scripts, table(&consent_row("vote")), "perfect", 23);
        let q = FormId(0x800);
        assert_eq!(consent::check(&mut r.host, q, 1), consent::Fork::Wait);
        assert_eq!(consent::waiting_for(&r.host, q).count(), 3);
        consent::vote(&mut r.host, q, 0, true);
        consent::vote(&mut r.host, q, 1, true);
        assert_eq!(
            consent::check(&mut r.host, q, 1),
            consent::Fork::Wait,
            "one answer still missing"
        );
        consent::vote(&mut r.host, q, 2, false);
        assert_eq!(consent::check(&mut r.host, q, 1), consent::Fork::Proceed);
        // one way: asking again does not reopen the vote
        assert_eq!(consent::check(&mut r.host, q, 1), consent::Fork::Proceed);
        assert_eq!(r.host.coop.live.fork_result, Some((1, true)));
    }

    #[test]
    fn unanimous_forks_die_on_one_no_and_other_rows_are_unaffected() {
        let (_d, order) = order("camp-fork-un");
        let scripts = ScriptCache::default();
        let mut r = Rig::new(
            &order,
            &scripts,
            table(&consent_row("unanimous")),
            "perfect",
            24,
        );
        let q = FormId(0x800);
        assert_eq!(consent::check(&mut r.host, q, 2), consent::Fork::Wait);
        consent::vote(&mut r.host, q, 0, true);
        consent::vote(&mut r.host, q, 1, false);
        assert!(matches!(
            consent::check(&mut r.host, q, 2),
            consent::Fork::Denied(_)
        ));
        assert_eq!(
            consent::check(&mut r.host, FormId(0x801), 2),
            consent::Fork::Proceed,
            "no row, no vote"
        );
    }

    #[test]
    fn a_vote_times_out_when_nobody_answers() {
        let (_d, order) = order("camp-fork-timeout");
        let scripts = ScriptCache::default();
        let mut r = Rig::new(
            &order,
            &scripts,
            table(&consent_row("unanimous")),
            "perfect",
            25,
        );
        let q = FormId(0x800);
        assert_eq!(consent::check(&mut r.host, q, 3), consent::Fork::Wait);
        r.host.coop.gate.as_mut().unwrap().tick(31.0);
        let done = consent::poll(&mut r.host);
        assert_eq!(done, vec![(q, 3, false)]);
    }
}

mod lifecycle {
    use super::*;

    #[test]
    fn a_downed_player_respawns_at_the_checkpoint_when_the_window_runs_out() {
        let (_d, order) = order("camp-down");
        let scripts = ScriptCache::default();
        let mut r = Rig::new(&order, &scripts, table(""), "perfect", 26);
        for s in 0..=2u8 {
            r.host.with_player(s, |g| {
                g.player_cell = Some(FormId(CELL));
                g.player_position = Some([100.0, 200.0, 0.0]);
            });
        }
        downed::frame(&order, &mut r.host, 0.1); // notes the checkpoint
        r.host
            .with_player(2, |g| g.player_position = Some([9000.0, 9000.0, 0.0]));
        downed::player_down(&mut r.host, 2, None);
        downed::frame(&order, &mut r.host, 10.0);
        assert!(
            r.host.coop.campaign.down.down.contains_key(&2),
            "still within the revive window"
        );
        downed::frame(&order, &mut r.host, 25.0);
        assert!(r.host.coop.campaign.down.down.is_empty());
        assert_eq!(r.host.coop.campaign.down.respawned, vec![2]);
        let p = r.host.shadow_of(2).unwrap().position.unwrap();
        assert_eq!(f32::from_bits(p[0]), 100.0, "back at the checkpoint");
    }

    #[test]
    fn a_revive_inside_the_window_keeps_the_player_where_they_fell() {
        let (_d, order) = order("camp-revive");
        let scripts = ScriptCache::default();
        let mut r = Rig::new(&order, &scripts, table(""), "perfect", 27);
        downed::player_down(&mut r.host, 1, None);
        assert!(downed::revive(&mut r.host, 1));
        downed::frame(&order, &mut r.host, 100.0);
        assert!(r.host.coop.campaign.down.respawned.is_empty());
    }

    #[test]
    fn a_wipe_follows_the_all_down_rule() {
        let (_d, order) = order("camp-wipe");
        let scripts = ScriptCache::default();
        let mut r = Rig::new(&order, &scripts, table(""), "perfect", 28);
        for s in 0..=2u8 {
            downed::player_down(&mut r.host, s, None);
        }
        downed::frame(&order, &mut r.host, 1.0);
        // respawn-at-checkpoint in this table: everyone is up again
        assert!(r.host.coop.campaign.down.down.is_empty());
        assert_eq!(r.host.coop.campaign.down.respawned.len(), 3);
        assert!(!r.host.coop.campaign.down.reload_requested);
    }

    #[test]
    fn a_collar_down_waits_for_the_checkpoint_cell() {
        let (_d, order) = order("camp-collar-down");
        let scripts = ScriptCache::default();
        let extra = format!(
            r#","shadow":{{"scope":"collar","vars":["fTimer"],"checkpoint":["t.esm:{CELL:X}"],"down":"until_checkpoint"}}"#
        );
        let mut r = Rig::new(
            &order,
            &scripts,
            table(&row("Collar", 0x900, &extra)),
            "perfect",
            29,
        );
        let out = downed::player_down(&mut r.host, 1, Some(FormId(0x900))).unwrap();
        assert!(matches!(
            out,
            coop_policy::lifecycle::DownOutcome::UntilCheckpoint { .. }
        ));
        downed::frame(&order, &mut r.host, 500.0);
        assert!(
            r.host.coop.campaign.down.down.contains_key(&1),
            "the clock does not respawn a collar down"
        );
    }

    #[test]
    fn a_collar_down_refuses_a_revive_and_tells_the_live_rules() {
        let (_d, order) = order("camp-collar-revive");
        let scripts = ScriptCache::default();
        let extra = format!(
            r#","shadow":{{"scope":"collar","vars":["fTimer"],"checkpoint":["t.esm:{CELL:X}"],"down":"until_checkpoint"}}"#
        );
        let mut r = Rig::new(
            &order,
            &scripts,
            table(&row("Collar", 0x900, &extra)),
            "perfect",
            29,
        );
        let out = downed::script_down(&mut r.host, 1, Some(FormId(0x900)));
        assert!(matches!(
            out,
            Some(coop_policy::lifecycle::DownOutcome::UntilCheckpoint { .. })
        ));
        assert_eq!(
            r.host.coop.campaign.down.to_live,
            vec![(1, false)],
            "the live rules hear of a down that is not revivable"
        );
        assert!(!downed::revivable(&r.host, 1));
        assert!(
            !downed::revive(&mut r.host, 1),
            "a collar death is not revivable"
        );
        assert!(r.host.coop.campaign.down.down.contains_key(&1));
        // an ordinary down is revivable
        let out = downed::script_down(&mut r.host, 2, None);
        assert!(matches!(
            out,
            Some(coop_policy::lifecycle::DownOutcome::Downed { .. })
        ));
        assert!(downed::revive(&mut r.host, 2));
    }

    #[test]
    fn a_late_joiner_waits_out_a_scene_then_is_warped_in() {
        let (_d, order) = order("camp-late");
        let scripts = ScriptCache::default();
        let mut r = Rig::new(
            &order,
            &scripts,
            table(&row("Scene", 0xA00, "")),
            "perfect",
            30,
        );
        for s in 0..=2u8 {
            r.host.with_player(s, |g| {
                g.player_cell = Some(FormId(CELL));
                g.player_position = Some([7.0, 8.0, 0.0]);
            });
        }
        downed::frame(&order, &mut r.host, 0.1);
        r.host
            .coop
            .gate
            .as_mut()
            .unwrap()
            .scene(FormId(0xA00), true);
        let plan = downed::on_join(&mut r.host, 2);
        assert_eq!(plan, Some(coop_policy::lifecycle::JoinPlan::WaitForScene));
        assert!(
            !r.host.coop.gate.as_ref().unwrap().present.contains(2),
            "held out of the gates"
        );
        r.host
            .with_player(2, |g| g.player_position = Some([5000.0, 0.0, 0.0]));
        downed::frame(&order, &mut r.host, 1.0);
        assert!(
            !r.host.coop.gate.as_ref().unwrap().present.contains(2),
            "still waiting"
        );
        r.host
            .coop
            .gate
            .as_mut()
            .unwrap()
            .scene(FormId(0xA00), false);
        downed::frame(&order, &mut r.host, 1.0);
        assert!(
            r.host.coop.gate.as_ref().unwrap().present.contains(2),
            "counted again"
        );
        assert_eq!(r.host.coop.campaign.down.warped, vec![2]);
        let p = r.host.shadow_of(2).unwrap().position.unwrap();
        assert_eq!(f32::from_bits(p[0]), 7.0);
    }
}

mod producers_and_hash {
    use super::*;

    #[test]
    fn zone_membership_follows_each_players_cell() {
        let (_d, order) = order("camp-zone");
        let scripts = ScriptCache::default();
        let cell_edid = order
            .get(FormId(CELL))
            .and_then(|r| r.editor_id().ok().flatten())
            .expect("the test cell has an editor id");
        let extra = format!(
            r#","tether":{{"cells":["{cell_edid}"],"gather":"wait"}},"barrier":{{"stages":[100]}}"#
        );
        let mut r = Rig::new(
            &order,
            &scripts,
            table(&row("Zone", 0xB00, &extra)),
            "perfect",
            31,
        );
        r.host
            .with_player(0, |g| g.player_cell = Some(FormId(CELL)));
        r.host
            .with_player(1, |g| g.player_cell = Some(FormId(CELL)));
        r.host
            .with_player(2, |g| g.player_cell = Some(FormId(0xDEAD)));
        producers::update_zones(&order, &mut r.host);
        let s = r
            .host
            .coop
            .gate
            .as_ref()
            .unwrap()
            .session(FormId(0xB00))
            .cloned()
            .unwrap();
        assert!(s.in_zone.contains(0) && s.in_zone.contains(1) && !s.in_zone.contains(2));
        assert!(
            s.passed.contains(0) && s.passed.contains(1) && !s.passed.contains(2),
            "barrier passes follow the zone"
        );
        // the straggler arrives
        r.host
            .with_player(2, |g| g.player_cell = Some(FormId(CELL)));
        producers::update_zones(&order, &mut r.host);
        assert!(r
            .host
            .coop
            .gate
            .as_ref()
            .unwrap()
            .session(FormId(0xB00))
            .unwrap()
            .in_zone
            .contains(2));
        // and leaving again
        r.host
            .with_player(1, |g| g.player_cell = Some(FormId(0xDEAD)));
        producers::update_zones(&order, &mut r.host);
        assert!(!r
            .host
            .coop
            .gate
            .as_ref()
            .unwrap()
            .session(FormId(0xB00))
            .unwrap()
            .in_zone
            .contains(1));
    }

    #[test]
    fn a_dead_actor_is_in_the_shared_export_and_repairs_a_lost_kill() {
        let (_d, order) = order("camp-dead-hash");
        let mut host = GameState::new(&order);
        host.set_authority(Authority::Host);
        let mut guest = GameState::new(&order);
        guest.set_authority(Authority::Guest);
        let gecko = FormId(GECKO_REF);
        let before = world::coop::export::export(&host);
        host.dead.insert(gecko);
        let after = world::coop::export::export(&host);
        assert_ne!(before, after, "the shared hash notices a death");
        // the guest never got the kill message: the keyed diff repairs it, once
        let want = world::coop::export::export(&host);
        world::coop::apply::reconcile(&mut guest, &want, Opts::default());
        assert!(guest.dead.contains(&gecko));
        let died = guest
            .events
            .iter()
            .filter(|e| matches!(e, world::scripting::Event::Died { .. }))
            .count();
        assert_eq!(died, 1);
        world::coop::apply::reconcile(&mut guest, &want, Opts::default());
        assert_eq!(
            guest
                .events
                .iter()
                .filter(|e| matches!(e, world::scripting::Event::Died { .. }))
                .count(),
            1
        );
        // players' own deaths stay out of it
        host.dead.insert(PLAYER_REF);
        assert_eq!(world::coop::export::export(&host), after);
    }

    #[test]
    fn invented_variable_names_are_replaced_when_game_data_arrives() {
        let (_d, order) = order("camp-names");
        let scripts = ScriptCache::default();
        let mut gs = GameState::new(&order);
        gs.set_authority(Authority::Guest);
        let owner = FormId(0x5252);
        let mut l = script::interp::Locals::default();
        l.insert("coopvar0", script::VarKind::Float, 1.5);
        l.insert("coopvar1", script::VarKind::Float, 2.5);
        gs.variables.insert(owner, l);
        // no script for this owner: nothing to heal with, nothing breaks
        assert_eq!(
            world::coop::campaign::names::heal(
                Ctx {
                    order: &order,
                    scripts: &scripts
                },
                &mut gs
            ),
            0
        );
        assert!(world::coop::campaign::names::is_unnamed(
            gs.variables.get(&owner).unwrap()
        ));
    }

    #[test]
    fn ready_checks_collect_answers_and_time_out() {
        let (_d, order) = order("camp-ready");
        let scripts = ScriptCache::default();
        let extra = r#","ready":{"infos":["t.esm:1234"],"timeout_s":20}"#;
        let mut r = Rig::new(
            &order,
            &scripts,
            table(&row("Ready", 0xC00, extra)),
            "perfect",
            32,
        );
        let q = FormId(0xC00);
        r.host.coop.campaign.ready.insert(q, 0x1234);
        r.host.coop.gate.as_mut().unwrap().begin_ready(q);
        producers::ready_answer(&mut r.host, q, 0, true);
        producers::ready_answer(&mut r.host, q, 1, true);
        assert!(matches!(
            producers::ready_outcome(&mut r.host, q),
            Some(coop_policy::Decision::DeferToBarrier { .. })
        ));
        producers::ready_answer(&mut r.host, q, 2, true);
        assert_eq!(
            producers::ready_outcome(&mut r.host, q),
            Some(coop_policy::Decision::Apply)
        );
        assert!(
            producers::ready_open(&r.host).is_empty(),
            "a finished check is forgotten"
        );
    }
}

#[test]
fn attribution_slots_for_combat_forms() {
    assert_eq!(attribution::slot_of(PLAYER_REF, 4), Some(4));
    assert_eq!(
        attribution::slot_of(world::coop::facts::puppet_of(PlayerId(2)), 0),
        Some(2)
    );
    assert_eq!(attribution::slot_of(FormId(0x1234), 0), None);
}

/// #74: the host counts a guest's named blows from its own copy of that guest.
mod named_blows {
    use super::*;
    use coop_core::actors::{BlastBlow, Blow};
    use coop_core::ids::FormId as CForm;
    use world::coop::facts::PendingHit;

    /// Guest 1's copy on the host holds `weapon`; guest hits go through the
    /// host's hold, as in a session. Returns the gecko's host health.
    fn rig<'a>(
        order: &'a LoadOrder,
        scripts: &'a ScriptCache,
        seed: u64,
        weapon: u32,
    ) -> (Rig<'a>, f64) {
        let mut r = Rig::new(order, scripts, table(""), "perfect", seed);
        r.hold_hits = true;
        // single-player numbers: the party scaling moves as the party forms
        r.sim
            .host
            .live
            .rules
            .set_mode(coop_core::rules::Mode::Normal);
        r.host.with_player(1, |g| {
            g.equipped.insert(PLAYER_REF, vec![FormId(weapon)]);
        });
        r.run_ms(1_000);
        let before = world::combat::health(order, &r.host, FormId(GECKO_REF)).unwrap();
        (r, before)
    }

    fn named(weapon: u32, flags: u8) -> Blow {
        Blow {
            by: None,
            weapon: Some(CForm(weapon)),
            part: None,
            flags,
            vats: None,
            blast: None,
        }
    }

    /// The host's own count of one blow of guest 1 with the weapon in its
    /// hand, before the party scaling.
    fn host_count(r: &mut Rig, order: &LoadOrder, power: bool) -> f64 {
        let gecko = FormId(GECKO_REF);
        r.host.with_player(1, |g| {
            let w = world::combat::weapon_in_hand(order, g, PLAYER_REF).unwrap();
            let inputs = world::blow::BlowInputs {
                attacker: PLAYER_REF,
                target: gecko,
                weapon: Some(&w),
                part: None,
                power,
                sneak: false,
                critical: false,
            };
            f64::from(world::blow::blow_damage(order, g, inputs).dealt)
        })
    }

    /// The whole points of the host's count `raw` (Normal rules: no scaling).
    fn assert_points(r: &Rig, lost: f64, raw: f64, what: &str) {
        let t = r
            .host
            .coop
            .live
            .scaling
            .map_or(1.0, |s| f64::from(s.toughness));
        assert_eq!(t, 1.0, "Normal rules");
        assert_eq!(lost, raw.floor(), "{what}");
    }

    /// The guest's 4000 points are not used: the gecko loses the host's count.
    #[test]
    fn the_host_counts_a_guests_named_blow() {
        let (_d, order) = order("blow-count");
        let scripts = ScriptCache::default();
        let (mut r, before) = rig(&order, &scripts, 31, MACHETE);
        let gecko = FormId(GECKO_REF);
        let expected = host_count(&mut r, &order, false);
        assert!(
            expected >= 1.0 && expected < before,
            "a machete blow ({expected} of {before})"
        );
        r.guests[0].coop.live.pending_hits.push(PendingHit {
            who: gecko,
            amount: 4000.0,
            by: PLAYER_REF,
            blow: Some(named(MACHETE, 0)),
        });
        r.run_ms(3_000);
        let after = world::combat::health(&order, &r.host, gecko).unwrap();
        assert!(after > 0.0, "not the guest's 4000 points");
        assert_points(&r, before - after, expected, "the host's count");
        assert_eq!(r.ha.stats.blows_refused, 0);
    }

    /// A blow with a weapon that the host's copy does not hold deals nothing.
    #[test]
    fn the_host_refuses_a_blow_with_a_weapon_not_in_hand() {
        let (_d, order) = order("blow-refuse");
        let scripts = ScriptCache::default();
        let (mut r, before) = rig(&order, &scripts, 32, MACHETE);
        let gecko = FormId(GECKO_REF);
        r.guests[0].coop.live.pending_hits.push(PendingHit {
            who: gecko,
            amount: 30.0,
            by: PLAYER_REF,
            blow: Some(named(PISTOL, 0)),
        });
        r.run_ms(3_000);
        assert_eq!(world::combat::health(&order, &r.host, gecko), Some(before));
        assert_eq!(r.ha.stats.blows_refused, 1);
    }

    /// A sneak attack needs the guest sneaking: the host drops that part of
    /// the blow and counts it without the sneak multiplier.
    #[test]
    fn the_host_drops_a_sneak_attack_the_guest_was_not_in() {
        let (_d, order) = order("blow-sneak");
        let scripts = ScriptCache::default();
        let (mut r, before) = rig(&order, &scripts, 33, MACHETE);
        let gecko = FormId(GECKO_REF);
        let expected = host_count(&mut r, &order, false);
        r.guests[0].coop.live.pending_hits.push(PendingHit {
            who: gecko,
            amount: 50.0,
            by: PLAYER_REF,
            blow: Some(named(MACHETE, Blow::SNEAK)),
        });
        r.run_ms(3_000);
        let after = world::combat::health(&order, &r.host, gecko).unwrap();
        assert!(expected * 2.0 >= before, "a sneak attack would kill it");
        assert!(after > 0.0, "no sneak multiplier");
        assert_points(&r, before - after, expected, "sneak dropped");
        assert_eq!(r.ha.stats.blow_parts_dropped, 1);
    }

    /// #74: a queued V.A.T.S. shot (the harness's `coop vats`) names its
    /// blow: the weapon in hand, the body part and the attack kind of the
    /// weapon. The host counts each hit from its own copy of the guest, as a
    /// V.A.T.S. attack, and not the guest's 4000 points.
    #[test]
    fn the_host_counts_a_queued_vats_shot() {
        let (_d, order) = order("blow-vats");
        let scripts = ScriptCache::default();
        let (mut r, before) = rig(&order, &scripts, 35, MACHETE);
        let gecko = FormId(GECKO_REF);
        let part = 0u8;
        let expected = r.host.with_player(1, |g| {
            let w = world::combat::weapon_in_hand(&order, g, PLAYER_REF).unwrap();
            let saved = (g.vats.mode, g.vats.attack.take());
            g.vats.mode = world::vats::mode::PLAYBACK;
            g.vats.attack = Some(world::vats::AttackFacts {
                weapon: Some(w.form_id),
                target: gecko,
                part_av: -1,
                kind: world::vats::attack_kind(Some(w.animation)),
                hit: true,
                distance: 0.0,
                paralyzing_palm: false,
            });
            let inputs = world::blow::BlowInputs {
                attacker: PLAYER_REF,
                target: gecko,
                weapon: Some(&w),
                part: Some(part),
                power: false,
                sneak: false,
                critical: false,
            };
            let d = f64::from(world::blow::blow_damage(&order, g, inputs).dealt);
            (g.vats.mode, g.vats.attack) = saved;
            d
        });
        assert!(
            expected >= 1.0 && expected < before,
            "{expected} of {before}"
        );
        // the guest's own world holds the machete too
        r.guests[0]
            .equipped
            .insert(PLAYER_REF, vec![FormId(MACHETE)]);
        let words = ["coop", "vats", "gecko", "0", "1000", "4000"];
        assert!(world::coop::combat_live::bot_line(
            &order,
            &mut r.guests[0],
            &words,
            &|_| Some(gecko)
        ));
        let ask = r.guests[0].coop.live.combat.vats[0];
        let blow = ask.blow.expect("a named shot");
        assert_eq!(blow.weapon, Some(CForm(MACHETE)));
        assert_eq!(blow.part, Some(part));
        r.run_ms(5_000);
        let results = &r.guests[0].coop.live.combat.vats_results;
        assert_eq!(results.len(), 1, "the host answered the queue");
        let hits = results[0].1.iter().filter(|x| x.hit).count();
        assert_eq!(hits, 1, "seed 35 rolls a hit");
        let after = world::combat::health(&order, &r.host, gecko).unwrap();
        assert!(after > 0.0, "not the guest's 4000 points");
        assert_points(&r, before - after, expected, "the host's count of the shot");
        assert_eq!(r.ha.stats.blows_refused, 0);
    }

    /// The gecko's place in the host's world, moved by `dx` units on x.
    fn beside_gecko(r: &Rig, order: &LoadOrder, dx: f32) -> [f32; 3] {
        let p = r.host.place(order, FormId(GECKO_REF)).expect("the gecko").2;
        [p[0] + dx, p[1], p[2]]
    }

    /// The host's own count of guest 1's frag at `at` on the gecko.
    fn host_blast(r: &mut Rig, order: &LoadOrder, at: [f32; 3]) -> f64 {
        let gecko = FormId(GECKO_REF);
        let place = r.host.place(order, gecko).unwrap().2;
        r.host.with_player(1, |g| {
            use world::explosions as x;
            let w = world::combat::Weapon::load(order, FormId(FRAG)).unwrap();
            let e = x::ExplosionRecord::load(order, FormId(FRAG_EXPLOSION)).unwrap();
            let radius = x::blast_radius(order, g, Some(PLAYER_REF), Some(w.form_id), &e);
            let d = [0, 1, 2].map(|k| at[k] - place[k]);
            let share = x::falloff(radius, (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt());
            let base = x::base_damage(order, g, Some(PLAYER_REF), Some(&w), &e);
            f64::from(x::hit_damage(
                order,
                g,
                base * share,
                Some(PLAYER_REF),
                Some(&w),
                gecko,
                false,
            ))
        })
    }

    /// The guest's world sets off its frag at `at` with `damage` on the gecko
    /// (`Runner::explosion_hit`, as the viewer's explosion does).
    fn guest_blast(
        r: &mut Rig,
        order: &LoadOrder,
        scripts: &ScriptCache,
        at: [f32; 3],
        damage: f32,
    ) {
        let frag = world::combat::Weapon::load(order, FormId(FRAG)).unwrap();
        world::scripting::Runner::new(order, scripts, &mut r.guests[0]).explosion_hit(
            Some(PLAYER_REF),
            FormId(GECKO_REF),
            Some(&frag),
            damage,
            Some((FormId(FRAG_EXPLOSION), at)),
        );
    }

    /// #74: a guest's explosion names its blow (the frag, the explosion and
    /// its centre). The host counts the damage from its own copy of the
    /// guest and its own gecko's place, not the guest's 4000 points.
    #[test]
    fn the_host_counts_a_guests_explosion() {
        let (_d, order) = order("blow-blast");
        let scripts = ScriptCache::default();
        let (mut r, before) = rig(&order, &scripts, 36, FRAG);
        let gecko = FormId(GECKO_REF);
        let at = beside_gecko(&r, &order, 200.0);
        let expected = host_blast(&mut r, &order, at);
        assert!(
            expected >= 1.0 && expected < before,
            "a frag at 200 units ({expected} of {before})"
        );
        guest_blast(&mut r, &order, &scripts, at, 4000.0);
        let hit = r.guests[0].coop.live.pending_hits[0];
        let blast = hit.blow.and_then(|b| b.blast).expect("a named explosion");
        assert_eq!(blast.explosion, CForm(FRAG_EXPLOSION));
        r.run_ms(3_000);
        let after = world::combat::health(&order, &r.host, gecko).unwrap();
        assert!(after > 0.0, "not the guest's 4000 points");
        assert_points(
            &r,
            before - after,
            expected,
            "the host's count of the explosion",
        );
        assert_eq!(r.ha.stats.blows_refused, 0);
    }

    /// The gecko outside the radius at the host's place takes nothing.
    #[test]
    fn the_host_refuses_an_explosion_outside_its_radius() {
        let (_d, order) = order("blow-blast-far");
        let scripts = ScriptCache::default();
        let (mut r, before) = rig(&order, &scripts, 37, FRAG);
        let at = beside_gecko(&r, &order, 450.0);
        guest_blast(&mut r, &order, &scripts, at, 30.0);
        r.run_ms(3_000);
        let gecko = FormId(GECKO_REF);
        assert_eq!(world::combat::health(&order, &r.host, gecko), Some(before));
        assert_eq!(r.ha.stats.blows_refused, 1);
    }

    /// The explosion must be one that the weapon makes: a machete makes none.
    #[test]
    fn the_host_refuses_an_explosion_the_weapon_does_not_make() {
        let (_d, order) = order("blow-blast-machete");
        let scripts = ScriptCache::default();
        let (mut r, before) = rig(&order, &scripts, 38, MACHETE);
        let at = beside_gecko(&r, &order, 100.0);
        let gecko = FormId(GECKO_REF);
        let mut blow = named(MACHETE, 0);
        blow.blast = Some(BlastBlow {
            explosion: CForm(FRAG_EXPLOSION),
            at: at.map(coop_core::avatar::quant_pos),
        });
        r.guests[0].coop.live.pending_hits.push(PendingHit {
            who: gecko,
            amount: 30.0,
            by: PLAYER_REF,
            blow: Some(blow),
        });
        r.run_ms(3_000);
        assert_eq!(world::combat::health(&order, &r.host, gecko), Some(before));
        assert_eq!(r.ha.stats.blows_refused, 1);
    }

    /// A thrown frag leaves the hand before it goes off. The host's copy held
    /// it in hand earlier, so the host still counts it. Mutation twin: a guest
    /// whose copy never held a frag has its frag explosion refused.
    #[test]
    fn a_frag_that_left_the_hand_counts_only_when_the_copy_held_it() {
        let (_d, order) = order("blow-blast-thrown");
        let scripts = ScriptCache::default();
        for (held, seed) in [(true, 39), (false, 40)] {
            let weapon = if held { FRAG } else { MACHETE };
            let (mut r, before) = rig(&order, &scripts, seed, weapon);
            let at = beside_gecko(&r, &order, 200.0);
            let expected = host_blast(&mut r, &order, at);
            // the last frag went: the copy's hand is empty and it carries none
            r.host.with_player(1, |g| {
                g.equipped.insert(PLAYER_REF, vec![]);
                assert_eq!(g.item_count(&order, PLAYER_REF, FormId(FRAG)), 0);
            });
            guest_blast(&mut r, &order, &scripts, at, 4000.0);
            r.run_ms(3_000);
            let gecko = FormId(GECKO_REF);
            let after = world::combat::health(&order, &r.host, gecko).unwrap();
            if held {
                assert_points(&r, before - after, expected, "the frag held earlier");
                assert_eq!(r.ha.stats.blows_refused, 0);
            } else {
                assert_eq!(after, before, "never held: refused");
                assert_eq!(r.ha.stats.blows_refused, 1);
            }
        }
    }

    /// Mutation twin: without the host's count, the guest's number lands.
    #[test]
    fn mutation_twin_a_blow_with_no_name_keeps_the_guests_number() {
        let (_d, order) = order("blow-unnamed");
        let scripts = ScriptCache::default();
        let (mut r, before) = rig(&order, &scripts, 34, MACHETE);
        let gecko = FormId(GECKO_REF);
        r.guests[0].coop.live.pending_hits.push(PendingHit {
            who: gecko,
            amount: 3.0,
            by: PLAYER_REF,
            blow: None,
        });
        r.run_ms(3_000);
        let after = world::combat::health(&order, &r.host, gecko).unwrap();
        assert_eq!(before - after, 3.0);
    }
}

mod pace {
    use super::*;
    use coop_core::avatar::dequant_pos;
    use coop_core::proto::Msg;
    use coop_core::roam::RoamMsg;

    fn host_x(r: &Rig) -> Option<f32> {
        let s = r.sim.host.live.avatars.state(PlayerId(1));
        s.map(|s| dequant_pos(s.pos[0]))
    }

    fn rig<'a>(order: &'a LoadOrder, scripts: &'a ScriptCache, test_warps: bool) -> Rig<'a> {
        let mut r = Rig::new(order, scripts, table(""), "perfect", 41);
        r.sim.host.live.test_warps = test_warps;
        r.run_ms(1_500);
        r
    }

    /// #74: the host follows a guest that walks. When the guest jumps with
    /// no warp, the host keeps the last place, and after the wait it sends
    /// the guest back there.
    #[test]
    fn the_host_bounds_a_guests_movement() {
        let (_d, order) = order("pace");
        let scripts = ScriptCache::default();
        let mut r = rig(&order, &scripts, false);
        assert_eq!(host_x(&r), Some(900.0));
        // a walk: 100 units in a second
        for i in 1..=10 {
            r.guests[0].player_position = Some([900.0 + 10.0 * i as f32, 1000.0, 5000.0]);
            r.run_ms(100);
        }
        r.run_ms(300);
        assert_eq!(host_x(&r), Some(1000.0));
        assert_eq!(r.sim.host.live.avatars.too_far, 0);
        // 20000 units in one sample, no warp
        r.guests[0].player_position = Some([21_000.0, 1000.0, 5000.0]);
        r.run_ms(500);
        assert_eq!(host_x(&r), Some(1000.0), "the host keeps the last place");
        assert!(r.sim.host.live.avatars.too_far > 0);
        let back = r.until(2_000, |r| r.sim.guests[0].sync.live.roam.place.is_some());
        assert!(back, "the host sends the guest back");
        let place = r.sim.guests[0].sync.live.roam.place.unwrap();
        assert_eq!(place.pos[0], 1000.0);
    }

    /// The harness's teleport is a warp the host knows in a test session.
    /// Mutation twin: a host not in a test session refuses the jump.
    #[test]
    fn a_test_warp_lets_the_jump_through_only_in_a_test_session() {
        let (_d, order) = order("pace-warp");
        let scripts = ScriptCache::default();
        for test in [true, false] {
            let mut r = rig(&order, &scripts, test);
            r.sim.guest_emit(0, vec![Msg::Roam(RoamMsg::TestWarp)]);
            r.guests[0].player_position = Some([21_000.0, 1000.0, 5000.0]);
            r.run_ms(500);
            let want = if test { 21_000.0 } else { 900.0 };
            assert_eq!(host_x(&r), Some(want), "test session {test}");
        }
    }
}

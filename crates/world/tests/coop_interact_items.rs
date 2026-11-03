//! Crafting and aid items in co-op: each is the acting player's own, done
//! once on the host in that player's shadow (`testdata::crafting` and
//! `testdata::companion_gear` worlds, no game data).

use coop_core::ids::{FormId as CForm, PlayerId};
use coop_core::interact::{Act, Why};
use coop_core::sim_test::Sim;
use coop_core::sync::GuestPhase;
use coop_core::transport::{ChaosProfile, PROFILE_NAMES};
use esm::{ActivePlugins, FormId, LoadOrder};
use world::coop::apply::{Ctx, Opts};
use world::coop::live::{LiveGuestAdapter, LiveHostAdapter};
use world::coop::{Authority, GuestBridge, HostBridge, ShadowPlan};
use world::dialogue::PLAYER_REF;
use world::scripting::{GameState, ScriptCache};

fn f(n: u32) -> FormId {
    FormId(n)
}

fn c(n: u32) -> CForm {
    CForm(n)
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
}

impl<'a> Rig<'a> {
    fn new(order: &'a LoadOrder, scripts: &'a ScriptCache, profile: &str, seed: u64) -> Rig<'a> {
        let mut host = GameState::new(order);
        host.set_authority(Authority::Host);
        host.enable_shadows(ShadowPlan::all(), 0);
        for s in 1..=2u8 {
            host.register_player(s);
            host.with_player(s, |g| g.stock(order, PLAYER_REF));
        }
        let guests: Vec<GameState> = (1..=2u8)
            .map(|s| {
                let mut g = GameState::new(order);
                g.set_authority(Authority::Guest);
                g.enable_shadows(ShadowPlan::all(), s);
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
        };
        let ok = r.sim.run_until(120_000, |s| {
            s.guests
                .iter()
                .all(|g| g.sync.phase() == GuestPhase::Active)
        });
        assert!(ok, "guests never became active");
        for (i, g) in std::iter::once(&mut r.host)
            .chain(r.guests.iter_mut())
            .enumerate()
        {
            g.player_cell = Some(f(0x100));
            g.player_position = Some([i as f32 * 10.0, 0.0, 0.0]);
        }
        r
    }

    fn step(&mut self) {
        let cx = Ctx {
            order: self.order,
            scripts: self.scripts,
        };
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
        self.sim.step();
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

    fn run_ms(&mut self, ms: u64) {
        self.until(ms, |_| false);
    }

    fn give(&mut self, slot: u8, item: u32, n: i32) {
        self.host.with_player(slot, |g| {
            g.stocked.insert(PLAYER_REF);
            *g.items.entry((PLAYER_REF, f(item))).or_insert(0) += n;
        });
    }

    fn held(&self, slot: u8, item: u32) -> i32 {
        self.host
            .shadow_of(slot)
            .and_then(|s| s.items.get(&f(item)).copied())
            .unwrap_or(0)
    }

    fn mine(&self, i: usize, item: u32) -> i32 {
        self.guests[i]
            .items
            .get(&(PLAYER_REF, f(item)))
            .copied()
            .unwrap_or(0)
    }

    fn results(&self, i: usize) -> Vec<(String, bool, Why)> {
        self.guests[i]
            .coop
            .interact
            .results
            .iter()
            .map(|r| (r.act.name().to_string(), r.ok, r.why))
            .collect()
    }
}

fn profiles() -> Vec<&'static str> {
    let mut v = vec!["perfect"];
    v.extend(PROFILE_NAMES.iter().copied());
    v
}

#[test]
fn crafting_spends_the_crafters_own_ingredients_and_obeys_their_skill() {
    use testdata::crafting::ids::*;
    for (n, profile) in profiles().into_iter().enumerate() {
        let data = testdata::crafting::crafting(&format!("ix-craft-{n}"));
        let order = LoadOrder::from_data_dir(data.path(), &ActivePlugins::OfficialOnly).unwrap();
        let scripts = ScriptCache::default();
        let mut r = Rig::new(&order, &scripts, profile, 1100 + n as u64);
        r.give(1, SCRAP, 3);
        r.give(1, FLOWER, 2);
        r.give(2, SCRAP, 1);
        assert!(
            r.until(90_000, |r| r.mine(0, SCRAP) == 3 && r.mine(1, SCRAP) == 1),
            "{profile}: packs arrived"
        );
        // guest 1 makes rounds twice; guest 2 asks for more than they have
        r.guests[0].coop_route(Act::Craft {
            bench: c(WORKBENCH),
            recipe: c(R_ROUNDS),
            times: 2,
        });
        r.guests[1].coop_route(Act::Craft {
            bench: c(WORKBENCH),
            recipe: c(R_ROUNDS),
            times: 2,
        });
        assert!(
            r.until(90_000, |r| r.held(1, ROUND) == 20
                && r.results(1).iter().any(|x| x.0 == "craft")),
            "{profile}: made"
        );
        assert_eq!(
            r.held(1, SCRAP),
            1,
            "{profile}: two scrap spent from guest 1"
        );
        assert_eq!(r.held(2, ROUND), 0, "{profile}: guest 2 made nothing");
        assert_eq!(r.held(2, SCRAP), 1, "{profile}: and lost nothing");
        assert!(
            r.results(1)
                .iter()
                .any(|x| x.0 == "craft" && !x.1 && x.2 == Why::Insufficient),
            "{profile}: told why"
        );
        assert_eq!(r.held(0, ROUND), 0, "{profile}: the host is not paid");
        assert!(
            r.until(60_000, |r| r.mine(0, ROUND) == 20 && r.mine(1, ROUND) == 0),
            "{profile}: each sees their own result"
        );
        // medicine skill 50 is missing: refused, nothing spent
        r.guests[0].coop_route(Act::Craft {
            bench: c(WORKBENCH),
            recipe: c(R_STIMPAK),
            times: 1,
        });
        assert!(
            r.until(60_000, |r| r
                .results(0)
                .iter()
                .any(|x| x.0 == "craft" && !x.1)),
            "{profile}: skill check"
        );
        assert_eq!(r.held(1, FLOWER), 2, "{profile}: flowers kept");
        // the host crafts through the same executor
        r.host.stocked.insert(PLAYER_REF);
        r.host.items.insert((PLAYER_REF, f(SCRAP)), 1);
        let cx = Ctx {
            order: &order,
            scripts: &scripts,
        };
        r.host.coop_do(
            cx,
            Act::Craft {
                bench: c(WORKBENCH),
                recipe: c(R_ROUNDS),
                times: 1,
            },
        );
        assert_eq!(
            r.host.items.get(&(PLAYER_REF, f(ROUND))),
            Some(&10),
            "{profile}: host crafted"
        );
        r.run_ms(500);
    }
}

#[test]
fn an_aid_item_is_used_up_by_the_player_who_used_it() {
    use testdata::companion_gear::ids::*;
    for (n, profile) in profiles().into_iter().enumerate() {
        let data = testdata::companion_gear::companion_gear(&format!("ix-use-{n}"));
        let order = LoadOrder::from_data_dir(data.path(), &ActivePlugins::OfficialOnly).unwrap();
        let scripts = ScriptCache::default();
        let mut r = Rig::new(&order, &scripts, profile, 1200 + n as u64);
        r.give(1, FOOD, 2);
        r.give(1, STIMPAK, 1);
        r.give(2, FOOD, 1);
        assert!(
            r.until(90_000, |r| r.mine(0, FOOD) == 2 && r.mine(1, FOOD) == 1),
            "{profile}"
        );
        r.guests[0].coop_route(Act::Use { item: c(FOOD) });
        r.guests[1].coop_route(Act::Use { item: c(NUKA_COLA) });
        assert!(
            r.until(90_000, |r| r.held(1, FOOD) == 1
                && r.results(1).iter().any(|x| x.0 == "use" && !x.1)),
            "{profile}"
        );
        assert_eq!(r.held(2, FOOD), 1, "{profile}: guest 2 keeps theirs");
        assert!(
            r.until(60_000, |r| r.mine(0, FOOD) == 1),
            "{profile}: guest 1 sees it"
        );
        // an effect that was on guest 1 is not on guest 2 (rads from the food)
        let rads = |r: &Rig, slot: u8| {
            r.host.shadow_of(slot).map_or(0.0, |s| {
                s.value_damage.values().sum::<f64>() + s.damage.unwrap_or(0.0)
            })
        };
        let (a, b) = (rads(&r, 1), rads(&r, 2));
        assert!(
            a != b || a == 0.0,
            "{profile}: effects stay with the user ({a} vs {b})"
        );
        // the verdict travels apart from the item record: under loss it can come later
        assert!(
            r.until(60_000, |r| r.results(0).iter().any(|x| x.0 == "use" && x.1)),
            "{profile}: verdict"
        );
        r.run_ms(500);
    }
}

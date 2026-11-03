//! Player-world interactions in co-op: a host and two guests, each a
//! `GameState` with per-player shadows, joined by coop-core's chaos transport
//! and the live adapters. No viewer, no game data (generated test worlds).
//!
//! Every interaction is a guest's act applied once on the host, with the
//! result replicated as shadow rows, shared domains or shared records.

use coop_core::ids::{CellId, FormId as CForm, PlayerId};
use coop_core::interact::{
    Act, CompanionCmd, PipOp, Shared, StealMode, TrapOutcome, Why, DROP_BASE,
};
use coop_core::proto::Msg;
use coop_core::rules::{Prompt, RulesMsg};
use coop_core::sim_test::Sim;
use coop_core::sync::GuestPhase;
use coop_core::transport::{ChaosProfile, PROFILE_NAMES};
use esm::{ActivePlugins, FormId, LoadOrder};
use testdata::{group, placed, record, stat, sub, zstr, TempData};
use world::coop::apply::{Ctx, Opts};
use world::coop::interact::Route;
use world::coop::live::{LiveGuestAdapter, LiveHostAdapter};
use world::coop::{Authority, GuestBridge, HostBridge, ShadowPlan, WorldBackend};
use world::dialogue::PLAYER_REF;
use world::scripting::{GameState, ScriptCache};

mod ids {
    pub const DOOR: u32 = 0xA00;
    pub const KEY: u32 = 0xA01;
    pub const ITEM_A: u32 = 0xA10;
    pub const ITEM_B: u32 = 0xA11;
    pub const CAPS: u32 = 0xF;
    pub const CELL: u32 = 0xD00;
    pub const SWING: u32 = 0xD10;
    pub const LOCKED: u32 = 0xD11;
    pub const PILE: u32 = 0xD20;
    pub const MINE: u32 = 0xD30;
    pub const TERMINAL: u32 = 0xD40;
    pub const CELL2: u32 = 0xD01;
    pub const MARKER: u32 = 0xD60;
    pub const VENDOR: u32 = 0x7000;
    pub const BODY: u32 = 0x7001;
    pub const MATE: u32 = 0x7002;
}
use ids::*;

fn f(n: u32) -> FormId {
    FormId(n)
}

fn c(n: u32) -> CForm {
    CForm(n)
}

/// A room with a swing door, a locked door, a pile of item A, a mine and a
/// terminal; item and caps bases.
fn world(tag: &str) -> TempData {
    let data = TempData::empty(tag);
    let edid = |s: &str| sub(b"EDID", &zstr(s));
    let named = |kind: &[u8; 4], id: u32, name: &str, rest: &[u8]| {
        let mut d = edid(name);
        d.extend(rest);
        record(kind, id, &d)
    };
    let mut hedr = 1.34f32.to_le_bytes().to_vec();
    hedr.extend([0; 8]);
    let mut plugin = record(b"TES4", 0, &sub(b"HEDR", &hedr));
    plugin.extend(group(
        *b"DOOR",
        0,
        &named(b"DOOR", DOOR, "IxDoor", &sub(b"FNAM", &[0])),
    ));
    plugin.extend(group(*b"KEYM", 0, &named(b"KEYM", KEY, "IxKey", &[])));
    let mut misc = named(b"MISC", ITEM_A, "IxItemA", &[]);
    misc.extend(named(b"MISC", ITEM_B, "IxItemB", &[]));
    misc.extend(named(b"MISC", CAPS, "Caps001", &[]));
    plugin.extend(group(*b"MISC", 0, &misc));
    plugin.extend(group(*b"STAT", 0, &stat(0xA20, "IxStat", "")));
    let mut refs = placed(SWING, DOOR, [0.0, 0.0, 0.0], [0.0; 3], &[]);
    refs.extend(placed(LOCKED, DOOR, [100.0, 0.0, 0.0], [0.0; 3], &[]));
    refs.extend(placed(PILE, ITEM_A, [0.0, 100.0, 0.0], [0.0; 3], &[]));
    refs.extend(placed(MINE, 0xA20, [0.0, 200.0, 0.0], [0.0; 3], &[]));
    refs.extend(placed(TERMINAL, 0xA20, [0.0, 300.0, 0.0], [0.0; 3], &[]));
    let mut cell = named(b"CELL", CELL, "IxRoom", &sub(b"DATA", &[1]));
    cell.extend(group(
        CELL.to_le_bytes(),
        6,
        &group(CELL.to_le_bytes(), 8, &refs),
    ));
    let marker = placed(
        MARKER,
        0xA20,
        [700.0, 0.0, 0.0],
        [0.0; 3],
        &sub(b"EDID", &zstr("IxMarker")),
    );
    let mut cell2 = named(b"CELL", CELL2, "IxRoom2", &sub(b"DATA", &[1]));
    cell2.extend(group(
        CELL2.to_le_bytes(),
        6,
        &group(CELL2.to_le_bytes(), 8, &marker),
    ));
    let mut cells = cell;
    cells.extend(cell2);
    plugin.extend(group(
        *b"CELL",
        0,
        &group([0; 4], 2, &group([0; 4], 3, &cells)),
    ));
    data.write("FalloutNV.esm", &plugin);
    data
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
        r
    }

    fn cx(&self) -> Ctx<'a> {
        Ctx {
            order: self.order,
            scripts: self.scripts,
        }
    }

    /// Everyone stands in `cell` at the given x positions (host, g1, g2).
    fn stand(&mut self, cell: u32, xs: [f32; 3]) {
        for (i, g) in std::iter::once(&mut self.host)
            .chain(self.guests.iter_mut())
            .enumerate()
        {
            g.player_cell = Some(f(cell));
            g.player_position = Some([xs[i], 1000.0, 0.0]);
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

    /// Gives a player items through the host's shadow of them.
    fn give(&mut self, slot: u8, item: u32, n: i32) {
        self.host.with_player(slot, |g| {
            g.stocked.insert(PLAYER_REF);
            *g.items.entry((PLAYER_REF, f(item))).or_insert(0) += n;
        });
    }

    /// What the host's shadow of `slot` holds.
    fn held(&self, slot: u8, item: u32) -> i32 {
        self.host
            .shadow_of(slot)
            .and_then(|s| s.items.get(&f(item)).copied())
            .unwrap_or(0)
    }

    /// What guest `i` sees in its own pack.
    fn mine(&self, i: usize, item: u32) -> i32 {
        self.guests[i]
            .items
            .get(&(PLAYER_REF, f(item)))
            .copied()
            .unwrap_or(0)
    }

    fn vendor_stock(&mut self, item: u32, n: i32) {
        self.host.stocked.insert(f(VENDOR));
        self.host.items.insert((f(VENDOR), f(item)), n);
        self.host.emit(world::coop::Domain::Stocked, f(VENDOR));
        self.host.emit(world::coop::Domain::Items, f(VENDOR));
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

    fn settled(&mut self) -> bool {
        let h = self.host.shard_hashes();
        self.guests.iter().all(|g| g.shard_hashes() == h)
    }
}

fn setup<'a>(order: &'a LoadOrder, scripts: &'a ScriptCache, profile: &str, seed: u64) -> Rig<'a> {
    let mut r = Rig::new(order, scripts, profile, seed);
    r.stand(CELL, [0.0, 10.0, 20.0]);
    r.run_ms(500);
    r
}

fn load(tag: &str) -> (TempData, LoadOrder) {
    let data = world(tag);
    let order = LoadOrder::from_data_dir(data.path(), &ActivePlugins::OfficialOnly).unwrap();
    (data, order)
}

fn all_profiles() -> Vec<&'static str> {
    let mut v = vec!["perfect"];
    v.extend(PROFILE_NAMES.iter().copied());
    v
}

#[test]
fn a_guest_barter_moves_vendor_stock_and_the_guests_own_pack_once() {
    for (n, profile) in all_profiles().into_iter().enumerate() {
        let (_d, order) = load(&format!("ix-barter-{n}"));
        let scripts = ScriptCache::default();
        let mut r = setup(&order, &scripts, profile, 300 + n as u64);
        r.vendor_stock(ITEM_A, 5);
        r.give(1, CAPS, 50);
        r.give(1, ITEM_B, 3);
        assert!(
            r.until(60_000, |r| r.mine(0, CAPS) == 50 && r.mine(0, ITEM_B) == 3),
            "{profile}: guest sees its pack"
        );
        // buys 2 of A, sells 1 of B, pays 30 caps
        let act = Act::Barter {
            vendor: c(VENDOR),
            buys: vec![(c(ITEM_A), 2)],
            sells: vec![(c(ITEM_B), 1)],
            total: -30,
        };
        assert_eq!(r.guests[0].clone().coop_route(act.clone()), Route::Remote);
        // the request goes out twice (a resend): it must not apply twice
        let dup = r.sim.guests[0].sync.live.interact.request(act);
        r.sim.guest_emit(0, vec![dup.clone(), dup]);
        assert!(
            r.until(90_000, |r| r.mine(0, ITEM_A) == 2
                && r.guests[1].items.get(&(f(VENDOR), f(ITEM_A))) == Some(&3)),
            "{profile}: trade applied"
        );
        r.run_ms(3_000);
        assert_eq!(
            r.held(1, ITEM_A),
            2,
            "{profile}: host's shadow of guest 1 bought 2"
        );
        assert_eq!(r.held(1, CAPS), 20, "{profile}: paid 30 once");
        assert_eq!(r.held(1, ITEM_B), 2, "{profile}: sold 1");
        assert_eq!(
            r.host.items.get(&(f(VENDOR), f(ITEM_A))),
            Some(&3),
            "{profile}: vendor stock"
        );
        assert_eq!(
            r.host.items.get(&(f(VENDOR), f(ITEM_B))),
            Some(&1),
            "{profile}: vendor received"
        );
        assert_eq!(
            r.host.items.get(&(f(VENDOR), f(CAPS))),
            Some(&30),
            "{profile}: vendor caps"
        );
        assert_eq!(r.held(2, CAPS), 0, "{profile}: guest 2 untouched");
        assert_eq!(
            r.guests[1].items.get(&(f(VENDOR), f(CAPS))),
            Some(&30),
            "{profile}: shared vendor caps seen by guest 2"
        );
        assert!(
            r.until(30_000, |r| r.mine(0, CAPS) == 20),
            "{profile}: guest 1 pack"
        );
        assert!(
            r.results(0)
                .iter()
                .filter(|x| x.0 == "barter" && x.1)
                .count()
                >= 1,
            "{profile}: verdict"
        );
        assert!(r.until(30_000, |r| r.settled()), "{profile}: converged");
        // too much: refused, nothing moves
        let act = Act::Barter {
            vendor: c(VENDOR),
            buys: vec![(c(ITEM_A), 9)],
            sells: vec![],
            total: 0,
        };
        r.guests[0].coop_route(act);
        r.run_ms(4_000);
        assert!(
            r.results(0)
                .iter()
                .any(|x| x.0 == "barter" && !x.1 && x.2 == Why::Insufficient),
            "{profile}: refused"
        );
        assert_eq!(
            r.host.items.get(&(f(VENDOR), f(ITEM_A))),
            Some(&3),
            "{profile}: unchanged"
        );
        // the host's own trade goes through the same executor
        r.host.coop_do(
            r.cx(),
            Act::Barter {
                vendor: c(VENDOR),
                buys: vec![(c(ITEM_A), 1)],
                sells: vec![],
                total: 0,
            },
        );
        assert!(
            r.until(30_000, |r| r.guests[1].items.get(&(f(VENDOR), f(ITEM_A)))
                == Some(&2)),
            "{profile}: host trade replicates"
        );
    }
}

#[test]
fn a_world_item_goes_to_exactly_one_of_two_simultaneous_takers() {
    for (n, profile) in all_profiles().into_iter().enumerate() {
        let (_d, order) = load(&format!("ix-pickup-{n}"));
        let scripts = ScriptCache::default();
        let mut r = setup(&order, &scripts, profile, 400 + n as u64);
        let act = Act::Pickup {
            obj: c(PILE),
            item: c(ITEM_A),
            count: 1,
        };
        r.guests[0].coop_route(act.clone());
        r.guests[1].coop_route(act);
        assert!(
            r.until(90_000, |r| r.results(0).len() == 1
                && r.results(1).len() == 1),
            "{profile}: both answered"
        );
        let wins = r
            .results(0)
            .iter()
            .chain(r.results(1).iter())
            .filter(|x| x.1)
            .count();
        assert_eq!(wins, 1, "{profile}: one winner");
        assert_eq!(
            r.held(1, ITEM_A) + r.held(2, ITEM_A),
            1,
            "{profile}: the item exists once"
        );
        assert!(
            r.until(30_000, |r| r.host.disabled.get(&f(PILE)) == Some(&true)
                && r.guests
                    .iter()
                    .all(|g| g.disabled.get(&f(PILE)) == Some(&true))),
            "{profile}: gone everywhere"
        );
        let loser = if r.results(0)[0].1 { 1 } else { 0 };
        assert_eq!(
            r.results(loser)[0].2,
            Why::Gone,
            "{profile}: loser told it is gone"
        );
        // the host cannot take it either
        assert!(r.until(30_000, |r| r.settled()), "{profile}: converged");
    }
}

#[test]
fn a_dropped_item_can_be_taken_by_another_player_with_the_right_count() {
    let (_d, order) = load("ix-drop");
    let scripts = ScriptCache::default();
    let mut r = setup(&order, &scripts, "bad_wifi", 41);
    r.give(1, ITEM_B, 5);
    assert!(r.until(60_000, |r| r.mine(0, ITEM_B) == 5));
    r.guests[0].coop_route(Act::Drop {
        item: c(ITEM_B),
        count: 3,
        cell: CellId(CELL),
        pos: [5, 6, 7],
    });
    assert!(r.until(90_000, |r| r.guests.iter().all(|g| g
        .coop
        .interact
        .placed
        .len()
        == 1)
        && !r.host.coop.interact.placed.is_empty()));
    let note = r.guests[1].coop.interact.placed[0];
    assert!(note.drop.0 >= DROP_BASE);
    assert_eq!((note.count, note.pos), (3, [5, 6, 7]));
    assert_eq!(r.held(1, ITEM_B), 2);
    // guest 2 takes 2 of the 3, then guest 1 the last one
    r.guests[1].coop_route(Act::Pickup {
        obj: CForm(note.drop.0),
        item: c(ITEM_B),
        count: 2,
    });
    assert!(r.until(60_000, |r| r.held(2, ITEM_B) == 2));
    assert_ne!(
        r.host.disabled.get(&note.drop),
        Some(&true),
        "part of a pile: the pile stays"
    );
    r.guests[0].coop_route(Act::Pickup {
        obj: CForm(note.drop.0),
        item: c(ITEM_B),
        count: 5,
    });
    assert!(r.until(60_000, |r| r.held(1, ITEM_B) == 3));
    assert_eq!(
        r.held(1, ITEM_B) + r.held(2, ITEM_B),
        5,
        "nothing lost, nothing made"
    );
    assert!(r.until(30_000, |r| r.host.disabled.get(&note.drop) == Some(&true)));
    // host drops are minted by the host bridge too
    r.host.coop_route(Act::Drop {
        item: c(ITEM_A),
        count: 1,
        cell: CellId(CELL),
        pos: [0, 0, 0],
    });
    r.run_ms(2_000);
    assert!(r.results(0).iter().all(|x| x.0 != "drop" || x.1));
}

#[test]
fn lockpick_hack_key_and_traps_are_shared_and_cost_the_acting_player() {
    for (n, profile) in all_profiles().into_iter().enumerate() {
        let (_d, order) = load(&format!("ix-locks-{n}"));
        let scripts = ScriptCache::default();
        let mut r = setup(&order, &scripts, profile, 500 + n as u64);
        r.host.locks.insert(f(LOCKED), Some(2));
        r.host.locks.insert(f(TERMINAL), Some(1));
        r.give(1, KEY, 1);
        // guest 1 plays the lockpick game locally and reports the win
        r.guests[0].locks.insert(f(LOCKED), Some(2));
        assert_eq!(
            r.guests[0].coop_route(Act::Lockpick {
                obj: c(LOCKED),
                won: true,
                pins: 2,
                broke: false
            }),
            Route::Remote
        );
        assert!(
            r.until(90_000, |r| r.host.locks.get(&f(LOCKED)) == Some(&None)
                && r.guests
                    .iter()
                    .all(|g| g.locks.get(&f(LOCKED)) == Some(&None))),
            "{profile}: unlocked for everyone"
        );
        // a lost forcing breaks the other lock for everyone
        r.host.locks.insert(f(SWING), Some(3));
        r.guests[1].coop_route(Act::Lockpick {
            obj: c(SWING),
            won: false,
            pins: 1,
            broke: true,
        });
        assert!(
            r.until(90_000, |r| r.guests.iter().all(|g| g
                .broken_locks
                .get(&f(SWING))
                .copied()
                .unwrap_or(0)
                > 0)),
            "{profile}: broken for everyone"
        );
        // the hacking game: won on one machine, a terminal for all
        r.guests[1].coop_route(Act::Hack {
            obj: c(TERMINAL),
            won: true,
        });
        assert!(
            r.until(90_000, |r| r
                .host
                .terminal_states
                .get(&f(TERMINAL))
                .is_some_and(|t| t.hacked)
                && r.guests[0]
                    .terminal_states
                    .get(&f(TERMINAL))
                    .is_some_and(|t| t.hacked)),
            "{profile}: hacked"
        );
        r.guests[0].coop_route(Act::Hack {
            obj: c(0xD41),
            won: false,
        });
        assert!(
            r.until(90_000, |r| r.guests[1]
                .terminal_states
                .get(&f(0xD41))
                .is_some_and(|t| t.lockouts == 1)),
            "{profile}: lockout shared"
        );
        // a mine goes off exactly once however many report it
        r.guests[0].coop_route(Act::Trap {
            obj: c(MINE),
            what: TrapOutcome::Detonated,
        });
        r.guests[1].coop_route(Act::Trap {
            obj: c(MINE),
            what: TrapOutcome::Detonated,
        });
        assert!(
            r.until(90_000, |r| r.guests.iter().all(|g| g
                .more
                .mines
                .gone
                .contains(&f(MINE)))),
            "{profile}: mine gone"
        );
        r.run_ms(3_000);
        assert_eq!(
            r.host.coop.interact.detonations.len(),
            1,
            "{profile}: detonated once"
        );
        let refused: usize = (0..2)
            .map(|i| {
                r.results(i)
                    .iter()
                    .filter(|x| x.0 == "trap" && !x.1 && x.2 == Why::Gone)
                    .count()
            })
            .sum();
        assert_eq!(
            refused, 1,
            "{profile}: the second report was told it was already gone"
        );
        // destroyed objects replicate through the destroyed domain
        r.guests[0].coop_route(Act::Trap {
            obj: c(PILE),
            what: TrapOutcome::Destroyed,
        });
        assert!(
            r.until(60_000, |r| r.guests[1].destroyed.contains(&f(PILE))),
            "{profile}: destroyed"
        );
        // a key opens its lock (the lock needs a placed key: refused for a wrong one)
        r.guests[0].coop_route(Act::Key {
            obj: c(LOCKED),
            key: c(KEY),
        });
        r.run_ms(3_000);
        assert!(
            r.results(0)
                .iter()
                .any(|x| x.0 == "key" && !x.1 && x.2 == Why::Locked),
            "{profile}: wrong key refused"
        );
        assert!(r.until(30_000, |r| r.settled()), "{profile}: converged");
    }
}

#[test]
fn a_swing_door_flips_once_per_request_and_late_state_is_shared() {
    for (n, profile) in all_profiles().into_iter().enumerate() {
        let (_d, order) = load(&format!("ix-door-{n}"));
        let scripts = ScriptCache::default();
        let mut r = setup(&order, &scripts, profile, 600 + n as u64);
        let act = Act::Door { door: c(SWING) };
        let dup = r.sim.guests[0].sync.live.interact.request(act);
        r.sim.guest_emit(0, vec![dup.clone(), dup.clone(), dup]);
        let open = |g: &GameState| world::doors::is_open(&order, g, f(SWING));
        assert!(
            r.until(90_000, |r| open(&r.host)
                && open(&r.guests[0])
                && open(&r.guests[1])),
            "{profile}: open everywhere"
        );
        r.run_ms(4_000);
        assert!(
            open(&r.host),
            "{profile}: still open (duplicates did not flip it back)"
        );
        // a locked door says no to a guest without the key
        r.host.locks.insert(f(LOCKED), Some(2));
        r.guests[1].coop_route(Act::Door { door: c(LOCKED) });
        assert!(
            r.until(60_000, |r| r
                .results(1)
                .iter()
                .any(|x| x.0 == "door" && !x.1 && x.2 == Why::Locked)),
            "{profile}: locked"
        );
        // a guest can close it again
        r.run_ms(2_000);
        r.host.seconds += 100.0;
        r.guests[1].seconds += 100.0;
        r.guests[0].seconds += 100.0;
        r.guests[1].coop_route(Act::Door { door: c(SWING) });
        assert!(
            r.until(90_000, |r| !open(&r.host) && !open(&r.guests[0])),
            "{profile}: closed"
        );
    }
}

#[test]
fn the_party_follows_the_leader_through_a_load_door_and_non_leaders_are_kept_in_the_group() {
    let (_d, order) = load("ix-transit");
    let scripts = ScriptCache::default();
    let mut r = setup(&order, &scripts, "good_wifi", 71);
    // everyone is in cell 1; cell 2 is linked to it; cell 9 is elsewhere
    r.stand(1, [0.0, 10.0, 20.0]);
    r.sim.host.live.interest.link(CellId(1), CellId(2));
    r.run_ms(1_000);
    // the host (leader) walks through a door: both guests load the destination
    r.host.coop_route(Act::Transition {
        door: c(0xD50),
        from: CellId(1),
        to: CellId(2),
        one_way: false,
    });
    assert!(r.until(60_000, |r| r.guests.iter().all(|g| g
        .coop
        .interact
        .follow
        .is_some())));
    for g in &r.guests {
        let fo = g.coop.interact.follow.unwrap();
        assert_eq!((fo.to, fo.door), (CellId(2), c(0xD50)));
    }
    for g in r.guests.iter_mut() {
        g.coop.interact.follow = None;
    }
    // a guest takes a door into the leader's group: allowed, alone
    r.guests[0].coop_route(Act::Transition {
        door: c(0xD51),
        from: CellId(1),
        to: CellId(2),
        one_way: false,
    });
    assert!(r.until(60_000, |r| r.guests[0].coop.interact.follow.is_some()));
    r.run_ms(1_000);
    assert!(
        r.guests[1].coop.interact.follow.is_none(),
        "nobody else is dragged along by a non-leader"
    );
    // and into a different part of the world: refused
    r.guests[1].coop_route(Act::Transition {
        door: c(0xD52),
        from: CellId(1),
        to: CellId(9),
        one_way: false,
    });
    assert!(r.until(60_000, |r| r
        .results(1)
        .iter()
        .any(|x| x.0 == "transition" && !x.1 && x.2 == Why::Policy)));
    assert!(r.guests[1].coop.interact.follow.is_none());
}

#[test]
fn a_one_way_transition_waits_for_the_whole_party() {
    let (_d, order) = load("ix-barrier");
    let scripts = ScriptCache::default();
    let mut r = setup(&order, &scripts, "perfect", 72);
    r.stand(1, [0.0, 10.0, 20.0]);
    r.run_ms(1_000);
    r.guests[0].coop_route(Act::Transition {
        door: c(0xD60),
        from: CellId(1),
        to: CellId(5),
        one_way: true,
    });
    // everyone is asked; nothing moves until all agree
    assert!(r.until(30_000, |r| r.sim.guests.iter().all(|g| matches!(
        g.sync.live.rules.prompt,
        Some(Prompt::Barrier { .. })
    ))));
    let id = match r.sim.guests[0].sync.live.rules.prompt {
        Some(Prompt::Barrier { id, .. }) => id,
        _ => unreachable!(),
    };
    r.run_ms(500);
    assert!(
        r.guests.iter().all(|g| g.coop.interact.follow.is_none()),
        "waiting"
    );
    r.sim
        .guest_emit(0, vec![Msg::Rules(RulesMsg::BarrierReady { id })]);
    r.run_ms(500);
    assert!(
        r.guests.iter().all(|g| g.coop.interact.follow.is_none()),
        "one yes is not enough"
    );
    r.sim
        .guest_emit(1, vec![Msg::Rules(RulesMsg::BarrierReady { id })]);
    r.sim.host.live.rules.on_msg(
        PlayerId::HOST,
        RulesMsg::BarrierReady { id },
        r.sim.clock.now(),
        &|_, _| None,
    );
    assert!(
        r.until(60_000, |r| r.guests.iter().all(|g| g
            .coop
            .interact
            .follow
            .is_some())),
        "released"
    );
    assert!(r.results(0).iter().any(|x| x.0 == "transition" && x.1));
    // a decline cancels
    for g in r.guests.iter_mut() {
        g.coop.interact.follow = None;
    }
    r.guests[1].coop_route(Act::Transition {
        door: c(0xD61),
        from: CellId(1),
        to: CellId(6),
        one_way: true,
    });
    assert!(r.until(30_000, |r| matches!(r.sim.guests[0].sync.live.rules.prompt, Some(Prompt::Barrier { id: i, .. }) if i != id)));
    let id2 = match r.sim.guests[0].sync.live.rules.prompt {
        Some(Prompt::Barrier { id, .. }) => id,
        _ => unreachable!(),
    };
    r.sim
        .guest_emit(0, vec![Msg::Rules(RulesMsg::BarrierDecline { id: id2 })]);
    assert!(r.until(60_000, |r| r
        .results(1)
        .iter()
        .any(|x| x.0 == "transition" && !x.1 && x.2 == Why::Policy)));
    assert!(
        r.guests.iter().all(|g| g.coop.interact.follow.is_none()),
        "nobody moved"
    );
}

#[test]
fn stealing_item_use_equipment_and_companions_are_per_player() {
    let (_d, order) = load("ix-personal");
    let scripts = ScriptCache::default();
    let mut r = setup(&order, &scripts, "good_wifi", 81);
    r.host.stocked.insert(f(BODY));
    r.host.items.insert((f(BODY), f(ITEM_A)), 4);
    r.host.dead.insert(f(BODY));
    r.host.emit(world::coop::Domain::Items, f(BODY));
    r.host.emit(world::coop::Domain::Destroyed, f(BODY));
    r.host.emit(world::coop::Domain::Stocked, f(BODY));
    r.give(1, ITEM_B, 2);
    r.give(2, ITEM_B, 1);
    r.run_ms(2_000);
    // loot a body outside the container menu, pickpocket a living man, plant on him
    r.guests[0].coop_route(Act::Steal {
        target: c(BODY),
        item: c(ITEM_A),
        count: 3,
        mode: StealMode::Loot,
    });
    r.guests[1].coop_route(Act::Steal {
        target: c(VENDOR),
        item: c(ITEM_A),
        count: 1,
        mode: StealMode::Loot,
    });
    r.guests[1].coop_route(Act::Steal {
        target: c(BODY),
        item: c(ITEM_A),
        count: 1,
        mode: StealMode::Pickpocket,
    });
    assert!(r.until(60_000, |r| r.held(1, ITEM_A) == 3 && r.held(2, ITEM_A) == 1));
    assert_eq!(
        r.host
            .items
            .get(&(f(BODY), f(ITEM_A)))
            .copied()
            .unwrap_or(0),
        0
    );
    assert!(
        r.until(30_000, |r| r
            .results(1)
            .iter()
            .any(|x| x.0 == "steal" && !x.1 && x.2 == Why::Policy)),
        "looting the living is refused"
    );
    r.guests[0].coop_route(Act::Steal {
        target: c(VENDOR),
        item: c(ITEM_B),
        count: 1,
        mode: StealMode::Plant,
    });
    assert!(r.until(60_000, |r| r.held(1, ITEM_B) == 1
        && r.host.items.get(&(f(VENDOR), f(ITEM_B))) == Some(&1)));
    // equipment, ammo, hot keys and notes
    r.guests[0].coop_route(Act::Equip {
        item: c(ITEM_A),
        on: true,
    });
    r.guests[0].coop_route(Act::Ammo { ammo: c(ITEM_B) });
    r.guests[0].coop_route(Act::Pip(PipOp::Hotkey {
        slot: 2,
        item: c(ITEM_A),
    }));
    r.guests[1].coop_route(Act::Pip(PipOp::Note(c(0xE00))));
    r.guests[1].coop_route(Act::Equip {
        item: c(ITEM_B),
        on: false,
    });
    r.guests[0].coop_route(Act::Equip {
        item: c(0xFFFF),
        on: true,
    });
    assert!(
        r.until(60_000, |r| r.host.shadow_of(1).is_some_and(|s| s.hotkeys
            [2]
            == Some(f(ITEM_A))
            && s.equipped.as_ref().is_some_and(|e| e.contains(&f(ITEM_A)))
            && s.ammo_loaded == Some(f(ITEM_B))))
    );
    assert!(r
        .host
        .shadow_of(2)
        .is_some_and(|s| s.hotkeys[2].is_none() && s.ammo_loaded.is_none()));
    assert!(
        r.until(60_000, |r| r
            .guests
            .iter()
            .all(|g| g.notes.contains(&f(0xE00)))),
        "notes are shared"
    );
    assert!(
        r.results(0).iter().any(|x| x.0 == "equip" && !x.1),
        "cannot wear what you do not have"
    );
    // companions belong to one player
    r.guests[0].coop_route(Act::Companion {
        who: c(MATE),
        cmd: CompanionCmd::Recruit,
    });
    assert!(r.until(60_000, |r| r
        .host
        .shadow_of(1)
        .is_some_and(|s| s.teammates.contains(&f(MATE)))));
    r.guests[1].coop_route(Act::Companion {
        who: c(MATE),
        cmd: CompanionCmd::Recruit,
    });
    r.guests[1].coop_route(Act::Companion {
        who: c(MATE),
        cmd: CompanionCmd::Wait,
    });
    assert!(r.until(60_000, |r| r
        .results(1)
        .iter()
        .filter(|x| x.0 == "companion" && !x.1)
        .count()
        == 2));
    r.guests[0].coop_route(Act::Companion {
        who: c(MATE),
        cmd: CompanionCmd::Dismiss,
    });
    assert!(r.until(60_000, |r| r
        .host
        .shadow_of(1)
        .is_some_and(|s| !s.teammates.contains(&f(MATE)))));
    // discovering a map marker unlocks fast travel for the whole party
    r.guests[1].coop_route(Act::Discover { marker: c(0xE10) });
    assert!(r.until(60_000, |r| r.host.map_markers.contains(&f(0xE10))
        && r.guests[0].map_markers.contains(&f(0xE10))));
    assert!(r.until(30_000, |r| r.settled()));
}

#[test]
fn a_late_joiner_to_shared_world_state_is_covered_by_apply_and_sync() {
    // the log is replayed by the host to a joiner; here the receiving end
    // applies every kind idempotently
    let (_d, order) = load("ix-apply");
    let scripts = ScriptCache::default();
    let cx = Ctx {
        order: &order,
        scripts: &scripts,
    };
    let mut g = GameState::new(&order);
    g.set_authority(Authority::Guest);
    let shared = vec![
        Shared::Door {
            door: c(SWING),
            open: true,
        },
        Shared::Lock {
            obj: c(LOCKED),
            state: coop_core::interact::LockState {
                level: None,
                broken: true,
            },
        },
        Shared::Terminal {
            obj: c(TERMINAL),
            hacked: true,
            lockouts: 2,
        },
        Shared::Mine {
            obj: c(MINE),
            phase: coop_core::interact::MinePhase::Gone,
        },
        Shared::Placed {
            drop: CForm(DROP_BASE + 1),
            item: c(ITEM_A),
            count: 2,
            cell: CellId(CELL),
            space: CellId(CELL),
            pos: [1, 2, 3],
        },
    ];
    for _ in 0..3 {
        world::coop::interact::apply_all(cx, &mut g, &shared);
    }
    assert!(world::doors::is_open(&order, &g, f(SWING)));
    assert_eq!(g.locks.get(&f(LOCKED)), Some(&None));
    assert_eq!(
        g.broken_locks.get(&f(LOCKED)),
        Some(&1),
        "broken once, not three times"
    );
    assert_eq!(
        g.terminal_states
            .get(&f(TERMINAL))
            .map(|t| (t.hacked, t.lockouts)),
        Some((true, 2))
    );
    assert!(g.more.mines.gone.contains(&f(MINE)));
    assert_eq!(g.coop.interact.placed.len(), 1, "announced once");
    assert_eq!(
        g.coop.interact.counters_json().contains("\"placed\":1"),
        true
    );
}

#[test]
fn what_the_host_world_does_on_its_own_reaches_the_guests_without_an_act() {
    // a script's Lock, the host player's own door, a quest's lock-out, a mine
    // that went off: nobody announces them, the host sweeps and publishes
    for (n, profile) in all_profiles().into_iter().enumerate() {
        let (_d, order) = load(&format!("ix-sweep-{n}"));
        let scripts = ScriptCache::default();
        let mut r = setup(&order, &scripts, profile, 1300 + n as u64);
        r.host.locks.insert(f(LOCKED), Some(3));
        world::doors::activate(&order, &mut r.host, f(SWING), None);
        world::terminal::lock_out(&mut r.host, f(TERMINAL));
        world::mines::gone(&mut r.host, f(MINE));
        let open = |g: &GameState| world::doors::is_open(&order, g, f(SWING));
        assert!(
            r.until(90_000, |r| r.guests.iter().all(|g| g.locks.get(&f(LOCKED))
                == Some(&Some(3))
                && open(g)
                && g.terminal_states
                    .get(&f(TERMINAL))
                    .is_some_and(|t| t.lockouts == 1)
                && g.more.mines.gone.contains(&f(MINE)))),
            "{profile}: guests follow the host world"
        );
        // and a later change (the lock opened by a script) follows too
        r.host.locks.insert(f(LOCKED), None);
        assert!(
            r.until(60_000, |r| r
                .guests
                .iter()
                .all(|g| g.locks.get(&f(LOCKED)) == Some(&None))),
            "{profile}: unlocked"
        );
    }
}

#[test]
fn a_script_that_moves_a_player_brings_the_party_or_that_guest() {
    use world::scripting::Runner;
    let (_d, order) = load("ix-scriptmove");
    let scripts = ScriptCache::default();
    let mut r = setup(&order, &scripts, "good_wifi", 1400);
    r.stand(CELL, [0.0, 10.0, 20.0]);
    r.sim.host.live.interest.link(CellId(CELL), CellId(CELL2));
    r.run_ms(1_000);
    // the host's own MoveTo: both guests, standing with the host, are told
    Runner::new(&order, &scripts, &mut r.host).run_source("player.MoveTo IxMarker", None, None);
    assert_eq!(r.host.player_cell, Some(f(CELL2)));
    assert!(r.until(60_000, |r| r.guests.iter().all(|g| g
        .coop
        .interact
        .follow
        .is_some_and(|x| x.to == CellId(CELL2) && x.door == c(MARKER)))));
    assert_eq!(r.host.coop.interact.done.get("script_move"), Some(&1));
    // a script run for guest 2 (its shadow) moves only guest 2
    for g in r.guests.iter_mut() {
        g.coop.interact.follow = None;
    }
    r.host.with_player(2, |g| {
        Runner::new(&order, &scripts, g).run_source("player.MoveTo IxMarker", None, None);
    });
    assert!(r.until(60_000, |r| r.guests[1].coop.interact.follow.is_some()));
    r.run_ms(1_000);
    assert!(r.guests[0].coop.interact.follow.is_none());
}

#[test]
fn single_player_pays_nothing() {
    let (_d, order) = load("ix-single");
    let scripts = ScriptCache::default();
    let mut s = GameState::new(&order);
    assert_eq!(s.coop_route(Act::Door { door: c(SWING) }), Route::Local);
    assert!(s.coop.interact.acts.is_empty());
    // coop_do on single player runs the same executor on the one player
    s.stocked.insert(PLAYER_REF);
    s.items.insert((PLAYER_REF, f(ITEM_A)), 2);
    let cx = Ctx {
        order: &order,
        scripts: &scripts,
    };
    assert_eq!(
        s.coop_do(
            cx,
            Act::Equip {
                item: c(ITEM_A),
                on: true
            }
        ),
        Route::Local
    );
    assert!(s.is_equipped(PLAYER_REF, f(ITEM_A)));
    assert_eq!(s.coop.interact.done.get("equip"), Some(&1));
    // a host's own acts are queued for publishing
    s.set_authority(Authority::Host);
    assert_eq!(s.coop_route(Act::Door { door: c(SWING) }), Route::Local);
    assert_eq!(
        s.coop_route(Act::Drop {
            item: c(ITEM_A),
            count: 1,
            cell: CellId(1),
            pos: [0; 3]
        }),
        Route::Remote
    );
    assert_eq!(s.coop.interact.acts.len(), 2);
}

#[test]
fn bot_lines_parse_into_acts() {
    let form = |w: &str| u32::from_str_radix(w, 16).ok().map(FormId);
    let p = |line: &str| {
        let words: Vec<&str> = line.split_whitespace().collect();
        world::coop::interact::parse_act(&words, &form, (7, [1, 2, 3]))
    };
    assert_eq!(
        p("barter 7000 -30 buy:A10:2 sell:A11"),
        Some(Act::Barter {
            vendor: c(0x7000),
            buys: vec![(c(0xA10), 2)],
            sells: vec![(c(0xA11), 1)],
            total: -30
        })
    );
    assert_eq!(
        p("drop A11 2"),
        Some(Act::Drop {
            item: c(0xA11),
            count: 2,
            cell: CellId(7),
            pos: [1, 2, 3]
        })
    );
    assert_eq!(
        p("pickup D20 A10"),
        Some(Act::Pickup {
            obj: c(0xD20),
            item: c(0xA10),
            count: 1
        })
    );
    assert_eq!(
        p("lockpick D11 won 2"),
        Some(Act::Lockpick {
            obj: c(0xD11),
            won: true,
            pins: 2,
            broke: false
        })
    );
    assert_eq!(
        p("lockpick D11 broke"),
        Some(Act::Lockpick {
            obj: c(0xD11),
            won: false,
            pins: 1,
            broke: true
        })
    );
    assert_eq!(
        p("trap D30 detonate"),
        Some(Act::Trap {
            obj: c(0xD30),
            what: TrapOutcome::Detonated
        })
    );
    assert_eq!(
        p("enter D50 1 2 oneway"),
        Some(Act::Transition {
            door: c(0xD50),
            from: CellId(1),
            to: CellId(2),
            one_way: true
        })
    );
    assert_eq!(
        p("companion 7002 gear"),
        Some(Act::Companion {
            who: c(0x7002),
            cmd: CompanionCmd::SortGear
        })
    );
    assert_eq!(
        p("steal 7001 A10 2 loot"),
        Some(Act::Steal {
            target: c(0x7001),
            item: c(0xA10),
            count: 2,
            mode: StealMode::Loot
        })
    );
    assert!(p("door").is_none());
    assert!(p("unknownverb 1").is_none());
}

#[test]
fn a_soak_of_mixed_interactions_converges_under_every_chaos_profile() {
    for (n, profile) in all_profiles().into_iter().enumerate() {
        for seed in 0..2u64 {
            let (_d, order) = load(&format!("ix-soak-{n}-{seed}"));
            let scripts = ScriptCache::default();
            let mut r = setup(&order, &scripts, profile, 900 + n as u64 * 7 + seed);
            r.vendor_stock(ITEM_A, 20);
            r.give(1, CAPS, 200);
            r.give(2, CAPS, 200);
            assert!(
                r.until(90_000, |r| r.mine(0, CAPS) == 200 && r.mine(1, CAPS) == 200),
                "{profile}"
            );
            for round in 0..4 {
                for i in 0..2 {
                    r.guests[i].coop_route(Act::Barter {
                        vendor: c(VENDOR),
                        buys: vec![(c(ITEM_A), 1)],
                        sells: vec![],
                        total: -5,
                    });
                }
                r.guests[round % 2].coop_route(Act::Door { door: c(SWING) });
                r.host.seconds += 50.0;
                for g in r.guests.iter_mut() {
                    g.seconds += 50.0;
                }
                r.run_ms(1_500);
            }
            assert!(
                r.until(120_000, |r| r.held(1, ITEM_A) == 4
                    && r.held(2, ITEM_A) == 4),
                "{profile}: both bought four"
            );
            assert_eq!(
                r.host.items.get(&(f(VENDOR), f(ITEM_A))),
                Some(&12),
                "{profile}: vendor stock exact"
            );
            assert_eq!(
                r.host.items.get(&(f(VENDOR), f(CAPS))),
                Some(&40),
                "{profile}: vendor caps exact"
            );
            assert!(
                r.until(120_000, |r| r.settled()),
                "{profile}: shared hashes converge"
            );
        }
    }
}

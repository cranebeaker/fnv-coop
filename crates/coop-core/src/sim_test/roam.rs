//! Multi-cell scenarios on the chaos simulator: players in different cells,
//! ownership and interest per loaded cell, adoption of state on entering a
//! cell, dormant actors, presence and late-join placement.

use std::collections::BTreeMap;

use super::live::{all_profiles, at, Rig, G1, G2, H};
use super::*;
use crate::actors::{ActorMsg, ActorState};
use crate::avatar::{quant_pos, AvatarMsg, AvatarState};
use crate::ids::{CellId, Seq};

const A: FormId = FormId(0x100);
const B: FormId = FormId(0x200);
const C: FormId = FormId(0x300);
const D: FormId = FormId(0x400);

/// A rig whose owners publish each actor with its own state.
struct SceneRig {
    r: Rig,
    states: BTreeMap<FormId, ActorState>,
    seq: u32,
}

fn st(cell: u32, x: f32, health: u16) -> ActorState {
    ActorState {
        cell: CellId(cell),
        pos: [quant_pos(x), 0, 0],
        health,
        ..Default::default()
    }
}

impl SceneRig {
    fn new(profile: ChaosProfile, seed: u64) -> SceneRig {
        SceneRig {
            r: Rig::new(profile, seed),
            states: BTreeMap::new(),
            seq: 0,
        }
    }

    fn actor(&mut self, f: FormId, s: ActorState) {
        self.r.sim.host.live.actors.register(f, s);
        self.states.insert(f, s);
    }

    fn step(&mut self) {
        let r = &mut self.r;
        if r.sim.steps % STEPS_PER_TICK == 0 {
            self.seq += 1;
            let m = Msg::Avatar(AvatarMsg::State {
                player: H,
                seq: Seq(self.seq),
                state: r.host_state,
            });
            r.sim.host_local(m);
            for a in r.host_actors.owned() {
                if let Some(m) = r.host_actors.publish(a, self.states[&a]) {
                    r.sim.host_local(Msg::Actor(m));
                }
            }
            for i in 0..r.sim.guests.len() {
                for a in r.sim.guests[i].sync.live.actors.owned() {
                    if let Some(m) = r.sim.guests[i].sync.live.actors.publish(a, self.states[&a]) {
                        r.sim.guest_emit(i, vec![Msg::Actor(m)]);
                    }
                }
            }
        }
        r.sim.step();
        for m in r.sim.host.live.take_local() {
            if let Msg::Actor(a) = m {
                r.host_actors.on_msg(a);
            }
        }
    }

    fn run_until(&mut self, ms: u64, mut cond: impl FnMut(&mut SceneRig) -> bool) -> bool {
        let end = self.r.sim.clock.now() + ms;
        while self.r.sim.clock.now() < end {
            if cond(self) {
                return true;
            }
            self.step();
        }
        cond(self)
    }

    fn run_ms(&mut self, ms: u64) {
        let end = self.r.sim.clock.now() + ms;
        while self.r.sim.clock.now() < end {
            self.step();
        }
    }
}

#[test]
fn actors_are_owned_in_their_own_cell_and_unknown_elsewhere() {
    for (n, profile) in all_profiles().into_iter().enumerate() {
        let name = profile.name;
        let mut s = SceneRig::new(profile, 2000 + n as u64);
        s.actor(A, st(1, 100.0, 100));
        s.actor(B, st(2, 100.0, 100));
        s.actor(C, st(3, 100.0, 100));
        s.actor(D, st(4, 100.0, 100));
        s.r.host_state = at(1, 0.0);
        s.r.g(0).live.local = at(2, 0.0);
        s.r.g(1).live.local = at(3, 0.0);
        let ok = s.run_until(60_000, |s| {
            s.r.host_actors.owns(A) && s.r.g(0).live.actors.owns(B) && s.r.g(1).live.actors.owns(C)
        });
        assert!(ok, "{name}: each player owns the actor in its own cell");
        s.run_ms(2_000);
        assert!(
            s.r.g(0).live.actors.get(A).is_none() && s.r.g(0).live.actors.get(C).is_none(),
            "{name}: guest 1 never hears of other cells"
        );
        assert!(
            s.r.g(1).live.actors.get(A).is_none() && s.r.g(1).live.actors.get(B).is_none(),
            "{name}"
        );
        assert!(s.r.host_actors.get(B).is_none(), "{name}");
        let own = &s.r.sim.host.live.actors.own;
        assert_eq!(
            own.get(D).unwrap().owner,
            None,
            "{name}: nobody near, the host ledger holds it"
        );
        // the stream of B stays in cell 2: the host ledger keeps its cell
        assert_eq!(own.get(B).unwrap().state.cell, CellId(2), "{name}");
        // presence: guest 1 knows where the host and guest 2 are as data
        let places = s.r.g(0).live.roam.places.clone();
        assert_eq!(places.get(&H).map(|p| p.cell), Some(CellId(1)), "{name}");
        assert_eq!(places.get(&G2).map(|p| p.cell), Some(CellId(3)), "{name}");
        assert_eq!(s.r.sim.decode_errors, 0, "{name}");
    }
}

#[test]
fn entering_a_cell_another_peer_owns_adopts_its_state() {
    for (n, profile) in all_profiles().into_iter().enumerate() {
        let name = profile.name;
        let mut s = SceneRig::new(profile, 2100 + n as u64);
        s.actor(A, st(1, 500.0, 77));
        s.r.host_state = at(1, 0.0);
        s.r.g(0).live.local = at(2, 0.0);
        s.r.g(1).live.local = at(2, 0.0);
        assert!(s.run_until(60_000, |s| s.r.host_actors.owns(A)), "{name}");
        s.run_ms(1_000);
        assert!(
            s.r.g(0).live.actors.get(A).is_none(),
            "{name}: not known before entering"
        );
        // guest 1 walks into cell 1, far from the actor: the host stays owner
        s.r.g(0).live.local = at(1, 6000.0);
        let ok = s.run_until(60_000, |s| {
            s.r.g(0)
                .live
                .actors
                .get(A)
                .is_some_and(|a| a.owner == Some(H) && a.state.is_some())
        });
        assert!(
            ok,
            "{name}: the entering peer adopts the owner and the seed"
        );
        let got = s.r.g(0).live.actors.get(A).unwrap().state.unwrap();
        assert_eq!(got.health, 77, "{name}");
        assert_eq!(got.cell, CellId(1), "{name}");
        assert!(
            (crate::avatar::dequant_pos(got.pos[0]) - 500.0).abs() < 1.0,
            "{name}"
        );
        assert!(!s.r.g(0).live.actors.owns(A), "{name}");
    }
}

#[test]
fn a_dead_actor_is_dead_for_a_peer_that_enters_later() {
    let mut s = SceneRig::new(ChaosProfile::GOOD_WIFI, 2150);
    s.actor(A, st(1, 500.0, 40));
    s.r.host_state = at(1, 0.0);
    s.r.g(0).live.local = at(2, 0.0);
    assert!(s.run_until(30_000, |s| s.r.host_actors.owns(A)));
    let hit = s.r.host_actors.hit(A, 100);
    s.r.sim.host_local(Msg::Actor(hit));
    s.run_ms(500);
    assert!(s.r.sim.host.live.actors.is_dead(A));
    s.r.g(0).live.local = at(1, 3000.0);
    assert!(
        s.run_until(30_000, |s| s
            .r
            .g(0)
            .live
            .actors
            .get(A)
            .is_some_and(|a| !a.alive)),
        "told that it is dead on entry"
    );
}

#[test]
fn exterior_neighbours_share_ownership_and_a_departing_peer_leaves_it_dormant() {
    for (n, profile) in all_profiles().into_iter().enumerate() {
        let name = profile.name;
        let mut s = SceneRig::new(profile, 2200 + n as u64);
        {
            let g = &mut s.r.sim.host.live.roam.model.graph;
            for x in 0..12 {
                g.register_exterior(CellId(100 + x as u32), 0x3C, x, 0);
            }
        }
        // actor in grid cell 101; the host stands far away in cell 110; guest 1 in 100 (cell 101 is in its square)
        s.actor(A, st(101, 300.0, 100));
        s.r.host_state = at(110, 0.0);
        s.r.g(0).live.local = at(100, 0.0);
        s.r.g(1).live.local = at(110, 5.0);
        assert!(
            s.run_until(60_000, |s| s.r.g(0).live.actors.owns(A)),
            "{name}: a neighbour that has the cell loaded owns an unattended actor"
        );
        assert!(
            s.r.g(1).live.actors.get(A).is_none(),
            "{name}: out of the other pair's square"
        );
        // guest 1 walks away to cell 110: nobody has cell 101 loaded
        s.r.g(0).live.local = at(110, 40.0);
        assert!(
            s.run_until(60_000, |s| !s.r.g(0).live.actors.owns(A)),
            "{name}: old owner told to stop"
        );
        s.run_ms(1_000);
        let own = &s.r.sim.host.live.actors.own;
        assert_eq!(own.get(A).unwrap().owner, None, "{name}: dormant");
        assert!(own.dormant_since.contains_key(&A), "{name}");
        // and a player walking back in takes it again with a newer epoch
        let e = own.get(A).unwrap().epoch;
        s.r.g(0).live.local = at(101, 100.0);
        assert!(
            s.run_until(60_000, |s| s.r.g(0).live.actors.owns(A)),
            "{name}"
        );
        assert!(s.r.g(0).live.actors.get(A).unwrap().epoch > e, "{name}");
    }
}

#[test]
fn a_late_joiner_is_placed_where_the_party_is() {
    for (n, profile) in all_profiles().into_iter().enumerate() {
        let name = profile.name;
        let mut sim = Sim::new(2, profile, 2300 + n as u64);
        sim.start_guest(0);
        assert!(
            sim.run_until(120_000, |s| s.guests[0].sync.phase() == GuestPhase::Active),
            "{name}"
        );
        let mut seq = 0u32;
        let host = AvatarState {
            cell: CellId(77),
            pos: [quant_pos(1234.0), 0, 0],
            ..Default::default()
        };
        let mut feed = |sim: &mut Sim| {
            if sim.steps % STEPS_PER_TICK == 0 {
                seq += 1;
                sim.host_local(Msg::Avatar(AvatarMsg::State {
                    player: H,
                    seq: Seq(seq),
                    state: host,
                }));
            }
            sim.step();
        };
        for _ in 0..80 {
            feed(&mut sim);
        }
        sim.start_guest(1);
        let mut placed = None;
        for _ in 0..4000 {
            feed(&mut sim);
            if let Some(p) = sim.guests[1].sync.live.roam.place {
                placed = Some(p);
                break;
            }
        }
        let p = placed
            .unwrap_or_else(|| panic!("{name}: the late joiner was never told where to start"));
        assert_eq!(p.cell, CellId(77), "{name}");
        assert!((p.pos[0] - 1234.0).abs() < 1.0, "{name}");
        let _ = (G1, G2, ActorMsg::Despawn { id: A });
    }
}

#[test]
fn a_tether_anchor_overrides_the_host_for_a_late_joiner() {
    let mut sim = Sim::new(1, ChaosProfile::GOOD_WIFI, 2400);
    sim.host
        .live
        .roam
        .model
        .set_anchor(Some(crate::roam::ZoneAnchor {
            cell: CellId(9),
            pos: [10.0, 20.0, 0.0],
            radius: 100.0,
            heading: 0.0,
        }));
    sim.host_local(Msg::Avatar(AvatarMsg::State {
        player: H,
        seq: Seq(1),
        state: at(77, 0.0),
    }));
    sim.start_guest(0);
    assert!(sim.run_until(120_000, |s| s.guests[0].sync.live.roam.place.is_some()));
    assert_eq!(sim.guests[0].sync.live.roam.place.unwrap().cell, CellId(9));
}

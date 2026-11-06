//! Live-layer scenarios on the chaos simulator: three peers (the host player
//! and two guests), every link under each chaos profile.

use super::*;
use crate::actors::{ActorClient, ActorMsg, ActorState};
use crate::authority::Authority;
use crate::avatar::{quant_pos, AvatarMsg, AvatarState};
use crate::dialogue::{CloseReason, DialogueClient, DialogueEvent};
use crate::ids::{CellId, Seq};
use crate::rules::RulesEvent;
use crate::transport::PROFILE_NAMES;

pub(super) const H: PlayerId = PlayerId(0);
pub(super) const G1: PlayerId = PlayerId(1);
pub(super) const G2: PlayerId = PlayerId(2);

/// The host player's side of the game, as the tests play it.
pub(super) struct Rig {
    pub sim: Sim,
    pub host_state: AvatarState,
    pub host_actors: ActorClient,
    pub host_dialogue: DialogueClient,
    pub rules_events: Vec<(u64, RulesEvent)>,
    pub host_social: Vec<Msg>,
    /// Where owners say their actors stand (actors stay put in the tests).
    pub actor_state: ActorState,
    host_seq: u32,
}

pub(super) fn all_profiles() -> Vec<ChaosProfile> {
    let mut v = vec![ChaosProfile::PERFECT];
    v.extend(
        PROFILE_NAMES
            .iter()
            .map(|n| ChaosProfile::by_name(n).unwrap()),
    );
    v
}

pub(super) fn at(cell: u32, x: f32) -> AvatarState {
    AvatarState {
        cell: CellId(cell),
        pos: [quant_pos(x), 0, 0],
        ..Default::default()
    }
}

impl Rig {
    pub fn new(profile: ChaosProfile, seed: u64) -> Rig {
        // COOP_SEED_OFFSET lets a soak run shift every scenario to fresh chaos
        let off: u64 = std::env::var("COOP_SEED_OFFSET")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(0);
        let mut sim = Sim::new(2, profile, seed + off);
        sim.start_guest(0);
        sim.start_guest(1);
        let ok = sim.run_until(120_000, |s| {
            s.guests
                .iter()
                .all(|g| g.sync.phase() == GuestPhase::Active)
        });
        assert!(ok, "{} guests never became active", profile.name);
        Rig {
            sim,
            host_state: at(1, 0.0),
            host_actors: ActorClient::new(H),
            host_dialogue: DialogueClient::new(H),
            rules_events: Vec::new(),
            host_social: Vec::new(),
            actor_state: ActorState {
                cell: CellId(1),
                health: 100,
                ..Default::default()
            },
            host_seq: 0,
        }
    }

    pub fn g(&mut self, i: usize) -> &mut GuestSync {
        &mut self.sim.guests[i].sync
    }

    /// One simulator step plus the host player's per-step game work.
    pub fn step(&mut self) {
        if self.sim.steps % STEPS_PER_TICK == 0 {
            self.host_seq += 1;
            let m = Msg::Avatar(AvatarMsg::State {
                player: H,
                seq: Seq(self.host_seq),
                state: self.host_state,
            });
            self.sim.host_local(m);
            // owners publish what they own
            for a in self.host_actors.owned() {
                if let Some(m) = self.host_actors.publish(a, self.actor_state) {
                    self.sim.host_local(Msg::Actor(m));
                }
            }
            for i in 0..self.sim.guests.len() {
                for a in self.sim.guests[i].sync.live.actors.owned() {
                    if let Some(m) = self.sim.guests[i]
                        .sync
                        .live
                        .actors
                        .publish(a, self.actor_state)
                    {
                        self.sim.guest_emit(i, vec![Msg::Actor(m)]);
                    }
                }
            }
        }
        self.sim.step();
        let now = self.sim.clock.now();
        for m in self.sim.host.live.take_local() {
            match m {
                Msg::Actor(a) => self.host_actors.on_msg(a),
                Msg::Dialogue(d) => self.host_dialogue.on_msg(d, now),
                m @ Msg::Social(_) => self.host_social.push(m),
                _ => {}
            }
        }
        for e in self.sim.host.live.rules.take_events() {
            self.rules_events.push((now, e));
        }
    }

    pub fn run_until(&mut self, max_ms: u64, mut cond: impl FnMut(&mut Rig) -> bool) -> bool {
        let deadline = self.sim.clock.now() + max_ms;
        while self.sim.clock.now() < deadline {
            if cond(self) {
                return true;
            }
            self.step();
        }
        cond(self)
    }

    pub fn run_ms(&mut self, ms: u64) {
        let end = self.sim.clock.now() + ms;
        while self.sim.clock.now() < end {
            self.step();
        }
    }

    pub fn guest_send(&mut self, i: usize, m: Msg) {
        self.sim.guest_emit(i, vec![m]);
    }
}

#[test]
fn combat_actions_reach_the_other_guest_in_order_under_every_profile() {
    use crate::avatar::ActionKind;
    for (n, profile) in all_profiles().into_iter().enumerate() {
        let name = profile.name;
        let mut r = Rig::new(profile, 700 + n as u64);
        r.g(0).live.local = at(1, 0.0);
        r.g(1).live.local = at(1, 20.0);
        r.run_ms(1_500);
        for k in 0..6u32 {
            let m = r.g(0).live.action(
                if k % 2 == 0 {
                    ActionKind::Swing
                } else {
                    ActionKind::Shoot
                },
                k,
            );
            r.guest_send(0, m);
            r.run_ms(150);
        }
        let mut got = Vec::new();
        let ok = r.run_until(90_000, |r| {
            got.extend(r.g(1).live.take_actions());
            got.len() >= 6
        });
        assert!(ok, "{name}: only {} of 6 action events arrived", got.len());
        let args: Vec<u32> = got.iter().map(|(_, a)| a.arg).collect();
        assert_eq!(args, vec![0, 1, 2, 3, 4, 5], "{name}: order");
        assert!(got.iter().all(|(p, _)| *p == G1), "{name}: sender");
    }
}

#[test]
fn actor_handoff_across_cells_under_every_profile() {
    for (n, profile) in all_profiles().into_iter().enumerate() {
        let name = profile.name;
        let mut r = Rig::new(profile, 100 + n as u64);
        let a = FormId(0x100);
        r.sim.host.live.actors.register(
            a,
            ActorState {
                cell: CellId(1),
                pos: [0; 3],
                health: 100,
                ..Default::default()
            },
        );
        r.host_state = at(1, 800.0);
        r.g(0).live.local = at(1, 50.0);
        r.g(1).live.local = at(2, 0.0);

        // nearest player in the cell owns it
        assert!(
            r.run_until(60_000, |r| r.g(0).live.actors.owns(a)),
            "{name}: guest 1 never became owner"
        );
        let e1 = r.g(0).live.actors.get(a).unwrap().epoch;
        r.run_ms(2_000);
        let seen = r.sim.host.live.actors.own.get(a).unwrap().state;
        assert_eq!(
            seen.cell,
            CellId(1),
            "{name}: owner stream reached the host"
        );

        // owner leaves the cell: the host player takes over with a newer epoch
        r.g(0).live.local = at(2, 50.0);
        assert!(
            r.run_until(60_000, |r| r.host_actors.owns(a)),
            "{name}: host never took over"
        );
        let e2 = r.host_actors.get(a).unwrap().epoch;
        assert!(e2 > e1, "{name}");
        assert!(
            r.run_until(60_000, |r| !r.g(0).live.actors.owns(a)),
            "{name}: old owner never told"
        );
        assert!(
            matches!(r.sim.host.auth.authority_of(a), Authority::Actor { owner: H, epoch } if epoch == e2)
        );

        // a late write from the old owner is fenced off
        let before = r.sim.host.live.actors.own.stale_rejected;
        let before_state = r.sim.host.live.actors.own.get(a).unwrap().state;
        let stale = ActorMsg::State {
            actor: a,
            epoch: e1,
            seq: Seq(900_000),
            state: ActorState {
                cell: CellId(1),
                pos: [quant_pos(7777.0), 0, 0],
                ..Default::default()
            },
        };
        r.guest_send(0, Msg::Actor(stale));
        r.run_ms(1_500);
        assert!(
            r.sim.host.live.actors.own.stale_rejected > before,
            "{name}: stale write was not rejected"
        );
        let after_state = r.sim.host.live.actors.own.get(a).unwrap().state;
        assert_ne!(
            after_state.pos[0],
            quant_pos(7777.0),
            "{name}: stale write was applied"
        );
        let _ = before_state;

        // guest 2 walks in right next to it while the host drifts away: third handoff
        r.host_state = at(1, 900.0);
        r.g(0).live.local = at(2, 50.0);
        r.g(1).live.local = at(1, 10.0);
        assert!(
            r.run_until(90_000, |r| r.g(1).live.actors.owns(a)),
            "{name}: guest 2 never became owner"
        );
        let e3 = r.g(1).live.actors.get(a).unwrap().epoch;
        assert!(e3 > e2, "{name}");

        // everybody who can see the cell agrees on the single owner
        assert!(
            r.run_until(60_000, |r| !r.host_actors.owns(a)
                && r.host_actors.get(a).map(|x| x.owner) == Some(Some(G2))),
            "{name}: host copy never learned the new owner"
        );
        assert_eq!(r.sim.host.live.actors.own.get(a).unwrap().owner, Some(G2));
        assert_eq!(r.sim.decode_errors, 0, "{name}");
    }
}

#[test]
fn actor_kill_and_spawn_are_resolved_once_by_the_host() {
    for (n, profile) in all_profiles().into_iter().enumerate() {
        let name = profile.name;
        let mut r = Rig::new(profile, 200 + n as u64);
        let a = FormId(0x200);
        r.sim.host.live.actors.register(
            a,
            ActorState {
                cell: CellId(1),
                health: 100,
                ..Default::default()
            },
        );
        r.g(0).live.local = at(1, 0.0);
        r.g(1).live.local = at(1, 20.0);
        r.run_ms(1_000);
        // both players hit it in the same instant: damage sums, death happens once
        for i in 0..2 {
            let m = r.g(i).live.actors.hit(a, 60);
            r.guest_send(i, Msg::Actor(m));
        }
        assert!(
            r.run_until(60_000, |r| r.sim.host.live.actors.is_dead(a)),
            "{name}"
        );
        r.run_ms(3_000);
        let deaths = |c: &ActorClient| c.get(a).map_or(0, |x| !x.alive as u32);
        assert_eq!(deaths(&r.g(0).live.actors), 1, "{name}");
        assert_eq!(deaths(&r.g(1).live.actors), 1, "{name}");
        assert_eq!(r.sim.host.live.actors.hits_applied, 2, "{name}");

        // both roll the same spawn site: one id, one winner
        let mut reqs = vec![];
        for i in 0..2 {
            let (req, m) =
                r.g(i)
                    .live
                    .actors
                    .request_spawn(FormId(0x50), FormId(0x60), CellId(1), [0; 3], 40);
            r.guest_send(i, Msg::Actor(m));
            reqs.push(req);
        }
        assert!(
            r.run_until(60_000, |r| (0..2).all(|i| r
                .g(i)
                .live
                .actors
                .spawn_results
                .contains_key(&reqs[i]))),
            "{name}"
        );
        let granted: Vec<_> = (0..2)
            .filter(|i| {
                matches!(
                    r.g(*i).live.actors.spawn_results[&reqs[*i]],
                    crate::actors::SpawnOutcome::Granted { .. }
                )
            })
            .collect();
        assert_eq!(granted.len(), 1, "{name}: exactly one spawn per site");

        // leveled pick: both propose different picks, everyone ends with the first
        for i in 0..2 {
            let m = r
                .g(i)
                .live
                .actors
                .request_leveled(FormId(9), 4, FormId(1000 + i as u32));
            r.guest_send(i, Msg::Actor(m));
        }
        assert!(
            r.run_until(60_000, |r| (0..2).all(|i| r
                .g(i)
                .live
                .actors
                .leveled
                .contains_key(&(FormId(9), 4)))),
            "{name}"
        );
        let (p0, p1) = (
            r.g(0).live.actors.leveled[&(FormId(9), 4)],
            r.g(1).live.actors.leveled[&(FormId(9), 4)],
        );
        assert_eq!(p0, p1, "{name}");
    }
}

fn open_session(r: &mut Rig, who: usize, npc: FormId) -> u32 {
    let (_, m) = r.g(who).live.dialogue.open(npc);
    r.guest_send(who, Msg::Dialogue(m));
    let ok = r.run_until(60_000, |r| {
        r.g(who).live.dialogue.view().is_some()
            && r.g(1 - who).live.dialogue.view().is_some()
            && r.host_dialogue.view().is_some()
    });
    assert!(ok, "session never started on every peer");
    r.g(who).live.dialogue.view().unwrap().session
}

#[test]
fn dialogue_session_runs_every_step_once_under_every_profile() {
    for (n, profile) in all_profiles().into_iter().enumerate() {
        let name = profile.name;
        let mut r = Rig::new(profile, 300 + n as u64);
        let s = open_session(&mut r, 0, FormId(0x77));
        for step in 1..=2u32 {
            let m = r
                .g(0)
                .live
                .dialogue
                .pick(step * 10)
                .expect("initiator may pick");
            r.guest_send(0, Msg::Dialogue(m));
            let playing = |c: &DialogueClient| {
                c.view()
                    .is_some_and(|v| v.step == step && v.playing.is_some())
            };
            assert!(
                r.run_until(60_000, |r| playing(&r.g(0).live.dialogue)
                    && playing(&r.g(1).live.dialogue)
                    && playing(&r.host_dialogue)),
                "{name}: step {step} never reached everyone"
            );
            // everyone finishes their lines
            for i in 0..2 {
                if let Some(m) = r.g(i).live.dialogue.line_done() {
                    r.guest_send(i, Msg::Dialogue(m));
                }
            }
            if let Some(m) = r.host_dialogue.line_done() {
                r.sim.host_local(Msg::Dialogue(m));
            }
            assert!(
                r.run_until(60_000, |r| !r.g(0).live.dialogue.ui_blocked()),
                "{name}: initiator stuck on step {step}"
            );
        }
        let m = r.g(0).live.dialogue.close().unwrap();
        r.guest_send(0, Msg::Dialogue(m));
        assert!(
            r.run_until(60_000, |r| r.g(1).live.dialogue.view().is_none()),
            "{name}"
        );
        let ev = r.sim.host.live.dialogue.take_events();
        let begins = ev
            .iter()
            .filter(|e| matches!(e, DialogueEvent::RunBegin { .. }))
            .count();
        let ends = ev
            .iter()
            .filter(|e| matches!(e, DialogueEvent::RunEnd { .. }))
            .count();
        assert_eq!((begins, ends), (2, 2), "{name}: {ev:?}");
        assert_eq!(
            r.g(1).live.dialogue.last_closed,
            Some((s, CloseReason::Finished)),
            "{name}"
        );
    }
}

#[test]
fn dialogue_with_a_dropped_initiator_flushes_and_unlocks() {
    for (n, profile) in all_profiles().into_iter().enumerate() {
        let name = profile.name;
        let mut r = Rig::new(profile, 400 + n as u64);
        let npc = FormId(0x77);
        let s = open_session(&mut r, 0, npc);
        let m = r.g(0).live.dialogue.pick(5).unwrap();
        r.guest_send(0, Msg::Dialogue(m));
        assert!(
            r.run_until(60_000, |r| r.g(1).live.dialogue.ui_blocked()
                && r.host_dialogue.ui_blocked()),
            "{name}"
        );
        // spectators finish, the initiator vanishes before reporting
        let m = r.g(1).live.dialogue.line_done().unwrap();
        r.guest_send(1, Msg::Dialogue(m));
        let m = r.host_dialogue.line_done().unwrap();
        r.sim.host_local(Msg::Dialogue(m));
        r.run_ms(200);
        r.sim.kick_guest(0);
        assert!(
            r.run_until(60_000, |r| r.g(1).live.dialogue.last_closed
                == Some((s, CloseReason::InitiatorLeft))),
            "{name}: spectator never saw the abort"
        );
        assert!(
            r.host_dialogue.view().is_none()
                || r.run_until(60_000, |r| r.host_dialogue.view().is_none()),
            "{name}"
        );
        let ev = r.sim.host.live.dialogue.take_events();
        let count = |f: fn(&DialogueEvent) -> bool| ev.iter().filter(|e| f(e)).count();
        assert_eq!(
            count(|e| matches!(e, DialogueEvent::RunBegin { .. })),
            1,
            "{name}"
        );
        assert_eq!(
            count(|e| matches!(e, DialogueEvent::RunEnd { .. })),
            1,
            "end script flushed once: {name}"
        );
        assert_eq!(
            count(|e| matches!(e, DialogueEvent::Closed { .. })),
            1,
            "{name}"
        );
        // the NPC is free again
        let (_, m) = r.g(1).live.dialogue.open(npc);
        r.guest_send(1, Msg::Dialogue(m));
        assert!(
            r.run_until(60_000, |r| r.g(1).live.dialogue.is_initiator()),
            "{name}: npc stayed locked"
        );
    }
}

pub(super) fn loot_pending(r: &mut Rig, i: usize, c: FormId) -> usize {
    r.g(i).live.loot.view(c).map_or(0, |v| v.pending())
}

pub(super) fn leased(r: &mut Rig, i: usize, c: FormId) -> bool {
    r.g(i).live.loot.view(c).is_some_and(|v| v.leased)
}

//! The live layer through `Session`: a host and a guest, each a `GameState`,
//! joined by the loopback transport, driven only through the frame calls the
//! viewer makes (`pre_frame_with`, `live_frame`, `post_frame`).

use coop_core::ids::PlayerId;
use coop_core::transport::{ChaosProfile, LoopbackNet};
use coop_session::{Intent, Session, SessionConfig};
use esm::{ActivePlugins, FormId, LoadOrder};
use world::coop::apply::{Ctx, Opts};
use world::coop::WorldBackend;
use world::dialogue::PLAYER_REF;
use world::scripting::{Event, GameState, ScriptCache};

const DT: f32 = 0.025;

struct Peer {
    s: Session,
    gs: GameState,
    pos: [f32; 3],
    cell: u32,
}

struct Rig<'a> {
    order: &'a LoadOrder,
    scripts: &'a ScriptCache,
    net: LoopbackNet,
    host: Peer,
    guest: Peer,
    chaos: Option<&'static str>,
}

fn cfg(
    name: &str,
    t: Box<dyn coop_core::transport::Transport>,
    chaos: Option<(&'static str, u64)>,
) -> SessionConfig {
    SessionConfig {
        name: name.into(),
        session_id: 0xABCD,
        transport: Some(t),
        chaos: chaos.map(|(p, seed)| (ChaosProfile::by_name(p).unwrap(), seed)),
        live: true,
        ..SessionConfig::default()
    }
}

impl<'a> Rig<'a> {
    fn new(
        order: &'a LoadOrder,
        scripts: &'a ScriptCache,
        cell: u32,
        chaos: Option<&'static str>,
    ) -> Rig<'a> {
        Self::with_guest(order, scripts, cell, chaos, |ep| Box::new(ep))
    }

    fn with_guest(
        order: &'a LoadOrder,
        scripts: &'a ScriptCache,
        cell: u32,
        chaos: Option<&'static str>,
        wrap: impl FnOnce(
            coop_core::transport::LoopbackEndpoint,
        ) -> Box<dyn coop_core::transport::Transport>,
    ) -> Rig<'a> {
        let net = LoopbackNet::new();
        let hs = Session::host(cfg(
            "Host",
            Box::new(net.endpoint(PlayerId::HOST)),
            chaos.map(|p| (p, 1)),
        ))
        .unwrap();
        let gsn = Session::join(cfg(
            "Ann",
            wrap(net.endpoint(PlayerId(1))),
            chaos.map(|p| (p, 2)),
        ))
        .unwrap();
        let mk = |s| Peer {
            s,
            gs: GameState::new(order),
            pos: [0.0, 0.0, 0.0],
            cell,
        };
        let mut r = Rig {
            order,
            scripts,
            net,
            host: mk(hs),
            guest: mk(gsn),
            chaos,
        };
        r.host.pos = [900.0, 1000.0, 0.0];
        r.guest.pos = [900.0, 1000.0, 0.0];
        r
    }

    fn cx(&self) -> Ctx<'a> {
        Ctx {
            order: self.order,
            scripts: self.scripts,
        }
    }

    fn frame(&mut self) {
        let cx = self.cx();
        for p in [&mut self.host, &mut self.guest] {
            p.s.pre_frame_with(
                &mut p.gs,
                Opts {
                    present: true,
                    ctx: Some(cx),
                },
            );
            p.gs.player_cell = Some(FormId(p.cell));
            p.gs.player_position = Some(p.pos);
            p.s.live_frame(&mut p.gs, cx, DT);
            p.s.post_frame(&mut p.gs, DT);
        }
    }

    fn frames(&mut self, n: u32) {
        for _ in 0..n {
            self.frame();
        }
    }

    fn until(&mut self, max: u32, mut cond: impl FnMut(&mut Rig<'a>) -> bool) -> bool {
        for _ in 0..max {
            if cond(self) {
                return true;
            }
            self.frame();
        }
        cond(self)
    }

    fn settled(&mut self) {
        let ok = self.until(8000, |r| {
            r.guest.s.is_connected() && r.host.s.is_ready() && r.guest.s.is_ready()
        });
        assert!(ok, "never connected ({:?})", self.chaos);
        self.frames(60);
    }
}

fn died(g: &GameState, who: FormId) -> usize {
    g.events
        .iter()
        .filter(|e| matches!(e, Event::Died { who: w, .. } if *w == who))
        .count()
}

mod fighting {
    use super::*;
    use testdata::fighting::ids::*;

    fn order(tag: &str) -> (testdata::TempData, LoadOrder) {
        let data = testdata::fighting::fighting(tag);
        let order = LoadOrder::from_data_dir(data.path(), &ActivePlugins::OfficialOnly).unwrap();
        (data, order)
    }

    #[test]
    fn setup_turns_on_policy_and_shadows() {
        let (_d, order) = order("sl-setup");
        let scripts = ScriptCache::default();
        let mut r = Rig::new(&order, &scripts, CELL, None);
        r.settled();
        assert!(r.host.gs.coop.gate.is_some(), "host has the policy gate");
        assert!(
            r.host.gs.coop.bank.enabled() && r.guest.gs.coop.bank.enabled(),
            "shadows on both"
        );
        assert_eq!(r.host.gs.active_slot(), 0);
        assert_eq!(r.guest.gs.active_slot(), 1);
        assert!(
            r.host.gs.coop.bank.parked.contains_key(&1),
            "host registered the guest"
        );
        assert!(r.host.gs.coop.gate.as_ref().unwrap().present.contains(1));
    }

    #[test]
    fn avatars_are_visible_and_move() {
        for chaos in [None, Some("good_wifi")] {
            let (_d, order) = order("sl-avatars");
            let scripts = ScriptCache::default();
            let mut r = Rig::new(&order, &scripts, CELL, chaos);
            r.settled();
            r.host.pos = [100.0, 0.0, 0.0];
            r.guest.pos = [0.0, 250.0, 0.0];
            let ok = r.until(2000, |r| {
                let on_guest = r.guest.s.live_view().and_then(|v| v.avatars.iter().find(|a| a.player.is_host()).cloned());
                let on_host = r.host.s.live_view().and_then(|v| v.avatars.iter().find(|a| a.player == PlayerId(1)).cloned());
                matches!((on_guest, on_host), (Some(a), Some(b)) if (a.pos[0] - 100.0).abs() < 5.0 && (b.pos[1] - 250.0).abs() < 5.0)
            });
            assert!(ok, "{chaos:?}: each side sees the other within tolerance");
            let v = r.host.s.live_view().unwrap();
            let a = v.avatars.iter().find(|a| a.player == PlayerId(1)).unwrap();
            assert_eq!(a.name, "Ann");
            assert_eq!(a.puppet, world::coop::facts::puppet_of(PlayerId(1)));
            assert_eq!(a.cell, CELL);
            // pose and parts travel too
            r.guest.gs.coop.live.local_anim = 9;
            r.guest.gs.coop.live.local_weapon = 0x1234;
            r.guest.gs.coop.live.local_flags = coop_core::avatar::FLAG_DRAWN;
            r.guest
                .s
                .set_local_parts(vec![(3, FormId(0x77))], vec![1, 2, 3]);
            let ok = r.until(2000, |r| {
                r.host.s.live_view().is_some_and(|v| {
                    v.avatars.iter().any(|a| {
                        a.anim == 9
                            && a.weapon == 0x1234
                            && a.drawn
                            && a.equipment == vec![(3, FormId(0x77))]
                    })
                })
            });
            assert!(ok, "{chaos:?}: anim, weapon, drawn flag and equipment");
            // leaving removes the avatar
            r.guest.s =
                Session::join(cfg("x", Box::new(r.net.endpoint(PlayerId(2))), None)).unwrap();
            let ok = r.until(4000, |r| {
                r.host.s.live_view().is_some_and(|v| !v.left.is_empty())
                    || r.host
                        .s
                        .live_view()
                        .is_some_and(|v| v.avatars.iter().all(|a| a.player != PlayerId(1)))
            });
            assert!(ok, "{chaos:?}: the avatar goes when the guest goes");
        }
    }

    #[test]
    fn the_dump_carries_both_positions_and_per_player_items() {
        let (_d, order) = order("sl-dump");
        let scripts = ScriptCache::default();
        let mut r = Rig::new(&order, &scripts, CELL, None);
        r.settled();
        r.host.pos = [100.0, 0.0, 0.0];
        r.guest.pos = [0.0, 250.0, 0.0];
        r.frames(200);
        let root = Session::state_root(&r.guest.gs);
        let text = coop_session::dump::state_json_live(
            &r.guest.gs,
            &order,
            coop_session::Role::Guest,
            1.0,
            1,
            root,
            r.guest.s.live_view(),
        );
        let host_at = text.find("\"host\":[100").is_some();
        let guest_at = text.find("\"guest\":[0,250").is_some();
        assert!(host_at && guest_at, "{text}");
        assert!(text.contains("\"live\":{\"avatars\":1"), "{text}");
    }

    #[test]
    fn combat_actions_of_one_player_reach_the_other_in_order() {
        use coop_session::ActionKind;
        for chaos in [None, Some("bad_wifi"), Some("flaky")] {
            let (_d, order) = order("sl-actions");
            let scripts = ScriptCache::default();
            let mut r = Rig::new(&order, &scripts, CELL, chaos);
            r.settled();
            r.frames(60);
            // the guest swings three times, shoots once, and dies
            r.guest
                .gs
                .coop
                .live
                .local_actions
                .push((ActionKind::Swing, 32));
            r.guest
                .gs
                .coop
                .live
                .local_actions
                .push((ActionKind::Swing, 33));
            r.guest
                .gs
                .coop
                .live
                .local_actions
                .push((ActionKind::Shoot, 40));
            r.guest
                .gs
                .coop
                .live
                .local_actions
                .push((ActionKind::Death, 0));
            let mut seen: Vec<(ActionKind, u32)> = Vec::new();
            let ok = r.until(4000, |r| {
                if let Some(v) = r.host.s.live_view() {
                    seen.extend(
                        v.actions
                            .iter()
                            .filter(|(p, _)| *p == PlayerId(1))
                            .map(|(_, a)| (a.kind, a.arg)),
                    );
                }
                seen.len() >= 4
            });
            assert!(ok, "{chaos:?}: {seen:?}");
            assert_eq!(
                seen,
                vec![
                    (ActionKind::Swing, 32),
                    (ActionKind::Swing, 33),
                    (ActionKind::Shoot, 40),
                    (ActionKind::Death, 0)
                ],
                "{chaos:?}"
            );
        }
    }

    #[test]
    fn a_guest_hit_kills_an_npc_once_on_both() {
        for chaos in [None, Some("bad_wifi")] {
            let (_d, order) = order("sl-kill");
            let scripts = ScriptCache::default();
            let mut r = Rig::new(&order, &scripts, CELL, chaos);
            r.settled();
            let gecko = FormId(GECKO_REF);
            let cx = r.cx();
            r.host.s.live_track(cx, &r.host.gs, &[gecko]);
            r.frames(60);
            // a hit on the guest is a request, nothing changes at once
            world::combat::hurt(&order, &mut r.guest.gs, gecko, 100.0, PLAYER_REF);
            assert!(r.guest.gs.damage.is_empty());
            let ok = r.until(4000, |r| {
                r.host.gs.dead.contains(&gecko) && r.guest.gs.dead.contains(&gecko)
            });
            assert!(ok, "{chaos:?}: dead on both");
            r.frames(200);
            assert_eq!(died(&r.host.gs, gecko), 1);
            assert_eq!(died(&r.guest.gs, gecko), 1);
            assert!(r.guest.s.live_hits_sent() >= 1);
            // the guest sees nothing owned by itself that the host does not know
            world::combat::hurt(&order, &mut r.guest.gs, gecko, 100.0, PLAYER_REF);
            r.frames(200);
            assert_eq!(died(&r.host.gs, gecko), 1);
            assert_eq!(died(&r.guest.gs, gecko), 1);
        }
    }

    #[test]
    fn remote_actor_state_reaches_the_view() {
        let (_d, order) = order("sl-actor-view");
        let scripts = ScriptCache::default();
        let mut r = Rig::new(&order, &scripts, CELL, None);
        r.guest.pos = [20.0, 1000.0, 0.0];
        r.settled();
        let gecko = FormId(GECKO_REF);
        let cx = r.cx();
        r.host.s.live_track(cx, &r.host.gs, &[gecko]);
        // the nearest player (the guest) is given the actor
        let ok = r.until(4000, |r| {
            r.guest
                .s
                .live_view()
                .is_some_and(|v| v.owned_actors.contains(&gecko))
        });
        assert!(ok, "ownership handed to the nearest player");
        r.guest
            .gs
            .positions
            .insert(gecko, ([321.0, 1000.0, 0.0], 1.0));
        r.guest.gs.coop.live.anim.insert(gecko, 5);
        let ok = r.until(4000, |r| {
            r.host.s.live_view().is_some_and(|v| {
                v.remote_actors
                    .iter()
                    .any(|a| a.id == gecko && (a.pos[0] - 321.0).abs() < 1.0 && a.anim == 5)
            })
        });
        assert!(ok, "the host sees the guest-owned actor move");
    }
}

mod dialogue_world {
    use super::*;
    use testdata::dialogue::ids::*;

    #[test]
    fn a_dialogue_session_through_intents() {
        let data = testdata::dialogue::world("sl-dialogue");
        let order = LoadOrder::from_data_dir(data.path(), &ActivePlugins::OfficialOnly).unwrap();
        let scripts = ScriptCache::default();
        let mut r = Rig::new(&order, &scripts, 0xB10, None);
        r.settled();
        let npc = FormId(0x1299);
        r.guest.s.live_intent(Intent::DialogueOpen(npc));
        let ok = r.until(2000, |r| {
            r.host
                .s
                .live_view()
                .is_some_and(|v| v.dialogue.as_ref().is_some_and(|d| !d.mine && d.npc == npc))
                && r.guest
                    .s
                    .live_view()
                    .is_some_and(|v| v.dialogue.as_ref().is_some_and(|d| d.mine))
        });
        assert!(ok, "both see the session, the guest picks");
        r.guest.s.live_intent(Intent::DialoguePick(FormId(BYE)));
        let door = FormId(DOOR_OPEN);
        assert!(
            r.until(2000, |r| r.host.gs.globals.get(&door) == Some(&1.0)),
            "the begin script ran on the host"
        );
        let mut host_lines = Vec::new();
        let mut guest_lines = Vec::new();
        let ok = r.until(2000, |r| {
            host_lines.extend(
                r.host
                    .s
                    .live_view()
                    .map(|v| v.lines.clone())
                    .unwrap_or_default(),
            );
            guest_lines.extend(
                r.guest
                    .s
                    .live_view()
                    .map(|v| v.lines.clone())
                    .unwrap_or_default(),
            );
            !host_lines.is_empty() && !guest_lines.is_empty()
        });
        assert!(ok, "both peers get the line to play");
        assert_eq!(host_lines, guest_lines);
        assert_eq!(host_lines[0].1, FormId(BYE));
        r.guest.s.live_intent(Intent::DialogueLineDone);
        r.host.s.live_intent(Intent::DialogueLineDone);
        assert!(
            r.until(3000, |r| r.host.gs.globals.get(&door) == Some(&2.0)),
            "the end script ran once"
        );
        r.guest.s.live_intent(Intent::DialogueClose);
        r.frames(100);
        assert_eq!(r.host.gs.globals.get(&door), Some(&2.0));
        assert!(r.until(3000, |r| r.guest.gs.globals.get(&door) == Some(&2.0)));
    }

    #[test]
    fn chat_goes_both_ways() {
        let data = testdata::dialogue::world("sl-chat");
        let order = LoadOrder::from_data_dir(data.path(), &ActivePlugins::OfficialOnly).unwrap();
        let scripts = ScriptCache::default();
        let mut r = Rig::new(&order, &scripts, 0xB10, None);
        r.settled();
        r.guest.s.live_intent(Intent::Say("hello host".into()));
        let mut got = Vec::new();
        assert!(r.until(1000, |r| {
            got.extend(
                r.host
                    .s
                    .live_view()
                    .map(|v| v.chat.clone())
                    .unwrap_or_default(),
            );
            got.iter()
                .any(|c| c.text == "hello host" && c.name == "Ann")
        }));
        r.host.s.live_intent(Intent::Say("hello guest".into()));
        let mut back = Vec::new();
        assert!(r.until(1000, |r| {
            back.extend(
                r.guest
                    .s
                    .live_view()
                    .map(|v| v.chat.clone())
                    .unwrap_or_default(),
            );
            back.iter().any(|c| c.text == "hello guest")
        }));
    }
}

/// A guest transport that reports the unassigned slot (255, as the UDP
/// endpoint does before the Welcome) and hears nothing for a while.
struct Unassigned {
    inner: coop_core::transport::LoopbackEndpoint,
    deaf: std::rc::Rc<std::cell::Cell<bool>>,
}

impl coop_core::transport::Transport for Unassigned {
    fn local(&self) -> PlayerId {
        PlayerId(255)
    }
    fn send(
        &mut self,
        to: PlayerId,
        c: coop_core::proto::Channel,
        d: Vec<u8>,
    ) -> Result<(), coop_core::transport::TransportError> {
        self.inner.send(to, c, d)
    }
    fn recv(&mut self) -> Option<coop_core::transport::Packet> {
        let p = self.inner.recv();
        if self.deaf.get() {
            return None;
        }
        p
    }
}

#[test]
fn a_delayed_welcome_never_creates_slot_255_rows() {
    for chaos in [None, Some("bad_wifi")] {
        let data = testdata::fighting::fighting("sl-late-welcome");
        let order = LoadOrder::from_data_dir(data.path(), &ActivePlugins::OfficialOnly).unwrap();
        let scripts = ScriptCache::default();
        let deaf = std::rc::Rc::new(std::cell::Cell::new(true));
        let d2 = deaf.clone();
        let mut r = Rig::with_guest(
            &order,
            &scripts,
            testdata::fighting::ids::CELL,
            chaos,
            move |inner| Box::new(Unassigned { inner, deaf: d2 }),
        );
        let check = |r: &Rig| {
            let b = &r.guest.gs.coop.bank;
            assert_ne!(b.active, 255, "no live rows under slot 255");
            assert!(
                !b.parked.contains_key(&255),
                "no parked rows under slot 255"
            );
            assert!(r.guest.gs.shadow_of(255).is_none());
            if b.enabled() {
                assert_ne!(
                    r.guest.s.status().player.0,
                    255,
                    "shadows only once the slot is assigned"
                );
            }
        };
        for i in 0..400 {
            r.frame();
            check(&r);
            if i == 300 {
                deaf.set(false);
            }
        }
        assert!(
            r.until(12_000, |r| r.guest.s.is_connected() && r.guest.s.is_ready()),
            "{chaos:?}: connected"
        );
        for _ in 0..400 {
            r.frame();
            check(&r);
        }
        assert!(
            r.guest.gs.coop.bank.enabled(),
            "shadows on after the Welcome"
        );
        assert_ne!(r.guest.gs.active_slot(), 255);
        let ok = r.until(12_000, |r| {
            r.host.gs.shard_hashes() == r.guest.gs.shard_hashes()
        });
        assert!(ok, "{chaos:?}: the shared shards agree");
    }
}

//! Gaps the headless simulator (`coop-headless`) found in the live layer's
//! entry points: an intent asked for before the guest joined was dropped, and
//! a host's container take (script or bot, no container screen) did nothing.

use coop_core::ids::PlayerId;
use coop_core::transport::LoopbackNet;
use coop_session::{Intent, Session, SessionConfig};
use esm::{ActivePlugins, FormId, LoadOrder};
use world::coop::apply::{Ctx, Opts};
use world::coop::facts::LootOp;
use world::dialogue::PLAYER_REF;
use world::scripting::{GameState, ScriptCache};

const DT: f32 = 0.025;

struct Peer {
    s: Session,
    gs: GameState,
}

struct Rig<'a> {
    order: &'a LoadOrder,
    scripts: &'a ScriptCache,
    cell: u32,
    host: Peer,
    guest: Peer,
}

fn cfg(name: &str, t: Box<dyn coop_core::transport::Transport>) -> SessionConfig {
    SessionConfig {
        name: name.into(),
        session_id: 0xABCD,
        transport: Some(t),
        live: true,
        ..SessionConfig::default()
    }
}

impl<'a> Rig<'a> {
    fn new(order: &'a LoadOrder, scripts: &'a ScriptCache, cell: u32) -> Rig<'a> {
        let net = LoopbackNet::new();
        let hs = Session::host(cfg("Host", Box::new(net.endpoint(PlayerId::HOST)))).unwrap();
        let gsn = Session::join(cfg("Ann", Box::new(net.endpoint(PlayerId(1))))).unwrap();
        Rig {
            order,
            scripts,
            cell,
            host: Peer {
                s: hs,
                gs: GameState::new(order),
            },
            guest: Peer {
                s: gsn,
                gs: GameState::new(order),
            },
        }
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
            p.gs.player_cell = Some(FormId(self.cell));
            p.gs.player_position = Some([0.0, 0.0, 0.0]);
            p.s.live_frame(&mut p.gs, cx, DT);
            p.s.post_frame(&mut p.gs, DT);
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
        assert!(self.until(8000, |r| r.guest.s.is_connected()
            && r.host.s.is_ready()
            && r.guest.s.is_ready()));
        for _ in 0..60 {
            self.frame();
        }
    }
}

#[test]
fn an_intent_asked_for_before_the_guest_joined_is_carried_once_it_has() {
    let data = testdata::dialogue::world("sg-queued");
    let order = LoadOrder::from_data_dir(data.path(), &ActivePlugins::OfficialOnly).unwrap();
    let scripts = ScriptCache::default();
    let mut r = Rig::new(&order, &scripts, 0xB10);
    let npc = FormId(0x1299);
    // the viewer's `--talk` fires as soon as its scene is loaded, before the join finishes
    assert!(!r.guest.s.is_connected());
    r.guest.s.live_intent(Intent::DialogueOpen(npc));
    r.settled();
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
    assert!(ok, "the early open reached the host and the session runs");
}

#[test]
fn a_stale_queued_intent_is_dropped() {
    let data = testdata::dialogue::world("sg-stale");
    let order = LoadOrder::from_data_dir(data.path(), &ActivePlugins::OfficialOnly).unwrap();
    let scripts = ScriptCache::default();
    let mut r = Rig::new(&order, &scripts, 0xB10);
    // the host never answers for a long time: the guest cannot join
    let cx = r.cx();
    r.guest.s.live_intent(Intent::DialogueOpen(FormId(0x1299)));
    for _ in 0..((coop_session::live::QUEUE_MS as f32 / 1000.0 / DT) as u32 + 100) {
        r.guest.s.pre_frame_with(
            &mut r.guest.gs,
            Opts {
                present: true,
                ctx: Some(cx),
            },
        );
        r.guest.s.post_frame(&mut r.guest.gs, DT);
    }
    r.settled();
    for _ in 0..200 {
        r.frame();
    }
    assert!(
        r.host.s.live_view().is_some_and(|v| v.dialogue.is_none()),
        "an open asked 15 s ago is not replayed"
    );
}

#[test]
fn a_host_container_take_moves_the_items_and_the_guest_can_take_the_rest() {
    use testdata::fighting::ids::*;
    let data = testdata::fighting::fighting("sg-loot");
    let order = LoadOrder::from_data_dir(data.path(), &ActivePlugins::OfficialOnly).unwrap();
    let scripts = ScriptCache::default();
    let mut r = Rig::new(&order, &scripts, CELL);
    r.settled();
    let (chest, item) = (FormId(TOWN_REF), FormId(0x7777));
    r.host.gs.stocked.insert(chest);
    r.host.gs.items.insert((chest, item), 5);
    let cx = r.cx();
    r.host.s.live_loot(cx, &mut r.host.gs, LootOp::Open(chest));
    r.host.s.live_loot(
        cx,
        &mut r.host.gs,
        LootOp::Take {
            container: chest,
            item,
            n: 2,
        },
    );
    assert_eq!(
        r.host.gs.items.get(&(PLAYER_REF, item)),
        Some(&2),
        "the host holds what it took"
    );
    assert_eq!(r.host.gs.items.get(&(chest, item)), Some(&3));
    for _ in 0..40 {
        r.frame();
    }
    r.host.s.live_loot(cx, &mut r.host.gs, LootOp::Close(chest));
    // the guest sees the reduced container and takes what is left
    r.guest.s.live_intent(Intent::LootOpen(chest));
    assert!(
        r.until(2000, |r| r.guest.gs.items.get(&(chest, item)) == Some(&3)),
        "the ledger followed the host's take"
    );
    for _ in 0..40 {
        r.frame();
    }
    r.guest.s.live_intent(Intent::LootTake {
        container: chest,
        item,
        n: 3,
    });
    assert!(
        r.until(2000, |r| r.guest.gs.items.get(&(PLAYER_REF, item))
            == Some(&3)),
        "the guest got the rest"
    );
    assert!(r.until(2000, |r| r.host.gs.items.get(&(chest, item)).is_none()));
    // the host's authoritative copy of the guest's inventory has it too, so the guest's own
    // rows (re-read from that copy on every sync) keep it
    for _ in 0..200 {
        r.frame();
    }
    assert_eq!(
        r.host
            .gs
            .shadow_of(1)
            .and_then(|s| s.items.get(&item).copied()),
        Some(3),
        "credited to the guest on the host"
    );
    assert_eq!(
        r.guest.gs.items.get(&(PLAYER_REF, item)),
        Some(&3),
        "what the guest took stays in its inventory"
    );
    // a host put goes back into the container
    r.host.s.live_loot(
        cx,
        &mut r.host.gs,
        LootOp::Put {
            container: chest,
            item,
            n: 1,
        },
    );
    assert_eq!(r.host.gs.items.get(&(PLAYER_REF, item)), Some(&1));
    assert_eq!(r.host.gs.items.get(&(chest, item)), Some(&1));
}

#[test]
fn party_scaling_thins_a_dead_bodys_supplies_but_not_a_living_persons_stock() {
    use coop_core::ids::FormId as CForm;
    use testdata::fighting::ids::*;
    let data = testdata::fighting::fighting("sg-thin");
    let order = LoadOrder::from_data_dir(data.path(), &ActivePlugins::OfficialOnly).unwrap();
    let scripts = ScriptCache::default();
    let mut r = Rig::new(&order, &scripts, CELL);
    r.settled();
    // let the party-size ramp finish
    for _ in 0..600 {
        r.frame();
    }
    let s = r.host.gs.coop.live.scaling.expect("scaling in force");
    assert!(s.reward < 0.9, "two players thin loot ({})", s.reward);
    let (alive, body) = (FormId(TOWN_REF), FormId(RAIDER_REF));
    // an item the roll drops for both containers
    let item = (0x7000u32..0x9000)
        .map(FormId)
        .find(|i| {
            [alive, body]
                .iter()
                .all(|c| !s.keeps_loot(CForm(c.0 ^ i.0.rotate_left(13))))
        })
        .expect("an item the thinning roll drops");
    for c in [alive, body] {
        r.host.gs.stocked.insert(c);
        r.host.gs.items.insert((c, item), 50);
    }
    r.host.gs.dead.insert(body);
    let cx = r.cx();
    r.host.s.live_loot(cx, &mut r.host.gs, LootOp::Open(alive));
    r.host.s.live_loot(cx, &mut r.host.gs, LootOp::Open(body));
    assert_eq!(
        r.host.gs.items.get(&(alive, item)),
        Some(&50),
        "a living person's stock stays"
    );
    assert_eq!(
        r.host.gs.items.get(&(body, item)),
        None,
        "a body's supplies are thinned"
    );
}

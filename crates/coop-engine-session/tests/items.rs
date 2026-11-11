//! Loot taken once and barter through the host, between games in one process
//! (docs/coop/SPECS.md S18). The games are test stories with the item kit's stand-ins for the
//! game's inventory functions; the transport is coop-core's loopback.

use coop_core::compat::plugin_list_hash;
use coop_core::ids::PlayerId;
use coop_core::proto::{Hello, PROTOCOL_VERSION};
use coop_core::sync::HostConfig;
use coop_core::transport::{LoopbackNet, Transport};
use coop_engine::itemkit::{self as kit, HolderSpec, ItemKitSpec, ItemSpec, Kind, WorldItem};
use coop_engine::testkit::{Story, StorySpec};
use coop_engine::worldkit::{self, RefSpec, WorldSpec};
use coop_engine::{items, world};
use coop_engine_session::items::{GuestItems, HostItems};
use coop_engine_session::world::{GuestWorld, Share};
use coop_engine_session::{gate_guest, GuestDriver, HostDriver};

engine_core::assemble!();

const MQ: u32 = 0x0010_4c0c;
const CELL: u32 = 0x0010_5a00;
const DOOR: u32 = 0x0010_5a10;
const CHEST: u32 = 0x0010_5b01;
const TRADER: u32 = 0x0010_5b02;
const STOCK: u32 = 0x0010_5b03;
const BOTTLE: u32 = 0x0010_5b04;
const PLAYER: u32 = 0x14;
const CAPS: u32 = 0xf;
const STIMPAK: u32 = 0x0001_5169;
const RIFLE: u32 = 0x0000_4333;
const AMMO: u32 = 0x0000_4241;

fn machine() -> Story {
    let mut s = Story::new(&StorySpec {
        quests: vec![(MQ, vec![10, 20])],
        cells: vec![CELL],
        ..StorySpec::default()
    });
    worldkit::install(
        &mut s,
        &WorldSpec {
            refs: vec![RefSpec {
                id: DOOR,
                cell: CELL,
                opens: true,
                ..RefSpec::default()
            }],
            ..WorldSpec::default()
        },
    );
    let item = |id| ItemSpec { id, health: 0 };
    kit::install(
        &mut s,
        &ItemKitSpec {
            items: vec![
                item(STIMPAK),
                item(AMMO),
                ItemSpec {
                    id: RIFLE,
                    health: 200,
                },
            ],
            holders: vec![
                HolderSpec {
                    id: CHEST,
                    kind: Kind::Container,
                    base: vec![(AMMO, 20)],
                    items: vec![(STIMPAK, 1, None), (CAPS, 30, None)],
                    merchant: 0,
                },
                HolderSpec {
                    id: TRADER,
                    kind: Kind::Actor,
                    merchant: STOCK,
                    ..HolderSpec::default()
                },
                HolderSpec {
                    id: STOCK,
                    kind: Kind::Container,
                    base: vec![],
                    items: vec![(RIFLE, 1, None), (CAPS, 100, None)],
                    merchant: 0,
                },
            ],
            world: vec![WorldItem {
                id: BOTTLE,
                item: STIMPAK,
                count: 1,
            }],
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
    now: u64,
}

impl Run {
    /// A host and `guests` guests, joined and settled; `shared` is whether items are shared.
    fn new(guests: u8, shared: bool) -> Run {
        let net = LoopbackNet::new();
        let mut host = machine();
        let mut hd = HostDriver::new(HostConfig::new(hello(), 0x5eed), [MQ], []);
        hd.share_world(Share::Doors.host().unwrap());
        if shared {
            hd.items = Some(HostItems::new());
        }
        hd.start(&mut host.e);
        let mut run = Run {
            host,
            hd,
            hostnet: Box::new(net.endpoint(PlayerId::HOST)),
            guests: Vec::new(),
            now: 0,
        };
        for p in 1..=guests {
            let mut g = machine();
            gate_guest(&mut g.e);
            let mut d = GuestDriver::new(PlayerId(p), hello());
            d.world = Some(GuestWorld::new());
            if shared {
                d.items = Some(GuestItems::new());
            }
            // Each player starts with caps of its own.
            let player = items::holder(&mut g.e, PLAYER).unwrap();
            items::set_caps(&mut g.e, player, 500);
            run.guests.push((g, d, Box::new(net.endpoint(PlayerId(p)))));
        }
        run.frames(20);
        run
    }

    fn frames(&mut self, n: usize) {
        for _ in 0..n {
            self.now += 50;
            self.hd
                .step(&mut self.host.e, self.hostnet.as_mut(), self.now);
            for (g, d, net) in self.guests.iter_mut() {
                d.step(&mut g.e, net.as_mut(), self.now);
            }
        }
    }

    fn hi(&self) -> &HostItems {
        self.hd.items.as_ref().unwrap()
    }

    fn gi(&self, i: usize) -> &GuestItems {
        self.guests[i].1.items.as_ref().unwrap()
    }

    /// Guest `i`'s player opens holder `id`'s menu; a frame passes before it can move anything.
    fn open(&mut self, i: usize, id: u32) {
        kit::open_container(&mut self.guests[i].0, id);
    }

    /// Every game's copy of holder `id` holds what the host's does, and no hash differed.
    fn assert_same(&mut self, id: u32) {
        let hosts = kit::holding(&mut self.host, id);
        for (i, (g, d, _)) in self.guests.iter_mut().enumerate() {
            assert_eq!(
                kit::holding(g, id),
                hosts,
                "guest {} holder {id:08x}",
                i + 1
            );
            assert_eq!(d.sync.desyncs_detected, 0);
            assert_eq!(g.scripts_run(), 0);
        }
    }

    /// How many of `item` all the players and holder `id` hold together.
    fn in_all(&mut self, id: u32, item: u32) -> i32 {
        let mut n = kit::has(&mut self.host, id, item) + kit::has(&mut self.host, PLAYER, item);
        for (g, _, _) in self.guests.iter_mut() {
            n += kit::has(g, PLAYER, item);
        }
        n
    }
}

#[test]
fn a_guests_take_leaves_the_hosts_container_and_every_games() {
    let mut run = Run::new(2, true);
    run.open(0, CHEST);
    run.frames(2);
    assert_eq!(
        kit::transfer(&mut run.guests[0].0, CHEST, PLAYER, AMMO, 5),
        5
    );
    run.frames(10);
    assert_eq!(kit::has(&mut run.host, CHEST, AMMO), 15);
    assert_eq!(kit::has(&mut run.guests[0].0, PLAYER, AMMO), 5);
    assert_eq!(kit::has(&mut run.guests[1].0, PLAYER, AMMO), 0);
    run.assert_same(CHEST);
    assert_eq!((run.hi().moved, run.hi().refused), (1, 0));
    assert_eq!((run.gi(0).asked_host, run.gi(0).taken_back), (1, 0));
    assert_eq!(run.gi(0).waiting(), 0);
    // What it puts back goes into the host's container too.
    assert_eq!(
        kit::transfer(&mut run.guests[0].0, PLAYER, CHEST, AMMO, 2),
        2
    );
    run.frames(10);
    assert_eq!(kit::has(&mut run.host, CHEST, AMMO), 17);
    run.assert_same(CHEST);
    // The host's own player takes: the guests' containers follow.
    kit::open_container(&mut run.host, CHEST);
    run.frames(1);
    kit::transfer(&mut run.host, CHEST, PLAYER, CAPS, 30);
    run.frames(10);
    run.assert_same(CHEST);
    assert_eq!(kit::has(&mut run.guests[1].0, CHEST, CAPS), 0);
    // The dispatcher still moves a guest's door, and nothing of the item layer was refused by
    // the world layer.
    worldkit::open(&mut run.guests[1].0, DOOR, true);
    run.frames(10);
    assert_eq!(world::open(&mut run.host.e, DOOR), Some(true));
    assert_eq!(run.hd.world.as_ref().unwrap().refused, 0);
}

#[test]
fn two_guests_taking_the_same_item_in_the_same_moment_get_it_once() {
    let mut run = Run::new(2, true);
    run.open(0, CHEST);
    run.open(1, CHEST);
    run.frames(2);
    // Both games move the one stimpak to their own player before either hears of the other.
    for i in 0..2 {
        assert_eq!(
            kit::transfer(&mut run.guests[i].0, CHEST, PLAYER, STIMPAK, 1),
            1
        );
    }
    assert_eq!(run.in_all(CHEST, STIMPAK), 3);
    run.frames(12);
    assert_eq!(run.in_all(CHEST, STIMPAK), 1, "the stimpak exists once");
    assert_eq!(kit::has(&mut run.host, CHEST, STIMPAK), 0);
    assert_eq!(kit::has(&mut run.guests[0].0, PLAYER, STIMPAK), 1);
    assert_eq!(kit::has(&mut run.guests[1].0, PLAYER, STIMPAK), 0);
    assert_eq!((run.hi().moved, run.hi().refused), (1, 1));
    assert_eq!((run.gi(0).taken_back, run.gi(1).taken_back), (0, 1));
    run.assert_same(CHEST);
    // The host's own player and a guest in the same moment: the host's game had it first.
    kit::open_container(&mut run.host, CHEST);
    run.frames(2);
    kit::transfer(&mut run.host, CHEST, PLAYER, CAPS, 30);
    assert_eq!(
        kit::transfer(&mut run.guests[1].0, CHEST, PLAYER, CAPS, 30),
        30
    );
    run.frames(12);
    assert_eq!(kit::has(&mut run.host, PLAYER, CAPS), 30);
    assert_eq!(kit::has(&mut run.guests[1].0, PLAYER, CAPS), 500);
    run.assert_same(CHEST);
}

#[test]
fn without_item_sharing_the_same_take_duplicates() {
    // The twin of the test above: with the layer off, both guests keep the one stimpak.
    let mut run = Run::new(2, false);
    run.open(0, CHEST);
    run.open(1, CHEST);
    run.frames(2);
    for i in 0..2 {
        kit::transfer(&mut run.guests[i].0, CHEST, PLAYER, STIMPAK, 1);
    }
    run.frames(12);
    assert_eq!(run.in_all(CHEST, STIMPAK), 3);
    assert!(items::take_opened(&mut run.guests[0].0.e).is_empty());
}

#[test]
fn a_placed_item_is_picked_up_by_one_player_and_leaves_every_world() {
    let mut run = Run::new(3, true);
    assert!(kit::pick_up(&mut run.guests[0].0, BOTTLE));
    assert!(kit::pick_up(&mut run.guests[1].0, BOTTLE));
    run.frames(12);
    assert_eq!(kit::has(&mut run.guests[0].0, PLAYER, STIMPAK), 1);
    assert_eq!(kit::has(&mut run.guests[1].0, PLAYER, STIMPAK), 0);
    assert!(!items::in_world(&mut run.host.e, BOTTLE));
    assert!(!items::in_world(&mut run.guests[2].0.e, BOTTLE));
    assert_eq!((run.hi().picked, run.hi().refused), (1, 1));
    assert_eq!(run.gi(1).taken_back, 1);
    assert_eq!(run.gi(2).removed, 1);
    assert_eq!(run.gi(0).waiting() + run.gi(1).waiting(), 0);
}

#[test]
fn the_hosts_own_pick_up_wins_over_a_guests() {
    let mut run = Run::new(1, true);
    assert!(kit::pick_up(&mut run.host, BOTTLE));
    assert!(kit::pick_up(&mut run.guests[0].0, BOTTLE));
    run.frames(12);
    assert_eq!(kit::has(&mut run.host, PLAYER, STIMPAK), 1);
    assert_eq!(kit::has(&mut run.guests[0].0, PLAYER, STIMPAK), 0);
    assert_eq!(run.gi(0).taken_back, 1);
}

/// Guest `i` buys the rifle for 80 caps in its own barter menu.
fn buy_rifle(run: &mut Run, i: usize) {
    let g = &mut run.guests[i].0;
    assert_eq!(kit::transfer(g, STOCK, PLAYER, RIFLE, 1), 1);
    assert_eq!(kit::transfer(g, PLAYER, STOCK, CAPS, 80), 80);
}

#[test]
fn a_guest_buys_from_the_hosts_stock_and_a_sold_out_trade_is_refused_whole() {
    let mut run = Run::new(2, true);
    for i in 0..2 {
        kit::open_barter(&mut run.guests[i].0, TRADER);
    }
    run.frames(2);
    // Both buy the one rifle in the same moment.
    buy_rifle(&mut run, 0);
    buy_rifle(&mut run, 1);
    run.frames(12);
    assert_eq!((run.hi().traded, run.hi().refused), (1, 1));
    assert_eq!(kit::has(&mut run.host, STOCK, RIFLE), 0);
    assert_eq!(kit::has(&mut run.host, STOCK, CAPS), 180);
    assert_eq!(kit::has(&mut run.guests[0].0, PLAYER, RIFLE), 1);
    assert_eq!(kit::has(&mut run.guests[0].0, PLAYER, CAPS), 420);
    // The loser has no rifle and its caps back.
    assert_eq!(kit::has(&mut run.guests[1].0, PLAYER, RIFLE), 0);
    assert_eq!(kit::has(&mut run.guests[1].0, PLAYER, CAPS), 500);
    run.assert_same(STOCK);
    // Sales the trader's caps do not both cover: guest 1 sells for 150 of the 180 caps, guest 0
    // for 100 in the same moment. The first is paid; the second is refused whole.
    for i in 0..2 {
        let g = &mut run.guests[i].0;
        let player = items::holder(&mut g.e, PLAYER).unwrap();
        items::add(&mut g.e, player, AMMO, 10, None);
    }
    run.frames(2);
    for (i, price) in [(1usize, 150), (0, 100)] {
        let g = &mut run.guests[i].0;
        assert_eq!(kit::transfer(g, PLAYER, STOCK, AMMO, 10), 10);
        assert_eq!(kit::transfer(g, STOCK, PLAYER, CAPS, price), price);
    }
    run.frames(12);
    assert_eq!((run.hi().traded, run.hi().refused), (2, 2));
    assert_eq!(kit::has(&mut run.host, STOCK, CAPS), 80);
    assert_eq!(kit::has(&mut run.host, STOCK, AMMO), 10);
    run.assert_same(STOCK);
    let paid: Vec<(i32, i32)> = (0..2)
        .map(|i| {
            let g = &mut run.guests[i].0;
            (kit::has(g, PLAYER, CAPS), kit::has(g, PLAYER, AMMO))
        })
        .collect();
    // Guest 0 (the lower slot) is heard first: paid 100; guest 1 keeps its ammunition.
    assert_eq!(paid, [(520, 0), (500, 10)]);
}

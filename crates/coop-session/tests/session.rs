//! Headless sessions: a host and guests, each a `GameState`, driven only
//! through `Session::pre_frame` / `post_frame`.

use std::cell::Cell;
use std::rc::Rc;

use coop_core::ids::PlayerId;
use coop_core::proto::Channel;
use coop_core::transport::{ChaosProfile, LoopbackNet, Packet, Transport, TransportError};
use coop_session::{Command, Event, Session, SessionConfig, State};
use esm::{ActivePlugins, FormId, LoadOrder};
use testdata::quest_ids::*;
use world::coop::WorldBackend;
use world::dialogue::PLAYER_REF;
use world::scripting::{GameState, Runner, ScriptCache};

const DT: f32 = 0.025;

/// A transport that can be cut: while cut it drops everything both ways.
struct Cut<T: Transport> {
    inner: T,
    cut: Rc<Cell<bool>>,
}

impl<T: Transport> Transport for Cut<T> {
    fn local(&self) -> PlayerId {
        self.inner.local()
    }
    fn send(&mut self, to: PlayerId, c: Channel, d: Vec<u8>) -> Result<(), TransportError> {
        if self.cut.get() {
            return Ok(());
        }
        self.inner.send(to, c, d)
    }
    fn recv(&mut self) -> Option<Packet> {
        let p = self.inner.recv();
        if self.cut.get() {
            return None;
        }
        p
    }
}

struct Peer {
    s: Session,
    gs: GameState,
    cut: Rc<Cell<bool>>,
}

struct Rig<'a> {
    order: &'a LoadOrder,
    scripts: &'a ScriptCache,
    net: LoopbackNet,
    host: Peer,
    guests: Vec<Peer>,
    profile: Option<&'static str>,
}

fn load(tag: &str) -> (testdata::TempData, LoadOrder) {
    let data = testdata::quests(tag);
    let order = LoadOrder::from_data_dir(data.path(), &ActivePlugins::OfficialOnly).unwrap();
    (data, order)
}

fn cfg(name: &str, t: Box<dyn Transport>, chaos: Option<(&'static str, u64)>) -> SessionConfig {
    SessionConfig {
        name: name.into(),
        session_id: 0xABCD,
        transport: Some(t),
        chaos: chaos.map(|(p, seed)| (ChaosProfile::by_name(p).unwrap(), seed)),
        ..SessionConfig::default()
    }
}

impl<'a> Rig<'a> {
    fn new(
        order: &'a LoadOrder,
        scripts: &'a ScriptCache,
        profile: Option<&'static str>,
    ) -> Rig<'a> {
        let net = LoopbackNet::new();
        let cut = Rc::new(Cell::new(false));
        let ep = Cut {
            inner: net.endpoint(PlayerId::HOST),
            cut: cut.clone(),
        };
        let s = Session::host(cfg("Host", Box::new(ep), profile.map(|p| (p, 1)))).unwrap();
        Rig {
            order,
            scripts,
            net,
            host: Peer {
                s,
                gs: GameState::new(order),
                cut,
            },
            guests: vec![],
            profile,
        }
    }

    fn add_guest(&mut self, name: &str) -> usize {
        let id = PlayerId(self.guests.len() as u8 + 1);
        let cut = Rc::new(Cell::new(false));
        let ep = Cut {
            inner: self.net.endpoint(id),
            cut: cut.clone(),
        };
        let chaos = self.profile.map(|p| (p, 10 + id.0 as u64));
        let s = Session::join(cfg(name, Box::new(ep), chaos)).unwrap();
        self.guests.push(Peer {
            s,
            gs: GameState::new(self.order),
            cut,
        });
        self.guests.len() - 1
    }

    fn frame(&mut self) {
        self.host.s.pre_frame(&mut self.host.gs);
        for g in &mut self.guests {
            g.s.pre_frame(&mut g.gs);
        }
        self.host.s.post_frame(&mut self.host.gs, DT);
        for g in &mut self.guests {
            g.s.post_frame(&mut g.gs, DT);
        }
    }

    fn frames(&mut self, n: u32) {
        for _ in 0..n {
            self.frame();
        }
    }

    fn same(&self) -> bool {
        let h = self.host.gs.shard_hashes();
        self.guests
            .iter()
            .all(|g| g.s.is_connected() && g.gs.shard_hashes() == h)
    }

    fn settle(&mut self) -> bool {
        for _ in 0..12_000 {
            self.frame();
            if self.same() {
                self.frames(40);
                if self.same() {
                    return true;
                }
            }
        }
        false
    }

    fn host_plays(&mut self) {
        let mut r = Runner::new(self.order, self.scripts, &mut self.host.gs);
        assert!(r.set_stage(FormId(QUEST), 10));
        for _ in 0..4 {
            r.update(1.0);
        }
    }
}

#[test]
fn host_and_two_guests_converge_on_a_perfect_link() {
    let (_d, order) = load("cs-perfect");
    let scripts = ScriptCache::default();
    let mut rig = Rig::new(&order, &scripts, None);
    rig.host.gs.player_cell = Some(FormId(CELL));
    rig.add_guest("Ann");
    rig.add_guest("Bo");
    rig.frames(20);
    rig.host_plays();
    rig.host
        .gs
        .move_item(&order, PLAYER_REF, FormId(DOC_REF), FormId(CAPS), 5);
    rig.host
        .gs
        .emit(world::coop::Domain::Items, FormId(DOC_REF));
    assert!(rig.settle());
    for g in &rig.guests {
        assert_eq!(g.gs.stages.get(&FormId(QUEST)), Some(&20));
        assert_eq!(g.gs.authority(), world::coop::Authority::Guest);
        assert_eq!(g.s.host_place().map(|p| p.cell), Some(CELL));
        assert_eq!(g.s.status().state, State::Connected);
    }
    // names reach the host, and the host's roster reaches the guests
    let st = rig.host.s.status();
    let names: Vec<_> = st.peers.iter().map(|p| p.name.as_str()).collect();
    assert_eq!(names, ["Host", "Ann", "Bo"]);
    assert_eq!(rig.guests[0].s.status().peers.len(), 2);
    let ev = rig.host.s.drain_events();
    assert_eq!(
        ev.iter()
            .filter(|e| matches!(e, Event::PeerJoined(..)))
            .count(),
        2
    );
    assert!(rig.guests[0]
        .s
        .status()
        .line()
        .starts_with("CO-OP guest | ping"));
    assert!(rig
        .host
        .s
        .status()
        .line()
        .starts_with("CO-OP host | 2 guests"));
}

#[test]
fn converges_under_chaos_profiles() {
    for profile in ["bad_wifi", "flaky", "partition"] {
        let (_d, order) = load(&format!("cs-{profile}"));
        let scripts = ScriptCache::default();
        let mut rig = Rig::new(&order, &scripts, Some(profile));
        rig.add_guest("Ann");
        rig.add_guest("Bo");
        rig.frames(40);
        rig.host_plays();
        assert!(rig.settle(), "profile {profile} did not settle");
        assert_eq!(
            rig.guests[1].gs.stages.get(&FormId(QUEST)),
            Some(&20),
            "{profile}"
        );
    }
}

#[test]
fn late_joiner_catches_up_and_gets_the_place() {
    let (_d, order) = load("cs-late");
    let scripts = ScriptCache::default();
    let mut rig = Rig::new(&order, &scripts, Some("good_wifi"));
    rig.host.gs.player_cell = Some(FormId(CELL));
    rig.add_guest("Ann");
    rig.frames(20);
    rig.host_plays();
    assert!(rig.settle());
    rig.guests[0].gs.events.clear();
    let i = rig.add_guest("Late");
    assert!(rig.settle());
    assert_eq!(rig.guests[i].gs.stages.get(&FormId(QUEST)), Some(&20));
    assert_eq!(rig.guests[i].s.host_place().map(|p| p.cell), Some(CELL));
    assert!(
        rig.guests[i].gs.events.is_empty(),
        "the join snapshot must not replay the journal"
    );
    assert!(rig.guests[0].gs.events.is_empty());
}

#[test]
fn guest_reconnects_after_a_short_and_a_long_cut() {
    for (secs, tag) in [(6.0f32, "short"), (14.0, "long")] {
        let (_d, order) = load(&format!("cs-cut-{tag}"));
        let scripts = ScriptCache::default();
        let mut rig = Rig::new(&order, &scripts, None);
        rig.add_guest("Ann");
        rig.frames(40);
        assert!(rig.settle());
        rig.guests[0].cut.set(true);
        rig.guests[0].s.drain_events();
        // the host moves on while the guest is away
        rig.host_plays();
        rig.frames((secs / DT) as u32);
        assert_eq!(rig.guests[0].s.status().state, State::Reconnecting, "{tag}");
        rig.guests[0].cut.set(false);
        assert!(rig.settle(), "{tag}: did not reconverge");
        assert_eq!(rig.guests[0].gs.stages.get(&FormId(QUEST)), Some(&20));
        let ev = rig.guests[0].s.drain_events();
        assert!(ev.contains(&Event::Lost), "{tag}: {ev:?}");
        assert!(ev.contains(&Event::Reconnected), "{tag}: {ev:?}");
        if secs > 10.0 {
            let host_ev = rig.host.s.drain_events();
            assert!(
                host_ev.iter().any(|e| matches!(e, Event::PeerLeft(..))),
                "{tag}"
            );
        }
    }
}

#[test]
fn guest_commands_run_on_the_host_exactly_once() {
    let (_d, order) = load("cs-cmd");
    let scripts = ScriptCache::default();
    let mut rig = Rig::new(&order, &scripts, Some("flaky"));
    rig.add_guest("Ann");
    // queued before the guest is even connected
    rig.guests[0]
        .s
        .send_command(Command::Stage("TestQuest".into(), 10));
    rig.guests[0]
        .s
        .send_command(Command::Line("set TestGlobal to 9".into()));
    let mut got = Vec::new();
    for _ in 0..4000 {
        rig.frame();
        for (from, c) in rig.host.s.take_commands() {
            assert_eq!(from, PlayerId(1));
            got.push(c);
        }
        if rig.guests[0].s.status().pending_commands == 0 && got.len() == 2 {
            break;
        }
    }
    rig.frames(200);
    got.extend(rig.host.s.take_commands().into_iter().map(|(_, c)| c));
    assert_eq!(
        got,
        [
            Command::Stage("TestQuest".into(), 10),
            Command::Line("set TestGlobal to 9".into())
        ]
    );
    assert_eq!(rig.guests[0].s.status().pending_commands, 0);
    // apply them the way the viewer does and watch the guest follow
    for c in &got {
        Runner::new(&order, &scripts, &mut rig.host.gs).run_source(&c.as_line(), None, None);
    }
    for _ in 0..4 {
        Runner::new(&order, &scripts, &mut rig.host.gs).update(1.0);
    }
    assert!(rig.settle());
    assert_eq!(rig.guests[0].gs.stages.get(&FormId(QUEST)), Some(&20));
}

#[test]
fn log_and_state_dump_are_written_and_comparable() {
    let (_d, order) = load("cs-out");
    let scripts = ScriptCache::default();
    let dir = std::env::temp_dir().join(format!("coop-session-test-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let net = LoopbackNet::new();
    let mut hc = cfg("Host", Box::new(net.endpoint(PlayerId::HOST)), None);
    hc.log = Some(dir.join("h.jsonl"));
    hc.fields_dump = Some(dir.join("h.dump"));
    let mut gc = cfg("Ann", Box::new(net.endpoint(PlayerId(1))), None);
    gc.fields_dump = Some(dir.join("g.dump"));
    let (mut h, mut g) = (Session::host(hc).unwrap(), Session::join(gc).unwrap());
    let (mut hs, mut gs) = (GameState::new(&order), GameState::new(&order));
    for i in 0..400 {
        h.pre_frame(&mut hs);
        g.pre_frame(&mut gs);
        if i == 20 {
            let mut r = Runner::new(&order, &scripts, &mut hs);
            r.set_stage(FormId(QUEST), 10);
            r.update(1.0);
        }
        h.post_frame(&mut hs, DT);
        g.post_frame(&mut gs, DT);
    }
    h.dump_now(&hs);
    g.dump_now(&gs);
    let hd = std::fs::read_to_string(dir.join("h.dump")).unwrap();
    let gd = std::fs::read_to_string(dir.join("g.dump")).unwrap();
    let (hh, hl) = hd.split_once('\n').unwrap();
    let (gh, gl) = gd.split_once('\n').unwrap();
    assert_eq!(hl, gl, "the field lines must be identical");
    assert!(hh.contains("\"role\":\"host\"") && gh.contains("\"role\":\"guest\""));
    let root = |s: &str| s.split("\"root\":\"").nth(1).unwrap()[..16].to_string();
    assert_eq!(root(hh), root(gh));
    let log = std::fs::read_to_string(dir.join("h.jsonl")).unwrap();
    assert!(log.lines().next().unwrap().contains("\"ev\":\"start\""));
    assert!(log.contains("\"ev\":\"peer_join\""));
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn real_udp_on_localhost() {
    let (_d, order) = load("cs-udp");
    let scripts = ScriptCache::default();
    let port = {
        let s = std::net::UdpSocket::bind("127.0.0.1:0").unwrap();
        s.local_addr().unwrap().port()
    };
    let mut h = Session::host(SessionConfig {
        name: "Host".into(),
        addr: port.to_string(),
        session_id: 7,
        ..SessionConfig::default()
    })
    .unwrap();
    let mut g = Session::join(SessionConfig {
        name: "Ann".into(),
        addr: format!("127.0.0.1:{port}"),
        ..SessionConfig::default()
    })
    .unwrap();
    let (mut hs, mut gs) = (GameState::new(&order), GameState::new(&order));
    hs.player_cell = Some(FormId(CELL));
    let start = std::time::Instant::now();
    let mut played = false;
    let mut ok = false;
    while start.elapsed().as_secs() < 20 {
        h.pre_frame(&mut hs);
        g.pre_frame(&mut gs);
        if !played && g.is_connected() {
            played = true;
            let mut r = Runner::new(&order, &scripts, &mut hs);
            r.set_stage(FormId(QUEST), 10);
            for _ in 0..4 {
                r.update(1.0);
            }
        }
        h.post_frame(&mut hs, 0.01);
        g.post_frame(&mut gs, 0.01);
        std::thread::sleep(std::time::Duration::from_millis(10));
        if played
            && gs.stages.get(&FormId(QUEST)) == Some(&20)
            && hs.shard_hashes() == gs.shard_hashes()
        {
            ok = true;
            break;
        }
    }
    assert!(ok, "no convergence over UDP: {:?}", g.status());
    assert_eq!(g.host_place().map(|p| p.cell), Some(CELL));
    assert!(h.take_link_errors().is_empty());
}

#[test]
fn a_mismatched_build_is_refused() {
    let (_d, order) = load("cs-reject");
    let net = LoopbackNet::new();
    let mut hc = cfg("Host", Box::new(net.endpoint(PlayerId::HOST)), None);
    hc.build_hash = 1;
    let mut gc = cfg("Ann", Box::new(net.endpoint(PlayerId(1))), None);
    gc.build_hash = 2;
    let (mut h, mut g) = (Session::host(hc).unwrap(), Session::join(gc).unwrap());
    let (mut hs, mut gs) = (GameState::new(&order), GameState::new(&order));
    for _ in 0..40 {
        h.pre_frame(&mut hs);
        g.pre_frame(&mut gs);
        h.post_frame(&mut hs, DT);
        g.post_frame(&mut gs, DT);
    }
    assert!(matches!(g.status().state, State::Rejected(_)));
    assert!(g
        .drain_events()
        .iter()
        .any(|e| matches!(e, Event::Rejected(_))));
    assert!(!g.is_connected());
}

#[test]
fn harness_dump_has_the_contract_keys_and_ready_is_reported() {
    let (_d, order) = load("cs-json");
    let scripts = ScriptCache::default();
    let mut rig = Rig::new(&order, &scripts, None);
    rig.add_guest("Ann");
    assert!(!rig.host.s.is_ready());
    rig.frames(10);
    rig.host_plays();
    assert!(rig.settle());
    assert!(rig.host.s.is_ready() && rig.guests[0].s.is_ready());
    let root = |p: &Peer| coop_session::Session::state_root(&p.gs);
    let h = coop_session::dump::state_json(
        &rig.host.gs,
        &order,
        coop_session::Role::Host,
        1.5,
        3,
        root(&rig.host),
    );
    let g = coop_session::dump::state_json(
        &rig.guests[0].gs,
        &order,
        coop_session::Role::Guest,
        1.5,
        3,
        root(&rig.guests[0]),
    );
    for key in [
        "\"schema\":1",
        "\"role\":\"host\"",
        "\"TestQuest\":{\"stage\":20",
        "\"quests\"",
        "\"globals\"",
        "\"items\"",
        "\"positions\"",
        "\"hash\":{\"shared\":",
    ] {
        assert!(h.contains(key), "{key} missing in {h}");
    }
    let hash = |s: &str| s.split("\"shared\":\"").nth(1).unwrap()[..16].to_string();
    assert_eq!(hash(&h), hash(&g));
    assert!(rig.host.s.drain_events().contains(&Event::Ready));
}

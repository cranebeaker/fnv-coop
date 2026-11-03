//! Voice frames, markers and pings, weather and clock, and the join
//! compatibility messages, through `Session` (the calls the viewer makes).

use coop_core::ids::PlayerId;
use coop_core::social::compass::{PingBook, KIND_PING};
use coop_core::social::SocialMsg;
use coop_core::transport::LoopbackNet;
use coop_session::{CompatConfig, Event, Intent, Session, SessionConfig, State};
use esm::{ActivePlugins, FormId, LoadOrder};
use world::coop::apply::{Ctx, Opts};
use world::scripting::{GameState, ScriptCache};

const DT: f32 = 0.025;
const CELL: u32 = 0x0100_0001;

struct Peer {
    s: Session,
    gs: GameState,
    voice: Vec<(PlayerId, Vec<u8>)>,
    markers: usize,
}

struct Rig<'a> {
    order: &'a LoadOrder,
    scripts: &'a ScriptCache,
    host: Peer,
    guest: Peer,
}

fn cfg(
    name: &str,
    t: Box<dyn coop_core::transport::Transport>,
    compat: CompatConfig,
) -> SessionConfig {
    SessionConfig {
        name: name.into(),
        session_id: 0xABCD,
        transport: Some(t),
        live: true,
        compat,
        ..SessionConfig::default()
    }
}

impl<'a> Rig<'a> {
    fn new(
        order: &'a LoadOrder,
        scripts: &'a ScriptCache,
        hc: CompatConfig,
        gc: CompatConfig,
    ) -> Rig<'a> {
        let net = LoopbackNet::new();
        let hs = Session::host(cfg("Host", Box::new(net.endpoint(PlayerId::HOST)), hc)).unwrap();
        let gsn = Session::join(cfg("Ann", Box::new(net.endpoint(PlayerId(1))), gc)).unwrap();
        let mk = |s| Peer {
            s,
            gs: GameState::new(order),
            voice: Vec::new(),
            markers: 0,
        };
        Rig {
            order,
            scripts,
            host: mk(hs),
            guest: mk(gsn),
        }
    }

    fn frame(&mut self) {
        let cx = Ctx {
            order: self.order,
            scripts: self.scripts,
        };
        for p in [&mut self.host, &mut self.guest] {
            p.s.pre_frame_with(
                &mut p.gs,
                Opts {
                    present: true,
                    ctx: Some(cx),
                },
            );
            p.gs.player_cell = Some(FormId(CELL));
            p.gs.player_position = Some([900.0, 1000.0, 0.0]);
            if let Some(v) = p.s.live_frame(&mut p.gs, cx, DT) {
                p.voice
                    .extend(v.voice.iter().map(|f| (f.speaker, f.bytes.clone())));
                p.markers = p.markers.max(v.markers.len());
            }
            p.s.post_frame(&mut p.gs, DT);
        }
    }

    fn frames(&mut self, n: u32) {
        for _ in 0..n {
            self.frame();
        }
    }

    fn settled(&mut self) {
        for _ in 0..8000 {
            if self.guest.s.is_connected() && self.host.s.is_ready() && self.guest.s.is_ready() {
                self.frames(60);
                return;
            }
            self.frame();
        }
        panic!("never connected");
    }
}

fn world(tag: &str) -> (testdata::TempData, LoadOrder) {
    let data = testdata::fighting::fighting(tag);
    let order = LoadOrder::from_data_dir(data.path(), &ActivePlugins::OfficialOnly).unwrap();
    (data, order)
}

/// An envelope-shaped frame: 12 byte header with the sequence at 4..6.
fn frame_bytes(seq: u16) -> Vec<u8> {
    let mut v = vec![1, 0, 1, 0];
    v.extend_from_slice(&seq.to_le_bytes());
    v.extend_from_slice(&[0; 4]);
    v.extend_from_slice(&3u16.to_le_bytes());
    v.extend_from_slice(&[7, 8, 9]);
    v
}

#[test]
fn voice_frames_go_both_ways_in_order_with_the_transports_speaker() {
    let (_d, order) = world("ls-voice");
    let scripts = ScriptCache::default();
    let mut r = Rig::new(
        &order,
        &scripts,
        CompatConfig::default(),
        CompatConfig::default(),
    );
    r.settled();
    for seq in 0..10u16 {
        r.guest.s.live_intent(Intent::Voice(frame_bytes(seq)));
        r.host.s.live_intent(Intent::Voice(frame_bytes(seq)));
        r.frames(2);
    }
    r.frames(20);
    assert_eq!(r.host.voice.len(), 10, "host heard the guest");
    assert!(r.host.voice.iter().all(|(who, _)| *who == PlayerId(1)));
    assert_eq!(r.guest.voice.len(), 10, "guest heard the host");
    assert!(r.guest.voice.iter().all(|(who, _)| who.is_host()));
    let seqs: Vec<u16> = r
        .host
        .voice
        .iter()
        .map(|(_, b)| u16::from_le_bytes([b[4], b[5]]))
        .collect();
    assert_eq!(seqs, (0..10).collect::<Vec<_>>());
}

#[test]
fn voice_before_the_session_is_ready_is_dropped_not_queued() {
    let (_d, order) = world("ls-voice-early");
    let scripts = ScriptCache::default();
    let mut r = Rig::new(
        &order,
        &scripts,
        CompatConfig::default(),
        CompatConfig::default(),
    );
    r.guest.s.live_intent(Intent::Voice(frame_bytes(0)));
    r.settled();
    r.frames(40);
    assert!(r.host.voice.is_empty(), "stale audio is never played late");
}

#[test]
fn a_ping_reaches_the_other_player_and_expires() {
    let (_d, order) = world("ls-ping");
    let scripts = ScriptCache::default();
    let mut r = Rig::new(
        &order,
        &scripts,
        CompatConfig::default(),
        CompatConfig::default(),
    );
    r.settled();
    let mut book = PingBook::new();
    let msgs = book
        .ping(PlayerId(1), CELL, [1200.0, 800.0, 0.0], "look", 1000)
        .unwrap();
    for m in msgs {
        r.guest.s.live_intent(Intent::Social(m));
    }
    r.frames(40);
    assert_eq!(r.host.markers, 1, "the host sees the guest's ping");
    let seen = r.host.s.live_view().unwrap().markers.clone();
    assert_eq!(seen.len(), 1);
    assert_eq!(
        (
            seen[0].owner,
            seen[0].kind,
            seen[0].cell,
            seen[0].label.as_str()
        ),
        (PlayerId(1), KIND_PING, CELL, "look")
    );
    assert!((seen[0].pos[0] - 1200.0).abs() < 1.0);
    // the host expires it after its lifetime (session time)
    let frames = (coop_core::social::compass::PING_TTL_MS as f32 / 1000.0 / DT) as u32 + 120;
    r.frames(frames);
    assert!(r.host.s.live_view().unwrap().markers.is_empty(), "expired");
}

#[test]
fn a_waypoint_is_set_and_cleared() {
    let (_d, order) = world("ls-way");
    let scripts = ScriptCache::default();
    let mut r = Rig::new(
        &order,
        &scripts,
        CompatConfig::default(),
        CompatConfig::default(),
    );
    r.settled();
    let mut book = PingBook::new();
    r.host.s.live_intent(Intent::Social(book.waypoint(
        PlayerId::HOST,
        CELL,
        [5.0, 6.0, 0.0],
        "camp",
    )));
    r.frames(40);
    assert_eq!(r.guest.s.live_view().unwrap().markers.len(), 1);
    r.host
        .s
        .live_intent(Intent::Social(book.clear_waypoint().unwrap()));
    r.frames(40);
    assert!(r.guest.s.live_view().unwrap().markers.is_empty());
    let _ = SocialMsg::MarkerClear { id: 1 };
}

#[test]
fn the_guests_sky_and_clock_follow_the_host() {
    let (_d, order) = world("ls-env");
    let scripts = ScriptCache::default();
    let mut r = Rig::new(
        &order,
        &scripts,
        CompatConfig::default(),
        CompatConfig::default(),
    );
    r.settled();
    r.host.gs.weather.force(FormId(0x100), false, 8.0);
    r.frames(60);
    assert_eq!(
        r.guest.gs.weather.current,
        Some(FormId(0x100)),
        "guest shows the host's weather"
    );
    assert_eq!(
        r.guest.gs.weather.forced,
        Some(FormId(0x100)),
        "and never rolls its own"
    );
    // the host starts a fade into rain
    r.host.gs.weather.set(FormId(0x200), true, 9.0);
    r.host.gs.weather.previous = r.host.gs.weather.current;
    r.host.gs.weather.current = Some(FormId(0x200));
    r.host.gs.weather.started = 9.0;
    r.host.gs.weather.fade = 0.2;
    r.frames(60);
    assert_eq!(r.guest.gs.weather.current, Some(FormId(0x200)));
    assert_eq!(
        r.guest.gs.weather.previous,
        Some(FormId(0x100)),
        "fading, not cut"
    );
    assert!(
        r.guest.s.live_view().unwrap().hour.is_some(),
        "the smoothed clock runs"
    );
    // the game's own weather step drops the override (a new climate): the host's sky comes back
    r.guest.gs.weather.forced = None;
    r.guest.gs.weather.current = Some(FormId(0x999));
    r.guest.gs.weather.previous = None;
    r.frames(4);
    assert_eq!(r.guest.gs.weather.forced, Some(FormId(0x200)));
    assert_eq!(r.guest.gs.weather.current, Some(FormId(0x200)));
}

fn plugins(v: &[&str]) -> CompatConfig {
    CompatConfig {
        plugins: v.iter().map(|s| s.to_string()).collect(),
        exe: "1.4.0.525".into(),
        ..CompatConfig::default()
    }
}

#[test]
fn a_missing_plugin_is_refused_with_a_clear_message_on_both_sides() {
    let (_d, order) = world("ls-compat");
    let scripts = ScriptCache::default();
    let mut r = Rig::new(
        &order,
        &scripts,
        plugins(&["FalloutNV.esm", "DeadMoney.esm"]),
        plugins(&["FalloutNV.esm"]),
    );
    r.frames(400);
    assert!(
        matches!(r.guest.s.status().state, State::Rejected(_)),
        "{:?}",
        r.guest.s.status().state
    );
    let g_events = r.guest.s.drain_events();
    let lines: Vec<String> = g_events
        .iter()
        .filter_map(|e| {
            if let Event::Compat {
                lines,
                refused: true,
            } = e
            {
                Some(lines.join(" "))
            } else {
                None
            }
        })
        .collect();
    assert!(!lines.is_empty(), "{g_events:?}");
    assert!(
        lines[0].contains("cannot join") && lines[0].contains("DeadMoney.esm"),
        "{}",
        lines[0]
    );
    let h_events = r.host.s.drain_events();
    assert!(h_events.iter().any(|e| matches!(e, Event::Compat { lines, refused: true } if lines.join(" ").contains("They lack: DeadMoney.esm"))), "{h_events:?}");
    assert!(
        r.guest.s.status().line().contains("plugins"),
        "{}",
        r.guest.s.status().line()
    );
}

#[test]
fn the_override_lets_a_mismatched_guest_in_and_both_are_told() {
    let (_d, order) = world("ls-compat-ok");
    let scripts = ScriptCache::default();
    let mut host = plugins(&["FalloutNV.esm", "A.esp"]);
    host.allow_mismatch = true;
    let mut r = Rig::new(&order, &scripts, host, plugins(&["FalloutNV.esm", "B.esp"]));
    r.settled();
    let g = r.guest.s.drain_events();
    assert!(
        g.iter()
            .any(|e| matches!(e, Event::Compat { refused: false, .. })),
        "{g:?}"
    );
    let h = r.host.s.drain_events();
    assert!(
        h.iter()
            .any(|e| matches!(e, Event::Compat { refused: false, .. })),
        "{h:?}"
    );
}

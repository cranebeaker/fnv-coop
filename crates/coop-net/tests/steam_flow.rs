//! Whole host/guest flow over MockSteam (needs `--features steam`).

use std::time::Duration;

use coop_core::ids::PlayerId;
use coop_core::proto::Channel;
use coop_core::transport::Transport;
use coop_net::steam::*;
use coop_net::{
    Backend, Clock, Endpoint, EndpointConfig, JoinCode, JoinTarget, LeaveReason, ManualClock,
    NetEvent, Readiness,
};

const TOKEN: [u8; 16] = [4; 16];

fn opts() -> SteamOptions {
    SteamOptions {
        retry_sleep: Duration::from_millis(1),
        ..Default::default()
    }
}

fn cfg(c: &Clock, n: &str) -> EndpointConfig {
    EndpointConfig {
        name: n.into(),
        clock: c.clone(),
        ..Default::default()
    }
}

struct Rig {
    uni: MockUniverse,
    mc: ManualClock,
    host: Endpoint<SteamBackend<MockSteam>>,
    lobby: u64,
}

fn rig() -> (Rig, MockSteam, MockSteam) {
    let uni = MockUniverse::new();
    let h = uni.user(1, "host");
    let g = uni.user(2, "guest");
    let stranger = uni.user(3, "stranger");
    uni.befriend(&h, &g);
    let mc = ManualClock::new();
    let clock = Clock::Manual(mc.clone());
    let mut host = Endpoint::host(SteamBackend::host(h, opts()), TOKEN, cfg(&clock, "h"));
    host.pump();
    let lobby = host.backend().lobby().expect("lobby created");
    (
        Rig {
            uni,
            mc,
            host,
            lobby,
        },
        g,
        stranger,
    )
}

fn settle<A: Backend, B: Backend>(a: &mut Endpoint<A>, b: &mut Endpoint<B>) {
    for _ in 0..10 {
        a.pump();
        b.pump();
    }
}

#[test]
fn host_creates_marked_lobby_idempotently() {
    let (r, g, _) = rig();
    // LobbyCreated was delivered twice; the phase is Hosting once and data set once.
    assert!(matches!(
        r.host.backend().phase(),
        SteamPhase::Hosting { .. }
    ));
    assert_eq!(g.get_lobby_data(r.lobby, MARKER_KEY).as_deref(), Some("1"));
    assert_eq!(g.get_lobby_data(r.lobby, "tag").as_deref(), Some("1"));
    assert_eq!(g.get_lobby_data(r.lobby, "mode").as_deref(), Some("lobby"));
    assert_eq!(
        r.host.backend().api().rich_presence("connect"),
        Some(format!("+connect_lobby {}", r.lobby))
    );
    let code = r.host.backend().join_code(TOKEN).unwrap().to_string();
    assert_eq!(
        code.parse::<JoinCode>().unwrap().target,
        JoinTarget::SteamLobby(r.lobby)
    );
}

#[test]
fn full_flow_with_all_channels_and_big_blob() {
    let (mut r, g, _) = rig();
    let clock = Clock::Manual(r.mc.clone());
    let mut guest = Endpoint::guest(
        SteamBackend::guest(g, r.lobby, opts()),
        TOKEN,
        cfg(&clock, "g"),
    );
    settle(&mut r.host, &mut guest);
    assert!(guest.is_connected());
    assert_eq!(guest.local(), PlayerId(1));
    assert!(r.host.events().contains(&NetEvent::PeerJoined {
        player: PlayerId(1),
        name: "g".into()
    }));

    let blob: Vec<u8> = (0..400_000u32).map(|i| (i % 253) as u8).collect();
    r.host
        .send(PlayerId(1), Channel::ReliableOrdered, blob.clone())
        .unwrap();
    r.host
        .send(PlayerId(1), Channel::Unreliable, b"pos".to_vec())
        .unwrap();
    guest
        .send(PlayerId(0), Channel::ReliableOrdered, b"hi".to_vec())
        .unwrap();
    settle(&mut r.host, &mut guest);
    let got: Vec<_> = std::iter::from_fn(|| guest.recv()).collect();
    assert!(got
        .iter()
        .any(|p| p.channel == Channel::ReliableOrdered && p.data == blob));
    assert!(got
        .iter()
        .any(|p| p.channel == Channel::Unreliable && p.data == b"pos"));
    assert_eq!(r.host.recv().unwrap().data, b"hi");
}

#[test]
fn works_in_either_event_order() {
    let (mut r, g, _) = rig();
    r.uni.faults(|f| f.reverse_event_order = true);
    let clock = Clock::Manual(r.mc.clone());
    let mut guest = Endpoint::guest(
        SteamBackend::guest(g, r.lobby, opts()),
        TOKEN,
        cfg(&clock, "g"),
    );
    settle(&mut r.host, &mut guest);
    assert!(guest.is_connected());
}

#[test]
fn stranger_cannot_join_or_talk() {
    let (mut r, _g, stranger) = rig();
    // Not a friend: the lobby refuses.
    let clock = Clock::Manual(r.mc.clone());
    let sb = SteamBackend::guest(stranger, r.lobby, opts());
    let mut s = Endpoint::guest(sb, TOKEN, cfg(&clock, "s"));
    settle(&mut r.host, &mut s);
    assert!(s
        .events()
        .iter()
        .any(|e| matches!(e, NetEvent::JoinFailed(m) if m.contains("not allowed"))));

    // A stranger who messages the host directly is held and never delivered.
    let mut stranger2 = r.uni.user(9, "intruder");
    stranger2.send_message(r.host.backend().api().self_id(), 1, 9, b"let me in");
    settle(&mut r.host, &mut s);
    assert!(r.host.players().is_empty());
    assert_eq!(r.host.backend().rejected_sessions, 1);
    let _ = &mut stranger2;
}

#[test]
fn wrong_marker_and_wrong_tag_are_refused() {
    let uni = MockUniverse::new();
    let mut other_game = uni.user(1, "someone");
    let g = uni.user(2, "guest");
    uni.befriend(&other_game, &g);
    other_game.create_lobby(4);
    let lobby = other_game
        .run_frame()
        .into_iter()
        .find_map(|e| {
            if let SteamEvent::LobbyCreated { lobby, .. } = e {
                Some(lobby)
            } else {
                None
            }
        })
        .unwrap();
    // A plain Spacewar lobby without our marker key.
    let mut b = SteamBackend::guest(g, lobby, opts());
    b.poll(0);
    assert_eq!(
        b.readiness(),
        Readiness::Failed("that lobby is not an nv-rs co-op game".into())
    );

    // Right marker, wrong tag.
    let mut h = uni.user(5, "h2");
    let g2 = uni.user(6, "g2");
    uni.befriend(&h, &g2);
    h.create_lobby(2);
    let l2 = h
        .run_frame()
        .into_iter()
        .find_map(|e| {
            if let SteamEvent::LobbyCreated { lobby, .. } = e {
                Some(lobby)
            } else {
                None
            }
        })
        .unwrap();
    h.set_lobby_data(l2, MARKER_KEY, "1");
    h.set_lobby_data(l2, "tag", "2");
    let mut b2 = SteamBackend::guest(g2, l2, opts());
    b2.poll(0);
    assert!(matches!(b2.readiness(), Readiness::Failed(m) if m.contains("different version")));
}

#[test]
fn invite_paths_reach_the_guest() {
    let (mut r, g, _) = rig();
    let gid = g.self_id();
    let mut gb = SteamBackend::guest(g, r.lobby, opts());
    r.host.backend_mut().invite_friend(gid);
    gb.poll(0);
    assert_eq!(
        gb.take_invites(),
        vec![Invite {
            lobby: r.lobby,
            from: r.host.backend().api().self_id()
        }]
    );
    // Hosting friends are discoverable for the "Friend X is hosting" button.
    assert_eq!(gb.hosting_friends().len(), 1);
    assert_eq!(gb.online_friends().len(), 1);
    // Join Game click and rich presence connect strings.
    gb.api().fake_join_request(77, 5);
    gb.poll(0);
    assert_eq!(gb.take_invites(), vec![Invite { lobby: 77, from: 5 }]);
}

#[test]
fn buffer_full_is_retried_not_dropped() {
    let (mut r, g, _) = rig();
    let clock = Clock::Manual(r.mc.clone());
    let gid = g.self_id();
    let mut guest = Endpoint::guest(
        SteamBackend::guest(g, r.lobby, opts()),
        TOKEN,
        cfg(&clock, "g"),
    );
    settle(&mut r.host, &mut guest);
    let hid = r.host.backend().api().self_id();
    r.uni.faults(|f| {
        f.buffer_full.insert(hid, 5);
    });
    r.host
        .send(
            PlayerId(1),
            Channel::ReliableOrdered,
            b"must arrive".to_vec(),
        )
        .unwrap();
    assert_eq!(r.host.backend().buffer_full_retries, 5);
    settle(&mut r.host, &mut guest);
    assert_eq!(guest.recv().unwrap().data, b"must arrive");
    let _ = gid;
}

#[test]
fn lobby_leave_and_session_failure_drop_the_peer() {
    let (mut r, g, _) = rig();
    let clock = Clock::Manual(r.mc.clone());
    let mut guest = Endpoint::guest(
        SteamBackend::guest(g, r.lobby, opts()),
        TOKEN,
        cfg(&clock, "g"),
    );
    settle(&mut r.host, &mut guest);
    r.host.events();
    // Crash: no goodbye, only the lobby flag.
    let gid = guest.backend().api().self_id();
    let crasher = guest.backend().api().clone_handle();
    r.uni.crash(&crasher);
    settle(&mut r.host, &mut guest);
    assert!(r.host.events().contains(&NetEvent::PeerLeft {
        player: PlayerId(1),
        reason: LeaveReason::BackendGone
    }));
    let _ = gid;
}

#[test]
fn silent_peer_times_out_over_steam() {
    let (mut r, g, _) = rig();
    let clock = Clock::Manual(r.mc.clone());
    let mut guest = Endpoint::guest(
        SteamBackend::guest(g, r.lobby, opts()),
        TOKEN,
        cfg(&clock, "g"),
    );
    settle(&mut r.host, &mut guest);
    r.host.events();
    let gid = guest.backend().api().self_id();
    r.uni.faults(|f| {
        f.silent.insert(gid);
    });
    for _ in 0..21 {
        r.mc.advance(1000);
        settle(&mut r.host, &mut guest);
    }
    assert!(r.host.events().contains(&NetEvent::PeerLeft {
        player: PlayerId(1),
        reason: LeaveReason::Timeout
    }));
}

#[test]
fn guest_sees_host_gone_when_lobby_owner_leaves() {
    let (mut r, g, _) = rig();
    let clock = Clock::Manual(r.mc.clone());
    let mut guest = Endpoint::guest(
        SteamBackend::guest(g, r.lobby, opts()),
        TOKEN,
        cfg(&clock, "g"),
    );
    settle(&mut r.host, &mut guest);
    guest.events();
    r.host.backend_mut().leave();
    settle(&mut r.host, &mut guest);
    assert!(guest
        .events()
        .iter()
        .any(|e| matches!(e, NetEvent::PeerLeft { player, .. } if *player == PlayerId(0))));
}

#[test]
fn unreliable_loss_does_not_break_reliable() {
    let (mut r, g, _) = rig();
    r.uni.faults(|f| f.unreliable_loss = 0.5);
    let clock = Clock::Manual(r.mc.clone());
    let mut guest = Endpoint::guest(
        SteamBackend::guest(g, r.lobby, opts()),
        TOKEN,
        cfg(&clock, "g"),
    );
    settle(&mut r.host, &mut guest);
    for i in 0..100u8 {
        r.host
            .send(PlayerId(1), Channel::Unreliable, vec![i])
            .unwrap();
        r.host
            .send(PlayerId(1), Channel::ReliableOrdered, vec![i])
            .unwrap();
    }
    settle(&mut r.host, &mut guest);
    let got: Vec<_> = std::iter::from_fn(|| guest.recv()).collect();
    let rel: Vec<u8> = got
        .iter()
        .filter(|p| p.channel == Channel::ReliableOrdered)
        .map(|p| p.data[0])
        .collect();
    let unrel = got
        .iter()
        .filter(|p| p.channel == Channel::Unreliable)
        .count();
    assert_eq!(rel, (0..100u8).collect::<Vec<_>>());
    assert!(unrel > 10 && unrel < 90, "unreliable {unrel}");
}

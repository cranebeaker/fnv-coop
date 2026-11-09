//! Scenario set run against the backends that need no external service:
//! the in-process MemBackend (manual clock) and the UDP backend on localhost.

use coop_core::ids::PlayerId;
use coop_core::proto::Channel;
use coop_core::transport::{Transport, TransportError};
use coop_net::mem::{MemBackend, MemNet};
use coop_net::{
    random_token, Backend, Clock, Endpoint, EndpointConfig, LeaveReason, ManualClock, NetEvent,
    RejectCode, UdpBackend, UdpConfig,
};

fn cfg(clock: &Clock, name: &str) -> EndpointConfig {
    EndpointConfig {
        name: name.into(),
        clock: clock.clone(),
        ..Default::default()
    }
}

fn mem_pair() -> (
    ManualClock,
    MemNet,
    Endpoint<MemBackend>,
    Endpoint<MemBackend>,
) {
    let mc = ManualClock::new();
    let clock = Clock::Manual(mc.clone());
    let net = MemNet::new();
    let tok = [7u8; 16];
    let host = Endpoint::host(net.host(), tok, cfg(&clock, "h"));
    let guest = Endpoint::guest(net.guest(), tok, cfg(&clock, "g"));
    (mc, net, host, guest)
}

fn settle<B: Backend, C: Backend>(a: &mut Endpoint<B>, b: &mut Endpoint<C>) {
    for _ in 0..10 {
        a.pump();
        b.pump();
    }
}

#[test]
fn handshake_and_all_channels() {
    let (_mc, _net, mut host, mut guest) = mem_pair();
    settle(&mut host, &mut guest);
    assert!(guest.is_connected());
    assert_eq!(guest.local(), PlayerId(1));
    assert_eq!(
        host.events(),
        vec![NetEvent::PeerJoined {
            player: PlayerId(1),
            name: "g".into()
        }]
    );
    assert_eq!(
        guest.events(),
        vec![NetEvent::Connected { local: PlayerId(1) }]
    );

    host.send(PlayerId(1), Channel::ReliableOrdered, b"r1".to_vec())
        .unwrap();
    host.send(PlayerId(1), Channel::Unreliable, b"u1".to_vec())
        .unwrap();
    guest
        .send(PlayerId(0), Channel::ReliableOrdered, b"r2".to_vec())
        .unwrap();
    settle(&mut host, &mut guest);
    let mut got: Vec<_> = std::iter::from_fn(|| guest.recv())
        .map(|p| (p.from, p.channel, p.data))
        .collect();
    got.sort_by_key(|x| x.2.clone());
    assert_eq!(
        got,
        vec![
            (PlayerId(0), Channel::ReliableOrdered, b"r1".to_vec()),
            (PlayerId(0), Channel::Unreliable, b"u1".to_vec())
        ]
    );
    let p = host.recv().unwrap();
    assert_eq!((p.from, p.data), (PlayerId(1), b"r2".to_vec()));
}

#[test]
fn big_message_is_fragmented_and_reassembled() {
    let (_mc, _net, mut host, mut guest) = mem_pair();
    settle(&mut host, &mut guest);
    let blob: Vec<u8> = (0..5_000_000u32).map(|i| (i % 251) as u8).collect();
    host.send(PlayerId(1), Channel::ReliableOrdered, blob.clone())
        .unwrap();
    host.send(PlayerId(1), Channel::ReliableOrdered, b"after".to_vec())
        .unwrap();
    settle(&mut host, &mut guest);
    assert_eq!(guest.recv().unwrap().data, blob);
    assert_eq!(guest.recv().unwrap().data, b"after");
}

#[test]
fn size_limits() {
    let (_mc, _net, mut host, mut guest) = mem_pair();
    settle(&mut host, &mut guest);
    assert!(host
        .send(PlayerId(1), Channel::Unreliable, vec![0; 1100])
        .is_ok());
    assert!(matches!(
        host.send(PlayerId(1), Channel::Unreliable, vec![0; 1101]),
        Err(TransportError::TooLarge { max: 1100, .. })
    ));
    assert_eq!(
        host.send(PlayerId(9), Channel::Unreliable, vec![]),
        Err(TransportError::UnknownPeer(PlayerId(9)))
    );
    // Guests can only reach the host.
    assert_eq!(
        guest.send(PlayerId(2), Channel::Unreliable, vec![]),
        Err(TransportError::UnknownPeer(PlayerId(2)))
    );
}

#[test]
fn unreliable_arrives_in_sequence() {
    let (_mc, _net, mut host, mut guest) = mem_pair();
    settle(&mut host, &mut guest);
    for i in 0..5u8 {
        host.send(PlayerId(1), Channel::Unreliable, vec![i])
            .unwrap();
    }
    settle(&mut host, &mut guest);
    let v: Vec<u8> = std::iter::from_fn(|| guest.recv())
        .map(|p| p.data[0])
        .collect();
    assert_eq!(v, vec![0, 1, 2, 3, 4]);
}

#[test]
fn silent_peer_times_out_after_20s_and_keepalives_prevent_it() {
    let (mc, net, mut host, mut guest) = mem_pair();
    settle(&mut host, &mut guest);
    host.events();
    guest.events();
    // Healthy: 60 s of keep-alives.
    for _ in 0..60 {
        mc.advance(1000);
        settle(&mut host, &mut guest);
    }
    assert!(host.events().is_empty());
    assert!(guest.is_connected());
    // Guest goes silent.
    net.silence(guest.backend().id(), true);
    for _ in 0..19 {
        mc.advance(1000);
        settle(&mut host, &mut guest);
    }
    assert!(host.events().is_empty());
    mc.advance(2000);
    settle(&mut host, &mut guest);
    assert_eq!(
        host.events(),
        vec![NetEvent::PeerLeft {
            player: PlayerId(1),
            reason: LeaveReason::Timeout
        }]
    );
    assert!(host.players().is_empty());
}

#[test]
fn goodbye_and_kick() {
    let (_mc, _net, mut host, mut guest) = mem_pair();
    settle(&mut host, &mut guest);
    host.events();
    guest.leave();
    settle(&mut host, &mut guest);
    assert_eq!(
        host.events(),
        vec![NetEvent::PeerLeft {
            player: PlayerId(1),
            reason: LeaveReason::Goodbye
        }]
    );

    let (_mc, _net, mut host, mut guest) = mem_pair();
    settle(&mut host, &mut guest);
    host.events();
    guest.events();
    host.kick(PlayerId(1), "afk");
    settle(&mut host, &mut guest);
    assert_eq!(
        guest.events(),
        vec![NetEvent::PeerLeft {
            player: PlayerId(0),
            reason: LeaveReason::Kicked("afk".into())
        }]
    );
    assert!(!guest.is_connected());
}

#[test]
fn bad_token_full_and_rejoin() {
    let mc = ManualClock::new();
    let clock = Clock::Manual(mc.clone());
    let net = MemNet::new();
    let mut host = Endpoint::host(
        net.host(),
        [1; 16],
        EndpointConfig {
            max_guests: 1,
            ..cfg(&clock, "h")
        },
    );
    let mut bad = Endpoint::guest(net.guest(), [2; 16], cfg(&clock, "bad"));
    settle(&mut host, &mut bad);
    assert!(bad
        .events()
        .contains(&NetEvent::Rejected(RejectCode::BadToken)));
    let mut a = Endpoint::guest(net.guest(), [1; 16], cfg(&clock, "a"));
    let mut b = Endpoint::guest(net.guest(), [1; 16], cfg(&clock, "b"));
    settle(&mut host, &mut a);
    settle(&mut host, &mut b);
    assert!(a.is_connected());
    assert!(b.events().contains(&NetEvent::Rejected(RejectCode::Full)));
    // Guest a gets kicked, rejoins and gets a slot again.
    host.kick(PlayerId(1), "x");
    settle(&mut host, &mut a);
    a.rejoin();
    settle(&mut host, &mut a);
    assert!(a.is_connected());
}

#[test]
fn lost_welcome_is_recovered_by_repeated_join() {
    let (mc, net, mut host, mut guest) = mem_pair();
    net.silence(0, true); // host's Welcome vanishes
    settle(&mut host, &mut guest);
    assert!(!guest.is_connected());
    net.silence(0, false);
    mc.advance(1100);
    settle(&mut host, &mut guest);
    assert!(guest.is_connected());
    assert_eq!(host.players(), vec![PlayerId(1)]);
}

#[test]
fn guest_join_fails_when_host_never_answers() {
    let mc = ManualClock::new();
    let clock = Clock::Manual(mc.clone());
    let net = MemNet::new();
    let _host_backend = net.host(); // exists but nobody pumps an endpoint
    let mut g = Endpoint::guest(net.guest(), [1; 16], cfg(&clock, "g"));
    for _ in 0..70 {
        mc.advance(1000);
        g.pump();
    }
    assert!(g
        .events()
        .iter()
        .any(|e| matches!(e, NetEvent::JoinFailed(_))));
}

#[test]
fn udp_backend_passes_the_same_basics() {
    let tok = random_token();
    let clock = Clock::system();
    let ucfg = UdpConfig {
        min_rto_ms: 10,
        ..Default::default()
    };
    let hb = UdpBackend::host("127.0.0.1:0".parse().unwrap(), tok, ucfg.clone()).unwrap();
    let addr = hb.local_addr().unwrap();
    let mut host = Endpoint::host(hb, tok, cfg(&clock, "h"));
    let mut guest = Endpoint::guest(
        UdpBackend::guest(addr, tok, ucfg).unwrap(),
        tok,
        cfg(&clock, "g"),
    );
    let t0 = std::time::Instant::now();
    while !guest.is_connected() && t0.elapsed().as_secs() < 5 {
        host.pump();
        guest.pump();
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
    assert!(guest.is_connected());
    let blob = vec![0xABu8; 300_000];
    host.send(PlayerId(1), Channel::ReliableOrdered, blob.clone())
        .unwrap();
    guest
        .send(PlayerId(0), Channel::ReliableOrdered, b"ping".to_vec())
        .unwrap();
    host.send(PlayerId(1), Channel::Unreliable, b"pos".to_vec())
        .unwrap();
    let (mut got_blob, mut got_ping, mut got_pos) = (false, false, false);
    while t0.elapsed().as_secs() < 10 && !(got_blob && got_ping && got_pos) {
        host.pump();
        guest.pump();
        while let Some(p) = guest.recv() {
            match p.channel {
                Channel::ReliableOrdered => {
                    assert_eq!(p.data, blob);
                    got_blob = true;
                }
                Channel::Unreliable => got_pos = true,
            }
        }
        if let Some(p) = host.recv() {
            assert_eq!(p.data, b"ping");
            got_ping = true;
        }
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
    assert!(
        got_blob && got_ping && got_pos,
        "blob {got_blob} ping {got_ping} pos {got_pos}"
    );
    assert!(host.rtt_ms(PlayerId(1)).is_some());
    // Clean leave reaches the host over UDP.
    guest.leave();
    let t1 = std::time::Instant::now();
    let mut left = false;
    while t1.elapsed().as_secs() < 5 && !left {
        guest.pump();
        left = host.events().iter().any(|e| {
            matches!(
                e,
                NetEvent::PeerLeft {
                    reason: LeaveReason::Goodbye,
                    ..
                }
            )
        });
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
    assert!(left, "goodbye did not reach the host");
}

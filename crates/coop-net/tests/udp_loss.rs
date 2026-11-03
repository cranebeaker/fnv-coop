//! Reliable-ordered delivery over UDP with seeded loss, duplication and
//! reordering injected at the socket layer.

use coop_core::ids::PlayerId;
use coop_core::proto::Channel;
use coop_core::transport::Transport;
use coop_net::{ChaosCfg, Clock, Endpoint, EndpointConfig, UdpBackend, UdpConfig};

fn message(i: u32) -> Vec<u8> {
    // Mixed sizes: tiny, one segment, several segments, and a few big ones.
    let len = match i % 25 {
        0 => 150_000,
        1..=5 => 3_000,
        6..=10 => 1_000,
        _ => 20 + (i as usize % 400),
    };
    let mut v = Vec::with_capacity(len + 4);
    v.extend_from_slice(&i.to_le_bytes());
    v.extend((0..len).map(|k| (k as u32).wrapping_mul(31).wrapping_add(i) as u8));
    v
}

fn run(seed: u64, drop: f64, dup: f64, reorder: f64, count: u32) -> (u64, u64) {
    let tok = [5u8; 16];
    let clock = Clock::system();
    let mk = |s: u64| UdpConfig {
        chaos: Some(ChaosCfg {
            seed: s,
            drop,
            dup,
            reorder,
        }),
        min_rto_ms: 15,
        ..Default::default()
    };
    let ec = |n: &str| EndpointConfig {
        name: n.into(),
        clock: clock.clone(),
        ..Default::default()
    };
    let hb = UdpBackend::host("127.0.0.1:0".parse().unwrap(), tok, mk(seed)).unwrap();
    let addr = hb.local_addr().unwrap();
    let mut host = Endpoint::host(hb, tok, ec("h"));
    let mut guest = Endpoint::guest(
        UdpBackend::guest(addr, tok, mk(seed ^ 0xFF)).unwrap(),
        tok,
        ec("g"),
    );

    let t0 = std::time::Instant::now();
    while !guest.is_connected() {
        assert!(
            t0.elapsed().as_secs() < 30,
            "handshake under loss timed out"
        );
        host.pump();
        guest.pump();
        std::thread::sleep(std::time::Duration::from_millis(1));
    }

    let (mut sent_h, mut sent_g) = (0u32, 0u32);
    let (mut next_h, mut next_g) = (0u32, 0u32); // next expected index at guest / host
    while (next_h < count || next_g < count) && t0.elapsed().as_secs() < 120 {
        for _ in 0..4 {
            if sent_h < count {
                host.send(PlayerId(1), Channel::ReliableOrdered, message(sent_h))
                    .unwrap();
                sent_h += 1;
            }
            if sent_g < count {
                guest
                    .send(
                        PlayerId(0),
                        Channel::ReliableOrdered,
                        message(sent_g + 1_000_000),
                    )
                    .unwrap();
                sent_g += 1;
            }
        }
        host.pump();
        guest.pump();
        while let Some(p) = guest.recv() {
            assert_eq!(
                p.data,
                message(next_h),
                "guest got message out of order or damaged at {next_h}"
            );
            next_h += 1;
        }
        while let Some(p) = host.recv() {
            assert_eq!(
                p.data,
                message(next_g + 1_000_000),
                "host got message out of order or damaged at {next_g}"
            );
            next_g += 1;
        }
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
    assert_eq!((next_h, next_g), (count, count), "not everything arrived");
    assert_eq!(host.frame_errors + guest.frame_errors, 0);
    let s = guest.backend().stats;
    let h = host.backend().stats;
    (s.dropped + h.dropped, s.retransmits + h.retransmits)
}

#[test]
fn twenty_percent_loss_reliable_ordered() {
    let (dropped, retrans) = run(1234, 0.20, 0.0, 0.0, 200);
    assert!(
        dropped > 50,
        "chaos should really drop packets, dropped {dropped}"
    );
    assert!(retrans > 0);
}

#[test]
fn loss_duplication_and_reordering_together() {
    let (dropped, _) = run(99, 0.20, 0.10, 0.15, 150);
    assert!(dropped > 20);
}

#[test]
fn clean_link_drops_nothing() {
    let (dropped, _) = run(1, 0.0, 0.0, 0.0, 100);
    assert_eq!(dropped, 0);
}

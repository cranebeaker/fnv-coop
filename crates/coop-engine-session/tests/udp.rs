//! The host and guest drivers over coop-net's real UDP endpoint on 127.0.0.1, as nv-viewer opens
//! it (viewer/src/coop.rs): join by token, the guest's slot from the endpoint's `Connected`, the
//! story replicated, a wrong token refused.

use std::net::SocketAddr;
use std::time::{Duration, Instant};

use coop_core::sync::HostConfig;
use coop_engine::quest;
use coop_engine::testkit::{Story, StorySpec};
use coop_engine_session::{echo, hello, record, GuestDriver, HostDriver};
use coop_net::{Endpoint, EndpointConfig, NetEvent, UdpBackend, UdpConfig};

engine_core::assemble!();

const MQ: u32 = 0x0010_4c0c;

fn spec() -> StorySpec {
    StorySpec {
        quests: vec![(MQ, vec![5, 10, 20])],
        globals: vec![(0x39, 0.0)],
        ..StorySpec::default()
    }
}

fn plugins() -> Vec<String> {
    vec!["FalloutNV.esm".into()]
}

/// Runs both sides for up to `secs`, until `done` says so.
fn run_until(
    host: &mut (Story, HostDriver, Endpoint<UdpBackend>),
    guest: &mut (Story, Option<GuestDriver>, Endpoint<UdpBackend>),
    secs: u64,
    mut done: impl FnMut(&mut Story, &Option<GuestDriver>) -> bool,
) -> Vec<NetEvent> {
    let start = Instant::now();
    let mut events = Vec::new();
    while start.elapsed() < Duration::from_secs(secs) {
        let now = start.elapsed().as_millis() as u64;
        host.2.pump();
        host.1.step(&mut host.0.e, &mut host.2, now);
        guest.2.pump();
        for ev in guest.2.events() {
            if let NetEvent::Connected { local } = ev {
                guest.1 = Some(GuestDriver::new(local, hello(plugins())));
            }
            events.push(ev);
        }
        if let Some(g) = guest.1.as_mut() {
            g.step(&mut guest.0.e, &mut guest.2, now);
        }
        if done(&mut guest.0, &guest.1) {
            break;
        }
        std::thread::sleep(Duration::from_millis(5));
    }
    events
}

fn host_on(token: [u8; 16]) -> ((Story, HostDriver, Endpoint<UdpBackend>), SocketAddr) {
    let b = UdpBackend::host(
        SocketAddr::from(([127, 0, 0, 1], 0)),
        token,
        UdpConfig::default(),
    )
    .expect("bind a UDP port on 127.0.0.1");
    let addr = b.local_addr().unwrap();
    let mut story = Story::new(&spec());
    let mut hd = HostDriver::new(HostConfig::new(hello(plugins()), 9), [MQ], [0x39]);
    hd.start(&mut story.e);
    let ep = Endpoint::host(b, token, EndpointConfig::default());
    ((story, hd, ep), addr)
}

fn guest_to(
    addr: SocketAddr,
    token: [u8; 16],
) -> (Story, Option<GuestDriver>, Endpoint<UdpBackend>) {
    let b = UdpBackend::guest(addr, token, UdpConfig::default()).unwrap();
    (
        Story::new(&spec()),
        None,
        Endpoint::guest(b, token, EndpointConfig::default()),
    )
}

#[test]
fn the_story_reaches_a_guest_over_udp() {
    let token = [5; 16];
    let (mut host, addr) = host_on(token);
    let mut guest = guest_to(addr, token);
    assert!(host.0.set_stage(MQ, 10));
    host.0.set_global(0x39, 2.0);
    let want = record::encode(&quest::record(&mut host.0.e, MQ).unwrap());
    assert!(want.is_some());
    run_until(&mut host, &mut guest, 20, |g, d| {
        d.as_ref().is_some_and(|d| d.sync.hash_checks > 0)
            && record::encode(&quest::record(&mut g.e, MQ).unwrap()) == want
            && coop_engine::forms::global_value(&mut g.e, 0x39) == Some(2.0)
    });
    let d = guest.1.as_ref().expect("the host accepted the guest");
    assert_eq!(
        record::encode(&quest::record(&mut guest.0.e, MQ).unwrap()),
        want
    );
    assert_eq!(
        coop_engine::forms::global_value(&mut guest.0.e, 0x39),
        Some(2.0)
    );
    assert!(echo(&mut guest.0.e, &d.sync.store).is_empty());
    assert_eq!(d.sync.desyncs_detected, 0);
    assert_eq!(guest.0.scripts_run(), 0);
}

#[test]
fn a_guest_with_the_wrong_token_gets_no_session() {
    let (mut host, addr) = host_on([5; 16]);
    let mut guest = guest_to(addr, [6; 16]);
    // A wrong token is dropped by the host's UDP layer without an answer (the guest gives up after
    // its own 20 s); what matters is that no session forms on either side.
    run_until(&mut host, &mut guest, 3, |_, _| false);
    assert!(guest.1.is_none(), "no session for a wrong token");
    assert!(host.2.players().is_empty(), "no guest connected");
    assert!(host.1.sync.players().is_empty());
}

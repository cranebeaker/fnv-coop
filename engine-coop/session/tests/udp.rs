//! The host and guest drivers over coop-net's real UDP endpoint on 127.0.0.1, as `nv-coop` opens
//! it: join by token, the guest's slot from the endpoint's `Connected`, the story replicated, a
//! wrong token refused. Each game is a fake program: no engine and no game files.

use std::net::SocketAddr;
use std::time::{Duration, Instant};

use coop_core::ids::FormId;
use coop_core::sync::HostConfig;
use coop_engine::fake::FakeProgram;
use coop_engine::game::{EngineCoop, EngineState, NameTable};
use coop_engine::{addr, Program};
use coop_engine_session::{hello, GuestDriver, HostDriver};
use coop_game::Shared;
use coop_net::{Endpoint, EndpointConfig, NetEvent, UdpBackend, UdpConfig};

const MQ: u32 = 0x0010_4c0c;
const STORY: u32 = 0x0010_0001;

fn names() -> NameTable {
    NameTable::parse("P 00 FalloutNV.esm\nF QUST 00104C0C MQ\nF GLOB 00100001 StoryGlobal\n")
        .unwrap()
}

fn game() -> FakeProgram {
    let mut p = FakeProgram::new();
    p.add_quest(MQ, &[5, 10, 20], &[10]);
    p.add_global(STORY, 0.0);
    p
}

type Host = (FakeProgram, HostDriver, Endpoint<UdpBackend>);
type Guest = (FakeProgram, Option<GuestDriver>, Endpoint<UdpBackend>);

fn host_on(token: [u8; 16]) -> (Host, SocketAddr) {
    let b = UdpBackend::host(
        SocketAddr::from(([127, 0, 0, 1], 0)),
        token,
        UdpConfig::default(),
    )
    .expect("bind a UDP port on 127.0.0.1");
    let addr = b.local_addr().unwrap();
    let cfg = HostConfig::new(hello(names().plugin_files()), 9);
    let driver = HostDriver::new(cfg, EngineState::new(names()), None);
    let ep = Endpoint::host(b, token, EndpointConfig::default());
    ((game(), driver, ep), addr)
}

fn guest_to(addr: SocketAddr, token: [u8; 16]) -> Guest {
    let b = UdpBackend::guest(addr, token, UdpConfig::default()).unwrap();
    (
        game(),
        None,
        Endpoint::guest(b, token, EndpointConfig::default()),
    )
}

/// Runs both sides for up to `secs`, until `done` says so.
fn run_until(
    host: &mut Host,
    guest: &mut Guest,
    secs: u64,
    mut done: impl FnMut(&mut FakeProgram, &mut Option<GuestDriver>) -> bool,
) -> Vec<NetEvent> {
    let start = Instant::now();
    let mut events = Vec::new();
    while start.elapsed() < Duration::from_secs(secs) {
        let now = start.elapsed().as_millis() as u64;
        host.2.pump();
        host.1.step(&mut host.0, &mut host.2, now).unwrap();
        guest.2.pump();
        for ev in guest.2.events() {
            if let NetEvent::Connected { local } = ev {
                guest.1 = Some(GuestDriver::new(
                    local,
                    hello(names().plugin_files()),
                    EngineState::new(names()),
                    None,
                ));
            }
            events.push(ev);
        }
        if let Some(g) = guest.1.as_mut() {
            g.step(&mut guest.0, &mut guest.2, now).unwrap();
        }
        if done(&mut guest.0, &mut guest.1) {
            break;
        }
        std::thread::sleep(Duration::from_millis(5));
    }
    events
}

fn stage_and_global(p: &mut FakeProgram, state: &mut EngineState) -> (Option<u16>, f32) {
    let stage = EngineCoop::over(p, state).stage(FormId(MQ));
    let g = p.call(addr::FORM_BY_ID, &[STORY]).eax;
    (stage, p.f32(g + addr::GLOBAL_VALUE))
}

#[test]
fn the_story_reaches_a_guest_over_udp() {
    let token = [5; 16];
    let (mut host, addr) = host_on(token);
    let mut guest = guest_to(addr, token);
    let q = host.0.call(addr::FORM_BY_ID, &[MQ]).eax;
    assert!(host.0.call(addr::QUEST_SET_STAGE, &[q, 10]).bool());
    let g = host.0.call(addr::FORM_BY_ID, &[STORY]).eax;
    host.0.set_f32(g + addr::GLOBAL_VALUE, 2.0);
    run_until(&mut host, &mut guest, 20, |p, d| match d {
        Some(d) => d.sync.hash_checks > 0 && stage_and_global(p, &mut d.game) == (Some(10), 2.0),
        None => false,
    });
    let d = guest.1.as_mut().expect("the host accepted the guest");
    assert_eq!(stage_and_global(&mut guest.0, &mut d.game), (Some(10), 2.0));
    assert_eq!(d.sync.desyncs_detected, 0);
    assert!(guest.0.stages_set.is_empty(), "no stage function ran");
    assert_eq!(guest.0.gates.len(), 1);
}

#[test]
fn a_guest_with_the_wrong_token_gets_no_session() {
    let (mut host, addr) = host_on([5; 16]);
    let mut guest = guest_to(addr, [6; 16]);
    // A wrong token is dropped by the host's UDP layer without an answer; what matters is that
    // no session forms on either side.
    run_until(&mut host, &mut guest, 3, |_, _| false);
    assert!(guest.1.is_none(), "no session for a wrong token");
    assert!(host.2.players().is_empty(), "no guest connected");
    assert!(host.1.sync.players().is_empty());
}

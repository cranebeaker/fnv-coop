//! Checks over the network and launcher crates: UDP transport, join codes, the Steam
//! mock, the wire format, the launcher command lines and install detection.
//!
//! Every check takes `mutate`: with it set the check sabotages the thing under test and must
//! then fail (the mutation twin of the row).

use std::collections::HashMap;
use std::io::ErrorKind;
use std::net::{SocketAddr, UdpSocket};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use coop_core::ids::PlayerId;
use coop_core::proto::Channel;
use coop_net::steam::api::{SEND_NO_NAGLE, SEND_RELIABLE};
use coop_net::steam::{
    Invite, MockUniverse, SteamApi, SteamBackend, SteamOptions, SteamPhase, MARKER_KEY,
};
use coop_net::{
    random_token, Backend, ChaosCfg, Clock, Endpoint, EndpointConfig, JoinCode, JoinError,
    JoinTarget, ManualClock, NetEvent, Token, Transport, UdpBackend, UdpConfig,
};
use nv_launcher::settings::Settings;
use nv_launcher::{cli, command, diag, install, joincode, party, portcheck, redact};

use super::builtin::{Check, Env};

/// The checks of this module by name.
pub fn registry() -> Vec<(&'static str, Check)> {
    vec![
        ("net_udp_handshake", net_udp_handshake),
        ("net_udp_reliable_loss", net_udp_reliable_loss),
        ("net_udp_keepalive", net_udp_keepalive),
        ("net_join_codes", net_join_codes),
        ("net_steam_mock", net_steam_mock),
        ("net_steam_mock_friends", net_steam_mock_friends),
        ("net_wire_plaintext", net_wire_plaintext),
        ("net_join_service_codes", net_join_service_codes),
        ("launcher_command_lines", launcher_command_lines),
        ("launcher_port_test", launcher_port_test),
        ("launcher_log_redaction", launcher_log_redaction),
        ("launcher_install_detect", launcher_install_detect),
    ]
}

// ---------------------------------------------------------------- shared helpers

/// Bind address of every UDP socket here: loopback, the OS picks the port.
const LOCAL: &str = "127.0.0.1:0";
/// Wall-clock cap of one UDP loop (the loops run on a manual clock and end far sooner).
const REAL_BUDGET: Duration = Duration::from_secs(4);

fn ecfg(clock: &Clock, name: &str) -> EndpointConfig {
    EndpointConfig {
        name: name.into(),
        clock: clock.clone(),
        ..Default::default()
    }
}

fn udp_host(
    token: Token,
    cfg: UdpConfig,
    clock: &Clock,
) -> Result<(Endpoint<UdpBackend>, SocketAddr), String> {
    let b = UdpBackend::host(LOCAL.parse().unwrap(), token, cfg)
        .map_err(|e| format!("the UDP host could not bind 127.0.0.1: {e}"))?;
    let addr = b
        .local_addr()
        .map_err(|e| format!("the UDP host has no address: {e}"))?;
    Ok((Endpoint::host(b, token, ecfg(clock, "host")), addr))
}

fn udp_guest(
    host: SocketAddr,
    token: Token,
    cfg: UdpConfig,
    clock: &Clock,
    name: &str,
) -> Result<Endpoint<UdpBackend>, String> {
    let b = UdpBackend::guest(host, token, cfg)
        .map_err(|e| format!("the UDP guest could not bind: {e}"))?;
    Ok(Endpoint::guest(b, token, ecfg(clock, name)))
}

/// Moves the manual clock and pumps every endpoint once.
fn step(mc: &ManualClock, ms: u64, eps: &mut [&mut Endpoint<UdpBackend>]) {
    mc.advance(ms);
    for e in eps.iter_mut() {
        e.pump();
    }
}

/// A token that differs from `t` in one byte.
fn other_token(t: Token, k: u8) -> Token {
    let mut o = t;
    o[0] ^= k;
    o
}

fn contains(hay: &[u8], needle: &[u8]) -> bool {
    !needle.is_empty() && hay.windows(needle.len()).any(|w| w == needle)
}

fn sv(a: &[&str]) -> Vec<String> {
    a.iter().map(|s| s.to_string()).collect()
}

fn unique() -> String {
    static N: AtomicU64 = AtomicU64::new(0);
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.subsec_nanos())
        .unwrap_or(0);
    format!(
        "{}-{nanos}-{}",
        std::process::id(),
        N.fetch_add(1, Ordering::Relaxed)
    )
}

/// A scratch folder under the temp folder (`std::env::temp_dir`). Drop removes this folder
/// only.
struct Scratch(PathBuf);

impl Scratch {
    fn new(tag: &str) -> Result<Scratch, String> {
        let p = std::env::temp_dir().join(format!("coop-checklist-{tag}-{}", unique()));
        std::fs::create_dir_all(&p).map_err(|e| format!("could not make a scratch folder: {e}"))?;
        Ok(Scratch(p))
    }
    fn path(&self) -> &Path {
        &self.0
    }
    /// Text with the scratch path replaced, so a report never shows the temp folder.
    fn hide(&self, s: &str) -> String {
        s.replace(&*self.0.to_string_lossy(), "<scratch>")
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

// ---------------------------------------------------------------- core.udp_transport

/// Two guests join a UDP host with the session token and get slots 1 and 2. A guest with a
/// wrong token stays out. Mutation: the two guests present a wrong token.
fn net_udp_handshake(_env: &Env, mutate: bool) -> Result<String, String> {
    let mc = ManualClock::new();
    let clock = Clock::Manual(mc.clone());
    let token = random_token();
    let cfg = UdpConfig {
        min_rto_ms: 20,
        ..Default::default()
    };
    let (mut host, addr) = udp_host(token, cfg.clone(), &clock)?;
    let guest_token = if mutate {
        other_token(token, 0xFF)
    } else {
        token
    };
    let mut ann = udp_guest(addr, guest_token, cfg.clone(), &clock, "ann")?;
    let mut bo = udp_guest(addr, guest_token, cfg.clone(), &clock, "bo")?;
    // An intruder with a wrong token repeats its open every 200 ms for the whole run.
    let mut eve = udp_guest(addr, other_token(token, 0x5A), cfg, &clock, "eve")?;
    let mut host_events = Vec::new();
    let t0 = Instant::now();
    while mc.now() < 3_000 && t0.elapsed() < REAL_BUDGET {
        step(&mc, 10, &mut [&mut host, &mut ann, &mut bo, &mut eve]);
        host_events.extend(host.events());
    }
    for (g, n) in [(&ann, "ann"), (&bo, "bo")] {
        if !g.is_connected() {
            return Err(format!(
                "guest {n} did not join the UDP host on 127.0.0.1 in 3 s"
            ));
        }
    }
    let mut slots = [ann.local(), bo.local()];
    slots.sort();
    if slots != [PlayerId(1), PlayerId(2)] {
        return Err(format!("the guests got slots {slots:?}, not 1 and 2"));
    }
    if host.players() != vec![PlayerId(1), PlayerId(2)] {
        return Err(format!("the host lists players {:?}", host.players()));
    }
    for (g, n) in [(&ann, "ann"), (&bo, "bo")] {
        let joined = NetEvent::PeerJoined {
            player: g.local(),
            name: n.into(),
        };
        if !host_events.contains(&joined) {
            return Err(format!("the host saw no join of {n}"));
        }
    }
    if !ann
        .events()
        .contains(&NetEvent::Connected { local: ann.local() })
    {
        return Err("guest ann saw no Connected event".into());
    }
    if eve.is_connected()
        || host_events
            .iter()
            .any(|e| matches!(e, NetEvent::PeerJoined { name, .. } if name == "eve"))
    {
        return Err("a guest with a wrong token got in".into());
    }
    // Each slot reaches its own guest.
    for g in [&ann, &bo] {
        let p = g.local();
        host.send(p, Channel::ReliableOrdered, vec![p.0])
            .map_err(|e| format!("host send to slot {}: {e}", p.0))?;
    }
    let (mut got_ann, mut got_bo) = (None, None);
    let end = mc.now() + 2_000;
    while (got_ann.is_none() || got_bo.is_none()) && mc.now() < end {
        step(&mc, 10, &mut [&mut host, &mut ann, &mut bo]);
        if let Some(p) = ann.recv() {
            got_ann = Some(p.data);
        }
        if let Some(p) = bo.recv() {
            got_bo = Some(p.data);
        }
    }
    if got_ann != Some(vec![ann.local().0]) || got_bo != Some(vec![bo.local().0]) {
        return Err(format!(
            "the per-slot messages went astray: ann got {got_ann:?}, bo got {got_bo:?}"
        ));
    }
    Ok("2 guests joined a UDP host on 127.0.0.1 with the session token (slots 1 and 2, each got the message sent to its slot), a guest with a wrong token stayed out".into())
}

const LOSS_MESSAGES: u32 = 200;

/// Message `i` of one side: index, side, then filler (1 or 2 UDP segments, at most 1100 bytes
/// so the unreliable lane of the mutation can carry it too).
fn loss_msg(i: u32, from_host: bool) -> Vec<u8> {
    let filler = [12usize, 300, 995, 1095, 60, 700][(i % 6) as usize];
    let mut v = Vec::with_capacity(filler + 5);
    v.extend_from_slice(&i.to_le_bytes());
    v.push(from_host as u8);
    v.extend((0..filler).map(|k| (k as u32).wrapping_mul(31).wrapping_add(i) as u8));
    v
}

/// 200 reliable messages each way arrive in order over UDP with seeded loss, duplication and
/// reordering on both sockets. Mutation: the messages go on the unreliable lane, so nothing
/// repairs the loss.
fn net_udp_reliable_loss(env: &Env, mutate: bool) -> Result<String, String> {
    let mc = ManualClock::new();
    let clock = Clock::Manual(mc.clone());
    let token = random_token();
    let chaos = |seed: u64| UdpConfig {
        chaos: Some(ChaosCfg {
            seed,
            drop: 0.20,
            dup: 0.05,
            reorder: 0.10,
        }),
        min_rto_ms: 20,
        ..Default::default()
    };
    let (mut host, addr) = udp_host(token, chaos(env.seed), &clock)?;
    let mut guest = udp_guest(addr, token, chaos(env.seed ^ 0x5EED_F00D), &clock, "guest")?;
    let t0 = Instant::now();
    while !guest.is_connected() {
        if mc.now() > 30_000 || t0.elapsed() > REAL_BUDGET {
            return Err("no UDP handshake under 20% loss in 30 s".into());
        }
        step(&mc, 10, &mut [&mut host, &mut guest]);
    }
    let lane = if mutate {
        Channel::Unreliable
    } else {
        Channel::ReliableOrdered
    };
    let n = LOSS_MESSAGES;
    let (mut sent_h, mut sent_g, mut next_h, mut next_g) = (0u32, 0u32, 0u32, 0u32);
    while (next_h < n || next_g < n) && mc.now() < 300_000 && t0.elapsed() < REAL_BUDGET {
        for _ in 0..4 {
            if sent_h < n {
                host.send(PlayerId(1), lane, loss_msg(sent_h, true))
                    .map_err(|e| format!("host send: {e}"))?;
                sent_h += 1;
            }
            if sent_g < n {
                guest
                    .send(PlayerId(0), lane, loss_msg(sent_g, false))
                    .map_err(|e| format!("guest send: {e}"))?;
                sent_g += 1;
            }
        }
        step(&mc, 5, &mut [&mut host, &mut guest]);
        while let Some(p) = guest.recv() {
            if p.data != loss_msg(next_h, true) {
                return Err(format!(
                    "the guest got a message out of order or damaged where host message {next_h} was due"
                ));
            }
            next_h += 1;
        }
        while let Some(p) = host.recv() {
            if p.data != loss_msg(next_g, false) {
                return Err(format!(
                    "the host got a message out of order or damaged where guest message {next_g} was due"
                ));
            }
            next_g += 1;
        }
    }
    if (next_h, next_g) != (n, n) {
        return Err(format!(
            "only {next_h} of {n} host messages and {next_g} of {n} guest messages arrived"
        ));
    }
    if host.frame_errors + guest.frame_errors != 0 {
        return Err("framing errors on a reliable link".into());
    }
    let (h, g) = (host.backend().stats, guest.backend().stats);
    let (dropped, retrans) = (h.dropped + g.dropped, h.retransmits + g.retransmits);
    if dropped == 0 || retrans == 0 {
        return Err(format!(
            "the link lost {dropped} datagrams and retransmitted {retrans}: no loss was repaired"
        ));
    }
    Ok(format!(
        "{n} reliable messages each way (1 and 2 segments) arrived in order over UDP 127.0.0.1 with 20% loss, 5% duplication and 10% reordering ({dropped} datagrams dropped, {retrans} segments retransmitted)"
    ))
}

/// An idle UDP link stays open past the 20 s silence limit on keep-alives alone. Mutation:
/// the guest stops running, so it sends no keep-alive.
fn net_udp_keepalive(_env: &Env, mutate: bool) -> Result<String, String> {
    let mc = ManualClock::new();
    let clock = Clock::Manual(mc.clone());
    let token = random_token();
    let cfg = UdpConfig {
        min_rto_ms: 20,
        ..Default::default()
    };
    let (mut host, addr) = udp_host(token, cfg.clone(), &clock)?;
    let mut guest = udp_guest(addr, token, cfg, &clock, "guest")?;
    let t0 = Instant::now();
    while !guest.is_connected() {
        if mc.now() > 3_000 || t0.elapsed() > REAL_BUDGET {
            return Err("no UDP handshake in 3 s".into());
        }
        step(&mc, 10, &mut [&mut host, &mut guest]);
    }
    host.events();
    guest.events();
    let rx0 = host.backend().stats.received;
    let mut left = Vec::new();
    // 30 s of idle time on the manual clock, no game data.
    for _ in 0..300 {
        mc.advance(100);
        host.pump();
        if !mutate {
            guest.pump();
        }
        left.extend(
            host.events()
                .into_iter()
                .filter(|e| matches!(e, NetEvent::PeerLeft { .. })),
        );
    }
    let rx = host.backend().stats.received - rx0;
    if !left.is_empty() {
        return Err(format!("the host dropped the idle guest: {left:?}"));
    }
    if host.players() != vec![PlayerId(1)] || !guest.is_connected() {
        return Err("the idle link closed".into());
    }
    if rx < 25 {
        return Err(format!(
            "the host got only {rx} datagrams in 30 s of idle time"
        ));
    }
    host.send(
        PlayerId(1),
        Channel::ReliableOrdered,
        b"still here".to_vec(),
    )
    .map_err(|e| format!("host send after the idle time: {e}"))?;
    let mut got = None;
    let end = mc.now() + 1_000;
    while got.is_none() && mc.now() < end {
        step(&mc, 10, &mut [&mut host, &mut guest]);
        got = guest.recv().map(|p| p.data);
    }
    if got.as_deref() != Some(&b"still here"[..]) {
        return Err("the link carried no data after the idle time".into());
    }
    Ok(format!(
        "an idle UDP link on 127.0.0.1 stayed open for 30 s of manual-clock time (silence limit 20 s) on keep-alives alone ({rx} datagrams reached the host), then carried a reliable message"
    ))
}

// ---------------------------------------------------------------- core.join_codes

/// The code with the character at `at` (a char index) replaced by another code character.
fn typo(code: &str, at: usize) -> String {
    let mut c: Vec<char> = code.chars().collect();
    c[at] = "0123456789ABCDEFGHJKMNPQRSTVWXYZ"
        .chars()
        .find(|a| *a != c[at].to_ascii_uppercase())
        .unwrap();
    c.into_iter().collect()
}

/// NVRS codes for IPv4, IPv6 and a Steam lobby carry target and random token through format and
/// parse. Every single-character typo is refused. Mutation: the formatted code gets a typo
/// before the parse.
fn net_join_codes(_env: &Env, mutate: bool) -> Result<String, String> {
    let cases = [
        (
            "IPv4",
            JoinTarget::Direct("203.0.113.7:27961".parse().unwrap()),
        ),
        (
            "IPv6",
            JoinTarget::Direct("[2001:db8::5]:40000".parse().unwrap()),
        ),
        (
            "Steam lobby",
            JoinTarget::SteamLobby(109_775_241_234_567_890),
        ),
    ];
    let mut tokens: Vec<Token> = Vec::new();
    let mut typos = 0usize;
    for (kind, target) in cases {
        let token = random_token();
        let code = JoinCode::new(target.clone(), token);
        let text = code.to_string();
        let body = text
            .strip_prefix("NVRS-")
            .ok_or_else(|| format!("the {kind} code {text} lacks the NVRS- prefix"))?;
        let groups: Vec<&str> = body.split('-').collect();
        let (last, full) = groups.split_last().unwrap();
        if full.iter().any(|g| g.len() != 5)
            || last.is_empty()
            || last.len() > 5
            || !body.chars().all(|c| c == '-' || c.is_ascii_alphanumeric())
        {
            return Err(format!("the {kind} code {text} is not in groups of five"));
        }
        // Positions of the code characters after the prefix.
        let pos: Vec<usize> = text
            .chars()
            .enumerate()
            .skip(5)
            .filter(|(_, c)| c.is_ascii_alphanumeric())
            .map(|(i, _)| i)
            .collect();
        // About three quarters in: inside the token bytes for all three kinds.
        let token_char = pos[pos.len() * 3 / 4];
        let typed = if mutate {
            typo(&text, token_char)
        } else {
            text.clone()
        };
        let back: JoinCode = typed
            .parse()
            .map_err(|e: JoinError| format!("the {kind} code did not parse back: {e}"))?;
        if back.target != target || back.token != token {
            return Err(format!("the {kind} code parsed to another target or token"));
        }
        let messy = typed
            .to_lowercase()
            .replace('-', " ")
            .replace('0', "o")
            .replace('1', "l");
        if messy.parse::<JoinCode>().as_ref() != Ok(&code) {
            return Err(format!(
                "the {kind} code typed in lower case with spaces and look-alikes did not parse"
            ));
        }
        for &i in &pos[..pos.len() - 1] {
            if typo(&typed, i).parse::<JoinCode>().is_ok() {
                return Err(format!("the {kind} code accepted a typo at character {i}"));
            }
            typos += 1;
        }
        match typo(&typed, token_char).parse::<JoinCode>() {
            Err(JoinError::BadChecksum) => {}
            other => {
                return Err(format!(
                    "a typo in the token of the {kind} code gave {other:?}, not a checksum error"
                ))
            }
        }
        tokens.push(token);
    }
    if tokens[0] == tokens[1] || tokens[1] == tokens[2] || tokens[0] == tokens[2] {
        return Err("random_token gave the same token twice".into());
    }
    Ok(format!(
        "IPv4, IPv6 and Steam lobby NVRS codes carried their target and random token through format and parse (lower case, spaces and look-alikes forgiven), all {typos} single-character typos were refused, a token typo read as a checksum mismatch"
    ))
}

// ---------------------------------------------------------------- core.steam_transport

fn steam_opts() -> SteamOptions {
    SteamOptions {
        retry_sleep: Duration::from_millis(1),
        ..Default::default()
    }
}

fn settle<A: Backend, B: Backend>(mc: &ManualClock, a: &mut Endpoint<A>, b: &mut Endpoint<B>) {
    for _ in 0..10 {
        mc.advance(50);
        a.pump();
        b.pump();
    }
}

/// A host makes a marked lobby on MockSteam, its NVRS code carries the lobby id and token, a
/// guest joins from the code and messages cross both ways. Mutation: everything the host sends
/// vanishes inside the mock.
fn net_steam_mock(_env: &Env, mutate: bool) -> Result<String, String> {
    let uni = MockUniverse::new();
    let h = uni.user(1, "host");
    let g = uni.user(2, "guest");
    uni.befriend(&h, &g);
    let hid = h.self_id();
    let mc = ManualClock::new();
    let clock = Clock::Manual(mc.clone());
    let token = random_token();
    let mut host = Endpoint::host(
        SteamBackend::host(h, steam_opts()),
        token,
        ecfg(&clock, "host"),
    );
    host.pump();
    let lobby = host
        .backend()
        .lobby()
        .ok_or("MockSteam: the host made no lobby")?;
    if !matches!(host.backend().phase(), SteamPhase::Hosting { .. }) {
        return Err(format!(
            "MockSteam: the host is in phase {:?}",
            host.backend().phase()
        ));
    }
    if host
        .backend()
        .api()
        .get_lobby_data(lobby, MARKER_KEY)
        .as_deref()
        != Some("1")
    {
        return Err("MockSteam: the lobby lacks the nv-rs marker".into());
    }
    if host.backend().api().rich_presence("connect") != Some(format!("+connect_lobby {lobby}")) {
        return Err("MockSteam: the host's rich presence has no connect string".into());
    }
    let code = host
        .backend()
        .join_code(token)
        .ok_or("MockSteam: the host has no join code")?
        .to_string();
    let parsed: JoinCode = code
        .parse()
        .map_err(|e| format!("MockSteam: the lobby code did not parse: {e}"))?;
    let JoinTarget::SteamLobby(target) = parsed.target else {
        return Err("MockSteam: the code names no Steam lobby".into());
    };
    if target != lobby || parsed.token != token {
        return Err("MockSteam: the code carries another lobby or token".into());
    }
    if mutate {
        uni.faults(|f| {
            f.silent.insert(hid);
        });
    }
    let mut guest = Endpoint::guest(
        SteamBackend::guest(g, target, steam_opts()),
        parsed.token,
        ecfg(&clock, "guest"),
    );
    settle(&mc, &mut host, &mut guest);
    if !guest.is_connected() || guest.local() != PlayerId(1) {
        return Err("MockSteam: the guest did not join the lobby session as slot 1".into());
    }
    if !host.events().contains(&NetEvent::PeerJoined {
        player: PlayerId(1),
        name: "guest".into(),
    }) {
        return Err("MockSteam: the host saw no join".into());
    }
    let blob: Vec<u8> = (0..200_000u32).map(|i| (i % 251) as u8).collect();
    host.send(PlayerId(1), Channel::ReliableOrdered, blob.clone())
        .map_err(|e| format!("host send: {e}"))?;
    host.send(PlayerId(1), Channel::Unreliable, b"pos".to_vec())
        .map_err(|e| format!("host send: {e}"))?;
    guest
        .send(
            PlayerId(0),
            Channel::ReliableOrdered,
            b"hello host".to_vec(),
        )
        .map_err(|e| format!("guest send: {e}"))?;
    settle(&mc, &mut host, &mut guest);
    let got: Vec<_> = std::iter::from_fn(|| guest.recv()).collect();
    if !got
        .iter()
        .any(|p| p.channel == Channel::ReliableOrdered && p.data == blob)
    {
        return Err("MockSteam: the 200000-byte reliable blob did not reach the guest".into());
    }
    if !got
        .iter()
        .any(|p| p.channel == Channel::Unreliable && p.data == b"pos")
    {
        return Err("MockSteam: the unreliable frame did not reach the guest".into());
    }
    if host.recv().map(|p| p.data).as_deref() != Some(&b"hello host"[..]) {
        return Err("MockSteam: the guest's reply did not reach the host".into());
    }
    Ok("MockSteam (coop-net feature steam-mock, not real Steam): the host made a marked lobby with a connect rich presence, its NVRS code carried the lobby id and token, the guest joined from the code as slot 1, a 200000-byte reliable blob and an unreliable frame reached the guest, a reliable reply reached the host".into())
}

/// Friends join and invites on MockSteam: a friend finds the host hosting in Spacewar 480 after
/// a stale code fails, joins the friends-only lobby and gets the host's invite. A stranger is
/// refused. Mutation: the host and the guest are not Steam friends.
fn net_steam_mock_friends(_env: &Env, mutate: bool) -> Result<String, String> {
    let uni = MockUniverse::new();
    let h = uni.user(1, "host");
    let g = uni.user(2, "friend");
    let s = uni.user(3, "stranger");
    if !mutate {
        uni.befriend(&h, &g);
    }
    let (hid, gid, sid) = (h.self_id(), g.self_id(), s.self_id());
    let mc = ManualClock::new();
    let clock = Clock::Manual(mc.clone());
    let mut host = Endpoint::host(
        SteamBackend::host(h, steam_opts()),
        random_token(),
        ecfg(&clock, "host"),
    );
    host.pump();
    let lobby = host
        .backend()
        .lobby()
        .ok_or("MockSteam: the host made no lobby")?;
    // A stale code names a lobby that is gone.
    let mut fb = SteamBackend::guest(g, lobby + 1_000, steam_opts());
    fb.poll(0);
    match fb.phase() {
        SteamPhase::Failed(m) if m.contains("does not exist") => {}
        p => return Err(format!("MockSteam: a stale lobby gave phase {p:?}")),
    }
    // Friends join: the host shows as hosting that lobby in Spacewar 480.
    let hosting = fb.hosting_friends();
    let entry = hosting
        .iter()
        .find(|f| f.id == hid)
        .ok_or("MockSteam: the host is not among the friends hosting in Spacewar 480")?;
    if entry.lobby != lobby {
        return Err("MockSteam: the friends list names another lobby".into());
    }
    fb.retarget(entry.lobby);
    fb.poll(0);
    match fb.phase() {
        SteamPhase::Guest { lobby: l, owner } if *l == lobby && *owner == hid => {}
        p => {
            return Err(format!(
                "MockSteam: the friend could not enter the lobby: {p:?}"
            ))
        }
    }
    // Invites: the host's invite reaches the friend.
    if !host.backend_mut().invite_friend(gid) {
        return Err("MockSteam: the host could not invite its friend".into());
    }
    fb.poll(0);
    if !fb.take_invites().contains(&Invite { lobby, from: hid }) {
        return Err("MockSteam: the host's invite did not reach the friend".into());
    }
    // The friends-only lobby refuses a stranger, who cannot be invited either.
    if host.backend_mut().invite_friend(sid) {
        return Err("MockSteam: the host invited a stranger".into());
    }
    let mut sb = SteamBackend::guest(s, lobby, steam_opts());
    sb.poll(0);
    match sb.phase() {
        SteamPhase::Failed(m) if m.contains("not allowed") => {}
        p => return Err(format!("MockSteam: a stranger got phase {p:?}")),
    }
    // A stranger who messages the host directly is never delivered.
    let mut intruder = uni.user(9, "intruder");
    intruder.send_message(hid, 1, SEND_RELIABLE | SEND_NO_NAGLE, b"let me in");
    host.pump();
    if host.backend().rejected_sessions == 0 || !host.players().is_empty() {
        return Err("MockSteam: a stranger's direct message got through".into());
    }
    Ok("MockSteam (coop-net feature steam-mock, not real Steam): after a stale code failed, the friend found the host hosting in Spacewar 480, entered its friends-only lobby and got the host's invite; a stranger was refused by the lobby, could not be invited, and its direct message was dropped".into())
}

// ---------------------------------------------------------------- core.packet_encryption

const COOP_DOC: &str = include_str!("../../../../docs/COOP.md");
const NOT_ENCRYPTED: &str = "Packets are not encrypted.";

/// A UDP relay between guest and host that records every datagram.
struct Relay {
    sock: UdpSocket,
    addr: SocketAddr,
    host: SocketAddr,
    guest: Option<SocketAddr>,
    seen: Vec<Vec<u8>>,
}

impl Relay {
    fn new(host: SocketAddr) -> Result<Relay, String> {
        let sock = UdpSocket::bind(LOCAL).map_err(|e| format!("relay bind: {e}"))?;
        sock.set_nonblocking(true)
            .map_err(|e| format!("relay: {e}"))?;
        let addr = sock.local_addr().map_err(|e| format!("relay: {e}"))?;
        Ok(Relay {
            sock,
            addr,
            host,
            guest: None,
            seen: Vec::new(),
        })
    }

    fn pump(&mut self) {
        let mut buf = [0u8; 2048];
        loop {
            match self.sock.recv_from(&mut buf) {
                Ok((n, from)) => {
                    let d = buf[..n].to_vec();
                    let to = if from == self.host {
                        self.guest
                    } else {
                        self.guest = Some(from);
                        Some(self.host)
                    };
                    if let Some(to) = to {
                        let _ = self.sock.send_to(&d, to);
                    }
                    self.seen.push(d);
                }
                Err(e) if e.kind() == ErrorKind::ConnectionReset => continue,
                Err(_) => break,
            }
        }
    }
}

/// docs/COOP.md says packets are not encrypted, and it is true: a relay between a UDP guest
/// and host reads the session token and the payloads in clear. Mutation: the recorded
/// datagrams are XOR-scrambled (as an encrypting transport would) before the search.
fn net_wire_plaintext(env: &Env, mutate: bool) -> Result<String, String> {
    let limits = COOP_DOC
        .split("\n## ")
        .find(|s| s.starts_with("Limits"))
        .ok_or("docs/COOP.md has no Limits section")?;
    if !limits.contains(NOT_ENCRYPTED) {
        return Err(format!(
            "docs/COOP.md (Limits) does not say \"{NOT_ENCRYPTED}\""
        ));
    }
    let mc = ManualClock::new();
    let clock = Clock::Manual(mc.clone());
    let token = random_token();
    let cfg = UdpConfig {
        min_rto_ms: 20,
        ..Default::default()
    };
    let (mut host, host_addr) = udp_host(token, cfg.clone(), &clock)?;
    let mut relay = Relay::new(host_addr)?;
    let mut guest = udp_guest(relay.addr, token, cfg, &clock, "guest")?;
    let round =
        |host: &mut Endpoint<UdpBackend>, guest: &mut Endpoint<UdpBackend>, relay: &mut Relay| {
            mc.advance(10);
            guest.pump();
            relay.pump();
            host.pump();
            relay.pump();
        };
    while !guest.is_connected() && mc.now() < 3_000 {
        round(&mut host, &mut guest, &mut relay);
    }
    if !guest.is_connected() {
        return Err("the guest did not join through the relay".into());
    }
    let probe = |tag: &str| format!("nv-rs wire probe: {tag} {}", env.seed).into_bytes();
    let (rel_h, rel_g, unrel_h) = (
        probe("host reliable"),
        probe("guest reliable"),
        probe("host unreliable"),
    );
    host.send(PlayerId(1), Channel::ReliableOrdered, rel_h.clone())
        .map_err(|e| format!("host send: {e}"))?;
    host.send(PlayerId(1), Channel::Unreliable, unrel_h.clone())
        .map_err(|e| format!("host send: {e}"))?;
    guest
        .send(PlayerId(0), Channel::ReliableOrdered, rel_g.clone())
        .map_err(|e| format!("guest send: {e}"))?;
    let (mut at_guest, mut at_host) = (Vec::new(), Vec::new());
    let end = mc.now() + 2_000;
    while (at_guest.len() < 2 || at_host.is_empty()) && mc.now() < end {
        round(&mut host, &mut guest, &mut relay);
        at_guest.extend(std::iter::from_fn(|| guest.recv()).map(|p| p.data));
        at_host.extend(std::iter::from_fn(|| host.recv()).map(|p| p.data));
    }
    if !at_guest.contains(&rel_h) || !at_guest.contains(&unrel_h) || !at_host.contains(&rel_g) {
        return Err("the probe messages did not cross the relay".into());
    }
    let mut wire = std::mem::take(&mut relay.seen);
    if mutate {
        for d in &mut wire {
            for b in d.iter_mut() {
                *b ^= 0x5A;
            }
        }
    }
    for (what, needle) in [
        ("the session token", &token[..]),
        ("the host's reliable payload", &rel_h[..]),
        ("the guest's reliable payload", &rel_g[..]),
        ("the host's unreliable payload", &unrel_h[..]),
    ] {
        if !wire.iter().any(|d| contains(d, needle)) {
            return Err(format!(
                "{what} is not readable in the {} datagrams on the wire",
                wire.len()
            ));
        }
    }
    Ok(format!(
        "docs/COOP.md (Limits) says \"{NOT_ENCRYPTED}\" and it is true for UDP: a relay on 127.0.0.1 read the session token and the reliable and unreliable payloads in clear in the {} datagrams it carried",
        wire.len()
    ))
}

// ---------------------------------------------------------------- core.join_service_codes

const SERVICE_CODE_TS: &str = include_str!("../../../../services/join-service/src/code.ts");

/// The value of `export const NAME = VALUE;` in TypeScript source, quotes removed.
fn ts_const<'a>(src: &'a str, name: &str) -> Option<&'a str> {
    let key = format!("export const {name} = ");
    let rest = &src[src.find(&key)? + key.len()..];
    Some(rest[..rest.find(';')?].trim().trim_matches('"'))
}

/// Partial: the game parses six-character codes over exactly the join service's alphabet and
/// refuses them with a clear message, because it never calls the service. Resolving a short
/// code is not proven (the row needs code). Mutation: the service issues a symbol the game
/// does not take.
fn net_join_service_codes(_env: &Env, mutate: bool) -> Result<String, String> {
    use coop_session::target::{normalize_code, JoinTarget as Typed, CODE_ALPHABET, CODE_LEN};
    let alphabet = ts_const(SERVICE_CODE_TS, "ALPHABET")
        .ok_or("services/join-service/src/code.ts has no ALPHABET")?;
    let len: usize = ts_const(SERVICE_CODE_TS, "CODE_LENGTH")
        .and_then(|s| s.parse().ok())
        .ok_or("services/join-service/src/code.ts has no CODE_LENGTH")?;
    let issued = if mutate {
        alphabet.replacen('Z', "5", 1)
    } else {
        alphabet.to_string()
    };
    if issued != CODE_ALPHABET || len != CODE_LEN {
        return Err(format!(
            "the game's short codes ({CODE_LEN} of {CODE_ALPHABET}) differ from the service's ({len} of {issued})"
        ));
    }
    for c in issued.chars() {
        let code: String = std::iter::repeat(c).take(len).collect();
        match Typed::parse(&code) {
            Ok(Typed::Code(n)) if n == code => {}
            other => {
                return Err(format!(
                    "the game does not take the service code {code}: {other:?}"
                ))
            }
        }
    }
    for typed in ["k7m-3xw", " k7 m3 xw ", "K7M3XW"] {
        if Typed::parse(typed) != Ok(Typed::Code("K7M3XW".into())) {
            return Err(format!(
                "the typed code {typed:?} did not normalize to K7M3XW"
            ));
        }
    }
    for c in ['0', 'O', '1', 'I', 'L', 'U', 'S', '5'] {
        if normalize_code(&format!("ABCDE{c}")).is_some() {
            return Err(format!(
                "the game takes the look-alike {c}, which the service never issues"
            ));
        }
    }
    if normalize_code("ABCDE").is_some() || normalize_code("ABCDEFG").is_some() {
        return Err("the game takes a code of the wrong length".into());
    }
    let req = coop_session::link::LinkRequest::new(coop_session::Role::Guest, "guest", "K7M3XW");
    let why = match coop_session::link::open(&req) {
        Ok(_) => {
            return Err(
                "the game opened a link from a short code: update this check and the row".into(),
            )
        }
        Err(e) => e.to_string(),
    };
    if !why.contains("join service") {
        return Err(format!(
            "a short code was refused without naming the join service: {why}"
        ));
    }
    Ok(format!(
        "the game parses {CODE_LEN}-character codes over the service's {}-symbol alphabet (look-alikes and wrong lengths refused); with no join service set it refuses a short code and names the service: \"{why}\" (the resolve itself: net_join_service_resolve)",
        CODE_ALPHABET.len()
    ))
}

// ---------------------------------------------------------------- save.launcher

const VIEWER: &str = r"C:\nv-rs\app\nv-viewer.exe";
const DATA: &str = r"D:\Games\Fallout New Vegas\Data";
const CAMPAIGN: &str = "00ab12cd34ef5678";
const STAMP: &str = "20261102-1200";

/// The launcher's host (continue and new game) and guest (NVRS code, own save, Steam lobby)
/// command lines, through `cli::plan_from_cli`, `command::build` and `party::extend_spec` as
/// `--start` runs them. Mutation: the command builder gets the host's and the guest's roles
/// swapped.
fn launcher_command_lines(_env: &Env, mutate: bool) -> Result<String, String> {
    let scratch = Scratch::new("launcher")?;
    let home = scratch.path().join("home");
    let exe_dir = scratch.path().join("exe");
    let random = [0x11u8; 8];
    let token = random_token();
    let s = |p: &Path| p.to_string_lossy().into_owned();
    let host_save = home
        .join("saves")
        .join(CAMPAIGN)
        .join("host-ann")
        .join("nv-rs-quicksave.txt");
    let guest_save = home
        .join("saves")
        .join(CAMPAIGN)
        .join("guest-bo")
        .join("nv-rs-quicksave.txt");
    let plan = |args: &[String], mode: &str| {
        let c = cli::parse(args)?;
        cli::plan_from_cli(
            &c,
            mode,
            Settings::default(),
            &home,
            &exe_dir,
            random,
            token,
            STAMP.into(),
            false,
        )
    };
    let base = |start: &str, name: &str| {
        sv(&[
            "--start", start, "--data", DATA, "--viewer", VIEWER, "--name", name,
        ])
    };
    // Host: continue the campaign from its save, on the default port.
    let mut a = base("host", "Ann");
    a.extend(sv(&[
        "--port",
        "27961",
        "--continue",
        CAMPAIGN,
        "--save",
        &s(&host_save),
    ]));
    let (mut hl, hli) = plan(&a, "host")?;
    // The join code the host shows for the address the player gave.
    let code = joincode::host_code("192.168.1.20", 27961, hli.token)?;
    // Guest: the code typed in lower case with spaces, and the guest's own save.
    let mut a = base("join", "Bo");
    a.extend(sv(&[
        "--join",
        &code.to_lowercase().replace('-', " "),
        "--campaign",
        CAMPAIGN,
        "--save",
        &s(&guest_save),
    ]));
    let (mut gl, gli) = plan(&a, "join")?;
    // Host: a new game.
    let (nl, nli) = plan(&base("host", "Ann"), "host")?;
    // Guest: a Steam lobby code.
    let steam_code =
        JoinCode::new(JoinTarget::SteamLobby(109_775_241_000_000_001), [3; 16]).to_string();
    let mut a = base("join", "Cy");
    a.extend(sv(&["--join", &steam_code]));
    let (sl, sli) = plan(&a, "join")?;
    if mutate {
        std::mem::swap(&mut hl.plan.mode, &mut gl.plan.mode);
    }
    let build = |l: &cli::Launch, li: &cli::LaunchInput| {
        let mut spec = command::build(&l.plan);
        party::extend_spec(&mut spec, l.role, li.new_game, &home, random);
        spec
    };
    let (hs, gs, ns, ss) = (
        build(&hl, &hli),
        build(&gl, &gli),
        build(&nl, &nli),
        build(&sl, &sli),
    );
    let log = s(&home.join("logs").join(format!("coop-{STAMP}.jsonl")));
    let tok = joincode::token_hex(&token);
    let shown = |v: &[String]| scratch.hide(&v.join(" | "));
    let want = sv(&[
        DATA,
        "--coop-load",
        &s(&host_save),
        "--coop-host",
        "27961",
        "--coop-token",
        &tok,
        "--coop-name",
        "Ann",
        "--coop-log",
        &log,
    ]);
    if hs.args != want {
        return Err(format!(
            "host continue: got {}, want {}",
            shown(&hs.args),
            shown(&want)
        ));
    }
    if hs.program != Path::new(VIEWER)
        || hs.cwd != home.join("saves").join(CAMPAIGN).join("host-ann")
        || !hs.env.is_empty()
    {
        return Err("host continue: wrong program, folder or environment".into());
    }
    if !command::render(&hs).contains(&format!("cmd: {VIEWER} \"{DATA}\" --coop-load ")) {
        return Err("host continue: the printed command differs from the built one".into());
    }
    let parsed: JoinCode = code
        .parse()
        .map_err(|e| format!("the host's join code: {e}"))?;
    if parsed.target != JoinTarget::Direct("192.168.1.20:27961".parse().unwrap())
        || joincode::token_hex(&parsed.token) != tok
    {
        return Err("the join code does not carry the host's address and --coop-token".into());
    }
    let want = sv(&[
        DATA,
        "--new-game",
        "--coop-host",
        "27961",
        "--coop-token",
        &tok,
        "--coop-name",
        "Ann",
        "--coop-log",
        &log,
    ]);
    if ns.args != want {
        return Err(format!(
            "host new game: got {}, want {}",
            shown(&ns.args),
            shown(&want)
        ));
    }
    let gdir = home.join("saves").join(CAMPAIGN).join("guest-bo");
    let key = std::fs::read_to_string(home.join("player-key.txt")).unwrap_or_default();
    let want = sv(&[
        DATA,
        "--coop-join",
        &code,
        "--coop-load",
        &s(&guest_save),
        "--coop-name",
        "Bo",
        "--coop-log",
        &log,
        "--coop-player-key",
        key.trim(),
        "--coop-character",
        &s(&gdir.join(party::CHARACTER_FILE)),
    ]);
    if gs.args != want || key.trim().len() != 16 {
        return Err(format!(
            "guest: got {}, want {}",
            shown(&gs.args),
            shown(&want)
        ));
    }
    if gs.cwd != gdir || gs.program != Path::new(VIEWER) {
        return Err("guest: wrong program or folder".into());
    }
    if ss.args.get(1..3) != Some(&sv(&["--coop-join", &steam_code])[..])
        || !ss.args.iter().any(|x| x == "--coop-steam")
        || ss
            .args
            .iter()
            .any(|x| x == "--coop-host" || x == "--coop-token")
    {
        return Err(format!("Steam guest: got {}", shown(&ss.args)));
    }
    Ok("host continue: Data, --coop-load with its save, --coop-host 27961, --coop-token equal to the token in its NVRS join code; host new game: --new-game; guest: --coop-join with the normalized code, its own save, player key and character; a Steam lobby code adds --coop-steam".into())
}

/// A free UDP port outside the OS's ephemeral ranges, so no other socket of this run gets it.
fn free_port() -> Result<u16, String> {
    let seed: u64 = unique().bytes().fold(0xcbf2_9ce4_8422_2325, |h, b| {
        (h ^ b as u64).wrapping_mul(0x100_0000_01b3)
    });
    for k in 0..64u64 {
        let p = 20_000 + ((seed.wrapping_add(k.wrapping_mul(7_919))) % 12_000) as u16;
        if UdpSocket::bind(("0.0.0.0", p)).is_ok() {
            return Ok(p);
        }
    }
    Err("no free UDP port between 20000 and 32000".into())
}

/// The launcher's port test: the listen test, the local round trip and a probe against a
/// listening launcher pass on a free port, and a held port reads as in use. Mutation: another
/// socket holds the port.
fn launcher_port_test(_env: &Env, mutate: bool) -> Result<String, String> {
    let port = free_port()?;
    let _held = if mutate {
        Some(UdpSocket::bind(("0.0.0.0", port)).map_err(|e| format!("hold the port: {e}"))?)
    } else {
        None
    };
    let b = portcheck::bind_test(port);
    if !b.ok {
        return Err(format!("listen test: {}", b.detail));
    }
    let l = portcheck::loopback_test(port);
    if !l.ok {
        return Err(format!("local round trip: {}", l.detail));
    }
    let stop = Arc::new(AtomicBool::new(false));
    let stop2 = stop.clone();
    let listener = std::thread::spawn(move || {
        let mut hits = 0usize;
        let r = portcheck::listen_echo(port, 3, &|| stop2.load(Ordering::Relaxed), &mut |_| {
            hits += 1
        });
        (r, hits)
    });
    std::thread::sleep(Duration::from_millis(50));
    let p = portcheck::probe(
        SocketAddr::from(([127, 0, 0, 1], port)),
        Duration::from_millis(1500),
    );
    stop.store(true, Ordering::Relaxed);
    let (r, hits) = listener
        .join()
        .map_err(|_| "the listening thread panicked".to_string())?;
    let answered = r.map_err(|e| format!("listen: {e}"))?;
    if !p.ok || answered == 0 || hits == 0 {
        return Err(format!("probe: {}", p.detail));
    }
    let hold = UdpSocket::bind(("0.0.0.0", port)).map_err(|e| format!("hold the port: {e}"))?;
    let busy = portcheck::bind_test(port);
    drop(hold);
    if busy.ok || !busy.detail.contains("already in use") {
        return Err(format!("a held port gave: {}", busy.detail));
    }
    Ok(format!(
        "UDP port test: a free port passed the listen test, a local round trip arrived, a probe to 127.0.0.1 got the listening launcher's echo ({answered} answered), a port held by another socket read as already in use"
    ))
}

/// A made-up Windows user name for the redaction check.
const FAKE_USER: &str = "kestrel7";

fn no_user(what: &str, text: &str) -> Result<(), String> {
    if text.to_ascii_lowercase().contains(FAKE_USER) {
        Err(format!("{what} still names the Windows user"))
    } else if !text.contains("<user>") {
        Err(format!("{what} shows no <user> mark"))
    } else {
        Ok(())
    }
}

/// The launcher log (the command line and the viewer's lines, as `run::run_viewer` writes them)
/// and the diagnostics folder (`diag::collect`) name the Windows user only as `<user>`.
/// Mutation: the launcher does not know the user name, so bare mentions stay.
fn launcher_log_redaction(_env: &Env, mutate: bool) -> Result<String, String> {
    let profile = format!(r"C:\Users\{FAKE_USER}");
    let home = PathBuf::from(format!(r"{profile}\AppData\Roaming\nv-rs-coop"));
    let settings = Settings {
        data_path: format!(r"{profile}\Games\Fallout New Vegas\Data"),
        player_name: "Ann".into(),
        ..Settings::default()
    };
    let viewer = format!(r"{profile}\nv-rs\nv-viewer.exe");
    let c = cli::parse(&sv(&["--start", "host", "--viewer", &viewer]))?;
    let (l, _) = cli::plan_from_cli(
        &c,
        "host",
        settings.clone(),
        &home,
        Path::new("."),
        [7; 8],
        random_token(),
        STAMP.into(),
        false,
    )?;
    let raw = command::render(&command::build(&l.plan));
    let viewer_line =
        format!("Co-op: signed in as Kestrel7, profile C:/Users/{FAKE_USER}/Documents");
    if !raw.contains(FAKE_USER) {
        return Err("the test command has no user name to remove".into());
    }
    let user = if mutate { None } else { Some(FAKE_USER) };
    // What run_viewer writes: the rendered command, then every viewer line, each redacted.
    let log = format!(
        "{}\n{}\n",
        redact::redact(&raw, user),
        redact::redact(&viewer_line, user)
    );
    no_user("the launcher log", &log)?;
    if !log.contains("--new-game --coop-host 27961 --coop-token ") {
        return Err("the redaction damaged the logged command".into());
    }
    // Diagnostics: settings and logs copied into a folder to share.
    let scratch = Scratch::new("redact")?;
    let dhome = scratch.path().join("home");
    std::fs::create_dir_all(dhome.join("logs")).map_err(|e| format!("scratch: {e}"))?;
    settings
        .save(&dhome.join("settings.json"))
        .map_err(|e| format!("scratch settings: {e}"))?;
    std::fs::write(
        dhome.join("logs").join(format!("launcher-{STAMP}.log")),
        format!("{raw}\n{viewer_line}\n"),
    )
    .map_err(|e| format!("scratch log: {e}"))?;
    let out = scratch.path().join("diag");
    let info = format!("data folder {}", settings.data_path);
    let files = diag::collect(&dhome, &out, user, &info).map_err(|e| scratch.hide(&e))?;
    if files.len() < 3 {
        return Err(format!("diagnostics wrote only {files:?}"));
    }
    for f in &files {
        let t = std::fs::read_to_string(out.join(f)).map_err(|e| format!("{f}: {e}"))?;
        no_user(&format!("diagnostics file {f}"), &t)?;
    }
    Ok(format!(
        "the launcher log (command line and viewer lines) and the {} diagnostics files (info, settings, log) keep the command but name the Windows user only as <user>",
        files.len()
    ))
}

// ---------------------------------------------------------------- save.install_detect

/// Registry entries as `(hive, key, value, data)`.
struct FakeRegistry(Vec<(&'static str, String, &'static str, String)>);

impl install::Registry for FakeRegistry {
    fn get(&self, hive: &str, key: &str, value: &str) -> Option<String> {
        self.0
            .iter()
            .find(|(h, k, v, _)| *h == hive && k == key && *v == value)
            .map(|e| e.3.clone())
    }
}

/// An MZ stub with a `VS_FIXEDFILEINFO` (signature 0xFEEF04BD) that holds the file version.
fn fake_exe(v: [u16; 4]) -> Vec<u8> {
    let mut b = b"MZ".to_vec();
    b.extend_from_slice(&[0; 62]);
    b.extend_from_slice(&0xFEEF_04BDu32.to_le_bytes());
    b.extend_from_slice(&0x0001_0000u32.to_le_bytes());
    b.extend_from_slice(&((v[0] as u32) << 16 | v[1] as u32).to_le_bytes());
    b.extend_from_slice(&((v[2] as u32) << 16 | v[3] as u32).to_le_bytes());
    b.extend_from_slice(&[0; 32]);
    b
}

/// A game folder: `Data\FalloutNV.esm` and, unless `no_exe`, `FalloutNV.exe` 1.4.0.525.
fn make_game(dir: &Path, no_exe: bool) -> Result<(), String> {
    let io = |e: std::io::Error| format!("scratch game folder: {e}");
    std::fs::create_dir_all(dir.join("Data")).map_err(io)?;
    std::fs::write(dir.join("Data").join("FalloutNV.esm"), b"TES4").map_err(io)?;
    if !no_exe {
        std::fs::write(dir.join(install::GAME_EXE), fake_exe(install::SUPPORTED)).map_err(io)?;
    }
    Ok(())
}

fn same_dir(a: &Path, b: &Path) -> bool {
    let n = |p: &Path| {
        p.to_string_lossy()
            .replace('/', "\\")
            .trim_end_matches('\\')
            .to_ascii_lowercase()
    };
    n(a) == n(b)
}

/// Fake Steam (registry, second library in libraryfolders.vdf, appmanifest installdir), GOG
/// and retail (registry) installs and a common drive folder under a scratch folder are all
/// found and pass the install check. Mutation: the game executables are missing.
fn launcher_install_detect(_env: &Env, mutate: bool) -> Result<String, String> {
    let scratch = Scratch::new("install")?;
    let root = scratch.path();
    let io = |e: std::io::Error| format!("scratch: {e}");
    let vdf_path = |p: &Path| p.to_string_lossy().replace('\\', "\\\\");
    // Steam: the registry names the Steam folder, its libraryfolders.vdf lists a second
    // library, and the app manifest there names a custom install folder.
    let steam = root.join("Steam");
    let lib2 = root.join("Library Two");
    std::fs::create_dir_all(steam.join("steamapps")).map_err(io)?;
    std::fs::create_dir_all(lib2.join("steamapps")).map_err(io)?;
    std::fs::write(
        steam.join("steamapps").join("libraryfolders.vdf"),
        format!(
            "\"libraryfolders\"\n{{\n\t\"0\"\n\t{{\n\t\t\"path\"\t\t\"{}\"\n\t}}\n\t\"1\"\n\t{{\n\t\t\"path\"\t\t\"{}\"\n\t}}\n}}\n",
            vdf_path(&steam),
            vdf_path(&lib2)
        ),
    )
    .map_err(io)?;
    std::fs::write(
        lib2.join("steamapps").join("appmanifest_22380.acf"),
        "\"AppState\"\n{\n\t\"appid\"\t\t\"22380\"\n\t\"installdir\"\t\t\"FNV Custom\"\n}\n",
    )
    .map_err(io)?;
    let steam_game = lib2.join("steamapps").join("common").join("FNV Custom");
    let gog_game = root
        .join("GOG Galaxy")
        .join("Games")
        .join("Fallout New Vegas");
    let retail_game = root
        .join("Program Files (x86)")
        .join("Bethesda Softworks")
        .join("Fallout New Vegas");
    let drive = root.join("drive");
    let common_game = drive.join("Games").join("Fallout New Vegas");
    for g in [&steam_game, &gog_game, &retail_game, &common_game] {
        make_game(g, mutate)?;
    }
    let reg = FakeRegistry(vec![
        (
            "HKCU",
            r"Software\Valve\Steam".into(),
            "SteamPath",
            steam.to_string_lossy().replace('\\', "/"),
        ),
        (
            "HKLM",
            r"SOFTWARE\WOW6432Node\GOG.com\Games\1454587428".into(),
            "path",
            gog_game.to_string_lossy().into_owned(),
        ),
        (
            "HKLM",
            r"SOFTWARE\WOW6432Node\Bethesda Softworks\FalloutNV".into(),
            "Installed Path",
            retail_game.to_string_lossy().into_owned(),
        ),
    ]);
    let found = install::candidates(&reg, &HashMap::new(), std::slice::from_ref(&drive));
    for (source, dir) in [
        ("steam", &steam_game),
        ("gog", &gog_game),
        ("retail", &retail_game),
        ("common", &common_game),
    ] {
        let c = found
            .iter()
            .find(|c| c.source == source)
            .ok_or_else(|| format!("the {source} install was not found"))?;
        if !same_dir(&c.game_dir, dir) {
            return Err(format!(
                "the {source} install was found at {}",
                scratch.hide(&c.game_dir.to_string_lossy())
            ));
        }
        let r = install::check(&c.game_dir);
        if !r.ok() || r.version != Some(install::SUPPORTED) {
            return Err(format!(
                "the {source} install failed the check: {}",
                scratch.hide(&r.problems.join(" "))
            ));
        }
    }
    if found.len() != 4 {
        return Err(format!("{} installs found, 4 expected", found.len()));
    }
    for typed in [
        retail_game.join("Data"),
        retail_game.join(install::GAME_EXE),
    ] {
        if !same_dir(&install::normalize_game_dir(&typed), &retail_game) {
            return Err("a typed path to Data or the exe does not give the game folder".into());
        }
    }
    Ok("Steam (registry, a second library in libraryfolders.vdf, appmanifest installdir), GOG and retail (registry) and a common drive folder: all 4 fake installs in a scratch folder were found and checked as 1.4.0.525".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn env() -> Env<'static> {
        Env {
            data: None,
            peers: 2,
            net: "perfect",
            seed: 1,
        }
    }

    fn twin(name: &str) {
        let reg = registry();
        let f = reg
            .iter()
            .find(|(n, _)| *n == name)
            .unwrap_or_else(|| panic!("{name} is not registered"))
            .1;
        let e = env();
        let t = Instant::now();
        match f(&e, false) {
            Ok(d) => println!("{name}: {d} ({} ms)", t.elapsed().as_millis()),
            Err(why) => panic!("{name} failed: {why}"),
        }
        let t = Instant::now();
        match f(&e, true) {
            Ok(d) => panic!("the twin of {name} passed: {d}"),
            Err(why) => println!("twin of {name}: {why} ({} ms)", t.elapsed().as_millis()),
        }
    }

    /// The seeded checks hold for other seeds too (the checklist runs seed 7).
    #[test]
    fn seeded_checks_hold_for_many_seeds() {
        for seed in 1..=12 {
            let e = Env { seed, ..env() };
            for (name, f) in [
                ("net_udp_reliable_loss", net_udp_reliable_loss as Check),
                ("net_wire_plaintext", net_wire_plaintext),
            ] {
                if let Err(why) = f(&e, false) {
                    panic!("{name} seed {seed}: {why}");
                }
                if let Ok(d) = f(&e, true) {
                    panic!("the twin of {name} passed with seed {seed}: {d}");
                }
            }
        }
    }

    #[test]
    fn names_are_unique() {
        let reg = registry();
        let mut names: Vec<_> = reg.iter().map(|(n, _)| *n).collect();
        names.sort();
        names.dedup();
        assert_eq!(names.len(), reg.len());
    }

    #[test]
    fn udp_handshake() {
        twin("net_udp_handshake");
    }

    #[test]
    fn udp_reliable_loss() {
        twin("net_udp_reliable_loss");
    }

    #[test]
    fn udp_keepalive() {
        twin("net_udp_keepalive");
    }

    #[test]
    fn join_codes() {
        twin("net_join_codes");
    }

    #[test]
    fn steam_mock() {
        twin("net_steam_mock");
    }

    #[test]
    fn steam_mock_friends() {
        twin("net_steam_mock_friends");
    }

    #[test]
    fn wire_plaintext() {
        twin("net_wire_plaintext");
    }

    #[test]
    fn join_service_codes() {
        twin("net_join_service_codes");
    }

    #[test]
    fn launcher_command_lines_twin() {
        twin("launcher_command_lines");
    }

    #[test]
    fn launcher_port_test_twin() {
        twin("launcher_port_test");
    }

    #[test]
    fn launcher_log_redaction_twin() {
        twin("launcher_log_redaction");
    }

    #[test]
    fn launcher_install_detect_twin() {
        twin("launcher_install_detect");
    }
}

//! Checks for the story effects, the join service and the harness tools: the
//! effect coverage of real script lines, a short join code end to end against
//! a local stand-in of the join service, and the flight recorder, `diverge`,
//! `shrink` and `gentest`.
//!
//! Every check takes `mutate`: with it set the check sabotages the thing under
//! test and must then fail (the mutation twin of the row).

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream, UdpSocket};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use coop_core::sim_test::test_hello;
use coop_core::sync::{HostConfig, HostSync};
use coop_net::joinsvc::{CurlHttp, Hosted, JoinService, CODE_ALPHABET, CODE_LEN};
use coop_session::link::{self, LinkRequest};
use coop_session::{Role, Session, SessionConfig};
use nv_launcher::joincode::{self, JoinInput};
use world::coop::export::{domain_of, export};
use world::coop::{Authority, Domain, HostBridge};
use world::scripting::{GameState, Runner};

use super::builtin::{Check, Env};
use crate::data::Data;
use crate::json::J;
use crate::recorder::{self, Recording};
use crate::scenario::Scenario;
use crate::sim::{self, RunOpts};

/// The checks of this module by name.
pub fn registry() -> Vec<(&'static str, Check)> {
    vec![
        ("story_effects_covered", story_effects_covered),
        ("net_join_service_resolve", net_join_service_resolve),
        ("harness_recorder", harness_recorder),
        ("harness_diverge", harness_diverge),
        ("harness_shrink", harness_shrink),
        ("harness_gentest", harness_gentest),
    ]
}

fn open_data(env: &Env) -> Result<Arc<Data>, String> {
    let d = env.data.ok_or("needs a data folder")?;
    Data::open(&d.dir).map(Arc::new)
}

fn scratch(name: &str, mutate: bool) -> Result<std::path::PathBuf, String> {
    let d = std::env::temp_dir().join(format!(
        "coop-check-{name}-{}-{}",
        std::process::id(),
        u8::from(mutate)
    ));
    if d.exists() {
        std::fs::remove_dir_all(&d).map_err(|e| format!("{}: {e}", d.display()))?;
    }
    std::fs::create_dir_all(&d).map_err(|e| format!("{}: {e}", d.display()))?;
    Ok(d)
}

// ------------------------------------------------------- story.effects_captured

/// Real script lines of the game (each one is in a vanilla script or result
/// script of `FalloutNV.esm`, or uses its editor ids), with the shared domain
/// each one must write.
const STORY_LINES: [(&str, Domain); 17] = [
    ("SetStage VCG01 75", Domain::Stage),
    ("player.additem Stimpak 3", Domain::Items),
    ("player.additem caps001 50", Domain::Items),
    ("player.removeitem caps001 10", Domain::Items),
    ("set TimeScale to 10", Domain::Global),
    ("RewardKarma 50", Domain::Karma),
    ("RewardXP 100", Domain::Experience),
    ("AddReputation RepNVNCR 1 3", Domain::Reputation),
    (
        "SetEnemy PlayerFaction PowderGangerFactionNV",
        Domain::FactionRelation,
    ),
    ("ShowMap NiptonMapMarker", Domain::MapMarker),
    ("AddNote AWMNotePassword", Domain::Note),
    ("player.addperk BetterHealing", Domain::Perk),
    ("addtopic PrimmSlimGun", Domain::Topic),
    ("set VCG01.bRunTimer to 1", Domain::Vars),
    ("SetObjectiveDisplayed VCG02 60 1", Domain::Objective),
    ("DocMitchellREF.Disable", Domain::Disabled),
    ("DocMitchellREF.Enable", Domain::Disabled),
];

/// The domains whose exported fields differ between two exports.
fn changed_domains(
    a: &world::coop::export::Fields,
    b: &world::coop::export::Fields,
) -> BTreeSet<Domain> {
    a.keys()
        .chain(b.keys())
        .filter(|k| a.get(k) != b.get(k))
        .map(domain_of)
        .collect()
}

/// `story.effects_captured`: real script lines run on the host through the
/// game's `Runner`, and `HostBridge::collect` (the per-frame host driver, with
/// its keyed-diff backstop on every call) finds no store write that a semantic
/// effect did not announce. Each line must write its own domain. Mutation: a
/// raw writer changes a reputation and a map marker with no effect, and the
/// backstop must report both.
fn story_effects_covered(env: &Env, mutate: bool) -> Result<String, String> {
    let d = env.data.ok_or("needs a data folder")?;
    let mut gs = GameState::new(&d.order);
    gs.set_authority(Authority::Host);
    let mut host = HostSync::new(HostConfig::new(test_hello(), 1));
    let mut hb = HostBridge::default();
    hb.backstop_every = 1;
    // the start state goes into the store first (no effect announces it)
    hb.collect(&mut gs, &mut host);
    hb.uncovered_total.clear();
    let mut writes = 0u32;
    let mut seen: BTreeSet<Domain> = BTreeSet::new();
    for (line, want) in STORY_LINES {
        let before = export(&gs);
        {
            let mut r = Runner::new(&d.order, &d.scripts, &mut gs);
            r.run_source(line, None, None);
            r.run_item_scripts();
        }
        let changed = changed_domains(&before, &export(&gs));
        let rep = hb.collect(&mut gs, &mut host);
        if !changed.contains(&want) {
            return Err(format!(
                "`{line}` wrote nothing in {want:?} (changed: {changed:?}); the line does not run here"
            ));
        }
        if !rep.uncovered.is_empty() {
            return Err(format!(
                "`{line}`: the backstop found store writes with no effect: {:?}",
                rep.uncovered
            ));
        }
        writes += rep.written + rep.cleared;
        seen.extend(changed);
    }
    if mutate {
        // a writer that forgot to emit its effect
        let rep_form = d
            .order
            .form_by_editor_id("RepNVGoodsprings")
            .ok_or("no RepNVGoodsprings in the data")?;
        gs.reputations.insert(rep_form, (9.0, 0.0));
        let marker = d
            .order
            .form_by_editor_id("GoodspringsMarkerREF")
            .or_else(|| d.order.form_by_editor_id("CrimsonCaravanMapMarker"))
            .ok_or("no map marker ref in the data")?;
        gs.map_markers.insert(marker);
        let rep = hb.collect(&mut gs, &mut host);
        if !rep.uncovered.is_empty() {
            return Err(format!(
                "a raw write with no effect: the backstop found {:?}",
                rep.uncovered
            ));
        }
    }
    if !hb.uncovered_total.is_empty() {
        return Err(format!("uncovered writes: {:?}", hb.uncovered_total));
    }
    Ok(format!(
        "{} real script lines on the host through Runner and HostBridge::collect (keyed-diff backstop on every call): {writes} store writes over {} domains ({:?}), 0 uncovered",
        STORY_LINES.len(),
        seen.len(),
        seen
    ))
}

// ---------------------------------------------------- core.join_service_codes

/// One code on the stand-in service.
struct Room {
    blob: String,
    secret_hash: u64,
    expires_ms: u64,
}

/// The stand-in keeps only a hash of the secret, like the worker (FNV-1a
/// here, SHA-256 there; no caller sees the hash).
fn secret_hash(s: &str) -> u64 {
    s.bytes().fold(0xcbf2_9ce4_8422_2325u64, |h, b| {
        (h ^ b as u64).wrapping_mul(0x0000_0100_0000_01b3)
    })
}

fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

struct StandRng(u64);

impl StandRng {
    fn new() -> StandRng {
        use std::hash::{BuildHasher, Hasher};
        let mut h = std::collections::hash_map::RandomState::new().build_hasher();
        h.write_u64(now_ms());
        StandRng(h.finish() | 1)
    }
    fn byte(&mut self) -> u8 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 >> 24) as u8
    }
}

/// `normalizeCode` of `services/join-service/src/code.ts`.
fn stand_normalize(input: &str) -> Option<String> {
    let c: String = input
        .chars()
        .filter(|c| !c.is_whitespace() && *c != '-')
        .collect::<String>()
        .to_uppercase();
    (c.chars().count() == CODE_LEN && c.chars().all(|ch| CODE_ALPHABET.contains(ch))).then_some(c)
}

/// `generateCode` of `code.ts`: rejection sampling over random bytes.
fn stand_code(rng: &mut StandRng) -> String {
    let a: Vec<char> = CODE_ALPHABET.chars().collect();
    let limit = 256 - (256 % a.len());
    let mut out = String::new();
    while out.len() < CODE_LEN {
        let b = rng.byte() as usize;
        if b < limit {
            out.push(a[b % a.len()]);
        }
    }
    out
}

fn err(status: u16, msg: &str) -> (u16, String) {
    (status, serde_json::json!({ "error": msg }).to_string())
}

/// `parseHostBody` of `validate.ts`: the blob and the clamped time to live.
fn stand_host_body(raw: &str) -> Result<(String, u64), String> {
    if raw.len() > 2048 {
        return Err("body too large".into());
    }
    let j: serde_json::Value = serde_json::from_str(raw).map_err(|_| "invalid json")?;
    let o = j.as_object().ok_or("invalid body")?;
    let blob = match o.get("blob").and_then(|b| b.as_str()) {
        Some(b) if !b.is_empty() && b.encode_utf16().count() <= 512 => b.to_string(),
        _ => return Err("blob must be a string of 1..512 chars".into()),
    };
    if blob.chars().any(|c| (c as u32) < 0x20 || c as u32 == 0x7f) {
        return Err("blob has control characters".into());
    }
    let ttl = match o.get("ttl_s") {
        None => 1800,
        Some(v) => {
            let t = v.as_f64().filter(|t| t.is_finite()).ok_or("bad ttl_s")?;
            t.floor().clamp(60.0, 21_600.0) as u64
        }
    };
    Ok((blob, ttl))
}

/// `handle` of `router.ts` (no rate limit: the stand-in never answers 429).
fn stand_route(
    rooms: &mut HashMap<String, Room>,
    rng: &mut StandRng,
    method: &str,
    path: &str,
    secret: Option<&str>,
    body: &str,
) -> (u16, String) {
    let parts: Vec<&str> = path
        .split('?')
        .next()
        .unwrap_or("")
        .split('/')
        .filter(|p| !p.is_empty())
        .collect();
    let m = method.to_ascii_uppercase();
    let bucket = match (m.as_str(), parts.as_slice()) {
        ("POST", ["host"]) => "host",
        ("GET", ["join", _]) => "join",
        ("DELETE", ["host", _]) => "delete",
        _ => return err(404, "not found"),
    };
    let now = now_ms();
    if bucket == "host" {
        let (blob, ttl) = match stand_host_body(body) {
            Ok(x) => x,
            Err(e) => return err(400, &e),
        };
        let secret: String = (0..24).map(|_| format!("{:02x}", rng.byte())).collect();
        for _ in 0..6 {
            let code = stand_code(rng);
            if rooms.get(&code).is_some_and(|r| r.expires_ms > now) {
                continue;
            }
            let exp = now + ttl * 1000;
            rooms.insert(
                code.clone(),
                Room {
                    blob,
                    secret_hash: secret_hash(&secret),
                    expires_ms: exp,
                },
            );
            return (
                201,
                serde_json::json!({ "code": code, "secret": secret, "expires_at": exp, "ttl_s": ttl })
                    .to_string(),
            );
        }
        return err(503, "no free code, retry");
    }
    let Some(code) = stand_normalize(parts[1]) else {
        return err(400, "bad code");
    };
    if rooms.get(&code).is_some_and(|r| r.expires_ms <= now) {
        rooms.remove(&code);
    }
    if bucket == "join" {
        return match rooms.get(&code) {
            Some(r) => (200, serde_json::json!({ "blob": r.blob }).to_string()),
            None => err(404, "unknown or expired code"),
        };
    }
    let Some(secret) = secret.filter(|s| !s.is_empty()) else {
        return err(401, "missing X-Host-Secret");
    };
    match rooms.get(&code) {
        None => err(404, "unknown or expired code"),
        Some(r) if r.secret_hash != secret_hash(secret) => err(403, "wrong secret"),
        Some(_) => {
            rooms.remove(&code);
            (200, serde_json::json!({ "ok": true }).to_string())
        }
    }
}

/// Reads one HTTP/1.1 request: method, path, lower-case headers, body.
fn read_request(s: &mut TcpStream) -> Option<(String, String, HashMap<String, String>, String)> {
    let mut buf = Vec::new();
    let mut chunk = [0u8; 2048];
    let head_end = loop {
        if let Some(i) = buf.windows(4).position(|w| w == b"\r\n\r\n") {
            break i;
        }
        let n = s.read(&mut chunk).ok()?;
        if n == 0 || buf.len() > 65_536 {
            return None;
        }
        buf.extend_from_slice(&chunk[..n]);
    };
    let head = String::from_utf8_lossy(&buf[..head_end]).to_string();
    let mut lines = head.split("\r\n");
    let mut first = lines.next()?.split_whitespace();
    let method = first.next()?.to_string();
    let path = first.next()?.to_string();
    let headers: HashMap<String, String> = lines
        .filter_map(|l| l.split_once(':'))
        .map(|(k, v)| (k.trim().to_ascii_lowercase(), v.trim().to_string()))
        .collect();
    let len: usize = headers
        .get("content-length")
        .and_then(|v| v.parse().ok())
        .unwrap_or(0);
    let mut body = buf[head_end + 4..].to_vec();
    while body.len() < len {
        let n = s.read(&mut chunk).ok()?;
        if n == 0 {
            break;
        }
        body.extend_from_slice(&chunk[..n]);
    }
    body.truncate(len);
    Some((
        method,
        path,
        headers,
        String::from_utf8_lossy(&body).to_string(),
    ))
}

fn reason(status: u16) -> &'static str {
    match status {
        200 => "OK",
        201 => "Created",
        400 => "Bad Request",
        401 => "Unauthorized",
        403 => "Forbidden",
        404 => "Not Found",
        _ => "Service Unavailable",
    }
}

/// A local stand-in of `services/join-service` on 127.0.0.1: the worker's
/// three calls, its code alphabet, body checks and secrets, over plain HTTP.
struct StandIn {
    addr: SocketAddr,
    stop: Arc<AtomicBool>,
    log: Arc<Mutex<Vec<String>>>,
    thread: Option<JoinHandle<()>>,
}

impl StandIn {
    fn start() -> Result<StandIn, String> {
        let l = TcpListener::bind("127.0.0.1:0").map_err(|e| format!("stand-in bind: {e}"))?;
        let addr = l.local_addr().map_err(|e| e.to_string())?;
        let stop = Arc::new(AtomicBool::new(false));
        let log = Arc::new(Mutex::new(Vec::new()));
        let (st, lg) = (stop.clone(), log.clone());
        let thread = std::thread::spawn(move || {
            let mut rooms: HashMap<String, Room> = HashMap::new();
            let mut rng = StandRng::new();
            for conn in l.incoming() {
                if st.load(Ordering::SeqCst) {
                    break;
                }
                let Ok(mut s) = conn else { continue };
                let _ = s.set_read_timeout(Some(Duration::from_secs(5)));
                let Some((method, path, headers, body)) = read_request(&mut s) else {
                    continue;
                };
                let secret = headers.get("x-host-secret").map(String::as_str);
                let (status, out) =
                    stand_route(&mut rooms, &mut rng, &method, &path, secret, &body);
                if let Ok(mut v) = lg.lock() {
                    v.push(format!("{method} {path} {status}"));
                }
                let resp = format!(
                    "HTTP/1.1 {status} {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{out}",
                    reason(status),
                    out.len()
                );
                let _ = s.write_all(resp.as_bytes());
                let _ = s.flush();
            }
        });
        Ok(StandIn {
            addr,
            stop,
            log,
            thread: Some(thread),
        })
    }

    fn url(&self) -> String {
        format!("http://{}", self.addr)
    }

    fn calls(&self) -> Vec<String> {
        self.log.lock().map(|v| v.clone()).unwrap_or_default()
    }
}

impl Drop for StandIn {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::SeqCst);
        let _ = TcpStream::connect_timeout(&self.addr, Duration::from_secs(1));
        if let Some(t) = self.thread.take() {
            let _ = t.join();
        }
    }
}

fn free_udp_port() -> Result<u16, String> {
    let s = UdpSocket::bind(("0.0.0.0", 0)).map_err(|e| format!("no free UDP port: {e}"))?;
    s.local_addr().map(|a| a.port()).map_err(|e| e.to_string())
}

/// The code as a person might type it: lower case with a dash.
fn typed(code: &str) -> String {
    let c = code.to_ascii_lowercase();
    format!("{}-{}", &c[..3], &c[3..])
}

/// The code with its first symbol changed to another symbol of the alphabet.
fn one_symbol_off(code: &str) -> String {
    let first = code.chars().next().unwrap_or('A');
    let other = CODE_ALPHABET.chars().find(|c| *c != first).unwrap_or('B');
    format!("{other}{}", &code[1..])
}

/// `core.join_service_codes`, end to end against a local stand-in of the join
/// service: the host's link publishes its `NVRS-` code and gets a short code,
/// a guest types the short code, the game resolves it and joins over UDP, and
/// the launcher reads and resolves it the same way. Removing the code needs
/// its secret. Mutation: the guest types the code with one symbol changed.
fn net_join_service_resolve(_env: &Env, mutate: bool) -> Result<String, String> {
    let svc = StandIn::start()?;
    let url = svc.url();
    let port = free_udp_port()?;
    let mut hreq = LinkRequest::new(Role::Host, "Host", &port.to_string());
    hreq.join_service = Some(url.clone());
    let ho = link::open(&hreq).map_err(|e| format!("host link: {e}"))?;
    let join_code = ho
        .join_code
        .clone()
        .ok_or("the host link has no join code")?;
    let hosted: Hosted = ho.short_code.clone().ok_or_else(|| {
        format!(
            "the host got no short code: {:?}",
            ho.notices
                .iter()
                .find(|n| n.ev == "coop_short_code_failed")
                .and_then(|n| n.get("reason"))
        )
    })?;
    if !ho
        .notices
        .iter()
        .any(|n| n.ev == "coop_short_code" && n.get("code") == Some(hosted.code.as_str()))
    {
        return Err("the host logged no coop_short_code notice".into());
    }
    if ho
        .notices
        .iter()
        .any(|n| n.fields.iter().any(|(_, v)| v.contains(&hosted.secret)))
    {
        return Err("a notice carries the host secret".into());
    }
    let mut host = Session::host(SessionConfig {
        name: "Host".into(),
        session_id: 0xC0DE,
        transport: Some(ho.transport),
        ..SessionConfig::default()
    })
    .map_err(|e| format!("host session: {e}"))?;
    // the guest types the short code
    let entered = if mutate {
        one_symbol_off(&hosted.code)
    } else {
        typed(&hosted.code)
    };
    let mut greq = LinkRequest::new(Role::Guest, "Guest", &entered);
    greq.join_service = Some(url.clone());
    let go = link::open(&greq)
        .map_err(|e| format!("the guest could not join with the code {entered:?}: {e}"))?;
    if !go
        .notices
        .iter()
        .any(|n| n.ev == "coop_short_code_resolved")
    {
        return Err("the guest link logged no coop_short_code_resolved notice".into());
    }
    let mut guest = Session::join(SessionConfig {
        name: "Guest".into(),
        transport: Some(go.transport),
        ..SessionConfig::default()
    })
    .map_err(|e| format!("guest session: {e}"))?;
    let t0 = Instant::now();
    while !guest.is_connected() {
        if t0.elapsed() > Duration::from_secs(10) {
            return Err(format!(
                "the guest resolved {entered} but did not connect in 10 s: {:?}",
                guest.status().state
            ));
        }
        host.poll(0.01);
        guest.poll(0.01);
        std::thread::sleep(Duration::from_millis(5));
    }
    let joined_ms = t0.elapsed().as_millis();
    // the launcher reads the short code as a code (never as host K7M3XW:27961)
    match joincode::parse_join(&hosted.code) {
        Ok(JoinInput::Short(c)) if c == hosted.code => {}
        other => return Err(format!("the launcher read {} as {other:?}", hosted.code)),
    }
    match joincode::parse_join("K7M3XW") {
        Ok(JoinInput::Short(c)) if c == "K7M3XW" => {}
        other => return Err(format!("the launcher read K7M3XW as {other:?}")),
    }
    match joincode::resolve_short(&entered, &JoinService::at(&url)) {
        Ok(JoinInput::Code { text, .. }) if text == join_code => {}
        other => {
            return Err(format!(
                "the launcher resolved {entered} to {other:?}, want {join_code}"
            ))
        }
    }
    // curl, the launcher's and the viewer's HTTPS transport, against the same contract
    let curl = CurlHttp::default();
    let curl_note = if curl.exe.is_file() {
        let c = JoinService::new(&url, Box::new(curl));
        let h = c.host("NVRS-curl-check", Some(5))?;
        if h.ttl_s != 60 {
            return Err(format!("the ttl 5 came back as {} (want 60)", h.ttl_s));
        }
        if c.resolve(&h.code)? != "NVRS-curl-check" {
            return Err("curl: the resolved blob differs".into());
        }
        c.remove(&h.code, &h.secret)?;
        "curl.exe speaks the same contract (host, resolve, remove)"
    } else {
        "curl.exe not found: the curl transport was not run"
    };
    // the body checks of the worker
    if JoinService::at(&url).host("bad\u{1}blob", None).is_ok() {
        return Err("the service took a blob with a control character".into());
    }
    // removing the code needs its secret
    let wrong = Hosted {
        secret: "0".repeat(48),
        ..hosted.clone()
    };
    match link::remove_short_code(&url, &wrong) {
        Err(e) if e.contains("403") => {}
        other => return Err(format!("a wrong secret removed the code: {other:?}")),
    }
    link::remove_short_code(&url, &hosted)?;
    match JoinService::at(&url).resolve(&hosted.code) {
        Err(e) if e.contains("unknown or has expired") => {}
        other => return Err(format!("a removed code still resolves: {other:?}")),
    }
    let calls = svc.calls();
    Ok(format!(
        "MODELLED (stand-in: a local Rust stand-in of services/join-service on 127.0.0.1; the Cloudflare worker is not called): the host link published {} and got the short code {}; the guest typed {entered:?}, the game resolved it and joined over UDP in {joined_ms} ms; the launcher reads K7M3XW as a short code and resolves {entered} to the host's code; a wrong secret gets 403, the right one removes the code, then it is unknown; {curl_note}; {} service calls",
        join_code,
        hosted.code,
        calls.len()
    ))
}

// ------------------------------------------------------------ harness.recorder

/// A small two-peer scenario; `extra` adds host lines.
fn rec_scenario(name: &str, extra: &str, duration: u32) -> String {
    format!(
        r#"{{"schema":1,"name":"{name}","cell":"GSDocMitchellHouse","network":"perfect","seed":5,"duration_s":{duration},"settle_s":2,"timeout_s":60,
"description":"recorder check","expect":[{{"id":"stims","path":"items.host.Stimpak","op":"eq","value":3,"on":"both"}}],
"host":{{"script":[{{"at":1,"run":"player.AddItem Stimpak 3"}}{extra}]}},"guest":{{"script":[{{"at":2,"run":"coop say hello"}}]}}}}"#
    )
}

fn recorded(text: &str, name: &str, d: &Arc<Data>, record: bool) -> Result<sim::RunResult, String> {
    let scn = Scenario::parse(text, name)?;
    let r = sim::run(
        &scn,
        d,
        &RunOpts {
            record,
            ..Default::default()
        },
    );
    if r.verdict.result != "pass" {
        return Err(format!(
            "{name}: the run itself failed: {} {}",
            r.verdict.result, r.verdict.reason
        ));
    }
    Ok(r)
}

/// The recording's tracks against the final dumps.
fn check_tracks(rec: &Recording, finals: &[&str], duration: u32) -> Result<usize, String> {
    let mut frames = 0;
    for (i, fin) in finals.iter().enumerate() {
        let tr = rec
            .peers
            .get(i)
            .ok_or_else(|| format!("the recording has {} peers", rec.peers.len()))?;
        if tr.frames.len() + 2 < duration as usize {
            return Err(format!(
                "{}: {} frames for a {duration} s run",
                tr.name,
                tr.frames.len()
            ));
        }
        if tr.frames.windows(2).any(|w| w[0].second() >= w[1].second()) {
            return Err(format!("{}: two frames for one second", tr.name));
        }
        for f in &tr.frames {
            let h = J::parse(&f.dump)?
                .path("hash.shared")
                .and_then(J::as_str)
                .map(str::to_string)
                .unwrap_or_default();
            if h != f.hash || h.is_empty() {
                return Err(format!(
                    "{} second {}: frame hash {} but dump hash {h}",
                    tr.name,
                    f.second(),
                    f.hash
                ));
            }
        }
        let last = tr.frames.last().ok_or("no frames")?;
        if J::parse(&last.dump)? != J::parse(fin)? {
            return Err(format!("{}: the last frame is not the final dump", tr.name));
        }
        frames += tr.frames.len();
    }
    Ok(frames)
}

/// `harness.recorder` (the flight recorder): a recorded run keeps one dump
/// and shared hash per peer and virtual second, its last frames are the final
/// dumps, `flight.jsonl` reads back to the same recording, and a 3-peer run
/// records every peer. Mutation: the run is made with the recorder off.
fn harness_recorder(env: &Env, mutate: bool) -> Result<String, String> {
    let d = open_data(env)?;
    let text = rec_scenario("chk_rec", "", 8);
    let scn = Scenario::parse(&text, "chk_rec")?;
    let out = scratch("recorder", mutate)?;
    let r = sim::run(
        &scn,
        &d,
        &RunOpts {
            record: !mutate,
            out: Some(out.clone()),
            ..Default::default()
        },
    );
    let rec = r.recording.ok_or("the run made no flight recording")?;
    let frames = check_tracks(&rec, &[&r.dumps[0], &r.dumps[1]], 8)?;
    let file = out.join("chk_rec").join("seed5").join("flight.jsonl");
    let back = std::fs::read_to_string(&file)
        .map_err(|e| format!("{}: {e}", file.display()))
        .and_then(|t| Recording::from_jsonl(&t))?;
    if back != rec {
        return Err("flight.jsonl does not read back to the recording".into());
    }
    // three peers through the multi-peer runner
    let three = r#"{"schema":1,"name":"chk_rec3","cell":"GSDocMitchellHouse","network":"perfect","seed":5,"duration_s":8,"settle_s":2,"timeout_s":60,
"description":"recorder check, 3 peers","expect":[{"id":"h","path":"hash.shared","on":"equal"}],
"host":{"script":[{"at":1,"run":"player.AddItem Stimpak 3"}]},"guests":[{"script":[]},{"script":[]}]}"#;
    let s3 = Scenario::parse(three, "chk_rec3")?;
    let r3 = crate::multi::run_multi(
        &s3,
        &d,
        &crate::multi::MultiOpts {
            run: RunOpts {
                record: true,
                ..Default::default()
            },
        },
    );
    let rec3 = r3.recording.ok_or("the 3-peer run made no recording")?;
    let names: Vec<&str> = rec3.peers.iter().map(|p| p.name.as_str()).collect();
    if names != ["host", "guest", "guest2"] {
        return Err(format!("the 3-peer recording has the peers {names:?}"));
    }
    let finals: Vec<&str> = r3.dumps.iter().map(String::as_str).collect();
    let frames3 = check_tracks(&rec3, &finals, 8)?;
    Ok(format!(
        "two peers: {frames} frames (one per peer and second, hash = dump hash, last = final dump), flight.jsonl reads back equal; three peers: {frames3} frames for {names:?}"
    ))
}

/// `harness.recorder` (`diverge`): two recorded runs of one scenario, the
/// second with a planted host line at 5 s, first differ at the second the
/// planted item reaches the host's dump, at `items.host.Caps001`. A clean run
/// has no host-guest divergence past the lag, and a guest track forged to a
/// different hash from second 4 is found at second 4 with its path. Mutation:
/// the second run has no planted line.
fn harness_diverge(env: &Env, mutate: bool) -> Result<String, String> {
    let d = open_data(env)?;
    let a = recorded(&rec_scenario("chk_div", "", 10), "chk_div", &d, true)?;
    let plant = if mutate {
        ""
    } else {
        r#",{"at":5,"run":"player.AddItem Caps001 25"}"#
    };
    let b = recorded(&rec_scenario("chk_div", plant, 10), "chk_div", &d, true)?;
    let (ra, rb) = (
        a.recording.ok_or("no recording")?,
        b.recording.ok_or("no recording")?,
    );
    let dv = recorder::diverge_runs(&ra, &rb)
        .ok_or("diverge found no difference between the two runs")?;
    let first_caps = rb.peers[0]
        .frames
        .iter()
        .find(|f| {
            J::parse(&f.dump)
                .ok()
                .and_then(|j| j.path("items.host.Caps001").cloned())
                .is_some()
        })
        .map(|f| f.second())
        .ok_or("the planted Caps001 is in no frame of the second run")?;
    if dv.second != first_caps || dv.a != "host" {
        return Err(format!(
            "diverge said second {} on {}, but the planted item reached the host at second {first_caps}",
            dv.second, dv.a
        ));
    }
    if !dv.paths.iter().any(|p| p == "items.host.Caps001") {
        return Err(format!(
            "diverge did not name items.host.Caps001: {:?}",
            dv.paths
        ));
    }
    for (name, rec) in [("first", &ra), ("second", &rb)] {
        if let Some(x) = recorder::diverge_peers(rec, 2.0) {
            return Err(format!(
                "the {name} run is clean, but diverge reported {}",
                x.line()
            ));
        }
    }
    // a guest that holds a different shared state from second 4 on
    let mut forged = ra.clone();
    for f in forged.peers[1]
        .frames
        .iter_mut()
        .filter(|f| f.second() >= 4)
    {
        let mut j = J::parse(&f.dump)?;
        if let J::Obj(m) = &mut j {
            if let Some(J::Obj(g)) = m.get_mut("globals") {
                g.insert("Planted".into(), J::Num(7.0));
            } else {
                m.insert(
                    "globals".into(),
                    J::Obj(BTreeMap::from([("Planted".to_string(), J::Num(7.0))])),
                );
            }
        }
        f.dump = j.dump();
        f.hash = "0bad0bad".into();
    }
    let fx = recorder::diverge_peers(&forged, 2.0).ok_or("diverge missed a forged guest")?;
    if fx.second != 4 || !fx.paths.iter().any(|p| p == "globals.Planted") {
        return Err(format!("the forged guest was found as: {}", fx.line()));
    }
    Ok(format!(
        "run against run: {}; clean runs have no host-guest divergence past a 2 s lag; a forged guest: {}",
        dv.line(),
        fx.line()
    ))
}

/// `harness.recorder` (`shrink`): a failing 5-line scenario (one planted line
/// gives the host too many Stimpaks) shrinks to the one planted line, and the
/// small scenario still fails the same way. Mutation: the planted line is
/// harmless, so nothing fails and shrink has nothing to do.
fn harness_shrink(env: &Env, mutate: bool) -> Result<String, String> {
    let d = open_data(env)?;
    let plant = if mutate { 2 } else { 9 };
    let text = format!(
        r#"{{"schema":1,"name":"chk_shrink","cell":"GSDocMitchellHouse","network":"perfect","seed":5,"duration_s":7,"settle_s":2,"timeout_s":60,
"description":"shrink check","expect":[{{"id":"few-stims","path":"items.host.Stimpak","op":"le","value":3,"on":"host"}}],
"host":{{"script":[{{"at":1,"run":"player.AddItem Caps001 5"}},{{"at":2,"run":"player.AddItem Stimpak 1"}},{{"at":3,"run":"player.AddItem Stimpak {plant}"}},{{"at":4,"run":"player.AddItem WaterPurified 2"}}]}},
"guest":{{"script":[{{"at":2,"run":"coop say hello"}}]}}}}"#
    );
    let s = recorder::shrink(&text, "chk_shrink", &d, &RunOpts::default())?;
    let small = J::parse(&s.text)?;
    let host_lines = small
        .path("host.script")
        .and_then(J::as_arr)
        .map(<[J]>::to_vec)
        .unwrap_or_default();
    let guest_lines = small
        .path("guest.script")
        .and_then(J::as_arr)
        .map_or(0, |a| a.len());
    let only = host_lines
        .first()
        .and_then(|l| l.get("run"))
        .and_then(J::as_str)
        .unwrap_or("");
    if s.before != 5 || s.after != 1 || host_lines.len() != 1 || guest_lines != 0 {
        return Err(format!(
            "shrink kept {} of {} lines ({} host, {guest_lines} guest)",
            s.after,
            s.before,
            host_lines.len()
        ));
    }
    if only != format!("player.AddItem Stimpak {plant}") {
        return Err(format!("shrink kept the wrong line: {only}"));
    }
    let scn = Scenario::parse(&s.text, "chk_shrink")?;
    let r = sim::run(&scn, &d, &RunOpts::default());
    if r.verdict.result != "fail" || !r.verdict.reason.contains("few-stims") {
        return Err(format!(
            "the shrunk scenario does not fail the same way: {} {}",
            r.verdict.result, r.verdict.reason
        ));
    }
    Ok(format!(
        "shrank {} script lines to {} in {} runs (kept `{only}`), still failing as {}",
        s.before, s.after, s.runs, s.failure
    ))
}

/// `harness.recorder` (`gentest`): scenarios generated from the bot verbs are
/// the same for one seed, differ between seeds, parse, and run green on their
/// own network. Mutation: one line of each generated scenario gets a verb the
/// bot does not know.
fn harness_gentest(env: &Env, mutate: bool) -> Result<String, String> {
    let d = open_data(env)?;
    let first = env.seed * 10;
    let mut texts: Vec<String> = Vec::new();
    let mut shown = Vec::new();
    for seed in first..first + 3 {
        let mut text = recorder::gentest(seed);
        if text != recorder::gentest(seed) {
            return Err(format!("gentest seed {seed} gave two different scenarios"));
        }
        if mutate {
            text = text.replacen("\"run\": \"", "\"run\": \"coop zzzverb ", 1);
        }
        let name = format!("gen_{seed}");
        let scn = Scenario::parse(&text, &name)?;
        let r = sim::run(&scn, &d, &RunOpts::default());
        if r.verdict.result != "pass" {
            return Err(format!(
                "{name} ({} guests, {}): {} {}",
                scn.guest_count(),
                scn.network,
                r.verdict.result,
                r.verdict.reason
            ));
        }
        shown.push(format!(
            "{name}: guests {} on {}",
            scn.guest_count(),
            scn.network
        ));
        texts.push(text);
    }
    if texts.windows(2).any(|w| w[0] == w[1]) {
        return Err("two seeds gave the same scenario".into());
    }
    Ok(format!(
        "3 seeds, deterministic, all different, all pass: {}",
        shown.join("; ")
    ))
}

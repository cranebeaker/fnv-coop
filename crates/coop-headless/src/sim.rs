//! One scenario run: two peers in one process on a virtual clock.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use coop_core::ids::PlayerId;
use coop_core::transport::{ChaosProfile, VirtualClock};
use coop_session::{Role, Session, SessionConfig};
use world::coop::apply::Ctx;

use crate::bot::is_bot_line;
use crate::data::Data;
use crate::json::J;
use crate::net::{advance_clock, set_cut, CtlRef, Link};
use crate::peer::Peer;
use crate::scenario::{Action, Scenario, Side};
use crate::verdict::{judge, Dir, PeerFacts, RelayFacts, RunFacts, Verdict};

/// Options of one run.
#[derive(Clone, Debug)]
pub struct RunOpts {
    /// Seed (`None`: the scenario's).
    pub seed: Option<u64>,
    /// Network profile override.
    pub net: Option<String>,
    /// Frames per virtual second.
    pub fps: u32,
    /// Where to write `verdict.json` and the logs.
    pub out: Option<PathBuf>,
    /// The scenario path, for the replay command.
    pub path: String,
    /// The `--data` argument to repeat in the replay command.
    pub data_arg: Option<String>,
    /// `--talk` waits for `coop_ready`. By default it fires on the first frame, as the viewer's does once its scene is loaded.
    pub talk_at_ready: bool,
    /// Container actions go through `Session::live_loot_ops` (the proposed viewer change); by default the viewer's own `intents::loot` is mirrored.
    pub fixed_glue: bool,
    /// The flight recorder: keep every peer's dump and shared hash once per
    /// virtual second ([`RunResult::recording`]; `flight.jsonl` with `out`).
    pub record: bool,
}

impl Default for RunOpts {
    fn default() -> Self {
        RunOpts {
            seed: None,
            net: None,
            fps: 40,
            out: None,
            path: "SCENARIO.json".into(),
            data_arg: None,
            talk_at_ready: false,
            fixed_glue: false,
            record: false,
        }
    }
}

/// A finished run.
pub struct RunResult {
    /// The judgement.
    pub verdict: Verdict,
    /// Hash of both final dumps (determinism check).
    pub hash: u64,
    /// Final dumps.
    pub dumps: [String; 2],
    /// The flight recording, when `RunOpts::record` was set.
    pub recording: Option<crate::recorder::Recording>,
}

fn fnv(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xcbf2_9ce4_8422_2325u64, |h, b| {
        (h ^ *b as u64).wrapping_mul(0x0000_0100_0000_01b3)
    })
}

/// The viewer flag that makes a guest skip its character making.
const NO_CHARGEN: &str = "--coop-no-chargen";

/// The viewer's host flags for the campaign rules (`--coop-difficulty normal|balanced`,
/// `--coop-friendly-fire on|off`). Each takes a value. The host session's config models them
/// (see [`rules_of`]).
const RULES_FLAGS: [&str; 2] = ["--coop-difficulty", "--coop-friendly-fire"];

/// The campaign rules a host's flags choose, the way the viewer reads them (`None`: no rules
/// flag).
pub(crate) fn rules_of(
    args: &[String],
) -> Result<Option<world::coop::party_save::PartyRules>, String> {
    use world::coop::party_save::PartyRules;
    let mut r: Option<PartyRules> = None;
    let mut it = args.iter();
    while let Some(a) = it.next() {
        if !RULES_FLAGS.contains(&a.as_str()) {
            continue;
        }
        let v = it.next().ok_or_else(|| format!("{a} needs a value"))?;
        let e = r.get_or_insert_with(PartyRules::default);
        if a == RULES_FLAGS[0] {
            e.mode = PartyRules::parse_mode(v)
                .ok_or_else(|| format!("{a} {v}: use normal or balanced"))?;
        } else {
            e.friendly_fire =
                PartyRules::parse_switch(v).ok_or_else(|| format!("{a} {v}: use on or off"))?;
        }
    }
    Ok(r)
}

/// A reason this scenario cannot run on this data (never guessed around).
pub fn needs_assets(scn: &Scenario, d: &Data) -> Option<String> {
    for side in [&scn.host, &scn.guest] {
        // modelled flags: `--walk`, a guest's `--coop-no-chargen` (see `build_peers`) and the
        // host's rules flags with their values (see `cfg`)
        let host = std::ptr::eq(side, &scn.host);
        let modelled = |a: &str| a == "--walk" || (a == NO_CHARGEN && !host);
        let mut args = side.extra_args.iter();
        while let Some(a) = args.next() {
            if host && RULES_FLAGS.contains(&a.as_str()) {
                args.next();
            } else if !modelled(a.as_str()) {
                return Some(format!("viewer flag {a} has no headless model"));
            }
        }
        if let Err(e) = rules_of(&side.extra_args) {
            return Some(format!("viewer flag {e}"));
        }
    }
    let cell = scn
        .cell
        .as_deref()
        .unwrap_or(coop_session::start::LOCAL_ROOM);
    if let Err(e) = d.place(cell) {
        return Some(format!(
            "{e} (this folder has {} plugins; run against the full Data folder)",
            d.order.plugins().len()
        ));
    }
    for side in [&scn.host, &scn.guest] {
        for l in &side.script {
            let Action::Run(r) = &l.action else { continue };
            let r = r.strip_prefix("local ").unwrap_or(r);
            let words: Vec<&str> = r.split_whitespace().collect();
            let mut names: Vec<&str> = Vec::new();
            if is_bot_line(r) {
                let verb = words
                    .get(1)
                    .map(|w| w.to_ascii_lowercase())
                    .unwrap_or_default();
                let refs = match verb.as_str() {
                    "hit" | "talk" | "pick" | "lootopen" | "lootclose" | "travel" | "tether"
                    | "cell" => 1,
                    "loottake" | "lootput" => 2,
                    _ => 0,
                };
                names.extend(words.iter().skip(2).take(refs));
            } else if let Some((who, _)) = words.first().and_then(|w| w.split_once('.')) {
                names.push(who);
            }
            if let Some(n) = names.iter().find(|n| d.form(n).is_none()) {
                return Some(format!(
                    "{n} is not in this load order (needs the full Data folder)"
                ));
            }
        }
    }
    None
}

/// People the scripts bring into the cell (the host tracks them once they stand there).
pub(crate) fn add_named_people(p: &mut crate::data::Place, scn: &Scenario, d: &Data) {
    for side in [&scn.host, &scn.guest] {
        for l in &side.script {
            let Action::Run(r) = &l.action else { continue };
            for w in r.split(|c: char| c.is_whitespace() || c == '.') {
                if let Some(base) = d.form(w).and_then(|f| d.actor_base(f).map(|b| (f, b))) {
                    if !p.actors.iter().any(|a| a.0 == base.0) {
                        p.actors.push(base);
                    }
                }
            }
        }
    }
}

fn peer_facts(p: &Peer, role: &str) -> PeerFacts {
    PeerFacts {
        role: role.into(),
        started: true,
        ready: p.ready_at.is_some(),
        shutdown_clean: true,
        errors: p.errors.clone(),
        repairs: p.repairs,
        events: p.events.len(),
        state: J::parse(p.dump_text()).ok(),
        tail: p.events.iter().rev().take(6).rev().cloned().collect(),
    }
}

fn dir_of(c: &crate::net::Ctl) -> Dir {
    Dir {
        inn: c.sent,
        forwarded: c.stats.released,
        dropped: c.stats.dropped + c.cut_drops,
        duplicated: c.stats.duplicated,
        reordered: c.stats.reordered,
    }
}

fn cfg(name: &str, t: crate::net::SimLink, side: &crate::scenario::Side) -> SessionConfig {
    SessionConfig {
        compat: coop_session::CompatConfig {
            exe: "1.4.0.525".into(),
            plugins: side.plugins.clone(),
            allow_mismatch: side.allow_mismatch,
            ..Default::default()
        },
        name: name.into(),
        session_id: 0xC00B,
        transport: Some(Box::new(t)),
        live: true,
        native_dialogue: true,
        // the viewer's `--coop-difficulty` and `--coop-friendly-fire` (host)
        rules: rules_of(&side.extra_args).ok().flatten(),
        ..SessionConfig::default()
    }
}

type Built = (Peer, Peer, CtlRef, CtlRef, VirtualClock);

/// A fresh link, two sessions and two peers; the party layer is attached when `party` is a
/// directory.
#[allow(clippy::too_many_arguments)]
fn build_peers(
    scn: &Scenario,
    d: &Arc<Data>,
    place: &crate::data::Place,
    profile: ChaosProfile,
    seed: u64,
    hs: &Side,
    gsd: &Side,
    party: Option<&Path>,
    key: u64,
    o: &RunOpts,
) -> Result<Built, String> {
    let link = Link::new(profile, seed);
    let Link {
        host: lh,
        guest: lg,
        h2g,
        g2h,
        clock,
    } = link;
    let (mut hsn, mut gsn) = match (
        Session::host(cfg("Host", lh, hs)),
        Session::join(cfg("Guest 1", lg, gsd)),
    ) {
        (Ok(h), Ok(g)) => (h, g),
        (Err(e), _) | (_, Err(e)) => return Err(format!("session: {e}")),
    };
    hsn.set_start_config(coop_session::StartConfig {
        new_game: scn.new_game,
        ..Default::default()
    });
    // the viewer's `--coop-no-chargen` (test harness only): the guest skips making its character
    let own = if scn.guest.extra_args.iter().any(|a| a == NO_CHARGEN) {
        coop_session::OwnCharacter::Skip
    } else if scn.new_game || scn.character_creation {
        coop_session::OwnCharacter::Create
    } else {
        coop_session::OwnCharacter::Skip
    };
    gsn.set_start_config(coop_session::StartConfig {
        new_game: false,
        own,
    });
    // `host_cell`: the host starts in another cell than the guest (late join into another scene)
    let mut host_place = place.clone();
    if let Some(c) = scn.host_cell.as_deref().and_then(|c| d.form(c)) {
        host_place.cell = c;
        host_place.actors.clear();
    }
    let mut host = Peer::new(
        Role::Host,
        hsn,
        d,
        &host_place,
        hs,
        !o.talk_at_ready,
        o.fixed_glue,
    );
    let mut guest = Peer::new(
        Role::Guest,
        gsn,
        d,
        place,
        gsd,
        !o.talk_at_ready,
        o.fixed_glue,
    );
    if let Some(dir) = party {
        crate::party::attach(&mut host, key, dir);
        crate::party::attach(&mut guest, key, dir);
    }
    Ok((host, guest, h2g, g2h, clock))
}

/// Runs one scenario to a verdict.
pub fn run(scn: &Scenario, d: &Arc<Data>, o: &RunOpts) -> RunResult {
    // a `budget` key needs the counted links of the multi-peer runner
    if scn.guest_count() > 1 || scn.budget.is_some() {
        return crate::multi::run_as_result(scn, d, o);
    }
    let seed = o.seed.unwrap_or(scn.seed);
    let mut scn = scn.clone();
    if let Some(n) = &o.net {
        scn.network = n.clone();
    }
    let rerun = format!(
        "coop-sim run {} --seed {}{}{}",
        o.path,
        seed,
        o.net
            .as_ref()
            .map(|n| format!(" --net {n}"))
            .unwrap_or_default(),
        o.data_arg
            .as_ref()
            .map(|n| format!(" --data {n}"))
            .unwrap_or_default()
    );
    let mut facts = RunFacts {
        rerun,
        seed,
        run_id: format!("seed{seed}"),
        ..RunFacts::default()
    };
    facts.relay.profile = scn.network.clone();
    let finish = |facts: RunFacts, dumps: [String; 2]| {
        let verdict = judge(&scn, &facts);
        let hash = fnv(dumps[0].as_bytes()) ^ fnv(dumps[1].as_bytes()).rotate_left(1);
        RunResult {
            verdict,
            hash,
            dumps,
            recording: None,
        }
    };
    let t_prof = std::time::Instant::now();
    let prof = std::env::var_os("COOP_SIM_PROF").is_some();
    if let Some(why) = needs_assets(&scn, d) {
        facts.needs_assets = Some(why);
        return finish(facts, [String::new(), String::new()]);
    }
    if prof {
        eprintln!("prof: needs_assets {:.2} s", t_prof.elapsed().as_secs_f64());
    }
    let cell_name = scn
        .cell
        .clone()
        .unwrap_or_else(|| coop_session::start::LOCAL_ROOM.to_string());
    let place = match d.place(&cell_name) {
        Ok(mut p) => {
            add_named_people(&mut p, &scn, d);
            p
        }
        Err(e) => {
            facts.harness_error = Some(e);
            return finish(facts, [String::new(), String::new()]);
        }
    };
    if prof {
        eprintln!(
            "prof: place {:.2} s, {} people {:?}",
            t_prof.elapsed().as_secs_f64(),
            place.actors.len(),
            place.actors.iter().map(|a| a.0).collect::<Vec<_>>()
        );
    }
    let profile = ChaosProfile::by_name(&scn.network).unwrap_or(ChaosProfile::PERFECT);
    let party_dir: Option<PathBuf> = scn.party.as_ref().map(|_| {
        let dir = std::env::temp_dir().join(format!(
            "coop-sim-party-{}-{}-{}",
            std::process::id(),
            scn.name,
            seed
        ));
        let _ = std::fs::remove_dir_all(&dir);
        let _ = std::fs::create_dir_all(&dir);
        dir
    });
    let key = scn.party.as_ref().map_or(0, |p| p.guest_key);
    let (mut host, mut guest, mut h2g, mut g2h, mut clock) = match build_peers(
        &scn,
        d,
        &place,
        profile,
        seed,
        &scn.host,
        &scn.guest,
        party_dir.as_deref(),
        key,
        o,
    ) {
        Ok(x) => x,
        Err(e) => {
            facts.harness_error = Some(e);
            return finish(facts, [String::new(), String::new()]);
        }
    };
    if o.record {
        host.flight = Some(Vec::new());
        guest.flight = Some(Vec::new());
    }
    let mut restarted = false;
    let _ = PlayerId::HOST;
    let cx = Ctx {
        order: &d.order,
        scripts: &d.scripts,
    };
    let step_ms = (1000 / o.fps.max(1)) as u64;
    let dt = step_ms as f32 / 1000.0;
    let launch_limit_s = 120.0;
    let mut now_ms = 0u64;
    let mut armed_at: Option<f64> = None;
    let mut end_s: Option<f64> = None;
    let mut fired = vec![false; scn.outages.len()];
    let mut healed = vec![false; scn.outages.len()];
    let mut switched = scn.network == "perfect";
    loop {
        let t = now_ms as f64 / 1000.0;
        advance_clock(&clock, now_ms);
        if armed_at.is_none() && g2h.borrow().sent > 0 {
            armed_at = Some(t);
        }
        if let Some(a) = armed_at {
            for (i, o) in scn.outages.iter().enumerate() {
                if !fired[i] && t >= a + o.at {
                    fired[i] = true;
                    set_cut(&h2g, &g2h, &o.dir, true);
                }
                if fired[i] && !healed[i] && t >= a + o.at + o.seconds {
                    healed[i] = true;
                    set_cut(&h2g, &g2h, &o.dir, false);
                }
            }
            if !switched && t >= a + scn.chaos_seconds() {
                switched = true;
                h2g.borrow_mut().new_profile = Some(ChaosProfile::PERFECT);
                g2h.borrow_mut().new_profile = Some(ChaosProfile::PERFECT);
            }
        }
        host.frame(t, dt, d, cx);
        guest.frame(t, dt, d, cx);
        if end_s.is_none() && scn.expect_refused && guest.counters.compat_refused > 0 {
            end_s = Some(t + 2.0 + scn.settle_s);
        }
        // "drop everything": both peers are replaced by new ones that start from the party save
        if let (Some(pc), Some(dir), false) = (&scn.party, &party_dir, restarted) {
            if let (Some(at), Some(a), Some(b)) = (pc.restart_at, host.ready_at, guest.ready_at) {
                if t >= a.max(b) + at {
                    restarted = true;
                    let key2 = if pc.guest_same_key {
                        pc.guest_key
                    } else {
                        pc.guest_key + 1
                    };
                    match build_peers(
                        &scn,
                        d,
                        &place,
                        profile,
                        seed ^ 0x5EC0,
                        &pc.second_host,
                        &pc.second_guest,
                        Some(dir),
                        key2,
                        o,
                    ) {
                        Ok((h, g, a2, b2, c2)) => {
                            // the flight recording goes on across the restart
                            let flights = (host.flight.take(), guest.flight.take());
                            (host, guest, h2g, g2h, clock) = (h, g, a2, b2, c2);
                            (host.flight, guest.flight) = flights;
                            if pc.host_loads {
                                if let Err(e) = crate::party::restore_host(&mut host, d) {
                                    facts.harness_error = Some(format!("party load: {e}"));
                                    break;
                                }
                            }
                            if pc.guest_offers_own {
                                if let Some(f) = crate::party::guest_file(dir) {
                                    guest.sess.party_guest_offer(f);
                                }
                            }
                            armed_at = None;
                            end_s = None;
                            switched = scn.network == "perfect";
                        }
                        Err(e) => {
                            facts.harness_error = Some(e);
                            break;
                        }
                    }
                }
            }
        }
        let restart_pending =
            scn.party.as_ref().is_some_and(|pc| pc.restart_at.is_some()) && !restarted;
        if end_s.is_none() && !restart_pending {
            if let (Some(a), Some(b)) = (host.ready_at, guest.ready_at) {
                let dur = match (&scn.party, restarted) {
                    (Some(pc), true) => pc.second_duration_s,
                    _ => scn.duration_s,
                };
                end_s = Some(a.max(b) + dur + scn.settle_s);
            }
        }
        now_ms += step_ms;
        let t_next = now_ms as f64 / 1000.0;
        if t_next >= end_s.unwrap_or(launch_limit_s) {
            break;
        }
    }
    if prof {
        eprintln!("prof: loop {:.2} s", t_prof.elapsed().as_secs_f64());
    }
    let t = now_ms as f64 / 1000.0;
    host.finish(t, d);
    guest.finish(t, d);
    facts.host = peer_facts(&host, "host");
    facts.guest = peer_facts(&guest, "guest");
    if scn.expect_refused {
        // a refusal is the end state: it is logged as an error by the guest, and neither peer reaches ready
        facts.host.ready = true;
        facts.guest.ready = true;
        facts
            .guest
            .errors
            .retain(|e| !e.contains("refused the connection"));
    }
    facts.relay = RelayFacts {
        profile: scn.network.clone(),
        armed: armed_at.is_some(),
        g2h: dir_of(&g2h.borrow()),
        h2g: dir_of(&h2g.borrow()),
        outages_fired: fired.iter().all(|f| *f),
    };
    facts.duration_s = t;
    let mut modelled: Vec<String> = host.modelled.union(&guest.modelled).cloned().collect();
    if scn
        .host
        .script
        .iter()
        .chain(scn.guest.script.iter())
        .any(|l| matches!(l.action, Action::Key(..)))
    {
        modelled.push("movement: keys move the player 150 units a second in its facing direction, no collision".into());
    }
    facts.modelled = modelled;
    if let Some(dir) = &o.out {
        write_out(dir, &scn, &facts, &host, &guest, d);
    }
    let dumps = [host.dump_text().to_string(), guest.dump_text().to_string()];
    if let Some(dir) = &party_dir {
        let _ = std::fs::remove_dir_all(dir);
    }
    let mut r = finish(facts, dumps);
    if o.record {
        let rec = crate::recorder::Recording::from_peers(
            &scn.name,
            seed,
            &scn.network,
            &[
                (
                    "host".to_string(),
                    host.ready_at,
                    host.flight.as_deref().unwrap_or(&[]),
                ),
                (
                    "guest".to_string(),
                    guest.ready_at,
                    guest.flight.as_deref().unwrap_or(&[]),
                ),
            ],
        );
        if let Some(dir) = &o.out {
            let _ = rec.save(&run_dir(dir, &scn, seed));
        }
        r.recording = Some(rec);
    }
    if let Some(dir) = &o.out {
        let _ = std::fs::write(
            run_dir(dir, &scn, seed).join("verdict.json"),
            r.verdict.json.dump(),
        );
    }
    r
}

fn run_dir(dir: &Path, scn: &Scenario, seed: u64) -> PathBuf {
    dir.join(&scn.name).join(format!("seed{seed}"))
}

fn write_out(dir: &Path, scn: &Scenario, facts: &RunFacts, host: &Peer, guest: &Peer, _d: &Data) {
    let rd = run_dir(dir, scn, facts.seed);
    if std::fs::create_dir_all(&rd).is_err() {
        return;
    }
    for (name, p) in [("host", host), ("guest", guest)] {
        let log: String = p.events.iter().map(|e| e.dump() + "\n").collect();
        let _ = std::fs::write(rd.join(format!("{name}.jsonl")), log);
        let _ = std::fs::write(rd.join(format!("{name}.state.json")), p.dump_text());
    }
}

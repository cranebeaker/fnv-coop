//! Tools to find the first divergence of a run: the flight recorder, `diverge`,
//! `shrink` and `gentest` (`coop-sim diverge|shrink|gentest`).
//!
//! * The flight recorder (`RunOpts::record`) keeps every peer's state dump and
//!   its shared-state hash once per virtual second (the dump cadence of
//!   [`crate::peer::Peer`]). A run with `--out` writes it as `flight.jsonl`.
//! * [`diverge_runs`] finds the first second at which two runs (the same seed,
//!   or a run and a changed copy) differ, with the dump paths that differ.
//!   [`diverge_peers`] finds the first second at which the host and a guest of
//!   one run hold different shared hashes and still do `lag_s` seconds later
//!   (a guest is always a moment behind its host).
//! * [`shrink`] removes script lines one by one from a failing scenario while
//!   it still fails the same way, and returns the smallest failing scenario.
//! * [`gentest`] makes a random valid scenario for a seed from the bot verbs.

use std::path::Path;
use std::sync::Arc;

use crate::data::Data;
use crate::json::{n, obj, s, J};
use crate::scenario::Scenario;
use crate::sim::{run, RunOpts};

/// One recorded second of one peer.
#[derive(Clone, Debug, PartialEq)]
pub struct Frame {
    /// Virtual time of the dump.
    pub t: f64,
    /// `hash.shared` of the dump.
    pub hash: String,
    /// The dump text.
    pub dump: String,
}

impl Frame {
    /// The whole second this frame stands for.
    pub fn second(&self) -> u64 {
        (self.t + 1e-6).floor().max(0.0) as u64
    }
}

/// One peer's recorded seconds.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Track {
    /// `host`, `guest`, `guest2`, ...
    pub name: String,
    /// `coop_ready` time.
    pub ready_at: Option<f64>,
    /// The dumps, oldest first.
    pub frames: Vec<Frame>,
}

/// A flight recording of one run.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Recording {
    /// Scenario name.
    pub scenario: String,
    /// Seed.
    pub seed: u64,
    /// Network profile.
    pub net: String,
    /// Host first.
    pub peers: Vec<Track>,
}

fn hash_of(dump: &str) -> String {
    J::parse(dump)
        .ok()
        .and_then(|j| {
            j.path("hash.shared")
                .and_then(J::as_str)
                .map(str::to_string)
        })
        .unwrap_or_default()
}

/// A dump in one canonical text (keys sorted, compact), so that a recording
/// read back from a file compares equal to the one in memory.
fn canon(dump: &str) -> String {
    J::parse(dump.trim())
        .map(|j| j.dump())
        .unwrap_or_else(|_| dump.trim().to_string())
}

/// One peer's flight log: its name, its `coop_ready` time, and its dumps with
/// their virtual times.
pub type FlightLog<'a> = (String, Option<f64>, &'a [(f64, String)]);

impl Recording {
    /// Builds a recording from the peers' flight logs.
    pub fn from_peers(scenario: &str, seed: u64, net: &str, peers: &[FlightLog]) -> Recording {
        Recording {
            scenario: scenario.into(),
            seed,
            net: net.into(),
            peers: peers
                .iter()
                .map(|(name, ready, log)| {
                    let mut frames: Vec<Frame> = Vec::new();
                    for (t, d) in log.iter() {
                        let f = Frame {
                            t: *t,
                            hash: hash_of(d),
                            dump: canon(d),
                        };
                        // one frame per second: the last dump of a second wins
                        match frames.last_mut() {
                            Some(l) if l.second() == f.second() => *l = f,
                            _ => frames.push(f),
                        }
                    }
                    Track {
                        name: name.clone(),
                        ready_at: *ready,
                        frames,
                    }
                })
                .collect(),
        }
    }

    /// A track by peer name.
    pub fn track(&self, name: &str) -> Option<&Track> {
        self.peers.iter().find(|p| p.name == name)
    }

    /// The `flight.jsonl` text: a header line, then one line per peer and second.
    pub fn to_jsonl(&self) -> String {
        let mut o = obj(vec![
            ("flight", n(1.0)),
            ("scenario", s(self.scenario.clone())),
            ("seed", n(self.seed as f64)),
            ("net", s(self.net.clone())),
        ])
        .dump();
        o.push('\n');
        for p in &self.peers {
            for f in &p.frames {
                let ready = p.ready_at.map_or(J::Null, n);
                let head = obj(vec![
                    ("peer", s(p.name.clone())),
                    ("t", n(f.t)),
                    ("ready_at", ready),
                    ("hash", s(f.hash.clone())),
                ])
                .dump();
                // the dump goes in raw: it is a JSON object already
                o.push_str(&format!(
                    "{},\"dump\":{}}}\n",
                    head.strip_suffix('}').unwrap_or(&head),
                    f.dump
                ));
            }
        }
        o
    }

    /// Reads a `flight.jsonl` text.
    pub fn from_jsonl(text: &str) -> Result<Recording, String> {
        let mut lines = text.lines().filter(|l| !l.trim().is_empty());
        let head = J::parse(lines.next().ok_or("an empty flight recording")?)?;
        if head.get("flight").is_none() {
            return Err("not a flight recording (no header line)".into());
        }
        let mut r = Recording {
            scenario: head
                .get("scenario")
                .and_then(J::as_str)
                .unwrap_or("")
                .into(),
            seed: head.get("seed").and_then(J::as_f64).unwrap_or(0.0) as u64,
            net: head.get("net").and_then(J::as_str).unwrap_or("").into(),
            peers: Vec::new(),
        };
        for (i, l) in lines.enumerate() {
            let j = J::parse(l).map_err(|e| format!("line {}: {e}", i + 2))?;
            let name = j
                .get("peer")
                .and_then(J::as_str)
                .ok_or_else(|| format!("line {}: no peer", i + 2))?
                .to_string();
            let f = Frame {
                t: j.get("t").and_then(J::as_f64).unwrap_or(0.0),
                hash: j.get("hash").and_then(J::as_str).unwrap_or("").into(),
                dump: j.get("dump").map(J::dump).unwrap_or_default(),
            };
            let ready = j.get("ready_at").and_then(J::as_f64);
            match r.peers.iter_mut().find(|p| p.name == name) {
                Some(p) => p.frames.push(f),
                None => r.peers.push(Track {
                    name,
                    ready_at: ready,
                    frames: vec![f],
                }),
            }
        }
        Ok(r)
    }

    /// Writes `flight.jsonl` into `dir`.
    pub fn save(&self, dir: &Path) -> Result<std::path::PathBuf, String> {
        std::fs::create_dir_all(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
        let p = dir.join("flight.jsonl");
        std::fs::write(&p, self.to_jsonl()).map_err(|e| format!("{}: {e}", p.display()))?;
        Ok(p)
    }
}

/// Where two recordings (or two peers) first differ.
#[derive(Clone, Debug, PartialEq)]
pub struct Divergence {
    /// The first differing second.
    pub second: u64,
    /// The peer (run against run) or `host` (peer against peer).
    pub a: String,
    /// The same peer of the other run, or the guest.
    pub b: String,
    /// The dump paths whose values differ (`items.host.Caps001`, ...).
    pub paths: Vec<String>,
    /// The shared hashes at that second.
    pub hashes: (String, String),
}

impl Divergence {
    /// One line for a person.
    pub fn line(&self) -> String {
        format!(
            "first divergence at second {}: {} vs {} (shared hash {} vs {}); differing paths: {}",
            self.second,
            self.a,
            self.b,
            self.hashes.0,
            self.hashes.1,
            if self.paths.is_empty() {
                "(none outside the hash)".to_string()
            } else {
                self.paths.join(", ")
            }
        )
    }
}

/// Top-level dump sections that mirror the shared state; a host-guest
/// difference elsewhere (positions, counters, the local view) is expected.
pub const SHARED_SECTIONS: [&str; 6] =
    ["quests", "globals", "items", "disabled", "dead", "objects"];

/// The leaf paths at which `a` and `b` differ (union of object keys).
pub fn diff_paths(a: &J, b: &J) -> Vec<String> {
    fn go(a: Option<&J>, b: Option<&J>, at: &str, out: &mut Vec<String>) {
        match (a, b) {
            (Some(J::Obj(x)), Some(J::Obj(y))) => {
                let keys: std::collections::BTreeSet<&String> = x.keys().chain(y.keys()).collect();
                for k in keys {
                    let p = if at.is_empty() {
                        k.clone()
                    } else {
                        format!("{at}.{k}")
                    };
                    go(x.get(k), y.get(k), &p, out);
                }
            }
            (Some(J::Arr(x)), Some(J::Arr(y))) if x.len() == y.len() => {
                for (i, (p, q)) in x.iter().zip(y).enumerate() {
                    go(Some(p), Some(q), &format!("{at}.{i}"), out);
                }
            }
            (x, y) if x != y => out.push(at.to_string()),
            _ => {}
        }
    }
    let mut out = Vec::new();
    go(Some(a), Some(b), "", &mut out);
    out
}

/// The first second at which any peer's dump differs between two runs.
pub fn diverge_runs(a: &Recording, b: &Recording) -> Option<Divergence> {
    let mut best: Option<Divergence> = None;
    for pa in &a.peers {
        let Some(pb) = b.track(&pa.name) else {
            return Some(Divergence {
                second: 0,
                a: pa.name.clone(),
                b: "(missing)".into(),
                paths: vec![format!("peer {} is only in the first run", pa.name)],
                hashes: (String::new(), String::new()),
            });
        };
        let seconds: std::collections::BTreeSet<u64> = pa
            .frames
            .iter()
            .chain(&pb.frames)
            .map(Frame::second)
            .collect();
        for sec in seconds {
            let fa = pa.frames.iter().find(|f| f.second() == sec);
            let fb = pb.frames.iter().find(|f| f.second() == sec);
            let differs = match (fa, fb) {
                (Some(x), Some(y)) => x.dump != y.dump,
                _ => true,
            };
            if !differs {
                continue;
            }
            if best.as_ref().is_some_and(|d| d.second <= sec) {
                break;
            }
            let paths = match (fa, fb) {
                (Some(x), Some(y)) => match (J::parse(&x.dump), J::parse(&y.dump)) {
                    (Ok(p), Ok(q)) => diff_paths(&p, &q),
                    _ => vec!["(a dump does not parse)".into()],
                },
                (None, _) => vec![format!("(no frame in the first run at second {sec})")],
                (_, None) => vec![format!("(no frame in the second run at second {sec})")],
            };
            best = Some(Divergence {
                second: sec,
                a: pa.name.clone(),
                b: pb.name.clone(),
                paths,
                hashes: (
                    fa.map(|f| f.hash.clone()).unwrap_or_default(),
                    fb.map(|f| f.hash.clone()).unwrap_or_default(),
                ),
            });
            break;
        }
    }
    best
}

/// The first second at which the host and a guest hold different shared
/// hashes and still differ `lag_s` seconds later, with the differing paths of
/// the shared sections ([`SHARED_SECTIONS`]). Paths that differed already at
/// the last second with equal hashes (the clock globals) are left out.
pub fn diverge_peers(r: &Recording, lag_s: f64) -> Option<Divergence> {
    let host = r.peers.first()?;
    let mut best: Option<Divergence> = None;
    for g in r.peers.iter().skip(1) {
        // both peers must be ready: before that a guest holds nothing yet
        let from = host
            .ready_at
            .unwrap_or(0.0)
            .max(g.ready_at.unwrap_or(f64::MAX));
        let differs = |sec: u64| -> Option<bool> {
            let h = host.frames.iter().find(|f| f.second() == sec)?;
            let x = g.frames.iter().find(|f| f.second() == sec)?;
            Some(h.hash != x.hash)
        };
        let lag = lag_s.ceil().max(0.0) as u64;
        for f in &host.frames {
            let sec = f.second();
            if (sec as f64) < from || differs(sec) != Some(true) {
                continue;
            }
            // a lag: the guest catches up within `lag_s`
            if (1..=lag).any(|k| differs(sec + k) == Some(false)) {
                continue;
            }
            if best.as_ref().is_some_and(|d| d.second <= sec) {
                break;
            }
            let x = g
                .frames
                .iter()
                .find(|q| q.second() == sec)
                .expect("checked");
            let shared_diff = |hd: &str, gd: &str| -> Option<Vec<String>> {
                let (p, q) = (J::parse(hd).ok()?, J::parse(gd).ok()?);
                Some(
                    SHARED_SECTIONS
                        .iter()
                        .flat_map(|k| {
                            diff_paths(p.get(k).unwrap_or(&J::Null), q.get(k).unwrap_or(&J::Null))
                                .into_iter()
                                .map(move |d| {
                                    if d.is_empty() {
                                        k.to_string()
                                    } else {
                                        format!("{k}.{d}")
                                    }
                                })
                        })
                        .collect(),
                )
            };
            // paths that differed already at the last second with equal hashes
            // (the clock globals a guest holds a moment behind) are not news
            let noise: Vec<String> = (0..sec)
                .rev()
                .find(|s| differs(*s) == Some(false))
                .and_then(|s| {
                    let h = host.frames.iter().find(|q| q.second() == s)?;
                    let y = g.frames.iter().find(|q| q.second() == s)?;
                    shared_diff(&h.dump, &y.dump)
                })
                .unwrap_or_default();
            let paths = match shared_diff(&f.dump, &x.dump) {
                Some(v) => v.into_iter().filter(|p| !noise.contains(p)).collect(),
                None => vec!["(a dump does not parse)".into()],
            };
            best = Some(Divergence {
                second: sec,
                a: host.name.clone(),
                b: g.name.clone(),
                paths,
                hashes: (f.hash.clone(), x.hash.clone()),
            });
            break;
        }
    }
    best
}

/// Pretty JSON text (two space indent, keys sorted), as the scenario files are kept.
pub fn pretty(j: &J) -> String {
    fn go(j: &J, ind: usize, out: &mut String) {
        let pad = "  ".repeat(ind + 1);
        match j {
            J::Arr(a) if !a.is_empty() => {
                out.push_str("[\n");
                for (i, v) in a.iter().enumerate() {
                    out.push_str(&pad);
                    go(v, ind + 1, out);
                    out.push_str(if i + 1 < a.len() { ",\n" } else { "\n" });
                }
                out.push_str(&"  ".repeat(ind));
                out.push(']');
            }
            J::Obj(m) if !m.is_empty() => {
                out.push_str("{\n");
                for (i, (k, v)) in m.iter().enumerate() {
                    out.push_str(&pad);
                    out.push_str(&J::Str(k.clone()).dump());
                    out.push_str(": ");
                    go(v, ind + 1, out);
                    out.push_str(if i + 1 < m.len() { ",\n" } else { "\n" });
                }
                out.push_str(&"  ".repeat(ind));
                out.push('}');
            }
            other => out.push_str(&other.dump()),
        }
    }
    let mut out = String::new();
    go(j, 0, &mut out);
    out.push('\n');
    out
}

// ------------------------------------------------------------------ shrink

/// What [`shrink`] did.
#[derive(Clone, Debug)]
pub struct Shrunk {
    /// The smallest failing scenario (pretty JSON).
    pub text: String,
    /// Script lines before.
    pub before: usize,
    /// Script lines after.
    pub after: usize,
    /// Runs made.
    pub runs: usize,
    /// The failure kept (class and the first failing check).
    pub failure: String,
}

/// Every script array of a scenario document, as (side key, guest index).
fn script_slots(j: &J) -> Vec<(String, Option<usize>)> {
    let mut v = Vec::new();
    for side in ["host", "guest"] {
        if j.get(side)
            .and_then(|x| x.get("script"))
            .and_then(J::as_arr)
            .is_some()
        {
            v.push((side.to_string(), None));
        }
    }
    if let Some(gs) = j.get("guests").and_then(J::as_arr) {
        for (i, g) in gs.iter().enumerate() {
            if g.get("script").and_then(J::as_arr).is_some() {
                v.push(("guests".to_string(), Some(i)));
            }
        }
    }
    v
}

fn script_mut<'a>(j: &'a mut J, slot: &(String, Option<usize>)) -> Option<&'a mut Vec<J>> {
    let J::Obj(m) = j else { return None };
    let side = m.get_mut(&slot.0)?;
    let side = match (side, slot.1) {
        (J::Arr(a), Some(i)) => a.get_mut(i)?,
        (x, None) => x,
        _ => return None,
    };
    match side {
        J::Obj(m) => match m.get_mut("script")? {
            J::Arr(a) => Some(a),
            _ => None,
        },
        _ => None,
    }
}

fn count_lines(j: &J) -> usize {
    let mut c = j.clone();
    script_slots(j)
        .iter()
        .map(|s| script_mut(&mut c, s).map_or(0, |a| a.len()))
        .sum()
}

/// How a run failed: its class and the id of the first failing check.
fn failure_of(scn: &Scenario, d: &Arc<Data>, o: &RunOpts) -> Option<String> {
    let r = run(scn, d, o);
    if r.verdict.result != "fail" {
        return None;
    }
    // the first failing check: `ID: SIDE PATH OP VALUE (got X)`
    let check = r
        .verdict
        .reason
        .split(" | ")
        .next()
        .unwrap_or("")
        .split(';')
        .next()
        .unwrap_or("");
    let first = check.split(':').next().unwrap_or("").trim();
    // a value that is not there at all is another failure than a wrong value
    let missing = if check.contains("<missing>") {
        " (missing)"
    } else {
        ""
    };
    Some(format!("{} {first}{missing}", r.verdict.class))
}

/// Removes script lines from the failing scenario `text` one by one while it
/// still fails with the same class and first failing check (a missing value is
/// another failure than a wrong value), until no line can go. `o` gives the
/// seed and network of every run.
pub fn shrink(text: &str, name: &str, d: &Arc<Data>, o: &RunOpts) -> Result<Shrunk, String> {
    let mut doc = J::parse(text)?;
    let scn = Scenario::parse(text, name)?;
    if scn.expect_fail {
        return Err("shrink takes a scenario that fails; this one is a mutation twin".into());
    }
    let want = failure_of(&scn, d, o).ok_or("the scenario does not fail, nothing to shrink")?;
    if want.starts_with("harness") || want.starts_with("needs_assets") {
        return Err(format!("the scenario does not run here ({want})"));
    }
    let before = count_lines(&doc);
    let mut runs = 1;
    loop {
        let mut removed = false;
        for slot in script_slots(&doc) {
            let mut i = 0;
            while i < script_mut(&mut doc, &slot).map_or(0, |a| a.len()) {
                let mut cand = doc.clone();
                if let Some(a) = script_mut(&mut cand, &slot) {
                    a.remove(i);
                }
                runs += 1;
                let still = Scenario::parse(&cand.dump(), name)
                    .ok()
                    .and_then(|s| failure_of(&s, d, o))
                    .is_some_and(|f| f == want);
                if still {
                    doc = cand;
                    removed = true;
                } else {
                    i += 1;
                }
            }
        }
        if !removed {
            break;
        }
    }
    Ok(Shrunk {
        text: pretty(&doc),
        before,
        after: count_lines(&doc),
        runs,
        failure: want,
    })
}

// ------------------------------------------------------------------ gentest

/// The seeded generator of [`gentest`] (xorshift64*).
struct Rng(u64);

impl Rng {
    fn new(seed: u64) -> Rng {
        Rng(seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1)
    }
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        self.0.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }
    fn below(&mut self, n: u64) -> u64 {
        self.next() % n.max(1)
    }
    fn pick<'a>(&mut self, v: &[&'a str]) -> &'a str {
        v[self.below(v.len() as u64) as usize]
    }
}

/// Networks a generated scenario runs on (the ones that end on a perfect link).
pub const GEN_NETS: [&str; 4] = ["perfect", "good_wifi", "bad_wifi", "mobile"];
/// The cell every generated scenario plays in.
pub const GEN_CELL: &str = "GSDocMitchellHouse";

/// One random script line from the bot verbs (and, for the host, the console).
fn gen_line(r: &mut Rng, host: bool) -> J {
    const WORDS: [&str; 8] = [
        "hello", "over", "here", "wait", "go", "left", "loot", "door",
    ];
    const ACTS: [&str; 6] = ["swing", "shoot", "draw", "holster", "aim", "reload"];
    let at = 1.0 + r.below(90) as f64 / 10.0;
    let kinds = if host { 8 } else { 7 };
    let line = match r.below(kinds) {
        0 => format!("coop say {} {}", r.pick(&WORDS), r.pick(&WORDS)),
        1 => format!("coop ping {}", 100 + r.below(400)),
        2 => format!("coop waypoint {} {}", r.below(200), r.below(200)),
        3 => format!("coop voice {}", 1 + r.below(3)),
        4 => format!("coop attack {}", r.pick(&ACTS)),
        5 => format!("coop face {}", r.below(360)),
        6 => {
            let key = r.pick(&["w", "a", "s", "d"]);
            let hold = 0.5 + r.below(15) as f64 / 10.0;
            return obj(vec![("at", n(at)), ("key", s(format!("{key}:{hold}")))]);
        }
        _ => format!(
            "player.AddItem {} {}",
            r.pick(&["Stimpak", "Caps001", "WaterPurified"]),
            1 + r.below(5)
        ),
    };
    obj(vec![("at", n(at)), ("run", s(line))])
}

/// A random valid scenario for `seed`: a host and 1 to 3 guests in
/// [`GEN_CELL`], each with 2 to 6 lines of the bot verbs, judged on one
/// shared state and on everyone seeing the others.
pub fn gentest(seed: u64) -> String {
    let mut r = Rng::new(seed);
    let guests = 1 + r.below(3) as usize;
    let side = |r: &mut Rng, host: bool| {
        let k = 2 + r.below(5);
        let mut lines: Vec<J> = (0..k).map(|_| gen_line(r, host)).collect();
        lines.sort_by(|a, b| {
            let t = |x: &J| x.get("at").and_then(J::as_f64).unwrap_or(0.0);
            t(a).total_cmp(&t(b))
        });
        obj(vec![("script", J::Arr(lines))])
    };
    let host = side(&mut r, true);
    let gs: Vec<J> = (0..guests).map(|_| side(&mut r, false)).collect();
    let net = r.pick(&GEN_NETS);
    let mut pairs = vec![
        ("name", s(format!("gen_{seed}"))),
        (
            "description",
            s(format!(
                "gentest seed {seed}: random valid play from the bot verbs ({} guests); one shared state and everyone in view",
                guests
            )),
        ),
        ("cell", s(GEN_CELL)),
        ("network", s(net)),
        ("seed", n(seed as f64)),
        ("duration_s", n(12.0)),
        ("settle_s", n(4.0)),
        ("host", host),
        (
            "expect",
            J::Arr(vec![
                obj(vec![
                    ("id", s("avatars")),
                    ("path", s("live.avatars")),
                    ("op", s("eq")),
                    ("value", n(guests as f64)),
                    ("on", s("all")),
                ]),
                obj(vec![
                    ("id", s("h")),
                    ("path", s("hash.shared")),
                    ("on", s("equal")),
                ]),
            ]),
        ),
    ];
    if guests == 1 {
        pairs.push(("guest", gs.into_iter().next().expect("one guest")));
    } else {
        pairs.push(("guests", J::Arr(gs)));
    }
    pretty(&obj(pairs))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rec(frames: &[(&str, &[(f64, &str)])]) -> Recording {
        Recording {
            scenario: "t".into(),
            seed: 1,
            net: "perfect".into(),
            peers: frames
                .iter()
                .map(|(name, fs)| Track {
                    name: name.to_string(),
                    ready_at: Some(0.5),
                    frames: fs
                        .iter()
                        .map(|(t, d)| Frame {
                            t: *t,
                            hash: hash_of(d),
                            dump: canon(d),
                        })
                        .collect(),
                })
                .collect(),
        }
    }

    const A: &str = r#"{"items":{"host":{"Stimpak":3}},"hash":{"shared":"aa"}}"#;
    const B: &str = r#"{"items":{"host":{"Stimpak":3,"Caps001":25}},"hash":{"shared":"bb"}}"#;

    #[test]
    fn runs_diverge_at_the_first_differing_second_with_the_path() {
        let a = rec(&[("host", &[(1.0, A), (2.0, A), (3.0, A)])]);
        let b = rec(&[("host", &[(1.0, A), (2.0, B), (3.0, B)])]);
        let d = diverge_runs(&a, &b).unwrap();
        assert_eq!(d.second, 2);
        // run against run: every differing path, the hash included
        assert_eq!(
            d.paths,
            vec!["hash.shared".to_string(), "items.host.Caps001".to_string()]
        );
        assert!(diverge_runs(&a, &a).is_none());
    }

    #[test]
    fn peers_diverge_only_when_the_guest_does_not_catch_up() {
        // a one second lag is not a divergence
        let lag = rec(&[
            ("host", &[(1.0, A), (2.0, B), (3.0, B), (4.0, B)]),
            ("guest", &[(1.0, A), (2.0, A), (3.0, B), (4.0, B)]),
        ]);
        assert!(diverge_peers(&lag, 2.0).is_none());
        let stuck = rec(&[
            ("host", &[(1.0, A), (2.0, B), (3.0, B), (4.0, B), (5.0, B)]),
            ("guest", &[(1.0, A), (2.0, A), (3.0, A), (4.0, A), (5.0, A)]),
        ]);
        let d = diverge_peers(&stuck, 2.0).unwrap();
        assert_eq!((d.second, d.b.as_str()), (2, "guest"));
        assert_eq!(d.paths, vec!["items.host.Caps001".to_string()]);
    }

    #[test]
    fn flight_files_round_trip() {
        let a = rec(&[("host", &[(1.0, A), (2.0, B)]), ("guest", &[(1.0, A)])]);
        let back = Recording::from_jsonl(&a.to_jsonl()).unwrap();
        assert_eq!(back.peers.len(), 2);
        assert_eq!(back.peers[0].frames[1].hash, "bb");
        assert!(diverge_runs(&a, &back).is_none());
    }

    #[test]
    fn gentest_is_valid_deterministic_and_seeded() {
        for seed in 1..40 {
            let t = gentest(seed);
            let scn = Scenario::parse(&t, "g").unwrap_or_else(|e| panic!("seed {seed}: {e}\n{t}"));
            assert!(scn.guest_count() >= 1);
            assert_eq!(t, gentest(seed));
        }
        assert_ne!(gentest(1), gentest(2));
    }

    #[test]
    fn script_slots_cover_every_side() {
        let j = J::parse(&gentest(3)).unwrap();
        let n = count_lines(&j);
        assert!(n >= 4, "{n}");
        let mut c = j.clone();
        let slot = script_slots(&j)[0].clone();
        script_mut(&mut c, &slot).unwrap().remove(0);
        assert_eq!(count_lines(&c), n - 1);
    }
}

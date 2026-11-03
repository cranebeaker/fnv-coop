//! Runs the rows of the manifest and turns the results into [`Outcome`]s.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Instant;

use super::builtin::{self, Env};
use super::{Outcome, Row, Status};
use crate::backend::{self, Verdict};
use crate::data::Data;
use crate::questwalk::{self, Depth, QStatus, QuestResult, WalkOpts};
use crate::scenario::{Scenario, PROFILES};
use crate::suite::{run_jobs, Done, Job};

/// What to run.
#[derive(Clone, Debug)]
pub struct Opts {
    /// Only this area (case-insensitive prefix).
    pub area: Option<String>,
    /// Seeds for scenarios; `None`: each scenario's own seed.
    pub seeds: Option<Vec<u64>>,
    /// `all`, a profile, or none (each scenario's own).
    pub net: Option<String>,
    /// Party size for the checks that take one.
    pub peers: u8,
    /// Worker threads.
    pub jobs: usize,
    /// Data folder argument.
    pub data: Option<String>,
    /// `world` or `engine`.
    pub backend: String,
    /// Folder of `tools/coop-run` outputs (`verdict.json` files).
    pub real_out: Option<PathBuf>,
    /// The big matrix: all profiles and 2, 3, 4 peers for the quest walk.
    pub full: bool,
    /// Manifest folder.
    pub manifest: PathBuf,
    /// Extra scenario folders.
    pub scenario_dirs: Vec<PathBuf>,
    /// Skip the per-quest walk.
    pub no_questwalk: bool,
    /// Dev loop: one profile for the quest walk.
    pub quick: bool,
}

impl Default for Opts {
    fn default() -> Self {
        Opts {
            area: None,
            seeds: None,
            net: None,
            peers: 2,
            jobs: 4,
            data: None,
            backend: "world".into(),
            real_out: None,
            full: false,
            manifest: super::default_manifest_dir(),
            scenario_dirs: Vec::new(),
            no_questwalk: false,
            quick: false,
        }
    }
}

/// Everything a run produced.
pub struct Report {
    /// One outcome per row (manifest rows, then one per quest).
    pub outcomes: Vec<Outcome>,
    /// The quest walk results.
    pub quests: Vec<QuestResult>,
    /// Wall seconds.
    pub wall_s: f64,
    /// Options used.
    pub opts: Opts,
    /// Plugins in the load order.
    pub plugins: Vec<String>,
    /// Quest rows the bundled table has that the load order does not.
    pub missing_quests: usize,
    /// Scenario files found.
    pub scenarios_found: usize,
}

/// One test's result.
#[derive(Clone, Debug)]
enum T {
    Pass(String),
    Fail(String, String),
    Pending(String),
    Skip(String),
}

/// A scenario file found on disk.
#[derive(Clone)]
struct Found {
    path: String,
    scn: Arc<Scenario>,
}

fn scenario_dirs(extra: &[PathBuf]) -> Vec<PathBuf> {
    let root = super::repo_root();
    let mut v = vec![
        root.join("tools").join("coop-run").join("scenarios"),
        root.join("crates").join("coop-headless").join("scenarios"),
    ];
    v.extend(extra.iter().cloned());
    v
}

fn find_scenarios(dirs: &[PathBuf]) -> BTreeMap<String, Found> {
    let mut m = BTreeMap::new();
    for d in dirs {
        let Ok(rd) = std::fs::read_dir(d) else {
            continue;
        };
        let mut files: Vec<PathBuf> = rd
            .flatten()
            .map(|e| e.path())
            .filter(|f| f.extension().is_some_and(|x| x == "json"))
            .collect();
        files.sort();
        for f in files {
            let Ok(s) = Scenario::load(&f) else { continue };
            let key = f
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_string();
            let rel = match (f.canonicalize(), super::repo_root().canonicalize()) {
                (Ok(a), Ok(r)) => a
                    .strip_prefix(&r)
                    .map(|p| p.display().to_string().replace('\\', "/"))
                    .unwrap_or_else(|_| f.display().to_string()),
                _ => f.display().to_string(),
            };
            m.entry(key).or_insert(Found {
                path: rel,
                scn: Arc::new(s),
            });
        }
    }
    m
}

fn dlc_key(s: &str) -> String {
    s.to_ascii_lowercase()
        .replace([' ', '\''], "_")
        .replace("__", "_")
}

/// Does a quest belong to a `questwalk:SCOPE`? Scopes: `all`, `base`, a DLC key
/// (`dead_money`), `line:LABEL` (the policy row's questline) or `edid:NAME`.
fn scope_match(scope: &str, q: &QuestResult) -> bool {
    if let Some(l) = scope.strip_prefix("line:") {
        return q.line.eq_ignore_ascii_case(l);
    }
    if let Some(e) = scope.strip_prefix("edid:") {
        return q.edid.eq_ignore_ascii_case(e);
    }
    let k = dlc_key(scope);
    k == "all" || dlc_key(&q.dlc) == k || (k == "base" && q.dlc == "Base game")
}

fn group_of(r: &QuestResult) -> String {
    if r.dlc != "Base game" {
        return format!("Quests: {}", r.dlc);
    }
    let l = &r.line;
    let g = if l.starts_with("Main") {
        "Main story"
    } else if l.starts_with("Side") {
        "Side quests"
    } else if l.starts_with("Systems") {
        "Systems and scripted"
    } else {
        "Factions and leftovers"
    };
    format!("Quests: Base game, {g}")
}

/// A row's data needs: one need, or several separated by commas (all must be here).
fn needs_ok(needs: &str, d: Option<&Data>) -> Result<(), String> {
    if needs.contains(',') {
        return needs
            .split(',')
            .map(str::trim)
            .try_for_each(|n| needs_ok(n, d));
    }
    let Some(d) = d else {
        return Err("no data folder".into());
    };
    if needs == "base" {
        return Ok(());
    }
    if needs == "full_assets" {
        return if d.has_meshes {
            Ok(())
        } else {
            Err("needs full assets (meshes)".into())
        };
    }
    if let Some(name) = needs.strip_prefix("dlc:") {
        let key = name.to_ascii_lowercase().replace([' ', '_'], "");
        let has = d.order.plugins().iter().any(|p| {
            p.name
                .to_ascii_lowercase()
                .replace([' ', '_'], "")
                .contains(&key)
        });
        return if has {
            Ok(())
        } else {
            Err(format!("needs DLC {name}"))
        };
    }
    Ok(())
}

/// Last real-game verdict per scenario from `tools/coop-run` output folders.
/// `--backend engine`: a world-bound row is PENDING, the engine backend runs nothing yet.
pub fn engine_gate(engine: bool, row_backend: &str) -> Option<String> {
    (engine && row_backend == "world").then(|| "engine backend".to_string())
}

pub fn read_real(dir: &Path) -> BTreeMap<String, (String, bool)> {
    fn walk(d: &Path, depth: u32, out: &mut Vec<PathBuf>) {
        let Ok(rd) = std::fs::read_dir(d) else { return };
        for e in rd.flatten() {
            let p = e.path();
            if p.is_dir() && depth < 3 {
                walk(&p, depth + 1, out);
            } else if p.file_name().is_some_and(|n| n == "verdict.json") {
                out.push(p);
            }
        }
    }
    let mut files = Vec::new();
    walk(dir, 0, &mut files);
    let mut best: BTreeMap<String, (String, String, bool)> = BTreeMap::new();
    for f in files {
        let Ok(text) = std::fs::read_to_string(&f) else {
            continue;
        };
        let Ok(j) = serde_json::from_str::<serde_json::Value>(&text) else {
            continue;
        };
        let (Some(name), Some(res)) = (
            j.get("scenario").and_then(|v| v.as_str()),
            j.get("result").and_then(|v| v.as_str()),
        ) else {
            continue;
        };
        let stamp = f
            .parent()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();
        let date = if stamp.len() >= 8 && stamp[..8].chars().all(|c| c.is_ascii_digit()) {
            format!("{}-{}-{}", &stamp[..4], &stamp[4..6], &stamp[6..8])
        } else {
            "undated".into()
        };
        let key = format!("{stamp}");
        if best.get(name).is_none_or(|(k, _, _)| key > *k) {
            best.insert(name.to_string(), (key, date, res == "pass"));
        }
    }
    best.into_iter().map(|(k, (_, d, p))| (k, (d, p))).collect()
}

fn real_cell(row: &Row, real: &Option<BTreeMap<String, (String, bool)>>) -> String {
    if row.real.is_empty() {
        return "-".into();
    }
    let Some(map) = real else {
        return "not read".into();
    };
    let mut parts = Vec::new();
    for n in &row.real {
        match map.get(n) {
            Some((d, true)) => parts.push(format!("{d} PASS")),
            Some((d, false)) => parts.push(format!("{d} FAIL")),
            None => parts.push("never run".into()),
        }
    }
    parts.sort();
    parts.dedup();
    parts.join(", ")
}

fn combine(rs: Vec<T>) -> (Status, String, Vec<String>) {
    let mut detail = Vec::new();
    let mut replay = Vec::new();
    let (mut fail, mut pend, mut skip) = (Vec::new(), Vec::new(), Vec::new());
    for r in rs {
        match r {
            T::Pass(d) => detail.push(d),
            T::Fail(m, rp) => {
                fail.push(m);
                if !rp.is_empty() {
                    replay.push(rp);
                }
            }
            T::Pending(m) => pend.push(m),
            T::Skip(m) => skip.push(m),
        }
    }
    let status = if !fail.is_empty() {
        Status::Fail(fail.join("; "))
    } else if !pend.is_empty() {
        Status::Pending(pend.join("; "))
    } else if !skip.is_empty() {
        Status::Skip(skip.join("; "))
    } else {
        Status::Pass
    };
    (status, detail.join("; "), replay)
}

fn parallel<T: Send + Sync, R: Send>(
    items: Vec<T>,
    jobs: usize,
    f: impl Fn(&T) -> R + Sync,
) -> Vec<R> {
    let next = AtomicUsize::new(0);
    let out: Mutex<Vec<(usize, R)>> = Mutex::new(Vec::new());
    std::thread::scope(|sc| {
        for _ in 0..jobs.max(1).min(items.len().max(1)) {
            sc.spawn(|| loop {
                let i = next.fetch_add(1, Ordering::SeqCst);
                let Some(it) = items.get(i) else { break };
                let r = f(it);
                out.lock().expect("out").push((i, r));
            });
        }
    });
    let mut v = out.into_inner().expect("out");
    v.sort_by_key(|x| x.0);
    v.into_iter().map(|x| x.1).collect()
}

/// Runs the checklist.
pub fn run(opts: &Opts) -> Result<Report, String> {
    let t0 = Instant::now();
    let mut rows = super::load(&opts.manifest)?;
    if let Some(a) = &opts.area {
        let a = a.to_ascii_lowercase();
        rows.retain(|r| r.area.to_ascii_lowercase().starts_with(&a));
    }
    let data: Option<Arc<Data>> = Data::locate(opts.data.as_deref())
        .and_then(|d| Data::open(&d))
        .ok()
        .map(Arc::new);
    let plugins: Vec<String> = data
        .as_ref()
        .map(|d| d.order.plugins().iter().map(|p| p.name.clone()).collect())
        .unwrap_or_default();
    let found = find_scenarios(&scenario_dirs(&opts.scenario_dirs));
    let real = opts.real_out.as_ref().map(|d| read_real(d));
    let engine = opts.backend == "engine";
    let be = backend::by_name(&opts.backend)?;

    // ---- gather the work
    let mut scn_jobs: Vec<(String, u64, Option<String>)> = Vec::new();
    let mut builtins: BTreeSet<(String, bool)> = BTreeSet::new();
    let mut conf: BTreeSet<(String, bool)> = BTreeSet::new();
    let mut want_walk: BTreeSet<String> = BTreeSet::new();
    let nets: Vec<Option<String>> = match opts.net.as_deref() {
        None => vec![None],
        Some("all") => PROFILES.iter().map(|p| Some(p.to_string())).collect(),
        Some(p) => vec![Some(p.to_string())],
    };
    let seeds_for =
        |f: &Found| -> Vec<u64> { opts.seeds.clone().unwrap_or_else(|| vec![f.scn.seed]) };
    let twin_names = |row: &Row, name: &str| -> Vec<String> {
        if row.twin_ids.is_empty() {
            vec![format!("twin_{name}")]
        } else {
            row.twin_ids.clone()
        }
    };
    for row in &rows {
        for t in &row.tests {
            let (kind, name) = t.split_once(':').unwrap_or(("", t));
            match kind {
                "scenario" => {
                    let mut names = vec![name.to_string()];
                    if row.twin {
                        names.extend(twin_names(row, name));
                    }
                    for n in names {
                        if let Some(f) = found.get(&n) {
                            for s in seeds_for(f) {
                                for net in &nets {
                                    scn_jobs.push((n.clone(), s, net.clone()));
                                }
                            }
                        }
                    }
                }
                "builtin" => {
                    builtins.insert((name.to_string(), false));
                    if row.twin {
                        builtins.insert((name.to_string(), true));
                    }
                }
                "conformance" => {
                    conf.insert((name.to_string(), false));
                    if row.twin {
                        conf.insert((name.to_string(), true));
                    }
                }
                "questwalk" => {
                    want_walk.insert(name.to_string());
                }
                _ => {}
            }
        }
    }
    scn_jobs.sort();
    scn_jobs.dedup();

    // ---- scenarios
    let mut done: BTreeMap<(String, u64, Option<String>), Done> = BTreeMap::new();
    if let Some(d) = &data {
        let jobs: Vec<Job> = scn_jobs
            .iter()
            .map(|(n, s, net)| Job {
                scn: found[n].scn.clone(),
                path: found[n].path.clone(),
                seed: *s,
                net: net.clone(),
            })
            .collect();
        let res = run_jobs(
            d,
            jobs,
            opts.jobs,
            None,
            false,
            opts.data.clone(),
            false,
            false,
        );
        for ((n, s, net), r) in scn_jobs.iter().zip(res) {
            done.insert((n.clone(), *s, net.clone()), r);
        }
    }

    // ---- builtins and conformance
    let env = |peers| Env {
        data: data.as_deref(),
        peers,
        net: opts
            .net
            .as_deref()
            .filter(|n| *n != "all")
            .unwrap_or("bad_wifi"),
        seed: 7,
    };
    let reg = builtin::registry();
    let bres: BTreeMap<(String, bool), Result<String, String>> = {
        let list: Vec<(String, bool)> = builtins.into_iter().collect();
        let prev = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let r = parallel(list.clone(), opts.jobs, |(n, m)| {
            match reg.iter().find(|(x, _)| x == n) {
                None => Err(format!("PENDING no builtin check {n}")),
                Some((_, f)) => std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    f(&env(opts.peers), *m)
                }))
                .unwrap_or_else(|_| Err("panic".into())),
            }
        });
        std::panic::set_hook(prev);
        list.into_iter().zip(r).collect()
    };
    let cres: BTreeMap<(String, bool), Verdict> = match &data {
        Some(d) => {
            let list: Vec<(String, bool)> = conf.into_iter().collect();
            let prev = std::panic::take_hook();
            std::panic::set_hook(Box::new(|_| {}));
            let r = parallel(list.clone(), opts.jobs, |(n, m)| {
                backend::run_test(&*be, d, n, *m)
            });
            std::panic::set_hook(prev);
            list.into_iter().zip(r).collect()
        }
        None => BTreeMap::new(),
    };

    // ---- quest walk
    let mut quests: Vec<QuestResult> = Vec::new();
    let mut missing_quests = 0;
    if !want_walk.is_empty() && !opts.no_questwalk && !engine {
        if let Some(d) = &data {
            let mut w = WalkOpts {
                jobs: opts.jobs,
                ..WalkOpts::default()
            };
            if opts.quick {
                w.nets = vec!["perfect".into()];
            }
            if let Some(n) = opts.net.as_deref() {
                w.nets = if n == "all" {
                    PROFILES.iter().map(|s| s.to_string()).collect()
                } else {
                    vec![n.to_string()]
                };
            }
            if opts.full {
                w.nets = PROFILES.iter().map(|s| s.to_string()).collect();
                w.peers = vec![2, 3, 4];
            } else if opts.peers != 2 {
                w.peers = vec![opts.peers];
            }
            let (q, miss) = questwalk::walk_all(d, &w)?;
            quests = q;
            missing_quests = miss.len();
        }
    }

    // ---- assemble
    let mut outcomes: Vec<Outcome> = Vec::new();
    let mk_scn = |name: &str, row: &Row| -> T {
        let Some(f) = found.get(name) else {
            return T::Pending(format!("scenario {name} does not exist yet"));
        };
        if data.is_none() {
            return T::Skip("no data folder".into());
        }
        let mut runs = 0;
        let mut skips = 0;
        for s in seeds_for(f) {
            for net in &nets {
                let Some(d) = done.get(&(name.to_string(), s, net.clone())) else {
                    continue;
                };
                runs += 1;
                match d.result.as_str() {
                    "pass" => {}
                    "skip" => skips += 1,
                    _ => {
                        return T::Fail(
                            format!(
                                "{name} seed {} ({}): {} {}",
                                d.seed,
                                d.net,
                                d.class,
                                d.line.trim_start_matches("RESULT: ")
                            ),
                            d.rerun.clone(),
                        )
                    }
                }
            }
        }
        let _ = row;
        if runs == 0 {
            return T::Pending(format!("scenario {name} not run"));
        }
        if skips > 0 {
            return T::Skip(format!("{name}: {skips} runs skipped (needs assets or a viewer flag with no headless model)"));
        }
        T::Pass(format!("{name} x{runs}"))
    };
    for row in &rows {
        let t_row = Instant::now();
        let mut rs: Vec<T> = Vec::new();
        if row.tests.is_empty() {
            rs.push(T::Pending("no test yet".into()));
        } else if let Some(why) = engine_gate(engine, &row.backend) {
            rs.push(T::Pending(why));
        } else if let Err(why) = needs_ok(&row.needs, data.as_deref()) {
            rs.push(T::Skip(why));
        } else {
            for t in &row.tests {
                let (kind, name) = t.split_once(':').unwrap_or(("", t));
                match kind {
                    "scenario" => {
                        rs.push(mk_scn(name, row));
                        if row.twin {
                            for tw in twin_names(row, name) {
                                match found.get(&tw) {
                                    None => rs.push(T::Pending(format!("twin {tw} does not exist yet"))),
                                    Some(f) => {
                                        let mut bad = None;
                                        let mut n = 0;
                                        for s in seeds_for(f) {
                                            for net in &nets {
                                                if let Some(d) = done.get(&(tw.clone(), s, net.clone())) {
                                                    n += 1;
                                                    if d.result == "skip" {
                                                        bad = Some(T::Skip(format!("twin {tw} skipped")));
                                                    } else if d.result != "pass" {
                                                        bad = Some(T::Fail(format!("twin {tw} seed {s} did not fail as expected: {}", d.line.trim_start_matches("RESULT: ")), d.rerun.clone()));
                                                    }
                                                }
                                            }
                                        }
                                        rs.push(bad.unwrap_or(if n == 0 { T::Pending(format!("twin {tw} not run")) } else { T::Pass(format!("twin {tw} fails as expected")) }));
                                    }
                                }
                            }
                        }
                    }
                    "builtin" => {
                        let main = &bres[&(name.to_string(), false)];
                        match main {
                            Err(e) if e.starts_with("PENDING") => rs.push(T::Pending(e.trim_start_matches("PENDING ").to_string())),
                            Err(e) => rs.push(T::Fail(format!("{name}: {e}"), format!("coop-sim checklist --area {} --peers {}", row.area, opts.peers))),
                            Ok(d) => rs.push(T::Pass(format!("{name}: {d}"))),
                        }
                        if row.twin {
                            match &bres[&(name.to_string(), true)] {
                                Err(_) => rs.push(T::Pass(format!("twin of {name} fails as expected"))),
                                Ok(_) => rs.push(T::Fail(format!("twin of {name} passed: the check cannot tell the feature from nothing"), String::new())),
                            }
                        }
                    }
                    "conformance" => {
                        let one = |m: bool| cres.get(&(name.to_string(), m)).cloned().unwrap_or(Verdict::Pending("not run".into()));
                        match one(false) {
                            Verdict::Pass => rs.push(T::Pass(format!("conformance {name} on {}", be.name()))),
                            Verdict::Fail(e) => rs.push(T::Fail(format!("conformance {name}: {e}"), format!("coop-sim checklist --area {}", row.area))),
                            Verdict::Pending(e) => rs.push(T::Pending(e)),
                        }
                        if row.twin {
                            match one(true) {
                                Verdict::Fail(_) => rs.push(T::Pass(format!("twin of {name} fails as expected"))),
                                Verdict::Pass => rs.push(T::Fail(format!("twin of {name} passed"), String::new())),
                                Verdict::Pending(e) => rs.push(T::Pending(e)),
                            }
                        }
                    }
                    "questwalk" => {
                        let (name, inv) = name.split_once('#').map_or((name, None), |(a, b)| (a, Some(b)));
                        let sel: Vec<&QuestResult> = quests.iter().filter(|q| scope_match(name, q)).collect();
                        if quests.is_empty() {
                            rs.push(T::Pending("the quest walk did not run".into()));
                        } else if sel.is_empty() {
                            rs.push(T::Pending(format!("no quests for scope {name}")));
                        } else {
                            let failed: Vec<&&QuestResult> = sel.iter().filter(|q| q.status == QStatus::Fail && inv.is_none_or(|i| q.fails.iter().any(|f| questwalk::fail_kind(f) == i))).collect();
                            if failed.is_empty() {
                                let po = sel.iter().filter(|q| q.depth == Depth::PolicyOnly).count();
                                rs.push(T::Pass(format!("{} quests walked ({po} policy-only)", sel.len())));
                            } else {
                                let names: Vec<String> = failed.iter().take(6).map(|q| q.edid.clone()).collect();
                                rs.push(T::Fail(format!("{} of {} quests fail ({})", failed.len(), sel.len(), names.join(", ")), format!("coop-sim questwalk --only {}", names.first().cloned().unwrap_or_default())));
                            }
                        }
                    }
                    "cargo" => rs.push(T::Pending(format!("covered by `cargo test -p {name}` only; the checklist does not run cargo tests"))),
                    other => rs.push(T::Pending(format!("unknown test kind {other:?} in {t}"))),
                }
            }
            if row.twin
                && !row.tests.iter().any(|t| {
                    t.starts_with("scenario:")
                        || t.starts_with("builtin:")
                        || t.starts_with("conformance:")
                })
            {
                rs.push(T::Pending(
                    "twin required but the row has no test a twin can attach to".into(),
                ));
            }
        }
        let (status, detail, replay) = combine(rs);
        let sp = if row.tests.iter().any(|t| t.starts_with("questwalk:")) {
            let scope = row
                .tests
                .iter()
                .find_map(|t| t.strip_prefix("questwalk:"))
                .unwrap_or("all");
            quests
                .iter()
                .filter(|q| scope_match(scope, q) && !q.sp_gaps.is_empty())
                .count()
        } else {
            0
        };
        outcomes.push(Outcome {
            row: row.clone(),
            status,
            detail,
            replay,
            ms: t_row.elapsed().as_millis() as u64,
            real_game: real_cell(row, &real),
            sp_gaps: sp,
        });
    }

    // ---- one outcome per quest (items of the checklist)
    let area_ok = |a: &str| {
        opts.area
            .as_ref()
            .is_none_or(|x| a.to_ascii_lowercase().starts_with(&x.to_ascii_lowercase()))
    };
    for q in &quests {
        let area = group_of(q);
        if !area_ok(&area) && !area_ok("quests") {
            continue;
        }
        let policy_only = q.depth == Depth::PolicyOnly;
        let row = Row {
            id: format!("quest.{}", q.edid),
            area,
            title: format!("{} ({})", q.edid, q.line),
            proves: "stage graph walked on host and guests with the real runner and the policy gate: shared hashes equal, per-player rows private, rewards once, no guest scripts, no panic".into(),
            tests: vec![format!("questwalk:{}", dlc_key(&q.edid))],
            needs: if q.dlc == "Base game" { "base".into() } else { format!("dlc:{}", q.dlc) },
            twin: false,
            twin_ids: vec![],
            fidelity: if policy_only { "modelled".into() } else { "headless-real-logic".into() },
            stand_in: if policy_only { "policy engine only (no quest record in this load order)".into() } else { String::new() },
            owner: "coop".into(),
            real: vec![],
            backend: "world".into(),
            source: "list C".into(),
            note: String::new(),
            layer: "coop".into(),
            group: String::new(),
            real_required: false,
            fix: None,
            meta: serde_json::Value::Null,
        };
        let status = if q.status == QStatus::Pass {
            Status::Pass
        } else {
            Status::Fail(
                q.fails
                    .iter()
                    .take(2)
                    .cloned()
                    .collect::<Vec<_>>()
                    .join(" | "),
            )
        };
        let detail = format!(
            "{} stages: {} applied, {} released, {} refused, {} per-player; {} runs{}",
            q.stages,
            q.applied,
            q.released,
            q.refused,
            q.per_player,
            q.runs,
            if q.sp_gaps.is_empty() {
                String::new()
            } else {
                format!(
                    "; SP-GAP: {}",
                    q.sp_gaps.keys().cloned().collect::<Vec<_>>().join(", ")
                )
            }
        );
        outcomes.push(Outcome {
            row,
            status,
            detail,
            replay: vec![q.replay.clone()],
            ms: q.ms,
            real_game: "-".into(),
            sp_gaps: q.sp_gaps.len(),
        });
    }

    Ok(Report {
        outcomes,
        quests,
        wall_s: t0.elapsed().as_secs_f64(),
        opts: opts.clone(),
        plugins,
        missing_quests,
        scenarios_found: found.len(),
    })
}

//! Many runs: a std::thread pool over (scenario, seed, profile), the soak
//! matrix and the determinism check.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Instant;

use crate::data::Data;
use crate::scenario::{Scenario, PROFILES};
use crate::sim::{run, RunOpts};
use crate::verdict::result_line;

/// One run to do.
#[derive(Clone)]
pub struct Job {
    /// The scenario.
    pub scn: Arc<Scenario>,
    /// Its file, for the replay command.
    pub path: String,
    /// Seed.
    pub seed: u64,
    /// Profile override.
    pub net: Option<String>,
}

/// One run done.
#[derive(Clone, Debug)]
pub struct Done {
    /// Scenario name.
    pub scenario: String,
    /// Seed.
    pub seed: u64,
    /// Profile used.
    pub net: String,
    /// `pass`, `fail`, `skip`.
    pub result: String,
    /// Class.
    pub class: String,
    /// `RESULT:` line.
    pub line: String,
    /// Replay command.
    pub rerun: String,
    /// Dump hash.
    pub hash: u64,
    /// Second hash from a repeat run (determinism check), if asked.
    pub hash2: Option<u64>,
    /// Wall seconds.
    pub wall_s: f64,
    /// Virtual seconds.
    pub virt_s: f64,
}

/// Parses `1..50`, `7` or `1,2,5`.
pub fn parse_seeds(t: &str) -> Result<Vec<u64>, String> {
    if let Some((a, b)) = t.split_once("..") {
        let (a, b): (u64, u64) = (
            a.parse().map_err(|_| format!("bad seed range {t}"))?,
            b.parse().map_err(|_| format!("bad seed range {t}"))?,
        );
        return Ok((a..=b.max(a)).collect());
    }
    t.split(',')
        .map(|x| x.trim().parse::<u64>().map_err(|_| format!("bad seed {x}")))
        .collect()
}

/// Loads every `*.json` scenario of a folder (or one file), sorted by name.
pub fn load_dir(p: &Path) -> Result<Vec<(String, Scenario)>, String> {
    let mut files: Vec<PathBuf> = if p.is_dir() {
        std::fs::read_dir(p)
            .map_err(|e| e.to_string())?
            .flatten()
            .map(|e| e.path())
            .filter(|f| f.extension().is_some_and(|x| x == "json"))
            .collect()
    } else {
        vec![p.to_path_buf()]
    };
    files.sort();
    files
        .into_iter()
        .map(|f| Scenario::load(&f).map(|s| (f.display().to_string(), s)))
        .collect()
}

/// Runs the jobs on `threads` worker threads, in job order of results.
#[allow(clippy::too_many_arguments)]
pub fn run_jobs(
    d: &Arc<Data>,
    jobs: Vec<Job>,
    threads: usize,
    out: Option<PathBuf>,
    twice: bool,
    data_arg: Option<String>,
    talk_at_ready: bool,
    fixed_glue: bool,
) -> Vec<Done> {
    let next = AtomicUsize::new(0);
    let results: Mutex<Vec<(usize, Done)>> = Mutex::new(Vec::new());
    std::thread::scope(|sc| {
        for _ in 0..threads.max(1).min(jobs.len().max(1)) {
            sc.spawn(|| loop {
                let i = next.fetch_add(1, Ordering::SeqCst);
                let Some(job) = jobs.get(i) else { break };
                let o = RunOpts {
                    seed: Some(job.seed),
                    net: job.net.clone(),
                    out: out.clone(),
                    path: job.path.clone(),
                    data_arg: data_arg.clone(),
                    talk_at_ready,
                    fixed_glue,
                    ..RunOpts::default()
                };
                let t0 = Instant::now();
                let r = run(&job.scn, d, &o);
                let wall_s = t0.elapsed().as_secs_f64();
                let hash2 = twice.then(|| {
                    run(
                        &job.scn,
                        d,
                        &RunOpts {
                            out: None,
                            ..o.clone()
                        },
                    )
                    .hash
                });
                let g = |k: &str| {
                    r.verdict
                        .json
                        .get(k)
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string()
                };
                let done = Done {
                    scenario: job.scn.name.clone(),
                    seed: job.seed,
                    net: g("network"),
                    result: r.verdict.result.clone(),
                    class: r.verdict.class.clone(),
                    line: result_line(&r.verdict),
                    rerun: g("rerun"),
                    hash: r.hash,
                    hash2,
                    wall_s,
                    virt_s: r
                        .verdict
                        .json
                        .get("duration_s")
                        .and_then(|v| v.as_f64())
                        .unwrap_or(0.0),
                };
                results.lock().expect("results").push((i, done));
            });
        }
    });
    let mut v = results.into_inner().expect("results");
    v.sort_by_key(|x| x.0);
    v.into_iter().map(|x| x.1).collect()
}

/// Builds the jobs: every scenario x seeds, or (soak) x every profile x seeds.
pub fn plan(
    scns: &[(String, Scenario)],
    seeds: Option<&[u64]>,
    soak: bool,
    net: Option<&str>,
) -> Vec<Job> {
    let mut jobs = Vec::new();
    for (path, s) in scns {
        let own = [s.seed];
        let seeds: &[u64] = seeds.unwrap_or(&own);
        let nets: Vec<Option<String>> = if soak {
            PROFILES.iter().map(|p| Some(p.to_string())).collect()
        } else {
            vec![net.map(str::to_string)]
        };
        let arc = Arc::new(s.clone());
        for n in &nets {
            for &seed in seeds {
                jobs.push(Job {
                    scn: arc.clone(),
                    path: path.clone(),
                    seed,
                    net: n.clone(),
                });
            }
        }
    }
    jobs
}

/// A table of pass rates per scenario (and per profile in a soak) and the first failing seed of each cell.
#[allow(clippy::type_complexity)]
pub fn summary(done: &[Done], soak: bool) -> Vec<String> {
    let mut cells: BTreeMap<(String, String), (u32, u32, u32, Option<&Done>)> = BTreeMap::new();
    for d in done {
        let key = (
            d.scenario.clone(),
            if soak { d.net.clone() } else { String::new() },
        );
        let e = cells.entry(key).or_insert((0, 0, 0, None));
        match d.result.as_str() {
            "pass" => e.0 += 1,
            "skip" => e.2 += 1,
            _ => {
                e.1 += 1;
                e.3.get_or_insert(d);
            }
        }
    }
    let mut out = Vec::new();
    for ((name, net), (p, f, sk, first)) in &cells {
        let label = if soak {
            format!("{name} [{net}]")
        } else {
            name.clone()
        };
        let mut l = format!("SUMMARY: {label}: {p}/{} pass", p + f + sk);
        if *sk > 0 {
            l += &format!(", {sk} skipped (needs_assets)");
        }
        if let Some(d) = first {
            l += &format!("; first failing seed {}: {}", d.seed, d.class);
            l += &format!("; replay: {}", d.rerun);
        }
        out.push(l);
    }
    out
}

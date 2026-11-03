//! `coop-sim checklist` and `coop-sim questwalk`.

use std::sync::Arc;

use crate::data::Data;
use crate::questwalk::{walk_all, Depth, QStatus, WalkOpts};

/// Runs a subcommand; returns the exit code.
pub fn run(cmd: &str, args: Vec<String>) -> i32 {
    match cmd {
        "questwalk" => questwalk(args),
        "checklist" => checklist(args),
        "diverge" | "shrink" | "gentest" => tools(cmd, args),
        _ => 2,
    }
}

/// Options of `coop-sim diverge|shrink|gentest`.
#[derive(Default)]
struct ToolArgs {
    targets: Vec<String>,
    seed: Option<u64>,
    seed2: Option<u64>,
    net: Option<String>,
    data: Option<String>,
    out: Option<String>,
    lag: f64,
    count: u64,
    run: bool,
}

fn tool_args(args: Vec<String>) -> Result<ToolArgs, String> {
    let mut t = ToolArgs {
        lag: 2.0,
        count: 1,
        ..ToolArgs::default()
    };
    let mut it = args.into_iter();
    while let Some(a) = it.next() {
        let mut val = |n: &str| it.next().ok_or(format!("{n} needs a value"));
        match a.as_str() {
            "--seed" => t.seed = Some(val("--seed")?.parse().map_err(|_| "bad --seed")?),
            "--seed2" => t.seed2 = Some(val("--seed2")?.parse().map_err(|_| "bad --seed2")?),
            "--net" => t.net = Some(val("--net")?),
            "--data" => t.data = Some(val("--data")?),
            "--out" => t.out = Some(val("--out")?),
            "--lag" => t.lag = val("--lag")?.parse().map_err(|_| "bad --lag")?,
            "--count" => t.count = val("--count")?.parse().map_err(|_| "bad --count")?,
            "--run" => t.run = true,
            x if x.starts_with("--") => return Err(format!("unknown option {x}")),
            x => t.targets.push(x.to_string()),
        }
    }
    if let Some(n) = &t.net {
        if !crate::scenario::PROFILES.contains(&n.as_str()) {
            return Err(format!("--net {n}: not a profile"));
        }
    }
    Ok(t)
}

fn open_data(data: Option<&str>) -> Result<Arc<Data>, String> {
    Data::locate(data)
        .and_then(|d| Data::open(&d))
        .map(Arc::new)
}

/// A recording from a `flight.jsonl` file, or from a recorded run of a scenario file.
fn recording_of(
    target: &str,
    seed: Option<u64>,
    t: &ToolArgs,
    d: &mut Option<Arc<Data>>,
) -> Result<crate::recorder::Recording, String> {
    use crate::recorder::Recording;
    let text = std::fs::read_to_string(target).map_err(|e| format!("{target}: {e}"))?;
    if target.ends_with(".jsonl") {
        return Recording::from_jsonl(&text).map_err(|e| format!("{target}: {e}"));
    }
    let scn = crate::scenario::Scenario::load(std::path::Path::new(target))?;
    if d.is_none() {
        *d = Some(open_data(t.data.as_deref())?);
    }
    let o = crate::sim::RunOpts {
        seed,
        net: t.net.clone(),
        path: target.to_string(),
        record: true,
        ..Default::default()
    };
    let r = crate::sim::run(&scn, d.as_ref().expect("opened"), &o);
    println!(
        "ran {} (seed {}): {}",
        scn.name,
        seed.unwrap_or(scn.seed),
        r.verdict.result
    );
    r.recording
        .ok_or_else(|| format!("{target}: the run made no recording"))
}

fn tools(cmd: &str, args: Vec<String>) -> i32 {
    let t = match tool_args(args) {
        Ok(t) => t,
        Err(e) => {
            println!("RESULT: FAIL config: {e}");
            return 2;
        }
    };
    let r = match cmd {
        "diverge" => tool_diverge(&t),
        "shrink" => tool_shrink(&t),
        _ => tool_gentest(&t),
    };
    match r {
        Ok(code) => code,
        Err(e) => {
            println!("RESULT: FAIL {cmd}: {e}");
            2
        }
    }
}

fn tool_diverge(t: &ToolArgs) -> Result<i32, String> {
    use crate::recorder::{diverge_peers, diverge_runs};
    let mut d = None;
    let found = match t.targets.as_slice() {
        [one] if t.seed2.is_none() => {
            let r = recording_of(one, t.seed, t, &mut d)?;
            diverge_peers(&r, t.lag)
        }
        [one] => {
            let a = recording_of(one, t.seed, t, &mut d)?;
            let b = recording_of(one, t.seed2, t, &mut d)?;
            diverge_runs(&a, &b)
        }
        [a, b] => {
            let a = recording_of(a, t.seed, t, &mut d)?;
            let b = recording_of(b, t.seed2.or(t.seed), t, &mut d)?;
            diverge_runs(&a, &b)
        }
        _ => return Err("diverge takes one or two runs (scenario or flight.jsonl)".into()),
    };
    Ok(match found {
        Some(x) => {
            println!("{}", x.line());
            println!("RESULT: FAIL diverged at second {}", x.second);
            1
        }
        None => {
            println!("RESULT: PASS no divergence");
            0
        }
    })
}

fn tool_shrink(t: &ToolArgs) -> Result<i32, String> {
    let [path] = t.targets.as_slice() else {
        return Err("shrink takes one scenario file".into());
    };
    let text = std::fs::read_to_string(path).map_err(|e| format!("{path}: {e}"))?;
    let name = std::path::Path::new(path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("scenario")
        .to_string();
    let d = open_data(t.data.as_deref())?;
    let o = crate::sim::RunOpts {
        seed: t.seed,
        net: t.net.clone(),
        path: path.clone(),
        ..Default::default()
    };
    let s = crate::recorder::shrink(&text, &name, &d, &o)?;
    let out = t
        .out
        .clone()
        .unwrap_or_else(|| path.trim_end_matches(".json").to_string() + ".min.json");
    std::fs::write(&out, &s.text).map_err(|e| format!("{out}: {e}"))?;
    println!(
        "shrank {} script lines to {} in {} runs, still failing as: {}",
        s.before, s.after, s.runs, s.failure
    );
    println!("RESULT: PASS wrote {out}");
    Ok(0)
}

fn tool_gentest(t: &ToolArgs) -> Result<i32, String> {
    let first = t.seed.unwrap_or(1);
    let d = if t.run {
        Some(open_data(t.data.as_deref())?)
    } else {
        None
    };
    let mut bad = 0;
    for seed in first..first + t.count.max(1) {
        let text = crate::recorder::gentest(seed);
        let name = format!("gen_{seed}");
        let scn = crate::scenario::Scenario::parse(&text, &name)?;
        match &t.out {
            Some(dir) => {
                std::fs::create_dir_all(dir).map_err(|e| format!("{dir}: {e}"))?;
                let p = std::path::Path::new(dir).join(format!("{name}.json"));
                std::fs::write(&p, &text).map_err(|e| format!("{}: {e}", p.display()))?;
                println!("wrote {}", p.display());
            }
            None if !t.run => print!("{text}"),
            None => {}
        }
        if let Some(d) = &d {
            let r = crate::sim::run(
                &scn,
                d,
                &crate::sim::RunOpts {
                    path: name.clone(),
                    ..Default::default()
                },
            );
            println!("{name}: {} {}", r.verdict.result, r.verdict.reason);
            if r.verdict.result != "pass" {
                bad += 1;
            }
        }
    }
    println!("RESULT: {}", if bad == 0 { "PASS" } else { "FAIL" });
    Ok(i32::from(bad > 0))
}

fn checklist(args: Vec<String>) -> i32 {
    use crate::checklist::{report, run::run as run_all, run::Opts};
    let mut o = Opts {
        seeds: Some(vec![1, 2]),
        jobs: std::thread::available_parallelism().map_or(4, |n| n.get().min(4)),
        ..Opts::default()
    };
    let (mut md, mut json) = (None, None);
    let mut it = args.into_iter();
    while let Some(a) = it.next() {
        let mut val = |n: &str| it.next().ok_or(format!("{n} needs a value"));
        let r: Result<(), String> = (|| {
            match a.as_str() {
                "--area" => o.area = Some(val("--area")?),
                "--seeds" => o.seeds = Some(crate::suite::parse_seeds(&val("--seeds")?)?),
                "--net" => o.net = Some(val("--net")?),
                "--peers" => o.peers = val("--peers")?.parse().map_err(|_| "bad --peers")?,
                "--jobs" => o.jobs = val("--jobs")?.parse().map_err(|_| "bad --jobs")?,
                "--data" => o.data = Some(val("--data")?),
                "--md" => md = Some(val("--md")?),
                "--json" => json = Some(val("--json")?),
                "--backend" => o.backend = val("--backend")?,
                "--real-out" => o.real_out = Some(val("--real-out")?.into()),
                "--manifest" => o.manifest = val("--manifest")?.into(),
                "--scenarios" => o.scenario_dirs.push(val("--scenarios")?.into()),
                "--quick" => {
                    o.quick = true;
                    o.seeds = None;
                }
                "--full" => o.full = true,
                "--no-questwalk" => o.no_questwalk = true,
                x => return Err(format!("unknown option {x}")),
            }
            Ok(())
        })();
        if let Err(e) = r {
            println!("RESULT: FAIL config: {e}");
            return 2;
        }
    }
    if let Some(n) = &o.net {
        if n != "all" && !crate::scenario::PROFILES.contains(&n.as_str()) {
            println!("RESULT: FAIL config: --net {n}: not a profile or all");
            return 2;
        }
    }
    let rep = match run_all(&o) {
        Ok(r) => r,
        Err(e) => {
            println!("RESULT: FAIL config: {e}");
            return 2;
        }
    };
    print!("{}", report::screen(&rep));
    if let Some(p) = md {
        if let Err(e) = std::fs::write(&p, report::markdown(&rep)) {
            println!("cannot write {p}: {e}");
            return 2;
        }
    }
    if let Some(p) = json {
        if let Err(e) = std::fs::write(&p, report::json(&rep)) {
            println!("cannot write {p}: {e}");
            return 2;
        }
    }
    let (_, total) = report::by_area(&rep);
    i32::from(!report::all_green(&total))
}

fn questwalk(args: Vec<String>) -> i32 {
    let mut o = WalkOpts::default();
    let mut data = None;
    let mut full = false;
    let mut it = args.into_iter();
    while let Some(a) = it.next() {
        let mut val = |n: &str| it.next().ok_or(format!("{n} needs a value"));
        let r: Result<(), String> = (|| {
            match a.as_str() {
                "--data" => data = Some(val("--data")?),
                "--net" => o.nets = val("--net")?.split(',').map(str::to_string).collect(),
                "--peers" => {
                    o.peers = val("--peers")?
                        .split(',')
                        .filter_map(|x| x.parse().ok())
                        .collect()
                }
                "--jobs" => o.jobs = val("--jobs")?.parse().map_err(|_| "bad --jobs")?,
                "--seed" => o.seed = val("--seed")?.parse().map_err(|_| "bad --seed")?,
                "--only" => o.filter = Some(val("--only")?),
                "--full" => full = true,
                x => return Err(format!("unknown option {x}")),
            }
            Ok(())
        })();
        if let Err(e) = r {
            println!("RESULT: FAIL config: {e}");
            return 2;
        }
    }
    if full {
        o.nets = crate::scenario::PROFILES
            .iter()
            .map(|s| s.to_string())
            .collect();
        o.peers = vec![2, 3, 4];
    }
    let d = match Data::locate(data.as_deref()).and_then(|d| Data::open(&d)) {
        Ok(d) => Arc::new(d),
        Err(e) => {
            println!("RESULT: FAIL config: {e}");
            return 2;
        }
    };
    let t0 = std::time::Instant::now();
    let (res, missing) = match walk_all(&d, &o) {
        Ok(r) => r,
        Err(e) => {
            println!("RESULT: FAIL config: {e}");
            return 2;
        }
    };
    let mut by: std::collections::BTreeMap<String, (u32, u32, u32, u32)> = Default::default();
    for r in &res {
        let e = by.entry(r.dlc.clone()).or_default();
        if r.status == QStatus::Pass {
            e.0 += 1
        } else {
            e.1 += 1
        }
        if r.depth == Depth::PolicyOnly {
            e.2 += 1
        }
        if !r.sp_gaps.is_empty() {
            e.3 += 1
        }
        if r.status == QStatus::Fail {
            println!(
                "FAIL {} ({}): {}",
                r.edid,
                r.plugin,
                r.fails
                    .iter()
                    .take(3)
                    .cloned()
                    .collect::<Vec<_>>()
                    .join(" | ")
            );
        }
    }
    for (k, (p, f, po, sp)) in &by {
        println!("{k}: {p} pass, {f} fail, {po} policy-only, {sp} with SP-GAPs");
    }
    let mut t = [0usize; 5];
    let mut gaps = std::collections::BTreeSet::new();
    for r in &res {
        t[0] += r.stages;
        t[1] += r.applied;
        t[2] += r.released;
        t[3] += r.refused;
        t[4] += r.per_player;
        gaps.extend(r.sp_gaps.keys().cloned());
    }
    println!(
        "stages {} applied {} released {} refused {} per-player {}; SP-gap functions {}",
        t[0],
        t[1],
        t[2],
        t[3],
        t[4],
        gaps.len()
    );
    if let Ok(d) = crate::questwalk::resolved_hits(&d) {
        println!(
            "policy rows whose (plugin, local id) resolves to the quest's runtime id: {} of {}",
            d.0, d.1
        );
    }
    if o.filter.is_some() {
        for r in &res {
            println!(
                "{} stages {} applied {} released {} refused {} gaps {:?}",
                r.edid,
                r.stages,
                r.applied,
                r.released,
                r.refused,
                r.sp_gaps.keys().collect::<Vec<_>>()
            );
        }
    }
    println!("not in load order: {}", missing.len());
    let bad: u32 = by.values().map(|v| v.1).sum();
    println!(
        "questwalk: {} quests in {:.1} s",
        res.len(),
        t0.elapsed().as_secs_f64()
    );
    println!("RESULT: {}", if bad == 0 { "PASS" } else { "FAIL" });
    i32::from(bad > 0)
}

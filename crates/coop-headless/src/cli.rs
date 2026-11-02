//! `coop-sim checklist` and `coop-sim questwalk`.

use std::sync::Arc;

use crate::data::Data;
use crate::questwalk::{walk_all, Depth, QStatus, WalkOpts};

/// Runs a subcommand; returns the exit code.
pub fn run(cmd: &str, args: Vec<String>) -> i32 {
    match cmd {
        "questwalk" => questwalk(args),
        "checklist" => checklist(args),
        _ => 2,
    }
}

fn checklist(args: Vec<String>) -> i32 {
    use crate::checklist::{report, run::run as run_all, run::Opts};
    let mut o = Opts { seeds: Some(vec![1, 2]), jobs: std::thread::available_parallelism().map_or(4, |n| n.get().min(4)), ..Opts::default() };
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
                "--peers" => o.peers = val("--peers")?.split(',').filter_map(|x| x.parse().ok()).collect(),
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
        o.nets = crate::scenario::PROFILES.iter().map(|s| s.to_string()).collect();
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
            println!("FAIL {} ({}): {}", r.edid, r.plugin, r.fails.iter().take(3).cloned().collect::<Vec<_>>().join(" | "));
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
    println!("stages {} applied {} released {} refused {} per-player {}; SP-gap functions {}", t[0], t[1], t[2], t[3], t[4], gaps.len());
    if let Ok(d) = crate::questwalk::resolved_hits(&d) {
        println!("policy rows whose (plugin, local id) resolves to the quest's runtime id: {} of {}", d.0, d.1);
    }
    if o.filter.is_some() {
        for r in &res {
            println!("{} stages {} applied {} released {} refused {} gaps {:?}", r.edid, r.stages, r.applied, r.released, r.refused, r.sp_gaps.keys().collect::<Vec<_>>());
        }
    }
    println!("not in load order: {}", missing.len());
    let bad: u32 = by.values().map(|v| v.1).sum();
    println!("questwalk: {} quests in {:.1} s", res.len(), t0.elapsed().as_secs_f64());
    println!("RESULT: {}", if bad == 0 { "PASS" } else { "FAIL" });
    i32::from(bad > 0)
}

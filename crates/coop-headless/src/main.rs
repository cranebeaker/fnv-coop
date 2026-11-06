// `coop-sim`: run tools/coop-run scenarios headlessly.
//
// ```text
// coop-sim run   SCENARIO.json [--seed N] [--net PROFILE] [--data DIR] [--out DIR] [--twice]
// coop-sim suite DIR [--seeds 1..50] [--jobs N] [--net PROFILE] [--soak] [--data DIR] [--out DIR] [--twice] [--rows]
// coop-sim checklist [--area X] [--seeds 1..N] [--quick] [--net all|PROFILE] [--peers 2|3|4] [--jobs N] [--full]
//                    [--backend world|engine] [--real-out DIR] [--data DIR] [--md out.md] [--json out.json]
// coop-sim questwalk [--only EDID|PLUGIN|DLC] [--net P,P] [--peers 2,3,4] [--full] [--jobs N] [--data DIR]
// coop-sim watch [--scenarios DIR] [--seeds 1..5] [--data DIR] [--net PROFILE] [--jobs N] [--once] [--no-clear]
//     rebuild and re-run when a crates/coop-* or world source or a scenario file changes (docs/FASTDEV.md)
// --talk-at-ready: `--talk` waits for coop_ready (default: first frame, as the viewer's does once its scene is loaded)
// --glue-fixed: container actions use Session::live_loot_ops (default: mirror the viewer's intents::loot)
// --record: the flight recorder (every peer's dump and shared hash per virtual second; flight.jsonl with --out)
// coop-sim diverge RUN [RUN2] [--seed N] [--seed2 N] [--net P] [--lag S] [--data DIR]
//     RUN is a scenario or a flight.jsonl; one RUN: host against each guest, two: run against run
// coop-sim shrink SCENARIO [--seed N] [--net P] [--out FILE] [--data DIR]
// coop-sim gentest [--seed N] [--count K] [--out DIR] [--run] [--data DIR]
// ```
// Exit code 0 pass, 1 fail, 2 usage or data error, 3 only skips (`needs_assets`).
// The last line is `RESULT: PASS|FAIL|SKIP ...`, in the format of `runner.py`.

use std::path::PathBuf;
use std::sync::Arc;

use coop_headless::data::Data;
use coop_headless::scenario::Scenario;
use coop_headless::sim::{run, RunOpts};
use coop_headless::suite::{load_dir, parse_seeds, plan, run_jobs, summary};
use coop_headless::verdict::result_line;

struct Args {
    cmd: String,
    target: String,
    seed: Option<u64>,
    seeds: Option<Vec<u64>>,
    net: Option<String>,
    data: Option<String>,
    out: Option<PathBuf>,
    jobs: usize,
    soak: bool,
    twice: bool,
    fps: u32,
    talk_at_ready: bool,
    fixed_glue: bool,
    record: bool,
    rows: bool,
    guests: Option<usize>,
    all_profiles: bool,
    gate_cpu: bool,
}

fn parse(mut a: impl Iterator<Item = String>) -> Result<Args, String> {
    let cmd = a
        .next()
        .ok_or("usage: coop-sim run|suite TARGET [options]")?;
    let target = a.next().ok_or("missing scenario file or folder")?;
    let mut r = Args {
        cmd,
        target,
        seed: None,
        seeds: None,
        net: None,
        data: None,
        out: None,
        jobs: 0,
        soak: false,
        twice: false,
        fps: 40,
        talk_at_ready: false,
        fixed_glue: false,
        record: false,
        rows: false,
        guests: None,
        all_profiles: false,
        gate_cpu: false,
    };
    while let Some(f) = a.next() {
        let mut val = |name: &str| a.next().ok_or(format!("{name} needs a value"));
        match f.as_str() {
            "--seed" => r.seed = Some(val("--seed")?.parse().map_err(|_| "bad --seed")?),
            "--seeds" => r.seeds = Some(parse_seeds(&val("--seeds")?)?),
            "--net" => r.net = Some(val("--net")?),
            "--data" => r.data = Some(val("--data")?),
            "--out" => r.out = Some(PathBuf::from(val("--out")?)),
            "--jobs" => r.jobs = val("--jobs")?.parse().map_err(|_| "bad --jobs")?,
            "--fps" => r.fps = val("--fps")?.parse().map_err(|_| "bad --fps")?,
            "--soak" => r.soak = true,
            "--twice" => r.twice = true,
            "--talk-at-ready" => r.talk_at_ready = true,
            "--glue-fixed" => r.fixed_glue = true,
            "--record" => r.record = true,
            "--rows" => r.rows = true,
            "--guests" => {
                r.guests = Some(
                    val("--guests")?
                        .parse()
                        .ok()
                        .filter(|n| (1..=coop_headless::scenario::MAX_GUESTS).contains(n))
                        .ok_or("--guests takes 1 to 3")?,
                )
            }
            "--all-profiles" => r.all_profiles = true,
            "--gate-cpu" => r.gate_cpu = true,
            other => return Err(format!("unknown option {other}")),
        }
    }
    if let Some(n) = &r.net {
        if !coop_headless::scenario::PROFILES.contains(&n.as_str()) {
            return Err(format!(
                "--net {n}: not a profile ({})",
                coop_headless::scenario::PROFILES.join(", ")
            ));
        }
    }
    Ok(r)
}

fn main() {
    std::process::exit(real_main());
}

fn real_main() -> i32 {
    if std::env::args().nth(1).as_deref() == Some("watch") {
        return match coop_headless::watch::parse_opts(std::env::args().skip(2)) {
            Ok(o) => coop_headless::watch::run(o),
            Err(e) => {
                println!("RESULT: FAIL config: {e}");
                2
            }
        };
    }
    if let Some(c) = std::env::args().nth(1).filter(|c| {
        matches!(
            c.as_str(),
            "checklist" | "questwalk" | "diverge" | "shrink" | "gentest"
        )
    }) {
        return coop_headless::cli::run(&c, std::env::args().skip(2).collect());
    }
    let args = match parse(std::env::args().skip(1)) {
        Ok(a) => a,
        Err(e) => {
            println!("RESULT: FAIL config: {e}");
            return 2;
        }
    };
    let t_open = std::time::Instant::now();
    let data = match Data::locate(args.data.as_deref()).and_then(|d| Data::open(&d)) {
        Ok(d) => {
            println!(
                "data: {} plugins, meshes {}, opened in {:.2} s",
                d.order.plugins().len(),
                if d.has_meshes { "present" } else { "absent" },
                t_open.elapsed().as_secs_f64()
            );
            Arc::new(d)
        }
        Err(e) => {
            println!("RESULT: FAIL config: {e}");
            return 2;
        }
    };
    match args.cmd.as_str() {
        "run" => one(&args, &data),
        "suite" => suite(&args, &data),
        "budget" => budget(&args, &data),
        other => {
            println!("RESULT: FAIL config: unknown command {other} (run or suite)");
            2
        }
    }
}

fn code(result: &str) -> i32 {
    match result {
        "pass" => 0,
        "skip" => 3,
        _ => 1,
    }
}

fn one(a: &Args, d: &Arc<Data>) -> i32 {
    let scn = match Scenario::load(std::path::Path::new(&a.target)) {
        Ok(s) => s,
        Err(e) => {
            println!("RESULT: FAIL config: {e}");
            return 2;
        }
    };
    let o = RunOpts {
        seed: a.seed,
        net: a.net.clone(),
        fps: a.fps,
        out: a.out.clone(),
        path: a.target.clone(),
        data_arg: a.data.clone(),
        talk_at_ready: a.talk_at_ready,
        fixed_glue: a.fixed_glue,
        record: a.record,
    };
    let t0 = std::time::Instant::now();
    let r = run(&scn, d, &o);
    println!(
        "ran {:.1} virtual seconds in {:.2} s",
        r.verdict
            .json
            .get("duration_s")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0),
        t0.elapsed().as_secs_f64()
    );
    if let Some(m) = r
        .verdict
        .json
        .get("modelled")
        .and_then(|m| m.as_arr())
        .filter(|m| !m.is_empty())
    {
        for x in m {
            println!("modelled: {}", x.as_str().unwrap_or(""));
        }
    }
    println!("dump hash {:016x}", r.hash);
    if a.twice {
        let r2 = run(&scn, d, &RunOpts { out: None, ..o });
        let same = r2.hash == r.hash;
        println!(
            "determinism: {}",
            if same {
                "same seed twice gave identical dumps"
            } else {
                "DIFFERENT dumps for the same seed"
            }
        );
        if !same {
            println!(
                "RESULT: FAIL harness: {} is not deterministic for this seed",
                scn.name
            );
            return 1;
        }
    }
    println!("{}", result_line(&r.verdict));
    code(&r.verdict.result)
}

/// `coop-sim budget SCENARIO [--guests N] [--all-profiles] [--gate-cpu]`: bandwidth and CPU per peer.
fn budget(a: &Args, d: &Arc<Data>) -> i32 {
    use coop_headless::multi::{run_multi, Limits, MultiOpts};
    let mut scn = match Scenario::load(std::path::Path::new(&a.target)) {
        Ok(s) => s,
        Err(e) => {
            println!("RESULT: FAIL config: {e}");
            return 2;
        }
    };
    if let Some(n) = a.guests {
        if scn.guests.len() != n {
            let side = scn.guest.clone();
            scn.guests = vec![side; n];
        }
    }
    let nets: Vec<Option<String>> = if a.all_profiles {
        coop_headless::scenario::PROFILES
            .iter()
            .filter(|p| **p != "partition")
            .map(|p| Some(p.to_string()))
            .collect()
    } else {
        vec![a.net.clone()]
    };
    let lim = Limits::default();
    let (mut bad, mut skipped) = (0, 0);
    for net in nets {
        let o = RunOpts {
            seed: a.seed,
            net: net.clone(),
            fps: a.fps,
            out: None,
            path: a.target.clone(),
            data_arg: a.data.clone(),
            talk_at_ready: a.talk_at_ready,
            fixed_glue: a.fixed_glue,
            record: false,
        };
        let r = run_multi(&scn, d, &MultiOpts { run: o });
        if r.verdict.result == "skip" {
            println!("RESULT: SKIP needs_assets {}", r.verdict.reason);
            skipped += 1;
            continue;
        }
        let nn = net.unwrap_or_else(|| scn.network.clone());
        println!("== {} peers, network {nn} ==", scn.guest_count() + 1);
        print!("{}", r.budget.report());
        let v = r.budget.violations(&lim, a.gate_cpu);
        for x in &v {
            println!("OVER BUDGET: {x}");
        }
        println!("{}", result_line(&r.verdict));
        if !v.is_empty() || r.verdict.result == "fail" {
            bad += 1;
        }
    }
    if bad > 0 {
        println!("RESULT: FAIL budget or checks failed on {bad} profile(s)");
        1
    } else if skipped > 0 {
        3
    } else {
        println!("RESULT: PASS within budget");
        0
    }
}

fn suite(a: &Args, d: &Arc<Data>) -> i32 {
    let scns = match load_dir(std::path::Path::new(&a.target)) {
        Ok(s) => s,
        Err(e) => {
            println!("RESULT: FAIL config: {e}");
            return 2;
        }
    };
    let threads = if a.jobs > 0 {
        a.jobs
    } else {
        std::thread::available_parallelism().map_or(2, |n| n.get().min(8))
    };
    let jobs = plan(&scns, a.seeds.as_deref(), a.soak, a.net.as_deref());
    let t0 = std::time::Instant::now();
    let done = run_jobs(
        d,
        jobs,
        threads,
        a.out.clone(),
        a.twice,
        a.data.clone(),
        a.talk_at_ready,
        a.fixed_glue,
    );
    let (mut pass, mut skip, mut nondet) = (0, 0, 0);
    for x in &done {
        if a.rows {
            println!(
                "ROW\t{}\t{}\t{}\t{}\t{:.3}\t{}",
                x.scenario, x.seed, x.result, x.class, x.wall_s, x.rerun
            );
        }
        if !a.soak && a.seeds.as_ref().map_or(true, |s| s.len() <= 3) {
            println!("{}", x.line);
        }
        match x.result.as_str() {
            "pass" => pass += 1,
            "skip" => skip += 1,
            _ => {}
        }
        if x.hash2.is_some_and(|h| h != x.hash) {
            nondet += 1;
            println!(
                "NONDETERMINISTIC: {} seed {} ({}): {}",
                x.scenario, x.seed, x.net, x.rerun
            );
        }
    }
    for l in summary(&done, a.soak) {
        println!("{l}");
    }
    let virt: f64 = done.iter().map(|x| x.virt_s).sum();
    let fail = done.len() - pass - skip;
    println!(
        "SUMMARY: {} runs, {pass} pass, {fail} fail, {skip} skipped; {virt:.0} virtual seconds in {:.1} s wall on {threads} threads{}",
        done.len(),
        t0.elapsed().as_secs_f64(),
        if a.twice { format!("; {nondet} nondeterministic") } else { String::new() }
    );
    let bad = fail > 0 || nondet > 0;
    println!(
        "RESULT: {}",
        if bad {
            "FAIL"
        } else if pass == 0 && skip > 0 {
            "SKIP"
        } else {
            "PASS"
        }
    );
    if bad {
        1
    } else if pass == 0 && skip > 0 {
        3
    } else {
        0
    }
}

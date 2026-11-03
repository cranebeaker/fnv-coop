//! `coop-sim watch`: rebuild and re-run the scenarios whenever a source file changes.
//!
//! ```text
//! coop-sim watch [--scenarios DIR] [--seeds 1..5] [--data DIR] [--net PROFILE] [--jobs N]
//!                [--root REPO] [--no-clear] [--once] [--poll-ms 300]
//! ```
//!
//! Polls file mtimes (std only) under `crates/coop-*/src`, `crates/world/src` (and the
//! `Cargo.toml` of those) and the scenario folder. A source change runs an incremental
//! `cargo build -p coop-headless --example sim`, then the whole suite; a scenario file
//! change re-runs just that file. The suite runs in a child process (`examples/sim`, which is
//! `main.rs` again) because this supervisor, `coop-sim` itself, is a running image that
//! cargo could not overwrite on Windows. Build errors are shown in compact form and the
//! last table stays up. Nothing is deleted or written outside cargo's target dir.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, Instant, SystemTime};

/// Files and their modification times.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Snapshot(pub BTreeMap<PathBuf, SystemTime>);

impl Snapshot {
    /// Walks `roots` (files or folders) for files whose name ends in one of `exts` (or is `Cargo.toml`).
    pub fn take(roots: &[PathBuf], exts: &[&str]) -> Snapshot {
        let mut m = BTreeMap::new();
        for r in roots {
            walk(r, exts, &mut m, 0);
        }
        Snapshot(m)
    }

    /// Files that appeared, changed or vanished going from `self` to `newer`.
    pub fn changed(&self, newer: &Snapshot) -> Vec<PathBuf> {
        let mut out = Vec::new();
        for (p, t) in &newer.0 {
            if self.0.get(p) != Some(t) {
                out.push(p.clone());
            }
        }
        out.extend(self.0.keys().filter(|p| !newer.0.contains_key(*p)).cloned());
        out.sort();
        out
    }
}

fn walk(p: &Path, exts: &[&str], out: &mut BTreeMap<PathBuf, SystemTime>, depth: u32) {
    let Ok(md) = std::fs::metadata(p) else { return };
    if md.is_dir() {
        if depth > 12 {
            return;
        }
        if let Ok(rd) = std::fs::read_dir(p) {
            for e in rd.flatten() {
                walk(&e.path(), exts, out, depth + 1);
            }
        }
    } else {
        let name = p
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();
        if name == "Cargo.toml" || exts.iter().any(|x| name.ends_with(x)) {
            if let Ok(t) = md.modified() {
                out.insert(p.to_path_buf(), t);
            }
        }
    }
}

/// The repository root: the first folder from `start` upward that has `crates/coop-session`.
pub fn find_root(start: &Path) -> Option<PathBuf> {
    start
        .ancestors()
        .find(|d| d.join("crates").join("coop-session").is_dir())
        .map(Path::to_path_buf)
}

/// The folders whose Rust sources trigger a rebuild: `crates/coop-*` and `crates/world`.
pub fn source_roots(root: &Path) -> Vec<PathBuf> {
    let mut v = Vec::new();
    if let Ok(rd) = std::fs::read_dir(root.join("crates")) {
        for e in rd.flatten() {
            let n = e.file_name().to_string_lossy().to_string();
            if n.starts_with("coop-") || n == "world" {
                v.push(e.path().join("src"));
                v.push(e.path().join("Cargo.toml"));
            }
        }
    }
    v.sort();
    v
}

/// One finished run, as `suite --rows` prints it.
#[derive(Clone, Debug, PartialEq)]
pub struct Row {
    /// Scenario name.
    pub scenario: String,
    /// Seed.
    pub seed: u64,
    /// `pass`, `fail` or `skip`.
    pub result: String,
    /// Failure class.
    pub class: String,
    /// Wall seconds.
    pub wall_s: f64,
    /// Command that replays it.
    pub rerun: String,
}

/// Parses `ROW<TAB>scenario<TAB>seed<TAB>result<TAB>class<TAB>wall<TAB>rerun`.
pub fn parse_row(line: &str) -> Option<Row> {
    let f: Vec<&str> = line.trim_end().split('\t').collect();
    if f.len() < 7 || f[0] != "ROW" {
        return None;
    }
    Some(Row {
        scenario: f[1].into(),
        seed: f[2].parse().ok()?,
        result: f[3].into(),
        class: f[4].into(),
        wall_s: f[5].parse().ok()?,
        rerun: f[6..].join("\t"),
    })
}

/// Error lines of `cargo build --message-format=short` output, at most `max`, plus a count of the rest.
pub fn compact_errors(text: &str, max: usize) -> Vec<String> {
    let errs: Vec<&str> = text
        .lines()
        .filter(|l| {
            let t = l.trim_start();
            (l.contains(": error") || t.starts_with("error"))
                && !t.starts_with("error: could not compile")
                && !t.starts_with("error: aborting")
        })
        .collect();
    let mut out: Vec<String> = errs
        .iter()
        .take(max)
        .map(|l| l.trim().to_string())
        .collect();
    if errs.len() > max {
        out.push(format!("... and {} more", errs.len() - max));
    }
    if out.is_empty() {
        out.extend(
            text.lines()
                .rev()
                .take(4)
                .map(|l| l.trim().to_string())
                .collect::<Vec<_>>()
                .into_iter()
                .rev(),
        );
    }
    out
}

/// The one-screen table: a line per scenario, one mark per seed (`.` pass, `F` fail, `s` skip).
pub fn render_table(rows: &[Row]) -> Vec<String> {
    let mut by: BTreeMap<&str, Vec<&Row>> = BTreeMap::new();
    for r in rows {
        by.entry(&r.scenario).or_default().push(r);
    }
    let w = by.keys().map(|k| k.len()).max().unwrap_or(8).max(8);
    let mut out = vec![format!(
        "{:<w$}  {:<24} {:>6}  {:>7}  {}",
        "scenario", "seeds", "pass", "wall", "first failure"
    )];
    for (name, mut rs) in by {
        rs.sort_by_key(|r| r.seed);
        let marks: String = rs
            .iter()
            .map(|r| match r.result.as_str() {
                "pass" => ".",
                "skip" => "s",
                _ => "F",
            })
            .collect::<Vec<_>>()
            .join(" ");
        let pass = rs.iter().filter(|r| r.result == "pass").count();
        let skip = rs.iter().filter(|r| r.result == "skip").count();
        let wall: f64 = rs.iter().map(|r| r.wall_s).sum();
        let first = rs.iter().find(|r| r.result != "pass" && r.result != "skip");
        let mut l = format!(
            "{:<w$}  {:<24} {:>6}  {:>6.2}s",
            name,
            marks,
            format!("{pass}/{}", rs.len() - skip),
            wall
        );
        if let Some(f) = first {
            l += &format!("  seed {} {}: {}", f.seed, f.class, f.rerun);
        }
        out.push(l);
    }
    out
}

/// Options of the watch loop.
pub struct Opts {
    /// Repository root.
    pub root: PathBuf,
    /// Scenario folder.
    pub scenarios: PathBuf,
    /// `--seeds` text.
    pub seeds: Option<String>,
    /// `--data`.
    pub data: Option<String>,
    /// `--net`.
    pub net: Option<String>,
    /// `--jobs`.
    pub jobs: Option<usize>,
    /// Clear the screen before each table.
    pub clear: bool,
    /// One cycle, then exit with its result.
    pub once: bool,
    /// Polling interval.
    pub poll: Duration,
}

/// Parses the arguments after `watch`.
pub fn parse_opts(mut a: impl Iterator<Item = String>) -> Result<Opts, String> {
    let mut root = None;
    let mut o = Opts {
        root: PathBuf::new(),
        scenarios: PathBuf::new(),
        seeds: None,
        data: None,
        net: None,
        jobs: None,
        clear: true,
        once: false,
        poll: Duration::from_millis(300),
    };
    let mut scenarios = None;
    while let Some(f) = a.next() {
        let mut val = |n: &str| a.next().ok_or(format!("{n} needs a value"));
        match f.as_str() {
            "--scenarios" => scenarios = Some(PathBuf::from(val("--scenarios")?)),
            "--seeds" => {
                let v = val("--seeds")?;
                crate::suite::parse_seeds(&v)?;
                o.seeds = Some(v);
            }
            "--data" => o.data = Some(val("--data")?),
            "--net" => o.net = Some(val("--net")?),
            "--jobs" => o.jobs = Some(val("--jobs")?.parse().map_err(|_| "bad --jobs")?),
            "--root" => root = Some(PathBuf::from(val("--root")?)),
            "--poll-ms" => {
                o.poll =
                    Duration::from_millis(val("--poll-ms")?.parse().map_err(|_| "bad --poll-ms")?)
            }
            "--no-clear" => o.clear = false,
            "--once" => o.once = true,
            other => return Err(format!("unknown option {other}")),
        }
    }
    o.root = match root {
        Some(r) => r,
        None => find_root(&std::env::current_dir().map_err(|e| e.to_string())?)
            .ok_or("run it inside the repository, or give --root")?,
    };
    o.scenarios =
        scenarios.unwrap_or_else(|| o.root.join("tools").join("coop-run").join("scenarios"));
    Ok(o)
}

/// Where cargo puts `examples/sim`, from the running executable (`target/debug/coop-sim`).
pub fn sim_path(exe: &Path) -> Result<PathBuf, String> {
    let dir = exe.parent().ok_or("no executable folder")?;
    let base = match dir.file_name().and_then(|n| n.to_str()) {
        Some("deps") => dir.parent().ok_or("no profile folder")?.to_path_buf(),
        Some("examples") => return Err(
            "run `coop-sim watch` from target/<profile>/coop-sim, not from the example it rebuilds"
                .into(),
        ),
        _ => dir.to_path_buf(),
    };
    Ok(base
        .join("examples")
        .join(format!("sim{}", std::env::consts::EXE_SUFFIX)))
}

fn cargo() -> Command {
    Command::new(std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into()))
}

/// Builds `examples/sim` incrementally; `Ok(seconds)` or the compact errors.
pub fn build(root: &Path) -> Result<f64, Vec<String>> {
    let t0 = Instant::now();
    let out = cargo()
        .current_dir(root)
        .args([
            "build",
            "-p",
            "coop-headless",
            "--example",
            "sim",
            "--message-format=short",
            "--color",
            "never",
        ])
        .output()
        .map_err(|e| vec![format!("cannot run cargo: {e}")])?;
    if out.status.success() {
        Ok(t0.elapsed().as_secs_f64())
    } else {
        Err(compact_errors(&String::from_utf8_lossy(&out.stderr), 12))
    }
}

/// Runs the suite for `target` in the built example; returns (rows, other output lines, exit code, seconds).
pub fn run_suite(sim: &Path, o: &Opts, target: &Path) -> (Vec<Row>, Vec<String>, i32, f64) {
    let t0 = Instant::now();
    let mut c = Command::new(sim);
    c.arg("suite").arg(target).arg("--rows");
    if let Some(s) = &o.seeds {
        c.args(["--seeds", s]);
    }
    if let Some(d) = &o.data {
        c.args(["--data", d]);
    }
    if let Some(n) = &o.net {
        c.args(["--net", n]);
    }
    if let Some(j) = o.jobs {
        c.args(["--jobs", &j.to_string()]);
    }
    match c.output() {
        Ok(out) => {
            let text = String::from_utf8_lossy(&out.stdout).to_string();
            let rows = text.lines().filter_map(parse_row).collect();
            let rest = text
                .lines()
                .filter(|l| {
                    !l.starts_with("ROW\t")
                        && (l.starts_with("data:")
                            || l.starts_with("RESULT: FAIL config")
                            || l.starts_with("SUMMARY: ") && l.contains("runs"))
                })
                .map(str::to_string)
                .collect();
            (
                rows,
                rest,
                out.status.code().unwrap_or(1),
                t0.elapsed().as_secs_f64(),
            )
        }
        Err(e) => (
            Vec::new(),
            vec![format!("cannot run {}: {e}", sim.display())],
            1,
            t0.elapsed().as_secs_f64(),
        ),
    }
}

fn clock() -> String {
    let s = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map_or(0, |d| d.as_secs());
    format!("{:02}:{:02}:{:02}", (s / 3600) % 24, (s / 60) % 60, s % 60)
}

/// The watch loop. Returns the exit code (only `--once` returns).
pub fn run(o: Opts) -> i32 {
    let exe = match std::env::current_exe()
        .map_err(|e| e.to_string())
        .and_then(|e| sim_path(&e))
    {
        Ok(p) => p,
        Err(e) => {
            println!("RESULT: FAIL config: {e}");
            return 2;
        }
    };
    let src_roots = source_roots(&o.root);
    let scn_roots = vec![o.scenarios.clone()];
    let take_src = || Snapshot::take(&src_roots, &[".rs"]);
    let take_scn = || Snapshot::take(&scn_roots, &[".json"]);
    let (mut src, mut scn) = (take_src(), take_scn());
    let mut targets = vec![o.scenarios.clone()];
    let mut changed: Vec<String> = vec!["(start)".into()];
    let mut last_rows: Vec<Row> = Vec::new();
    let mut have_binary = false;
    loop {
        let mut head = vec![format!(
            "coop-sim watch  {}  changed: {}",
            clock(),
            summarize(&changed)
        )];
        let mut stale = false;
        let rebuild = !have_binary
            || changed
                .iter()
                .any(|c| c.ends_with(".rs") || c.ends_with("Cargo.toml") || c == "(start)");
        let mut code = 0;
        if rebuild {
            match build(&o.root) {
                Ok(s) => {
                    have_binary = true;
                    head.push(format!("build {s:.1} s"));
                }
                Err(errs) => {
                    stale = true;
                    head.push("BUILD FAILED (showing the last good table below)".into());
                    head.extend(errs.into_iter().map(|e| format!("  {e}")));
                    code = 1;
                }
            }
        }
        let mut rows_now = last_rows.clone();
        if !stale {
            for t in &targets {
                let (rows, rest, c, secs) = run_suite(&exe, &o, t);
                head.extend(rest);
                head.push(format!("suite {secs:.1} s"));
                if rows.is_empty() {
                    code = code.max(if c == 0 { 1 } else { c });
                }
                if t.is_dir() {
                    rows_now = rows;
                } else {
                    let names: Vec<String> = rows.iter().map(|r| r.scenario.clone()).collect();
                    rows_now.retain(|r| !names.contains(&r.scenario));
                    rows_now.extend(rows);
                }
                code = code.max(if c == 3 { 0 } else { c });
            }
            last_rows = rows_now.clone();
        }
        if o.clear {
            print!("\x1b[2J\x1b[H");
        }
        for l in &head {
            println!("{l}");
        }
        for l in render_table(&last_rows) {
            println!("{l}");
        }
        let bad = last_rows
            .iter()
            .filter(|r| r.result != "pass" && r.result != "skip")
            .count();
        let good = bad == 0 && !stale && !last_rows.is_empty();
        println!(
            "RESULT: {} ({} runs, {bad} failing){}",
            if good { "PASS" } else { "FAIL" },
            last_rows.len(),
            if stale {
                ", stale: build failed"
            } else if last_rows.is_empty() {
                ", no runs"
            } else {
                ""
            }
        );
        if o.once {
            return if good { 0 } else { code.max(1) };
        }
        // wait for the next change
        loop {
            std::thread::sleep(o.poll);
            let (ns, nc) = (take_src(), take_scn());
            if ns == src && nc == scn {
                continue;
            }
            std::thread::sleep(Duration::from_millis(250)); // an editor's second write
            let (ns, nc) = (take_src(), take_scn());
            let src_changed = src.changed(&ns);
            let scn_changed = scn.changed(&nc);
            src = ns;
            scn = nc;
            changed = src_changed
                .iter()
                .chain(&scn_changed)
                .map(|p| {
                    p.strip_prefix(&o.root)
                        .unwrap_or(p)
                        .display()
                        .to_string()
                        .replace('\\', "/")
                })
                .collect();
            targets = if !src_changed.is_empty() {
                vec![o.scenarios.clone()]
            } else {
                scn_changed.into_iter().filter(|p| p.exists()).collect()
            };
            if !targets.is_empty() || !changed.is_empty() {
                break;
            }
        }
        if targets.is_empty() {
            targets = vec![o.scenarios.clone()];
        }
    }
}

fn summarize(changed: &[String]) -> String {
    match changed {
        [] => "-".into(),
        [one] => one.clone(),
        [first, rest @ ..] => format!("{first} (+{})", rest.len()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn row(s: &str, seed: u64, r: &str) -> Row {
        Row {
            scenario: s.into(),
            seed,
            result: r.into(),
            class: if r == "fail" {
                "desync".into()
            } else {
                String::new()
            },
            wall_s: 0.5,
            rerun: format!("coop-sim run {s}.json --seed {seed}"),
        }
    }

    #[test]
    fn rows_parse_and_render_one_line_per_scenario() {
        let r = parse_row("ROW\tlive_loot\t3\tfail\tdesync\t0.25\tcoop-sim run a.json --seed 3\n")
            .unwrap();
        assert_eq!(
            (r.seed, r.result.as_str(), r.rerun.as_str()),
            (3, "fail", "coop-sim run a.json --seed 3")
        );
        assert!(parse_row("RESULT: PASS").is_none() && parse_row("ROW\tx\tnope").is_none());
        let t = render_table(&[
            row("a", 1, "pass"),
            row("a", 2, "fail"),
            row("b", 1, "pass"),
            row("c", 1, "skip"),
        ]);
        assert_eq!(t.len(), 4, "{t:?}");
        assert!(
            t[1].starts_with("a ")
                && t[1].contains(". F")
                && t[1].contains("1/2")
                && t[1].contains("seed 2 desync: coop-sim run a.json --seed 2"),
            "{}",
            t[1]
        );
        assert!(t[2].contains("1/1") && t[3].contains("0/0") && t[3].contains('s'));
    }

    #[test]
    fn snapshots_see_changes_new_and_removed_files() {
        let d = std::env::temp_dir().join(format!("coop-watch-test-{}", std::process::id()));
        let src = d.join("crates").join("coop-x").join("src");
        std::fs::create_dir_all(&src).unwrap();
        std::fs::create_dir_all(d.join("crates").join("coop-session")).unwrap();
        std::fs::write(src.join("a.rs"), "1").unwrap();
        std::fs::write(src.join("notes.txt"), "1").unwrap();
        assert_eq!(find_root(&src).as_deref(), Some(d.as_path()));
        let roots = source_roots(&d);
        let s1 = Snapshot::take(&roots, &[".rs"]);
        assert_eq!(s1.0.len(), 1, "only .rs files count");
        assert!(s1.changed(&s1).is_empty());
        std::fs::write(src.join("b.rs"), "2").unwrap();
        let s2 = Snapshot::take(&roots, &[".rs"]);
        assert_eq!(s1.changed(&s2), vec![src.join("b.rs")]);
        std::fs::remove_file(src.join("b.rs")).unwrap();
        assert_eq!(
            s2.changed(&Snapshot::take(&roots, &[".rs"])),
            vec![src.join("b.rs")]
        );
        // best effort cleanup of this test's own files
        let _ = std::fs::remove_file(src.join("a.rs"));
        let _ = std::fs::remove_file(src.join("notes.txt"));
        let _ = std::fs::remove_dir(&src);
        let _ = std::fs::remove_dir(src.parent().unwrap());
        let _ = std::fs::remove_dir(d.join("crates").join("coop-session"));
        let _ = std::fs::remove_dir(d.join("crates").join("coop-x"));
        let _ = std::fs::remove_dir(d.join("crates"));
        let _ = std::fs::remove_dir(&d);
    }

    #[test]
    fn cargo_errors_are_compacted() {
        let text = "   Compiling x\ncrates/a/src/lib.rs:3:5: error[E0425]: cannot find value `q`\ncrates/a/src/lib.rs:9:1: warning: unused\nerror: could not compile `a` (lib) due to 1 previous error\n";
        assert_eq!(
            compact_errors(text, 5),
            vec!["crates/a/src/lib.rs:3:5: error[E0425]: cannot find value `q`".to_string()]
        );
        let many: String = (0..20)
            .map(|i| format!("f.rs:{i}:1: error: e{i}\n"))
            .collect();
        let c = compact_errors(&many, 3);
        assert_eq!((c.len(), c[3].as_str()), (4, "... and 17 more"));
    }

    #[test]
    fn sim_path_follows_the_profile_folder() {
        let p = sim_path(Path::new("/t/debug/coop-sim")).unwrap();
        assert!(
            p.starts_with("/t/debug/examples")
                && p.file_name().unwrap().to_string_lossy().starts_with("sim")
        );
        assert!(sim_path(Path::new("/t/debug/deps/coop_sim-abc"))
            .unwrap()
            .starts_with("/t/debug/examples"));
        assert!(sim_path(Path::new("/t/debug/examples/sim")).is_err());
    }

    #[test]
    fn options_parse() {
        let o = parse_opts(
            [
                "--root",
                "/r",
                "--seeds",
                "1..5",
                "--no-clear",
                "--once",
                "--jobs",
                "2",
            ]
            .map(String::from)
            .into_iter(),
        )
        .unwrap();
        assert_eq!(
            (o.seeds.as_deref(), o.clear, o.once, o.jobs),
            (Some("1..5"), false, true, Some(2))
        );
        assert!(o.scenarios.ends_with("scenarios"));
        assert!(parse_opts(["--seeds", "x"].map(String::from).into_iter()).is_err());
    }
}

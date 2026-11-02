//! nv-pure: the automatic oracle check for pure translations (#182,
//! docs/HANDOFF.md next task 10).
//!
//! ```text
//! nv-pure list --purity <purity.tsv> [--out <pure.tsv>]
//! nv-pure check [--list <pure.tsv>] [--limit N] [--only addr,addr] [--count N]
//!               [--exe-timeout S] [--slow-ms MS] [--rust-timeout S] [--fpcw <hex>]
//! nv-pure x87 [--list <pure.tsv>] [--count N]
//! ```
//!
//! Time limits: a vector nv-call hasn't finished in `--exe-timeout` seconds
//! (default 10) is abandoned and nv-call restarted after it; one the exe took
//! over `--slow-ms` (default 1000) is not run on the engine side; and the
//! engine running one vector past `--rust-timeout` seconds (default 120)
//! stops the check with an error naming it. All three are reported.
//!
//! `list` joins the exe's purity facts (research/ghidra/NvPurity.java, run on
//! your own project) with the translation markers and `entry!` signatures
//! of the engine crates (crates/engine, crates/engine-*) and writes the
//! checkable functions (addresses and signatures only).
//!
//! `check` runs each listed function on the real CPU through `nv-call` and in
//! the engine with the same inputs and compares the results bit for bit. It
//! needs `NV_ORACLE_EXE` (an unpacked FalloutNV.exe 1.4.0.525); without it,
//! it prints that it skipped and exits 0, so it is safe in CI. `NV_CALL`
//! overrides the nv-call.exe path; `NV_ORACLE_OUT` the private output folder
//! (default `%USERPROFILE%\nv-re\oracle\pure`). Exit status 1 when any
//! vector fails.
//!
//! `check --fpcw 0x007F` runs both sides under that x87 control word: nv-call
//! loads it before each call and each engine gets it as its main thread's
//! word, as after Direct3D created the device (24-bit precision); the
//! translations that compute through the x87 helpers follow it.
//!
//! `x87` checks the x87 helpers themselves (one-instruction stubs) and
//! helper-built models of a few pure x87 functions against the CPU under
//! several control words (`x87check.rs`).

mod check;
mod json;
mod list;
mod purity;
mod source;
mod vectors;
mod x87check;

use std::path::{Path, PathBuf};
use std::process::ExitCode;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .unwrap_or_else(|_| PathBuf::from("."))
}

fn arg(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn usage() -> ExitCode {
    eprintln!("usage: nv-pure list --purity <purity.tsv> [--out <pure.tsv>]");
    eprintln!("       nv-pure check [--list <pure.tsv>] [--limit N] [--only addr,...] [--count N]");
    eprintln!(
        "                     [--exe-timeout S] [--slow-ms MS] [--rust-timeout S] [--fpcw <hex>]"
    );
    eprintln!("       nv-pure x87 [--list <pure.tsv>] [--count N]");
    ExitCode::from(2)
}

fn default_list() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("pure.tsv")
}

fn cmd_list(args: &[String]) -> Result<ExitCode, String> {
    let Some(purity_path) = arg(args, "--purity") else {
        return Ok(usage());
    };
    let out = arg(args, "--out")
        .map(PathBuf::from)
        .unwrap_or_else(default_list);
    let text = std::fs::read_to_string(&purity_path).map_err(|e| format!("{purity_path}: {e}"))?;
    let facts = purity::parse(&text)?;
    let src = source::scan(&repo_root());
    let l = list::build(&src, &facts);
    std::fs::write(&out, list::to_tsv(&l.pure)).map_err(|e| format!("{}: {e}", out.display()))?;
    println!(
        "exe functions {}, translated {}, checkable {}",
        facts.len(),
        src.translated.len(),
        l.pure.len()
    );
    for (k, n) in &l.skipped {
        println!("  skipped {n:6}  {k:?}");
    }
    let mut cc = std::collections::BTreeMap::new();
    for p in &l.pure {
        *cc.entry(p.cc).or_insert(0usize) += 1;
    }
    println!("  conventions {cc:?}");
    for (a, why) in l.convention_mismatches.iter().take(20) {
        println!("  convention mismatch {a:08x}: {why}");
    }
    println!("wrote {}", out.display());
    Ok(ExitCode::SUCCESS)
}

fn cmd_check(args: &[String]) -> Result<ExitCode, String> {
    let Some(exe) = std::env::var_os("NV_ORACLE_EXE") else {
        println!("nv-pure check: skipped (set NV_ORACLE_EXE to an unpacked FalloutNV.exe 1.4.0.525 to run it)");
        return Ok(ExitCode::SUCCESS);
    };
    let list_path = arg(args, "--list")
        .map(PathBuf::from)
        .unwrap_or_else(default_list);
    let text =
        std::fs::read_to_string(&list_path).map_err(|e| format!("{}: {e}", list_path.display()))?;
    let mut all = list::from_tsv(&text)?;
    if let Some(only) = arg(args, "--only") {
        let want: Vec<u32> = only
            .split(',')
            .filter_map(|s| u32::from_str_radix(s.trim().trim_start_matches("0x"), 16).ok())
            .collect();
        all.retain(|p| want.contains(&p.addr));
    }
    if let Some(n) = arg(args, "--limit").and_then(|s| s.parse::<usize>().ok()) {
        all.truncate(n);
    }
    let per_function = arg(args, "--count")
        .and_then(|s| s.parse().ok())
        .unwrap_or(64);
    let nv_call = nv_call_path();
    let out_dir = std::env::var_os("NV_ORACLE_OUT")
        .map(PathBuf::from)
        .unwrap_or_else(check::default_out_dir);
    let secs = |name: &str, default: u64| {
        std::time::Duration::from_secs(
            arg(args, name)
                .and_then(|s| s.parse().ok())
                .unwrap_or(default),
        )
    };
    let fpcw = match arg(args, "--fpcw") {
        Some(s) => Some(
            u16::from_str_radix(
                s.trim().trim_start_matches("0x").trim_start_matches("0X"),
                16,
            )
            .map_err(|e| format!("--fpcw {s}: {e}"))?,
        ),
        None => None,
    };
    let o = check::Options {
        exe: PathBuf::from(exe),
        nv_call,
        out_dir,
        per_function,
        exe_timeout: secs("--exe-timeout", 10),
        slow: std::time::Duration::from_millis(
            arg(args, "--slow-ms")
                .and_then(|s| s.parse().ok())
                .unwrap_or(1000),
        ),
        rust_timeout: secs("--rust-timeout", 120),
        fpcw,
    };
    if let Some(word) = fpcw {
        println!("control word 0x{word:04X} on both sides");
    }
    let r = check::run(&all, &o)?;

    let vectors: usize = r.counts.values().sum();
    println!(
        "functions run {}, skipped {}, vectors {vectors}",
        r.functions,
        r.skipped.len()
    );
    for (k, n) in &r.counts {
        println!("  {k:?}: {n}");
    }
    println!(
        "  functions with varying exe results {}, constant or fault-only {}",
        r.varying,
        r.constant.len()
    );
    if !r.extended.is_empty() {
        let list: Vec<String> = r
            .extended
            .iter()
            .map(|(a, n)| format!("{a:08x} ({n})"))
            .collect();
        println!(
            "  note: f32 results wider than a float in ST0 (informational): {}",
            list.join(", ")
        );
    }
    for (a, why) in r.skipped.iter().take(10) {
        println!("  skipped {a:08x}: {why}");
    }
    for (a, n) in &r.failing_functions {
        println!("  FAIL {a:08x}: {n} vector(s)");
    }
    for (a, unit, id, ins, ex, ru, k) in r.failures.iter().take(30) {
        println!("    {k:?} {a:08x} ({unit}) {id}: in [{ins}] exe {ex} | rust {ru}");
    }
    println!("details: {}", o.out_dir.join("failures.tsv").display());
    Ok(if r.failing_functions.is_empty() {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    })
}

fn nv_call_path() -> PathBuf {
    std::env::var_os("NV_CALL")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../target/i686-pc-windows-msvc/release/nv-call.exe")
        })
}

fn cmd_x87(args: &[String]) -> Result<ExitCode, String> {
    let Some(exe) = std::env::var_os("NV_ORACLE_EXE") else {
        println!("nv-pure x87: skipped (set NV_ORACLE_EXE to an unpacked FalloutNV.exe 1.4.0.525 to run it)");
        return Ok(ExitCode::SUCCESS);
    };
    let list_path = arg(args, "--list")
        .map(PathBuf::from)
        .unwrap_or_else(default_list);
    let text =
        std::fs::read_to_string(&list_path).map_err(|e| format!("{}: {e}", list_path.display()))?;
    let o = x87check::Options {
        exe: PathBuf::from(exe),
        nv_call: nv_call_path(),
        out_dir: std::env::var_os("NV_ORACLE_OUT")
            .map(PathBuf::from)
            .unwrap_or_else(check::default_out_dir),
        count: arg(args, "--count")
            .and_then(|s| s.parse().ok())
            .unwrap_or(2000),
        list: list::from_tsv(&text)?,
    };
    std::fs::create_dir_all(&o.out_dir).map_err(|e| format!("{}: {e}", o.out_dir.display()))?;
    let r = x87check::run(&o)?;
    let mut failed = false;
    for (title, groups) in [("helpers", &r.helpers), ("functions", &r.functions)] {
        let (mut v, mut m) = (0, 0);
        for t in groups.values() {
            v += t.vectors;
            m += t.matched;
        }
        println!("{title}: {} groups, {v} vectors, {m} match", groups.len());
        for ((word, name), t) in groups {
            let mark = if t.failed + t.faults == 0 {
                "ok"
            } else {
                "FAIL"
            };
            failed |= t.failed + t.faults != 0;
            println!(
                "  {mark:4} 0x{word:04X} {name:16} {:6} vectors, {:6} match, {} differ, {} without a result, {} outside the model",
                t.vectors, t.matched, t.failed, t.faults, t.outside
            );
        }
    }
    for (g, id, ins, ex, en) in r.failures.iter().take(40) {
        println!("    {g} {id}: in [{ins}] exe {ex} | engine {en}");
    }
    println!("details: {}", x87check::files(&o.out_dir));
    Ok(if failed {
        ExitCode::from(1)
    } else {
        ExitCode::SUCCESS
    })
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let res = match args.first().map(String::as_str) {
        Some("list") => cmd_list(&args),
        Some("check") => cmd_check(&args),
        Some("x87") => cmd_x87(&args),
        _ => return usage(),
    };
    match res {
        Ok(c) => c,
        Err(e) => {
            eprintln!("nv-pure: {e}");
            ExitCode::from(1)
        }
    }
}

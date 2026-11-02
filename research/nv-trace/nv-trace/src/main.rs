//! nv-trace: compare two call traces, or print one.
//!
//! ```text
//! nv-trace diff <real.nvtrace> <port.nvtrace> [--list FILE] [--context N] [--pair REAL:PORT]...
//!               [--image LO-HI] [--out FILE]
//! nv-trace dump <file.nvtrace> [--list FILE] [--tid N] [--fn ADDR] [--limit N]
//! nv-trace stats <file.nvtrace>
//! nv-trace d3d <real.nvtrace> <port.nvtrace> [--limit N] [--out FILE]   (draw-by-draw state)
//! nv-trace d3d-draws <file.nvtrace> [<names-from.nvtrace>] [--limit N] [--out FILE]   (one run's draws and state)
//! nv-trace genlist --src <crates dir> [--max N] ADDR...   (list lines from the port's translations)
//! ```
//!
//! The list defaults to the `fn=` lines of the real trace's header.

use nv_trace_core::d3d;
use nv_trace_core::diff::{self, Normalizer, Options};
use nv_trace_core::format::{calls, Trace};
use nv_trace_core::list::{self, FnSpec};
use std::path::PathBuf;
use std::process::ExitCode;

fn usage() -> ExitCode {
    eprintln!(
        "usage:\n  nv-trace diff <real.nvtrace> <port.nvtrace> [--list FILE] [--context N] [--pair REAL:PORT]... [--image LO-HI] [--out FILE]\n  nv-trace dump <file.nvtrace> [--list FILE] [--tid N] [--fn ADDR] [--limit N]\n  nv-trace stats <file.nvtrace>\n  nv-trace d3d <real.nvtrace> <port.nvtrace> [--limit N] [--out FILE]\n  nv-trace d3d-draws <file.nvtrace> [<names-from.nvtrace>] [--limit N] [--out FILE]"
    );
    ExitCode::from(2)
}

fn hex(s: &str) -> Result<u32, String> {
    u32::from_str_radix(s.trim_start_matches("0x"), 16).map_err(|_| format!("bad hex {s:?}"))
}

fn load_list(path: Option<&PathBuf>, trace: &Trace) -> Result<Vec<FnSpec>, String> {
    match path {
        Some(p) => {
            let text = std::fs::read_to_string(p).map_err(|e| format!("{}: {e}", p.display()))?;
            list::parse(&text).map_err(|e| format!("{}: {e}", p.display()))
        }
        None => list::from_header(&trace.header),
    }
}

fn run(args: Vec<String>) -> Result<ExitCode, String> {
    let Some(cmd) = args.first() else {
        return Ok(usage());
    };
    let mut files: Vec<PathBuf> = Vec::new();
    let mut list_path = None;
    let mut out_path: Option<PathBuf> = None;
    let mut opts = Options {
        context: 12,
        ..Options::default()
    };
    let (mut tid, mut func, mut limit) = (None, None, usize::MAX);
    let (mut src, mut max): (Option<PathBuf>, Option<u64>) = (None, None);
    let mut it = args[1..].iter();
    while let Some(a) = it.next() {
        let mut val = || it.next().cloned().ok_or(format!("{a} needs a value"));
        match a.as_str() {
            "--list" => list_path = Some(PathBuf::from(val()?)),
            "--out" => out_path = Some(PathBuf::from(val()?)),
            "--context" => opts.context = val()?.parse().map_err(|_| "bad --context")?,
            "--pair" => {
                let v = val()?;
                let (x, y) = v.split_once(':').ok_or("--pair REAL:PORT")?;
                opts.pairs.push((
                    x.parse().map_err(|_| "bad --pair")?,
                    y.parse().map_err(|_| "bad --pair")?,
                ));
            }
            "--image" => {
                let v = val()?;
                let (lo, hi) = v.split_once('-').ok_or("--image LO-HI")?;
                opts.image = Some((hex(lo)?, hex(hi)?));
            }
            "--tid" => tid = Some(val()?.parse::<u32>().map_err(|_| "bad --tid")?),
            "--fn" => func = Some(hex(&val()?)?),
            "--limit" => limit = val()?.parse().map_err(|_| "bad --limit")?,
            "--src" => src = Some(PathBuf::from(val()?)),
            "--max" => max = Some(val()?.parse().map_err(|_| "bad --max")?),
            s if s.starts_with("--") => return Err(format!("unknown option {s}")),
            s => files.push(PathBuf::from(s)),
        }
    }
    match (cmd.as_str(), files.as_slice()) {
        ("diff", [a, b]) => {
            let ta = Trace::read(a)?;
            let tb = Trace::read(b)?;
            let list = load_list(list_path.as_ref(), &ta)?;
            let mut text = format!(
                "real: {} ({})\nport: {} ({})\n",
                a.display(),
                ta.header_values("source").next().unwrap_or("?"),
                b.display(),
                tb.header_values("source").next().unwrap_or("?")
            );
            for (name, t) in [("real", &ta), ("port", &tb)] {
                for (tid, n) in &t.torn {
                    text.push_str(&format!(
                        "{name}: thread {tid} ends with {n} bytes of a torn record\n"
                    ));
                }
            }
            let r = diff::diff(&ta, &tb, &list, &opts);
            text.push_str(&r.text);
            print!("{text}");
            if let Some(p) = out_path {
                std::fs::write(&p, &text).map_err(|e| format!("{}: {e}", p.display()))?;
            }
            Ok(if r.first_value.is_none() && r.first_gap.is_none() {
                ExitCode::SUCCESS
            } else {
                ExitCode::from(1)
            })
        }
        ("dump", [f]) => {
            let t = Trace::read(f)?;
            let list = load_list(list_path.as_ref(), &t)?;
            println!("{}", t.header.trim_end());
            let mut norm = Normalizer::new(diff::IMAGE);
            let mut all: Vec<_> = calls(&t).into_values().flatten().collect();
            all.sort_by_key(|c| c.seq);
            for c in all
                .iter()
                .filter(|c| tid.is_none_or(|t| c.tid == t))
                .filter(|c| func.is_none_or(|f| c.addr == f))
                .take(limit)
            {
                let spec = list.iter().find(|s| s.addr == c.addr);
                let s = diff::shape(&mut norm, spec, c);
                println!("t{:<6} {}", c.tid, diff::describe(&list, c, &s));
                // The list's dumps at entry, then at return: token, address, bytes (hex).
                for (at_return, dumps) in [(false, &c.dumps), (true, &c.rdumps)] {
                    let texts: Vec<&str> = spec
                        .map(|s| {
                            s.dumps
                                .iter()
                                .filter(|d| d.at_return == at_return)
                                .map(|d| d.text.as_str())
                                .collect()
                        })
                        .unwrap_or_default();
                    for (k, d) in dumps.iter().enumerate() {
                        let text = texts.get(k).copied().unwrap_or("?");
                        match &d.bytes {
                            Some(b) => {
                                let hex: String = b.iter().map(|x| format!("{x:02x}")).collect();
                                println!("         {text} @{:08x}: {hex}", d.addr);
                            }
                            None => println!("         {text} @{:08x}: unreadable", d.addr),
                        }
                    }
                }
            }
            Ok(ExitCode::SUCCESS)
        }
        ("d3d", [a, b]) => {
            let (ta, tb) = (Trace::read(a)?, Trace::read(b)?);
            let (ra, rb) = (d3d::replay(&ta), d3d::replay(&tb));
            let text = format!(
                "real: {}\nport: {}\n{}",
                a.display(),
                b.display(),
                d3d::diff(&ra, &rb, ("real", "port"), limit.min(8))
            );
            print!("{text}");
            if let Some(p) = out_path {
                std::fs::write(&p, &text).map_err(|e| format!("{}: {e}", p.display()))?;
            }
            Ok(ExitCode::SUCCESS)
        }
        ("d3d-draws", [f, names @ ..]) if names.len() <= 1 => {
            let t = Trace::read(f)?;
            // A second trace (the real game's) only lends its texture file names.
            let files = match names.first() {
                Some(n) => d3d::replay(&Trace::read(n)?).files,
                None => Default::default(),
            };
            let text = d3d::describe(&d3d::replay(&t), limit, &files);
            print!("{text}");
            if let Some(p) = out_path {
                std::fs::write(&p, &text).map_err(|e| format!("{}: {e}", p.display()))?;
            }
            Ok(ExitCode::SUCCESS)
        }
        ("stats", [f]) => {
            let t = Trace::read(f)?;
            let list = list::from_header(&t.header).unwrap_or_default();
            println!("{}", t.header.trim_end());
            for (tid, cs) in calls(&t) {
                let mut by: std::collections::BTreeMap<u32, usize> = Default::default();
                for c in &cs {
                    *by.entry(c.addr).or_default() += 1;
                }
                println!("thread {tid}: {} calls", cs.len());
                for (addr, n) in by {
                    let name = list
                        .iter()
                        .find(|s| s.addr == addr)
                        .map_or("", |s| s.name.as_str());
                    println!("  {addr:08x} {n:>8} {name}");
                }
            }
            Ok(ExitCode::SUCCESS)
        }
        ("genlist", addrs) if !addrs.is_empty() => {
            let src = src.ok_or("genlist needs --src <crates dir>")?;
            let mut files = Vec::new();
            rust_files(&src, &mut files);
            let texts: Vec<(PathBuf, String)> = files
                .into_iter()
                .filter_map(|p| std::fs::read_to_string(&p).ok().map(|t| (p, t)))
                .collect();
            for a in addrs {
                let addr = hex(&a.to_string_lossy())?;
                let found = texts
                    .iter()
                    .find_map(|(p, t)| nv_trace_core::genlist::line_for(t, addr).map(|l| (p, l)));
                match found {
                    Some((p, line)) => {
                        let line = match max {
                            Some(m) => insert_max(&line, m),
                            None => line,
                        };
                        println!("# from {}", p.display());
                        println!("{line}");
                    }
                    None => println!("# {addr:08x}: no translation found under {}", src.display()),
                }
            }
            Ok(ExitCode::SUCCESS)
        }
        _ => Ok(usage()),
    }
}

/// Every `.rs` file under `dir`.
fn rust_files(dir: &std::path::Path, out: &mut Vec<PathBuf>) {
    let Ok(rd) = std::fs::read_dir(dir) else {
        return;
    };
    for e in rd.flatten() {
        let p = e.path();
        if p.is_dir() {
            if p.file_name().is_some_and(|n| n != "target") {
                rust_files(&p, out);
            }
        } else if p.extension().is_some_and(|x| x == "rs") {
            out.push(p);
        }
    }
}

/// `line` with `max=N` after its return kind (the fourth field).
fn insert_max(line: &str, max: u64) -> String {
    let fields: Vec<&str> = line.split_whitespace().collect();
    if fields.len() < 4 {
        return line.to_string();
    }
    let mut out = fields[..4].join(" ");
    out.push_str(&format!(" max={max}"));
    for f in &fields[4..] {
        out.push(' ');
        out.push_str(f);
    }
    out
}

fn main() -> ExitCode {
    match run(std::env::args().skip(1).collect()) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("nv-trace: {e}");
            ExitCode::from(2)
        }
    }
}

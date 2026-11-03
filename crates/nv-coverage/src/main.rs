//! `nv-coverage <command> [--data DIR] [--out DIR] [--sweep records|full|off] [--stride N]
//! [--max-interiors N] [--max-squares N]`
//!
//! Commands: `report` (everything; writes `nv-coverage.json`, `nv-coverage.md`),
//! `functions`, `quests`, `dialogue`, `sweep`, `checklist` (each prints JSON).
//! The data folder is `--data`, else `FNV_ESM_DIR`.

use std::path::PathBuf;

use nv_coverage::json::J;
use nv_coverage::sweep::Mode;
use nv_coverage::{build, data, render, report, Options};

fn usage() -> ! {
    eprintln!(
        "usage: nv-coverage <report|functions|quests|dialogue|sweep|checklist> [--data DIR] [--out DIR]\n\
         \x20      [--sweep records|full|off] [--stride N] [--max-interiors N] [--max-squares N]"
    );
    std::process::exit(2);
}

fn main() {
    let mut args = std::env::args().skip(1);
    let Some(cmd) = args.next() else { usage() };
    if cmd == "object-worker" {
        // Internal: `object-worker --data DIR --from N`, see `objects::worker`.
        let rest: Vec<String> = args.collect();
        let val = |k: &str| {
            rest.iter()
                .position(|a| a == k)
                .and_then(|i| rest.get(i + 1))
                .cloned()
        };
        let dir = data::locate(val("--data").as_deref()).unwrap_or_else(|e| {
            eprintln!("{e}");
            std::process::exit(1)
        });
        let d = data::open(&dir).unwrap_or_else(|e| {
            eprintln!("{e}");
            std::process::exit(1)
        });
        let from = val("--from").and_then(|v| v.parse().ok()).unwrap_or(0);
        let stdout = std::io::stdout();
        nv_coverage::objects::worker(
            &d.order,
            &nv_coverage::funcs::handled_set(),
            from,
            &mut stdout.lock(),
        );
        return;
    }
    if cmd == "gamefacts" {
        // `nv-coverage gamefacts [--data DIR] [--out FILE]`: the measurements the GAME rows are made from.
        let rest: Vec<String> = args.collect();
        let val = |k: &str| {
            rest.iter()
                .position(|a| a == k)
                .and_then(|i| rest.get(i + 1))
                .cloned()
        };
        let dir = data::locate(val("--data").as_deref()).unwrap_or_else(|e| {
            eprintln!("{e}");
            std::process::exit(1)
        });
        let j = nv_coverage::gamerows::facts(&dir).unwrap_or_else(|e| {
            eprintln!("{e}");
            std::process::exit(1)
        });
        match val("--out") {
            Some(p) => std::fs::write(&p, j.pretty()).expect("write the facts"),
            None => print!("{}", j.pretty()),
        }
        return;
    }
    if cmd == "quest-list" {
        // `nv-coverage quest-list [--data DIR]`: one tab separated line per quest.
        let dir = args.skip_while(|a| a != "--data").nth(1);
        let dir = data::locate(dir.as_deref()).unwrap_or_else(|e| {
            eprintln!("{e}");
            std::process::exit(1)
        });
        let d = data::open(&dir).unwrap_or_else(|e| {
            eprintln!("{e}");
            std::process::exit(1)
        });
        print!("{}", nv_coverage::gamerows::quest_tsv(&d.order));
        return;
    }
    if cmd == "script" {
        // `nv-coverage script EDITOR_ID [--data DIR]`: prints a script's source.
        let name = args.next().unwrap_or_else(|| usage());
        let dir = args.skip_while(|a| a != "--data").nth(1);
        let dir = data::locate(dir.as_deref()).unwrap_or_else(|e| {
            eprintln!("{e}");
            std::process::exit(1)
        });
        let d = data::open(&dir).unwrap_or_else(|e| {
            eprintln!("{e}");
            std::process::exit(1)
        });
        for s in nv_coverage::scan::sources(&d.order)
            .iter()
            .filter(|s| s.editor.eq_ignore_ascii_case(&name))
        {
            println!("; {} {} ({})\n{}", s.record, s.form, s.plugin, s.text);
        }
        return;
    }
    let (mut dir, mut out) = (None, None);
    let mut sweep: Option<Option<Mode>> = None;
    let mut sweep_given = false;
    let mut opts = Options::everything();
    while let Some(a) = args.next() {
        let mut val = |name: &str| {
            args.next().unwrap_or_else(|| {
                eprintln!("{name} needs a value");
                usage()
            })
        };
        match a.as_str() {
            "--data" => dir = Some(val("--data")),
            "--out" => out = Some(PathBuf::from(val("--out"))),
            "--sweep" => {
                sweep_given = true;
                sweep = match val("--sweep").as_str() {
                    "records" => Some(Some(Mode::Records)),
                    "full" => Some(Some(Mode::Full)),
                    "off" => None,
                    "auto" => Some(None),
                    _ => usage(),
                };
            }
            "--stride" => opts.limits.stride = val("--stride").parse().unwrap_or_else(|_| usage()),
            "--max-interiors" => {
                opts.limits.max_interiors =
                    Some(val("--max-interiors").parse().unwrap_or_else(|_| usage()))
            }
            "--max-squares" => {
                opts.limits.max_squares_per_world =
                    Some(val("--max-squares").parse().unwrap_or_else(|_| usage()))
            }
            _ => usage(),
        }
    }
    let dir = match data::locate(dir.as_deref()) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    };
    if sweep_given {
        opts.sweep = sweep;
    }
    let key = match cmd.as_str() {
        "report" => None,
        "functions" => {
            (opts.quests, opts.dialogue, opts.objects, opts.sweep) = (false, false, false, None);
            Some("script_functions")
        }
        "quests" => {
            (opts.functions, opts.dialogue, opts.objects, opts.sweep) = (false, false, false, None);
            Some("quest_walk")
        }
        "dialogue" => {
            (opts.functions, opts.quests, opts.sweep) = (false, false, None);
            Some("dialogue")
        }
        "sweep" => {
            (opts.functions, opts.quests, opts.dialogue, opts.objects) =
                (false, false, false, false);
            if !sweep_given {
                opts.sweep = Some(None);
            }
            Some("load_sweep")
        }
        "checklist" => {
            (
                opts.functions,
                opts.quests,
                opts.dialogue,
                opts.objects,
                opts.sweep,
            ) = (false, false, false, false, None);
            Some("system_checklist")
        }
        _ => usage(),
    };
    let full = match build(&dir, &opts) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    };
    let json = report::to_json(&full);
    match key {
        Some(k) => {
            let J::Obj(fields) = json else { unreachable!() };
            let part = fields
                .into_iter()
                .find(|(n, _)| n == k)
                .map_or(J::Null, |(_, v)| v);
            print!("{}", part.pretty());
        }
        None => {
            let out = out.unwrap_or_else(|| PathBuf::from("nv-coverage-out"));
            std::fs::create_dir_all(&out).expect("create the output folder");
            let (jp, mp) = (out.join("nv-coverage.json"), out.join("nv-coverage.md"));
            std::fs::write(&jp, json.pretty()).expect("write the JSON");
            std::fs::write(&mp, render::markdown(&full)).expect("write the Markdown");
            println!("wrote {} and {}", jp.display(), mp.display());
        }
    }
}

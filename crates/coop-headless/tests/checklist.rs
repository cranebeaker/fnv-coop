//! The master checklist: manifest sanity, builtin checks and their twins,
//! the policy probe over every row, the quest walk on a few quests (when the
//! game data is on this machine) and the engine backend stub.

use std::path::PathBuf;
use std::sync::Arc;

use coop_headless::backend::{self, Verdict};
use coop_headless::checklist::builtin::{registry, Env};
use coop_headless::checklist::report::{all_green, verdict_line, Counts};
use coop_headless::checklist::run::{read_real, run, Opts};
use coop_headless::checklist::{default_manifest_dir, load};
use coop_headless::data::Data;
use coop_headless::questwalk::{policy_probe, walk_all, WalkOpts};

static LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
static DATA: std::sync::OnceLock<Option<Arc<Data>>> = std::sync::OnceLock::new();

/// The game data, opened once; tests that use it run one at a time (the plugins are large).
fn data() -> Option<Arc<Data>> {
    DATA.get_or_init(|| Data::open(&Data::locate(None).ok()?).ok().map(Arc::new))
        .clone()
}

#[test]
fn manifest_loads_and_every_test_reference_is_known() {
    let rows = load(&default_manifest_dir()).expect("manifest");
    assert!(rows.len() > 150, "{} rows", rows.len());
    let reg: Vec<&str> = registry().iter().map(|(n, _)| *n).collect();
    for r in &rows {
        assert!(
            !r.title.is_empty() && !r.area.is_empty() && r.id.contains('.'),
            "{}",
            r.id
        );
        assert!(
            ["headless-real-logic", "modelled"].contains(&r.fidelity.as_str()),
            "{}",
            r.id
        );
        assert!(
            r.fidelity != "modelled" || !r.stand_in.is_empty(),
            "{} is modelled without a named stand-in",
            r.id
        );
        assert!(
            ["agnostic", "world"].contains(&r.backend.as_str()),
            "{}",
            r.id
        );
        assert!(
            r.needs == "base" || r.needs == "full_assets" || r.needs.starts_with("dlc:"),
            "{}",
            r.id
        );
        for t in &r.tests {
            let (kind, name) = t.split_once(':').unwrap_or(("", t));
            match kind {
                "builtin" => assert!(reg.contains(&name), "{}: no builtin {name}", r.id),
                "conformance" => assert!(
                    backend::TESTS.iter().any(|(n, _)| *n == name),
                    "{}: no conformance test {name}",
                    r.id
                ),
                "scenario" | "questwalk" | "cargo" => {}
                _ => panic!("{}: unknown test kind in {t}", r.id),
            }
        }
        if r.twin {
            assert!(
                r.tests.iter().any(|t| t.starts_with("scenario:")
                    || t.starts_with("builtin:")
                    || t.starts_with("conformance:")),
                "{} asks for a twin but has no test one can attach to",
                r.id
            );
        }
    }
}

#[test]
fn manifest_covers_every_area_the_owner_listed() {
    let rows = load(&default_manifest_dir()).expect("manifest");
    for a in [
        "CORE", "STORY", "QUESTS", "SYSTEMS", "WORLD", "SCALE", "BACKEND",
    ] {
        assert!(rows.iter().any(|r| r.area == a), "no rows in {a}");
    }
    for dlc in [
        "dead_money",
        "honest_hearts",
        "old_world_blues",
        "lonesome_road",
    ] {
        assert!(
            rows.iter().any(|r| r.id == format!("quests.rollup.{dlc}")),
            "no rollup for {dlc}"
        );
    }
}

#[test]
fn builtin_checks_pass_and_their_twins_fail() {
    // checks that read the game data run only when it is on this machine
    const NEEDS_DATA: &[&str] = &[
        "harness_verdicts",
        "harness_sim_deterministic",
        "backend_engine_stub",
        "story_effects_covered",
        "harness_recorder",
        "harness_diverge",
        "harness_shrink",
        "harness_gentest",
    ];
    let _g = LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let d = data();
    for (name, f) in registry() {
        if [
            "policy_ids_resolve",
            "core_chaos_all",
            "scale_four_peer_budget",
        ]
        .contains(&name)
        {
            continue;
        }
        let needs = NEEDS_DATA.contains(&name);
        if needs && d.is_none() {
            continue;
        }
        let env = Env {
            data: if needs { d.as_deref() } else { None },
            peers: 3,
            net: "bad_wifi",
            seed: 7,
        };
        if let Err(e) = f(&env, false) {
            panic!("{name} failed: {e}");
        }
        assert!(
            f(&env, true).is_err(),
            "{name}: the twin passed, so the check cannot fail"
        );
    }
}

#[test]
fn policy_probe_agrees_with_every_row_at_every_party_size() {
    let t = coop_policy::PolicyTable::bundled().expect("table");
    for n in [2u8, 3, 4] {
        for row in t.rows() {
            let f = policy_probe(&t, row, n);
            assert!(f.is_empty(), "{} with {n} peers: {f:?}", row.edid);
        }
    }
}

#[test]
fn verdict_line_and_exit_rule() {
    let c = Counts {
        rows: 10,
        pass: 6,
        modelled: 2,
        fail: 1,
        pending: 1,
        skip: 0,
    };
    assert_eq!(
        verdict_line(&c),
        "CO-OP CHECKLIST: 8/10 items green, 1 pending, 1 failing"
    );
    assert!(!all_green(&c));
    assert!(all_green(&Counts {
        rows: 3,
        pass: 2,
        modelled: 1,
        ..Counts::default()
    }));
    assert!(!all_green(&Counts::default()));
}

#[test]
fn real_game_verdicts_are_read_from_runner_folders() {
    let dir: PathBuf = std::env::temp_dir().join(format!("ck-real-{}", std::process::id()));
    let a = dir.join("doc_stage_sync").join("20261102-101500-1");
    std::fs::create_dir_all(&a).unwrap();
    std::fs::write(
        a.join("verdict.json"),
        r#"{"scenario":"doc_stage_sync","result":"pass"}"#,
    )
    .unwrap();
    let b = dir.join("doc_stage_sync").join("20261103-090000-1");
    std::fs::create_dir_all(&b).unwrap();
    std::fs::write(
        b.join("verdict.json"),
        r#"{"scenario":"doc_stage_sync","result":"fail"}"#,
    )
    .unwrap();
    let m = read_real(&dir);
    assert_eq!(m["doc_stage_sync"], ("2026-11-03".to_string(), false));
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn engine_backend_reports_pending() {
    let _g = LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let Some(d) = data() else { return };
    let e = backend::by_name("engine").unwrap();
    for (id, _) in backend::TESTS {
        assert!(
            matches!(backend::run_test(&*e, &d, id, false), Verdict::Pending(m) if m.contains("engine backend")),
            "{id}"
        );
    }
}

#[test]
fn engine_checklist_marks_world_rows_pending() {
    let _g = LOCK.lock().unwrap_or_else(|e| e.into_inner());
    if data().is_none() {
        return;
    }
    let o = Opts {
        backend: "engine".into(),
        area: Some("BACKEND".into()),
        jobs: 2,
        ..Opts::default()
    };
    let r = run(&o).expect("run");
    assert!(r.outcomes.iter().filter(|x| x.row.id.starts_with("backend.conformance.")).all(|x| matches!(&x.status, coop_headless::checklist::Status::Pending(m) if m.contains("engine backend"))));
}

#[test]
fn world_backend_conformance_holds_and_its_twins_fail() {
    let _g = LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let Some(d) = data() else { return };
    let b = backend::by_name("world").unwrap();
    for (id, _) in backend::TESTS {
        assert_eq!(backend::run_test(&*b, &d, id, false), Verdict::Pass, "{id}");
        assert!(
            matches!(backend::run_test(&*b, &d, id, true), Verdict::Fail(_)),
            "{id}: the twin must fail"
        );
    }
}

#[test]
fn questwalk_walks_the_prologue_and_a_dlc_quest() {
    let _g = LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let Some(d) = data() else { return };
    for only in ["VCG0", "NVDLC02DialogueJoshua"] {
        let o = WalkOpts {
            filter: Some(only.into()),
            jobs: 2,
            ..WalkOpts::default()
        };
        let (res, _) = walk_all(&d, &o).expect("walk");
        assert!(!res.is_empty(), "{only}");
        for r in &res {
            assert!(r.fails.is_empty(), "{}: {:?}", r.edid, r.fails);
        }
    }
}

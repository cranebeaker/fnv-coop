//! The multi-cell scenarios (peers in different cells) on the generated test
//! data, each under several chaos profiles, with their mutation twins.

use std::path::PathBuf;
use std::sync::Arc;

use coop_headless::data::Data;
use coop_headless::scenario::Scenario;
use coop_headless::sim::{run, RunOpts};

const NAMES: [&str; 5] = [
    "live_two_cells_actor_owner",
    "live_cell_enter_adopts_state",
    "live_deferred_script_unloaded_ref",
    "live_tether_across_cells",
    "live_late_join_other_cell",
];
const PROFILES: [&str; 4] = ["perfect", "good_wifi", "bad_wifi", "mobile"];

fn data(tag: &str) -> (testdata::TempData, Arc<Data>) {
    let t = testdata::fighting::fighting(tag);
    let d = Arc::new(Data::open(t.path()).expect("generated data opens"));
    (t, d)
}

fn load(name: &str) -> Scenario {
    let p = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("scenarios")
        .join(format!("{name}.json"));
    Scenario::load(&p).unwrap_or_else(|e| panic!("{name}: {e}"))
}

fn go(s: &Scenario, d: &Arc<Data>, net: &str) -> coop_headless::sim::RunResult {
    run(
        s,
        d,
        &RunOpts {
            net: Some(net.into()),
            ..RunOpts::default()
        },
    )
}

#[test]
fn every_scene_scenario_passes_under_every_profile() {
    let (_t, d) = data("scene-pass");
    for name in NAMES {
        let s = load(name);
        for net in PROFILES {
            let r = go(&s, &d, net);
            assert_eq!(
                r.verdict.result, "pass",
                "{name} on {net}: {}",
                r.verdict.reason
            );
        }
    }
}

#[test]
fn every_mutation_twin_fails_as_expected() {
    let (_t, d) = data("scene-twin");
    for name in NAMES {
        let s = load(&format!("twin_{name}"));
        assert!(s.expect_fail, "{name}");
        let r = go(&s, &d, "good_wifi");
        assert_eq!(
            (r.verdict.result.as_str(), r.verdict.class.as_str()),
            ("pass", "twin"),
            "twin_{name}: {}",
            r.verdict.reason
        );
    }
}

#[test]
fn a_scene_run_is_deterministic() {
    let (_t, d) = data("scene-det");
    let s = load("live_two_cells_actor_owner");
    let (a, b) = (go(&s, &d, "bad_wifi"), go(&s, &d, "bad_wifi"));
    assert_eq!(a.hash, b.hash);
    assert!(
        a.dumps[0].contains("\"roam\":{\"cells\":"),
        "{}",
        a.dumps[0]
    );
}

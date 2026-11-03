//! Scenes, movies, control locks, endings and time through the simulator
//! (the scenario files of `tools/coop-run/scenarios`, on a generated world
//! that has the game clock's globals).

use std::path::PathBuf;
use std::sync::Arc;

use coop_headless::data::Data;
use coop_headless::scenario::{Scenario, PROFILES};
use coop_headless::sim::{run, RunOpts};
use coop_headless::suite::{plan, run_jobs, summary};

const REAL_CELL: &str = "GSDocMitchellHouse";
const TEST_CELL: &str = "TestAiCell";

fn data(tag: &str) -> (testdata::TempData, Arc<Data>) {
    let t = testdata::ai::world(tag);
    let d = Arc::new(Data::open(t.path()).expect("generated data opens"));
    (t, d)
}

fn scenario(name: &str) -> Scenario {
    let p: PathBuf = [
        env!("CARGO_MANIFEST_DIR"),
        "..",
        "..",
        "tools",
        "coop-run",
        "scenarios",
        &format!("{name}.json"),
    ]
    .iter()
    .collect();
    let text = std::fs::read_to_string(&p)
        .unwrap_or_else(|e| panic!("{}: {e}", p.display()))
        .replace(REAL_CELL, TEST_CELL);
    Scenario::parse(&text, name).unwrap_or_else(|e| panic!("{name}: {e}"))
}

fn passes(name: &str) {
    let (_t, d) = data(&format!("scene-{name}"));
    let r = run(&scenario(name), &d, &RunOpts::default());
    assert_eq!(
        r.verdict.result, "pass",
        "{name}: {}\n{}\n{}",
        r.verdict.reason, r.dumps[0], r.dumps[1]
    );
}

fn twin_fails(name: &str) {
    let (_t, d) = data(&format!("scene-twin-{name}"));
    let r = run(&scenario(&format!("twin_{name}")), &d, &RunOpts::default());
    assert_eq!(
        (r.verdict.result.as_str(), r.verdict.class.as_str()),
        ("pass", "twin"),
        "{name}: {}",
        r.verdict.reason
    );
}

#[test]
fn live_scene_barrier() {
    passes("live_scene_barrier");
}

#[test]
fn live_scene_barrier_twin() {
    twin_fails("live_scene_barrier");
}

#[test]
fn live_scene_late_join() {
    passes("live_scene_late_join");
}

#[test]
fn live_scene_late_join_twin() {
    twin_fails("live_scene_late_join");
}

#[test]
fn live_sleep_vote() {
    passes("live_sleep_vote");
}

#[test]
fn live_sleep_vote_twin() {
    twin_fails("live_sleep_vote");
}

#[test]
fn live_sleep_refused() {
    passes("live_sleep_refused");
}

#[test]
fn live_movie_sync() {
    passes("live_movie_sync");
}

#[test]
fn live_movie_sync_twin() {
    twin_fails("live_movie_sync");
}

#[test]
fn live_control_lock_per_player() {
    passes("live_control_lock_per_player");
}

#[test]
fn live_control_lock_per_player_twin() {
    twin_fails("live_control_lock_per_player");
}

#[test]
fn a_movie_is_declared_as_modelled() {
    let (_t, d) = data("scene-modelled");
    let r = run(&scenario("live_movie_sync"), &d, &RunOpts::default());
    let m = r
        .verdict
        .json
        .get("modelled")
        .and_then(|m| m.as_arr())
        .expect("modelled list");
    assert!(
        m.iter()
            .any(|x| x.as_str().is_some_and(|s| s.starts_with("movie playback:"))),
        "the Bink player stand-in is declared"
    );
}

#[test]
fn the_same_seed_twice_gives_identical_dumps() {
    let (_t, d) = data("scene-det");
    let s = scenario("live_sleep_vote");
    let a = run(&s, &d, &RunOpts::default());
    let b = run(&s, &d, &RunOpts::default());
    assert_eq!(a.hash, b.hash);
    assert_eq!(a.dumps, b.dumps);
}

#[test]
fn a_guests_own_request_is_a_vote_and_the_host_runs_the_hours() {
    passes("live_sleep_guest_asks");
}

#[test]
fn soak_across_every_chaos_profile() {
    let t = testdata::ai::world("scene-soak");
    let d = Arc::new(Data::open(t.path()).unwrap());
    let names = [
        "live_scene_barrier",
        "live_scene_late_join",
        "live_sleep_vote",
        "live_sleep_guest_asks",
        "live_sleep_refused",
        "live_movie_sync",
        "live_control_lock_per_player",
    ];
    let scns: Vec<(String, Scenario)> = names
        .iter()
        .map(|n| (format!("{n}.json"), scenario(n)))
        .collect();
    let jobs = plan(&scns, Some(&[1, 2, 3]), true, None);
    assert_eq!(jobs.len(), names.len() * PROFILES.len() * 3);
    let done = run_jobs(&d, jobs, 2, None, false, None, false, false);
    let bad: Vec<_> = done
        .iter()
        .filter(|x| x.result != "pass")
        .map(|x| {
            format!(
                "{} [{}] seed {}: {} ({})",
                x.scenario, x.net, x.seed, x.class, x.line
            )
        })
        .collect();
    assert!(
        bad.is_empty(),
        "{}\n{}",
        bad.join("\n"),
        summary(&done, true).join("\n")
    );
}

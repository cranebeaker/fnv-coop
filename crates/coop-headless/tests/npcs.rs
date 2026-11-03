//! People and creatures on both peers, on a generated world (no game data).

use std::sync::Arc;

use coop_headless::data::Data;
use coop_headless::scenario::Scenario;
use coop_headless::sim::{run, RunOpts};

fn data(tag: &str) -> (testdata::TempData, Arc<Data>) {
    let t = testdata::fighting::fighting(tag);
    let d = Arc::new(Data::open(t.path()).expect("generated data opens"));
    (t, d)
}

fn scn(body: &str) -> Scenario {
    Scenario::parse(
        &format!(r#"{{"cell":"TestFightCell","duration_s":12,"settle_s":4,{body}}}"#),
        "t",
    )
    .unwrap()
}

/// Everyone placed in the cell is known to both peers, alive, owned by
/// somebody (no frozen dormant copies) and not dead in the world.
const NPCS: &str = r#""name":"live_npcs_present","network":"good_wifi","seed":3,
  "host":{"script":[]},"guest":{"script":[]},
  "expect":[
    {"id":"guard-host","path":"actors.TestGuardRef.alive","op":"eq","value":true,"on":"host"},
    {"id":"guard-guest","path":"actors.TestGuardRef.alive","op":"eq","value":true,"on":"guest"},
    {"id":"town-host","path":"actors.TestTownRef.alive","op":"eq","value":true,"on":"host"},
    {"id":"town-guest","path":"actors.TestTownRef.alive","op":"eq","value":true,"on":"guest"},
    {"id":"gecko-guest","path":"actors.TestGeckoRef.alive","op":"eq","value":true,"on":"guest"},
    {"id":"known-guest","path":"live.actors_known","op":"ge","value":5,"on":"guest"},
    {"id":"nobody-dormant-host","path":"live.actors_dormant","op":"eq","value":0,"on":"host"},
    {"id":"nobody-dormant-guest","path":"live.actors_dormant","op":"eq","value":0,"on":"guest"},
    {"id":"guest-reported","path":"live.actors_tracked","op":"ge","value":5,"on":"guest"},
    {"id":"people-in-scene-host","path":"live.npc_entities_visible","op":"ge","value":2,"on":"host"},
    {"id":"people-in-scene-guest","path":"live.npc_entities_visible","op":"ge","value":2,"on":"guest"},
    {"id":"creatures-in-scene","path":"live.creature_entities_visible","op":"ge","value":1,"on":"both"},
    {"id":"none-dead","path":"dead.TestGuardRef","op":"absent","on":"both"}]"#;

/// The host makes a creature once; the guest sees it under the same id; one
/// hit from the guest kills it on both, once.
const SPAWN: &str = r#""name":"live_creature_spawn_shared","network":"good_wifi","seed":11,
  "host":{"script":[{"at":2,"run":"TestGuardRef.PlaceAtMe TestFightGecko 1"}]},
  "guest":{"script":[{"at":7,"run":"coop hit FF000001 100000"}]},
  "expect":[
    {"id":"spawned-host","path":"live.spawned","op":"eq","value":1,"on":"host"},
    {"id":"spawned-guest","path":"live.spawned","op":"eq","value":1,"on":"guest"},
    {"id":"known-guest","path":"actors.FF000001.creature","op":"eq","value":true,"on":"guest"},
    {"id":"one-more-creature-host","path":"live.creature_entities","op":"ge","value":2,"on":"host"},
    {"id":"one-more-creature-guest","path":"live.creature_entities","op":"ge","value":2,"on":"guest"},
    {"id":"dead-host","path":"dead.FF000001","op":"eq","value":true,"on":"host"},
    {"id":"dead-guest","path":"dead.FF000001","op":"eq","value":true,"on":"guest"},
    {"id":"alive-flag-guest","path":"actors.FF000001.alive","op":"eq","value":false,"on":"guest"},
    {"id":"h","path":"hash.shared","on":"equal"}]"#;

#[test]
fn placed_people_are_known_alive_and_owned_on_both_peers() {
    let (_t, d) = data("npcs-alive");
    let r = run(&scn(NPCS), &d, &RunOpts::default());
    assert_eq!(r.verdict.result, "pass", "{}", r.verdict.reason);
}

#[test]
fn a_host_made_creature_appears_once_on_the_guest_and_dies_once_on_both() {
    let (_t, d) = data("npcs-spawn");
    let r = run(&scn(SPAWN), &d, &RunOpts::default());
    assert_eq!(r.verdict.result, "pass", "{}", r.verdict.reason);
}

#[test]
fn the_twin_without_the_spawn_fails_as_expected() {
    let (_t, d) = data("npcs-twin");
    let twin = SPAWN.replace(
        r#""script":[{"at":2,"run":"TestGuardRef.PlaceAtMe TestFightGecko 1"}]"#,
        r#""script":[]"#,
    );
    let r = run(
        &scn(&format!(r#"{twin},"expect_fail":true"#)),
        &d,
        &RunOpts::default(),
    );
    assert_eq!(
        (r.verdict.result.as_str(), r.verdict.class.as_str()),
        ("pass", "twin"),
        "{}",
        r.verdict.reason
    );
}

#[test]
fn people_and_a_spawn_hold_on_every_chaos_profile() {
    use coop_headless::suite::{plan, run_jobs, summary};
    let (_t, d) = data("npcs-soak");
    let scns = vec![
        ("live_npcs_present.json".to_string(), scn(NPCS)),
        ("live_creature_spawn_shared.json".to_string(), scn(SPAWN)),
    ];
    let jobs = plan(&scns, Some(&[1, 2, 3]), true, None);
    assert_eq!(jobs.len(), 2 * 7 * 3);
    let done = run_jobs(&d, jobs, 3, None, true, None, false, false);
    let bad: Vec<String> = done
        .iter()
        .filter(|x| x.result != "pass")
        .map(|x| format!("{} {} seed {} {}", x.scenario, x.net, x.seed, x.result))
        .collect();
    assert!(
        bad.is_empty(),
        "failing: {bad:?}
{}",
        summary(&done, true).join(
            "
"
        )
    );
    assert!(
        done.iter().all(|x| x.hash2 == Some(x.hash)),
        "deterministic"
    );
}

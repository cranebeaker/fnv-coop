//! The simulator end to end on a generated world (no game data needed).

use std::sync::Arc;

use coop_headless::data::Data;
use coop_headless::scenario::Scenario;
use coop_headless::sim::{run, RunOpts};
use coop_headless::suite::{plan, run_jobs, summary};

fn data(tag: &str) -> (testdata::TempData, Arc<Data>) {
    let t = testdata::fighting::fighting(tag);
    let d = Arc::new(Data::open(t.path()).expect("generated data opens"));
    (t, d)
}

fn scn(body: &str) -> Scenario {
    Scenario::parse(
        &format!(r#"{{"cell":"TestFightCell","duration_s":8,"settle_s":3,{body}}}"#),
        "t",
    )
    .unwrap()
}

const MOVE: &str = r#""name":"move","network":"good_wifi","seed":5,
  "host":{"script":[{"at":1,"run":"coop teleport 100 200 0"},{"at":3,"key":"w:1"}],"extra_args":["--walk"]},
  "guest":{"script":[{"at":1,"run":"coop teleport 900 400 0"}]},
  "expect":[
    {"id":"guest-seen","path":"positions.guest","op":"near","value":[900,400],"tol":30,"dims":2,"on":"host"},
    {"id":"host-walked","path":"positions.host","op":"dist_from","from":[100,200],"cmp":"ge","value":250,"on":"guest"},
    {"id":"eq","path":"positions.host","on":"equal","tol":40,"dims":2},
    {"id":"avatars","path":"live.avatars","op":"ge","value":1,"on":"both"},
    {"id":"h","path":"hash.shared","on":"equal"}]"#;

const KILL: &str = r#""name":"kill","network":"bad_wifi","seed":9,"outages":[{"at":2,"seconds":2}],
  "guest":{"script":[{"at":3,"run":"coop hit TestGeckoRef 100000"},{"at":5,"run":"coop hit TestGeckoRef 100000"}]},
  "expect":[
    {"id":"dead-host","path":"dead.TestGeckoRef","op":"eq","value":true,"on":"host"},
    {"id":"dead-guest","path":"dead.TestGeckoRef","op":"eq","value":true,"on":"guest"},
    {"id":"hits","path":"live.hits_sent","op":"ge","value":1,"on":"guest"},
    {"id":"h","path":"hash.shared","on":"equal"}]"#;

#[test]
fn teleport_and_walking_reach_the_other_peer() {
    let (_t, d) = data("sim-move");
    let r = run(&scn(MOVE), &d, &RunOpts::default());
    assert_eq!(r.verdict.result, "pass", "{}", r.verdict.reason);
    let m = r
        .verdict
        .json
        .get("modelled")
        .and_then(|m| m.as_arr())
        .unwrap();
    assert!(
        m.iter()
            .any(|x| x.as_str().is_some_and(|s| s.starts_with("movement:"))),
        "movement is declared as modelled"
    );
}

#[test]
fn a_guest_hit_kills_through_a_bad_link_and_an_outage() {
    let (_t, d) = data("sim-kill");
    let r = run(&scn(KILL), &d, &RunOpts::default());
    assert_eq!(r.verdict.result, "pass", "{}", r.verdict.reason);
    assert!(
        r.dumps[0].contains("\"dead\":{\"TestGeckoRef\":true}"),
        "{}",
        r.dumps[0]
    );
}

#[test]
fn the_twin_without_the_hits_fails_as_expected() {
    let (_t, d) = data("sim-twin");
    let twin = KILL.replace(r#""script":[{"at":3,"run":"coop hit TestGeckoRef 100000"},{"at":5,"run":"coop hit TestGeckoRef 100000"}]"#, r#""script":[]"#);
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
    // and the same assertions without the twin flag are a plain assert failure
    let r = run(&scn(&twin), &d, &RunOpts::default());
    assert_eq!(
        (r.verdict.result.as_str(), r.verdict.class.as_str()),
        ("fail", "assert")
    );
    let packet = r
        .verdict
        .json
        .get("packet")
        .expect("a failure packet")
        .dump();
    assert!(packet.len() < 3000, "{}", packet.len());
    assert!(r
        .verdict
        .json
        .get("rerun")
        .and_then(|v| v.as_str())
        .unwrap()
        .contains("--seed 9"));
}

#[test]
fn the_same_seed_twice_gives_identical_dumps_and_another_seed_still_passes() {
    let (_t, d) = data("sim-det");
    let s = scn(KILL);
    let a = run(&s, &d, &RunOpts::default());
    let b = run(&s, &d, &RunOpts::default());
    assert_eq!(a.hash, b.hash);
    assert_eq!(a.dumps, b.dumps);
    let c = run(
        &s,
        &d,
        &RunOpts {
            seed: Some(77),
            ..RunOpts::default()
        },
    );
    assert_eq!(c.verdict.result, "pass", "{}", c.verdict.reason);
}

#[test]
fn no_seed_leaves_a_guest_without_the_hosts_actors() {
    // regression: live messages that beat the join snapshot used to be dropped, so the
    // guest never learned the actors, and a kill the host resolved never reached it
    // (about 4 seeds in 10 on bad_wifi); the hash does not cover `dead`, so only this check saw it
    let (_t, d) = data("sim-seeds");
    let s = Scenario::parse(
        &format!(r#"{{"cell":"TestFightCell","duration_s":8,"settle_s":6,{KILL}}}"#),
        "t",
    )
    .unwrap();
    let fails: Vec<u64> = (1..=30)
        .filter(|seed| {
            run(
                &s,
                &d,
                &RunOpts {
                    seed: Some(*seed),
                    ..RunOpts::default()
                },
            )
            .verdict
            .result
                != "pass"
        })
        .collect();
    assert!(fails.is_empty(), "failing seeds {fails:?}");
}

#[test]
fn a_cell_this_data_lacks_is_a_needs_assets_skip_not_a_guess() {
    let (_t, d) = data("sim-assets");
    let s = Scenario::parse(r#"{"name":"far","cell":"NoSuchCell","expect":[]}"#, "t").unwrap();
    let r = run(&s, &d, &RunOpts::default());
    assert_eq!(
        (r.verdict.result.as_str(), r.verdict.class.as_str()),
        ("skip", "needs_assets")
    );
    assert!(
        r.verdict.reason.contains("NoSuchCell"),
        "{}",
        r.verdict.reason
    );
    let ng = Scenario::parse(r#"{"name":"ng","new_game":true}"#, "t").unwrap();
    assert_eq!(
        run(&ng, &d, &RunOpts::default()).verdict.class,
        "needs_assets"
    );
    let unknown = scn(r#""name":"u","host":{"extra_args":["--gizmo"]}"#);
    assert_eq!(
        run(&unknown, &d, &RunOpts::default()).verdict.class,
        "needs_assets"
    );
    let missing_ref = scn(r#""name":"m","host":{"script":[{"at":1,"run":"NoSuchRef.Enable"}]}"#);
    assert_eq!(
        run(&missing_ref, &d, &RunOpts::default()).verdict.class,
        "needs_assets"
    );
}

#[test]
fn the_pool_runs_scenarios_across_seeds_and_profiles_and_summarises() {
    let (_t, d) = data("sim-pool");
    let scns = vec![
        ("kill.json".to_string(), scn(KILL)),
        ("move.json".to_string(), scn(MOVE)),
    ];
    let jobs = plan(&scns, Some(&[1, 2]), true, None);
    assert_eq!(jobs.len(), 2 * 7 * 2);
    let done = run_jobs(&d, jobs, 3, None, true, None, false, false);
    assert_eq!(done.len(), 28);
    assert!(
        done.iter().all(|x| x.hash2 == Some(x.hash)),
        "every run is deterministic"
    );
    let lines = summary(&done, true);
    assert_eq!(lines.len(), 14, "one line per scenario and profile");
    assert!(lines.iter().any(|l| l.starts_with("SUMMARY: kill [flaky]")));
    // a failing cell names its first failing seed and the replay command
    for l in lines.iter().filter(|l| l.contains("first failing")) {
        assert!(
            l.contains("replay: coop-sim run kill.json")
                || l.contains("replay: coop-sim run move.json"),
            "{l}"
        );
    }
}

const ATTACK: &str = r#""name":"live_attack_visible","network":"good_wifi","seed":61,
  "host":{"script":[{"at":1,"run":"coop attack swing 32"},{"at":2,"run":"coop attack shoot 40"},{"at":3,"run":"coop attack swing 32"}]},
  "guest":{"script":[{"at":1,"run":"coop attack swing 32"},{"at":2,"run":"coop attack reload 90"},{"at":3,"run":"coop attack shoot 40"},{"at":4,"run":"coop attack swing 32"}]},
  "expect":[
    {"id":"guest-saw","path":"live.puppet_actions","op":"ge","value":3,"on":"guest"},
    {"id":"host-saw","path":"live.puppet_actions","op":"ge","value":4,"on":"host"},
    {"id":"h","path":"hash.shared","on":"equal"}]"#;

#[test]
fn live_attack_visible_the_other_peer_receives_every_attack_event() {
    let (_t, d) = data("sim-attack");
    let r = run(&scn(ATTACK), &d, &RunOpts::default());
    assert_eq!(r.verdict.result, "pass", "{}", r.verdict.reason);
    assert!(r.dumps[0].contains("\"puppet_actions\":"), "{}", r.dumps[0]);
}

#[test]
fn the_attack_twin_without_attacks_fails_as_expected() {
    let (_t, d) = data("sim-attack-twin");
    let quiet = ATTACK
        .replace(r#""script":[{"at":1,"run":"coop attack swing 32"},{"at":2,"run":"coop attack shoot 40"},{"at":3,"run":"coop attack swing 32"}]"#, r#""script":[]"#)
        .replace(r#""script":[{"at":1,"run":"coop attack swing 32"},{"at":2,"run":"coop attack reload 90"},{"at":3,"run":"coop attack shoot 40"},{"at":4,"run":"coop attack swing 32"}]"#, r#""script":[]"#);
    let r = run(
        &scn(&format!(r#"{quiet},"expect_fail":true"#)),
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

//! The start flow in the simulator: host `new_game`, a guest that makes its own character,
//! per-player `VCG00`/`VCG01` rows, and `start.phase` per peer. Generated world, no game data
//! (the quests are keyed by their form ids, `00102037` and `00104C1C`; the generated data has no
//! quest records, so the host's own opening stages are covered by the real-data scenario
//! `tools/coop-run/scenarios/live_new_game_start.json`).

use std::sync::Arc;

use coop_headless::data::Data;
use coop_headless::scenario::Scenario;
use coop_headless::sim::{run, RunOpts};

fn data(tag: &str) -> (testdata::TempData, Arc<Data>) {
    let t = testdata::fighting::fighting(tag);
    let d = Arc::new(Data::open(t.path()).expect("generated data opens"));
    (t, d)
}

const START: &str = r#""name":"start","network":"perfect","seed":3,"new_game":true,"cell":"TestFightCell",
  "host":{"script":[]},
  "guest":{"script":[]},
  "expect":[
    {"id":"guest-phase","path":"start.phase","op":"eq","value":"playing","on":"guest"},
    {"id":"host-phase","path":"start.phase","op":"eq","value":"character_creation","on":"host"},
    {"id":"guest-venue","path":"start.venue","op":"eq","value":"host_place","on":"guest"},
    {"id":"guest-vcg01","path":"player_quests.guest.00104C1C.stage","op":"ge","value":105,"on":"both"},
    {"id":"not-shared","path":"quests.00104C1C","op":"absent","on":"both"},
    {"id":"h","path":"hash.shared","on":"equal"}]"#;

fn scn(body: &str) -> Scenario {
    Scenario::parse(&format!(r#"{{"duration_s":14,"settle_s":4,{body}}}"#), "t").unwrap()
}

#[test]
fn a_guest_makes_its_own_character_while_the_host_is_in_the_opening() {
    let (_t, d) = data("hl-start");
    let r = run(&scn(START), &d, &RunOpts::default());
    assert_eq!(
        r.verdict.result, "pass",
        "{}\n{}",
        r.verdict.reason, r.dumps[1]
    );
    assert!(
        r.dumps[1].contains("\"start\":{\"phase\":\"playing\""),
        "{}",
        r.dumps[1]
    );
    let m = r
        .verdict
        .json
        .get("modelled")
        .and_then(|m| m.as_arr())
        .unwrap();
    assert!(
        m.iter().any(|x| x
            .as_str()
            .is_some_and(|s| s.starts_with("character creation"))),
        "the menus are declared as stood in"
    );
}

#[test]
fn the_twin_where_the_guest_skips_its_character_fails_as_expected() {
    let (_t, d) = data("hl-start-twin");
    // no new game and no character creation: the guest plays at once and never gets VCG rows
    let twin = START.replace(r#""new_game":true,"#, "");
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

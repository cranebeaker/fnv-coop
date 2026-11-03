//! The party-save scenarios of `tools/coop-run/scenarios` on a generated world (no game data):
//! the same files, with the names of Doc's house swapped for the generated world's.

use std::sync::Arc;

use coop_headless::data::Data;
use coop_headless::scenario::Scenario;
use coop_headless::sim::{run, RunOpts};

fn data(tag: &str) -> (testdata::TempData, Arc<Data>) {
    let t = testdata::fighting::fighting(tag);
    let d = Arc::new(Data::open(t.path()).expect("generated data opens"));
    (t, d)
}

const FILES: [(&str, &str); 6] = [
    (
        "live_party_save_roundtrip",
        include_str!("../../../tools/coop-run/scenarios/live_party_save_roundtrip.json"),
    ),
    (
        "twin_live_party_save_roundtrip",
        include_str!("../../../tools/coop-run/scenarios/twin_live_party_save_roundtrip.json"),
    ),
    (
        "live_guest_rejoin_own_character",
        include_str!("../../../tools/coop-run/scenarios/live_guest_rejoin_own_character.json"),
    ),
    (
        "twin_live_guest_rejoin_own_character",
        include_str!("../../../tools/coop-run/scenarios/twin_live_guest_rejoin_own_character.json"),
    ),
    (
        "live_late_joiner_with_save",
        include_str!("../../../tools/coop-run/scenarios/live_late_joiner_with_save.json"),
    ),
    (
        "twin_live_late_joiner_with_save",
        include_str!("../../../tools/coop-run/scenarios/twin_live_late_joiner_with_save.json"),
    ),
];

fn scn(name: &str) -> Scenario {
    let text = FILES
        .iter()
        .find(|(n, _)| *n == name)
        .expect("known scenario")
        .1;
    let text = text
        .replace("GSDocMitchellHouse", "TestFightCell")
        .replace("DocMitchellREF", "TestGuardRef")
        .replace("Caps001", "TestSMG")
        .replace("Stimpak", "TestRifle");
    Scenario::parse(&text, name).unwrap_or_else(|e| panic!("{name}: {e}"))
}

fn check(name: &str, opts: RunOpts) {
    let (_t, d) = data(name);
    let r = run(&scn(name), &d, &opts);
    let twin = name.starts_with("twin_");
    let want = if twin { ("pass", "twin") } else { ("pass", "") };
    assert_eq!(r.verdict.result, want.0, "{name}: {}", r.verdict.reason);
    if twin {
        assert_eq!(r.verdict.class, "twin", "{name}: {}", r.verdict.reason);
    }
}

#[test]
fn the_party_scenario_files_parse() {
    for (n, _) in FILES {
        let s = scn(n);
        assert!(s.party.is_some(), "{n}");
        assert_eq!(s.expect_fail, n.starts_with("twin_"), "{n}");
    }
}

#[test]
fn live_party_save_roundtrip() {
    check("live_party_save_roundtrip", RunOpts::default());
}

#[test]
fn live_party_save_roundtrip_twin_fails_as_expected() {
    check("twin_live_party_save_roundtrip", RunOpts::default());
}

#[test]
fn live_guest_rejoin_own_character() {
    check("live_guest_rejoin_own_character", RunOpts::default());
}

#[test]
fn live_guest_rejoin_own_character_twin_fails_as_expected() {
    check("twin_live_guest_rejoin_own_character", RunOpts::default());
}

#[test]
fn live_late_joiner_with_save() {
    check("live_late_joiner_with_save", RunOpts::default());
}

#[test]
fn live_late_joiner_with_save_twin_fails_as_expected() {
    check("twin_live_late_joiner_with_save", RunOpts::default());
}

/// The soak: every scenario over every chaos profile and a few seeds, and the same seed twice
/// gives identical dumps.
#[test]
fn party_scenarios_survive_every_chaos_profile() {
    let (_t, d) = data("party-soak");
    let mut bad = Vec::new();
    for name in [
        "live_party_save_roundtrip",
        "live_guest_rejoin_own_character",
        "live_late_joiner_with_save",
    ] {
        for net in [
            "perfect",
            "good_wifi",
            "bad_wifi",
            "mobile",
            "congested",
            "flaky",
            "partition",
        ] {
            for seed in [1u64, 2, 3] {
                let o = RunOpts {
                    seed: Some(seed),
                    net: Some(net.into()),
                    ..RunOpts::default()
                };
                let r = run(&scn(name), &d, &o);
                if r.verdict.result != "pass" {
                    bad.push(format!("{name} {net} seed {seed}: {}", r.verdict.reason));
                }
            }
        }
    }
    assert!(bad.is_empty(), "{bad:#?}");
    let o = RunOpts {
        seed: Some(9),
        net: Some("bad_wifi".into()),
        ..RunOpts::default()
    };
    let a = run(&scn("live_party_save_roundtrip"), &d, &o);
    let b = run(&scn("live_party_save_roundtrip"), &d, &o);
    assert_eq!(a.dumps, b.dumps, "deterministic");
}

//! Combat depth scenarios on the headless simulator (generated world): NPC
//! attacks on a remote player, downed and revive, real-time VATS, area
//! blasts, per-player stealth, encounter extras. Each has a mutation twin
//! that must fail, and the whole set is soaked over every chaos profile.

use std::sync::Arc;

use coop_headless::data::Data;
use coop_headless::scenario::Scenario;
use coop_headless::sim::{run, RunOpts};
use coop_headless::suite::{plan, run_jobs};

fn data(tag: &str) -> (testdata::TempData, Arc<Data>) {
    let t = testdata::fighting::fighting(tag);
    let d = Arc::new(Data::open(t.path()).expect("generated data opens"));
    (t, d)
}

fn scn(duration: u32, body: &str) -> Scenario {
    Scenario::parse(
        &format!(r#"{{"cell":"TestFightCell","duration_s":{duration},"settle_s":4,{body}}}"#),
        "t",
    )
    .unwrap()
}

const NPC_ATTACKS: &str = r#""name":"live_npc_attacks_remote_player","network":"good_wifi","seed":11,
  "host":{"script":[{"at":3,"run":"coop npchit TestGeckoRef 1 10"},{"at":4,"run":"coop npchit TestGeckoRef 1 10"}]},
  "guest":{"script":[{"at":3.5,"run":"coop npchit TestGeckoRef 0 10"}]},
  "expect":[
    {"id":"sent","path":"combat.npc_hits_sent","op":"ge","value":2,"on":"host"},
    {"id":"guest-felt-both","path":"combat.hurts_applied","op":"eq","value":2,"on":"guest"},
    {"id":"host-felt-one","path":"combat.hurts_applied","op":"eq","value":1,"on":"host"},
    {"id":"h","path":"hash.shared","on":"equal"}]"#;

const DOWNED_REVIVE: &str = r#""name":"live_downed_revive_from_npc","network":"good_wifi","seed":12,
  "host":{"script":[{"at":0.5,"run":"coop health 0 100"},{"at":0.6,"run":"coop health 1 100"},{"at":3,"run":"coop npchit TestGeckoRef 1 500"},
    {"at":4.5,"run":"coop npchit TestGeckoRef 1 30"},{"at":6,"run":"coop revive 1"},{"at":15,"run":"coop npchit TestGeckoRef 1 5"}]},
  "guest":{"script":[]},
  "expect":[
    {"id":"went-down","path":"combat.downs","op":"ge","value":1,"on":"guest"},
    {"id":"downed-hit-ignored-later-hit-lands","path":"combat.hurts_applied","op":"eq","value":2,"on":"guest"},
    {"id":"h","path":"hash.shared","on":"equal"}]"#;

const VATS: &str = r#""name":"live_vats_no_freeze","network":"good_wifi","seed":13,
  "host":{"script":[{"at":1,"run":"coop teleport 100 200 0"},{"at":4,"key":"w:5"}],"extra_args":["--walk"]},
  "guest":{"script":[{"at":3,"run":"coop vatsmark 1"},{"at":4,"run":"coop vats TestGeckoRef 0 1000 4000"},
    {"at":4,"run":"coop vats TestGeckoRef 0 1000 4000"},{"at":4,"run":"coop vats TestGeckoRef 0 1000 4000"},{"at":9,"run":"coop vatsmark 0"}]},
  "expect":[
    {"id":"one-queue","path":"combat.vats_sent","op":"eq","value":1,"on":"guest"},
    {"id":"hits","path":"combat.vats_hits","op":"ge","value":1,"on":"guest"},
    {"id":"world-clock-kept","path":"combat.vats_clock_kept","op":"ge","value":1,"on":"guest"},
    {"id":"dead-host","path":"dead.TestGeckoRef","op":"eq","value":true,"on":"host"},
    {"id":"dead-guest","path":"dead.TestGeckoRef","op":"eq","value":true,"on":"guest"},
    {"id":"host-kept-walking","path":"positions.host","op":"dist_from","from":[100,200],"cmp":"ge","value":250,"on":"guest"},
    {"id":"h","path":"hash.shared","on":"equal"}]"#;

const EXPLOSION: &str = r#""name":"live_explosion_area","network":"good_wifi","seed":14,
  "host":{"script":[{"at":1,"run":"TestGeckoRef.MoveTo TestGuardRef"},{"at":2.5,"run":"coop ff 1"},{"at":5,"run":"coop explode TestGrenade 4000 3000"}]},
  "guest":{"script":[]},
  "expect":[
    {"id":"npc-dead-host","path":"dead.TestGeckoRef","op":"eq","value":true,"on":"host"},
    {"id":"npc-dead-guest","path":"dead.TestGeckoRef","op":"eq","value":true,"on":"guest"},
    {"id":"blast-hits","path":"combat.blast_hits","op":"ge","value":2,"on":"host"},
    {"id":"player-hit","path":"combat.hurts_applied","op":"ge","value":1,"on":"guest"},
    {"id":"shown","path":"combat.blasts_seen","op":"ge","value":1,"on":"guest"},
    {"id":"h","path":"hash.shared","on":"equal"}]"#;

const STEALTH: &str = r#""name":"live_stealth_per_player","network":"good_wifi","seed":15,
  "host":{"script":[{"at":3,"run":"coop probe TestGeckoRef 700"},{"at":9,"run":"coop probe TestGeckoRef 700"}]},
  "guest":{"script":[{"at":5,"run":"coop sneak 1"}]},
  "expect":[
    {"id":"sneaking-hides-only-the-sneaker","path":"combat.probe_drop","op":"ge","value":10,"on":"host"},
    {"id":"h","path":"hash.shared","on":"equal"}]"#;

const EXTRAS: &str = r#""name":"live_extra_enemies_scaling","network":"good_wifi","seed":16,
  "host":{"script":[{"at":3,"run":"coop spawnenc TestGeckoRef 40"}]},
  "guest":{"script":[]},
  "expect":[
    {"id":"some-extras","path":"combat.extras_made","op":"ge","value":1,"on":"host"},
    {"id":"capped-at-2-per-added-player","path":"combat.extras_made","op":"le","value":2,"on":"host"},
    {"id":"h","path":"hash.shared","on":"equal"}]"#;

/// (scenario body, duration, the mutation that removes the cause)
fn all() -> Vec<(&'static str, u32, String)> {
    vec![
        (NPC_ATTACKS, 9, NPC_ATTACKS.replace(r#""script":[{"at":3,"run":"coop npchit TestGeckoRef 1 10"},{"at":4,"run":"coop npchit TestGeckoRef 1 10"}]"#, r#""script":[]"#)),
        (DOWNED_REVIVE, 20, DOWNED_REVIVE.replace(r#"{"at":6,"run":"coop revive 1"},"#, "")),
        (VATS, 12, VATS.replace(r#"{"at":4,"run":"coop vats TestGeckoRef 0 1000 4000"},"#, "")),
        (EXPLOSION, 9, EXPLOSION.replace(r#"{"at":2.5,"run":"coop ff 1"},"#, "")),
        (STEALTH, 13, STEALTH.replace(r#"{"at":5,"run":"coop sneak 1"}"#, "")),
        (EXTRAS, 9, EXTRAS.replace(r#"{"at":3,"run":"coop spawnenc TestGeckoRef 40"}"#, "")),
    ]
}

#[test]
fn every_combat_scenario_passes_and_its_twin_fails() {
    let (_t, d) = data("combat-scn");
    for (body, secs, twin) in all() {
        let s = scn(secs, body);
        let r = run(&s, &d, &RunOpts::default());
        assert_eq!(r.verdict.result, "pass", "{}: {}", s.name, r.verdict.reason);
        assert_ne!(twin, body, "{}: the twin changes something", s.name);
        let t = scn(secs, &format!(r#"{twin},"expect_fail":true"#));
        let r = run(&t, &d, &RunOpts::default());
        assert_eq!(
            (r.verdict.result.as_str(), r.verdict.class.as_str()),
            ("pass", "twin"),
            "{} twin: {}",
            s.name,
            r.verdict.reason
        );
    }
}

#[test]
fn the_dump_carries_the_combat_counters() {
    let (_t, d) = data("combat-dump");
    let r = run(&scn(9, NPC_ATTACKS), &d, &RunOpts::default());
    assert!(
        r.dumps
            .iter()
            .all(|x| x.contains("\"combat\":{\"npc_hits_sent\":")),
        "{}",
        r.dumps[0]
    );
}

#[test]
fn soak_over_every_chaos_profile() {
    let (_t, d) = data("combat-soak");
    let scns: Vec<(String, Scenario)> = all()
        .into_iter()
        .map(|(b, secs, _)| (format!("c{secs}.json"), scn(secs + 6, b)))
        .collect();
    let jobs = plan(&scns, Some(&[1, 2]), true, None);
    let done = run_jobs(&d, jobs, 3, None, true, None, false, false);
    assert!(
        done.iter().all(|x| x.hash2 == Some(x.hash)),
        "every run is deterministic"
    );
    let failed: Vec<String> = done
        .iter()
        .filter(|x| x.result != "pass")
        .map(|x| format!("{} [{}] {}", x.scenario, x.net, x.line))
        .collect();
    assert!(failed.is_empty(), "{failed:#?}");
}

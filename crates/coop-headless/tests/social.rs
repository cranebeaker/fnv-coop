//! Chat, voice, markers, compatibility and four peer runs on a generated world.

use std::sync::Arc;

use coop_headless::data::Data;
use coop_headless::multi::{run_multi, Limits, MultiOpts};
use coop_headless::scenario::Scenario;
use coop_headless::sim::{run, RunOpts};

fn data(tag: &str) -> (testdata::TempData, Arc<Data>) {
    let t = testdata::fighting::fighting(tag);
    let d = Arc::new(Data::open(t.path()).expect("generated data opens"));
    (t, d)
}

fn scn(body: &str) -> Scenario {
    Scenario::parse(
        &format!(r#"{{"cell":"TestFightCell","duration_s":12,"settle_s":3,{body}}}"#),
        "t",
    )
    .unwrap()
}

const CHAT: &str = r#""name":"chat","network":"good_wifi","seed":3,
  "host":{"script":[{"at":3,"run":"coop say hi there"}]},
  "guest":{"script":[{"at":4,"run":"coop say hello back"}]},
  "expect":[{"id":"h","path":"live.chat_in","op":"ge","value":1,"on":"host"},{"id":"g","path":"live.chat_in","op":"ge","value":1,"on":"guest"},
            {"id":"hash","path":"hash.shared","on":"equal"}]"#;

const VOICE: &str = r#""name":"voice","network":"bad_wifi","seed":4,
  "host":{"script":[{"at":3,"run":"coop voice 2"}]},
  "guest":{"script":[{"at":7,"run":"coop voice 2"}]},
  "expect":[{"id":"gin","path":"live.voice_in","op":"ge","value":50,"on":"guest"},{"id":"hin","path":"live.voice_in","op":"ge","value":50,"on":"host"},
            {"id":"mix","path":"live.voiced_frames","op":"ge","value":30,"on":"both"},{"id":"ind","path":"live.speaking_frames","op":"ge","value":1,"on":"both"},
            {"id":"hash","path":"hash.shared","on":"equal"}]"#;

const MARK: &str = r#""name":"marks","network":"good_wifi","seed":5,
  "host":{"script":[{"at":4,"run":"coop waypoint 150 250"},{"at":6,"run":"coop ping 400"}]},
  "guest":{"script":[{"at":3,"run":"coop ping 500"},{"at":5,"run":"coop waypoint 900 100"}]},
  "expect":[{"id":"hm","path":"live.markers","op":"ge","value":1,"on":"host"},{"id":"gm","path":"live.markers","op":"ge","value":1,"on":"guest"},
            {"id":"gp","path":"live.pings","op":"ge","value":2,"on":"guest"},{"id":"hp","path":"live.pings","op":"ge","value":2,"on":"host"},
            {"id":"hash","path":"hash.shared","on":"equal"}]"#;

fn strip_scripts(body: &str) -> String {
    let mut b = body.to_string();
    while let Some(i) = b.find(r#""script":["#) {
        let j = b[i..].find("]}").unwrap() + i;
        b.replace_range(i..j + 1, r#""script":[]"#);
        b = b.replacen(r#""script":[]"#, r#""script_":[]"#, 1);
    }
    b.replace(r#""script_":[]"#, r#""script":[]"#)
}

fn passes(body: &str, tag: &str) {
    let (_t, d) = data(tag);
    let r = run(&scn(body), &d, &RunOpts::default());
    assert_eq!(r.verdict.result, "pass", "{}", r.verdict.reason);
}

fn twin_fails(body: &str, tag: &str) {
    let (_t, d) = data(tag);
    let twin = strip_scripts(body);
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
fn chat_round_trips_through_the_host() {
    passes(CHAT, "soc-chat");
}

#[test]
fn chat_twin_fails_as_expected() {
    twin_fails(CHAT, "soc-chat-twin");
}

#[test]
fn voice_packets_flow_both_ways_over_a_bad_link() {
    passes(VOICE, "soc-voice");
}

#[test]
fn voice_twin_fails_as_expected() {
    twin_fails(VOICE, "soc-voice-twin");
}

#[test]
fn markers_and_pings_are_shared() {
    passes(MARK, "soc-mark");
}

#[test]
fn markers_twin_fails_as_expected() {
    twin_fails(MARK, "soc-mark-twin");
}

const COMPAT: &str = r#""name":"compat","network":"perfect","seed":6,"hash_check":false,"expect_refused":true,
  "host":{"plugins":["FalloutNV.esm","DeadMoney.esm"]},
  "guest":{"plugins":["FalloutNV.esm"]},
  "expect":[{"id":"g","path":"live.compat_refused","op":"ge","value":1,"on":"guest"},{"id":"h","path":"live.compat_refused","op":"ge","value":1,"on":"host"},
            {"id":"alone","path":"live.avatars","op":"eq","value":0,"on":"host"}]"#;

#[test]
fn a_guest_missing_a_plugin_is_refused() {
    passes(COMPAT, "soc-compat");
}

#[test]
fn compat_twin_with_matching_plugins_joins_and_fails_its_checks() {
    let (_t, d) = data("soc-compat-twin");
    let twin = COMPAT.replace(
        r#""guest":{"plugins":["FalloutNV.esm"]}"#,
        r#""guest":{"plugins":["FalloutNV.esm","DeadMoney.esm"]}"#,
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
fn the_override_admits_the_guest_with_a_warning() {
    let (_t, d) = data("soc-compat-ok");
    let body = r#""name":"compat-ok","network":"perfect","seed":6,
      "host":{"plugins":["FalloutNV.esm","A.esp"],"allow_mismatch":true},
      "guest":{"plugins":["FalloutNV.esm","B.esp"]},
      "expect":[{"id":"w","path":"live.compat_warned","op":"ge","value":1,"on":"both"},{"id":"in","path":"live.avatars","op":"ge","value":1,"on":"both"},
                {"id":"hash","path":"hash.shared","on":"equal"}]"#;
    let r = run(&scn(body), &d, &RunOpts::default());
    assert_eq!(r.verdict.result, "pass", "{}", r.verdict.reason);
}

const FOUR: &str = r#""name":"four","network":"good_wifi","seed":7,"duration_s":16,
  "host":{"script":[{"at":3,"run":"coop say host here"},{"at":9,"run":"coop waypoint 10 20"}]},
  "guests":[
    {"script":[{"at":4,"run":"coop say one"},{"at":3,"run":"coop voice 2"}]},
    {"script":[{"at":5,"run":"coop say two"},{"at":7,"run":"coop voice 2"},{"at":4,"run":"coop ping 300"}]},
    {"script":[{"at":6,"run":"coop say three"},{"at":11,"run":"coop voice 2"},{"at":12,"run":"coop waypoint 40 50"}]}],
  "expect":[{"id":"chat","path":"live.chat_in","op":"ge","value":3,"on":"all"},{"id":"voice","path":"live.voice_in","op":"ge","value":50,"on":"all"},
            {"id":"av","path":"live.avatars","op":"ge","value":1,"on":"all"},{"id":"mk","path":"live.markers","op":"ge","value":1,"on":"all"},
            {"id":"hash","path":"hash.shared","on":"equal"}]"#;

#[test]
fn four_peers_agree_and_stay_inside_the_budget() {
    let (_t, d) = data("soc-four");
    let r = run_multi(&scn(FOUR), &d, &MultiOpts::default());
    assert_eq!(
        r.verdict.result,
        "pass",
        "{} | {:?}",
        r.verdict.reason,
        r.checks
            .iter()
            .filter(|c| !c.ok)
            .map(|c| &c.detail)
            .collect::<Vec<_>>()
    );
    assert_eq!(r.budget.peers.len(), 4);
    println!("{}", r.budget.report());
    let over = r.budget.violations(&Limits::default(), false);
    assert!(over.is_empty(), "{over:?}");
    for g in &r.budget.peers[1..] {
        assert!(
            g.down_bps > 100.0,
            "{} received almost nothing: {:.0} B/s",
            g.name,
            g.down_bps
        );
    }
    assert!(r.budget.peers[0].frames > 100);
}

#[test]
fn four_peer_runs_are_deterministic() {
    let (_t, d) = data("soc-four-det");
    let a = run_multi(&scn(FOUR), &d, &MultiOpts::default());
    let b = run_multi(&scn(FOUR), &d, &MultiOpts::default());
    assert_eq!(a.hash, b.hash);
}

#[test]
fn four_peer_twin_fails_as_expected() {
    let (_t, d) = data("soc-four-twin");
    let twin = FOUR
        .replace(r#"{"at":3,"run":"coop say host here"},{"at":9,"run":"coop waypoint 10 20"}"#, "")
        .replace(r#"{"at":4,"run":"coop say one"},{"at":3,"run":"coop voice 2"}"#, "")
        .replace(r#"{"at":5,"run":"coop say two"},{"at":7,"run":"coop voice 2"},{"at":4,"run":"coop ping 300"}"#, "")
        .replace(r#"{"at":6,"run":"coop say three"},{"at":11,"run":"coop voice 2"},{"at":12,"run":"coop waypoint 40 50"}"#, "");
    let r = run_multi(
        &scn(&format!(r#"{twin},"expect_fail":true"#)),
        &d,
        &MultiOpts::default(),
    );
    assert_eq!(
        (r.verdict.result.as_str(), r.verdict.class.as_str()),
        ("pass", "twin"),
        "{}",
        r.verdict.reason
    );
}

#[test]
fn four_peers_converge_under_every_chaos_profile() {
    let (_t, d) = data("soc-four-chaos");
    for net in ["good_wifi", "bad_wifi", "mobile", "congested", "flaky"] {
        let mut o = MultiOpts::default();
        o.run.net = Some(net.to_string());
        let mut s = scn(FOUR);
        // under loss the voice/chat counts vary; the shared state must still agree
        s.expect.retain(|e| e.id == "hash" || e.id == "av");
        let r = run_multi(&s, &d, &o);
        assert_eq!(r.verdict.result, "pass", "{net}: {}", r.verdict.reason);
    }
}

#[test]
fn shipped_scenarios_parse() {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("scenarios");
    let mut n = 0;
    for e in std::fs::read_dir(dir).unwrap() {
        let p = e.unwrap().path();
        if p.extension().is_some_and(|x| x == "json") {
            Scenario::load(&p).unwrap_or_else(|e| panic!("{}: {e}", p.display()));
            n += 1;
        }
    }
    assert!(n >= 15, "{n}");
}

#[test]
fn budget_limits_catch_a_chatty_peer() {
    use coop_headless::multi::{Budget, PeerBudget};
    let p = |name: &str, up: f64, down: f64| PeerBudget {
        name: name.into(),
        up_bps: up,
        down_bps: down,
        ..PeerBudget::default()
    };
    let ok = Budget {
        peers: vec![p("host", 100_000.0, 1_000.0), p("guest", 5_000.0, 40_000.0)],
        steady_s: 10.0,
    };
    assert!(ok.violations(&Limits::default(), true).is_empty());
    let bad = Budget {
        peers: vec![
            p("host", 300_000.0, 1_000.0),
            p("guest", 30_000.0, 70_000.0),
        ],
        steady_s: 10.0,
    };
    assert_eq!(bad.violations(&Limits::default(), true).len(), 3);
    let slow = Budget {
        peers: vec![PeerBudget {
            name: "host".into(),
            frame_mean_us: 9_000.0,
            frame_p99_us: 30_000.0,
            ..PeerBudget::default()
        }],
        steady_s: 1.0,
    };
    assert_eq!(slow.violations(&Limits::default(), true).len(), 2);
    assert!(
        slow.violations(&Limits::default(), false).is_empty(),
        "debug builds are not gated on CPU"
    );
}

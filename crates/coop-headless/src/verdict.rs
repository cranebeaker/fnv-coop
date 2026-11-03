//! Judging a run: the classes, `verdict.json` and the failure packet of
//! `tools/coop-run/verdict.py`, plus the headless-only `needs_assets` skip.

use std::collections::BTreeMap;

use crate::expect::{evaluate, Check};
use crate::json::{n, obj, s, J};
use crate::scenario::Scenario;

/// The packet limit of the real harness, bytes of JSON.
pub const PACKET_LIMIT: usize = 3000;

/// What one simulated peer left behind (the viewer's log and dump).
#[derive(Clone, Debug, Default)]
pub struct PeerFacts {
    /// `host` or `guest`.
    pub role: String,
    /// A `start` event was logged.
    pub started: bool,
    /// `coop_ready` was logged.
    pub ready: bool,
    /// A clean `shutdown` event was logged.
    pub shutdown_clean: bool,
    /// `error` events.
    pub errors: Vec<String>,
    /// `coop_repair` events.
    pub repairs: u32,
    /// Events logged (count).
    pub events: usize,
    /// The final state dump.
    pub state: Option<J>,
    /// The last events, as JSON objects.
    pub tail: Vec<J>,
}

/// Datagram counters of one direction.
#[derive(Clone, Copy, Debug, Default)]
pub struct Dir {
    /// Handed to the link.
    pub inn: u64,
    /// Delivered.
    pub forwarded: u64,
    /// Lost.
    pub dropped: u64,
    /// Doubled.
    pub duplicated: u64,
    /// Delayed.
    pub reordered: u64,
}

/// What the simulated link did.
#[derive(Clone, Debug, Default)]
pub struct RelayFacts {
    /// Profile name at the start.
    pub profile: String,
    /// The guest spoke.
    pub armed: bool,
    /// Guest to host.
    pub g2h: Dir,
    /// Host to guest.
    pub h2g: Dir,
    /// Every scheduled outage began.
    pub outages_fired: bool,
}

/// Everything `judge` needs.
#[derive(Clone, Debug, Default)]
pub struct RunFacts {
    /// Host.
    pub host: PeerFacts,
    /// Guest.
    pub guest: PeerFacts,
    /// Link.
    pub relay: RelayFacts,
    /// Run id.
    pub run_id: String,
    /// Replay command.
    pub rerun: String,
    /// Seed used.
    pub seed: u64,
    /// Virtual seconds the run covered.
    pub duration_s: f64,
    /// The simulator itself failed.
    pub harness_error: Option<String>,
    /// The scenario needs data this folder lacks.
    pub needs_assets: Option<String>,
    /// Stand-ins the headless model used for this run (viewer behaviour it replaces).
    pub modelled: Vec<String>,
}

/// A judged run.
pub struct Verdict {
    /// The verdict.json object.
    pub json: J,
    /// `pass`, `fail` or `skip`.
    pub result: String,
    /// Class.
    pub class: String,
    /// Reason.
    pub reason: String,
}

fn dir_json(d: &Dir) -> J {
    obj(vec![
        ("in", n(d.inn as f64)),
        ("forwarded", n(d.forwarded as f64)),
        ("dropped", n(d.dropped as f64)),
        ("duplicated", n(d.duplicated as f64)),
        ("reordered", n(d.reordered as f64)),
    ])
}

fn loss_of(profile: &str) -> f64 {
    coop_core::transport::ChaosProfile::by_name(profile).map_or(0.0, |p| p.loss)
}

/// Judges a run like `verdict.judge`.
pub fn judge(scn: &Scenario, run: &RunFacts) -> Verdict {
    let (mut cls, mut reason): (Option<&str>, String) = (None, String::new());
    let fail = |c: &'static str, why: String, cls: &mut Option<&str>, reason: &mut String| {
        if cls.is_none() {
            *cls = Some(c);
            *reason = why;
        }
    };
    if let Some(why) = &run.needs_assets {
        return skip(scn, run, why);
    }
    if let Some(e) = &run.harness_error {
        fail("harness", e.clone(), &mut cls, &mut reason);
    }
    for p in [&run.host, &run.guest] {
        if p.started && !p.shutdown_clean {
            fail(
                "crash",
                format!("{} has no clean shutdown event", p.role),
                &mut cls,
                &mut reason,
            );
        }
    }
    for p in [&run.host, &run.guest] {
        if let Some(e) = p.errors.first() {
            fail(
                "log",
                format!(
                    "{} logged error: {}",
                    p.role,
                    e.chars().take(120).collect::<String>()
                ),
                &mut cls,
                &mut reason,
            );
        }
    }
    for p in [&run.host, &run.guest] {
        if !p.started {
            fail(
                "hollow",
                format!("{} wrote no start event (log missing or empty)", p.role),
                &mut cls,
                &mut reason,
            );
        } else if !p.ready {
            fail(
                "hollow",
                format!(
                    "{} never reached coop_ready (single player fallback or no join)",
                    p.role
                ),
                &mut cls,
                &mut reason,
            );
        } else if p.state.is_none() {
            fail(
                "hollow",
                format!("{} wrote no state dump", p.role),
                &mut cls,
                &mut reason,
            );
        } else if p
            .state
            .as_ref()
            .and_then(|s| s.get("role"))
            .and_then(J::as_str)
            .is_some_and(|r| r != p.role)
        {
            fail(
                "hollow",
                format!("{} state dump says another role", p.role),
                &mut cls,
                &mut reason,
            );
        }
    }
    let (g, h) = (&run.relay.g2h, &run.relay.h2g);
    if !run.relay.armed || g.inn == 0 || h.inn == 0 {
        fail(
            "hollow",
            format!(
                "relay saw no traffic in both directions (g2h={} h2g={})",
                g.inn, h.inn
            ),
            &mut cls,
            &mut reason,
        );
    } else {
        let total = g.inn + h.inn;
        let did: u64 = [g, h]
            .iter()
            .map(|d| d.dropped + d.duplicated + d.reordered)
            .sum();
        let loss = loss_of(&scn.network);
        if loss > 0.0 && total as f64 * loss >= 8.0 && did == 0 {
            fail(
                "hollow",
                format!(
                    "profile {} should have dropped about {} of {} datagrams, relay changed none",
                    scn.network,
                    (total as f64 * loss).round(),
                    total
                ),
                &mut cls,
                &mut reason,
            );
        }
    }
    if !scn.outages.is_empty() && !run.relay.outages_fired {
        fail(
            "hollow",
            "scheduled outage never fired".into(),
            &mut cls,
            &mut reason,
        );
    }
    let mut checks: Vec<Check> = Vec::new();
    if let (Some(hs), Some(gs)) = (&run.host.state, &run.guest.state) {
        checks = evaluate(&scn.expect, hs, gs);
        if scn.hash_check {
            if let (Some(a), Some(b)) = (hs.path("hash.shared"), gs.path("hash.shared")) {
                let d = |v: &J| v.as_str().unwrap_or("").to_string();
                checks.push(Check {
                    id: "auto.hash.shared".into(),
                    ok: a == b,
                    kind: "desync",
                    detail: format!("hash.shared host={} guest={}", d(a), d(b)),
                });
            }
        }
        for kind in ["desync", "assert"] {
            if let Some(c) = checks.iter().find(|c| !c.ok && c.kind == kind) {
                fail(
                    if kind == "desync" { "desync" } else { "assert" },
                    format!("{}: {}", c.id, c.detail),
                    &mut cls,
                    &mut reason,
                );
            }
        }
    }
    let max_rep = scn.max_repairs.or(if scn.network == "perfect" {
        Some(0)
    } else {
        None
    });
    let reps = run.host.repairs + run.guest.repairs;
    if max_rep.is_some_and(|m| reps > m) {
        fail(
            "repair",
            format!(
                "{reps} repairs (host {}, guest {}), allowed {}",
                run.host.repairs,
                run.guest.repairs,
                max_rep.unwrap_or(0)
            ),
            &mut cls,
            &mut reason,
        );
    }
    let mut twin = false;
    if scn.expect_fail {
        match cls {
            Some("assert") | Some("desync") => {
                twin = true;
                reason = format!("twin failed as expected ({reason})");
                cls = None;
            }
            None => {
                cls = Some("twin");
                reason = "twin passed: its assertions did not fail, so they cannot tell the feature from nothing".into();
            }
            _ => {}
        }
    }
    let class = cls
        .unwrap_or(if twin { "twin" } else { "pass" })
        .to_string();
    let result = if cls.is_some() { "fail" } else { "pass" };
    let reason = if reason.is_empty() {
        "all checks passed".to_string()
    } else {
        reason
    };
    let mut v = base(scn, run, result, &class, &reason);
    if let J::Obj(m) = &mut v {
        m.insert(
            "checks".into(),
            J::Arr(
                checks
                    .iter()
                    .map(|c| {
                        obj(vec![
                            ("id", s(c.id.clone())),
                            ("ok", J::Bool(c.ok)),
                            ("detail", s(c.detail.chars().take(160).collect::<String>())),
                        ])
                    })
                    .collect(),
            ),
        );
        if cls.is_some() {
            m.insert("packet".into(), packet(scn, run, &class, &reason, &checks));
        }
    }
    Verdict {
        json: v,
        result: result.into(),
        class,
        reason,
    }
}

fn skip(scn: &Scenario, run: &RunFacts, why: &str) -> Verdict {
    let v = base(scn, run, "skip", "needs_assets", why);
    Verdict {
        json: v,
        result: "skip".into(),
        class: "needs_assets".into(),
        reason: why.into(),
    }
}

fn base(scn: &Scenario, run: &RunFacts, result: &str, class: &str, reason: &str) -> J {
    let peer = |p: &PeerFacts| {
        obj(vec![
            ("exit_code", n(0.0)),
            ("timed_out", J::Bool(false)),
            ("events", n(p.events as f64)),
            ("ready", J::Bool(p.ready)),
            ("shutdown_clean", J::Bool(p.shutdown_clean)),
            ("repairs", n(p.repairs as f64)),
        ])
    };
    let mut m: BTreeMap<String, J> = BTreeMap::new();
    for (k, v) in [
        ("schema", n(1.0)),
        ("scenario", s(scn.name.clone())),
        ("run_id", s(run.run_id.clone())),
        ("result", s(result)),
        ("class", s(class)),
        ("reason", s(reason)),
        ("network", s(scn.network.clone())),
        ("seed", n(run.seed as f64)),
        ("duration_s", n((run.duration_s * 10.0).round() / 10.0)),
        ("rerun", s(run.rerun.clone())),
        ("engine", s("headless")),
        (
            "modelled",
            J::Arr(run.modelled.iter().map(|x| s(x.clone())).collect()),
        ),
        (
            "relay",
            obj(vec![
                ("profile", s(run.relay.profile.clone())),
                ("armed", J::Bool(run.relay.armed)),
                ("g2h", dir_json(&run.relay.g2h)),
                ("h2g", dir_json(&run.relay.h2g)),
            ]),
        ),
        (
            "peers",
            obj(vec![("host", peer(&run.host)), ("guest", peer(&run.guest))]),
        ),
        ("checks", J::Arr(vec![])),
    ] {
        m.insert(k.into(), v);
    }
    J::Obj(m)
}

fn clip(t: &str, max: usize) -> String {
    if t.chars().count() <= max {
        t.to_string()
    } else {
        format!(
            "{}...",
            t.chars().take(max.saturating_sub(3)).collect::<String>()
        )
    }
}

/// The failure packet, always under [`PACKET_LIMIT`] bytes of JSON.
pub fn packet(scn: &Scenario, run: &RunFacts, class: &str, reason: &str, checks: &[Check]) -> J {
    let failed: Vec<&Check> = checks.iter().filter(|c| !c.ok).collect();
    let mut last = J::Null;
    for tail_n in [4usize, 2, 1, 0] {
        for cl in [160usize, 80, 40] {
            let tail =
                |p: &PeerFacts| J::Arr(p.tail.iter().rev().take(tail_n).rev().cloned().collect());
            let pk = obj(vec![
                ("scenario", s(scn.name.clone())),
                ("class", s(class)),
                ("reason", s(clip(reason, 200))),
                ("network", s(scn.network.clone())),
                ("seed", n(run.seed as f64)),
                ("rerun", s(clip(&run.rerun, 200))),
                (
                    "failed_checks",
                    J::Arr(
                        failed
                            .iter()
                            .take(4)
                            .map(|c| {
                                obj(vec![
                                    ("id", s(c.id.clone())),
                                    ("detail", s(clip(&c.detail, cl))),
                                ])
                            })
                            .collect(),
                    ),
                ),
                (
                    "relay",
                    obj(vec![
                        ("profile", s(run.relay.profile.clone())),
                        ("g2h", dir_json(&run.relay.g2h)),
                        ("h2g", dir_json(&run.relay.h2g)),
                    ]),
                ),
                (
                    "ready",
                    obj(vec![
                        ("host", J::Bool(run.host.ready)),
                        ("guest", J::Bool(run.guest.ready)),
                    ]),
                ),
                (
                    "tail",
                    obj(vec![("host", tail(&run.host)), ("guest", tail(&run.guest))]),
                ),
            ]);
            if pk.dump().len() < PACKET_LIMIT {
                return pk;
            }
            last = pk;
        }
    }
    if let J::Obj(m) = &mut last {
        m.remove("tail");
        if let Some(J::Arr(a)) = m.get_mut("failed_checks") {
            a.truncate(1);
        }
    }
    last
}

/// The last output line, in the runner's format.
pub fn result_line(v: &Verdict) -> String {
    let j = &v.json;
    let g = |k: &str| j.get(k).and_then(J::as_str).unwrap_or("").to_string();
    let seed = j.get("seed").and_then(J::as_f64).unwrap_or(0.0);
    let dur = j.get("duration_s").and_then(J::as_f64).unwrap_or(0.0);
    match (v.result.as_str(), v.class.as_str()) {
        ("pass", "twin") => format!(
            "RESULT: PASS twin {} ({}) {}",
            g("scenario"),
            g("network"),
            clip(&v.reason, 140)
        ),
        ("pass", _) => format!(
            "RESULT: PASS {} ({}, seed {}, {:.0} s)",
            g("scenario"),
            g("network"),
            seed,
            dur
        ),
        ("skip", _) => format!("RESULT: SKIP needs_assets {}: {}", g("scenario"), v.reason),
        _ => format!(
            "RESULT: FAIL {} {}: {} | rerun: {}",
            v.class,
            g("scenario"),
            v.reason,
            g("rerun")
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scn(extra: &str) -> Scenario {
        Scenario::parse(&format!(r#"{{"cell":"X","network":"perfect","expect":[{{"id":"q","path":"quests.Q.stage","op":"eq","value":1}}]{extra}}}"#), "t").unwrap()
    }

    fn peer(role: &str, stage: i32) -> PeerFacts {
        let st = J::parse(&format!(
            r#"{{"role":"{role}","quests":{{"Q":{{"stage":{stage}}}}},"hash":{{"shared":"a"}}}}"#
        ))
        .unwrap();
        PeerFacts {
            role: role.into(),
            started: true,
            ready: true,
            shutdown_clean: true,
            state: Some(st),
            events: 5,
            ..Default::default()
        }
    }

    fn facts() -> RunFacts {
        let d = Dir {
            inn: 5,
            forwarded: 5,
            ..Dir::default()
        };
        RunFacts {
            host: peer("host", 1),
            guest: peer("guest", 1),
            relay: RelayFacts {
                profile: "perfect".into(),
                armed: true,
                g2h: d,
                h2g: d,
                outages_fired: true,
            },
            rerun: "coop-sim run t.json --seed 1".into(),
            seed: 1,
            duration_s: 12.0,
            ..Default::default()
        }
    }

    #[test]
    fn baseline_passes_and_the_known_bad_runs_fail_with_their_class() {
        let sc = scn("");
        assert_eq!(judge(&sc, &facts()).class, "pass");
        let mut f = facts();
        f.host.started = false;
        assert_eq!(judge(&sc, &f).class, "hollow");
        let mut f = facts();
        f.guest.ready = false;
        assert_eq!(judge(&sc, &f).class, "hollow");
        let mut f = facts();
        f.host.state = None;
        assert_eq!(judge(&sc, &f).class, "hollow");
        let mut f = facts();
        f.relay.armed = false;
        assert_eq!(judge(&sc, &f).class, "hollow");
        let mut f = facts();
        f.host.shutdown_clean = false;
        assert_eq!(judge(&sc, &f).class, "crash");
        let mut f = facts();
        f.guest.errors.push("x".into());
        assert_eq!(judge(&sc, &f).class, "log");
        let mut f = facts();
        f.host.repairs = 1;
        assert_eq!(judge(&sc, &f).class, "repair");
        let mut f = facts();
        f.guest = peer("guest", 2);
        assert_eq!(judge(&sc, &f).class, "assert");
        let mut f = facts();
        f.guest.state = Some(
            J::parse(r#"{"role":"guest","quests":{"Q":{"stage":1}},"hash":{"shared":"b"}}"#)
                .unwrap(),
        );
        assert_eq!(judge(&sc, &f).class, "desync");
    }

    #[test]
    fn a_hollow_chaos_run_and_a_missed_outage_are_caught() {
        let sc = Scenario::parse(
            r#"{"cell":"X","network":"flaky","outages":[{"at":1,"seconds":1}]}"#,
            "t",
        )
        .unwrap();
        let mut f = facts();
        f.relay.g2h.inn = 100;
        assert_eq!(judge(&sc, &f).class, "hollow");
        f.relay.g2h.dropped = 20;
        f.relay.outages_fired = false;
        assert_eq!(judge(&sc, &f).reason, "scheduled outage never fired");
    }

    #[test]
    fn twins_invert_assert_failures_only() {
        let sc = scn(r#","expect_fail":true"#);
        assert_eq!(judge(&sc, &facts()).class, "twin");
        assert_eq!(judge(&sc, &facts()).result, "fail");
        let mut f = facts();
        f.guest = peer("guest", 9);
        let v = judge(&sc, &f);
        assert_eq!((v.result.as_str(), v.class.as_str()), ("pass", "twin"));
        assert!(
            result_line(&v).starts_with("RESULT: PASS twin t (perfect) twin failed as expected")
        );
        f.host.shutdown_clean = false;
        assert_eq!(
            judge(&sc, &f).class,
            "crash",
            "a crash is never hidden by a twin"
        );
    }

    #[test]
    fn the_packet_stays_under_three_kilobytes_and_result_lines_match_the_runner() {
        let sc = scn("");
        let mut f = facts();
        f.guest = peer("guest", 2);
        f.guest.tail = (0..6)
            .map(|i| {
                obj(vec![
                    ("ev", s("script")),
                    ("line", s("x".repeat(400))),
                    ("i", n(i as f64)),
                ])
            })
            .collect();
        f.host.tail = f.guest.tail.clone();
        let v = judge(&sc, &f);
        assert!(v.json.get("packet").unwrap().dump().len() < PACKET_LIMIT);
        assert!(result_line(&v).starts_with(
            "RESULT: FAIL assert t: q@guest: guest quests.Q.stage eq 1 (got 2) | rerun: "
        ));
        let ok = judge(&sc, &facts());
        assert_eq!(result_line(&ok), "RESULT: PASS t (perfect, seed 1, 12 s)");
        let mut f = facts();
        f.needs_assets = Some("no meshes".into());
        let sk = judge(&sc, &f);
        assert_eq!(sk.class, "needs_assets");
        assert_eq!(result_line(&sk), "RESULT: SKIP needs_assets t: no meshes");
    }
}

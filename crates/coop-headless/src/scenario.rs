//! Scenario files, the same JSON `tools/coop-run/scenario.py` reads.

use std::path::Path;

use crate::json::J;

/// The chaos profiles of coop-core.
pub const PROFILES: [&str; 7] = ["perfect", "good_wifi", "bad_wifi", "mobile", "congested", "flaky", "partition"];
const OPS: [&str; 11] = ["eq", "ne", "ge", "le", "gt", "lt", "near", "present", "absent", "dist_from", "sum"];

/// What one script line does.
#[derive(Clone, Debug, PartialEq)]
pub enum Action {
    /// A script-language line (`--run` / `--run-at`), maybe `local ` or `coop ` prefixed.
    Run(String),
    /// `SetStage QUEST N`.
    Stage(String, u16),
    /// `--talk`.
    Talk,
    /// `--choose N,N`.
    Choose(Vec<i64>),
    /// `--say TEXT`.
    Say(String),
    /// `--key-at`: key name and hold seconds.
    Key(String, f64),
}

/// One script line, with its time if it has one (seconds from ready).
#[derive(Clone, Debug, PartialEq)]
pub struct Line {
    /// Seconds after the peer is ready (`None`: at ready).
    pub at: Option<f64>,
    /// What it does.
    pub action: Action,
}

/// One peer's script.
#[derive(Clone, Debug, Default)]
pub struct Side {
    /// Lines.
    pub script: Vec<Line>,
    /// Extra viewer flags (`--walk` is the only one that matters here).
    pub extra_args: Vec<String>,
}

/// A scheduled link outage.
#[derive(Clone, Debug)]
pub struct Outage {
    /// Seconds after the guest first talks.
    pub at: f64,
    /// Length.
    pub seconds: f64,
    /// `both`, `g2h`, `h2g`.
    pub dir: String,
}

/// One end-state check.
#[derive(Clone, Debug)]
pub struct Expect {
    /// Name.
    pub id: String,
    /// Dotted path into the dump.
    pub path: String,
    /// Operator.
    pub op: String,
    /// `host`, `guest`, `both`, `equal`.
    pub on: String,
    /// Wanted value.
    pub value: Option<J>,
    /// Tolerance.
    pub tol: Option<f64>,
    /// Keep the first N list components.
    pub dims: Option<usize>,
    /// `dist_from` origin.
    pub from: Vec<f64>,
    /// Comparison for `sum` and `dist_from`.
    pub cmp: String,
    /// More paths for `sum`.
    pub add: Vec<String>,
}

/// A loaded scenario.
#[derive(Clone, Debug)]
pub struct Scenario {
    /// Name.
    pub name: String,
    /// Description.
    pub description: String,
    /// Cell editor id.
    pub cell: Option<String>,
    /// Start a new game (needs the opening cell and character generation).
    pub new_game: bool,
    /// Chaos profile.
    pub network: String,
    /// Seed.
    pub seed: u64,
    /// Seconds of chaos and script time.
    pub duration_s: f64,
    /// Seconds of quiet before the final dump.
    pub settle_s: f64,
    /// Whole-run timeout of the real harness.
    pub timeout_s: f64,
    /// Outages.
    pub outages: Vec<Outage>,
    /// Allowed repairs (`None`: 0 on perfect, unlimited otherwise).
    pub max_repairs: Option<u32>,
    /// When the link turns perfect (`None`: after `duration_s`).
    pub chaos_s: Option<f64>,
    /// Checks.
    pub expect: Vec<Expect>,
    /// A mutation twin.
    pub expect_fail: bool,
    /// Compare `hash.shared` automatically.
    pub hash_check: bool,
    /// Host script.
    pub host: Side,
    /// Guest script.
    pub guest: Side,
}

impl Scenario {
    /// Loads and validates a file.
    pub fn load(path: &Path) -> Result<Scenario, String> {
        let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
        let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("scenario");
        Scenario::parse(&text, stem)
    }

    /// Parses and validates text; `default_name` fills a missing `name`.
    pub fn parse(text: &str, default_name: &str) -> Result<Scenario, String> {
        let j = J::parse(text)?;
        if !matches!(j, J::Obj(_)) {
            return Err("scenario must be a JSON object".into());
        }
        let mut errs: Vec<String> = Vec::new();
        let num = |k: &str, d: f64, errs: &mut Vec<String>| match j.get(k) {
            None => d,
            Some(J::Num(n)) if *n >= 0.0 => *n,
            _ => {
                errs.push(format!("{k} must be a number >= 0"));
                d
            }
        };
        let name = j.get("name").and_then(J::as_str).unwrap_or(default_name).to_string();
        let cell = j.get("cell").and_then(J::as_str).filter(|s| !s.is_empty()).map(str::to_string);
        let new_game = j.get("new_game").and_then(J::as_bool).unwrap_or(false);
        let network = j.get("network").and_then(J::as_str).unwrap_or("perfect").to_string();
        let mut s = Scenario {
            name,
            description: j.get("description").and_then(J::as_str).unwrap_or("").to_string(),
            cell,
            new_game,
            network,
            seed: 1,
            duration_s: num("duration_s", 20.0, &mut errs),
            settle_s: num("settle_s", 3.0, &mut errs),
            timeout_s: num("timeout_s", 180.0, &mut errs),
            outages: Vec::new(),
            max_repairs: None,
            chaos_s: None,
            expect: Vec::new(),
            expect_fail: j.get("expect_fail").and_then(J::as_bool).unwrap_or(false),
            hash_check: j.get("hash_check").and_then(J::as_bool).unwrap_or(true),
            host: Side::default(),
            guest: Side::default(),
        };
        if s.cell.is_none() && !s.new_game {
            errs.push("cell is required unless new_game is true".into());
        }
        if !PROFILES.contains(&s.network.as_str()) {
            errs.push(format!("network {:?} is not a profile ({})", s.network, PROFILES.join(", ")));
        }
        match j.get("seed") {
            None => {}
            Some(J::Num(n)) if n.fract() == 0.0 => s.seed = *n as u64,
            _ => errs.push("seed must be an integer".into()),
        }
        match j.get("chaos_s") {
            None | Some(J::Null) => {}
            Some(J::Num(n)) => s.chaos_s = Some(*n),
            _ => errs.push("chaos_s must be a number".into()),
        }
        match j.get("max_repairs") {
            None | Some(J::Null) => {}
            Some(J::Num(n)) => s.max_repairs = Some(*n as u32),
            _ => errs.push("max_repairs must be a number".into()),
        }
        for (i, o) in j.get("outages").and_then(J::as_arr).unwrap_or(&[]).iter().enumerate() {
            let dir = o.get("dir").and_then(J::as_str).unwrap_or("both").to_string();
            match (o.get("at").and_then(J::as_f64), o.get("seconds").and_then(J::as_f64)) {
                (Some(at), Some(seconds)) if ["both", "g2h", "h2g"].contains(&dir.as_str()) => {
                    s.outages.push(Outage { at, seconds, dir })
                }
                _ => errs.push(format!("outages[{i}] needs at, seconds and optional dir both|g2h|h2g")),
            }
        }
        for (role, side) in [("host", &mut s.host), ("guest", &mut s.guest)] {
            if let Some(r) = j.get(role) {
                side.extra_args = r
                    .get("extra_args")
                    .and_then(J::as_arr)
                    .map(|a| a.iter().filter_map(|x| x.as_str().map(str::to_string)).collect())
                    .unwrap_or_default();
                for (i, l) in r.get("script").and_then(J::as_arr).unwrap_or(&[]).iter().enumerate() {
                    match parse_line(l) {
                        Ok(line) => side.script.push(line),
                        Err(e) => errs.push(format!("{role}.script[{i}]: {e}")),
                    }
                }
            }
        }
        let mut ids: Vec<String> = Vec::new();
        for (i, e) in j.get("expect").and_then(J::as_arr).unwrap_or(&[]).iter().enumerate() {
            match parse_expect(e, i) {
                Ok(x) => {
                    if ids.contains(&x.id) {
                        errs.push(format!("expect id {:?} repeated", x.id));
                    }
                    ids.push(x.id.clone());
                    s.expect.push(x);
                }
                Err(es) => errs.extend(es),
            }
        }
        if errs.is_empty() {
            Ok(s)
        } else {
            Err(errs.join("; "))
        }
    }

    /// Seconds of chaos before the link turns perfect.
    pub fn chaos_seconds(&self) -> f64 {
        self.chaos_s.unwrap_or(self.duration_s)
    }
}

fn parse_line(l: &J) -> Result<Line, String> {
    const KEYS: [&str; 6] = ["run", "stage", "talk", "choose", "say", "key"];
    let present: Vec<&str> = KEYS.iter().copied().filter(|k| l.get(k).is_some()).collect();
    if present.len() != 1 {
        return Err("needs exactly one of run/stage/talk/choose/say/key".into());
    }
    let at = match l.get("at") {
        None => None,
        Some(J::Num(n)) if *n >= 0.0 => Some(*n),
        _ => return Err("at must be seconds >= 0".into()),
    };
    let k = present[0];
    if at.is_some() && ["talk", "choose", "say"].contains(&k) {
        return Err(format!("{k} cannot have a time (it is a startup flag)"));
    }
    let v = l.get(k).expect("present");
    let action = match k {
        "run" => Action::Run(v.as_str().ok_or("run must be a string")?.to_string()),
        "stage" => {
            let t = v.as_str().unwrap_or("");
            let w: Vec<&str> = t.split_whitespace().collect();
            match (w.len(), w.get(1).and_then(|n| n.parse::<u16>().ok())) {
                (2, Some(n)) => Action::Stage(w[0].to_string(), n),
                _ => return Err("stage must be \"QUEST STAGE\"".into()),
            }
        }
        "talk" => {
            if v.as_bool() == Some(false) {
                return Ok(Line { at, action: Action::Choose(Vec::new()) });
            }
            Action::Talk
        }
        "choose" => {
            let a = v.as_arr().ok_or("choose must be a list of integers")?;
            Action::Choose(a.iter().map(|x| x.as_f64().map(|n| n as i64)).collect::<Option<_>>().ok_or("choose must be a list of integers")?)
        }
        "say" => Action::Say(v.as_str().ok_or("say must be a string")?.to_string()),
        _ => {
            let t = v.as_str().unwrap_or("").trim().to_string();
            if t.is_empty() || t.split_whitespace().count() != 1 {
                return Err("key must be \"KEY\" or \"KEY:HOLD\" (no spaces)".into());
            }
            if at.is_none() {
                return Err("key needs a time (at)".into());
            }
            let (key, hold) = match t.split_once(':') {
                Some((k, h)) => (k.to_string(), h.parse::<f64>().ok().filter(|h| *h > 0.0).ok_or("key hold must be a number > 0")?),
                None => (t, 0.1),
            };
            Action::Key(key, hold)
        }
    };
    Ok(Line { at, action })
}

fn parse_expect(e: &J, i: usize) -> Result<Expect, Vec<String>> {
    let id = e.get("id").and_then(J::as_str).map(str::to_string).unwrap_or_else(|| format!("e{i}"));
    let op = e.get("op").and_then(J::as_str).unwrap_or("eq").to_string();
    let on = e.get("on").and_then(J::as_str).unwrap_or("both").to_string();
    let path = e.get("path").and_then(J::as_str).unwrap_or("").to_string();
    let mut errs = Vec::new();
    if !OPS.contains(&op.as_str()) {
        errs.push(format!("expect {id}: op {op:?} not in {OPS:?}"));
    }
    if !["host", "guest", "both", "equal"].contains(&on.as_str()) {
        errs.push(format!("expect {id}: on {on:?} not in host/guest/both/equal"));
    }
    if path.is_empty() {
        errs.push(format!("expect {id}: path is required"));
    }
    let value = e.get("value").cloned();
    if on != "equal" && !["present", "absent"].contains(&op.as_str()) && value.is_none() {
        errs.push(format!("expect {id}: value is required"));
    }
    let tol = e.get("tol").and_then(J::as_f64);
    if op == "near" && tol.is_none() {
        errs.push(format!("expect {id}: near needs tol"));
    }
    let cmp = e.get("cmp").and_then(J::as_str).unwrap_or("").to_string();
    let add: Vec<String> = e.get("add").and_then(J::as_arr).map(|a| a.iter().filter_map(|x| x.as_str().map(str::to_string)).collect()).unwrap_or_default();
    if op == "sum" {
        if !["eq", "ge", "le", "gt", "lt"].contains(&cmp.as_str()) {
            errs.push(format!("expect {id}: sum needs cmp eq|ge|le|gt|lt"));
        }
        if value.as_ref().and_then(J::as_f64).is_none() {
            errs.push(format!("expect {id}: sum needs a numeric value"));
        }
    }
    let from: Vec<f64> = e.get("from").and_then(J::as_arr).map(|a| a.iter().filter_map(J::as_f64).collect()).unwrap_or_default();
    if op == "dist_from" {
        if from.len() < 2 {
            errs.push(format!("expect {id}: dist_from needs from: [x, y(, z)]"));
        }
        if !["ge", "le", "gt", "lt"].contains(&cmp.as_str()) {
            errs.push(format!("expect {id}: dist_from needs cmp ge|le|gt|lt"));
        }
        if value.as_ref().and_then(J::as_f64).is_none() {
            errs.push(format!("expect {id}: dist_from needs a numeric value (units)"));
        }
    }
    if !errs.is_empty() {
        return Err(errs);
    }
    Ok(Expect { id, path, op, on, value, tol, dims: e.get("dims").and_then(J::as_f64).map(|n| n as usize), from, cmp, add })
}

#[cfg(test)]
mod tests {
    use super::*;

    const BASE: &str = r#"{"name":"t","cell":"X","network":"bad_wifi","seed":21,"outages":[{"at":6,"seconds":3}],
      "host":{"script":[{"at":3,"stage":"VCG03 10"},{"run":"set G to 3"},{"at":2,"key":"w:1.5"}],"extra_args":["--walk"]},
      "guest":{"script":[{"talk":true},{"choose":[1,2]}]},
      "expect":[{"id":"a","path":"quests.VCG03.stage","op":"ge","value":10},{"path":"hash.shared","on":"equal"}]}"#;

    #[test]
    fn parses_a_full_scenario() {
        let s = Scenario::parse(BASE, "d").unwrap();
        assert_eq!((s.name.as_str(), s.seed, s.network.as_str()), ("t", 21, "bad_wifi"));
        assert_eq!(s.host.script.len(), 3);
        assert_eq!(s.host.script[0].action, Action::Stage("VCG03".into(), 10));
        assert_eq!(s.host.script[2].action, Action::Key("w".into(), 1.5));
        assert_eq!(s.guest.script[0].action, Action::Talk);
        assert_eq!(s.expect[1].id, "e1");
        assert_eq!(s.outages[0].dir, "both");
        assert_eq!(s.chaos_seconds(), 20.0);
    }

    #[test]
    fn rejects_what_the_python_loader_rejects() {
        for (bad, why) in [
            (r#"{"network":"perfect"}"#, "cell is required"),
            (r#"{"cell":"X","network":"nope"}"#, "not a profile"),
            (r#"{"cell":"X","host":{"script":[{"run":"a","stage":"A 1"}]}}"#, "exactly one"),
            (r#"{"cell":"X","host":{"script":[{"at":1,"talk":true}]}}"#, "startup flag"),
            (r#"{"cell":"X","host":{"script":[{"key":"w"}]}}"#, "needs a time"),
            (r#"{"cell":"X","expect":[{"path":"a","op":"near","value":1}]}"#, "needs tol"),
            (r#"{"cell":"X","expect":[{"path":"a","op":"sum","cmp":"eq"}]}"#, "numeric value"),
            (r#"{"cell":"X","expect":[{"path":"a","op":"dist_from","value":1,"cmp":"ge"}]}"#, "from:"),
            (r#"{"cell":"X","outages":[{"at":1}]}"#, "outages[0]"),
        ] {
            let e = Scenario::parse(bad, "d").unwrap_err();
            assert!(e.contains(why), "{bad}: {e}");
        }
    }
}

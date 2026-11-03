//! End-state checks on the two dumps (the ops of `scenario.py`'s `evaluate`).

use crate::json::J;
use crate::scenario::Expect;

/// One check result.
#[derive(Clone, Debug)]
pub struct Check {
    /// `id` or `id@peer`.
    pub id: String,
    /// Held.
    pub ok: bool,
    /// `assert` or `desync` (equal checks).
    pub kind: &'static str,
    /// One line.
    pub detail: String,
}

fn short(v: Option<&J>) -> String {
    let t = match v {
        None => "<missing>".to_string(),
        Some(v) => py_dump(v),
    };
    if t.chars().count() <= 80 {
        t
    } else {
        format!("{}...", t.chars().take(77).collect::<String>())
    }
}

/// `json.dumps(sort_keys=True)` spacing.
fn py_dump(v: &J) -> String {
    match v {
        J::Arr(a) => format!("[{}]", a.iter().map(py_dump).collect::<Vec<_>>().join(", ")),
        J::Obj(m) => format!(
            "{{{}}}",
            m.iter()
                .map(|(k, v)| format!("{}: {}", J::Str(k.clone()).dump(), py_dump(v)))
                .collect::<Vec<_>>()
                .join(", ")
        ),
        other => other.dump(),
    }
}

fn dims(v: &J, e: &Expect) -> J {
    match (v, e.dims) {
        (J::Arr(a), Some(n)) => J::Arr(a.iter().take(n).cloned().collect()),
        _ => v.clone(),
    }
}

fn near(a: &J, b: &J, tol: f64) -> bool {
    match (a, b) {
        (J::Arr(x), J::Arr(y)) => {
            let n = x.len().min(y.len());
            n > 0 && (0..n).all(|i| near(&x[i], &y[i], tol))
        }
        (J::Num(x), J::Num(y)) => (x - y).abs() <= tol,
        _ => false,
    }
}

fn cmp_num(op: &str, got: f64, want: f64) -> bool {
    match op {
        "eq" => got == want,
        "ne" => got != want,
        "ge" => got >= want,
        "le" => got <= want,
        "gt" => got > want,
        "lt" => got < want,
        _ => false,
    }
}

fn cmp(op: &str, got: &J, want: &J, tol: f64) -> bool {
    match op {
        "eq" => got == want,
        "ne" => got != want,
        "near" => near(got, want, tol),
        _ => match (got, want) {
            (J::Num(a), J::Num(b)) => cmp_num(op, *a, *b),
            _ => false,
        },
    }
}

/// XY distance between a position list and a point.
pub fn dist_xy(a: &J, b: &[f64]) -> Option<f64> {
    let a = a.as_arr()?;
    let (x, y) = (a.first()?.as_f64()?, a.get(1)?.as_f64()?);
    Some(((x - b[0]).powi(2) + (y - b[1]).powi(2)).sqrt())
}

/// Runs every check against the final dumps.
pub fn evaluate(expect: &[Expect], host: &J, guest: &J) -> Vec<Check> {
    let mut out = Vec::new();
    for e in expect {
        if e.on == "equal" {
            let (a, b) = (host.path(&e.path), guest.path(&e.path));
            let ok = match (a, b) {
                (Some(a), Some(b)) => match e.tol {
                    Some(t) => near(&dims(a, e), &dims(b, e), t),
                    None => a == b,
                },
                _ => false,
            };
            out.push(Check {
                id: e.id.clone(),
                ok,
                kind: "desync",
                detail: format!("{} host={} guest={}", e.path, short(a), short(b)),
            });
            continue;
        }
        let who: Vec<(&str, &J)> = if e.on == "both" {
            vec![("host", host), ("guest", guest)]
        } else if e.on == "host" {
            vec![("host", host)]
        } else {
            vec![("guest", guest)]
        };
        let multi = who.len() > 1;
        for (w, st) in who {
            let id = if multi {
                format!("{}@{}", e.id, w)
            } else {
                e.id.clone()
            };
            let got = st.path(&e.path);
            let want = e.value.as_ref();
            let (ok, detail) = match e.op.as_str() {
                "sum" => {
                    let total: f64 = std::iter::once(&e.path)
                        .chain(e.add.iter())
                        .filter_map(|p| st.path(p).and_then(J::as_f64))
                        .sum();
                    let parts = std::iter::once(e.path.clone())
                        .chain(e.add.iter().cloned())
                        .collect::<Vec<_>>()
                        .join(", ");
                    (
                        cmp_num(&e.cmp, total, want.and_then(J::as_f64).unwrap_or(0.0)),
                        format!(
                            "{w} sum({parts}) {} {} (got {}; missing counts 0)",
                            e.cmp,
                            short(want),
                            short(Some(&J::Num(total)))
                        ),
                    )
                }
                "dist_from" => {
                    let d = got.and_then(|g| dist_xy(g, &e.from));
                    let ok = d.is_some_and(|d| {
                        cmp_num(&e.cmp, d, want.and_then(J::as_f64).unwrap_or(0.0))
                    });
                    let fr = J::Arr(e.from.iter().map(|x| J::Num(*x)).collect());
                    (
                        ok,
                        format!(
                            "{w} {} dist_from {} {} {} (got {})",
                            e.path,
                            short(Some(&fr)),
                            e.cmp,
                            short(want),
                            d.map_or("<missing>".into(), |d| format!("{d:.1}"))
                        ),
                    )
                }
                "present" => (
                    got.is_some(),
                    format!("{w} {} present (got {})", e.path, short(got)),
                ),
                "absent" => (
                    got.is_none(),
                    format!("{w} {} absent (got {})", e.path, short(got)),
                ),
                op => {
                    let ok = match (got, want) {
                        (Some(g), Some(v)) => {
                            cmp(op, &dims(g, e), &dims(v, e), e.tol.unwrap_or(0.0))
                        }
                        _ => false,
                    };
                    (
                        ok,
                        format!("{w} {} {op} {} (got {})", e.path, short(want), short(got)),
                    )
                }
            };
            out.push(Check {
                id,
                ok,
                kind: "assert",
                detail,
            });
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scenario::Scenario;

    fn run(expect: &str, host: &str, guest: &str) -> Vec<Check> {
        let s = Scenario::parse(&format!(r#"{{"cell":"X","expect":{expect}}}"#), "t").unwrap();
        evaluate(
            &s.expect,
            &J::parse(host).unwrap(),
            &J::parse(guest).unwrap(),
        )
    }

    #[test]
    fn comparison_ops() {
        let c = run(
            r#"[{"id":"a","path":"q.s","op":"ge","value":10},{"id":"b","path":"q.s","op":"eq","value":10,"on":"host"},
                {"id":"c","path":"q.x","op":"absent"},{"id":"d","path":"q.s","op":"present","on":"guest"},
                {"id":"e","path":"q.s","op":"lt","value":10}]"#,
            r#"{"q":{"s":10}}"#,
            r#"{"q":{"s":12}}"#,
        );
        let ok: Vec<(&str, bool)> = c.iter().map(|x| (x.id.as_str(), x.ok)).collect();
        assert_eq!(
            ok,
            vec![
                ("a@host", true),
                ("a@guest", true),
                ("b", true),
                ("c@host", true),
                ("c@guest", true),
                ("d", true),
                ("e@host", false),
                ("e@guest", false)
            ]
        );
    }

    #[test]
    fn near_dist_sum_and_equal() {
        let c = run(
            r#"[{"id":"n","path":"p","op":"near","value":[100,50],"tol":5,"on":"host"},
                {"id":"d","path":"p","op":"dist_from","from":[0,0],"cmp":"ge","value":100,"on":"host"},
                {"id":"s","path":"i.a","op":"sum","add":["i.b","i.zz"],"cmp":"eq","value":50,"on":"guest"},
                {"id":"e","path":"p","on":"equal","tol":40,"dims":2},{"id":"h","path":"hash.shared","on":"equal"}]"#,
            r#"{"p":[102,48,7,0],"hash":{"shared":"a"}}"#,
            r#"{"p":[120,48,900,1],"i":{"a":20,"b":30},"hash":{"shared":"b"}}"#,
        );
        let m: Vec<(&str, bool, &str)> = c.iter().map(|x| (x.id.as_str(), x.ok, x.kind)).collect();
        assert_eq!(
            m,
            vec![
                ("n", true, "assert"),
                ("d", true, "assert"),
                ("s", true, "assert"),
                ("e", true, "desync"),
                ("h", false, "desync")
            ]
        );
        assert!(c[1].detail.contains("(got 112.7)"), "{}", c[1].detail);
    }

    #[test]
    fn a_missing_path_fails_comparisons_and_dist() {
        let c = run(
            r#"[{"id":"a","path":"nope","op":"eq","value":1,"on":"host"},{"id":"b","path":"nope","op":"dist_from","from":[0,0],"cmp":"ge","value":1,"on":"host"}]"#,
            "{}",
            "{}",
        );
        assert!(c.iter().all(|x| !x.ok));
        assert!(c[0].detail.contains("<missing>"));
    }
}

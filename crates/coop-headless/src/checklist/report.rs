//! The one-screen report, `docs/COOP-CHECKLIST.md` and the JSON file.

use std::collections::BTreeMap;
use std::fmt::Write;

use super::run::Report;
use super::{Outcome, Status};

/// Counts of one group of rows.
#[derive(Clone, Copy, Debug, Default)]
pub struct Counts {
    /// Rows.
    pub rows: usize,
    /// Green without a stand-in.
    pub pass: usize,
    /// Green with a named stand-in.
    pub modelled: usize,
    /// Failing.
    pub fail: usize,
    /// Pending.
    pub pending: usize,
    /// Skipped for missing data.
    pub skip: usize,
}

impl Counts {
    fn add(&mut self, o: &Outcome) {
        self.rows += 1;
        match &o.status {
            Status::Pass if o.row.modelled() => self.modelled += 1,
            Status::Pass => self.pass += 1,
            Status::Fail(_) => self.fail += 1,
            Status::Pending(_) => self.pending += 1,
            Status::Skip(_) => self.skip += 1,
        }
    }
    /// Green rows.
    pub fn green(&self) -> usize {
        self.pass + self.modelled
    }
}

/// Counts per area, in first-seen order, and the total.
pub fn by_area(r: &Report) -> (Vec<(String, Counts)>, Counts) {
    let mut order: Vec<String> = Vec::new();
    let mut m: BTreeMap<String, Counts> = BTreeMap::new();
    let mut total = Counts::default();
    for o in &r.outcomes {
        if !order.contains(&o.row.area) {
            order.push(o.row.area.clone());
        }
        m.entry(o.row.area.clone()).or_default().add(o);
        total.add(o);
    }
    (
        order.into_iter().map(|a| (a.clone(), m[&a])).collect(),
        total,
    )
}

/// The final verdict line.
pub fn verdict_line(t: &Counts) -> String {
    let mut s = format!(
        "CO-OP CHECKLIST: {}/{} items green, {} pending, {} failing",
        t.green(),
        t.rows,
        t.pending,
        t.fail
    );
    if t.skip > 0 {
        let _ = write!(s, ", {} skipped for missing data", t.skip);
    }
    s
}

/// True when every row is green.
pub fn all_green(t: &Counts) -> bool {
    t.rows > 0 && t.green() == t.rows
}

fn sp_summary(r: &Report) -> (usize, BTreeMap<String, (usize, String)>) {
    let mut m: BTreeMap<String, (usize, String)> = BTreeMap::new();
    let mut quests = 0;
    for q in &r.quests {
        if !q.sp_gaps.is_empty() {
            quests += 1;
        }
        for (k, v) in &q.sp_gaps {
            let e = m
                .entry(k.clone())
                .or_insert((0, format!("{}: {}", q.edid, v)));
            e.0 += 1;
        }
    }
    (quests, m)
}

fn quest_groups(r: &Report) -> Vec<(String, usize, usize, usize, usize)> {
    let mut m: BTreeMap<String, (usize, usize, usize, usize)> = BTreeMap::new();
    for o in &r.outcomes {
        if !o.row.id.starts_with("quest.") {
            continue;
        }
        let e = m.entry(o.row.area.clone()).or_default();
        e.0 += 1;
        if o.green() {
            e.1 += 1;
        }
        if o.row.modelled() {
            e.2 += 1;
        }
        if o.sp_gaps > 0 {
            e.3 += 1;
        }
    }
    m.into_iter()
        .map(|(k, v)| (k, v.0, v.1, v.2, v.3))
        .collect()
}

fn clip(s: &str, n: usize) -> String {
    if s.chars().count() <= n {
        s.to_string()
    } else {
        let mut t: String = s.chars().take(n.saturating_sub(3)).collect();
        t.push_str("...");
        t
    }
}

/// The one-screen text.
pub fn screen(r: &Report) -> String {
    let (areas, total) = by_area(r);
    let mut s = String::new();
    let o = &r.opts;
    let _ = writeln!(
        s,
        "co-op checklist | backend {} | seeds {} | net {} | peers {} | {} plugins | {:.1} s",
        o.backend,
        o.seeds
            .as_ref()
            .map_or("scenario's own".to_string(), |v| format!(
                "{}..{}",
                v.first().copied().unwrap_or(0),
                v.last().copied().unwrap_or(0)
            )),
        o.net.as_deref().unwrap_or("scenario's own"),
        o.peers,
        r.plugins.len(),
        r.wall_s
    );
    let _ = writeln!(
        s,
        "{:<30} {:>5} {:>5} {:>8} {:>5} {:>7} {:>5}",
        "AREA", "rows", "PASS", "MODELLED", "FAIL", "PENDING", "SKIP"
    );
    for (a, c) in areas.iter().filter(|(a, _)| !a.starts_with("Quests: ")) {
        let _ = writeln!(
            s,
            "{:<30} {:>5} {:>5} {:>8} {:>5} {:>7} {:>5}",
            clip(a, 30),
            c.rows,
            c.pass,
            c.modelled,
            c.fail,
            c.pending,
            c.skip
        );
    }
    let groups = quest_groups(r);
    if !groups.is_empty() {
        let _ = writeln!(
            s,
            "{:<30} {:>5} {:>5} {:>8} {:>5}",
            "QUEST WALK (per quest)", "quests", "green", "policy", "SP-GAP"
        );
        for (g, n, green, po, sp) in &groups {
            let _ = writeln!(
                s,
                "{:<30} {:>5} {:>5} {:>8} {:>5}",
                clip(g.trim_start_matches("Quests: "), 30),
                n,
                green,
                po,
                sp
            );
        }
        let (spq, spf) = sp_summary(r);
        let _ = writeln!(s, "SP-GAP: {spq} quests hit {} unsupported single-player script functions (not co-op failures)", spf.len());
    }
    for (shown, o) in r
        .outcomes
        .iter()
        .filter(|o| matches!(o.status, Status::Fail(_)))
        .enumerate()
    {
        if shown == 12 {
            let _ = writeln!(
                s,
                "  ... {} more failing, see docs/COOP-CHECKLIST.md",
                total.fail - 12
            );
            break;
        }
        if let Status::Fail(m) = &o.status {
            let _ = writeln!(s, "FAIL {}: {}", o.row.id, clip(m, 150));
        }
    }
    let _ = writeln!(s, "{}", verdict_line(&total));
    s
}

fn esc(s: &str) -> String {
    s.replace('|', "/").replace('\n', " ")
}

/// `docs/COOP-CHECKLIST.md`.
pub fn markdown(r: &Report) -> String {
    let (areas, total) = by_area(r);
    let mut s = String::new();
    let _ = writeln!(s, "# Co-op master checklist\n");
    let _ = writeln!(s, "Generated by `coop-sim checklist --md docs/COOP-CHECKLIST.md`. Do not edit by hand; edit the rows in `crates/coop-headless/checklist/*.json`.\n");
    let _ = writeln!(s, "**{}**\n", verdict_line(&total));
    let _ = writeln!(
        s,
        "Run: backend `{}`, seeds {}, net {}, party {}, {} plugins loaded ({}), {:.1} s wall.\n",
        r.opts.backend,
        r.opts
            .seeds
            .as_ref()
            .map_or("each scenario's own".to_string(), |v| format!(
                "{}..{}",
                v.first().copied().unwrap_or(0),
                v.last().copied().unwrap_or(0)
            )),
        r.opts.net.as_deref().unwrap_or("each scenario's own"),
        r.opts.peers,
        r.plugins.len(),
        r.plugins.join(", "),
        r.wall_s
    );
    s.push_str(HEADER);
    let _ = writeln!(s, "\n## Summary\n");
    let _ = writeln!(s, "| Area | Rows | PASS | MODELLED | FAIL | PENDING | SKIP |\n|---|---:|---:|---:|---:|---:|---:|");
    for (a, c) in areas.iter().filter(|(a, _)| !a.starts_with("Quests: ")) {
        let _ = writeln!(
            s,
            "| {a} | {} | {} | {} | {} | {} | {} |",
            c.rows, c.pass, c.modelled, c.fail, c.pending, c.skip
        );
    }
    let qs: Counts = {
        let mut c = Counts::default();
        for o in r.outcomes.iter().filter(|o| o.row.id.starts_with("quest.")) {
            c.add(o);
        }
        c
    };
    if qs.rows > 0 {
        let _ = writeln!(
            s,
            "| Quests (one row each, see per-quest tables) | {} | {} | {} | {} | {} | {} |",
            qs.rows, qs.pass, qs.modelled, qs.fail, qs.pending, qs.skip
        );
    }
    let _ = writeln!(
        s,
        "| **Total** | {} | {} | {} | {} | {} | {} |\n",
        total.rows, total.pass, total.modelled, total.fail, total.pending, total.skip
    );

    // failures with replay
    let fails: Vec<&Outcome> = r
        .outcomes
        .iter()
        .filter(|o| matches!(o.status, Status::Fail(_)))
        .collect();
    if !fails.is_empty() {
        let _ = writeln!(s, "## Failing\n");
        for o in fails {
            if let Status::Fail(m) = &o.status {
                let _ = writeln!(s, "- `{}`: {}", o.row.id, esc(m));
                for rp in &o.replay {
                    let _ = writeln!(s, "  - replay: `{rp}`");
                }
            }
        }
        s.push('\n');
    }

    // real game
    let real: Vec<&Outcome> = r
        .outcomes
        .iter()
        .filter(|o| !o.row.real.is_empty())
        .collect();
    let _ = writeln!(s, "## Proven in the real game (tools/coop-run, test PC)\n");
    if real.is_empty() {
        let _ = writeln!(s, "No row names a real-game scenario.\n");
    } else {
        let _ = writeln!(s, "| Row | Real scenarios | Last verdict |\n|---|---|---|");
        for o in real {
            let _ = writeln!(
                s,
                "| `{}` | {} | {} |",
                o.row.id,
                o.row.real.join(", "),
                o.real_game
            );
        }
        s.push('\n');
    }

    // tables by area
    for (a, _) in areas.iter().filter(|(a, _)| !a.starts_with("Quests: ")) {
        let _ = writeln!(s, "## {a}\n");
        let _ = writeln!(s, "| Id | Title | Status | Fidelity | Twin | Needs | Real game | Owner | Detail |\n|---|---|---|---|---|---|---|---|---|");
        for o in r.outcomes.iter().filter(|o| &o.row.area == a) {
            let st = match &o.status {
                Status::Pass if o.row.modelled() => "MODELLED".to_string(),
                Status::Pass => "PASS".to_string(),
                Status::Fail(_) => "FAIL".to_string(),
                Status::Pending(m) => format!("PENDING ({})", esc(m)),
                Status::Skip(m) => format!("SKIP ({})", esc(m)),
            };
            let fid = if o.row.modelled() {
                format!("modelled: {}", esc(&o.row.stand_in))
            } else {
                "headless-real-logic".into()
            };
            let detail = match &o.status {
                Status::Fail(m) => esc(m),
                _ => esc(&o.detail),
            };
            let _ = writeln!(
                s,
                "| `{}` | {} | {} | {} | {} | {} | {} | {} | {} |",
                o.row.id,
                esc(&o.row.title),
                st,
                fid,
                if o.row.twin { "yes" } else { "no" },
                o.row.needs,
                o.real_game,
                o.row.owner,
                clip(&detail, 260)
            );
        }
        s.push('\n');
        let proves: Vec<&Outcome> = r
            .outcomes
            .iter()
            .filter(|o| &o.row.area == a && !o.row.proves.is_empty())
            .collect();
        if !proves.is_empty() {
            let _ = writeln!(s, "<details><summary>What each row proves</summary>\n");
            for o in proves {
                let _ = writeln!(
                    s,
                    "- `{}`: {} [tests: {}]",
                    o.row.id,
                    esc(&o.row.proves),
                    o.row.tests.join(", ")
                );
            }
            let _ = writeln!(s, "\n</details>\n");
        }
    }

    // quests
    let groups = quest_groups(r);
    if !groups.is_empty() {
        let _ = writeln!(s, "## Per-quest co-op walk\n");
        let _ = writeln!(s, "Every quest of `crates/coop-policy/data/policy.json` walked on a host and guests (profiles {}, parties {:?}). {} bundled rows are not in the loaded plugins and run policy-only.\n", r.quests.first().map_or(String::new(), |_| walk_nets(r)), walk_peers(r), r.quests.iter().filter(|q| q.depth == crate::questwalk::Depth::PolicyOnly).count());
        let _ = writeln!(
            s,
            "| Group | Quests | Green | Policy-only | With SP-GAP |\n|---|---:|---:|---:|---:|"
        );
        for (g, n, green, po, sp) in &groups {
            let _ = writeln!(
                s,
                "| {} | {n} | {green} | {po} | {sp} |",
                g.trim_start_matches("Quests: ")
            );
        }
        s.push('\n');
        for (g, ..) in &groups {
            let _ = writeln!(s, "### {g}\n");
            let _ = writeln!(
                s,
                "| Quest | Questline | Status | Walk | SP-GAP functions |\n|---|---|---|---|---|"
            );
            for o in r.outcomes.iter().filter(|o| &o.row.area == g) {
                let st = if o.green() {
                    if o.row.modelled() {
                        "policy-only"
                    } else {
                        "PASS"
                    }
                } else {
                    "FAIL"
                };
                let gaps = o.detail.split("SP-GAP: ").nth(1).unwrap_or("");
                let walk = o.detail.split("; SP-GAP").next().unwrap_or("");
                let _ = writeln!(
                    s,
                    "| `{}` | {} | {} | {} | {} |",
                    o.row.id.trim_start_matches("quest."),
                    esc(o
                        .row
                        .title
                        .split(" (")
                        .nth(1)
                        .unwrap_or("")
                        .trim_end_matches(')')),
                    st,
                    esc(walk),
                    esc(gaps)
                );
            }
            s.push('\n');
        }
        let (spq, spf) = sp_summary(r);
        let _ = writeln!(
            s,
            "## SP-GAP: single-player script functions without an implementation\n"
        );
        let _ = writeln!(s, "{spq} quests ran a result script that called one of these. They are gaps of the single-player interpreter (`world::scripting`), not co-op failures: the call returns nothing on the host and the guest sees whatever the host ended with.\n");
        let _ = writeln!(s, "| Function | Quests | First seen |\n|---|---:|---|");
        let mut v: Vec<_> = spf.into_iter().collect();
        v.sort_by(|a, b| b.1 .0.cmp(&a.1 .0).then(a.0.cmp(&b.0)));
        for (f, (n, first)) in v {
            let _ = writeln!(s, "| `{f}` | {n} | {} |", esc(&clip(&first, 140)));
        }
        s.push('\n');
    }
    let _ = writeln!(s, "## Pending rows (no test yet)\n");
    for o in r
        .outcomes
        .iter()
        .filter(|o| matches!(o.status, Status::Pending(_)))
    {
        if let Status::Pending(m) = &o.status {
            let _ = writeln!(s, "- `{}` {}: {}", o.row.id, esc(&o.row.title), esc(m));
        }
    }
    s
}

fn walk_nets(r: &Report) -> String {
    if r.opts.full {
        "all seven".into()
    } else {
        r.opts
            .net
            .clone()
            .unwrap_or_else(|| "perfect, bad_wifi".into())
    }
}

fn walk_peers(r: &Report) -> Vec<u8> {
    if r.opts.full {
        vec![2, 3, 4]
    } else {
        vec![r.opts.peers]
    }
}

const HEADER: &str = r#"## How to read this

1. Read the verdict line, then the Summary table: one line per area.
2. Failing rows come next, each with the command that replays it.
3. "Proven in the real game" lists the rows that also have a `tools/coop-run` scenario, with the date of the last verdict found in `--real-out`.
4. Each area table has one row per feature: status, whether it passes with real logic or a named stand-in (MODELLED), whether a mutation twin guards it, the data it needs, and a detail.
5. Per-quest tables list every quest of the campaign policy by DLC and questline.

Statuses: PASS (all tests and the twin green), MODELLED (green, but a named stand-in replaces part of the real game), FAIL, PENDING (no test yet, or a scenario or twin the row names does not exist yet), SKIP (the data the row needs is not on this machine).

## What "all green" proves, and what it does not

All green means: every co-op rule we have written down has a headless test that passes, each guarded by a mutation twin where the row asks for one, and every quest of the policy table can be walked stage by stage on a host and guests with the real script runner without a co-op invariant breaking.

It does not prove:

- **Real-game behaviour.** This is headless logic: no renderer, no engine, no physics, no audio. Only the rows listed under "Proven in the real game" were seen in two real processes. Everything else is "in theory".
- **Single-player completeness.** A quest that passes can still call script functions the single-player layer does not implement (SP-GAP). Co-op cannot be more complete than the game under it.
- **Steam.** The Steam transport runs against a mock; the two-account items need two real accounts. UDP is exercised over a simulated link.
- **Modelled rows.** A MODELLED row passes with a stand-in for the viewer's per-frame glue (menus, input, rendering); the stand-in is named in the row.
- **The translated engine.** Backend conformance runs against the hand-written world layer. Run `coop-sim checklist --backend engine` to see the rows the engine still owes (all PENDING).

## How to add a row

1. Add an object to the right file in `crates/coop-headless/checklist/` (`id`, `area`, `title`, `proves`, `tests`, `needs`, `twin`, `fidelity`, `owner`, optional `real`).
2. A row with no `tests` is PENDING. A test named but not written is PENDING. Both are visible, neither is green.
3. **A feature is not done until its row is green with a twin.** Write the scenario (`crates/coop-headless/scenarios/NAME.json`) and its mutation twin (`twin_NAME.json`), or a `builtin:` check, or a backend `conformance:` test, then set `"twin": true`.
4. Run `coop-sim checklist --area YOURAREA`, then regenerate this file with `--md docs/COOP-CHECKLIST.md` and commit both.
"#;

/// JSON for tools.
pub fn json(r: &Report) -> String {
    let (_, total) = by_area(r);
    let rows: Vec<serde_json::Value> = r
        .outcomes
        .iter()
        .map(|o| {
            serde_json::json!({
                "id": o.row.id, "area": o.row.area, "title": o.row.title,
                "status": o.status.label(), "modelled": o.row.modelled(),
                "reason": match &o.status { Status::Fail(m) | Status::Pending(m) | Status::Skip(m) => m.clone(), _ => String::new() },
                "twin": o.row.twin, "needs": o.row.needs, "owner": o.row.owner,
                "real_game": o.real_game, "detail": o.detail, "replay": o.replay, "sp_gaps": o.sp_gaps,
            })
        })
        .collect();
    let v = serde_json::json!({
        "verdict": verdict_line(&total),
        "rows": total.rows, "green": total.green(), "pending": total.pending, "failing": total.fail, "skipped": total.skip,
        "wall_s": r.wall_s, "items": rows,
    });
    serde_json::to_string_pretty(&v).unwrap_or_default()
}

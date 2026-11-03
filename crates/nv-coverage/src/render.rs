//! The Markdown summary.

use std::fmt::Write;

use crate::checklist::{file_state, ITEMS};
use crate::quests::Verdict;
use crate::report::{pct, Full};

pub fn markdown(f: &Full) -> String {
    let mut o = String::new();
    let fr = &f.funcs;
    let used = fr.used_count();
    let uh = fr.used_handled();
    let _ = writeln!(o, "# nv-rs single-player coverage\n");
    let _ = writeln!(
        o,
        "Measured by `nv-coverage` on the plugins: {}. Meshes and archives present: {}. \
         Everything below is counted from the game's own records; \"handled\" means the world host carries the function out \
         (`world::scripting::HANDLED`, `world::script_functions::handled()`), not that the result matches the original game.\n",
        f.plugins.join(", "),
        if f.has_meshes { "yes (full data)" } else { "no (ESMs only; the load sweep ran in records mode)" }
    );
    let _ = writeln!(o, "## Headline\n");
    let _ = writeln!(o, "| Measure | Value |\n|---|---|");
    let _ = writeln!(
        o,
        "| Script functions in the exe table (named) | {} ({} carried out) |",
        fr.named, fr.named_handled
    );
    let _ = writeln!(
        o,
        "| Functions the game's scripts and conditions use | {used} |"
    );
    let _ = writeln!(o, "| Used and carried out | {uh} ({}%) |", pct(uh, used));
    let _ = writeln!(o, "| Used and missing | {} |", used - uh);
    let _ = writeln!(
        o,
        "| Missing, weighted by uses | {} of {} uses ({}% covered) |",
        fr.weight(false) - fr.weight(true),
        fr.weight(false),
        pct(fr.weight(true), fr.weight(false))
    );
    for (l, r) in &f.lines {
        let _ = writeln!(
            o,
            "| Quest stages runnable: {} | {} of {} ({}%) |",
            l.label(),
            r.clean(),
            r.stages.len(),
            pct(r.clean(), r.stages.len())
        );
    }
    let d = &f.dialogue;
    let _ = writeln!(
        o,
        "| INFO conditions with all functions supported | {}% of {} INFOs with conditions |",
        pct(
            d.infos_with_conditions - d.infos_unsupported_condition,
            d.infos_with_conditions
        ),
        d.infos_with_conditions
    );
    let _ = writeln!(
        o,
        "| Dialogue result scripts running clean | {} of {} ({}%) |",
        d.results_ran_clean,
        d.results,
        pct(d.results_ran_clean, d.results)
    );
    if let Some(s) = &f.sweep {
        let ext: usize = s.exteriors.values().map(|c| c.tried).sum();
        let extok: usize = s.exteriors.values().map(|c| c.loaded).sum();
        let _ = writeln!(
            o,
            "| Load sweep ({}) interiors | {} of {} loaded |",
            if s.full { "full" } else { "records" },
            s.interiors.loaded,
            s.interiors.tried
        );
        let _ = writeln!(
            o,
            "| Load sweep exterior squares | {extok} of {ext} loaded |"
        );
    }

    let _ = writeln!(o, "\n## 1. Script function coverage\n");
    let _ = writeln!(
        o,
        "### Sources parsed\n\n| Kind | Scripts | Parsed |\n|---|---|---|"
    );
    for (k, t) in &fr.by_kind {
        let _ = writeln!(o, "| {} | {} | {} |", k.label(), t.sources, t.parsed);
    }
    if !fr.parse_errors.is_empty() {
        let _ = writeln!(
            o,
            "\n### Parse failures\n\n| Message | Count | Examples |\n|---|---|---|"
        );
        let mut v: Vec<_> = fr.parse_errors.iter().collect();
        v.sort_by_key(|(_, (n, _))| std::cmp::Reverse(*n));
        for (m, (n, ex)) in v.into_iter().take(25) {
            let _ = writeln!(o, "| {m} | {n} | {} |", ex.join("; "));
        }
    }
    let _ = writeln!(
        o,
        "\n### By system\n\n| System | Used | Carried out | Uses |\n|---|---|---|---|"
    );
    let mut groups: std::collections::BTreeMap<&str, (usize, usize, usize)> = Default::default();
    for s in fr.used() {
        let g = groups.entry(s.group).or_default();
        g.0 += 1;
        g.1 += usize::from(s.handled);
        g.2 += s.calls + s.conditions;
    }
    for (g, (u, h, w)) in &groups {
        let _ = writeln!(o, "| {g} | {u} | {h} ({}%) | {w} |", pct(*h, *u));
    }
    let _ = writeln!(o, "\n### Top missing functions\n\nRanked by main-line quests that use them, then by uses. Quests = base main, VMS and DLC main quests whose scripts or dialogue results call the function.\n");
    let _ = writeln!(o, "| # | Function | System | Calls | In conditions | Scripts | Main-line quests | Example quests |\n|---|---|---|---|---|---|---|---|");
    for (i, s) in fr.missing().into_iter().take(60).enumerate() {
        let ex: Vec<&str> = s.quests.iter().take(4).map(String::as_str).collect();
        let _ = writeln!(
            o,
            "| {} | {} | {} | {} | {} | {} | {} | {} |",
            i + 1,
            s.name,
            s.group,
            s.calls,
            s.conditions,
            s.scripts,
            s.quests.len(),
            ex.join(", ")
        );
    }
    let _ = writeln!(
        o,
        "\n### Events (script block kinds)\n\n| Block | Scripts | Runtime reacts |\n|---|---|---|"
    );
    for (k, n, h) in &f.events {
        let _ = writeln!(
            o,
            "| {k} | {n} | {} |",
            h.map_or("unknown (no repo)", |h| if h {
                "yes (name found in world/viewer source)"
            } else {
                "no"
            })
        );
    }

    let _ = writeln!(o, "\n## 2. Quest walk\n\nEach stage is set in order with `world::scripting::Runner::set_stage`, then one second of game time runs so quest scripts execute. Verdicts: clean, parse error, unhandled function (the host returned no value at run time), missing function (static: on a path the stub world did not take), panic, refused.\n");
    for (l, r) in &f.lines {
        let _ = writeln!(o, "### {}\n\n{} quests, {} stages, {} runnable ({}%). Objectives {}, targets {}, unresolved targets {}.\n", l.label(), r.quests, r.stages.len(), r.clean(), pct(r.clean(), r.stages.len()), r.objectives, r.targets, r.gaps.len());
        let mut tags: std::collections::BTreeMap<&str, usize> = Default::default();
        for s in &r.stages {
            *tags.entry(s.verdict.tag()).or_default() += 1;
        }
        let _ = writeln!(
            o,
            "Verdicts: {}\n",
            tags.iter()
                .map(|(k, n)| format!("{k} {n}"))
                .collect::<Vec<_>>()
                .join(", ")
        );
        let bad: Vec<_> = r.stages.iter().filter(|s| !s.verdict.is_clean()).collect();
        if !bad.is_empty() {
            let _ = writeln!(o, "| Quest | Stage | Verdict | Detail |\n|---|---|---|---|");
            for s in bad.iter().take(80) {
                let d = match &s.verdict {
                    Verdict::ParseError(e) | Verdict::Panic(e) => e.clone(),
                    Verdict::Unhandled(n) | Verdict::StubLimit(n) | Verdict::StaticMissing(n) => {
                        n.join(", ")
                    }
                    _ => String::new(),
                };
                let _ = writeln!(
                    o,
                    "| {} | {} | {} | {d} |",
                    s.editor,
                    s.stage,
                    s.verdict.tag()
                );
            }
            if bad.len() > 80 {
                let _ = writeln!(o, "\n({} more in the JSON.)", bad.len() - 80);
            }
        }
        for g in r.gaps.iter().take(15) {
            let _ = writeln!(
                o,
                "- target {} of {} objective {}: {}",
                g.target, g.editor, g.objective, g.why
            );
        }
        let _ = writeln!(o);
    }

    let g = &f.graph;
    let _ = writeln!(
        o,
        "### Main-line quests that depend on a missing function\n"
    );
    let _ = writeln!(o, "{} quests (scripts, dialogue results or stage results). {} main-line stages are never set by any script in the data (only the player's route, an engine hook or a console sets them).\n", g.deps.len(), g.never_set.len());
    for (q, fs) in &g.deps {
        let _ = writeln!(
            o,
            "- {q}: {}",
            fs.iter().cloned().collect::<Vec<_>>().join(", ")
        );
    }
    if !g.never_set.is_empty() {
        let _ = writeln!(
            o,
            "\nStages with no setter: {}\n",
            g.never_set
                .iter()
                .take(60)
                .map(|(q, s)| format!("{q}:{s}"))
                .collect::<Vec<_>>()
                .join(", ")
        );
    }
    let ob = &f.objects;
    let _ = writeln!(o, "### Object scripts against the stub world\n");
    let _ = writeln!(o, "{} placed references carry a script ({} distinct scripts). Each ran once with every event block: {} clean, {} hit a function nv-rs does not carry out, {} panicked.\n", ob.references_with_script, ob.distinct_scripts, ob.ran_clean, ob.scripts_with_unhandled, ob.panicked);
    for (title, map) in [
        ("Not carried out", &ob.unhandled),
        (
            "Handled but the stub world could not answer",
            &ob.stub_limited,
        ),
    ] {
        if map.is_empty() {
            continue;
        }
        let mut v: Vec<_> = map.iter().collect();
        v.sort_by_key(|(_, n)| std::cmp::Reverse(**n));
        let _ = writeln!(
            o,
            "{title}: {}\n",
            v.into_iter()
                .take(25)
                .map(|(k, n)| format!("{k} {n}"))
                .collect::<Vec<_>>()
                .join(", ")
        );
    }
    if !ob.overflows.is_empty() {
        let _ = writeln!(o, "{} scripts killed the worker process (stack overflow or abort in the host; each is a host bug to fix, most likely `Activate` re-entering `OnActivate`): {}
", ob.overflows.len(), ob.overflows.iter().map(|(n, r)| format!("{n} on {r}")).collect::<Vec<_>>().join(", "));
    }
    for p in &ob.panics {
        let _ = writeln!(o, "- panic: {p}");
    }
    let _ = writeln!(o, "\n**What the walk does not measure.** It runs scripts against a stub world: no loaded 3D, no live AI, no timers over real time, no player input, no presentation. A runnable stage means its result script runs; it does not mean the objective can be reached, the NPC pathfinds, the trigger fires, or the menu draws. Those gaps are what the system checklist and the full-data load sweep point at, and what a bot playthrough (docs/PLAYTESTING.md) must close.\n");
    let _ = writeln!(o, "## 3. Cell and world load sweep\n");
    match &f.sweep {
        None => {
            let _ = writeln!(o, "Not run.\n");
        }
        Some(s) => {
            if let Some(why) = &s.skipped {
                let _ = writeln!(o, "Skipped: {why}\n");
            }
            let _ = writeln!(
                o,
                "Mode: {}. Interiors: {} tried, {} loaded, {} failed.\n",
                if s.full {
                    "full (viewer loader, models/textures/collision/terrain)"
                } else {
                    "records (ESM-only loader; run on the full Data for asset checks)"
                },
                s.interiors.tried,
                s.interiors.loaded,
                s.interiors.failed
            );
            let _ = writeln!(
                o,
                "| Worldspace | Squares tried | Loaded | Failed |\n|---|---|---|---|"
            );
            for (w, c) in &s.exteriors {
                let _ = writeln!(o, "| {w} | {} | {} | {} |", c.tried, c.loaded, c.failed);
            }
            if !s.failures.is_empty() {
                let _ = writeln!(o, "\n| Problem | Cells | Examples |\n|---|---|---|");
                let mut v: Vec<_> = s.failures.iter().collect();
                v.sort_by_key(|(_, (n, _))| std::cmp::Reverse(*n));
                for (k, (n, ex)) in v.into_iter().take(30) {
                    let _ = writeln!(o, "| {k} | {n} | {} |", ex.join("; "));
                }
            }
            for (k, m) in &s.assets {
                let _ = writeln!(o, "\n{k}: {} distinct. Top: {}", m.len(), {
                    let mut v: Vec<_> = m.iter().collect();
                    v.sort_by_key(|(_, n)| std::cmp::Reverse(**n));
                    v.into_iter()
                        .take(8)
                        .map(|(p, n)| format!("{p} ({n})"))
                        .collect::<Vec<_>>()
                        .join(", ")
                });
            }
            if !s.full {
                let _ = writeln!(o, "\nDistinct model paths named by the loaded cells: {} (checked for existence in full mode).", s.model_paths.len());
            }
        }
    }

    let _ = writeln!(o, "\n## 4. Dialogue coverage\n");
    let _ = writeln!(o, "{} INFOs; {} with conditions ({} conditions); {} use a condition function nv-rs does not carry out. Result scripts: {} total, {} do not parse, {} call a missing function, {} ran clean, {} panicked.\n", d.infos, d.infos_with_conditions, d.conditions, d.infos_unsupported_condition, d.results, d.results_parse_failed, d.results_missing_function, d.results_ran_clean, d.results_panicked);
    for (title, map) in [
        (
            "Unsupported condition functions (INFOs)",
            &d.unsupported_conditions,
        ),
        (
            "Unsupported result functions (scripts)",
            &d.unsupported_results,
        ),
    ] {
        if map.is_empty() {
            continue;
        }
        let mut v: Vec<_> = map.iter().collect();
        v.sort_by_key(|(_, n)| std::cmp::Reverse(**n));
        let _ = writeln!(
            o,
            "{title}: {}\n",
            v.into_iter()
                .take(25)
                .map(|(k, n)| format!("{k} {n}"))
                .collect::<Vec<_>>()
                .join(", ")
        );
    }

    let pc = &f.perks;
    let _ = writeln!(o, "### Perks: entry points\n");
    let _ = writeln!(o, "{} perks with {} entry points; {} of those are on entry points the world crate applies (a constant in `perks.rs` that is used elsewhere).\n", pc.perks, pc.entry_points, pc.applied_entry_points());
    let mut rows: Vec<_> = pc.by_entry.iter().filter(|(_, e)| !e.2).collect();
    rows.sort_by_key(|(_, e)| std::cmp::Reverse(e.1));
    let _ = writeln!(
        o,
        "Not applied (most perks first): {}\n",
        rows.iter()
            .take(25)
            .map(|(n, e)| format!("{} #{n} ({})", e.0, e.1))
            .collect::<Vec<_>>()
            .join(", ")
    );
    let _ = writeln!(o, "## 5. System checklist\n\nStatus is a guess from which files exist; none of it is compared with the original game.\n");
    let _ = writeln!(o, "| System | Status | Files | Note |\n|---|---|---|---|");
    for i in ITEMS {
        let files: Vec<String> = file_state(f.repo.as_deref(), i)
            .into_iter()
            .map(|(p, e)| match e {
                Some(false) => format!("{p} (MISSING)"),
                _ => p.to_string(),
            })
            .collect();
        let _ = writeln!(
            o,
            "| {} | {} | {} | {} |",
            i.system,
            i.status.label(),
            files.join(", "),
            i.note
        );
    }

    let _ = writeln!(
        o,
        "\n## 6. Fix list\n\nOrdered by quest stages blocked, then main-line quests, then uses.\n"
    );
    let _ = writeln!(o, "| # | Item | Stages blocked | Quests | Uses | Size | Source | Already translated from the exe (name, address) |\n|---|---|---|---|---|---|---|---|");
    for x in f.fixes.iter().take(40) {
        let tr: Vec<String> = x
            .translated
            .iter()
            .map(|t| t.split(' ').take(2).collect::<Vec<_>>().join(" "))
            .collect();
        let _ = writeln!(
            o,
            "| {} | {} | {} | {} | {} | {} | {} | {} |",
            x.rank,
            x.title,
            x.stages_blocked,
            x.quests,
            x.uses,
            x.size,
            x.source,
            tr.join(", ")
        );
    }
    o
}

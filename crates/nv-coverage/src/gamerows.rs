//! Measurement export for the GAME checklist layer: `nv-coverage gamefacts`.
//!
//! The facts are what the row author (`tools/gamerows/make_rows.py`) turns
//! into manifest rows: every quest with who sets each stage and how a stage
//! run ends, the worldspaces, interior cells and map markers, regions,
//! perks with their entry points, record counts, and the coverage report.

use std::collections::BTreeSet;
use std::path::Path;

use esm::{FourCC, LoadOrder};

use crate::gamelayer;
use crate::json::J;
use crate::quests::Verdict;
use crate::scan;

/// One line per quest: plugin, editor id, name, stages, flags, script, objectives.
pub fn quest_tsv(order: &LoadOrder) -> String {
    let mut s = String::new();
    for q in gamelayer::quests(order) {
        s.push_str(&format!(
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
            q.plugin,
            q.edid,
            q.name,
            q.stages.len(),
            q.flags,
            q.script.is_some(),
            q.objectives
        ));
    }
    s
}

fn verdict_json(v: &Verdict) -> (J, J) {
    let funcs: Vec<String> = match v {
        Verdict::Unhandled(n) | Verdict::StubLimit(n) | Verdict::StaticMissing(n) => n.clone(),
        Verdict::ParseError(e) | Verdict::Panic(e) => vec![e.clone()],
        _ => Vec::new(),
    };
    (J::s(v.tag()), J::arr(funcs.into_iter().map(J::s)))
}

/// Everything the row author needs, as one JSON value.
pub fn facts(dir: &Path) -> Result<J, String> {
    let d = crate::data::open(dir)?;
    let order = &d.order;
    let list = gamelayer::quests(order);
    let sources = scan::sources(order);
    let ev = gamelayer::evidence(order, &sources, &list);
    let never: BTreeSet<(String, u16)> = gamelayer::never_set(&list, &ev)
        .into_iter()
        .map(|(q, s)| (q.to_ascii_lowercase(), s))
        .collect();
    let runs = gamelayer::walk(order, &list, None);

    let quests = list.iter().map(|q| {
        let key = q.edid.to_ascii_lowercase();
        let stage_rows = runs
            .get(&key)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
            .iter()
            .map(|r| {
                let (tag, funcs) = verdict_json(&r.verdict);
                let setters: Vec<J> = ev
                    .setters
                    .get(&(key.clone(), r.stage))
                    .into_iter()
                    .flatten()
                    .map(|k| J::s(k.label()))
                    .collect();
                J::obj(vec![
                    ("stage", J::u(r.stage as usize)),
                    ("verdict", tag),
                    ("detail", funcs),
                    ("has_script", J::Bool(r.has_script)),
                    (
                        "never_set",
                        J::Bool(never.contains(&(key.clone(), r.stage))),
                    ),
                    ("setters", J::Arr(setters)),
                ])
            });
        J::obj(vec![
            ("edid", J::s(&q.edid)),
            ("name", J::s(&q.name)),
            ("plugin", J::s(&q.plugin)),
            ("flags", J::u(q.flags as usize)),
            ("priority", J::u(q.priority as usize)),
            ("objectives", J::u(q.objectives)),
            ("own_script", J::Bool(q.script.is_some())),
            (
                "dynamic_setters",
                J::u(ev.dynamic.get(&key).copied().unwrap_or(0)),
            ),
            (
                "mentions",
                J::arr(
                    ev.mentions
                        .get(&key)
                        .into_iter()
                        .flatten()
                        .map(|(k, o)| J::arr([J::s(k.label()), J::s(o)])),
                ),
            ),
            ("stages", J::arr(stage_rows)),
        ])
    });

    // Worldspaces.
    let squares = crate::sweep::squares(order);
    let worlds = order.records_of_type(FourCC::new(b"WRLD")).map(|rr| {
        let r = rr.record().ok();
        J::obj(vec![
            (
                "edid",
                J::s(r.as_ref().and_then(|r| r.editor_id()).unwrap_or_default()),
            ),
            (
                "name",
                J::s(r.as_ref().and_then(|r| r.full_name()).unwrap_or_default()),
            ),
            ("plugin", J::s(&rr.plugin.name)),
            (
                "squares",
                J::u(squares.get(&rr.form_id).map_or(0, |v| v.len())),
            ),
        ])
    });

    // Interior cells, with the place key the runtime groups them by.
    let interiors = world::interior_cells(order).unwrap_or_default();
    let interior_rows = interiors.iter().map(|c| {
        let edid = c.editor_id.clone().unwrap_or_default();
        J::obj(vec![
            ("edid", J::s(&edid)),
            ("name", J::s(c.name.clone().unwrap_or_default())),
            ("plugin", J::s(&c.plugin)),
            ("refs", J::u(c.references)),
            ("place", J::s(gamelayer::place_of(&edid))),
        ])
    });

    // Map markers (fast travel), by worldspace.
    let mut seen = BTreeSet::new();
    let mut markers = Vec::new();
    for rr in order.records_of_type(FourCC::new(b"CELL")) {
        if rr.entry.header.is_deleted() {
            continue;
        }
        let w = order.world_of(&rr);
        let wname = w
            .and_then(|w| order.get(w))
            .and_then(|r| r.editor_id().ok().flatten())
            .unwrap_or_default();
        for m in world::map::markers(order, rr.form_id) {
            if !seen.insert(m.reference) {
                continue;
            }
            markers.push(J::obj(vec![
                ("name", J::s(&m.name)),
                ("world", J::s(&wname)),
                ("x", J::f(m.position[0] as f64)),
                ("y", J::f(m.position[1] as f64)),
                ("visible", J::Bool(m.flags & world::map::VISIBLE != 0)),
                ("kind", J::u(m.kind as usize)),
                ("flags", J::u(m.flags as usize)),
                ("arrival", J::Bool(m.arrival.is_some())),
            ]));
        }
    }

    // Regions: the data types each names.
    let regions = order
        .records_of_type(FourCC::new(b"REGN"))
        .filter_map(|rr| {
            let r = rr.record().ok()?;
            let mut types = BTreeSet::new();
            for s in r.get_all(FourCC::new(b"RDAT")) {
                if let Some(b) = s.data.get(..4) {
                    types.insert(u32::from_le_bytes([b[0], b[1], b[2], b[3]]) as usize);
                }
            }
            let world_name = r
                .get(FourCC::new(b"WNAM"))
                .and_then(|s| {
                    s.data.get(..4).map(|b| {
                        rr.plugin
                            .to_global(esm::FormId(u32::from_le_bytes([b[0], b[1], b[2], b[3]])))
                    })
                })
                .and_then(|w| order.get(w))
                .and_then(|w| w.editor_id().ok().flatten())
                .unwrap_or_default();
            Some(J::obj(vec![
                ("edid", J::s(r.editor_id().unwrap_or_default())),
                ("world", J::s(world_name)),
                ("plugin", J::s(&rr.plugin.name)),
                ("data_types", J::arr(types.into_iter().map(J::u))),
            ]))
        });

    // Perks with their entry points.
    let perks = order.records_of_type(FourCC::new(b"PERK")).map(|rr| {
        let r = rr.record().ok();
        let eps = world::perks::entry_points(order, rr.form_id);
        J::obj(vec![
            (
                "edid",
                J::s(r.as_ref().and_then(|r| r.editor_id()).unwrap_or_default()),
            ),
            (
                "name",
                J::s(r.as_ref().and_then(|r| r.full_name()).unwrap_or_default()),
            ),
            ("plugin", J::s(&rr.plugin.name)),
            (
                "entries",
                J::arr(eps.iter().map(|e| J::u(e.entry as usize))),
            ),
        ])
    });

    let counts = order
        .type_counts()
        .into_iter()
        .map(|(k, n)| (k.to_string(), J::u(n)))
        .collect::<Vec<_>>();

    // The coverage report without the sweep (the runtime does the sweep).
    let mut opts = crate::Options::everything();
    opts.objects = false;
    opts.sweep = None;
    let full = crate::build(dir, &opts)?;
    let coverage = crate::report::to_json(&full);

    Ok(J::obj(vec![
        (
            "plugins",
            J::arr(d.order.plugins().iter().map(|p| J::s(&p.name))),
        ),
        ("quests", J::arr(quests)),
        ("worlds", J::arr(worlds)),
        ("interiors", J::arr(interior_rows)),
        ("markers", J::Arr(markers)),
        ("regions", J::arr(regions)),
        ("perks", J::arr(perks)),
        ("record_counts", J::Obj(counts)),
        ("coverage", coverage),
    ]))
}

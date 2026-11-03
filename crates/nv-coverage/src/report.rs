//! The whole report and its JSON form.

use std::collections::BTreeMap;
use std::path::PathBuf;

use crate::data::Line;
use crate::dialogue::DialogueReport;
use crate::fixlist::Fix;
use crate::funcs::FuncReport;
use crate::json::J;
use crate::quests::{LineReport, Verdict};
use crate::sweep::SweepReport;

pub struct Full {
    pub data_dir: PathBuf,
    pub plugins: Vec<String>,
    pub has_meshes: bool,
    pub repo: Option<PathBuf>,
    pub funcs: FuncReport,
    pub lines: BTreeMap<Line, LineReport>,
    pub dialogue: DialogueReport,
    pub objects: crate::objects::ObjectReport,
    pub graph: crate::graph::QuestGraph,
    pub perks: crate::content::PerkCensus,
    pub sweep: Option<SweepReport>,
    pub fixes: Vec<Fix>,
    /// Block kinds with whether the runtime reacts to them (needs the repository).
    pub events: Vec<(String, usize, Option<bool>)>,
}

pub fn pct(part: usize, whole: usize) -> f64 {
    if whole == 0 {
        100.0
    } else {
        (1000.0 * part as f64 / whole as f64).round() / 10.0
    }
}

pub fn to_json(f: &Full) -> J {
    let fr = &f.funcs;
    let used = fr.used_count();
    let used_handled = fr.used_handled();
    let function_rows = fr.used().map(|s| {
        J::obj(vec![
            ("name", J::s(&s.name)),
            ("index", J::u(s.index as usize)),
            ("group", J::s(s.group)),
            ("handled", J::Bool(s.handled)),
            ("calls", J::u(s.calls)),
            ("scripts", J::u(s.scripts)),
            ("conditions", J::u(s.conditions)),
            (
                "by_kind",
                J::Obj(
                    s.by_kind
                        .iter()
                        .map(|(k, n)| (k.label().to_string(), J::u(*n)))
                        .collect(),
                ),
            ),
            ("main_line_quests", J::arr(s.quests.iter().map(J::s))),
        ])
    });
    let mut groups: BTreeMap<&str, (usize, usize, usize)> = BTreeMap::new();
    for s in fr.used() {
        let g = groups.entry(s.group).or_default();
        g.0 += 1;
        if s.handled {
            g.1 += 1;
        }
        g.2 += s.calls + s.conditions;
    }
    let lines = f.lines.iter().map(|(l, r)| {
        let stages = r.stages.iter().filter(|s| !s.verdict.is_clean()).map(|s| {
            let detail = match &s.verdict {
                Verdict::ParseError(e) | Verdict::Panic(e) => vec![e.clone()],
                Verdict::Unhandled(n) | Verdict::StubLimit(n) | Verdict::StaticMissing(n) => {
                    n.clone()
                }
                _ => Vec::new(),
            };
            J::obj(vec![
                ("quest", J::s(&s.editor)),
                ("stage", J::u(s.stage as usize)),
                ("has_script", J::Bool(s.has_script)),
                ("verdict", J::s(s.verdict.tag())),
                ("detail", J::arr(detail.into_iter().map(J::s))),
            ])
        });
        (
            l.label(),
            J::obj(vec![
                ("quests", J::u(r.quests)),
                ("stages", J::u(r.stages.len())),
                ("stages_runnable", J::u(r.clean())),
                ("stages_strictly_clean", J::u(r.strictly_clean())),
                ("percent_runnable", J::f(pct(r.clean(), r.stages.len()))),
                ("objectives", J::u(r.objectives)),
                ("targets", J::u(r.targets)),
                ("targets_unresolved", J::u(r.gaps.len())),
                ("problem_stages", J::arr(stages)),
                (
                    "unresolved_targets",
                    J::arr(r.gaps.iter().take(200).map(|g| {
                        J::obj(vec![
                            ("quest", J::s(&g.editor)),
                            ("objective", J::u(g.objective as usize)),
                            ("target", J::s(g.target.to_string())),
                            ("why", J::s(&g.why)),
                        ])
                    })),
                ),
            ]),
        )
    });
    let d = &f.dialogue;
    let counts = |c: &crate::sweep::Counts| {
        J::obj(vec![
            ("tried", J::u(c.tried)),
            ("loaded", J::u(c.loaded)),
            ("failed", J::u(c.failed)),
        ])
    };
    let sweep = match &f.sweep {
        None => J::Null,
        Some(s) => J::obj(vec![
            ("mode", J::s(if s.full { "full" } else { "records" })),
            ("skipped", s.skipped.clone().map_or(J::Null, J::s)),
            ("interiors", counts(&s.interiors)),
            (
                "exteriors",
                J::Obj(
                    s.exteriors
                        .iter()
                        .map(|(k, v)| (k.clone(), counts(v)))
                        .collect(),
                ),
            ),
            (
                "failures",
                J::Obj(
                    s.failures
                        .iter()
                        .map(|(k, (n, ex))| {
                            (
                                k.clone(),
                                J::obj(vec![
                                    ("cells", J::u(*n)),
                                    ("examples", J::arr(ex.iter().map(J::s))),
                                ]),
                            )
                        })
                        .collect(),
                ),
            ),
            (
                "assets",
                J::Obj(
                    s.assets
                        .iter()
                        .map(|(k, m)| {
                            let mut v: Vec<_> = m.iter().collect();
                            v.sort_by_key(|(_, n)| std::cmp::Reverse(**n));
                            (
                                k.clone(),
                                J::obj(vec![
                                    ("distinct", J::u(m.len())),
                                    (
                                        "top",
                                        J::arr(v.into_iter().take(50).map(|(p, n)| {
                                            J::obj(vec![("path", J::s(p)), ("cells", J::u(*n))])
                                        })),
                                    ),
                                ]),
                            )
                        })
                        .collect(),
                ),
            ),
            ("distinct_model_paths", J::u(s.model_paths.len())),
        ]),
    };
    J::obj(vec![
        (
            "data_dir_name",
            J::s(
                f.data_dir
                    .file_name()
                    .map_or(String::new(), |n| n.to_string_lossy().into_owned()),
            ),
        ),
        ("plugins", J::arr(f.plugins.iter().map(J::s))),
        ("has_meshes", J::Bool(f.has_meshes)),
        (
            "script_functions",
            J::obj(vec![
                ("table_size", J::u(fr.table)),
                ("named", J::u(fr.named)),
                ("named_handled", J::u(fr.named_handled)),
                ("used", J::u(used)),
                ("used_handled", J::u(used_handled)),
                ("used_missing", J::u(used - used_handled)),
                ("use_weight_total", J::u(fr.weight(false))),
                ("use_weight_handled", J::u(fr.weight(true))),
                (
                    "sources",
                    J::Obj(
                        fr.by_kind
                            .iter()
                            .map(|(k, t)| {
                                (
                                    k.label().to_string(),
                                    J::obj(vec![
                                        ("total", J::u(t.sources)),
                                        ("parsed", J::u(t.parsed)),
                                    ]),
                                )
                            })
                            .collect(),
                    ),
                ),
                (
                    "parse_errors",
                    J::Obj(
                        fr.parse_errors
                            .iter()
                            .map(|(m, (n, ex))| {
                                (
                                    m.clone(),
                                    J::obj(vec![
                                        ("count", J::u(*n)),
                                        ("examples", J::arr(ex.iter().map(J::s))),
                                    ]),
                                )
                            })
                            .collect(),
                    ),
                ),
                (
                    "groups",
                    J::Obj(
                        groups
                            .iter()
                            .map(|(g, (u, h, w))| {
                                (
                                    g.to_string(),
                                    J::obj(vec![
                                        ("used", J::u(*u)),
                                        ("handled", J::u(*h)),
                                        ("weight", J::u(*w)),
                                    ]),
                                )
                            })
                            .collect(),
                    ),
                ),
                (
                    "events",
                    J::arr(f.events.iter().map(|(k, n, h)| {
                        J::obj(vec![
                            ("event", J::s(k)),
                            ("scripts", J::u(*n)),
                            ("handled", h.map_or(J::Null, J::Bool)),
                        ])
                    })),
                ),
                ("functions", J::arr(function_rows)),
            ]),
        ),
        ("quest_walk", J::Obj(lines.collect())),
        (
            "dialogue",
            J::obj(vec![
                ("infos", J::u(d.infos)),
                ("infos_with_conditions", J::u(d.infos_with_conditions)),
                ("conditions", J::u(d.conditions)),
                (
                    "infos_unsupported_condition",
                    J::u(d.infos_unsupported_condition),
                ),
                ("condition_percent_supported", J::f(d.condition_percent())),
                (
                    "unsupported_conditions",
                    J::Obj(
                        d.unsupported_conditions
                            .iter()
                            .map(|(k, n)| (k.clone(), J::u(*n)))
                            .collect(),
                    ),
                ),
                ("results", J::u(d.results)),
                ("results_parse_failed", J::u(d.results_parse_failed)),
                ("results_missing_function", J::u(d.results_missing_function)),
                ("results_ran_clean", J::u(d.results_ran_clean)),
                ("results_panicked", J::u(d.results_panicked)),
                ("result_percent_clean", J::f(d.result_percent())),
                (
                    "unsupported_results",
                    J::Obj(
                        d.unsupported_results
                            .iter()
                            .map(|(k, n)| (k.clone(), J::u(*n)))
                            .collect(),
                    ),
                ),
            ]),
        ),
        (
            "object_scripts",
            J::obj(vec![
                (
                    "references_with_script",
                    J::u(f.objects.references_with_script),
                ),
                ("distinct_scripts", J::u(f.objects.distinct_scripts)),
                ("ran_clean", J::u(f.objects.ran_clean)),
                (
                    "with_unhandled_function",
                    J::u(f.objects.scripts_with_unhandled),
                ),
                ("panicked", J::u(f.objects.panicked)),
                ("panics", J::arr(f.objects.panics.iter().map(J::s))),
                (
                    "worker_crashes",
                    J::arr(
                        f.objects.overflows.iter().map(|(n, r)| {
                            J::obj(vec![("script", J::s(n)), ("reference", J::s(r))])
                        }),
                    ),
                ),
                (
                    "unhandled_functions",
                    J::Obj(
                        f.objects
                            .unhandled
                            .iter()
                            .map(|(k, n)| (k.clone(), J::u(*n)))
                            .collect(),
                    ),
                ),
                (
                    "stub_limited_functions",
                    J::Obj(
                        f.objects
                            .stub_limited
                            .iter()
                            .map(|(k, n)| (k.clone(), J::u(*n)))
                            .collect(),
                    ),
                ),
            ]),
        ),
        (
            "quest_graph",
            J::obj(vec![
                (
                    "main_line_quests_with_missing_functions",
                    J::Obj(
                        f.graph
                            .deps
                            .iter()
                            .map(|(q, fs)| (q.clone(), J::arr(fs.iter().map(J::s))))
                            .collect(),
                    ),
                ),
                (
                    "stages_never_set_by_data",
                    J::arr(f.graph.never_set.iter().map(|(q, s)| {
                        J::obj(vec![("quest", J::s(q)), ("stage", J::u(*s as usize))])
                    })),
                ),
            ]),
        ),
        (
            "perks",
            J::obj(vec![
                ("perks", J::u(f.perks.perks)),
                ("entry_points", J::u(f.perks.entry_points)),
                ("entry_points_applied", J::u(f.perks.applied_entry_points())),
                (
                    "by_entry_point",
                    J::arr(f.perks.by_entry.iter().map(|(n, (name, count, applied))| {
                        J::obj(vec![
                            ("entry", J::u(*n as usize)),
                            ("name", J::s(name)),
                            ("in_perks", J::u(*count)),
                            ("applied", J::Bool(*applied)),
                        ])
                    })),
                ),
            ]),
        ),
        ("load_sweep", sweep),
        (
            "system_checklist",
            J::arr(crate::checklist::ITEMS.iter().map(|i| {
                J::obj(vec![
                    ("system", J::s(i.system)),
                    ("status_guess", J::s(i.status.label())),
                    (
                        "files",
                        J::arr(
                            crate::checklist::file_state(f.repo.as_deref(), i)
                                .into_iter()
                                .map(|(p, e)| {
                                    J::obj(vec![
                                        ("path", J::s(p)),
                                        ("exists", e.map_or(J::Null, J::Bool)),
                                    ])
                                }),
                        ),
                    ),
                    ("note", J::s(i.note)),
                ])
            })),
        ),
        (
            "fix_list",
            J::arr(f.fixes.iter().map(|x| {
                J::obj(vec![
                    ("rank", J::u(x.rank)),
                    ("title", J::s(&x.title)),
                    ("system", J::s(&x.system)),
                    ("functions", J::arr(x.functions.iter().map(J::s))),
                    ("stages_blocked", J::u(x.stages_blocked)),
                    ("main_line_quests", J::u(x.quests)),
                    ("uses", J::u(x.uses)),
                    ("size", J::s(x.size)),
                    ("translation_source", J::s(x.source)),
                    ("files", J::arr(x.files.iter().map(|p| J::s(*p)))),
                    (
                        "translated_handlers_in_tree",
                        J::arr(x.translated.iter().map(J::s)),
                    ),
                    ("agent_brief", J::s(&x.brief)),
                ])
            })),
        ),
    ])
}

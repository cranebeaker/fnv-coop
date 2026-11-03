//! `nv-coverage`: measures what is missing from nv-rs single-player, from the
//! game's own data. No Bevy; the same loaders the viewer uses.

pub mod checklist;
pub mod content;
pub mod data;
pub mod dialogue;
pub mod fixlist;
pub mod funcs;
pub mod gamelayer;
pub mod gamerows;
pub mod graph;
pub mod json;
pub mod objects;
pub mod quests;
pub mod render;
pub mod report;
pub mod scan;
pub mod sweep;
pub mod systems;

use std::collections::BTreeMap;
use std::path::Path;

pub use report::Full;

/// What to run.
#[derive(Debug, Clone)]
pub struct Options {
    pub functions: bool,
    pub quests: bool,
    pub dialogue: bool,
    pub objects: bool,
    /// `None` skips the sweep; `Some(None)` picks the mode from the data.
    pub sweep: Option<Option<sweep::Mode>>,
    pub limits: sweep::Limits,
}

impl Options {
    pub fn everything() -> Options {
        Options {
            functions: true,
            quests: true,
            dialogue: true,
            objects: true,
            sweep: Some(None),
            limits: sweep::Limits {
                max_interiors: None,
                stride: 1,
                max_squares_per_world: None,
            },
        }
    }
}

pub fn build(dir: &Path, opts: &Options) -> Result<Full, String> {
    let d = data::open(dir)?;
    let handled = funcs::handled_set();
    let sources = scan::sources(&d.order);
    let mut funcs_rep = funcs::FuncReport::default();
    if opts.functions || opts.dialogue {
        funcs_rep = funcs::analyse(&d.order, &sources);
    }
    let lines = if opts.quests {
        quests::walk(&d.order)
    } else {
        BTreeMap::new()
    };
    let dlg = if opts.dialogue {
        dialogue::analyse(&d.order, &sources, &handled)
    } else {
        Default::default()
    };
    let obj = if opts.objects {
        std::env::current_exe()
            .map(|exe| objects::walk(&exe, dir))
            .unwrap_or_default()
    } else {
        Default::default()
    };
    let graph = graph::build(&d.order, &sources, &funcs_rep);
    let sweep_rep = opts.sweep.map(|m| {
        let mode = m.unwrap_or(if d.has_meshes {
            sweep::Mode::Full
        } else {
            sweep::Mode::Records
        });
        sweep::run(dir, &d.order, mode, opts.limits)
    });
    let repo = checklist::repo_root();
    let kinds: Vec<String> = funcs_rep.events.keys().cloned().collect();
    let handled_events: BTreeMap<String, bool> = repo
        .as_deref()
        .map(|r| checklist::events_handled(r, &kinds).into_iter().collect())
        .unwrap_or_default();
    let mut events: Vec<(String, usize, Option<bool>)> = funcs_rep
        .events
        .iter()
        .map(|(k, n)| {
            (
                k.clone(),
                *n,
                repo.as_ref()
                    .map(|_| handled_events.get(k).copied().unwrap_or(false)),
            )
        })
        .collect();
    events.sort_by_key(|e| std::cmp::Reverse(e.1));
    let parse_errors: usize = funcs_rep.parse_errors.values().map(|v| v.0).sum();
    let trans = repo
        .as_deref()
        .map(content::translations)
        .unwrap_or_default();
    let perks = content::perk_census(&d.order, repo.as_deref());
    let fixes = fixlist::build(&funcs_rep, &lines, parse_errors, &trans);
    Ok(Full {
        data_dir: dir.to_path_buf(),
        plugins: d.order.plugins().iter().map(|p| p.name.clone()).collect(),
        has_meshes: d.has_meshes,
        repo,
        funcs: funcs_rep,
        lines,
        dialogue: dlg,
        objects: obj,
        graph,
        perks,
        sweep: sweep_rep,
        fixes,
        events,
    })
}

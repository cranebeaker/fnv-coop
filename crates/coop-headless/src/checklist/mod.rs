//! The master co-op checklist: a manifest of rows, each tied to the tests
//! that prove it, run in parallel and reported as one screen plus
//! `docs/COOP-CHECKLIST.md`.
//!
//! Rows live in `crates/coop-headless/checklist/*.json` (one file per area,
//! one object per row). A row names its tests as strings:
//!
//! * `scenario:NAME`: a scenario file (`tools/coop-run/scenarios` or
//!   `crates/coop-headless/scenarios`), run headless over the seeds and
//!   profiles asked for; unknown names are PENDING;
//! * `builtin:NAME`: a check compiled into [`builtin`];
//! * `conformance:NAME`: a backend conformance test ([`crate::backend`]);
//! * `questwalk:SCOPE`: the per-quest walk rollup (`all`, or a DLC name).
//!
//! A row with no tests is PENDING (no test yet). A row that asks for a
//! mutation twin is green only when its twin exists and fails as expected.

pub mod builtin;
pub mod report;
pub mod run;
pub mod units_harness;
pub mod units_more;
pub mod units_net;
pub mod units_sim;

use serde::Deserialize;
use std::path::{Path, PathBuf};

/// One row of the manifest.
#[derive(Clone, Debug, Deserialize)]
pub struct Row {
    /// Stable id (`area.topic`).
    pub id: String,
    /// Area (a table of the report).
    pub area: String,
    /// Short title.
    pub title: String,
    /// What a green row proves.
    #[serde(default)]
    pub proves: String,
    /// The tests, see the module docs.
    #[serde(default)]
    pub tests: Vec<String>,
    /// `base`, `dlc:NAME` or `full_assets`.
    #[serde(default = "base")]
    pub needs: String,
    /// A mutation twin is required.
    #[serde(default)]
    pub twin: bool,
    /// Explicit twin scenario ids; default `twin_` plus the scenario name.
    #[serde(default)]
    pub twin_ids: Vec<String>,
    /// `headless-real-logic` or `modelled`.
    #[serde(default = "real_logic")]
    pub fidelity: String,
    /// The named stand-in when `fidelity` is `modelled`.
    #[serde(default)]
    pub stand_in: String,
    /// Who owns the row.
    #[serde(default = "owner")]
    pub owner: String,
    /// `tools/coop-run` scenario names that prove the row in the real game.
    #[serde(default)]
    pub real: Vec<String>,
    /// `agnostic` (no world state involved) or `world` (needs the world backend).
    #[serde(default = "world")]
    pub backend: String,
    /// Where the row came from (parity audit section, list letter).
    #[serde(default)]
    pub source: String,
    /// A note for readers.
    #[serde(default)]
    pub note: String,
    /// `coop` (default) or `game`: the single-player completeness layer the
    /// co-op rows stand on.
    #[serde(default = "coop")]
    pub layer: String,
    /// Sub-area inside the area (a region, a questline, a system group).
    #[serde(default)]
    pub group: String,
    /// Green only with a real-game verdict (`realroute:` or `real`), on top of
    /// the headless tests.
    #[serde(default)]
    pub real_required: bool,
    /// How to fix the row when it is not green.
    #[serde(default)]
    pub fix: Option<Fix>,
    /// Row data the test kinds read (route quests, loadsweep members...).
    #[serde(default)]
    pub meta: serde_json::Value,
}

/// A fix brief for a non-green GAME row.
#[derive(Clone, Debug, Default, Deserialize)]
pub struct Fix {
    /// Exe addresses of the functions that implement it in the original.
    #[serde(default)]
    pub addresses: Vec<String>,
    /// Where the matching decompilation is (paths and function names only).
    #[serde(default)]
    pub decomp_ref: Vec<DecompRef>,
    /// `direct`, `partial` or `none`.
    #[serde(default)]
    pub decomp_ready: String,
    /// Why, in a sentence (what is missing when `partial`, what is absent when `none`).
    #[serde(default)]
    pub decomp_why: String,
    /// Repository files to change.
    #[serde(default)]
    pub files: Vec<String>,
    /// `S`, `M`, `L` or `XL`.
    #[serde(default)]
    pub size: String,
    /// Hours of work assuming the decomp is used.
    #[serde(default)]
    pub hours: f32,
    /// Row ids this one depends on.
    #[serde(default)]
    pub deps: Vec<String>,
    /// A ready agent brief.
    #[serde(default)]
    pub brief: String,
}

/// One decompilation unit and the functions to read in it.
#[derive(Clone, Debug, Default, Deserialize)]
pub struct DecompRef {
    /// Path under the decompilation root (`src/...`).
    #[serde(default)]
    pub path: String,
    /// `Class::Method` names with their exe addresses.
    #[serde(default)]
    pub functions: Vec<DecompFn>,
}

/// A named function of the decompilation.
#[derive(Clone, Debug, Default, Deserialize)]
pub struct DecompFn {
    /// `Class::Method`.
    #[serde(default)]
    pub name: String,
    /// Exe address.
    #[serde(default)]
    pub address: String,
}

fn base() -> String {
    "base".into()
}
fn real_logic() -> String {
    "headless-real-logic".into()
}
fn owner() -> String {
    "coop".into()
}
fn coop() -> String {
    "coop".into()
}
fn world() -> String {
    "world".into()
}

impl Row {
    /// True when the row passes only with a stand-in.
    pub fn modelled(&self) -> bool {
        self.fidelity == "modelled"
    }
}

#[derive(Deserialize)]
struct File {
    rows: Vec<Row>,
}

/// Where the manifest is: next to the crate sources.
pub fn default_manifest_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("checklist")
}

/// The repository root (two levels above the crate).
pub fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..")
}

/// Loads every `*.json` of the folder, in file name order; duplicate ids are an error.
pub fn load(dir: &Path) -> Result<Vec<Row>, String> {
    let mut files: Vec<PathBuf> = std::fs::read_dir(dir)
        .map_err(|e| format!("{}: {e}", dir.display()))?
        .flatten()
        .map(|e| e.path())
        .filter(|f| f.extension().is_some_and(|x| x == "json"))
        .collect();
    files.sort();
    // The GAME layer lives in `game/` next to the co-op files.
    if let Ok(rd) = std::fs::read_dir(dir.join("game")) {
        let mut g: Vec<PathBuf> = rd
            .flatten()
            .map(|e| e.path())
            .filter(|f| f.extension().is_some_and(|x| x == "json"))
            .collect();
        g.sort();
        files.extend(g);
    }
    let mut rows: Vec<Row> = Vec::new();
    for f in files {
        let text = std::fs::read_to_string(&f).map_err(|e| format!("{}: {e}", f.display()))?;
        let file: File =
            serde_json::from_str(&text).map_err(|e| format!("{}: {e}", f.display()))?;
        for mut r in file.rows {
            if f.parent().is_some_and(|p| p.ends_with("game")) && r.layer == "coop" {
                r.layer = "game".into();
            }
            if rows.iter().any(|x| x.id == r.id) {
                return Err(format!("{}: duplicate row id {}", f.display(), r.id));
            }
            rows.push(r);
        }
    }
    Ok(rows)
}

/// The state of one row after a run.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Status {
    /// All tests (and the twin) passed.
    Pass,
    /// Something failed; the text says what and how to replay it.
    Fail(String),
    /// No test yet, or a piece of it does not exist yet.
    Pending(String),
    /// The data the row needs is not here.
    Skip(String),
}

impl Status {
    /// Short label.
    pub fn label(&self) -> &'static str {
        match self {
            Status::Pass => "PASS",
            Status::Fail(_) => "FAIL",
            Status::Pending(_) => "PENDING",
            Status::Skip(_) => "SKIP",
        }
    }
}

/// A row and what happened to it.
#[derive(Clone, Debug)]
pub struct Outcome {
    /// The row (generated rows for single quests are built by the runner).
    pub row: Row,
    /// Status.
    pub status: Status,
    /// What ran (for the report).
    pub detail: String,
    /// Replay commands of failures.
    pub replay: Vec<String>,
    /// Wall milliseconds.
    pub ms: u64,
    /// Last real-game verdict date, if known.
    pub real_game: String,
    /// The row's SP-GAP count (questwalk rollups).
    pub sp_gaps: usize,
}

impl Outcome {
    /// Green: counts toward the verdict.
    pub fn green(&self) -> bool {
        self.status == Status::Pass
    }
}

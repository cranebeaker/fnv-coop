//! The unit trees of the engine crates (docs/ENGINE_CRATE.md).
//!
//! The engine is split into crates so a compile fits the machine:
//! `crates/engine-core` (the memory model, `Engine`, the CRT and platform
//! units), part crates `crates/engine-<part>` that each hold some subsystem
//! folders, and `crates/engine`, the facade that depends on all of them and
//! registers every function. Each crate with units has them in
//! `src/units`: one file per source unit of the engine map, in one folder per
//! subsystem: `fallout/ai/processlists.cpp` →
//! `crates/engine-fallout-ai/src/units/fallout_ai/processlists.rs`.
//! The `mod.rs` files are generated from the files present (`units`), so
//! translators only ever edit their own unit files.
//!
//! The game's own folders are spread over the `engine-fallout-*` crates by
//! unit, not by folder: `fallout_shared` and `unplaced` are split between
//! several of them. [`FALLOUT_CRATES`] says which crate each unit file goes
//! to; [`regenerate`] re-exports a split folder's units from the crates below
//! under the folder's one name, so `super::<unit>` and
//! `crate::units::<folder>::<unit>` reach them wherever they live.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

const KEYWORDS: &[&str] = &[
    "as", "async", "await", "box", "break", "const", "continue", "crate", "dyn", "else", "enum",
    "extern", "false", "fn", "for", "gen", "if", "impl", "in", "let", "loop", "match", "mod",
    "move", "mut", "priv", "pub", "ref", "return", "self", "static", "struct", "super", "trait",
    "true", "try", "type", "unsafe", "use", "where", "while", "yield", "abstract", "become", "do",
    "final", "macro", "override", "typeof", "unsized", "virtual",
];

/// A Rust module name from a path piece.
pub fn ident(s: &str) -> String {
    let mut out = String::new();
    for c in s.chars() {
        if c.is_ascii_alphanumeric() {
            out.push(c.to_ascii_lowercase());
        } else if !out.ends_with('_') {
            out.push('_');
        }
    }
    let out = out.trim_matches('_').to_string();
    if out.is_empty()
        || out.starts_with(|c: char| c.is_ascii_digit())
        || KEYWORDS.contains(&out.as_str())
    {
        format!("u_{out}")
    } else {
        out
    }
}

/// (subsystem folder, file stem) of a unit from the map.
pub fn module_of(unit: &str, subsystem: &str) -> (String, String) {
    let file = unit.rsplit('/').next().unwrap_or(unit);
    let stem = file.rsplit_once('.').map(|(a, _)| a).unwrap_or(file);
    (ident(subsystem), ident(stem))
}

/// The facade crate's folder under `crates/`.
pub const FACADE: &str = "engine";
/// The core crate's folder under `crates/`.
pub const CORE: &str = "engine-core";
/// The placement rules of the game's own units (folder, stem, crate; first
/// match wins), relative to the repository.
pub const FALLOUT_CRATES: &str = "scripts/ledger/fallout_crates.tsv";
/// The one crate the game's own units were in before they were split by area.
const FALLOUT_WHOLE: &str = "engine-fallout";

/// The part crate a subsystem folder that exists in no crate yet goes to
/// (`scaffold`): the same grouping as the existing folders
/// (docs/ENGINE_CRATE.md, "The engine crates"). The game's own folders go to
/// `engine-fallout` while that crate exists; after the split by area, a new
/// one needs a row in [`FALLOUT_CRATES`] ([`crate_of_unit`]).
pub fn home_of_new_subsystem(folder: &str) -> &'static str {
    if folder.starts_with("ni") {
        "engine-gamebryo"
    } else if folder.starts_with("havok") || folder == "bshavok" {
        "engine-havok"
    } else if folder.starts_with("bs") {
        "engine-bs"
    } else {
        FALLOUT_WHOLE
    }
}

/// One engine crate: its folder under `crates/` (also its package name)
/// and the engine crates it depends on directly, and those its tests add.
#[derive(Debug, Clone)]
pub struct Crate {
    pub dir: String,
    pub deps: Vec<String>,
    pub dev_deps: Vec<String>,
}

/// Every engine crate (`crates/engine` and `crates/engine-*`), sorted by
/// folder, with its engine dependencies read from its `Cargo.toml`
/// (`[dependencies]` and `[dev-dependencies]` entries with
/// `path = "../engine..."`).
pub fn crates(root: &Path) -> Result<Vec<Crate>, String> {
    let dir = root.join("crates");
    let mut out = Vec::new();
    for e in fs::read_dir(&dir)
        .map_err(|e| format!("{}: {e}", dir.display()))?
        .flatten()
    {
        let name = e.file_name().to_string_lossy().to_string();
        if !(name == FACADE || name.starts_with("engine-")) || !e.path().is_dir() {
            continue;
        }
        let toml = e.path().join("Cargo.toml");
        let Ok(text) = fs::read_to_string(&toml) else {
            continue;
        };
        out.push(Crate {
            dir: name,
            deps: engine_paths(&text, "[dependencies]"),
            dev_deps: engine_paths(&text, "[dev-dependencies]"),
        });
    }
    out.sort_by(|a, b| a.dir.cmp(&b.dir));
    Ok(out)
}

/// The engine crates a `Cargo.toml` names under `[dependencies]` (not
/// dev-dependencies), by their folder.
#[cfg(test)]
fn engine_deps(toml: &str) -> Vec<String> {
    engine_paths(toml, "[dependencies]")
}

/// The engine crates a `Cargo.toml` names in one section, by their folder.
fn engine_paths(toml: &str, want: &str) -> Vec<String> {
    let mut section = "";
    let mut out = Vec::new();
    for line in toml.lines() {
        let l = line.trim();
        if l.starts_with('[') {
            section = l;
            continue;
        }
        if section != want {
            continue;
        }
        if let Some(at) = l.find("path = \"../") {
            let rest = &l[at + "path = \"../".len()..];
            if let Some(end) = rest.find('"') {
                let dep = &rest[..end];
                if (dep == FACADE || dep.starts_with("engine-")) && !out.iter().any(|d| d == dep) {
                    out.push(dep.to_string());
                }
            }
        }
    }
    out
}

/// The `src/units` folder of a crate.
pub fn units_dir(root: &Path, krate: &str) -> PathBuf {
    root.join("crates").join(krate).join("src/units")
}

/// The modules a crate's `src/units` holds itself: subsystem folders with
/// their unit files, and top-level unit files such as `platform.rs`.
type Own = (BTreeMap<String, Vec<String>>, Vec<String>);

/// The unit modules a crate's `src/units` holds itself: (subsystem folders
/// with their unit files, top-level unit files such as `platform.rs`).
fn own_modules(units: &Path) -> Result<Own, String> {
    let mut subs: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut top: Vec<String> = Vec::new();
    let Ok(rd) = fs::read_dir(units) else {
        return Ok((subs, top));
    };
    for e in rd.flatten() {
        let p = e.path();
        let name = e.file_name().to_string_lossy().to_string();
        if p.is_dir() {
            let mut files = Vec::new();
            for f in fs::read_dir(&p).map_err(|e| e.to_string())?.flatten() {
                let n = f.file_name().to_string_lossy().to_string();
                if let Some(stem) = n.strip_suffix(".rs") {
                    if stem != "mod" {
                        files.push(stem.to_string());
                    }
                }
            }
            files.sort();
            subs.insert(name, files);
        } else if let Some(stem) = name.strip_suffix(".rs") {
            if stem != "mod" {
                top.push(stem.to_string());
            }
        }
    }
    // A folder named like a top-level file (`crt.rs` with `crt/`) holds that file's own
    // submodules (declared in the file), not a subsystem: it gets no `mod.rs` and no
    // `pub mod` or re-export of its own.
    subs.retain(|name, _| !top.contains(name));
    top.sort();
    Ok((subs, top))
}

/// Every engine crate `krate` depends on, directly or not, lowest first
/// (the core first, then in dependency order, ties by folder).
fn below(all: &[Crate], krate: &str) -> Vec<String> {
    let by: BTreeMap<&str, &Crate> = all.iter().map(|c| (c.dir.as_str(), c)).collect();
    let mut seen = BTreeSet::new();
    let mut order = Vec::new();
    fn visit(
        by: &BTreeMap<&str, &Crate>,
        c: &str,
        seen: &mut BTreeSet<String>,
        order: &mut Vec<String>,
    ) {
        let Some(k) = by.get(c) else { return };
        let mut deps = k.deps.clone();
        deps.sort();
        for d in deps {
            if seen.insert(d.clone()) {
                visit(by, &d, seen, order);
                order.push(d);
            }
        }
    }
    visit(&by, krate, &mut seen, &mut order);
    order
}

const HEADER: &str = "// Generated by `ledger units` from the files in this folder; do not edit.\n";

/// A crate's name as a Rust path.
fn rust_name(krate: &str) -> String {
    krate.replace('-', "_")
}

/// The `mod.rs` of each subsystem folder of one crate. A folder that crates
/// below this one also hold units of (a split folder) re-exports those units
/// by name, so `super::<unit>` reaches them as when the folder was in one
/// crate; `funcs` registers only this crate's own.
fn write_folders(
    units: &Path,
    subs: &BTreeMap<String, Vec<String>>,
    lower: &[String],
    owned: &BTreeMap<String, Own>,
) -> Result<usize, String> {
    let mut n = 0;
    for (sub, files) in subs {
        let mut s = String::from(HEADER);
        // A unit may share its subsystem's name (`bsaudio/bsaudio.cpp`).
        s.push_str(
            "#![allow(clippy::module_inception)]

",
        );
        for f in files {
            let _ = writeln!(s, "pub mod {f};");
        }
        let mut lines = Vec::new();
        for d in lower {
            if let Some(theirs) = owned.get(d).and_then(|o| o.0.get(sub)) {
                for f in theirs {
                    lines.push(format!("pub use {}::units::{sub}::{f};", rust_name(d)));
                }
            }
        }
        if !lines.is_empty() {
            let _ = writeln!(
                s,
                "\n// This folder's units in the crates below this one, so `super::<unit>` reaches them."
            );
            write_uses(&mut s, lines);
        }
        let _ = writeln!(s, "\npub fn funcs(out: &mut Vec<(u32, crate::AbiFn)>) {{");
        for f in files {
            let _ = writeln!(s, "    out.extend({f}::funcs());");
        }
        let _ = writeln!(s, "}}");
        write_if_changed(&units.join(sub).join("mod.rs"), &s)?;
        n += files.len();
    }
    Ok(n)
}

/// `funcs` over a crate's own modules.
fn own_funcs(s: &mut String, subs: &BTreeMap<String, Vec<String>>, top: &[String]) {
    for t in top {
        let _ = writeln!(s, "    out.extend({t}::funcs());");
    }
    for sub in subs.keys() {
        let _ = writeln!(s, "    {sub}::funcs(out);");
    }
}

/// The modules of the crates in `lower` that a crate's `units` reaches
/// without holding them itself (`mine`), appended to `s`: a module one crate
/// holds is re-exported whole (`pub use <crate>::units::<module>;`); a folder
/// several crates hold units of is an inline module that re-exports each of
/// those units by name.
fn write_below(
    s: &mut String,
    lower: &[String],
    owned: &BTreeMap<String, Own>,
    mine: &BTreeSet<&String>,
) -> Result<(), String> {
    let mut holders: BTreeMap<&String, Vec<&String>> = BTreeMap::new();
    for d in lower {
        let (subs, top) = &owned[d];
        for m in top.iter().chain(subs.keys()) {
            if !mine.contains(m) {
                holders.entry(m).or_default().push(d);
            }
        }
    }
    let mut lines = Vec::new();
    let mut merged = Vec::new();
    for (m, ds) in &holders {
        if let [d] = ds.as_slice() {
            lines.push(format!("pub use {}::units::{m};", rust_name(d)));
            continue;
        }
        let mut inner = Vec::new();
        for d in ds {
            let Some(files) = owned[*d].0.get(*m) else {
                return Err(format!(
                    "units::{m}: a top-level module of several crates ({ds:?})"
                ));
            };
            for f in files {
                inner.push(format!("    pub use {}::units::{m}::{f};", rust_name(d)));
            }
        }
        inner.sort();
        merged.push((m, inner));
    }
    write_uses(s, lines);
    if !merged.is_empty() {
        let _ = writeln!(
            s,
            "\n// Folders whose units are split between several crates below: each unit by name."
        );
        for (i, (m, inner)) in merged.iter().enumerate() {
            if i > 0 {
                s.push('\n');
            }
            let _ = writeln!(s, "pub mod {m} {{");
            for l in inner {
                let _ = writeln!(s, "{l}");
            }
            let _ = writeln!(s, "}}");
        }
    }
    Ok(())
}

/// Regenerates every `mod.rs` under the `src/units` of every engine crate
/// (the hand-written `platform.rs` and any other top-level file are kept as
/// modules), and the facade's `src/units/mod.rs`. Returns how many unit
/// modules there are. Refuses, changing nothing, when a file of an
/// `engine-fallout-*` crate is not where [`FALLOUT_CRATES`] puts it.
///
/// A crate's `units` also re-exports the subsystems of every engine crate
/// below it, so `crate::units::<subsystem>` reaches them as before the split;
/// its `funcs` registers only its own. A part crate's unit tests register
/// its own functions, those of the crates below it and those of the engine
/// crates its `[dev-dependencies]` name (`engine_core::assemble!` under
/// `cfg(test)`); the facade registers every part crate's.
pub fn regenerate(root: &Path) -> Result<usize, String> {
    let wrong = misplaced(root)?;
    if !wrong.is_empty() {
        return Err(format!(
            "{} file(s) not where {FALLOUT_CRATES} puts them:\n{}",
            wrong.len(),
            wrong.join("\n")
        ));
    }
    let all = crates(root)?;
    let mut owned: BTreeMap<String, Own> = BTreeMap::new();
    for c in all.iter().filter(|c| c.dir != FACADE) {
        owned.insert(c.dir.clone(), own_modules(&units_dir(root, &c.dir))?);
    }
    let mut n = 0;
    for c in all.iter().filter(|c| c.dir != FACADE) {
        let units = units_dir(root, &c.dir);
        if !units.is_dir() {
            continue;
        }
        let (subs, top) = &owned[&c.dir];
        let lower = below(&all, &c.dir);
        n += write_folders(&units, subs, &lower, &owned)? + top.len();
        let mut s = String::from(HEADER);
        let _ = writeln!(
            s,
            "//! Translated units of `{}`, one folder per subsystem (docs/ENGINE_CRATE.md).\n",
            c.dir
        );
        let mut mods: Vec<&String> = top.iter().chain(subs.keys()).collect();
        mods.sort();
        for m in &mods {
            let _ = writeln!(s, "pub mod {m};");
        }
        if !lower.is_empty() {
            let _ = writeln!(
                s,
                "\n// The units of the crates below this one, so `crate::units::<subsystem>` reaches them."
            );
            write_below(&mut s, &lower, &owned, &mods.iter().copied().collect())?;
        }
        let _ = writeln!(s, "\n/// Every registered function of this crate's units.");
        let _ = writeln!(s, "pub fn funcs(out: &mut Vec<(u32, crate::AbiFn)>) {{");
        own_funcs(&mut s, subs, top);
        let _ = writeln!(s, "}}");
        let tested: Vec<&String> = c
            .dev_deps
            .iter()
            .filter(|d| **d != CORE && **d != c.dir && !lower.contains(d))
            .filter(|d| owned.contains_key(*d))
            .collect();
        if tested.is_empty() {
            let _ = writeln!(
                s,
                "\n// This crate's unit tests: `Engine::new()` registers the core's functions,\n\
                 // this crate's and those of the crates below it."
            );
        } else {
            let _ = writeln!(
                s,
                "\n// This crate's unit tests: `Engine::new()` registers the core's functions,\n\
                 // this crate's, those of the crates below it and those of the engine crates its\n\
                 // tests add (`[dev-dependencies]`)."
            );
        }
        if c.dir == CORE {
            let _ = writeln!(s, "#[cfg(test)]\ncrate::assemble!();");
        } else {
            let mut list = vec!["funcs".to_string()];
            for d in lower.iter().filter(|d| *d != CORE).chain(tested) {
                list.push(format!("{}::units::funcs", rust_name(d)));
            }
            let _ = writeln!(s, "#[cfg(test)]");
            assemble(&mut s, &list);
        }
        write_if_changed(&units.join("mod.rs"), &s)?;
    }
    if all.iter().any(|c| c.dir == FACADE) {
        let lower = below(&all, FACADE);
        let mut s = String::from(HEADER);
        let _ = writeln!(
            s,
            "//! Every translated unit, from the engine crates (docs/ENGINE_CRATE.md):\n\
             //! `engine::units::<subsystem>` as before the split.\n"
        );
        write_below(&mut s, &lower, &owned, &BTreeSet::new())?;
        let _ = writeln!(s, "\n/// Every registered function of every unit.");
        let _ = writeln!(s, "pub fn funcs(out: &mut Vec<(u32, crate::AbiFn)>) {{");
        for d in &lower {
            let _ = writeln!(s, "    {}::units::funcs(out);", rust_name(d));
        }
        let _ = writeln!(s, "}}");
        let _ = writeln!(
            s,
            "\n/// One engine crate's `units::funcs`.\n\
             pub type Funcs = fn(&mut Vec<(u32, crate::AbiFn)>);"
        );
        let _ = writeln!(
            s,
            "\n/// Each engine crate's registered functions, by crate (no address may be in two)."
        );
        let _ = writeln!(s, "pub const PARTS: &[(&str, Funcs)] = &[");
        for d in &lower {
            // As rustfmt writes it: a tuple whose items are wider than 60 columns
            // (`fn_call_width`) goes one item per line.
            let (name, funcs) = (
                format!("\"{d}\""),
                format!("{}::units::funcs", rust_name(d)),
            );
            if name.len() + 2 + funcs.len() > 60 {
                let _ = writeln!(s, "    (\n        {name},\n        {funcs},\n    ),");
            } else {
                let _ = writeln!(s, "    ({name}, {funcs}),");
            }
        }
        let _ = writeln!(s, "];");
        let parts: Vec<String> = lower
            .iter()
            .filter(|d| *d != CORE)
            .map(|d| format!("{}::units::funcs", rust_name(d)))
            .collect();
        let _ = writeln!(
            s,
            "\n// The full engine: `Engine::new()` registers the core's functions and every part crate's."
        );
        assemble(&mut s, &parts);
        write_if_changed(&units_dir(root, FACADE).join("mod.rs"), &s)?;
    }
    Ok(n)
}

/// `engine_core::assemble!(..);` over `list`, laid out as rustfmt lays it out.
fn assemble(s: &mut String, list: &[String]) {
    let line = format!("engine_core::assemble!({});", list.join(", "));
    if line.len() <= 100 {
        let _ = writeln!(s, "{line}");
    } else {
        let _ = writeln!(s, "engine_core::assemble!(");
        for f in list {
            let _ = writeln!(s, "    {f},");
        }
        let _ = writeln!(s, ");");
    }
}

/// Writes sorted `use` lines.
fn write_uses(s: &mut String, mut lines: Vec<String>) {
    lines.sort();
    for l in lines {
        let _ = writeln!(s, "{l}");
    }
}

fn write_if_changed(p: &Path, s: &str) -> Result<(), String> {
    if fs::read_to_string(p).ok().as_deref() == Some(s) {
        return Ok(());
    }
    fs::write(p, s).map_err(|e| format!("{}: {e}", p.display()))
}

/// The engine crate whose `src/units` holds subsystem folder `sub`, or the
/// one a new folder goes to ([`home_of_new_subsystem`]). For a folder whose
/// units are split between crates, the first of them by name: place a unit
/// with [`crate_of_unit`].
pub fn crate_of(root: &Path, sub: &str) -> String {
    if let Ok(all) = crates(root) {
        for c in all.iter().filter(|c| c.dir != FACADE) {
            let units = units_dir(root, &c.dir);
            if units.join(sub).is_dir() || units.join(format!("{sub}.rs")).is_file() {
                return c.dir.clone();
            }
        }
    }
    home_of_new_subsystem(sub).to_string()
}

/// A unit file's stem without a part suffix (`actor_p3` → `actor`): part
/// files go where their unit goes.
fn unit_stem(stem: &str) -> &str {
    if let Some(at) = stem.rfind("_p") {
        let digits = &stem[at + 2..];
        if !digits.is_empty() && digits.bytes().all(|b| b.is_ascii_digit()) {
            return &stem[..at];
        }
    }
    stem
}

/// One row of [`FALLOUT_CRATES`]: folder pattern, stem pattern, crate.
type Rule = (Regex, Regex, String);

/// The rows of [`FALLOUT_CRATES`] (none when the file does not exist).
fn rules(root: &Path) -> Result<Vec<Rule>, String> {
    let Ok(text) = fs::read_to_string(root.join(FALLOUT_CRATES)) else {
        return Ok(Vec::new());
    };
    parse_rules(&text)
}

fn parse_rules(text: &str) -> Result<Vec<Rule>, String> {
    let mut out = Vec::new();
    for (i, line) in text.lines().enumerate() {
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let f: Vec<&str> = line.split('\t').collect();
        let [folder, stem, krate] = f.as_slice() else {
            return Err(format!(
                "{FALLOUT_CRATES}:{}: want three tab-separated fields",
                i + 1
            ));
        };
        let at = |e: String| format!("{FALLOUT_CRATES}:{}: {e}", i + 1);
        out.push((
            Regex::new(folder).map_err(at)?,
            Regex::new(stem).map_err(at)?,
            krate.to_string(),
        ));
    }
    Ok(out)
}

/// The crate the rules put a unit file in (`folder`, file stem), whether or
/// not that crate exists.
fn rule_crate<'a>(rules: &'a [Rule], folder: &str, stem: &str) -> Option<&'a str> {
    let stem = unit_stem(stem);
    rules
        .iter()
        .find(|(f, s, _)| f.matches(folder) && s.matches(stem))
        .map(|(_, _, c)| c.as_str())
}

/// The engine crate a unit file belongs in: the crate that already holds
/// `src/units/<folder>/<stem>.rs`; otherwise the first row of
/// [`FALLOUT_CRATES`] that matches and whose crate exists; otherwise the
/// crate of the folder ([`crate_of`]), unless that is the game's single
/// crate from before the split by area and it no longer exists: then the
/// folder needs a row in [`FALLOUT_CRATES`] first.
pub fn crate_of_unit(root: &Path, folder: &str, stem: &str) -> Result<String, String> {
    if let Ok(all) = crates(root) {
        for c in all.iter().filter(|c| c.dir != FACADE) {
            if units_dir(root, &c.dir)
                .join(folder)
                .join(format!("{stem}.rs"))
                .is_file()
            {
                return Ok(c.dir.clone());
            }
        }
    }
    let stem_base = unit_stem(stem);
    for (f, s, krate) in rules(root)? {
        if f.matches(folder) && s.matches(stem_base) && root.join("crates").join(&krate).is_dir() {
            return Ok(krate);
        }
    }
    let krate = crate_of(root, folder);
    if krate == FALLOUT_WHOLE && !root.join("crates").join(FALLOUT_WHOLE).is_dir() {
        return Err(format!(
            "units/{folder}/{stem}.rs: no row of {FALLOUT_CRATES} places it; add one"
        ));
    }
    Ok(krate)
}

/// The files under `crates/engine-fallout-*/src/units/<folder>/` that are
/// not in the crate [`FALLOUT_CRATES`] puts them in (`mod.rs` aside).
pub fn misplaced(root: &Path) -> Result<Vec<String>, String> {
    let rules = rules(root)?;
    let mut out = Vec::new();
    for c in crates(root)?
        .iter()
        .filter(|c| c.dir.starts_with("engine-fallout-"))
    {
        let (subs, top) = own_modules(&units_dir(root, &c.dir))?;
        for t in &top {
            out.push(format!(
                "crates/{}/src/units/{t}.rs: not in a folder",
                c.dir
            ));
        }
        for (folder, files) in &subs {
            for f in files {
                let want = rule_crate(&rules, folder, f);
                if want != Some(c.dir.as_str()) {
                    out.push(format!(
                        "crates/{}/src/units/{folder}/{f}.rs: belongs in {}",
                        c.dir,
                        want.unwrap_or("(no rule)")
                    ));
                }
            }
        }
    }
    Ok(out)
}

/// The repository path of a unit's file (whether or not it exists yet):
/// `crates/<crate>/src/units/<subsystem>/<unit>.rs`, and its crate.
pub fn unit_path(root: &Path, unit: &str, subsystem: &str) -> Result<(String, String), String> {
    let (sub, stem) = module_of(unit, subsystem);
    let krate = crate_of_unit(root, &sub, &stem)?;
    Ok((format!("crates/{krate}/src/units/{sub}/{stem}.rs"), krate))
}

/// Creates the file of a unit if it does not exist yet. Returns its path
/// relative to the repository.
pub fn scaffold(root: &Path, unit: &str, subsystem: &str) -> Result<String, String> {
    let (rel, _) = unit_path(root, unit, subsystem)?;
    let file = root.join(&rel);
    if let Some(dir) = file.parent() {
        fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    }
    if file.exists() {
        return Ok(rel);
    }
    let s = format!(
        "//! `{unit}` (Xbox PDB source unit), subsystem `{subsystem}`: its functions in\n\
         //! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).\n\
         \n\
         #[allow(unused_imports)]\n\
         use crate::prelude::*;\n\
         \n\
         /// This unit's translated functions, by exe address.\n\
         pub fn funcs() -> Vec<(u32, AbiFn)> {{\n    vec![]\n}}\n"
    );
    fs::write(&file, s).map_err(|e| format!("{}: {e}", file.display()))?;
    Ok(rel)
}

/// The extended-regex subset [`FALLOUT_CRATES`] is written in (the rules are
/// also read by `awk`), matched against a whole name: alternatives (`a|b`),
/// `.`, bracket classes with ranges (`[4-7]`, `[0-9a-f]`), `\` before a
/// literal, and `*`, `+`, `?`, `{n}`, `{n,m}` after an atom. Anything else
/// (groups, anchors) is refused.
#[derive(Debug, Clone)]
pub struct Regex(Vec<Vec<Atom>>);

#[derive(Debug, Clone)]
struct Atom {
    /// `None`: any character; otherwise inclusive ranges.
    set: Option<Vec<(char, char)>>,
    min: usize,
    max: usize,
}

impl Regex {
    pub fn new(pattern: &str) -> Result<Regex, String> {
        let mut alts = Vec::new();
        for alt in pattern.split('|') {
            let cs: Vec<char> = alt.chars().collect();
            let mut atoms: Vec<Atom> = Vec::new();
            let mut i = 0;
            while i < cs.len() {
                let set = match cs[i] {
                    '.' => None,
                    '[' => {
                        let mut ranges = Vec::new();
                        i += 1;
                        while i < cs.len() && cs[i] != ']' {
                            if i + 2 < cs.len() && cs[i + 1] == '-' && cs[i + 2] != ']' {
                                ranges.push((cs[i], cs[i + 2]));
                                i += 3;
                            } else {
                                ranges.push((cs[i], cs[i]));
                                i += 1;
                            }
                        }
                        if i == cs.len() || ranges.is_empty() || ranges[0].0 == '^' {
                            return Err(format!("{pattern}: unsupported bracket class"));
                        }
                        Some(ranges)
                    }
                    '\\' if i + 1 < cs.len() => {
                        i += 1;
                        Some(vec![(cs[i], cs[i])])
                    }
                    '(' | ')' | '^' | '$' | '\\' | '*' | '+' | '?' | '{' | '}' | ']' => {
                        return Err(format!("{pattern}: unsupported `{}`", cs[i]));
                    }
                    c => Some(vec![(c, c)]),
                };
                i += 1;
                let (min, max, used) = match cs.get(i) {
                    Some('*') => (0, usize::MAX, 1),
                    Some('+') => (1, usize::MAX, 1),
                    Some('?') => (0, 1, 1),
                    Some('{') => {
                        let end = cs[i..]
                            .iter()
                            .position(|&c| c == '}')
                            .ok_or(format!("{pattern}: unclosed `{{`"))?;
                        let body: String = cs[i + 1..i + end].iter().collect();
                        let num = |s: &str| {
                            s.trim()
                                .parse::<usize>()
                                .map_err(|_| format!("{pattern}: bad repeat `{{{body}}}`"))
                        };
                        let r = match body.split_once(',') {
                            Some((a, b)) => (num(a)?, num(b)?),
                            None => (num(&body)?, num(&body)?),
                        };
                        (r.0, r.1, end + 1)
                    }
                    _ => (1, 1, 0),
                };
                i += used;
                atoms.push(Atom { set, min, max });
            }
            alts.push(atoms);
        }
        Ok(Regex(alts))
    }

    /// Whether the whole of `text` matches.
    pub fn matches(&self, text: &str) -> bool {
        let cs: Vec<char> = text.chars().collect();
        self.0.iter().any(|atoms| match_from(atoms, &cs))
    }
}

fn match_from(atoms: &[Atom], cs: &[char]) -> bool {
    let Some((a, rest)) = atoms.split_first() else {
        return cs.is_empty();
    };
    let fits = |c: char| match &a.set {
        None => true,
        Some(r) => r.iter().any(|&(lo, hi)| lo <= c && c <= hi),
    };
    let most = cs.iter().take(a.max).take_while(|&&c| fits(c)).count();
    (a.min..=most).rev().any(|n| match_from(rest, &cs[n..]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dependencies_are_the_engine_paths_under_dependencies() {
        let toml = "[package]\nname = \"engine-havok\"\n\n[dependencies]\n\
                    engine-core = { path = \"../engine-core\" }\n\
                    engine-gamebryo = { path = \"../engine-gamebryo\" }\n\
                    esm = { path = \"../esm\" }\n\n[dev-dependencies]\n\
                    engine-bs = { path = \"../engine-bs\" }\n\
                    engine-core = { path = \"../engine-core\", features = [\"test-support\"] }\n";
        assert_eq!(engine_deps(toml), ["engine-core", "engine-gamebryo"]);
        assert_eq!(
            engine_paths(toml, "[dev-dependencies]"),
            ["engine-bs", "engine-core"]
        );
    }

    #[test]
    fn crates_below_come_lowest_first() {
        let c = |dir: &str, deps: &[&str]| Crate {
            dir: dir.into(),
            deps: deps.iter().map(|d| d.to_string()).collect(),
            dev_deps: Vec::new(),
        };
        let all = [
            c("engine", &["engine-havok", "engine-core"]),
            c("engine-core", &[]),
            c("engine-gamebryo", &["engine-core"]),
            c("engine-havok", &["engine-gamebryo", "engine-core"]),
        ];
        assert_eq!(
            below(&all, "engine"),
            ["engine-core", "engine-gamebryo", "engine-havok"]
        );
        assert_eq!(below(&all, "engine-core"), Vec::<String>::new());
    }

    #[test]
    fn a_folder_beside_a_same_named_file_is_that_files_submodules() {
        let units = std::env::temp_dir().join(format!("ledger_units_{}", std::process::id()));
        let _ = fs::remove_dir_all(&units);
        for dir in ["crt", "bsaudio"] {
            fs::create_dir_all(units.join(dir)).unwrap();
        }
        for file in [
            "crt.rs",
            "platform.rs",
            "crt/math.rs",
            "bsaudio/bsaudio.rs",
            "bsaudio/mod.rs",
        ] {
            fs::write(units.join(file), "").unwrap();
        }
        let (subs, top) = own_modules(&units).unwrap();
        fs::remove_dir_all(&units).unwrap();
        assert_eq!(top, ["crt", "platform"]);
        assert_eq!(subs.keys().collect::<Vec<_>>(), ["bsaudio"]);
        assert_eq!(subs["bsaudio"], ["bsaudio"]);
    }

    #[test]
    fn the_rules_subset_matches_whole_names() {
        let r = Regex::new("run_00[4-7][0-9a-f]{5}").unwrap();
        assert!(r.matches("run_004012a0"));
        assert!(r.matches("run_007fffff"));
        assert!(!r.matches("run_008a81e0"));
        assert!(!r.matches("run_004012a0x"));
        assert!(!r.matches("run_004012a"));
        let r = Regex::new("tes|tesobjectcell|.*_x").unwrap();
        assert!(r.matches("tes"));
        assert!(r.matches("tesobjectcell"));
        assert!(!r.matches("tesobjectcel"));
        assert!(r.matches("a_x"));
        assert!(Regex::new(".*").unwrap().matches(""));
        assert!(Regex::new("a{1,2}b?").unwrap().matches("aab"));
        assert!(!Regex::new("a{1,2}b?").unwrap().matches("aaab"));
        for bad in ["(a|b)", "^a", "a$", "[^a]", "*a"] {
            assert!(Regex::new(bad).is_err(), "{bad}");
        }
    }

    #[test]
    fn part_files_follow_their_unit() {
        assert_eq!(unit_stem("actor_p3"), "actor");
        assert_eq!(unit_stem("actor"), "actor");
        assert_eq!(unit_stem("run_p"), "run_p");
        assert_eq!(unit_stem("a_px1"), "a_px1");
        let rules = parse_rules(
            "# folder\tstem\tcrate\nunplaced\trun_00[4-7][0-9a-f]{5}\tA\n\
             unplaced\t.*\tB\nfallout_shared\ttes|extradatalist\tC\n",
        )
        .unwrap();
        assert_eq!(rule_crate(&rules, "unplaced", "run_004012a0_p2"), Some("A"));
        assert_eq!(rule_crate(&rules, "unplaced", "run_00e673f0"), Some("B"));
        assert_eq!(rule_crate(&rules, "fallout_shared", "tes_p2"), Some("C"));
        assert_eq!(rule_crate(&rules, "fallout_shared", "tesnpc"), None);
    }

    /// A temporary repository: `files` (path, contents) under it.
    fn tree(name: &str, files: &[(&str, &str)]) -> PathBuf {
        let root = std::env::temp_dir().join(format!("ledger_{name}_{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        for (path, text) in files {
            let p = root.join(path);
            fs::create_dir_all(p.parent().unwrap()).unwrap();
            fs::write(p, text).unwrap();
        }
        root
    }

    fn toml(deps: &[&str], dev: &[&str]) -> String {
        let mut s = String::from("[package]\n\n[dependencies]\n");
        for d in deps {
            s += &format!("{d} = {{ path = \"../{d}\" }}\n");
        }
        s += "\n[dev-dependencies]\n";
        for d in dev {
            s += &format!("{d} = {{ path = \"../{d}\" }}\n");
        }
        s
    }

    const RULES: &str = "# folder\tstem\tcrate\n\
                         shared\tlow|low_run\tengine-fallout-low\n\
                         shared\t.*\tengine-fallout-high\n\
                         ai\t.*\tengine-fallout-high\n";

    #[test]
    fn a_split_folder_reaches_every_unit_under_one_name() {
        let core = toml(&[], &[]);
        let low = toml(&["engine-core"], &["engine-core", "engine-fallout-high"]);
        let high = toml(&["engine-core", "engine-fallout-low"], &["engine-core"]);
        let facade = toml(
            &["engine-core", "engine-fallout-low", "engine-fallout-high"],
            &[],
        );
        let root = tree(
            "split",
            &[
                ("scripts/ledger/fallout_crates.tsv", RULES),
                ("crates/engine-core/Cargo.toml", &core),
                ("crates/engine-core/src/units/platform.rs", ""),
                ("crates/engine-fallout-low/Cargo.toml", &low),
                ("crates/engine-fallout-low/src/units/shared/low.rs", ""),
                ("crates/engine-fallout-low/src/units/shared/low_p2.rs", ""),
                ("crates/engine-fallout-high/Cargo.toml", &high),
                ("crates/engine-fallout-high/src/units/shared/form.rs", ""),
                ("crates/engine-fallout-high/src/units/ai/actor.rs", ""),
                ("crates/engine/Cargo.toml", &facade),
                ("crates/engine/src/units/mod.rs", ""),
            ],
        );
        assert_eq!(regenerate(&root), Ok(5));
        let read = |p: &str| fs::read_to_string(root.join(p)).unwrap();
        let high_shared = read("crates/engine-fallout-high/src/units/shared/mod.rs");
        let low_units = read("crates/engine-fallout-low/src/units/mod.rs");
        let facade_units = read("crates/engine/src/units/mod.rs");
        // Placement: an existing file stays, a new one follows the rules, a new folder
        // without a row and without the old single crate is refused.
        let placed = (
            crate_of_unit(&root, "shared", "low_p3"),
            crate_of_unit(&root, "shared", "tesnpc"),
            crate_of_unit(&root, "ai", "form"),
            crate_of_unit(&root, "fallout_new", "x").is_err(),
        );
        // A file in the wrong crate stops `units`.
        fs::write(
            root.join("crates/engine-fallout-high/src/units/shared/low_run.rs"),
            "",
        )
        .unwrap();
        let wrong = regenerate(&root);
        fs::remove_dir_all(&root).unwrap();

        assert!(high_shared.contains("pub mod form;\n"));
        assert!(high_shared.contains("pub use engine_fallout_low::units::shared::low;\n"));
        assert!(high_shared.contains("pub use engine_fallout_low::units::shared::low_p2;\n"));
        assert!(high_shared.contains("    out.extend(form::funcs());\n}"));
        assert!(!high_shared.contains("low::funcs"));
        // The lower crate's tests also register the crate its dev-dependencies add.
        assert!(
            low_units.contains("engine_core::assemble!(funcs, engine_fallout_high::units::funcs);")
        );
        assert!(facade_units.contains("pub use engine_fallout_high::units::ai;\n"));
        assert!(facade_units.contains(
            "pub mod shared {\n    pub use engine_fallout_high::units::shared::form;\n    \
             pub use engine_fallout_low::units::shared::low;\n    \
             pub use engine_fallout_low::units::shared::low_p2;\n}\n"
        ));
        assert!(facade_units
            .contains("    (\"engine-fallout-high\", engine_fallout_high::units::funcs),\n"));
        assert_eq!(
            placed,
            (
                Ok("engine-fallout-low".to_string()),
                Ok("engine-fallout-high".to_string()),
                Ok("engine-fallout-high".to_string()),
                true
            )
        );
        let wrong = wrong.unwrap_err();
        assert!(
            wrong.contains(
                "engine-fallout-high/src/units/shared/low_run.rs: belongs in engine-fallout-low"
            ),
            "{wrong}"
        );
    }
}

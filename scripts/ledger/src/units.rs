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

/// The part crate a subsystem folder that exists in no crate yet goes to
/// (`scaffold`): the same grouping as the existing folders
/// (docs/ENGINE_CRATE.md, "The engine crates").
pub fn home_of_new_subsystem(folder: &str) -> &'static str {
    if folder.starts_with("ni") {
        "engine-gamebryo"
    } else if folder.starts_with("havok") || folder == "bshavok" {
        "engine-havok"
    } else if folder.starts_with("bs") {
        "engine-bs"
    } else {
        "engine-fallout"
    }
}

/// One engine crate: its folder under `crates/` (also its package name)
/// and the engine crates it depends on directly.
#[derive(Debug, Clone)]
pub struct Crate {
    pub dir: String,
    pub deps: Vec<String>,
}

/// Every engine crate (`crates/engine` and `crates/engine-*`), sorted by
/// folder, with its engine dependencies read from its `Cargo.toml`
/// (`[dependencies]` entries with `path = "../engine..."`).
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
            deps: engine_deps(&text),
        });
    }
    out.sort_by(|a, b| a.dir.cmp(&b.dir));
    Ok(out)
}

/// The engine crates a `Cargo.toml` names under `[dependencies]` (not
/// dev-dependencies), by their folder.
fn engine_deps(toml: &str) -> Vec<String> {
    let mut section = "";
    let mut out = Vec::new();
    for line in toml.lines() {
        let l = line.trim();
        if l.starts_with('[') {
            section = l;
            continue;
        }
        if section != "[dependencies]" {
            continue;
        }
        if let Some(at) = l.find("path = \"../") {
            let rest = &l[at + "path = \"../".len()..];
            if let Some(end) = rest.find('"') {
                let dep = &rest[..end];
                if dep == FACADE || dep.starts_with("engine-") {
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

/// The unit modules a crate's `src/units` holds itself: (subsystem folders
/// with their unit files, top-level unit files such as `platform.rs`).
fn own_modules(units: &Path) -> Result<(BTreeMap<String, Vec<String>>, Vec<String>), String> {
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
    fn visit<'a>(
        by: &BTreeMap<&str, &'a Crate>,
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

/// The `mod.rs` of each subsystem folder of one crate.
fn write_folders(units: &Path, subs: &BTreeMap<String, Vec<String>>) -> Result<usize, String> {
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

/// Regenerates every `mod.rs` under the `src/units` of every engine crate
/// (the hand-written `platform.rs` and any other top-level file are kept as
/// modules), and the facade's `src/units/mod.rs`. Returns how many unit
/// modules there are.
///
/// A crate's `units` also re-exports the subsystems of every engine crate
/// below it, so `crate::units::<subsystem>` reaches them as before the split;
/// its `funcs` registers only its own. A part crate's unit tests register
/// its own functions and those of the crates below it (`engine_core::assemble!`
/// under `cfg(test)`); the facade registers every part crate's.
pub fn regenerate(root: &Path) -> Result<usize, String> {
    let all = crates(root)?;
    let mut owned: BTreeMap<String, (BTreeMap<String, Vec<String>>, Vec<String>)> = BTreeMap::new();
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
        n += write_folders(&units, subs)? + top.len();
        let mut s = String::from(HEADER);
        let _ = writeln!(
            s,
            "//! Translated units of `{}`, one folder per subsystem (docs/ENGINE_CRATE.md).\n",
            c.dir
        );
        let mut mods: Vec<&String> = top.iter().chain(subs.keys()).collect();
        mods.sort();
        for m in mods {
            let _ = writeln!(s, "pub mod {m};");
        }
        let lower = below(&all, &c.dir);
        if !lower.is_empty() {
            let _ = writeln!(
                s,
                "\n// The units of the crates below this one, so `crate::units::<subsystem>` reaches them."
            );
            let mut lines = Vec::new();
            for d in &lower {
                reexport(&mut lines, d, &owned[d]);
            }
            write_uses(&mut s, lines);
        }
        let _ = writeln!(s, "\n/// Every registered function of this crate's units.");
        let _ = writeln!(s, "pub fn funcs(out: &mut Vec<(u32, crate::AbiFn)>) {{");
        own_funcs(&mut s, subs, top);
        let _ = writeln!(s, "}}");
        let _ = writeln!(
            s,
            "\n// This crate's unit tests: `Engine::new()` registers the core's functions,\n\
             // this crate's and those of the crates below it."
        );
        if c.dir == CORE {
            let _ = writeln!(s, "#[cfg(test)]\ncrate::assemble!();");
        } else {
            let mut list = vec!["funcs".to_string()];
            for d in lower.iter().filter(|d| *d != CORE) {
                list.push(format!("{}::units::funcs", d.replace('-', "_")));
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
        let mut lines = Vec::new();
        for d in &lower {
            reexport(&mut lines, d, &owned[d]);
        }
        write_uses(&mut s, lines);
        let _ = writeln!(s, "\n/// Every registered function of every unit.");
        let _ = writeln!(s, "pub fn funcs(out: &mut Vec<(u32, crate::AbiFn)>) {{");
        for d in &lower {
            let _ = writeln!(s, "    {}::units::funcs(out);", d.replace('-', "_"));
        }
        let _ = writeln!(s, "}}");
        let parts: Vec<String> = lower
            .iter()
            .filter(|d| *d != CORE)
            .map(|d| format!("{}::units::funcs", d.replace('-', "_")))
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

/// One `pub use <crate>::units::<module>;` line per module a crate holds
/// itself, appended to `lines` (sorted by the caller, as rustfmt orders them).
fn reexport(
    lines: &mut Vec<String>,
    krate: &str,
    own: &(BTreeMap<String, Vec<String>>, Vec<String>),
) {
    let (subs, top) = own;
    for m in top.iter().chain(subs.keys()) {
        lines.push(format!("pub use {}::units::{m};", krate.replace('-', "_")));
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
/// one a new folder goes to ([`home_of_new_subsystem`]).
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

/// The repository path of a unit's file (whether or not it exists yet):
/// `crates/<crate>/src/units/<subsystem>/<unit>.rs`.
pub fn unit_path(root: &Path, unit: &str, subsystem: &str) -> (String, String) {
    let (sub, stem) = module_of(unit, subsystem);
    let krate = crate_of(root, &sub);
    (format!("crates/{krate}/src/units/{sub}/{stem}.rs"), krate)
}

/// Creates the file of a unit if it does not exist yet. Returns its path
/// relative to the repository.
pub fn scaffold(root: &Path, unit: &str, subsystem: &str) -> Result<String, String> {
    let (rel, _) = unit_path(root, unit, subsystem);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dependencies_are_the_engine_paths_under_dependencies() {
        let toml = "[package]\nname = \"engine-havok\"\n\n[dependencies]\n\
                    engine-core = { path = \"../engine-core\" }\n\
                    engine-gamebryo = { path = \"../engine-gamebryo\" }\n\
                    esm = { path = \"../esm\" }\n\n[dev-dependencies]\n\
                    engine-bs = { path = \"../engine-bs\" }\n";
        assert_eq!(engine_deps(toml), ["engine-core", "engine-gamebryo"]);
    }

    #[test]
    fn crates_below_come_lowest_first() {
        let c = |dir: &str, deps: &[&str]| Crate {
            dir: dir.into(),
            deps: deps.iter().map(|d| d.to_string()).collect(),
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
        for file in ["crt.rs", "platform.rs", "crt/math.rs", "bsaudio/bsaudio.rs", "bsaudio/mod.rs"] {
            fs::write(units.join(file), "").unwrap();
        }
        let (subs, top) = own_modules(&units).unwrap();
        fs::remove_dir_all(&units).unwrap();
        assert_eq!(top, ["crt", "platform"]);
        assert_eq!(subs.keys().collect::<Vec<_>>(), ["bsaudio"]);
        assert_eq!(subs["bsaudio"], ["bsaudio"]);
    }
}

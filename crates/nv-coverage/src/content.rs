//! Measurements that need the repository's source text as well as the data:
//! which perk entry points the world crate applies, and which script
//! functions already have a translated exe handler to port from.

use std::collections::{BTreeMap, HashMap};
use std::path::Path;

use esm::{FourCC, LoadOrder};

#[derive(Debug, Default)]
pub struct PerkCensus {
    pub perks: usize,
    pub entry_points: usize,
    /// Entry point number → (name, entry points in perks, applied by the world crate).
    pub by_entry: BTreeMap<u8, (String, usize, bool)>,
}

impl PerkCensus {
    pub fn applied_entry_points(&self) -> usize {
        self.by_entry.values().filter(|e| e.2).map(|e| e.1).sum()
    }
}

fn rust_files(dir: &Path, out: &mut Vec<std::path::PathBuf>, depth: usize) {
    let Ok(rd) = std::fs::read_dir(dir) else {
        return;
    };
    for e in rd.flatten() {
        let p = e.path();
        if p.is_dir() && depth < 6 {
            rust_files(&p, out, depth + 1);
        } else if p.extension().is_some_and(|x| x == "rs") {
            out.push(p);
        }
    }
}

/// `pub const NAME: u8 = N;` lines inside `perks.rs`'s entry module.
fn entry_constants(perks_rs: &str) -> Vec<(String, u8)> {
    let mut out = Vec::new();
    for line in perks_rs.lines() {
        let l = line.trim();
        if let Some(rest) = l.strip_prefix("pub const ") {
            if let Some((name, tail)) = rest.split_once(": u8 = ") {
                if let Ok(n) = tail.trim_end_matches(';').trim().parse::<u8>() {
                    if name
                        .chars()
                        .all(|c| c.is_ascii_uppercase() || c == '_' || c.is_ascii_digit())
                    {
                        out.push((name.to_string(), n));
                    }
                }
            }
        }
    }
    out
}

pub fn perk_census(order: &LoadOrder, repo: Option<&Path>) -> PerkCensus {
    let mut c = PerkCensus::default();
    // An entry point is "applied" when its constant is named somewhere besides its definition.
    let mut applied: HashMap<u8, bool> = HashMap::new();
    if let Some(root) = repo {
        let perks_rs =
            std::fs::read_to_string(root.join("crates/world/src/perks.rs")).unwrap_or_default();
        let consts = entry_constants(&perks_rs);
        let mut files = Vec::new();
        for sub in ["crates/world/src", "viewer/src"] {
            rust_files(&root.join(sub), &mut files, 0);
        }
        let texts: Vec<String> = files
            .iter()
            .filter_map(|f| std::fs::read_to_string(f).ok())
            .collect();
        for (name, n) in consts {
            let uses: usize = texts.iter().map(|t| t.matches(name.as_str()).count()).sum();
            // One hit is the definition itself.
            if uses > 1 {
                applied.insert(n, true);
            }
        }
    }
    for rr in order.records_of_type(FourCC::new(b"PERK")) {
        c.perks += 1;
        for e in world::perks::entry_points(order, rr.form_id) {
            c.entry_points += 1;
            let name = world::perks::ENTRY_POINT_NAMES
                .get(usize::from(e.entry))
                .copied()
                .unwrap_or("?")
                .to_string();
            let slot = c.by_entry.entry(e.entry).or_insert((
                name,
                0,
                applied.get(&e.entry).copied().unwrap_or(false),
            ));
            slot.1 += 1;
        }
    }
    c
}

/// Script functions that already have a translated exe handler in the tree:
/// lower case name → (address, file). Found from the `// Translated from ADDR`
/// line above a doc comment naming `Script::<Name>Function`.
pub fn translations(repo: &Path) -> HashMap<String, (String, String)> {
    let mut files = Vec::new();
    rust_files(&repo.join("crates/engine-fallout/src"), &mut files, 0);
    let mut out = HashMap::new();
    for f in files {
        let Ok(text) = std::fs::read_to_string(&f) else {
            continue;
        };
        let mut addr: Option<String> = None;
        for line in text.lines() {
            if let Some(rest) = line.strip_prefix("// Translated from ") {
                addr = rest.split_whitespace().next().map(str::to_string);
            } else if let (Some(a), Some(i)) = (&addr, line.find("`Script::")) {
                let tail = &line[i + 9..];
                if let Some(end) = tail.find("Function`") {
                    let name = tail[..end].to_ascii_lowercase();
                    out.entry(name).or_insert((
                        a.clone(),
                        f.strip_prefix(repo)
                            .unwrap_or(&f)
                            .to_string_lossy()
                            .replace('\\', "/"),
                    ));
                }
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_entry_constants() {
        let c = entry_constants("    pub const RELOAD_SPEED: u8 = 37;\n    pub const x: u8 = 3;\n    pub const A_B: u16 = 4;\n");
        assert_eq!(c, vec![("RELOAD_SPEED".to_string(), 37)]);
    }

    #[test]
    fn finds_translated_handlers() {
        let root = std::env::temp_dir().join("nv_cov_translations_test");
        let dir = root.join("crates/engine-fallout/src/units");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("a.rs"), "// Translated from 005de700 (decompiled)\n/// `Script::AddItemToLeveledListFunction` (Xbox PDB): x\npub fn f() {}\n").unwrap();
        let t = translations(&root);
        assert_eq!(t.get("additemtoleveledlist").unwrap().0, "005de700");
    }
}

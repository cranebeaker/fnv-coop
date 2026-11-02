//! Data folder commands that look at game files: `find` and `check-assets`.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::io::Write;
use std::panic::{self, AssertUnwindSafe};
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::time::Instant;

use assets::{
    archive_list_from, default_ini_candidates, mesh_path, normalize_path, texture_path,
    ArchiveList, ArchiveReason, ArchiveSettings, Assets, Source,
};
use esm::{FormId, FourCC, LoadOrder};
use nif::Nif;

use crate::fmt::{human_bytes, thousands};
use crate::{CliError, Options};

/// Record subrecords that hold model paths (world models, and the male/
/// female variants on armor and weapons).
const MODEL_FIELDS: [&[u8; 4]; 4] = [b"MODL", b"MOD2", b"MOD3", b"MOD4"];

/// Record types that never carry model paths; skipping them saves most of
/// the decoding time (placed objects alone are two thirds of all records).
const NO_MODEL_TYPES: [&[u8; 4]; 9] = [
    b"REFR", b"ACHR", b"ACRE", b"PGRE", b"PMIS", b"LAND", b"NAVM", b"INFO", b"DIAL",
];

const TEXTURE_SLOTS: [&str; 6] = [
    "diffuse",
    "normal map",
    "glow map",
    "height map",
    "environment map",
    "environment mask",
];

/// Opens the game's files using its own archive list (from `--ini`, the
/// user's Fallout.ini, or the install's Fallout_default.ini).
pub fn open(
    order: &LoadOrder,
    data_dir: &Path,
    options: &Options,
) -> Result<(Assets, ArchiveList), CliError> {
    let list = match &options.ini {
        Some(ini) => {
            if !ini.is_file() {
                return Err(CliError::Open {
                    path: ini.display().to_string(),
                    message: "no such file".into(),
                });
            }
            archive_list_from(std::slice::from_ref(ini))
        }
        None => archive_list_from(&default_ini_candidates(data_dir)),
    };
    let plugins: Vec<String> = order.plugins().iter().map(|p| p.name.clone()).collect();
    let mut files = assets::default_settings_files(data_dir);
    files.extend(options.ini.iter().cloned());
    let settings =
        ArchiveSettings::from_ini(&assets::IniSettings::load(&files), list.names.clone());
    let assets = Assets::open_with_settings(data_dir, &plugins, &settings)
        .map_err(|e| CliError::NotFound(e.to_string()))?;
    for warning in assets.warnings() {
        eprintln!("warning: {warning}");
    }
    Ok((assets, list))
}

fn size_of(source: &Source) -> String {
    match source {
        Source::Loose(path) => std::fs::metadata(path)
            .map(|m| human_bytes(m.len()))
            .unwrap_or_else(|_| "?".into()),
        Source::Archive { entry, .. } => {
            let size = human_bytes(u64::from(entry.stored_size));
            if entry.compressed {
                format!("{size} compressed")
            } else {
                size
            }
        }
    }
}

/// " (in X.bsa, which the game doesn't load)" when a missing file is there.
fn unused_note(assets: &Assets, path: &str) -> String {
    let found = assets.in_unused_archives(path);
    if found.is_empty() {
        String::new()
    } else {
        format!("  [in {}, which the game doesn't load]", found.join(", "))
    }
}

/// Shows where the game would load a file from, and any copies it hides.
pub fn find(out: &mut impl Write, assets: &Assets, wanted: &str) -> Result<(), CliError> {
    let path = normalize_path(wanted);
    let versions = assets.versions(&path);
    let Some(used) = versions.first() else {
        let name = path.rsplit('\\').next().unwrap_or(&path).to_string();
        let mut similar: Vec<&str> = assets
            .paths()
            .filter(|p| p.rsplit('\\').next() == Some(name.as_str()))
            .collect();
        similar.sort();
        similar.truncate(5);
        let unused = assets.in_unused_archives(&path);
        let mut hint = if unused.is_empty() {
            String::new()
        } else {
            format!(
                " It's in {}, which the game doesn't load.",
                unused.join(", ")
            )
        };
        if !similar.is_empty() {
            hint.push_str(&format!(" Files with that name: {}", similar.join(", ")));
        }
        return Err(CliError::NotFound(format!(
            "the game can't see '{path}' in the Data folder or the archives it loads.{hint}"
        )));
    };
    writeln!(out, "{path}")?;
    writeln!(out, "  used:       {} ({})", used.describe(), size_of(used))?;
    if let Source::Loose(p) = used {
        writeln!(out, "              {}", p.display())?;
    }
    for hidden in &versions[1..] {
        writeln!(
            out,
            "  overrides:  {} ({})",
            hidden.describe(),
            size_of(hidden)
        )?;
    }
    for name in assets.in_unused_archives(&path) {
        writeln!(out, "  also in:    {name} (not loaded by the game)")?;
    }
    Ok(())
}

#[derive(Default)]
struct MeshScan {
    /// texture path -> (slot, a mesh that uses it)
    textures: HashMap<String, (usize, String)>,
    unreadable: Vec<(String, String)>,
}

impl MeshScan {
    fn merge(&mut self, other: MeshScan) {
        for (k, v) in other.textures {
            self.textures.entry(k).or_insert(v);
        }
        self.unreadable.extend(other.unreadable);
    }
}

fn scan_mesh(assets: &Assets, path: &str, scan: &mut MeshScan) {
    let result = panic::catch_unwind(AssertUnwindSafe(|| -> Result<nif::Scene, String> {
        let bytes = assets
            .read(path)
            .map_err(|e| e.to_string())?
            .ok_or("vanished while reading")?;
        Nif::parse(bytes)
            .and_then(|n| n.scene())
            .map_err(|e| e.to_string())
    }));
    match result {
        Ok(Ok(scene)) => {
            for mesh in &scene.meshes {
                for (slot, texture) in mesh.textures.iter().enumerate() {
                    if texture.trim().is_empty() {
                        continue;
                    }
                    scan.textures
                        .entry(texture_path(texture))
                        .or_insert_with(|| (slot, path.to_string()));
                }
            }
        }
        Ok(Err(message)) => scan.unreadable.push((path.to_string(), message)),
        Err(_) => scan
            .unreadable
            .push((path.to_string(), "the reader panicked on this file".into())),
    }
}

/// Reads the given meshes in parallel, collecting texture use.
fn scan_meshes(assets: &Assets, meshes: &[&str]) -> MeshScan {
    let next = AtomicUsize::new(0);
    let total = Mutex::new(MeshScan::default());
    let threads = std::thread::available_parallelism()
        .map_or(4, |n| n.get())
        .min(16);
    let previous_hook = panic::take_hook();
    panic::set_hook(Box::new(|_| {}));
    std::thread::scope(|scope| {
        for _ in 0..threads {
            scope.spawn(|| {
                let mut local = MeshScan::default();
                loop {
                    let i = next.fetch_add(1, Ordering::Relaxed);
                    let Some(path) = meshes.get(i) else { break };
                    scan_mesh(assets, path, &mut local);
                }
                total.lock().unwrap_or_else(|p| p.into_inner()).merge(local);
            });
        }
    });
    panic::set_hook(previous_hook);
    total.into_inner().unwrap_or_else(|p| p.into_inner())
}

/// Model paths used by the winning records, each with one record using it.
/// A record that uses a model: a description and its form ID.
/// The records that use a model: a description of the first, and every
/// user's type and form ID.
struct ModelUser {
    description: String,
    users: Vec<(FourCC, FormId)>,
}

/// Record types that only appear in the game by being placed in the world.
/// (Items such as weapons and armor also show up in inventories and
/// containers, so their placement count says nothing about use.)
const WORLD_OBJECT_TYPES: [&[u8; 4]; 13] = [
    b"STAT", b"MSTT", b"SCOL", b"DOOR", b"FURN", b"ACTI", b"TACT", b"TREE", b"LIGH", b"CONT",
    b"TERM", b"ADDN", b"PWAT",
];

impl ModelUser {
    /// Placements across all users, or `None` when some user can appear in
    /// the game without being placed.
    fn placements(&self, counts: &HashMap<FormId, usize>) -> Option<usize> {
        let world_only = self
            .users
            .iter()
            .all(|(kind, _)| WORLD_OBJECT_TYPES.iter().any(|t| *kind == FourCC::new(t)));
        world_only.then(|| {
            self.users
                .iter()
                .map(|(_, id)| counts.get(id).copied().unwrap_or(0))
                .sum()
        })
    }

    /// The first user, plus how many others there are.
    fn describe(&self) -> String {
        match self.users.len() {
            0 | 1 => self.description.clone(),
            n => format!(
                "{} and {} other record{}",
                self.description,
                n - 1,
                if n == 2 { "" } else { "s" }
            ),
        }
    }

    fn note(&self, counts: &HashMap<FormId, usize>) -> String {
        match self.placements(counts) {
            None => String::new(),
            Some(0) => ", never placed in the world".into(),
            Some(1) => ", placed once in the world".into(),
            Some(n) => format!(", placed {} times in the world", thousands(n)),
        }
    }
}

/// Record types that place objects in the world, and the subrecord naming
/// the placed object.
const PLACEMENT_TYPES: [&[u8; 4]; 3] = [b"REFR", b"ACHR", b"ACRE"];

/// How many times each of `wanted` is placed in the world.
fn count_placements(
    order: &LoadOrder,
    wanted: &HashSet<FormId>,
) -> Result<HashMap<FormId, usize>, CliError> {
    let mut counts = HashMap::new();
    if wanted.is_empty() {
        return Ok(counts);
    }
    for kind in PLACEMENT_TYPES {
        for rr in order.records_of_type(FourCC::new(kind)) {
            let record = rr.record()?;
            let Some(name) = record.get(FourCC::new(b"NAME")) else {
                continue;
            };
            let Some(bytes) = name.data.get(0..4) else {
                continue;
            };
            let local = FormId(u32::from_le_bytes(bytes.try_into().expect("4 bytes")));
            let base = rr.plugin.to_global(local);
            if wanted.contains(&base) {
                *counts.entry(base).or_default() += 1;
            }
        }
    }
    Ok(counts)
}

fn models_used_by_records(order: &LoadOrder) -> Result<BTreeMap<String, ModelUser>, CliError> {
    let mut models = BTreeMap::new();
    for (kind, _) in order.type_counts() {
        if NO_MODEL_TYPES.iter().any(|t| kind == FourCC::new(t)) {
            continue;
        }
        for rr in order.records_of_type(kind) {
            let record = rr.record()?;
            for sub in record
                .subrecords
                .iter()
                .filter(|s| MODEL_FIELDS.iter().any(|f| s.kind == FourCC::new(f)))
            {
                let model = sub.zstring();
                if !model.to_ascii_lowercase().ends_with(".nif") {
                    continue;
                }
                let entry = models.entry(mesh_path(&model)).or_insert_with(|| {
                    let editor_id = record.editor_id().unwrap_or_default();
                    ModelUser {
                        description: format!("{kind} {} {editor_id}", rr.form_id)
                            .trim_end()
                            .to_string(),
                        users: Vec::new(),
                    }
                });
                if !entry.users.contains(&(kind, rr.form_id)) {
                    entry.users.push((kind, rr.form_id));
                }
            }
        }
    }
    Ok(models)
}

pub fn check(
    out: &mut impl Write,
    order: &LoadOrder,
    assets: &Assets,
    list: &ArchiveList,
    options: &Options,
) -> Result<(), CliError> {
    let show = options.limit.unwrap_or(20);
    writeln!(out, "Archive list from: {}", list.source)?;
    writeln!(
        out,
        "Archives the game loads, the one whose copy of a file wins first:"
    )?;
    let width = assets
        .archives()
        .iter()
        .map(|a| a.name.len())
        .max()
        .unwrap_or(0);
    for loaded in assets.by_priority() {
        let why = match &loaded.reason {
            ArchiveReason::Default => String::new(),
            ArchiveReason::Patch => "  (the game's patch archive)".to_string(),
            ArchiveReason::Plugin(p) => format!("  (for {p})"),
        };
        let count = loaded.archive.files().len();
        let files = if count == 1 { "file " } else { "files" };
        let line = format!(
            "  {:<width$}  {:>9} {files}{why}",
            loaded.name,
            thousands(count)
        );
        writeln!(out, "{}", line.trim_end())?;
    }
    if !assets.unused_archives().is_empty() {
        writeln!(
            out,
            "Not loaded (not in the list, and no active plugin uses them): {}",
            assets.unused_archives().join(", ")
        )?;
    }
    writeln!(out, "Loose files:    {}", thousands(assets.loose_count()))?;
    writeln!(out, "Files visible:  {}", thousands(assets.len()))?;

    let started = Instant::now();
    let models = models_used_by_records(order)?;
    let missing_models: Vec<(&String, &ModelUser)> =
        models.iter().filter(|(m, _)| !assets.contains(m)).collect();

    // By default only meshes the game actually uses are checked; archives
    // also carry leftover meshes (some from Fallout 3) whose textures were
    // never shipped.
    let all_meshes: Vec<&str> = assets.paths().filter(|p| p.ends_with(".nif")).collect();
    let used_meshes: Vec<&str> = models
        .keys()
        .map(String::as_str)
        .filter(|m| assets.contains(m))
        .collect();
    let to_scan = if options.all_meshes {
        &all_meshes
    } else {
        &used_meshes
    };
    let scan = scan_meshes(assets, to_scan);
    let mut missing_textures: Vec<(&String, &(usize, String))> = scan
        .textures
        .iter()
        .filter(|(t, _)| !assets.contains(t))
        .collect();
    missing_textures.sort();

    writeln!(out)?;
    writeln!(
        out,
        "{:<31}{:>7} distinct, {:>7} found, {} missing",
        "Models used by records:",
        thousands(models.len()),
        thousands(models.len() - missing_models.len()),
        thousands(missing_models.len())
    )?;
    let label = if options.all_meshes {
        "Textures used by all meshes:"
    } else {
        "Textures used by those models:"
    };
    writeln!(
        out,
        "{label:<31}{:>7} distinct, {:>7} found, {} missing",
        thousands(scan.textures.len()),
        thousands(scan.textures.len() - missing_textures.len()),
        thousands(missing_textures.len())
    )?;
    writeln!(
        out,
        "Meshes read: {}, unreadable: {}   (in {:.1} s)",
        thousands(to_scan.len()),
        thousands(scan.unreadable.len()),
        started.elapsed().as_secs_f64()
    )?;
    if !options.all_meshes {
        writeln!(
            out,
            "Meshes no record uses (not checked; add --all-meshes to include them): {}",
            thousands(all_meshes.len().saturating_sub(used_meshes.len()))
        )?;
    }

    // Whether the records behind missing files are ever placed in the world
    // tells real gaps apart from unused leftovers.
    let mut wanted: HashSet<FormId> = HashSet::new();
    for (_, user) in &missing_models {
        wanted.extend(user.users.iter().map(|(_, id)| *id));
    }
    for (_, (_, mesh)) in &missing_textures {
        if let Some(user) = models.get(mesh) {
            wanted.extend(user.users.iter().map(|(_, id)| *id));
        }
    }
    let placements = count_placements(order, &wanted)?;
    let unused = |user: &ModelUser| user.placements(&placements) == Some(0);
    let unplaced_models = missing_models.iter().filter(|(_, u)| unused(u)).count();
    let unplaced_textures = missing_textures
        .iter()
        .filter(|(_, (_, mesh))| models.get(mesh).is_some_and(unused))
        .count();
    if !missing_models.is_empty() || !missing_textures.is_empty() {
        writeln!(
            out,
            "Missing files whose objects are never placed in the world: {} of {} models, {} of {} textures",
            thousands(unplaced_models),
            thousands(missing_models.len()),
            thousands(unplaced_textures),
            thousands(missing_textures.len())
        )?;
    }

    if !missing_models.is_empty() {
        let traced = missing_models
            .iter()
            .filter(|(m, _)| !assets.in_unused_archives(m).is_empty())
            .count();
        writeln!(out, "\nMissing models (first {show}):")?;
        if traced > 0 {
            writeln!(
                out,
                "  ({traced} of {} {} in archives the game doesn't load)",
                missing_models.len(),
                if traced == 1 { "is" } else { "are" }
            )?;
        }
        for (model, user) in missing_models.iter().take(show) {
            writeln!(
                out,
                "  {model}{}\n      used by {}{}",
                unused_note(assets, model),
                user.describe(),
                user.note(&placements)
            )?;
        }
    }
    if !missing_textures.is_empty() {
        writeln!(out, "\nMissing textures (first {show}):")?;
        for (texture, (slot, mesh)) in missing_textures.iter().take(show) {
            let slot = TEXTURE_SLOTS
                .get(*slot)
                .map_or_else(|| format!("slot {slot}"), |s| s.to_string());
            let user = models.get(mesh).map_or_else(String::new, |user| {
                format!(", used by {}{}", user.describe(), user.note(&placements))
            });
            writeln!(
                out,
                "  {texture}{}\n      {slot} in {mesh}{user}",
                unused_note(assets, texture)
            )?;
        }
    }
    if !scan.unreadable.is_empty() {
        let mut unreadable = scan.unreadable;
        unreadable.sort();
        writeln!(out, "\nMeshes that couldn't be read (first {show}):")?;
        for (path, message) in unreadable.iter().take(show) {
            writeln!(out, "  {path}\n      {message}")?;
        }
    }
    Ok(())
}

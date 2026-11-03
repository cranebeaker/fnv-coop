//! Opening the game data, and the quest sets the reports are about.

use std::path::{Path, PathBuf};

use esm::{ActivePlugins, FormId, LoadOrder};

pub struct Data {
    pub order: LoadOrder,
    pub dir: PathBuf,
    /// The folder holds meshes (loose or in a BSA): the load sweep can go further.
    pub has_meshes: bool,
}

/// The data folder: `--data`, else `FNV_ESM_DIR`.
pub fn locate(arg: Option<&str>) -> Result<PathBuf, String> {
    match arg
        .map(str::to_string)
        .or_else(|| std::env::var("FNV_ESM_DIR").ok())
    {
        Some(d) => cellview::find_data_folder(Path::new(&d)).map_err(|e| e.to_string()),
        None => Err("no data folder: pass --data DIR or set FNV_ESM_DIR".into()),
    }
}

pub fn open(dir: &Path) -> Result<Data, String> {
    let order = LoadOrder::from_data_dir(dir, &ActivePlugins::OfficialOnly)
        .map_err(|e| format!("cannot read the load order in {}: {e}", dir.display()))?;
    let has_meshes = dir.join("Meshes").is_dir()
        || std::fs::read_dir(dir)
            .map(|d| {
                d.flatten().any(|f| {
                    let n = f.file_name().to_string_lossy().to_ascii_lowercase();
                    n.contains("meshes") && n.ends_with(".bsa")
                })
            })
            .unwrap_or(false);
    Ok(Data {
        order,
        dir: dir.to_path_buf(),
        has_meshes,
    })
}

/// A quest set the walk reports on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Line {
    /// The main questline: VMQ, VCG, VMS series, the four factions, endings.
    Main,
    /// The Mojave side-story series (`VMS`), which the owner counts with the main game.
    Vms,
    Dlc(&'static str),
}

impl Line {
    pub fn label(self) -> String {
        match self {
            Line::Main => "main (base game)".into(),
            Line::Vms => "VMS series".into(),
            Line::Dlc(n) => format!("DLC {n}"),
        }
    }
}

/// Editor-id prefixes of the base game's main story. Measured from
/// `FalloutNV.esm`'s quest list: the opening and Goodsprings (VCG), the
/// Mojave Main Quest, House, Yes-Man and the end states (VMQ), character
/// generation (CG), the faction reaction quests (VEFR).
pub const MAIN_PREFIXES: &[&str] = &["VMQ", "VCG", "CG0", "CGTutorial", "VEFR"];

/// Which line a quest belongs to, from its plugin and editor id.
pub fn line_of(plugin: &str, editor: &str) -> Option<Line> {
    let p = plugin.to_ascii_lowercase();
    match p.as_str() {
        "deadmoney.esm" if editor.to_ascii_uppercase().contains("MQ") => {
            Some(Line::Dlc("Dead Money"))
        }
        "honesthearts.esm" if editor.to_ascii_uppercase().contains("MQ") => {
            Some(Line::Dlc("Honest Hearts"))
        }
        "oldworldblues.esm" if editor.to_ascii_uppercase().contains("MQ") => {
            Some(Line::Dlc("Old World Blues"))
        }
        "lonesomeroad.esm" if editor.to_ascii_uppercase().contains("MQ") => {
            Some(Line::Dlc("Lonesome Road"))
        }
        "falloutnv.esm" if editor.starts_with("VMS") => Some(Line::Vms),
        "falloutnv.esm" if MAIN_PREFIXES.iter().any(|m| editor.starts_with(m)) => Some(Line::Main),
        _ => None,
    }
}

/// Every quest as (form id, editor id, plugin).
pub fn quests(order: &LoadOrder) -> Vec<(FormId, String, String)> {
    order
        .records_of_type(esm::FourCC::new(b"QUST"))
        .map(|rr| {
            (
                rr.form_id,
                rr.editor_id().ok().flatten().unwrap_or_default(),
                rr.plugin.name.clone(),
            )
        })
        .collect()
}

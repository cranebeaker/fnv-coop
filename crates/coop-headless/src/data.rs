//! Game data without a renderer: the load order, the script cache and the
//! cell facts a scenario needs (arrival point, who stands in it).

use std::path::{Path, PathBuf};

use esm::{ActivePlugins, FormId, LoadOrder};
use world::scripting::ScriptCache;

/// The folder of ESMs (and, on the test PC, the rest of `Data`).
pub struct Data {
    /// Records.
    pub order: LoadOrder,
    /// Parsed scripts and quests.
    pub scripts: ScriptCache,
    /// The folder.
    pub dir: PathBuf,
    /// The folder holds meshes (a BSA or loose files): collision and
    /// navigation could be read. The simulator never reads them.
    pub has_meshes: bool,
}

/// The facts of one cell.
#[derive(Clone, Debug)]
pub struct Place {
    /// The cell record.
    pub cell: FormId,
    /// The worldspace of an exterior cell (`None` for an interior): the
    /// viewer starts an exterior cell in its worldspace.
    pub world: Option<FormId>,
    /// Where the player arrives (feet).
    pub spawn: [f32; 3],
    /// Arrival heading, radians.
    pub heading: f32,
    /// Placed people (also those a script may enable later).
    pub actors: Vec<(FormId, FormId)>,
}

impl Data {
    /// Opens a folder; the reason is a `needs_assets` message when the base
    /// game is missing.
    pub fn open(dir: &Path) -> Result<Data, String> {
        if !dir.is_dir() {
            return Err(format!("data folder {} does not exist", dir.display()));
        }
        let order = LoadOrder::from_data_dir(dir, &ActivePlugins::OfficialOnly)
            .map_err(|e| format!("cannot read the load order in {}: {e}", dir.display()))?;
        let has_meshes = dir.join("Meshes").is_dir()
            || std::fs::read_dir(dir)
                .map(|d| {
                    d.flatten().any(|f| {
                        f.file_name()
                            .to_string_lossy()
                            .to_ascii_lowercase()
                            .contains("meshes")
                            && f.file_name().to_string_lossy().ends_with(".bsa")
                    })
                })
                .unwrap_or(false);
        Ok(Data {
            order,
            scripts: ScriptCache::default(),
            dir: dir.to_path_buf(),
            has_meshes,
        })
    }

    /// The data folder: `--data`, else `FNV_ESM_DIR`.
    pub fn locate(arg: Option<&str>) -> Result<PathBuf, String> {
        match arg
            .map(str::to_string)
            .or_else(|| std::env::var("FNV_ESM_DIR").ok())
        {
            Some(d) => Ok(PathBuf::from(d)),
            None => Err("no data folder: pass --data DIR or set FNV_ESM_DIR".into()),
        }
    }

    /// A form by editor id or 8-digit hex id (`player` is the player).
    pub fn form(&self, word: &str) -> Option<FormId> {
        if word.eq_ignore_ascii_case("player") {
            return Some(world::dialogue::PLAYER_REF);
        }
        self.order
            .form_by_editor_id(word)
            .or_else(|| u32::from_str_radix(word, 16).ok().map(FormId))
    }

    /// The base record of a placed person (`ACHR` / `ACRE`), or `None` for any other reference.
    pub fn actor_base(&self, r: FormId) -> Option<FormId> {
        let rr = self.order.get(r)?;
        let kind = rr.entry.header.kind;
        if kind.as_bytes() != b"ACHR" && kind.as_bytes() != b"ACRE" {
            return None;
        }
        let rec = rr.record().ok()?;
        let name = rec.get(esm::sig::NAME)?;
        let raw = name.data.get(..4)?;
        Some(
            rr.plugin
                .to_global(FormId(u32::from_le_bytes([raw[0], raw[1], raw[2], raw[3]]))),
        )
    }

    /// Looks up a cell by editor id.
    pub fn place(&self, name: &str) -> Result<Place, String> {
        let cell = world::find_cells(&self.order, name)
            .ok()
            .and_then(|v| v.into_iter().next())
            .ok_or_else(|| format!("cell {name} is not in this load order"))?;
        let loaded =
            world::load_cell(&self.order, cell).map_err(|e| format!("cell {name}: {e}"))?;
        let (spawn, heading) = loaded
            .arrivals
            .first()
            .map_or(([0.0; 3], 0.0), |a| (a.position, a.rotation[2]));
        let mut actors: Vec<(FormId, FormId)> =
            loaded.actors.iter().map(|a| (a.form_id, a.base)).collect();
        // people a script may enable later are left out of the first load
        let people = |t: esm::FourCC| t.as_bytes() == b"ACHR" || t.as_bytes() == b"ACRE";
        actors.extend(
            loaded
                .left_out
                .iter()
                .filter(|l| people(l.record_type) && l.reason.starts_with("disabled"))
                .filter_map(|l| l.base.map(|b| (l.form_id, b))),
        );
        Ok(Place {
            cell,
            world: self.world_of_cell(cell),
            spawn,
            heading,
            actors,
        })
    }

    /// The worldspace an exterior cell belongs to (`None` for an interior).
    pub fn world_of_cell(&self, cell: FormId) -> Option<FormId> {
        let rr = self.order.get(cell)?;
        (rr.entry.header.kind.as_bytes() == b"CELL")
            .then(|| self.order.world_of(&rr))
            .flatten()
    }
}

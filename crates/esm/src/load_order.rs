//! Combining the game's master file, DLC and mods into one view.
//!
//! Every plugin numbers its form IDs relative to its own master list: the top
//! byte is an index into that list, and one past the end means "this file".
//! A load order renumbers them so the top byte is the plugin's position in
//! the load order (what the in-game console shows), and resolves overrides:
//! when several plugins contain the same record, the one loaded last wins.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use std::time::SystemTime;

use crate::error::{Error, Result};
use crate::plugin::{Plugin, RecordEntry};
use crate::record::Record;
use crate::text;
use crate::types::{sig, FormId, FourCC};

/// The game's main master file, always loaded first.
pub const MAIN_MASTER: &str = "FalloutNV.esm";

/// The official master files: the base game, the five DLCs and the four
/// Courier's Stash packs.
pub const OFFICIAL_FILES: [&str; 10] = [
    "FalloutNV.esm",
    "DeadMoney.esm",
    "HonestHearts.esm",
    "OldWorldBlues.esm",
    "LonesomeRoad.esm",
    "GunRunnersArsenal.esm",
    "ClassicPack.esm",
    "MercenaryPack.esm",
    "TribalPack.esm",
    "CaravanPack.esm",
];

/// Record types for objects placed in cells: references to base objects,
/// NPCs, creatures, and placed grenades and missiles.
pub const PLACED_TYPES: [FourCC; 5] = [sig::REFR, sig::ACHR, sig::ACRE, sig::PGRE, sig::PMIS];

/// Form IDs have one byte for the load-order index, and 0xFF is reserved
/// for objects created while playing.
const MAX_PLUGINS: usize = 255;

/// Which plugins are active.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActivePlugins {
    /// FalloutNV.esm plus whichever official DLC and pack files are present.
    OfficialOnly,
    /// The names listed in a `plugins.txt` file. FalloutNV.esm is always
    /// loaded, listed or not.
    List(Vec<String>),
}

/// One plugin in a load order.
pub struct LoadedPlugin {
    /// File name, as found on disk.
    pub name: String,
    pub path: Option<PathBuf>,
    pub plugin: Plugin,
    /// Position in the load order: the top byte of this plugin's new form IDs.
    pub load_index: u8,
    /// Maps a local mod index (position in this plugin's master list, or the
    /// list's length for the plugin itself) to a load-order index.
    index_map: Vec<u8>,
    /// Records decoded once and kept ([`RecordRef::record_shared`]), by
    /// their index in the plugin.
    decoded: Vec<std::sync::OnceLock<std::sync::Arc<Record>>>,
}

impl LoadedPlugin {
    /// Converts a form ID stored in this plugin's data (a reference to
    /// another form) to its load-order form ID. Translated from the
    /// game's form ID adjustment (`00485d50`, decompiled, FalloutNV.exe
    /// 1.4.0.525): the top byte indexes the master list, and one at or
    /// past its end means the plugin itself (`TESFile::GetIndexFile`
    /// `00471a10` finds no master); the engine's own forms, IDs 1 to
    /// 0x7FF (`TESForm::IsDefaultForm` `00484b40`), are kept as they are.
    pub fn to_global(&self, local: FormId) -> FormId {
        if is_default_form(local) {
            return local;
        }
        let i = usize::from(local.mod_index()).min(self.index_map.len() - 1);
        FormId((u32::from(self.index_map[i]) << 24) | local.object_id())
    }

    /// Converts the form ID in one of this plugin's record headers to its
    /// load-order form ID: as [`LoadedPlugin::to_global`], except that a
    /// record whose object ID is 1 to 0x7FF (an engine form a plugin
    /// changes) is the base game's whatever its top byte
    /// (`TESFile::ReadFormHeader` `00472bc0`).
    pub fn record_id(&self, local: FormId) -> FormId {
        if is_default_form(FormId(local.object_id())) {
            return FormId(local.object_id());
        }
        self.to_global(local)
    }
}

/// A reference to one version of a record inside a load order.
#[derive(Clone, Copy)]
pub struct RecordRef<'a> {
    pub plugin_index: usize,
    pub plugin: &'a LoadedPlugin,
    pub entry: &'a RecordEntry,
    /// The load-order form ID.
    pub form_id: FormId,
    /// Its index among the plugin's records.
    pub index: usize,
}

impl RecordRef<'_> {
    pub fn record(&self) -> Result<Record> {
        self.plugin.plugin.record(self.entry)
    }

    pub fn editor_id(&self) -> Result<Option<String>> {
        self.plugin.plugin.editor_id_of(self.entry)
    }
}

impl<'a> RecordRef<'a> {
    /// The record decoded once and kept for the load order's life: for
    /// what's asked for every frame (the same as [`RecordRef::record`];
    /// records don't change once loaded).
    pub fn record_shared(&self) -> Result<std::sync::Arc<Record>> {
        let slot = &self.plugin.decoded[self.index];
        if let Some(r) = slot.get() {
            return Ok(r.clone());
        }
        let record = std::sync::Arc::new(self.record()?);
        Ok(slot.get_or_init(|| record).clone())
    }

    /// The data of the record's first subrecord of a type, without decoding
    /// the rest ([`crate::Plugin::subrecord_of`]).
    pub fn subrecord(&self, kind: FourCC) -> Result<Option<std::borrow::Cow<'a, [u8]>>> {
        self.plugin.plugin.subrecord_of(self.entry, kind)
    }
}

/// (plugin index, record index within that plugin)
type Slot = (u16, u32);

/// Several plugins combined, with overrides resolved.
pub struct LoadOrder {
    plugins: Vec<LoadedPlugin>,
    /// File name for each load-order index.
    slot_names: Vec<String>,
    /// The winning (last-loaded) version of every record.
    winners: HashMap<FormId, Slot>,
    /// Every version, in load order, of records defined more than once.
    history: HashMap<FormId, Vec<Slot>>,
    warnings: Vec<String>,
    single: bool,
    /// Every record version inside each cell's child groups, by cell; built
    /// the first time it's needed.
    by_cell: OnceLock<HashMap<FormId, Vec<Slot>>>,
    /// Every dialogue line (`INFO`) version by topic; built when needed.
    by_topic: OnceLock<HashMap<FormId, Vec<Slot>>>,
    /// Every winning record by lower-case editor ID; built when needed.
    by_editor_id: OnceLock<HashMap<String, FormId>>,
}

impl LoadOrder {
    /// Wraps one plugin on its own, without loading its masters. Form IDs
    /// keep their local numbering, and records it overrides show their
    /// master's name as their origin.
    pub fn single(name: impl Into<String>, path: Option<PathBuf>, plugin: Plugin) -> Result<Self> {
        let name = name.into();
        let masters = plugin.header().masters.clone();
        if masters.len() >= MAX_PLUGINS {
            return Err(Error::TooManyPlugins {
                count: masters.len() + 1,
            });
        }
        let load_index = masters.len() as u8;
        let mut slot_names = masters;
        slot_names.push(name.clone());
        let decoded = (0..plugin.records().len())
            .map(|_| std::sync::OnceLock::new())
            .collect();
        let loaded = LoadedPlugin {
            decoded,
            name,
            path,
            plugin,
            load_index,
            index_map: (0..=load_index).collect(),
        };
        Ok(Self::build(vec![loaded], slot_names, Vec::new(), true))
    }

    /// Combines plugins already in load order. A plugin's masters are
    /// found by name anywhere in the list, as the game finds them
    /// (`TESFile` master list, `00471870`): one that loads after the plugin
    /// still works, with a warning.
    pub fn from_plugins(list: Vec<(String, Option<PathBuf>, Plugin)>) -> Result<Self> {
        if list.len() > MAX_PLUGINS {
            return Err(Error::TooManyPlugins { count: list.len() });
        }
        let slot_names: Vec<String> = list.iter().map(|(name, ..)| name.clone()).collect();
        let mut warnings = Vec::new();
        let mut loaded = Vec::with_capacity(list.len());
        for (i, (name, path, plugin)) in list.into_iter().enumerate() {
            let mut index_map = Vec::with_capacity(plugin.header().masters.len() + 1);
            for master in &plugin.header().masters {
                let position = slot_names
                    .iter()
                    .position(|loaded_name| loaded_name.eq_ignore_ascii_case(master))
                    .ok_or_else(|| Error::MissingMaster {
                        plugin: name.clone(),
                        master: master.clone(),
                    })?;
                if position > i {
                    warnings.push(format!(
                        "{name} loads before its master {master}, so {master}'s versions of \
                         records they share win"
                    ));
                }
                index_map.push(position as u8);
            }
            index_map.push(i as u8);
            let decoded = (0..plugin.records().len())
                .map(|_| std::sync::OnceLock::new())
                .collect();
            loaded.push(LoadedPlugin {
                decoded,
                name,
                path,
                plugin,
                load_index: i as u8,
                index_map,
            });
        }
        warnings.extend(master_size_warnings(&loaded));
        Ok(Self::build(loaded, slot_names, warnings, false))
    }

    /// Loads the active plugins from a game's `Data` folder in the game's
    /// order (see [`game_load_order`]).
    pub fn from_data_dir(data_dir: impl AsRef<Path>, active: &ActivePlugins) -> Result<Self> {
        let dir = data_dir.as_ref();
        let (names, mut warnings) = game_load_order(dir, active)?;
        let mut loaded = Vec::with_capacity(names.len());
        for name in names {
            let path = dir.join(&name);
            let plugin = Plugin::open(&path).map_err(|source| Error::InPlugin {
                name: name.clone(),
                source: Box::new(source),
            })?;
            loaded.push((name, Some(path), plugin));
        }
        let mut order = Self::from_plugins(loaded)?;
        warnings.append(&mut order.warnings);
        order.warnings = warnings;
        Ok(order)
    }

    fn build(
        plugins: Vec<LoadedPlugin>,
        slot_names: Vec<String>,
        warnings: Vec<String>,
        single: bool,
    ) -> Self {
        let total: usize = plugins.iter().map(|p| p.plugin.records().len()).sum();
        let mut winners: HashMap<FormId, Slot> = HashMap::with_capacity(total);
        let mut history: HashMap<FormId, Vec<Slot>> = HashMap::new();
        for (pi, p) in plugins.iter().enumerate() {
            for (ri, entry) in p.plugin.records().iter().enumerate() {
                let id = p.record_id(entry.header.form_id);
                let slot = (pi as u16, ri as u32);
                if let Some(previous) = winners.insert(id, slot) {
                    history
                        .entry(id)
                        .or_insert_with(|| vec![previous])
                        .push(slot);
                }
            }
        }
        Self {
            plugins,
            slot_names,
            winners,
            history,
            warnings,
            single,
            by_cell: OnceLock::new(),
            by_topic: OnceLock::new(),
            by_editor_id: OnceLock::new(),
        }
    }

    pub fn plugins(&self) -> &[LoadedPlugin] {
        &self.plugins
    }

    /// True when built with [`LoadOrder::single`].
    pub fn is_single(&self) -> bool {
        self.single
    }

    /// Non-fatal problems found while building the load order.
    pub fn warnings(&self) -> &[String] {
        &self.warnings
    }

    /// The file that a load-order index refers to.
    pub fn slot_name(&self, mod_index: u8) -> Option<&str> {
        self.slot_names
            .get(usize::from(mod_index))
            .map(String::as_str)
    }

    /// Number of distinct records after overrides.
    pub fn len(&self) -> usize {
        self.winners.len()
    }

    pub fn is_empty(&self) -> bool {
        self.winners.is_empty()
    }

    fn at(&self, (pi, ri): Slot, form_id: FormId) -> RecordRef<'_> {
        let plugin = &self.plugins[usize::from(pi)];
        RecordRef {
            plugin_index: usize::from(pi),
            plugin,
            entry: &plugin.plugin.records()[ri as usize],
            form_id,
            index: ri as usize,
        }
    }

    fn is_winner(&self, form_id: FormId, slot: Slot) -> bool {
        self.winners.get(&form_id) == Some(&slot)
    }

    /// The winning version of a record, by load-order form ID.
    pub fn get(&self, form_id: FormId) -> Option<RecordRef<'_>> {
        self.winners
            .get(&form_id)
            .map(|&slot| self.at(slot, form_id))
    }

    /// Every version of a record in load order; the last one wins.
    pub fn versions(&self, form_id: FormId) -> Vec<RecordRef<'_>> {
        match self.history.get(&form_id) {
            Some(slots) => slots.iter().map(|&s| self.at(s, form_id)).collect(),
            None => self.get(form_id).into_iter().collect(),
        }
    }

    /// The winning version of every record of one type, in load order.
    pub fn records_of_type(&self, kind: FourCC) -> impl Iterator<Item = RecordRef<'_>> + '_ {
        self.plugins.iter().enumerate().flat_map(move |(pi, p)| {
            p.plugin
                .record_indices_of_type(kind)
                .iter()
                .filter_map(move |&ri| {
                    let entry = &p.plugin.records()[ri];
                    let id = p.record_id(entry.header.form_id);
                    let slot = (pi as u16, ri as u32);
                    self.is_winner(id, slot).then(|| self.at(slot, id))
                })
        })
    }

    pub fn count_of_type(&self, kind: FourCC) -> usize {
        self.records_of_type(kind).count()
    }

    /// Record counts per type after overrides, most common first.
    pub fn type_counts(&self) -> Vec<(FourCC, usize)> {
        let mut counts: HashMap<FourCC, usize> = HashMap::new();
        for &(pi, ri) in self.winners.values() {
            let kind = self.plugins[usize::from(pi)].plugin.records()[ri as usize]
                .header
                .kind;
            *counts.entry(kind).or_default() += 1;
        }
        let mut counts: Vec<_> = counts.into_iter().collect();
        counts.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
        counts
    }

    /// How many records a plugin adds, and how many it overrides.
    pub fn plugin_stats(&self, plugin_index: usize) -> (usize, usize) {
        let p = &self.plugins[plugin_index];
        let new = p
            .plugin
            .records()
            .iter()
            .filter(|e| p.record_id(e.header.form_id).mod_index() == p.load_index)
            .count();
        (new, p.plugin.records().len() - new)
    }

    /// The cell a record version sits in, as a load-order form ID.
    pub fn cell_of(&self, record: &RecordRef<'_>) -> Option<FormId> {
        record.entry.cell.map(|c| record.plugin.to_global(c))
    }

    /// The winning versions of a topic's lines (`INFO`), in load order and
    /// file order. A line a later plugin moves to another topic belongs
    /// there.
    pub fn in_topic(&self, topic: FormId) -> Vec<RecordRef<'_>> {
        let index = self.by_topic.get_or_init(|| {
            let mut index: HashMap<FormId, Vec<Slot>> = HashMap::new();
            for (pi, p) in self.plugins.iter().enumerate() {
                for (ri, entry) in p.plugin.records().iter().enumerate() {
                    if let Some(t) = entry.topic {
                        index
                            .entry(p.to_global(t))
                            .or_default()
                            .push((pi as u16, ri as u32));
                    }
                }
            }
            index
        });
        let Some(slots) = index.get(&topic) else {
            return Vec::new();
        };
        slots
            .iter()
            .filter_map(|&slot| {
                let (pi, ri) = slot;
                let p = &self.plugins[usize::from(pi)];
                let id = p.record_id(p.plugin.records()[ri as usize].header.form_id);
                self.is_winner(id, slot).then(|| self.at(slot, id))
            })
            .collect()
    }

    /// The dialogue topic a line (`INFO`) belongs to, as a load-order form
    /// ID.
    pub fn topic_of(&self, record: &RecordRef<'_>) -> Option<FormId> {
        record.entry.topic.map(|t| record.plugin.to_global(t))
    }

    /// The worldspace a record version sits in, as a load-order form ID.
    pub fn world_of(&self, record: &RecordRef<'_>) -> Option<FormId> {
        record.entry.world.map(|w| record.plugin.to_global(w))
    }

    /// The winning versions of every object placed in a cell (references,
    /// actors, grenades, missiles), in load order. A reference that a later
    /// plugin moves to another cell belongs to the cell it was moved to.
    pub fn references_in_cell(&self, cell: FormId) -> Vec<RecordRef<'_>> {
        let mut found = self.in_cell(cell);
        found.retain(|r| PLACED_TYPES.contains(&r.entry.header.kind));
        // By type, then load order, as the records come out of the files.
        found.sort_by_key(|r| PLACED_TYPES.iter().position(|&k| k == r.entry.header.kind));
        found
    }

    /// The winning versions of every record in a cell's child groups:
    /// placed objects, terrain (`LAND`), navmeshes and so on, in load
    /// order. A record a later plugin moves to another cell belongs there.
    pub fn in_cell(&self, cell: FormId) -> Vec<RecordRef<'_>> {
        let index = self.by_cell.get_or_init(|| {
            let mut index: HashMap<FormId, Vec<Slot>> = HashMap::new();
            for (pi, p) in self.plugins.iter().enumerate() {
                for (ri, entry) in p.plugin.records().iter().enumerate() {
                    if let Some(c) = entry.cell {
                        index
                            .entry(p.to_global(c))
                            .or_default()
                            .push((pi as u16, ri as u32));
                    }
                }
            }
            index
        });
        let Some(slots) = index.get(&cell) else {
            return Vec::new();
        };
        slots
            .iter()
            .filter_map(|&slot| {
                let (pi, ri) = slot;
                let p = &self.plugins[usize::from(pi)];
                let id = p.record_id(p.plugin.records()[ri as usize].header.form_id);
                self.is_winner(id, slot).then(|| self.at(slot, id))
            })
            .collect()
    }

    /// How many placed objects each cell holds, counting winning versions.
    pub fn reference_counts(&self) -> HashMap<FormId, usize> {
        let mut counts = HashMap::new();
        for kind in PLACED_TYPES {
            for r in self.records_of_type(kind) {
                if let Some(cell) = self.cell_of(&r) {
                    *counts.entry(cell).or_default() += 1;
                }
            }
        }
        counts
    }

    /// The winning record with an editor ID (case-insensitive), from an
    /// index of every record built the first time it's asked (a few
    /// seconds for the whole game). When two records share an editor ID,
    /// the later-loaded one wins.
    pub fn form_by_editor_id(&self, editor_id: &str) -> Option<FormId> {
        let index = self.by_editor_id.get_or_init(|| {
            let mut index = HashMap::new();
            for (pi, p) in self.plugins.iter().enumerate() {
                for (ri, entry) in p.plugin.records().iter().enumerate() {
                    let id = p.record_id(entry.header.form_id);
                    if !self.is_winner(id, (pi as u16, ri as u32)) {
                        continue;
                    }
                    if let Ok(Some(edid)) = p.plugin.editor_id_of(entry) {
                        index.insert(edid.to_ascii_lowercase(), id);
                    }
                }
            }
            index
        });
        index.get(&editor_id.to_ascii_lowercase()).copied()
    }

    /// Finds the winning record with an editor ID (case-insensitive),
    /// optionally of one type. Later plugins are searched first.
    pub fn find_by_editor_id(
        &self,
        editor_id: &str,
        kind: Option<FourCC>,
    ) -> Result<Option<RecordRef<'_>>> {
        for (pi, p) in self.plugins.iter().enumerate().rev() {
            let indices: Box<dyn Iterator<Item = usize>> = match kind {
                Some(kind) => Box::new(p.plugin.record_indices_of_type(kind).iter().copied()),
                None => Box::new(0..p.plugin.records().len()),
            };
            for ri in indices {
                let entry = &p.plugin.records()[ri];
                let id = p.record_id(entry.header.form_id);
                let slot = (pi as u16, ri as u32);
                if !self.is_winner(id, slot) {
                    continue;
                }
                if let Some(found) = p.plugin.editor_id_of(entry)? {
                    if found.eq_ignore_ascii_case(editor_id) {
                        return Ok(Some(self.at(slot, id)));
                    }
                }
            }
        }
        Ok(None)
    }
}

/// IDs 1 to 0x7FF belong to forms the engine makes itself (the player,
/// default objects); `TESForm::IsDefaultForm` (`00484b40`).
fn is_default_form(id: FormId) -> bool {
    (1..=0x7FF).contains(&id.0)
}

/// A plugin file in the Data folder, as the game lists it before loading.
struct ListedFile {
    name: String,
    master: bool,
    modified: Option<SystemTime>,
    masters: Vec<String>,
    active: bool,
}

/// The plugins the game loads from a Data folder, in its load order, with
/// warnings. Translated from `TESDataHandler::BuildFileList` (`004624b0`),
/// `Main::InitTES` (`0086cf20`) and the start of the data handler's
/// `LoadFiles` (`00463070`) (decompiled, FalloutNV.exe 1.4.0.525):
///
/// 1. Every non-empty `*.esm`, then every `*.esp` (each in the folder's
///    name order) is put in a list: files with the master flag before the
///    others, each group by modification time, oldest first; a file with
///    the same time as one already listed goes before it. FalloutNV.esm
///    has no special place.
/// 2. Each master-flagged file's masters that come after it are moved to
///    just before it (and the moved file is checked in turn).
/// 3. Active: FalloutNV.esm (`[General] sTestFile1`), the names in
///    `plugins.txt` (`Main::LoadPluginsFromFile` `00872430`; its order
///    doesn't count), and every plugin with a `<name>.nam` file beside it
///    (how the DLC switch themselves on). [`ActivePlugins::OfficialOnly`]
///    takes FalloutNV.esm and the official files instead of the last two.
/// 4. In list order, each active file's masters are made active; one
///    missing from the folder stops the loading ("Unable to find
///    masterfile").
/// 5. The load order is the active files in list order.
pub fn game_load_order(dir: &Path, active: &ActivePlugins) -> Result<(Vec<String>, Vec<String>)> {
    let entries = std::fs::read_dir(dir)
        .map_err(|e| Error::DataDir(format!("could not read the folder {}: {e}", dir.display())))?;
    let mut esm = Vec::new();
    let mut esp = Vec::new();
    let mut nam = Vec::new();
    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().into_owned();
        let lower = name.to_ascii_lowercase();
        let Ok(meta) = entry.metadata() else { continue };
        if !meta.is_file() {
            continue;
        }
        if lower.ends_with(".nam") {
            nam.push(lower[..lower.len() - 4].to_string());
        }
        if meta.len() == 0 {
            continue;
        }
        let modified = meta.modified().ok();
        if lower.ends_with(".esm") {
            esm.push((name, modified));
        } else if lower.ends_with(".esp") {
            esp.push((name, modified));
        }
    }
    // FindFirstFile's order on NTFS: by name, upper-cased.
    esm.sort_by_key(|(n, _)| n.to_ascii_uppercase());
    esp.sort_by_key(|(n, _)| n.to_ascii_uppercase());

    let mut list: Vec<ListedFile> = Vec::new();
    for (name, modified) in esm.into_iter().chain(esp) {
        // A header that can't be read lists the file as a plain plugin; it
        // fails properly if it's loaded.
        let header = crate::plugin::read_header(dir.join(&name)).unwrap_or_default();
        let file = ListedFile {
            name,
            master: header.is_master,
            modified,
            masters: header.masters,
            active: false,
        };
        let at = list.iter().position(|e| {
            if e.master == file.master {
                e.modified >= file.modified
            } else {
                file.master
            }
        });
        list.insert(at.unwrap_or(list.len()), file);
    }

    // Masters of master files moved before them.
    let mut i = 0;
    while i < list.len() {
        let mut moved = false;
        if list[i].master {
            for master in list[i].masters.clone() {
                let later = (i..list.len()).find(|&j| list[j].name.eq_ignore_ascii_case(&master));
                if let Some(j) = later.filter(|&j| j != i) {
                    let m = list.remove(j);
                    list.insert(i, m);
                    moved = true;
                }
            }
        }
        if !moved {
            i += 1;
        }
    }

    let find = |list: &[ListedFile], wanted: &str| {
        list.iter()
            .position(|e| e.name.eq_ignore_ascii_case(wanted))
    };
    let main = find(&list, MAIN_MASTER)
        .ok_or_else(|| Error::DataDir(format!("there is no {MAIN_MASTER} in {}", dir.display())))?;
    list[main].active = true;
    let mut warnings = Vec::new();
    match active {
        ActivePlugins::OfficialOnly => {
            for name in &OFFICIAL_FILES[1..] {
                if let Some(i) = find(&list, name) {
                    list[i].active = true;
                }
            }
        }
        ActivePlugins::List(names) => {
            for name in names {
                match find(&list, name) {
                    Some(i) => list[i].active = true,
                    None => warnings.push(format!(
                        "{name} is listed as active but isn't in the Data folder (or is empty), so it was skipped"
                    )),
                }
            }
            for file in &mut list {
                let stem = file.name[..file.name.len() - 4].to_ascii_lowercase();
                if nam.contains(&stem) {
                    file.active = true;
                }
            }
        }
    }
    for i in 0..list.len() {
        if !list[i].active {
            continue;
        }
        for master in list[i].masters.clone() {
            match find(&list, &master) {
                Some(m) => list[m].active = true,
                None => {
                    return Err(Error::MissingMaster {
                        plugin: list[i].name.clone(),
                        master,
                    })
                }
            }
        }
    }
    let names = list
        .into_iter()
        .filter(|e| e.active)
        .map(|e| e.name)
        .collect();
    Ok((names, warnings))
}

/// The game's warning for a plugin whose master has changed size since the
/// plugin was saved (`00471af0`, checked for plugins without the master
/// flag: "One of the files that "%s" is dependent on has changed since the
/// last save.").
fn master_size_warnings(plugins: &[LoadedPlugin]) -> Vec<String> {
    let mut out = Vec::new();
    for p in plugins {
        let header = p.plugin.header();
        if header.is_master {
            continue;
        }
        for (master, size) in header.masters.iter().zip(&header.master_sizes) {
            let Some(size) = size else { continue };
            let actual = plugins
                .iter()
                .find(|m| m.name.eq_ignore_ascii_case(master))
                .map(|m| m.plugin.file_size() as u64);
            if actual != Some(*size) {
                out.push(format!(
                    "one of the files that {} is dependent on has changed since the last save: \
                     {master} is {} bytes, it was saved against {size}",
                    p.name,
                    actual.map_or("missing".into(), |a| a.to_string())
                ));
            }
        }
    }
    out
}

/// Parses a `plugins.txt` active-plugin list as the game reads it
/// (`Main::LoadPluginsFromFile` `00872430`): line by line (a carriage
/// return before the line feed dropped, as a text-mode read drops it),
/// skipping lines that start with `#` and lines of one character or none
/// counting the line feed; the line feed is cut off and nothing else, so a
/// name with spaces around it doesn't match a file.
pub fn parse_plugins_txt(bytes: &[u8]) -> Vec<String> {
    text::decode_cp1252(bytes)
        .split_inclusive('\n')
        .map(|line| match line.strip_suffix("\r\n") {
            Some(rest) => format!("{rest}\n"),
            None => line.to_string(),
        })
        .filter(|line| !line.starts_with('#') && line.chars().count() > 1)
        .map(|line| line.strip_suffix('\n').unwrap_or(&line).to_string())
        .collect()
}

/// Where the game keeps its active-plugin list on Windows:
/// `%LOCALAPPDATA%\FalloutNV\plugins.txt`. Returns `None` if it doesn't exist.
pub fn default_plugins_txt() -> Option<PathBuf> {
    let base = std::env::var_os("LOCALAPPDATA")?;
    let path = Path::new(&base).join("FalloutNV").join("plugins.txt");
    path.is_file().then_some(path)
}

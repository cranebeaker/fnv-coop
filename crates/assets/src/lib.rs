//! Finding game files the way Fallout: New Vegas does.
//!
//! Files come from two places: loose files under the `Data` folder, and
//! the BSA archives the game loads: the ones its settings list
//! (`SArchiveList`), then `Update.bsa`, then for each active plugin in load
//! order the archives whose names start with the plugin's
//! (`DeadMoney - Main.bsa` for `DeadMoney.esm`). Which archive's copy of a
//! file wins follows the game's archive list ([`archive_priority`]: DLC and
//! mod archives before the base game's, the first-loaded mod archive
//! first), and with `bInvalidateOlderFiles` on (the exe's default) a loose
//! file beats every archived copy. docs/MODS.md has the rules and the
//! addresses they were traced from.
//!
//! ```no_run
//! let plugins = vec!["FalloutNV.esm".to_string(), "DeadMoney.esm".to_string()];
//! let assets = assets::Assets::open("Data", &plugins)?;
//! if let Some(bytes) = assets.read("textures/weapons/1handpistol/10mmpistol.dds")? {
//!     println!("{} bytes", bytes.len());
//! }
//! # Ok::<(), assets::Error>(())
//! ```

use std::collections::HashMap;
use std::fmt;
use std::path::{Path, PathBuf};

pub use bsa::normalize_path;

/// The archives New Vegas loads by default (its `SArchiveList` setting),
/// in order. Used only when no ini file with the setting is found.
pub const DEFAULT_ARCHIVES: [&str; 6] = [
    "Fallout - Textures.bsa",
    "Fallout - Textures2.bsa",
    "Fallout - Meshes.bsa",
    "Fallout - Voices1.bsa",
    "Fallout - Sound.bsa",
    "Fallout - Misc.bsa",
];

/// The archive New Vegas's patches added. The shipped ini files don't list
/// it, but records in FalloutNV.esm use meshes that only it contains (the
/// NCR guard towers and Novac motel pieces among them), so it's loaded
/// right after the listed archives.
pub const PATCH_ARCHIVE: &str = "Update.bsa";

/// Guards against symlink loops while scanning loose files.
const MAX_FOLDER_DEPTH: usize = 32;

#[derive(Debug)]
pub enum Error {
    Io {
        path: PathBuf,
        source: std::io::Error,
    },
    Archive {
        name: String,
        source: bsa::Error,
    },
}

pub type Result<T> = std::result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io { path, source } => write!(f, "{}: {source}", path.display()),
            Error::Archive { name, source } => write!(f, "{name}: {source}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io { source, .. } => Some(source),
            Error::Archive { source, .. } => Some(source),
        }
    }
}

/// Why an archive is loaded.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArchiveReason {
    /// Listed in the game's default archive list.
    Default,
    /// The game's patch archive (see [`PATCH_ARCHIVE`]).
    Patch,
    /// Named after this active plugin.
    Plugin(String),
}

pub struct LoadedArchive {
    pub name: String,
    pub reason: ArchiveReason,
    pub archive: bsa::Archive,
}

/// Where a file comes from.
#[derive(Clone, Copy)]
pub enum Source<'a> {
    Loose(&'a Path),
    Archive {
        name: &'a str,
        archive: &'a bsa::Archive,
        entry: &'a bsa::FileEntry,
    },
}

impl Source<'_> {
    pub fn read(&self) -> Result<Vec<u8>> {
        match *self {
            Source::Loose(path) => std::fs::read(path).map_err(|source| Error::Io {
                path: path.to_path_buf(),
                source,
            }),
            Source::Archive {
                name,
                archive,
                entry,
            } => archive.read(entry).map_err(|source| Error::Archive {
                name: name.to_string(),
                source,
            }),
        }
    }

    /// A short description such as `loose file` or `Fallout - Meshes.bsa`.
    pub fn describe(&self) -> String {
        match self {
            Source::Loose(_) => "loose file".into(),
            Source::Archive { name, .. } => (*name).to_string(),
        }
    }
}

/// The game's own list of archives to load, and where it was read from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchiveList {
    pub names: Vec<String>,
    /// The ini file the list came from, or a note that the built-in default
    /// was used.
    pub source: String,
}

/// Reads `SArchiveList` from an ini file, if the file has it.
pub fn read_archive_list(ini: &Path) -> Option<Vec<String>> {
    let text = std::fs::read(ini).ok()?;
    let text = String::from_utf8_lossy(&text);
    text.lines()
        .map(str::trim)
        .filter(|l| !l.starts_with(';') && !l.starts_with('#'))
        .filter_map(|l| l.split_once('='))
        .find(|(key, _)| key.trim().eq_ignore_ascii_case("SArchiveList"))
        .map(|(_, value)| {
            value
                .split(',')
                .map(str::trim)
                .filter(|n| !n.is_empty())
                .map(String::from)
                .collect()
        })
}

/// The game's INI settings, as `[Section]` and `key=value` lines from its
/// INI files, later files winning where they share a key (the user's
/// `Fallout.ini`, then `FalloutPrefs.ini`, over the install's
/// `Fallout_default.ini`: [`default_settings_files`]). Names compare without
/// case.
#[derive(Debug, Clone, Default)]
pub struct IniSettings {
    values: std::collections::HashMap<(String, String), String>,
}

impl IniSettings {
    /// Reads the files in order (missing ones are skipped).
    pub fn load(files: &[PathBuf]) -> IniSettings {
        let mut out = IniSettings::default();
        for file in files {
            if let Ok(bytes) = std::fs::read(file) {
                out.add(&String::from_utf8_lossy(&bytes));
            }
        }
        out
    }

    /// Adds one file's text.
    pub fn add(&mut self, text: &str) {
        let mut section = String::new();
        for line in text.lines().map(str::trim) {
            if line.starts_with(';') || line.starts_with('#') {
                continue;
            }
            if let Some(name) = line.strip_prefix('[').and_then(|l| l.strip_suffix(']')) {
                section = name.trim().to_ascii_lowercase();
            } else if let Some((key, value)) = line.split_once('=') {
                self.values.insert(
                    (section.clone(), key.trim().to_ascii_lowercase()),
                    value.trim().to_string(),
                );
            }
        }
    }

    pub fn get(&self, section: &str, key: &str) -> Option<&str> {
        self.values
            .get(&(section.to_ascii_lowercase(), key.to_ascii_lowercase()))
            .map(String::as_str)
    }

    pub fn float(&self, section: &str, key: &str) -> Option<f32> {
        self.get(section, key)?.parse().ok()
    }
}

/// The INI files the game reads its settings from, lowest first: the
/// install's `Fallout_default.ini`, then the user's `Fallout.ini` and
/// `FalloutPrefs.ini` under `Documents\My Games\FalloutNV` (or OneDrive's
/// Documents).
pub fn default_settings_files(data_dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    if let Some(game_dir) = data_dir.parent() {
        files.push(game_dir.join("Fallout_default.ini"));
    }
    if let Some(home) = user_home() {
        for documents in [
            home.join("Documents"),
            home.join("OneDrive").join("Documents"),
        ] {
            let dir = documents.join("My Games").join("FalloutNV");
            if dir.join("Fallout.ini").exists() {
                files.push(dir.join("Fallout.ini"));
                files.push(dir.join("FalloutPrefs.ini"));
                break;
            }
        }
    }
    files
}

/// Uses the first candidate ini file that has an archive list, or the
/// built-in default.
pub fn archive_list_from(candidates: &[PathBuf]) -> ArchiveList {
    for ini in candidates {
        if let Some(names) = read_archive_list(ini) {
            return ArchiveList {
                names,
                source: ini.display().to_string(),
            };
        }
    }
    ArchiveList {
        names: DEFAULT_ARCHIVES.iter().map(|s| s.to_string()).collect(),
        source: "built-in default (no ini file with SArchiveList found)".into(),
    }
}

/// Where the game's settings usually are: `Fallout.ini` under
/// `Documents\My Games\FalloutNV` (also checked under OneDrive, which
/// often holds Documents), then `Fallout_default.ini` in the install folder
/// that contains `Data`.
pub fn default_ini_candidates(data_dir: &Path) -> Vec<PathBuf> {
    let mut candidates = Vec::new();
    if let Some(home) = user_home() {
        for documents in [
            home.join("Documents"),
            home.join("OneDrive").join("Documents"),
        ] {
            candidates.push(
                documents
                    .join("My Games")
                    .join("FalloutNV")
                    .join("Fallout.ini"),
            );
        }
    }
    if let Some(game_dir) = data_dir.parent() {
        candidates.push(game_dir.join("Fallout_default.ini"));
    }
    candidates
}

/// The game's `[Archive]` settings that decide which archives load and
/// whether loose files beat archived ones. Read by `00876d20`: `bUseArchives`
/// and `SArchiveList` go to `ArchiveManager::Init` (`00af43a0`),
/// `bInvalidateOlderFiles` and `SInvalidationFile` to
/// `ArchiveManager::SetInvalidation` (`00af4490`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchiveSettings {
    /// `bUseArchives` (the exe's default 1): off, no archive is opened.
    pub use_archives: bool,
    /// `SArchiveList`: the archives opened first, in order.
    pub list: Vec<String>,
    /// `bInvalidateOlderFiles` (the exe's default 1): a loose file beats
    /// the archived copy of the same path, whatever their dates.
    pub invalidate_older_files: bool,
    /// `SInvalidationFile` (the exe's default `ArchiveInvalidation.txt`):
    /// a list of archived files and folders to drop when an archive opens.
    pub invalidation_file: String,
}

impl Default for ArchiveSettings {
    /// The exe's defaults, with the shipped `Fallout_default.ini`'s list.
    fn default() -> Self {
        Self {
            use_archives: true,
            list: DEFAULT_ARCHIVES.iter().map(|s| s.to_string()).collect(),
            invalidate_older_files: true,
            invalidation_file: "ArchiveInvalidation.txt".into(),
        }
    }
}

impl ArchiveSettings {
    /// The settings from the game's INI files (the exe's defaults where a
    /// file doesn't say), with an archive list chosen by
    /// [`archive_list_from`].
    pub fn from_ini(ini: &IniSettings, list: Vec<String>) -> Self {
        let flag = |key: &str, default: bool| {
            ini.get("Archive", key)
                .and_then(|v| v.trim().parse::<i64>().ok())
                .map_or(default, |v| v != 0)
        };
        let defaults = Self::default();
        Self {
            use_archives: flag("bUseArchives", defaults.use_archives),
            list,
            invalidate_older_files: flag("bInvalidateOlderFiles", defaults.invalidate_older_files),
            invalidation_file: ini
                .get("Archive", "SInvalidationFile")
                .map_or(defaults.invalidation_file, |v| v.trim().to_string()),
        }
    }
}

/// What an archive invalidation file (`ArchiveInvalidation.txt`) lists.
/// Read by `ArchiveManager::LoadInvalidationFile` (`00af5ab0`): lines end
/// at a carriage return (the character after it, the line feed, is
/// skipped); a line without a backslash names a file (its name only), a
/// line with one names the folder it's in (a leading backslash dropped).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Invalidation {
    /// File names (`crate.dds`), lower case.
    pub files: std::collections::BTreeSet<String>,
    /// Folders (`textures\clutter`), lower case.
    pub folders: std::collections::BTreeSet<String>,
}

impl Invalidation {
    pub fn parse(bytes: &[u8]) -> Self {
        let mut out = Self::default();
        let mut rest = bytes;
        while !rest.is_empty() {
            let end = rest.iter().position(|&b| b == b'\r').unwrap_or(rest.len());
            let line = String::from_utf8_lossy(&rest[..end]).into_owned();
            // The line, its carriage return and the character after it.
            rest = rest.get(end + 2..).unwrap_or(&[]);
            if line.is_empty() {
                continue;
            }
            // `_splitpath`: both slashes separate folders.
            let split = |l: &str| match l.rfind(['\\', '/']) {
                Some(i) => (l[..i].to_string(), l[i + 1..].to_string()),
                None => (String::new(), l.to_string()),
            };
            if line.contains('\\') {
                let line = line.strip_prefix('\\').unwrap_or(&line);
                let (folder, _) = split(line);
                let folder = if folder.is_empty() {
                    ".".into()
                } else {
                    folder
                };
                out.folders
                    .insert(folder.replace('/', "\\").to_ascii_lowercase());
            } else {
                out.files.insert(split(&line).1.to_ascii_lowercase());
            }
        }
        out
    }

    pub fn is_empty(&self) -> bool {
        self.files.is_empty() && self.folders.is_empty()
    }

    /// Whether an archived file is dropped when its archive opens
    /// (`00afad00`): its folder is listed, or its name is and the folder
    /// exists loose under `Data`.
    pub fn drops(&self, folder: &str, name: &str, folder_on_disk: impl Fn(&str) -> bool) -> bool {
        let folder = folder.replace('/', "\\").to_ascii_lowercase();
        self.folders.contains(&folder)
            || (self.files.contains(&name.to_ascii_lowercase()) && folder_on_disk(&folder))
    }
}

/// Where in the archive manager's list a new archive goes, by its name:
/// translated from `ArchiveManager::OpenArchive` (`00af4be0`, decompiled,
/// FalloutNV.exe 1.4.0.525). Names containing (case matters) `Fallo` go to
/// the end; the others (rank by the first of `DeadM` 1, `Hones` 2, `Lones`
/// 3, `OldWo` 4, `Updat` 5 they contain, anything else 6, "a user created
/// or misnamed BSA") go before the first archive already listed whose name
/// has `Fallo` or `DeadM`, or `Hones` when the new rank is over 1, `Lones`
/// over 2, `OldWo` or `Updat` over 3; at the front if none. Lookups walk
/// the list from the front, so the first archive holding a file wins.
/// Returns the list (indexes into `names`) after adding each in turn.
pub fn archive_priority(names: &[&str]) -> Vec<usize> {
    let rank = |n: &str| {
        ["Fallo", "DeadM", "Hones", "Lones", "OldWo", "Updat"]
            .iter()
            .position(|m| n.contains(m))
            .unwrap_or(6)
    };
    let mut list: Vec<usize> = Vec::with_capacity(names.len());
    for (i, name) in names.iter().enumerate() {
        let r = rank(name);
        if r == 0 {
            list.push(i);
            continue;
        }
        let before = list.iter().position(|&j| {
            let n = names[j];
            n.contains("Fallo")
                || n.contains("DeadM")
                || (n.contains("Hones") && r > 1)
                || (n.contains("Lones") && r > 2)
                || (n.contains("OldWo") && r > 3)
                || (n.contains("Updat") && r > 3)
        });
        list.insert(before.unwrap_or(0), i);
    }
    list
}

/// Windows game installs use `USERPROFILE`; native Unix builds use `HOME`.
/// Prefer the Windows variable when running in a compatibility environment.
fn user_home() -> Option<PathBuf> {
    user_home_from(std::env::var_os("USERPROFILE"), std::env::var_os("HOME"))
}

fn user_home_from(
    userprofile: Option<std::ffi::OsString>,
    home: Option<std::ffi::OsString>,
) -> Option<PathBuf> {
    userprofile.or(home).map(PathBuf::from)
}

/// Every file the game can see, with overrides resolved.
pub struct Assets {
    data_dir: PathBuf,
    /// Loaded archives in the order they were opened.
    archives: Vec<LoadedArchive>,
    /// Indexes into `archives`, highest priority first.
    priority: Vec<usize>,
    unused_archives: Vec<String>,
    /// Archives in the folder that aren't loaded, kept open so missing
    /// files can be traced to them.
    inactive: Vec<LoadedArchive>,
    /// Normalized path -> file on disk.
    loose: HashMap<String, PathBuf>,
    /// Normalized path -> (archive index, file index) of the winning copy.
    archived: HashMap<String, (usize, usize)>,
    /// Archived copies dropped by the invalidation file: (archive, file).
    dropped: std::collections::HashSet<(usize, usize)>,
    /// Loose files beat archived ones (`bInvalidateOlderFiles`).
    loose_first: bool,
    warnings: Vec<String>,
}

fn stem_of(file_name: &str) -> &str {
    file_name
        .rsplit_once('.')
        .map_or(file_name, |(stem, _)| stem)
}

impl Assets {
    /// Opens a `Data` folder using the built-in default archive list.
    /// `plugins` are the active plugin file names in load order; they decide
    /// which extra archives are loaded.
    pub fn open(data_dir: impl AsRef<Path>, plugins: &[String]) -> Result<Self> {
        Self::open_with_settings(data_dir, plugins, &ArchiveSettings::default())
    }

    /// Opens a `Data` folder with an explicit archive list (see
    /// [`archive_list_from`]) and the exe's other defaults.
    pub fn open_with(
        data_dir: impl AsRef<Path>,
        plugins: &[String],
        archive_list: &[String],
    ) -> Result<Self> {
        let settings = ArchiveSettings {
            list: archive_list.to_vec(),
            ..ArchiveSettings::default()
        };
        Self::open_with_settings(data_dir, plugins, &settings)
    }

    /// Opens a `Data` folder the way the game does with these settings.
    ///
    /// Archives open in this order: `SArchiveList`'s
    /// (`ArchiveManager::OpenMasterArchives` `00af4550`), then `Update.bsa`
    /// (`TESDataHandler::BuildFileList` `004624b0`), then for each plugin in
    /// load order every non-empty `Data\<plugin name without extension>*.bsa`
    /// not already open (`00463070`: `DeadMoney - Main.bsa` for
    /// `DeadMoney.esm`, but also `ModAExtra.bsa` for `ModA.esp`). An archive
    /// that can't be read is left out with a warning, as the game leaves it
    /// out. Which copy of a file wins: [`archive_priority`], and loose files
    /// first when `bInvalidateOlderFiles` is on.
    pub fn open_with_settings(
        data_dir: impl AsRef<Path>,
        plugins: &[String],
        settings: &ArchiveSettings,
    ) -> Result<Self> {
        let data_dir = data_dir.as_ref().to_path_buf();
        let io_error = |source| Error::Io {
            path: data_dir.clone(),
            source,
        };
        // FindFirstFile's order on NTFS: by name, upper-cased.
        let mut bsa_files: Vec<(String, u64)> = std::fs::read_dir(&data_dir)
            .map_err(io_error)?
            .flatten()
            .filter(|e| e.path().is_file())
            .map(|e| {
                let size = e.metadata().map_or(0, |m| m.len());
                (e.file_name().to_string_lossy().into_owned(), size)
            })
            .filter(|(n, _)| n.to_ascii_lowercase().ends_with(".bsa"))
            .collect();
        bsa_files.sort_by_key(|(n, _)| n.to_ascii_uppercase());

        // Pick archives in the order the game opens them.
        let mut chosen: Vec<(String, ArchiveReason)> = Vec::new();
        let take =
            |name: &str, chosen: &mut Vec<(String, ArchiveReason)>, reason: ArchiveReason| {
                let already = chosen.iter().any(|(n, _)| n.eq_ignore_ascii_case(name));
                if !already {
                    chosen.push((name.to_string(), reason));
                }
            };
        if settings.use_archives {
            for wanted in &settings.list {
                if let Some((found, _)) = bsa_files
                    .iter()
                    .find(|(n, _)| n.eq_ignore_ascii_case(wanted))
                {
                    take(found, &mut chosen, ArchiveReason::Default);
                }
            }
            if let Some((found, _)) = bsa_files
                .iter()
                .find(|(n, _)| n.eq_ignore_ascii_case(PATCH_ARCHIVE))
            {
                take(found, &mut chosen, ArchiveReason::Patch);
            }
            for plugin in plugins {
                let stem = stem_of(plugin).to_ascii_lowercase();
                for (name, size) in &bsa_files {
                    if *size > 0 && name.to_ascii_lowercase().starts_with(&stem) {
                        take(name, &mut chosen, ArchiveReason::Plugin(plugin.clone()));
                    }
                }
            }
        }
        let unused_archives: Vec<String> = bsa_files
            .iter()
            .map(|(n, _)| n)
            .filter(|n| !chosen.iter().any(|(c, _)| c == *n))
            .cloned()
            .collect();
        // Unloaded archives that can't be read are simply left out.
        let inactive = unused_archives
            .iter()
            .filter_map(|name| {
                let archive = bsa::Archive::open(data_dir.join(name)).ok()?;
                Some(LoadedArchive {
                    name: name.clone(),
                    reason: ArchiveReason::Default,
                    archive,
                })
            })
            .collect();

        let mut warnings = Vec::new();
        let mut archives = Vec::with_capacity(chosen.len());
        for (name, reason) in chosen {
            match bsa::Archive::open(data_dir.join(&name)) {
                Ok(archive) => archives.push(LoadedArchive {
                    name,
                    reason,
                    archive,
                }),
                Err(e) => {
                    warnings.push(format!("{name} couldn't be read, so it isn't loaded: {e}"))
                }
            }
        }
        let names: Vec<&str> = archives.iter().map(|a| a.name.as_str()).collect();
        let priority = archive_priority(&names);

        let mut loose = HashMap::new();
        scan_loose(&data_dir, "", 0, &mut loose)?;

        // The invalidation file, found as any file is: in the game's
        // folder, then under Data.
        let invalidation =
            if settings.invalidate_older_files && !settings.invalidation_file.is_empty() {
                let name = &settings.invalidation_file;
                data_dir
                    .parent()
                    .map(|game| game.join(name))
                    .into_iter()
                    .chain([data_dir.join(name)])
                    .find_map(|p| std::fs::read(p).ok())
                    .map(|b| Invalidation::parse(&b))
                    .unwrap_or_default()
            } else {
                Invalidation::default()
            };
        let mut dropped = std::collections::HashSet::new();
        if !invalidation.is_empty() {
            for (ai, loaded) in archives.iter().enumerate() {
                let named = bsa::archive_flags::DIRECTORY_NAMES | bsa::archive_flags::FILE_NAMES;
                if loaded.archive.header().archive_flags & named != named {
                    // The game compares loose files' dates with the
                    // archive's instead (Archive::InvalidateOlderFilesByPath
                    // 00b01590); archives without names aren't read here.
                    continue;
                }
                for (fi, file) in loaded.archive.files().iter().enumerate() {
                    let folder = &loaded.archive.folders()[file.folder].name;
                    if invalidation.drops(folder, &file.name, |f| {
                        data_dir.join(f.replace('\\', "/")).is_dir()
                    }) {
                        dropped.insert((ai, fi));
                    }
                }
            }
        }

        let mut archived = HashMap::new();
        for &ai in priority.iter().rev() {
            for (fi, file) in archives[ai].archive.files().iter().enumerate() {
                if !dropped.contains(&(ai, fi)) {
                    archived.insert(file.path.clone(), (ai, fi));
                }
            }
        }

        Ok(Self {
            data_dir,
            archives,
            priority,
            unused_archives,
            inactive,
            loose,
            archived,
            dropped,
            loose_first: settings.invalidate_older_files,
            warnings,
        })
    }

    /// Problems that didn't stop the folder opening (archives that
    /// couldn't be read).
    pub fn warnings(&self) -> &[String] {
        &self.warnings
    }

    /// Loaded archives, highest priority (the one a shared file comes
    /// from) first.
    pub fn by_priority(&self) -> impl Iterator<Item = &LoadedArchive> + '_ {
        self.priority.iter().map(|&i| &self.archives[i])
    }

    pub fn data_dir(&self) -> &Path {
        &self.data_dir
    }

    /// Loaded archives in the order they were opened (see
    /// [`Assets::by_priority`] for which one wins).
    pub fn archives(&self) -> &[LoadedArchive] {
        &self.archives
    }

    /// Archives in the folder that the game doesn't load with these plugins.
    pub fn unused_archives(&self) -> &[String] {
        &self.unused_archives
    }

    /// Names of archives the game doesn't load that contain this path; a
    /// quick way to explain a "missing" file.
    pub fn in_unused_archives(&self, path: &str) -> Vec<&str> {
        let key = normalize_path(path);
        self.inactive
            .iter()
            .filter(|a| a.archive.find(&key).is_some())
            .map(|a| a.name.as_str())
            .collect()
    }

    pub fn loose_count(&self) -> usize {
        self.loose.len()
    }

    /// Number of distinct paths visible to the game.
    pub fn len(&self) -> usize {
        self.archived.len()
            + self
                .loose
                .keys()
                .filter(|k| !self.archived.contains_key(*k))
                .count()
    }

    pub fn is_empty(&self) -> bool {
        self.archived.is_empty() && self.loose.is_empty()
    }

    fn archived_source(&self, ai: usize, fi: usize) -> Source<'_> {
        let loaded = &self.archives[ai];
        Source::Archive {
            name: &loaded.name,
            archive: &loaded.archive,
            entry: &loaded.archive.files()[fi],
        }
    }

    /// Where the game would load a file from, if anywhere. Translated from
    /// the file finder (`00afe220`, decompiled, FalloutNV.exe 1.4.0.525):
    /// the archives are asked first (`ArchiveManager::GetArchiveForFile`
    /// `00af6160`, the first archive by [`archive_priority`]), the loose
    /// file only if none has it; but with `bInvalidateOlderFiles` on, an
    /// archived entry whose path exists loose under `Data` is dropped the
    /// first time it's looked up (`Archive::CheckInvalidateFile` `00afb190`),
    /// so the loose file wins whatever its date.
    pub fn locate(&self, path: &str) -> Option<Source<'_>> {
        let key = normalize_path(path);
        let loose = self.loose.get(&key).map(|p| Source::Loose(p));
        if self.loose_first && loose.is_some() {
            return loose;
        }
        self.archived
            .get(&key)
            .map(|&(ai, fi)| self.archived_source(ai, fi))
            .or(loose)
    }

    pub fn contains(&self, path: &str) -> bool {
        let key = normalize_path(path);
        self.loose.contains_key(&key) || self.archived.contains_key(&key)
    }

    /// Reads a file, or returns `None` if the game can't see it.
    pub fn read(&self, path: &str) -> Result<Option<Vec<u8>>> {
        self.locate(path).map(|s| s.read()).transpose()
    }

    /// Every copy of a file the game could use, highest priority (the
    /// one used) first. Copies the invalidation file dropped aren't listed.
    pub fn versions(&self, path: &str) -> Vec<Source<'_>> {
        let key = normalize_path(path);
        let mut out = Vec::new();
        for &ai in &self.priority {
            let loaded = &self.archives[ai];
            if let Some(entry) = loaded.archive.find(&key) {
                let fi = loaded
                    .archive
                    .files()
                    .iter()
                    .position(|f| std::ptr::eq(f, entry))
                    .unwrap_or(usize::MAX);
                if self.dropped.contains(&(ai, fi)) {
                    continue;
                }
                out.push(Source::Archive {
                    name: &loaded.name,
                    archive: &loaded.archive,
                    entry,
                });
            }
        }
        if let Some(p) = self.loose.get(&key) {
            let at = if self.loose_first { 0 } else { out.len() };
            out.insert(at, Source::Loose(p));
        }
        out
    }

    /// Every visible path (each once), in no particular order.
    pub fn paths(&self) -> impl Iterator<Item = &str> + '_ {
        self.archived.keys().map(String::as_str).chain(
            self.loose
                .keys()
                .filter(|k| !self.archived.contains_key(*k))
                .map(String::as_str),
        )
    }
}

/// Indexes files in subfolders of `Data` (files directly in `Data` are
/// plugins and archives, not assets).
fn scan_loose(
    dir: &Path,
    prefix: &str,
    depth: usize,
    out: &mut HashMap<String, PathBuf>,
) -> Result<()> {
    if depth > MAX_FOLDER_DEPTH {
        return Ok(());
    }
    let entries = std::fs::read_dir(dir).map_err(|source| Error::Io {
        path: dir.to_path_buf(),
        source,
    })?;
    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().to_ascii_lowercase();
        let Ok(kind) = entry.file_type() else {
            continue;
        };
        let relative = if prefix.is_empty() {
            name.clone()
        } else {
            format!("{prefix}\\{name}")
        };
        if kind.is_dir() {
            scan_loose(&entry.path(), &relative, depth + 1, out)?;
        } else if depth > 0 {
            out.insert(relative, entry.path());
        }
    }
    Ok(())
}

/// Normalizes a texture path as written in a mesh: case and slashes, a
/// leading `data\`, and the implied `textures\` folder (paths sometimes
/// omit it, or include an artist's full path before it).
pub fn texture_path(reference: &str) -> String {
    folder_relative(reference, "textures")
}

/// Normalizes a model path as written in a plugin record, which is
/// relative to `meshes\`.
pub fn mesh_path(reference: &str) -> String {
    folder_relative(reference, "meshes")
}

fn folder_relative(reference: &str, folder: &str) -> String {
    let path = normalize_path(reference.trim());
    let marker = format!("{folder}\\");
    if path.starts_with(&marker) {
        return path;
    }
    match path.find(&format!("\\{marker}")) {
        Some(i) => path[i + 1..].to_string(),
        None => format!("{marker}{path}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_texture_and_mesh_references() {
        assert_eq!(
            texture_path("Textures\\Weapons\\Gun.DDS"),
            "textures\\weapons\\gun.dds"
        );
        assert_eq!(
            texture_path("weapons/gun.dds"),
            "textures\\weapons\\gun.dds"
        );
        assert_eq!(texture_path("Data\\Textures\\x.dds"), "textures\\x.dds");
        assert_eq!(
            texture_path("C:\\Projects\\Fallout\\Data\\Textures\\Clutter\\x.dds"),
            "textures\\clutter\\x.dds"
        );
        assert_eq!(
            mesh_path("Weapons\\1HandPistol\\10mm.NIF"),
            "meshes\\weapons\\1handpistol\\10mm.nif"
        );
        assert_eq!(mesh_path("meshes\\a.nif"), "meshes\\a.nif");
    }

    #[test]
    fn home_discovery_falls_back_to_macos_home() {
        assert_eq!(
            user_home_from(None, Some("player-home".into())),
            Some(PathBuf::from("player-home"))
        );
        assert_eq!(
            user_home_from(Some("windows-home".into()), Some("mac-home".into())),
            Some(PathBuf::from("windows-home"))
        );
    }
}

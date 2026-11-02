//! Mod setups for the load-order and archive tests: plugins with masters
//! and overrides, BSA archives, loose files, `plugins.txt`, `.nam` files,
//! INI settings and `ArchiveInvalidation.txt`, each written to a fresh
//! Data folder. Nothing from the game is used.
//!
//! The same cases are written to disk for inspection (or for trying them
//! in the viewer, nvinspect or the original game) by the `mod-cases`
//! program: `cargo run -p testdata --bin mod-cases -- <folder>`. Each case
//! gets its own folder with a `Data` folder and a `CASE.txt` saying what
//! the game does with it (docs/MODS.md has the rules and their addresses).

use std::path::Path;

use crate::{group, sub, zstr};

/// A form record with an editor ID and a display name.
pub fn named(kind: &[u8; 4], id: u32, editor_id: &str, name: &str) -> Vec<u8> {
    let mut d = sub(b"EDID", &zstr(editor_id));
    d.extend(sub(b"FULL", &zstr(name)));
    form(kind, id, 0, &d)
}

/// A record with flags (the record header written in full).
pub fn form(kind: &[u8; 4], id: u32, flags: u32, data: &[u8]) -> Vec<u8> {
    let mut v = kind.to_vec();
    v.extend((data.len() as u32).to_le_bytes());
    v.extend(flags.to_le_bytes());
    v.extend(id.to_le_bytes());
    v.extend([0; 4]);
    v.extend(15u16.to_le_bytes());
    v.extend([0; 2]);
    v.extend(data);
    v
}

/// The `TES4` header's master flag.
pub const MASTER_FLAG: u32 = 1;

/// A plugin file being put together.
#[derive(Debug, Clone, Default)]
pub struct PluginFile {
    master: bool,
    version: f32,
    /// Masters in order, with the size each was saved against (`DATA`
    /// after `MAST`; the game warns when the file's size differs).
    masters: Vec<(String, Option<u64>)>,
    groups: Vec<u8>,
}

impl PluginFile {
    /// A plugin with or without the master flag, format version 1.34.
    pub fn new(master: bool) -> Self {
        Self {
            master,
            version: 1.34,
            ..Self::default()
        }
    }

    pub fn master(mut self, name: &str, size: u64) -> Self {
        self.masters.push((name.to_string(), Some(size)));
        self
    }

    /// A master without its size (no `DATA` after the `MAST`).
    pub fn master_unsized(mut self, name: &str) -> Self {
        self.masters.push((name.to_string(), None));
        self
    }

    pub fn version(mut self, version: f32) -> Self {
        self.version = version;
        self
    }

    /// A top-level group of records of one type.
    pub fn top(mut self, kind: &[u8; 4], records: &[Vec<u8>]) -> Self {
        self.groups.extend(group(*kind, 0, &records.concat()));
        self
    }

    /// An interior cell's top-level group: block 0, sub-block 0, the cell
    /// and its temporary children (placed objects).
    pub fn interior(mut self, cell: Vec<u8>, cell_id: u32, placed: &[Vec<u8>]) -> Self {
        let mut children = cell;
        let temporary = group(cell_id.to_le_bytes(), 9, &placed.concat());
        children.extend(group(cell_id.to_le_bytes(), 6, &temporary));
        let sub_block = group(0u32.to_le_bytes(), 3, &children);
        let block = group(0u32.to_le_bytes(), 2, &sub_block);
        self.groups.extend(group(*b"CELL", 0, &block));
        self
    }

    pub fn build(&self) -> Vec<u8> {
        let mut hedr = self.version.to_le_bytes().to_vec();
        hedr.extend(0i32.to_le_bytes());
        hedr.extend(0x800u32.to_le_bytes());
        let mut d = sub(b"HEDR", &hedr);
        d.extend(sub(b"CNAM", &zstr("nv-rs mod-cases")));
        for (name, size) in &self.masters {
            d.extend(sub(b"MAST", &zstr(name)));
            if let Some(size) = size {
                d.extend(sub(b"DATA", &size.to_le_bytes()));
            }
        }
        let mut out = form(b"TES4", 0, u32::from(self.master), &d);
        out.extend(&self.groups);
        out
    }
}

// ---------------------------------------------------------------------------
// Archives
// ---------------------------------------------------------------------------

/// Archive content bits (the header's file flags), which the game checks
/// against a file's extension before looking in the archive.
pub mod content {
    pub const MESHES: u32 = 0x001;
    pub const TEXTURES: u32 = 0x002;
    pub const MENUS: u32 = 0x004;
    pub const SOUNDS: u32 = 0x008;
    pub const VOICES: u32 = 0x010;
    pub const SHADERS: u32 = 0x020;
    pub const TREES: u32 = 0x040;
    pub const FONTS: u32 = 0x080;
    pub const MISC: u32 = 0x100;
    pub const ALL: u32 = 0x1FF;
}

/// The name hash version 104 archives store (the same as `bsa::hash`,
/// kept here so this crate needs nothing; the assets tests check they
/// agree).
pub fn bsa_hash(name: &str, is_file: bool) -> u64 {
    let name = name.replace('/', "\\").to_ascii_lowercase();
    let bytes = name.as_bytes();
    let (stem, ext) = match name.rfind('.') {
        Some(dot) if is_file => (&bytes[..dot], &bytes[dot..]),
        _ => (bytes, &b""[..]),
    };
    let len = stem.len();
    let mut low: u32 = 0;
    if len > 0 {
        low = u32::from(stem[len - 1])
            | if len > 2 {
                u32::from(stem[len - 2]) << 8
            } else {
                0
            }
            | (len as u32) << 16
            | u32::from(stem[0]) << 24;
    }
    match ext {
        b".kf" => low |= 0x80,
        b".nif" => low |= 0x8000,
        b".dds" => low |= 0x8080,
        b".wav" => low |= 0x8000_0000,
        _ => {}
    }
    let rolling = |b: &[u8]| {
        b.iter().fold(0u32, |h, &c| {
            h.wrapping_mul(0x1003F).wrapping_add(u32::from(c))
        })
    };
    let middle = if len > 2 {
        rolling(&stem[1..len - 2])
    } else {
        0
    };
    (u64::from(middle.wrapping_add(rolling(ext))) << 32) | u64::from(low)
}

/// A folder being put in an archive: its hash, its name and its files
/// (hash, name, data).
type BsaFolder<'a> = (u64, String, Vec<(u64, String, &'a [u8])>);

/// An uncompressed version 104 archive with folder and file names, its
/// folders and files sorted by hash as the game's lookups expect.
pub fn bsa(files: &[(&str, &[u8])], content_flags: u32) -> Vec<u8> {
    let mut folders: Vec<BsaFolder> = Vec::new();
    for (path, data) in files {
        let path = path.replace('/', "\\").to_ascii_lowercase();
        let (folder, name) = path.rsplit_once('\\').unwrap_or(("", path.as_str()));
        let fh = bsa_hash(folder, false);
        let i = match folders.iter().position(|f| f.1 == folder) {
            Some(i) => i,
            None => {
                folders.push((fh, folder.to_string(), Vec::new()));
                folders.len() - 1
            }
        };
        folders[i]
            .2
            .push((bsa_hash(name, true), name.to_string(), data));
    }
    folders.sort_by_key(|f| f.0);
    for f in &mut folders {
        f.2.sort_by_key(|x| x.0);
    }
    let file_count: usize = folders.iter().map(|f| f.2.len()).sum();
    let folder_names: usize = folders.iter().map(|f| f.1.len() + 1).sum();
    let file_names: usize = folders
        .iter()
        .flat_map(|f| &f.2)
        .map(|x| x.1.len() + 1)
        .sum();
    let header_len = 36;
    let records_len = folders.len() * 16;
    let directory =
        header_len + records_len + folders.len() + folder_names + file_count * 16 + file_names;

    let mut out = b"BSA\0".to_vec();
    for v in [
        104u32,
        36,
        // Directory and file names.
        0x3,
        folders.len() as u32,
        file_count as u32,
        folder_names as u32,
        file_names as u32,
        content_flags,
    ] {
        out.extend(v.to_le_bytes());
    }
    // Folder records: hash, count, offset of the folder's name block
    // (counted with the file name table's length, as the game writes it).
    let mut block = header_len + records_len + file_names;
    for (hash, name, list) in &folders {
        out.extend(hash.to_le_bytes());
        out.extend((list.len() as u32).to_le_bytes());
        out.extend((block as u32).to_le_bytes());
        block += 1 + name.len() + 1 + list.len() * 16;
    }
    let mut offset = directory;
    for (_, name, list) in &folders {
        out.push((name.len() + 1) as u8);
        out.extend(name.as_bytes());
        out.push(0);
        for (hash, _, data) in list {
            out.extend(hash.to_le_bytes());
            out.extend((data.len() as u32).to_le_bytes());
            out.extend((offset as u32).to_le_bytes());
            offset += data.len();
        }
    }
    for (_, _, list) in &folders {
        for (_, name, _) in list {
            out.extend(name.as_bytes());
            out.push(0);
        }
    }
    for (_, _, list) in &folders {
        for (_, _, data) in list {
            out.extend(*data);
        }
    }
    out
}

// ---------------------------------------------------------------------------
// Cases
// ---------------------------------------------------------------------------

/// One file of a case: its path under the case folder (`Data\...` for the
/// game's files), its bytes, and its modification time in seconds after
/// 2001-09-09 (`None`: whenever it's written).
#[derive(Debug, Clone)]
pub struct CaseFile {
    pub path: String,
    pub bytes: Vec<u8>,
    pub age: Option<u64>,
}

/// A mod setup and what the game makes of it.
#[derive(Debug, Clone)]
pub struct Case {
    pub name: &'static str,
    /// What the game does with it, for `CASE.txt`.
    pub about: String,
    pub files: Vec<CaseFile>,
}

impl Case {
    fn new(name: &'static str, about: &str) -> Self {
        Self {
            name,
            about: about.to_string(),
            files: Vec::new(),
        }
    }

    fn file(mut self, path: &str, bytes: Vec<u8>, age: Option<u64>) -> Self {
        self.files.push(CaseFile {
            path: path.to_string(),
            bytes,
            age,
        });
        self
    }

    /// Writes the case under `dir` (made if needed): its files, with their
    /// modification times, and `CASE.txt`.
    pub fn write(&self, dir: &Path) -> std::io::Result<()> {
        use std::time::{Duration, SystemTime};
        for f in &self.files {
            let path = dir.join(f.path.replace('\\', "/"));
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::write(&path, &f.bytes)?;
            if let Some(age) = f.age {
                let time = SystemTime::UNIX_EPOCH + Duration::from_secs(1_000_000_000 + age);
                std::fs::File::options()
                    .write(true)
                    .open(&path)?
                    .set_modified(time)?;
            }
        }
        std::fs::create_dir_all(dir)?;
        std::fs::write(dir.join("CASE.txt"), format!("{}\n", self.about))
    }

    /// The case's `Data` folder under `dir`.
    pub fn data(dir: &Path) -> std::path::PathBuf {
        dir.join("Data")
    }
}

/// A stand-in for FalloutNV.esm: one weapon (`000800`), one NPC
/// (`000801`) and an interior cell (`000A00`) with a placed object
/// (`000A01`, the weapon, at 0,0,0).
pub fn main_master() -> Vec<u8> {
    let cell = named(b"CELL", 0x0000_0A00, "TestCell", "Test Cell");
    PluginFile::new(true)
        .top(
            b"WEAP",
            &[named(b"WEAP", 0x0000_0800, "BaseGun", "Base Gun")],
        )
        .top(
            b"NPC_",
            &[named(b"NPC_", 0x0000_0801, "BaseNpc", "Base Npc")],
        )
        .interior(
            cell,
            0x0000_0A00,
            &[placed(0x0000_0A01, 0x0000_0800, [0.0; 3])],
        )
        .build()
}

/// A placed reference to a base object.
pub fn placed(id: u32, base: u32, pos: [f32; 3]) -> Vec<u8> {
    let mut d = sub(b"NAME", &base.to_le_bytes());
    let mut data = Vec::new();
    for v in [pos[0], pos[1], pos[2], 0.0, 0.0, 0.0] {
        data.extend(v.to_le_bytes());
    }
    d.extend(sub(b"DATA", &data));
    form(b"REFR", id, 0, &d)
}

/// A plugin with no records, saved against the stand-in FalloutNV.esm
/// ([`main_master`]) and other masters of unknown size.
fn plain(master: bool, masters: &[&str]) -> Vec<u8> {
    let main = main_master().len() as u64;
    masters
        .iter()
        .fold(PluginFile::new(master), |p, m| {
            p.master(m, if *m == "FalloutNV.esm" { main } else { 0 })
        })
        .build()
}

/// Every case.
pub fn cases() -> Vec<Case> {
    vec![
        load_order(),
        overrides(),
        archives(),
        loose_files(),
        nvse_scripts(),
        doc_house_override(),
    ]
}

/// A case by name.
pub fn case(name: &str) -> Case {
    cases()
        .into_iter()
        .find(|c| c.name == name)
        .unwrap_or_else(|| panic!("no mod case {name}"))
}

/// Which plugins load, and in what order.
pub fn load_order() -> Case {
    let main = main_master();
    let main_size = main.len() as u64;
    let needed = plain(false, &["FalloutNV.esm"]);
    let needed_size = needed.len() as u64;
    Case::new(
        "load-order",
        "Which plugins load and in what order (TESDataHandler::BuildFileList 004624b0,\n\
         Main::LoadPluginsFromFile 00872430, LoadFiles 00463070).\n\
         Active: FalloutNV.esm (sTestFile1), the plugins.txt lines, DeadMoney.esm (its\n\
         DeadMoney.nam), and NeededMaster.esp (a master of an active plugin).\n\
         Order: master-flagged files first, then the rest, each by modification time,\n\
         equal times in reverse name order; plugins.txt's order doesn't count:\n\
         FalloutNV.esm, DeadMoney.esm, MasterFlagged.esp, NeededMaster.esp, Same2.esp,\n\
         Same1.esp, ZMod.esp, AMod.esp.\n\
         Warnings: Missing.esp listed but absent; AMod.esp was saved against a\n\
         FalloutNV.esm of another size.",
    )
    .file("Data\\FalloutNV.esm", main, Some(0))
    .file(
        "Data\\DeadMoney.esm",
        plain(true, &["FalloutNV.esm"]),
        Some(10),
    )
    .file("Data\\DeadMoney.nam", Vec::new(), Some(10))
    .file(
        "Data\\MasterFlagged.esp",
        plain(true, &["FalloutNV.esm"]),
        Some(15),
    )
    .file(
        "Data\\NeededMaster.esp",
        plain(false, &["FalloutNV.esm"]),
        Some(16),
    )
    .file(
        "Data\\ZMod.esp",
        PluginFile::new(false)
            .master("FalloutNV.esm", main_size)
            .master("NeededMaster.esp", needed_size)
            .build(),
        Some(20),
    )
    .file(
        "Data\\AMod.esp",
        PluginFile::new(false)
            .master("FalloutNV.esm", main_size + 1)
            .build(),
        Some(30),
    )
    .file(
        "Data\\Same1.esp",
        plain(false, &["FalloutNV.esm"]),
        Some(17),
    )
    .file(
        "Data\\Same2.esp",
        plain(false, &["FalloutNV.esm"]),
        Some(17),
    )
    .file(
        "Data\\Inactive.esp",
        plain(false, &["FalloutNV.esm"]),
        Some(5),
    )
    .file("Data\\Empty.esp", Vec::new(), Some(6))
    .file(
        "plugins.txt",
        b"# active plugins\r\nAMod.esp\r\nZMod.esp\r\nSame1.esp\r\nSame2.esp\r\n\
          MasterFlagged.esp\r\nMissing.esp\r\nEmpty.esp\r\n"
            .to_vec(),
        None,
    )
}

/// Masters, form ID renumbering and overrides across several plugins.
pub fn overrides() -> Case {
    let main = main_master();
    // Dlc.esm: renames the weapon, adds its own (local 01 = itself).
    let dlc = PluginFile::new(true)
        .master("FalloutNV.esm", main.len() as u64)
        .top(
            b"WEAP",
            &[
                named(b"WEAP", 0x0000_0800, "BaseGun", "Base Gun (Dlc)"),
                named(b"WEAP", 0x0100_0900, "DlcGun", "Dlc Gun"),
            ],
        )
        .build();
    // ModB.esp lists only Dlc.esm: its 00 is Dlc.esm, 01 itself.
    let mod_b = PluginFile::new(false)
        .master("Dlc.esm", dlc.len() as u64)
        .top(
            b"WEAP",
            &[
                named(b"WEAP", 0x0000_0900, "DlcGun", "Dlc Gun (ModB)"),
                named(b"WEAP", 0x0100_0A00, "ModBGun", "ModB Gun"),
            ],
        )
        .build();
    // ModC.esp lists its masters in another order (ModB first): 00 is
    // ModB, 01 FalloutNV, 02 itself; an index past the end means itself.
    // It renames the NPC, moves the placed weapon and places ModB's gun.
    let cell = named(b"CELL", 0x0100_0A00, "TestCell", "Test Cell (ModC)");
    let mod_c = PluginFile::new(false)
        .master("ModB.esp", mod_b.len() as u64)
        .master("FalloutNV.esm", main.len() as u64)
        .top(
            b"NPC_",
            &[
                named(b"NPC_", 0x0100_0801, "BaseNpc", "Base Npc (ModC)"),
                named(b"NPC_", 0x0700_0B00, "ModCNpc", "ModC Npc"),
            ],
        )
        .interior(
            cell,
            0x0100_0A00,
            &[
                placed(0x0100_0A01, 0x0100_0800, [100.0, 0.0, 0.0]),
                placed(0x0200_0C00, 0x0000_0A00, [0.0, 50.0, 0.0]),
            ],
        )
        .build();
    Case::new(
        "overrides",
        "Form IDs renumbered by load order and the last-loaded version of a record\n\
         winning (TESFile master list; LoadFiles 00463070). Load order FalloutNV.esm 00,\n\
         Dlc.esm 01, ModB.esp 02, ModC.esp 03. 00000800 is \"Base Gun (Dlc)\";\n\
         01000900 \"Dlc Gun (ModB)\"; 02000A00 \"ModB Gun\"; 00000801 \"Base Npc (ModC)\";\n\
         03000B00 \"ModC Npc\" (written 07000B00: past the master list = the file\n\
         itself); the test cell 00000A00 is named \"Test Cell (ModC)\" and holds 00000A01\n\
         moved to 100,0,0 and 03000C00, a placed ModB Gun.",
    )
    .file("Data\\FalloutNV.esm", main_master(), Some(0))
    .file("Data\\Dlc.esm", dlc, Some(10))
    .file("Data\\ModB.esp", mod_b, Some(20))
    .file("Data\\ModC.esp", mod_c, Some(30))
    .file(
        "plugins.txt",
        b"FalloutNV.esm\r\nDlc.esm\r\nModB.esp\r\nModC.esp\r\n".to_vec(),
        None,
    )
}

/// Which archive a file comes from.
pub fn archives() -> Case {
    let a = |files: &[(&str, &[u8])]| bsa(files, content::ALL);
    let tex = "textures\\test\\shared.dds";
    Case::new(
        "archives",
        "Archive load order and priority (ArchiveManager::OpenArchive 00af4be0,\n\
         GetArchiveForFile 00af6160). Opened: the SArchiveList archives, then Update.bsa,\n\
         then for each loaded plugin every Data\\<name without extension>*.bsa.\n\
         The first archive in the manager's list that holds a file wins; archives\n\
         whose name has \"Fallo\" go to the end of the list, the others before the first\n\
         official one (by the DLC rank in their names). For textures\\test\\shared.dds:\n\
         ModA.bsa (loaded first of the mods) wins over ModAExtra.bsa, ModB.bsa,\n\
         DeadMoney - Main.bsa, Update.bsa and the Fallout archives; among the Fallout\n\
         archives the first listed wins (Fallout - Textures.bsa over Textures2).\n\
         Unloaded: Unlisted.bsa, Empty.bsa (size 0), ModZ.bsa (ModZ.esp inactive).",
    )
    .file("Data\\FalloutNV.esm", main_master(), Some(0))
    .file(
        "Data\\DeadMoney.esm",
        plain(true, &["FalloutNV.esm"]),
        Some(10),
    )
    .file("Data\\ModA.esp", plain(false, &["FalloutNV.esm"]), Some(20))
    .file("Data\\ModB.esp", plain(false, &["FalloutNV.esm"]), Some(30))
    .file("Data\\ModZ.esp", plain(false, &["FalloutNV.esm"]), Some(40))
    .file(
        "Data\\Fallout - Textures.bsa",
        a(&[(tex, b"Fallout - Textures")]),
        None,
    )
    .file(
        "Data\\Fallout - Textures2.bsa",
        a(&[(tex, b"Fallout - Textures2")]),
        None,
    )
    .file("Data\\Update.bsa", a(&[(tex, b"Update")]), None)
    .file(
        "Data\\DeadMoney - Main.bsa",
        a(&[(tex, b"DeadMoney - Main")]),
        None,
    )
    .file("Data\\ModA.bsa", a(&[(tex, b"ModA")]), None)
    .file("Data\\ModAExtra.bsa", a(&[(tex, b"ModAExtra")]), None)
    .file("Data\\ModB.bsa", a(&[(tex, b"ModB")]), None)
    .file("Data\\ModZ.bsa", a(&[(tex, b"ModZ")]), None)
    .file("Data\\Unlisted.bsa", a(&[(tex, b"Unlisted")]), None)
    .file("Data\\Empty.bsa", Vec::new(), None)
    .file(
        "Fallout.ini",
        b"[Archive]\r\nSArchiveList=Fallout - Textures.bsa, Fallout - Textures2.bsa\r\n".to_vec(),
        None,
    )
    .file(
        "plugins.txt",
        b"FalloutNV.esm\r\nDeadMoney.esm\r\nModA.esp\r\nModB.esp\r\n".to_vec(),
        None,
    )
}

/// Loose files against archived ones.
pub fn loose_files() -> Case {
    let packed = bsa(
        &[
            ("textures\\a\\loose.dds", b"archived"),
            ("textures\\a\\other.dds", b"archived"),
            ("textures\\b\\listed.dds", b"archived"),
            ("textures\\c\\inside.dds", b"archived"),
        ],
        content::ALL,
    );
    Case::new(
        "loose-files",
        "Loose files against archived ones (FileFinder 00afe220, Archive::CheckInvalidateFile\n\
         00afb190, ArchiveManager::LoadInvalidationFile 00af5ab0, Archive 00afad00).\n\
         Archives are asked first. With bInvalidateOlderFiles=1 (the exe's default) an\n\
         archived file is dropped when Data\\<its path> exists loose, whatever the times:\n\
         textures\\a\\loose.dds comes from the loose file. textures\\a\\other.dds has no\n\
         loose copy: archived. ArchiveInvalidation.txt drops archived entries when an\n\
         archive opens: a line with a folder (textures\\c\\anything.dds) drops that whole\n\
         folder, so textures\\c\\inside.dds can't be found at all; a bare name\n\
         (listed.dds) drops files of that name in folders that exist loose under Data\n\
         (textures\\b does), so textures\\b\\listed.dds can't be found either.\n\
         With bInvalidateOlderFiles=0 (Fallout-off.ini) nothing is dropped and the\n\
         archive wins for every file.",
    )
    .file("Data\\FalloutNV.esm", main_master(), Some(0))
    .file("Data\\Fallout - Textures.bsa", packed, Some(100))
    .file("Data\\textures\\a\\loose.dds", b"loose".to_vec(), Some(50))
    .file(
        "Data\\textures\\b\\unrelated.dds",
        b"loose".to_vec(),
        Some(50),
    )
    .file(
        "Data\\ArchiveInvalidation.txt",
        b"listed.dds\r\ntextures\\c\\anything.dds\r\n".to_vec(),
        None,
    )
    .file(
        "Fallout.ini",
        b"[Archive]\r\nSArchiveList=Fallout - Textures.bsa\r\nbInvalidateOlderFiles=1\r\n\
          SInvalidationFile=ArchiveInvalidation.txt\r\n"
            .to_vec(),
        None,
    )
    .file(
        "Fallout-off.ini",
        b"[Archive]\r\nSArchiveList=Fallout - Textures.bsa\r\nbInvalidateOlderFiles=0\r\n\
          SInvalidationFile=ArchiveInvalidation.txt\r\n"
            .to_vec(),
        None,
    )
}

/// A compiled script calling script extender functions.
pub fn nvse_script() -> Vec<u8> {
    let mut bytes = vec![
        0x1D, 0x00, 0x00, 0x00, // ScriptName
        0x10, 0x00, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Begin GameMode
        0x00, 0x14, 0x02, 0x00, 0x00, 0x00, // GetNVSEVersion
    ];
    // if GetNVSERevision (an expression stored the game's way)
    let expr = [b' ', b'X', 0x01, 0x14, 0x02, 0x00, 0x00, 0x00];
    bytes.extend([0x16, 0x00]);
    bytes.extend(((4 + expr.len()) as u16).to_le_bytes());
    bytes.extend([0x00, 0x00]);
    bytes.extend((expr.len() as u16).to_le_bytes());
    bytes.extend(expr);
    // let ... := ar_List (xNVSE's expression), then a plugin's function.
    let nvse = [0x01, 0x01, b'X', 0x00, 0x00, 0x67, 0x15, 0x02, 0x00];
    bytes.extend([0x39, 0x15]);
    bytes.extend((nvse.len() as u16).to_le_bytes());
    bytes.extend(nvse);
    bytes.extend([0x00, 0x26, 0x02, 0x00, 0x00, 0x00]); // opcode 0x2600
    bytes.extend([0x11, 0x00, 0x00, 0x00]); // End
    bytes
}

/// A plugin whose script calls script extender functions.
pub fn nvse_scripts() -> Case {
    let mut d = sub(b"EDID", &zstr("NvseUserScript"));
    d.extend(sub(b"SCDA", &nvse_script()));
    d.extend(sub(b"SCTX", b"scn NvseUserScript\r\nbegin GameMode\r\nGetNVSEVersion\r\nif GetNVSERevision\r\nendif\r\nlet aArray := ar_List\r\nSomePluginFunction\r\nend\0"));
    let plugin = PluginFile::new(false)
        .master("FalloutNV.esm", main_master().len() as u64)
        .top(b"SCPT", &[form(b"SCPT", 0x0100_0800, 0, &d)])
        .build();
    Case::new(
        "nvse-scripts",
        "A script calling xNVSE functions (GetNVSEVersion 0x1400, GetNVSERevision 0x1401,\n\
         Let 0x1539 with ar_List 0x1567 inside) and an NVSE plugin's opcode (0x2600).\n\
         `nvinspect Data\\NvseUser.esp coverage nvse` lists them.",
    )
    .file("Data\\FalloutNV.esm", main_master(), Some(0))
    .file("Data\\NvseUser.esp", plugin, Some(10))
}

/// A plugin for the real game: it changes the model of the lit wall lamps
/// in Doc Mitchell's house (`dlc04lightwall01on`, `000F1E85`, a static the
/// hall's first view shows) to the cave chandelier
/// (`Dungeons\Caves\Lamplight\LamplightChandelier.NIF`). Only form IDs,
/// an editor ID and a model path from the game are named; nothing of its
/// data is copied.
pub fn doc_house_override() -> Case {
    let mut d = sub(b"EDID", &zstr("dlc04lightwall01on"));
    d.extend(sub(b"OBND", &[0; 12]));
    d.extend(sub(
        b"MODL",
        &zstr("Dungeons\\Caves\\Lamplight\\LamplightChandelier.NIF"),
    ));
    let plugin = PluginFile::new(false)
        .master_unsized("FalloutNV.esm")
        .top(b"STAT", &[form(b"STAT", 0x000F_1E85, 0, &d)])
        .build();
    Case::new(
        "doc-house-override",
        "For the real game's data: DocHouseOverride.esp replaces the model of the lit wall\n\
         lamps (STAT 000F1E85) with the cave chandelier. Link the game's Data files into\n\
         this Data folder (they're not copied here), then e.g.\n\
         nv-viewer <this>\\Data GSDocMitchellHouse --plugins <this>\\plugins.txt",
    )
    .file("Data\\DocHouseOverride.esp", plugin, None)
    .file(
        "plugins.txt",
        b"FalloutNV.esm\r\nDocHouseOverride.esp\r\n".to_vec(),
        None,
    )
}

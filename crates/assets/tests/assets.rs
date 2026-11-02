//! Asset lookup tests against a Data folder built in a temp directory.

use std::fs;
use std::path::{Path, PathBuf};

use assets::{archive_list_from, read_archive_list, ArchiveReason, Assets, Source};

/// Files grouped by folder: (folder, [(file name, contents)]).
type Folders<'a> = Vec<(String, Vec<(String, &'a [u8])>)>;

/// A minimal uncompressed BSA holding the given (path, contents) files.
fn bsa(files: &[(&str, &[u8])]) -> Vec<u8> {
    let mut folders: Folders = Vec::new();
    for (path, data) in files {
        let (folder, name) = path.rsplit_once('\\').unwrap();
        match folders.iter_mut().find(|(f, _)| f == folder) {
            Some((_, list)) => list.push((name.to_string(), data)),
            None => folders.push((folder.to_string(), vec![(name.to_string(), data)])),
        }
    }
    let total_folder_names: usize = folders.iter().map(|(f, _)| f.len() + 1).sum();
    let total_file_names: usize = files
        .iter()
        .map(|(p, _)| p.rsplit_once('\\').unwrap().1.len() + 1)
        .sum();
    let directory =
        36 + folders.len() * 17 + total_folder_names + files.len() * 16 + total_file_names;

    let mut out = b"BSA\0".to_vec();
    for v in [
        104u32,
        36,
        3,
        folders.len() as u32,
        files.len() as u32,
        total_folder_names as u32,
        total_file_names as u32,
        0,
    ] {
        out.extend(v.to_le_bytes());
    }
    for (_, list) in &folders {
        out.extend(0u64.to_le_bytes());
        out.extend((list.len() as u32).to_le_bytes());
        out.extend(0u32.to_le_bytes());
    }
    let mut offset = directory;
    for (folder, list) in &folders {
        out.push((folder.len() + 1) as u8);
        out.extend(folder.as_bytes());
        out.push(0);
        for (_, data) in list {
            out.extend(0u64.to_le_bytes());
            out.extend((data.len() as u32).to_le_bytes());
            out.extend((offset as u32).to_le_bytes());
            offset += data.len();
        }
    }
    for (_, list) in &folders {
        for (name, _) in list {
            out.extend(name.as_bytes());
            out.push(0);
        }
    }
    for (_, list) in &folders {
        for (_, data) in list {
            out.extend(*data);
        }
    }
    out
}

struct TempData(PathBuf);

impl TempData {
    fn new(tag: &str) -> Self {
        let path = std::env::temp_dir().join(format!("nv-rs-assets-{tag}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&path);
        fs::create_dir_all(&path).unwrap();
        TempData(path)
    }

    fn write(&self, relative: &str, bytes: &[u8]) {
        let path = self.0.join(relative);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, bytes).unwrap();
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TempData {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn plugins(names: &[&str]) -> Vec<String> {
    names.iter().map(|s| s.to_string()).collect()
}

fn setup(tag: &str) -> TempData {
    let data = TempData::new(tag);
    data.write(
        "Fallout - Meshes.bsa",
        &bsa(&[("meshes\\clutter\\crate.nif", b"crate mesh")]),
    );
    data.write(
        "Fallout - Textures.bsa",
        &bsa(&[
            ("textures\\clutter\\crate.dds", b"base"),
            ("textures\\sky\\stars.dds", b"stars"),
        ]),
    );
    data.write(
        "DeadMoney - Main.bsa",
        &bsa(&[("textures\\clutter\\crate.dds", b"dlc")]),
    );
    data.write(
        "MyMod.bsa",
        &bsa(&[("meshes\\mymod\\thing.nif", b"mod mesh")]),
    );
    data.write("Unused.bsa", &bsa(&[("textures\\unused.dds", b"nope")]));
    data.write("FalloutNV.esm", b"not read by this crate");
    data.write("readme.txt", b"root files aren't assets");
    data.write("Textures/Sky/Stars.dds", b"loose stars");
    data
}

#[test]
fn loads_archives_in_game_order() {
    let data = setup("order");
    let assets = Assets::open(
        data.path(),
        &plugins(&["FalloutNV.esm", "DeadMoney.esm", "MyMod.esp"]),
    )
    .unwrap();
    let names: Vec<&str> = assets.archives().iter().map(|a| a.name.as_str()).collect();
    assert_eq!(
        names,
        [
            "Fallout - Textures.bsa",
            "Fallout - Meshes.bsa",
            "DeadMoney - Main.bsa",
            "MyMod.bsa"
        ]
    );
    assert_eq!(assets.archives()[0].reason, ArchiveReason::Default);
    assert_eq!(
        assets.archives()[2].reason,
        ArchiveReason::Plugin("DeadMoney.esm".into())
    );
    assert_eq!(assets.unused_archives(), ["Unused.bsa"]);
    assert!(!assets.contains("textures\\unused.dds"));
}

#[test]
fn later_archives_and_loose_files_win() {
    let data = setup("override");
    let assets = Assets::open(data.path(), &plugins(&["FalloutNV.esm", "DeadMoney.esm"])).unwrap();

    assert_eq!(
        assets
            .read("textures\\clutter\\crate.dds")
            .unwrap()
            .unwrap(),
        b"dlc"
    );
    let versions: Vec<String> = assets
        .versions("textures\\clutter\\crate.dds")
        .iter()
        .map(Source::describe)
        .collect();
    assert_eq!(versions, ["DeadMoney - Main.bsa", "Fallout - Textures.bsa"]);

    // A loose file beats every archive, whatever its case on disk.
    assert_eq!(
        assets.read("Textures/Sky/STARS.dds").unwrap().unwrap(),
        b"loose stars"
    );
    let versions: Vec<String> = assets
        .versions("textures\\sky\\stars.dds")
        .iter()
        .map(Source::describe)
        .collect();
    assert_eq!(versions, ["loose file", "Fallout - Textures.bsa"]);
}

#[test]
fn archives_for_inactive_plugins_are_not_loaded() {
    let data = setup("inactive");
    let assets = Assets::open(data.path(), &plugins(&["FalloutNV.esm"])).unwrap();
    assert_eq!(
        assets
            .read("textures\\clutter\\crate.dds")
            .unwrap()
            .unwrap(),
        b"base"
    );
    assert!(!assets.contains("meshes\\mymod\\thing.nif"));
    assert_eq!(assets.unused_archives().len(), 3);
}

#[test]
fn lists_every_visible_path_once() {
    let data = setup("paths");
    let assets = Assets::open(
        data.path(),
        &plugins(&["FalloutNV.esm", "DeadMoney.esm", "MyMod.esp"]),
    )
    .unwrap();
    let mut paths: Vec<&str> = assets.paths().collect();
    paths.sort();
    assert_eq!(
        paths,
        [
            "meshes\\clutter\\crate.nif",
            "meshes\\mymod\\thing.nif",
            "textures\\clutter\\crate.dds",
            "textures\\sky\\stars.dds",
        ]
    );
    assert_eq!(assets.len(), 4);
    assert_eq!(assets.loose_count(), 1);
    assert!(assets.locate("readme.txt").is_none());
    assert!(assets.read("textures\\missing.dds").unwrap().is_none());
}

#[test]
fn leaves_out_unreadable_archives_with_a_warning() {
    // The game drops an archive it can't open (ArchiveManager::OpenArchive
    // 00af4be0) and goes on.
    let data = setup("broken");
    data.write("Fallout - Misc.bsa", b"BSA\0 truncated");
    let assets = Assets::open(data.path(), &plugins(&["FalloutNV.esm"])).unwrap();
    assert_eq!(assets.warnings().len(), 1);
    assert!(assets.warnings()[0].contains("Fallout - Misc.bsa"));
    assert!(assets
        .archives()
        .iter()
        .all(|a| a.name != "Fallout - Misc.bsa"));
    assert!(assets.contains("meshes\\clutter\\crate.nif"));
}

#[test]
fn reads_the_archive_list_from_ini_files() {
    let data = TempData::new("ini");
    data.write(
        "Fallout.ini",
        b"[General]\r\nsStartingCell=\r\n[Archive]\r\n; comment\r\nSArchiveList = Fallout - Textures.bsa,  Update.bsa ,Fallout - Meshes.bsa\r\n",
    );
    data.write("Other.ini", b"[Archive]\nbInvalidateOlderFiles=1\n");
    // Settings: later files win, names compare without case.
    data.write(
        "Prefs.ini",
        b"[Landscape]\r\nfLandTextureTilingMult=3.5\r\n[Display]\r\nfSunBaseSize=750\r\n",
    );
    let settings = assets::IniSettings::load(&[
        data.path().join("Fallout.ini"),
        data.path().join("Missing.ini"),
        data.path().join("Prefs.ini"),
    ]);
    assert_eq!(
        settings.float("landscape", "FLANDTEXTURETILINGMULT"),
        Some(3.5)
    );
    assert_eq!(settings.get("General", "sStartingCell"), Some(""));
    assert_eq!(settings.float("Display", "fSunBaseSize"), Some(750.0));
    assert_eq!(settings.get("Display", "nothing"), None);
    let list = read_archive_list(&data.path().join("Fallout.ini")).unwrap();
    assert_eq!(
        list,
        [
            "Fallout - Textures.bsa",
            "Update.bsa",
            "Fallout - Meshes.bsa"
        ]
    );
    assert!(read_archive_list(&data.path().join("Other.ini")).is_none());
    assert!(read_archive_list(&data.path().join("Missing.ini")).is_none());

    // The first candidate that has the setting wins.
    let candidates = [
        data.path().join("Missing.ini"),
        data.path().join("Other.ini"),
        data.path().join("Fallout.ini"),
    ];
    let chosen = archive_list_from(&candidates);
    assert_eq!(chosen.names.len(), 3);
    assert!(chosen.source.ends_with("Fallout.ini"), "{}", chosen.source);
    let fallback = archive_list_from(&candidates[..2]);
    assert_eq!(fallback.names.len(), 6);
    assert!(fallback.source.contains("built-in"));
}

#[test]
fn loads_archives_from_the_given_list_and_traces_the_rest() {
    let data = setup("list");
    data.write(
        "OldMod.bsa",
        &bsa(&[("meshes\\old\\statue.nif", b"statue")]),
    );
    let plugins = plugins(&["FalloutNV.esm"]);

    let without = Assets::open(data.path(), &plugins).unwrap();
    assert!(!without.contains("meshes\\old\\statue.nif"));
    assert_eq!(
        without.in_unused_archives("Meshes/Old/Statue.nif"),
        ["OldMod.bsa"]
    );

    let list: Vec<String> = [
        "Fallout - Meshes.bsa",
        "OldMod.bsa",
        "Fallout - Textures.bsa",
    ]
    .map(String::from)
    .to_vec();
    let with = Assets::open_with(data.path(), &plugins, &list).unwrap();
    let names: Vec<&str> = with.archives().iter().map(|a| a.name.as_str()).collect();
    assert_eq!(
        names,
        [
            "Fallout - Meshes.bsa",
            "OldMod.bsa",
            "Fallout - Textures.bsa"
        ]
    );
    assert!(with.contains("meshes\\old\\statue.nif"));
    assert!(with
        .in_unused_archives("meshes\\old\\statue.nif")
        .is_empty());
}

#[test]
fn loads_the_patch_archive_after_the_list() {
    let data = setup("patch");
    data.write("Update.bsa", &bsa(&[("meshes\\nv\\tower.nif", b"tower")]));
    let list: Vec<String> = ["Fallout - Textures.bsa", "Fallout - Meshes.bsa"]
        .map(String::from)
        .to_vec();
    let assets = Assets::open_with(
        data.path(),
        &plugins(&["FalloutNV.esm", "DeadMoney.esm"]),
        &list,
    )
    .unwrap();
    let names: Vec<&str> = assets.archives().iter().map(|a| a.name.as_str()).collect();
    assert_eq!(
        names,
        [
            "Fallout - Textures.bsa",
            "Fallout - Meshes.bsa",
            "Update.bsa",
            "DeadMoney - Main.bsa"
        ]
    );
    assert_eq!(assets.archives()[2].reason, ArchiveReason::Patch);
    assert!(assets.contains("meshes\\nv\\tower.nif"));
}

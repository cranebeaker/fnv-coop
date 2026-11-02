//! The mod setups of `testdata::mods` that concern archives and loose
//! files, checked against what the game does with them (docs/MODS.md).

use assets::{archive_priority, ArchiveSettings, Assets, IniSettings, Invalidation, Source};
use testdata::mods::{bsa_hash, case};
use testdata::TempData;

fn plugins(names: &[&str]) -> Vec<String> {
    names.iter().map(|s| s.to_string()).collect()
}

/// A case written to a fresh folder, and its INI settings (`ini` names
/// the INI file in the case).
fn written(name: &str, ini: &str) -> (TempData, ArchiveSettings) {
    let dir = TempData::empty(&format!("mods-{name}"));
    case(name).write(dir.path()).unwrap();
    let ini = IniSettings::load(&[dir.path().join(ini)]);
    let list = assets::read_archive_list(&dir.path().join("Fallout.ini")).unwrap();
    (dir, ArchiveSettings::from_ini(&ini, list))
}

fn read(assets: &Assets, path: &str) -> Option<String> {
    assets
        .read(path)
        .unwrap()
        .map(|b| String::from_utf8(b).unwrap())
}

#[test]
fn the_generated_archives_hash_names_as_the_game_does() {
    for name in ["shared.dds", "a.nif", "x.kf", "long_name.wav", "noext"] {
        assert_eq!(
            bsa_hash(name, true),
            bsa::hash::hash_file_name(name),
            "{name}"
        );
    }
    for folder in ["textures\\test", "meshes", "a"] {
        assert_eq!(bsa_hash(folder, false), bsa::hash::hash_folder_name(folder));
    }
    let bytes = testdata::mods::bsa(&[("textures\\test\\shared.dds", b"x")], 0x2);
    let archive = bsa::Archive::from_bytes(bytes).unwrap();
    let check = archive.check_name_hashes();
    assert_eq!(check.files_matched, check.files_total);
    assert_eq!(check.folders_matched, check.folders_total);
}

#[test]
fn archive_priority_follows_open_archive() {
    // Opened in this order; the list the manager keeps (00af4be0).
    let names = [
        "Fallout - Textures.bsa",
        "Fallout - Textures2.bsa",
        "Update.bsa",
        "DeadMoney - Main.bsa",
        "HonestHearts - Main.bsa",
        "OldWorldBlues - Main.bsa",
        "LonesomeRoad - Main.bsa",
        "DeadMoney - Sounds.bsa",
        "ModA.bsa",
        "ModB.bsa",
    ];
    let order: Vec<&str> = archive_priority(&names)
        .into_iter()
        .map(|i| names[i])
        .collect();
    assert_eq!(
        order,
        [
            "ModA.bsa",
            "ModB.bsa",
            "OldWorldBlues - Main.bsa",
            "Update.bsa",
            "LonesomeRoad - Main.bsa",
            "HonestHearts - Main.bsa",
            "DeadMoney - Sounds.bsa",
            "DeadMoney - Main.bsa",
            "Fallout - Textures.bsa",
            "Fallout - Textures2.bsa",
        ]
    );
    // Case matters: a lower-case "fallout" archive ranks as a mod's.
    let names = ["Fallout - Meshes.bsa", "fallout - extra.bsa"];
    assert_eq!(archive_priority(&names), [1, 0]);
}

#[test]
fn archives_open_and_win_in_the_games_order() {
    let (dir, settings) = written("archives", "Fallout.ini");
    let data = testdata::mods::Case::data(dir.path());
    let assets = Assets::open_with_settings(
        &data,
        &plugins(&["FalloutNV.esm", "DeadMoney.esm", "ModA.esp", "ModB.esp"]),
        &settings,
    )
    .unwrap();
    let opened: Vec<&str> = assets.archives().iter().map(|a| a.name.as_str()).collect();
    assert_eq!(
        opened,
        [
            "Fallout - Textures.bsa",
            "Fallout - Textures2.bsa",
            "Update.bsa",
            "DeadMoney - Main.bsa",
            "ModA.bsa",
            "ModAExtra.bsa",
            "ModB.bsa",
        ]
    );
    let by_priority: Vec<&str> = assets.by_priority().map(|a| a.name.as_str()).collect();
    assert_eq!(
        by_priority,
        [
            "ModA.bsa",
            "ModAExtra.bsa",
            "ModB.bsa",
            "Update.bsa",
            "DeadMoney - Main.bsa",
            "Fallout - Textures.bsa",
            "Fallout - Textures2.bsa",
        ]
    );
    let shared = "textures\\test\\shared.dds";
    assert_eq!(read(&assets, shared).as_deref(), Some("ModA"));
    let versions: Vec<String> = assets
        .versions(shared)
        .iter()
        .map(Source::describe)
        .collect();
    assert_eq!(versions[..2], ["ModA.bsa", "ModAExtra.bsa"]);
    let mut unused = assets.unused_archives().to_vec();
    unused.sort();
    assert_eq!(unused, ["Empty.bsa", "ModZ.bsa", "Unlisted.bsa"]);
    assert!(assets.warnings().is_empty());
}

#[test]
fn no_archives_with_use_archives_off() {
    let (dir, mut settings) = written("archives", "Fallout.ini");
    settings.use_archives = false;
    let data = testdata::mods::Case::data(dir.path());
    let assets =
        Assets::open_with_settings(&data, &plugins(&["FalloutNV.esm", "ModA.esp"]), &settings)
            .unwrap();
    assert!(assets.archives().is_empty());
    assert_eq!(read(&assets, "textures\\test\\shared.dds"), None);
}

#[test]
fn loose_files_win_with_invalidate_older_files() {
    let (dir, settings) = written("loose-files", "Fallout.ini");
    assert!(settings.invalidate_older_files);
    let data = testdata::mods::Case::data(dir.path());
    let assets =
        Assets::open_with_settings(&data, &plugins(&["FalloutNV.esm"]), &settings).unwrap();
    // Loose, though older than the archive.
    assert_eq!(
        read(&assets, "textures\\a\\loose.dds").as_deref(),
        Some("loose")
    );
    assert_eq!(
        read(&assets, "textures\\a\\other.dds").as_deref(),
        Some("archived")
    );
    // Dropped by ArchiveInvalidation.txt: the folder line, and the name
    // line where the folder exists loose.
    assert_eq!(read(&assets, "textures\\c\\inside.dds"), None);
    assert_eq!(read(&assets, "textures\\b\\listed.dds"), None);
    assert!(assets.versions("textures\\c\\inside.dds").is_empty());
}

#[test]
fn archives_win_without_invalidate_older_files() {
    let (dir, settings) = written("loose-files", "Fallout-off.ini");
    assert!(!settings.invalidate_older_files);
    let data = testdata::mods::Case::data(dir.path());
    let assets =
        Assets::open_with_settings(&data, &plugins(&["FalloutNV.esm"]), &settings).unwrap();
    assert_eq!(
        read(&assets, "textures\\a\\loose.dds").as_deref(),
        Some("archived")
    );
    assert_eq!(
        read(&assets, "textures\\c\\inside.dds").as_deref(),
        Some("archived")
    );
    assert_eq!(
        read(&assets, "textures\\b\\listed.dds").as_deref(),
        Some("archived")
    );
    // Loose files still fill in what no archive has.
    assert_eq!(
        read(&assets, "textures\\b\\unrelated.dds").as_deref(),
        Some("loose")
    );
    let versions: Vec<String> = assets
        .versions("textures\\a\\loose.dds")
        .iter()
        .map(Source::describe)
        .collect();
    assert_eq!(versions, ["Fallout - Textures.bsa", "loose file"]);
}

#[test]
fn reads_the_invalidation_file_like_the_game() {
    let list = Invalidation::parse(
        b"Crate.DDS\r\n\\Textures\\Clutter\\x.dds\r\nmeshes/a/b.nif\r\n\r\nnoline",
    );
    assert_eq!(
        list.files.iter().map(String::as_str).collect::<Vec<_>>(),
        ["b.nif", "crate.dds", "noline"]
    );
    assert_eq!(
        list.folders.iter().map(String::as_str).collect::<Vec<_>>(),
        ["textures\\clutter"]
    );
    // Only carriage returns end lines (a lone line feed doesn't).
    let unix = Invalidation::parse(b"a.dds\nb.dds\n");
    assert_eq!(unix.files.len(), 1);
    assert!(list.drops("Textures\\Clutter", "any.nif", |_| false));
    assert!(list.drops("meshes\\x", "crate.dds", |_| true));
    assert!(!list.drops("meshes\\x", "crate.dds", |_| false));
}

#[test]
fn reads_the_archive_settings_with_the_exes_defaults() {
    let mut ini = IniSettings::default();
    let defaults = ArchiveSettings::from_ini(&ini, Vec::new());
    assert!(defaults.use_archives && defaults.invalidate_older_files);
    assert_eq!(defaults.invalidation_file, "ArchiveInvalidation.txt");
    ini.add("[Archive]\r\nbInvalidateOlderFiles=0\r\nbUseArchives=0\r\nSInvalidationFile=\r\n");
    let set = ArchiveSettings::from_ini(&ini, Vec::new());
    assert!(!set.use_archives && !set.invalidate_older_files);
    assert_eq!(set.invalidation_file, "");
}

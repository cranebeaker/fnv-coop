//! Load-order tests: form ID renumbering, overrides, sorting and errors.

use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

use esm::load_order::parse_plugins_txt;
use esm::{flags, sig, ActivePlugins, Error, FormId, LoadOrder, Plugin};

mod common;
use common::*;

fn named_record(kind: &[u8; 4], form_id: u32, editor_id: &str, name: &str) -> Vec<u8> {
    let mut data = sub(b"EDID", &zstr(editor_id));
    data.extend(sub(b"FULL", &zstr(name)));
    record(kind, form_id, 0, &data)
}

/// Base.esm: a weapon and an NPC.
fn base() -> Vec<u8> {
    let mut file = tes4(flags::MASTER, &[], "t");
    file.extend(group(
        *b"WEAP",
        0,
        &named_record(b"WEAP", 0x0000_0800, "BaseGun", "Base Gun"),
    ));
    file.extend(group(
        *b"NPC_",
        0,
        &named_record(b"NPC_", 0x0000_0801, "BaseNpc", "Base Npc"),
    ));
    file
}

/// Dlc.esm: overrides Base's weapon and adds one of its own.
fn dlc(masters: &[&str]) -> Vec<u8> {
    let mut weapons = named_record(b"WEAP", 0x0000_0800, "BaseGun", "Base Gun (DLC)");
    weapons.extend(named_record(b"WEAP", 0x0100_0900, "DlcGun", "Dlc Gun"));
    let mut file = tes4(flags::MASTER, masters, "t");
    file.extend(group(*b"WEAP", 0, &weapons));
    file
}

/// ModB.esp: lists only Dlc.esm as a master, so its local index 00 means
/// Dlc.esm (load index 1) and 01 means itself (load index 2).
fn mod_b() -> Vec<u8> {
    let mut weapons = named_record(b"WEAP", 0x0000_0900, "DlcGun", "Dlc Gun (Mod)");
    weapons.extend(named_record(b"WEAP", 0x0100_0A00, "ModGun", "Mod Gun"));
    let mut file = tes4(0, &["Dlc.esm"], "t");
    file.extend(group(*b"WEAP", 0, &weapons));
    file
}

fn p(bytes: Vec<u8>) -> Plugin {
    Plugin::from_bytes(bytes).unwrap()
}

fn three_plugins() -> LoadOrder {
    LoadOrder::from_plugins(vec![
        ("Base.esm".into(), None, p(base())),
        ("Dlc.esm".into(), None, p(dlc(&["Base.esm"]))),
        ("ModB.esp".into(), None, p(mod_b())),
    ])
    .unwrap()
}

fn name_of(order: &LoadOrder, id: u32) -> Option<String> {
    order.get(FormId(id)).unwrap().record().unwrap().full_name()
}

#[test]
fn later_plugins_override_earlier_ones() {
    let order = three_plugins();
    assert_eq!(
        name_of(&order, 0x0000_0800).as_deref(),
        Some("Base Gun (DLC)")
    );
    let versions: Vec<_> = order
        .versions(FormId(0x0000_0800))
        .iter()
        .map(|v| v.plugin.name.clone())
        .collect();
    assert_eq!(versions, ["Base.esm", "Dlc.esm"]);
    assert_eq!(order.versions(FormId(0x0000_0801)).len(), 1);
}

#[test]
fn form_ids_are_renumbered_to_load_order() {
    let order = three_plugins();
    // ModB's local 00000900 is Dlc.esm's 01000900.
    let gun = order.get(FormId(0x0100_0900)).unwrap();
    assert_eq!(gun.plugin.name, "ModB.esp");
    assert_eq!(
        name_of(&order, 0x0100_0900).as_deref(),
        Some("Dlc Gun (Mod)")
    );
    // ModB's own record gets load index 02.
    assert_eq!(name_of(&order, 0x0200_0A00).as_deref(), Some("Mod Gun"));
    assert!(order.get(FormId(0x0100_0A00)).is_none());
    assert_eq!(order.slot_name(2), Some("ModB.esp"));
    assert_eq!(order.slot_name(3), None);
}

#[test]
fn iterates_and_counts_winning_records_only() {
    let order = three_plugins();
    let mut ids: Vec<u32> = order
        .records_of_type(sig::WEAP)
        .map(|r| r.form_id.0)
        .collect();
    ids.sort();
    assert_eq!(ids, [0x0000_0800, 0x0100_0900, 0x0200_0A00]);
    assert_eq!(order.len(), 4);
    assert_eq!(order.count_of_type(sig::NPC_), 1);
    assert_eq!(order.type_counts(), vec![(sig::WEAP, 3), (sig::NPC_, 1)]);
    assert_eq!(order.plugin_stats(0), (2, 0));
    assert_eq!(order.plugin_stats(1), (1, 1));
    assert_eq!(order.plugin_stats(2), (1, 1));
}

#[test]
fn finds_the_winning_record_by_editor_id() {
    let order = three_plugins();
    let found = order
        .find_by_editor_id("dlcgun", Some(sig::WEAP))
        .unwrap()
        .unwrap();
    assert_eq!(found.plugin.name, "ModB.esp");
    assert_eq!(found.form_id, FormId(0x0100_0900));
    assert!(order
        .find_by_editor_id("BaseNpc", Some(sig::WEAP))
        .unwrap()
        .is_none());
    assert!(order.find_by_editor_id("BaseNpc", None).unwrap().is_some());
}

#[test]
fn reports_missing_or_misordered_masters() {
    let alone = LoadOrder::from_plugins(vec![("ModB.esp".into(), None, p(mod_b()))]);
    match alone {
        Err(Error::MissingMaster { plugin, master }) => {
            assert_eq!((plugin.as_str(), master.as_str()), ("ModB.esp", "Dlc.esm"))
        }
        other => panic!("unexpected: {:?}", other.err()),
    }
    // A master that loads later is still found by name, as the game finds
    // it; its own version of a shared record then wins.
    let reversed = LoadOrder::from_plugins(vec![
        ("Dlc.esm".into(), None, p(dlc(&["Base.esm"]))),
        ("Base.esm".into(), None, p(base())),
    ])
    .unwrap();
    assert!(reversed.warnings()[0].contains("loads before its master Base.esm"));
    assert_eq!(name_of(&reversed, 0x0100_0800).as_deref(), Some("Base Gun"));
    assert_eq!(name_of(&reversed, 0x0000_0900).as_deref(), Some("Dlc Gun"));
}

#[test]
fn master_names_match_case_insensitively() {
    let order = LoadOrder::from_plugins(vec![
        ("base.ESM".into(), None, p(base())),
        ("Dlc.esm".into(), None, p(dlc(&["Base.esm"]))),
    ]);
    assert!(order.is_ok());
}

#[test]
fn rejects_more_than_255_plugins() {
    let list = (0..256)
        .map(|i| (format!("P{i}.esp"), None, p(tes4(0, &[], "t"))))
        .collect();
    assert!(matches!(
        LoadOrder::from_plugins(list),
        Err(Error::TooManyPlugins { count: 256 })
    ));
}

#[test]
fn single_plugin_keeps_local_numbering() {
    let order = LoadOrder::single("Dlc.esm", None, p(dlc(&["Base.esm"]))).unwrap();
    assert!(order.is_single());
    assert_eq!(order.slot_name(0), Some("Base.esm"));
    assert_eq!(order.slot_name(1), Some("Dlc.esm"));
    assert!(order.get(FormId(0x0000_0800)).is_some());
    assert!(order.get(FormId(0x0100_0900)).is_some());
}

/// A REFR placing `base` inside the cell children of `cell`.
fn placed(form_id: u32, base: u32) -> Vec<u8> {
    record(b"REFR", form_id, 0, &sub(b"NAME", &base.to_le_bytes()))
}

/// An interior cell with its temporary children, wrapped in the CELL top
/// group and block/sub-block groups.
fn cell_group(cell: u32, children: &[u8]) -> Vec<u8> {
    let temporary = group(cell.to_le_bytes(), 9, children);
    let mut contents = record(b"CELL", cell, 0, &sub(b"EDID", &zstr("Room")));
    contents.extend(group(cell.to_le_bytes(), 6, &temporary));
    let sub_block = group(0i32.to_le_bytes(), 3, &contents);
    group(*b"CELL", 0, &group(0i32.to_le_bytes(), 2, &sub_block))
}

#[test]
fn lists_the_winning_references_in_a_cell() {
    // Base.esm: a room holding two objects, and a second room holding one.
    let mut base_file = tes4(flags::MASTER, &[], "t");
    let mut refs = placed(0x0000_0A01, 0x0000_0800);
    refs.extend(placed(0x0000_0A02, 0x0000_0800));
    let mut both = cell_group(0x0000_0A00, &refs);
    both.extend(cell_group(0x0000_0B00, &placed(0x0000_0B01, 0x0000_0800)));
    base_file.extend(both);

    // Mod.esp: changes one object in the first room, adds a new one, and
    // moves the object in the second room into the first.
    let mut mod_refs = placed(0x0000_0A01, 0x0000_0801);
    mod_refs.extend(placed(0x0100_0C00, 0x0000_0800));
    mod_refs.extend(placed(0x0000_0B01, 0x0000_0800));
    let mut mod_file = tes4(0, &["Base.esm"], "t");
    mod_file.extend(cell_group(0x0000_0A00, &mod_refs));

    let order = LoadOrder::from_plugins(vec![
        ("Base.esm".into(), None, p(base_file)),
        ("Mod.esp".into(), None, p(mod_file)),
    ])
    .unwrap();

    let room = order.references_in_cell(FormId(0x0000_0A00));
    let mut ids: Vec<u32> = room.iter().map(|r| r.form_id.0).collect();
    ids.sort();
    assert_eq!(ids, [0x0000_0A01, 0x0000_0A02, 0x0000_0B01, 0x0100_0C00]);
    let changed = room.iter().find(|r| r.form_id.0 == 0x0000_0A01).unwrap();
    assert_eq!(changed.plugin.name, "Mod.esp");
    assert_eq!(
        changed.record().unwrap().get(sig::NAME).unwrap().data,
        0x0000_0801u32.to_le_bytes()
    );
    assert_eq!(order.cell_of(changed), Some(FormId(0x0000_0A00)));

    assert!(order.references_in_cell(FormId(0x0000_0B00)).is_empty());
    let counts = order.reference_counts();
    assert_eq!(counts.get(&FormId(0x0000_0A00)), Some(&4));
    assert_eq!(counts.get(&FormId(0x0000_0B00)), None);
}

#[test]
fn parses_plugins_txt() {
    // As the game reads it (00872430): comments and empty lines skipped,
    // the line feed (and a carriage return before it) cut, nothing
    // trimmed, and a last line of one character without a line feed
    // skipped.
    let text = b"# Active plugins\r\nFalloutNV.esm\r\n\r\nMod One.esp\n  Caf\xe9.esp \n#Off.esp\nX";
    assert_eq!(
        parse_plugins_txt(text),
        ["FalloutNV.esm", "Mod One.esp", "  Café.esp "]
    );
}

// ---------------------------------------------------------------------------
// Data folder loading
// ---------------------------------------------------------------------------

struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let path = std::env::temp_dir().join(format!("nv-rs-test-{tag}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&path);
        fs::create_dir_all(&path).unwrap();
        TempDir(path)
    }

    /// Writes a file with a given modification time (seconds after an epoch).
    fn write(&self, name: &str, bytes: &[u8], age: u64) {
        let path = self.0.join(name);
        fs::write(&path, bytes).unwrap();
        let time = SystemTime::UNIX_EPOCH + Duration::from_secs(1_000_000_000 + age);
        fs::File::options()
            .write(true)
            .open(&path)
            .unwrap()
            .set_modified(time)
            .unwrap();
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn plain_plugin(master_flag: u32) -> Vec<u8> {
    tes4(master_flag, &["FalloutNV.esm"], "t")
}

fn data_folder(tag: &str) -> TempDir {
    let dir = TempDir::new(tag);
    // Lower-case on disk; other files list it as "FalloutNV.esm".
    dir.write("falloutnv.esm", &base(), 50);
    dir.write("DeadMoney.esm", &plain_plugin(flags::MASTER), 10);
    dir.write("ZMod.esp", &plain_plugin(0), 20);
    dir.write("AMod.esp", &plain_plugin(0), 30);
    dir.write("MasterFlagged.esp", &plain_plugin(flags::MASTER), 40);
    dir.write("Inactive.esp", &plain_plugin(0), 5);
    dir.write("readme.txt", b"not a plugin", 1);
    dir
}

fn names(order: &LoadOrder) -> Vec<&str> {
    order.plugins().iter().map(|p| p.name.as_str()).collect()
}

#[test]
fn orders_active_plugins_like_the_game() {
    let dir = data_folder("order");
    let active = ActivePlugins::List(
        [
            "ZMod.esp",
            "AMod.esp",
            "MasterFlagged.esp",
            "DeadMoney.esm",
            "Missing.esp",
        ]
        .map(String::from)
        .to_vec(),
    );
    let order = LoadOrder::from_data_dir(dir.path(), &active).unwrap();
    // Master-flagged files by age (FalloutNV.esm, though newer, moved
    // before DeadMoney.esm, which needs it), then the rest by age.
    assert_eq!(
        names(&order),
        [
            "falloutnv.esm",
            "DeadMoney.esm",
            "MasterFlagged.esp",
            "ZMod.esp",
            "AMod.esp"
        ]
    );
    let listed: Vec<&String> = order
        .warnings()
        .iter()
        .filter(|w| w.contains("listed as active"))
        .collect();
    assert_eq!(listed.len(), 1);
    assert!(listed[0].contains("Missing.esp"));
}

#[test]
fn official_only_loads_just_the_official_files_present() {
    let dir = data_folder("official");
    let order = LoadOrder::from_data_dir(dir.path(), &ActivePlugins::OfficialOnly).unwrap();
    assert_eq!(names(&order), ["falloutnv.esm", "DeadMoney.esm"]);
    assert!(order.warnings().is_empty());
}

#[test]
fn requires_the_main_master() {
    let dir = TempDir::new("nomain");
    dir.write("AMod.esp", &plain_plugin(0), 1);
    let err = LoadOrder::from_data_dir(dir.path(), &ActivePlugins::OfficialOnly)
        .err()
        .unwrap();
    assert!(err.to_string().contains("FalloutNV.esm"), "{err}");
}

#[test]
fn names_the_plugin_that_failed_to_load() {
    let dir = data_folder("broken");
    dir.write("Broken.esp", b"TES4 but truncated", 60);
    let active = ActivePlugins::List(vec!["Broken.esp".into()]);
    let err = LoadOrder::from_data_dir(dir.path(), &active).err().unwrap();
    assert!(
        matches!(&err, Error::InPlugin { name, .. } if name == "Broken.esp"),
        "{err}"
    );
}

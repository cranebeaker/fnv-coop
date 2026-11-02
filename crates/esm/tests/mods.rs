//! The mod setups of `testdata::mods` that concern plugins, checked
//! against what the game does with them (docs/MODS.md).

use esm::load_order::{game_load_order, parse_plugins_txt};
use esm::{sig, ActivePlugins, FormId, LoadOrder};
use testdata::mods::{case, Case};
use testdata::TempData;

fn written(name: &str) -> TempData {
    let dir = TempData::empty(&format!("esm-mods-{name}"));
    case(name).write(dir.path()).unwrap();
    dir
}

fn active(dir: &TempData) -> ActivePlugins {
    let text = std::fs::read(dir.path().join("plugins.txt")).unwrap();
    ActivePlugins::List(parse_plugins_txt(&text))
}

fn names(order: &LoadOrder) -> Vec<&str> {
    order.plugins().iter().map(|p| p.name.as_str()).collect()
}

fn name_of(order: &LoadOrder, id: u32) -> Option<String> {
    order.get(FormId(id))?.record().unwrap().full_name()
}

#[test]
fn loads_the_plugins_the_game_loads_in_its_order() {
    let dir = written("load-order");
    let data = Case::data(dir.path());
    let order = LoadOrder::from_data_dir(&data, &active(&dir)).unwrap();
    assert_eq!(
        names(&order),
        [
            "FalloutNV.esm",
            "DeadMoney.esm",
            "MasterFlagged.esp",
            "NeededMaster.esp",
            "Same2.esp",
            "Same1.esp",
            "ZMod.esp",
            "AMod.esp",
        ]
    );
    let warnings = order.warnings().join("\n");
    assert!(warnings.contains("Missing.esp is listed"), "{warnings}");
    // Empty.esp is listed but empty: the game doesn't list empty files.
    assert!(warnings.contains("Empty.esp is listed"), "{warnings}");
    assert!(
        warnings.contains("AMod.esp is dependent on has changed"),
        "{warnings}"
    );
    // ZMod.esp was saved against FalloutNV.esm's real size.
    assert!(!warnings.contains("ZMod.esp is dependent"), "{warnings}");
    assert_eq!(order.warnings().len(), 3, "{warnings}");
}

#[test]
fn official_only_still_takes_the_masters_order() {
    let dir = written("load-order");
    let data = Case::data(dir.path());
    let (names, warnings) = game_load_order(&data, &ActivePlugins::OfficialOnly).unwrap();
    assert_eq!(names, ["FalloutNV.esm", "DeadMoney.esm"]);
    assert!(warnings.is_empty());
}

#[test]
fn a_missing_master_stops_the_loading() {
    let dir = written("load-order");
    let data = Case::data(dir.path());
    std::fs::remove_file(data.join("NeededMaster.esp")).unwrap();
    let err = LoadOrder::from_data_dir(&data, &active(&dir))
        .err()
        .unwrap();
    assert!(
        matches!(&err, esm::Error::MissingMaster { plugin, master }
            if plugin == "ZMod.esp" && master == "NeededMaster.esp"),
        "{err}"
    );
}

#[test]
fn renumbers_form_ids_and_lets_the_last_plugin_win() {
    let dir = written("overrides");
    let data = Case::data(dir.path());
    let order = LoadOrder::from_data_dir(&data, &active(&dir)).unwrap();
    assert_eq!(
        names(&order),
        ["FalloutNV.esm", "Dlc.esm", "ModB.esp", "ModC.esp"]
    );
    assert_eq!(
        name_of(&order, 0x0000_0800).as_deref(),
        Some("Base Gun (Dlc)")
    );
    assert_eq!(
        name_of(&order, 0x0100_0900).as_deref(),
        Some("Dlc Gun (ModB)")
    );
    assert_eq!(name_of(&order, 0x0200_0A00).as_deref(), Some("ModB Gun"));
    assert_eq!(
        name_of(&order, 0x0000_0801).as_deref(),
        Some("Base Npc (ModC)")
    );
    // Written 07000B00: past ModC's two masters, so ModC itself.
    assert_eq!(name_of(&order, 0x0300_0B00).as_deref(), Some("ModC Npc"));
    assert_eq!(
        name_of(&order, 0x0000_0A00).as_deref(),
        Some("Test Cell (ModC)")
    );

    // The cell's placed objects: the moved original and ModC's new one,
    // whose base is ModB's gun.
    let placed = order.references_in_cell(FormId(0x0000_0A00));
    let ids: Vec<FormId> = placed.iter().map(|r| r.form_id).collect();
    assert_eq!(ids, [FormId(0x0000_0A01), FormId(0x0300_0C00)]);
    let moved = placed[0].record().unwrap();
    let data = moved.get(sig::DATA).unwrap();
    assert_eq!(
        f32::from_le_bytes(data.data[..4].try_into().unwrap()),
        100.0
    );
    let base = |r: &esm::RecordRef| {
        let name = r.record().unwrap();
        let b = name.get(esm::FourCC::new(b"NAME")).unwrap().data.clone();
        r.plugin
            .to_global(FormId(u32::from_le_bytes(b[..4].try_into().unwrap())))
    };
    assert_eq!(base(&placed[0]), FormId(0x0000_0800));
    assert_eq!(base(&placed[1]), FormId(0x0200_0A00));
    assert_eq!(order.versions(FormId(0x0000_0A01)).len(), 2);
}

#[test]
fn engine_forms_keep_their_ids() {
    use testdata::mods::{main_master, named, PluginFile};
    // Mod.esp's only master is Dlc.esm (00), so its 00 means Dlc.esm, but
    // IDs 1 to 0x7FF are the engine's own forms (00484b40): a reference to
    // 00000014 (the player) stays 00000014, and a record changing the
    // engine form 014, written 01000014, is the base game's 00000014.
    let dlc = PluginFile::new(true).master("FalloutNV.esm", 0).build();
    let modded = PluginFile::new(false)
        .master("Dlc.esm", 0)
        .top(
            b"NPC_",
            &[named(b"NPC_", 0x0100_0014, "Player", "Modded Player")],
        )
        .build();
    let p = |b: Vec<u8>| esm::Plugin::from_bytes(b).unwrap();
    let order = LoadOrder::from_plugins(vec![
        ("FalloutNV.esm".into(), None, p(main_master())),
        ("Dlc.esm".into(), None, p(dlc)),
        ("Mod.esp".into(), None, p(modded)),
    ])
    .unwrap();
    let modded = &order.plugins()[2];
    assert_eq!(modded.to_global(FormId(0x0000_0014)), FormId(0x14));
    assert_eq!(modded.to_global(FormId(0x0000_0800)), FormId(0x0100_0800));
    assert_eq!(modded.record_id(FormId(0x0100_0014)), FormId(0x14));
    assert_eq!(name_of(&order, 0x14).as_deref(), Some("Modded Player"));
}

//! End-to-end tests against small plugins assembled byte by byte, so no
//! game files are needed to run them.

use esm::{flags, sig, Error, FormId, FormOrigin, FourCC, Plugin, Weapon, WeaponStats};

mod common;
use common::*;

/// A plugin exercising nested groups, compression, XXXX and odd text.
fn sample_plugin() -> Vec<u8> {
    let mut weapons = Vec::new();
    let mut pistol = sub(b"EDID", &zstr("TestPistol"));
    pistol.extend(sub(b"FULL", &zstr("Test Pistol")));
    pistol.extend(sub(b"DATA", &weapon_data(250, 150, 3.5, 22, 8)));
    weapons.extend(record(b"WEAP", 0x0100_0801, 0, &pistol));

    let mut rifle = sub(b"EDID", &zstr("TestRifle"));
    rifle.extend(sub(b"FULL", b"Hunter\x92s Rifle\0"));
    rifle.extend(sub(b"DATA", &weapon_data(1200, 400, 9.0, 48, 5)));
    weapons.extend(compressed_record(b"WEAP", 0x0100_0802, 0, &rifle));

    let mut npc = sub(b"EDID", &zstr("TestTrader"));
    npc.extend(sub(b"FULL", &zstr("Test Trader")));
    npc.extend(big_sub(b"DNAM", &vec![0xAB; 70_000]));
    npc.extend(sub(b"SNAM", &[1, 2, 3, 4]));
    let npcs = compressed_record(b"NPC_", 0x0000_1234, 0, &npc);

    // CELL top group → interior block → sub-block → CELL + its children.
    let mut cell = sub(b"EDID", &zstr("TestInterior"));
    cell.extend(sub(b"FULL", &zstr("Test Room")));
    let cell_record = record(b"CELL", 0x0100_0900, 0, &cell);
    let refr = record(
        b"REFR",
        0x0100_0901,
        flags::PERSISTENT,
        &sub(b"NAME", &0x0100_0801u32.to_le_bytes()),
    );
    let deleted = record(b"REFR", 0x0100_0902, flags::DELETED, &[]);
    let mut refs = refr;
    refs.extend(deleted);
    let temporary = group(0x0100_0900u32.to_le_bytes(), 9, &refs);
    let children = group(0x0100_0900u32.to_le_bytes(), 6, &temporary);
    let mut sub_block_contents = cell_record;
    sub_block_contents.extend(children);
    let sub_block = group(3i32.to_le_bytes(), 3, &sub_block_contents);
    let block = group(0i32.to_le_bytes(), 2, &sub_block);

    let mut file = tes4(0, &["FalloutNV.esm"], "Tester");
    file.extend(group(*b"WEAP", 0, &weapons));
    file.extend(group(*b"NPC_", 0, &npcs));
    file.extend(group(*b"CELL", 0, &block));
    file
}

fn weap() -> FourCC {
    FourCC::new(b"WEAP")
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[test]
fn reads_header_and_masters() {
    let plugin = Plugin::from_bytes(sample_plugin()).unwrap();
    let h = plugin.header();
    assert!((h.version - 1.34).abs() < 1e-6);
    assert_eq!(h.declared_record_count, 7);
    assert_eq!(h.author.as_deref(), Some("Tester"));
    assert_eq!(h.description.as_deref(), Some("Test plugin"));
    assert_eq!(h.masters, vec!["FalloutNV.esm".to_string()]);
    assert!(!h.is_master);

    let master = Plugin::from_bytes(tes4(flags::MASTER, &[], "Tester")).unwrap();
    assert!(master.header().is_master);
    assert!(master.records().is_empty());
}

#[test]
fn indexes_records_by_type_and_form_id() {
    let plugin = Plugin::from_bytes(sample_plugin()).unwrap();
    assert_eq!(plugin.records().len(), 6);
    assert_eq!(plugin.group_count(), 7);
    assert_eq!(plugin.count_of_type(weap()), 2);
    assert_eq!(plugin.count_of_type(sig::REFR), 2);
    assert_eq!(plugin.count_of_type(FourCC::new(b"ARMO")), 0);
    assert_eq!(plugin.type_counts()[0], (sig::REFR, 2));

    let entry = plugin.get(FormId(0x0000_1234)).unwrap();
    assert_eq!(entry.header.kind, sig::NPC_);
    assert!(plugin.get(FormId(0xDEAD_BEEF)).is_none());
}

#[test]
fn walks_nested_cell_groups() {
    let plugin = Plugin::from_bytes(sample_plugin()).unwrap();
    let cell = plugin.records_of_type(sig::CELL).next().unwrap();
    assert_eq!(
        plugin.record(cell).unwrap().full_name().as_deref(),
        Some("Test Room")
    );

    let refs: Vec<_> = plugin.records_of_type(sig::REFR).collect();
    assert!(refs.iter().all(|r| r.top_group == sig::CELL));
    assert!(refs.iter().all(|r| r.cell == Some(FormId(0x0100_0900))));
    assert!(refs.iter().all(|r| r.world.is_none()));
    assert_eq!(cell.cell, None);
    assert!(refs[0].header.flags & flags::PERSISTENT != 0);
    assert!(!refs[0].header.is_deleted());
    assert!(refs[1].header.is_deleted());
    assert!(plugin.subrecords(refs[1]).unwrap().is_empty());
}

#[test]
fn tracks_the_topic_of_dialogue_lines() {
    // DIAL top group → topic → its children (type 7) → INFO lines.
    let mut topics = record(b"DIAL", 0x0100_0B00, 0, &sub(b"EDID", &zstr("GREETING")));
    let lines = record(b"INFO", 0x0100_0B01, 0, &sub(b"NAM1", &zstr("Hello.")));
    topics.extend(group(0x0100_0B00u32.to_le_bytes(), 7, &lines));
    topics.extend(record(
        b"DIAL",
        0x0100_0B02,
        0,
        &sub(b"EDID", &zstr("Other")),
    ));
    let mut file = tes4(0, &["FalloutNV.esm"], "Tester");
    file.extend(group(*b"DIAL", 0, &topics));
    let plugin = Plugin::from_bytes(file).unwrap();

    let info = plugin.records_of_type(FourCC::new(b"INFO")).next().unwrap();
    assert_eq!(info.topic, Some(FormId(0x0100_0B00)));
    // The topic after the group isn't inside it.
    let other = plugin.get(FormId(0x0100_0B02)).unwrap();
    assert_eq!(other.topic, None);
}

#[test]
fn tracks_the_worldspace_and_cell_of_exterior_references() {
    // WRLD top group → world → world children → exterior block → sub-block
    // → CELL + its children.
    let world = record(b"WRLD", 0x0100_0A00, 0, &sub(b"EDID", &zstr("TestWorld")));
    let cell = record(b"CELL", 0x0100_0A01, 0, &sub(b"EDID", &zstr("Outside")));
    let refr = record(b"REFR", 0x0100_0A02, 0, &sub(b"NAME", &[0; 4]));
    let temporary = group(0x0100_0A01u32.to_le_bytes(), 9, &refr);
    let children = group(0x0100_0A01u32.to_le_bytes(), 6, &temporary);
    let mut sub_block = cell;
    sub_block.extend(children);
    let sub_block = group([0, 0, 0, 0], 5, &sub_block);
    let block = group([0, 0, 0, 0], 4, &sub_block);
    let world_children = group(0x0100_0A00u32.to_le_bytes(), 1, &block);
    let mut worlds = world;
    worlds.extend(world_children);

    let mut file = tes4(0, &["FalloutNV.esm"], "Tester");
    file.extend(group(*b"WRLD", 0, &worlds));
    let plugin = Plugin::from_bytes(file).unwrap();

    let world = plugin.records_of_type(sig::WRLD).next().unwrap();
    assert_eq!((world.cell, world.world), (None, None));
    let cell = plugin.records_of_type(sig::CELL).next().unwrap();
    assert_eq!(cell.top_group, sig::WRLD);
    assert_eq!((cell.cell, cell.world), (None, Some(FormId(0x0100_0A00))));
    let refr = plugin.records_of_type(sig::REFR).next().unwrap();
    assert_eq!(refr.cell, Some(FormId(0x0100_0A01)));
    assert_eq!(refr.world, Some(FormId(0x0100_0A00)));
}

#[test]
fn decodes_names_and_weapon_stats() {
    let plugin = Plugin::from_bytes(sample_plugin()).unwrap();
    let weapons: Vec<Weapon> = plugin
        .records_of_type(weap())
        .map(|e| Weapon::from_record(&plugin.record(e).unwrap()).unwrap())
        .collect();

    assert_eq!(weapons[0].editor_id.as_deref(), Some("TestPistol"));
    assert_eq!(weapons[0].name.as_deref(), Some("Test Pistol"));
    assert_eq!(
        weapons[0].stats,
        Some(WeaponStats {
            value: 250,
            health: 150,
            weight: 3.5,
            base_damage: 22,
            clip_size: 8
        })
    );
    // Compressed record, with a Windows-1252 apostrophe in the name.
    assert_eq!(weapons[1].name.as_deref(), Some("Hunter’s Rifle"));
    assert_eq!(weapons[1].stats.unwrap().base_damage, 48);
}

#[test]
fn decompresses_records_and_reads_large_subrecords() {
    let plugin = Plugin::from_bytes(sample_plugin()).unwrap();
    let entry = plugin.get(FormId(0x0000_1234)).unwrap();
    assert!(entry.header.is_compressed());

    let record = plugin.record(entry).unwrap();
    assert_eq!(record.editor_id().as_deref(), Some("TestTrader"));
    let dnam = record.get(FourCC::new(b"DNAM")).unwrap();
    assert_eq!(dnam.data.len(), 70_000);
    assert!(dnam.data.iter().all(|&b| b == 0xAB));
    // The subrecord after the XXXX one still parses.
    assert_eq!(record.get(FourCC::new(b"SNAM")).unwrap().data, [1, 2, 3, 4]);
}

#[test]
fn reads_one_subrecord_as_the_whole_record_does() {
    let plugin = Plugin::from_bytes(sample_plugin()).unwrap();
    for entry in plugin.records() {
        let record = plugin.record(entry).unwrap();
        let mut kinds: Vec<FourCC> = record.subrecords.iter().map(|s| s.kind).collect();
        kinds.push(FourCC::new(b"ZZZZ"));
        for kind in kinds {
            let one = plugin.subrecord_of(entry, kind).unwrap();
            assert_eq!(
                one.as_deref(),
                record.get(kind).map(|s| s.data.as_slice()),
                "{:?} {kind:?}",
                entry.header.form_id
            );
        }
    }
    // The compressed record's large subrecord, and the one after it.
    let trader = plugin.get(FormId(0x0000_1234)).unwrap();
    let dnam = plugin
        .subrecord_of(trader, FourCC::new(b"DNAM"))
        .unwrap()
        .unwrap();
    assert_eq!(dnam.len(), 70_000);
    let snam = plugin
        .subrecord_of(trader, FourCC::new(b"SNAM"))
        .unwrap()
        .unwrap();
    assert_eq!(&*snam, &[1, 2, 3, 4]);
}

#[test]
fn finds_records_by_editor_id_case_insensitively() {
    let plugin = Plugin::from_bytes(sample_plugin()).unwrap();
    let found = plugin
        .find_by_editor_id("testrifle", None)
        .unwrap()
        .unwrap();
    assert_eq!(found.header.form_id, FormId(0x0100_0802));
    let npc = plugin
        .find_by_editor_id("TESTTRADER", Some(sig::NPC_))
        .unwrap();
    assert!(npc.is_some());
    assert!(plugin
        .find_by_editor_id("TestTrader", Some(weap()))
        .unwrap()
        .is_none());
    assert!(plugin.find_by_editor_id("Nope", None).unwrap().is_none());
}

#[test]
fn resolves_form_id_origins() {
    let plugin = Plugin::from_bytes(sample_plugin()).unwrap();
    assert_eq!(
        plugin.form_origin(FormId(0x0000_1234)),
        FormOrigin::Master("FalloutNV.esm")
    );
    assert_eq!(
        plugin.form_origin(FormId(0x0100_0801)),
        FormOrigin::ThisPlugin
    );
    assert_eq!(
        plugin.form_origin(FormId(0x0500_0001)),
        FormOrigin::UnknownMaster(5)
    );
}

#[test]
fn rejects_files_that_are_not_plugins() {
    match Plugin::from_bytes(b"BSA\0 some archive".to_vec()) {
        Err(Error::NotAPlugin { found }) => assert_eq!(found, FourCC::new(b"BSA\0")),
        other => panic!("unexpected result: {:?}", other.err()),
    }
    assert!(matches!(
        Plugin::from_bytes(Vec::new()),
        Err(Error::NotAPlugin { .. })
    ));
}

#[test]
fn reports_truncated_files() {
    let full = sample_plugin();
    for cut in [10, 30, full.len() / 2, full.len() - 1] {
        let err = Plugin::from_bytes(full[..cut].to_vec()).err();
        assert!(err.is_some(), "truncating to {cut} bytes should fail");
    }
}

#[test]
fn reports_records_that_overrun_their_group() {
    let mut contents = record(b"WEAP", 1, 0, &sub(b"EDID", &zstr("X")));
    contents[4] = 0xFF; // inflate the data size far past the group's end
    let mut file = tes4(0, &[], "t");
    file.extend(group(*b"WEAP", 0, &contents));
    let err = Plugin::from_bytes(file).err().unwrap();
    assert!(matches!(err, Error::BadRecord { .. }), "{err}");
}

#[test]
fn reports_corrupt_compressed_data_with_the_form_id() {
    let mut rec = compressed_record(b"WEAP", 0x0100_0803, 0, &sub(b"EDID", &zstr("Broken")));
    rec[24 + 4] = 0x79; // the zlib header: an unknown compression method
    let mut file = tes4(0, &[], "t");
    file.extend(group(*b"WEAP", 0, &rec));

    let plugin = Plugin::from_bytes(file).unwrap(); // indexing doesn't decompress
    let entry = plugin.records_of_type(weap()).next().unwrap();
    let err = plugin.record(entry).unwrap_err();
    assert!(err.to_string().contains("01000803"), "{err}");
}

#[test]
fn accepts_a_wrong_checksum_when_the_size_is_right() {
    // The game doesn't check it, and FalloutNV.esm has a record (LAND
    // 00150FC0) whose stored checksum is wrong.
    let mut rec = compressed_record(b"WEAP", 0x0100_0803, 0, &sub(b"EDID", &zstr("Fine")));
    let last = rec.len() - 1;
    rec[last] ^= 0xFF;
    let mut file = tes4(0, &[], "t");
    file.extend(group(*b"WEAP", 0, &rec));

    let plugin = Plugin::from_bytes(file).unwrap();
    let entry = plugin.records_of_type(weap()).next().unwrap();
    let record = plugin.record(entry).unwrap();
    assert_eq!(record.editor_id().as_deref(), Some("Fine"));
}

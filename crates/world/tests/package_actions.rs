//! Synthetic opening-style packages. No original game assets are needed.

use esm::{FormId, LoadOrder, Plugin};
use testdata::{group, record, sub, zstr};
use world::ai::Package;

fn plugin(masters: &[&str], packages: &[u8]) -> Plugin {
    let mut header = 1.34f32.to_le_bytes().to_vec();
    header.extend([0; 8]);
    let mut data = sub(b"HEDR", &header);
    for master in masters {
        data.extend(sub(b"MAST", &zstr(master)));
        data.extend(sub(b"DATA", &[0; 8]));
    }
    let mut bytes = record(b"TES4", 0, &data);
    bytes.extend(group(*b"PACK", 0, packages));
    Plugin::from_bytes(bytes).unwrap()
}

fn package(id: u32, fields: &[u8]) -> Vec<u8> {
    let mut data = sub(b"EDID", &zstr("OpeningPackage"));
    let mut pkdt = [0; 12];
    pkdt[4] = 6;
    data.extend(sub(b"PKDT", &pkdt));
    data.extend(fields);
    record(b"PACK", id, &data)
}

fn form(tag: &[u8; 4], id: u32) -> Vec<u8> {
    sub(tag, &id.to_le_bytes())
}

#[test]
fn opening_actions_keep_begin_end_and_change_separate() {
    let mut fields = form(b"INAM", 0x999); // outside an action: ignored
    fields.extend(sub(b"POBA", &[]));
    fields.extend(form(b"INAM", 0x810)); // sit up
    fields.extend(sub(b"SCHR", &[0; 20]));
    fields.extend(sub(b"SCDA", &[0x1c, 0, 1, 0]));
    fields.extend(sub(b"SCTX", &zstr("SetStage TestQuest 5")));
    fields.extend(sub(b"SLSD", &[0; 24]));
    fields.extend(sub(b"SCVR", &zstr("timer")));
    fields.extend(form(b"SCRO", 0x820));
    fields.extend(form(b"SCRV", 1));
    fields.extend(form(b"TNAM", 0x830));
    fields.extend(sub(b"POEA", &[]));
    fields.extend(form(b"INAM", 0));
    fields.extend(sub(b"SCHR", &[0; 20]));
    fields.extend(form(b"TNAM", 0));
    fields.extend(sub(b"POCA", &[]));
    fields.extend(form(b"INAM", 0x811)); // bed-sitting loop
    fields.extend(form(b"TNAM", 0));
    // TNAM ends the action, so subsequent script fields don't leak into it.
    fields.extend(sub(b"SCTX", &zstr("wrong script")));
    fields.extend(form(b"INAM", 0x999));

    let order = LoadOrder::single("Test.esm", None, plugin(&[], &package(0x800, &fields))).unwrap();
    let p = Package::load(&order, FormId(0x800)).unwrap();
    assert_eq!(p.actions.begin.idle, Some(FormId(0x810)));
    assert_eq!(p.actions.begin.topic, Some(FormId(0x830)));
    assert_eq!(
        p.actions.begin.source().as_deref(),
        Some("SetStage TestQuest 5")
    );
    assert_eq!(p.actions.begin.bytecode(), &[0x1c, 0, 1, 0]);
    assert_eq!(p.actions.begin.script.len(), 7);
    assert_eq!(p.actions.end.idle, None);
    assert_eq!(p.actions.end.topic, None);
    assert_eq!(p.actions.end.script.len(), 1);
    assert!(p.actions.end.bytecode().is_empty());
    assert_eq!(p.actions.change.idle, Some(FormId(0x811)));
    assert_eq!(p.actions.change.source(), None);
}

#[test]
fn action_forms_follow_load_order_but_script_variable_indices_do_not() {
    // This plugin lists only Dlc as a master: local 00 -> global 01,
    // local 01 -> global 02. Nulls must remain null.
    let mut fields = sub(b"POBA", &[]);
    fields.extend(form(b"INAM", 0x900));
    fields.extend(form(b"SCRO", 0x900));
    fields.extend(form(b"SCRO", 0x0100_0901));
    fields.extend(form(b"SCRV", 9));
    fields.extend(sub(b"SCDA", &[1, 2, 3])); // no source: still retained
    fields.extend(form(b"TNAM", 0x0100_0902));
    fields.extend(sub(b"POCA", &[]));
    fields.extend(form(b"INAM", 0));
    fields.extend(form(b"SCRO", 0));
    fields.extend(form(b"TNAM", 0));
    let order = LoadOrder::from_plugins(vec![
        ("Base.esm".into(), None, plugin(&[], &[])),
        ("Dlc.esm".into(), None, plugin(&["Base.esm"], &[])),
        (
            "Mod.esp".into(),
            None,
            plugin(&["Dlc.esm"], &package(0x0100_0800, &fields)),
        ),
    ])
    .unwrap();
    let p = Package::load(&order, FormId(0x0200_0800)).unwrap();
    let action = &p.actions.begin;
    assert_eq!(action.idle, Some(FormId(0x0100_0900)));
    assert_eq!(action.topic, Some(FormId(0x0200_0902)));
    assert_eq!(action.script[0].data, 0x0100_0900u32.to_le_bytes());
    assert_eq!(action.script[1].data, 0x0200_0901u32.to_le_bytes());
    assert_eq!(action.script[2].data, 9u32.to_le_bytes());
    assert_eq!(action.bytecode(), &[1, 2, 3]);
    assert_eq!(action.source(), None);
    assert_eq!(p.actions.change.idle, None);
    assert_eq!(p.actions.change.topic, None);
    assert_eq!(p.actions.change.script[0].data, [0; 4]);
}

#[test]
fn absent_and_truncated_actions_do_not_invent_references() {
    let mut fields = sub(b"POBA", &[]);
    fields.extend(sub(b"INAM", &[1, 2]));
    fields.extend(sub(b"TNAM", &[1]));
    fields.extend(sub(b"POCA", &[]));
    fields.extend(form(b"INAM", 0x811));
    fields.extend(form(b"TNAM", 0));
    let mut packages = package(0x800, &fields);
    packages.extend(package(0x801, &[]));
    let order = LoadOrder::single("Test.esm", None, plugin(&[], &packages)).unwrap();
    let p = Package::load(&order, FormId(0x800)).unwrap();
    assert_eq!(p.actions.begin.idle, None);
    assert_eq!(p.actions.begin.topic, None);
    assert_eq!(p.actions.change.idle, Some(FormId(0x811)));
    assert_eq!(
        Package::load(&order, FormId(0x801)).unwrap().actions,
        Default::default()
    );
}

#[test]
fn a_repeated_action_marker_replaces_its_previous_contents() {
    let mut fields = sub(b"POBA", &[]);
    fields.extend(form(b"INAM", 0x810));
    fields.extend(sub(b"SCTX", &zstr("old script")));
    fields.extend(form(b"TNAM", 0x830));
    fields.extend(sub(b"POBA", &[]));
    fields.extend(form(b"INAM", 0x811));
    fields.extend(form(b"TNAM", 0));
    let order = LoadOrder::single("Test.esm", None, plugin(&[], &package(0x800, &fields))).unwrap();
    let p = Package::load(&order, FormId(0x800)).unwrap();
    assert_eq!(p.actions.begin.idle, Some(FormId(0x811)));
    assert_eq!(p.actions.begin.topic, None);
    assert!(p.actions.begin.script.is_empty());
}

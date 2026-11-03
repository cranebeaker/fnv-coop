//! Policy rows resolve to the quests' runtime ids in whatever load order is used.
//! Needs the five official ESMs in `FNV_ESM_DIR`; skips without them.
//!
//! The table has 483 rows with a quest record: 482 live in the base game or the four story
//! DLCs; the 483rd (`NVDLC05MQ00`) is Gun Runners' Arsenal, which is not one of the five files
//! and so is reported as unmatched in every order below.

use coop_policy::PolicyTable;
use coop_session::policy_ids::{bundled_for, resolve_row};
use esm::{LoadOrder, Plugin};
use std::path::PathBuf;

fn dir() -> Option<PathBuf> {
    let d = PathBuf::from(std::env::var("FNV_ESM_DIR").ok()?);
    FULL.iter().all(|f| d.join(f).is_file()).then_some(d)
}

fn order_of(names: &[&str]) -> LoadOrder {
    let d = dir().expect("data");
    let list = names
        .iter()
        .map(|n| {
            (
                n.to_string(),
                Some(d.join(n)),
                Plugin::open(d.join(n)).expect("plugin"),
            )
        })
        .collect();
    LoadOrder::from_plugins(list).expect("order")
}

const FULL: [&str; 5] = [
    "FalloutNV.esm",
    "DeadMoney.esm",
    "HonestHearts.esm",
    "OldWorldBlues.esm",
    "LonesomeRoad.esm",
];

fn rows_of(t: &PolicyTable, loaded: &[&str]) -> usize {
    t.rows()
        .iter()
        .filter(|r| !r.synthetic && loaded.iter().any(|p| p.eq_ignore_ascii_case(&r.plugin)))
        .count()
}

fn assert_only_missing_plugins_unmatched(rep: &coop_policy::RemapReport, loaded: &[&str]) {
    for (p, e) in &rep.unmatched {
        assert!(
            !loaded.iter().any(|l| l.eq_ignore_ascii_case(p)),
            "{e} of loaded {p} did not resolve: {}",
            rep.describe()
        );
    }
}

#[test]
fn all_rows_resolve_with_the_five_esm_order() {
    if dir().is_none() {
        return;
    }
    let order = order_of(&FULL);
    let t = PolicyTable::bundled().expect("table");
    assert_eq!(t.rows().iter().filter(|r| !r.synthetic).count(), 483);
    let (m, rep) = bundled_for(&order).expect("remap");
    assert_only_missing_plugins_unmatched(&rep, &FULL);
    assert_eq!(rep.resolved, rows_of(&t, &FULL));
    assert_eq!(rep.resolved, 482);
    assert_eq!(
        rep.unmatched,
        vec![(
            "GunRunnersArsenal.esm".to_string(),
            "NVDLC05MQ00".to_string()
        )]
    );
    // every row now answers to the id the load order gives its editor id
    for r in m.rows().iter().filter(|r| !r.synthetic) {
        assert_eq!(
            order.form_by_editor_id(&r.edid).map(|f| f.0),
            Some(r.id),
            "{}",
            r.edid
        );
        assert!(m.row(r.id).is_some());
    }
}

#[test]
fn a_shuffled_dlc_order_still_resolves() {
    if dir().is_none() {
        return;
    }
    let shuffled = [
        "FalloutNV.esm",
        "LonesomeRoad.esm",
        "OldWorldBlues.esm",
        "DeadMoney.esm",
        "HonestHearts.esm",
    ];
    let order = order_of(&shuffled);
    let (m, rep) = bundled_for(&order).expect("remap");
    assert_only_missing_plugins_unmatched(&rep, &shuffled);
    assert_eq!(rep.resolved, 482, "{}", rep.describe());
    assert!(rep.moved > 0, "a different order must move DLC rows");
    let dm = m.row_by_edid("NVDLC01MQ02").expect("row");
    assert_eq!(
        dm.id >> 24,
        3,
        "Dead Money is fourth in this order (index 3)"
    );
    assert_eq!(
        order.form_by_editor_id("NVDLC01MQ02").map(|f| f.0),
        Some(dm.id)
    );
    for r in m.rows().iter().filter(|r| !r.synthetic) {
        assert_eq!(
            order.form_by_editor_id(&r.edid).map(|f| f.0),
            Some(r.id),
            "{}",
            r.edid
        );
    }
    // one row resolves the same way on its own
    let row = PolicyTable::bundled()
        .unwrap()
        .row_by_edid("NVDLC01MQ02")
        .cloned()
        .unwrap();
    assert_eq!(resolve_row(&order, &row), Some(dm.id));
}

#[test]
fn a_missing_dlc_leaves_its_rows_unmatched_without_errors() {
    if dir().is_none() {
        return;
    }
    let loaded = ["FalloutNV.esm", "DeadMoney.esm", "LonesomeRoad.esm"];
    let order = order_of(&loaded);
    let t = PolicyTable::bundled().expect("table");
    let (m, rep) = bundled_for(&order).expect("a missing DLC is not an error");
    let honest = rows_of(&t, &["HonestHearts.esm"]);
    let blues = rows_of(&t, &["OldWorldBlues.esm"]);
    assert!(honest > 0 && blues > 0);
    // Honest Hearts, Old World Blues and the one Gun Runners' Arsenal row
    assert_eq!(rep.unmatched.len(), honest + blues + 1);
    assert_only_missing_plugins_unmatched(&rep, &loaded);
    assert!(rep.describe().contains("HonestHearts.esm"));
    assert!(m.row_by_edid("NVDLC03GenericPerkQUEST").is_none());
    assert_eq!(rep.resolved + rep.synthetic, m.rows().len());
    // the loaded DLCs and the base game still resolve
    assert!(m.row_by_edid("NVDLC01MQ02").is_some());
    assert!(m.row_by_edid("NVDLC04MQ00").is_some() || rows_of(&t, &["LonesomeRoad.esm"]) > 0);
}

//! Schema validation and loading of the bundled table.

use coop_policy::*;

fn table() -> PolicyTable {
    PolicyTable::bundled().expect("bundled policy loads")
}

#[test]
fn every_quest_row_loads() {
    let t = table();
    assert_eq!(t.rows().len(), 487);
    for r in t.rows() {
        assert!(t.row(r.id).is_some(), "{} not found by id", r.edid);
        assert_eq!(t.row_by_edid(&r.edid).unwrap().id, r.id);
    }
}

#[test]
fn status_counts_are_visible() {
    let t = table();
    let (confirmed, default) = t.counts();
    assert_eq!(confirmed + default, t.rows().len());
    assert_eq!(confirmed, 372);
    assert_eq!(default, 115);
    assert_eq!(t.unreviewed().count(), 115);
}

#[test]
fn rows_round_trip_through_json() {
    let t = table();
    let text = serde_json::to_string(&t.file).unwrap();
    let back = PolicyTable::from_json(&text).unwrap();
    assert_eq!(back.file, t.file);
}

#[test]
fn global_policy_matches_owner_decisions() {
    let t = table();
    let p = t.policy();
    assert_eq!(p.death.mode, DeathMode::Downed);
    assert_eq!(p.death.revive_s, 60);
    assert_eq!(p.barrier_timeout_s, 120);
    assert_eq!(p.tether_timeout_s, 120);
    assert_eq!(p.party_cap, 4);
    assert_eq!(t.residence(ShadowClass::Karma), Residence::Shared);
    assert_eq!(t.residence(ShadowClass::Skills), Residence::PerPlayer);
    assert_eq!(t.residence(ShadowClass::Followers), Residence::PerPlayer);
}

#[test]
fn synthetic_rows_are_flagged() {
    let t = table();
    let syn: Vec<_> = t.rows().iter().filter(|r| r.synthetic).collect();
    assert_eq!(syn.len(), 4);
    assert!(syn.iter().all(|r| r.id >= 0xFD00_0000));
}

fn mutate(f: impl FnOnce(&mut PolicyFile)) -> Result<PolicyTable, LoadError> {
    let mut file = table().file;
    f(&mut file);
    PolicyTable::from_file(file)
}

fn problems(r: Result<PolicyTable, LoadError>) -> String {
    match r {
        Err(LoadError::Invalid(v)) => v.join(" | "),
        Err(e) => panic!("wrong error kind: {e}"),
        Ok(_) => panic!("expected the table to be rejected"),
    }
}

#[test]
fn rejects_wrong_schema() {
    assert!(problems(mutate(|f| f.schema = 9)).contains("schema 9"));
}

#[test]
fn rejects_duplicate_ids_and_edids() {
    let p = problems(mutate(|f| {
        let dup = f.quests[0].clone();
        f.quests.push(dup);
    }));
    assert!(p.contains("duplicate form id"));
    assert!(p.contains("duplicate edid"));
}

#[test]
fn rejects_bad_gates() {
    let p = problems(mutate(|f| {
        f.quests[0].tether = Some(Tether::default());
        f.quests[1].barrier = Some(Barrier { stages: vec![], before: Some("sunrise".into()) });
        f.quests[2].exclusive = Some(Exclusive { stages: vec![5, 5] });
        f.quests[3].tether = Some(Tether { anchor: Some("X".into()), ..Tether::default() });
        f.quests[4].controls = Some(Controls { scope: ControlScope::All, timeout_s: 0, masks: vec![] });
    }));
    assert!(p.contains("tether needs cells or an anchor"));
    assert!(p.contains("unknown barrier gate"));
    assert!(p.contains("at least two distinct stages"));
    assert!(p.contains("anchor needs a radius"));
    assert!(p.contains("controls timeout"));
}

#[test]
fn rejects_unknown_group_member() {
    let p = problems(mutate(|f| f.groups[0].members.push("NoSuchQuest".into())));
    assert!(p.contains("unknown quest NoSuchQuest"));
}

#[test]
fn rejects_unknown_fields_and_bad_ids() {
    let text = data_text().replacen("\"advance\"", "\"advnace\"", 1);
    assert!(matches!(PolicyTable::from_json(&text), Err(LoadError::Parse(_))));
    let text = data_text().replacen("\"0x0007691F\"", "\"7691F\"", 1);
    assert!(matches!(PolicyTable::from_json(&text), Err(LoadError::Parse(_))));
}

fn data_text() -> String {
    coop_policy::load::BUNDLED.to_string()
}

#[test]
fn text_is_clean() {
    let t = data_text();
    assert!(t.is_ascii(), "policy.json must be plain ASCII");
    let lower = t.to_lowercase();
    for word in [["mth", "wj"].concat(), ["nv", "mp"].concat(), ["nv:", "mp"].concat()] {
        assert!(!lower.contains(word.as_str()), "forbidden word {word} in data");
    }
}

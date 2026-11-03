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
    assert_eq!(confirmed, 468);
    assert_eq!(default, 19);
    assert_eq!(t.unreviewed().count(), 19);
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
        f.quests[1].barrier = Some(Barrier {
            stages: vec![],
            before: Some("sunrise".into()),
        });
        f.quests[2].exclusive = Some(Exclusive { stages: vec![5, 5] });
        f.quests[3].tether = Some(Tether {
            anchor: Some("X".into()),
            ..Tether::default()
        });
        f.quests[4].controls = Some(Controls {
            scope: ControlScope::All,
            timeout_s: 0,
            masks: vec![],
        });
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
    assert!(matches!(
        PolicyTable::from_json(&text),
        Err(LoadError::Parse(_))
    ));
    let text = data_text().replacen("\"0x0007691F\"", "\"7691F\"", 1);
    assert!(matches!(
        PolicyTable::from_json(&text),
        Err(LoadError::Parse(_))
    ));
}

fn data_text() -> String {
    coop_policy::load::BUNDLED.to_string()
}

#[test]
fn remap_keys_rows_by_the_resolver_and_drops_what_is_not_loaded() {
    let t = PolicyTable::bundled().unwrap();
    let blues = t
        .rows()
        .iter()
        .filter(|r| r.plugin == "OldWorldBlues.esm")
        .count();
    assert!(blues > 0);
    // a load order where Old World Blues is absent and Dead Money sits at index 4
    let (m, rep) = t
        .remap(|r| match r.plugin.as_str() {
            "OldWorldBlues.esm" => None,
            "DeadMoney.esm" => Some((4 << 24) | r.local),
            _ => Some(r.id),
        })
        .expect("a missing plugin is not an error");
    assert_eq!(rep.unmatched.len(), blues);
    assert!(rep.moved > 0);
    assert_eq!(m.rows().len(), t.rows().len() - blues);
    assert!(m.row_by_edid("NVDLC01MQ02").unwrap().id >> 24 == 4);
    assert!(
        m.row(t.row_by_edid("NVDLC01MQ02").unwrap().id).is_none(),
        "the old id no longer answers"
    );
    assert!(rep.describe().contains("OldWorldBlues.esm"));
    assert!(m
        .rows()
        .iter()
        .all(|r| r.local == t.row_by_edid(&r.edid).unwrap().local));
}

#[test]
fn text_is_clean() {
    let t = data_text();
    assert!(t.is_ascii(), "policy.json must be plain ASCII");
    let lower = t.to_lowercase();
    for word in [
        ["mth", "wj"].concat(),
        ["nv", "mp"].concat(),
        ["nv:", "mp"].concat(),
    ] {
        assert!(
            !lower.contains(word.as_str()),
            "forbidden word {word} in data"
        );
    }
}

const RESOLVED_TETHERS: [&str; 29] = [
    "NVDLC02Ending",
    "NVDLC02IntroSlides",
    "NVDLC02MQ00",
    "NVDLC02MQ01",
    "NVDLC02MQ02",
    "NVDLC02MQ02d",
    "NVDLC02MQ03d",
    "NVDLC02MQ04",
    "NVDLC02MQ05",
    "NVDLC02MQ06",
    "NVDLC02MS04",
    "NVDLC03MQ00",
    "VCG03",
    "VCassCompanion",
    "VEnding",
    "VHDKimballSpeech",
    "VHDOliverDeathTimer",
    "VMQ03aTimer",
    "VMQ03bTimer",
    "VMQ04",
    "VMS01",
    "VMS15",
    "VMS16",
    "VMS16b",
    "VMS29",
    "VMS29a",
    "VMS40",
    "VMS56",
    "nvNelsonExplorerBark",
];

const RESOLVED_WARPS: [&str; 3] = [
    "NVDLC01NewVegasFollowerFireQuest",
    "NVDLC03X8ExperimentQuest",
    "vDialogueEDE",
];

#[test]
fn resolved_tethers_and_warps_exist_and_validate() {
    let t = table();
    for e in RESOLVED_TETHERS {
        let r = t.row_by_edid(e).unwrap_or_else(|| panic!("{e} missing"));
        let th = r
            .tether
            .as_ref()
            .unwrap_or_else(|| panic!("{e} has no tether"));
        assert!(
            !th.cells.is_empty() || th.anchor.is_some(),
            "{e}: tether names nothing"
        );
        if th.anchor.is_some() {
            assert!(th.radius.unwrap_or(0) > 0, "{e}: anchor without radius");
        }
        assert!(th.cells.iter().all(|c| !c.is_empty()));
    }
    for e in RESOLVED_WARPS {
        let r = t.row_by_edid(e).unwrap_or_else(|| panic!("{e} missing"));
        let w = r.warp.as_ref().unwrap_or_else(|| panic!("{e} has no warp"));
        assert!(
            !w.markers.is_empty() && w.markers.iter().all(|m| !m.is_empty()),
            "{e}: bad markers"
        );
    }
    // the table as a whole still validates after a round trip
    let text = serde_json::to_string(&t.file).unwrap();
    PolicyTable::from_json(&text).expect("table validates");
}

#[test]
fn no_default_gate_row_lets_anyone_advance() {
    let t = table();
    for r in t.unreviewed() {
        if matches!(r.tier, Tier::T3 | Tier::T4) {
            assert_ne!(
                r.advance,
                Advance::Any,
                "{} is a default {:?} row with advance any",
                r.edid,
                r.tier
            );
        }
        if r.synthetic {
            assert_eq!(
                r.advance,
                Advance::Host,
                "synthetic {} must advance by host",
                r.edid
            );
        }
    }
}

#[test]
fn unreviewed_lists_exactly_the_remaining_rows() {
    let t = table();
    let mut got: Vec<&str> = t.unreviewed().map(|r| r.edid.as_str()).collect();
    got.sort();
    let mut want = vec![
        "Doctors",
        "FerociousLoyaltyQuest",
        "FriendOfTheNightQuest",
        "Generic",
        "NVDLC02MQ07",
        "NVDLC03AutoDoc",
        "NVDLC03Teleporter",
        "NVDLC03ThinkTankWeaponLock",
        "NVDLC04WeakPlayerScaling",
        "PerkWildWasteland",
        "VDoctors",
        "VFactionSquad",
        "VFreeformCampGolf",
        "VMS13",
        "VMS33",
        "VMS50",
        "VTechatticup",
        "vDialogueSL",
        "vFreeformForlornHope03",
    ];
    want.sort();
    assert_eq!(got, want);
}

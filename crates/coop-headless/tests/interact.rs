//! The player-world interaction scenarios (barter, door follow, world item
//! pickup, lockpick unlock, crafting, vendor stock) on a generated world with
//! the headless simulator, each with its mutation twin, and a soak of all of
//! them over every chaos profile. The real-data versions of the same scenarios
//! are in `tools/coop-run/scenarios` (`live_barter.json` and so on).

use std::sync::Arc;

use coop_headless::data::Data;
use coop_headless::scenario::{Scenario, PROFILES};
use coop_headless::sim::{run, RunOpts};
use testdata::{f32s, group, placed, record, sub, zstr, TempData};

const CELL: u32 = 0xD00;
const OTHER: u32 = 0xD01;

/// A room with a vendor, a swing door, a locked door and a load door to a
/// second room; two items, caps, a recipe.
fn world(tag: &str) -> TempData {
    let data = TempData::empty(tag);
    let edid = |s: &str| sub(b"EDID", &zstr(s));
    let named = |kind: &[u8; 4], id: u32, name: &str, rest: &[u8]| {
        let mut d = edid(name);
        d.extend(rest);
        record(kind, id, &d)
    };
    let mut hedr = 1.34f32.to_le_bytes().to_vec();
    hedr.extend([0; 8]);
    let mut plugin = record(b"TES4", 0, &sub(b"HEDR", &hedr));
    plugin.extend(group(
        *b"DOOR",
        0,
        &named(b"DOOR", 0xA00, "IxDoor", &sub(b"FNAM", &[0])),
    ));
    let mut misc = named(b"MISC", 0xA10, "IxItemA", &[]);
    misc.extend(named(b"MISC", 0xA11, "IxScrap", &[]));
    misc.extend(named(b"MISC", 0xA12, "IxRound", &[]));
    misc.extend(named(b"MISC", 0xF, "Caps001", &[]));
    plugin.extend(group(*b"MISC", 0, &misc));
    plugin.extend(group(*b"STAT", 0, &named(b"STAT", 0xA20, "IxStat", &[])));
    // a recipe: 1 scrap makes 10 rounds, no skill
    let mut rc = sub(b"FULL", &zstr("Rounds"));
    let mut dat = (-1i32).to_le_bytes().to_vec();
    dat.extend(0u32.to_le_bytes());
    dat.extend(0u32.to_le_bytes());
    dat.extend(0u32.to_le_bytes());
    rc.extend(sub(b"DATA", &dat));
    rc.extend(sub(b"RCIL", &0xA11u32.to_le_bytes()));
    rc.extend(sub(b"RCQY", &1u32.to_le_bytes()));
    rc.extend(sub(b"RCOD", &0xA12u32.to_le_bytes()));
    rc.extend(sub(b"RCQY", &10u32.to_le_bytes()));
    plugin.extend(group(*b"RCPE", 0, &named(b"RCPE", 0xB00, "IxRecipe", &rc)));
    let with_id = |name: &str| sub(b"EDID", &zstr(name));
    let load_door = |id: u32, name: &str, pos: [f32; 3], to: u32, arrive: [f32; 3]| {
        let mut xtel = to.to_le_bytes().to_vec();
        xtel.extend(f32s(&[arrive[0], arrive[1], arrive[2], 0.0, 0.0, 0.0]));
        xtel.extend(0u32.to_le_bytes());
        let mut extra = with_id(name);
        extra.extend(sub(b"XTEL", &xtel));
        let mut r = placed(id, 0xA00, pos, [0.0; 3], &extra);
        r[8..12].copy_from_slice(&0x400u32.to_le_bytes());
        r
    };
    let plain = |id: u32, base: u32, name: &str, pos: [f32; 3]| {
        placed(id, base, pos, [0.0; 3], &with_id(name))
    };
    let mut room = load_door(
        0xD50,
        "IxLoadDoor",
        [0.0, 0.0, 0.0],
        0xD51,
        [500.0, 0.0, 0.0],
    );
    room.extend(plain(0xD10, 0xA00, "IxSwingDoor", [50.0, 0.0, 0.0]));
    room.extend(plain(0xD11, 0xA00, "IxLockedDoor", [100.0, 0.0, 0.0]));
    room.extend(plain(0xD20, 0xA20, "IxVendorREF", [0.0, 100.0, 0.0]));
    room.extend(plain(0xD40, 0xA20, "IxTerminalREF", [0.0, 300.0, 0.0]));
    let other = load_door(
        0xD51,
        "IxOtherDoor",
        [500.0, 0.0, 0.0],
        0xD50,
        [0.0, 0.0, 0.0],
    );
    let cell = |id: u32, name: &str, refs: &[u8]| {
        let mut c = named(b"CELL", id, name, &sub(b"DATA", &[1]));
        c.extend(group(
            id.to_le_bytes(),
            6,
            &group(id.to_le_bytes(), 8, refs),
        ));
        c
    };
    let mut cells = cell(CELL, "IxRoom", &room);
    cells.extend(cell(OTHER, "IxOther", &other));
    plugin.extend(group(
        *b"CELL",
        0,
        &group([0; 4], 2, &group([0; 4], 3, &cells)),
    ));
    data.write("FalloutNV.esm", &plugin);
    data
}

fn data(tag: &str) -> (TempData, Arc<Data>) {
    let t = world(tag);
    let d = Arc::new(Data::open(t.path()).expect("generated data opens"));
    (t, d)
}

fn scn(name: &str, network: &str, host: &str, guest: &str, expect: &str) -> Scenario {
    let text = format!(
        r#"{{"name":"{name}","cell":"IxRoom","network":"{network}","seed":7,"duration_s":30,"settle_s":6,
        "host":{{"script":[{host}]}},"guest":{{"script":[{guest}]}},"expect":[{expect}]}}"#
    );
    Scenario::parse(&text, name).unwrap_or_else(|e| panic!("{name}: {e:?}\n{text}"))
}

fn line(at: f64, run: &str) -> String {
    format!(r#"{{"at":{at},"run":"{run}"}}"#)
}

fn eq(id: &str, path: &str, v: i64, on: &str) -> String {
    format!(r#"{{"id":"{id}","path":"{path}","op":"eq","value":{v},"on":"{on}"}}"#)
}

fn ge(id: &str, path: &str, v: i64, on: &str) -> String {
    format!(r#"{{"id":"{id}","path":"{path}","op":"ge","value":{v},"on":"{on}"}}"#)
}

fn present(id: &str, path: &str, on: &str) -> String {
    format!(r#"{{"id":"{id}","path":"{path}","op":"present","on":"{on}"}}"#)
}

fn absent(id: &str, path: &str, on: &str) -> String {
    format!(r#"{{"id":"{id}","path":"{path}","op":"absent","on":"{on}"}}"#)
}

const SHARED: &str = r#"{"id":"shared-hash","path":"hash.shared","on":"equal"}"#;

/// One scenario: (name, host lines, guest lines, expectations).
struct Case {
    name: &'static str,
    host: Vec<String>,
    guest: Vec<String>,
    /// The guest lines the twin leaves out (all of them).
    twin_host: Option<Vec<String>>,
    expect: Vec<String>,
}

fn cases() -> Vec<Case> {
    let both = |id: &str, path: &str, v: i64| {
        vec![
            eq(&format!("{id}-host"), path, v, "host"),
            eq(&format!("{id}-guest"), path, v, "guest"),
        ]
    };
    let mut barter_expect = both("bought", "items.guest.IxItemA", 2);
    barter_expect.extend(both("paid", "items.guest.Caps001", 20));
    barter_expect.extend(both("stock", "items.IxVendorREF.IxItemA", 3));
    barter_expect.extend(both("till", "items.IxVendorREF.Caps001", 30));
    barter_expect.push(ge("counted", "interact.barter", 1, "guest"));
    barter_expect.push(SHARED.into());

    let mut door = vec![
        ge("guest-followed", "live.doors_followed", 1, "guest"),
        ge("guest-told", "interact.follows_in", 1, "guest"),
        ge("host-led", "interact.transition", 1, "host"),
    ];
    door.push(r#"{"id":"together","path":"positions.guest","op":"near","value":[500,0],"tol":80,"dims":2,"on":"host"}"#.into());
    door.push(SHARED.into());

    let mut pickup = both("guest-has", "items.guest.IxItemA", 2);
    pickup.extend(both("host-kept", "items.host.IxItemA", 1));
    pickup.push(present("gone-host", "disabled.FD000001", "host"));
    pickup.push(present("gone-guest", "disabled.FD000001", "guest"));
    pickup.push(SHARED.into());

    let mut lock = both("unlocked", "objects.locks.IxLockedDoor", -1);
    lock.push(ge("counted", "interact.lockpick", 1, "guest"));
    lock.push(SHARED.into());

    let mut craft = both("made", "items.guest.IxRound", 20);
    craft.extend(both("spent", "items.guest.IxScrap", 1));
    craft.push(absent("host-not-paid", "items.host.IxRound", "host"));
    craft.push(SHARED.into());

    let mut vendor = both("stock", "items.IxVendorREF.IxItemA", 2);
    vendor.extend(both("guest", "items.guest.IxItemA", 2));
    vendor.extend(both("host", "items.host.IxItemA", 1));
    vendor.push(SHARED.into());

    vec![
        Case {
            name: "live_barter",
            host: vec![
                line(2.0, "coop give IxVendorREF IxItemA 5"),
                line(2.0, "coop give 1 Caps001 50"),
            ],
            guest: vec![line(9.0, "coop barter IxVendorREF -30 buy:IxItemA:2")],
            twin_host: None,
            expect: barter_expect,
        },
        Case {
            name: "live_door_follow",
            host: vec![line(9.0, "coop enter IxLoadDoor IxRoom IxOther")],
            guest: vec![],
            twin_host: Some(vec![]),
            expect: door,
        },
        Case {
            name: "live_pickup_world_item",
            host: vec![
                line(2.0, "coop give 0 IxItemA 3"),
                line(4.0, "coop drop IxItemA 2"),
            ],
            guest: vec![line(10.0, "coop pickup FD000001 IxItemA 2")],
            twin_host: None,
            expect: pickup,
        },
        Case {
            name: "live_lockpick_unlock_shared",
            host: vec![line(2.0, "coop setlock IxLockedDoor 2")],
            guest: vec![line(10.0, "coop lockpick IxLockedDoor won 2")],
            twin_host: None,
            expect: lock,
        },
        Case {
            name: "live_craft_per_player",
            host: vec![line(2.0, "coop give 1 IxScrap 3")],
            guest: vec![line(10.0, "coop craft IxVendorREF IxRecipe 2")],
            twin_host: None,
            expect: craft,
        },
        Case {
            name: "live_vendor_stock_shared",
            host: vec![
                line(2.0, "coop give IxVendorREF IxItemA 5"),
                line(16.0, "coop barter IxVendorREF 0 buy:IxItemA:1"),
            ],
            guest: vec![line(9.0, "coop barter IxVendorREF 0 buy:IxItemA:2")],
            twin_host: Some(vec![line(2.0, "coop give IxVendorREF IxItemA 5")]),
            expect: vendor,
        },
    ]
}

fn build(c: &Case, network: &str, twin: bool) -> Scenario {
    let (host, guest) = if twin {
        (
            c.twin_host.clone().unwrap_or_else(|| c.host.clone()),
            if c.twin_host.is_some() {
                c.guest.clone()
            } else {
                vec![]
            },
        )
    } else {
        (c.host.clone(), c.guest.clone())
    };
    let mut s = scn(
        c.name,
        network,
        &host.join(","),
        &guest.join(","),
        &c.expect.join(","),
    );
    s.expect_fail = twin;
    s
}

#[test]
fn every_interaction_scenario_passes_on_a_good_link() {
    for c in cases() {
        let (_t, d) = data(&format!("hl-{}", c.name));
        let r = run(&build(&c, "good_wifi", false), &d, &RunOpts::default());
        assert_eq!(
            r.verdict.result,
            "pass",
            "{}: {}\n{}",
            c.name,
            r.verdict.reason,
            r.dumps.join("\n")
        );
    }
}

#[test]
fn every_mutation_twin_fails_as_expected() {
    for c in cases() {
        let (_t, d) = data(&format!("hl-twin-{}", c.name));
        let r = run(&build(&c, "good_wifi", true), &d, &RunOpts::default());
        assert_eq!(
            (r.verdict.result.as_str(), r.verdict.class.as_str()),
            ("pass", "twin"),
            "{}: {}",
            c.name,
            r.verdict.reason
        );
    }
}

#[test]
fn the_interaction_scenarios_hold_over_every_chaos_profile() {
    for c in cases() {
        for profile in PROFILES {
            let (_t, d) = data(&format!("hl-soak-{}-{profile}", c.name));
            let r = run(&build(&c, profile, false), &d, &RunOpts::default());
            assert_eq!(
                r.verdict.result,
                "pass",
                "{} on {profile}: {}\n{}",
                c.name,
                r.verdict.reason,
                r.dumps.join("\n")
            );
        }
    }
}

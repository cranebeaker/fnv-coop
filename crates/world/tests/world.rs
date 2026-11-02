//! Cell loading tests against a small plugin assembled byte by byte.

use esm::{flags, FormId, LoadOrder, Plugin};
use world::{find_cells, interior_cells, load_cell, Emittance, RotationConvention};

fn sub(kind: &[u8; 4], data: &[u8]) -> Vec<u8> {
    let mut v = kind.to_vec();
    v.extend((data.len() as u16).to_le_bytes());
    v.extend(data);
    v
}

fn zstr(s: &str) -> Vec<u8> {
    let mut v = s.as_bytes().to_vec();
    v.push(0);
    v
}

fn record(kind: &[u8; 4], form_id: u32, flags: u32, data: &[u8]) -> Vec<u8> {
    let mut v = kind.to_vec();
    v.extend((data.len() as u32).to_le_bytes());
    v.extend(flags.to_le_bytes());
    v.extend(form_id.to_le_bytes());
    v.extend([0; 4]);
    v.extend(15u16.to_le_bytes());
    v.extend([0; 2]);
    v.extend(data);
    v
}

fn group(label: [u8; 4], group_type: i32, contents: &[u8]) -> Vec<u8> {
    let mut v = b"GRUP".to_vec();
    v.extend(((24 + contents.len()) as u32).to_le_bytes());
    v.extend(label);
    v.extend(group_type.to_le_bytes());
    v.extend([0; 8]);
    v.extend(contents);
    v
}

fn tes4(masters: &[&str]) -> Vec<u8> {
    let mut hedr = 1.34f32.to_le_bytes().to_vec();
    hedr.extend(0i32.to_le_bytes());
    hedr.extend(0x800u32.to_le_bytes());
    let mut data = sub(b"HEDR", &hedr);
    for m in masters {
        data.extend(sub(b"MAST", &zstr(m)));
        data.extend(sub(b"DATA", &0u64.to_le_bytes()));
    }
    let flag = if masters.is_empty() { flags::MASTER } else { 0 };
    record(b"TES4", 0, flag, &data)
}

fn floats(values: &[f32]) -> Vec<u8> {
    values.iter().flat_map(|v| v.to_le_bytes()).collect()
}

fn base(kind: &[u8; 4], id: u32, editor_id: &str, extra: &[u8]) -> Vec<u8> {
    let mut data = sub(b"EDID", &zstr(editor_id));
    data.extend(extra);
    record(kind, id, 0, &data)
}

fn with_model(kind: &[u8; 4], id: u32, editor_id: &str, model: &str) -> Vec<u8> {
    base(kind, id, editor_id, &sub(b"MODL", &zstr(model)))
}

/// A reference to `base` at `pos` with `rot`, plus extra subrecords.
fn placed(kind: &[u8; 4], id: u32, flags: u32, base: u32, pos: [f32; 3], extra: &[u8]) -> Vec<u8> {
    let mut data = sub(b"NAME", &base.to_le_bytes());
    data.extend(extra);
    data.extend(sub(
        b"DATA",
        &floats(&[pos[0], pos[1], pos[2], 0.0, 0.0, 0.5]),
    ));
    record(kind, id, flags, &data)
}

fn xtel(door: u32, pos: [f32; 3], heading: f32) -> Vec<u8> {
    let mut d = door.to_le_bytes().to_vec();
    d.extend(floats(&[pos[0], pos[1], pos[2], 0.0, 0.0, heading]));
    d.extend(0u32.to_le_bytes());
    sub(b"XTEL", &d)
}

fn xesp(parent: u32, opposite: bool) -> Vec<u8> {
    let mut d = parent.to_le_bytes().to_vec();
    d.extend([u8::from(opposite), 0, 0, 0]);
    sub(b"XESP", &d)
}

fn lighting(ambient: [u8; 3], directional: [u8; 3]) -> Vec<u8> {
    let mut d = vec![ambient[0], ambient[1], ambient[2], 0];
    d.extend([directional[0], directional[1], directional[2], 0]);
    d.extend([10, 20, 30, 0]);
    d.extend(floats(&[64.0, 4000.0]));
    d.extend(45i32.to_le_bytes());
    d.extend(30i32.to_le_bytes());
    d.extend(floats(&[1.0, 5000.0, 1.0]));
    d
}

/// An interior cell with its contents in the temporary children group.
fn interior(id: u32, editor_id: &str, name: &str, extra: &[u8], refs: &[u8]) -> Vec<u8> {
    let mut data = sub(b"EDID", &zstr(editor_id));
    data.extend(sub(b"FULL", &zstr(name)));
    data.extend(sub(b"DATA", &[1]));
    data.extend(extra);
    let mut contents = record(b"CELL", id, 0, &data);
    let temporary = group(id.to_le_bytes(), 9, refs);
    contents.extend(group(id.to_le_bytes(), 6, &temporary));
    contents
}

fn cells_group(cells: &[u8]) -> Vec<u8> {
    let sub_block = group(0i32.to_le_bytes(), 3, cells);
    group(*b"CELL", 0, &group(0i32.to_le_bytes(), 2, &sub_block))
}

/// Master.esm: base objects and two connected rooms.
fn master() -> Vec<u8> {
    let mut file = tes4(&[]);

    let mut stats = with_model(b"STAT", 0x0000_0032, "COCMarkerHeading", "marker_coc.nif");
    stats.extend(with_model(b"STAT", 0x0000_003B, "XMarker", "marker_x.nif"));
    stats.extend(with_model(
        b"STAT",
        0x0000_0900,
        "Wall",
        "Architecture\\Wall01.nif",
    ));
    stats.extend(with_model(
        b"STAT",
        0x0000_0901,
        "Crate",
        "Clutter\\Crate.nif",
    ));
    stats.extend(with_model(
        b"STAT",
        0x0000_0902,
        "TravelMarkerish",
        "Markers\\Travel.nif",
    ));
    file.extend(group(*b"STAT", 0, &stats));

    let mut scol = sub(b"MODL", &zstr("scol\\walls.nif"));
    scol.extend(sub(b"ONAM", &0x0000_0900u32.to_le_bytes()));
    scol.extend(sub(
        b"DATA",
        &floats(&[
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 256.0, 0.0, 0.0, 0.0, 0.0, 1.5, 2.0,
        ]),
    ));
    file.extend(group(
        *b"SCOL",
        0,
        &base(b"SCOL", 0x0000_0910, "WallPair", &scol),
    ));

    let mut armor = sub(b"MODL", &zstr("armor\\worn.nif"));
    armor.extend(sub(b"MOD2", &zstr("armor\\ground.nif")));
    file.extend(group(
        *b"ARMO",
        0,
        &base(b"ARMO", 0x0000_0920, "Jacket", &armor),
    ));

    let mut light = vec![0u8; 32];
    light[0..4].copy_from_slice(&(-1i32).to_le_bytes());
    light[4..8].copy_from_slice(&300u32.to_le_bytes());
    light[8..12].copy_from_slice(&[255, 200, 150, 0]);
    light[16..20].copy_from_slice(&1.0f32.to_le_bytes());
    light[20..24].copy_from_slice(&90.0f32.to_le_bytes());
    let mut off = light.clone();
    off[12..16].copy_from_slice(&0x20u32.to_le_bytes());
    let mut lights = base(b"LIGH", 0x0000_0930, "WarmLight", &sub(b"DATA", &light));
    lights.extend(base(b"LIGH", 0x0000_0931, "OffLight", &sub(b"DATA", &off)));
    file.extend(group(*b"LIGH", 0, &lights));

    file.extend(group(
        *b"DOOR",
        0,
        &with_model(b"DOOR", 0x0000_0940, "HouseDoor", "Doors\\Door01.nif"),
    ));
    file.extend(group(*b"SOUN", 0, &base(b"SOUN", 0x0000_0950, "Hum", &[])));

    // A region with weather data (`RDAT` type 3, then `RDWT`), whose
    // likeliest weather has a sunlight color by day and at high noon.
    let mut rdwt = Vec::new();
    for (weather, chance) in [(0x0000_09A2u32, 30u32), (0x0000_09A1, 70)] {
        rdwt.extend(weather.to_le_bytes());
        rdwt.extend(chance.to_le_bytes());
        rdwt.extend(0u32.to_le_bytes());
    }
    let mut region = sub(b"RDAT", &[3, 0, 0, 0, 0, 50, 0, 0]);
    region.extend(sub(b"RDWT", &rdwt));
    file.extend(group(
        *b"REGN",
        0,
        &base(b"REGN", 0x0000_09A0, "IndoorRegion", &region),
    ));
    // NAM0: 10 colors × 6 times of day; Sunlight (5th) by day (2nd) and at
    // high noon (5th), as `NVWastelandInterior`'s.
    let mut nam0 = vec![0u8; 240];
    nam0[(4 * 6 + 1) * 4..(4 * 6 + 1) * 4 + 4].copy_from_slice(&[255, 227, 170, 0]);
    nam0[(4 * 6 + 4) * 4..(4 * 6 + 4) * 4 + 4].copy_from_slice(&[255, 227, 170, 0]);
    let mut other = vec![0u8; 240];
    other[(4 * 6 + 1) * 4..(4 * 6 + 1) * 4 + 4].copy_from_slice(&[1, 2, 3, 0]);
    let mut default = vec![0u8; 240];
    default[(4 * 6 + 1) * 4..(4 * 6 + 1) * 4 + 4].copy_from_slice(&[244, 206, 149, 0]);
    let mut weathers = base(
        b"WTHR",
        0x0000_015E,
        "DefaultWeather",
        &sub(b"NAM0", &default),
    );
    weathers.extend(base(b"WTHR", 0x0000_09A1, "Indoor", &sub(b"NAM0", &nam0)));
    weathers.extend(base(b"WTHR", 0x0000_09A2, "Rare", &sub(b"NAM0", &other)));
    file.extend(group(*b"WTHR", 0, &weathers));
    file.extend(group(*b"NPC_", 0, &base(b"NPC_", 0x0000_0960, "Doc", &[])));
    file.extend(group(
        *b"LGTM",
        0,
        &base(
            b"LGTM",
            0x0000_0970,
            "DimTemplate",
            &sub(b"DATA", &lighting([90, 90, 90], [1, 2, 3])),
        ),
    ));

    // A warm image space: cinematic values after 25 HDR, bloom, get-hit and
    // night-eye values, then padding and flags (152 bytes in all).
    let mut dnam = vec![0.0f32; 38];
    dnam[25..33].copy_from_slice(&[0.8, 0.5, 1.2, 1.1, 1.0, 0.8, 0.5, 0.3]);
    file.extend(group(
        *b"IMGS",
        0,
        &base(
            b"IMGS",
            0x0000_0980,
            "WarmSpace",
            &sub(b"DNAM", &floats(&dnam)),
        ),
    ));

    // The house.
    // The wall glows in the warm light's color.
    let mut refs = placed(
        b"REFR",
        0x0000_1001,
        0,
        0x0000_0900,
        [0.0, 0.0, 0.0],
        &sub(b"XEMI", &0x0000_0930u32.to_le_bytes()),
    );
    refs.extend(placed(
        b"REFR",
        0x0000_1002,
        0,
        0x0000_0901,
        [100.0, 50.0, 0.0],
        &sub(b"XSCL", &floats(&[2.0])),
    ));
    refs.extend(placed(
        b"REFR",
        0x0000_1003,
        flags::INITIALLY_DISABLED,
        0x0000_0901,
        [0.0; 3],
        &[],
    ));
    refs.extend(placed(
        b"REFR",
        0x0000_1004,
        flags::DELETED,
        0x0000_0901,
        [0.0; 3],
        &[],
    ));
    // A disabled marker, a crate that follows it, and one that does the opposite.
    refs.extend(placed(
        b"REFR",
        0x0000_1005,
        flags::INITIALLY_DISABLED,
        0x0000_003B,
        [0.0; 3],
        &[],
    ));
    refs.extend(placed(
        b"REFR",
        0x0000_1006,
        0,
        0x0000_0901,
        [0.0; 3],
        &xesp(0x0000_1005, false),
    ));
    refs.extend(placed(
        b"REFR",
        0x0000_1007,
        flags::INITIALLY_DISABLED,
        0x0000_0901,
        [7.0, 0.0, 0.0],
        &xesp(0x0000_1005, true),
    ));
    refs.extend(placed(
        b"REFR",
        0x0000_1008,
        0,
        0x0000_0032,
        [10.0, 20.0, 30.0],
        &[],
    ));
    refs.extend(placed(b"REFR", 0x0000_1009, 0, 0x0000_0902, [0.0; 3], &[]));
    refs.extend(placed(b"REFR", 0x0000_100A, 0, 0x0000_0910, [0.0; 3], &[]));
    // An emittance pointing at a sound: not a light or a region.
    refs.extend(placed(
        b"REFR",
        0x0000_100B,
        0,
        0x0000_0920,
        [0.0; 3],
        &sub(b"XEMI", &0x0000_0950u32.to_le_bytes()),
    ));
    refs.extend(placed(
        b"REFR",
        0x0000_100C,
        0,
        0x0000_0930,
        [0.0, 0.0, 200.0],
        &[
            sub(b"XRDS", &floats(&[512.0])),
            sub(b"XEMI", &0x0000_0930u32.to_le_bytes()),
        ]
        .concat(),
    ));
    refs.extend(placed(b"REFR", 0x0000_100D, 0, 0x0000_0931, [0.0; 3], &[]));
    refs.extend(placed(b"REFR", 0x0000_100E, 0, 0x0000_0950, [0.0; 3], &[]));
    refs.extend(placed(
        b"ACHR",
        0x0000_100F,
        0,
        0x0000_0960,
        [5.0, 5.0, 0.0],
        &[],
    ));
    refs.extend(placed(b"REFR", 0x0000_1010, 0, 0x0000_0999, [0.0; 3], &[]));
    refs.extend(placed(
        b"REFR",
        0x0000_1011,
        0,
        0x0000_0940,
        [0.0, -200.0, 0.0],
        &xtel(0x0000_2001, [0.0, 90.0, 0.0], 3.1),
    ));
    let mut extra = sub(b"XCLL", &lighting([40, 50, 60], [200, 190, 180]));
    extra.extend(sub(b"LTMP", &0x0000_0970u32.to_le_bytes()));
    extra.extend(sub(b"LNAM", &1u32.to_le_bytes()));
    let mut cells = interior(0x0000_0A00, "TestHouse", "Doc's House", &extra, &refs);

    // Its porch, with the door back.
    let porch_door = placed(
        b"REFR",
        0x0000_2001,
        0,
        0x0000_0940,
        [0.0, 0.0, 0.0],
        &xtel(0x0000_1011, [0.0, -150.0, 5.0], 0.25),
    );
    cells.extend(interior(
        0x0000_0B00,
        "TestPorch",
        "Porch",
        &[],
        &porch_door,
    ));
    file.extend(cells_group(&cells));
    file
}

/// Patch.esp: moves the crate and renames the house.
fn patch() -> Vec<u8> {
    let mut file = tes4(&["Master.esm"]);
    let crate_ref = placed(
        b"REFR",
        0x0000_1002,
        0,
        0x0000_0901,
        [300.0, 50.0, 0.0],
        &[],
    );
    let mut extra = sub(b"XCLL", &lighting([40, 50, 60], [200, 190, 180]));
    extra.extend(sub(b"XCIM", &0x0000_0980u32.to_le_bytes()));
    file.extend(cells_group(&interior(
        0x0000_0A00,
        "TestHouse",
        "Doc Mitchell's House",
        &extra,
        &crate_ref,
    )));
    file
}

fn order() -> LoadOrder {
    LoadOrder::from_plugins(vec![
        (
            "Master.esm".into(),
            None,
            Plugin::from_bytes(master()).unwrap(),
        ),
        (
            "Patch.esp".into(),
            None,
            Plugin::from_bytes(patch()).unwrap(),
        ),
    ])
    .unwrap()
}

fn ids(list: &[world::Placement]) -> Vec<u32> {
    list.iter().map(|p| p.form_id.0).collect()
}

#[test]
fn finds_cells_by_editor_id_name_or_form_id() {
    let order = order();
    let house = FormId(0x0000_0A00);
    assert_eq!(find_cells(&order, "testhouse").unwrap(), [house]);
    assert_eq!(find_cells(&order, "00000A00").unwrap(), [house]);
    assert_eq!(find_cells(&order, "doc mitchell's house").unwrap(), [house]);
    assert_eq!(find_cells(&order, "Test").unwrap().len(), 2);
    assert!(find_cells(&order, "vault").unwrap().is_empty());
    // A form ID that isn't a cell falls through to a text search.
    assert!(find_cells(&order, "00000900").unwrap().is_empty());

    let listing = interior_cells(&order).unwrap();
    assert_eq!(listing.len(), 2);
    assert_eq!(listing[0].name.as_deref(), Some("Doc Mitchell's House"));
    assert_eq!(listing[0].plugin, "Patch.esp");
    assert_eq!(listing[0].references, 17);
    assert_eq!(listing[1].references, 1);
}

#[test]
fn keeps_only_what_the_game_shows_at_first() {
    let order = order();
    let cell = load_cell(&order, FormId(0x0000_0A00)).unwrap();
    assert!(cell.info.interior);
    assert_eq!(
        ids(&cell.objects),
        [0x1001, 0x1002, 0x1007, 0x100A, 0x100B, 0x100C, 0x100D, 0x1011]
    );
    assert_eq!(ids(&cell.markers), [0x1008, 0x1009]);
    assert_eq!(ids(&cell.actors), [0x100F]);
    let skipped: Vec<(&str, usize)> = cell.skipped.iter().map(|(k, v)| (k.as_str(), *v)).collect();
    assert_eq!(
        skipped,
        [
            ("base object missing", 1),
            ("deleted", 1),
            ("disabled when the cell loads", 3),
            ("no model (SOUN)", 1),
        ]
    );
    let left_out: Vec<(u32, &str)> = cell
        .left_out
        .iter()
        .map(|l| (l.form_id.0, l.reason.as_str()))
        .collect();
    assert_eq!(
        left_out,
        [
            (0x1003, "disabled when the cell loads"),
            (0x1004, "deleted"),
            (0x1005, "disabled when the cell loads"),
            (0x1006, "disabled when the cell loads"),
            (0x100E, "no model (SOUN)"),
            (0x1010, "base object missing"),
        ]
    );
    let follower = cell.objects.iter().find(|p| p.form_id.0 == 0x1007).unwrap();
    assert_eq!(follower.enable_parent, Some((FormId(0x1005), true)));
    let hum = &cell.left_out[4];
    assert_eq!(hum.base_editor_id.as_deref(), Some("Hum"));
}

/// `VCG02BottleMarkerREF.Enable`: the disabled marker's `Enable` brings in
/// the crate that follows it and takes away the one doing the opposite.
#[test]
fn a_parents_enable_brings_in_its_children() {
    let order = order();
    let refs = [0x1005, 0x1006, 0x1007, 0x1001].map(FormId);
    let before = world::Disabled::new();
    let mut now = before.clone();
    now.insert(FormId(0x1005), false);
    assert_eq!(
        world::newly_enabled(&order, refs, &before, &now),
        [FormId(0x1005), FormId(0x1006)]
    );
    // Disabled again: those two go, the opposite one comes back.
    let mut later = now.clone();
    later.insert(FormId(0x1005), true);
    assert_eq!(
        world::newly_enabled(&order, refs, &now, &later),
        [FormId(0x1007)]
    );
    assert!(world::newly_enabled(&order, refs, &now, &now).is_empty());
}

#[test]
fn reads_placement_models_and_lights() {
    let order = order();
    let cell = load_cell(&order, FormId(0x0000_0A00)).unwrap();
    let get = |id: u32| cell.objects.iter().find(|p| p.form_id.0 == id).unwrap();

    let moved = get(0x1002);
    assert_eq!(moved.position, [300.0, 50.0, 0.0]);
    assert_eq!(moved.scale, 1.0);
    assert_eq!(moved.plugin, "Patch.esp");
    assert_eq!(moved.model.as_deref(), Some("Clutter\\Crate.nif"));
    assert_eq!(moved.rotation, [0.0, 0.0, 0.5]);

    assert_eq!(get(0x100B).model.as_deref(), Some("armor\\ground.nif"));

    // Emittance: a light's color, something else, or nothing set.
    assert_eq!(
        get(0x1001).emittance,
        Some(Emittance::Light([255, 200, 150]))
    );
    assert_eq!(get(0x100B).emittance, Some(Emittance::Other(FormId(0x950))));
    assert_eq!(moved.emittance, None);

    let collection = get(0x100A);
    assert_eq!(collection.parts.len(), 2);
    assert_eq!(collection.parts[1].position, [256.0, 0.0, 0.0]);
    assert_eq!(collection.parts[1].scale, 2.0);
    assert_eq!(collection.parts[1].rotation, [0.0, 0.0, 1.5]);
    assert_eq!(
        collection.parts[0].model.as_deref(),
        Some("Architecture\\Wall01.nif")
    );

    let lights: Vec<_> = cell.lights().collect();
    assert_eq!(lights.len(), 2);
    let (placed, warm) = lights[0];
    assert_eq!(placed.radius, Some(512.0));
    assert_eq!(warm.radius, 300.0);
    assert_eq!(warm.color, [255, 200, 150]);
    assert!(!warm.is_off_by_default());
    assert!(lights[1].1.is_off_by_default());

    // As the game lights with it: the reference's radius is added to the
    // base's, and the light its Emittance names tints the color.
    let used = cell.placed_light(placed).unwrap();
    assert_eq!(used.radius, 812.0);
    let expected = [1.0, (200.0f32 / 255.0).powi(2), (150.0f32 / 255.0).powi(2)];
    assert!(
        used.color
            .iter()
            .zip(expected)
            .all(|(a, b)| (a - b).abs() < 1e-6),
        "{:?}",
        used.color
    );
    // Without either, it's the base light.
    let plain = cell.placed_light(lights[1].0).unwrap();
    assert_eq!(plain.radius, 300.0);
    assert_eq!(plain.color, [1.0, 200.0 / 255.0, 150.0 / 255.0]);
}

#[test]
fn a_region_emittance_names_the_region() {
    let order = order();
    assert_eq!(
        world::resolve_emittance(&order, FormId(0x0000_09A0)),
        Emittance::Region(FormId(0x0000_09A0))
    );
    assert_eq!(
        world::resolve_emittance(&order, FormId(0x0000_0950)),
        Emittance::Other(FormId(0x0000_0950))
    );
}

#[test]
fn a_region_gives_its_weathers_sunlight_for_the_hour() {
    use world::weather::{EmittanceNow, WeatherState};
    let order = order();
    let close = |a: [f32; 3], b: [f32; 3]| a.iter().zip(b).all(|(x, y)| (x - y).abs() < 1e-4);
    let region = FormId(0x0000_09A0);
    let indoor = [1.0, 227.0 / 255.0, 170.0 / 255.0];

    // As the game starts: the region's likeliest weather, whose day and
    // high noon colors are the same (the recording's (255, 227, 170)).
    let start = EmittanceNow::at_start(&order, [region]);
    assert!(close(start.regions[&region], indoor), "{start:?}");
    assert_eq!(start.player_region, None);

    // The rolled weather counts. A region with none rolled gives
    // `DefaultWeather`, whose high noon is black: its day sunlight × w,
    // w 1 at sunrise's end (8:00), 0 at noon, 1 again at sunset's begin
    // (18:00 here, as `NVDefaultClimate`).
    let mut state = WeatherState::default();
    state.region_weathers.insert(region, FormId(0x0000_09A1));
    let at = |state: &WeatherState, hour: f32| EmittanceNow::new(&order, state, hour, [region]);
    assert!(close(at(&state, 12.0).regions[&region], indoor));
    let none = WeatherState::default();
    let day = [244.0 / 255.0, 206.0 / 255.0, 149.0 / 255.0];
    assert!(close(at(&none, 8.0).regions[&region], day));
    assert!(close(at(&none, 12.0).regions[&region], [0.0; 3]));
    let half = at(&none, 15.0).regions[&region];
    assert!(close(half, day.map(|c| c * 0.5)), "{half:?}");
}

#[test]
fn glows_without_an_emittance_take_the_players_weather_region() {
    let order = order();
    let mut cell = load_cell(&order, FormId(0x0000_0A00)).unwrap();
    let close = |a: [f32; 3], b: [f32; 3]| a.iter().zip(b).all(|(x, y)| (x - y).abs() < 1e-3);
    let id = |id: u32| cell.objects.iter().position(|p| p.form_id.0 == id).unwrap();
    let (plain, warm, hum) = (id(0x1002), id(0x1001), id(0x100B));

    // No weather region for the player (a cell read on its own): nothing,
    // the mesh keeps its own color. A light: its color. Something that
    // gives no color: none.
    assert_eq!(cell.emittance_color(&cell.objects[plain]), None);
    let color = cell.emittance_color(&cell.objects[warm]).unwrap();
    assert!(close(color, [1.0, 200.0 / 255.0, 150.0 / 255.0]));
    assert_eq!(cell.emittance_color(&cell.objects[hum]), None);

    // The recording of Doc Mitchell's house: the player's weather region
    // had no weather of its own (so `DefaultWeather`), and the "on" wall
    // lamps (multiplier 5) got (2.957, 2.496, 1.805): day sunlight ×
    // 0.618, which is 15:43 with sunset beginning at 18:00.
    let state = world::weather::WeatherState {
        region: Some(FormId(0x0000_09A0)),
        ..Default::default()
    };
    cell.set_emittance(&order, &state, 12.0 + 0.618 * 6.0);
    let lamp = cell
        .emittance_color(&cell.objects[plain])
        .unwrap()
        .map(|c| c * 5.0);
    assert!(close(lamp, [2.9565, 2.4961, 1.8054]), "{lamp:?}");
}
#[test]
fn the_directional_light_shines_along_its_angles() {
    let lighting = |around: i32, up: i32| {
        let mut data = vec![0u8; 40];
        data[20..24].copy_from_slice(&around.to_le_bytes());
        data[24..28].copy_from_slice(&up.to_le_bytes());
        world::Lighting::parse(&data).unwrap()
    };
    let close = |a: [f32; 3], b: [f32; 3]| a.iter().zip(b).all(|(x, y)| (x - y).abs() < 1e-5);
    // Doc Mitchell's house: 35° around, 270° up. The game hands its shaders
    // (0, 0, 1) toward the light: it shines straight down.
    let doc = lighting(35, 270).toward_directional();
    assert!(close(doc, [0.0, 0.0, 1.0]), "{doc:?}");
    // 90° up shines straight up, so the light is below.
    let below = lighting(0, 90).toward_directional();
    assert!(close(below, [0.0, 0.0, -1.0]), "{below:?}");
    // The Mojave Outpost barracks: 0° around, 0° up. The game hands its
    // shaders (-1, 0, 0) on an unrotated floor piece: the light shines
    // east, the around angle counting from east.
    let barracks = lighting(0, 0).toward_directional();
    assert!(close(barracks, [-1.0, 0.0, 0.0]), "{barracks:?}");
}

#[test]
fn resolves_lighting_templates_and_arrivals() {
    let order = order();
    let cell = load_cell(&order, FormId(0x0000_0A00)).unwrap();
    // The patch's version of the cell wins; it has no template.
    let lighting = cell.info.lighting.unwrap();
    assert_eq!(lighting.ambient, [40, 50, 60]);
    assert_eq!(lighting.directional_rotation_xy, 45);
    assert_eq!(cell.info.lighting_source, "cell");

    // It names the warm image space.
    let space = cell.info.image_space.as_ref().unwrap();
    assert_eq!(space.editor_id.as_deref(), Some("WarmSpace"));
    assert_eq!(space.size, 152);
    let c = space.cinematic().unwrap();
    assert_eq!(
        (c.saturation, c.contrast_average, c.contrast, c.brightness),
        (0.8, 0.5, 1.2, 1.1)
    );
    assert_eq!((c.tint, c.tint_amount), ([1.0, 0.8, 0.5], 0.3));
    // New Vegas's own 132-byte layout, one value shorter before the
    // cinematic part: ShackInterior01, which Doc Mitchell's house uses.
    let mut shack = space.clone();
    shack.size = 132;
    shack.values = vec![0.0; 33];
    shack.values[20..33].copy_from_slice(&[
        0.35, 0.498, 0.776, 2.5, 0.9, 0.14, 1.2, 1.1, 0.69, 0.561, 0.302, 0.5, 0.0,
    ]);
    let c = shack.cinematic().unwrap();
    assert_eq!(
        (c.saturation, c.contrast_average, c.contrast, c.brightness),
        (0.9, 0.14, 1.2, 1.1)
    );
    assert_eq!((c.tint, c.tint_amount), ([0.69, 0.561, 0.302], 0.5));
    // Its HDR values, which lead every layout: eye adaptation speed, blur
    // radius, passes, emissive multiplier, target, upper clamp, bright
    // scale, bright clamp.
    shack.values[0..8].copy_from_slice(&[0.3, 6.0, 4.0, 1.0, 1.0, 1.0, 2.4, 0.9]);
    let hdr = shack.hdr().unwrap();
    assert_eq!((hdr.bright_clamp, hdr.bright_scale), (0.9, 2.4));
    assert_eq!((hdr.blur_radius, hdr.eye_adapt_speed), (6.0, 0.3));
    assert_eq!((hdr.target_lum, hdr.upper_lum_clamp), (1.0, 1.0));
    assert_eq!(hdr.emissive_mult, 1.0);
    // The Mojave Outpost barracks' OfficeDefaultImageSpace: its emissive
    // multiplier of 3 is what made the tube fixtures' glow 1.3 arrive as
    // 3.9 in the recording.
    shack.values[0..8].copy_from_slice(&[0.9, 8.0, 2.0, 3.0, 1.0, 2.0, 1.5, 0.35]);
    let office = shack.hdr().unwrap();
    assert_eq!(office.emissive_mult, 3.0);
    assert_eq!((office.target_lum, office.upper_lum_clamp), (1.0, 2.0));

    // Values that can't be right (a layout misread) give no cinematic part.
    let mut garbled = space.clone();
    garbled.values[29] = 40.0;
    assert_eq!(garbled.cinematic(), None);

    let arrivals: Vec<(&str, [f32; 3])> = cell
        .arrivals
        .iter()
        .map(|a| (a.via.as_str(), a.position))
        .collect();
    assert_eq!(
        arrivals,
        [
            ("the coc marker", [10.0, 20.0, 30.0]),
            ("door 00001011 from TestPorch", [0.0, -150.0, 5.0]),
        ]
    );

    // Without the patch, the master's cell takes ambient from its template.
    let alone = LoadOrder::from_plugins(vec![(
        "Master.esm".into(),
        None,
        Plugin::from_bytes(master()).unwrap(),
    )])
    .unwrap();
    let info = world::cell_info(&alone, FormId(0x0000_0A00)).unwrap();
    let lighting = info.lighting.unwrap();
    assert_eq!(lighting.ambient, [90, 90, 90]);
    assert_eq!(lighting.directional, [200, 190, 180]);
    assert_eq!(
        info.lighting_source,
        "cell, partly from template DimTemplate"
    );

    let err = world::cell_info(&alone, FormId(0x0000_0900)).unwrap_err();
    assert_eq!(err.to_string(), "00000900 is a STAT record, not a cell");
}

#[test]
fn placement_transforms_use_the_chosen_convention() {
    let t = RotationConvention::DEFAULT.transform(
        [100.0, 0.0, 0.0],
        [0.0, 0.0, std::f32::consts::FRAC_PI_2],
        2.0,
    );
    let p = t.apply_point([0.0, 10.0, 0.0]);
    assert!((p[0] - 120.0).abs() < 1e-4 && p[1].abs() < 1e-4, "{p:?}");
}
